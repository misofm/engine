# Retire the one-shot rack, builtins and graph-compiler benchmarks

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, rows B2-B4. No
ruling is needed: each is a "sole exactly-once" benchmark for a closed issue. It has either
produced its record or can no longer produce one, and none of them measures the shipped host path.

## Context

| family | files | lines | why it cannot run again, or is not wanted |
|---|---|---:|---|
| Rack #038 | `scripts/run-rack-benchmark.sh`, `scripts/test-rack-benchmark.sh`, `scripts/rack-benchmark-{record-lib,record-validator,validator}.jq`, `scripts/check-rack-benchmark-fixture.sh`, `scripts/fixtures/rack-benchmark-validator-record.json`, `scripts/operator/preflight-rack-benchmark.sh`, `fixtures/rack/issue038-v1/`, `tools/bench/src/rack.rs` | 1,905 (947 Rust) | `artifacts/issue038/` exists, and the runner refuses to overwrite it; the operator preflight also cannot run (section 5 of the audit) |
| Builtins #072 and #431 | `scripts/{run,preflight,test}-builtins-benchmark.sh`, `scripts/builtins-benchmark-{record-validator,validator}.jq`, `scripts/{run,preflight,test}-builtins-current-benchmark.sh`, `scripts/builtins-current-benchmark-{record-validator,validator}.jq`, `tools/bench/src/builtins.rs` | 4,935 (2,095 Rust) | #431's `artifacts/issue431-full-chain/` exists; #072's runner is "sole exactly-once" and reads spec 068's text |
| Graph compiler #006 | `scripts/run-graph-compiler-benchmark.sh`, `scripts/promote-issue006-graph-benchmark.sh`, `scripts/test-graph-benchmark.sh`, `scripts/graph-benchmark-{record-validator,validator}.jq`, `scripts/fixtures/graph-benchmark-validator-record.json`, `tools/bench/src/graph.rs` | 1,350 (847 Rust) | "sole future Issue-006 entry point"; it measures compile time, not a host render path |
| Wrapper | `scripts/check-builtins-targets.sh` | 10 | a "thin wrapper kept so any caller keeps working", with no live caller: `test-builtins-benchmark.sh:597` only writes a stub of that name into a scratch template, and every other mention is a comment |

**CI steps that exist only for these benchmarks:**

- `qualification.yml` lint:
  - "Console and rack benchmark fixture integrity": its rack half.
  - "Builtins benchmark real-tree manifest consumers" (`test-builtins-benchmark.sh
    --check-manifest-consumers`). It checks that one fixture hash is copied identically into 7
    places; 6 of those places are the files deleted here.
  - The `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` lines of "Benchmark
    preconditions and validator mutation tests".
- `nightly.yml` `moved-mutation-suites`: `test-builtins-benchmark.sh` and `test-graph-benchmark.sh`.

**Coupling that must be edited, not deleted:**

- `scripts/test-env-vocabulary.sh` uses `run-rack-benchmark.sh` as its mutation target (`:34`,
  `:46`, `:94`, `:147`, `:163`). Re-point it at `run-console-benchmark.sh`.
- `tools/bench/src/main.rs` lists the `builtins`, `graph` and `rack` subjects.
- `docs/ENGINE_ENV_VOCABULARY.md` lists variables used only by these runners. For example,
  `MISO_ENGINE_BENCH_CANDIDATE_SHA256` is used only by rack. `check-env-vocabulary.sh` must still
  pass.
- `tools/audit/src/builtins_graph.rs`'s `ACCEPTED_MANIFEST_SHA256` stays. It guards the live
  `fixtures/builtins/v1` corpus in `audit builtins-graph`.

**Not coupled:** `scripts/check-builtins-listening-033.py` pins hashes of #110 builtins-benchmark
outputs as constants and reads none of these files.

## Smallest closable slice

1. Delete every file in the table, and `artifacts/issue038/` and `artifacts/issue431-full-chain/`.
2. Remove the `builtins`, `graph` and `rack` subjects from `tools/bench/src/main.rs`. Delete any
   `bench-support` or `bench` helper that becomes unused, proven by compile.
3. Edit the CI steps listed above: delete the lint manifest-consumer step, drop the rack half of
   the fixture step and the two mutation-test lines, and drop the two nightly lines.
4. Re-point `test-env-vocabulary.sh`. Drop only the vocabulary entries that no remaining script or
   crate reads, as `check-env-vocabulary.sh` proves.
