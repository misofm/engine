FAIL

# #1347 attempt 1 -- adversarial verdict

Reviewed `git diff e61b85c51^ e61b85c51` on an export of e61b85c51 at /tmp/claude-1002/v1347/tree
(git-initialised for the reachability step), built with its own target dir. Mutations ran on a
second export with a second target dir. One probe ran on an export of the parent with a third
target dir. Read the spec at e61b85c51: the body did not change against e61b85c51^, and only the
Attempt record was added. Also read AGENTS.md, decision 15, the #1312 spec (D7, Amendment 1),
the #1346 verdict, #1446 D4, #1509 and STREAMS.md.

The product change is correct. Render is safe, the acked-batch answer is no, and all 60 gates
pass. The attempt fails on one test-value finding: a rewritten browser test has no catch of its
own that I could find, spec gate 4 says to delete it, and the attempt record claims a mutation
result for it that does not reproduce.

## What was checked and holds

- **D1/D3, graph.** `RouteLane` is a `LatestCell<6>` with a `DirtyWord`, in one `Arc`.
  `LiveRoute::drain` (crates/graph/src/runtime.rs:889-930) does these steps:
  - It takes the dirty bits, then reads the cell under the block's snapshot.
  - For a value that it has not applied, it retargets once from
    `ramp.coefficients_at(self.position)`. This is the coefficient the route is at.
  - It adds `s - p - 1` to the session count and returns 0 or 1.
  - It still runs in every block, active or not (runtime.rs:3450-3460, before the `active()`
    check).
  - In the render path there is no allocation, no lock and no syscall. It does atomic loads, one
    `swap`, and a load and store on the count.
  - `from_words` runs the constructor's own check. A mute word above 1 decodes to `None`.
  - `debug_assert!` is the only check that can fire, and only in a debug build.
- **Acked-batch question: can an ack ever precede a drop? No.**
  - Browser: kinds 13-15 build their record at staging (`producer.record`, a refusal before any
    write). The follow pass builds its records before the stamp. The room pass skips the send
    band (lib.rs:5755-5759), because a cell has no room.
  - `write_strip_cells` writes the send cells under the batch's one stamp, after every check. Its
    two `expect`s cannot fail: staging resolved the route through `route_slot` and
    `route_controls.get`.
  - A write cannot fail. A replaced value is in the committed mirror and is counted.
  - Host-core `RouteControlProducer::set` is `record()?` and then an infallible write.
  - M5 (the cells written before the room pass) turns six browser refusal tests red. M8 (`set`
    writes on refusal) turns `a_refused_send_record_writes_nothing` red.
- **Point 1, the move to `engine::realtime`.**
  - Forced: `graph` cannot depend on `builtins-compiler`, and the route lanes must add to the same
    session count that strips add to. M9 (prepare gives the routes a fresh count) turns both
    gate-1 tests red.
  - Within stream B's ownership: STREAMS gives `crates/engine/src/realtime` to B, and no hot-file
    row names `latest_cell.rs` or `mod.rs`.
  - Recorded: Attempt record item 1.
  - No `unsafe` moved. `SharedAllocation<T>` is a safe `repr(C)` layout struct. The unsafe
    allowlist did not change. One `REALTIME_POLICY` region left builtins-compiler and one entered
    `latest_cell.rs`, so the gate still reports 102 regions in 27 files.
  - `builtins_compiler::LiveValuesSuperseded` stays as a re-export, so no caller moved. But see m2.
- **Point 2, `write(stamp, record)`.** Forced. #1312 D7 requires every cell write to carry the
  transaction's stamp. This spec's Amendment 1 makes the route cells revision-bounded, and a write
  without a stamp cannot satisfy it. The spec's D2 wording is the pre-amendment shorthand (n5).
  Root does not need to ratify it.
- **Point 3, files outside the list.** All are forced:
  - `crates/host-core/src/route_controls.rs`: `free`, `push` and `Full` are gone.
  - The builtins-compiler delegate signature.
  - Four test files that used the queue.
  - The docs paragraphs, which would otherwise be false.
  - Both `test_only_write_count` accessors are `#[cfg(any(test, feature = "test-support"))]` and
    `#[doc(hidden)]`. No shipped build turns on `host-core/test-support` or `graph/test-support`:
    capi has them only as dev-dependency features, and `check-capi-abi.sh` builds `-p capi`
    alone. The shipped wasm and libcapi pass the scalar-oracle and call-graph gates.
