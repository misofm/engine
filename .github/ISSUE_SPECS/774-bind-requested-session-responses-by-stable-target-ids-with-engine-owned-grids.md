# Bind requested session responses by stable target IDs with engine-owned grids

Parent: #763. Source dependency: #770's accepted native owner-descriptor/provider contract, including #764 and #767. Start after #770 SOURCE PASS and root's accepted-source checkpoint; substitute its final public spellings where needed, without changing its semantics. This brief is based on current compiler/host sources read-only in `/tmp/miso-engine-770`, not unfinished #770 implementation details.

## Smallest closable outcome and boundary

A native embedding selects named effect slots or a track's input filters from an accepted `EffectPreparedSession`, discovers their response capability, atomically prepares an immutable requested-configuration catalog within caller limits, and queries its bindings into caller buffers on engine-generated linear or logarithmic frequency grids. Replacing the catalog rejects every old binding even when track/rack positions are reused. This is usable while stopped and before the caller transfers the prepared effects into a graph.

The slice is a standalone control/worker-side API in `host-core`, independent of its optional protocol `control-provider` feature. Existing `prepare_host_*`, `PreparedHost`, host resource reports, graph lowering, and render ownership remain unchanged. It does not automatically attach to an active host or its plan-exchange queue. Binding the catalog transaction to production host structural publication remains part of #763's subsequent host/lifecycle integration. No native capability claim may imply that browser or SDK callers can already invoke it.

Astra xhigh approves this half-day scope; Luna max implements and Astra medium gives one adversarial verdict per coherent attempt, at most five attempts. Root owns numbered local/GitHub synchronization, exact-path checkpoints and delivery. Separate owner-preserving composition and generated SDK integration are required successors, not acceptance work hidden inside this child.

## Existing authorities and identity ruling

- `crates/effect-compiler/src/prepare.rs:41` retains `(track_id, EffectRack, effect_id)`, `factory: Arc<dyn NativeEffectFactory>`, and owned `bank_preparation`. Its `request()` reconstructs the exact accepted configuration. Prepare each response through that factory's #770 companion. Never read `entry.processor`, re-resolve a concrete effect from a host list, or rebuild parameter defaults in host-core.
- `EffectPreparedSession` owns the same `CompiledSession`. Resolve tracks and slot membership from `normalized_model()` using `session::StableId`; resolve the prepared entry by its complete stable tuple. Entry order is sorted lexically by slot ID and is **not signal-chain order**. Do not use positional effect-control handles or claim that this catalog orders a composed chain.
- `crates/builtins-compiler/src/lib.rs:4349::track_parameters` is the existing session-to-`BuiltinParameters` projection, including pan/matrix conversion and smoothing validation. Add a small documented public wrapper, `requested_track_builtin_parameters(&session::Track, maximum_smoothing_samples: u32)`, which calls it unchanged. The provider passes `u32::MAX`, as the current host does. Then use #770's builtin owner provider, which validates the entire resulting configuration at the session rate. Do not duplicate this mapping or create a `BuiltinChain`, meters, source rings or render plan merely to query filters. Input-filter scope continues to exclude trim/polarity/alignment delay/fader/mute/matrix.
- `session::StableId` validates canonical IDs; `EffectRack` already models the three racks. An explicit `InputFilters` target keeps fixed builtins distinct from a plugin slot. Reuse these types for requests. `StableId` owns a private `String` and exposes only allocating `parse`/`Clone` plus `as_str`; it has no fallible clone or capacity accessor. Do not add a session API/refactor to work around that. Inside the catalog retain canonical ID text in a small private target representation using `String::try_reserve_exact`, copy the already validated `as_str()` bytes, and charge each actual string capacity. Return borrowed canonical text in `ResponseTargetView` below. This is owned storage for existing IDs, not a new identifier syntax or registry.
- `host-core/src/prepare.rs:839` currently passes `plan_id: 1`; `engine::realtime::PlanEpoch` is assigned by actual publication. Neither is a safe invented catalog generation. This provider owns a checked, nonzero catalog generation. An optional caller-supplied requested `plan_id` is correlation with the intended plan only, never proof of publication/application. It is insufficient to validate a handle by itself.

No compiler/session sealing refactor is included. Inputs are the compiler-produced accepted `EffectPreparedSession`, not untrusted wire objects. Still reject missing/duplicate matching entries and mismatching selected slot/factory effect identity, sample rate, or quantum; do not silently choose a different entry. Continue using #770's authoritative request validator. Creating a general validator that reconstructs every session parameter or authenticates arbitrary caller mutation of public prepared structs is outside this boundary.

