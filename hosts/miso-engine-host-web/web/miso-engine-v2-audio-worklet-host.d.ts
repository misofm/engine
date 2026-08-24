// Browser AudioWorklet host, ABI version 1 (issue 024, amended by issues 106 and 083 W4-D1).
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
