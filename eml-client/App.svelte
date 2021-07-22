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
        <NavItem>
          <NavLink active on:click={() => {}}>
            <Icon icon={faStream} />
            <h2><span>tag:inbox</span></h2>
          </NavLink>
        </NavItem>
      </Nav>
    </Col>

    <Col xs="4" class="mh-100 scroll">
      <EmlList bind:emlSelected />
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
