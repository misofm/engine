# Drop the identity select from the fused fader-matrix kernel when no lane is the identity

## Amendments (adversarial verification, 2026-09-27; these override any conflicting text below)

The verification (evidence: `docs/handoffs/gain-pan-2026-09-26/VERIFY-954.md`) found the x86
premise false and the browser case real.

- **A1. What the select costs.** The release `bench` binary's fused loop
  (`FaderMatrixBankProcessor::process`) has no `vmaskmovps`: its select's first arm is
  `load * g andnot mute`, not the loaded word, so LLVM emits two `vblendvps` feeding plain `vmovups`.
  Wasm emits two `v128.bitselect` per frame. Measured, one runtime, interleaved, digests identical:
  native fused gain/pan -170 ns of 10.41 us per block (builtins -150 ns, console -200 ns, the
  metered row unresolvable); **wasm under V8, in place: gain/pan -1.13 us of 21.14 us, builtins
  -1.07 us, console -0.98 us**. Today on wasm the product's fused gain/pan plan (21.14 us) is slower
  than the split plan (20.57 us); this issue brings it to 19.99 us. The case for the issue is the
  browser. Codegen evidence becomes: the select-free loops contain no `vblendvps` and no `bitselect`,
  and the select arm keeps them.
- **A2. Call sites.** The kernel is at `crates/lane/src/kernels/builtins.rs:346`. Its four callers:
  `crates/builtins/src/lib.rs:3833` and `:3845` (the banked product path), `:4179` (per-track
  `process_fader_matrix`) and `:3119` (`BuiltinChain`, tools only). All four take the new dispatch.
- **A3. Where the guard runs.** Evaluate `L::mask_any(matrix.coef.identity)` inside each function,
  after its settled check, which runs after the control drains
  (`crates/builtins-compiler/src/lib.rs:1070-1071`). Never cache it. The fused path has no ramp tail
  (a ramping block falls back to the split stages), so there is no tail site and no M4.
- **A4. Gate 1, two oracles.** Compare against the select form with an all-false mask and against
  `gain_mute_block` followed by `matrix2x2_block_without_identity`; cover mixed mutes and gains of
  0 and 1; NaN words compare as "both NaN"; run in dev and release. (Release saw 0 changed non-NaN
  words and 54 changed NaN payloads at `f32`, the #944 class statement.)
- **A5. Scenario test,** through `try_process_settled_with_matrix`, pinned on the unmodified base: a
  full bank with an exact `IDENTITY` lane and one frame with `l = -0.0, r = +0.0` (so M1 fails on the
  output words), an instant (0-sample) retarget to `IDENTITY`, and a partial bank. Add a dispatch
  witness for M2.
- **A6. Row gates.** Every `WORKLOADS` row uses concurrent delivery and makes zero fused calls, so
  "every console digest unchanged" witnesses nothing here. The row gates are #881's metered pair test
  and `composite_live_sequence_…`; M3 fails both (checked).
- **A7. Benchmark statement.** No native row can resolve the saving; say so. The wasm artifact is
  repinned at the batch boundary; the kernel count and render closure are unchanged (checked).

## Product outcome

#944 gave the settled, unpaired pan matrix a select-free arm when no lane of a bank is the exact identity, and the gain/pan row moved from 14.1 to 11.1 us per block (every digest unchanged). The #881 verification found that the default web boot does not take that path: between-render-calls delivery fuses each cohort's fader and matrix into one stage, `fader_matrix_block` (`crates/lane/src/kernels/builtins.rs`), which #944 left untouched as a non-goal. The product's fused stage therefore still evaluates the per-lane identity select every frame (on x86 folded into a `vmaskmovps` masked store, which #944 measured at about 10 extra cycles per frame on Zen 3; on wasm a `bitselect`). Give the fused kernel the same select-free arm.

## Smallest closable slice

Mirror #944 on the fused pair: a `fader_matrix_block_without_identity` kernel (the second arm verbatim, no `L::select`), chosen once per call when `!L::mask_any(identity)` over all lanes, padding included, at every settled call site of the fused pair; the existing kernel stays the oracle. Same class statement as #944 (every non-NaN word unchanged, a NaN stays NaN; LLVM may commute the final add), same gates adapted: dev and release differential against the oracle at `f32`, `Simd4` and `Simd8`; a scenario test pinned on base; every console digest unchanged; red mutations; x86 disassembly showing plain stores; wasm census via the build script's cargo line.

## Rows it can move

`sixty_four_track_console_metered` (#881) and any plan prepared with between-render-calls delivery. No digest may move.

## Dependencies

#944 (merged in the optimisation batch), #881.

Found by the #881 attempt 1 verification.
