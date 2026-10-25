import { decodeObservationEnded } from "../core/observation-ended.ts";
import { MisoEngineAsset } from "../core/asset.ts";
import { WasmBoundary } from "../core/boundary.ts";
import type { EngineCallResult, SessionShape } from "../core/boundary.ts";
import { MisoEngineError, MisoUsageError, SessionSeekProducerError, resultName } from "../core/errors.ts";
import type { SdkTimeoutCode } from "../core/errors.ts";
import { constantValue } from "../core/abi.ts";
import { ABI_LAYOUT } from "../generated/abi.ts";
import type { SourceSpec } from "../core/types.ts";
import { SessionBuilder } from "../core/session.ts";
import {
  assertQuantumMatch,
  assertWebDeliverableSources,
  scratchBootOptions,
  validateBrowserSpectrumHopFrames,
  workletBootOptions,
} from "./policy.ts";
import type { BrowserBootPolicy } from "./policy.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";
import { createMeasurementFeeds } from "./measurement.ts";
import { linkFeedEngine } from "./pcm-feed.ts";
import type {
  MeterListener,
  TelemetryListener,
} from "./measurement.ts";
import type { EngineLiveControls } from "../core/live-controls.ts";
import { scratchBootWithWorker } from "./scratch.ts";
import type { ScratchWorkerFactory } from "./scratch.ts";
import { createDefaultHost, BrowserBootError } from "./default-host.ts";
import { createBrowserControlListTransport, createBrowserControlQueue, createBrowserLiveControls, createBrowserStripOrders } from "./live-controls.ts";
import { readControlList, readControlState } from "../core/control-list.ts";
import type { ControlList, ControlListFilter, ControlState } from "../core/control-list.ts";
import { ProtocolClient, decodeCapabilitiesReply, decodeProtocolEvent } from "../core/protocol-messages.ts";
import { AppliedLedger, watermarkSnapshot } from "../core/session-edit.ts";
import type { EditOutcome, WatermarkSnapshot } from "../core/session-edit.ts";
import type { SessionTransaction } from "../core/session-transaction.ts";
import { controlCallFailure, encodeRevisionProbe, malformed } from "../core/control-frame.ts";
import type { ControlTransactionQueue } from "../core/control-queue.ts";
import type {
  CommittedDocument,
  CounterName,
  CounterSnapshot,
  DiagnosticsPage,
  DiagnosticsQuery,
  EngineCapabilities,
  ProtocolEvent,
  TelemetryConfiguration,
  TransportRequest,
  TransportSnapshot,
} from "../core/protocol-messages.ts";
import { isResponseReady } from "../core/response-frame.ts";
import { TrackResponseRequester } from "../core/live-response.ts";
import type { TrackResponseQuery, TrackResponseResult } from "../core/live-response.ts";
import type { ResponseWorkerFactory } from "./response-worker.ts";
import {
  cloneSpectrumCollection,
  cloneSpectrumQuery,
  spectrumCallResult,
  spectrumConfigureRequest,
  spectrumConfiguredOf,
  spectrumMetadataOf,
  spectrumRefusalError,
  spectrumResultOfStream,
  spectrumResultOfWindow,
  spectrumSelection,
} from "../core/spectrum.ts";
import type {
  SpectrumCollection,
  SpectrumConfigureOptions,
  SpectrumConfigured,
  SpectrumHopFrames,
  SpectrumQuery,
  SpectrumResult,
  SpectrumStreamSelection,
  SpectrumStreamRead,
  SpectrumStreamStart,
} from "../core/spectrum.ts";
import {
  decodeSpectrumCaptureReply,
  decodeSpectrumConfiguredReply,
  decodeSpectrumStreamReply,
  encodeSpectrumCommand,
} from "../core/spectrum-frame.ts";
import type { SpectrumFrameReply, SpectrumFrameRequest, SpectrumFrameStream } from "../core/spectrum-frame.ts";
import {
  ObservationSubscriptionOwner,
  observationFrameTransport,
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
 * live-control words -- are identical by construction. The two `require_*` words are role-defined:
 * zero in the scratch boot, physical in the worklet boot. See `./policy.ts` for why "identical
 * options struct" had to be restated that way, and for the executable form of the rule.
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

/**
 * A PCM producer `BrowserEngine.seek` repositions after the engine accepted a session seek (issue
 * #1644 D3): it moves every ring it writes to `frame`, clamped to each source's region, under
 * `generation`. `CanonicalPcmPump` from the stem store implements it.
 */
export interface SessionSeekProducer {
  seek(frame: number, generation: bigint): void | Promise<void>;
}

/** The options of `BrowserEngine.seek`. */
export interface BrowserSeekOptions {
  /** The generation to seek under; by default one above the largest this engine has used. */
  readonly generation?: bigint;
  /** The producers to reposition, in order, after the engine accepted the seek. */
  readonly producers?: readonly SessionSeekProducer[];
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
  /**
   * Subscribe to bounded resident observation rows; browser delivery is off-render.
   *
   * The subscription is the core's `Observe` edit and its rows are `OBSERVATION_READ` reads, both
   * through the worklet's `control` message (#1589 D3), so a subscription survives a rebuild. A
   * subscribe, update or close refused with `backpressure` (an earlier `Observe` of the same tap is
   * not yet drained) waits for the context's next render and is sent again. The wait is bounded by
   * `requestDeadlineMs` (5,000 ms by default) and ends in the same typed `backpressure` error, so a
   * context that does not render never holds the caller. Any other refusal is thrown at once, as
   * `MisoEngineError` whose `diagnosticCode` is the core's code. A session that binds no
   * observation tap (booted with no observation taps) refuses with `observation.unavailable`
   * before any edit is sent; a full frame of armed taps is `observationLimit`.
   */
  subscribeObservations(request: ObservationSubscriptionRequest): Promise<ObservationSubscription>;
  /**
   * Subscribe to one selected track response; browser delivery is off-render. Each read is a
   * `queryTrackResponse`, so it settles only when the context renders.
   */
  subscribeTrackResponse(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscription>;
  /**
   * The response of one track, as a C ABI host reads it (#1579 D1, D4): a strip `RESPONSE_GET`
   * through the host's `control` message, answered by the `RESPONSE_READY` of the block that
   * captured it. It settles after the next rendered block, so a suspended context rejects it at
   * `responseLimits.requestDeadlineMs` (default 5 s). Queries run one at a time, in call order. A
   * failed or disposed host rejects every pending and queued query at once with a lifecycle
   * `MisoEngineError`; `close()` rejects them with `MisoUsageError`.
   */
  queryTrackResponse(request: TrackResponseQuery): Promise<TrackResponseResult>;
  /**
   * Subscribe to one configured spectrum tap's continuous stream (`SPECTRUM_STREAM`); browser
   * delivery is off-render.
   */
  subscribeSpectrum(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscription>;
  /**
   * Capture one complete 2048-frame window of a configured tap, analyzed by the core
   * (`SPECTRUM_CAPTURE` arm and read; with a collection, a select first). A core refusal rejects with
   * `MisoEngineError` whose `diagnosticCode` is the core's code (`spectrum.unprepared`,
   * `spectrum.busy`, `spectrum.entry.unknown`, ...).
   */
  querySpectrum(request: SpectrumQuery): Promise<SpectrumResult>;
  /**
   * Set, change or remove the spectrum taps of the running session with one `SPECTRUM_CONFIGURE`
   * (#1596 D5): a query sets one tap, a collection several, and an empty collection with budget 0
   * removes them. A change of taps prepares a replacement plan that the audio thread adopts at a
   * later block; the taps are in effect once a spectrum reply reports
   * `adoptedEpoch === configuredEpoch`. A stream on an entry that a collection keeps runs on; a
   * change between the single form and a collection ends it (#1594 D6). A core refusal
   * rejects with `MisoEngineError` whose `diagnosticCode` is the core's code.
   */
  configureSpectrum(
    configuration: SpectrumQuery | SpectrumCollection,
    options?: SpectrumConfigureOptions,
  ): Promise<SpectrumConfigured>;
  /** Bind the semantic live controls once (every value edit is a `SET_CONTROL` transaction). */
  liveControls(): Promise<EngineLiveControls>;
  /**
   * Every control of the running session, with its descriptor and committed value (decision 17, C4;
   * #1564 D1): `CONTROL_LIST_GET` pages posted to the host until `eof`. The core decides the rows,
   * their order and their values; this call only pages.
   *
   * A page refused `control.list.stale` (the structure changed between two pages) restarts the read
   * from the first page, and after three restarts rejects with `MisoEngineError` `control.list.stale`.
   * `filter.owner` restricts the list to one owner, `filter.pageLimit` sets the rows per request.
   */
  controls(filter?: ControlListFilter): Promise<ControlList>;
  /**
   * The committed values of `list`'s control rows (`CONTROL_STATE_GET`), by handle. A structure that
   * changed since `list` was read rejects with `MisoEngineError` `control.list.stale`: list again.
   */
  controlState(list: ControlList, options?: Pick<ControlListFilter, "pageLimit">): Promise<ControlState>;
  /**
   * The core's capabilities (`CAPABILITIES_GET`, #1571 D3), posted to the host's `control` message.
   * `supportedCommands` differs per host.
   */
  capabilities(): Promise<EngineCapabilities>;
  /**
   * The canonical committed session document and its revision (`SESSION_SNAPSHOT_GET` pages,
   * #1571 D3), in pages the host's 16 KiB frame limit holds. A session that changes between pages
   * starts the read again, at most three times.
   */
  committedDocument(): Promise<CommittedDocument>;
  /** The transport's state and the sample it is effective at (`TRANSPORT_GET`). */
  transport(): Promise<TransportSnapshot>;
  /**
   * Start or stop the transport (`TRANSPORT_SET`) at the committed revision, which a revision probe
   * reads from the core first; resolves to the new state. A revision that went stale before the
   * command arrived (another commit in between) is refused by the core with status
   * `revision_conflict` (code `protocol.failure`) as a `MisoControlRefusal`; it is not retried.
   */
  setTransport(request: TransportRequest): Promise<TransportSnapshot>;
  /**
   * Replace the telemetry configuration (`TELEMETRY_CONFIGURE`) at the committed revision; resolves
   * to the configuration the core applied. The meters, counters and diagnostics it names arrive as
   * `onEvent` events. A stale revision is refused with status `revision_conflict` (code
   * `protocol.failure`), as for `setTransport`; it is not retried.
   */
  configureTelemetry(configuration: TelemetryConfiguration): Promise<TelemetryConfiguration>;
  /** The named counters, or every counter the core serves when `ids` is absent (`COUNTERS_GET`). */
  counters(ids?: readonly CounterName[]): Promise<CounterSnapshot>;
  /** One page of the core's retained diagnostics (`DIAGNOSTICS_GET`); defaults `0n`, 64 and `"info"`. */
  diagnostics(query?: DiagnosticsQuery): Promise<DiagnosticsPage>;
  /**
   * Subscribe to the protocol events, decoded (#1571 D4): `sessionCommitted`, `transportState`,
   * `observationEnded`, `meterBatch`, `counterSnapshot` and `diagnostic`, in engine order. The
   * events are the frames `host.onControlEvent` delivers, decoded with the headless engine's code.
   * The worklet reads its events once per message it handles, and its lossy lane holds one event,
   * so a reliable event (`sessionCommitted`, `transportState`, `observationEnded`) always arrives,
   * while a block's lossy telemetry (`meterBatch`, `counterSnapshot`, `diagnostic`) can be replaced
   * by a later block's before it is read. An event that arrives with no listener at all is dropped
   * and counted in `host.discardedControlEvents`. A listener that throws, or a frame that does not
   * decode, is reported asynchronously and does not stop the other listeners. Returns the
   * unsubscribe function.
   */
  onEvent(listener: (event: ProtocolEvent) => void): () => void;
  /**
   * Seek the timeline and every source to `frame` under one new generation, all or nothing (issue
   * #1644 D3), and resolve with the generation used. The engine declares the discontinuity in the
   * same call. The generation is `options.generation`, else one above the largest this object has
   * used for a seek (the first is `2n`; every source starts at `1n`). Only after the engine
   * accepted the seek are `options.producers` repositioned, in order, to `frame` under that
   * generation; the feed then consumes their ring seeks without seeking the sources again. An app
   * that drives its producers itself must also wait for this call to resolve: a producer that
   * seeks first makes the session seek stale. A refused seek (a stale generation is
   * `invalidArgument`, `source.generation.stale`), or a second seek sent while one is unsettled
   * (`backpressure`), repositions nothing and rejects with a `MisoEngineError` that carries the
   * result. A producer that fails after the engine accepted the seek rejects the call with
   * `SessionSeekProducerError`, which carries the applied generation. A `frame` that is not a
   * non-negative bigint at most `Number.MAX_SAFE_INTEGER`, or a closed engine, throws
   * `MisoUsageError` before anything is sent.
   */
  seek(frame: bigint, options?: BrowserSeekOptions): Promise<{ readonly generation: bigint }>;
  /**
   * Apply one structural transaction (`transaction(base)...build()`) at the committed revision
   * (#1296 D1), and resolve with the revision it committed and the path the core chose: `live` (the
   * running plan ramps to the new values), `model_only` (nothing renders differently) or `rebuild`
   * (a successor plan, adopted at a later block, with every unchanged source and node playing on).
   * It resolves at the commit; `applied(revision)` resolves when the audio has it. A refusal the
   * core reports rejects with `MisoControlRefusal`, a refused call (a new sample rate:
   * `reprepareRequired`) with `MisoEngineError`; neither commits. A stale revision (another commit
   * landed first) is refused `revision_conflict` and is not retried. On a page without cross-origin
   * isolation a `rebuild` runs in the worklet's message handler and blocks the audio thread for
   * the rebuild; `host.status()` counts each one in `blockingRebuilds`.
   */
  apply(transaction: SessionTransaction): Promise<EditOutcome>;
  /**
   * Replace the session with `document` (#1296 D1): the core diffs it against the committed model
   * into one transaction, so it is an edit, not a reboot: sources keep playing and unchanged nodes
   * keep their state. Resolves as `apply` does, or with `unchanged` (the revision that stays) when
   * the document equals the committed model. A `{ toJson() }` session is sent as its canonical
   * text; bytes or a string are sent as given, up to the boot's 1 MiB document bound. After a
   * committed replace, `liveControls()` binds to `document`; a refused replace changes nothing.
   */
  replaceSession(document: Uint8Array | string | { toJson(): string }): Promise<EditOutcome>;
  /**
   * The newest committed session revision this engine has seen: from the replies of its own calls,
   * its live controls' transactions, `sessionCommitted` events it decoded for `onEvent` and the
   * watermark. It is never read from the host on access.
   */
  readonly revision: bigint;
  /**
   * The newest applied-revision watermark the worklet posted (`{ revision: 0n, sample: 0n }`,
   * exact, before the first): which committed revision is audible, since which render sample.
   */
  watermark(): WatermarkSnapshot;
  /**
   * Resolve with the first watermark that covers `revision`: the revision is in effect, with every
   * revision before it, from the snapshot's `sample` (D15-17). It settles from the worklet's
   * watermark posts, never at the commit, so it stays pending while the context renders nothing.
   * A revision above the one the core reports committed (probed when it is above `revision`)
   * rejects with `MisoUsageError`, and so does every pending one at `close()`.
   */
  applied(revision: bigint): Promise<WatermarkSnapshot>;
  /**
   * Dispose the worklet host, then close its context. Safe to call more than once.
   *
   * Each step that can hang waits at most 10 s. A step that does not settle rejects `close()` with
   * a `lifecycle` `MisoEngineError` whose code names it (`hostDisposeTimeout`,
   * `contextCloseTimeout`); the context close is still attempted. A settled
   * close is not run again: every later call returns the same promise.
   */
  close(): Promise<void>;
}

/**
 * Report a listener's error outside the host's dispatch, as an event target does: `reportError`
 * where the global has one, otherwise an uncaught throw from a microtask.
 */
function reportListenerError(error: unknown): void {
  const report = (globalThis as { reportError?: (error: unknown) => void }).reportError;
  queueMicrotask(typeof report === "function"
    ? () => report.call(globalThis, error)
    : () => {
      throw error;
    });
}

/** How often a browser engine reads its host's status while a track response query is pending. */
const TRACK_RESPONSE_HOST_POLL_MS = 20;

function documentBytes(document: CreateEngineOptions["document"]): Uint8Array<ArrayBuffer> {
  if (typeof document === "string") return new TextEncoder().encode(document);
  if (document instanceof Uint8Array) {
    return new Uint8Array(document);
  }
  return new TextEncoder().encode(document.toJson());
}

/** Capability flag bit 16 of `CAPABILITIES_GET` (#1594 D10): the core serves the spectrum frames. */
const CAPABILITY_SPECTRUM = 1n << 16n;

function spectrumCapabilityRefusal(reason: string): MisoEngineError {
  return new MisoEngineError(`the engine cannot serve the spectrum frames: ${reason}`, {
    phase: "asset",
    code: "unsupported",
    result: constantValue("resultCodes", "unsupported"),
    diagnostics: [{ code: "sdk.asset.capability", path: "CAPABILITIES_GET.flags.spectrum" }],
  });
}

function spectrumTargetKey(target: SpectrumQuery["target"]): string {
  return JSON.stringify([target.kind, target.kind === "output" ? target.outputId : target.trackId]);
}

/**
 * Send one spectrum command (#1596 D1) through the host's `control` message, on the host's one
 * request-ID allocator (the transaction queue's), and decode its reply. The port delivers frames in
 * the order they are sent, so a frame sent after another is served after it.
 */
async function sendSpectrumFrame<T>(
  host: MisoAudioWorkletHost,
  queue: ControlTransactionQueue,
  request: SpectrumFrameRequest,
  decode: (bytes: Uint8Array) => SpectrumFrameReply<T>,
): Promise<SpectrumFrameReply<T>> {
  const requestId = queue.requestIds.next();
  const reply = await host.control(encodeSpectrumCommand(requestId, request));
  queue.noteCommit(reply.revision);
  if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
  const decoded = decode(reply.response);
  if (decoded.requestId !== requestId) {
    throw malformed(`the reply answers request ${decoded.requestId}, not ${requestId}`);
  }
  return decoded;
}

/**
 * D2 of #1596 for the browser engine, as for the headless one: refuse an engine whose
 * `CAPABILITIES_GET` lacks flag bit 16, then send one `SPECTRUM_CONFIGURE` of the boot's taps with
 * the policy's hop. The context is suspended until the engine is returned, so no block has
 * rendered: the engine prepares the plan with the taps and adopts it in place of the boot's plan
 * (#1597), and the reply reports `adoptedEpoch === configuredEpoch` at once.
 */
async function configureBootSpectrum(
  host: MisoAudioWorkletHost,
  configuration: SpectrumQuery | SpectrumCollection,
  hopFrames: SpectrumHopFrames | undefined,
  deadlineMs: number,
  signal: AbortSignal | undefined,
): Promise<ControlTransactionQueue> {
  const started = Date.now();
  // The boot's own deadline is the SDK's boot timeout, not a usage error of the caller's.
  const expired = (): Error => new MisoEngineError(
    `the boot spectrum configuration did not complete within ${deadlineMs} ms`,
    { phase: "boot", code: "bootConfigureTimeout" },
  );
  const queue = await awaitSpectrum(createBrowserControlQueue(host), started, deadlineMs, signal, expired);
  const requestId = queue.requestIds.next();
  const probe = await awaitSpectrum(host.control(encodeRevisionProbe(requestId)), started, deadlineMs, signal, expired);
  queue.noteCommit(probe.revision);
  if (probe.result !== 0) throw controlCallFailure(probe.result, probe.response);
  const capabilities = decodeCapabilitiesReply(probe.response);
  if (capabilities.requestId !== requestId) {
    throw malformed(`the reply answers request ${capabilities.requestId}, not ${requestId}`);
  }
  if (!capabilities.ok) throw spectrumCapabilityRefusal(`CAPABILITIES_GET was refused with ${capabilities.status}`);
  if ((capabilities.value.flags & CAPABILITY_SPECTRUM) === 0n) {
    throw spectrumCapabilityRefusal("capability flag bit 16 is clear");
  }
  const { request } = spectrumConfigureRequest(configuration, hopFrames === undefined ? undefined : { hopFrames });
  const reply = await awaitSpectrum(
    sendSpectrumFrame(host, queue, request, decodeSpectrumConfiguredReply),
    started,
    deadlineMs,
    signal,
    expired,
  );
  if (!reply.ok) throw spectrumRefusalError("the engine refused the boot spectrum configuration", reply, "boot");
  if (reply.value.adoptedEpoch !== reply.value.configuredEpoch) {
    throw malformed("the boot spectrum configuration was not carried by the first plan");
  }
  return queue;
}

function spectrumDeadlineMs(value: unknown): number {
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0 || value > 2_147_483_647) {
    throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
  }
  return value;
}
/** The longest `close()` waits for one step that can hang (D1 of issue #1532). */
const CLOSE_STEP_TIMEOUT_MS = 10_000;

/**
 * Await `run()` for at most `CLOSE_STEP_TIMEOUT_MS`. A step that does not settle in time rejects
 * with a `lifecycle` `MisoEngineError` that names `step` and carries `code`. The timer is cleared
 * when the step settles, and a step that settles after its timeout stays observed, so it never
 * becomes an unhandled rejection.
 */
function boundedCloseStep<T>(step: string, code: SdkTimeoutCode, run: () => Promise<T>): Promise<T> {
  return new Promise<T>((resolve, reject) => {
    const timer = setTimeout(() => {
      reject(new MisoEngineError(`close: ${step} did not settle within ${CLOSE_STEP_TIMEOUT_MS} ms`, {
        phase: "lifecycle",
        code,
      }));
    }, CLOSE_STEP_TIMEOUT_MS);
    let pending: Promise<T>;
    try {
      pending = run();
    } catch (error) {
      clearTimeout(timer);
      reject(error);
      return;
    }
    pending.then(
      (value) => { clearTimeout(timer); resolve(value); },
      (error: unknown) => { clearTimeout(timer); reject(error); },
    );
  });
}

function boundedContextClose(context: AudioContextLike): Promise<void> {
  return boundedCloseStep("AudioContext close", "contextCloseTimeout", async () => { await context.close(); });
}

async function awaitSpectrum<T>(
  promise: Promise<T>,
  started: number,
  deadline: number,
  signal: AbortSignal | undefined,
  expired: () => Error = () => new MisoUsageError("spectrum query exceeded its deadline"),
): Promise<T> {
  // A request may still reject after the total deadline has elapsed before this wrapper is called.
  // Observe it even when there is no time left, so deadline refusal never creates an unhandled
  // host/Worker rejection.
  void promise.catch(() => undefined);
  const remaining = deadline - (Date.now() - started);
  if (remaining <= 0) throw expired();
  signal?.throwIfAborted();
  let timer: ReturnType<typeof setTimeout> | undefined;
  let onAbort: (() => void) | undefined;
  try {
    return await new Promise<T>((resolve, reject) => {
      timer = setTimeout(() => reject(expired()), remaining);
      onAbort = () => reject(signal?.reason ?? new MisoUsageError("spectrum query was aborted"));
      signal?.addEventListener("abort", onAbort, { once: true });
      promise.then(resolve, reject);
    });
  } finally {
    if (timer !== undefined) clearTimeout(timer);
    if (onAbort !== undefined) signal?.removeEventListener("abort", onAbort);
  }
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
  // The document as booted, kept apart from the one the scratch boot and the host are handed, for
  // `withSession()` to hold a builder to.
  const bootedDocument = document.slice();
  const policy = {
    ...options.policy,
    ...(spectrumHopFrames === undefined ? {} : { spectrumHopFrames }),
    ...(typeof options.policy?.liveControls === "object" ? { liveControls: { ...options.policy.liveControls } } : {}),
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
    await boundedContextClose(candidate);
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
    // The boot error stays the answer; the bound only ends the wait for the context.
    try { await boundedContextClose(context); } catch { /* the quantum refusal is the failure to report */ }
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
      }),
      simd128ModuleUrl,
      workletModuleUrl,
      ...(preparedModule === undefined ? {} : { preparedModule }),
    });
    // D2 of #1596: the boot's taps through one `SPECTRUM_CONFIGURE`, before the engine is returned.
    let spectrumQueue: ControlTransactionQueue | undefined;
    const bootSpectrum = preparedSpectrumCollection ?? preparedSpectrum;
    if (bootSpectrum !== undefined) {
      try {
        spectrumQueue = await configureBootSpectrum(
          host,
          bootSpectrum,
          spectrumHopFrames,
          spectrumDeadlineMs(options.requestDeadlineMs ?? 5_000),
          options.signal,
        );
      } catch (error) {
        try {
          await boundedCloseStep("worklet host dispose", "hostDisposeTimeout", async () => { await host.dispose(); });
        } catch { /* the spectrum boot refusal is the failure to report */ }
        throw error;
      }
    }
    let semanticLiveControls: Promise<EngineLiveControls> | undefined;
    let closePromise: Promise<void> | undefined;
    // The track response queries of this engine (#1579 D4): the same requester as the headless
    // engine, over the host's `control` message and event lane. Created with the host's one
    // transaction queue, whose allocator its request IDs come from.
    let trackResponses: Promise<TrackResponseRequester> | undefined;
    let trackResponseRequester: TrackResponseRequester | undefined;
    // The spectrum taps the core holds, as the boot or the last accepted `configureSpectrum` set
    // them: the subscription owner checks a subscription against them.
    let spectrumQuery: SpectrumQuery | undefined = preparedSpectrum;
    let spectrumCollection: SpectrumCollection | undefined = preparedSpectrumCollection;
    let spectrumPending = false;
    // Whether a managed stream start is in flight or its stream runs: the capture is the stream's,
    // so a one-shot query is refused, and a start is refused while a query runs (the core would
    // answer the overlap `spectrum.busy` only for some of its orders).
    let spectrumStreamHeld = false;
    let closed = false;
    // Issue #1644 D3: the largest generation a session seek of this object has used; every source
    // and the timeline start at 1.
    let highestSeekGeneration = 1n;
    // Issue #1297b: the PCM feeds whose `engine` is this one. A feed's anchored start takes its
    // generation from the same sequence as `seek`, so a later session seek is always above it.
    const feedRetirement = linkFeedEngine(host, () => {
      highestSeekGeneration += 1n;
      return highestSeekGeneration;
    });
    // Issue #1296: the session edits. `liveSession` and `liveDocument` are what `liveControls()`
    // binds to: the booted document until a committed `replaceSession` names another.
    let liveSession: SessionBuilder | undefined = options.document instanceof SessionBuilder ? options.document : undefined;
    let liveDocument: Uint8Array = bootedDocument;
    // The newest committed revision a reply, the queue, an event or the watermark showed this page.
    let seenRevision = 0n;
    // This host's one transaction queue once a call resolved it: the engine tells it every revision
    // a frame sent beside it reported (#1389 root ruling (b)), and reads its revision.
    let editQueue: ControlTransactionQueue | undefined;
    const noteRevision = (revision: bigint): void => {
      if (revision > seenRevision) seenRevision = revision;
    };
    const committedRevision = (): bigint => {
      // Folded into `seenRevision`, so the value read never goes back.
      noteRevision(editQueue?.revision ?? 0n);
      return seenRevision;
    };
    // The watermark is state the worklet posts only when it changes; the boot record is
    // `(0, 0, EXACT)` and is not posted, so it is this page's first snapshot.
    let latestWatermark = watermarkSnapshot(0n, 0n, 1n);
    const appliedLedger = new AppliedLedger();
    appliedLedger.settle(latestWatermark);
    // Subscribed at boot, before any frame that can commit is sent, so no post is missed. A host
    // from `createHost` that has no `onWatermark` (a stand-in) cannot report completion:
    // `applied()` then rejects `unsupported` instead of waiting forever.
    const watermarkHost = typeof host.onWatermark === "function";
    const stopWatermark = watermarkHost
      ? host.onWatermark((posted) => {
        if (closed) return;
        latestWatermark = watermarkSnapshot(posted.revision, posted.sample, posted.outcome);
        noteRevision(latestWatermark.revision);
        appliedLedger.settle(latestWatermark);
      })
      : () => undefined;
    // Issue #1210 D5: meter frames and observation bindings carry strip indices, the tracks then
    // the submixes. The order is the core's control list (#1564 D5), read again whenever its
    // structure epoch moves, and kept by epoch (#1562b): the plan that renders keeps naming its
    // strips by the previous list until the block that adopts a rebuild's replacement, and its
    // frames and map say which list that is.
    const stripOrders = createBrowserStripOrders(host);
    const measurementFeeds = createMeasurementFeeds(
      host,
      stripOrders,
      policy.liveControls !== undefined,
    );
    let observationSubscriptions: ObservationSubscriptionOwner | undefined;
    // The map and the strip IDs that name its bindings' indices, from one plan: the map follows the
    // plan that renders, whose list is the one the reply names (#1562b).
    const observationMapWithStrips = async (): Promise<{
      readonly map: ObservationMap;
      readonly stripIds: readonly string[];
    }> => {
      const reply = await host.observationMap();
      if (reply.result !== constantValue("resultCodes", "ok")) {
        throw new MisoEngineError("the browser host refused the observation map", {
          phase: "output",
          code: resultName(reply.result, "call"),
          result: reply.result,
        });
      }
      const order = await stripOrders.of(reply.structureEpoch);
      if (order === undefined) {
        // The list of the rendering plan was replaced before this page ever read it. The plan is
        // replaced too (a commit is adopted within a block or two), so a second ask is answered.
        throw new MisoEngineError("the observation map belongs to a control list that is gone", {
          phase: "lifecycle",
          code: "control.list.stale",
          diagnostics: [{ code: "control.list.stale", path: "structureEpoch" }],
        });
      }
      const stripIds = Object.freeze([...order.tracks, ...order.submixes]);
      return Object.freeze({
        map: enrichObservationMap(stripIds, reply.bindings as readonly RawObservationBinding[]),
        stripIds,
      });
    };
    const observationMap = async (): Promise<ObservationMap> => (await observationMapWithStrips()).map;
    const readObservations = async (
      selections: readonly ObservationSelection[],
    ): Promise<readonly ObservationReadResult[]> => {
      validateObservationSelections(selections);
      const { map, stripIds } = await observationMapWithStrips();
      const addresses = resolveObservationAddressesWithTracks(map, stripIds, selections);
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
        stripIds,
        selections,
        addresses,
        reply.rows as readonly RawObservationRow[],
        shape.sampleRateHz,
      );
    };
    const getLiveControls = (): Promise<EngineLiveControls> => {
      // Every value edit is a SET_CONTROL transaction, so the semantic controls need no boot word.
      if (semanticLiveControls === undefined) {
        const pending: Promise<EngineLiveControls> = createBrowserLiveControls(
          host,
          liveSession,
          liveDocument,
        );
        semanticLiveControls = pending;
        pending.then((controls) => { editQueue = controls.queue; }, () => undefined);
        // A probe that failed is not the answer for good: the next call asks the host again.
        pending.catch(() => {
          if (semanticLiveControls === pending) semanticLiveControls = undefined;
        });
      }
      return semanticLiveControls;
    };
    // A managed subscription is the core's `Observe` edit and its reads are `OBSERVATION_READ`,
    // both posted as the worklet's `control` message (#1589 D3) with the host's one request-ID
    // allocator. A reply's revision is noted for the transaction queue, as every frame beside it
    // is. A session booted with no observation taps binds none, so its map is empty and the
    // subscription is refused `observation.unavailable` before any edit (#1589 R1).
    const observationFrames = async () => {
      const queue = await createBrowserControlQueue(host);
      return observationFrameTransport({
        send: async (frame) => {
          const reply = await host.control(frame);
          queue.noteCommit(reply.revision);
          if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
          return reply.response;
        },
        requestIds: queue.requestIds,
        committedRevision: () => queue.revision,
        maximumFrameBytes: () => queue.maximumFrameBytes,
      });
    };
    // #1596 D1-D7: every spectrum call is one of the three frames, posted to the host's `control`
    // message; the core analyzes where the session runs (D4), so no analysis Worker starts.
    const spectrumFrame = async <T>(
      request: SpectrumFrameRequest,
      decode: (bytes: Uint8Array) => SpectrumFrameReply<T>,
    ): Promise<SpectrumFrameReply<T>> => {
      if (closed) throw new MisoUsageError("the browser engine is closed");
      // Held once known, so a frame is posted in the same turn as its call: a frame that undoes a
      // timed-out one is then posted, and so served, after it.
      const queue = spectrumQueue ?? await createBrowserControlQueue(host);
      spectrumQueue = queue;
      editQueue ??= queue;
      return sendSpectrumFrame(host, queue, request, decode);
    };
    /** Send `request` and drop its answer: it undoes a frame whose caller stopped waiting. */
    const sendQuietly = (request: SpectrumFrameRequest): void => {
      if (closed) return;
      const decode: (bytes: Uint8Array) => SpectrumFrameReply<unknown> = request.op === "stream"
        ? decodeSpectrumStreamReply : decodeSpectrumCaptureReply;
      void spectrumFrame(request, decode).catch(() => undefined);
    };
    const queryDeadline = (query?: SpectrumQuery): number =>
      spectrumDeadlineMs(query?.spectrumLimits?.requestDeadlineMs ?? options.requestDeadlineMs ?? 5_000);
    /**
     * One `SPECTRUM_STREAM` frame for the managed subscriptions, bounded by the deadline. A frame the
     * caller stopped waiting for may still start or restart the stream, so the stream is stopped
     * behind it and the managed lifetime ends.
     */
    const streamFrame = async (
      request: Extract<SpectrumFrameRequest, { op: "stream" }>,
      query?: SpectrumQuery,
    ): Promise<SpectrumFrameReply<SpectrumFrameStream>> => {
      const started = Date.now();
      const deadline = queryDeadline(query);
      try {
        return await awaitSpectrum(spectrumFrame(request, decodeSpectrumStreamReply), started, deadline, options.signal);
      } catch (error) {
        if (request.stream !== "stop" && !closed) {
          sendQuietly({ op: "stream", stream: "stop" });
          observationSubscriptions?.invalidateSpectrum();
        }
        spectrumStreamHeld = false;
        throw error;
      }
    };
    const configureSpectrum = async (
      configuration: SpectrumQuery | SpectrumCollection,
      configureOptions?: SpectrumConfigureOptions,
    ): Promise<SpectrumConfigured> => {
      if (closed) throw new MisoUsageError("the browser engine is closed");
      const { request, query, collection } = spectrumConfigureRequest(configuration, configureOptions);
      const reply = await awaitSpectrum(
        spectrumFrame(request, decodeSpectrumConfiguredReply),
        Date.now(),
        queryDeadline(),
        options.signal,
      );
      if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum configuration", reply);
      spectrumQuery = query;
      spectrumCollection = collection;
      return spectrumConfiguredOf(reply.value);
    };
    // The owner's select before a collection's first stream start: the capture is the stream's from
    // here, so the hold is taken now and given back if the select fails (the start keeps it, or
    // gives it back when it fails).
    const selectSpectrum = async (query: SpectrumQuery): Promise<EngineCallResult> => {
      if (spectrumPending) throw new MisoUsageError("a one-shot spectrum query is in flight");
      const selected = spectrumSelection(query);
      spectrumStreamHeld = true;
      let reply: SpectrumFrameReply<unknown>;
      try {
        reply = await awaitSpectrum(spectrumFrame({
          op: "capture",
          capture: "select",
          target: selected.target,
          channels: selected.channels ?? "both",
        }, decodeSpectrumCaptureReply), Date.now(), queryDeadline(query), options.signal);
      } catch (error) {
        spectrumStreamHeld = false;
        throw error;
      }
      if (!reply.ok) spectrumStreamHeld = false;
      return spectrumCallResult(reply);
    };
    const selectSpectrumStream = async (
      query: SpectrumQuery,
      smoothingMs: number,
    ): Promise<SpectrumStreamSelection> => {
      const selected = spectrumSelection(query);
      const reply = await streamFrame({
        op: "stream",
        stream: "select",
        target: selected.target,
        channels: selected.channels ?? "both",
        smoothingMs,
      }, query);
      if (!reply.ok) return spectrumCallResult(reply);
      return Object.freeze({ ...spectrumCallResult(reply), metadata: spectrumMetadataOf(reply.value) });
    };
    const startSpectrumStream = async (smoothingMs: number, query?: SpectrumQuery): Promise<SpectrumStreamStart> => {
      if (spectrumPending) throw new MisoUsageError("a one-shot spectrum query is in flight");
      spectrumStreamHeld = true;
      const reply = await streamFrame({ op: "stream", stream: "start", smoothingMs }, query);
      if (!reply.ok) {
        spectrumStreamHeld = false;
        throw spectrumRefusalError("the engine refused the spectrum stream start", reply);
      }
      return Object.freeze({ ...spectrumCallResult(reply), metadata: spectrumMetadataOf(reply.value) });
    };
    const readSpectrumStream = async (query?: SpectrumQuery): Promise<SpectrumStreamRead> => {
      const reply = await streamFrame({ op: "stream", stream: "read" }, query);
      if (!reply.ok) throw spectrumRefusalError("the engine refused the spectrum stream read", reply);
      const metadata = spectrumMetadataOf(reply.value);
      const result = spectrumResultOfStream(reply.value);
      return Object.freeze(result === undefined ? { metadata } : { metadata, result });
    };
    const stopSpectrumStream = async (): Promise<EngineCallResult> => {
      const reply = await streamFrame({ op: "stream", stream: "stop" });
      // A stop gives the capture back, whatever the core answers (a stream it already ended).
      spectrumStreamHeld = false;
      return spectrumCallResult(reply);
    };
    const observationOwner = (): ObservationSubscriptionOwner => observationSubscriptions ??=
      new ObservationSubscriptionOwner({
        observationMap,
        apply: async (edits) => (await observationFrames()).apply(edits),
        read: async (selections) => (await observationFrames()).read(selections),
        sampleRateHz: () => shape.sampleRateHz,
        // The core ends a subscription whose effect a rebuild removed with a reliable event; the
        // owner forgets the tap and tells its subscriber (#1562b-K).
        observationEnded: (listener) => host.onControlEvent((event) => {
          const ended = event.lane === 0 ? decodeObservationEnded(event.frame) : undefined;
          if (ended !== undefined) listener(ended);
        }),
        // The audio thread renders on its own clock, so an `Observe` refused with backpressure (an
        // earlier one is not yet drained) may wait for the host's render count to move and be sent
        // again. The wait is bounded by the engine's request deadline and ends in the same
        // typed refusal, so a context that does not render never holds the caller.
        renderWait: {
          boundMs: options.requestDeadlineMs ?? 5_000,
          mark: async () => (await host.status()).renderedQuanta,
          next: (timeoutMs, since) => new Promise<boolean>((resolve, reject) => {
            let settled = false;
            let poll: ReturnType<typeof setTimeout> | undefined;
            const finish = (rendered: boolean, error?: unknown): void => {
              if (settled) return;
              settled = true;
              clearTimeout(limit);
              if (poll !== undefined) clearTimeout(poll);
              if (error === undefined) resolve(rendered); else reject(error);
            };
            const limit = setTimeout(() => finish(false), timeoutMs);
            void (async () => {
              for (;;) {
                await new Promise<void>((tick) => { poll = setTimeout(tick, 4); });
                if (settled) return;
                if (closed) return finish(false);
                const current = await host.status();
                if (current.renderedQuanta > since) return finish(true);
              }
            })().catch((error: unknown) => finish(false, error));
          }),
        },
        responseRead: queryTrackResponse,
        spectrumPrepared: () => spectrumQuery,
        spectrumPreparedCollection: () => spectrumCollection,
        spectrumSelect: selectSpectrum,
        spectrumStreamSelect: selectSpectrumStream,
        spectrumStart: startSpectrumStream,
        spectrumRead: readSpectrumStream,
        spectrumStop: stopSpectrumStream,
        scheduler: {
          setInterval: (callback, milliseconds) => globalThis.setInterval(callback, milliseconds),
          clearInterval: (handle) => globalThis.clearInterval(handle as ReturnType<typeof setInterval>),
        },
      }, options.observationSubscriptionLimits, options.responseSubscriptionLimits, options.spectrumSubscriptionLimits);
    /**
     * One track response, as a C ABI host reads it: a strip `RESPONSE_GET` through the host's
     * `control` message, then the `RESPONSE_READY` with its tag from the event lane. The capture
     * runs in the render step, so the query settles after the next rendered block.
     */
    const trackResponseClosed = (): MisoUsageError => new MisoUsageError("the browser engine is closed");
    const requesterOf = (): Promise<TrackResponseRequester> => {
      if (trackResponses !== undefined) return trackResponses;
      const created = createBrowserControlQueue(host).then((queue) => {
        const requester = new TrackResponseRequester({
          // A frame beside the queue tells it the revision its reply reports (#1389 root ruling (b)).
          send: async (frame) => {
            const reply = await host.control(frame);
            queue.noteCommit(reply.revision);
            if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
            return reply.response;
          },
          onControlEvent: (listener) => host.onControlEvent(listener),
          nextRequestId: () => queue.requestIds.next(),
        });
        if (closed) requester.close(trackResponseClosed());
        trackResponseRequester = requester;
        return requester;
      });
      trackResponses = created;
      // A failed queue probe is not remembered; the next query probes again.
      created.catch(() => {
        if (trackResponses === created) trackResponses = undefined;
      });
      return created;
    };
    // While a query is pending, the host's status is read every few milliseconds: a failed or
    // disposed host rejects every pending and queued query at once with a lifecycle error (and so
    // every later one), instead of at the deadline. The host has no failure event to subscribe to.
    let trackResponsesInFlight = 0;
    let trackResponseWatch: Promise<void> | undefined;
    const trackResponseHostLost = (result: number): MisoEngineError =>
      new MisoEngineError("the browser host no longer owns a ready live response boundary", {
        phase: "lifecycle",
        code: resultName(result, "call"),
        result,
      });
    const watchTrackResponseHost = (requester: TrackResponseRequester): void => {
      if (trackResponseWatch !== undefined) return;
      trackResponseWatch = (async () => {
        try {
          while (trackResponsesInFlight > 0 && !closed) {
            let lost: MisoEngineError | undefined;
            try {
              const current = await host.status();
              if (current.result !== constantValue("resultCodes", "ok")) {
                lost = trackResponseHostLost(current.result);
              } else if (current.state !== constantValue("states", "ready")) {
                lost = trackResponseHostLost(constantValue("resultCodes", "wrongState"));
              }
            } catch (error) {
              const result = (error as { readonly result?: unknown } | null)?.result;
              lost = trackResponseHostLost(typeof result === "number"
                ? result : constantValue("resultCodes", "wrongState"));
            }
            if (lost !== undefined) {
              if (!closed) requester.close(lost);
              return;
            }
            await new Promise<void>((resolve) => { setTimeout(resolve, TRACK_RESPONSE_HOST_POLL_MS); });
          }
        } finally {
          trackResponseWatch = undefined;
        }
      })();
    };
    /**
     * One track response, as a C ABI host reads it: a strip `RESPONSE_GET` through the host's
     * `control` message, then the `RESPONSE_READY` with its tag from the event lane. The capture
     * runs in the render step, so the query settles after the next rendered block.
     */
    const queryTrackResponse = async (request: TrackResponseQuery): Promise<TrackResponseResult> => {
      if (closed) throw trackResponseClosed();
      trackResponsesInFlight += 1;
      try {
        const requester = await requesterOf();
        const query = requester.query(request);
        watchTrackResponseHost(requester);
        return await query;
      } finally {
        trackResponsesInFlight -= 1;
      }
    };
    const querySpectrum = async (request: SpectrumQuery): Promise<SpectrumResult> => {
      if (closed) throw new MisoUsageError("the browser engine is closed");
      if (spectrumPending) throw new MisoUsageError("a spectrum query is already in flight");
      if (spectrumStreamHeld) throw new MisoUsageError("a managed spectrum stream holds the capture");
      const selected = spectrumSelection(request);
      const channels = selected.channels ?? "both";
      const deadline = queryDeadline(request);
      // An arm and a read name no target: in the single form the SDK holds the one tap the core has.
      const single = spectrumQuery;
      if (single !== undefined && spectrumTargetKey(single.target) !== spectrumTargetKey(selected.target)) {
        throw new MisoUsageError("the spectrum query target is not the configured tap");
      }
      spectrumPending = true;
      const started = Date.now();
      // Whether the core may hold this query's arm: a query that ends before its window is read
      // cancels it, in a frame posted behind the arm.
      let armed = false;
      try {
        if (spectrumCollection !== undefined) {
          const selection = await awaitSpectrum(spectrumFrame({
            op: "capture", capture: "select", target: selected.target, channels,
          }, decodeSpectrumCaptureReply), started, deadline, options.signal);
          if (!selection.ok) throw spectrumRefusalError("the engine refused the spectrum selection", selection);
        }
        // Give back any arm this query does not own before it arms: a collection select arms the
        // entry for the core (#1592 D4), and an earlier query's undo may have been lost (a cancel
        // refused under backpressure), which would leave every later arm `spectrum.busy`.
        const released = await awaitSpectrum(
          spectrumFrame({ op: "capture", capture: "cancel" }, decodeSpectrumCaptureReply),
          started,
          deadline,
          options.signal,
        );
        if (!released.ok) throw spectrumRefusalError("the engine refused the spectrum capture cancel", released);
        armed = true;
        const arm = await awaitSpectrum(
          spectrumFrame({ op: "capture", capture: "arm" }, decodeSpectrumCaptureReply),
          started,
          deadline,
          options.signal,
        );
        if (!arm.ok) {
          armed = false;
          throw spectrumRefusalError("the engine refused the spectrum capture arm", arm);
        }
        for (;;) {
          const read = await awaitSpectrum(
            spectrumFrame({ op: "capture", capture: "read", channels }, decodeSpectrumCaptureReply),
            started,
            deadline,
            options.signal,
          );
          if (!read.ok) throw spectrumRefusalError("the engine refused the spectrum capture read", read, "lifecycle");
          if (read.value.status !== "pending") {
            armed = false;
            if (read.value.window === undefined) throw malformed("a spectrum read answered done");
            return spectrumResultOfWindow(read.value.window);
          }
          // The window is not complete: ask again after the audio thread rendered a little more.
          await awaitSpectrum(new Promise<void>((resolve) => setTimeout(resolve, 4)), started, deadline, options.signal);
        }
      } finally {
        if (armed) sendQuietly({ op: "capture", capture: "cancel" });
        spectrumPending = false;
      }
    };
    // The protocol calls (#1571 D2) share the host's one request-ID allocator, the transaction
    // queue's. The host exposes the committed revision only in a reply, so `TRANSPORT_SET` and
    // `TELEMETRY_CONFIGURE` read it with a revision probe, which also sees another sender's commit.
    let protocol: Promise<ProtocolClient> | undefined;
    const protocolClient = (): Promise<ProtocolClient> => {
      if (protocol !== undefined) return protocol;
      const created = (async () => {
        // The queue's own revision probe (`readCommittedRevision`): the browser has one probe.
        const queue = await createBrowserControlQueue(host);
        editQueue = queue;
        return new ProtocolClient({
          // Every frame of these calls goes beside the queue, so the queue is told the revision its
          // reply reports in the continuation of that reply (#1389 root ruling (b)): an `apply`
          // would otherwise make the queue's next `SET_CONTROL` expect an old revision. A live
          // control frame sent while an edit's reply is still in flight is refused
          // `revision_conflict`, and the queue takes the new revision from that refusal's reply
          // (no lock, no retry: #1389 R3).
          send: async (frame, document) => {
            const reply = await host.control(frame, document);
            queue.noteCommit(reply.revision);
            if (reply.result !== 0) throw controlCallFailure(reply.result, reply.response);
            return reply.response;
          },
          // The host's one allocator, the queue's: the engine's replay cache refuses a reused ID.
          requestIds: queue.requestIds,
          committedRevision: async () => {
            const revision = await queue.readCommittedRevision();
            if (revision === undefined) throw new MisoEngineError("the browser queue cannot read the committed revision", { phase: "lifecycle", code: "internal" });
            return revision;
          },
        });
      })();
      protocol = created;
      // A failed queue probe is not remembered; the next call probes again.
      created.catch(() => {
        if (protocol === created) protocol = undefined;
      });
      return created;
    };
    const engine: BrowserEngine<AudioContextLike> = Object.freeze({
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
      configureSpectrum,
      liveControls: getLiveControls,
      controls: async (filter?: ControlListFilter) =>
        readControlList(await createBrowserControlListTransport(host), filter),
      controlState: async (list: ControlList, options?: Pick<ControlListFilter, "pageLimit">) =>
        readControlState(await createBrowserControlListTransport(host), list, options),
      capabilities: async () => (await protocolClient()).capabilities(),
      committedDocument: async () => (await protocolClient()).committedDocument(),
      transport: async () => (await protocolClient()).transport(),
      setTransport: async (request: TransportRequest) => (await protocolClient()).setTransport(request),
      configureTelemetry: async (configuration: TelemetryConfiguration) =>
        (await protocolClient()).configureTelemetry(configuration),
      counters: async (ids?: readonly CounterName[]) => (await protocolClient()).counters(ids),
      diagnostics: async (query?: DiagnosticsQuery) => (await protocolClient()).diagnostics(query),
      onEvent: (listener: (event: ProtocolEvent) => void) => {
        if (typeof listener !== "function") {
          throw new MisoUsageError("an event listener must be a function");
        }
        return host.onControlEvent((event) => {
          // The host calls its listeners in turn from the port's message handler: a throw here
          // would skip the listeners after this one, so it is reported outside the call instead.
          // A `RESPONSE_READY` answers a track response query, which reads it (#1579a).
          if (isResponseReady(event.frame)) return;
          try {
            const decoded = decodeProtocolEvent(event.frame);
            // Another sender's commit is seen here too (#1296 root R2).
            if (decoded.kind === "sessionCommitted") noteRevision(decoded.revision);
            listener(decoded);
          } catch (error: unknown) {
            reportListenerError(error);
          }
        });
      },
      seek: async (frame: bigint, options?: BrowserSeekOptions) => {
        if (closed) throw new MisoUsageError("the browser engine is closed");
        if (typeof frame !== "bigint" || frame < 0n || frame > BigInt(Number.MAX_SAFE_INTEGER)) {
          throw new MisoUsageError("seek frame must be a non-negative bigint at most Number.MAX_SAFE_INTEGER");
        }
        const generation = options?.generation ?? highestSeekGeneration + 1n;
        if (typeof generation !== "bigint" || generation <= 0n) {
          throw new MisoUsageError("seek generation must be a positive bigint");
        }
        const producers = options?.producers ?? [];
        if (!Array.isArray(producers) || producers.some((producer) => typeof producer?.seek !== "function")) {
          throw new MisoUsageError("seek producers must each have a seek method");
        }
        const refused = (result: number): MisoEngineError =>
          new MisoEngineError("the browser host refused the session seek", {
            phase: "source",
            code: resultName(result, "call"),
            result,
          });
        let reply: Awaited<ReturnType<MisoAudioWorkletHost["seekSession"]>>;
        try {
          reply = await host.seekSession({ generation, frame });
        } catch (error) {
          // The host rejects a local refusal (a second unsettled seek: `backpressure`) and a
          // sticky failure with its numeric record; keep the result in the SDK's vocabulary.
          const result = (error as { readonly result?: unknown }).result;
          if (typeof result === "number" && Number.isSafeInteger(result) && result >= 0) {
            throw refused(result);
          }
          throw error;
        }
        if (reply.result !== constantValue("resultCodes", "ok")) throw refused(reply.result);
        if (generation > highestSeekGeneration) highestSeekGeneration = generation;
        for (const [index, producer] of producers.entries()) {
          try {
            await producer.seek(Number(frame), generation);
          } catch (error) {
            throw new SessionSeekProducerError(generation, index, error);
          }
        }
        return Object.freeze({ generation });
      },
      apply: async (transaction: SessionTransaction) => {
        if (closed) throw new MisoUsageError("the browser engine is closed");
        const outcome = await (await protocolClient()).applyTransaction(transaction);
        // Issue #1297b D6: a removed source's ring keeps draining until the removal is in effect.
        const removed = transaction.edits.flatMap((edit) => edit.kind === "removeSource" ? [edit.sourceId] : []);
        if (removed.length !== 0) feedRetirement.retire(removed, () => engine.applied(outcome.revision));
        return outcome;
      },
      replaceSession: async (replacement: CreateEngineOptions["document"]) => {
        if (closed) throw new MisoUsageError("the browser engine is closed");
        const bytes = documentBytes(replacement);
        const outcome = await (await protocolClient()).replaceSession(bytes);
        // The committed model is now `replacement`'s: the next `liveControls()` binds to it. A
        // refusal threw above and rebinds nothing.
        liveSession = replacement instanceof SessionBuilder ? replacement : undefined;
        liveDocument = bytes;
        semanticLiveControls = undefined;
        // Issue #1297b D6 (root R4): the sources the replace removed are the fed ones the committed
        // session map no longer lists; the SDK never parses the document for them.
        if (outcome.path !== "unchanged") {
          feedRetirement.retireAbsent(async () => {
            const map = await host.sessionMap();
            if (map.result !== constantValue("resultCodes", "ok")) throw new Error("session map refused");
            return new Set(map.sources.map((source) => source.id));
          }, () => engine.applied(outcome.revision));
        }
        return outcome;
      },
      get revision(): bigint {
        return committedRevision();
      },
      watermark: () => latestWatermark,
      applied: (revision: bigint): Promise<WatermarkSnapshot> => {
        if (typeof revision !== "bigint" || revision < 0n) {
          return Promise.reject(new MisoUsageError("applied() takes a non-negative bigint revision"));
        }
        if (closed) return Promise.reject(new MisoUsageError("the browser engine is closed"));
        if (!watermarkHost) {
          return Promise.reject(new MisoEngineError("the browser host posts no applied-revision watermark", {
            phase: "lifecycle",
            code: "unsupported",
            result: constantValue("resultCodes", "unsupported"),
            diagnostics: [{ code: "sdk.host.member", path: "onWatermark" }],
          }));
        }
        if (revision <= committedRevision()) return appliedLedger.applied(revision);
        // Above every revision this page saw: another sender may have committed it unseen (no
        // `onEvent` listener, no render yet), so ask the core before refusing.
        return (async () => {
          const queue = editQueue ?? await createBrowserControlQueue(host);
          editQueue = queue;
          const probed = await queue.readCommittedRevision();
          if (probed !== undefined) noteRevision(probed);
          const committed = committedRevision();
          if (revision > committed) {
            throw new MisoUsageError(
              `revision ${revision} is not committed; the committed revision is ${committed}`,
            );
          }
          if (closed) throw new MisoUsageError("the browser engine is closed");
          return appliedLedger.applied(revision);
        })();
      },
      close: () => {
        closed = true;
        stopWatermark();
        appliedLedger.rejectAll(new MisoUsageError("the browser engine is closed"));
        editQueue?.close();
        measurementFeeds.close();
        observationSubscriptions?.invalidate(true);
        // Every pending and queued track response query rejects now, not at its deadline.
        trackResponseRequester?.close(trackResponseClosed());
        // Edits staged for the host are rejected and no new one is accepted; a transaction already
        // in flight settles.
        void semanticLiveControls?.then((controls) => controls.queue.close(), () => undefined);
        closePromise ??= (async () => {
          // The first failure in step order wins; later ones are dropped (D4).
          let failed = false;
          let failure: unknown;
          const fail = (error: unknown): void => {
            if (!failed) { failed = true; failure = error; }
          };
          try {
            await boundedCloseStep("worklet host dispose", "hostDisposeTimeout", async () => { await host.dispose(); });
          } catch (error) {
            fail(error);
          }
          try {
            // A failed or stalled MessagePort disposal must not leak the much larger AudioContext.
            await boundedContextClose(context);
          } catch (error) {
            fail(error);
          }
          if (failed) throw failure;
        })();
        return closePromise;
      },
    });
    return engine;
  } catch (error) {
    // The boot error stays the answer; the bound only ends the wait for the context.
    try { await boundedContextClose(context); } catch { /* the worklet boot failure is the failure to report */ }
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
  const options = { ...request.options, ...(typeof request.options.liveControls === "object" ? { liveControls: { ...request.options.liveControls } } : {}) };
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
  const options = { ...request.options, ...(typeof request.options.liveControls === "object" ? { liveControls: { ...request.options.liveControls } } : {}) };
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
