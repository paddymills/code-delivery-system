
# Code Delivery System frontend

written in [svelte-js](https://svelte.dev/) using [bootstrap](https://getbootstrap.com/) components via [sveltestrap](https://sveltestrap.js.org/)

## Development

Install 

```bash
cd svelte
yarn

# or use npm (tauri scripts use yarn)
npm install
```

start tauri app
```bash
cargo tauri dev
```

## Using TypeScript

This template comes with a script to set up a TypeScript development environment, you can run it immediately after cloning the template with:

```bash
node scripts/setupTypeScript.js
```

Or remove the script via:

```bash
rm scripts/setupTypeScript.js
```

If you want to use `baseUrl` or `path` aliases within your `tsconfig`, you need to set up `@rollup/plugin-alias` to tell Rollup to resolve the aliases. For more info, see [this StackOverflow question](https://stackoverflow.com/questions/63427935/setup-tsconfig-path-in-svelte).
