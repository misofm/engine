# Red-mutation record for the effect-contract and effect-compiler gates (issue #95)

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named command was run, the failure was
observed, and the mutation was reverted in the same session. Nothing in this file is a claim about
code that was not run.

Host: `x86_64` (Zen 5 class), workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`,
debug profile.

Reproduce one row with:

```
# apply the "mutation" edit, then
<command>
# and revert
```

| # | mutation | file | command | result |
|---|---|---|---|---|
| 1 | D11 undone: `ParameterSmoother::next_value`'s `Linear` arm goes back to the audited `self.current + (self.target - self.current) / self.remaining as f32` | `crates/miso-engine-effect-contract/src/lib.rs` | `cargo test -p miso-engine-effect-runtime --test contract_ramp_identity` | RED (`linear_smoother_is_bit_identical_to_the_linear_ramp`) |
| 2 | the heterogeneous-cohort divergence returns: the limiter's program-key mismatch raises `Err("effect.bank.program")` again instead of setting `same_program = false` | `crates/miso-engine-true-peak-limiter/src/lib.rs` | `cargo test -p miso-engine-true-peak-limiter --lib bank_binding` | RED (`bank_binding_validates_before_fallback_and_retains_exact_width_bytes`) |
| 3 | the shape divergence returns: the EQ's `request.validate_shape()?` is replaced by the old combined `if !has_matching_backend_width() \|\| len != lanes \|\| lanes != current { return Ok(None) }` | `crates/miso-engine-parametric-eq/src/lib.rs` | `cargo test -p miso-engine-parametric-eq --test bank` | RED (`bank_binding_rejects_malformed_shapes_and_declines_a_foreign_width`) |
| 4 | a platform transcendental comes back to the contract, whose allowlist row #95 deleted | fixture workspace | `bash scripts/test-math-policy.sh .` | RED (`the-cleared-contract-row-cannot-come-back`) |
| 5 | the contract gains `miso-engine-lane` as a dependency | fixture manifest | `bash scripts/test-effect-runtime-policy.sh .` | RED (`contract-gains-lane`) |
| 6 | the contract loses its `miso-engine-math` dependency | fixture manifest | `bash scripts/test-effect-runtime-policy.sh .` | RED (`contract-loses-math`) |
| 7 | the deleted orphan header `include/miso_engine_effect_contract_v1.h` is recreated | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (orphan contract header) |
| 8 | a `#[repr(C)]` record is added to the contract crate | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (contract has no C ABI) |
| 9 | E4: a private `fn normalize_zero` copy is appended to `miso-engine-delay` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`normalize_zero-copy-in-an-effect`) |
| 10 | E4: a per-value `fn sanitize(v, counter)` is appended to `miso-engine-delay` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`sanitize-comes-back`) |
| 11 | E4: a private `struct Ramp` is appended to `miso-engine-compressor` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`private-ramp-struct-comes-back`) |
| 12 | E4: a second `struct LinearRamp` is added to `effect-runtime::ramp` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`second-linear-ramp`) |
| 13 | E4 ratchets **down** too: `advance_ramps` is renamed away without updating its manifest row | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (stale manifest row) |
| 14 | E6: the EQ declares `LatencySamples(1)` while its kernel stays at zero delay | `crates/miso-engine-parametric-eq/src/lib.rs` | `cargo test -p miso-engine-parametric-eq --test conformance` | RED (`latency.frame_boundaries`, `latency.impulse`, `latency.repetition`) |
| 15 | E6: the compressor declares `LatencySamples(latency - 1)` while its ring stays the same length | `crates/miso-engine-compressor/src/lib.rs` | `cargo test -p miso-engine-compressor --test conformance` | RED (`state.continuation`, `state.deterministic`) |
| 16 | E6, the harness fix is load-bearing: the impulse probe goes back to rendering a single block (`blocks_for_latency = 1`) | `crates/miso-engine-conformance/src/effect.rs` | `cargo test -p miso-engine-compressor --test conformance` | RED (`latency.impulse`) |
| 17 | E6, the harness fix is load-bearing: the lane-isolation control instance stops rendering, so the comparison is against the initial state again | `crates/miso-engine-conformance/src/effect.rs` | `cargo test -p miso-engine-compressor --test conformance` | RED (`state.lane_isolation`) |

