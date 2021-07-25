<script>
  import { createEventDispatcher } from "svelte";
  import {
    Button,
    Col,
    Modal,
    ModalHeader,
    ModalBody,
    ModalFooter,
    Row,
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  export let isOpen;
  export let tag;
  export let query;

  const dispatch = createEventDispatcher();

  const applyTag = (query, tag) =>
    tauri
      .invoke("apply_tag", { query, tag })
      .then(() => dispatch("retagComplete", { query, tag }));

  const countMatches = (query) => tauri.invoke("count_matches", { query });

  const rmTag = (query, tag) =>
    tauri
      .invoke("rm_tag", { query, tag })
      .then(() => dispatch("retagComplete"));

  const toggle = () => (isOpen = !isOpen);
</script>

<Modal {isOpen} {toggle}>
  <ModalHeader {toggle}><h3>Tag query</h3></ModalHeader>

  <ModalBody>
    <Row>
      <p>
        There are {#await countMatches(query)}...{:then n}
          <mark class="info">{n}</mark>
        {/await} results for the selected query; of which {#await countMatches(`(${query}) and tag:${tag}`)}...{:then n}
          <mark class="info">{n}</mark>
        {/await} are currently tagged <code>{tag}</code>.
      </p>
      <p>The selected query is:</p>
      <pre>{query}</pre>
    </Row>

    <Row>
      <Col style="text-align: center;">
        <span class="tag-modal-button">
          <Button color="success" on:click={applyTag(query, tag).then(toggle)}>
            Apply to all
          </Button>
        </span>

        <span class="tag-modal-button">
          <Button color="danger" on:click={rmTag(query, tag).then(toggle)}>
            Remove from all
          </Button>
        </span>
      </Col>
    </Row>
  </ModalBody>

  <ModalFooter>
    <Button color="secondary" on:click={toggle}>Cancel</Button>
  </ModalFooter>
</Modal>
