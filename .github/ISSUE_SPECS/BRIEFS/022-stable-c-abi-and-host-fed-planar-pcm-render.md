# Sol implementation brief — issue 022 stable C ABI and host-fed planar PCM render

## Decision and attempt budget

**FINAL SOL PASS AT `8190baf`.** Terra attempt 1 and the sole bounded Sol correction are consumed.
The one authorized non-timed 100,000-call C-entry audit passed with every forbidden-operation
counter and render error at zero. Benchmark/timed invocation count is zero. Native file runner,
target breadth, and broader ABI qualification remain open Issue 073 scope and are not claimed here.

## Product seam

Implement only in `miso-engine-capi` plus a checked-in header and focused C smoke fixture. Reuse the
accepted session compiler, launch effect registry, builtin compiler, graph compiler, host
`PcmSourceRing`/`prepare_graph_source_set`, and scalar prepared render plan. Do not add a second graph,
source, protocol, scheduler, effect, or memory abstraction.

One narrow accepted-seam completion is required because the sealed builtin wrapper currently offers
only `into_bound`, while the underlying graph already offers transactional `bind_with_source_set`.
Add `PreparedBuiltinsGraphArtifact::into_bound_with_source_set` and an ownership-returning failure
type. It must apply the identical builtin/observer prevalidation as `into_bound`, append only the
genuine private bindings, delegate to the existing graph source-set bind, and return artifact,
external bindings and source set on every rejection. Do not expose or clone sealed parts.

One equally narrow source visibility correction is required. Add only:

```rust
pub fn prepare_host_region(
    config: PcmSourceRingConfig,
    initial_frame: SourceFrame,
) -> Result<(PcmSourceProducer, PcmSourceConsumer, SourceResourceReport), PcmSourceRingError>
```

as an associated function on `PcmSourceRing`. Its body delegates directly to the existing
`prepare_at_source_frame(config, initial_frame)` path; keep that shared implementation private and
leave zero-origin `prepare` unchanged. Tests cover origins 0, 1 and a representative nonzero session
start; identical shape/resource reports; first accepted absolute chunk; wrong-zero/noncontiguous
rejection without prefix; a strictly newer nonzero seek; and producer/consumer ownership after
rejection. Do not expose native workers, decoder providers, ring internals, consumers or the private
constructor. Adjacent review found no other required visibility seam: `SourceGraphSource::new`,
`prepare_graph_source_set`, mappings, reports, `into_host_chunk_provider`, submit and seek are
already public.

The final report seam is equally bounded. Extend private `TimingResult` and public
`GraphCompileReport` with `output_latency: LatencySamples` and `output_tail: TailSamples`. After the
existing single-output cardinality check and the existing timing loop, read the sole output node's
already-computed `arrivals` and `extents`; copy those values into the report. Do not reconstruct them
from route rows, walk the graph again, or change canonical debug/SHA/DOT identity. Focused tests
cover a direct zero-latency/zero-tail output, unequal routes aligned by PDC, a finite effect tail, an
infinite delay tail, latency-preserving bypass, and equal scalar/banked/builtin artifacts. The only
permitted graph edit is `crates/miso-engine-graph-compiler/src/lib.rs` plus its colocated tests.

Compilation is an off-render transaction:

```text
strict TOML -> compiled session -> host rings/producers+consumers
            -> effects+builtins -> graph/PDC -> bound prepared plan
            -> publish {session-control, render-plan} together
```

Any failure drops all provisional Rust ownership and returns both output handles null. The control
handle owns producers, source-ID lookup, canonical session bytes and bounded Issue-005 capability
controller state. The render handle owns consumers/source set and the prepared plan. They do not
borrow the engine or each other after successful publication.

For every source, retain the compiled checked region start and end in the control handle. Initialize
the ring at the start through `prepare_host_region`. Before calling the public host provider, reject
any chunk whose checked `[start_frame,start_frame+frames)` is outside the region; accept the final
full/short block or zero marker only when its end equals the compiled end. Reject seek targets
outside `[region_start,region_end]` before advancing generation, preserving the native controller's
inclusive end seek for a zero-frame final marker. Update wrapper-local expected position only
after the underlying submit/seek succeeds, so backpressure and validation failures remain atomic.

The session handle has exactly one serial producer/control owner. Source submit and source seek may
execute concurrently with the render handle because they use the accepted disjoint SPSC endpoints;
they may not race each other or generic command submission. The plan has exactly one exclusive
render owner. Every destroy and every structural/non-SPSC operation requires both sides quiescent.

## ABI rules to implement literally

