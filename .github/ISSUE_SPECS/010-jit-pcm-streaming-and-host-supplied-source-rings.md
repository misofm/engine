# 010 JIT PCM streaming and host-supplied source rings

## Outcome

Provide one bounded just-in-time PCM stream per declared source, shared by every track that maps
that source, so stem duration never determines retained engine memory.

## Context

Engine V2 is a greenfield Rust, agent-first mixing/mastering engine. Never inspect, copy,
benchmark, or inherit V1/legacy work. The realtime plane exclusively owns a preallocated
`PreparedRenderPlan`: graph/schedule/capacities are immutable while its DSP state is mutated only
through exclusive render ownership. Render performs no allocation/free, locks, file/network I/O,
logging, syscalls, structural plan mutation, or data-dependent unbounded work; displaced plans are
retired and freed off-thread. There is no compiled track limit. Audio is planar `f32`; dual-mono
L/R state and parameters are independent unless an explicit link mode or smoothed 2x2 matrix
declares otherwise. Launch-supported session/render rates are exactly 44,100, 48,000, 88,200, and
96,000 Hz; 176,400, 192,000, 352,800, and 384,000 Hz are extended compatibility evidence only.
Source/engine mismatches have no implicit SRC. Output is PCM.

The accepted graph currently binds each track input independently, while the accepted session
schema permits several tracks to map different channel pairs from one source. Issue 010 must not
silently turn that fan-out into duplicate per-track rings or impose a one-track-per-source launch
restriction. Its additive graph source-set seam pulls each source once on the render coordinator,
then copies from preallocated source planes to the mapped track-input buffers. Native dependency
waves never share or consume a source ring from worker threads.

This issue is independently implementable only after its exact dependencies are complete. Its
change must follow the Sol-approved brief -> Terra attempt 1 with evidence -> Sol adversarial
review workflow; Sol may make at most two further revisions, then the work must be
rescoped/rebriefed rather than weakening gates.

## Scope

Add one `miso-engine-source` crate; native RIFF/WAVE and RF64 metadata parsing and fixed-buffer
decode workers; one bounded planar SPSC source ring per declared source; generation-tagged seeks;
deterministic prefetch, end-of-region and underrun behavior; an equivalent explicit-rate
host-chunk producer for mobile/browser embedding; transactional source resolution/preparation; and
the narrow graph source-set binding needed for shared-source fan-out.

## Required public interfaces/contracts

`PcmSourceRing` prepares ownership-split producer/consumer endpoints and exposes exact channel,
quantum and frame capacity, active generation, cumulative frame write/read cursors, typed
backpressure, underrun frames/events and stale-generation discard counts. Storage and all transfer
blocks are allocated before render. `SourceCommand::Seek { generation, frame }` requires a
strictly increasing nonzero generation. A seek becomes audible only at a block boundary; after the
new generation is requested, older chunks are discarded and missing new-generation frames are
zero, never old audio.

`HostChunkProvider::submit(HostPlanarChunk<'_>)` accepts borrowed planar `f32`, an explicit source
rate, generation and absolute source-frame start only outside render. It rejects wrong rate,
channel/plane shape, noncontiguous position, stale generation and full capacity without silently
accepting a prefix. Native decode and host submission feed the same prepared ring semantics.

The graph receives one sealed `GraphPreparedSourceSet`, whose immutable track-input claims must
exactly match the session mapping. At block start the coordinator pulls each source once into
preallocated planar source storage and copies the selected channels to every claimed
`TrackStage::Input`. Ordinary graph bindings cannot overlap those input claims. Binding/cap failure
returns all caller-owned inputs and no `PreparedRenderPlan`.

Diagnostics have stable dotted codes and `$.sources[id=...]` paths. At minimum the registry owns
`source.asset.unresolved`, `source.content.identity_mismatch`, `source.rate.mismatch`,
`source.channels.mismatch`, `source.region.out_of_bounds`, `source.container.invalid`,
`source.format.unsupported`, `source.generation.non_monotonic`,
`source.resource.arithmetic_overflow`, `source.resource.limit`, and
`source.graph.binding_mismatch`.

## Frozen native format boundary

