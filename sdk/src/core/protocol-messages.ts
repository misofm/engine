import {
  DECODER,
  FLAG_REVISION_ANY,
  WIRE_BOOL,
  WIRE_BYTES,
  WIRE_MESSAGE,
  WIRE_PACKED_U16,
  WIRE_PACKED_U32,
  WIRE_U16,
  WIRE_U32,
  WIRE_U64,
  WIRE_U8,
  WIRE_UTF8,
  MESSAGE_SESSION_REPLACE,
  commandFrame,
  concat,
  decodeAppliedReply,
  decodeControlReply,
  decodeProtocolDiagnostic,
  decodeRefusalFields,
  decodeResponseEnvelope,
  encodeRevisionProbe,
  field,
  malformed,
  nested as nestedField,
  nestedFields,
  readFields,
  requireU32,
  requireU64,
  scalar,
  text,
  u32,
  u8,
} from "./control-frame.ts";
import type { ControlBackpressure, ControlStatusName, RawField } from "./control-frame.ts";
import type { ControlRequestIds } from "./control-queue.ts";
import { MisoControlRefusal, MisoUsageError } from "./errors.ts";
import { decodeObservationEnded } from "./observation-ended.ts";
import { documentSha256, editOutcome, encodeSessionReplaceRequest } from "./session-edit.ts";
import type { EditOutcome } from "./session-edit.ts";
import { encodeSessionTransaction } from "./session-transaction.ts";
import type { SessionTransaction } from "./session-transaction.ts";
import type { ObservationEndedKey } from "./observation-ended.ts";

/**
 * The SDK's one codec of the protocol's query and setting messages and of its six events
 * (decision 17, C1 and C13; #1571 D1), and the calls both engines make with it (D2, D3).
 *
 * # What it encodes and decodes
 *
 * Requests: `CAPABILITIES_GET`, `SESSION_SNAPSHOT_GET`, `TRANSPORT_GET`, `TRANSPORT_SET`,
 * `TELEMETRY_CONFIGURE`, `COUNTERS_GET` and `DIAGNOSTICS_GET`. Replies: the success payload of each,
 * and the non-OK body (status, diagnostics, omitted count, backpressure) every command shares.
 * Events: `SESSION_COMMITTED`, `TRANSPORT_STATE`, `OBSERVATION_ENDED` (through the one decoder in
 * `observation-ended.ts`), `METER_BATCH`, `COUNTER_SNAPSHOT` and `DIAGNOSTIC`. Field IDs, wire
 * types and presence rules are the protocol schema's (`crates/protocol/src/schema.rs`).
 * `sdk/test/fixtures/protocol-messages.json` holds the Rust codec's frames of each
 * (`crates/protocol/tests/sdk_message_fixture.rs` keeps it equal to what the codec writes), and
 * `sdk/test/protocol-messages-evals.mjs` holds this file to it byte for byte and value for value.
 *
 * # What it does not do
 *
 * It checks only what makes a frame encodable (D6): types, the integer ranges of the wire fields,
 * and names the protocol has a code for. The core answers everything else with its own status and
 * diagnostic codes: a zero diagnostics `limit`, an unsorted counter list or an unknown meter handle
 * is the core's to refuse. Bytes that are not a valid frame raise `MisoEngineError`
 * (`phase: "lifecycle"`, `code: "internal"`).
 */

const MESSAGE_SESSION_SNAPSHOT_GET = 0x0002;
const MESSAGE_TRANSPORT_GET = 0x0007;
const MESSAGE_TRANSPORT_SET = 0x0008;
const MESSAGE_TELEMETRY_CONFIGURE = 0x0009;
const MESSAGE_COUNTERS_GET = 0x000a;
const MESSAGE_DIAGNOSTICS_GET = 0x000b;
const MESSAGE_CAPABILITIES_GET = 0x0001;

const EVENT_SESSION_COMMITTED = 0x8001;
const EVENT_TRANSPORT_STATE = 0x8010;
const EVENT_OBSERVATION_ENDED = 0x8011;
const EVENT_METER_BATCH = 0x8020;
const EVENT_COUNTER_SNAPSHOT = 0x8021;
const EVENT_DIAGNOSTIC = 0x8030;

const HEADER_BYTES = 48;
const MAGIC = [0x4d, 0x49, 0x53, 0x4f, 0x43, 0x54, 0x4c, 0x00] as const;
const KIND_EVENT = 3;
const U16_MAXIMUM = 0xffff;
const METER_RECORD_BYTES = 16;

/**
 * The protocol's counter IDs, by the camel-case name of the Rust `CounterId` variant. IDs 2 and 11
 * to 14 are retired (#1629). The fixture lists every ID the codec accepts, and the SDK eval holds
 * this table equal to it, so a counter the core adds or retires turns the eval red until it is here.
 */
export const COUNTER_IDS = Object.freeze([
  { name: "controlCommandBackpressure", code: 1 },
  { name: "reliableResponseBackpressure", code: 3 },
  { name: "reliableEventBackpressure", code: 4 },
  { name: "telemetryCoalesced", code: 5 },
  { name: "telemetryDropped", code: 6 },
  { name: "malformedFrames", code: 7 },
  { name: "replayHits", code: 8 },
  { name: "requestIdReuse", code: 9 },
  { name: "replayExpired", code: 10 },
  { name: "validationFailures", code: 15 },
  { name: "liveValuesSuperseded", code: 16 },
  { name: "transitionReprepareRefusals", code: 17 },
  { name: "stripsRestarted", code: 18 },
  { name: "liveBlockErrors", code: 19 },
  { name: "sourceUnderrunBlocks", code: 20 },
  { name: "sourceFramesDiscarded", code: 21 },
  { name: "meterWindowsLost", code: 22 },
] as const);

/** A protocol counter, by name. */
export type CounterName = (typeof COUNTER_IDS)[number]["name"];
/** The transport's absolute state. */
export type TransportStateName = "stopped" | "playing";
/** A diagnostic's severity. */
export type DiagnosticSeverityName = "info" | "warning" | "error";
/** A meter record's component. */
export type MeterComponentName = "left" | "right" | "aggregate";

