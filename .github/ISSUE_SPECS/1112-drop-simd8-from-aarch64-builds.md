# Drop the eight-lane kernels from the AArch64 builds

## Problem

AArch64 always runs four lanes: `lane::Backend::current()` is `Simd4` on `aarch64` (iOS, Android,
and Apple-silicon desktop). #1110 compiled the eight-lane backend off wasm32 with
`#[cfg(not(target_arch = "wasm32"))]`, which leaves it on AArch64, where it can never run. Sol's
#1110 verdict (L4) measured the iOS `capi` library: 27 eight-lane functions, about 114,520 B of its
2,469,368 B `__text` (4.6 %).

Owner, 2026-10-01: "there's no point in having code that can't be run in the iPhone build."

## Smallest closable slice

1. The eight-lane items #1110 gated (`lane::Simd8`, `Backend::Simd8`, `BankWidth::Eight`, the
   `match_bank_width!` arm, and the few local cfgs in builtins, rack and the corpora) compile only
   on `target_arch = "x86_64"`, the one target that selects eight lanes. Narrow the predicate at
   each of #1110's sites, or route it through one shared predicate if that is simple. Do not add
   cfg sites.
2. Tests and gates on AArch64 compare Scalar with Simd4. x86-64 keeps Scalar, Simd4 and Simd8. The
   wasm build is unchanged by this issue.
3. No behaviour changes anywhere. No target changes the width it selects.

Authorized paths: #1110's cfg sites and the crates that hold them, the tests and gates that name
eight lanes on AArch64 (including the AArch64 CI legs' test lists, `scripts/run-aarch64-tests.sh`
and `scripts/lib/aarch64-known-defects.py` if a count moves), and this spec.

## Objective gates

1. The AArch64 iOS and Android `capi` builds contain no eight-lane function. A committed check
   fails if one returns, for example in `check-cross-targets.sh` or the AArch64 release leg. Record
   the iOS and Android `__text`/`.text` size before and after.
2. x86-64 is untouched: its `capi` `.text`/`.rodata` are identical to `089ef456`'s, and every x86
   Simd8 digest is unchanged. The wasm module is unchanged (`cd49dc1c…` shipped).
3. Class A: all console-workload digests and the wasm-gate digests equal `089ef456`'s.
4. `check-cross-targets.sh` passes, and so do the AArch64 legs' `--no-run`/`--list` resolution,
   `judge-skips` and the silent-skip scan. This host has no AArch64 runner, so CI's AArch64 debug
   and release legs are the NEON gate.
5. fmt, clippy `-D warnings` (x86 and both AArch64 targets), rustdoc, both debug splits, the release
   digest gates, `run-wasm-gates.sh` and every lint-job step pass.

## Non-goals

- No change to which width any target runs.
- No other size work for the mobile library.
