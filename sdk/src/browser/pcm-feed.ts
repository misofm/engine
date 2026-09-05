import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { MisoUsageError } from "../core/errors.ts";
import { MSB1_CONTROL, MSB1_CONTROL_BYTES, MSB1_CONTROL_I64_OFFSET, createMsb1Ring } from "./pcm-ring.ts";

export type PcmFeedOperation = "moduleLoad" | "nodeCreate" | "attachPost" | "readyTimeout" | "closed"
  | "prepareState" | "prepareBusy" | "preparePost" | "prepareTimeout" | "prepareSuperseded" | "prepareRefused";

export class PcmFeedError extends Error {
  readonly operation: PcmFeedOperation;
  readonly result: number | undefined;
  constructor(operation: PcmFeedOperation, message: string, cause?: unknown, result?: number) {
    super(message, { cause }); this.name = "PcmFeedError"; this.operation = operation; this.result = result;
  }
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
