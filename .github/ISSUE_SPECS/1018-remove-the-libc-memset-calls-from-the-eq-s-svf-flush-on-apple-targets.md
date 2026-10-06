# Remove the libc memset calls from the EQ's SVF flush on Apple targets

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

## Problem

On Darwin (`aarch64-apple-ios`, and by extension `aarch64-apple-darwin`), `svf_step`'s `L::splat(FLUSH_EPS)` in `crates/lane/src/kernels.rs` (around lines 280-281) compiles to `bl _memset_pattern16`: two libc calls per frame inside the kernel loop. The dead-code verification reproduced 151 `memset_pattern16` calls in `parametric-eq`'s iOS release assembly. Render must perform no libc or other calls of this kind (AGENTS.md, "Approved audio architecture"), so this breaches the realtime rules on every iPhone. It has no effect on `x86_64` or `wasm32`. Registered in `docs/TARGET_MATRIX.md` ("Deferred-defect register"); evidence in `docs/handoffs/dead-code-2026-09-28/VERIFY-DEAD-CODE.md`, finding F4. Native AArch64 is now an official target (`docs/rulings/engine-footprint-2026-09-28.md`).

## Smallest closable slice

Change the constant's construction (for example, hoist the splat out of the loop, or build the vector from a register constant) so that no Apple build of any EQ or builtin kernel calls `memset_pattern16` or any other libc routine in render. Class A: the rendered bits must not change on any target. One code shape for all targets; no target-specific code.

## Objective gates

1. The iOS release assembly of `parametric-eq`, `builtins` and every other crate that uses `svf_step` contains no `memset_pattern16` (or other libc) call inside render, checked by a committed script that fails if one appears; the script runs on the AArch64 CI leg (the #1017 issue).
2. Every console digest and the EQ differentials are unchanged on x86, wasm and AArch64.
3. The shipped AudioWorklet artifact's performance does not regress (V8 one-band and two-band isolates through the render export, under the timing lock); `scripts/run-wasm-gates.sh` (including the V8 spill gate) passes.
4. Remove the entry from the deferred-defect register.

## Scope found by #1017

#1017 scanned every product crate's `aarch64-apple-ios` release assembly (Rust 1.97.1; attempt 2,
after Sol's attempt 1 verdict). The problem is not the SVF flush alone. It is the shape "a stored
`f32x4` splat constant". LLVM lowers any such constant to `bl _memset_pattern16` on Apple
targets: `lane::FLUSH_EPS` (`0x1e3ce508`), and also `1.0`, `0.5`, `2.0`, `1e-8`,
`f32::MIN_POSITIVE` and others.

There are 3,494 calls in ten crates, almost all of them inside render:

| crate | calls | render functions |
|---|---|---|
| `multiband-compressor` | 1,132 | `PreparedMultibandCompressorBank::process_bank` |
| `compressor` | 970 | `kernel::process_block`, `ramping_main_scalar`, `process_block_mono`, `settled_sidechain` (`f32x4`) |
| `transient-shaper` | 534 | `Shaper::process_block` |
| `builtins` | 376 | `BuiltinInputBank::process`/`process_mono`, `InputStage::process` |
| `gate-expander` | 181 | `PreparedGate::process_bank` |
| `parametric-eq` | 151 | `PreparedParametricEq::process_bank`/`process_bank_mono`, `Channel::snap_ended` |
| `true-peak-limiter` | 104 | `LimiterCore::process_block`, `process_bank_inner` |
| `soft-clip` | 22 | `Channel::process`, `PreparedSoftClipBank::process_bank` |
| `graph` | 20 | `runtime::bank_meter_pass`, `runtime::bank_sample_peak` |
| `host-core` | 4 | `spectrum::SpectrumAnalyzer::analyze`/`analyze_continuous` (observation) |

The other fifteen product crates have none. `capi`, scanned as an rlib, has none.

**What this changes for this issue.** Gate 1's "every crate that uses `svf_step`" leaves out six
of the ten crates. The standing check is `ios-asm-memset-pattern16` in
`scripts/check-cross-targets.sh`, which runs in every PR's `cross-target` job. It keeps one row per
crate, with the count above as a ceiling, in `scripts/lib/aarch64-known-defects.py`.
- A crate that reaches zero fails until its row is deleted.
- A count that rises fails.
- A crate with calls and no row fails.

So the defect reads fixed only when every row is gone. **Root to rule:** either widen this issue
to the stored-splat shape in every kernel, or split the remaining crates into a successor issue.
The rows name #1018 until then.

## Root ruling (2026-10-06)

The decision-15 root coordinator ruled on the question above: **widen**. This issue covers the
stored-splat shape in every crate, not only the EQ's SVF flush, and no successor issue is split
off.

- *Let the builtins splat their chain constants without iOS memset calls* (#1451, Stream G of
  decision 15) executes the ruling. Its D1 finds the cause. If the cause is the splat lowering (for
  example `wide`'s `transmute([elem; N])` array repeat, reached through `Lane::splat` in
  `crates/lane/src/wide_impl.rs`), the fix has one shape, lives in the `lane` crate and applies to
  every crate.
- Every ceiling in `IOS_MEMSET_CEILINGS` (`scripts/lib/aarch64-known-defects.py`) may only go down,
  and #1451 re-measures each one.
- This issue closes when every ceiling reaches 0 and the gates above are met. If some do not, #1451
  records what remains and why, those rows stay this issue's, and this issue stays open.
- #1451 owns the cause, the fix and the builtins constants only. *Undo the iOS memset ratchet
  workarounds once splats are free* (#1452, Stream G, after #1451) undoes the earlier ratchet workarounds.
