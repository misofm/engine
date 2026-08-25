# The multiband's missing ramping split: what it saved, and where it could not be measured

**Candidate.** Issue #149 phase 3, opened by a finding of phase 1: the multiband compressor ran
twenty unconditional lane additions per frame — ten parameters on each of two channels — for
parameter smoothing, with no counter-based ramping split at all, unlike every other builtin.

**Status.** Adopted. Worth **6.4% of the multiband bank's own block cost** at the eight-lane width
with no automation in flight, reproducibly. Two boundaries are recorded here, because the number
this change is *worth* and the number the sprint's standing benchmark can *see* are not the same
number, and the second one is zero.

## Boundary 1 — the standing console fixture cannot see this change at all

`fixtures/session/v1/console-sixty-four-track.toml` carries a parametric EQ in `simd1` and a
compressor in `dynamic` on each of its sixty-four tracks. `simd2` is empty on all sixty-four. There
is no multiband compressor anywhere in it, and the nine-track ragged strip and the
hundred-and-twenty-eight track stretch are that same file cloned down and up. The stationary
hoist's ruling records the equivalent fact for its own arms; this one is starker, because the
effect under optimisation is simply absent from the workload.

Measured anyway, because a null result is worth more written down than assumed — the run is
`scripts/run-console-benchmark.sh --phase3`, record
`artifacts/issue149-phase3/console-benchmark.accepted.jsonl`:

| workload | phase 1 | phase 2 | phase 3 | phase 3 vs phase 2 |
|---|---|---|---|---|
| nine-track EQ-only baseline (control) | 13.235 | 13.255 | 13.285 µs | +0.23% |
| nine-track ragged strip | 44.945 | 36.109 | 36.079 µs | −0.08% |
| sixty-four-track console | 286.925 | 222.122 | **221.922 µs** | −0.09% |
| 128-track stretch | 578.088 | 449.124 | 448.622 µs | −0.11% |

Every column moves less than the EQ-only control does, which is the honest reading: this is
run-to-run noise on an unchanged workload, not a small win. The hoist rows agree — the sixty-four
track paired delta is 4 078 ns against phase 2's 4 058 ns, and the quiet arm is 31 329 ns against
31 330 — as they must, since those arms are EQ banks.

**So the post-phase-2 downstream numbers this change composes with are phase 2's, unchanged**:
221.9 µs/block p50 and 232.2 p99 at sixty-four tracks, 8.3% of one core at 48 kHz and a 128-frame
quantum. Phase 3 adds nothing to them and takes nothing away. Anyone reading the sprint's headline
should not credit this change with a share of it.

The fixture was deliberately left alone. Putting a multiband into `simd2` would move every
workload's output digest, redefine all three console workloads at once — they are one file — and
end their comparability with the phase-1 and phase-2 records, which are consumed one-shot
authorities. A fixture that measures the multiband is a separate, stateless piece of work.

## Boundary 2 — measured where the effect is, the win is real and it is not the additions alone

Measured at the multiband bank boundary, which is where phase 1 measured its EQ hoist and for the
same reason. Paired alternation per issue #104: the six subjects — three automation arms, split on
and split off — are interleaved observation by observation, one warmup pass and two measured
rounds, 1 000 observations each, 128-frame blocks at 48 kHz. The two arms of a pair differ only in
`FORCE_RAMPING`, so the paired delta is the split and nothing else: same construction, same
stimulus, same traffic, same plumbing, and the traffic is admitted outside the timed region.

Round 2, paired delta p50 (round 1 agreed to within 2 ns on every row):

| width | quiet | restated | moving |
|---|---|---|---|
| scalar | +110 ns/block (2.8%) | +551 ns (12.7%) | +50 ns (1.3%) |
| four-lane | +261 ns (4.4%) | +261 ns (4.4%) | +130 ns (2.1%) |
| **eight-lane** | **+432 ns (6.5%)** | +400 ns (6.0%) | +191 ns (2.8%) |

Three things in that table are worth reading rather than skimming.

**The moving arm still wins.** A sixty-four sample smoothing window opened at the top of a
128-frame block is closed for the second half of it. Before this change that half still ran the
twenty additions per frame with every step at `+0.0`; now it does not. That is the "a settled ramp
stops paying" half of the change, and it is visible as its own row: about half the quiet arm's
saving, which is about what half a block of savings should look like.

**The win grows with width, in nanoseconds, and shrinks per track.** Eight lanes save 432 ns a
block against four lanes' 261, because the additions removed are lane-wide either way — but per
frame per track that is 0.42 ns against 0.51 ns. The saving is per *bank*, not per track, which is
exactly what removing whole-bank arithmetic should produce.

**The scalar restated row is an outlier and is not claimed as the split's doing.** At 12.7% it is
four times the scalar quiet row, and the asymmetry sits in the split-*off* arm: split-on renders
restated and quiet in the same 3 787 ns, while split-off takes 4 338 ns restated against 3 888
quiet. The likeliest reading is that `store_segment`'s read-modify-write over ramp words that
`apply_automation` has just written costs more than the same write-back over untouched words, and
the split skips that write-back. That is a hypothesis about a store pattern, not a measurement of
one, so the row is reported and not explained.

