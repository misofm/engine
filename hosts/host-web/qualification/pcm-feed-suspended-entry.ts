import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { attachEngineFeed } from "../../../sdk/src/browser/pcm-feed.ts";
import { MSB1_CONTROL, MSB1_CONTROL_BYTES, Msb1RingWriter } from "../../../sdk/src/browser/pcm-ring.ts";

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

/** One live seek's measurement: the engine's underrun frames for the source over the seek. */
export interface LiveSeekRow { readonly generation: string; readonly underrunFrames: string }

/**
 * #2185: plain ring seeks while audio plays, in one instance mode, on this isolated page. A
 * main-thread writer keeps the shared ring full (a 5 ms timer, the pump's cadence), as an app's
 * pump does; each seek publishes a plain seek with `Msb1RingWriter.seek` while the ring holds the
 * old generation, and the writer then feeds the new one. Each row is the engine's own missing
 * in-region frames for the source (its seek report) over the seek: from just before it to 300 ms
 * after. `baselineUnderrunFrames` is the same count over 500 ms of steady play before the first
 * seek, so a red seek row on a sink that underruns anyway is told apart. The page resumes the
 * context inside a trusted click on the button `data-live-seek-resume` (WebKit needs one).
 */
export async function runLivePcmFeedSeek(instanceMode: "worker" | "single", seeks = 5) {
  if (!crossOriginIsolated) throw new Error("PCM feed row requires cross-origin isolation");
  const context = new AudioContext({ sampleRate: 48_000 });
  await context.suspend();
  const frames = 48_000n * 120n;
  const sourceId = "live-control-source";
  let engine: Awaited<ReturnType<typeof createEngine>> | undefined;
  let feed: ReturnType<typeof attachEngineFeed> | undefined;
  let timer: ReturnType<typeof setInterval> | undefined;
  const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
  try {
    const session = JSON.parse(await (await fetch("/qualification/observation-session.json")).text());
    session.sources[0].frames = frames.toString();
    engine = await createEngine({
      document: JSON.stringify(session),
      instanceMode,
      pcmFeed: { moduleUrl: "/sdk/miso-engine-v1-pcm-feed-worklet.js" },
      scratchBoot: async () => ({
        sampleRateHz: 48_000, quantumFrames: 128, sourceRingFrames: 0,
        backend: "simd128" as const,
        sources: [{ id: sourceId, channels: 2, frames }],
        tracks: ["track"], submixes: [],
      }),
      createContext: () => context,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
      simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
      workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    });
    if (engine.instanceMode !== instanceMode) throw new Error(`asked ${instanceMode}, booted ${engine.instanceMode}`);
    engine.host.node.connect(context.destination);
    feed = attachEngineFeed({ engine, context, quantumFrames: 128, sources: [{ sourceId, channels: 2 }] });
    await feed.ready();
    const writer = new Msb1RingWriter(feed.rings[0]!);
    const pen = { generation: 1n, next: 0n };
    writer.engage(pen.generation);
    const fill = () => {
      while (writer.occupancy < writer.capacity) {
        const planes = writer.reserve(128);
        if (planes === null) return;
        planes[0]!.fill(0.125); planes[1]!.fill(-0.125);
        writer.commit({ generation: pen.generation, startFrame: pen.next, frames: 128, endOfRegion: false });
        pen.next += 128n;
      }
    };
    fill();
    timer = setInterval(fill, 5);
    const underrun = async () => {
      const reply = await engine!.host.seekReport(sourceId);
      if (reply.result !== 0 || reply.report === null) throw new Error(`seek report refused: ${reply.result}`);
      return reply.report.cumulativeUnderrunFrames;
    };
    const button = document.createElement("button");
    button.setAttribute("data-live-seek-resume", instanceMode);
    button.textContent = `Resume live seek: ${instanceMode}`;
    document.body.append(button);
    await new Promise<void>((resolve, reject) => {
      button.addEventListener("click", () => context.resume().then(resolve, reject), { once: true });
    });
    button.remove();
    await wait(300);
    const start = await underrun();
    await wait(500);
    const baseline = await underrun();
    const rows: LiveSeekRow[] = [];
    for (let seek = 0; seek < seeks; seek += 1) {
      const before = await underrun();
      pen.generation += 1n;
      pen.next = BigInt(seek + 1) * 480_000n;
      writer.seek(pen.generation, pen.next);
      fill();
      await wait(300);
      rows.push({ generation: pen.generation.toString(), underrunFrames: (await underrun() - before).toString() });
    }
    const control = new Int32Array(feed.rings[0]!, 0, MSB1_CONTROL_BYTES / 4);
    return {
      instanceMode: engine.instanceMode, baselineUnderrunFrames: (baseline - start).toString(), seeks: rows,
      seeksApplied: Atomics.load(control, MSB1_CONTROL.SEEKS_APPLIED),
      refused: Atomics.load(control, MSB1_CONTROL.REFUSED),
      renderAllocations: await engine.host.renderAllocationCount(),
    };
  } finally {
    if (timer !== undefined) clearInterval(timer);
    feed?.close();
    if (engine !== undefined) await engine.close();
    else if (context.state !== "closed") await context.close();
  }
}
