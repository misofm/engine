import { CONTROLS } from "../generated/controls.ts";
import type { ControlLane, ControlOwner, ControlSection } from "../generated/controls.ts";
import { MisoEngineError, MisoUsageError, parseDiagnostics, resultName } from "./errors.ts";
import type { DiagnosticPathSegment, ProtocolDiagnostic } from "./protocol-messages.ts";

/**
 * The SDK's one encoder of `SESSION_TRANSACTION_APPLY` frames of `SET_CONTROL` edits, and its one
 * decoder of the reply (decision 17, C5; #1563 D1).
 *
 * # What this file does not do
 *
 * It checks no domain. A value is a JavaScript number, and the core refuses an out-of-range one with
 * its own `control.domain`; an unknown strip or row comes back as `control.address.unknown`. The only
 * refusals here make the frame unencodable (an ID that is not a string, a ramp that is not a `u32`).
 *
 * # The wire it writes
 *
 * MISO Control BTLV v1 (`crates/protocol`): a 48-byte little-endian outer header, then fields of
 * `id:u16, wire:u8, flags:u8, length:u32`, the value, and zero padding to 8 bytes. A nested message
 * starts with `count:u32, reserved:u32`. The field numbers below are the protocol schema's
 * (`crates/protocol/src/schema.rs`: `session::transaction`, `session::edit`, `session::set_control`,
 * `set_control_address`); `crates/protocol/tests/fixtures/set-control-frames.json` holds the Rust codec's
 * bytes for a fixed list of edits and `sdk/test/control-frame-evals.mjs` compares this encoder to it.
 */

const HEADER_BYTES = 48;
const MAGIC = [0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00] as const;
const PROTOCOL_MAJOR = 1;
const PROTOCOL_MINOR = 0;
const KIND_COMMAND = 1;
const KIND_RESPONSE = 2;
const MESSAGE_CAPABILITIES_GET = 0x0001;
/** @internal */
export const MESSAGE_SESSION_TRANSACTION_APPLY = 0x0003;
/** `SESSION_REPLACE` (#1386): replace the session with a document passed beside the frame. @internal */
export const MESSAGE_SESSION_REPLACE = 0x0013;
/** Header flag byte: the command takes any session revision (`CAPABILITIES_GET` requires it). */
/** @internal */
export const FLAG_REVISION_ANY = 1;
const EDIT_SET_CONTROL = 0x0800;
const U32_MAXIMUM = 0xffff_ffff;
const U64_MAXIMUM = 0xffff_ffff_ffff_ffffn;

// The protocol's wire type codes (`crates/protocol/src/schema.rs`, `Wire`).
/** @internal */
export const WIRE_U8 = 1;
/** @internal */
export const WIRE_U16 = 2;
/** @internal */
export const WIRE_U32 = 3;
/** @internal */
export const WIRE_U64 = 4;
/** @internal */
export const WIRE_F32 = 6;
/** @internal */
export const WIRE_BOOL = 8;
/** @internal */
export const WIRE_UTF8 = 9;
/** @internal */
export const WIRE_BYTES = 10;
/** @internal */
export const WIRE_MESSAGE = 11;
/** @internal */
export const WIRE_PACKED_U16 = 12;
/** @internal */
export const WIRE_PACKED_U32 = 13;
/** @internal */
export const WIRE_PACKED_U64 = 14;
/** @internal */
export const WIRE_PACKED_F32 = 15;

/** The stable address of one lane of one control, by the row's numeric ID. */
export interface ControlWireAddress {
  /** The owner: the session, or a strip, route or VCA by stable ID. */
  readonly owner: ControlOwner;
  /** The part of the owner: its own rows, builtins, a console slot or an insert. */
  readonly section: ControlSection;
  /**
   * The row's ID inside the section (the core's `control` number). In an insert or console section
   * `0` is the effect's bypass and every other value is the effect's parameter ID.
   */
  readonly control: number;
  /** `left`, `right`, or `both` lanes at once. */
  readonly lane: ControlLane;
  /**
   * The native effect (its catalog ID, `miso.compressor`) the edit was written for, in a console or
   * insert section only (#1678). The numeric `control` of such a row depends on the effect, and a
   * rebuild can replace the effect an insert or slot runs: the core refuses the edit with
   * `control.effect.changed` when what runs there now is another effect, and applies it to whatever
   * runs there when this is absent.
   */
  readonly expectedEffect?: string;
}

/** One `SET_CONTROL` edit: a value for one lane of one control, with an optional ramp length. */
export interface ControlFrameEdit {
  readonly address: ControlWireAddress;
  /** Sent as an IEEE 754 binary32. The core refuses what the row's domain does not allow. */
  readonly value: number;
  /** The edit's own ramp in samples, a `u32`. Absent leaves the choice to the core's row default. */
  readonly rampSamples?: number;
}

/** What a frame of `SET_CONTROL` edits needs besides the edits. */
export interface ControlTransactionRequest {
  /** A nonzero `u64` the endpoint correlates the reply by and refuses to see twice. */
  readonly requestId: bigint;
  /** The committed revision this transaction was built against, as the reply last reported it. */
  readonly expectedRevision: bigint;
  readonly edits: readonly ControlFrameEdit[];
}

/** @internal */
export const OWNER_CODES = new Map<string, number>(
  CONTROLS.address.ownerKinds.map((row) => [row.name, row.code] as const),
);
const SECTION_CODES = new Map<string, number>(
  CONTROLS.address.sectionKinds.map((row) => [row.name, row.code] as const),
);
const LANE_CODES = new Map<string, number>(
  CONTROLS.address.lanes.map((row) => [row.name, row.code] as const),
);

const ENCODER = new TextEncoder();
/** @internal */
export const DECODER = new TextDecoder("utf-8", { fatal: true });

