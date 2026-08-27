import { effect, session, type SessionPlan, type TrackConsole } from "../src/index.js";

const effects = [effect("miso.compressor", { threshold: -18 })] as const;
declare const consoleForTrack: TrackConsole<typeof effects>;

consoleForTrack.effect(0);
// @ts-expect-error The exact tuple index type rejects the tuple length/off-by-one index.
consoleForTrack.effect(1);

effect("miso.delay", { "cross feedback": 0.25 });
// @ts-expect-error Shared effect parameters must not accept a per-lane object.
effect("miso.delay", { "cross feedback": { left: 0.25, right: 0.5 } });
effect("miso.compressor", { threshold: [-24, -12] });

const plan = session({ id: "typed", sampleRateHz: 48_000 })
  .source("source", { channels: 1, frames: 1 })
  .track("lead", { source: "source", dynamic: effects })
  .build();
type PlanShape = typeof plan extends SessionPlan<infer Shape> ? Shape : never;
const knownTrack: keyof PlanShape = "lead";
void knownTrack;
// @ts-expect-error Builder track IDs must remain exact through SessionPlan.
const unknownTrack: keyof PlanShape = "missing";
void unknownTrack;
declare const consoleFromPlan: TrackConsole<PlanShape["lead"]>;
consoleFromPlan.effect(0);
// @ts-expect-error Builder effect tuples must remain exact through SessionPlan.
consoleFromPlan.effect(1);