- Hand-write `crates/miso-engine-capi/include/miso_engine_v2.h`; do not use Rust repr as the
  specification or generate a
  host-dependent header. ABI version is `0x0001_0000`.
- Use opaque pointer handle declarations, fixed-width values and `{struct_size,...reserved}` V1
  structs. All entrypoints catch panics and return one frozen `uint32_t` result code.
- Null is accepted only by destroy and rejected elsewhere. A live handle carries a private kind/
  version cookie to reject wrong live handle types. Forged/stale pointers remain caller UB.
- All caller input is borrowed for the duration of the call. Source submit copies PCM atomically;
  render writes caller planes and retains nothing. There are no allocator, diagnostic, source,
  output or log callbacks.
- Required-length output follows one rule: null/zero queries length; adequate buffer writes the
  complete value; inadequate buffer writes no prefix and returns BUFFER_TOO_SMALL with required
  length. Lengths are bytes, not element counts.
- Render is exact-quantum/exact-time stereo planar output. Failed validation occurs before entering
  the prepared plan and changes no state. The C wrapper itself performs no allocation, lookup growth,
  feature detection or formatting in render.

Freeze result values as `OK=0`, `INVALID_ARGUMENT=1`, `ABI_MISMATCH=2`, `WRONG_HANDLE=3`,
`BUFFER_TOO_SMALL=4`, `COMPILE_REJECTED=5`, `BACKPRESSURE=6`, `UNSUPPORTED=7`,
`RENDER_REJECTED=8`, and `INTERNAL=255`. Tail kinds are `FINITE=0` and `INFINITE=1`; source final
and report flags are `uint32_t` restricted to zero or one. These are macros/constants, not C enums
whose storage is implementation-defined.

The V1 header freezes these field sequences; every unnamed `reserved` element is required-zero:

- `miso_engine_v2_engine_config {u32 struct_size, u32 abi_version, u64 reserved[4]}`;
- `miso_engine_v2_compile_limits {u32 struct_size, u32 source_ring_frames, u32
  maximum_automation_spans_per_block, u32 reserved0, then u64 maximum_toml_bytes,
  maximum_diagnostic_bytes, maximum_tracks, maximum_sources, maximum_routes, maximum_effects,
  maximum_graph_session_plus_plan_bytes, maximum_source_total_bytes,
  maximum_source_overhead_bytes, maximum_effect_state_bytes, maximum_effect_scratch_bytes,
  maximum_builtin_retained_bytes, maximum_capi_retained_bytes,
  maximum_named_allocation_bytes,
  maximum_meter_streams, maximum_meter_items, maximum_meter_bytes, maximum_control_frame_bytes,
  maximum_replay_bytes, maximum_replay_entries, reserved[4]}`;
- `miso_engine_v2_bytes_out {u32 struct_size, u32 reserved0, u8 *data, u64 capacity_bytes,
  u64 required_bytes}`;
- `miso_engine_v2_source_chunk {u32 struct_size, u32 sample_rate_hz, u64 generation,
  u64 start_frame, const float *const *planes, u32 plane_count, u32 frames, u32 end_of_region,
  u32 reserved0}` and `miso_engine_v2_submit_report {u32 struct_size, u32 reserved0,
  u64 accepted_frames, u64 cumulative_written_frames, u64 active_generation}`;
- `miso_engine_v2_planar_output {u32 struct_size, u32 channels, float *samples,
  u64 sample_capacity, u32 frames, u32 plane_stride_samples, u64 reserved[2]}`;
- `miso_engine_v2_capabilities {u32 struct_size, u32 abi_version, u64 exact_launch_rate_mask,
  u64 feature_mask, u64 reserved[4]}` where mask
  bits 0..3 mean 44.1/48/88.2/96 kHz and V1 features name immutable-session, host-planar-source,
  source-seek, planar-stereo-render and capability-command only; and
- `miso_engine_v2_plan_resource_report {u32 struct_size, u32 abi_version, u32 sample_rate_hz,
  u32 quantum_frames, u64 source_count, track_count, latency_samples, tail_kind, tail_samples,
  graph_session_plus_plan_bytes, graph_incremental_plan_bytes, graph_metadata_bytes,
  graph_delay_bytes, effect_bank_scratch_bytes, effect_bank_runtime_buffer_bytes,
  effect_bank_metadata_bytes, builtin_bank_bytes, builtin_bank_scratch_bytes,
  source_pcm_payload_bytes, source_overhead_bytes, source_total_bytes, effect_scalar_state_bytes,
  effect_scalar_scratch_bytes, builtin_processor_payload_bytes, builtin_meter_payload_bytes,
  builtin_retained_payload_bytes, capi_retained_bytes, largest_named_allocation_bytes,
  reserved[4]}`.
  `tail_samples` is zero when tail kind is infinite. Rows retain their production meanings and may
  overlap; consumers must not sum them. No global allocation count is exposed. The named largest
  field excludes opaque effect processor internals.

