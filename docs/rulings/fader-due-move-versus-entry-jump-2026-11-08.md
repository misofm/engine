# A fader move due at an entry keeps priority over an offsets jump at that entry

**Decision (2026-11-08):** made by root under the owner's explicit delegation of 2026-11-08 ("Please reason about it and decide"), for issue #2374, the question #2362 recorded. Authority kind: owner-delegated answer.

**Ruling: option A.** The rule on `main` stands, and no rendered bit moves.

## The case
A live unmute's held ramp ends on a block's last sample, while the fader automation's grid events arrived during the hold (they are remembered, so a move is owed). The move comes due at the next block's entry `W` (`HoldAtBlockEnd::EndsAtBlockEnd`, `crates/builtins-compiler`). An offsets write (a jump, with its own ramp of 1 or 480 samples) also lands at `W`. On `main`, the due move lands over the fader length and replaces the jump's ramp. The lane then reaches the composed target, the curve plus the jump's offset, which is the same final value the jump asked for.

## Why the jump does not win
1. **F1 governs releases, not obligations.** F1 (U2 fold 3) protects a jump's hold from the R17-10 *release* of a ramp that a live write cut at the same entry. The cut ramp is superseded intent, so the newer jump wins. In this case nothing is released: the unmute's ramp ended by itself. The due move is the engine's obligation to bring the lane from its held value back onto the authored curve. It is not an older ramp competing with the jump. Applying F1's "newer" argument here treats an obligation as intent, which is the category error #2362 suspected.
2. **The catch-up must keep a glitch-free length.** The owed move can span the whole difference between the held gain and the curve, which can be tens of dB. If the jump's ramp won, a 1-sample jump would carry that whole catch-up as a step, which is an audible click. Decision 15 requires every live value to ramp, and the fader length is the smoothing that makes the catch-up safe. The offset's own delta is still reached, inside the same move.
3. **No target is lost.** Both readings converge to the same value: the curve plus the offset. The jump's request differs from the result only in the transition shape over the move's length, never in the value it settles at. This satisfies the D15-2 convergence rule.
4. **Option B costs state and adds no correctness.** Honouring both lengths would need either a second concurrent ramp per lane, which is new lane words in a SIMD bank and a per-sample cost, or a rule that re-bases the move onto the jump's length. That rule would bring back the step in reason 2 whenever the jump's ramp is shorter than the fader length. Neither fixes a defect, and the second introduces one.

## Reopening condition
Reopen if a product need appears for an offsets write whose transition shape must be exact even while an owed catch-up is due, for example sample-accurate VCA moves that must match an external timeline. That reopening must say how the catch-up stays glitch-free.

## Where it is pinned
`crates/host-core/tests/stored_fader_automation.rs`, `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp`: its contrast case, at quanta 128, 64 and 37 on every render form. Also `HoldAtBlockEnd::EndsAtBlockEnd` and `jumped_at_entry` in `crates/builtins-compiler/src/lib.rs`.
