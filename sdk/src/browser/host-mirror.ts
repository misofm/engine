import { refuseRetiredObservationTaps } from "../internal/retired-options.ts";
import type { BootOptions } from "../core/abi.ts";
import { MisoUsageError } from "./../core/errors.ts";
import type { MisoWebBootOptions } from "./shipped-host.d.ts";

/**
 * The bridge from the SDK's `BootOptions` to the shipped host factory's `MisoWebBootOptions`.
 *
 * # Why there are two shapes at all
 *
 * The shipped `.d.ts` predates this SDK and is the app's direct interface to the worklet host: it
 * is flat, every field is required, and the two live-control-adjacent words are `bigint` because
 * that is what the wire takes. `BootOptions` is the SDK's: every field optional, absent meaning
 * zero meaning *the engine's* default, and the three live-control words grouped, because a caller
 * who wants no live controls should be able to say nothing rather than write three zeroes.
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

  const liveControls = options.liveControls;
  // Issue #1689: the retired master boot word is refused, never ignored, for an untyped caller.
  if ((liveControls as { masterTrackPlusOne?: unknown } | undefined)?.masterTrackPlusOne !== undefined) {
    throw new MisoUsageError(
      "liveControls.masterTrackPlusOne was retired (issue #1689): designate the master after boot with strip.master(true)",
    );
  }
  refuseRetiredObservationTaps(liveControls);

  return {
    sourceRingFrames: nonNegative("sourceRingFrames", options.sourceRingFrames ?? 0),
    maximumMemoryBytes: options.maximumMemoryBytes ?? 0n,
    maximumControlFrameBytes: 0n,
    liveControlMeterBlocks: word("liveControls.meterBlocks", liveControls?.meterBlocks),
    maximumDocumentBytes: 0n,
    reserved2: 0n,
  };
}

