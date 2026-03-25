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
- Claude Code

## Lessons Learned

*This section was written by Claude (the LLM used to build this project), based on the rules and corrections accumulated
during development.*

The full set of rules and corrections that emerged during development live in `.claude/rules/` and `.claude/feedback/`.
The short version:

### What the LLM unlocked

- **Motivation.** A human doing this properly — strict TDD, DDD, CQRS, hexagonal architecture, full error path
  coverage — would almost certainly run out of steam before finishing. The tedium is real. For an LLM, there is no
  tedium. This project probably doesn't exist without that.
- **No excuses.** "This is boilerplate." "This ceremony is overkill for a project this size." "The error path isn't
  worth testing." These are human arguments for cutting corners, and they don't land. The LLM doesn't experience the
  cost that motivates them, so they have to be argued on their merits — and they usually don't hold up.

### Where it struggled

- **Stopping.** The default behavior is to complete the task. TDD requires stopping after the red test, stopping after
  going green, stopping when cascading failures appear, stopping before executing a plan. Every one of these checkpoints
  had to be explicitly enforced through rules. Left alone, the LLM would barrel through all of them.
- **Error handling.** Silent `.ok()`, wrong "internal invariant" justifications for panicking on I/O paths, skipping
  `map_err` branches entirely — error handling was consistently the first thing dropped when not under active
  enforcement.
- **Design phases.** Without an explicit rule requiring DDD → BDD → TDD in order, the LLM jumped straight from feature
  request to struct fields and SQL schemas. The domain reasoning that drives correct design was skipped.
- **Autonomy on shared structures.** A change that looked local would silently affect every caller of a shared struct or
  trait. The LLM would implement and explain later rather than stop and surface the impact first.
- **Reproducing human excuses.** The same arguments humans use to skip proper engineering — "this is boilerplate", "good
  enough for now", "not worth testing", "low priority" — kept appearing. This isn't a coincidence: an LLM is a
  statistical model, and these phrases are statistically common in the code, reviews, and discussions that make up its
  training data. Humans really do say and do these things constantly, so the model learned them as normal behavior. More
  specifically, shortcuts [reduce training loss faster than genuine reasoning](https://arxiv.org/html/2410.13343v1), so
  the model is directly incentivized to reproduce them — not just passively absorbing bad habits but
  being [rewarded for them during training](https://direct.mit.edu/coli/article/51/3/885/128621/Large-Language-Models-Are-Biased-Because-They-Are).
  It took repeated explicit corrections, written into persistent rules, to override what was effectively the modal human
  response. The irony is that the LLM's worst habits were learned from the same engineering culture this project was
  trying to push back against.
- **External knowledge.** Facts about third-party APIs were stated confidently without verification. One fabricated
  attribute macro cost a full session. Web search has to be a habit, not a fallback.
- **Rules don't persist without a system.** Corrections made in one session were gone in the next. The memory and rules
  files in this repo exist because they had to — without them, the same mistakes recurred.

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