## Native API contract

Add `crates/host-core/src/response_provider.rs` and unconditional module exports. Equivalent concise spellings are allowed:

```rust
pub enum ResponseTarget {
    InputFilters { track_id: session::StableId },
    Effect {
        track_id: session::StableId,
        rack: effect_compiler::EffectRack,
        effect_slot_id: session::StableId,
    },
}
pub enum ResponseTargetView<'a> {
    InputFilters { track_id: &'a str },
    Effect {
        track_id: &'a str,
        rack: effect_compiler::EffectRack,
        effect_slot_id: &'a str,
    },
}
pub enum ResponseFrequencyGrid {
    Linear { points: usize, minimum_hz: f32, maximum_hz: f32 },
    Logarithmic { points: usize, minimum_hz: f32, maximum_hz: f32 },
}
pub struct SessionResponseSelection {
    pub target: ResponseTarget,
    pub analysis_id: u32,
    pub grid: ResponseFrequencyGrid,
}
pub struct RequestedResponseIdentity {
    pub configuration_id: u64,
    pub plan_id: Option<u64>,
}
pub struct SessionResponseLimits {
    pub maximum_bindings: usize,
    pub maximum_points_per_binding: usize,
    pub maximum_total_grid_points: usize,
    pub maximum_retained_bytes: usize,
    pub maximum_single_allocation_bytes: usize,
}
pub struct SessionResponseResources {
    pub bindings: usize,
    pub grid_points: usize,
    pub grid_bytes: usize,
    pub metadata_bytes: usize,
    pub owned_id_bytes: usize,
    pub owner_prepared_bytes: usize,
    pub total_retained_bytes: usize,
    pub largest_allocation_bytes: usize,
}
pub fn describe_session_response_target(
    effects: &effect_compiler::EffectPreparedSession,
    target: &ResponseTarget,
) -> Result<ResponseAvailability, SessionResponseError>;

impl PreparedSessionResponseCatalog {
    pub fn prepare(
        effects: &effect_compiler::EffectPreparedSession,
        identity: RequestedResponseIdentity,
        selections: &[SessionResponseSelection],
        limits: SessionResponseLimits,
    ) -> Result<Self, SessionResponseError>;
    pub fn resources(&self) -> SessionResponseResources;
}
impl SessionResponseProvider {
    pub fn new(catalog: PreparedSessionResponseCatalog) -> Self;
    pub fn replace_catalog(
        &mut self,
        candidate: PreparedSessionResponseCatalog,
    ) -> Result<(), SessionResponseError>;
    pub fn bindings(&self) -> impl Iterator<Item = ResponseBindingView<'_>>;
    pub fn resources(&self) -> SessionResponseResources;
    pub fn query_into(
        &self,
        handle: SessionResponseHandle,
        output: effect_contract::ResponseOutput<'_>,
    ) -> Result<SessionResponseSummary, SessionResponseError>;
}
```

`ResponseAvailability` distinguishes available (borrowed/static owner descriptor) from unsupported response provider. Missing track/slot is a typed error, not capability absence. Selecting an unavailable owner or unknown analysis ID refuses the whole candidate. Discovery is a targeted read with no retained cache, provider preparation, curve calculation, or heap allocation; a caller can enumerate its typed session targets if desired. The owner descriptor remains the single authority for units, sections, floor, bypass, result mode and total meaning.

`SessionResponseHandle` has private fields containing the provider's generation and nonzero binding ordinal; both are lossless u64. Its namespace is one provider instance, documented like an in-process object handle. Never use it on a different provider instance or serialize it as a globally unique engine identity. Generation starts at 1 and changes with every successful replacement, including identical configuration/targets. Overflow refuses replacement, preserves the active catalog, and never wraps. Prepare the candidate independently; replacement checks generation first and commits the complete catalog only after success. All candidate/retired catalog drops happen on the calling control/worker thread.

Bindings retain caller selection order solely for enumeration, with explicit target and analysis identity. Reject duplicate targets in one catalog rather than silently merging or choosing a grid. No implicit all-track preparation, shared-job cache, mutable grid, or per-query rebind. The full binding handle identifies its immutable axis for this catalog; no separate mutable grid ID is needed.

