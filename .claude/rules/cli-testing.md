# CLI Testing Strategy

All CLI behavior must be tested using both of the following tools:

## `assert_cmd` — programmatic tests
Use for error cases, edge cases, and any assertions requiring conditional logic.

```toml
[dev-dependencies]
assert_cmd = "2"
predicates = "3"
```

```rust
// tests/integration.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_some_behavior() {
    Command::cargo_bin("borrowr")
        .unwrap()
        .args(["subcommand", "arg"])
        .assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}
```

## `trycmd` — snapshot/file-based tests
Use for locking exact output of happy-path commands. Snapshots live in `tests/cmd/*.toml`.

```toml
[dev-dependencies]
trycmd = "0.15"
```

```rust
// tests/cli_tests.rs
#[test]
fn cli_tests() {
    trycmd::TestCases::new()
        .case("tests/cmd/*.toml");
}
```

```toml
# tests/cmd/example.toml
[command]
bin.name = "borrowr"
args = ["--help"]

[output]
status.code = 0
stdout = """
...exact expected output...
"""
```

Run `TRYCMD=overwrite cargo test` to auto-generate or update snapshots.

## When to use which
- **`trycmd`**: happy-path output, exact stdout/stderr locking
- **`assert_cmd`**: error cases, dynamic assertions, edge cases

## Explicit navigation — no flow terminators

Every test must navigate explicitly all the way to Quit. Never use `end()`, `end_item()`,
`end_person()`, or `end_input()` as a shortcut to stop a flow mid-way. These helpers exist
**only** in tests that are specifically testing ESC behaviour.

```rust
// BAD — None used as a silent flow terminator; breaks when ESC gains back navigation
sel.expect_select_item().times(1).returning(end_item()); // just to "get out"

// GOOD — explicit path to exit
sel.expect_select_item().times(1).returning(end_item()); // testing ESC on item list
sel.expect_select()
    .with(predicate::eq("Lend an existing item or a new one?"), predicate::always())
    .times(1)
    .returning(|_, _| Some("Back".to_string())); // Back → Main Menu
sel.expect_select()
    .with(predicate::eq("What would you like to do?"), predicate::always())
    .times(1)
    .returning(|_, _| Some("Quit".to_string())); // Quit → exit
```

**Why:** `end_*` helpers return `None` (ESC). Every time ESC gains proper back navigation,
any test using `end_*` as a terminator cascades and requires updating. Explicit paths are
stable — a test that says `nav("Back")` then `nav("Quit")` is unaffected by how ESC is
handled on any screen.

## Entry points are behaviors

A different path to a screen is a different behavior. If the observable outcome after an
action differs based on how you arrived — different navigation target, different prompts
shown, different repo calls made — it is a distinct scenario requiring its own test. Do
not collapse multiple entry points into one test on the grounds that the action (e.g.
Cancel) is the same.

In BDD terms: a different "Given" context with a different observable "Then" is a
different behavior, regardless of whether the "When" is identical.

## Flow coverage doc block

The `//!` doc comment at the top of the `tests` module in `src/adapters/cli/mod.rs` is the authoritative map of UI flows to test functions. Keep it in sync:

- When a test is added, add it to the doc block linked with `[`fn_name`]`
- When a test turns green, remove its `⚠ RED` marker
- When a flow node has no test yet, mark it `*(no test yet)*`
- When a flow node is intentionally untested (e.g. 3rd party behavior), mark it with a reason
