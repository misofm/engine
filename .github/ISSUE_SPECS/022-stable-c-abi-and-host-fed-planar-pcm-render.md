# 022 Stable C ABI and host-fed planar PCM render

## Sol briefing checkpoint — 2026-08-22

**READY FOR TERRA ATTEMPT 1.** The authoritative implementation brief is
`BRIEFS/022-stable-c-abi-and-host-fed-planar-pcm-render.md`. This issue permits one Terra attempt
and one bounded Sol correction; a second failure stops and rescopes. No benchmark or timed workload
is authorized.

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
  bytes; C-wrapper retained bytes; largest allocation; source-ring frames; meter streams/items/
  bytes; automation spans; and control-frame/replay bytes. Zero is invalid; configured counts are
  resource ceilings, not compiled track maxima. Checked overflow or any one-byte-below row rejects
  before either child handle publishes.
- Source submission names the canonical source ID by borrowed UTF-8 bytes and provides generation,
  absolute source frame, channel-plane pointers, valid frames, and final-block flag. It copies
  atomically into the preallocated ring, retains no caller pointer, rejects rate/channel/length/
  generation/order mismatches without accepting a prefix, and reports typed bounded backpressure.
  Seek requires a strictly increasing nonzero generation and is visible only at the next block.
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
  allocation, quantum/rate, source/track count, latency and tail kind/value. It does not invent a
  global allocation count or claim overlapping rows are additive.

## Deliverables

- the hand-written installed header and matching `miso-engine-capi` exports;
- one Rust ownership/orchestration layer over accepted V2 crates;
- C and Rust ABI/layout/symbol tests plus one linked C smoke program; and
- representative host-fed full-graph PCM, source, diagnostics, resources, and realtime evidence.

The permitted implementation surface is `crates/miso-engine-capi`, the one sealed source-set bind
addition and its focused tests in `miso-engine-builtins-compiler`, workspace manifests/lock required
only by those direct dependencies, `tools/miso-engine-capi-audit`, `scripts/check-capi-abi.sh`,
`scripts/test-capi-abi.sh`, the exact realtime unsafe-policy checker/mutation scripts, and concise
Issue-022 evidence. Any other production API change is a STOP.

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
   buffers, wrong quantum/time,
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
   separately enforced graph/source/effect/builtin/C-wrapper row and largest-allocation cap,
   transactionally with both child outputs null.
6. A 100,000-call non-timed render audit on the C entrypoint reports zero allocations/frees, locks,
   I/O, logging, syscalls, feature detection and panic unwinds, with stable output pointers and no
   callback. Focused crate tests, format, warning-denied Clippy/rustdoc, locked workspace tests and
   applicable workspace/realtime/effect/graph/source policies pass.

## Required evidence

Record exact header/source hashes, ABI/layout/symbol transcript, direct-vs-C PCM/state/resource
hashes, error mutation table, 100,000-call audit counters, ownership/drop assertions, focused and
workspace/policy gates, attempt owner/count, and strict PASS/FAIL. Benchmark/timed invocation count
must remain zero.
