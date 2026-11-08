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
