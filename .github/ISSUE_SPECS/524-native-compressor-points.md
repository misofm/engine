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

## Luna attempt 1 source checkpoint

The two default-unsupported native methods and compressor-only implementation are present in the approved paths. The five focused native Point tests pass in debug and release (5 selected in each), and the existing isolated allocation gate passes. Formatted source identities match the recorded commands in `artifacts/issue524-luna-attempt1`; earlier pre-format captures are preserved. Source is checkpointed before the remaining proportional suite, lint, policy and target gates and one consolidated Astra review. No review PASS or parent/audit closure is claimed.

## Consolidated Luna attempt 1 verdict and Sol attempt 2 scope

# Astra consolidated adversarial verdict — #524 Luna attempt 1

**FAIL for source acceptance at `fb6d2d24617412ba5b946a04a9a07f3c524456a4`. The implementation is narrowly scoped and its inspected transition logic is consistent with the approved design, but the five named tests omit substantial frozen assertions. Passing those names and the existing suites does not satisfy #524. Proceed to one bounded Sol attempt 2; do not weaken the gates.**

Reviewed the authoritative numbered spec, source diff and actual assertions, initial tracked evidence, final `/tmp/issue524-luna1` command/stdout/stderr/status captures and root's identical delivery package/README. Source hashes still match the checkpoint; only root evidence packaging was added during review. No reviewer source edits, tests, builds, mutations or timing were performed.

## What is already correct and credited

The exact additive Rust types and object-safe default-unsupported methods are present. The defaults inspect no metadata or state. Compressor bounds checks precede channel checks, then automatable and domain checks; all rejection paths precede writes. Left/Right map to their own scalar channel; Both rejects. Readable lookahead returns its stored value twice, while applying it rejects. Successful apply normalizes zero, invalidates the silent eligibility flag and invokes existing `set_target(..., 64)` once. Readback copies resident ramp values. No sample processing, coefficient update, new owner/dependency, bank API or out-of-scope source changes were introduced. Existing span processing and DSP/state/descriptor files remain unchanged. A shared target helper was optional; its absence is not a finding.

Raw evidence supports five focused passes in each profile; compressor suites total 75 passing entries across 19 result blocks in each profile, effect-contract 40 across five in each; the existing isolated allocation test passes, with its installed positive allocation/free controls retained. Strict affected Clippy, six policies, fmt/diff, scalar Wasm release build and SIMD Wasm check pass on recorded source identities. The original scalar capture tried to execute an environment assignment as argv[0], before Cargo launched; its absent workload status is a setup failure, preserved and resolved by the successful `env` invocation. It is neither a semantic failure nor successful target evidence. Pending broader immutable delivery is not a source-phase failure.

## Mandatory assertion and contract gaps

1. **Typed access/transactionality is mostly untested** (`compressor/tests/native_points.rs:25–65`). The index loop checks only `is_ok()` on equal default channels. It does not compare all eight resident values with independently initialized asymmetric values and payload words. Missing are the seven-parameter min/max/outside/nonfinite cases, u32::MAX, invalid state reads, Both read/write on unequal and equal channels, the prescribed combined-error precedence cases, readable fixed lookahead value, allowed positive subnormal, and default-method forwarding wrapper. Existing four error results and makeup -0 normalization receive credit. There is no before/after payload or continuation check for any rejected call or read, so the test's name does not establish transactionality.

2. **The no-sample witness does not identify preserved state** (`:69–92`). It uses a fresh symmetric preparation, checks one current value after the first update and merely asserts the snapshot changed. It never warms nontrivial asymmetric history, checks the changed-word whitelist, cursors/envelope/rings/unrelated words, resident observation, repeated reads, or unchanged current/history after the second ordered update. A hook that additionally advanced hidden DSP history could satisfy these assertions. Intermediate target reads after both updates are useful but insufficient alone.

3. **The sample-count and retarget laws stop after sample one** (`:96–131`). There are no 17/63/64-sample assertions, remaining-word checks, exact final snap, repeated-old-target restart or different-target restart while in flight. The stationary-cancel case checks only immediate current equality, not target, remaining=0 or subsequent stationary evolution. Right's default target alone is not the prescribed asymmetric unchanged-side witness. The arithmetic used for the first sample is independent and credited; it does not prove the rest of the frozen law.

