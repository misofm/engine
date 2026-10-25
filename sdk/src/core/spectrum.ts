import { ABI_LAYOUT } from "../generated/abi.ts";
import type { ResultCodeName } from "../generated/abi.ts";
import type { ControlStatusName } from "./control-frame.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import type { ErrorPhase } from "./errors.ts";
import {
  SPECTRUM_FRAME_WINDOW_FRAMES,
  requireSpectrumFrameChannels,
  spectrumFrameTarget,
} from "./spectrum-frame.ts";
import type {
  SpectrumFrameConfigured,
  SpectrumFrameReply,
  SpectrumFrameRefusal,
  SpectrumFrameRequest,
  SpectrumFrameStream,
  SpectrumFrameWindow,
} from "./spectrum-frame.ts";

import type { Channel } from "./types.ts";

/**
 * One of the three graph boundaries supported by the one-shot spectrum capture.
 *
 * `trackPostInput` is a track at its `post_input` tap (after the input section) and
 * `trackPostPan` at its `post_pan` tap (after pan or matrix); decision 12 renamed both in place,
 * with codes 1 and 2 unchanged. `output` is a session output.
 */
export type SpectrumTarget =
  | Readonly<{ readonly kind: "trackPostInput"; readonly trackId: string }>
  | Readonly<{ readonly kind: "trackPostPan"; readonly trackId: string }>
  | Readonly<{ readonly kind: "output"; readonly outputId: string }>;

/** Bounds for one spectrum capture/query. */
export interface SpectrumLimits {
  /** Bounds native prepared capture storage and serialized raw capture bytes. FFT and result storage have separate fixed bounds. */
  readonly maximumCaptureBytes?: number;
  /** Maximum time spent waiting for a browser capture/analysis reply. */
  readonly requestDeadlineMs?: number;
}

/** A one-shot 2048-sample spectrum query. */
export interface SpectrumQuery {
  readonly target: SpectrumTarget;
  readonly channels?: Channel;
  readonly spectrumLimits?: SpectrumLimits;
}

/** One exact graph boundary and channel mask in a prepared spectrum collection. */
export interface SpectrumCollectionEntry {
  readonly target: SpectrumTarget;
  readonly channels?: Channel;
}

/** Several fixed spectrum boundaries prepared for one atomic managed selection owner. */
export interface SpectrumCollection {
  readonly entries: readonly SpectrumCollectionEntry[];
  /** Aggregate retained capture budget for all prepared entries. Empty collections use zero. */
  readonly maximumCaptureBytes: number;
}

/** The stream hops a spectrum configuration may name, in frames. */
export type SpectrumHopFrames = 256 | 512 | 1024 | 2048;

/** What `configureSpectrum` sends beside the taps. */
export interface SpectrumConfigureOptions {
  /** The explicit stream hop in frames; absent for the session shape's default cadence. */
  readonly hopFrames?: SpectrumHopFrames;
}

/**
 * The core's answer to `configureSpectrum` (`SPECTRUM_CONFIGURE`, #1594 D3; #1596 D5). The new
 * taps are in effect once a spectrum reply reports `adoptedEpoch === configuredEpoch`.
 */
export interface SpectrumConfigured {
  /** The epoch of the configuration now set. */
  readonly configuredEpoch: bigint;
  /** The epoch of the configuration the running plan carries. */
  readonly adoptedEpoch: bigint;
  /** Whether the command prepared a replacement plan: it is adopted at a later render block. */
  readonly rebuild: boolean;
  /** The epoch of that plan, present exactly when `rebuild` is true. */
  readonly planEpoch?: bigint;
}

/** The owned result of one completed spectrum query. */
export interface SpectrumResult {
  readonly target: SpectrumTarget;
  readonly channels: Channel;
  readonly sampleRateHz: number;
  readonly windowFrames: number;
  readonly binCount: number;
  readonly floorDb: number;
  readonly frequenciesHz: Float32Array;
  readonly leftDb?: Float32Array;
  readonly rightDb?: Float32Array;
  readonly capturedSample: bigint;
  readonly endSample: bigint;
  /**
   * The host-scoped monotonic token of a one-shot window, from 1. A stream reply carries no token
   * (#1595 D3), so a stream result reports its window count in the capture epoch (`windows`).
   */
  readonly snapshotToken: bigint;
  /** True when any source in the graph underran during this captured window. */
  readonly graphSourceUnderrun: boolean;
  readonly resultBytes: bigint;
}

