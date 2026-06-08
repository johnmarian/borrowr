# Use DDD → BDD → TDD before implementing any behavior

Before writing any test or implementation for a new behavior, work through three levels in order:

1. **DDD** — What is this in domain terms? What entity/value object/aggregate is involved? What does the behavior mean to the domain? Do not skip to SQL, structs, or method signatures.
2. **BDD** — Express the behaviors as Given/When/Then scenarios. Each scenario is a distinct behavior. Present these to the user and confirm before moving to tests.
3. **TDD** — Only after BDD scenarios are agreed, write one red test per scenario.

**Why:** Jumping straight to implementation details (SQL queries, struct fields, method signatures) skips the domain and behavior reasoning that drives correct design. The SQL is a consequence of the behavior, not the starting point.

**How to apply:**
- When asked to implement a new repository method, query, or domain behavior: start with "What does this mean in domain terms?" and "What are the scenarios?"
- Present BDD scenarios to the user before writing any tests
- Do not mention SQL, struct fields, or implementation details until BDD scenarios are agreed
- Each BDD scenario maps to one red test