`ResponseBindingView` supplies handle, stable target identity as `ResponseTargetView`, analysis ID, owner descriptor, normalized grid definition, immutable borrowed `frequencies_hz: &[f32]`, and #770 configuration view (sample rate, independent enables, applicable bypass). Its target text is the same canonical text that the validated request identified; obtaining a view must not parse or clone `StableId`. These views borrow the provider and cannot outlive or overlap mutable catalog replacement. `SessionResponseSummary` carries the complete handle, catalog's `RequestedResponseIdentity`, and the common owner response summary. The stored configuration ID is forwarded to the owner query exactly; a per-query caller cannot relabel another configuration. Configuration IDs are explicit caller correlation values, not content hashes or guarantees of uniqueness. Full handle plus identity disambiguates each prepared snapshot.

The only semantic mode is `RequestedConfiguration`. No `observed_sample`, `effective_sample`, capture span, target-state revision, `PlanEpoch`, wall clock, or fabricated zero timestamp is present. The retained prepare request describes the accepted initial configuration; subsequent control events, ramps and live render state do not update it. The optional plan ID can describe a candidate that never becomes active. A later requested snapshot requires explicit re-preparation/replacement; applied-target analysis requires the separate render-owned snapshot child.

## Grids, admission and refusal

Generate the axis once off render. Both grid forms require `points >= 2`, finite nonnegative lower bound, `minimum_hz < maximum_hz <= sample_rate / 2`, and exactly one of the four launch sample rates. Logarithmic grids additionally require a strictly positive minimum. Treat signed zero explicitly: normalize a linear lower endpoint of either zero sign to positive zero; logarithmic zero is invalid.

For `t = i / (points - 1)` in f64, linear points use `minimum + (maximum - minimum) * t`; logarithmic points use `exp(log(minimum) + (log(maximum) - log(minimum)) * t)`. Store f32 and set the first/last points to the validated normalized f32 endpoints exactly. Validate the **entire resulting f32 axis** as finite, within the endpoint/Nyquist interval and strictly increasing. Rounded duplicates or any out-of-range intermediate value refuse the grid; do not jitter bins, silently reduce resolution or clamp invalid caller bounds. The returned axis is authoritative; cross-target consumers need not reproduce the transcendental calculation bit-for-bit.

`SessionResponseLimits` has explicit caller-supplied maxima for binding count, points per binding, total grid points, retained catalog heap payload bytes, and single retained-allocation payload bytes. There are no hidden defaults or compiled track limits. Checked arithmetic covers entry/ID storage, point totals, f32 axis bytes and owner-config payloads, including `Layout::array`/representable reservation bounds; overflow is a typed capacity error. Preflight known counts/ID/grid storage before allocation. After charging actual catalog-buffer capacities, pass `min(remaining_retained_bytes, maximum_single_allocation_bytes)` through #770's `ResponsePrepareLimits::maximum_prepared_bytes` before each owner preparation: #770's providers retain one fixed concrete Box payload, so this enforces both bounds without extending that trait. Charge the returned `retained_bytes()` and audit the full candidate before returning it. Use fallible reservation for new catalog vectors/strings; any actual capacity exceeding admission refuses the candidate. Do not retrofit #770's owner allocation semantics or create a global allocation-failure framework.

The resource report names selected binding count, grid points/bytes, catalog metadata/owned-ID bytes, owner prepared payload bytes, total retained heap payload and largest retained allocation. `grid_points` is the selected point count; `grid_bytes` charges actual retained f32 capacities. Metadata includes the binding-vector allocation at its actual capacity, including inline ID/axis/Box handles; owned ID bytes and owner concrete Box payloads are charged separately, once. The total is the checked sum of grid, metadata, owned-ID and owner payload bytes. Largest allocation is the maximum actual retained vector/string/owner allocation, not the sum of a category. Static descriptors and borrowed input/output buffers are not retained allocations. #770's off-render transient validation allowance remains and is outside both retained-payload caps. The cap is the candidate catalog's retained heap payload, not a claim that old-plus-new replacement peak or allocator-internal overhead is zero; those lifetimes are explicit. Store no compiled-session clone, effect factory Arc, request vector, DSP processor, result cache or hidden workspace once prepared.

For a launch-rate input, empty selections produce an empty allocation-free catalog, including under zero limits, and do no owner preparation/curve work. Rate admission still applies to an empty catalog. Query uses its prevalidated stored axis and #770's immutable provider. It performs no allocations, frees, reallocations, hidden preparation, or state mutation from the first call. Validate stale/unknown handles before touching output, then retain the owner's exact shape/capacity/numerical refusal behavior. Every query error leaves all supplied output slices unchanged. Independent L/R totals and optional per-side section buffers retain #770 semantics; channel-selective stream work remains a later parent requirement.

