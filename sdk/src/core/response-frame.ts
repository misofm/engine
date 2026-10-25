import { RESPONSE_FRAME } from "../generated/response-frame.ts";
import {
  FLAG_REVISION_ANY,
  WIRE_BOOL,
  WIRE_F32,
  WIRE_MESSAGE,
  WIRE_PACKED_F32,
  WIRE_U16,
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
  nestedFields,
  readFields,
  requireU32,
  requireU64,
  scalar,
  text,
  u32,
  u8,
  DECODER,
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
export function responseRefusalResult(
  status: ControlStatusName,
): "invalidArgument" | "unsupported" | "refusedBudget" | "backpressure" | "internal" {
  switch (status) {
    case "invalid_field":
    case "validation_failed":
    case "not_found":
      return "invalidArgument";
    case "unavailable":
      return "unsupported";
    case "limit_exceeded":
      return "refusedBudget";
    case "backpressure":
      return "backpressure";
    default:
      return "internal";
  }
}

// ----------------------------------------------------------------------------------------------
// The strip target (#1579a D1): the request a C ABI host sends for a track's response, the tag its
// reply carries and the `RESPONSE_READY` event that answers the tag.

/** A strip `RESPONSE_GET`: the strip, the grid and the lanes; the core decides every refusal. */
export interface ResponseFrameStripRequest {
  readonly requestId: bigint;
  readonly stripId: string;
  readonly grid: ResponseFrameGrid;
  readonly points: number;
  readonly minimumHz: number;
  readonly maximumHz: number;
  readonly channels: ResponseFrameLane;
  readonly maximumReplyBytes?: number | undefined;
}

/**
 * Encode a `RESPONSE_GET` command for a strip target (field 3), without sections or overrides. It
 * takes any session revision. Throws `MisoUsageError`, before producing a byte, when a name has no
 * code in the frame table or a number is not one its field can carry.
 */
export function encodeStripResponseRequest(request: ResponseFrameStripRequest): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (typeof request.stripId !== "string") throw new MisoUsageError("the strip ID must be a string");
  const fields: Uint8Array[] = [
    putText(REQUEST.stripId, request.stripId),
    putU8(REQUEST.grid, code(GRID_CODES, request.grid, "the grid")),
    putU32(REQUEST.points, request.points, "points"),
    putF32(REQUEST.minimumHz, request.minimumHz, "minimumHz"),
    putF32(REQUEST.maximumHz, request.maximumHz, "maximumHz"),
    putU8(REQUEST.channels, code(LANE_CODES, request.channels, "the channels")),
    putBool(REQUEST.sections, false),
  ];
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

/** A decoded reply to a strip `RESPONSE_GET`: the capture's tag, or the refusal. */
export type ResponseFrameTagReply =
  | { readonly ok: true; readonly requestId: bigint; readonly tag: bigint }
  | {
      readonly ok: false;
      readonly requestId: bigint;
      readonly status: ControlStatusName;
      readonly diagnostics: readonly ControlDiagnostic[];
    };

/**
 * Decode the reply to a strip `RESPONSE_GET`: the request tag alone (field 1). A refusal is a value;
 * bytes that are not a valid reply raise `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeResponseTagReply(bytes: Uint8Array): ResponseFrameTagReply {
  const { requestId, status, fields } = decodeResponseEnvelope(bytes, RESPONSE_FRAME.command);
  if (status !== "ok") {
    const { diagnostics } = decodeRefusalBody(fields);
    return Object.freeze({ ok: false, requestId, status, diagnostics });
  }
  if (fields.length !== 1) throw malformed("a strip reply holds more than its request tag");
  return Object.freeze({ ok: true, requestId, tag: readU64(fields, RESULT.requestTag, "the request tag") });
}

/**
 * The fields of `RESPONSE_READY` (`0x8040`, #1578 D2) and of its member message. The generated frame
 * table holds the `RESPONSE_GET` messages only, so these IDs are written here; the C ABI fixture
 * (`sdk/test/fixtures/response-ready.json`, written by `crates/capi/tests/response_strip.rs`) holds
 * them to the codec's.
 */
const MESSAGE_RESPONSE_READY = 0x8040;
const EVENT_HEADER_BYTES = 48;
const KIND_EVENT = 3;
const EVENT_MAGIC = [0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00] as const;
const READY = {
  sequence: { id: 1, wire: WIRE_U64, mandatory: false },
  requestTag: { id: 2, wire: WIRE_U64, mandatory: false },
  stripId: { id: 3, wire: WIRE_UTF8, mandatory: false },
  outcome: { id: 4, wire: WIRE_U16, mandatory: false },
  diagnostic: { id: 5, wire: WIRE_UTF8, mandatory: false },
  capturedSample: { id: 6, wire: WIRE_U64, mandatory: false },
  sampleRateHz: { id: 7, wire: WIRE_U32, mandatory: false },
  floorDb: { id: 8, wire: WIRE_F32, mandatory: false },
  frequenciesHz: { id: 9, wire: WIRE_PACKED_F32, mandatory: false },
  leftDb: { id: 10, wire: WIRE_PACKED_F32, mandatory: false },
  rightDb: { id: 11, wire: WIRE_PACKED_F32, mandatory: false },
  excludedCount: { id: 12, wire: WIRE_U32, mandatory: false },
  member: { id: 13, wire: WIRE_MESSAGE, mandatory: false },
} as const;
const MEMBER = {
  section: { id: 1, wire: WIRE_U8, mandatory: false },
  slotId: { id: 2, wire: WIRE_UTF8, mandatory: false },
  effect: { id: 3, wire: WIRE_UTF8, mandatory: false },
  kind: { id: 4, wire: WIRE_U8, mandatory: false },
  bypassed: { id: 5, wire: WIRE_BOOL, mandatory: false },
  enabledLeft: { id: 6, wire: WIRE_U32, mandatory: false },
  enabledRight: { id: 7, wire: WIRE_U32, mandatory: false },
  sectionCount: { id: 8, wire: WIRE_U8, mandatory: false },
} as const;

/** The outcome statuses a `RESPONSE_READY` may carry, by wire number (#1578 D2). */
const READY_OUTCOMES: ReadonlyMap<number, ControlStatusName> = new Map([
  [0, "ok"],
  [6, "limit_exceeded"],
  [13, "not_found"],
  [14, "unavailable"],
  [18, "internal"],
]);

/** One owner a `RESPONSE_READY` names, as the frame carries it. */
export interface ResponseReadyMember {
  /** The section code: builtins `4`, console `5`, inserts `2`. */
  readonly section: number;
  /** The slot's stable ID, present exactly for the console and the inserts. */
  readonly slotId: string | undefined;
  readonly effect: string;
  /** Parametric EQ `1`, input filters `2`, unavailable `3`. */
  readonly kind: number;
  readonly bypassed: boolean;
  readonly enabledLeft: number;
  readonly enabledRight: number;
  readonly sectionCount: number;
}

/** The capture of a `RESPONSE_READY` whose outcome is `OK`. */
export interface ResponseReadyCapture {
  readonly capturedSample: bigint;
  readonly sampleRateHz: number;
  readonly floorDb: number;
  readonly frequenciesHz: Float32Array;
  readonly leftDb: Float32Array | undefined;
  readonly rightDb: Float32Array | undefined;
  readonly excludedCount: number;
  readonly members: readonly ResponseReadyMember[];
}

/** A decoded `RESPONSE_READY` event. */
export interface ResponseReadyEvent {
  readonly sequence: bigint;
  readonly requestTag: bigint;
  readonly stripId: string;
  readonly status: ControlStatusName;
  /** The dotted diagnostic code, present exactly when `status` is not `ok`. */
  readonly diagnostic: string | undefined;
  /** Present exactly when `status` is `ok`. */
  readonly capture: ResponseReadyCapture | undefined;
  /** The event frame's length in bytes. */
  readonly frameBytes: number;
}

/** The codec's limits (`ProtocolLimits::default`): fields per message level, bytes per string. */
const MAXIMUM_TLV_COUNT = 1_024;
const MAXIMUM_STRING_BYTES = 64 * 1_024;
/** The longest identity or diagnostic a `RESPONSE_READY` carries (`RESPONSE_READY_TEXT_BYTES`). */
const READY_TEXT_BYTES = 127;
/** The stable-ID grammar (`valid_stable_id`): a lowercase letter, then `[a-z0-9._-]`, 1 to 127 bytes. */
const STABLE_ID = /^[a-z][a-z0-9._-]{0,126}$/;

function readyText(value: string, what: string): string {
  if (new TextEncoder().encode(value).length > READY_TEXT_BYTES) throw malformed(`${what} is longer than 127 bytes`);
  return value;
}

function stableId(value: string, what: string): string {
  if (!STABLE_ID.test(value)) throw malformed(`${what} is not a stable ID`);
  return value;
}

function optionalText(fields: readonly RawField[], row: { readonly id: number }): string | undefined {
  const found = fields.filter((candidate) => candidate.id === row.id);
  if (found.length > 1) throw malformed(`field ${row.id} repeats`);
  const entry = found[0];
  if (entry === undefined) return undefined;
  if (entry.wire !== WIRE_UTF8 || entry.value.length === 0) throw malformed(`field ${row.id} is not text`);
  if (entry.value.length > MAXIMUM_STRING_BYTES) throw malformed(`field ${row.id} exceeds the string limit`);
  try {
    return DECODER.decode(entry.value);
  } catch {
    throw malformed(`field ${row.id} is not UTF-8`);
  }
}

function requiredText(fields: readonly RawField[], row: { readonly id: number }, what: string): string {
  const value = optionalText(fields, row);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function decodeMember(row: RawField): ResponseReadyMember {
  if (row.wire !== WIRE_MESSAGE) throw malformed("a response member is not a message");
  if (row.value.length >= 8
      && new DataView(row.value.buffer, row.value.byteOffset, 4).getUint32(0, true) > MAXIMUM_TLV_COUNT) {
    throw malformed("a response member holds too many fields");
  }
  const fields = nestedFields(row.value);
  const section = required(fields, MEMBER.section, 1, "a member's section").getUint8(0);
  const slot = optionalText(fields, MEMBER.slotId);
  const slotId = slot === undefined ? undefined : stableId(slot, "a member's slot ID");
  if (section !== 4 && section !== 5 && section !== 2) throw malformed(`a member names section ${section}`);
  if ((section === 4) !== (slotId === undefined)) throw malformed("a member's slot ID does not match its section");
  const sectionCount = required(fields, MEMBER.sectionCount, 1, "a member's section count").getUint8(0);
  const kind = required(fields, MEMBER.kind, 1, "a member's kind").getUint8(0);
  if (kind < 1 || kind > 3) throw malformed(`a member names kind ${kind}`);
  if (sectionCount > 32) throw malformed("a member has more than 32 sections");
  const enabledLeft = readU32(fields, MEMBER.enabledLeft, "a member's left mask");
  const enabledRight = readU32(fields, MEMBER.enabledRight, "a member's right mask");
  // No mask bit at or above the section count.
  const fits = (mask: number): boolean => sectionCount >= 32 || mask >>> sectionCount === 0;
  if (!fits(enabledLeft) || !fits(enabledRight)) throw malformed("a member's mask exceeds its sections");
  return Object.freeze({
    section,
    slotId,
    effect: readyText(requiredText(fields, MEMBER.effect, "a member's effect"), "a member's effect"),
    kind,
    bypassed: readBool(fields, MEMBER.bypassed, "a member's bypass"),
    enabledLeft,
    enabledRight,
    sectionCount,
  });
}

/** Whether an event frame's header names `RESPONSE_READY`; nothing past the header is read. */
export function isResponseReady(frame: Uint8Array): boolean {
  if (frame.length < EVENT_HEADER_BYTES) return false;
  const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
  return frame[14] === KIND_EVENT && view.getUint16(16, true) === MESSAGE_RESPONSE_READY;
}

/**
 * Read an event frame the engine encoded. Returns `undefined` for any event that is not
 * `RESPONSE_READY`. The frame's header and the core's validity rules (`response_ready_is_valid`
 * and the codec's limits) are checked: a frame that breaks one raises `MisoEngineError`
 * (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeResponseReady(frame: Uint8Array): ResponseReadyEvent | undefined {
  if (!isResponseReady(frame)) return undefined;
  const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
  for (let index = 0; index < EVENT_MAGIC.length; index += 1) {
    if (frame[index] !== EVENT_MAGIC[index]) throw malformed("the magic is wrong");
  }
  if (view.getUint16(8, true) !== 1 || view.getUint16(12, true) !== EVENT_HEADER_BYTES) {
    throw malformed("the event header is not version 1");
  }
  if (frame[15] !== 0 || view.getUint16(18, true) !== 0) throw malformed("the frame is not an event");
  if (view.getBigUint64(24, true) !== 0n || view.getUint32(44, true) !== 0) {
    throw malformed("an event carries a request ID or a reserved word");
  }
  if (EVENT_HEADER_BYTES + view.getUint32(20, true) !== frame.length) {
    throw malformed("the event payload length is wrong");
  }
  const count = view.getUint32(40, true);
  if (count > MAXIMUM_TLV_COUNT) throw malformed("the event holds too many fields");
  const fields = readFields(frame.subarray(EVENT_HEADER_BYTES), count);
  const outcome = required(fields, READY.outcome, 2, "the outcome").getUint16(0, true);
  const status = READY_OUTCOMES.get(outcome);
  if (status === undefined) throw malformed(`a response event carries outcome ${outcome}`);
  const diagnosticText = optionalText(fields, READY.diagnostic);
  const diagnostic = diagnosticText === undefined ? undefined : readyText(diagnosticText, "the diagnostic");
  if ((status === "ok") === (diagnostic !== undefined)) {
    throw malformed("a response event's diagnostic does not match its outcome");
  }
  let capture: ResponseReadyCapture | undefined;
  if (status === "ok") {
    const frequenciesHz = packed(fields, READY.frequenciesHz);
    if (frequenciesHz === undefined) throw malformed("the frequency axis is missing");
    // The core's own validity rules (`response_ready_is_valid`): at least two finite axis points,
    // a finite floor, at least one side, each finite and of the axis's length.
    const leftDb = packed(fields, READY.leftDb);
    const rightDb = packed(fields, READY.rightDb);
    const floorDb = readF32(fields, READY.floorDb, "the floor");
    const finite = (values: Float32Array): boolean => values.every(Number.isFinite);
    const sideFits = (side: Float32Array | undefined): boolean =>
      side === undefined || (side.length === frequenciesHz.length && finite(side));
    if (frequenciesHz.length < 2 || !finite(frequenciesHz) || !Number.isFinite(floorDb)
        || (leftDb === undefined && rightDb === undefined) || !sideFits(leftDb) || !sideFits(rightDb)) {
      throw malformed("a response event's curves are not valid");
    }
    capture = Object.freeze({
      capturedSample: readU64(fields, READY.capturedSample, "the captured sample"),
      sampleRateHz: readU32(fields, READY.sampleRateHz, "the sample rate"),
      floorDb,
      frequenciesHz,
      leftDb,
      rightDb,
      excludedCount: readU32(fields, READY.excludedCount, "the excluded count"),
      members: Object.freeze(fields.filter((row) => row.id === READY.member.id).map(decodeMember)),
    });
  } else if (fields.some((row) => row.id > READY.diagnostic.id)) {
    throw malformed("a refused response event carries a capture");
  }
  return Object.freeze({
    sequence: readU64(fields, READY.sequence, "the event sequence"),
    requestTag: readU64(fields, READY.requestTag, "the request tag"),
    stripId: stableId(requiredText(fields, READY.stripId, "the strip ID"), "the strip ID"),
    status,
    diagnostic,
    capture,
    frameBytes: frame.length,
  });
}
