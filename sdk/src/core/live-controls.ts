import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import { CONTROLS } from "../generated/controls.ts";
import type { ControlLane, ControlOwner, ControlSection } from "../generated/controls.ts";
import type {
  EffectId,
  EffectDescriptor,
  EffectParameter,
  EffectParameterName,
  TapName,
} from "../generated/catalog.ts";
import type { CommandReport, SessionMap } from "./boundary.ts";
import type { ControlFrameEdit } from "./control-frame.ts";
import type { ControlAppliedReport, ControlTransactionQueue } from "./control-queue.ts";
import { MisoUsageError } from "./errors.ts";
import { writeCanonicalSessionDocument } from "../internal/session-json.ts";
import type { SessionLike, SessionModel } from "./session.ts";
import { requireU32 } from "../internal/ramp.ts";
import { LiveControlWriter } from "./writer.ts";
import type { ControlWriterOptions, LaneEdit } from "./writer.ts";

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

export interface LaneOptions extends SmoothingOptions {
  readonly channel?: LiveControlChannel;
}

/** The two cutoff values carried by one `inputFilters` edit set. */
export interface InputFilterValues {
  readonly hpfHz: number;
  readonly lpfHz: number;
}

/** Input-filter edits select a lane. The filters ramp over the row's fixed policy. */
export interface InputFilterOptions {
  readonly channel?: LiveControlChannel;
}

type LiveParameter<E extends EffectId> = Extract<
  EffectParameter<E>,
  { readonly liveUpdatable: true }
>;

export type LiveEffectParameterName<E extends EffectId> = LiveParameter<E>["name"];

type ParameterRow<
  E extends EffectId,
  N extends LiveEffectParameterName<E>,
> = Extract<LiveParameter<E>, { readonly name: N }>;

type ParameterOfDescriptor<D> = D extends {
  readonly parameters: readonly (infer Parameter)[];
} ? Parameter : never;
type CatalogParameterRow = ParameterOfDescriptor<EffectDescriptor>;
type RuntimeParameterRow = Omit<
  CatalogParameterRow,
  "domainName" | "enumChoices" | "minimum" | "maximum" | "channelPolicyName"
> & {
  readonly domainName: "boolean" | "enumeration" | "continuous";
  readonly enumChoices: readonly { readonly label: string; readonly value: number }[];
  readonly minimum: number | null;
  readonly maximum: number | null;
  readonly channelPolicyName: "shared" | "perLane";
};

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

export interface MatrixValues {
  readonly ll: number;
  readonly lr: number;
  readonly rl: number;
  readonly rr: number;
}


/**
 * One value edit as a builder returns it: the `SET_CONTROL` edits that go in one transaction. A
 * fader or a mute is one edit; a pan is its two lanes, a matrix its four coefficients and the input
 * filters their two cutoffs, and they apply together or not at all.
 */
export interface ControlEditSet {
  readonly edits: readonly ControlFrameEdit[];
}

/** What live controls send through: the engine's one transaction queue, and the observation records. */
export interface LiveControlTransport {
  /** Every value edit goes here, as a `SET_CONTROL` transaction. */
  readonly queue: ControlTransactionQueue;
  /** Observation records (kinds 7 and 8) stay 48-byte commands until the browser-feature phase. */
  readonly observe: (edits: readonly LaneEdit[]) => CommandReport | Promise<CommandReport>;
}

/**
 * Optional owner hook used to serialize observation edits with managed subscriptions. It sees the
 * observation edits as the builders made them.
 */
export type LiveControlBeforeSubmit = (
  edits: readonly LaneEdit[],
  managed?: boolean,
) => void | Promise<void>;

