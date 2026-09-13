import { CATALOG } from "../generated/catalog.ts";
import type { EffectDescriptor } from "../generated/catalog.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";

/** The three prepared effect racks, in signal order. */
export type ObservationRack = "simd1" | "dynamic" | "simd2";

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

/** One current prepared effect that owns one or more resident observation taps. */
export interface ObservationBinding {
  readonly trackId: string;
  readonly rack: ObservationRack;
  readonly effectIndex: number;
  readonly effectSlotId: string;
  readonly nativeEffectId: string;
  readonly tapIds: readonly number[];
}

/** The current owner map used to resolve stable selections to numeric Wasm addresses. */
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

/** Numeric address carried by the additive Wasm request/reply boundary. */
export interface ObservationAddress {
  readonly trackIndex: number;
  readonly rack: number;
  readonly effectIndex: number;
  readonly tapId: number;
  readonly channels: number;
}

/** Numeric row copied from the fixed Wasm result records. */
export interface RawObservationRow extends ObservationAddress {
  readonly status: number;
  readonly sampleRateHz: number;
  readonly firstSample: bigint;
  readonly endSample: bigint;
  readonly sequence: bigint;
  readonly blocks: number;
  readonly leftPresent: number;
  readonly rightPresent: number;
  readonly left: number;
  readonly right: number;
}

/** Raw binding shape returned by the shipped Worklet host before SDK enrichment. */
export interface RawObservationBinding {
  readonly trackIndex: number;
  readonly rack: number;
  readonly effectIndex: number;
  readonly effectSlotId: string;
  readonly nativeEffectId: string;
  readonly tapIds: readonly number[];
}

const RACK_VALUES: Readonly<Record<ObservationRack, number>> = Object.freeze({
  simd1: 0,
  dynamic: 1,
  simd2: 2,
});

const RACK_NAMES: readonly ObservationRack[] = ["simd1", "dynamic", "simd2"];
const CHANNEL_VALUES: Readonly<Record<ObservationChannels, number>> = Object.freeze({
  left: 1,
  right: 2,
  both: 3,
});
const CHANNEL_NAMES: readonly ObservationChannels[] = ["left", "right", "both"];
const STATUS_NAMES: readonly ObservationStatus[] = ["pending", "unarmed", "ready"];

export const MAXIMUM_OBSERVATION_READS = 256;

export function observationRackValue(rack: ObservationRack): number {
  return RACK_VALUES[rack];
}

export function observationChannelsValue(channels: ObservationChannels): number {
  return CHANNEL_VALUES[channels];
}

function rackName(value: number): ObservationRack {
  const name = RACK_NAMES[value];
  if (name === undefined) throw new Error(`unknown observation rack ${value}`);
  return name;
}

function channelsName(value: number): ObservationChannels {
  const name = CHANNEL_NAMES[value - 1];
  if (name === undefined) throw new Error(`unknown observation channel mask ${value}`);
  return name;
}

function statusName(value: number): ObservationStatus {
  const name = STATUS_NAMES[value - 1];
  if (name === undefined) throw new Error(`unknown observation status ${value}`);
  return name;
}

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

/** Enrich the numeric owner map with canonical track IDs from the same current owner. */
export function enrichObservationMap(
  tracks: readonly string[],
  raw: readonly RawObservationBinding[],
): ObservationMap {
  const bindings = raw.map((binding, index) => {
    if (!Number.isSafeInteger(binding.trackIndex) || binding.trackIndex < 0
        || binding.trackIndex >= tracks.length || typeof tracks[binding.trackIndex] !== "string"
        || !Number.isSafeInteger(binding.effectIndex) || binding.effectIndex < 0
        || !RACK_NAMES[binding.rack] || typeof binding.effectSlotId !== "string"
        || binding.effectSlotId.length === 0 || typeof binding.nativeEffectId !== "string"
        || binding.nativeEffectId.length === 0 || !Array.isArray(binding.tapIds)
        || binding.tapIds.length === 0 || binding.tapIds.some((tapId) => !Number.isSafeInteger(tapId)
          || tapId <= 0 || tapId > 0xffff_ffff)) {
      throw new MisoEngineError("the engine returned a malformed observation map", {
        phase: "output",
        code: "abiMismatch",
        result: 2,
        diagnostics: [{ code: "sdk.observation.map", path: String(index) }],
      });
    }
    return Object.freeze({
      trackId: tracks[binding.trackIndex]!,
      rack: rackName(binding.rack),
      effectIndex: binding.effectIndex,
      effectSlotId: binding.effectSlotId,
      nativeEffectId: binding.nativeEffectId,
      tapIds: Object.freeze([...binding.tapIds]),
    });
  });
  return Object.freeze({ bindings: Object.freeze(bindings) });
}

