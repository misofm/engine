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
| 1 | D11 undone: `ParameterSmoother::next_value`'s `Linear` arm goes back to the audited `self.current + (self.target - self.current) / self.remaining as f32` | `crates/effect-contract/src/lib.rs` | `cargo test -p effect-runtime --test contract_ramp_identity` | RED (`linear_smoother_is_bit_identical_to_the_linear_ramp`) |
| 2 | the heterogeneous-cohort divergence returns: the limiter's program-key mismatch raises `Err("effect.bank.program")` again instead of setting `same_program = false` | `crates/true-peak-limiter/src/lib.rs` | `cargo test -p true-peak-limiter --lib bank_binding` | RED (`bank_binding_validates_before_fallback_and_retains_exact_width_bytes`) |
| 3 | the shape divergence returns: the EQ's `request.validate_shape()?` is replaced by the old combined `if !has_matching_backend_width() \|\| len != lanes \|\| lanes != current { return Ok(None) }` | `crates/parametric-eq/src/lib.rs` | `cargo test -p parametric-eq --test bank` | RED (`bank_binding_rejects_malformed_shapes_and_declines_a_foreign_width`) |
| 4 | a platform transcendental comes back to the contract, whose `#[expect(clippy::disallowed_methods)]` row #95 deleted | `crates/effect-contract/src/lib.rs` | `cargo clippy -p effect-contract` (formerly `bash scripts/test-math-policy.sh .`) | RED (`use of a disallowed method`) |
| 5 | the contract gains `lane` as a dependency | fixture manifest | `bash scripts/test-effect-runtime-policy.sh .` | RED (`contract-gains-lane`) |
| 6 | the contract loses its `math` dependency | fixture manifest | `bash scripts/test-effect-runtime-policy.sh .` | RED (`contract-loses-math`) |
| 7 | the deleted orphan header `include/miso_engine_effect_contract_v1.h` is recreated | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (orphan contract header) |
| 8 | a `#[repr(C)]` record is added to the contract crate | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (contract has no C ABI) |
| 9 | E4: a private `fn normalize_zero` copy is appended to `delay` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`normalize_zero-copy-in-an-effect`) |
| 10 | E4: a per-value `fn sanitize(v, counter)` is appended to `delay` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`sanitize-comes-back`) |
| 11 | E4: a private `struct Ramp` is appended to `compressor` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`private-ramp-struct-comes-back`) |
| 12 | E4: a second `struct LinearRamp` is added to `effect-runtime::ramp` | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (`second-linear-ramp`) |
| 13 | E4 ratchets **down** too: `advance_ramps` is renamed away without updating its manifest row | fixture workspace | `bash scripts/test-effect-runtime-policy.sh .` | RED (stale manifest row) |
| 14 | E6: the EQ declares `LatencySamples(1)` while its kernel stays at zero delay | `crates/parametric-eq/src/lib.rs` | `cargo test -p parametric-eq --test conformance` | RED (`latency.frame_boundaries`, `latency.impulse`, `latency.repetition`) |
| 15 | E6: the compressor declares `LatencySamples(latency - 1)` while its ring stays the same length | `crates/compressor/src/lib.rs` | `cargo test -p compressor --test conformance` | RED (`state.continuation`, `state.deterministic`) |
| 16 | E6, the harness fix is load-bearing: the impulse probe goes back to rendering a single block (`blocks_for_latency = 1`) | `crates/conformance/src/effect.rs` | `cargo test -p compressor --test conformance` | RED (`latency.impulse`) |
| 17 | E6, the harness fix is load-bearing: the lane-isolation control instance stops rendering, so the comparison is against the initial state again | `crates/conformance/src/effect.rs` | `cargo test -p compressor --test conformance` | RED (`state.lane_isolation`) |

Rows 5-13 are mutation *tests*: `scripts/test-effect-runtime-policy.sh` applies each mutation to
a scratch copy of the workspace, asserts the policy script rejects it, and restores. Row 4 is
now `cargo clippy`'s own `disallowed-methods` lint (formerly `scripts/test-math-policy.sh`,
retired once that migration was mutation-proven). They run in CI, so these rows are re-proven
on every commit rather than only on the day they were written.

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
