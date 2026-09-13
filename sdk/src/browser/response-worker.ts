import { ResponsePreviewModule } from "../core/response.ts";
import type {
  ResponsePreviewLimits,
  ResponsePreviewQuery,
  ResponsePreviewResult,
} from "../core/response.ts";
import { TrackResponseModule } from "../core/live-response.ts";
import type { TrackResponseQuery, TrackResponseResult } from "../core/live-response.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";

export type ResponseWorkerRequest =
  | { readonly type: "response-init"; readonly module: WebAssembly.Module; readonly responseLimits: ResponsePreviewLimits }
  | { readonly type: "response-query"; readonly requestId: number; readonly query: ResponsePreviewQuery }
  | { readonly type: "track-response-init"; readonly module?: WebAssembly.Module; readonly moduleUrl?: string }
  | { readonly type: "track-response-query"; readonly requestId: number; readonly query: TrackResponseQuery; readonly snapshot: Uint8Array }
  | { readonly type: "response-close" | "track-response-close" };

export type ResponseWorkerReply =
  | { readonly type: "worker-ready" }
  | { readonly type: "response-ready" }
  | { readonly type: "response-result"; readonly requestId: number; readonly result: ResponsePreviewResult }
  | { readonly type: "track-response-ready" }
  | { readonly type: "track-response-result"; readonly requestId: number; readonly result: TrackResponseResult }
  | { readonly type: "response-failure"; readonly requestId?: number; readonly error: ResponseWorkerError };

export type ResponseWorkerError = Readonly<{
  readonly kind: "engine" | "usage" | "error";
  readonly name: string;
  readonly message: string;
  readonly phase?: string;
  readonly code?: string;
  readonly result?: number;
  readonly diagnostics?: readonly Readonly<{ readonly code: string; readonly path: string }>[];
}>;

export interface ResponseWorker {
  postMessage(message: ResponseWorkerRequest, transfer?: readonly Transferable[]): void;
  terminate(): void;
  addEventListener(type: "message" | "error" | "messageerror", listener: (event: any) => void): void;
  removeEventListener(type: "message" | "error" | "messageerror", listener: (event: any) => void): void;
}

export type ResponseWorkerFactory = (url: URL, options: { readonly type: "module" }) => ResponseWorker;

interface ResponseWorkerScope {
  onmessage: ((event: MessageEvent<ResponseWorkerRequest>) => void) | null;
  postMessage(message: ResponseWorkerReply, transfer?: readonly Transferable[]): void;
}

const candidateScope = (globalThis as unknown as { readonly self?: ResponseWorkerScope }).self ?? globalThis;
const scope = candidateScope as ResponseWorkerScope;
let preview: ResponsePreviewModule | undefined;
let trackResponse: TrackResponseModule | undefined;

scope.onmessage = (event) => { void handle(event); };

async function handle(event: MessageEvent<ResponseWorkerRequest>): Promise<void> {
  try {
    const request = event.data;
    if (request.type === "response-init") {
      preview?.close();
      preview = new ResponsePreviewModule(new WebAssembly.Instance(request.module, {}), request.responseLimits);
      scope.postMessage({ type: "response-ready" });
    } else if (request.type === "track-response-init") {
      trackResponse?.close();
      let module = request.module;
      if (module === undefined) {
        if (request.moduleUrl === undefined) throw new MisoUsageError("the live response Worker has no engine module");
        const response = await fetch(request.moduleUrl);
        if (!response.ok) throw new MisoUsageError(`the live response engine module could not be fetched (${response.status})`);
        module = await WebAssembly.compile(await response.arrayBuffer());
      }
      trackResponse = new TrackResponseModule(new WebAssembly.Instance(module, {}));
      scope.postMessage({ type: "track-response-ready" });
    } else if (request.type === "response-query") {
      if (preview === undefined) throw new MisoUsageError("the response Worker is not initialized");
      const result = preview.query(request.query);
      scope.postMessage({ type: "response-result", requestId: request.requestId, result }, transferFor(result));
    } else if (request.type === "track-response-query") {
      if (trackResponse === undefined) throw new MisoUsageError("the live response Worker is not initialized");
      const result = trackResponse.querySnapshot(request.query, request.snapshot);
      scope.postMessage({ type: "track-response-result", requestId: request.requestId, result }, transferForTrackResponse(result));
    } else {
      preview?.close();
      preview = undefined;
      trackResponse?.close();
      trackResponse = undefined;
    }
  } catch (error) {
    const request = event.data;
    scope.postMessage({
      type: "response-failure",
      ...((request.type === "response-query" || request.type === "track-response-query")
        ? { requestId: request.requestId } : {}),
      error: serializeError(error),
    });
  }
}

scope.postMessage({ type: "worker-ready" });

function transferFor(result: ResponsePreviewResult): Transferable[] {
  const buffers: ArrayBuffer[] = [result.frequenciesHz.buffer as ArrayBuffer];
  if (result.totalLeftDb !== undefined) buffers.push(result.totalLeftDb.buffer as ArrayBuffer);
  if (result.totalRightDb !== undefined) buffers.push(result.totalRightDb.buffer as ArrayBuffer);
  for (const section of result.sections) {
    if (section.leftDb !== undefined) buffers.push(section.leftDb.buffer as ArrayBuffer);
    if (section.rightDb !== undefined) buffers.push(section.rightDb.buffer as ArrayBuffer);
  }
  return buffers;
}

function transferForTrackResponse(result: TrackResponseResult): Transferable[] {
  const buffers: ArrayBuffer[] = [result.frequenciesHz.buffer as ArrayBuffer];
  if (result.leftDb !== undefined) buffers.push(result.leftDb.buffer as ArrayBuffer);
  if (result.rightDb !== undefined) buffers.push(result.rightDb.buffer as ArrayBuffer);
  return buffers;
}

function serializeError(error: unknown): ResponseWorkerError {
  if (error instanceof MisoEngineError) {
    return {
      kind: "engine",
      name: error.name,
      message: error.message,
      phase: error.phase,
      code: error.code,
      result: error.result,
      diagnostics: error.diagnostics,
    };
  }
  if (error instanceof MisoUsageError) {
    return { kind: "usage", name: error.name, message: error.message };
  }
  return {
    kind: "error",
    name: error instanceof Error ? error.name : "Error",
    message: error instanceof Error ? error.message : String(error),
  };
}
