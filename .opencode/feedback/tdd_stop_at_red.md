# Stop at red — show output and wait for confirmation before going green

The core rule is **no untested implementation**. Red-green-refactor enforces this when writing new code. When adding a focused test for already-implemented, already-covered behavior, follow this workflow:
1. Write the focused test — it will be immediately green
2. Raise it to the user for confirmation
3. Introduce a targeted regression to verify the test catches it
4. Show the failing output
5. Revert — back to green

**Each test is a contract for one observable behavior, not a test of shared implementation.** A behavior that appears "covered" by another test is only covered incidentally via shared code. If that code is later refactored or split, the coverage disappears silently. Every distinct observable behavior must have its own test, regardless of whether current implementation shares a path with another behavior.

After writing OR UPDATING a failing test and running `cargo test`, STOP. Show the user the failing output and do not proceed to the green step until the user explicitly confirms.

**Why:** The red step exists not just to confirm the test compiles and fails, but as a checkpoint for the user to review what is being tested before implementation begins. This applies equally to updating existing tests — a test change is a spec change and requires the same checkpoint.

**How to apply:**
1. Write or update a failing test
2. Run `cargo test`, paste the failing output
3. STOP — say nothing more about implementation
4. Wait for the user to say "go green", "yes", "proceed", or similar
5. Only then write the implementation

**Critical:** Never change a test and immediately write production code in the same step, even if the reasoning seems obvious. The checkpoint is non-negotiable.

**Stop between ALL steps, not just red→green.** After going green, STOP. Do not immediately proceed to the next path's tests. Do not remove panic tests. Do not update doc blocks. Wait for explicit confirmation before each next action.

**Removing panic tests requires explicit instruction.** Panic tests that say "*(currently: panic — TODO)*" are left in place until the user explicitly says to remove them.

**Every behavioral change — whether implementing or proposing — must list "write red test" as its explicit first step.** A plan that lists implementation steps without listing the red test first for each one is wrong and incomplete.

**Presenting a plan does NOT mean you can execute it.** After laying out any plan, STOP and wait for explicit confirmation before touching any files.
