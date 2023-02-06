<script>
  import {
    createEventDispatcher,
  } from "svelte"
  import {
    Button,
    Col,
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
    Row,
    Spinner,
  } from "sveltestrap"

  import * as api from "../api.js"
  import EmlAddresses from "./EmlAddresses.svelte"
  import EmlAttachment from "./EmlAttachment.svelte"
  import EmlBodyPart from "./EmlBodyPart.svelte"
  import EmlReplyModal from "./EmlReplyModal.svelte"
  import TagBadges from "./TagBadges.svelte"
  import VCalSummary from "./VCalSummary.svelte"

  export let emlMeta

  const dispatch = createEventDispatcher()

  let body = null
  let selectedAlt = null

  const refreshDefaultSelection = (eml) => {
    body = eml
    const altHtml = body.alternatives.filter((a) => a.is_cleaned_html)[0]
    selectedAlt = !body.is_cleaned_html && altHtml ? altHtml : body
  }

  $: api.viewEml(emlMeta.id.valueOf())
    .then(refreshDefaultSelection)

  let alts
  $: if (body) {
    alts = [
      body,
    ].concat(body.alternatives)
  }

  let attachments = []
  $: if (selectedAlt) {
    attachments = [
      selectedAlt,
    ]
      .concat(selectedAlt.extra)
      .filter((e) => e.disposition == "Attachment")
  }

  let inlines = []
  $: if (selectedAlt) {
    inlines = [
      selectedAlt,
    ]
      .concat(selectedAlt.extra)
      .filter((e) => e.disposition == "Inline")
  }

  let replyModalOpen = false

  let content
  $: if (content && emlMeta.id) {
    content.scrollTop = 0
  }
</script>

{#if body == null}
  <Spinner primary />
{:else}
  <Row style="padding-top: 1rem;">
    <EmlAddresses {emlMeta} />

    <h2>{emlMeta.subject}</h2>

    {#each alts
      .filter((a) => a.mimetype == "text/calendar")
      .map((v) => v.content) as vcal}
      <VCalSummary {vcal} primaryEventSummary={emlMeta.subject} />
    {/each}

    <span>
      {Intl.DateTimeFormat("en-GB", {
        dateStyle: "short",
        timeStyle: "long",
      })
        .format(new Date(emlMeta.timestamp * 1000))}
    </span>

    <TagBadges tags={emlMeta.tags} />
  </Row>

  <Row class="border-bottom p-1">
    <Col xs="1" class="align-left text-nowrap">
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
                  selectedAlt = alt
                }}
              >
                {alt.mimetype}
              </DropdownItem>
            {/each}
          </DropdownMenu>
        </Dropdown>
      {/if}
    </Col>

    <Col xs="1" class="align-left text-nowrap">
      {#if emlMeta.tags.includes("inbox")}
        <Button
          on:click={() => api.rmTag(`id:${emlMeta.id}`, "inbox")
            .then(dispatch)}
        >
          Archive
        </Button>
      {:else}
        <Button
          on:click={() => api.applyTag(`id:${emlMeta.id}`, "inbox")
            .then(dispatch)}
        >
          Unarchive
        </Button>
      {/if}
    </Col>

    <Col xs="1" class="align-left text-nowrap">
      {#if !emlMeta.tags.includes("spam")}
        <Button
          on:click={() => api.applyTag(`id:${emlMeta.id}`, "spam")
            .then(dispatch)}
        >
          Spam
        </Button>
      {:else}
        <Button
          on:click={() => api.rmTag(`id:${emlMeta.id}`, "spam")
            .then(dispatch)}
        >
          Not spam
        </Button>
      {/if}
    </Col>

    <Col xs="1" class="align-left text-nowrap">
      <Button class="" on:click={() => (replyModalOpen = true)}>Reply</Button>
      <EmlReplyModal {emlMeta} bind:isOpen={replyModalOpen} />
    </Col>
  </Row>

  <Row class="flex-fill mh-100 scroll" bind:inner={content}>
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
