import { ABI_LAYOUT } from "../generated/abi.ts";
import { constantValue } from "./abi.ts";
import { CATALOG } from "../generated/catalog.ts";
import {
  decodeControlReply,
  decodeObservationReadReply,
  encodeObservationRead,
  encodeObserveTransaction,
  malformed,
  OBSERVATION_READ_BASE_BYTES,
  OBSERVATION_READ_MAXIMUM_SELECTIONS,
  OBSERVATION_READ_ROW_BYTES,
} from "./control-frame.ts";
import type {
  ControlStatusName,
  ObservationFrameRow,
  ObservationFrameSelection,
  ObservationReadPage,
  ObserveEdit,
} from "./control-frame.ts";
import type { ControlRequestIds } from "./control-queue.ts";
import { MisoEngineError, MisoUsageError } from "./errors.ts";
import type { MisoDiagnostic } from "./errors.ts";
import {
  MAXIMUM_OBSERVATION_READS,
  validateObservationSelections,
  type ObservationBinding,
  type ObservationChannels,
  type ObservationDescriptor,
  type ObservationMap,
  type ObservationReadResult,
  type ObservationSelection,
} from "./observation.ts";
import type { ObservationEnded } from "./observation-ended.ts";
import {
  TRACK_RESPONSE_MAXIMUM_RESULT_BYTES,
  normalizeTrackResponseQuery,
  sameTrackResponseResult,
} from "./live-response.ts";
import type {
  TrackResponseLimits,
  TrackResponseQuery,
  TrackResponseResult,
} from "./live-response.ts";
import {
  cloneSpectrumQuery,
  cloneSpectrumStreamMetadata,
  spectrumCaptureWindowBytes,
  SPECTRUM_MAXIMUM_CAPTURE_BYTES,
  spectrumRefusalResult,
} from "./spectrum.ts";
import { SPECTRUM_FRAME_BIN_COUNT } from "./spectrum-frame.ts";
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

/**
 * How many renders a refused subscription waits through before the refusal is thrown. One render
 * drains an effect's observation queue, so more than one means another writer keeps refilling it.
 */
const MAXIMUM_ADMISSION_RETRIES = 8;

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
  /**
   * Receives the end of selections the engine ended (#1562b-K): a rebuild removed the effect that
   * owned the tap, or the ID now names an effect without it. The core announces it with a reliable
   * event and the SDK reports it here once for each commit. A subscription whose selections have all
   * ended is closed.
   */
  readonly onEnd?: (end: ObservationSubscriptionEnd) => void;
}

/** What `onEnd` receives: the selections of one subscription that one commit ended. */
export interface ObservationSubscriptionEnd {
  readonly handle: ObservationSubscription;
  readonly owner: bigint;
  readonly epoch: bigint;
  /** The revision of the commit that ended them. */
  readonly revision: bigint;
  /** The selections of this subscription that this commit ended, as the subscription asked for them. */
  readonly ended: readonly ObservationSelection[];
  /** The selections still subscribed; empty when the whole subscription ended and was closed. */
  readonly remaining: readonly ObservationSelection[];
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
  readonly bounds: ObservationSubscriptionBounds;
  /** The handle's `armed()` for the configuration this receipt reports. */
  armed(): Promise<bigint>;
}

export interface ObservationSubscription {
  /** The handle itself; present so callers can treat subscribe's direct return as a receipt. */
  readonly handle: ObservationSubscription;
  readonly id: bigint;
  readonly owner: bigint;
  readonly epoch: bigint;
  readonly configuration: ObservationSubscriptionConfiguration;
  readonly bounds: ObservationSubscriptionBounds;
  /** Every selection of this subscription the engine has ended so far, in the order it ended them. */
  readonly ended: readonly ObservationSelection[];
  /**
   * The first sample at which every selection of the current configuration is in effect (#1057
   * design note 8.4): it resolves once a read reports, for every selection, the tap armed with the
   * arm tag of the `Observe` edit that armed its binding, with the largest of their arm samples. A
   * read is made by `pump()` and, in the browser, by the delivery timer; the headless engine reads
   * only when its caller pumps, so `armed()` stays pending until a render applied the arm and a
   * pump read it. It settles once per configuration (`update` that changes the selections or the
   * window starts a new one) and rejects with `MisoEngineError`: `wrongState`
   * `sdk.observation.closed` when the handle is closed or updated away first (or the engine
   * reboots), `wrongState` `sdk.engine.disposed` when the engine is disposed, `wrongState`
   * `sdk.observation.owner_removed` when a commit removed the effect that owns a selection, and
   * `internal` `sdk.observation.superseded` when a read shows a newer arm on a pending binding.
   */
  armed(): Promise<bigint>;
  readLatest(): readonly ObservationReadResult[];
  pump(): Promise<ObservationSubscriptionNotification | undefined>;
  /**
   * Replace the selections and the window. A window change on a tap this handle alone holds is one
   * re-arm: a disarm of the old window and an arm of the new one in one transaction. Like
   * `close()`, it meets a typed, retryable `backpressure` error when an earlier `Observe` of the
   * tap still holds its queue slot and no render has drained it yet. The headless engine refuses at
   * once, so the caller renders and calls again. The browser engine first waits for a render, up to
   * `requestDeadlineMs`, and ends in the same error when none comes.
   *
   * An `Observe` edit names its tap by stable key (the strip, the effect and the tap), and the core
   * resolves it on the newest committed plan, so an edit sent between a rebuild's commit and the
   * block that adopts the replacement plan arms the tap it names.
   */
  update(request: ObservationSubscriptionRequest): Promise<ObservationSubscriptionReceipt>;
  close(): Promise<void>;
}

