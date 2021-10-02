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

  import * as api from "../api.js";

  const dispatch = createEventDispatcher();

  let tags = [];
  api.tagList().then((ts) => (tags = ts));
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
