import { CATALOG } from "../generated/catalog.ts";
import type { EffectDescriptor } from "../generated/catalog.ts";
import { MisoUsageError } from "./errors.ts";

/** Where an observed effect lives: a session console slot or one of the strip's inserts. */
export type ObservationRack = "console" | "inserts";

/** The independent lanes a caller wants copied from one resident observation. */
export type ObservationChannels = "left" | "right" | "both";

/** Availability of the latest complete resident window. */
export type ObservationStatus = "pending" | "unarmed" | "ready";

/** One stable caller-owned observation target. */
export interface ObservationSelection {
  readonly trackId: string;
  readonly rack: ObservationRack;
  readonly effectSlotId: string;
  readonly tapId: number;
  readonly channels: ObservationChannels;
}

/**
 * One effect of the running session that owns one or more resident observation taps, named by
 * stable IDs only (#1590 D3, ruling C2): the strip, the console slot's or insert's ID, the effect
 * identity and its declared taps.
 */
export interface ObservationBinding {
  readonly trackId: string;
  readonly rack: ObservationRack;
  readonly effectSlotId: string;
  readonly nativeEffectId: string;
  readonly tapIds: readonly number[];
}

/**
 * The effects of the running session that declare taps, from the core's control list and the
 * generated catalog (#1590 D3). A subscription checks a selection against it before any edit; the
 * edit and the read name their tap by stable key and the core resolves it.
 */
export interface ObservationMap {
  readonly bindings: readonly ObservationBinding[];
}

/** The untouched window identity and span published by the effect owner. */
export interface ObservationWindow {
  readonly firstSample: bigint;
  readonly endSample: bigint;
  readonly sequence: bigint;
  readonly blocks: number;
}

/** One selected resident value, preserving the descriptor's native unit and timing. */
export interface ObservationReadResult {
  readonly trackId: string;
  readonly rack: ObservationRack;
  readonly effectSlotId: string;
  readonly tapId: number;
  readonly nativeEffectId: string;
  readonly descriptor: ObservationDescriptor;
  readonly channels: ObservationChannels;
  readonly sampleRateHz: number;
  readonly status: ObservationStatus;
  readonly left?: number;
  readonly right?: number;
  readonly window?: ObservationWindow;
}

/** The generated catalog row describing one effect-owned observation tap. */
export type ObservationDescriptor = EffectDescriptor["observations"][number];

const RACK_NAMES: readonly ObservationRack[] = ["console", "inserts"];
const CHANNEL_NAMES: readonly ObservationChannels[] = ["left", "right", "both"];

export const MAXIMUM_OBSERVATION_READS = 256;

function u32(value: number, name: string): void {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffff_ffff) {
    throw new MisoUsageError(`${name} must be a u32`);
  }
}

/** Validate one public selection before any boundary staging or request is attempted. */
export function validateObservationSelection(selection: ObservationSelection, index: number): void {
  if (selection === null || typeof selection !== "object"
      || typeof selection.trackId !== "string" || selection.trackId.length === 0
      || typeof selection.effectSlotId !== "string" || selection.effectSlotId.length === 0
      || !RACK_NAMES.includes(selection.rack)
      || !CHANNEL_NAMES.includes(selection.channels)) {
    throw new MisoUsageError(`observation selection ${index} has an invalid stable address`);
  }
  u32(selection.tapId, `observation selection ${index}.tapId`);
  if (selection.tapId === 0) {
    throw new MisoUsageError(`observation selection ${index}.tapId must be nonzero`);
  }
}

/** Validate a complete bounded selection batch, including duplicate stable targets. */
export function validateObservationSelections(selections: readonly ObservationSelection[]): void {
  if (!Array.isArray(selections) || selections.length === 0) {
    throw new MisoUsageError("observation selections must be a nonempty array");
  }
  if (selections.length > MAXIMUM_OBSERVATION_READS) {
    throw new MisoUsageError(`observation selections are capped at ${MAXIMUM_OBSERVATION_READS}`);
  }
  const seen = new Set<string>();
  selections.forEach((selection, index) => {
    validateObservationSelection(selection, index);
    const key = `${selection.trackId}\u0000${selection.rack}\u0000${selection.effectSlotId}\u0000${selection.tapId}`;
    if (seen.has(key)) throw new MisoUsageError(`observation selection ${index} duplicates an earlier target`);
    seen.add(key);
  });
}
