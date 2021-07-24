<script>
  import Icon from "fa-svelte";
  import { faStream } from "@fortawesome/free-solid-svg-icons/faStream";
  import {
    Col,
    Container,
    Nav,
    NavItem,
    Navbar,
    NavbarBrand,
    NavLink,
    Row,
  } from "sveltestrap";

  import EmlList from "./components/EmlList.svelte";
  import Eml from "./components/Eml.svelte";

  let emlSelected = null;
  const queries = ["tag:inbox", "tag:unread"];
  let querySelected = queries[0];

</script>

<Container fluid class="h-100 d-flex flex-column">
  <Row class="flex-shrink-0">
    <Navbar color="primary">
      <NavbarBrand><h1>Amail</h1></NavbarBrand>
    </Navbar>
  </Row>

  <Row class="flex-fill" style="min-height: 0;">
    <Col xs="1" class="border">
      <Nav vertical pills>
        {#each queries as query}
          <NavItem>
            <NavLink
              active={query == querySelected}
              on:click={() => (querySelected = query)}
            >
              <Icon icon={faStream} />
              <h2><span>{query}</span></h2>
            </NavLink>
          </NavItem>
        {/each}
      </Nav>
    </Col>

    <Col xs="4" class="mh-100 scroll">
      <EmlList query={querySelected} bind:emlSelected />
    </Col>

    <Col class="bg-light mh-100 scroll">
      {#if emlSelected}
        <Eml emlMeta={emlSelected} />
      {/if}
    </Col>
  </Row>

  <Row class="flex-shrink-0">
    <Col class="mh-100" />
  </Row>
</Container>

<style scoped>
</style>
