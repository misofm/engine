# Tests: close the coverage gaps the implicit-state-machine reviews recorded

Successor bundle filed when the implicit-state-machine tracker #2258 closed. Each item below is a surviving mutant or an unreached path that an adversarial review recorded and left out of its own issue. Every row is independent; land them as separate commits, or split rows out, and delete any row found already covered.

## Rows
Each row names the plausible defect its test must turn red.

| # | Crate | Gap | Origin |
|---|---|---|---|
| T1 | builtins | `FaderMuteRampBuiltins::target_gain` and `is_muted` take `lane % 2`, so a lane above 1 wraps onto another channel, while their siblings return `None`. Make them total (return `None`) and test a lane above 1. | #2355 |
| T2 | builtins | Mutant H13: an export that takes `fader_gain` from lane 0 survives, because every carry test exports lane 0. Export from lane 1. H7 (a dead `ramp[channel].remaining` f32 write) is equivalent: remove the write instead. | #2354 |
| T3 | builtins | Symmetry mutants S1 and S2: `apply_prepared_filter` and `import_lane` may drop their symmetry refresh. Retarget one channel's filter, or import an asymmetric lane, then read `lane_symmetry`. | #2364 |
| T4 | control-plane | The `if true` replan mutant survives: no test pins that a declared discontinuity plans no transition (`transition` is `None`, the shape is `Ordinary`). | #2348 |
| T5 | host-core / builtins | Fader automation mutants f3 and n1: the cursor's muted `rehold` arm (`EndsAtBlockEnd`) is untested, and so is a curve unmute whose ramp crosses a block and is followed by a live write that frees the lane at the next entry. f2, n3 and n8 were judged probably equivalent: confirm each or record it equivalent. | #2362 |
| T6 | host-core | Detached threads with unbounded waits: `tests/session_seek.rs` `run`, and the cancel-race test in `src/spectrum.rs`. A render panic spins forever. Use scoped threads, a stop-on-drop guard (the `tools/bench-support/src/producer.rs` pattern) and named deadlines. No hang test is committed; injected-failure evidence goes in the PR. | #2296 |
| T7 | conformance | The randomized harness stops a scenario at its terminal words (`conformance/src/randomized.rs`, around lines 2175-2191), so its own-snapshot assertion never met #2324's state. Continue past them, or show why that is unsound. | #2324 |
| T8 | host-web (JS) | No test reads the released host's shell `state` after a control failure, so the shell's lifecycle mirror is unpinned. Pin it, or remove the mirror if it has no reader. | #2332 |
| T9 | host-web (JS) | 23 pre-existing factory-input, returned-plane and boot-reply validation guards in `miso-engine-v1-audio-worklet-host.js` have no rows (the list is in #2279's spec, at git history `a2a2fab94`). Add one table-driven row per guard, or delete any guard that is unreachable. | #2279 |

## Gates
- Each new test is red under its named mutant or defect. The PR records the red run, and the test-value sentence is answered per row.
- No product behaviour change except T1's `None` and T2's dead write. T1 is API-visible only within builtins.
- One test per claim: a row whose claim an existing test already covers is deleted.

## Evidence / attempt record
Attempt 1 (Terra), branch `codex/issue-2377-coverage-gaps` from `origin/main` `90932ff03`. One commit per row.

### T1 (#2355): the scalar fader's `target_gain` and `is_muted` refuse a lane above 1
- Claim held at base: `crates/builtins/src/lib.rs` `FaderMuteRampBuiltins::target_gain` and `is_muted` read `self.stage.*(0, lane % 2)` while `is_armed`, `gain_held` and `ramp_remaining` return `None` above lane 1.
- Fix: both return `Option` and `None` for a lane above `1`. Callers: builtins-compiler's `FaderJumpStage` impl (`== Some(true)`) and its automation closure (`?`, which already returns `Option`), plus `tests/fader_ramp.rs`. No rendered sample changes: the methods are readbacks.
- Test: `the_scalar_fader_refuses_a_lane_out_of_range` (`crates/builtins/tests/fader_ramp.rs`) gains `is_muted(2) == None` and `target_gain(3) == None`, with the in-range answers beside them.
- Red under the defect: restoring the wrap (`Some(self.stage.is_muted(0, lane % 2))`, no `None` arm) fails at `fader_ramp.rs:1214` `left: Some(true) right: None`; the same for `target_gain` alone fails at `:1216` `left: Some(1.0) right: None`.
- Test value: a lane query that wraps a lane above 1 onto another channel turns it red, and no other test asks the scalar fader about such a lane.

### T2 (#2354): the fader carry is exported from lane 1; the dead countdown-word write is removed
- H13: the first scenario of `lane_state::a_fader_lane_imported_mid_fade_renders_what_the_exported_lane_renders` (`crates/builtins/src/lib.rs`) now mutes, moves and exports source lane `1`, whose remembered gain (left -4 dB, right -9 dB) differs from lane 0's (left -3 dB, right -6 dB). No new test.
- Red under H13 (`FaderRampStage::export_lane` reads `self.fader_gain[channel][0]`): with the change the test fails at `lib.rs:7673` (the unmute block pair); with the base test the same mutant leaves the `lane_state` module green (`4 passed`), as #2354 recorded.
- Test value: an export that reads the remembered fader gain, or any per-lane word, from lane 0 instead of the exported lane turns it red, because the unmute then returns to lane 0's gain; every other carry test exports lane 0.
- H7 confirmed equivalent and removed: `import_lane` wrote `ramp[channel].remaining` (the kernel's `f32` countdown word), but the only reader is `gain_mute_ramp_block`, called once, from `process_plane`, which rebuilds the word from the `u32` countdown right before it (`lib.rs` `process_plane`). `rest_tests::lane_state` already left the word out as residue. `reset`'s zeroing of the same word is equally dead and is left alone (outside the row).
- No bit moved (one-time, not committed): a scratch test drove two 3-lane fader banks per width through 400 seeded blocks of mutes, fader moves with windows 0-1000, cross-bank imports and same- and cross-bank `swap_lanes`, hashing every output word. Base and change both print `9fa1fa1f8be8817c`; a control mutant (`import_lane` halving the `u32` countdown) prints `4d4eabd9a34d5804`, so the digest sees an import.
- Gates: `cargo test -p builtins --features test-support` green; `realtime_policy` ok (298 regions).

### T3 (#2364): a one-channel filter retarget and an asymmetric import refresh `lane_symmetry`
- Claim held at base: with S1 (`InputStage::apply_prepared_filter` drops its `refresh_channel_symmetry()`) or S2 (`InputStage::import_lane` drops `refresh_channel_symmetry_lane(lane)`), the whole `cargo test -p builtins --features test-support` stays green.
- Test: `tests::a_one_channel_filter_retarget_and_an_asymmetric_import_refresh_lane_symmetry` (`crates/builtins/src/tests.rs`). A 3-lane bank gets a left-only high-pass on lane 1; that lane is exported over lane 2 of a second bank, then a symmetric lane is imported back over it; `lane_symmetry(..).eligible()` is read for every lane after each step. Product code: only the `import_lane` comment that said no test needs the symmetry refresh.
- Red: S1 debug fails at `lib.rs:2750` (`the cached channel-symmetry flag is stale for lane 1`), release at `tests.rs:887` (`left: [true, true, true] right: [true, false, true]`). S2 debug fails at `lib.rs:2750` (`stale for lane 2`), release at `tests.rs:896` (`left: [true, true, true] right: [true, true, false]`).
- Test value: a filter retarget or lane import that leaves the held symmetry bit stale turns it red; without it, the bank would collapse a lane whose channels differ onto its left channel, and no other test reads `lane_symmetry` right after either writer.

### T4 (#2348): a declared stop over a ducking candidate plans no transition
- Claim held at base: the `if true` replan mutant (`if !matches!(shape, SuccessorShape::Ordinary)` -> `if true`, `crates/control-plane/src/control.rs`) is observable only when the superseded candidate's overlay ducks a strip: the stop's transition is then planned, joins that overlay, and the stop is held `NoEarlierThan(S)` instead of `Next`. The existing declared-stop rows supersede a W whose overlay is empty, where the mutant is equivalent.
- Test: a fourth row of `warm_successor::edits_while_the_warm_candidate_waits`: W (growth) pending, then B (remove `eq1` and `aux-source`) published `Primed { not_before: S }` with `S` past the next block, then `declare_discontinuity`; the swap is at block `BEFORE + 1`.
- Red under the mutant: `control.rs:11709` `the stop is adopted at once`, `left: [] right: [7]`.
- Test value: a declared discontinuity that plans a transition, and so inherits the ducking overlay of the candidate it supersedes, waits for that overlay's `S` instead of being adopted at the next block; no other test declares a stop over a candidate that ducks a strip.

### T5 (#2362): fader automation mutants f3, n1, f2, n3 and n8
Mutants (all in `crates/builtins-compiler/src/lib.rs`), each run against `cargo test -p builtins-compiler --features test-support` and host-core's `stored_fader_automation`, `stored_mute_automation`, `stored_vca_automation`, `stored_automation`, `vca` and `successor_swap` (test-support):
- **n1** (`FaderCellCursor::rehold` records `EndsAtBlockEnd` for any unmute ramp that ends past the block, not only one that ends on its last sample): killed only by the new test `a_curve_unmute_ramping_past_its_block_and_freed_at_the_next_entry_releases_its_hold` (`crates/host-core/tests/stored_fader_automation.rs`), at `:3873` `plane 0: completion sample 1087`, `left: 1048236726 right: 1027377908`. The test: a fade from -30 to 0 dB under a mute curve muted over `[300, 900)` (mute length 240), so the curve unmute at 900 ramps past its block's end at 1,024; a live fader write at the entry 1,024 frees the lane; from 1,087 every completion sample `64k + 63` equals the curve, on every render form. Test value: a curve-unmute hold that runs past the block, recorded as ending on its last sample, makes the owed move due at the next entry where a live write freed the lane; the move then lands over `J` instead of the R17-10 release, and the lane lags the curve. No other test frees a lane at the entry after a curve unmute that crosses a block.
- **f3** (the muted arm of `rehold` records `EndsAtBlockEnd`): **equivalent; no test.** `HeldRampObligation::block` reads the entry only in its free-stage arms; a held stage overwrites it. After a muted `rehold` the stage is muted at the next entry: its mute is `curve || terms` and the curve was heard muted, so a live mute record cannot clear it (`mute_terms` returns when `heard == stage_muted`), a fader or offsets write on a muted stage is only remembered, and the curve's own unmute is pushed inside the next block, after `block()` has read the entry. Probe (scratch, not committed): curves muted over `[300, 900)`, `[380, 2000)` and `[383, 2000)`, at mute lengths 5 ms and 0 ms, with each of an exact live unmute, a live fader write, a hand mute then unmute, an offsets jump at 384, and session seeks at 384 to 100, 1,000 and 3,000. All 42 runs hash the same output words on base and under f3.
- **f2** (the R17-10 release also fires on `EndsAtBlockEnd` when nothing is owed): survives; **recorded equivalent**, as the #2362 review judged. A hold that ended on the block's last sample has the cell's hold of it ending on the same sample, so the release at the next entry's node time minus one shortens nothing.
- **n3** (the due arm of `block()` also releases the cell's hold): survives; **recorded equivalent**. The due move lands at offset 0 and re-commits the cell from its own landing, which supersedes the hold the release would shorten.
- **n8** (`mute_terms` ignores whether the block has jump lengths): survives; **recorded probably equivalent**, as the #2362 review judged, not proven here. `take_block_time` takes the span and `block_jumps(time)` from the same `GraphBlockTime`, whose `jump_lengths()` is `None` only for a plan with no jump-length cell (`crates/graph/src/lib.rs` `GraphBlockTime::jump_lengths`); the mutant differs only on a repositioned cell in such a plan, and no suite builds one.
- Product code unchanged. Gates: the suites above green at head; clippy (`-p host-core --all-targets --all-features`) and fmt clean.
