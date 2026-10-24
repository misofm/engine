# Refresh the stale KERNEL_ROSTER derivation table and the AudioWorklet check's count table

Filed by root from #1920 (backlog sprint, 2026-10-24).

## Defect

The derivation table in `scripts/check-web-audioworklet-callgraph.py` and the prose count table in `scripts/check-web-audioworklet.sh` give vector/scalar counts that no longer match the named twin. For example the compressor measures 949/0 against a listed 267/0, and the parametric EQ measures 1352/1 against a listed 168/0.

## Smallest closable slice

1. Measure every roster row once on the named twin (`build-web-audioworklet.sh --module-only --named-twin`, pinned nightly) and update both tables.
2. Check that each ceiling still follows the table rule. For the parametric EQ's one scalar op, name it and confirm it is within its ceiling. Do not raise a ceiling to make a row pass: report it.
3. Docs and prose only, plus the ceilings that the rule derives. No kernel change.

## Paths

`scripts/check-web-audioworklet-callgraph.py`, `scripts/check-web-audioworklet.sh`, this spec.

## Evidence and decision record (attempt 1)

- Measured on the named twin built from `019194414` (`build-web-audioworklet.sh --module-only --named-twin`, toolchain 1.99.1 from `rust-toolchain.toml`, which is what CI's build uses). Rows that moved (vector / scalar, old -> new): compressor dual 267/0 -> 949/0, compressor collapsed 138/0 -> 524/0, limiter dual 448/0 -> 913/0, limiter collapsed 224/0 -> 424/0, parametric EQ dual 168/0 -> 1352/1, parametric EQ collapsed 84/0 -> 748/1, soft-clip 25/0 -> 55/0. The other eleven rows are unchanged.
- The EQ's one scalar op per body is an `f32.sub` once per block: `lane::silence_armable_holding` computes `armed_after - frames` from two splats, and LLVM does the subtraction on the scalars before the splat. Ratios 0.0007 and 0.0013; four times them is below the 0.10 floor, so every ceiling stays 0.10. No ceiling changed.
- The merge of `origin/main` at `e88082e1a` touched no roster kernel, so no row was re-measured.
