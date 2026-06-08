# Design before validation before implementation

Design → Validation → Implementation is the holistic guiding principle. TDD is one mechanism for enforcing it, not the principle itself.

**Why:** Jumping to implementation details (visibility issues, structural changes, how something will be wired) during the design phase derails the conversation and produces worse designs. Those concerns don't matter yet.

**How to apply:** Stay at the current level until it is complete. During design, only discuss what the behavior should be. During validation setup, only discuss how to verify it. Implementation details come last, and only after the design and validation are settled.

---

Start with the simplest case first. Do not raise future complexity until it is actually needed.

**Why:** Getting bogged down in edge cases, error paths, and later scenarios before the basic case is even designed wastes time and blocks progress.

**How to apply:** When designing scenarios or planning work, begin with the most basic happy path. Add complexity only after the foundation is in place.

---

When hitting an implementation constraint, look for the obvious abstraction before escalating to the user or proposing structural changes.

**Why:** Fixating on a concrete-level problem (e.g. lifetime of a borrowed struct) while ignoring an abstraction that already exists (e.g. trait objects) produces unnecessary complexity and wastes the user's time.

**How to apply:** Before raising a constraint as a blocker, ask whether an existing abstraction resolves it.

---

When introducing a tool or framework, research it fully before using it. Know its idioms, hooks, and canonical patterns upfront.

**Why:** Suggesting a tool without knowing it wastes the user's time.

**How to apply:** Before using a framework feature, read its docs well enough to propose the idiomatic solution, not just the first thing that compiles.
