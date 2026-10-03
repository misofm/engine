import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import type {
  EffectId,
  EffectDescriptor,
  EffectParameter,
  EffectParameterName,
  TapName,
} from "../generated/catalog.ts";
import type { CommandReport, SessionMap } from "./boundary.ts";
import { MisoUsageError } from "./errors.ts";
import { writeCanonicalSessionDocument } from "../internal/session-json.ts";
import type { SessionLike, SessionModel } from "./session.ts";
import type { LaneEdit } from "./writer.ts";

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
  readonly smoothingSamples?: number;
}

export interface LaneOptions extends SmoothingOptions {
  readonly channel?: LiveControlChannel;
}

/** The two cutoff values carried atomically by one prepared input-filter command. */
export interface InputFilterValues {
  readonly hpfHz: number;
  readonly lpfHz: number;
}

/** Input-filter edits select a lane but use the builtin fixed 64-update policy. */
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

export type LiveControlSubmit = (
  edits: readonly LaneEdit[],
) => CommandReport | Promise<CommandReport>;

/** Optional owner hook used to serialize observation edits with managed subscriptions. */
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
 * The command record's rack byte, from the generated layout's `racks` table (S1c): `1` inserts,
 * `3` console and `255` not applicable. The retired `0` and `2` are never written.
 */
const RACKS: Readonly<Record<LiveControlRack, number>> = Object.freeze({
  console: rackCode("console"),
  inserts: rackCode("inserts"),
});
const CHANNELS = Object.freeze({ left: 0, right: 1, both: 2 } as const);
const NONE = rackCode("notApplicable");

/**
 * The native effects whose session bypass stays a prepared bypass (S1c; issues #1087, #1100).
 *
 * Every other launch effect lowers a session bypass to a per-lane shunt, so a live bypass edit
 * both sets and lifts it. These two do not: the delay never banks (the engine's
 * `effect_compiler::NEVER_BANKED_EFFECTS`), and the multiband keeps its prepared bypass so a
 * bypassed lane cannot silence its bank-mates (`PREPARED_BYPASS_EFFECTS`). A live un-bypass of a
 * session-bypassed instance of either is admitted by the engine and changes nothing, so
 * `EffectEdits.bypass(false)` refuses it whenever the SDK knows the instance's authored bypass.
 *
 * The metadata does not publish the list, so the SDK holds a copy, and `console-evals.mjs` holds
 * the copy to the engine: it lifts a session bypass live on every catalog effect and requires the
 * render to stay bypassed on exactly these.
 */
export const PREPARED_BYPASS_EFFECTS: readonly EffectId[] = Object.freeze([
  "miso.delay",
  "miso.multiband-compressor",
]);

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

function u32(value: number, name: string): number {
  if (!Number.isSafeInteger(value) || value < 0 || value > 0xffff_ffff) {
    throw new MisoUsageError(`${name} must be a u32`);
  }
  return value;
}

function smoothing(options: SmoothingOptions): number {
  return u32(options.smoothingSamples ?? 0, "smoothingSamples");
}

type RuntimeParameterOptions = {
  readonly channel?: unknown;
  readonly smoothingSamples?: unknown;
};

function runtimeSmoothing(options: RuntimeParameterOptions): number {
  const sampleCount = options.smoothingSamples ?? 0;
  if (typeof sampleCount !== "number") {
    throw new MisoUsageError("smoothingSamples must be a number");
  }
  return u32(sampleCount, "smoothingSamples");
}

