import {
  FLAG_REVISION_ANY,
  WIRE_BOOL,
  WIRE_F32,
  WIRE_U64,
  WIRE_U8,
  commandFrame,
  decodeRefusalFields,
  decodeResponseEnvelope,
  field,
  malformed,
  requireU64,
  scalar,
  u8,
} from "./control-frame.ts";
import type { RawField } from "./control-frame.ts";
import { MisoUsageError } from "./errors.ts";
import type { SpectrumFrameRefusal } from "./spectrum-frame.ts";

/**
 * The SDK's codec of the `LOUDNESS` frame (`0x0014`, #2115): one operation on the session's output
 * loudness meter, and the reply that carries the meter's state after it. It is the frame a C ABI
 * host sends to the same `SessionState::command`.
 *
 * # The wire it writes
 *
 * MISO Control BTLV v1, as `control-frame.ts` writes it. The field numbers, wire types and mandatory
 * flags are the protocol schema's (`crates/protocol/src/schema.rs`: `loudness_request`,
 * `loudness_reply`; `crates/protocol/src/message_wire/loudness.rs` owns the rules). The command
 * takes any revision. The request is `1:op U8 R` (start 1, read 2, reset 3, stop 4).
 *
 * # What the decoder refuses
 *
 * A reply that breaks a rule the Rust codec holds on both encode and decode: a level that is not
 * finite or is below {@link LOUDNESS_FRAME_LEVEL_FLOOR}, a negative loudness range, one half of a
 * true-peak pair, or a true peak since reset without `integrating-since` (or the reverse). A field
 * the schema does not hold is refused too.
 */

/** @internal */
export const MESSAGE_LOUDNESS = 0x0014;

/** The lowest level a `LOUDNESS` reply carries (`protocol::LOUDNESS_LEVEL_FLOOR`). @internal */
export const LOUDNESS_FRAME_LEVEL_FLOOR = -120;

/** One `LOUDNESS` operation. @internal */
export type LoudnessFrameOperation = "start" | "read" | "reset" | "stop";

const OPERATION_CODES: Readonly<Record<LoudnessFrameOperation, number>> = Object.freeze({
  start: 1,
  read: 2,
  reset: 3,
  stop: 4,
});

/** The success payload of a `LOUDNESS` reply, field for field. @internal */
export interface LoudnessFrameState {
  readonly observedSample: bigint;
  readonly armed: boolean;
  readonly integratingSince: bigint | undefined;
  readonly hopsDropped: bigint;
  readonly sanitizedSamples: bigint;
  readonly gatedBlockCount: bigint;
  readonly momentaryLufs: number | undefined;
  readonly shortTermLufs: number | undefined;
  readonly integratedLufs: number | undefined;
  readonly loudnessRangeLu: number | undefined;
  readonly momentaryMaxLufs: number | undefined;
  readonly shortTermMaxLufs: number | undefined;
  readonly truePeakDbtp: readonly [number, number] | undefined;
  readonly hopTruePeakDbtp: readonly [number, number] | undefined;
}

/** A decoded `LOUDNESS` reply; a refusal has the shape every session command's refusal has. @internal */
export type LoudnessFrameReply =
  | { readonly ok: true; readonly requestId: bigint; readonly revision: bigint; readonly value: LoudnessFrameState }
  | SpectrumFrameRefusal;

/**
 * Encode one `LOUDNESS` command frame. It takes any revision. Throws `MisoUsageError` before any
 * byte for an operation outside the table. @internal
 */
export function encodeLoudnessCommand(requestId: bigint, operation: LoudnessFrameOperation): Uint8Array {
  const id = requireU64(requestId, "requestId");
  if (id === 0n) throw new MisoUsageError("requestId must not be zero");
  const code = Object.hasOwn(OPERATION_CODES, operation) ? OPERATION_CODES[operation] : undefined;
  if (code === undefined) throw new MisoUsageError("a loudness operation must be start, read, reset or stop");
  const body = field(1, WIRE_U8, true, u8(code));
  return commandFrame(MESSAGE_LOUDNESS, FLAG_REVISION_ANY, id, 0n, 1, body);
}

/** The highest field number of the reply (`16:hop-true-peak-right-dbtp`). */
const LAST_REPLY_FIELD = 16;

function word(fields: readonly RawField[], id: number): bigint {
  const value = scalar(fields, id, WIRE_U64, 8);
  if (value === undefined) throw malformed(`field ${id} is missing`);
  return value.getBigUint64(0, true);
}

function level(fields: readonly RawField[], id: number): number | undefined {
  const value = scalar(fields, id, WIRE_F32, 4)?.getFloat32(0, true);
  if (value !== undefined && !(Number.isFinite(value) && value >= LOUDNESS_FRAME_LEVEL_FLOOR)) {
    throw malformed(`field ${id} is not a level at or above ${LOUDNESS_FRAME_LEVEL_FLOOR}`);
  }
  return value;
}

function pair(fields: readonly RawField[], left: number, right: number): readonly [number, number] | undefined {
  const l = level(fields, left);
  const r = level(fields, right);
  if (l === undefined && r === undefined) return undefined;
  if (l === undefined || r === undefined) throw malformed(`fields ${left} and ${right} are not present together`);
  return Object.freeze([l, r] as const);
}

function state(fields: readonly RawField[]): LoudnessFrameState {
  if (fields.some((row) => row.id > LAST_REPLY_FIELD)) throw malformed("a loudness reply carries an unknown field");
  const armed = scalar(fields, 2, WIRE_BOOL, 1)?.getUint8(0);
  if (armed === undefined || armed > 1) throw malformed("a loudness reply has no armed flag");
  const integratingSince = scalar(fields, 3, WIRE_U64, 8)?.getBigUint64(0, true);
  const loudnessRangeLu = scalar(fields, 10, WIRE_F32, 4)?.getFloat32(0, true);
  if (loudnessRangeLu !== undefined && !(Number.isFinite(loudnessRangeLu) && loudnessRangeLu >= 0)) {
    throw malformed("a loudness range is not finite and at least 0");
  }
  const truePeakDbtp = pair(fields, 13, 14);
  if ((truePeakDbtp === undefined) !== (integratingSince === undefined)) {
    throw malformed("a true peak since reset is present exactly with integrating-since");
  }
  return Object.freeze({
    observedSample: word(fields, 1),
    armed: armed === 1,
    integratingSince,
    hopsDropped: word(fields, 4),
    sanitizedSamples: word(fields, 5),
    gatedBlockCount: word(fields, 6),
    momentaryLufs: level(fields, 7),
    shortTermLufs: level(fields, 8),
    integratedLufs: level(fields, 9),
    loudnessRangeLu,
    momentaryMaxLufs: level(fields, 11),
    shortTermMaxLufs: level(fields, 12),
    truePeakDbtp,
    hopTruePeakDbtp: pair(fields, 15, 16),
  });
}

/** Decode the reply to a `LOUDNESS`. @internal */
export function decodeLoudnessReply(bytes: Uint8Array): LoudnessFrameReply {
  const { requestId, status, revision, fields } = decodeResponseEnvelope(bytes, MESSAGE_LOUDNESS);
  if (status === "ok") return Object.freeze({ ok: true, requestId, revision, value: state(fields) });
  const { diagnostics, omittedDiagnostics, backpressure } = decodeRefusalFields(fields);
  return Object.freeze({ ok: false, requestId, revision, status, diagnostics, omittedDiagnostics, backpressure });
}
