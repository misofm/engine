# Builtins: a fader lane's phase is four parallel flags, and a lane import copies them unchecked

Found by the implicit-state-machine audit of 2026-11-06 (tracker #2258, item GR-1).

## Mission
Give the ramped fader stage (`FaderRampStage`, the one body behind every banked fader slot and
the per-node `FaderMuteRampBuiltins`) one explicit per-lane, per-channel phase, so the flag
combinations no transition produces cannot be represented in the stage or in the lane record a
plan swap carries, and so a lane import re-derives what it cannot carry wrong. The SIMD ramp
words (`current`, `target`, `step`, the `f32` countdown word and the mute mask) and the
authoritative `u32` countdown stay what they are.

## Verified defect (base `76ff20b54`)
`crates/builtins/src/lib.rs`:
- The phase is four parallel `[[bool; MAX_BANK_LANES]; 2]` arrays beside the `u32` countdown:
  `muted` (3176), `mute_ramp` (3184), `armed` (3191) and `opens` (3195). The carry record
  `FaderChannelState` (3057-3076) holds the same four bools.
- **`mute_ramp && remaining == 0`.** Representable; no transition produces it (a mute change sets
  `mute_ramp = window != 0` right after `retarget` sets `remaining = window`, 3392-3393; the
  countdown's end and `reset` clear both together, 3519-3523, 3678-3679). On an unmuted lane it
  would hold the gain for good: every timed gain move (`timed_gain`, 3422-3427, through
  `gain_held`, 3398-3400) only remembers, because nothing clears the flag until another ramp
  runs out, and `lanes_at_rest` declines the lane forever (3458).
- **`armed && !muted`.** Representable; unreachable (`arm_fade_in` mutes before it arms,
  3252-3264; an armed channel takes mute records as a retarget of `opens` only, 3379-3382).
- **`opens` without `armed`.** Reachable and harmless: a fire clears `armed` and leaves `opens`
  (3271-3280), which the field doc calls meaningless. It is a second encoding of one state, not an
  illegal one.
- **Unchecked copy.** `import_lane` (3649-3668) writes all four flags and the countdown verbatim.
  `FaderLaneState` is opaque (private fields; `FaderChannelState` is crate-private), so its only
  producer is `export_lane` (3629-3644) of a stage whose own transitions never build either
  illegal combination: the import path is unreachable with a bad record today. What holds is that
  the record and the stage can *represent* one, and nothing at the boundary would catch it.
- The tracker's correction stands: `reset` keeping an arm is intentional (an arm is "cleared only
  by `fire_fade_in`").

**Is the fader's arm a shadow copy of `graph`'s `FadeArmState` (#2257)?** No, though one bit of
fact is recorded on both sides. `graph` does not depend on `builtins`; it fires through
`GraphPreparedBuiltinBankProcessor::fire_fade_in`. The graph's `FadeArmState`
(`crates/graph/src/lib.rs` 4753-4763) owns the *schedule* (waiting, fire at a sample, settling,
done), and the fader's arm owns the *hold*: the lane kept at `+0.0` and `opens`, the fire's
target that every mute record before the fire retargets on the render thread. Neither can be read
from the other without a render-time lookup across the crate boundary. The shared fact is
"fired" for a fader target, `FadeArmState::fired()` against the fader's arm having ended. Two
points keep it in agreement: `FadeFire::Fired` is reported exactly when the fire reached the lane
(#2257), and `fire_fade_in` ends an arm unconditionally. Both halves are carried at the same swap
block (the lane import, then `carry_fade_arms`). This issue does not change that split.

## Decision / model
- `FaderPhase`, one byte per lane and channel, `phase: [[FaderPhase; MAX_BANK_LANES]; 2]` replaces
  the four arrays:
  - `Open`: unmuted, gain free. A nonzero countdown is a gain ramp.
  - `Unmuting`: an unmute ramp in flight; the gain is held until the countdown ends.
  - `Muted`: muted. A nonzero countdown is the mute ramp toward `+0.0`; the gain is held either way.
  - `Armed { opens }`: a fade-in a plan swap armed holds the lane muted; `opens` is the fire's
    target, which every mute record before the fire retargets.
  So `muted` is "not `Open`/`Unmuting`", `gain_held` is "not `Open`", `is_armed` is
  `Armed { opens: true }`, and `armed && !muted` and `opens` without an arm cannot be written.
  A muted lane's mute ramp is its nonzero countdown, which is all the old `mute_ramp` said for a
  muted lane (a muted lane's countdown is set only by its mute change).
- One rule ends a finished unmute ramp: `FaderPhase::settled` (`Unmuting` becomes `Open`). The
  countdown's end (`end_ramp_windows`), `reset` and `import_lane` all apply it to a lane whose
  countdown is `0`. So `Unmuting` with no countdown left, the one combination the phase and the
  `u32` countdown can still disagree on, cannot survive any of the three boundaries.
- `import_lane` re-derives the phase from the record's phase and countdown, with that rule, rather
  than refusing. Every phase/countdown pair then imports as the lane the stage's own transitions
  would hold. No new `BuiltinParameterError` variant (its exhaustive matches in
  `builtins-compiler` and `tools/audit` stay untouched), and a lane carry has no new way to fail
  at the swap block.
- `FaderLaneState`'s per-channel record carries `phase` in place of the four bools. It is still
  opaque, plain, fixed size and never persisted (R6b).
- The ramp countdown words stay branch-free `u32`/`f32` lane words. The phase is read only in
  control bookkeeping (`mute_channel`, `live_gain`, `timed_gain`, arm/fire, export/import/reset)
  and in the per-lane loops of `end_ramp_windows` and `lanes_at_rest`, exactly where the bools
  were read.

## Non-goals
No change to `graph`'s fade arm table, `FadeFire`, the carry program, or any other crate. No new
public API; `is_armed`, `gain_held` and `is_muted` answer as before. Only the fader parts of
`crates/builtins/src/lib.rs` change: #2274 edits the input and mono-collapse parts of the same
file.

## Gates
- A regression test in `crates/builtins/src/lib.rs` `lane_state`, red on base, green at head: a
  lane record whose unmute ramp has no countdown left, imported into a bank lane, must not hold
  the lane's gain (with the instant-unmute and reset entries to the same rule).
- No rendered bit moves: a one-time digest comparison, base against head, never committed,
  driving banked (every bank width) and per-node faders through mute and unmute ramps, gain moves
  during them, timed operations, fade arms and fires (including records before the fire), resets,
  lane exports/imports and lane swaps, and folding every output bit and every state readback.
- Mutation runs of the fix's key lines, recorded here.
- `cargo test --locked` for `builtins`, `builtins-compiler` and `graph`;
  `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`;
  `cargo fmt --check`; `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`;
  `bash scripts/check-workspace-policy.sh`; `cargo run --locked --release -p realtime-policy`.

## Evidence (attempt 1)

**Strictly required beyond the fader stage.** `crates/builtins/src/rest_tests.rs`: the fader
rest probe read `stage.muted` and `stage.mute_ramp`; it now reads `is_muted` and the phase, so the
lane state the probe compares still covers the whole phase. `test_support`'s two fader word
readers derive their two mute words from the phase; the words they return are unchanged.

**Red on base.** The import half of
`lane_state::an_unmute_holds_the_gain_only_while_its_countdown_runs`, written against base's
record shape (`muted: false, mute_ramp: true, remaining: 0`), fails on base `76ff20b54`:
```
assertion `left == right` failed: Simd4: the gain is free
  left: Some(true)
 right: Some(false)
```
With that assertion removed, the timed move is only remembered:
```
assertion `left == right` failed: Simd4: a timed move retargets
  left: Some(1.0)
 right: Some(0.5)
```
At head the test passes, at both bank widths. Its instant-unmute and reset halves are green on
base. They are there because the rewrite moved those two decisions into new lines that no
existing test defended (mutations D and C below).

**Mutations (each reverted; `cargo test -p builtins`).**
- A. `import_lane` copies the phase verbatim (no `at_countdown`): only the new test is red
  (`Simd4: an import`). The digest below does not move, because `export_lane` never writes such
  a record.
- B. The countdown's end does not settle `Unmuting`: red in
  `fader_ramp::gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it`.
- C. `reset` does not settle `Unmuting`: red only in the new test (`Simd4: a reset`). Before the
  test was extended, no builtins test caught it.
- D. An unmute with no window enters `Unmuting`: red only in the new test
  (`Simd4: an instant unmute`). Before the test was extended, no builtins test caught it.
- E. A re-arm overwrites an existing arm (drops `opens`): no builtins test is red, though the
  digest moves. Re-arming a lane is unreachable in product (`install_fade_arms` refuses a
  repeated target and runs once per plan). Base had the same untested branch; left as is.
- F. The fire does not end the arm: red in
  `fader_ramp::an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute`.

**Test value.** `an_unmute_holds_the_gain_only_while_its_countdown_runs` is red if an instant
unmute, a reset in the middle of an unmute ramp, or the import of a record whose unmute ramp has
no frames left leaves the lane's gain held with no countdown to end the hold. No existing test
catches any of the three (mutations A, C, D). It replaces no test; none is made useless.

**No bit moved.** A scratch harness (`crates/builtins/tests/gr1_bits.rs`, never committed) drives
400 seeds per subject. Each seed runs 300 random steps over two banks of each width (Simd4 and
Simd8, 1 to width members, random prepared gains and mutes) and over one per-node
`FaderMuteRampBuiltins`. The steps are: live mutes and unmutes with windows of 0 to 1000 frames;
live gain moves; timed operations at the block boundary and inside blocks (`process_with`);
arms, re-arms and fires, with mute records before the fire; resets; exports and imports between
the banks (plan-swap carries); same-bank and cross-bank lane swaps; and `process` and
`process_from_left` over random, partly silent input. Every rendered output bit is folded into
one FNV-1a digest per subject, together with every state readback after each step
(`fader_bank_lane_words` / `scalar_fader_words`, `is_armed`, `gain_held`, `is_muted`,
`fader_gain`, `ramp_target`, `ramp_remaining`, `lanes_at_rest`). Results:
```
base 76ff20b54: simd4 c50ab7ba3c148cb5  simd8 c632900598d806ce  scalar 1d4ee1dfcd9290db
head:           simd4 c50ab7ba3c148cb5  simd8 c632900598d806ce  scalar 1d4ee1dfcd9290db
```
The harness discriminates: mutations B to F each move all three digests.

**Gates (head).**
- `cargo test --locked -p builtins -p builtins-compiler -p graph`: every binary passes, 0 failed (builtins lib 22 tests).
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in
  46 files)`.

**Left as is.** `crates/graph/src/lib.rs`'s `FadeFire` doc still says the fader lane's "own
`armed` flag". That is accurate in meaning (the lane's arm, which only a fire ends), and this
issue changes no other crate.