const COUNTER_CODES: ReadonlyMap<string, number> = new Map(COUNTER_IDS.map((row) => [row.name, row.code]));
const COUNTER_NAMES: ReadonlyMap<number, CounterName> = new Map(COUNTER_IDS.map((row) => [row.code, row.name]));
const TRANSPORT_CODES: ReadonlyMap<string, number> = new Map([["stopped", 1], ["playing", 2]]);
const TRANSPORT_NAMES: readonly (TransportStateName | undefined)[] = [undefined, "stopped", "playing"];
const SEVERITY_CODES: ReadonlyMap<string, number> = new Map([["info", 1], ["warning", 2], ["error", 3]]);
const SEVERITY_NAMES: readonly (DiagnosticSeverityName | undefined)[] = [undefined, "info", "warning", "error"];
const COMPONENT_NAMES: readonly (MeterComponentName | undefined)[] = [undefined, "left", "right", "aggregate"];
const METRIC_CODES: ReadonlyMap<string, number> = new Map([["peak", 1], ["gainReduction", 2]]);
const METRIC_NAMES: readonly (MeterMetricName | undefined)[] = [undefined, "peak", "gainReduction"];

/** A protocol version an endpoint supports. */
export interface ProtocolVersion {
  readonly major: number;
  readonly minor: number;
}

/** The fields of a `CAPABILITIES_GET` reply, by the Rust `Capabilities` field names. */
export interface EngineCapabilities {
  readonly minimumVersion: ProtocolVersion;
  readonly maximumVersion: ProtocolVersion;
  readonly maximumFrameBytes: bigint;
  readonly maximumTlvs: number;
  readonly maximumStringBytes: bigint;
  readonly maximumNesting: number;
  readonly controlCommandSlots: bigint;
  readonly controlCommandBytes: bigint;
  readonly reliableResponseSlots: bigint;
  readonly reliableEventSlots: bigint;
  readonly telemetrySlots: bigint;
  readonly replayEntries: bigint;
  readonly replayBytes: bigint;
  readonly maximumCachedResponseBytes: bigint;
  readonly maximumControlPageItems: number;
  readonly maximumDiagnosticPageItems: number;
  readonly maximumTelemetryHandles: number;
  readonly maximumTransactionEdits: number;
  /** The command message IDs the endpoint serves, ascending. They differ per host. */
  readonly supportedCommands: readonly number[];
  /** The event message IDs the endpoint sends, ascending. */
  readonly supportedEvents: readonly number[];
  readonly flags: bigint;
}

/** One page of the canonical committed document (`SESSION_SNAPSHOT_GET`). */
export interface SessionSnapshotPage {
  readonly totalBytes: bigint;
  readonly offset: bigint;
  /** The page's bytes; a page may end inside a UTF-8 code point. */
  readonly chunk: Uint8Array;
  readonly eof: boolean;
}

/** The committed session document and the revision it is the document of. */
export interface CommittedDocument {
  readonly revision: bigint;
  /** The canonical JSON the core holds, decoded as UTF-8 once every page was read. */
  readonly document: string;
}

/** The transport as the core reports it. */
export interface TransportSnapshot {
  readonly state: TransportStateName;
  /** The engine sample at which this state is effective. */
  readonly effectiveSample: bigint;
}

/** What `setTransport` asks for. The stored position is retired (#1635): a seek moves the playhead. */
export interface TransportRequest {
  readonly state: TransportStateName;
}

/**
 * One meter table entry (#1583 D1): the session output, or a strip by its stable ID, and the metric.
 * The master strip (#1690 D3) is the strip the session's master row names; it has no strip ID and
 * carries gain reduction only. Its handle is its 1-based position in the table.
 */
export type MeterEntry =
  | { readonly target: "output"; readonly metric: MeterMetricName }
  | { readonly target: "master"; readonly metric: "gainReduction" }
  | { readonly target: "strip"; readonly stripId: string; readonly metric: MeterMetricName };

/** A meter table entry's metric. */
export type MeterMetricName = "peak" | "gainReduction";

/** The endpoint's telemetry: which meters and counters it sends as events, and how often. */
export interface TelemetryConfiguration {
  /** The meter table: entry `i` is meter handle `i + 1`. */
  readonly meters: readonly MeterEntry[];
  /** Blocks per meter batch: a whole number of meter windows when the table is not empty. */
  readonly meterPeriodBlocks: number;
  /** The engine's meter window in blocks, in a reply only, when the engine delivers meters. */
  readonly meterWindowBlocks?: number;
  readonly counterIds: readonly CounterName[];
  readonly counterPeriodBlocks: number;
  readonly diagnosticsEnabled: boolean;
  readonly minimumSeverity: DiagnosticSeverityName;
}

/** One counter's value. Counters never reset. */
export interface CounterValue {
  readonly id: CounterName;
  readonly value: bigint;
}

/** The counters the core reported, all observed at one sample. */
export interface CounterSnapshot {
  readonly observedSample: bigint;
  readonly values: readonly CounterValue[];
}

/** One segment of a diagnostic's path: a field name, an array index or a stable ID. */
export type DiagnosticPathSegment =
  | { readonly kind: "field"; readonly field: string }
  | { readonly kind: "index"; readonly index: bigint }
  | { readonly kind: "stableId"; readonly stableId: string };

/** One protocol diagnostic, with every field the core sent. An absent optional field is an absent key. */
export interface ProtocolDiagnostic {
  /** The core's stable dotted code. */
  readonly code: string;
  readonly severity: DiagnosticSeverityName;
  /** The path into the document; empty is the root. */
  readonly path: readonly DiagnosticPathSegment[];
  readonly detail?: string;
  readonly operationIndex?: number;
  readonly sampleTime?: bigint;
  readonly providerSequence?: bigint;
}

/** What `diagnostics` reads. Each field left out takes the default shown. */
export interface DiagnosticsQuery {
  /** Read diagnostics after this provider sequence. Default `0n`. */
  readonly afterSequence?: bigint;
  /** The most diagnostics in the page, a `u16`. Default 64. The core refuses `0`. */
  readonly limit?: number;
  /** Default `"info"`. */
  readonly minimumSeverity?: DiagnosticSeverityName;
}

