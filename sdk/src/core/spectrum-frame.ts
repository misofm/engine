import {
  FLAG_REVISION_ANY,
  WIRE_BOOL,
  WIRE_F32,
  WIRE_MESSAGE,
  WIRE_PACKED_F32,
  WIRE_U32,
  WIRE_U64,
  WIRE_U8,
  WIRE_UTF8,
  DECODER,
  commandFrame,
  concat,
  decodeRefusalFields,
  decodeResponseEnvelope,
  field,
  lookup,
  malformed,
  messageBody,
  requireId,
  requireU32,
  requireU64,
  scalar,
  text,
  u32,
  u8,
} from "./control-frame.ts";
import type { ControlBackpressure, ControlStatusName, RawField } from "./control-frame.ts";
import { MisoUsageError } from "./errors.ts";
import type { ProtocolDiagnostic } from "./protocol-messages.ts";
import type { Channel } from "./types.ts";
import type { SpectrumTarget } from "./spectrum.ts";

/**
 * The SDK's one codec of the three spectrum frames (#1596 D1): `SPECTRUM_CONFIGURE` (`0x0010`,
 * #1594 D2-D3), `SPECTRUM_CAPTURE` (`0x0011`, #1595 D2) and `SPECTRUM_STREAM` (`0x0012`, #1595 D3),
 * the frames a C ABI host sends to the same `SessionState::command`.
 *
 * # What this file does not do
 *
 * It checks nothing the core checks (#1596 D6): no budget, no hop value, no identity length, no
 * duplicate entry, no smoothing range. It refuses, with `MisoUsageError` before any byte, only a value
 * that has no wire spelling: a target kind or a channel mask the code tables do not hold, an identity
 * that is not a string, a smoothing that is not a finite number, a number its field cannot carry.
 *
 * # The wire it writes
 *
 * MISO Control BTLV v1, as `control-frame.ts` writes it. The field numbers, wire types and mandatory
 * flags are the protocol schema's (`crates/protocol/src/schema.rs`: `spectrum_entry`,
 * `spectrum_configure_request`, `spectrum_configured`, `spectrum_capture_request`,
 * `spectrum_capture_reply`, `spectrum_stream_request`, `spectrum_stream_reply`). The three commands
 * take any revision. `crates/protocol/tests/fixtures/spectrum-frames.json` holds the Rust codec's
 * bytes for a fixed list of requests and replies, and `sdk/test/control-frame-evals.mjs` holds this
 * codec to it.
 */

/** @internal */
export const MESSAGE_SPECTRUM_CONFIGURE = 0x0010;
/** @internal */
export const MESSAGE_SPECTRUM_CAPTURE = 0x0011;
/** @internal */
export const MESSAGE_SPECTRUM_STREAM = 0x0012;

const WIRE_F64 = 7;

/** The bins of one analyzed window, DC through Nyquist (`SPECTRUM_CAPTURE_BIN_COUNT`). @internal */
export const SPECTRUM_FRAME_BIN_COUNT = 1_025;
/** The frames of one spectrum window (`SPECTRUM_CAPTURE_WINDOW_FRAMES`). @internal */
export const SPECTRUM_FRAME_WINDOW_FRAMES = 2_048;

const TARGET_CODES: ReadonlyMap<string, number> = new Map([
  ["trackPostInput", 1],
  ["trackPostPan", 2],
  ["output", 3],
]);
const CHANNEL_CODES: ReadonlyMap<string, number> = new Map([
  ["left", 1],
  ["right", 2],
  ["both", 3],
]);
const CHANNEL_NAMES: readonly (Channel | undefined)[] = [undefined, "left", "right", "both"];
const CAPTURE_STATUS_NAMES = [undefined, "done", "pending", "ready"] as const;
const STREAM_STATUS_NAMES = [
  "inactive",
  "warming",
  "pending",
  "gap",
  "failed",
  "stopped",
  "ready",
] as const;

/** One tap of a `SPECTRUM_CONFIGURE`. @internal */
export interface SpectrumFrameEntry {
  readonly target: SpectrumTarget;
  readonly channels: Channel;
}

