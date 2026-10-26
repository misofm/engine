# Measure BS.1770-5 loudness and true peak at the master

Filed by root for the console sprint (2026-10-26). The owner chose loudness metering in the engine.

## Product outcome
An engine observation at the master (and any output) that gives ITU-R BS.1770-5 loudness: momentary (400 ms), short-term (3 s), integrated (gated) and loudness range (EBU Tech 3342), plus true-peak (BS.1770-5 Annex 2, 4x oversampling) in dBTP. It is readable by the console, by voice control and by future fan mixes on both hosts, through the core's observation frames.

## Smallest slice (the scope note decides)
K-weighting filters at the four launch rates, the 400 ms and 3 s windows with 75 % overlap, the absolute (-70 LUFS) and relative (-10 LU) gates, LRA by EBU Tech 3342, and true-peak via the existing Annex 2 detector code where possible. Reset and integration control (start or reset the integrated measure).

## Gates
EBU Tech 3341 and 3342 conformance signals (the minimum requirements), with each listed case within its tolerance; tests at all four launch rates; zero render-path allocation; the observation frames on both hosts.

## Decision record (root, 2026-10-26; scope note `scope/2115.md` accepted)
- **Shape.** A session-level output loudness meter, the output peak meter's twin, at the rendered master pair; not an
  effect tap, not a strip `MeterMetricSet` bit. Render half and control half live in the new `crates/loudness`; the
  control-plane wiring, arm gate and SPSC are S2; a new `LOUDNESS` (`0x0014`) monitor command is S2; the SDK is S3.
- **S1 paths.** `crates/loudness` (new), `crates/dsp-reference/src/{loudness.rs,lib.rs}`, `crates/true-peak-limiter/src/lib.rs`
  (`pub mod annex2`, visibility only: the limiter keeps calling the same code and no rendered bit moves), `Cargo.toml`,
  `Cargo.lock`, `dsp-research/{loudness.md,BIBLIOGRAPHY.md}`, this spec.
- **Hop = 10 ms (rate/100 samples), changed from the scope note's 100 ms.** EBU Tech 3341 cases 13/14 put a 400 ms
  burst at offsets of i x 20 ms and require Max M within 0.1 LU; a 100 ms grid can miss 50 ms of the burst
  (-0.58 LU). Every Tech 3341/3342 recipe duration is a multiple of 10 ms, and on any programme a 10 ms grid misses
  at most 5 ms of a window. M and S therefore update every 10 ms; gating blocks (I) and S-blocks (LRA) keep the
  100 ms step. S2 sizes its record queue for 100 records/s.
- **K-weighting.** Designed at the session rate (44.1/48/88.2/96 kHz only) by bilinear transform with prewarping, in
  `f64`, from analog prototype constants that reproduce the BS.1770-5 48 kHz table to about 1e-15. Stage 2 keeps
  BS.1770's 48 kHz passband gain (+0.0433 dB, numerator exactly 1, -2, 1 at 48 kHz) at every rate, so a signal reads
  the same at every rate; an unnormalized 1, -2, 1 at every rate would spread 0.025 dB across the four rates (44.1 kHz +0.0471 dB to 96 kHz +0.0217 dB).
- **True peak.** The limiter's Annex-2 detector (`true_peak_limiter::annex2`), `f32`, L/R as two lanes of one
  native-width bank, the same 4x table at all four rates.
- **Integrated and LRA.** Bounded 0.1 LU histograms over [-70, +30) LUFS (above full scale: the `f32` master can
  exceed it) with `u64` counts and rounded `f64` energy sums; the gate-straddling bin is the only approximation, bounded by the oracle test (|dI| <= 0.05 LU,
  |dLRA| <= 0.1 LU).
- **Conformance scope.** Recipe-defined cases only: Tech 3341 1-5, 9-23; Tech 3342 1-4. Excluded: Tech 3341 case 6
  (5.0 layout; the engine measures stereo), Tech 3341 7-8 and Tech 3342 5-6 (authentic programme files), the
  BS.2217 file set. These are successors, filed by root.

