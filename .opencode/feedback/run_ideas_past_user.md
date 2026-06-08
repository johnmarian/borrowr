# Run non-obvious design decisions past the user before implementing

Do not introduce new fields, methods, or abstractions that weren't part of the agreed plan without first raising them with the user.

**Why:** A `success_message` field was added to `PersonTypeMenu` without discussion. The message is derivable from existing context (`direction`, `item.description`, `person.name`) — the field was unnecessary and the user had to roll back the change.

**How to apply:** Before adding anything structural that wasn't explicitly agreed, ask: "I'm thinking of adding X — does that make sense?" If the answer can be derived from existing context, don't add it.
