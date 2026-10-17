# Agent instructions

- No compliments. Read README.md and docs/decisions/ before semantic changes.
- Use Rust 1.99.1 and the separately pinned nightly formatter.
- Write failing regressions first; never use cargo check or bypass hooks.
- Commit validated focused changes promptly. Never push main.
- Shared semantic data supports checked Serde in text and binary formats.
- Exact profile names identify actual rules and exclusions; reject unsupported selections.
- No backend dependencies, graph-sized collections, global mutable state or filesystem I/O.
- Rule metadata is finite; caller adapters own graph storage and streaming execution.
- Use scoped Conventional Commits and the shared tested-main release pipeline.
