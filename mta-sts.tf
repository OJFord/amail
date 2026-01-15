locals {
  mta_sts_content = join("\n", [
    "version: STSv1",
    "mode: enforce",
    "mx: ${cloudflare_dns_record.mx[keys(local.email_domain_zones)[0]].value}",
    "max_age: 604800",
  ])
}

moved {
  from = cloudflare_worker_script.mta-sts
  to   = cloudflare_workers_script.mta-sts
}
resource "cloudflare_workers_script" "mta-sts" {
  for_each = local.email_domain_zones

  account_id  = var.cloudflare_account_id
  script_name = "mta-sts-${replace(each.value.zone, ".", "-")}"
  content     = <<EOC
    async function handleRequest(request) {
      return new Response(`${local.mta_sts_content}`, {
        headers: {
          'content-type': 'text/plain',
        }
      })
    }

    addEventListener('fetch', event => {
      return event.respondWith(handleRequest(event.request));
    })
  EOC
}

moved {
  from = cloudflare_worker_route.mta-sts
  to   = cloudflare_workers_route.mta-sts
}
resource "cloudflare_workers_route" "mta-sts" {
  for_each = local.email_domain_zones

  zone_id = each.value.id
  pattern = "mta-sts.${each.value.zone}/.well-known/mta-sts.txt"
  script  = cloudflare_workers_script.mta-sts[each.key].script_name
}

moved {
  from = cloudflare_record.mta-sts
  to   = cloudflare_dns_record.mta-sts
}
resource "cloudflare_dns_record" "mta-sts" {
  for_each = local.email_domain_zones
  depends_on = [
    cloudflare_workers_route.mta-sts,
  ]

  zone_id = each.value.id
  type    = "CNAME"
  name    = "mta-sts"
  content = "${cloudflare_workers_script.mta-sts[each.key].script_name}.${var.cloudflare_workers_subdomain}.workers.dev"
  proxied = true
  ttl     = 1
}

moved {
  from = cloudflare_record._mta-sts
  to   = cloudflare_dns_record._mta-sts
}
resource "cloudflare_dns_record" "_mta-sts" {
  for_each = local.email_domain_zones
  depends_on = [
    cloudflare_workers_route.mta-sts,
  ]

  zone_id = each.value.id
  type    = "TXT"
  name    = "_mta-sts"
  content = "v=STSv1; id=${md5(local.mta_sts_content)};"
  ttl     = 1
}
