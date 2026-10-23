# Multiband compressor: a ramp's cut moves a lane's bits in a bank

Found by #1051's randomized differential (`crates/multiband-compressor/tests/randomized.rs`, ignored reproducer "#1051 defect 1", seed 1).

## Problem

When a parameter ramp is cut (a new automation segment replaces an in-flight ramp), one lane of a banked multiband compressor renders different bits from the same track rendered alone. That breaks the class-A rule that banking regroups lanes and never changes per-lane arithmetic, and partition invariance: moving a track into or out of a bank must not move a rendered bit.

## Smallest closable slice

Find where the cut applies per bank rather than per lane (or reads a neighbour's ramp state), fix it so each lane's ramp state is independent, and un-ignore the reproducer.

## Gates

1. The reproducer passes at seed 1 and across the randomized differential's seeds, at Simd4 and Simd8 and against the scalar oracle.
2. A planted cross-lane ramp read turns it red.
3. Console digests unchanged unless the defect affects a console workload; if so, explain every moved digest.
4. Render stays allocation-, lock- and syscall-free.

## Evidence and decision record

Attempt 1 (implementer), on main be2044c42.

- **Reproduced on main.** The ignored reproducer failed at seed 0 (the first seed run): `Four` bank,
  block 1, lane 0 left frame 41, bank `0xc08a912e` against scalar `0xc08a9049`.
- **Root cause** (line numbers at be2044c42, `crates/multiband-compressor/src/lib.rs`).
  `process_block` (1129-1133) calls `Side::band_coefficients` (1277) once per segment. It samples each
  lane's ratio, attack and release ramps (ramps 1-3 and 6-8) at the segment's first frame and holds
  the designed words (`1/R - 1`, two retention coefficients) for the whole segment; threshold and
  makeup advance per frame inside the kernel. `Instance::plan_segment` (1326, the scan at 1343) ends a
  segment at the nearest arrival of any ramp of any lane on either channel, and at the block end. So
  where a neighbour lane's ramp, the other channel's ramp or the host's block ends decided when this
  lane's coefficient words moved. That is the cut that moves bits: a cross-lane (and cross-channel)
  dependence of the sample point, not a cross-lane read of ramp state.
- **Attempt 1 (FAIL on design and cost).** `plan_segment` capped the segment at one frame while any
  ratio, attack or release ramp was in flight. Correct, but the bank ran 3.4x slower at W8 (2.6x at
  W4) during such a ramp, and one ramping lane cost almost as much as eight.
- **Attempt 2 design: the compressor's precedent** (`crates/compressor/src/kernel.rs`,
  `design_curve` and `rate_ramps`/`set_parameter_target`). Segments are arrival-bounded again.
  - Ratio: on a ramped segment the kernel derives `1/R - 1` lane-wide every frame from the ratio
    word after that frame's step, the convention threshold and makeup already use. Lane division
    and subtraction are the IEEE operations the scalar `1.0 / ratio - 1.0` ran, so settled bits
    do not move.
  - Attack and release: four coefficient ramps per channel and track (slots 10-13 beside the ten
    parameter ramps). `apply_automation` designs `retention_coefficient(target)` once per time
    retarget, and the kernel walks the coefficient word to it with `ramp_toward` over the same
    64 samples. `plan_segment` snaps it to the exact design and scans it, so a retarget to the
    time in force (whose parameter ramp the hoist settles at once) still returns a coefficient
    caught mid-walk to the exact design. That is the compressor's Point-cancels rule, kept by the
    scan.
  - The per-track `BandCache` and `band_coefficients` are gone: no exponential runs on the render
    path between retargets, and a flat segment derives its coefficients once.
- **Semantics.** Ratio ramps linearly in R; attack and release ramp linearly in the coefficient
  domain, as in the compressor, not in milliseconds. A mid-ramp frame differs both from the old
  value held and from the target jumped to (`tests/identity.rs`, `partition_invariance`).
- **State payload.** Each channel gains four four-word coefficient ramps after the silence counter:
  64 words, 256 bytes per channel (was 192). Restore holds each coefficient word and target to
  `(0, 1]`, inside its path, and settled at its target, which is the compressor's rule. The layout
  identity stays `STATE_LAYOUT_VERSION = 1` (prelaunch). The sizes are pinned only in
  `tests/product.rs`, which was updated. The corpus restores through snapshot then restore, and
  the conformance payload gates read the descriptor's sizes.
- **Changed bits.** Against main and attempt 1, every render with no ratio, attack or release ramp
  in flight is bit-identical. No committed digest moved: `corpus_digests.in` is untouched, and the
  corpus automates thresholds only. During such a ramp the bits differ by design:
  - main held the old words until the arrival (as a lone instance; any cut moved them);
  - attempt 1 redesigned the exact exponential every frame from the value at the frame's start;
  - attempt 2 uses the value after the step, with times interpolated in the coefficient domain.
- **Cost** (frozen bench1069 scratch workload: 128-frame blocks, 4,000 per round; one invocation,
  one warmup, two measured rounds; ns per block, m1/m2, main against attempt 2):

  | width | workload | main | attempt 2 |
  |---|---|---|---|
  | 8 | None | 9995 / 9964 | 10004 / 10027 |
  | 8 | ThresholdAll | 11598 / 11604 | 12214 / 12199 |
  | 8 | RatioAll | 11682 / 11684 | 12195 / 12208 |
  | 8 | RatioLane0 | 11572 / 11564 | 12152 / 12145 |
  | 4 | None | 9221 / 9225 | 9215 / 9214 |
  | 4 | ThresholdAll | 10455 / 10453 | 10876 / 10895 |
  | 4 | RatioAll | 10497 / 10503 | 10930 / 10905 |
  | 4 | RatioLane0 | 10453 / 10449 | 10838 / 10852 |

  The ratio rows sit on the threshold rows (attempt 1: 39,543 at W8 RatioAll). Every ramped row is
  about 4-5% above main. The arithmetic explains it: each ramped frame now steps 8 more slots
  (2 channels x 4 coefficient ramps, `ramp_toward` = add, min, max, max, min, with step and target
  reloaded from the stack, since 84 ramp vectors exceed 16 ymm registers) and runs 4 lane divisions.
  W8 ThresholdAll is +605 ns over 63 ramped frames, about 9.6 ns or 36 cycles per frame at
  3.7 GHz (EPYC 7313P, Zen 3), which is that work. No row is unexplained.
  A segment-uniform switch that skipped the coefficient slots on threshold-only ramps took
  ThresholdAll to +1.8% (W8), but moved the settled W8 None row +4% in its one run. It was reverted
  in favour of the simpler kernel (records `fix-a2b.txt` and `main-a2b.txt`). Making
  threshold-only ramps free is a weekly-optimisation candidate, not a #1069 gate.
