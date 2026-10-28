import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { attachEngineFeed } from "../../../sdk/src/browser/pcm-feed.ts";
import { MSB1_CONTROL, MSB1_CONTROL_BYTES } from "../../../sdk/src/browser/pcm-ring.ts";

/** #2103: attach and acknowledge without starting the audio clock, using ready()'s default 2 s. */
export async function runSuspendedPcmFeed() {
  if (!crossOriginIsolated) throw new Error("PCM feed row requires cross-origin isolation");
  const context = new AudioContext({ sampleRate: 48_000 });
  await context.suspend();
  let engine: Awaited<ReturnType<typeof createEngine>> | undefined;
  let feed: ReturnType<typeof attachEngineFeed> | undefined;
  try {
    const document = await (await fetch("/qualification/observation-session.json")).text();
    engine = await createEngine({
      document,
      pcmFeed: { moduleUrl: "/sdk/miso-engine-v1-pcm-feed-worklet.js" },
      scratchBoot: async () => ({
        sampleRateHz: 48_000, quantumFrames: 128, sourceRingFrames: 2_048,
        backend: "simd128" as const,
        sources: [{ id: "live-control-source", channels: 2, frames: 2_048n }],
        tracks: ["track"], submixes: [],
      }),
      createContext: () => context,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
      simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
      workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    });
    engine.host.node.connect(context.destination);
    const before = await engine.host.status();
    const beforeTime = context.currentTime;
    const beforeState = context.state;
    feed = attachEngineFeed({
      engine, context, quantumFrames: 128,
      sources: [{ sourceId: "live-control-source", channels: 2 }],
    });
    const began = performance.now();
    await feed.ready();
    const elapsedMs = performance.now() - began;
    const after = await engine.host.status();
    const control = new Int32Array(feed.rings[0]!, 0, MSB1_CONTROL_BYTES / 4);
    const result = {
      beforeState, afterState: context.state, beforeTime, afterTime: context.currentTime,
      beforeSample: before.nextAbsoluteSample.toString(), afterSample: after.nextAbsoluteSample.toString(),
      elapsedMs, state: feed.state, attached: Atomics.load(control, MSB1_CONTROL.ATTACHED),
      drainBlocks: Atomics.load(control, MSB1_CONTROL.DRAIN_BLOCKS),
      // Issue #1387 (S11): an isolated page runs worker mode with the feed; its Worker drains it.
      instanceMode: engine.instanceMode,
      underruns: Atomics.load(control, MSB1_CONTROL.UNDERRUNS),
      renderAllocations: await engine.host.renderAllocationCount(),
    };
    if (before.result !== 0 || after.result !== 0 || result.beforeState !== "suspended"
      || result.afterState !== "suspended" || result.beforeTime !== result.afterTime
      || result.beforeSample !== "0" || result.afterSample !== "0"
      || result.elapsedMs >= 2_000 || result.state !== "active" || result.attached !== 1
      || result.drainBlocks !== 0 || result.instanceMode !== "worker" || result.underruns !== 0
      || result.renderAllocations.result !== 0
      || result.renderAllocations.count !== 0) {
      throw new Error(`suspended PCM feed invariant failed: ${JSON.stringify(result)}`);
    }
    return result;
  } finally {
    feed?.close();
    if (engine !== undefined) await engine.close();
    else if (context.state !== "closed") await context.close();
  }
}
