<script>
  import {
    Button,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
  } from "sveltestrap"

  import * as api from "../api.js"
  import EmlCompose from "./EmlCompose.svelte"

  export let isOpen

  let attachments
  let body
  let confirm
  let emlMeta

  const init = () => {
    emlMeta = {
      from: [],
      id: "",
      id_thread: "",
      tags: [],
      timestamp: Number((Date.now() / 1000).toFixed()),
      to: [],
    }
    body = ""
    attachments = []
    confirm = null
  }

  init()

  const toggle = () => {
    isOpen = !isOpen
    return Promise.resolve(isOpen)
  }

  const toggleConfirm = async () => {
    if (confirm) {
      confirm = null
    } else {
      confirm = await api.previewEml(emlMeta, body, attachments)
    }
  }

  const send = () => api.sendEml(emlMeta, body, attachments)
    .then(toggle)
    .then(init)
</script>

<Modal {isOpen} class="modal-lg" scrollable>
  <ModalHeader
    toggle={() => toggle()
      .then((open) => (open ? null : (confirm = false)))}
  >
    {#if confirm}Sure?{/if}
  </ModalHeader>
  <ModalBody>
    {#if confirm}
      <pre>{confirm}</pre>
    {:else}
      <EmlCompose bind:emlMeta bind:body bind:attachments />
    {/if}
  </ModalBody>

  <ModalFooter>
    <Button on:click={confirm ? send : toggleConfirm}>Send</Button>
  </ModalFooter>
</Modal>
