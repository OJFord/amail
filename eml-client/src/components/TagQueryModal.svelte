<script>
  import {
    createEventDispatcher,
  } from "svelte"
  import {
    Button,
    Col,
    Modal,
    ModalBody,
    ModalFooter,
    ModalHeader,
    Row,
  } from "@sveltestrap/sveltestrap"

  import * as api from "../api.js"

  export let isOpen
  export let tag
  export let query

  const dispatch = createEventDispatcher()

  const toggle = () => (isOpen = !isOpen)
  const onRetag = (event, detail) => {
    dispatch(event, detail)
    toggle()
  }
</script>

<Modal {isOpen} {toggle}>
  <ModalHeader {toggle}><h3>Tag query</h3></ModalHeader>

  <ModalBody>
    <Row>
      <p>
        There are {#await api.countMatches(query)}...{:then n}
          <mark class="info">{n}</mark>
        {/await} results for the selected query; of which {#await api.countMatches(`(${query}) and tag:${tag}`)}...{:then n}
          <mark class="info">{n}</mark>
        {/await} are currently tagged <code>{tag}</code>.
      </p>
      <p>The selected query is:</p>
      <pre>{query}</pre>
    </Row>

    <Row>
      <Col style="text-align: center;">
        <span class="tag-modal-button">
          <Button
            color="success"
            on:click={api.applyTag(query, tag)
              .then(onRetag)}
          >
            Apply to all
          </Button>
        </span>

        <span class="tag-modal-button">
          <Button color="danger" on:click={api.rmTag(query, tag)
            .then(onRetag)}>
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
