import { MisoEngineAsset } from "../../../sdk/src/core/asset.ts";
import { effect } from "../../../sdk/src/core/session.ts";
import { createResponsePreview } from "../../../sdk/src/browser/response.ts";

const EQ_CONFIGURATION_ID = 9_007_199_254_740_993n;

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
    };
  } finally {
    await preview.close();
  }
}
