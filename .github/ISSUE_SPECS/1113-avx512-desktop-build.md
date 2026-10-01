# Ship an AVX-512 desktop build that the installer selects

**On hold (owner, 2026-10-01: "Let's hold off on AVX512 implementation").** It is also blocked on
hardware. Do not start until the owner lifts the hold and the blockers below are cleared. Until
then, keep CPU architecture and lane shape distinct: lane widths come from target features, never
from `target_arch`, so that this issue stays a clean addition.

## Owner decision (2026-10-01)

- AVX-512 is worth it: the desktop app may be the product that sells, and "AMD Zen users would
  expect an implementation that makes maximal use of their hardware".
- Distribution: "installer choosing the ideal one is the right move". Ship two native desktop
  builds, x86-64-v3 (AVX2 + FMA, 8 lanes) and x86-64-v4 (AVX-512, 16 lanes). The installer picks
  one from the CPU's features.
- There is no in-binary runtime dispatch. Master plan D4's compile-time pin stands: each build is
  pinned, and `lane::attest_host` refuses a CPU that lacks its features.

Why not runtime dispatch: every width's kernels would ship in one binary; the `#[target_feature]`
boundaries would act as inlining barriers; the test matrix grows; and it risks paths with and without FMA
that differ in bits.

## Scope when unblocked

1. **Lanes.** A 16-lane backend (`Simd16`, `BankWidth::Sixteen`), keyed on `target_feature =
   "avx512f"` through the same feature predicate #1112 introduces for eight lanes. One kernel shape;
   only the width differs.
2. **Banking.** Console slots pad to 16, as S2 (#1098) pads to 8. Each effect's padded bank factory
   accepts W = 16 (P2a-P2e's contract).
3. **Build.** A second pinned x86 profile (`+avx512f` and whatever the chosen v4 feature set needs,
   decided with measurements). The engine ships both `capi` libraries. The installer's CPU check is
   the desktop app's, outside this repo; the engine documents the exact feature set each library
   requires.
4. **Class A.** The v3 and v4 builds render bit-identical output for every session. Both have FMA,
   and banking never changes per-lane arithmetic, so a producer and a listener on different
   builds hear the same bits. A committed differential proves it.
5. **Measurement.** The console benchmark rows on Zen 4 (512-bit work split into two 256-bit
   halves) and Zen 5 (full 512-bit), against the v3 build on the same machine, with padding cost
   reported at small track counts.

## Blockers

- An AVX-512 CI runner for correctness (or Intel SDE emulation, if it is accepted for the class-A
  gates), and Zen 4/5 benchmark hardware. Dedicated hardware is planned once there is funding. The
  current host is Zen 3 and has no AVX-512.
- A native desktop app and its installer to make the selection.

## Dependencies

- #1112, the feature-keyed lane width predicate.
