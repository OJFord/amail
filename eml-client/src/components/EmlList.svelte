<script>
  import {
    ListGroup,
    ListGroupItem,
    Spinner,
  } from "@sveltestrap/sveltestrap"

  import * as api from "../api.js"
  import EmlListItem from "./EmlListItem.svelte"

  export let emlSelected = null
  export let hideTags = new Set()
  export let query

  let emls = null
  $: api.listEml(query)
    .then((emlList) => (emls = emlList))
</script>

{#if emls == null}
  <Spinner primary />
{:else}
  <ListGroup flush>
    {#each emls as emlMeta}
      <ListGroupItem
        tag="a"
        href="#"
        on:click={() => (emlSelected = emlMeta.Ok)}
        color={emlMeta.Err
          ? "warning"
          : emlSelected && emlSelected.id == emlMeta.Ok.id
            ? "secondary"
            : ""}
      >
        {#if emlMeta.Ok}
          <EmlListItem emlMeta={emlMeta.Ok} {hideTags} />
        {:else}
          <div>
            {#if emlMeta.Err.id}
              <h4>{emlMeta.Err.id}</h4>
            {/if}
            <span>
              Parsing failed
              {#if emlMeta.Err.within} in {emlMeta.Err.within}{/if}
              - {emlMeta.Err.reason}
            </span>
          </div>
        {/if}
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}

<style scoped>
</style>
