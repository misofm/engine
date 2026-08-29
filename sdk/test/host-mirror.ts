/**
 * Issue #243 eval 6, second half: the SDK's host mirror is pinned to the shipped declaration.
 *
 * This file contains no runtime code and runs no assertions. It is checked by `tsc --noEmit`, and
 * its job is to fail COMPILATION if the shipped `.d.ts` and the SDK's view of it ever disagree --
 * the same pattern the app uses at `host-factory.ts:20-28`, kept here so the SDK's own adapter is
 * held to the artifact it adapts.
 *
 * Three separate disagreements are made into compile errors:
 *
 * 1. `toWebBootOptions` must return exactly the shipped `MisoWebBootOptions` -- no missing field,
 *    no extra one, no widened type. A `bigint` field that became a `number` would be caught here
 *    and nowhere else until a browser refused the boot.
 * 2. The SDK's `createHost` seam must be *satisfiable by* the shipped factory. If the factory's
 *    options gained a required field the SDK does not pass, this stops compiling rather than
 *    failing at the first real session.
 * 3. The SDK's narrow `AudioContextLike` must be a supertype of what the factory demands, so a
 *    caller that satisfies the SDK also satisfies the factory.
 */

import type { BootOptions } from "../src/core/abi.ts";
import type { AudioContextLike } from "../src/browser/engine.ts";
import { toWebBootOptions } from "../src/browser/host-mirror.ts";
import type {
  CreateMisoAudioWorkletHostOptions,
  MisoAudioWorkletHost,
  MisoWebBootOptions,
} from "../src/browser/shipped-host.d.ts";

/** Assert two types are mutually assignable. A one-way check would miss a widened field. */
type Exact<A, B> = [A] extends [B] ? ([B] extends [A] ? true : never) : never;

// 1. The adapter's return type IS the shipped options type, exactly.
const _returnsShippedOptions: Exact<ReturnType<typeof toWebBootOptions>, MisoWebBootOptions> = true;

// A concrete round trip, so the field names are checked and not merely the shape.
const _adapted: MisoWebBootOptions = toWebBootOptions({
  sourceRingFrames: 0,
  maximumMemoryBytes: 0n,
  console: { commandQueueRecords: 64, meterBlocks: 12, observationTaps: 0, masterTrackPlusOne: 0 },
} satisfies BootOptions);

// 2. The SDK's `createHost` seam is satisfiable by the shipped factory's signature.
type ShippedFactory = (options: CreateMisoAudioWorkletHostOptions) => Promise<MisoAudioWorkletHost>;
declare const shippedFactory: ShippedFactory;
const _factoryFits: (request: {
  context: CreateMisoAudioWorkletHostOptions["context"];
  document: Uint8Array;
  options: MisoWebBootOptions;
  simd128ModuleUrl: string;
  workletModuleUrl: string;
}) => Promise<unknown> = shippedFactory;

// 3. Anything satisfying the factory's context requirement also satisfies the SDK's narrow view --
//    the SDK must not demand more of a context than the host it hands it to.
declare const shippedContext: CreateMisoAudioWorkletHostOptions["context"];
const _contextIsNarrower: Pick<AudioContextLike, "sampleRate"> = shippedContext;

export type { ShippedFactory };
export const HOST_MIRROR_PINNED: readonly unknown[] = [
  _returnsShippedOptions, _adapted, _factoryFits, _contextIsNarrower,
];
