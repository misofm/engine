import { WasmBoundary } from "../../../sdk/src/core/boundary.ts";
import { EngineConsole } from "../../../sdk/src/core/console.ts";
import { encodeLaneEdits } from "../../../sdk/src/core/writer.ts";
import { MisoEngineAsset } from "../../../sdk/src/core/asset.ts";
import { ABI_LAYOUT } from "../../../sdk/src/generated/abi.ts";
import { CATALOG } from "../../../sdk/src/generated/catalog.ts";
import { effect } from "../../../sdk/src/core/session.ts";
import type { ObservationReadResult } from "../../../sdk/src/core/observation.ts";
import type { TrackResponseResult } from "../../../sdk/src/core/live-response.ts";
import { createResponsePreview } from "../../../sdk/src/browser/response.ts";
import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { createDefaultHost } from "../../../sdk/src/browser/default-host.ts";

const EQ_CONFIGURATION_ID = 9_007_199_254_740_993n;
const OBSERVATION_FRAMES = 2_048;
const SPECTRUM_FRAMES = 2_048;
const CONTINUOUS_FRAMES = 6_144;
const CONTINUOUS_BLOCKS = CONTINUOUS_FRAMES / 128;
const PEAK_FREQUENCY_HZ = 750;
const PEAK_LEFT_AMPLITUDE = 0.1;
const PEAK_RIGHT_AMPLITUDE = 0.05;
const PEAK_EQ_GAIN_DB = 6;

const SPECTRUM_QUERIES = [
  {
    target: { kind: "trackPostInputBuiltins" as const, trackId: "track" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  },
  {
    target: { kind: "trackPostMatrix" as const, trackId: "track" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  },
  {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  },
];

const SPECTRUM_COLLECTION_ENTRIES = [
  {
    target: { kind: "trackPostMatrix" as const, trackId: "track-a" },
    channels: "both" as const,
  },
  {
    target: { kind: "trackPostMatrix" as const, trackId: "track-b" },
    channels: "left" as const,
  },
];

const SPECTRUM_COLLECTION = {
  entries: SPECTRUM_COLLECTION_ENTRIES,
  maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes * 2,
};

function observationSelection(effectSlotId: string, channels: "left" | "right" | "both" = "both") {
  return { trackId: "track", rack: "dynamic" as const, effectSlotId, tapId: 1, channels };
}

function observationPlanes(block: number): Float32Array[] {
  const left = new Float32Array(128);
  const right = new Float32Array(128);
  left.fill(0.5 + block * 0.001);
  right.fill(0.5 + block * 0.001);
  return [left, right];
}

function observationRowKey(row: ObservationReadResult): string {
  const window = row.window;
  return JSON.stringify([
    row.effectSlotId,
    row.channels,
    row.status,
    row.left,
    row.right,
    window?.firstSample.toString(),
    window?.endSample.toString(),
    window?.sequence.toString(),
    window?.blocks,
  ]);
}

function observationRowsEqual(
  left: readonly ObservationReadResult[],
  right: readonly ObservationReadResult[],
): boolean {
  return left.length === right.length && left.every((row, index) => {
    const other = right[index];
    return other !== undefined && observationRowKey(row) === observationRowKey(other);
  });
}

function serializeObservationRows(rows: readonly ObservationReadResult[]) {
  return rows.map((row) => ({
    effectSlotId: row.effectSlotId,
    nativeEffectId: row.nativeEffectId,
    channels: row.channels,
    status: row.status,
    left: row.left,
    right: row.right,
    window: row.window === undefined ? null : {
      firstSample: row.window.firstSample.toString(),
      endSample: row.window.endSample.toString(),
      sequence: row.window.sequence.toString(),
      blocks: row.window.blocks,
    },
  }));
}

/** Keep the browser probe on the same compressor/gate fixture as the headless evals. */
function observationDocumentWithGate(raw: string): Uint8Array {
  const document = JSON.parse(raw);
  const track = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  const gate = CATALOG.effects.find((candidate) => candidate.id === "miso.gate-expander");
  if (track === undefined || gate === undefined) throw new Error("observation gate fixture is unavailable");
  const gateEntry = {
    id: "gate",
    identity: { kind: "native", effect_id: gate.id },
    quality: "normal",
    bypass: false,
    link_mode: "dual_mono",
    params: gate.parameters.map((parameter) => ({
      parameter_id: parameter.id,
      channel: "both",
      unit: parameter.unitName,
      value: parameter.default,
    })),
    sidechain: { kind: "none" },
  };
  track.dynamic.effects = [
    ...track.dynamic.effects.filter((entry: { id?: string }) => entry.id !== "gate"),
    gateEntry,
  ];
  return new TextEncoder().encode(JSON.stringify(document));
}

function responseArrayEqual(left: ArrayLike<number> | undefined, right: ArrayLike<number> | undefined): boolean {
  if (left === undefined || right === undefined) return left === undefined && right === undefined;
  if (left.length !== right.length) return false;
  for (let index = 0; index < left.length; index += 1) {
    if (left[index] !== right[index]) return false;
  }
  return true;
}

function responseMemberKey(member: TrackResponseResult["members"][number]): string {
  return JSON.stringify([
    member.trackId,
    member.nativeId,
    member.stableId,
    member.rack,
    member.rackValue,
    member.slot,
    member.kind,
    member.kindValue,
    member.bypassed,
    member.available,
    member.excludedReason,
    member.enabledLeft,
    member.enabledRight,
  ]);
}

function responseValuesEqual(left: TrackResponseResult, right: TrackResponseResult): boolean {
  return left.trackId === right.trackId
    && left.mode === right.mode
    && left.meaning === right.meaning
    && left.sampleRateHz === right.sampleRateHz
    && left.floorDb === right.floorDb
    && responseArrayEqual(left.frequenciesHz, right.frequenciesHz)
    && responseArrayEqual(left.leftDb, right.leftDb)
    && responseArrayEqual(left.rightDb, right.rightDb)
    && left.excludedMemberCount === right.excludedMemberCount
    && left.members.length === right.members.length
    && left.members.every((member, index) => responseMemberKey(member) === responseMemberKey(right.members[index]!));
}

function serializeTrackResponse(result: TrackResponseResult) {
  return {
    trackId: result.trackId,
    mode: result.mode,
    meaning: result.meaning,
    sampleRateHz: result.sampleRateHz,
    points: result.frequenciesHz.length,
    frequencies: Array.from(result.frequenciesHz),
    left: result.leftDb === undefined ? undefined : Array.from(result.leftDb),
    right: result.rightDb === undefined ? undefined : Array.from(result.rightDb),
    members: result.members.map((member) => ({
      nativeId: member.nativeId,
      stableId: member.stableId,
      rack: member.rack,
      slot: member.slot,
      kind: member.kind,
      available: member.available,
      bypassed: member.bypassed,
      excludedReason: member.excludedReason,
      enabledLeft: Array.from(member.enabledLeft),
      enabledRight: Array.from(member.enabledRight),
    })),
    capturedSample: result.capturedSample.toString(),
    snapshotToken: result.snapshotToken.toString(),
    resultBytes: result.resultBytes.toString(),
  };
}

function countingTrackResponseWorker(stats: { queries: number; initializations: number }) {
  return (url: URL, options: { readonly type: "module" }) => {
    const worker = new Worker(url, options);
    return {
      postMessage(message: { readonly type?: string }, transfer?: readonly Transferable[]) {
        if (message.type === "track-response-init") stats.initializations += 1;
        if (message.type === "track-response-query") stats.queries += 1;
        worker.postMessage(message, transfer);
      },
      terminate: () => worker.terminate(),
      addEventListener: (type: "message" | "error" | "messageerror", listener: (event: any) => void) =>
        worker.addEventListener(type, listener),
      removeEventListener: (type: "message" | "error" | "messageerror", listener: (event: any) => void) =>
        worker.removeEventListener(type, listener),
    };
  };
}

function spectrumPlanes(block: number): Float32Array[] {
  const left = new Float32Array(128);
  const right = new Float32Array(128);
  left.fill(0.25 + block * 0.002);
  right.fill(-0.5 - block * 0.001);
  return [left, right];
}

function spectrumTargetKey(target: typeof SPECTRUM_QUERIES[number]["target"]): string {
  return target.kind === "output"
    ? `${target.kind}:${target.outputId}`
    : `${target.kind}:${target.trackId}`;
}

async function createSpectrumBrowser(query: typeof SPECTRUM_QUERIES[number]) {
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createEngine({
    document,
    spectrum: query,
    policy: { sourceRingFrames: SPECTRUM_FRAMES },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: SPECTRUM_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(SPECTRUM_FRAMES) }],
      tracks: ["track"],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, SPECTRUM_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  return browser;
}

function spectrumDocument(raw: string, frames: number, peak = false): Uint8Array {
  const document = JSON.parse(raw);
  const source = document.sources?.find((candidate: { id?: string }) => candidate.id === "console-source");
  const track = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  if (source === undefined || track === undefined) throw new Error("spectrum fixture is unavailable");
  source.frames = String(frames);
  if (peak) {
    for (const lane of [track.builtins?.left, track.builtins?.right]) {
      if (lane === undefined) throw new Error("spectrum peak fixture has no builtins");
      lane.hpf_hz = 0;
      lane.lpf_hz = 0;
      lane.delay_samples = 0;
      lane.trim_db = 0;
      lane.polarity_invert = false;
    }
    const eq = track.simd1?.effects?.find((entry: { identity?: { effect_id?: string } }) =>
      entry.identity?.effect_id === "miso.parametric-eq");
    const definition = CATALOG.effects.find((candidate) => candidate.id === "miso.parametric-eq");
    if (eq === undefined || definition === undefined) throw new Error("spectrum peak EQ fixture is unavailable");
    const names = new Map([
      ["band-1-enabled", 1],
      ["band-1-kind", 1],
      ["band-1-frequency", PEAK_FREQUENCY_HZ],
      ["band-1-gain", PEAK_EQ_GAIN_DB],
    ]);
    eq.params = definition.parameters
      .filter((parameter) => names.has(parameter.name) || parameter.name === "band-1-q")
      .map((parameter) => ({
        parameter_id: parameter.id,
        channel: "both",
        unit: parameter.unitName,
        value: names.get(parameter.name) ?? parameter.default,
      }));
    track.dynamic.effects = [];
    track.simd2.effects = [];
  }
  return new TextEncoder().encode(JSON.stringify(document));
}

/** Build the same known-signal fixture with two prepared, differently tuned track boundaries. */
function spectrumCollectionDocument(raw: string, frames: number): Uint8Array {
  const document = JSON.parse(new TextDecoder().decode(spectrumDocument(raw, frames, true)));
  const source = document.sources?.find((candidate: { id?: string }) => candidate.id === "console-source");
  const original = document.tracks?.find((candidate: { id?: string }) => candidate.id === "track");
  const route = document.routes?.[0];
  if (source === undefined || original === undefined || route === undefined) {
    throw new Error("spectrum collection fixture is unavailable");
  }
  const trackA = structuredClone(original);
  const trackB = structuredClone(original);
  trackA.id = "track-a";
  trackB.id = "track-b";
  // The input is the same 750 Hz signal for both tracks; this trim makes a selected B
  // window distinguishable without changing the capture boundary or adding a DSP fixture.
  for (const lane of [trackB.builtins?.left, trackB.builtins?.right]) {
    if (lane === undefined) throw new Error("spectrum collection track has no builtins");
    lane.trim_db = -6;
  }
  document.tracks = [trackA, trackB];
  document.routes = [
    { ...structuredClone(route), id: "track-a-main", source: {
      ...structuredClone(route.source), track_id: "track-a",
    } },
    { ...structuredClone(route), id: "track-b-main", source: {
      ...structuredClone(route.source), track_id: "track-b",
    } },
  ];
  return new TextEncoder().encode(JSON.stringify(document));
}

async function createContinuousSpectrumBrowser(
  query,
  frames = CONTINUOUS_FRAMES,
  peak = false,
  spectrumHopFrames?: 256 | 512 | 1024 | 2048,
  live = false,
) {
  const raw = await (await fetch("/qualification/observation-session.json")).text();
  const document = spectrumDocument(raw, frames, peak);
  const browser = await createEngine({
    document,
    spectrum: query,
    policy: {
      sourceRingFrames: frames,
      console: { commandQueueRecords: 64, meterBlocks: 16 },
      ...(spectrumHopFrames === undefined ? {} : { spectrumHopFrames }),
    },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: frames,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(frames) }],
      tracks: ["track"],
    }),
    createContext: () => {
      const context = live
        ? new AudioContext({ sampleRate: 48_000, latencyHint: "interactive" })
        : new OfflineAudioContext(2, frames, 48_000);
      // Firefox can create a later context already running after this page's first trusted
      // resume. Keep a suspension pending across addModule so the shipped host observes its
      // required suspended preparation state, matching the collection qualification below.
      if (live) void (context as AudioContext).suspend();
      else Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  return browser;
}

function streamMetadataFields(metadata) {
  if (metadata === undefined) return undefined;
  return {
    status: metadata.status,
    hopFrames: metadata.hopFrames,
    sampleRateHz: metadata.sampleRateHz,
    quantumFrames: metadata.quantumFrames,
    smoothingMs: metadata.smoothingMs,
    capturedSample: metadata.capturedSample.toString(),
    endSample: metadata.endSample.toString(),
    sequence: metadata.sequence.toString(),
    windows: metadata.windows.toString(),
    droppedCaptures: metadata.droppedCaptures.toString(),
  };
}

async function createSpectrumCollectionBrowser() {
  const raw = await (await fetch("/qualification/observation-session.json")).text();
  const frames = 16 * SPECTRUM_FRAMES;
  const document = spectrumCollectionDocument(raw, frames);
  const browser = await createEngine({
    document,
    spectrumCollection: SPECTRUM_COLLECTION,
    policy: { sourceRingFrames: frames },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: frames,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(frames) }],
      tracks: ["track-a", "track-b"],
    }),
    createContext: () => {
      const context = new AudioContext({ sampleRate: 48_000, latencyHint: "interactive" });
      // Keep the known first capture at sample zero even when autoplay is permitted.
      void context.suspend();
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  return browser;
}

