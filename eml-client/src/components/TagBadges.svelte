<script>
  import {
    createEventDispatcher,
  } from "svelte"
  import {
    Badge,
  } from "@sveltestrap/sveltestrap"

  import * as api from "../api.js"

  let {
    tags,
    query = null,
  } = $props()

  const dispatch = createEventDispatcher()

  const rgbForTag = (tag) => {
    const n = tag.split("")
      .reduce((acc, c) => acc + c.charCodeAt(0), 0) % 256
    return `rgb(${n}, ${n}, ${n})`
  }

  const removeTag = (tag) => api.rmTag(query, tag)
    .then(() => dispatch("tagsUpdated", {
      tag,
    }))
</script>

<h3 class="tags">
  {#each tags as tag}
    <span class="tag">
      <Badge
        pill
        color=""
        class={query != null ? "tag-badge-removable" : ""}
        style={`background-color: ${rgbForTag(tag)};`}
      >
        {tag}
        {#if query != null}
          <button
            type="button"
            class="tag-remove"
            aria-label={`Remove tag ${tag}`}
            title={`Remove tag ${tag}`}
            on:click|stopPropagation|preventDefault={() => removeTag(tag)}
          >
            ×
          </button>
        {/if}
      </Badge>
    </span>
  {/each}
</h3>

<style lang="scss">
  .tags {
    overflow: hidden;
    white-space: nowrap;
  }

  .tag {
    display: inline-block;
    margin-right: 0.25rem;
  }

  :global(.tag-badge-removable) {
    padding-right: 1.4rem;
    position: relative;
  }

  .tag-remove {
    align-items: center;
    background: rgba(255, 255, 255, 0.9);
    border: 0;
    border-radius: 999px;
    color: #000;
    display: inline-flex;
    font-size: 0.75rem;
    height: 1rem;
    justify-content: center;
    line-height: 1;
    opacity: 0;
    padding: 0;
    pointer-events: none;
    position: absolute;
    right: -0.2rem;
    top: -0.2rem;
    transition: opacity 0.15s ease-in-out;
    width: 1rem;
  }

  .tag:hover .tag-remove,
  .tag .tag-remove:focus {
    opacity: 1;
    pointer-events: auto;
  }
</style>
