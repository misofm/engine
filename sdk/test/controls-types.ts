/** Decision 17 slice S2b: the generated control types accept the core's rows and refuse others. */

import type { ControlAddress, EffectControlName } from "../src/index.ts";

// A strip's fader, a send's gain and a compressor's threshold are rows of the core's tables.
export const fader: ControlAddress = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "builtins" },
  control: "fader_db",
  lane: "left",
};
export const sendGain: ControlAddress = {
  owner: { kind: "route", id: "drums-to-bus" },
  section: { kind: "own" },
  control: "gain_db",
  lane: "both",
};
export const threshold: ControlAddress = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "inserts", effect: "comp" },
  control: "threshold",
  lane: "right",
};
// The same name, narrowed to one effect.
export const compressorBypass: EffectControlName<"miso.compressor"> = "bypass";

export const unknownBuiltin: ControlAddress = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "builtins" },
  // @ts-expect-error no builtin row has this name
  control: "fader_decibels",
  lane: "left",
};
// @ts-expect-error a fader is a strip row, not a route row
export const wrongOwner: ControlAddress = {
  owner: { kind: "route", id: "drums-to-bus" },
  section: { kind: "own" },
  control: "fader_db",
  lane: "both",
};
// @ts-expect-error the compressor declares no such parameter
export const unknownEffectRow: EffectControlName<"miso.compressor"> = "ceiling";
