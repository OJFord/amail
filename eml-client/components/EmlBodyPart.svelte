<script>
  import VCalSummary from "./VCalSummary.svelte";

  export let part;

  const friendlySize = (s) => {
    let si;
    for (si = 0; s > 150; si++) {
      s /= 1000;
    }
    return `${s.toFixed(1)}${["", "k", "M", "G"][si]}`;
  };
</script>

{#if part.is_cleaned_html}
  <div class="html-body">
    {@html part.content}
  </div>
{:else if part.mimetype == "text/plain"}
  {#each part.content.split(/\r\n\r\n/) as para}
    <pre>{para}</pre>
  {/each}
{:else if part.mimetype == "text/calendar"}
  <VCalSummary vcal={part.content} full={true} />
{:else}
  {#if part.size}{friendlySize(part.size)}of{/if}
  <em>{part.mimetype}</em> content
{/if}

<style scoped lang="scss">
  .html-body {
    margin: auto;
    max-width: 70rem;

    :global(blockquote) {
      background: #fafafa;
      border-left: 0.5rem solid lightgrey;
      margin: 1.5rem 0.5rem;
      padding: 0.5rem 1rem;
    }

    :global(p) {
      text-align: justify;
    }
  }

  pre {
    overflow-wrap: break-word;
    white-space: pre-wrap;
  }
</style>
