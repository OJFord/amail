variable "domains" {
  description = "Domains to verify and use"
  type        = list(string)
}

variable "smtp_relay" {
  description = "SMTP relay options"
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
