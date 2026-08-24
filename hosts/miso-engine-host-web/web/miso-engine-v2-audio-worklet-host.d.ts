// Browser AudioWorklet host, ABI version 1 (issue 024, amended by issues 106, 083 W4-D1 and 137).
//
// # The live console (issue 137)
//
// V1 as issue 024 froze it was a deterministic *renderer*: create, stream, render, dispose. Issue
// 137 adds the three things a live mixing console needs on top, additively -- no existing message,
// field or result code changed meaning, and the 192-byte configuration and 224-byte resource
// report kept their exact layouts.
//
//   1. `miso.command.v1`, a control path from the main realm into the engine, acknowledged with
//      the exact absolute sample the batch took effect at.
//   2. `miso.meters.v1` / `miso.meter.v1`, a decimated peak stream on a lease.
//   3. `miso.telemetry.v1`, windowed render-time telemetry on a lease, measured in JavaScript
//      around the render export.
//
// ## What the control path can and cannot move, and why
//
// **This is the most important thing to know before writing an app against it.** The engine has no
// general post-preparation parameter write path. `BUILTIN_PARAMETER_DESCRIPTORS_V1` -- the builtin
// parameter ABI -- says which builtin parameters may move after preparation, and the answer is
// four of them: `matrix_ll`, `matrix_lr`, `matrix_rl` and `matrix_rr`, which declare
// `BuiltinParameterUpdateRate::BlockTarget` with a linear smoothing policy. `polarity_invert`,
// `trim_db`, `hpf_hz`, `lpf_hz`, `fader_db` and `mute` all declare `PreparedOnly`. Effect
// parameters have no write path at all: the graph passes an empty automation slice to every
// prepared effect and its runtime module is private and positionally addressed.
//
// So `MisoCommandKindV1.Pan` and `.Matrix` are applied, and `.FaderDb`, `.Mute`, `.EffectParam`
// and `.EffectBypass` are **declared, addressed, domain-checked and then refused** with
// `result: 7` and `reason: MisoCommandReasonV1.UnsupportedKind`. That refusal is deliberately
// distinguishable from `Malformed` and from the `Unknown*` reasons: the parameter exists and the
// value is legal, and the engine cannot move it yet. The build-time parameter-metadata JSON marks
// every such parameter, so an app never has to discover this at runtime.
//
// ## Addressing is session-stable and string-free
//
// A command names a track by its index in the compiled session's canonical normalized track order,
// which `sessionMap()` returns, plus a rack, an effect index and a numeric parameter ID. No string
// crosses the command path. The identity mapping is `sessionMap()` for the session and the
// build-time metadata JSON for the effect vocabulary.
//
// ## One batch is one transaction
//
// The worklet validates a whole submission -- shape, addressing, domain, and free queue room --
// before it pushes a single record. A refused batch admits nothing, so a half-applied fader move
// cannot exist, and a flood is refused before it reaches a queue. `RESULT_BACKPRESSURE` (6) is
// returned by the main-realm host locally when its own in-flight bound is reached, and by the
// engine when a bounded per-track control queue has no room.
//
// ## Application timing is exact
//
// A track's matrix stage drains its control queue at the top of the block, before it touches a
// sample. `appliedAtSample` on the acknowledgement is therefore the first sample of the next
// rendered block, and every sample of that block carries the change. It is an exact statement, not
// an estimate.
//
// ## What metering costs
//
// `consoleMeterBlocks === 0n` binds no meter observer at all: the render path folds nothing, and
// `meters({ enabled: true })` is refused with `RESULT_UNSUPPORTED` rather than reporting zeros. A
// nonzero value binds one post-matrix meter per track with a `blocks * quantumFrames` window and
// makes the port lease a second, finer switch over the master fold and every drain and post. The
// honest summary: *not attaching* meters costs nothing at all; attaching them and releasing the
// lease costs one branch per block plus the per-track observation fold, which runs whenever the
// observers exist.
//
// Gain reduction is **not** in the meter frame. No effect in the engine exposes a per-block gain
// reduction observation point -- `PreparedNativeEffect` has no observation method, GR lives as
// private smoother state in each dynamics kernel, and the graph binds observers only to track
// stages. `MisoMeterFrameV1` will gain `trackGrDb` additively once that observation point exists.
//
// ## What a frame costs the render callback
//
// One `postMessage` per window, not per block. The body is a frozen object allocated at
// construction whose only array is a `Float32Array` of `2 * trackCount + 2` elements, also
// allocated at construction and overwritten in place; nothing is transferred, so the caller keeps
// no ownership obligation. The single allocation left is the structured clone `postMessage`
// performs, which is `4 * (2 * trackCount + 2)` bytes plus four small numbers -- 264 bytes for a
// 32-track console. That cost has not been separately benchmarked in a browser; the telemetry
// lease measures the render export, not the post around it.
//
// # Exactly one artifact
//
// Owner decision W4-D1: the shipped module is built with `+simd128`. There is no scalar artifact
// and no dual-artifact selection. `createMisoAudioWorkletHost` validates a canned `simd128` module
// with `WebAssembly.validate` before it fetches anything and rejects with a typed
// `MisoUnsupportedBrowserV1` (`tag: "miso.unsupported.v1"`, `capability: "simd128"`) when the probe
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
// nowhere for a further chunk to go -- plus one unsettled seek per source and one unsettled status.
// A request over its bound rejects locally with `RESULT_BACKPRESSURE` (6) before any transfer, so
// the caller keeps its planes and can retry.
//
// The default ring covers a 100 ms main-thread stall: `(ceil(100ms * fs / quantum) + 2) * quantum`
// frames, which is 5 120 frames (40 KiB for stereo `f32`) at 48 kHz with a 128-frame quantum. It is
// a prefill ahead of the render position and adds no output latency.
//
// Two options deliberately not taken, recorded so they are not rediscovered as bugs:
//
//   1. Zero-copy submission (`source_reserve` / `source_commit`) and a chunk size decoupled from
//      the quantum. Both need a `miso-engine-source` contract change, which is that crate's issue
//      (#101); today `validate_submission_metadata` requires exactly one quantum unless the chunk
//      ends the region.
//   2. A `SharedArrayBuffer` ring under `crossOriginIsolated === true`, which would remove the
//      message round trip entirely. It requires the engine's atomics-free policy to be revisited
//      and is an owner decision, not a host change.
//
// # Compilation happens on the rendering thread
//
// Session TOML is compiled inside the `AudioWorkletProcessor` constructor, which runs on the
// rendering thread before the first `process()` call. This is documented V1 behaviour (issue 024,
// owner open question 2): construction allocates, later rendering does not.
//
/// The frozen backend names of the issue-024 ABI. Only `"simd128"` is shipped (W4-D1); `"scalar"`
/// remains a legal ABI value because the Rust artifact still reports backend `0` when it is built
/// without `+simd128`, and the processor rejects such a module rather than rendering with it.
export type MisoWebBackendV1 = "scalar" | "simd128";

