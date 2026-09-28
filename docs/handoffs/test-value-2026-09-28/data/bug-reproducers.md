<!-- Produced for the 2026-09-28 test-value audit (base a9414c0c) by a helper agent, then spot-checked by the auditor. Paths such as `ci/`, `tva/` or `method/` refer to the auditor's scratch directory, which was deleted after the audit; CI run and job IDs are enough to re-fetch the logs with `gh api`. -->

> **Auditor's note (2026-09-28).** This file covers the DSP and graph group only. Also keep the
> seven #970 reproducers in `crates/host-core/tests/collapse_arming.rs`, which re-injecting #970
> at HEAD turns red. When #994 was re-injected at HEAD (the `if true` revert of
> `knee_coefficients`), `knee_overflow.rs:185`
> `an_automated_knee_ramp_through_the_overflow_band_does_not_duck` stayed **green**. It no longer
> reproduces its bug on its own and should be repaired or replaced; see `../TEST-VALUE-AUDIT.md`
> §4.2.


# Regression reproducers for #966, #970, #994 and #1015 in the DSP/graph group

Source of truth: the revert rows in each crate's `tests/MUTATIONS.md`. A test is listed as a
reproducer only when a recorded mutation that restores the pre-fix behaviour turned it red.
Paths are relative to the repository root at a9414c0c. Every test below must stay. Where a
recorded mutation turns exactly one test red, that test is the only guard for the mutation and is
marked **sole guard**.

## #966: a graph-compiler effect bank spanning dependency levels

Record: `crates/graph-compiler/tests/MUTATIONS.md:276-293`. Command `cargo test -p graph-compiler
--test bank_levels` (9 tests, all in `crates/graph-compiler/tests/bank_levels.rs`). CI runs the file in
`test-debug-a`.

