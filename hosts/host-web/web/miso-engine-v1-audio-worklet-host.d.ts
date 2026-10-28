// Browser AudioWorklet host, ABI version 1.
//
// # The live controls (issue 137)
//
// V1 as issue 024 froze it was a deterministic *renderer*: create, stream, render, dispose. Issue
// 137 adds the three things a live mixing console needs on top, additively -- no existing message,
// field or live result code changed meaning, and the resource report keeps its 224-byte layout.
//
//   1. `control()`, the engine's one command entry from the main realm (issue 1562a), which every
//      live value and every observation subscription goes through.
//   2. `miso.meters.v1` / `miso.meter.v1`, a decimated peak stream on a lease.
//   3. `miso.telemetry.v1`, windowed render-time telemetry on a lease, measured in JavaScript
//      around the render export.
//
// ## One command entry
//
// **This is the most important thing to know before writing an app against it.** The browser has
// no command record of its own (issues 1565 and 1589). Every live value -- a fader, a mute, a pan
// or matrix, an input trim, polarity or filter, a send, a VCA, a solo, an effect parameter or
// bypass -- moves through `control()`: a `SESSION_TRANSACTION_APPLY` carrying `SET_CONTROL` edits,
// the same command frame the C ABI's `miso_engine_v1_submit_command` takes, decoded and classified
// once by the engine's control plane. An observation tap is armed and disarmed the same way, by an
// `Observe` edit in a `SESSION_TRANSACTION_APPLY`, and read with an `OBSERVATION_READ` frame: the
// edit names its tap by the stable key the engine's `OBSERVATION_ENDED` event uses (the strip's ID,
// the insert's or the console slot's ID, and the effect-local tap ID), and the reply carries its
// arm tag. Every plan binds a queue for every declared tap of every effect (issue 1590), as on the
// C ABI.
//
// ## What metering costs
//
// Every plan meters every strip (one post-matrix peak pair per strip, tracks then submixes) and the
// output. `liveControlMeterBlocks === 0n` gives the strip meters the engine's default window (about
// 32 ms) and delivers nothing: `meters({ enabled: true })` is refused with `RESULT_UNSUPPORTED`
// rather than reporting zeros, and the output fold stays shut at one load per block. A nonzero
// value sets a `blocks * quantumFrames` window and makes the port lease a second, finer switch
// over the output fold and every drain and post. The honest summary: releasing the lease, or
// never taking it, costs one load per block plus the per-strip observation fold, which runs on
// every plan.
//
// Gain reduction **is** in the meter frame, since issue #143. The frame carries `trackGrDb` --
// one non-negative decibel magnitude per track -- and `masterGrDb`. These are latest folds from
// independently aged effect observations; their sequence/span is not transported in this frame,
// and they never supply or inherit the `firstSample`/`endSample` peak timestamps. The additive
// `generation`, `validity` and `lossCount` fields make lease resets, producer resets and dropped
// peak windows visible without changing the peak array.
//
// The gain-reduction section is recomputed from the taps armed when each frame is polled. A
// disarm is in effect from the block render applies it, so every meter frame polled after that
// block leaves that tap out. An arm, also one that re-arms an armed tap, restarts its tap's window
// at the block render applies it (the arm sample an `OBSERVATION_READ` row reports), and the frame
// leaves that tap out until a window that opens at or after that sample closes. A strip's slot
// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at or
// after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is `null`
// when the designated strip has none.
//
// What a tap costs is declared, not guessed. Every effect publishes an `observations` menu in the
// build-time metadata JSON; a `resident` tap is a copy out of state the block already wrote and is
// `subscribable`, a `computed` tap is an analysis pass that does not ship in V1 and is refused by
// the engine's `Observe` edit. An unarmed tap costs one predicted branch per driven effect per
// block.
//
// ## What a frame costs the render callback
//
// One `postMessage` per window, not per block. The body is a frozen object allocated at
// construction whose four arrays are a `Float32Array` of `2 * trackCount + 2` peaks, one of
// `trackCount` gain-reduction magnitudes, and the buses' `2 * submixCount` peaks and
// `submixCount` magnitudes, all allocated at construction and overwritten in place; nothing is
// transferred, so the caller keeps no ownership obligation. The counts are the plan's: a structural
// apply that adds or removes a strip gives its successor a frame of its own size, whose body and
// views are made in the message handler that committed it (never in the render callback), and the
// first window the successor closes is published in it, in the same stream (`sequence`,
// `generation` and `lossCount` continue). The single allocation left is the
// structured clone `postMessage` performs, which is `4 * (3 * (trackCount + submixCount) + 2)`
// bytes plus a handful of small numbers -- 392 bytes for a 32-track console with no buses. That cost has not been separately benchmarked in a browser; the telemetry
// lease measures the render export, not the post around it.
//
// # Exactly one artifact
//
// Owner decision W4-D1: the shipped module is built with `+simd128`. There is no scalar artifact
// and no dual-artifact selection. `createMisoAudioWorkletHost` validates a canned `simd128` module
// with `WebAssembly.validate` before it fetches anything and rejects with a typed
// `MisoUnsupportedBrowser` (`tag: "miso.unsupported.v1"`, `capability: "simd128"`) when the probe
// fails -- the browser twin of the native x86-64-v3 boot attestation. A browser below the floor is
// refused, never silently degraded.
//
// # Trap means processor death
//
// `wasm32-unknown-unknown` is `panic = abort`. A Rust panic inside the worklet aborts the Wasm
// instance; there is nothing to catch inside Rust and no `catch_unwind` exists on this path.
// `process()` converts a throw from the render export into sticky `RESULT_INTERNAL` (255) and
// positive-zero output, and the user agent may also fire `processorerror`. Any other export that
// throws settles the pending request as `miso.error.v1` with result 255.
//
// # A render failure never frees
//
// `RESULT_RENDER_REJECTED` (8) and `RESULT_INTERNAL` (255) from `process()` are sticky: the engine
// keeps the render plan, compiled session and source rings alive in its one-slot retirement queue
// and emits positive-zero silence. `dispose()` -- delivered on the port, never inside `process()`
// -- is the single point at which that storage is freed.
//
// # Streaming model
//
// Sources stream just-in-time into bounded per-source rings. One message carries exactly one
// quantum of planar PCM (the final chunk of a region may be shorter). Up to
// `sourceRingFrames / quantumFrames` chunks may be unsettled per source ID at once -- there is
// nowhere for a further chunk to go -- plus one unsettled seek per source. With a zero
// `sourceRingFrames` the bound is the ring the engine reports it resolved at boot (the default ring
// below; the engine's `miso_engine_web_v1_resolved_source_ring_frames`). A request over its bound
// rejects locally with `RESULT_BACKPRESSURE` (6) before any transfer, so the caller keeps its planes
// and can retry. Status is never refused for being in flight: see `status()`.
//
// The default ring (a zero `sourceRingFrames`) covers a 100 ms main-thread stall and then holds a
// warm successor's largest prime (issue #1406): in quanta, `ceil(100ms * fs / quantum) + 2 +
// 4 * ceil(L_max / quantum) + 1`, with `L_max` the true-peak limiter's `fs / 100 + 6`. That is
// 7 296 frames (57 KiB for stereo `f32`) at 48 kHz with a 128-frame quantum. The ABI layout's
// `sourceRing` publishes the rule. It is a prefill ahead of the render position and adds no output
// latency.
//
// Two options deliberately not taken, recorded so they are not rediscovered as bugs:
//
//   1. Zero-copy submission (`source_reserve` / `source_commit`) and a chunk size decoupled from
//      the quantum. Both need a `source` contract change, which is that crate's issue
//      (#101); today `validate_submission_metadata` requires exactly one quantum unless the chunk
//      ends the region.
//   2. A `SharedArrayBuffer` ring under `crossOriginIsolated === true`, which would remove the
//      message round trip entirely. It requires the engine's atomics-free policy to be revisited
//      and is an owner decision, not a host change.
//
// # Compilation happens on the rendering thread
//
// Session JSON is compiled inside the `AudioWorkletProcessor` constructor, which runs on the
// rendering thread before the first `process()` call. This is documented V1 behaviour (issue 024,
// owner open question 2): construction allocates, later rendering does not.
//
/// The frozen backend names of the issue-024 ABI. Only `"simd128"` is shipped (W4-D1); `"scalar"`
/// remains a legal ABI value because the Rust artifact still reports backend `0` when it is built
/// without `+simd128`, and the processor rejects such a module rather than rendering with it.
export type MisoWebBackend = "scalar" | "simd128";