/** One page of the core's retained diagnostics. */
export interface DiagnosticsPage {
  readonly lastSequence: bigint;
  readonly eof: boolean;
  readonly diagnostics: readonly ProtocolDiagnostic[];
}

/** One 16-byte meter record of a `METER_BATCH` event. */
export interface MeterRecord {
  readonly handle: number;
  readonly component: MeterComponentName;
  /** The `VALID` (1), `CLIPPED` (2) and `HELD` (4) bits. */
  readonly flags: number;
  readonly value: number;
}

/** A protocol event, decoded. `revision` is the event header's session revision. */
export type ProtocolEvent =
  | {
    readonly kind: "sessionCommitted";
    readonly revision: bigint;
    readonly sequence: bigint;
    readonly originRequestId: bigint;
    readonly previousRevision: bigint;
    readonly appliedOperations: number;
  }
  | {
    readonly kind: "transportState";
    readonly revision: bigint;
    readonly sequence: bigint;
    readonly state: TransportStateName;
    readonly effectiveSample: bigint;
    /** The `TRANSPORT_SET` request that made the change, when a command made it. */
    readonly originRequestId?: bigint;
  }
  | {
    readonly kind: "observationEnded";
    readonly revision: bigint;
    readonly sequence: bigint;
    readonly keys: readonly ObservationEndedKey[];
  }
  | {
    readonly kind: "meterBatch";
    readonly revision: bigint;
    /** One past the window's last sample: the window is `[firstSample, observedSample)`. */
    readonly observedSample: bigint;
    readonly firstSample: bigint;
    /** The window's sequence since the accepted configuration, from 1. */
    readonly windowSequence: bigint;
    readonly records: readonly MeterRecord[];
  }
  | {
    readonly kind: "counterSnapshot";
    readonly revision: bigint;
    readonly observedSample: bigint;
    readonly values: readonly CounterValue[];
  }
  | { readonly kind: "diagnostic"; readonly revision: bigint; readonly diagnostic: ProtocolDiagnostic };

/** The kinds `ProtocolEvent` takes. */
export type ProtocolEventKind = ProtocolEvent["kind"];

/** A refused reply: nothing changed, and the core says why. */
export interface ProtocolRefusal {
  readonly ok: false;
  readonly requestId: bigint;
  /** The revision the core observed. */
  readonly revision: bigint;
  readonly status: ControlStatusName;
  readonly diagnostics: readonly ProtocolDiagnostic[];
  readonly omittedDiagnostics: number;
  /** Present exactly when `status` is `backpressure`. */
  readonly backpressure?: ControlBackpressure;
}

/** A decoded reply: the value, or the refusal. */
export type ProtocolReply<T> =
  | { readonly ok: true; readonly requestId: bigint; readonly revision: bigint; readonly value: T }
  | ProtocolRefusal;

// ----------------------------------------------------------------------------------------------
// Encoding.

function code(table: ReadonlyMap<string, number>, name: unknown, what: string): number {
  const found = typeof name === "string" ? table.get(name) : undefined;
  if (found === undefined) {
    throw new MisoUsageError(`${what} ${String(name)} is not one of ${[...table.keys()].join(", ")}`);
  }
  return found;
}

function requestIdOf(value: unknown): bigint {
  const id = requireU64(value, "requestId");
  if (id === 0n) throw new MisoUsageError("requestId must not be zero");
  return id;
}

function u64(value: bigint): Uint8Array {
  const out = new Uint8Array(8);
  new DataView(out.buffer).setBigUint64(0, value, true);
  return out;
}

function u16(value: number): Uint8Array {
  const out = new Uint8Array(2);
  new DataView(out.buffer).setUint16(0, value, true);
  return out;
}

function requireU16(value: unknown, what: string): number {
  if (typeof value !== "number" || !Number.isInteger(value) || value < 0 || value > U16_MAXIMUM) {
    throw new MisoUsageError(`${what} must be an integer in [0, ${U16_MAXIMUM}]`);
  }
  return value;
}

function packedU32(values: readonly number[]): Uint8Array {
  const out = new Uint8Array(values.length * 4);
  const view = new DataView(out.buffer);
  values.forEach((value, index) => view.setUint32(index * 4, value, true));
  return out;
}

function list<T>(value: unknown, what: string): readonly T[] {
  if (!Array.isArray(value)) throw new MisoUsageError(`${what} must be an array`);
  return value as readonly T[];
}

function query(message: number, requestId: unknown, fields: readonly Uint8Array[]): Uint8Array {
  return commandFrame(message, FLAG_REVISION_ANY, requestIdOf(requestId), 0n, fields.length, concat(fields));
}

function mutation(
  message: number,
  requestId: unknown,
  expectedRevision: unknown,
  fields: readonly Uint8Array[],
): Uint8Array {
  const revision = requireU64(expectedRevision, "expectedRevision");
  return commandFrame(message, 0, requestIdOf(requestId), revision, fields.length, concat(fields));
}

/** Encode `CAPABILITIES_GET`: no fields, any revision. */
export function encodeCapabilitiesRequest(request: { readonly requestId: bigint }): Uint8Array {
  return encodeRevisionProbe(requestIdOf(request.requestId));
}

/**
 * Encode `SESSION_SNAPSHOT_GET`: `maximumBytes` of the canonical committed document from `offset`.
 * With `expectedRevision` the core answers only for that revision; without it, for any.
 */
export function encodeSessionSnapshotRequest(request: {
  readonly requestId: bigint;
  readonly expectedRevision?: bigint;
  readonly offset: bigint;
  readonly maximumBytes: number;
}): Uint8Array {
  const fields = [
    field(1, WIRE_U64, true, u64(requireU64(request.offset, "offset"))),
    field(2, WIRE_U32, true, u32(requireU32(request.maximumBytes, "maximumBytes"))),
  ];
  return request.expectedRevision === undefined
    ? query(MESSAGE_SESSION_SNAPSHOT_GET, request.requestId, fields)
    : mutation(MESSAGE_SESSION_SNAPSHOT_GET, request.requestId, request.expectedRevision, fields);
}

/** Encode `TRANSPORT_GET`: no fields, any revision. */
export function encodeTransportGetRequest(request: { readonly requestId: bigint }): Uint8Array {
  return query(MESSAGE_TRANSPORT_GET, request.requestId, []);
}