| test (file:line) | red under the revert (966-M1, M10) | other recorded mutations that turn it red |
|---|---|---|
| `bank_levels.rs:881 the_sixty_four_track_console_less_one_eq_binds_at_every_width` | M1, M10 | none (reproducer, ragged lane first in group) |
| `bank_levels.rs:896 the_console_less_a_middle_lanes_eq_binds_at_every_width` | M1, M10 | M3 (first/last-only comparison); M3 is also red in the seed-412 shape and the probe |
| `bank_levels.rs:908 the_console_less_a_last_lanes_eq_binds_at_every_width` | M1, M10 | none |
| `bank_levels.rs:920 a_cohort_lane_behind_an_extra_dynamic_eq_binds_at_every_width` | M1, M10 | M3, with :896 and the probe (the #962 seed-412 shape) |
| `bank_levels.rs:933 the_mono_console_less_one_eq_binds_and_collapses_at_every_width` | M1, M10 | M9, with the probe (the armed leg sees a wrong collapse) |
| `bank_levels.rs:960 the_reduced_mono_console_from_the_970_probe_binds_at_every_width` | M1, M10 | none |
| `bank_levels.rs:976 lanes_that_skip_the_same_slot_still_bank_it` | M1, M10 | M2 (over-strict check), with :995 |
| `bank_levels.rs:995 a_slot_after_a_misaligned_one_realigns_and_still_banks` | M1, M10 | M2, and **sole guard** of M7 (`continue` becomes `break`) |
| `bank_levels.rs:1088 randomized_consoles_compile_bind_and_render_the_scalar_bits` | M1 (28 lines, 22 seeds), M10 | **sole guard** of M8 (skip lane 1) and of 966-M970 (see #970) |

Three of the nine (`:920`, `:960`, `:995`) have no assertion macro in their own body. They assert through the helpers
`assert_binds_and_renders_the_scalar_bits`, `misaligned_slots` and `native_bank_count`
(`bank_levels.rs:797-872`), so they are not vacuous. `native_bank_count` returns early only on a
scalar host, so CI (Simd8) runs it.

## #970: the mono collapse armed on a later chain

The fix is in `crates/graph/src/runtime.rs` (commit 954790d1). Its reproducers (six plus a count gate; the fix's commit message says "seven", and the Sol review
nit at spec line 256 corrects it) live in `crates/host-core/tests/collapse_arming.rs`, which is outside this group. The #970
spec (`.github/ISSUE_SPECS/970-…md:157-167`) also records `console-workload --test chain_shape` as red
under the over-disarm mutation M2. That file is outside this group too.

Inside this group the only recorded catcher of the #970 revert is
`crates/graph-compiler/tests/bank_levels.rs:1088 randomized_consoles_compile_bind_and_render_the_scalar_bits`:
966-M970 turns it red on 5 armed lines over seeds 12, 48, 50 and 54 (`MUTATIONS.md:293`). The same
row records that the other eight `bank_levels` tests stay green under the revert. No test in
`graph` (runtime.rs, lib.rs), `rack` or `builtins-compiler` has a recorded red under the #970
revert. The #970 fix commit added no test to `graph`.

## #994: a compressor knee whose reciprocal overflows

Records: `crates/effect-runtime/tests/MUTATIONS.md:227-249`, `crates/compressor/tests/MUTATIONS.md:348-362`,
`crates/multiband-compressor/tests/MUTATIONS.md:203-216`. Under the revert (the `is_finite` test
dropped), `--no-fail-fast` across effect-runtime, compressor and multiband showed "exactly the eight
#994 tests failed … No gate that existed before #994 saw the overflow" (effect-runtime
`MUTATIONS.md:241-244`).

| test (file:line) | red under | role |
|---|---|---|
| `crates/effect-runtime/tests/dynamics.rs:283 the_overflow_bound_is_exact_and_only_it_moves` | 994-R1, R2, R3 | reproducer + exact-bound guard (both one-ulp mutations) |
| `crates/effect-runtime/tests/dynamics.rs:345 a_knee_whose_reciprocal_overflows_is_a_hard_knee` | 994-R1, R3 | reproducer |
| `crates/effect-runtime/tests/dynamics.rs:426 a_randomized_sweep_never_leaves_the_finite_curve` | 994-R1, R3 | reproducer (randomized) |
| `crates/effect-runtime/tests/dynamics.rs:385 the_narrowest_soft_knees_are_finite_at_the_threshold` | 994-R2 only | over-reach guard. With `:283`, it is one of two tests that see the bound set one ulp too high |
| `crates/compressor/tests/knee_overflow.rs:138 a_prepared_overflowing_knee_does_not_duck_a_sample_at_the_threshold` | 994-C1, C2 | reproducer (prepare entry) |
| `crates/compressor/tests/knee_overflow.rs:185 an_automated_knee_ramp_through_the_overflow_band_does_not_duck` | 994-C1 | reproducer (automation entry; the ramp passes through the band) |
| `crates/compressor/tests/knee_overflow.rs:213 a_restored_overflowing_knee_does_not_duck` | 994-C1 | reproducer (restore entry) |
| `crates/compressor/tests/knee_overflow.rs:241 a_bank_of_overflowing_knees_does_not_duck` | 994-C1, C2 | reproducer (bank entry; `native_bank_width()` skip, CI runs at Simd8) |
| `crates/compressor/tests/knee_overflow.rs:151 the_narrowest_soft_knees_do_not_duck_a_sample_at_the_threshold` | none (green under C1 and C2 by design) | preservation guard, not a reproducer (`compressor/tests/MUTATIONS.md:361-362`) |
| `crates/multiband-compressor/src/lib.rs:1920 knee_tests::a_band_level_at_the_threshold_never_takes_a_nan_target` | 994-M1, M3 | reproducer (band curve) |
| `crates/multiband-compressor/src/lib.rs:1884 knee_tests::the_fixed_knee_words_are_unchanged` | 994-M2 only | pins `BAND_KNEE` to the shared design. It is a word pin, but it is the sole guard of M2 |

The four compressor tests share one claim (no duck at the threshold for an overflowing knee)
through four entry points. The record shows they are not interchangeable: C2 turns only `:138` and
`:241` red, and each test reaches the shared design through a different entry.

## #1015: restored subnormal EQ state elided (-0.0 vs +0.0)

Record: `crates/parametric-eq/tests/MUTATIONS.md:409-427`. Driver: the old leg (c) restored in
`cascade_sections` and/or `cascade_sections_mono`; `cargo test -p parametric-eq --lib`, dev and
release.

| test (file:line) | red under | role |
|---|---|---|
| `crates/parametric-eq/src/lib.rs:8015 a_restored_subnormal_live_state_renders_the_full_cascade_bits` | 1015-M1, M1d, M1m, B2 | reproducer (channel level, f32/Simd4/Simd8 in one test, dual and collapsed) |
| `crates/parametric-eq/src/lib.rs:8024 a_restored_subnormal_live_state_renders_the_full_cascade_bits_through_the_contract` | 1015-M1, M1d, M1m, B2 | reproducer (contract restore and `process_bank`/`process_bank_mono`) |
| `crates/parametric-eq/src/lib.rs:8136 leg_c_refuses_below_flush_eps_admits_it_and_re_engages` | 1015-B1, B2, B3 | boundary guard (**sole guard** of B1 and B3: the `FLUSH_EPS` edge) |

Related but not a #1015 reproducer: `crates/parametric-eq/src/lib.rs:6728
a_restored_subnormal_live_state_refuses_the_list` was added by #1005 (commit 39908196) for the
ramping list's leg (c), the same bug class on the ramping path. It is the sole gate of 1005-M8
(parametric-eq MUTATIONS.md, #1005 section), so it stays as well.

Nearest in-group guards of the same class, -0.0 against +0.0 in an elision decision, with no
recorded #1015 row: `crates/lane/tests/input_chain_elision.rs:377` (-0.0 mix words) and `:433`
(-0.0 state words refused by the bitwise gate).

## Summary

- 23 tests in this group must stay as regression guards for the four bugs:
  - 9 for #966, of which 1, the probe at `bank_levels.rs:1088`, is also the only in-group #970 catcher;
  - 11 for #994: 10 are recorded red under a #994 mutation, and 1 (`knee_overflow.rs:151`) is a
    preservation guard;
  - 3 for #1015.

  Add the #1005 sibling `parametric-eq/src/lib.rs:6728` for the same bug class on the ramping
  path. The four compressor and three effect-runtime R1 reproducers, plus multiband `:1920`, are
  the "exactly eight" tests the effect-runtime record names.
- The only in-group guard against re-arming a later chain (#970) is one randomized probe,
  `bank_levels.rs:1088`. Its catch depends on the generator keeping both the symmetric and the
  asymmetric all-mono strip (`MUTATIONS.md:293`). If that probe's seed count or generator is ever
  trimmed, re-run 966-M970 first.
- None of these tests is a cut candidate in `candidates.md`.
