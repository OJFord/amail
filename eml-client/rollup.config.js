import commonjs from "@rollup/plugin-commonjs";
import css from "rollup-plugin-css-only";
import livereload from "rollup-plugin-livereload";
import resolve from "@rollup/plugin-node-resolve";
import svelte from "rollup-plugin-svelte";
import { terser } from "rollup-plugin-terser";

const watch = Boolean(process.env.ROLLUP_WATCH);
const production = !process.env.DEBUG;
const distDir = "dist";

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
      renderer = require("child_process").spawn("yarn", ["tauri", "dev"], {
        stdio: [process.stdin, process.stdout, process.stderr],
      });
    },
  };
}

function serve() {
  return {
    writeBundle() {
      if (server) return;
      server = require("child_process").spawn(
        "yarn",
        ["sirv", "--dev", "--port=4000", "--no-clear", "dist"],
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
      compilerOptions: {
        dev: !production,
      },
    }),
    css({ output: "bundle.css" }),
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
