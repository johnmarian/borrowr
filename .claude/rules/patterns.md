# Code Patterns

## Avoid magic return values for control flow

Do NOT use return values to silently signal control flow to the caller. This is an anti-pattern:

```rust
// BAD — caller must remember what true/false means, intent is hidden
fn select_person_type(ctx: &Ctx, ...) -> bool {
    match ... {
        Some("Back") => false,
        _ => true,
    }
}

if !select_person_type(ctx, ...) {
    do_something(); // why? not obvious from the call site
}
```

Instead, make the intent explicit — pass the behaviour in, or handle it inside:

```rust
// GOOD — intent is clear at the call site and inside the handler
PersonTypeMenu {
    on_existing: "Select a person to lend to:",
    on_new: "Enter person name:",
    on_back: &LEND_EXISTING,
}.select(ctx);
```

This applies everywhere: nav actions, repository results, command handlers, etc. If a return value requires the caller to interpret a `bool`, `0`, `""`, or similar sentinel to decide what to do next, that's a signal to restructure so the decision lives where the knowledge is.

## Documentation: use doc blocks, not inline comments

Use `///` doc comments to document public items. Do not scatter explanations as `//` inline comments alongside the code.

```rust
// BAD — explanation buried as an inline comment next to the code
let path = borrowr_db.unwrap_or_else(|| format!("{}/borrowr.db", home)); // defaults to $HOME/borrowr.db

// GOOD — captured in a doc block where it can be rendered and searched
/// Resolves the path to the SQLite database file.
///
/// Defaults to `$HOME/borrowr.db`. Override with the `BORROWR_DB` environment variable.
pub fn db_path(borrowr_db: Option<String>, home: String) -> String {
```

Doc blocks should include:
- A one-line summary
- An `# Environment variables` table if the function reads env vars
- An `# Examples` section with runnable code (these are compiled as doc tests)

## Struct field access: use declarative getters, not tuple index syntax

Do not access struct fields via positional index (`.0`, `.1`). Define a named getter method instead.

```rust
// BAD — opaque, fragile if fields are reordered
let path = dir.0.join("subdir");

// GOOD — intent is clear at the call site
let path = dir.path().join("subdir");
```

Implement the getter on the struct:

```rust
impl TempDir {
    fn path(&self) -> &std::path::Path {
        &self.0
    }
}
```

This applies to all tuple structs, not just test helpers.

## Always recommend the correct solution

Never default to a simpler or "good enough for now" approach on the grounds of implementation effort or tedium. Tedium is a human constraint — it does not apply here. When there is a more correct solution, recommend and implement it. Do not offer shortcuts as defaults.

## Panic on internal invariant violations, don't return a sentinel

If a condition cannot occur without a bug in our own code, use `expect()` or `panic!()` — not a fallback value, `None`, or silent exit. A sentinel return hides the bug; a panic surfaces it immediately with context.

```rust
// BAD — silently exits as if nothing happened; the bug is invisible
None => Nav::Exit,

// GOOD — crashes loudly if our own code desynced
Nav::Go(*nav_items.get(label).unwrap_or_else(|| panic!("selected label {label:?} not in nav map")))
```

This is distinct from external errors (user input, I/O, repositories) which should be handled gracefully. Internal invariants — things that can only fail if we wrote incorrect code — should panic.

### What is NOT an internal invariant

A condition is only truly internal if it is **impossible without a bug in our own in-process logic** — for example, a nav label appearing in a selection result that wasn't in the list we built.

Anything that touches the outside world is NOT an internal invariant, no matter how unlikely it seems:

- A record fetched from a list and then looked up by ID **can** be missing — the database could be wiped, reverted, corrupted, or modified externally between the two calls.
- A file that existed moments ago can be deleted.
- A network resource can disappear.

These must return proper errors, not panic. Do not use "internal invariant" as a justification for avoiding error handling on any path that involves I/O or external state. When in doubt, ask: *could this fail due to something outside the running process?* If yes, handle it as an error.

## Raise deferred error handling explicitly

When writing code that discards or defers error handling — such as `.ok()`, `.unwrap_or_default()`, `.unwrap_or_else(|_| ...)`, or silently ignoring a `Result` — flag it to the user immediately. Do not leave silent error suppression in place without acknowledgement.

```rust
// BAD — silently treats a repository error the same as "not found"
let item = ctx.item_repo.find_by_id(&self.item_id).ok().flatten();

// GOOD — raise it: "find_by_id returns Result<Option<T>> and we have no error
// display path yet — flagging this for proper handling once that exists"
```

This applies to any pattern that swallows errors: `.ok()`, `.unwrap_or_default()`, empty `Err(_) => {}` match arms, and similar. If the correct handling cannot be implemented yet (e.g. no error display mechanism exists), say so explicitly and note what is needed.
