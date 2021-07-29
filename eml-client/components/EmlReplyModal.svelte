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

  let replyMeta;
  const refreshMeta = () =>
    (replyMeta = {
      from: emlMeta.to,
      to: emlMeta.reply_to ? emlMeta.reply_to : emlMeta.from,
      cc: emlMeta.cc || [],
      bcc: emlMeta.bcc || [],
      subject: emlMeta.subject,
    });
  $: isOpen, refreshMeta();

  // prettier-ignore
  $: replyEml = dedent`
    Message-ID: <${new Date().toISOString()}.${emlMeta.thread_id}.${replyMeta.from.address}>
    From: ${replyMeta.from.map(formatMailAddr).join(",")}
    To: ${replyMeta.to.map(formatMailAddr).join(",")}
    Cc: ${replyMeta.cc.map(formatMailAddr).join(",")}
    Bcc: ${replyMeta.bcc.map(formatMailAddr).join(",")}
    In-Reply-To: ${emlMeta.id}
    References: ${emlMeta.references || ""} ${emlMeta.id}
    Subject: ${replyMeta.subject}

    ${body}
  `;

  $: ogDate = new Date(emlMeta.timestamp * 1000).toLocaleString("en-GB", {
    weekday: "short",
    day: "numeric",
    month: "short",
    year: "numeric",
    hour: "numeric",
    minute: "numeric",
    second: "numeric",
  });

  $: ogPlain = emlBody
    ? [emlBody]
        .concat(emlBody.alternatives)
        .filter((a) => a.mimetype.valueOf() == "text/plain")
        .map((a) => a.content)[0]
    : null;

  $: body =
    "\r\n\r\n" +
    dedent`
      On ${ogDate} (GMT), ${emlMeta.from
      .map((m) => m.address)
      .join(" & ")} wrote:
      ${
        ogPlain
          ? ogPlain
              .split(/\n/)
              .map((l) => `> ${l}`)
              .join("\n")
          : "[no plaintext]"
      }
    `;

  const toggle = () => (isOpen = !isOpen);

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
  <ModalHeader {toggle}>
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
