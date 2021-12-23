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
    Modal,
    ModalBody,
    ModalHeader,
    Row,
    //
  } from "sveltestrap";
  import PdfAttachmentViewer from "./PdfAttachmentViewer.svelte";
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

  let previewOpen = false;
  const previewToggle = () => (previewOpen = !previewOpen);
</script>

<svelte:window
  on:keydown={(ev) => {
    if (ev.key == "Escape") previewOpen = false;
  }}
/>

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
          <Button on:click={previewToggle} outline>
            <Icon icon={faEye} />
          </Button>

          {#if part.mimetype == "application/pdf"}
            <Modal
              isOpen={previewOpen}
              fullscreen={true}
              contentClassName="bg-transparent"
            >
              <ModalHeader toggle={previewToggle} class="bg-secondary">
                {part.filename}
              </ModalHeader>
              <ModalBody>
                <PdfAttachmentViewer b64Data={part.content_base64} />
              </ModalBody>
            </Modal>
          {/if}
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
