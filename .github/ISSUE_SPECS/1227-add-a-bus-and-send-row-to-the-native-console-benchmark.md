# Add a bus-and-send row to the native console benchmark

Successor performance issue BM1, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is tooling only: no engine change and no
timing. *Add the bus-and-send session to the browser mixing benchmark* (#1228, BM2) adds the same session
to the V8 path, and *Record the bus-and-send baseline and its route-work profile* (#1229, BM3) times both.

## Product outcome

The native console benchmark gains one row, `sixty_four_track_console_sends`, that renders a real
bus-and-send session:

- 64 tracks feeding 8 processed submix strips;
- two effect returns;
- two sends per track.

It is prepared as the C ABI fan-playback host prepares (static, no live controls). The fixture, the
row, its floor entry, every record count and validator, and a short-run record test are frozen here,
so the row is proven before anything is timed (`AGENTS.md`: preflight the schema before timing).

Today no benchmark row has a single submix, so the cost of what the umbrella shipped is unmeasured.

## Context (verified on `fe8ac679`; batches K1-K3 add the grammar this needs and move no anchor below)

- **The benchmark paths** that owner ruling R9 keeps (`docs/rulings/engine-footprint-2026-09-28.md:35`):
  the native console `--step` rows, run by `scripts/operator/run-console-benchmark.sh`, and the V8
  rows on the shipped artifact. Records live under `artifacts/steps/<NAME>/`.
- **Native rows** (`tools/console-workload/src/lib.rs`):
  - `pub enum Workload` (`:142`). `kind()` (`:664`) is an exhaustive match with no wildcard;
    `fixture_id()` (`:702`), `strip()` (`:744`), `strip_content()` (`:762`) and `strip_layout()`
    (`:799`) have wildcard arms; `synthetic()` (`:723`) is true unless a row is listed.
  - `native_session_rows()` (`:579`) chains `WORKLOADS` (`:517`, 15), `DRIVER_FED_WORKLOADS`
    (`:548`), `METERED_WORKLOADS` (`:561`) and `CONSOLE_STRIP_WORKLOADS` (`:569`, 5): 22 rows.
  - The row count and the metered row's index are pinned at `:2867-2878`, and
    `the_console_strip_rows_state_their_facts_and_are_emitted_last` (`:3096`) requires the strip rows
    to be emitted **last**.
  - `console_model()` (`:986-1009`) picks the fixture text by workload with a **wildcard** arm
    (`_ => SIXTY_FOUR_TRACK`, the intended fixture, `include_str!` at `:112-113`). A new row needs its
    own arm, or it silently renders the intended fixture.
  - `apply_strip`'s doc (`:845-847`) says every in-code edit is a removal or a neutralisation, never
    an addition. A bus-and-send row is an addition, so its additions live in a committed fixture.
  - `build_full` (`:1257-1452`) prepares effects, prepares builtins with
    `builtins_compiler::prepare_session_builtins` (`:1324`), then `GraphCompiler::compile_with_builtins`
    (`:1327-1333`). Under `PlanConfig::BASELINE` (`:950-954`) it attaches no live controls and no
    meters. These are the compilers host-core calls, not host-core: `tools/console-workload` links no
    host crate.
  - `SessionRuntime::bank_route_folds()` (`:1814`) reads the plan's folded route lanes (#218).
- **Plan facts are readable from the compiled artifact.** `GraphCompiler::evidence(graph, report)`
  (`crates/graph-compiler/src/compile.rs:35`) returns the canonical text, whose `route-transform`
  rows are one per route (`crates/graph-compiler/src/canonical.rs:262-273`) and whose
  `route-timing\t<route>\t<source arrival>\t<compensation>\t<destination arrival>` rows carry each
  route's compensation delay (`:274-283`). `GraphCompiler::reductions(graph)` (`compile.rs:65`) lists
  the reductions. `crates/graph-compiler/tests/compile_shapes.rs:488-496` reads an artifact this way.
- **Derived fixtures.** The precedent is #1085 (`e1b0fed3`), which added
  `fixtures/session/v1/console-sixty-four-track-app.json` and `scripts/derive-app-console-fixture.py`.
  A derive script loads its parent fixture, edits it, and canonicalises it through
  `session-validator validate --canonical`. `scripts/check-console-fixtures.sh` regenerates every
  derived fixture and `cmp`s it byte for byte (`:33-38`), then asserts semantic witnesses in Python
  (`:40-` onward). CI runs `bash scripts/check-console-fixtures.sh target/release/session_validator`.
- **The parent fixture** is `fixtures/session/v1/console-sixty-four-track-intended.json`: one source;
  console `pre_insert: [eq (miso.parametric-eq), comp (miso.compressor)]` and
  `post_insert: [limiter (miso.true-peak-limiter, link maximum)]`; 64 tracks; 64 routes `chNN-main`
  from `post_pan` to `output_input` `main-out`; no submixes and no automation.
- **Floor table** (`tools/bench/src/floor.rs`): `floor_row` (`:154`) is an exhaustive match; an
  underived row returns `None` and is listed in the test helper `underived()` (`:370`). The jq mirror
  is `floor_pins` (`scripts/console-benchmark-record-lib.jq:93`), held to parity by
  `rust_and_jq_floor_tables_have_exact_key_value_parity` (`floor.rs:470`).
- **The hard-coded record count.** `tools/bench/src/console.rs` prints 22 `console_session` records
  plus 8 others per round: 60 across the two measured rounds. That 60 is pinned at:
  - `scripts/operator/run-console-benchmark.sh:381` (`wc -l`), and in the runner's header comment
    (`:20`, "validates the 60 records");
  - `scripts/console-benchmark-validator.jq` (`:10` length, `:15` the 44 `console_session` records,
    `:16-27` per-kind counts, `:30` unique `record:kind:round` triples);
  - `scripts/operator/preflight-console-benchmark.sh:84` (`records_required: 60`), whose sha256 list
    (`:71-82`) names the fixtures and scripts it holds.
- **The record library** (`scripts/console-benchmark-record-lib.jq`): `session_kinds` (`:212`), the
  exact sorted list; `session_kind_shape` (`:296-431`), one branch per kind ending `else false end`;
  the fixture constants `console_fixture`, `app_console_fixture`, `legacy_console_fixture` and
  `mono_console_fixture` (`:268-283`).
- **The aggregate's sparse-triple rule** (`scripts/console-benchmark-validator.jq:64-75`, #1085): the
  sparse row's digest must differ from both the all-active and the idle row's, so a row cannot publish
  another row's cost under its own name.
- **The native mutation test** (`scripts/test-console-benchmark.sh`, required CI at
  `.github/workflows/qualification.yml:1036`): a literal `sessions` list of all 22 rows (`:1184-`), an
  index-map comment (`:1276-1282`), positional mutations `.[27]`, `.[30]`, `.[54]` and `.[59]`
  (`:1283-1324`), and the labels "sixty-record", "fifty-nine" and "sixty-one" (`:1274`, `:1283-1284`,
  `:1409`). Its browser section (`:1424-1590`) is *Add the bus-and-send session to the browser mixing
  benchmark*'s (BM2) and is not touched here.
- **The short-run record test** (#1085 precedent):
  `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`
  (`tools/bench/src/console.rs:2841`) runs each console-strip row for `SHORT_RUN_OBSERVATIONS`,
  asserts no render error and no forbidden operation, and holds the record to the validator. It is
  how a bad `session_kind_shape` branch shows up before a timed run.
- **Folds.** `every_standing_workload_folds_one_route_per_track`
  (`tools/console-workload/tests/chain_shape.rs:346`) iterates only `WORKLOADS` and
  `DRIVER_FED_WORKLOADS`. A bus row cannot fold one route per track: `route_fold` targets only the
  master reduction (`crates/graph/src/runtime.rs:6351`), and the sends read mid-chain taps.
- **Policy.** `scripts/check-bench-policy.sh` holds the `timed_subjects` ratchet (`:193`), which lists
  only `tools/bench/src/console.rs`, so a new row inside it needs no change.
  `scripts/check-conformance-boundaries.sh:220-224` pins `tools/bench`'s dependency set.
- **#1107** (open) may add a sparse-lane row through its own qualification issue
  (`.github/ISSUE_SPECS/1107-skip-silent-lanes-in-console-banks.md:28-29`), which would move the same
  counts.
- **After batch K3:** submix strips, bus taps, console slots on buses, and the route keys `mute` and
  `follows_mute` exist; `follows_mute: true` is legal only on a route into a submix.

## Decisions frozen for this issue

- **D1. The fixture.** `fixtures/session/v1/console-sixty-four-track-sends.json` is derived by a new
  `scripts/derive-sends-console-fixture.py` from the intended fixture and canonicalised through the
  session validator. The script's edits are exactly these additions:
  - **8 submix strips**, `bus-0`..`bus-7`: every console slot, with entries cloned from track
    `ch00`'s; one `miso.compressor` insert with `link_mode: maximum` and default parameters; an
    identity input section, a 0 dB unmuted fader and the identity `matrix` with smoothing 0.
  - **Two returns**: `fx-a` with one `miso.delay` insert and `fx-b` with one `miso.parametric-eq`
    insert; every console slot cloned from `ch00`'s; otherwise as the buses.
  - **Main routes**: track `chNN`'s main route is re-pointed from the output to
    `submix_input bus-<NN div 8>`; its ID, tap (`post_pan`), matrix and gain are unchanged. Each bus
    and each return routes `post_pan` to `main-out` at unity.
  - **Sends**: every track sends `pre_fader` to `fx-a` at -12 dB and `post_fader` to `fx-b` at
    -18 dB, with the identity matrix.
  - **Route flags**: every route has `mute: false`; every route into a submix has
    `follows_mute: true`; every route into the output has `follows_mute: false` (the only legal
    value there).
  - `session_id` becomes `console-sixty-four-track-sends`. The source, the rate (48 kHz), the
    quantum (128) and every track strip stay as they are.
- **D2. The row.** `Workload::SixtyFourTrackConsoleSends`, kind `sixty_four_track_console_sends`.
  - `fixture_id()` returns the new file; `synthetic()` is false; `strip()`, `strip_content()` and
    `strip_layout()` state the intended fixture's strip.
  - `console_model()` gets an explicit arm reading a new `include_str!` constant for the fixture.
  - It sits in a new list `BUS_SEND_WORKLOADS: [Workload; 1]`, which `native_session_rows()` chains
    **before** `CONSOLE_STRIP_WORKLOADS`, so the strip rows stay last. It is not added to
    `WORKLOADS` or `DRIVER_FED_WORKLOADS`, and it is prepared at `PlanConfig::BASELINE`.
- **D3. Floor.** `floor_row` returns `None` for it; it is listed in `underived()`, and `floor_pins`
  gains `[null, 1, "none", "not_derived"]` for it.
- **D4. Counts.** Each round prints 31 records, so the aggregate is 62, with 46 `console_session`
  records. Every site in the Context moves from 60 to 62 (and 44 to 46), including the runner's
  header comment, and every positional mutation is re-pointed at the record it meant. If a #1107 row
  lands first, this issue re-points from that issue's counts; if this lands first, that issue does.
- **D5. The aggregate rule.** The validator adds, beside the sparse triple: the sends row's
  `output_sha256` differs from `sixty_four_track_console`'s. Equal digests would mean the sends
  fixture never reached the plan.
- **D6. Facts, derived from the plan, not pinned digests.** A test builds the row's compiled plan and
  asserts only facts the fixture check cannot see, each with a comment that derives it:
  - route ops executed per block: the `route-transform` row count of `GraphCompiler::evidence`
    minus `bank_route_folds()`;
  - delayed route edges: `route-timing` rows with a nonzero compensation delay;
  - `bank_route_folds()` itself.

  It does not recount buses, routes or sends (`check-console-fixtures.sh` witnesses those). If
  `SessionRuntime` exposes no evidence, add one read-only accessor that returns
  `GraphCompiler::evidence` of the compiled plan, captured in `build_full` before bind.
- **D7. Fixture constant.** The record library defines `sends_console_fixture`, used by the native
  `session_kind_shape` branch and, in BM2, by the browser document's validator.

## Deliverables

1. `scripts/derive-sends-console-fixture.py` and the committed fixture (D1).
2. `scripts/check-console-fixtures.sh`: regenerate and `cmp` the new fixture; semantic witnesses (8
   buses and 2 returns, each carrying every console slot; 64 main routes into buses; 128 sends; every
   route's `mute` false; `follows_mute` true exactly on routes into a submix).
3. The row (D2), with the row-count pin (`lib.rs:2867-2878`) updated.
4. The floor (D3) in `floor.rs` and `console-benchmark-record-lib.jq`.
5. The validators and runners (D4, D5, D7): `console-benchmark-validator.jq`;
   `console-benchmark-record-lib.jq` (`session_kinds`, a `session_kind_shape` branch,
   `sends_console_fixture`); `run-console-benchmark.sh` (`:381` and the header comment `:20`);
   `preflight-console-benchmark.sh` (`:84`, and the sha256 list gains the new fixture and its derive
   script).
6. `scripts/test-console-benchmark.sh`, native section only: the `sessions` list, the index map, every
   positional mutation and its label, and a case refusing a set whose sends digest equals the
   console's (D5).
7. The short-run record test: `tools/bench/src/console.rs` runs the new row beside the console-strip
   rows in `the_console_strip_rows_print_their_facts_and_the_validator_pins_them` (or a sibling test
   of the same shape), asserting no render error, no forbidden operation and an accepted record.
8. The facts test (D6), in `tools/console-workload/tests/chain_shape.rs` or a new test file.

## Authorized paths

- `fixtures/session/v1/console-sixty-four-track-sends.json` (new)
- `scripts/derive-sends-console-fixture.py` (new), `scripts/check-console-fixtures.sh`
- `tools/console-workload/src/lib.rs`, `tools/console-workload/tests/chain_shape.rs` (or one new test
  file)
- `tools/bench/src/floor.rs`, `tools/bench/src/console.rs`
- `scripts/console-benchmark-record-lib.jq`, `scripts/console-benchmark-validator.jq`,
  `scripts/console-benchmark-record-validator.jq` (only if it names kinds),
  `scripts/test-console-benchmark.sh` (the native section, `:1-1423`)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
  (counts, comments and the sha256 list only)
- this spec

## Non-goals

- No timing: the runner's timed step is not invoked here.
- No V8 document (*Add the bus-and-send session to the browser mixing benchmark*, BM2).
- No engine change, no tuning and no new timed subject.
- No host-core dependency in `tools/bench` (it would fail `check-conformance-boundaries.sh`).
- No live-controlled native row; the live-controlled shape is the V8 document.
- No change to `every_standing_workload_folds_one_route_per_track` or to the standing rows.
- No projected saving quoted anywhere.

## Hazards

- **Emission order.** The strip rows must stay last. Insert the new list before them and re-point
  every positional mutation; a mutation that silently lands on the wrong record still "passes".
- **Canonical form.** The fixture must be the session validator's canonical output; hand-edited JSON
  fails the `cmp`.
- **The wildcard model arm.** Without its own `console_model()` arm the row renders the intended
  fixture under the sends row's name; D5's aggregate rule and the facts test both catch it.

## Objective gates

1. **The fixture is derived.** `cargo build --locked --release -p audit -p bench -p capi -p session-validator`,
   then `bash scripts/check-console-fixtures.sh target/release/session_validator` passes,
   regenerating the new fixture byte for byte and holding its witnesses.
2. **The row is wired and proven before timing.**
   - `cargo test --locked --release -p audit -p bench -p console-workload` passes, including the
     short-run record test (deliverable 7) and the floor parity test.
     *Test value (short-run record): it turns red if the new row's record is refused by
     `session_kind_shape`, renders with an error, or performs a forbidden operation, which would
     otherwise surface only in the one timed run.*
   - The facts test passes.
     *Test value (facts): it turns red if the row silently renders another fixture, or gains or loses
     route ops, delayed edges or folds, so the baseline would time a different plan.*
   - `bash scripts/test-console-benchmark.sh` passes, with every mutation case still refused on its
     intended record and the D5 case refused.
     *Test value (D5 case): it turns red if the aggregate accepts a sends row that rendered the
     standing console's bits.*
   - `bash scripts/operator/preflight-console-benchmark.sh --step bus-send-preflight` passes and
     prints `records_required: 62`; it launches no timed workload and leaves no artifact directory
     behind.
3. **The standing rows are untouched.** `every_standing_workload_folds_one_route_per_track` and the
   strip-row tests pass unchanged.
4. **Policy and boundaries.**
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh` (`timed_subjects`
     unchanged)
   - `bash scripts/check-conformance-boundaries.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The derive script's output diff against the intended fixture: additions only.
- The preflight JSON output.
- The facts test's numbers, each with its derivation.
- The short-run record printed by the record test.

## Dependencies

- *Let a send follow its source strip's mute live in the browser* (#1224, batch K3 closed and pushed)

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Freeze the row, fixture and validators in this issue. Nothing is timed here; never invoke a timed
  step.
- Do not quote a projected saving.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
