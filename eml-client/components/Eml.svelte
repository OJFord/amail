<script>
  import sanitizeHtml from "sanitize-html";
  import {
    Row,
    Spinner,
    //
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  import TagBadges from "./TagBadges.svelte";

  export let emlMeta;

  let contents = null;

  $: tauri
    .invoke("view_eml", { emlMeta })
    .then((eml) => {
      contents = sanitizeHtml(eml, {
        allowedTags: sanitizeHtml.defaults.allowedTags.filter(
          (tag) => tag != "img"
        ),
      });
    })
    .catch(console.error);

  const _formatAddr = (m) => m.address || `[${m.members.length} mailboxes]`;
  const formatMailAddr = (m) =>
    m.name ? `${m.name} <${_formatAddr(m)}>` : _formatAddr(m);
</script>

{#if contents == null}
  <Spinner primary />
{:else}
  <Row style="padding-top: 1rem;">
    <h4>
      <i>From:</i>
      {emlMeta.from.map(formatMailAddr).join("; ")}
    </h4>

    {#if emlMeta.sender && !emlMeta.from.includes(emlMeta.sender.valueOf())}
      <h4>
        <i>Sent by:</i>
        {formatMailAddr(emlMeta.sender)}
      </h4>
    {/if}

    {#if emlMeta.to}
      <h4>
        <i>To:</i>
        {emlMeta.to.map(formatMailAddr).join("; ")}
      </h4>
    {/if}

    {#if emlMeta.cc}
      <h4>
        <i>Cc:</i>
        {emlMeta.cc.map(formatMailAddr).join("; ")}
      </h4>
    {/if}

    {#if emlMeta.bcc}
      <h4>
        <i>Bcc:</i>
        {emlMeta.bcc.map(formatMailAddr).join("; ")}
      </h4>
    {/if}

    <h2>{emlMeta.subject}</h2>

    <TagBadges tags={emlMeta.tags} />
  </Row>

  <Row>
    <div>
      {@html contents}
    </div>
  </Row>
{/if}
