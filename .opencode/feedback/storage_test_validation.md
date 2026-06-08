# Storage adapter tests must validate via direct raw SQL, not repository methods

Storage adapter unit tests must validate command effects via direct raw SQL on the in-memory connection. Never use other repository methods (e.g., `find_by_person`, `find_all`) to verify what a command did.

**Why:** Repository methods are themselves under test. Using `find_by_person` to verify `lend` creates a dependency between two units — a bug in `find_by_person` could mask a failure in `lend`, or vice versa. Direct SQL is authoritative and has no such coupling.

**How to apply:**
- After calling a command (e.g., `repo.lend(...)`), query the DB directly: `conn.query_row("SELECT ... FROM loans WHERE ...", ...)`
- The `setup()` helper must retain a raw `Connection` reference alongside the repo for direct SQL queries
- Because `Connection` cannot be shared (moves into repo), use a pattern that keeps a reference: either store the connection separately and pass `&conn` to both, or use `Arc<Mutex<Connection>>`
- This applies to all three storage repositories: `PersonRepository`, `ItemRepository`, `LoanRepository`
