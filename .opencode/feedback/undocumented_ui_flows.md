# Undocumented UI flows — ask first, update design doc before tests

When implementing a UI flow node that is not documented in `.claude/rules/cli-ui-flows.md`, stop and ask the user what the intended behavior should be. Do not assume and write tests or code.

**Why:** Assumptions about undocumented flows led to tests being written and then reverted when the design turned out to be different (e.g. assuming "Confirm → Main Menu" when the actual design needed a dismissable result screen).

**How to apply:**
1. If a flow node is missing from `cli-ui-flows.md`, ask the user to describe the behavior before writing anything.
2. Once the user confirms the design, update `cli-ui-flows.md` FIRST.
3. Only then write tests and implementation.
