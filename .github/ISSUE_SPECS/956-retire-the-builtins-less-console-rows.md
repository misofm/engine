# Retire the builtins-less console rows

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (owner-directed removal, 2026-09-27; option (b) per the owner's second ruling below).

## Amendments (adversarial verification, 2026-09-27; override conflicting text)

Evidence: `docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md`.
1. Delete `tools/console-workload/tests/plumbing_profile.rs` here (it uses the deleted variant at
   `:109`); retargeting the harness is #960.
2. Re-home, do not delete, the tests at `tools/console-workload/src/lib.rs:2398` and `:2538` onto
   the gain/pan ring row.
3. Remove the assert at `tools/console-workload/src/lib.rs:1193-1200` with the builtins-less branch.
4. Write every record count as "the base count minus 2" and derive it in the change, because #955,
   #961, #938 and #965 edit the same lines and land one at a time after this issue.
5. The native pure-path target is `sixty_four_track_gain_pan_ring` (production feed), not the
   bound-feed `gain_pan_only`; say so in the row docs and the floor ruling. #965 moves every
   other row to the production feed.

## Rulings (coordinator, 2026-09-27)

The owner directed the complete removal of the builtins-less path ("I don't think we should be
benchmarking something that never gets used in the real world"). Option (b) is taken, by the owner's
second ruling ("We should remove everything related to a builtins-less compile because a
builtins-less compile is never needed in production"): the builtins-less compile is removed
entirely, including any test-only entry point, builtins become mandatory in the graph compiler,
and every test that compiled without builtins is ported to `compile_with_builtins`. The optimisation batch lands unchanged and this work deletes #937's
code afterwards. The wasm console arm is re-indexed. `sixty_four_track_gain_pan_only` is the
pure-audio-path target; its fused twin follows. #938 is re-based onto the gain/pan session. The
phase-profile harness is retargeted to `gain_pan_only`. Ported tools compile at
`Backend::current()`.

## Product outcome

The console benchmark's `sixty_four_track_plumbing_only` and `sixty_four_track_plumbing_ring` rows compile without builtins, a plan no host builds. Retire them, turn the driver-fed row into a with-builtins gain/pan row, and rewrite the floor ruling's plumbing inventory as a routing component. Tooling only; lands before #957.

## Smallest closable slice

Authorized paths:
- `tools/console-workload/src/lib.rs` and `tools/console-workload/tests/`;
- `tools/bench/src/console.rs` and `tools/bench/src/floor.rs`;
- `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-lib.jq` and
  `scripts/wasm-console-benchmark-validator.jq`;
- `scripts/test-console-benchmark.sh` and `scripts/test-wasm-console-benchmark.sh`;
- `scripts/run-console-benchmark.sh` and `scripts/operator/preflight-console-benchmark.sh`
  (counts only), and `scripts/operator/run-wasm-console-benchmark.sh:96` (a comment);
- `docs/rulings/effect-floor-accounting.md` (the "Plumbing inventory" section and lines 12
  and 621);
- that issue's spec.

1. Delete `Workload::SixtyFourTrackPlumbingOnly`, `Strip::PlumbingOnly`, the builtins-less branch
   of `build_full` (`lib.rs:1108-1191`) and its guard (`:1056-1066`).
2. Turn `SixtyFourTrackPlumbingRing` into `SixtyFourTrackGainPanRing`
   (`sixty_four_track_gain_pan_ring`).
   - Strip `GainPan`.
   - Prepare with `prepare_session_builtins` and compile with `compile_with_builtins`.
   - Bind with `into_bound_with_source_set` (`crates/builtins-compiler/src/lib.rs:2974`).
   - `FrozenSourceDriver` is unchanged.
3. Re-home the row's tests onto the new row, and delete the plumbing-only ones:
   - `tools/console-workload/src/lib.rs`: `:2133`, `:2186`, `:2268`, `:2309`, `:2398`, `:2538`;
   - `tests/chain_shape.rs`: `:329`, `:834`, `:890`, `:995`, `:1084`.
   Delete `tests/plumbing_profile.rs`, or retarget it (#960).
4. Bench, floor and jq: the gain/pan feed pair replaces `PLUMBING_FEED_PAIR`.
   - In `floor.rs`, `gain_pan_ring` takes 22 lane-ops with no control. The plumbing arm, its
     constants and its tests go.
   - Record counts go from 50 to 48, and session records from 36 to 34.
   - The pair-digest clause names `gain_pan_only` and `gain_pan_ring`.
5. Wasm arm: remove `WORKLOADS[11]`. Update the wasm validator (32 to 30 records, 16 to 15 kinds)
   and the index mutations in `test-wasm-console-benchmark.sh:165-176`.
6. Rewrite the ruling's "Plumbing inventory" as "Routing component". The 4 lane-ops stay lines of
   the identity and builtins inventories, and the identity inventory (22) becomes the floor of the
   table. Record the retirement and its reason.

Gates:
1. `gain_pan_ring`'s 64-block digest equals `gain_pan_only`'s. Its source-plane counters show
   every claim read in place by its bank gather, and none copied.
2. At the native width, `gain_pan_ring` dispatches exactly `units − 64` units per block, and `gain_pan_only` dispatches
   all of its units. This is #936's dispatch gate, re-homed.
3. The metadata-charge test passes on `gain_pan_ring`'s with-builtins graph.
4. Every other row's digest and unit census is unchanged, and
   `rg 'GraphCompiler::compile\(' tools/console-workload` is empty.
5. `floor.rs` and the jq restatement agree. `test-console-benchmark.sh`,
   `test-wasm-console-benchmark.sh` and the preflight pass. Do not run the timed runner.
6. fmt; clippy with `-D warnings`; `cargo doc` with `-D warnings`; `cargo test -p console-workload
   -p bench`.

## Standing rules for the implementer

- Work only from this body. Every "unchanged" gate is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run timed benchmarks.


## Attempt 1 evidence

Implementer: Terra (Claude Opus 5.5), 2026-09-27, branch `codex/956-retire-builtins-less-rows` on
`2b1e67df` (the local optimisation batch through #954 plus the removal briefs). Tooling commit
`6652354d`. Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, jq 1.7, `CARGO_INCREMENTAL=0`, the
worktree's own `target/` (deleted afterwards). No engine crate changed. The timed runner was not
run.

### What changed

- `tools/console-workload/src/lib.rs`: `Workload::SixtyFourTrackPlumbingOnly`,
  `Strip::PlumbingOnly`, the builtins-less branch of `build_full` and its guard are gone, and so is
  the "a driver-fed row must take the builtins-less path" assert (amendment 3).
  `SixtyFourTrackPlumbingRing` is now `SixtyFourTrackGainPanRing`
  (`sixty_four_track_gain_pan_ring`): strip `GainPan`, `prepare_session_builtins`,
  `compile_with_builtins`, bound through `into_bound_with_source_set`; `FrozenSourceDriver` is
  unchanged. `build_full` has one with-builtins path, and only the bind depends on the feed.
  `WORKLOADS` is 15 rows (index 11 removed), and `DRIVER_FED_WORKLOADS == [GainPanRing]`. The row
  docs name `gain_pan_ring` as the native pure-path target and `gain_pan_only` as its bound-feed
  twin (amendment 5).
- Re-homed tests in `lib.rs`: `:2133` becomes `the_driver_fed_row_states_the_gain_pan_rows_facts_and_its_own_feed`;
  `:2186` becomes `the_driver_fed_gain_pan_row_renders_the_bound_rows_bits` (gate 1); `:2268`
  becomes `the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs` (gate 2); `:2309`
  becomes `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables` on the
  with-builtins graph (gate 3). `:2398` (`the_frozen_source_driver_lends_the_words_it_copies`) is kept,
  with its prose fixed, and `:2538` (the #935 alignment test) moves onto the gain/pan pair
  (amendment 2).
- `tests/chain_shape.rs`: at `:329`, the plumbing arm goes, and the fold census now iterates
  `WORKLOADS` and `DRIVER_FED_WORKLOADS`. `:834` keeps only its identity-pair half, as
  `the_identity_pair_banks_alike_and_renders_apart`, and adds the ring row's shape and bits. `:890`
  and `:1084` are deleted. At `:995`, the skip arm goes, and the folded-master oracle also runs on
  the ring row. `tests/plumbing_profile.rs` is deleted (amendment 1).
- `tools/bench/src/console.rs`: `GAIN_PAN_FEED_PAIR` (`gain_pan_only`, `gain_pan_ring`) replaces
  `PLUMBING_FEED_PAIR`, both in the in-run digest assert and in the record/validator test.
- `tools/bench/src/floor.rs`: removed the `ROUTE`, `REDUCTION` and `PLUMBING_LANE_OPS` constants
  and the plumbing arm. `gain_pan_only` and `gain_pan_ring` share one arm: 22 lane-ops, no control.
  Removed `:566`. `:592` becomes
  `the_identity_inventory_is_the_floor_of_the_table_and_the_identity_pair_shares_it`, and `:622`
  becomes `the_driver_fed_gain_pan_row_is_costed_at_the_identity_inventory_and_isolates_nothing`.
- jq changes:
  - The record lib drops `plumbing_lane_ops` and pins `gain_pan_ring` at
    `[$bi, 1, "none", "…: builtins, identity"]`.
  - `session_kinds` and `driver_fed_kinds` now name `gain_pan_ring`. Its session shape is the
    six facts `gain_pan_only` has.
  - In the aggregate, the pair-digest clause names `gain_pan_only` and `gain_pan_ring`.
  - In the wasm validator, the plumbing pin is gone.
- Mutation suites: the native suite drops the plumbing cases. It adds these rejects: the retired
  kinds, gain/pan costed at 4 lane-ops or citing `plumbing`, and each gain/pan feed isolated against
  the other. It is re-indexed to the 48-record order. The wasm suite drops `.[11]` and moves
  `.[12]`–`.[16]`, `.[0:16]` down one index. It adds a reject for the retired row in place of
  `gain_pan_only`.
- Ruling: "Plumbing inventory, and the overhead floor" is now "Routing component".
  - The 4 lane-ops stay lines of both builtins inventories.
  - No row is costed at them alone.
  - The identity inventory (22) is the floor of the table, shared by `dispatch_only`,
    `gain_pan_only` and `gain_pan_ring`.
  - `gain_pan_ring` is the native pure-path target.
  - The section records the retirement and its reason, and keeps the undeclared 47-lane-op
    `builtins_only − gain_pan_only` control.
  - Line 12 and the derived-floor table's first row (now a component line) are rewritten.

### Record counts, each derived as the base count minus 2

| where | base (`2b1e67df`) | after |
|---|---:|---:|
| `console-benchmark-validator.jq` `length` | 50 | 50 − 2 = 48 |
| `console-benchmark-validator.jq` `console_session` records | 36 | 36 − 2 = 34 |
| `console-benchmark-validator.jq` unique `record:kind:round` | 50 | 50 − 2 = 48 |
| `run-console-benchmark.sh` `wc -l` | 50 | 50 − 2 = 48 |
| `operator/preflight-console-benchmark.sh` `records_required` | 50 | 50 − 2 = 48 |
| `wasm-console-benchmark-validator.jq` `length` and unique `kind:round` | 32 | 32 − 2 = 30 |
| `operator/run-wasm-console-benchmark.sh` `wc -l` | 32 | 32 − 2 = 30 |
| `operator/preflight-wasm-console-benchmark.sh` `records_required` | 32 | 32 − 2 = 30 |
| wasm kinds (`unique` kinds and `kind:digest` pairs) | 16 | 16 − 1 = 15 |
| native session kinds (`session_kinds`) | 18 | 17 |

### Gates

1. **Digest and source planes.** `cargo test --locked -p console-workload
   the_driver_fed_gain_pan_row_renders_the_bound_rows_bits`: PASS.
   - `gain_pan_ring`'s 64-block digest equals `gain_pan_only`'s, and both equal the standing pin
     `01e465a7…2dfdb4`.
   - The source-plane counts are `[0, 64 × 64, 0]`: every claim is read in place by its bank
     gather, none is copied and none is served silence. The bound row counts `[0, 0, 0]`.
   - Shape `[8, 24]` and transposes are equal. There are 64 folds on each row, no forbidden
     operation, and no collapse or transition.
2. **Dispatch.** `the_driver_fed_gain_pan_row_dispatches_every_unit_but_its_inputs`: PASS.
   - At `Backend::current()` (Simd8) both rows bind 73 units.
   - `gain_pan_ring` dispatches 73 − 64 = 9 units per block, on each of 64 blocks.
   - `gain_pan_only` dispatches all 73.
3. **Metadata charge.** `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables`:
   PASS on the with-builtins graph.
   - `estimate − semantic_estimate` graph metadata equals the recomputed runtime metadata plus the
     bank-slot reservation. The reservation is `GraphBankSlotResourceEstimate::checked_for_mask`
     over the artifact's retained builtin banks, masked at their width.
   - At a vector width with no effect, no scalar owner, effect control or effect bank is charged.
   - Each executor table is one entry per emitted op.
   - The bound tables are `[9 × 4, 0]` and fit.
4. **Every other row unchanged, and no builtins-less compile.**
   - A scratch probe (`tests/zz_census_probe.rs`, not committed) ran every native session row for
     64 blocks at Simd8 and at Simd4, before (`2b1e67df`) and after. For each row it recorded the
     digest, units, banked units, symmetry census, bank shape, transposes, folds and redirects.
   - `diff` shows only the removed `plumbing_only` lines and `plumbing_ring` renamed to
     `gain_pan_ring`. The other 32 lines (16 rows × 2 widths) are byte-identical.
   - `gain_pan_ring`: Simd8 digest `01e465a7…`, units 73 (8 banked), census `[129, 129]`, shape
     `[8, 24]`, 512 transposes, 64 folds. Simd4: the same digest, units 81, shape `[16, 48]`, 64
     folds. `gain_pan_only` is the same except census `[65, 129]`: bound host processors decline
     the witness, and the claimed `SourceInput` ops do not.
   - `rg 'GraphCompiler::compile\(' tools/console-workload` is empty (exit 1).
5. **Floor, jq and runner checks.**
   - `floor::tests::rust_and_jq_floor_tables_have_exact_key_value_parity`: PASS.
   - `bash scripts/test-console-benchmark.sh`: PASS.
   - `bash scripts/test-wasm-console-benchmark.sh`: PASS.
   - `bash scripts/operator/preflight-console-benchmark.sh --step preflight-956`: PASS
     (`records_required` 48, workload launches 0, candidate `6652354d`). The preflight left no
     `artifacts/steps/preflight-956` behind.
6. **Build, lint and tests.**
   - `cargo fmt --all --check`: clean.
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: exit 0.
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: exit 0.
   - `set -o pipefail; cargo test --locked -p console-workload -p bench`: exit 0. That is bench 64,
     console-workload lib 8, automation 4, chain_shape 23 and placement 3.
   - `bash scripts/check-bench-policy.sh`: ok.
   - `bash scripts/check-realtime-policy.sh`: ok, 56 marked regions in 16 files.
   - `bash scripts/check-workspace-policy.sh`: ok.

Red mutation, run and reverted: `source_feed` returns `Bound` for `gain_pan_ring`. Four lib tests
go red: the facts test, gate 1 (the planes), gate 2 (dispatches) and gate 3 (the bound tables).

### Deviations

1. **Outside the authorised paths.**
   - `scripts/operator/run-wasm-console-benchmark.sh:483` (`wc -l` 32 → 30) and
     `scripts/operator/preflight-wasm-console-benchmark.sh:256` (`records_required` 32 → 30). Both
     are record counts of the wasm arm this issue re-indexes. Without them the wasm runner refuses
     every run at `record_count`.
   - The one-word doc fix "sixteen" → "fifteen" in `tools/wasm-console/src/main.rs:5` and
     `tools/wasm-console-guest/src/lib.rs:7`.
2. **Kept, not deleted.** From `chain_shape.rs:834`, the identity-pair half
   (`gain_pan_only` and `dispatch_only` bank alike and render apart) is kept, because it is not
   plumbing-specific. The ring row's shape and bits were added to it.
3. **Two tests widened to the ring row.** `chain_shape.rs` `:329` and `:995` now iterate
   `DRIVER_FED_WORKLOADS` beside `WORKLOADS`, because the ring row is banked and folds like any
   other. The folded-master oracle, meters included, therefore also binds it through
   `into_bound_with_source_set`.
4. **An assertion kept in `floor.rs:592`.** Beside the identity-pair half, the test keeps a
   floor-of-the-table assertion, now anchored on the identity inventory, as item 6 states.
5. **A new facility path.** `build_full` no longer refuses a console facility on any row, so the
   ring row may carry meters. No bench arm asks for them.
6. **Left as history.**
   - `crates/graph/tests/MUTATIONS.md` (`:309`, `:350`, `:416-419`, `:431`, `:486-489`) still
     names the deleted console tests in its historical mutation records. That file is graph-side,
     and #957 owns it.
   - The plumbing-floor arm prose and the `--plumbing-floor*` registrations in
     `scripts/run-console-benchmark.sh` are kept, because only the count there was authorised.
