# Implementation plan

## Completed baseline work

1. Inventory the workspace, crates, data, integrations, and existing tests.
2. Preserve the pre-existing VS Code lockfile change.
3. Document architecture, coverage, and known limitations.

## Execution order

1. Stabilize core models and data validation, including duplicate IDs and malformed-file diagnostics.
2. Improve deterministic search ranking and completion behavior.
3. Add CLI data validation/rebuild commands and useful noninteractive output.
4. Replace the TUI starter with an interactive ratatui workflow.
5. Add language-aware highlighting and context-aware LSP tests.
6. Add WASM bindings and package editor integrations.
7. Add CI, benchmarks, security/license checks, and release documentation. CI is now in place; the remaining work is benchmark and release hardening.

## Verification policy

Run formatting, workspace check, tests, clippy, release build, WASM checks, and VS Code TypeScript checks when the required toolchains are available. Record unavailable-toolchain blockers instead of claiming success. Update this plan and the feature matrix after each milestone.
