<script>
  import * as dialog from "@tauri-apps/api/dialog";
  import * as fs from "@tauri-apps/api/fs";
  import * as path from "@tauri-apps/api/path";
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

  const save = () => {
    return path
      .downloadDir()
      .then((downloadDir) =>
        dialog.save({
          defaultPath: `${downloadDir}/${part.filename}`,
        })
      )
      .then((path) => {
        if (path)
          return fs.writeBinaryFile({
            contents: part.content_encoded,
            path,
          });
        // else cancelled, that's ok
      })
      .catch(() => console.error("failed to save attachment"));
  };
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
          <Button on:click={save} outline>
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