/** @internal */
export function pad(length: number): number {
  return (8 - (length & 7)) & 7;
}

/** A field: its 8-byte prefix, the value, and zero padding to the next multiple of 8. */
/** @internal */
export function field(id: number, wire: number, mandatory: boolean, value: Uint8Array): Uint8Array {
  const out = new Uint8Array(8 + value.length + pad(value.length));
  const view = new DataView(out.buffer);
  view.setUint16(0, id, true);
  out[2] = wire;
  out[3] = mandatory ? 1 : 0;
  view.setUint32(4, value.length, true);
  out.set(value, 8);
  return out;
}

/** @internal */
export function concat(parts: readonly Uint8Array[]): Uint8Array {
  let total = 0;
  for (const part of parts) total += part.length;
  const out = new Uint8Array(total);
  let offset = 0;
  for (const part of parts) {
    out.set(part, offset);
    offset += part.length;
  }
  return out;
}

/** A nested message body: `count:u32, reserved:u32`, then its fields. @internal */
export function messageBody(fields: readonly Uint8Array[]): Uint8Array {
  const header = new Uint8Array(8);
  new DataView(header.buffer).setUint32(0, fields.length, true);
  return concat([header, ...fields]);
}

/** @internal */
export function nested(id: number, mandatory: boolean, fields: readonly Uint8Array[]): Uint8Array {
  return field(id, WIRE_MESSAGE, mandatory, messageBody(fields));
}

/** @internal */
export function u8(value: number): Uint8Array {
  return Uint8Array.of(value);
}

/** @internal */
export function u16(value: number): Uint8Array {
  const out = new Uint8Array(2);
  new DataView(out.buffer).setUint16(0, value, true);
  return out;
}

/** @internal */
export function u32(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setUint32(0, value, true);
  return out;
}

/** @internal */
export function f32(value: number): Uint8Array {
  const out = new Uint8Array(4);
  new DataView(out.buffer).setFloat32(0, value, true);
  return out;
}

/** @internal */
export function text(value: string): Uint8Array {
  return ENCODER.encode(value);
}

/** @internal */
export function requireId(value: unknown, what: string): string {
  if (typeof value !== "string") {
    throw new MisoUsageError(`${what} must be a string`);
  }
  return value;
}

/** @internal */
export function requireU32(value: unknown, what: string): number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > U32_MAXIMUM) {
    throw new MisoUsageError(`${what} must be an integer in [0, ${U32_MAXIMUM}]`);
  }
  return value;
}

/** @internal */
export function requireU64(value: unknown, what: string): bigint {
  if (typeof value !== "bigint" || value < 0n || value > U64_MAXIMUM) {
    throw new MisoUsageError(`${what} must be a bigint in [0, 2^64 - 1]`);
  }
  return value;
}

/** @internal */
export function lookup(table: ReadonlyMap<string, number>, name: unknown, what: string): number {
  const code = typeof name === "string" ? table.get(name) : undefined;
  if (code === undefined) {
    throw new MisoUsageError(`${what} ${String(name)} is not one of ${[...table.keys()].join(", ")}`);
  }
  return code;
}

function addressMessage(address: ControlWireAddress): Uint8Array {
  const { owner, section } = address;
  const fields: Uint8Array[] = [
    field(1, WIRE_U8, true, u8(lookup(OWNER_CODES, owner?.kind, "an owner kind"))),
  ];
  if (owner.kind !== "session") {
    fields.push(field(2, WIRE_UTF8, false, text(requireId(owner.id, "an owner ID"))));
  }
  fields.push(field(3, WIRE_U8, true, u8(lookup(SECTION_CODES, section?.kind, "a section kind"))));
  if (section.kind === "console") {
    fields.push(field(4, WIRE_UTF8, false, text(requireId(section.slot, "a console slot ID"))));
  } else if (section.kind === "inserts") {
    fields.push(field(4, WIRE_UTF8, false, text(requireId(section.effect, "an insert ID"))));
  }
  fields.push(field(5, WIRE_U32, false, u32(requireU32(address.control, "a control row ID"))));
  fields.push(field(6, WIRE_U8, false, u8(lookup(LANE_CODES, address.lane, "a lane"))));
  if (address.expectedEffect !== undefined) {
    if (section.kind !== "console" && section.kind !== "inserts") {
      throw new MisoUsageError("an expected effect names the effect of a console slot or an insert");
    }
    fields.push(field(7, WIRE_UTF8, false, text(requireId(address.expectedEffect, "an expected effect ID"))));
  }
  return messageBody(fields);
}

function editField(edit: ControlFrameEdit): Uint8Array {
  if (typeof edit.value !== "number") {
    throw new MisoUsageError("a control value must be a number");
  }
  const payload: Uint8Array[] = [
    field(1, WIRE_MESSAGE, true, addressMessage(edit.address)),
    field(2, WIRE_F32, true, f32(edit.value)),
  ];
  if (edit.rampSamples !== undefined) {
    payload.push(field(3, WIRE_U32, false, u32(requireU32(edit.rampSamples, "rampSamples"))));
  }
  return nested(1, true, [
    field(1, WIRE_U16, true, u16(EDIT_SET_CONTROL)),
    nested(2, true, payload),
  ]);
}

/**
 * The error a control call that did not return `ok` is, on either transport. The core answers a
 * failed call with the refusal's diagnostic text (`code\tpath\n` lines, for example
 * `web.apply.rebuild_pending`) where a successful call answers with the response frame; a refused
 * transaction is still an `ok` call with a refusal frame. Browser and headless build the same error
 * from the same bytes.
 */
export function controlCallFailure(result: number, response: Uint8Array): MisoEngineError {
  const text = new TextDecoder("utf-8").decode(response);
  return new MisoEngineError("the engine refused a control frame", {
    phase: "lifecycle",
    code: resultName(result, "call"),
    result,
    diagnostics: parseDiagnostics(text),
  });
}

