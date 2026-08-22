# 022 Stable C ABI and host-fed planar PCM render

## Final Sol verdict — 2026-08-22

**PASS.** The authoritative implementation brief is
`BRIEFS/022-stable-c-abi-and-host-fed-planar-pcm-render.md`. Terra attempt 1 and the sole bounded Sol
correction are consumed; the exact non-timed audit passed on the clean evidence candidate. No
benchmark or timed workload was authorized or invoked. Stateless Issue 073 remains open and owns
the native PCM runner and broader ABI qualification; this verdict does not claim that scope.

### Bounded preimplementation correction — host region origin

**READY; TERRA MAY PROCEED.** Strict sessions permit nonzero `SourceRegion.start_sample`, and ABI
chunk/seek positions are absolute source frames. The existing public `PcmSourceRing::prepare`
hardcodes frame zero while the already-used `prepare_at_source_frame` ownership path is crate-private.
Permit exactly one new public `PcmSourceRing::prepare_host_region(config, initial_frame)` constructor
in `miso-engine-source`. It delegates unchanged to that existing private implementation; it adds no
validation policy, allocation, worker, decoder or state. Focused source tests prove zero/nonzero
origins, identical resource/shape reports, first-chunk contiguity, seek transition and unchanged
rejection ownership. No other source visibility change is permitted.

### Final preimplementation report correction

**READY; THIS IS THE FINAL PERMITTED-SURFACE CORRECTION.** `timings` already computes the sole
session output's final arrival and propagated finite/infinite extent, but `GraphCompileReport`
currently discards both. Permit `miso-engine-graph-compiler` to add only `output_latency:
LatencySamples` and `output_tail: TailSamples` to that report, copied from the existing checked
`TimingResult`; no second timing walk or C-ABI reconstruction is allowed. Focused graph tests cover
direct, PDC-merged, finite-tail, infinite-tail, bypass and bank/scalar-equal cases while proving
canonical graph identity is unchanged.

The C report's `effect_scalar_state_bytes` and `effect_scalar_scratch_bytes` are the checked sums of
metadata-declared scalar-equivalent state/scratch for every declared effect instance, including
instances executing as homogeneous bank members. They are contract rows, not an observation of an
effect processor's private allocation layout. `largest_named_allocation_bytes` is therefore the max
only of the named graph, source, builtin and C-wrapper largest-allocation rows; it explicitly excludes
opaque effect processor internals. No effect-contract/report change is permitted.

All C-wrapper control-frame scratch, error/diagnostic storage, source-ID/region records, canonical
session bytes, capability-command replay payload and replay-entry metadata are allocated to their
configured fixed capacities during successful session compilation, included in
`capi_retained_bytes`, and never lazily grown. Compile rejects before publishing either child when
that checked payload exceeds its cap. No `ProtocolController` or protocol-queue resource seam is
required for Issue 022's capability-only command surface. Any further fundamental visibility or
reporting gap after this correction is a STOP/rescope, not another briefing amendment.

## Outcome

Ship the smallest useful stable native embedding boundary: compile one immutable strict-TOML
session into separately owned control and render handles, feed its declared sources with borrowed
planar `f32` chunks outside render, and render deterministic caller-owned planar stereo PCM through
the accepted builtin/effect/graph/source stack.

## Context

Engine V2's render plane exclusively owns a preallocated `PreparedRenderPlan`. Render may not
allocate/free, lock, perform I/O, log, call user code, or mutate graph structure. The accepted
session compiler, native effect registry, builtin compiler, graph compiler, host-fed source rings,
and runtime-selection contracts already provide the Rust ownership boundary this ABI must preserve.
The ABI is a hand-written C contract, not a projection of Rust layout.

The former issue combined this product boundary with a native WAV/RF64 runner and broad ABI
qualification. Stateless Issue 073 owns that separable completion work. It also owns complete BTLV
provider/mutation integration; this issue proves only the frozen capability query over the generic
frame exchange and exposes source seek as a typed control operation.

## Frozen ABI V1

The checked-in C11 header is `crates/miso-engine-capi/include/miso_engine_v2.h`. Every public symbol
uses the `miso_engine_v2_` prefix, `extern "C"`, fixed-width integer fields, explicit byte lengths,
and a `uint32_t struct_size` first field for extensible structs. No Rust enum, `bool`, `usize`,
slice, string, allocator, or unwinding crosses the boundary. ABI V1 is `0x0001_0000`; reserved input
fields must be zero and output structs zero their reserved fields.

