# Sol implementation brief — issue 010 JIT PCM streaming and host-supplied source rings

## Post-review stop notice (2026-08-21)

**STOPPED — DO NOT START ANOTHER ISSUE-010 ATTEMPT.** The historical brief and gates below remain
the record for checkpoint `5dbe1cb` and its strict Sol FAIL. Production closure moved to
**Issue-010 launch-critical source ownership and accounting closure**; expanded qualification moved
to **Issue-010 source streaming qualification tooling and adversarial evidence**. Neither successor
retroactively makes Issue 010 PASS.

## Decision and attempt budget

**READY FOR TERRA ATTEMPT 1** on the current accepted dependency line. Implement one bounded
streaming vertical. The normal issue budget applies: one Terra attempt and at most two Sol
correction/review attempts, then stop and rescope. Do not inspect V1/legacy. No timed benchmark is
authorized or required; record invocation count zero.

## Accepted inputs — retain, do not redesign

- Issue 003 owns move-only/native SPSC and browser-local ring semantics, exclusive `Send`/`!Sync`
  render ownership, render audit hooks, block-boundary publication and off-render destruction.
  Reuse those safe queues. Any specialized unsafe storage would have to remain in core's existing
  `realtime/spsc.rs` exception, but the preferred design moves preallocated boxed planar transfer
  blocks through the accepted move-only SPSC and adds no unsafe.
- Issue 004 owns the strict source declaration: stable ID, nonzero declared source rate, opaque
  identity/locator, channel count, finite `start_sample + length_samples`, track source references
  and channel indices, and `limits.pcm_ring_frames`. It already charges
  `ring_frames * source_channels * sizeof(f32)` per declared source. Issue 010 resolves the opaque
  asset and validates observed identity/rate/channels/region without changing TOML.
- Issue 006 owns stable graph order, `TrackStage::Input`, prepared binding ownership, exact PDC and
  graph resource accounting. Source audio has zero algorithmic latency and does not alter graph
  topology, schedule, PDC, reductions, canonical bytes/hash or output order.
- Issue 032 owns the four launch rates. A source rate is a lossless metadata carrier, but issue 010
  rejects any source/engine mismatch before publication. Do not add SRC or reinterpret an extended
  compatibility rate as launch support.
- Issue 005 forbids PCM in control frames. A semantic transport locate may reach the source
  controller, but source audio and host chunks use only the in-process ring boundary.
- Issue 002 supplies checksum/tolerance conventions, not a required production dependency. Keep
  the source fixture oracle independent from the decoder under test and do not add a second
  general conformance framework.

## Production and dependency boundary

Add only `crates/miso-engine-source` / `miso_engine_source`. It may depend on core, session and
graph; graph must not depend on it. Native file/resolver/worker modules are cfg-excluded from
browser Wasm. Use no external dependency. Add only the narrow additive source-set trait/binding
and after-disarm telemetry-copy seam to graph/core that this feature needs. Update workspace,
realtime/source dependency policy and target commands accordingly.

Do not edit the session schema, control wire, effects, builtins, racks, scheduler algorithms,
hosts, C ABI or release tooling. Existing ordinary input-node bindings remain functional for
tests/external input. A sealed source set and an ordinary `GraphNodeBinding` may not both claim the
same track input.

## Frozen prepared ownership and interfaces

Use these semantic public types; field naming may follow existing Rust style but the ownership and
units may not change:

```text
SourceGeneration(u64)                 // zero invalid
SourceFrame(u64)                      // absolute frame in decoded source
SourceCommand::Seek { generation, frame }
PcmSourceRingConfig {
    channel_count, quantum_frames, frame_capacity, initial_generation
}
PcmSourceRing::prepare(config) -> (PcmSourceProducer, PcmSourceConsumer, SourceResourceReport)
HostPlanarChunk<'a> {
    sample_rate_hz, generation, start_frame, planes, frames, end_of_region
}
HostChunkProvider::submit(chunk) -> Result<SubmitReport, HostChunkError>
```

`frame_capacity` is `limits.pcm_ring_frames`, at least one quantum and exactly divisible by it.
Prepare exactly `frame_capacity / quantum` transfer blocks, each holding
`channel_count * quantum` planar `f32` samples. A data SPSC moves filled blocks to the render owner;
a recycle SPSC returns the same blocks to the sole producer. Boxes/queues are created and
destroyed off render, and render only moves ownership, copies samples and updates owner-local
saturating counters. A full submit returns all caller data logically unconsumed; it never accepts
a prefix. The only short block is a tagged final region block or zero-frame discontinuity marker.

Producer and consumer expose exact shape/capacity, active generation, cumulative frame
write/read cursors, full/empty counts, stale chunk count, sanitized decoded-sample count,
underrun frames/events and end-of-region state. Cursors count accepted/consumed audio frames, not
queue slots, and saturate at `u64::MAX`. One underrun event is one maximal missing in-region run
within a render call; every missing frame increments `underrun_frames`. EOF/declared-region zeros
do not increment underrun. All silence written by the source layer is positive zero.

`SourceController::try_seek(SourceCommand)` validates a strictly increasing nonzero generation,
publishes it through bounded prepared control state and wakes only the off-render native worker.
The render coordinator observes at most the prepared bounded command count at block entry, switches
generation only there, discards older transfer blocks within the prepared ring bound, and emits
zero until the matching absolute frame is available. Native/mobile atomics, if used for the
generation notification, must be proven lock-free and must not introduce atomics into baseline
Wasm; the browser single-agent implementation uses accepted local-ring/host mediation.

## Source-set graph seam

