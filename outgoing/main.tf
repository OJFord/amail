data "aws_region" "current" {}

locals {
  subdomain = "outgoing"
}

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

resource "aws_ses_domain_mail_from" "outgoing" {
  for_each = { for z in var.email_domain_zones : z.zone => z }

  domain           = each.value.zone
  mail_from_domain = "${local.subdomain}.${each.value.zone}"
}

resource "cloudflare_record" "spf" {
  for_each = { for z in var.email_domain_zones : z.zone => z }

  zone_id = each.value.id
  type    = "TXT"
  name    = local.subdomain
  value   = "v=spf1 include:amazonses.com -all"
}

resource "cloudflare_record" "mx" {
  for_each = { for z in var.email_domain_zones : z.zone => z }

  zone_id  = each.value.id
  type     = "MX"
  name     = local.subdomain
  priority = 10
  value    = "feedback-smtp.${data.aws_region.current.name}.amazonses.com"
}
