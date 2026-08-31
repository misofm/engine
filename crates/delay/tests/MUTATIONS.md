# Red mutations for the issue-#93 delay gates

Every test in `crates/delay` (the `tests` module of `src/lib.rs` and
`tests/determinism.rs`) was seen **red** under the mutation named here before it was committed
green, per issue #125's rule and master plan #83 §1.6. Each row is one edit to production code, the
test that caught it, and what the failure looked like. Reproduce by applying the edit and running
`cargo test -p delay --all-targets`.

Mutations are listed in the order the gates appear in the issue-#93 plan's eval table.

| # | eval | mutation (production edit) | test that goes red | observed failure |
|---|---|---|---|---|
| M1 | E1 | `resources`: the ring is `2*Fs + 2` words instead of `2*Fs + 3` | `descriptor_exact_resources_caps_and_integer_mapping_are_frozen` | ring word count 88202, expected 88203 |
| M2 | E2 | `DelayLane::copy_window`: the window starts one cell late (`cursor + r - d + 1`) | `tap_timing_is_sample_exact` | the 1 ms impulse no longer lands on sample 48: bits 0 instead of 1.0 |
| M3 | E3 | `damping_coefficient` returns the raw control `c` instead of the designed `g` | `damped_matrix_tail_matches_reference_oracle` | worst deviation 7.6e-3 at 48 kHz against a 4e-6 bound |
| M4 | E4 | the `min(19_845 Hz)` cutoff clamp is dropped | `damping_mapping_is_rate_invariant_and_monotone` | recovered cutoff -8919 Hz at 44.1 kHz: `tan` has gone past Nyquist |
| M5 | E5 | `ramp_bound`: the bound is `remaining` rather than `remaining - 1`, so the D11 snap lands inside a chunk | `ramp_updates_retarget_and_partition_are_exact` | the chunked coefficient sequence stops matching `LinearRamp::next_value` |
| M6 | E6 | `tap_sample`: update 128 blends instead of selecting the new tap | `crossfade_updates_are_exact_and_queued_retarget_completes` | sample 127 is `+0.0` instead of the new tap's `1e-8` |
| M7a | P1 | `delay_chunk`: the damping states stay in locals and are never written back | `partition_invariance_over_1_7_64_128_512` | the left output differs from the reference at block size 1 |
| M7b | P1 | `chunk_frames`: the `transition_remaining` bound is dropped | `partition_invariance_over_1_7_64_128_512` | `transition_remaining -= advanced` underflows: a crossfade ran past its end |
| M7c | P1 | `DelayLane::history_bound` always returns `usize::MAX`, dropping the `D - valid_history` bound | `partition_invariance_over_1_7_64_128_512` | the left output differs from the reference at block size 1, where the 13 ms tap turns valid |
| M8 | E7 | `recover_lane` no longer scans the ring cells the block wrote | `nonfinite_state_recovers_per_block_lane_locally_at_p_zero` | the injected infinity escapes for a block: `nonfinite_left_blocks` is 0 |
| M9 | E8 | `restore_state_payload` commits the left lane before validating the right | `invalid_restore_is_atomic_and_both_resets_are_word_exact` | a rejected restore leaves a mixture of the old and the new state |
| M10 | E9 | the D7 `flush` is removed from the ring write | `dry_identities_warm_histories_with_canonical_zero_state` | the ring keeps `1e-30` and `-0.0` instead of canonical `+0.0` |
| M11 | specs | `PARAMETER_SPECS[1]` is derived from the delay-time descriptor | `descriptor_and_specs_agree` | spec minimum 1.0 against descriptor minimum -0.95 |
| M12 | automation | `apply_automation` accepts out-of-order spans | `malformed_automation_is_counted_and_never_applied` | 4 invalid spans counted where 5 are malformed |
| M13 | G5 | `mix_sample`: the wet mix is a multiply and an add instead of one `Lane::fma` | `corpus_digests_match_their_pins` | the `dual_mono` digest moves |
| M14 | G5 vacuity | `corpus::run_case` reports zeros instead of what it rendered | `corpus_cases_are_finite_distinct_and_alive` | `case dual_mono is silent` -- the guard that stops a vacuous digest passing |
| M15 | E10 | `bind_homogeneous_bank` answers `Ok(None)` before validating its members | `bank_fallback_validates_every_member` | a malformed member is accepted instead of rejected |
| M16 | E3 oracle | the oracle's `damping_coefficient` returns `G` instead of `G / (1 + G)` | `damped_matrix_tail_matches_reference_oracle` | worst deviation 5.2e-2 -- the oracle is independent of the engine's mapping |

## Two things the table is careful about

**Every row was seen red, and three of them only after the test was made stronger.** The first pass
left `M7c`, `M9` and `M10` alive, and each survivor was a real gap rather than an equivalent mutant:

* `M7c` survived because the partition corpus started from the 250 ms default tap, whose history
  never becomes valid inside the run -- the automation moves the tap to a few milliseconds long
  before sample 12_000. The corpus now starts at 13 ms, so the `D - valid_history` bound is load
  bearing at sample 624.
* `M9` survived because the payload being restored was the effect's *own* current state, so
  committing the left lane early was a write of the bytes that were already there. The test now
  advances the effect past the snapshot it restores, which is what makes a partial commit visible.
* `M10` survived because `-0.0 + +0.0` is `+0.0` in IEEE arithmetic: the addition, not the flush,
  was canonicalising the ring word in the case the test used. The test now writes a value inside
  the flush band (`1e-30`, which is *normal* -- the D7 band is far wider than the subnormals) and a
  sum of two negative zeros, both of which only the flush can canonicalise.

**Two mutations fail by panicking rather than by asserting.** `M7b` underflows
`transition_remaining` and `M15` reaches the test's own `panic!`. Both are unambiguous reds, and
both are recorded as what they are rather than dressed up as assertion failures.
