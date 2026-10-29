import { CONTROLS } from "../generated/controls.ts";
import type { ControlLane, ControlOwner, ControlSection } from "../generated/controls.ts";
import { MisoControlRefusal, MisoEngineError, MisoUsageError } from "./errors.ts";
import {
  DECODER,
  FLAG_REVISION_ANY,
  OWNER_CODES,
  WIRE_BOOL,
  WIRE_BYTES,
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
  lookup,
  malformed,
  nestedFields,
  requireId,
  requireU32,
  scalar,
  text,
  u8,
  u32,
} from "./control-frame.ts";
import type { ControlDiagnostic, ControlWireAddress, RawField } from "./control-frame.ts";
import type { ControlRequestIds } from "./control-queue.ts";

/**
 * The SDK's one reader of the core's control list (decision 17, C4; #1564 D1-D2).
 *
 * `CONTROL_LIST_GET` (`0x000c`) and `CONTROL_STATE_GET` (`0x000d`) page through the rows of the
 * running session. The core decides everything: which rows exist, their order, their descriptors, the
 * value and whether a structure is still current. This file builds the request, reads the pages and
 * follows `last_handle` until `eof`; it validates nothing the core validates. The field numbers are
 * the protocol schema's (`crates/protocol/src/schema.rs`: `control_list_request`, `control_entry`,
 * `control_list_page`, `control_state_page`, `control_address`), and the code tables are
 * `crates/protocol/src/message_wire.rs`'s `control_code_table!` blocks.
 *
 * # Structure and handles
 *
 * A page names `structure_epoch`, the structure its handles belong to. A request that names another
 * epoch is refused `control.list.stale`; `readControlList` then starts over from the first page, up
 * to {@link CONTROL_LIST_RESTARTS} times, so a list never mixes the handles of two structures.
 */

const MESSAGE_CONTROL_LIST_GET = 0x000c;
const MESSAGE_CONTROL_STATE_GET = 0x000d;

/** The most rows one page holds (`CONTROL_PAGE_ITEMS`). */
export const CONTROL_LIST_PAGE_ROWS = 256;
/** How many times a list read starts over after a `control.list.stale` page before it gives up. */
export const CONTROL_LIST_RESTARTS = 3;
/** The core's code for a request that names a structure that is no longer current. */
export const CONTROL_LIST_STALE = "control.list.stale";

const SECTION_NAMES = new Map<number, ControlSection["kind"]>(
  CONTROLS.address.sectionKinds.map((row) => [row.code, row.name] as const),
);
const OWNER_NAMES = new Map<number, ControlOwner["kind"]>(
  CONTROLS.address.ownerKinds.map((row) => [row.code, row.name] as const),
);
const LANE_NAMES = new Map<number, ControlLane>(
  CONTROLS.address.lanes.map((row) => [row.code, row.name] as const),
);

/** What an owner header row describes. Track and submix are both strips. */
export type ControlOwnerHeaderKind = "track" | "submix" | "route" | "vca" | "session";
/** What kind of value a control holds. */
export type ControlValueKind = "continuous" | "toggle" | "enumeration";
/** How a frontend maps a control's range to a knob. */
export type ControlTaperName = "linear" | "logarithmic" | "exponential" | "decibel" | "stepped";
/** The unit of a control's value. */
export type ControlUnitName = "db" | "hz" | "milliseconds" | "samples" | "linear" | "ratio";
/** The ramp row a live control's default ramp length comes from. */
export type ControlRampRowName =
  | "fader"
  | "inputTrim"
  | "sendGain"
  | "vcaOffset"
  | "linkGlide"
  | "mute"
  | "solo"
  | "polarityInvert"
  | "sendMute"
  | "sendFollowsMute"
  | "vcaMute"
  | "bypassCrossfade"
  | "pan"
  | "matrix"
  | "sendMatrix";
/** How a change to a control reaches a running plan. */
export type ControlClassName = "live" | "rebuildOnly" | "derived";
/** Why a control is rebuild-only. */
export type ControlReasonName =
  | "newMemory"
  | "newGraph"
  | "processingOrder"
  | "latency"
  | "glitch"
  | "optimisation"
  | "correctness"
  | "ruledLiveNotBuilt";
/** Where a control's value lives. */
export type ControlScopeName = "model" | "monitor";
/** The render family that carries a live change. */
export type ControlFamilyName =
  | "stripLevel"
  | "stripMatrix"
  | "stripInput"
  | "route"
  | "effect"
  | "setting";
/** The send tap a route reads. */
export type ControlTapName =
  | "input"
  | "post_input"
  | "insert_send"
  | "insert_return"
  | "pre_fader"
  | "post_fader"
  | "post_pan";

