import { RESPONSE_FRAME } from "../generated/response-frame.ts";
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
import { MisoUsageError } from "./errors.ts";

/**
 * The SDK's one codec of the `RESPONSE_GET` frame a response preview sends and the reply it reads
 * (decision 17, C13; #1574 D4; #1623). It is the frame a C ABI host sends: the core decides every
 * refusal, and this file checks nothing the core checks (no point count, no domain, no rate, no
 * quality, no addressing). It refuses only a value that has no wire spelling: a name the code
 * tables do not hold, or a number its field cannot carry.
 *
 * Every field ID, wire type, mandatory flag and enum code is the generated frame table's
 * (`RESPONSE_FRAME`, written from the protocol schema by `parameter-metadata
 * --print-response-frame`). A field helper takes the table row of the wire type it writes, so a
 * schema change of a field's type is a type error here, not a frame the core refuses.
 */

const REQUEST = RESPONSE_FRAME.messages.request.fields;
const CONFIGURATION = RESPONSE_FRAME.messages.configuration.fields;
const OVERRIDE = RESPONSE_FRAME.messages.override.fields;
const RESULT = RESPONSE_FRAME.messages.result.fields;

type Named<Table extends readonly { readonly name: string }[]> = Table[number]["name"];
type CodeRow = { readonly name: string; readonly code: number };

function codes(rows: readonly CodeRow[]): ReadonlyMap<string, number> {
  return new Map(rows.map((row) => [row.name, row.code] as const));
}

const KIND_CODES = codes(RESPONSE_FRAME.codes.configurationKind);
const GRID_CODES = codes(RESPONSE_FRAME.codes.grid);
const LANE_CODES = codes(RESPONSE_FRAME.codes.lane);
const QUALITY_CODES = codes(RESPONSE_FRAME.codes.quality);
const LINK_CODES = codes(RESPONSE_FRAME.codes.linkMode);

/** A lane, by the name the lane code table holds. */
export type ResponseFrameLane = Named<typeof RESPONSE_FRAME.codes.lane>;
/** How a response axis spaces its points. */
export type ResponseFrameGrid = Named<typeof RESPONSE_FRAME.codes.grid>;
/** The preparation quality a configuration names. */
export type ResponseFrameQuality = Named<typeof RESPONSE_FRAME.codes.quality>;
/** The channel-link mode a configuration names. */
export type ResponseFrameLinkMode = Named<typeof RESPONSE_FRAME.codes.linkMode>;

/** What a response is computed for. */
export type ResponseFrameTarget =
  | {
      readonly kind: Extract<Named<typeof RESPONSE_FRAME.codes.configurationKind>, "effect">;
      readonly effect: string;
      readonly quality: ResponseFrameQuality;
      readonly linkMode: ResponseFrameLinkMode;
    }
  | { readonly kind: Extract<Named<typeof RESPONSE_FRAME.codes.configurationKind>, "inputFilters"> };

/** One override row: a control ID, the lane it applies to and its value. Control 0 is the bypass. */
export interface ResponseFrameOverride {
  readonly control: number;
  readonly lane: ResponseFrameLane;
  readonly value: number;
}

/**
 * A `RESPONSE_GET` request for a configuration target. A limit left out is a field left out of the
 * frame, so the core's own default applies; the SDK holds no copy of it.
 */
