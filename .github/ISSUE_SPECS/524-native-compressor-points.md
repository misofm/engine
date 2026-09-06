# Native compressor Point application without advancing samples

Status: bounded queued prerequisite under #140 / audit #349 IO-5, ordered by #518. This brief authorizes no implementation before root completes #478's delivered boundary, assigns the matching numbered issue, and checkpoints its adopted scope. Read-only source basis: delivered main `c34383fcfeea29de855005c15ba89eccab8f45c7`; these effect/contract inputs are unchanged by accepted PR #523. No implementation, tests, timing or Git/GitHub mutation performed for this brief.

## Smallest closable capability and reason for the split

A caller holding one prepared native compressor can apply one checked parameter Point immediately without rendering a sample, then read its actual current and target values. Consecutive calls preserve each ordered target-setting transition, including distinct Points that a later scheduler finds late at the same boundary. This is a reusable native semantic primitive; it does not claim admitted protocol automation reaches PCM or close #140/IO-5.

The source has no such hook today. `PreparedNativeEffect` exposes process/reset/payload/observation (`effect-contract/src/lib.rs:1505`); `EffectProcessBlock::new` rejects zero frames (`:1067`). Compressor span admission (`compressor/src/lib.rs:378`) rejects duplicate same-target entries and noncanonical order within one call. Calling process twice would advance samples; staging only the last target would discard transitions. The endpoint also needs #460 ownership/cancellation, bounded due-record scheduling and an applied-state snapshot boundary. The existing ProtocolController still owns separate queues (`protocol/src/controller.rs:1429`), and SessionControlProvider returns stored initial values (`host-core/src/control_provider.rs:275`). Combining those boundaries with a new native mutation/read API makes a half-day endpoint child unreliable. Deliver this two-crate primitive first; the following opt-in scalar endpoint reuses it and #460 without solving controller/graph/host integration here. This split is about these concrete missing contracts, not additional tooling.

## Frozen additive Rust API

Add these unversioned, object-safe types/methods in `crates/effect-contract/src/lib.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedParameterState {
    pub current_value: f32,
    pub target_value: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParameterAccessError {
    Unsupported,
    InvalidParameterIndex,
    InvalidChannel,
    NotAutomatable,
    InvalidValue,
}

// New methods on PreparedNativeEffect:
fn apply_parameter_point(
    &mut self,
    parameter_index: u32,
    channel: ParameterChannel,
    value: f32,
) -> Result<(), ParameterAccessError>;

fn parameter_state(
    &self,
    parameter_index: u32,
    channel: ParameterChannel,
) -> Result<PreparedParameterState, ParameterAccessError>;
```

Both methods have default bodies returning `Err(ParameterAccessError::Unsupported)` without calling metadata, touching DSP state or inspecting arguments. Unsupported denotes absence of this native access implementation; invalid arguments must never make a default implementation appear capable. Existing implementations compile unchanged. No trait on banks is changed and no capability is inferred for other effects from descriptor `automatable` alone. No blanket process/reset/payload fallback or successful no-op default. This is an internal Rust API addition, not new serialization, wire status codes or a stable ABI representation.

The index is the zero-based descriptor position, never ParameterId, wire handle or processor index. Inputs use actual parameter units; no normalized mapping, clamping or lattice quantization. No timestamp argument or internal clock is introduced: the caller owns scheduling/late counters and supplies exclusive between-slice mutable ownership. `Ok(())` means this individual transition was applied, including a legal stationary retarget. It says nothing about admission, pending records, batch atomicity or acknowledgements.

State returns resident current and target values in those same units. Current is the value reached after the last processed sample (or preparation/reset); target is the last successful target update. Reading advances no sample and changes no state. Neither value is an observed timestamp, pending protocol target, envelope observation or coefficient cache. A scheduler will attach its own coherent sample position later. Do not add a remaining-samples field to this general API; the existing compressor payload independently exposes it for the required tests.

## Exact compressor access semantics

Implement only `PreparedCompressor` in `crates/compressor/src/lib.rs`, through its `Instance<f32>` lane zero. All eight descriptors are readable. Indices 0–6 are writable Points; index 7 (lookahead, ID 8) reads its stored per-channel `lookahead_ms` as equal current/target but returns `NotAutomatable` for apply. The seven writable index/ID/unit/domain rows stay exactly:

