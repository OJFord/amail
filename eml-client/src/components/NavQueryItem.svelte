<script>
  import {
    FontAwesomeIcon,
  } from "@fortawesome/svelte-fontawesome"
  import {
    createEventDispatcher,
  } from "svelte"
  import {
    faTag,
  } from "@fortawesome/free-solid-svg-icons"
  import {
    Badge,
    NavItem,
    NavLink,
  } from "@sveltestrap/sveltestrap"

  export let name
  export let selected
  export let unreadCount

  const dispatch = createEventDispatcher()
</script>

<NavItem>
  <NavLink active={selected} on:click={() => dispatch("select")}>
    <FontAwesomeIcon icon={faTag} />
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
