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
// @ts-expect-error Headless console track IDs remain exact.
engine.console.track("missing");
// @ts-expect-error Headless effect positions remain exact.
engine.console.track("lead").effect(1);
// @ts-expect-error Effect parameter names remain exact.
engine.console.track("lead").effect(0).set({ missing: 1 });
