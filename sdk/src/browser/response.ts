import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { MisoEngineAsset } from "../core/asset.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";
import type { MisoEngineErrorInit } from "../core/errors.ts";
import { RESPONSE_CAPABILITIES } from "../core/response.ts";
import type {
  ResponsePreviewCapability,
  ResponsePreviewLimits,
  ResponsePreviewQuery,
  ResponsePreviewResult,
} from "../core/response.ts";
import type { ResponseWorker, ResponseWorkerError, ResponseWorkerFactory, ResponseWorkerReply } from "./response-worker.ts";

export interface BrowserResponsePreviewOptions {
  readonly asset: MisoEngineAsset;
  readonly limits?: ResponsePreviewLimits;
  readonly responseWorkerModuleUrl?: string | URL;
  readonly createWorker?: ResponseWorkerFactory;
  readonly signal?: AbortSignal;
}

/** A dedicated browser Worker response preview with one bounded request in flight. */
export class BrowserResponsePreview {
  readonly #asset: MisoEngineAsset;
  readonly #worker: ResponseWorker;
  readonly #limits: ResponsePreviewLimits;
  #closed = false;
  #nextRequestId = 1;
  #pending: { readonly requestId: number; readonly resolve: (result: ResponsePreviewResult) => void; readonly reject: (error: unknown) => void; readonly timer: ReturnType<typeof setTimeout> } | undefined;
  #ready: Promise<void>;
  #resolveReady!: () => void;
  #rejectReady!: (error: unknown) => void;

