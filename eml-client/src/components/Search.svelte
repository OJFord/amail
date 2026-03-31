<script>
  import {
    Input,
  } from "@sveltestrap/sveltestrap"

  /**
   * @typedef {Object} Props
   * @property {any} [quietQueries]
   * @property {any} [querySelected]
   */

  /** @type {Props} */
  let {
    querySelected = $bindable(null),
    quietQueries = [],
    //
  } = $props()
</script>

<div class="search">
  <Input
    type="search"
    placeholder={querySelected ? querySelected : "Enter a query"}
    on:input={(q) => (querySelected = q.target.value)}
    value={quietQueries.includes(querySelected.valueOf())
      ? null
      : querySelected}
  />
</div>

<style lang="scss">
</style>
