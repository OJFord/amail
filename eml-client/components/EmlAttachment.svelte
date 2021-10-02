<script>
  import Icon from "fa-svelte";
  import { faEye } from "@fortawesome/free-solid-svg-icons/faEye";
  import { faFileDownload } from "@fortawesome/free-solid-svg-icons/faFileDownload";
  import {
    Button,
    Card,
    CardHeader,
    CardBody,
    CardFooter,
    Col,
    Row,
    //
  } from "sveltestrap";
  import EmlBodyPart from "./EmlBodyPart.svelte";

  export let part;

  const href = `data:${part.mimetype};charset=utf-8,${encodeURIComponent(
    part.content_encoded
  )}`;
</script>

<span class="attachment">
  <Card>
    <CardHeader>
      {part.filename || "Unnamed attachment"}
    </CardHeader>

    <CardBody>
      <EmlBodyPart {part} />
    </CardBody>

    <CardFooter>
      <Row>
        <Col class="d-flex justify-content-center">
          <Button outline>
            <Icon icon={faEye} />
          </Button>
        </Col>

        <Col class="d-flex justify-content-center">
          <!-- not yet implemented: https://github.com/tauri-apps/wry/issues/349 -->
          <Button download={part.filename} {href} outline>
            <Icon icon={faFileDownload} />
          </Button>
        </Col>
      </Row>
    </CardFooter>
  </Card>
</span>

<style lang="scss" scoped>
  .attachment {
    margin: 0.5rem;
  }
</style>
