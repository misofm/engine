# Remove the fused fader and matrix stage that no product runs

Owner decision (2026-10-23): "Yes, remove it." Follows #1754's diagnosis and measurement.

## Problem

The fused fader+matrix bank stage (#466; its settled pass #881 and its ramping pass #890) forms only when both banks use `BuiltinControlDelivery::BetweenRenderCalls` (`make_fader_matrix`, `crates/builtins-compiler/src/lib.rs` near 5505-5525). No product host prepares that delivery (#1754 D1: the C ABI passes `between_render_calls = false`, `crates/host-core/src/prepare.rs` near 1666; the browser calls no `_between_render_calls` entry since #1400b). #1754 measured that pairing it under the product delivery would save at most 1.81 % of the fader-ride row (below the 2 % bar), and that it would add a hazard: a post-fader send splits a fused bank, and a swap does not carry strip state between fused and split banks, so one send edit would restart every strip of that bank. The stage is the engine's only second builtin bank kind and the only both-or-neither strip carry.

## Smallest closable slice

1. **Inventory first (record in this spec before any deletion).** List every item that exists only for the fused stage or only for the `BetweenRenderCalls` delivery: kernels (`fader_matrix_ramp_block`), stage methods (`MatrixStage::fused_ramp_block`, `try_process_ramping_with_matrix(_and_operations)`, the settled fused pass), processors (`FaderMatrixBankProcessor`), the fused `adopt_strip_lane`, `BuiltinStripStage::FaderMatrix` and the both-or-neither carry in `resolve_strip_carry`, the `chains_into` merge, prepare entries named `_between_render_calls`, the bench's metered row preparation, test-only counters (`FADER_MATRIX_RAMP_FUSED_CALLS`), and their tests. For each: delete, or keep with the reason (for example a `BetweenRenderCalls` use that is not the fused stage).
2. **Delete** every item the inventory marks, with its tests, in one change. Keep the separate fader and matrix stages exactly as they are: no rendered bit moves on any product plan.
3. Correct docs and comments that describe the fused stage as live (for example the metered row's doc in `tools/bench/src/console.rs` and `tools/console-workload/src/lib.rs`).

## Authorized paths

- `crates/lane/src/kernels/`, `crates/builtins/`, `crates/builtins-compiler/`, `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs`, `crates/graph-compiler/` (only the fused-stage and BetweenRenderCalls items)
- `crates/host-core/src/prepare.rs`, `crates/host-core/src/lib.rs` and host-core tests (the fused carry and `_between_render_calls` entries)
- `tools/console-workload/src/lib.rs`, `tools/bench/src/console.rs` and the console-benchmark scripts (the metered row's preparation; a removed row moves the record count)
- `hosts/host-web/src/tests.rs` (only tests of a removed entry)
- this spec

## Objective gates

1. The inventory, committed before any deletion.
2. Class A: every product plan renders bit-identically to main. One-time base comparison as PR evidence: the console benchmark's digests (the product-lane row, the fader-ride arms and every standing row) and host-core's control parity tests.
3. `cargo fmt --all --check`, the workspace clippy line, `tools/realtime-policy`, `scripts/check-web-audioworklet.sh`, `cargo check --target aarch64-unknown-linux-gnu --tests` on touched crates, and `RUSTDOCFLAGS='-D warnings' cargo doc --no-deps` on touched crates.
4. Net line count: report lines deleted and added.

## Test value

No new test. Every deleted test defended only the removed stage.

## Dependencies

- Supersedes #1754 (its diagnosis stays in history), #955 and #961. Close them with this issue.
- #1765 then covers only the separate fader and matrix stages.

## Evidence and decision record

### Step 1 inventory (attempt 1, 2026-10-23, at a9188e347; code identical to main be2044c42)

**What `BetweenRenderCalls` does besides the fused stage: nothing that a product runs.** `control_delivery` is read in four places only: the bank pairing `make_fader_matrix`, the test-only scalar oracle's pair factories (`LiveControlFaderProcessor::scalar_pair_factory` / `scalar_split_pair_factory` and the two `make_scalar_*pair` checks), the scalar resource estimate's pair terms (`outer_total`, `split_runtime` in `graph_scalar_owner_resource`), and `GraphPreparedBuiltinBankInfo::control_delivery`, which only tests read. No reader, drain or kernel depends on it (#1754 Q1). So the delivery itself goes, with the enum. The scalar oracle's serialized pair (#1059) is the fused stage's scalar twin: it forms only under `BetweenRenderCalls`, so it goes too.

Delete:

- `crates/lane/src/kernels/builtins.rs`: `fader_matrix_ramp_block`, `FaderMatrixArm`, `MatrixArmFrame` and its arm frames, `fader_matrix_ramp_frames` (#890). `crates/lane/tests/fader_matrix_ramp.rs` (whole file).
- `crates/builtins/src/lib.rs`: `MatrixStage::fused_ramp_block`; `BuiltinFaderBank::try_process_settled_with_matrix(_and_operations)` and `try_process_ramping_with_matrix(_and_operations)`; `FaderMuteRampBuiltins::process_fader_matrix` (used only by the scalar pair). Tests: `matrix.rs` `fused_fader_matrix_shapes_match_the_separate_stages`, `fused_ramping_fader_matrix_matches_the_separate_stages`, `scalar_fused_fader_matrix_matches_the_separate_stages`; `fader_ramp.rs` `the_fused_settled_path_declines_only_an_operation_inside_the_block`.
- `crates/graph/src/lib.rs`: `BuiltinStripStage::FaderMatrix`; `BuiltinControlDelivery`; `BuiltinPairFactory` and `GraphPreparedBuiltinBankProcessor::pair_factory` / `control_delivery`; `GraphPreparedBuiltinBankInfo::control_delivery`; `ScalarPairFactory`, `ScalarSplitPairFactory`, `GraphRuntimeSplitPairProcessor`, the `scalar_*pair_factory` hooks; `GraphScalarSplitRuntimeResourceEstimate`, the `split_pair_table_bytes` term and `scalar_split_runtime_owner_layout`, with their tests.
- `crates/graph/src/runtime.rs`: the builtin pair branch in the bank-chain build (`pair_factory()` at the `Membership::Builtin` pair); the scalar pairing passes and admission (`select_scalar_pairs`, `scalar_pair_is_in_place`, `scalar_split_interval_is_clear`); `SplitPairSlot`, `SplitPairRole`, `RuntimeOp::split_pair`, the `split_pairs` table, its never-taken dispatch in `execute_op`, `complete_pending` walk, `scalar_split_op_layout` / `scalar_split_runtime_layout` and the `*WithoutSplitPair*` layout twins; the split-pair witnesses; their tests. The slot and table stayed in every build only because every plan's estimate charged them (#1059). With no factory left, they are dead, so every plan's runtime metadata estimate falls by those terms. No rendered bit depends on them.
- `crates/builtins-compiler/src/lib.rs`: `make_fader_matrix`, `FaderMatrixBankProcessor` and its impls (drain, fused `adopt_strip_lane`, `carried_strip_lanes`, settled and ramping fused process), `TestOnlyFaderMatrixPair`; the fused-only witness counters (`FADER_MATRIX_FUSED_CALLS`, `FADER_MATRIX_RAMP_FUSED_CALLS`, `FADER_MATRIX_FALLBACK_CALLS`, `FADER_MATRIX_PROCESS_CALLS`, `FADER_MATRIX_FACTORY_CALLS` and the matching `TestOnlyFaderMatrixWitness` fields); `ScalarPairProcessor`, `ScalarSplitPairProcessor`, `make_scalar_pair`, `make_scalar_split_pair`, their `test_only_*scalar*pair*` helpers and `SCALAR_OUTER_LIFETIME`; the `control_delivery` fields of the bank and per-node owners; `prepare_session_builtins_between_render_calls`, `prepare_selected_session_builtins_between_render_calls` and the policy parameter; the estimate's pair terms. Tests: every in-crate test that exercises the pair, the factory, the fused drain or the delivery metadata, and `tests/allocation_tracker.rs` `actual_serialized_composite_render_allocates_and_frees_nothing`, `actual_queued_scalar_split_graph_allocates_and_frees_nothing`, `actual_scalar_split_table_and_failed_render_fit_the_resource_gate` and the pair halves of the others.
- `crates/host-core/src/prepare.rs` and `lib.rs`: `prepare_host_runtime_between_render_calls(_with_backend)`, `prepare_host_runtime_with_selected_meters_between_render_calls`, `test_only_prepare_host_runtime_between_render_calls_on` / `_successor_on`, the `between_render_calls` parameter; the both-or-neither branch in `resolve_strip_carry`.
- `crates/graph-compiler/src/lib.rs` tests: the `between_render_calls` arms of the meter tests (the `false` arm stays), `graph_compiler_produces_nonadjacent_scalar_split_schedule_fixture`, and the witness reads.
- Host-core tests: `live_lanes.rs` (the delivery half of `fader_and_matrix_lanes_attach_nothing_else_...`), `browser_successor.rs` `the_concurrent_selected_meter_entry_forms_no_fused_bank_...`, the `between` forms of `stored_*_automation.rs`, `successor_swap.rs`, `route_mute.rs`, the `randomized.rs` serialized arm, and `stored_fader_automation.rs` `the_scalar_oracle_renders_the_banks_fader_events`'s pair reads. A test that used a `_between_render_calls` entry only to reach selected meters moves to `prepare_host_runtime_with_selected_meters` (`prepare.rs`, `strip_meters.rs`, `hosts/host-web/src/tests.rs`): its claim does not depend on delivery.
- `tools/console-workload/src/lib.rs`: the metered row prepares through `prepare_selected_session_builtins_with_live_controls` (the product delivery), its doc and the pair test stop describing a fused stage, and the pair test's stage counts become equal (48 and 48). `tools/bench/src/console.rs`: the metered row's doc. The record count and the record schema do not change: the metered row stays, and the ride and product-lane records keep their literal `control_delivery: "concurrent"`.

Keep, with the reason:

- `fader_matrix_block`, `fader_matrix_block_without_identity` (lane) and `MatrixStage::fused_settled_block` (builtins): `BuiltinChain::process_dual_mono` uses them for its settled fader and matrix, with no bank and no delivery. `crates/lane/tests/fader_matrix.rs` defends that kernel.
- `BuiltinStripStage::{Fader, Matrix}` and the `own.stage == before.stage` carry rule: the split banks carry with them.
- `chains_into` and the bank-chain merge: they merge every bank chain (effects, input, fader, matrix). Only the builtin pair branch after the merge is the fused stage's.
- The drains `drain_fader_controls` / `drain_matrix_controls` and the witness counters the split stages also increment (`fader_values_applied`, `matrix_values_applied`, the output bits).
- `scripts/check-scalar-oracle-absent.py` (outside this spec's paths): its `FORBIDDEN` entries for the deleted scalar pair still hold (absent stays absent); its doc line on the split-pair slot becomes stale. Reported to root.

### Estimate fixture (root-approved path, regenerated with `graph_fixture -- --write`, no hand edits)

`fixtures/graph/v1/direct-route.resources.json`, the only resources fixture: `graph_metadata_bytes` 6476 -> 6316 (-160: each emitted op no longer carries the split-pair slot, and the executor no longer holds the split-pair table field); `builtin_bank_bytes` 7239 -> 7191 (-48: the fader bank no longer charges `FaderMatrixBankProcessor`'s size, and the bank owners lose their `control_delivery` field); `incremental_plan_bytes` and `session_plus_plan_bytes` 48535 -> 48327 (-208, the sum). `largest_allocation_bytes` stays 5336. These are estimate changes only. No rendered bit moved (see the class-A comparison).

### Class-A comparison (one-time PR evidence, no timed run)

An uncommitted ignored test, appended to `tools/bench/src/console.rs` and then removed, printed the output digests of every console measurement through its untimed `run_for`/`run` paths in release on AVX2: all 23 `native_session_rows()` rows (64 blocks each), both hoist rows, the meters and observation facilities, placement, automation, mono, mixing automation, the fader-ride arms (quiet and riding) and the product-lane row. Base a9188e347 (code identical to main be2044c42) and head cdd061338 (before the merge of main) gave 33 equal digest lines, for example `sixty_four_track_console` and `sixty_four_track_console_metered` `fe5bed9b...` and ride `66f87de4...` / `8e6cfd17...`. host-web `control_parity` (33 tests) and the host-core suites pass unchanged on the merged tree. The record count (74 after #1763) and the record schema did not change.

## Verifier (attempt 1, 2026-10-23)

PASS. Its own digest comparison (live fader, mute and matrix ramps in flight, a successor swap adding a post-fader send, metered plans, Scalar/Simd4/Simd8) found every digest equal. Before merge the implementer deleted the dead scalar-owner witnesses, renamed the randomized test to `..._armed_and_dual`, removed the one-variant `Form { Split }`, and retired #1288's gate 5. The capi release-only vacuity failure is pre-existing on main and is #1775.
