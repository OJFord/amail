variable "aws_account_id" {
  type = string
}

variable "aws_iam_policy" {
  type = object({
    logging = object({
      arn = string
    })
    eml_fetch = object({
      arn = string
    })
  })
}

variable "eml_bucket" {
  type = object({
    id     = string
    region = string
  })
}

variable "user_params" {
  type = object({
    kms_key = object({
      arn = string
    })
    envelope_from = string
    envelope_to   = string
    smtp_host     = string
    smtp_user     = string
    smtp_password = string
  })

  default = {
    kms_key = {
      arn = ""
    }
    envelope_from = ""
    envelope_to   = ""
    smtp_host     = ""
    smtp_user     = ""
    smtp_password = ""
  }
}
