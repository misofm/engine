| `test-web-audioworklet.mjs` unsupported-browser test (W4-D1) | delete the `if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");` guard in `createMisoAudioWorkletHost` | the refusal becomes a generic `miso.error.v1` 255 and the `compileCount` assertion fails |
| `test-web-audioworklet.mjs` source-ID UTF-8 parity test (#132) | change the four-byte sequence's `0xf0` lead-byte mask to `0xe0` in `writeBoundedUtf8` | the non-ASCII submit and seek bytes differ from the independent `TextEncoder` oracle |
| `check-web-audioworklet-callgraph.py --callgraph` on the shipped artifact (E1/E2) | restore `self.ready = None;` in `fail` and rebuild the artifact | closure 6 -> 23, traps 5 -> 16, 13 forbidden names appear (`drop_glue<Option<ReadyOwnership>>`, `drop_glue<PreparedRenderPlan>`, `drop_glue<SessionModel>`, `BTreeMap<StableId,_>` drop glue, `Arc<spsc::Ring<_>>::drop_slow` x2, `__rdl_dealloc`, `__rust_dealloc`, `dlmalloc::free`, `unlink_chunk`, `insert_large_chunk`, ...), and four of them become unexpected trap owners |
| `check-web-audioworklet-callgraph.py --self-test` (a)-(f) | synthetic disassembly per case | each case is the red mutation of one rule; the runner fails if any escapes |
| `test-web-audioworklet.mjs` trap-containment test (F5) | remove the `try` around `miso_engine_web_v1_render` in `process()` | the `process()` call throws instead of returning `true` |
| `tests::facade_source_rules_reach_the_browser_host` (F1) | delete the `end_of_region != (end == region_end)` check from `host_core::SourceControlSet::submit` | the region-end submission returns `RESULT_BACKPRESSURE` (6) instead of `RESULT_INVALID_ARGUMENT` (1) |
| `tests::default_ring_covers_stall_tolerance` (F3) | `+ 2` -> `+ 1` in `default_source_ring_frames` | 48 000/128 yields 4 992 where 5 120 is required |
| `tests::ring_prefill_survives_stall` (F3) | `SOURCE_STALL_TOLERANCE_MS = 50` (a 21-quantum ring) | the ring runs dry mid-stall and a starved quantum renders zeros instead of the ramp |
| `test-web-audioworklet.mjs` pipelining test (F3) | make `#saturated` return `true` at one unsettled source request | the second of four in-flight chunks is refused and its planes are never transferred |
| `tests::native_identity_session_digest_pins_the_wasm_parity` + `direct-oracle.mjs` parity assertion (F4/E4) | flip one hex digit of `directOracle.nativePcmF32leSha256` in `expected.json` | both legs fail against the pin, and they fail with the same value |
| `qualification/run.mjs --self-test-mutations` attestation gate (#74) | change the supported result's attestation outcome to `miso.unsupported.v1` | `<browser>: attestation` fails because the probe and typed outcome disagree |
| `qualification/run.mjs --self-test-mutations` AudioWorklet boot gate (#74) | change the real worklet ready result to `false` | `<browser>: AudioWorklet-boot` fails |
| `qualification/run.mjs --self-test-mutations` native corpus gate (#74) | replace one in-browser PCM digest with 64 zeroes | `<browser>: native-corpus-digest` fails against the frozen native pin |
| `qualification/run.mjs --self-test-mutations` stall gate (#74) | change the measured injected-stall duration to zero | `<browser>: main-thread-stall` fails before a no-stall run can claim coverage |
| `qualification/run.mjs --check-matrix` deployment matrix gate (#74) | append `-red-mutation` to the checked version floor in memory | `<browser>: deployment-matrix` fails |
| `web-audioworklet-identity.py` candidate lineage rule (#338; moved from `run.mjs --check-matrix` by #1061, release changes only) | record a 39-hex-digit `candidateCommit` in a release change | `--self-test`: `**FAIL** candidate-lineage` |
| `web-audioworklet-identity.py` artifact lineage rule (#338; moved from `run.mjs --check-matrix` by #1061, release changes only) | record another module's `wasmSha256` in a release change | `--self-test`: `**FAIL** artifact-lineage` |
| `host_core::PreparedHost` `compile_fail` doctest (callback contract) | add `unsafe impl Sync for PreparedHost {}` | the doctest compiles and `cargo test --doc` exits 101 |
| `tests::command_ack_names_the_exact_application_sample` (#137 E1) | move the `drain_controls` call in `LiveControlMatrixProcessor::process` to after `self.matrix.process(block)` | the reported sample is one block early and the block at `applied_at_sample` still renders the pre-command value |
| `tests::command_flood_is_typed_backpressure_and_leaves_the_render_untouched` (#137 E3) | delete the free-room pre-check loop in `admit_commands` | the flood is admitted record by record until `try_push` fails, the transaction stops being all-or-nothing, and the flooded run's output differs from the clean run's |
| `tests::unknown_targets_are_typed_and_leave_the_engine_untouched` (#137 E4) | delete the `track >= track_count` leg in `admit_commands` | the unknown-track record is refused as `UNSUPPORTED` instead of `INVALID_ARGUMENT`/`UNKNOWN_TRACK` |
| `tests::meter_frames_equal_an_offline_fold_and_cost_the_render_nothing` (#137 E5) | make `live_control_request` use `blocks` frames instead of `blocks * quantum_frames` for the meter period | a window closes mid-block, `poll_meters` reports more windows than blocks rendered, and the cadence assertion fails |
| `tests::native_command_timeline_digest_pins_the_wasm_parity` + `direct-oracle.mjs::runCommandTimeline` (#137 E2) | change the matrix retarget's expected `applied_at_sample` to `2 * QUANTUM` | the native assertion fails; moving the drain in `LiveControlMatrixProcessor::process` instead moves both digests together, which is the point |
| `tests/matrix.rs::explicit_window_retarget_ramps_over_the_requested_window_and_is_adopted` (#137 D1) | drop `self.smoothing_samples[lane] = samples;` from `MatrixStage::set_target_over` | the second retarget runs over the prepared window of `0` instead of the requested `4` and settles on the first frame |
| `builtins-compiler::live_control_requests_are_validated_sealed_and_charged_per_track` (#137 D1) | delete the `control_tracks.insert` / `known_tracks.contains` legs in `prepare_session_builtins_with_live_controls` | the duplicate and unknown-track requests are accepted instead of producing `builtin.control.duplicate` / `builtin.control.unknown_track` |
| `host-core::live_controls_attach_bounded_control_and_meter_halves_in_canonical_track_order` (#137 D1/D2) | drop the `bound.track_controls.len() != control_requests.len()` leg of the live-control arity check | a silently skipped channel leaves nine tracks with eight producers and the per-track walk panics |
| `check-web-audioworklet.sh --source-policy` pinned-post rule (#137 D2/D3) | rename one pinned post, drop the telemetry post, or remove the meter lease guard at its call site | the occurrence count or the pinned line no longer matches and the frozen render-callback policy fails; `test-web-audioworklet.sh` runs all three |
| `check-web-audioworklet.sh --source-policy` pinned-clock rule (#137 D3) | read `Date.now()` anywhere outside `renderClock()`, including inside `process()` | the pinned-site count disagrees, or `process_policy_re` catches it in the frozen body |
| `check-web-audioworklet-callgraph.py --self-test` (b1)/(b1b) (#137) | `--trap-owner` naming a different symbol, or `--allocation-only` over a closure that reaches a free | each case fails; neither new mode can admit an allocator |
| `check-parameter-metadata-v1.py --self-test` (#137 D4) | fourteen document mutations, including "a prepared-only builtin claims to be live" and "an effect parameter claims to be live" | every one is refused by the schema gate |
| `parameter_metadata -- --check` (#137 D4) | hand-edit one `liveUpdatable` in the shipped document | byte equality against a freshly generated document fails |
| `tools/parameter-metadata/tests/round_trip.rs` (#137 E7) | delete the `effect_index >= rack_effects[rack]` leg in `CommandRecord::into_matrix` | an out-of-range effect index is refused as `UNSUPPORTED_KIND`, so the test stops distinguishing "resolved" from "did not resolve" and its negative case fails |
| `qualification/run.mjs --self-test-mutations` control-path gates (#137 E8) | `exactRetargetedOutput = false`, `masterPeak = 0`, or `commandAdmitted = 0` | `<browser>: control-path` fails on the applied change, on the meter frame, and on the admission |
| `qualification/run.mjs --self-test-mutations` `stall-live-controls-load` (#137 E6) | `stall.liveControlMeterFrames = 0` | `<browser>: main-thread-stall` fails because the stall no longer carried a live command and meter load |

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-11 | the free-room pass reads `ready.command_wanted[0]` instead of `ready.command_wanted[slot]`, so a fader flood is checked against the matrix queue's count | `host-web/src/lib.rs` | `tests::a_mixed_batch_is_one_transaction_across_every_queue` | RED (`not even the matrix record in the refused batch reached the engine`) |
| 140-12 | the metadata emitter hardcodes `liveUpdatable: false` for every effect parameter again | `tools/parameter-metadata/src/lib.rs` | `scripts/check-parameter-metadata-v1.py` on the emitted document | RED (`FAIL parameter metadata: effect liveUpdatable follows automatable`) |

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

## Issue #143 E12 — the three-browser observation row

`qualification/run.mjs --browser all --self-test-mutations --record-matrix --candidate-commit <40-hex>`, Playwright 1.62.1
headless Linux, over the shipped `simd128` artifact.

```
chromium: all qualification gates passed (151.0.7922.34)
firefox:  all qualification gates passed (153.0)
webkit:   all qualification gates passed (26.5)
```

The row subscribes to the compressor's declared tap, renders sixteen blocks, and requires:
`trackGrDb` positive and finite, `masterGrDb` equal to the designated track's own reading,
`firstSample` strictly monotonic with the windows tiling, an unsubscribe that actually stops the
traffic, and the armed and unarmed renders of the same sixteen blocks producing **bit-identical
audio**. Four self-test mutations run against every browser's real result:

| mutation | gate |
|---|---|
| `observation-armed` (the eval's named case, `observationArmed = 0`) | `an armed tap published no reduction at all` |
| `observation-unsubscribe` | `an unsubscribed tap kept publishing` |
| `observation-identity` | `arming a declared tap moved a rendered sample` |
| `observation-window` | `observation windows did not advance monotonically and tile` |

### And the same mutation against a real engine, in a real browser

The self-test mutates a *result*. To prove the gate catches a mutated *engine*, the
`ObservationLane::accumulate` armed guard was changed to `return;` — so no armed tap ever
accumulates — the browser artifact was rebuilt from that tree, and chromium was re-qualified:

```
Error: chromium: observation-armed: an armed tap published no reduction at all
```

Reverted in the same session; the recorded `results.json` and matrix are from the unmutated tree.

## Issue #151 — the command-reason cap and the `observe()` typing gap

The field defect, found in `misofm/app` PR #32: `#receive` bounded a command acknowledgement's
`reason` at the literal `<= 9`, but #143 froze `UNKNOWN_TAP = 10` and `OBSERVATION_UNBOUND = 11`
and those are the **only** two reasons the observation path ever returns. Either one therefore read
as a malformed acknowledgement and tripped the host-wide sticky 255, so a single refused
subscription failed every unsettled request and every later one. That is what kept the app's
gain-reduction meters dead. The shipped metadata JSON's `commandReasons` vocabulary stopped at `9`
for the same reason, and the `.d.ts` declared neither `observe()` nor the request-side subscription
type at all.

Every row below was applied, the named gate run, the red observed, and the tree restored. Host:
`x86_64`, `-C target-feature=+avx2,+fma`, toolchain 1.97.1.

| gate | mutation | observed red |
|---|---|---|
| `test-web-audioworklet.mjs` observation-refusal tests (the shipped defect) | restore `validU32(message.reason) && message.reason <= 9` in `#receive` | `{ tag: 'miso.error.v1', requestId: 250, result: 255 }` — the sticky signature, thrown out of the *first* refused `observe()` instead of settling as a typed `miso.observe.v1` ack. `test-web-audioworklet.sh` runs this mutation on disk and requires the suite red |
| `check-command-reason-vocabulary.py` (the drift class) | add `pub const COMMAND_REASON_FUTURE_TAP: u32 = 15;` after reason 14 (`UNKNOWN_VCA`, issue #1245; it was 14 after reason 13 under issue #1222, and 13 after reason 12 under issue #1212) in `host-web/src/lib.rs` and nothing else | `host JS table disagrees with the Rust host constants` — a Rust reason bumped without the other five spellings. `test-web-audioworklet.sh` performs this one on a copied file tree, not only in memory |
| `check-command-reason-vocabulary.py --self-test` | twenty in-memory mutations across all six spellings, among them: a renumbered Rust constant; the JS table truncated at `wrongState`; the literal `<= 9` reinstated; the derived bound replaced by `reason <= 14`; the `.d.ts` enum missing or renaming a reason; a generator row dropped or emitting the wrong name for its own constant; the schema gate's list truncated; the render-thread worklet renumbering or renaming the one reason it produces itself | every one refused |
| `check-command-reason-vocabulary.py --self-test` (#151's typing half) | drop `observe()` from `MisoAudioWorkletHost`; drop `windowBlocks` from the declared subscription; add a `channel?` the implementation refuses; drop `frameSlot` from the declared binding; drop `reason` from the declared ack; add a binding field to the implementation the `.d.ts` does not declare | every one refused — the declaration is held to the shipped implementation's actual field sets, not to the issue's sketch |
| `check-parameter-metadata-v1.py --self-test` | truncate `commandReasons` at `wrongState`; rename reason 10; renumber reason 14 to `15` (issue #1245; reason 13 to `14` under issue #1222, reason 12 to `13` under issue #1212, and reason 11 to `12` before that) | `command reasons` / `command reason values` — the exact shape of the shipped vocabulary drift |

## Issue #241 — source introspection follows the declaration

Issue #241 deletes the per-source rate and start frame, leaving exactly four queries:
`source_count`, `source_id`, `source_channels`, and `source_frames`. The session-map row is exactly
`{ id, channels, frames }`; the root session status remains the sole sample-rate authority.

Every product mutation below was applied to the working tree on 2026-08-29, the named gate was
run, RED was observed, and the mutation was reverted.

| gate | mutation | observed red |
|---|---|---|
| `check-session-map-shape.py --self-test` | fifteen in-memory mutations across the Rust exports, exact export list, worklet reads/posts, main-realm field sets, and `.d.ts` types | all 15 refused; the normal gate then reported one shape across all spellings |
| `check-web-audioworklet.sh` frozen export set | restore `miso_engine_web_v1_source_sample_rate(handle,index)->u32` in the Rust FFI | RED with an exact export diff naming `+miso_engine_web_v1_source_sample_rate`; a deleted query cannot remain as an unused compatibility export |
| `test-web-audioworklet.mjs` source-binding tests | five mis-wired surviving reads: zero channels, channels past the configured maximum, zero frames, empty ID, and ID longer than staging | each fails initialization with sticky `255`; the unmutated suite reports `web AudioWorklet hermetic tests passed` |
| `check-session-map-shape.py` copied-tree width mutation | `readonly frames: bigint` → `number` | the gate derives JavaScript `bigint` from the Rust `u64` export and refuses the declaration drift |
| `tests::session_source_introspection_is_canonical_ordered_shaped_and_bounded` | reverse the normalized source list | declaration order leaks as `["zeta", "mid", "alpha"]` where canonical `["alpha", "mid", "zeta"]` is required |
| `direct-oracle.mjs` (real module/session) | copy the canonical track ID instead of the source ID | `track` differs from `fixture-source`; the real-module oracle addresses the source by the ID introspection reports |

## Issue #210 phase 1 — solo in place

Solo is 100% control plane, so every pin below is either a rendered-sample assertion or an
assertion on the one piece of host state the ABI deliberately has no readback for. Each mutation
was performed on the working tree, run, and reverted; every one was observed red.

| gate | mutation | observed red |
|---|---|---|
| `tests::solo_is_bit_identically_mute_on_the_complement` (P1-1) | drop `&& !self.solo(track)` from `LiveControlSoloState::effective_mute`, so the gate silences everything | the soloed tracks silence with the rest and the first commanded block differs from the explicit-mute arm |
| `tests::un_solo_restores_the_exact_per_lane_user_mute_set` (P1-2) | restore from the gate alone — `strip_delta` composes `any_solo && !solo(track)` instead of `effective_mute` | the session's baked `left_mute` comes back unmuted and every block after the settle differs from the never-soloed arm |
| `tests::mute_and_solo_are_separate_states` (P1-4) | make `set_solo` clear that track's `user_mute` | a repeated solo engage un-mutes the track it re-engages, and the host mirror reads `[false, false]` where the user set `[true, true]` |
| `tests::a_refused_solo_submission_leaves_the_live_controls_untouched` (P1-5) | delete the `ready.solo.rollback()` on `admit_commands`'s refusal path | the refused engage sticks in host state; the refused host and the untouched host diverge on the retry |
| `tests::a_solo_that_changes_nothing_emits_nothing` (the −0.0 pin) | drop the changed-lanes test in `LiveControlSoloState::strip_delta` — `match (true, true)` | soloing the only track of a one-track console re-mutes its already-settled-muted lanes, the ramp kernel runs instead of the fill, and a negative input renders `-0.0` where the settled path renders exact `+0.0` |
| `tests::a_batch_of_alternating_solo_toggles_coalesces_to_its_net_effect` (the coalescing pin) | run the net-emission pass once per solo record rather than once per submission, and drop its `record_emitted` sync — per-command fan-out | a 256-record batch of alternating toggles fans out a gate record per track per transition and is refused instead of admitted |
| `tests::live_controls_that_never_solo_render_what_they_always_did` (the class-A OFF gate) | route `mute` through the coalesced net emission instead of staging its own record | the redundant re-mute of a settled-muted lane stages nothing, the plane stays `+0.0`, and the pinned `-0.0` ramp block is gone — a digest change on a path no solo command touched |
| `tests::the_decode_staging_holds_a_full_batch_plus_a_solo_transition` (the sizing correction) | size `command_decoded` `2 * MAXIMUM_COMMAND_RECORDS` again, without the `2 * track_count` term | 255 `channel = both` effect-parameter records (510 spans) plus one solo record on a four-track console need 513 entries; the batch is refused `malformed` by the staging bound |

`host_core::solo`'s own unit tests carry the state machine's algebra — the complement
composition, the per-lane restore, the two-record delta shape, `solo_count`'s incremental
maintenance, and the shadow/rollback — independently of any host.

## Issue #210 phase 3 — command kinds 10 (`trimDb`) and 11 (`polarityInvert`)

Driver: one mutation at a time on the committed tree, `cargo test -p host-web`, tree
restored between rows.

| # | mutation | test | result |
|---|---|---|---|
| P3-M31 | the decode whitelist drops `COMMAND_TRIM_DB` | `trim_and_polarity_are_admitted_on_every_lane_selector` (+5) | RED — every arm refuses `malformed` |
| P3-M32 | the admission dispatch drops the two kinds, so they fall to the `_ =>` arm | same (+5) | RED. This is the drift the kind-vocabulary gate **cannot** see: the constant still exists, the decode still admits it, and only the dispatch forgot it |
| P3-M33 | an input record is routed to the fader band | (5 tests) | RED |
| P3-M34 | the effect band is not moved past the new per-track band | (7 tests) | RED |
| P3-M35 | `queue_count` is not widened for the third band | (13 tests) | RED |
| P3-M36 | `queue_capacity` reports the fader depth for an input slot | — | **EQUIVALENT, and argued in the test**: live controls lease all three of a track's queues at one depth (`TrackControlRequest::queue_capacity` is a single field), so the wrong queue's capacity is the right number. It becomes observable the day the three depths can differ, and the line is written per band so that day is a one-line change |
| P3-M37 | the `trim_db` domain check is dropped | `trim_and_polarity_refuse_on_the_declared_terms` | RED |
| P3-M38 | the `polarity_invert` boolean-exact check is dropped | same | RED |
| P3-M39 | a trim record accepts a rack byte | same | RED |
| P3-M40 | `channel = 255` is admitted as `Both` | same (+1) | RED |
| P3-M41 | `channel = both` lowers to a single-lane record | `a_both_lane_trim_command_is_one_record_and_one_queue_slot` | RED — the `both` arm renders the `left` arm's bits. The queue-slot half of that test does **not** catch this, and the two halves are asserted together for exactly that reason |
| P3-M42 | a refused submission commits the solo transaction instead of rolling it back | `trim_and_polarity_leave_the_solo_transaction_closed` (+1) | RED — a solo bit survives a batch its *trim* record refused |
| P3-M43 | the admission couples a trim record to the solo composition | `a_trim_is_not_a_mute_and_solo_does_not_move_it` | RED — a trim ride sets the strip's user mute, and clearing a solo no longer restores what the caller set |

The three lane-index defects the banked drain can have -- a missed member queue, an off-by-one
lane, a constant lane -- are **not** reachable from this file: the web host's fixtures are one and
four tracks and the mix cannot tell identical tracks apart. They are gated end to end, per track,
through the post-matrix meters, in `crates/host-core/tests/input_liveness_live_controls.rs`.

## Issue #240 — atomic document-owned boot

Every product mutation below was applied to the working tree on 2026-08-28, the named gate was
run, RED was observed, and the mutation was reverted before the next row. The browser resource
gate also runs its own copied-fixture mutation suite, so those self-tests never alter this tree.

| gate | mutation | observed red |
|---|---|---|
| `quoted_root_shape_keys_self_configure_without_a_second_parser` | report `compiled.quantum() + 1` from the shared document-shape helper | the raw quoted-key 48k/128 and 96k/127 boot fixture refuses `host.source.ring_frames` instead of self-configuring |
| `test-web-audioworklet.mjs` one-module-lifetime assertion | fetch and compile the selected wasm module a second time before `addModule` | the exact event sequence is `compile, compile` instead of `compile, addModule` |
| `boot_transient_budget::pinned_multiplier_bounds_the_worst_accepted_parse_and_model_build_peak` | disable the pre-parse projection refusal | the one-byte-under-budget leg reaches parsing and fails `one byte below the pre-parse projection refuses` |
| same native peak fixture | stale the pinned multiplier from `80` to `1` | measured peak `34,875,248` exceeds `1 × 1,048,576` |
| `check-web-boot-budget.mjs` (run by `check-web-audioworklet.sh`) | disable the pre-parse projection refusal in the wasm artifact | the refused leg returns live handle `1` where zero is required, before it can report typed `refusedBudget`; the unmutated accepted leg independently pins the wasm high-water mark |
| `dense_refusal_diagnostics_are_count_bounded` | remove the encoder's 64-item `take` | the exact line-count pin sees `16,384` lines instead of `64` (the release-only `dense_refusal_diagnostics_finish_under_one_second_in_release` separately measures the wall bound) |
| `maximum_document_dense_invalid_fixture_reaches_bounded_semantic_validation` / `maximum_document_dense_invalid_boot_is_typed_and_bounded` | restore the schema-invalid empty-object footer, or bypass the semantic validator's 64-diagnostic accumulation guard | the phase oracle rejects any parser diagnostic and requires exactly 64 ordered `automation.invalid_range` paths through segment 63; the production test independently pins the same bounded semantic refusal at the exact 1,048,576-byte document ceiling |
| `raw_ffi_validates_handle_layout_overflow_and_transactional_failure` (F2/F4) | return address `1` instead of zero for an invalid handle's status answer | whole-structure emptiness fails: `left: 1, right: 0` |
| same raw lifecycle fixture | skip the live-host check in `boot` | boot-while-live reports `0` instead of typed lifecycle result `3` |
| `each_boot_option_rule_has_its_own_typed_refusal` | skip the nonzero-`reserved0` check | the invalid option boots and the fixture fails `invalid option must refuse`; the same table independently pins struct size, ABI version, and ring divisibility diagnostics |
| `session_validation_owns_the_launch_rate_set` (F3) | make `is_launch_sample_rate` accept every rate | `44,099` compiles where the exact launch-tier pin requires `sample_rate.unsupported_at_launch` |
| `ring_zero_derives_from_the_document_and_matches_the_explicit_value` (capi) | bypass the zero-ring derivation before shared preparation | zero refuses `resource.limit_exceeded` instead of matching the explicit derived ring |
| `exact_retained_total_is_checked_as_one_budget_not_independent_caps` | omit `graph_session_plus_plan_bytes` from the independent retained aggregate | one byte below the true aggregate boots; the fixture fails `one byte below exact aggregate must refuse` |
| `representative_retained_projection_tracks_the_post_prepare_exact_aggregate` | omit the compiled-model row from the retained projector | the 64-track representative reports gap `2,889,216` above the documented `1,396,479` bound, so an underbound projector cannot silently stale |
| `retained_projection_budget_diagnostic_names_projected_bytes` | spell the early diagnostic's `projected_bytes` field as `exact_bytes` | the byte-for-byte diagnostic assertion rejects the mislabeled measured value |
| `exact_retained_total_is_checked_as_one_budget_not_independent_caps` | spell the post-prepare veto's `exact_bytes` field as `projected_bytes` | the byte-for-byte diagnostic assertion rejects the mislabeled measured value |
| `check-web-audioworklet.sh` frozen export set | delete `miso_engine_web_v1_boot_result` from the expected list | the artifact reports it as an unexpected wasm export and the exact diff is printed |
| `check-session-map-shape.py --self-test` via `check-web-audioworklet.sh` | add an unused `handle: u32` parameter to `miso_engine_web_v1_boot_options_ptr`; the same derived probe covers all five S2 boot signatures | RED before any JS/runtime assertion: `miso_engine_web_v1_boot_options_ptr has ABI signature ('handle: u32',) -> u32; expected () -> u32 (the boot family takes no handle)` |
| `check-browser-expected-resources.py --self-test` plus direct/browser oracle | copied-fixture mutations: each budgeted row one byte over its ceiling, a zeroed or unclassified row, each exact row class, each of the three frozen PCM digests, the render transcript and the boot vocabulary (#1060) | all 32 mutations are refused from a document printed at every ceiling; identity, command, and observation PCM digest movement is never admitted as a re-pin |

## Issue #272 — the qualification session identities

The three `qualification/*.json` documents declared `content` values minted from the old #241
locator names, not from canonical PCM, and nothing read them. `qualification/session-identities.mjs`
now re-derives each identity from the harness's own exported generator and `run.mjs::main` calls it
before a browser launches. Every row below was applied to the working tree, then `node
./session-identities.mjs` (or `node ./run.mjs`) was run from the qualification directory, the
failure was observed, and the mutation was reverted in the same session.

| Target | Mutation | Observed failure |
|---|---|---|
| `session-identities.mjs` live-control row | flip one hex digit of `live-control-session.json`'s declared `content` | `session-identity: live-control-session.json: declared source row is not the fed PCM's canonical identity` |
| `session-identities.mjs` stall row | flip one hex digit of `stall-session.json`'s declared `content` | same refusal, naming `stall-session.json` |
| `session-identities.mjs` observation row | flip one hex digit of `observation-session.json`'s declared `content` | same refusal, naming `observation-session.json` |
| the #272 defect itself | restore the pre-#272 name-minted `sha256("web-browser-console")` on `live-control-session.json` | refused; the check states the derived identity the document must carry |
| cross-document reuse | declare the stall document's identity on the live-control document | refused; one digest cannot stand for two different fed regions |
| shape drift | `"frames": "5120"` -> `"5121"` on `stall-session.json` | refused; shape and identity are one pinned row, because the preimage length is `frames * channels * 4` |
| generator drift | `OBSERVATION_LEVEL` `0.5` -> `0.25` in `qualification.js` | the derived identity moves to `680aca77…` and the unchanged document is refused — a pinned hex string would have stayed green |
| generator drift | flip the sign of `sourcePlanes`'s right plane | the live-control identity moves to `7499a91c…` and the unchanged document is refused |
| stale row beside a truthful one | add a second `"content": "sha256:…"` source row to `stall-session.json` | `expected exactly one source content identity, found 2` |
| the check's own comparison | the flipped-digit self-proof inside `checkSessionIdentities` | asserts a one-digit-off identity never matches, so the comparison cannot be loosened into a vacuous pass |

## Issues #280 and #281 — the qualification harness's artifact pin and its boot options

Two defects that together kept `npm run qualify` — the step `.github/workflows/qualification.yml`'s
`browser` job runs (the old `browser-qualification.yml` ran it before design #359 §12 stage 3
retired that workflow) — from reaching a browser at all on `main`.
Derivations, the document audit that cleared the #241-fallout hypothesis, and the
digest-immobility argument are in `docs/derivations/281-qualification-harness-boot.md`.

### #280 — the served artifact set, five names to six

`server.mjs::exactArtifacts` still required #139's five-file set; `build-web-audioworklet.sh` has
emitted six since #243. Every row below was applied to the working tree, then
`node ./run.mjs --artifacts <build> --browser chromium --self-test-mutations` was run from the
qualification directory (the artifact proofs run before the server starts, so each failure lands in
seconds), the failure was observed, and the mutation was reverted in the same session.
`run.mjs::artifactSetProofs` mutates *copies* of the real built directory under a temporary root,
so the built artifacts are never touched.

| Target | Mutation | Observed failure |
|---|---|---|
| the #280 defect itself | restore the pre-#280 five-name `ARTIFACT_NAMES` | `artifact-set: the built directory is not the exact shipped set` — the shipped six-file build is refused, which is the workflow-blocking behaviour |
| `exactArtifacts` count clause | delete `names.length !== ARTIFACT_NAMES.size` (a subset would pass) | `Missing expected rejection: artifact-set: miso-engine-v1-abi-layout.json removed: red mutation escaped the artifact pin` |
| `exactArtifacts` name clause | delete `names.some((name) => !ARTIFACT_NAMES.has(name))` (a substitution keeping the count at six would pass) | `Missing expected rejection: artifact-set: miso-engine-v1-abi-layout.json replaced by a stray of the same count: red mutation escaped the artifact pin` |
| `exactArtifacts` regular-file clause | delete the `stat(...).isFile()` loop | `Missing expected rejection: artifact-set: directory named like an artifact: red mutation escaped the artifact pin` |
| the whole set check | delete the `throw` and its condition outright | `Missing expected rejection: artifact-set: miso-engine-v1-abi-layout.json removed: …` |

The proof set covers all six names in both directions: each one removed (which no minimum-style
pin survives) and each one replaced by a stray of the same count (which no count-only pin
survives), plus one stray added and one directory wearing an artifact's name. That is what keeps
"widen the pin" from becoming "loosen the pin".

### #281 — the pre-#240 caller shape

`qualification.js` still called `createMisoAudioWorkletHost` with
`{ quantumFrames, sessionToml, limits }`; #240 replaced that with `{ document, options }` and cut
`limits`'s 21 capacity ceilings down to six boot words. Both guards are `hasExactFields`, so the
harness was refused with `miso.error.v1` requestId 0 result 1 before the module was fetched. Each
row was applied to `qualification.js`, `node ./run.mjs --artifacts <build> --browser chromium` was
run, the failure was observed, and the mutation was reverted in the same session.

| Target | Mutation | Observed failure |
|---|---|---|
| the #281 defect itself | restore `quantumFrames`/`sessionToml`/`limits` on the corpus row | `chromium: browser-execution: corpus qualification failed: {"error":{"tag":"miso.error.v1","requestId":0,"result":1}, …}` — the exact transcript #281 reported. The `diagnostic` leg now answers `miso.ready.v1` result 0 with a full resource report, so the refusal is localized to the caller rather than echoing itself |
| `bootOptions` completeness | delete `maximumMemoryBytes: 0n` | same typed refusal; the six boot words are not optional |
| `bootOptions` exactness | leave one #240-deleted ceiling (`sessionDocumentBytes: 1 << 20`) in the returned object | same typed refusal; a superset is as invalid as a subset |

### The shipped AudioWorklet artifact digest pin (this change)

`scripts/build-web-audioworklet.sh` did not remap `CARGO_HOME` or the repo root into its
`RUSTFLAGS`, the same defect `scripts/build-flac-decoder.sh` had before #300: rustc bakes
dependency source paths into panic locations, those sources live under `CARGO_HOME`, and
`CARGO_HOME` differs between a developer machine and a CI runner. Reproduced by changing only
`CARGO_HOME` and rebuilding the pre-fix script:

  CARGO_HOME=/root/.cargo         -> 678a9e38d0cbeac982d5852f1a046e22037d58b6d80a97f0a1e3383d496884e7
  CARGO_HOME=/home/runner/.cargo  -> 126b5d61b85ceef69f2f9d9653ef37240b5c511488e98e9c34514badf292baa4

The artifact had no pin of its own, so nothing failed in this repo; the app's
`miso-engine-v1.provenance.json` pins the digest one repo over, and would have moved underneath it
silently.

Fixed the same way as #300: `--remap-path-prefix` for both roots, plus a `decoder-artifact.sha256`-
style pin (`hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`) with a
`MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1` escape hatch. Verified reproducible across three independent
combinations of repo path and `CARGO_HOME` (`/root/.cargo`, `/home/runner/.cargo`,
`/opt/alt/.cargo` from a second worktree), all producing
`e3a8ba3130dc33823146a90804ef0245a66602586dca7cf9295f33d384d9686d`, which is the new pin.

The pin gate itself was proven to go red: with the pin file hand-edited to
`deadbeef…deadbeef`, `scripts/build-web-audioworklet.sh` exited 1 with `AudioWorklet artifact pin
mismatch: expected=deadbeef… observed=14328e23…` and wrote nothing to the output directory; restoring
the correct pin made it pass again.

| Target | Mutation | Observed failure |
|---|---|---|
| the digest pin | hand-edit `miso-engine-v1-audio-worklet-artifact.sha256` to `deadbeef…` | `AudioWorklet artifact pin mismatch: expected=deadbeef… observed=e3a8ba31…`; exit 1, output directory left empty |

## Issue #1207 — every strip in the live-control handles; bus effects filed in the browser

Each mutation was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted.

| gate | mutation | observed red |
|---|---|---|
| `tests::a_bus_session_boots_live_controlled_and_files_every_bus_effect` (segment-aware lookup, D2) | make `strip_index` one binary search over the concatenation `tracks ++ submixes` | `aaa-bus` sorts before `t0`, so its producer is never found and the boot refuses with `web.live_controls.effects` (both #1207 tests panic at boot) |
| `tests::a_bus_session_boots_live_controlled_and_files_every_bus_effect` (per-strip effect tables, D3/D5) | put the K1 interim back: skip every prepared entry whose owner is not a track in `attach_effect_live_controls` and `attach_effect_observation` | the boot succeeds but `aaa-bus`'s console slot 0 has no filed producer at `dense_effect_slot(effect_base[2], rack_effects[2], ..)` |
| `tests::a_bus_session_admits_and_renders_without_allocating` (gate 4) | allocate a copy of `effect_base` inside `ReadyOwnership::effect_slot` | `admission/render allocated`: the measured submission plus `render_next` counts one allocation |

## Issue #1209 — submix strips in the browser meter frame

Each mutation was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted.

| gate | mutation | observed red |
|---|---|---|
| `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (frame section, D1) | write the master pair at `2T` (after the tracks) instead of `2(T + S)` | `aaa-bus 0.773352 is t0 + t1` fails: the bus's slot holds the master |
| `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (frame size, D1) | size the frame `3T + 3` | `3(T + S) + 3 words`: 12 where 18 was required |
| `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (meters, D3) | request meters for the tracks only | `every bus is metered`: the bus slots stay zero |
| `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (gain base, D5) | `gain_base = 2T + 2` | `every bus is metered`: the bus peak words are zeroed as gain-reduction words |
| `tests::the_meter_frame_carries_a_peak_pair_and_a_gain_word_per_submix` (header field, D2) | leave `submix_count` at 0 | `submix_count == 2` fails |
| `tests::bus_meters_render_and_poll_without_allocating` (gate 5) | allocate a `submixes.len()`-byte buffer per published window in `poll_meters` | `render/poll allocated` |
| `tests::bus_meters_render_and_poll_without_allocating` (gate 5) | box each folded gain-reduction value in the poll | `render/poll allocated` |
| `test-web-audioworklet.mjs` main-realm frame validation (gate 3) | drop the `submixPeaks.length === 2 * submixCount` rule | the `submixPeaks` length-1/length-4 frames are accepted: `expected rejection` |
| `test-web-audioworklet.mjs` main-realm frame validation (gate 3) | keep the 12-field exact-field list | the one-bus frame fails the host and is never delivered |
| `test-web-audioworklet.mjs` main-realm frame validation (gate 3) | drop the finite non-negative rule for `submixGrDb` | the `-1` and `NaN` frames are accepted: `expected rejection` |
| `test-web-audioworklet.mjs` worklet meter frame (D1/D4) | build the master peak view at `2T` | `peaks` carries the bus value `0.75` where the master's `0.625` was required |
| `test-web-audioworklet.mjs` worklet meter frame (D4) | do not copy the bus peak view into `submixPeaks` | `submixPeaks` reads `[0, 0]` |
| `test-web-audioworklet.mjs` worklet meter frame (D4) | read the master gain word at `3T + 2` | `masterGrDb` reads a bus word, not `7.5` |
| `capability-evals.mjs` bus frame (gate 2, `check-sdk-headless.sh`) | size the headless reader's frame `3T + 3` | `sdk.meter.frame` refusal: the frame with buses is rejected |
| `capability-evals.mjs` bus frame (gate 2, `check-sdk-headless.sh`) | return an empty `submixPeaks` | `2S bus peak words` fails |
| `tests::submix_ids_enumerate_in_canonical_order_through_staging_sized_for_them` (#1210 D2) | drop `.max(shape.longest_submix_id_bytes)` from the ID-staging projection | `id_staging_bytes` is 14, not the 63-byte submix ID (the copy past it would trap) |
| `tests::submix_ids_enumerate_in_canonical_order_through_staging_sized_for_them` (#1210 D1) | answer `_submix_id` from `ready.submixes` reversed | submix 0 is the 63-byte `zz-` ID, not `a-bus` |
| `test-web-audioworklet.mjs` main-realm session map (#1210 gate 3) | drop `"submixes"` from the host's `sessionMap` expected fields | the well-formed map with `submixes` fails the host with 255 |
| `test-web-audioworklet.mjs` main-realm session map (#1210 gate 3) | drop the nonempty-string rule for `submixes` entries | `the host accepted a session map with a non-string submix` |
| `test-web-audioworklet.mjs` worklet session map (#1210 D3) | stop posting `submixes` in the `miso.sessionmap.v1` reply | `the enumerated submix order` assertion fails |
| `test-web-audioworklet.mjs` worklet construction (#1210 D3) | drop the header `submix_count` == enumerated count check | `a submix count the meter header disagrees with must fail initialization` |
| `check-session-map-shape.py --self-test` (#1210) | the host list or the worklet reply without `submixes` | both new self-test mutations are caught (17 in all) |
| `tests::live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track` (#1213 gate 1, the moved bound) | compare the generic guard against `ready.tracks.len()` | the bus pan (index `T`) is refused `unknownTrack`: `pan (smoothing 0) at strip 3`, result 1 |
| `tests::live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track` (#1213 gate 1, a band spelling) | `push` keeps `let tracks = self.tracks.len()` | the bus fader record misses its queue: result 255 |
| `tests::live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track` (#1213 gate 1, `queue_count`) | size the queues `3T + effects` | the bus compressor's threshold record finds no queue: result 7 |
| `tests::live_strip_edits_on_a_bus_equal_the_same_edits_on_a_track` (#1213 gate 1, per-strip shadows) | build `input_filter_shadows` from the tracks only | the bus's input-filter configuration copy is unsupported (7) |
| `tests::a_bus_mute_command_equals_the_bus_booted_muted`, `tests::a_bus_can_be_unmuted_while_a_track_is_soloed` (#1213, per-strip solo state) | seed the solo state per track only | kind 4 at the bus index is refused `unknownTrack` (no mute owner) |
| `tests::soloing_a_track_keeps_its_bus_and_return_audible` (#1213 gate 2) | seed the submixes `solo_safe: false` | soloing `a` solo-mutes `drums` and `verb`: block 1 differs from explicit mutes |
| `tests::a_solo_at_a_bus_index_refuses_not_soloable_and_stages_nothing` (#1213 gate 2, reason 12) | delete the `solo_safe` check before `set_solo` | the refusal reports reason 2 (`unknownTrack`), not 12 |
| `tests::a_bus_can_be_unmuted_while_a_track_is_soloed` (#1213 gate 3, the deleted inline composition) | restore `muted \|\| (any_solo && !solo(track))` in kind 4 | the unmute stages `muted: true` and `drums` stays silent: block 4 differs |
| `tests::a_bus_record_ahead_of_a_bad_track_record_is_never_pushed` (#1213 gate 4) | push a bus fader record during pass one | strip 3's fader queue holds a record after the refusal |
| `tests::overfilling_a_bus_queue_is_typed_backpressure_with_no_push` (#1213 gate 4) | skip the room check for bus fader slots | the push fails half-way: result 255 instead of backpressure |
| `tests::a_bus_compressor_reports_its_gain_reduction_in_the_bus_word`, `tests::a_bus_limiter_can_be_the_designated_master` (#1213 gates 5, 6) | restore #1207's GR-fold skip of strip indices `>= T` | the bus word stays `0` and `master_gr_present` is `0` |
| `tests::a_bus_compressor_reports_its_gain_reduction_in_the_bus_word` (#1213, K2 verdict (a)) | `observation_selection_for_address` looks up `ready.tracks` only | the selected read at the bus's strip index is `InvalidSelection` |
| `capability-evals.mjs` bus selected read (#1213, K2 verdict (a), `check-sdk-headless.sh`) | the same tracks-only lookup, artifact rebuilt | `readObservations` fails with `invalidArgument` |
| `capability-evals.mjs` bus refusal classifier (#1213 D5a, `check-sdk-headless.sh`) | the SDK classifier compares against the track count | the bus's missing insert reports `unknownTrack` |
| `tests::bus_edits_and_a_bus_observation_admit_and_render_without_allocating` (#1213 gate 7) | allocate a `submixes.len()`-capacity `Vec` in admission | `admission/render allocated` (1) |
| `capability-evals.mjs` bus solo refusal (#1213 D4, #1212 NIT-2, `check-sdk-headless.sh`) | delete the `solo_safe` check before `set_solo`, artifact rebuilt | the shipped admission reports reason 2, not 12 |
| `tests::a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track` (#1213 gate 1, prepared EQ; attempt 2 MINOR-1) | spell `prepared_queue_address`'s Eq base `ready.tracks.len() * 3` | the bus EQ edit misses its owner: `submit_prepared_commands` is not `RESULT_OK` |
| `tests::a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track` (#1213 gate 1, prepared EQ; attempt 2 MINOR-1) | spell the admission's EQ owner marker `queue_slot: ready.tracks.len() * 3 + effect` | the same: the prepared submission at the bus is refused |
| `tests::a_prepared_eq_edit_on_a_bus_equals_the_same_edit_on_a_track` (#1213 gate 1, prepared EQ; attempt 2 verdict MINOR-A) | stage the bus's band-1 gain at -6 dB while the reference track's stays -12 dB | the render comparison fails at block 2, sample 1 (band 1 is enabled, so the edit is audible) |
| `tests::a_single_lane_bus_mute_equals_the_bus_booted_with_that_lane_muted` (#1213 gate 2, kind 4 lane; attempt 2 MINOR-2) | kind 4 reads `let lane = 0_usize` | the right-lane mute stages `muted: false`: the bus's right lane stays audible against the booted twin |
| `test-web-audioworklet.mjs` worklet eq-config classifier (#1213 D5a; attempt 2 MINOR-3) | compare `message.trackIndex >= this.trackCount` | the bus's missing insert reports `2` (`unknownTrack`), not `4` |
| `test-web-audioworklet.mjs` bus `observe()` binding (#1213, attempt 2 MAJOR-1) | the host reports `frameSlot: 0` or `Math.min(trackIndex, 1)` for a bus; or the mapping is read as a plain `trackGrDb` index | `frameSlot` is not `T`; the old-contract read is `undefined`, not `submixGrDb[0]` |

## Issue #1222 — browser live send commands

Each row was applied, its test run red, and the tree restored.

| gate | mutation | observed red |
|---|---|---|
| `tests::every_kind_is_bounded_by_the_count_it_addresses` (gate 3, the moved bounds check) | restore the generic `track >= strip_count` check ahead of kind dispatch for every kind | send 7 reports reason 2 (`unknownTrack`), not 13 |
| `tests::every_kind_is_bounded_by_the_count_it_addresses` (gate 3, reason 13) | refuse an out-of-range send with `COMMAND_REASON_UNKNOWN_TRACK` | the reason is 2, not 13 |
| `tests::every_kind_is_bounded_by_the_count_it_addresses` (gate 3, fewer sends than strips) | bound the send arm by the strip count and index `route_controls[track]` | send 6 of 7 (past 6 strips) is refused (result 1); on 3 sends and 4 strips, index 3 panics out of bounds |
| `tests::a_live_send_edit_lands_on_a_fresh_plans_bits` (gate 1, `routeMatrix`) | build the record from `[ll, rl, lr, rr]` | trial 0: the live output's bits differ from the fresh plan's |
| `tests::a_live_send_edit_lands_on_a_fresh_plans_bits` (gate 1, `routeMatrix` decode) | decode `values` as `[v0, v2, v1, v3]` | the mirror differs from the edited values |
| `tests::a_live_send_edit_lands_on_a_fresh_plans_bits` (gate 1, the send band) | push a send's record onto the next send's queue | trial 0: the output's bits differ |
| `tests::a_live_send_edit_lands_on_a_fresh_plans_bits` (gate 1, a stale mirror field) | `routeGainDb` stages its record but leaves the mirror's gain | the mirror keeps the seed gain |
| `tests::a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (gate 2, the mirror committed before the room check) | `ready.routes.commit()` as each send record is staged | the domain refusal leaves the mirror at -9 dB |
| `tests::a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (gate 2, the send band's room) | `queue_available` reports `u32::MAX` for every send queue | the overfilled send queue fails half-way: result 255, not backpressure |
| `tests::a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (gate 2, push before validation) | push each send record in pass one | the domain refusal leaves send 0's queue at 3 of 4 |
| `tests::send_records_are_shape_checked` (D1) | drop the `effect_index != 0` rule | the record with an effect word is admitted (reason 0) |
| `tests::send_edits_admit_and_render_without_allocating` (gate 7) | allocate an 8-byte `Vec` per staged send record | `admission/render allocated` (3) |
| `tests::the_exact_retained_budget_charges_the_send_lanes` (#1221 verdict MINOR-1) | leave `route_control_resources.total_bytes` out of the bridge rows | the retained delta between depths 8 and 64 is 0, not the lanes' |
| K3 follow-up (verdict MINOR-1/2, V1): `tests::a_following_sends_seeded_source_lanes_land_on_a_fresh_plans_bits` | `compile_ready` seeds the send mirror with `\|_, _\| false` | RED here and in #1224's `a_one_lane_mute_follows_into_its_own_source_column` |
| K3 follow-up (V8) | the record is built with `[false; 2]`, not `next.source_lane_muted` | RED in this test only |
| K3 follow-up (V2) | `route_slot` omits `+ self.effect_controls.len()` | RED here (inserts run), in gate 2 (refused at index 0, not 1) and in #1224's `a_delayed_send_follows_like_an_explicit_send_mute` |
| K3 follow-up (V6): `tests::a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (gate 2, admitted then refused) | `ready.routes.commit()` dropped on success | RED here and in #1224's `a_full_send_queue_refuses_the_strip_mutes_it_follows` |
| K3 follow-up (verdict MINOR-4, V3): `tests::the_exact_retained_budget_charges_the_send_lanes` (absolute check) | the bridge rows charge `route_control_resources.queue_bytes`, not `total_bytes` | RED |
| K3 follow-up (V4) | the mirror and shadow bytes dropped from the bridge rows | RED |
| `check-command-kind-vocabulary.py --self-test` and `test-web-audioworklet.sh` (the "added and not threaded" class) | `pub const COMMAND_SOLO_MODE: u32 = 18;` after `COMMAND_VCA_MUTE = 17` (issue #1245; it was 16 after `COMMAND_ROUTE_MATRIX = 15` under issue #1222) | `.d.ts MisoCommandKind disagrees with the Rust host constants` -- the threading rule, no longer the contiguity rule a shipped value 12 tripped |
| `check-command-kind-vocabulary.py --self-test` (the JS set gains an undecoded kind) | the host JS set ends `…, 14, 16]` | `the host JS COMMAND_KINDS set disagrees with the Rust host constants` |

## Issue #1223 — live route enumeration and the SDK's send edits

Each row was applied, its test run red, and the tree restored. SDK rows ran
`node --test test/live-controls-evals.mjs` (or `tsc`) against the slice's built module; E1 rebuilt
the module with the mutation.

| gate | mutation | observed red |
|---|---|---|
| `tests::live_route_ids_enumerate_in_send_index_order_through_staging_sized_for_them` (gate 4, staging) | drop `longest_route_id_bytes` from the ID staging size | `id_staging_bytes` is 2, not 63; with that assertion removed, the 63-byte copy trips `compiled ID exceeds its projected staging capacity` |
| the same test (D1, the enumeration) | the route-ID export reads the model's routes, not the live send producers | live route 0 is 7 bytes (`bx-main`), not 6 (`send-a`) |
| `test-web-audioworklet.mjs` (the processor's session map) | the worklet posts `[...this.routeIds].sort()` | `issue #1223: the enumerated live-route order` |
| `test-web-audioworklet.mjs` (the host's acknowledgement validator) | drop the `routes.every(...)` check | `the host accepted a session map with a non-string route` |
| `test-web-audioworklet.mjs` (worklet construction) | read a route ID without the length/capacity check | `an empty route ID must fail initialization` |
| `live-controls-evals.mjs` gate 1 (and gate 3) | `route(id)` indexes the send among every route of the session, sorted | `a send edit encodes its kind at the engine's live-route index` and `a live send gain equals the session booted at that gain` |
| `live-controls-evals.mjs` gate 1 | `RouteEdits.matrix` writes `ll, rl, lr, rr` | the browser request's values differ |
| `live-controls-evals.mjs` gate 1 | the browser map prepends the track IDs to `routes` | `kick-verb` encodes index 4, not 1 |
| `live-controls-evals.mjs` gate 1 (and gates 2, 3) | `OfflineEngine.sessionMap()` reverses the engine's route list | all three #1223 evals |
| `live-controls-evals.mjs` gate 2 | an unknown route ID falls back to index 0 | `an unknown or output route ID refuses before any record is built` |
| `live-controls-evals.mjs` gate 2 | drop the output-route branch | the same eval: the message lists the live routes instead |
| `live-controls-evals.mjs` gate 3 only (E1, rebuilt module) | admission pushes a send record onto route `index ^ 1`'s queue | `a live send gain equals the session booted at that gain`; gate 1 stays green (admission is `ok`) |
| `live-controls-evals.mjs` (the vocabulary test) | drop `routeMatrix` from `kindNames` | `the semantic methods cover the generated command vocabulary exactly` |
| `live-controls-types.ts` | `RouteEdits.gainDb` takes `LaneOptions` | `Unused '@ts-expect-error'` (a send has no lane) |
| `live-controls-types.ts` | `RouteEdits` gains `faderDb` | `_RouteSurface` fails, and the `faderDb` `@ts-expect-error` is unused |

## Issue #1224 — sends follow their source strip's mute

Each row was applied, its test run red, and the tree restored. "Audio" rows ran with the tests'
`follow_lanes` mirror assertions removed, so the red is the rendered output's, not the mirror's.

| gate | mutation | observed red |
|---|---|---|
| deliverable 3 (a): `tests::a_follow_never_pushes_a_redundant_send_record` (gate 2) | `delta` yields every following route, changed or not | `solo drums` is refused: `delta` yields the unchanged `drums-verb`, whose strip staged no mute record (`malformed`); gates 3, 5, 9 and the staging test go red too. With that guard also bypassed (a missing strip record read as ramp 0), the red is `only bass-room follows`: a redundant `drums-verb` record was pushed |
| deliverable 3 (b): `tests::soloing_a_track_silences_the_followed_pre_fader_sends_of_the_rest` (gate 1) | the follow pass placed before the solo coalescing pass | `solo vocal` is refused (`malformed`): no strip mute record is staged yet for the follow's ramp; gates 2, 3, 4, 9 and the staging test go red too. The red is the `malformed` guard's: gate 1 checks settled blocks only; with the guard replaced by a ramp-0 fallback, the audio red is `a_solo_follow_ramps_at_the_solo_window` (K3 follow-up row below) |
| deliverable 3 (c): `tests::a_full_send_queue_refuses_the_strip_mutes_it_follows` (gate 3) | push each follow record as it is built, before the room check | `solo: send queues`: `bass-room`'s queue lost a slot on a refused batch |
| `tests::soloing_a_track_silences_the_followed_pre_fader_sends_of_the_rest` (gate 1, audio) | no follow pass (`if false && …`) | `soloed vs booted muted: block 6 sample 0: 0.031146944 vs 0.6195009` -- the measured leak |
| gates 4, 5 and 6 (audio) | no follow pass | gate 4 `[false, false] -> [true, false]` block 4; gate 5 `bus lanes [true, true]` block 4; gate 6 `muted` block 5 sample 102 (frame 742 = the edit at 256 + 486) |
| gate 1 and gate 6 (audio) | build the follow record with the mirror's old `source_lane_muted` | gate 1 at block 6 sample 0; gate 6 `muted` at block 5 sample 102 |
| `tests::a_one_lane_mute_follows_into_its_own_source_column` (gate 4, audio) | `followed_lanes` returns `[lane 1, lane 0]` | `[false, false] -> [true, false]`: block 4 sample 0; gate 5 `bus lanes [false, true]` too |
| gate 4 (audio) | `followed_lanes` reads lane 0 for both | `[false, false] -> [true, false]`: block 4 sample 0; gate 5 too |
| `tests::muting_a_bus_silences_its_followed_send` (gate 5, audio) | the follow pass's effective mute is `strip < tracks.len() && …` | `bus lanes [true, true]`: block 4 sample 0 |
| `tests::a_delayed_send_follows_like_an_explicit_send_mute` (gate 6, audio) | the ramp is the *first* strip mute record staged for the source | `un-muted`: block 13 sample 102 (200-sample ramp against 480) |
| gate 6 (audio) | the follow record's ramp length is 0 | `muted`: block 5 sample 102 |
| gate 3 | `ready.routes.commit()` right after the follow pass | `solo: follow lanes`: the refused batch kept the followed lanes |
| gate 3 (the overlong window) | clamp the follow ramp to `2^22` | `overlong`: the strip mute is admitted (`0`), not `invalidArgument` |
| gate 2 | skip `LiveRouteState::follow` after staging | `solo vocal too` is refused: the stale mirror re-yields `bass-room` in a batch that staged no `bass` mute record (`malformed`); with that guard bypassed, `no send record`: the redundant record is pushed |
| `tests::follow_records_admit_and_render_without_allocating` (gate 9) | allocate an 8-byte `Vec` per follow record | `admission/render allocated` |
| `tests::the_decode_staging_holds_a_full_batch_and_its_follow_records` (D4) | `command_staging_count` adds `route_count * 0` | the length pin (536 + 0, not 572); with the pin removed, the batch is refused `reason 1` (`malformed`, the staging bound) |
| K3 follow-up (verdict MINOR-3, probe P2): `tests::each_follow_takes_its_own_source_strips_window` | the ramp lookup ignores the source strip (any strip's last mute record) | RED in this test only |
| K3 follow-up (verdict MINOR-3, probe P1): `tests::a_follow_record_carries_the_mirrors_live_values` | the follow record drops the send's own `mute` (`entry.mute` -> `false`) | RED in this test only |
| K3 follow-up (verdict MINOR-3, probe P1) | the follow record uses 0 dB, not the mirror's `gain_db` | RED in this test only |
| K3 follow-up (verdict MINOR-3, probe P4): `tests::a_solo_follow_ramps_at_the_solo_window` | the ramp lookup sees only records staged before the coalescing pass, with a ramp-0 fallback for the guard (the pass-ordering defect with its guard bypassed) | RED in this test only |
| K3 follow-up (verdict MINOR-1, probe P3): `tests::a_no_op_record_on_the_other_lane_never_sets_the_follow_ramp` | the ramp lookup matches any strip-mute record on the source strip, changed lane or not (attempt 1's rule) | RED in this test only; it was red at `eff44271d` |
| K3 follow-ups verdict MINOR-1 (probe V4): `tests::a_both_lane_record_with_one_lane_changed_sets_the_follow_ramp` | the changed-lane rule's `Both` arm is `changed[0] && changed[1]` (for `\|\|`) | RED in this test only (the batch is refused, `malformed`); every other lib test stays green |
| #1242 attempt 2 (verdict MAJOR-1, probe P1): `tests::a_both_lane_unmute_keeps_a_one_lane_vca_mute` | the `COMMAND_MUTE` arm stages one record from the first covered lane's effective mute (attempt 1) | RED: the `emitted` mirror; with that assertion removed, the render differs at block 1 sample 128 |
| #1242 attempt 2 (verdict MAJOR-1, probe P2): `tests::a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute` | the same attempt-1 `COMMAND_MUTE` arm | RED |
| #1242 attempt 2 (verdict MINOR-1, M2): `tests::a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute` | host-web seeds submixes with `vca_mute: [false; 2]` | RED (seed 5); the other three #1242 host-web tests stay green |
| #1242 attempt 2 verdict MINOR-1 (M3), VCA follow-ups: P2 draws each record's smoothing from {0, 64} | the split `Both` kind 4 records are staged at `smoothing_samples: 0` | RED in `a_browser_vca_renders_as_its_effective_faders_under_solo_and_mute` only |
| #1242 attempt 2 verdict MINOR-1 (M6), VCA follow-ups: P1 counts `drums`' and `bass`' queue records | every `Both` kind 4 splits (the `left != right` guard dropped) | RED in `a_both_lane_unmute_keeps_a_one_lane_vca_mute` only (two records on `bass`, not one) |

## Issue #1245 — browser live VCA groups

Each row was applied alone, the whole host-web lib suite run, the named tests observed red, and the
tree restored (driver and log: the #1245 attempt 1 record).

| gate | mutation | observed red |
|---|---|---|
| `tests::a_live_vca_ride_lands_on_a_fresh_plans_bits` (gate 1) and `tests::a_member_move_and_a_vca_move_compose` (gate 2) | kind 3 on a reached strip stages its own value, not the effective one | both: the live output's bits differ from the fresh plan's |
| gate 1, gate 2, `tests::a_vca_batch_that_overfills_a_queue_is_refused_whole` (gate 4) | the VCA fader pass stages on the next strip's fader queue | all three |
| `tests::a_vca_move_that_changes_no_effective_value_stages_nothing` (gate 3) | the VCA fader pass stages every reached lane, changed or not | gate 3 only: fader room moves |
| gate 4 and `tests::vca_records_are_addressed_and_shape_checked` (gate 6) | the VCA state commits as a ride is read | both: a refused submission leaves the VCA mirror moved |
| gate 4 | the VCA state is never committed on success | gate 4: the admitted ride is rolled back by the next refusal |
| gates 1, 2, 4 and `tests::a_vca_mute_silences_member_sends_and_survives_solo` (gate 5) | a VCA mute does not run the coalescing pass (`if solo_seen`) | all four |
| gates 1, 2, 4 and 5 | the follow pass ignores a VCA mute (`solo_seen \|\| mute_seen`) | all four |
| gates 1, 2, 4 and 5 | a VCA mute never reaches the strip-mute owner | all four |
| `tests::a_member_mute_after_a_vca_mute_in_one_batch_composes_with_it` (D3, amendment A1) | a later kind 4 does not refresh its strip's VCA mute term first | this test only: the un-mute stages `false`, and the coalesced mute ramps over 480 samples |
| gate 6 and the bound test | the VCA kinds are bounded by the strip count | gate 6 (VCA 5 is `unknownTrack`), and the bound test's VCA 255 is refused |
| gate 6 | an unknown VCA is refused `unknownRoute` | gate 6 only |
| gate 6 | the VCA shape drops the `effect_index` rule | gate 6 only |
| `tests::the_decode_staging_holds_a_full_batch_and_its_vca_records` (gate 7) | `command_staging_count` adds `vca_reached_strips * 0` | gate 7 (the length pin; with it removed, `malformed`) and the retained-budget test |
| `tests::vca_rides_and_mutes_admit_and_render_without_allocating` (gate 8) | allocate an 8-byte `Vec` per VCA fader record | gate 8 only |
| `tests::the_exact_retained_budget_charges_the_vca_state` (D4) | leave the VCA state out of the bridge rows | this test only |
| `tests::the_browser_bounds_vca_reach_and_admits_a_worst_batch_at_the_bound` (amendment A1; renamed in the VCA follow-ups, which moved its timing half to the ignored `..._fits_a_quantum_in_release`) | the reach-pair bound is not enforced | this test only: 16,385 pairs boot |
| the bound test | the VCA count bound is 512, not 256 | this test only: 257 VCAs boot |
| gate 1 and the bound test | the boot-time pair count ignores nesting (a VCA's ancestors are not passed to its VCA members) | both: the boot count disagrees with the live state's reach |
| `check-command-kind-vocabulary.py --self-test` and `test-web-audioworklet.sh` | the "added last" mutations re-anchored past kind 17: the JS set `[1 … 17]` stops at 16 or gains an undecoded 18, the literal `<= 17`, the schema gate's list and the `.d.ts` enum drop `vcaMute` | every one refused (32 red mutations) |
| `check-command-reason-vocabulary.py --self-test` | re-anchored past reason 14: `FUTURE_TAP = 15`, `UNKNOWN_TAP` renumbered to 15, the worklet's `UNSUPPORTED_KIND = 15`, `reason <= 14`, the JS table and the schema gate's list truncated | every one refused (20 red mutations) |
| `live-controls-evals.mjs` (the vocabulary test) | drop `vcaMute` from `kindsAwaitingSdk` | `the semantic methods cover the generated command vocabulary exactly` |
| #1245 verdict MINOR-1 (S1), VCA follow-ups: `tests::a_vca_ride_and_mute_ramp_as_direct_member_moves` | the VCA fader pass stages its member records at `smoothing_samples: 0` | RED in this test and `a_split_member_move_ramps_and_the_last_ride_sets_the_window` |
| #1245 verdict MINOR-1 (S2) | kind 17 does not set `coalesce_smoothing` | RED in `a_vca_ride_and_mute_ramp_as_direct_member_moves` and `a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index` |
| #1245 verdict MINOR-1 (S3): `tests::a_split_member_move_ramps_and_the_last_ride_sets_the_window` | the split kind 3 records on a member are staged at `smoothing_samples: 0` | RED in this test only |
| #1245 verdict MINOR-1 (S23) | the VCA fader pass takes the first ride's ramp, not the last's | RED in `a_split_member_move_ramps_and_the_last_ride_sets_the_window` only |
| #1245 verdict MINOR-2 (S12): `tests::a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index` | kind 17 does not set `coalesce_first_wire_index` | RED in this test only (the refusal names index 0) |


## Issue #1246 — VCA enumeration and the SDK's VCA edits

Each row was applied, its test run red, and the tree restored. SDK rows ran
`node --test test/live-controls-evals.mjs` (or `check-sdk-types.sh`) against the slice's built
module; E1 rebuilt the module with the mutation.

| gate | mutation | observed red |
|---|---|---|
| `tests::live_vca_ids_enumerate_in_vca_index_order_through_staging_sized_for_them` (gate 3, staging) | drop `longest_vca_id_bytes` from the ID staging size | `id_staging_bytes` is not the 73-byte VCA ID's length |
| the same test (D1, the enumeration) | `_vca_id` reads the VCAs in reverse | VCA 0 is 73 bytes, not 2 (`aa`) |
| the same test (D1, the count) | `_vca_count` reads the model's VCA count, not the live VCA state's | the host without live controls answers 3, not 0 |
| the same test (D1, the bound) | `_vca_id` drops its live-state bound | the host without live controls copies VCA 0 |
| `test-web-audioworklet.mjs` (the processor's session map) | the worklet posts `[...this.vcaIds].sort()` | `issue #1246: the enumerated VCA order` |
| `test-web-audioworklet.mjs` (the processor's session map; #1246 verdict m1: the stub enumerates three VCAs beside two routes) | the worklet reads `vcaCount` from `miso_engine_web_v1_live_control_route_count` (the route block it was cloned beside) | `issue #1246: the enumerated VCA order` (two VCAs delivered, not three) |
| `test-web-audioworklet.mjs` (the host's acknowledgement validator) | drop the `vcas.every(...)` check | `the host accepted a session map with a non-string VCA` |
| `test-web-audioworklet.mjs` (the host's acknowledgement validator) | drop the `Array.isArray(message.vcas)` check | the "no VCA list" reply throws inside the validator instead of failing the host |
| `test-web-audioworklet.mjs` (worklet construction) | read a VCA ID without the empty-length check | `an empty VCA ID must fail initialization` |
| `test-web-audioworklet.mjs` (worklet construction) | read a VCA ID without the capacity check | `a VCA ID longer than staging must fail initialization` |
| `live-controls-evals.mjs` gates 1 and 2 | `vca(id)` indexes the engine's list reversed | all three #1246 evals |
| `live-controls-evals.mjs` gate 1 | the browser map's `vcas` is `[]` | `a VCA edit encodes its kind at the engine's VCA index` (the browser transport refuses `drums`) |
| `live-controls-evals.mjs` gates 1 and 2 | `OfflineEngine.sessionMap()` reverses the engine's VCA list | all three #1246 evals |
| `live-controls-evals.mjs` gates 1 and 2 | `VcaEdits.mute` ignores the lane option (always both) | gate 1's channel word and gate 2's left-lane mute render |
| `live-controls-evals.mjs` gate 1 | `VcaEdits.mute` builds kind 4 (`mute`) | gate 1's kinds; gate 2 is blind here (`fx`'s index 2 is `vox`'s strip index) |
| `live-controls-evals.mjs` gates 1 and 2 | `VcaEdits.faderDb` writes `-db` without the domain check | gate 1's value word and gate 2's render |
| `live-controls-evals.mjs` (the refusal eval) | an unknown VCA is refused `unknownRoute` | `an unknown VCA ID refuses with unknownVca` |
| `live-controls-evals.mjs` (the refusal eval) | `vca(id)` falls back to a track's index | the same eval: `vca("kick")` returns edits |
| `live-controls-evals.mjs` gate 2 only (E1, rebuilt module) | admission rides VCA `index ^ 1`'s offset | `a live VCA fader and mute equal the session booted`; gate 1 stays green (admission is `ok`) |
| `live-controls-evals.mjs` (the vocabulary test) | drop `vcaMute` from `kindNames` | `the semantic methods cover the generated command vocabulary exactly` |
| `live-controls-types.ts` | `SessionMap.vcas` is optional | `_SdkSessionMapVcas` fails, and the missing-`vcas` `@ts-expect-error` is unused |
| `live-controls-types.ts` | `VcaEdits` gains `solo` | `_VcaSurface` fails |
| `live-controls-types.ts` | `VcaEdits.faderDb` accepts a `gainDb` option | `Unused '@ts-expect-error'` |
| `live-controls-types.ts` | `VcaEdits.mute` accepts a number | `Unused '@ts-expect-error'` |

## Issue #1333 — the render-locked allocation counter and the render-thread static rules

Each row was applied, its test run red, and the tree restored. Rust rows ran on `x86_64`, debug
profile; browser rows rebuilt the module with `scripts/build-web-audioworklet.sh` and ran the
Chromium leg of `qualification/run.mjs`.

| gate | mutation | observed red |
|---|---|---|
| `render_lock::tests::render_lock_counts_each_entry_point_only_inside_its_own_window` | `count_if_locked` tests `!locked()` | `an unlocked thread counts nothing` fails |
| same | `count_if_locked` never does its `fetch_add` | the in-window count of 5 fails |
| same | `dealloc` (or `realloc`) skips `count_if_locked` | the in-window count of 5 fails |
| same | `render_locked` never clears the flag | `the window clears the flag when it returns` |
| `const _: () = assert!(!needs_drop::<..>())` in `ffi.rs` | add a `Vec<u8>` field to `BootStaging` | `error[E0080]`: `assertion failed: !needs_drop::<RefCell<BootStaging>>()` |
| `tests/render_locked_staging.rs` phase 1 (gate 8) | drop `response_staging()` from `reserve_stagings` | `a staging accessor allocated inside its render-locked window`: 7 |
| same | drop `observation_staging()` from `reserve_stagings` | the same assertion: 6 |
| same | drop the `reserve_stagings()` call from `boot_staged`'s success path, which both boot exports share | the same assertion: 13 |
| `tests/render_locked_staging.rs` phase 2 (Amendment 1, A1; mutation rewritten by #1479, which removed `selected_entry`) | `PreparedSpectrumCapture::channels`'s collection arm runs `let _ = capture.selected_target().cloned();` before `capture.selected_channels()` | `a collection capture's spectrum read allocated`: 4 |
| `host-core::spectrum::tests::selected_channels_reports_the_selected_entrys_mask` | `selected_channels` returns `Some(self.captures[self.selected.unwrap_or(0)].channels())` (a mask while nothing is selected) | `Some(Left)` where `None` is expected; no other host-core or host-web test goes red |
| `check-web-audioworklet-callgraph.py --self-test` (h2) | the destructor-registration test is `False` | both (h2) cases |
| same, (h3) | the atomic-wait test is `False` | (h3) `wait32` and `wait64` |
| same, (h1)/(h1a)/(h1b) | the `INDIRECT_SITES` comparison is skipped | (h1) new site, (h1a) removed site, (h1b) changed count |
| same, (h1d) | `unhashed` keeps the mangling hash | both (h1d) schemes |
| same, (h4) | `closure` takes no exact-name root | (h4): `miso_engine_web_v1_render` is ambiguous with `_render_allocation_count` |
| `qualification/run.mjs` `render-allocations` | a `Vec::with_capacity(1)` planted in `render_next` (rebuilt module) | every instance reads more than 0 (corpus 2 and 4, live control 260, stall 80, ...) and the gate fails |
| same | `PreparedSpectrumCapture::channels` back to the clone (rebuilt module, no `--sdk-root`) | `staging-reads-one-shot` reads 4 and `staging-reads-stream` reads 2; every other instance reads 0 |
| `qualification/run.mjs --self-test-mutations` `render-allocations` | one instance's count set to 1 | `<browser>: render-allocations` fails |
| `qualification/run.mjs --self-test-mutations` `staging-reads` | the one-shot spectrum read's result set to backpressure (6) | `<browser>: staging-reads` fails |
| `qualification/run.mjs` `sdk-render-allocations` (#1476) | `drop(Vec::<u8>::with_capacity(1))` planted in `EffectControlLane::stage`'s `EffectControlRecord::Bypass` arm (rebuilt module); no raw workload submits a bypass record | the four live-bypass SDK instances that submit one read 2 each and the gate fails naming them; `render-allocations` stays green (all ten raw rows 0) |
| same | one SDK instance (`spectrum-query-closed`) closed with `browser.close()` instead of `closeSdkEngine` | `<browser>: sdk-render-allocations` fails: 16 rows for 17 instances |
| `qualification/run.mjs --self-test-mutations` `sdk-render-allocations` (#1476) | one SDK instance's count set to 1 | `<browser>: sdk-render-allocations` fails naming `resident-observation=1` |
| `qualification/run.mjs --self-test-mutations` `sdk-render-allocations-missing` (#1476) | one SDK row removed | `<browser>: sdk-render-allocations` fails on the length check (16 rows for 17 instances) |
