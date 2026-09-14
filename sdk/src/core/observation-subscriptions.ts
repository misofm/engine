import { ABI_LAYOUT } from "../generated/abi.ts";
import { constantValue } from "./abi.ts";
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
import { normalizeTrackResponseQuery } from "./live-response.ts";
import type {
  TrackResponseLimits,
  TrackResponseObservedState,
  TrackResponseQuery,
  TrackResponseRead,
  TrackResponseResult,
} from "./live-response.ts";
import {
  cloneSpectrumQuery,
  cloneSpectrumStreamMetadata,
  spectrumCaptureWindowBytes,
} from "./spectrum.ts";
import type {
  SpectrumCollection,
  SpectrumQuery,
  SpectrumResult,
  SpectrumStreamMetadata,
  SpectrumStreamRead,
  SpectrumStreamSelection,
  SpectrumStreamStart,
  SpectrumStreamStatus,
} from "./spectrum.ts";
import type { EngineCallResult } from "./boundary.ts";

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
  /** Optional live-response capture/evaluation seam owned by the same lifetime. */
  readonly responseRead?: (
    request: TrackResponseQuery,
    previousState?: TrackResponseObservedState,
  ) => MaybePromise<TrackResponseRead>;
  /** Optional managed spectrum stream owned by this same engine lifetime. */
  readonly spectrumPrepared?: () => SpectrumQuery | undefined;
  /** Optional collection of exact targets available to the same managed spectrum owner. */
  readonly spectrumPreparedCollection?: () => SpectrumCollection | undefined;
  /** Atomically select one exact prepared collection entry without stopping its stream. */
  readonly spectrumSelect?: (query: SpectrumQuery) => MaybePromise<EngineCallResult>;
  /** Atomically update an active collection entry, including its smoothing profile. */
  readonly spectrumStreamSelect?: (
    query: SpectrumQuery,
    smoothingMs: number,
  ) => MaybePromise<SpectrumStreamSelection>;
  readonly spectrumStart?: (smoothingMs: number, query: SpectrumQuery) => MaybePromise<SpectrumStreamStart>;
  readonly spectrumRead?: (query: SpectrumQuery) => MaybePromise<SpectrumStreamRead>;
  readonly spectrumStop?: () => MaybePromise<EngineCallResult | void>;
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

class SpectrumSubscriptionImpl implements SpectrumSubscription {
  readonly #owner: ObservationSubscriptionOwner;
  readonly #state: SpectrumHandleState;

  constructor(owner: ObservationSubscriptionOwner, state: SpectrumHandleState) {
    this.#owner = owner;
    this.#state = state;
  }

