import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { attachEngineFeed } from "../../../sdk/src/browser/pcm-feed.ts";
import { MSB1_CONTROL, MSB1_CONTROL_BYTES, Msb1RingWriter } from "../../../sdk/src/browser/pcm-ring.ts";

/** One cell of the on-demand soak (#2243): what `soak.mjs` asks the page to run. */
export interface SoakRequest {
  readonly instanceMode: "worker" | "single";
  readonly sampleRateHz: number;
  /** The measured play time, warm-up included. */
  readonly seconds: number;
  readonly warmupSeconds: number;
  readonly sampleIntervalSeconds: number;
  /** The session document, rewritten by the runner to `sampleRateHz` and a source long enough. */
  readonly document: string;
  readonly sourceId: string;
  readonly sourceFrames: string;
  readonly tracks: readonly string[];
  /** The button the runner clicks to resume the context (WebKit needs a trusted click). */
  readonly resumeLabel: string;
}

/** One `host.status()` reading, taken every `sampleIntervalSeconds`. */
interface SoakSample {
  readonly wallSeconds: number;
  readonly contextSeconds: number;
  readonly afterWarmup: boolean;
  readonly memoryBytes: number;
  readonly renderedQuanta: string;
  readonly lastResult: number;
  readonly blockingRebuilds: number;
  readonly singleModeControlAllocations: number;
  readonly underrunFrames: string;
  readonly ringUnderruns: number;
  readonly contextState: string;
}

const wait = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/** The tone the writer feeds: 440 Hz at -12 dBFS on both channels. */
const TONE_HZ = 440;
const TONE_AMPLITUDE = 10 ** (-12 / 20);

/**
 * #2243: one soak cell. An engine in `instanceMode` plays the 64-track console session at
 * `sampleRateHz` for `seconds` from one long stereo source that a main-thread writer keeps full on
 * a 5 ms timer, as `livePlayback` does. The page reads render telemetry through the SDK's lease and
 * samples the host's status, the source's underrun frames and the clocks; the runner applies the
 * gates. A context that cannot be built at the rate returns `contextUnavailable` and nothing else.
 */
