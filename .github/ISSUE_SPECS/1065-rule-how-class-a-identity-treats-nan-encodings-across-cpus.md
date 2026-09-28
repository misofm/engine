# Rule how class-A identity treats NaN encodings across CPUs

Found by the AArch64 CI leg (#1017). Owner ruling needed before implementation.

## Problem

When an operation produces a NaN (for example `inf - inf`), x86 gives `0xFFC00000` and AArch64
gives `0x7FC00000`, and the two CPUs choose differently when both inputs of an operation are NaN.
The WebAssembly spec leaves the sign and payload of such NaNs unspecified, so the shipped browser
module inherits the host CPU's rule: V8 on an x86 laptop and V8 on an Apple-silicon Mac or an
Android phone can emit different NaN bits for the same session.

Under emulation on #1017's AArch64 leg, six tests differ only in NaN bits, in debug and release:
compressor `kernel::settled_body_tests::scenario_{981,983,985,995,1006}` and the EQ bank's
`admitted_blocks_render_the_base_bits_without_selects`. With every NaN folded to one value, all
six compressor scenarios and all three EQ legs give identical digests on both CPUs. #1017 carries
them as named expected failures under this issue.

Since #1049, two remain: compressor `kernel::settled_body_tests::scenario_1006_ramping_prefix_is_pinned`
and the EQ bank's `admitted_blocks_render_the_base_bits_without_selects`. #1049 deleted the
compressor's `scenario_{981,983,985,995}` pins (and `scenario_982`) as dominated, with their
expected-failure rows.

This is not LANE-3 (#1019). LANE-3 is LLVM folding `max`/`min` into `fmaxnm`/`fminnm`, which a
code shape can prevent. NaN generation and propagation is the CPU's arithmetic rule, and no code
shape changes it short of canonicalizing NaNs.

## Owner decision

- **A. Class-A identity treats all NaNs as one value (recommended).** Class-A comparisons, digests
  and differentials fold every NaN to one canonical word before hashing. The claim becomes: same
  bits on every target, except that a NaN may carry a different sign or payload. This matches what
  the wasm spec already guarantees, costs nothing at render, and keeps the existing NaN-safety
  rules: finite input must not produce NaN, and each effect's documented NaN behaviour still holds.
- **B. The engine canonicalizes NaNs.** Every kernel that can produce a NaN, or the output
  boundary, rewrites NaNs to one word. That gives exact bits everywhere, at a per-sample cost in
  hot kernels and a new rule that every future kernel must follow.

## Smallest closable slice (option A)

Fold NaNs in the class-A digest and differential helpers (one shared helper, test-side only);
document the rule in the effect contract and target matrix; turn #1017's remaining named expected
failures into ordinary passing tests on the AArch64 leg; keep one test that proves a NaN-producing
scenario still yields NaN (so folding cannot hide a NaN appearing where finite output is required).

## Objective gates

1. The two remaining tests pass on the AArch64 leg and on x86, with no expected-failure entries left
   for them.
2. A planted change that turns a finite output into NaN still fails the finite-output tests.
3. #1019's gate 1 refers to this rule for NaN payloads instead of requiring raw NaN-bit identity.
