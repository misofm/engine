import { constantValue } from "../core/abi.ts";
import type { BootOptions } from "../core/abi.ts";
import { MisoEngineAsset } from "../core/asset.ts";
import { WasmBoundary } from "../core/boundary.ts";
import type {
  ControlEvent,
  EngineCallResult,
  EngineStatus,
  MeterFrame,
  PlanLatency,
  SessionMap,
  SessionShape,
} from "../core/boundary.ts";
import type {
  ObservationMap,
  ObservationReadResult,
  ObservationSelection,
} from "../core/observation.ts";
import { isResponseReady } from "../core/response-frame.ts";
import type { TrackResponseQuery, TrackResponseResult } from "../core/live-response.ts";
import { cloneSpectrumCollection, cloneSpectrumQuery } from "../core/spectrum.ts";
import { LoudnessOwner } from "../core/loudness.ts";
import type { LoudnessSubscription, LoudnessSubscriptionRequest } from "../core/loudness.ts";
import { decodeLoudnessReply, encodeLoudnessCommand } from "../core/loudness-frame.ts";
import type {
  SpectrumCollection,
  SpectrumConfigureOptions,
  SpectrumConfigured,
  SpectrumQuery,
  SpectrumResult,
} from "../core/spectrum.ts";
import {
  ObservationSubscriptionOwner,
  observationFrameTransport,
  observationMapOf,
  readObservationResults,
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
import { decodeControlLimits, encodeRevisionProbe, malformed } from "../core/control-frame.ts";
import { ControlTransactionQueue } from "../core/control-queue.ts";
import { readControlList, readControlState } from "../core/control-list.ts";
import type {
  ControlList,
  ControlListFilter,
  ControlListTransport,
  ControlState,
} from "../core/control-list.ts";
import { EngineLiveControls, sessionDescribesStrips } from "../core/live-controls.ts";
import { decodeObservationEnded } from "../core/observation-ended.ts";
import { ProtocolClient, decodeProtocolEvent } from "../core/protocol-messages.ts";
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
import { SessionBuilder } from "../core/session.ts";
import { AppliedLedger } from "../core/session-edit.ts";
import type { EditOutcome, WatermarkSnapshot } from "../core/session-edit.ts";
import type { SessionTransaction } from "../core/session-transaction.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";
import type { ErrorPhase, MisoDiagnostic, MisoErrorCode } from "../core/errors.ts";
import { loadBundledEngineAsset } from "./assets.ts";

let defaultAsset: Promise<MisoEngineAsset> | undefined;

function defaultBundledAsset(): Promise<MisoEngineAsset> {
  if (defaultAsset === undefined) {
    defaultAsset = loadBundledEngineAsset().catch((error: unknown) => {
      // A transient read failure must not poison the process forever. Successful compilation is
      // cached for the SDK lifetime; a failed attempt may be retried after the deployment is fixed.
      defaultAsset = undefined;
      throw error;
    });
  }
  return defaultAsset;
}

/**
 * Faster-than-realtime rendering over the browser engine, in Node or Bun.
 *
 * # One verb, two input paths
 *
 * The pre-boot-v1 SDK had a plan path and a raw JSON path that reached the engine differently, and
 * the raw path is where every one of #207's red probes failed: a regex header sniff that could not
 * read a quoted key, a silent 48 kHz/128 fallback when it failed, and a fabricated ring that was
 * not a multiple of a 127-frame quantum. Both paths now do exactly one thing -- hand the engine
 * bytes and ask it what it compiled -- so a `SessionPlan` and a raw document are the same call with
 * a different `toString`.
 *
 * # The headless engine needs no scratch instance
 *
 * A browser has a physical rate and quantum it must satisfy, so it boots a scratch instance in a
 * Worker to discover a document's shape before committing an `AudioContext`. Headless has no
 * physical anything: its one instance *is* the boot, with both `require_*` words at zero, and it
 * accepts whatever the document declares. That is why there is no scratch path in this file.
 */

/** Anything that can be handed to the engine as a Session V1 document. */
export type SessionDocument = string | Uint8Array | { toJson(): string };

/**
 * The builder behind a document, when the SDK built it: what lets live controls resolve console
 * slot and insert IDs without the SDK ever parsing a document.
 */
function builtSession(document: SessionDocument): SessionBuilder | undefined {
  return document instanceof SessionBuilder ? document : undefined;
}

function documentBytes(document: SessionDocument): Uint8Array<ArrayBuffer> {
  if (typeof document === "string") return new TextEncoder().encode(document);
  if (document instanceof Uint8Array) {
    return document.buffer instanceof ArrayBuffer
      ? (document as Uint8Array<ArrayBuffer>)
      : new Uint8Array(document);
  }
  if (typeof document.toJson === "function") {
    return new TextEncoder().encode(document.toJson());
  }
  throw new MisoUsageError(
    "a session document must be a string, a Uint8Array, or an object with toJson()",
  );
}

export interface OfflineEngineOptions extends BootOptions {
  /** The verified, compiled module. Absent loads the engine embedded in this package. */
  readonly asset?: MisoEngineAsset;
  /** Finite SDK-side bounds for resident observation subscriptions. */
  readonly observationSubscriptionLimits?: ObservationSubscriptionLimits;
  /** Finite SDK-side bounds for managed live track-response subscriptions. */
  readonly responseSubscriptionLimits?: TrackResponseSubscriptionLimits;
  /** Finite SDK-side bounds for managed spectrum subscriptions. */
  readonly spectrumSubscriptionLimits?: SpectrumSubscriptionLimits;
}

/** A booted headless session. */
export class OfflineEngine {
  readonly #boundary: WasmBoundary;
  readonly #asset: MisoEngineAsset;
  #session: SessionBuilder | undefined;
  /** The document the engine booted, as staged: what `withSession()` holds a builder to. */
  #booted: Uint8Array | undefined;
  readonly #observationLimits: ObservationSubscriptionLimits | undefined;
  #observationSubscriptions: ObservationSubscriptionOwner | undefined;
  /** The engine's one loudness owner (#2115 S3), made on first use. */
  #loudness: LoudnessOwner | undefined;
  /** The engine's one transaction queue (#1563 D4), made on first use. */
  #controlQueue: ControlTransactionQueue | undefined;
  /** The engine's protocol calls (#1571 D2), made on first use. */
  #protocol: ProtocolClient | undefined;
  readonly #responseLimits: TrackResponseSubscriptionLimits | undefined;
  readonly #spectrumLimits: SpectrumSubscriptionLimits | undefined;
  /**
   * The largest generation a successful `seek`, `seekSource` or `submitSource` of this object has
   * passed to the engine. Every source and the timeline start at generation 1. It is the only
   * state `seek` adds: the core refuses a stale generation, so this only picks the next one.
   */
  #highestGeneration = 1n;
  /** The pending `applied(revision)` promises, settled from the watermark after each `render()`. */
  readonly #applied = new AppliedLedger();
  /** Bumped by every `loadSession`, so a replace that awaited across one rebinds nothing. */
  #sessionEpoch = 0;

  private constructor(
    asset: MisoEngineAsset,
    boundary: WasmBoundary,
    session: SessionBuilder | undefined,
    booted: Uint8Array,
    observationLimits: ObservationSubscriptionLimits | undefined,
    responseLimits: TrackResponseSubscriptionLimits | undefined,
    spectrumLimits: SpectrumSubscriptionLimits | undefined,
  ) {
    this.#asset = asset;
    this.#boundary = boundary;
    this.#session = session;
    this.#booted = booted;
    this.#observationLimits = observationLimits;
    this.#responseLimits = responseLimits;
    this.#spectrumLimits = spectrumLimits;
  }

  static async create(
    document: SessionDocument,
    options: OfflineEngineOptions = {},
  ): Promise<OfflineEngine> {
    const {
      asset: suppliedAsset,
      observationSubscriptionLimits,
      responseSubscriptionLimits,
      spectrumSubscriptionLimits,
      ...boot
    } = options;
    const spectrumQuery = boot.spectrum === undefined ? undefined : cloneSpectrumQuery(boot.spectrum);
    const spectrumCollection = boot.spectrumCollection === undefined
      ? undefined : cloneSpectrumCollection(boot.spectrumCollection);
    if (spectrumQuery !== undefined && spectrumCollection !== undefined) {
      throw new MisoUsageError("spectrum and spectrumCollection are mutually exclusive");
    }
    const bootOptions = spectrumQuery === undefined && spectrumCollection === undefined
      ? boot
      : {
        ...boot,
        ...(spectrumQuery === undefined ? {} : { spectrum: spectrumQuery }),
        ...(spectrumCollection === undefined ? {} : { spectrumCollection }),
      };
    const asset = suppliedAsset ?? await defaultBundledAsset();
    const bytes = documentBytes(document);
    return new OfflineEngine(
      asset,
      await WasmBoundary.boot(asset, bytes, bootOptions),
      builtSession(document),
      // A copy: a caller's Uint8Array is staged as is, and may change after the boot.
      bytes.slice(),
      observationSubscriptionLimits,
      responseSubscriptionLimits,
      spectrumSubscriptionLimits,
    );
  }

  /** The asset this engine booted from, including its compile count and provenance. */
  get asset(): MisoEngineAsset {
    return this.#asset;
  }

  /**
   * What the engine compiled: rate, quantum, derived ring, backend, sources, tracks.
   *
   * Every field is the engine's answer. Nothing here was parsed out of the document's text, which
   * is the whole point -- a consumer that reads its rate from this object cannot be told 48000 by
   * a fallback that never looked.
   */
  shape(): SessionShape {
    return this.#boundary.shape();
  }

  state(): "ready" | "failed" | "disposed" {
    return this.#boundary.state();
  }

  status(): EngineStatus {
    return this.#boundary.status();
  }

  sessionMap(): SessionMap {
    return this.#boundary.sessionMap();
  }

  /**
   * The session's effects that declare taps, by stable IDs: the core's control list (its section
   * headers name each console slot's and insert's effect) and the generated catalog's taps (#1590
   * D3).
   */
  observationMap(): ObservationMap {
    return observationMapOf(this.#boundary.stripOrder());
  }

  /**
   * Read one bounded non-consuming batch from selected resident observation taps: one
   * `OBSERVATION_READ` through this engine's control entry (#1590), each selection checked against
   * {@link observationMap} first.
   */
  readObservations(selections: readonly ObservationSelection[]): readonly ObservationReadResult[] {
    return readObservationResults(this.observationMap(), selections, this.#boundary.status().sampleRateHz, {
      send: (frame) => this.controlFrame(frame),
      requestIds: this.#boundary.controlRequestIds,
      maximumFrameBytes: this.controlTransactions().maximumFrameBytes,
    });
  }

  /**
   * Subscribe to bounded resident observation rows; values are refreshed by `pump()`.
   *
   * The subscription is the core's `Observe` edit, sent in a `SESSION_TRANSACTION_APPLY` through
   * this engine's control entry, and its rows are `OBSERVATION_READ` reads (#1589 D3). An effect's
   * observation queue holds one `Observe` per tap until a block drains it: a subscribe, update or
   * close that finds the slot taken by an earlier one is refused at once with the typed, retryable
   * `backpressure` error. It never waits: this engine renders only when the caller does, so the
   * caller calls `render()` and submits again. A core refusal rejects with `MisoEngineError` whose
   * `diagnosticCode` is the core's code. A session that binds no observation tap (no effect
   * declares one) refuses with `observation.unavailable` before any edit is sent; a full frame
   * of armed taps is `observationLimit`. `armed()` settles at the `pump()` that reads the arm.
   */
  subscribeObservations(request: ObservationSubscriptionRequest): Promise<ObservationSubscription> {
    return this.#observationOwner().subscribe(request).then((receipt) => receipt.handle);
  }

  /**
   * Subscribe to one selected track response; values are refreshed by `pump()`. Each read is a
   * track response query, which settles at the next `render()` the app calls: the subscription and
   * each `pump()` that reads settle only once the app renders.
   */
  subscribeTrackResponse(request: TrackResponseSubscriptionRequest): Promise<TrackResponseSubscription> {
    return this.#observationOwner().subscribeTrackResponse(request).then((receipt) => receipt.handle);
  }

  /** Subscribe to the prepared continuous spectrum boundary; values are refreshed by `pump()`. */
  subscribeSpectrum(request: SpectrumSubscriptionRequest): Promise<SpectrumSubscription> {
    return this.#observationOwner().subscribeSpectrum(request).then((receipt) => receipt.handle);
  }

  /**
   * Start the session's output loudness meter (`LOUDNESS` start, #2115) and return its managed
   * handle; readings are refreshed by the handle's `pump()`, one `LOUDNESS` read each, which reads
   * the hops the renders since the last read delivered. `close()` stops the meter. The core refuses
   * a start while the meter is armed (`invalidArgument`, `diagnosticCode` `loudness.active`), and an
   * engine whose `CAPABILITIES_GET` lacks flag bit 17 is refused `unsupported`
   * (`sdk.asset.capability`). `loadSession()` and `dispose()` end the handle.
   */
  subscribeLoudness(request?: LoudnessSubscriptionRequest): Promise<LoudnessSubscription> {
    this.#loudness ??= new LoudnessOwner({
      send: async (operation) => {
        const requestId = this.#boundary.controlRequestIds.next();
        const reply = decodeLoudnessReply(this.controlFrame(encodeLoudnessCommand(requestId, operation)));
        if (reply.requestId !== requestId) throw malformed(`the reply answers request ${reply.requestId}, not ${requestId}`);
        return reply;
      },
      capabilityFlags: async () => (await this.capabilities()).flags,
    });
    return this.#loudness.subscribe(request);
  }

  /**
   * The response of one track, captured at a render boundary (#1579a D2): it sends the strip
   * `RESPONSE_GET` a C ABI host sends and settles with the `RESPONSE_READY` that answers it, which
   * the core produces in its render step. The promise therefore settles after the next `render()`
   * the app calls, and rejects when the request deadline (`responseLimits.requestDeadlineMs`,
   * default 5 s) passes first. Queries run one at a time, in call order.
   * A query made from inside an event callback sends its frame there, which runs a nested event
   * drain: the events still queued reach the other listeners before the callback returns.
   */
  queryTrackResponse(request: TrackResponseQuery): Promise<TrackResponseResult> {
    return this.#boundary.queryTrackResponse(request);
  }

  /**
   * Set, change or remove the spectrum taps of the running session with one `SPECTRUM_CONFIGURE`
   * (#1596 D5): a query sets one tap, a collection several, and an empty collection with budget 0
   * removes them. A change of taps prepares a replacement plan that the next `render()` adopts; the
   * taps are in effect once a spectrum reply reports `adoptedEpoch === configuredEpoch`. A core
   * refusal rejects with `MisoEngineError` whose `diagnosticCode` is the core's code.
   */
  configureSpectrum(
    configuration: SpectrumQuery | SpectrumCollection,
    options?: SpectrumConfigureOptions,
  ): Promise<SpectrumConfigured> {
    try {
      return Promise.resolve(this.#boundary.configureSpectrum(configuration, options));
    } catch (error) {
      return Promise.reject(error);
    }
  }

  /** Arm the selected spectrum capture for its next complete 2048-frame window. */
  armSpectrum(): EngineCallResult {
    return this.#boundary.armSpectrum();
  }

  /** Read the completed spectrum window, analyzed; `undefined` means more explicit renders are needed. */
  readSpectrum(): SpectrumResult | undefined {
    return this.#boundary.readSpectrum();
  }

  /** Cancel the pending spectrum capture. */
  cancelSpectrum(): EngineCallResult {
    if (this.#observationSubscriptions?.managedSpectrumActive()) {
      const result = constantValue("resultCodes", "wrongState");
      return Object.freeze({ ok: false, result, code: "wrongState" });
    }
    return this.#boundary.cancelSpectrum();
  }

  /**
   * Semantic live controls bound to the currently loaded session.
   *
   * When that session was booted from a `SessionBuilder`, console slots and inserts are also
   * addressable by their stable IDs; otherwise use `withSession()` or address them by index.
   */
  liveControls(): EngineLiveControls {
    const map = this.sessionMap();
    // A rebuild can move the strips away from the session the SDK built; the engine is then no
    // longer that session, and the core's control list is the only description of it.
    const session = this.#session !== undefined && sessionDescribesStrips(this.#session, map)
      ? this.#session
      : undefined;
    return new EngineLiveControls(map, {
      queue: this.controlTransactions(),
      // The boundary caches the order by structure epoch: a probe each call, the list when it moved.
      order: this.#boundary.stripOrder(),
    }, session, this.#booted);
  }

  /**
   * This engine's one control transaction queue (#1563 D4).
   *
   * Every `SET_CONTROL` transaction this engine sends goes through it, one in flight at a time, each
   * with the committed revision the last reply reported. The queue calls the core in process through
   * the module's control entry, and starts from the core's own committed revision
   * (`miso_engine_web_v1_control_revision`); a module without those exports refuses here with
   * `unsupported`. It starts over at the revision of the session that replaces the booted one on
   * `loadSession()`, and closes on `dispose()`.
   */
  controlTransactions(): ControlTransactionQueue {
    if (this.#controlQueue === undefined) {
      // The boundary's one allocator: its strip-order reads draw from the same endpoint.
      const requestIds = this.#boundary.controlRequestIds;
      // The core reports its own frame ceiling, which the queue holds every transaction under.
      const limits = decodeControlLimits(
        this.#boundary.controlFrame(encodeRevisionProbe(requestIds.next())),
      );
      this.#controlQueue = new ControlTransactionQueue(
        (frame) => this.#boundary.controlFrame(frame),
        {
          initialRevision: this.#boundary.controlRevision(),
          requestIds,
          limits,
          readRevision: () => this.#boundary.controlRevision(),
        },
      );
    }
    return this.#controlQueue;
  }

  /**
   * Every control of the running session, with its descriptor and committed value (decision 17, C4;
   * #1564 D1): `CONTROL_LIST_GET` pages read through this engine's control endpoint until `eof`.
   * The core decides the rows, their order and their values; this call only pages.
   *
   * A page refused `control.list.stale` (the structure changed between two pages) restarts the read
   * from the first page, and after three restarts rejects with `MisoEngineError` `control.list.stale`.
   * `filter.owner` restricts the list to one owner, `filter.pageLimit` sets the rows per request.
   */
  controls(filter?: ControlListFilter): Promise<ControlList> {
    return readControlList(this.#controlListTransport(), filter);
  }

  /**
   * The committed values of `list`'s control rows (`CONTROL_STATE_GET`), by handle. A structure that
   * changed since `list` was read rejects with `MisoEngineError` `control.list.stale`: list again.
   */
  controlState(list: ControlList, options?: Pick<ControlListFilter, "pageLimit">): Promise<ControlState> {
    return readControlState(this.#controlListTransport(), list, options);
  }

  #controlListTransport(): ControlListTransport {
    return {
      send: (frame) => this.#boundary.controlFrame(frame),
      // The one allocator of this endpoint: the engine's replay cache refuses a reused ID.
      requestIds: this.#boundary.controlRequestIds,
    };
  }

  /**
   * Send one command frame to the core and return its reply frame: the entry the C ABI's
   * `miso_engine_v1_submit_command` runs. Nothing in the frame is decoded here. The engine's
   * replay cache refuses a reused request ID, so a frame sent this way draws its ID from
   * `controlTransactions().requestIds`, the one allocator that serves this endpoint.
   */
  controlFrame(frame: Uint8Array): Uint8Array {
    try {
      return this.#boundary.controlFrame(frame);
    } finally {
      this.#noteCommit();
    }
  }

  /** The committed session revision the core reports (`miso_engine_web_v1_control_revision`). */
  controlRevision(): bigint {
    return this.#boundary.controlRevision();
  }

  /**
   * Subscribe to the protocol events of the engine's control plane (#1570b): `{ lane, frame }` for
   * each event, in arrival order, `lane` `0` for the reliable lane and `1` for the lossy lane. The
   * engine is read for events after every control frame, because a commit queues two and the next
   * commit waits for them to be read, and after every `render()`, because a block queues its
   * telemetry and the lossy lane holds one event. An event that arrives with no listener is dropped
   * and counted in `discardedControlEvents` as it is read: with no listener, each block's lossy
   * telemetry is dropped after that block, so a listener subscribed later does not receive the last
   * block's lossy event. A listener that throws never loses the reply of the frame that
   * produced the event: the throw is counted in `controlListenerFailures` and reported
   * asynchronously. Returns the unsubscribe function.
   */
  onControlEvent(listener: (event: ControlEvent) => void): () => void {
    return this.#boundary.onControlEvent(listener);
  }

  /** The core's capabilities (`CAPABILITIES_GET`, #1571 D3). `supportedCommands` differs per host. */
  capabilities(): Promise<EngineCapabilities> {
    return this.#protocolClient().capabilities();
  }

  /**
   * The canonical committed session document and its revision (`SESSION_SNAPSHOT_GET` pages,
   * #1571 D3). A session that changes between pages starts the read again, at most three times.
   */
  committedDocument(): Promise<CommittedDocument> {
    return this.#protocolClient().committedDocument();
  }

  /** The transport's state and the sample it is effective at (`TRANSPORT_GET`). */
  transport(): Promise<TransportSnapshot> {
    return this.#protocolClient().transport();
  }

  /** Start or stop the transport (`TRANSPORT_SET`) at the committed revision; resolves to the new state. */
  setTransport(request: TransportRequest): Promise<TransportSnapshot> {
    return this.#protocolClient().setTransport(request);
  }

  /**
   * Replace the telemetry configuration (`TELEMETRY_CONFIGURE`) at the committed revision; resolves
   * to the configuration the core applied. The meters, counters and diagnostics it names arrive as
   * `onEvent` events after the renders that produce them.
   */
  configureTelemetry(configuration: TelemetryConfiguration): Promise<TelemetryConfiguration> {
    return this.#protocolClient().configureTelemetry(configuration);
  }

  /** The named counters, or every counter when `ids` is absent (`COUNTERS_GET`). */
  counters(ids?: readonly CounterName[]): Promise<CounterSnapshot> {
    return this.#protocolClient().counters(ids);
  }

  /** One page of the core's retained diagnostics (`DIAGNOSTICS_GET`); defaults `0n`, 64 and `"info"`. */
  diagnostics(query?: DiagnosticsQuery): Promise<DiagnosticsPage> {
    return this.#protocolClient().diagnostics(query);
  }

  /**
   * Subscribe to the protocol events, decoded (#1571 D4): `sessionCommitted`, `transportState`,
   * `observationEnded`, `meterBatch`, `counterSnapshot` and `diagnostic`, in engine order. The engine
   * reads its events after every control frame and every `render()`; it decodes the frames
   * `onControlEvent` delivers, so an event that arrives with no listener at all is dropped and
   * counted in `discardedControlEvents`, and a listener that throws (or a frame that does not
   * decode) is counted in `controlListenerFailures`. A `RESPONSE_READY` is not one of them: it
   * answers a `queryTrackResponse`, which reads it (#1579a). Returns the unsubscribe function.
   */
  onEvent(listener: (event: ProtocolEvent) => void): () => void {
    if (typeof listener !== "function") {
      throw new MisoUsageError("an event listener must be a function");
    }
    return this.#boundary.onControlEvent((event) => {
      if (isResponseReady(event.frame)) return;
      listener(decodeProtocolEvent(event.frame));
    });
  }

  /**
   * Apply one structural transaction (`transaction(base)...build()`) at the committed revision, and
   * resolve with the revision it committed and the path the core chose: `live` (the running plan
   * ramps to the new values), `model_only` (nothing to render differently) or `rebuild` (a successor
   * plan, adopted at the next `render()`, with every unchanged source and node playing on). A
   * refusal rejects with `MisoControlRefusal`, and nothing is committed. The call is synchronous in
   * the core; the promise is settled before control returns to the event loop.
   */
  apply(transaction: SessionTransaction): Promise<EditOutcome> {
    return this.#protocolClient().applyTransaction(transaction);
  }

  /**
   * Replace the session with `document`: the core diffs it against the committed model into one
   * transaction, so it is an edit, not a reboot (`loadSession`): sources keep playing and unchanged
   * nodes keep their state. Resolves as `apply` does, or with `unchanged` (no revision) when the
   * document equals the committed model. Live controls then resolve IDs against `document`.
   */
  async replaceSession(document: SessionDocument): Promise<EditOutcome> {
    // A copy: the digest is awaited, and a caller's array may change before the bytes are staged.
    const bytes = documentBytes(document).slice();
    const epoch = this.#sessionEpoch;
    const outcome = await this.#protocolClient().replaceSession(bytes);
    // The committed model is now `document`'s: live controls resolve against it. A `loadSession`
    // while the digest was awaited booted another session, which this replace does not describe.
    if (epoch === this.#sessionEpoch) {
      this.#session = builtSession(document);
      this.#booted = bytes;
    }
    return outcome;
  }

  /** The committed session revision: 0 at boot and after `loadSession`, +1 for each commit. */
  get revision(): bigint {
    return this.#boundary.controlRevision();
  }

  /** The applied-revision watermark, read now: which committed revision is audible, since which sample. */
  watermark(): WatermarkSnapshot {
    const snapshot = this.#boundary.readWatermark();
    this.#applied.settle(snapshot);
    return snapshot;
  }

  /**
   * The output latency and tail of the plan that renders now (#2116), read now: the words of the C
   * ABI's plan resource report, in samples at the session rate. While a rebuild's successor is
   * pending this is still the running plan's; the successor shows from the first read after the
   * `render()` that adopts it. A disposed or failed engine rejects.
   */
  async planLatency(): Promise<PlanLatency> {
    return this.#boundary.readPlanLatency();
  }

  /**
   * Resolve with the first watermark that covers `revision`: the revision is in effect, with every
   * revision before it, from the snapshot's `sample`. The watermark is read after each `render()`,
   * so this stays pending while the caller renders nothing (D15-17). A revision above the committed
   * one rejects with `MisoUsageError`, and so does every pending one at `loadSession` or `dispose`.
   */
  applied(revision: bigint): Promise<WatermarkSnapshot> {
    try {
      if (typeof revision !== "bigint" || revision < 0n) {
        throw new MisoUsageError("applied() takes a non-negative bigint revision");
      }
      // A disposed engine throws here: every error is a rejection, never a synchronous throw.
      const committed = this.revision;
      if (revision > committed) {
        throw new MisoUsageError(`revision ${revision} is not committed; the committed revision is ${committed}`);
      }
      this.watermark();
      return this.#applied.applied(revision);
    } catch (error: unknown) {
      return Promise.reject(error);
    }
  }

  /**
   * Hand the transaction queue the core's committed revision right after a frame sent beside it
   * (#1389, root ruling (b)): a structural `apply` or a `replaceSession` would otherwise make the
   * queue's next `SET_CONTROL` expect an old revision and be refused. It runs in the same call as
   * the frame, so no queued transaction can be sent between the commit and the note.
   */
  #noteCommit(): void {
    if (this.#controlQueue === undefined || this.#boundary.disposed) return;
    let revision: bigint;
    try {
      revision = this.#boundary.controlRevision();
    } catch {
      // The core cannot be read: the queue's next transaction reports what is wrong.
      return;
    }
    this.#controlQueue.noteCommit(revision);
  }

  #protocolClient(): ProtocolClient {
    return this.#protocol ??= new ProtocolClient({
      send: (frame, document) => {
        try {
          return this.#boundary.controlFrame(frame, document);
        } finally {
          this.#noteCommit();
        }
      },
      // The one allocator of this endpoint, which the transaction queue and the control list share.
      requestIds: this.#boundary.controlRequestIds,
      // The core's own committed revision: the queue's, and also a commit another sender made.
      committedRevision: () => this.#boundary.controlRevision(),
    });
  }

  /** The number of control events dropped because no listener was subscribed. */
  get discardedControlEvents(): number {
    return this.#boundary.discardedControlEvents;
  }

  /** The number of times a control event listener threw; the throw is reported outside the control call. */
  get controlListenerFailures(): number {
    return this.#boundary.controlListenerFailures;
  }

  nextAbsoluteSample(): bigint {
    return this.#boundary.nextAbsoluteSample();
  }

  renderedQuanta(): bigint {
    return this.#boundary.renderedQuanta();
  }

  resources(): Readonly<Record<string, number | bigint | readonly number[]>> {
    return this.#boundary.resources();
  }

  /** The exact bytes this engine wrote to the boot options block. */
  get optionBytes(): Uint8Array {
    return this.#boundary.optionBytes;
  }

  submitSource(request: {
    readonly sourceId: string;
    readonly generation: bigint;
    readonly startFrame: bigint;
    readonly planes: readonly Float32Array[];
    readonly endOfRegion: boolean;
  }): EngineCallResult {
    const result = this.#boundary.submitSource(request);
    if (result.ok) this.#noteGeneration(request.generation);
    return result;
  }

  seekSource(request: {
    readonly sourceId: string;
    readonly generation: bigint;
    readonly sourceFrame: bigint;
  }): EngineCallResult {
    const result = this.#boundary.seekSource(request);
    if (result.ok) this.#noteGeneration(request.generation);
    return result;
  }

  /**
   * Seek the timeline and every source to `frame` under one new generation, all or nothing, and
   * resolve with the generation used. The headless engine has no live audio to keep, so every
   * consumer lands the seek at the next rendered block (#2205: no runway wait, no old audio after
   * it); the caller submits each source's PCM from `min(frame, source frames)` under that
   * generation before that block. Until it does, a source underruns. The generation is `options.generation`, else one above the
   * largest this object has passed through `seek`, `seekSource` or `submitSource`. A stale
   * generation is refused (`source.generation.stale`, result `invalidArgument`) and nothing
   * changes. The engine declares the discontinuity in the same call (the floors a rebuild kept
   * reset), and a refused seek declares nothing, as on the C ABI.
   */
  seek(
    frame: bigint,
    options?: { readonly generation?: bigint },
  ): { readonly generation: bigint } & EngineCallResult {
    const generation = options?.generation ?? this.#highestGeneration + 1n;
    const result = this.#boundary.sessionSeek({ generation, frame });
    if (result.ok) this.#noteGeneration(generation);
    return Object.freeze({ generation, ...result });
  }

  #noteGeneration(generation: bigint): void {
    if (generation > this.#highestGeneration) this.#highestGeneration = generation;
  }

  /** Take or release the decimated meter lease. */
  meters(enabled: boolean): EngineCallResult {
    return this.#boundary.meterLease(enabled);
  }

  /** Drain one completed meter frame, if a full window is ready. */
  pollMeters(): MeterFrame | undefined {
    return this.#boundary.pollMeters();
  }

  render(actualFrames?: number): { readonly left: Float32Array; readonly right: Float32Array } {
    const output = this.#boundary.render(actualFrames ?? this.status().quantumFrames);
    // The block's control-event drain ran the core's service step; the watermark it published
    // settles every `applied` it covers (D3).
    this.watermark();
    return output;
  }

  /**
   * The supported headless mix switch: dispose this session and boot another on the same instance.
   *
   * `dispose()` + `OfflineEngine.create()` is *not* the mix switch and is not documented as one --
   * it makes a fresh instance with a fresh linear memory. Issue #243 S2(a) asks for one exposed
   * reboot verb; this is it.
   */
  loadSession(document: SessionDocument, options: BootOptions = {}): void {
    this.#sessionEpoch += 1;
    this.#observationSubscriptions?.invalidate();
    this.#loudness?.invalidate();
    // Revisions restart with the new session: a pending revision of the old one never applies.
    this.#applied.rejectAll(new MisoUsageError(
      "loadSession() rebooted the engine: the pending revision belongs to the replaced session",
    ));
    // Cleared first, so a refused reboot cannot leave IDs resolving against the replaced session.
    this.#session = undefined;
    this.#booted = undefined;
    const bytes = documentBytes(document);
    // The queue starts over at the revision the core reports for the new session, never at an
    // assumed `0`. A refused reboot disposed the old session, so the queue only drops what was
    // staged for it; its next send finds a disposed engine and refuses.
    let revision = 0n;
    try {
      this.#boundary.reboot(bytes, options);
      revision = this.#boundary.controlRevision();
    } finally {
      this.#controlQueue?.reset(revision);
    }
    this.#session = builtSession(document);
    this.#booted = bytes.slice();
  }

  dispose(): void {
    this.#applied.rejectAll(new MisoUsageError("this engine was disposed"));
    this.#controlQueue?.close();
    this.#observationSubscriptions?.invalidate(true);
    this.#loudness?.invalidate(true);
    this.#boundary.dispose();
  }

  #observationOwner(): ObservationSubscriptionOwner {
    return this.#observationSubscriptions ??= new ObservationSubscriptionOwner({
      observationMap: () => this.observationMap(),
      // The core's own frames through the module's control entry (#1589 D3): `controlFrame` notes
      // the revision for the transaction queue, and the endpoint's one allocator numbers them.
      ...observationFrameTransport({
        send: (frame) => this.controlFrame(frame),
        requestIds: this.#boundary.controlRequestIds,
        committedRevision: () => this.#boundary.controlRevision(),
        maximumFrameBytes: () => this.controlTransactions().maximumFrameBytes,
      }),
      sampleRateHz: () => this.#boundary.status().sampleRateHz,
      observationEnded: (listener) => this.#boundary.onControlEvent((event) => {
        // The reliable lane carries it; the core decides what ended, the SDK only reads the frame.
        const ended = event.lane === 0 ? decodeObservationEnded(event.frame) : undefined;
        if (ended !== undefined) listener(ended);
      }),
      spectrumPrepared: () => this.#boundary.preparedSpectrumQuery(),
      spectrumPreparedCollection: () => this.#boundary.preparedSpectrumCollection(),
      spectrumSelect: (query) => this.#boundary.selectSpectrum(query),
      spectrumStreamSelect: (query, smoothingMs) =>
        this.#boundary.selectSpectrumStream(query, smoothingMs),
      // A read is one query (it settles after the app's next `render()`); the owner publishes it
      // only when a member field or a curve bit changed.
      responseRead: (request: TrackResponseQuery) => this.#boundary.queryTrackResponse(request),
      spectrumStart: (smoothingMs, query) => this.#boundary.startSpectrumStream(smoothingMs, query),
      spectrumRead: (query) => this.#boundary.readSpectrumStream(query),
      spectrumStop: () => this.#boundary.stopSpectrumStream(),
    }, this.#observationLimits, this.#responseLimits, this.#spectrumLimits);
  }
}

