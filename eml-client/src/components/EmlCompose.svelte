<script>
  import {
    FontAwesomeIcon,
  } from "@fortawesome/svelte-fontawesome"
  import {
    faPlus,
    faXmark,
  } from "@fortawesome/free-solid-svg-icons"
  import {
    Button,
    Col,
    Container,
    Input,
    Row,
  } from "@sveltestrap/sveltestrap"
  import * as dialog from "@tauri-apps/plugin-dialog"

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
      .then((f) => {
        attachments[attachments.length] = {
          name: f.name,
          path: f.path,
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
          <FontAwesomeIcon icon={faXmark} />
        </Button>
      </Col>
    </Row>
  {/each}
  <Row>
    <Button on:click={() => addAttachment()}>
      <FontAwesomeIcon icon={faPlus} on:click={() => addAttachment()} />
    </Button>
  </Row>
</Container>