/** One spectrum command, as `encodeSpectrumCommand` takes it. @internal */
export type SpectrumFrameRequest =
  | {
    readonly op: "configure";
    readonly form: "single" | "collection";
    readonly entries: readonly SpectrumFrameEntry[];
    readonly maximumCaptureBytes: bigint;
    readonly hopFrames?: number | undefined;
  }
  | { readonly op: "capture"; readonly capture: "arm" | "cancel" }
  | { readonly op: "capture"; readonly capture: "read"; readonly channels: Channel }
  | {
    readonly op: "capture";
    readonly capture: "select";
    readonly target: SpectrumTarget;
    readonly channels: Channel;
  }
  | { readonly op: "stream"; readonly stream: "start"; readonly smoothingMs: number }
  | {
    readonly op: "stream";
    readonly stream: "select";
    readonly target: SpectrumTarget;
    readonly channels: Channel;
    /** Absent keeps the running stream's smoothing. */
    readonly smoothingMs?: number | undefined;
  }
  | { readonly op: "stream"; readonly stream: "read" | "reset" | "stop" };

function targetId(target: SpectrumTarget): string {
  if (target === null || typeof target !== "object") {
    throw new MisoUsageError("a spectrum target must be an object");
  }
  return requireId(target.kind === "output" ? target.outputId : (target as { trackId?: unknown }).trackId,
    "a spectrum target identity");
}

function targetFields(kindId: number, idId: number, target: SpectrumTarget): Uint8Array[] {
  const kind = lookup(TARGET_CODES, target?.kind, "a spectrum target kind");
  return [
    field(kindId, WIRE_U8, false, u8(kind)),
    field(idId, WIRE_UTF8, false, text(targetId(target))),
  ];
}

function channelCode(channels: unknown): number {
  return lookup(CHANNEL_CODES, channels, "a spectrum channel mask");
}

function smoothing(value: unknown): Uint8Array {
  if (typeof value !== "number" || !Number.isFinite(value)) {
    throw new MisoUsageError("smoothingMs must be a finite number");
  }
  const out = new Uint8Array(8);
  new DataView(out.buffer).setFloat64(0, value, true);
  return out;
}

