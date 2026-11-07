# Owner ruling: should a fader move that comes due at an entry override an offsets jump applied at that same entry?

Owner question recorded by #2362 (GR-9), filed when the implicit-state-machine tracker #2258 closed. This issue asks for a decision and then carries it out. No code changes until the owner rules.

## The behaviour on `main`
A live unmute drains over a window `W`, and its hold ends on a block's last sample, so a fader move is owed and comes due at `W`. When an offsets write lands at `W` (a jump, with ramp 1 or 480), the due move lands over the fader length and replaces the jump's ramp. From `W` on, the output is bit-identical to the session without the jump's ramp.

`crates/host-core/tests/stored_fader_automation.rs`, `an_offsets_jump_at_the_entry_that_frees_the_lane_keeps_its_ramp`, pins this as its contrast case at quanta 128, 64 and 37.

## Why it is a question
The F1 rule says a jump's hold is newer than a ramp that a live write ended. By that reading, the jump at `W` is the newer intent and should keep its ramp. The due move is older, because it was owed before `W`. Both readings are self-consistent, and #2362 deliberately changed no rule.

## Options
- **A (keep):** the due move wins, as today. Record the ruling, and turn the contrast case's comment into a citation of it.
- **B (jump wins):** an offsets jump at the entry where the move comes due keeps its ramp, and the move is dropped or re-based onto the jump's target. This changes rendered bits, so it needs a no-bit-moved comparison for every other case, a flipped contrast case that is red on base, and a release note.

## Gates (after the ruling)
- A: a ruling record under `docs/rulings/`, and a comment-only change.
- B: the flipped contrast case is red on base, the rest of `stored_fader_automation.rs` is unchanged and green, the graph fixtures are regenerated if a plan word moves, and the no-bit-moved evidence covers every other case.

## Evidence / attempt record
(empty)
