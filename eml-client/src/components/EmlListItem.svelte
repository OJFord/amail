<script>
  import {
    Col,
    Row,
  } from "@sveltestrap/sveltestrap"

  import RelativeDate from "./RelativeDate.svelte"
  import TagBadges from "./TagBadges.svelte"

  let {
    emlMeta,
    hideTags = new Set(),
    //
  } = $props()

  const mailboxLabel = (mailbox) => mailbox.name || mailbox.address

  const sentRecipientsSummary = (emlMeta) => {
    const recipients = [
      ...(emlMeta.to ?? []),
      ...(emlMeta.cc ?? []),
      ...(emlMeta.bcc ?? []),
    ]
      .flatMap((addr) => addr.members || [
        addr,
      ])
      .map(mailboxLabel)

    return recipients.length > 1
      ? `${recipients[0]} & ${recipients.length - 1} others`
      : recipients[0]
  }

  $effect(() => {
    hideTags.add("unread")
  })
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
      {#if emlMeta.tags.includes("sent")}
        {sentRecipientsSummary(emlMeta)}
      {:else}
        {#if emlMeta.sender}
          {#if emlMeta.from.map((m) => m.name)
            .includes(emlMeta.sender.name)}
            {emlMeta.sender.address}
          {:else}
            {mailboxLabel(emlMeta.sender)}
          {/if}
          <small><i>on behalf of</i></small>
        {/if}
        {emlMeta.from.map(mailboxLabel)
          .join(", ")}
      {/if}
    </Col>
  </Row>
</div>
