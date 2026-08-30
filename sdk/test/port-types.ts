/**
 * Issue #278: the port table's COMPILE-TIME half.
 *
 * `builder-evals.mjs` proves what `effect()` refuses at runtime. That half cannot see the other
 * one: a `portId` whose type is wrong never reaches a running test, because the program does not
 * build. So this file, like `host-mirror.ts` and `barrel-surface.ts`, runs nothing and is checked
 * by `scripts/check-sdk-types.sh`. Its job is to fail COMPILATION -- and, via `@ts-expect-error`,
 * to fail it just as loudly if a refusal it pins were ever to start compiling.
 *
 * Red mutation: retype `SidechainSpec.portId` back to `string` -> every `@ts-expect-error` below
 * becomes an unused-directive error and this file stops compiling.
 */

import { effect } from "../src/core/session.ts";
import type { PortName, SidechainPortName } from "../src/generated/catalog.ts";

type Exact<A, B> =
  (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? true : false;
type Assert<T extends true> = T;

// --- the derived port-name types are the descriptor's own rows ---------------------------------

type CompressorPorts = Assert<
  Exact<PortName<"miso.compressor">, "main-in" | "main-out" | "sidechain-in">
>;
type SoftClipPorts = Assert<Exact<PortName<"miso.soft-clip">, "main-in" | "main-out">>;
type CompressorSidechain = Assert<Exact<SidechainPortName<"miso.compressor">, "sidechain-in">>;
type GateSidechain = Assert<Exact<SidechainPortName<"miso.gate-expander">, "sidechain-in">>;
/** Six of the eight launch effects declare none, and `never` is what makes that unwritable. */
type SoftClipSidechain = Assert<Exact<SidechainPortName<"miso.soft-clip">, never>>;
type DelaySidechain = Assert<Exact<SidechainPortName<"miso.delay">, never>>;

// --- and `effect()` is typed by them ------------------------------------------------------------

const source = { kind: "track", trackId: "bass", tap: "post_fader" } as const;

/** The legal call. */
export const routed = effect(
  "miso.compressor",
  {},
  { sidechain: { source, portId: "sidechain-in" } },
);

/** A misspelling is a compile error, not a boot-time diagnostic. */
export const misspelled = effect(
  "miso.compressor",
  {},
  // @ts-expect-error 'sidechan-in' is not one of miso.compressor's declared sidechain inputs
  { sidechain: { source, portId: "sidechan-in" } },
);

/** So is a real port of the wrong role. */
export const wrongRole = effect(
  "miso.compressor",
  {},
  // @ts-expect-error 'main-in' is a mainInput port, not a sidechain input
  { sidechain: { source, portId: "main-in" } },
);

/** And an effect that declares no sidechain input cannot be given one at all. */
export const portless = effect(
  "miso.soft-clip",
  {},
  // @ts-expect-error SidechainPortName<"miso.soft-clip"> is never
  { sidechain: { source, portId: "sidechain-in" } },
);