Accept little-endian RIFF/WAVE and RF64/WAVE with one `fmt ` and one `data` payload, even-byte
chunk padding and bounded skipped metadata. Accept format tags PCM, IEEE float, and
WAVE_FORMAT_EXTENSIBLE only when its subformat is PCM or IEEE float and valid bits equal container
bits. The sample encodings are unsigned PCM8; signed little-endian PCM16/24/32; and IEEE
float32/64. Unknown chunks are skipped, never retained. Reject compressed formats, big-endian
RIFX, multiple data payloads, malformed `ds64`, inconsistent block-align/byte-rate/data length,
and channel counts or regions that disagree with the accepted session.

Integer conversion is `(unsigned8 - 128) / 2^7` or signed-N / `2^(N-1)`. Finite normal float32 and
signed zero preserve their `f32` bits; finite float64 rounds once to nearest-even `f32`. A decoded
non-finite, subnormal, or finite-f64 value whose conversion is non-finite becomes positive zero
and increments a sanitation counter. There is no dither, gain, channel-role interpretation or
SRC.

## Deliverables

Source crate and graph binding seam; native parser/worker and resolver boundary; planar source
rings, host chunk provider, seek/control ownership, checked source resource report, stable
diagnostics and telemetry; a single checksummed WAV/RF64 fixture corpus including generated sparse
long files; and focused policy/audit tests.

## Explicit non-goals

Full-stem preload; memory mapping a whole data payload; compressed formats; BW64/broadcast metadata
semantics; source-rate conversion; time stretching; network fetch; locator or content-identity
scheme design; control-protocol PCM; render-thread filesystem/decode; lossless recovery after an
underrun; device/browser adapter runtime; a general streaming service; or performance tuning and a
timed benchmark.

## Dependencies by exact issue title

- Real-time memory, buffers, queues, and plan lifetime
- Versioned TOML schema and transactional session compiler
- Deterministic graph compiler, sends, submixes, sidechains, and PDC
- Launch sample-rate scope: 44.1–96 kHz and extended-rate deferral

## Hazards/decisions

The control protocol remains PCM-free; transport locate can cause a control-plane seek but PCM
chunks never become issue-005 frames. Native file operations, resolver calls, parsing, decode,
thread start/stop/join and telemetry aggregation stay outside render. EOF or the end of the
declared region emits zero without claiming an underrun; an unavailable in-region frame emits
exact positive zero and increments saturating underrun-frame and underrun-event counters. Browser
baseline has no Rust filesystem or thread claim and uses the host-supplied local-ring contract.

The session's `pcm_ring_frames` is the exact per-source planar payload capacity and must be at
least one quantum and an integer multiple of the quantum. Source preparation checks all
channel/frame/byte arithmetic, exact retained engine payload, largest allocation, parser metadata,
worker scratch, queue items and combined session+graph+source bytes before binding. Ring payload
bytes already charged by issue 004 are not charged twice; all source overhead is charged exactly
once. There is no duration term and no compiled source/track ceiling.

## Acceptance gates with objective measurements

1. A checksummed valid corpus covers both RIFF and RF64, classic and extensible headers, all six
   advertised encodings, mono/dual-mono/multichannel order, odd unknown-chunk padding and regions.
   Expected `f32` bits come from an independent fixture oracle; every nonsanitized output is exact
   where representable and otherwise within 0.5 ULP. Malformed container/format/cap mutations
   return the exact diagnostic and never panic or allocate from declared data length.
2. Resolution compares declared and observed content identity, rate, channel count and region.
   Every launch rate succeeds when source and engine match; every mismatch, including an extended
   compatibility source against a launch session, returns `source.rate.mismatch` and no source set
   or publishable plan.
3. Capacity-one-quantum, wraparound, full/empty and end-of-region tests prove FIFO PCM and typed
   ownership return. Randomized seek races use a frozen seed, strictly increasing generations and
   adversarial delayed old chunks; no sample from an older generation renders after the newer
   request boundary, and stale chunks are counted.
4. One four-channel source fans out through one ring to at least three track inputs with repeated
   and crossed channel selections. Direct sequential and prepared native-fallback graph renders
   produce the exact mapped PCM and consume each source frame once. Missing/extra/duplicate or
   overlapping source claims reject transactionally.
