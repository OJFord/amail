<script>
  import {
    Col,
    Dropdown,
    DropdownToggle,
    DropdownMenu,
    DropdownItem,
    Row,
    Spinner,
    //
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  import EmlAttachment from "./EmlAttachment.svelte";
  import EmlBodyPart from "./EmlBodyPart.svelte";
  import TagBadges from "./TagBadges.svelte";

  export let emlMeta;

  let body = null;
  let selectedAlt = null;

  const refreshDefaultSelection = () => {
    let altHtml = body.alternatives.filter((a) => a.is_cleaned_html)[0];
    selectedAlt = !body.is_cleaned_html && altHtml ? altHtml : body;
  };

  $: tauri.invoke("view_eml", { id: emlMeta.id.valueOf() }).then((eml) => {
    body = eml;
    refreshDefaultSelection();
  });

  const _formatAddr = (m) => m.address || `[${m.members.length} mailboxes]`;
  const formatMailAddr = (m) =>
    m.name ? `${m.name} <${_formatAddr(m)}>` : _formatAddr(m);

  let alts;
  $: if (body) alts = [body].concat(body.alternatives);

  let attachments = [];
  $: if (selectedAlt)
    attachments = [selectedAlt]
      .concat(selectedAlt.extra)
      .filter((e) => e.disposition == "Attachment");

  let inlines = [];
  $: if (selectedAlt)
    inlines = [selectedAlt]
      .concat(selectedAlt.extra)
      .filter((e) => e.disposition == "Inline");
</script>

{#if body == null}
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
    {#if !selectedAlt}
      <Spinner />
    {:else if body.alternatives}
      <Dropdown>
        <DropdownToggle caret class="btn-xs">
          {selectedAlt.mimetype}
        </DropdownToggle>
        <DropdownMenu>
          {#each alts.filter((a) => a != selectedAlt) as alt}
            <DropdownItem
              on:click={() => {
                selectedAlt = alt;
              }}
            >
              {alt.mimetype}
            </DropdownItem>
          {/each}
        </DropdownMenu>
      </Dropdown>
    {/if}
  </Row>

  <hr class="border-bottom" />

  <Row class="flex-fill mh-100 scroll">
    <div class="body">
      {#each inlines as part}
        <EmlBodyPart {part} />
      {/each}
    </div>
  </Row>

  {#if attachments.length}
    <Row class="border-top">
      {#each attachments as part}
        <Col xs="3">
          <EmlAttachment {part} />
        </Col>
      {/each}
    </Row>
  {/if}
{/if}

<style lang="scss" scoped>
  .body {
    padding: 1rem;
  }
</style>
