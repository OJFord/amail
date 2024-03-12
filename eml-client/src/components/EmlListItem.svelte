<script>
  export let emlMeta
  export let hideTags = new Set()
  $: hideTags.add("unread")

  import {
    Col,
    Row,
  } from "@sveltestrap/sveltestrap"

  import RelativeDate from "./RelativeDate.svelte"
  import TagBadges from "./TagBadges.svelte"
</script>

<div>
  <Row>
    <Col>
      <h3>
        {#if emlMeta.tags.includes("unread")}
          <strong>{emlMeta.subject}</strong>
        {:else}
          {emlMeta.subject}
        {/if}
      </h3>
    </Col>

    <Col xs="3">
      <TagBadges tags={emlMeta.tags.filter((t) => !hideTags.has(t))} />
    </Col>
  </Row>

  <Row class="h4 text-muted">
    <Col xs="3">
      <!-- UNIX timestamp *1000 to get ms -->
      <RelativeDate date={emlMeta.timestamp * 1000} />
    </Col>

    <Col>
      {#if emlMeta.sender}
        {#if emlMeta.from.map((m) => m.name)
          .includes(emlMeta.sender.name)}
          {emlMeta.sender.address}
        {:else}
          {emlMeta.sender.name}
        {/if}
        <small><i>on behalf of</i></small>
      {/if}
      {emlMeta.from.map((m) => m.name)
        .join(", ")}
    </Col>
  </Row>
</div>
