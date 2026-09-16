import { MisoEngineAsset } from "../core/asset.ts";
import { WasmBoundary } from "../core/boundary.ts";
import type { EngineCallResult, SessionShape } from "../core/boundary.ts";
import { MisoEngineError, MisoUsageError, resultName } from "../core/errors.ts";
import { constantValue } from "../core/abi.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import type { SourceSpec } from "../core/types.ts";
import {
  assertQuantumMatch,
  assertWebDeliverableSources,
  scratchBootOptions,
  validateBrowserSpectrumHopFrames,
  workletBootOptions,
} from "./policy.ts";
import type { BrowserBootPolicy } from "./policy.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import type {
  MisoAudioWorkletHost,
  MisoSpectrumStreamMetadata,
} from "./shipped-host.d.ts";
import { createMeasurementFeeds } from "./measurement.ts";
import type {
  MeterListener,
  TelemetryListener,
} from "./measurement.ts";
import type { EngineConsole } from "../core/console.ts";
import { scratchBootWithWorker } from "./scratch.ts";
import type { ScratchWorkerFactory } from "./scratch.ts";
import { createDefaultHost, BrowserBootError } from "./default-host.ts";
import { createBrowserConsole } from "./console.ts";
import { createTrackResponse } from "./response.ts";
import type { BrowserTrackResponse } from "./response.ts";
import { createSpectrum } from "./response.ts";
import type { BrowserSpectrum } from "./response.ts";
import {
  parseTrackResponseState,
  sameTrackResponseState,
} from "../core/live-response.ts";
import type {
  TrackResponseHostRequest,
  TrackResponseObservedState,
  TrackResponseQuery,
  TrackResponseRead,
  TrackResponseResult,
} from "../core/live-response.ts";
import type { ResponseWorkerFactory } from "./response-worker.ts";
import {
  cloneSpectrumCollection,
  cloneSpectrumQuery,
  cloneSpectrumStreamMetadata,
} from "../core/spectrum.ts";
import type {
  SpectrumCollection,
  SpectrumQuery,
  SpectrumResult,
  SpectrumStreamSelection,
  SpectrumStreamMetadata,
  SpectrumStreamRead,
  SpectrumStreamStart,
  SpectrumStreamStatus,
} from "../core/spectrum.ts";
import {
  ObservationSubscriptionOwner,
} from "../core/observation-subscriptions.ts";
import type {
  ObservationSubscription,
  ObservationSubscriptionLimits,
  ObservationSubscriptionRequest,
  TrackResponseSubscription,
  TrackResponseSubscriptionLimits,
  TrackResponseSubscriptionRequest,
  SpectrumSubscription,
  SpectrumSubscriptionLimits,
  SpectrumSubscriptionRequest,
} from "../core/observation-subscriptions.ts";
import {
  decodeObservationRows,
  enrichObservationMap,
  resolveObservationAddressesWithTracks,
  validateObservationSelections,
} from "../core/observation.ts";
import type {
  ObservationMap,
  ObservationReadResult,
  ObservationSelection,
  RawObservationBinding,
  RawObservationRow,
} from "../core/observation.ts";

/**
 * The browser entry (issue #243 S3, consuming #240 S5's sealed choreography).
 *
 * # Why a scratch boot exists at all
 *
 * A headless engine has no physical shape to satisfy: it accepts whatever the document declares.
 * A browser does. An `AudioContext` is constructed at a sample rate, and once constructed it is
 * expensive to replace and, on some platforms, limited in how often it may be. So the browser
 * cannot simply hand the document to the worklet and see what happens -- by then it has already
 * committed to a rate.
 *
 * The scratch boot answers the question first: boot the document in a Worker, with both `require_*`
 * words at zero, ask it what shape it declared, dispose it. That answer is what the `AudioContext`
 * is then constructed for. The Worker is mandatory rather than convenient: compiling and booting a
 * two-and-a-half megabyte module on the main realm blocks it for long enough to be visible, and
 * the answer is wanted before any audio graph exists.
 *
 * # The two boots agree, and where they do not they agree about that too
 *
 * Both boots read the same policy object, so the policy words -- ring, memory budget, and all four
 * console words -- are identical by construction. The two `require_*` words are role-defined: zero
 * in the scratch boot, physical in the worklet boot. See `./policy.ts` for why "identical options
 * struct" had to be restated that way, and for the executable form of the rule.
 *
 * # This file is deliberately thin
 *
 * Everything decidable lives in `./policy.ts` as pure functions, because those are what a harness
 * can prove. What remains here is `AudioContext` construction and module loading, which only a
 * browser can exercise; #246 owns those matrices end to end.
 */

/** The narrow slice of the Web Audio surface this entry needs. */
export interface AudioContextLike {
  readonly sampleRate: number;
  readonly renderQuantumSize?: number;
  readonly state: string;
  close(): Promise<void>;
  readonly audioWorklet: { addModule(url: string): Promise<void> };
}

/** Resolves against the consumer's ambient browser constructor without requiring DOM libs here. */
export type DefaultAudioContext = typeof globalThis extends {
  AudioContext: abstract new (...args: never[]) => infer Context extends AudioContextLike;
} ? Context : AudioContextLike;

export interface CreateEngineOptions<Context extends AudioContextLike = AudioContextLike> {
  /** The Session V1 document, or the SDK builder session that produced it. */
  readonly document: Uint8Array | string | { toJson(): string };
  /**
   * The sources this session declares, when they were authored through the SDK.
   *
   * Supplied so a `32f` document can be refused before an `AudioContext` is constructed. Absent
   * for a raw document, whose sources the SDK has never seen as values -- that case belongs to the
   * resolver/ingest boundary (#244), and the SDK has no JSON parser to substitute for one.
   */
  readonly sources?: readonly { readonly id: string; readonly spec: SourceSpec }[];
  /** Release URLs, from the same release as the module bytes. */
  readonly simd128ModuleUrl?: string;
  readonly preparedModule?: WebAssembly.Module;
  readonly workletModuleUrl?: string;
  readonly hostModuleUrl?: string;
  readonly scratchWorkerModuleUrl?: string;
  readonly createWorker?: ScratchWorkerFactory;
  readonly responseWorkerModuleUrl?: string | URL;
  readonly createResponseWorker?: ResponseWorkerFactory;
  /** One prepared spectrum boundary. The query target must match it. */
  readonly spectrum?: SpectrumQuery;
  /** Several exact prepared spectrum boundaries for one atomic managed owner. */
  readonly spectrumCollection?: SpectrumCollection;
  /** Finite SDK-side bounds for resident observation subscriptions. */
  readonly observationSubscriptionLimits?: ObservationSubscriptionLimits;
  /** Finite SDK-side bounds for managed live track-response subscriptions. */
  readonly responseSubscriptionLimits?: TrackResponseSubscriptionLimits;
  /** Finite SDK-side bounds for managed spectrum subscriptions. */
  readonly spectrumSubscriptionLimits?: SpectrumSubscriptionLimits;
  readonly requestDeadlineMs?: number;
  readonly signal?: AbortSignal;
  /** Constructs an `AudioContext` at the requested rate. Injected so the entry stays testable. */
  readonly createContext?: (options: {
    readonly sampleRate: number;
    readonly renderSizeHint: number;
  }) => Context;
  /** Boots a scratch instance in a Worker and returns the shape it read back. */
  readonly scratchBoot?: (request: {
    readonly document: Uint8Array;
    readonly options: ReturnType<typeof scratchBootOptions>;
  }) => Promise<SessionShape>;
  /** Creates the worklet host once the context is verified. Normally the shipped factory. */
  readonly createHost?: (request: {
    readonly context: Context;
    readonly document: Uint8Array;
    readonly options: ReturnType<typeof workletBootOptions>;
    readonly simd128ModuleUrl: string;
    readonly preparedModule?: WebAssembly.Module;
    readonly workletModuleUrl: string;
  }) => Promise<MisoAudioWorkletHost>;
  readonly policy?: BrowserBootPolicy;
  /** How many construct-verify-close-retry rounds to allow. */
  readonly contextAttempts?: number;
}