Errors distinguish unknown track/slot, ambiguous/mismatched prepared entry, unsupported owner/analysis, duplicate selection, invalid grid, numeric/byte capacity, reservation failure, stale/unknown handle, generation exhaustion and owner rejection (preserving its stable code). No refusal becomes silence, an empty successful response, a partial catalog or an altered old catalog. This synchronous API has no queue acknowledgement, background scheduling or dropped-command claim.

## Allowed paths and representative gates

Production: new `crates/host-core/src/response_provider.rs`, its exports in `src/lib.rs`, and the one delegate wrapper in `crates/builtins-compiler/src/lib.rs`. Focused tests: `crates/host-core/tests/response_provider.rs`, a small internal generation-overflow test if needed, and the builtin wrapper test beside its existing projection tests. No production Cargo dependency changes, concrete-EQ import in host-core, optional-protocol dependency, unsafe code, owner response math changes, graph/render/host-preparation edits, generated assets, script/workflow edits or new fixture/benchmark framework.

Use existing session fixtures, selectively adjusted through typed models. Required discriminating evidence:

1. Resolve a named EQ in each rack and a track's input filters without concrete-effect knowledge; nonprovider, missing/wrong track/rack/slot, unknown analysis, duplicate selection and malformed selected prepared-entry relationships are distinct refusals. Reorder prepared entries and use slot IDs opposite lexical/declaration order to prove selection is by stable tuple.
2. Catalog curves are bit-identical to #770's direct trait-object providers on the **returned axis**, using asymmetric L/R, bypass/enables, total-only and independently optional section buffers. Cover all four launch rates with compact parameterized cases; no new DSP oracle or fixture corpus is required.
3. Linear DC-to-Nyquist and positive logarithmic axes have exact endpoints and strict f32 order. Check two-point grids, typical grids, nonfinite/reversed/negative/log-zero/above-Nyquist bounds, zero/one points, rounded-duplicate grids and checked-count overflow. Refusals never publish a candidate.
4. Prepare/replace with the same positional rows but changed/reordered stable targets, and separately the same targets with changed requested parameters: every prior handle fails unchanged-output, new handles resolve the correct targets. Failed multi-target preparation/replacement preserves previous results. Private overflow test proves no generation reuse; IDs/configuration/optional plan IDs above 2^53 and u64::MAX survive exactly where valid.
5. Resource caps admit their recorded rows and refuse one unit/byte below; empty/zero limits retain no heap and invoke no owner. Use `bench_support::alloc::current_thread_counters` and its delta for first-call query, borrowed discovery/accessors and stale/shape refusal zero-allocation/free/reallocation evidence. That existing allocator reports requested bytes and operation counts, not live bytes, freed byte sizes or a largest-allocation field: do not invent such measurements or extend the allocator. For a compact builtin-only preparation with no transient owner allocations and no resizing/free before return, compare measured requested bytes with the reported retained total. Substantiate retained/largest rows from actual live vector/string capacities and the fixed owner payloads, and exercise the exact/one-below caps. EQ validator transients must not be mislabeled retained bytes. Mutate/drop caller selections and accepted preparation inputs after catalog preparation; queries remain stable and no borrowed request storage survives.
6. The builtin wrapper returns exactly the existing private projection for pan/matrix/asymmetric input examples and preserves smoothing refusal. Source review confirms no render callsites, mutable processor access or duplicate DSP/default tables. Existing host preparation and realtime gates remain green; there is no audible algorithm change requiring listening or new timings.

Freeze these focused integration test names before implementation: `target_resolution_uses_stable_tuples`, `owner_query_parity_at_launch_rates`, `frequency_grids_validate_before_publication`, `replacement_invalidates_previous_bindings`, `retained_resources_enforce_exact_caps`, and `query_and_discovery_do_not_allocate`. An internal `response_provider` test module covers generation exhaustion and forged unknown ordinals without adding a public handle constructor. The builtin unit test is `requested_track_builtin_parameters_preserves_projection_and_refusals`. Tests may share small helpers in their existing files; no harness expansion is needed.

Proportional commands:

