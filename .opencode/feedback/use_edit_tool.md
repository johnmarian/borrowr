# Never write scripts — use built-in tools directly

Never write scripts (Python, shell one-liners, sed, awk, etc.) for any purpose — whether to edit files, parse output, or analyze data.

**Why:** The built-in tools (Read, Grep, Edit, Write, Bash) can do everything a script would do, and the user can review each action directly. Scripts obscure what's happening and add unnecessary indirection.

**How to apply:**
- File edits → Edit or Write tools (never sed, awk, or any shell one-liner that writes to files)
- Bulk identical replacements → Edit with `replace_all: true` — do NOT reach for sed just because there are many occurrences or a multi-line pattern feels awkward
- Finding multi-line patterns → Grep with `multiline: true` instead of sed/perl one-liners
- Reading matches with context → Grep with `output_mode: "content"` and `-A`/`-B`/`-C` instead of grep + read
- Reading specific lines → Read with `offset` + `limit` instead of head/tail/sed
- Searching/reading files → Read, Grep, Glob
- Parsing command output or HTML → Bash with targeted grep, or Read the file directly
- Any multi-step data processing → break into individual tool calls, not a script
