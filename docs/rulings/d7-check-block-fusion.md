# D7 `check_block` fusion into the kernels' final pass — null

**Candidate.** Issue #163 phase 4 item 5. The master-plan §4.4 non-finite boundary scan
(`miso_engine_effect_runtime::bank::check_block`) runs as a **separate full-block vector pass per
channel per effect**, after the kernel has already written that block. D7 requires the *check*, not
the separate traversal, so the proposal was to fuse the accumulate
(`ok = mask_and(ok, |x| < BLOCK_LIMIT)`) into each kernel's existing final pass, deleting one whole
read of every block per channel per effect. The check reads and never writes, so fusing moves no
rendered bit — provided it does not perturb a kernel's frozen operation order.

**Claim under test.** That the deleted traversal is a material share of the active console block.

## Measurement

Frozen workload: the standing 64-track console bench (`scripts/run-console-benchmark.sh`), the
phase-1 subject and fixture, 48 kHz, 128-frame quantum, 1 000 observations, `Simd8`.

The candidate is measured as an **upper bound** rather than by implementing the fusion: the whole
of `check_block` was stubbed to `return true`, which deletes the traversal *and* the check. Any
real fusion can only recover a subset of this, because the fused form still performs the compare
and the once-per-block `mask_any`; it removes only the second read of the block.

Baseline is the phase-4 tree at `e5c75f4`. Both arms were captured back to back on the same host in
the same load epoch. `sixty_four_track_dispatch_only` is the internal drift control: its strip
carries no effect at all, so it executes **no** `check_block`, and its arm-to-arm delta is pure
host drift.

| row | phase 4 (µs/block) | check stubbed | raw delta | drift-corrected |
|---|---|---|---|---|
| `sixty_four_track_console` | 110.330 | 108.576 | +1.59% | **+0.28%** |
| `sixty_four_track_eq_only` | 60.475 | 59.312 | +1.92% | +0.61% |
| `sixty_four_track_compressor_only` | 72.207 | 70.935 | +1.76% | +0.45% |
| `one_twenty_eight_track_stretch` | 224.055 | 219.066 | +2.23% | +0.92% |
| `sixty_four_track_idle` | 36.389 | 36.048 | +0.94% | −0.37% |
| **`sixty_four_track_dispatch_only` (control)** | 22.192 | 21.902 | **+1.31%** | — (by construction) |

## Ruling: null

Deleting the entire D7 traversal is worth about **0.3% of the 64-track console block**, and the
measurement sits at or below this host's drift floor — the no-effect control moved +1.31% between
the two arms, more than the console row's own raw delta. The honest statement is therefore a
**bound, not a value**: the traversal costs less than the noise, and a fusion that recovers part of
it cannot be resolved by this instrument at all.

Against that, fusion is not free to build or to own:

* `miso-engine-parametric-eq` would have to thread a mask accumulator through `svf_block` and
  `svf_block_ramped` in `miso-engine-lane`, which are **shared, frozen kernels** used by other
  effects, and through the per-section ramp-segment split that cuts a block at every distinct ramp
  end. That is precisely the "if fusing into a kernel would touch its frozen order, skip that
  kernel and record it" case in the phase-4 brief.
* `miso-engine-compressor` could fuse inside its own crate, but its final store sits in a
  `RAMPING`-generic per-frame loop whose register pressure is already the thing that makes the bank
  kernel fast. Adding an accumulator there risks a regression larger than the 0.45% it could win.

**The idle path does not want it at all.** Phase 4 item 1's silent fast path skips the kernel *and*
its boundary scan together, and engages on 100% of the idle row's timed blocks, so on the row this
phase exists to improve the traversal is already gone.

## Boundary of this ruling

Rejected: fusing the D7 scan into the existing final pass of the EQ and compressor bank kernels, on
the evidence that the traversal it removes is below this instrument's noise floor on the active
console row. Not ruled on, and not rejected:

* fusing in `miso-engine-delay`, which calls `check_block` **four times** per block (output, two
  ring segments and the damping word) — a different and much larger ratio that was not measured
  here because the console fixture carries no delay;
* the wasm/`Simd4` case. Every number above is native AVX2. A target where one FMA costs ~54
  instructions has a different arithmetic-to-bandwidth ratio, and the traversal's relative share
  there is unmeasured (see #163 phase 0b);
* removing or weakening the D7 check itself, which is not on the table.

## Reopening

New evidence that would justify revisiting: a fixture whose strip carries a delay or another
multi-scan effect; a wasm-native measurement of the same decomposition; or an instrument whose
drift floor is materially below 1% per row, which would let the 0.3% be resolved rather than
bounded.

## Provenance

* Issue #163 phase 4 item 5; subject tree `e5c75f4`, worktree `floor-phase4`.
* Measured 2026-08-25 on the phase-0a preconditions host (AMD Ryzen 7 9700X, pinned core).
  Both arms **uncontrolled** by the #163 phase 0a preconditions — a concurrent workspace test run
  in a sibling worktree held the load average near 9 — which is why the drift control carries the
  ruling rather than the absolute numbers, and why the result is stated as a bound.
* Baseline record: `artifacts/issue163-phase1/console-benchmark.accepted.jsonl`.
* Phase-4 record: `artifacts/issue163-phase4/console-benchmark.accepted.jsonl`.
