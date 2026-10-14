/**
 * What the generated control builders (`src/generated/control-builders.ts`) stand on: the option
 * and edit types they speak, the small value helpers every builder uses, and the strip base that
 * owns effect addressing.
 *
 * The generated classes hold one method per row of the core's control tables. Every method builds
 * an address and calls the engine's `set` with it (the same staged edit `edit.set` stages for that
 * address and value), so nothing here knows a row. This module imports nothing generated except
 * types, which is what lets the generated classes extend {@link StripAddressing} without a runtime
 * import cycle.
 */

import { CONTROLS } from "../generated/controls.ts";
import type { ControlAddress, ControlLane, ControlOwner } from "../generated/controls.ts";
import type { EffectId, EffectParameter } from "../generated/catalog.ts";
import type { ControlFrameEdit, ControlWireAddress } from "./control-frame.ts";
import { MisoUsageError } from "./errors.ts";
import type { EffectEdits } from "./live-controls.ts";
import { requireU32 } from "../internal/ramp.ts";

/**
 * Where a live effect edit lands (owner decision 12, S1c's live address).
 *
 * A `console` address is a session console slot by its index in the session's slot order --
 * `pre_insert`, then `post_insert` -- which is also its index in every strip's (track's or
 * submix's) `console` array. An `inserts` address is the strip's insert by its index in chain
 * order. `StripEdits.console()` and `StripEdits.insert()` resolve stable IDs to those indices;
 * `StripEdits.effect()` takes the index.
 */
export type LiveControlRack = "console" | "inserts";
export type LiveControlChannel = "left" | "right" | "both";

export interface SmoothingOptions {
  /**
   * The ramp length in samples, sent as the edit's `ramp_samples`. Omitted, the field is absent and
   * the core ramps over the session's own length for the row (`controlSmoothing`, or the default
   * table at the session's sample rate). An explicit value overrides it, and `0` is a hard step.
   * The SDK resolves no ramp and checks only that it is an integer in `[0, 4294967295]`; a row
   * whose ramp is fixed refuses an explicit one with the core's `control.ramp.fixed`.
   */
  readonly smoothingSamples?: number;
}

/** A per-lane row with a fixed ramp selects a lane and takes no ramp. */
export interface ChannelOptions {
  readonly channel?: LiveControlChannel;
}

export interface LaneOptions extends SmoothingOptions {
  readonly channel?: LiveControlChannel;
}

/** Input-filter edits select a lane. The filters ramp over the row's fixed policy. */
export type InputFilterOptions = ChannelOptions;

/** What `edit.set` takes besides the address and the value. */
export interface SetControlOptions {
  /**
   * The edit's own ramp in samples, an integer in `[0, 4294967295]`; `0` is a hard step. Omitted,
   * the core ramps over the session's own length for the row. A row whose ramp is fixed refuses an
   * explicit one with the core's `control.ramp.fixed`.
   */
  readonly rampSamples?: number;
}

/**
 * One value edit as a builder returns it: the `SET_CONTROL` edits that go in one transaction. A
 * fader or a mute is one edit; a pan is its two lanes, a matrix its four coefficients and the input
 * filters their two cutoffs, and they apply together or not at all.
 */
export interface ControlEditSet {
  readonly edits: readonly ControlFrameEdit[];
}

/** `edit.set`: the one call every typed builder is sugar over. */
export type ControlSetter = (
  address: ControlAddress | ControlWireAddress,
  value: number,
  options?: SetControlOptions,
) => ControlEditSet;

type LiveParameter<E extends EffectId> = Extract<
  EffectParameter<E>,
  { readonly liveUpdatable: true }
>;

export type LiveEffectParameterName<E extends EffectId> = LiveParameter<E>["name"];

type ParameterRow<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = Extract<LiveParameter<E>, { readonly name: N }>;

type EnumerationLabel<P> = P extends {
  readonly enumChoices: readonly (infer Choice)[];
} ? Choice extends { readonly label: infer Label extends string } ? Label : never : never;

type ParameterValue<P> = P extends { readonly domainName: "boolean" }
  ? boolean
  : P extends { readonly domainName: "enumeration" }
    ? EnumerationLabel<P>
    : number;

type ParameterOptions<P> = P extends { readonly channelPolicyName: "shared" }
  ? SmoothingOptions & { readonly channel?: "both" }
  : LaneOptions;

export type LiveEffectParameterValue<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = ParameterValue<ParameterRow<E, N>>;

export type LiveEffectParameterOptions<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = ParameterOptions<ParameterRow<E, N>>;

