import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { MisoEngineError, MisoUsageError, resultName } from "../core/errors.ts";
import {
  MSB1_CONTROL,
  MSB1_CONTROL_BYTES,
  MSB1_CONTROL_I64_OFFSET,
  createMsb1Ring,
  Msb1RingObserver,
  Msb1RingWriter,
} from "./pcm-ring.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";

export type PcmFeedOperation = "moduleLoad" | "nodeCreate" | "attachPost" | "readyTimeout" | "closed"
  | "prepareState" | "prepareBusy" | "preparePost" | "prepareTimeout" | "prepareSuperseded" | "prepareRefused"
  | "detachPost" | "detachTimeout" | "detachRefused";

export class PcmFeedError extends Error {
  readonly operation: PcmFeedOperation;
  readonly result: number | undefined;
  constructor(operation: PcmFeedOperation, message: string, cause?: unknown, result?: number) {
    super(message, { cause }); this.name = "PcmFeedError"; this.operation = operation; this.result = result;
  }
}

export type PcmRunwayReason = "mismatch" | "timeout";

/**
 * A codec-neutral source runway could not be proved.
 *
 * The bounded observer algorithm is intentionally kept beside the feed rather than in a source
 * adapter. Its useful generic provenance is `engine-web-adapter` commit
 * `f833303f146de7cbe1705fe88ae68a6d6e0d4e45`, `src/session.ts::waitForRunway`; this SDK helper
 * owns the ring proof only and does not own source I/O, seeking, or audio-context lifecycle.
 */
export class PcmRunwayError extends Error {
  readonly reason: PcmRunwayReason;
  readonly sourceId: string | undefined;

  constructor(reason: PcmRunwayReason, message: string, sourceId?: string) {
    super(message);
    this.name = "PcmRunwayError";
    this.reason = reason;
    this.sourceId = sourceId;
  }
}

export interface PcmRunwaySource {
  readonly sourceId: string;
  readonly frames: bigint;
  readonly ring: SharedArrayBuffer;
}

export interface WaitForPcmRunwayOptions {
  readonly sources: readonly PcmRunwaySource[];
  readonly targetFrame: bigint;
  readonly generation: bigint;
  readonly timeoutMs: number;
  readonly minimumFrames?: bigint;
  readonly signal?: AbortSignal;
}

export interface FeedPort { postMessage(message: unknown): void; onmessage?: ((event: MessageEvent) => void) | null }
export interface FeedNode { readonly port: FeedPort; disconnect(): void }
export interface FeedContext { readonly audioWorklet: { addModule(url: string): Promise<void> }; readonly state?: string }
export interface FeedNodeOptions { readonly numberOfInputs: number; readonly numberOfOutputs: number }
/** The anchored seek that starts an added source (issue 1297b D4). */
export interface FeedStartSeek {
  readonly sourceId: string;
  /** The new generation the source and its ring move to. */
  readonly generation: bigint;
  /** The source frame the stem plays at `anchorSample`: `frameAt(anchorSample)`'s answer. */
  readonly frame: bigint;
  /** The absolute render sample (a quantum multiple) of the block the stem starts in. */
  readonly anchorSample: bigint;
}
/**
 * How an added source starts in time with the playing stems (issue 1297b D4/D5). The engine has
 * no playhead: `frameAt(anchorSample)` is the app's mapping from a render sample to the source's
 * frame. `prime(seek)` writes the source's PCM into its ring from `seek.frame` at
 * `seek.generation` with the app's own `Msb1RingWriter`; it must not seek the ring, because the
 * SDK has already published the anchored seek in it.
 */
