# Fix OpenCode Configuration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the broken OpenCode configuration by creating AGENTS.md and opencode.json, then clean up obsolete Claude Code files.

**Architecture:** Replace the broken CLAUDE.md (pointing to non-existent .claude/ paths) with OpenCode's proper config: AGENTS.md for project guidance and opencode.json for external file loading.

**Tech Stack:** Rust, OpenCode config files, markdown

---

### Task 1: Create AGENTS.md with project-specific guidance

**Files:**
- Create: `AGENTS.md`
- Modify: `.gitignore`

- [ ] **Step 1: Create AGENTS.md**

Create `AGENTS.md` with the following content:

```markdown
# OpenCode Rules

## Build & Test Commands

```
cargo build       # compile
cargo test        # run all tests
cargo clippy      # lint
cargo fmt         # format
```

Coverage (requires `cargo-llvm-cov`):
```
cargo llvm-cov --text 2>&1 > /tmp/cov.txt
grep "| *0|" /tmp/cov.txt
```

## Project Structure

- `src/` - Application source code
- `tests/` - Integration tests using trycmd
- `Cargo.toml` - Rust package manifest (borrowr, Rust edition 2024)

## What It Does

Terminal app for tracking lent/borrowed items. Data stored in SQLite at `~/.borrowr/borrowr.db`.

- Lend an item to someone (existing or new item/person)
- Borrow an item from someone
- View active loans
- Mark items as returned

## Configuration

| Variable   | Purpose                  | Default                     |
|------------|--------------------------|-----------------------------|
| `BORROWR_DB` | Override database path | `$HOME/.borrowr/borrowr.db` |

## Architecture

- Domain Driven Design (DDD)
- Test Driven Design (TDD)
- CQRS
- Hexagonal Architecture

## External Instructions

This project uses external instruction files loaded via `opencode.json`.
```

- [ ] **Step 2: Update .gitignore**

Add `.claude/` to `.gitignore`. The file currently contains:
```
/target
*.db
```

Change it to:
```
/target
*.db
.claude/
```

- [ ] **Step 3: Commit**

```bash
git add AGENTS.md .gitignore
git commit -m "feat: add AGENTS.md and .gitignore for OpenCode"
```

---

### Task 2: Create opencode.json with instructions

**Files:**
- Create: `opencode.json`

- [ ] **Step 1: Create opencode.json**

Create `opencode.json` with the following content:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "instructions": [
    ".opencode/rules/*.md",
    ".opencode/feedback/*.md"
  ]
}
```

- [ ] **Step 2: Commit**

```bash
git add opencode.json
git commit -m "feat: add opencode.json with instructions for rules and feedback"
```

---

### Task 3: Delete obsolete Claude Code files

**Files:**
- Delete: `CLAUDE.md`
- Delete: `OPENCODE.md`
- Delete: `.claude/` (entire directory)

- [ ] **Step 1: Delete obsolete files and commit**

```bash
git rm CLAUDE.md OPENCODE.md
rm -rf .claude
git commit -m "chore: remove obsolete Claude Code files (CLAUDE.md, OPENCODE.md, .claude/)"
```

---

## Plan Self-Review

**Spec coverage:**
- Create AGENTS.md — Task 1, Step 1 ✅
- Create opencode.json with instructions — Task 2 ✅
- Delete CLAUDE.md, OPENCODE.md, .claude/ — Task 3 ✅
- Add .claude/ to .gitignore — Task 1, Step 2 ✅
- Eager loading of rules/feedback — Task 2 opencode.json ✅

**Placeholder scan:** No TBD, TODO, or "implement later" patterns found.

**Type consistency:** N/A — no code changes.

**Order dependency:** Tasks must run in order — Task 1 before Task 2 before Task 3 (delete CLAUDE.md before Task 2 creates opencode.json to avoid confusion about which is active).