- **Point 4, MUTATIONS.md.** Leaving `hosts/host-web/MUTATIONS.md` as history is correct, not
  stale evidence. The ledger is per issue ("Each row was applied, its test run red, and the tree
  restored"). Its rows already name removed code (`try_push`, queue rooms). #1446 D4 and #1509 D1
  keep such records unedited. The same holds for the callgraph self-test data that names
  `GraphRouteControlProducer::free`, which is a synthetic name.
- **Point 6, resources.**
  - `cell_bytes = routes x size_of::<SharedAllocation<RouteLane>>()` is 144 B:
    16 + 3 x (24 + 8 + 8) + 8. The graph-compiler gate 8 checks it against that formula.
  - M12 (the browser bridge rows leave the cells out) turns
    `the_exact_retained_budget_charges_the_send_lanes` red.
  - No pinned digest or fixture should have moved, and none did: no corpus carries a live route.
    These gates all pass: `check-builtins-fixtures.sh` (50 files, the manifest digest),
    `check-console-fixtures.sh`, `check-graph-determinism.sh`, the release audit tests,
    `check-browser-expected-resources.py`, the browser bridge budget tests and the host-core cap
    test.
  - The shipped module's sha256 is `0ce76dd0ca726c67...`, which equals the attempt record's value.
- **#1509.** No new person-role prose. "Producer" appears only as the existing type names, the
  test producer thread and the "cell producer" handle, which is the queue-role sense that #1509
  G4 excludes.

## Findings

### MAJOR

- **M1. `a_vca_batch_with_a_late_refusal_is_refused_whole`
  (hosts/host-web/src/tests.rs:14750-14800) is a rewritten test with no catch of its own. Spec
  gate 4 says to delete it, and the attempt record's evidence for it is wrong.**
  - Its predecessor filled `drums-verb`'s send queue and expected `COMMAND_REASON_BACKPRESSURE`.
    That is exactly the class gate 4 deletes.
  - The rewrite stages a VCA mute at index 0 and an out-of-domain `routeMatrix` at index 1. The
    send is refused inside the staging loop (`producer.record` -> `DOMAIN`), before the follow
    pass and before the room pass. So neither the VCA mute's member-mute records nor its follow
    records are ever staged. The test's doc comment and the attempt record (spec :186-187,
    "a domain refusal after the VCA mute's follow write was staged") are wrong about this.
  - **Measured:**
    - M5 (`write_strip_cells` moved before the room pass) turns six tests red, and this test stays
      green. The attempt record's row (spec :238) says "16 host-web tests, among them ...
      `a_vca_batch_with_a_late_refusal_...`".
    - M7 (no `vcas.rollback()`), M14 (`LiveVcaState::rollback` keeps `vca_mute`) and M15 (the VCA
      mute arm commits early) each turn exactly the same three tests red:
      `a_long_vca_mute_ramp_refuses_at_the_first_coalesced_index`,
      `vca_records_are_addressed_and_shape_checked` and this one.
    - Its send-domain half is `a_refused_send_batch_writes_nothing_and_keeps_the_mirror`'s
      "domain" case.
    - I found no plausible defect that only this test catches. AGENTS.md: "there is no PASS
      without it", and "a change that supersedes a test deletes it in the same PR".
  - **Fix:** delete it, as gate 4 says. Or move its late refusal to a real room-pass refusal
    after the follow pass, such as a VCA mute plus a bypass on a full insert queue, and show a
    mutation that only it catches. Then correct the mutation-table row and the "Tests moved"
    sentence.
  - Note for the record: dropping `ready.vcas.commit()` on success (M13) is green on every browser
    test, here and also at e61b85c51^. The 1245 verdict credited that catch to the predecessor,
    so the gap is older than this slice.

### MINOR

- **m1. The public browser typings still promise send backpressure.**
  - `sdk/src/browser/shipped-host.d.ts:63-66` and `:342`, and the identical
    `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts:63-66` and `:342`, say:
    "Effect, input trim/polarity and live-send records keep bounded per-destination queues ... a
    flood of those is refused". `Backpressure` is documented as "A bounded control queue (an
    effect, input trim/polarity or live-send lane) had no room".
  - After #1346 and #1347, only effect lanes are queues. A send flood is never refused for room.
  - Stream B's batch 2 corrected the same prose for #1312 (dd23aea09). The b3 batch must do the
    same for input and send before its push, in this slice or at the batch boundary. The file row
    in STREAMS lists B.
