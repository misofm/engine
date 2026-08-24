# SIMD wrappers around scalar inner loops are not a vectorization win

## Hypothesis

Wrapping a transient-shaper bank in a W8 API was expected to improve throughput over the scalar
product simply because eight tracks were presented together.

## Measurement

The pre-audit implementation measured **58.86 ns per lane-frame** for W8 versus **41.97 ns per
lane-frame** for W1, release profile, 48 kHz, 128-frame blocks, 20,000 blocks, one warm-up and two
measured rounds through the public factory. More than 95% of the bank's work remained in a scalar
per-track loop; the SIMD kernel was called per sample for only four operations. The “SIMD bank” was
therefore about 40% slower per lane-frame than scalar on that workload.

Source: [Issue #20 decision record](../../.github/ISSUE_SPECS/020-transient-shaper.md),
“Descriptive before/after (not a gate).” The audit index independently records the same failure
class for builtins, compressor, gate/expander, and multiband compressor in
[Issue #83](../../.github/ISSUE_SPECS/083-audit-numeric-and-kernel-contract.md).

## Ruling

Do not treat a bank-shaped API or a small per-sample vector call as evidence of vectorized
throughput. The rejected candidate is specifically a SIMD wrapper whose dominant work stays in a
scalar inner loop. A replacement must move the full block kernel into the lane-generic body and
must report per-lane-frame scalar and bank measurements.

The ruling does not reject SIMD banking. The same Issue #20 record later measured the reworked W8
block kernel at 4.96 ns per lane-frame versus 20.47 ns for W1. That later result supersedes the
implementation, not this warning about the rejected wrapper shape.

Reopen only for a materially different candidate with disassembly showing the dominant kernel
body uses the intended vector family and a frozen scalar-versus-bank measurement on the same
workload.
