PASS

# #1348 attempt 1: verdict (adversarial verifier, opus-xhigh)

Commit `244378b87` (parent `efad0080a`), branch `codex/d15-stream-b`. I reviewed it from an export
(`git archive 244378b87` to `/tmp/claude-1002/v1348/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1348/target`).
I did not build in the worktree and did not change it. I ignored the uncommitted #1312 work.

Result: no BLOCKER and no MAJOR findings. One MINOR finding is a spec-text problem for root. The
other findings are NITs.

## Findings

### MINOR

- **M-1 (spec text, for root): gate 3's red claim cannot be true as written.** The gate says
  "submit, then `COUNTERS_GET` ... *Red if the refresh stays in `command` only*". `COUNTERS_GET`
  is a `command`, and `command` runs `service()` first (`crates/control-plane/src/control.rs:1047-1048`),
  so the refresh happens before the reply. I tested this. I applied M3a (refresh in `command`
  only) and removed only the test's per-submit observer assertion
  (`crates/capi/src/runtime/tests.rs:3506`). The literal procedure, submit then `COUNTERS_GET`
  equal to the controller count, stayed **green**. With the observer it is red. So the
  implementer's deviation is correct and necessary: after each submit, the test reads the
  provider snapshot through `test_provider_counters`. That is the same `ControlProvider::counters`
  read that `COUNTERS_GET` makes, and it has no side effects (`crates/host-core/src/control_provider.rs:345`).
  The test also keeps the end-to-end `COUNTERS_GET` check. This deviation makes the gate stronger,
  not weaker. Root should change gate 3's text to name the read made after the submit. No change to
  the implementation is needed.

### NIT

- **N-1 (gate 4 location).** The spec says "control-plane test, `test-support`". The test is in
  `crates/capi/src/runtime/tests.rs:3556`. It uses the new control-plane `test-support` hook
  `test_set_next_adoption` (`control.rs:887-894`, which `command` reads at `control.rs:1157-1162`).
  This is acceptable. `control-plane` has no adapter of its own. Its
  `cargo test --features test-support` runs 0 tests and only proves that the feature compiles.
  Also, only the C ABI render entry publishes the `render_sequence` observation that the
  early-promotion defect depends on. Root can change the spec text if it wants to.
- **N-2 (gate 5's unique catch is limited).** The idle-state assertions catch M5 (stage the last
  observation again), but the existing `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes`
  test also catches M5. I confirmed this. The unique catch among the cargo tests is that the new
  export refuses a null handle (M6) or a wrong handle. That gives the test a valid test-value
  sentence. The implementer declared this.
- **N-3 (first refresh has no observable effect today).** D1 puts a counter refresh before
  `collect_render_activity` (`control.rs:1028`). I removed it, and all 94 capi tests stayed green.
  Today `collect_render_activity` stages `value: sequence` for every counter and does not read the
  provider's counters (`control.rs:665-681`). D1 requires this refresh for #1351 D3. Note for the
  #1351 verifier: #1351's snapshot test must go red when this first refresh is removed.
- **N-4 (header wording).** `miso_engine_v1.h:89-91` and the qualification doc say that every other
  session call "runs the same step first". A call refused at argument validation returns before it
  reaches `SessionState` and does not service. Examples: a null `source_id`, a bad struct size, an
  invalid lane or an empty request (`ffi.rs:436-468`, `:690-705`, `:782-794`). D2 holds at the
  `SessionState` level, which is where the spec places it. The header could say "every session
  call that is not refused for its arguments". This is optional.
- **N-5 (term not yet defined).** The duty text uses "warm successor that misses its deadline"
  (`miso_engine_v1.h:98-99`, `C_ABI_V1_QUALIFICATION.md:174-175`), but the header does not define
  the term. The deadline check ships in #1358 and #1360, not here. D5 requires this sentence, so
  this is not an implementer defect. #1360 can define the term when it lands.
