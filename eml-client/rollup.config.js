import commonjs from "@rollup/plugin-commonjs";
import css from "rollup-plugin-css-only";
import json from "@rollup/plugin-json";
import livereload from "rollup-plugin-livereload";
import resolve from "@rollup/plugin-node-resolve";
import serve from "rollup-plugin-serve";
import scss from "rollup-plugin-scss";
import svelte from "rollup-plugin-svelte";
import sveltePreprocess from "svelte-preprocess";
import { terser } from "rollup-plugin-terser";

const watch = process.env.ROLLUP_WATCH === "true";
const production = process.env.DEBUG !== "true";

const distDir = "dist";
const port = process.env.PORT;

export default {
  input: "index.js",

  output: {
    file: `${distDir}/build/bundle.js`,
    format: "iife",
    inlineDynamicImports: true,
    name: "app",
    sourcemap: !production,
  },

  plugins: [
    svelte({
      emitCss: true,
      compilerOptions: {
        dev: !production,
      },
      preprocess: sveltePreprocess({
        sourceMap: !production,
      }),
    }),
    css({ output: "bundle.css" }),
    scss({ output: `${distDir}/build/global.css` }),
    json(),
    resolve({
      browser: true,
      dedupe: ["svelte"],
    }),
    commonjs(),
    !production &&
      watch &&
      serve({
        contentBase: distDir,
        host: "localhost",
        port,
      }) &&
      livereload({
        delay: 0,
        watch: distDir,
      }),
    production && terser(),
  ],

  watch: {
    clearScreen: false,
  },
};
