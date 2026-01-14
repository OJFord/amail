output "smtp" {
  value = {
    host = "email-smtp.${data.aws_region.current.region}.amazonaws.com"
    ports = [
      25,
      465,
      587,
    ]
    tls      = true
    username = aws_iam_access_key.smtp.id
    password = aws_iam_access_key.smtp.ses_smtp_password_v4
  }
}