/** Encode `TRANSPORT_SET`, which needs the exact committed revision. */
export function encodeTransportSetRequest(request: {
  readonly requestId: bigint;
  readonly expectedRevision: bigint;
  readonly state: TransportStateName;
}): Uint8Array {
  return mutation(MESSAGE_TRANSPORT_SET, request.requestId, request.expectedRevision, [
    field(1, WIRE_U8, true, u8(code(TRANSPORT_CODES, request.state, "a transport state"))),
  ]);
}

/** Encode `TELEMETRY_CONFIGURE`, which needs the exact committed revision. */
export function encodeTelemetryConfigureRequest(request: {
  readonly requestId: bigint;
  readonly expectedRevision: bigint;
  readonly configuration: TelemetryConfiguration;
}): Uint8Array {
  const configuration = request.configuration;
  if (typeof configuration !== "object" || configuration === null) {
    throw new MisoUsageError("a telemetry configuration must be an object");
  }
  const meters = list<MeterEntry>(configuration.meters, "meters").map(meterEntryField);
  const counters = list<CounterName>(configuration.counterIds, "counterIds")
    .map((name) => code(COUNTER_CODES, name, "a counter"));
  if (typeof configuration.diagnosticsEnabled !== "boolean") {
    throw new MisoUsageError("diagnosticsEnabled must be a boolean");
  }
  // Field 1, the positional meter handles, is retired (#1583 D2); the table is field 7.
  return mutation(MESSAGE_TELEMETRY_CONFIGURE, request.requestId, request.expectedRevision, [
    field(2, WIRE_U32, true, u32(requireU32(configuration.meterPeriodBlocks, "meterPeriodBlocks"))),
    field(3, WIRE_PACKED_U32, true, packedU32(counters)),
    field(4, WIRE_U32, true, u32(requireU32(configuration.counterPeriodBlocks, "counterPeriodBlocks"))),
    field(5, WIRE_BOOL, true, u8(configuration.diagnosticsEnabled ? 1 : 0)),
    field(6, WIRE_U8, true, u8(code(SEVERITY_CODES, configuration.minimumSeverity, "a severity"))),
    ...meters,
  ]);
}

/**
 * One meter table entry as its repeated field 7 message: target, the strip ID of a strip, metric.
 * No metric check here: the engine refuses an invalid target and metric pair (`INVALID_FIELD`).
 */
function meterEntryField(entry: MeterEntry): Uint8Array {
  if (typeof entry !== "object" || entry === null) throw new MisoUsageError("a meter entry must be an object");
  const metric = field(3, WIRE_U8, true, u8(code(METRIC_CODES, entry.metric, "a meter metric")));
  if (entry.target === "output") return nestedField(7, false, [field(1, WIRE_U8, true, u8(1)), metric]);
  if (entry.target === "master") return nestedField(7, false, [field(1, WIRE_U8, true, u8(3)), metric]);
  if (entry.target === "strip") {
    if (typeof entry.stripId !== "string") throw new MisoUsageError("a strip meter entry needs a stripId string");
    return nestedField(7, false, [field(1, WIRE_U8, true, u8(2)), field(2, WIRE_UTF8, false, text(entry.stripId)), metric]);
  }
  throw new MisoUsageError(`a meter target ${String((entry as { target?: unknown }).target)} is not output, master or strip`);
}

/** Encode `COUNTERS_GET`: the named counters, in the order given, or every counter when `ids` is absent. */
export function encodeCountersRequest(request: {
  readonly requestId: bigint;
  readonly ids?: readonly CounterName[];
}): Uint8Array {
  if (request.ids === undefined) {
    return query(MESSAGE_COUNTERS_GET, request.requestId, [field(1, WIRE_BOOL, true, u8(1))]);
  }
  const codes = list<CounterName>(request.ids, "ids").map((name) => code(COUNTER_CODES, name, "a counter"));
  return query(MESSAGE_COUNTERS_GET, request.requestId, [
    field(1, WIRE_BOOL, true, u8(0)),
    field(2, WIRE_PACKED_U32, false, packedU32(codes)),
  ]);
}

/** Encode `DIAGNOSTICS_GET`. Nothing is defaulted here: the call (`ProtocolClient`) holds the defaults. */
export function encodeDiagnosticsRequest(request: {
  readonly requestId: bigint;
  readonly afterSequence: bigint;
  readonly limit: number;
  readonly minimumSeverity: DiagnosticSeverityName;
}): Uint8Array {
  return query(MESSAGE_DIAGNOSTICS_GET, request.requestId, [
    field(1, WIRE_U64, true, u64(requireU64(request.afterSequence, "afterSequence"))),
    field(2, WIRE_U16, true, u16(requireU16(request.limit, "limit"))),
    field(3, WIRE_U8, true, u8(code(SEVERITY_CODES, request.minimumSeverity, "a severity"))),
  ]);
}

// ----------------------------------------------------------------------------------------------
// Decoding.

function only(fields: readonly RawField[], id: number, wire: number): RawField | undefined {
  const found = fields.filter((row) => row.id === id);
  if (found.length > 1) throw malformed(`field ${id} repeats`);
  const row = found[0];
  if (row !== undefined && row.wire !== wire) throw malformed(`field ${id} has the wrong type`);
  return row;
}

function need(fields: readonly RawField[], id: number, wire: number): RawField {
  const row = only(fields, id, wire);
  if (row === undefined) throw malformed(`field ${id} is missing`);
  return row;
}

function fixed(fields: readonly RawField[], id: number, wire: number, width: number): DataView {
  const view = scalar(fields, id, wire, width);
  if (view === undefined) throw malformed(`field ${id} is missing`);
  return view;
}

const readU8 = (fields: readonly RawField[], id: number) => fixed(fields, id, WIRE_U8, 1).getUint8(0);
const readU16 = (fields: readonly RawField[], id: number) => fixed(fields, id, WIRE_U16, 2).getUint16(0, true);
const readU32 = (fields: readonly RawField[], id: number) => fixed(fields, id, WIRE_U32, 4).getUint32(0, true);
const readU64 = (fields: readonly RawField[], id: number) => fixed(fields, id, WIRE_U64, 8).getBigUint64(0, true);

