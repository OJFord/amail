<script>
  import { createEventDispatcher } from "svelte";
  import {
    Button,
    Col,
    Dropdown,
    DropdownToggle,
    DropdownMenu,
    DropdownItem,
    Row,
    Spinner,
    //
  } from "sveltestrap";

  import * as api from "../api.js";
  import EmlAddresses from "./EmlAddresses.svelte";
  import EmlAttachment from "./EmlAttachment.svelte";
  import EmlBodyPart from "./EmlBodyPart.svelte";
  import EmlReplyModal from "./EmlReplyModal.svelte";
  import TagBadges from "./TagBadges.svelte";

  export let emlMeta;

  const dispatch = createEventDispatcher();

  let body = null;
  let selectedAlt = null;

  const refreshDefaultSelection = (eml) => {
    body = eml;
    let altHtml = body.alternatives.filter((a) => a.is_cleaned_html)[0];
    selectedAlt = !body.is_cleaned_html && altHtml ? altHtml : body;
  };

  $: api.viewEml(emlMeta.id.valueOf()).then(refreshDefaultSelection);

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

  let replyModalOpen = false;
</script>

{#if body == null}
  <Spinner primary />
{:else}
  <Row style="padding-top: 1rem;">
    <EmlAddresses {emlMeta} />

    <h2>{emlMeta.subject}</h2>

    <TagBadges tags={emlMeta.tags} />
  </Row>

  <Row>
    <Col xs="1" class="align-left">
      {#if !selectedAlt}
        <Spinner />
      {:else}
        <Dropdown>
          <DropdownToggle
            caret
            class={`${body.alternatives.length ? "" : "disabled"}`}
          >
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
    </Col>

    <Col xs="1" class="align-left">
      <Button
        on:click={() => api.rmTag(`id:${emlMeta.id}`, "inbox").then(dispatch)}
      >
        Archive
      </Button>
    </Col>

    <Col xs="1" class="align-left">
      {#if !emlMeta.tags.includes("spam")}
        <Button
          on:click={() =>
            api.applyTag(`id:${emlMeta.id}`, "spam").then(dispatch)}
        >
          Spam
        </Button>
      {:else}
        <Button
          on:click={() => api.rmTag(`id:${emlMeta.id}`, "spam").then(dispatch)}
        >
          Not spam
        </Button>
      {/if}
    </Col>

    <Col xs="1" class="align-left">
      <Button class="" on:click={() => (replyModalOpen = true)}>Reply</Button>
      <EmlReplyModal {emlMeta} emlBody={body} bind:isOpen={replyModalOpen} />
    </Col>
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
