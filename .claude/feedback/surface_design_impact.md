# Surface design impact before implementing — especially on shared structures

Before implementing any change, identify whether it touches a shared or generalized structure (e.g. a base struct, a trait, a reused component). If it does, stop and raise it explicitly:

"You asked for X, but this touches [shared structure], which would introduce a new behavior across all uses of it. How would you like to handle that?"

**Why:** A change that looks local (e.g. ESC on one menu) can silently affect all instances of a shared structure (e.g. `BasicNav` used everywhere). Making that decision unilaterally — even if the change seems obvious — takes architectural control away from the user.

**How to apply:** Before touching any code, ask: "Is this change in a shared/generalized place?" If yes, surface the scope of the impact and ask the user how to proceed. Do not implement first and explain later.
