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
  commandFrame,
  concat,
  decodeRefusalBody,
  decodeResponseEnvelope,
  field,
  malformed,
  requireU32,
  requireU64,
  scalar,
  text,
  u32,
  u8,
} from "./control-frame.ts";
import type { ControlDiagnostic, ControlStatusName, RawField } from "./control-frame.ts";

/**
 * The SDK's one codec of the `RESPONSE_GET` frame (`0x000e`) a response preview sends and the
 * reply it reads (decision 17, C13; #1574 D4). It is the frame a C ABI host sends: the core decides
 * every refusal, and this file checks nothing the core checks (no point count, no domain, no rate).
 * The field numbers are the protocol schema's (`crates/protocol/src/schema.rs`:
 * `response_request`, `response_configuration`, `response_override`, `response_result`).
 */

const MESSAGE_RESPONSE_GET = 0x000e;

const KIND_EFFECT = 1;
const KIND_INPUT_FILTERS = 2;
const GRID_CODES = new Map<string, number>([["linear", 1], ["logarithmic", 2]]);
const LANE_CODES = new Map<string, number>([["left", 1], ["right", 2], ["both", 3]]);
const QUALITY_CODES = new Map<string, number>([["draft", 1], ["normal", 2], ["high", 3]]);
const LINK_CODES = new Map<string, number>([["dual_mono", 1], ["maximum", 2], ["average", 3]]);

export type ResponseFrameLane = "left" | "right" | "both";

/** What a response is computed for. */
export type ResponseFrameTarget =
  | {
      readonly kind: "effect";
      readonly effect: string;
      readonly quality: "draft" | "normal" | "high";
      readonly linkMode: "dual_mono" | "maximum" | "average";
    }
  | { readonly kind: "inputFilters" };

/** One override row: a control ID, the lane it applies to and its value. Control 0 is the bypass. */
export interface ResponseFrameOverride {
  readonly control: number;
  readonly lane: ResponseFrameLane;
  readonly value: number;
}

/** A `RESPONSE_GET` request for a configuration target. */
export interface ResponseFrameRequest {
  readonly requestId: bigint;
  readonly target: ResponseFrameTarget;
  readonly grid: "linear" | "logarithmic";
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
  readonly channels: ResponseFrameLane;
  readonly sections: boolean;
  readonly overrides: readonly ResponseFrameOverride[];
  readonly configurationId: bigint;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly maximumPreparedBytes: bigint;
  readonly maximumStateBytes: bigint;
  readonly maximumScratchBytes: bigint;
  readonly maximumAutomationSpans: number;
  readonly maximumReplyBytes: number;
}

/** The curves of a successful reply. */
export interface ResponseFrameCurves {
  readonly sampleRateHz: number;
  readonly configurationId: bigint;
  readonly floorDb: number;
  readonly bypass: boolean;
  readonly sectionCount: number;
  readonly enabledLeft: number;
  readonly enabledRight: number;
  readonly retainedBytes: bigint;
  readonly frequenciesHz: Float32Array;
  readonly leftDb: Float32Array | undefined;
  readonly rightDb: Float32Array | undefined;
  readonly sectionsLeftDb: Float32Array | undefined;
  readonly sectionsRightDb: Float32Array | undefined;
}

/** A decoded reply: the curves, or the refusal's status and diagnostics. */
export type ResponseFrameReply =
  | { readonly ok: true; readonly requestId: bigint; readonly curves: ResponseFrameCurves }
  | {
      readonly ok: false;
      readonly requestId: bigint;
      readonly status: ControlStatusName;
      readonly diagnostics: readonly ControlDiagnostic[];
    };

function code(table: ReadonlyMap<string, number>, name: string): number {
  const found = table.get(name);
  if (found === undefined) throw malformed(`the request names ${name}, which has no code`);
  return found;
}