/** The bytes of a `SESSION_TRANSACTION_APPLY` frame before its first edit. */
export const CONTROL_TRANSACTION_HEADER_BYTES = HEADER_BYTES;

/**
 * The bytes one edit adds to a transaction frame. It throws `MisoUsageError` for an edit that cannot
 * be encoded, as `validateControlEdit` does.
 */
export function controlEditBytes(edit: ControlFrameEdit): number {
  return editField(edit).length;
}

/**
 * The address an edit writes: two edits with one key write the same lane of the same control, for
 * the same expected effect. The expected effect is part of the key: an edit written for one effect
 * and an edit written for another (or for none) are different requests, which the core may answer
 * differently, so neither may supersede the other.
 */
export function controlEditKey(edit: ControlFrameEdit): string {
  const { owner, section, control, lane, expectedEffect } = edit.address;
  return JSON.stringify([
    owner.kind,
    "id" in owner ? owner.id : "",
    section.kind,
    section.kind === "console" ? section.slot : section.kind === "inserts" ? section.effect : "",
    control,
    lane,
    expectedEffect ?? null,
  ]);
}

/**
 * Refuse, with `MisoUsageError`, an edit that cannot be encoded, before it is staged anywhere. It is
 * the same check `encodeControlTransaction` makes, and nothing about the value's domain.
 */
export function validateControlEdit(edit: ControlFrameEdit): void {
  editField(edit);
}

/**
 * Encode one `SESSION_TRANSACTION_APPLY` command frame of `SET_CONTROL` edits, in order.
 *
 * Throws `MisoUsageError` before producing any byte when a request cannot be encoded: no edits, a
 * zero or out-of-range request ID or revision, an ID that is not a string, a ramp that is not a
 * `u32`, or an owner, section or lane the protocol has no code for.
 */
export function encodeControlTransaction(request: ControlTransactionRequest): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (requestId === 0n) throw new MisoUsageError("requestId must not be zero");
  const expectedRevision = requireU64(request.expectedRevision, "expectedRevision");
  if (request.edits.length === 0) {
    throw new MisoUsageError("a control transaction needs at least one edit");
  }
  const payload = concat(request.edits.map(editField));
  return commandFrame(
    MESSAGE_SESSION_TRANSACTION_APPLY,
    0,
    requestId,
    expectedRevision,
    request.edits.length,
    payload,
  );
}

/**
 * One command frame: the 48-byte header, then `payload`, which holds `fieldCount` top-level fields.
 * @internal
 */
export function commandFrame(
  message: number,
  flags: number,
  requestId: bigint,
  expectedRevision: bigint,
  fieldCount: number,
  payload: Uint8Array,
): Uint8Array {
  const out = new Uint8Array(HEADER_BYTES + payload.length);
  const view = new DataView(out.buffer);
  out.set(MAGIC, 0);
  view.setUint16(8, PROTOCOL_MAJOR, true);
  view.setUint16(10, PROTOCOL_MINOR, true);
  view.setUint16(12, HEADER_BYTES, true);
  out[14] = KIND_COMMAND;
  out[15] = flags;
  view.setUint16(16, message, true);
  view.setUint16(18, 0, true);
  view.setUint32(20, payload.length, true);
  view.setBigUint64(24, requestId, true);
  view.setBigUint64(32, expectedRevision, true);
  view.setUint32(40, fieldCount, true);
  view.setUint32(44, 0, true);
  out.set(payload, HEADER_BYTES);
  return out;
}

// ----------------------------------------------------------------------------------------------
// Observation frames (#1589b; decision 17, C9 and C13): the `Observe` edit of a
// `SESSION_TRANSACTION_APPLY` (`session::edit::observe`, #1584 D1) and `OBSERVATION_READ` (`0x000f`,
// #1586 D1). `crates/protocol/tests/fixtures/observation-frames.json` holds the Rust codec's bytes
// for a fixed list of both and of the read's replies; `sdk/test/control-frame-evals.mjs` compares
// this codec to it.

const EDIT_OBSERVE = 0x0801;
/** `OBSERVATION_READ` (#1586 D1). @internal */
export const MESSAGE_OBSERVATION_READ = 0x000f;
/** The most selections one `OBSERVATION_READ` carries (#1586 D1). @internal */
export const OBSERVATION_READ_MAXIMUM_SELECTIONS = 256;
/** The widest reply one selection adds to an `OBSERVATION_READ` (#1586 D4: `64 + 192 n`). @internal */
export const OBSERVATION_READ_ROW_BYTES = 192;
/** The fixed bytes of the widest `OBSERVATION_READ` reply (#1586 D4). @internal */
export const OBSERVATION_READ_BASE_BYTES = 64;

/** Where an observed effect lives: a session console slot or one of the strip's inserts. */
export type ObservationFrameRack = "console" | "inserts";

/** One `Observe` edit (#1584 D1): arm or disarm one tap of the effect at a stable address. @internal */
export interface ObserveEdit {
  /** The track or submix, by its ID. */
  readonly stripId: string;
  readonly rack: ObservationFrameRack;
  /** The console slot's or the insert's ID. */
  readonly effectId: string;
  /** The effect-local tap ID. */
  readonly tapId: number;
  /** `true` subscribes, `false` ends one subscription. */
  readonly armed: boolean;
  /** Render blocks per window; `0` is the plan's meter window. A disarm names the window it armed. */
  readonly windowBlocks: number;
}

/** One `OBSERVATION_READ` selection: a tap by its stable address and the lanes to return. @internal */
export interface ObservationFrameSelection {
  readonly stripId: string;
  readonly rack: ObservationFrameRack;
  readonly effectId: string;
  readonly tapId: number;
  readonly channels: ControlLane;
}