Opaque live handle types are engine, session-control, and render-plan. Successful compilation
transactionally returns the latter two together. The engine is needed only for later compile calls;
children own their allocations independently. Session-control owns source producers and canonical
session/control state. Render-plan owns source consumers plus the prepared graph and DSP state.
Each handle has one matching off-render destroy function accepting null as a no-op. Handles must be
live values returned by this ABI and of the required type; arbitrary, forged, or use-after-destroy
pointers are C caller undefined behavior and are not advertised as safely probeable.

Required symbols are:

- `miso_engine_v2_abi_version` and `miso_engine_v2_query_capabilities`;
- `miso_engine_v2_engine_create` / `miso_engine_v2_engine_destroy`;
- `miso_engine_v2_compile_session`;
- `miso_engine_v2_source_submit_planar_f32` and `miso_engine_v2_source_seek`;
- `miso_engine_v2_submit_command` for Issue-005 V1 framing, supporting the exact capability query
  and returning its canonical typed unsupported response for every other well-formed opcode;
- `miso_engine_v2_render_f32_planar`;
- `miso_engine_v2_session_destroy` and `miso_engine_v2_plan_destroy`; and
- `miso_engine_v2_plan_resources` plus `miso_engine_v2_last_error`.

Every function returns a fixed `uint32_t` result code: OK, invalid argument, ABI mismatch, wrong live
handle type, buffer too small, compile rejected, backpressure, unsupported, render rejected, and
internal failure. No function panics or unwinds across C. `last_error` is handle-local, caller-buffer
copy-out with required-length reporting; buffer-too-small writes no partial UTF-8. Compile diagnostics
are canonical UTF-8 bytes returned through the same query/retry convention. Successful calls clear
the relevant handle's prior diagnostic.

## Session, source, render, and resource contract

- `compile_session` accepts borrowed strict TOML bytes plus a V1 limits struct. It supports exactly
  44,100/48,000/88,200/96,000 Hz, preserves the TOML quantum, uses the launch native effect registry,
  and prepares builtins, effect banks/tails, graph/PDC, and one host-fed ring for every declared
  source. No file resolver, decoder worker, path callback, or implicit SRC is used.
- The only required dependency seam is a sealed
  `PreparedBuiltinsGraphArtifact::into_bound_with_source_set` sibling of the existing graph bind:
  it consumes genuine builtin bindings plus one `GraphPreparedSourceSet`, delegates to the existing
  transactional graph source-set bind, and returns the opaque artifact, caller bindings, and source
  set unchanged on rejection. It exposes no processor parts and changes no render behavior.
- Limits include maximum TOML and diagnostics bytes; tracks, sources, routes and effects; graph
  session-plus-plan bytes; source total/overhead bytes; effect state/scratch; builtin retained
  bytes; C-wrapper retained bytes; named largest allocation; source-ring frames; meter streams/items/
  bytes; automation spans; and control-frame/replay bytes. Zero is invalid; configured counts are
  resource ceilings, not compiled track maxima. Checked overflow or any one-byte-below row rejects
  before either child handle publishes.
- Source submission names the canonical source ID by borrowed UTF-8 bytes and provides generation,
  absolute source frame, channel-plane pointers, valid frames, and final-block flag. It copies
  atomically into the preallocated ring, retains no caller pointer, rejects rate/channel/length/
  generation/order mismatches without accepting a prefix, and reports typed bounded backpressure.
  The session-control wrapper also enforces the compiled region `[start_sample,
  start_sample+length_samples)` before touching the producer: chunks may not cross the end, the sole
  final full/short block or zero marker must end exactly there, and seek targets may name any frame
  from the start through the end (inclusive), matching native source semantics. Seek
  requires a strictly increasing nonzero generation and is visible only at the next block.
- Render accepts only the prepared quantum, exact next `absolute_sample` (initially zero), and one
  caller-owned contiguous planar region described by base pointer, sample capacity and plane stride.
  V1 requires two channels, stride at least quantum, and capacity at least stride plus quantum; the
  wrapper borrows that region directly as `PlanarBufferMut` and retains no PCM pointer. There is no
  hidden staging buffer, interleave, allocation or copy.
  Source underrun emits zero and advances deterministic counters. Success advances time by exactly
  one quantum; rejection advances neither time nor state. Output may be `-0.0` where product DSP
  permits it.
- The session-control handle has one serial non-render producer owner. Its source submit/seek calls
  may run concurrently with the plan's one exclusive render owner through the accepted disjoint SPSC
  endpoints; submit/seek may not race each other or generic command submission. The plan is `Send`
  but not `Sync`. Engine/session/plan destroy and structural/non-SPSC operations require quiescence;
  plan destruction and all reclamation are off-render. The library invokes no user callback from
  render.
