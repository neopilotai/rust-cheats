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
| Generic LSP | Implemented | stdio initialize/completion/hover with context-aware request handling |
| Vim/Neovim | Starter | Commands exist; packaging/smoke tests remain future work |
| VS Code | Starter | TypeScript source exists; packaging behavior remains future work |
| WASM | Planned | Notes only; no binding crate |
| CI | Implemented | GitHub Actions runs format, check, tests, clippy, and release build |
| Benchmarks | Planned | No benchmark target present |
