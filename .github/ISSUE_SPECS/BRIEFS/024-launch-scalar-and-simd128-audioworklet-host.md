# Sol implementation brief — issue 024 launch scalar and simd128 AudioWorklet host

## Decision

**FINAL SOL FAIL / STOPPED / RESCOPED.** Terra attempt 1 and the sole bounded Sol correction are
consumed. Checkpoint `ba7ffc6` preserves the safe host, 14-export ABI and focused-green JS boundary
only as technical input; it is not PASS. Browser-correctness/workload/benchmark/timed invocation
counts are zero. Issue 075, **Close AudioWorklet lifecycle and backend identity**, owns only backend/
rate proof, constructor cleanup, strict schema/static evidence and the single representative browser
gate. Issue 074 remains broad qualification.

## Smallest vertical

Extend `miso-engine-host-web` directly over the accepted public session, source, effect/builtin and
graph APIs. Do not call or modify `miso-engine-capi`; its native ABI is only the accepted ownership,
error and resource precedent. Preparation on the AudioWorklet global thread, while the context is
suspended and outside `process()`, may parse and allocate. Once ready, topology, capacity, Wasm
memory and cached output views are immutable.

The prepared Rust object owns the strict session, bounded host source producers/consumers, sealed
builtins/effects, graph and exclusive plan. The processor constructor synchronously instantiates the
selected module, writes config, prepares, writes TOML, compiles, reacquires all pointers and caches
views only against the final post-compile `memory.buffer`; there is no init message. The message
handler serially accepts one transferred source/seek request at a time, copies into preallocated Wasm
staging and ACKs it. `process()` sees no message payload, invokes one exact-time render and copies
cached contiguous L/R views into the two browser planes. No plan publication/retirement or general
command controller is in scope.

Preparation binds `context.sampleRate`, caller-supplied explicit `quantumFrames` and strict TOML
exactly. Do not claim a suspended main realm can discover the actual quantum:
`context.renderQuantumSize` is comparison evidence only when exposed and nonzero. Before ready, the
processor also compares `AudioWorkletGlobalScope.renderQuantumSize` only when exposed and nonzero.
Every callback validates actual output-plane length and post-compile `memory.buffer` identity before
touching cached views. Render is exactly `render(handle, actual_frames: u32)`; the safe host owns and
advances absolute time internally. A shape mismatch passes zero, leaves the plan unrendered and
becomes sticky reprepare-required; an identity mismatch likewise emits positive zero and fails.
`process()` never creates/advances a JavaScript `BigInt`, parses, allocates or reprepares. Launch rates only.
Test synthetic 64/128/256 plus the representative actual-browser path without treating 128 as
normative.

## Literal ABI and retained layout

Implement the exact V1 export names, result values, `WebPrepareConfigV1` fields, buffer kinds and JS/
TS message tags frozen in the issue. Keep JavaScript-visible pointers as Wasm32 `u32` offsets and
timeline/generation values as Wasm `u64`/JavaScript `BigInt`. `config_new` allocates only a fixed
config/error object. JavaScript fills the exported config layout; `prepare` validates it and creates
all staging/storage. No pointer/view is cached before successful preparation.

Retain exactly:

```text
config + fixed status/error
session TOML bytes
diagnostic bytes
source-ID bytes
source PCM bytes = maximum_source_channels * quantum * sizeof(f32)
output PCM bytes = 2 * quantum * sizeof(f32)
bridge metadata
accepted session/source/effect/builtin/graph ownership
```

Every product/size/sum/max uses checked Wasm32 arithmetic. Report each bridge row, bridge retained
sum/max and existing underlying production rows without summing overlapping reports or claiming a
global allocation count. Apply host-retained and named-allocation caps before publishing; return all
provisional ownership on rejection. `compile` publishes control/source ownership and plan or neither.

Source IDs are exact UTF-8 session IDs. Submission uses staged planar PCM, exact rate, generation
and absolute frame; accepts no more than one quantum per message; and inherits accepted region/final/
backpressure semantics. Seek is absolute and generation-tagged. Update browser-local expected
position only after product acceptance. Request IDs are caller-supplied safe integers that must be
strictly increasing and unique. One pending slot covers submit, seek, status and dispose; any second
call rejects locally with `BACKPRESSURE`. Settle and clear it exactly once on ACK/error,
`messageerror`, `processorerror` or disposal; repeated settled disposal is a local no-op.

