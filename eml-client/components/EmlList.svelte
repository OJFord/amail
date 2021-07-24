<script>
  import {
    ListGroup,
    ListGroupItem,
    Spinner,
    //
  } from "sveltestrap";

  import * as tauri from "@tauri-apps/api/tauri";

  import EmlListItem from "./EmlListItem.svelte";

  export let emlSelected = null;
  export let query;

  let emls = null;
  $: tauri
    .invoke("list_eml", { query })
    .then((emlList) => {
      emls = emlList;
    })
    .catch(console.error);

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
        color={emlSelected && emlSelected == emlMeta.Ok ? "secondary" : ""}
      >
        {#if emlMeta.Ok}
          <EmlListItem emlMeta={emlMeta.Ok} />
        {:else}
          <div class="bg-error">
            {#if emlMeta.Err.id}
              <h4>{emlMeta.Err.id}</h4>
            {/if}
            Failed to parse: {emlMeta.Err.reason}
          </div>
        {/if}
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}

<style scoped>
</style>