  get id(): bigint { return this.#state.id; }
  get handle(): SpectrumSubscription { return this; }
  get owner(): bigint { return this.#state.owner; }
  get epoch(): bigint { return this.#state.epoch; }
  get job(): bigint { return this.#state.job.id; }
  get revision(): bigint { return this.#state.job.revision; }
  get configuration(): SpectrumSubscriptionConfiguration { return this.#state.configuration; }
  get bounds(): SpectrumSubscriptionBounds { return this.#owner.spectrumBounds; }

  readLatest(): SpectrumResult | undefined {
    return this.#owner.readLatestSpectrum(this.#state);
  }

  pump(): Promise<SpectrumSubscriptionNotification | undefined> {
    return this.#owner.pumpSpectrum(this.#state);
  }

  update(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscriptionReceipt> {
    return this.#owner.updateSpectrum(this.#state, request);
  }

  close(): Promise<void> {
    return this.#owner.closeSpectrum(this.#state);
  }
}

/** One engine-local owner shared by all resident-observation handles. */
export class ObservationSubscriptionOwner {
  readonly #transport: ObservationSubscriptionTransport;
  readonly #subscriptionLimits: EffectiveLimits;
  readonly #responseSubscriptionLimits: ResponseSubscriptionEffectiveLimits | undefined;
  readonly #spectrumSubscriptionLimits: SpectrumSubscriptionEffectiveLimits | undefined;
  readonly #owner = nextOwner++;
  #epoch = 1n;
  #nextHandle = 1n;
  #nextResponseHandle = 1n;
  #nextResponseJob = 1n;
  #nextSpectrumHandle = 1n;
  #nextSpectrumJob = 1n;
  #handles = new Map<bigint, HandleState>();
  #bindings = new Map<string, BindingState>();
  #responseHandles = new Map<bigint, ResponseHandleState>();
  #responseJobs = new Map<string, ResponseJobState>();
  #spectrumHandles = new Map<bigint, SpectrumHandleState>();
  #spectrumJob: SpectrumJobState | undefined;
  #mutation: Promise<void> = Promise.resolve();
  #polling: Promise<void> | undefined;
  #mutationBusy = false;
  #timer: unknown;
  #timerCadence: number | undefined;
  #disposed = false;

  constructor(
    transport: ObservationSubscriptionTransport,
    subscriptionLimits?: ObservationSubscriptionLimits,
    responseSubscriptionLimits?: TrackResponseSubscriptionLimits,
    spectrumSubscriptionLimits?: SpectrumSubscriptionLimits,
  ) {
    this.#transport = transport;
    this.#subscriptionLimits = canonicalLimits(subscriptionLimits);
    this.#responseSubscriptionLimits = transport.responseRead === undefined
      ? undefined
      : canonicalResponseSubscriptionLimits(responseSubscriptionLimits);
    this.#spectrumSubscriptionLimits = transport.spectrumStart === undefined
      || transport.spectrumRead === undefined || transport.spectrumStop === undefined
      ? undefined
      : canonicalSpectrumSubscriptionLimits(spectrumSubscriptionLimits);
  }

  get bounds(): ObservationSubscriptionBounds {
    return Object.freeze({
      ...this.#subscriptionLimits,
      maximumReadSelections: MAXIMUM_OBSERVATION_READS,
      maximumCommandRecords: ABI_LAYOUT.constants.maximumCommandRecords,
    });
  }

  get responseBounds(): TrackResponseSubscriptionBounds {
    const subscriptionLimits = this.#responseSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      throw new MisoUsageError("live response subscriptions are not configured on this owner");
    }
    return Object.freeze({
      ...subscriptionLimits,
      maximumPoints: ABI_LAYOUT.constants.maximumLiveResponsePoints,
      maximumCaptureBytes: ABI_LAYOUT.constants.liveResponseCaptureBytes,
    });
  }

  get spectrumBounds(): SpectrumSubscriptionBounds {
    const subscriptionLimits = this.#spectrumSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      throw new MisoUsageError("managed spectrum subscriptions are not configured on this owner");
    }
    const hopFrames = this.#spectrumJob?.metadata.hopFrames ?? 0;
    return Object.freeze({
      ...subscriptionLimits,
      maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes,
      maximumResultBytes: ABI_LAYOUT.constants.spectrumCaptureBytes,
      hopFrames,
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
    for (const state of this.#responseHandles.values()) state.closed = true;
    for (const state of this.#spectrumHandles.values()) state.closed = true;
    if (this.#spectrumJob !== undefined && this.#spectrumJob.refs > 0) {
      const stop = this.#transport.spectrumStop?.();
      if (stop !== undefined) void Promise.resolve(stop).catch(() => undefined);
    }
    this.#handles.clear();
    this.#bindings.clear();
    this.#responseHandles.clear();
    this.#responseJobs.clear();
    this.#spectrumHandles.clear();
    this.#spectrumJob = undefined;
    this.#stopTimer();
    if (disposed) this.#disposed = true;
  }

  /** Invalidate only the spectrum lifetime after its analysis transport has failed. */
  invalidateSpectrum(): void {
    for (const state of this.#spectrumHandles.values()) state.closed = true;
    this.#spectrumHandles.clear();
    this.#spectrumJob = undefined;
    this.#stopTimerIfIdle();
  }

  /** @internal Whether a managed spectrum handle owns the prepared stream. */
  managedSpectrumActive(): boolean {
    return this.#spectrumHandles.size !== 0;
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
    const work = this.#refresh(epoch)
      .then(() => this.#refreshResponseJobs(epoch))
      .then(() => this.#refreshSpectrumJobs(epoch))
      .then(() => { this.#assertEpoch(epoch); });
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
          state.nativeMissedSeen.set(entry.key, binding.nativeMissed);
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
    if (scheduler === undefined || (this.#handles.size === 0 && this.#responseHandles.size === 0
      && this.#spectrumHandles.size === 0)) return;
    const cadences = [
      ...[...this.#handles.values()].map((state) => state.configuration.cadenceMs),
      ...[...this.#responseHandles.values()].map((state) => state.configuration.cadenceMs),
      ...(this.#spectrumJob === undefined ? [] : [this.#spectrumJob.captureCadenceMs]),
    ];
    // Native spectrum hops can be fractional milliseconds. Round the shared timer up so a
    // cadence tick never arrives before the native deadline and shifts every read by one tick.
    const cadence = Math.max(1, Math.ceil(Math.min(...cadences)));
    if (this.#timer !== undefined && this.#timerCadence === cadence) return;
    this.#stopTimer();
    this.#timerCadence = cadence;
    this.#timer = scheduler.setInterval(() => {
      if (this.#mutationBusy || this.#polling !== undefined) return;
      void this.#poll()
        .then(() => {
          this.#notify([...this.#handles.values()], true);
          for (const state of this.#responseHandles.values()) this.#notifyResponse(state, true);
          for (const state of this.#spectrumHandles.values()) this.#notifySpectrum(state, true);
        })
        .catch(() => undefined);
    }, cadence);
  }

  #restartTimer(): void { this.#startTimer(); }

  #stopTimerIfIdle(): void {
    if (this.#handles.size === 0 && this.#responseHandles.size === 0 && this.#spectrumHandles.size === 0) this.#stopTimer();
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

  /** Subscribe to one selected track response while sharing this owner's lifetime and poll. */
  subscribeTrackResponse(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscriptionReceipt> {
    const subscriptionLimits = this.#responseSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      return Promise.reject(new MisoUsageError("live response subscriptions are not configured on this owner"));
    }
    const normalized = normalizedTrackResponseSubscription(request, subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertOpen();
      const epoch = this.#epoch;
      if (this.#responseHandles.size >= subscriptionLimits.maximumHandles) {
        throw new MisoUsageError(`track response subscriptions are capped at ${subscriptionLimits.maximumHandles}`);
      }
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      const job = await this.#ensureResponseJob(normalized.configuration, epoch);
      this.#assertEpoch(epoch);
      try {
        this.#assertResponseDeliveryBudget(job, normalized.configuration.cadenceMs);
      } catch (error) {
        this.#dropUnreferencedResponseJob(job);
        throw error;
      }
      job.refs += 1;
      const state = this.#newResponseHandle(normalized.configuration, normalized.callback, job);
      this.#responseHandles.set(state.id, state);
      this.#refreshResponseJobCadence(job);
      this.#startTimer();
      return this.#responseReceipt(state);
    });
  }

  updateTrackResponse(
    state: ResponseHandleState,
    request: TrackResponseSubscriptionRequest,
  ): Promise<TrackResponseSubscriptionReceipt> {
    const subscriptionLimits = this.#responseSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      return Promise.reject(new MisoUsageError("live response subscriptions are not configured on this owner"));
    }
    const normalized = normalizedTrackResponseSubscription(request, subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertResponseHandle(state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      const current = state.job;
      const job = await this.#ensureResponseJob(normalized.configuration, epoch);
      this.#assertEpoch(epoch);
      try {
        this.#assertResponseDeliveryBudget(job, normalized.configuration.cadenceMs, state);
      } catch (error) {
        this.#dropUnreferencedResponseJob(job);
        throw error;
      }
      if (job !== current) {
        current.refs -= 1;
        if (current.refs === 0) this.#responseJobs.delete(current.key);
        job.refs += 1;
        state.job = job;
      }
      state.configuration = normalized.configuration;
      state.callback = normalized.callback;
      state.cursor = job.publicationSequence;
      state.nextDeliveryAt = 0;
      this.#refreshResponseJobCadence(current);
      if (job !== current) this.#refreshResponseJobCadence(job);
      this.#restartTimer();
      return this.#responseReceipt(state);
    });
  }

  pumpTrackResponse(state: ResponseHandleState): Promise<TrackResponseSubscriptionNotification | undefined> {
    return this.#enqueue(async () => {
      this.#assertResponseHandle(state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      if (this.#responseJobs.get(state.job.key) === state.job) state.job.nextCaptureAt = 0;
      await this.#poll();
      this.#assertEpoch(epoch);
      return this.#notifyResponse(state, false);
    });
  }

  closeTrackResponse(state: ResponseHandleState): Promise<void> {
    if (state.closing !== undefined) return state.closing;
    const pending = this.#enqueue(async () => {
      if (state.closed) return;
      this.#assertResponseHandle(state);
      const job = state.job;
      state.closed = true;
      this.#responseHandles.delete(state.id);
      job.refs -= 1;
      if (job.refs === 0) this.#responseJobs.delete(job.key);
      else this.#refreshResponseJobCadence(job);
      this.#stopTimerIfIdle();
    });
    const retryable = pending.catch((error: unknown) => {
      if (state.closing === retryable) state.closing = undefined;
      throw error;
    });
    state.closing = retryable;
    return retryable;
  }

  /** Subscribe to the one prepared managed spectrum stream. */
  subscribeSpectrum(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscriptionReceipt> {
    const subscriptionLimits = this.#spectrumSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      return Promise.reject(new MisoUsageError("managed spectrum subscriptions are not configured on this owner"));
    }
    const normalized = normalizedSpectrumSubscription(request, subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertOpen();
      const epoch = this.#epoch;
      if (this.#spectrumHandles.size >= subscriptionLimits.maximumHandles) {
        throw new MisoUsageError(`spectrum subscriptions are capped at ${subscriptionLimits.maximumHandles}`);
      }
      this.#assertSpectrumPrepared(normalized.configuration);
      this.#assertSpectrumAdmission(normalized.configuration);
      const hadJob = this.#spectrumJob !== undefined;
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      let job: SpectrumJobState;
      try {
        job = await this.#ensureSpectrumJob(normalized.configuration, epoch);
        this.#assertEpoch(epoch);
        this.#assertSpectrumDeliveryBudget(
          job,
          normalized.configuration.cadenceMs,
          undefined,
          normalized.configuration.channels,
        );
      } catch (error) {
        // A first admission may have started a native stream before a returned
        // profile proved unusable. Ensure that refusal cannot leave an orphan.
        if (!hadJob && this.#spectrumJob !== undefined && this.#spectrumJob.refs === 0) {
          const stop = await Promise.resolve(this.#transport.spectrumStop!()).catch(() => undefined);
          if (stop === undefined || stop.ok) this.#spectrumJob = undefined;
        }
        throw error;
      }
      job.refs += 1;
      const state = this.#newSpectrumHandle(normalized.configuration, normalized.callback, job);
      this.#spectrumHandles.set(state.id, state);
      this.#startTimer();
      return this.#spectrumReceipt(state);
    });
  }

  updateSpectrum(
    state: SpectrumHandleState,
    request: SpectrumSubscriptionRequest,
  ): Promise<SpectrumSubscriptionReceipt> {
    const subscriptionLimits = this.#spectrumSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      return Promise.reject(new MisoUsageError("managed spectrum subscriptions are not configured on this owner"));
    }
    const normalized = normalizedSpectrumSubscription(request, subscriptionLimits);
    return this.#enqueue(async () => {
      this.#assertSpectrumHandle(state);
      this.#assertSpectrumPrepared(normalized.configuration);
      this.#assertSpectrumAdmission(normalized.configuration, state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      const current = state.job;
      const desiredQuery = this.#spectrumQuery(normalized.configuration);
      const desiredKey = spectrumJobKey(normalized.configuration, desiredQuery);
      if (desiredKey !== current.key && current.refs > 1) {
        throw new MisoUsageError("spectrum smoothing conflicts with a shared managed subscriber");
      }
      // Check the replacement against the current union before stopping the working stream. This
      // keeps a refused channel/limit/cadence update atomic at the public boundary.
      this.#assertSpectrumDeliveryBudget(
        current,
        normalized.configuration.cadenceMs,
        state,
        normalized.configuration.channels,
      );
      const selectionChanged = spectrumCaptureKey(current.query) !== spectrumCaptureKey(desiredQuery);
      const smoothingChanged = normalized.configuration.smoothingMs !== current.smoothingMs;
      const collection = this.#transport.spectrumPreparedCollection?.();
      if (collection !== undefined && (selectionChanged || smoothingChanged)) {
        // The native collection selection is one transaction. Do not implement a target or
        // smoothing change as a stop + fallible start: the active stream and its shared Worker
        // must remain owned by this handle throughout the update.
        const spectrumStreamSelect = this.#transport.spectrumStreamSelect;
        if (spectrumStreamSelect === undefined) {
          throw new MisoUsageError("the spectrum transport cannot update a prepared collection entry");
        }
        const selected = await spectrumStreamSelect(
          desiredQuery,
          normalized.configuration.smoothingMs,
        );
        this.#assertEpoch(epoch);
        if (!selected.ok) {
          throw new MisoEngineError("the engine refused the prepared spectrum selection", {
            phase: selected.code === "wrongState" ? "lifecycle" : "output",
            code: selected.code as never,
            result: selected.result,
          });
        }
        const selectedMetadata = selected.metadata;
        if (selectedMetadata === undefined) {
          throw new MisoEngineError("the spectrum selection returned no committed metadata", {
            phase: "output", code: "abiMismatch", result: 2,
          });
        }
        if (selectedMetadata.status !== "warming" && selectedMetadata.status !== "pending") {
          throw new MisoEngineError("the spectrum selection returned an invalid warming state", {
            phase: "output", code: "abiMismatch", result: 2,
          });
        }
        if (selectedMetadata.target !== undefined
            && spectrumTargetKey(selectedMetadata.target) !== spectrumTargetKey(normalized.configuration.target)) {
          throw new MisoEngineError("the spectrum selection returned the wrong target", {
            phase: "output", code: "abiMismatch", result: 2,
          });
        }
        if (selectedMetadata.channels !== undefined && selectedMetadata.channels !== normalized.configuration.channels) {
          throw new MisoEngineError("the spectrum selection returned the wrong channels", {
            phase: "output", code: "abiMismatch", result: 2,
          });
        }
        if (selectedMetadata.smoothingMs !== normalized.configuration.smoothingMs) {
          throw new MisoEngineError("the spectrum selection returned the wrong smoothing profile", {
            phase: "output", code: "abiMismatch", result: 2,
          });
        }
        const metadata = cloneSpectrumStreamMetadata(selectedMetadata);
        const replacement: SpectrumJobState = {
          ...current,
          id: this.#nextSpectrumJob++,
          key: desiredKey,
          query: Object.freeze({ ...desiredQuery }),
          smoothingMs: normalized.configuration.smoothingMs,
          metadata,
          result: undefined,
          revision: current.revision + 1n,
          publicationSequence: current.publicationSequence + 1n,
          retainedBytes: 0,
          nextCaptureAt: 0,
          publicationStamp: spectrumPublicationStamp(metadata),
        };
        this.#spectrumJob = replacement;
        current.refs = 0;
        replacement.refs = 1;
        state.job = replacement;
      } else if (desiredKey !== current.key) {
        const stop = await this.#transport.spectrumStop!();
        this.#assertSpectrumStop(stop);
        this.#assertEpoch(epoch);
        this.#spectrumJob = undefined;
        current.refs = 0;
        const replacement = await this.#ensureSpectrumJob(normalized.configuration, epoch);
        this.#assertEpoch(epoch);
        this.#assertSpectrumDeliveryBudget(
          replacement,
          normalized.configuration.cadenceMs,
          state,
          normalized.configuration.channels,
        );
        replacement.refs = 1;
        state.job = replacement;
      } else {
        this.#assertSpectrumDeliveryBudget(current, normalized.configuration.cadenceMs, state);
      }
      state.configuration = normalized.configuration;
      state.callback = normalized.callback;
      state.cursor = state.job.publicationSequence;
      if (state.job !== current) {
        state.nativeMissedEpoch = state.job.metadata.captureEpoch;
        state.nativeMissedSeen = state.job.metadata.droppedCaptures;
        state.pendingNativeMissed = 0n;
      }
      state.nextDeliveryAt = 0;
      this.#restartTimer();
      return this.#spectrumReceipt(state);
    });
  }

  pumpSpectrum(state: SpectrumHandleState): Promise<SpectrumSubscriptionNotification | undefined> {
    return this.#enqueue(async () => {
      this.#assertSpectrumHandle(state);
      const epoch = this.#epoch;
      await this.#waitForPoll();
      this.#assertEpoch(epoch);
      if (this.#spectrumJob === state.job) state.job.nextCaptureAt = 0;
      await this.#poll();
      this.#assertEpoch(epoch);
      return this.#notifySpectrum(state, false);
    });
  }

  closeSpectrum(state: SpectrumHandleState): Promise<void> {
    if (state.closing !== undefined) return state.closing;
    const pending = this.#enqueue(async () => {
      if (state.closed) return;
      this.#assertSpectrumHandle(state);
      await this.#waitForPoll();
      if (state.closed) return;
      this.#assertSpectrumHandle(state);
      const job = state.job;
      if (job.refs === 1) {
        const stop = await this.#transport.spectrumStop!();
        this.#assertSpectrumStop(stop);
        this.#assertEpoch(state.epoch);
      }
      state.closed = true;
      this.#spectrumHandles.delete(state.id);
      job.refs -= 1;
      if (job.refs === 0 && this.#spectrumJob === job) this.#spectrumJob = undefined;
      this.#stopTimerIfIdle();
    });
    const retryable = pending.catch((error: unknown) => {
      if (state.closing === retryable) state.closing = undefined;
      throw error;
    });
    state.closing = retryable;
    return retryable;
  }

  readLatestSpectrum(state: SpectrumHandleState): SpectrumResult | undefined {
    this.#assertSpectrumHandle(state);
    return copySpectrumResult(state.job.result);
  }

  #newSpectrumHandle(
    configuration: SpectrumSubscriptionConfiguration,
    callback: ((notification: SpectrumSubscriptionNotification) => void) | undefined,
    job: SpectrumJobState,
  ): SpectrumHandleState {
    const state: SpectrumHandleState = {
      id: this.#nextSpectrumHandle++, owner: this.#owner, epoch: this.#epoch,
      job, configuration, callback, cursor: job.publicationSequence,
      nativeMissedEpoch: job.metadata.captureEpoch,
      nativeMissedSeen: job.metadata.droppedCaptures,
      pendingNativeMissed: 0n,
      nextDeliveryAt: 0, closed: false, closing: undefined, publicHandle: undefined,
    };
    state.publicHandle = new SpectrumSubscriptionImpl(this, state);
    return state;
  }

  #spectrumReceipt(state: SpectrumHandleState): SpectrumSubscriptionReceipt {
    return Object.freeze({
      handle: state.publicHandle!, owner: state.owner, epoch: state.epoch, job: state.job.id,
      revision: state.job.revision, configuration: state.configuration, bounds: this.spectrumBounds,
    });
  }

  async #ensureSpectrumJob(
    configuration: SpectrumSubscriptionConfiguration,
    epoch: bigint,
  ): Promise<SpectrumJobState> {
    const subscriptionLimits = this.#spectrumSubscriptionLimits!;
    const spectrumStart = this.#transport.spectrumStart;
    const spectrumStop = this.#transport.spectrumStop;
    if (spectrumStart === undefined || this.#transport.spectrumRead === undefined
        || this.#transport.spectrumStop === undefined) {
      throw new MisoUsageError("managed spectrum subscriptions are not configured on this owner");
    }
    this.#assertSpectrumPrepared(configuration);
    const query = this.#spectrumQuery(configuration);
    const key = spectrumJobKey(configuration, query);
    const existing = this.#spectrumJob;
    if (existing !== undefined) {
      if (existing.key !== key) {
        throw new MisoUsageError("the prepared spectrum stream has a conflicting configuration");
      }
      return existing;
    }
    let nativeStarted = false;
    try {
      const preparedCollection = this.#transport.spectrumPreparedCollection?.();
      if (preparedCollection !== undefined) {
        const spectrumSelect = this.#transport.spectrumSelect;
        if (spectrumSelect === undefined) {
          throw new MisoUsageError("the spectrum transport cannot select a prepared collection entry");
        }
        const selected = await spectrumSelect(query);
        if (!selected.ok) {
          throw new MisoEngineError("the engine refused the prepared spectrum selection", {
            phase: selected.code === "wrongState" ? "lifecycle" : "output",
            code: selected.code as never,
            result: selected.result,
          });
        }
      }
      const started = await spectrumStart(configuration.smoothingMs, query);
      nativeStarted = started.ok;
      this.#assertEpoch(epoch);
      if (!started.ok) {
        throw new MisoEngineError("the engine refused the spectrum stream start", {
          phase: started.code === "wrongState" ? "lifecycle" : "output",
          code: started.code as never,
          result: started.result,
        });
      }
      const metadata = started.metadata;
      if (metadata.status !== "warming" && metadata.status !== "pending") {
        throw new MisoEngineError("the spectrum stream returned an invalid start state", {
          phase: "output", code: "abiMismatch", result: 2,
        });
      }
      if (metadata.target !== undefined && spectrumTargetKey(metadata.target) !== spectrumTargetKey(configuration.target)) {
        throw new MisoEngineError("the spectrum stream target differs from the prepared target", {
          phase: "output", code: "abiMismatch", result: 2,
        });
      }
      if (metadata.channels !== undefined && metadata.channels !== configuration.channels) {
        throw new MisoUsageError("the spectrum stream channels were not prepared");
      }
      const captureCadenceMs = metadata.sampleRateHz > 0 && metadata.hopFrames > 0
        ? metadata.hopFrames * 1000 / metadata.sampleRateHz : 0;
      if (!Number.isFinite(captureCadenceMs) || captureCadenceMs <= 0) {
        throw new MisoEngineError("the spectrum stream returned an invalid cadence", {
          phase: "output", code: "abiMismatch", result: 2,
        });
      }
      const job: SpectrumJobState = {
        id: this.#nextSpectrumJob++, key, query: Object.freeze({
          ...query,
        }), smoothingMs: configuration.smoothingMs, refs: 0,
        metadata: cloneSpectrumStreamMetadata(metadata),
        result: undefined, revision: 0n, publicationSequence: 0n,
        retainedBytes: 0, captureCadenceMs, nextCaptureAt: 0,
        publicationStamp: spectrumPublicationStamp(metadata),
      };
      this.#assertSpectrumRetainedBytes(job.retainedBytes);
      this.#spectrumJob = job;
      return job;
    } catch (error) {
      if (nativeStarted) {
        try { await this.#transport.spectrumStop!(); } catch { /* preserve the original admission error */ }
      }
      throw error;
    }
  }

  #assertSpectrumPrepared(configuration: SpectrumSubscriptionConfiguration): void {
    const collection = this.#transport.spectrumPreparedCollection?.();
    if (collection !== undefined) {
      const wantedTarget = spectrumTargetKey(configuration.target);
      if (!collection.entries.some((entry) =>
        spectrumTargetKey(entry.target) === wantedTarget
        && (entry.channels ?? "both") === configuration.channels)) {
        throw new MisoUsageError("the spectrum stream target and channels were not prepared");
      }
      return;
    }
    const prepared = this.#transport.spectrumPrepared?.();
    if (prepared === undefined) return;
    if (spectrumTargetKey(prepared.target) !== spectrumTargetKey(configuration.target)) {
      throw new MisoUsageError("the spectrum stream target was not prepared");
    }
    if ((prepared.channels ?? "both") !== configuration.channels) {
      throw new MisoUsageError("the spectrum stream channels were not prepared");
    }
  }

  #spectrumQuery(configuration: SpectrumSubscriptionConfiguration): SpectrumQuery {
    const prepared = this.#transport.spectrumPrepared?.();
    const maximumCaptureBytes = configuration.spectrumLimits?.maximumCaptureBytes
      ?? prepared?.spectrumLimits?.maximumCaptureBytes
      ?? ABI_LAYOUT.constants.spectrumCaptureBytes;
    const requestDeadlineMs = configuration.spectrumLimits?.requestDeadlineMs ?? 5_000;
    return cloneSpectrumQuery({
      target: configuration.target,
      channels: configuration.channels,
      spectrumLimits: { maximumCaptureBytes, requestDeadlineMs },
    });
  }

  #assertSpectrumAdmission(
    configuration: SpectrumSubscriptionConfiguration,
    replacing?: SpectrumHandleState,
  ): void {
    const subscriptionLimits = this.#spectrumSubscriptionLimits!;
    const query = this.#spectrumQuery(configuration);
    const maximumCaptureBytes = query.spectrumLimits!.maximumCaptureBytes!;
    const preparedMaximumCaptureBytes = this.#transport.spectrumPrepared?.()?.spectrumLimits?.maximumCaptureBytes
      ?? ABI_LAYOUT.constants.spectrumCaptureBytes;
    if (maximumCaptureBytes > preparedMaximumCaptureBytes) {
      throw new MisoUsageError("the spectrum capture limit exceeds the prepared bound");
    }
    if (maximumCaptureBytes < spectrumCaptureWindowBytes(configuration.channels)) {
      throw new MisoUsageError("the spectrum capture limit is below one serialized window");
    }
    const candidateBytes = spectrumVectorBytes(configuration.channels);
    if (candidateBytes > subscriptionLimits.maximumRetainedBytes) {
      throw new MisoUsageError("the spectrum result exceeds the retained-byte bound");
    }
    let delivered = spectrumDeliveryRate(candidateBytes, configuration.cadenceMs);
    for (const handle of this.#spectrumHandles.values()) {
      if (handle === replacing) continue;
      delivered += spectrumDeliveryRate(
        spectrumVectorBytes(handle.configuration.channels),
        handle.configuration.cadenceMs,
      );
    }
    if (!Number.isFinite(delivered) || delivered > subscriptionLimits.maximumDeliveredBytesPerSecond) {
      throw new MisoUsageError("the spectrum vectors exceed the delivery bound");
    }
  }

  #assertSpectrumResultBounds(result: SpectrumResult): void {
    const subscriptionLimits = this.#spectrumSubscriptionLimits!;
    const retainedBytes = spectrumResultRetainedBytes(result);
    if (retainedBytes > subscriptionLimits.maximumRetainedBytes) {
      throw new MisoUsageError("the spectrum result exceeds the retained-byte bound");
    }
  }

  #assertSpectrumDeliveryBudget(
    candidate: SpectrumJobState,
    candidateCadenceMs?: number,
    replacing?: SpectrumHandleState,
    candidateChannels?: SpectrumSubscriptionConfiguration["channels"],
  ): void {
    const maximum = this.#spectrumSubscriptionLimits!.maximumDeliveredBytesPerSecond;
    let delivered = 0;
    for (const handle of this.#spectrumHandles.values()) {
      if (handle === replacing) continue;
      const bytes = handle.job.result === undefined
        ? spectrumVectorBytes(handle.configuration.channels) : spectrumResultVectorBytes(handle.job.result);
      delivered += spectrumDeliveryRate(bytes, handle.configuration.cadenceMs);
    }
    if (candidateCadenceMs !== undefined) {
      const bytes = candidate.result === undefined || candidateChannels !== undefined
        ? spectrumVectorBytes(candidateChannels ?? candidate.query.channels ?? "both")
        : spectrumResultVectorBytes(candidate.result);
      delivered += spectrumDeliveryRate(bytes, candidateCadenceMs);
    }
    if (!Number.isFinite(delivered) || delivered > maximum) {
      throw new MisoUsageError("the spectrum vectors exceed the delivery bound");
    }
  }

