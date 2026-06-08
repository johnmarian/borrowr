# Never use `git rebase -i`

`git rebase -i` consistently breaks in this environment — it spawns a vim session that hangs or aborts, leaving leftover `.git/rebase-merge` state.

Use `git commit --amend` to fix the HEAD commit message:

```bash
git commit --amend -m "fix: add AGENTS.md and .gitignore for OpenCode"
```

If you need to rewrite older commits, use `git reset --soft <base>` + re-commit:

```bash
git reset --soft a724ac9
git add AGENTS.md .gitignore
git commit -m "fix: add AGENTS.md and .gitignore for OpenCode"
git add opencode.json
git commit -m "fix: add opencode.json with instructions"
git rm -r CLAUDE.md OPENCODE.md .claude/
git commit -m "fix: remove obsolete Claude Code files"
```

If a rebase somehow leaves leftover state, clean it up:

```bash
rm -rf .git/rebase-merge .git/rebase-apply
```
