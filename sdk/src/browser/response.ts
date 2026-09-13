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
import type {
  TrackResponseCapture,
  TrackResponseLimits,
  TrackResponseQuery,
  TrackResponseResult,
} from "../core/live-response.ts";
import type { SpectrumQuery, SpectrumResult, SpectrumLimits } from "../core/spectrum.ts";
import type { ResponseWorker, ResponseWorkerError, ResponseWorkerFactory, ResponseWorkerReply } from "./response-worker.ts";

export interface BrowserResponsePreviewOptions {
  readonly asset: MisoEngineAsset;
  readonly responseLimits?: ResponsePreviewLimits;
  readonly responseWorkerModuleUrl?: string | URL;
  readonly createWorker?: ResponseWorkerFactory;
  readonly signal?: AbortSignal;
}

/** A dedicated browser Worker response preview with one bounded request in flight. */
export class BrowserResponsePreview {
  readonly #asset: MisoEngineAsset;
  readonly #worker: ResponseWorker;
  readonly #responseLimits: ResponsePreviewLimits;
  #closed = false;
  #nextRequestId = 1;
  #pending: { readonly requestId: number; readonly resolve: (result: ResponsePreviewResult) => void; readonly reject: (error: unknown) => void; readonly timer: ReturnType<typeof setTimeout> } | undefined;
  #ready: Promise<void>;
  #resolveReady!: () => void;
  #rejectReady!: (error: unknown) => void;