function rackCode(name: string): number {
  const row = ABI_LAYOUT.constants.racks.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated ABI layout has no rack ${name}`);
  return row.value;
}

/**
 * The observation record's rack byte, from the generated layout's `racks` table (S1c): `1` inserts,
 * `3` console and `255` not applicable. The retired `0` and `2` are never written.
 */
const RACKS: Readonly<Record<LiveControlRack, number>> = Object.freeze({
  console: rackCode("console"),
  inserts: rackCode("inserts"),
});
const NONE = rackCode("notApplicable");

/** One addressable effect instance, as the session the SDK built declares it. */
interface LayoutRow {
  readonly id: string;
  readonly effectId: string;
}

/** One strip's instance of a console slot or an insert, with the bypass its session authored. */
interface InstanceRow extends LayoutRow {
  readonly bypass: boolean;
}

/** One strip's console entries (in slot order) and inserts (in chain order). */
interface StripInstances {
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
interface LiveControlLayout {
  readonly console: readonly LayoutRow[];
  readonly tracks: ReadonlyMap<string, StripInstances>;
  readonly submixes: ReadonlyMap<string, StripInstances>;
  /** The IDs of the session's routes into the output, which are never live (issue #1223 D4). */
  readonly outputRoutes: ReadonlySet<string>;
}

/** Which kind of strip a strip-level edit builder addresses; it names the strip in messages. */
type StripKind = "track" | "submix";

function modelOf(session: SessionLike): SessionModel {
  const candidate = session as { readonly toJSON?: unknown; readonly schema_version?: unknown };
  if (candidate !== null && typeof candidate === "object" && typeof candidate.toJSON === "function") {
    return (candidate.toJSON as () => SessionModel)();
  }
  if (candidate === null || typeof candidate !== "object" || candidate.schema_version !== 1) {
    throw new MisoUsageError("live controls resolve IDs against a session built by the SDK");
  }
  return session as SessionModel;
}

function layoutRow(record: unknown, idKey: "slot" | "id"): LayoutRow {
  const row = record as Readonly<Record<string, unknown>>;
  const identity = row.identity as Readonly<Record<string, unknown>> | undefined;
  return Object.freeze({ id: String(row[idKey]), effectId: String(identity?.effect_id ?? "") });
}

/**
 * Each strip's instances: a track and a submix carry the same `console` and `inserts` shapes, so
 * one walk serves both.
 */
function stripInstances(
  strips: readonly Readonly<Record<string, unknown>>[],
  slots: readonly LayoutRow[],
): Map<string, StripInstances> {
  const instances = new Map<string, StripInstances>();
  for (const strip of strips) {
    const entries = (strip.console as readonly Readonly<Record<string, unknown>>[] | undefined) ?? [];
    const effects = (strip.inserts as Readonly<Record<string, readonly unknown[]>> | undefined)?.effects ?? [];
    instances.set(String(strip.id), Object.freeze({
      // Entries follow slot order, so entry `i` is slot `i`'s instance on this strip.
      console: Object.freeze(slots.map((slot, index) =>
        Object.freeze({ ...slot, bypass: entries[index]?.bypass === true }))),
      inserts: Object.freeze(effects.map((effect) => Object.freeze({
        ...layoutRow(effect, "id"),
        bypass: (effect as Readonly<Record<string, unknown>>).bypass === true,
      }))),
    }));
  }
  return instances;
}

function sameIds(declared: Iterable<string>, compiled: readonly string[]): boolean {
  const left = [...declared].sort();
  const right = [...compiled].sort();
  return left.length === right.length && left.every((id, index) => id === right[index]);
}

function layoutOf(session: SessionLike, map: SessionMap): LiveControlLayout {
  const model = modelOf(session);
  const consoleRecord = model.console as Readonly<Record<string, readonly unknown[]>>;
  const slots = [...(consoleRecord.pre_insert ?? []), ...(consoleRecord.post_insert ?? [])]
    .map((slot) => layoutRow(slot, "slot"));
  const tracks = stripInstances(model.tracks, slots);
  const submixes = stripInstances(model.submixes ?? [], slots);
  if (!sameIds(tracks.keys(), map.tracks) || !sameIds(submixes.keys(), map.submixes)) {
    const list = (ids: Iterable<string>) => [...ids].sort().join(", ") || "none";
    throw new MisoUsageError(
      `the session declares tracks ${list(tracks.keys())} and submixes ${list(submixes.keys())}, `
        + `but the engine compiled tracks ${list(map.tracks)} and submixes ${list(map.submixes)}; `
        + "live controls resolve IDs only against the session the engine booted",
    );
  }
  const outputRoutes = new Set(model.routes
    .filter((route) => (route.destination as Readonly<Record<string, unknown>> | undefined)?.kind
      === "output_input")
    .map((route) => String(route.id)));
  return Object.freeze({ console: Object.freeze(slots), tracks, submixes, outputRoutes });
}

/**
 * Hold `session` to the document the engine booted, byte for byte (#1097 verdict M1).
 *
 * The SDK never parses a document, so the only identity it can check without a parser is the
 * canonical text: the builder's `toJson()` is the engine's canonical writer's output for the same
 * session, byte for byte, so the builder that wrote the booted document reproduces it exactly, and
 * any other builder -- a reordered console, a renamed slot, a different insert chain on any track,
 * or any other difference -- does not. A track-set check alone would let a live bypass resolve
 * against a reordered console and land on another slot, acknowledged `ok`.
 */
function assertBootedSession(session: SessionLike, booted: Uint8Array): void {
  const written = new TextEncoder().encode(writeCanonicalSessionDocument(modelOf(session)));
  const length = Math.min(written.length, booted.length);
  let offset = 0;
  while (offset < length && written[offset] === booted[offset]) offset += 1;
  if (offset === length && written.length === booted.length) return;
  throw new MisoUsageError(
    `withSession(): the session's canonical document is not the document this engine booted `
      + `(they first differ at byte ${offset}); live controls resolve console slot and insert IDs `
      + "only against the session the engine booted, so pass the builder whose toJson() is that "
      + "document (the SDK never parses a document, so a booted text that is not canonical cannot "
      + "be matched: boot the builder's toJson() or the engine's canonical text)",
  );
}

