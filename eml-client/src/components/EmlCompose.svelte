<script>
  import Icon from "fa-svelte"
  import {
    faXmark,
  } from "@fortawesome/free-solid-svg-icons/faXmark"
  import {
    faPlus,
  } from "@fortawesome/free-solid-svg-icons/faPlus"
  import {
    Button,
    Col,
    Container,
    Input,
    Row,
  } from "sveltestrap"
  import * as dialog from "@tauri-apps/api/dialog"

  import * as api from "../api.js"
  import EmlAddresses from "./EmlAddresses.svelte"

  export let attachments
  export let body
  export let emlMeta

  let sysName
  api.getName()
    .then((name) => (sysName = name))

  $: if (sysName && emlMeta.from.filter((f) => !f.name)) {
    emlMeta.from = emlMeta.from.map((from) => from.name
      ? from
      : {
        ...from,
        name: sysName,
      },
    )
  }

  const removeAttachment = (attachment) => (attachments = attachments.filter((a) => a.path != attachment.path))

  const addAttachment = () => {
    dialog.open()
      .then((path) => {
        attachments[attachments.length] = {
          name: path.replace(/.*\//, ""),
          path,
        }
      })
  }
</script>

<EmlAddresses bind:emlMeta editable={true} />

<h4>
  <i>Subject:</i>
  <Input name="subject" bind:value={emlMeta.subject} />
</h4>

<Input type="textarea" name="body" bind:value={body} rows="25" />

<Container>
  {#each attachments as attachment}
    <Row>
      <Col xs="10">
        <Input bind:value={attachment.name} />
      </Col>
      <Col>
        <Button on:click={() => removeAttachment(attachment)}>
          <Icon icon={faXmark} />
        </Button>
      </Col>
    </Row>
  {/each}
  <Row>
    <Button on:click={() => addAttachment()}>
      <Icon icon={faPlus} on:click={() => addAttachment()} />
    </Button>
  </Row>
</Container>