| Index / ID | Parameter | Inclusive domain |
| --- | --- | --- |
| 0 / 1 | threshold | -80..0 dB |
| 1 / 2 | ratio | 1..20 |
| 2 / 3 | knee | 0..24 dB |
| 3 / 4 | attack | 0.1..200 ms |
| 4 / 5 | release | 5..5000 ms |
| 5 / 6 | makeup | -24..24 dB |
| 6 / 7 | mix | 0..1 linear |

Use existing `ParameterChannel` (`effect-contract/src/lib.rs:137`), not an invented `ChannelSelection` type. Every compressor parameter is `PerLane`. Left and Right address independent states. **Both returns `InvalidChannel` for both operations, whether the channels are equal or different.** This follows existing `valid_runtime_span` channel-policy checks (`:1163`) and compressor span semantics; there is no hidden merge, arbitrary lane selection, conditional equality response or fan-out mutation. Read Left and Right separately under the same exclusive owner. A future shared parameter can use Both under its own explicitly implemented access contract; this issue delivers none.

Compressor error precedence is fixed: bounds/index check first; channel-policy check second; for apply, automatable check third, finite inclusive domain check fourth. Read has no value/automatable check. Thus index 8 or `u32::MAX` gives `InvalidParameterIndex`; index 7/Left/NaN gives `NotAutomatable`; index 5/Both/NaN gives `InvalidChannel`. All errors are exact no-ops, including silent-fixed-point eligibility. Validation completes before any write. NaN and infinities reject; finite out-of-domain rejects. Existing parameter validation admits domain-valid subnormals. Valid -0.0 is normalized to +0.0 with `effect_runtime::params::normalize_zero`; it is not rejected merely for its sign. Lookahead preparation/payload rules remain unchanged.

Successful apply runs the existing ramp `set_target(normalized_value, 64)` once and invalidates `silent_fixed_point`, matching a valid process Point's eligibility invalidation. It changes no audio/ring cursor, envelope, coefficient cache, staged scratch, observation or unrelated channel/parameter. Current stays bit-identical; target, step and remaining may change. No `next_value`, coefficient redesign, fake process block or snapshot restore is permitted. A small shared per-ramp target helper in compressor lib may serve existing span commit and the new hook; retain span admission/pending/order/capacity/invalid-span report behavior and bank behavior exactly. Do not route the old span entry point through a newly stricter public validation path.

## Frozen ramp and compatibility laws

`effect-runtime/src/ramp.rs:74` already defines the operation: target is stored; a bit-identical finite stationary target (excluding -0.0) settles step/remaining to zero, otherwise step is `(target - current) / 64.0f32` and remaining is 64. Repeated retarget to an old target while current differs restarts 64 samples from current. Two updates with zero samples between leave current fixed and execute both target transitions in call order; each call's readback must expose its target. The first subsequent sample uses current+step, updates 1–63 use iterated f32 addition, and update 64 assigns target exactly and clears step/remaining. An update back to current cancels flight using the existing stationary law. Never replace this with interpolation from elapsed time or final-target coalescing.

Preserve descriptors/CIDs, serialized state bytes/layout/version, preparation metadata, latency/tail, reset laws, link modes, bypass/identity behavior, coefficient equations and PCM arithmetic. The payload intentionally omits step and has documented mid-ramp restore rounding; do not repair or tighten that unrelated behavior. No new prepared owner, queue, heap field, dependency or resource row is needed. Additive methods affect vtables/code, not per-instance state accounting. Existing artifact qualification may observe a changed module hash; any integration follows the observed-artifact process, not a guessed repin in this issue.

## Finite discriminating acceptance gates

Add focused `crates/compressor/tests/native_points.rs`, reusing `tests/support/mod.rs` and existing payload word decoding. Freeze these five exact test names for debug/release evidence:

