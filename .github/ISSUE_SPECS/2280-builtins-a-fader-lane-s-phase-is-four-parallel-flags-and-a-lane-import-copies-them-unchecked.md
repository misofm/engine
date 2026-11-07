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

## Verdict (attempt 1)
**FAIL**, on one MAJOR test gap; the reviewer found the code correct. The reviewer confirmed the
phase model is complete, that re-deriving the phase on import is right, that the fader's arm is
not a shadow of `FadeArmState`, that the no-bit-moved digests reproduce, and that the branch
merges cleanly with #2274.
- *MAJOR.* `live_gain`'s exit from `Unmuting` (`self.phase[channel][lane] = FaderPhase::Open`),
  the exit that product traffic actually reaches, was untested: deleting it (M2) survived in every
  crate. With a window of `0` it leaves an unmute ramp with no countdown, which holds the gain for
  good.
- *MINOR.* The re-arm branch of `arm_fade_in` is unreachable from a plan
  (`graph::install_fade_arms` refuses a repeated target) and said nothing about it.
- *MINOR.* The tracker's "fader arm vs `FadeArmState`" shadow-copy wording; the coordinator
  corrects it on #2258.

## Evidence (attempt 2)
`origin/main` merged first (`60054a023`); then one commit (`71a0f18fc`). Attempt 3 re-merged `origin/main` at `8d181a6cb`.

**Changes.**
- `lane_state::an_unmute_holds_the_gain_only_while_its_countdown_runs` gains two cases. A lane in
  an unmute ramp of 64 frames takes a live fader move with no window (`set_fader_db(.., 0)`) and
  is then open: `gain_held == Some(false)`, and a timed `SetGain` retargets it (M2). A lane
  imported in the middle of an unmute ramp still holds its gain, and a timed `SetGain` only
  remembers it. That case kills mutant P4 below, which attempt 1's tests let through.
- `arm_fade_in`'s re-arm branch carries a comment: defensive, unreachable from a plan, because
  `graph::install_fade_arms` arms each lane once per plan and refuses a repeated target.
- No other code change; the stage's render path is as attempt 1 left it, so attempt 1's
  no-bit-moved comparison stands.

**Guard table.** Every phase write and read in the fader stage, each mutated alone and reverted.
"Killed by" lists the failing tests. The first pass ran `-p builtins --lib --test fader_ramp
--test stage --test randomized --test contract` (W1-W3 ran the full `-p builtins`). Each survivor
then ran against `-p builtins -p builtins-compiler -p graph -p rack`.

