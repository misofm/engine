# Parametric EQ: a touched row restarts a disabled band's disable ramp, so it never completes at small quanta

Filed by root from the #2091 research and its verify (2026-11-04). A defect on main today, independent of #2091.

## Problem
A disabled parametric-EQ band's disable ramp restarts whenever a row of its section is touched: each touched row sends the identity target again (`crates/parametric-eq/src/control.rs:198-226`), and `apply_prepared_target` / `start_ramp` (`crates/parametric-eq/src/lib.rs:1565, 1606`) restart the ramp because nothing compares the new target with the one already in flight. With a render quantum below 64 frames and an automated (or repeatedly set) parameter on a disabled band, the 64-frame disable never completes, so P5 (re-enable from zero state, the #1337 D4 contract at `lib.rs:1602`, "whatever its history") never fires, and the band never reaches rest, which blocks rest detection and elision. Browser hosts (quantum 128) are not affected; C ABI and mobile hosts with small quanta are.

## Scope
Do not restart a ramp whose in-flight target equals the new target (the identity for a disabled band; the general equal-target rule moves bits for re-sent enabled values and is split to #2250, which needs an owner ruling); keep the remaining frames. Show which rendered bits move (only the restarted-disable path at quanta below 64) and confirm that every other path is bit-identical.

## Gates
- A test at quantum 16 with a parameter written every block on a disabled band: the disable completes at frame 64 and a later enable starts from zero state. Red on today's code.
- No bit moves at quantum 128 or on any `fixtures/session/v1` session (one-time comparison, PR evidence).
- Bits move on the restarted path at small quanta: this is the fix of a broken contract, recorded for the owner with the #2091 hold ruling.


## Evidence (attempt 1)

- **Rule.** `Channel::start_ramp` keeps a ramp in flight when the new words are the identity and
  the lane's target words are already the identity: it returns before any write, so the ramp
  keeps its increments and its remaining frames. The semantic target is still stored. The check is
  three bit compares of six words and one countdown read, with no allocation.
- **Why the narrow rule, not "equal target words".** The general rule moves bits outside the
  broken path. An enabled band can be sent its own words again mid-ramp, for example a control
  re-sent with the same value. Under a block below 64 frames, today's code restarts that ramp,
  and the general rule would keep it. Measured once in the scratch EQ harness: with the general
  rule, an enabled band re-sent its gain every block moves bits at 32 and 16 frames (equal at 128
  and 64); with the identity rule it moves none. The owner ruling allows
  bits to move only on the restarted disable. An enabled design is never the identity (its
  `a2 > 0`, `validate_svf_within`), so the identity rule is exactly the restated disable. The
  enabled case has the same shape (a ramp that a restatement every block never ends), and it is
  left for the owner.
- **Interactions.** The keep runs before `stationary_at`. When no ramp is in flight, today's path
  runs unchanged, so a settled disabled lane still settles and D4's zeroing in
  `apply_prepared_target` (which needs `remaining == 0`) is unchanged. A `Both` retarget takes the
  same decision on both channels, because both carry the same countdown and target, so the
  collapse witness is unchanged.
- **Gate.** `a_live_re_enable_starts_from_rest` (`crates/parametric-eq/tests/bank.rs`, gate 4 of
  #1337) gains a second leg. Blocks of 16 frames, band 2's frequency written on every block while
  it is disabled, and the re-enable at the first boundary after frame 64 of the disable. The
  re-enabled lane must render the bits of a lane prepared disabled, at `W = 1` and the native bank.
  The live helpers take the block length. On base `1b5d0e4b6` the leg is red ("16 frames, W = 1:
  a re-enabled lane renders as one prepared disabled"); at head it is green.
- **No bit moved.** 300 blocks of every preparable `fixtures/session/v1` session, plain and live:
  the 34 digests are equal at base and at head. A scratch EQ harness (300 blocks of noise, blocks
  of 128, 64, 32 and 16 frames) gives equal digests at base and head for: an enabled band
  restated with the same value every block; an enabled bell's slope written every block; and a
  disable and re-enable with no writes between. Both harnesses are scratch only.
- **Bits that move.** Only the automated disable (disable at block 1, band 2's frequency written
  every block, re-enable at block 40), and only below 64 frames. At 128 and 64 frames the digests
  are equal. At 32 frames, 1,078 of 9,600 frames differ, from frame 65 (the first restart) to frame
  1,682 (block 52), largest difference 0.028. At 16 frames, 4,748 of 4,800 frames differ, from
  frame 33 to the end, largest difference 0.024: the disable ends at frame 64 instead of never,
  and the re-enable starts from rest instead of from the frozen state. At 16 frames the
  difference lasts to the end of the 300 blocks; the cause was not investigated.
