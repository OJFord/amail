terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
    }
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.34.0"
    }
    random = {
      source = "hashicorp/random"
    }
  }
  required_version = "~> 1.7"
}