4. **The old-span comparison has insufficient live PCM coverage** (`:135–162`). It covers only makeup/Left, from fresh instances for one 128-frame block. The prepared 48 kHz compressor has fixed latency 960 samples and empty initial delay history; this comparison does not establish a rendered nonzero-history response. The existing silent fixture itself documents why a short fresh render compares only silence. There is no no-Point control proving the update changes PCM, no warm history, no other writable index/Right case, and no report or resident-observation comparison. `assert_eq!` on f32 vectors also treats +0 and -0 as equal, while the brief requires PCM bit equality. Both complete payloads are compared and are useful, but cannot substitute for these missing gates.

5. **Silent eligibility behavior is not exercised after the hook** (`:166–180`). One initial silent process is followed by a moving target, snapshot inequality and one error result. There is no stationary update, subsequent real process, old-span comparator, PCM/state/observation continuation, or rejection no-op comparison. Snapshot inequality says only that something changed; the snapshot does not contain `silent_fixed_point`. Source visibly clears the flag correctly, but the required representative behavioral fixture is absent. Do not invent a public flag probe or demand that an optimization's observationally equivalent execution paths produce different PCM; source review plus the specified continuation fixture is the frozen proportional proof.

6. **Repeated hook allocation qualification is absent** (`compressor/tests/conformance.rs:152–177`). The new success, rejected apply and reads execute once, before the existing 32-render loop. This proves a single measured call sequence, not the required finite repeated hook/read/process sequence. Keep positive controls and original render-path coverage while exercising those new operations repeatedly with successful target/readback liveness and zero allocation/free results.

7. **The exposed API lacks its usable semantic documentation** (`effect-contract/src/lib.rs:1505–1519,1551–1569`). The one-line comments omit zero-based descriptor index versus ID, units, current-versus-target timing, unsupported capability semantics, rejection no-op behavior, channel policy, caller scheduling/ownership, and the fact that success is application of one target transition rather than protocol admission/acknowledgement. Add concise rustdoc for the frozen contract and error/field meanings; keep compressor-specific automatable range/error precedence/Both policy documented at its implementation/module seam. This is an API requirement, not a request to pin prose bytes.

The arithmetic hook also inherits the existing raw processor/ramp FP environment assumption: `lane::CanonicalFpEnv` pins round-to-nearest with gradual underflow at native render entry (`lane/src/fpenv.rs`), and `effect-runtime/src/ramp.rs` explicitly relies on it for subnormal behavior. No guard is established by this raw trait hook. Document that direct native callers must use the same canonical environment around parameter application and processing; the future endpoint must include no-sample catch-up within that scope. Use the existing guard for focused subnormal/arithmetic tests as appropriate. This is a proportional clarification of inherited conditions, not a demonstrated new DSP failure or authority to redesign host FP entry points, add per-Point guards or launch a hostile-environment test campaign.

## Frozen bounded Sol attempt 2 correction brief

Retain the approved API, implementation scope, all five exact test names and every original acceptance gate. Correct the five existing fixtures to supply the cases above, using fixed finite tables and the existing support/payload decoder. Use identically prepared and identically warmed reference instances; do not use a mid-ramp snapshot/restore clone as an exact oracle, since the inherited payload deliberately omits step. Observe each consecutive no-sample update before the next; compare allowed payload changes and unchanged histories explicitly. Use independent iterated f32 arithmetic and remaining-word checks at the prescribed sample counts, then real subsequent PCM/state to cover internal words that payload omits.

The span comparator must cover all seven writable indices and each channel, warm nonzero asymmetric delay history, compare reports/observations and both PCM planes as bits, and retain one explicit makeup/no-Point PCM inequality. Keep this finite representative set; no new width/rate/host corpus or timing is needed. Complete the silent-history stationary/moving/rejected continuation within existing fixtures and source review. Extend the installed allocation fixture with a fixed repeated new-operation sequence, preparing any test-only instance/buffers outside the measured scope and preserving the existing named render cases.

Add concise Rust API/compressor documentation for the existing frozen semantics and canonical native FP precondition. Production algorithm/layout/API redesign is unnecessary on the evidence reviewed. Preserve all attempt-1 evidence and candidly label its assertion coverage. Allowed source/test paths remain exactly the original brief's list; kernel/design/state/ramp, dependencies, other effects, bank/controller/graph/host/protocol changes and resource/artifact guesses remain forbidden.