export interface FeedStart {
  frameAt(anchorSample: bigint): bigint;
  prime(seek: FeedStartSeek): void | Promise<void>;
}
export interface FeedSource { readonly sourceId: string; readonly channels: 1 | 2; readonly start?: FeedStart }
/** The engine a feed belongs to: what `createEngine` resolves with. */
export interface FeedEngine { readonly host: object }
export interface FeedOptions<Context extends FeedContext = FeedContext> {
  readonly context: Context;
  readonly sources: readonly FeedSource[];
  /**
   * The engine this feed delivers to (issue 1297b). With it, `ready()` starts each source that
   * has a `start` with one anchored seek, and a committed `engine.apply` or
   * `engine.replaceSession` that removes a source of this feed detaches that source's ring once
   * `engine.applied(revision)` resolved. A source with a `start` requires it.
   */
  readonly engine?: FeedEngine;
  readonly quantumFrames: number;
  readonly capacityChunks?: number;
  readonly moduleUrl?: string | URL;
  readonly createNode?: (context: Context, name: string, options: FeedNodeOptions) => FeedNode;
}
export interface EngineFeed {
  /** The rings the feed still delivers, in source order: `detachSource` removes one. */
  readonly rings: readonly SharedArrayBuffer[];
  readonly state: "pending" | "active" | "closed";
  /**
   * Resolve once the worklet bound every ring; then start each source that has a `start` (issue
   * 1297b D4), in order. A start reads `nextAbsoluteSample` once, sets the anchor `A` to it plus the
   * ring's capacity in frames, rounded up to a quantum, asks `frameAt(A)` once, publishes the
   * anchored seek in the ring, awaits `prime`, and then sends the same seek to the engine's message
   * handler (`miso.seek-at.v1`), which applies it once. Until it did, the drain reads none of that
   * ring's chunks. An anchor the audio reaches before the handler is late, not lost: the engine
   * starts the stem at the frame plus the lateness and reports it in the source's seek report. A
   * refused seek rejects with a `MisoEngineError` that carries the result, and closes the feed.
   */
  ready(options?: { readonly timeoutMs?: number; readonly now?: () => number; readonly wait?: (ms: number) => Promise<void> }): Promise<void>;
  /** Prepare the producer's published seeks while suspended, freeing stale consumer slots.
   * Await the producer's seek acknowledgement before calling, and serialize this handoff
   * against other producer seek commands. The snapshot/post occurs before the first await;
   * a later seek may supersede the pending request, but an in-progress publication is not input.
   * Does not render or supply PCM. Refill after this resolves and before resuming.
   * One operation may be outstanding; a newer published seek rejects the old proof.
   * Timeout closes the feed, preventing an unobserved late acknowledgement from granting readiness. */
  prepareSeek(options?: { readonly timeoutMs?: number }): Promise<void>;
  /**
   * Withdraw one source's ring (issue 1297b D5): resolves after the worklet's drain stopped reading
   * it, then releases it. The other rings keep draining. A worklet that withdrew no ring rejects
   * with `detachRefused`; it and a timeout close the feed, which withdraws every ring.
   */
  detachSource(sourceId: string, options?: { readonly timeoutMs?: number }): Promise<void>;
  close(): void;
}

/** The engine side of the feeds linked to one engine (issue 1297b). */
interface FeedLink {
  readonly host: Pick<MisoAudioWorkletHost, "status" | "seekSourceAt">;
  readonly reserveSeekGeneration: () => bigint;
  readonly feeds: Set<LinkedFeed>;
}
interface LinkedFeed {
  /** The source IDs whose rings the feed still delivers. */
  sourceIds(): readonly string[];
  detachSource(sourceId: string): Promise<void>;
}
/** Keyed by the engine's host object, which `FeedEngine.host` names. */
const feedLinks = new WeakMap<object, FeedLink>();

/** What `createEngine` tells the feeds linked to it after a commit (issue 1297b D6). */
export interface FeedRetirement {
  /** After a commit that removed `sourceIds`, detach each linked feed's ring of one once `applied` resolved. */
  retire(sourceIds: readonly string[], applied: () => Promise<unknown>): void;
  /** After a replace: `committedSources` reads the committed session's source IDs; detach the rest. */
  retireAbsent(committedSources: () => Promise<ReadonlySet<string>>, applied: () => Promise<unknown>): void;
}