const OWNER_HEADER_KINDS = names<ControlOwnerHeaderKind>(["track", "submix", "route", "vca", "session"]);
const VALUE_KINDS = names<ControlValueKind>(["continuous", "toggle", "enumeration"]);
const TAPERS = names<ControlTaperName>(["linear", "logarithmic", "exponential", "decibel", "stepped"]);
const UNITS = names<ControlUnitName>(["db", "hz", "milliseconds", "samples", "linear", "ratio"]);
const RAMP_ROWS = names<ControlRampRowName>([
  "fader",
  "inputTrim",
  "sendGain",
  "vcaOffset",
  "linkGlide",
  "mute",
  "solo",
  "polarityInvert",
  "sendMute",
  "sendFollowsMute",
  "vcaMute",
  "bypassCrossfade",
  "pan",
  "matrix",
  "sendMatrix",
]);
const CLASSES = names<ControlClassName>(["live", "rebuildOnly", "derived"]);
const REASONS = names<ControlReasonName>([
  "newMemory",
  "newGraph",
  "processingOrder",
  "latency",
  "glitch",
  "optimisation",
  "correctness",
  "ruledLiveNotBuilt",
]);
const SCOPES = names<ControlScopeName>(["model", "monitor"]);
const FAMILIES = names<ControlFamilyName>([
  "stripLevel",
  "stripMatrix",
  "stripInput",
  "route",
  "effect",
  "setting",
]);
const TAPS = names<ControlTapName>([
  "input",
  "post_input",
  "insert_send",
  "insert_return",
  "pre_fader",
  "post_fader",
  "post_pan",
]);

/** A code table whose codes are `1..` in the order of `rows`. */
function names<T extends string>(rows: readonly T[]): ReadonlyMap<number, T> {
  return new Map(rows.map((name, index) => [index + 1, name] as const));
}

/** Which rows a list read asks for. */
export interface ControlListFilter {
  /** Only the rows of this owner. A strip, route or VCA is named by its stable ID. */
  readonly owner?: ControlOwner;
  /**
   * The rows asked for per request, `1..=256` (default 256). The core cuts a page smaller when its
   * reply would not fit, so this bounds a page and never the list.
   */
  readonly pageLimit?: number;
}

/** The address of a header row: an owner, or one section of an owner. */
export interface ControlHeaderAddress {
  readonly owner: ControlOwner;
  /** `own` for an owner header. */
  readonly section: ControlSection;
}

/** The ends of a route owner. */
export interface ControlRouteEnds {
  readonly source: string;
  readonly destination: string;
  readonly tap: ControlTapName;
}

/** A header row that opens an owner (a strip, route, VCA or the session). */
export interface ControlOwnerHeader {
  readonly row: "owner";
  /** The row's handle in the list's structure. */
  readonly handle: number;
  readonly address: ControlHeaderAddress;
  /** What the owner is: a track or a submix strip, a route, a VCA or the session. */
  readonly kind: ControlOwnerHeaderKind;
  readonly name: string;
  /** Present exactly for a route. */
  readonly route: ControlRouteEnds | undefined;
}

/** A header row that opens a section of the owner above it. */
export interface ControlSectionHeader {
  readonly row: "section";
  readonly handle: number;
  readonly address: ControlHeaderAddress;
  readonly name: string;
  /** The effect identity of a console slot or an insert. */
  readonly effect: string | undefined;
}

export type ControlHeader = ControlOwnerHeader | ControlSectionHeader;

/** One choice of an enumeration control. */
export interface ControlChoice {
  readonly value: number;
  readonly label: string;
}

/**
 * One control row of the list: its address, the descriptor the core resolved at the session's
 * sample rate, and the committed value. The names mirror the wire fields of `control_entry`.
 */
export interface ControlListEntry {
  readonly handle: number;
  /** The address `edit.set` takes: stable IDs and the row's numeric `control`. */
  readonly address: ControlWireAddress;
  readonly name: string;
  readonly displayName: string;
  readonly displayUnit: string;
  readonly unit: ControlUnitName;
  readonly kind: ControlValueKind;
  readonly minimum: number;
  readonly maximum: number;
  /** A value outside the range that also is accepted; `undefined` when there is none. */
  readonly disabledValue: number | undefined;
  readonly default: number;
  readonly taper: ControlTaperName;
  readonly step: number;
  readonly precision: number;
  readonly ladder: readonly number[];
  readonly choices: readonly ControlChoice[];
  /** The ramp row the default ramp comes from; `undefined` for a fixed ramp. */
  readonly rampRow: ControlRampRowName | undefined;
  readonly defaultRampSamples: number;
  readonly class: ControlClassName;
  /** Why the control is rebuild-only; present exactly when `class` is `rebuildOnly`. */
  readonly reason: ControlReasonName | undefined;
  readonly reasonIssue: number | undefined;
  readonly reopen: string | undefined;
  readonly scope: ControlScopeName;
  readonly automatable: boolean;
  readonly family: ControlFamilyName;
  /** `false` for a row the owner's present shape does not use. */
  readonly active: boolean;
  /**
   * The committed value when its page was read. On an automated lane (`automated`) of a strip's
   * `fader_db` row it is the lane's stored hand-move offset in dB: a `SET_CONTROL` of the row
   * writes the offset, and the stored curve renders under it (#1650).
   */
  readonly value: number;
  /** Whether a rendered stored automation drives the row's lane (#1650 D4). */
  readonly automated: boolean;
}

