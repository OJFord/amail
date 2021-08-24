<script>
  import dedent from "dedent";
  import {
    Button,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
    //
  } from "sveltestrap";

  import * as api from "../api.js";
  import { formatMailAddr } from "./EmlAddresses.svelte";
  import EmlCompose from "./EmlCompose.svelte";

  export let emlBody;
  export let emlMeta;
  export let isOpen;

  const rfc5322Date = (date) => date.toGMTString();

  $: ogDate = rfc5322Date(new Date(emlMeta.timestamp * 1000));

  $: ogPlain = emlBody
    ? [emlBody]
        .concat(emlBody.alternatives)
        .filter((a) => a.mimetype.valueOf() == "text/plain")
        .map((a) => a.content)[0]
    : null;

  $: bodyTemplate =
    "\r\n\r\n" +
    dedent`
      On ${ogDate}, ${emlMeta.from.map((m) => m.address).join(" & ")} wrote:
      ${
        ogPlain
          ? ogPlain
              .split(/\n/)
              .map((l) => `> ${l}`)
              .join("\n")
          : "[no plaintext]"
      }
    `;

  let body = bodyTemplate;
  let replyMeta;

  const refreshMeta = () => {
    body = bodyTemplate;
    replyMeta = {
      from: emlMeta.to,
      to: emlMeta.reply_to ? emlMeta.reply_to : emlMeta.from,
      cc: emlMeta.cc || [],
      bcc: emlMeta.bcc || [],
      subject: emlMeta.subject,
    };
  };

  $: isOpen, refreshMeta();

  // prettier-ignore
  $: replyEml = dedent`
    Message-ID: <${new Date().toISOString()}.${emlMeta.id_thread}.${replyMeta.from[0].address}>
    Date: ${rfc5322Date(new Date())}
    From: ${replyMeta.from.map(formatMailAddr).join(",")}
    To: ${replyMeta.to.map(formatMailAddr).join(",")}
    Cc: ${replyMeta.cc.map(formatMailAddr).join(",")}
    Bcc: ${replyMeta.bcc.map(formatMailAddr).join(",")}
    In-Reply-To: ${emlMeta.id}
    References: ${emlMeta.references || ""} ${emlMeta.id}
    Subject: ${replyMeta.subject}

    ${body}
  `;

  const toggle = () => {
    isOpen = !isOpen;
    return Promise.resolve(isOpen);
  };

  let confirm = false;
  const toggleConfirm = () => (confirm = !confirm);

  const send = () =>
    api
      .sendEml(
        replyMeta.to.map((e) => e.address),
        replyMeta.from.map((e) => e.address)[0],
        replyEml
      )
      .then(toggle);
</script>

<Modal {isOpen} class="modal-lg">
  <ModalHeader
    toggle={() => toggle().then((open) => (open ? null : (confirm = false)))}
  >
    {#if confirm}Sure?{:else}Reply{/if}
  </ModalHeader>
  <ModalBody>
    {#if confirm}
      <pre>
        {replyEml}
      </pre>
    {:else}
      <EmlCompose bind:emlMeta={replyMeta} bind:body />
    {/if}
  </ModalBody>

  <ModalFooter>
    <Button on:click={confirm ? send : toggleConfirm}>Send</Button>
  </ModalFooter>
</Modal>