/** Replace the placeholder track index in resolved addresses using the current track map. */
export function resolveObservationAddressesWithTracks(
  map: ObservationMap,
  tracks: readonly string[],
  selections: readonly ObservationSelection[],
): readonly ObservationAddress[] {
  validateObservationSelections(selections);
  return Object.freeze(selections.map((selection) => {
    const binding = map.bindings.find((candidate) => candidate.trackId === selection.trackId
      && candidate.rack === selection.rack
      && candidate.effectSlotId === selection.effectSlotId);
    if (binding === undefined || !binding.tapIds.includes(selection.tapId)) {
      throw new MisoEngineError("the selected observation is unavailable in the current owner", {
        phase: "output",
        code: "unsupported",
        result: 7,
        diagnostics: [{ code: "sdk.observation.selection", path: selection.effectSlotId }],
      });
    }
    const trackIndex = tracks.indexOf(selection.trackId);
    if (trackIndex < 0) {
      throw new MisoEngineError("the selected observation track is unavailable in the current owner", {
        phase: "output",
        code: "unsupported",
        result: 7,
        diagnostics: [{ code: "sdk.observation.track", path: selection.trackId }],
      });
    }
    return Object.freeze({
      trackIndex,
      rack: observationRackValue(binding.rack),
      effectIndex: binding.effectIndex,
      tapId: selection.tapId,
      channels: observationChannelsValue(selection.channels),
    });
  }));
}

function descriptorFor(nativeEffectId: string, tapId: number): ObservationDescriptor {
  const effect = CATALOG.effects.find((candidate) => candidate.id === nativeEffectId);
  const descriptor = effect?.observations.find((candidate) => candidate.id === tapId);
  if (descriptor === undefined || descriptor.subscribable !== true) {
    throw new MisoEngineError("the engine returned an unknown observation descriptor", {
      phase: "asset",
      code: "abiMismatch",
      result: 2,
      diagnostics: [{ code: "sdk.observation.descriptor", path: `${nativeEffectId}/${tapId}` }],
    });
  }
  return descriptor;
}

/** Decode copied rows into owned public values while preserving the owner's raw units and timing. */
export function decodeObservationRows(
  map: ObservationMap,
  tracks: readonly string[],
  selections: readonly ObservationSelection[],
  addresses: readonly ObservationAddress[],
  rows: readonly RawObservationRow[],
  sampleRateHz: number,
): readonly ObservationReadResult[] {
  if (rows.length !== selections.length || addresses.length !== selections.length) {
    throw new MisoEngineError("the engine returned the wrong observation row count", {
      phase: "output",
      code: "abiMismatch",
      result: 2,
      diagnostics: [{ code: "sdk.observation.rows", path: "$" }],
    });
  }
  const result = rows.map((row, index) => {
    const selection = selections[index]!;
    const address = addresses[index]!;
    const binding = map.bindings.find((candidate) => candidate.trackId === selection.trackId
      && candidate.rack === selection.rack && candidate.effectSlotId === selection.effectSlotId);
    if (binding === undefined || row.trackIndex !== address.trackIndex || row.rack !== address.rack
        || row.effectIndex !== address.effectIndex || row.tapId !== address.tapId
        || row.channels !== address.channels || row.sampleRateHz !== sampleRateHz
        || ![1, 2, 3].includes(row.status) || ![1, 2, 3].includes(row.channels)
        || ![0, 1].includes(row.leftPresent) || ![0, 1].includes(row.rightPresent)
        || !Number.isFinite(row.left) || !Number.isFinite(row.right)) {
      throw new MisoEngineError("the engine returned a malformed observation row", {
        phase: "output",
        code: "abiMismatch",
        result: 2,
        diagnostics: [{ code: "sdk.observation.row", path: String(index) }],
      });
    }
    const expectedLeft = address.channels === 1 || address.channels === 3 ? 1 : 0;
    const expectedRight = address.channels === 2 || address.channels === 3 ? 1 : 0;
    if (row.leftPresent !== expectedLeft || row.rightPresent !== expectedRight) {
      throw new MisoEngineError("the engine returned an invalid observation channel projection", {
        phase: "output",
        code: "abiMismatch",
        result: 2,
        diagnostics: [{ code: "sdk.observation.channels", path: String(index) }],
      });
    }
    const descriptor = descriptorFor(binding.nativeEffectId, selection.tapId);
    const status = statusName(row.status);
    let window: ObservationWindow | undefined;
    if (status === "ready") {
      if (row.firstSample < 0n || row.endSample <= row.firstSample || row.sequence <= 0n
          || row.blocks <= 0 || !Number.isSafeInteger(row.blocks)) {
        throw new MisoEngineError("the engine returned an invalid observation window", {
          phase: "output",
          code: "abiMismatch",
          result: 2,
          diagnostics: [{ code: "sdk.observation.window", path: String(index) }],
        });
      }
      window = Object.freeze({
        firstSample: row.firstSample,
        endSample: row.endSample,
        sequence: row.sequence,
        blocks: row.blocks,
      });
    } else if (row.firstSample !== 0n || row.endSample !== 0n || row.sequence !== 0n
        || row.blocks !== 0 || row.leftPresent !== 0 || row.rightPresent !== 0) {
      throw new MisoEngineError("the engine returned historical data for an unavailable observation", {
        phase: "output",
        code: "abiMismatch",
        result: 2,
        diagnostics: [{ code: "sdk.observation.availability", path: String(index) }],
      });
    }
    return Object.freeze({
      trackId: tracks[address.trackIndex] ?? selection.trackId,
      rack: selection.rack,
      effectSlotId: binding.effectSlotId,
      tapId: selection.tapId,
      nativeEffectId: binding.nativeEffectId,
      descriptor,
      channels: selection.channels,
      sampleRateHz,
      status,
      ...(row.leftPresent === 1 ? { left: row.left } : {}),
      ...(row.rightPresent === 1 ? { right: row.right } : {}),
      ...(window === undefined ? {} : { window }),
    });
  });
  return Object.freeze(result);
}