- **m2. `LiveValuesSuperseded::add` is now `pub` on an engine type**
  (crates/engine/src/realtime/latest_cell.rs:697-711).
  - Its single-writer rule ("render is its only writer", a load and a store, not a
    read-modify-write) is now only prose on a public API. Any crate can call it from control and
    race render.
  - Before the move it was private to builtins-compiler.
  - The move is forced, but the exposure is not. One option is a reader-side API in `engine`
    that applies a `CellRead::Applied`'s `superseded` to a count, so only `engine` calls `add`.
    Another is `#[doc(hidden)]` plus a filed follow-up.
  - Under the owner principle, record the choice. Do not leave it implicit.

### NIT

- **n1.** Stale test doc comments that still describe queues:
  - hosts/host-web/src/tests.rs:11258: "take exactly one slot of its own send's queue".
  - :11351: "take one slot of their own send's queue each".
  - The assertions now count cell writes.
- **n2.** The comment at hosts/host-web/src/lib.rs:5104-5108 says "the render plane, applying
  them in order, settles on the last". Render now applies only the newest value and counts the
  rest.
- **n3.** `RouteControlRecord::from_words` (crates/graph/src/lib.rs:1012) is reachable from render
  but sits outside a marked `REALTIME_POLICY` region, like the record's existing accessors. #1346
  marked its render helpers. Consider marking it.
