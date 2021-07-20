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

  import RelativeDate from "../components/RelativeDate.svelte";
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
    {#each emls as [eml, id]}
      <ListGroupItem
        tag="a"
        href="#"
        on:click={() =>
          dispatch("view", { page: Single, props: { id, emlMeta: eml } })}
      >
        <Container fluid>
          <Row>
            <h3>{eml.subject}</h3>
          </Row>

          <Row>
            <Col xs="3">
              <!-- UNIX timestamp *1000 to get ms -->
              <RelativeDate date={eml.timestamp * 1000} />
            </Col>

            <Col>
              {eml.from.name}
            </Col>
          </Row>
        </Container>
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}
