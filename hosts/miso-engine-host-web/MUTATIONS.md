| `test-web-audioworklet.mjs` unsupported-browser test (W4-D1) | delete the `if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");` guard in `createMisoAudioWorkletHost` | the refusal becomes a generic `miso.error.v1` 255 and the `compileCount` assertion fails |
| `test-web-audioworklet.mjs` source-ID UTF-8 parity test (#132) | change the four-byte sequence's `0xf0` lead-byte mask to `0xe0` in `writeBoundedUtf8` | the non-ASCII submit and seek bytes differ from the independent `TextEncoder` oracle |
| `check-web-audioworklet-callgraph.py --callgraph` on the shipped artifact (E1/E2) | restore `self.ready = None;` in `fail` and rebuild the artifact | closure 6 -> 23, traps 5 -> 16, 13 forbidden names appear (`drop_glue<Option<ReadyOwnership>>`, `drop_glue<PreparedRenderPlan>`, `drop_glue<SessionTomlV1>`, `BTreeMap<StableId,_>` drop glue, `Arc<spsc::Ring<_>>::drop_slow` x2, `__rdl_dealloc`, `__rust_dealloc`, `dlmalloc::free`, `unlink_chunk`, `insert_large_chunk`, ...), and four of them become unexpected trap owners |
| `check-web-audioworklet-callgraph.py --self-test` (a)-(f) | synthetic disassembly per case | each case is the red mutation of one rule; the runner fails if any escapes |
| `test-web-audioworklet.mjs` trap-containment test (F5) | remove the `try` around `miso_engine_web_v1_render` in `process()` | the `process()` call throws instead of returning `true` |
| `tests::facade_source_rules_reach_the_browser_host` (F1) | delete the `end_of_region != (end == region_end)` check from `miso_engine_host_core::SourceControlSet::submit` | the region-end submission returns `RESULT_BACKPRESSURE` (6) instead of `RESULT_INVALID_ARGUMENT` (1) |
| `tests::default_ring_covers_stall_tolerance` (F3) | `+ 2` -> `+ 1` in `default_source_ring_frames` | 48 000/128 yields 4 992 where 5 120 is required |
| `tests::ring_prefill_survives_stall` (F3) | `SOURCE_STALL_TOLERANCE_MS = 50` (a 21-quantum ring) | the ring runs dry mid-stall and a starved quantum renders zeros instead of the ramp |
| `test-web-audioworklet.mjs` pipelining test (F3) | make `#saturated` return `true` at one unsettled source request | the second of four in-flight chunks is refused and its planes are never transferred |
| `tests::native_identity_session_digest_pins_the_wasm_parity` + `direct-oracle.mjs` parity assertion (F4/E4) | flip one hex digit of `directOracle.nativePcmF32leSha256` in `expected.json` | both legs fail against the pin, and they fail with the same value |
| `qualification/run.mjs --self-test-mutations` attestation gate (#74) | change the supported result's attestation outcome to `miso.unsupported.v1` | `<browser>: attestation` fails because the probe and typed outcome disagree |
| `qualification/run.mjs --self-test-mutations` AudioWorklet boot gate (#74) | change the real worklet ready result to `false` | `<browser>: AudioWorklet-boot` fails |
| `qualification/run.mjs --self-test-mutations` native corpus gate (#74) | replace one in-browser PCM digest with 64 zeroes | `<browser>: native-corpus-digest` fails against the frozen native pin |
| `qualification/run.mjs --self-test-mutations` stall gate (#74) | change the measured injected-stall duration to zero | `<browser>: main-thread-stall` fails before a no-stall run can claim coverage |
| `qualification/run.mjs --check-matrix` deployment matrix gate (#74) | append `-red-mutation` to the checked version floor in memory | `<browser>: deployment-matrix` fails |
| `miso_engine_host_core::PreparedHost` `compile_fail` doctest (callback contract) | add `unsafe impl Sync for PreparedHost {}` | the doctest compiles and `cargo test --doc` exits 101 |
| `tests::command_ack_names_the_exact_application_sample` (#137 E1) | move the `while let Ok(record) = self.control.try_pop()` drain in `ConsoleMatrixProcessor::process` to after `self.matrix.process(block)` | the reported sample is one block early and the block at `applied_at_sample` still renders the pre-command value |
| `tests::command_flood_is_typed_backpressure_and_leaves_the_render_untouched` (#137 E3) | delete the free-room pre-check loop in `admit_commands` | the flood is admitted record by record until `try_push` fails, the transaction stops being all-or-nothing, and the flooded run's output differs from the clean run's |
| `tests::unknown_targets_are_typed_and_leave_the_engine_untouched` (#137 E4) | delete the `track >= track_count` leg in `admit_commands` | the unknown-track record is refused as `UNSUPPORTED` instead of `INVALID_ARGUMENT`/`UNKNOWN_TRACK` |
| `tests::meter_frames_equal_an_offline_fold_and_cost_the_render_nothing` (#137 E5) | make `console_request` use `blocks` frames instead of `blocks * quantum_frames` for the meter period | a window closes mid-block, `poll_meters` reports more windows than blocks rendered, and the cadence assertion fails |
| `tests::native_command_timeline_digest_pins_the_wasm_parity` + `direct-oracle.mjs::runCommandTimeline` (#137 E2) | change the matrix retarget's expected `applied_at_sample` to `2 * QUANTUM` | the native assertion fails; moving the drain in `ConsoleMatrixProcessor::process` instead moves both digests together, which is the point |
| `tests/matrix.rs::explicit_window_retarget_ramps_over_the_requested_window_and_is_adopted` (#137 D1) | drop `self.smoothing_samples[lane] = samples;` from `MatrixStage::set_target_over` | the second retarget runs over the prepared window of `0` instead of the requested `4` and settles on the first frame |
| `builtins-compiler::console_control_requests_are_validated_sealed_and_charged_per_track` (#137 D1) | delete the `control_tracks.insert` / `known_tracks.contains` legs in `prepare_session_builtins_with_console` | the duplicate and unknown-track requests are accepted instead of producing `builtin.control.duplicate` / `builtin.control.unknown_track` |
| `host-core::console_attaches_bounded_control_and_meter_halves_in_canonical_track_order` (#137 D1/D2) | drop the `bound.track_controls.len() != control_requests.len()` leg of the console arity check | a silently skipped channel leaves nine tracks with eight producers and the per-track walk panics |
| `check-web-audioworklet.sh --source-policy` pinned-post rule (#137 D2/D3) | rename one pinned post, drop the telemetry post, or remove the meter lease guard at its call site | the occurrence count or the pinned line no longer matches and the frozen render-callback policy fails; `test-web-audioworklet.sh` runs all three |
| `check-web-audioworklet.sh --source-policy` pinned-clock rule (#137 D3) | read `Date.now()` anywhere outside `renderClock()`, including inside `process()` | the pinned-site count disagrees, or `process_policy_re` catches it in the frozen body |
| `check-web-audioworklet-callgraph.py --self-test` (b1)/(b1b) (#137) | `--trap-owner` naming a different symbol, or `--allocation-only` over a closure that reaches a free | each case fails; neither new mode can admit an allocator |
| `check-parameter-metadata-v1.py --self-test` (#137 D4) | fourteen document mutations, including "a prepared-only builtin claims to be live" and "an effect parameter claims to be live" | every one is refused by the schema gate |
| `miso_engine_parameter_metadata -- --check` (#137 D4) | hand-edit one `liveUpdatable` in the shipped document | byte equality against a freshly generated document fails |
| `tools/miso-engine-parameter-metadata/tests/round_trip.rs` (#137 E7) | delete the `effect_index >= rack_effects[rack]` leg in `CommandRecord::into_matrix` | an out-of-range effect index is refused as `UNSUPPORTED_KIND`, so the test stops distinguishing "resolved" from "did not resolve" and its negative case fails |
| `qualification/run.mjs --self-test-mutations` control-path gates (#137 E8) | `exactRetargetedOutput = false`, `masterPeak = 0`, or `commandAdmitted = 0` | `<browser>: control-path` fails on the applied change, on the meter frame, and on the admission |
| `qualification/run.mjs --self-test-mutations` stall console load (#137 E6) | `stall.consoleMeterFrames = 0` | `<browser>: main-thread-stall` fails because the stall no longer carried a live command and meter load |

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-11 | the free-room pass reads `ready.command_wanted[0]` instead of `ready.command_wanted[slot]`, so a fader flood is checked against the matrix queue's count | `host-web/src/lib.rs` | `tests::a_mixed_batch_is_one_transaction_across_every_queue` | RED (`not even the matrix record in the refused batch reached the engine`) |
| 140-12 | the metadata emitter hardcodes `liveUpdatable: false` for every effect parameter again | `tools/miso-engine-parameter-metadata/src/lib.rs` | `scripts/check-parameter-metadata-v1.py` on the emitted document | RED (`FAIL parameter metadata: effect liveUpdatable follows automatable`) |

## Issue #143 — the effect observation surface

Every row applied to the working tree, the named binary run, the result recorded, the tree
restored. Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), `-C target-feature=+avx2,+fma`.

| gate | mutation | observed red |
|---|---|---|
| `tests::the_meter_frame_carries_the_app_shaped_gain_reduction` (E4) | publish the negative decibels raw instead of the declared `PeakMagnitude` fold | the app's `Math.max(0, -6)` is `0` and the frame reads dead; the "positive magnitude, not a negative decibel" assertion fires |
| `tests::observation_misuse_is_typed_and_all_or_nothing` (E8) | drop the all-or-nothing free-room pre-check for the observe kinds | the oversized batch reaches a queue and returns `255` where `6` (backpressure) was required |
| `tests::native_observation_timeline_digest_pins_the_wasm_parity` (E8) | an unknown tap answers `UnknownParameter` (5) instead of `UnknownTap` (10) | three tests fail; a caller could no longer tell which namespace it got wrong |
| `tests::a_computed_tap_is_refused_with_unsupported_kind` (E9) | bind the computed tap instead of refusing it | `None` where `Some(7)` was required — a bound computed tap is a lane that never publishes |
| `round_trip::every_metadata_observation_tap_resolves_through_a_command_acknowledgement` (E9) | offset the tap id by one in the lowering (equivalent to a hand-edited id in the document) | `miso.compressor tap 1 did not resolve`, reason `10` |
| `tests::the_meter_frame_carries_the_app_shaped_gain_reduction` (E4/D6) | `master_gr_present = 1` unconditionally | `no designation means absent, never zero`: `Some(0.0)` where `None` was required |
| `tests::the_meter_frame_carries_the_app_shaped_gain_reduction` (E4/D5) | keep the pre-#143 `2T + 2` frame shape | the frame is 8 words where `3T + 3 = 12` was required |
| `tests::native_observation_timeline_digest_pins_the_wasm_parity` (E8) | never clear an armed bit, so an unsubscribed tap keeps publishing its last window | `an unsubscribed tap publishes nothing`: `8.437999` where `0.0` was required |
| `tests::observation_unit_conversion_is_declared_and_clamped` (R4) | publish the linear reduction word unconverted | `0.5` reports `0.5 dB` instead of `6.02 dB` — a meter reading a tenth of the reduction actually happening |
| `test-web-audioworklet.mjs` main-realm frame validation (E4) | drop the "`trackGrDb` is finite and non-negative" rule | a `-6.5` frame is accepted; the rejection the test requires never arrives |
| `test-web-audioworklet.mjs` main-realm frame validation (E4) | drop the "`masterGrDb` is a number or `null`" rule | a `"6.5"` string frame is accepted |
| `test-web-audioworklet.mjs` processor frame test (E4) | the worklet posts the peak view where `trackGrDb` belongs | `frame.trackGrDb.every((value) => value === 6.5)` is false. The two fake sections carry different values precisely so this is visible |

### The two the browser gate catches instead

`subarray` inside the frozen `process()` policy body is banned — a per-block view is a per-block
allocation — so the first attempt at the frame post failed `check-web-audioworklet.sh` with
`render callback violates the frozen static policy`. The two views are built once, at construction.

The callgraph gate over the **shipped artifact** is green with the observation code in
`miso_engine_web_v1_meter_poll`'s closure: `closure=5 traps=2`, trap owner
`AudioWorkletEngineHost::poll_meters` and nothing else, and no allocator, deallocator or drop glue
anywhere in it.