/** Native availability state for one managed continuous spectrum capture. */
export type SpectrumStreamStatus =
  | "inactive"
  | "warming"
  | "pending"
  | "gap"
  | "failed"
  | "stopped"
  | "ready";

/** Copied metadata for one managed spectrum stream state or window. */
export interface SpectrumStreamMetadata {
  readonly result: number;
  readonly status: SpectrumStreamStatus;
  readonly target?: SpectrumTarget;
  readonly channels?: Channel;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly hopFrames: number;
  readonly sourceUnderrun: boolean;
  readonly captureEpoch: bigint;
  readonly sequence: bigint;
  readonly droppedCaptures: bigint;
  readonly windows: bigint;
  readonly capturedSample: bigint;
  readonly endSample: bigint;
  readonly analysisEpoch: bigint;
  readonly historyStartSample: bigint;
  readonly smoothingMs: number;
}

/** One copied result of a managed stream read; availability states may have no result yet. */
export interface SpectrumStreamRead {
  readonly metadata: SpectrumStreamMetadata;
  readonly result?: SpectrumResult;
}

/** Native profile acknowledgement returned when a managed spectrum stream starts. */
export interface SpectrumStreamStart {
  readonly ok: boolean;
  readonly result: number;
  readonly code: string;
  readonly metadata: SpectrumStreamMetadata;
}

/** Result of an atomic managed-stream target/channel/smoothing update. */
export interface SpectrumStreamSelection {
  readonly ok: boolean;
  readonly result: number;
  readonly code: string;
  /** Present only after a native selection commit; it is the host's copied profile. */
  readonly metadata?: SpectrumStreamMetadata;
}

/**
 * The longest spectrum target identity in UTF-8 bytes, the frame's bound
 * (`protocol::SPECTRUM_TARGET_ID_MAXIMUM_BYTES`). @internal
 */
export const SPECTRUM_MAXIMUM_ID_BYTES = 127;

/**
 * The largest capture budget of a tap, in bytes (`host_core::SPECTRUM_MAXIMUM_CAPTURE_BYTES`): the
 * default `maximumCaptureBytes` and the bound of an explicit one. @internal
 */
export const SPECTRUM_MAXIMUM_CAPTURE_BYTES = 1 << 20;

/**
 * The bytes the SDK counts for one result header and for one window header beside their planes:
 * `SpectrumResult.resultBytes` and a subscription's window bound count a read as the browser's old
 * spectrum records counted it (88 and 64 bytes, #1596b), so a byte bound means what it meant.
 */
const RESULT_HEADER_BYTES = 88;
const WINDOW_HEADER_BYTES = 64;

function requireChannel(channel: Channel | undefined): void {
  if (channel === undefined || channel === "both" || channel === "left" || channel === "right") return;
  throw new MisoUsageError("channels must be left, right, or both");
}

