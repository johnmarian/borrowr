# Raise deferred error handling explicitly

Always flag deferred or suppressed error handling to the user at the point of writing it.

**Why:** `.ok().flatten()` was used in `PersonTypeMenu::confirm()` to silently discard repository errors, treating them identically to "not found." The user caught this and called it out — it should have been raised proactively.

**How to apply:** Any time you write `.ok()`, `.unwrap_or_default()`, an empty `Err(_) => {}` arm, or any other pattern that swallows a `Result` error, immediately say so: explain what error is being discarded, why proper handling isn't in place yet, and what would be needed to handle it correctly. Do not leave silent error suppression without explicit acknowledgement.