/// Typed refusal of a browser that cannot run the shipped artifact.
///
/// Distinct from `MisoError` on purpose: a caller must be able to tell "this browser is out of
/// scope" apart from "something went wrong". It is thrown by `createMisoAudioWorkletHost` before
/// any node exists and never crosses the `MessagePort`.
export interface MisoUnsupportedBrowser {
  readonly tag: "miso.unsupported.v1";
  readonly requestId: 0;
  readonly result: 7;
  readonly capability: "simd128";
}

/// One compiled source, as the engine declares it (issue 207).
///
/// Named without a version suffix on purpose: issue 215 removes all version scoping at this
/// sprint's close, and a type minted now has nothing pinned against it to break.
///
/// This is what the *compiled* session knows about a source -- not what a decoder will eventually
/// find in a file. Every source is declared at the session rate and spans its full content from
/// frame zero, so only its channel count and total frame count belong in the per-source map.
export interface MisoSessionSource {
  /// Stable source ID -- the string `submitSource`/`seekSource` address the source by.
  readonly id: string;
  /// Declared source channels. Always at least one, and never above `maximumSourceChannels`.
  readonly channels: number;
  /// Full content length in source sample frames. Always at least one.
  readonly frames: bigint;
}

/// The compiled session's addressing authority (issue 137 D1, extended by issue 207).
export interface MisoSessionMap {
  readonly tag: "miso.sessionmap.v1";
  readonly requestId: number;
  readonly result: number;
  /// Canonical normalized source order (issue 207).
  ///
  /// The engine's own order: session compilation sorts `sources` by stable ID, and the queries
  /// behind this list read that sorted model rather than a copy of it. This is what makes a
  /// session compiled from raw JSON drivable -- the shapes a render loop must feed are here, and
  /// there is nowhere else in the browser ABI to learn them.
  readonly sources: MisoSessionSource[];
  /// Whether preparation bound meter observers at all.
  readonly metersAttached: boolean;
}

