<script>
  import sanitizeHtml from "sanitize-html";
  import {
    Spinner,
    //
  } from "sveltestrap";
  import * as tauri from "@tauri-apps/api/tauri";

  export let emlMeta;

  let contents = null;

  tauri
    .invoke("view_eml", { emlMeta })
    .then((eml) => {
      contents = sanitizeHtml(eml, {
        allowedTags: sanitizeHtml.defaults.allowedTags.filter(
          (tag) => tag != "img"
        ),
      });
    })
    .catch(console.error);
</script>

{#if contents == null}
  <Spinner primary />
{:else}
  <h2>{emlMeta.subject}</h2>
  <div>
    {@html contents}
  </div>
{/if}
