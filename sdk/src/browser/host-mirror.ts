import { ABI_LAYOUT } from "../generated/abi.ts";
import type { BootOptions } from "../core/abi.ts";
import { MisoUsageError } from "./../core/errors.ts";
import type { MisoWebBootOptions } from "./shipped-host.d.ts";
import type { SpectrumCollection, SpectrumQuery } from "../core/spectrum.ts";
import { validateBrowserSpectrumHopFrames } from "./policy.ts";

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
  const observationTaps = word("liveControls.observationTaps", liveControls?.observationTaps);
  const spectrumHopFrames = validateBrowserSpectrumHopFrames(options.spectrumHopFrames) ?? 0;

  // The engine refuses this at boot with `web.options.live_controls`. Catching it here turns a
  // boot-time refusal into a caller-time message, without duplicating the engine's authority: the
  // engine still refuses if this check is wrong.
  if (observationTaps > BigInt(ABI_LAYOUT.constants.maximumObservationTaps)) {
    throw new MisoUsageError(
      `liveControlObservationTaps is capped at ${ABI_LAYOUT.constants.maximumObservationTaps}`,
    );
  }

  let spectrum: MisoWebBootOptions["spectrum"] = null;
  if (options.spectrum !== undefined) {
    spectrum = toWebSpectrumOptions(options.spectrum);
  }
  let spectrumCollection: MisoWebBootOptions["spectrumCollection"] = null;
  if (options.spectrumCollection !== undefined) {
    spectrumCollection = toWebSpectrumCollectionOptions(options.spectrumCollection);
  }

  return {
    sourceRingFrames: nonNegative("sourceRingFrames", options.sourceRingFrames ?? 0),
    maximumMemoryBytes: options.maximumMemoryBytes ?? 0n,
    reserved1: 0n,
    liveControlMeterBlocks: word("liveControls.meterBlocks", liveControls?.meterBlocks),
    liveControlObservationTaps: observationTaps,
    reserved2: 0n,
    spectrumHopFrames,
    spectrum,
    spectrumCollection,
  };
}

function toWebSpectrumOptions(query: SpectrumQuery): NonNullable<MisoWebBootOptions["spectrum"]> {
  const id = query.target.kind === "output" ? query.target.outputId : query.target.trackId;
  if (typeof id !== "string" || id.length === 0) {
    throw new MisoUsageError("spectrum target identity must be a nonempty string");
  }
  const idBytes = new TextEncoder().encode(id).byteLength;
  const maximumIdBytes = ABI_LAYOUT.constants.maximumSpectrumIdBytes;
  if (idBytes > maximumIdBytes) {
    throw new MisoUsageError("spectrum target identity exceeds its bound");
  }
  const channels = query.channels ?? "both";
  if (channels !== "left" && channels !== "right" && channels !== "both") {
    throw new MisoUsageError("spectrum channels must be left, right, or both");
  }
  const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes
    ?? ABI_LAYOUT.constants.spectrumCaptureBytes;
  if (!Number.isSafeInteger(maximumCaptureBytes) || maximumCaptureBytes < 1
      || maximumCaptureBytes > ABI_LAYOUT.constants.spectrumCaptureBytes) {
    throw new MisoUsageError(
      `maximumCaptureBytes must be an integer in 1..=${ABI_LAYOUT.constants.spectrumCaptureBytes}`,
    );
  }
  return {
    target: query.target.kind,
    targetId: id,
    channels,
    maximumCaptureBytes,
  };
}

function toWebSpectrumCollectionOptions(
  collection: SpectrumCollection,
): NonNullable<MisoWebBootOptions["spectrumCollection"]> {
  if (!Number.isSafeInteger(collection.maximumCaptureBytes) || collection.maximumCaptureBytes < 1) {
    throw new MisoUsageError("spectrum collection maximumCaptureBytes must be a positive safe integer");
  }
  if (!Array.isArray(collection.entries) || collection.entries.length === 0) {
    throw new MisoUsageError("spectrum collection entries must be a nonempty array");
  }
  const entries = collection.entries.map((entry, index) => {
    const id = entry.target.kind === "output" ? entry.target.outputId : entry.target.trackId;
    if (typeof id !== "string" || id.length === 0) {
      throw new MisoUsageError(`spectrum collection entry ${index} has an empty target identity`);
    }
    if (new TextEncoder().encode(id).byteLength > ABI_LAYOUT.constants.maximumSpectrumIdBytes) {
      throw new MisoUsageError(`spectrum collection entry ${index} target identity exceeds its bound`);
    }
    const channels = entry.channels ?? "both";
    if (channels !== "left" && channels !== "right" && channels !== "both") {
      throw new MisoUsageError(`spectrum collection entry ${index} has invalid channels`);
    }
    return {
      target: entry.target.kind,
      targetId: id,
      channels,
    };
  });
  return { entries, maximumCaptureBytes: collection.maximumCaptureBytes };
}
