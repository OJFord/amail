import {
  defineConfig,
} from "vite"
import {
  svelte,
} from "@sveltejs/vite-plugin-svelte"

export default defineConfig({
  build: {
    target: "esnext",
  },

  optimizeDeps: {
    esbuildOptions: {
      target: "esnext",
    },
  },

  plugins: [
    svelte(),
  ],

  preprocessorOptions: {
    scss: {
      additionalData: "@import global",
    },
  },
})
