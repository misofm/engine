import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { MisoUsageError } from "../core/errors.ts";
import { MSB1_CONTROL, MSB1_CONTROL_BYTES, createMsb1Ring } from "./pcm-ring.ts";

export type PcmFeedOperation = "moduleLoad" | "nodeCreate" | "attachPost" | "readyTimeout" | "closed";

export class PcmFeedError extends Error {
  readonly operation: PcmFeedOperation;
  constructor(operation: PcmFeedOperation, message: string, cause?: unknown) {
    super(message, { cause }); this.name = "PcmFeedError"; this.operation = operation;
  }
}

export interface FeedPort { postMessage(message: unknown): void }
export interface FeedNode { readonly port: FeedPort; disconnect(): void }
export interface FeedContext { readonly audioWorklet: { addModule(url: string): Promise<void> } }
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
  const close = (): void => {
    if (state === "closed") return;
    state = "closed"; release(rings);
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
    close,
  };
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
