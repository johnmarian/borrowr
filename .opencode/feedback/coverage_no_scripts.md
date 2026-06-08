# Coverage analysis — use text output, no scripts

Run coverage using `--text` output, save to a temp file, then grep for zero-hit lines:

```bash
cargo llvm-cov --text 2>&1 > /tmp/cov.txt
grep "| *0|" /tmp/cov.txt
```

The format is `line_num | hit_count | source`. Hit count `0` = uncovered.

**Why:** The text output is line-per-line and greppable. The HTML report is a single minified line that can't be paginated or read with the Read tool.

**How to report:** Group uncovered lines by file. For each gap, explain what the code does and whether it's a genuine missing test, intentionally untested (e.g. production impl only reachable at runtime), or dead code. Don't just list line numbers.
