# Compressor: make the ramping prefix cost scale with the ramping lanes


Compressor follow-up (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/COMPRESSOR-DIAGNOSIS.md`, section 1, "Collapsed (mono) body and
the ramping prefix". This slice was **measured but not prototyped**. It is a bounded first slice,
and it carries its own measurement gate.

## Product outcome

When a parameter receives a Point, its 64-sample `Linear 64` ramp makes `process_block` run
`frames_loop::<L, true>` (`crates/compressor/src/kernel.rs:452-497`) for the ramp's frames. That
happens during a knob drag and during automation playback. The ramping body is the generic loop,
and for every frame it:

* runs `Channel::advance_ramps` (`:207-238`) for both channels. That scans 7 ramps times
  `L::WIDTH` lanes, advances every in-flight ramp as scalar per-lane `LinearRamp::next_value`, and
  re-designs the static curve per lane per frame (`GainComputerCoef::new`: two divisions and a
  branch) when threshold, ratio or knee is ramping;
* reloads `Coef` for both channels (8 vector loads and 3 compares each);
* runs today's one-pass frame law.

A lane that is not ramping still pays the scan, the reloads and the generic loop for the ramp's
length. Measured with one fixture bank (EPYC 7313P, pinned, in process; the `real_bank_ramp*` rows
of `compressor-diagnosis-harness.patch`):

| one bank-block, 128 frames | native `Simd8` | V8 `Simd4` |
|---|---:|---:|
| settled | 4.91 us | 5.00 us |
| one lane's threshold Point every block | 8.79 us (**+79 %**) | 9.59 us (**+92 %**) |
| all 8 lanes' threshold Point every block | 13.2-13.4 us (+170 %) | 12.75 us (+155 %) |
| all lanes' attack (a coefficient ramp, no curve redesign) | 11.0 us | not measured |
| all lanes' makeup (a pass-through word) | 12.4-12.7 us | not measured |

Only 64 of the 128 frames ramp. So a ramping frame costs about 2.6 times a settled one when one
lane of eight ramps, and about 4.4 times when all eight do. No standing console row automates a
compressor, and the frozen MQ-2 measurement (`crates/compressor/tests/bench_ramp.rs`) covers
attack and release only.

## Invariants

* **Class A.** Every output word, recursive word, ramp state (`current`, `target`, `step`,
  `remaining` of each `LinearRamp` and rate ramp), coefficient word, report and payload is
  unchanged. `ramps`, `partition`, `native_points`, `payload`, the `coefficient_ramp_tests` unit
  tests and the pinned `cross_target` case `dual_mono_ramping` are the existing gates.
* A lane whose ramps are all finished must not be advanced. Today's `advance_ramps` already skips
  its design, and `LinearRamp::next_value` on a finished ramp is the identity
  (`docs/rulings/compressor-idle-lane-guard-console-under-resolved.md`). This slice must not change
  the values any lane computes.
* Allocation-free. No new heap state. A per-block, stack-local lane mask is allowed.

## Scope of the first slice (choose after step 1's measurement, then record the choice here)

1. **Measure first.** Add to `crates/compressor/tests/bench_ramp.rs` a `threshold_one_lane` and a
   `threshold_all_lanes` arm, alongside `release_only` and `attack_and_release`. Follow MQ-2's
   frozen protocol: one warmup and two measured rounds, run only through
   `scripts/run-issue880-mq2-benchmark.sh`, and preflight without timing. Freeze the arms before
   anything is timed.
2. **Then one of these, whichever the measurement ranks first:**
   * (a) A per-block ramping-lane mask per channel, computed once from `remaining`, so the frame
     loop advances and re-designs only the lanes in it, and reloads only the coefficient words
     those lanes change.
   * (b) Slice 1's loop hygiene applied to the ramping body: chunked iteration, the recursive word
     in a local, the detector hoisted.
   * (c) The static-curve redesign of a ramping lane computed once per frame as a `Lane`-wide
     `GainComputerCoef` over the ramping lanes. This is only class A if it is the same `f32`
     division and the same branch per lane; prove it with a word-for-word test against today's
     scalar `design_lane`.

## Non-goals

* Changing the smoothing law (`Linear 64`) or the ramp length. Both are class B and would need a
  ruling.
* The settled body (slices 1-5).

## Objective gates

1. **Old body is the oracle.** A kernel-level test that drives identical `Channel` pairs through
   today's ramping body and the new one. It covers:
   * at `f32`, `Simd4` and `Simd8`;
   * Points on every parameter, singly and together, on one lane and on all lanes, in both
     channels, and at mid-ramp retargets (`coefficient_ramp_tests` shows the sequence);
   * hostile audio.

   Compare every output word, recursive word, ramp field and coefficient word by bits. Run in dev
   and release.
2. **Existing gates stay green:** `ramps`, `partition`, `native_points`, `payload`,
   `cross_target`, `mono_collapse`, and the kernel's `coefficient_ramp_tests`.
3. **Mutations.** Apply each alone and record it red:
   * a ramping lane dropped from the mask;
   * the mask computed from the left channel only;
   * the coefficient reload skipped for a changed word.
4. **The measurement of step 1**, recorded before and after, is descriptive and not a threshold.

## Dependencies

None. It is independent of slices 1-5, but it touches `frames_loop`, which slice 1 keeps as the
oracle of the settled body. Land it after slice 1, and keep a copy of today's ramping body as this
slice's oracle.

## Standing rules for the implementer

As slice 1. Benchmarks are descriptive. Freeze the workload before timing, and do not tune or
retry.