/// Typed refusal of a browser that cannot run the shipped artifact.
///
/// Distinct from `MisoErrorV1` on purpose: a caller must be able to tell "this browser is out of
/// scope" apart from "something went wrong". It is thrown by `createMisoAudioWorkletHost` before
/// any node exists and never crosses the `MessagePort`.
export interface MisoUnsupportedBrowserV1 {
  readonly tag: "miso.unsupported.v1";
  readonly requestId: 0;
  readonly result: 7;
  readonly capability: "simd128";
}

/// Frozen live-console command kinds (issue 137 D1).
export const enum MisoCommandKindV1 {
  /// Retarget the track's pan pair over an explicit ramp window. Applied.
  Pan = 1,
  /// Retarget the track's full 2x2 matrix over an explicit ramp window. Applied.
  Matrix = 2,
  /// Set a lane fader in decibels. Refused: `fader_db` declares `PreparedOnly`.
  FaderDb = 3,
  /// Set a lane mute. Refused: `mute` declares `PreparedOnly`.
  Mute = 4,
  /// Set an effect parameter. Refused: no post-preparation effect write path exists.
  EffectParam = 5,
  /// Set an effect bypass. Refused: no post-preparation effect write path exists.
  EffectBypass = 6,
}

/// Frozen typed reasons a live-console submission was refused (issue 137 D1).
export const enum MisoCommandReasonV1 {
  /// The submission was admitted whole.
  None = 0,
  /// Unknown kind, nonzero reserved word, non-finite value, or a field set for the wrong kind.
  Malformed = 1,
  /// `trackIndex` is not a track of the compiled session.
  UnknownTrack = 2,
  /// `rack` is not one of the three declared racks.
  UnknownRack = 3,
  /// `effectIndex` is not an effect of the addressed rack.
  UnknownEffect = 4,
  /// `parameterId` is not a parameter of the addressed effect.
  UnknownParameter = 5,
  /// A value is outside the addressed parameter's declared domain.
  Domain = 6,
  /// Well formed and correctly addressed; this ABI version cannot apply that kind.
  UnsupportedKind = 7,
  /// A bounded control queue had no room; nothing was admitted.
  Backpressure = 8,
  /// The engine is not `STATE_READY`.
  WrongState = 9,
}

