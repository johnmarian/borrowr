# Cascading failures after a trait/signature change are by design — stop and ask

When a small intentional change (e.g. updating a trait signature or return type) causes many compile errors or test failures, that is expected and correct TDD behaviour. It means the change propagated correctly and downstream callers need updating.

**Why:** The user explicitly asked for this. "Fixing" cascading failures by reverting the change or silently patching call sites breaks the TDD discipline and hides the true scope of the change.

**How to apply:**
1. Make the intentional change (e.g. update the trait)
2. Run `cargo test` (or `cargo build`)
3. If many things fail as a result, show the failures to the user
4. STOP — do not patch call sites, do not revert, do not try to make everything green in one shot
5. Ask the user: "X things are now failing — which should I update next?" or similar
6. The user decides the order and scope of follow-up work.

This applies to: trait signature changes, return type changes, renamed variants, removed methods — any change that has intended downstream impact.
