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
