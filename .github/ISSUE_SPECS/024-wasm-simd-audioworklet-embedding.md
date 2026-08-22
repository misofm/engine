# 024 Launch scalar and simd128 AudioWorklet host

## Outcome

Ship the smallest browser host that prepares one accepted immutable, host-fed Engine V2 session in
one nonshared Wasm instance and renders deterministic stereo PCM from an `AudioWorkletProcessor`.
Publish separately built scalar and base-`simd128` artifacts with preparation-time feature selection
and scalar fallback. This issue is the launch product; broad browser qualification is Issue 074.

## Status and attempt budget

**SOL-BRIEFED / READY FOR TERRA ATTEMPT 1.** Permit exactly one Terra implementation attempt and one
bounded Sol correction/review. A second failed attempt stops and rescopes; gates may not be weakened.
No implementation, browser workload, benchmark or timing invocation has occurred while briefing.
The local and remote Issue 024 title/body must be synchronized before implementation.

## Context and fixed boundary

Consume the accepted strict-session, source-ring, builtin/effect, graph/PDC and prepared-plan APIs
directly in `miso-engine-host-web`. Do not reuse the native C ABI implementation: its pointer ABI and
layouts are native-host oriented and its orchestration is crate-private. Issue 022 remains the
accepted ownership/error/resource pattern, not a Wasm call-through layer. No new core, graph, source,
session, effect, protocol or C-ABI seam is permitted. Discovering one is a briefing STOP.

The browser host owns exactly one immutable prepared session and plan. There is no plan swap,
structural session edit, filesystem/network work in the processor, decoder, SRC, third-party Wasm,
thread, shared memory, `memory64`, Wasm atomics or relaxed SIMD. Launch rates remain exactly 44,100,
48,000, 88,200 and 96,000 Hz.

## Product topology and ownership

The main-realm module selects and compiles one Wasm artifact, loads the checked-in worklet module,
and constructs a node while its `AudioContext` is suspended. The processor constructor synchronously
instantiates the selected module, writes and validates configuration, prepares fixed staging, writes
the strict TOML, compiles the immutable session/plan, reacquires every post-growth pointer/view, pins
the post-compile `memory.buffer` identity, caches the output/status/resource views and only then posts
ready. Its message handler performs source submit/seek copies, status encoding and destruction
between callbacks. Only `process()` calls the already-prepared render export and copies its two
cached contiguous output views into the browser output planes.

One processor owns one Wasm instance, engine host, source producers/consumers and prepared plan.
JavaScript never retains a Rust pointer or typed view across an allocation-capable call. Config and
TOML views are temporary; the constructor reacquires all pointers after both `prepare` and `compile`.
Only post-compile views are cached, and `process()` compares `memory.buffer` identity before touching
them. Any identity change writes positive-zero output and becomes sticky reprepare-required. The
memory byte length may not change after compile. Transferred source buffers become owned by the
worklet message, are copied into bounded Wasm staging, and are transferred back in the matching ACK
or error before the pending call settles, including validation failure or engine backpressure.
There is exactly one in-flight source/control request: the main wrapper sends the next only after the
matching ACK, providing typed browser-side backpressure without `SharedArrayBuffer` or `Atomics`.

The caller supplies explicit `quantumFrames` for preparation alongside the actual
`BaseAudioContext.sampleRate`; both must equal the strict session. The main realm compares
`context.renderQuantumSize` only when that property is exposed and nonzero, because the Web Audio
contract may report zero before the suspended context has transitioned to running. The processor
likewise compares `AudioWorkletGlobalScope.renderQuantumSize` only when exposed and nonzero before
ready. Every `process()` call still validates the actual output-plane lengths against the prepared
quantum. Any mismatch, unsupported rate, output-shape change or context reconfiguration rejects
before ready or becomes sticky `reprepare_required`; it never hardcodes 128, inserts SRC, or parses,
allocates or reprepares inside `process()`.

## Frozen Wasm ABI V1

The cdylib exports memory and only these browser entrypoints, all returning fixed `u32` result codes
unless noted:

```text
miso_engine_web_v1_abi_version() -> u32                 // 0x0001_0000
miso_engine_web_v1_config_new() -> u32                  // nonzero config handle
miso_engine_web_v1_config_ptr(handle) -> u32
miso_engine_web_v1_config_bytes() -> u32
miso_engine_web_v1_prepare(handle) -> u32               // handle becomes prepared on OK
miso_engine_web_v1_buffer_ptr(handle, kind) -> u32
miso_engine_web_v1_buffer_capacity(handle, kind) -> u32
miso_engine_web_v1_compile(handle, toml_bytes) -> u32
miso_engine_web_v1_source_submit(handle, source_id_bytes, generation: u64,
    start_frame: u64, channels, frames, end_of_region) -> u32
miso_engine_web_v1_source_seek(handle, source_id_bytes, generation: u64,
    source_frame: u64) -> u32
miso_engine_web_v1_render(handle, actual_frames: u32) -> u32
miso_engine_web_v1_resource_ptr(handle) -> u32
miso_engine_web_v1_status_ptr(handle) -> u32
miso_engine_web_v1_dispose(handle) -> u32
```

V1 result codes are `OK=0`, `INVALID_ARGUMENT=1`, `ABI_MISMATCH=2`, `WRONG_STATE=3`,
`BUFFER_TOO_SMALL=4`, `PREPARE_REJECTED=5`, `BACKPRESSURE=6`, `UNSUPPORTED=7`,
`RENDER_REJECTED=8`, `REPREPARE_REQUIRED=9`, and `INTERNAL=255`. Null/zero handles reject except
that disposing zero is `OK`.

`WebPrepareConfigV1` starts with `struct_size`, `abi_version`, `sample_rate_hz`, `quantum_frames`,
then `u32` fields `session_toml_bytes`, `diagnostic_bytes`, `source_id_bytes`,
`maximum_source_channels`, `source_ring_frames`, `maximum_automation_spans_per_block`, followed by
checked `u64` limits for tracks, sources, routes, effects, graph session-plus-plan bytes, source total
bytes, source overhead bytes, effect state bytes,
effect scratch bytes, builtin retained bytes, host retained bytes, named allocation bytes, meter
streams/items/bytes, and four required-zero reserved words. The TypeScript writer, Rust `repr(C)`
type and offset/size test are one contract. Every finite value must fit Wasm32 `usize`/`isize` and
all byte formulas use checked arithmetic.

Buffer kinds are `SESSION_TOML=1`, `SOURCE_ID=2`, `SOURCE_PCM=3`, `DIAGNOSTIC=4`,
`OUTPUT_PCM=5`. Source PCM is planar contiguous
`maximum_source_channels * quantum_frames`; output PCM is exactly left quantum followed by right
quantum. Buffer pointers are stable only after `prepare` and until `dispose`. `compile` is atomic:
failure retains no partial session/plan and leaves the config host reusable only for diagnostic read
and disposal. Source submit/seek preserve the accepted absolute-frame, generation, bounded-ring and
transactional ownership semantics.

The render export owns no caller timeline: the safe host's `next_absolute_sample` is authoritative.
It first compares `actual_frames` with the prepared quantum; a mismatch atomically sets state
`FAILED`, records `REPREPARE_REQUIRED`, leaves the plan unrendered and requires positive-zero browser
output. An exact match invokes one `render_next()` and advances the internal timeline. The worklet
passes zero for any malformed/missing/unequal output-plane shape, otherwise the common exact length.

`WebStatusV1` is exactly `struct_size`, `abi_version`, `state`, `last_result`, `backend`,
`sample_rate_hz`, `quantum_frames`, `reserved0`, `next_absolute_sample`, `rendered_quanta`,
`reserved[4]`. State values are `CONFIG=0`, `PREPARED=1`, `READY=2`, `FAILED=3`, `DISPOSED=4`;
backend values are `SCALAR=0`, `SIMD128=1`. Status is written only outside `process()` except for
fixed-word `last_result`, next-sample and rendered-quantum updates; the message handler copies it
into the address-free `miso.status.v1` response.

`WebResourceReportV1` starts with `struct_size`, `abi_version`, `sample_rate_hz`, `quantum_frames`,
`backend`, `reserved0[3]`, then `u64` rows in this order: `config_bytes`, `status_bytes`,
`session_toml_bytes`, `diagnostic_bytes`, `source_id_bytes`, `source_pcm_staging_bytes`,
`output_pcm_bytes`, `bridge_metadata_bytes`, `bridge_retained_bytes`,
`largest_bridge_allocation_bytes`, `source_total_bytes`, `source_overhead_bytes`,
`effect_scalar_state_bytes`, `effect_scalar_scratch_bytes`, `builtin_retained_bytes`,
`graph_session_plus_plan_bytes`, `graph_incremental_plan_bytes`, `graph_metadata_bytes`,
`graph_delay_bytes`, `largest_named_allocation_bytes`, `reserved[4]`. Preserve the production
overlap meanings; do not invent a global allocation count. Equal caps pass and one-byte-below
host-retained/named-allocation caps reject transactionally.