function u64(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function configureFields(request: Extract<SpectrumFrameRequest, { op: "configure" }>): Uint8Array[] {
  const form = request.form === "single" ? 1 : request.form === "collection" ? 2 : undefined;
  if (form === undefined) throw new MisoUsageError("a spectrum form must be single or collection");
  if (!Array.isArray(request.entries)) throw new MisoUsageError("spectrum entries must be an array");
  const fields = [field(1, WIRE_U8, true, u8(form))];
  for (const entry of request.entries) {
    if (entry === null || typeof entry !== "object") {
      throw new MisoUsageError("a spectrum entry must be an object");
    }
    const kind = lookup(TARGET_CODES, entry.target?.kind, "a spectrum target kind");
    fields.push(field(2, WIRE_MESSAGE, false, messageBody([
      field(1, WIRE_U8, true, u8(kind)),
      field(2, WIRE_UTF8, true, text(targetId(entry.target))),
      field(3, WIRE_U8, true, u8(channelCode(entry.channels))),
    ])));
  }
  fields.push(field(3, WIRE_U64, true, u64(requireU64(request.maximumCaptureBytes, "maximumCaptureBytes"))));
  if (request.hopFrames !== undefined) {
    fields.push(field(4, WIRE_U32, false, u32(requireU32(request.hopFrames, "hopFrames"))));
  }
  return fields;
}

function captureFields(request: Extract<SpectrumFrameRequest, { op: "capture" }>): Uint8Array[] {
  switch (request.capture) {
    case "arm":
      return [field(1, WIRE_U8, true, u8(1))];
    case "read":
      // The SDK never asks for the raw window (#1596 R3): `SpectrumResult` carries bins only.
      return [field(1, WIRE_U8, true, u8(2)), field(2, WIRE_U8, false, u8(channelCode(request.channels)))];
    case "cancel":
      return [field(1, WIRE_U8, true, u8(3))];
    case "select":
      return [
        field(1, WIRE_U8, true, u8(4)),
        field(2, WIRE_U8, false, u8(channelCode(request.channels))),
        ...targetFields(3, 4, request.target),
      ];
    default:
      throw new MisoUsageError("a spectrum capture operation must be arm, read, cancel or select");
  }
}

function streamFields(request: Extract<SpectrumFrameRequest, { op: "stream" }>): Uint8Array[] {
  switch (request.stream) {
    case "start":
      return [field(1, WIRE_U8, true, u8(1)), field(2, WIRE_F64, false, smoothing(request.smoothingMs))];
    case "select": {
      const channels = channelCode(request.channels);
      return [
        field(1, WIRE_U8, true, u8(2)),
        ...(request.smoothingMs === undefined ? [] : [field(2, WIRE_F64, false, smoothing(request.smoothingMs))]),
        ...targetFields(3, 4, request.target),
        field(5, WIRE_U8, false, u8(channels)),
      ];
    }
    case "read":
      return [field(1, WIRE_U8, true, u8(3))];
    case "reset":
      return [field(1, WIRE_U8, true, u8(4))];
    case "stop":
      return [field(1, WIRE_U8, true, u8(5))];
    default:
      throw new MisoUsageError("a spectrum stream operation must be start, select, read, reset or stop");
  }
}

/**
 * Encode one spectrum command frame. It takes any revision. Throws `MisoUsageError` before any byte
 * for a request that has no wire spelling (see the file's note); everything else is the core's to
 * refuse. @internal
 */
export function encodeSpectrumCommand(requestId: bigint, request: SpectrumFrameRequest): Uint8Array {
  const id = requireU64(requestId, "requestId");
  if (id === 0n) throw new MisoUsageError("requestId must not be zero");
  let message: number;
  let fields: Uint8Array[];
  if (request?.op === "configure") {
    message = MESSAGE_SPECTRUM_CONFIGURE;
    fields = configureFields(request);
  } else if (request?.op === "capture") {
    message = MESSAGE_SPECTRUM_CAPTURE;
    fields = captureFields(request);
  } else if (request?.op === "stream") {
    message = MESSAGE_SPECTRUM_STREAM;
    fields = streamFields(request);
  } else {
    throw new MisoUsageError("a spectrum command must be configure, capture or stream");
  }
  return commandFrame(message, FLAG_REVISION_ANY, id, 0n, fields.length, concat(fields));
}

// ----------------------------------------------------------------------------------------------
// Decoding.

/** A refused spectrum reply: the status and every diagnostic, the first one's code is the core's. @internal */
export interface SpectrumFrameRefusal {
  readonly ok: false;
  readonly requestId: bigint;
  readonly revision: bigint;
  readonly status: Exclude<ControlStatusName, "ok">;
  readonly diagnostics: readonly ProtocolDiagnostic[];
  readonly omittedDiagnostics: number;
  readonly backpressure: ControlBackpressure | undefined;
}

/** A decoded spectrum reply. @internal */
export type SpectrumFrameReply<T> =
  | { readonly ok: true; readonly requestId: bigint; readonly revision: bigint; readonly value: T }
  | SpectrumFrameRefusal;

/** The `SPECTRUM_CONFIGURE` success payload. @internal */
export interface SpectrumFrameConfigured {
  readonly configuredEpoch: bigint;
  readonly adoptedEpoch: bigint;
  readonly rebuild: boolean;
  /** Present exactly when `rebuild` is true. */
  readonly planEpoch: bigint | undefined;
}

/** The analyzed window of a ready `SPECTRUM_CAPTURE` read. @internal */
export interface SpectrumFrameWindow {
  readonly target: SpectrumTarget;
  readonly channels: Channel;
  readonly sampleRateHz: number;
  readonly windowFrames: number;
  readonly binCount: number;
  readonly floorDb: number;
  readonly capturedSample: bigint;
  readonly endSample: bigint;
  readonly snapshotToken: bigint;
  readonly sourceUnderrun: boolean;
  readonly frequenciesHz: Float32Array;
  readonly leftDb: Float32Array | undefined;
  readonly rightDb: Float32Array | undefined;
}

/** The `SPECTRUM_CAPTURE` success payload. @internal */
export interface SpectrumFrameCapture {
  readonly adoptedEpoch: bigint;
  readonly selectionEpoch: bigint;
  readonly status: "done" | "pending" | "ready";
  /** Present exactly when `status` is `ready`. */
  readonly window: SpectrumFrameWindow | undefined;
}

/** The `SPECTRUM_STREAM` success payload: the stream's metadata after the operation. @internal */
export interface SpectrumFrameStream {
  readonly adoptedEpoch: bigint;
  readonly selectionEpoch: bigint;
  readonly status: (typeof STREAM_STATUS_NAMES)[number];
  /** The selected entry; absent without a selection. */
  readonly target: SpectrumTarget | undefined;
  readonly channels: Channel | undefined;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly hopFrames: number;
  readonly captureEpoch: bigint;
  readonly sequence: bigint;
  readonly droppedCaptures: bigint;
  readonly windows: bigint;
  readonly capturedSample: bigint;
  readonly endSample: bigint;
  readonly analysisEpoch: bigint;
  readonly historyStartSample: bigint;
  readonly smoothingMs: number;
  readonly sourceUnderrun: boolean;
  /** The smoothed bins: present exactly when `status` is `ready`. */
  readonly bins: {
    readonly frequenciesHz: Float32Array;
    readonly leftDb: Float32Array | undefined;
    readonly rightDb: Float32Array | undefined;
  } | undefined;
}

function decodeReply<T>(
  bytes: Uint8Array,
  message: number,
  body: (fields: readonly RawField[]) => T,
): SpectrumFrameReply<T> {
  const { requestId, status, revision, fields } = decodeResponseEnvelope(bytes, message);
  if (status === "ok") return Object.freeze({ ok: true, requestId, revision, value: body(fields) });
  const { diagnostics, omittedDiagnostics, backpressure } = decodeRefusalFields(fields);
  return Object.freeze({ ok: false, requestId, revision, status, diagnostics, omittedDiagnostics, backpressure });
}

function need(fields: readonly RawField[], id: number, wire: number, width: number): DataView {
  const value = scalar(fields, id, wire, width);
  if (value === undefined) throw malformed(`field ${id} is missing`);
  return value;
}

function u8Of(fields: readonly RawField[], id: number): number | undefined {
  return scalar(fields, id, WIRE_U8, 1)?.getUint8(0);
}

function u32Of(fields: readonly RawField[], id: number): number {
  return need(fields, id, WIRE_U32, 4).getUint32(0, true);
}

function u64Of(fields: readonly RawField[], id: number): bigint {
  return need(fields, id, WIRE_U64, 8).getBigUint64(0, true);
}

function boolOf(fields: readonly RawField[], id: number): boolean {
  const value = need(fields, id, WIRE_BOOL, 1).getUint8(0);
  if (value > 1) throw malformed(`field ${id} is not a boolean`);
  return value === 1;
}

function textOf(fields: readonly RawField[], id: number): string | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return undefined;
  if (row.wire !== WIRE_UTF8) throw malformed(`field ${id} is not text`);
  try {
    return DECODER.decode(row.value);
  } catch {
    throw malformed(`field ${id} is not UTF-8`);
  }
}

