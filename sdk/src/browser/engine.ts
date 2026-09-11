import { MisoEngineAsset } from "../core/asset.ts";
import { WasmBoundary } from "../core/boundary.ts";
import type { SessionShape } from "../core/boundary.ts";
import { MisoEngineError, MisoUsageError } from "../core/errors.ts";
import { constantValue } from "../core/abi.ts";
import type { SourceSpec } from "../core/types.ts";
import {
  assertQuantumMatch,
  assertWebDeliverableSources,
  scratchBootOptions,
  workletBootOptions,
} from "./policy.ts";
import type { BrowserBootPolicy } from "./policy.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";
import type { EngineConsole } from "../core/console.ts";
import { scratchBootWithWorker } from "./scratch.ts";
import type { ScratchWorkerFactory } from "./scratch.ts";
import { createDefaultHost, BrowserBootError } from "./default-host.ts";
import { createBrowserConsole } from "./console.ts";

/**
 * The browser entry (issue #243 S3, consuming #240 S5's sealed choreography).
 *
 * # Why a scratch boot exists at all
 *
 * A headless engine has no physical shape to satisfy: it accepts whatever the document declares.
 * A browser does. An `AudioContext` is constructed at a sample rate, and once constructed it is
 * expensive to replace and, on some platforms, limited in how often it may be. So the browser
 * cannot simply hand the document to the worklet and see what happens -- by then it has already
 * committed to a rate.
 *
 * The scratch boot answers the question first: boot the document in a Worker, with both `require_*`
 * words at zero, ask it what shape it declared, dispose it. That answer is what the `AudioContext`
 * is then constructed for. The Worker is mandatory rather than convenient: compiling and booting a
 * two-and-a-half megabyte module on the main realm blocks it for long enough to be visible, and
 * the answer is wanted before any audio graph exists.
 *
 * # The two boots agree, and where they do not they agree about that too
 *
 * Both boots read the same policy object, so the policy words -- ring, memory budget, and all four
 * console words -- are identical by construction. The two `require_*` words are role-defined: zero
 * in the scratch boot, physical in the worklet boot. See `./policy.ts` for why "identical options
 * struct" had to be restated that way, and for the executable form of the rule.
 *
 * # This file is deliberately thin
 *
 * Everything decidable lives in `./policy.ts` as pure functions, because those are what a harness
 * can prove. What remains here is `AudioContext` construction and module loading, which only a
 * browser can exercise; #246 owns those matrices end to end.
 */

/** The narrow slice of the Web Audio surface this entry needs. */
export interface AudioContextLike {
  readonly sampleRate: number;
  readonly renderQuantumSize?: number;
  readonly state: string;
  close(): Promise<void>;
  readonly audioWorklet: { addModule(url: string): Promise<void> };
}

/** Resolves against the consumer's ambient browser constructor without requiring DOM libs here. */
export type DefaultAudioContext = typeof globalThis extends {
  AudioContext: abstract new (...args: never[]) => infer Context extends AudioContextLike;
} ? Context : AudioContextLike;

export interface CreateEngineOptions<Context extends AudioContextLike = AudioContextLike> {
  /** The Session V1 document, or the SDK builder session that produced it. */
  readonly document: Uint8Array | string | { toJson(): string };
  /**
   * The sources this session declares, when they were authored through the SDK.
   *
   * Supplied so a `32f` document can be refused before an `AudioContext` is constructed. Absent
   * for a raw document, whose sources the SDK has never seen as values -- that case belongs to the
   * resolver/ingest boundary (#244), and the SDK has no JSON parser to substitute for one.
   */
  readonly sources?: readonly { readonly id: string; readonly spec: SourceSpec }[];
  /** Release URLs, from the same release as the module bytes. */
  readonly simd128ModuleUrl?: string;
  readonly preparedModule?: WebAssembly.Module;
  readonly workletModuleUrl?: string;
  readonly hostModuleUrl?: string;
  readonly scratchWorkerModuleUrl?: string;
  readonly createWorker?: ScratchWorkerFactory;
  readonly requestDeadlineMs?: number;
  readonly signal?: AbortSignal;
  /** Constructs an `AudioContext` at the requested rate. Injected so the entry stays testable. */
  readonly createContext?: (options: {
    readonly sampleRate: number;
    readonly renderSizeHint: number;
  }) => Context;
  /** Boots a scratch instance in a Worker and returns the shape it read back. */
  readonly scratchBoot?: (request: {
    readonly document: Uint8Array;
    readonly options: ReturnType<typeof scratchBootOptions>;
  }) => Promise<SessionShape>;
  /** Creates the worklet host once the context is verified. Normally the shipped factory. */
  readonly createHost?: (request: {
    readonly context: Context;
    readonly document: Uint8Array;
    readonly options: ReturnType<typeof workletBootOptions>;
    readonly simd128ModuleUrl: string;
    readonly preparedModule?: WebAssembly.Module;
    readonly workletModuleUrl: string;
  }) => Promise<MisoAudioWorkletHost>;
  readonly policy?: BrowserBootPolicy;
  /** How many construct-verify-close-retry rounds to allow. */
  readonly contextAttempts?: number;
}

