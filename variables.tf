variable "aws_iam_path_prefix" {
  description = "Prefix to use for IAM paths"
  type        = string
  default     = ""
}

variable "domains" {
  description = "Domains to verify and use"
  type        = list(string)
}

variable "modules" {
  description = "Optional modules to enable. If enabled, the parameter map of the same name must also be given."
  type = object({
    outgoing   = bool
    smtp_relay = bool
  })
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
