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
- **Fix.** `plan_segment` caps the segment at one frame while any ratio, attack or release ramp is in
  flight (`COEFFICIENT_RAMPS`). Each lane's coefficients then follow its own ramp frame by frame (the
  value at the frame's start); when none of them ramps, every lane's coefficient words are constant,
  so the sample point cannot matter. Threshold-only and makeup-only ramps keep long segments, so the
  crate's corpus (whose two automation points are thresholds) keeps its digests.
  A lone instance's ratio, attack and release now ramp over 64 samples; before, a lone instance held
  the old words for 63 frames and stepped at the arrival.
- **Not chosen.** Interpolating the designed coefficients (the compressor's coefficient ramps) needs
  new ramp state in the payload and a layout change; a per-frame lane-wide exponential needs a lane
  `expf` bit-identical to `math::expf`. Both are larger than this slice. Either is a possible later
  optimisation if the one-frame segments' cost matters.
