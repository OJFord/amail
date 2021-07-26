<script>
  export let part;
</script>

{#if part.is_cleaned_html}
  {@html part.content}
{:else if part.mimetype == "text/plain"}
  {#each part.content.split(/\r\n\r\n/) as para}
    <p>{para}</p>
  {/each}
{:else}
  [Unimplemented {part.mimetype} content]
{/if}
