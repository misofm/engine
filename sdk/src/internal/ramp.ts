import { MisoUsageError } from "../core/errors.ts";

/**
 * Internal word rules of the observation command record. This module is deliberately absent from
 * every package export map and public barrel.
 *
 * Value edits carry a ramp as `rampSamples` of a `SET_CONTROL` edit (`control-frame.ts`); the
 * session-default marker of the retired 48-byte value records is gone with them, and the core
 * resolves an absent ramp.
 */

/** A `u32`, as `DataView.setUint32` needs it: it would wrap `-1` and truncate `1.5`. */
export function requireU32(samples: number, name = "smoothingSamples"): number {
  if (!Number.isSafeInteger(samples) || samples < 0 || samples > 0xffff_ffff) {
    throw new MisoUsageError(`${name} must be a u32`);
  }
  return samples;
}
