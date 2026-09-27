# Add a metered live-console row to the console benchmark

## Product outcome

The standing console benchmark (`scripts/run-console-benchmark.sh`, `tools/bench/src/console.rs`) measures 64 tracks with no meters and no automation. The product workload is a browser DAW with a meter on every track and a fader moving. Two of the largest optimisation candidates (meter kernel, route fold under observation) are invisible on the current rows. This issue adds one descriptive row that makes them measurable. Tooling only; no engine change.

## Root evidence

- `tools/bench/src/console.rs` module doc (lines ~60-110) lists the rows: `sixty_four_track_console`, `_eq_only`, `_compressor_only`, `_builtins_only`, `_dispatch_only`, `_idle`, plus `console_hoist` and `console_automation` (one Point span per block through the live console queue).
- Meters are bound at prepare through `prepare_selected_session_builtins_between_render_calls` (`crates/host-core/src/prepare.rs`, the `SelectedMeterRequest` construction near line 1110 builds one request per track when `meter_period_frames` is set; defaults near line 1117 set `peak_hold_frames: 0`).
- `GraphNodeObserverBinding::new` vs `::controlled` at `crates/builtins-compiler/src/lib.rs:3582-3590` select the permanent vs controlled policy.

## Smallest closable slice

Authorized paths: `tools/bench/src/console.rs`, `scripts/run-console-benchmark.sh`, `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-validator.jq`, `scripts/console-benchmark-record-lib.jq`, `scripts/check-console-benchmark-fixture.sh`, `scripts/test-console-benchmark.sh`, and this spec.

Add one row, `sixty_four_track_console_metered`: the `sixty_four_track_console` plan with a `SAMPLE_PEAK` meter selected at `PostMatrix` on every track (the same request shape the web host builds, `hosts/host-web/src/lib.rs` near line 7734), bound through the same host-core prepare entry point the web host uses, with the meter lease held and every published snapshot consumed each block so the observer path is exercised. Record `us_per_block` with the same metadata as the other rows. Extend the validators so the new record shape is accepted and any missing field is rejected. The row carries `bank_route_folds` and `bank_scatter_redirects` per arm, as "Carry fold and redirect counters in the console meters record" defines them for `console_meters`.

## Non-goals

No engine, host or effect change. No new automation arm (reuse `console_automation` if a moving fader is wanted later). No threshold; the row is descriptive.

## Objective gates

1. `scripts/test-console-benchmark.sh` passes with the new row and rejects a record missing it.
2. `scripts/check-console-benchmark-fixture.sh` passes.
3. One descriptive run: `sixty_four_track_console_metered` and `sixty_four_track_console` recorded back to back, attached as the artifact directory the script writes. Report the ratio; make no claim about it.
4. `scripts/check-bench-policy.sh` and `scripts/test-bench-policy.sh` pass.

## Dependencies