/** One catalog-derived object edit for a live effect parameter. */
export type LiveEffectParameterEdit<E extends EffectId> = LiveParameter<E> extends infer P
  ? P extends { readonly name: infer N extends LiveEffectParameterName<E> }
    ? {
      readonly key: N;
      readonly value: ParameterValue<P>;
    } & ParameterOptions<P>
    : never
  : never;

/** Options the runtime reads off a parameter edit, whatever its static type. */
export type RuntimeParameterOptions = {
  readonly channel?: unknown;
  readonly smoothingSamples?: unknown;
};

/** The edit's `ramp_samples`: absent when none was given, else an integer in `[0, 2^32 - 1]`. */
export function rampOf(options: { readonly smoothingSamples?: unknown }): number | undefined {
  const samples = options.smoothingSamples;
  if (samples === undefined) return undefined;
  if (typeof samples !== "number") throw new MisoUsageError("smoothingSamples must be a number");
  return requireU32(samples);
}

/** `set`'s options for a builder's ramp: the field is absent when the caller gave none. */
export function rampOptions(options: { readonly smoothingSamples?: unknown }): SetControlOptions {
  const ramp = rampOf(options);
  return ramp === undefined ? {} : { rampSamples: ramp };
}

/** The lane the encoder writes: `left`, `right` or `both`, the only names the frame can hold. */
export function laneOf(channel: unknown): ControlLane {
  if (channel === undefined) return "both";
  if (channel === "left" || channel === "right" || channel === "both") return channel;
  throw new MisoUsageError("channel must be left, right, or both");
}

export function flag(enabled: boolean): number {
  return enabled ? 1 : 0;
}

export function editSet(...edits: readonly ControlFrameEdit[]): ControlEditSet {
  return Object.freeze({ edits: Object.freeze([...edits]) });
}

/** Several builders' edits as one set: they apply together or not at all. */
export function mergeEdits(...sets: readonly ControlEditSet[]): ControlEditSet {
  return editSet(...sets.flatMap((set) => set.edits));
}

export function id(value: unknown, what: string): string {
  if (typeof value !== "string") throw new MisoUsageError(`${what} must be a string`);
  return value;
}

/** The number an enumeration row takes for a label, from the row's choices in the core's table. */
export function enumValue(
  choices: readonly (readonly [string, number])[],
  label: unknown,
  what: string,
): number {
  const choice = choices.find(([candidate]) => candidate === label);
  if (choice === undefined) {
    throw new MisoUsageError(`${what} must be one of ${choices.map(([name]) => name).join(", ")}`);
  }
  return choice[1];
}

/** An object argument of a grouped builder (a matrix, the input filters) must be an object. */
export function requireGroup(value: unknown, method: string, fields: readonly string[]): void {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    throw new MisoUsageError(`${method} requires an object with ${fields.join(" and ")}`);
  }
}

const LIVE_PARAMETER_EDIT_FIELDS = new Set(["key", "value", "channel", "smoothingSamples"]);

export interface NormalizedParameterEdit {
  readonly key: string;
  readonly value: unknown;
  readonly options: RuntimeParameterOptions;
}

export function normalizeParameterEdit(
  keyOrEdit: unknown,
  value: unknown,
  options: RuntimeParameterOptions | undefined,
): NormalizedParameterEdit {
  if (typeof keyOrEdit === "string") {
    return { key: keyOrEdit, value, options: options ?? {} };
  }
  if (keyOrEdit === null || typeof keyOrEdit !== "object" || Array.isArray(keyOrEdit)) {
    throw new MisoUsageError("a live parameter edit must be an object");
  }
  for (const field of Reflect.ownKeys(keyOrEdit)) {
    if (typeof field !== "string" || !LIVE_PARAMETER_EDIT_FIELDS.has(field)) {
      throw new MisoUsageError(`live parameter edit has unknown field '${String(field)}'`);
    }
  }
  const edit = keyOrEdit as Record<string, unknown>;
  if (!Object.prototype.hasOwnProperty.call(edit, "key")) {
    throw new MisoUsageError("live parameter edit requires a key");
  }
  if (!Object.prototype.hasOwnProperty.call(edit, "value")) {
    throw new MisoUsageError("live parameter edit requires a value");
  }
  if (typeof edit.key !== "string") {
    throw new MisoUsageError("live parameter edit key must be a string");
  }
  return {
    key: edit.key,
    value: edit.value,
    options: {
      channel: edit.channel,
      smoothingSamples: edit.smoothingSamples,
    },
  };
}