- `plan_resources` is address-free and copies exact existing production rows: graph
  session-plus-plan/incremental/metadata/delay and bank payloads, source PCM/overhead/total, effect
  scalar state/scratch, builtin processor/meter/retained payloads, C-wrapper retained bytes, largest
  named allocation, quantum/rate, source/track count, and the new graph-reported output latency and
  tail kind/value. It does not invent a global allocation count, include opaque effect internals in
  the named maximum, or claim overlapping rows are additive.

## Deliverables

- the hand-written installed header and matching `miso-engine-capi` exports;
- one Rust ownership/orchestration layer over accepted V2 crates;
- C and Rust ABI/layout/symbol tests plus one linked C smoke program; and
- representative host-fed full-graph PCM, source, diagnostics, resources, and realtime evidence.

The permitted implementation surface is `crates/miso-engine-capi`, the one sealed source-set bind
addition and its focused tests in `miso-engine-builtins-compiler`, workspace manifests/lock required
only by those direct dependencies, `tools/miso-engine-capi-audit`, `scripts/check-capi-abi.sh`,
`scripts/test-capi-abi.sh`, the exact realtime unsafe-policy checker/mutation scripts, and concise
Issue-022 evidence. It additionally permits only `crates/miso-engine-source/src/lib.rs` for the
named host-region constructor and its colocated focused tests, plus
`crates/miso-engine-graph-compiler/src/lib.rs` for the two final-output report fields and focused
tests. Any other production API, resource-report, or visibility change is a STOP.

## Explicit non-goals

Native WAV/RF64 decoding, filesystem callbacks, CLI/reference runner, interleaved PCM, variable
render frames, user callbacks during render, plan replacement, complete Issue-005 provider/mutation
support, telemetry streaming, mobile/browser adapters, arbitrary-pointer hardening, C++, network,
codecs, target/object breadth, benchmark, timing, and listening. Issue 073 owns runner and ABI
qualification without changing ABI V1.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Transport-neutral binary control protocol
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Native effect runtime contract and conformance
- Production SIMD builtin bank graph retention and reachability qualification
- Exact lock-free native source sanitation telemetry handoff
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## Ordered acceptance gates

1. Header and Rust declarations independently agree on every V1 constant, struct size/alignment/
   offset and symbol signature on the pinned 64-bit native ABI; a C11 consumer compiles and links,
   a C++17 translation unit includes the header, the export list is exact, and panic/unwind is
   caught before FFI return.
2. Null, wrong live handle type, wrong struct size/version/reserved fields, invalid UTF-8/TOML,
   duplicate/unknown source IDs, malformed source planes, invalid output stride/capacity, short
   buffers, wrong quantum/time, chunks crossing the compiled region, early/late final markers and
   out-of-region seeks,
   unsupported rate, overflow and one-byte-below caps return exact codes with atomic ownership and
   state. Tests do not dereference forged or stale pointers.
3. A representative one-track and ten-track host-fed session at each launch rate renders consecutive
   quanta byte-identically to the direct Rust path, including signed-zero input, partial final source
   block, underrun, seek generation, PDC/bypass, banks plus scalar tail, effects, state continuation,
   and deterministic destroy order. A bounded barrier-controlled schedule proves one producer can
   submit/seek while the exclusive render owner consumes, without lock, race, lost accepted chunk or
   broadened concurrency.
4. Issue-005 capability request bytes produce the exact canonical response through caller buffers;
   malformed frames and every other well-formed opcode produce the frozen typed response without
   mutation. Source submit/seek backpressure and retry are deterministic.
5. Resource reports match exact production projections; equal and one-byte-below tests cover each
   separately enforced graph/source/effect/builtin/C-wrapper row and named-largest-allocation cap,
   transactionally with both child outputs null. Output latency/tail equals the final graph report;
   effect scalar-equivalent rows include bank members; C-wrapper capacity is fully charged at
   compile; and named largest-allocation semantics exclude opaque effect internals.
6. A 100,000-call non-timed render audit on the C entrypoint reports zero allocations/frees, locks,
   I/O, logging, syscalls, feature detection and panic unwinds, with stable output pointers and no
   callback. Focused crate tests, format, warning-denied Clippy/rustdoc, locked workspace tests and
   applicable workspace/realtime/effect/graph/source policies pass.

## Required evidence

Record exact header/source hashes, ABI/layout/symbol transcript, direct-vs-C PCM/state/resource
hashes, error mutation table, 100,000-call audit counters, ownership/drop assertions, focused and
workspace/policy gates, attempt owner/count, and strict PASS/FAIL. Benchmark/timed invocation count
must remain zero.

## Sol pre-audit evidence — 2026-08-22

