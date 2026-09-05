import type { BootOptions } from "../core/abi.ts";
import type { SessionShape } from "../core/boundary.ts";
import { MisoUsageError } from "../core/errors.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { BrowserBootError } from "./default-host.ts";

export interface ScratchBootRequest {
  readonly type: "scratch"; readonly requestId: number; readonly moduleUrl: string;
  readonly document: Uint8Array; readonly options: BootOptions;
}
export type ScratchBootReply =
  | { readonly type: "worker-ready" }
  | { readonly type: "scratch-result"; readonly requestId: number; readonly ok: true; readonly shape: SessionShape }
  | { readonly type: "scratch-result"; readonly requestId: number; readonly ok: false; readonly error: { readonly name: string; readonly message: string } };

/** Only the Worker operations used by a one-shot scratch boot. */
export interface ScratchWorker {
  postMessage(message: ScratchBootRequest): void;
  terminate(): void;
  addEventListener(type: "message" | "error" | "messageerror", listener: (event: any) => void): void;
  removeEventListener(type: "message" | "error" | "messageerror", listener: (event: any) => void): void;
}
export type ScratchWorkerFactory = (url: URL, options: { readonly type: "module" }) => ScratchWorker;

/** Boot once on a bounded module Worker, terminating before either outcome becomes observable. */
export async function scratchBootWithWorker(options: {
  readonly document: Uint8Array;
  readonly options: BootOptions;
  readonly moduleUrl: string | URL;
  readonly scratchWorkerModuleUrl?: string;
  readonly createWorker?: ScratchWorkerFactory;
  readonly requestDeadlineMs?: number;
  readonly signal?: AbortSignal;
}): Promise<SessionShape> {
  options.signal?.throwIfAborted();
  const deadline = options.requestDeadlineMs ?? 5_000;
  if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
    throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
  }
  let worker: ScratchWorker;
  try {
    if (options.createWorker !== undefined) {
      worker = options.createWorker(options.scratchWorkerModuleUrl === undefined
        ? BUNDLED_ENGINE_ASSETS.scratchWorkerModule : new URL(options.scratchWorkerModuleUrl, import.meta.url), { type: "module" });
    } else if (options.scratchWorkerModuleUrl !== undefined) {
      worker = new Worker(options.scratchWorkerModuleUrl, { type: "module" });
    } else {
      worker = new Worker(new URL("./scratch-worker.js", import.meta.url), { type: "module" });
    }
  } catch (error) { throw new BrowserBootError("scratch-start", "Scratch module Worker could not start", error); }
  return new Promise<SessionShape>((resolve, reject) => {
    let settled = false;
    let requested = false;
    let timer: ReturnType<typeof setTimeout>;
    const finish = (error: unknown, shape?: SessionShape) => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      options.signal?.removeEventListener("abort", abort);
      worker.removeEventListener("message", message);
      worker.removeEventListener("error", failure);
      worker.removeEventListener("messageerror", decodeFailure);
      worker.terminate();
      if (shape !== undefined) resolve(shape); else reject(error);
    };
    const arm = () => {
      clearTimeout(timer);
      timer = setTimeout(() => finish(new BrowserBootError("scratch-deadline",
        requested ? "Scratch request exceeded its deadline" : "Scratch Worker handshake exceeded its deadline")), deadline);
    };
    const abort = () => finish(options.signal?.reason);
    const failure = (event: ErrorEvent) => finish(new BrowserBootError("scratch-load", "Scratch Worker failed", event.error ?? new Error(event.message)));
    const decodeFailure = () => finish(new BrowserBootError("scratch-load", "Scratch Worker reply could not be decoded"));
    const message = (event: MessageEvent<ScratchBootReply>) => {
      if (settled) return;
      const reply = event.data;
      if (reply.type === "worker-ready" && !requested) {
        if (options.signal?.aborted) { abort(); return; }
        requested = true;
        arm();
        try { worker.postMessage({ type: "scratch", requestId: 1, moduleUrl: String(options.moduleUrl), document: options.document, options: options.options }); }
        catch (error) { finish(error); }
      } else if (reply.type === "scratch-result" && requested && reply.requestId === 1) {
        if (options.signal?.aborted) { abort(); return; }
        if (reply.ok) finish(undefined, reply.shape);
        else finish(Object.assign(new Error(reply.error.message), { name: reply.error.name }));
      }
    };
    worker.addEventListener("message", message);
    worker.addEventListener("error", failure);
    worker.addEventListener("messageerror", decodeFailure);
    options.signal?.addEventListener("abort", abort, { once: true });
    arm();
    if (options.signal?.aborted) abort();
  });
}