- **N-6 (untested INTERNAL path).** No test covers service's `INTERNAL` + `capi.source.epoch`
  path (`ffi.rs:660-667`), because no fault hook reaches a `synchronize_plan_epochs` failure. The
  mapping reuses `SourceFailure::Internal.report()`, and
  `every_failure_kind_reports_its_frozen_c_abi_bytes` pins that function's bytes. No spec gate
  requires a test.

## Scrutiny items

- **D1 order.** `service()` (`control.rs:1026-1032`) runs `synchronize_plan_epochs`, then refresh,
  then `collect_render_activity`, then refresh. This is the order that D1 states. The first refresh
  serves #1351 D3, so that the staged snapshot reads refreshed values (see N-3). The second refresh
  makes sure a record that this same call coalesced or dropped is counted (M3b makes it red).
- **D2.** `command` (`:1048`), `dequeue_event` (`:1630`), `submit` (`:1738`), `seek` (`:1752`) and
  `seek_at` (`:1768`) each call `service()` once, as their first statement. The removed lines were
  the same leading calls. No other callers of `synchronize_plan_epochs`, `collect_render_activity`
  or `refresh_provider_counters` exist (I searched the repo). `commit_live` does not change. The
  order of the fallible checks in `command` does not change: before, the only fallible step ahead
  of `prepare_command_frame` was the synchronization, and that is still true. `dequeue_event` keeps
  its error mapping. The source calls keep `SourceFailure::Internal`. `service()` can fail only
  through `synchronize_plan_epochs`, and every error there is `Internal`.
- **D3 bound.** A publication (on the control thread) reserves each retirement credit, and
  `service` never publishes, so one call can reclaim at most the capacity's worth of plans. The
  reclaim loop cannot follow a render that keeps running. `collect_render_activity` compares one
  sequence word and stages at most one observation. Nothing waits. The `reports` mutex is the same
  lock that every earlier control call already took, and render never takes it.
- **D6 and D7, acked-batch question: can an ack ever precede a drop? No.** `service` makes no
  `withdraw` or `reserve_replacement` call and no mailbox call. It writes no response and adds no
  queue. It frees only plans that render already displaced. `synchronize_plan_epochs` does not
  change. It promotes a provider only on `active_epoch` or when a reclaimed plan's epoch equals the
  current provider's, and render produces both only after it adopts. Gate 4 is the evidence: an
  acked scheduled candidate stays through 1,000 calls, and render adopts it at exactly
  `64 * 128`. Lossy meter staging can drop records, and that is documented, counted, noncritical
  telemetry.
- **D4 C ABI.** The export is `uint32_t miso_engine_v1_service(miso_engine_v1_session *)`
  (`ffi.rs:647`, header `:370`). It uses `session_kind`, so a null handle returns
  `INVALID_ARGUMENT` and a wrong handle returns `WRONG_HANDLE`. On success it clears `last_error`,
  like the other session calls. On failure it returns `INTERNAL` and sets `capi.source.epoch` through
  `SourceFailure::Internal.report()`. `FEATURE_SERVICE = 1 << 7`, and the mask is `0xff`. On `main`
  the bits stop at `1 << 5`. Only this branch uses bits 6 and 7, and I checked every `d15` branch.
  The header changes add the define, the mask, the prototype, the session thread list (`:18-22`)
  and the "Control service" paragraph. The frozen list in `scripts/check-capi-abi.sh` now has 17
  symbols. The qualification doc says 17. The naming is correct: the boundary symbol has `_v1`, and
  the Rust names `service`, `FEATURE_SERVICE` and `refresh_provider_counters` have no version. The
  Rust tests and `abi_smoke.c` use the bit by its symbol. The mask pins (`0xff` and `255`) follow
  the existing ABI pattern.
- **D5 duty text.** Both the header and the qualification doc have it, with all the parts the spec
  requires: progress only on control calls, the same duty as draining events, retired plans kept
  allocated, a warm-successor fallback only at the next call, the watermark, and at least one call
  per render-buffer period.