/**
 * @internal `createEngine` links its host here (issue 1297b): a feed whose `engine` names it
 * starts its added sources with generations this engine reserved, and has its removed sources
 * retired by `FeedRetirement`. The SDK's own seam; not an application API.
 */
export function linkFeedEngine(
  host: Pick<MisoAudioWorkletHost, "status" | "seekSourceAt">,
  reserveSeekGeneration: () => bigint,
): FeedRetirement {
  const link: FeedLink = { host, reserveSeekGeneration, feeds: new Set() };
  feedLinks.set(host, link);
  const detachAfter = (owned: readonly (readonly [LinkedFeed, string])[], applied: () => Promise<unknown>): void => {
    if (owned.length === 0) return;
    // Keep delivering until the removal is in effect (#1325: a removal's revision completes at its
    // phase-2 swap), so the strip's ramp-out is never starved. A rejected `applied` (a closed
    // engine) detaches nothing: closing the feed is the app's.
    void applied().then(
      () => Promise.allSettled(owned.map(([feed, sourceId]) => feed.detachSource(sourceId))),
      () => undefined,
    );
  };
  const owners = (removed: (sourceId: string) => boolean) => {
    const owned: (readonly [LinkedFeed, string])[] = [];
    for (const feed of link.feeds) {
      for (const sourceId of feed.sourceIds()) if (removed(sourceId)) owned.push([feed, sourceId]);
    }
    return owned;
  };
  return {
    retire(sourceIds, applied) {
      detachAfter(owners((sourceId) => sourceIds.includes(sourceId)), applied);
    },
    retireAbsent(committedSources, applied) {
      if (link.feeds.size === 0) return;
      void committedSources().then(
        (present) => detachAfter(owners((sourceId) => !present.has(sourceId)), applied),
        () => undefined,
      );
    },
  };
}

export async function prepareEngineFeed(context: FeedContext, moduleUrl: string | URL = BUNDLED_ENGINE_ASSETS.pcmFeedWorklet): Promise<void> {
  try { await context.audioWorklet.addModule(String(moduleUrl)); }
  catch (error) { throw new PcmFeedError("moduleLoad", "PCM feed worklet prelude could not load", error); }
}

const MAX_I64 = (1n << 63n) - 1n;

interface PcmRunwayPending {
  readonly sourceId: string;
  readonly total: bigint;
  readonly end: bigint;
  next: bigint;
}

/**
 * Prove that every source has published a contiguous PCM runway at one acknowledged generation.
 *
 * The caller is responsible for serializing producer seeks, receiving the producer acknowledgement,
 * and completing `EngineFeed.prepareSeek()` before invoking this function. This function only
 * observes existing MSB1 rings and never advances a consumer index or changes producer state.
 */