The exact prototypes are:

```c
uint32_t miso_engine_v2_abi_version(void);
uint32_t miso_engine_v2_query_capabilities(miso_engine_v2_capabilities *out);
uint32_t miso_engine_v2_engine_create(const miso_engine_v2_engine_config *config,
                                      miso_engine_v2_engine **out_engine);
void miso_engine_v2_engine_destroy(miso_engine_v2_engine *engine);
uint32_t miso_engine_v2_compile_session(miso_engine_v2_engine *engine,
    const uint8_t *toml, uint64_t toml_bytes, const miso_engine_v2_compile_limits *limits,
    miso_engine_v2_bytes_out *diagnostics, miso_engine_v2_session **out_session,
    miso_engine_v2_plan **out_plan);
uint32_t miso_engine_v2_source_submit_planar_f32(miso_engine_v2_session *session,
    const uint8_t *source_id, uint64_t source_id_bytes, const miso_engine_v2_source_chunk *chunk,
    miso_engine_v2_submit_report *out_report);
uint32_t miso_engine_v2_source_seek(miso_engine_v2_session *session,
    const uint8_t *source_id, uint64_t source_id_bytes, uint64_t generation, uint64_t source_frame);
uint32_t miso_engine_v2_submit_command(miso_engine_v2_session *session,
    const uint8_t *request, uint64_t request_bytes, miso_engine_v2_bytes_out *response);
uint32_t miso_engine_v2_render_f32_planar(miso_engine_v2_plan *plan,
    uint64_t absolute_sample, const miso_engine_v2_planar_output *output);
uint32_t miso_engine_v2_plan_resources(const miso_engine_v2_plan *plan,
    miso_engine_v2_plan_resource_report *out);
uint32_t miso_engine_v2_last_error(const void *live_handle, miso_engine_v2_bytes_out *out);
void miso_engine_v2_session_destroy(miso_engine_v2_session *session);
void miso_engine_v2_plan_destroy(miso_engine_v2_plan *plan);
```

`query_capabilities` is pure and accepts no handle. The three destroy functions are the only void
operations and are null-safe. All other null outputs/inputs reject. `compile_session` always writes
both child outputs null before validation; BUFFER_TOO_SMALL diagnostics still publish no children.

V1 output is deliberately one contiguous caller-owned planar region because the accepted
`PlanarBufferMut` requires a single allocation plus stride. Require `channels=2`, `frames=quantum`,
`plane_stride_samples>=frames`, and checked
`plane_stride_samples+frames<=sample_capacity`. This is zero-copy: do not add an engine staging
buffer or pretend two unrelated C pointers are one Rust slice.

Raw pointer/slice conversion and owned-handle reconstruction require a narrow FFI unsafe boundary.
Keep workspace `unsafe_code=deny`; allow it only inside `crates/miso-engine-capi/src/ffi.rs`, with a
local module lint override and a safety invariant beside every unsafe block. Amend
`scripts/check-realtime-policy.sh` to allow exactly that file and
`scripts/test-realtime-policy.sh` to prove unsafe in any other capi file or a second FFI path is
rejected. No DSP, graph, source or other crate gains unsafe.

The non-timed audit lives only in `tools/miso-engine-capi-audit/src/main.rs`, with its audited global
allocator added to the same exact policy allowlist. It invokes the exported C render symbol 100,000
times; it is not a benchmark and contains no timer. `scripts/check-capi-abi.sh` builds and links one
C11 consumer against the produced native library, compiles one C++17 include-only translation unit,
and checks the exact symbol set. `scripts/test-capi-abi.sh` uses scratch headers/libraries/tools to
prove missing compiler, header drift, layout/signature drift, symbol addition/removal and link
failure are rejected without executing an engine workload. No general host/runner tool is allowed.

Expose exactly the symbols enumerated in the issue. The generic command symbol supports only the
Issue-005 V1 capability request in this product; all other valid opcodes return canonical typed
unsupported responses. Do not pretend session edits publish a plan. Typed host-source seek remains
the only launch mutation in this slice.

## Limits and reports

Map the V1 limits struct explicitly into existing checked session, effect, builtin, graph and source
caps. `maximum_tracks` and similar values are caller-configured resource limits only. Ring frames
must be nonzero, at least one quantum and a quantum multiple. Reject extended sample rates and source
rate mismatch; never insert SRC.