function readBool(fields: readonly RawField[], id: number): boolean {
  const value = fixed(fields, id, WIRE_BOOL, 1).getUint8(0);
  if (value > 1) throw malformed(`field ${id} is not a boolean`);
  return value === 1;
}

function packed(row: RawField, width: 2 | 4): number[] {
  if (row.value.length % width !== 0) throw malformed(`field ${row.id} is not a packed list`);
  const view = new DataView(row.value.buffer, row.value.byteOffset, row.value.byteLength);
  const out: number[] = [];
  for (let at = 0; at < row.value.length; at += width) {
    out.push(width === 2 ? view.getUint16(at, true) : view.getUint32(at, true));
  }
  return out;
}

function named<T>(table: readonly (T | undefined)[], value: number, what: string): T {
  const name = table[value];
  if (name === undefined) throw malformed(`${what} ${value} is not registered`);
  return name;
}

function counterName(value: number): CounterName {
  const name = COUNTER_NAMES.get(value);
  if (name === undefined) throw malformed(`counter ${value} is not registered`);
  return name;
}

function nested(row: RawField): RawField[] {
  if (row.wire !== WIRE_MESSAGE) throw malformed(`field ${row.id} is not a message`);
  return nestedFields(row.value);
}

function decodeReply<T>(
  bytes: Uint8Array,
  message: number,
  body: (fields: readonly RawField[]) => T,
): ProtocolReply<T> {
  const { requestId, status, revision, fields } = decodeResponseEnvelope(bytes, message);
  if (status === "ok") return Object.freeze({ ok: true, requestId, revision, value: body(fields) });
  const { diagnostics, omittedDiagnostics, backpressure } = decodeRefusalFields(fields);
  return Object.freeze({
    ok: false,
    requestId,
    revision,
    status,
    diagnostics,
    omittedDiagnostics,
    ...(backpressure === undefined ? {} : { backpressure }),
  });
}

function version(fields: readonly RawField[], major: number): ProtocolVersion {
  return Object.freeze({ major: readU16(fields, major), minor: readU16(fields, major + 1) });
}

/** Decode the reply to `CAPABILITIES_GET`. */
export function decodeCapabilitiesReply(bytes: Uint8Array): ProtocolReply<EngineCapabilities> {
  return decodeReply(bytes, MESSAGE_CAPABILITIES_GET, (fields) => Object.freeze({
    minimumVersion: version(fields, 1),
    maximumVersion: version(fields, 3),
    maximumFrameBytes: readU64(fields, 5),
    maximumTlvs: readU32(fields, 6),
    maximumStringBytes: readU64(fields, 7),
    maximumNesting: readU8(fields, 8),
    controlCommandSlots: readU64(fields, 10),
    controlCommandBytes: readU64(fields, 11),
    reliableResponseSlots: readU64(fields, 13),
    reliableEventSlots: readU64(fields, 14),
    telemetrySlots: readU64(fields, 15),
    replayEntries: readU64(fields, 16),
    replayBytes: readU64(fields, 17),
    maximumCachedResponseBytes: readU64(fields, 18),
    maximumControlPageItems: readU16(fields, 21),
    maximumDiagnosticPageItems: readU16(fields, 22),
    maximumTelemetryHandles: readU16(fields, 23),
    maximumTransactionEdits: readU32(fields, 24),
    supportedCommands: Object.freeze(packed(need(fields, 25, WIRE_PACKED_U16), 2)),
    supportedEvents: Object.freeze(packed(need(fields, 26, WIRE_PACKED_U16), 2)),
    flags: readU64(fields, 27),
  }));
}

/** Decode the reply to `SESSION_SNAPSHOT_GET`. The page's bytes are copied out of the frame. */
export function decodeSessionSnapshotReply(bytes: Uint8Array): ProtocolReply<SessionSnapshotPage> {
  return decodeReply(bytes, MESSAGE_SESSION_SNAPSHOT_GET, (fields) => Object.freeze({
    totalBytes: readU64(fields, 1),
    offset: readU64(fields, 2),
    chunk: need(fields, 3, WIRE_BYTES).value.slice(),
    eof: readBool(fields, 4),
  }));
}

function transportBody(fields: readonly RawField[]): TransportSnapshot {
  return Object.freeze({
    state: named(TRANSPORT_NAMES, readU8(fields, 1), "transport state"),
    effectiveSample: readU64(fields, 3),
  });
}

/** Decode the reply to `TRANSPORT_GET` (`"get"`) or `TRANSPORT_SET` (`"set"`). */
export function decodeTransportReply(bytes: Uint8Array, command: "get" | "set"): ProtocolReply<TransportSnapshot> {
  return decodeReply(bytes, command === "get" ? MESSAGE_TRANSPORT_GET : MESSAGE_TRANSPORT_SET, transportBody);
}

/** Decode the reply to `TELEMETRY_CONFIGURE`: the configuration the core applied. */
export function decodeTelemetryReply(bytes: Uint8Array): ProtocolReply<TelemetryConfiguration> {
  return decodeReply(bytes, MESSAGE_TELEMETRY_CONFIGURE, (fields) => Object.freeze({
    meters: Object.freeze(fields.filter((row) => row.id === 7).map(meterEntryOf)),
    meterPeriodBlocks: readU32(fields, 2),
    ...meterWindowOf(fields),
    counterIds: Object.freeze(packed(need(fields, 3, WIRE_PACKED_U32), 4).map(counterName)),
    counterPeriodBlocks: readU32(fields, 4),
    diagnosticsEnabled: readBool(fields, 5),
    minimumSeverity: named(SEVERITY_NAMES, readU8(fields, 6), "severity"),
  }));
}

function meterEntryOf(row: RawField): MeterEntry {
  const inner = nested(row);
  const metric = named(METRIC_NAMES, readU8(inner, 3), "meter metric");
  const target = readU8(inner, 1);
  const strip = only(inner, 2, WIRE_UTF8);
  if (target === 1 && strip === undefined) return Object.freeze({ target: "output", metric });
  if (target === 3 && strip === undefined) {
    if (metric !== "gainReduction") throw malformed("a master meter entry's metric is not gain reduction");
    return Object.freeze({ target: "master", metric });
  }
  if (target === 2 && strip !== undefined) {
    let stripId: string;
    try {
      stripId = DECODER.decode(strip.value);
    } catch {
      throw malformed("a meter entry's strip ID is not UTF-8");
    }
    return Object.freeze({ target: "strip", stripId, metric });
  }
  throw malformed("a meter entry's target and strip ID disagree");
}