Accept only nonshared `ArrayBuffer` plane storage. Reject SAB, validate each view and deduplicate the
transfer list by underlying buffer. Successful posting transfers ownership to the worklet. After the
synchronous staging copy/submission, return the original typed plane views and their unique buffers
in every ACK/error—including validation failure and engine `BACKPRESSURE`—so caller ownership is
restored for reuse or retry before the pending call settles.

`submitSource`/`seekSource` resolve the complete `MisoAckV1`
`{tag: "miso.ack.v1", requestId, result, planes?}`, never a payload or numeric result alone. Source
ACKs include the returned typed plane views. Nonzero engine results remain resolved typed ACKs;
transport/schema/processor failures reject with the complete address-free error record.

`process()` has cached exact-quantum `Float32Array` views. Default/not-ready/fatal/mismatch output is
positive zero. Render failure stores only a fixed numeric sticky status inside preallocated state;
the message handler later reports it. Do not call `postMessage`, allocate typed arrays, take
subarrays, format/log, feature-detect or grow memory in `process()`.

Raw pointer/slice handling is confined to `hosts/miso-engine-host-web/src/ffi.rs`; add exactly that
path to realtime-policy allowlisting and prove mutations elsewhere reject. Needing any new public
session/source/graph/effect/core/CAPI API is an immediate STOP.

## Artifact and loader contract

The build script requires an empty output directory and emits exactly the five issue-named files.
Both Wasm modules bind the same source and `Cargo.lock`. Scalar is built with `-simd128`; SIMD with
`+simd128`, never relaxed SIMD. Neither uses shared memory/atomics/threads/WASI.

The main ESM validates the canonical simd128 probe and compiles the SIMD module before
`audioWorklet.addModule`. Validation or compilation failure selects scalar. Pass only that compiled
module, config and TOML to `processorOptions`; instantiate synchronously in the processor
constructor and complete config/prepare/TOML/compile there. Reacquire all views after each
allocation-capable call; only post-compile views may be cached. Selection and readiness are
immutable, address-free and returned in `miso.ready.v1`.
Ready and status responses also report JS-layer `memoryBytes` from
`instance.exports.memory.buffer.byteLength`, sampled only after compile or in the status handler and
required to equal the pinned post-compile value. This adds no Wasm export or Rust status/resource
field; `process()` checks buffer identity only and never encodes or posts the value.
After explicit quiescent disposal, destroy plan/source/session ownership in reverse preparation order
and make later `process()` return `false`.

## Implementation order and stop gates

1. Add exact direct workspace dependencies and implement the safe preparation/ownership object plus
   resource/state tests. Stop if any accepted public seam is insufficient.
2. Add the single raw Wasm FFI module, layout tests and exact policy/mutation changes. Stop unless
   scalar host tests and checked overflow/ownership rejection are green.
3. Add build script and checked-in ESM/worklet/type declaration. Run static/hermetic loader and object
   gates first; stop on any import, atomics, relaxed-SIMD or export mismatch.
4. Add one small strict fixture and independent direct-V2 expected result, then run the single
   representative Chromium correctness gate for forced scalar and supported simd128. Compare
   consecutive PCM plus status/resources; no inaccessible DSP-state snapshot is claimed. No long run
   or performance measurement.
5. On a clean unchanged candidate run focused and locked workspace tests/check, warning-denied
   Clippy/rustdoc, format, policies/mutations and no-artifact/no-timer scans. Record strict PASS/FAIL.

Checkpoint each green stage before layering the next. Allowed files and all objective gates are
exactly those in the issue. `browser_correctness_invocations` must name actual browser launches;
`benchmark_or_timed_invocations` remains zero.

## Deferred qualification

Issue 074 alone owns checked-in demo/deployment breadth, multi-browser/mobile versions, million-
quantum and ten-minute rows, GC/memory instrumentation, bundle size and descriptive performance. It
consumes this API and artifacts without redesign.
