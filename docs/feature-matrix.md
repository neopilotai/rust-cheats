# Feature matrix

| Capability | Status | Evidence / next step |
|---|---|---|
| Offline TOML loading | Implemented | `cheats-data` |
| Schema and duplicate validation | Implemented | `CheatSheet::validate`, loader checks |
| Exact/substring/fuzzy search | Implemented | `cheats-search` |
| Language filters and aliases | Implemented | `cheats-language`, search filter |
| Prefix completion | Implemented | `cheats-complete` |
| Rule-based intent parsing | Implemented | `cheats-intent` |
| CLI search/show/ask | Implemented | `apps/cli` |
| Shell completion generation | Implemented | `clap_complete` |
| Privacy flags | Partial | No telemetry/history; broader persistence policy pending |
| Interactive TUI | Planned | Current command is a starter message |
| Syntax highlighting | Extension point | Grammar registration pending |
| Generic LSP | Partial | stdio initialize/completion/hover; context and protocol tests pending |
| Vim/Neovim | Starter | Commands exist; packaging/smoke tests pending |
| VS Code | Starter | TypeScript source exists; packaging behavior pending |
| WASM | Planned | Notes only; no binding crate |
| CI | Planned | Workflow not present in checkout |
| Benchmarks | Planned | No benchmark target present |
