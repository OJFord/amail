<script>
  import {
    Input,
    //
  } from "sveltestrap";

  import EmlAddresses from "./EmlAddresses.svelte";

  export let body = "";
  export let emlMeta;
</script>

<EmlAddresses bind:emlMeta editable={true} />

<Input type="textarea" name="body" value={body} rows="25" />
