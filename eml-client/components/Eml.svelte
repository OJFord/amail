<script>
  import {
    Dropdown,
    DropdownToggle,
    DropdownMenu,
    DropdownItem,
    Row,
    Spinner,
    //
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  import TagBadges from "./TagBadges.svelte";

  export let emlMeta;

  let body = null;
  let selectedAlt = null;

  const refreshDefaultSelection = () =>
    (selectedAlt =
      body.Contents ||
      body.Alternatives.sort((a) => -a.Contents.is_cleaned_html)[0].Contents);
  $: tauri.invoke("view_eml", { id: emlMeta.id.valueOf() }).then((eml) => {
    body = eml;
    refreshDefaultSelection();
  });

  const _formatAddr = (m) => m.address || `[${m.members.length} mailboxes]`;
  const formatMailAddr = (m) =>
    m.name ? `${m.name} <${_formatAddr(m)}>` : _formatAddr(m);
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
    {#if body.Alternatives}
      <Dropdown>
        <DropdownToggle caret class="btn-xs">
          {selectedAlt.mimetype}
        </DropdownToggle>
        <DropdownMenu>
          {#each body.Alternatives as alt}
            <DropdownItem
              on:click={() => {
                selectedAlt = alt.Contents;
              }}>{alt.Contents.mimetype}</DropdownItem
            >
          {/each}
        </DropdownMenu>
      </Dropdown>
    {/if}
  </Row>

  <hr class="border-bottom" />

  <Row class="flex-fill mh-100 scroll">
    <div class="body">
      {#if selectedAlt.is_cleaned_html}
        {@html selectedAlt.content}
      {:else}
        {selectedAlt.content}
      {/if}
    </div>
  </Row>
{/if}

<style lang="scss" scoped>
  .body {
    padding: 1rem;
  }
</style>
