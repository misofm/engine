import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BootOptions } from "../core/abi.ts";
import { MisoUsageError } from "./../core/errors.ts";
import type { MisoWebBootOptions } from "./shipped-host.d.ts";

/**
 * The bridge from the SDK's `BootOptions` to the shipped host factory's `MisoWebBootOptions`.
 *
 * # Why there are two shapes at all
 *
 * The shipped `.d.ts` predates this SDK and is the app's direct interface to the worklet host: it
 * is flat, every field is required, and the two console-adjacent words are `bigint` because that is
 * what the wire takes. `BootOptions` is the SDK's: every field optional, absent meaning zero
 * meaning *the engine's* default, and the four console words grouped, because a caller who wants
 * no console should be able to say nothing rather than write four zeroes.
 *
 * Both are right for their callers, and neither should be bent to match the other. What must not
 * happen is a third, hand-written copy of the mapping between them appearing in an app -- which is
 * exactly the shape of the drift N-13(d) found five times over. So the mapping lives here, once,
 * and `sdk/test/host-mirror.ts` pins it against the shipped declaration at typecheck time: if the
 * shipped `MisoWebBootOptions` gains, loses or retypes a field, this file stops compiling.
 *
 * # Why the shipped declaration is vendored as a `.d.ts` rather than imported from the artifact
 *
 * The artifact directory is a build output that does not exist in a source checkout, so a type
 * import from it could not be part of `tsc --noEmit`. `shipped-host.d.ts` is a copy, and
 * `scripts/check-sdk-generated.sh` proves it is byte-identical to
 * `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` -- so it is a mirror the
 * way the generated modules are mirrors: checked, not trusted.
 */
export function toWebBootOptions(options: BootOptions): MisoWebBootOptions {
  const nonNegative = (name: string, value: number): number => {
    if (!Number.isInteger(value) || value < 0) {
      throw new MisoUsageError(`${name} must be a non-negative integer, got ${value}`);
    }
    return value;
  };
  const word = (name: string, value: number | undefined): bigint =>
    BigInt(nonNegative(name, value ?? 0));

  const console = options.console;
  const observationTaps = word("console.observationTaps", console?.observationTaps);
  const commandQueueRecords = word("console.commandQueueRecords", console?.commandQueueRecords);
  const masterTrackPlusOne = word("console.masterTrackPlusOne", console?.masterTrackPlusOne);

  // The engine refuses these combinations at boot with `web.options.console`. Catching them here
  // turns a boot-time refusal into a caller-time message that says which pair is inconsistent,
  // without duplicating the engine's authority: the engine still refuses if this check is wrong.
  if (observationTaps !== 0n && commandQueueRecords === 0n) {
    throw new MisoUsageError(
      "observation taps ride the effect's own command queue, so consoleObservationTaps requires "
      + "consoleCommandQueueRecords",
    );
  }
  if (masterTrackPlusOne !== 0n && observationTaps === 0n) {
    throw new MisoUsageError(
      "a designated master track is a designation over observed effects, so "
      + "consoleMasterTrackPlusOne requires consoleObservationTaps",
    );
  }
  if (observationTaps > BigInt(ABI_LAYOUT.constants.maximumObservationTaps)) {
    throw new MisoUsageError(
      `consoleObservationTaps is capped at ${ABI_LAYOUT.constants.maximumObservationTaps}`,
    );
  }
  if (commandQueueRecords > BigInt(ABI_LAYOUT.constants.maximumCommandRecords)) {
    throw new MisoUsageError(
      `consoleCommandQueueRecords is capped at ${ABI_LAYOUT.constants.maximumCommandRecords}`,
    );
  }

  return {
    sourceRingFrames: nonNegative("sourceRingFrames", options.sourceRingFrames ?? 0),
    maximumMemoryBytes: options.maximumMemoryBytes ?? 0n,
    consoleCommandQueueRecords: commandQueueRecords,
    consoleMeterBlocks: word("console.meterBlocks", console?.meterBlocks),
    consoleObservationTaps: observationTaps,
    consoleMasterTrackPlusOne: masterTrackPlusOne,
  };
}