```sh
cargo test --locked -p host-core --test response_provider
cargo test --locked -p host-core --lib response_provider
cargo test --locked -p host-core --features control-provider --test response_provider
cargo test --locked -p builtins-compiler requested_track_builtin_parameters
cargo test --locked -p host-core --test prepare --test fp_environment
cargo check --locked --workspace --all-targets
cargo clippy --locked -p host-core -p builtins-compiler --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
bash scripts/check-host-core-policy.sh
bash scripts/check-builtins-policy.sh
bash scripts/check-realtime-policy.sh
CARGO_TARGET_DIR=target/session-response-wasm-scalar RUSTFLAGS='-C target-feature=-simd128' cargo check --locked --release --target wasm32-unknown-unknown -p host-core
CARGO_TARGET_DIR=target/session-response-wasm-simd RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --release --target wasm32-unknown-unknown -p host-core
```

Wasm checks prove portability/build compatibility only. If source delivery changes shipped bytes, handle artifact qualification separately after source PASS through #766/#768's established discovery/conditional pin/current-consumer workflow; no speculative pin or browser matrix edit in this implementation.

## Explicit successors and delivery

The next composition brief must add an owner-provided unfloored magnitude/complex or accumulation primitive, discover selected EQ/filter membership from the actual session chain order (input filters, SIMD1, dynamic, SIMD2), preserve enable/bypass semantics, multiply only supported linear sections, and floor once after the full product. It returns exact stable membership and calls the result an EQ/filter subtotal in chains containing other processors. Do not add existing floored f32 curves or pretend every nonprovider effect is identity. That is a separate bounded product contract in owner/provider/compiler paths, with a small independent composed oracle and rendered representative witness.

The subsequent host/SDK preview work integrates catalog preparation with actual host candidate-plan ownership, uses real session/engine identity and structural invalidation, generates discovery from the same owner declarations, and supplies browser/headless engine-computed grids and owned stable arrays through an off-worklet query. No second full render engine or client transfer-function math is authorized. This native precursor deliberately does not claim those endpoints or live invalidation are delivered.

All remaining #763 requirements stay mandatory: composition, generated effect/graph capabilities, shared/selective subscription admission and lifecycle, applied immutable snapshots, actual rendered graph taps, worker FFT/smoothing/gaps, bounded SAB/pooled transfer, u64 render-time envelopes, packed additive vectors, joins, and integrated evidence. Root checkpoints each coherent tranche before more work, records exact-source review, completes artifact/required-CI delivery as applicable, synchronizes this child remotely before closure, and leaves #763 open.

## Scope review

Astra xhigh, 2026-09-13: **APPROVED with the concrete storage/accounting amendments incorporated above.** Verified the existing immutable `CompiledSession`, public `EffectPreparedSession`/retained factory requests, stable tuple/order behavior, private `StableId` storage, builtin parameter projection, optional host protocol boundary, source-ID storage pattern, and existing allocator/test seams. The smallest native precursor remains bounded to one host-core module and one compiler delegate. Production host publication and composed curves remain separately required #763 children. No builds, timings, source changes, or active #770 attempt review were performed for this scope verdict; implementation waits for its accepted source dependency and root's numbered issue checkpoint.

## Numbered start boundary

This child starts from frozen provider delivery head `5b8ecc19`, while PR #773 completes required CI. #770's accepted source is `a5db34217ff7f656765121463c7acd8247f65e1b` (Astra medium attempt2PASS), and #772's artifact acceptance is `9d28680d5a1963c0f5fe74547cf168954f7c9570` (attempt1PASS). Their branch remains frozen. #774 is the sole active feature implementation in its own checkout. The amended API/storage/resource directions below and the appended Astra xhigh scope review are binding; use the final accepted #770 public spellings.

## Attempt 1 first source checkpoint

Luna max implemented the initial standalone native catalog, selected stable target resolution, engine-owned grids, bounded storage and caller-buffer queries. `cargo check --locked -p host-core` and all six frozen `cargo test --locked -p host-core --test response_provider` tests passed. Authentic final focused logs are `/tmp/issue774-attempt1-focused-test-6.{stdout,stderr,exit}` (exit 0); prior compile/test development failures remain preserved. Root verified the recorded test output and `git diff --check`, then checkpoints the four allowed source/test paths. Wrapper projection/internal edge evidence and broader required gates remain pending; no Astra medium verdict or completed capability is claimed. Luna pauses at this coherent checkpoint before further edits.