Rows 4-13 are mutation *tests*: `scripts/test-math-policy.sh` and
`scripts/test-effect-runtime-policy.sh` apply each mutation to a scratch copy of the workspace,
assert the policy script rejects it, and restore. They run in CI, so these rows are re-proven on
every commit rather than only on the day they were written.

## Issue #140 — the automation-span feed, the live fader, and GR observation

Every row below was applied to the working tree, the named test was run, the failure was observed,
and the mutation was reverted in the same session. Host: `x86_64`, workspace `.cargo/config.toml`
pin `-C target-feature=+avx2,+fma`, debug profile. Sweep driver: one mutation at a time,
`cargo test -p <pkg> <test>`, tree restored before the next row.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 140-1 | `EffectControlLane::stage` loses its sorted-insertion leg (`if existing > key`), so records land in arrival order | `effect-contract/src/live.rs` | `live_control::a_drain_emits_the_contract_canonical_span_order` | RED (`spans must leave the drain in (parameter_index, channel) order`) |
| 140-2 | `BypassShunt::capture` returns before the `pdc_delay_block` exchange, so the dry block is the *current* input rather than the input `latency` samples ago | `effect-contract/src/live.rs` | `live_control::the_shunt_reproduces_the_dry_signal_at_the_declared_latency` | RED (`sample 2 must be the input delayed by exactly 1`) |

## Issue #143 P2 — the observation transport and the arm/disarm seam

`tests/observation_lane.rs`; each row was applied, run, recorded and reverted in the same session.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 143-P2-a | the `Observe` arm of `EffectControlLane::stage` falls through to the span builder instead of `continue`ing | `effect-contract/src/live.rs` | `observation_lane` | RED — 2 of 9 fail: `the observations did not crowd it out` (a subscription displaced a parameter from a full staging window) and the unbound count moves |
| 143-P2-b | a closed window restarts at `first_sample = 0` instead of at `end_sample` | `effect-contract/src/live.rs` | `observation_lane::windows_tile_exactly_and_fold_by_the_declared_rule` | RED — `windows tile`, `0` vs `1408` |
| 143-P2-c | `PeakMagnitude` folds the raw signed value instead of `max(\|x\|)` | `effect-contract/src/live.rs` | `observation_lane` | RED — 3 of 9 fail; `max(\|x\|) over the window, non-negative` reports `0.0` where `9.5` was required. This is the precursor of E4's dead-meter bug |
| 143-P2-d | `accumulate` drops the `armed` guard, so an unarmed tap still folds and publishes | `effect-contract/src/live.rs` | `observation_lane::disarming_all_stops_every_tap_without_disturbing_published_windows` | RED — a disarmed tap published sequence 2 at `50.0` |
| 143-P2-e | a re-subscribe keeps the older `window_blocks` | `effect-contract/src/live.rs` | `observation_lane` | RED — 3 of 9 fail; the window closes a block early |

## Issue #143 — summary of where each eval's red mutation is recorded

| eval | recorded in |
|---|---|
| E1 digest identity per tap | `graph-compiler/tests/MUTATIONS.md` (143-E1) |
| E2 bank-lane correctness | `compressor/tests/MUTATIONS.md` (143-E2-a), `graph-compiler/tests/MUTATIONS.md` (143-E2) |
| E3 window exactness vs `applied_at_sample` | `graph-compiler/tests/MUTATIONS.md` (143-E3-bank, 143-E3-scalar), plus 143-P2-b/e here |
| E4 app-shape frame | `host-web/MUTATIONS.md` |
| E5 zero binding, zero cost | `graph-compiler/tests/MUTATIONS.md` (143-E5) |
| E6 resident means resident | `compressor/tests/MUTATIONS.md` (143-E6-a/b), `true-peak-limiter/tests/MUTATIONS.md` (143-E6-c) |
| E7 cost classes bench-backed | `graph-compiler/tests/MUTATIONS.md` (143-E7) |
| E8 flood and misuse | `host-web/MUTATIONS.md` |
| E9 metadata round-trip | `effect-package/tests/MUTATIONS.md` (143-E9-a), `host-web/MUTATIONS.md` |
| E10 wire and identity accounting | `effect-package/tests/MUTATIONS.md` (143-E10-a/b) |
| E11 transport never tears | `core/tests/MUTATIONS.md` (143-E11-a..d) |
| E12 three-browser qualification | `host-web/MUTATIONS.md` |
| E13 plan replacement | `graph-compiler/tests/MUTATIONS.md` (143-E13) |

