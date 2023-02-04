import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

const production = process.env.DEBUG !== "true";

export default {
  compilerOptions: {
    dev: !production,
  },
  preprocess: vitePreprocess({
    enableSourcemap: !production,
  }),
  vitePlugin: {
    emitCss: true,
  },
};
