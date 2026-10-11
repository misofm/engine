# Hold route-lane values in latest-target cells

Stream B of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-2).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A live route's (a send's) gain, mute and matrix target is held in one latest-target cell instead
of a bounded queue, through the shared graph lane on both hosts. Any number of send edits between
two render calls is accepted; render ramps from where the route is to the last committed target at
the next block, and #1312's counter records every replaced target. The C ABI's send edits
(#1225, #1226, #1247) are built on these cells.

## Context

- **Record.** `RouteControlRecord { target: [f32; 4], mute: bool, length: u32 }`, validated at
  construction (`crates/graph/src/lib.rs:929-966`; `RouteControlRecord::new` refuses a ramp above
  `ROUTE_RAMP_LENGTH_MAXIMUM` and a muted route with a non-zero target).
- **Ring and producer.** `attach_route_controls(depth)` (`lib.rs:1697`) creates one bounded SPSC
  per live route (`:1756`); `RouteControlLane` holds the consumer (`:969-975`),
  `GraphRouteControlProducer` the producer, with `free()` and `try_push` (`:986-1019`);
  `route_control_resources` charges the rings (`:1074-1090`).
- **Render.** The route op's `drain` (`crates/graph/src/runtime.rs:895-913`) applies, in order,
  every record available at block entry, each ramping from the coefficients the route is at.
- **Browser admission.** The route band (`queue_available`, `hosts/host-web/src/lib.rs:1783-1786`;
  `push`, `:1821-1827`). `free()` is `#[inline(always)]` for the worklet call-graph gate
  (`crates/graph/src/lib.rs:1005-1013`).
- **C ABI.** No route lane today (`crates/control-plane/src/compile.rs:19-28`); #1225 attaches
  them, written to these cells.

## Decisions frozen for this slice

- **D1. One cell per live route**, on #1312's primitive: six words (four target coefficient bits,
  mute, ramp length), one dirty bit per route. The ring, the `depth` argument of
  `attach_route_controls` and `free()` go; the cells' rows replace the ring rows in
  `route_control_resources`. The route-activity table rows are unchanged.
- **D2. Producer.** `GraphRouteControlProducer::write(record)` is infallible; validation stays in
  `RouteControlRecord::new`, before any write.