  #assertSpectrumRetainedBytes(additional: number): void {
    const maximum = this.#spectrumSubscriptionLimits!.maximumRetainedBytes;
    const existing = this.#spectrumJob?.retainedBytes ?? 0;
    if (!Number.isSafeInteger(existing + additional) || existing + additional > maximum) {
      throw new MisoUsageError("the spectrum result exceeds the retained-byte bound");
    }
  }

  async #refreshSpectrumJobs(epoch: bigint): Promise<void> {
    const job = this.#spectrumJob;
    const spectrumRead = this.#transport.spectrumRead;
    if (job === undefined || job.refs === 0 || spectrumRead === undefined) return;
    if (Date.now() < job.nextCaptureAt) return;
    // Reserve the next capture before awaiting the asynchronous transport. The shared timer may
    // fire again while the read is in flight; anchoring the deadline at dispatch keeps transport
    // completion latency from shifting every subsequent capture past its native cadence.
    job.nextCaptureAt = Date.now() + job.captureCadenceMs;
    const read = await spectrumRead(job.query);
    this.#assertEpoch(epoch);
    this.#publishSpectrumRead(job, read);
    this.#recordSpectrumLoss(job);
    // A native gap reports loss before popping the queued window. Automatic polling must drain
    // once now: waiting another hop can fill the queue again and perpetually return only gaps.
    // Manual pumps already let their caller request the next read without waiting for a timer.
    if (this.#transport.scheduler === undefined || read.metadata.status !== "gap") return;
    for (const state of this.#spectrumHandles.values()) this.#notifySpectrum(state, true);
    const recovery = await spectrumRead(job.query);
    this.#assertEpoch(epoch);
    this.#publishSpectrumRead(job, recovery);
    this.#recordSpectrumLoss(job);
  }

  #publishSpectrumRead(job: SpectrumJobState, read: SpectrumStreamRead): void {
    const metadata = cloneSpectrumStreamMetadata(read.metadata);
    const stamp = spectrumPublicationStamp(metadata);
    if (read.result !== undefined) {
      this.#assertSpectrumResultBounds(read.result);
      const maximumCaptureBytes = job.query.spectrumLimits?.maximumCaptureBytes;
      if (maximumCaptureBytes !== undefined && read.result.resultBytes > BigInt(maximumCaptureBytes)) {
        throw new MisoUsageError("the spectrum result exceeded the subscription capture bound");
      }
      if (metadata.status !== "ready") {
        throw new MisoEngineError("the spectrum stream result has an invalid status", {
          phase: "output", code: "abiMismatch", result: 2,
        });
      }
      if (job.publicationStamp !== stamp || job.result === undefined) {
        const retainedBytes = spectrumResultRetainedBytes(read.result);
        job.result = copySpectrumResult(read.result);
        job.retainedBytes = retainedBytes;
        job.metadata = metadata;
        job.revision += 1n;
        job.publicationSequence += 1n;
        job.publicationStamp = stamp;
      }
      return;
    }
    if (job.publicationStamp !== stamp && (metadata.status === "gap" || metadata.status === "failed")) {
      job.metadata = metadata;
      job.revision += 1n;
      job.publicationSequence += 1n;
      job.publicationStamp = stamp;
    } else {
      job.metadata = metadata;
    }
  }

  #recordSpectrumLoss(job: SpectrumJobState): void {
    // Capture records queued before a gap can carry an older drop count. Accrue loss before
    // cadence coalescing so that a recovery publication cannot erase the intervening gap.
    for (const state of this.#spectrumHandles.values()) {
      if (state.job !== job) continue;
      if (job.metadata.captureEpoch > state.nativeMissedEpoch) {
        state.nativeMissedEpoch = job.metadata.captureEpoch;
        state.nativeMissedSeen = 0n;
        state.pendingNativeMissed = 0n;
      }
      if (job.metadata.captureEpoch === state.nativeMissedEpoch
          && job.metadata.droppedCaptures > state.nativeMissedSeen) {
        state.pendingNativeMissed += job.metadata.droppedCaptures - state.nativeMissedSeen;
        state.nativeMissedSeen = job.metadata.droppedCaptures;
      }
    }
  }

  #notifySpectrum(
    state: SpectrumHandleState,
    respectCadence: boolean,
  ): SpectrumSubscriptionNotification | undefined {
    if (state.closed || state.publicHandle === undefined) return undefined;
    const job = this.#spectrumJob;
    if (job === undefined || state.job !== job) return undefined;
    const now = Date.now();
    if (respectCadence && now < state.nextDeliveryAt) return undefined;
    if (job.publicationSequence <= state.cursor) return undefined;
    const skippedPublications = job.publicationSequence - state.cursor - 1n;
    const nativeMissedWindows = state.pendingNativeMissed;
    state.pendingNativeMissed = 0n;
    state.cursor = job.publicationSequence;
    if (respectCadence) state.nextDeliveryAt = now + state.configuration.cadenceMs;
    const notification = Object.freeze({
      handle: state.publicHandle,
      owner: this.#owner,
      epoch: this.#epoch,
      job: job.id,
      revision: job.revision,
      status: job.metadata.status,
      metadata: job.metadata,
      available: job.result !== undefined && job.metadata.status === "ready",
      nativeMissedWindows,
      skippedPublications,
    });
    try { state.callback?.(notification); } catch { /* callbacks cannot break the owner */ }
    return notification;
  }

  #assertSpectrumStop(stop: EngineCallResult | void): void {
    if (stop === undefined || stop.ok) return;
    throw new MisoEngineError("the spectrum stream stop was refused", {
      phase: stop.code === "wrongState" ? "lifecycle" : "output",
      code: stop.code as never,
      result: stop.result,
    });
  }

  #assertSpectrumHandle(state: SpectrumHandleState): void {
    this.#assertOpen();
    if (state.closed || state.epoch !== this.#epoch || this.#spectrumHandles.get(state.id) !== state) {
      throw new MisoUsageError("the spectrum subscription handle is closed or stale");
    }
  }

  readLatestTrackResponse(state: ResponseHandleState): TrackResponseResult | undefined {
    this.#assertResponseHandle(state);
    return copyTrackResponseResult(state.job.result);
  }

  async #ensureResponseJob(
    configuration: TrackResponseSubscriptionConfiguration,
    epoch: bigint,
  ): Promise<ResponseJobState> {
    const subscriptionLimits = this.#responseSubscriptionLimits!;
    const responseRead = this.#transport.responseRead;
    if (responseRead === undefined) throw new MisoUsageError("live response subscriptions are not configured on this owner");
    const query: TrackResponseQuery = Object.freeze({
      trackId: configuration.trackId,
      grid: configuration.grid,
      channels: configuration.channels,
      responseLimits: configuration.responseLimits,
    });
    const key = responseJobKey(query);
    const existing = this.#responseJobs.get(key);
    if (existing !== undefined) return existing;
    if (this.#responseJobs.size >= subscriptionLimits.maximumJobs) {
      throw new MisoUsageError(`track response jobs are capped at ${subscriptionLimits.maximumJobs}`);
    }
    if (this.#responseJobs.size + 1 > subscriptionLimits.maximumCaptureAttempts) {
      throw new MisoUsageError(
        `track response capture attempts are capped at ${subscriptionLimits.maximumCaptureAttempts}`,
      );
    }
    const read = await responseRead(query);
    this.#assertEpoch(epoch);
    if (read.result === undefined) {
      throw new MisoEngineError("the first live response capture did not produce a result", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    const state = copyObservedState(read.state);
    const result = copyTrackResponseResult(read.result);
    this.#assertResponseResultBounds(result, state);
    const job: ResponseJobState = {
      id: this.#nextResponseJob++, key, query, refs: 0, state, result,
      revision: 1n, publicationSequence: 1n,
      retainedBytes: responseRetainedBytes(result, state),
      fastestCadenceMs: configuration.cadenceMs,
      nextCaptureAt: Date.now() + configuration.cadenceMs,
    };
    this.#assertResponseRetainedBytes(job.retainedBytes);
    this.#responseJobs.set(key, job);
    return job;
  }

  #newResponseHandle(
    configuration: TrackResponseSubscriptionConfiguration,
    callback: ((notification: TrackResponseSubscriptionNotification) => void) | undefined,
    job: ResponseJobState,
  ): ResponseHandleState {
    const state: ResponseHandleState = {
      id: this.#nextResponseHandle++, owner: this.#owner, epoch: this.#epoch,
      job, configuration, callback, cursor: job.publicationSequence,
      nextDeliveryAt: 0, closed: false, closing: undefined, publicHandle: undefined,
    };
    state.publicHandle = new TrackResponseSubscriptionImpl(this, state);
    return state;
  }

  #responseReceipt(state: ResponseHandleState): TrackResponseSubscriptionReceipt {
    return Object.freeze({
      handle: state.publicHandle!, owner: state.owner, epoch: state.epoch, job: state.job.id,
      revision: state.job.revision, configuration: state.configuration, bounds: this.responseBounds,
    });
  }

  #assertResponseResultBounds(result: TrackResponseResult, state: TrackResponseObservedState): void {
    const subscriptionLimits = this.#responseSubscriptionLimits!;
    if (result.frequenciesHz.length > this.responseBounds.maximumPoints
        || (result.leftDb?.length ?? 0) > this.responseBounds.maximumPoints
        || (result.rightDb?.length ?? 0) > this.responseBounds.maximumPoints) {
      throw new MisoUsageError("the live response points exceed the subscription bound");
    }
    if (responseRetainedBytes(result, state) > subscriptionLimits.maximumRetainedBytes) {
      throw new MisoUsageError("the live response retained bytes exceed the subscription bound");
    }
  }

  #assertResponseDeliveryBudget(
    candidate: ResponseJobState,
    candidateCadenceMs?: number,
    replacing?: ResponseHandleState,
    candidateResult?: TrackResponseResult,
  ): void {
    const maximum = this.#responseSubscriptionLimits!.maximumDeliveredBytesPerSecond;
    let delivered = 0;
    for (const handle of this.#responseHandles.values()) {
      if (handle === replacing) continue;
      const result = handle.job === candidate && candidateResult !== undefined
        ? candidateResult : handle.job.result;
      delivered += responseDeliveryRate(result, handle.configuration.cadenceMs);
      if (!Number.isFinite(delivered) || delivered > maximum) {
        throw new MisoUsageError("the live response vectors exceed the delivery bound");
      }
    }
    if (candidateCadenceMs !== undefined) {
      delivered += responseDeliveryRate(candidateResult ?? candidate.result, candidateCadenceMs);
      if (!Number.isFinite(delivered) || delivered > maximum) {
        throw new MisoUsageError("the live response vectors exceed the delivery bound");
      }
    }
  }

  #dropUnreferencedResponseJob(job: ResponseJobState): void {
    if (job.refs === 0 && this.#responseJobs.get(job.key) === job) this.#responseJobs.delete(job.key);
  }

  #refreshResponseJobCadence(job: ResponseJobState): void {
    if (job.refs === 0) return;
    let fastest = this.#responseSubscriptionLimits!.maximumCadenceMs;
    for (const handle of this.#responseHandles.values()) {
      if (handle.job === job && handle.configuration.cadenceMs < fastest) {
        fastest = handle.configuration.cadenceMs;
      }
    }
    if (fastest !== job.fastestCadenceMs) {
      job.fastestCadenceMs = fastest;
      job.nextCaptureAt = Date.now() + fastest;
    }
  }

  #assertResponseRetainedBytes(additional: number, replacing?: ResponseJobState): void {
    const subscriptionLimits = this.#responseSubscriptionLimits!;
    let retained = 0;
    for (const job of this.#responseJobs.values()) {
      if (job !== replacing) retained += job.retainedBytes;
    }
    if (!Number.isSafeInteger(retained + additional)
        || retained + additional > subscriptionLimits.maximumRetainedBytes) {
      throw new MisoUsageError("the live response retained bytes exceed the subscription bound");
    }
  }

  async #refreshResponseJobs(epoch: bigint): Promise<void> {
    const responseRead = this.#transport.responseRead;
    if (responseRead === undefined || this.#responseJobs.size === 0) return;
    const subscriptionLimits = this.#responseSubscriptionLimits!;
    const jobs = [...this.#responseJobs.values()];
    if (jobs.length > subscriptionLimits.maximumCaptureAttempts) {
      throw new MisoUsageError(
        `track response capture attempts are capped at ${subscriptionLimits.maximumCaptureAttempts}`,
      );
    }
    for (const job of jobs) {
      if (this.#responseJobs.get(job.key) !== job || job.refs === 0) continue;
      if (Date.now() < job.nextCaptureAt) continue;
      const read = await responseRead(job.query, job.state);
      this.#assertEpoch(epoch);
      job.nextCaptureAt = Date.now() + job.fastestCadenceMs;
      if (!read.changed) continue;
      if (read.result === undefined) {
        throw new MisoEngineError("the changed live response capture did not produce a result", {
          phase: "output", code: "abiMismatch", result: 2,
        });
      }
      const state = copyObservedState(read.state);
      const result = copyTrackResponseResult(read.result);
      this.#assertResponseResultBounds(result, state);
      this.#assertResponseDeliveryBudget(job, undefined, undefined, result);
      this.#assertResponseRetainedBytes(responseRetainedBytes(result, state), job);
      if (this.#responseJobs.get(job.key) !== job || job.refs === 0) continue;
      job.state = state;
      job.result = result;
      job.retainedBytes = responseRetainedBytes(result, state);
      job.revision += 1n;
      job.publicationSequence += 1n;
    }
  }

  #notifyResponse(
    state: ResponseHandleState,
    respectCadence: boolean,
  ): TrackResponseSubscriptionNotification | undefined {
    if (state.closed || state.publicHandle === undefined) return undefined;
    const job = this.#responseJobs.get(state.job.key);
    if (job === undefined) return undefined;
    const now = Date.now();
    if (respectCadence && now < state.nextDeliveryAt) return undefined;
    if (job.publicationSequence <= state.cursor) return undefined;
    const skippedPublications = job.publicationSequence - state.cursor - 1n;
    state.cursor = job.publicationSequence;
    if (respectCadence) state.nextDeliveryAt = now + state.configuration.cadenceMs;
    const notification = Object.freeze({
      handle: state.publicHandle,
      owner: this.#owner,
      epoch: this.#epoch,
      job: job.id,
      revision: job.revision,
      available: true,
      skippedPublications,
    });
    try { state.callback?.(notification); } catch { /* callbacks cannot break scheduling */ }
    return notification;
  }

  #assertResponseHandle(state: ResponseHandleState): void {
    this.#assertOpen();
    if (state.closed || state.epoch !== this.#epoch || this.#responseHandles.get(state.id) !== state) {
      throw new MisoUsageError("the track response subscription handle is closed or stale");
    }
  }

  #assertEpoch(epoch: bigint): void {
    if (epoch !== this.#epoch) {
      throw new MisoUsageError("the observation subscription owner changed while the request was pending");
    }
  }
}

