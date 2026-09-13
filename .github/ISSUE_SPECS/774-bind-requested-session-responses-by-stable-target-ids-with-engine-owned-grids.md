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