function targetId(target: SpectrumTarget): string {
  if (target === null || typeof target !== "object") {
    throw new MisoUsageError("spectrum target must be an object");
  }
  let id: unknown;
  switch (target.kind) {
    case "trackPostInput":
    case "trackPostPan":
      id = target.trackId;
      break;
    case "output":
      id = target.outputId;
      break;
    default:
      throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
  if (typeof id !== "string" || id.length === 0) {
    throw new MisoUsageError("spectrum target identity must be a nonempty string");
  }
  return id;
}

function copyTarget(target: SpectrumTarget): SpectrumTarget {
  const id = targetId(target);
  if (new TextEncoder().encode(id).byteLength > SPECTRUM_MAXIMUM_ID_BYTES) {
    throw new MisoUsageError("spectrum target identity exceeds its bound");
  }
  return target.kind === "output"
    ? Object.freeze({ kind: target.kind, outputId: id })
    : Object.freeze({ kind: target.kind, trackId: id });
}

function checkedInteger(value: number, name: string, minimum: number, maximum: number): number {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) {
    throw new MisoUsageError(`${name} must be an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

/** Copy caller-owned spectrum metadata before it crosses a boot or async boundary. */
export function cloneSpectrumQuery(query: SpectrumQuery): SpectrumQuery {
  if (query === null || typeof query !== "object") {
    throw new MisoUsageError("spectrum query must be an object");
  }
  requireChannel(query.channels);
  const sourceLimits = query.spectrumLimits;
  let spectrumLimits: SpectrumLimits | undefined;
  if (sourceLimits !== undefined) {
    if (sourceLimits === null || typeof sourceLimits !== "object") {
      throw new MisoUsageError("spectrumLimits must be an object");
    }
    if (sourceLimits.maximumCaptureBytes !== undefined) {
      checkedInteger(
        sourceLimits.maximumCaptureBytes,
        "maximumCaptureBytes",
        1,
        SPECTRUM_MAXIMUM_CAPTURE_BYTES,
      );
    }
    if (sourceLimits.requestDeadlineMs !== undefined) {
      checkedInteger(sourceLimits.requestDeadlineMs, "requestDeadlineMs", 1, 2_147_483_647);
    }
    spectrumLimits = Object.freeze({
      ...(sourceLimits.maximumCaptureBytes === undefined
        ? {} : { maximumCaptureBytes: sourceLimits.maximumCaptureBytes }),
      ...(sourceLimits.requestDeadlineMs === undefined
        ? {} : { requestDeadlineMs: sourceLimits.requestDeadlineMs }),
    });
  }
  const copy: SpectrumQuery = {
    target: copyTarget(query.target),
    ...(query.channels === undefined ? {} : { channels: query.channels }),
    ...(spectrumLimits === undefined ? {} : { spectrumLimits }),
  };
  return Object.freeze(copy);
}

function spectrumEntryKey(entry: SpectrumCollectionEntry): string {
  const target = copyTarget(entry.target);
  const channels = entry.channels ?? "both";
  requireChannel(channels);
  return `${target.kind}\u0000${target.kind === "output" ? target.outputId : target.trackId}\u0000${channels}`;
}

/** Copy and validate every exact entry before a collection crosses a boot or async boundary. */
export function cloneSpectrumCollection(collection: SpectrumCollection): SpectrumCollection {
  if (collection === null || typeof collection !== "object") {
    throw new MisoUsageError("spectrum collection must be an object");
  }
  if (!Array.isArray(collection.entries)) {
    throw new MisoUsageError("spectrum collection entries must be an array");
  }
  if (!Number.isSafeInteger(collection.maximumCaptureBytes)
      || collection.maximumCaptureBytes < 0) {
    throw new MisoUsageError("spectrum collection maximumCaptureBytes must be a non-negative safe integer");
  }
  if (collection.entries.length === 0 && collection.maximumCaptureBytes !== 0) {
    throw new MisoUsageError("an empty spectrum collection must use maximumCaptureBytes 0");
  }
  if (collection.entries.length > 0 && collection.maximumCaptureBytes === 0) {
    throw new MisoUsageError("a non-empty spectrum collection needs a positive capture budget");
  }
  const seen = new Set<string>();
  const entries = collection.entries.map((entry, index) => {
    if (entry === null || typeof entry !== "object") {
      throw new MisoUsageError(`spectrum collection entry ${index} must be an object`);
    }
    const target = copyTarget(entry.target);
    const channels = entry.channels ?? "both";
    requireChannel(channels);
    const copy = Object.freeze({ target, channels });
    const key = spectrumEntryKey(copy);
    if (!seen.add(key)) {
      throw new MisoUsageError(`spectrum collection entry ${index} duplicates an earlier target and mask`);
    }
    return copy;
  });
  return Object.freeze({
    entries: Object.freeze(entries),
    maximumCaptureBytes: collection.maximumCaptureBytes,
  });
}

/** Copy stream metadata received across the browser boundary, including its nested target. */
export function cloneSpectrumStreamMetadata(metadata: SpectrumStreamMetadata): SpectrumStreamMetadata {
  const target = metadata.target;
  const targetCopy = target === undefined
    ? undefined
    : target.kind === "output"
      ? Object.freeze({ kind: target.kind, outputId: target.outputId })
      : Object.freeze({ kind: target.kind, trackId: target.trackId });
  return Object.freeze({
    ...metadata,
    ...(targetCopy === undefined ? {} : { target: targetCopy }),
  });
}

/** Minimum serialized raw capture size for one fixed spectrum window. Internal SDK admission aid. */
export function spectrumCaptureWindowBytes(channels: NonNullable<SpectrumQuery["channels"]>): number {
  const channelCount = channels === "both" ? 2 : 1;
  return WINDOW_HEADER_BYTES
    + SPECTRUM_FRAME_WINDOW_FRAMES * Float32Array.BYTES_PER_ELEMENT * channelCount;
}

// ----------------------------------------------------------------------------------------------
// The SDK's public spectrum types over the spectrum frames (`spectrum-frame.ts`), shared by both
// engines. They live here, not in the codec, because they read the generated ABI layout.

/** The engine result class a refused spectrum reply's status maps to (as `responseRefusalResult`). @internal */
export function spectrumRefusalResult(status: ControlStatusName): ResultCodeName {
  switch (status) {
    case "backpressure":
      return "backpressure";
    case "invalid_field":
    case "validation_failed":
    case "not_found":
      return "invalidArgument";
    case "unavailable":
      return "unsupported";
    case "limit_exceeded":
      return "refusedBudget";
    default:
      return "internal";
  }
}

function frameResultValue(name: ResultCodeName): number {
  const row = ABI_LAYOUT.constants.resultCodes.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated ABI layout has no result ${name}`);
  return row.value;
}

/** The `EngineCallResult` words of a spectrum reply: `ok`, or the class its status maps to. @internal */
export function spectrumCallResult(reply: SpectrumFrameReply<unknown>): {
  readonly ok: boolean;
  readonly result: number;
  readonly code: ResultCodeName;
} {
  const code = reply.ok ? "ok" : spectrumRefusalResult(reply.status);
  return Object.freeze({ ok: reply.ok, result: frameResultValue(code), code });
}

/**
 * The error a refused spectrum reply raises: `code` is the result class its status maps to, and the
 * diagnostics are the core's, so `diagnosticCode` is the core's own code (`spectrum.busy`,
 * `spectrum.entry.unknown`, ...). @internal
 */
export function spectrumRefusalError(message: string, refusal: SpectrumFrameRefusal, phase: ErrorPhase = "output"): MisoEngineError {
  const code = spectrumRefusalResult(refusal.status);
  return new MisoEngineError(message, {
    phase,
    code,
    result: frameResultValue(code),
    diagnostics: refusal.diagnostics.length === 0
      ? [{ code: refusal.status, path: "" }]
      : refusal.diagnostics.map((row) => ({ code: row.code, path: "" })),
  });
}

/**
 * The one entry a select names, copied: a target and a channel mask with a wire spelling, and
 * nothing the core checks (#1596 D6). @internal
 */
export function spectrumSelection(query: SpectrumQuery): SpectrumQuery {
  if (query === null || typeof query !== "object") throw new MisoUsageError("a spectrum query must be an object");
  const channels = query.channels ?? "both";
  requireSpectrumFrameChannels(channels);
  return Object.freeze({ target: spectrumFrameTarget(query.target), channels });
}

function frameBudget(value: unknown): bigint {
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value < 0) {
    throw new MisoUsageError("maximumCaptureBytes must be a non-negative safe integer");
  }
  return BigInt(value);
}