/** One effect parameter as the generated table lists it: the core's row name and value domain. */
export interface EffectParameterRow {
  /** The row's machine name in the core's control table (`threshold_db`). */
  readonly control: string;
  readonly kind: "boolean" | "enumeration" | "continuous";
  readonly choices: readonly (readonly [string, number])[];
}

/** The number a parameter row takes for `value`; whether it is in the row's domain is the core's. */
export function parameterScalar(effectId: string, name: string, row: EffectParameterRow, value: unknown): number {
  switch (row.kind) {
    case "boolean":
      if (typeof value !== "boolean") throw new MisoUsageError(`${effectId}.${name} must be boolean`);
      return flag(value);
    case "enumeration": {
      const choice = row.choices.find(([label]) => label === value);
      if (choice === undefined) {
        throw new MisoUsageError(
          `${effectId}.${name} must be one of ${row.choices.map(([label]) => label).join(", ")}`,
        );
      }
      return choice[1];
    }
    case "continuous":
      if (typeof value !== "number") throw new MisoUsageError(`${effectId}.${name} must be numeric`);
      return value;
  }
}

/**
 * The numeric ID of an effect's row in the core's control tables, by the row's machine name: the
 * bypass, a parameter or the link mode. An effect instance knows which effect it runs, so its
 * builders address a row by this number and need no session to name it.
 */
export function effectControlId(effectId: string, name: string): number {
  const effect = CONTROLS.effects[effectId as EffectId];
  if (effect === undefined) {
    throw new MisoUsageError(`the effect '${effectId}' has no control table`);
  }
  if (name === effect.bypass.name) return effect.bypass.control;
  if (effect.linkMode !== null && effect.linkMode !== undefined && name === effect.linkMode.name) {
    return effect.linkMode.control;
  }
  const row = effect.parameters.find((candidate) => candidate.name === name);
  if (row === undefined) {
    throw new MisoUsageError(`the generated controls have no row ${name} of ${effectId}`);
  }
  return row.control;
}

/** One addressable effect instance, as the session the SDK built declares it. */
export interface LayoutRow {
  readonly id: string;
  readonly effectId: string;
}

/** One strip's instance of a console slot or an insert, with the bypass its session authored. */
export interface InstanceRow extends LayoutRow {
  readonly bypass: boolean;
}

/** One strip's console entries (in slot order) and inserts (in chain order). */
export interface StripInstances {
  readonly console: readonly InstanceRow[];
  readonly inserts: readonly InstanceRow[];
}

/**
 * The part of a built session live controls need to resolve stable IDs to live addresses.
 *
 * The SDK never parses a document (ruling 5438024085), so this comes only from a session the SDK
 * built: the console slots in slot order, and each strip's -- every track's and every submix's --
 * console entries (in the same order) and inserts (in chain order), each with its authored bypass.
 */
export interface LiveControlLayout {
  readonly console: readonly LayoutRow[];
  readonly tracks: ReadonlyMap<string, StripInstances>;
  readonly submixes: ReadonlyMap<string, StripInstances>;
}

/** Which kind of strip a strip-level edit builder addresses; it names the strip in messages. */
export type StripKind = "track" | "submix";

export const NO_LAYOUT = "this engine was not booted from a session the SDK built, so there is nothing to "
  + "resolve an index against (the SDK never parses a document); call liveControls().withSession(session) "
  + "with the builder, or address the effect by its stable ID with console() or insert()";

/** What an effect instance is made from: the strip, the rack and the two ways to name it. */
export interface EffectInstance<E extends EffectId> {
  readonly stripId: string;
  readonly stripIndex: number | undefined;
  readonly rack: LiveControlRack;
  /** Its live index, which observation records name it by. */
  readonly effectIndex: number | undefined;
  /** Its console slot ID or insert ID, which a control address names it by. */
  readonly stableId: string | undefined;
  readonly effectId: E;
}

/** What a strip's edits reach through: `edit.set`, and the engine's effect builders. */
export interface StripContext {
  readonly set: ControlSetter;
  readonly effect: <E extends EffectId>(instance: EffectInstance<E>) => EffectEdits<E>;
}

/**
 * What every strip -- a track or a submix -- shares besides its generated control builders: its
 * owner address, and how an effect instance on it is named (`effect()`, `console()`, `insert()`).
 * The generated `StripEdits` extends this with one builder per row of the strip's tables.
 */
export abstract class StripAddressing {
  /** The strip's index in the session map (the tracks, then the submixes), for observation. */
  protected readonly stripIndex: number | undefined;
  protected readonly stripId: string;
  /** `track 'kick'` or `submix 'drums'`, for messages. */
  readonly #label: string;
  readonly #layout: LiveControlLayout | undefined;
  readonly #instances: StripInstances | undefined;
  readonly #context: StripContext;