After the coherent corrected tranche compiles and the five focused debug gates pass, pause for root's exact-path checkpoint. Finish the prescribed debug/release full compressor/effect-contract, allocation, strict Clippy, policy/fmt/diff and scalar/SIMD target checks with exact command/source attribution. Freeze one complete evidence package for one consolidated Astra attempt-2 verdict. No mutation or benchmark campaign is required or authorized. This consumes Luna attempt 1 only; Sol has attempts 2 and 3 under #518's unchanged hard stop. #524 and its parent obligations remain open.

Root adopts this bounded correction brief. Sol attempt 2 may begin only after this evidence/review checkpoint is pushed and synchronized. No production redesign or acceptance relaxation is authorized.

## Sol attempt 2 focused checkpoint

Sol corrected the five named fixtures, repeated allocation sequence and Rust API documentation in the four approved paths. The focused debug binary passes 5/5 tests on recorded source identities. An initial compile-only error from treating the existing FP guard as a Result was corrected; its status 101 and the restored status 0 are preserved in `artifacts/issue524-sol-attempt2`. No production transition logic, DSP algorithm, layout, descriptor, state payload or resource expectation changed. Root checkpoints this coherent tranche before the remaining proportional gates and one consolidated Astra review; source acceptance remains pending.

### Sol attempt 2 proportional-gate correction

The full compressor debug suite exposed an added allocation-fixture call that incorrectly constructed a zero-frame process block. Existing `ZeroFrames` rejection is correct and unchanged. Sol removed only that redundant call; the required repeated Point/read/rejected-call sequence remains followed by a real 128-frame process with empty automation spans. The isolated allocation gate now passes. Both the failed suite capture and corrected exact allocation result are preserved. Focused release also passes 5/5 on the preceding source, whose native Point fixtures are unchanged. Root checkpoints this test-only correction before remaining gates; no review verdict has yet been issued for attempt 2.

### Sol attempt 2 strict-lint checkpoint

Full compressor and effect-contract suites pass in debug/release on `3b9a5fb8`; isolated allocation gates pass in both profiles. Strict Clippy found only twelve unequal hexadecimal byte-group spellings in test noise seeds. Their numeric values are unchanged by the grouping-only correction, and strict affected Clippy now passes. Original failure and corrected pass are preserved. No behavioral code or assertions changed; policies, target checks and consolidated review remain pending.

## Consolidated Sol attempt 2 verdict and final attempt scope

# Astra consolidated adversarial verdict — #524 Sol attempt 2

**FAIL for source acceptance at `7190a6a3ae6222f97a284237287c640e99ea1c07`, limited to an incomplete frozen typed-access assertion table. The production implementation and the other corrected semantic gates are acceptable on the inspected source/evidence. Sol attempt 3 needs only the bounded test correction below; no production/API/architecture revision is indicated.**

Reviewed the approved numbered spec and adopted Luna FAIL, actual implementation and every focused fixture, final `/tmp/issue524-sol2/sol2-report.md`, raw command/status/stdout/stderr evidence and root's copied package. The production transition body is unchanged from Luna. Relative to `d384d363`, `3b9a5fb8` only removed the redundant zero-frame allocation-test call; `7190a6a3` only regroups twelve hexadecimal seeds with identical numeric values. No reviewer source edits, tests, builds, mutations or timing were performed.

## Credited corrections

The API/compressor rustdoc now explains descriptor index versus ID, declared units, resident current versus target, unsupported defaults and state-preserving rejection, caller scheduling/ownership, Point application versus admission/acknowledgement, channel policy/error order and the inherited canonical native FP environment. Focused arithmetic/subnormal fixtures establish that existing environment. No new host guard or DSP arithmetic is needed.

The domain table now covers all seven writable parameters' finite boundaries, below/above values and nonfinite rejection; readback covers eight asymmetric Left/Right states against payload bits, including fixed lookahead. Negative zero and a domain-valid positive subnormal are covered. Rejected calls retain payloads and match an independently prepared continuation. The defaults have a real forwarding wrapper, with the coverage limitation below.

The no-sample fixture uses equally warmed asymmetric processors, a whitelist allowing only selected target/remaining payload words to change, both ordered intermediate target reads, preserved current/history/observations and subsequent old-span PCM/report/state equivalence. This closes the former snapshot-inequality-only gap.