const OBSERVATION_CHANNEL_CODES: ReadonlyMap<string, number> = new Map([["left", 1], ["right", 2], ["both", 3]]);

/** The address message of an `Observe` edit or a read selection: a strip, a section and its slot. */
function observeAddress(stripId: unknown, rack: unknown, effectId: unknown): Uint8Array {
  if (rack !== "console" && rack !== "inserts") {
    throw new MisoUsageError(`an observation rack ${String(rack)} is not console or inserts`);
  }
  return messageBody([
    field(1, WIRE_U8, true, u8(lookup(OWNER_CODES, "strip", "an owner kind"))),
    field(2, WIRE_UTF8, false, text(requireId(stripId, "a strip ID"))),
    field(3, WIRE_U8, true, u8(lookup(SECTION_CODES, rack, "a section kind"))),
    field(4, WIRE_UTF8, false, text(requireId(effectId, rack === "console" ? "a console slot ID" : "an insert ID"))),
  ]);
}

function observeEditField(edit: ObserveEdit): Uint8Array {
  if (edit === null || typeof edit !== "object" || typeof edit.armed !== "boolean") {
    throw new MisoUsageError("an Observe edit needs a boolean armed");
  }
  return nested(1, true, [
    field(1, WIRE_U16, true, u16(EDIT_OBSERVE)),
    nested(2, true, [
      field(1, WIRE_MESSAGE, true, observeAddress(edit.stripId, edit.rack, edit.effectId)),
      field(2, WIRE_U32, true, u32(requireU32(edit.tapId, "tapId"))),
      field(3, WIRE_BOOL, true, u8(edit.armed ? 1 : 0)),
      field(4, WIRE_U32, true, u32(requireU32(edit.windowBlocks, "windowBlocks"))),
    ]),
  ]);
}

/**
 * Encode one `SESSION_TRANSACTION_APPLY` command frame of `Observe` edits, in order. It checks only
 * what makes the frame unencodable; the core checks the address, the tap and the window. @internal
 */
export function encodeObserveTransaction(request: {
  readonly requestId: bigint;
  readonly expectedRevision: bigint;
  readonly edits: readonly ObserveEdit[];
}): Uint8Array {
  const requestId = requireU64(request.requestId, "requestId");
  if (requestId === 0n) throw new MisoUsageError("requestId must not be zero");
  const expectedRevision = requireU64(request.expectedRevision, "expectedRevision");
  if (!Array.isArray(request.edits) || request.edits.length === 0) {
    throw new MisoUsageError("an Observe transaction needs at least one edit");
  }
  const payload = concat(request.edits.map(observeEditField));
  return commandFrame(MESSAGE_SESSION_TRANSACTION_APPLY, 0, requestId, expectedRevision, request.edits.length, payload);
}

/** Encode one `OBSERVATION_READ` command frame (any revision) of 1 to 256 selections. @internal */
export function encodeObservationRead(requestId: bigint, selections: readonly ObservationFrameSelection[]): Uint8Array {
  const id = requireU64(requestId, "requestId");
  if (id === 0n) throw new MisoUsageError("requestId must not be zero");
  if (!Array.isArray(selections) || selections.length === 0
      || selections.length > OBSERVATION_READ_MAXIMUM_SELECTIONS) {
    throw new MisoUsageError(`an observation read holds 1 to ${OBSERVATION_READ_MAXIMUM_SELECTIONS} selections`);
  }
  const payload = concat(selections.map((selection) => nested(1, true, [
    field(1, WIRE_MESSAGE, true, observeAddress(selection.stripId, selection.rack, selection.effectId)),
    field(2, WIRE_U32, true, u32(requireU32(selection.tapId, "tapId"))),
    field(3, WIRE_U8, true, u8(lookup(OBSERVATION_CHANNEL_CODES, selection.channels, "observation channels"))),
  ])));
  return commandFrame(MESSAGE_OBSERVATION_READ, FLAG_REVISION_ANY, id, 0n, selections.length, payload);
}

/** The status of one `OBSERVATION_READ` row (#1586 D2, under #1585's status contract). */
export type ObservationFrameStatus = "pending" | "unarmed" | "ready";

/** The window of a ready row. @internal */
export interface ObservationFrameWindow {
  readonly firstSample: bigint;
  readonly endSample: bigint;
  readonly sequence: bigint;
  readonly blocks: number;
}

/**
 * One row of an `OBSERVATION_READ` reply. `armed`, `armSample` and `armTag` are the arm state render
 * applied (`false`/`0`/`0` while render has not applied the tap's last record). @internal
 */
export interface ObservationFrameRow {
  readonly status: ObservationFrameStatus;
  readonly armed: boolean;
  readonly armSample: bigint;
  readonly armTag: bigint;
  /** Present exactly when `status` is `ready`. */
  readonly window: ObservationFrameWindow | undefined;
  /** Present exactly when ready and asked for. */
  readonly left: number | undefined;
  readonly right: number | undefined;
  /** The tap's native `ParameterUnit` code. */
  readonly unit: number;
}

/** The success payload of an `OBSERVATION_READ`: one row per selection, in selection order. @internal */
export interface ObservationReadPage {
  /** The render sample at the read: the end of the last block render completed. */
  readonly observedSample: bigint;
  readonly rows: readonly ObservationFrameRow[];
}

/** A decoded `OBSERVATION_READ` reply. @internal */
export type ObservationReadReply =
  | { readonly ok: true; readonly requestId: bigint; readonly revision: bigint; readonly value: ObservationReadPage }
  | {
    readonly ok: false;
    readonly requestId: bigint;
    readonly revision: bigint;
    readonly status: Exclude<ControlStatusName, "ok">;
    readonly diagnostics: readonly ProtocolDiagnostic[];
    readonly omittedDiagnostics: number;
    readonly backpressure: ControlBackpressure | undefined;
  };

