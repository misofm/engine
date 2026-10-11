PASS

# #1504 attempt 1: verdict (adversarial verifier, 2026-10-11)

Commit 70017bf09 on `codex/d15-stream-b2` (parent ed62ed794, a root spec amendment to #1503 that I did
not judge). I exported 70017bf09 to `/tmp/claude-1002/v1504/tree` (gates) and to a second copy
`/tmp/claude-1002/v1504/mut` (mutations), touched every file, and built with external target dirs. I
did not build in the worktree or edit it. Small evidence: `/tmp/claude-1002/v1504/evidence/`.

The attempt meets D1-D4 and gates 1-5. There is no BLOCKER and no MAJOR. One MINOR finding is a gate
that is not met to the letter. The rest are NITs, mostly spec-inventory gaps for root.

## The implementer's seven points

1. **Two test files outside the authorized list.** `crates/builtins-compiler/tests/allocation_tracker.rs`
   and `crates/graph-compiler/tests/bypass_shunt_identity.rs` call `BankChain::run`. The new argument
   reaches them, so the change is mechanical and it is necessary. The spec's caller inventory (Context)
   left them out. Accepted (NIT n1). `LiveSnapshot::ALL` in a direct, quiescent test call is what the
   `GraphBindingBlock` doc and design 4.1 allow. **No production path uses `ALL`.** Every hit is in a
   `#[cfg(test)]` module (`rack/src/lib.rs` from :3332, `graph/src/runtime.rs` from :7962,
   `graph/src/lib.rs` from :3501), in the test-support-only test module of
   `builtins-compiler/src/lib.rs` (from :5230), in a `tests/` file, or in a doc comment
   (`graph/src/lib.rs:2434`). The three production `GraphBindingBlock` constructions
   (`runtime.rs:3160` `complete_pending`, `:3528` split pair, `:3556` `Bound`) all pass the block's `S`.
2. **`GraphExecutor` 392 to 408 bytes.** D1 says that the executor stores `S`. Design 4.1 says this
   too. `PreparedPlanExecutor::render` belongs to `engine`, which is outside the authorized paths, and
   #1502 D2 (PASSED) chose the `begin_live_block` hook. A design without the field was therefore not
   available in this slice. **No resource row moves.** I built the browser artifact at the parent and
   at the commit: every `check-browser-expected-resources.py` budget row is the same
   (`graphSessionPlusPlanBytes` 29842 of 35648, `bridgeMetadataBytes` 1146700, and the other rows).
   On native, `size_of::<GraphExecutor>()` feeds only `largest_allocation_bytes` as a `max`
   (`graph/src/lib.rs:531`). It is not added to a retained total, and the mirror struct keeps the
   split-pair field delta the same. No test or row moved. NIT n2 is only about the wording of D4.
3. **`S` stops at the graph-side wrappers.** D2 makes `S` an argument "of the per-node live effect's
   staging". The new `LiveControlEffect::stage(snapshot, first_sample)` (`runtime.rs:1237`) and
   `LiveControlEffectBankStage::drain(first_sample, snapshot)` (`rack/src/lib.rs:1317`) satisfy this
   inside the authorized paths. `EffectControlLane::stage` is in `effect-contract`, and #1345 changes
   it. The spec says "Queue drains ignore `S`", so the input, fader, matrix, effect-bank and per-node
   effect lanes are correct to ignore it. This is not an incomplete slice. These lanes have no
   observable `S` until they read cells, so no probe is possible now.
4. **The preparation assertion covers effect banks only.** `BuiltinStage` reports `live_lanes() == 0`,
   so the assertion is vacuous for builtin banks. But every graph slot carries the chain's own lane
   mask (`bank_chain_with_fold`, `runtime.rs:4337`), and a padded group always has lane 0 active
   (`trailing_active_mask`). So in production no slot is wholly inactive, and the guard never skips a
   builtin drain. NIT n3.
5. **"One lane bypassed" is a partial bank.** See MINOR m1.
6. **The `InvalidEnvelope` refusal for a block without its snapshot.** It cannot fire in production.
   The only production caller of `executor.render` is `PreparedRenderPlan::render_block`
   (`engine/src/realtime/plan.rs:1122-1123`), and it calls `begin_live_block` immediately before
   `render`. If it did fire, `render_inner` reports `BlockFailure { after_snapshot: true }`,
   `render_active` counts `live_block_errors` and the watermark does not advance
   (`plan_exchange.rs:755-767`). No drain ran and no queue record was consumed. So the path is a
   counted, safe error path. The error kind is a contract breach that has the name of an envelope
   error. `RenderError` is in `engine`, which is outside the paths. NIT n4.
7. **Full workspace re-run on the final tree.** `cargo test --locked --workspace --no-fail-fast`:
   exit 0, 341 `test result: ok` lines, 0 failed.

## Findings

### MINOR

- **m1. Gate 2 does not bypass a lane.** The spec says "one lane of the probe bank is bypassed". The
  test (`crates/graph/tests/live_snapshot_drains.rs:28-30`, `:432-433`) uses a partial bank: 3
  populated lanes, and the other lanes are structurally inactive. That exercises the slot guard's mask,
  not a live bypass. I checked the defect class that the clause targets. Mutation B1 (the effect bank
  stage skips the drain of a bypassed lane, `rack/src/lib.rs:1323`) turns the existing
  `un_bypassing_returns_the_current_wet_signal` red. So no plausible defect is left uncovered. But the
  gate is not met to the letter, and the attempt record (deviation 5) says this openly. Root can accept
  it, or #1345 can add a bypassed-lane probe when its lanes read cells.

