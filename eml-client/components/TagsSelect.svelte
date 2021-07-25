<script>
  import Icon from "fa-svelte";
  import { faTags } from "@fortawesome/free-solid-svg-icons/faTags";
  import { createEventDispatcher } from "svelte";
  import {
    Dropdown,
    DropdownItem,
    DropdownMenu,
    DropdownToggle,
    //
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  const dispatch = createEventDispatcher();

  let tags = [];
  tauri
    .invoke("list_tags")
    .then((tagList) => (tags = tagList))
    .catch(console.error);
</script>

<Dropdown>
  <DropdownToggle caret>
    <Icon icon={faTags} />
  </DropdownToggle>
  <DropdownMenu>
    {#each tags as tag}
      <DropdownItem
        on:click={() => {
          dispatch("tagSelected", tag);
        }}
      >
        {tag}
      </DropdownItem>
    {/each}
  </DropdownMenu>
</Dropdown>