/** Boot a headless engine. */
export async function createOfflineEngine(
  document: SessionDocument,
  options: OfflineEngineOptions = {},
): Promise<OfflineEngine> {
  return OfflineEngine.create(document, options);
}

export type ValidationResult =
  | { readonly ok: true; readonly shape: SessionShape }
  | {
    readonly ok: false;
    readonly phase: ErrorPhase;
    readonly code: MisoErrorCode;
    /** The engine result code; `undefined` when the SDK raised `code` without one. */
    readonly result: number | undefined;
    readonly diagnostics: readonly MisoDiagnostic[];
  };

/**
 * Validate a document by booting it and throwing the result away.
 *
 * # Why this is the same verb rather than a parser
 *
 * There is no JSON parser in this SDK and there never will be (ruling 5438024085): a second
 * implementation of the grammar would answer a question the engine did not ask. `validate()` runs
 * the real engine, so its diagnostics *are* the engine's diagnostics and its budget checks are the
 * real ones, under the real physics gate.
 *
 * # The unbounded-admission bug this fixes
 *
 * The pre-boot-v1 `validateSession` sized its staging from the input -- `max(1 MiB, input.length)`
 * -- so handing it a 64 MiB document made it allocate 64 MiB to discover the document was too
 * large. Boot v1's `document_ptr` refuses an oversize length *before any allocation*, and this
 * function is a thin wrapper over that, so admission of an over-maximum document now costs no
 * staging at all (issue #243 eval 3, and S1's "standing unbounded-admission bug").
 *
 * A refusal is returned, not thrown: validation's whole job is to produce an answer about a
 * document, and a refusal is the most useful answer it has.
 */
export async function validate(
  document: SessionDocument,
  options: OfflineEngineOptions = {},
): Promise<ValidationResult> {
  let engine: OfflineEngine | undefined;
  try {
    engine = await OfflineEngine.create(document, options);
    return Object.freeze({ ok: true as const, shape: engine.shape() });
  } catch (error) {
    if (error instanceof MisoEngineError) {
      return Object.freeze({
        ok: false as const,
        phase: error.phase,
        code: error.code,
        result: error.result,
        diagnostics: error.diagnostics,
      });
    }
    throw error;
  } finally {
    engine?.dispose();
  }
}

