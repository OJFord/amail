locals {
  one_if_enabled = var.enable ? 1 : 0
}

resource "null_resource" "lambda_runtime" {
  count = local.one_if_enabled

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
    working_dir = path.module
  }
}

data "archive_file" "lambda_runtime" {
  depends_on = [
    null_resource.lambda_runtime,
  ]

  type        = "zip"
  source_file = "${path.module}/bootstrap"
  output_path = "${path.module}/handler.zip"
}

data "aws_iam_policy_document" "smtp_relay" {
  statement {
    effect = "Allow"
    actions = [
      "sts:AssumeRole",
    ]

    principals {
      type = "Service"
      identifiers = [
        "lambda.amazonaws.com",
      ]
    }
  }
}

resource "aws_iam_role" "smtp_relay" {
  count = local.one_if_enabled

  path               = var.aws_iam_path
  name               = "smtp-relay"
  assume_role_policy = data.aws_iam_policy_document.smtp_relay.json
}

resource "aws_lambda_function" "smtp_relay" {
  count = local.one_if_enabled

  function_name    = "smtp-relay"
  filename         = data.archive_file.lambda_runtime.output_path
  source_code_hash = data.archive_file.lambda_runtime.output_base64sha256
  runtime          = "provided"
  handler          = "main"
  role             = aws_iam_role.smtp_relay[count.index].arn

  kms_key_arn = var.user_params.kms_key.arn
  environment {
    variables = {
      S3_BUCKET           = var.eml_bucket.id
      S3_REGION           = var.eml_bucket.region
      RELAY_ENVELOPE_FROM = var.user_params.envelope_from
      RELAY_ENVELOPE_TO   = var.user_params.envelope_to
      SMTP_HOST           = var.user_params.smtp_host
      SMTP_USER           = var.user_params.smtp_user
      SMTP_PASS           = var.user_params.smtp_password
    }
  }
}

resource "aws_lambda_permission" "eml_store" {
  count = local.one_if_enabled

  statement_id   = "AllowSESInvocation"
  action         = "lambda:InvokeFunction"
  function_name  = aws_lambda_function.smtp_relay[count.index].function_name
  principal      = "ses.amazonaws.com"
  source_account = var.aws_account_id
}

resource "aws_iam_role_policy_attachment" "logging" {
  count = local.one_if_enabled

  role       = aws_iam_role.smtp_relay[count.index].name
  policy_arn = var.aws_iam_policy.logging.arn
}

resource "aws_iam_role_policy_attachment" "eml_fetch" {
  count = local.one_if_enabled

  role       = aws_iam_role.smtp_relay[count.index].name
  policy_arn = var.aws_iam_policy.eml_fetch.arn
}
