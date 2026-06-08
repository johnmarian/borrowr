# TDD applies to ALL behavioral code — always red first, one branch at a time

Any change to production code behavior requires a failing test first — no exceptions. Each branch is a separate behavior.

The "no red test required" exemption for structural/type-level definitions applies **only to new definitions being introduced for the first time**. Modifying existing code that is already under test requires a red test first — even if the change appears purely structural (adding a field to a struct, making a unit struct data-carrying, changing a static to a dynamically constructed instance).

**Why:** Recurring violation pattern: write a complete method (happy path + all `map_err` error branches) and only test the happy path. The error branches are real behaviors — they return specific error variants — and go untested.

**How to apply:**
- Write one test, one branch of implementation at a time. Never write the full method body upfront.
- Specifically: every `map_err(|_| DomainError::Xxx)`, `Err(_) =>` match arm, `unwrap_or_else`, `.ok()` — any line that changes control flow on failure — requires its own red test written before that line exists in production code.
- Sequence for a method with happy path + error handling:
  1. Red test for happy path → go green (write only enough to pass — no error handling yet)
  2. Red test for the error branch → go green (add the `map_err`)
- Run `cargo test` and confirm each test fails with the right kind of failure before touching implementation.
- `#[should_panic(expected = "...")]` is the correct test pattern for `todo!()` and `panic!()` branches.
- If implementation already exists without tests, revert the untested code, get red, then re-implement.
- When replacing a `todo!()`, only write the branch the current red test exercises. If the implementation requires a `match` with multiple arms and Rust forces exhaustion, stop and raise it: "This match requires an `Err` arm that has no red test yet — do you want a placeholder `todo!()`, a `#[should_panic]` test, or a proper red test first?" Let the user decide before writing any behavior into that arm.
- This applies to ALL exhaustive matches — not just `Result`. A `match self.direction { Lend => ..., Borrow => ... }` where only `Lend` has a test means `Borrow` must be `todo!()` until a red test exists.
- `panic!()` arms ARE behaviors — they are not "just structural". A `_ => panic!(...)` wildcard arm requires a `#[should_panic(expected = "...")]` red test before that line exists.
- Before writing ANY error/fallback arm, ask what the desired behavior should be. Do not copy patterns from existing code without confirming the same choice applies here.
