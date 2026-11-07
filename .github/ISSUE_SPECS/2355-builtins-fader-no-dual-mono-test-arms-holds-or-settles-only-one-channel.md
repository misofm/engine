# Builtins fader: no dual-mono test arms, holds or settles only one channel

## Origin

Found by #2280's reviews (tracker #2258, GR-1). These gaps predate #2280. The following mutants survive every crate:

- **N12:** the scalar `is_armed(0, 0)` reads a fixed channel.
- **N15:** the bank's `is_armed(lane, 0)`.
- **N16:** the bank's `gain_held(lane, 0)`.
- **F7:** `end_ramp_windows` settles the phase only for `channel == 0`, so the right channel's unmute hold never ends.

No test arms, holds or settles only one channel of a dual-mono strip, so a defect that reads or writes channel 0 for both channels is invisible.

Scope correction (issue comment): F8 and F14, with G3 and G5 (import or export uses channel 0's `remaining`), moved into #2354, as its per-channel lane carry contract. This issue keeps N12, N15, N16 and F7.

## Scope

- One table-driven test over the fader's per-channel queries and transitions. For each row: drive only the right channel (an unmute with a window, an arm, a fire), then assert each channel's `is_muted`, `is_armed` and `gain_held`, plus a rendered block per channel.
- Mutants N12, N15, N16 and F7 must each go red.
- If a mutant proves to be a real defect on main, fix it here.

## Non-goals

- `InputStage` (open #2364 edits it in the same file).
- The lane carry (`import_lane` / `export_lane`), which #2354 owns.

## Gates

- `builtins` (test-support) tests, clippy `-D warnings`, fmt, `scripts/check-workspace-policy.sh`.
- Mutants N12, N15, N16 and F7 are red.

## Dependency

Starts after the GR-1 phase-enum successor, #2354 (merged as #2360), because the phase enum changes these sites.

## Evidence and decision record

### Attempt 1

Base: `origin/main` at `4ba877b3b`.

**No real defect.** The new test is green on main, unchanged, so N12, N15, N16 and F7 were coverage gaps, not defects. No product code changes; the change is test-only.

**The test.** `a_transition_on_the_right_channel_leaves_the_left_channel_alone` in `crates/builtins/tests/fader_ramp.rs` runs on every bank width (simd4, and simd8 under AVX2) and on the scalar track (`FaderMuteRampBuiltins`). No existing `LaneState` test fits: those compare `process_with` against a split `process` and query only channel 0. The test therefore extends that file's `Subject` harness with:

- `subjects_with(left_muted)`, which can prepare lane 0's left channel muted; `subjects()` is `subjects_with(false)`.
- `channel_mute`, a mute on one channel.
- `channel_state`, which returns `[is_muted, is_armed, gain_held]` per channel.

There are four rows: an unmute with a window, a mute with a window, an arm whose left channel the session mutes, and a fire. Each row drives the right channel only. It then checks both channels' state after the drive, and the left channel against an undriven twin. Over three blocks of 128 frames, with a 200-frame window, the left plane must match the twin's bits in every block. After the window, the test checks both channels' state again, and lane 0's right plane must be exact `+0.0` or the twin's bits.

Test value: the test goes red when a per-channel query or transition reads or writes channel 0 for both channels. No existing test catches that, as the table below shows.

**Mutants.** Each was run against the full `cargo test --locked -p builtins --features test-support`. The mutants were applied one at a time with a scratch script and reverted.

| # | mutation (`crates/builtins/src/lib.rs`) | tests that fail |
| --- | --- | --- |
| N12 | `FaderMuteRampBuiltins::is_armed`: `stage.is_armed(0, lane)` -> `(0, 0)` | the new test only |
| N15 | `BuiltinFaderBank::is_armed`: `stage.is_armed(lane, channel)` -> `(lane, 0)` | the new test only |
| N16 | `BuiltinFaderBank::gain_held`: `stage.gain_held(lane, channel)` -> `(lane, 0)` | the new test only |
| F7 | `FaderRampStage::end_ramp_windows`: the phase settle runs only `if channel == 0` | the new test only |
| X1 (fresh) | `end_ramp_windows` settles from the left phase: `self.phase[0][lane].settled()` | the new test; `rest_tests::a_fader_lane_called_at_rest_stays_there_on_a_silent_block`; `randomized::the_banks_render_their_scalar_sections_under_random_retargets` |
| X2 (fresh) | `fire_fade_in` fires the left channel only (`0..1`) | the new test; `an_armed_fade_in_is_a_held_mute_that_fires_as_an_unmute` |
| X3 (fresh) | `mute_channel` re-derives the left mask: `sync_mute(0)` | the new test, and four existing tests in `lib`, `fader_ramp` and `stage` |
| X4 (fresh) | `BuiltinFaderBank::is_muted`: `stage.is_muted(lane, channel)` -> `(lane, 0)` | the new test only |
| X5 (fresh) | `FaderMuteRampBuiltins::is_muted`: `stage.is_muted(0, lane % 2)` -> `(0, 0)` | the new test only |

On main, without the new test, N12, N15, N16, F7, X4 and X5 survive every builtins test. The new test kills all nine. No existing test becomes redundant: X1, X2 and X3's other catchers defend different claims (exact rest, the arm's both-channel fire, the mute mask).

### Gates

All run on this branch, with `CARGO_TARGET_DIR` in the scratchpad and x86-64 pinned to `+avx2,+fma` (so simd8 runs):

- `cargo test --locked -p builtins --features test-support`: all 18 test binaries green.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: ok.
- `cargo test --locked -p graph-compiler --bin graph_fixture checked_in_fixtures`: green (the change is test-only).
