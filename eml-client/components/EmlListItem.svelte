<script>
  export let emlMeta;

  import {
    Col,
    Container,
    Row,
    //
  } from "sveltestrap";

  import RelativeDate from "./RelativeDate.svelte";

</script>

<Container fluid>
  <Row>
    <h3 class:strong={emlMeta.tags.includes("unread")}>
      {emlMeta.subject}
    </h3>
  </Row>

  <Row class="h4 text-muted">
    <Col xs="3">
      <!-- UNIX timestamp *1000 to get ms -->
      <RelativeDate date={emlMeta.timestamp * 1000} />
    </Col>

    <Col>
      {emlMeta.from.name}
    </Col>
  </Row>
</Container>