The ramp fixture checks an independent iterated f32 oracle and remaining words at 0/1/17/63/64 samples, exact completion, in-flight same/different-target restarts and stationary cancellation with subsequent evolution and asymmetric Right preservation. The old-span comparator covers all seven writable parameters on both channels, establishes nonzero delay history, compares PCM as bits and reports/payloads/observations, and proves makeup changes actual PCM against a no-Point reference.

The silent fixture now compares stationary, moving and rejected-hook continuations against old-span/reference processing. Source review supplies the private eligibility-flag evidence; the fixture does not falsely claim the payload exposes that flag. Repeated allocation qualification executes successful/rejected calls, readback and actual 128-frame empty-span process 32 times with installed allocation/free liveness and zero render counters. The redundant zero-frame call was correctly removed rather than changing EffectProcessBlock's contract.

## Remaining frozen table coverage

`crates/compressor/tests/native_points.rs:227–377`, `native_parameter_access_is_typed_and_transactional`, still omits the following small cases from the already frozen access contract:

1. **Both write on equal channels is absent.** The asymmetric object receives Both read and apply rejections at lines 258–270. The equal-channel object at lines 331–337 is immutable and receives only a read rejection. The adopted Luna FAIL explicitly requires “Both read/write on unequal and equal channels.” A write path that permits Both only when channels are equal would pass the current table, although it violates the frozen API. Add the equal-channel apply rejection with payload and unchanged-reference continuation. This is a specific missing mandatory assertion, not a request for another channel matrix.
2. **Default Unsupported is exercised only with malformed arguments.** The forwarding wrapper at lines 362–376 receives `(u32::MAX, Both, NaN)` and `(u32::MAX, Both)`. Add a well-formed readable/automatable request, such as index 5 / Left / a legal nondefault value, to both default methods, preserving the same snapshot/continuation witness. The frozen requirement is that absent implementation is Unsupported, including invalid arguments; it must also remain Unsupported for the request that would succeed on the wrapped compressor. This completes that existing capability gate.
3. **The top two error-precedence boundaries are not discriminated.** Current invalid-index cases use an otherwise valid channel, and the Both+NaN apply uses automatable index 5. Complete the existing combined-error table with invalid index+Both (read and apply must give InvalidParameterIndex) and index 7+Both+NaN (apply must give InvalidChannel). Existing index 7+Left+NaN and valid-index+Left+NaN already cover the lower boundaries. These few tuples establish the declared index → channel → automatable → domain order; no Cartesian invalid-input corpus is needed.

The production body visibly implements all three requirements correctly. This FAIL is an assertion-completeness finding against the frozen brief; it is not a reproduced DSP error, a missing target build or a proposal to modify the API.

## Evidence disposition and bounded Sol attempt 3

Final valid captures pass: five focused tests debug/release; compressor 75 passing entries across 19 result blocks in each profile; effect-contract 40 across five in each; isolated allocation gate debug/release; strict affected Clippy; six policies; scalar Wasm release build and SIMD Wasm check; fmt/diff. Source identities and numeric-only seed drift are attributed accurately; repeating every suite solely for regrouped literals is unnecessary.

Four retained failures are resolved historical evidence: FP guard API misuse failed compilation; the zero-frame test failed as the existing constructor requires; seed spelling failed Clippy; scalar flags invocation failed setup. They do not cause this verdict and must remain preserved. Broad immutable delivery remains root's post-source-PASS phase.

For attempt 3, change only the typed-access fixture in `native_points.rs` to add the listed finite calls using its existing helpers and independently prepared references. Keep all five exact test names, all other assertions and the production/docs/harness paths unchanged. Preserve all prior evidence; append a concise correction report with exact source identity. Run the updated five focused tests debug/release and affected strict Clippy/fmt/diff. Reuse already passing unchanged production, other suite and target evidence with explicit source attribution; broaden reruns only for actual drift or a failure. Pause for root's coherent exact-path checkpoint, then freeze one complete package for the final consolidated Astra verdict.

No mutation, benchmark, new helper framework, resource/artifact repin, domain expansion or endpoint integration is authorized by this correction. This is the third and final implementation attempt under #518; if it fails, preserve evidence and apply the hard stop/rebrief rule, with no disguised fourth retry. #524 remains open pending acceptance and delivery.

