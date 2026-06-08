# Architecture

Rust 2024 edition. Both a binary and library crate:

- `src/main.rs` — binary entry point
- `src/lib.rs` — library crate (logic lives here, testable independently)

## Current Structure

```
src/
├── main.rs               — entry point, wires adapters
├── lib.rs                — declares modules
└── adapters/
    ├── mod.rs
    └── cli/
        └── mod.rs        — run() renders header, prompts via inquire
```

## Domain Model (DDD)

Three independent entities, each with their own identity and lifecycle:

- **`Person`** — an individual who can lend or borrow. Has an ID. Can appear on many loans.
- **`Item`** — a thing that can be lent or borrowed. Has an ID. Can appear on many loans.
- **`Loan`** — references a `Person` and an `Item` by ID. Has direction (lend/borrow) and status (active/returned).

`Loan` does NOT own `Person` or `Item`. It holds references to them. People and items exist independently and require their own CRUD.

## CQRS

**Commands (write — change state):**
- `CreatePerson`, `UpdatePerson`, `DeletePerson`
- `CreateItem`, `UpdateItem`, `DeleteItem`
- `LendItem`, `BorrowItem`, `ReturnItem`

**Queries (read — no state change):**
- `GetPeople`, `GetItems`
- `GetActiveLoans`

## Target: Hexagonal Architecture (Ports & Adapters)

```
src/
├── main.rs
├── lib.rs
├── domain/
│   ├── person.rs         — Person entity (ID + name)
│   ├── item.rs           — Item entity (ID + description)
│   └── loan.rs           — Loan aggregate (Person ID + Item ID + direction + status)
├── commands/
│   ├── people/           — CreatePerson, UpdatePerson, DeletePerson + handlers
│   ├── items/            — CreateItem, UpdateItem, DeleteItem + handlers
│   └── loans/            — LendItem, BorrowItem, ReturnItem + handlers
├── queries/
│   ├── people.rs         — GetPeople + handler
│   ├── items.rs          — GetItems + handler
│   └── loans.rs          — GetActiveLoans + handler
├── ports/
│   ├── person_repository.rs  — trait PersonRepository (CRUD)
│   ├── item_repository.rs    — trait ItemRepository (CRUD)
│   └── loan_repository.rs    — trait LoanRepository (read + write)
└── adapters/
    ├── cli/              — existing, dispatches commands/queries
    └── storage/          — future (implements all repository ports)
```

## Flow

**Command flow (e.g. lend an item):**
```
CLI adapter
  → LendItem command
    → LendItemHandler validates, builds Loan
      → LoanRepository port
        → storage adapter
```

**Query flow (e.g. view active loans):**
```
CLI adapter
  → GetActiveLoans query
    → GetActiveLoansHandler
      → LoanRepository port
        → storage adapter
          → Vec<Loan> back to CLI for display
```

## Adapters

- **CLI** — current adapter. Uses inquire for interaction. Calls commands and queries.
- **Storage** — future. Implements all repository/reader ports (initially SQLite).
- **Web** — future. Alternate adapter reusing all commands and queries untouched.
