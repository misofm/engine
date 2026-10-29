| `test-web-audioworklet.mjs` unsupported-browser test (W4-D1) | delete the `if (!WebAssembly.validate(SIMD128_PROBE)) throw unsupportedBrowser("simd128");` guard in `createMisoAudioWorkletHost` | the refusal becomes a generic `miso.error.v1` 255 and the `compileCount` assertion fails |
| `test-web-audioworklet.mjs` source-ID UTF-8 parity test (#132) | change the four-byte sequence's `0xf0` lead-byte mask to `0xe0` in `writeBoundedUtf8` | the non-ASCII submit and seek bytes differ from the independent `TextEncoder` oracle |
| `check-web-audioworklet-callgraph.py --callgraph` on the shipped artifact (E1/E2) | restore `self.ready = None;` in `fail` and rebuild the artifact | closure 6 -> 23, traps 5 -> 16, 13 forbidden names appear (`drop_glue<Option<ReadyOwnership>>`, `drop_glue<PreparedRenderPlan>`, `drop_glue<SessionModel>`, `BTreeMap<StableId,_>` drop glue, `Arc<spsc::Ring<_>>::drop_slow` x2, `__rdl_dealloc`, `__rust_dealloc`, `dlmalloc::free`, `unlink_chunk`, `insert_large_chunk`, ...), and four of them become unexpected trap owners |
| `check-web-audioworklet-callgraph.py --self-test` (a)-(f) | synthetic disassembly per case | each case is the red mutation of one rule; the runner fails if any escapes |
| `test-web-audioworklet.mjs` trap-containment test (F5) | remove the `try` around `miso_engine_web_v1_render` in `process()` | the `process()` call throws instead of returning `true` |
| `tests::facade_source_rules_reach_the_browser_host` (F1) | delete the `end_of_region != (end == region_end)` check from `host_core::SourceControlSet::submit` | the region-end submission returns `RESULT_BACKPRESSURE` (6) instead of `RESULT_INVALID_ARGUMENT` (1) |
| `host-core` `tests/prepare.rs::default_ring_adds_the_largest_prime_and_a_quantum_to_the_stall_ring` (F3; `tests::default_ring_covers_stall_tolerance` was deleted by TA4; re-run for #1406b) | `+ 2` -> `+ 1` in `stall_ring_frames` | 48 000/128 yields 7 168 where 7 296 is required (the four rates give `[6784, 7168, 12672, 13952]`, not `[6912, 7296, 12800, 14080]`); `parameter-metadata`'s `the_published_source_ring_rule_reproduces_the_engine_derivation` also fails, one quantum off |
| `tests::ring_prefill_survives_stall` (F3; re-run for #1406b) | `SOURCE_STALL_TOLERANCE_MS = 50` (a 19-quantum stall and a 38-quantum default ring: a 21-quantum stall ring, plus the 16-quantum prime and one quantum, #1406 D2) | the `stall_quanta == 38` assertion fails (`left: 19`, `right: 38`) |
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
| `tests::native_set_control_timeline_digest_pins_the_wasm_parity` + `direct-oracle.mjs::runSetControlTimeline` (#1565A D6) | have `control_command` stage the committed frame's cells one block later (move the service step's publication after the render) | the native digest and the shipped module's digest move together and stop matching the pin |
| `tests::set_control_frames_are_the_committed_fixtures` (#1565A D6) | change one byte of the encoder's request ID in `set_control_timeline_frames` | the committed frame fixture differs from the encoded frame |
| `tests::the_retired_boot_words_are_reserved_and_the_frame_word_is_bounded` (#1590 gate 4, #2191 gate 3; renamed from `the_retired_boot_words_are_reserved` by #2191, when offset 32 became the control frame word) | drop the `resolved_control_frame_bytes(&options).is_none()` refusal in `validate_options` | a frame word below 16 KiB or above 16 MiB boots |
| `tests::meter_frames_equal_an_offline_fold_and_cost_the_render_nothing` (#137 E5) | make `live_control_request` use `blocks` frames instead of `blocks * quantum_frames` for the meter period | a window closes mid-block, `poll_meters` reports more windows than blocks rendered, and the cadence assertion fails |
| `tests/matrix.rs::explicit_window_retarget_ramps_over_the_requested_window_and_is_adopted` (#137 D1) | drop `self.smoothing_samples[lane] = samples;` from `MatrixStage::set_target_over` | the second retarget runs over the prepared window of `0` instead of the requested `4` and settles on the first frame |
| `builtins-compiler::live_control_requests_are_validated_sealed_and_charged_per_track` (#137 D1) | delete the `control_tracks.insert` / `known_tracks.contains` legs in `prepare_session_builtins_with_live_controls` | the duplicate and unknown-track requests are accepted instead of producing `builtin.control.duplicate` / `builtin.control.unknown_track` |
| `host-core::live_controls_attach_bounded_control_and_meter_halves_in_canonical_track_order` (#137 D1/D2) | drop the `bound.track_controls.len() != control_requests.len()` leg of the live-control arity check | a silently skipped channel leaves nine tracks with eight producers and the per-track walk panics |
| `check-web-audioworklet.sh --source-policy` pinned-post rule (#137 D2/D3) | rename one pinned post, drop the telemetry post, or remove the meter lease guard at its call site | the occurrence count or the pinned line no longer matches and the frozen render-callback policy fails; `test-web-audioworklet.sh` runs all three |
| `check-web-audioworklet.sh --source-policy` pinned-clock rule (#137 D3) | read `Date.now()` anywhere outside `renderClock()`, including inside `process()` | the pinned-site count disagrees, or `process_policy_re` catches it in the frozen body |
| `check-web-audioworklet-callgraph.py --self-test` (b1) (#137; (b1b) and `--allocation-only` deleted by #1589c) | `--trap-owner` naming a different symbol | the case fails; the mode cannot admit an allocator |
| `check-parameter-metadata-v1.py --self-test` (#137 D4) | fourteen document mutations, including "a prepared-only builtin claims to be live" and "an effect parameter claims to be live" | every one is refused by the schema gate |
| `parameter_metadata -- --check` (#137 D4) | hand-edit one `liveUpdatable` in the shipped document | byte equality against a freshly generated document fails |
| `qualification/run.mjs --self-test-mutations` control-path gates (#137 E8) | `exactRetargetedOutput = false`, `masterPeak = 0`, or `commandApplied = 0` | `<browser>: control-path` fails on the applied change, on the meter frame, and on the application |
| `qualification/run.mjs --self-test-mutations` `stall-live-controls-load` (#137 E6) | `stall.liveControlMeterFrames = 0` | `<browser>: main-thread-stall` fails because the stall no longer carried a live command and meter load |

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-12 | the metadata emitter hardcodes `liveUpdatable: false` for every effect parameter again | `tools/parameter-metadata/src/lib.rs` | `scripts/check-parameter-metadata-v1.py` on the emitted document | RED (`FAIL parameter metadata: effect liveUpdatable follows automatable`) |

## Issue #143 — the effect observation surface

Every row applied to the working tree, the named binary run, the result recorded, the tree
restored. Host: `x86_64` (AMD Ryzen 7 9700X, Zen 5), `-C target-feature=+avx2,+fma`.

| gate | mutation | observed red |
|---|---|---|
| `tests::the_meter_frame_carries_the_app_shaped_gain_reduction` (E4) | publish the negative decibels raw instead of the declared `PeakMagnitude` fold | the app's `Math.max(0, -6)` is `0` and the frame reads dead; the "positive magnitude, not a negative decibel" assertion fires |
| control-plane `observation_bindings::a_computed_tap_is_refused_and_an_undeclared_tap_is_unknown` (E9; renamed by #1589c, moved from host-web `tests` by #1590) | bind the computed tap instead of refusing it | `Ok` where `Err(ComputedTap)` was required — a bound computed tap is a lane that never publishes |
| `round_trip::every_metadata_observation_tap_resolves_through_an_observe_edit` (E9; through the core's `Observe` edit since #1589c) | offset the tap id by one in the lowering (equivalent to a hand-edited id in the document) | `miso.compressor tap 1 resolves and binds`: `Err(["observation.tap.unknown"])` |
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
`firstSample` strictly monotonic with the windows tiling, a never-armed tap publishing no
reduction, and the armed and unarmed renders of the same sixteen blocks producing **bit-identical
audio**. (The row once also required an unsubscribe to stop the traffic; that real-browser gate was
removed by #1345 Amendment 7, see the notes below, and the Rust test
`a_withdrawn_tap_leaves_the_first_polled_frame_and_only_its_own_reading` holds the claim.) Self-test
mutations run against every browser's real result:

| mutation | gate |
|---|---|
| `observation-armed` (the eval's named case, `observationArmed = 0`) | `an armed tap published no reduction at all` |
| `observation-disarmed` (was `observation-unsubscribe`; the offline run that never subscribes must report no reduction) | `a tap that was never armed published a reduction` |
| `observation-identity` | `arming a declared tap moved a rendered sample` |
| `observation-window` | `observation windows did not advance monotonically and tile` |

*Note, 2026-10-13 (#1345 Amendment 7, split, root 2026-10-13).* The real-browser
`observation-unsubscribe` gate is removed: `runObservationUnsubscribeRun`, `unsubscribe-session.json`
and its `qualificationSessionSources` row, the gate in `run.mjs` and its four result-field mutations
(`observation-unsubscribe`, `-silent`, `-short`, `-norender`), the two page mutations
(`skip-unsubscribe`, `feed-ends-at-boundary`) and the eleventh render-allocation row
(`observation-unsubscribe`; the gate counts ten again). The notes and tables below that describe them
are kept as history and marked removed; the gate moves to successor S1 ("Qualify observation
unsubscribe on a real AudioContext in three browsers"), which starts from the saved patch
(`/home/bl/misofm/submix-verdicts/b3-acceptance/split/observation-unsubscribe-gate.patch`). The
removal also took the code only that gate called: the bounded waits, the suspended-context poll, the
trusted-click resume (`resumeThroughClick`, `resumeUntil`, `renderUntil`), `sampleContext` and the stall
classifier (`classifyStallSignature`, its printing in `run.mjs` and `stallSignatureProofs()`), because
no real-context gate remains in `qualification.js` to call them; the patch carries them. What stays:
the offline "disarmed" identity run (its zero-reduction check is now the `observation-disarmed` gate),
the `resume-clicks` gate (now two SDK clicks, none without the SDK) and its three mutations, and the
`spectrum-query-closed` handler fix.

*Note, 2026-10-12 (stream B batch-3 verdict B1, #1345 Amendment 5; the gate described is removed by Amendment 7).* The engine admits one
observation record per tap per render (#1345 D8, D2 Amendments 1 and 2), so the raw `host.observe`
unsubscribe of this row, sent before any render, was refused with typed backpressure on all three
browsers. Firefox has no `OfflineAudioContext.suspend`, so the unsubscribe now runs on a real
`AudioContext` held suspended between steps (`runObservationUnsubscribeRun`): feed the region,
subscribe, resume until a frame arrives, suspend, unsubscribe, seek the source back and feed the
region again, resume until sixteen more blocks have rendered. The `observation-unsubscribe` mutation
now sets `observation.unsubscribe.maximumTrackGrDbAfterUnsubscribe = 1`, and the gate's message
reads `an unsubscribed tap kept publishing, or the run never exercised the armed tap` followed by
the run's numbers. The offline "disarmed" run never subscribes; it still feeds the identity gate
and must report no reduction. A render-allocation row, `observation-unsubscribe`, makes eleven.

*Note, 2026-10-12 (batch verdict 2, B2; removed by #1345 Amendment 7 with the gate and its bounded waits).* Every wait of `runObservationUnsubscribeRun` is bounded
at 10 s, the bound `sdk-response-entry.ts` uses, and a stall fails the gate as `<kind> timeout at
<step>` (kind `suspend`, `resume` or `await`) with the context state; a timeout is never a pass, a
skip or a retry. The first `suspend()` of the new context is not awaited (it is only a request; a bounded
poll replaces the await); a bounded poll requires `context.state` to be `suspended` before boot and
again before the unsubscribe, and the run records both states. New mutation
`observation-unsubscribe-context-running` sets the second recorded state to `running`; the
`observation-unsubscribe` gate must then fail (the same gate, the same message). *Page-level checks
(not committed; chromium, firefox and webkit, one run each):* with `context.suspend` stubbed to
resolve without suspending after boot and the "before the unsubscribe" check replaced by
`context.state`, the recorded state is `running` and the `observation-unsubscribe` gate is red in
all three browsers; with the check in place the same stub fails the run as `suspend timeout at
before the unsubscribe`. With the stub from the start, the check in place fails as `suspend timeout
at the new context before boot` (chromium creates its context running; run on chromium only); with that first check
removed too, `createHost` refuses the running context (`miso.error.v1`, result 1) in all three.
*Engine-side red check (not a committed mutation):* with the page's unsubscribe replaced by a
stub that sends nothing, chromium reported `maximumTrackGrDbAfterUnsubscribe` 5.38 and the gate
failed; with the unsubscribe sent, 0.

*Note, 2026-10-12 (#1345 Amendment 6, root: one resume, no suspend after it; the unsubscribe run is removed by Amendment 7).* This note supersedes the two
notes above for the unsubscribe run: `runObservationUnsubscribeRun` resumes its real `AudioContext`
once, through one trusted click, and never suspends it before the final close. The render between the
subscribe and the unsubscribe is proved, not arranged: the host status read after the subscribe
resolved (`R1`) must be exceeded by the status read just before the unsubscribe. The deleted
`contextStateBeforeUnsubscribe` check and its `observation-unsubscribe-context-running` mutation went
with the suspend. The run has its own session document, `unsubscribe-session.json`, with a 512-block
source (the observation document's sixteen blocks cannot hold the armed blocks, a render and sixteen
blocks after the acknowledgement; `observation-session.json` is unchanged and the run uses no seek). It
feeds a +-0.5 square wave (a constant would be removed by the strip's input high-pass within a few
blocks, so a later block could not tell signal from silence), boots the default 5,120-frame ring and a
one-block meter window, and keeps the source twelve blocks ahead of the render: every meter frame tops
the ring up to twelve blocks past the block the frame closed, with the blocks sent back to back
(the feed asks the processor for nothing; a feed that read the status and awaited one acknowledgement
per block starved on WebKit and Firefox and dropped the signal for several blocks). The gate
(`unsubscribeVerdict`, shared with the page mutations) requires the armed tap to publish a positive
reduction, `R2 > R1`, `trackGrDb == 0` on **every** frame the page received after the unsubscribe
acknowledgement (the acknowledgement and the frames use one FIFO port, so those frames were polled
after the tap was disarmed; no frame and no block after the boundary is excluded), at least sixteen
blocks of such frames, and a track peak above 0.05 on every such frame. The host publishes no source
underrun count, so the per-block peak is the evidence that no block of zeros was rendered. Under
Amendment 6 `resume-clicks` declares two SDK clicks and one gate click.

Result-field mutations **(removed 2026-10-13, #1345 Amendment 7; history)** (`mutate()` in `run.mjs`,
each required to fail `<browser>: ...` on every browser's real result):

| mutation | change | gate |
|---|---|---|
| `observation-unsubscribe` (a) | `maximumTrackGrDbAfterAcknowledgement = 1` | `observation-unsubscribe` |
| `observation-unsubscribe-silent` (b) | `minimumPeakAfterAcknowledgement = 0` | the same gate: a zero from silence |
| `observation-unsubscribe-short` (c) | `blocksAfterAcknowledgement = 15` | the same gate: fewer than sixteen blocks |
| `observation-unsubscribe-norender` | `renderedBeforeUnsubscribe = renderedAfterSubscribe` | the same gate: no render between the two |

Page mutations **(removed 2026-10-13, #1345 Amendment 7; history)** (`unsubscribePageMutationProofs` in `run.mjs`, under `--self-test-mutations`, a fresh
page of every browser; each prints its numbers, requires the verdict to refuse them, and requires the
verdict to accept them again when only the one field that should refuse them is repaired, so it proves
which check turned red):

| mutation | defect applied in the page | refused by |
|---|---|---|
| (d) `skip-unsubscribe` | the unsubscribe is never sent (a fabricated result 0 stands in) | `maximumTrackGrDbAfterAcknowledgement` > 0 on every frame after the "acknowledgement" |
| (e) `feed-ends-at-boundary` | the source feed stops when the acknowledgement arrives | `minimumPeakAfterAcknowledgement` below the floor, with `maximumTrackGrDbAfterAcknowledgement` 0 and 16 or more blocks, so only the floor sees it |

*Test value.* (a) defends the claim that an unsubscribed tap leaves the frame. (b) and (e): a zero
reduction that comes from a source that stopped feeding, which the reduction check alone would pass.
(c): a gate that proves nothing from a handful of frames. The `norender` mutation: an unsubscribe sent
with no render beyond the subscribe, which the engine would refuse with backpressure and which the
Amendment-5 suspend used to arrange by hand. (d): a gate whose reduction check cannot tell a disarmed
tap from an armed one; no result-field mutation can show that, because it never touches the browser.

*Note, 2026-10-12 (batch verdict 2, B2 diagnosis and second round).* Every `resume()` of a real
`AudioContext` of the page now runs inside a trusted click. WebKit starts or restarts an
`AudioContext` only inside a user activation and otherwise leaves `resume()` pending. The page
declares its click plan (`resumeClickPlan`: the two SDK controls, when the SDK workloads run, and the
two `data-qualification-resume` buttons (one since Amendment 6) of `resumeThroughClick`; since Amendment 7 the plan is the two SDK
controls only and empty without the SDK, the buttons having gone with the gate), `run.mjs` clicks exactly those
controls, counts its clicks and runs the page under one 300 s deadline, and the page counts every
`AudioContext.resume()` by whether `window.event` was a trusted click. The result carries
`resumes: { declared, inClick, direct, planned, clicksPerformed }`; the `resume-clicks` gate requires
`direct` 0 and the other four equal. The unsubscribe run had no recovery: the former `resumeUntil` kick
(`suspend()` then another click) was removed because it is a retry, and a context that did not pull its
graph failed as `stall timeout at <step> after 10000 ms` with the context state, `currentTime` and the
host's status (`renderedQuanta`) sampled right after the resume and at the timeout. That stall
sampling (`sampleContext`, `renderUntil`) is itself removed by Amendment 7. Three committed
self-test mutations of the real result, each required to fail with `<browser>: resume-clicks:`:

| mutation | change | gate |
|---|---|---|
| `resume-clicks-direct` | `resumes.direct = 1` | `resume-clicks`: a real AudioContext was resumed outside a trusted click |
| `resume-clicks-missing` | `resumes.clicksPerformed` one fewer than declared | `resume-clicks`: the harness did not click exactly the declared controls |
| `resume-clicks-extra` | `resumes.clicksPerformed` one more than declared | the same gate |

**The stall signature (root ruling, 2026-10-12, stream B batch 3; removed 2026-10-13, #1345 Amendment 7: the classifier, its printing and `stallSignatureProofs()` had no caller once the unsubscribe gate went, and are in the saved patch for S1; history).** A `stall timeout` failure carries a
structured `stallSignature` (`classifyStallSignature` in `qualification.js`), and `run.mjs` prints it as
`<browser>: stall-signature: {...}` and keeps it in the failure's `qualificationError.value`. It is
`full: true` only if the context state is "running" at the resume and at the timeout, `currentTime` and
`renderedQuanta` are equal at both, at least two status replies were answered in the window (polled every
250 ms, each bounded to 1 s; the processor answers from its port handler, so an answered reply shows
`process()` was not on the stack) and `lastResult` is 0 at both. `--self-test-mutations` also runs
`stallSignatureProofs()` once: a full record must stay full and each of eight records, each missing one
condition (state not running at the timeout or the resume, `currentTime` moved, `renderedQuanta` moved,
one reply, no reply, `lastResult` 6, unanswered timeout sample), must give `full: false`.
Applied to the classifier in the working tree (red; reverted: green): dropping the `renderedQuanta`
equality gives `a record without renderedQuanta moved was classified full`; `answered >= 2` -> `>= 1`
gives `a record without one status reply ...`; dropping the `lastResult` test at the timeout gives
`a record without last result not 0 ...`.

*Page-level checks (not committed; chromium and webkit, no busy loops):* with the first
`resumeThroughClick` replaced by a direct `context.resume()`, both browsers go red as `resume timeout at
the replayed region to render (click)` (the harness waits for the first control, the page offers the next
one, and no run reaches the result; `/tmp/claude-1002/b2r2/logs/mut-direct-chromium.log` and
`mut-direct-webkit.log`); with `resumeClickPlan` declaring one control the page never offers, chromium
fails `resume-clicks` with `{"declared":5,"inClick":4,"direct":0,"planned":5,"clicksPerformed":4}`
(`/tmp/claude-1002/b2r2/logs/mut-missing-chromium.log`). Before the click existed,
the gate's first `resume()` called directly with the page idle for 6 s after the SDK phase failed as
`resume timeout at a frame from the armed tap` on WebKit (`/tmp/claude-1002/b2diag/logs/mut-A-webkit.log`);
with the click it passed (`mut-B`).
*The kick's hang count, one sourced figure:* a script `suspend()` followed by a script or click `resume()`
as the recovery left the minimal page (a real `AudioWorkletNode`, no engine code, 40 fresh contexts per
run, 32 busy loops, WebKit) hung with `resume()` never returning in 3 of 8 runs
(`/tmp/claude-1002/b2diag/drv5.out`; the `HANG` lines of runs 2, 6 and 8).

### And the same mutation against a real engine, in a real browser

The self-test mutates a *result*. To prove the gate catches a mutated *engine*, the
`ObservationLane::accumulate` armed guard was changed to `return;` — so no armed tap ever
accumulates — the browser artifact was rebuilt from that tree, and chromium was re-qualified:

```
Error: chromium: observation-armed: an armed tap published no reduction at all
```

Reverted in the same session; the recorded `results.json` and matrix are from the unmutated tree.

## Issue #151 — the command-reason cap and the `observe()` typing gap

Its gates (`check-command-reason-vocabulary.py`, the reason-cap mutation, `observe()`'s typing) were
deleted with the command record, its reasons and `observe()` by #1589c: an observation tap is armed
by the core's `Observe` edit and its refusal is a control reply, not an acknowledgement reason.

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
| `representative_retained_projection_tracks_the_post_prepare_exact_aggregate` | omit the compiled-model row from the retained projector | the 64-track representative reports gap `2,889,216` above the documented `1,396,479` bound, so an underbound projector cannot silently stale (dated: the bounds were re-pinned by #2191 and #2194) |
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
| `tests::a_bus_compressor_reports_its_gain_reduction_in_the_bus_word`, `tests::a_bus_limiter_can_be_the_designated_master` (#1213 gates 5, 6) | restore #1207's GR-fold skip of strip indices `>= T` | the bus word stays `0` and `master_gr_present` is `0` |
| (retired by #1590: `observation_selection_for_address` and the numeric read are deleted; the bus test and `capability-evals.mjs` read the bus tap by its stable key through `OBSERVATION_READ`) | | |
| `capability-evals.mjs` bus refusal classifier (#1213 D5a, `check-sdk-headless.sh`) | the SDK classifier compares against the track count | the bus's missing insert reports `unknownTrack` |
| `capability-evals.mjs` bus solo refusal (#1213 D4, #1212 NIT-2, `check-sdk-headless.sh`) | delete the `solo_safe` check before `set_solo`, artifact rebuilt | the shipped admission reports reason 2, not 12 |
| `test-web-audioworklet.mjs` worklet eq-config classifier (#1213 D5a; attempt 2 MINOR-3) | compare `message.trackIndex >= this.trackCount` | the bus's missing insert reports `2` (`unknownTrack`), not `4` |

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
| (retired by #1590: `tests/render_locked_staging.rs`, `reserve_stagings` and the observation staging are deleted; no staging is left for a booted instance to allocate in a render-locked window) | | |
| `host-core::spectrum::tests::selected_channels_reports_the_selected_entrys_mask` | `selected_channels` returns `Some(self.captures[self.selected.unwrap_or(0)].channels())` (a mask while nothing is selected) | `Some(Left)` where `None` is expected; no other host-core or host-web test goes red |
| `check-web-audioworklet-callgraph.py --self-test` (h2) | the destructor-registration test is `False` | both (h2) cases |
| same, (h3) | the atomic-wait test is `False` | (h3) `wait32` and `wait64` |
| same, (h1)/(h1a)/(h1b) | the `INDIRECT_SITES` comparison is skipped | (h1) new site, (h1a) removed site, (h1b) changed count |
| same, (h1d) | `unhashed` keeps the mangling hash | both (h1d) schemes |
| same, (h4) | `closure` takes no exact-name root | (h4): `miso_engine_web_v1_render` is ambiguous with `_render_allocation_count` |
| `qualification/run.mjs` `render-allocations` | a `Vec::with_capacity(1)` planted in `render_next` (rebuilt module) | every instance reads more than 0 (corpus 2 and 4, live control 260, stall 80, ...) and the gate fails |
| `qualification/run.mjs --self-test-mutations` `render-allocations` | one instance's count set to 1 | `<browser>: render-allocations` fails |
| `qualification/run.mjs --self-test-mutations` `staging-reads` | the observation read's result set to backpressure (6) (#1597, #1580: the staging reads have no spectrum or track-response read) | `<browser>: staging-reads` fails |
| `qualification/run.mjs --self-test-mutations` `track-response-host-outcome` (#1580 D2) | the host-level `RESPONSE_READY`'s status set to `unavailable` | `<browser>: track-response-host` fails |
| `qualification/run.mjs --self-test-mutations` `track-response-host-tag` (#1580 D2) | no `RESPONSE_READY` carries the reply's tag | `<browser>: track-response-host` fails |
| `qualification/run.mjs --self-test-mutations` `observation-arm-refused` (#1589c) | the observation run's `Observe` arm reply status set to `not_found` | `<browser>: observation` fails |
| `qualification/run.mjs` `sdk-render-allocations` (#1476) | `drop(Vec::<u8>::with_capacity(1))` planted in `EffectControlLane::stage`'s `EffectControlRecord::Bypass` arm (rebuilt module); no raw workload submits a bypass record | the four live-bypass SDK instances that submit one read 2 each and the gate fails naming them; `render-allocations` stays green (all ten raw rows 0) |
| same | one SDK instance (`spectrum-query-closed`) closed with `browser.close()` instead of `closeSdkEngine` | `<browser>: sdk-render-allocations` fails: 16 rows for 17 instances |
| `qualification/run.mjs --self-test-mutations` `sdk-render-allocations` (#1476) | one SDK instance's count set to 1 | `<browser>: sdk-render-allocations` fails naming `resident-observation=1` |
| `qualification/run.mjs --self-test-mutations` `sdk-render-allocations-missing` (#1476) | one SDK row removed | `<browser>: sdk-render-allocations` fails on the length check (16 rows for 17 instances) |
| `qualification/run.mjs` `sdk-spectrum-continuous` `gap` (#1480) | `nativeMissedWindows: 0n` and `skippedPublications: 0n` in the spectrum notification (`sdk/src/core/observation-subscriptions.ts`, `#notifySpectrum`) | `<browser>: sdk-spectrum-continuous` fails with `false: gap` on 5 of 5 runs in each of Chromium, Firefox and WebKit; `renderLoss` stays true (the native loss happened, the SDK hid it) |
| `qualification/run.mjs` `sdk-spectrum-continuous` `renderLoss` (#1480 D2) | delete the read hold in `runContinuousSpectrumQualification` (the `await readsHeld` and the `Promise.allSettled` of the in-flight reads), so reads race the offline render as before | Firefox: red on 23 of 25 runs, 22 naming `renderLoss` (a mid-render read left 0-8 drops instead of 16) and 1 naming `gap, renderLoss` (the #1480 D1 race); the 2 green runs had no read execute during the render |
| `test-web-audioworklet.mjs` qualification boot contract, staging-read caller (#1477 D1) | remove `runStagingReadRun` from `qualificationBootContract` | the suite fails at the caller (`hooks.runStagingReadRun is not a function`) before its witness counts |
| `test-web-audioworklet.mjs` render allocation reply check (#1477 D3) | drop `message.result === RESULT_OK` from `validRenderAllocations` | the host accepts `result 6` |
| same | `validU32(message.count)` -> `Number.isInteger(message.count)` | the host accepts `count -1` and `count 4294967296` |
| same | drop `validU32(message.count)` | the host accepts `count -1`, `4294967296`, `1.5`, `"0"` and `0n` |
| same | add `"extra"` to the `renderAllocations` reply field list | the host accepts the extra-field reply |
| same | change the `renderAllocations` expected tag to `miso.status.v1` | the host accepts the wrong-tag reply |

## Issue #1530 — the browser observation wait against the real AudioWorklet

Gate `sdk-observation-wait` in `qualification/run.mjs` (predicates 1-3) over two SDK instances,
`observation-wait-deadline` and `observation-wait-render`, in the three `browser` legs.

### Result mutations (`--self-test-mutations`)

Each makes `validate` throw `<browser>: sdk-observation-wait:`; `mutationProofs` requires that prefix.
All eight were checked against a green fixture of the result (node, no browser); the real three-browser
runs are the `browser` legs of `qualification.yml`.

| Where | Mutation | Observed red |
|---|---|---|
| `sdk-observation-wait-unbounded` | `wait.deadline.settled = false` | gate 1 (`settled`) |
| `sdk-observation-wait-late` | `wait.deadline.elapsedMs = 5_000` | gate 1 (upper bound 4_000) |
| `sdk-observation-wait-early` | `wait.deadline.elapsedMs = 0` | gate 1 (lower bound 225) |
| `sdk-observation-wait-untyped` | `wait.deadline.errorCode = "invalidArgument"` | gate 1 (`errorCode`) |
| `sdk-observation-wait-rendered` | `wait.deadline.renderedQuantaAfterRefusal = "1"` | gate 1 (the context did not render) |
| `sdk-observation-wait-after-render` | `wait.deadline.closeAfterRender.resolved = false` | gate 2 |
| `sdk-observation-wait-resend` | a second `bp` entry before the last `ok` of `wait.render`'s command log | gate 3 (log) |

### Source mutations (PR evidence; one at a time in `sdk/src`, then reverted)

Not run in this slice (sprint mode, root 2026-10-13: no mutation runs locally). The expected red
is the spec's (gate 5); the observed red and message belong to the successor "Post-sprint
qualification of decision 15's heavy gates" (#1605) or the first CI run that applies them.

| Where | Mutation | Expected red |
|---|---|---|
| `qualification/run.mjs` `sdk-observation-wait` (S1) | delete the `limit` timer in `next` (`sdk/src/browser/engine.ts`, `setTimeout(() => finish(false), timeoutMs)`; keep `limit` defined) | gate 1: `settled: false` at the 10 s guard |
| same (S2), Chromium, Firefox, WebKit | `boundMs: 5_000` regardless of `requestDeadlineMs` | gate 1: late (`elapsedMs` above 4_000) |
| same (S3), Chromium, Firefox, WebKit | in `#submit` (`sdk/src/core/observation-subscriptions.ts`), drop `if (!rendered)` so the typed refusal is thrown whatever `next` returns | gate 3: the render instance's close ends in `backpressure` (`resolved: false`, log `[ok, bp]`) |
| same (S4) | `current.renderedQuanta >= since` in `next` | gate 1: early, and more than one close send in the log |
| same (S5) | the browser engine passes no `renderWait` | gates 1 and 3 |

## Issue #1533 — any page error or unhandled rejection fails the browser qualification

Gate `page-errors` in `qualification/run.mjs` (`validate`, right after `result-schema`) over two
channels: Playwright `pageerror`, and page-side `error`/`unhandledrejection` listeners installed by
`page.addInitScript` before `page.goto`. Unmutated tree (gate 1): green in Chromium 151.0.7922.34,
Firefox 153.0 and WebKit 26.5 (Playwright 1.62.1), one unloaded run each, with
`--sdk-root --check-matrix --self-test-mutations`.

### Result mutations (`--self-test-mutations`)

Each mutation changes only `result.pageErrors` of the real result, and `validate` passes on the
unmutated result, so the throw is `<browser>: page-errors:`. All three red in the three legs above.

| Where | Mutation | Observed red |
|---|---|---|
| `page-errors` | push `"pageerror: Error: planted"` into `pageErrors.playwright` | `page-errors` |
| `page-errors-window` | push `"unhandledrejection: Error: planted"` into `pageErrors.window` | `page-errors` |
| `page-errors-missing` | delete `result.pageErrors` | `page-errors` (`null`) |

### Page mutations (PR evidence; one at a time, then reverted)

| Where | Mutation | Observed red |
|---|---|---|
| P3, Chromium, Firefox | `void Promise.reject(new Error("planted"));` as the first line of `runQualification` (`qualification.js`) | `page-errors: {"playwright":["pageerror: Error: planted"],"window":["unhandledrejection: Error: planted"]}` |
| P3, WebKit (with and without `--sdk-root`) | same | `page-errors: {"playwright":["pageerror: Unhandled Promise Rejection: Error: planted"],"window":["unhandledrejection: Error: planted"]}` |

Not run in this slice (sprint mode, root 2026-10-24): P1 (the B2 order restored in
`sdk-response-entry.ts`), P2 (P1 without the `addInitScript` channel) and the 5 loaded WebKit runs
(spec gate 4). They move to #1605 "Post-sprint qualification of decision 15's heavy gates".

## Issue #1535 — observation unsubscribe on a real AudioContext (2026-10-24)

Gate `observation-unsubscribe` in `qualification/run.mjs` on `unsubscribeFailures`
(`qualification/qualification.js`, D5), over `runObservationUnsubscribeRun`: one real
`AudioContext`, one trusted-click resume, `unsubscribe-session.json` (512 blocks). Unmutated tree:
green in Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 (Playwright 1.62.1), one unloaded
run each with `--sdk-root --check-matrix --self-test-mutations` (`resume-clicks` 3), and in
Firefox and WebKit without the SDK (`resume-clicks` 1). This replaces the gate #1345 Amendment 7
removed (the history rows above).

Reds on the unmutated tree: two Firefox 153.0 runs failed while the PulseAudio socket path was too
long (113 characters), so there was no sink. Without the SDK: `resume timeout at the armed tap to
render (click) after 10000 ms: state=suspended`. With the SDK: `spectrum collection audio resume
timed out (gesture=true, state=suspended)`. `main` was red the same way under that path and green
with a short one (spec, Attempt record).

### Node proof (`--self-test-mutations`, no browser)

`unsubscribeFailureProofs()` holds `unsubscribeFailures` to a table: a green run at the bounds (four
armed frames, 48 signal frames, 40 sources in flight) gives `[]`, three low-peak frames with minimum
peak 0 give `[]` (root ruling R1), and each row below gives exactly its named failure. It also
asserts `UNSUBSCRIBE_SIGNAL_FRAMES` (48) > `RING_BLOCKS` (40), and that `requestFailure` names the
step, the kind, the counts and a refusal record's `result`/`requestId`, never `[object Object]`.

| Row | Change | Named failure |
|---|---|---|
| context | `contextStateBeforeBoot = "running"` | `context` |
| armed frames / armed reduction | `armedFrames = 3` / `armedMaximumTrackGrDb = 0` | `armed` |
| render-between | `renderedBeforeUnsubscribe = renderedAfterSubscribe` | `render-between` |
| unsubscribe-ack | `unsubscribeBindings = 1` | `unsubscribe-ack` |
| reduction / no frame (D9) | `maximumTrackGrDbAfterAcknowledgement = 1` / `framesAfterAcknowledgement = 0` | `reduction` |
| shortfall frames / flag | `signalFramesAfterAcknowledgement = 47` / `signalShortfall = true` | `shortfall` |
| shortfall, source exhausted | `sourceExhausted = { renderBlock: 481, fed: 512 }` | `shortfall` |
| feed in flight / behind | `maximumSourcesInFlight = 41` / `feedBehind = { block: 20, renderBlock: 21 }` | `feed` |

Source mutations of the proof, run once each and reverted (PR evidence):

| Mutation | Observed red |
|---|---|
| drop the `signalShortfall !== false` term of `shortfall` | table row "shortfall flag" |
| `UNSUBSCRIBE_SIGNAL_FRAMES = 16` | table row "shortfall frames" |
| `requestFailure` formats a record as `String(cause)` | `request-failure: "result":6 missing ... cause=[object Object]` |

### Result mutation (`--self-test-mutations`)

| Where | Mutation | Observed red |
|---|---|---|
| `observation-disarmed-empty` | `observation.disarmed.frames = 0` | `observation-disarmed` (D9) |

`stallSignatureProofs()` is restored unchanged from the saved patch.

### Moved to #1605 (sprint mode, root 2026-10-24)

The seven result-field mutations and `requestFailureProofs()` of the spec are replaced by the node
proof above. Page mutations (d) `skip-unsubscribe` and (e) `feed-ends-at-boundary`, and source
mutations M1-M3 (spec gate 6), are not run in this slice; they move to "Post-sprint qualification
of decision 15's heavy gates" (#1605) with the loaded acceptance (gate 8).

## Issue #2092 — Firefox is no longer run

Owner ruling 2026-10-25: the browser qualification runs Chromium and WebKit only. The Firefox rows
above are dated evidence of runs made before that ruling; no later run repeats them.

## Issue #2194 — effect state admitted before allocation (2026-10-29)

Each row was run once against the implementation with the mutation applied, then restored.

| Gate | Mutation | Red result |
| --- | --- | --- |
| `boot_transient_budget::effect_state_the_budget_cannot_hold_is_refused_before_preparation` (gate C) | drop the effect row from `projected_retained_bytes` (the tree at the slice-1 commit) | the boot prepares every delay before the exact aggregate refuses it: `the refusal peaked at 400289233 bytes, over the 6940780-byte parse transient` |
| `tests::a_replace_parse_is_projected_beside_the_live_session` (gate G) | skip `replace_parse_refusal` in `stage_replace_document` | the derived document limit stages beside the live session: `the boot limit's document beside the live session` |
| `tests::retained_projection_budget_diagnostic_names_projected_bytes` | drop `effect_state_bytes=` from the retained-projection diagnostic | the byte-for-byte diagnostic assertion fails |
| `check-web-boot-budget.mjs` effect-state leg (gate D) | none run: the wasm leg is red only on the tree before #2194 (host-core's effect cap alone already refuses this document at the default budget), and a base module build was not repeated | not recorded |
