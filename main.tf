locals {
  email_domain_zones = {
    for z in data.cloudflare_zones.email_domain : z.filter[0].name => {
      zone = z.zones[0].name
      id   = z.zones[0].id
    }
  }

  num_ses_dkim_tokens = 3

  aws_iam_path = "${var.aws_iam_path_prefix}/amail/"
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
  content = each.value.verification_token
}

resource "aws_ses_domain_identity_verification" "email_domain" {
  for_each = aws_ses_domain_identity.email_domain

  domain = each.value.domain
  depends_on = [
    cloudflare_record.email_domain_verification,
  ]
}

resource "aws_ses_domain_dkim" "dkim" {
  for_each = aws_ses_domain_identity.email_domain

  domain = each.value.domain
}

# DKIM here (i.e. even if outgoing module disabled) since it's now used for verification.
resource "cloudflare_record" "dkim" {
  for_each = {
    for domain_i in setproduct(var.domains, range(local.num_ses_dkim_tokens))
    : "${local.email_domain_zones[domain_i[0]].zone}-dkim${domain_i[1]}" => merge(
      local.email_domain_zones[domain_i[0]],
      { token = aws_ses_domain_dkim.dkim[domain_i[0]].dkim_tokens[domain_i[1]] }
    )
  }

  zone_id = each.value.id
  type    = "CNAME"
  name    = "${each.value.token}._domainkey"
  content = "${each.value.token}.dkim.amazonses.com"
}

resource "cloudflare_record" "spf" {
  for_each = {
    for pair in setproduct(values(local.email_domain_zones), ["@", "*"])
    : "${pair[0].zone}-${pair[1]}" => {
      zone_id = pair[0].id
      name    = pair[1]
    }
  }

  zone_id = each.value.zone_id
  type    = "TXT"
  name    = each.value.name
  content = "v=spf1 -all"
}

# DMARC here to ensure fail if outgoing module disabled.
resource "cloudflare_record" "dmarc" {
  for_each = local.email_domain_zones

  zone_id = each.value.id
  type    = "TXT"
  name    = "_dmarc"
  content = trimspace(join("; ", [
    "v=DMARC1",
    "p=reject",
    "sp=reject",
    "adkim=s",
    "aspf=${var.modules.outgoing ? "r" : "s"}",
    "rua=${join(",", [for addr in var.outgoing.monitoring.dmarc.aggregates : "mailto:${addr}"])}",
    "ruf=${join(",", [for addr in var.outgoing.monitoring.dmarc.forensics : "mailto:${addr}"])}",
    "" # trailing ;
  ]))
}

resource "cloudflare_record" "tlsrpt" {
  for_each = local.email_domain_zones

  zone_id = each.value.id
  type    = "TXT"
  name    = "_smtp._tls"
  content = trimspace(join("; ", [
    "v=TLSRPTv1",
    "rua=${join(",", [for addr in var.outgoing.monitoring.tls : "mailto:${addr}"])}",
    "" # trailing ;
  ]))
}

data "aws_region" "current" {}

resource "cloudflare_record" "mx" {
  for_each = local.email_domain_zones

  zone_id  = each.value.id
  type     = "MX"
  name     = "@"
  content  = "inbound-smtp.${data.aws_region.current.region}.amazonaws.com"
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

  dynamic "lambda_action" {
    for_each = var.modules.smtp_relay ? [module.smtp_relay[0].function_arn] : []

    content {
      function_arn    = lambda_action.value
      invocation_type = "Event"
      position        = 2
    }
  }
}

data "aws_iam_policy_document" "logging" {
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

resource "aws_iam_policy" "logging" {
  path   = local.aws_iam_path
  name   = join("", data.aws_iam_policy_document.logging.statement.*.sid)
  policy = data.aws_iam_policy_document.logging.json
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
  path   = local.aws_iam_path
  name   = join("", data.aws_iam_policy_document.eml_fetch.statement.*.sid)
  policy = data.aws_iam_policy_document.eml_fetch.json
}

module "smtp_relay" {
  source = "./lambda-smtp-relay"
  count  = var.modules.smtp_relay ? 1 : 0

  aws_account_id = data.aws_caller_identity.current.account_id
  aws_iam_path   = local.aws_iam_path
  aws_iam_policy = {
    logging   = aws_iam_policy.logging
    eml_fetch = aws_iam_policy.eml_fetch
  }
  eml_bucket  = aws_s3_bucket.eml_store
  user_params = var.smtp_relay
}

module "outgoing" {
  source = "./outgoing"
  count  = var.modules.outgoing ? 1 : 0

  aws_iam_path       = local.aws_iam_path
  email_domain_zones = values(local.email_domain_zones)
}
