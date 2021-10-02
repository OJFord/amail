import * as tauri from "@tauri-apps/api/tauri";

export const applyTag = (query, tag, dispatch) =>
  tauri
    .invoke("apply_tag", { query, tag })
    .then(() => Promise.resolve("tagsUpdated", { tags: [tag] }));

export const getName = () => tauri.invoke("get_name");

export const listEml = (query) => tauri.invoke("list_eml", { query });

export const listTags = () => tauri.invoke("list_tags");

export const countMatches = (query) => tauri.invoke("count_matches", { query });

export const rmTag = (query, tag) =>
  tauri
    .invoke("rm_tag", { query, tag })
    .then(() => Promise.resolve("tagsUpdated", { tags: [tag] }));

export const sendEml = (to, from, eml) =>
  tauri.invoke("send_eml", { to, from, eml });

export const tagList = () => tauri.invoke("list_tags");

export const viewEml = (id) => tauri.invoke("view_eml", { id });
