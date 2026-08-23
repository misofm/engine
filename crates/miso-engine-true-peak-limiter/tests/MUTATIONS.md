# Red mutations for `miso-engine-true-peak-limiter` (issue #90, wave 2)

Every gate in this crate landed with the mutation below applied once and observed red. Each row
states the edit, the test that failed, and — where it matters — why that test and not the obvious
one. Restore the file after each run; the mutations are not committed.

Run one with, e.g.:

```
cargo test -p miso-engine-true-peak-limiter --lib phase_outputs
```

| # | mutation | file | test that turned red |
|---|---|---|---|
| 1 | `fir.iter().zip(history.iter())` → `fir.iter().rev().zip(history.iter().rev())` (FIR summed in decreasing tap order) | `src/lib.rs` `annex2_phases` | `phase_outputs_match_the_frozen_scalar_order` |
| 2 | tap 1 phase 0 of `ANNEX2_FIR` moved by one decimal digit (`0.010_986_328` → `0.010_986_329`) | `src/lib.rs` | `bs1770_annex2_conformance_is_unchanged` |
| 3 | main ring `N + 6` → `N + 7` | `src/lib.rs` `Shape::new` | `fixed_latency_guarded_ceiling_and_bypass_bits_hold` |
| 4 | `MINIMUM_RAMP_WINDOW` 32 → 1 | `src/lib.rs` | `production_tracks_the_f64_oracle` (E5) |
| 5 | `smoothed = box_sum / window` → `smoothed = quantised` (box smoother removed, bare minimum applied) | `src/lib.rs` `channel_frame` | `the_gain_ramp_falls_gradually_and_arrives_at_the_requirement` |
| 6 | `flush(target.max(released))` → `target.max(released)` | `src/lib.rs` `channel_frame` | `silence_restores_exact_identity_including_signed_zero` |
| 7 | van Herk suffix pass guarded by `complete && width < 2`, so it never runs at W4/W8 | `src/lib.rs` `sliding_minimum` | `lane_identity_holds_across_widths` |
| 8 | `left.prefix.fill(1.0)` / `right.prefix.fill(1.0)` at block entry (min-filter prefix not carried across blocks) | `src/lib.rs` `limiter_block` | `partition_invariance_holds_over_block_sizes` |
| 9 | `return;` inserted after `limiter_block` so the §4.4 boundary check never runs | `src/lib.rs` `LimiterCore::process_block` | `a_nonfinite_block_is_zeroed_reset_and_counted` |
| 10 | the `recomputed == box_sum` check in restore disabled | `src/lib.rs` `read_lane` | `state_v2_round_trips_and_rejects_corruption` |
| 11 | `peak = history[6].abs()` → `peak = L::zero()` (sample term dropped from `P`) | `src/lib.rs` `detector_peak` | `fixed_latency_guarded_ceiling_and_bypass_bits_hold` |
| 12 | `box_sum.div(window)` → `box_sum.mul(1.0.div(window))` (reciprocal instead of divide) | `src/lib.rs` `channel_frame` | `silence_restores_exact_identity_including_signed_zero` |
| 13 | `current = select(remaining > 0, current + step, target)` → `current = current + step` (D11 snap removed) | `src/lib.rs` `RampLanes::advance` | `the_lane_ramp_reproduces_the_scalar_ramp_bit_for_bit` |
| 14 | `write_u32(bytes, MAIN_CURSOR, cursors.main.swap_bytes())` (payload endianness) | `src/lib.rs` `snapshot_lane` | `state_v2_round_trips_and_rejects_corruption` |
| 15 | `let leak: Vec<f32> = vec![0.0; 4];` inside the van Herk suffix pass | `src/lib.rs` `sliding_minimum` | `the_render_path_allocates_nothing` |
| 16 | `crates/miso-engine-true-peak-limiter/src/lib.rs` re-added to the `check-math-policy.sh` allowlist while `limit_coefficient` calls `10.0_f32.powf(..)` | `src/lib.rs` | `scripts/check-math-policy.sh` (allowlist entry with zero call sites fails) |

## Mutations that survived their first target, and what was done about it

Recorded rather than quietly re-aimed, because each one says something about what a gate does and
does not prove.

* **`MINIMUM_RAMP_WINDOW` 32 → 1** does **not** turn the ceiling gate (E4) red. It moves the worst
  true-peak margin over the whole E4 matrix from **−0.961 dB to −0.398 dB** — it eats 0.56 dB of
  the 1 dB internal guard without breaching the user ceiling on these corpora. The floor is
  therefore gated by E5 (the `f64` oracle carries the same `W_MIN` and disagrees immediately) and
  by E6 (the ramp shape), and the 0.56 dB is reported to #49 as the measured cost of removing it.
  The plan's suggested E4 mutation assumed the floor was load-bearing for the ceiling itself; on
  the corpora that exist it is load-bearing for the *headroom*.
* **`flush(d)` removed** does not change any *output* sample: the release decays past
  `FLUSH_EPS` into the far-subnormal range, and `1 - 1e-30` still rounds to exactly `1.0`, so the
  identity holds anyway. What it changes is the recursive word itself, which is where FTZ makes
  two targets disagree. E7 was strengthened to assert the reduction word in the state payload is
  exactly `+0.0` bits; the mutation is red against that.
* **Reciprocal instead of divide** survives at most window lengths because `Wb * (1 / Wb)` happens
  to round back to exactly `1.0`. It does not at `Wb = 97`, which is a 2 ms lookahead at 48 kHz, so
  E7 sweeps lookaheads `{0, 2, 5, 10} ms` and the mutation is red at the 2 ms one.
* **Min-filter prefix always `1.0`** (rather than not carried across blocks) is *partition
  invariant* — it is consistently wrong — so it is not a P1 mutation at all. The P1 mutation is
  clearing the prefix at block entry, which is what row 8 does.
* **D11 snap removed** survives the cross-target corpus, because no corpus case retargets a ramp:
  every case runs with `remaining == 0` throughout, so `RampLanes::advance` is a no-op there. The
  lane ramp got its own dedicated gate (E13, row 13) instead of widening the corpus.