- **D3. Render.** At block entry the route op applies its cell if dirty, under the block's live
  snapshot (#1312 D12; Amendment 1): one retarget from the coefficients it is at to the cell's
  target over its ramp, exactly as one drained record does today. Its drain runs every block,
  active or not (*Hand each block's live snapshot to every live drain*, #1504 D3). Render
  applies the newest value whose revision is `<= S`, never a torn or newer one, and never skips
  it (#1312 D1). When it applies sequence `s` after `p`, it adds `s - p - 1` to
  `live_values_superseded` (#1312 D2).
- **D4. Browser admission.** The route band leaves the room pass and `in_flight`; a send edit is
  validated (record construction, follow-mute composition) for the whole batch before the first
  write.
- **D5. The acked-batch question: can an ack ever precede a drop? No.** Every check precedes the
  first write, writes cannot fail, a replaced target is in the committed model and counted, and
  the route ramps to the newest.

## Deliverables

1. Route cells in `graph` (D1-D3), its tests and resource rows.
2. Browser admission (D4) and the producer type #1225 writes to.

## Authorized paths

- `crates/graph/src/lib.rs`, `crates/graph/src/runtime.rs` (the route lane, producer, drain and
  resources only): stream A's crate, sequenced by the coordinator.
- `crates/host-core/src/prepare.rs` (the attach call only; stream A's file).
- `hosts/host-web/src/lib.rs`, `hosts/host-web/src/tests.rs` (route band; stream H's files).

## Non-goals

- The C ABI send edits (#1225, #1226, #1247). `follows_mute` in the browser (#1342).

## Hazards

- Two records in one drain used to ramp to the first target and then from wherever that ramp
  stood; now one ramp goes to the last target. Re-pin only fixtures that send two route edits in
  one block, each with that reason.
- The callgraph gate reads function names (`crates/graph/src/lib.rs:1005-1013`): keep the
  write path free of a function named like an allocator.

## Objective gates

1. **Many edits, one block (new host-web test).** 30 send-gain edits on one route, each
   admitted; one render equals a twin that sent only the last; the counter grows by 29.
2. **Ramp from where the route is (new graph test).** A ramp in flight, then a new target in the
   cell: the next block starts from the in-flight coefficients, bit-identical to today's one-record
   drain.
3. **Mute rule (keep green):** a muted target with a non-zero coefficient is refused by
   `RouteControlRecord::new` before any write; the route's existing mute tests pass.
4. **Superseded tests.** Graph or browser tests that fill a route queue and expect a full queue
   (`RouteQueueFull`, `COMMAND_REASON_BACKPRESSURE` on a send kind) are deleted with
   `RouteQueueFull`; gate 1 replaces them.
5. **Realtime and workspace.** `cargo build --locked --release -p audit -p capi &&
   target/release/audit capi` (all violation counts 0); `bash scripts/check-realtime-policy.sh`;
   `bash scripts/check-graph-policy.sh`; `bash scripts/check-web-audioworklet.sh`;
   `cargo test --locked -p graph --features test-support`; `cargo test --locked -p host-web
   --features test-support`; `cargo test --locked -p host-core --features
   control-provider,test-support`; `cargo fmt --all -- --check`; `cargo clippy --locked
   --workspace --all-targets --all-features -- -D warnings`.

## Test value

- Gate 1: a route lane left a queue, or a miscounted supersession.
- Gate 2: a cell drain that ramps from the target or from rest instead of from the route's
  current coefficients, which clicks.

## Amendment 1 (root, 2026-10-05): revision-bounded cells

Root's binding requirement (2026-10-05) makes every live value of one revision take effect in one
block (#1432 Amendment 1, #1312 Amendment 2). D3 reads the route's cell under the block's
snapshot, which #1504 hands to the route op's drain.

## Dependencies

- *Hold live values in latest-target cells on both hosts* (#1312).

Dependents: *Deliver value-only send edits to the running C ABI plan* (#1225), *Let C ABI sends
follow their source strip's mute live* (#1226) and *Deliver value-only VCA edits to the running C
ABI plan* (#1247) write these cells.

## Attempt record

### Attempt 1 (implementer, on `codex/d15-stream-b3`, one code-and-evidence commit)

**What landed.**

- `graph`: a live route's lane is one latest-target cell (`RouteLane`: a `LatestCell<6>` and a
  `DirtyWord`, one `Arc`) shared by `GraphRouteControlProducer` (control: a `CellWriter<6>`) and
  `RouteControlLane` (render: a `CellReader`, a `DirtyReader`, the session counter). The six words
  are the four target coefficient bits, the mute flag and the ramp length (D1). `RouteControlRecord`
  gains the crate-private `to_words`/`from_words` (decode runs the constructor's own check), and
  `RouteQueueFull`, `GraphRouteControlError::QueueCapacity`, `try_push`, `free` and the `depth`
  argument of `attach_route_controls` are gone.
  `attach_route_controls(&LiveValuesSuperseded)` hands every lane a clone of the session's count.
- `GraphRouteControlProducer::write(stamp, record)` is infallible (D2). The spec's `write(record)`
  takes the stamp that every other cell writer takes (#1312 D7, Amendment 1); validation stays in
  `RouteControlRecord::new`, before any write.
- Render (D3): `LiveRoute::drain(snapshot)` takes the dirty bits, reads the cell under `S` and, for
  a value not applied before, retargets once from `ramp.coefficients_at(position)` to the cell's
  target over its ramp, adds `s - p - 1` to the session count, and returns 0 or 1. It still runs in
  every block, active or not (#1504 D3). The test-only drained counter now counts values applied.
- `route_control_resources`: `queue_bytes` becomes `cell_bytes`, `routes x size_of::<
  SharedAllocation<RouteLane>>()` (144 bytes: owner counts 16, three slots of 40, dirty word 8).
  Owner, producer-table, route-ID and activity rows keep their formulas; the box and producer sizes
  follow the new types.
- `host-core`: `RouteControlProducer::{write(stamp, record), set(stamp, ..)}` replace `push`/`set`;
  `free` and `RouteControlError::Full` are gone; prepare attaches with the preparation's own
  `live_superseded` (the session's count, or the preparation's fresh one) and `control_queue_depth`
  sizes the effect queues only.
- `host-web` (D4): the send band leaves the room pass and `in_flight`; `write_strip_cells` writes the
  batch's `Route` records under the batch's one stamp, after every check, and publishes the one
  revision. A strip-mute follow record no longer joins `command_wanted`. The slot layout is
  unchanged (the band stays an unaddressable queue slot range, as the other cell bands did).
- D5, the acked-batch question: no. Every check precedes the first write, a write cannot fail, a
  replaced target is in the committed model and counted, and the route ramps to the newest.

**Choices the spec left open, and files outside the authorized list (each forced).**

1. `LiveValuesSuperseded` and the `Arc` layout struct `SharedAllocation<T>` moved from
   `crates/builtins-compiler/src/lib.rs` to `crates/engine/src/realtime/latest_cell.rs` (re-exported
   from `engine::realtime`, and `builtins_compiler::LiveValuesSuperseded` stays a re-export, so
   `host_core::LiveValuesSuperseded` and every caller are unmoved). `graph` cannot depend on
   `builtins-compiler`, and the route lane must add to the same session count the strips add to; the
   primitive's own module is the one place both crates already depend on. `add` is `pub` (it must be
   callable from `graph`); its comment states that render's cell reads are its only callers. Realtime
   regions: one region leaves `builtins-compiler` and one enters `latest_cell.rs`, so the awk gate's
   count (102 regions in 27 files) did not move.
2. `crates/builtins-compiler/src/lib.rs`: the `attach_route_live_controls` delegate takes the count.
3. `crates/host-core/src/route_controls.rs` (the host-core wrapper of the producer; not on the
   authorized list, forced by the removed `free`/`push`/`Full`), plus `test_only_write_count`.
4. Tests that used the queue: `crates/graph/tests/live_snapshot_drains.rs`,
   `crates/graph-compiler/tests/live_routes.rs`, `crates/host-core/tests/{live_routes,vca_live}.rs`.
5. `docs/BUILTINS_AND_METERING_V1.md`: the send paragraphs said "full send queue refuses the whole
   submission" and "one bounded SPSC queue"; they now state the cell contract. Left alone:
   `hosts/host-web/MUTATIONS.md` (a dated record of past mutation runs, whose rows name tests this
   slice deletes or renames) and the synthetic mangled name `GraphRouteControlProducer::free` in
   `scripts/check-web-audioworklet-callgraph.py`'s self-test data (it tests the name filter, not
   the item).
6. No fixture or digest moved: the audit resource grid, `fixtures/graph` and `fixtures/builtins` carry
   no live route, and the realtime-policy floors did not move (item 1).

**Tests moved or deleted.** Deleted with the queue: `the_drain_is_bounded_and_lossless`'s full-queue
half (graph-compiler; its last-wins, reversed-order and unmute-while-inactive halves are the new
`the_last_of_many_writes_is_applied_and_an_inactive_route_still_reads`),
`a_refused_send_record_pushes_nothing`'s full-queue arm (host-core; now
`a_refused_send_record_writes_nothing`), `a_full_send_queue_refuses_the_strip_mutes_it_follows`
(host-web; its overlong-window half survives as
`a_follow_ramp_past_the_send_bound_refuses_the_strip_mute_whole`), the overfilled-send-queue arm of
`a_refused_send_batch_pushes_nothing_and_keeps_the_mirror` (now `..._writes_nothing_...`, whose
effect-queue arm stays), `a_vca_batch_that_overfills_a_queue_is_refused_whole` (rewritten in attempt 1 as
`a_vca_batch_with_a_late_refusal_is_refused_whole`; **deleted in attempt 2**, see M1 there). Send queue-room assertions in host-web became cell write counts
(`send_write_counts`), and `the_exact_retained_budget_charges_the_send_lanes` now asserts that
the route lanes' charge and the bridge rows do not move with the queue depth.

**New tests (the plausible defect each defends).**

| Test | Defect it turns red |
|---|---|
| host-web `many_send_edits_before_one_render_apply_the_last_and_are_counted` (gate 1) | a send ring in the band again (the 5th edit refused); a miscounted supersession; an older value applied |
| host-core `many_send_edits_before_one_block_apply_the_last_and_are_counted` | the same, through `RouteControlProducer`, with random mutes and source-lane mutes |
| graph-compiler `a_new_cell_value_ramps_from_the_coefficients_the_route_is_at` (gate 2) | a cell read that ramps from the old ramp's start, the new target or rest (a click), against a scalar `f32` oracle |
| graph-compiler `a_route_cell_value_above_the_blocks_snapshot_waits_for_its_revision` | a route drain that reads `LiveSnapshot::ALL` (a value applied a block before its revision is covered); a deferred value never applied |
| graph-compiler `the_last_of_many_writes_is_applied_and_an_inactive_route_still_reads` | a read skipped while the route is inactive; the oldest value applied; the count off by one or on the wrong counter |
| host-core `a_refused_send_record_writes_nothing` | a refused `set` that writes, or a `record` that publishes by itself |

The two threaded allocation gates (graph-compiler `live_routes_render_without_allocating`, host-core
`live_sends_render_without_allocating`) keep `render_while_producing`; "queued" is now "a transaction
published past the last block's snapshot", which is what guarantees the next block applies a value.
Gate 3 (the mute rule) is `RouteControlRecord::new`'s and its tests, unchanged and green.

**Gates (final tree).**

| Gate | Result |
|---|---|
| `cargo test --locked --workspace --no-fail-fast` | all test binaries and doctests pass |
| `cargo test --locked -p graph --features test-support` | pass |
| `cargo test --locked -p host-web --features test-support` | 192 lib tests pass, plus the integration binaries |
| `cargo test --locked -p host-core --features control-provider,test-support` | pass (the four `route_controls` doctests included) |
| `cargo build --locked --release -p audit -p capi && target/release/audit capi` | allocations, deallocations, locks, syscalls 0; `total_violations` 0 |
| `check-realtime-policy.sh` (102 marked regions in 27 files), `test-realtime-policy.sh` | ok |
| `check-rack-policy.sh`, `check-workspace-policy.sh`, `check-builtins-policy.sh`, `check-graph-policy.sh`, `check-host-core-policy.sh` | ok |
| `check-builtins-fixtures.sh . target/release/audit`, `check-console-fixtures.sh`, `check-graph-determinism.sh`, `cargo test --release -p audit --bin audit` | ok (50 files; 33 passed) |
| `trace-builtins-graph-audit.sh`, `trace-graph-audit.sh` | PASS |
| `check-cross-targets.sh` | PASS |
| `cargo fmt --all -- --check`; clippy `--workspace --all-targets --all-features -D warnings` | ok |
| Worklet chain (shipped module `0ce76dd0...`): `build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`, `check-web-audioworklet-v8-spill.py`, `test-web-audioworklet.sh` | pass |

**Mutation evidence.** Each applies one defect to a committed file, runs the named tests, reverts.

| Defect | Red |
|---|---|
| `LiveRoute::drain` reads `LiveSnapshot::ALL` | graph-compiler `a_route_cell_value_above_the_blocks_snapshot_waits_for_its_revision` |
| the replaced count is not added | graph-compiler `the_last_of_many_writes_...`; host-core `many_send_edits_before_one_block_...`; host-web `many_send_edits_before_one_render_...` |
| the replaced count is added `+ 1` | the same three, and graph-compiler `a_route_cell_value_above_...` |
| the lane adds to a fresh count instead of the session's | the same three |
| the retarget starts from `coefficients_at(0)` | graph-compiler `a_new_cell_value_ramps_from_...`, `a_delayed_send_round_trips_through_mute_...` |
| the read is skipped while the route is inactive | graph-compiler `the_last_of_many_writes_is_applied_and_an_inactive_route_still_reads` |
| the mute word decodes inverted | the render aborts in `drain`'s debug assertion (SIGABRT) in graph-compiler and host-core `live_routes` |
| browser admission does not write the send cell | 16 host-web tests, among them gate 1, `a_live_send_edit_lands_on_a_fresh_plans_bits` and `a_refused_send_batch_writes_nothing_...` |
| the send band is back in the room pass | 31 host-web tests (every send edit is refused `UNSUPPORTED_KIND`) |
| the batch's cells are written before the room pass | (attempt 1 claimed 16 tests, among them `a_vca_batch_with_a_late_refusal_...`; the attempt-1 verdict measured six and not that one; re-run in attempt 2 below) |

**Open items.** (a) The C ABI's send edits (#1225, #1226, #1247) write
`RouteControlProducer::write` behind the stamp, as #1312's fader edits do. (b) The browser send band's
queue slots stay as unaddressable slot numbers beside the other cell bands; a renumbering of the
queue-slot layout is a separate change. (c) `hosts/host-web/MUTATIONS.md` rows that name deleted
tests are history and were not rewritten. (d) The call-graph script's self-test data still names
`GraphRouteControlProducer::free`.

### Attempt 2 (implementer, on `codex/d15-stream-b3`; findings of the attempt-1 verdict: M1, m1, m2, n1 to n3)

**M1. `a_vca_batch_with_a_late_refusal_is_refused_whole` is deleted.** Gate 4 says the full-queue
predecessor is deleted with the queue, and I found no defect only the rewrite catches. The rewrite
stages a VCA mute and then an out-of-domain `routeMatrix`; the send is refused inside the staging
loop, before the follow pass and the room pass, so the VCA mute's member-mute and follow records
were never staged, and its doc comment and the attempt-1 sentence "a domain refusal after the VCA
mute's follow write was staged" were wrong. Its send-domain half is the "domain" arm of
`a_refused_send_batch_writes_nothing_and_keeps_the_mirror`; its VCA-rollback half is held by
`a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index` and
`vca_records_are_addressed_and_shape_checked`. I did not write a room-pass replacement: a VCA mute
plus a bypass on a full insert queue would be re-written when #1345 turns bypass into a cell (the
same reason as #1346 follow-up m1), and the rollback it would defend is already held by two tests.
Mutation rows, re-run on the attempt-2 tree (one defect at a time, then reverted):

| Defect (applied to `hosts/host-web/src/lib.rs`) | Red (host-web `--features test-support --lib`, 191 tests green before) |
|---|---|
| `write_strip_cells(..)` moved before the room pass (attempt 1's M5) | six: `a_mixed_batch_is_one_transaction_across_every_queue`, `a_refused_solo_submission_leaves_the_live_controls_untouched`, `a_refused_send_batch_writes_nothing_and_keeps_the_mirror`, `a_refused_browser_batch_writes_no_cell_and_leaves_the_gate_alone`, `overfilling_a_bus_queue_is_typed_backpressure_with_no_push`, `prepared_builtin_input_filter_owner_is_atomic_across_mixed_batches` |
| `ready.vcas.rollback()` removed from the refusal arm | two: `a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index`, `vca_records_are_addressed_and_shape_checked` |

The attempt-1 record's "16 host-web tests, among them `a_vca_batch_with_a_late_refusal_...`" for the
first row was wrong; the count is six. (Attempt 1's "browser admission does not write the send
cell: 16 host-web tests" row is a different mutation and the verdict reproduced it.)

**m1. Browser typings.** `sdk/src/browser/shipped-host.d.ts` and
`hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts` (still identical): the "one batch is one
transaction" paragraph now says that fader, mute, pan and matrix values, an input's trim, polarity
and filter values and a live send's gain, matrix and mute are latest-target cells (issues 1312,
1346, 1347) and never refused for room, and that only effect records keep bounded queues and return
`RESULT_BACKPRESSURE`. `MisoCommandReason.Backpressure` is documented as an effect queue with no
room. #1399's `live_values_superseded` edits in both files are untouched.

**m2. `LiveValuesSuperseded::add` is private to `engine::realtime::latest_cell`.** The only way to
write the count is the new `CellReader::read_counted(cell, snapshot, stage, bit, superseded)`,
which is `read` plus the add of an `Applied` read's replaced writes. The four render drains
(`FaderReader`, `MatrixReader`, `InputReader` through `read_input_cell`, and `LiveRoute::drain`)
call it; their local sums and `.add` calls are gone (the saturating sum of the fader and input
drains became one saturating add per applied read, the same result). Control cannot reach the
count's write, and no `CellReader` exists on the control side, so the single-writer rule is
enforced by the type system and the reader's own one-reader rule, not by prose. Mutation: the
`add` in `read_counted` removed turns red builtins-compiler
`a_matrix_drain_counts_the_replaced_writes_and_applies_only_what_its_snapshot_covers`,
`a_drain_applies_the_newest_value_its_snapshot_covers_and_counts_the_rest` and
`many_input_edits_before_one_block_apply_the_last_and_count_each_replaced_value`, and host-web
`many_input_edits_before_one_render_...`, `many_send_edits_before_one_render_...` and
`the_status_reports_the_sessions_superseded_count`.

**Nits.** n1: the two stale doc comments in host-web `tests.rs` now say each record writes once
into its own send's cell. n2: the comment above the send kinds in `hosts/host-web/src/lib.rs`
says the send's cell holds the last record and render applies only that one and counts the others
as superseded. n3: `RouteControlRecord`'s constructor and accessors (a region) and `from_words`
(a region) are inside `REALTIME_POLICY` regions, and the floors rose from 102 to 104 regions in
27 files (`check-realtime-policy.sh`, with its self-test fixture pad from 59 to 61 regions).

**SDK.** `scripts/check-sdk-headless.sh` failed since #1346 on one eval, which #1346's follow-ups
now record (Part B of this attempt). No SDK eval asserts a send queue or a send `Backpressure`.

**Gates (this tree).** `cargo fmt --all -- --check` and clippy `--workspace --all-targets
--all-features -D warnings`: ok. `cargo test --locked` for engine, graph (`--features
test-support`), graph-compiler, builtins-compiler and host-core (`--features
control-provider,test-support`), all test binaries: pass; host-web `--features test-support`: lib
191 passed, 2 ignored (one fewer: the deleted test), integration binaries pass.
`check-workspace-policy.sh`, `check-host-core-policy.sh`, `check-builtins-policy.sh`,
`check-graph-policy.sh`, `check-rack-policy.sh`, `check-realtime-policy.sh` (104 regions in 27
files) and `test-realtime-policy.sh`: ok. `check-cross-targets.sh`: PASS. `cargo build --locked
--release -p audit -p capi && target/release/audit capi`: allocations, deallocations, locks and
syscalls 0, `total_violations` 0; `check-capi-abi.sh`: ok. Worklet chain: `build-web-audioworklet.sh
--named-twin` (shipped module `2502cd4bec95...`; it moved from `0ce76dd0...` because the readers now
call `read_counted`), `check-web-audioworklet.sh --without-metadata-regeneration`,
`check-browser-expected-resources.py --artifacts`, `check-scalar-oracle-absent.py --wasm`,
`check-web-audioworklet-v8-spill.py` and `test-web-audioworklet.sh` (private TMPDIR left empty):
pass. SDK job (source-bundle mode, `sdk/dist` removed afterwards): `check-sdk-generated.sh`,
`check-sdk-deletions.py`, `check-sdk-types.sh` (the mirror pin holds for both `.d.ts` copies),
`check-sdk-headless.sh` (362 passed) and `sdk-package.sh check`: pass. Not run: the Playwright
`browser` legs (no browsers on this box).

**Open items.** STREAMS hot-file rows for `crates/builtins-compiler/src/lib.rs`,
`crates/host-core/src/prepare.rs` and now `crates/engine/src/realtime/latest_cell.rs` do not name
#1347 (n4, root's). The spec's D2 still reads `write(record)` (n5, root's).