function meterWindowOf(fields: readonly RawField[]): { meterWindowBlocks?: number } {
  const window = scalar(fields, 8, WIRE_U32, 4);
  return window === undefined ? {} : { meterWindowBlocks: window.getUint32(0, true) };
}

function counterBody(fields: readonly RawField[]): CounterSnapshot {
  const values = fields.filter((row) => row.id === 2).map((row) => {
    const inner = nested(row);
    return Object.freeze({ id: counterName(readU32(inner, 1)), value: readU64(inner, 2) });
  });
  return Object.freeze({ observedSample: readU64(fields, 1), values: Object.freeze(values) });
}

/** Decode the reply to `COUNTERS_GET`. */
export function decodeCountersReply(bytes: Uint8Array): ProtocolReply<CounterSnapshot> {
  return decodeReply(bytes, MESSAGE_COUNTERS_GET, counterBody);
}

/** Decode the reply to `DIAGNOSTICS_GET`. */
export function decodeDiagnosticsReply(bytes: Uint8Array): ProtocolReply<DiagnosticsPage> {
  return decodeReply(bytes, MESSAGE_DIAGNOSTICS_GET, (fields) => Object.freeze({
    lastSequence: readU64(fields, 1),
    eof: readBool(fields, 2),
    diagnostics: Object.freeze(fields.filter((row) => row.id === 3).map(decodeProtocolDiagnostic)),
  }));
}

function meterRecords(fields: readonly RawField[]): MeterRecord[] {
  const count = readU16(fields, 2);
  if (readU16(fields, 3) !== METER_RECORD_BYTES) throw malformed("a meter record is not 16 bytes");
  const raw = need(fields, 4, WIRE_BYTES).value;
  if (raw.length !== count * METER_RECORD_BYTES) throw malformed("the meter records are ragged");
  const view = new DataView(raw.buffer, raw.byteOffset, raw.byteLength);
  const records: MeterRecord[] = [];
  for (let at = 0; at < raw.length; at += METER_RECORD_BYTES) {
    if (view.getUint32(at + 12, true) !== 0) throw malformed("a meter record's reserved bytes are not zero");
    records.push(Object.freeze({
      handle: view.getUint32(at, true),
      component: named(COMPONENT_NAMES, view.getUint16(at + 4, true), "meter component"),
      flags: view.getUint16(at + 6, true),
      value: view.getFloat32(at + 8, true),
    }));
  }
  return records;
}

/**
 * Decode one event frame the engine encoded, of any of the six events. Bytes that are not a valid
 * event frame raise `MisoEngineError` (`phase: "lifecycle"`, `code: "internal"`).
 */
export function decodeProtocolEvent(frame: Uint8Array): ProtocolEvent {
  if (frame.length < HEADER_BYTES) throw malformed("the event is shorter than its header");
  const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
  for (let index = 0; index < MAGIC.length; index += 1) {
    if (frame[index] !== MAGIC[index]) throw malformed("the magic is wrong");
  }
  if (view.getUint16(8, true) !== 1 || view.getUint16(12, true) !== HEADER_BYTES) {
    throw malformed("the event header is not version 1");
  }
  if (frame[14] !== KIND_EVENT || frame[15] !== 0 || view.getUint16(18, true) !== 0) {
    throw malformed("the frame is not an event");
  }
  if (view.getBigUint64(24, true) !== 0n || view.getUint32(44, true) !== 0) {
    throw malformed("an event carries a request ID or a reserved word");
  }
  if (HEADER_BYTES + view.getUint32(20, true) !== frame.length) throw malformed("the event payload length is wrong");
  const message = view.getUint16(16, true);
  const revision = view.getBigUint64(32, true);
  if (message === EVENT_OBSERVATION_ENDED) {
    const ended = decodeObservationEnded(frame);
    if (ended === undefined) throw malformed("an ended-observation event did not decode");
    return Object.freeze({ kind: "observationEnded", revision, sequence: ended.sequence, keys: ended.keys });
  }
  const fields = readFields(frame.subarray(HEADER_BYTES), view.getUint32(40, true));
  switch (message) {
    case EVENT_SESSION_COMMITTED:
      return Object.freeze({
        kind: "sessionCommitted",
        revision,
        sequence: readU64(fields, 1),
        originRequestId: readU64(fields, 2),
        previousRevision: readU64(fields, 3),
        appliedOperations: readU32(fields, 4),
      });
    case EVENT_TRANSPORT_STATE: {
      const origin = scalar(fields, 5, WIRE_U64, 8);
      return Object.freeze({
        kind: "transportState",
        revision,
        sequence: readU64(fields, 1),
        state: named(TRANSPORT_NAMES, readU8(fields, 2), "transport state"),
        effectiveSample: readU64(fields, 4),
        ...(origin === undefined ? {} : { originRequestId: origin.getBigUint64(0, true) }),
      });
    }
    case EVENT_METER_BATCH:
      return Object.freeze({
        kind: "meterBatch",
        revision,
        observedSample: readU64(fields, 1),
        firstSample: readU64(fields, 5),
        windowSequence: readU64(fields, 6),
        records: Object.freeze(meterRecords(fields)),
      });
    case EVENT_COUNTER_SNAPSHOT:
      return Object.freeze({ kind: "counterSnapshot", revision, ...counterBody(fields) });
    case EVENT_DIAGNOSTIC:
      return Object.freeze({
        kind: "diagnostic",
        revision,
        diagnostic: decodeProtocolDiagnostic(need(fields, 1, WIRE_MESSAGE)),
      });
    default:
      throw malformed(`event ${message} is not registered`);
  }
}

// ----------------------------------------------------------------------------------------------
// The calls.

