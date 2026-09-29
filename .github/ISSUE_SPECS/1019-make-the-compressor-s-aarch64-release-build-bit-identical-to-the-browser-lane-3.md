# Make the compressor's AArch64 release build bit-identical to the browser (LANE-3)

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

## Problem

On AArch64 release builds, the compressor's `max`/`min` (the D8 rule) fold into `fmaxnm`/`fminnm`, which treat NaN and signed zero differently from the pinned oracle. So a fan's phone renders a compressor mix whose bits differ from the producer's browser mix, and gate G1 is red there (LANE-3, #366, closed as deferred). Registered in `docs/TARGET_MATRIX.md`; evidence in `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F4. Native AArch64 is now an official target (`docs/rulings/engine-footprint-2026-09-28.md`).

## Smallest closable slice

Express the compressor's min/max (and any other kernel the AArch64 leg shows with the same fold) so that LLVM cannot fold it into `fmaxnm`/`fminnm` on AArch64, while keeping the x86 and wasm instruction selection and performance. One code shape for all targets; no target-specific code. First measure which kernels are affected on the AArch64 CI leg (#1017), not only the compressor.

## Objective gates

1. On AArch64 hardware (the #1017 leg), the compressor differentials and every console digest equal the x86 and wasm digests, including signed-zero cases and NaN cases under class-A identity's NaN rule (#1065: every NaN compares as one value, so NaN payloads are not compared); gate G1 is green there.
2. A committed assembly check fails if `fmaxnm`/`fminnm` reappear in the affected render kernels on AArch64.
3. No regression on x86 (native console `compressor_only` isolate) or in the shipped browser artifact (V8 compressor isolates), measured under the timing lock; the V8 spill gate passes.
4. Remove LANE-3 from the deferred-defect register.
