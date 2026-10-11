# Hand each block's live snapshot to every live drain

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Code anchors verified on `codex/d15-stream-b` at `13f335d79`.

## Product outcome

Every live drain receives the block's live snapshot `S`: the revision render read from the running
plan's gate at the start of the block (*Give each plan its own revision gate and take each block's
live snapshot from it*, #1502).

- `S` reaches every drain kind: bank stages, builtin banks, per-node processors and live routes.
- Every live drain runs once in every rendered block, including blocks whose input is silent and
  blocks in which the owner does no audio work.
- Rendered bits do not change. Queue drains ignore `S`. #1312, #1345, #1346 and #1347 then read
  cells under it.

## Context

- **The drains.**
  - **Bank stages.** `BankChain::run` (`crates/rack/src/lib.rs:2371`) calls every slot's
    `BankStage::begin_block` (`:659`) before anything else (`:2449-2457`). It skips a slot that
    has no active lane (`has_active_lanes`, `:1488`), because such a slot runs no lane.
    `LiveControlEffectBankStage::begin_block` (`:1211`) drains effect lanes.
  - **Builtin banks.** `GraphPreparedBuiltinBankProcessor::begin_block`
    (`crates/graph/src/lib.rs:1389`) is forwarded from the bank stage
    (`crates/graph/src/runtime.rs:4036`). The input bank drains there
    (`crates/builtins-compiler/src/lib.rs:527`). The fader and matrix banks drain inside
    `process` today (`drain_fader_controls`, `drain_matrix_controls`); #1312 D12 moves them to
    `begin_block`.
  - **Per-node processors.** `GraphRuntimeProcessor::process` takes `GraphBindingBlock`
    (`crates/graph/src/lib.rs:2394`), built at 33 sites: 27 in `crates/builtins-compiler`, 6 in
    `crates/graph`. A per-node live effect drains through `stage`
    (`crates/graph/src/runtime.rs:3511`).
  - **Live routes.** The route op drains first, every block, active or not (`runtime.rs:3352-3356`;
    `drain` `:896`).
- **Implementations outside the three crates.**
  - `BankStage` is implemented in `crates/rack/tests/mono_reengage.rs` and
    `tools/console-workload/tests/paired_spans.rs`. `BankChain::run` is called from
    `crates/rack/tests/{mono_reengage,live_control_bank}.rs` and
    `tools/console-workload/tests/paired_spans.rs`.
  - `GraphPreparedBuiltinBankProcessor` is implemented in `crates/graph/tests/rt1_direct_bank_alloc.rs`,
    `crates/graph/tests/rt9_resident_bank_input_alloc.rs` and
    `crates/graph-compiler/tests/scale.rs` (review m7).
- **The forward hazard (review m3; #1053 D11).**
  - With queues, a skipped drain made the watermark late.
  - Under `S`, a skipped drain makes it early: the watermark covers `S` while a cell holding a
    value `<= S` was never read.
  - The design keeps `skip work on silence`, but silence may skip processing only, never the
    drain.

## Decisions frozen for this slice

- **D1. The executor keeps `S` for the block.** The graph executor implements
  `PreparedPlanExecutor::begin_live_block` (#1502 D2) by storing `S` for the block it is about
  to render.
- **D2. `S` reaches every drain as data of the block.**
  - It is an argument of `BankStage::begin_block` and
    `GraphPreparedBuiltinBankProcessor::begin_block`, through `BankChain::run`.
  - It is a field of `GraphBindingBlock`.
  - It is an argument of the route op's drain and of the per-node live effect's staging.
  - No drain falls back to a default snapshot. A queue drain takes `S` and ignores it until its
    lane moves to cells.
- **D3. Every live drain runs every block (the invariant m3 asks for).**
  - Each live owner's drain runs exactly once in every rendered block. That includes a block whose
    input is silent, a block of an inactive route, and a block in which a bypassed or collapsed
    lane does no audio work.
  - A silence skip, now or later, skips processing only, never the drain. A clean cell costs one
    `Relaxed` load (#1432 D4).
  - The bank slot guard (`has_active_lanes`) stays, because a slot with no active lane has no live
    lane. A debug assertion at preparation holds that a slot with a live lane is active.
- **D4. No behaviour change.** No rendered bit, counter or resource row moves.

## Deliverables

1. D1-D2 in `crates/graph` and `crates/rack`, and the signatures and `GraphBindingBlock` sites in
   `crates/builtins-compiler`.
2. The out-of-crate implementations and callers above, moved to the new signatures.
3. Gates 1-2.

## Authorized paths

- `crates/graph/src/{lib,runtime}.rs`, `crates/rack/src/lib.rs` (stream A's crates, sequenced by
  the coordinator).
- `crates/builtins-compiler/src/lib.rs`: the `begin_block` signatures and the `GraphBindingBlock`
  sites only.
- `crates/rack/tests/{mono_reengage,live_control_bank}.rs`,
  `tools/console-workload/tests/paired_spans.rs`, `crates/graph/tests/rt1_direct_bank_alloc.rs`,
  `crates/graph/tests/rt9_resident_bank_input_alloc.rs`, `crates/graph-compiler/tests/scale.rs`:
  only where the new signatures reach them.
- A new test file `crates/graph/tests/live_snapshot_drains.rs` (gates 1-2).

## Non-goals

- Reading cells under `S` (#1312, #1345, #1346, #1347).
- Taking `S` (#1502).

## Hazards

- `BankChain::run`'s callers are numerous (Context). The new argument is mechanical at each call
  site. A defaulted setter that a production path could forget is refused (D2).
- The callgraph gate of the browser worklet reads function names. Keep the drain path free of a
  function named like an allocator.

## Objective gates

1. **Every drain kind receives the block's `S` (new graph test).**
   - A plan with a probe bank stage, a probe builtin bank, a probe per-node live processor and a
     live route, rendered through the exchange (`RealtimePlanOwner`).
   - Revisions are published on the plan's gate between blocks.
   - Each drain kind records, per block, the `S` it received, and each equals the `S` the
     executor received from `begin_live_block` in that block.
   - Mutation (PR evidence): any one drain kind passed a fixed snapshot turns it red.
2. **Every live drain runs every block (same file; review m3).**
   - Over 8 blocks, the probes record exactly one drain per block per live owner.
   - Some blocks have a silent source and an inactive (muted) route, and one lane of the probe
     bank is bypassed.
   - Mutation (PR evidence): skipping a drain when the owner's input is silent turns it red.
3. **No bit moves.** `cargo test --locked -p graph --features test-support`;
   `cargo test --locked -p rack`; `cargo test --locked -p builtins-compiler --features
   test-support`; `cargo test --locked -p graph-compiler`; `cargo test --locked -p console-workload`.
4. **Realtime.** `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`
   (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-web-audioworklet.sh` (its call-graph gate).
5. **Workspace.** `bash scripts/check-cross-targets.sh`; `cargo fmt --all -- --check`;
   `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a drain kind that never receives `S`, or receives another block's.
- Gate 2: a silence or activity skip that also skips a drain. Under `S` that makes the watermark
  early, and no existing test counts drains per block.

## Dependencies

- *Give each plan its own revision gate and take each block's live snapshot from it*
  (#1502): `begin_live_block`.

Dependents: #1312, #1345, #1346, #1347.
