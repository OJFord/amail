<script>
  import * as dialog from "@tauri-apps/api/dialog"
  import * as fs from "@tauri-apps/api/fs"
  import * as path from "@tauri-apps/api/path"
  import {
    FontAwesomeIcon,
  } from "@fortawesome/svelte-fontawesome"
  import {
    faEye,
    faFileDownload,
  } from "@fortawesome/free-solid-svg-icons"
  import {
    Button,
    Card,
    CardBody,
    CardFooter,
    CardHeader,
    Col,
    Modal,
    ModalBody,
    ModalHeader,
    Row,
  } from "sveltestrap"
  import PdfAttachmentViewer from "./PdfAttachmentViewer.svelte"
  import EmlBodyPart from "./EmlBodyPart.svelte"

  export let part

  const previewable = (mimetype) => [
    "application/pdf",
  ].some((m) => m == mimetype)

  const save = async () => path
    .downloadDir()
    .then((downloadDir) => dialog.save({
      defaultPath: `${downloadDir}/${part.filename}`,
    }),
    )
    .then((selectedPath) => path
      .homeDir()
      .then((homeDir) => selectedPath?.replace(new RegExp(`^${homeDir}`), ""),
      ),
    )
    .then((path) => {
      if (path) {
        return fs.writeBinaryFile(
          {
            contents: part.content_encoded,
            path,
          },
          {
            dir: 11,
          },
        )
      }
      // Else cancelled, that's ok
    })
    .catch((e) => console.error(`failed to save attachment: ${e}`))

  let previewOpen = false
  const previewToggle = () => (previewOpen = !previewOpen)
</script>

<svelte:window
  on:keydown={(ev) => {
    if (ev.key == "Escape") {
      previewOpen = false
    }
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
          {#if previewable(part.mimetype)}
            <Button on:click={previewToggle} outline>
              <FontAwesomeIcon icon={faEye} />
            </Button>

            <Modal
              isOpen={previewOpen}
              fullscreen={true}
              contentClassName="bg-transparent"
            >
              <ModalHeader toggle={previewToggle} class="bg-secondary">
                {part.filename}
              </ModalHeader>
              <ModalBody>
                {#if part.mimetype == "application/pdf"}
                  <PdfAttachmentViewer b64Data={part.content_base64} />
                {/if}
              </ModalBody>
            </Modal>
          {/if}
        </Col>

        <Col class="d-flex justify-content-center">
          <Button on:click={save} outline>
            <FontAwesomeIcon icon={faFileDownload} />
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