/** Finite SDK-side bounds for managed live track-response jobs and handles. */
export interface TrackResponseSubscriptionLimits {
  readonly maximumHandles?: number;
  readonly maximumJobs?: number;
  readonly maximumRetainedBytes?: number;
  readonly maximumCaptureAttempts?: number;
  readonly maximumDeliveredBytesPerSecond?: number;
  readonly maximumCadenceMs?: number;
}

export const DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS = Object.freeze({
  maximumHandles: 64,
  maximumJobs: 16,
  maximumRetainedBytes: 16 * 1024 * 1024,
  maximumCaptureAttempts: 16,
  maximumDeliveredBytesPerSecond: 16 * 1024 * 1024,
  maximumCadenceMs: 60_000,
});

export interface TrackResponseSubscriptionRequest extends TrackResponseQuery {
  readonly cadenceMs?: number;
  readonly onUpdate?: (notification: TrackResponseSubscriptionNotification) => void;
}

export interface TrackResponseSubscriptionConfiguration extends TrackResponseQuery {
  readonly channels: "left" | "right" | "both";
  readonly responseLimits: TrackResponseLimits;
  readonly cadenceMs: number;
}

export interface TrackResponseSubscriptionBounds {
  readonly maximumHandles: number;
  readonly maximumJobs: number;
  readonly maximumRetainedBytes: number;
  readonly maximumCaptureAttempts: number;
  readonly maximumDeliveredBytesPerSecond: number;
  readonly maximumCadenceMs: number;
  readonly maximumPoints: number;
  readonly maximumCaptureBytes: number;
}

