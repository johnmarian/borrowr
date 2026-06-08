# Avoid replace_all when removing inline markers

Do not use `replace_all` to remove markers (like `⚠ RED`) that appear at the end of lines alongside other content. The replacement removes the marker but collapses the newline into the next line, breaking formatting.

**Why:** This happened when removing `⚠ RED` markers from the doc block — lines were merged instead of just having the marker stripped.

**How to apply:** Use targeted `Edit` calls that include the full line context and restore the correct line ending explicitly.
