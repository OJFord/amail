<script>
  import { faArrowLeft } from "@fortawesome/free-solid-svg-icons/faArrowLeft";
  import { faArrowRight } from "@fortawesome/free-solid-svg-icons/faArrowRight";
  import Icon from "fa-svelte";
  import {
    onMount,
    //
  } from "svelte";
  import { Document, Page } from "svelte-pdfjs";

  export let b64Data;

  $: docUrl = `data:application/pdf;charset=US-ASCII;base64,${b64Data}`;
  let pageNumber = 1;
  let scale = 2;

  $: atBeginning = pageNumber == 1;
  $: atEnd = false; // TODO: no way to tell?

  const handleKey = (ev) => {
    switch (ev.key) {
      case "ArrowLeft":
        pageNumber -= atBeginning ? 0 : 1;
        break;
      case "ArrowRight":
        pageNumber += atEnd ? 0 : 1;
        break;
    }
  };
</script>

<svelte:window on:keydown={handleKey} />

<Document file={docUrl}>
  <div>
    <Page {scale} num={pageNumber} />

    {#if pageNumber > 1}
      <span
        class="page-turn-btn page-turn-left"
        on:click={() => handleKey({ key: "ArrowLeft" })}
      >
        <Icon icon={faArrowLeft} />
      </span>
    {/if}

    <span
      class="page-turn-btn page-turn-right"
      on:click={() => handleKey({ key: "ArrowRight" })}
    >
      <Icon icon={faArrowRight} />
    </span>
  </div>
</Document>

<style>
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

    font-size: 16em;
    opacity: 0.5;
  }

  .page-turn-btn:hover {
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
