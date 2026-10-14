/**
 * Issue #1532: `MisoEngineError.result` is an engine result code only when the engine returned
 * one, and the SDK's own timeout codes carry none. This file runs nothing; it is checked by
 * `scripts/check-sdk-types.sh` and fails COMPILATION when either shape stops being accepted.
 *
 * Red mutation: retype `MisoEngineError.result` back to `number` -> `engineSide.result` below no
 * longer matches, the SDK-side construction stops compiling, and both asserts fail.
 */

import { MisoEngineError } from "../src/core/errors.ts";
import type { MisoErrorCode, SdkTimeoutCode } from "../src/core/errors.ts";
import type { SdkTimeoutCode as HeadlessSdkTimeoutCode } from "../src/headless/index.ts";

type Exact<A, B> =
  (<T>() => T extends A ? 1 : 2) extends (<T>() => T extends B ? 1 : 2) ? true : false;
type Assert<T extends true> = T;

type TimeoutCodes = Assert<
  Exact<SdkTimeoutCode, "spectrumCloseTimeout" | "hostDisposeTimeout" | "contextCloseTimeout">
>;
type TimeoutCodesAreExported = Assert<Exact<HeadlessSdkTimeoutCode, SdkTimeoutCode>>;
type TimeoutCodesAreErrorCodes = Assert<Exact<SdkTimeoutCode extends MisoErrorCode ? true : false, true>>;

// Engine side: a result code rides along.
const engineSide = new MisoEngineError("refused", { phase: "boot", code: "invalidArgument", result: 1 });
// SDK side: no engine result exists, and none is claimed.
const sdkSide = new MisoEngineError("close: AudioContext close did not settle within 10000 ms", {
  phase: "lifecycle",
  code: "contextCloseTimeout",
});

type EngineResult = Assert<Exact<typeof engineSide.result, number | undefined>>;
type SdkResult = Assert<Exact<typeof sdkSide.result, number | undefined>>;

export type { TimeoutCodes, TimeoutCodesAreExported, TimeoutCodesAreErrorCodes, EngineResult, SdkResult };