/** How a `ProtocolClient` reaches one engine's core. */
export interface ProtocolTransport {
  /**
   * Send one command frame and resolve with its reply frame. `document`, when present, is the
   * document a `SESSION_REPLACE` frame names, passed beside the frame (#1386 D7).
   */
  readonly send: (frame: Uint8Array, document?: Uint8Array) => Uint8Array | PromiseLike<Uint8Array>;
  /** The endpoint's one request-ID allocator: the core's replay cache refuses a reused ID. */
  readonly requestIds: ControlRequestIds;
  /** The core's committed revision, which `TRANSPORT_SET` and `TELEMETRY_CONFIGURE` must expect exactly. */
  readonly committedRevision: () => bigint | PromiseLike<bigint>;
}

/** The most bytes one `SESSION_SNAPSHOT_GET` page asks for (D3). */
export const SESSION_SNAPSHOT_PAGE_BYTES = 65_536;
/** How many times `committedDocument` starts again after a revision conflict before it rejects (D3). */
export const SESSION_SNAPSHOT_RESTARTS = 3;
/**
 * A snapshot reply's bytes besides its chunk: the header and the total, offset, chunk prefix and eof
 * fields. A page asks for no more than the core's frame limit leaves for the chunk.
 */
const SNAPSHOT_REPLY_OVERHEAD = HEADER_BYTES + 16 + 16 + 8 + 16;

function refusalError(refusal: ProtocolRefusal): MisoControlRefusal {
  return new MisoControlRefusal({
    code: refusal.diagnostics[0]?.code ?? refusal.status,
    status: refusal.status,
    revision: refusal.revision,
    editIndex: undefined,
    diagnostics: refusal.diagnostics.map((row) => ({
      code: row.code,
      detail: row.detail,
      operationIndex: row.operationIndex,
    })),
  });
}

/**
 * The SDK's refusal of a frame larger than the core accepts (#2190), raised before it is sent: the
 * status the core's codec gives the same frame, `limit_exceeded`, naming the frame's bytes and the
 * limit. `revision` is the revision the frame expected (its header's bytes 32 to 40).
 */
function frameTooLarge(frame: Uint8Array, maximumFrameBytes: bigint): MisoControlRefusal {
  const expected = new DataView(frame.buffer, frame.byteOffset, frame.byteLength).getBigUint64(32, true);
  return new MisoControlRefusal({
    code: "limit_exceeded",
    status: "limit_exceeded",
    revision: expected,
    editIndex: undefined,
    diagnostics: [{
      code: "limit_exceeded",
      detail: `frame_bytes=${frame.length},maximum_frame_bytes=${maximumFrameBytes}`,
      operationIndex: undefined,
    }],
    message: `a control frame of ${frame.length} bytes is over the core's frame limit of ${maximumFrameBytes} bytes `
      + "(limit_exceeded); it was not sent",
  });
}

/**
 * The calls of #1571 D3 over one engine's transport. Both engines make them through this class, so
 * they send the same frames and decode the same replies. A call the core refuses rejects with
 * `MisoControlRefusal`, its `code` the core's first diagnostic code or, with none, the status name;
 * a reply that is not valid rejects with `MisoEngineError` `internal`. Every frame but the first
 * `CAPABILITIES_GET` is checked against the core's `maximumFrameBytes` before it is sent, and a
 * larger one rejects with `MisoControlRefusal` `limit_exceeded` and is not sent (#2190); the first
 * call that needs the limit reads the capabilities once.
 */
export class ProtocolClient {
  readonly #transport: ProtocolTransport;
  #capabilities: EngineCapabilities | undefined;

  constructor(transport: ProtocolTransport) {
    this.#transport = transport;
  }