## Why this is class A, and how that is checked rather than argued

Every rendered sample is byte-identical split on versus split off. The unsplit form was not deleted
to make that checkable: it is `FORCE_RAMPING`, `false` on every production path, and
`crates/miso-engine-multiband-compressor/src/split.rs` runs both arms over each boundary — a window
in flight when a block opens, one arriving mid-block, one arriving exactly on a block boundary,
overlapping windows on different tracks, a parameter restated at the value it already holds, a
restatement delivered mid-flight, and no traffic at all — at all three widths, under all three link
modes and bypassed, comparing every rendered sample *and* the whole instance afterwards by bit
pattern.

The soundness argument the tests check is narrow. `LinearRamp` keeps `remaining == 0` implying
`step == +0.0`, and every door a parameter word enters this crate by — prepared defaults, an
automation point, a snap, a restored payload — runs `normalize_zero` over a `parameter_value_valid`
word. So on the skipped path every step is `+0.0` and every current is finite and not `-0.0`, which
makes `current + step` the identity on every lane. The two exclusions are
`LinearRamp::stationary_at`'s own, for its own reasons: `-0.0 + 0.0` is `+0.0`, and an addition
quiets a NaN. `Instance::flat_path_is_identity` asserts that precondition in debug builds at the
point of use.

No digest moved and nothing was re-pinned. The wasm gates report 133 cases, 331 comparisons, 0
mismatches on all three legs, unchanged. The fast dB seal still counts exactly two calls in this
file: X5 and X6 are where phase 2 left them.

## Two gaps this work found in the crate it touched

Both are pre-existing, both are now closed, and both are recorded because an A/B comparison is
structurally blind to them: a fault in the kernel the two arms *share* cancels out of it, so
"identical to the unsplit form" was never going to catch either.

* **The right channel's ramp advance could be deleted outright** and every gate in this crate still
  passed. `the_ramped_path_advances_both_channels` now moves a makeup window on one channel at a
  time and pins that it moves that channel's own first sample and leaves the other alone.
* **D11's snap could be removed from the segment planner** and everything still passed, leaving a
  window to arrive at the iterated sum rather than on its target — a rounding error, and exactly
  the one the snap exists to remove. `a_window_lands_on_its_target_on_the_exact_sample` pins the
  arrival sample and the exact bits, and first asserts that its own window accumulates an error so
  that it cannot pass for the wrong reason.

Eleven mutations, eleven red, run in release where the debug precondition is compiled out: the
split inverted, forced flat, forced ramped; either channel's advance deleted; the write-back
skipped while ramping; the snap removed and moved to `remaining == 2`; the segment floor raised;
the ramping flag pinned false; the segment bound off by one.

## What would justify reopening what was left

* **A console fixture that contains a multiband.** Boundary 1 is a statement about the workload, not
  about the effect. Until such a fixture exists, this change's value is a bank-boundary number and
  should be quoted as one.
* **The per-segment overhead the split does not remove.** A quiet block still gathers twenty lane
  vectors in `Side::segment`, still scans a hundred and sixty counters in `plan_segment`, and still
  refreshes two band caches. That is per segment, not per frame, so it is small beside the 2 560
  additions removed from a 128-frame block — but it is what is left, and it is where the next
  measurable thing would be.
* **The twelve additions that are pure bookkeeping.** Of the twenty per frame, only eight are read
  by the frame body: each band's threshold and makeup. The other twelve — ratio, attack and release
  on each band of each channel — are advanced solely so `store_segment` can write back the right
  value, since `band_coefficients` reads the scalar ramps at segment granularity instead. They
  cannot be closed-form (iterated `+` is not `n *`), but they need not be interleaved with the
  dynamics in the hot loop. That is a register-pressure change, not a ramping one, and it is a
  separate candidate.

## Links

* Optimisation: issue #149 phase 3, opened by phase 1's investigation.
* Implementation: `crates/miso-engine-multiband-compressor/src/lib.rs` — `Instance::plan_segment`,
  `SegmentPlan`, `run_segment`'s `RAMPING`, `process_block`'s `FORCE_RAMPING`,
  `Instance::flat_path_is_identity`.
* Gates: `crates/miso-engine-multiband-compressor/src/split.rs` (bit-identity, the two closed gaps,
  and the descriptive paired-alternation measurement, `--ignored`).
* Bench protocol: `AGENTS.md` "Benchmarks are descriptive during feature development"; issue #104;
  `tools/miso-engine-bench/src/console.rs` "Paired alternation".
* Prior rulings in this sprint: `docs/rulings/stationary-smoother-hoist-boundary.md` (phase 1),
  `docs/rulings/fast-db-tier-boundaries.md` (phase 2).