1. `native_parameter_access_is_typed_and_transactional`: loop eight readable indices and Left/Right with distinct legal initial values. For all seven writable indices, test min/max, one finite below/above, NaN and both infinities; include index 8/u32::MAX, lookahead apply, and Both on unequal and equal channels with the error precedence above. Include valid -0 normalization and one allowed positive subnormal (knee). Readback agrees bitwise with payload current/target. Invalid applies and all reads leave both complete payload sections unchanged; a cloned/reference continuation checks ensuing PCM/state. An omitted-new-method forwarding wrapper around a real prepared compressor proves both defaults return Unsupported and leave the delegated instance untouched, including invalid arguments. No dependency on another production effect is needed.
2. `native_point_changes_target_without_advancing_samples`: nontrivial asymmetric warm state; makeup index 5/ID 6 update. Before rendering, both cursors, envelope/rings, all current words and unrelated parameter/channel words remain exact; only selected target/remaining payload words change. Check resident observation and repeated reads. Verify intermediate readback after two distinct sequential same-target calls at one boundary. No new sample counter: existing cursor/full payload and unchanged histories supply the zero-sample witness.
3. `native_point_retarget_obeys_current_and_sample_count_laws`: independent f32 arithmetic oracle, not production LinearRamp. Observe makeup after 0/1/17/63/64 processed samples, remainder payload and final exact snap. Include an in-flight repeat-target restart, different-target restart and retarget-to-current stationary cancellation; no sample occurs between consecutive hook calls. Verify asymmetric Right remains unchanged. Assertions inspect each transition before the next call; final-only last-wins equality is insufficient evidence of ordered API behavior.
4. `native_point_matches_existing_span_pcm_and_state`: each writable index and each channel uses one legal nondefault target, on identical warmed real compressors. Compare old legal Point-at-first-sample process with hook then same nonempty process with empty spans: reports, both PCM planes bitwise, both complete payloads, resident observations. Use finite asymmetric signal, one actual nonzero lookahead history, and enough samples to cover the 64-sample ramp and expose delayed output. Check at least one makeup case changes PCM versus a no-Point reference so identity silence cannot pass. Existing partition and bank/scalar fixtures continue to protect shared helper behavior; do not create a new cross-product corpus.
5. `native_point_invalidates_silent_fixed_point_without_advancing_it`: first establish silence eligibility using the existing fixture pattern, apply a valid stationary and then moving makeup Point, and compare subsequent actual process/PCM/state with equivalent old-span calls; preserve no-sample state checks. Invalid calls remain true no-ops. Production validation ordering plus this behavioral fixture must preserve eligibility handling without adding public test-only state APIs.

Extend the existing isolated allocation test in `crates/compressor/tests/conformance.rs` with a fixed finite loop of actual successful and rejected hook calls, state reads, and empty-span process on the preallocated scalar instance. Reuse its installed `bench_support::alloc`/engine audit and positive allocation/free controls. Prepare instances/buffers outside the measured scope; require zero allocations and frees and a successful target/readback change to prove the new methods ran. No new allocator, generic conformance runner, mutation campaign or benchmark. Public payload does not serialize every internal cache/step: pair payload assertions with independent ramp arithmetic, next-sample PCM and read-only source review rather than claiming payload alone is full internal-state proof.

## Paths, execution and stop boundary

Allowed implementation paths: `crates/effect-contract/src/lib.rs`, `crates/compressor/src/lib.rs`, new `crates/compressor/tests/native_points.rs`, existing `crates/compressor/tests/conformance.rs`, and only small reused helper additions in `crates/compressor/tests/support/mod.rs` if needed. Numbered spec/evidence are root-owned. No edits to kernel/design/state/ramp sources, descriptors, host/controller/protocol/graph/bank APIs, Cargo dependencies or harness frameworks.

Focused five tests in debug/release, the complete existing compressor suite (including ramps, payload, partition, conformance, silent-fixed-point and bank equivalence) and effect-contract tests, affected strict Clippy, formatting/diff/policy checks, and supported scalar/SIMD Wasm compilation of effect-contract/compressor are proportional gates. Retain recorded commands, actual selected counts and source identities. Root performs the ordinary immutable delivery/required-CI boundary after source PASS; no fresh timing, listening project or expanded target matrix is required for unchanged DSP arithmetic. Coherent compiling/focused-green source pauses for root's exact-path checkpoint before further implementation. Follow the #518 three-attempt review workflow, with one consolidated adversarial verdict per attempt.

Stop and rebrief if delivering this API requires a new retained owner, algorithm or payload change, shared/bank parameter semantics, a dependency expansion or another effect implementation. After closure, queue the actual #460-admitted scalar Point→DSP/PCM endpoint, then controller/graph/host publication, other effects/banks/segments and host rollout. The independent meter issues #516/#519/#520 and #444 builtin integration remain outside this child.

## Root scope adoption and unchanged DSP evidence

Root adopts this bounded Astra brief after #478 / PR #523 delivery at main `70ce3d7b3eb57513390096f84a25e0a675d0bb22`, required PR qualification `34031771524` and post-main `34032117797` SUCCESS, and verified remote #478 closure. The effect-contract/compressor inputs match the inspected `c34383fc` source. This child owns the two-crate native semantic primitive; the actual #460-admitted scalar Point-to-PCM endpoint is its next named product successor, followed by the remaining #140 controller/graph/host/bank/segment obligations. No parent or audit-row closure is authorized by this primitive.

