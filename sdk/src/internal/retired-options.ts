import { MisoUsageError } from "../core/errors.ts";

/**
 * Issue #1590 D5: the retired observation-taps word is refused, never ignored, for an untyped
 * caller: every plan binds every declared observation tap, so a caller that still sizes the
 * capacity is told so. One check for every path that reads a caller's `liveControls`. This
 * module is deliberately absent from every package export map and public barrel.
 */
export function refuseRetiredObservationTaps(liveControls: unknown): void {
  if ((liveControls as { observationTaps?: unknown } | undefined)?.observationTaps !== undefined) {
    throw new MisoUsageError(
      "liveControls.observationTaps was retired (issue #1590): every plan binds every declared observation tap",
    );
  }
}
