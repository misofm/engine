/** Issue #323: pin the optional Effect entry's success, failure, and requirement types. */

import type { Effect, Scope } from "effect";

import type { BrowserEngine } from "../src/browser/engine.ts";
import type { CommandReport } from "../src/core/boundary.ts";
import { EngineConsole } from "../src/core/console.ts";
import type { OfflineEngine } from "../src/headless/engine.ts";
import {
  type EngineEffectError,
  scopedBrowserEngine,
  scopedOfflineEngine,
  submitConsole,
} from "../src/effect.ts";

declare const browserOptions: Parameters<typeof scopedBrowserEngine>[0];

const scopedOffline: Effect.Effect<OfflineEngine, EngineEffectError, Scope.Scope> =
  scopedOfflineEngine("session");
const scopedBrowser: Effect.Effect<BrowserEngine, EngineEffectError, Scope.Scope> =
  scopedBrowserEngine(browserOptions);

const console = new EngineConsole(
  { tracks: ["track"], sources: [], metersAttached: false },
  async () => ({
    ok: true,
    result: 0,
    code: "ok",
    reason: 0,
    reasonName: "none",
    rejectedIndex: 0,
    admitted: 1,
    appliedAtSample: 0n,
  }),
);
const submission: Effect.Effect<CommandReport, EngineEffectError> = submitConsole(
  console,
  console.edit.track("track").faderDb(-3),
);

void [scopedOffline, scopedBrowser, submission];
