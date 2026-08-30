| `test-web-audioworklet.mjs` unsupported-browser test (W4-D1) | delete the `if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");` guard in `createMisoAudioWorkletHost` | the refusal becomes a generic `miso.error.v1` 255 and the `compileCount` assertion fails |
| `test-web-audioworklet.mjs` source-ID UTF-8 parity test (#132) | change the four-byte sequence's `0xf0` lead-byte mask to `0xe0` in `writeBoundedUtf8` | the non-ASCII submit and seek bytes differ from the independent `TextEncoder` oracle |
| `check-web-audioworklet-callgraph.py --callgraph` on the shipped artifact (E1/E2) | restore `self.ready = None;` in `fail` and rebuild the artifact | closure 6 -> 23, traps 5 -> 16, 13 forbidden names appear (`drop_glue<Option<ReadyOwnership>>`, `drop_glue<PreparedRenderPlan>`, `drop_glue<SessionToml>`, `BTreeMap<StableId,_>` drop glue, `Arc<spsc::Ring<_>>::drop_slow` x2, `__rdl_dealloc`, `__rust_dealloc`, `dlmalloc::free`, `unlink_chunk`, `insert_large_chunk`, ...), and four of them become unexpected trap owners |
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

## Issue #143 E12 — the three-browser observation row

`qualification/run.mjs --browser all --self-test-mutations --record-matrix`, Playwright 1.62.1
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
| `check-command-reason-vocabulary.py` (the drift class) | add `pub const COMMAND_REASON_FUTURE_TAP: u32 = 12;` to `host-web/src/lib.rs` and nothing else | `host JS table disagrees with the Rust host constants` — a Rust reason bumped without the other five spellings. `test-web-audioworklet.sh` performs this one on a copied file tree, not only in memory |
| `check-command-reason-vocabulary.py --self-test` | eighteen in-memory mutations across all six spellings: a renumbered Rust constant; the JS table truncated at `wrongState`; the literal `<= 9` reinstated; the derived bound replaced by `reason <= 11`; the `.d.ts` enum missing or renaming a reason; a generator row dropped or emitting the wrong name for its own constant; the schema gate's list truncated; the render-thread worklet renumbering or renaming the one reason it produces itself | every one refused |
| `check-command-reason-vocabulary.py --self-test` (#151's typing half) | drop `observe()` from `MisoAudioWorkletHost`; drop `windowBlocks` from the declared subscription; add a `channel?` the implementation refuses; drop `frameSlot` from the declared binding; drop `reason` from the declared ack; add a binding field to the implementation the `.d.ts` does not declare | every one refused — the declaration is held to the shipped implementation's actual field sets, not to the issue's sketch |
| `check-parameter-metadata-v1.py --self-test` | truncate `commandReasons` at `wrongState`; rename reason 10; renumber reason 11 to `12` | `command reasons` / `command reason values` — the exact shape of the shipped vocabulary drift |

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
| `tests::solo_is_bit_identically_mute_on_the_complement` (P1-1) | drop `&& !self.solo(track)` from `ConsoleSoloState::effective_mute`, so the gate silences everything | the soloed tracks silence with the rest and the first commanded block differs from the explicit-mute arm |
| `tests::un_solo_restores_the_exact_per_lane_user_mute_set` (P1-2) | restore from the gate alone — `track_delta` composes `any_solo && !solo(track)` instead of `effective_mute` | the session's baked `left_mute` comes back unmuted and every block after the settle differs from the never-soloed arm |
| `tests::mute_and_solo_are_separate_states` (P1-4) | make `set_solo` clear that track's `user_mute` | a repeated solo engage un-mutes the track it re-engages, and the host mirror reads `[false, false]` where the user set `[true, true]` |
| `tests::a_refused_solo_submission_leaves_the_console_untouched` (P1-5) | delete the `ready.solo.rollback()` on `admit_commands`'s refusal path | the refused engage sticks in host state; the refused console and the untouched console diverge on the retry |
| `tests::a_solo_that_changes_nothing_emits_nothing` (the −0.0 pin) | drop the changed-lanes test in `ConsoleSoloState::track_delta` — `match (true, true)` | soloing the only track of a one-track console re-mutes its already-settled-muted lanes, the ramp kernel runs instead of the fill, and a negative input renders `-0.0` where the settled path renders exact `+0.0` |
| `tests::a_batch_of_alternating_solo_toggles_coalesces_to_its_net_effect` (the coalescing pin) | run the net-emission pass once per solo record rather than once per submission, and drop its `record_emitted` sync — per-command fan-out | a 256-record batch of alternating toggles fans out a gate record per track per transition and is refused instead of admitted |
| `tests::a_console_that_never_solos_renders_what_it_always_did` (the class-A OFF gate) | route `mute` through the coalesced net emission instead of staging its own record | the redundant re-mute of a settled-muted lane stages nothing, the plane stays `+0.0`, and the pinned `-0.0` ramp block is gone — a digest change on a path no solo command touched |
| `tests::the_decode_staging_holds_a_full_batch_plus_a_solo_transition` (the sizing correction) | size `command_decoded` `2 * MAXIMUM_COMMAND_RECORDS` again, without the `2 * track_count` term | 255 `channel = both` effect-parameter records (510 spans) plus one solo record on a four-track console need 513 entries; the batch is refused `malformed` by the staging bound |

`miso_engine_host_core::solo`'s own unit tests carry the state machine's algebra — the complement
composition, the per-lane restore, the two-record delta shape, `solo_count`'s incremental
maintenance, and the shadow/rollback — independently of any host.

## Issue #210 phase 3 — command kinds 10 (`trimDb`) and 11 (`polarityInvert`)

Driver: one mutation at a time on the committed tree, `cargo test -p miso-engine-host-web`, tree
restored between rows.

| # | mutation | test | result |
|---|---|---|---|
| P3-M31 | the decode whitelist drops `COMMAND_TRIM_DB` | `trim_and_polarity_are_admitted_on_every_lane_selector` (+5) | RED — every arm refuses `malformed` |
| P3-M32 | the admission dispatch drops the two kinds, so they fall to the `_ =>` arm | same (+5) | RED. This is the drift the kind-vocabulary gate **cannot** see: the constant still exists, the decode still admits it, and only the dispatch forgot it |
| P3-M33 | an input record is routed to the fader band | (5 tests) | RED |
| P3-M34 | the effect band is not moved past the new per-track band | (7 tests) | RED |
| P3-M35 | `queue_count` is not widened for the third band | (13 tests) | RED |
| P3-M36 | `queue_capacity` reports the fader depth for an input slot | — | **EQUIVALENT, and argued in the test**: a console leases all three of a track's queues at one depth (`TrackControlRequest::queue_capacity` is a single field), so the wrong queue's capacity is the right number. It becomes observable the day the three depths can differ, and the line is written per band so that day is a one-line change |
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
through the post-matrix meters, in `crates/miso-engine-host-core/tests/input_liveness_console.rs`.

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
| `dense_refusal_diagnostics_are_count_bounded_and_finish_under_one_second` | remove the encoder's 64-item `take` | the exact line-count pin sees `16,384` lines instead of `64` (the real tree also keeps the adversarial population below the one-second wall bound) |
| `maximum_document_dense_invalid_boot_is_typed_and_finishes_under_one_second` | bypass the semantic validator's 64-diagnostic accumulation guard | the production boot still returns its typed automation refusal, but the exact 1,048,576-byte document spends `36.580209774s` materializing every source span and fails the fixed `<1s` wall |
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
| `check-browser-expected-resources.py --self-test` plus direct/browser oracle | copied-fixture mutations perturb every resource row and each of the three frozen PCM digests | all 26 mutations are refused; identity, command, and observation PCM digest movement is never admitted as a re-pin |

## Issue #272 — the qualification session identities

The three `qualification/*.toml` documents declared `content` values minted from the old #241
locator names, not from canonical PCM, and nothing read them. `qualification/session-identities.mjs`
now re-derives each identity from the harness's own exported generator and `run.mjs::main` calls it
before a browser launches. Every row below was applied to the working tree, then `node
./session-identities.mjs` (or `node ./run.mjs`) was run from the qualification directory, the
failure was observed, and the mutation was reverted in the same session.

| Target | Mutation | Observed failure |
|---|---|---|
| `session-identities.mjs` console row | flip one hex digit of `console-session.toml`'s declared `content` | `session-identity: console-session.toml: declared source row is not the fed PCM's canonical identity` |
| `session-identities.mjs` stall row | flip one hex digit of `stall-session.toml`'s declared `content` | same refusal, naming `stall-session.toml` |
| `session-identities.mjs` observation row | flip one hex digit of `observation-session.toml`'s declared `content` | same refusal, naming `observation-session.toml` |
| the #272 defect itself | restore the pre-#272 name-minted `sha256("web-browser-console")` on `console-session.toml` | refused; the check states the derived identity the document must carry |
| cross-document reuse | declare the stall document's identity on the console document | refused; one digest cannot stand for two different fed regions |
| shape drift | `frames = 5120` -> `5121` on `stall-session.toml` | refused; shape and identity are one pinned row, because the preimage length is `frames * channels * 4` |
| generator drift | `OBSERVATION_LEVEL` `0.5` -> `0.25` in `qualification.js` | the derived identity moves to `680aca77…` and the unchanged document is refused — a pinned hex string would have stayed green |
| generator drift | flip the sign of `sourcePlanes`'s right plane | the console identity moves to `7499a91c…` and the unchanged document is refused |
| stale row beside a truthful one | add a second `content = "sha256:…"` source row to `stall-session.toml` | `expected exactly one source content identity, found 2` |
| the check's own comparison | the flipped-digit self-proof inside `checkSessionIdentities` | asserts a one-digit-off identity never matches, so the comparison cannot be loosened into a vacuous pass |

## Issues #280 and #281 — the qualification harness's artifact pin and its boot options

Two defects that together kept `npm run qualify` — the step
`.github/workflows/browser-qualification.yml` runs — from reaching a browser at all on `main`.
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
| `exactArtifacts` count clause | delete `names.length !== ARTIFACT_NAMES.size` (a subset would pass) | `Missing expected rejection: artifact-set: miso-engine-v2-abi-layout.json removed: red mutation escaped the artifact pin` |
| `exactArtifacts` name clause | delete `names.some((name) => !ARTIFACT_NAMES.has(name))` (a substitution keeping the count at six would pass) | `Missing expected rejection: artifact-set: miso-engine-v2-abi-layout.json replaced by a stray of the same count: red mutation escaped the artifact pin` |
| `exactArtifacts` regular-file clause | delete the `stat(...).isFile()` loop | `Missing expected rejection: artifact-set: directory named like an artifact: red mutation escaped the artifact pin` |
| the whole set check | delete the `throw` and its condition outright | `Missing expected rejection: artifact-set: miso-engine-v2-abi-layout.json removed: …` |

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
| `bootOptions` exactness | leave one #240-deleted ceiling (`sessionTomlBytes: 1 << 20`) in the returned object | same typed refusal; a superset is as invalid as a subset |
