<script>
  import {
    Button,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
    //
  } from "sveltestrap";

  import * as api from "../api.js";
  import { parseAddr } from "./EmlAddresses.svelte";
  import EmlCompose from "./EmlCompose.svelte";

  export let emlMeta;
  export let isOpen;

  let confirm, headers, body, replyMeta;
  const refreshMeta = async () => {
    ({ headers, body } = await api.getReplyTemplate(emlMeta.id));
    replyMeta = {
      from: parseAddr(headers.From),
      to: parseAddr(headers.To),
      cc: parseAddr(headers.Cc) ?? [],
      bcc: parseAddr(headers.Bcc) ?? [],
      subject: headers.Subject,
    };
    confirm = null;
  };

  $: if (isOpen) {
    refreshMeta();
  } else {
    replyMeta = null;
  }
  const toggle = () => {
    isOpen = !isOpen;
    return Promise.resolve(isOpen);
  };

  const toggleConfirm = async () => {
    if (confirm) confirm = null;
    else confirm = await api.previewEml(headers, body);
  };

  const send = () => api.sendEml(headers, body).then(toggle);
</script>

<Modal {isOpen} class="modal-lg">
  <ModalHeader
    toggle={() => toggle().then((open) => (open ? null : (confirm = false)))}
  >
    {#if confirm}Sure?{:else}Reply{/if}
  </ModalHeader>
  <ModalBody>
    {#if confirm != null}
      <pre>
        {confirm}
      </pre>
    {:else if replyMeta}
      <EmlCompose bind:emlMeta={replyMeta} bind:body />
    {/if}
  </ModalBody>

  <ModalFooter>
    <Button on:click={confirm ? send : toggleConfirm}>Send</Button>
  </ModalFooter>
</Modal>
