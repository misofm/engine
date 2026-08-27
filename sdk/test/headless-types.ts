import { effect, session, type OfflineEngine, type SessionPlan } from "../src/headless/index.js";

const compressor = effect("miso.compressor", { threshold: -18 });
const plan = session({ id: "headless-types", sampleRateHz: 48_000 })
  .source("source", { channels: 2, frames: 128 })
  .track("lead", { source: "source", dynamic: [compressor] as const })
  .build();
type Shape = typeof plan extends SessionPlan<infer S> ? S : never;
declare const engine: OfflineEngine<Shape>;

engine.console.track("lead").fader(-6);
engine.console.track("lead").effect(0).set({ threshold: [-24, -12] });
engine.console.track("lead").effect(0).observe("Gain Reduction");
engine.console.sessionMap().then((map) => {
  const frames: bigint = map.sources[0].frames;
  const startFrame: bigint = map.sources[0].startFrame;
  const sourceId: string = map.sources[0].id;
  void [frames, startFrame, sourceId];
  // @ts-expect-error Source regions stay exact BigInts across the Wasm boundary.
  const narrowed: number = map.sources[0].frames;
  void narrowed;
});
// @ts-expect-error Headless console track IDs remain exact.
engine.console.track("missing");
// @ts-expect-error Headless effect positions remain exact.
engine.console.track("lead").effect(1);
// @ts-expect-error Effect parameter names remain exact.
engine.console.track("lead").effect(0).set({ missing: 1 });
// @ts-expect-error Effect observation names remain exact.
engine.console.track("lead").effect(0).observe("Output Peak");