5. Injected worker delay during exactly 100,000 48-kHz/128-frame renders never blocks render.
   Every unavailable in-region sample is positive zero, counters equal the exact missing frame and
   contiguous-event counts, and resumed PCM begins at the declared next frame. The existing render
   audit reports zero allocation/free, lock, log, file/network I/O, syscall and structural change.
6. A sparse multi-hour source and a one-minute source prepared with identical channel/ring settings
   have byte-identical engine allocation-layout multisets and equal `SourceResourceReport` values.
   File size appears only in metadata/region counters; fixed read scratch and retained PCM do not
   scale with duration. RSS is recorded as descriptive OS evidence, not substituted for the exact
   engine-owned allocation gate.
7. Locked native workspace tests/check/Clippy/rustdoc and workspace/realtime/source dependency
   policies pass. Native worker/runtime tests run on Linux. The source/graph surface compiles for
   Android ARM64, iOS ARM64, Wasm scalar and Wasm `simd128`; Wasm links no filesystem worker and
   the baseline local source path introduces no atomic opcode. Device and browser runtime remain
   owned by issues 023 and 024.

## Target matrix

Linux/cloud native owns RIFF/RF64 parsing, decode workers and the shared SPSC path. iOS and Android
compile the SPSC plus host-chunk path but do not claim device decode integration here. Browser
Wasm scalar and `simd128` compile the single-render-agent local ring and host-chunk path with no
Rust filesystem, worker-thread or shared-memory requirement.

## Required evidence

Public API/resource-report diff; fixture manifest and decoded PCM hashes; invalid diagnostic
matrix; seek-race and fan-out transcripts; exact allocation layouts plus descriptive RSS records;
underrun/EOF trace; 100,000-render realtime audit; native and cross-target logs; source dependency
policy results; and explicit benchmark invocation count **0**.

## Sol-approved implementation boundary (2026-08-21)

The bounded contract above is approved for Terra attempt 1. Use the companion stateless brief in
`BRIEFS/010-jit-pcm-streaming-and-host-supplied-source-rings.md`. Keep one source crate, one fixture
framework and one render audit. Do not add a benchmark, PCM protocol message, host adapter, codec,
SRC, broad parser catalog or generic qualification tool. Split any such need into a later issue.

## Terra attempt 1 evidence (2026-08-21)

**Status: PASS — ready for Sol adversarial review. Benchmark invocation count: 0.**

- The single sorted `fixtures/sources/v1` manifest/checker passed, including RIFF/RF64,
  classic/extensible, all six scalar encodings, odd padding, independent `f32`-bit oracles, and
  malformed header/format/cap/duplicate-data mutations.
- Native source preparation tests cover identity/rate/channel/region rejection, fixed caps,
  one-source/three-track repeated-and-crossed 4-channel fan-out, transactional source-set binding,
  and rollback after a later resolver failure. Ring tests cover capacity-one-quantum, wraparound,
  full/empty, EOF, and generation-tagged stale discard behavior.
- `miso-engine-source-audit` ran once as functional evidence (not a benchmark): exactly 100,000
  48-kHz/128-frame reads; one injected unavailable block produced 128 positive-zero underrun
  frames and one maximal event; a generation-tagged source resumed at frame 256; output storage
  address stayed fixed; allocation, free, lock, log, file-I/O, network-I/O, and syscall counters
  were all zero. The one-minute and sparse-multi-hour labels use the identical fixed source
  settings and produced equal exact retained resource reports; retained engine layout has no
  duration input. `descriptive_rss_bytes` is deliberately `null` in this headless runner and is
  not substituted for the exact engine-owned report.
- Locked quality gates passed: workspace check/test, `-D warnings` Clippy, warning-denied rustdoc,
  and workspace formatting. Realtime policy and its mutation tests passed. The Wasm local-source
  opcode inspection now includes `miso-engine-source` and found no atomic opcode.
- Source+graph compile logs passed for Android ARM64, iOS ARM64, Wasm scalar, and Wasm `simd128`.
  The native parser/worker modules are cfg-excluded on Wasm; no device/browser runtime claim is
  made. No timed benchmark, timing threshold, codec, SRC, protocol PCM, or host runtime was added.
