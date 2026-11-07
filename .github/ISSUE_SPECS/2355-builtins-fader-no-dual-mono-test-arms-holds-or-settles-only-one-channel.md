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

**The test.** `a_transition_on_the_right_channel_leaves_the_left_channel_alone` in `crates/builtins/tests/fader_ramp.rs` runs on every bank width (simd4, and simd8 under AVX2) and on the scalar track (`FaderMuteRampBuiltins`). It lives in `fader_ramp.rs`, not in `lib.rs`'s `lane_state` module, because that file's `Subject` harness already runs one body over all three subjects (simd4, simd8, scalar). The `lane_state` tests are bank-only. The test extends the harness with:

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

**Verdict.** The review of attempt 1 (head `80b4f0adc`) returned **PASS**, with MINORs only. Three of them are folded in below, after a merge of `origin/main` at `7ba45651b`.

### Folds after the PASS

1. **`ramp_remaining` and the target gain are columns of `channel_state`.** Mutants R1, R7 and R2 survived every builtins test, and `ramp_remaining` has product consumers in builtins-compiler.
   - `channel_state` now returns a `ChannelState`: the three flags, plus `ramp_remaining` and the bits of the target (`ramp_target` on the bank, `target_gain` on the scalar).
   - Each row states the right channel's countdown after the drive: `200` for the windowed rows and the fire, `0` for the arm.
   - After every block, the countdown must be that value less the frames rendered, saturating at `0`.
   - The right target must be `+0.0` or the undriven twin's.
   - The left channel's whole `ChannelState` must equal the twin's after the drive and after every block.
2. **R6, in `FaderRampStage::process_from_left`.** The guard reads `remaining[0]` where it should read `remaining[1]`, so the right channel's settled guard reads the left channel's countdown. It survived builtins, and it also survived `rack`, `graph` and `builtins-compiler`, each run alone. That is an audio defect on the mono-source path: a right ramp beside a settled left one freezes.
   - The table now runs twice, once on a dual input and once on a mono one (`Subject::process_mono`). The mono run feeds a right input equal to the left plane and renders through the bank's `process_from_left`; when that declines, it copies the input and calls `process`, as the chain's caller does. The scalar track has no such path, so it copies and calls `process`.
   - Under R6, the per-block countdown check catches the freeze: simd4, unmute row, mono, block 0 reads `200` where `72` is due.
3. **Sibling lanes.** In every block, the right plane's lanes other than `0` must match the twin's bits, so a transition that leaks to a sibling lane is red.

| # | mutation (`crates/builtins/src/lib.rs`) | tests that fail (full builtins suite) |
| --- | --- | --- |
| R1 | `FaderMuteRampBuiltins::ramp_remaining`: `(0, lane)` -> `(0, 0)` | the new test only |
| R7 | `BuiltinFaderBank::ramp_remaining`: `(lane, channel)` -> `(lane, 0)` | the new test only |
| R2 | `FaderMuteRampBuiltins::target_gain`: `(0, lane % 2)` -> `(0, 0)` | the new test only |
| R8 (fresh) | `BuiltinFaderBank::ramp_target`: `(lane, channel)` -> `(lane, 0)` | the new test only |
| R6 | `FaderRampStage::process_from_left`: the guard reads `remaining[0]` | the new test only |

After the folds, all nine attempt-1 mutants are still red on `--test fader_ramp`: N12, N15, N16, F7 and X1 to X5. Without the new test, every R mutant survives; R6 survives in rack, graph and builtins-compiler too.

### Follow-up (not filed)

`FaderMuteRampBuiltins::target_gain` and `is_muted` take `lane % 2`, so a lane above `1` wraps silently onto another channel. Their siblings `is_armed`, `gain_held` and `ramp_remaining` return `None` for such a lane. No caller passes a lane above `1` today, so it is unreachable; making them return `None`, or making them total over a channel type, is a separate API change.

### Gates

Re-run after the folds on the tree merged with `origin/main` `7ba45651b`, with the same results. All run on this branch, with `CARGO_TARGET_DIR` in the scratchpad and x86-64 pinned to `+avx2,+fma` (so simd8 runs):

- `cargo test --locked -p builtins --features test-support`: all 18 test binaries green.
- `cargo clippy --locked -p builtins --all-targets --all-features -- -D warnings`: clean.
- `cargo fmt --check`: clean.
- `bash scripts/check-workspace-policy.sh`: ok.
- `cargo test --locked -p graph-compiler --bin graph_fixture checked_in_fixtures`: green (the change is test-only).
