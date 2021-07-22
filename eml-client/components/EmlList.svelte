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

  let emls = null;
  tauri
    .invoke("list_eml")
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
        on:click={() => (emlSelected = emlMeta)}
        color={emlSelected && emlSelected == emlMeta ? "secondary" : ""}
      >
        <EmlListItem {emlMeta} />
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}

<style scoped>
</style>
