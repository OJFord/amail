<script context="module">
  export const formatMailAddr = (m) =>
    m.address
      ? m.name
        ? `${m.name} <${m.address}>`
        : m.address
      : `${m.name}: ${m.members.map(formatMailAddr).join(", ")};`;

  export const parseAddr = (addrList) =>
    addrList
      ? addrList
          .split(",")
          .map(
            (addr) =>
              addr.match(/((?<name>.+)(?: <))?(?<address>.+@[^>]+)>?/).groups
          )
      : [];
</script>

<script>
  import {
    Input,
    //
  } from "sveltestrap";

  export let emlMeta;
  export let editable = false;

  $: from = emlMeta.from.map(formatMailAddr).join(", ");
  $: to = emlMeta.to.map(formatMailAddr).join(", ");
  $: cc = emlMeta.cc ? emlMeta.cc.map(formatMailAddr).join(", ") : "";
  $: bcc = emlMeta.bcc ? emlMeta.bcc.map(formatMailAddr).join(", ") : "";
</script>

<h4>
  <i>From:</i>
  {#if editable}
    <Input
      name="from"
      value={from}
      on:change={(e) => (emlMeta.from = parseAddr(e.target.value))}
    />
  {:else}
    {from}
  {/if}
</h4>

{#if emlMeta.sender && !from.includes(formatMailAddr(emlMeta.sender))}
  <h4>
    <i>Sent by:</i>
    {formatMailAddr(emlMeta.sender)}
  </h4>
{/if}

{#if emlMeta.received_by && !to.includes(formatMailAddr(emlMeta.received_by))}
  <h4>
    <i>Received by:</i>
    {formatMailAddr(emlMeta.received_by)}
  </h4>
{/if}

{#if emlMeta.to}
  <h4>
    <i>To:</i>
    {#if editable}
      <Input
        name="to"
        value={to}
        on:change={(e) => (emlMeta.to = parseAddr(e.target.value))}
      />
    {:else}
      {to}
    {/if}
  </h4>
{/if}

{#if emlMeta.cc}
  <h4>
    <i>Cc:</i>
    {#if editable}
      <Input
        name="cc"
        value={cc}
        on:change={(e) => (emlMeta.cc = parseAddr(e.target.value))}
      />
    {:else}
      {cc}
    {/if}
  </h4>
{/if}

{#if emlMeta.bcc}
  <h4>
    <i>Bcc:</i>
    {#if editable}
      <Input
        name="bcc"
        value={bcc}
        on:change={(e) => (emlMeta.bcc = parseAddr(e.target.value))}
      />
    {:else}
      {bcc}
    {/if}
  </h4>
{/if}
