<script>
  import { Input } from "sveltestrap";

  export let quietQueries = [];
  export let querySelected = null;
</script>

<div class="search">
  <Input
    type="search"
    placeholder={querySelected ? querySelected : "Enter a query"}
    on:input={(q) => (querySelected = q.target.value)}
    value={quietQueries.includes(querySelected) ? null : querySelected}
  />
</div>

<style lang="scss">
  .search {
    margin: 0.3rem;
  }
</style>