type MaybePromise<T> = T | Promise<T>;

export interface ObservationSubscriptionScheduler {
  setInterval(callback: () => void, milliseconds: number): unknown;
  clearInterval(handle: unknown): void;
}

/** What one applied transaction of `Observe` edits reports (`TransactionApplied`, #1585 D5). */
export interface ObserveApplied {
  /** The committed revision the reply reports; an `Observe` transaction does not advance it. */
  readonly revision: bigint;
  /** The arm tag of each `Observe` edit, in edit order. */
  readonly observeArmTags: readonly bigint[];
  /** The tags of the subscriptions the commit ended. */
  readonly removedArmTags: readonly bigint[];
}

/**
 * Small transport seam shared by the browser and direct Wasm entries (#1589 D3): the core's own
 * frames, through the engine's one command entry. {@link observationFrameTransport} builds `apply`
 * and `read` from a frame endpoint, the same on both engines.
 */
export interface ObservationSubscriptionTransport {
  /** The map of the plan that renders: it checks a selection and names its effect before any edit. */
  observationMap(): MaybePromise<ObservationMap>;
  /**
   * Send one `SESSION_TRANSACTION_APPLY` of `Observe` edits. A refusal rejects with
   * `MisoEngineError`: `code` the result class, `diagnosticCode` the core's code; a full tap queue
   * is `code: "backpressure"`.
   */
  apply(edits: readonly ObserveEdit[]): Promise<ObserveApplied>;
  /** Read the selections with `OBSERVATION_READ`: one row per selection, in order. */
  read(selections: readonly ObservationFrameSelection[]): Promise<ObservationReadPage>;
  /** The running session's sample rate, which a read row reports. */
  sampleRateHz(): number;
  /**
   * Subscribe to the core's `OBSERVATION_ENDED` events (#1584): the listener gets each one, once,
   * as decoded from the reliable lane. Returns the unsubscribe function. A transport that cannot
   * read the lane leaves this absent, and a managed subscription then ends only when its owner
   * closes it.
   */
  observationEnded?(listener: (ended: ObservationEnded) => void): () => void;
  /**
   * A transport whose renders run on their own clock (a browser's audio thread) lets an `Observe`
   * transaction that was refused with `backpressure` wait for a render and be sent again. A
   * transport whose caller drives the renders leaves this absent: the refusal is thrown at once,
   * typed and retryable, and the caller renders and submits again.
   */
  readonly renderWait?: {
    /** The longest one managed submission waits for renders, across all of its resends. */
    readonly boundMs: number;
    /**
     * Reads the engine's render position. The owner asks for it before it sends a submission, so
     * a block rendered between that send and the refusal reaching the owner still counts as a
     * render after the send (it may have drained the slot).
     */
    mark(): Promise<bigint>;
    /**
     * Resolves `true` once the engine has rendered a block after `since` (a `mark()` read before
     * the refused send), and `false` when `timeoutMs` passes first or the engine is closed or
     * replaced. It never waits longer than `timeoutMs`.
     */
    next(timeoutMs: number, since: bigint): Promise<boolean>;
  };
  /**
   * Optional live track response read owned by the same lifetime: one `queryTrackResponse` of the
   * engine (#1579 D4). The owner compares each result with the one it published last.
   */
  readonly responseRead?: (request: TrackResponseQuery) => MaybePromise<TrackResponseResult>;
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
  readonly windowBlocks: number;
  readonly baseKey: string;
  readonly key: string;
}

interface BindingState {
  readonly key: string;
  readonly baseKey: string;
  readonly selection: ObservationSelection;
  readonly windowBlocks: number;
  readonly nativeEffectId: string;
  readonly descriptor: ObservationDescriptor;
  refs: number;
  latest: ObservationReadResult | undefined;
  lastSequence: bigint | undefined;
  nativeMissed: bigint;
  /** The arm tag the `Observe` edit that armed this binding was given (#1585 D5). */
  readonly armTag: bigint;
  /** The arm sample a read reported with {@link armTag} armed; `undefined` until one did. */
  armSample: bigint | undefined;
  /**
   * The revision the arm's reply reported. An `OBSERVATION_ENDED` event of a commit at or before it
   * belongs to an earlier arm of the same tap (a late event), and does not end this one.
   */
  readonly armRevision: bigint;
}

/** One settle-once `armed()` promise (#1057 design note 8.4). */
interface ArmedWait {
  readonly promise: Promise<bigint>;
  readonly resolve: (sample: bigint) => void;
  readonly reject: (error: MisoEngineError) => void;
  settled: boolean;
}

function armedWait(): ArmedWait {
  let resolve!: (sample: bigint) => void;
  let reject!: (error: MisoEngineError) => void;
  const promise = new Promise<bigint>((ok, fail) => { resolve = ok; reject = fail; });
  // A wait nobody asked for is not an unhandled rejection; a caller's `armed()` still sees it.
  promise.catch(() => undefined);
  const wait: ArmedWait = {
    promise,
    resolve: (sample) => { if (!wait.settled) { wait.settled = true; resolve(sample); } },
    reject: (error) => { if (!wait.settled) { wait.settled = true; reject(error); } },
    settled: false,
  };
  return wait;
}

