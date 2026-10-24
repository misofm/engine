import { MisoUsageError } from "../core/errors.ts";
import type { LaneEdit } from "../core/writer.ts";
import { requireRack } from "./rack.ts";
import { requireU32 } from "./ramp.ts";

/**
 * Internal encodability rules of the observation command record. This module is deliberately absent
 * from every package export map and public barrel.
 */

const ID_ENCODER = new TextEncoder();
const ID_FIELD_BYTES = 128;

/** One record ID's UTF-8 bytes; the field holds 127 of them and a zero after the last. */
function recordId(id: string, what: string): void {
  if (typeof id !== "string") throw new MisoUsageError(`${what} must be a string`);
  const bytes = ID_ENCODER.encode(id).length;
  if (bytes >= ID_FIELD_BYTES) {
    throw new MisoUsageError(`${what} is ${bytes} bytes; a record holds at most ${ID_FIELD_BYTES - 1}`);
  }
}

/**
 * Check that an observation edit can be encoded and names a rack the record may carry; the core
 * judges what it names. Both SDK transports call it before the host sees the edit (#1542).
 */
export function checkLaneEdit(edit: LaneEdit): void {
  requireRack(edit.rack);
  recordId(edit.stripId, "an observation record's strip ID");
  recordId(edit.effectId, "an observation record's effect ID");
  requireU32(edit.tapId, "tapId");
  requireU32(edit.windowBlocks, "windowBlocks");
}
