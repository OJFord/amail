output "function_arn" {
  value = var.enable ? aws_lambda_function.smtp_relay[0].arn : null
}