function armedRefusal(code: "wrongState" | "internal", diagnostic: string, message: string): MisoEngineError {
  return new MisoEngineError(message, {
    phase: "lifecycle",
    code,
    result: constantValue("resultCodes", code),
    diagnostics: [{ code: diagnostic, path: "" }],
  });
}

interface HandleState {
  readonly id: bigint;
  readonly owner: bigint;
  epoch: bigint;
  configuration: ObservationSubscriptionConfiguration;
  entries: readonly ResolvedSelection[];
  /** The selections the engine ended. */
  ended: readonly ObservationSelection[];
  callback: ((notification: ObservationSubscriptionNotification) => void) | undefined;
  onEnd: ((end: ObservationSubscriptionEnd) => void) | undefined;
  /** The `armed()` of the current configuration. */
  armed: ArmedWait;
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
): {
  readonly configuration: ObservationSubscriptionConfiguration;
  readonly callback: ((notification: ObservationSubscriptionNotification) => void) | undefined;
  readonly onEnd: ((end: ObservationSubscriptionEnd) => void) | undefined;
} {
  if (request === null || typeof request !== "object") {
    throw new MisoUsageError("observation subscription request must be an object");
  }
  if (!Array.isArray(request.selections)) {
    throw new MisoUsageError("observation subscription selections must be an array");
  }
  if (request.selections.length > subscriptionLimits.maximumSelections) {
    throw new MisoUsageError(`observation subscription selections are capped at ${subscriptionLimits.maximumSelections}`);
  }
  // `readObservations` also validates, but admission must happen before the live-control
  // transaction. Calling it here avoids reserving a binding for a malformed target.
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
  if (request.onEnd !== undefined && typeof request.onEnd !== "function") {
    throw new MisoUsageError("observation onEnd must be a function");
  }
  return {
    configuration: Object.freeze({
      selections: Object.freeze(selections),
      windowBlocks: request.windowBlocks,
      cadenceMs,
    }),
    callback: request.onUpdate,
    onEnd: request.onEnd,
  };
}

function mapSelection(map: ObservationMap, selection: ObservationSelection, windowBlocks: number): ResolvedSelection {
  // A session whose plans bind no observation tap (#1589 R1: booted with no observation taps) has
  // an empty map; its refusal is the core's `observation.unavailable`, before any edit is sent.
  if (map.bindings.length === 0) {
    throw new MisoEngineError("this session binds no observation tap", {
      phase: "output",
      code: "unsupported",
      result: constantValue("resultCodes", "unsupported"),
      diagnostics: [{ code: "observation.unavailable", path: "" }],
    });
  }
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
    windowBlocks,
    baseKey: keyFor(selection),
    key: bindingKey(selection, windowBlocks),
  });
}

function tapDescriptor(binding: ObservationBinding, tapId: number): ObservationDescriptor {
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
  return tap;
}

/** The read selection of a binding: its tap with both lanes, which each handle then projects. */
function frameSelection(selection: ObservationSelection): ObservationFrameSelection {
  return Object.freeze({
    stripId: selection.trackId,
    rack: selection.rack,
    effectId: selection.effectSlotId,
    tapId: selection.tapId,
    channels: "both" as const,
  });
}

/**
 * The public row of one `OBSERVATION_READ` row for a binding. A window of another arm of the tap
 * (another tag) is never given to the binding: it reads `pending` (#1057 design note 8.4).
 */
function bindingRow(binding: BindingState, row: ObservationFrameRow, sampleRateHz: number): ObservationReadResult {
  const own = row.status !== "ready" || row.armTag === binding.armTag;
  const status = own ? row.status : "pending";
  return Object.freeze({
    trackId: binding.selection.trackId,
    rack: binding.selection.rack,
    effectSlotId: binding.selection.effectSlotId,
    tapId: binding.selection.tapId,
    nativeEffectId: binding.nativeEffectId,
    descriptor: binding.descriptor,
    channels: "both" as const,
    sampleRateHz,
    status,
    ...(status === "ready" && row.left !== undefined ? { left: row.left } : {}),
    ...(status === "ready" && row.right !== undefined ? { right: row.right } : {}),
    ...(status === "ready" && row.window !== undefined ? { window: row.window } : {}),
  });
}

