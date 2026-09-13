import { ABI_LAYOUT } from "../generated/abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import type { CommandReport } from "./boundary.ts";
import { EngineConsole } from "./console.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import {
  MAXIMUM_OBSERVATION_READS,
  validateObservationSelections,
  type ObservationBinding,
  type ObservationChannels,
  type ObservationMap,
  type ObservationReadResult,
  type ObservationSelection,
} from "./observation.ts";
import type { LaneEdit } from "./writer.ts";

/** The bounded configuration accepted by `subscribeObservations`. */
export interface ObservationSubscriptionLimits {
  readonly maximumHandles?: number;
  readonly maximumBindings?: number;
  readonly maximumSelections?: number;
  readonly maximumWindowBlocks?: number;
  readonly maximumCadenceMs?: number;
}

/** The finite SDK-side limits used when the caller does not provide overrides. */
export const DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS = Object.freeze({
  maximumHandles: 64,
  maximumBindings: MAXIMUM_OBSERVATION_READS,
  maximumSelections: MAXIMUM_OBSERVATION_READS,
  maximumWindowBlocks: 4_096,
  maximumCadenceMs: 60_000,
});

export interface ObservationSubscriptionRequest {
  readonly selections: readonly ObservationSelection[];
  readonly windowBlocks: number;
  /** Browser notification cadence. Headless retains the value for the acknowledgement. */
  readonly cadenceMs?: number;
  /** Receives availability notifications; call `handle.readLatest()` for owned rows. */
  readonly onUpdate?: (notification: ObservationSubscriptionNotification) => void;
}

export interface ObservationSubscriptionConfiguration {
  readonly selections: readonly ObservationSelection[];
  readonly windowBlocks: number;
  readonly cadenceMs: number;
}

export interface ObservationSubscriptionBounds {
  readonly maximumHandles: number;
  readonly maximumBindings: number;
  readonly maximumSelections: number;
  readonly maximumWindowBlocks: number;
  readonly maximumCadenceMs: number;
  readonly maximumReadSelections: number;
  readonly maximumCommandRecords: number;
}

export interface ObservationSubscriptionNotification {
  readonly handle: ObservationSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly available: boolean;
  /** Windows the native owner advanced past between two reads in this epoch. */
  readonly nativeMissedWindows: bigint;
  /** Published rows a handle did not receive between its delivery cursors. */
  readonly skippedPublications: bigint;
}

export interface ObservationSubscriptionReceipt {
  readonly handle: ObservationSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly configuration: ObservationSubscriptionConfiguration;
  readonly appliedAtSample: bigint;
  readonly bounds: ObservationSubscriptionBounds;
}

export interface ObservationSubscription {
  /** The handle itself; present so callers can treat subscribe's direct return as a receipt. */
  readonly handle: ObservationSubscription;
  readonly id: bigint;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly configuration: ObservationSubscriptionConfiguration;
  readonly appliedAtSample: bigint;
  readonly bounds: ObservationSubscriptionBounds;
  readLatest(): readonly ObservationReadResult[];
  pump(): Promise<ObservationSubscriptionNotification | undefined>;
  update(request: ObservationSubscriptionRequest): Promise<ObservationSubscriptionReceipt>;
  close(): Promise<void>;
}

type MaybePromise<T> = T | Promise<T>;

export interface ObservationSubscriptionScheduler {
  setInterval(callback: () => void, milliseconds: number): unknown;
  clearInterval(handle: unknown): void;
}

/** Small transport seam shared by the browser and direct Wasm entries. */
export interface ObservationSubscriptionTransport {
  observationMap(): MaybePromise<ObservationMap>;
  readObservations(selections: readonly ObservationSelection[]): MaybePromise<readonly ObservationReadResult[]>;
  console(): MaybePromise<EngineConsole>;
  readonly scheduler?: ObservationSubscriptionScheduler;
}

interface EffectiveLimits {
  readonly maximumHandles: number;
  readonly maximumBindings: number;
  readonly maximumSelections: number;
  readonly maximumWindowBlocks: number;
  readonly maximumCadenceMs: number;
}

