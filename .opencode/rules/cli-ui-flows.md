# CLI UI Flows

The authoritative UI flow map lives in the `//!` doc comment at the top of the `tests`
module in `src/adapters/cli/mod.rs`. That is the single source of truth for both flow
design and test coverage.

When a flow node has no documented behavior, **ask the user before assuming**. Once
decided, update the doc block in `mod.rs` first, then write tests and implementation.