- **n4.** STREAMS hot-file rows for `crates/builtins-compiler/src/lib.rs` (B #1312, #1346 only)
  and `crates/host-core/src/prepare.rs` (B #1312 -> A ... -> B #1344) do not name #1347. Each edit
  is one call or one signature. Root sequences these, as in the #1346 verdict's m5.
- **n5.** Spec D2 still reads `write(record)`. The attempt record explains the stamp. Root may
  amend D2 to `write(stamp, record)` when it syncs the body. No ratification is needed.

## Test value (new or rewritten tests)

- graph-compiler `a_new_cell_value_ramps_from_the_coefficients_the_route_is_at` (gate 2): red if
  a new cell value retargets from the old ramp's target instead of the in-flight coefficients.
  Mutation M3b (`coefficients_at(u32::MAX)`) turns only this test red. M3 (`coefficients_at(0)`)
  turns it and the existing delayed-send test red.
- graph-compiler `a_route_cell_value_above_the_blocks_snapshot_waits_for_its_revision`: red if the
  route drain reads `LiveSnapshot::ALL` and applies a value a block before its revision is
  covered. M1 turns only this test red.
- graph-compiler `the_last_of_many_writes_is_applied_and_an_inactive_route_still_reads`: red if
  the graph's route read is skipped while the route is inactive (M4) or does not charge the session
  count (M2). It is the only graph-crate test that catches M2.
- host-core `many_send_edits_before_one_block_apply_the_last_and_are_counted`: red if host-core's
  preparation gives the route lanes a count other than the session's (M9) or does not count the
  replaced writes (M2, M2b). It is the only host-core catch of each.
- host-core `a_refused_send_record_writes_nothing` (rewritten): red if a refused `set` writes any
  value to the cell. M8 turns only this test red.
- host-web `many_send_edits_before_one_render_apply_the_last_and_are_counted` (gate 1): red if the
  browser's send lane does not count replaced writes in the session count (M2, M9). It is the only
  browser catch. M6 also turns it red.
- host-web `a_follow_ramp_past_the_send_bound_refuses_the_strip_mute_whole` (the surviving half,
  renamed): red if the follow pass clamps an overlong strip-mute window instead of refusing it
  (M10). This is the same catch the half had before, and the VCA long-mute test shares it, as
  before.
- host-web `a_refused_send_batch_writes_nothing_and_keeps_the_mirror` (renamed; the superseded
  full-queue arm is deleted): red if cells are written before the room pass (M5) or the send cell
  is not written (M6). It keeps its domain and effect-queue arms.
- host-web `the_exact_retained_budget_charges_the_send_lanes` (rewritten depth arm): red if the
  bridge rows leave out the cells (M12, the only catch).
- host-web `a_vca_batch_with_a_late_refusal_is_refused_whole`: **no unique catch found** (M1
  above).

## Mutations re-run (applied one at a time, then reverted)

| # | Defect | Red |
|---|---|---|
| M1 | route drain reads `LiveSnapshot::ALL` | GC `a_route_cell_value_above_...` only |
| M2 | replaced count not added | GC `the_last_of_many_...`; HC `many_send_edits_...`; WEB `many_send_edits_...` |
| M2b | count `+ 1` | the same three, and GC `a_route_cell_value_above_...` |
| M3 | retarget from `coefficients_at(0)` | GC `a_new_cell_value_ramps_...`, `a_delayed_send_round_trips_...` |
| M3b | retarget from the old target | GC `a_new_cell_value_ramps_...` only |
| M4 | read skipped while inactive | GC `the_last_of_many_...`; G `every_live_drain_runs_once_...`; HC `live_vca_moves_...`; 7 WEB tests |
| M5 | cells written before the room pass | 6 WEB tests (not `a_vca_batch_with_a_late_refusal_...`) |
| M6 | browser writes no send cell | 16 WEB tests incl. gate 1 |
| M7 | no `vcas.rollback()` | WEB long-VCA-mute, VCA shape check, `a_vca_batch_with_a_late_refusal_...` |
| M8 | host-core `set` writes on refusal | HC `a_refused_send_record_writes_nothing` only |
| M9 | prepare attaches routes with a fresh count | HC and WEB `many_send_edits_...` |
| M10 | follow window clamped | WEB `a_follow_ramp_past_...`, long-VCA-mute |
| M11 | send band back in the room pass | 31 WEB tests |
| M12 | bridge rows leave out `cell_bytes` | WEB `the_exact_retained_budget_...` only |
| M13 | `vcas.commit()` dropped | none, here and at e61b85c51^ |
| M14 | VCA rollback keeps `vca_mute` | same three as M7 |
| M15 | VCA mute arm commits early | same three as M7 |

## Gates run (export of e61b85c51; all exit 0)

- **lint job, every step:**
  - `cargo fmt --check`, then clippy with `--workspace --all-targets --all-features -D warnings`,
    then `cargo doc -D warnings`.
  - Workspace policy and its mutation tests, then session policy.
  - Env vocabulary, then bench policy and its mutations.
  - test-support CI policy and its mutations.
  - Scalar-oracle self-test.
  - Script reachability and its mutations (on the git-initialised copy).
  - Console benchmark fixture, then the builtins fixture manifest mutation tests.
  - Bench preconditions, then host-core policy, then protocol policy.
  - Realtime policy (102 regions in 27 files) with its mutations, the audit-leak check and the
    artifact-evidence-leak check.
  - Trace validator, then lane policy, then the unfused seal and its self-test.
  - **check-rack-policy**, builtins policy and graph policy, each with its mutations.
  - Effect-runtime policy and fixtures.
  - Conformance boundaries, then the PEQ render seal.
  - Release-shape self-test, then the npm publish modes.
  - Stem store, then stem identity.
  - The two sub-v3 refusal probes, then the AVX2+FMA probe.
- **test-release job:**
  - lane, math and wasm-gates (G5 native digests included).
  - filter_liveness, then tail_contract, then ramp_endpoint (8 crates), then `designer_total
    --ignored`.
  - The M3 FMA cfg check.
  - **The loom leg (`spsc_loom`, 13 passed).**
  - Not run: M1 and F1 exhaustive, which the math closure leaves out (no math change).
- **test-debug-a:** the workspace `--all-targets` run and the doctests, with the CI feature list.
- **Spec gates:**
  - `cargo test -p graph --features test-support`.
  - `-p host-web --features test-support` (191 lib passed).
  - `-p host-core --features control-provider,test-support`.
- **audit-native subset:**
  - The release build of audit, bench, capi and session-validator.
  - **`audit capi`**: allocations, deallocations, locks and syscalls 0; `total_violations` 0.
  - `trace-builtins-audit.sh`, `trace-builtins-graph-audit.sh` and `trace-graph-audit.sh`.
  - Realtime, builtins and builtins-graph probe mutations.
  - `check-capi-abi.sh` and its self-test, then the libcapi scalar-oracle check.
  - `check-graph-determinism.sh`.
  - **`check-builtins-fixtures.sh . target/release/audit` (the manifest digest, 50 files)**, then
    `check-console-fixtures.sh`.
  - `cargo test --release -p audit -p bench -p console-workload`.
- **Worklet chain:**
  - `build-web-audioworklet.sh --named-twin`. The module is `0ce76dd0ca726c674a6c22fd2ec6470f80fab1f94271e1dfb7ff6b00bc3859f7`.
  - `strip-wasm-names.py` self-test and check.
  - `check-web-audioworklet.sh --without-metadata-regeneration`, then
    `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh`, with no TMPDIR leftovers.
  - The V8 spill gate and its self-test.
- **`check-cross-targets.sh`:** PASS.