/**
 * The `SPECTRUM_CONFIGURE` request of a public configuration, and the copy the SDK keeps of it. A
 * single query without a capture limit asks for the session's largest window budget, as the boot
 * staging does. It refuses only what has no wire spelling (#1596 D6). @internal
 */
export function spectrumConfigureRequest(
  configuration: SpectrumQuery | SpectrumCollection,
  options: SpectrumConfigureOptions | undefined,
): {
  readonly request: Extract<SpectrumFrameRequest, { op: "configure" }>;
  readonly query: SpectrumQuery | undefined;
  readonly collection: SpectrumCollection | undefined;
} {
  if (configuration === null || typeof configuration !== "object") {
    throw new MisoUsageError("a spectrum configuration must be a query or a collection");
  }
  const hopFrames = options?.hopFrames;
  if ("entries" in configuration) {
    if (!Array.isArray(configuration.entries)) throw new MisoUsageError("spectrum entries must be an array");
    const entries = configuration.entries.map((entry) => {
      if (entry === null || typeof entry !== "object") throw new MisoUsageError("a spectrum entry must be an object");
      const channels = entry.channels ?? "both";
      requireSpectrumFrameChannels(channels);
      return Object.freeze({ target: spectrumFrameTarget(entry.target), channels });
    });
    const maximumCaptureBytes = frameBudget(configuration.maximumCaptureBytes);
    return {
      request: { op: "configure", form: "collection", entries, maximumCaptureBytes, hopFrames },
      query: undefined,
      collection: Object.freeze({
        entries: Object.freeze(entries),
        maximumCaptureBytes: configuration.maximumCaptureBytes,
      }),
    };
  }
  const channels = configuration.channels ?? "both";
  requireSpectrumFrameChannels(channels);
  const target = spectrumFrameTarget(configuration.target);
  const maximum = configuration.spectrumLimits?.maximumCaptureBytes
    ?? SPECTRUM_MAXIMUM_CAPTURE_BYTES;
  const maximumCaptureBytes = frameBudget(maximum);
  return {
    request: {
      op: "configure",
      form: "single",
      entries: [{ target, channels }],
      maximumCaptureBytes,
      hopFrames,
    },
    query: Object.freeze({
      target,
      channels,
      spectrumLimits: Object.freeze({ maximumCaptureBytes: maximum }),
    }),
    collection: undefined,
  };
}

