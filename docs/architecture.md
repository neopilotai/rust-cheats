# Architecture

## Principles

- **Offline first:** bundled TOML sheets and a local directory are the source of truth; no backend is required.
- **One core API:** CLI, TUI, LSP, Vim, Neovim, VS Code, and future WASM consumers should reuse the same data/search/completion crates.
- **Graceful degradation:** absent grammars or editor services must not prevent search.
- **Privacy by default:** no telemetry and no persistent query history.
- **Small dependency surface:** frontends depend on the core abstractions, not each other.

## Request flow

```text
CLI / TUI / LSP / editor adapters
                |
       cheats-core API types
                |
  +-------------+--------------+
  |             |              |
cheats-data  cheats-search  cheats-complete
  |             |              |
local TOML   ranking/index   prefix suggestions
                |
       cheats-language / highlight
                |
       grammar + editor context
```

## Search ranking

The initial implementation combines case-insensitive substring matching and fuzzy matching. Exact title and ID matches rank above title substring matches, which rank above description/tag matches and fuzzy matches. This is intentionally simple and deterministic; later versions can add a persistent inverted index and BM25 scoring.

## Language support

Initial content includes Rust, TypeScript, JavaScript, Python, Go, Bash, C, and C++. Language IDs are normalized strings. Tree-sitter integration is isolated in `cheats-highlight`; language grammar crates should be added only when licensing, versions, and target support have been reviewed.

## LSP boundary

The generic LSP adapter is a separate crate. It should provide completion and hover from the same local data API and should not attempt to replace compiler-aware language servers. A client can run `rust-analyzer`, Pyright, gopls, clangd, or another language server alongside this cheat-sheet server.

## Security and privacy

Data files are treated as content, not executable instructions. The CLI does not execute code snippets. Private mode suppresses output and avoids persistent history. Shell-history modification and stealth against security tooling are explicitly out of scope.
