# TDD Workflow (Red-Green-Refactor)

This project follows strict TDD. This is non-negotiable:

1. **RED**: Write a failing test first. Run it with `cargo test` to confirm it fails. **Stop and show the user the failing output before writing any implementation.**
2. **GREEN**: Write the minimum code to make the test pass.
3. **REFACTOR**: Clean up code while keeping tests green.

**The core rule is: no untested implementation.** Red-green-refactor is the mechanism for enforcing this when writing new code — write a failing test first so the implementation is never untested.

When adding a focused test for behavior that already exists and is already covered by another test, follow this workflow:

1. Write the focused test — it will be immediately green
2. Raise it to the user for confirmation
3. Introduce a targeted regression in the implementation to verify the test catches it
4. Show the user the failing output
5. Revert the regression — back to green

**Each test is a contract for one observable behavior — not a test of shared implementation.** A behavior that appears "covered" by another test is only covered incidentally via shared code. If that shared code is later refactored or split, the coverage disappears silently. Every distinct observable behavior must have its own test, regardless of whether current implementation shares a path with another behavior.

**Every behavioral change — whether implementing or proposing — must list "write red test" as its explicit first step.** A plan that lists implementation steps without listing the red test first for each one is wrong and incomplete.

## What requires a red test

TDD applies to **behavioral code** — anything with logic that can be right or wrong:
- Function and method implementations
- Command and query handlers
- Validation logic
- Any code with conditionals, transformations, or side effects

## Each branch is a separate behavior

A method with a happy path and error branches requires multiple tests written in sequence — one per branch. Do NOT write the whole method and only test the happy path.

```rust
// This method has THREE behaviors, each needing its own red test:
fn find_all(&self) -> Result<Vec<Item>, DomainError> {
    let mut stmt = self.conn
        .prepare("SELECT ...")
        .map_err(|_| DomainError::ItemLoadFailed)?; // ← behavior 2: error branch
    stmt.query_map([], map_row)
        .map_err(|_| DomainError::ItemLoadFailed)?  // ← behavior 3: error branch
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| DomainError::ItemLoadFailed)   // ← same error, same test
        // ↑ behavior 1: happy path
}
```

Write them one at a time:
1. Red test for happy path → go green (write only enough to pass)
2. Red test for the error branch → go green (add the `map_err`)

This means `map_err`, `Err(_) =>` match arms, `unwrap_or_else`, `.ok()` — any line that changes control flow on failure — cannot be written without a red test first.

## Cascading failures are by design

When a small intentional change (e.g. changing a trait signature) causes many tests or compile errors to fail, **that is expected and correct**. It means the change is right and downstream callers need to be updated. Do NOT "fix" the failures by reverting the change or adding workarounds or unsolicited fallback behavior.

Instead: **stop, show the user the failures, and ask what to do next.**

Do not silently patch call sites, revert the change, or try to make everything green in one shot. The user decides the order and scope of the follow-up work.

## What does NOT require a red test

New structural/type-level definitions being introduced for the first time have no behavior to test and can be written directly:
- `trait` definitions (ports, interfaces)
- `struct` and `enum` definitions used as data containers
- Type aliases
- `mod` declarations and module structure

**This exemption applies only to new definitions.** Any modification to existing code that is already under test requires a red test first — even if the change appears purely structural, such as adding a field to a struct, making a unit struct data-carrying, or changing a static instance to a dynamically constructed one.