interface ResolvedSelection {
  readonly selection: ObservationSelection;
  readonly binding: ObservationBinding;
  readonly baseKey: string;
  readonly key: string;
  readonly nativeSelection: ObservationSelection;
}

interface BindingState {
  readonly key: string;
  readonly baseKey: string;
  readonly selection: ObservationSelection;
  readonly nativeSelection: ObservationSelection;
  readonly binding: ObservationBinding;
  readonly windowBlocks: number;
  refs: number;
  latest: ObservationReadResult | undefined;
  lastSequence: bigint | undefined;
  nativeMissed: bigint;
  appliedAtSample: bigint;
}

interface HandleState {
  readonly id: bigint;
  readonly owner: bigint;
  epoch: bigint;
  configuration: ObservationSubscriptionConfiguration;
  entries: readonly ResolvedSelection[];
  callback: ((notification: ObservationSubscriptionNotification) => void) | undefined;
  appliedAtSample: bigint;
  readonly cursor: Map<string, bigint>;
  readonly nativeMissedSeen: Map<string, bigint>;
  nextDeliveryAt: number;
  closed: boolean;
  closing: Promise<void> | undefined;
  publicHandle: ObservationSubscriptionImpl | undefined;
}

let nextOwner = 1n;

function keyFor(selection: ObservationSelection): string {
  return `${selection.trackId}\u0000${selection.rack}\u0000${selection.effectSlotId}\u0000${selection.tapId}`;
}

function bindingKey(selection: ObservationSelection, windowBlocks: number): string {
  return `${keyFor(selection)}\u0000${windowBlocks}`;
}

function cloneSelection(selection: ObservationSelection, channels = selection.channels): ObservationSelection {
  return Object.freeze({
    trackId: selection.trackId,
    rack: selection.rack,
    effectSlotId: selection.effectSlotId,
    tapId: selection.tapId,
    channels,
  });
}

function canonicalLimits(overrides: ObservationSubscriptionLimits | undefined): EffectiveLimits {
  const value = (name: keyof EffectiveLimits, fallback: number): number => {
    const requested = overrides?.[name];
    if (requested === undefined) return fallback;
    if (!Number.isSafeInteger(requested) || requested <= 0) {
      throw new MisoUsageError(`${name} must be a positive safe integer`);
    }
    return requested;
  };
  const subscriptionLimits = {
    maximumHandles: value("maximumHandles", DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS.maximumHandles),
    maximumBindings: value("maximumBindings", DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS.maximumBindings),
    maximumSelections: value("maximumSelections", DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS.maximumSelections),
    maximumWindowBlocks: value("maximumWindowBlocks", DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS.maximumWindowBlocks),
    maximumCadenceMs: value("maximumCadenceMs", DEFAULT_OBSERVATION_SUBSCRIPTION_LIMITS.maximumCadenceMs),
  };
  if (subscriptionLimits.maximumSelections > MAXIMUM_OBSERVATION_READS) {
    throw new MisoUsageError(`maximumSelections cannot exceed ${MAXIMUM_OBSERVATION_READS}`);
  }
  if (subscriptionLimits.maximumBindings > MAXIMUM_OBSERVATION_READS) {
    throw new MisoUsageError(`maximumBindings cannot exceed ${MAXIMUM_OBSERVATION_READS}`);
  }
  return Object.freeze(subscriptionLimits);
}

