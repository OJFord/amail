<script>
  import {
    faArrowLeft,
    faArrowRight,
  } from "@fortawesome/free-solid-svg-icons"
  import {
    FontAwesomeIcon,
  } from "@fortawesome/svelte-fontawesome"
  import {
    Document,
    Page,
  } from "svelte-pdfjs"
  import {
    set_pdfjs_context,
  } from "svelte-pdfjs/vite"

  let {
    b64Data,
  } = $props()

  set_pdfjs_context()

  let docUrl = $derived(
    `data:application/pdf;charset=US-ASCII;base64,${b64Data}`,
  )
  let doc = $state()
  let pageNumber = $state(1)
  const scale = 2

  let atBeginning = $derived(pageNumber == 1)
  let atEnd = $derived(pageNumber == doc?.numPages)

  const handleKey = (ev) => {
    switch (ev.key) {
    case "ArrowLeft":
      pageNumber -= atBeginning ? 0 : 1
      break
    case "ArrowRight":
      pageNumber += atEnd ? 0 : 1
      break
    }
  }

  const loadOptions = {
    isEvalSupported: false,
  }
</script>

<svelte:window onkeydown={handleKey} />

<Document
  file={docUrl}
  {loadOptions}
  on:loadsuccess={(ev) => (doc = ev.detail)}
>
  <div>
    <Page {scale} num={pageNumber} />

    {#if !atBeginning}
      <button
        class="page-turn-btn page-turn-left"
        onclick={() => handleKey({
          key: "ArrowLeft",
        })}
      >
        <FontAwesomeIcon icon={faArrowLeft} size="6x" />
      </button>
    {/if}

    {#if !atEnd}
      <button
        class="page-turn-btn page-turn-right"
        onclick={() => handleKey({
          key: "ArrowRight",
        })}
      >
        <FontAwesomeIcon icon={faArrowRight} size="6x" />
      </button>
    {/if}
  </div>
</Document>

<style lang="scss">
  div {
    display: grid;
    place-items: center;
  }

  .page-turn-btn {
    cursor: pointer;

    position: absolute;
    height: 100%;
    width: 50%;
    padding: 10%;
    border: 0;

    opacity: 0;
  }

  .page-turn-btn:hover {
    opacity: 0.5;
    background-color: rgba(255, 255, 255, 30);
  }

  .page-turn-left {
    left: 0;
    text-align: left;
  }

  .page-turn-right {
    right: 0;
    text-align: right;
  }
</style>
