import type { CommandKindName } from "../generated/catalog.ts";
import { MisoUsageError } from "../core/errors.ts";
import type { LaneEdit } from "../core/writer.ts";

/**
 * Internal ramp-length rules shared by the live-control builders, `encodeLaneEdits` and the
 * browser transport. This module is deliberately absent from every package export map and public
 * barrel: `core/writer.ts` re-exports {@link SESSION_DEFAULT_RAMP} (the one name callers see), and
 * the two helpers below are the encoders' single source of truth, not public API.
 */

/**
 * The `smoothingSamples` word of a command record that means "no length given" (`u32::MAX`).
 *
 * A ramped edit built without `smoothingSamples` carries this word, and the engine replaces it
 * with the session's own length for that edit's row: the session's `controlSmoothing` keys, or the
 * default table (decision 15 D15-1). It is not a length, so it is never accepted as one.
 */
export const SESSION_DEFAULT_RAMP = 0xffff_ffff;

const RAMPED_KINDS: ReadonlySet<CommandKindName> = new Set<CommandKindName>([
  "pan", "matrix", "faderDb", "mute", "solo", "trimDb", "polarityInvert",
  "routeGainDb", "routeMute", "routeMatrix", "vcaFaderDb", "vcaMute",
]);

/**
 * Check an explicit ramp length and pass an absent one through. `u32::MAX` is the marker, never a
 * length, and a non-`u32` is no length either: both throw a `MisoUsageError`. The builders call
 * this once per edit and leave an absent length absent; the encoders resolve it by kind
 * ({@link laneEditSmoothingWord}), so there is one place an absent length becomes a word.
 */
export function checkedRampLength(samples: number | undefined): number | undefined {
  if (samples === undefined) return undefined;
  if (!Number.isSafeInteger(samples) || samples < 0 || samples > 0xffff_ffff) {
    throw new MisoUsageError("smoothingSamples must be a u32");
  }
  if (samples === SESSION_DEFAULT_RAMP) {
    throw new MisoUsageError(
      "smoothingSamples must be a u32 below 4294967295: 4294967295 is the session-default marker, not a length",
    );
  }
  return samples;
}

/**
 * The `smoothingSamples` word a literal {@link LaneEdit} puts on the wire, by its kind: the
 * ramped kinds resolve an absent length to the session default; every other kind writes `0`.
 * The one rule behind `encodeLaneEdits` and the browser transport.
 */
export function laneEditSmoothingWord(edit: LaneEdit): number {
  if (RAMPED_KINDS.has(edit.kind)) {
    return checkedRampLength(edit.smoothingSamples) ?? SESSION_DEFAULT_RAMP;
  }
  return edit.smoothingSamples ?? 0;
}