function normalizedRequest(
  request: ObservationSubscriptionRequest,
  subscriptionLimits: EffectiveLimits,
): { readonly configuration: ObservationSubscriptionConfiguration; readonly callback: ((notification: ObservationSubscriptionNotification) => void) | undefined } {
  if (request === null || typeof request !== "object") {
    throw new MisoUsageError("observation subscription request must be an object");
  }
  if (!Array.isArray(request.selections)) {
    throw new MisoUsageError("observation subscription selections must be an array");
  }
  if (request.selections.length > subscriptionLimits.maximumSelections) {
    throw new MisoUsageError(`observation subscription selections are capped at ${subscriptionLimits.maximumSelections}`);
  }
  // `readObservations` also validates, but admission must happen before the console transaction.
  // Calling it here avoids reserving a binding for a malformed target.
  validateObservationSelections(request.selections);
  const selections = request.selections.map((selection) => cloneSelection(selection));
  if (!Number.isSafeInteger(request.windowBlocks)
      || request.windowBlocks <= 0 || request.windowBlocks > subscriptionLimits.maximumWindowBlocks) {
    throw new MisoUsageError(
      `observation windowBlocks must be an integer in 1..=${subscriptionLimits.maximumWindowBlocks}`,
    );
  }
  const cadenceMs = request.cadenceMs ?? 100;
  if (!Number.isSafeInteger(cadenceMs) || cadenceMs <= 0 || cadenceMs > subscriptionLimits.maximumCadenceMs) {
    throw new MisoUsageError(
      `observation cadenceMs must be an integer in 1..=${subscriptionLimits.maximumCadenceMs}`,
    );
  }
  if (request.onUpdate !== undefined && typeof request.onUpdate !== "function") {
    throw new MisoUsageError("observation onUpdate must be a function");
  }
  return {
    configuration: Object.freeze({
      selections: Object.freeze(selections),
      windowBlocks: request.windowBlocks,
      cadenceMs,
    }),
    callback: request.onUpdate,
  };
}

function mapSelection(map: ObservationMap, selection: ObservationSelection, windowBlocks: number): ResolvedSelection {
  const binding = map.bindings.find((candidate) => candidate.trackId === selection.trackId
    && candidate.rack === selection.rack && candidate.effectSlotId === selection.effectSlotId);
  if (binding === undefined || !binding.tapIds.includes(selection.tapId)) {
    throw new MisoEngineError("the selected observation is unavailable in the current owner", {
      phase: "output",
      code: "unsupported",
      result: 7,
      diagnostics: [{ code: "sdk.observation.selection", path: selection.effectSlotId }],
    });
  }
  return Object.freeze({
    selection,
    binding,
    baseKey: keyFor(selection),
    key: bindingKey(selection, windowBlocks),
    nativeSelection: cloneSelection(selection, "both"),
  });
}

function tapName(binding: ObservationBinding, tapId: number): string {
  const effect = CATALOG.effects.find((candidate) => candidate.id === binding.nativeEffectId);
  const tap = effect?.observations.find((candidate) => candidate.id === tapId);
  if (tap === undefined || tap.subscribable !== true) {
    throw new MisoEngineError("the selected observation descriptor is not subscribable", {
      phase: "asset",
      code: "abiMismatch",
      result: 2,
      diagnostics: [{ code: "sdk.observation.descriptor", path: `${binding.nativeEffectId}/${tapId}` }],
    });
  }
  return tap.name;
}

function pendingRow(row: ObservationReadResult): ObservationReadResult {
  if (row.status !== "unarmed") return row;
  return Object.freeze({
    trackId: row.trackId,
    rack: row.rack,
    effectSlotId: row.effectSlotId,
    tapId: row.tapId,
    nativeEffectId: row.nativeEffectId,
    descriptor: row.descriptor,
    channels: row.channels,
    sampleRateHz: row.sampleRateHz,
    status: "pending" as const,
  });
}

function projectRow(row: ObservationReadResult, channels: ObservationChannels): ObservationReadResult {
  if (row.channels === channels) return row;
  const { left, right, ...withoutValues } = row;
  if (channels === "left") {
    return Object.freeze({
      ...withoutValues,
      channels,
      ...(left === undefined ? {} : { left }),
    });
  }
  if (channels === "right") {
    return Object.freeze({
      ...withoutValues,
      channels,
      ...(right === undefined ? {} : { right }),
    });
  }
  return Object.freeze({
    ...withoutValues,
    channels,
    ...(left === undefined ? {} : { left }),
    ...(right === undefined ? {} : { right }),
  });
}

function commandFailure(message: string, report: CommandReport): MisoEngineError {
  return new MisoEngineError(message, {
    phase: report.code === "wrongState" ? "lifecycle" : "output",
    code: report.code,
    result: report.result,
  });
}

class ObservationSubscriptionImpl implements ObservationSubscription {
  readonly #owner: ObservationSubscriptionOwner;
  readonly #state: HandleState;

  constructor(
    owner: ObservationSubscriptionOwner,
    state: HandleState,
  ) {
    this.#owner = owner;
    this.#state = state;
  }

