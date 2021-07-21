<script>
  import Icon from "fa-svelte";
  import { faStream } from "@fortawesome/free-solid-svg-icons/faStream";
  import {
    Col,
    Container,
    Nav,
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
    <NavbarBrand>EML Viewer</NavbarBrand>
  </Navbar>
</div>

<Container fluid>
  <Row>
    <div id="side">
      <Nav class="sticky-top" vertical>
        <NavLink
          on:click={() => {
            page = List;
            pageProps = {};
          }}
        >
          <Icon icon={faStream} />
        </NavLink>
      </Nav>
    </div>

    <Col>
      <div id="page">
        <svelte:component
          this={page}
          on:view={(event) => {
            page = event.detail.page;
            pageProps = event.detail.props;
          }}
          {...pageProps}
        />
      </div>
    </Col>
  </Row>
</Container>

<style scoped>
  #page {
  }

  #side {
    top: 5rem;
  }

  #top {
    margin-bottom: 1rem;
  }

</style>