export interface ResponseFrameRequest {
  readonly requestId: bigint;
  readonly target: ResponseFrameTarget;
  readonly grid: ResponseFrameGrid;
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
  readonly channels: ResponseFrameLane;
  readonly sections: boolean;
  readonly overrides: readonly ResponseFrameOverride[];
  readonly configurationId: bigint;
  readonly sampleRateHz: number;
  readonly quantumFrames: number;
  readonly maximumPreparedBytes?: bigint | undefined;
  readonly maximumStateBytes?: bigint | undefined;
  readonly maximumScratchBytes?: bigint | undefined;
  readonly maximumAutomationSpans?: number | undefined;
  readonly maximumReplyBytes?: number | undefined;
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

/** A table row of the wire type `Wire`: the field's ID and whether a frame must carry it. */
type Row<Wire extends number> = {
  readonly id: number;
  readonly wire: Wire;
  readonly mandatory: boolean;
};

function code(table: ReadonlyMap<string, number>, name: string, what: string): number {
  const found = table.get(name);
  if (found === undefined) throw new MisoUsageError(`${what} ${String(name)} has no wire code`);
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

function putU8(row: Row<typeof WIRE_U8>, value: number): Uint8Array {
  return field(row.id, row.wire, row.mandatory, u8(value));
}

function putBool(row: Row<typeof WIRE_BOOL>, value: boolean): Uint8Array {
  return field(row.id, row.wire, row.mandatory, u8(value ? 1 : 0));
}

function putText(row: Row<typeof WIRE_UTF8>, value: string): Uint8Array {
  return field(row.id, row.wire, row.mandatory, text(value));
}

function putMessage(row: Row<typeof WIRE_MESSAGE>, body: Uint8Array): Uint8Array {
  return field(row.id, row.wire, row.mandatory, body);
}

function putU32(row: Row<typeof WIRE_U32>, value: number, what: string): Uint8Array {
  return field(row.id, row.wire, row.mandatory, u32(requireU32(value, what)));
}

function putU64(row: Row<typeof WIRE_U64>, value: bigint, what: string): Uint8Array {
  return field(row.id, row.wire, row.mandatory, u64Bytes(requireU64(value, what)));
}

/**
 * An `f32` field: any number has a spelling (`NaN` and the infinities too), and the core refuses
 * the ones it does not take. A value that is not a number has none: it is a `MisoUsageError`,
 * never coerced (`"20"` is not 20, and `null` is not 0).
 */
function putF32(row: Row<typeof WIRE_F32>, value: number, what: string): Uint8Array {
  if (typeof value !== "number") throw new MisoUsageError(`${what} must be a number`);
  return field(row.id, row.wire, row.mandatory, f32Bytes(value));
}

/** A nested message body: `count:u32, reserved:u32`, then its fields. */
function messageBody(fields: readonly Uint8Array[]): Uint8Array {
  const header = new Uint8Array(8);
  new DataView(header.buffer).setUint32(0, fields.length, true);
  return concat([header, ...fields]);
}

function configurationMessage(target: ResponseFrameTarget): Uint8Array {
  const kind = putU8(CONFIGURATION.kind, code(KIND_CODES, target.kind, "the configuration kind"));
  if (target.kind === "inputFilters") return messageBody([kind]);
  return messageBody([
    kind,
    putText(CONFIGURATION.effect, target.effect),
    putU8(CONFIGURATION.quality, code(QUALITY_CODES, target.quality, "the quality")),
    putU8(CONFIGURATION.linkMode, code(LINK_CODES, target.linkMode, "the link mode")),
  ]);
}

function overrideMessage(row: ResponseFrameOverride): Uint8Array {
  return messageBody([
    putU32(OVERRIDE.control, row.control, "a control ID"),
    putU8(OVERRIDE.lane, code(LANE_CODES, row.lane, "the override lane")),
    putF32(OVERRIDE.value, row.value, "an override value"),
  ]);
}

/**
 * Encode a `RESPONSE_GET` command for a configuration target. It takes any session revision. Throws
 * `MisoUsageError`, before producing a byte, when a name has no code in the frame table or a number
 * is not one its field can carry.
 */
export function encodeResponseRequest(request: ResponseFrameRequest): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  const fields: Uint8Array[] = [
    putMessage(REQUEST.configuration, configurationMessage(request.target)),
    putU8(REQUEST.grid, code(GRID_CODES, request.grid, "the grid")),
    putU32(REQUEST.points, request.points, "points"),
    putF32(REQUEST.minimumHz, request.minimumHz, "minimumHz"),
    putF32(REQUEST.maximumHz, request.maximumHz, "maximumHz"),
    putU8(REQUEST.channels, code(LANE_CODES, request.channels, "the channels")),
    putBool(REQUEST.sections, request.sections),
  ];
  for (const row of request.overrides) {
    fields.push(putMessage(REQUEST.override, overrideMessage(row)));
  }
  fields.push(
    putU64(REQUEST.configurationId, request.configurationId, "configurationId"),
    putU32(REQUEST.sampleRateHz, request.sampleRateHz, "sampleRateHz"),
    putU32(REQUEST.quantumFrames, request.quantumFrames, "quantumFrames"),
  );
  if (request.maximumPreparedBytes !== undefined) {
    fields.push(putU64(REQUEST.maximumPreparedBytes, request.maximumPreparedBytes, "maximumPreparedBytes"));
  }
  if (request.maximumStateBytes !== undefined) {
    fields.push(putU64(REQUEST.maximumStateBytes, request.maximumStateBytes, "maximumStateBytes"));
  }
  if (request.maximumScratchBytes !== undefined) {
    fields.push(putU64(REQUEST.maximumScratchBytes, request.maximumScratchBytes, "maximumScratchBytes"));
  }
  if (request.maximumAutomationSpans !== undefined) {
    fields.push(putU32(REQUEST.maximumAutomationSpans, request.maximumAutomationSpans, "maximumAutomationSpans"));
  }
  if (request.maximumReplyBytes !== undefined) {
    fields.push(putU32(REQUEST.maximumReplyBytes, request.maximumReplyBytes, "maximumReplyBytes"));
  }
  return commandFrame(
    RESPONSE_FRAME.command,
    FLAG_REVISION_ANY,
    requestId,
    0n,
    fields.length,
    concat(fields),
  );
}

function required<Wire extends number>(
  fields: readonly RawField[],
  row: Row<Wire>,
  width: number,
  what: string,
): DataView {
  const value = scalar(fields, row.id, row.wire, width);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function readU32(fields: readonly RawField[], row: Row<typeof WIRE_U32>, what: string): number {
  return required(fields, row, 4, what).getUint32(0, true);
}

function readU64(fields: readonly RawField[], row: Row<typeof WIRE_U64>, what: string): bigint {
  return required(fields, row, 8, what).getBigUint64(0, true);
}

function readF32(fields: readonly RawField[], row: Row<typeof WIRE_F32>, what: string): number {
  return required(fields, row, 4, what).getFloat32(0, true);
}

function readBool(fields: readonly RawField[], row: Row<typeof WIRE_BOOL>, what: string): boolean {
  const value = required(fields, row, 1, what).getUint8(0);
  if (value > 1) throw malformed(`${what} is not a boolean`);
  return value === 1;
}

function packed(fields: readonly RawField[], row: Row<typeof WIRE_PACKED_F32>): Float32Array | undefined {
  const found = fields.filter((candidate) => candidate.id === row.id);
  if (found.length > 1) throw malformed(`field ${row.id} repeats`);
  const entry = found[0];
  if (entry === undefined) return undefined;
  if (entry.wire !== row.wire || entry.value.length % 4 !== 0) {
    throw malformed(`field ${row.id} is not a packed f32 array`);
  }
  // The bytes are copied out of the frame: the result owns its arrays.
  const out = new Float32Array(entry.value.length / 4);
  const view = new DataView(entry.value.buffer, entry.value.byteOffset, entry.value.byteLength);
  for (let index = 0; index < out.length; index += 1) out[index] = view.getFloat32(index * 4, true);
  return out;
}

/**
 * Decode the reply to a `RESPONSE_GET` command. A refusal is a value (its status and
 * diagnostics); bytes that are not a valid reply to this command raise `MisoEngineError`
 * (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeResponseReply(bytes: Uint8Array): ResponseFrameReply {
  const { requestId, status, fields } = decodeResponseEnvelope(bytes, RESPONSE_FRAME.command);
  if (status !== "ok") {
    const { diagnostics } = decodeRefusalBody(fields);
    return Object.freeze({ ok: false, requestId, status, diagnostics });
  }
  if (fields.some((row) => row.id === RESULT.requestTag.id)) {
    throw malformed("a configuration reply holds a request tag");
  }
  const bypass = readBool(fields, RESULT.bypass, "the bypass flag");
  const frequenciesHz = packed(fields, RESULT.frequenciesHz);
  if (frequenciesHz === undefined) throw malformed("the frequency axis is missing");
  return Object.freeze({
    ok: true,
    requestId,
    curves: Object.freeze({
      sampleRateHz: readU32(fields, RESULT.sampleRateHz, "the sample rate"),
      configurationId: readU64(fields, RESULT.configurationId, "the configuration ID"),
      floorDb: readF32(fields, RESULT.floorDb, "the floor"),
      bypass,
      sectionCount: readU32(fields, RESULT.sectionCount, "the section count"),
      enabledLeft: readU32(fields, RESULT.enabledLeft, "the left enabled mask"),
      enabledRight: readU32(fields, RESULT.enabledRight, "the right enabled mask"),
      retainedBytes: readU64(fields, RESULT.retainedBytes, "the retained bytes"),
      frequenciesHz,
      leftDb: packed(fields, RESULT.leftDb),
      rightDb: packed(fields, RESULT.rightDb),
      sectionsLeftDb: packed(fields, RESULT.sectionsLeftDb),
      sectionsRightDb: packed(fields, RESULT.sectionsRightDb),
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
