<script>
  import {
    Badge,
    //
  } from "sveltestrap";

  export let tags;

  const rgbForTag = (tag) => {
    let n = tag.split("").reduce((acc, c) => acc + c.charCodeAt(0), 0) % 256;
    return `rgb(${n}, ${n}, ${n})`;
  };
</script>

<h3 class="tags">
  {#each tags as tag}
    <span class="tag">
      <Badge pill color="" style={`background-color: ${rgbForTag(tag)};`}>
        {tag}
      </Badge>
    </span>
  {/each}
</h3>

<style lang="scss">
  .tags {
    overflow: hidden;
    white-space: nowrap;
  }
</style>
