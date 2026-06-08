# OpenCode Migration Design

## Scope

Migrate all Claude Code configuration from `.claude/` to `.opencode/` for the borrowr Rust project.

## What

1. **Create `.opencode/rules/`** — copy all 7 rule files from `.claude/rules/`:
   - `architecture.md`, `cargo.md`, `tdd.md`, `patterns.md`, `cli-testing.md`, `cli-ui-flows.md`, `mocking.md`

2. **Create `.opencode/feedback/`** — copy all 21 feedback files from `.claude/feedback/`

3. **Create `OPENCODE.md`** — mirror `CLAUDE.md` with `.claude/` references changed to `.opencode/`

4. **Update `README.md`** — change all `.claude/` references to `.opencode/`, and all "Claude Code" / "Claude" references to "OpenCode"

## What Stays

- `.claude/` directory untouched for migration verification
- `CLAUDE.md` untouched for migration verification
- All project code untouched

## Rationale

`.opencode/` in the repo is the only way for cloned repositories to include LLM assistant configuration. OpenCode project config lives in the user's home directory and is not portable.
