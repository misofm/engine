import { ABI_LAYOUT } from "../generated/abi.ts";
import { constantValue } from "./abi.ts";
import { MisoEngineError, MisoUsageError, resultName } from "./errors.ts";
import type { ResponsePreviewGrid } from "./response.ts";
import type { Channel } from "./types.ts";

/** A bounded grid accepted by a live track-response query. */
export type TrackResponseGrid = ResponsePreviewGrid;

/** Bounds this live endpoint actually enforces. */
export interface TrackResponseLimits {
  readonly maximumResultBytes?: number;
  readonly requestDeadlineMs?: number;
}

/** One on-demand live track response request. */
export interface TrackResponseQuery {
  readonly trackId: string;
  readonly grid: TrackResponseGrid;
  readonly channels?: Channel;
  readonly responseLimits?: TrackResponseLimits;
}

/** One response-capable owner in the selected track's declared signal order. */
export interface TrackResponseMember {
  readonly trackId: string;
  readonly nativeId: string;
  readonly stableId: string;
  /** `input`, `simd1`, `dynamic`, or `simd2`. */
  readonly rack: "input" | "simd1" | "dynamic" | "simd2";
  /** Numeric native rack word, retained for callers that persist wire metadata. */
  readonly rackValue: number;
  /** Position in the actual declared response-owner order. */
  readonly slot: number;
  /** `parametricEq`, `inputFilters`, or `unavailable`. */
  readonly kind: "parametricEq" | "inputFilters" | "unavailable";
  readonly kindValue: number;
  readonly bypassed: boolean;
  readonly available: boolean;
  /** A declared owner without a provider remains in place with this reason. */
  readonly excludedReason?: "linearResponseUnavailable";
  readonly enabledLeft: readonly boolean[];
  readonly enabledRight: readonly boolean[];
}

/** The owned result of one live target response query. */
export interface TrackResponseResult {
  readonly trackId: string;
  readonly mode: "target";
  readonly meaning: "eqFilterSubtotal";
  readonly sampleRateHz: number;
  readonly floorDb: number;
  readonly frequenciesHz: Float32Array;
  /** The selected left subtotal, omitted when `channels` excludes left. */
  readonly leftDb?: Float32Array;
  /** The selected right subtotal, omitted when `channels` excludes right. */
  readonly rightDb?: Float32Array;
  readonly members: readonly TrackResponseMember[];
  readonly capturedSample: bigint;
  readonly snapshotToken: bigint;
  readonly excludedMemberCount: number;
  readonly resultBytes: bigint;
}

/** Raw capture bytes returned by the live browser host. */
export interface TrackResponseCapture {
  readonly result: number;
  readonly snapshot: Uint8Array;
}

/** Host-side live response request/reply shape used by the browser SDK. */
export interface TrackResponseHost {
  captureTrackResponse(request: TrackResponseHostRequest): Promise<TrackResponseCapture>;
}

export interface TrackResponseHostRequest {
  readonly trackId: string;
  readonly grid: 1 | 2;
  readonly channels: 1 | 2 | 3;
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
  readonly maximumResultBytes: number;
}

type LiveExport = (...args: number[]) => number;
type LiveExportTable = Record<string, unknown> & { readonly memory: WebAssembly.Memory };

type AbiField = Readonly<{ readonly name: string; readonly offset: number }>;
type AbiStructure = Readonly<{ readonly bytes: number; readonly fields: readonly AbiField[] }>;
type LiveContract = Readonly<{
  readonly abiVersion: number;
  readonly request: AbiStructure;
  readonly owner: AbiStructure;
  readonly section: AbiStructure;
  readonly result: AbiStructure;
  readonly maximumIdBytes: number;
  readonly maximumPoints: number;
  readonly maximumCaptureBytes: number;
  readonly maximumOwners: number;
  readonly resultOk: number;
  readonly resultWrongState: number;
  readonly resultRefusedBudget: number;
  readonly modeTarget: number;
  readonly meaningEqFilterSubtotal: number;
}>;

type GeneratedAbi = Readonly<{
  readonly abiVersion: number;
  readonly structures: Record<string, AbiStructure>;
  readonly constants: Record<string, unknown>;
}>;

function generatedAbi(): GeneratedAbi {
  return ABI_LAYOUT as unknown as GeneratedAbi;
}