5. Update live docs that name these runners (`scripts/operator/README.md`, the rulings'
   inventories only where they state current state). Leave spec 068 in place; `R10-…` decides on
   closed specs.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
   - `cargo test --locked --release -p bench -p console-workload` passes. Its test list shrinks
     only by tests inside the deleted subject modules; list them.
2. **Console digests.** The `gain_pan_profile digests` output is byte-identical on base and change.
   No engine source changes.
3. **Shipped artifact: unchanged.** No crate in its closure changes. `git diff --stat` shows none
   of `crates/`, `hosts/`.
4. **CI routing.**
   - Every edited step still runs something, or is deleted whole. The `verdict` expectation table
     is unchanged, because no job is added or removed.
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-bench-policy.sh`, `test-bench-policy.sh`, `check-env-vocabulary.sh`,
     `test-env-vocabulary.sh`, `test-console-benchmark.sh` and `check-bench-preconditions.sh` pass.
5. **No live claim lost.** The only tests removed are the deleted runners' own mutation suites and
   the unit tests inside the three bench subjects: `rack` 3, `builtins` 11 and `graph` 7. Some of
   those drive production code without timing, for example
   `builtins::all_render_workloads_arm_only_product_render_without_timing`,
   `rack::zero_launch_preflight_prepares_each_exact_production_workload` and
   `graph::canonical_benchmark_fixture_prepares_and_compiles`. For **each** deleted test, the
   evidence either:
   - names the surviving test in the owning crate that asserts the same product claim; or
   - ports the untimed assertion into that crate's tests before deleting it.

   `fixtures/builtins/v1` stays guarded by `check-builtins-fixtures.sh` and
   `audit builtins-graph`.

## Dependencies

`00-…`. Land it in the same CI-conscious batch as `04a-…`, `04c-…` and `05-…`. It must land before
`R10-…`, because spec 068 is read here.

## Standing rules for the implementer

- Launch no benchmark workload.
- Commit on `codex/<issue>-retire-one-shot-benchmarks`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F14. A scratch copy with the three subjects deleted builds and
`cargo test --release -p bench -p audit -p wasm-gates --no-run` passes, but **the required lint job
fails**, and the draft does not budget for it.

1. **`scripts/check-bench-policy.sh:184-185` fails.** Its `timed_subjects` ratchet ("it never
   shrinks") names `tools/bench/src/rack.rs`; on the scratch tree lint stops with
   `converted subject is missing: tools/bench/src/rack.rs`. The slice must replace that entry (for
   example with `tools/bench/src/console.rs`, which already uses the shared timer) and amend the
   "never shrinks" comment with the reason. `check-bench-policy.sh:205-252` names further bench
   files; re-check each.
2. **Mutation tests that write into the deleted files** must be re-pointed, keeping each case's
   intent: `scripts/test-bench-policy.sh` (rack.rs at `:113`, `:251-266`; graph.rs at `:224`,
   `:229`; builtins.rs at `:234`), `scripts/test-realtime-policy.sh:281` (rack.rs),
   `scripts/test-effect-runtime-policy.sh:93-99` (rack.rs) and `scripts/test-env-vocabulary.sh:39`
   (rack.rs, in addition to the `run-rack-benchmark.sh` lines the draft already names).
3. **Do not delete the only check of a live audit constant.** The "Builtins benchmark real-tree
   manifest consumers" step (`qualification.yml:355-356`) is the only thing that ties
   `tools/audit/src/builtins_graph.rs:48` `ACCEPTED_MANIFEST_SHA256` to
   `fixtures/builtins/v1/MANIFEST.tsv`: `audit builtins-graph` only prints the constant (`:297`),
   and `check-builtins-fixtures.sh` never compares it. Failure scenario: the fixture manifest
   changes, the audit constant silently goes stale, and nothing turns red. Keep a one-consumer
   check (a few lines in `check-builtins-fixtures.sh` comparing the constant with the manifest's
   hash, with a mutation case) before deleting `test-builtins-benchmark.sh`.
4. **Gates:** add `check-realtime-policy.sh`, `test-realtime-policy.sh`,
   `check-effect-runtime-policy.sh` and `test-effect-runtime-policy.sh` to gate 4.
5. Mobile scope: no effect (`tools/bench` is x86 tooling).

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1026-retire-oneshot-benches` from `codex/batch-slim-1`
(`b8bea8e1`). Commits: `560f2119` (the retirement and every gate edit) and `b9295585` (gate 5 ports
and the dependency-union pin). No benchmark workload was launched.