function runtimeChannel(channel: unknown): LiveControlChannel {
  if (channel === undefined || channel === "both") return "both";
  if (channel === "left" || channel === "right") return channel;
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

function finite(value: number, name: string, minimum?: number, maximum?: number): number {
  if (!Number.isFinite(value)) throw new MisoUsageError(`${name} must be finite`);
  if (minimum !== undefined && value < minimum) {
    throw new MisoUsageError(`${name} must be at least ${minimum}`);
  }
  if (maximum !== undefined && value > maximum) {
    throw new MisoUsageError(`${name} must be at most ${maximum}`);
  }
  return value;
}

function builtinNumber(name: string, value: number): number {
  const row = CATALOG.builtins.parameters.find((candidate) => candidate.name === name);
  if (row === undefined || !row.liveUpdatable) {
    throw new MisoUsageError(`the generated catalog has no live builtin ${name}`);
  }
  return finite(
    value,
    name,
    row.minimum ?? undefined,
    typeof row.maximum === "number" ? row.maximum : undefined,
  );
}

function values(a = 0, b = 0, c = 0, d = 0): readonly [number, number, number, number] {
  return Object.freeze([a, b, c, d]);
}

function lane(options: LaneOptions): number {
  return CHANNELS[options.channel ?? "both"];
}

function trackEdit(
  kind: LaneEdit["kind"],
  trackIndex: number,
  options: {
    readonly rack?: number;
    readonly channel?: number;
    readonly effectIndex?: number;
    readonly parameterId?: number;
    readonly smoothingSamples?: number;
    readonly values?: readonly [number, number, number, number];
  } = {},
): LaneEdit {
  return Object.freeze({
    kind,
    trackIndex,
    rack: options.rack ?? NONE,
    channel: options.channel ?? NONE,
    effectIndex: options.effectIndex ?? 0,
    parameterId: options.parameterId ?? 0,
    smoothingSamples: options.smoothingSamples ?? 0,
    values: options.values ?? values(),
  });
}

/**
 * A semantic edit builder bound to the engine's canonical session map.
 *
 * A record's index word is a strip index: the tracks in canonical order (`0..T`), then the
 * submixes (`T + j` for `SessionMap.submixes[j]`). `track(id)` and `submix(id)` resolve an ID in
 * their own list only, so neither can reach the other kind of strip.
 *
 * Given the session the SDK built for this engine, it also resolves console slot IDs and insert
 * IDs to live addresses (`console()`, `insert()`) on every strip. Without one, effects are
 * addressed by index through `effect()`.
 */
export class LiveControlEdits {
  readonly #tracks: ReadonlyMap<string, number>;
  readonly #submixes: ReadonlyMap<string, number>;
  readonly #routes: ReadonlyMap<string, number>;
  readonly #vcas: ReadonlyMap<string, number>;
  readonly #layout: LiveControlLayout | undefined;

  constructor(map: SessionMap, session?: SessionLike) {
    this.#tracks = new Map(map.tracks.map((id, index) => [id, index] as const));
    const trackCount = map.tracks.length;
    this.#submixes = new Map(map.submixes.map((id, index) => [id, trackCount + index] as const));
    // Issue #1223 D4: a send's index is its position in the engine's own enumeration, never a
    // sort of the session's routes here.
    this.#routes = new Map(map.routes.map((id, index) => [id, index] as const));
    // Issue #1246 D4: a VCA's index is its position in the engine's own enumeration, never a sort
    // of the session's VCAs here.
    this.#vcas = new Map(map.vcas.map((id, index) => [id, index] as const));
    this.#layout = session === undefined ? undefined : layoutOf(session, map);
  }

  track(trackId: string): TrackEdits {
    const index = this.#tracks.get(trackId);
    if (index === undefined) {
      throw new MisoUsageError(
        `the compiled session has no track '${trackId}'; expected one of ${[...this.#tracks.keys()].join(", ")}`,
      );
    }
    return new TrackEdits(index, trackId, this.#layout);
  }

  /**
   * A submix strip's live edits: everything a track's strip offers except `solo` (a bus is
   * solo-safe; the engine refuses a solo at a submix index with `notSoloable`).
   */
  submix(submixId: string): SubmixEdits {
    const index = this.#submixes.get(submixId);
    if (index === undefined) {
      throw new MisoUsageError(
        `the compiled session has no submix '${submixId}'; expected one of `
          + `${[...this.#submixes.keys()].join(", ") || "none"}`,
      );
    }
    return new SubmixEdits(index, submixId, this.#layout);
  }

  /**
   * Either kind of strip by its ID -- a track, else a submix (strip IDs are unique across both) --
   * for an edit every strip shares, such as arming an effect observation. It never offers `solo`.
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
   * A live send's edits, by its route ID (issue #1223 D4). Only routes into submixes are live; a
   * route into the output changes only through the session, and is refused here as the engine
   * would refuse its index, with `unknownRoute`.
   */
  route(routeId: string): RouteEdits {
    const index = this.#routes.get(routeId);
    if (index === undefined) {
      if (this.#layout?.outputRoutes.has(routeId) === true) {
        throw new MisoUsageError(
          `route '${routeId}' goes to the output, and output routes are not live: only routes into `
            + "submixes (sends) take live edits; change an output route through the session",
          "unknownRoute",
        );
      }
      throw new MisoUsageError(
        `the compiled session has no live route '${routeId}'; expected one of `
          + `${[...this.#routes.keys()].join(", ") || "none"}`,
        "unknownRoute",
      );
    }
    return new RouteEdits(index);
  }

  /**
   * A VCA's live edits, by its VCA ID (issue #1246 D4): its per-lane fader offset and mute. An ID
   * the engine did not enumerate -- every ID without live controls -- is refused as the engine
   * would refuse its index, with `unknownVca`.
   */
  vca(vcaId: string): VcaEdits {
    const index = this.#vcas.get(vcaId);
    if (index === undefined) {
      throw new MisoUsageError(
        `the compiled session has no VCA '${vcaId}'; expected one of `
          + `${[...this.#vcas.keys()].join(", ") || "none"}`,
        "unknownVca",
      );
    }
    return new VcaEdits(index);
  }
}

const NO_LAYOUT = "this engine was not booted from a session the SDK built, so there is nothing to "
  + "resolve IDs against (the SDK never parses a document); call liveControls().withSession(session) "
  + "with the builder, or address the effect by index with effect()";

/**
 * Every live edit a strip -- a track or a submix -- shares. Methods build data and never mutate
 * the engine. `TrackEdits` adds `solo`; `SubmixEdits` adds nothing.
 */
export abstract class StripEdits {
  /** The record's index word: a strip index, the tracks then the submixes. */
  protected readonly stripIndex: number;
  /** `track 'kick'` or `submix 'drums'`, for messages. */
  readonly #label: string;
  readonly #layout: LiveControlLayout | undefined;
  readonly #instances: StripInstances | undefined;

  protected constructor(kind: StripKind, stripIndex: number, stripId?: string, layout?: LiveControlLayout) {
    this.stripIndex = stripIndex;
    this.#label = `${kind} '${stripId}'`;
    this.#layout = layout;
    this.#instances = stripId === undefined
      ? undefined
      : (kind === "track" ? layout?.tracks : layout?.submixes)?.get(stripId);
  }

  pan(left: number, right: number, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("pan", this.stripIndex, {
      smoothingSamples: smoothing(options),
      values: values(
        builtinNumber("matrix_ll", left),
        builtinNumber("matrix_rr", right),
      ),
    });
  }

  matrix(matrix: MatrixValues, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("matrix", this.stripIndex, {
      smoothingSamples: smoothing(options),
      values: values(
        builtinNumber("matrix_ll", matrix.ll),
        builtinNumber("matrix_lr", matrix.lr),
        builtinNumber("matrix_rl", matrix.rl),
        builtinNumber("matrix_rr", matrix.rr),
      ),
    });
  }

  faderDb(db: number, options: LaneOptions = {}): LaneEdit {
    return trackEdit("faderDb", this.stripIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(builtinNumber("fader_db", db)),
    });
  }

  mute(enabled: boolean, options: LaneOptions = {}): LaneEdit {
    return trackEdit("mute", this.stripIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  trimDb(db: number, options: LaneOptions = {}): LaneEdit {
    return trackEdit("trimDb", this.stripIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(builtinNumber("trim_db", db)),
    });
  }

  polarityInvert(enabled: boolean, options: LaneOptions = {}): LaneEdit {
    return trackEdit("polarityInvert", this.stripIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  /** Set one lane's HPF cutoff through the prepared input-filter owner. */
  hpfHz(value: number, options: InputFilterOptions = {}): LaneEdit {
    return trackEdit("inputFilters", this.stripIndex, {
      channel: lane(options),
      parameterId: 3,
      values: values(builtinNumber("hpf_hz", value)),
    });
  }

  /** Set one lane's LPF cutoff through the prepared input-filter owner. */
  lpfHz(value: number, options: InputFilterOptions = {}): LaneEdit {
    return trackEdit("inputFilters", this.stripIndex, {
      channel: lane(options),
      parameterId: 4,
      values: values(builtinNumber("lpf_hz", value)),
    });
  }

  /** Set both cutoffs as one indivisible wire edit. */
  inputFilters(filters: InputFilterValues, options: InputFilterOptions = {}): LaneEdit {
    if (filters === null || typeof filters !== "object" || Array.isArray(filters)) {
      throw new MisoUsageError("inputFilters requires an object with hpfHz and lpfHz");
    }
    return trackEdit("inputFilters", this.stripIndex, {
      channel: lane(options),
      values: values(
        builtinNumber("hpf_hz", filters.hpfHz),
        builtinNumber("lpf_hz", filters.lpfHz),
      ),
    });
  }

  /**
   * Address an effect by its live address: a console slot by its index in the session's slot order
   * (`pre_insert`, then `post_insert`), or an insert by its index in the strip's chain.
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
    const index = u32(effectIndex, "effectIndex");
    // With a layout, the instance at this address is known, whatever `effectId` claims, so its
    // authored bypass is carried to `bypass()`.
    const instance = this.#instances?.[rack][index];
    return new EffectEdits(
      this.stripIndex,
      RACKS[rack],
      index,
      effectId,
      instance === undefined ? undefined : authoredInstance(rack, instance, this.#label),
    );
  }

  /**
   * Address a session console slot by its stable slot ID.
   *
   * The ID resolves to the slot's index in the session's slot order, which is the live address; the
   * section is not part of it. `effectId` must be the slot's own effect, and it types the edits.
   */
  console<E extends EffectId>(slot: string, effectId: E): EffectEdits<E> {
    if (this.#layout === undefined) throw new MisoUsageError(`console('${slot}'): ${NO_LAYOUT}`);
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
      const row = this.#instances?.inserts[insert];
      if (this.#layout !== undefined && row === undefined) {
        throw new MisoUsageError(`${this.#label} has no insert at index ${insert}`);
      }
      return row === undefined
        ? this.effect("inserts", insert, effectId)
        : this.#resolved("inserts", insert, row, effectId);
    }
    if (this.#layout === undefined) throw new MisoUsageError(`insert('${insert}'): ${NO_LAYOUT}`);
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
  constructor(trackIndex: number, trackId?: string, layout?: LiveControlLayout) {
    super("track", trackIndex, trackId, layout);
  }

  solo(enabled: boolean, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("solo", this.stripIndex, {
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }
}

/**
 * A submix strip's live edits: every strip edit and no `solo`. `stripIndex` is the submix's strip
 * index, `T + j`; `LiveControlEdits.submix(id)` computes it from the session map.
 */
export class SubmixEdits extends StripEdits {
  constructor(stripIndex: number, submixId?: string, layout?: LiveControlLayout) {
    super("submix", stripIndex, submixId, layout);
  }
}

/**
 * A live send's edits (issue #1223 D4): its gain, its mute and its 2x2 matrix, each over an
 * optional `smoothingSamples`. `routeIndex` is the send's live-route index, its position in
 * `SessionMap.routes`; `LiveControlEdits.route(id)` computes it. The record's `rack` and
 * `channel` are not applicable (`255`). The engine holds the gain and the matrix to the prepared
 * route's domain and refuses a value outside it with `domain`.
 */
export class RouteEdits {
  readonly #routeIndex: number;

  constructor(routeIndex: number) {
    this.#routeIndex = u32(routeIndex, "routeIndex");
  }

  gainDb(db: number, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("routeGainDb", this.#routeIndex, {
      smoothingSamples: smoothing(options),
      values: values(finite(db, "gainDb")),
    });
  }

  /** `true` silences the send, `false` opens it. */
  mute(enabled: boolean, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("routeMute", this.#routeIndex, {
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }

  /** The send's 2x2 matrix, written `ll, lr, rl, rr`. */
  matrix(matrix: MatrixValues, options: SmoothingOptions = {}): LaneEdit {
    return trackEdit("routeMatrix", this.#routeIndex, {
      smoothingSamples: smoothing(options),
      values: values(
        finite(matrix.ll, "matrix.ll"),
        finite(matrix.lr, "matrix.lr"),
        finite(matrix.rl, "matrix.rl"),
        finite(matrix.rr, "matrix.rr"),
      ),
    });
  }
}

/**
 * A VCA's live edits (issue #1246 D4): its per-lane fader offset and mute, built exactly as a
 * strip's `faderDb` and `mute` are, over an optional lane and `smoothingSamples`. `vcaIndex` is
 * the VCA's position in `SessionMap.vcas`; `LiveControlEdits.vca(id)` computes it. The record's
 * `rack` is not applicable (`255`). A VCA has no audio path: the engine adds the offset to every
 * member's own fader, and its mute mutes every member.
 */
export class VcaEdits {
  readonly #vcaIndex: number;

  constructor(vcaIndex: number) {
    this.#vcaIndex = u32(vcaIndex, "vcaIndex");
  }

  /** The VCA's offset in dB, in the builtin fader's own domain. */
  faderDb(db: number, options: LaneOptions = {}): LaneEdit {
    return trackEdit("vcaFaderDb", this.#vcaIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(builtinNumber("fader_db", db)),
    });
  }

  /** `true` mutes every member on the selected lanes, `false` lifts the VCA's mute. */
  mute(enabled: boolean, options: LaneOptions = {}): LaneEdit {
    return trackEdit("vcaMute", this.#vcaIndex, {
      channel: lane(options),
      smoothingSamples: smoothing(options),
      values: values(enabled ? 1 : 0),
    });
  }
}

/**
 * What the session authored for the instance a live edit addresses, when the SDK knows it: the
 * instance's real effect (whatever the caller's `effectId` claims), its session bypass, and a name
 * for messages.
 */
export interface AuthoredInstance {
  readonly effectId: string;
  readonly bypass: boolean;
  readonly label: string;
}

function authoredInstance(rack: LiveControlRack, row: InstanceRow, strip: string): AuthoredInstance {
  return Object.freeze({
    effectId: row.effectId,
    bypass: row.bypass,
    label: `${rack === "console" ? "console slot" : "insert"} '${row.id}' on ${strip}`,
  });
}

/** Catalog-derived edits for one effect instance. */
export class EffectEdits<E extends EffectId> {
  readonly #trackIndex: number;
  readonly #rack: number;
  readonly #effectIndex: number;
  readonly #effectId: E;
  readonly #authored: AuthoredInstance | undefined;

  /**
   * `authored` is the addressed instance as the booted session declares it, when the SDK has that
   * session; `StripEdits` supplies it. Without it, `bypass(false)` cannot know the instance keeps a
   * prepared bypass.
   */
  constructor(trackIndex: number, rack: number, effectIndex: number, effectId: E, authored?: AuthoredInstance) {
    this.#trackIndex = trackIndex;
    this.#rack = rack;
    this.#effectIndex = effectIndex;
    this.#effectId = effectId;
    this.#authored = authored;
  }

  parameter<N extends LiveEffectParameterName<E>>(
    name: N,
    value: LiveEffectParameterValue<E, N>,
    options?: LiveEffectParameterOptions<E, N>,
  ): LaneEdit;
  parameter(edit: LiveEffectParameterEdit<E>): LaneEdit;
  parameter<N extends LiveEffectParameterName<E>>(
    keyOrEdit: string | LiveEffectParameterEdit<E>,
    value?: unknown,
    options: RuntimeParameterOptions = {},
  ): LaneEdit {
    const normalized = normalizeParameterEdit(keyOrEdit, value, options);
    const name = normalized.key;
    value = normalized.value;
    options = normalized.options;
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const row = descriptor?.parameters.find(
      (candidate) => candidate.name === name,
    ) as RuntimeParameterRow | undefined;
    if (row === undefined || !row.liveUpdatable) {
      throw new MisoUsageError(`${this.#effectId}.${String(name)} is not live-updatable`);
    }
    let scalar: number;
    if (row.domainName === "boolean") {
      if (typeof value !== "boolean") {
        throw new MisoUsageError(`${this.#effectId}.${row.name} must be boolean`);
      }
      scalar = value ? 1 : 0;
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
      scalar = finite(
        value,
        `${this.#effectId}.${row.name}`,
        row.minimum ?? undefined,
        row.maximum ?? undefined,
      );
    }
    const requestedChannel = runtimeChannel(options.channel);
    if (row.channelPolicyName === "shared" && requestedChannel !== "both") {
      throw new MisoUsageError(`${this.#effectId}.${row.name} is shared and must address both lanes`);
    }
    return trackEdit("effectParam", this.#trackIndex, {
      rack: this.#rack,
      channel: CHANNELS[requestedChannel],
      effectIndex: this.#effectIndex,
      parameterId: row.id,
      smoothingSamples: runtimeSmoothing(options),
      values: values(scalar),
    });
  }

  /**
   * Set or lift this instance's bypass, live, through the latency-preserving shunt.
   *
   * It can lift a bypass the session authored too, except on the effects in
   * `PREPARED_BYPASS_EFFECTS` -- the delay and the multiband compressor -- which keep a
   * session bypass as a prepared one: the engine admits a live un-bypass of either and renders
   * nothing different. So when the SDK knows the addressed instance (the engine booted from a
   * builder, or `withSession()` supplied it), `bypass(false)` on a session-bypassed delay or
   * multiband throws a `MisoUsageError` before anything is sent, rather than let an `ok`
   * acknowledgement stand for a no-op. Author the instance unbypassed and reload the session to
   * hear it. Addressed by index with no session, the SDK cannot know, and the lift is sent.
   */
  bypass(enabled: boolean): LaneEdit {
    const authored = this.#authored;
    if (!enabled && authored !== undefined && authored.bypass
      && PREPARED_BYPASS_EFFECTS.includes(authored.effectId as EffectId)) {
      throw new MisoUsageError(
        `${authored.label} is a ${authored.effectId} the session bypassed, and it keeps its prepared `
          + "bypass: the engine would acknowledge a live un-bypass and render nothing different. "
          + "Author it with bypass false and reload the session to hear it",
      );
    }
    return trackEdit("effectBypass", this.#trackIndex, {
      rack: this.#rack,
      effectIndex: this.#effectIndex,
      values: values(enabled ? 1 : 0),
    });
  }

  observe(tap: TapName<E>, armed: boolean, windowBlocks = 0): LaneEdit {
    const descriptor = CATALOG.effects.find((row) => row.id === this.#effectId);
    const observation = descriptor?.observations.find((candidate) => candidate.name === tap);
    if (observation === undefined || !observation.subscribable) {
      throw new MisoUsageError(`${this.#effectId} has no subscribable tap '${String(tap)}'`);
    }
    return trackEdit(armed ? "observeSubscribe" : "observeUnsubscribe", this.#trackIndex, {
      rack: this.#rack,
      effectIndex: this.#effectIndex,
      parameterId: observation.id,
      smoothingSamples: u32(windowBlocks, "windowBlocks"),
    });
  }
}

/** One transaction-oriented set of live controls over either SDK transport. */
export class EngineLiveControls {
  readonly edit: LiveControlEdits;
  readonly #map: SessionMap;
  readonly #submit: LiveControlSubmit;
  readonly #beforeSubmit: LiveControlBeforeSubmit | undefined;
  readonly #booted: Uint8Array | undefined;

  /**
   * `session` is the session the SDK built for this engine, when there is one; it is what lets
   * `edit.track(id)`'s or `edit.submix(id)`'s `.console(slot, ...)` and `.insert(id, ...)` resolve
   * stable IDs. `booted` is the
   * document the engine booted, exactly as it was staged; both SDK engines supply it, and
   * `withSession()` holds its session to it.
   */
  constructor(
    map: SessionMap,
    submit: LiveControlSubmit,
    beforeSubmit?: LiveControlBeforeSubmit,
    session?: SessionLike,
    booted?: Uint8Array,
  ) {
    this.edit = new LiveControlEdits(map, session);
    this.#map = map;
    this.#submit = submit;
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
    return new EngineLiveControls(this.#map, this.#submit, this.#beforeSubmit, session, this.#booted);
  }

  async submit(...edits: readonly LaneEdit[]): Promise<CommandReport> {
    return this.#submitChecked(edits, false);
  }

  /** @internal Managed observation owner submission with an owner-specific guard bypass. */
  async submitManaged(...edits: readonly LaneEdit[]): Promise<CommandReport> {
    return this.#submitChecked(edits, true);
  }

  async #submitChecked(edits: readonly LaneEdit[], managed: boolean): Promise<CommandReport> {
    if (edits.length === 0 || edits.length > ABI_LAYOUT.constants.maximumCommandRecords) {
      throw new MisoUsageError(
        `a live-control transaction must contain 1..${ABI_LAYOUT.constants.maximumCommandRecords} edits`,
      );
    }
    const beforeSubmit = this.#beforeSubmit?.(edits, managed);
    if (beforeSubmit !== undefined) await beforeSubmit;
    const report = await this.#submit(edits);
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
