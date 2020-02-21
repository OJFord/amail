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
    working_dir = path.module
  }
}

data "archive_file" "smtp_relay" {
  depends_on = [
    null_resource.smtp_relay,
  ]

  type        = "zip"
  source_file = "${path.module}/bootstrap"
  output_path = "${path.module}/handler.zip"
}

data "aws_iam_policy_document" "lambda_assume_role" {
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
  statement_id   = "smtp-relay"
  action         = "lambda:InvokeFunction"
  function_name  = aws_lambda_function.smtp_relay.function_name
  principal      = "ses.amazonaws.com"
  source_account = var.aws_account_id
}

resource "aws_iam_role_policy_attachment" "smtp_relay_cloudwatch" {
  role       = aws_iam_role.smtp_relay.name
  policy_arn = var.aws_iam_policy.logging.arn
}

resource "aws_iam_role_policy_attachment" "smtp_relay_eml_fetch" {
  role       = aws_iam_role.smtp_relay.name
  policy_arn = var.aws_iam_policy.eml_fetch.arn
}
