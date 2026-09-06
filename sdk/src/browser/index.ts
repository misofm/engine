/** Package-owned browser boot, with independent context, scratch and host overrides. */
export * from "./engine.ts";
export * from "./console.ts";
export * from "./policy.ts";
// The `BootOptions` -> `MisoWebBootOptions` adapter. It belongs to whoever mounts the shipped
// worklet host directly instead of going through `createEngine`, which is a browser consumer, so
// it is barrel surface rather than a deep import.
export * from "./host-mirror.ts";
export * from "./pcm-ring.ts";
export * from "./pcm-feed.ts";

export { scratchBootWithWorker } from "./scratch.ts";
export type { ScratchWorker, ScratchWorkerFactory } from "./scratch.ts";
export { createDefaultHost, BrowserBootError } from "./default-host.ts";
