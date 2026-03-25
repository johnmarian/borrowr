# Do not misuse "internal invariant" to avoid error handling on I/O paths

Do not use "internal invariant violation" as a justification for panicking on any code path that involves I/O, databases, file systems, or external state.

**Why:** The "internal invariant" justification was used when `Ok(None)` from `find_by_id` caused a panic because the ID came from a prior `find_all`. The reasoning was that the record "can't disappear." But the database could be wiped, reverted, corrupted, or modified externally between calls — none of that is under the application's control.

**How to apply:**
- A true internal invariant can ONLY be violated by a bug in our own in-process logic — e.g. a nav label appearing in a result that we never put in our own map.
- Any condition that could be caused by external factors (database, file system, network, OS) must be handled as a proper error and returned to the caller.
- When tempted to panic on an `Ok(None)` or similar from a repository call, ask: *could this fail due to something outside the running process?* If yes, return an error.
- This is a pattern for avoiding work — flag it and implement the correct error handling.
