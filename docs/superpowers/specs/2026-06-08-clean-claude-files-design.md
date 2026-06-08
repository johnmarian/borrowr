# Design: Clean out old Claude Code files

## Problem

The project migrated from Claude Code to OpenCode but left behind broken configuration:
- `CLAUDE.md` points to `.claude/rules/` and `.claude/feedback/`, which don't exist (they were copied to `.opencode/`)
- `OPENCODE.md` has no special meaning to OpenCode
- `.claude/` directory still exists in the repo
- No `AGENTS.md` (OpenCode's primary instruction file) exists

## Design

### Create `AGENTS.md`
Primary instruction file with concise project guidance (build commands, architecture, conventions).

### Create `opencode.json`
Use the `instructions` field to eagerly load all rule and feedback files:
```json
{
  "$schema": "https://opencode.ai/config.json",
  "instructions": [".opencode/rules/*.md", ".opencode/feedback/*.md"]
}
```

### Delete obsolete files
- `CLAUDE.md` — broken fallback
- `OPENCODE.md` — not recognized by OpenCode
- `.claude/` — old directory, already copied to `.opencode/`

### Add `.claude/` to `.gitignore`
Prevent accidental re-committing.

## Scope
Single project cleanup. No decomposition needed.