Add a graph-owned object-safe `GraphPreparedSourceSet: Send` with immutable claimed input-node
metadata, `begin_block(first_sample, frames)`, `copy_track_input(node, left, right)`, exact resource
report and bounded after-disarm telemetry copy. `GraphRuntimeBindings` accepts at most one sealed
source set. Bind requires its claimed nodes to be a sorted, duplicate-free subset consisting only
of required `TrackStage::Input` nodes; the union of source claims and ordinary node bindings must
equal the graph's required bindings exactly.

`begin_block` runs once on the coordinator before any graph node or native wave. It pulls each
declared source once into preallocated source planes. `copy_track_input` applies only the accepted
session's left/right channel indices and may be called once for every claiming input. The native
executor copies these completed dual-mono inputs into the owning jobs before dispatch; auxiliary
workers never touch a ring or source-set state. Thus one source may fan out to any configured
number of tracks without duplicate rings, decoding or mutable sharing.

The source set is sealed to the accepted session rate/quantum, sorted source declarations, track
mapping and graph input claims. Missing/extra/duplicate/changed mapping, ordinary-binding overlap,
envelope mismatch or resource-report mismatch returns the graph, source set and ordinary bindings
unconsumed with `source.graph.binding_mismatch`. Plan retirement owns worker stop/join and final
buffer destruction off render.

## Resolution, caps and resource accounting

The locator and identity formats remain opaque. A native `SourceResolver` runs off render and
returns an opened seekable asset plus an observed identity. Compare identity byte-for-byte. Parse
metadata before starting playback and require observed sample rate/channel count and the declared
region to match. Resolution or any later preparation failure returns a sorted diagnostic set and
no source set or plan; opened files/workers are closed/stopped off render.

Freeze explicit nonzero `SourcePrepareCaps` for open sources, parser chunk count, skipped metadata
bytes, command items, total retained source bytes, largest allocation and fixed worker-read bytes.
All count/multiply/add/offset conversions are checked against `u64`, `usize` and `isize`. The
source report separates:

- planar PCM payload already charged by issue 004;
- data/recycle/control queue payload and headers;
- transfer-block metadata, source cache, worker read scratch and parser metadata;
- total/largest engine-owned allocations and combined session+graph+source bytes.

The graph bind checks that reported PCM payload equals issue 004's declared source-ring bytes,
adds overhead exactly once, and rechecks compile caps plus `limits.memory_bytes`. Allocator headers,
page cache and RSS are reported separately and are never invented as exact engine-owned bytes.
No count depends on source duration and there is no compiled source/track maximum.

## Native parser, decode and worker contract

Accept only little-endian RIFF/WAVE and RF64/WAVE with one `fmt ` and one `data` payload. RF64 must
have a valid bounded `ds64` before data. Honor even-byte chunk padding; skip unknown chunks with
seek/read scratch rather than retaining their payload. Classic PCM (`0x0001`), IEEE float
(`0x0003`) and extensible (`0xfffe`) with exact PCM/float GUID are allowed. Extensible valid bits
must equal container bits. Accepted encodings are U8, S16LE, S24LE, S32LE, F32LE and F64LE.

Require consistent channel count, bits/container, block align, byte rate, data-frame divisibility,
RF64 sizes and declared region. Reject RIFX, compression, multiple `fmt `/`data`, unsupported GUID,
truncation/overflow and cap excess with the frozen source diagnostic registry.

Decode U8 as `(x - 128) * 2^-7` and signed N-bit PCM as `x * 2^-(N-1)`. Preserve finite normal
F32 bits and either signed zero. Round finite F64 once to nearest-even F32. Replace non-finite,
subnormal or out-of-F32-range results with positive zero and count them. Do not dither, clamp valid
finite PCM, infer speaker roles or resample.

One worker owns one file/decoder and one producer. It fills every available recycled quantum up to
ring capacity, checks commands before and after blocking reads, and discards decoded work whose
generation became stale. When idle/full it may wait only on the worker thread. Start, initial
prefill, seek, stop, wake and join never occur in render. A read error becomes bounded zero/
underrun behavior plus an off-render diagnostic; panic is not a recovery contract.

## One fixture framework and representative proof

Use one sorted checksummed `fixtures/sources/v1` manifest and one generator/checker. Its valid
matrix covers RIFF/RF64, classic/extensible, all six encodings, mono/stereo/multichannel,
odd-padded unknown chunks, finite boundaries, signed zero and sanitation values. Expected decoded
bits are stored/generated by an oracle that cannot call production parsing or conversion. Invalid
cases mutate header/chunk/size/format/cap classes. Generate sparse one-minute and multi-hour files
during tests; do not check in duration-sized PCM.

The minimum gates are exactly the seven acceptance groups in Issue 010: format/conversion;
resolution/rates; ring/seek; one-ring multitrack fan-out; one 100,000-render delay/underrun audit;
duration-independent allocation layouts plus descriptive RSS; and native/cross-target/policy
checks. Reuse current graph and realtime audit facilities rather than creating a general source
qualification tool.

## Ordered stop conditions

FAIL immediately for duplicate rings per track; one-track-per-source restriction; render or
worker shared mutable state; source consumption by auxiliary graph workers; wrong/old-generation
audio after a seek boundary; partial silent host acceptance; nonzero underrun samples; EOF counted
as underrun; duration-scaled retained PCM; double-charged or uncharged memory; implicit SRC;
protocol PCM; render allocation/free/lock/I/O/log/syscall; Wasm filesystem/thread/atomic baseline
reachability; parser/catalog/host expansion; any timed benchmark; or a fourth attempt. Preserve
evidence and split codecs, platform runtime, parser expansion, benchmark tooling, streaming
optimization and extended fault research into later stateless issues.