  /**
   * Apply one structural transaction (`SESSION_TRANSACTION_APPLY`) at the core's committed revision
   * and resolve with the committed revision and the path the core chose (#1389). A refusal rejects
   * with `MisoControlRefusal`, and nothing is committed. There is no retry: the transaction was
   * built against one session, and a conflict means the session moved under it.
   */
  async applyTransaction(transaction: SessionTransaction): Promise<EditOutcome> {
    const limit = await this.#frameLimit();
    const expectedRevision = await this.#transport.committedRevision();
    const requestId = this.#transport.requestIds.next();
    const reply = decodeControlReply(await this.#send(
      encodeSessionTransaction(transaction, { requestId, expectedRevision }),
      limit,
    ));
    if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
    return editOutcome(reply);
  }

  /**
   * Replace the session with `document` (`SESSION_REPLACE`, #1386): the core diffs it against the
   * committed model into one transaction. The frame names the document by its length and SHA-256
   * and the transport passes the bytes beside it. Resolves with the committed revision and the path,
   * `unchanged` when the document equals the committed model; a refusal rejects with
   * `MisoControlRefusal`, and nothing is committed.
   */
  async replaceSession(document: Uint8Array<ArrayBuffer>): Promise<EditOutcome> {
    if (!(document instanceof Uint8Array) || document.length === 0) {
      throw new MisoUsageError("a replacement document must be non-empty bytes");
    }
    const digest = await documentSha256(document);
    const limit = await this.#frameLimit();
    // Read after the digest, right before the send: the revision the frame expects is current.
    const expectedRevision = await this.#transport.committedRevision();
    const requestId = this.#transport.requestIds.next();
    const frame = encodeSessionReplaceRequest({
      requestId,
      expectedRevision,
      documentBytes: document.length,
      documentSha256: digest,
    });
    const reply = decodeAppliedReply(await this.#send(frame, limit, document), MESSAGE_SESSION_REPLACE);
    if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
    return editOutcome(reply);
  }

  /** The core's frame limit, read once with `CAPABILITIES_GET` before the first frame that needs it. */
  async #frameLimit(): Promise<bigint> {
    return (this.#capabilities ?? await this.capabilities()).maximumFrameBytes;
  }

  /** Send `frame` if it fits the core's frame limit; refuse it, unsent, if it does not. */
  #send(frame: Uint8Array, limit: bigint | undefined, document?: Uint8Array): Uint8Array | PromiseLike<Uint8Array> {
    if (limit !== undefined && BigInt(frame.length) > limit) throw frameTooLarge(frame, limit);
    return document === undefined ? this.#transport.send(frame) : this.#transport.send(frame, document);
  }

  /**
   * Send one call. `checked: false` is only `CAPABILITIES_GET`, the call that reads the limit: its
   * frame is a header and no fields, smaller than any limit a core reports.
   */
  async #call<T>(
    encode: (requestId: bigint) => Uint8Array,
    decode: (bytes: Uint8Array) => ProtocolReply<T>,
    checked = true,
  ): Promise<ProtocolReply<T>> {
    const limit = checked ? await this.#frameLimit() : undefined;
    const requestId = this.#transport.requestIds.next();
    const reply = decode(await this.#send(encode(requestId), limit));
    if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
    return reply;
  }

  async #value<T>(
    encode: (requestId: bigint) => Uint8Array,
    decode: (bytes: Uint8Array) => ProtocolReply<T>,
    checked = true,
  ): Promise<T> {
    const reply = await this.#call(encode, decode, checked);
    if (!reply.ok) throw refusalError(reply);
    return reply.value;
  }

  /** The core's capabilities (`CAPABILITIES_GET`). */
  async capabilities(): Promise<EngineCapabilities> {
    const value = await this.#value((requestId) => encodeCapabilitiesRequest({ requestId }), decodeCapabilitiesReply, false);
    this.#capabilities = value;
    return value;
  }

  /**
   * The canonical committed document and its revision, read in `SESSION_SNAPSHOT_GET` pages of at
   * most 65,536 bytes and no more than the core's frame limit leaves. The first page takes any
   * revision; each next page names the first page's. A revision conflict (the session changed
   * between pages) starts again at offset 0, at most three times, and then rejects with the core's
   * code. The bytes are decoded as UTF-8 only once they are all read, since a page may end inside a
   * code point.
   */
  async committedDocument(): Promise<CommittedDocument> {
    const capabilities = this.#capabilities ?? await this.capabilities();
    const room = capabilities.maximumFrameBytes - BigInt(SNAPSHOT_REPLY_OVERHEAD);
    const pageBytes = room >= BigInt(SESSION_SNAPSHOT_PAGE_BYTES)
      ? SESSION_SNAPSHOT_PAGE_BYTES
      : Math.max(1, Number(room) & ~7);
    for (let restarts = 0; ; restarts += 1) {
      let revision: bigint | undefined;
      let total: bigint | undefined;
      let offset = 0n;
      const pages: Uint8Array[] = [];
      let conflict: ProtocolRefusal | undefined;
      for (;;) {
        const expected = revision;
        const reply = await this.#call(
          (requestId) => encodeSessionSnapshotRequest({
            requestId,
            offset,
            maximumBytes: pageBytes,
            ...(expected === undefined ? {} : { expectedRevision: expected }),
          }),
          decodeSessionSnapshotReply,
        );
        if (!reply.ok) {
          if (reply.status === "revision_conflict" && expected !== undefined) {
            conflict = reply;
            break;
          }
          throw refusalError(reply);
        }
        const page = reply.value;
        revision ??= reply.revision;
        total ??= page.totalBytes;
        if (page.offset !== offset || page.totalBytes !== total) throw malformed("a snapshot page is out of place");
        pages.push(page.chunk);
        offset += BigInt(page.chunk.length);
        if (page.eof) {
          if (offset !== total) throw malformed("the snapshot ended before its total");
          let document: string;
          try {
            document = DECODER.decode(concat(pages));
          } catch {
            throw malformed("the committed document is not UTF-8");
          }
          return Object.freeze({ revision, document });
        }
        if (page.chunk.length === 0) throw malformed("a snapshot page before the end is empty");
      }
      // The inner loop leaves only through a conflict.
      if (restarts >= SESSION_SNAPSHOT_RESTARTS) throw refusalError(conflict as ProtocolRefusal);
    }
  }

  /** The transport (`TRANSPORT_GET`). */
  transport(): Promise<TransportSnapshot> {
    return this.#value((requestId) => encodeTransportGetRequest({ requestId }), (bytes) => decodeTransportReply(bytes, "get"));
  }

  /**
   * Set the transport's state (`TRANSPORT_SET`) at the core's committed revision. The stored
   * position is retired (#1635): a `position` has no wire field, so it is refused here, not dropped.
   */
  async setTransport(request: TransportRequest): Promise<TransportSnapshot> {
    if (typeof request !== "object" || request === null) throw new MisoUsageError("a transport request must be an object");
    if ("position" in request) {
      throw new MisoUsageError("the transport has no stored position (#1635): seek moves the playhead");
    }
    // The limit first: a first call's CAPABILITIES_GET must not fall between the revision read and
    // the send, where it would widen the `revision_conflict` window.
    await this.#frameLimit();
    const expectedRevision = await this.#transport.committedRevision();
    return this.#value(
      (requestId) => encodeTransportSetRequest({ requestId, expectedRevision, state: request.state }),
      (bytes) => decodeTransportReply(bytes, "set"),
    );
  }

  /** Replace the endpoint's telemetry configuration (`TELEMETRY_CONFIGURE`); resolves to what the core applied. */
  async configureTelemetry(configuration: TelemetryConfiguration): Promise<TelemetryConfiguration> {
    await this.#frameLimit();
    const expectedRevision = await this.#transport.committedRevision();
    return this.#value(
      (requestId) => encodeTelemetryConfigureRequest({ requestId, expectedRevision, configuration }),
      decodeTelemetryReply,
    );
  }

  /** The named counters, or every counter when `ids` is absent (`COUNTERS_GET`). */
  counters(ids?: readonly CounterName[]): Promise<CounterSnapshot> {
    return this.#value(
      (requestId) => encodeCountersRequest(ids === undefined ? { requestId } : { requestId, ids }),
      decodeCountersReply,
    );
  }

  /** One page of the retained diagnostics (`DIAGNOSTICS_GET`); defaults `0n`, 64 and `"info"`. */
  diagnostics(query: DiagnosticsQuery = {}): Promise<DiagnosticsPage> {
    return this.#value(
      (requestId) => encodeDiagnosticsRequest({
        requestId,
        afterSequence: query.afterSequence ?? 0n,
        limit: query.limit ?? 64,
        minimumSeverity: query.minimumSeverity ?? "info",
      }),
      decodeDiagnosticsReply,
    );
  }
}