## Evidence
S1 attempt 1 (implementer; root's verifier has not reviewed it yet).
- **Debug cost.** The production observer runs about 3 us per frame unoptimised, because the Annex-2 FIR uses the generic
  `wide` lane ops, and the suite feeds about 160M frames (13 min 22 s in debug). Root decision: `[profile.dev.package.loudness]
  opt-level = 3` in the root `Cargo.toml` (debug assertions and overflow checks stay on), and the recipe tests check
  the oracle against the expected values at 48 kHz only. The histogram test cross-checks the oracle at 44.1 and 96 kHz
  against production. Debug `cargo test -p loudness` now takes 25 s.
- **Tech 3341, production (the oracle agrees to 4 decimals), LUFS, at 44.1/48/88.2/96 kHz, all within +-0.1 LU:**
  case 1 -22.994/-22.993/-22.989/-22.989; 2 -32.994/-32.993/-32.989/-32.989; 3 and 4 -23.015/-23.014/-23.010/-23.010;
  5 -22.980/-22.979/-22.975/-22.974; 9 (S after 3 s) -22.987/-22.986/-22.982/-22.982; 12 (M after 1 s)
  -22.962/-22.960/-22.956/-22.956; 10 (Max S, 20 files) -22.994/-22.993/-22.989/-22.989; 13 (Max M, 20 files)
  -22.995/-22.994/-22.990/-22.989; 11 and 14 (20 successive maxima, -38 to -19): worst deviation +0.011 LU.
- **Tech 3342 cases 1-4:** LRA 10.000/5.000/20.000/15.000 LU at every rate (tolerance +-1 LU).
- **True peak, Tech 3341 cases 15-23, in dBTP, the same at every rate (tolerance +0.2/-0.4):** -6.021, -5.976, -6.316,
  -6.028, +3.029, -0.130, -0.079, -0.204, -0.079. The largest difference from the f64 Annex-2 reference is 9.6e-8 linear
  (the bound is 1e-6). Case 17 (fs/6 at 60 degrees) is 0.084 dB inside its lower limit; that is how a 4x Annex-2
  detector reads it, and the oracle reads the same value.
- **Histogram against exact gating (150 s of stepped noise):** dI +0.0056 / -0.0113 LU and dLRA -0.0070 / +0.0131 LU at
  44.1 / 96 kHz (bounds 0.05 / 0.1).
- **Defaults.** M and S read -120 LUFS (the floor) for silence. The maxima count only windows whose hops all come after
  the reset. A reset replays the held hops at or after its sample. `gated_block_count` counts the blocks that pass
  both gates.
- **Mutants.** Each of the 15 mutants turned its own gate red (implementer's table, in the hand-back). The true-peak
  limiter's full debug suite passes after the `annex2` visibility change.

## Review (S1 attempt 1)
- Root's verifier passed S1. Root accepts D1 (the 10 ms hop), D2 (the 48 kHz stage-2 gain at every rate), D3 (the
  `loudness` dev-profile opt-level) and D4 (the defaults: a silent window reads -120 LUFS; the maxima count only windows
  wholly after the reset; a reset replays the held hops; `gated_block_count` counts the blocks that pass both gates).
- Folded in: a continuity test (a block that does not continue the last one restarts the observer), a test of the
  production percentile rank rule, the histogram ceiling raised to +30 LUFS, the REALTIME_POLICY region extended over
  `State::rest`, and corrections to the research note (a silent window reads -120 LUFS; subnormal states are flushed at the
  hop end). The S2 note about a non-monotone absolute sample is left for S2.

## S2 decision record (root, 2026-10-26; core wiring and the `LOUDNESS` command)
- **Placement.** The render half (`control_plane::loudness`, gate word plus the observer plus a record queue) is a field
  of the session's one `PlanState`, called in `observe_rendered_block`. The control half stays in `SessionState` on
  both hosts, and nothing crosses a plan swap: `PlanState` lives for the whole session and the swap continues the
  clock. While disarmed, render pays one `Acquire` load per block. Records carry their arming's word, so a hop pushed
  during stop-then-start is discarded.
- **Queue.** Depth 512 hop records (5.12 s at 100 records/s). The service step drains it on both hosts (C ABI
  `miso_engine_v1_service` and every session call; the browser worklet services after each message and on a host tick),
  and each read and reset also drains first. A full queue drops the hop and counts it exactly (lossy telemetry,
  AGENTS). `hops-dropped` counts over the session's life, so a reader diffs it. A drop is a gap for the analysis.
- **Acked-batch answer.** No ack can come before a drop. `start` acknowledges arming, never delivery. A read drains
  before it answers and reports only drained hops, together with the exact drop count.
- **Monotone sample.** On both hosts the rendered absolute sample is strictly continuous: `render_contiguous` refuses a
  block off the clock, an adopted plan continues the clock, and observation runs only after a successful block. A
  session seek moves source time, not the render clock, so integration continues across a seek; a host that wants
  otherwise resets. So `LoudnessAnalysis` needs no backward-step case; any non-continuation is already a gap.
- **Ops.** start (a full `LoudnessAnalysis::restart`, then arm), read, reset, stop. A start while armed is refused
  with `VALIDATION_FAILED` `loudness.active`. Every session serves the command (`ProviderFeatures::ALL`), and
  capability bit 17 is set exactly when 0x0014 is listed.
- **Cost.** About 71 KB per session at boot (the analysis 46 KB, the queue 24 KB), charged exactly. The C ABI and
  browser resource ceilings are raised from measurements.
- **Parity.** One host row only: the byte-equal script row in `protocol_cross_host`, which drives both hosts through
  their real entries. A second C ABI row would test the same claim twice, so `capi/src/runtime/tests.rs` gets only the
  forced capability re-pins.
- **Extra paths (beyond the scope note), needed by the spectrum pattern and the gates:**
  - protocol: `message_wire.rs`, `message_wire/loudness.rs` (new), `controller.rs`, and their tests;
  - `control-plane/Cargo.toml`, `control-plane/tests/loudness_monitor.rs` (new), `Cargo.lock`;
  - `fuzz/corpus/complete-schema-manifest.md`;
  - `tools/realtime-policy/src/policy.rs`;
  - `capi/tests/resource_lifecycle.rs`;
  - `hosts/host-web/tests/browser-v1/expected.json`;
  - the S1 crate (`restart()`; `tests/allocation.rs` deleted, because the armed meter joins the cross-host render
    zero-allocation test).

## S2 evidence (attempt 1, on the merged tree dd739e007)
Root accepts the S2 decision record as written: the one cross-host parity row, the 512-deep queue, and the restart on
start.
- **Charge.** `loudness_rows()` charges 75,432 B per session exactly: the queue 256 + 28,728, two `Arc` words 48, and
  the analysis 14,400 + 4 x 8,000.
- **Ceilings (measured; the meter is preallocated in every session):**
  - C ABI `capi_retained_bytes`: measured 340,878 before and 418,934 after, on x86-64. The ceiling goes from 344,192 to
    460,864 (+10 %, rounded up to 64).
  - Browser `bridgeMetadataBytes`: 496,495 before, 573,439 after; the ceiling goes from 546,176 to 630,784.
  - Browser `bridgeRetainedBytes`: 533,603 before, 610,547 after; the ceiling goes from 587,008 to 671,616.
  - `memoryBytesCeiling` stays at 2,097,152. The peak went from 1,900,544 to 1,966,080 (one page), which leaves 2
    pages free.
- **Bytes** (request ID 7, revision `Any`):
  - The start request (64 B) is
    `4d49534f43544c000100000030000101140000001000000007000000000000000000000000000000010000000000000001000101010000000100000000000000`.
    Read, reset and stop are the same bytes with the op byte at offset 56 set to `02`, `03` or `04`.
  - A read reply with every field present (levels -23, LRA 0, `integrating-since` 384, 47 gated blocks; 304 B):
    `4d49534f43544c0001000000300002001400000000010000070000000000000000000000000000001000000000000000010004010800000000ab0300000000000200080101000000010000000000000003000400080000008001000000000000040004010800000000000000000000000500040108000000000000000000000006000401080000002f0000000000000007000600040000000000b8c10000000008000600040000000000b8c10000000009000600040000000000b8c1000000000a0006000400000000000000000000000b000600040000000000b8c1000000000c000600040000000000b8c1000000000d000600040000000000b8c1000000000e000600040000000000b8c1000000000f000600040000000000b8c10000000010000600040000000000b8c100000000`.
  - The corpus hash goes from dea650f7e506833e to de7d7cf504ca191a, with 61 frames. The 59 old frames still hash to
    the old pin.
- **Tests and their test value:**
  - (a) `loudness_monitor::start_read_reset_and_stop_measure_the_master`: a meter not wired to the master, a start from
    the wrong sample or not refused while armed, a reset off the hop boundary, a stop that stays armed, or an arming
    that does not restart the windows or the hop grid.
  - (b) `a_full_record_queue_drops_and_counts_exactly` (640 hops with no service step, exactly 128 dropped): a full
    queue that blocks, panics, overwrites or miscounts.
  - (c) the armed meter in `protocol_cross_host::the_render_call_never_allocates_with_telemetry_configured`, on both
    hosts, with a check that a hop was delivered: an allocation in the armed render half.
    `crates/loudness/tests/allocation.rs` is deleted.
  - (d) the corpus rows `command.loudness` and `response.loudness`: a moved tag, type or field order.
  - (d2) `loudness_reply_rules_hold_both_ways_and_an_unknown_op_is_refused`: a non-finite or below-floor level, a
    negative range, a true peak without `integrating-since`, or an op outside 1-4 crossing the wire.
  - (e) step 20 of the cross-host script: the two hosts answer with different bytes.
  - (f) the bit-17 assertions in the codec and in the controller.
- **Mutants:** 13 of 15 went red. The two that survived:
  - "read does not drain" is equivalent: the service step drains before every command.
  - "the arming tag is ignored" is a known gap: a stale record needs a block in flight on another thread across a stop
    and a start, which the single-thread rig cannot produce.
- **Gates** (the stream coordinator re-ran them all on dd739e007, and the implementer ran them too):
  - cargo test: loudness, protocol, conformance, control-plane `--features test-support`, capi, host-web
    `--features test-support`, plus host-web, protocol and control-plane with CI's feature set (476 passed);
  - fmt, clippy `-D warnings`, the aarch64 `--tests` check and the `-D warnings` release check, the avx2/fma release
    build, rustdoc `-D warnings`;
  - realtime-policy (233 regions in 43 files), check-workspace-policy, `lint-checks.sh --self-tests`;
  - `build-web-audioworklet.sh`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`
    (fresh; self-test 43 red mutations).

## S3 (SDK) decisions and evidence (attempt 1)
- **API.** On both engines: `subscribeLoudness(request?: LoudnessSubscriptionRequest): Promise<LoudnessSubscription>`.
  - The handle has `readLatest()`, `pump()`, `reset()` and `close()`. `close()` sends stop and is idempotent.
  - `onUpdate` is optional.
  - The browser reads on a 100 ms timer (`LOUDNESS_READ_CADENCE_MS`); headless reads on `pump()`.
  - `LoudnessReading` has these fields:
    - `momentary`, `shortTerm`, `integrated` and `range`;
    - `maxMomentary` and `maxShortTerm`;
    - `truePeak` and `truePeakLastHop`, each an L/R pair in dBTP;
    - `integratingSince`, `hopsDropped` and `observedSample` as `bigint`;
    - `armed`.
  - An absent wire field is `undefined`.
  - After `close()`, `readLatest()` keeps the stop's reading.
- **Refusals.**
  - A second open subscription is refused: `MisoEngineError` `invalidArgument` with `diagnosticCode` `loudness.active`.
  - Capability bit 17 clear: `unsupported` (`sdk.asset.capability`).
  - `pump()` or `reset()` on a closed handle: `MisoUsageError`.
- **Paths beyond the scope note:**
  - `crates/protocol/tests/sdk_message_fixture.rs` and `sdk/test/fixtures/protocol-messages.json`, which carry the
    LOUDNESS frames with distinct field values (regenerated by the generator, not edited by hand);
  - one export-type line in each of `sdk/src/{headless,browser}/index.ts`.
- **Tests** (`sdk/test/loudness-evals.mjs`; headless on the real module unless stated):
  1. A -23 LUFS tone, then silence. I, M and S read -23.004; after the silence, M, S, I and the true-peak fields move as
     expected. Red on a wrong op code, a field read from the wrong tag, or wiring that never reaches the core.
  2. A second subscribe: red if the core's `loudness.active` refusal is lost or retyped.
  3. Close: red if close does not stop the meter, if a closed handle still sends reset, or if the stop's reading is
     lost.
  4. The codec against the Rust fixture bytes: red on a field swap (for example the two maxima) or a dropped reply
     rule.
  5. The timer, through a fake scheduler: red if the timer never starts, a tick does not read, or `onUpdate` never
     fires.
  6. The browser/headless parity row in `hosts/host-web/qualification/protocol-calls.mjs`: red if the browser path
     differs.
- **Mutants:** the implementer ran 14 (8 in the first pass, 6 in the fold). Each went red in the test that owns its claim.
- **Gates** (the stream coordinator re-ran them on aafb6833b; artifacts built fresh from this branch):
  - `check-sdk-types`, `check-sdk-generated`, `check-sdk-headless` (677/677), `check-sdk-deletions.py` and
    `sdk-package.sh check`;
  - `cargo test -p protocol --test sdk_message_fixture`, fmt, and clippy on protocol;
  - `lint-checks.sh --self-tests`.
  - The implementer ran the Chromium browser qualification once (`--check-matrix --self-test-mutations --sdk-root
    sdk`): every gate passed, with 0 render allocations.

## S3 attempt 2 (after the review found two MAJOR host-stall defects)
- **Handle states.** A handle is `open`, then `closing`, then `closed`.
  - At most one timed read is in flight. A tick is skipped while that read is pending. A failed timed read goes to
    `onError`, and `observedSample` stops advancing until a read lands.
  - `close()` stops the timer. A read or reset whose turn comes while the handle is closing is not sent; it rejects.
    The stop therefore waits only behind the frame already in flight.
  - If the stop fails, the handle reopens, the timer restarts, and `close()` rejects and can be called again.
  - A start that fails in transport (deadline, abort, lost reply) queues a quiet `stop` on the owner's chain, as the
    spectrum stream does. A typed core refusal sends none.
  - A reply is delivered only if the handle is not closed when it comes back.
- **New evals, and what each catches:**
  - (5) One decoder row per reply rule: a decoder that accepts a reply the Rust codec refuses.
  - (7) A stuck host, from the verifier's `stuck.mjs`: the timer queue grows, close waits behind skipped reads, a read
    is sent after close, or a timed-read failure is lost.
  - (8) A start that lands after the deadline (the frames must be start, stop, start): a lost start that leaves the
    meter armed with no handle.
  - (9) A stop that fails: a dead handle afterwards.
  - (10) Capability bit 17 clear: a dropped capability check.
- **Red evidence.**
  - On attempt 1's `loudness.ts`, evals 7, 8 and 9 are red; close sent about 30 reads after the call.
  - Mutants: bit-17 check dropped (10); each decoder rule dropped or the floor moved (5); in-flight guard removed (7);
    reads allowed while closing (7); the quiet stop after a failed start removed (8); no timer restart after a failed
    stop (9); `onError` never called (7).
  - The closed check before deliver has no eval.
- **Eval-file defect, fixed.** A top-level `await` registered tests late, so a run without artifacts registered only
  the live tests. CI sets the artifacts, so CI was not affected.
- **Gates** (the stream coordinator re-ran them on d4653cc56): `check-sdk-types`, `check-sdk-headless` (682/682),
  `check-sdk-deletions.py`, `sdk-package.sh check` and `lint-checks.sh`. The timing evals passed 12 of 12 concurrent
  runs under load.
