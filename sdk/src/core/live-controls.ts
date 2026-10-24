import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import { CONTROLS } from "../generated/controls.ts";
import type { ControlAddress, ControlLane, ControlOwner, ControlSection } from "../generated/controls.ts";
import {
  EffectControlBuilders,
  RouteEdits,
  SessionEdits,
  SubmixEdits,
  TrackEdits,
  VcaEdits,
} from "../generated/control-builders.ts";
import type { StripEdits } from "../generated/control-builders.ts";
import type {
  EffectId,
  EffectParameterName,
  TapName,
} from "../generated/catalog.ts";
import type { CommandReport, SessionMap } from "./boundary.ts";
import { stripEffectOf } from "./control-list.ts";
import type { ControlStripOrder } from "./control-list.ts";
import { validateControlEdit } from "./control-frame.ts";
import type { ControlFrameEdit, ControlWireAddress } from "./control-frame.ts";
import type { ControlAppliedReport, ControlTransactionQueue } from "./control-queue.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import { constantValue } from "./abi.ts";
import { writeCanonicalSessionDocument } from "../internal/session-json.ts";
import type { SessionLike, SessionModel } from "./session.ts";
import { RACKS } from "../internal/rack.ts";
import { requireU32 } from "../internal/ramp.ts";
import { LiveControlWriter } from "./writer.ts";
import type { ControlWriterOptions, LaneEdit } from "./writer.ts";
import { NO_LAYOUT, editSet, effectControlId, id } from "./control-builder-support.ts";
import type {
  ControlEditSet,
  ControlSetter,
  EffectInstance,
  InstanceRow,
  LayoutRow,
  LiveControlLayout,
  LiveControlRack,
  SetControlOptions,
  StripContext,
  StripInstances,
} from "./control-builder-support.ts";

export type {
  ChannelOptions,
  ControlEditSet,
  ControlSetter,
  InputFilterOptions,
  LaneOptions,
  LiveControlChannel,
  LiveControlLayout,
  LiveControlRack,
  LiveEffectParameterEdit,
  LiveEffectParameterName,
  LiveEffectParameterOptions,
  LiveEffectParameterValue,
  SetControlOptions,
  SmoothingOptions,
} from "./control-builder-support.ts";
// The builders are generated from the core's control tables (`src/generated/control-builders.ts`).
export { EffectControlBuilders, RouteEdits, SessionEdits, StripEdits, SubmixEdits, TrackEdits, VcaEdits }
  from "../generated/control-builders.ts";
export type { InputFilterValues, MatrixValues } from "../generated/control-builders.ts";

/** What live controls send through: the engine's one transaction queue, and the observation records. */
export interface LiveControlTransport {
  /** Every value edit goes here, as a `SET_CONTROL` transaction. */
  readonly queue: ControlTransactionQueue;
  /** Observation records (kinds 7 and 8): command records that name their tap by stable key (#1677). */
  readonly observe: (edits: readonly LaneEdit[]) => CommandReport | Promise<CommandReport>;
  /** The order the transport read when it made these live controls. */
  readonly order?: ControlStripOrder;
}

/**
 * Optional owner hook used to serialize observation edits with managed subscriptions. It sees the
 * observation edits as the builders made them.
 */
export type LiveControlBeforeSubmit = (
  edits: readonly LaneEdit[],
  managed?: boolean,
) => void | Promise<void>;

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