Raw Wasm pointer/slice conversion is confined to
`hosts/miso-engine-host-web/src/ffi.rs` with local `unsafe_code` permission, an invariant beside each
block, exact realtime-policy allowlisting and mutations proving a second unsafe file/path rejects.
Workspace `unsafe_code=deny` remains unchanged.

## Frozen JavaScript/TypeScript API and messages

Checked-in ESM exposes:

```ts
createMisoAudioWorkletHost(options: {
  context: BaseAudioContext;
  quantumFrames: number;
  sessionToml: Uint8Array;
  limits: MisoWebPrepareLimitsV1;
  scalarModuleUrl: string;
  simd128ModuleUrl: string;
  workletModuleUrl: string;
}): Promise<MisoAudioWorkletHost>
```

The resolved host exposes immutable `node`, `backend: "scalar" | "simd128"`, exact `resources`,
`submitSource(request): Promise<MisoAckV1>`, `seekSource(request): Promise<MisoAckV1>`,
`status(): Promise<MisoStatusV1>`, and idempotent `dispose(): Promise<void>`. A source request is
exactly `{requestId, sourceId, generation, startFrame, sampleRateHz, planes, frames, endOfRegion}`;
a seek request is exactly `{requestId, sourceId, generation, sourceFrame}`. Requests carry a
monotonic safe-integer `requestId`, UTF-8 source ID, `BigInt` generation/absolute frame, exact sample
rate, planar `Float32Array` data and final marker. The wrapper rejects non-safe, duplicate or
non-increasing request IDs before transfer. Message tags are `miso.source.v1`, `miso.seek.v1`,
`miso.status.v1`, `miso.dispose.v1`, `miso.ready.v1`,
`miso.ack.v1`, `miso.error.v1`. Every response echoes `requestId` and returns a frozen result code;
unknown tags/fields reject, and errors are sticky/address-free. There is no generic session mutation
or PCM on a protocol/network transport.

Only nonshared `ArrayBuffer`-backed planes are accepted; `SharedArrayBuffer` rejects. Build a unique
transfer list by underlying buffer so multiple planar views over one buffer do not duplicate a
transferable. Preserve each view's exact type/offset/length. Once `postMessage` succeeds the worklet
owns those buffers; its ACK/error returns the original plane views with the unique buffers in the
response transfer list on every result, including `BACKPRESSURE`, so the resolved/rejected result
restores caller ownership for reuse or retry. The one pending slot covers submit, seek, status and
dispose. ACK/error, `messageerror`, `processorerror` and disposal each settle and clear it exactly
once; a concurrent call rejects locally with typed `BACKPRESSURE`. Repeated dispose after the first
settled disposal resolves without another message or Wasm call.

Selection occurs in the main realm before `addModule`: validate the canonical minimal simd128 probe,
then compile the simd artifact; either failure selects scalar. The selected compiled
`WebAssembly.Module`, configuration and TOML are transferred/cloned into `processorOptions`. There
is no fallback after node construction and no render-time capability detection.

Before ready, on sticky error, post-compile memory-buffer identity change or output mismatch,
`process()` writes `+0.0` to every available output sample. While ready it validates the actual
two-plane shape, invokes exactly one `render(handle, actual_frames)` per quantum and uses pre-created
`Float32Array` views plus `output[0][0].set(left)`/`output[0][1].set(right)`. It does not allocate,
construct or advance a JavaScript `BigInt`, grow memory,
post messages, format/log, feature-detect or call a fallible unbounded operation. It returns `true`
until explicit disposal; after quiescent disposal it returns `false`. Abrupt user-agent teardown
reclaims the isolated Wasm instance; no native worker or external resource exists.

## Build artifacts and allowed files

`scripts/build-web-audioworklet.sh` builds into a fresh caller-supplied directory and refuses
overwrite. It emits exactly:

- `miso-engine-v2-audio-worklet.scalar.wasm` built with `-simd128`;
- `miso-engine-v2-audio-worklet.simd128.wasm` built with `+simd128` and no relaxed SIMD;
- `miso-engine-v2-audio-worklet.js` (processor);
- `miso-engine-v2-audio-worklet-host.js` (main-realm wrapper); and
- `miso-engine-v2-audio-worklet-host.d.ts`.

Allowed implementation surface is the root workspace manifest/lock only for exact new direct
dependency rows; `hosts/miso-engine-host-web/**`; new exact web build/check/test scripts; and the
minimum exact realtime/workspace policy allowlist plus mutation changes for the one FFI file.
No accepted corpus, C ABI, session, source, graph, DSP/effect/builtin, protocol, CI or native-host
file may change.

## Representative product gates

1. Rust unit tests prove ABI layout/result/buffer/state transitions; config overflow and malformed
   values; atomic compile; source submit/final/backpressure/seek; 64/128/256 quantum preparation;
   exact resource equal/one-below caps; stable post-prepare pointers; disposal order; and no memory
   growth across representative render calls.
2. Scalar and simd128 Wasm artifacts build from one sealed source/lock in unique scratch. Object
   checks reuse Issue 068 rules: scalar has no SIMD/atomics; simd has required base `f32x4`
   operations, no relaxed SIMD/atomics; neither imports shared memory, WASI, filesystem, network,
   clocks, threads or an allocator callable from `process`.
3. Hermetic JS tests prove exact message schema, one-in-flight ACK/backpressure, selected-module
   fallback, transferred-buffer ownership, cached-view invalidation detection, `+0.0` silence and
   sticky errors without launching a browser workload.
4. One representative installed Chromium/Chrome local `OfflineAudioContext` gate runs both forced
   scalar and supported simd128 at 48 kHz and an explicit actual-browser `quantumFrames` value,
   confirms each nonzero exposed main/worklet quantum matches it, renders the same
   strict one-track host-fed session over consecutive quanta, exercises submit and seek, and compares
   PCM plus `WebStatusV1` and `WebResourceReportV1` to an independent direct V2 fixture. Consecutive
   PCM blocks prove continuation without claiming inaccessible DSP-state introspection. Each backend is deterministic across two
   fresh contexts; scalar/simd parity uses the accepted backend tolerance. The memory byte length and
   resource report stay unchanged after ready. This is a correctness test, not a timer or benchmark.
5. Focused plus locked workspace check/tests, warning-denied Clippy/rustdoc, format, web/realtime/
   workspace policies and mutations pass on one unchanged clean candidate. Static scans prove no
   SAB/Atomics/threads, no hardcoded quantum, no artifacts, and no benchmark/timer invocation.

## Explicit non-goals and successor

Issue 074 owns the browser family/version/mobile matrix, checked-in demo and deployment breadth,
million-quantum offline and ten-minute live stability, GC/memory instrumentation, bundle-size and
descriptive performance. It may not reopen this API or product correctness. This issue includes no
SAB fallback, browser multicore, worklet plan swap, BTLV mutation controller, filesystem streaming,
decode, SRC, third-party Wasm, WebSocket, benchmark, timing or listening.

## Dependencies by exact issue title

- Bootstrap Rust workspace and target matrix
- Real-time memory, buffers, queues, and plan lifetime
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Stable C ABI and host-fed planar PCM render
- Exact lock-free native source sanitation telemetry handoff
- Production SIMD builtin bank graph retention and reachability qualification
- Builtin native, AArch64, and Wasm runtime-selection and instruction qualification

## References and evidence

Web Audio API 1.1 defines AudioWorklet, `MessagePort`, context/worklet sample rate and the reported
render quantum: https://www.w3.org/TR/webaudio-1.1/. The Rust target boundary is
https://doc.rust-lang.org/rustc/platform-support/wasm32-unknown-unknown.html.

Required evidence is the clean candidate and source/lock seals; exact artifact hashes/export/import
lists; ABI/layout/resource reports; scalar/simd object records; hermetic mutation results; browser
name/version and fixture/PCM/status hashes; unchanged memory bytes; strict Terra/Sol verdicts;
`browser_correctness_invocations` and `benchmark_or_timed_invocations=0`. No qualification claim
from Issue 074 is implied.
