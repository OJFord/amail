<script>
  import Icon from "fa-svelte";
  import { createEventDispatcher } from "svelte";
  import { faTag } from "@fortawesome/free-solid-svg-icons/faTag";
  import {
    Badge,
    NavItem,
    NavLink,
    //
  } from "sveltestrap";

  export let name, selected, unreadCount;

  const dispatch = createEventDispatcher();
</script>

<NavItem>
  <NavLink active={selected} on:click={() => dispatch("select")}>
    <Icon icon={faTag} />
    <h2 class="tag">
      <span>{name}</span>
    </h2>
    {#if unreadCount > 0}
      <Badge color="info" style="font-size: 0.6rem;">
        {unreadCount}
      </Badge>
    {/if}
  </NavLink>
</NavItem>

<style lang="scss">
  h2.tag {
    margin-right: 0.3rem;
  }
</style>
