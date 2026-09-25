# Carry fold and redirect counters in the console meters record

## Product outcome

The pure-path measurement (PR #913) could not say whether #885's route fold fired on the
`meters_on` arm of `console_meters`, because the record carries timings and digests but no
shape counter. Make both arms of the record state `bank_route_folds` and
`bank_scatter_redirects`, pin `meters_on` equal to `meters_off` for both (the #885/#886
contract at console level), and reject a record without them. Tooling only; no engine change.

## Root evidence

- `tools/bench/src/console.rs:828` `FacilityMeasurement::meters_record` emits the
  `console_meters` JSON (`:839-856`) from `FacilityMeasurement` (`:737-748`), which holds
  timings, digests, `meter_frames` and observation counts but not the arms' plan counters;
  `run` (`:749`) builds the two `SessionRuntime` arms from `METER_CONFIGS` (`:704-710`) and
  drops them after timing.
- `tools/console-workload/src/lib.rs:1323` `SessionRuntime::bank_route_folds` exists;
  there is no `bank_scatter_redirects` accessor, though the plan exposes one
  (`crates/engine/src/realtime/plan.rs:792`, beside `bank_route_folds` at `:804`).
- `scripts/console-benchmark-record-lib.jq:24` `meters_keys` is the exact sorted key list the
  validator (`meters_record_valid`, `:407-430`) requires; `scripts/test-console-benchmark.sh:91`
  builds the fixture record that must validate, and `:599-601`, `:685`, `:831` are the reject
  cases.
- `tools/console-workload/tests/chain_shape.rs:903` `the_folded_master_is_the_reductions_own_bits`
  already asserts, at test level, that the metered candidate folds exactly as many routes as the
  unmetered plan.

## Smallest closable slice

Authorized paths: `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` (the one
accessor), `scripts/console-benchmark-record-lib.jq`, `scripts/console-benchmark-record-validator.jq`,
`scripts/console-benchmark-validator.jq`, `scripts/test-console-benchmark.sh`,
`scripts/run-console-benchmark.sh` (only if a comment enumerates the record's fields), and this
spec.

1. `SessionRuntime::bank_scatter_redirects(&self) -> u64` delegating to the plan.
2. `FacilityMeasurement` records, per arm and read once after the warm-up renders, the two
   counters; `meters_record` emits `meters_off_bank_route_folds`, `meters_on_bank_route_folds`,
   `meters_off_bank_scatter_redirects`, `meters_on_bank_scatter_redirects`.
3. `meters_keys` gains the four keys; `meters_record_valid` requires all four to be
   non-negative integers, `meters_on_bank_route_folds == meters_off_bank_route_folds`,
   `meters_on_bank_scatter_redirects == meters_off_bank_scatter_redirects`, and
   `meters_off_bank_route_folds == .tracks` (every route of the 64-track console folds, which
   `every_standing_workload_folds_one_route_per_track` pins in code). The fixture in
   `test-console-benchmark.sh` carries the four fields; add reject cases for a missing key and
   for unequal fold counts.

## Non-goals

No engine change, no new row, no threshold on any timing. The metered live-console row of #881
is a separate open issue; when it is implemented it must carry the same four counters per arm
(recorded in `PLAN.md` as an amendment to #881).

## Objective gates

1. `scripts/test-console-benchmark.sh` passes with the new fixture and rejects the two new
   negative cases.
2. `scripts/check-bench-policy.sh`, `scripts/test-bench-policy.sh`,
   `scripts/check-console-benchmark-fixture.sh` pass; `cargo test -p console-workload -p bench`.
3. One descriptive `console_meters` record from a preflight-only or short run is attached showing
   the four fields populated (the counters are what matter; timings are not to be read).

## Console benchmark rows

None move. This is what lets the cycle's paired benchmark be read honestly on the metered pair.

## Dependencies

None. Merge first in the cycle so the batch-boundary benchmark records carry the counters.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit; this issue touches no render code at all, and `scripts/check-realtime-policy.sh` must still pass.
- The owner's copy rule applies to the engine, not to this tooling issue.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests and scripts named above before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. Do not run the full console benchmark for this issue.
- Source of these findings: PR #913 (`codex/pure-path-measurement`) and
  `docs/audits/render-path-cost-audit-2026-09-24.md` O1 (PR #879).

## What the implementer will hit

- `scripts/console-benchmark-record-validator.jq` and `console-benchmark-validator.jq` may
  restate key lists or counts for the aggregate; keep them consistent with `record-lib.jq`.
- The `issue` field of the record is the constant `ISSUE` (`console.rs:134`); leave it.
- Every existing accepted record under `artifacts/` lacks the new keys and is historical
  evidence; the validator is applied to new records only, so do not rewrite artifacts.

## Attempt 1 evidence

Implementation commit: `569ba338` (Terra, attempt 1), on `codex/914-meters-fold-counters` from
`12b621f2`. Tooling only: no file under `crates/` changed.

**Design.** `SessionRuntime::bank_scatter_redirects` delegates to the plan beside
`bank_route_folds` (`tools/console-workload/src/lib.rs`). `FacilityMeasurement` gains per-arm
`bank_route_folds` and `bank_scatter_redirects` vectors, read once per arm right after the 64
warm-up renders and before the audit is armed, so outside every clock; `meters_record` emits
`meters_off_bank_route_folds`, `meters_on_bank_route_folds`, `meters_off_bank_scatter_redirects`
and `meters_on_bank_scatter_redirects` from index 0 (`PlanConfig::BASELINE`) and index 1
(`meters: true`) of `METER_CONFIGS`. The counters are recorded, not asserted in-run: an arm that
stopped folding is a finding the raw record must keep (the runner preserves raw bytes on a
validation failure) and the validator must refuse, whereas an in-run panic would lose the record
and every row after it. `meters_keys` gains the four keys; `meters_record_valid` requires all four
to be non-negative integers, `meters_off_bank_route_folds == .tracks` (which `.tracks == 64`
already pins), and on == off for both counters.

**Deviation (inside `console.rs`).** To produce the gate-3 record without running the benchmark,
`FacilityMeasurement::run` now delegates to `run_for(configs, observations)`, and both facility
records emit the stored `observations` rather than the `OBSERVATIONS` constant. Every run the
runner launches passes `OBSERVATIONS` (1000), so production records are unchanged; a shortened run
says so in its record, and `common_shape` refuses it (`.observations == 1000`).

**Not changed, and why.** `console-benchmark-record-validator.jq` only includes the library.
`console-benchmark-validator.jq` restates record counts (still two `console_meters` records,
46 in total) and digest consistency, never the meters key list, so it is consistent as is.
`run-console-benchmark.sh` has no comment enumerating the meters record's fields. No artifact
under `artifacts/` was touched.

**Observed values.** Both arms fold 64 routes and redirect 0 scatters. Zero redirects is what the
graph runtime's own filter implies: a folded run's lanes are excluded from the redirect count
(`crates/graph/src/runtime.rs`, the `folded_runs` filter under the comment "A folded chain is
excluded"), and every route of this console folds. The validator pins the redirect pair equal, not
to zero, as the brief specifies.

**Tests added.**

- `tools/bench/src/console.rs`: `console::tests::the_meters_record_carries_each_arms_fold_and_redirect_counters`
  -- runs `FacilityMeasurement::run_for(&METER_CONFIGS, 8)`, asserts both arms fold `tracks`
  routes and agree on redirects, then asserts each of the four keys appears exactly once in the
  emitted record carrying its arm's value, and that the record states `"observations":8`.
- `scripts/test-console-benchmark.sh`: the meters fixture carries the four fields (64, 64, 0, 0),
  so the per-key structural sweep now deletes and nulls each of them; new named rejects: "a meters
  record without its fold and redirect counters (the shape before #914)", "a meters record missing
  the metered arm fold count", "a metered arm whose route fold declined", "a metered arm folding
  more routes than the unmetered arm", "a metered arm whose scatter redirects differ from the
  unmetered arm", "two arms that agree but do not fold every route of the console", "a negative
  redirect count", "a fractional redirect count", "a redirect count written as a string"; new
  accept: "a meters pair whose arms redirect the same lanes".

**Gates** (all run in the worktree at `569ba338`).

| gate | result |
|---|---|
| `bash scripts/test-console-benchmark.sh` | `console benchmark validators: PASS (real runner/workload/timing invocations: 0/0/0)` |
| `bash scripts/check-bench-policy.sh` | `bench policy: ok (1 allocator, 1 escaper, 1 percentile, 1 digest sink, 6 unsafe owners, 3 subjects on the shared timer)` |
| `bash scripts/test-bench-policy.sh` | `... bench policy mutations: ok` |
| `bash scripts/check-console-benchmark-fixture.sh` | `console fixture: ok (64 tracks, 8 full banks, EQ + compressor strip, 13 distinct trims)` |
| `bash scripts/check-realtime-policy.sh` | `realtime policy: ok (50 marked regions in 14 files)` |
| `cargo test -p bench -p console-workload` | bench 62 passed; console-workload automation 4, chain_shape 22, placement 3 passed; 0 failed |
| `cargo clippy -p bench -p console-workload --all-targets --all-features -- -D warnings` | clean |
| `cargo fmt --all --check` | clean |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |

`scripts/run-console-benchmark.sh` was not run (coordinator instruction; the batch boundary
benchmarks once).

**Mutations run** (each applied alone, suite or test observed red, source restored and verified
byte-identical with `cmp`).

| mutation | red result |
|---|---|
| validator: drop the `all(nonnegative_integer)` clause | 3 failed: negative, fractional, string redirect counts accepted |
| validator: drop `.meters_off_bank_route_folds == .tracks` | 1 failed: "two arms that agree but do not fold every route" accepted |
| validator: drop the fold on == off clause | 2 failed: declined fold and over-fold accepted |
| validator: drop the redirect on == off clause | 1 failed: unequal redirects accepted |
| validator: drop two of the four keys from `meters_keys` | 4 failed: base record, redirect accept case and both 46-record aggregates rejected |
| validator: fold on == off weakened to on <= off | 1 failed: declined fold accepted |
| Rust: emit `bank_scatter_redirects[1]` as `meters_on_bank_route_folds` | test panics "meters_on_bank_route_folds must carry 64" |
| Rust: misspell the `meters_on_bank_route_folds` key | test panics "must appear exactly once" |
| Rust: read `bank_scatter_redirects` into `bank_route_folds` | test panics on the fold assertion |
| Rust: emit `OBSERVATIONS` instead of `self.observations` | test panics "a shortened run must say it was shortened" |

**Gate 3: descriptive record.** Printed by
`cargo test -p bench the_meters_record -- --nocapture` at `569ba338`: the real subject (both
`SessionRuntime` arms, 64 warm-up blocks, the counter read), 8 timed observations, **debug
profile**, no runner metadata. The four counters are the point; the timings are debug-build numbers
and are not to be read. Its key set equals `meters_keys` exactly; as emitted the record validator
rejects it (8 observations, null metadata), and with only `observations` set to 1000 and the
suite fixture's eleven metadata fields substituted it is accepted, so the emitted shape and the
validator agree.

```json
{"schema_version":1,"issue":149,"record":"console_meters","workload_kind":"sixty_four_track_console","tracks":64,"round":1,"backend":"Simd8","observations":8,"pairing":"alternating_per_observation","arms":["meters_off","meters_on"],"meter_streams":64,"meter_tap":"post_matrix","meter_window_blocks":4,"meter_frames_drained":128,"units":"ns_per_block","percentile_method":"nearest_rank","meters_off_p50_ns":23685797,"meters_off_p95_ns":24056402,"meters_off_p99_ns":24056402,"meters_on_p50_ns":24465401,"meters_on_p95_ns":24523560,"meters_on_p99_ns":24523560,"paired_delta_median_ns":761620,"meters_off_output_sha256":"d1971a3639cc2e9400c372e6f750b85307874fde2182d7d86963762658d47e9f","meters_on_output_sha256":"d1971a3639cc2e9400c372e6f750b85307874fde2182d7d86963762658d47e9f","bit_identity":"meters_off == meters_on, asserted in-run","meters_off_bank_route_folds":64,"meters_on_bank_route_folds":64,"meters_off_bank_scatter_redirects":0,"meters_on_bank_scatter_redirects":0,"render_errors":0,"render_total_forbidden_operations":0,"cpu_model":null,"os":"linux","governor_or_power_mode":null,"rust_version":null,"llvm_version":null,"target_triple":null,"target_features":null,"profile":null,"background_load_note":null,"measurement_control":null,"cpu_affinity":null,"candidate_commit":null,"missing_metadata":["background_load_note","candidate_commit","cpu_affinity","cpu_model","governor_or_power_mode","llvm_version","measurement_control","profile","rust_version","target_features","target_triple"],"descriptive_only":true,"statistical_method":"two arms alternated per observation; nearest-rank percentiles over per-block nanoseconds; paired delta is meters_on minus meters_off per observation; descriptive only; no threshold"}
```
