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
  control: "threshold_db",
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

// The lane is tied to the row's `lanes`: a shared row (one value for its owner) is addressed
// `both` only, a per-lane row accepts `left`, `right` and `both`.
export const perLaneBoth: ControlAddress = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "builtins" },
  control: "fader_db",
  lane: "both",
};
export const sharedBoth: ControlAddress = {
  owner: { kind: "route", id: "drums-to-bus" },
  section: { kind: "own" },
  control: "mute",
  lane: "both",
};
// @ts-expect-error a shared row accepts only the lane `both`
export const sharedLeft: ControlAddress = {
  owner: { kind: "route", id: "drums-to-bus" },
  section: { kind: "own" },
  control: "mute",
  lane: "left",
};
// An insert and a console entry of an effect with a choice of link modes both have the link-mode
// row (#1369); an effect with one mode has none.
export const insertLinkMode: ControlAddress<"miso.compressor"> = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "inserts", effect: "comp" },
  control: "link_mode",
  lane: "both",
};
export const consoleBypass: ControlAddress<"miso.compressor"> = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "console", slot: "glue" },
  control: "bypass",
  lane: "both",
};
export const consoleLinkMode: ControlAddress<"miso.compressor"> = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "console", slot: "glue" },
  control: "link_mode",
  lane: "both",
};
export const consoleEqLinkMode: ControlAddress<"miso.parametric-eq"> = {
  owner: { kind: "strip", id: "drums" },
  section: { kind: "console", slot: "tone" },
  // @ts-expect-error the EQ has one link mode, so its console entry has no link-mode row
  control: "link_mode",
  lane: "both",
};