export async function runSoak(request: SoakRequest) {
  if (!crossOriginIsolated) throw new Error("the soak requires cross-origin isolation");
  let context: AudioContext;
  try {
    context = new AudioContext({ sampleRate: request.sampleRateHz });
  } catch (error) {
    return { contextUnavailable: String(error) };
  }
  if (context.sampleRate !== request.sampleRateHz) {
    await context.close();
    return { contextUnavailable: `asked ${request.sampleRateHz} Hz, the context runs ${context.sampleRate} Hz` };
  }
  await context.suspend();
  const frames = BigInt(request.sourceFrames);
  const sourceId = request.sourceId;
  let engine: Awaited<ReturnType<typeof createEngine>> | undefined;
  let feed: ReturnType<typeof attachEngineFeed> | undefined;
  let timer: ReturnType<typeof setInterval> | undefined;
  let unsubscribe: (() => void) | undefined;
  try {
    engine = await createEngine({
      document: request.document,
      instanceMode: request.instanceMode,
      // The telemetry lease rides on the live controls, as an app's console has them.
      policy: { liveControls: {} },
      pcmFeed: { moduleUrl: "/sdk/miso-engine-v1-pcm-feed-worklet.js" },
      scratchBoot: async () => ({
        sampleRateHz: request.sampleRateHz, quantumFrames: 128, sourceRingFrames: 0,
        backend: "simd128" as const,
        sources: [{ id: sourceId, channels: 2, frames }],
        tracks: [...request.tracks], submixes: [],
      }),
      createContext: () => context,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
      simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
      workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    });
    if (engine.instanceMode !== request.instanceMode) {
      throw new Error(`asked ${request.instanceMode}, booted ${engine.instanceMode}`);
    }
    const live = engine;
    live.host.node.connect(context.destination);
    // 256 chunks of 128 frames: 340 ms of runway at 96 kHz, so the harness pump is not what underruns.
    feed = attachEngineFeed({
      engine: live, context, quantumFrames: 128, capacityChunks: 256,
      sources: [{ sourceId, channels: 2 }],
    });
    await feed.ready();
    const ring = feed.rings[0]!;
    const writer = new Msb1RingWriter(ring);
    const step = (2 * Math.PI * TONE_HZ) / request.sampleRateHz;
    let next = 0n;
    let phase = 0;
    writer.engage(1n);
    const fill = () => {
      while (writer.occupancy < writer.capacity) {
        const planes = writer.reserve(128);
        if (planes === null) return;
        const left = planes[0]!;
        const right = planes[1]!;
        for (let index = 0; index < 128; index += 1) {
          const value = TONE_AMPLITUDE * Math.sin(phase);
          left[index] = value;
          right[index] = value;
          phase += step;
        }
        phase %= 2 * Math.PI;
        writer.commit({ generation: 1n, startFrame: next, frames: 128, endOfRegion: false });
        next += 128n;
      }
    };
    fill();
    timer = setInterval(fill, 5);

    const button = document.createElement("button");
    button.setAttribute("data-soak-resume", request.resumeLabel);
    button.textContent = `Resume soak: ${request.resumeLabel}`;
    document.body.append(button);
    await new Promise<void>((resolve, reject) => {
      button.addEventListener("click", () => context.resume().then(resolve, reject), { once: true });
    });
    button.remove();
    const startedAt = performance.now();
    const warmupEndsAt = startedAt + request.warmupSeconds * 1_000;
    const endsAt = startedAt + request.seconds * 1_000;

    const telemetry = {
      windows: 0, blocks: 0, misses: 0,
      windowsAfterWarmup: 0, blocksAfterWarmup: 0, missesAfterWarmup: 0,
      peakBlockMs: 0, peakBlockMsAfterWarmup: 0, cpuPercentSum: 0, budgetMs: 0, resolutionMs: 0, belowResolution: 0,
    };
    unsubscribe = await live.subscribeTelemetry((update) => {
      const after = performance.now() >= warmupEndsAt;
      telemetry.windows += 1;
      telemetry.blocks += update.blocks;
      telemetry.misses += update.deadlineMisses;
      telemetry.peakBlockMs = Math.max(telemetry.peakBlockMs, update.peakBlockMs);
      telemetry.budgetMs = update.budgetMs;
      telemetry.resolutionMs = update.resolutionMs;
      if (update.belowResolution) telemetry.belowResolution += 1;
      if (after) {
        telemetry.windowsAfterWarmup += 1;
        telemetry.blocksAfterWarmup += update.blocks;
        telemetry.missesAfterWarmup += update.deadlineMisses;
        telemetry.peakBlockMsAfterWarmup = Math.max(telemetry.peakBlockMsAfterWarmup, update.peakBlockMs);
        telemetry.cpuPercentSum += update.cpuPercent;
      }
    });

    const control = new Int32Array(ring, 0, MSB1_CONTROL_BYTES / 4);
    const samples: SoakSample[] = [];
    const sample = async () => {
      const wallSeconds = (performance.now() - startedAt) / 1_000;
      const contextSeconds = context.currentTime;
      const status = await live.host.status();
      const report = await live.host.seekReport(sourceId);
      if (report.result !== 0 || report.report === null) throw new Error(`seek report refused: ${report.result}`);
      samples.push({
        wallSeconds, contextSeconds, afterWarmup: performance.now() >= warmupEndsAt,
        memoryBytes: status.memoryBytes, renderedQuanta: status.renderedQuanta.toString(),
        lastResult: status.lastResult, blockingRebuilds: status.blockingRebuilds,
        singleModeControlAllocations: status.singleModeControlAllocations,
        underrunFrames: report.report.cumulativeUnderrunFrames.toString(),
        ringUnderruns: Atomics.load(control, MSB1_CONTROL.UNDERRUNS),
        contextState: context.state,
      });
    };
    await sample();
    // One sample at the warm-up instant, one every interval after it, and one at the end.
    const instants = [warmupEndsAt];
    for (let at = warmupEndsAt + request.sampleIntervalSeconds * 1_000; at < endsAt; at += request.sampleIntervalSeconds * 1_000) {
      instants.push(at);
    }
    if (endsAt > instants[instants.length - 1]!) instants.push(endsAt);
    for (const at of instants) {
      for (let now = performance.now(); now < at; now = performance.now()) await wait(Math.min(at - now, 1_000));
      await sample();
    }
    unsubscribe();
    unsubscribe = undefined;
    const allocations = await live.host.renderAllocationCount();
    return {
      instanceMode: live.instanceMode,
      contextSampleRate: context.sampleRate,
      renderAllocations: allocations,
      telemetry,
      samples,
      feed: { refused: Atomics.load(control, MSB1_CONTROL.REFUSED), lastRefusal: Atomics.load(control, MSB1_CONTROL.LAST_RESULT) },
    };
  } finally {
    unsubscribe?.();
    if (timer !== undefined) clearInterval(timer);
    feed?.close();
    if (engine !== undefined) await engine.close();
    else if (context.state !== "closed") await context.close();
  }
}