- **Deleted hook `test_synchronize_plan_epochs`.** The commit correctly deletes it because
  `service()` replaces it. Its two call sites
  (`structural_command_keeps_protocol_plan_provider_and_event_epochs_atomic`) now call `service()`
  and stay green.
- **Authorized paths.** Every changed file is in the spec's list.
- **Realtime.** The render path does not change. `audit capi` reports 0 allocations, 0
  deallocations, 0 syscalls and 0 violations.

## Mutation runs (each one applied alone in the export, then restored; `cmp` against a fresh archive confirmed the restore)

| Mutation | Red | Existing tests red |
|---|---|---|
| M1: the export returns `OK` without running the step | gate 1, gate 4, gate 5 tests | none |
| M2: `submit` runs only `synchronize_plan_epochs` | gate 2, gate 3 tests | none (lib, `plan_swap_race`, `resource_lifecycle` all green) |
| M3a: the refresh is in `command` only | gate 3 test only (`tests.rs:3506`) | none |
| M3a with the literal gate 3 only (observer assertion removed) | **nothing** (the gate 3 test is green) | proves M-1 |
| M3b: the second refresh is removed | gate 3 test only | none |
| M4b: promote the pending provider after any block rendered since the last observation | gate 4 test only (`tests.rs:3582`) | none |
| M5 (my version): stage the last observation again on every call | gate 3, gate 5 tests | `all_six_event_families_cross_c_dequeue_with_exact_oracle_bytes` (as declared) |
| M6: the export returns `OK` for a null session | gate 5 test; `check-capi-abi.sh` exits with 6 (abi-smoke) | none |
| First refresh removed (N-3) | nothing | none |

## Test-value sentences

- `service_alone_reclaims_the_retired_plan_and_promotes_its_successor` (gate 1): red if the export
  does not run the step, so it neither reclaims nor promotes (M1). No other test reaches the step
  without a command or dequeue call.
- `a_source_submit_services_the_session_first` (gate 2): red if a source submit skips the telemetry
  part of the step, which was the shape before #1348 (M2).
- `every_control_call_refreshes_the_provider_counters` (gate 3): red if the counter refresh stays
  in `command` only (M3a), or if the step refreshes only before it stages (M3b). The provider
  snapshot that #1312 and #1351 read would then be stale.
- `a_scheduled_candidate_survives_service_until_render_adopts_it` (gate 4): red if
  `synchronize_plan_epochs` promotes or disposes of a scheduled candidate that render has not
  adopted (M4b). All other reclaim tests publish `Next` and miss this.
- `service_with_nothing_pending_changes_nothing` (gate 5): red if the new export accepts a null
  session (M6) or a wrong handle. Its idle assertions also catch M5, as an existing test does.

## Gates run (in the export)

- `cargo test --locked -p capi`: ok (81 lib, 2 `plan_swap_race`, 11 `resource_lifecycle`).
- `cargo test --locked -p control-plane --features test-support`: ok (0 tests; the feature compiles).
- `bash scripts/check-capi-abi.sh`: ok (shared and static). `--self-test`: ok.
- `cargo build --locked --release -p audit -p capi && ./target/release/audit capi`: 0
  allocations, 0 deallocations, 0 locks, 0 syscalls, `total_violations` 0.
- `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-realtime-policy.sh`: ok.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: ok.
- `cargo fmt --all -- --check`: ok.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps --keep-going`: only the three
  known failures (`spsc.rs:521` `Self::admits`, `watermark.rs:85` `MAXIMUM_READ_ATTEMPTS`,
  capi `runtime/mod.rs:148` `limits_are_valid`). No new failure. `control-plane --features
  test-support` documents with no warnings.
- Not run: `scripts/check-cross-targets.sh`. It is not a spec gate, the change has no
  target-specific code, and free disk was at 29 GB, near the 25 GB stop line. AArch64 runs in CI.
