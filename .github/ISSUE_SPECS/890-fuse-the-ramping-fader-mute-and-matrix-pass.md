# Fuse the ramping fader, mute and matrix pass

## Product outcome

The settled output section is one fused pass (fader, mute, 2x2 matrix). As soon as any lane of a bank has a fader or matrix ramp in flight, the whole bank falls back to three passes (fader L, fader R, matrix), each split into a ramp segment and a settled tail, plus a few hundred scalar bookkeeping operations; a live console riding one fader keeps its bank there indefinitely. Add a fused ramping pass. Class A.

## Root evidence

- `crates/builtins/src/lib.rs:3796` `try_process_settled_with_matrix` returns `false` when either bank has a nonzero countdown; the fallback in `crates/builtins-compiler/src/lib.rs` near `:1089` runs `FaderRampStage::process_plane` (impl at `crates/builtins/src/lib.rs:2592`, plane method near `:2731`) twice, then `MatrixStage::process` (impl at `:2821`, method near `:2979`).
- Kernels: `fader_matrix_block` (`crates/lane/src/kernels/builtins.rs:292`, settled fused), `gain_mute_ramp_block` (`:211`), `matrix2x2_ramp_block` (`:350`). The rule "the identity select is not applied while the matrix ramps" is documented near `:295-296` and must be preserved.

## Smallest closable slice

Authorized paths: `crates/lane/src/kernels/builtins.rs` (new `fader_matrix_ramp_block`), `crates/lane/tests/`, `crates/builtins/src/lib.rs` (dispatch), `crates/builtins-compiler/src/lib.rs` (dispatch), their tests, and this spec.

One traversal per frame: load L and R, apply the per-lane `GainMuteRamp` step for each channel exactly as `gain_mute_ramp_block` does (same op order: sample*gain, andnot mute), then the matrix ramp or settled coefficient exactly as `matrix2x2_ramp_block`/`fader_matrix_block` do, store L and R. Keep the same ramp-segment/settled-tail split boundaries. Fold the per-block countdown/`sync_settled` bookkeeping into one pass over the lane words.

## Non-goals

No change to settled dispatch, to ramp step arithmetic, to the input section, or to filter ramps.

## Objective gates

1. New lane test: for random ramps (fader only, matrix only, both, with and without mute, at scalar/`Simd4`/`Simd8`), the fused kernel's output and the lane words after the block are bit-identical to running the three current kernels in sequence.
2. New builtins test: a bank with one lane ramping renders bit-identically before and after, across the ramp's start, middle, end and the first settled block.
3. `scripts/check-builtins-fixtures.sh`, `scripts/check-builtins-policy.sh`, `scripts/check-lane-policy.sh`, `scripts/check-realtime-policy.sh` pass.
4. One descriptive `console_automation` row before/after, attached.

## Dependencies

None.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.
