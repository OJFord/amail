<script>
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
  {@html part.content}
{:else if part.mimetype == "text/plain"}
  {#each part.content.split(/\r\n\r\n/) as para}
    <pre>{para}</pre>
  {/each}
{:else}
  {#if part.size}{friendlySize(part.size)}of{/if}
  <em>{part.mimetype}</em> content
{/if}

<style scoped lang="scss">
  pre {
    overflow-wrap: break-word;
    white-space: pre-wrap;
  }
</style>