/// One live-console command. `255` means "not applicable to this kind".
export interface MisoCommandV1 {
  kind: MisoCommandKindV1;
  /// `0` simd1, `1` dynamic, `2` simd2, `255` for a builtin-addressed kind.
  rack: number;
  /// `0` left, `1` right, `2` both, `255` for a kind with no lane.
  channel: number;
  /// Index into the canonical track order `sessionMap()` returns.
  trackIndex: number;
  effectIndex: number;
  parameterId: number;
  /// Ramp window in sample updates for `Pan` and `Matrix`; ignored by every other kind.
  smoothingSamples: number;
  /// `Pan`: `[left, right, 0, 0]`. `Matrix`: `[ll, lr, rl, rr]`. Everything else: `[value, 0, 0, 0]`.
  values: [number, number, number, number];
}

export interface MisoCommandRequestV1 {
  requestId: number;
  commands: MisoCommandV1[];
}

/// One live-console acknowledgement. `result` is `0` only when the whole batch was admitted.
export interface MisoCommandAckV1 {
  readonly tag: "miso.ack.v1";
  readonly requestId: number;
  readonly result: number;
  readonly reason: MisoCommandReasonV1;
  /// Index of the first refused record; `0` on success.
  readonly rejectedIndex: number;
  /// The whole batch, or zero.
  readonly admitted: number;
  /// The exact absolute sample the admitted records take effect at.
  readonly appliedAtSample: bigint;
  /// The caller's record block, handed straight back.
  readonly records: Uint8Array;
}

/// The compiled session's addressing authority (issue 137 D1).
export interface MisoSessionMapV1 {
  readonly tag: "miso.sessionmap.v1";
  readonly requestId: number;
  readonly result: number;
  /// Canonical normalized track order. `trackIndex` indexes this.
  readonly tracks: string[];
  /// Whether preparation bound meter observers at all.
  readonly metersAttached: boolean;
}

/// One decimated meter window (issue 137 D2).
export interface MisoMeterFrameV1 {
  readonly tag: "miso.meter.v1";
  readonly sequence: number;
  /// Complete windows folded into this frame; normally `1`.
  readonly windows: number;
  readonly trackCount: number;
  /// `[track0 L, track0 R, .., trackN L, trackN R, master L, master R]` peak magnitudes.
  readonly peaks: Float32Array;
}