export interface BrowserEngine<Context extends AudioContextLike = DefaultAudioContext> {
  readonly shape: SessionShape;
  readonly context: Context;
  readonly host: MisoAudioWorkletHost;
  /** One shared decimated meter lease for all SDK listeners. */
  subscribeMeters(listener: MeterListener): Promise<() => void>;
  /** One shared render-telemetry lease for all SDK listeners. */
  subscribeTelemetry(listener: TelemetryListener): Promise<() => void>;
  /** Read the current prepared owner's stable resident-observation bindings. */
  observationMap(): Promise<ObservationMap>;
  /** Read one bounded non-consuming batch from the current prepared owner. */
  readObservations(selections: readonly ObservationSelection[]): Promise<readonly ObservationReadResult[]>;
  /** Subscribe to bounded resident observation rows; browser delivery is off-render. */
  subscribeObservations(request: ObservationSubscriptionRequest): Promise<ObservationSubscription>;
  /** Subscribe to one selected track response; browser delivery is off-render. */
  subscribeTrackResponse(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscription>;
  /** Capture and evaluate one immutable selected-track response at the current render boundary. */
  queryTrackResponse(request: TrackResponseQuery): Promise<TrackResponseResult>;
  /** Subscribe to one prepared continuous spectrum boundary; browser delivery is off-render. */
  subscribeSpectrum(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscription>;
  /** Arm, capture and analyze one complete 2048-frame spectrum window. */
  querySpectrum(request: SpectrumQuery): Promise<SpectrumResult>;
  /** Bind the semantic console once; rejects with MisoUsageError when no console was attached. */
  console(): Promise<EngineConsole>;
  /** Dispose the worklet host, then close its context. Safe to call more than once. */
  close(): Promise<void>;
}

function documentBytes(document: CreateEngineOptions["document"]): Uint8Array<ArrayBuffer> {
  if (typeof document === "string") return new TextEncoder().encode(document);
  if (document instanceof Uint8Array) {
    return new Uint8Array(document);
  }
  return new TextEncoder().encode(document.toJson());
}

function trackResponseHostRequest(request: TrackResponseQuery): TrackResponseHostRequest {
  const maximumIdBytes = ABI_LAYOUT.constants.maximumLiveResponseIdBytes;
  const maximumPoints = ABI_LAYOUT.constants.maximumLiveResponsePoints;
  const maximumCaptureBytes = ABI_LAYOUT.constants.liveResponseCaptureBytes;
  if (typeof request.trackId !== "string" || request.trackId.length === 0
      || new TextEncoder().encode(request.trackId).byteLength > maximumIdBytes) {
    throw new MisoUsageError("trackId must be a nonempty string within the live response identity bound");
  }
  if (request.grid.kind !== "linear" && request.grid.kind !== "logarithmic") {
    throw new MisoUsageError("grid.kind must be linear or logarithmic");
  }
  if (!Number.isSafeInteger(request.grid.points) || request.grid.points < 2 || request.grid.points > maximumPoints) {
    throw new MisoUsageError(`grid.points must be an integer in 2..=${maximumPoints}`);
  }
  if (!Number.isFinite(request.grid.minimumHz) || !Number.isFinite(request.grid.maximumHz)) {
    throw new MisoUsageError("grid endpoints must be finite");
  }
  if (request.grid.kind === "logarithmic" && request.grid.minimumHz <= 0) {
    throw new MisoUsageError("logarithmic grid minimumHz must be positive");
  }
  const channels = request.channels === undefined || request.channels === "both"
    ? 3 : request.channels === "left" ? 1 : request.channels === "right" ? 2 : undefined;
  if (channels === undefined) throw new MisoUsageError("channels must be left, right, or both");
  const maximumResultBytes = request.responseLimits?.maximumResultBytes ?? maximumCaptureBytes;
  if (!Number.isSafeInteger(maximumResultBytes) || maximumResultBytes < 1 || maximumResultBytes > maximumCaptureBytes) {
    throw new MisoUsageError(`maximumResultBytes must be an integer in 1..=${maximumCaptureBytes}`);
  }
  const requestDeadlineMs = request.responseLimits?.requestDeadlineMs;
  if (requestDeadlineMs !== undefined
      && (!Number.isFinite(requestDeadlineMs) || requestDeadlineMs <= 0 || requestDeadlineMs > 2_147_483_647)) {
    throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
  }
  return {
    trackId: request.trackId,
    grid: request.grid.kind === "linear" ? 1 : 2,
    channels,
    points: request.grid.points,
    minimumHz: request.grid.minimumHz,
    maximumHz: request.grid.maximumHz,
    maximumResultBytes,
  };
}

function spectrumTargetId(target: SpectrumQuery["target"]): string {
  if (target === null || typeof target !== "object") {
    throw new MisoUsageError("spectrum target must be an object");
  }
  let id: unknown;
  switch (target.kind) {
    case "trackPostInputBuiltins":
    case "trackPostMatrix":
      id = target.trackId;
      break;
    case "output":
      id = target.outputId;
      break;
    default:
      throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
  if (typeof id !== "string" || id.length === 0) {
    throw new MisoUsageError("spectrum target identity must be a nonempty string");
  }
  return id;
}

function sameSpectrumTarget(left: SpectrumQuery["target"], right: SpectrumQuery["target"]): boolean {
  if (left === null || typeof left !== "object" || right === null || typeof right !== "object") {
    throw new MisoUsageError("spectrum target must be an object");
  }
  const leftKind = left.kind;
  const rightKind = right.kind;
  if (leftKind !== "trackPostInputBuiltins" && leftKind !== "trackPostMatrix" && leftKind !== "output") {
    throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
  if (rightKind !== "trackPostInputBuiltins" && rightKind !== "trackPostMatrix" && rightKind !== "output") {
    throw new MisoUsageError("target.kind must name a supported spectrum boundary");
  }
  if (leftKind !== rightKind) return false;
  return spectrumTargetId(left) === spectrumTargetId(right);
}

function spectrumChannelMask(channels: SpectrumQuery["channels"]): number {
  if (channels === undefined || channels === "both") return 3;
  if (channels === "left") return 1;
  if (channels === "right") return 2;
  throw new MisoUsageError("spectrum channels must be left, right, or both");
}

function spectrumHostSelection(query: SpectrumQuery): {
  readonly target: "trackPostInputBuiltins" | "trackPostMatrix" | "output";
  readonly targetId: string;
  readonly channels: "left" | "right" | "both";
} {
  const targetId = query.target.kind === "output" ? query.target.outputId : query.target.trackId;
  return Object.freeze({
    target: query.target.kind,
    targetId,
    channels: query.channels ?? "both",
  });
}

function validateSpectrumQuery(request: SpectrumQuery, prepared: SpectrumQuery): SpectrumQuery {
  const query = cloneSpectrumQuery(request);
  if (!sameSpectrumTarget(query.target, prepared.target)) {
    throw new MisoUsageError("spectrum query target does not match the prepared boundary");
  }
  const requestedChannels = spectrumChannelMask(query.channels);
  const preparedChannels = spectrumChannelMask(prepared.channels);
  if ((requestedChannels & ~preparedChannels) !== 0) {
    throw new MisoUsageError("spectrum query channels were not prepared");
  }
  const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes
    ?? prepared.spectrumLimits?.maximumCaptureBytes
    ?? ABI_LAYOUT.constants.spectrumCaptureBytes;
  if (!Number.isSafeInteger(maximumCaptureBytes) || maximumCaptureBytes < 1
      || maximumCaptureBytes > (prepared.spectrumLimits?.maximumCaptureBytes
        ?? ABI_LAYOUT.constants.spectrumCaptureBytes)) {
    throw new MisoUsageError("spectrum maximumCaptureBytes exceeds the prepared bound");
  }
  const deadline = query.spectrumLimits?.requestDeadlineMs;
  if (deadline !== undefined
      && (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647)) {
    throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
  }
  return query;
}

function validateSpectrumCollectionQuery(
  request: SpectrumQuery,
  collection: SpectrumCollection,
): SpectrumQuery {
  const query = cloneSpectrumQuery(request);
  const channels = query.channels ?? "both";
  if (!collection.entries.some((entry) =>
    sameSpectrumTarget(entry.target, query.target) && (entry.channels ?? "both") === channels)) {
    throw new MisoUsageError("spectrum query target and channels were not prepared");
  }
  const maximumCaptureBytes = query.spectrumLimits?.maximumCaptureBytes;
  if (maximumCaptureBytes !== undefined
      && maximumCaptureBytes > ABI_LAYOUT.constants.spectrumCaptureBytes) {
    throw new MisoUsageError("spectrum maximumCaptureBytes exceeds the browser capture bound");
  }
  return query;
}

async function awaitSpectrum<T>(
  promise: Promise<T>,
  started: number,
  deadline: number,
  signal: AbortSignal | undefined,
): Promise<T> {
  // A request may still reject after the total deadline has elapsed before this wrapper is called.
  // Observe it even when there is no time left, so deadline refusal never creates an unhandled
  // host/Worker rejection.
  void promise.catch(() => undefined);
  const remaining = deadline - (Date.now() - started);
  if (remaining <= 0) throw new MisoUsageError("spectrum query exceeded its deadline");
  signal?.throwIfAborted();
  let timer: ReturnType<typeof setTimeout> | undefined;
  let onAbort: (() => void) | undefined;
  try {
    return await new Promise<T>((resolve, reject) => {
      timer = setTimeout(() => reject(new MisoUsageError("spectrum query exceeded its deadline")), remaining);
      onAbort = () => reject(signal?.reason ?? new MisoUsageError("spectrum query was aborted"));
      signal?.addEventListener("abort", onAbort, { once: true });
      promise.then(resolve, reject);
    });
  } finally {
    if (timer !== undefined) clearTimeout(timer);
    if (onAbort !== undefined) signal?.removeEventListener("abort", onAbort);
  }
}

function spectrumStreamMetadata(
  raw: MisoSpectrumStreamMetadata,
  prepared: SpectrumQuery,
): SpectrumStreamMetadata {
  const target = prepared.target;
  const targetRaw = ABI_LAYOUT.constants.spectrumTargets.find((row) => row.name === target.kind)?.value;
  if (targetRaw === undefined) {
    throw new MisoEngineError("the prepared spectrum target is absent from the generated ABI", {
      phase: "asset", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
    });
  }
  if (raw.target !== targetRaw) {
    throw new MisoEngineError("the browser host returned a different spectrum target", {
      phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
    });
  }
  const channels = prepared.channels ?? "both";
  const channelsRaw = ABI_LAYOUT.constants.spectrumChannels.find((row) => row.name === channels)?.value;
  if (channelsRaw === undefined) {
    throw new MisoEngineError("the prepared spectrum channels are absent from the generated ABI", {
      phase: "asset", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
    });
  }
  if (raw.channels !== channelsRaw) {
    throw new MisoEngineError("the browser host returned different spectrum channels", {
      phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
    });
  }
  const status = ABI_LAYOUT.constants.spectrumStreamStatuses.find((row) => row.value === raw.status);
  if (status === undefined) {
    throw new MisoEngineError("the browser host returned an unknown spectrum stream status", {
      phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
    });
  }
  return cloneSpectrumStreamMetadata(Object.freeze({
    result: raw.result,
    status: status.name as SpectrumStreamStatus,
    target,
    channels,
    sampleRateHz: raw.sampleRateHz,
    quantumFrames: raw.quantumFrames,
    hopFrames: raw.hopFrames,
    sourceUnderrun: raw.sourceUnderrun !== 0,
    captureEpoch: raw.captureEpoch,
    sequence: raw.sequence,
    droppedCaptures: raw.droppedCaptures,
    windows: raw.windows,
    capturedSample: raw.capturedSample,
    endSample: raw.endSample,
    analysisEpoch: raw.analysisEpoch,
    historyStartSample: raw.historyStartSample,
    smoothingMs: raw.smoothingMs,
  }));
}

/**
 * Open a browser session.
 *
 * The order is the sealed one, and each step exists because the step after it is expensive to undo:
 *
 * 1. **Refuse what web delivery does not carry.** A `32f` source is refused here, before anything
 *    is constructed, when the caller authored its sources through the SDK.
 * 2. **Scratch boot in a Worker.** Learn the document's declared shape from the engine, not from
 *    its text.
 * 3. **Construct-verify-close-retry.** An `AudioContext` is *asked* for a rate; it is not obliged
 *    to give one. So the rate it actually reports is verified, and a context that came back at the
 *    wrong rate is closed and retried rather than used. `renderSizeHint` is passed
 *    unconditionally -- a browser that ignores it is no worse off, and one that honours it saves
 *    the whole session.
 * 4. **Pre-worklet quantum refusal.** Checked against the context that exists, before `addModule`,
 *    so a mismatch is an answer to the caller rather than a sticky failure on a live graph.
 * 5. **Worklet boot as the backstop.** Its two `require_*` words carry the physical shape, so if
 *    the context changed under everything above, the engine still refuses rather than rendering at
 *    a rate nobody agreed to.
 */
export function createEngine<Context extends AudioContextLike>(
  options: CreateEngineOptions<Context> & { readonly createContext: NonNullable<CreateEngineOptions<Context>["createContext"]> },
): Promise<BrowserEngine<Context>>;
export function createEngine(
  options: CreateEngineOptions & { readonly createContext?: undefined },
): Promise<BrowserEngine<DefaultAudioContext>>;
export function createEngine(options: CreateEngineOptions): Promise<BrowserEngine<AudioContextLike>>;
export async function createEngine(options: CreateEngineOptions): Promise<BrowserEngine<AudioContextLike>> {
  const spectrumHopFrames = validateBrowserSpectrumHopFrames(options.policy?.spectrumHopFrames);
  const preparedSpectrum = options.spectrum === undefined ? undefined : cloneSpectrumQuery(options.spectrum);
  const preparedSpectrumCollection = options.spectrumCollection === undefined
    ? undefined : cloneSpectrumCollection(options.spectrumCollection);
  if (preparedSpectrum !== undefined && preparedSpectrumCollection !== undefined) {
    throw new MisoUsageError("spectrum and spectrumCollection are mutually exclusive");
  }
  const document = documentBytes(options.document);
  const policy = {
    ...options.policy,
    ...(spectrumHopFrames === undefined ? {} : { spectrumHopFrames }),
    ...(typeof options.policy?.console === "object" ? { console: { ...options.policy.console } } : {}),
  };
  const preparedModule = options.preparedModule;
  const simd128ModuleUrl = options.simd128ModuleUrl ?? BUNDLED_ENGINE_ASSETS.wasm.href;
  const workletModuleUrl = options.workletModuleUrl ?? BUNDLED_ENGINE_ASSETS.workletModule.href;

  // 1. Web delivery scope.
  if (options.sources !== undefined) assertWebDeliverableSources(options.sources);

  // 2. The scratch boot's answer.
  const scratchBoot = options.scratchBoot ?? ((request) => scratchBootWithWorker({
    ...request,
    moduleUrl: simd128ModuleUrl,
    ...(options.scratchWorkerModuleUrl === undefined ? {} : { scratchWorkerModuleUrl: options.scratchWorkerModuleUrl }),
    ...(options.createWorker === undefined ? {} : { createWorker: options.createWorker }),
    ...(options.requestDeadlineMs === undefined ? {} : { requestDeadlineMs: options.requestDeadlineMs }),
    ...(options.signal === undefined ? {} : { signal: options.signal }),
  }));
  const shape = await scratchBoot({
    document,
    options: scratchBootOptions(policy),
  });

  // 3. Construct-verify-close-retry.
  const attempts = options.contextAttempts ?? 2;
  if (!Number.isInteger(attempts) || attempts < 1) {
    throw new MisoUsageError("contextAttempts must be a positive integer");
  }
  const createContext = options.createContext ?? defaultCreateContext;
  let context: AudioContextLike | undefined;
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const candidate = createContext({
      sampleRate: shape.sampleRateHz,
      renderSizeHint: shape.quantumFrames,
    });
    if (candidate.sampleRate === shape.sampleRateHz) {
      context = candidate;
      break;
    }
    await candidate.close();
  }
  if (context === undefined) {
    throw new MisoEngineError(
      `no AudioContext could be constructed at ${shape.sampleRateHz} Hz in ${attempts} attempts`,
      {
        phase: "boot",
        code: "reprepareRequired",
        result: constantValue("resultCodes", "reprepareRequired"),
        diagnostics: [{ code: "host.session.shape", path: "$.sample_rate_hz" }],
      },
    );
  }

  // 4. The pre-worklet quantum refusal.
  try {
    assertQuantumMatch(context.renderQuantumSize, shape.quantumFrames);
  } catch (error) {
    await context.close();
    throw error;
  }

  // 5. The worklet boot, with the physical shape required.
  try {
    const createHost = options.createHost ?? ((request) => createDefaultHost({
      ...request,
      ...(options.hostModuleUrl === undefined ? {} : { hostModuleUrl: options.hostModuleUrl }),
    }));
    const host = await createHost({
      context,
      document,
      options: workletBootOptions(policy, {
        sampleRateHz: shape.sampleRateHz,
        quantumFrames: shape.quantumFrames,
      }, preparedSpectrum, preparedSpectrumCollection),
      simd128ModuleUrl,
      workletModuleUrl,
      ...(preparedModule === undefined ? {} : { preparedModule }),
    });
    let semanticConsole: Promise<EngineConsole> | undefined;
    let closePromise: Promise<void> | undefined;
    let trackResponse: BrowserTrackResponse | undefined;
    let trackResponsePromise: Promise<BrowserTrackResponse> | undefined;
    let trackResponsePending = false;
    let spectrum: BrowserSpectrum | undefined;
    let spectrumPromise: Promise<BrowserSpectrum> | undefined;
    let spectrumPending = false;
    let spectrumCleanup: Promise<void> | undefined;
    let spectrumStreamActive = false;
    let spectrumStreamPending = false;
    let spectrumStreamBuffer: ArrayBuffer | undefined;
    let spectrumActiveQuery: SpectrumQuery | undefined = preparedSpectrum;
    let closed = false;
    const measurementFeeds = createMeasurementFeeds(
      host,
      shape.tracks,
      policy.console !== undefined,
    );
    let observationSubscriptions: ObservationSubscriptionOwner | undefined;
    const observationMap = async (): Promise<ObservationMap> => {
      const reply = await host.observationMap();
      if (reply.result !== constantValue("resultCodes", "ok")) {
        throw new MisoEngineError("the browser host refused the observation map", {
          phase: "output",
          code: resultName(reply.result, "call"),
          result: reply.result,
        });
      }
      return enrichObservationMap(shape.tracks, reply.bindings as readonly RawObservationBinding[]);
    };
    const readObservations = async (
      selections: readonly ObservationSelection[],
    ): Promise<readonly ObservationReadResult[]> => {
      validateObservationSelections(selections);
      const map = await observationMap();
      const addresses = resolveObservationAddressesWithTracks(map, shape.tracks, selections);
      const reply = await host.readObservations({ selections: [...addresses] });
      if (reply.result !== constantValue("resultCodes", "ok")) {
        throw new MisoEngineError("the browser host refused the selected observation read", {
          phase: reply.result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
          code: resultName(reply.result, "call"),
          result: reply.result,
        });
      }
      return decodeObservationRows(
        map,
        shape.tracks,
        selections,
        addresses,
        reply.rows as readonly RawObservationRow[],
        shape.sampleRateHz,
      );
    };
    const getConsole = (): Promise<EngineConsole> => {
      semanticConsole ??= (policy.console?.commandQueueRecords ?? 0) === 0
        ? Promise.reject(new MisoUsageError(
          "this engine booted with no console attached; set policy.console.commandQueueRecords",
        ))
        : createBrowserConsole(host, (edits, managed) =>
          observationSubscriptions?.beforeConsoleSubmit(edits, managed));
      return semanticConsole;
    };
    const ensureSpectrumWorker = (query: SpectrumQuery): Promise<BrowserSpectrum> => {
      if (spectrum !== undefined) return Promise.resolve(spectrum);
      if (spectrumPromise !== undefined) return spectrumPromise;
      let pending: Promise<BrowserSpectrum>;
      pending = createSpectrum({
        ...(preparedModule === undefined ? { moduleUrl: simd128ModuleUrl } : { module: preparedModule }),
        ...(options.responseWorkerModuleUrl === undefined ? {} : { responseWorkerModuleUrl: options.responseWorkerModuleUrl }),
        ...(options.createResponseWorker === undefined ? {} : { createWorker: options.createResponseWorker }),
        ...(options.signal === undefined ? {} : { signal: options.signal }),
        ...(query.spectrumLimits === undefined ? {} : { spectrumLimits: query.spectrumLimits }),
      }).then((client) => {
        if (closed || spectrumPromise !== pending) {
          void client.close().catch(() => undefined);
          throw new MisoUsageError("the spectrum Worker was superseded");
        }
        spectrum = client;
        return client;
      }, (error) => {
        if (spectrumPromise === pending) spectrumPromise = undefined;
        throw error;
      });
      spectrumPromise = pending;
      return spectrumPromise;
    };
    const selectSpectrum = async (query: SpectrumQuery): Promise<EngineCallResult> => {
      const collection = preparedSpectrumCollection;
      if (collection === undefined) {
        return Object.freeze({
          ok: false,
          result: constantValue("resultCodes", "unsupported"),
          code: "unsupported",
        });
      }
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (spectrumStreamActive || spectrumStreamPending) {
        throw new MisoUsageError("a spectrum stream is already active");
      }
      if (spectrumPending || spectrumCleanup !== undefined) {
        throw new MisoUsageError("a one-shot spectrum query is already in flight");
      }
      if (typeof host.selectSpectrum !== "function") {
        throw new MisoUsageError("the browser host does not support spectrum collections");
      }
      const effectiveQuery = validateSpectrumCollectionQuery(query, collection);
      spectrumStreamPending = true;
      try {
        const started = Date.now();
        const deadline = effectiveQuery.spectrumLimits?.requestDeadlineMs ?? options.requestDeadlineMs ?? 5_000;
        // Preflight before the collection operation arms a capture, so Worker initialization
        // refusal cannot leave an unowned selected capture behind.
        await awaitSpectrum(ensureSpectrumWorker(effectiveQuery), started, deadline, options.signal);
        spectrumStreamBuffer ??= new ArrayBuffer(ABI_LAYOUT.constants.spectrumCaptureBytes);
        const selection = host.selectSpectrum(spectrumHostSelection(effectiveQuery));
        let reply: Awaited<typeof selection>;
        try {
          reply = await awaitSpectrum(selection, started, deadline, options.signal);
        } catch (error) {
          // Initial selection arms a one-shot capture before stream activation. Hold its late
          // cancellation so a replacement cannot inherit or race that unowned capture.
          holdSpectrumCleanup(selection.then(
            () => cancelSpectrumCapture(),
            () => cancelSpectrumCapture(),
          ));
          throw error;
        }
        if (closed) throw new MisoUsageError("the browser engine is closed");
        if (reply.result === constantValue("resultCodes", "ok")) spectrumActiveQuery = effectiveQuery;
        return Object.freeze({
          ok: reply.result === constantValue("resultCodes", "ok"),
          result: reply.result,
          code: resultName(reply.result, "call"),
        });
      } finally {
        spectrumStreamPending = false;
      }
    };
    const selectSpectrumStream = async (
      query: SpectrumQuery,
      smoothingMs: number,
    ): Promise<SpectrumStreamSelection> => {
      const collection = preparedSpectrumCollection;
      if (collection === undefined) {
        return Object.freeze({
          ok: false,
          result: constantValue("resultCodes", "unsupported"),
          code: "unsupported",
        });
      }
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (!spectrumStreamActive) throw new MisoUsageError("the spectrum stream is not active");
      if (spectrumStreamPending) throw new MisoUsageError("a spectrum stream request is already in flight");
      if (spectrumPending || spectrumCleanup !== undefined) {
        throw new MisoUsageError("a one-shot spectrum query is already in flight");
      }
      if (typeof host.selectSpectrumStream !== "function") {
        throw new MisoUsageError("the browser host does not support spectrum collection updates");
      }
      const effectiveQuery = validateSpectrumCollectionQuery(query, collection);
      if (!Number.isFinite(smoothingMs) || smoothingMs < 0 || smoothingMs > 10_000) {
        throw new MisoUsageError("spectrum smoothing must be finite and between 0 and 10000 milliseconds");
      }
      spectrumStreamPending = true;
      try {
        const selection = host.selectSpectrumStream({
          ...spectrumHostSelection(effectiveQuery),
          smoothingMs,
        });
        let reply: Awaited<typeof selection>;
        try {
          reply = await awaitSpectrum(selection, Date.now(),
            effectiveQuery.spectrumLimits?.requestDeadlineMs ?? options.requestDeadlineMs ?? 5_000,
            options.signal);
        } catch (error) {
          // A late transaction may have committed after the caller's deadline. Retire this
          // spectrum lifetime and hold its cleanup before permitting another selection.
          spectrumStreamActive = false;
          observationSubscriptions?.invalidateSpectrum();
          holdSpectrumCleanup(selection.then(
            () => host.stopSpectrumStream?.(),
            () => host.stopSpectrumStream?.(),
          ));
          throw error;
        }
        if (closed) throw new MisoUsageError("the browser engine is closed");
        if (reply.metadata === undefined) {
          throw new MisoEngineError("the browser host returned no spectrum selection metadata", {
            phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
          });
        }
        const metadata = spectrumStreamMetadata(reply.metadata, effectiveQuery);
        if (reply.result === constantValue("resultCodes", "ok")) spectrumActiveQuery = effectiveQuery;
        return Object.freeze({
          ok: reply.result === constantValue("resultCodes", "ok"),
          result: reply.result,
          code: resultName(reply.result, "call"),
          metadata,
        });
      } finally {
        spectrumStreamPending = false;
      }
    };
    const startSpectrumStream = async (smoothingMs: number, query?: SpectrumQuery): Promise<SpectrumStreamStart> => {
      const prepared = preparedSpectrum;
      const collection = preparedSpectrumCollection;
      if (prepared === undefined && collection === undefined) {
        throw new MisoUsageError("this engine has no prepared spectrum boundary");
      }
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (spectrumStreamActive || spectrumStreamPending) {
        throw new MisoUsageError("a spectrum stream is already active");
      }
      if (spectrumPending || spectrumCleanup !== undefined) {
        throw new MisoUsageError("a one-shot spectrum query is already in flight");
      }
      if (typeof host.startSpectrumStream !== "function") {
        throw new MisoUsageError("the browser host does not support managed spectrum streams");
      }
      const effectiveQuery = query === undefined
        ? prepared
        : collection === undefined
          ? validateSpectrumQuery(query, prepared!)
          : validateSpectrumCollectionQuery(query, collection);
      if (effectiveQuery === undefined) {
        throw new MisoUsageError("a spectrum collection stream needs a selected query");
      }
      spectrumStreamPending = true;
      const deadline = effectiveQuery.spectrumLimits?.requestDeadlineMs
        ?? options.requestDeadlineMs
        ?? 5_000;
      const started = Date.now();
      let startRequest: Promise<Awaited<ReturnType<NonNullable<MisoAudioWorkletHost["startSpectrumStream"]>>>> | undefined;
      try {
        await awaitSpectrum(ensureSpectrumWorker(effectiveQuery), started, deadline, options.signal);
        spectrumStreamBuffer ??= new ArrayBuffer(ABI_LAYOUT.constants.spectrumCaptureBytes);
        startRequest = host.startSpectrumStream(smoothingMs);
        const reply = await awaitSpectrum(startRequest, started, deadline, options.signal);
        const metadata = spectrumStreamMetadata(reply.metadata, effectiveQuery);
        if (closed) {
          if (reply.result === constantValue("resultCodes", "ok")) {
              void Promise.resolve(host.stopSpectrumStream?.()).catch(() => undefined);
          }
          throw new MisoUsageError("the browser engine is closed");
        }
        if (reply.result === constantValue("resultCodes", "ok")) {
          spectrumStreamActive = true;
          spectrumActiveQuery = effectiveQuery;
        }
        return Object.freeze({
          ok: reply.result === constantValue("resultCodes", "ok"),
          result: reply.result,
          code: resultName(reply.result, "call"),
          metadata,
        });
      } catch (error) {
        // A timed-out MessagePort request can still complete after this call returns. If it arms
        // the native stream late, stop it at completion rather than leaving an unowned stream.
        if (startRequest !== undefined) {
          void startRequest.then((reply) => {
            if (reply.result === constantValue("resultCodes", "ok")) {
          return host.stopSpectrumStream?.();
            }
            return undefined;
          }, () => undefined).catch(() => undefined);
        }
        throw error;
      } finally {
        spectrumStreamPending = false;
      }
    };
    const readSpectrumStream = async (query?: SpectrumQuery): Promise<SpectrumStreamRead> => {
      const prepared = preparedSpectrum;
      const collection = preparedSpectrumCollection;
      if (prepared === undefined && collection === undefined) {
        throw new MisoUsageError("this engine has no prepared spectrum boundary");
      }
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (!spectrumStreamActive) throw new MisoUsageError("the spectrum stream is not active");
      if (spectrumStreamPending) throw new MisoUsageError("a spectrum stream read is already in flight");
      if (typeof host.readSpectrumStream !== "function") {
        throw new MisoUsageError("the browser host does not support managed spectrum streams");
      }
      const initialBuffer = spectrumStreamBuffer
        ?? new ArrayBuffer(ABI_LAYOUT.constants.spectrumCaptureBytes);
      const expectedBufferBytes = initialBuffer.byteLength;
      const requestedQuery = query ?? spectrumActiveQuery ?? prepared;
      if (requestedQuery === undefined) {
        throw new MisoUsageError("a spectrum collection stream has no selected query");
      }
      const effectiveQuery = collection === undefined
        ? validateSpectrumQuery(requestedQuery, prepared!)
        : validateSpectrumCollectionQuery(requestedQuery, collection);
      const deadline = effectiveQuery.spectrumLimits?.requestDeadlineMs
        ?? options.requestDeadlineMs
        ?? 5_000;
      const started = Date.now();
      spectrumStreamBuffer = undefined;
      spectrumStreamPending = true;
      let returned = false;
      let readRequest: Promise<Awaited<ReturnType<NonNullable<MisoAudioWorkletHost["readSpectrumStream"]>>>> | undefined;
      try {
        readRequest = host.readSpectrumStream(initialBuffer);
        const reply = await awaitSpectrum(readRequest, started, deadline, options.signal);
        if (!(reply.buffer instanceof ArrayBuffer) || reply.buffer.byteLength !== expectedBufferBytes) {
          throw new MisoEngineError("the browser host returned an invalid spectrum stream buffer", {
            phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
          });
        }
        spectrumStreamBuffer = reply.buffer;
        returned = true;
        const metadata = spectrumStreamMetadata(reply.metadata, effectiveQuery);
        const backpressure = constantValue("resultCodes", "backpressure");
        const renderRejected = constantValue("resultCodes", "renderRejected");
        const ok = constantValue("resultCodes", "ok");
        if (reply.result === backpressure || reply.result === renderRejected) {
          return Object.freeze({ metadata });
        }
        if (reply.result !== ok) {
          throw new MisoEngineError("the browser host refused the spectrum stream read", {
            phase: reply.result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
            code: resultName(reply.result, "call"),
            result: reply.result,
          });
        }
        if (metadata.status !== "ready" || reply.byteLength === 0) {
          return Object.freeze({ metadata });
        }
        if (!Number.isSafeInteger(reply.byteLength) || reply.byteLength <= 0
            || reply.byteLength > reply.buffer.byteLength) {
          throw new MisoEngineError("the browser host returned an invalid spectrum stream length", {
            phase: "output", code: "abiMismatch", result: constantValue("resultCodes", "abiMismatch"),
          });
        }
        const buffer = spectrumStreamBuffer;
        spectrumStreamBuffer = undefined;
        returned = false;
        let client: BrowserSpectrum | undefined;
        let analyzed: Awaited<ReturnType<BrowserSpectrum["queryStream"]>>;
        try {
          client = await ensureSpectrumWorker(effectiveQuery);
          analyzed = await awaitSpectrum(
            client.queryStream(effectiveQuery, buffer!, reply.byteLength, metadata),
            started,
            deadline,
            options.signal,
          );
        } catch (error) {
          if (client !== undefined && spectrum === client) spectrum = undefined;
          spectrumPromise = undefined;
          if (client !== undefined) {
            try { await client.close(); } catch { /* the failed Worker is already terminal */ }
          }
          if (buffer !== undefined && buffer.byteLength !== 0) {
            spectrumStreamBuffer = buffer;
            returned = true;
          }
          spectrumStreamActive = false;
          // The read operation still owns the stream-pending flag, so the shared owner's stop
          // callback cannot issue its normal serialized stop here. Close the native stream at the
          // transport seam before invalidating the managed lifetime.
          spectrumStreamPending = false;
          await Promise.resolve(host.stopSpectrumStream?.()).catch(() => undefined);
          if (!closed) observationSubscriptions?.invalidateSpectrum();
          throw error;
        }
        spectrumStreamBuffer = analyzed.buffer;
        returned = true;
        return Object.freeze({ metadata: analyzed.metadata, result: analyzed.result });
      } catch (error) {
        const recovered = (error as Error & { spectrumBuffer?: unknown }).spectrumBuffer;
        if (recovered instanceof ArrayBuffer) {
          spectrumStreamBuffer = recovered;
          returned = true;
        } else if (!returned && initialBuffer.byteLength !== 0) {
          spectrumStreamBuffer = initialBuffer;
          returned = true;
        }
        if (!returned && readRequest !== undefined && !closed) {
          spectrumStreamActive = false;
          observationSubscriptions?.invalidateSpectrum();
          void readRequest.then(() => Promise.resolve(host.stopSpectrumStream?.()), () => undefined).catch(() => undefined);
        }
        throw error;
      } finally {
        spectrumStreamPending = false;
      }
    };
    const stopSpectrumStream = async (): Promise<EngineCallResult> => {
      if (typeof host.stopSpectrumStream !== "function") {
        throw new MisoUsageError("the browser host does not support managed spectrum streams");
      }
      if (!spectrumStreamActive && !spectrumStreamPending) {
        return Object.freeze({ ok: true, result: constantValue("resultCodes", "ok"), code: "ok" });
      }
      if (spectrumStreamPending) throw new MisoUsageError("a spectrum stream request is already in flight");
      spectrumStreamPending = true;
      try {
        const reply = await host.stopSpectrumStream();
        if (reply.result === constantValue("resultCodes", "ok")) spectrumStreamActive = false;
        return Object.freeze({
          ok: reply.result === constantValue("resultCodes", "ok"),
          result: reply.result,
          code: resultName(reply.result, "call"),
        });
      } finally {
        spectrumStreamPending = false;
      }
    };
    const observationOwner = (): ObservationSubscriptionOwner => observationSubscriptions ??=
      new ObservationSubscriptionOwner({
        observationMap,
        readObservations,
        console: getConsole,
        responseRead: readTrackResponse,
        ...(preparedSpectrum === undefined ? {} : {
          spectrumPrepared: () => preparedSpectrum,
        }),
        ...(preparedSpectrumCollection === undefined ? {} : {
          spectrumPreparedCollection: () => preparedSpectrumCollection,
          spectrumSelect: selectSpectrum,
          spectrumStreamSelect: selectSpectrumStream,
        }),
        ...((preparedSpectrum === undefined && preparedSpectrumCollection === undefined) ? {} : {
          spectrumStart: startSpectrumStream,
          spectrumRead: readSpectrumStream,
          spectrumStop: stopSpectrumStream,
        }),
        scheduler: {
          setInterval: (callback, milliseconds) => globalThis.setInterval(callback, milliseconds),
          clearInterval: (handle) => globalThis.clearInterval(handle as ReturnType<typeof setInterval>),
        },
      }, options.observationSubscriptionLimits, options.responseSubscriptionLimits, options.spectrumSubscriptionLimits);
    const cancelSpectrumCapture = (): Promise<void> => {
      try {
        return Promise.resolve(host.cancelSpectrum?.()).then(() => undefined, () => undefined);
      } catch {
        return Promise.resolve();
      }
    };
    const holdSpectrumCleanup = (cleanup: Promise<unknown>): void => {
      const safe = Promise.resolve(cleanup).then(() => undefined, () => undefined);
      let held: Promise<void>;
      held = safe.finally(() => {
        if (spectrumCleanup === held) spectrumCleanup = undefined;
      });
      spectrumCleanup = held;
    };
    const readTrackResponse = async (
      request: TrackResponseQuery,
      previousState?: TrackResponseObservedState,
    ): Promise<TrackResponseRead> => {
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (trackResponsePending) throw new MisoUsageError("a live track response query is already in flight");
      trackResponsePending = true;
      try {
        const hostRequest = trackResponseHostRequest(request);
        if (typeof host.captureTrackResponse !== "function") {
          throw new MisoUsageError("the browser host does not support live track responses");
        }
        const capture = await host.captureTrackResponse(hostRequest);
        if (closed) throw new MisoUsageError("the browser engine is closed");
        if (capture.result !== constantValue("resultCodes", "ok")) {
          throw new MisoEngineError("the browser host refused the live track response capture", {
            phase: capture.result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
            code: resultName(capture.result, "call"),
            result: capture.result,
          });
        }
        let state: TrackResponseObservedState;
        try {
          state = parseTrackResponseState(capture.snapshot);
        } catch (error) {
          // One-shot queries retain the Worker as the authoritative payload validator. Managed
          // reads must validate before dispatch because their unchanged path never reaches it.
          if (previousState !== undefined) throw error;
          state = Object.freeze({ words: Object.freeze([]) });
        }
        const status = async (): Promise<void> => {
          const current = await host.status();
          if (current.result !== constantValue("resultCodes", "ok")
              || current.state !== constantValue("states", "ready")) {
            const lifecycleResult = current.result !== constantValue("resultCodes", "ok")
              ? current.result
              : constantValue("resultCodes", "wrongState");
            throw new MisoEngineError("the browser host no longer owns a ready live response boundary", {
              phase: "lifecycle",
              code: resultName(lifecycleResult, "call"),
              result: lifecycleResult,
            });
          }
        };
        if (previousState !== undefined && sameTrackResponseState(previousState, state)) {
          await status();
          if (closed) throw new MisoUsageError("the browser engine is closed");
          return Object.freeze({ changed: false, state });
        }
        trackResponsePromise ??= createTrackResponse({
          ...(preparedModule === undefined ? { moduleUrl: simd128ModuleUrl } : { module: preparedModule }),
          ...(options.responseWorkerModuleUrl === undefined ? {} : { responseWorkerModuleUrl: options.responseWorkerModuleUrl }),
          ...(options.createResponseWorker === undefined ? {} : { createWorker: options.createResponseWorker }),
          ...(options.signal === undefined ? {} : { signal: options.signal }),
        }).then((client) => {
          trackResponse = client;
          return client;
        });
        const client = trackResponse ?? await trackResponsePromise;
        if (closed) throw new MisoUsageError("the browser engine is closed");
        const result = await client.query(request, capture);
        if (closed) {
          await client.close();
          throw new MisoUsageError("the browser engine is closed");
        }
        await status();
        if (closed) {
          await client.close();
          throw new MisoUsageError("the browser engine is closed");
        }
        return Object.freeze({ changed: true, state, result });
      } finally {
        trackResponsePending = false;
      }
    };
    const queryTrackResponse = async (request: TrackResponseQuery): Promise<TrackResponseResult> => {
      const read = await readTrackResponse(request);
      if (read.result === undefined) throw new MisoEngineError("the live response query returned no result", {
        phase: "output", code: "abiMismatch", result: 2,
      });
      return read.result;
    };
    const querySpectrum = async (request: SpectrumQuery): Promise<SpectrumResult> => {
      if (closed) throw new MisoUsageError("the browser engine is closed");
      const prepared = preparedSpectrum;
      if (prepared === undefined) throw new MisoUsageError("this engine has no prepared spectrum boundary");
      if (spectrumPending || spectrumCleanup !== undefined || spectrumStreamActive || spectrumStreamPending) {
        throw new MisoUsageError("a spectrum query is already in flight");
      }
      spectrumPending = true;
      let armed = false;
      let armSettled = true;
      let armNeedsCancel = false;
      let armCompletion: Promise<void> | undefined;
      let readSettled = true;
      let readCompletion: Promise<void> | undefined;
      try {
        const query = validateSpectrumQuery(request, prepared);
        const deadline = query.spectrumLimits?.requestDeadlineMs
          ?? options.requestDeadlineMs
          ?? 5_000;
        if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
          throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
        }
        const started = Date.now();
        if (typeof host.armSpectrum !== "function" || typeof host.readSpectrum !== "function") {
          throw new MisoUsageError("the browser host does not support spectrum capture");
        }
        armSettled = false;
        let armRequest: Promise<Awaited<ReturnType<MisoAudioWorkletHost["armSpectrum"]>>>;
        try {
          armRequest = host.armSpectrum();
        } catch (error) {
          armSettled = true;
          throw error;
        }
        armCompletion = armRequest.then(
          (reply) => {
            armSettled = true;
            armNeedsCancel = reply.result === constantValue("resultCodes", "ok");
          },
          () => {
            armSettled = true;
            // A rejected transport request may have reached the Worklet before the reply failed.
            armNeedsCancel = true;
          },
        );
        const arm = await awaitSpectrum(armRequest, started, deadline, options.signal);
        armSettled = true;
        armNeedsCancel = arm.result === constantValue("resultCodes", "ok");
        if (closed) throw new MisoUsageError("the browser engine is closed");
        if (arm.result !== constantValue("resultCodes", "ok")) {
          throw new MisoEngineError("the browser host refused the spectrum capture arm", {
            phase: arm.result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
            code: resultName(arm.result, "call"),
            result: arm.result,
          });
        }
        armed = true;
        let capture: Awaited<ReturnType<MisoAudioWorkletHost["readSpectrum"]>>;
        for (;;) {
          if (closed) throw new MisoUsageError("the browser engine is closed");
          readSettled = false;
          let readRequest: Promise<Awaited<ReturnType<MisoAudioWorkletHost["readSpectrum"]>>>;
          try {
            readRequest = host.readSpectrum({ channels: query.channels ?? "both" });
          } catch (error) {
            readSettled = true;
            throw error;
          }
          readCompletion = readRequest.then(
            () => { readSettled = true; },
            () => { readSettled = true; },
          );
          capture = await awaitSpectrum(
            readRequest,
            started,
            deadline,
            options.signal,
          );
          readSettled = true;
          if (closed) throw new MisoUsageError("the browser engine is closed");
          if (capture.result === constantValue("resultCodes", "ok")) break;
          if (capture.result !== constantValue("resultCodes", "backpressure")) {
            throw new MisoEngineError("the browser host refused the spectrum capture read", {
              phase: capture.result === constantValue("resultCodes", "wrongState") ? "lifecycle" : "output",
              code: resultName(capture.result, "call"),
              result: capture.result,
            });
          }
          if (Date.now() - started >= deadline) {
            throw new MisoUsageError("spectrum query exceeded its deadline");
          }
          await new Promise<void>((resolve) => setTimeout(resolve, 0));
        }
        armed = false;
        if (capture.snapshot.byteLength === 0) {
          throw new MisoEngineError("the browser host returned an empty spectrum capture", {
            phase: "output",
            code: "abiMismatch",
            result: constantValue("resultCodes", "abiMismatch"),
          });
        }
        if (closed) throw new MisoUsageError("the browser engine is closed");
        const client = await awaitSpectrum(ensureSpectrumWorker(query), started, deadline, options.signal);
        if (closed) throw new MisoUsageError("the browser engine is closed");
        const result = await awaitSpectrum(
          client.query(query, capture.snapshot),
          started,
          deadline,
          options.signal,
        );
        if (closed) {
          await client.close();
          throw new MisoUsageError("the browser engine is closed");
        }
        const status = await awaitSpectrum(host.status(), started, deadline, options.signal);
        if (status.result !== constantValue("resultCodes", "ok")
            || status.state !== constantValue("states", "ready")) {
          const lifecycleResult = status.result !== constantValue("resultCodes", "ok")
            ? status.result
            : constantValue("resultCodes", "wrongState");
          throw new MisoEngineError("the browser host no longer owns a ready spectrum boundary", {
            phase: "lifecycle",
            code: resultName(lifecycleResult, "call"),
            result: lifecycleResult,
          });
        }
        return result;
      } finally {
        if (armCompletion !== undefined && !armSettled) {
          holdSpectrumCleanup(armCompletion.then(() => armNeedsCancel ? cancelSpectrumCapture() : undefined));
        } else if (armNeedsCancel && !armed) {
          holdSpectrumCleanup(cancelSpectrumCapture());
        } else if (readCompletion !== undefined && !readSettled) {
          holdSpectrumCleanup(readCompletion.then(() => cancelSpectrumCapture()));
        } else if (armed) {
          holdSpectrumCleanup(cancelSpectrumCapture());
        }
        spectrumPending = false;
      }
    };
    return Object.freeze({
      shape,
      context,
      host,
      subscribeMeters: (listener: MeterListener) => measurementFeeds.meters(listener),
      subscribeTelemetry: (listener: TelemetryListener) => measurementFeeds.telemetry(listener),
      observationMap,
      readObservations,
      subscribeObservations: (request: ObservationSubscriptionRequest) => observationOwner().subscribe(request)
        .then((receipt) => receipt.handle),
      subscribeTrackResponse: (request: TrackResponseSubscriptionRequest) => observationOwner().subscribeTrackResponse(request)
        .then((receipt) => receipt.handle),
      subscribeSpectrum: (request: SpectrumSubscriptionRequest) => observationOwner().subscribeSpectrum(request)
        .then((receipt) => receipt.handle),
      queryTrackResponse,
      querySpectrum,
      console: getConsole,
      close: () => {
        closed = true;
        measurementFeeds.close();
        observationSubscriptions?.invalidate(true);
        closePromise ??= (async () => {
          try {
            if (trackResponsePromise !== undefined) {
              try { await (trackResponse ?? await trackResponsePromise)?.close(); } catch { /* host close remains authoritative */ }
            }
            if (spectrumPromise !== undefined) {
              try { await (spectrum ?? await spectrumPromise)?.close(); } catch { /* host close remains authoritative */ }
            }
            await host.dispose();
          } finally {
            // A failed MessagePort disposal must not leak the much larger AudioContext.
            await context.close();
          }
        })();
        return closePromise;
      },
    });
  } catch (error) {
    await context.close();
    throw error;
  }
}

/**
 * The scratch boot's body, to be run inside a Worker.
 *
 * The packaged Worker calls this primitive. Custom Workers may also import it, call it, and post
 * the result back; context/host ownership remains with the browser entry.
 */
export async function scratchBootInWorker(request: {
  readonly moduleBytes: Uint8Array<ArrayBuffer>;
  readonly document: Uint8Array;
  readonly options: ReturnType<typeof scratchBootOptions>;
  readonly expectedSha256?: string;
}): Promise<SessionShape> {
  const document = new Uint8Array(request.document);
  const options = { ...request.options, ...(typeof request.options.console === "object" ? { console: { ...request.options.console } } : {}) };
  const asset = await MisoEngineAsset.load(request.moduleBytes, request.expectedSha256);
  const boundary = await WasmBoundary.boot(asset, document, options);
  try {
    return boundary.shape();
  } finally {
    // The scratch instance's whole purpose is discharged by the answer. Holding it would keep a
    // second engine's worth of memory alive beside the one that is about to render.
    boundary.dispose();
  }
}

/** Compile and rehearse only disposable DSP state; retain the compiled code for live boot. */
export async function prepareBrowserSessionInWorker(request: Parameters<typeof scratchBootInWorker>[0]): Promise<import("./scratch.ts").PreparedBrowserSession> {
  const document = new Uint8Array(request.document);
  const options = { ...request.options, ...(typeof request.options.console === "object" ? { console: { ...request.options.console } } : {}) };
  const asset = await MisoEngineAsset.load(request.moduleBytes, request.expectedSha256);
  const boundary = await WasmBoundary.boot(asset, document, options);
  try {
    const shape = boundary.shape();
    const quantum = shape.quantumFrames;
    // Only one quantum per channel is retained, regardless of source count or duration.
    const plane = new Float32Array(quantum);
    for (let frame = 0; frame < quantum; frame++) plane[frame] = 0.125 * Math.sin(frame * 0.13) + 0.0625;
    const planesByChannels = new Map<number, readonly Float32Array[]>();
    const meters = boundary.sessionMap().metersAttached;
    if (meters && !boundary.meterLease(true).ok) throw new MisoUsageError("Preparation meter lease refused");
    for (let block = 0; block < 64; block++) {
      const startFrame = BigInt(block * quantum);
      for (const source of shape.sources) {
        if (startFrame >= source.frames) continue;
        const frames = Number(source.frames - startFrame < BigInt(quantum) ? source.frames - startFrame : BigInt(quantum));
        let planes = planesByChannels.get(source.channels);
        if (planes === undefined) {
          planes = Array.from({ length: source.channels }, () => plane);
          planesByChannels.set(source.channels, planes);
        }
        const submitted = boundary.submitSource({ sourceId: source.id, generation: 1n, startFrame,
          planes: frames === quantum ? planes : planes.map(channel => channel.subarray(0, frames)),
          endOfRegion: startFrame + BigInt(frames) === source.frames });
        if (!submitted.ok) throw new MisoUsageError(`Preparation source submission refused: ${submitted.code}`);
      }
      boundary.render(quantum);
      if (meters) boundary.pollMeters();
    }
    return { shape, module: asset.module };
  } finally { boundary.dispose(); }
}

function defaultCreateContext(options: { sampleRate: number; renderSizeHint: number }): AudioContextLike {
  const constructor = (globalThis as { AudioContext?: new (options: {
    sampleRate: number; renderSizeHint: number;
  }) => AudioContextLike }).AudioContext;
  if (typeof constructor !== "function") {
    throw new BrowserBootError("context-unavailable", "AudioContext is unavailable");
  }
  return new constructor(options);
}