/** The highest `ParameterUnit` code (`ratio`) and the last field of an `ObservationRow` (#1586 D1). */
const OBSERVATION_UNIT_MAXIMUM = 6;
const OBSERVATION_ROW_FIELDS = 11;

const ROW_STATUS: readonly (ObservationFrameStatus | undefined)[] = [undefined, "pending", "unarmed", "ready"];

function u64Of(fields: readonly RawField[], id: number): bigint | undefined {
  return scalar(fields, id, WIRE_U64, 8)?.getBigUint64(0, true);
}

function decodeObservationRow(row: RawField): ObservationFrameRow {
  if (row.wire !== WIRE_MESSAGE) throw malformed("an observation row is not a message");
  const fields = nestedFields(row.value);
  const status = ROW_STATUS[scalar(fields, 1, WIRE_U8, 1)?.getUint8(0) ?? 0];
  const armedByte = scalar(fields, 2, WIRE_BOOL, 1)?.getUint8(0);
  const armSample = u64Of(fields, 3);
  const armTag = u64Of(fields, 4);
  const unit = scalar(fields, 11, WIRE_U8, 1)?.getUint8(0);
  if (status === undefined || (armedByte !== 0 && armedByte !== 1) || armSample === undefined
      || armTag === undefined || unit === undefined || unit === 0 || unit > OBSERVATION_UNIT_MAXIMUM) {
    throw malformed("an observation row is incomplete or names an unknown unit");
  }
  if (fields.some((row) => row.id > OBSERVATION_ROW_FIELDS)) {
    throw malformed("an observation row holds an unknown field");
  }
  const armed = armedByte === 1;
  const first = u64Of(fields, 5);
  const end = u64Of(fields, 6);
  const sequence = u64Of(fields, 7);
  const blocks = scalar(fields, 8, WIRE_U32, 4)?.getUint32(0, true);
  const left = scalar(fields, 9, WIRE_F32, 4)?.getFloat32(0, true);
  const right = scalar(fields, 10, WIRE_F32, 4)?.getFloat32(0, true);
  const anyWindow = first !== undefined || end !== undefined || sequence !== undefined || blocks !== undefined;
  let window: ObservationFrameWindow | undefined;
  if (status === "ready") {
    // The core's own presence rule (#1586 D1): a ready row is armed, tagged, and carries a
    // non-empty window that opened at or after the arm, and at least one lane.
    if (first === undefined || end === undefined || sequence === undefined || blocks === undefined
        || !armed || armTag === 0n || first < armSample || end <= first || blocks === 0
        || (left === undefined && right === undefined)) {
      throw malformed("a ready observation row breaks the presence rule");
    }
    window = Object.freeze({ firstSample: first, endSample: end, sequence, blocks });
  } else if (anyWindow || left !== undefined || right !== undefined) {
    throw malformed("an observation row that is not ready carries a window or a lane");
  }
  return Object.freeze({ status, armed, armSample, armTag, window, left, right, unit });
}

/**
 * Decode the reply to an `OBSERVATION_READ`. A refusal is a value; bytes that are not a valid reply
 * raise `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`). @internal
 */
export function decodeObservationReadReply(bytes: Uint8Array): ObservationReadReply {
  const { requestId, status, revision, fields } = decodeResponseEnvelope(bytes, MESSAGE_OBSERVATION_READ);
  if (status !== "ok") {
    const refusal = decodeRefusalFields(fields);
    return Object.freeze({ ok: false, requestId, revision, status, ...refusal });
  }
  const observedSample = u64Of(fields, 1);
  if (observedSample === undefined) throw malformed("an observation page has no observed sample");
  const rows = fields.filter((row) => row.id === 2).map(decodeObservationRow);
  if (rows.length === 0 || rows.length > OBSERVATION_READ_MAXIMUM_SELECTIONS
      || fields.some((row) => row.id !== 1 && row.id !== 2)) {
    throw malformed("an observation page holds no rows, too many, or an unknown field");
  }
  return Object.freeze({
    ok: true,
    requestId,
    revision,
    value: Object.freeze({ observedSample, rows: Object.freeze(rows) }),
  });
}

/**
 * Encode a `CAPABILITIES_GET` command: a header and no fields, which takes any revision. The reply
 * to it reports the core's committed revision, which is how an endpoint that cannot read the
 * revision directly (the browser host) seeds its queue after boot. The caller draws `requestId`
 * from the endpoint's one allocator.
 */
export function encodeRevisionProbe(requestId: bigint): Uint8Array {
  const id = requireU64(requestId, "requestId");
  if (id === 0n) throw new MisoUsageError("requestId must not be zero");
  return commandFrame(MESSAGE_CAPABILITIES_GET, FLAG_REVISION_ANY, id, 0n, 0, new Uint8Array(0));
}

/** The limits a `CAPABILITIES_GET` reply reports that bound what one transaction may hold. */
export interface ControlLimits {
  /** The most bytes one command frame may hold, header included. */
  readonly maximumFrameBytes: number;
  /** The most edits one transaction may hold. */
  readonly maximumTransactionEdits: number;
}

