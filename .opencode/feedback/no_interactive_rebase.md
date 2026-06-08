# Never use `git rebase` or complex git history rewriting

`git rebase -i` consistently breaks in this environment — it spawns a vim session that hangs or aborts, leaving leftover `.git/rebase-merge` state.

Use `git commit --amend` to fix the HEAD commit message:

```bash
git commit --amend -m "fix: add AGENTS.md and .gitignore for OpenCode"
```

If you need to rewrite anything beyond the HEAD commit — older commits, multi-commit squashes, reordering, removing intermediate commits — **stop and flag the user**. These operations require user handling. Do not attempt `git reset`, `git rebase`, or any history manipulation beyond `--amend`.

If a rebase somehow leaves leftover state, clean it up:

```bash
rm -rf .git/rebase-merge .git/rebase-apply
```