## Issue #127 — the named nudge ladder

`tests/nudge.rs` and `effect-compiler/tests/nudge_launch_set.rs`. Each row was applied to the
working tree by `scripts`-free driver, the named command was run, the failure was observed, and the
mutation was reverted before the next row. Host: `x86_64` (Zen 5 class), workspace
`.cargo/config.toml` pin `-C target-feature=+avx2,+fma`, debug profile.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 127-1 | `NudgeRatioClassV1::Human`'s multipliers become `[1, 3, 3, 10, 30]` | `effect-contract/src/nudge.rs` | `cargo test -p miso-engine-effect-contract --test nudge` | RED — 5 of 11 fail, including `the_multiplier_table_is_a_strict_ladder` (`Human multipliers must strictly ascend`) and `a_linear_decibel_ladder_steps_by_equal_decibels` |
| 127-2 | the `Absolute` arm resolves `xs / b` instead of `xs / (b - a)` | `effect-contract/src/nudge.rs` | same | RED — 4 of 11, `an xs rung is 0.5 dB` |
| 127-3 | the `Cents` arm divides by 1200 twice | `effect-contract/src/nudge.rs` | same | RED — `one xs rung is 20 cents at 40 Hz` |
| 127-4 | the `Steps` arm divides by `choice_count` instead of `choice_count - 1` | `effect-contract/src/nudge.rs` | same | RED — `a_stepped_ladder_advances_whole_choices_and_clamps`, and the round trip stops being exact |
| 127-5 | the grid disappears: `nudge_parameter_value_v1` adds `count * step` to `x` directly instead of rounding to the nearest `xs` multiple first | `effect-contract/src/nudge.rs` | same | RED — `a_grid_nudge_is_exactly_reversible`: `+1 then -1 must restore the exact bits` |
| 127-6 | the `clamp(0.0, 1.0)` on the resolved position is dropped | `effect-contract/src/nudge.rs` | same | RED — 3 of 11, including `a_nudge_past_an_endpoint_lands_on_the_declared_endpoint_bits` |
| 127-8 | a zero `count` returns `Ok(current)` instead of `Err(NudgeErrorV1::Count)` | `effect-contract/src/nudge.rs` | same | RED — `every_refusal_is_typed` |
| 127-9 | `validate_descriptor_v1` stops calling `check_nudge_ladder_v1` | `effect-contract/src/lib.rs` | same | RED — `a_broken_ladder_stops_the_descriptor`: `the ladder is refused` |
| 127-10 | `miso-engine-compressor`'s parameter helper stops setting `nudge` | `compressor/src/lib.rs` | `cargo test -p miso-engine-effect-compiler --test nudge_launch_set` | RED — 4 of 5, `miso.compressor/threshold could declare a ladder and does not` |
| 127-11 | the dB class default moves from 0.5 to 5.0 dB | `effect-contract/src/nudge.rs` | same | RED — 4 of 5; `the_class_defaults_are_jnd_anchored` reports an xs rung of 5 where 0.5 is anchored, and the registry refuses two descriptors outright because `xl` now crosses their domains |
| 127-12 | `miso.delay`'s `delay time` override becomes 2.0 ms without a matching row in `OVERRIDES` | `delay/src/lib.rs` | same | RED — `every_declared_ladder_is_its_class_default_or_a_listed_override` |

Row 127-7 was attempted and is **not** recorded as a gate: removing the explicit
`ParameterDomain::Boolean` guard from `check_nudge_ladder_parts_v1` left every test green, because
no `(step unit, domain, mapping)` triple resolves against a boolean and the resolve already refuses
one. The redundant guard was deleted rather than kept with a mutation nothing could prove.