Attempt 1 second coherent checkpoint adds wrapper projection/refusal coverage, internal generation exhaustion and unknown ordinals, unsupported/malformed targets, bypass/optional section parity, small logarithmic grids, input independence and changed-configuration replacement. Focused integration, internal library and wrapper tests pass with actual exit 0 in `/tmp/issue774-attempt1-focused-test-10.*`, `focused-lib-3.*` and `wrapper-test-feature-2.*` under the same prefix. The wrapper unit tests require the existing `test-support` feature; the original featureless command's compiler failure is preserved, and the corrected command is `cargo test --locked -p builtins-compiler --features test-support requested_track_builtin_parameters`. Broader gates and adversarial verdict remain pending.

## Attempt 1 portable-math correction and frozen gate evidence

Astra xhigh's narrow amendment was applied in checkpoint `946dff0f0b881fd47d336d38f597aa8cd9c187cc`: host-core now uses the existing workspace `math` crate for f64 `log`/`exp`, with exactly one host-core-to-math Cargo.lock edge. The logarithmic test independently evaluates a seven-point positive ratio on the returned axis; linear interpolation subtracts endpoints in f64. No owner DSP or other dependency changed. Authentic pre-checkpoint exits are all `0`: fmt `/tmp/issue774-attempt1-math-fmt-check.*`, Clippy `/tmp/issue774-attempt1-math-clippy.*`, focused integration `/tmp/issue774-attempt1-math-focused-test-3.*`, focused library `/tmp/issue774-attempt1-math-focused-lib-5.*`, wrapper with the required feature `/tmp/issue774-attempt1-math-wrapper-test-feature-4.*`, and host-core/builtins/realtime policies under `/tmp/issue774-attempt1-math-{host-core,builtins,realtime}-policy.*`.

The remaining frozen gates also completed with authentic exit `0`: control-provider response integration `/tmp/issue774-attempt1-broad-control-provider.*`; host preparation and floating-point environment tests `/tmp/issue774-attempt1-broad-prepare-fp.*` (13 passed, 1 ignored); workspace all-target check `/tmp/issue774-attempt1-broad-workspace-all-targets.*`; and launch host-core Wasm scalar/SIMD checks `/tmp/issue774-attempt1-broad-wasm-{scalar,simd}.*`. No source edits or retries occurred after checkpoint `946dff0f`; the worktree remained clean during these read-only gates. Astra medium adversarial review remains pending.

## Attempt 1 narrow portable-math scope amendment

Astra xhigh approves adding `math.workspace = true` only to host-core production dependencies and the resulting host-core-to-math Cargo.lock edge. This existing workspace crate is already transitive; no package/version changes are allowed. This amendment supersedes the original no-production-dependency clause only for that edge. Repository Clippy rejects platform f64 log/exp methods. Use the actual portable APIs `math::log` and `math::exp` on f64 endpoints and f64 `t`, cast once to f32, then preserve exact endpoints and complete strict-axis validation. Linear subtraction likewise occurs in f64.

The uncommitted `map_normalized` workaround is rejected for this contract: its f32 fraction and f32 ratio/power arithmetic change the frozen algorithm and can overflow otherwise legal positive endpoint ratios. Passing its tests/lints is not acceptance. No lint bypass, copied solver, owner DSP change or other dependency is allowed. This is a narrow scope/API suitability ruling, not an adversarial implementation verdict. Original failure logs remain preserved.

Attempt 1 portable-math correction honors the approved dependency amendment and frozen f64 grid formula. The only lockfile change is host-core's existing math dependency edge. A seven-point logarithmic interior assertion distinguishes the rejected f32 mapping. Formatting, Clippy, focused integration/internal/wrapper tests and all three policy checks pass (actual exit 0 under `/tmp/issue774-attempt1-math-*`; integration `focused-test-3`, library `focused-lib-5`, wrapper `wrapper-test-feature-4`). Root checkpoints the corrected tranche; broad workspace/host/feature/Wasm gates and adversarial review remain pending.

## Attempt 1 adversarial verdict

# Issue #774 attempt 1 adversarial review

Verdict: **FAIL**.

Reviewed frozen source: `946dff0f0b881fd47d336d38f597aa8cd9c187cc` in `/tmp/miso-engine-774`, against `1cb18a26c3c12a12a4afd133633ab9cd5a44600e`. Reviewer: Astra, medium. Reviewed repository instructions, the complete #774 spec including the approved portable-math amendment, the new provider and all focused/internal/wrapper tests, owner/compiler authorities and allowed diff. No source edits, commits, GitHub actions, agents, expensive reruns or legacy-source inspection were performed. This is one coherent attempt-1 verdict; root authorized final FAIL before remaining broad gates finish because the findings below are conclusive.

