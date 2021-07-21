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

  import List from "./routes/List.svelte";

  let page = List;
  let pageProps = {};

</script>

<div id="top">
  <Navbar class="sticky-top" color="primary">
    <NavbarBrand><h1>Amail</h1></NavbarBrand>
  </Navbar>
</div>

<Container fluid>
  <Row>
    <Col xs="1">
      <Nav vertical pills>
        <NavItem>
          <NavLink
            active
            on:click={() => {
              page = List;
              pageProps = {};
            }}
          >
            <Icon icon={faStream} />
            <h2>tag:inbox</h2>
          </NavLink>
        </NavItem>
      </Nav>
    </Col>

    <Col>
      <Container fluid>
        <svelte:component
          this={page}
          on:view={(event) => {
            page = event.detail.page;
            pageProps = event.detail.props;
          }}
          {...pageProps}
        />
      </Container>
    </Col>
  </Row>
</Container>

<style scoped>
  #top {
    margin-bottom: 1rem;
  }

</style>