export async function waitForPcmRunway(options: WaitForPcmRunwayOptions): Promise<void> {
  const snapshot = snapshotRunwayOptions(options);
  const observers = new Map<string, Msb1RingObserver>();
  const pending = new Map<string, PcmRunwayPending>();
  const started = performance.now();

  try {
    for (const source of snapshot.sources) {
      let observer: Msb1RingObserver;
      try {
        observer = new Msb1RingObserver(source.ring);
      } catch (cause) {
        throw new MisoUsageError(`source ${source.sourceId} has an invalid MSB1 ring${cause instanceof Error ? `: ${cause.message}` : ""}`);
      }

      // Bind every supplied ring before waiting, including a source already at EOF. Only sources
      // that need data retain an observer for the duration of this call.
      const control = new Int32Array(source.ring, 0, MSB1_CONTROL_BYTES / 4);
      const capacity = BigInt(Atomics.load(control, MSB1_CONTROL.CAPACITY));
      const frameCapacity = BigInt(observer.frameCapacity);
      if (snapshot.targetFrame < source.frames) {
        const available = frameCapacity * capacity;
        if (snapshot.minimumFrames !== undefined && snapshot.minimumFrames > available) {
          observer.close();
          throw new MisoUsageError(`minimumFrames exceeds source ${source.sourceId} ring capacity`);
        }
        const requested = snapshot.minimumFrames ?? available;
        const end = snapshot.targetFrame + requested < source.frames
          ? snapshot.targetFrame + requested
          : source.frames;
        observers.set(source.sourceId, observer);
        pending.set(source.sourceId, {
          sourceId: source.sourceId,
          total: source.frames,
          end,
          next: snapshot.targetFrame,
        });
      } else {
        observer.close();
      }
    }

    throwIfRunwayAborted(snapshot.signal);
    if (pending.size === 0) return;

    for (;;) {
      throwIfRunwayAborted(snapshot.signal);
      for (const [sourceId, expected] of pending) {
        // A previous source callback may have completed this source's requirement while the map
        // was being iterated. The map iterator normally skips deleted entries, but this explicit
        // check keeps the proof robust if a callback is instrumented by a caller.
        if (!pending.has(sourceId)) continue;
        observers.get(sourceId)!.pull((chunk) => {
          if (chunk.generation !== snapshot.generation ||
              chunk.startFrame !== expected.next ||
              chunk.startFrame + BigInt(chunk.frames) > expected.total) {
            throw new PcmRunwayError(
              "mismatch",
              "PCM runway is not contiguous at the acknowledged generation and frame",
              expected.sourceId,
            );
          }
          expected.next += BigInt(chunk.frames);
          if (expected.next >= expected.end) pending.delete(sourceId);
        }, 32);
      }
      if (pending.size === 0) return;
      if (performance.now() - started >= snapshot.timeoutMs) {
        throw new PcmRunwayError("timeout", "PCM runway did not arrive before the deadline");
      }
      await yieldRunwayTurn(snapshot.signal);
    }
  } finally {
    for (const observer of observers.values()) observer.close();
  }
}

interface PcmRunwaySnapshot {
  readonly sources: readonly PcmRunwaySource[];
  readonly targetFrame: bigint;
  readonly generation: bigint;
  readonly timeoutMs: number;
  readonly minimumFrames: bigint | undefined;
  readonly signal: AbortSignal | undefined;
}

function snapshotRunwayOptions(options: WaitForPcmRunwayOptions): PcmRunwaySnapshot {
  if (options === null || typeof options !== "object") throw new MisoUsageError("PCM runway options are required");
  const input = options as unknown as Record<string, unknown>;
  if (!Array.isArray(input.sources)) throw new MisoUsageError("PCM runway sources must be an array");
  const targetFrame = input.targetFrame;
  if (!nonnegativeI64(targetFrame)) throw new MisoUsageError("targetFrame must be a nonnegative bigint within the MSB1 frame domain");
  const generation = input.generation;
  if (!positiveI64(generation)) throw new MisoUsageError("generation must be a positive bigint within the MSB1 generation domain");
  const timeoutMs = input.timeoutMs;
  if (typeof timeoutMs !== "number" || !Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    throw new MisoUsageError("timeoutMs must be finite and positive");
  }
  const minimumValue = input.minimumFrames;
  if (minimumValue !== undefined && !positiveI64(minimumValue)) {
    throw new MisoUsageError("minimumFrames must be a positive bigint within the MSB1 frame domain");
  }
  const signal = input.signal;
  if (signal !== undefined && !isAbortSignal(signal)) throw new MisoUsageError("signal must be an AbortSignal");

  const ids = new Set<string>();
  const sources = input.sources.map((candidate, index): PcmRunwaySource => {
    if (candidate === null || typeof candidate !== "object") throw new MisoUsageError(`PCM runway source ${index} is invalid`);
    const source = candidate as Record<string, unknown>;
    const sourceId = source.sourceId;
    if (typeof sourceId !== "string" || sourceId.length === 0) {
      throw new MisoUsageError(`PCM runway source ${index} requires a nonempty sourceId`);
    }
    if (ids.has(sourceId)) throw new MisoUsageError(`PCM runway source IDs must be unique: ${sourceId}`);
    ids.add(sourceId);
    const frames = source.frames;
    if (!nonnegativeI64(frames)) throw new MisoUsageError(`PCM runway source ${sourceId} frames must be a nonnegative bigint within the MSB1 frame domain`);
    const ring = source.ring;
    if (typeof SharedArrayBuffer === "undefined" || !(ring instanceof SharedArrayBuffer)) {
      throw new MisoUsageError(`PCM runway source ${sourceId} ring must be a SharedArrayBuffer`);
    }
    return Object.freeze({ sourceId, frames, ring });
  });

  return Object.freeze({
    sources: Object.freeze(sources),
    targetFrame,
    generation,
    timeoutMs,
    minimumFrames: minimumValue,
    signal: signal as AbortSignal | undefined,
  });
}