Candidate `334b680f6e561f4d679e4cc240b4acec25f0835f` passes the complete non-audit
seal after Terra attempt 1 and the sole bounded Sol correction. Sol corrected one ABI defect before
this clean seal: capability and plan-resource output structs now reject nonzero reserved input
atomically instead of overwriting it. No ABI shape, symbol, resource row, DSP, graph, source, or
threading contract changed.

Requirement audit before the executing audit:

- ABI/header/layout/link: CAPI unit tests pass 14/14; the C11 linked consumer, C++17 include-only
  consumer, exact export list, and all ABI mutation rows pass. The relevant frozen SHA-256 values
  are header `e7ba468361e0255cb465828c5dd317f1e5293213662c7bf9a5225cb2afaba4e7`,
  `abi.rs` `ec1d6f2b3f27108f540da869d200125343e12f3088c48c4614b0cd626a1971aa`,
  `ffi.rs` `d12cab166d917371020efb7ef380c3ee5d0efa752571198455453a4835a125b0`, and
  `runtime.rs` `6f58a176b264a6f19a610cae0c8dafd71ba438a424cc007677d9696d55d7c5d0`.
- Product behavior: executed C/direct tests cover transactional children, handle-local diagnostics,
  absolute-region submit/seek, exact-time caller-owned render, capability replay/typed unsupported,
  exact resource caps, one/ten-track bit parity at all four launch rates, signed zero, partial final
  blocks, underrun, seek, bank/scalar/PDC continuation, destroy orders, and the bounded two-thread
  source-producer/render-owner schedule. Dependency tests cover strict session/source/protocol error
  and retry semantics; static review confirms validation precedes publication or render entry.
- Sealed seams/resources: source-origin preparation, source-aware builtin graph bind ownership
  return, graph output latency/tail propagation, scalar-equivalent effect rows, named-largest rules,
  and fixed C controller/replay storage pass their focused and full-workspace tests. The only source
  production diff remains the permitted `crates/miso-engine-source/src/lib.rs` constructor surface.
- Clean gates pass: `cargo fmt --all --check`; locked workspace all-target/all-feature check and
  tests; warning-denied workspace all-target/all-feature Clippy; warning-denied workspace rustdoc;
  workspace, realtime, effect-runtime, graph/determinism, builtin, protocol-control, and rack policy
  checks plus their available mutation suites. `git diff --check` and the transient-artifact scan
  pass.
- Unsafe remains confined to `crates/miso-engine-capi/src/ffi.rs` and
  `tools/miso-engine-capi-audit/src/main.rs`; the exact policy and mutations pass. Audit-tool unit
  tests pass 3/3, its source SHA-256 is
  `9db77ecf95fd67ea8d37491b346f5353a181fd10caa661cee4a9ee73e731bdda`, and a static scan proves no
  timer API. The audit main has not been invoked.

Pre-audit verdict: **PASS TO ONE NON-TIMED CAPI AUDIT AFTER THIS EVIDENCE IS COMMITTED ON A CLEAN
CANDIDATE.** This is not overall Issue-022 PASS. `capi_audit_main_invocations=0`,
`timed_benchmark_invocations=0`, and no benchmark, trace, target, or listening workload ran.

## Final C-entry audit evidence — 2026-08-22

The sole authorized non-timed audit ran once on clean candidate
`8190baf24539dd31e38122712c791139d2fbe6d4` and exited zero. The candidate delta from the reviewed
`334b680` was exactly the two Issue-022 evidence documents; the frozen FFI and audit source SHA-256
values remained `d12cab166d917371020efb7ef380c3ee5d0efa752571198455453a4835a125b0` and
`9db77ecf95fd67ea8d37491b346f5353a181fd10caa661cee4a9ee73e731bdda`.

Preserved stdout `/tmp/engine-v2-issue-022-8190baf-capi-audit.json` is exactly 349 bytes with SHA-256
`9bc0b8a1b8a032e0a29fef7154141646be65be29cd5c41901ce66d2666f6c408`. Its closed schema reports
`calls=100000`, `sample_rate_hz=48000`, `quantum_frames=128`, stable output address, PCM digest
`37380b654988f7cc`, zero render errors, zero allocations, deallocations, locks, feature detection,
logs, file I/O, network I/O, syscalls, and panic unwinds, and zero total violations. Preserved stderr
is empty with SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`.

Final verdict: **PASS** for the bounded stable C ABI and host-fed planar PCM render product.
`capi_audit_main_invocations=1`, `benchmark_workload_invocations=0`, and
`timed_benchmark_invocations=0`. No retry, direct audit-binary run, benchmark, timing, trace, target,
or Issue-073 qualification workload occurred.