**Size.** 60 files, +905 / −8,652 lines. Deleted: 41 files (−8,650), all 28 files in the Context
table plus `fixtures/rack/issue038-v1/` (2 files), `artifacts/issue038/` (3) and
`artifacts/issue431-full-chain/` (8). That is 3,889 lines of Rust in the three subjects and 4,329
lines of scripts. Added: 811 lines of Rust tests (gate 5) and 94 lines of gate and doc edits, before this record.

### What changed besides deletion, and why

| file | change | reason |
|---|---|---|
| `tools/bench/src/main.rs`, `Cargo.toml`, `Cargo.lock` | drop the `builtins`, `graph` and `rack` subjects and the now-unused `rack` dependency | step 2; clippy `-D warnings` on `bench` then showed no dead helper, and no `bench-support` item became unused |
| `scripts/check-bench-policy.sh` | `timed_subjects` names `tools/bench/src/console.rs` in place of `rack.rs`; the "never shrinks" comment now says a subject leaves only with its benchmark, and its slot goes to a surviving subject | Amendment 1 (F14). `console.rs` already calls `timing::timed` and has no clock or digest of its own, so the list keeps 3 entries |
| `scripts/test-bench-policy.sh` | second-allocator, second-percentile, second-percentile-summary-owner, second-digest-sink, the three converted-subject cases and subject-bypasses-metadata-snapshot now write into `console.rs`. The escaper multi-file fault expects `effect_interchange.rs` and `json.rs` (2 lines, still reversible). The timer-loss `sed` is now `/g` | Amendment 2; each case keeps its intent |
| `scripts/test-realtime-policy.sh:281` | the fixture's safe bench subject is `console.rs` | Amendment 2 |
| `scripts/test-effect-runtime-policy.sh:93-99` | the unrelated-tool mutation writes into `console.rs` | Amendment 2 |
| `scripts/test-env-vocabulary.sh` | the five `run-rack-benchmark.sh` targets now use `run-console-benchmark.sh` (it holds `MISO_ENGINE_BENCH_CPU_MODEL`, the full-mode payload, here and on #1025's branch). `:39` now uses `console.rs`. The documented-name count is 134 → 117 | step 4 and Amendment 2 |
| `docs/ENGINE_ENV_VOCABULARY.md` | 17 rows dropped, exactly the names `check-env-vocabulary.sh` reported unused: `BENCH_CANDIDATE_SHA256`, `CPU_ARCHITECTURE`, `KERNEL`, `LOGICAL_CORE_COUNT`, `OS`, `PHYSICAL_CORE_COUNT`, and 11 `TEST_*` stub hooks | step 4 |
| `scripts/check-builtins-fixtures.sh` | before any audit run, requires `tools/audit/src/builtins_graph.rs` to declare `ACCEPTED_MANIFEST_SHA256` exactly once, equal to the manifest's SHA-256 | Amendment 3 (F16). The script runs in audit-native, which a `fixtures/` or `tools/` change routes to |
| `scripts/test-builtins-fixtures.sh` (lint) | a positive control: a copy with the real constant passes the whole script, with a stub audit binary that accepts the corpus and refuses the canary. Also `stale` (zeroed hash) and `missing` (renamed constant) mutations, each required to fail with the consumer diagnostic | Amendment 3. Counter-mutant: with the comparison disabled, the suite fails with `builtins manifest consumer mutation escaped: stale` |
| `.github/workflows/qualification.yml` (lint) | "Console and rack benchmark fixture integrity" becomes "Console benchmark fixture integrity". The manifest-consumer step is deleted whole. The `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` lines are dropped, and the comment about the moved suites is updated | step 3. No job was added or removed, so the verdict table is untouched |
| `.github/workflows/nightly.yml` | the "Benchmark-lifecycle mutation suites" step is deleted whole. So is "Install ripgrep", whose only users were those two suites (the stem-store ledger is node-only). The job's display name and its failure-issue row become "stem-store mutation ledger"; its id is unchanged | step 3 |
| `scripts/check-conformance-boundaries.sh` and `test-conformance-boundaries.sh` | the pinned `bench` dependency union loses `rack`, and so does the test's synthetic manifest | not named in the spec, found by the lint sweep: `consolidated benchmark dependency union changed` |
| `scripts/check-cross-targets.sh` | a comment: the builtins wrapper was deleted, since it had no caller | the comment said all three wrappers exist |

Left alone on purpose:
- `run-console-benchmark.sh`, which is #1025's.
- `scripts/operator/README.md`, which names no deleted script.
- The ruling inventories (`de-versioning-inventory.md:783`, `prefix-strip-inventory.md:274`) and `docs/audits/`, which record past state.
- Spec 068.
- `test-realtime-policy.sh:362`, whose case name mentions the rack benchmark but writes only `other.rs`.
- `fixtures/builtins/v1/benchmark/*.toml`, which stay in the manifest and are still checked by `audit fixture-builtins`.

### Gates

1. **Build.** All pass:
   - `cargo check --locked --workspace --all-targets --all-features`;
   - `cargo clippy … -- -D warnings`;
   - `cargo fmt --all -- --check`;
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`.

   `cargo test --locked --release -p bench -p console-workload`: all pass. The `-- --list` diff, base `b8bea8e1` against the change, is 130 → 109 names. Exactly 21 are removed, all inside the deleted modules: `rack::tests::*` (3), `builtins::tests::*` (11) and `graph::tests::*` (7). Nothing was added or renamed.
2. **Console digests.** `gain_pan_profile -- --ignored --exact digests`: all 17 rows are byte-identical on base and change.
3. **Shipped artifact.** No `src/`, manifest or feature change under `crates/` or `hosts/`. The `crates/` diff is only three new integration-test files (gate 5), which are outside every shipped closure. The literal "none of `crates/`" proxy therefore does not hold; gate 5's porting option requires the owning crate's tests.
4. **CI routing and policy.** All pass:
   - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
   - `check-bench-policy.sh`, `test-bench-policy.sh`, `check-env-vocabulary.sh` (117 names) and `test-env-vocabulary.sh`;
   - `test-console-benchmark.sh`, `check-bench-preconditions.sh`, `check-realtime-policy.sh`, `test-realtime-policy.sh`, `check-effect-runtime-policy.sh` and `test-effect-runtime-policy.sh`;
   - `check-conformance-boundaries.sh` and `test-conformance-boundaries.sh`;
   - `check-test-support-ci.py` and `test-test-support-ci.py`.

   Every argument-free command of the lint job also ran on the final tree: 47 commands, including `cargo doc`, all green. `check-builtins-fixtures.sh . target/release/audit` passes (50 files).
5. **No live claim lost.** See below. `fixtures/builtins/v1` remains guarded by `check-builtins-fixtures.sh` and `audit builtins-graph`.

### Gate 5: the 21 deleted tests

The first group of tests asserted the retired runners' own record format, schedule or metadata. They had no product claim, so they were deleted with the runners and nothing replaces them:
- `builtins::issue035_record_has_exact_render_identity_shape_and_typed_missing_metadata`
- `builtins::issue035_preparation_record_uses_only_required_null_and_not_applicable_values`
- `builtins::measured_plan_is_the_frozen_twenty_row_cartesian_set`
- `builtins::matrix_targets_alternate_by_global_operation_across_batch_boundaries`
- `graph::metadata_command_projection_preserves_graph_policy`
- `graph::raw_metadata_projection_extracts_commands_and_tracks_environment_states`
- `graph::fixed_metadata_projection_preserves_record_fields`
- `rack::semantic_input_identity_is_layout_independent_and_workload_specific`

`builtins::benchmark_inputs_take_their_hashes_from_the_checked_manifest_rows` is in the same position. Its input files keep their manifest hashes in `check-builtins-fixtures.sh`, and the manifest's own hash is now pinned through the audit constant (Amendment 3).

Shared-helper tests are covered by `tools/bench-support` (which runs in test-debug-a):
- `rack::nearest_rank_uses_the_frozen_one_thousand_observation_indices` → `stats.rs:95 percentiles_cover_the_complete_tuple_over_one_thousand_observations`, the identical tuple.
- `graph::nearest_rank_is_ordered_at_all_frozen_percentiles` → `stats.rs:131 small_samples_round_up_to_the_next_rank` and `:114`.
- `graph::sha256_hex_matches_known_abc_digest` and `builtins::digest_helpers_encode_known_abc_without_rehashing_finalized_bytes` → `digest.rs:101 matches_the_published_vectors` and `:89`.
- `builtins::audited_allocator_is_live_outside_the_render_scope` → `alloc.rs:241 the_installed_allocator_is_the_audited_one` and `:251`.

Product claims that are named elsewhere:
- `builtins::checked_toml_and_referenced_pcm_drive_render_configuration_bit_exactly`. Identity-chain signed zero → `crates/builtins/tests/stage.rs:662` and the corpus pins in `determinism.rs:34`. The checked TOML and PCM semantics → `tools/audit/src/fixture_builtins.rs:2261` `verify_pcm_semantics`, run by `check-builtins-fixtures.sh`. Also ported, see below.
- `graph::scale_benchmark_fixture_prepares_and_compiles` (ignored) → `crates/graph-compiler/tests/scale.rs:160 compiles_and_binds_65_537_tracks_with_builtins` (not ignored; test-debug-a) and `:91` (`routes == 1`).

Ported to the owning crate (none of these existed at this scale or in this combination):

| deleted test | ported to | also still covered by |
|---|---|---|
| `rack::zero_launch_preflight_prepares_each_exact_production_workload` | `graph-compiler/tests/compile_shapes.rs` `mixed_rack_depths_bank_every_stage_and_leave_subsequences_and_sidechains_scalar`: the same mixed twelve-track session, at `Simd4` and `Simd8` explicitly (the original asserted a Simd8 host). It checks builtin banks `3 × ⌈12/W⌉` with per-stage sizes `[4,8]`/`[4,4,4]`, one two-slot cohort with `8/W` full bound groups, `2·8/W` bound slots equal to `effect_bank_count`, scalar = `{fallback10, fallback11, rack02, rack05}`, then bind and render | the scalar and bank chain rows: `builtins/tests/stage.rs:616`, `:210`; `graph-compiler` `lib.rs:4081`, `:4544`, `:6503`, `:14439` |
| `graph::canonical_benchmark_fixture_prepares_and_compiles` | `compile_shapes.rs` `representative_console_compiles_with_builtins_and_reports_its_shape`: the same 256/1,024/32/64/32 fixture and assertions | `bank_levels.rs:1088`; `graph_fixture.rs:413` for evidence bytes |
| `builtins::real_meter_tap_plans_use_the_compiled_seven_taps_and_preserve_full_queue_state` | `compile_shapes.rs` `seven_meter_taps_bind_in_tap_order_and_a_full_queue_drops_one_window` | `tools/audit/src/builtins_graph.rs:724`; `builtins-compiler` `lib.rs:12165` |
| `builtins::preparation_projection_is_complete_address_free_and_deterministic` | `builtins-compiler/tests/metered_preparation.rs` `metered_256_track_preparation_is_exact_and_repeatable`: the fixture's shape at 48 and 96 kHz. Counts `[768, 256, 56, 56]`, `meter_items == 280`, non-empty layouts, and an identical projection (counts, tails, full resource report) on a second prepare | `scale.rs:34`; `allocation_tracker.rs:1290` |
| `builtins::all_render_workloads_arm_only_product_render_without_timing` | the identity-chain row, which nothing else arms: `builtins/tests/identity_chain.rs` `identity_chain_renders_audit_clean_deterministic_and_positive_zero`. It checks an armed render scope with an empty `AuditSnapshot` (with a live-audit positive probe), two chains bit-equal, and output = input with `-0.0 → +0.0` | full-chain and matrix-ramp rows: `audit builtins` (`tools/audit/src/builtins.rs:38`, 1,000,000 blocks under strace). Meter success and full rows: `audit builtins-graph` (`builtins_graph.rs:158`, `:724`) |

The ports pass under the CI feature sets too: test-debug-a's `builtins-compiler/test-support,graph/test-support,engine/realtime-audit`, and test-debug-b's `builtins/test-support`.

Red mutations, each applied, run and reverted:
- `compile_shapes`:
  - `rack05` given the full chain: the builtin-bank count goes red, because the rack depths and levels move;
  - fallbacks without sidechains: the `Simd4` bound-group count goes red;
  - queue capacity 1 → 2: the drop count goes red;
  - sidechains on every 16th track: the sidechain count goes red.
- `metered_preparation`, capacity 4 → 3: 224 ≠ 280.
- `identity_chain`:
  - an allocation inside the armed scope: the audit snapshot goes red;
  - left trim 0.5 dB: the identity check goes red.