/** The control list of the running session, in the core's order. */
export interface ControlList {
  /** The session revision the first page was read at. */
  readonly revision: bigint;
  /** The structure the handles of this list belong to. */
  readonly structureEpoch: bigint;
  /** The owner and section header rows, in list order. */
  readonly headers: readonly ControlHeader[];
  /** The control rows, in list order. */
  readonly controls: readonly ControlListEntry[];
  /** The owner the list was restricted to, if any; `controlState` reads the same rows. */
  readonly owner: ControlOwner | undefined;
}

/** The committed values of a list's control rows. */
export interface ControlState {
  /** The session revision the first page was read at. */
  readonly revision: bigint;
  readonly structureEpoch: bigint;
  /** The value of each control row, by the row's handle. */
  readonly values: ReadonlyMap<number, number>;
}

/** One page of the list. */
export interface ControlListPage {
  readonly revision: bigint;
  readonly structureEpoch: bigint;
  readonly lastHandle: number;
  readonly eof: boolean;
  readonly headers: readonly ControlHeader[];
  readonly controls: readonly ControlListEntry[];
}

/** One page of values. */
export interface ControlStatePage {
  readonly revision: bigint;
  readonly structureEpoch: bigint;
  readonly lastHandle: number;
  readonly eof: boolean;
  readonly records: readonly { readonly handle: number; readonly value: number }[];
}

/** A page request the core refused. */
export interface ControlPageRefusal {
  readonly status: string;
  readonly revision: bigint;
  readonly diagnostics: readonly ControlDiagnostic[];
}

export type ControlPageResult<P> =
  | { readonly ok: true; readonly page: P }
  | { readonly ok: false; readonly refusal: ControlPageRefusal };

/** What a list read asks for: one page of `kind`. */
export interface ControlListRequest {
  readonly kind: "list" | "state";
  /** A nonzero `u64` the endpoint correlates the reply by and refuses to see twice. */
  readonly requestId: bigint;
  /** The structure the cursor belongs to; `0n` for the first page. */
  readonly structureEpoch: bigint;
  /** The last handle the caller has; `0` starts the list. */
  readonly afterHandle: number;
  /** The most rows to return. */
  readonly limit: number;
  readonly owner?: ControlOwner | undefined;
}

function u64Bytes(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function u16Bytes(value: number): Uint8Array {
  const out = new Uint8Array(2);
  new DataView(out.buffer).setUint16(0, value, true);
  return out;
}

/**
 * Encode a `CONTROL_LIST_GET` or `CONTROL_STATE_GET` command, which takes any session revision.
 * Throws `MisoUsageError` before producing any byte when a field cannot be encoded.
 */
export function encodeControlListRequest(request: ControlListRequest): Uint8Array {
  const requestId = request.requestId;
  if (typeof requestId !== "bigint" || requestId <= 0n || requestId > 0xffff_ffff_ffff_ffffn) {
    throw new MisoUsageError("requestId must be a nonzero u64 bigint");
  }
  const epoch = request.structureEpoch;
  if (typeof epoch !== "bigint" || epoch < 0n || epoch > 0xffff_ffff_ffff_ffffn) {
    throw new MisoUsageError("structureEpoch must be a bigint in [0, 2^64 - 1]");
  }
  const limit = requireU32(request.limit, "limit");
  if (limit > 0xffff) throw new MisoUsageError("limit must fit a u16");
  const fields: Uint8Array[] = [
    field(1, WIRE_U64, true, u64Bytes(epoch)),
    field(2, WIRE_U32, true, u32(requireU32(request.afterHandle, "afterHandle"))),
    field(3, WIRE_U16, true, u16Bytes(limit)),
  ];
  const owner = request.owner;
  if (owner !== undefined) {
    fields.push(field(4, WIRE_U8, false, u8(lookup(OWNER_CODES, owner.kind, "an owner kind"))));
    if (owner.kind !== "session") {
      fields.push(field(5, WIRE_UTF8, false, text(requireId(owner.id, "an owner ID"))));
    }
  }
  return commandFrame(
    request.kind === "list" ? MESSAGE_CONTROL_LIST_GET : MESSAGE_CONTROL_STATE_GET,
    FLAG_REVISION_ANY,
    requestId,
    0n,
    fields.length,
    concat(fields),
  );
}

function fieldOf(fields: readonly RawField[], id: number, wire: number): RawField | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row === undefined) return undefined;
  if (row.wire !== wire) throw malformed(`field ${id} has the wrong type`);
  return row;
}

