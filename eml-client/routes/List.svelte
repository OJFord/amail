<script>
  import { createEventDispatcher } from "svelte";
  import {
    ListGroup,
    ListGroupItem,
    Col,
    Container,
    Row,
    Spinner,
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  import EmlListItem from "../components/EmlListItem.svelte";
  import Single from "./Single.svelte";

  const dispatch = createEventDispatcher();

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
        on:click={() => dispatch("view", { page: Single, props: { emlMeta } })}
      >
        <EmlListItem {emlMeta} />
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}