/** The public `SpectrumConfigured` of a configure reply's value. @internal */
export function spectrumConfiguredOf(value: SpectrumFrameConfigured): SpectrumConfigured {
  return Object.freeze({
    configuredEpoch: value.configuredEpoch,
    adoptedEpoch: value.adoptedEpoch,
    rebuild: value.rebuild,
    ...(value.planEpoch === undefined ? {} : { planEpoch: value.planEpoch }),
  });
}

/**
 * The analysis floor of every bin, in dB (`host_core::SPECTRUM_FLOOR_DB`). A ready capture reply
 * carries it; a stream reply does not (#1595 D3), and its bins are analyzed with the same floor.
 */
const SPECTRUM_FLOOR_DB = -120;

/**
 * `SpectrumResult.resultBytes`: the bytes of the result record the browser's old spectrum exports
 * wrote for these planes (its header, the axis and each plane), so a subscription's byte bounds
 * count a frame read as they counted a record read.
 */
function frameResultBytes(binCount: number, planes: number): bigint {
  return BigInt(RESULT_HEADER_BYTES + binCount * 4 * (1 + planes));
}

function planeCount(left: Float32Array | undefined, right: Float32Array | undefined): number {
  return (left === undefined ? 0 : 1) + (right === undefined ? 0 : 1);
}

/** The public result of a ready one-shot window. @internal */
export function spectrumResultOfWindow(window: SpectrumFrameWindow): SpectrumResult {
  return Object.freeze({
    target: window.target,
    channels: window.channels,
    sampleRateHz: window.sampleRateHz,
    windowFrames: window.windowFrames,
    binCount: window.binCount,
    floorDb: window.floorDb,
    frequenciesHz: window.frequenciesHz,
    ...(window.leftDb === undefined ? {} : { leftDb: window.leftDb }),
    ...(window.rightDb === undefined ? {} : { rightDb: window.rightDb }),
    capturedSample: window.capturedSample,
    endSample: window.endSample,
    snapshotToken: window.snapshotToken,
    graphSourceUnderrun: window.sourceUnderrun,
    resultBytes: frameResultBytes(window.binCount, planeCount(window.leftDb, window.rightDb)),
  });
}

/** The public metadata of a stream reply. @internal */
export function spectrumMetadataOf(stream: SpectrumFrameStream): SpectrumStreamMetadata {
  return Object.freeze({
    result: frameResultValue("ok"),
    status: stream.status,
    ...(stream.target === undefined ? {} : { target: stream.target }),
    ...(stream.channels === undefined ? {} : { channels: stream.channels }),
    sampleRateHz: stream.sampleRateHz,
    quantumFrames: stream.quantumFrames,
    hopFrames: stream.hopFrames,
    sourceUnderrun: stream.sourceUnderrun,
    captureEpoch: stream.captureEpoch,
    sequence: stream.sequence,
    droppedCaptures: stream.droppedCaptures,
    windows: stream.windows,
    capturedSample: stream.capturedSample,
    endSample: stream.endSample,
    analysisEpoch: stream.analysisEpoch,
    historyStartSample: stream.historyStartSample,
    smoothingMs: stream.smoothingMs,
  });
}

/** The public result of a ready stream reply's bins; `undefined` when the reply has none. @internal */
export function spectrumResultOfStream(stream: SpectrumFrameStream): SpectrumResult | undefined {
  const bins = stream.bins;
  if (bins === undefined || stream.target === undefined || stream.channels === undefined) return undefined;
  return Object.freeze({
    target: stream.target,
    channels: stream.channels,
    sampleRateHz: stream.sampleRateHz,
    windowFrames: SPECTRUM_FRAME_WINDOW_FRAMES,
    binCount: bins.frequenciesHz.length,
    floorDb: SPECTRUM_FLOOR_DB,
    frequenciesHz: bins.frequenciesHz,
    ...(bins.leftDb === undefined ? {} : { leftDb: bins.leftDb }),
    ...(bins.rightDb === undefined ? {} : { rightDb: bins.rightDb }),
    capturedSample: stream.capturedSample,
    endSample: stream.endSample,
    snapshotToken: stream.windows,
    graphSourceUnderrun: stream.sourceUnderrun,
    resultBytes: frameResultBytes(bins.frequenciesHz.length, planeCount(bins.leftDb, bins.rightDb)),
  });
}
