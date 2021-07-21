import commonjs from "@rollup/plugin-commonjs";
import css from "rollup-plugin-css-only";
import json from "@rollup/plugin-json";
import livereload from "rollup-plugin-livereload";
import resolve from "@rollup/plugin-node-resolve";
import scss from "rollup-plugin-scss";
import svelte from "rollup-plugin-svelte";
import sveltePreprocess from "svelte-preprocess";
import { terser } from "rollup-plugin-terser";

const watch = Boolean(process.env.ROLLUP_WATCH);
const production = !process.env.DEBUG;
const distDir = "dist";

const PORT = (Math.random() * (65535 - 1024) + 1024).toFixed(0);

let renderer;
let server;

function exit() {
  if (renderer) renderer.kill(0);
  if (server) server.kill(0);
  process.exit(0);
}

function render() {
  return {
    writeBundle() {
      if (renderer) return;
      renderer = require("child_process").spawn(
        "yarn",
        [
          "tauri",
          "dev",
          `--config={"build": {"devPath": "http://localhost:${PORT}"}}`,
        ],
        {
          stdio: [process.stdin, process.stdout, process.stderr],
        }
      );
    },
  };
}

function serve() {
  return {
    writeBundle() {
      if (server) return;
      server = require("child_process").spawn(
        "yarn",
        ["sirv", "--dev", `--port=${PORT}`, "--no-clear", "dist"],
        {
          stdio: [process.stdin, process.stdout, process.stderr],
        }
      );

      server.on("exit", exit);
      process.on("SIGTERM", exit);
      process.on("exit", exit);
    },
  };
}

export default {
  input: "index.js",

  output: {
    file: `${distDir}/build/bundle.js`,
    format: "iife",
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
    !production && watch && serve(),
    !production && watch && livereload(distDir),
    !production && watch && render(),
    production && terser(),
  ],

  watch: {
    clearScreen: false,
  },
};
