# Red-mutation log — issue #85 (`miso-engine-builtins`)

Every gate this job added landed with the one-line mutation that makes it fail. Each mutation below
was applied to the committed code, run, and reverted; the failing test and the first line of its
output are recorded. Two mutations turned out to be **equivalent** — they change no observable
behaviour — and are recorded as such rather than quietly dropped.

Command form: `cargo test -p miso-engine-builtins --test <file> <name> -- --exact`.

| # | mutation | file | gate it must break | observed |
| --- | --- | --- | --- | --- |
| M1 | `let denominator = 1.0 + t1;` → `1.0 - t1` | `builtins/src/lib.rs` (`SvfSection::design`) | T1 `prepared_sections_match_reference_coefficients` | FAILED, `rate=44100, cutoff=10, high_pass=true` |
| M2 | drop the D7 flush: `self.s1 = flush(n1)` → `self.s1 = n1` | `dsp-reference/src/tpt.rs` | T2 `scalar_stage_is_bit_identical_to_reference_recurrence` | FAILED, `rate=44100, signal=1, index=3670` |
| M3a | every bank lane takes lane 0's coefficients (`zip(sections)` → `sections[0]`) | `builtins/src/lib.rs` (`svf_coef`) | T3 `bank_is_bit_identical_to_scalar_stage_at_every_width` | FAILED, `width=4, members=3, lane=1, frame=0` |
| M3c | report counters sum every lane, not only the members (`take(self.members)` → `take(L::WIDTH)`) | `builtins/src/lib.rs` (`InputStage::members_sum`) | T3, padding-lane arm | FAILED, `width=4, members=1` |
| M4 | high-pass mix `m1 = -k` → `-k * 1.001` | `builtins/src/lib.rs` (`SvfSection::design`) | T1, T2 **and** the determinism corpus | all three FAILED |
| M5 | the fixture oracle drops the identity-section `-0.0` mapping | `tools/miso-engine-builtins-fixture/src/main.rs` | `issue064_checked_corpus_is_read_only_complete_and_has_no_authoring_reachability` | FAILED at the PCM semantics check |
| M6 | the boundary check zeroes the whole block instead of the masked lanes | `lane/src/kernels/builtins.rs` (`zero_lanes_block`) | T6 `boundary_check_is_lane_local_per_block` | FAILED, `width=4, lane=0, frame=0` |
| M7 | the matrix ramp segment is one frame short (`.min(frames)` → `.min(frames).saturating_sub(1)`) | `builtins/src/lib.rs` (`MatrixStage::process`) | T7 `partition_invariance_over_master_plan_quanta` | FAILED, `quantum=1, frame=0` |
| M8 | the ramp recomputes its step per sample (D11 → the pre-#83 law) | `lane/src/kernels/builtins.rs` (`matrix2x2_ramp_block`) | T8 `matrix_ramp_matches_reference_d11_law` | FAILED, `samples=2, frame=0, ll` |
| M9 | mute multiplies by zero instead of clearing with `andnot` | `lane/src/kernels/builtins.rs` (`gain_mute_block`) | T9 `signed_zero_and_mute_laws` | FAILED (`-1.0` muted becomes `-0.0`) |
| M10 | the meter segment ignores the window boundary (`period - self.frames` → `period`) | `builtins/src/lib.rs` (`MeterAccumulator::observe`) | T10 `meter_segment_law_is_exact` | FAILED, `quanta=[7]` |
| M11 | the D7 threshold `1e30` → `1e31` | `lane/src/kernels/builtins.rs` (`NONFINITE_LIMIT`) | determinism corpus, `input_stage/nonfinite` | FAILED, digest moved |

## What M4 says about the response gates

`m1 = -k * 1.001` is a 0.1 % error in the high-pass output mix. It is **invisible** to every
response gate in `tests/response.rs`, whose tolerances are 0.005 dB on the cast state-space
transfer and 0.05 dB on impulse and sustained measurements: at the cutoffs those gates probe, the
perturbation lands below both. It is caught immediately by the three gates that compare *bits* —
T1 against the reference design, T2 against the reference recurrence, and the cross-target digest.
That is master plan §1 in one measurement: fixtures confirm, they do not define.

The response gates were still strengthened while chasing this: `state_space()` now builds the
model from **all seven** prepared words rather than re-deriving the mix from `k`, so a mix error is
at least *visible* to them, and `cast_tpt_state_space_matches_independent_rbj_transfer_at_compatibility_rates`
uses it.

## Equivalent mutants, kept honest

**E1 — dropping the `active` mask from the boundary check.**
`let bad = L::mask_and(nonfinite_lanes_block(io, frames), self.active)` → without the mask, T3
still passes, including with a padding lane whose retained state is NaN. The mask is inert given
two other properties that *are* gated: `zero_lanes_block` clears only the lanes the mask selects,
and `members_sum` counts only lanes below `members` (M3c and M6 prove both). It is kept because it
is the statement of the contract #86 consumes — a padding lane is excluded from the boundary check
— and because either of the two properties it leans on could be changed by a later job without
anything noticing. One `mask_and` per block per channel is the price.

**E2 — running the ramp over the whole block (`ramp_frames = frames`).**
Per lane the ramp holds its target once its countdown reaches zero, so extending the segment past
the last ramping frame changes no lane's coefficients. The only observable difference would be a
lane that settles *at the identity matrix* mid-block, which then misses the settled path's identity
select and its `-0.0` preservation. No corpus case does that today. The off-by-one form (M7) is the
mutation that binds, and it is red.
