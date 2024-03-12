<script>
  import {
    Button,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
  } from "@sveltestrap/sveltestrap"

  import * as api from "../api.js"
  import EmlCompose from "./EmlCompose.svelte"

  export let emlMeta
  export let isOpen

  let attachments
  let body
  let confirm
  let replyMeta

  const refreshMeta = async () => {
    attachments = []
    confirm = null
    console.debug(`getting template for reply to ${emlMeta.id}`);
    ({
      meta: replyMeta, body,
    } = await api.getReplyTemplate(emlMeta.id))
    console.debug(replyMeta)
  }

  $: if (isOpen) {
    refreshMeta()
  } else {
    replyMeta = null
  }
  const toggle = () => {
    isOpen = !isOpen
    return Promise.resolve(isOpen)
  }

  const toggleConfirm = async () => {
    if (confirm) {
      confirm = null
    } else {
      confirm = await api.previewEml(replyMeta, body, attachments)
    }
  }

  const send = () => api.sendEml(replyMeta, body, attachments)
    .then(toggle)
</script>

<Modal {isOpen} class="modal-lg" scrollable>
  <ModalHeader
    toggle={() => toggle()
      .then((open) => (open ? null : (confirm = false)))}
  >
    {#if confirm}Sure?{:else}Reply{/if}
  </ModalHeader>
  <ModalBody>
    {#if confirm != null}
      <pre>{confirm}</pre>
    {:else if replyMeta}
      <EmlCompose bind:emlMeta={replyMeta} bind:body bind:attachments />
    {/if}
  </ModalBody>

  <ModalFooter>
    <Button on:click={confirm ? send : toggleConfirm}>Send</Button>
  </ModalFooter>
</Modal>