export interface BrowserEngine<Context extends AudioContextLike = DefaultAudioContext> {
  readonly shape: SessionShape;
  readonly context: Context;
  readonly host: MisoAudioWorkletHost;
  /** Bind the semantic console once; rejects with MisoUsageError when no console was attached. */
  console(): Promise<EngineConsole>;
  /** Dispose the worklet host, then close its context. Safe to call more than once. */
  close(): Promise<void>;
}

function documentBytes(document: CreateEngineOptions["document"]): Uint8Array<ArrayBuffer> {
  if (typeof document === "string") return new TextEncoder().encode(document);
  if (document instanceof Uint8Array) {
    return new Uint8Array(document);
  }
  return new TextEncoder().encode(document.toJson());
}

/**
 * Open a browser session.
 *
 * The order is the sealed one, and each step exists because the step after it is expensive to undo:
 *
 * 1. **Refuse what web delivery does not carry.** A `32f` source is refused here, before anything
 *    is constructed, when the caller authored its sources through the SDK.
 * 2. **Scratch boot in a Worker.** Learn the document's declared shape from the engine, not from
 *    its text.
 * 3. **Construct-verify-close-retry.** An `AudioContext` is *asked* for a rate; it is not obliged
 *    to give one. So the rate it actually reports is verified, and a context that came back at the
 *    wrong rate is closed and retried rather than used. `renderSizeHint` is passed
 *    unconditionally -- a browser that ignores it is no worse off, and one that honours it saves
 *    the whole session.
 * 4. **Pre-worklet quantum refusal.** Checked against the context that exists, before `addModule`,
 *    so a mismatch is an answer to the caller rather than a sticky failure on a live graph.
 * 5. **Worklet boot as the backstop.** Its two `require_*` words carry the physical shape, so if
 *    the context changed under everything above, the engine still refuses rather than rendering at
 *    a rate nobody agreed to.
 */
export function createEngine<Context extends AudioContextLike>(
  options: CreateEngineOptions<Context> & { readonly createContext: NonNullable<CreateEngineOptions<Context>["createContext"]> },
): Promise<BrowserEngine<Context>>;
export function createEngine(
  options: CreateEngineOptions & { readonly createContext?: undefined },
): Promise<BrowserEngine<DefaultAudioContext>>;
export function createEngine(options: CreateEngineOptions): Promise<BrowserEngine<AudioContextLike>>;
export async function createEngine(options: CreateEngineOptions): Promise<BrowserEngine<AudioContextLike>> {
  const document = documentBytes(options.document);
  const policy = { ...options.policy, ...(typeof options.policy?.console === "object" ? { console: { ...options.policy.console } } : {}) };
  const preparedModule = options.preparedModule;
  const simd128ModuleUrl = options.simd128ModuleUrl ?? BUNDLED_ENGINE_ASSETS.wasm.href;
  const workletModuleUrl = options.workletModuleUrl ?? BUNDLED_ENGINE_ASSETS.workletModule.href;

  // 1. Web delivery scope.
  if (options.sources !== undefined) assertWebDeliverableSources(options.sources);

  // 2. The scratch boot's answer.
  const scratchBoot = options.scratchBoot ?? ((request) => scratchBootWithWorker({
    ...request,
    moduleUrl: simd128ModuleUrl,
    ...(options.scratchWorkerModuleUrl === undefined ? {} : { scratchWorkerModuleUrl: options.scratchWorkerModuleUrl }),
    ...(options.createWorker === undefined ? {} : { createWorker: options.createWorker }),
    ...(options.requestDeadlineMs === undefined ? {} : { requestDeadlineMs: options.requestDeadlineMs }),
    ...(options.signal === undefined ? {} : { signal: options.signal }),
  }));
  const shape = await scratchBoot({
    document,
    options: scratchBootOptions(policy),
  });

  // 3. Construct-verify-close-retry.
  const attempts = options.contextAttempts ?? 2;
  if (!Number.isInteger(attempts) || attempts < 1) {
    throw new MisoUsageError("contextAttempts must be a positive integer");
  }
  const createContext = options.createContext ?? defaultCreateContext;
  let context: AudioContextLike | undefined;
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    const candidate = createContext({
      sampleRate: shape.sampleRateHz,
      renderSizeHint: shape.quantumFrames,
    });
    if (candidate.sampleRate === shape.sampleRateHz) {
      context = candidate;
      break;
    }
    await candidate.close();
  }
  if (context === undefined) {
    throw new MisoEngineError(
      `no AudioContext could be constructed at ${shape.sampleRateHz} Hz in ${attempts} attempts`,
      {
        phase: "boot",
        code: "reprepareRequired",
        result: constantValue("resultCodes", "reprepareRequired"),
        diagnostics: [{ code: "host.session.shape", path: "$.sample_rate_hz" }],
      },
    );
  }

  // 4. The pre-worklet quantum refusal.
  try {
    assertQuantumMatch(context.renderQuantumSize, shape.quantumFrames);
  } catch (error) {
    await context.close();
    throw error;
  }

  // 5. The worklet boot, with the physical shape required.
  try {
    const createHost = options.createHost ?? ((request) => createDefaultHost({
      ...request,
      ...(options.hostModuleUrl === undefined ? {} : { hostModuleUrl: options.hostModuleUrl }),
    }));
    const host = await createHost({
      context,
      document,
      options: workletBootOptions(policy, {
        sampleRateHz: shape.sampleRateHz,
        quantumFrames: shape.quantumFrames,
      }),
      simd128ModuleUrl,
      workletModuleUrl,
      ...(preparedModule === undefined ? {} : { preparedModule }),
    });
    let semanticConsole: Promise<EngineConsole> | undefined;
    let closePromise: Promise<void> | undefined;
    return Object.freeze({
      shape,
      context,
      host,
      console: () => {
        semanticConsole ??= (policy.console?.commandQueueRecords ?? 0) === 0
          ? Promise.reject(new MisoUsageError(
            "this engine booted with no console attached; set policy.console.commandQueueRecords",
          ))
          : createBrowserConsole(host);
        return semanticConsole;
      },
      close: () => {
        closePromise ??= (async () => {
          try {
            await host.dispose();
          } finally {
            // A failed MessagePort disposal must not leak the much larger AudioContext.
            await context.close();
          }
        })();
        return closePromise;
      },
    });
  } catch (error) {
    await context.close();
    throw error;
  }
}

