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
  import EmlCompose from "./EmlCompose.svelte";

  export let isOpen;

  let emlMeta = {
    from: [],
    id: "",
    id_thread: "",
    tags: [],
    timestamp: Number((Date.now() / 1000).toFixed()),
    to: [],
  };
  let body = "";
  let confirm;

  const toggle = () => {
    isOpen = !isOpen;
    return Promise.resolve(isOpen);
  };

  const toggleConfirm = async () => {
    if (confirm) confirm = null;
    else confirm = await api.previewEml(emlMeta, body);
  };

  const send = () => api.sendEml(emlMeta, body).then(toggle);
</script>

<Modal {isOpen} class="modal-lg">
  <ModalHeader
    toggle={() => toggle().then((open) => (open ? null : (confirm = false)))}
  >
    {#if confirm}Sure?{/if}
  </ModalHeader>
  <ModalBody>
    {#if confirm}
      <pre>
        {confirm}
      </pre>
    {:else}
      <EmlCompose bind:emlMeta bind:body />
    {/if}
  </ModalBody>

  <ModalFooter>
    <Button on:click={confirm ? send : toggleConfirm}>Send</Button>
  </ModalFooter>
</Modal>
