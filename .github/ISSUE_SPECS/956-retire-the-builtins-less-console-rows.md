# Retire the builtins-less console rows

Scoping study: `docs/handoffs/builtins-less-removal-2026-09-27/SCOPE.md` (option (a), owner-directed removal, 2026-09-27).

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

