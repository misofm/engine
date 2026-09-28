# Mid-chain silence: skip slots behind a silent point, and a gate that closes to silence

Draft, slice S9 of the silence architecture issue (A0). Two parts: S9a is class A; S9b is a new
product mode (class B for sessions that select it, class A for every other session). Split them if
S9b is ruled separately. Evidence: `docs/handoffs/silence-2026-09-27/DESIGN.md` section 4.7.

## Product outcome

The owner's example: a near-silent source goes through a noise gate that makes it truly silent, and
nothing after the gate should process it.

* **S9a, propagation.** Inside a running chain, a slot whose input block is known silent and which is
  latched (S4/S8) is skipped in place: the block already holds its `+0.0` output. The chain derives
  each slot's input fact from the previous slot: known by state (a latched slot, a settled full mute
  on every lane, a hard-closed gate on every lane), or measured by #942 on the block the previous
  slot wrote (about 3 ns per live block, 55 ns per silent eight-lane block).
* **S9b, the gate's hard close.** Today a gate never emits exact `+0.0` from live input: its gain is
  at least -96 dB (`range <= 96`, `crates/gate-expander/src/lib.rs:311`; `fast_exp2` is always
  positive, and `select(identity, dry, dry * gain)` keeps the sign of `x`). A hard-close mode (a new
  range value or a mode parameter) emits `andnot(x, closed)` (`+0.0`) once the smoothed gain reaches a
  floor (for example -144 dB) and until it rises above it, so the gate's output is silent and
  everything downstream at rest skips.

Lockstep still applies: the downstream slots skip only when every lane of the bank is silent at that
point. S7's co-silence grouping raises how often that happens.

## Authorized paths

S9a: `crates/rack/src/lib.rs` (the chain's per-slot fact), tests. S9b: `crates/gate-expander/src/{lib.rs,kernel.rs}`,
its descriptor and corpus, tests.

## Objective gates

1. S9a: bit identity against the declined oracle over S4's corpus plus strips whose first slot is a
   muted-and-settled fader (test-only strips); red mutation: skip a slot whose input fact is stale
   from the previous block.
2. S9b: every existing gate fixture and digest unchanged (the mode is off by default); in the mode, a
   closed gate's output is `+0.0` by bits, open/close transitions follow the declared ballistics, the
   mode's parameter is in the descriptor, state payload and response analysis; listening evidence per
   `AGENTS.md` (the close is click-free because the gain is already at the floor).
3. Realtime and rule 3 as S4.

## Console benchmark rows

None of the standing rows carries a gate. A gated sparse variant is the evidence.

## Dependencies

S8 (the per-slot latch), S4.

## Rulings

S9b is a product and DSP decision (the mode, its floor, its name).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "bit-identical" or "unchanged" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