Freeze one address-free V1 resource report by copying named fields from existing graph, source,
effect and builtin production reports, plus one checked C-wrapper retained byte row. Preserve each
row's existing overlap semantics and do not synthesize a total or global allocation count. Tests
derive equal and one-byte-below caps from the corresponding production row rather than duplicating
guessed literals. Do not expose addresses, Rust layout, capacity slack, allocator internals, or
duration-dependent source size.

`effect_scalar_state_bytes` is the checked sum of `metadata.state_sizes.total()` and
`effect_scalar_scratch_bytes` is the checked sum of `metadata.scratch_bytes` for every declared
prepared effect entry before graph ownership consumes it. This includes every bank member: the row
describes its scalar-equivalent per-instance contract even when execution is banked. Aggregate equal
and one-byte-below C caps are enforced after preparation and before graph/child publication. These
rows do not assert the largest private allocation inside an opaque processor.

`largest_named_allocation_bytes` is exactly the maximum of
`GraphResourceEstimate::largest_allocation_bytes`, source-set/source-ring largest allocation,
`BuiltinResourceEstimate::maximum_single_allocation_bytes`, and the C-wrapper largest allocation
described below. `maximum_named_allocation_bytes` maps to those existing graph/source/builtin caps
and the C-wrapper cap. No effect largest-allocation field is added, inferred or advertised.

The session child preallocates its complete capability-command state before either child publishes:
canonical session bytes; a sorted boxed source record array and contiguous source-ID bytes;
per-handle fixed error/diagnostic bytes; request/decode and response/encode scratch each bounded by
`maximum_control_frame_bytes`; a replay payload arena bounded by `maximum_replay_bytes`; and a fixed
replay-entry record array bounded by `maximum_replay_entries`. The Issue-022 controller supports only
capability command/replay and typed unsupported responses; it does not instantiate the full
`ProtocolController` or protocol queues. Every retained request is represented by a checked
`Layout::array` payload row; their checked sum is `capi_retained_bytes` and their max is the C-wrapper
input to `largest_named_allocation_bytes`. Allocate them during compile, reject equal/one-below caps
transactionally, and never lazily reserve/grow retained storage in submit, render or error paths.
Transient off-render codec locals are not retained-resource rows. Issue 073 may consume the frozen
reserved controller/replay storage but may not change ABI V1.

The permitted source edit is only `crates/miso-engine-source/src/lib.rs` for
`prepare_host_region` and its focused tests. `native_source.rs`, region parsing, producer/consumer
semantics and resource formulas remain unchanged. Needing any second source visibility or behavior
change is a briefing STOP. Likewise, no protocol/controller resource seam and no effect metadata or
resource seam is permitted. Any further fundamental API/reporting gap is final STOP/rescope.

## Representative proof

Use compact generated strict-TOML sessions, not a broad corpus:

- one track proves source submit/final/underrun/seek and exact direct-vs-C PCM/state;
- ten tracks prove host-selected full banks plus scalar tails, effects, PDC/bypass and no track cap;
- repeat both at 44.1/48/88.2/96 kHz for consecutive quanta;
- one barrier-controlled two-thread row proves the sole serial source producer may submit/seek while
  the exclusive render owner consumes, then joins both before either handle is destroyed;
- mutation rows cover every result code, live wrong-kind handles, layouts, caller buffers, atomic
  compile, source ordering/backpressure and equal/one-below resources;
- one C11 translation unit includes and links the header; one C++17 translation unit proves header
  compatibility only; and
- the C render entrypoint executes 100,000 calls under the existing non-timed audit with all nine
  categories zero.

The oracle is the same accepted V2 product prepared directly in Rust from the same typed session and
host chunks, compared outside render. It is an integration oracle, not an independent DSP oracle;
effect/corpus qualification is already owned elsewhere.

## Checkpoint sequence and STOP surface

1. Header, ABI constants/results/structs, opaque handle wrappers and layout/link tests.
2. Transactional compile plus host-source submit/seek and exact resource report.
3. Render and representative direct parity, error atomicity and 100,000-call audit.
4. Clean focused/workspace/policy seal and candid evidence.

Pause for a root commit after each focused-green tranche. Stop rather than expand if full Issue-005
mutation application, native decoding, plan exchange, a global handle registry, unsafe arbitrary-
pointer validation, a new graph/source API, or any callback on render becomes necessary.

## PASS boundary

PASS means the exact V1 header and symbols are linked, the immutable host-fed session renders the
accepted graph through a zero-operation render wrapper with correct ownership/errors/resources,
and all representative gates pass. It does not claim native file-runner readiness, complete control
mutation support, cross-platform ABI qualification, benchmark performance, or release readiness;
Issue 073 owns those claims without changing ABI V1.