/** Whether the strips of `session` are the strips `map` names (false once a rebuild moved them). */
export function sessionDescribesStrips(session: SessionLike, map: SessionMap): boolean {
  const model = modelOf(session);
  return sameIds(model.tracks.map((track) => String(track.id)), map.tracks)
    && sameIds((model.submixes ?? []).map((submix) => String(submix.id)), map.submixes);
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
  return Object.freeze({ console: Object.freeze(slots), tracks, submixes });
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

function ownRow(
  table: readonly { readonly control: number; readonly name: string }[],
  name: string,
): number {
  const row = table.find((candidate) => candidate.name === name);
  if (row === undefined) throw new MisoUsageError(`the generated controls have no row ${name}`);
  return row.control;
}

function controlEdit(
  owner: ControlOwner,
  section: ControlSection,
  control: number,
  value: number,
  lane: ControlLane,
  rampSamples: number | undefined,
  expectedEffect: string | undefined,
): ControlFrameEdit {
  return Object.freeze({
    address: Object.freeze({
      owner,
      section,
      control,
      lane,
      // The effect the edit was written for; the core refuses it if the slot runs another (#1678).
      ...(expectedEffect === undefined ? {} : { expectedEffect }),
    }),
    value,
    // Absent when none was given: the core resolves the row's ramp.
    ...(rampSamples === undefined ? {} : { rampSamples }),
  });
}

/**
 * One observation record: the subscription or withdrawal of the tap `tapId` of the console slot or
 * insert `effectId` (its ID in the session) of strip `stripId`. The record names its tap by this
 * stable key and no position, so the core resolves it on the plan it admits the record against and
 * a rebuild cannot make it arm another effect's tap (#1677). Whether the key names a tap is the
 * core's to say, with a typed reason.
 */
export function observationEdit(
  armed: boolean,
  key: {
    readonly stripId: string;
    readonly rack: LiveControlRack;
    readonly effectId: string;
    readonly tapId: number;
  },
  windowBlocks: number,
): LaneEdit {
  return Object.freeze({
    kind: armed ? "observeSubscribe" : "observeUnsubscribe",
    rack: RACKS[key.rack],
    stripId: id(key.stripId, "a strip ID"),
    effectId: id(key.effectId, "an effect ID"),
    tapId: requireU32(key.tapId, "tapId"),
    windowBlocks: requireU32(windowBlocks, "windowBlocks"),
  });
}

/**
 * A semantic edit builder bound to the engine's session map.
 *
 * Builders address by stable ID and build `SET_CONTROL` edits; they check no value, no lane's
 * meaning and no index, and send nothing. An ID the engine does not have, a value outside a row's
 * domain and a lane a row does not allow come back from the core as its own `control.*` code, the
 * same on the browser and on the headless engine. An observation record names its tap by the
 * strip's ID, the effect's ID in the session and the tap's ID, and the core resolves them (#1677).
 *
 * Given the session the SDK built for this engine, it also resolves console slot IDs and insert IDs
 * on every strip (`console()`, `insert()`) against the booted session, and an effect addressed by
 * index (`effect()`) to its stable ID.
 */
export class LiveControlEdits {
  readonly #tracks: ReadonlySet<string>;
  readonly #submixes: ReadonlySet<string>;
  readonly #layout: LiveControlLayout | undefined;
  /** The core's strips and effects as the live controls were made; supplies the effect of a named section. */
  readonly #order: ControlStripOrder | undefined;
  /** What a strip reaches `set` and its effect builders through. */
  readonly #context: StripContext;

  constructor(map: SessionMap, session?: SessionLike, order?: ControlStripOrder) {
    this.#tracks = new Set(map.tracks);
    this.#submixes = new Set(map.submixes);
    this.#layout = session === undefined ? undefined : layoutOf(session, map);
    this.#order = order;
    this.#context = {
      set: (address, value, options) => this.set(address, value, options),
      effect: <E extends EffectId>(instance: EffectInstance<E>) => new EffectEdits(
        instance,
        (address, value, options) => this.set(address, value, options),
      ),
    };
  }

  /**
   * Set any control of the running session: one edit for one lane of one row, the sugar every typed
   * builder (`faderDb`, `gainDb`, an effect's `parameter`, ...) is built on. Returns the edit to pass
   * to `submit()` or a writer, in the same transaction as the typed builders' edits.
   *
   * `address` is either a {@link ControlAddress} (the owner, the section, the row's machine name and
   * the lane) or a {@link ControlWireAddress} whose `control` is the row's numeric ID, which is what
   * every row of `engine.controls()` carries as its `address`. A row of a console slot or an insert
   * is named by its machine name only when the effect it runs is known, which it is for a session
   * the SDK built (`withSession()`); otherwise name it by its numeric ID.
   *
   * An edit of a console slot or insert row carries the effect it was written for (the typed effect of
   * an effect builder, the effect of the section a row is named by, or `expectedEffect` of a numeric
   * address), and the core refuses it with `control.effect.changed` when the slot or insert runs
   * another effect now (#1678); an address that names no effect applies to whatever runs there.
   *
   * It checks only that the edit can be encoded (a name that is a row, an ID that is a string, a
   * ramp that is a `u32`). The value's domain, the lane, the owner and whether the row can be set
   * are the core's to say, and its refusal (`control.domain`, `control.read_only`,
   * `control.address.unknown`, `web.apply.rebuild_pending`, ...) comes back from `submit()`.
   */
  set(
    address: ControlAddress | ControlWireAddress,
    value: number,
    options: SetControlOptions = {},
  ): ControlEditSet {
    if (address === null || typeof address !== "object") {
      throw new MisoUsageError("set() needs a control address");
    }
    const rampSamples = options.rampSamples === undefined
      ? undefined
      : requireU32(options.rampSamples, "rampSamples");
    const named = typeof address.control !== "number";
    const { owner, section } = address;
    // A row of a console slot or an insert is sent with the effect it was resolved for: the typed
    // effect of an effect builder, the effect of the named section, or the one a numeric address
    // names itself. The core refuses the edit when the section runs another effect now (#1678).
    const expectedEffect = named && owner.kind === "strip"
      && (section.kind === "console" || section.kind === "inserts")
      ? this.#effectOf(owner, section)
      : (address as ControlWireAddress).expectedEffect;
    const edit = controlEdit(
      owner,
      section,
      named ? this.#rowId(address as ControlAddress) : address.control as number,
      value,
      address.lane,
      rampSamples,
      expectedEffect,
    );
    validateControlEdit(edit);
    return editSet(edit);
  }

  /** The numeric ID of the row a named address names. */
  #rowId(address: ControlAddress): number {
    const { owner, section, control: name } = address;
    const table = (
      rows: readonly { readonly control: number; readonly name: string }[],
    ): number => ownRow(rows, name);
    switch (section.kind) {
      case "own":
        switch (owner.kind) {
          case "session":
            return table(CONTROLS.sessionOwn);
          case "strip":
            return table(CONTROLS.stripOwn);
          case "route":
            return table(CONTROLS.routeOwn);
          case "vca":
            return table(CONTROLS.vcaOwn);
        }
        break;
      case "builtins":
        return table(CONTROLS.builtins);
      case "console":
      case "inserts":
        return this.#effectRowId(owner, section, name);
    }
    // Only a caller outside the types reaches here: the owner or section kind is not one the
    // protocol has.
    throw new MisoUsageError(
      `cannot name a control of owner '${String((owner as { kind?: unknown }).kind)}' `
        + `section '${String((section as { kind?: unknown }).kind)}'`,
    );
  }

  #effectRowId(owner: ControlOwner, section: ControlSection, name: string): number {
    const effectId = this.#effectOf(owner, section);
    if (effectId === undefined) {
      throw new MisoUsageError(
        `the control '${name}' of ${section.kind === "console" ? `console slot '${section.slot}'` : section.kind === "inserts" ? `insert '${section.effect}'` : section.kind}`
          + " is named, and its numeric ID depends on the effect it runs, which the engine's control "
          + "list does not name for this section (is the ID right?); address it by its numeric "
          + "row ID instead (the `address` of its row in engine.controls())",
      );
    }
    return effectControlId(effectId, name);
  }

  /**
   * The effect a strip's console slot or insert runs: the core's section header when the list is
   * known (it follows a rebuild), else the session the SDK built for this engine.
   */
  #effectOf(owner: ControlOwner, section: ControlSection): string | undefined {
    if (owner.kind !== "strip" || (section.kind !== "console" && section.kind !== "inserts")) return undefined;
    const listed = this.#order === undefined ? undefined : stripEffectOf(this.#order, owner.id, section);
    if (listed !== undefined) return listed;
    return this.#layout === undefined ? undefined : this.#instanceOf(this.#layout, owner.id, section)?.effectId;
  }

  #instanceOf(
    layout: LiveControlLayout,
    stripId: string,
    section: ControlSection,
  ): LayoutRow | undefined {
    const instances = layout.tracks.get(stripId) ?? layout.submixes.get(stripId);
    if (section.kind === "console") return instances?.console.find((row) => row.id === section.slot);
    if (section.kind === "inserts") return instances?.inserts.find((row) => row.id === section.effect);
    return undefined;
  }

  /**
   * A track strip's live edits. The ID is not checked here: an unknown strip is refused by the core
   * with `control.address.unknown`.
   */
  track(trackId: string): TrackEdits {
    return new TrackEdits(
      id(trackId, "a track ID"),
      this.#layout,
      this.#context,
    );
  }

  /**
   * A submix strip's live edits: everything a track's strip offers except `solo` (a bus is
   * solo-safe; the core refuses a solo of a submix).
   */
  submix(submixId: string): SubmixEdits {
    return new SubmixEdits(
      id(submixId, "a submix ID"),
      this.#layout,
      this.#context,
    );
  }

  /**
   * Either kind of strip by its ID -- a track, else a submix (strip IDs are unique across both) --
   * for an edit every strip shares, such as arming an effect observation. It never offers `solo`.
   * An ID that is neither is refused here.
   */
  strip(stripId: string): StripEdits {
    if (this.#tracks.has(stripId)) return this.track(stripId);
    if (this.#submixes.has(stripId)) return this.submix(stripId);
    throw new MisoUsageError(
      `the compiled session has no track or submix '${stripId}'; expected one of `
        + `${[...this.#tracks, ...this.#submixes].join(", ")}`,
    );
  }

  /**
   * A live send's edits, by its route ID. The ID is not checked here: a route the core does not
   * enumerate (an unknown ID, a route into the output) is refused with `control.address.unknown`.
   */
  route(routeId: string): RouteEdits {
    return new RouteEdits(id(routeId, "a route ID"), this.#context.set);
  }

  /**
   * A VCA's live edits, by its VCA ID: its per-lane fader offset and mute. The ID is not checked
   * here: an unknown VCA is refused by the core with `control.address.unknown`.
   */
  vca(vcaId: string): VcaEdits {
    return new VcaEdits(id(vcaId, "a VCA ID"), this.#context.set);
  }

  /** The session's own live settings: the lengths of its default fader, mute and pan ramps. */
  session(): SessionEdits {
    return new SessionEdits(this.#context.set);
  }
}

/**
 * Catalog-derived edits for one effect instance: the generated parameter, bypass and link-mode
 * builders over this instance's address, and the observation records.
 */
export class EffectEdits<E extends EffectId> extends EffectControlBuilders<E> {
  readonly #stripId: string;
  readonly #rack: LiveControlRack;
  readonly #stableId: string | undefined;
  readonly #set: ControlSetter;
  protected readonly effectId: E;

  /**
   * `instance.stableId` is the instance's console slot ID or insert ID, which a control address and
   * an observation record name it by. It is unknown for an instance named by index without a
   * session to resolve the index against.
   */
  constructor(
    instance: EffectInstance<E>,
    set: ControlSetter,
  ) {
    super();
    this.#stripId = instance.stripId;
    this.#rack = instance.rack;
    this.#stableId = instance.stableId;
    this.effectId = instance.effectId;
    this.#set = set;
  }

  #section(): ControlSection {
    const stable = this.#stableId;
    if (stable === undefined) {
      throw new MisoUsageError(
        `this ${this.effectId} was addressed by index, and a value edit names an instance by its `
          + `stable ID: ${NO_LAYOUT}`,
      );
    }
    return this.#rack === "console"
      ? { kind: "console", slot: stable }
      : { kind: "inserts", effect: stable };
  }

  protected setRow(
    control: string,
    value: number,
    lane: ControlLane,
    options?: SetControlOptions,
  ): ControlEditSet {
    // The row is one of this effect's, by the machine name the generated table gives it; the
    // instance knows its effect, so the address carries the row's number and the effect it was
    // resolved for, which the core holds the edit to (#1678).
    return this.#set(
      {
        owner: { kind: "strip", id: this.#stripId },
        section: this.#section(),
        control: effectControlId(this.effectId, control),
        lane,
        expectedEffect: this.effectId,
      },
      value,
      options,
    );
  }

  observe(tap: TapName<E>, armed: boolean, windowBlocks = 0): LaneEdit {
    const descriptor = CATALOG.effects.find((row) => row.id === this.effectId);
    const observation = descriptor?.observations.find((candidate) => candidate.name === tap);
    if (observation === undefined || !observation.subscribable) {
      throw new MisoUsageError(`${this.effectId} has no subscribable tap '${String(tap)}'`);
    }
    const stable = this.#stableId;
    if (stable === undefined) {
      throw new MisoUsageError(
        `an observation record names its effect by its stable ID, and this ${this.effectId} was `
          + `addressed by index with nothing to resolve it against: ${NO_LAYOUT}`,
      );
    }
    return observationEdit(armed, {
      stripId: this.#stripId, rack: this.#rack, effectId: stable, tapId: observation.id,
    }, windowBlocks);
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
    this.edit = new LiveControlEdits(map, session, transport.order);
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
    // After a rebuild moved the strips the engine is no longer the booted session: the names then
    // resolve from the core's control list alone.
    const describes = sessionDescribesStrips(session, this.#map);
    return new EngineLiveControls(
      this.#map,
      this.#transport,
      this.#beforeSubmit,
      describes ? session : undefined,
      this.#booted,
    );
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

