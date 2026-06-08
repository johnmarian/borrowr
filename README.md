# borrowr

A seriously over-engineered terminal app for tracking what you've lent out and borrowed.

## Why make this?

I wanted a concrete example of how to use LLM coding assistants to follow all the patterns, practices, and
architectures we've all been talking about for decades, but rarely implement properly. It makes sense, they're tedious,
even when we know they're the right solution.

Well, LLMs should be able to remove that tedium, right? So here we have an application with a **VERY** small scope.
I used this so that I could tightly control the tools and not only get it to follow the above-mentioned ideas, but also
be small enough that I could verify that it **did** follow those ideas.

I'm also very forgetful and just needed a utility to remember who borrowed my table saw...

### Principles used

- Domain Driven Design
- Test Driven Design
- CQRS
- Hexagonal Architecture

## Technologies (so far)

- Rust
- SQLite
- llama.cpp + Qwen3.6-35B-A3B (local model, [unsloth/Qwen3.6-35B-A3B-GGUF](https://huggingface.co/unsloth/Qwen3.6-35B-A3B-GGUF))

## Lessons Learned

*These lessons were accumulated through repeated corrections during the project's development. They live in full
detail in `.opencode/rules/` and `.opencode/feedback/`. The summaries below are what the LLM itself reported as
the most impactful patterns.*

### What made this project possible

- **No tedium bias.** Humans skip boilerplate because it feels wasteful. LLMs don't feel that — which means they
  have no internal reason to skip it either, unless told otherwise. This project only exists because the rules
  made skipping corners impossible, not because the LLM naturally cared about the right thing.
- **No fatigue.** A human would likely abandon strict TDD, DDD, CQRS, and hexagonal architecture on a project
  this small — the ceremony would outweigh the perceived value. The LLM doesn't get bored, so it doesn't make
  the argument that "it's overkill for something this tiny."

### What had to be enforced

- **Stopping at red and green.** TDD requires three distinct pauses: before writing any code, after the test fails,
  and after the code just barely passes. The LLM's default impulse is to power through all three. Each one had
  to be codified as an explicit stop point.
- **Branch-level testing.** Every error path in a function — `map_err`, `Err(_)` arms, `unwrap_or_else` — is
  a separate behavior that needs its own test. The LLM consistently wrote only the happy path and treated error
  handling as an afterthought.
- **DDD before code.** Without an explicit rule, the LLM goes straight from "add a feature" to struct fields and
  SQL queries. The domain reasoning — what the feature *means* in terms of the domain — is the first thing that
  gets skipped.
- **Surfacing shared impact.** A local change often ripples through callers, traits, and shared types. The LLM
  tends to implement the fix first and surface the impact later. The rule had to be reversed: surface impact
  before implementing anything.
- **Not calling I/O an invariant.** Panicking on a database read or file operation is not the same as panicking on
  a logic bug. The LLM frequently classified I/O failures as "internal invariants" to avoid writing error handling.
  Anything outside the process boundary must return errors, never panic.
- **Deferring error suppression.** Patterns like `.ok()` and `.unwrap_or_default()` silently discard errors. The LLM
  used them liberally. Each instance had to be flagged: if the correct handling can't be implemented yet, say so
  explicitly instead of hiding the problem.
- **Not making things up.** The LLM states unverified facts about third-party APIs with the same confidence as
  verified knowledge. One fabricated attribute macro cost a full debugging session. Web search must be a default,
  not an exception.
- **Rules need memory.** Corrections made in one session were forgotten in the next. The `.opencode/` directory
  exists because the LLM has no persistent memory between sessions — the files are the memory.

## What it does

- **Lend** an item to someone — existing or new item, existing or new person
- **Borrow** an item from someone — same flows
- **View active loans** — see everything currently lent out or borrowed
- **Mark as returned** — close out a loan with a return date

Data is stored locally in a SQLite database at `~/.borrowr/borrowr.db`.

## Install

```
cargo install --path .
```

## Run

```
borrowr
```

## Configuration

| Variable     | Purpose                    | Default                     |
|--------------|----------------------------|-----------------------------|
| `BORROWR_DB` | Override the database path | `$HOME/.borrowr/borrowr.db` |

## Development

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

## Optimizations and future plans

- Take all the UI flow tests and port them to an actual BDD testing framework like Cucumber
- Since this was built with hexagonal architecture, I'm planning to work bit-by-bit on getting the LLM to expand it to a
  web app, native app, who knows?