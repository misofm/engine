import { CONTROLS } from "../generated/controls.ts";
import {
  DECODER,
  WIRE_MESSAGE,
  WIRE_U32,
  WIRE_U64,
  WIRE_U8,
  WIRE_UTF8,
  malformed,
  nestedFields,
  readFields,
  scalar,
} from "./control-frame.ts";

/**
 * The reliable `OBSERVATION_ENDED` event (`8011`, #1584 D6a): one per commit that ended host-local
 * observation subscriptions, carrying the stable key of every ended tap. The core decides what
 * ended; this file only reads the frame it encoded.
 *
 * The wire is `docs/CONTROL_PROTOCOL_REGISTRY.md`'s: the 48-byte event header (kind 3, message
 * `8011`, request ID zero, the committing revision), then `sequence` (field 1, u64) and one repeated
 * `key` message (field 2) of strip ID (1), section (2, the control section code: inserts or console), the insert's or console
 * slot's stable ID (3) and the effect-local tap ID (4).
 */
const HEADER_BYTES = 48;
const KIND_EVENT = 3;
const MESSAGE_OBSERVATION_ENDED = 0x8011;

/** One observation tap the core ended, by the stable IDs that name it. */
export interface ObservationEndedKey {
  /** The strip's ID, a track or a submix. */
  readonly stripId: string;
  readonly rack: "console" | "inserts";
  /** The console slot's ID or the insert's ID. */
  readonly effectSlotId: string;
  /** The effect-local tap ID. */
  readonly tapId: number;
}

/** What one `OBSERVATION_ENDED` event says. */
export interface ObservationEnded {
  /** The event's own sequence on the reliable lane. */
  readonly sequence: bigint;
  /** The revision of the commit that ended the taps. */
  readonly revision: bigint;
  readonly keys: readonly ObservationEndedKey[];
}

function rackName(code: number): "console" | "inserts" {
  const row = CONTROLS.address.sectionKinds.find((candidate) => candidate.code === code);
  if (row?.name !== "console" && row?.name !== "inserts") {
    throw malformed(`an ended tap names section ${code}, which is neither the console nor the inserts`);
  }
  return row.name;
}

/**
 * Read an event frame the engine encoded. Returns `undefined` for any event that is not
 * `OBSERVATION_ENDED`, and raises `MisoEngineError` (`internal`) for an `OBSERVATION_ENDED` frame
 * that is not valid.
 */
export function decodeObservationEnded(frame: Uint8Array): ObservationEnded | undefined {
  if (frame.length < HEADER_BYTES) return undefined;
  const view = new DataView(frame.buffer, frame.byteOffset, frame.byteLength);
  if (frame[14] !== KIND_EVENT || view.getUint16(16, true) !== MESSAGE_OBSERVATION_ENDED) return undefined;
  if (HEADER_BYTES + view.getUint32(20, true) !== frame.length) throw malformed("the event payload length is wrong");
  const revision = view.getBigUint64(32, true);
  const fields = readFields(frame.subarray(HEADER_BYTES), view.getUint32(40, true));
  const sequence = scalar(fields, 1, WIRE_U64, 8)?.getBigUint64(0, true);
  if (sequence === undefined) throw malformed("an ended-observation event has no sequence");
  const keys = fields.filter((row) => row.id === 2).map((row) => {
    if (row.wire !== WIRE_MESSAGE) throw malformed("an ended tap is not a message");
    const inner = nestedFields(row.value);
    const strip = inner.find((candidate) => candidate.id === 1);
    const slot = inner.find((candidate) => candidate.id === 3);
    const section = scalar(inner, 2, WIRE_U8, 1)?.getUint8(0);
    const tapId = scalar(inner, 4, WIRE_U32, 4)?.getUint32(0, true);
    if (strip?.wire !== WIRE_UTF8 || slot?.wire !== WIRE_UTF8 || section === undefined || tapId === undefined) {
      throw malformed("an ended tap is incomplete");
    }
    return Object.freeze({
      stripId: DECODER.decode(strip.value),
      rack: rackName(section),
      effectSlotId: DECODER.decode(slot.value),
      tapId,
    });
  });
  if (keys.length === 0) throw malformed("an ended-observation event has no key");
  return Object.freeze({ sequence, revision, keys: Object.freeze(keys) });
}