/**
 * Read the frame limits out of the reply to `encodeRevisionProbe`. A reply that is not a
 * `CAPABILITIES_GET` response at all (a scripted host that answers a probe with a blank frame)
 * reports `undefined`, which leaves a queue unbounded by bytes. A response that is one but does not
 * hold the limits raises `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeControlLimits(bytes: Uint8Array): ControlLimits | undefined {
  if (bytes.length < HEADER_BYTES) return undefined;
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  for (let index = 0; index < MAGIC.length; index += 1) {
    if (bytes[index] !== MAGIC[index]) return undefined;
  }
  if (bytes[14] !== KIND_RESPONSE || view.getUint16(16, true) !== MESSAGE_CAPABILITIES_GET) {
    return undefined;
  }
  if (view.getUint16(18, true) !== 0) throw malformed("the capabilities reply is not ok");
  if (HEADER_BYTES + view.getUint32(20, true) !== bytes.length) throw malformed("the payload length is wrong");
  const fields = readFields(bytes.subarray(HEADER_BYTES), view.getUint32(40, true));
  const frame = scalar(fields, 5, WIRE_U64, 8)?.getBigUint64(0, true);
  const edits = scalar(fields, 24, WIRE_U32, 4)?.getUint32(0, true);
  if (frame === undefined || edits === undefined || frame === 0n || edits === 0) {
    throw malformed("the capabilities reply holds no frame or transaction limit");
  }
  const cap = BigInt(Number.MAX_SAFE_INTEGER);
  return Object.freeze({
    maximumFrameBytes: Number(frame > cap ? cap : frame),
    maximumTransactionEdits: edits,
  });
}

/**
 * How the core applied a committed transaction (`TransactionApplied.edit_path`). `"unchanged"` is
 * only a `SESSION_REPLACE` reply's: the document equals the committed model, nothing committed and
 * the revision did not advance (#1386).
 */
export type ControlEditPath = "live" | "model_only" | "rebuild" | "monitor" | "unchanged";

const EDIT_PATHS: ReadonlyMap<number, ControlEditPath> = new Map([
  [1, "live"],
  [2, "model_only"],
  [3, "rebuild"],
  // The monitor overlay (solo) lands with #1559; its code is accepted here so the decoder needs no
  // change when the core starts to send it.
  [4, "monitor"],
  [5, "unchanged"],
]);

/** The protocol's response status registry, by name. */
export type ControlStatusName =
  | "ok"
  | "malformed_frame"
  | "unsupported_version"
  | "unsupported_message"
  | "unknown_required_field"
  | "invalid_field"
  | "limit_exceeded"
  | "revision_conflict"
  | "revision_exhausted"
  | "request_id_reuse"
  | "replay_expired"
  | "backpressure"
  | "validation_failed"
  | "not_found"
  | "unavailable"
  | "pcm_forbidden"
  | "internal";

/**
 * The status names, indexed by the wire number. Numbers 15 and 16 were `TIME_IN_PAST` and
 * `AUTOMATION_ORDER`, retired with `AUTOMATION_ENQUEUE` and never reallocated: a frame that
 * carries one is "not registered", like any other unallocated number.
 */
const STATUS_NAMES: readonly (ControlStatusName | undefined)[] = [
  "ok",
  "malformed_frame",
  "unsupported_version",
  "unsupported_message",
  "unknown_required_field",
  "invalid_field",
  "limit_exceeded",
  "revision_conflict",
  "revision_exhausted",
  "request_id_reuse",
  "replay_expired",
  "backpressure",
  "validation_failed",
  "not_found",
  "unavailable",
  undefined,
  undefined,
  "pcm_forbidden",
  "internal",
];

const SEVERITY_NAMES = ["", "info", "warning", "error"] as const;

/** One diagnostic of a refused transaction. */
export interface ControlDiagnostic {
  /** The core's stable dotted code, such as `control.domain` or `control.address.unknown`. */
  readonly code: string;
  readonly severity: "info" | "warning" | "error";
  /** The index of the edit the diagnostic names, when it names one. */
  readonly operationIndex: number | undefined;
  readonly detail: string | undefined;
}

/** The queue detail a `backpressure` reply carries. */
export interface ControlBackpressure {
  readonly queueKind: number;
  readonly capacity: bigint;
  readonly occupancy: bigint;
}

/** What a successful `SESSION_TRANSACTION_APPLY` reply reports (`TransactionApplied`). */
export interface ControlApplied {
  readonly appliedOperations: number;
  readonly editPath: ControlEditPath;
  /**
   * The arm tag of each `Observe` edit, in transaction order (#1585): the tag of the tap's
   * subscription after that edit. Empty when the transaction held no `Observe` edit. An `Observe`
   * is in effect when a read reports its tap armed with this tag, not when the reply returns. A tag
   * of `0` means no record is published for that edit: the tap was never subscribed, or the same
   * transaction removed its target.
   */
  readonly observeArmTags: readonly bigint[];
  /** The tags of the subscriptions the commit ended, in key order (#1585); empty when none. */
  readonly removedArmTags: readonly bigint[];
}

/** A decoded reply to a `SESSION_TRANSACTION_APPLY` command. */
export interface ControlReply {
  readonly requestId: bigint;
  readonly status: ControlStatusName;
  /** The session revision the reply reports: committed on success, observed on a refusal. */
  readonly revision: bigint;
  /** Present exactly when `status` is `ok`. */
  readonly applied: ControlApplied | undefined;
  /** The leading diagnostics of a refusal, in the core's order. */
  readonly diagnostics: readonly ControlDiagnostic[];
  readonly omittedDiagnostics: number;
  readonly backpressure: ControlBackpressure | undefined;
}

/** @internal The error every decoder of a core reply raises for bytes that are not a valid reply. */
export function malformed(why: string): MisoEngineError {
  return new MisoEngineError(`the control reply is malformed: ${why}`, {
    phase: "lifecycle",
    code: "internal",
  });
}

/** @internal One decoded field of a message level. */
export interface RawField {
  readonly id: number;
  readonly wire: number;
  readonly value: Uint8Array;
}