The existing compressor algorithm/coefficients/numerical limits remain governed by `.github/ISSUE_SPECS/013-compressor.md`, its `BRIEFS/013-compressor.md`, qualification #046, and `dsp-research/dynamics.md` / `BIBLIOGRAPHY.md` (primary `[REISS-COMP]`, `[SMITH-SASP]`, `[VST3-LATENCY]`). This change only exposes the frozen 64-sample f32 retarget law; no gain computer, detector, envelope, event-rate coefficient rule or arithmetic reassociation changes. Fixed integer latency remains Fs/50 (20 ms at every launch rate), tail remains Infinite, domains/units and finite/subnormal/signed-zero rules are frozen above, and existing audio sanitation/denormal behavior remains unchanged. Existing static-curve, response/oracle, ramp, payload, partition, link/latency, bank/scalar, realtime and target fixtures are reused. No fresh benchmark or blinded-listening result is claimed or required for an API exposure proven bit-identical to existing Point processing; the original #013/#046 evidence and its stated listening limitations remain intact.

## Numbered/current-base readiness and activation

# Astra numbered/current-base readiness review — #524

**APPROVED for Luna implementation attempt 1 on clean `a9cf7a45bfe2f4a77146114cd658b5646fc8c6c4`, branch `codex/native-compressor-points`, based on current main `70ce3d7b3eb57513390096f84a25e0a675d0bb22`. No scope blocker or gate amendment is required.**

Read-only checks confirm the local numbered spec `.github/ISSUE_SPECS/524-native-compressor-points.md` and OPEN GitHub issue #524 have exactly equal bodies and matching titles. The adopted original brief is unchanged apart from its numbered title; root appended the delivery disposition and unchanged-DSP evidence context. The checkout is clean, current main is the stated ancestor, and effect-contract/compressor plus the relevant existing ramp/domain implementation have no source delta from the inspected `c34383fc` basis. Remote #478 is CLOSED; post-main run `34032117797` completed successfully for `70ce3d7b`. The preceding issue boundary is satisfied.

The brief is a coherent smallest closable native primitive: two object-safe default-unsupported methods, the frozen current/target result and typed error enum, and a compressor-only implementation using its existing ramp transition. It resolves zero-based index versus stable ID, actual units and finite inclusive domain validation, deterministic error precedence, negative-zero normalization, readable fixed lookahead versus nonautomatable writes, and explicit rejection of Both for all compressor per-lane reads/writes. No ambiguous L/R merging or hidden fan-out remains.

The zero-sample law and existing stationary/restart/64th-sample snap behavior are frozen. The five finite fixtures, reused installed allocator/liveness, old legal-span comparison, independent arithmetic/payload/readback observations and continuation PCM checks discriminate those claims. Payload-only evidence is explicitly insufficient for omitted internal caches/step. Silent-fixed-point invalidation remains a source and behavior requirement; no instrumentation expansion or repeated mutation campaign is authorized. Existing descriptor, state, numerical/latency/reset and bank/scalar contracts remain load-bearing.

Luna may implement only the listed contract/compressor source and focused test paths. No kernel/design/state/ramp changes, dependency or retained-owner additions, bank API, host/controller/graph/protocol rollout, artifact/resource guesses or benchmark work are included. The unchanged-DSP appendix correctly reuses existing source research and fixtures without inventing fresh listening or performance evidence. The actual #460-admitted Point-to-PCM endpoint remains the named next product successor; this prerequisite cannot close #140, IO-5 or #518.

Retain all specified focused debug/release, affected suite/Clippy/policy and supported-target gates. Pause on a coherent compiling/focused-green tranche for root's exact-path checkpoint before further implementation. Then freeze source/evidence for one consolidated Astra adversarial verdict. Under #518, this is Luna attempt 1; Sol attempts 2/3 follow only a consolidated FAIL, with the hard three-attempt stop unchanged. Root owns immutable delivery, required qualification and remote synchronization after source PASS.

No implementation, source edits, builds, tests, mutations or timing were performed for this readiness review.

Root activates Luna attempt 1 under this exact approved scope after checkpoint/push and GitHub synchronization. The native Point/readback primitive is the sole active implementation issue. #478 is delivered and closed with post-main SUCCESS; #516 remains separately owned.
