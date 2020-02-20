locals {
  email_domain_zones = {
    for z in data.cloudflare_zones.email_domain : z.filter[0].name => {
      zone = z.zones[0].name
      id   = z.zones[0].id
    }
  }

  num_ses_dkim_tokens = 3
}

resource "aws_ses_domain_identity" "email_domain" {
  for_each = toset(var.domains)

  domain = each.value
}

data "cloudflare_zones" "email_domain" {
  for_each = aws_ses_domain_identity.email_domain

  filter {
    name = each.key
  }
}

resource "cloudflare_record" "email_domain_verification" {
  for_each = aws_ses_domain_identity.email_domain

  zone_id = local.email_domain_zones[each.key].id
  type    = "TXT"
  name    = "_amazonses"
  value   = each.value.verification_token
}

resource "aws_ses_domain_identity_verification" "email_domain" {
  for_each = aws_ses_domain_identity.email_domain

  domain = each.value.domain
  depends_on = [
    cloudflare_record.email_domain_verification,
  ]
}

resource "aws_ses_domain_dkim" "email_provenance" {
  for_each = aws_ses_domain_identity.email_domain

  domain = each.value.domain
}

resource "cloudflare_record" "email_provenance" {
  for_each = {
    for domain_i in setproduct(var.domains, range(local.num_ses_dkim_tokens))
    : "${local.email_domain_zones[domain_i[0]].zone}-dkim${domain_i[1]}" => merge(
      local.email_domain_zones[domain_i[0]],
      { token = aws_ses_domain_dkim.email_provenance[domain_i[0]].dkim_tokens[domain_i[1]] }
    )
  }

  zone_id = each.value.id
  type    = "CNAME"
  name    = "${each.value.token}._domainkey"
  value   = "${each.value.token}.dkim.amazonses.com"
}

resource "cloudflare_record" "email_antispoof" {
  for_each = local.email_domain_zones

  zone_id = each.value.id
  type    = "TXT"
  name    = "@"
  value   = "v=spf1 include:amazonses.com ~all"
}

data "aws_region" "current" {}

resource "cloudflare_record" "email_receiving" {
  for_each = local.email_domain_zones

  zone_id  = each.value.id
  type     = "MX"
  name     = "@"
  value    = "inbound-smtp.${data.aws_region.current.name}.amazonaws.com"
  priority = 10
}

resource "random_id" "eml_store" {
  byte_length = 2
}

resource "aws_s3_bucket" "eml_store" {
  bucket = "email-${random_id.eml_store.hex}"
}

data "aws_caller_identity" "current" {}

data "aws_iam_policy_document" "eml_store" {
  statement {
    sid    = "AllowSESPuts"
    effect = "Allow"
    principals {
      type = "Service"
      identifiers = [
        "ses.amazonaws.com",
      ]
    }
    actions = [
      "s3:PutObject",
    ]
    resources = [
      "${aws_s3_bucket.eml_store.arn}/*",
    ]
    condition {
      test     = "StringEquals"
      variable = "aws:Referer"
      values = [
        data.aws_caller_identity.current.account_id,
      ]
    }
  }
}

resource "aws_s3_bucket_policy" "eml_store" {
  bucket = aws_s3_bucket.eml_store.id
  policy = data.aws_iam_policy_document.eml_store.json
}

resource "aws_ses_receipt_rule_set" "main" {
  rule_set_name = "main"
}

resource "aws_ses_active_receipt_rule_set" "main" {
  rule_set_name = aws_ses_receipt_rule_set.main.rule_set_name
}

resource "aws_ses_receipt_rule" "eml_store" {
  name          = "store"
  rule_set_name = aws_ses_receipt_rule_set.main.rule_set_name
  enabled       = true
  scan_enabled  = true

  s3_action {
    bucket_name = aws_s3_bucket_policy.eml_store.bucket
    position    = 1
  }

  lambda_action {
    function_arn    = aws_lambda_function.smtp_relay.arn
    invocation_type = "Event"
    position        = 2
  }
}

