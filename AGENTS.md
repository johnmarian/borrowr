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
