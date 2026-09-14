import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { MisoUsageError } from "../core/errors.ts";
import {
  MSB1_CONTROL,
  MSB1_CONTROL_BYTES,
  MSB1_CONTROL_I64_OFFSET,
  createMsb1Ring,
  Msb1RingObserver,
} from "./pcm-ring.ts";

export type PcmFeedOperation = "moduleLoad" | "nodeCreate" | "attachPost" | "readyTimeout" | "closed"
  | "prepareState" | "prepareBusy" | "preparePost" | "prepareTimeout" | "prepareSuperseded" | "prepareRefused";

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
export interface FeedSource { readonly sourceId: string; readonly channels: 1 | 2 }
export interface FeedOptions<Context extends FeedContext = FeedContext> {
  readonly context: Context;
  readonly sources: readonly FeedSource[];
  readonly quantumFrames: number;
  readonly capacityChunks?: number;
  readonly moduleUrl?: string | URL;
  readonly createNode?: (context: Context, name: string, options: FeedNodeOptions) => FeedNode;
}
export interface EngineFeed {
  readonly rings: readonly SharedArrayBuffer[];
  readonly state: "pending" | "active" | "closed";
  ready(options?: { readonly timeoutMs?: number; readonly now?: () => number; readonly wait?: (ms: number) => Promise<void> }): Promise<void>;
  /** Prepare the producer's published seeks while suspended, freeing stale consumer slots.
   * Await the producer's seek acknowledgement before calling, and serialize this handoff
   * against other producer seek commands. The snapshot/post occurs before the first await;
   * a later seek may supersede the pending request, but an in-progress publication is not input.
   * Does not render or supply PCM. Refill after this resolves and before resuming.
   * One operation may be outstanding; a newer published seek rejects the old proof.
   * Timeout closes the feed, preventing an unobserved late acknowledgement from granting readiness. */
  prepareSeek(options?: { readonly timeoutMs?: number }): Promise<void>;
  close(): void;
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
  const rings = options.sources.map((source) => createMsb1Ring({ sourceId: source.sourceId, channels: source.channels, frameCapacity: options.quantumFrames, capacity: options.capacityChunks ?? 64 }));
  let node: FeedNode;
  try {
    node = options.createNode?.(options.context, "miso-sab-feed-attach", { numberOfInputs: 0, numberOfOutputs: 1 }) ?? defaultNode(options.context);
  } catch (error) {
    release(rings); throw new PcmFeedError("nodeCreate", "Engine feed attach processor is unavailable", error);
  }
  let state: "pending" | "active" | "closed" = rings.length === 0 ? "active" : "pending";
  let signalTerminal!: () => void;
  const terminal = new Promise<void>((resolve) => { signalTerminal = resolve; });
  let terminalSignaled = false;
  let requestId = 0;
  let pending: { id: number; seeks: SeekSnapshot[]; finish: (error?: PcmFeedError) => void } | undefined;
  node.port.onmessage = ({ data }: MessageEvent): void => {
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
  const close = (): void => {
    if (state === "closed") return;
    state = "closed"; release(rings);
    pending?.finish(new PcmFeedError("closed", "Engine feed is closed"));
    node.port.onmessage = null;
    if (!terminalSignaled) { terminalSignaled = true; signalTerminal(); }
    try { node.port.postMessage({ op: "detach" }); } catch { /* context already closed */ }
    try { node.disconnect(); } catch { /* never connected */ }
  };
  try { node.port.postMessage({ op: "attach", rings }); }
  catch (error) { close(); throw new PcmFeedError("attachPost", "Engine feed attach message could not be posted", error); }
  return {
    rings,
    get state() { return state; },
    async ready(settings = {}): Promise<void> {
      if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
      if (state === "active") return;
      const now = settings.now ?? (() => performance.now());
      const wait = settings.wait ?? ((milliseconds: number) => new Promise<void>((resolve) => setTimeout(resolve, milliseconds)));
      const deadline = now() + (settings.timeoutMs ?? 2_000);
      while (state === "pending") {
        if (rings.every(attached)) { state = "active"; return; }
        if (now() >= deadline) { close(); throw new PcmFeedError("readyTimeout", "Engine feed attach confirmation timed out"); }
        await Promise.race([wait(0), terminal]);
      }
      if (state === "closed") throw new PcmFeedError("closed", "Engine feed is closed");
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
    close,
  };
}

interface SeekSnapshot { readonly epoch: number; readonly generation: bigint; readonly frame: bigint }
function seekSnapshot(ring: SharedArrayBuffer): SeekSnapshot {
  const control = new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4);
  const wide = new BigInt64Array(ring, MSB1_CONTROL_I64_OFFSET, 2);
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
