import importPlugin from "eslint-plugin-import"
import modulesNewlines from "@spence1115/eslint-plugin-modules-newlines"
import svelteParser from "svelte-eslint-parser"
import svelte from "eslint-plugin-svelte"

export default [
  "eslint:recommended",

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
      modulesNewlines,
      import: importPlugin,
      svelte,
    },

    rules: {
      "array-bracket-newline": [
        "error",
        {
          minItems: 1,
        },
      ],
      "array-element-newline": [
        "error",
        "always",
      ],
      "comma-dangle": [
        "error",
        "always-multiline",
      ],
      "dot-location": [
        "error",
        "property",
      ],
      "modulesNewlines/import-declaration-newline": "error",
      "implicit-arrow-linebreak": [
        "error",
        "beside",
      ],
      "import/order": [
        "error",
      ],
      "import/newline-after-import": [
        "error",
        {
          considerComments: true,
        },
      ],
      indent: [
        "error",
        2,
      ],
      "newline-per-chained-call": [
        "error",
        {
          ignoreChainWithDepth: 1,
        },
      ],
      "no-trailing-spaces": "error",
      "object-curly-newline": [
        "error",
        {
          minProperties: 1,
        },
      ],
      "object-property-newline": [
        "error",
        {
          allowAllPropertiesOnSameLine: false,
        },
      ],
      "one-var": [
        "error",
        "never",
      ],
      "operator-linebreak": [
        "error",
        "before",
        {
          overrides: {
            "=": "none",
          },
        },
      ],
      "padded-blocks": [
        "error",
        "never",
        {
          allowSingleLineBlocks: false,
        },
      ],
      semi: [
        "error",
        "never",
      ],
      "sort-imports": [
        "error",
        {
          allowSeparatedGroups: true,
          ignoreDeclarationSort: true, // Handled by import/order rule
        },
      ],
      "quote-props": [
        "error",
        "as-needed",
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
      "svelte/no-at-html-tags": "warn", // pending disable line
    },
  },
]