/// One decimated meter window (issue 137 D2, extended by issue 143).
export interface MisoMeterFrame {
  readonly tag: "miso.meter.v1";
  readonly sequence: number;
  /// Publication generation; changes on each real lease transition or detected producer reset.
  readonly generation: bigint;
  /// Header validity bits: complete (`1`), master aligned (`2`), loss (`4`), gain reduction (`8`).
  readonly validity: number;
  /// Saturating count of dropped or rejected meter windows observed before this frame.
  readonly lossCount: number;
  /// Complete windows folded into this frame; normally `1`.
  readonly windows: number;
  /// The tracks of the plan that rendered the window. A structural apply that adds or removes a
  /// track changes it between two frames of one stream; every array below is sized for it.
  readonly trackCount: number;
  /// The structure epoch of the control list that the plan's strip order belongs to (issue
  /// 1562b): the `structureEpoch` that `CONTROL_LIST_GET` pages carry, as the commit that prepared
  /// the plan left it. The frame's strips (the tracks, then the submixes) are named by the list
  /// of this epoch, so a page that holds a list of another epoch reads the list again before it
  /// names a frame's strips. A structural apply that adds, removes or reorders a strip changes it
  /// between two frames of one stream, in the frame of the block that adopted the plan, and a
  /// replacement that keeps the structure keeps it. A positive safe integer.
  readonly structureEpoch: number;
  /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R]` peak magnitudes.
  readonly peaks: Float32Array;
  /// Gain reduction per track, in **non-negative decibels**, `trackCount` long (issue 143).
  ///
  /// Non-negative because the tap declares a `peakMagnitude` fold, not because the app clamps it:
  /// an effect holds its reduction as negative decibels internally, and the fold is `max(|x|)` over
  /// the window. An app's `Math.max(0, x ?? 0)` is therefore a no-op rather than a silent zeroing.
  /// A `gainChangeDb` tap (`peakSigned`, issue 1918) is signed, positive for a boost: it adds only
  /// a cut's depth here and a boost adds `0`; its signed value is read with `OBSERVATION_READ`.
  ///
  /// Every entry is finite. `0` deliberately conflates "not reducing" with "no observed effect on
  /// this track", because the array is positional and read without null checks; the distinction
  /// lives in the `OBSERVATION_READ` rows. Several armed taps on one
  /// track fold max-magnitude into the one slot, on the control plane. Each value is the latest
  /// available independently aged effect fold; this frame carries no GR sequence or sample span.
  ///
  /// The gain-reduction section is recomputed from the taps armed when each frame is polled. A
  /// disarm is in effect from the block render applies it, so every frame polled after that block
  /// leaves that tap out. An arm, also one that re-arms an armed tap, restarts its tap's window at
  /// the block render applies it, and the frame leaves that tap out until a window that opens at or
  /// after that sample closes. A strip's slot
  /// (`trackGrDb`, `submixGrDb`) is the maximum over its armed taps' latest windows that opened at
  /// or after their last subscribe, and `0` when no armed tap has such a window. `masterGrDb` is
  /// `null` when the designated strip has none.
  readonly trackGrDb: Float32Array;
  /// The designated master strip's own folded reading, or `null` (issue 143 D6). The master is the
  /// strip whose `master` monitor row is on (issue 1683).
  ///
  /// `null` -- never `0` -- when no strip was designated or the designated strip published no
  /// window, because `0` would be indistinguishable from "the master is not reducing".
  readonly masterGrDb: number | null;
  /// Absolute sample the reported window opened at, inclusive.
  ///
  /// This timestamps only `peaks`. Correlate it against an arm sample (an `OBSERVATION_READ` row's
  /// `arm_sample`): the first peak window whose `firstSample` is at or after it is the first wholly
  /// after the arm. A generation change starts a fresh delivery epoch. Within one generation, a gap
  /// between this frame's `endSample` and the next frame's `firstSample`, or the loss bit/count,
  /// reports omitted/rejected telemetry rather than a contiguous measurement.
  readonly firstSample: bigint;
  /// Absolute sample the reported window closed at, exclusive.
  readonly endSample: bigint;
  /// Submix strips (buses) this frame carries, in canonical submix order (issue 1209).
  readonly submixCount: number;
  /// `[bus0 L, bus0 R, .., busM L, busM R]` post-matrix peak magnitudes, `2 * submixCount` long,
  /// timestamped by `firstSample`/`endSample` exactly as `peaks` is.
  readonly submixPeaks: Float32Array;
  /// Gain reduction per bus, in non-negative decibels, `submixCount` long; every entry finite. It
  /// follows `trackGrDb`'s rules: positional, `0` for "not reducing" or "no observed effect",
  /// independently aged.
  readonly submixGrDb: Float32Array;
}

/// One windowed render-telemetry frame (issue 137 D3). JavaScript only; Wasm never sees the lease.
export interface MisoTelemetryFrame {
  readonly tag: "miso.telemetry.v1";
  readonly sequence: number;
  /// Blocks in the window.
  readonly blocks: number;
  /// Render time as a percentage of the block budget over the window.
  readonly cpuPercent: number;
  readonly peakBlockMs: number;
  readonly meanBlockMs: number;
  readonly budgetMs: number;
  /// Blocks whose measured render time exceeded the block budget.
  readonly deadlineMisses: number;
  /// Resolution of the clock the worklet actually found.
  readonly resolutionMs: number;
  /// `true` when the window measured exactly zero -- the clock could not see the work.
  readonly belowResolution: boolean;
}

export interface MisoWebBootOptions {
  /// Per-source ring override, or `0` for the default ring: in `single` mode the engine's (the
  /// 100 ms stall ring plus the largest warm-successor prime plus one quantum, issue #1406), in
  /// `worker` mode 65 536 frames (issue 1387: the control Worker's timer-driven feed drain needs
  /// the engine ring to ride out a render burst).
  sourceRingFrames: number;
  /// Total boot memory budget, or `0n` for the engine-owned default ceiling.
  /// A budget above the module's shared-memory maximum (the ABI layout's `memory.maximumPages`
  /// pages, 512 MiB) is refused with `web.boot.memory_maximum` (issue #1380).
  maximumMemoryBytes: bigint;
  /// The control frame in bytes, or `0n` for the engine's default, 256 KiB (offset 32 of the boot
  /// options, issue 2191). It bounds every control frame, response and event; the engine refuses
  /// a nonzero value below 16 KiB or above 16 MiB with `web.options.control_frame_bytes`. The host
  /// reports the frame the engine resolved as `MisoAudioWorkletHost.controlFrameBytes`.
  maximumControlFrameBytes: bigint;
  /// Meter window in render blocks, or `0n` for the engine's default window with no delivery
  /// (issues 137, 1581).
  liveControlMeterBlocks: bigint;
  /// Reserved (offset 48 of the boot options): must be exactly `0n`. It was the observation-taps
  /// word until issue 1590 retired it: every plan binds a queue for every declared tap of every
  /// effect, as on the C ABI, so the session always serves the `Observe` edit and
  /// `OBSERVATION_READ`.
  reserved3: bigint;
  /// Reserved (offset 56 of the boot options): must be exactly `0n`. It was the master boot word
  /// until issue 1689 retired it: the app designates the master with the strip's `master` monitor
  /// row (`SET_CONTROL`), as on every host, and the core holds it by strip ID. With no tap on the
  /// strip `masterGrDb` is `null`.
  reserved2: bigint;
}

export interface MisoWebResourceReport {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly backend: number;
  readonly optionsBytes: bigint;
  readonly statusBytes: bigint;
  readonly sessionDocumentBytes: bigint;
  readonly diagnosticBytes: bigint;
  readonly idStagingBytes: bigint;
  readonly sourcePcmStagingBytes: bigint;
  readonly outputPcmBytes: bigint;
  readonly bridgeMetadataBytes: bigint;
  readonly bridgeRetainedBytes: bigint;
  readonly largestBridgeAllocationBytes: bigint;
  readonly sourceTotalBytes: bigint;
  readonly sourceOverheadBytes: bigint;
  readonly effectScalarStateBytes: bigint;
  readonly effectScalarScratchBytes: bigint;
  readonly builtinRetainedBytes: bigint;
  readonly graphSessionPlusPlanBytes: bigint;
  readonly graphIncrementalPlanBytes: bigint;
  readonly graphMetadataBytes: bigint;
  readonly graphDelayBytes: bigint;
  readonly largestNamedAllocationBytes: bigint;
  /// Engine-owned bytes the plan's observation lanes and conflating cells retain (issue 143).
  ///
  /// Walked over the built runtime rather than computed from the configuration; `0n` for a session
  /// with no effect that declares a tap.
  readonly observationRetainedBytes: bigint;
}

export interface MisoStatus {
  readonly tag: "miso.status.v1";
  readonly requestId: number;
  readonly result: number;
  readonly state: number;
  readonly lastResult: number;
  readonly backend: number;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  /// The structural commands this instance rebuilt on the audio thread (issue 1294): a page
  /// without cross-origin isolation runs each one in the worklet's message handler, outside
  /// `process()`, as a blocking rebuild that is counted here. Saturates at `2 ** 32 - 1`.
  readonly blockingRebuilds: number;
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  /// The session's count of live values that a later committed value replaced before a block
  /// applied them (decision 15, D15-2), in the C ABI's unit (`COUNTERS_GET`). Every value reaches
  /// the engine through `control()` (issue #1565), the one path the C ABI takes. A value on a strip
  /// that a plan swap starts at rest is not counted here: a `COUNTERS_GET` sent with `control()`
  /// reports each such strip, once per adopted plan, as `STRIPS_RESTARTED` (protocol counter 18,
  /// issue 1768).
  readonly liveValuesSuperseded: bigint;
  /// The applied-revision watermark's revision (issue 1349): the highest committed revision in
  /// effect together with every revision before it, copied from the engine's render report.
  /// @internal The SDK's `engine.apply` is the public surface.
  readonly appliedRevision: bigint;
  /// The absolute render sample of the first block in which `appliedRevision` was in effect.
  /// @internal
  readonly appliedSample: bigint;
  /// The watermark's outcome flags, the C ABI's bits: `EXACT` 1, `TRANSITION_FALLBACK` 2,
  /// `SUPERSEDED` 4. `TRANSITION_FALLBACK` also reports a rebuild that carries a send following a
  /// stored mute into one that follows none (conservative: it can be set when no send moves at the
  /// swap; #1713 removes it). It also reports any rebuild that carries an automated following
  /// send while a live send move may still ramp (conservative; #1718 removes it).
  /// @internal
  readonly appliedOutcome: bigint;
  /// The module's memory size now: at least the boot size (`MisoAudioWorkletHost.memoryBytes`),
  /// larger after a control call grew it.
  readonly memoryBytes: number;
  /// The allocator calls the instance made outside its render-locked windows (issue 1381a, issue
  /// 1332 D1): its boot, its control messages and its service steps. In this module's `single`
  /// mode the control work runs in the worklet's message handler, so this counts the one place the
  /// engine allocates on an audio thread; the render-locked count (`renderAllocationCount()`)
  /// stays exactly zero.
  readonly singleModeControlAllocations: number;
  /// Always 0 (issue 1332 S3). The module's memory is shared: a growth gives it a new buffer object
  /// while every view over the older one stays valid, so `process()` never silences a quantum for
  /// it. Kept so the status shape does not change.
  readonly staleViewQuanta: number;
  /// The meter frames the worklet skipped in `worker` mode (issue 1332 S7) because render reported
  /// a header for which no matching set had been cut yet: possible only when a plan is adopted
  /// within one control-Worker round trip of its commit. Always 0 in `single` mode.
  readonly meterFramesSkipped: number;
}

/// The instance's render-locked allocator count (issue 1333 D3). Decision 15 requires zero.
export interface MisoRenderAllocations {
  readonly tag: "miso.renderallocations.v1";
  readonly requestId: number;
  readonly result: number;
  readonly count: number;
}

/// The reply to one control frame (issue 1562a D1). `response` is the response frame the engine's
/// control plane built, unchanged, when `result` is `0`; for another result it is the refusal's
/// diagnostic text (`code\tpath\n` lines), for example `web.apply.shape_unavailable` with `result: 10`
/// for a preparation that could not hand its plan to the host. A transaction that needs a plan
/// rebuild is prepared and published like any other (issue 1562b). A refused transaction is still `result: 0` with a non-ok response frame, as on the C ABI.
/// `revision` is the committed session revision after the call.
export interface MisoControlReply {
  readonly tag: "miso.control.v1";
  readonly requestId: number;
  readonly result: number;
  readonly response: Uint8Array;
  readonly revision: bigint;
}

/// One protocol event of the engine's control plane (issue 1570b D5): the frame bytes the engine
/// encoded, unchanged, and the lane it came from, `0` for the reliable lane and `1` for the lossy
/// lane.
export interface MisoControlEvent {
  readonly lane: 0 | 1;
  readonly frame: Uint8Array;
}

/// @internal The applied-revision watermark (issue 1294a), posted each time its
/// `(revision, sample)` pair changes. `revision` is the highest committed revision in effect
/// together with every revision before it, `sample` the absolute render sample of the first block
/// in which it was in effect, `outcome` the OR of the outcome flags over the revisions the advance
/// covered (`EXACT` 1, `TRANSITION_FALLBACK` 2, `SUPERSEDED` 4), and the three counts the
/// saturating, cumulative counts of revisions completed with each outcome.
export interface MisoWatermark {
  readonly revision: bigint;
  readonly sample: bigint;
  readonly outcome: bigint;
  readonly exactCount: bigint;
  readonly transitionFallbackCount: bigint;
  readonly supersededCount: bigint;
}

export interface MisoAck {
  readonly tag: "miso.ack.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoError {
  readonly tag: "miso.error.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoSourceRequest {
  sourceId: string;
  generation: bigint;
  startFrame: bigint;
  sampleRateHz: number;
  planes: Float32Array[];
  frames: number;
  endOfRegion: boolean;
}

export interface MisoSeekRequest {
  sourceId: string;
  generation: bigint;
  sourceFrame: bigint;
}

/// @internal A session seek (issue 1644 D1): the generation every source and the timeline move to,
/// the session frame, and, for the anchored form, the absolute render sample (a quantum multiple)
/// of the block that applies it.
export interface MisoSessionSeekRequest {
  generation: bigint;
  frame: bigint;
  anchorSample?: bigint;
}

/// @internal The reply to one session seek (issue 1644 D1): the engine's result and the generation
/// the request named. A result other than `0` changed nothing.
export interface MisoSessionSeekReply {
  readonly tag: "miso.session-seek.v1";
  readonly requestId: number;
  readonly result: number;
  readonly generation: bigint;
}

/// @internal An anchored source seek (issue 1297a): `sourceFrame` enters the graph in the block
/// that starts at absolute render sample `anchorSample`, a quantum multiple.
export interface MisoSeekAtRequest {
  sourceId: string;
  generation: bigint;
  sourceFrame: bigint;
  anchorSample: bigint;
}

/// @internal Where a source landed after its newest seek (issue 1297a), the C ABI seek report's
/// words: the generation in effect (`0n` until its ring rendered a block), the source-read sample
/// and source frame of that generation's first block, and the cumulative underrun and held frames.
export interface MisoSeekReport {
  readonly generation: bigint;
  readonly firstSample: bigint;
  readonly sourceFrame: bigint;
  readonly cumulativeUnderrunFrames: bigint;
  readonly cumulativeHeldFrames: bigint;
}

/// @internal The reply to one seek report read (issue 1297a): the engine's result and, for `0`,
/// the report; `null` otherwise.
export interface MisoSeekReportReply {
  readonly tag: "miso.seek-report.v1";
  readonly requestId: number;
  readonly result: number;
  readonly report: MisoSeekReport | null;
}

/// The output latency and tail of the plan that renders now (issue 2116): the words of the C ABI's
/// plan resource report. `tailKind` is `0n` for a tail of `tailSamples`, `1n` for an unbounded tail
/// (`tailSamples` is then `0n`).
export interface MisoPlanLatency {
  readonly samples: bigint;
  readonly tailKind: bigint;
  readonly tailSamples: bigint;
}

/// The reply to one plan latency read (issue 2116): the engine's result and, for `0`, the latency;
/// `null` otherwise.
export interface MisoPlanLatencyReply {
  readonly tag: "miso.plan-latency.v1";
  readonly requestId: number;
  readonly result: number;
  readonly latency: MisoPlanLatency | null;
}

/// Where the engine's control plane runs (issue 1332 D1). `"worker"`: on a cross-origin-isolated
/// page, a dedicated Worker boots and edits the session on one shared memory and the worklet only
/// renders; the status's `blockingRebuilds` and `singleModeControlAllocations` read 0. `"single"`:
/// one instance, whose control work runs in the worklet's message handler, never in `process()`.
export type MisoInstanceMode = "worker" | "single";

/// Typed refusal of a control Worker that raised `error` or `messageerror` before its boot reply
/// (issue 1332 D6). The page never falls back to `"single"` mode silently.
export interface MisoWorkerFailed {
  readonly tag: "miso.worker-failed.v1";
  readonly requestId: 0;
  readonly result: 255;
  readonly reason: "worker-failed";
}

/// The typed reason a dispose ended in a teardown nobody answered (issue 1332 S8). The page has
/// terminated the control Worker and disconnected the node. `context-closed` and
/// `dispose-timeout`: no dispose reply within 5 s, on a closed context and on one that is not;
/// `release-timeout` (`"worker"` mode, carrying the dispose's request ID): the worklet did not
/// release its render half within 2 s, and the Worker disposed without it. `requestId` is `0`
/// for the two reasons the page itself finds.
export interface MisoTeardown {
  readonly tag: "miso.teardown.v1";
  readonly requestId: number;
  readonly result: 255;
  readonly reason: "context-closed" | "dispose-timeout" | "release-timeout";
}

export interface MisoAudioWorkletHost {
  readonly node: AudioWorkletNode;
  readonly backend: MisoWebBackend;
  readonly resources: MisoWebResourceReport;
  readonly memoryBytes: number;
  /// The control frame the engine resolved at boot, in bytes (issue 2191): the boot option, or
  /// the engine's default for `0n`. `control` refuses a longer frame.
  readonly controlFrameBytes: number;
  /// Where the control plane runs (issue 1332 D1).
  readonly instanceMode: MisoInstanceMode;
  submitSource(request: MisoSourceRequest): Promise<MisoAck>;
  /// Seeks source content while absolute render time remains continuous. Meter peak windows can
  /// straddle this content discontinuity; the seek does not advance the meter generation.
  seekSource(request: MisoSeekRequest): Promise<MisoAck>;
  /// @internal Seek one source at an anchor sample (issue 1297a); the worklet's message handler
  /// applies it, never `process()`. It resolves with the engine's result. It shares the per-source
  /// bound with `seekSource`: a second unsettled seek of a source rejects locally with result `6`.
  seekSourceAt(request: MisoSeekAtRequest): Promise<MisoAck>;
  /// @internal Read one source's seek report (issue 1297a); `$timeline` reads the timeline's. One
  /// read of a source may be unsettled at a time; a second rejects locally with result `6`.
  seekReport(sourceId: string): Promise<MisoSeekReportReply>;
  /// @internal Seek the timeline and every source under one new generation, all or nothing
  /// (issue 1644 D1). A plain seek (no `anchorSample`) declares the discontinuity in the same
  /// call; an anchored one keeps continuity. Reposition the PCM producers only after a `0` result,
  /// so a ring seek at this generation is consumed without a second per-source seek. One may be
  /// unsettled at a time; a second rejects locally with result `6`. The SDK's `engine.seek` is
  /// the public surface.
  seekSession(request: MisoSessionSeekRequest): Promise<MisoSessionSeekReply>;
  /// Reads the engine status. Concurrent calls share one read, and a reply reflects every request
  /// the host sent before the call: a call joins the in-flight read while no later request was
  /// sent, and otherwise joins the one follow-up read sent when that read settles. A joined read
  /// settles every joined caller with the same reply or the same rejection, and a caller that stops
  /// waiting does not cancel it. `status()` is never refused for being in flight.
  status(): Promise<MisoStatus>;
  /// Read the allocator calls this instance made in the exports its processor calls on the render
  /// thread after boot (issue 1333 D3). Read it on each host before that host's `dispose()`.
  renderAllocationCount(): Promise<MisoRenderAllocations>;
  /// @internal Issue 1387: in `worker` mode, one port to the control Worker's MSB1 feed drain,
  /// on which the SDK's PCM feed speaks its attach protocol; `null` in `single` mode and once the
  /// host failed or a dispose began.
  openFeedPort(): MessagePort | null;
  /// Read the output latency and tail of the plan that renders now (issue 2116). A rebuild's
  /// successor shows from the first read after the block that adopts it. One read may be unsettled
  /// at a time; a second rejects locally with result `6`.
  planLatency(): Promise<MisoPlanLatencyReply>;
  /// Send one command frame to the engine's control plane (issue 1562a D1): the frames the C ABI's
  /// `miso_engine_v1_submit_command` accepts, a `SESSION_TRANSACTION_APPLY` of `SET_CONTROL` and
  /// `Observe` edits, `OBSERVATION_READ`, `CONTROL_LIST_GET`, `CONTROL_STATE_GET`, `SESSION_SNAPSHOT_GET`, counters and capabilities.
  /// Nothing in the frame is decoded here, and the engine decodes it in the worklet's message
  /// handler. The frame is copied. One request-ID allocator must serve every frame sent through
  /// this method, because the engine's replay cache refuses a reused ID with different bytes. A
  /// frame that is empty or over `controlFrameBytes` rejects with `RESULT_INVALID_ARGUMENT` (1), and more
  /// than 64 frames in flight with `RESULT_BACKPRESSURE` (6), before the port. `document` (issue
  /// 1697) is the document a `SESSION_REPLACE` frame names, copied and sent beside it, up to the
  /// boot's 1 MiB document bound; an empty one rejects with `RESULT_INVALID_ARGUMENT` (1) before
  /// the port, and one over the bound answers `RESULT_INVALID_ARGUMENT` with an empty response.
  control(frame: Uint8Array, document?: Uint8Array): Promise<MisoControlReply>;
  /// Read the compiled session's canonical source order (issues 137 D1, 207).
  ///
  /// `sources` is what `submitSource`/`seekSource` feed, with the channel count and region every
  /// submission has to agree with. The strip order is the control list's, not this map's (#1565).
  sessionMap(): Promise<MisoSessionMap>;
  /// Take or release the decimated meter lease (issue 137 D2).
  meters(
    request: { enabled: boolean; onFrame: ((frame: MisoMeterFrame) => void) | null },
  ): Promise<MisoAck>;
  /// Take or release the render-telemetry lease (issue 137 D3).
  telemetry(
    request: { enabled: boolean; onFrame: ((frame: MisoTelemetryFrame) => void) | null },
  ): Promise<MisoAck>;
  /// @internal Subscribe to the engine's protocol events (issue 1570b D5): every command's
  /// `SESSION_COMMITTED`, `OBSERVATION_ENDED` (reliable: a commit removed the effect of a
  /// subscribed observation tap, and the event names every ended tap key, #1584), `TRANSPORT_STATE`,
  /// `METER_BATCH`, `COUNTER_SNAPSHOT`, `DIAGNOSTIC` and the other events of the control protocol,
  /// as the engine encoded them. The listener gets each event with its lane (`0` reliable, `1`
  /// lossy) in arrival order. Returns the unsubscribe
  /// function. An event is not the ack of a command: the command's reply is.
  onControlEvent(listener: (event: MisoControlEvent) => void): () => void;
  /// @internal Subscribe to the applied-revision watermark (issue 1294a): the listener gets each
  /// change of the watermark's `(revision, sample)` pair, the newest record only. Returns the
  /// unsubscribe function. The SDK's `engine.apply` is the public surface.
  onWatermark(listener: (watermark: MisoWatermark) => void): () => void;
  /// @internal The count of engine events that arrived while no listener was subscribed (issue
  /// 1570b D5). Such an event is dropped here, which changes no engine state.
  readonly discardedControlEvents: number;
  /// Dispose the host. Every later call returns the same promise once it ended in teardown. It
  /// rejects with the engine's error, or with a `MisoTeardown` when nobody answered (issue 1332
  /// S8); the node is disconnected (and in `"worker"` mode the Worker terminated) either way.
  dispose(): Promise<void>;
}

/// Boot preparation accepts same-realm plain-data records and undecorated arrays with standard
/// non-Proxy intrinsics. `context` and `preparedModule` are capability exceptions; `document` is
/// privately copied from a genuine `Uint8Array`. Invalid shapes reject the returned Promise before
/// loading.
export interface CreateMisoAudioWorkletHostOptions {
  context: BaseAudioContext;
  document: Uint8Array;
  options: MisoWebBootOptions;
  simd128ModuleUrl: string;
  preparedModule?: WebAssembly.Module;
  /// One encoded `SPECTRUM_CONFIGURE` command frame (#2166), privately copied like `document`: the
  /// engine applies it before it prepares the first plan, so the boot's spectrum taps are that
  /// plan's and the boot counts no blocking rebuild. Absent for a boot without taps.
  bootCommand?: Uint8Array;
  workletModuleUrl: string;
  /// Issue 1332 S7: `"single"` runs one instance on an isolated page too; `"worker"` is refused on
  /// a page without cross-origin isolation. Absent: `"worker"` exactly when the page is isolated.
  /// The control Worker (`miso-engine-v1-control-worker.js`) is resolved beside
  /// `workletModuleUrl`; a Worker failure before its boot reply rejects with `MisoWorkerFailed`.
  instanceMode?: MisoInstanceMode;
}

export function createMisoAudioWorkletHost(
  options: CreateMisoAudioWorkletHostOptions,
): Promise<MisoAudioWorkletHost>;
