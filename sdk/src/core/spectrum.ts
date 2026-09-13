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

type SpectrumExport = (...args: number[]) => number | bigint;
type SpectrumExports = Record<string, unknown> & { readonly memory: WebAssembly.Memory };
type AbiField = Readonly<{ readonly name: string; readonly offset: number; readonly type?: string }>;
type AbiStructure = Readonly<{ readonly bytes: number; readonly fields: readonly AbiField[] }>;

const TARGETS = ABI_LAYOUT.constants.spectrumTargets;
const CHANNELS = ABI_LAYOUT.constants.spectrumChannels;

function structure(name: "spectrumRequest" | "spectrumWindow" | "spectrumResult"): AbiStructure {
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

function targetValue(target: SpectrumTarget): number {
  switch (target.kind) {
    case "trackPostInputBuiltins": return value(TARGETS, "trackPostInputBuiltins");
    case "trackPostMatrix": return value(TARGETS, "trackPostMatrix");
    case "output": return value(TARGETS, "output");
    default: throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
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

function contract() {
  return {
    request: structure("spectrumRequest"),
    window: structure("spectrumWindow"),
    result: structure("spectrumResult"),
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

  #analyzeCurrent(query: SpectrumQuery): SpectrumResult {
    const captureBytes = Number(this.#captureBytes());
    if (!Number.isSafeInteger(captureBytes) || captureBytes < this.#contract.window.bytes
        || captureBytes > this.#contract.maximumCaptureBytes) {
      throw malformed("invalid spectrum capture length");
    }
    const result = Number(this.#analysis());
    const pointer = Number(this.#resultPtr());
    const length = Number(this.#resultBytes());
    const bytes = bytesView(this.#exports);
    if (pointer <= 0 || !Number.isSafeInteger(length) || length < this.#contract.result.bytes
        || length > this.#contract.maximumCaptureBytes) throw malformed("invalid spectrum result staging");
    range(bytes, pointer, length, "result");
    const payload = bytes.slice(pointer, pointer + length);
    const layout = this.#contract.result;
    const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
    const at = (name: string) => field(layout, name).offset;
    const u32 = (name: string) => view.getUint32(at(name), true);
    const u64 = (name: string) => view.getBigUint64(at(name), true);
    if (u32("structSize") !== layout.bytes || u32("abiVersion") !== this.#contract.abiVersion) {
      throw malformed("the engine returned a malformed spectrum result header");
    }
    if (result !== RESULT_OK || u32("result") !== RESULT_OK) throw refusal("the spectrum analyzer refused the capture", u32("result"));
    const target = targetValue(query.target);
    const channels = u32("channels");
    if (u32("target") !== target || channels !== channelValue(query.channels)
        || u32("sampleRateHz") <= 0 || u32("windowFrames") !== this.#contract.windowFrames
        || u32("binCount") !== this.#contract.binCount || u32("sourceUnderrun") > 1
        || !Number.isFinite(view.getFloat32(at("floorDb"), true))
        || u64("endSample") !== u64("capturedSample") + BigInt(this.#contract.windowFrames)
        || u64("resultBytes") !== BigInt(payload.byteLength)
        || u32("reserved0") !== 0) {
      throw malformed("the engine returned a malformed spectrum result");
    }
    const count = this.#contract.binCount;
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
}
