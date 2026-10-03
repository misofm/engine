# #1227 attempt 1 verdict: Add a bus-and-send row to the native console benchmark

**Verdict: PASS.** No BLOCKER and no MAJOR. The fixture is a production-shaped bus-and-send
session, and the row is the C ABI's fan-playback preparation. Its plan facts are derived. The
counts and positional mutations move correctly from 60 to 62. No timed step ran. Every spec gate
re-ran green, and each new test has a unique, plausible catch. The findings below are two MINOR
documentation/test-value precision items and three NITs.

- **Implementation:** `e7bd95f88` on parent `f4cb3dc34`, branch `codex/batch-bench`.
- **Review copies:** a `git archive` export of `e7bd95f88` at `/tmp/claude-1002/v1227/src`, made a
  throwaway git repo so the preflight's `git rev-parse HEAD` works, with
  `CARGO_TARGET_DIR=/tmp/claude-1002/v1227/target`. A second copy at `mut` held the jq and Python
  mutations. Both were deleted after review. The worktree, branch and GitHub were not touched.
  `wt-bench` shows only the #1228 implementer's modified files, and it has no
  `artifacts/steps/*bus*`.
- **Host:** x86-64-v3 AVX2, 32 cores.

## Gates re-run on `e7bd95f88` (all exit 0)