/** A packed plane of exactly `SPECTRUM_FRAME_BIN_COUNT` words, copied out of the frame. */
function plane(fields: readonly RawField[], id: number): Float32Array | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return undefined;
  if (row.wire !== WIRE_PACKED_F32 || row.value.length !== SPECTRUM_FRAME_BIN_COUNT * 4) {
    throw malformed(`field ${id} is not a plane of ${SPECTRUM_FRAME_BIN_COUNT} words`);
  }
  const view = new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
  const out = new Float32Array(SPECTRUM_FRAME_BIN_COUNT);
  for (let index = 0; index < out.length; index += 1) out[index] = view.getFloat32(index * 4, true);
  return out;
}

function targetOf(kind: number, id: string): SpectrumTarget {
  if (id.length === 0) throw malformed("a spectrum target identity is empty");
  switch (kind) {
    case 1: return Object.freeze({ kind: "trackPostInput", trackId: id });
    case 2: return Object.freeze({ kind: "trackPostPan", trackId: id });
    case 3: return Object.freeze({ kind: "output", outputId: id });
    default: throw malformed(`spectrum target kind ${kind} is not registered`);
  }
}

function channelsOf(code: number | undefined): Channel {
  const name = code === undefined ? undefined : CHANNEL_NAMES[code];
  if (name === undefined) throw malformed("a spectrum channel mask is not registered");
  return name;
}

