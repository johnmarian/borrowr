# Cargo Commands

## Dependency Versions

Always use the latest stable version of any crate. Before adding a dependency,
check the current version on crates.io. Never pin to an outdated version found
in docs or search results.

```bash
cargo build          # compile
cargo run            # build and run
cargo test           # run all tests
cargo test <name>    # run a single test by name
cargo clippy         # lint
cargo fmt            # format code
```

## Code Coverage

`cargo-llvm-cov` is installed. To run and report coverage:

```bash
cargo llvm-cov --text 2>&1 > /tmp/cov.txt   # generate text report
grep "| *0|" /tmp/cov.txt                    # show all uncovered lines
```

The text format is `line_num | hit_count | source`. A hit count of `0` means uncovered.

When reporting, group uncovered lines by file and explain what each gap represents — don't just list line numbers. Distinguish between intentionally uncovered code (e.g. production impls only reachable at runtime, dead methods) and genuine missing tests.