function nonnegativeI64(value: unknown): value is bigint {
  return typeof value === "bigint" && value >= 0n && value <= MAX_I64;
}

function positiveI64(value: unknown): value is bigint {
  return typeof value === "bigint" && value > 0n && value <= MAX_I64;
}

function isAbortSignal(value: unknown): value is AbortSignal {
  return typeof value === "object" && value !== null &&
    typeof (value as { readonly aborted?: unknown }).aborted === "boolean" &&
    typeof (value as { readonly addEventListener?: unknown }).addEventListener === "function" &&
    typeof (value as { readonly removeEventListener?: unknown }).removeEventListener === "function";
}

function abortReason(signal: AbortSignal): unknown {
  return signal.reason;
}

function throwIfRunwayAborted(signal: AbortSignal | undefined): void {
  if (signal?.aborted) throw abortReason(signal);
}

function yieldRunwayTurn(signal: AbortSignal | undefined): Promise<void> {
  return new Promise<void>((resolve, reject) => {
    let settled = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const finish = (error?: unknown): void => {
      if (settled) return;
      settled = true;
      if (timer !== undefined) clearTimeout(timer);
      if (signal !== undefined) signal.removeEventListener("abort", onAbort);
      if (error === undefined) resolve(); else reject(error);
    };
    const onAbort = (): void => finish(abortReason(signal!));
    if (signal !== undefined) {
      signal.addEventListener("abort", onAbort, { once: true });
      if (signal.aborted) { finish(abortReason(signal)); return; }
    }
    timer = setTimeout(() => finish(), 0);
  });
}