function holds(channels: Channel, plane: "left" | "right"): boolean {
  return channels === "both" || channels === plane;
}

/** Decode the reply to a `SPECTRUM_CONFIGURE`. @internal */
export function decodeSpectrumConfiguredReply(bytes: Uint8Array): SpectrumFrameReply<SpectrumFrameConfigured> {
  return decodeReply(bytes, MESSAGE_SPECTRUM_CONFIGURE, (fields) => {
    const rebuild = boolOf(fields, 4);
    const planEpoch = scalar(fields, 5, WIRE_U64, 8)?.getBigUint64(0, true);
    if (rebuild !== (planEpoch !== undefined)) throw malformed("a plan epoch is present exactly with a rebuild");
    // Field 1 repeats the header's committed revision.
    u64Of(fields, 1);
    return Object.freeze({
      configuredEpoch: u64Of(fields, 2),
      adoptedEpoch: u64Of(fields, 3),
      rebuild,
      planEpoch,
    });
  });
}

/** The fields a ready capture reply adds, in tag order (`spectrum_capture_reply::READY`). */
const CAPTURE_READY_FIELDS = [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19] as const;

/** Decode the reply to a `SPECTRUM_CAPTURE`. @internal */
export function decodeSpectrumCaptureReply(bytes: Uint8Array): SpectrumFrameReply<SpectrumFrameCapture> {
  return decodeReply(bytes, MESSAGE_SPECTRUM_CAPTURE, (fields) => {
    const code = u8Of(fields, 3);
    const status = code === undefined ? undefined : CAPTURE_STATUS_NAMES[code];
    if (status === undefined) throw malformed("a capture status is not registered");
    const adoptedEpoch = u64Of(fields, 1);
    const selectionEpoch = u64Of(fields, 2);
    if (status !== "ready") {
      if (fields.some((row) => (CAPTURE_READY_FIELDS as readonly number[]).includes(row.id))) {
        throw malformed("a capture reply that is not ready carries a window field");
      }
      return Object.freeze({ adoptedEpoch, selectionEpoch, status, window: undefined });
    }
    // The SDK never asks for the raw window (#1596 R3), so a reply that carries one is not its.
    if (fields.some((row) => row.id === 18 || row.id === 19)) {
      throw malformed("a capture reply carries a raw window the SDK did not ask for");
    }
    const channels = channelsOf(u8Of(fields, 6));
    const id = textOf(fields, 5);
    if (id === undefined) throw malformed("a ready capture reply has no target identity");
    const kind = u8Of(fields, 4);
    if (kind === undefined) throw malformed("a ready capture reply has no target kind");
    const windowFrames = u32Of(fields, 8);
    const binCount = u32Of(fields, 9);
    const floorDb = need(fields, 10, WIRE_F32, 4).getFloat32(0, true);
    const capturedSample = u64Of(fields, 11);
    const endSample = u64Of(fields, 12);
    const snapshotToken = u64Of(fields, 13);
    const frequenciesHz = plane(fields, 15);
    const leftDb = plane(fields, 16);
    const rightDb = plane(fields, 17);
    if (windowFrames !== SPECTRUM_FRAME_WINDOW_FRAMES || binCount !== SPECTRUM_FRAME_BIN_COUNT
        || !Number.isFinite(floorDb) || snapshotToken === 0n
        || endSample !== capturedSample + BigInt(windowFrames)
        || frequenciesHz === undefined
        || (leftDb !== undefined) !== holds(channels, "left")
        || (rightDb !== undefined) !== holds(channels, "right")) {
      throw malformed("a ready capture reply's window is not valid");
    }
    return Object.freeze({
      adoptedEpoch,
      selectionEpoch,
      status,
      window: Object.freeze({
        target: targetOf(kind, id),
        channels,
        sampleRateHz: u32Of(fields, 7),
        windowFrames,
        binCount,
        floorDb,
        capturedSample,
        endSample,
        snapshotToken,
        sourceUnderrun: boolOf(fields, 14),
        frequenciesHz,
        leftDb,
        rightDb,
      }),
    });
  });
}