  private constructor(asset: MisoEngineAsset, worker: ResponseWorker, responseLimits: ResponsePreviewLimits) {
    this.#asset = asset;
    this.#worker = worker;
    this.#responseLimits = responseLimits;
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
    const responseLimits = options.responseLimits ?? {};
    const deadline = responseLimits.requestDeadlineMs ?? 5_000;
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
    const preview = new BrowserResponsePreview(options.asset, worker, responseLimits);
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
    const deadline = this.#responseLimits.requestDeadlineMs ?? 5_000;
    if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
      return Promise.reject(new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647"));
    }
    return new Promise<ResponsePreviewResult>((resolve, reject) => {
      const timer = setTimeout(() => {
        if (this.#pending?.requestId !== requestId) return;
        this.#terminate(new MisoUsageError("response query exceeded its deadline"));
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
    this.#terminate(new MisoUsageError("the response preview was closed"));
  }

  async initialize(signal: AbortSignal | undefined, deadline: number): Promise<void> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const abort = () => this.#rejectReady(signal?.reason ?? new MisoUsageError("response preview initialization was aborted"));
    signal?.addEventListener("abort", abort, { once: true });
    try {
      timer = setTimeout(() => this.#rejectReady(new MisoUsageError("response Worker initialization exceeded its deadline")), deadline);
      this.#worker.postMessage({ type: "response-init", module: this.#asset.module, responseLimits: this.#responseLimits });
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
    if (reply.type !== "response-result" || this.#pending?.requestId !== reply.requestId) return;
    const pending = this.#pending;
    this.#pending = undefined;
    clearTimeout(pending.timer);
    pending.resolve(reply.result);
  };

  readonly #failure = (event: ErrorEvent): void => {
    const error = new MisoUsageError(`response Worker failed: ${event.message || "unknown error"}`);
    this.#rejectReady(error);
    this.#terminate(error);
  };

  readonly #messageFailure = (): void => {
    const error = new MisoUsageError("response Worker reply could not be decoded");
    this.#rejectReady(error);
    this.#terminate(error);
  };

  #terminate(error: unknown): void {
    if (this.#closed) return;
    this.#closed = true;
    this.#rejectPending(error);
    try { this.#worker.postMessage({ type: "response-close" }); } finally {
      this.#removeListeners();
      this.#worker.terminate();
    }
  }

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

export interface BrowserTrackResponseOptions {
  readonly module?: WebAssembly.Module;
  readonly moduleUrl?: string | URL;
  readonly responseWorkerModuleUrl?: string | URL;
  readonly responseLimits?: TrackResponseLimits;
  readonly createWorker?: ResponseWorkerFactory;
  readonly signal?: AbortSignal;
}

/** A dedicated Worker client for one copied live track-response snapshot at a time. */
export class BrowserTrackResponse {
  readonly #worker: ResponseWorker;
  readonly #responseLimits: TrackResponseLimits;
  #closed = false;
  #nextRequestId = 1;
  #pending: {
    readonly requestId: number;
    readonly resolve: (result: TrackResponseResult) => void;
    readonly reject: (error: unknown) => void;
    readonly timer: ReturnType<typeof setTimeout>;
  } | undefined;
  #ready: Promise<void>;
  #resolveReady!: () => void;
  #rejectReady!: (error: unknown) => void;

  private constructor(worker: ResponseWorker, responseLimits: TrackResponseLimits) {
    this.#worker = worker;
    this.#responseLimits = responseLimits;
    this.#ready = new Promise<void>((resolve, reject) => {
      this.#resolveReady = resolve;
      this.#rejectReady = reject;
    });
    worker.addEventListener("message", this.#message);
    worker.addEventListener("error", this.#failure);
    worker.addEventListener("messageerror", this.#messageFailure);
  }

  static async create(options: BrowserTrackResponseOptions): Promise<BrowserTrackResponse> {
    if (options.module === undefined && options.moduleUrl === undefined) {
      throw new MisoUsageError("a live response engine module or module URL is required");
    }
    const limits = options.responseLimits ?? {};
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
    const client = new BrowserTrackResponse(worker, limits);
    try {
      await client.initialize(options.module, options.moduleUrl, options.signal, deadline);
      return client;
    } catch (error) {
      await client.close();
      throw error;
    }
  }

  query(request: TrackResponseQuery, capture: TrackResponseCapture): Promise<TrackResponseResult> {
    if (this.#closed) return Promise.reject(new MisoUsageError("the live response Worker is closed"));
    if (this.#pending !== undefined) return Promise.reject(new MisoUsageError("a live track response query is already in flight"));
    if (!(capture.snapshot instanceof Uint8Array)) return Promise.reject(new MisoUsageError("the live response snapshot must be bytes"));
    const requestId = this.#nextRequestId;
    this.#nextRequestId = requestId === 0x7fff_ffff ? 1 : requestId + 1;
    const deadline = request.responseLimits?.requestDeadlineMs ?? this.#responseLimits.requestDeadlineMs ?? 5_000;
    if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
      return Promise.reject(new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647"));
    }
    return new Promise<TrackResponseResult>((resolve, reject) => {
      const timer = setTimeout(() => {
        if (this.#pending?.requestId !== requestId) return;
        this.#terminate(new MisoUsageError("live track response query exceeded its deadline"));
      }, deadline);
      this.#pending = { requestId, resolve, reject, timer };
      try {
        const snapshot = capture.snapshot.slice();
        this.#worker.postMessage(
          { type: "track-response-query", requestId, query: request, snapshot },
          [snapshot.buffer],
        );
      } catch (error) {
        clearTimeout(timer);
        this.#pending = undefined;
        reject(error);
      }
    });
  }

  async close(): Promise<void> {
    this.#terminate(new MisoUsageError("the live response Worker was closed"));
  }

  async initialize(
    module: WebAssembly.Module | undefined,
    moduleUrl: string | URL | undefined,
    signal: AbortSignal | undefined,
    deadline: number,
  ): Promise<void> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const abort = () => this.#rejectReady(signal?.reason ?? new MisoUsageError("live response initialization was aborted"));
    signal?.addEventListener("abort", abort, { once: true });
    try {
      timer = setTimeout(() => this.#rejectReady(new MisoUsageError("live response Worker initialization exceeded its deadline")), deadline);
      const init = module === undefined
        ? { type: "track-response-init" as const, moduleUrl: String(moduleUrl) }
        : { type: "track-response-init" as const, module };
      this.#worker.postMessage(init);
      await this.#ready;
    } finally {
      if (timer !== undefined) clearTimeout(timer);
      signal?.removeEventListener("abort", abort);
    }
  }

  readonly #message = (event: MessageEvent<ResponseWorkerReply>): void => {
    const reply = event.data;
    if (reply.type === "worker-ready") return;
    if (reply.type === "track-response-ready") {
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
    if (reply.type !== "track-response-result" || this.#pending?.requestId !== reply.requestId) return;
    const pending = this.#pending;
    this.#pending = undefined;
    clearTimeout(pending.timer);
    pending.resolve(reply.result);
  };

  readonly #failure = (event: ErrorEvent): void => {
    const error = new MisoUsageError(`response Worker failed: ${event.message || "unknown error"}`);
    this.#rejectReady(error);
    this.#terminate(error);
  };

  readonly #messageFailure = (): void => {
    const error = new MisoUsageError("response Worker reply could not be decoded");
    this.#rejectReady(error);
    this.#terminate(error);
  };

  #terminate(error: unknown): void {
    if (this.#closed) return;
    this.#closed = true;
    this.#rejectReady(error);
    this.#rejectPending(error);
    try { this.#worker.postMessage({ type: "track-response-close" }); } finally {
      this.#removeListeners();
      this.#worker.terminate();
    }
  }

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

/** Create a browser live track-response client in a dedicated Worker. */
export async function createTrackResponse(
  options: BrowserTrackResponseOptions,
): Promise<BrowserTrackResponse> {
  return BrowserTrackResponse.create(options);
}

export interface BrowserSpectrumOptions {
  readonly module?: WebAssembly.Module;
  readonly moduleUrl?: string | URL;
  readonly responseWorkerModuleUrl?: string | URL;
  readonly spectrumLimits?: SpectrumLimits;
  readonly createWorker?: ResponseWorkerFactory;
  readonly signal?: AbortSignal;
}

/** A dedicated Worker client for one copied spectrum snapshot at a time. */
export class BrowserSpectrum {
  readonly #worker: ResponseWorker;
  readonly #spectrumLimits: SpectrumLimits;
  #closed = false;
  #nextRequestId = 1;
  #pending: {
    readonly requestId: number;
    readonly resolve: (result: SpectrumResult) => void;
    readonly reject: (error: unknown) => void;
    readonly timer: ReturnType<typeof setTimeout>;
  } | undefined;
  #ready: Promise<void>;
  #resolveReady!: () => void;
  #rejectReady!: (error: unknown) => void;

  private constructor(worker: ResponseWorker, spectrumLimits: SpectrumLimits) {
    this.#worker = worker;
    this.#spectrumLimits = spectrumLimits;
    this.#ready = new Promise<void>((resolve, reject) => {
      this.#resolveReady = resolve;
      this.#rejectReady = reject;
    });
    worker.addEventListener("message", this.#message);
    worker.addEventListener("error", this.#failure);
    worker.addEventListener("messageerror", this.#messageFailure);
  }

  static async create(options: BrowserSpectrumOptions): Promise<BrowserSpectrum> {
    if (options.module === undefined && options.moduleUrl === undefined) {
      throw new MisoUsageError("a spectrum engine module or module URL is required");
    }
    const limits = options.spectrumLimits ?? {};
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
      throw new MisoUsageError(`spectrum Worker could not start: ${error instanceof Error ? error.message : String(error)}`);
    }
    const client = new BrowserSpectrum(worker, limits);
    try {
      await client.initialize(options.module, options.moduleUrl, options.signal, deadline);
      return client;
    } catch (error) {
      await client.close();
      throw error;
    }
  }

  query(request: SpectrumQuery, snapshot: Uint8Array): Promise<SpectrumResult> {
    if (this.#closed) return Promise.reject(new MisoUsageError("the spectrum Worker is closed"));
    if (this.#pending !== undefined) return Promise.reject(new MisoUsageError("a spectrum query is already in flight"));
    if (!(snapshot instanceof Uint8Array)) return Promise.reject(new MisoUsageError("the spectrum snapshot must be bytes"));
    const requestId = this.#nextRequestId;
    this.#nextRequestId = requestId === 0x7fff_ffff ? 1 : requestId + 1;
    const deadline = request.spectrumLimits?.requestDeadlineMs
      ?? this.#spectrumLimits.requestDeadlineMs
      ?? 5_000;
    if (!Number.isFinite(deadline) || deadline <= 0 || deadline > 2_147_483_647) {
      return Promise.reject(new MisoUsageError("requestDeadlineMs must be positive and at most 2147483647"));
    }
    return new Promise<SpectrumResult>((resolve, reject) => {
      const timer = setTimeout(() => {
        if (this.#pending?.requestId !== requestId) return;
        this.#terminate(new MisoUsageError("spectrum query exceeded its deadline"));
      }, deadline);
      this.#pending = { requestId, resolve, reject, timer };
      try {
        const copy = snapshot.slice();
        this.#worker.postMessage(
          { type: "spectrum-query", requestId, query: request, snapshot: copy },
          [copy.buffer],
        );
      } catch (error) {
        clearTimeout(timer);
        this.#pending = undefined;
        reject(error);
      }
    });
  }

  async close(): Promise<void> {
    this.#terminate(new MisoUsageError("the spectrum Worker was closed"));
  }

  async initialize(
    module: WebAssembly.Module | undefined,
    moduleUrl: string | URL | undefined,
    signal: AbortSignal | undefined,
    deadline: number,
  ): Promise<void> {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const abort = () => this.#rejectReady(signal?.reason ?? new MisoUsageError("spectrum initialization was aborted"));
    signal?.addEventListener("abort", abort, { once: true });
    try {
      timer = setTimeout(() => this.#rejectReady(new MisoUsageError("spectrum Worker initialization exceeded its deadline")), deadline);
      const init = module === undefined
        ? { type: "spectrum-init" as const, moduleUrl: String(moduleUrl) }
        : { type: "spectrum-init" as const, module };
      this.#worker.postMessage(init);
      await this.#ready;
    } finally {
      if (timer !== undefined) clearTimeout(timer);
      signal?.removeEventListener("abort", abort);
    }
  }

  readonly #message = (event: MessageEvent<ResponseWorkerReply>): void => {
    const reply = event.data;
    if (reply.type === "worker-ready") return;
    if (reply.type === "spectrum-ready") {
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
    if (reply.type !== "spectrum-result" || this.#pending?.requestId !== reply.requestId) return;
    const pending = this.#pending;
    this.#pending = undefined;
    clearTimeout(pending.timer);
    pending.resolve(reply.result);
  };

  readonly #failure = (event: ErrorEvent): void => {
    const error = new MisoUsageError(`spectrum Worker failed: ${event.message || "unknown error"}`);
    this.#rejectReady(error);
    this.#terminate(error);
  };

  readonly #messageFailure = (): void => {
    const error = new MisoUsageError("spectrum Worker reply could not be decoded");
    this.#rejectReady(error);
    this.#terminate(error);
  };

  #terminate(error: unknown): void {
    if (this.#closed) return;
    this.#closed = true;
    this.#rejectReady(error);
    this.#rejectPending(error);
    try { this.#worker.postMessage({ type: "spectrum-close" }); } finally {
      this.#removeListeners();
      this.#worker.terminate();
    }
  }

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

/** Create a browser spectrum analysis Worker client. */
export async function createSpectrum(options: BrowserSpectrumOptions): Promise<BrowserSpectrum> {
  return BrowserSpectrum.create(options);
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
