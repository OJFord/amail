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

  import EmlListItem from "./EmlListItem.svelte";
  import Eml from "./Eml.svelte";

  const dispatch = createEventDispatcher();

  let emls = null;
  let selected = null;
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
  <Row>
    <Col xs="4">
      <ListGroup flush>
        {#each emls as emlMeta}
          <ListGroupItem
            tag="a"
            href="#"
            on:click={() => (selected = emlMeta)}
            color={selected && selected == emlMeta ? "secondary" : ""}
          >
            <EmlListItem {emlMeta} />
          </ListGroupItem>
        {/each}
      </ListGroup>
    </Col>

    <Col class="bg-light">
      {#if selected}
        <Eml emlMeta={selected} />
      {/if}
    </Col>
  </Row>
{/if}