async function submitSpectrumSource(browser, frames = SPECTRUM_FRAMES, planeForBlock = spectrumPlanes) {
  for (let block = 0; block < frames / 128; block += 1) {
    const acknowledgement = await browser.host.submitSource({
      sourceId: "console-source",
      generation: 1n,
      startFrame: BigInt(block * 128),
      sampleRateHz: 48_000,
      planes: planeForBlock(block, block * 128),
      frames: 128,
      endOfRegion: block === frames / 128 - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("SDK spectrum source submission refused");
  }
}

function toneSpectrumPlanes(block: number, startFrame: number): Float32Array[] {
  const scale = block < 2 * SPECTRUM_FRAMES / 128 ? 1 : 0.5;
  return [PEAK_LEFT_AMPLITUDE * scale, PEAK_RIGHT_AMPLITUDE * scale].map((amplitude) =>
    Float32Array.from({ length: 128 }, (_, index) => amplitude * Math.sin(
      2 * Math.PI * PEAK_FREQUENCY_HZ * (startFrame + index) / 48_000,
    )));
}

function collectionSpectrumPlanes(_block: number, startFrame: number): Float32Array[] {
  return [PEAK_LEFT_AMPLITUDE, PEAK_RIGHT_AMPLITUDE].map((amplitude) =>
    Float32Array.from({ length: 128 }, (_, index) => amplitude * Math.sin(
      2 * Math.PI * PEAK_FREQUENCY_HZ * (startFrame + index) / 48_000,
    )));
}

function spectrumPeak(values: Float32Array): { readonly index: number; readonly value: number } {
  let index = 0;
  for (let candidate = 1; candidate < values.length; candidate += 1) {
    if (values[candidate]! > values[index]!) index = candidate;
  }
  return { index, value: values[index]! };
}

async function runContinuousSpectrumQualification(): Promise<Record<string, unknown>> {
  const query = {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  };
  const browser = await createContinuousSpectrumBrowser(query, CONTINUOUS_FRAMES, true, 256);
  const nativeStarts = [];
  const nativeReads = [];
  const nativeStartSpectrumStream = browser.host.startSpectrumStream.bind(browser.host);
  browser.host.startSpectrumStream = async (...args) => {
    const reply = await nativeStartSpectrumStream(...args);
    nativeStarts.push(reply.metadata);
    return reply;
  };
  const nativeReadSpectrumStream = browser.host.readSpectrumStream.bind(browser.host);
  browser.host.readSpectrumStream = async (...args) => {
    const reply = await nativeReadSpectrumStream(...args);
    nativeReads.push(reply.metadata);
    return reply;
  };
  const meterFrames: Array<{ readonly peaks: Float32Array; readonly firstSample: bigint; readonly endSample: bigint }> = [];
  const automaticNotifications = [];
  let callbackCount = 0;
  let callbackGap = false;
  let callbackResolve: (() => void) | undefined;
  const callbackSeen = new Promise<void>((resolve) => { callbackResolve = resolve; });
  let subscription;
  let shared;
  try {
    const meterLease = await browser.host.meters({
      enabled: true,
      onFrame: (frame) => meterFrames.push(frame),
    });
    if (meterLease.result !== 0) throw new Error("continuous spectrum meter lease refused");
    const response = await browser.queryTrackResponse({
      trackId: "track",
      grid: { kind: "linear", points: 1_025, minimumHz: 0, maximumHz: 24_000 },
      channels: "both",
      responseLimits: { maximumResultBytes: 1 << 20, requestDeadlineMs: 5_000 },
    });
    subscription = await browser.subscribeSpectrum({
      ...query,
      smoothingMs: 0,
      cadenceMs: 1,
      onUpdate: (notification) => {
        automaticNotifications.push(notification);
        callbackCount += 1;
        callbackGap ||= notification.nativeMissedWindows > 0n || notification.skippedPublications > 0n;
        callbackResolve?.();
      },
    });
    shared = await browser.subscribeSpectrum({ ...query, smoothingMs: 0, cadenceMs: 100 });
    const pendingBeforeRender = subscription.readLatest() === undefined;
    const sharedJob = shared.job === subscription.job
      && shared.owner === subscription.owner && shared.epoch === subscription.epoch;
    await submitSpectrumSource(browser, CONTINUOUS_FRAMES, toneSpectrumPlanes);
    const rendered = await browser.context.startRendering();
    const firstPcmPeak = Math.max(
      ...[rendered.getChannelData(0), rendered.getChannelData(1)].map((plane) => {
        let peak = 0;
        for (const value of plane) peak = Math.max(peak, Math.abs(value));
        return peak;
      }),
    );
    const automaticDelivery = await Promise.race([
      callbackSeen.then(() => true),
      new Promise<boolean>((resolve) => setTimeout(() => resolve(false), 100)),
    ]);
    // Both automatic reads and explicit pumps invoke onUpdate. Keep its live list: the first
    // callback can report a gap while the same poll is still draining its recovery window.
    const notifications = automaticNotifications;
    for (let attempt = 0; attempt < CONTINUOUS_BLOCKS; attempt += 1) {
      const notification = await subscription.pump();
      if (notification?.status === "gap" && subscription.readLatest() !== undefined) break;
    }
    const first = subscription.readLatest();
    if (first === undefined) throw new Error("continuous spectrum did not publish a window");
    const firstLeft = first.leftDb;
    const firstRight = first.rightDb;
    if (firstLeft === undefined || firstRight === undefined) throw new Error("continuous spectrum omitted a channel");
    const effectiveHopFrames = subscription.bounds.hopFrames;
    const leftPeak = spectrumPeak(firstLeft);
    const rightPeak = spectrumPeak(firstRight);
    const sharedResult = shared.readLatest();
    if (sharedResult === undefined) throw new Error("shared continuous spectrum had no result");
    const original = sharedResult.leftDb?.[0];
    if (original === undefined || subscription.readLatest()?.leftDb?.[0] !== original) {
      throw new Error("continuous spectrum did not preserve owned arrays");
    }
    sharedResult.leftDb![0] = original + 100;
    const ownedArrays = subscription.readLatest()?.leftDb?.[0] === original;
    const secondNotification = notifications.at(-1);
    const gap = secondNotification?.status === "gap"
      || secondNotification?.nativeMissedWindows > 0n
      || secondNotification?.skippedPublications > 0n
      || callbackGap;
    const readyNotification = notifications.find((notification) => notification.available);
    const sharedAfterFirstClose = (await subscription.close(), shared.readLatest() !== undefined);
    await shared.close();
    let staleReadRefused = false;
    try { shared.readLatest(); } catch { staleReadRefused = true; }
    const meter = meterFrames.find((frame) => frame.peaks.length >= 4);
    const expectedLeft = PEAK_LEFT_AMPLITUDE * 10 ** (PEAK_EQ_GAIN_DB / 20);
    const expectedRight = PEAK_RIGHT_AMPLITUDE * 10 ** (PEAK_EQ_GAIN_DB / 20);
    const expectedLeftDb = 20 * Math.log10(PEAK_LEFT_AMPLITUDE) + PEAK_EQ_GAIN_DB;
    const expectedRightDb = 20 * Math.log10(PEAK_RIGHT_AMPLITUDE) + PEAK_EQ_GAIN_DB;
    return {
      pendingBeforeRender,
      sharedJob,
      callbackCount,
      automaticDelivery,
      windows: notifications.length,
      statuses: notifications.map((notification) => notification.status),
      sequences: notifications.map((notification) => notification.metadata.sequence.toString()),
      hopFrames: effectiveHopFrames,
      nativeStart: streamMetadataFields(nativeStarts[0]),
      nativeReadHopFrames: nativeReads.map((metadata) => metadata.hopFrames),
      publicationHopFrames: notifications.map((notification) => notification.metadata.hopFrames),
      publicationSmoothingMs: notifications.map((notification) => notification.metadata.smoothingMs),
      gap,
      ownedArrays,
      sharedAfterFirstClose,
      staleReadRefused,
      first: {
        capturedSample: first.capturedSample.toString(),
        endSample: first.endSample.toString(),
        sequence: readyNotification?.metadata.sequence.toString() ?? "",
        snapshotToken: first.snapshotToken.toString(),
        hopFrames: notifications.find((notification) => notification.available
          && notification.metadata.capturedSample === first.capturedSample)?.metadata.hopFrames,
        smoothingMs: notifications.find((notification) => notification.available
          && notification.metadata.capturedSample === first.capturedSample)?.metadata.smoothingMs,
        peakBins: [leftPeak.index, rightPeak.index],
        peakHz: [first.frequenciesHz[leftPeak.index], first.frequenciesHz[rightPeak.index]],
        peakDbfs: [leftPeak.value, rightPeak.value],
        expectedPeakDbfs: [expectedLeftDb, expectedRightDb],
        responseHz: response.frequenciesHz[32],
        responseGainDb: [response.leftDb[32], response.rightDb[32]],
        sampleRateHz: first.sampleRateHz,
        windowFrames: first.windowFrames,
        binCount: first.binCount,
        sourceUnderrun: first.graphSourceUnderrun,
        firstPcmPeak,
        expectedLinearPeaks: [expectedLeft, expectedRight],
        meterPeaks: meter === undefined ? [] : Array.from(meter.peaks),
        meterSpan: meter === undefined ? [] : [meter.firstSample.toString(), meter.endSample.toString()],
      },
    };
  } finally {
    await shared?.close();
    await subscription?.close();
    await browser.close();
  }
}

/** A short live probe drains two overlapping H1024 windows through the real Worklet queue. */
async function runConfiguredSpectrumHopQualification(): Promise<Record<string, unknown>> {
  const query = {
    target: { kind: "output" as const, outputId: "main-out" },
    channels: "both" as const,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  };
  const hopFrames = 1_024;
  const smoothingMs = 37.5;
  const frames = 8 * SPECTRUM_FRAMES;
  const browser = await createContinuousSpectrumBrowser(query, frames, false, hopFrames, true);
  const nativeStarts = [];
  const nativeReads = [];
  const nativeStartSpectrumStream = browser.host.startSpectrumStream.bind(browser.host);
  browser.host.startSpectrumStream = async (...args) => {
    const reply = await nativeStartSpectrumStream(...args);
    nativeStarts.push(reply.metadata);
    return reply;
  };
  const nativeReadSpectrumStream = browser.host.readSpectrumStream.bind(browser.host);
  browser.host.readSpectrumStream = async (...args) => {
    const reply = await nativeReadSpectrumStream(...args);
    nativeReads.push(reply.metadata);
    return reply;
  };
  let subscription;
  const notifications = [];
  const publications = [];
  const publicationKeys = new Set();
  const remember = (notification) => {
    notifications.push(notification);
    if (!notification.available || subscription === undefined) return;
    const result = subscription.readLatest();
    if (result === undefined || result.capturedSample !== notification.metadata.capturedSample) return;
    const key = `${notification.metadata.captureEpoch}:${notification.metadata.sequence}`;
    if (publicationKeys.has(key)) return;
    publicationKeys.add(key);
    publications.push({ metadata: notification.metadata, result });
  };
  try {
    subscription = (await browser.subscribeSpectrum({
      ...query,
      smoothingMs,
      cadenceMs: 1,
      onUpdate: remember,
    })).handle;
    await submitSpectrumSource(browser, frames);
    const context = browser.context as AudioContext;
    const resumeButton = document.createElement("button");
    resumeButton.id = "spectrum-hop-resume";
    resumeButton.textContent = "Start configured spectrum qualification audio";
    document.body.append(resumeButton);
    let resumeTimer: ReturnType<typeof setTimeout> | undefined;
    let resumeGesture = false;
    try {
      await new Promise<void>((resolve, reject) => {
        resumeTimer = setTimeout(() => reject(new Error(
          `configured spectrum audio resume timed out (gesture=${resumeGesture}, state=${context.state})`,
        )), 10_000);
        resumeButton.addEventListener("click", () => {
          resumeGesture = true;
          void context.resume().then(resolve, reject);
        }, { once: true });
      });
    } finally {
      if (resumeTimer !== undefined) clearTimeout(resumeTimer);
      resumeButton.remove();
    }
    const deadline = performance.now() + 10_000;
    for (let attempt = 0; attempt < 512 && publications.length < 2; attempt += 1) {
      await subscription.pump();
      if (performance.now() >= deadline) break;
    }
    if (publications.length < 2) {
      throw new Error(`H1024 browser probe published ${publications.length} windows`);
    }
    const first = publications[0];
    const second = publications[1];
    const firstResult = first.result;
    const secondResult = second.result;
    const firstFrequencies = firstResult.frequenciesHz.slice();
    const firstLeft = firstResult.leftDb?.slice();
    const firstRight = firstResult.rightDb?.slice();
    secondResult.leftDb?.set([secondResult.leftDb[0]! + 100], 0);
    const ownedArrays = firstResult.frequenciesHz !== secondResult.frequenciesHz
      && firstResult.leftDb !== secondResult.leftDb
      && firstResult.rightDb !== secondResult.rightDb
      && firstResult.frequenciesHz.every((value, index) => value === firstFrequencies[index])
      && (firstLeft === undefined || firstResult.leftDb?.every((value, index) => value === firstLeft[index]))
      && (firstRight === undefined || firstResult.rightDb?.every((value, index) => value === firstRight[index]));
    return {
      requestHopFrames: hopFrames,
      smoothingMs,
      boundsHopFrames: subscription.bounds.hopFrames,
      nativeStart: streamMetadataFields(nativeStarts[0]),
      nativeRead: nativeReads.slice(0, 4).map(streamMetadataFields),
      publicationCount: publications.length,
      notificationHops: notifications.map((notification) => notification.metadata.hopFrames),
      notificationSmoothingMs: notifications.map((notification) => notification.metadata.smoothingMs),
      publications: publications.slice(0, 2).map(({ metadata, result }) => ({
        metadata: streamMetadataFields(metadata),
        result: {
          capturedSample: result.capturedSample.toString(),
          endSample: result.endSample.toString(),
          windowFrames: result.windowFrames,
          binCount: result.binCount,
          finite: Array.from(result.frequenciesHz).every(Number.isFinite)
            && Array.from(result.leftDb ?? []).every(Number.isFinite)
            && Array.from(result.rightDb ?? []).every(Number.isFinite),
          ownedArrays: result.frequenciesHz !== result.leftDb
            && result.frequenciesHz !== result.rightDb
            && result.leftDb !== result.rightDb,
        },
      })),
      startDelta: (secondResult.capturedSample - firstResult.capturedSample).toString(),
      firstSpan: (firstResult.endSample - firstResult.capturedSample).toString(),
      secondSpan: (secondResult.endSample - secondResult.capturedSample).toString(),
      resultHopFrames: first.metadata.hopFrames,
      resultSmoothingMs: first.metadata.smoothingMs,
      ownedArrays,
    };
  } finally {
    await subscription?.close();
    await browser.close();
  }
}

async function querySpectrumAfterArm(browser, query) {
  let acknowledgeArm;
  let rejectArm;
  const armed = new Promise((resolve, reject) => {
    acknowledgeArm = resolve;
    rejectArm = reject;
  });
  const originalArm = browser.host.armSpectrum.bind(browser.host);
  browser.host.armSpectrum = async () => {
    const reply = await originalArm();
    if (reply.result === 0) acknowledgeArm();
    else rejectArm(new Error(`spectrum arm refused with result ${reply.result}`));
    return reply;
  };
  const pending = browser.querySpectrum(query);
  await armed;
  await browser.context.startRendering();
  return pending;
}

function serializeSpectrum(result, query) {
  const frequencies = Array.from(result.frequenciesHz);
  const left = result.leftDb === undefined ? [] : Array.from(result.leftDb);
  const right = result.rightDb === undefined ? [] : Array.from(result.rightDb);
  const finite = frequencies.every(Number.isFinite)
    && left.every(Number.isFinite) && right.every(Number.isFinite);
  return {
    target: result.target,
    targetKey: spectrumTargetKey(query.target),
    channels: result.channels,
    sampleRateHz: result.sampleRateHz,
    windowFrames: result.windowFrames,
    binCount: result.binCount,
    floorDb: result.floorDb,
    frequencies,
    left,
    right,
    capturedSample: result.capturedSample.toString(),
    endSample: result.endSample.toString(),
    snapshotToken: result.snapshotToken.toString(),
    sourceUnderrun: result.graphSourceUnderrun,
    resultBytes: result.resultBytes.toString(),
    finite,
    ownedArrays: result.frequenciesHz !== result.leftDb
      && result.frequenciesHz !== result.rightDb
      && result.leftDb !== result.rightDb,
  };
}

async function runSdkSpectrumQualification(): Promise<Record<string, unknown>> {
  const rows = [];
  let busyRefused = false;
  for (const query of SPECTRUM_QUERIES) {
    const browser = await createSpectrumBrowser(query);
    try {
      await submitSpectrumSource(browser);
      const pending = querySpectrumAfterArm(browser, query);
      if (rows.length === 0) {
        try {
          await browser.querySpectrum(query);
        } catch {
          busyRefused = true;
        }
      }
      const result = await pending;
      const frequencies = result.frequenciesHz.slice();
      const left = result.leftDb?.slice();
      const right = result.rightDb?.slice();
      const row = { ...serializeSpectrum(result, query), ownedAfterClose: false };
      await browser.close();
      row.ownedAfterClose = frequencies.every(
        (value, index) => value === row.frequencies[index],
      ) && (left === undefined || left.every(
        (value, index) => value === row.left[index],
      )) && (right === undefined || right.every(
        (value, index) => value === row.right[index],
      ));
      rows.push(row);
    } finally {
      await browser.close();
    }
  }

  const closedBrowser = await createSpectrumBrowser(SPECTRUM_QUERIES[0]);
  let closedRefused = false;
  try {
    await submitSpectrumSource(closedBrowser);
    let acknowledgeArm;
    const armed = new Promise((resolve) => { acknowledgeArm = resolve; });
    const originalArm = closedBrowser.host.armSpectrum.bind(closedBrowser.host);
    closedBrowser.host.armSpectrum = async () => {
      const reply = await originalArm();
      if (reply.result === 0) acknowledgeArm();
      return reply;
    };
    const pending = closedBrowser.querySpectrum(SPECTRUM_QUERIES[0]);
    await armed;
    await closedBrowser.close();
    try {
      await pending;
    } catch {
      closedRefused = true;
    }
  } finally {
    await closedBrowser.close();
  }

  return {
    targets: rows,
    targetKeys: rows.map((row) => row.targetKey),
    allTargetsDistinct: new Set(rows.map((row) => row.targetKey)).size === 3,
    postChainDiffers: rows[0]?.left.some((value, index) => value !== rows[1]?.left[index]) === true,
    windows: rows.map((row) => [row.capturedSample, row.endSample, row.windowFrames]),
    allFinite: rows.every((row) => row.finite),
    ownedArrays: rows.every((row) => row.ownedArrays && row.ownedAfterClose),
    busyRefused,
    closedRefused,
    continuous: await runContinuousSpectrumQualification(),
    collection: await runSpectrumCollectionQualification(),
    configuredHop: await runConfiguredSpectrumHopQualification(),
  };
}

async function runSpectrumCollectionQualification(): Promise<Record<string, unknown>> {
  const browser = await createSpectrumCollectionBrowser();
  const [entryA, entryB] = SPECTRUM_COLLECTION_ENTRIES;
  if (entryA === undefined || entryB === undefined) throw new Error("spectrum collection fixture is incomplete");
  const query = (entry: typeof entryA) => ({
    ...entry,
    spectrumLimits: { maximumCaptureBytes: ABI_LAYOUT.constants.spectrumCaptureBytes },
  });
  const firstQuery = query(entryA);
  const secondQuery = query(entryB);
  const readinessTimeoutMs = 10_000;
  const totalFrames = 16 * SPECTRUM_FRAMES;
  const totalBlocks = totalFrames / 128;
  const submitBlocks = async (firstBlock: number, lastBlock: number, endOfRegion: boolean) => {
    for (let block = firstBlock; block < lastBlock; block += 1) {
      let acknowledgement;
      for (let retry = 0; retry < 200; retry += 1) {
        acknowledgement = await browser.host.submitSource({
          sourceId: "console-source",
          generation: 1n,
          startFrame: BigInt(block * 128),
          sampleRateHz: 48_000,
          planes: collectionSpectrumPlanes(block, block * 128),
          frames: 128,
          endOfRegion: endOfRegion && block === lastBlock - 1,
        });
        if (acknowledgement.result === 0) break;
        if (acknowledgement.result !== 6) {
          throw new Error(`SDK spectrum collection source submission refused (${acknowledgement.result}) at block ${block}`);
        }
        await new Promise((resolve) => globalThis.setTimeout(resolve, 5));
      }
      if (acknowledgement?.result !== 0) {
        throw new Error(`SDK spectrum collection source submission remained backpressured at block ${block}`);
      }
    }
  };
  const waitForResult = async (subscription, entry) => {
    const targetKey = spectrumTargetKey(entry.target);
    let lastNotification;
    const startedAt = performance.now();
    // A running live context can still be starting its output backend. Poll count
    // is not rendered progress: use the same bounded readiness budget as resume.
    while (performance.now() - startedAt < readinessTimeoutMs) {
      const current = subscription.readLatest();
      if (current !== undefined
          && spectrumTargetKey(current.target) === targetKey
          && current.channels === (entry.channels ?? "both")) {
        return current;
      }
      lastNotification = await subscription.pump() ?? lastNotification;
      await new Promise((resolve) => globalThis.setTimeout(resolve, 10));
    }
    const status = await browser.host.status();
    throw new Error(`SDK spectrum collection did not publish ${targetKey}: ${JSON.stringify({
      elapsedMs: performance.now() - startedAt,
      contextState: browser.context.state,
      contextTime: (browser.context as AudioContext).currentTime,
      nativeStatus: status,
      lastStatus: lastNotification?.status,
      lastMetadata: lastNotification?.metadata,
    }, (_, value) => typeof value === "bigint" ? value.toString() : value)}`);
  };
  try {
    const subscription = (await browser.subscribeSpectrum({
      ...firstQuery,
      smoothingMs: 0,
      cadenceMs: 1,
    })).handle;
    const context = browser.context as AudioContext;
    // Feed the fixed short signal before playback; this gate measures selection continuity,
    // not per-message producer throughput on the browser's main thread.
    await submitBlocks(0, totalBlocks, true);
    const resumeButton = document.createElement("button");
    resumeButton.id = "spectrum-collection-resume";
    resumeButton.textContent = "Start spectrum qualification audio";
    document.body.append(resumeButton);
    let resumeTimer: ReturnType<typeof setTimeout> | undefined;
    let resumeGesture = false;
    try {
      await new Promise<void>((resolve, reject) => {
        resumeTimer = setTimeout(() => reject(new Error(
          `spectrum collection audio resume timed out (gesture=${resumeGesture}, state=${context.state})`,
        )), readinessTimeoutMs);
        resumeButton.addEventListener("click", () => {
          resumeGesture = true;
          void context.resume().then(resolve, reject);
        }, { once: true });
      });
    } finally {
      if (resumeTimer !== undefined) clearTimeout(resumeTimer);
      resumeButton.remove();
    }
    const first = await waitForResult(subscription, entryA);
    const firstClock = context.currentTime;
    const firstLeft = first.leftDb;
    if (firstLeft === undefined) throw new Error("spectrum collection omitted the first A channel");
    const firstPeak = spectrumPeak(firstLeft);

    const toB = await subscription.update({ ...secondQuery, smoothingMs: 0, cadenceMs: 1 });
    const bCleared = subscription.readLatest() === undefined;
    const second = await waitForResult(subscription, entryB);
    const secondClock = context.currentTime;
    const secondLeft = second.leftDb;
    if (secondLeft === undefined) throw new Error("spectrum collection omitted the selected B channel");
    const secondPeak = spectrumPeak(secondLeft);

    const toA = await subscription.update({ ...firstQuery, smoothingMs: 0, cadenceMs: 1 });
    const aCleared = subscription.readLatest() === undefined;
    const result = await waitForResult(subscription, entryA);
    const finalClock = context.currentTime;
    const left = result.leftDb;
    if (left === undefined) throw new Error("spectrum collection omitted the final A channel");
    const peak = spectrumPeak(left);
    const owned = result.frequenciesHz !== result.leftDb
      && result.frequenciesHz !== result.rightDb
      && result.leftDb !== result.rightDb;
    const resultTarget = spectrumTargetKey(result.target);
    await subscription.close();
    const audioContinued = context.state === "running" && secondClock >= firstClock && finalClock >= secondClock;
    return {
      selections: [firstQuery, secondQuery, firstQuery].map((entry) => spectrumTargetKey(entry.target)),
      masks: [firstQuery.channels, secondQuery.channels],
      toBTarget: spectrumTargetKey(toB.configuration.target),
      toBChannels: toB.configuration.channels,
      toBOk: true,
      toAOks: true,
      bCleared,
      aCleared,
      firstTarget: spectrumTargetKey(first.target),
      firstChannels: first.channels,
      firstCapturedSample: first.capturedSample.toString(),
      firstEndSample: first.endSample.toString(),
      secondTarget: spectrumTargetKey(second.target),
      secondChannels: second.channels,
      secondCapturedSample: second.capturedSample.toString(),
      secondEndSample: second.endSample.toString(),
      resultTarget,
      resultChannels: result.channels,
      capturedSample: result.capturedSample.toString(),
      endSample: result.endSample.toString(),
      peakBin: peak.index,
      peakHz: result.frequenciesHz[peak.index],
      firstPeakDb: firstPeak.value,
      secondPeakDb: secondPeak.value,
      finalPeakDb: peak.value,
      firstPeakHz: first.frequenciesHz[firstPeak.index],
      secondPeakHz: second.frequenciesHz[secondPeak.index],
      distinctSelectedSignal: Math.abs(firstPeak.value - secondPeak.value) > 3,
      finite: Array.from(result.frequenciesHz).every(Number.isFinite)
        && Array.from(result.leftDb ?? []).every(Number.isFinite)
        && Array.from(result.rightDb ?? []).every(Number.isFinite),
      owned,
      audioContinued,
      switchedWithoutRestart: true,
    };
  } finally {
    await browser.close();
  }
}

async function createTrackResponseSubscriptionBrowser(stats: { queries: number; initializations: number }) {
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createEngine({
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES, console: {
      commandQueueRecords: 64, observationTaps: 4,
    } },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    createResponseWorker: countingTrackResponseWorker(stats),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  return browser;
}

async function runTrackResponseSubscriptionQualification(reference: TrackResponseResult): Promise<Record<string, unknown>> {
  const stats = { queries: 0, initializations: 0 };
  const browser = await createTrackResponseSubscriptionBrowser(stats);
  const document = new Uint8Array(await (await fetch("/qualification/observation-session.json")).arrayBuffer());
  const asset = await MisoEngineAsset.load(await (await fetch(
    "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
  )).arrayBuffer());
  const headless = await WasmBoundary.boot(asset, document, {
    sourceRingFrames: OBSERVATION_FRAMES,
    console: { commandQueueRecords: 64, observationTaps: 4 },
  });
  const headlessConsole = new EngineConsole(headless.sessionMap(), (edits) =>
    headless.submitCommands(encodeLaneEdits(edits), edits.length));
  const request = {
    trackId: "track",
    grid: { kind: "logarithmic" as const, points: 5, minimumHz: 20, maximumHz: 20_000 },
    channels: "both" as const,
    responseLimits: { maximumResultBytes: 1 << 20, requestDeadlineMs: 5_000 },
  };
  let callbackCount = 0;
  let callbackOwner = "";
  let callbackEpoch = "";
  let callbackJob = "";
  let callbackRevision = "";
  let callbackResult: TrackResponseResult | undefined;
  let resolveCallback: (() => void) | undefined;
  const callbackDelivered = new Promise<void>((resolve) => { resolveCallback = resolve; });
  try {
    const subscription = await browser.subscribeTrackResponse({
      ...request,
      cadenceMs: 10,
      onUpdate: (notification) => {
        callbackCount += 1;
        callbackOwner = notification.owner.toString();
        callbackEpoch = notification.epoch.toString();
        callbackJob = notification.job.toString();
        callbackRevision = notification.revision.toString();
        callbackResult = notification.handle.readLatest();
        resolveCallback?.();
      },
    });
    const initial = subscription.readLatest();
    if (initial === undefined) throw new Error("track response subscription returned no initial result");
    const initialQueries = stats.queries;
    const console = await browser.console();
    const builtinPair = { hpfHz: 80, lpfHz: 12_000 } as const;
    const edits = (owner: EngineConsole) => {
      const eq = owner.edit.track("track").effect("simd1", 0, "miso.parametric-eq");
      return [eq.parameter("band-1-gain", -3), eq.parameter("hpf-frequency", 400),
        eq.parameter("hpf-q", 0.8), eq.parameter("hpf-enabled", true),
        eq.parameter("lpf-frequency", 6_000), eq.parameter("lpf-q", 0.9),
        eq.parameter("lpf-enabled", true), owner.edit.track("track").inputFilters(builtinPair, { channel: "left" })];
    };
    const command = await console.submit(...edits(console));
    const headlessCommand = await headlessConsole.submit(...edits(headlessConsole));
    if (!headlessCommand.ok || command.appliedAtSample !== headlessCommand.appliedAtSample) {
      throw new Error("browser/headless live EQ and builtin-filter acknowledgements disagree");
    }
    if (!command.ok || command.admitted !== 8 || headlessCommand.admitted !== 8) {
      throw new Error("queued response subscription EQ and builtin-filter edits were refused or split");
    }
    const beforeRender = subscription.readLatest();
    const unchangedBeforeRender = beforeRender !== undefined
      && responseValuesEqual(initial, beforeRender)
      && stats.queries === initialQueries;
    const unchangedNotificationCount = callbackCount;

    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const headlessSource = headless.submitSource({ sourceId: "console-source", generation: 1n,
        startFrame: BigInt(block * 128), planes,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1 });
      if (!headlessSource.ok) throw new Error("headless live EQ source refused");
      const acknowledgement = await browser.host.submitSource({
        sourceId: "console-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK response subscription source submission refused");
    }
    const rendered = await browser.context.startRendering();
    let postRampMaximumDifference = 0;
    let postRampComparedFrames = 0;
    const settledAt = Number(command.appliedAtSample) + 64;
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const expected = headless.render(128);
      for (let frame = 0; frame < 128; frame += 1) {
        const absolute = block * 128 + frame;
        if (absolute < settledAt) continue;
        postRampComparedFrames += 1;
        for (const [channel, plane] of [expected.left, expected.right].entries()) {
          const difference = Math.abs(rendered.getChannelData(channel)[absolute]! - plane[frame]!);
          if (!Number.isFinite(difference)) throw new Error("live EQ produced nonfinite PCM");
          postRampMaximumDifference = Math.max(postRampMaximumDifference, difference);
        }
      }
    }
    const timerDelivered = await Promise.race([
      callbackDelivered.then(() => true),
      new Promise<boolean>((resolve) => globalThis.setTimeout(() => resolve(false), 500)),
    ]);
    let pumped = false;
    if (!timerDelivered) pumped = (await subscription.pump())?.available === true;
    const afterRender = subscription.readLatest();
    if (afterRender === undefined) throw new Error("track response subscription lost its result after render");
    const changedPublication = !responseValuesEqual(initial, afterRender)
      && subscription.revision > 1n
      && stats.queries >= initialQueries + 1;
    const callbackMatches = callbackResult !== undefined && responseValuesEqual(callbackResult, afterRender);
    const queriesAfterChange = stats.queries;
    await subscription.pump();
    await new Promise<void>((resolve) => globalThis.setTimeout(resolve, 30));
    const unchangedPollSuppressed = stats.queries === queriesAfterChange;

    const shared = await browser.subscribeTrackResponse({ ...request, cadenceMs: 25 });
    const sharedJob = shared.job === subscription.job
      && shared.owner === subscription.owner
      && responseValuesEqual(shared.readLatest()!, afterRender);
    const sharedView = shared.readLatest();
    const originalSharedLeft = sharedView?.leftDb?.[0];
    if (sharedView?.leftDb !== undefined && originalSharedLeft !== undefined) {
      sharedView.leftDb[0] = originalSharedLeft + 100;
    }
    const arraysIsolated = originalSharedLeft !== undefined
      && subscription.readLatest()?.leftDb?.[0] === originalSharedLeft
      && shared.readLatest()?.leftDb?.[0] === originalSharedLeft;
    await subscription.close();
    const firstCloseKeepsJob = responseValuesEqual(shared.readLatest()!, afterRender);

    const oldJob = shared.job;
    const oldResult = shared.readLatest();
    let invalidUpdateRefused = false;
    try {
      await shared.update({
        ...request,
        cadenceMs: 10,
        grid: { ...request.grid, points: 1 },
      });
    } catch {
      invalidUpdateRefused = true;
    }
    const invalidUpdatePreserved = shared.job === oldJob
      && oldResult !== undefined && responseValuesEqual(shared.readLatest()!, oldResult);
    const updatedReceipt = await shared.update({
      ...request,
      cadenceMs: 10,
      grid: { ...request.grid, points: 7 },
    });
    const updated = shared.readLatest();
    const gridUpdated = updated !== undefined
      && updatedReceipt.job === shared.job
      && shared.configuration.grid.points === 7
      && updated.frequenciesHz.length === 7
      && stats.queries === queriesAfterChange + 1;
    const leftOnly = await browser.subscribeTrackResponse({
      ...request,
      cadenceMs: 10,
      grid: { ...request.grid, points: 7 },
      channels: "left" as const,
    });
    const leftOnlyResult = leftOnly.readLatest();
    const independentChannelJob = leftOnly.job !== shared.job
      && leftOnlyResult?.leftDb !== undefined && leftOnlyResult.rightDb === undefined;
    await shared.close();
    await leftOnly.close();
    const queriesAtLastClose = stats.queries;
    await new Promise<void>((resolve) => globalThis.setTimeout(resolve, 40));
    const lastCloseStoppedCapture = stats.queries === queriesAtLastClose;
    let staleReadRefused = false;
    try {
      shared.readLatest();
    } catch {
      staleReadRefused = true;
    }
    return {
      liveEq: {
        appliedAtSample: command.appliedAtSample.toString(),
        capturedSample: afterRender.capturedSample.toString(),
        postRampComparedFrames,
        postRampMaximumDifference,
      },
      liveInputFilters: {
        appliedAtSample: command.appliedAtSample.toString(),
        hpfHz: builtinPair.hpfHz,
        lpfHz: builtinPair.lpfHz,
        channel: "left",
        mixedBatchAdmitted: command.admitted === 8 && headlessCommand.admitted === 8,
        browserHeadlessAckParity: command.appliedAtSample === headlessCommand.appliedAtSample,
      },
      initial: serializeTrackResponse(initial),
      afterRender: serializeTrackResponse(afterRender),
      updated: updated === undefined ? null : serializeTrackResponse(updated),
      leftOnly: leftOnlyResult === undefined ? null : serializeTrackResponse(leftOnlyResult),
      workerInitializations: stats.initializations,
      workerQueries: stats.queries,
      unchangedBeforeRender,
      changedPublication,
      callbackCount,
      callbackOwner,
      callbackEpoch,
      callbackJob,
      callbackRevision,
      callbackMatches,
      automaticDelivery: timerDelivered,
      pumpDelivered: pumped,
      unchangedPollSuppressed,
      unchangedNotificationCount,
      sharedJob,
      arraysIsolated,
      firstCloseKeepsJob,
      invalidUpdateRefused,
      invalidUpdatePreserved,
      gridUpdated,
      independentChannelJob,
      lastCloseStoppedCapture,
      staleReadRefused,
      parityWithOneShot: responseValuesEqual(initial, reference),
      owner: subscription.owner.toString(),
      epoch: subscription.epoch.toString(),
      job: subscription.job.toString(),
      revision: subscription.revision.toString(),
      capturedSample: initial.capturedSample.toString(),
      snapshotToken: initial.snapshotToken.toString(),
      bounds: subscription.bounds,
    };
  } finally {
    headless.dispose();
    await browser.close();
  }
}

async function createResidentObservationBrowser(): Promise<Awaited<ReturnType<typeof createEngine>>> {
  const document = observationDocumentWithGate(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createEngine({
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES, console: {
      commandQueueRecords: 64, observationTaps: 4,
    } },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  return browser;
}

async function runResidentObservationQualification(): Promise<Record<string, unknown>> {
  const browser = await createResidentObservationBrowser();
  const selections = [observationSelection("comp"), observationSelection("gate", "left")];
  const callbackRows: ReturnType<typeof serializeObservationRows>[] = [];
  let callbackCount = 0;
  let callbackAvailable = false;
  let callbackOwner = "";
  let callbackEpoch = "";
  let resolveCallback: (() => void) | undefined;
  const callbackDelivered = new Promise<void>((resolve) => { resolveCallback = resolve; });
  try {
    const map = await browser.observationMap();
    const subscription = await browser.subscribeObservations({
      selections,
      windowBlocks: 2,
      cadenceMs: 10,
      onUpdate: (notification) => {
        callbackCount += 1;
        callbackAvailable ||= notification.available;
        callbackOwner = notification.owner.toString();
        callbackEpoch = notification.epoch.toString();
        callbackRows.push(serializeObservationRows(notification.handle.readLatest()));
        resolveCallback?.();
      },
    });
    const pending = subscription.readLatest();
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const acknowledgement = await browser.host.submitSource({
        sourceId: "console-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK resident observation source submission refused");
    }
    await browser.context.startRendering();
    const timerDelivered = await Promise.race([
      callbackDelivered.then(() => true),
      new Promise<boolean>((resolve) => globalThis.setTimeout(() => resolve(false), 500)),
    ]);
    let pumped = false;
    if (!timerDelivered) pumped = (await subscription.pump())?.available === true;
    const ready = subscription.readLatest();
    const repeated = subscription.readLatest();
    const firstSnapshot = ready.map(observationRowKey);

    const shared = await browser.subscribeObservations({
      selections,
      windowBlocks: 2,
      cadenceMs: 10,
    });
    const sharedBinding = shared.owner === subscription.owner
      && shared.epoch === subscription.epoch
      && observationRowsEqual(shared.readLatest(), ready);
    await subscription.close();
    const firstCloseKeepsLive = observationRowsEqual(shared.readLatest(), ready);

    let invalidUpdateRefused = false;
    try {
      await shared.update({
        selections: [observationSelection("missing")],
        windowBlocks: 2,
        cadenceMs: 10,
      });
    } catch {
      invalidUpdateRefused = true;
    }
    const invalidUpdatePreserved = observationRowsEqual(shared.readLatest(), ready);
    const updatedReceipt = await shared.update({
      selections: [observationSelection("comp", "right")],
      windowBlocks: 2,
      cadenceMs: 10,
    });
    const updated = shared.readLatest();
    const firstSnapshotRetained = JSON.stringify(firstSnapshot) === JSON.stringify(ready.map(observationRowKey));
    const updatedRightOnly = updated.length === 1
      && updated[0]?.channels === "right"
      && updated[0]?.left === undefined
      && updated[0]?.right !== undefined
      && Number.isFinite(updated[0].right);
    await shared.close();
    let staleReadRefused = false;
    try {
      shared.readLatest();
    } catch {
      staleReadRefused = true;
    }
    return {
      mapBindings: map.bindings.map((binding) => binding.effectSlotId),
      selectionEffectSlots: selections.map((selection) => selection.effectSlotId),
      pendingStatuses: pending.map((row) => row.status),
      readyStatuses: ready.map((row) => row.status),
      readyChannels: ready.map((row) => row.channels),
      readyRows: serializeObservationRows(ready),
      readyValuesFinite: ready.length === 2 && ready.every((row) =>
        (row.left === undefined || Number.isFinite(row.left))
        && (row.right === undefined || Number.isFinite(row.right))),
      windows: ready.map((row) => row.window === undefined ? null : {
        firstSample: row.window.firstSample.toString(),
        endSample: row.window.endSample.toString(),
        sequence: row.window.sequence.toString(),
        blocks: row.window.blocks,
      }),
      ownedReadStable: observationRowsEqual(ready, repeated),
      callbackCount,
      callbackAvailable,
      callbackOwner,
      callbackEpoch,
      callbackRows,
      automaticDelivery: timerDelivered,
      pumpDelivered: pumped,
      subscriptionId: subscription.id.toString(),
      owner: subscription.owner.toString(),
      epoch: subscription.epoch.toString(),
      appliedAtSample: subscription.appliedAtSample.toString(),
      bounds: subscription.bounds,
      sharedBinding,
      firstCloseKeepsLive,
      invalidUpdateRefused,
      invalidUpdatePreserved,
      updatedSelections: updatedReceipt.configuration.selections.map((selection) => ({
        effectSlotId: selection.effectSlotId,
        channels: selection.channels,
      })),
      updatedRightOnly,
      firstSnapshotRetained,
      staleReadRefused,
    };
  } finally {
    await browser.close();
  }
}

async function runSdkObservationQualification(): Promise<Record<string, unknown>> {
  const resident = await runResidentObservationQualification();
  const document = new TextEncoder().encode(
    await (await fetch("/qualification/observation-session.json")).text(),
  );
  const browser = await createEngine({
    document,
    policy: { sourceRingFrames: OBSERVATION_FRAMES, console: {
      commandQueueRecords: 64, observationTaps: 4,
    } },
    scratchBoot: async () => ({
      sampleRateHz: 48_000,
      quantumFrames: 128,
      sourceRingFrames: OBSERVATION_FRAMES,
      backend: "simd128" as const,
      sources: [{ id: "console-source", channels: 2, frames: BigInt(OBSERVATION_FRAMES) }],
      tracks: ["track"],
    }),
    createContext: () => {
      const context = new OfflineAudioContext(2, OBSERVATION_FRAMES, 48_000);
      // OfflineAudioContext has no close() lifecycle method; BrowserEngine's injected-context
      // seam still requires one so the host can use the same cleanup path as a live context.
      Object.defineProperty(context, "close", { value: async () => {} });
      return context;
    },
    createHost: (request) => createDefaultHost({
      ...request,
      hostModuleUrl: "/artifacts/miso-engine-v1-audio-worklet-host.js",
    }),
    simd128ModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.simd128.wasm",
    workletModuleUrl: "/artifacts/miso-engine-v1-audio-worklet.js",
    responseWorkerModuleUrl: "/sdk/response-worker.js",
  });
  browser.host.node.connect(browser.context.destination);
  try {
    const selections = [observationSelection("comp")];
    const map = await browser.observationMap();
    const pendingBeforeArm = await browser.readObservations(selections);
    const subscriptions = await browser.host.observe({ subscriptions: [
      { trackIndex: 0, rack: 1, effectIndex: 0, tapId: 1, windowBlocks: 2, armed: true },
    ] });
    const pendingAfterArm = await browser.readObservations(selections);
    for (let block = 0; block < OBSERVATION_FRAMES / 128; block += 1) {
      const planes = observationPlanes(block);
      const acknowledgement = await browser.host.submitSource({
        sourceId: "console-source",
        generation: 1n,
        startFrame: BigInt(block * 128),
        sampleRateHz: 48_000,
        planes,
        frames: 128,
        endOfRegion: block === OBSERVATION_FRAMES / 128 - 1,
      });
      if (acknowledgement.result !== 0) throw new Error("SDK observation source submission refused");
    }
    await browser.context.startRendering();
    const ready = await browser.readObservations(selections);
    const projected = await browser.readObservations([observationSelection("comp", "left")]);
    const repeated = await browser.readObservations(selections);
    const liveRequest = {
      trackId: "track",
      grid: { kind: "logarithmic" as const, points: 5, minimumHz: 20, maximumHz: 20_000 },
      channels: "both" as const,
      responseLimits: { maximumResultBytes: 1 << 20, requestDeadlineMs: 5_000 },
    };
    const livePromise = browser.queryTrackResponse(liveRequest);
    let livePendingRefused = false;
    try {
      await browser.queryTrackResponse(liveRequest);
    } catch {
      livePendingRefused = true;
    }
    const live = await livePromise;
    const liveAgain = await browser.queryTrackResponse(liveRequest);
    const liveLeft = live.leftDb === undefined ? [] : Array.from(live.leftDb);
    const liveRight = live.rightDb === undefined ? [] : Array.from(live.rightDb);
    const liveAgainLeft = liveAgain.leftDb === undefined ? [] : Array.from(liveAgain.leftDb);
    const liveAgainRight = liveAgain.rightDb === undefined ? [] : Array.from(liveAgain.rightDb);
    const liveFinite = live.frequenciesHz.length === liveRequest.grid.points
      && liveLeft.length === liveRequest.grid.points
      && liveRight.length === liveRequest.grid.points
      && Array.from(live.frequenciesHz).every(Number.isFinite)
      && liveLeft.every(Number.isFinite)
      && liveRight.every(Number.isFinite);
    const liveOwnedAfterSecondQuery = liveFinite
      && liveAgain.frequenciesHz.length === live.frequenciesHz.length
      && Array.from(live.frequenciesHz).every((value, index) => value === liveAgain.frequenciesHz[index])
      && liveLeft.every((value, index) => value === liveAgainLeft[index])
      && liveRight.every((value, index) => value === liveAgainRight[index]);
    const liveMembers = live.members.map((member) => ({
      nativeId: member.nativeId,
      stableId: member.stableId,
      rack: member.rack,
      slot: member.slot,
      kind: member.kind,
      available: member.available,
      bypassed: member.bypassed,
      enabledLeft: member.enabledLeft.length,
      enabledRight: member.enabledRight.length,
    }));
    const managedResponse = await runTrackResponseSubscriptionQualification(live);
    await browser.close();
    let liveClosedRefused = false;
    try {
      await browser.queryTrackResponse(liveRequest);
    } catch {
      liveClosedRefused = true;
    }
    const first = ready[0];
    const projectedRow = projected[0];
    return {
      mapBindings: map.bindings.map((binding) => binding.effectSlotId),
      subscribeResult: subscriptions.result,
      pendingBeforeArm: pendingBeforeArm.map((row) => row.status),
      pendingAfterArm: pendingAfterArm.map((row) => row.status),
      readyStatuses: ready.map((row) => row.status),
      readyChannels: ready.map((row) => row.channels),
      readyValuesFinite: ready.every((row) =>
        (row.left === undefined || Number.isFinite(row.left))
        && (row.right === undefined || Number.isFinite(row.right))),
      windows: ready.map((row) => row.window === undefined ? null : {
        firstSample: row.window.firstSample.toString(),
        endSample: row.window.endSample.toString(),
        sequence: row.window.sequence.toString(),
        blocks: row.window.blocks,
      }),
      ownedAfterSecondQuery: ready.length === repeated.length && ready.every((row, index) => {
        const other = repeated[index];
        return row.left === other?.left && row.right === other?.right
          && row.window?.firstSample === other?.window?.firstSample
          && row.window?.endSample === other?.window?.endSample
          && row.window?.sequence === other?.window?.sequence
          && row.window?.blocks === other?.window?.blocks;
      }),
      projectedChannels: projected.map((row) => row.channels),
      distinctChannelProjection: first?.left !== undefined && first?.right !== undefined
        && projectedRow?.left !== undefined && projectedRow?.right === undefined,
      liveResponse: {
        trackId: live.trackId,
        mode: live.mode,
        meaning: live.meaning,
        sampleRateHz: live.sampleRateHz,
        points: live.frequenciesHz.length,
        firstFrequency: live.frequenciesHz[0],
        lastFrequency: live.frequenciesHz[live.frequenciesHz.length - 1],
        left: liveLeft,
        right: liveRight,
        finite: liveFinite,
        members: liveMembers,
        excludedMemberCount: live.excludedMemberCount,
        capturedSample: live.capturedSample.toString(),
        snapshotToken: live.snapshotToken.toString(),
        resultBytes: live.resultBytes.toString(),
        ownedAfterSecondQuery: liveOwnedAfterSecondQuery,
        pendingRefused: livePendingRefused,
        closedRefused: liveClosedRefused,
      },
      resident,
      managedResponse,
    };
  } finally {
    await browser.close();
  }
}

export async function runSdkResponseQualification(): Promise<Record<string, unknown>> {
  const bytes = new Uint8Array(await (await fetch("/artifacts/miso-engine-v1-audio-worklet.simd128.wasm")).arrayBuffer());
  const asset = await MisoEngineAsset.load(bytes);
  const preview = await createResponsePreview({
    asset,
    responseWorkerModuleUrl: "/sdk/response-worker.js",
    responseLimits: { requestDeadlineMs: 5_000 },
  });
  try {
    const eq = await preview.query({
      configurationId: EQ_CONFIGURATION_ID,
      sampleRateHz: 48_000,
      quantumFrames: 128,
      configuration: effect("miso.parametric-eq", {
        "band-1-enabled": true,
        "band-1-kind": "bell",
        "band-1-frequency": 1_000,
        "band-1-gain": 3,
        "band-1-q": 1,
      }),
      grid: { kind: "logarithmic", points: 32, minimumHz: 20, maximumHz: 20_000 },
      channels: "both",
      fields: "totalAndSections",
    });
    const eqLeft = Array.from(eq.totalLeftDb ?? []);
    const filters = await preview.query({
      configurationId: EQ_CONFIGURATION_ID + 1n,
      sampleRateHz: 48_000,
      quantumFrames: 128,
      configuration: { kind: "inputFilters", left: { hpfHz: 80 }, right: { lpfHz: 12_000 } },
      grid: { kind: "linear", points: 16, minimumHz: 0, maximumHz: 24_000 },
      channels: "both",
      fields: "totalAndSections",
    });
    return {
      capabilities: preview.capabilities.map((row) => ({ owner: row.owner, target: row.target })),
      eq: {
        configurationId: eq.configurationId.toString(),
        mode: eq.mode,
        points: eq.frequenciesHz.length,
        sections: eq.sections.length,
        left: eqLeft,
        right: Array.from(eq.totalRightDb ?? []),
        frequencies: Array.from(eq.frequenciesHz),
        enabledLeft: Array.from(eq.enabledLeft),
        enabledRight: Array.from(eq.enabledRight),
      },
      filters: {
        configurationId: filters.configurationId.toString(),
        points: filters.frequenciesHz.length,
        sections: filters.sections.length,
        firstFrequency: filters.frequenciesHz[0],
        lastFrequency: filters.frequenciesHz[filters.frequenciesHz.length - 1],
        left: Array.from(filters.totalLeftDb ?? []),
        right: Array.from(filters.totalRightDb ?? []),
      },
      ownedAfterSecondQuery: eqLeft.every((value, index) => value === eq.totalLeftDb?.[index]),
      observations: await runSdkObservationQualification(),
      spectrum: await runSdkSpectrumQualification(),
    };
  } finally {
    await preview.close();
  }
}