None. Unblocks measurement for "Bank-wide lane meter kernel for peak and count metrics" and "Keep the route fold eligible when the folded lane is observed".

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A means the change moves no rendered bit: same arithmetic in the same order, fewer passes, loads, stores, copies or branches. Every gate below that says "bit-identical" is a hard stop, not a tolerance.
- Render paths stay allocation-free, lock-free and syscall-free (`scripts/check-realtime-policy.sh` is mandatory). Only `crates/lane` may name `wide` or intrinsics (`scripts/check-lane-policy.sh`).
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`, and the focused tests named below before every checkpoint. Commit on a `codex/<issue>-<slug>` branch from synchronized `main`; do not touch paths outside the authorized list.
- Do not quote a projected saving. If a benchmark row is listed, run it exactly once, one warmup and two measured rounds, and attach the record as descriptive evidence.
- Source of these findings: `docs/audits/render-path-cost-audit-2026-09-24.md` (PR #879) and tracker #349.

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27, on `codex/881-metered-console-row` (base `64665d99`). Code
commit `0621d78c`. Tooling only: no engine, host or effect file is touched. The paths changed are
`tools/console-workload/src/lib.rs`, `tools/bench/src/console.rs`, `tools/bench/src/floor.rs`,
the four console benchmark scripts named in the brief, the runner's record count and the
preflight's `records_required`. All are authorized by the brief or by the amendment.

### Design

- **The row.** `Workload::SixtyFourTrackConsoleMetered`, kind `sixty_four_track_console_metered`,
  lives in `console-workload`. It is `sixty_four_track_console` as written: same fixture, strip
  (`AsWritten`), compile, frozen bound sources and graph, with `synthetic_fixture: false` and
  `source_feed: "bound"`. It is not in `WORKLOADS`, because the wasm arm addresses that array by
  index. It sits in a new `METERED_WORKLOADS` list, and `native_session_rows()` gives the emission
  order: `WORKLOADS`, then `DRIVER_FED_WORKLOADS`, then `METERED_WORKLOADS`. The bench and the
  floor tests use that function.
- **The meters.** `web_meter_requests` builds one `SelectedMeterRequest` per track of the compiled
  session's normalized model, the request `host_core` builds for the default web boot:
  - `MeterMetricSet::SAMPLE_PEAK` at `MeterTap::PostMatrix`;
  - handles `index + 1`;
  - `period_frames = 12 * 128`, from `WEB_METER_BLOCKS = 12`, which mirrors the web host's
    `DEFAULT_METER_BLOCKS`;
  - `peak_hold_frames = 0` and `peak_decay_db_per_second = 0.0`;
  - queue depth 8, as the web host's `console_request` sets it;
  - `reset_generation = 0`.

  `build_full` passes these requests to
  `builtins_compiler::prepare_selected_session_builtins_between_render_calls(&session, &requests, &[], caps)`.
  That entry is the one `host_core::prepare_host_runtime_with_selected_meters_between_render_calls`
  reaches, and it binds the meters as **permanent** observers (`MeterBindingPolicy::Permanent`).
  No control channel is attached, so the meters are the only thing this row adds. The row refuses
  any `PlanConfig` other than `BASELINE`. Every other row keeps `prepare_session_builtins`.
- **Timing.** The session timing loop allows the drain to sit outside the clock, and it does.
  `timing::timed` wraps `runtime.render` alone. After each timed block, and after `hash_output`,
  the row drains every snapshot from every stream with `SessionRuntime::drain_meter_snapshots`.
  The consumers stay held for the whole run. The drain is gated on the row, so every other row's
  loop is unchanged.
- **Record.** Every other row's `console_session` record keeps its old shape: the new group is
  spliced in as an empty string for them. The metered row adds eight keys. `meter_streams`,
  `meter_tap`, `meter_metrics` and `meter_window_blocks` are *observed*: the tap comes from the
  bound streams, and the metric set and window come from the consumed snapshots, reading `other`
  or `0` if they differ. The other four are `meter_snapshots`, `meter_dropped_snapshots` (the sum
  over streams of the last cumulative drop counter), `bank_route_folds` and
  `bank_scatter_redirects`. The two counters are read once at bind. The row has a single arm, and
  the aggregate ties its counters to the `console_meters` `meters_off` arm, which is the same
  session unmetered.
- **In-run assertion.** `METERED_PAIR`: the run asserts that the metered row's digest equals
  `sixty_four_track_console`'s before emitting either record, as the plumbing pair does.
- **Floor.** `floor_row` returns `None` for the row, and `floor_pins` holds
  `[null, 1, "none", "not_derived"]`. No ruling inventories a metered strip.
- **Validators.** The record library has these changes:
  - `metered_session_keys` and `metered_kinds` are new;
  - `session_row_keys` / `session_row_floor_keys` choose the exact key set by kind, so a standing
    row that carries the group fails, and so does the metered row without it;
  - `session_kind_shape` pins the standing console's six facts;
  - `metered_session_shape` pins `meter_streams == tracks`, `post_matrix`, `sample_peak`, window
    12, `meter_snapshots == streams * floor(observations / 12)` (5312 at 1000 observations),
    0 dropped, `bank_route_folds == tracks`, and non-negative integer counters.

  The aggregate expects 50 records (36 sessions), pins the metered and standing console digests
  equal across both rounds, and pins the metered row's folds and redirects to the `console_meters`
  `meters_off` values. The runner's line count is now 50, and so is the preflight's
  `records_required`.

### Gates

| Gate | Command | Result |
|---|---|---|
| Amendment 3: digest, cadence, none dropped | `cargo test -p console-workload --lib metered` | PASS (2 tests) |
| Record printed and pinned | `cargo test -p bench the_metered_console_row -- --nocapture` | PASS |
| 1: runner self-test | `bash scripts/test-console-benchmark.sh` | PASS |
| 2: fixture check | `bash scripts/check-console-benchmark-fixture.sh` | PASS |
| Record count | runner, preflight, aggregate validator, suite index map | 50 everywhere |
| Preflight | `bash scripts/operator/preflight-console-benchmark.sh --step preflight-881` | PASS at `0621d78c` |
| 4: bench policy | `scripts/check-bench-policy.sh`, `scripts/test-bench-policy.sh` | both PASS |
| Standing rules | fmt, clippy, doc, `cargo test -p bench -p console-workload`, `scripts/check-realtime-policy.sh` | all PASS |
| Existing rows unchanged | see below | unchanged |

What each gate shows:

- **Amendment 3, digest, cadence, none dropped.**
  `the_metered_console_row_renders_the_console_bits_and_publishes_every_window` renders both rows
  for 64 blocks.
  - The standing row's digest equals its existing pin in `tests/chain_shape.rs`,
    `fe5bed9becdbc101d7ad4b77e7e1969ca3888cae34857333f79531b03a4868de`, and the metered row's
    digest equals it.
  - Both rows have zero forbidden operations under the realtime audit, the same `[chains, slots]`
    and the same transposes. Both fold 64 routes, and both have the same redirects (0 on this
    host).
  - The metered row binds 64 `PostMatrix` streams and publishes 5 x 64 snapshots: 64 after each
    of blocks 11, 23, 35, 47 and 59, and none after any other block.
  - Each handle carries window sequences 0-4, contiguous 1536-frame spans from sample 0, the
    `SAMPLE_PEAK` presence mask, generations 0, 0 dropped, 0 discontinuities, and a positive
    finite peak on both channels.
  - The unmetered row binds no stream.
  - The window is pinned as the literal 12 x 128, not read back from the mirror constant.
  - The companion test `the_metered_row_states_the_console_rows_facts_and_is_the_only_web_metered_row`
    checks the stated facts, uniqueness and emission order.
- **Record printed and pinned.** `the_metered_console_row_prints_its_meters_and_the_validator_pins_them`
  runs the real `SessionMeasurement` for 36 timed blocks (three windows).
  - The printed short-run record is: `meter_streams 64`, `post_matrix`, `sample_peak`, window 12,
    `meter_snapshots 192`, dropped 0, `bank_route_folds 64`, `bank_scatter_redirects 0`, forbidden
    0.
  - Its digest equals the standing row's.
  - The standing row's record carries none of the eight keys.
  - The validator accepts the metered record once it is set to the frozen counts, and the standing
    record once it is set to 1000 observations. It rejects the record with each group key deleted,
    the group grafted onto the standing kind, window 4, `all` metrics, the `post_fader` tap, a lost
    snapshot, a dropped snapshot, and a declined fold.
- **Gate 1, runner self-test.** Prints `console benchmark validators: PASS (real
  runner/workload/timing invocations: 0/0/0)`.
  - New cases cover the full per-key deletion and null sweep of the metered record, plus the
    deletion half-sweep of its underived floor variant.
  - About 30 semantic mutations are refused, including: the group on the standing row, the kind
    without the group, a window of 4 with its consistent count, `other` metrics or tap, counts of
    5311, 5313 and 16000, dropped 1, folds 0 and 65, bad redirect types, and the metered row
    given the strip inventory, the strip's basis, or a control.
  - Aggregate: a set missing the metered row is **rejected**. Also rejected: either console-pair
    digest moved alone, and the redirects differing from the `meters_off` arm in either direction.
    Accepted: the pair moving together, and the redirects moving together.
- **Gate 2, fixture check.** `console fixture: ok`.
- **Preflight.** `bash scripts/operator/preflight-console-benchmark.sh --step preflight-881`
  passes at `0621d78c` with `records_required: 50` and `workload_launches: 0`. The `binary_sha256`
  is `c195a0c3...`. The script only checks that the artifact directory does not exist and creates
  nothing, so no `artifacts/steps/preflight-881` directory is left behind. It also ran once before
  the commit on the identical tree.
- **Standing rules.**
  - `cargo fmt --all --check`;
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
  - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
  - `cargo test --locked -p bench -p console-workload`: bench 65, console-workload lib 7,
    automation 4, chain_shape 25, placement 3, plumbing_profile 2 ignored as before;
  - `scripts/check-realtime-policy.sh`.
- **Existing rows unchanged.** A non-metered row's prepare path is byte-for-byte the old one:
  `web_meters` is empty, it still calls `prepare_session_builtins`, and the meter-count assertion
  adds zero. Its timing loop only gains a `None` branch outside the clock, and its record splice is
  empty. The pinned 64-block digests of `sixty_four_track_console`, `_gain_pan_only`,
  `_dispatch_only`, `_builtins_only` and `_plumbing_only` (`tests/chain_shape.rs`) and of the
  plumbing ring row still pass. The standing record validates on the unchanged `session_keys`.
  `drain_meters` now delegates to `drain_meter_snapshots` with the same count.

Red mutations run, each restored and confirmed with `cmp`:

- **Record library.**
  - Drop the window pin: the suite fails `a metered row at the facility arm window`.
  - Drop the metered kind from `session_kinds`: 5 aggregate cases fail.
  - Drop the count rule: 3 cases fail.
  - Stop applying `metered_session_shape`: 34 cases fail.
- **Aggregate validator.**
  - Drop the digest pin: 2 cases fail.
  - Drop the redirect agreement: 2 cases fail.
- **Rust.**
  - `SAMPLE_PEAK` changed to `ALL` in `web_meter_requests`: the console-workload presence assertion
    fails, and so does the bench's observed-shape assertion.
  - `WEB_METER_BLOCKS = 4`: the console-workload window count fails.
  - Stop draining in `run_for`: the bench's observed-shape assertion fails.

### Deviations and notes for the verifier

- **Entry point.** The brief body says "the same host-core prepare entry point". The amendment
  names `prepare_selected_session_builtins_between_render_calls`, and that function is called
  directly. `host_core` is not linked, for two reasons:
  - it would replace the frozen bound sources with PCM rings and plan id 1, so the digest could
    not be compared with `sixty_four_track_console`'s;
  - `scripts/check-conformance-boundaries.sh` pins the bench's dependency set.

  The request fields are transcribed from `host_core`'s selected-meter branch and the web host's
  `console_request`.
- **No control channels.** The default web boot also attaches control queues. They are left off
  deliberately, as the amendment specifies meters only, so the row isolates the observer path
  against `sixty_four_track_console`.
- **Gate 3 (timed run) not run.** The coordinator records it, per the amendment.
- **Scratch.** No new target directory was created beyond this worktree's own `target/` (2.2 GB).
  Scratch files live only in the session scratchpad.

## Sol attempt 1 verdict: FAIL

Reviewer: Sol, 2026-09-27, `git diff 64665d99..2f93ecb4` in the `engine-881` worktree. Re-run
here, all green: `cargo fmt --all --check`; workspace clippy `-D warnings`;
`RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`;
`cargo test --locked -p bench -p console-workload` (65; 7, 4, 25, 3, 2 ignored);
`scripts/test-console-benchmark.sh`; `scripts/check-console-benchmark-fixture.sh`;
`scripts/check-bench-policy.sh`; `scripts/test-bench-policy.sh`; realtime, conformance-boundary,
workspace and lane policies; and `preflight-console-benchmark.sh --step preflight-881-sol`
(PASS, `records_required: 50`, no artifact directory left). Two red mutations were re-run and both
went red: deleting the aggregate's console-pair digest pin (2 cases) and deleting the
`meter_dropped_snapshots == 0` pin (4 cases). No timed run was made.

The row is shaped like the product, and the gates discriminate. It fails on one false claim:
the implementation and its evidence say the claim holds, and that claim is the premise of the
gate 3 comparison.

### Findings, by severity

1. **Major: meters are not the only difference between the row and `sixty_four_track_console`.**
   - **The entry changes control delivery.** The amendment requires
     `prepare_selected_session_builtins_between_render_calls`. That entry selects
     `BuiltinControlDelivery::BetweenRenderCalls` (`crates/builtins-compiler/src/lib.rs:3301-3316`).
   - **Under that delivery, fader and matrix fuse.** The chain builder
     (`crates/graph/src/runtime.rs:4524`) and `make_fader_matrix`
     (`crates/builtins-compiler/src/lib.rs:1008-1040`) pair each cohort's fader bank and matrix bank
     into one `FaderMatrixBankProcessor`. That stage renders through
     `try_process_settled_with_matrix` (`:1079`).
   - **The standing row keeps them apart.** It is prepared through `Concurrent`
     `prepare_session_builtins`, so it keeps two stages, and its matrix runs through
     `MatrixStage::process`, which carries #944's select-free arm.
   - **Probe (temporary test, deleted).**
     `graph::test_only_bank_chain_construction_facts().runtime_slots` is **48** on the standing
     row and **40** on the metered row. `bank_shape` is `[8, 48]` on both, because it counts
     memberships and not runtime stages. Folds are 64 and redirects 0 on both.
   - **Consequence for the ratio.** Metered minus standing is the meter cost plus the difference
     between the fused and unfused fader/matrix. #943 F2 measured the unmetered web-shape plan
     about 2 us below the concurrent one. Any fader/matrix optimisation, #944 included, can also
     move one row of the pair and not the other.
   - **False statements to correct:**
     - `tools/console-workload/src/lib.rs:354-356`: "the meters are the only thing this row adds";
     - `:1171-1175`: "its between-render-calls delivery has nothing to deliver";
     - `:2523` and `:2581`: "the meters change no bank" / "no bank moves";
     - `tools/bench/src/console.rs:165-167`: "one session, rendered with and without the ... meter
       set";
     - this spec's Design, line 74.
   - **Invariant this change falsifies.** `tools/console-workload/tests/chain_shape.rs:915-916`
     (#944): "Every banked row prepares builtins through `Concurrent` delivery, so its fader and
     matrix are never paired".
   - **Required for attempt 2** (authorized paths only; no engine change):
     - State the delivery difference truthfully at each site above.
     - Correct the #944 sentence.
     - Pin the difference in the pair test: runtime stages 48 against 40 through
       `graph::test_only_bank_chain_construction_facts`, which console-workload's `graph`
       test-support dev-dependency already exposes. Replace "no bank moves" with the true
       statement: memberships are unchanged and each chain's fader and matrix are fused.
     - Keep the delivery, which is the product's.
   - **Coordinator's option.** An unmetered between-render-calls control row would isolate the
     meters. It is not required if the gate 3 ratio is labelled as meters plus delivery.

2. **Low: the aggregate compares the redirect counts of two different plans.**
   `scripts/console-benchmark-validator.jq` (the #881 block) pins the metered row's
   `bank_scatter_redirects` to `console_meters`' `meters_off` arm, which is a `Concurrent`,
   unfused plan. The comment says the two are the same session unmetered, and they are not. Both
   counts are 0 today. Restate the comment, or accept that a change which makes the counts depend
   on delivery will refuse a truthful record.

3. **Info: product parity of the observer path.** This is the answer to verification item 1. The
   two differences are no host-core link and no control channels, and neither changes the
   observer path. Nothing to fix.
   - **The product.** The default web boot is `compile_ready`'s `(None, None)` arm
     (`hosts/host-web/src/lib.rs:7794-7801`), which calls
     `prepare_host_runtime_with_selected_meters_between_render_calls` with no observation
     demand. It binds through `into_bound_with_source_set` without an activation
     (`crates/host-core/src/prepare.rs:1392-1406`).
   - **The row.** It binds through `into_bound`, also without an activation.
   - **The same render path.** In both, `selective_observation` is false and every meter runs
     through the permanent `observe_unit` (`crates/graph/src/lib.rs:2692-2708`).
     `MeterBindingPolicy::Permanent` is fixed by the entry (`builtins-compiler/src/lib.rs:3315`),
     and track controls do not affect it.
   - **So #943 moves the row and the product alike.** The product's control channels add per-block
     empty-queue drains outside the observer path, and the row omits them, as its notes say.
     Request fields match `host_core`'s selected-meter branch
     (`crates/host-core/src/prepare.rs:1097-1127`) and the web host's `console_request`
     (`hosts/host-web/src/lib.rs:8284-8308`): 12 x 128, hold 0, decay 0, depth 8, handles
     `index + 1` over `compiled.normalized_model()`.

4. **Info: items 2 to 6 verified.**
   - **Timing.** `timing::timed` wraps `runtime.render` alone. The drain follows `hash_output`,
     outside the clock, as in the `console_meters` arm. So the row times observation and window
     publication inside render and never times the host's drain.
   - **Digest and cadence.** The 64-block digest equals the `fe5bed9b...` pin
     (`chain_shape.rs:943`). 5 x 64 snapshots are published, on blocks 11, 23, 35, 47 and 59, with
     none dropped. The bench record gives 192 snapshots over 36 blocks, and the frozen record is
     pinned at 5312.
   - **Record counts.** The count is 50 in the runner, the preflight, the aggregate and the suite
     index map. The suite rejects a set missing the row.
   - **No drift.** Existing rows' prepare path, record shape, floor pins and digests are
     unchanged. `WORKLOADS`, and so the wasm arm, is untouched. The scope is within the amended
     paths, and no engine, host or effect file changes.