function generatedLayout(name: string, expectedBytes: number): AbiStructure {
  const layout = generatedAbi().structures[name];
  if (layout === undefined || layout.bytes !== expectedBytes || !Array.isArray(layout.fields)) {
    throw new MisoUsageError(`the generated ABI layout is missing or changed: ${name}`);
  }
  return layout;
}

function generatedOffset(layout: AbiStructure, name: string): number {
  const field = layout.fields.find((candidate) => candidate.name === name);
  if (field === undefined || !Number.isSafeInteger(field.offset) || field.offset < 0) {
    throw new MisoUsageError(`the generated ABI layout has no valid field ${name}`);
  }
  return field.offset;
}

function enumValue(group: string, name: string): number {
  const rows = generatedAbi().constants[group];
  if (!Array.isArray(rows)) throw new MisoUsageError(`the generated ABI has no ${group} vocabulary`);
  const row = rows.find((candidate): candidate is { readonly name: string; readonly value: number } =>
    typeof candidate === "object" && candidate !== null
      && (candidate as { readonly name?: unknown }).name === name
      && Number.isSafeInteger((candidate as { readonly value?: unknown }).value));
  if (row === undefined) throw new MisoUsageError(`the generated ABI has no ${group}.${name}`);
  return row.value;
}

function generatedMaximum(name: string): number {
  const value = generatedAbi().constants[name];
  if (typeof value !== "number" || !Number.isSafeInteger(value) || value <= 0) {
    throw new MisoUsageError(`the generated ABI has no valid ${name}`);
  }
  return value;
}

function liveContract(): LiveContract {
  return {
    abiVersion: generatedAbi().abiVersion,
    request: generatedLayout("liveResponseRequest", 48),
    owner: generatedLayout("liveResponseOwner", 64),
    section: generatedLayout("liveResponseSection", 44),
    result: generatedLayout("liveResponseResult", 104),
    maximumIdBytes: generatedMaximum("maximumLiveResponseIdBytes"),
    maximumPoints: generatedMaximum("maximumLiveResponsePoints"),
    maximumCaptureBytes: generatedMaximum("liveResponseCaptureBytes"),
    maximumOwners: generatedMaximum("maximumLiveResponseOwners"),
    resultOk: constantValue("resultCodes", "ok"),
    resultWrongState: constantValue("resultCodes", "wrongState"),
    resultRefusedBudget: constantValue("resultCodes", "refusedBudget"),
    modeTarget: enumValue("liveResponseModes", "target"),
    meaningEqFilterSubtotal: enumValue("liveResponseMeanings", "eqFilterSubtotal"),
  };
}

function offset(layout: AbiStructure, name: string): number {
  return generatedOffset(layout, name);
}

function callable(exports: LiveExportTable, name: string): LiveExport {
  const value = exports[name];
  if (typeof value !== "function") throw new MisoUsageError(`the engine asset does not export ${name}`);
  return value as LiveExport;
}

function optionalCallable(exports: LiveExportTable, names: readonly string[]): LiveExport | undefined {
  for (const name of names) {
    const value = exports[name];
    if (typeof value === "function") return value as LiveExport;
  }
  return undefined;
}

function finiteInteger(value: number, name: string, minimum: number, maximum: number): number {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) {
    throw new MisoUsageError(`${name} must be an integer in ${minimum}..=${maximum}`);
  }
  return value;
}

function channelValue(value: Channel | undefined): number {
  if (value === undefined || value === "both") return 3;
  if (value === "left") return 1;
  if (value === "right") return 2;
  throw new MisoUsageError("channels must be left, right, or both");
}

function gridValue(grid: TrackResponseGrid, maximumPoints: number): { readonly kind: number; readonly points: number } {
  if (grid.kind !== "linear" && grid.kind !== "logarithmic") {
    throw new MisoUsageError("grid.kind must be linear or logarithmic");
  }
  const points = finiteInteger(grid.points, "grid.points", 2, maximumPoints);
  if (!Number.isFinite(grid.minimumHz) || !Number.isFinite(grid.maximumHz)) {
    throw new MisoUsageError("grid endpoints must be finite");
  }
  if (grid.kind === "logarithmic" && grid.minimumHz <= 0) {
    throw new MisoUsageError("logarithmic grid minimumHz must be positive");
  }
  return { kind: grid.kind === "linear" ? 1 : 2, points };
}

