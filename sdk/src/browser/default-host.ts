import type { AudioContextLike } from "./engine.ts";
import type { BootOptions } from "../core/abi.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { toWebBootOptions } from "./host-mirror.ts";

/** Failures in package-owned browser construction, before an engine refusal is available. */
export class BrowserBootError extends Error {
  readonly operation: "context-unavailable" | "scratch-start" | "scratch-load" | "scratch-deadline" | "host-import" | "host-create";
  constructor(operation: BrowserBootError["operation"], message: string, cause?: unknown) {
    super(message, { cause });
    this.name = "BrowserBootError";
    this.operation = operation;
  }
}

/** Import the shipped host and forward the existing boot request without installing a feed. */
export async function createDefaultHost(request: {
  readonly context: AudioContextLike;
  readonly document: Uint8Array;
  readonly options: BootOptions;
  readonly simd128ModuleUrl: string;
  readonly workletModuleUrl: string;
  readonly hostModuleUrl?: string;
}): Promise<MisoAudioWorkletHost> {
  const options = toWebBootOptions(request.options);
  const url = request.hostModuleUrl ?? BUNDLED_ENGINE_ASSETS.hostModule.href;
  let module: typeof import("./shipped-host.d.ts");
  try { module = await import(/* @vite-ignore */ url); }
  catch (error) { throw new BrowserBootError("host-import", "Engine host module could not load", error); }
  // The host's declaration uses the ambient BaseAudioContext; the runtime consumes this exact
  // structural context. No public return type is asserted or enriched here.
  return module.createMisoAudioWorkletHost({
    context: request.context,
    document: request.document,
    options,
    simd128ModuleUrl: request.simd128ModuleUrl,
    workletModuleUrl: request.workletModuleUrl,
  });
}
