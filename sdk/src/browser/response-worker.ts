import { ResponsePreviewModule } from "../core/response.ts";
import type {
  ResponsePreviewLimits,
  ResponsePreviewQuery,
  ResponsePreviewResult,
} from "../core/response.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";

export type ResponseWorkerRequest =
  | { readonly type: "response-init"; readonly module: WebAssembly.Module; readonly responseLimits: ResponsePreviewLimits }
  | { readonly type: "response-query"; readonly requestId: number; readonly query: ResponsePreviewQuery }
  | { readonly type: "response-close" };

export type ResponseWorkerReply =
  | { readonly type: "worker-ready" }
  | { readonly type: "response-ready" }
  | { readonly type: "response-result"; readonly requestId: number; readonly result: ResponsePreviewResult }
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

scope.onmessage = (event) => {
  try {
    const request = event.data;
    if (request.type === "response-init") {
      preview?.close();
      preview = new ResponsePreviewModule(new WebAssembly.Instance(request.module, {}), request.responseLimits);
      scope.postMessage({ type: "response-ready" });
    } else if (request.type === "response-query") {
      if (preview === undefined) throw new MisoUsageError("the response Worker is not initialized");
      const result = preview.query(request.query);
      scope.postMessage({ type: "response-result", requestId: request.requestId, result }, transferFor(result));
    } else {
      preview?.close();
      preview = undefined;
    }
  } catch (error) {
    const request = event.data;
    scope.postMessage({
      type: "response-failure",
      ...(request.type === "response-query" ? { requestId: request.requestId } : {}),
      error: serializeError(error),
    });
  }
};

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