function view(row: RawField): DataView {
  return new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
}

function stringOf(fields: readonly RawField[], id: number): string | undefined {
  const row = fieldOf(fields, id, WIRE_UTF8);
  return row === undefined ? undefined : DECODER.decode(row.value);
}

function requiredString(fields: readonly RawField[], id: number, what: string): string {
  const value = stringOf(fields, id);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function u8Of(fields: readonly RawField[], id: number): number | undefined {
  return scalar(fields, id, WIRE_U8, 1)?.getUint8(0);
}

function requiredU8(fields: readonly RawField[], id: number, what: string): number {
  const value = u8Of(fields, id);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function u32Of(fields: readonly RawField[], id: number): number | undefined {
  return scalar(fields, id, WIRE_U32, 4)?.getUint32(0, true);
}

function requiredU32(fields: readonly RawField[], id: number, what: string): number {
  const value = u32Of(fields, id);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function f32Of(fields: readonly RawField[], id: number): number | undefined {
  return scalar(fields, id, WIRE_F32, 4)?.getFloat32(0, true);
}

function requiredF32(fields: readonly RawField[], id: number, what: string): number {
  const value = f32Of(fields, id);
  if (value === undefined) throw malformed(`${what} is missing`);
  return value;
}

function boolOf(fields: readonly RawField[], id: number, what: string): boolean {
  const value = scalar(fields, id, WIRE_BOOL, 1)?.getUint8(0);
  if (value === undefined) throw malformed(`${what} is missing`);
  if (value > 1) throw malformed(`${what} is not a boolean`);
  return value === 1;
}

function named<T>(table: ReadonlyMap<number, T>, code: number, what: string): T {
  const found = table.get(code);
  if (found === undefined) throw malformed(`${what} ${code} is not in its table`);
  return found;
}

function decodeOwner(fields: readonly RawField[]): ControlOwner {
  const kind = named(OWNER_NAMES, requiredU8(fields, 1, "an owner kind"), "the owner kind");
  if (kind === "session") return Object.freeze({ kind });
  return Object.freeze({ kind, id: requiredString(fields, 2, "an owner ID") });
}

function decodeSection(fields: readonly RawField[]): ControlSection {
  const kind = named(SECTION_NAMES, requiredU8(fields, 3, "a section kind"), "the section kind");
  if (kind === "console") {
    return Object.freeze({ kind, slot: requiredString(fields, 4, "a console slot ID") });
  }
  if (kind === "inserts") {
    return Object.freeze({ kind, effect: requiredString(fields, 4, "an insert ID") });
  }
  return Object.freeze({ kind });
}

function decodeAddress(row: RawField | undefined): {
  readonly owner: ControlOwner;
  readonly section: ControlSection;
  readonly control: number | undefined;
  readonly lane: ControlLane | undefined;
} {
  if (row === undefined || row.wire !== WIRE_MESSAGE) throw malformed("a row has no address");
  const fields = nestedFields(row.value);
  const lane = u8Of(fields, 6);
  return {
    owner: decodeOwner(fields),
    section: decodeSection(fields),
    control: u32Of(fields, 5),
    lane: lane === undefined ? undefined : named(LANE_NAMES, lane, "the lane"),
  };
}

function decodeChoice(row: RawField): ControlChoice {
  if (row.wire !== WIRE_MESSAGE) throw malformed("a choice is not a message");
  const fields = nestedFields(row.value);
  return Object.freeze({
    value: requiredF32(fields, 1, "a choice value"),
    label: requiredString(fields, 2, "a choice label"),
  });
}

/** The three kinds of row (`control_entry` field 2). */
const ROW_OWNER_HEADER = 1;
const ROW_SECTION_HEADER = 2;
const ROW_CONTROL = 3;

function decodeEntry(row: RawField): ControlHeader | ControlListEntry {
  if (row.wire !== WIRE_MESSAGE) throw malformed("a list row is not a message");
  const fields = nestedFields(row.value);
  const handle = requiredU32(fields, 1, "a handle");
  const kind = requiredU8(fields, 2, "a row kind");
  const address = decodeAddress(fields.find((candidate) => candidate.id === 3));
  if (kind === ROW_OWNER_HEADER) {
    const headerKind = named(OWNER_HEADER_KINDS, requiredU8(fields, 4, "an owner header kind"), "the owner header kind");
    return Object.freeze({
      row: "owner" as const,
      handle,
      address: Object.freeze({ owner: address.owner, section: address.section }),
      kind: headerKind,
      name: requiredString(fields, 5, "an owner name"),
      route: headerKind === "route"
        ? Object.freeze({
          source: requiredString(fields, 7, "a route source"),
          destination: requiredString(fields, 8, "a route destination"),
          tap: named(TAPS, requiredU8(fields, 9, "a route tap"), "the route tap"),
        })
        : undefined,
    });
  }
  if (kind === ROW_SECTION_HEADER) {
    return Object.freeze({
      row: "section" as const,
      handle,
      address: Object.freeze({ owner: address.owner, section: address.section }),
      name: requiredString(fields, 5, "a section name"),
      effect: stringOf(fields, 6),
    });
  }
  if (kind !== ROW_CONTROL) throw malformed(`row kind ${kind} is not in its table`);
  if (address.control === undefined || address.lane === undefined) {
    throw malformed("a control row has no control or lane");
  }
  const ladder = fieldOf(fields, 22, WIRE_PACKED_F32);
  if (ladder === undefined || ladder.value.length % 4 !== 0) throw malformed("a ladder is missing or ragged");
  const ladderView = view(ladder);
  const steps: number[] = [];
  for (let at = 0; at < ladder.value.length; at += 4) steps.push(ladderView.getFloat32(at, true));
  const rampRow = u8Of(fields, 24);
  const reason = u8Of(fields, 27);
  return Object.freeze({
    handle,
    address: Object.freeze({
      owner: address.owner,
      section: address.section,
      control: address.control,
      lane: address.lane,
    }),
    name: requiredString(fields, 10, "a control name"),
    displayName: requiredString(fields, 11, "a display name"),
    displayUnit: requiredString(fields, 12, "a display unit"),
    unit: named(UNITS, requiredU8(fields, 13, "a unit"), "the unit"),
    kind: named(VALUE_KINDS, requiredU8(fields, 14, "a control kind"), "the control kind"),
    minimum: requiredF32(fields, 15, "a minimum"),
    maximum: requiredF32(fields, 16, "a maximum"),
    disabledValue: f32Of(fields, 17),
    default: requiredF32(fields, 18, "a default"),
    taper: named(TAPERS, requiredU8(fields, 19, "a taper"), "the taper"),
    step: requiredF32(fields, 20, "a step"),
    precision: requiredU8(fields, 21, "a precision"),
    ladder: Object.freeze(steps),
    choices: Object.freeze(fields.filter((candidate) => candidate.id === 23).map(decodeChoice)),
    rampRow: rampRow === undefined ? undefined : named(RAMP_ROWS, rampRow, "the ramp row"),
    defaultRampSamples: requiredU32(fields, 25, "a default ramp"),
    class: named(CLASSES, requiredU8(fields, 26, "a class"), "the class"),
    reason: reason === undefined ? undefined : named(REASONS, reason, "the reason"),
    reasonIssue: u32Of(fields, 28),
    reopen: stringOf(fields, 29),
    scope: named(SCOPES, requiredU8(fields, 30, "a scope"), "the scope"),
    automatable: boolOf(fields, 31, "automatable"),
    family: named(FAMILIES, requiredU8(fields, 32, "a family"), "the family"),
    active: boolOf(fields, 33, "active"),
    value: requiredF32(fields, 34, "a value"),
    automated: boolOf(fields, 35, "automated"),
  });
}

function refusalOf<P>(
  envelope: ReturnType<typeof decodeResponseEnvelope>,
): ControlPageResult<P> {
  const body = decodeRefusalBody(envelope.fields);
  return {
    ok: false,
    refusal: Object.freeze({
      status: envelope.status,
      revision: envelope.revision,
      diagnostics: body.diagnostics,
    }),
  };
}

function pageHeader(fields: readonly RawField[]): {
  readonly revision: bigint;
  readonly structureEpoch: bigint;
  readonly lastHandle: number;
  readonly eof: boolean;
} {
  const revision = scalar(fields, 1, WIRE_U64, 8)?.getBigUint64(0, true);
  const structureEpoch = scalar(fields, 2, WIRE_U64, 8)?.getBigUint64(0, true);
  if (revision === undefined || structureEpoch === undefined) throw malformed("a page has no revision or epoch");
  return {
    revision,
    structureEpoch,
    lastHandle: requiredU32(fields, 3, "a last handle"),
    eof: boolOf(fields, 4, "eof"),
  };
}

/**
 * Decode the reply to a `CONTROL_LIST_GET`. A refusal is a value (`ok: false`); bytes that are not
 * a valid reply raise `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeControlListPage(bytes: Uint8Array): ControlPageResult<ControlListPage> & {
  readonly requestId: bigint;
} {
  const envelope = decodeResponseEnvelope(bytes, MESSAGE_CONTROL_LIST_GET);
  if (envelope.status !== "ok") return { ...refusalOf<ControlListPage>(envelope), requestId: envelope.requestId };
  const header = pageHeader(envelope.fields);
  const headers: ControlHeader[] = [];
  const controls: ControlListEntry[] = [];
  for (const row of envelope.fields.filter((candidate) => candidate.id === 5)) {
    const entry = decodeEntry(row);
    if ("row" in entry) headers.push(entry);
    else controls.push(entry);
  }
  return {
    ok: true,
    requestId: envelope.requestId,
    page: Object.freeze({ ...header, headers: Object.freeze(headers), controls: Object.freeze(controls) }),
  };
}

/** The bytes of one state record: `handle:u32`, `value:f32`. */
const STATE_RECORD_BYTES = 8;

/** Decode the reply to a `CONTROL_STATE_GET`, as {@link decodeControlListPage} does. */
export function decodeControlStatePage(bytes: Uint8Array): ControlPageResult<ControlStatePage> & {
  readonly requestId: bigint;
} {
  const envelope = decodeResponseEnvelope(bytes, MESSAGE_CONTROL_STATE_GET);
  if (envelope.status !== "ok") return { ...refusalOf<ControlStatePage>(envelope), requestId: envelope.requestId };
  const header = pageHeader(envelope.fields);
  const count = scalar(envelope.fields, 5, WIRE_U16, 2)?.getUint16(0, true);
  const width = scalar(envelope.fields, 6, WIRE_U16, 2)?.getUint16(0, true);
  const raw = fieldOf(envelope.fields, 7, WIRE_BYTES);
  if (count === undefined || width !== STATE_RECORD_BYTES || raw === undefined) {
    throw malformed("a state page has no records or an unknown record width");
  }
  if (raw.value.length !== count * STATE_RECORD_BYTES) throw malformed("the state records are ragged");
  const records = view(raw);
  const out: { readonly handle: number; readonly value: number }[] = [];
  for (let at = 0; at < raw.value.length; at += STATE_RECORD_BYTES) {
    out.push(Object.freeze({ handle: records.getUint32(at, true), value: records.getFloat32(at + 4, true) }));
  }
  return {
    ok: true,
    requestId: envelope.requestId,
    page: Object.freeze({ ...header, records: Object.freeze(out) }),
  };
}

/** How a list read reaches the core: the endpoint's send function and its one request-ID allocator. */
export interface ControlListTransport {
  readonly send: (frame: Uint8Array) => Uint8Array | PromiseLike<Uint8Array>;
  readonly requestIds: ControlRequestIds;
}

function pageLimit(filter: ControlListFilter | undefined): number {
  const limit = filter?.pageLimit;
  if (limit === undefined) return CONTROL_LIST_PAGE_ROWS;
  if (!Number.isInteger(limit) || limit < 1 || limit > CONTROL_LIST_PAGE_ROWS) {
    throw new MisoUsageError(`pageLimit must be an integer in [1, ${CONTROL_LIST_PAGE_ROWS}]`);
  }
  return limit;
}

function isStale(refusal: ControlPageRefusal): boolean {
  return refusal.diagnostics.some((row) => row.code === CONTROL_LIST_STALE);
}

function staleError(): MisoEngineError {
  return new MisoEngineError("the control list's structure changed while it was read", {
    phase: "lifecycle",
    code: CONTROL_LIST_STALE,
    diagnostics: [{ code: CONTROL_LIST_STALE, path: "" }],
  });
}

function refusalError(refusal: ControlPageRefusal): MisoControlRefusal {
  return new MisoControlRefusal({
    code: refusal.diagnostics[0]?.code ?? refusal.status,
    status: refusal.status,
    revision: refusal.revision,
    editIndex: undefined,
    diagnostics: refusal.diagnostics,
  });
}

function requireReply(reply: { readonly requestId: bigint }, requestId: bigint): void {
  if (reply.requestId !== requestId) {
    throw new MisoEngineError(
      `the control reply answers request ${reply.requestId}, not ${requestId}`,
      { phase: "lifecycle", code: "internal" },
    );
  }
}

function noProgress(): MisoEngineError {
  return malformed("a page is not the last and does not advance the cursor");
}

/** The steps of a read: a generator yields each request frame and is resumed with the core's reply. */
type Steps<T> = Generator<Uint8Array, T, Uint8Array>;

/** Run a read against a send function that answers at once. */
function driveSync<T>(steps: Steps<T>, send: (frame: Uint8Array) => Uint8Array): T {
  let step = steps.next();
  while (step.done !== true) step = steps.next(send(step.value));
  return step.value;
}

/** Run a read against a send function that may answer later. */
async function driveAsync<T>(
  steps: Steps<T>,
  send: (frame: Uint8Array) => Uint8Array | PromiseLike<Uint8Array>,
): Promise<T> {
  let step = steps.next();
  while (step.done !== true) step = steps.next(await send(step.value));
  return step.value;
}

function* listSteps(requestIds: ControlRequestIds, filter: ControlListFilter | undefined): Steps<ControlList> {
  const limit = pageLimit(filter);
  const owner = filter?.owner;
  for (let attempt = 0; attempt <= CONTROL_LIST_RESTARTS; attempt += 1) {
    let epoch = 0n;
    let revision = 0n;
    let after = 0;
    let restart = false;
    const headers: ControlHeader[] = [];
    const controls: ControlListEntry[] = [];
    for (;;) {
      const requestId = requestIds.next();
      const reply = decodeControlListPage(yield encodeControlListRequest({
        kind: "list",
        requestId,
        structureEpoch: epoch,
        afterHandle: after,
        limit,
        owner,
      }));
      requireReply(reply, requestId);
      if (!reply.ok) {
        if (isStale(reply.refusal)) {
          restart = true;
          break;
        }
        throw refusalError(reply.refusal);
      }
      const { page } = reply;
      if (after === 0) {
        epoch = page.structureEpoch;
        revision = page.revision;
      } else if (page.structureEpoch !== epoch) {
        // The core refuses a request for another structure; a page that names one anyway is not mixed in.
        restart = true;
        break;
      }
      headers.push(...page.headers);
      controls.push(...page.controls);
      if (page.eof) break;
      if (page.lastHandle <= after) throw noProgress();
      after = page.lastHandle;
    }
    if (restart) continue;
    return Object.freeze({
      revision,
      structureEpoch: epoch,
      headers: Object.freeze(headers),
      controls: Object.freeze(controls),
      owner,
    });
  }
  throw staleError();
}

/**
 * Read the whole control list: `CONTROL_LIST_GET` pages of `filter.pageLimit` rows (256 by
 * default) until `eof`.
 *
 * A page refused `control.list.stale` (the structure changed since the first page) restarts the read
 * from the first page; after {@link CONTROL_LIST_RESTARTS} restarts it rejects with
 * `MisoEngineError` `control.list.stale`. Any other refusal rejects with `MisoControlRefusal`. The
 * pages of one list always share one `structure_epoch`.
 */
export function readControlList(
  transport: ControlListTransport,
  filter?: ControlListFilter,
): Promise<ControlList> {
  return driveAsync(listSteps(transport.requestIds, filter), transport.send);
}

/**
 * The strips of the running session in the core's order (decision 17, C4; #1564 D5): the owner
 * header rows of kind `track`, then `submix`, as the list gives them. Strip `i` of a meter frame, an
 * observation binding or a response is `[...tracks, ...submixes][i]`. The order belongs to one
 * `structureEpoch`; a reply that names another epoch means it may have changed.
 */
export interface ControlStripOrder {
  readonly structureEpoch: bigint;
  readonly tracks: readonly string[];
  readonly submixes: readonly string[];
  /**
   * The effect each console slot and insert of each strip runs, from the list's section headers
   * (#1562b-K). Read it with {@link stripEffectOf}; a rebuild can change an insert's effect, so the
   * map belongs to this order's `structureEpoch`.
   */
  readonly effects: ReadonlyMap<string, string>;
}

function effectKey(stripId: string, kind: "console" | "inserts", id: string): string {
  return `${stripId}\u0000${kind}\u0000${id}`;
}

/**
 * The effect a strip's console slot or insert runs, as the core's section header names it, or
 * `undefined` when `order` has no such section.
 */
export function stripEffectOf(
  order: ControlStripOrder,
  stripId: string,
  section: { readonly kind: "console"; readonly slot: string } | { readonly kind: "inserts"; readonly effect: string },
): string | undefined {
  return order.effects?.get(
    section.kind === "console"
      ? effectKey(stripId, "console", section.slot)
      : effectKey(stripId, "inserts", section.effect),
  );
}

/**
 * One console slot or insert section of a strip, as the list's section header names it: the
 * strip, the section kind, the slot's or insert's ID and the effect identity it runs.
 */
type ControlStripSection = {
  readonly stripId: string;
  readonly rack: "console" | "inserts";
  readonly sectionId: string;
  readonly effect: string;
};

/** The sections {@link stripOrderOf} read from the same headers as each order's `effects`. */
const orderSections = new WeakMap<ControlStripOrder, readonly ControlStripSection[]>();

/**
 * @internal The console slot and insert sections of every strip of `order`, in list order, read in
 * the same pass as its `effects` (#1590 D3). Empty for an order this module did not build.
 */
export function stripSections(order: ControlStripOrder): readonly {
  readonly stripId: string;
  readonly rack: "console" | "inserts";
  readonly sectionId: string;
  readonly effect: string;
}[] {
  return orderSections.get(order) ?? [];
}

/** @internal Whether this module read `order`'s sections, so {@link stripSections} is its whole list. */
export function stripSectionsKnown(order: ControlStripOrder): boolean {
  return orderSections.has(order);
}

/** The strip order of an already read list: its track and submix owner headers, in list order. */
export function stripOrderOf(list: ControlList): ControlStripOrder {
  const tracks: string[] = [];
  const submixes: string[] = [];
  const effects = new Map<string, string>();
  const sections: ControlStripSection[] = [];
  for (const header of list.headers) {
    if (header.row === "section") {
      const { owner, section } = header.address;
      if (owner.kind === "strip" && header.effect !== undefined) {
        if (section.kind === "console") {
          effects.set(effectKey(owner.id, "console", section.slot), header.effect);
          sections.push(Object.freeze({ stripId: owner.id, rack: "console" as const, sectionId: section.slot, effect: header.effect }));
        } else if (section.kind === "inserts") {
          effects.set(effectKey(owner.id, "inserts", section.effect), header.effect);
          sections.push(Object.freeze({ stripId: owner.id, rack: "inserts" as const, sectionId: section.effect, effect: header.effect }));
        }
      }
      continue;
    }
    if (header.kind !== "track" && header.kind !== "submix") continue;
    const owner = header.address.owner;
    if (owner.kind !== "strip") throw malformed("a strip header does not name a strip");
    (header.kind === "track" ? tracks : submixes).push(owner.id);
  }
  const order: ControlStripOrder = Object.freeze({
    structureEpoch: list.structureEpoch,
    tracks: Object.freeze(tracks),
    submixes: Object.freeze(submixes),
    effects,
  });
  orderSections.set(order, Object.freeze(sections));
  return order;
}

/**
 * Probe the current structure epoch with the smallest list request the core serves (one row of the
 * session owner), then read the full list only when the epoch is not `known`'s.
 */
function* stripOrderSteps(
  requestIds: ControlRequestIds,
  known: ControlStripOrder | undefined,
): Steps<ControlStripOrder> {
  const requestId = requestIds.next();
  const probe = decodeControlListPage(yield encodeControlListRequest({
    kind: "list",
    requestId,
    structureEpoch: 0n,
    afterHandle: 0,
    limit: 1,
    owner: { kind: "session" },
  }));
  requireReply(probe, requestId);
  if (!probe.ok) throw refusalError(probe.refusal);
  if (known !== undefined && known.structureEpoch === probe.page.structureEpoch) return known;
  return stripOrderOf(yield* listSteps(requestIds, undefined));
}

/**
 * The strip order of the session, read from the core's list. Pass the order read last as `known`:
 * a probe that finds the same epoch returns it without reading the list again. A browser reads it
 * through the host's `control` message, so the send may answer later.
 */
export function readControlStripOrder(
  transport: ControlListTransport,
  known?: ControlStripOrder,
): Promise<ControlStripOrder> {
  return driveAsync(stripOrderSteps(transport.requestIds, known), transport.send);
}

/** {@link readControlStripOrder} against a core that answers at once (the in-process engine). */
export function readControlStripOrderSync(
  transport: { readonly send: (frame: Uint8Array) => Uint8Array; readonly requestIds: ControlRequestIds },
  known?: ControlStripOrder,
): ControlStripOrder {
  return driveSync(stripOrderSteps(transport.requestIds, known), transport.send);
}

/**
 * Read the committed values of `list`'s control rows: `CONTROL_STATE_GET` pages of the structure the
 * list was read at. A structure that changed since rejects with `MisoEngineError`
 * `control.list.stale`; the caller lists again.
 */
export async function readControlState(
  transport: ControlListTransport,
  list: ControlList,
  filter?: Pick<ControlListFilter, "pageLimit">,
): Promise<ControlState> {
  const limit = pageLimit(filter);
  const values = new Map<number, number>();
  let revision = 0n;
  let after = 0;
  for (;;) {
    const requestId = transport.requestIds.next();
    const reply = decodeControlStatePage(await transport.send(encodeControlListRequest({
      kind: "state",
      requestId,
      structureEpoch: list.structureEpoch,
      afterHandle: after,
      limit,
      owner: list.owner,
    })));
    requireReply(reply, requestId);
    if (!reply.ok) {
      if (isStale(reply.refusal)) throw staleError();
      throw refusalError(reply.refusal);
    }
    const { page } = reply;
    if (page.structureEpoch !== list.structureEpoch) throw staleError();
    if (after === 0) revision = page.revision;
    for (const record of page.records) values.set(record.handle, record.value);
    if (page.eof) break;
    if (page.lastHandle <= after) throw noProgress();
    after = page.lastHandle;
  }
  return Object.freeze({ revision, structureEpoch: list.structureEpoch, values });
}