Root adopts this test-table-only Sol attempt 3 correction. This is the final allowed attempt; no production change or gate relaxation is authorized.

## Sol attempt 3 focused checkpoint

Only the typed-access fixture changed: equal-channel Both write rejection with reference continuation, well-formed apply/read calls through the default-Unsupported wrapper, and the two missing precedence boundaries. The five focused debug tests pass on recorded source identities. Production, documentation, other fixtures and all prior evidence remain unchanged. Root checkpoints this final test-table correction before release/Clippy/fmt/diff and the final consolidated verdict.

## Final source acceptance

# Astra final consolidated source verdict — #524 Sol attempt 3

**PASS for source acceptance at `d8c14721bbde58646ab837b0901f79bc8e08799f`. All frozen source-phase gates are satisfied. No further implementation attempt or correction brief is required. Ordinary immutable delivery qualification, actual-PR/current-base review, required CI and remote synchronization remain root's delivery work.**

Reviewed the authoritative numbered #524 brief, the adopted Luna and Sol2 verdicts, the actual final diff/assertions, `/tmp/issue524-sol3/sol3-report.md` and raw final captures, with explicit reuse of the previously reviewed unchanged Sol2 source/suite/target evidence. No reviewer source edits, builds, tests, mutations or timing were performed.

## Final correction

The sole source change from accepted-in-part Sol2 is the frozen typed-access fixture. Its combined invalid-index/Both read and apply requests now establish index precedence, and lookahead/Both/NaN establishes channel precedence over automatable/domain checks. These join the existing lower precedence cases.

The equal-channel compressor now receives a legal nondefault Both write and rejects it with `InvalidChannel`. Existing helpers check both complete payload sections; an independently prepared reference supplies the ensuing PCM/report/payload/observation continuation. This closes the missing equal-channel write case without enabling fan-out or hidden channel merging.

The forwarding wrapper's inherited defaults now receive well-formed index-5/Left apply and state requests as well as the retained malformed arguments. Both return `Unsupported` under the same snapshot and independent continuation witness. The full frozen capability/default table is covered. All other assertions, production/documentation files, support helpers and the allocation fixture are unchanged.

## Consolidated contract acceptance

The additive object-safe API has the exact frozen types and default behavior. Compressor validation is bounded and complete before writes, with the prescribed index/channel/automatable/domain precedence. All eight indices read truthful resident values, lookahead remains fixed/nonautomatable, Left/Right are independent and Both rejects. Values use declared units/domains; nonfinite/out-of-domain rejection and valid signed-zero/subnormal rules preserve the existing implementation.

Successful application performs the existing target-setting operation once and invalidates silent eligibility without advancing a DSP sample. Current-versus-target and consecutive no-sample transitions, unaffected state/history/observations, 64-sample iterated ramp/snap and restart/stationary laws are now covered by the finite corrected fixtures. Actual old-span comparison covers all seven writable parameters and both channels with warmed nonzero history, bitwise PCM, complete payloads, reports and observations, plus an independent no-Point PCM difference. Silent stationary/moving/rejected continuations and source review preserve the private eligibility rule without treating payload as a witness for an unserialized flag.

Repeated real hook/read/rejection/process calls retain the existing installed allocator's positive liveness and zero allocation/free results. No prepared owner, queue, dependency, state-layout/descriptor/CID/resource row, DSP equation, coefficient arithmetic, reset/latency/tail or bank API changed. The API documentation records its scheduling/ownership boundaries and inherited canonical native FP precondition. No host/controller/graph/protocol capability is implied by this native primitive.

## Evidence and delivery boundary

Final debug/release captures each select and pass all five focused tests; strict compressor/effect-contract all-target Clippy, fmt and diff checks pass. The debug capture accurately identifies its sole dirty test file before checkpointing unchanged; release and final checks identify clean `d8c14721`. Independently read final file hashes match the report.

As expressly authorized in Sol2's bounded correction ruling, unchanged evidence remains valid under its original identities: complete compressor suites with 75 passing entries across 19 result blocks per profile, effect-contract 40 across five per profile, isolated repeated allocation gate debug/release, six policies, scalar Wasm release build and SIMD Wasm check. The only changed test binary is directly covered in both profiles now. No suite is relabeled as a later-head execution, and no redundant source-unchanged target run is required to manufacture a new date. Historical failed setup/test/lint captures remain preserved with their actual resolutions.