function u64Bytes(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function f32Bytes(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setFloat32(0, value, true);
  return out;
}

/** A nested message body: `count:u32, reserved:u32`, then its fields. */
function messageBody(fields: readonly Uint8Array[]): Uint8Array {
  const header = new Uint8Array(8);
  new DataView(header.buffer).setUint32(0, fields.length, true);
  return concat([header, ...fields]);
}

function configurationMessage(target: ResponseFrameTarget): Uint8Array {
  if (target.kind === "inputFilters") {
    return messageBody([field(1, WIRE_U8, true, u8(KIND_INPUT_FILTERS))]);
  }
  return messageBody([
    field(1, WIRE_U8, true, u8(KIND_EFFECT)),
    field(2, WIRE_UTF8, false, text(target.effect)),
    field(3, WIRE_U8, false, u8(code(QUALITY_CODES, target.quality))),
    field(4, WIRE_U8, false, u8(code(LINK_CODES, target.linkMode))),
  ]);
}

function overrideMessage(row: ResponseFrameOverride): Uint8Array {
  return messageBody([
    field(1, WIRE_U32, true, u32(requireU32(row.control, "a control ID"))),
    field(2, WIRE_U8, true, u8(code(LANE_CODES, row.lane))),
    field(3, WIRE_F32, true, f32Bytes(row.value)),
  ]);
}

/**
 * Encode a `RESPONSE_GET` command for a configuration target. It takes any session revision. Throws
 * before producing a byte when a number cannot be encoded.
 */
export function encodeResponseRequest(request: ResponseFrameRequest): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (requestId === 0n) throw malformed("the request ID is zero");
  const fields: Uint8Array[] = [
    field(1, WIRE_MESSAGE, false, configurationMessage(request.target)),
    field(4, WIRE_U8, true, u8(code(GRID_CODES, request.grid))),
    field(5, WIRE_U32, true, u32(requireU32(request.points, "points"))),
    field(6, WIRE_F32, true, f32Bytes(request.minimumHz)),
    field(7, WIRE_F32, true, f32Bytes(request.maximumHz)),
    field(8, WIRE_U8, true, u8(code(LANE_CODES, request.channels))),
    field(9, WIRE_BOOL, true, u8(request.sections ? 1 : 0)),
  ];
  for (const row of request.overrides) {
    fields.push(field(10, WIRE_MESSAGE, false, overrideMessage(row)));
  }
  fields.push(
    field(11, WIRE_U64, false, u64Bytes(requireU64(request.configurationId, "configurationId"))),
    field(12, WIRE_U32, false, u32(requireU32(request.sampleRateHz, "sampleRateHz"))),
    field(13, WIRE_U32, false, u32(requireU32(request.quantumFrames, "quantumFrames"))),
    field(14, WIRE_U64, false, u64Bytes(requireU64(request.maximumPreparedBytes, "maximumPreparedBytes"))),
    field(15, WIRE_U64, false, u64Bytes(requireU64(request.maximumStateBytes, "maximumStateBytes"))),
    field(16, WIRE_U64, false, u64Bytes(requireU64(request.maximumScratchBytes, "maximumScratchBytes"))),
    field(17, WIRE_U32, false, u32(requireU32(request.maximumAutomationSpans, "maximumAutomationSpans"))),
    field(18, WIRE_U32, false, u32(requireU32(request.maximumReplyBytes, "maximumReplyBytes"))),
  );
  return commandFrame(
    MESSAGE_RESPONSE_GET,
    FLAG_REVISION_ANY,
    requestId,
    0n,
    fields.length,
    concat(fields),
  );
}

function required(fields: readonly RawField[], id: number, wire: number, width: number, what: string): DataView {
  const value = scalar(fields, id, wire, width);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function packed(fields: readonly RawField[], id: number): Float32Array | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return undefined;
  if (row.wire !== WIRE_PACKED_F32 || row.value.length % 4 !== 0) {
    throw malformed(`field ${id} is not a packed f32 array`);
  }
  // The bytes are copied out of the frame: the result owns its arrays.
  const out = new Float32Array(row.value.length / 4);
  const view = new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
  for (let index = 0; index < out.length; index += 1) out[index] = view.getFloat32(index * 4, true);
  return out;
}

/**
 * Decode the reply to a `RESPONSE_GET` command. A refusal is a value (its status and
 * diagnostics); bytes that are not a valid reply to this command raise `MisoEngineError`
 * (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeResponseReply(bytes: Uint8Array): ResponseFrameReply {
  const { requestId, status, fields } = decodeResponseEnvelope(bytes, MESSAGE_RESPONSE_GET);
  if (status !== "ok") {
    const { diagnostics } = decodeRefusalBody(fields);
    return Object.freeze({ ok: false, requestId, status, diagnostics });
  }
  if (fields.some((row) => row.id === 1)) throw malformed("a configuration reply holds a request tag");
  const bypass = required(fields, 5, WIRE_BOOL, 1, "the bypass flag").getUint8(0);
  if (bypass > 1) throw malformed("the bypass flag is not a boolean");
  const frequenciesHz = packed(fields, 10);
  if (frequenciesHz === undefined) throw malformed("the frequency axis is missing");
  return Object.freeze({
    ok: true,
    requestId,
    curves: Object.freeze({
      sampleRateHz: required(fields, 2, WIRE_U32, 4, "the sample rate").getUint32(0, true),
      configurationId: required(fields, 3, WIRE_U64, 8, "the configuration ID").getBigUint64(0, true),
      floorDb: required(fields, 4, WIRE_F32, 4, "the floor").getFloat32(0, true),
      bypass: bypass === 1,
      sectionCount: required(fields, 6, WIRE_U32, 4, "the section count").getUint32(0, true),
      enabledLeft: required(fields, 7, WIRE_U32, 4, "the left enabled mask").getUint32(0, true),
      enabledRight: required(fields, 8, WIRE_U32, 4, "the right enabled mask").getUint32(0, true),
      retainedBytes: required(fields, 9, WIRE_U64, 8, "the retained bytes").getBigUint64(0, true),
      frequenciesHz,
      leftDb: packed(fields, 11),
      rightDb: packed(fields, 12),
      sectionsLeftDb: packed(fields, 13),
      sectionsRightDb: packed(fields, 14),
    }),
  });
}

/** The engine result name a refused reply's status maps to. */
export function responseRefusalResult(status: ControlStatusName): "invalidArgument" | "unsupported" | "refusedBudget" | "internal" {
  switch (status) {
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
