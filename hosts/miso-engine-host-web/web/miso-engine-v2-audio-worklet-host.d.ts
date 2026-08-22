export type MisoWebBackendV1 = "scalar" | "simd128";

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
  scalarModuleUrl: string;
  simd128ModuleUrl: string;
  workletModuleUrl: string;
}

export function createMisoAudioWorkletHost(
  options: CreateMisoAudioWorkletHostOptionsV1,
): Promise<MisoAudioWorkletHost>;
