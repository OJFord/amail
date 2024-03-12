variable "aws_iam_path_prefix" {
  description = "Prefix to use for IAM paths"
  type        = string
  default     = ""
}

variable "cloudflare_account_id" {
  type = string
}

# Remove once https://github.com/cloudflare/terraform-provider-cloudflare/issues/1921
variable "cloudflare_workers_subdomain" {
  description = "Cloudflare account <subdomain>.workers.dev - https://dash.cloudflare.com/workers/subdomain"
  type        = string
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

variable "outgoing" {
  description = "Outgoing module options"
  type = object({
    monitoring = object({
      dmarc = object({
        aggregates = list(string)
        forensics  = list(string)
      })
      tls = list(string)
    })
  })

  default = {
    monitoring = {
      dmarc = {
        aggregates = []
        forensics  = []
      }
      tls = []
    }
  }
}

variable "smtp_relay" {
  description = "SMTP relay module options"
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