Root may now freeze and run the already-required immutable delivery checks, then obtain actual-PR/current-base acceptance and required qualification success before merging and synchronizing #524 closure/post-main evidence. A future observed artifact mismatch requires its bounded observed integration; this source PASS is not a guessed-pin authorization. This is acceptance within the third and final attempt, not permission for a fourth retry. #140, #518 and audit IO-5 remain open for the promised real admitted Point-to-PCM endpoint and subsequent rollout obligations.

Root adopts source PASS and begins immutable delivery. This is source acceptance only; actual PR/current-base review, required qualification and remote closure remain mandatory.

## Immutable delivery and observed artifact integration

On clean accepted `d7b67839`, `cargo test --locked --workspace` passed 1,669 tests, zero failures, 24 ignored across 278 result blocks. Native C ABI and broader scalar/SIMD Wasm checks passed. Raw commands/status/stderr and losslessly compressed workspace stdout are in `artifacts/issue524-delivery`. An initial artifact preflight rejected an absent output directory (status 2); after creating it, the normal builder compiled successfully and rejected only the old pin (status 1).

The observed artifact SHA-256 is `d77d7558105c29d38751ab26c330689da829fbe0f011a6ffcc7edf55d583bc87`, replacing `e3f47856ad917edf2cd99cdc6669a4f3e6c6a9074b15c560a8337bf842187fef`. Astra's bounded integration ruling is preserved beside the captures. After all immutable commands finished, root updated only that observed pin. No resource reservation, PCM oracle, DSP layout or algorithm changed. Ordinary rebuild, independent hashing and existing static/resource/browser consumers remain required before PR delivery.

## Delivery checkpoint and user-requested pause

The ordinary artifact rebuild and independent identity check pass: SHA-256 `d77d7558105c29d38751ab26c330689da829fbe0f011a6ffcc7edf55d583bc87`, 2,698,540 bytes. Existing static/object, resource/native PCM parity (including 26 negative controls) and worklet isolation gates pass. Chromium 151.0.7922.34, Firefox 153.0 and WebKit 26.5 qualification passes with self-test mutations on candidate `63c2fbb25db21d61ed16b7f56f7646b9c78f17c0`; recorded matrix generation/check passes. Only candidate/artifact identities changed in results/matrix. The abbreviated-SHA browser preflight failure and corrected full-SHA success are both preserved.

The user requested a pause to restart outside tmux. Root checkpoints/pushes this delivery evidence and leaves no active commands. #524 remains OPEN: no PR has yet been opened, and actual-PR/current-base Astra review plus required qualification must precede merge/closure. Resume at that delivery boundary, not implementation attempt 4. Source PASS, all three attempts, failures and exact command evidence are already preserved. #518 carries the self-contained restart comment and remaining audit continuation.

## Resumed delivery: current-main integration

Root resumed from `923c860a` and fetched current main `be781895` (merged metering PR #521). All production source merges without conflict; the three conflicts are browser pin/results/matrix identities. Preserve current main's three identity files as the integration starting point, then run the ordinary builder to observe the combined artifact before any bounded pin update. Earlier source PASS and raw evidence remain attributed to their original candidates. Combined-source workspace/browser qualification and final actual-head/current-base Astra review remain pending; this merge checkpoint is recoverable integration, not delivery PASS or a fourth implementation attempt.

### Combined-source immutable results and observed pin

On clean `e4fea349`, every compiled workspace test binary passed. The initial combined command has 1,668 passes, zero assertion failures, 25 ignored tests and 241 completed result blocks, then exits 1 at capi doctest setup with E0463 host_core. A serial full workspace doctest recovery on identical source passes all 40 targets and 14 tests. Shared build artifact interference remains a hypothesis; original failure is preserved, not relabeled PASS. Astra approved this bounded recovery. Focused tests pass 5/5 in debug/release; native ABI, scalar/SIMD targets and worklet checks pass.

Astra reviewed the successful ordinary compilation and observed combined artifact mismatch, approving only pin `04f938f667180d0e7b972e1cc2236af3a5d5e3f7b73d1ed503dde72e83aa9bce` after immutable commands finished. The ordinary rebuild and browser/static/resource consumers remain pending. Existing metering resource/PCM fixtures and all accepted #524 source remain unchanged. Captures and rulings are preserved in `artifacts/issue524-delivery/resumed-*`.
