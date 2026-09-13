import { ABI_LAYOUT } from "../generated/abi.ts";
import { MisoEngineError, MisoUsageError, resultName } from "./errors.ts";

import type { Channel } from "./types.ts";

/** One of the three graph boundaries supported by the one-shot spectrum capture. */
export type SpectrumTarget =
  | Readonly<{ readonly kind: "trackPostInputBuiltins"; readonly trackId: string }>
  | Readonly<{ readonly kind: "trackPostMatrix"; readonly trackId: string }>
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

/** @internal Encoded stream result returned while its reusable transfer buffer is still owned. */
export interface SpectrumEncodedStreamRead {
  readonly metadata: SpectrumStreamMetadata;
  readonly resultByteLength: number;
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

type SpectrumExport = (...args: number[]) => number | bigint;
type SpectrumExports = Record<string, unknown> & { readonly memory: WebAssembly.Memory };
type AbiField = Readonly<{ readonly name: string; readonly offset: number; readonly type?: string }>;
type AbiStructure = Readonly<{ readonly bytes: number; readonly fields: readonly AbiField[] }>;

const TARGETS = ABI_LAYOUT.constants.spectrumTargets;
const CHANNELS = ABI_LAYOUT.constants.spectrumChannels;

function structure(name: "spectrumRequest" | "spectrumCollectionRequest" | "spectrumCollectionEntry"
  | "spectrumWindow" | "spectrumResult" | "spectrumStreamMetadata"): AbiStructure {
  const value = ABI_LAYOUT.structures[name];
  if (value === undefined || !Number.isSafeInteger(value.bytes) || !Array.isArray(value.fields)) {
    throw new MisoUsageError(`the generated ABI layout has no valid ${name} structure`);
  }
  return value as AbiStructure;
}

function field(layout: AbiStructure, name: string): AbiField {
  const value = layout.fields.find((candidate) => candidate.name === name);
  if (value === undefined || !Number.isSafeInteger(value.offset) || value.offset < 0) {
    throw new MisoUsageError(`the generated ABI layout has no valid spectrum field ${name}`);
  }
  return value;
}

function value(group: readonly { readonly value: number; readonly name: string }[], name: string): number {
  const row = group.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated ABI layout has no spectrum value ${name}`);
  return row.value;
}

const RESULT_OK = value(ABI_LAYOUT.constants.resultCodes, "ok");
const RESULT_ABI_MISMATCH = value(ABI_LAYOUT.constants.resultCodes, "abiMismatch");
const RESULT_REFUSED_BUDGET = value(ABI_LAYOUT.constants.resultCodes, "refusedBudget");
const RESULT_WRONG_STATE = value(ABI_LAYOUT.constants.resultCodes, "wrongState");

const STREAM_STATUSES = ABI_LAYOUT.constants.spectrumStreamStatuses;

function exportFunction(exports: SpectrumExports, name: string): SpectrumExport {
  const value = exports[name];
  if (typeof value !== "function") throw new MisoUsageError(`the engine asset does not export ${name}`);
  return value as SpectrumExport;
}

function channelValue(channel: Channel | undefined): number {
  if (channel === undefined || channel === "both") return value(CHANNELS, "both");
  if (channel === "left") return value(CHANNELS, "left");
  if (channel === "right") return value(CHANNELS, "right");
  throw new MisoUsageError("channels must be left, right, or both");
}

/** Numeric channel mask used by the generated host bridge. Internal SDK transport helper. */
export function spectrumChannelsRaw(channel: Channel | undefined): number {
  return channelValue(channel);
}

function channelName(raw: number): Channel {
  if (raw === value(CHANNELS, "left")) return "left";
  if (raw === value(CHANNELS, "right")) return "right";
  if (raw === value(CHANNELS, "both")) return "both";
  throw malformed("the engine returned an unknown spectrum channel mask");
}

function streamStatusName(raw: number): SpectrumStreamStatus {
  const row = STREAM_STATUSES.find((candidate) => candidate.value === raw);
  if (row === undefined) throw malformed("the engine returned an unknown spectrum stream status");
  return row.name as SpectrumStreamStatus;
}

function targetValue(target: SpectrumTarget): number {
  switch (target.kind) {
    case "trackPostInputBuiltins": return value(TARGETS, "trackPostInputBuiltins");
    case "trackPostMatrix": return value(TARGETS, "trackPostMatrix");
    case "output": return value(TARGETS, "output");
    default: throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
}

/** Numeric target kind used by the generated host bridge. Internal SDK transport helper. */
export function spectrumTargetRaw(target: SpectrumTarget): number {
  return targetValue(target);
}

function targetId(target: SpectrumTarget): string {
  if (target === null || typeof target !== "object") {
    throw new MisoUsageError("spectrum target must be an object");
  }
  let id: unknown;
  switch (target.kind) {
    case "trackPostInputBuiltins":
    case "trackPostMatrix":
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

/** Stable target identity used by the generated host bridge. Internal SDK transport helper. */
export function spectrumTargetId(target: SpectrumTarget): string {
  return targetId(target);
}

function copyTarget(target: SpectrumTarget): SpectrumTarget {
  const id = targetId(target);
  if (new TextEncoder().encode(id).byteLength > ABI_LAYOUT.constants.maximumSpectrumIdBytes) {
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
  channelValue(query.channels);
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
        ABI_LAYOUT.constants.spectrumCaptureBytes,
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
  channelValue(channels);
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
    channelValue(channels);
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
  return ABI_LAYOUT.constants.spectrumWindowHeaderBytes
    + ABI_LAYOUT.constants.spectrumWindowFrames * Float32Array.BYTES_PER_ELEMENT * channelCount;
}

function contract() {
  return {
    request: structure("spectrumRequest"),
    collectionRequest: structure("spectrumCollectionRequest"),
    collectionEntry: structure("spectrumCollectionEntry"),
    window: structure("spectrumWindow"),
    result: structure("spectrumResult"),
    streamMetadata: structure("spectrumStreamMetadata"),
    abiVersion: ABI_LAYOUT.abiVersion,
    maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes,
    maximumIdBytes: ABI_LAYOUT.constants.maximumSpectrumIdBytes,
    windowFrames: ABI_LAYOUT.constants.spectrumWindowFrames,
    binCount: ABI_LAYOUT.constants.spectrumBinCount,
  };
}

function malformed(message: string): MisoEngineError {
  return new MisoEngineError(message, {
    phase: "output",
    code: "abiMismatch",
    result: RESULT_ABI_MISMATCH,
  });
}

function refusal(message: string, result: number): MisoEngineError {
  return new MisoEngineError(message, {
    phase: result === RESULT_WRONG_STATE ? "lifecycle" : "output",
    code: resultName(result, "call"),
    result,
  });
}

function bytesView(exports: SpectrumExports): Uint8Array<ArrayBuffer> {
  return new Uint8Array(exports.memory.buffer) as Uint8Array<ArrayBuffer>;
}

function range(bytes: Uint8Array, offset: number, length: number, name: string): [number, number] {
  if (!Number.isSafeInteger(offset) || offset < 0 || !Number.isSafeInteger(length) || length < 0) {
    throw malformed(`the engine returned invalid spectrum ${name} bounds`);
  }
  const end = offset + length;
  if (!Number.isSafeInteger(end) || end > bytes.byteLength) {
    throw malformed(`the engine returned invalid spectrum ${name} bounds`);
  }
  return [offset, end];
}

function readF32Array(bytes: Uint8Array, offset: number, count: number, name: string): Float32Array {
  const [start, end] = range(bytes, offset, count * Float32Array.BYTES_PER_ELEMENT, name);
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const result = new Float32Array(count);
  for (let index = 0; index < count; index += 1) {
    result[index] = view.getFloat32(start + index * 4, true);
  }
  if (!result.every(Number.isFinite)) throw malformed(`the engine returned nonfinite spectrum ${name}`);
  return result;
}

/** Stage the optional spectrum observer request before a host boot. */
export function stageSpectrumRequest(
  exports: SpectrumExports,
  query: SpectrumQuery | undefined,
): void {
  const shape = contract();
  const requestPtr = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_request_ptr")());
  const requestBytes = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_request_bytes")());
  const idPtr = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_target_id_ptr")());
  const idCapacity = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_target_id_capacity")());
  if (requestPtr <= 0 || requestBytes !== shape.request.bytes || idPtr <= 0
      || idCapacity !== shape.maximumIdBytes) throw malformed("invalid spectrum request staging");
  const bytes = bytesView(exports);
  range(bytes, requestPtr, shape.request.bytes, "request");
  range(bytes, idPtr, idCapacity, "target identity");
  const requestView = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const requestOffset = (name: string) => field(shape.request, name).offset;
  for (let index = 0; index < shape.request.bytes; index += 1) bytes[requestPtr + index] = 0;
  requestView.setUint32(requestPtr + requestOffset("structSize"), shape.request.bytes, true);
  requestView.setUint32(requestPtr + requestOffset("abiVersion"), shape.abiVersion, true);
  if (query === undefined) return;
  const id = new TextEncoder().encode(targetId(query.target));
  if (id.length > shape.maximumIdBytes) throw new MisoUsageError("spectrum target identity exceeds its bound");
  const channels = channelValue(query.channels);
  const maximum = checkedInteger(
    query.spectrumLimits?.maximumCaptureBytes ?? shape.maximumCaptureBytes,
    "maximumCaptureBytes",
    1,
    shape.maximumCaptureBytes,
  );
  bytes.set(id, idPtr);
  requestView.setUint32(requestPtr + requestOffset("target"), targetValue(query.target), true);
  requestView.setUint32(requestPtr + requestOffset("channels"), channels, true);
  requestView.setUint32(requestPtr + requestOffset("targetIdBytes"), id.length, true);
  requestView.setBigUint64(requestPtr + requestOffset("maximumCaptureBytes"), BigInt(maximum), true);
}

/** Stage several exact spectrum collection entries before a host boot. */
export function stageSpectrumCollectionRequest(
  exports: SpectrumExports,
  collection: SpectrumCollection | undefined,
): void {
  const shape = contract();
  const requestPtr = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_request_ptr")());
  const requestBytes = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_request_bytes")());
  if (requestPtr <= 0 || requestBytes !== shape.collectionRequest.bytes) {
    throw malformed("invalid spectrum collection request staging");
  }
  let bytes = bytesView(exports);
  range(bytes, requestPtr, shape.collectionRequest.bytes, "collection request");
  const requestView = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const requestOffset = (name: string) => field(shape.collectionRequest, name).offset;
  for (let index = 0; index < shape.collectionRequest.bytes; index += 1) bytes[requestPtr + index] = 0;
  requestView.setUint32(requestPtr + requestOffset("structSize"), shape.collectionRequest.bytes, true);
  requestView.setUint32(requestPtr + requestOffset("abiVersion"), shape.abiVersion, true);
  if (collection === undefined) return;
  const entries = collection.entries;
  requestView.setUint32(requestPtr + requestOffset("entryCount"), entries.length, true);
  requestView.setBigUint64(
    requestPtr + requestOffset("maximumCaptureBytes"),
    BigInt(collection.maximumCaptureBytes),
    true,
  );

  // Both collection pointer calls may grow Wasm memory. Re-read the backing view after each
  // allocator boundary; a view retained across growth is detached and would corrupt staging.
  const entryPtr = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_entry_ptr")());
  const entryBytes = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_entry_bytes")());
  const entryCapacity = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_entry_capacity")());
  if (entryPtr <= 0 || entryBytes !== shape.collectionEntry.bytes || entryCapacity !== entries.length) {
    throw malformed("invalid spectrum collection entry staging");
  }
  bytes = bytesView(exports);
  range(bytes, entryPtr, entries.length * entryBytes, "collection entries");
  const entriesView = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const entryOffset = (name: string) => field(shape.collectionEntry, name).offset;
  const encodedIds = entries.map((entry) => new TextEncoder().encode(targetId(entry.target)));
  encodedIds.forEach((id, index) => {
    const offset = entryPtr + index * entryBytes;
    for (let byte = 0; byte < entryBytes; byte += 1) bytes[offset + byte] = 0;
    entriesView.setUint32(offset + entryOffset("target"), targetValue(entries[index]!.target), true);
    entriesView.setUint32(offset + entryOffset("channels"), channelValue(entries[index]!.channels), true);
    entriesView.setUint32(offset + entryOffset("targetIdBytes"), id.byteLength, true);
  });

  const idPtr = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_target_ids_ptr")());
  const idCapacity = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_collection_target_ids_capacity")());
  const totalIdBytes = encodedIds.reduce((total, id) => total + id.byteLength, 0);
  if (idPtr <= 0 || idCapacity !== totalIdBytes) {
    throw malformed("invalid spectrum collection target identity staging");
  }
  bytes = bytesView(exports);
  range(bytes, idPtr, totalIdBytes, "collection target identities");
  let idOffset = idPtr;
  encodedIds.forEach((id) => {
    bytes.set(id, idOffset);
    idOffset += id.byteLength;
  });
}

function writeStreamMetadata(
  exports: SpectrumExports,
  query: SpectrumQuery,
  metadata: SpectrumStreamMetadata,
  shape: ReturnType<typeof contract>,
): void {
  if (metadata.target !== undefined && targetValue(metadata.target) !== targetValue(query.target)) {
    throw malformed("the imported spectrum stream target differs from the prepared target");
  }
  const channels = channelValue(metadata.channels ?? query.channels);
  if (channels !== channelValue(query.channels)) {
    throw malformed("the imported spectrum stream channels differ from the prepared channels");
  }
  const statusRow = STREAM_STATUSES.find((candidate) => candidate.name === metadata.status);
  if (statusRow === undefined) throw malformed("the imported spectrum stream status is unknown");
  const status = statusRow.value;
  const pointer = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_stream_metadata_ptr")());
  const bytesLength = Number(exportFunction(exports, "miso_engine_web_v1_spectrum_stream_metadata_bytes")());
  if (pointer <= 0 || bytesLength !== shape.streamMetadata.bytes) {
    throw malformed("invalid spectrum stream metadata staging");
  }
  const bytes = bytesView(exports);
  range(bytes, pointer, shape.streamMetadata.bytes, "stream metadata");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const at = (name: string) => field(shape.streamMetadata, name).offset;
  for (let index = 0; index < shape.streamMetadata.bytes; index += 1) bytes[pointer + index] = 0;
  view.setUint32(pointer + at("structSize"), shape.streamMetadata.bytes, true);
  view.setUint32(pointer + at("abiVersion"), shape.abiVersion, true);
  view.setUint32(pointer + at("result"), metadata.result, true);
  view.setUint32(pointer + at("status"), status, true);
  view.setUint32(pointer + at("target"), targetValue(query.target), true);
  view.setUint32(pointer + at("channels"), channels, true);
  view.setUint32(pointer + at("sampleRateHz"), metadata.sampleRateHz, true);
  view.setUint32(pointer + at("quantumFrames"), metadata.quantumFrames, true);
  view.setUint32(pointer + at("hopFrames"), metadata.hopFrames, true);
  view.setUint32(pointer + at("sourceUnderrun"), metadata.sourceUnderrun ? 1 : 0, true);
  view.setBigUint64(pointer + at("captureEpoch"), metadata.captureEpoch, true);
  view.setBigUint64(pointer + at("sequence"), metadata.sequence, true);
  view.setBigUint64(pointer + at("droppedCaptures"), metadata.droppedCaptures, true);
  view.setBigUint64(pointer + at("windows"), metadata.windows, true);
  view.setBigUint64(pointer + at("capturedSample"), metadata.capturedSample, true);
  view.setBigUint64(pointer + at("endSample"), metadata.endSample, true);
  view.setBigUint64(pointer + at("analysisEpoch"), metadata.analysisEpoch, true);
  view.setBigUint64(pointer + at("historyStartSample"), metadata.historyStartSample, true);
  view.setFloat64(pointer + at("smoothingMs"), metadata.smoothingMs, true);
}

/** Internal analyzer and raw capture decoder shared by headless and browser Workers. */
export class SpectrumModule {
  readonly #exports: SpectrumExports;
  readonly #captureSetBytes: SpectrumExport;
  readonly #analysis: SpectrumExport;
  readonly #capturePtr: SpectrumExport;
  readonly #captureBytes: SpectrumExport;
  readonly #captureCapacity: SpectrumExport;
  readonly #resultPtr: SpectrumExport;
  readonly #resultBytes: SpectrumExport;
  readonly #close: SpectrumExport;
  readonly #streamAnalysis: SpectrumExport;
  readonly #streamAnalysisConfigure: SpectrumExport;
  readonly #streamReset: SpectrumExport;
  readonly #streamMetadataPtr: SpectrumExport;
  readonly #streamMetadataBytes: SpectrumExport;
  readonly #contract = contract();
  #closed = false;

  constructor(instance: WebAssembly.Instance | SpectrumExports) {
    const table = "exports" in instance ? instance.exports as unknown as SpectrumExports : instance;
    if (!(table.memory instanceof WebAssembly.Memory)) throw new MisoUsageError("the spectrum asset has no linear memory");
    this.#exports = table;
    this.#captureSetBytes = exportFunction(table, "miso_engine_web_v1_spectrum_capture_set_bytes");
    this.#analysis = exportFunction(table, "miso_engine_web_v1_spectrum_analysis");
    this.#capturePtr = exportFunction(table, "miso_engine_web_v1_spectrum_capture_ptr");
    this.#captureBytes = exportFunction(table, "miso_engine_web_v1_spectrum_capture_bytes");
    this.#captureCapacity = exportFunction(table, "miso_engine_web_v1_spectrum_capture_capacity");
    this.#resultPtr = exportFunction(table, "miso_engine_web_v1_spectrum_result_ptr");
    this.#resultBytes = exportFunction(table, "miso_engine_web_v1_spectrum_result_bytes");
    this.#close = exportFunction(table, "miso_engine_web_v1_spectrum_close");
    this.#streamAnalysis = exportFunction(table, "miso_engine_web_v1_spectrum_stream_analysis");
    this.#streamAnalysisConfigure = exportFunction(table, "miso_engine_web_v1_spectrum_stream_analysis_configure");
    this.#streamReset = exportFunction(table, "miso_engine_web_v1_spectrum_stream_reset");
    this.#streamMetadataPtr = exportFunction(table, "miso_engine_web_v1_spectrum_stream_metadata_ptr");
    this.#streamMetadataBytes = exportFunction(table, "miso_engine_web_v1_spectrum_stream_metadata_bytes");
  }

  close(): void {
    if (this.#closed) return;
    this.#close();
    this.#closed = true;
  }

  /** Analyze bytes returned by a browser Worklet capture. */
  analyzeSnapshot(query: SpectrumQuery, snapshot: Uint8Array): SpectrumResult {
    if (this.#closed) throw new MisoUsageError("the spectrum module is closed");
    if (!(snapshot instanceof Uint8Array)) throw new MisoUsageError("the spectrum snapshot must be bytes");
    const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes
      ?? this.#contract.maximumCaptureBytes;
    if (!Number.isSafeInteger(maximumCaptureBytes) || maximumCaptureBytes < 1
        || maximumCaptureBytes > this.#contract.maximumCaptureBytes) {
      throw new MisoUsageError("maximumCaptureBytes exceeds the spectrum bound");
    }
    if (snapshot.byteLength > maximumCaptureBytes) {
      throw refusal("the spectrum snapshot exceeded its bound", RESULT_REFUSED_BUDGET);
    }
    // A Worker may have analyzed a managed stream before this one-shot request. The native
    // stream reset is a no-op for an inactive analyzer and clears the stream mode when needed.
    const reset = Number(this.#streamReset());
    if (reset !== RESULT_OK && reset !== RESULT_WRONG_STATE) {
      throw refusal("the spectrum stream analyzer could not reset for one-shot analysis", reset);
    }
    // The analysis Worker uses the same request staging as the headless path. This also wakes
    // lazy bridge buffers in a host that deliberately retained no capture storage at boot.
    stageSpectrumRequest(this.#exports, query);
    const pointer = Number(this.#capturePtr());
    const capacity = Number(this.#captureCapacity());
    if (pointer <= 0 || capacity !== this.#contract.maximumCaptureBytes) throw malformed("invalid spectrum capture staging");
    const bytes = bytesView(this.#exports);
    range(bytes, pointer, snapshot.byteLength, "capture");
    bytes.set(snapshot, pointer);
    const set = Number(this.#captureSetBytes(snapshot.byteLength));
    if (set !== RESULT_OK) throw refusal("the spectrum capture was refused", set);
    return this.#analyzeCurrent(query);
  }

  /** Analyze the capture currently staged by the headless/native host. */
  analyzeCurrent(query: SpectrumQuery): SpectrumResult {
    if (this.#closed) throw new MisoUsageError("the spectrum module is closed");
    return this.#analyzeCurrent(query);
  }

  /** Read the copied metadata for the current managed stream state. */
  streamMetadata(query: SpectrumQuery): SpectrumStreamMetadata {
    if (this.#closed) throw new MisoUsageError("the spectrum module is closed");
    const layout = structure("spectrumStreamMetadata");
    const pointer = Number(this.#streamMetadataPtr());
    const bytesLength = Number(this.#streamMetadataBytes());
    if (pointer <= 0 || bytesLength !== layout.bytes) {
      throw malformed("invalid spectrum stream metadata staging");
    }
    const bytes = bytesView(this.#exports);
    range(bytes, pointer, layout.bytes, "stream metadata");
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const at = (name: string) => field(layout, name).offset;
    const u32 = (name: string) => view.getUint32(pointer + at(name), true);
    const u64 = (name: string) => view.getBigUint64(pointer + at(name), true);
    const structSize = u32("structSize");
    const abiVersion = u32("abiVersion");
    if (structSize !== layout.bytes || abiVersion !== this.#contract.abiVersion
        || u32("reserved0") !== 0 || u32("reserved1") !== 0) {
      throw malformed("the engine returned a malformed spectrum stream metadata record");
    }
    const rawTarget = u32("target");
    const target = rawTarget === 0 ? undefined : copyTarget(query.target);
    if (target !== undefined && rawTarget !== targetValue(query.target)) {
      throw malformed("the engine returned a spectrum stream target different from the prepared target");
    }
    const rawChannels = u32("channels");
    const channels = rawChannels === 0 ? undefined : channelName(rawChannels);
    const smoothingMs = view.getFloat64(pointer + at("smoothingMs"), true);
    if (!Number.isFinite(smoothingMs) || smoothingMs < 0 || smoothingMs > 10_000) {
      throw malformed("the engine returned an invalid spectrum smoothing duration");
    }
    return Object.freeze({
      result: u32("result"),
      status: streamStatusName(u32("status")),
      ...(target === undefined ? {} : { target }),
      ...(channels === undefined ? {} : { channels }),
      sampleRateHz: u32("sampleRateHz"),
      quantumFrames: u32("quantumFrames"),
      hopFrames: u32("hopFrames"),
      sourceUnderrun: u32("sourceUnderrun") !== 0,
      captureEpoch: u64("captureEpoch"),
      sequence: u64("sequence"),
      droppedCaptures: u64("droppedCaptures"),
      windows: u64("windows"),
      capturedSample: u64("capturedSample"),
      endSample: u64("endSample"),
      analysisEpoch: u64("analysisEpoch"),
      historyStartSample: u64("historyStartSample"),
      smoothingMs,
    });
  }

  /** Analyze the raw window staged by a managed stream read and return its owned result/metadata. */
  analyzeStreamCurrent(query: SpectrumQuery): SpectrumStreamRead {
    if (this.#closed) throw new MisoUsageError("the spectrum module is closed");
    const result = Number(this.#streamAnalysis());
    const metadata = this.streamMetadata(query);
    if (result !== RESULT_OK) throw refusal("the spectrum stream analyzer refused the capture", result);
    return Object.freeze({ metadata, result: this.#decodeCurrent(query, result) });
  }

  /** Analyze one stream window in a caller-owned reusable transfer buffer. */
  analyzeStreamBuffer(
    query: SpectrumQuery,
    buffer: ArrayBuffer,
    byteLength: number,
    importedMetadata: SpectrumStreamMetadata,
  ): SpectrumEncodedStreamRead {
    if (this.#closed) throw new MisoUsageError("the spectrum module is closed");
    const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes
      ?? this.#contract.maximumCaptureBytes;
    if (!Number.isSafeInteger(maximumCaptureBytes) || maximumCaptureBytes < 1
        || maximumCaptureBytes > this.#contract.maximumCaptureBytes) {
      throw new MisoUsageError("maximumCaptureBytes exceeds the spectrum bound");
    }
    if (!(buffer instanceof ArrayBuffer) || !Number.isSafeInteger(byteLength)
        || byteLength < this.#contract.window.bytes || byteLength > buffer.byteLength
        || byteLength > this.#contract.maximumCaptureBytes || byteLength > maximumCaptureBytes) {
      throw malformed("the reusable spectrum capture buffer is malformed");
    }
    stageSpectrumRequest(this.#exports, query);
    const capturePointer = Number(this.#capturePtr());
    const captureCapacity = Number(this.#captureCapacity());
    if (capturePointer <= 0 || captureCapacity !== this.#contract.maximumCaptureBytes) {
      throw malformed("invalid spectrum capture staging");
    }
    const bytes = bytesView(this.#exports);
    range(bytes, capturePointer, byteLength, "capture");
    bytes.set(new Uint8Array(buffer, 0, byteLength), capturePointer);
    const set = Number(this.#captureSetBytes(byteLength));
    if (set !== RESULT_OK) throw refusal("the spectrum capture was refused", set);
    writeStreamMetadata(this.#exports, query, importedMetadata, this.#contract);
    const configured = Number(this.#streamAnalysisConfigure());
    if (configured !== RESULT_OK) throw refusal("the spectrum stream metadata was refused", configured);
    const result = Number(this.#streamAnalysis());
    const metadata = this.streamMetadata(query);
    if (result !== RESULT_OK) throw refusal("the spectrum stream analyzer refused the capture", result);
    const pointer = Number(this.#resultPtr());
    const length = Number(this.#resultBytes());
    if (pointer <= 0 || !Number.isSafeInteger(length)
        || length < this.#contract.result.bytes || length > buffer.byteLength
        || length > maximumCaptureBytes) {
      if (length > maximumCaptureBytes) throw refusal("the spectrum result exceeded its bound", RESULT_REFUSED_BUDGET);
      throw malformed("invalid reusable spectrum result staging");
    }
    range(bytes, pointer, length, "result");
    new Uint8Array(buffer, 0, length).set(new Uint8Array(bytes.buffer, pointer, length));
    return Object.freeze({ metadata, resultByteLength: length });
  }

  #analyzeCurrent(query: SpectrumQuery): SpectrumResult {
    return this.#decodeCurrent(query, Number(this.#analysis()));
  }

  #decodeCurrent(query: SpectrumQuery, result: number): SpectrumResult {
    const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes
      ?? this.#contract.maximumCaptureBytes;
    if (!Number.isSafeInteger(maximumCaptureBytes) || maximumCaptureBytes < 1
        || maximumCaptureBytes > this.#contract.maximumCaptureBytes) {
      throw new MisoUsageError("maximumCaptureBytes exceeds the spectrum bound");
    }
    const captureBytes = Number(this.#captureBytes());
    if (!Number.isSafeInteger(captureBytes) || captureBytes < this.#contract.window.bytes
        || captureBytes > this.#contract.maximumCaptureBytes || captureBytes > maximumCaptureBytes) {
      if (captureBytes > maximumCaptureBytes) throw refusal("the spectrum capture exceeded its bound", RESULT_REFUSED_BUDGET);
      throw malformed("invalid spectrum capture length");
    }
    const pointer = Number(this.#resultPtr());
    const length = Number(this.#resultBytes());
    const bytes = bytesView(this.#exports);
    if (pointer <= 0 || !Number.isSafeInteger(length) || length < this.#contract.result.bytes
        || length > this.#contract.maximumCaptureBytes || length > maximumCaptureBytes) {
      if (length > maximumCaptureBytes) throw refusal("the spectrum result exceeded its bound", RESULT_REFUSED_BUDGET);
      throw malformed("invalid spectrum result staging");
    }
    range(bytes, pointer, length, "result");
    const payload = bytes.slice(pointer, pointer + length);
    return decodeSpectrumResult(query, result, payload);
  }
}

/** @internal Decode an owned encoded result after the stream buffer returns to the SDK. */
export function decodeSpectrumResult(
  query: SpectrumQuery,
  result: number,
  payload: Uint8Array,
): SpectrumResult {
  const shape = contract();
  const layout = shape.result;
  const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
  const at = (name: string) => field(layout, name).offset;
  const u32 = (name: string) => view.getUint32(at(name), true);
  const u64 = (name: string) => view.getBigUint64(at(name), true);
  if (u32("structSize") !== layout.bytes || u32("abiVersion") !== shape.abiVersion) {
    throw malformed("the engine returned a malformed spectrum result header");
  }
  if (result !== RESULT_OK || u32("result") !== RESULT_OK) {
    throw refusal("the spectrum analyzer refused the capture", u32("result"));
  }
  const target = targetValue(query.target);
  const channels = u32("channels");
  if (u32("target") !== target || channels !== channelValue(query.channels)
      || u32("sampleRateHz") <= 0 || u32("windowFrames") !== shape.windowFrames
      || u32("binCount") !== shape.binCount || u32("sourceUnderrun") > 1
      || !Number.isFinite(view.getFloat32(at("floorDb"), true))
      || u64("endSample") !== u64("capturedSample") + BigInt(shape.windowFrames)
      || u64("resultBytes") !== BigInt(payload.byteLength)
      || u32("reserved0") !== 0) {
    throw malformed("the engine returned a malformed spectrum result");
  }
  const count = shape.binCount;
  const frequenciesHz = readF32Array(payload, u32("frequenciesOffset"), count, "frequency axis");
  const left = channels === value(CHANNELS, "left") || channels === value(CHANNELS, "both")
    ? readF32Array(payload, u32("leftOffset"), count, "left spectrum") : undefined;
  const right = channels === value(CHANNELS, "right") || channels === value(CHANNELS, "both")
    ? readF32Array(payload, u32("rightOffset"), count, "right spectrum") : undefined;
  const targetCopy = query.target.kind === "output"
    ? Object.freeze({ kind: query.target.kind, outputId: query.target.outputId })
    : Object.freeze({ kind: query.target.kind, trackId: query.target.trackId });
  return Object.freeze({
    target: targetCopy,
    channels: channelName(channels),
    sampleRateHz: u32("sampleRateHz"),
    windowFrames: u32("windowFrames"),
    binCount: u32("binCount"),
    floorDb: view.getFloat32(at("floorDb"), true),
    frequenciesHz,
    ...(left === undefined ? {} : { leftDb: left }),
    ...(right === undefined ? {} : { rightDb: right }),
    capturedSample: u64("capturedSample"),
    endSample: u64("endSample"),
    snapshotToken: u64("snapshotToken"),
    graphSourceUnderrun: u32("sourceUnderrun") !== 0,
    resultBytes: u64("resultBytes"),
  });
}