/** @internal Read `count` fields of one message level: ascending IDs, zero padding, nothing left over. */
export function readFields(bytes: Uint8Array, count: number): RawField[] {
  const fields: RawField[] = [];
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  let cursor = 0;
  let previous = 0;
  for (let index = 0; index < count; index += 1) {
    if (cursor + 8 > bytes.length) throw malformed("a field ends the frame early");
    const id = view.getUint16(cursor, true);
    const wire = bytes[cursor + 2] ?? 0;
    const flags = bytes[cursor + 3] ?? 0;
    const length = view.getUint32(cursor + 4, true);
    if (id === 0 || wire < 1 || wire > 15 || (flags & ~1) !== 0 || (index !== 0 && id < previous)) {
      throw malformed("a field header is invalid");
    }
    const start = cursor + 8;
    const end = start + length;
    const padded = end + pad(length);
    if (padded > bytes.length) throw malformed("a field is longer than its message");
    for (let at = end; at < padded; at += 1) {
      if (bytes[at] !== 0) throw malformed("field padding is not zero");
    }
    fields.push({ id, wire, value: bytes.subarray(start, end) });
    previous = id;
    cursor = padded;
  }
  if (cursor !== bytes.length) throw malformed("bytes follow the last field");
  return fields;
}

/** @internal The fields of a nested message. */
export function nestedFields(value: Uint8Array): RawField[] {
  if (value.length < 8) throw malformed("a nested message has no header");
  const view = new DataView(value.buffer, value.byteOffset, value.byteLength);
  if (view.getUint32(4, true) !== 0) throw malformed("a nested message reserved word is not zero");
  return readFields(value.subarray(8), view.getUint32(0, true));
}

/** @internal The one fixed-width field `id` of a message, or `undefined`; a repeat or a wrong type is malformed. */
export function scalar(fields: readonly RawField[], id: number, wire: number, width: number): DataView | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return undefined;
  if (row.wire !== wire || row.value.length !== width) throw malformed(`field ${id} has the wrong type`);
  return new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
}

/** A reply's optional arm-tag list (#1585 D5): absent is empty, and a present list is not. */
function armTags(fields: readonly RawField[], id: number): readonly bigint[] {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return Object.freeze([]);
  if (row.wire !== WIRE_PACKED_U64 || row.value.length === 0 || row.value.length % 8 !== 0) {
    throw malformed(`field ${id} has the wrong type`);
  }
  const view = new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
  const tags: bigint[] = [];
  for (let at = 0; at < row.value.length; at += 8) tags.push(view.getBigUint64(at, true));
  return Object.freeze(tags);
}

function textOf(row: RawField): string {
  if (row.wire !== WIRE_UTF8) throw malformed(`field ${row.id} is not text`);
  try {
    return DECODER.decode(row.value);
  } catch {
    throw malformed(`field ${row.id} is not UTF-8`);
  }
}

function oneText(fields: readonly RawField[], id: number): string | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  return found[0] === undefined ? undefined : textOf(found[0]);
}

function decodePathSegment(row: RawField): DiagnosticPathSegment {
  if (row.wire !== WIRE_MESSAGE) throw malformed("a diagnostic path segment is not a message");
  const fields = nestedFields(row.value);
  const tag = scalar(fields, 1, WIRE_U8, 1)?.getUint8(0);
  const name = oneText(fields, 2);
  const index = scalar(fields, 3, WIRE_U64, 8);
  const stable = oneText(fields, 4);
  if (tag === 1 && name !== undefined && index === undefined && stable === undefined) {
    return Object.freeze({ kind: "field", field: name });
  }
  if (tag === 2 && name === undefined && index !== undefined && stable === undefined) {
    return Object.freeze({ kind: "index", index: index.getBigUint64(0, true) });
  }
  if (tag === 3 && name === undefined && index === undefined && stable !== undefined) {
    return Object.freeze({ kind: "stableId", stableId: stable });
  }
  throw malformed("a diagnostic path segment is invalid");
}

/**
 * The SDK's one decoder of the protocol's `Diagnostic` message, every field it carries; an absent
 * optional field is an absent key. A refusal of any command and the diagnostics replies and events
 * all read it here. @internal
 */
export function decodeProtocolDiagnostic(row: RawField): ProtocolDiagnostic {
  if (row.wire !== WIRE_MESSAGE) throw malformed("a diagnostic is not a message");
  const fields = nestedFields(row.value);
  const code = oneText(fields, 1);
  if (code === undefined) throw malformed("a diagnostic has no code");
  const severity = scalar(fields, 2, WIRE_U8, 1)?.getUint8(0);
  const named = severity === undefined ? undefined : SEVERITY_NAMES[severity];
  if (named === undefined || named === "") throw malformed("a diagnostic has no severity");
  const detail = oneText(fields, 4);
  const operationIndex = scalar(fields, 5, WIRE_U32, 4);
  const sampleTime = scalar(fields, 6, WIRE_U64, 8);
  const providerSequence = scalar(fields, 7, WIRE_U64, 8);
  return Object.freeze({
    code,
    severity: named,
    path: Object.freeze(fields.filter((candidate) => candidate.id === 3).map(decodePathSegment)),
    ...(detail === undefined ? {} : { detail }),
    ...(operationIndex === undefined ? {} : { operationIndex: operationIndex.getUint32(0, true) }),
    ...(sampleTime === undefined ? {} : { sampleTime: sampleTime.getBigUint64(0, true) }),
    ...(providerSequence === undefined ? {} : { providerSequence: providerSequence.getBigUint64(0, true) }),
  });
}

/** A response frame's header and its top-level fields, before the message-specific reading. @internal */
export interface ResponseEnvelope {
  readonly requestId: bigint;
  readonly status: ControlStatusName;
  /** The session revision the header reports. */
  readonly revision: bigint;
  readonly fields: readonly RawField[];
}

/**
 * Read a response frame to command `message`: its header checks and its top-level fields.
 * Bytes that are not a valid response to that command raise `MisoEngineError`
 * (`phase: "lifecycle"`, `code: "internal"`).
 * @internal
 */