type RuntimeParameterOptions = {
  readonly channel?: unknown;
  readonly smoothingSamples?: unknown;
};

/** The edit's `ramp_samples`: absent when none was given, else an integer in `[0, 2^32 - 1]`. */
function rampOf(options: { readonly smoothingSamples?: unknown }): number | undefined {
  const samples = options.smoothingSamples;
  if (samples === undefined) return undefined;
  if (typeof samples !== "number") throw new MisoUsageError("smoothingSamples must be a number");
  return requireU32(samples);
}

/** The lane the encoder writes: `left`, `right` or `both`, the only names the frame can hold. */
function laneOf(channel: unknown): ControlLane {
  if (channel === undefined) return "both";
  if (channel === "left" || channel === "right" || channel === "both") return channel;
  throw new MisoUsageError("channel must be left, right, or both");
}

const LIVE_PARAMETER_EDIT_FIELDS = new Set(["key", "value", "channel", "smoothingSamples"]);

interface NormalizedParameterEdit {
  readonly key: string;
  readonly value: unknown;
  readonly options: RuntimeParameterOptions;
}

function normalizeParameterEdit(
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


function builtinRow(name: string): number {
  const row = CONTROLS.builtins.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated controls have no builtin ${name}`);
  return row.control;
}

function ownRow(
  table: readonly { readonly control: number; readonly name: string }[],
  name: string,
): number {
  const row = table.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated controls have no row ${name}`);
  return row.control;
}

function flag(enabled: boolean): number {
  return enabled ? 1 : 0;
}

function editSet(...edits: readonly ControlFrameEdit[]): ControlEditSet {
  return Object.freeze({ edits: Object.freeze([...edits]) });
}

function controlEdit(
  owner: ControlOwner,
  section: ControlSection,
  control: number,
  value: number,
  lane: ControlLane,
  rampSamples: number | undefined,
): ControlFrameEdit {
  return Object.freeze({
    address: Object.freeze({ owner, section, control, lane }),
    value,
    // Absent when none was given: the core resolves the row's ramp.
    ...(rampSamples === undefined ? {} : { rampSamples }),
  });
}

function observationEdit(
  kind: LaneEdit["kind"],
  stripIndex: number,
  options: {
    readonly rack: number;
    readonly effectIndex: number;
    readonly parameterId: number;
    readonly smoothingSamples: number;
  },
): LaneEdit {
  return Object.freeze({
    kind,
    trackIndex: stripIndex,
    rack: options.rack,
    channel: NONE,
    effectIndex: options.effectIndex,
    parameterId: options.parameterId,
    smoothingSamples: options.smoothingSamples,
    values: Object.freeze([0, 0, 0, 0]) as unknown as readonly [number, number, number, number],
  });
}

function id(value: unknown, what: string): string {
  if (typeof value !== "string") throw new MisoUsageError(`${what} must be a string`);
  return value;
}

/**
 * A semantic edit builder bound to the engine's session map.
 *
 * Builders address by stable ID and build `SET_CONTROL` edits; they check no value, no lane's
 * meaning and no index, and send nothing. An ID the engine does not have, a value outside a row's
 * domain and a lane a row does not allow come back from the core as its own `control.*` code, the
 * same on the browser and on the headless engine. The session map supplies only the strip order,
 * for observation records, which address a strip by index.
 *
 * Given the session the SDK built for this engine, it also resolves console slot IDs and insert IDs
 * on every strip (`console()`, `insert()`) against the booted session, and an effect addressed by
 * index (`effect()`) to its stable ID.
 */
