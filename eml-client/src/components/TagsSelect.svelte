<script>
  import {
    FontAwesomeIcon,
  } from "@fortawesome/svelte-fontawesome"
  import {
    faTags,
  } from "@fortawesome/free-solid-svg-icons"
  import {
    createEventDispatcher,
  } from "svelte"
  import {
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
  } from "sveltestrap"

  import * as api from "../api.js"

  const dispatch = createEventDispatcher()

  let tags = []
  api.tagList()
    .then((ts) => (tags = ts))
</script>

<Dropdown>
  <DropdownToggle caret>
    <FontAwesomeIcon icon={faTags} />
  </DropdownToggle>
  <DropdownMenu class="vh-50 overflow-scroll">
    {#each tags as tag}
      <DropdownItem
        on:click={() => {
          dispatch("tagSelected", tag)
        }}
      >
        {tag}
      </DropdownItem>
    {/each}
  </DropdownMenu>
</Dropdown>