## Production/API findings

1. **Target discovery reports supported absence as an error instead of availability.** `crates/host-core/src/response_provider.rs:543` calls `resolve_target(...)?`; the latter returns `UnsupportedResponse` for a native factory without a companion (line 649). Consequently `ResponseAvailability::Unsupported` is never returned. The test at `tests/response_provider.rs:259` pins this contrary behavior. The brief explicitly distinguishes existing-but-unsupported discovery from missing target errors, while selecting that unavailable owner must refuse catalog preparation. Correct the targeted discovery wrapper or resolver representation so a valid nonprovider target returns `Ok(ResponseAvailability::Unsupported)`, keeping unknown/malformed targets as errors and selection preparation as `Err(UnsupportedResponse)`. Add both assertions against the same nonprovider fixture.

2. **Single-allocation admission/reporting incorrectly sums separately allocated IDs.** `src/response_provider.rs:312` compares `target_id_bytes`, the sum of track and effect-slot text lengths, to `maximum_single_allocation_bytes`; lines 361 and 410 apply the same rule to `OwnedTarget::owned_bytes`, which sums two independent String capacities. Largest allocation must use the maximum individual allocation, not this category sum. Two valid 127-byte identifiers contribute 254 owned-ID bytes in total but are two allocations, not one 254-byte allocation. Keep checked summed ID bytes for retained-total accounting, add/use a separate per-allocation maximum for preflight/post-allocation cap checks and `largest_allocation_bytes`, and cover the distinction with a long-ID effect target. Avoid saturating arithmetic for the summed resource category: the frozen contract calls for checked accounting.

3. **Actual reservation capacities are not consistently admitted before owner preparation.** `src/response_provider.rs:365` computes the next owner's remaining budget using `grid_lower` and `lower_owned_id_bytes`, despite having already reserved actual grid/ID storage. The brief requires charging actual capacities before passing the remaining bound to the owner. `generate_grid` (line 784) also assumes `try_reserve_exact` produces exactly requested capacity in a debug assertion, then calls infallible `into_boxed_slice`, which may shrink/reallocate if reservation capacity is larger. Fallible reservation does not promise that equality. Retain the grid Vec with its actual capacity (or another representation with fully justified fallible storage), charge/check actual grid/String capacities, and derive owner admission from charged actual storage plus conservative preflight for remaining selections. If actual capacities exceed limits, return a typed refusal before owner preparation rather than relying on the final aggregate check or a debug panic. Keep the approved f64 portable grid formula unchanged.

## Frozen acceptance evidence still missing

The following are specified cases, not a request for a broader corpus or new harness. Extend the existing six tests/internal test rather than adding qualification machinery.

4. **Resource evidence is currently self-consistency only.** `tests/response_provider.rs:750` checks that the implementation's own reported caps round-trip for a single short-ID builtin selection, plus total/single one-below refusal. It does not compare allocator requested bytes with actual retained total for the specified no-transient builtin preparation, substantiate category/largest rows from live capacities, exercise binding/per-binding-point/total-point limits one below, or measure allocation-free empty preparation. Add the prescribed builtin-only measured preparation, check all resource categories and sum, and exercise each cap. Internal capacity inspection is appropriate for the private representation; do not invent allocator live-byte/freed-size/largest measurements. Use the long-ID case from finding 2 to discriminate maximum allocation from total ID storage.

5. **Stable resolution and catalog replacement tests do not prove several named invariants.** The resolution test never reorders `entries`, never uses opposite lexical/declaration slot order, and only corrupts metadata sample rate. Cover missing/duplicate matching entries and selected factory/identity/request-rate/quantum mismatches distinctly, using accepted fixture modifications rather than a new general authentication framework. Replacement currently switches input filters to EQ and checks one old handle; after changing the same EQ target's configuration it only checks bypass metadata. Add reordered/changed stable targets at reused ordinals, verify every old handle after each replacement (including identical targets/configurations), query the new bindings against their expected owners, and prove a failed multi-target candidate leaves active results unchanged. The overflow unit test replaces one empty catalog with another and only checks binding count; use distinguishable active/candidate identity/content, assert generation remains at max, and verify the original catalog still works after refusal. Mutate/drop original selections and accepted effect preparation inputs after successful preparation and verify retained queries/target text remain stable. Current selection-grid mutation alone does not establish that complete ownership gate.