/// One windowed render-telemetry frame (issue 137 D3). JavaScript only; Wasm never sees the lease.
export interface MisoTelemetryFrameV1 {
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

export interface MisoWebPrepareLimitsV1 {
  sessionTomlBytes: number;
  diagnosticBytes: number;
  sourceIdBytes: number;
  maximumSourceChannels: number;
  sourceRingFrames: number;
  maximumAutomationSpansPerBlock: number;
  maximumTracks: bigint;
  maximumSources: bigint;
  maximumRoutes: bigint;
  maximumEffects: bigint;
  maximumGraphSessionPlusPlanBytes: bigint;
  maximumSourceTotalBytes: bigint;
  maximumSourceOverheadBytes: bigint;
  maximumEffectStateBytes: bigint;
  maximumEffectScratchBytes: bigint;
  maximumBuiltinRetainedBytes: bigint;
  maximumHostRetainedBytes: bigint;
  maximumNamedAllocationBytes: bigint;
  maximumMeterStreams: bigint;
  maximumMeterItems: bigint;
  maximumMeterBytes: bigint;
  /// Per-track control-queue depth in records, or `0n` for the engine default of 64 (issue 137).
  consoleCommandQueueRecords: bigint;
  /// Meter window in render blocks, or `0n` to bind no meter observer at all (issue 137).
  consoleMeterBlocks: bigint;
}

export interface MisoWebResourceReportV1 {
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly backend: number;
  readonly configBytes: bigint;
  readonly statusBytes: bigint;
  readonly sessionTomlBytes: bigint;
  readonly diagnosticBytes: bigint;
  readonly sourceIdBytes: bigint;
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
}

export interface MisoStatusV1 {
  readonly tag: "miso.status.v1";
  readonly requestId: number;
  readonly result: number;
  readonly state: number;
  readonly lastResult: number;
  readonly backend: number;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly nextAbsoluteSample: bigint;
  readonly renderedQuanta: bigint;
  readonly memoryBytes: number;
}

export interface MisoAckV1 {
  readonly tag: "miso.ack.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoErrorV1 {
  readonly tag: "miso.error.v1";
  readonly requestId: number;
  readonly result: number;
  readonly planes?: Float32Array[];
}

export interface MisoSourceRequestV1 {
  requestId: number;
  sourceId: string;
  generation: bigint;
  startFrame: bigint;
  sampleRateHz: number;
  planes: Float32Array[];
  frames: number;
  endOfRegion: boolean;
}

export interface MisoSeekRequestV1 {
  requestId: number;
  sourceId: string;
  generation: bigint;
  sourceFrame: bigint;
}

export interface MisoAudioWorkletHost {
  readonly node: AudioWorkletNode;
  readonly backend: MisoWebBackendV1;
  readonly resources: MisoWebResourceReportV1;
  readonly memoryBytes: number;
  submitSource(request: MisoSourceRequestV1): Promise<MisoAckV1>;
  seekSource(request: MisoSeekRequestV1): Promise<MisoAckV1>;
  status(): Promise<MisoStatusV1>;
  /// Submit one live-console batch as a single transaction (issue 137 D1).
  command(request: MisoCommandRequestV1): Promise<MisoCommandAckV1>;
  /// Read the canonical track order that `trackIndex` addresses (issue 137 D1).
  sessionMap(): Promise<MisoSessionMapV1>;
  /// Take or release the decimated meter lease (issue 137 D2).
  meters(
    request: { requestId: number; enabled: boolean; onFrame: ((frame: MisoMeterFrameV1) => void) | null },
  ): Promise<MisoAckV1>;
  /// Take or release the render-telemetry lease (issue 137 D3).
  telemetry(
    request: { requestId: number; enabled: boolean; onFrame: ((frame: MisoTelemetryFrameV1) => void) | null },
  ): Promise<MisoAckV1>;
  dispose(): Promise<void>;
}

export interface CreateMisoAudioWorkletHostOptionsV1 {
  context: BaseAudioContext;
  quantumFrames: number;
  sessionToml: Uint8Array;
  limits: MisoWebPrepareLimitsV1;
  simd128ModuleUrl: string;
  workletModuleUrl: string;
}

export function createMisoAudioWorkletHost(
  options: CreateMisoAudioWorkletHostOptionsV1,
): Promise<MisoAudioWorkletHost>;