function maximumResultBytes(limits: TrackResponseLimits | undefined, maximumCaptureBytes: number): number {
  return finiteInteger(
    limits?.maximumResultBytes ?? maximumCaptureBytes,
    "maximumResultBytes",
    1,
    maximumCaptureBytes,
  );
}

function trackIdBytes(trackId: string, maximumIdBytes: number): Uint8Array<ArrayBuffer> {
  if (typeof trackId !== "string" || trackId.length === 0) {
    throw new MisoUsageError("trackId must be a nonempty string");
  }
  const bytes = new TextEncoder().encode(trackId);
  if (bytes.length === 0 || bytes.length > maximumIdBytes) {
    throw new MisoUsageError("trackId exceeds the live response identity bound");
  }
  return bytes;
}

function memoryView(exports: LiveExportTable): Uint8Array<ArrayBuffer> {
  return new Uint8Array(exports.memory.buffer) as Uint8Array<ArrayBuffer>;
}

function checkedRange(bytes: Uint8Array, offset: number, length: number, name: string): [number, number] {
  if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(length) || offset < 0 || length < 0) {
    throw new MisoEngineError(`the engine returned invalid live response ${name} bounds`, {
      phase: "output", code: "abiMismatch", result: 2,
    });
  }
  const end = offset + length;
  if (!Number.isSafeInteger(end) || end > bytes.byteLength) {
    throw new MisoEngineError(`the engine returned invalid live response ${name} bounds`, {
      phase: "output", code: "abiMismatch", result: 2,
    });
  }
  return [offset, end];
}

function readUtf8(bytes: Uint8Array, offset: number, length: number, name: string, maximumIdBytes: number): string {
  const [start, end] = checkedRange(bytes, offset, length, name);
  if (length === 0 || length > maximumIdBytes) {
    throw new MisoEngineError(`the engine returned an invalid live response ${name}`, {
      phase: "output", code: "abiMismatch", result: 2,
    });
  }
  try {
    const value = new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(start, end));
    if (value.length === 0) throw new Error("empty");
    return value;
  } catch {
    throw new MisoEngineError(`the engine returned invalid UTF-8 in live response ${name}`, {
      phase: "output", code: "abiMismatch", result: 2,
    });
  }
}

function invalidPayload(message: string): MisoEngineError {
  return new MisoEngineError(message, { phase: "output", code: "abiMismatch", result: 2 });
}

function responseError(result: number, contract: LiveContract): MisoEngineError {
  return new MisoEngineError("the live track response query was refused", {
    phase: result === contract.resultWrongState ? "lifecycle" : "output",
    code: resultName(result, "call"),
    result,
  });
}

function freezeMember(raw: {
  readonly trackId: string;
  readonly nativeId: string;
  readonly stableId: string;
  readonly rack: number;
  readonly slot: number;
  readonly kind: number;
  readonly bypassed: boolean;
  readonly available: boolean;
  readonly enabledLeft: readonly boolean[];
  readonly enabledRight: readonly boolean[];
}): TrackResponseMember {
  const rack = ["input", "simd1", "dynamic", "simd2"][raw.rack];
  if (rack === undefined) throw invalidPayload("the engine returned an unknown live response rack");
  const kind = raw.available
    ? raw.kind === 1 ? "parametricEq" : raw.kind === 2 ? "inputFilters" : undefined
    : "unavailable";
  if (kind === undefined) throw invalidPayload("the engine returned an unknown live response owner kind");
  return Object.freeze({
    trackId: raw.trackId,
    nativeId: raw.nativeId,
    stableId: raw.stableId,
    rack: rack as TrackResponseMember["rack"],
    rackValue: raw.rack,
    slot: raw.slot,
    kind,
    kindValue: raw.kind,
    bypassed: raw.bypassed,
    available: raw.available,
    ...(raw.available ? {} : { excludedReason: "linearResponseUnavailable" as const }),
    enabledLeft: Object.freeze([...raw.enabledLeft]),
    enabledRight: Object.freeze([...raw.enabledRight]),
  });
}

interface ParsedCapture {
  readonly capturedSample: bigint;
  readonly snapshotToken: bigint;
  readonly sampleRateHz: number;
  readonly members: readonly TrackResponseMember[];
}

/**
 * Synchronous native evaluator for a copied live response snapshot.
 *
 * The same instance is usable by a headless boundary between renders and by the browser response
 * Worker after it uploads bytes received from the real Worklet host. No caller configuration is
 * interpreted as live state; all member metadata and target words come from the captured bytes.
 */
