import {
  mount,
} from "svelte"
import App from "./App.svelte"
import "./global.scss"

const app = mount(App, {
  target: document.body,
  props: {},
})

export default app
