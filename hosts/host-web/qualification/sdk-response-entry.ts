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

async function submitSpectrumSource(browser) {
  for (let block = 0; block < SPECTRUM_FRAMES / 128; block += 1) {
    const acknowledgement = await browser.host.submitSource({
      sourceId: "console-source",
      generation: 1n,
      startFrame: BigInt(block * 128),
      sampleRateHz: 48_000,
      planes: spectrumPlanes(block),
      frames: 128,
      endOfRegion: block === SPECTRUM_FRAMES / 128 - 1,
    });
    if (acknowledgement.result !== 0) throw new Error("SDK spectrum source submission refused");
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
  };
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
    const command = await console.submit(
      console.edit.track("track").effect("simd1", 0, "miso.parametric-eq")
        .parameter("band-1-gain", -3),
    );
    if (!command.ok) throw new Error("queued response subscription control edit was refused");
    const beforeRender = subscription.readLatest();
    const unchangedBeforeRender = beforeRender !== undefined
      && responseValuesEqual(initial, beforeRender)
      && stats.queries === initialQueries;
    const unchangedNotificationCount = callbackCount;

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
      if (acknowledgement.result !== 0) throw new Error("SDK response subscription source submission refused");
    }
    await browser.context.startRendering();
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
