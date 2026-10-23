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