resource "null_resource" "smtp_relay" {
  triggers = {
    # TODO: not trigger always, download from GitHub releases?
    always = uuid() # always trigger and let cargo decide if anything to do
  }

  provisioner "local-exec" {
    command     = "docker-compose run lambda-smtp-relay-build"
    working_dir = path.module
  }

  provisioner "local-exec" {
    command     = "cp target/x86_64-unknown-linux-musl/release/handler bootstrap"
    working_dir = "${path.module}/lambda-smtp-relay"
  }
}

data "archive_file" "smtp_relay" {
  depends_on = [
    null_resource.smtp_relay,
  ]

  type        = "zip"
  source_file = "${path.module}/lambda-smtp-relay/bootstrap"
  output_path = "${path.module}/lambda-smtp-relay/handler.zip"
}

data "aws_iam_policy_document" "lambda_assume_role" {
  statement {
    effect = "Allow"
    actions = [
      "sts:AssumeRole",
    ]

    principals {
      type        = "Service"
      identifiers = ["lambda.amazonaws.com"]
    }
  }
}

resource "aws_iam_role" "smtp_relay" {
  name               = "smtp-relay"
  assume_role_policy = data.aws_iam_policy_document.lambda_assume_role.json
}

resource "aws_lambda_function" "smtp_relay" {
  function_name    = "smtp-relay"
  filename         = data.archive_file.smtp_relay.output_path
  source_code_hash = data.archive_file.smtp_relay.output_base64sha256
  runtime          = "provided"
  handler          = "main"
  role             = aws_iam_role.smtp_relay.arn

  kms_key_arn = var.smtp_relay.kms_key.arn
  environment {
    variables = {
      S3_BUCKET           = aws_s3_bucket.eml_store.id
      S3_REGION           = aws_s3_bucket.eml_store.region
      RELAY_ENVELOPE_FROM = var.smtp_relay.envelope_from
      RELAY_ENVELOPE_TO   = var.smtp_relay.envelope_to
      SMTP_HOST           = var.smtp_relay.smtp_host
      SMTP_USER           = var.smtp_relay.smtp_user
      SMTP_PASS           = var.smtp_relay.smtp_password
    }
  }
}

resource "aws_lambda_permission" "eml_store" {
  statement_id   = "smtp-relay"
  action         = "lambda:InvokeFunction"
  function_name  = aws_lambda_function.smtp_relay.function_name
  principal      = "ses.amazonaws.com"
  source_account = data.aws_caller_identity.current.account_id
}

data "aws_iam_policy_document" "log_to_cloudwatch" {
  statement {
    sid    = "AllowLogging"
    effect = "Allow"
    actions = [
      "logs:CreateLogGroup",
      "logs:CreateLogStream",
      "logs:PutLogEvents",
    ]
    resources = [
      "*",
    ]
  }
}

resource "aws_iam_policy" "log_to_cloudwatch" {
  name   = join("", data.aws_iam_policy_document.log_to_cloudwatch.statement.*.sid)
  policy = data.aws_iam_policy_document.log_to_cloudwatch.json
}

resource "aws_iam_role_policy_attachment" "smtp_relay_cloudwatch" {
  role       = aws_iam_role.smtp_relay.name
  policy_arn = aws_iam_policy.log_to_cloudwatch.arn
}

data "aws_iam_policy_document" "eml_fetch" {
  statement {
    sid    = "AllowFetchEmail${random_id.eml_store.hex}"
    effect = "Allow"
    actions = [
      "s3:GetObject",
    ]
    resources = [
      "${aws_s3_bucket.eml_store.arn}/*",
    ]
  }
}

resource "aws_iam_policy" "eml_fetch" {
  name   = join("", data.aws_iam_policy_document.eml_fetch.statement.*.sid)
  policy = data.aws_iam_policy_document.eml_fetch.json
}

resource "aws_iam_role_policy_attachment" "smtp_relay_eml_fetch" {
  role       = aws_iam_role.smtp_relay.name
  policy_arn = aws_iam_policy.eml_fetch.arn
}