| Gate | Result |
|---|---|
| 1. `cargo build --locked --release -p audit -p bench -p capi -p session-validator` | ok |
| 1. `check-console-fixtures.sh target/release/session_validator` | ok ("intended/mono/app/sends witnesses") |
| 1. `cargo test --locked -p session-validator` | ok. The glob-based five-stage corpus test picks up the new fixture. |
| 2. `cargo test --locked --release -p audit -p bench -p console-workload` | 113 passed, 0 failed (matches the record); includes floor parity, both new console-workload tests, the bench short-run test, `every_standing_workload_folds_one_route_per_track` and the strip-row tests |
| 2. `test-console-benchmark.sh` | PASS (real runner/workload/timing invocations 0/0/0) |
| 2. `preflight-console-benchmark.sh --step bus-send-preflight` | PASS, `workload_launches: 0`, `records_required: 62`, `sends_fixture_sha256 22904bc6…4712`, `sends_fixture_generator_sha256 faef2d6c…b7` (both equal the record's); `artifacts/steps/` count 25 before and after, no `bus-send-preflight/` |
| 4. `check-bench-policy`, `test-bench-policy`, `check-conformance-boundaries`, `check-workspace-policy`, `test-workspace-policy` | ok |
| 4. `cargo fmt --all -- --check`, workspace clippy `--all-targets --all-features -D warnings` | clean |
| extra: `check-ci-path-routing.py`; `ci-path-router.py` over this diff | contract passed; the diff routes `full` |

The preflight also refuses overwrites. With `artifacts/steps/ow-probe/console-benchmark.raw.jsonl`
present, `--step ow-probe` exits 1 in 5 ms ("console artifact already exists"), before any tool,
cargo or binary runs. `--step Bad/Name` exits 2.

## What holds up

- **The fixture represents production, and the follow rule holds.** I parsed the committed fixture
  and compared it with the intended fixture. Only `session_id`, `submixes` and `routes` differ.
  The 64 `chNN-main` routes differ in `destination` (into `bus-<NN div 8>`) and in `follows_mute`
  only. The route census is:
  - 64 `track/post_pan -> submix_input`, `follows_mute: true`, 0 dB;
  - 64 `track/pre_fader -> fx-a`, true, -12 dB;
  - 64 `track/post_fader -> fx-b`, true, -18 dB;
  - 10 `submix/post_pan -> output_input`, `follows_mute: false`, 0 dB.

  No route is muted.

  **The claim that the "64 main routes carry `follows_mute: true`" is correct and legal.** Those
  routes now go into a submix, and `validate.rs:708` refuses the flag only on an
  `OutputInput` destination. No route into `main-out` carries it. This is also the SDK's own
  default (`session.ts`: `followsMute ?? destination.kind === "submix_input"`), so main routes into
  buses carry it in production too. From a `post_pan` tap the follow changes no sample (schema
  doc).

  Each of the ten submixes has:
  - `Submix::unity`'s input section (checked against `model.rs:713`);
  - all three console slots, active, with `ch00`'s params;
  - one insert: the `maximum`-linked compressor on each bus, the delay on `fx-a`, the EQ on `fx-b`;
  - an unmuted 0 dB fader and the identity matrix.

  There is no VCA and no automation.
- **The row is the real C ABI path.** `capi::prepare_runtime` calls host-core's
  `prepare_host_runtime`, which takes the default `HostLiveControlRequest`: no meters and no
  control requests. That means `prepare_session_builtins_with_live_controls(.., &[], &[], ..)`,
  then `compile_with_builtins`, and no route live lanes (`attach_route_live_controls` runs only
  with a control queue). That is the `PlanConfig::BASELINE` shape of `build_full`.

  **Corroboration (scratch test, not committed).** I prepared the committed fixture through
  host-core's `prepare_host_session` with generous caps:
  - sends: `submixes=10 routes=202 latency=972 delay_bytes=0 folds=0`;
  - standing: `routes=64 latency=486 delay_bytes=0 folds=64`.

  The bench row's printed `bank_route_folds=0` matches the shipped pipeline exactly.
- **It differs from `sixty_four_track_console` only by the buses and sends.** The tracks, the
  console and the sources are equal (held by a witness). `Strip::AsWritten`, `Tone`, `Bound`,
  warmup 0, `BASELINE`, no web meters and no collapse override are all wildcards shared with the
  standing row. `console_model` gives the row its own arm.

  The ignored `gain_pan_profile::digests` harness, run over every row, shows that the sends row
  renders its own bits (`437a9d3c…` against the standing row's `fe5bed9b…`) and that the probes
  move no bit.
- **The plan facts are derived, not hand-typed.** D6 compiles through `build_full`'s calls. Each
  count is computed from D1's 64 tracks, 8 buses and 2 returns, with a derivation comment:
  202 transforms, 11 reductions and 202 timing rows with no compensation. It asserts no
  `route-mute` and no `route-follow-zeroed` row. The test does not copy the probe's 486/972
  arrivals, and it prints folds without asserting them.

  The field indices match `canonical.rs`: `route-timing` field 3 is the compensation, and
  `reduction` field 1 is the node. Printed:
  `route_transforms=202 reduction_nodes=11 bank_route_folds=0 route_ops_per_block=202 delayed_route_edges=0`.
- **The positional and count pins are right.** I dumped the suite's 62-record aggregate:
  - Round one: 0-22 sessions, with the sends row at 17. Then 23-24 hoists, 25 meters,
    26 observation, 27 placement, 28 automation, 29 mono and 30 mixing.
  - Round two: 31-61.

  `.[31]` is round two's first session. `.[56]`, `.[57]`, `.[58]`, `.[59]`, `.[60]` and `.[61]`
  are round two's meters, observation, placement, automation, mono and mixing records, each the
  record its case names. These counts are all in place:
  - `length == 62`, 46 `console_session` records and 62 unique triples;
  - `wc -l == 62` and `records_required: 62`;
  - every renamed label.

  A grep finds no stale 60, 44 or "twenty-two" count.
- **The derive script is deterministic, and its witnesses bite.** Two runs are byte-identical to
  each other and to the committed fixture. The script asserts the parent's shape before editing
  and canonicalises through the validator.
- **No ceremony.** The preflight's sha256 fields are run provenance, on the standing and mono
  precedent, not a committed pin. The `cmp` is a derived-fixture regeneration check for a document
  the browser arm will boot (BM2). The record `contains` checks read wire JSON, not source. No
  test greps source or prose.
- **Scope.** Every touched path is on the authorized list. The browser part of the mutation suite
  is untouched. `WORKLOADS`, `DRIVER_FED_WORKLOADS` and the fold law test are unchanged. The
  strip-row tests are unedited.

## Mutations I ran (each restored afterward)

| # | Mutation | Result |
|---|---|---|
| J1 | Record lib: replace the sends branch's `.fixture_id == sends_console_fixture` with `true` | **red**: `test-console-benchmark.sh` fails "a bus-and-send row naming the standing fixture" and "console_session with fixture_id nulled" (the `session_sends` base in the per-key loop) |
| P1 | Derive script: drop `main_route["follows_mute"] = True`, regenerate the fixture (the validator accepts it), run `check-console-fixtures.sh` | **red** on the `follows_mute`-exactly-into-submixes witness |
| R1 | Fixture: `fx-b`'s insert becomes `miso.true-peak-limiter` (latency) | **red**: the D6 test finds `delayed_route_edges` 9 (8 bus returns and `fx-a-main` compensated 486) |
| R2 | Fixture: mute `bus-3`'s fader (both lanes) | **green** on D6 (see MINOR-1); caught only by the fixture witness |
| M3 | Chain `BUS_SEND_WORKLOADS` after the strip rows | red on the new stated-facts test **and** on the existing `the_console_strip_rows_state_their_facts_and_are_emitted_last` |
| M4 | Drop the row's `fixture_id` arm | red on the new stated-facts test **and** on the bench short-run test |
| M5 | Attach `Strip::LimiterRemoved` to the row in `strip()` | red on all three new tests: the console declaration loses `limiter`, so the submixes name an undeclared slot (`MissingEntityReference`) |
| M6 | Attach `Strip::HalfMono` to the row in `strip()` | red **only** on the new stated-facts test: the bench record and D6 stay green |

## Test value (AGENTS.md: one sentence each)

- **`the_bus_send_rows_plan_carries_every_route_unmuted_and_uncompensated`** turns red if the row
  compiles a fixture other than D1's (a missing `console_model` arm gives 64 transforms and one
  reduction), or if a muted send, a muted track strip or a latency-bearing submix effect enters the
  plan the baseline times. No other test compiles this row's plan (R1).
- **`the_bus_send_row_prints_its_facts_and_the_validator_pins_them`** (bench) turns red if the
  row's real record is refused by `session_kind_shape` or `session_kinds`, names another fixture,
  or renders with an error or a forbidden operation. Otherwise those defects surface only in the
  one timed run (M4, J1's companion).
- **`the_bus_send_row_states_its_facts_and_is_emitted_before_the_strip_rows`** turns red if a
  non-removal strip edit (M6: `HalfMono`) or a warmup is attached to the sends row. Such a row still
  compiles, renders and passes the record validator. That is because `strip_content` is a separate
  wildcard and warmup is not a record key, so it is the only red test under M6. A removal edit is
  caught by all three new tests anyway, because the submixes then name an undeclared console slot
  (M5). Its listed red mutations are all caught elsewhere (M3, M4). See NIT-1.

## Findings

### MINOR-1: D6's doc and the spec's test-value line overclaim "a muted strip"

`lib.rs` (the D6 doc, "Red mutations: … mute one send or strip in the fixture (a `route-mute` or
`route-follow-zeroed` row)") and the spec's gate-2 test value ("a muted send, a muted strip … enters
the plan") both say too much. A muted **submix** strip leaves D6 green (R2): the returns into
`main-out` have `follows_mute: false`, so the plan writes no `route-follow-zeroed` row, and a
strip mute is not a route mute. Coverage exists, because `check-console-fixtures.sh` asserts every
submix fader unmuted and the `cmp` refuses a hand edit. The test's stated value is wrong, though.

**Fix:** say "a muted send or a muted **track** strip" in both places, and add one clause naming
the fixture witness as the holder of submix-strip mutes. Do not add a model assertion to D6: it
would duplicate the witness.

### MINOR-2: the bench module doc is stale and omits the new row

`tools/bench/src/console.rs:162` still says the console-strip rows are "emitted after the metered
row". They are now emitted after the bus-and-send row. The module doc has a section for every row
family (driver-fed, metered, mixing, console-strip) but none for this row or its in-run digest
rule (`SENDS_PAIR`).

**Fix:** change `:162` to "emitted after the bus-and-send row (#1227)". Add a short
`# The bus-and-send row (issue #1227)` paragraph:
- the committed fixture;
- BASELINE plus Bound, the C ABI's fan-playback shape;
- no floor;
- the in-run digest inequality against `sixty_four_track_console`.

### NIT-1: the stated-facts test lists only red mutations that other tests also catch

Its doc lists dropping `fixture_id`, leaving the row out of `synthetic`, and chaining after the
strip rows. The bench short-run test and the strip-row test already catch all three (M3, M4). Its
unique catch is the parity the record cannot show: a non-removal `strip()` edit (M6), warmup and
collapse.

**Fix:** name that unique catch in the doc ("a non-removal strip edit or a warmup attached to the
row, which its record cannot show"). Optionally drop the asserts that only restate record-pinned
facts.

### NIT-2: D6's failure message dumps every timing row

`assert_eq!(delayed_route_edges, 0, "{timings:?}")` prints all 202 rows (R1 produced a 20 KB
message). **Fix:** print only the rows with `fields[3] != "0"`.

### NIT-3: a stale comment in `main` (pre-existing)

The `rows` comment in `main` (`console.rs` ~`:272-274`) names only the standing, driver-fed and
metered rows. It already omitted the strip rows before this change. **Fix:** say "every
`native_session_rows()` row", or name all five lists.

## Scratch test used for the C ABI corroboration (not committed)

```rust
// crates/host-core/tests/zz_verifier_sends_capi_path.rs (deleted after the run)
let (_, prepared) = prepare_host_session(document, &caps()).unwrap();   // generous caps, no meters
println!("verifier {name}: submixes={} routes={} latency={} delay_bytes={} folds={}",
    prepared.report.submix_count, prepared.report.route_count, prepared.report.latency_samples,
    prepared.report.graph_delay_bytes, prepared.plan.bank_route_folds());
// standing: submixes=0 routes=64 latency=486 delay_bytes=0 folds=64
// sends:    submixes=10 routes=202 latency=972 delay_bytes=0 folds=0
```