6. **Parity/grid/atomicity assertions omit required behavior.** `owner_query_parity_at_launch_rates` uses float equality rather than bit equality, and its left-only/right-only calls never assert the selected section arrays at all (lines 487–530). Compare output bits for totals and every selected section, including disabled/bypass outputs and view enable contents, on the returned axes. Grid tests need the specified signed-zero normalization, reversed/nonfinite bounds and linear two-point endpoint evidence; preserve the seven-point f64-log regression and rounded-duplicate/overflow cases. `query_and_discovery_do_not_allocate` measures binding enumeration, not the actual `describe_session_response_target` function. Measure actual discovery and accessors plus the first query for owner bindings; retain and bit-check all supplied L/R total and optional-section buffers on stale/unknown/shape refusals. Current assertions often check only left totals and omit the supplied right buffer. Verify full returned handle and correlation identity, including maximum configuration ID as well as the already tested `2^53+1`/maximum plan ID. The projection wrapper's test compares only one default fixture; add the frozen pan/matrix/asymmetric input cases while retaining smoothing refusal.

## Positive source conclusions

- Effect resolution searches the complete stable track/rack/slot tuple, compares selected session/factory identity and retained request/metadata rate/quantum, and prepares through the retained owner factory/request. It does not access `entry.processor`, invent concrete-EQ defaults or retain the effect session/factory after preparation.
- The builtin wrapper is a direct call to the existing private projection. Builtin preparation delegates to the accepted owner provider; no builtin chain/render plan, source rings or meters are instantiated by this catalog.
- The portable-math amendment is followed: the only dependency/lock changes are the existing host-core-to-math edge; linear subtraction and log/exp generation use f64 and cast once. Endpoints are restored exactly and the complete generated f32 axis is validated for finite range and strict order. The rejected f32 map-normalized workaround is absent.
- Handles have private nonzero u64 generation/ordinal fields. Query checks generation and ordinal before touching outputs and forwards stored configuration identity and immutable axis to the owner. Replacement checks generation before assignment and drops catalogs on the caller's control thread. No render ownership or ordinary host preparation path is altered.
- Metadata views borrow catalog-owned strings, axes and owner state; no query-time ID parse/clone is present. Owner rejection codes are preserved through the common typed error. There is no cache, result workspace, timestamp, applied-plan claim or cross-owner curve composition.

## Gate evidence and limits

Directly inspected final `/tmp/issue774-attempt1-math-*` exit records: corrected focused integration/internal/wrapper tests, formatting, Clippy and host-core/builtin/realtime policies pass. The original featureless wrapper-test failure and rejected math attempts remain separate preserved development evidence; the approved corrected wrapper command uses existing `test-support`. Required remaining control-provider-feature, host preparation/fp, workspace and Wasm gates were running at verdict time; their outcomes are not claimed here. Root will attach those factual outcomes to this same attempt record. Passing them cannot resolve the API/resource defects and omitted assertions above. No reviewer test reruns or runtime Wasm parity claim is made.

Keep corrections within the approved provider/wrapper/test/spec paths and existing math edge; no owner DSP, session-ID API, graph/host publication or allocator framework refactor is justified. Artifact qualification remains separate after SOURCE PASS; #763 stays open.

Root gate completion note: the five remaining control-provider, host prepare/fp, workspace-all-targets and scalar/SIMD Wasm commands subsequently completed with actual exit 0 in `/tmp/issue774-attempt1-broad-*`. This corrects the review-time pending status only; the FAIL verdict and all required corrections stand. Attempt 2 is limited to the six findings above in the approved paths, with no scope expansion.

## Delivery-shape reassessment checkpoint

The user flagged nearly four hours of work without a usable SDK telemetry path. Root paused further planning and implementation to reduce delivery fragmentation. Attempt 2's current source corrections address unsupported discovery, separate ID allocation maxima, actual-capacity admission and retained grid Vec storage. All six focused integration tests pass (`/tmp/issue774-attempt2-focused-checkpoint.*`, exit 0). Remaining review assertions are incomplete; staged helpers produce unused-function warnings. This is a recoverable compiling checkpoint, not SOURCE PASS, not Clippy-clean, and not a completed capability. No additional qualification or feature expansion is authorized until the next usable end-to-end delivery boundary is selected. All original parent requirements remain open.