  protected constructor(
    kind: StripKind,
    stripId: string,
    stripIndex: number | undefined,
    layout: LiveControlLayout | undefined,
    context: StripContext,
  ) {
    this.stripId = stripId;
    this.stripIndex = stripIndex;
    this.#label = `${kind} '${stripId}'`;
    this.#layout = layout;
    this.#instances = (kind === "track" ? layout?.tracks : layout?.submixes)?.get(stripId);
    this.#context = context;
  }

  /** The strip as the owner of an address. */
  protected controlOwner(): Extract<ControlOwner, { readonly kind: "strip" }> {
    return { kind: "strip", id: this.stripId };
  }

  /** `edit.set`, which every generated builder of this strip calls. */
  protected setControl(address: ControlAddress | ControlWireAddress, value: number, options?: SetControlOptions): ControlEditSet {
    return this.#context.set(address, value, options);
  }

  /**
   * Address an effect by its live index: a console slot by its index in the session's slot order
   * (`pre_insert`, then `post_insert`), or an insert by its index in the strip's chain. Observation
   * records address an effect this way. Value edits address it by its stable ID, so the index must
   * resolve against the session the SDK built (`withSession()`); use `console()` or `insert()`
   * with an ID otherwise.
   */
  effect<E extends EffectId>(
    rack: LiveControlRack,
    effectIndex: number,
    effectId: E,
  ): EffectEdits<E> {
    if (rack !== "console" && rack !== "inserts") {
      throw new MisoUsageError(
        `rack must be console or inserts, not '${String(rack)}' (simd1, dynamic and simd2 are retired)`,
      );
    }
    const index = requireU32(effectIndex, "effectIndex");
    return this.#context.effect({
      stripId: this.stripId,
      stripIndex: this.stripIndex,
      rack,
      effectIndex: index,
      stableId: this.#instances?.[rack][index]?.id,
      effectId,
    });
  }

  /**
   * Address a session console slot by its stable slot ID. `effectId` must be the slot's own effect,
   * and it types the edits. With a session the SDK built, the slot is checked against it.
   */
  console<E extends EffectId>(slot: string, effectId: E): EffectEdits<E> {
    id(slot, "a console slot ID");
    if (this.#layout === undefined) {
      return this.#context.effect({
        stripId: this.stripId,
        stripIndex: this.stripIndex,
        rack: "console",
        effectIndex: undefined,
        stableId: slot,
        effectId,
      });
    }
    const index = this.#layout.console.findIndex((row) => row.id === slot);
    const declared = this.#layout.console[index];
    if (declared === undefined) {
      throw new MisoUsageError(
        `the session has no console slot '${slot}'; its slots are `
          + `${this.#layout.console.map((candidate) => candidate.id).join(", ") || "none"}`,
      );
    }
    return this.#resolved("console", index, declared, effectId);
  }

  /** Address one of this strip's inserts by its stable ID or its index in chain order. */
  insert<E extends EffectId>(insert: string | number, effectId: E): EffectEdits<E> {
    if (typeof insert === "number") {
      if (this.#layout === undefined) throw new MisoUsageError(`insert(${insert}): ${NO_LAYOUT}`);
      const row = this.#instances?.inserts[insert];
      if (row === undefined) {
        throw new MisoUsageError(`${this.#label} has no insert at index ${insert}`);
      }
      return this.#resolved("inserts", insert, row, effectId);
    }
    id(insert, "an insert ID");
    if (this.#layout === undefined) {
      return this.#context.effect({
        stripId: this.stripId,
        stripIndex: this.stripIndex,
        rack: "inserts",
        effectIndex: undefined,
        stableId: insert,
        effectId,
      });
    }
    const rows = this.#instances?.inserts ?? [];
    const index = rows.findIndex((row) => row.id === insert);
    const row = rows[index];
    if (row === undefined) {
      throw new MisoUsageError(
        `${this.#label} has no insert '${insert}'; its inserts are `
          + `${rows.map((candidate) => candidate.id).join(", ") || "none"}`,
      );
    }
    return this.#resolved("inserts", index, row, effectId);
  }

  #resolved<E extends EffectId>(
    rack: LiveControlRack,
    index: number,
    row: LayoutRow,
    effectId: E,
  ): EffectEdits<E> {
    if (row.effectId !== effectId) {
      throw new MisoUsageError(
        `${rack === "console" ? "console slot" : "insert"} '${row.id}' is ${row.effectId}, not ${String(effectId)}`,
      );
    }
    return this.effect(rack, index, effectId);
  }
}
