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

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b2`)

**What landed.**

- `rack`: `BankStage::begin_block(first_sample, snapshot)`; `BankChain::{run, run_with_resident_input}`
  take the block's `LiveSnapshot` and hand it to every active slot's drain. New `BankStage::live_lanes()`
  (default 0; `LiveControlEffectBankStage` reports the lanes holding a channel) and a debug assertion in
  `BankChain::new` that a slot with a live lane has an active lane (D3). The slot guard stays.
- `graph`: `GraphPreparedBuiltinBankProcessor::begin_block(first_sample, snapshot)` (forwarded by
  `BuiltinStage`); `GraphBindingBlock::live_snapshot`; `GraphExecutor` keeps `Option<LiveSnapshot>`, set by
  `begin_live_block` and **taken** by `render`, so a block rendered without its own snapshot is refused with
  `InvalidEnvelope` before any drain and one block's `S` never serves the next (no default snapshot, D2).
  `Runtime::{execute, complete_pending}`, `execute_op` carry `S` to the route op's `LiveRoute::drain`, to the
  per-node `Bound`/split-pair `GraphBindingBlock`, to `LiveControlEffect::stage` (a wrapper that calls
  `EffectControlLane::stage`) and to the bank chain.
- `builtins-compiler`: the input bank's `begin_block` signature; the three `GraphBindingBlock` destructures
  name `live_snapshot: _` (fader and matrix queues, #1312 moves them); test construction sites pass
  `LiveSnapshot::ALL`.
- Test-support hooks `test_only_route_drain_calls` and `test_only_route_drain_snapshots`, counted **inside**
  `LiveRoute::drain` (a first draft counted at the call site and could not see a skipped drain; mutation S2).
- Callers moved to the new signatures with `LiveSnapshot::ALL` (direct, quiescent calls): rack unit tests,
  `rack/tests/{live_control_bank,mono_reengage}.rs`, graph `runtime.rs` tests, `rt9`,
  `console-workload/tests/paired_spans.rs`, and two files the spec's list does not name but the signature
  reaches: `builtins-compiler/tests/allocation_tracker.rs`, `graph-compiler/tests/bypass_shunt_identity.rs`.
  `scale.rs`, `rt1` and `rt10` implement the trait without overriding `begin_block`, so they did not change.

**Tests.** `crates/graph/tests/live_snapshot_drains.rs` (gates 1-2, both bank widths); rack
`run_hands_the_blocks_snapshot_to_every_active_slot_in_every_block`,
`a_slot_with_a_live_lane_must_have_an_active_lane`, `a_live_control_stage_reports_the_lanes_that_hold_a_channel`;
graph `a_block_without_its_live_snapshot_is_refused_and_a_snapshot_serves_one_block`.

**Gates (all on the final tree).**

| Gate | Result |
|---|---|
| `cargo test --locked -p graph --features test-support`, `-p rack`, `-p builtins-compiler --features test-support`, `-p graph-compiler`, `-p host-core` | pass |
| `cargo test --locked --workspace --no-fail-fast` (before the route-hook move and the last graph test) | exit 0, 341 result lines ok |
| `cargo test --locked -p console-workload` | pass |
| `cargo build --locked --release -p audit -p capi && ./target/release/audit capi` | allocations, deallocations, locks, syscalls 0; `total_violations` 0 |
| `check-realtime-policy.sh` | ok (97 regions in 27 files) |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh`) | pass; call-graph gate needed no re-pin; graph rows unchanged (graphSessionPlusPlanBytes 29842 of 35648) |
| `check-cross-targets.sh`, `check-workspace-policy.sh`, `cargo fmt --all -- --check`, clippy `--workspace --all-targets --all-features -D warnings` | pass |

**Mutation evidence.** Each row applies one defect to a committed file and reverts it.

| Defect | Red |
|---|---|
| `BankChain` hands `LiveSnapshot::ALL` to a stage | rack `run_hands_...`; graph `every_drain_kind_receives_the_blocks_snapshot` |
| `BuiltinStage` forwards `ALL` | `every_drain_kind_receives_the_blocks_snapshot` |
| `GraphBindingBlock` of a `Bound` node built with `ALL` | the same |
| route op drains with `ALL` | the same |
| executor hands `ALL` to `execute` | the same |
| `render` reads the stored snapshot without taking it | `a_block_without_its_live_snapshot_is_refused_...` |
| S1: chain skips every drain when the gathered inputs are silent | `every_live_drain_runs_once_in_every_block_silent_or_not` |
| S2: route skips its drain when its input is silent | the same |
| S2b: route skips its drain when inactive | the same |
| S3: per-node processor skipped when its input is silent | the same |
| debug assertion in `BankChain::new` removed | rack `a_slot_with_a_live_lane_must_have_an_active_lane` |
| `LiveControlEffectBankStage::live_lanes` reports 0 | `a_live_control_stage_reports_the_lanes_that_hold_a_channel` |

**Test value, one line each.** Gate 1: a drain kind handed a fixed or another block's snapshot. Gate 2: a
silence or activity skip that also skips a drain. Rack chain test: the chain's hand-over to a bare stage, and
that a stage with no active lane is not drained. Assertion tests: a live lane bound into a slot marked
inactive, and a stage that stops reporting its live lanes. Executor test: a block that reuses a stale or
default snapshot.

**Deviations and open items.**

1. **Unprobed lanes.** The per-node live effect (`LiveControlEffect::stage`), the effect bank stage
   (`LiveControlEffectBankStage::drain`) and the input and fader/matrix builtin banks ignore `S` (queue lanes),
   so no test can observe `S` there; the signature is type-checked, and `begin_block`'s hand-over to a builtin
   bank is probed. #1345 and #1312 add the probes when those lanes read cells.
2. **`EffectControlLane::stage` keeps its signature.** It lives in `effect-contract`, outside the authorized
   paths; `S` stops at the graph-side `LiveControlEffect::stage` wrapper and at `LiveControlEffectBankStage::drain`.
   #1345 changes `stage` itself.
3. **The preparation assertion covers effect banks only.** A builtin bank reports no live lanes
   (`GraphPreparedBuiltinBankProcessor` has no `live_lanes`; the input bank is in `builtins-compiler`, whose
   authorization is signatures and block sites only). In graph plans a slot carries the chain's own lane mask,
   so a builtin slot cannot be wholly inactive in a chain that runs.
4. **D4 and the resource row.** `GraphExecutor` grows by one `Option<LiveSnapshot>` (`size_of` 392 to 408 bytes
   on x86-64; the metadata mirror struct carries the field too, so the field delta is unchanged). No browser
   row moved and every ceiling test passed, but the native executor allocation is 16 bytes larger; D1's
   "the executor stores `S`" cannot be met with no field.
5. **"One lane of the probe bank is bypassed" (gate 2)** is a partial bank: `LANES` (3) of the bank's lanes
   are populated and the rest inactive. A bank with a live-bypassed lane drains the same way and has no
   probe seam without an effect.
6. **A direct executor render** (graph `render_host` helper) now names `LiveSnapshot::ALL` through
   `begin_live_block`; the only direct callers are those tests.
