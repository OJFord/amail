import js from "@eslint/js"
import importPlugin from "eslint-plugin-import"
import svelteParser from "svelte-eslint-parser"
import svelte from "eslint-plugin-svelte"

export default [
  js.configs.recommended,

  {
    ignores: [
      "**/dist/*",
      "**/target/*",
    ],
  },

  {
    languageOptions: {
      globals: {
        Intl: true,
        console: true,
        document: true,
        process: true,
        setInterval: true,
      },
    },

    plugins: {
      import: importPlugin,
      svelte,
    },

    rules: {
      // Import organization
      "import/order": "error",
      "import/newline-after-import": [
        "error",
        {
          considerComments: true,
        },
      ],

      // Code quality rules
      "no-trailing-spaces": "error",
      "one-var": [
        "error",
        "never",
      ],
    },
  },

  {
    files: [
      "**/*.svelte",
    ],

    languageOptions: {
      parser: svelteParser,
    },

    rules: {
      ...svelte.configs.recommended.rules,
      "init-declarations": "off",
      "svelte/no-at-html-tags": "warn",
    },
  },
]
