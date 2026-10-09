# Editor integration plan

## Generic LSP

Run `rust-cheats lsp` to start the stdio LSP process. The initial adapter advertises local completion and hover from cheat-sheet content. Keep the process separate from other language servers.

Example Neovim configuration (adjust the executable path):

```lua
vim.api.nvim_create_autocmd("FileType", {
  pattern = { "rust", "typescript", "javascript", "python", "go", "sh", "c", "cpp" },
  callback = function(args)
    vim.lsp.start({
      name = "rust-cheats",
      cmd = { "rust-cheats", "lsp" },
      root_dir = vim.fs.root(args.buf, { ".git" }) or vim.fn.getcwd(),
    })
  end,
})
```

## Vim / Neovim

`integrations/nvim/rust-cheats.lua` provides a starter user command that calls the CLI. `integrations/vim/rust-cheats.vim` provides a basic `:RustCheats` command. These are deliberately lightweight and do not require a separate plugin framework.

## VS Code

`integrations/vscode` contains a TypeScript extension starter with a command to query the CLI. Run `npm install`, then package it with `npm run package` after installing `vsce`. For production, use the LSP client package and provide platform-specific binary distribution.

## WASM / web

The core types use serde-compatible data structures, but not all workspace crates are WASM-compatible. Build a dedicated `cdylib` wrapper in `bindings/wasm` and depend only on core/data/search crates. Keep filesystem loading behind a browser-compatible data provider.