export function attachEngineFeed<Context extends FeedContext>(options: FeedOptions<Context>): EngineFeed {
  if (!Number.isSafeInteger(options.quantumFrames) || options.quantumFrames <= 0) throw new MisoUsageError("quantumFrames must be a positive integer");
  const capacity = options.capacityChunks ?? 64;
  const ids = new Set<string>();
  for (const source of options.sources) {
    if (ids.has(source.sourceId)) throw new MisoUsageError(`feed source IDs must be unique: ${source.sourceId}`);
    ids.add(source.sourceId);
  }
  const starts = options.sources.map((source) => source.start);
  if (starts.some((start) => start !== undefined && (typeof start?.frameAt !== "function" || typeof start.prime !== "function"))) {
    throw new MisoUsageError("a feed source start needs frameAt and prime functions");
  }
  const link = options.engine === undefined ? undefined : feedLinks.get(options.engine.host);
  if (options.engine !== undefined && link === undefined) throw new MisoUsageError("the feed's engine is not one createEngine resolved with");
  if (link === undefined && starts.some((start) => start !== undefined)) throw new MisoUsageError("a feed source start needs the feed's engine");
  const created = options.sources.map((source) => createMsb1Ring({ sourceId: source.sourceId, channels: source.channels, frameCapacity: options.quantumFrames, capacity }));
  // The rings still delivered, by source ID, with each one's index in the attach list: a port
  // clones a SharedArrayBuffer into a new object, so the worklet is told the index, never the ring.
  const live = new Map(options.sources.map((source, index) => [source.sourceId, { ring: created[index]!, index }] as const));
  const liveRings = (): readonly SharedArrayBuffer[] => Object.freeze([...live.values()].map((entry) => entry.ring));
  let rings = liveRings();
  let node: FeedNode;
  try {
    node = options.createNode?.(options.context, "miso-sab-feed-attach", { numberOfInputs: 0, numberOfOutputs: 1 }) ?? defaultNode(options.context);
  } catch (error) {
    release(created); throw new PcmFeedError("nodeCreate", "Engine feed attach processor is unavailable", error);
  }
  let state: "pending" | "active" | "closed" = created.length === 0 ? "active" : "pending";
  let signalTerminal!: () => void;
  const terminal = new Promise<void>((resolve) => { signalTerminal = resolve; });
  let terminalSignaled = false;
  let requestId = 0;
  let pending: { id: number; seeks: SeekSnapshot[]; finish: (error?: PcmFeedError) => void } | undefined;
  const detaching = new Map<number, (error?: PcmFeedError) => void>();
  // Issue #1297b D4: the starts not yet run, and the one `ready()` that runs them.
  let unstarted = options.sources.flatMap((source, index) => source.start === undefined ? [] : [{ source, start: source.start, ring: created[index]! }]);
  let starting: Promise<void> | undefined;
  node.port.onmessage = ({ data }: MessageEvent): void => {
    if (data?.op === "source-detached") {
      detaching.get(data.requestId)?.(data.detached === true ? undefined : new PcmFeedError("detachRefused", "PCM feed worklet withdrew no ring for the source"));
      return;
    }
    if (data?.op !== "seek-prepared" || data.requestId !== pending?.id || pending === undefined) return;
    if (options.context.state !== "suspended") {
      pending.finish(new PcmFeedError("prepareState", "AudioContext resumed during seek preparation"));
    } else if (data.kind === "superseded" || !sameSeeks(rings, pending.seeks)) {
      pending.finish(new PcmFeedError("prepareSuperseded", "PCM seek changed during preparation"));
    } else if (data.kind !== "confirmed" || !Array.isArray(data.seeks) || !equalSeeks(data.seeks, pending.seeks)) {
      pending.finish(new PcmFeedError("prepareRefused", "PCM consumer refused seek preparation", undefined, typeof data.result === "number" ? data.result : undefined));
    } else {
      pending.finish();
    }
  };
  const linked: LinkedFeed = {
    sourceIds: () => [...live.keys()],
    detachSource: (sourceId) => feed.detachSource(sourceId),
  };
  const close = (): void => {
    if (state === "closed") return;
    state = "closed"; release(created);
    link?.feeds.delete(linked);
    pending?.finish(new PcmFeedError("closed", "Engine feed is closed"));
    for (const finish of [...detaching.values()]) finish(new PcmFeedError("closed", "Engine feed is closed"));
    node.port.onmessage = null;
    if (!terminalSignaled) { terminalSignaled = true; signalTerminal(); }
    try { node.port.postMessage({ op: "detach" }); } catch { /* context already closed */ }
    try { node.disconnect(); } catch { /* never connected */ }
  };
  try { node.port.postMessage({ op: "attach", rings: created }); }
  catch (error) { close(); throw new PcmFeedError("attachPost", "Engine feed attach message could not be posted", error); }
  link?.feeds.add(linked);
  /** One anchored start (D4): the SDK publishes the seek in the ring once, and the engine's handler applies it once. */
  const startSource = async (host: FeedLink["host"], reserve: () => bigint, entry: { source: FeedSource; start: FeedStart; ring: SharedArrayBuffer }): Promise<void> => {
    const quantum = BigInt(options.quantumFrames);
    const status = await host.status();
    if (status.result !== 0) throw refusedStart(entry.source.sourceId, status.result);
    const earliest = status.nextAbsoluteSample + BigInt(capacity) * quantum;
    const anchorSample = (earliest + quantum - 1n) / quantum * quantum;
    const frame = entry.start.frameAt(anchorSample);
    if (!nonnegativeI64(frame)) throw new MisoUsageError(`frameAt for source ${entry.source.sourceId} must return a nonnegative bigint within the MSB1 frame domain`);
    const generation = reserve();
    // The ring's seek is published first: from here the drain reads none of its chunks until the
    // handler applied this seek, so the primed PCM at the new generation waits for it.
    new Msb1RingWriter(entry.ring).seek(generation, frame, anchorSample);
    const seek = Object.freeze({ sourceId: entry.source.sourceId, generation, frame, anchorSample });
    await entry.start.prime(seek);
    if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
    let result: number;
    try {
      result = (await host.seekSourceAt({ sourceId: seek.sourceId, generation, sourceFrame: frame, anchorSample })).result;
    } catch (error) {
      const code = (error as { readonly result?: unknown }).result;
      if (typeof code === "number" && Number.isSafeInteger(code) && code >= 0) throw refusedStart(seek.sourceId, code);
      throw error;
    }
    if (result !== 0) throw refusedStart(seek.sourceId, result);
  };
  const feed: EngineFeed = {
    get rings() { return rings; },
    get state() { return state; },
    async ready(settings = {}): Promise<void> {
      if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
      if (state === "pending") {
        const now = settings.now ?? (() => performance.now());
        const wait = settings.wait ?? ((milliseconds: number) => new Promise<void>((resolve) => setTimeout(resolve, milliseconds)));
        const deadline = now() + (settings.timeoutMs ?? 2_000);
        while (state === "pending") {
          if (rings.every(attached)) { state = "active"; break; }
          if (now() >= deadline) { close(); throw new PcmFeedError("readyTimeout", "Engine feed attach confirmation timed out"); }
          await Promise.race([wait(0), terminal]);
        }
        if (state !== "active") throw new PcmFeedError("closed", "Engine feed is closed");
      }
      if (unstarted.length === 0) return;
      starting ??= (async () => {
        try {
          for (const entry of unstarted) {
            // A source detached while an earlier start was in flight is not started.
            if (live.get(entry.source.sourceId)?.ring !== entry.ring) continue;
            await startSource(link!.host, link!.reserveSeekGeneration, entry);
          }
          unstarted = [];
        } catch (error) {
          close();
          throw error;
        }
      })();
      return starting;
    },
    async prepareSeek(settings = {}): Promise<void> {
      if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
      if (options.context.state !== "suspended" || !rings.every(attached)) throw new PcmFeedError("prepareState", "PCM seek preparation requires an attached feed and suspended context");
      if (pending !== undefined) throw new PcmFeedError("prepareBusy", "PCM seek preparation is already pending");
      const timeoutMs = settings.timeoutMs ?? 2_000;
      if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) throw new MisoUsageError("prepareSeek timeoutMs must be finite and positive");
      const seeks = rings.map(seekSnapshot);
      if (seeks.some((seek) => seek.epoch === 0 || seek.generation === 0n) || !sameSeeks(rings, seeks)) throw new PcmFeedError("prepareSuperseded", "PCM seek is unpublished or changing");
      if (rings.length === 0) return;
      await new Promise<void>((resolve, reject) => {
        const id = ++requestId;
        const timer = setTimeout(() => {
          if (pending?.id !== id) return;
          pending.finish(new PcmFeedError("prepareTimeout", "PCM seek preparation timed out"));
          close();
        }, timeoutMs);
        pending = { id, seeks, finish(error) {
          clearTimeout(timer);
          pending = undefined;
          if (error === undefined) resolve(); else reject(error);
        } };
        try { node.port.postMessage({ op: "prepare-seek", requestId: id, seeks }); }
        catch (cause) { pending?.finish(new PcmFeedError("preparePost", "PCM seek preparation could not be posted", cause)); close(); }
      });
    },
    async detachSource(sourceId: string, settings = {}): Promise<void> {
      if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
      const entry = live.get(sourceId);
      if (entry === undefined) throw new MisoUsageError(`the feed delivers no source ${sourceId}`);
      const { ring, index } = entry;
      const timeoutMs = settings.timeoutMs ?? 2_000;
      if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) throw new MisoUsageError("detachSource timeoutMs must be finite and positive");
      // The ring leaves the delivered set now, so no prepareSeek or later retirement names it.
      live.delete(sourceId);
      rings = liveRings();
      unstarted = unstarted.filter((entry) => entry.source.sourceId !== sourceId);
      await new Promise<void>((resolve, reject) => {
        const id = ++requestId;
        const timer = setTimeout(() => {
          if (!detaching.has(id)) return;
          detaching.get(id)!(new PcmFeedError("detachTimeout", "PCM source detach confirmation timed out"));
          close();
        }, timeoutMs);
        detaching.set(id, (error) => {
          clearTimeout(timer);
          detaching.delete(id);
          if (error === undefined) { resolve(); return; }
          reject(error);
          // An unconfirmed withdrawal is not left to leak: closing withdraws every ring of the node.
          if (error.operation === "detachRefused") close();
        });
        try { node.port.postMessage({ op: "detach-source", requestId: id, index }); }
        catch (cause) { detaching.get(id)?.(new PcmFeedError("detachPost", "PCM source detach could not be posted", cause)); close(); }
      });
      // Confirmed: the drain no longer reads it, so the writer is released.
      release([ring]);
    },
    close,
  };
  return feed;
}