export function decodeResponseEnvelope(bytes: Uint8Array, message: number): ResponseEnvelope {
  if (bytes.length < HEADER_BYTES) throw malformed("the frame is shorter than its header");
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  for (let index = 0; index < MAGIC.length; index += 1) {
    if (bytes[index] !== MAGIC[index]) throw malformed("the magic is wrong");
  }
  if (view.getUint16(8, true) !== PROTOCOL_MAJOR) throw malformed("the protocol major version is not 1");
  if (view.getUint16(12, true) !== HEADER_BYTES) throw malformed("the header length is not 48");
  if (bytes[14] !== KIND_RESPONSE || bytes[15] !== 0) throw malformed("the frame is not a response");
  if (view.getUint16(16, true) !== message) {
    throw malformed("the response is for another command");
  }
  const status = STATUS_NAMES[view.getUint16(18, true)];
  if (status === undefined) throw malformed("the status is not registered");
  const payloadLength = view.getUint32(20, true);
  if (HEADER_BYTES + payloadLength !== bytes.length) throw malformed("the payload length is wrong");
  const requestId = view.getBigUint64(24, true);
  if (requestId === 0n) throw malformed("the request ID is zero");
  if (view.getUint32(44, true) !== 0) throw malformed("the reserved word is not zero");
  const fields = readFields(bytes.subarray(HEADER_BYTES), view.getUint32(40, true));
  return { requestId, status, revision: view.getBigUint64(32, true), fields };
}

/** The common body of a refusal: the leading diagnostics, the omitted count and any backpressure. @internal */
export interface ControlRefusalBody {
  readonly diagnostics: readonly ControlDiagnostic[];
  readonly omittedDiagnostics: number;
  readonly backpressure: ControlBackpressure | undefined;
}

/** The non-`ok` body every command shares, with every diagnostic field. @internal */
export interface ProtocolRefusalFields {
  readonly diagnostics: readonly ProtocolDiagnostic[];
  readonly omittedDiagnostics: number;
  readonly backpressure: ControlBackpressure | undefined;
}

/** Read the non-`ok` body every command shares, each diagnostic decoded once. @internal */
export function decodeRefusalFields(fields: readonly RawField[]): ProtocolRefusalFields {
  const diagnostics = fields.filter((row) => row.id === 1).map(decodeProtocolDiagnostic);
  const omittedDiagnostics = scalar(fields, 2, WIRE_U32, 4)?.getUint32(0, true);
  if (omittedDiagnostics === undefined) throw malformed("a refusal has no omitted-diagnostics count");
  const backpressureField = fields.find((row) => row.id === 3);
  let backpressure: ControlBackpressure | undefined;
  if (backpressureField !== undefined) {
    if (backpressureField.wire !== WIRE_MESSAGE) throw malformed("backpressure is not a message");
    const inner = nestedFields(backpressureField.value);
    const queueKind = scalar(inner, 1, WIRE_U8, 1)?.getUint8(0);
    const capacity = scalar(inner, 2, WIRE_U64, 8)?.getBigUint64(0, true);
    const occupancy = scalar(inner, 3, WIRE_U64, 8)?.getBigUint64(0, true);
    if (queueKind === undefined || capacity === undefined || occupancy === undefined) {
      throw malformed("backpressure is incomplete");
    }
    backpressure = Object.freeze({ queueKind, capacity, occupancy });
  }
  return { diagnostics: Object.freeze(diagnostics), omittedDiagnostics, backpressure };
}

/** Read the non-`ok` body every command shares, as a control refusal reports it. @internal */
export function decodeRefusalBody(fields: readonly RawField[]): ControlRefusalBody {
  const { diagnostics, omittedDiagnostics, backpressure } = decodeRefusalFields(fields);
  return {
    diagnostics: Object.freeze(diagnostics.map((row) => Object.freeze({
      code: row.code,
      severity: row.severity,
      operationIndex: row.operationIndex,
      detail: row.detail,
    }))),
    omittedDiagnostics,
    backpressure,
  };
}

/**
 * Decode the reply to a `SESSION_TRANSACTION_APPLY` command.
 *
 * A reply the core sent is always decodable, refusals included: a refusal is a value (its `status`,
 * `diagnostics` and the `revision` the core observed), not an exception. Bytes that are not a valid
 * reply to this command raise `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeControlReply(bytes: Uint8Array): ControlReply {
  return decodeAppliedReply(bytes, MESSAGE_SESSION_TRANSACTION_APPLY);
}

/**
 * Decode the reply to `message`, a command whose success payload is `TransactionApplied`:
 * `SESSION_TRANSACTION_APPLY` or `SESSION_REPLACE` (#1386).
 * @internal
 */
export function decodeAppliedReply(bytes: Uint8Array, message: number): ControlReply {
  const { requestId, status, revision, fields } = decodeResponseEnvelope(bytes, message);

  if (status === "ok") {
    const appliedOperations = scalar(fields, 1, WIRE_U32, 4)?.getUint32(0, true);
    const code = scalar(fields, 2, WIRE_U8, 1)?.getUint8(0);
    const editPath = code === undefined ? undefined : EDIT_PATHS.get(code);
    if (appliedOperations === undefined || editPath === undefined) {
      throw malformed("a successful reply holds no applied-operations count and known edit path");
    }
    const observeArmTags = armTags(fields, 3);
    const removedArmTags = armTags(fields, 4);
    return Object.freeze({
      requestId,
      status,
      revision,
      applied: Object.freeze({ appliedOperations, editPath, observeArmTags, removedArmTags }),
      diagnostics: Object.freeze([]),
      omittedDiagnostics: 0,
      backpressure: undefined,
    });
  }

  const { diagnostics, omittedDiagnostics, backpressure } = decodeRefusalBody(fields);
  return Object.freeze({
    requestId,
    status,
    revision,
    applied: undefined,
    diagnostics,
    omittedDiagnostics,
    backpressure,
  });
}