  private constructor(asset: MisoEngineAsset, worker: ResponseWorker, limits: ResponsePreviewLimits) {
    this.#asset = asset;
    this.#worker = worker;
    this.#limits = limits;
    this.#ready = new Promise<void>((resolve, reject) => {
      this.#resolveReady = resolve;
      this.#rejectReady = reject;
    });
    worker.addEventListener("message", this.#message);
    worker.addEventListener("error", this.#failure);
    worker.addEventListener("messageerror", this.#messageFailure);
  }

  static async create(options: BrowserResponsePreviewOptions): Promise<BrowserResponsePreview> {
    if (options.asset === undefined) throw new MisoUsageError("a verified response asset is required");
    const limits = options.limits ?? {};
    const deadline = limits.requestDeadlineMs ?? 5_000;
    if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
      throw new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647");
    }
    options.signal?.throwIfAborted();
    let worker: ResponseWorker;
    try {
      const url = options.responseWorkerModuleUrl === undefined
        ? BUNDLED_ENGINE_ASSETS.responseWorkerModule
        : new URL(options.responseWorkerModuleUrl, import.meta.url);
      worker = options.createWorker === undefined
        ? new Worker(url, { type: "module" })
        : options.createWorker(url, { type: "module" });
    } catch (error) {
      throw new MisoUsageError(`response Worker could not start: ${error instanceof Error ? error.message : String(error)}`);
    }
    const preview = new BrowserResponsePreview(options.asset, worker, limits);
    try {
      await preview.initialize(options.signal, deadline);
      return preview;
    } catch (error) {
      preview.close();
      throw error;
    }
  }

  get asset(): MisoEngineAsset { return this.#asset; }

  get capabilities(): readonly ResponsePreviewCapability[] { return RESPONSE_CAPABILITIES; }

  query(request: ResponsePreviewQuery): Promise<ResponsePreviewResult> {
    if (this.#closed) return Promise.reject(new MisoUsageError("the response preview is closed"));
    if (this.#pending !== undefined) return Promise.reject(new MisoUsageError("a response query is already in flight"));
    const requestId = this.#nextRequestId;
    this.#nextRequestId = requestId === 0x7fff_ffff ? 1 : requestId + 1;
    const deadline = this.#limits.requestDeadlineMs ?? 5_000;
    return new Promise<ResponsePreviewResult>((resolve, reject) => {
      const timer = setTimeout(() => {
        if (this.#pending?.requestId !== requestId) return;
        this.#pending = undefined;
        reject(new MisoUsageError("response query exceeded its deadline"));
      }, deadline);
      this.#pending = { requestId, resolve, reject, timer };
      try {
        this.#worker.postMessage({ type: "response-query", requestId, query: request });
      } catch (error) {
        clearTimeout(timer);
        this.#pending = undefined;
        reject(error);
      }
    });
  }

  async close(): Promise<void> {
    if (this.#closed) return;
    this.#closed = true;
    this.#rejectPending(new MisoUsageError("the response preview was closed"));
    try { this.#worker.postMessage({ type: "response-close" }); } finally {
      this.#removeListeners();
      this.#worker.terminate();
    }
  }

  async initialize(signal: AbortSignal | undefined, deadline: number): Promise<void> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const abort = () => this.#rejectReady(signal?.reason ?? new MisoUsageError("response preview initialization was aborted"));
    signal?.addEventListener("abort", abort, { once: true });
    try {
      timer = setTimeout(() => this.#rejectReady(new MisoUsageError("response Worker initialization exceeded its deadline")), deadline);
      this.#worker.postMessage({ type: "response-init", module: this.#asset.module, limits: this.#limits });
      await this.#ready;
    } finally {
      if (timer !== undefined) clearTimeout(timer);
      signal?.removeEventListener("abort", abort);
    }
  }

  readonly #message = (event: MessageEvent<ResponseWorkerReply>): void => {
    const reply = event.data;
    if (reply.type === "worker-ready") return;
    if (reply.type === "response-ready") {
      this.#resolveReady();
      return;
    }
    if (reply.type === "response-failure") {
      const error = deserializeError(reply.error);
      if (reply.requestId === undefined) this.#rejectReady(error);
      else if (this.#pending?.requestId === reply.requestId) {
        const pending = this.#pending;
        this.#pending = undefined;
        clearTimeout(pending.timer);
        pending.reject(error);
      }
      return;
    }
    if (this.#pending?.requestId !== reply.requestId) return;
    const pending = this.#pending;
    this.#pending = undefined;
    clearTimeout(pending.timer);
    pending.resolve(reply.result);
  };

  readonly #failure = (event: ErrorEvent): void => {
    const error = new MisoUsageError(`response Worker failed: ${event.message || "unknown error"}`);
    this.#rejectReady(error);
    this.#rejectPending(error);
  };

  readonly #messageFailure = (): void => {
    const error = new MisoUsageError("response Worker reply could not be decoded");
    this.#rejectReady(error);
    this.#rejectPending(error);
  };

  #rejectPending(error: unknown): void {
    if (this.#pending === undefined) return;
    const pending = this.#pending;
    this.#pending = undefined;
    clearTimeout(pending.timer);
    pending.reject(error);
  }

  #removeListeners(): void {
    this.#worker.removeEventListener("message", this.#message);
    this.#worker.removeEventListener("error", this.#failure);
    this.#worker.removeEventListener("messageerror", this.#messageFailure);
  }
}

/** Create a response preview in a dedicated browser Worker. */
export async function createResponsePreview(
  options: BrowserResponsePreviewOptions,
): Promise<BrowserResponsePreview> {
  return BrowserResponsePreview.create(options);
}

function deserializeError(error: ResponseWorkerError): Error {
  if (error.kind === "engine" && error.phase !== undefined && error.code !== undefined && error.result !== undefined) {
    const init: MisoEngineErrorInit = {
      phase: error.phase as MisoEngineErrorInit["phase"],
      code: error.code as MisoEngineErrorInit["code"],
      result: error.result,
      ...(error.diagnostics === undefined ? {} : { diagnostics: error.diagnostics }),
    };
    const typed = new MisoEngineError("", init);
    typed.message = error.message;
    typed.name = error.name;
    return typed;
  }
  const typed = error.kind === "usage" ? new MisoUsageError(error.message) : new Error(error.message);
  typed.name = error.name;
  return typed;
}