function refusedStart(sourceId: string, result: number): MisoEngineError {
  return new MisoEngineError(`the browser host refused the anchored start of source ${sourceId}`, {
    phase: "source",
    code: resultName(result, "call"),
    result,
  });
}

interface SeekSnapshot { readonly epoch: number; readonly generation: bigint; readonly frame: bigint }
function seekSnapshot(ring: SharedArrayBuffer): SeekSnapshot {
  const control = new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4);
  const wide = new BigInt64Array(ring, MSB1_CONTROL_I64_OFFSET, 3);
  return { epoch: Atomics.load(control, MSB1_CONTROL.SEEK_EPOCH), generation: Atomics.load(wide, 0), frame: Atomics.load(wide, 1) };
}
function equalSeeks(actual: readonly SeekSnapshot[], expected: readonly SeekSnapshot[]): boolean {
  return actual.length === expected.length && actual.every((seek, index) => seek?.epoch === expected[index]!.epoch && seek.generation === expected[index]!.generation && seek.frame === expected[index]!.frame);
}
function sameSeeks(rings: readonly SharedArrayBuffer[], expected: readonly SeekSnapshot[]): boolean {
  return equalSeeks(rings.map(seekSnapshot), expected);
}

function release(rings: readonly SharedArrayBuffer[]): void {
  for (const ring of rings) Atomics.store(new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4), MSB1_CONTROL.WRITER_STATE, 0);
}
function attached(ring: SharedArrayBuffer): boolean {
  return Atomics.load(new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4), MSB1_CONTROL.ATTACHED) === 1;
}
function defaultNode(context: FeedContext): FeedNode {
  const Constructor = (globalThis as unknown as { AudioWorkletNode?: new (context: unknown, name: string, options: FeedNodeOptions) => FeedNode }).AudioWorkletNode;
  if (Constructor === undefined) throw new Error("AudioWorkletNode is unavailable");
  return new Constructor(context, "miso-sab-feed-attach", { numberOfInputs: 0, numberOfOutputs: 1 });
}
