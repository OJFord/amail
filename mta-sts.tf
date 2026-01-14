locals {
  mta_sts_content = join("\n", [
    "version: STSv1",
    "mode: enforce",
    "mx: ${cloudflare_record.mx[keys(local.email_domain_zones)[0]].value}",
    "max_age: 604800",
  ])
}

moved {
  from = cloudflare_worker_script.mta-sts
  to   = cloudflare_workers_script.mta-sts
}
resource "cloudflare_workers_script" "mta-sts" {
  for_each = local.email_domain_zones

  account_id = var.cloudflare_account_id
  name       = "mta-sts-${replace(each.value.zone, ".", "-")}"
  content    = <<EOC
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

  zone_id     = each.value.id
  pattern     = "mta-sts.${each.value.zone}/.well-known/mta-sts.txt"
  script_name = cloudflare_workers_script.mta-sts[each.key].name
}

resource "cloudflare_record" "mta-sts" {
  for_each = local.email_domain_zones
  depends_on = [
    cloudflare_workers_route.mta-sts,
  ]

  zone_id = each.value.id
  type    = "CNAME"
  name    = "mta-sts"
  value   = "${cloudflare_workers_script.mta-sts[each.key].name}.${var.cloudflare_workers_subdomain}.workers.dev"
  proxied = true
}

resource "cloudflare_record" "_mta-sts" {
  for_each = local.email_domain_zones
  depends_on = [
    cloudflare_workers_route.mta-sts,
  ]

  zone_id = each.value.id
  type    = "TXT"
  name    = "_mta-sts"
  value   = "v=STSv1; id=${md5(local.mta_sts_content)};"
}
