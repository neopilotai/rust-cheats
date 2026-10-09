# rust-cheats

An offline-first, extensible programming cheat-sheet engine written in Rust. The workspace starts with a shared library, local data loader, fuzzy search, completion API, language registry, syntax-highlighting adapter, intent parser, configuration, generic LSP server, CLI, and TUI starter.

## Status

This is a phased scaffold, not a claim that every editor adapter or grammar is production-complete yet. Core search, data loading, CLI commands, completion, basic intent parsing, and tests are implemented. Tree-sitter grammar registration and editor integrations are extension points for subsequent phases.

## Requirements

- Rust stable (see `rust-toolchain.toml`)
- Git (optional)

## Quick start

```bash
cargo test --workspace
cargo run -p rust-cheats -- search "rust error handling"
cargo run -p rust-cheats -- search "iter map colct" --language rust
cargo run -p rust-cheats -- languages
cargo run -p rust-cheats -- show rust/iterators
cargo run -p rust-cheats -- complete "rust/"
cargo run -p rust-cheats -- ask "How do I read JSON in Rust?"
cargo run -p rust-cheats -- tui
```

By default, the CLI reads bundled data from `data/`. Override it with `RUST_CHEATS_DATA_DIR` or `--data-dir`. No network, account, API key, or backend is required.

## Workspace map

```text
crates/
  cheats-core/       Shared data types and API
  cheats-data/       Markdown/TOML data loading
  cheats-search/     Full-text + fuzzy ranking
  cheats-complete/   Prefix completion and suggestions
  cheats-language/   Language registry and extension metadata
  cheats-highlight/  Tree-sitter highlighting integration point
  cheats-intent/     Lightweight local natural-language intent parsing
  cheats-config/     Privacy-conscious configuration
  cheats-lsp/        Generic LSP adapter
apps/
  cli/               rust-cheats CLI
  tui/               Interactive terminal starter
integrations/
  vim/               Vim plugin
  nvim/              Neovim plugin
  vscode/            VS Code extension starter
bindings/
  wasm/              WASM integration notes
data/
  languages/         Cheat-sheet content
docs/
  architecture.md
  integrations.md
```

## Privacy / stealth mode

`--quiet` suppresses nonessential output. `--private` avoids writing query history and forces quiet output. The scaffold has no telemetry and does not change shell history, hide processes, or evade security monitoring. Search is local and read-only.

## Data format

Each sheet is a TOML file:

```toml
id = "rust/iterators"
title = "Rust iterators"
language = "rust"
category = "collections"
description = "Common iterator operations"
tags = ["iterator", "map", "filter", "collect"]

[[examples]]
title = "Map and collect"
language = "rust"
code = "let doubled: Vec<_> = values.iter().map(|x| x * 2).collect();"
description = "Transform each item and collect the results."
```

Use unique IDs. `language` should be a stable identifier, not a display name. See `data/`.

## Development

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

GitHub Actions runs format, clippy, and tests on pushes and pull requests. Some editor integrations are starter examples and may require local tooling to package.

## Roadmap

1. Stabilize core schema and offline search.
2. Improve ranking, completion, and TUI interaction.
3. Register Tree-sitter grammars and syntax queries.
4. Expand LSP features and editor integrations.
5. Add WASM bindings and optional sync without making the backend mandatory.

## License

Dual-licensed under MIT OR Apache-2.0.