### NIT

- **n1.** The spec's inventory (Context, Authorized paths) misses two callers of `BankChain::run`:
  `builtins-compiler/tests/allocation_tracker.rs` and `graph-compiler/tests/bypass_shunt_identity.rs`.
- **n2.** D4 says "no resource row moves". This is true. But the executor body grows by 16 bytes on
  x86-64, which D1 makes necessary. A future reader could take D4 to forbid this. Wording only.
- **n3.** D3's preparation assertion holds only for stages that report live lanes. Builtin banks report
  none (`GraphPreparedBuiltinBankProcessor` has no `live_lanes`). This is safe today (point 4). When
  #1312 / #1346 move the builtin lanes to cells, they should make `BuiltinStage` report its lanes.
- **n4.** A block without its snapshot is refused with `InvalidEnvelope` (`graph/src/lib.rs:3304-3306`).
  This is a contract breach, not an envelope problem. It is unreachable (point 6). A dedicated variant
  would need `engine`.
- **n5.** `live_snapshot_drains.rs:6` says "Four drain kinds are probed" and then names three. The
  fourth, the bank chain's hand-over to a bare `BankStage`, is in the rack crate's own test.
- **n6.** `graph/src/lib.rs:3272` is a doc line of 122 columns. Nearby docs wrap at 100.

## Acked-batch question

Can an ack ever precede a drop? **No.** This slice adds no queue and no ack. `S` is a `Copy` value. The
executor takes it once per block, and `take()` means that one block's `S` never serves the next. A
block refused before its drains consumes no record. A block that fails after some drains does not
advance the watermark, and the owner counts it (`live_block_errors`), so no ack covers a value that
was not applied. Silence and inactivity skip no drain (gate 2, mutations S1-S3).

## Test value (one sentence per new test)

- `every_drain_kind_receives_the_blocks_snapshot` (gate 1): a drain kind that gets a fixed or stale `S`
  (mutations M2 builtin forward, M3 `Bound` block, M4 route drain, M5 executor-to-`execute`) turns
  only this test red.
- `every_live_drain_runs_once_in_every_block_silent_or_not` (gate 2): a skip of a drain on silent input
  or on an inactive route (S2 route on silence, S2b route while inactive, S3 per-node on silence)
  turns only this test red in the graph and rack suites.
- `run_hands_the_blocks_snapshot_to_every_active_slot_in_every_block` (rack): the chain gives a slot a
  fixed snapshot (M1), or skips the drains on silent input (S1).
- `a_slot_with_a_live_lane_must_have_an_active_lane` (rack, debug): the removal of the preparation
  assertion (A1).
- `a_live_control_stage_reports_the_lanes_that_hold_a_channel` (rack): `live_lanes` returns the trait
  default 0, which makes the assertion vacuous (verified red).
- `a_block_without_its_live_snapshot_is_refused_and_a_snapshot_serves_one_block` (graph lib): `render`
  reads the stored snapshot without taking it (M6), so a block reuses the previous block's `S`.

## Mutations run again (one at a time, then reverted; logs in `evidence/mutations/`)

| Mutation | Red |
|---|---|
| M1 `BankChain` hands `ALL` to every slot | gate 1; rack `run_hands_...` |
| M2 `BuiltinStage` forwards `ALL` | gate 1 only |
| M3 `Bound` `GraphBindingBlock` built with `ALL` | gate 1 only |
| M4 route op drains with `ALL` | gate 1 only |
| M5 executor gives `ALL` to `execute` | gate 1 only |
| M6 `render` copies the stored snapshot (no `take`) | executor refusal test only |
| A1 preparation assertion removed | rack should-panic test |
| `live_lanes` returns 0 | `a_live_control_stage_reports_...` |
| S1 chain skips every drain on silent gathered input (full graph and rack suites) | gate 2; rack `run_hands_...`; 4 existing graph/rack tests |
| S2 route skips its drain on silent input (full suites) | gate 2 only |
| S2b route skips its drain while inactive (full suites) | gate 2 only |
| S3 per-node processor skipped on silent input (full suites) | gate 2 only |
| B1 effect bank skips a bypassed lane's drain (full suites) | existing `un_bypassing_returns_the_current_wet_signal` |

## Gates run (on the exported commit)

| Gate | Result |
|---|---|
| `cargo test --locked --workspace --no-fail-fast` | exit 0, 341 ok, 0 failed |
| `cargo test --locked -p graph --features test-support` | pass |
| `cargo test --locked -p rack` | pass |
| `cargo test --locked -p builtins-compiler --features test-support` | pass |
| `cargo test --locked -p graph-compiler`, `-p console-workload`, `-p host-core` | pass |
| `cargo build --locked --release -p audit -p capi && audit capi` | allocations 0, deallocations 0, locks 0, syscalls 0, total_violations 0 |
| `check-realtime-policy.sh` | ok (97 marked regions in 27 files) |
| `check-workspace-policy.sh` | pass |
| `check-cross-targets.sh` | PASS |
| `cargo fmt --all -- --check` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| Worklet chain: `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration` (call-graph gate), `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `test-web-audioworklet.sh` (private TMPDIR, left empty), `check-web-audioworklet-v8-spill.py` | all pass; budget rows identical to the parent ed62ed794 build |