/** The longest stream smoothing, in milliseconds (`SPECTRUM_SMOOTHING_MAXIMUM_MS`). */
const SMOOTHING_MAXIMUM_MS = 10_000;

/** Decode the reply to a `SPECTRUM_STREAM`. @internal */
export function decodeSpectrumStreamReply(bytes: Uint8Array): SpectrumFrameReply<SpectrumFrameStream> {
  return decodeReply(bytes, MESSAGE_SPECTRUM_STREAM, (fields) => {
    const code = u8Of(fields, 3);
    const status = code === undefined ? undefined : STREAM_STATUS_NAMES[code];
    if (status === undefined) throw malformed("a stream status is not registered");
    const kind = u8Of(fields, 4);
    const channelCodeValue = u8Of(fields, 6);
    if (kind === undefined || channelCodeValue === undefined) {
      throw malformed("a stream reply has no target kind or channel mask");
    }
    const id = textOf(fields, 5);
    // Without a selection the kind and the channels are `0` and the identity is absent.
    let target: SpectrumTarget | undefined;
    let channels: Channel | undefined;
    if (kind === 0 && id === undefined && channelCodeValue === 0) {
      target = undefined;
      channels = undefined;
    } else if (kind === 0 || id === undefined) {
      throw malformed("a stream reply's target is incomplete");
    } else {
      target = targetOf(kind, id);
      channels = channelsOf(channelCodeValue);
    }
    const capturedSample = u64Of(fields, 14);
    const endSample = u64Of(fields, 15);
    const sequence = u64Of(fields, 11);
    const windows = u64Of(fields, 13);
    const smoothingMs = need(fields, 18, WIRE_F64, 8).getFloat64(0, true);
    if (!Number.isFinite(smoothingMs) || smoothingMs < 0 || smoothingMs > SMOOTHING_MAXIMUM_MS) {
      throw malformed("a stream reply's smoothing is out of range");
    }
    const frequenciesHz = plane(fields, 20);
    const leftDb = plane(fields, 21);
    const rightDb = plane(fields, 22);
    let bins: SpectrumFrameStream["bins"];
    if (frequenciesHz === undefined) {
      if (leftDb !== undefined || rightDb !== undefined) throw malformed("a stream reply carries a plane without its axis");
      if (status === "ready") throw malformed("a ready stream reply carries no bins");
      bins = undefined;
    } else {
      if (status !== "ready" || channels === undefined
          || endSample !== capturedSample + BigInt(SPECTRUM_FRAME_WINDOW_FRAMES)
          || sequence + 1n !== windows
          || (leftDb !== undefined) !== holds(channels, "left")
          || (rightDb !== undefined) !== holds(channels, "right")) {
        throw malformed("a stream reply's bins are not valid");
      }
      bins = Object.freeze({ frequenciesHz, leftDb, rightDb });
    }
    return Object.freeze({
      adoptedEpoch: u64Of(fields, 1),
      selectionEpoch: u64Of(fields, 2),
      status,
      target,
      channels,
      sampleRateHz: u32Of(fields, 7),
      quantumFrames: u32Of(fields, 8),
      hopFrames: u32Of(fields, 9),
      captureEpoch: u64Of(fields, 10),
      sequence,
      droppedCaptures: u64Of(fields, 12),
      windows,
      capturedSample,
      endSample,
      analysisEpoch: u64Of(fields, 16),
      historyStartSample: u64Of(fields, 17),
      smoothingMs,
      sourceUnderrun: boolOf(fields, 19),
      bins,
    });
  });
}
