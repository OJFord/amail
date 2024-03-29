import * as tauri from "@tauri-apps/api/core"

export const applyTag = (query, tag) => tauri
  .invoke("apply_tag", {
    query,
    tag,
  })
  .then(() => Promise.resolve("tagsUpdated", {
    tags: [
      tag,
    ],
  }),
  )

export const getName = () => tauri.invoke("get_name")

export const getReplyTemplate = (id) => tauri.invoke("get_reply_template", {
  id,
})

export const listEml = (query) => tauri.invoke("list_eml", {
  query,
})

export const listTags = () => tauri.invoke("list_tags")

export const countMatches = (query) => tauri.invoke("count_matches", {
  query,
})

export const rmTag = (query, tag) => tauri
  .invoke("rm_tag", {
    query,
    tag,
  })
  .then(() => Promise.resolve("tagsUpdated", {
    tags: [
      tag,
    ],
  }),
  )

export const previewEml = (meta, body, attachments = []) => tauri.invoke("preview_eml", {
  meta,
  body,
  attachments,
})

export const sendEml = (meta, body, attachments = []) => tauri.invoke("send_eml", {
  meta,
  body,
  attachments,
})

export const tagList = () => tauri.invoke("list_tags")

export const viewEml = (id) => tauri.invoke("view_eml", {
  id,
})
