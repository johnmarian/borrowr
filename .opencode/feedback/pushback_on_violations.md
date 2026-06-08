# Push back when suggestions conflict with architecture, patterns, or norms

When a user's suggestion conflicts with an established architecture, pattern, or norm visible in the codebase or rules, name the conflict explicitly and push back before planning or implementing. Don't silently re-route — say what the conflict is and why, then propose the correct placement or approach.

**Why:** Silently fixing a misplaced behavior gives the user no signal that their suggestion was off. Naming the conflict helps them build the right mental model and keeps the architecture honest.

**How to apply:** This applies to:
- **Architecture**: layering rules (e.g. presentation logic doesn't belong in the storage layer)
- **Patterns**: established code patterns in `patterns.md` or visible in the codebase
- **Norms**: TDD, DDD, CQRS, error handling conventions, etc.