/** The row a new binding holds until its first read: its arm is in flight. */
function pendingBindingRow(binding: BindingState, sampleRateHz: number): ObservationReadResult {
  return Object.freeze({
    trackId: binding.selection.trackId,
    rack: binding.selection.rack,
    effectSlotId: binding.selection.effectSlotId,
    tapId: binding.selection.tapId,
    nativeEffectId: binding.nativeEffectId,
    descriptor: binding.descriptor,
    channels: "both" as const,
    sampleRateHz,
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

/** How many times an `Observe` transaction is sent again after another sender committed first. */
const MAXIMUM_REVISION_RETRIES = 8;

/** The error a refused observation frame raises, as the spectrum and response codecs map it (#1596b). */
function observationRefusal(
  message: string,
  status: Exclude<ControlStatusName, "ok">,
  diagnostics: readonly { readonly code: string; readonly operationIndex?: number | undefined }[],
): MisoEngineError {
  const code = spectrumRefusalResult(status);
  const rows: MisoDiagnostic[] = diagnostics.length === 0
    ? [{ code: status, path: "" }]
    : diagnostics.map((row) => ({
      code: row.code,
      path: row.operationIndex === undefined ? "" : `edits[${row.operationIndex}]`,
    }));
  return new MisoEngineError(message, {
    phase: "output",
    code,
    result: constantValue("resultCodes", code),
    diagnostics: rows,
  });
}

/**
 * An `Observe`-only transaction the core refuses as a resource limit (`capi.resource.limit`) is
 * the full frame (#1676, #1685): the one event that would end every armed tap no longer fits one
 * control frame. No render makes room, so it is never resent; the error keeps its named diagnostic
 * `observationLimit` and says what does make room.
 */
function fullFrameRefusal(error: unknown): unknown {
  if (!(error instanceof MisoEngineError) || error.diagnosticCode !== "capi.resource.limit") return error;
  return new MisoEngineError(
    "the observation transaction was refused: observationLimit (the control frame that would end the armed observation taps is full; unsubscribe another tap first)",
    { phase: "output", code: error.code, result: error.result, diagnostics: [{ code: "observationLimit", path: "" }] },
  );
}

/** The engine's one command entry, as the observation owner sends frames through it (#1589 D3). */
export interface ObservationFrameEndpoint {
  /** Send one command frame and return its reply; a failed call throws (`controlCallFailure`). */
  send(frame: Uint8Array): MaybePromise<Uint8Array>;
  /** The endpoint's one request-ID allocator. */
  readonly requestIds: ControlRequestIds;
  /** The revision an `Observe` transaction is built against: the core's committed revision. */
  committedRevision(): MaybePromise<bigint>;
  /** The endpoint's frame ceiling in bytes, when it reports one. */
  maximumFrameBytes(): number | undefined;
}

/**
 * `apply` and `read` over an engine's command entry: one `SESSION_TRANSACTION_APPLY` of `Observe`
 * edits, and `OBSERVATION_READ` (#1589 D3). Both engines build their transport from this.
 *
 * An `Observe` transaction is refused `revision_conflict` when another sender committed between the
 * revision read and the send (the browser's live controls, a structural edit). An `Observe` writes
 * no model value, so the same edits at the revision the refusal reports are the same request; it is
 * sent again, at most {@link MAXIMUM_REVISION_RETRIES} times. Every other refusal rejects.
 *
 * A read of more selections than one reply frame can carry (`64 + 192 n` bytes, #1586 D4) is sent
 * as several reads, in order; the rows are one page.
 */
export function observationFrameTransport(
  endpoint: ObservationFrameEndpoint,
): Pick<ObservationSubscriptionTransport, "apply" | "read"> {
  const apply = async (edits: readonly ObserveEdit[]): Promise<ObserveApplied> => {
    let expectedRevision = await endpoint.committedRevision();
    for (let attempt = 0; ; attempt += 1) {
      const requestId = endpoint.requestIds.next();
      const frame = encodeObserveTransaction({ requestId, expectedRevision, edits });
      const limit = endpoint.maximumFrameBytes();
      if (limit !== undefined && frame.length > limit) {
        throw new MisoUsageError(
          `the observation transaction needs ${frame.length} bytes and one control frame holds ${limit}; subscribe fewer taps at once`,
        );
      }
      let bytes: Uint8Array;
      try {
        bytes = await endpoint.send(frame);
      } catch (error) {
        throw fullFrameRefusal(error);
      }
      const reply = decodeControlReply(bytes);
      if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
      const applied = reply.applied;
      if (applied !== undefined) {
        if (applied.observeArmTags.length !== edits.length) {
          throw malformed("an Observe transaction's reply holds no arm tag per edit");
        }
        return Object.freeze({
          revision: reply.revision,
          observeArmTags: applied.observeArmTags,
          removedArmTags: applied.removedArmTags,
        });
      }
      if (reply.status === "revision_conflict" && reply.revision !== expectedRevision
          && attempt < MAXIMUM_REVISION_RETRIES) {
        expectedRevision = reply.revision;
        continue;
      }
      throw observationRefusal(
        "the engine refused the observation transaction",
        reply.status as Exclude<ControlStatusName, "ok">,
        reply.diagnostics,
      );
    }
  };
  const read = async (selections: readonly ObservationFrameSelection[]): Promise<ObservationReadPage> => {
    const limit = endpoint.maximumFrameBytes();
    const perRead = limit === undefined
      ? OBSERVATION_READ_MAXIMUM_SELECTIONS
      : Math.max(1, Math.min(
        OBSERVATION_READ_MAXIMUM_SELECTIONS,
        Math.floor((limit - OBSERVATION_READ_BASE_BYTES) / OBSERVATION_READ_ROW_BYTES),
      ));
    const rows: ObservationFrameRow[] = [];
    let observedSample = 0n;
    for (let start = 0; start < selections.length; start += perRead) {
      const chunk = selections.slice(start, start + perRead);
      const requestId = endpoint.requestIds.next();
      const reply = decodeObservationReadReply(await endpoint.send(encodeObservationRead(requestId, chunk)));
      if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
      if (!reply.ok) throw observationRefusal("the engine refused the observation read", reply.status, reply.diagnostics);
      if (reply.value.rows.length !== chunk.length) throw malformed("an observation page holds the wrong row count");
      rows.push(...reply.value.rows);
      observedSample = reply.value.observedSample;
    }
    return Object.freeze({ observedSample, rows: Object.freeze(rows) });
  };
  return { apply, read };
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
  get bounds(): ObservationSubscriptionBounds { return this.#owner.bounds; }
  get ended(): readonly ObservationSelection[] { return this.#state.ended; }

  armed(): Promise<bigint> {
    return this.#state.armed.promise;
  }

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
  #unsubscribeEnded: (() => void) | undefined;
  /** `OBSERVATION_ENDED` events that arrived while a mutation was in flight; applied after it. */
  #endedWhileBusy: ObservationEnded[] = [];

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
    this.#unsubscribeEnded = transport.observationEnded?.((ended) => this.#onEnded(ended));
  }

  get bounds(): ObservationSubscriptionBounds {
    return Object.freeze({
      ...this.#subscriptionLimits,
      maximumReadSelections: MAXIMUM_OBSERVATION_READS,
    });
  }

  get responseBounds(): TrackResponseSubscriptionBounds {
    const subscriptionLimits = this.#responseSubscriptionLimits;
    if (subscriptionLimits === undefined) {
      throw new MisoUsageError("live response subscriptions are not configured on this owner");
    }
    return Object.freeze({
      ...subscriptionLimits,
      maximumPoints: ABI_LAYOUT.constants.maximumResponsePoints,
      maximumCaptureBytes: TRACK_RESPONSE_MAXIMUM_RESULT_BYTES,
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
      maximumCaptureBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES,
      maximumResultBytes: SPECTRUM_MAXIMUM_CAPTURE_BYTES,
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
      const map = await this.#map(epoch);
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
      const edits = newEntries.map((entry) => this.#edit(entry, true, normalized.configuration.windowBlocks));
      await this.#preflight(newEntries, epoch);
      const applied = await this.#apply(edits, epoch);
      this.#assertEpoch(epoch);
      const state = this.#newHandle(normalized.configuration, entries, normalized.callback, normalized.onEnd);
      newEntries.forEach((entry, index) => {
        this.#bindings.set(entry.key, this.#newBinding(entry, applied!, applied!.observeArmTags[index]!));
      });
      entries.filter((entry) => !newEntries.includes(entry)).forEach((entry) => {
        const held = this.#bindings.get(entry.key);
        if (held !== undefined) held.refs += 1;
      });
      this.#handles.set(state.id, state);
      // A subscription that joins bindings already armed resolves with their arm sample at once.
      this.#settleArmed(state);
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
      const map = await this.#map(epoch);
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
      // The bindings this handle alone holds and drops: each holds one core reference, which a
      // disarm ends. A window change is the disarm of the old window and the arm of the new one in
      // one transaction, which the core applies as one re-arm (#1584 gate 1).
      const release = remove.filter((entry) => {
        const binding = this.#bindings.get(entry.key);
        return binding !== undefined && binding.refs === 1;
      });
      const edits: ObserveEdit[] = [
        ...release.map((entry) => this.#edit(entry, false, entry.windowBlocks)),
        ...newEntries.map((entry) => this.#edit(entry, true, normalized.configuration.windowBlocks)),
      ];
      // A tap this handle alone re-arms with another window reads armed now; only the others must
      // read unarmed before they are acquired.
      await this.#preflight(newEntries.filter((entry) => !release.some((released) => released.baseKey === entry.baseKey)), epoch);
      const applied = await this.#apply(edits, epoch);
      this.#assertEpoch(epoch);
      release.forEach((entry) => this.#bindings.delete(entry.key));
      remove.filter((entry) => !release.includes(entry)).forEach((entry) => {
        const binding = this.#bindings.get(entry.key);
        if (binding !== undefined) binding.refs -= 1;
      });
      newEntries.forEach((entry, index) => {
        this.#bindings.set(entry.key, this.#newBinding(entry, applied!, applied!.observeArmTags[release.length + index]!));
      });
      add.filter((entry) => !newEntries.includes(entry)).forEach((entry) => {
        const held = this.#bindings.get(entry.key);
        if (held !== undefined) held.refs += 1;
      });
      // `armed()` follows the selections: a configuration that removed or added one is another wait,
      // and a wait still pending on the old one is closed.
      if (remove.length !== 0 || add.length !== 0) {
        state.armed.reject(armedRefusal(
          "wrongState", "sdk.observation.closed", "the observation subscription was updated before it was armed",
        ));
        state.armed = armedWait();
      }
      state.configuration = normalized.configuration;
      state.entries = entries;
      state.callback = normalized.callback;
      state.onEnd = normalized.onEnd;
      state.cursor.clear();
      state.nativeMissedSeen.clear();
      state.nextDeliveryAt = 0;
      this.#settleArmed(state);
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
      const edits = disarm.map((entry) => this.#edit(entry, false, entry.windowBlocks));
      await this.#apply(edits, epoch);
      this.#assertEpoch(epoch);
      disarm.forEach((entry) => this.#bindings.delete(entry.key));
      state.entries.filter((entry) => !disarm.includes(entry)).forEach((entry) => {
        const binding = this.#bindings.get(entry.key);
        if (binding !== undefined) binding.refs -= 1;
      });
      this.#handles.delete(state.id);
      state.closed = true;
      state.armed.reject(armedRefusal(
        "wrongState", "sdk.observation.closed", "the observation subscription was closed before it was armed",
      ));
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

  /**
   * Invalidate all old handles at a headless replacement or browser disposal. A pending `armed()`
   * rejects: `sdk.engine.disposed` when the engine is gone, `sdk.observation.closed` when a reboot
   * closed the handles.
   */
  invalidate(disposed = false): void {
    this.#epoch += 1n;
    const ended = disposed
      ? armedRefusal("wrongState", "sdk.engine.disposed", "the engine was disposed before the subscription was armed")
      : armedRefusal("wrongState", "sdk.observation.closed", "the engine was rebooted before the subscription was armed");
    for (const state of this.#handles.values()) {
      state.closed = true;
      state.armed.reject(ended);
    }
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
    this.#endedWhileBusy = [];
    if (disposed) {
      this.#disposed = true;
      this.#unsubscribeEnded?.();
      this.#unsubscribeEnded = undefined;
    }
  }

  /**
   * The engine ended taps (`OBSERVATION_ENDED`): the commit removed the effect that owned them, or
   * the ID now names an effect without that tap. The core decided it and has already dropped the
   * arms, so nothing is sent; the owner forgets the bindings and tells each subscription once what
   * it lost (#1562b-K). A subscription left with no selection is closed, and one that lost a
   * selection before it was armed rejects its `armed()` with `sdk.observation.owner_removed`.
   *
   * The event, not the reply's `removedArmTags`, settles it: the commit that removes an effect is
   * any sender's (a structural edit, a replace, a frame the app sent itself), and only the event
   * reaches this owner for every one of them.
   */
  #onEnded(ended: ObservationEnded): void {
    if (this.#disposed) return;
    if (this.#mutationBusy) {
      this.#endedWhileBusy.push(ended);
      return;
    }
    const lost = new Set(ended.keys.map((key) => keyFor({
      trackId: key.stripId, rack: key.rack, effectSlotId: key.effectSlotId, tapId: key.tapId, channels: "both",
    })));
    const gone = new Set<string>();
    for (const binding of [...this.#bindings.values()]) {
      if (!lost.has(binding.baseKey)) continue;
      // An event of a commit at or before this arm's reply ended an earlier arm of the tap (the
      // event came late, behind a removal and a re-add); this arm is the core's now.
      if (ended.revision <= binding.armRevision) continue;
      this.#bindings.delete(binding.key);
      gone.add(binding.baseKey);
    }
    if (gone.size === 0) return;
    for (const state of [...this.#handles.values()]) {
      const dropped = state.entries.filter((entry) => gone.has(entry.baseKey));
      if (dropped.length === 0) continue;
      state.armed.reject(armedRefusal(
        "wrongState", "sdk.observation.owner_removed", "a commit removed the effect of a subscription before it was armed",
      ));
      const remaining = state.entries.filter((entry) => !gone.has(entry.baseKey));
      for (const entry of dropped) {
        state.cursor.delete(entry.key);
        state.nativeMissedSeen.delete(entry.key);
      }
      state.entries = Object.freeze(remaining);
      const droppedSelections = Object.freeze(dropped.map((entry) => entry.selection));
      state.ended = Object.freeze([...state.ended, ...droppedSelections]);
      if (remaining.length === 0) {
        state.closed = true;
        this.#handles.delete(state.id);
      }
      if (state.publicHandle === undefined) continue;
      const end: ObservationSubscriptionEnd = Object.freeze({
        handle: state.publicHandle,
        owner: this.#owner,
        epoch: this.#epoch,
        revision: ended.revision,
        ended: droppedSelections,
        remaining: Object.freeze(remaining.map((entry) => entry.selection)),
      });
      try { state.onEnd?.(end); } catch { /* callbacks cannot break the owner */ }
    }
    this.#stopTimerIfIdle();
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

  #newHandle(
    configuration: ObservationSubscriptionConfiguration,
    entries: readonly ResolvedSelection[],
    callback: ((notification: ObservationSubscriptionNotification) => void) | undefined,
    onEnd: ((end: ObservationSubscriptionEnd) => void) | undefined,
  ): HandleState {
    const state: HandleState = {
      id: this.#nextHandle++,
      owner: this.#owner,
      epoch: this.#epoch,
      configuration,
      entries: Object.freeze([...entries]),
      ended: Object.freeze([]),
      callback,
      onEnd,
      armed: armedWait(),
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

  #newBinding(entry: ResolvedSelection, applied: ObserveApplied, armTag: bigint): BindingState {
    const binding: BindingState = {
      key: entry.key,
      baseKey: entry.baseKey,
      selection: entry.selection,
      windowBlocks: entry.windowBlocks,
      nativeEffectId: entry.binding.nativeEffectId,
      descriptor: tapDescriptor(entry.binding, entry.selection.tapId),
      refs: 1,
      latest: undefined,
      lastSequence: undefined,
      nativeMissed: 0n,
      armTag,
      armSample: undefined,
      armRevision: applied.revision,
    };
    binding.latest = pendingBindingRow(binding, this.#transport.sampleRateHz());
    return binding;
  }

  #receipt(state: HandleState): ObservationSubscriptionReceipt {
    const armed = state.armed.promise;
    return Object.freeze({
      handle: state.publicHandle!,
      owner: state.owner,
      epoch: state.epoch,
      configuration: state.configuration,
      bounds: this.bounds,
      armed: () => armed,
    });
  }

  #newEntries(entries: readonly ResolvedSelection[]): readonly ResolvedSelection[] {
    return entries.filter((entry) => !this.#bindings.has(entry.key));
  }

  /** Resolve `state.armed` once every selection's binding read armed with its own tag. */
  #settleArmed(state: HandleState): void {
    if (state.armed.settled) return;
    let sample = 0n;
    for (const entry of state.entries) {
      const armSample = this.#bindings.get(entry.key)?.armSample;
      if (armSample === undefined) return;
      if (armSample > sample) sample = armSample;
    }
    if (state.entries.length !== 0) state.armed.resolve(sample);
  }

  /**
   * A new binding's tap must read unarmed: one armed already is held by someone else (another
   * sender's `Observe`), whose window this owner cannot share.
   */
  async #preflight(newEntries: readonly ResolvedSelection[], epoch: bigint): Promise<void> {
    if (newEntries.length === 0) return;
    const page = await this.#transport.read(newEntries.map((entry) => frameSelection(entry.selection)));
    this.#assertEpoch(epoch);
    if (page.rows.length !== newEntries.length) {
      throw new MisoEngineError("the observation preflight returned the wrong row count", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    if (page.rows.some((row) => row.status !== "unarmed")) {
      throw new MisoUsageError("the observation is already armed by another subscriber and cannot be acquired");
    }
  }

  #edit(entry: ResolvedSelection, armed: boolean, windowBlocks: number): ObserveEdit {
    // The edit names its tap by stable key, which the selection already is: the strip, the console
    // slot or insert, the tap. The core resolves it on the newest committed plan, so nothing here can
    // address another effect after a rebuild (#1677). The catalog says whether the tap is
    // subscribable at all, which is refused before anything is sent.
    tapDescriptor(entry.binding, entry.selection.tapId);
    return Object.freeze({
      stripId: entry.selection.trackId,
      rack: entry.selection.rack,
      effectId: entry.selection.effectSlotId,
      tapId: entry.selection.tapId,
      armed,
      windowBlocks,
    });
  }

  /**
   * The map of the plan that renders. It validates the selection (the effect and its tap exist, a
   * read can reach them) and names its effect. It does not address an edit: the edit names the tap
   * by stable key and the core resolves it on the newest committed plan (#1677).
   */
  async #map(epoch: bigint): Promise<ObservationMap> {
    const map = await this.#transport.observationMap();
    this.#assertEpoch(epoch);
    return map;
  }

  /**
   * Send one transaction of `Observe` edits; none sends nothing. A refusal for room
   * (`backpressure`) means an earlier `Observe` of a tap still holds its queue slot, and a render
   * drains it: that refusal alone is flow control, typed and retryable. Anything else, a malformed
   * or unbound request or one past a full frame of armed taps included, never succeeds on resend
   * and is thrown at once.
   */
  async #apply(edits: readonly ObserveEdit[], epoch: bigint): Promise<ObserveApplied | undefined> {
    if (edits.length === 0) return undefined;
    const wait = this.#transport.renderWait;
    let deadline: number | undefined;
    for (let attempt = 0; ; attempt += 1) {
      this.#assertEpoch(epoch);
      // The position is requested before the send, in the same synchronous turn, so the host
      // answers it first. A block rendered after it and before the refusal returns is then seen.
      const since = wait?.mark();
      since?.catch(() => undefined);
      try {
        return await this.#transport.apply(edits);
      } catch (error) {
        if (!(error instanceof MisoEngineError) || error.code !== "backpressure" || wait === undefined
            || attempt >= MAXIMUM_ADMISSION_RETRIES) {
          throw error;
        }
        // Renders on their own clock: wait for one within the bound, then send the same edits
        // again. When the bound passes without a render the refusal stands, so a context that does
        // not render ends in the typed error and never in an endless wait.
        deadline ??= Date.now() + wait.boundMs;
        const remaining = deadline - Date.now();
        if (remaining <= 0) throw error;
        const rendered = await wait.next(remaining, await since!);
        this.#assertEpoch(epoch);
        if (!rendered) throw error;
      }
    }
  }

  async #refresh(epoch = this.#epoch): Promise<void> {
    if (this.#bindings.size === 0) return;
    const bindings = [...this.#bindings.values()];
    if (bindings.length > MAXIMUM_OBSERVATION_READS) {
      throw new MisoUsageError(`managed observation reads are capped at ${MAXIMUM_OBSERVATION_READS}`);
    }
    const page = await this.#transport.read(bindings.map((binding) => frameSelection(binding.selection)));
    this.#assertEpoch(epoch);
    if (page.rows.length !== bindings.length) {
      throw new MisoEngineError("the observation read returned the wrong row count", {
        phase: "output", code: "abiMismatch", result: 2,
      });
    }
    const sampleRateHz = this.#transport.sampleRateHz();
    const superseded = new Set<string>();
    page.rows.forEach((row, index) => {
      const binding = bindings[index]!;
      if (binding.armSample === undefined) {
        if (row.armed && row.armTag === binding.armTag) binding.armSample = row.armSample;
        // By the reference rule a newer `Observe` of the tap is sent only after this binding's last
        // subscription closed, so a newer tag on a pending binding is an inconsistent state.
        else if (row.armTag > binding.armTag) superseded.add(binding.key);
      }
      const result = bindingRow(binding, row, sampleRateHz);
      const previous = binding.lastSequence;
      if (result.status === "ready" && result.window !== undefined) {
        if (previous !== undefined && result.window.sequence > previous + 1n) {
          binding.nativeMissed += result.window.sequence - previous - 1n;
        }
        if (previous === undefined || result.window.sequence > previous) binding.lastSequence = result.window.sequence;
      }
      binding.latest = result;
    });
    for (const state of this.#handles.values()) {
      if (state.entries.some((entry) => superseded.has(entry.key))) {
        state.armed.reject(armedRefusal(
          "internal", "sdk.observation.superseded", "a read showed a newer arm on a pending observation binding",
        ));
      }
      this.#settleArmed(state);
    }
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
      if (state.ended.length > 0 && state.entries.length === 0 && state.epoch === this.#epoch) {
        throw new MisoUsageError("the observation subscription ended: the engine removed every tap it held");
      }
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
    return run.finally(() => {
      this.#mutationBusy = false;
      // An end that arrived mid-operation applies now, to the state the operation left.
      const waiting = this.#endedWhileBusy;
      this.#endedWhileBusy = [];
      for (const ended of waiting) this.#onEnded(ended);
    });
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
          publicationMetadata: undefined,
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
        publicationMetadata: undefined,
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
      ?? SPECTRUM_MAXIMUM_CAPTURE_BYTES;
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
      ?? SPECTRUM_MAXIMUM_CAPTURE_BYTES;
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
    let recovery: SpectrumStreamRead;
    try {
      recovery = await spectrumRead(job.query);
    } catch (error) {
      for (const state of this.#spectrumHandles.values()) this.#notifySpectrum(state, true);
      throw error;
    }
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
      // Keep the live native state current for loss accounting. Publication metadata is updated
      // below only when a new public result/status is committed, so a pending read cannot relabel
      // an undispatched ready publication.
      job.metadata = metadata;
      if (job.publicationStamp !== stamp || job.result === undefined) {
        const retainedBytes = spectrumResultRetainedBytes(read.result);
        job.result = copySpectrumResult(read.result);
        job.retainedBytes = retainedBytes;
        job.publicationMetadata = metadata;
        job.revision += 1n;
        job.publicationSequence += 1n;
        job.publicationStamp = stamp;
      }
      return;
    }
    // A non-result read still advances the live native state for loss accounting, while only a
    // gap or failure is allowed to replace the public publication snapshot.
    job.metadata = metadata;
    if (job.publicationStamp !== stamp && (metadata.status === "gap" || metadata.status === "failed")) {
      job.publicationMetadata = metadata;
      job.revision += 1n;
      job.publicationSequence += 1n;
      job.publicationStamp = stamp;
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
    const publicationMetadata = job.publicationMetadata;
    if (publicationMetadata === undefined) return undefined;
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
      status: publicationMetadata.status,
      metadata: publicationMetadata,
      available: job.result !== undefined && publicationMetadata.status === "ready",
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
    const result = copyTrackResponseResult(read);
    this.#assertResponseResultBounds(result);
    const job: ResponseJobState = {
      id: this.#nextResponseJob++, key, query, refs: 0, result,
      revision: 1n, publicationSequence: 1n,
      retainedBytes: responseRetainedBytes(result),
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

  #assertResponseResultBounds(result: TrackResponseResult): void {
    const subscriptionLimits = this.#responseSubscriptionLimits!;
    if (result.frequenciesHz.length > this.responseBounds.maximumPoints
        || (result.leftDb?.length ?? 0) > this.responseBounds.maximumPoints
        || (result.rightDb?.length ?? 0) > this.responseBounds.maximumPoints) {
      throw new MisoUsageError("the live response points exceed the subscription bound");
    }
    if (responseRetainedBytes(result) > subscriptionLimits.maximumRetainedBytes) {
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
      const read = await responseRead(job.query);
      this.#assertEpoch(epoch);
      job.nextCaptureAt = Date.now() + job.fastestCadenceMs;
      // Unchanged: nothing is published (#1579 D4).
      if (sameTrackResponseResult(job.result, read)) continue;
      const result = copyTrackResponseResult(read);
      this.#assertResponseResultBounds(result);
      this.#assertResponseDeliveryBudget(job, undefined, undefined, result);
      this.#assertResponseRetainedBytes(responseRetainedBytes(result), job);
      if (this.#responseJobs.get(job.key) !== job || job.refs === 0) continue;
      job.result = result;
      job.retainedBytes = responseRetainedBytes(result);
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
  /**
   * Read the job when its cadence is due and deliver a changed result. On the headless engine the
   * read is a query that settles at the next `render()` the app calls.
   */
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
  responseRead(request: TrackResponseQuery): MaybePromise<TrackResponseResult>;
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
  /** Metadata belonging to the latest public publication, kept separate from live native state. */
  publicationMetadata: SpectrumStreamMetadata | undefined;
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
    effectiveQuery.spectrumLimits?.maximumCaptureBytes ?? SPECTRUM_MAXIMUM_CAPTURE_BYTES,
    effectiveQuery.spectrumLimits?.requestDeadlineMs ?? 5_000,
  ]);
}

function spectrumCaptureKey(query: SpectrumQuery): string {
  return JSON.stringify([
    spectrumTargetKey(query.target),
    query.channels ?? "both",
    query.spectrumLimits?.maximumCaptureBytes ?? SPECTRUM_MAXIMUM_CAPTURE_BYTES,
    query.spectrumLimits?.requestDeadlineMs ?? 5_000,
  ]);
}

function spectrumVectorBytes(channels: SpectrumSubscriptionConfiguration["channels"]): number {
  const channelCount = channels === "both" ? 2 : 1;
  return SPECTRUM_FRAME_BIN_COUNT * Float32Array.BYTES_PER_ELEMENT * (channelCount + 1);
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

function responseRetainedBytes(result: TrackResponseResult): number {
  const resultBytes = Number(result.resultBytes);
  if (!Number.isSafeInteger(resultBytes) || resultBytes < 0) {
    throw new MisoEngineError("the response subscription retained bytes exceeded its bound", {
      phase: "output", code: "refusedBudget", result: 8,
    });
  }
  return resultBytes;
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
