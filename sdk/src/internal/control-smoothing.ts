import type { ModelRecord } from "../core/session.ts";
import { CODE, f32, fail, freeze, knownKeys } from "./validation.ts";

/**
 * The `controlSmoothing` bounds and `f32` check, used by `session()` today and reused by the
 * session transaction builder when #1383 and #1385 land (neither exists yet). This module is in no
 * export map or public barrel, like `ramp.ts`.
 */

/** The inclusive upper bound of each `control_smoothing` length, a copy of the engine's
 * `session::CONTROL_SMOOTHING_MS_MAXIMUM` (#1054 D1), held to it by the builder evals. */
const CONTROL_SMOOTHING_MS_MAXIMUM = 1000;
const CONTROL_SMOOTHING_KEYS: ReadonlyMap<string, string> = new Map([
  ["muteMs", "mute_ms"],
  ["faderMs", "fader_ms"],
  ["panMs", "pan_ms"],
]);

/**
 * The `control_smoothing` record for the options' `controlSmoothing`: `{ kind: "default" }` when
 * omitted, else `explicit` with each length a finite `f32` in `[0, 1000]` ms. A missing, unknown or
 * out-of-range key is refused at `path`, under the engine's own codes. `session()` passes the path
 * `session().controlSmoothing`; the transaction builder will pass its own method path.
 */
export function normalizeControlSmoothing(value: unknown, path: string): ModelRecord {
  if (value === undefined) return freeze({ kind: "default" });
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    fail(path, "expected an object with muteMs, faderMs and panMs", CODE.wrongType);
  }
  knownKeys(value, new Set(CONTROL_SMOOTHING_KEYS.keys()), path, "controlSmoothing");
  const record: Record<string, string | number> = { kind: "explicit" };
  for (const [name, key] of CONTROL_SMOOTHING_KEYS) {
    const supplied = (value as Record<string, unknown>)[name];
    if (supplied === undefined) fail(`${path}.${name}`, "a length in milliseconds is required");
    const length = f32(supplied, `${path}.${name}`);
    if (length < 0 || length > CONTROL_SMOOTHING_MS_MAXIMUM) {
      fail(
        `${path}.${name}`,
        `a control smoothing length must lie in [0, ${CONTROL_SMOOTHING_MS_MAXIMUM}] ms`,
        CODE.outOfRange,
      );
    }
    record[key] = length;
  }
  return freeze(record);
}
