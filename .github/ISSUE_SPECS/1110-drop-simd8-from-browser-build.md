# Drop the eight-lane kernels from the browser build

## Problem

The browser always runs four lanes: `lane::Backend::current()` is `Simd4` on wasm `simd128`
(`crates/lane/src/backend.rs`; #183 measured eight lanes on wasm as a null, and #1038 removed the
cfg that selected them). Yet the shipped module still carries the eight-lane (`Simd8`, `f32x8`)
instantiations of the kernels and banks, because the effect crates' `match backend` arms
instantiate every width on every target. That was about 135 KB of the module when measured on
2026-09-29, and it is code the browser can never execute.

The wasm gate guest also exercises all three widths (Scalar, Simd4 and Simd8) on wasm. The owner
asked (2026-09-29) why eight is checked on a target that never runs it. On wasm it should check
Simd4 against Scalar. Native x86 keeps its 1/4/8 checks.

Owner, 2026-09-30: do this "if it doesn't actually help run any of the browser code".

## Smallest closable slice

1. The eight-lane backend and every kernel, bank or dispatch arm that instantiates it compile only
   where eight lanes run: `x86_64`, plus `test-support` builds on native hosts if tests need them.
   On wasm32, `Backend::Simd8` and its arms are absent at compile time, not merely unreachable.
   This is the one allowed difference between targets: the lane width. The implementation shape
   stays one generic kernel per effect (see the owner's "no target-specific code" rule). Record
   the mechanism, for example `#[cfg]` on the variant and a single cfg'd helper rather than a
   `cfg` at every match site.
2. The wasm gate guest (`scripts/run-wasm-gates.sh` and its guest crate) checks Scalar against
   Simd4 on wasm. Native gates keep Scalar, Simd4 and Simd8.
3. No behavioural change anywhere: the browser already selects Simd4, and native already selects
   Simd8 on x86-64 and Simd4 on AArch64.

Authorized paths: `crates/lane` and each crate whose backend match must change, the wasm gate
guest and its script, their tests, and this spec. Keep the edit mechanical; if it reaches more than
the lane crate plus one cfg'd dispatch point per crate, say why.

## Objective gates

1. The named browser module contains no eight-lane instantiation. The check greps its name section
   for the eight-lane symbols (`f32x8`, `Simd8`, or the kernel names the audit uses) and finds
   none. Record the module's before and after size.
2. Class A: every console-workload digest, every wasm-gate digest (Scalar against Simd4), the V8
   harness digests and the browser qualification render digests equal `7d030945`'s.
3. Native x86-64 is untouched: the workspace tests, both debug splits, the release digest gates
   and the audits pass, and the Simd8 digests are unchanged.
4. The AArch64 iOS and Android compile checks and `check-cross-targets.sh` pass. The AArch64 CI legs
   gate NEON.
5. `scripts/run-wasm-gates.sh` passes on the narrowed guest. The browser qualification passes in
   three browsers, and so do the SDK checks, fmt, clippy `-D warnings`, rustdoc and every lint-job
   step.

## Non-goals

- No change to which width any target runs.
- No change to native x86 or AArch64 code generation.
- Stripping the name section is its own issue.
