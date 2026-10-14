import { MisoUsageError } from "../core/errors.ts";
import type { LaneEdit } from "../core/writer.ts";

/**
 * Internal word rules of the observation command record. This module is deliberately absent from
 * every package export map and public barrel.
 *
 * Value edits carry a ramp as `rampSamples` of a `SET_CONTROL` edit (`control-frame.ts`); the
 * session-default marker of the retired 48-byte value records is gone with them, and the core
 * resolves an absent ramp. What is left of a {@link LaneEdit} is the observation records (kinds 7
 * and 8), whose `smoothingSamples` word is the subscription window in blocks.
 */

/** A `u32`, as `DataView.setUint32` needs it: it would wrap `-1` and truncate `1.5`. */
export function requireU32(samples: number, name = "smoothingSamples"): number {
  if (!Number.isSafeInteger(samples) || samples < 0 || samples > 0xffff_ffff) {
    throw new MisoUsageError(`${name} must be a u32`);
  }
  return samples;
}

/** The `smoothingSamples` word an observation record puts on the wire: its window, or `0`. */
export function laneEditWord(edit: LaneEdit): number {
  return requireU32(edit.smoothingSamples ?? 0);
}
