import { MisoEngineAsset } from "../../../sdk/src/core/asset.ts";
import { effect } from "../../../sdk/src/core/session.ts";
import { createResponsePreview } from "../../../sdk/src/browser/response.ts";
import { createEngine } from "../../../sdk/src/browser/engine.ts";
import { createDefaultHost } from "../../../sdk/src/browser/default-host.ts";

const EQ_CONFIGURATION_ID = 9_007_199_254_740_993n;
const OBSERVATION_FRAMES = 2_048;

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

async function runSdkObservationQualification(): Promise<Record<string, unknown>> {
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
    };
  } finally {
    await preview.close();
  }
}