export interface TrackResponseSubscriptionNotification {
  readonly handle: TrackResponseSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  /** Monotonic observed captured-state identity for this owner/epoch/job. */
  readonly revision: bigint;
  readonly available: boolean;
  /** Known changed publications skipped by this handle's delivery cursor. */
  readonly skippedPublications: bigint;
}

export interface TrackResponseSubscriptionReceipt {
  readonly handle: TrackResponseSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  readonly revision: bigint;
  readonly configuration: TrackResponseSubscriptionConfiguration;
  readonly bounds: TrackResponseSubscriptionBounds;
}

export interface TrackResponseSubscription {
  readonly handle: TrackResponseSubscription;
  readonly id: bigint;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  readonly revision: bigint;
  readonly configuration: TrackResponseSubscriptionConfiguration;
  readonly bounds: TrackResponseSubscriptionBounds;
  readLatest(): TrackResponseResult | undefined;
  pump(): Promise<TrackResponseSubscriptionNotification | undefined>;
  update(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscriptionReceipt>;
  close(): Promise<void>;
}

/** Finite SDK-side limits for the one prepared managed spectrum job. */
export interface SpectrumSubscriptionLimits {
  readonly maximumHandles?: number;
  readonly maximumRetainedBytes?: number;
  readonly maximumDeliveredBytesPerSecond?: number;
  readonly maximumCadenceMs?: number;
}

export const DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS = Object.freeze({
  maximumHandles: 64,
  maximumRetainedBytes: 16 * 1024 * 1024,
  maximumDeliveredBytesPerSecond: 16 * 1024 * 1024,
  maximumCadenceMs: 60_000,
});

export interface SpectrumSubscriptionRequest extends SpectrumQuery {
  readonly smoothingMs?: number;
  readonly cadenceMs?: number;
  readonly onUpdate?: (notification: SpectrumSubscriptionNotification) => void;
}

export interface SpectrumSubscriptionConfiguration extends SpectrumQuery {
  readonly channels: NonNullable<SpectrumQuery["channels"]>;
  readonly smoothingMs: number;
  readonly cadenceMs: number;
}

export interface SpectrumSubscriptionBounds {
  readonly maximumHandles: number;
  readonly maximumRetainedBytes: number;
  readonly maximumDeliveredBytesPerSecond: number;
  readonly maximumCadenceMs: number;
  readonly maximumCaptureBytes: number;
  readonly maximumResultBytes: number;
  readonly hopFrames: number;
}

export interface SpectrumSubscriptionNotification {
  readonly handle: SpectrumSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  readonly revision: bigint;
  readonly status: SpectrumStreamStatus;
  readonly metadata: SpectrumStreamMetadata;
  readonly available: boolean;
  readonly nativeMissedWindows: bigint;
  readonly skippedPublications: bigint;
}

export interface SpectrumSubscriptionReceipt {
  readonly handle: SpectrumSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  readonly revision: bigint;
  readonly configuration: SpectrumSubscriptionConfiguration;
  readonly bounds: SpectrumSubscriptionBounds;
}

export interface SpectrumSubscription {
  readonly handle: SpectrumSubscription;
  readonly id: bigint;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly job: bigint;
  readonly revision: bigint;
  readonly configuration: SpectrumSubscriptionConfiguration;
  readonly bounds: SpectrumSubscriptionBounds;
  readLatest(): SpectrumResult | undefined;
  pump(): Promise<SpectrumSubscriptionNotification | undefined>;
  update(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscriptionReceipt>;
  close(): Promise<void>;
}

/** One capture/evaluation seam shared by browser and headless managed response owners. */
export interface TrackResponseSubscriptionTransport {
  responseRead(
    request: TrackResponseQuery,
    previousState?: TrackResponseObservedState,
  ): MaybePromise<TrackResponseRead>;
  readonly scheduler?: ObservationSubscriptionScheduler;
}

interface ResponseSubscriptionEffectiveLimits {
  readonly maximumHandles: number;
  readonly maximumJobs: number;
  readonly maximumRetainedBytes: number;
  readonly maximumCaptureAttempts: number;
  readonly maximumDeliveredBytesPerSecond: number;
  readonly maximumCadenceMs: number;
}

interface ResponseJobState {
  readonly id: bigint;
  readonly key: string;
  readonly query: TrackResponseQuery;
  refs: number;
  state: TrackResponseObservedState;
  result: TrackResponseResult;
  revision: bigint;
  publicationSequence: bigint;
  retainedBytes: number;
  fastestCadenceMs: number;
  nextCaptureAt: number;
}

interface ResponseHandleState {
  readonly id: bigint;
  readonly owner: bigint;
  epoch: bigint;
  job: ResponseJobState;
  configuration: TrackResponseSubscriptionConfiguration;
  callback: ((notification: TrackResponseSubscriptionNotification) => void) | undefined;
  cursor: bigint;
  nextDeliveryAt: number;
  closed: boolean;
  closing: Promise<void> | undefined;
  publicHandle: TrackResponseSubscriptionImpl | undefined;
}

interface SpectrumSubscriptionEffectiveLimits {
  readonly maximumHandles: number;
  readonly maximumRetainedBytes: number;
  readonly maximumDeliveredBytesPerSecond: number;
  readonly maximumCadenceMs: number;
}

interface SpectrumJobState {
  readonly id: bigint;
  key: string;
  query: SpectrumQuery;
  smoothingMs: number;
  refs: number;
  metadata: SpectrumStreamMetadata;
  result: SpectrumResult | undefined;
  revision: bigint;
  publicationSequence: bigint;
  retainedBytes: number;
  captureCadenceMs: number;
  nextCaptureAt: number;
  publicationStamp: string;
}

interface SpectrumHandleState {
  readonly id: bigint;
  readonly owner: bigint;
  readonly epoch: bigint;
  job: SpectrumJobState;
  configuration: SpectrumSubscriptionConfiguration;
  callback: ((notification: SpectrumSubscriptionNotification) => void) | undefined;
  cursor: bigint;
  nativeMissedEpoch: bigint;
  nativeMissedSeen: bigint;
  pendingNativeMissed: bigint;
  nextDeliveryAt: number;
  closed: boolean;
  closing: Promise<void> | undefined;
  publicHandle: SpectrumSubscriptionImpl | undefined;
}

function canonicalResponseSubscriptionLimits(overrides: TrackResponseSubscriptionLimits | undefined): ResponseSubscriptionEffectiveLimits {
  const value = (name: keyof ResponseSubscriptionEffectiveLimits, fallback: number): number => {
    const requested = overrides?.[name];
    if (requested === undefined) return fallback;
    if (!Number.isSafeInteger(requested) || requested <= 0) {
      throw new MisoUsageError(`${name} must be a positive safe integer`);
    }
    return requested;
  };
  return Object.freeze({
    maximumHandles: value("maximumHandles", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumHandles),
    maximumJobs: value("maximumJobs", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumJobs),
    maximumRetainedBytes: value("maximumRetainedBytes", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumRetainedBytes),
    maximumCaptureAttempts: value("maximumCaptureAttempts", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumCaptureAttempts),
    maximumDeliveredBytesPerSecond: value("maximumDeliveredBytesPerSecond", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumDeliveredBytesPerSecond),
    maximumCadenceMs: value("maximumCadenceMs", DEFAULT_TRACK_RESPONSE_SUBSCRIPTION_LIMITS.maximumCadenceMs),
  });
}

function normalizedTrackResponseSubscription(
  request: TrackResponseSubscriptionRequest,
  subscriptionLimits: ResponseSubscriptionEffectiveLimits,
): { readonly configuration: TrackResponseSubscriptionConfiguration; readonly callback: ((notification: TrackResponseSubscriptionNotification) => void) | undefined } {
  if (request === null || typeof request !== "object") {
    throw new MisoUsageError("track response subscription request must be an object");
  }
  const query = normalizeTrackResponseQuery(request);
  const cadenceMs = request.cadenceMs ?? 100;
  if (!Number.isSafeInteger(cadenceMs) || cadenceMs <= 0 || cadenceMs > subscriptionLimits.maximumCadenceMs) {
    throw new MisoUsageError(
      `track response cadenceMs must be an integer in 1..=${subscriptionLimits.maximumCadenceMs}`,
    );
  }
  if (request.onUpdate !== undefined && typeof request.onUpdate !== "function") {
    throw new MisoUsageError("track response onUpdate must be a function");
  }
  return {
    configuration: Object.freeze({
      trackId: query.trackId,
      grid: query.grid,
      channels: query.channels!,
      responseLimits: query.responseLimits!,
      cadenceMs,
    }),
    callback: request.onUpdate,
  };
}

function canonicalSpectrumSubscriptionLimits(
  overrides: SpectrumSubscriptionLimits | undefined,
): SpectrumSubscriptionEffectiveLimits {
  const positive = (name: keyof SpectrumSubscriptionEffectiveLimits, fallback: number): number => {
    const requested = overrides?.[name];
    if (requested === undefined) return fallback;
    if (!Number.isSafeInteger(requested) || requested <= 0) {
      throw new MisoUsageError(`${name} must be a positive safe integer`);
    }
    return requested;
  };
  return Object.freeze({
    maximumHandles: positive("maximumHandles", DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS.maximumHandles),
    maximumRetainedBytes: positive("maximumRetainedBytes", DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS.maximumRetainedBytes),
    maximumDeliveredBytesPerSecond: positive(
      "maximumDeliveredBytesPerSecond",
      DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS.maximumDeliveredBytesPerSecond,
    ),
    maximumCadenceMs: positive("maximumCadenceMs", DEFAULT_SPECTRUM_SUBSCRIPTION_LIMITS.maximumCadenceMs),
  });
}

function spectrumTargetKey(target: SpectrumQuery["target"]): string {
  return target.kind === "output"
    ? `${target.kind}\u0000${target.outputId}`
    : `${target.kind}\u0000${target.trackId}`;
}

function normalizedSpectrumSubscription(
  request: SpectrumSubscriptionRequest,
  subscriptionLimits: SpectrumSubscriptionEffectiveLimits,
): { readonly configuration: SpectrumSubscriptionConfiguration; readonly callback: ((notification: SpectrumSubscriptionNotification) => void) | undefined } {
  if (request === null || typeof request !== "object") {
    throw new MisoUsageError("spectrum subscription request must be an object");
  }
  const channels = request.channels ?? "both";
  const query = cloneSpectrumQuery({
    target: request.target,
    channels,
    ...(request.spectrumLimits === undefined ? {} : { spectrumLimits: request.spectrumLimits }),
  });
  const smoothingMs = request.smoothingMs ?? 100;
  if (typeof smoothingMs !== "number" || !Number.isFinite(smoothingMs)
      || smoothingMs < 0 || smoothingMs > 10_000) {
    throw new MisoUsageError("smoothingMs must be finite and in 0..=10000");
  }
  const cadenceMs = request.cadenceMs ?? 100;
  if (!Number.isSafeInteger(cadenceMs) || cadenceMs <= 0 || cadenceMs > subscriptionLimits.maximumCadenceMs) {
    throw new MisoUsageError(
      `spectrum cadenceMs must be an integer in 1..=${subscriptionLimits.maximumCadenceMs}`,
    );
  }
  if (request.onUpdate !== undefined && typeof request.onUpdate !== "function") {
    throw new MisoUsageError("spectrum onUpdate must be a function");
  }
  return {
    configuration: Object.freeze({
      target: query.target,
      channels: query.channels!,
      ...(query.spectrumLimits === undefined ? {} : { spectrumLimits: query.spectrumLimits }),
      smoothingMs,
      cadenceMs,
    }),
    callback: request.onUpdate,
  };
}

function spectrumJobKey(configuration: SpectrumSubscriptionConfiguration, query?: SpectrumQuery): string {
  const effectiveQuery = query ?? configuration;
  return JSON.stringify([
    spectrumTargetKey(configuration.target),
    configuration.channels,
    configuration.smoothingMs,
    effectiveQuery.spectrumLimits?.maximumCaptureBytes ?? ABI_LAYOUT.constants.spectrumCaptureBytes,
    effectiveQuery.spectrumLimits?.requestDeadlineMs ?? 5_000,
  ]);
}

function spectrumCaptureKey(query: SpectrumQuery): string {
  return JSON.stringify([
    spectrumTargetKey(query.target),
    query.channels ?? "both",
    query.spectrumLimits?.maximumCaptureBytes ?? ABI_LAYOUT.constants.spectrumCaptureBytes,
    query.spectrumLimits?.requestDeadlineMs ?? 5_000,
  ]);
}

function spectrumVectorBytes(channels: SpectrumSubscriptionConfiguration["channels"]): number {
  const channelCount = channels === "both" ? 2 : 1;
  return ABI_LAYOUT.constants.spectrumBinCount * Float32Array.BYTES_PER_ELEMENT * (channelCount + 1);
}

function spectrumDeliveryRate(bytes: number, cadenceMs: number): number {
  const rate = bytes * 1000 / cadenceMs;
  if (!Number.isFinite(rate) || rate < 0) {
    throw new MisoUsageError("the spectrum vectors exceed the delivery bound");
  }
  return rate;
}

function copySpectrumResult(result: SpectrumResult | undefined): SpectrumResult | undefined {
  if (result === undefined) return undefined;
  const target = result.target.kind === "output"
    ? Object.freeze({ kind: result.target.kind, outputId: result.target.outputId })
    : Object.freeze({ kind: result.target.kind, trackId: result.target.trackId });
  return Object.freeze({
    ...result,
    target,
    frequenciesHz: result.frequenciesHz.slice(),
    ...(result.leftDb === undefined ? {} : { leftDb: result.leftDb.slice() }),
    ...(result.rightDb === undefined ? {} : { rightDb: result.rightDb.slice() }),
  });
}

function spectrumResultVectorBytes(result: SpectrumResult): number {
  return result.frequenciesHz.byteLength
    + (result.leftDb?.byteLength ?? 0)
    + (result.rightDb?.byteLength ?? 0);
}

function spectrumResultRetainedBytes(result: SpectrumResult): number {
  return spectrumResultVectorBytes(result);
}

function spectrumPublicationStamp(metadata: SpectrumStreamMetadata): string {
  return [
    metadata.status,
    metadata.captureEpoch.toString(),
    metadata.sequence.toString(),
    metadata.droppedCaptures.toString(),
    metadata.analysisEpoch.toString(),
    metadata.capturedSample.toString(),
  ].join("\u0000");
}

function responseJobKey(query: TrackResponseQuery): string {
  return JSON.stringify([
    query.trackId,
    query.grid.kind,
    query.grid.points,
    query.grid.minimumHz,
    query.grid.maximumHz,
    query.channels,
    query.responseLimits?.maximumResultBytes,
    query.responseLimits?.requestDeadlineMs ?? 5_000,
  ]);
}

function copyObservedState(state: TrackResponseObservedState): TrackResponseObservedState {
  if (state === null || typeof state !== "object" || !Array.isArray(state.words)) {
    throw new MisoEngineError("the response state read was malformed", {
      phase: "output", code: "abiMismatch", result: 2,
    });
  }
  const words = state.words.map((word) => {
    if (!Number.isSafeInteger(word) || word < 0 || word > 0xffff_ffff) {
      throw new MisoEngineError("the response state read contained an invalid word", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    return word;
  });
  return Object.freeze({ words: Object.freeze(words) });
}

function responseVectorBytes(result: TrackResponseResult): number {
  return result.frequenciesHz.byteLength
    + (result.leftDb?.byteLength ?? 0)
    + (result.rightDb?.byteLength ?? 0);
}

function responseDeliveryRate(result: TrackResponseResult, cadenceMs: number): number {
  const bytes = responseVectorBytes(result);
  const rate = bytes * 1000 / cadenceMs;
  if (!Number.isFinite(rate) || rate < 0) {
    throw new MisoUsageError("the live response vectors exceed the delivery bound");
  }
  return rate;
}

function responseRetainedBytes(result: TrackResponseResult, state: TrackResponseObservedState): number {
  const resultBytes = Number(result.resultBytes);
  const keyBytes = state.words.length * 4;
  if (!Number.isSafeInteger(resultBytes) || resultBytes < 0
      || !Number.isSafeInteger(keyBytes) || resultBytes + keyBytes > Number.MAX_SAFE_INTEGER) {
    throw new MisoEngineError("the response subscription retained bytes exceeded its bound", {
      phase: "output", code: "refusedBudget", result: 8,
    });
  }
  return resultBytes + keyBytes;
}

function copyTrackResponseResult(result: TrackResponseResult): TrackResponseResult {
  return Object.freeze({
    ...result,
    frequenciesHz: result.frequenciesHz.slice(),
    ...(result.leftDb === undefined ? {} : { leftDb: result.leftDb.slice() }),
    ...(result.rightDb === undefined ? {} : { rightDb: result.rightDb.slice() }),
    members: Object.freeze(result.members.map((member) => Object.freeze({
      ...member,
      enabledLeft: Object.freeze([...member.enabledLeft]),
      enabledRight: Object.freeze([...member.enabledRight]),
    }))),
  });
}

class TrackResponseSubscriptionImpl implements TrackResponseSubscription {
  readonly #owner: ObservationSubscriptionOwner;
  readonly #state: ResponseHandleState;

  constructor(owner: ObservationSubscriptionOwner, state: ResponseHandleState) {
    this.#owner = owner;
    this.#state = state;
  }

  get handle(): TrackResponseSubscription { return this; }
  get id(): bigint { return this.#state.id; }
  get owner(): bigint { return this.#state.owner; }
  get epoch(): bigint { return this.#state.epoch; }
  get job(): bigint { return this.#state.job.id; }
  get revision(): bigint { return this.#state.job.revision; }
  get configuration(): TrackResponseSubscriptionConfiguration { return this.#state.configuration; }
  get bounds(): TrackResponseSubscriptionBounds { return this.#owner.responseBounds; }

  readLatest(): TrackResponseResult | undefined { return this.#owner.readLatestTrackResponse(this.#state); }
  pump(): Promise<TrackResponseSubscriptionNotification | undefined> { return this.#owner.pumpTrackResponse(this.#state); }
  update(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscriptionReceipt> {
    return this.#owner.updateTrackResponse(this.#state, request);
  }
  close(): Promise<void> { return this.#owner.closeTrackResponse(this.#state); }
}
