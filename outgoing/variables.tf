variable "aws_iam_path" {
  type = string
}

variable "email_domain_zones" {
  type = list(object({
    id   = string
    zone = string
  }))
}