export class TrackResponseModule {
  readonly #exports: LiveExportTable;
  readonly #requestPtr: LiveExport;
  readonly #requestBytes: LiveExport;
  readonly #trackIdPtr: LiveExport;
  readonly #trackIdCapacity: LiveExport;
  readonly #capture: LiveExport | undefined;
  readonly #analysis: LiveExport;
  readonly #resultPtr: LiveExport;
  readonly #resultBytes: LiveExport;
  readonly #close: LiveExport | undefined;
  readonly #snapshotPtr: LiveExport | undefined;
  readonly #snapshotCapacity: LiveExport | undefined;
  readonly #snapshotCommit: LiveExport | undefined;
  readonly #contract: LiveContract;
  #busy = false;
  #closed = false;

  constructor(instance: WebAssembly.Instance | LiveExportTable) {
    this.#contract = liveContract();
    const table = "exports" in instance
      ? instance.exports as unknown as LiveExportTable
      : instance;
    if (!(table.memory instanceof WebAssembly.Memory)) {
      throw new MisoUsageError("the live response asset does not export linear memory");
    }
    this.#exports = table;
    this.#requestPtr = callable(table, "miso_engine_web_v1_track_response_request_ptr");
    this.#requestBytes = callable(table, "miso_engine_web_v1_track_response_request_bytes");
    this.#trackIdPtr = callable(table, "miso_engine_web_v1_track_response_track_id_ptr");
    this.#trackIdCapacity = callable(table, "miso_engine_web_v1_track_response_track_id_capacity");
    this.#capture = optionalCallable(table, ["miso_engine_web_v1_track_response_capture"]);
    this.#analysis = callable(table, "miso_engine_web_v1_track_response_analysis");
    this.#resultPtr = callable(table, "miso_engine_web_v1_track_response_result_ptr");
    this.#resultBytes = callable(table, "miso_engine_web_v1_track_response_result_bytes");
    this.#close = optionalCallable(table, ["miso_engine_web_v1_track_response_close"]);
    this.#snapshotPtr = optionalCallable(table, [
      "miso_engine_web_v1_track_response_snapshot_ptr",
      "miso_engine_web_v1_track_response_upload_ptr",
    ]);
    this.#snapshotCapacity = optionalCallable(table, [
      "miso_engine_web_v1_track_response_snapshot_capacity",
      "miso_engine_web_v1_track_response_upload_capacity",
    ]);
    this.#snapshotCommit = optionalCallable(table, [
      "miso_engine_web_v1_track_response_snapshot_set_bytes",
      "miso_engine_web_v1_track_response_snapshot_bytes",
      "miso_engine_web_v1_track_response_upload",
      "miso_engine_web_v1_track_response_upload_bytes",
    ]);
  }

  close(): void {
    if (this.#closed) return;
    this.#close?.();
    this.#closed = true;
  }

  /** Query a live snapshot already captured by a host and copied into this instance. */
  querySnapshot(request: TrackResponseQuery, snapshot: Uint8Array): TrackResponseResult {
    return this.#run(request, undefined, snapshot);
  }

  /** Capture from an active headless owner, then evaluate that immutable captured state. */
  queryActive(request: TrackResponseQuery, handle: number): TrackResponseResult {
    if (this.#capture === undefined) throw new MisoUsageError("the engine asset cannot capture live responses");
    return this.#run(request, handle);
  }

  #run(request: TrackResponseQuery, handle: number | undefined, snapshot?: Uint8Array): TrackResponseResult {
    if (this.#closed) throw new MisoUsageError("the live response module is closed");
    if (this.#busy) throw new MisoUsageError("a live track response query is already in flight");
    this.#busy = true;
    try {
      const parsedRequest = this.#writeRequest(request);
      if (snapshot !== undefined) {
        const capture = this.#copySnapshot(snapshot, parsedRequest.maximumResultBytes);
        const parsed = this.#parseCapture(capture);
        const result = this.#analyze(parsedRequest);
        return this.#withCaptureMetadata(result, parsed, request.trackId);
      }
      if (handle === undefined || this.#capture === undefined) throw new MisoUsageError("a live response owner handle is required");
      const result = this.#capture(handle);
      if (result !== this.#contract.resultOk) throw responseError(result, this.#contract);
      const capture = this.#copyCurrentResult(parsedRequest.maximumResultBytes);
      const parsed = this.#parseCapture(capture);
      const analyzed = this.#analysis();
      if (analyzed !== this.#contract.resultOk) throw responseError(analyzed, this.#contract);
      const resultPayload = this.#readResult(parsedRequest.maximumResultBytes);
      const analyzedResult = this.#decodeResult(resultPayload, parsedRequest);
      return this.#withCaptureMetadata(analyzedResult, parsed, request.trackId);
    } finally {
      this.#busy = false;
    }
  }

  #writeRequest(request: TrackResponseQuery): {
    readonly channels: number;
    readonly maximumResultBytes: number;
    readonly points: number;
    readonly gridKind: number;
  } {
    const id = trackIdBytes(request.trackId, this.#contract.maximumIdBytes);
    const grid = gridValue(request.grid, this.#contract.maximumPoints);
    const channels = channelValue(request.channels);
    const maximum = maximumResultBytes(request.responseLimits, this.#contract.maximumCaptureBytes);
    if (this.#requestBytes() !== this.#contract.request.bytes) throw invalidPayload("the live response request size changed");
    const requestPtr = this.#requestPtr();
    const trackIdPtr = this.#trackIdPtr();
    const trackIdCapacity = this.#trackIdCapacity();
    if (!Number.isSafeInteger(requestPtr) || requestPtr <= 0 || !Number.isSafeInteger(trackIdPtr) || trackIdPtr <= 0
        || !Number.isSafeInteger(trackIdCapacity) || trackIdCapacity < id.length
        || trackIdCapacity !== this.#contract.maximumIdBytes) {
      throw invalidPayload("the engine returned invalid live response request staging");
    }
    const bytes = memoryView(this.#exports);
    checkedRange(bytes, requestPtr, this.#contract.request.bytes, "request");
    checkedRange(bytes, trackIdPtr, id.length, "track ID");
    bytes.set(id, trackIdPtr);
    const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
    const field = (name: string) => offset(this.#contract.request, name);
    view.setUint32(requestPtr + field("structSize"), this.#contract.request.bytes, true);
    view.setUint32(requestPtr + field("abiVersion"), this.#contract.abiVersion, true);
    view.setUint32(requestPtr + field("trackIdBytes"), id.length, true);
    view.setUint32(requestPtr + field("grid"), grid.kind, true);
    view.setUint32(requestPtr + field("channels"), channels, true);
    view.setUint32(requestPtr + field("points"), grid.points, true);
    view.setFloat32(requestPtr + field("minimumHz"), request.grid.minimumHz, true);
    view.setFloat32(requestPtr + field("maximumHz"), request.grid.maximumHz, true);
    view.setUint32(requestPtr + field("maximumResultBytes"), maximum, true);
    for (let index = 0; index < 3; index += 1) {
      view.setUint32(requestPtr + field("reserved") + index * 4, 0, true);
    }
    return { channels, maximumResultBytes: maximum, points: grid.points, gridKind: grid.kind };
  }

  #copySnapshot(snapshot: Uint8Array, maximumBytes: number): Uint8Array {
    if (this.#snapshotPtr === undefined || this.#snapshotCapacity === undefined || this.#snapshotCommit === undefined) {
      throw new MisoUsageError("the response asset cannot upload a live snapshot");
    }
    const pointer = this.#snapshotPtr();
    const capacity = this.#snapshotCapacity();
    if (!Number.isSafeInteger(pointer) || pointer <= 0 || !Number.isSafeInteger(capacity)
        || capacity !== this.#contract.maximumCaptureBytes
        || snapshot.byteLength > capacity || snapshot.byteLength > maximumBytes) {
      throw new MisoEngineError("the live response snapshot exceeded its bound", {
        phase: "output", code: "refusedBudget", result: this.#contract.resultRefusedBudget,
      });
    }
    const bytes = memoryView(this.#exports);
    checkedRange(bytes, pointer, snapshot.byteLength, "snapshot");
    bytes.set(snapshot, pointer);
    const result = this.#snapshotCommit(snapshot.byteLength);
    if (result !== this.#contract.resultOk) throw responseError(result, this.#contract);
    return snapshot.slice();
  }

  #copyCurrentResult(maximumBytes: number): Uint8Array {
    const pointer = this.#resultPtr();
    const length = this.#resultBytes();
    const bytes = memoryView(this.#exports);
    if (!Number.isSafeInteger(pointer) || pointer <= 0 || !Number.isSafeInteger(length)
        || length > maximumBytes || length > this.#contract.maximumCaptureBytes) {
      throw new MisoEngineError("the live response capture exceeded its requested bound", {
        phase: "output", code: "refusedBudget", result: this.#contract.resultRefusedBudget,
      });
    }
    checkedRange(bytes, pointer, length, "capture");
    return bytes.slice(pointer, pointer + length);
  }

  #parseCapture(capture: Uint8Array): ParsedCapture {
    const resultLayout = this.#contract.result;
    const resultOffset = (name: string) => offset(resultLayout, name);
    if (capture.byteLength < resultLayout.bytes) throw invalidPayload("the live response capture header is truncated");
    const view = new DataView(capture.buffer, capture.byteOffset, capture.byteLength);
    const u32 = (offset: number) => view.getUint32(offset, true);
    const u64 = (offset: number) => view.getBigUint64(offset, true);
    if (u32(resultOffset("structSize")) !== resultLayout.bytes
        || u32(resultOffset("abiVersion")) !== this.#contract.abiVersion
        || u32(resultOffset("result")) !== this.#contract.resultOk
        || u32(resultOffset("mode")) !== this.#contract.modeTarget
        || u32(resultOffset("meaning")) !== this.#contract.meaningEqFilterSubtotal
        || u32(resultOffset("points")) !== 0
        || u32(resultOffset("ownerRecordBytes")) !== this.#contract.owner.bytes
        || u32(resultOffset("sectionRecordBytes")) !== this.#contract.section.bytes
        || u32(resultOffset("reserved0")) !== 0 || u32(resultOffset("reserved1")) !== 0
        || u32(resultOffset("reserved") + 0) !== 0 || u32(resultOffset("reserved") + 4) !== 0
    ) {
      throw invalidPayload("the engine returned a malformed live response capture header");
    }
    const resultBytes = u64(resultOffset("resultBytes"));
    if (resultBytes !== BigInt(capture.byteLength)) throw invalidPayload("the live response capture length is inconsistent");
    const ownerCount = u32(resultOffset("ownerCount"));
    if (ownerCount > this.#contract.maximumOwners) throw invalidPayload("the live response owner count exceeds its bound");
    const ownerOffset = u32(resultOffset("ownersOffset"));
    const ownerEnd = ownerOffset + ownerCount * this.#contract.owner.bytes;
    if (!Number.isSafeInteger(ownerEnd) || ownerOffset < resultLayout.bytes || ownerEnd > capture.byteLength) {
      throw invalidPayload("the live response owner records exceed the capture");
    }
    const members: TrackResponseMember[] = [];
    let excluded = 0;
    const ownerLayout = this.#contract.owner;
    const ownerOffsetOf = (name: string) => offset(ownerLayout, name);
    for (let index = 0; index < ownerCount; index += 1) {
      const at = ownerOffset + index * ownerLayout.bytes;
      const trackId = readUtf8(capture, u32At(view, at + ownerOffsetOf("trackIdOffset")), u32At(view, at + ownerOffsetOf("trackIdBytes")), "trackId", this.#contract.maximumIdBytes);
      const nativeId = readUtf8(capture, u32At(view, at + ownerOffsetOf("nativeIdOffset")), u32At(view, at + ownerOffsetOf("nativeIdBytes")), "nativeId", this.#contract.maximumIdBytes);
      const stableId = readUtf8(capture, u32At(view, at + ownerOffsetOf("stableIdOffset")), u32At(view, at + ownerOffsetOf("stableIdBytes")), "stableId", this.#contract.maximumIdBytes);
      const rack = u32At(view, at + ownerOffsetOf("rack"));
      const slot = u32At(view, at + ownerOffsetOf("slot"));
      const kind = u32At(view, at + ownerOffsetOf("kind"));
      const bypassed = u32At(view, at + ownerOffsetOf("bypassed"));
      const availability = u32At(view, at + ownerOffsetOf("availability"));
      if (bypassed > 1 || availability > 1 || u32At(view, at + ownerOffsetOf("reserved")) !== 0) {
        throw invalidPayload("the live response owner record is malformed");
      }
      const left = this.#parseSections(capture, view, u32At(view, at + ownerOffsetOf("leftOffset")), u32At(view, at + ownerOffsetOf("leftCount")));
      const right = this.#parseSections(capture, view, u32At(view, at + ownerOffsetOf("rightOffset")), u32At(view, at + ownerOffsetOf("rightCount")));
      if (availability === 0) {
        if (kind !== 0 || left.length !== 0 || right.length !== 0) throw invalidPayload("the live response exclusion is malformed");
        excluded += 1;
      } else if (kind !== 1 && kind !== 2) {
        throw invalidPayload("the live response owner kind is malformed");
      }
      members.push(freezeMember({
        trackId, nativeId, stableId, rack, slot, kind, bypassed: bypassed !== 0,
        available: availability !== 0,
        enabledLeft: left.map((section) => section.enabled),
        enabledRight: right.map((section) => section.enabled),
      }));
    }
    if (u32(resultOffset("excludedCount")) !== excluded) throw invalidPayload("the live response exclusion count is inconsistent");
    const sampleRateHz = u32(resultOffset("sampleRateHz"));
    if (sampleRateHz <= 0) throw invalidPayload("the live response sample rate is invalid");
    return {
      capturedSample: u64(resultOffset("capturedSample")),
      snapshotToken: u64(resultOffset("snapshotToken")),
      sampleRateHz,
      members: Object.freeze(members),
    };
  }

  #parseSections(capture: Uint8Array, view: DataView, sectionOffset: number, count: number): readonly Readonly<{ enabled: boolean }>[] {
    if (count > 4) throw invalidPayload("the live response section count exceeds its bound");
    if (count === 0) {
      checkedRange(capture, sectionOffset, 0, "empty live response sections");
      return Object.freeze([]);
    }
    const end = sectionOffset + count * this.#contract.section.bytes;
    if (!Number.isSafeInteger(end) || sectionOffset < this.#contract.result.bytes || end > capture.byteLength) {
      throw invalidPayload("the live response sections exceed the capture");
    }
    const sections: Array<Readonly<{ enabled: boolean }>> = [];
    const sectionLayout = this.#contract.section;
    const sectionOffsetOf = (name: string) => offset(sectionLayout, name);
    for (let index = 0; index < count; index += 1) {
      const at = sectionOffset + index * sectionLayout.bytes;
      const wordCount = view.getUint32(at + sectionOffsetOf("wordCount"), true);
      if (view.getUint32(at + sectionOffsetOf("enabled"), true) > 1 || wordCount > 7
          || view.getUint32(at + sectionOffsetOf("kind"), true) === 0) {
        throw invalidPayload("the live response section record is malformed");
      }
      for (let word = wordCount; word < 7; word += 1) {
        if (view.getUint32(at + sectionOffsetOf("words") + word * 4, true) !== 0) {
          throw invalidPayload("the live response section padding is nonzero");
        }
      }
      sections.push(Object.freeze({ enabled: view.getUint32(at + sectionOffsetOf("enabled"), true) !== 0 }));
    }
    return Object.freeze(sections);
  }

  #analyze(request: {
    readonly channels: number;
    readonly maximumResultBytes: number;
    readonly points: number;
  }): TrackResponseResult {
    const result = this.#analysis();
    if (result !== this.#contract.resultOk) throw responseError(result, this.#contract);
    const payload = this.#readResult(request.maximumResultBytes);
    return this.#decodeResult(payload, request);
  }

  #readResult(maximumBytes: number): Uint8Array {
    const pointer = this.#resultPtr();
    const length = this.#resultBytes();
    const bytes = memoryView(this.#exports);
    if (!Number.isSafeInteger(pointer) || pointer <= 0 || !Number.isSafeInteger(length)
        || length > maximumBytes || length > this.#contract.maximumCaptureBytes) {
      throw new MisoEngineError("the live response result exceeded its requested bound", {
        phase: "output", code: "refusedBudget", result: this.#contract.resultRefusedBudget,
      });
    }
    checkedRange(bytes, pointer, length, "result");
    return bytes.slice(pointer, pointer + length);
  }

  #decodeResult(
    payload: Uint8Array,
    request: { readonly channels: number; readonly maximumResultBytes: number; readonly points: number },
  ): TrackResponseResult {
    const resultLayout = this.#contract.result;
    const resultOffset = (name: string) => offset(resultLayout, name);
    if (payload.byteLength < resultLayout.bytes) throw invalidPayload("the live response result header is truncated");
    const view = new DataView(payload.buffer, payload.byteOffset, payload.byteLength);
    const u32 = (offset: number) => view.getUint32(offset, true);
    const u64 = (offset: number) => view.getBigUint64(offset, true);
    if (u32(resultOffset("structSize")) !== resultLayout.bytes
        || u32(resultOffset("abiVersion")) !== this.#contract.abiVersion
        || u32(resultOffset("result")) !== this.#contract.resultOk
        || u32(resultOffset("mode")) !== this.#contract.modeTarget
        || u32(resultOffset("meaning")) !== this.#contract.meaningEqFilterSubtotal
        || u32(resultOffset("channels")) !== request.channels
        || u32(resultOffset("points")) !== request.points
        || u32(resultOffset("ownerRecordBytes")) !== this.#contract.owner.bytes
        || u32(resultOffset("sectionRecordBytes")) !== this.#contract.section.bytes
        || u32(resultOffset("reserved0")) !== 0 || u32(resultOffset("reserved1")) !== 0
        || u32(resultOffset("reserved")) !== 0 || u32(resultOffset("reserved") + 4) !== 0) {
      throw invalidPayload("the engine returned a malformed live response result header");
    }
    const resultBytes = u64(resultOffset("resultBytes"));
    if (resultBytes !== BigInt(payload.byteLength) || payload.byteLength > request.maximumResultBytes
        || payload.byteLength > this.#contract.maximumCaptureBytes) {
      throw new MisoEngineError("the live response result exceeded its requested bound", {
        phase: "output", code: "refusedBudget", result: this.#contract.resultRefusedBudget,
      });
    }
    const points = u32(resultOffset("points"));
    if (points < 2 || points > this.#contract.maximumPoints) throw invalidPayload("the live response point count is invalid");
    const frequencyOffset = u32(resultOffset("frequenciesOffset"));
    const frequencies = this.#copyF32(payload, frequencyOffset, points, "frequencies");
    const left = (request.channels & 1) !== 0 ? this.#copyF32(payload, u32(resultOffset("leftOffset")), points, "left") : undefined;
    const right = (request.channels & 2) !== 0 ? this.#copyF32(payload, u32(resultOffset("rightOffset")), points, "right") : undefined;
    const sampleRateHz = u32(resultOffset("sampleRateHz"));
    if (sampleRateHz === 0) throw invalidPayload("the live response sample rate is invalid");
    return {
      trackId: "",
      mode: "target",
      meaning: "eqFilterSubtotal",
      sampleRateHz,
      floorDb: -120,
      frequenciesHz: frequencies,
      ...(left === undefined ? {} : { leftDb: left }),
      ...(right === undefined ? {} : { rightDb: right }),
      members: Object.freeze([]),
      capturedSample: u64(resultOffset("capturedSample")),
      snapshotToken: u64(resultOffset("snapshotToken")),
      excludedMemberCount: u32(resultOffset("excludedCount")),
      resultBytes,
    };
  }

  #copyF32(payload: Uint8Array, offset: number, count: number, name: string): Float32Array {
    if (offset === 0) throw invalidPayload(`the live response ${name} vector is missing`);
    const [start, end] = checkedRange(payload, offset, count * 4, name);
    const values = new Float32Array(payload.slice(start, end).buffer);
    for (const value of values) {
      if (!Number.isFinite(value)) throw invalidPayload(`the live response ${name} vector is nonfinite`);
    }
    return values;
  }

  #withCaptureMetadata(result: TrackResponseResult, capture: ParsedCapture, trackId: string): TrackResponseResult {
    if (result.sampleRateHz !== capture.sampleRateHz || result.capturedSample !== capture.capturedSample
        || result.snapshotToken !== capture.snapshotToken) {
      throw invalidPayload("the live response result changed its captured identity");
    }
    if (capture.members.some((member) => member.trackId !== trackId)) {
      throw invalidPayload("the live response capture contains a different track");
    }
    return Object.freeze({
      ...result,
      trackId,
      members: capture.members,
      excludedMemberCount: capture.members.filter((member) => !member.available).length,
      capturedSample: capture.capturedSample,
      snapshotToken: capture.snapshotToken,
    });
  }
}

function u32At(view: DataView, offset: number): number {
  return view.getUint32(offset, true);
}
