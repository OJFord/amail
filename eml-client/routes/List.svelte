<script>
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
    {#each emls as eml}
      <ListGroupItem tag="a" href="#">
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
              {eml.author}
            </Col>
          </Row>
        </Container>
      </ListGroupItem>
    {/each}
  </ListGroup>
{/if}