  get id(): bigint { return this.#state.id; }
  get handle(): ObservationSubscription { return this; }
  get owner(): bigint { return this.#state.owner; }
  get epoch(): bigint { return this.#state.epoch; }
  get configuration(): ObservationSubscriptionConfiguration { return this.#state.configuration; }
  get appliedAtSample(): bigint { return this.#state.appliedAtSample; }
  get bounds(): ObservationSubscriptionBounds { return this.#owner.bounds; }

  readLatest(): readonly ObservationReadResult[] {
    return this.#owner.readLatest(this.#state);
  }

  pump(): Promise<ObservationSubscriptionNotification | undefined> {
    return this.#owner.pump(this.#state);
  }

  update(request: ObservationSubscriptionRequest): Promise<ObservationSubscriptionReceipt> {
    return this.#owner.update(this.#state, request);
  }

  close(): Promise<void> {
    return this.#owner.close(this.#state);
  }
}

/** One engine-local owner shared by all resident-observation handles. */
export class ObservationSubscriptionOwner {
  readonly #transport: ObservationSubscriptionTransport;
  readonly #subscriptionLimits: EffectiveLimits;
  readonly #owner = nextOwner++;
  #epoch = 1n;
  #nextHandle = 1n;
  #handles = new Map<bigint, HandleState>();
  #bindings = new Map<string, BindingState>();
  #mutation: Promise<void> = Promise.resolve();
  #polling: Promise<void> | undefined;
  #mutationBusy = false;
  #timer: unknown;
  #timerCadence: number | undefined;
  #disposed = false;

  constructor(
    transport: ObservationSubscriptionTransport,
    subscriptionLimits?: ObservationSubscriptionLimits,
  ) {
    this.#transport = transport;
    this.#subscriptionLimits = canonicalLimits(subscriptionLimits);
  }

  get bounds(): ObservationSubscriptionBounds {
    return Object.freeze({
      ...this.#subscriptionLimits,
      maximumReadSelections: MAXIMUM_OBSERVATION_READS,
      maximumCommandRecords: ABI_LAYOUT.constants.maximumCommandRecords,
    });
  }

  subscribe(request: ObservationSubscriptionRequest): Promise<ObservationSubscriptionReceipt> {
    const normalized = normalizedRequest(request, this.#subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertOpen();
      const epoch = this.#epoch;
      if (this.#handles.size >= this.#subscriptionLimits.maximumHandles) {
        throw new MisoUsageError(`observation subscriptions are capped at ${this.#subscriptionLimits.maximumHandles}`);
      }
      await this.#waitForPoll();
      const map = await this.#transport.observationMap();
      this.#assertEpoch(epoch);
      const entries = normalized.configuration.selections.map((selection) =>
        mapSelection(map, selection, normalized.configuration.windowBlocks));
      if (entries.some((entry) => {
        const managed = [...this.#bindings.values()].find((binding) => binding.baseKey === entry.baseKey);
        return managed !== undefined && managed.key !== entry.key;
      })) {
        throw new MisoUsageError("observation window conflicts with a managed subscriber");
      }
      const newEntries = this.#newEntries(entries);
      if (this.#bindings.size + newEntries.length > this.#subscriptionLimits.maximumBindings) {
        throw new MisoUsageError(`managed observation bindings are capped at ${this.#subscriptionLimits.maximumBindings}`);
      }
      const preflight = await this.#preflight(newEntries, epoch);
      this.#assertCommandSize(newEntries.length);
      const console = newEntries.length === 0 ? undefined : await this.#transport.console();
      this.#assertEpoch(epoch);
      const edits = newEntries.map((entry) =>
        this.#edit(console!, entry, true, normalized.configuration.windowBlocks));
      const report = await this.#submit(console, edits, epoch);
      this.#assertEpoch(epoch);
      const appliedAtSample = report?.appliedAtSample ?? this.#existingSample(entries);
      const state = this.#newHandle(normalized.configuration, entries, normalized.callback, appliedAtSample);
      newEntries.forEach((entry) => {
        const row = preflight.get(entry.key);
        this.#bindings.set(entry.key, {
          key: entry.key,
          baseKey: entry.baseKey,
          selection: entry.selection,
          nativeSelection: entry.nativeSelection,
          binding: entry.binding,
          windowBlocks: normalized.configuration.windowBlocks,
          refs: 1,
          latest: row === undefined ? undefined : pendingRow(row),
          lastSequence: undefined,
          nativeMissed: 0n,
          appliedAtSample: report?.appliedAtSample ?? 0n,
        });
      });
      entries.filter((entry) => !newEntries.includes(entry)).forEach((entry) => {
        this.#bindings.get(entry.key)!.refs += 1;
      });
      this.#handles.set(state.id, state);
      this.#startTimer();
      return this.#receipt(state);
    });
  }

  update(state: HandleState, request: ObservationSubscriptionRequest): Promise<ObservationSubscriptionReceipt> {
    const normalized = normalizedRequest(request, this.#subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertHandle(state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      const map = await this.#transport.observationMap();
      this.#assertEpoch(epoch);
      const entries = normalized.configuration.selections.map((selection) =>
        mapSelection(map, selection, normalized.configuration.windowBlocks));
      const currentByKey = new Map(state.entries.map((entry) => [entry.key, entry] as const));
      const desiredByKey = new Map(entries.map((entry) => [entry.key, entry] as const));
      const remove = state.entries.filter((entry) => !desiredByKey.has(entry.key));
      const add = entries.filter((entry) => !currentByKey.has(entry.key));
      const conflicts = entries.filter((entry) => {
        const base = [...this.#bindings.values()].find((binding) => binding.baseKey === entry.baseKey);
        if (base === undefined || base.key === entry.key) return false;
        const ownedByThisHandle = state.entries.some((current) => current.key === base.key);
        return !ownedByThisHandle || base.refs > 1;
      });
      if (conflicts.length > 0) {
        throw new MisoUsageError("observation window conflicts with a managed subscriber");
      }
      const newEntries = add.filter((entry) => !this.#bindings.has(entry.key));
      if (this.#bindings.size + newEntries.length > this.#subscriptionLimits.maximumBindings) {
        throw new MisoUsageError(`managed observation bindings are capped at ${this.#subscriptionLimits.maximumBindings}`);
      }
      const disarm = remove.filter((entry) => {
        const binding = this.#bindings.get(entry.key);
        return binding !== undefined && binding.refs === 1;
      });
      this.#assertCommandSize(newEntries.length + disarm.length);
      const preflightEntries = newEntries.filter((entry) => !state.entries.some((current) => {
        const binding = this.#bindings.get(current.key);
        return current.baseKey === entry.baseKey && binding?.refs === 1;
      }));
      const preflight = await this.#preflight(preflightEntries, epoch);
      const console = disarm.length + newEntries.length === 0
        ? undefined
        : await this.#transport.console();
      this.#assertEpoch(epoch);
      const edits: LaneEdit[] = [
        ...disarm.map((entry) => this.#edit(
          console!, entry, false, this.#bindings.get(entry.key)!.windowBlocks,
        )),
        ...newEntries.map((entry) => this.#edit(
          console!, entry, true, normalized.configuration.windowBlocks,
        )),
      ];
      const report = await this.#submit(console, edits, epoch);
      this.#assertEpoch(epoch);
      const appliedAtSample = report?.appliedAtSample ?? this.#existingSample(entries);
      disarm.forEach((entry) => this.#bindings.delete(entry.key));
      remove.filter((entry) => !disarm.includes(entry)).forEach((entry) => {
        const binding = this.#bindings.get(entry.key);
        if (binding !== undefined) binding.refs -= 1;
      });
      newEntries.forEach((entry) => {
        const row = preflight.get(entry.key);
        this.#bindings.set(entry.key, {
          key: entry.key,
          baseKey: entry.baseKey,
          selection: entry.selection,
          nativeSelection: entry.nativeSelection,
          binding: entry.binding,
          windowBlocks: normalized.configuration.windowBlocks,
          refs: 1,
          latest: row === undefined ? undefined : pendingRow(row),
          lastSequence: undefined,
          nativeMissed: 0n,
          appliedAtSample: report?.appliedAtSample ?? 0n,
        });
      });
      add.filter((entry) => !newEntries.includes(entry)).forEach((entry) => {
        this.#bindings.get(entry.key)!.refs += 1;
      });
      state.configuration = normalized.configuration;
      state.entries = entries;
      state.callback = normalized.callback;
      state.appliedAtSample = appliedAtSample;
      state.cursor.clear();
      state.nativeMissedSeen.clear();
      state.nextDeliveryAt = 0;
      this.#restartTimer();
      return this.#receipt(state);
    });
  }

  pump(state: HandleState): Promise<ObservationSubscriptionNotification | undefined> {
    return this.#enqueue(async () => {
      this.#assertHandle(state);
      const epoch = this.#epoch;
      await this.#poll();
      this.#assertEpoch(epoch);
      return this.#notify([state]);
    });
  }

  close(state: HandleState): Promise<void> {
    if (state.closing !== undefined) return state.closing;
    const pending = this.#enqueue(async () => {
      if (state.closed) return;
      this.#assertHandle(state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      const disarm = state.entries.filter((entry) => {
        const binding = this.#bindings.get(entry.key);
        return binding !== undefined && binding.refs === 1;
      });
      this.#assertCommandSize(disarm.length);
      const console = disarm.length === 0 ? undefined : await this.#transport.console();
      this.#assertEpoch(epoch);
      const edits = disarm.map((entry) => this.#edit(
        console!, entry, false, this.#bindings.get(entry.key)!.windowBlocks,
      ));
      const report = await this.#submit(console, edits, epoch);
      this.#assertEpoch(epoch);
      if (report !== undefined && !report.ok) throw commandFailure("observation close was refused", report);
      disarm.forEach((entry) => this.#bindings.delete(entry.key));
      state.entries.filter((entry) => !disarm.includes(entry)).forEach((entry) => {
        const binding = this.#bindings.get(entry.key);
        if (binding !== undefined) binding.refs -= 1;
      });
      this.#handles.delete(state.id);
      state.closed = true;
      this.#stopTimerIfIdle();
    });
    const retryable = pending.catch((error: unknown) => {
      if (state.closing === retryable) state.closing = undefined;
      throw error;
    });
    state.closing = retryable;
    return retryable;
  }

  readLatest(state: HandleState): readonly ObservationReadResult[] {
    this.#assertHandle(state);
    const rows: ObservationReadResult[] = [];
    for (const entry of state.entries) {
      const row = this.#bindings.get(entry.key)?.latest;
      if (row !== undefined) rows.push(projectRow(row, entry.selection.channels));
    }
    return Object.freeze(rows);
  }

  /** Invalidate all old handles at a headless replacement or browser disposal. */
  invalidate(disposed = false): void {
    this.#epoch += 1n;
    for (const state of this.#handles.values()) state.closed = true;
    this.#handles.clear();
    this.#bindings.clear();
    this.#stopTimer();
    if (disposed) this.#disposed = true;
  }

  /** Guard the public console's observation edits while this owner has managed bindings. */
  beforeConsoleSubmit(edits: readonly LaneEdit[], managed = false): void {
    if (managed) return;
    if (!this.#mutationBusy && this.#bindings.size === 0) return;
    if (edits.some((edit) => edit.kind === "observeSubscribe" || edit.kind === "observeUnsubscribe")) {
      throw new MisoUsageError("manual observation edits conflict with managed subscriptions");
    }
  }

  #newHandle(
    configuration: ObservationSubscriptionConfiguration,
    entries: readonly ResolvedSelection[],
    callback: ((notification: ObservationSubscriptionNotification) => void) | undefined,
    appliedAtSample: bigint,
  ): HandleState {
    const state: HandleState = {
      id: this.#nextHandle++,
      owner: this.#owner,
      epoch: this.#epoch,
      configuration,
      entries: Object.freeze([...entries]),
      callback,
      appliedAtSample,
      cursor: new Map(),
      nativeMissedSeen: new Map(),
      nextDeliveryAt: 0,
      closed: false,
      closing: undefined,
      publicHandle: undefined,
    };
    state.publicHandle = new ObservationSubscriptionImpl(this, state);
    return state;
  }

  #receipt(state: HandleState): ObservationSubscriptionReceipt {
    return Object.freeze({
      handle: state.publicHandle!,
      owner: state.owner,
      epoch: state.epoch,
      configuration: state.configuration,
      appliedAtSample: state.appliedAtSample,
      bounds: this.bounds,
    });
  }

  #newEntries(entries: readonly ResolvedSelection[]): readonly ResolvedSelection[] {
    return entries.filter((entry) => !this.#bindings.has(entry.key));
  }

  #existingSample(entries: readonly ResolvedSelection[]): bigint {
    return entries.reduce((sample, entry) => {
      const value = this.#bindings.get(entry.key)?.appliedAtSample ?? 0n;
      return value > sample ? value : sample;
    }, 0n);
  }

  async #preflight(
    newEntries: readonly ResolvedSelection[],
    epoch: bigint,
  ): Promise<ReadonlyMap<string, ObservationReadResult>> {
    if (newEntries.length === 0) return new Map();
    const rows = await this.#transport.readObservations(newEntries.map((entry) => entry.nativeSelection));
    this.#assertEpoch(epoch);
    if (rows.length !== newEntries.length) {
      throw new MisoEngineError("the observation preflight returned the wrong row count", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    const result = new Map<string, ObservationReadResult>();
    rows.forEach((row, index) => {
      if (row.status !== "unarmed") {
        throw new MisoUsageError("the observation is already manually armed and cannot be acquired");
      }
      result.set(newEntries[index]!.key, row);
    });
    return result;
  }

  #edit(
    console: EngineConsole,
    entry: ResolvedSelection,
    armed: boolean,
    windowBlocks: number,
  ): LaneEdit {
    // `effect` and tap names come from the generated catalog after map validation. The console's
    // generic type cannot express a runtime catalog row, so this is the one narrow cast at the seam.
    return console.edit.track(entry.selection.trackId)
      .effect(entry.selection.rack, entry.binding.effectIndex, entry.binding.nativeEffectId as never)
      .observe(tapName(entry.binding, entry.selection.tapId) as never, armed, windowBlocks);
  }

  async #submit(
    console: EngineConsole | undefined,
    edits: readonly LaneEdit[],
    epoch: bigint,
  ): Promise<CommandReport | undefined> {
    if (edits.length === 0) return undefined;
    this.#assertEpoch(epoch);
    let report: CommandReport;
    const managedSubmit = (console as EngineConsole & {
      readonly submitManaged?: (...edits: readonly LaneEdit[]) => Promise<CommandReport>;
    } | undefined)?.submitManaged;
    report = managedSubmit === undefined
      ? await console!.submit(...edits)
      : await managedSubmit.call(console, ...edits);
    if (!report.ok) throw commandFailure("observation transaction was refused", report);
    return report;
  }

  #assertCommandSize(count: number): void {
    if (count > ABI_LAYOUT.constants.maximumCommandRecords) {
      throw new MisoUsageError(
        `observation transaction needs ${count} records, maximum is ${ABI_LAYOUT.constants.maximumCommandRecords}`,
      );
    }
  }

  async #refresh(epoch = this.#epoch): Promise<void> {
    if (this.#bindings.size === 0) return;
    const bindings = [...this.#bindings.values()];
    if (bindings.length > MAXIMUM_OBSERVATION_READS) {
      throw new MisoUsageError(`managed observation reads are capped at ${MAXIMUM_OBSERVATION_READS}`);
    }
    const rows = await this.#transport.readObservations(bindings.map((binding) => binding.nativeSelection));
    this.#assertEpoch(epoch);
    if (rows.length !== bindings.length) {
      throw new MisoEngineError("the observation read returned the wrong row count", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    rows.forEach((row, index) => {
      const binding = bindings[index]!;
      const previous = binding.lastSequence;
      if (row.status === "ready" && row.window !== undefined) {
        if (previous !== undefined && row.window.sequence > previous + 1n) {
          binding.nativeMissed += row.window.sequence - previous - 1n;
        }
        if (previous === undefined || row.window.sequence > previous) binding.lastSequence = row.window.sequence;
      }
      binding.latest = row;
    });
  }

  async #poll(): Promise<void> {
    if (this.#polling !== undefined) return this.#polling;
    const epoch = this.#epoch;
    const work = this.#refresh(epoch).then(() => { this.#assertEpoch(epoch); });
    const settled = work.finally(() => {
      if (this.#polling === settled) this.#polling = undefined;
    });
    this.#polling = settled;
    return settled;
  }

  #notify(
    targets: readonly HandleState[],
    respectCadence = false,
  ): ObservationSubscriptionNotification | undefined {
    let requested: ObservationSubscriptionNotification | undefined;
    const now = Date.now();
    for (const state of targets) {
      if (state.closed || state.publicHandle === undefined) continue;
      if (respectCadence && now < state.nextDeliveryAt) continue;
      let available = false;
      let nativeMissed = 0n;
      let skipped = 0n;
      for (const entry of state.entries) {
        const binding = this.#bindings.get(entry.key);
        const row = binding?.latest;
        if (binding === undefined || row?.status !== "ready" || row.window === undefined) continue;
        const sequence = row.window.sequence;
        const cursor = state.cursor.get(entry.key);
        const seenNative = state.nativeMissedSeen.get(entry.key) ?? 0n;
        if (cursor === undefined) {
          state.cursor.set(entry.key, sequence);
          state.nativeMissedSeen.set(entry.key, binding.nativeMissed);
          available = true;
          continue;
        }
        if (sequence > cursor) {
          const gap = sequence - cursor - 1n;
          const nativeDelta = binding.nativeMissed - seenNative;
          nativeMissed += nativeDelta;
          skipped += gap > nativeDelta ? gap - nativeDelta : 0n;
          state.cursor.set(entry.key, sequence);
          available = true;
        }
      }
      if (!available) continue;
      if (respectCadence) state.nextDeliveryAt = now + state.configuration.cadenceMs;
      const notification = Object.freeze({
        handle: state.publicHandle,
        owner: this.#owner,
        epoch: this.#epoch,
        available: true,
        nativeMissedWindows: nativeMissed,
        skippedPublications: skipped,
      });
      try { state.callback?.(notification); } catch { /* callbacks cannot break the owner */ }
      requested ??= notification;
    }
    return requested;
  }

  #startTimer(): void {
    const scheduler = this.#transport.scheduler;
    if (scheduler === undefined || this.#handles.size === 0) return;
    const cadence = Math.min(...[...this.#handles.values()].map((state) => state.configuration.cadenceMs));
    if (this.#timer !== undefined && this.#timerCadence === cadence) return;
    this.#stopTimer();
    this.#timerCadence = cadence;
    this.#timer = scheduler.setInterval(() => {
      void this.#poll()
        .then(() => this.#notify([...this.#handles.values()], true))
        .catch(() => undefined);
    }, cadence);
  }

  #restartTimer(): void { this.#startTimer(); }

  #stopTimerIfIdle(): void {
    if (this.#handles.size === 0) this.#stopTimer();
    else this.#startTimer();
  }

  #stopTimer(): void {
    if (this.#timer !== undefined) this.#transport.scheduler?.clearInterval(this.#timer);
    this.#timer = undefined;
    this.#timerCadence = undefined;
  }

  async #waitForPoll(): Promise<void> {
    if (this.#polling !== undefined) await this.#polling.catch(() => undefined);
  }

  #assertOpen(): void {
    if (this.#disposed) throw new MisoUsageError("the observation subscription owner is disposed");
  }

  #assertHandle(state: HandleState): void {
    this.#assertOpen();
    if (state.closed || state.epoch !== this.#epoch || this.#handles.get(state.id) !== state) {
      throw new MisoUsageError("the observation subscription handle is closed or stale");
    }
  }

  #enqueue<T>(operation: () => Promise<T>): Promise<T> {
    if (this.#mutationBusy) {
      return Promise.reject(new MisoUsageError("an observation subscription operation is already in flight"));
    }
    this.#mutationBusy = true;
    const run = this.#mutation.then(operation, operation);
    this.#mutation = run.then(() => undefined, () => undefined);
    return run.finally(() => { this.#mutationBusy = false; });
  }

  #assertEpoch(epoch: bigint): void {
    if (epoch !== this.#epoch) {
      throw new MisoUsageError("the observation subscription owner changed while the request was pending");
    }
  }
}
