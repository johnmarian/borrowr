# Stop and re-confirm after context compaction

After a context compaction, stop before doing any work and re-confirm the next step with the user.

**Why:** When resuming from a compaction summary, it is easy to barrel through into implementation, ignoring rules like TDD red-first, plan confirmation, and cascading failure stops. The compaction is visible — the conversation starts with a `Summary:` block instead of actual message history. That is the signal to pause.

**How to apply:** When the first message in the conversation is a compaction summary, do NOT immediately resume executing the last task described. Instead, briefly state what you understand the next step to be and ask the user to confirm before proceeding.
