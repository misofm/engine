import type { AudioContextLike } from "./engine.ts";
import type { BootOptions } from "../core/abi.ts";
import type { MisoAudioWorkletHost } from "./shipped-host.d.ts";
import { BUNDLED_ENGINE_ASSETS } from "../assets.ts";
import { toWebBootOptions } from "./host-mirror.ts";

/** Failures in package-owned browser construction, before an engine refusal is available. */
export class BrowserBootError extends Error {
  readonly operation: "context-unavailable" | "scratch-start" | "scratch-load" | "scratch-deadline" | "host-import" | "worker-failed";
  constructor(operation: BrowserBootError["operation"], message: string, cause?: unknown) {
    super(message, { cause });
    this.name = "BrowserBootError";
    this.operation = operation;
  }
}

/** Import the shipped host and forward the existing boot request without installing a feed. */
export async function createDefaultHost(request: {
  readonly context: AudioContextLike & { suspend?(): Promise<void> };
  readonly document: Uint8Array;
  readonly options: BootOptions;
  readonly simd128ModuleUrl: string;
  readonly preparedModule?: WebAssembly.Module;
  readonly bootCommand?: Uint8Array;
  readonly workletModuleUrl: string;
  readonly hostModuleUrl?: string;
  readonly instanceMode?: "single" | "worker";
}): Promise<MisoAudioWorkletHost> {
  // Native contexts constructed after user activation may start running. The shipped host
  // requires suspension before it installs its worklet; this is the adapter's existing prelude.
  if (request.context.state === "running") await request.context.suspend?.();
  const options = toWebBootOptions(request.options);
  const url = request.hostModuleUrl ?? BUNDLED_ENGINE_ASSETS.hostModule.href;
  let module: typeof import("./shipped-host.d.ts");
  try { module = await import(/* @vite-ignore */ url); }
  catch (error) { throw new BrowserBootError("host-import", "Engine host module could not load", error); }
  // The host's declaration uses the ambient BaseAudioContext; the runtime consumes this exact
  // structural context. No public return type is asserted or enriched here.
  try {
    return await module.createMisoAudioWorkletHost({
      context: request.context,
      document: request.document,
      options,
      simd128ModuleUrl: request.simd128ModuleUrl,
      ...(request.preparedModule === undefined ? {} : { preparedModule: request.preparedModule }),
      ...(request.bootCommand === undefined ? {} : { bootCommand: request.bootCommand }),
      workletModuleUrl: request.workletModuleUrl,
      ...(request.instanceMode === undefined ? {} : { instanceMode: request.instanceMode }),
    });
  } catch (error) {
    // #1332 D6: the control Worker failed before its boot reply; never a silent `single` boot.
    if ((error as { tag?: unknown } | null)?.tag === "miso.worker-failed.v1") {
      throw new BrowserBootError("worker-failed", "Engine control Worker failed before its boot reply", error);
    }
    throw error;
  }
}
