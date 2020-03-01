resource "aws_iam_access_key" "smtp" {
  user = aws_iam_user.smtp.name
}

resource "aws_iam_user" "smtp" {
  name = "smtp"
  path = var.aws_iam_path
}

data "aws_iam_policy_document" "smtp" {
  statement {
    sid    = "AllowSendRawEmail"
    effect = "Allow"
    actions = [
      "ses:SendRawEmail",
    ]
    resources = [
      "*",
    ]
  }
}

resource "aws_iam_user_policy" "smtp" {
  name   = join("", data.aws_iam_policy_document.smtp.statement.*.sid)
  user   = aws_iam_user.smtp.name
  policy = data.aws_iam_policy_document.smtp.json
}
