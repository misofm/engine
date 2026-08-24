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
