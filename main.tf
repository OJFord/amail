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

  zone_id = each.value.id
  type    = "MX"
  name    = "@"
  value   = "inbound-smtp.${data.aws_region.current.name}.amazonaws.com"
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
}