export class LiveControlEdits {
  readonly #tracks: ReadonlyMap<string, number>;
  readonly #submixes: ReadonlyMap<string, number>;
  readonly #layout: LiveControlLayout | undefined;

  constructor(map: SessionMap, session?: SessionLike) {
    this.#tracks = new Map(map.tracks.map((trackId, index) => [trackId, index] as const));
    const trackCount = map.tracks.length;
    this.#submixes = new Map(map.submixes.map((submixId, index) => [submixId, trackCount + index] as const));
    this.#layout = session === undefined ? undefined : layoutOf(session, map);
  }

  /**
   * A track strip's live edits. The ID is not checked here: an unknown strip is refused by the core
   * with `control.address.unknown`.
   */
  track(trackId: string): TrackEdits {
    return new TrackEdits(id(trackId, "a track ID"), this.#tracks.get(trackId), this.#layout);
  }

  /**
   * A submix strip's live edits: everything a track's strip offers except `solo` (a bus is
   * solo-safe; the core refuses a solo of a submix).
   */
  submix(submixId: string): SubmixEdits {
    return new SubmixEdits(id(submixId, "a submix ID"), this.#submixes.get(submixId), this.#layout);
  }

  /**
   * Either kind of strip by its ID -- a track, else a submix (strip IDs are unique across both) --
   * for an edit every strip shares, such as arming an effect observation. It never offers `solo`.
   * Observation addresses a strip by its index, so an ID that is neither is refused here.
   */
  strip(stripId: string): StripEdits {
    if (this.#tracks.has(stripId)) return this.track(stripId);
    if (this.#submixes.has(stripId)) return this.submix(stripId);
    throw new MisoUsageError(
      `the compiled session has no track or submix '${stripId}'; expected one of `
        + `${[...this.#tracks.keys(), ...this.#submixes.keys()].join(", ")}`,
    );
  }

  /**
   * A live send's edits, by its route ID. The ID is not checked here: a route the core does not
   * enumerate (an unknown ID, a route into the output) is refused with `control.address.unknown`.
   */
  route(routeId: string): RouteEdits {
    return new RouteEdits(id(routeId, "a route ID"));
  }

  /**
   * A VCA's live edits, by its VCA ID: its per-lane fader offset and mute. The ID is not checked
   * here: an unknown VCA is refused by the core with `control.address.unknown`.
   */
  vca(vcaId: string): VcaEdits {
    return new VcaEdits(id(vcaId, "a VCA ID"));
  }
}

const NO_LAYOUT = "this engine was not booted from a session the SDK built, so there is nothing to "
  + "resolve an index against (the SDK never parses a document); call liveControls().withSession(session) "
  + "with the builder, or address the effect by its stable ID with console() or insert()";

/**
 * Every live edit a strip -- a track or a submix -- shares. Methods build data and never mutate
 * the engine. `TrackEdits` adds `solo`; `SubmixEdits` adds nothing.
 *
 * `pan`, `matrix`, `faderDb`, `mute`, `trimDb` and `polarityInvert` take an optional
 * `smoothingSamples`, sent as the edit's `ramp_samples`; absent, the core ramps over the session's
 * own length for the row. The input filters (`hpfHz`, `lpfHz`, `inputFilters`) and `bypass` have a
 * fixed ramp and take none.
 *
 * `pan` writes the strip's two pan lanes and `matrix` its four coefficients. A strip in the other
 * shape is refused by the core with `control.inactive`.
 */
export abstract class StripEdits {
  /** The strip's index in the session map (the tracks, then the submixes), for observation. */
  protected readonly stripIndex: number | undefined;
  protected readonly stripId: string;
  /** `track 'kick'` or `submix 'drums'`, for messages. */
  readonly #label: string;
  readonly #layout: LiveControlLayout | undefined;
  readonly #instances: StripInstances | undefined;

  protected constructor(
    kind: StripKind,
    stripId: string,
    stripIndex: number | undefined,
    layout?: LiveControlLayout,
  ) {
    this.stripId = stripId;
    this.stripIndex = stripIndex;
    this.#label = `${kind} '${stripId}'`;
    this.#layout = layout;
    this.#instances = (kind === "track" ? layout?.tracks : layout?.submixes)?.get(stripId);
  }

  #owner(): ControlOwner {
    return { kind: "strip", id: this.stripId };
  }

  #builtin(name: string, value: number, lane: ControlLane, ramp?: number): ControlFrameEdit {
    return controlEdit(this.#owner(), { kind: "builtins" }, builtinRow(name), value, lane, ramp);
  }

  /** Set the pan of each lane: `left` on the left lane and `right` on the right. */
  pan(left: number, right: number, options: SmoothingOptions = {}): ControlEditSet {
    const ramp = rampOf(options);
    return editSet(
      this.#builtin("pan", left, "left", ramp),
      this.#builtin("pan", right, "right", ramp),
    );
  }

  matrix(matrix: MatrixValues, options: SmoothingOptions = {}): ControlEditSet {
    const ramp = rampOf(options);
    return editSet(
      this.#builtin("matrix_ll", matrix.ll, "both", ramp),
      this.#builtin("matrix_lr", matrix.lr, "both", ramp),
      this.#builtin("matrix_rl", matrix.rl, "both", ramp),
      this.#builtin("matrix_rr", matrix.rr, "both", ramp),
    );
  }

  faderDb(db: number, options: LaneOptions = {}): ControlEditSet {
    return editSet(this.#builtin("fader_db", db, laneOf(options.channel), rampOf(options)));
  }

  mute(enabled: boolean, options: LaneOptions = {}): ControlEditSet {
    return editSet(this.#builtin("mute", flag(enabled), laneOf(options.channel), rampOf(options)));
  }

  trimDb(db: number, options: LaneOptions = {}): ControlEditSet {
    return editSet(this.#builtin("trim_db", db, laneOf(options.channel), rampOf(options)));
  }

  polarityInvert(enabled: boolean, options: LaneOptions = {}): ControlEditSet {
    return editSet(
      this.#builtin("polarity_invert", flag(enabled), laneOf(options.channel), rampOf(options)),
    );
  }

  /** Set one lane's HPF cutoff. */
  hpfHz(value: number, options: InputFilterOptions = {}): ControlEditSet {
    return editSet(this.#builtin("hpf_hz", value, laneOf(options.channel)));
  }

  /** Set one lane's LPF cutoff. */
  lpfHz(value: number, options: InputFilterOptions = {}): ControlEditSet {
    return editSet(this.#builtin("lpf_hz", value, laneOf(options.channel)));
  }

  /** Set both cutoffs in one transaction. */
  inputFilters(filters: InputFilterValues, options: InputFilterOptions = {}): ControlEditSet {
    if (filters === null || typeof filters !== "object" || Array.isArray(filters)) {
      throw new MisoUsageError("inputFilters requires an object with hpfHz and lpfHz");
    }
    const lane = laneOf(options.channel);
    return editSet(
      this.#builtin("hpf_hz", filters.hpfHz, lane),
      this.#builtin("lpf_hz", filters.lpfHz, lane),
    );
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
    return new EffectEdits(
      this.stripId,
      this.stripIndex,
      rack,
      index,
      this.#instances?.[rack][index]?.id,
      effectId,
    );
  }

  /**
   * Address a session console slot by its stable slot ID. `effectId` must be the slot's own effect,
   * and it types the edits. With a session the SDK built, the slot is checked against it.
   */
  console<E extends EffectId>(slot: string, effectId: E): EffectEdits<E> {
    id(slot, "a console slot ID");
    if (this.#layout === undefined) {
      return new EffectEdits(this.stripId, this.stripIndex, "console", undefined, slot, effectId);
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
      return new EffectEdits(this.stripId, this.stripIndex, "inserts", undefined, insert, effectId);
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

/** A track strip's live edits: every strip edit, plus `solo`. */
export class TrackEdits extends StripEdits {
  constructor(trackId: string, trackIndex?: number, layout?: LiveControlLayout) {
    super("track", trackId, trackIndex, layout);
  }

  /** Solo is a monitor overlay: the core reports the edit with the `monitor` path. */
  solo(enabled: boolean, options: SmoothingOptions = {}): ControlEditSet {
    return editSet(controlEdit(
      { kind: "strip", id: this.stripId },
      { kind: "own" },
      ownRow(CONTROLS.stripOwn, "solo"),
      flag(enabled),
      "both",
      rampOf(options),
    ));
  }
}

/**
 * A submix strip's live edits: every strip edit and no `solo`. `LiveControlEdits.submix(id)` also
 * finds the submix's strip index, `T + j`, for observation.
 */
export class SubmixEdits extends StripEdits {
  constructor(submixId: string, stripIndex?: number, layout?: LiveControlLayout) {
    super("submix", submixId, stripIndex, layout);
  }
}

/**
 * A live send's edits: its gain, its mute, its follow-mute flag and its 2x2 matrix, each over an
 * optional `smoothingSamples` (omitted, the core's ramp for the row). A route is addressed by its
 * stable ID. Only routes into submixes are live: the core refuses an output route.
 */
export class RouteEdits {
  readonly #routeId: string;

  constructor(routeId: string) {
    this.#routeId = routeId;
  }

  #row(name: string, value: number, ramp: number | undefined): ControlFrameEdit {
    return controlEdit(
      { kind: "route", id: this.#routeId },
      { kind: "own" },
      ownRow(CONTROLS.routeOwn, name),
      value,
      "both",
      ramp,
    );
  }

  gainDb(db: number, options: SmoothingOptions = {}): ControlEditSet {
    return editSet(this.#row("gain_db", db, rampOf(options)));
  }

  /** `true` silences the send, `false` opens it. */
  mute(enabled: boolean, options: SmoothingOptions = {}): ControlEditSet {
    return editSet(this.#row("mute", flag(enabled), rampOf(options)));
  }

  /** The send's 2x2 matrix, written `ll, lr, rl, rr`. */
  matrix(matrix: MatrixValues, options: SmoothingOptions = {}): ControlEditSet {
    const ramp = rampOf(options);
    return editSet(
      this.#row("matrix_ll", matrix.ll, ramp),
      this.#row("matrix_lr", matrix.lr, ramp),
      this.#row("matrix_rl", matrix.rl, ramp),
      this.#row("matrix_rr", matrix.rr, ramp),
    );
  }

  /** Whether the send follows its source strip's mute (a send into a submix only). */
  followsMute(enabled: boolean, options: SmoothingOptions = {}): ControlEditSet {
    return editSet(this.#row("follows_mute", flag(enabled), rampOf(options)));
  }
}

/**
 * A VCA's live edits: its per-lane fader offset and mute, built as a strip's `faderDb` and `mute`
 * are, over an optional lane and `smoothingSamples`. A VCA has no audio path: the engine adds the
 * offset to every member's own fader, and its mute mutes every member.
 */
export class VcaEdits {
  readonly #vcaId: string;

  constructor(vcaId: string) {
    this.#vcaId = vcaId;
  }

  #row(name: string, value: number, options: LaneOptions): ControlFrameEdit {
    return controlEdit(
      { kind: "vca", id: this.#vcaId },
      { kind: "own" },
      ownRow(CONTROLS.vcaOwn, name),
      value,
      laneOf(options.channel),
      rampOf(options),
    );
  }

  /** The VCA's offset in dB, in the builtin fader's own domain. */
  faderDb(db: number, options: LaneOptions = {}): ControlEditSet {
    return editSet(this.#row("offset_db", db, options));
  }

  /** `true` mutes every member on the selected lanes, `false` lifts the VCA's mute. */
  mute(enabled: boolean, options: LaneOptions = {}): ControlEditSet {
    return editSet(this.#row("mute", flag(enabled), options));
  }
}

/** Catalog-derived edits for one effect instance. */
export class EffectEdits<E extends EffectId> {
  readonly #stripId: string;
  readonly #stripIndex: number | undefined;
  readonly #rack: LiveControlRack;
  readonly #effectIndex: number | undefined;
  readonly #stableId: string | undefined;
  readonly #effectId: E;

  /**
   * `stableId` is the instance's console slot ID or insert ID, which the control address names it
   * by; `effectIndex` is its live index, which observation records name it by. Either may be
   * unknown: an instance named by ID without a session has no index, and one named by index
   * without a session has no ID.
   */
  constructor(
    stripId: string,
    stripIndex: number | undefined,
    rack: LiveControlRack,
    effectIndex: number | undefined,
    stableId: string | undefined,
    effectId: E,
  ) {
    this.#stripId = stripId;
    this.#stripIndex = stripIndex;
    this.#rack = rack;
    this.#effectIndex = effectIndex;
    this.#stableId = stableId;
    this.#effectId = effectId;
  }

  #section(): ControlSection {
    const stable = this.#stableId;
    if (stable === undefined) {
      throw new MisoUsageError(
        `this ${this.#effectId} was addressed by index, and a value edit names an instance by its `
          + `stable ID: ${NO_LAYOUT}`,
      );
    }
    return this.#rack === "console"
      ? { kind: "console", slot: stable }
      : { kind: "inserts", effect: stable };
  }

  #edit(control: number, value: number, lane: ControlLane, ramp: number | undefined): ControlEditSet {
    return editSet(controlEdit(
      { kind: "strip", id: this.#stripId },
      this.#section(),
      control,
      value,
      lane,
      ramp,
    ));
  }

  parameter<N extends LiveEffectParameterName<E>>(
    name: N,
    value: LiveEffectParameterValue<E, N>,
    options?: LiveEffectParameterOptions<E, N>,
  ): ControlEditSet;
  parameter(edit: LiveEffectParameterEdit<E>): ControlEditSet;
  parameter<N extends LiveEffectParameterName<E>>(
    keyOrEdit: string | LiveEffectParameterEdit<E>,
    value?: unknown,
    options: RuntimeParameterOptions = {},
  ): ControlEditSet {
    const normalized = normalizeParameterEdit(keyOrEdit, value, options);
    const name = normalized.key;
    value = normalized.value;
    options = normalized.options;
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const row = descriptor?.parameters.find(
      (candidate) => candidate.name === name,
    ) as RuntimeParameterRow | undefined;
    if (row === undefined) {
      throw new MisoUsageError(`${this.#effectId} has no parameter '${String(name)}'`);
    }
    // The value is made a number the frame can hold; whether it is in the row's domain is the
    // core's to say.
    let scalar: number;
    if (row.domainName === "boolean") {
      if (typeof value !== "boolean") {
        throw new MisoUsageError(`${this.#effectId}.${row.name} must be boolean`);
      }
      scalar = flag(value);
    } else if (row.domainName === "enumeration") {
      const choice = row.enumChoices.find(
        (candidate) => candidate.label === (value as unknown as string),
      );
      if (choice === undefined) {
        throw new MisoUsageError(
          `${this.#effectId}.${row.name} must be one of ${row.enumChoices.map((item) => item.label).join(", ")}`,
        );
      }
      scalar = choice.value;
    } else {
      if (typeof value !== "number") {
        throw new MisoUsageError(`${this.#effectId}.${row.name} must be numeric`);
      }
      scalar = value;
    }
    return this.#edit(row.id, scalar, laneOf(options.channel), rampOf(options));
  }

  /**
   * Set or lift this instance's bypass (control `0` of the instance). The core crossfades it on
   * both hosts and refuses what it cannot do.
   */
  bypass(enabled: boolean): ControlEditSet {
    return this.#edit(0, flag(enabled), "both", undefined);
  }

  observe(tap: TapName<E>, armed: boolean, windowBlocks = 0): LaneEdit {
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const observation = descriptor?.observations.find((candidate) => candidate.name === tap);
    if (observation === undefined || !observation.subscribable) {
      throw new MisoUsageError(`${this.#effectId} has no subscribable tap '${String(tap)}'`);
    }
    if (this.#stripIndex === undefined || this.#effectIndex === undefined) {
      throw new MisoUsageError(
        `an observation record names its effect by index, and this ${this.#effectId} has none: `
          + "address it with liveControls().withSession(session) or effect(rack, index, effectId)",
      );
    }
    return observationEdit(armed ? "observeSubscribe" : "observeUnsubscribe", this.#stripIndex, {
      rack: RACKS[this.#rack],
      effectIndex: this.#effectIndex,
      parameterId: observation.id,
      smoothingSamples: requireU32(windowBlocks, "windowBlocks"),
    });
  }
}

function isEditSet(candidate: unknown): candidate is ControlEditSet {
  return candidate !== null && typeof candidate === "object"
    && Array.isArray((candidate as { readonly edits?: unknown }).edits);
}

/** One transaction-oriented set of live controls over either SDK transport. */
export class EngineLiveControls {
  readonly edit: LiveControlEdits;
  readonly #map: SessionMap;
  readonly #transport: LiveControlTransport;
  readonly #beforeSubmit: LiveControlBeforeSubmit | undefined;
  readonly #booted: Uint8Array | undefined;

  /**
   * `session` is the session the SDK built for this engine, when there is one; it is what lets
   * `edit.track(id)`'s or `edit.submix(id)`'s `.console(slot, ...)` and `.insert(id, ...)` check
   * IDs and indices. `booted` is the document the engine booted, exactly as it was staged; both SDK
   * engines supply it, and `withSession()` holds its session to it.
   */
  constructor(
    map: SessionMap,
    transport: LiveControlTransport,
    beforeSubmit?: LiveControlBeforeSubmit,
    session?: SessionLike,
    booted?: Uint8Array,
  ) {
    this.edit = new LiveControlEdits(map, session);
    this.#map = map;
    this.#transport = transport;
    this.#beforeSubmit = beforeSubmit;
    this.#booted = booted;
  }

  /**
   * The same live controls, resolving console slot and insert IDs against `session`.
   *
   * For an engine booted from document text: the SDK never parses a document, so the caller hands
   * over the builder that wrote it, and that builder must have written exactly the booted document:
   * its canonical `toJson()` must equal the booted bytes. Any other session -- its console
   * reordered, a slot renamed, a track's inserts changed -- is refused with a `MisoUsageError`,
   * before any edit is built, because its IDs would resolve to addresses that name other instances.
   */
  withSession(session: SessionLike): EngineLiveControls {
    if (this.#booted === undefined) {
      throw new MisoUsageError(
        "withSession() holds the session to the document the engine booted, and these live "
          + "controls were constructed without it; construct them with the booted document (the "
          + "constructor's `booted` argument) so the session can be checked against it",
      );
    }
    assertBootedSession(session, this.#booted);
    return new EngineLiveControls(this.#map, this.#transport, this.#beforeSubmit, session, this.#booted);
  }

  /** The engine's one transaction queue, which every value edit goes through. */
  get queue(): ControlTransactionQueue {
    return this.#transport.queue;
  }

  /** A writer that stages, coalesces and flushes `SET_CONTROL` edits through the engine's queue. */
  writer(options: Omit<ControlWriterOptions, "queue" | "submit" | "submitEdits"> = {}): LiveControlWriter {
    return new LiveControlWriter({ ...options, queue: this.#transport.queue });
  }

  /**
   * Send edits.
   *
   * Value edits (what the builders return) go in one `SET_CONTROL` transaction and resolve with the
   * core's applied report for the transaction that carried them; the core's refusal of them rejects
   * with `MisoControlRefusal`, whatever other callers staged beside them. Observation edits
   * (`observe()`) go as command records and resolve with a `CommandReport`. One call sends one
   * kind.
   */
  submit(...edits: readonly ControlEditSet[]): Promise<ControlAppliedReport>;
  submit(...edits: readonly LaneEdit[]): Promise<CommandReport>;
  async submit(
    ...edits: readonly (ControlEditSet | LaneEdit)[]
  ): Promise<ControlAppliedReport | CommandReport> {
    if (edits.length === 0) throw new MisoUsageError("a live-control submit needs at least one edit");
    const sets = edits.filter(isEditSet);
    if (sets.length === edits.length) {
      return this.#transport.queue.submit(sets.flatMap((set) => set.edits));
    }
    if (sets.length !== 0) {
      throw new MisoUsageError("one submit sends value edits or observation edits, not both");
    }
    return this.#submitObservation(edits as readonly LaneEdit[], false);
  }

  /** @internal Managed observation owner submission with an owner-specific guard bypass. */
  async submitManaged(...edits: readonly LaneEdit[]): Promise<CommandReport> {
    return this.#submitObservation(edits, true);
  }

  async #submitObservation(edits: readonly LaneEdit[], managed: boolean): Promise<CommandReport> {
    if (edits.length === 0 || edits.length > ABI_LAYOUT.constants.maximumCommandRecords) {
      throw new MisoUsageError(
        `an observation submit must contain 1..${ABI_LAYOUT.constants.maximumCommandRecords} edits`,
      );
    }
    if (edits.some((edit) => edit.kind !== "observeSubscribe" && edit.kind !== "observeUnsubscribe")) {
      throw new MisoUsageError("a command record carries an observation edit only; value edits are SET_CONTROL");
    }
    const beforeSubmit = this.#beforeSubmit?.(edits, managed);
    if (beforeSubmit !== undefined) await beforeSubmit;
    const report = await this.#transport.observe(edits);
    const resultConsistent = report.ok === (report.result === 0);
    const wholeBatch = report.ok
      ? report.admitted === edits.length
      : report.admitted === 0;
    if (!resultConsistent || !wholeBatch) {
      throw new MisoUsageError(
        `transport violated whole-batch admission: result=${report.result}, ok=${report.ok}, admitted=${report.admitted}, requested=${edits.length}`,
      );
    }
    return report;
  }
}

/** Type-only proof that the live-name filter is derived from the catalog's own rows. */
export type _AllEffectParameterNames<E extends EffectId> = EffectParameterName<E>;