/**
 * The scratch boot's body, to be run inside a Worker.
 *
 * The packaged Worker calls this primitive. Custom Workers may also import it, call it, and post
 * the result back; context/host ownership remains with the browser entry.
 */
export async function scratchBootInWorker(request: {
  readonly moduleBytes: Uint8Array<ArrayBuffer>;
  readonly document: Uint8Array;
  readonly options: ReturnType<typeof scratchBootOptions>;
  readonly expectedSha256?: string;
}): Promise<SessionShape> {
  const document = new Uint8Array(request.document);
  const options = { ...request.options, ...(typeof request.options.console === "object" ? { console: { ...request.options.console } } : {}) };
  const asset = await MisoEngineAsset.load(request.moduleBytes, request.expectedSha256);
  const boundary = await WasmBoundary.boot(asset, document, options);
  try {
    return boundary.shape();
  } finally {
    // The scratch instance's whole purpose is discharged by the answer. Holding it would keep a
    // second engine's worth of memory alive beside the one that is about to render.
    boundary.dispose();
  }
}

/** Compile and rehearse only disposable DSP state; retain the compiled code for live boot. */
export async function prepareBrowserSessionInWorker(request: Parameters<typeof scratchBootInWorker>[0]): Promise<import("./scratch.ts").PreparedBrowserSession> {
  const document = new Uint8Array(request.document);
  const options = { ...request.options, ...(typeof request.options.console === "object" ? { console: { ...request.options.console } } : {}) };
  const asset = await MisoEngineAsset.load(request.moduleBytes, request.expectedSha256);
  const boundary = await WasmBoundary.boot(asset, document, options);
  try {
    const shape = boundary.shape();
    const quantum = shape.quantumFrames;
    // Only one quantum per channel is retained, regardless of source count or duration.
    const plane = new Float32Array(quantum);
    for (let frame = 0; frame < quantum; frame++) plane[frame] = 0.125 * Math.sin(frame * 0.13) + 0.0625;
    const planesByChannels = new Map<number, readonly Float32Array[]>();
    const meters = boundary.sessionMap().metersAttached;
    if (meters && !boundary.meterLease(true).ok) throw new MisoUsageError("Preparation meter lease refused");
    for (let block = 0; block < 64; block++) {
      const startFrame = BigInt(block * quantum);
      for (const source of shape.sources) {
        if (startFrame >= source.frames) continue;
        const frames = Number(source.frames - startFrame < BigInt(quantum) ? source.frames - startFrame : BigInt(quantum));
        let planes = planesByChannels.get(source.channels);
        if (planes === undefined) {
          planes = Array.from({ length: source.channels }, () => plane);
          planesByChannels.set(source.channels, planes);
        }
        const submitted = boundary.submitSource({ sourceId: source.id, generation: 1n, startFrame,
          planes: frames === quantum ? planes : planes.map(channel => channel.subarray(0, frames)),
          endOfRegion: startFrame + BigInt(frames) === source.frames });
        if (!submitted.ok) throw new MisoUsageError(`Preparation source submission refused: ${submitted.code}`);
      }
      boundary.render(quantum);
      if (meters) boundary.pollMeters();
    }
    return { shape, module: asset.module };
  } finally { boundary.dispose(); }
}

function defaultCreateContext(options: { sampleRate: number; renderSizeHint: number }): AudioContextLike {
  const constructor = (globalThis as { AudioContext?: new (options: {
    sampleRate: number; renderSizeHint: number;
  }) => AudioContextLike }).AudioContext;
  if (typeof constructor !== "function") {
    throw new BrowserBootError("context-unavailable", "AudioContext is unavailable");
  }
  return new constructor(options);
}
