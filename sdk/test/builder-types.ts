import { effect, type TrackConsole } from "../src/index.js";

const effects = [effect("miso.compressor", { threshold: -18 })] as const;
declare const consoleForTrack: TrackConsole<typeof effects>;

consoleForTrack.effect(0);
// @ts-expect-error The exact tuple index type rejects the tuple length/off-by-one index.
consoleForTrack.effect(1);
