output "outgoing_smtp" {
  value = var.modules.smtp_relay ? module.outgoing[0].smtp : null
}