| Id | Site | Mutation | Killed by |
|---|---|---|---|
| W1 | `new` | prepared mute ignored | `a_move_made_while_muted_applies_on_unmute`, the new test, `the_banks_render_their_scalar_sections_under_random_retargets` |
| W2 | `arm_fade_in` | `opens` inverted | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| W2b | `arm_fade_in` | `Armed` never written | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| W3 | `fire_fade_in` | arm not ended | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| W4 (M2) | `live_gain` | `Unmuting` not ended | the new test (attempt 2's case) |
| W5 | `mute_channel`, armed | `opens = muted` | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| W5b | `mute_channel`, armed | `opens` not written | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| W6 | `mute_channel` | a mute enters `Open` | `a_repeated_mute_leaves_the_stage_as_it_was`, `a_settled_banked_mute_is_exactly_positive_zero`, `a_zero_window_mute_is_the_exact_prepared_mute`, others |
| W7 | `mute_channel` | an instant unmute enters `Unmuting` | the new test |
| W8 | `mute_channel` | a ramped unmute enters `Open` | the new test |
| W9 | `end_ramp_windows` | `Unmuting` not settled | `gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it` |
| W10 | `export_lane` | phase not carried | `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders` |
| W11 | `import_lane` | phase copied verbatim | the new test |
| W11b | `import_lane` | phase not written | `a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders`, the new test |
| W12 | `reset` | `Unmuting` not settled | the new test |
| R1 | `arm_fade_in` | a re-arm overwrites the arm | survives in all four crates (unreachable; see below) |
| R2 | `fire_fade_in` | a lane not armed is fired too | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| R2b | `fire_fade_in` | `opens` ignored (always opens) | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| R3 | `is_armed` | `opens` ignored | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| R4 | `sync_mute` | mask from nothing | `a_settled_banked_mute_is_exactly_positive_zero`, `a_zero_window_mute_is_the_exact_prepared_mute`, others |
| R5 | `live_gain` | a muted lane not held | `a_move_made_while_muted_applies_on_unmute`, `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute`, others |
| R6 | `mute_channel` | the arm ignored | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| R7 | `mute_channel` | a repeated state is not a no-op | `a_repeated_mute_leaves_the_stage_as_it_was` |
| R8 | `gain_held` | never held | `gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it`, the new test, others |
| R9 | `lanes_at_rest` | `muted()` dropped from `zero_out` | survives in all four crates (equivalent; see below) |
| R10 | `is_muted` | never muted | `mute_is_a_fader_endpoint_and_settles_to_exact_positive_zero` |
| P1 | `FaderPhase::muted` | `Armed` not muted | `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| P2 | `FaderPhase::gain_held` | `Unmuting` not held | the new test |
| P3 | `FaderPhase::settled` | `Unmuting` kept | `gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it`, the new test |
| P4 | `FaderPhase::at_countdown` | settles a live countdown | the new test (attempt 2's import case; survived attempt 1's tests) |
| P5 | `FaderPhase::prepared` | inverted | `a_move_made_while_muted_applies_on_unmute`, `a_repeated_mute_leaves_the_stage_as_it_was`, others |

Survivors:
- **R1** is the re-arm branch, unreachable from a plan, now commented. It keeps base's behaviour:
  an arm stands.
- **R9** is an equivalent mutant. A muted lane's mute target is the literal `0.0`, and a settle
  assigns `current = target`, so a settled muted lane's gain is exactly `+0.0`. Its
  `!gain.is_sign_negative()` therefore already holds, and `muted()` in `zero_out` changes no
  answer. Base had the same redundant term.

**Gates (head).** 
- `cargo test --locked -p builtins -p builtins-compiler -p graph -p rack`: 42 binaries, all ok.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (292 marked regions in
  46 files)`.

## Verdict (attempt 2)
**FAIL**, on one MAJOR test gap. The reviewer confirmed the model, M2, the import re-derivation
and no-bit-moved, and accepted R1 (unreachable) and R9 (equivalent).
- *MAJOR.* The `Armed` arm of the phase rules was untested. X5 (`settled` ends an arm) and its
  reset-only form X4 survived every crate: either drops an unfired arm on a plan-swap carry or a
  reset, and the lane stays silent for good. X6 (`gain_held` excludes `Armed`) survived too: a
  timed `SetGain` retargets an armed lane, so the fire's fade-in becomes a step.
- *MINOR.* Two gaps that predate this issue, on lines it rewrote: X2 (`fire_fade_in` ends the arm
  only when it opens) and X3 (`end_ramp_windows` settles `Unmuting` outside
  `if *remaining == 0`).
- *MINOR.* The attempt-2 merge SHA was stale after the re-merge.

## Evidence (attempt 3)
`origin/main` (`10ce797ca`, the merge's second parent, including #2274 as `97a45db0a`) was merged
first (`8d181a6cb`); then
one commit.

**Changes (tests only).**
- New sibling test `lane_state::an_arm_holds_the_lane_until_its_fire_across_a_carry_and_a_reset`
  folds in the reviewer's probe:
  - a timed `SetGain` on an armed lane leaves `ramp_target == 0.0` (X6);
  - an exported armed lane imports with `is_armed == Some(true)` (X5);
  - `reset` keeps `is_armed == Some(true)` (X4, X5);
  - a lane armed muted whose fire stays muted, then unmuted, is open (X2).
- `an_unmute_holds_the_gain_only_while_its_countdown_runs` gains X3: an unmute ramp of
  `4 * FRAMES` still holds the gain after one block.

**Guard table, attempt 3.** Every arm of every `FaderPhase` method (`prepared`, `muted`,
`gain_held`, `settled`, `at_countdown`), every arm of the stage's `is_armed`, every phase write,
and every call site of each method (including the stage's `gain_held`, `is_muted`, `sync_mute`
and `timed_gain`). Each mutant was applied alone and reverted, against
`-p builtins --lib --test fader_ramp` on the merged tree; W1, W2 and W2b against `--lib --test
fader_ramp --test stage --test randomized --test contract`. The attempt-2 survivors R1 and R9 had
already run against builtins, builtins-compiler, graph and rack. "New" marks a mutant first run
in attempt 3. `lane_state::` is shortened to `ls::`, and `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute`
to `armed_fade_in`.

| Id | Site | Mutation | Killed by |
|---|---|---|---|
| W1 | `new` | prepared mute ignored | `a_move_made_while_muted_applies_on_unmute`, `ls::an_unmute_holds_…` |
| W2 | `arm_fade_in` | `opens` inverted | `armed_fade_in`, `ls::an_arm_holds_…` |
| W2b | `arm_fade_in` | `Armed` never written | `armed_fade_in`, `ls::an_arm_holds_…` |
| W3 | `fire_fade_in` | arm not ended | `armed_fade_in`, `ls::an_arm_holds_…` |
| X2 (new) | `fire_fade_in` | arm ended only when it opens | `ls::an_arm_holds_…` |

_Correction (attempt 4): W3 and X2 did not cover the value the closed fire writes (`Muted`); N26
replaces it and survived. Likewise W4 did not cover a windowed live move (N24). See attempt 4._

| Id | Site | Mutation | Killed by |
|---|---|---|---|
| W4 (M2) | `live_gain` | `Unmuting` not ended | `ls::an_unmute_holds_…` |
| W5 | `mute_channel`, armed | `opens = muted` | `armed_fade_in` |
| W5b | `mute_channel`, armed | `opens` not written | `armed_fade_in` |
| W6 | `mute_channel` | a mute enters `Open` | `a_repeated_mute_leaves_the_stage_as_it_was`, `a_zero_window_mute_is_the_exact_prepared_mute`, others |
| W7 | `mute_channel` | an instant unmute enters `Unmuting` | `ls::an_unmute_holds_…` |
| W8 | `mute_channel` | a ramped unmute enters `Open` | `ls::an_unmute_holds_…` |
| C3 (new) | `mute_channel` | mute mask not re-derived | `a_zero_window_mute_is_the_exact_prepared_mute`, `armed_fade_in`, others |
| W9 | `end_ramp_windows` | `Unmuting` not settled | `gain_moves_during_a_mute_ramp_are_remembered_and_never_restart_it` |
| X3 (new) | `end_ramp_windows` | settles every lane, ended or not | `ls::an_unmute_holds_…` |
| W10 | `export_lane` | phase not carried | `ls::a_fader_lane_imported_mid_fade_…`, `ls::an_arm_holds_…`, `ls::an_unmute_holds_…` |
| W11 | `import_lane` | phase copied verbatim | `ls::an_unmute_holds_…` |
| W11b | `import_lane` | phase not written | `ls::a_fader_lane_imported_mid_fade_…`, `ls::an_arm_holds_…`, `ls::an_unmute_holds_…` |
| C2 (new) | `import_lane` | mute mask not re-derived | `ls::a_fader_lane_imported_mid_fade_…` |
| W12 | `reset` | `Unmuting` not settled | `ls::an_unmute_holds_…` |
| X4 (new) | `reset` | ends an arm | `ls::an_arm_holds_…` |
| R1 | `arm_fade_in` | a re-arm overwrites the arm | survives: unreachable (accepted) |
| R2 | `fire_fade_in` | a lane not armed is fired too | `armed_fade_in` |
| R2b | `fire_fade_in` | `opens` ignored | `armed_fade_in` |
| R3 | `is_armed` | `opens` ignored | `armed_fade_in` |
| I2 (new) | `is_armed` | `opens` inverted | `armed_fade_in`, `ls::an_arm_holds_…` |
| R4 | `sync_mute` | mask from nothing | `a_zero_window_mute_is_the_exact_prepared_mute`, `armed_fade_in`, others |
| R5 | `live_gain` | a muted lane not held | `a_move_made_while_muted_applies_on_unmute`, `armed_fade_in`, others |
| R6 | `mute_channel` | the arm ignored | `armed_fade_in` |
| R7 | `mute_channel` | a repeated state is not a no-op | `a_repeated_mute_leaves_the_stage_as_it_was` |
| R8 | stage `gain_held` | never held | `gain_moves_during_a_mute_ramp_…`, `ls::an_arm_holds_…`, `ls::an_unmute_holds_…` |
| C1 (new) | `timed_gain` | always retargets | `gain_moves_during_a_mute_ramp_…`, `ls::an_arm_holds_…`, `ls::an_unmute_holds_…` |
| R9 | `lanes_at_rest` | `muted()` dropped from `zero_out` | survives: equivalent (accepted) |
| R10 | stage `is_muted` | never muted | `mute_is_a_fader_endpoint_and_settles_to_exact_positive_zero` |
| P5 | `FaderPhase::prepared` | inverted | `a_move_made_while_muted_applies_on_unmute`, others |
| P5a (new) | `prepared` | a muted lane prepares `Open` | `a_move_made_while_muted_applies_on_unmute`, `ls::an_unmute_holds_…` |
| P5b (new) | `prepared` | an unmuted lane prepares `Muted` | `a_repeated_mute_leaves_the_stage_as_it_was`, others |
| M1 (new) | `FaderPhase::muted` | `Muted` not muted | `a_move_made_while_muted_applies_on_unmute`, `armed_fade_in`, others |
| P1 | `muted` | `Armed` not muted | `armed_fade_in` |
| M3 (new) | `muted` | `Unmuting` muted | `gain_moves_during_a_mute_ramp_…`, `ls::an_unmute_holds_…`, others |
| M4 (new) | `muted` | `Open` muted | `a_move_made_while_muted_applies_on_unmute`, others |
| G4 (new) | `FaderPhase::gain_held` | `Open` held | `a_repeated_mute_leaves_the_stage_as_it_was`, `ls::an_unmute_holds_…`, others |
| P2 | `gain_held` | `Unmuting` not held | `ls::an_unmute_holds_…` |
| G3 (new) | `gain_held` | `Muted` not held | `gain_moves_during_a_mute_ramp_…`, `the_scalar_fader_refuses_a_lane_out_of_range` |
| X6 (new) | `gain_held` | `Armed` not held | `ls::an_arm_holds_…` |
| P3 | `FaderPhase::settled` | `Unmuting` kept | `gain_moves_during_a_mute_ramp_…`, `ls::an_unmute_holds_…` |
| S4 (new) | `settled` | `Open` becomes `Unmuting` | `rest_tests::a_fader_lane_called_at_rest_stays_there_on_a_silent_block` |
| S3 (new) | `settled` | `Muted` becomes `Open` | `gain_moves_during_a_mute_ramp_…`, `mute_is_a_fader_endpoint_…`, others |
| X5 (new) | `settled` | ends an arm | `ls::an_arm_holds_…` |
| P4 | `FaderPhase::at_countdown` | settles a live countdown | `ls::an_unmute_holds_…` |
| A2 (new) | `at_countdown` | never settles | `ls::an_unmute_holds_…` |

So 48 of 50 mutants are killed (the attempt-3 review then found N26 and N24, which this table
missed; attempt 4), each by a named test, and R1 and R9 survive with the reasons the
attempt-2 review accepted. Every arm of every `FaderPhase` method (`prepared` 2, `muted` 4,
`gain_held` 4, `settled` 4, `at_countdown` 2) and of `is_armed` (2) has a mutant that a named
test turns red.

**No bit moved, on the merged tree.** The attempt-1 harness (`gr1_bits.rs`, scratch, never
committed) was run on the merged tree twice: at head, and with `origin/main`'s
`crates/builtins/src/lib.rs` and `rest_tests.rs` in their place (read from `origin/main`, which another
worktree's fetch had by then moved to `fd33447dc`; neither file differs between `10ce797ca` and `fd33447dc`). Both give simd4
`c50ab7ba3c148cb5`, simd8 `c632900598d806ce`, scalar `1d4ee1dfcd9290db`, the attempt-1 values.

**Gates (head).** 
- `cargo test --locked -p builtins -p builtins-compiler --features builtins-compiler/test-support
  -p graph -p rack`: 42 binaries, all ok.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: `workspace policy: ok`.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (295 marked regions in
  46 files)` (the count rose with `origin/main`, not with this change).

## Verdict (attempt 3)
**FAIL**, on one MAJOR. The refactor still holds: no bit moved, and every attempt-2 mutant is
killed. But two new mutants survived builtins, builtins-compiler, graph and rack.
- *MAJOR, N26.* `fire_fade_in` writes `FaderPhase::Muted` for every fire. N26 writes `Open`
  instead for a fire that does not open the lane. Then:
  - `is_muted` reads `Some(false)`, so the control-plane readback reports a muted strip as unmuted;
  - a timed `SetGain` retargets the lane;
  - a peer lane's mute re-derives the mask (`sync_mute`), which clears the lane's bit, so the lane
    plays.

  The attempt-3 table had counted this write as covered by W3 and X2. Neither test checks the
  value the fire writes.
- *MINOR, N24.* `live_gain` ends `Unmuting` for every move. N24 ends it only for a move with no
  window, so a windowed live fader move during an unmute ramp keeps the hold. The tests moved the
  fader only with a window of `0`.
- *MINOR.* The attempt-3 evidence named the merged `origin/main` as `fd33447dc`; the merge's
  second parent is `10ce797ca`. Corrected above.

## Evidence (attempt 4)
`origin/main` at `de3263034` (the merge's second parent) was merged first, as `7a0e79c14`; then
one commit. `crates/builtins` is identical at `10ce797ca`, `fd33447dc` and `de3263034`.

**Changes (tests only).**
- `lane_state::an_arm_holds_the_lane_until_its_fire_across_a_carry_and_a_reset`. Right after a
  fire that stays muted, it now asserts that the lane is still muted
  (`is_muted(1, 0) == Some(true)`). It then applies a timed `SetGain`, mutes a peer lane (which
  re-derives the mask), and asserts that two rendered blocks of the lane are exact `+0.0`. The
  bank grows from two lanes to three, so the peer is a lane that is not armed. This kills N26.
- `an_unmute_holds_the_gain_only_while_its_countdown_runs`: lane 3's live fader move during its
  unmute ramp now has a window of 16 frames instead of 0. This kills N24, and M2 (W4) stays
  killed.

**Mutants, attempt 4** (`-p builtins --lib`, each alone and reverted):

| Id | Site | Mutation | Killed by |
|---|---|---|---|
| N26 | `fire_fade_in` | a closed fire writes `Open` | `ls::an_arm_holds_…` |
| N24 | `live_gain` | only a move with no window ends `Unmuting` | `ls::an_unmute_holds_…` |
| W3, X2, W4 | (re-run) | as above | `ls::an_arm_holds_…`; `ls::an_arm_holds_…`; `ls::an_unmute_holds_…` |
| W2, W2b, W7, W8, W10, W11b, W12, X3, X4, X5, X6, A2, I2 | (re-run after the edits) | as in attempt 3 | each still red, in `ls::an_arm_holds_…` or `ls::an_unmute_holds_…` |

**Follow-up, not part of this issue.** The wrapper channel-index mutants N12, N15 and N16 survive
because no dual-mono test arms or holds only one channel. That code predates #2280.

**No bit moved, on the merged tree.** `gr1_bits.rs` (scratch, never committed) gives
simd4 `c50ab7ba3c148cb5`, simd8
`c632900598d806ce` and scalar `1d4ee1dfcd9290db`. That holds at head, and also with
`de3263034`'s `crates/builtins/src/lib.rs` and `rest_tests.rs` in their place. These are the
attempt-1 values.

**Gates (head).** Each crate run on its own:
- `cargo test --locked -p builtins --features builtins/test-support`: 18 binaries ok.
- `cargo test --locked -p builtins-compiler --features builtins-compiler/test-support`: 8 ok.
- `cargo test --locked -p graph`: 9 ok.
- `cargo test --locked -p rack`: 7 ok.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps -p builtins`: clean.
- `bash scripts/check-workspace-policy.sh`: ok.
- `cargo run --locked --release -p realtime-policy`: `realtime policy: ok (298 marked regions in
  48 files)`. The count rose with `origin/main`.
