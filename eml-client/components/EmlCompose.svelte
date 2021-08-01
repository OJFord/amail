<script>
  import {
    Input,
    //
  } from "sveltestrap";

  import * as api from "../api.js";
  import EmlAddresses from "./EmlAddresses.svelte";

  export let body = "";
  export let emlMeta;

  let sysName;
  api.getName().then((name) => (sysName = name));

  $: if (sysName && emlMeta.from.filter((f) => !f.name))
    emlMeta.from = emlMeta.from.map((from) =>
      from.name ? from : { ...from, name: sysName }
    );
</script>

<EmlAddresses bind:emlMeta editable={true} />

<Input type="textarea" name="body" value={body} rows="25" />
