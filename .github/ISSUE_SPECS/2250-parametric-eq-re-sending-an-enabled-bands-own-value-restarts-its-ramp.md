# Parametric EQ: re-sending an enabled band's own value restarts its ramp, so it never ends at small quanta

Filed by root from the #2247 verify (2026-11-05). A defect on main, in the same class as #2247; the fix moves bits, so it needs an owner ruling before a build.

## Problem
An enabled parametric-EQ band whose row is written again with its own value on every block (automation that holds a value, or a host that re-sends unchanged values) restarts its 64-frame ramp each time (`Channel::start_ramp`, `crates/parametric-eq/src/lib.rs`). Below a 64-frame quantum the ramp never ends, so `no_ramp_in_flight` stays false: the stationary path, the hoist, rest and elision are blocked, and the whole bank stays on the ramped kernel. #2247 fixed only the disabled band (a restated identity); the general rule "keep an in-flight ramp whose target words equal the new target" was left out because it moves bits on this path (#2247 scenario B digests, at 32 and 16 frames).

## Scope
Keep an in-flight ramp whose target words equal the new target words, for any design. State the exact bit-move scope (only re-sent unchanged values at quanta below 64) and confirm that every other path is bit-identical. Measure the render cost saved on a 64-track session at quantum 32 with held automation.

## Gates
- A test at quantum 16 that re-sends an enabled band's value every block: the ramp ends at frame 64 and the band reaches the stationary path. Red on today's code.
- No bit moves at quantum 128 or on any `fixtures/session/v1` session (one-time comparison, PR evidence).

## Decision record
- Owner ruling (2026-11-05): fix it. Bits may move only for re-sent unchanged values at quanta
  below 64.

## Evidence (attempt 1)

- **Rule.** `Channel::start_ramp` returns before any write when the lane has a ramp in flight
  and its target words are bit for bit the new words. The ramp keeps its increments and its
  remaining frames. This replaces the #2247 identity rule; the restated disable is one instance
  of it. The semantic target is still stored by `apply_prepared_target`. Cost: six lane reads,
  six integer compares and one countdown read, no allocation.
- **Why bits, not `==`.** The snap at a ramp's end copies `target`, so equal bits are exactly
  the case where a kept ramp and a restarted ramp land on the same words. `==` would also match
  `-0.0` to `+0.0`, which land on different bits.
- **Scope of the bit move.** Targets are applied only at a host block start
  (`crates/graph/src/runtime.rs`, before `process`). At a quantum of 64 or more a ramp ends in
  the block that starts it, so `remaining` is 0 at the next boundary and the rule cannot fire.
  Below 64 it fires only for a re-sent target. D4's zeroing needs `remaining == 0`, so it is
  unchanged. A `Both` retarget takes the decision on each channel alone; the HPF and LPF go
  through the same `start_ramp`, and the dry mask needs `remaining == 0`, so neither changes.
- **Gate.** `restated_target::a_re_sent_enabled_target_ends_its_ramp_at_frame_64`
  (`crates/parametric-eq/src/lib.rs`), at `W = 1` and every vector width of the build. One
  enabled bell moves from +9 dB to +3 dB, and the +3 dB target is applied again before every
  16-frame block. `no_ramp_in_flight` must be false for blocks 0 to 3 and true from block 4
  (frame 64). With the base rule it is red: "Scalar: block 4 starts at frame 64; the ramp must
  run to frame 64 and no further". The #2247 leg in `tests/bank.rs` stays: it defends D4's
  re-enable from rest after a restated disable, which this test does not reach.
- **No bit moved.** 300 blocks of every preparable `fixtures/session/v1` session, plain and live:
  the 34 digests are equal at base `031d931d9` and at head, and equal to the #2247 head. The
  scratch EQ harness (#2247 scenarios A to D at 128, 64, 32 and 16 frames) is equal at base and
  head for every row except scenario B at 32 and 16 frames.
- **Bits that move.** Only scenario B (an enabled band's gain re-sent with the same value every
  block), only at 32 and 16 frames; the head digests equal the #2247 "general rule" run. At 32
  frames 9,485 of 9,600 frames differ, from frame 65 (the first re-send in a ramp) to the end,
  largest difference 0.032. At 16 frames 4,752 of 4,800 differ, from frame 33 to the end, largest
  difference 0.046. In the last 100 frames the largest difference is 2.2e-7 and 2.7e-7: on base
  the ramp restarts each block, so the words approach the target but never snap to it.
- **Stopped: a bit move outside the re-sent-value path.** `contract.rs`
  `every_live_enable_or_family_ramp_is_contractive_and_bounded` fails, release and debug, at
  "44100 Hz 20 Hz 0 dB Q 0.1: 0 -> 2 -> next". The band is enabled as a low shelf (a ramp from
  the identity), and 17 frames later its family changes to a high shelf. At 0 dB both shelves
  design the same six words, so the new rule keeps the ramp where base restarts it from the words
  in force. That is a changed value (the family) with unchanged words, in a 17-frame block. The
  end words are the same; the path between differs. The brief says to stop here, so the cost run
  and the rest of the checks were not done. Fmt and workspace clippy passed. Open question for
  the owner: does "re-sent unchanged values" mean unchanged words (keep the rule, and the test
  must retarget to a state with different words) or unchanged semantic values (compare the
  semantic target too)?
