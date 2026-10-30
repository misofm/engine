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

/** A live page: an engine in `instanceMode` playing one long source from a main-thread writer
 *  that keeps the shared ring full on a 5 ms timer (the pump's cadence), as an app's pump does,
 *  until `pausedUntil` (a `performance.now()` time) lets it publish again. */
async function livePlayback(instanceMode: "worker" | "single") {
  if (!crossOriginIsolated) throw new Error("PCM feed row requires cross-origin isolation");
  const context = new AudioContext({ sampleRate: 48_000 });
  await context.suspend();
  const frames = 48_000n * 120n;
  const sourceId = "live-control-source";
  let engine: Awaited<ReturnType<typeof createEngine>> | undefined;
  let feed: ReturnType<typeof attachEngineFeed> | undefined;
  let timer: ReturnType<typeof setInterval> | undefined;
  const close = async () => {
    if (timer !== undefined) clearInterval(timer);
    feed?.close();
    if (engine !== undefined) await engine.close();
    else if (context.state !== "closed") await context.close();
  };
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
    const pen = { generation: 1n, next: 0n, pausedUntil: 0 };
    writer.engage(pen.generation);
    const fill = () => {
      if (performance.now() < pen.pausedUntil) return;
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
    const live = engine;
    const report = async (id: string) => {
      const reply = await live.host.seekReport(id);
      if (reply.result !== 0 || reply.report === null) throw new Error(`seek report refused: ${reply.result}`);
      return reply.report;
    };
    const underrun = async () => (await report(sourceId)).cumulativeUnderrunFrames;
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
    const counters = async () => {
      const control = new Int32Array(feed!.rings[0]!, 0, MSB1_CONTROL_BYTES / 4);
      return {
        instanceMode: live.instanceMode, baselineUnderrunFrames: (baseline - start).toString(),
        seeksApplied: Atomics.load(control, MSB1_CONTROL.SEEKS_APPLIED),
        refused: Atomics.load(control, MSB1_CONTROL.REFUSED),
        lastRefusal: Atomics.load(control, MSB1_CONTROL.LAST_RESULT),
        renderAllocations: await live.host.renderAllocationCount(),
      };
    };
    return { engine: live, writer, pen, fill, report, underrun, counters, close, sourceId };
  } catch (error) {
    await close();
    throw error;
  }
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * #2185: plain ring seeks while audio plays, in one instance mode, on this isolated page. Each seek
 * publishes a plain seek with `Msb1RingWriter.seek` while the ring holds the old generation, and the
 * writer then feeds the new one. Each row is the engine's own missing in-region frames for the
 * source (its seek report) over the seek: from just before it to 300 ms after.
 * `baselineUnderrunFrames` is the same count over 500 ms of steady play before the first seek, so a
 * red seek row on a sink that underruns anyway is told apart. The page resumes the context inside
 * a trusted click on the button `data-live-seek-resume` (WebKit needs one).
 */
export async function runLivePcmFeedSeek(instanceMode: "worker" | "single", seeks = 5) {
  const live = await livePlayback(instanceMode);
  try {
    const rows: LiveSeekRow[] = [];
    for (let seek = 0; seek < seeks; seek += 1) {
      const before = await live.underrun();
      live.pen.generation += 1n;
      live.pen.next = BigInt(seek + 1) * 480_000n;
      live.writer.seek(live.pen.generation, live.pen.next);
      live.fill();
      await wait(300);
      rows.push({ generation: live.pen.generation.toString(), underrunFrames: (await live.underrun() - before).toString() });
    }
    return { ...(await live.counters()), seeks: rows };
  } finally {
    await live.close();
  }
}

/** One transport seek's measurement (#2205). */
export interface TransportSeekRow {
  readonly generation: string;
  /** The engine's underrun frames for the source over the seek. */
  readonly underrunFrames: string;
  /** `engine.position().atSample` right after `engine.seek` resolved: the render sample at accept. */
  readonly acceptSample: string;
  /** The first sample of the block the source and the timeline landed in (their seek reports). */
  readonly sourceLandedSample: string;
  readonly timelineLandedSample: string;
  readonly timelineGeneration: string;
  readonly sourceFrame: string;
  /** The latency the landing rule adds: landed sample minus accept sample, in samples. */
  readonly addedLatencySamples: string;
}

/**
 * #2205: transport seeks, `engine.seek(frame, { producers })`, while audio plays, in one instance
 * mode on this isolated page. The producer repositions its ring at once and publishes the new
 * position only after a 20 ms delay (a pump's window read), so the engine must keep the old
 * position playing until the new one has its runway. Each row is the source's underrun frames over
 * the seek (from just before it to 300 ms after), the block the source and the timeline landed in,
 * and the added latency against the render sample at accept.
 */
export async function runLiveTransportSeek(instanceMode: "worker" | "single", seeks = 5) {
  const live = await livePlayback(instanceMode);
  try {
    const producer = {
      seek: async (frame: number, generation: bigint) => {
        live.pen.generation = generation;
        live.pen.next = BigInt(frame);
        live.writer.seek(generation, BigInt(frame));
        live.pen.pausedUntil = performance.now() + 20;
      },
    };
    const rows: TransportSeekRow[] = [];
    for (let seek = 0; seek < seeks; seek += 1) {
      const before = await live.underrun();
      const frame = BigInt(seek + 1) * 480_000n;
      const { generation } = await live.engine.seek(frame, { producers: [producer] });
      const accept = (await live.engine.position()).atSample;
      await wait(300);
      const source = await live.report(live.sourceId);
      const timeline = await live.report("$timeline");
      if (source.generation !== generation) throw new Error(`the source did not land seek ${generation}`);
      rows.push({
        generation: generation.toString(),
        underrunFrames: (source.cumulativeUnderrunFrames - before).toString(),
        acceptSample: accept.toString(),
        sourceLandedSample: source.firstSample.toString(),
        timelineLandedSample: timeline.firstSample.toString(),
        timelineGeneration: timeline.generation.toString(),
        sourceFrame: source.sourceFrame.toString(),
        addedLatencySamples: (source.firstSample - accept).toString(),
      });
    }
    return { ...(await live.counters()), seeks: rows };
  } finally {
    await live.close();
  }
}

/** One stopped seek's measurement (#2205). */
export interface StoppedSeekRow {
  readonly generation: string;
  /** The engine's underrun frames for the source over the seek. */
  readonly underrunFrames: string;
  /** The host's next render sample after `stop()` and before `play()`: the first block after resume. */
  readonly resumeSample: string;
  /** The first sample of the block the source and the timeline landed in (their seek reports). */
  readonly sourceLandedSample: string;
  readonly timelineLandedSample: string;
  readonly timelineGeneration: string;
}

/**
 * #2205: seeks with no live audio to keep. While audio plays, `stop()`, then
 * `engine.seek(frame, { producers })` (the producer publishes 20 ms after the seek), then
 * `play()`, with no separate `feed.prepareSeek()`: the SDK runs it. The seek must land at the
 * first block rendered after resume, every source and the timeline together, so no block of the
 * stopped position plays after resume: the source's and the timeline's landing sample is the
 * render sample the host reported between `stop()` and `play()`; and the target is queued then,
 * so the source underruns no frame.
 */
export async function runLiveStoppedSeek(instanceMode: "worker" | "single", seeks = 5) {
  const live = await livePlayback(instanceMode);
  try {
    const producer = {
      seek: async (frame: number, generation: bigint) => {
        live.pen.generation = generation;
        live.pen.next = BigInt(frame);
        live.writer.seek(generation, BigInt(frame));
        live.pen.pausedUntil = performance.now() + 20;
      },
    };
    const rows: StoppedSeekRow[] = [];
    for (let seek = 0; seek < seeks; seek += 1) {
      const before = await live.underrun();
      await live.engine.stop();
      const frame = BigInt(seek + 1) * 480_000n;
      const { generation } = await live.engine.seek(frame, { producers: [producer] });
      await wait(50);
      const resume = (await live.engine.host.status()).nextAbsoluteSample;
      await live.engine.play();
      await wait(300);
      const source = await live.report(live.sourceId);
      const timeline = await live.report("$timeline");
      if (source.generation !== generation) throw new Error(`the source did not land seek ${generation}`);
      rows.push({
        generation: generation.toString(),
        underrunFrames: (source.cumulativeUnderrunFrames - before).toString(),
        resumeSample: resume.toString(),
        sourceLandedSample: source.firstSample.toString(),
        timelineLandedSample: timeline.firstSample.toString(),
        timelineGeneration: timeline.generation.toString(),
      });
    }
    return { ...(await live.counters()), seeks: rows };
  } finally {
    await live.close();
  }
}
