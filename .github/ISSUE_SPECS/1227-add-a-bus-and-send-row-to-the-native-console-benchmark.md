# Add a bus-and-send row to the native console benchmark

Successor performance issue BM1, outside *Submix strips and live aux sends* (#1196, filed by *Record the
submix, send and VCA ruling* (#1197) as a standalone issue). It is tooling only: no engine change and no
timing. *Add the bus-and-send session to the browser mixing benchmark* (#1228, BM2) adds the same session
to the V8 path, and *Record the bus-and-send baseline and its route-work profile* (#1229, BM3) times both.

## Product outcome

The native console benchmark gains one row, `sixty_four_track_console_sends`, that renders a real
bus-and-send session from a committed fixture:

- 64 tracks feeding 8 processed submix strips (buses);
- two effect returns;
- two sends per track.

The row is prepared the way the C ABI prepares a plan today: no live controls, no meters,
`Concurrent` builtins (`crates/capi/src/runtime/compile.rs:421` calls host-core's
`prepare_host_runtime`, which attaches nothing). That is the fan-playback shape while *Deliver
value-only fader, mute and pan transactions to the running C ABI plan through the live console
lanes* (#1053) and *Deliver value-only send and submix-strip edits to the running C ABI plan*
(#1225) are open. The fixture, the row, its floor entry, every record count and validator, a
short-run record test and a plan-facts test are frozen here, so the row is proven before anything
is timed (`AGENTS.md`: freeze the workload and validator before timing; preflight without
launching the workload).

Today no benchmark row has a single submix, so the cost of what the umbrella shipped is unmeasured.

## Context (verified on `main` at `1cb677a76`: K1-K3 and the VCA batch are merged)

- **The benchmark paths** that owner ruling R9 keeps (`docs/rulings/engine-footprint-2026-09-28.md:35`):
  the native console `--step` rows, run by `scripts/operator/run-console-benchmark.sh`, and the V8
  rows on the shipped module. Records live under `artifacts/steps/<NAME>/`.
- **Native rows** (`tools/console-workload/src/lib.rs`):
  - `pub enum Workload` (`:142`). `kind()` (`:664`) is an exhaustive match with no wildcard.
    `fixture_id()` (`:702`) has a wildcard that returns the **intended** fixture, so the row needs
    its own arm there. `tracks()` (`:691`, wildcard 64), `strip()` (`:744`, wildcard
    `AsWritten`), `strip_content()` (`:762`, wildcard `eq+compressor+limiter`), `strip_layout()`
    (`:799`, wildcard the intended layout), `input()` (`:823`, wildcard `Tone`),
    `warmup_blocks()` (`:835`, wildcard 0), `web_meters()` (`:418`), `collapse_forced_off()`
    (`:434`) and `source_feed()` (`:443`, wildcard `Bound`) need no arm for the new row.
    `synthetic()` (`:723`) is true unless the row is listed in its `matches!`.
  - `native_session_rows()` (`:579`) chains `WORKLOADS` (`:517`, 15 rows, the wasm arm's
    append-only address space), `DRIVER_FED_WORKLOADS` (`:548`), `METERED_WORKLOADS` (`:561`) and
    `CONSOLE_STRIP_WORKLOADS` (`:569`, 5): 22 rows.
  - `console_model()` (`:986-1009`) picks the fixture text with a **wildcard** arm
    (`_ => SIXTY_FOUR_TRACK`, `:994`; the fixture constants are `include_str!`s at `:100-138`). A
    new row needs its own arm, or it silently renders the intended fixture under its own name.
  - `apply_strip`'s doc (`:845-847`) says every in-code edit is a removal or a neutralisation. A
    bus-and-send row is an addition, so its additions live in a committed fixture and the row is
    `Strip::AsWritten`.
  - `build_full` (`:1257-1452`): effects, then `builtins_compiler::prepare_session_builtins`
    (`:1324`) unless the row is web-metered, then `GraphCompiler::compile_with_builtins`
    (`:1327-1333`), then a `Bound` feed (`FrozenGraphSource` per track input). Under
    `PlanConfig::BASELINE` (`:950-954`) it attaches no live controls and no meters.
    `tools/console-workload` links no host crate.
  - Tests in the crate's test module (`:2298`): the row-count pin and the "no other row is
    web-metered" loop in `the_metered_row_states_the_console_rows_facts_and_is_the_only_web_metered_row`
    (`:2846`; the loop `:2861-2867`, the count `:2868-2879`); and
    `the_console_strip_rows_state_their_facts_and_are_emitted_last` (`:3096`, `STRIP_ROWS` at
    `:3074`), which requires the strip rows to be emitted **last** (`:3177-3180`) and holds every
    emitted row's `BypassCensus` (`:3184-3199`).
  - `the_driver_fed_rows_metadata_charge_grows_by_exactly_the_executor_tables` (`:2516-2535`) is the
    precedent for recompiling a row's model in the test module through the calls `build_full`
    makes, then reading `artifact.graph()`.
  - `SessionRuntime::bank_route_folds()` (`:1814`) reads the plan's folded route lanes (#218).
- **Plan facts are readable from the compiled artifact.** `GraphCompiler::evidence(graph, report)`
  (`crates/graph-compiler/src/compile.rs:35`) returns the canonical text
  (`crates/graph-compiler/src/canonical.rs`):
  - `reduction\t<node>\t<rank>\t<edge>` rows, one per contribution (`:252-261`);
  - one `route-transform\t<node>\t...` row per route (`:262-273`), followed by a `route-mute` row
    for a muted route (`:275-277`) and a `route-follow-zeroed` row for a route with a zeroed
    source lane (`:280-289`);
  - one `route-timing\t<route>\t<source arrival>\t<compensation>\t<destination arrival>` row per
    route (`:291-300`).

  `crates/graph-compiler/tests/compile_shapes.rs:505` reads it as
  `GraphCompiler::evidence(artifact.graph(), artifact.report())`.
- **Derived fixtures.** The precedent is #1085 (`e1b0fed3`): `scripts/derive-app-console-fixture.py`
  loads the intended fixture, asserts its shape, edits it and canonicalises it through
  `session-validator validate --canonical` (its `canonicalise`, with the `--validator <path>` and
  `cargo run -q -p session-validator` forms). `scripts/check-console-fixtures.sh` regenerates every
  derived fixture and `cmp`s it byte for byte (`:33-38`), then asserts semantic witnesses in an
  inline Python block (`:40-`). CI runs `bash scripts/check-console-fixtures.sh
  target/release/session_validator` (`.github/workflows/qualification.yml:830`). Every
  `fixtures/session/v1/*.json` is also validated through all five stages by
  `fixtures_distinguish_schema_examples_from_launch_effects`
  (`tools/session-validator/tests/validate.rs`).
- **The parent fixture** `fixtures/session/v1/console-sixty-four-track-intended.json`: one source;
  console `pre_insert: [eq (miso.parametric-eq), comp (miso.compressor)]`, `post_insert: [limiter
  (miso.true-peak-limiter, link maximum)]`; 64 tracks `ch00`..`ch63`, each with three active
  console entries and no inserts; 64 routes `chNN-main` from `post_pan` to `output_input`
  `main-out`, each already carrying `"mute": false, "follows_mute": false`; `submixes: []`,
  `vcas: []`, `automation: []`.
- **The session grammar the fixture uses** (`docs/SESSION_SCHEMA_V1.md:43-61`, `:276-310`): a
  submix is `{id, builtins, console, inserts, fader, matrix}` (a track's grammar, with `matrix`
  in place of `pan`); `Submix::unity` (`crates/session/src/model.rs:713`) is the transparent strip.
  A route's source is `{kind: "track", track_id, tap}` or `{kind: "submix", submix_id, tap}`; its
  destination `{kind: "submix_input", submix_id}` or `{kind: "output_input", output_id}`; every
  route carries `mute` and `follows_mute`, and `follows_mute: true` is legal only into a submix.
  The SDK writes `follows_mute: true` by default on a route into a submix
  (`sdk/src/core/session.ts:1639`). A VCA is applied at preparation by baking each member's
  effective fader (`docs/SESSION_SCHEMA_V1.md:76-86`), so a VCA adds no render work.
- **Effects the fixture adds.** `miso.compressor` supports every link mode
  (`crates/compressor/src/lib.rs:345`) and has latency 0 (`:295`). `miso.delay` supports
  `dual_mono` only, has latency 0 and has no bank kernel, so it renders per node
  (`crates/delay/src/lib.rs:254`, `:280`, `:287`). `miso.parametric-eq` has latency 0
  (`crates/parametric-eq/src/lib.rs:652`). The limiter's latency is fixed per quality
  (`crates/true-peak-limiter/src/lib.rs:242`).
- **Planner probe (on `1cb677a76`, not committed).** A draft of exactly D1's fixture passed all
  five `session-validator` stages, and `SessionRuntime::build(.., PlanConfig::BASELINE)` rendered
  it without error. Its compiled plan had 202 `route-transform` rows, 202 `route-timing` rows all
  with compensation 0 (tracks arrive at 486 samples, submixes at 972), 11 reduction nodes
  (`main-out` and the 10 submix inputs), no `route-mute` or `route-follow-zeroed` row, and
  `bank_route_folds() == 0`. These are what D6 derives; the test computes them, it does not copy
  them.
- **Floor table** (`tools/bench/src/floor.rs`): `floor_row` (`:154`) is an exhaustive match whose
  first arm returns `None` for underived rows (`:157-163`); the test helper `underived()` (`:370`)
  lists them. The jq mirror is `floor_pins` (`scripts/console-benchmark-record-lib.jq:93`),
  underived rows as `[null, 1, "none", "not_derived"]` (`:151-168`), held to parity by
  `rust_and_jq_floor_tables_have_exact_key_value_parity` (`floor.rs:470`).
- **The bench subject** (`tools/bench/src/console.rs`): `main` measures every
  `native_session_rows()` row, asserts the in-run digest pairs (`GAIN_PAN_FEED_PAIR`,
  `METERED_PAIR`, `SPARSE_TRIPLE` at `:206-235`, asserted at `:278-298`), then prints 22
  `console_session` records plus 8 others per round: 60 across the two measured rounds.
- **The hard-coded counts.**
  - `scripts/operator/run-console-benchmark.sh:381` (`wc -l` is 60) and its header comment (`:20`,
    "validates the 60 records").
  - `scripts/console-benchmark-validator.jq`: header comment (`:1-7`), `length == 60` (`:10`), 44
    `console_session` records (`:15`), unique `record:kind:round` triples `== 60` (`:30`).
  - Prose that states the count or the row lists: `scripts/test-console-benchmark.sh:5` ("sixty
    records"); `scripts/console-benchmark-record-lib.jq:208-211` ("twenty-two session workloads");
    `tools/console-workload/src/lib.rs:563` (`CONSOLE_STRIP_WORKLOADS` "emits after
    [`METERED_WORKLOADS`]") and `:577-578` (`native_session_rows`'s doc); the docs of `floor_row`
    and `underived()` in `tools/bench/src/floor.rs`.
  - `scripts/operator/preflight-console-benchmark.sh:84` (`records_required: 60`); its sha256 list
    has two halves, the `--arg` lines (`:68-82`) and the object's fields (`:85-96`).
- **The record library** (`scripts/console-benchmark-record-lib.jq`): `session_kinds` (`:212`), an
  exact sorted list; `session_kind_shape` (`:296-431`), one branch per kind ending `else false end`;
  the fixture constants (`:270-283`).
- **The aggregate's sparse-triple rule** (`scripts/console-benchmark-validator.jq:64-75`, #1085): a
  row must not publish another row's bits under its own name.
- **The mutation test** (`scripts/test-console-benchmark.sh`, required CI at
  `.github/workflows/qualification.yml:1036`). Its native part is `:1-1422`:
  - single-record bases and their accept cases (`:340-374`), and the per-key structural loop over
    every base (`:379-`);
  - the aggregate's record builder (`:1174-1272`): a standalone `jq -cn` program that does **not**
    include the record library and defines its fixture paths locally (`:1179-1183`); its
    `sessions` list (`:1184-1248`) has 22 entries in emission order, each with a one-character
    digest suffix (`$a[0:63] + $s.digest`), and between them they already use all sixteen hex
    digits;
  - `expect_aggregate_accept .. 'the sixty-record set'` (`:1274`);
  - the index-map comment (`:1276-1282`) and the positional mutations `.[0]`, `.[27]`, `.[30]`,
    `.[54]`, `.[55]`, `.[56]`, `.[57]`, `.[58]` and `.[59]` (`:1283-1325`), labelled
    "fifty-nine records" and "sixty-one records";
  - the floor section's per-kind `scale` map (`:1374-1388`), which has no default (a kind missing
    from it makes `rescale` fail), and `'the sixty-record set with floor accounting'` (`:1409`).

  Its browser part (`:1423-`) is BM2's and is not touched here.
- **The short-run record test** (#1085 precedent):
  `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`
  (`tools/bench/src/console.rs:2841`) runs each console-strip row for `SHORT_RUN_OBSERVATIONS`
  (`:2372`, 8), asserts no render error, no forbidden operation and no meter group, and holds the
  record to the validator through `record_validator_accepts` (`:2436`): refused as it is (shortened)
  and accepted at `.observations = 1000`.
- **Folds.** `every_standing_workload_folds_one_route_per_track`
  (`tools/console-workload/tests/chain_shape.rs:346`) iterates only `WORKLOADS` and
  `DRIVER_FED_WORKLOADS`; the bus row is not added to either. `route_fold`
  (`crates/graph/src/runtime.rs:7106`) folds one reduction's routes only, and only when that
  reduction's contributors are exactly the folded chains' lanes in order.
- **Policy.** `scripts/check-bench-policy.sh` holds the `timed_subjects` ratchet (`:193`), which
  lists only `tools/bench/src/console.rs`, so a new row inside it needs no change.
  `scripts/check-conformance-boundaries.sh:220-224` pins `tools/bench`'s dependency set.
- **Other rows in flight.** #1107, #938, #955, #961 and #965 may add or move rows. Whichever lands
  second re-points its counts and positional mutations from the other's.

## Decisions frozen for this issue

- **D1. The fixture.** `fixtures/session/v1/console-sixty-four-track-sends.json` is the session
  validator's canonical output of a draft that a new `scripts/derive-sends-console-fixture.py`
  builds from the intended fixture. The script first asserts the parent's shape as listed in the
  Context, and then makes exactly these edits:
  - **`session_id`** becomes `console-sixty-four-track-sends`. The source, the rate (48 kHz), the
    quantum (128), the console declaration and every track stay byte for byte as they are.
  - **10 submix strips**, `bus-0`..`bus-7`, `fx-a` and `fx-b`. Each has:
    - `Submix::unity`'s input section on both lanes (`polarity_invert: false`, `trim_db: 0.0`,
      `hpf_hz: 0.0`, `lpf_hz: 0.0`, `delay_samples: 0`);
    - `console`: a deep copy of track `ch00`'s three entries (`eq`, `comp`, `limiter`, all
      `bypass: false`, with `ch00`'s params);
    - one insert in `inserts.effects`, `quality: "normal"`, `bypass: false`, `params: []` (the
      descriptor defaults) and `sidechain: {kind: "none"}`:
      - each bus: `{id: "glue", identity: miso.compressor, link_mode: "maximum"}`;
      - `fx-a`: `{id: "echo", identity: miso.delay, link_mode: "dual_mono"}`;
      - `fx-b`: `{id: "tone", identity: miso.parametric-eq, link_mode: "dual_mono"}`;
    - `fader`: 0 dB on both lanes, unmuted;
    - `matrix`: `{ll: 1, lr: 0, rl: 0, rr: 1, smoothing_samples: 0}`.
  - **Main routes.** Track `chNN`'s route `chNN-main` is re-pointed to
    `{kind: "submix_input", submix_id: "bus-<NN div 8>"}`. Its tap (`post_pan`), matrix and gain
    are unchanged.
  - **Sends.** Every track gets `chNN-fx-a` (tap `pre_fader`, into `fx-a`, `gain_db: -12.0`) and
    `chNN-fx-b` (tap `post_fader`, into `fx-b`, `gain_db: -18.0`), each with the identity
    `channel_matrix`.
  - **Returns to the output.** `bus-K-main`, `fx-a-main` and `fx-b-main` route each submix's
    `post_pan` to `main-out` at 0 dB with the identity matrix.
  - **Route flags.** Every route has `mute: false`. Every route into a submix has
    `follows_mute: true`, the SDK's default there, so the browser document of BM2 carries the
    production follow. Every route into the output has `follows_mute: false`, the only legal value.
  - **Not added, on purpose.** No muted send: a muted send is skipped (no mix, no load), so the
    baseline times every send active. No VCA: a VCA is baked into the faders at preparation and
    adds no render work. `vcas` and `automation` stay `[]`.
- **D2. The row.** `Workload::SixtyFourTrackConsoleSends`, kind `sixty_four_track_console_sends`.
  - New arms: `kind()`; `fixture_id()` returns `fixtures/session/v1/console-sixty-four-track-sends.json`;
    `synthetic()` lists it (rendered as written, so `false`); `console_model()` reads a new
    `SIXTY_FOUR_TRACK_SENDS` `include_str!` constant, documented as the other fixture constants are.
  - Every other method keeps its wildcard. `strip_content()` and `strip_layout()` describe the
    tracks' strip, which D1 leaves as written (`eq+compressor+limiter`, the intended layout); the
    submixes' strips are fixture facts that `check-console-fixtures.sh` holds.
  - It sits in a new `pub const BUS_SEND_WORKLOADS: [Workload; 1]`, documented as its siblings are,
    which `native_session_rows()` chains **after** `METERED_WORKLOADS` and **before**
    `CONSOLE_STRIP_WORKLOADS`, so the strip rows stay last. It is not in `WORKLOADS` (the wasm
    arm's address space) or `DRIVER_FED_WORKLOADS`. It is measured at `PlanConfig::BASELINE` with
    the `Bound` feed, like `sixty_four_track_console`, so the two rows differ by the buses and
    sends alone (#965 would move every row to a source set together).
- **D3. Floor.** `floor_row` returns `None` for it; `underived()` lists it; `floor_pins` gains
  `"sixty_four_track_console_sends": [null, 1, "none", "not_derived"]` with a one-line comment (no
  inventory exists for a bus or a send).
- **D4. Counts.** Each round prints 31 records (23 `console_session` + 8), so the aggregate is 62,
  with 46 `console_session` records. Every count in the Context moves from 60 to 62 and from 44 to
  46, the prose included. In the aggregate's emission order the sends row is index 17 of each
  round, so round one is `0-22` sessions, `23-24` hoists, `25` meters, `26` observation, `27`
  placement, `28` automation, `29` mono, `30` mixing, and round two is `31-61`. Re-point:
  `.[30]` to `.[31]`, `.[54]` to `.[56]`, `.[55]` to `.[57]`, `.[56]` to `.[58]`, `.[57]` to
  `.[59]`, `.[58]` to `.[60]` and `.[59]` to `.[61]`; `.[0]` and `.[27]` may stay. Rename
  "sixty-record" to "sixty-two-record", "fifty-nine records" to "sixty-one records" and
  "sixty-one records" to "sixty-three records".
- **D5. The digest rule.** The sends row must not render `sixty_four_track_console`'s bits; equal
  digests mean the fixture never reached the plan. The subject asserts it in-run beside
  `SPARSE_TRIPLE`, and `console-benchmark-validator.jq` adds the same rule beside the sparse
  triple's.
- **D6. Plan facts, derived, not pinned.** A unit test in the test module of
  `tools/console-workload/src/lib.rs`, on the `:2516` precedent, compiles
  `console_model(Workload::SixtyFourTrackConsoleSends)` through the calls `build_full` makes at
  `PlanConfig::BASELINE` (`compile_session`, `prepare_native_session_effects`,
  `prepare_session_builtins(&session, &[], builtin_caps())`, `compile_with_builtins` at
  `Backend::current()`), reads `GraphCompiler::evidence(artifact.graph(), artifact.report())`, and
  asserts, each with a comment that derives the number from D1:
  - `route-transform` rows `== 202`: one per route, 64 main routes + 128 sends + 10 returns;
  - distinct `reduction` nodes `== 11`: `main-out` and the 10 submix inputs, each summing two or
    more routes;
  - `route-timing` rows `== 202`, none with a nonzero compensation: every effect but the limiter
    has latency 0, every strip carries the limiter slot exactly once, and every tap a route reads
    (`pre_fader`, `post_fader`, `post_pan`) lies after `post_insert`, so all contributions to a
    reduction arrive at the same sample;
  - no `route-mute` and no `route-follow-zeroed` row: no route and no strip is muted, so every
    send is active.

  It then builds `SessionRuntime::new(Workload::SixtyFourTrackConsoleSends)` and prints, on one
  line, `route_transforms`, `reduction_nodes`, `bank_route_folds` (from `bank_route_folds()`),
  `route_ops_per_block` (= transforms − folds) and `delayed_route_edges` (the `route-timing` rows
  with a nonzero compensation, 0 by the assertion above), for BM3's report. The fold count is
  printed, not asserted: a derivation of it from `route_fold`'s rules is not short enough to state
  here, and a pinned count would only be a byte pin. (The planner's probe saw 0 because
  `route_fold` folds only its first candidate's master, `bus-0`'s input, which is a bank member,
  so the fold declines.)
- **D7. Fixture constant.** The record library defines
  `def sends_console_fixture: "fixtures/session/v1/console-sixty-four-track-sends.json";`, used by
  the native `session_kind_shape` branch and, in BM2, by the browser document validator.

## Deliverables

1. `scripts/derive-sends-console-fixture.py` (on the app script's shape: module docstring, parent
   assertions, `canonicalise`, `--validator <path>`) and the committed fixture (D1).
2. `scripts/check-console-fixtures.sh`: regenerate and `cmp` the new fixture beside the others;
   witnesses in the Python block:
   - `session_id == "console-sixty-four-track-sends"`, and rate, quantum, `sources`, `console` and
     `tracks` equal the intended fixture's;
   - `submixes` ids are exactly `bus-0`..`bus-7`, `fx-a`, `fx-b`; each carries every console slot
     in slot order, none bypassed, and exactly one insert (the compressor with link `maximum` on
     each bus, the delay on `fx-a`, the EQ on `fx-b`);
   - 202 routes: 64 `post_pan` routes into buses (track `chNN` into `bus-<NN div 8>`), 64
     `pre_fader` sends into `fx-a`, 64 `post_fader` sends into `fx-b`, 10 `post_pan` returns into
     `main-out`;
   - every route's `mute` is false, and `follows_mute` is true exactly on the 192 routes into a
     submix;
   - `vcas == []` and `automation == []`.
3. The row (D2) in `tools/console-workload/src/lib.rs`, with the crate's own tests extended:
   - the row-count pin (`:2868-2879`) counts `BUS_SEND_WORKLOADS`, and the metered row stays at
     index `WORKLOADS.len() + DRIVER_FED_WORKLOADS.len()`;
   - the "no other row is web-metered" loop (`:2861-2867`) iterates `BUS_SEND_WORKLOADS` too;
   - a stated-facts assertion for the row (kind, 64 tracks, fixture, `synthetic == false`,
     `Strip::AsWritten`, `Tone`, `Bound`, not in `WORKLOADS`), in the strip-row test or a sibling;
   - the census loop (`:3184-3199`) already iterates `native_session_rows()`, so it covers the row
     with no edit (no bypass);
   - the docs of `CONSOLE_STRIP_WORKLOADS` (`:563`) and `native_session_rows` (`:577-578`) name
     `BUS_SEND_WORKLOADS`.
4. The facts test (D6), in the same test module.
5. The floor (D3) in `tools/bench/src/floor.rs` (with the `floor_row` and `underived()` docs) and
   `scripts/console-benchmark-record-lib.jq`.
6. The subject: D5's in-run assertion in `tools/bench/src/console.rs`, and a new short-run record
   test for the row, a sibling of `the_console_strip_rows_print_their_facts_and_the_validator_pins_them`
   (that test is left as it is): no render error, no forbidden operation, no meter group, no bypass
   group, refused as shortened, accepted at `.observations = 1000`, and refused at the frozen count
   when its `fixture_id` is set to the intended fixture.
7. The validators and runners (D4, D5, D7): `console-benchmark-validator.jq` (counts, header
   comment, the digest rule); `console-benchmark-record-lib.jq` (`session_kinds` in sorted
   position and its comment at `:208-211`, a `session_kind_shape` branch with a comment,
   `sends_console_fixture`);
   `run-console-benchmark.sh` (`:381` and the comment at `:20`); `preflight-console-benchmark.sh`
   (`records_required: 62`, and the sha256 list gains `sends_fixture_sha256` and
   `sends_fixture_generator_sha256`, each as an `--arg` line and an object field).
8. `scripts/test-console-benchmark.sh`, native part only:
   - a single-record base `session_sends` with `expect_accept`, added to the per-key loop's bases;
   - the `sessions` entry for the row at its emission position (after the metered row, before
     `ten_track_ragged_strip`), its fixture path defined locally in that jq program as the others
     are (`sends_console_fixture` is not in scope there), and a digest suffix other than
     `sixty_four_track_console`'s `3` (for example `6`: no rule needs more uniqueness than D5's and
     the sparse triple's);
   - the header comment's "sixty records" (`:5`);
   - D4's index map, positional mutations and labels;
   - the floor `scale` map gains the kind;
   - new cases: a set missing the sends row is refused; a sends row whose digest equals
     `sixty_four_track_console`'s in both rounds is refused (D5); the same edit with a fresh digest
     in both rounds is accepted.

## Authorized paths

- `fixtures/session/v1/console-sixty-four-track-sends.json` (new)
- `scripts/derive-sends-console-fixture.py` (new), `scripts/check-console-fixtures.sh`
- `tools/console-workload/src/lib.rs`
- `tools/bench/src/floor.rs`, `tools/bench/src/console.rs`
- `scripts/console-benchmark-record-lib.jq`, `scripts/console-benchmark-validator.jq`
- `scripts/test-console-benchmark.sh` (the native part, `:1-1422`)
- `scripts/operator/run-console-benchmark.sh`, `scripts/operator/preflight-console-benchmark.sh`
  (counts, comments and the sha256 list only)
- this spec

## Non-goals

- No timing: `run-console-benchmark.sh` is never invoked here.
- No V8 document (BM2).
- No engine change, no tuning and no new timed subject.
- No host-core dependency in `tools/console-workload` or `tools/bench` (for `tools/bench` it would
  fail `check-conformance-boundaries.sh:220-224`; for `tools/console-workload` it would make the
  native subject link a host, which `REVISION-1.md` section 4 declined), so no live-controlled
  native row: the live-controlled shape is BM2's V8 document.
- No change to `WORKLOADS`, `every_standing_workload_folds_one_route_per_track` or the standing
  rows' records.
- No muted send, VCA or automation in the fixture (D1).
- No projected saving quoted anywhere.

## Hazards

- **Emission order.** The strip rows must stay last. Insert the new list before them and re-point
  every positional mutation (D4); a mutation that lands on the wrong record still "passes".
- **Canonical form.** The fixture must be the session validator's output; a hand edit fails the
  `cmp`.
- **The wildcard model arm.** Without its own `console_model()` arm the row renders the intended
  fixture under its own name. D6 catches it at test time (64 transforms, 1 reduction), D5 in the
  run.
- **Sorted kind list.** `session_kinds` is compared as a sorted list; insert the kind in order.

## Objective gates

1. **The fixture is derived.**
   `cargo build --locked --release -p audit -p bench -p capi -p session-validator`, then
   `bash scripts/check-console-fixtures.sh target/release/session_validator` regenerates the new
   fixture byte for byte and holds its witnesses, and
   `cargo test --locked -p session-validator` passes with the fixture in its corpus.
2. **The row is wired and proven before timing.**
   - `cargo test --locked --release -p audit -p bench -p console-workload` passes (the CI step at
     `.github/workflows/qualification.yml:710`), including the floor parity test, the facts test
     and the short-run record test.
     *Test value (facts test): it turns red if the row renders a fixture other than D1's, or if a
     muted send, a muted track strip or a compensated edge enters the plan the baseline times; no
     existing test compiles this row. A muted submix strip is not a route mute and its returns do
     not follow mutes, so `check-console-fixtures.sh`'s every-submix-fader-unmuted witness holds
     it, not this test.*
     *Test value (short-run record test): it turns red if the row's record is refused by
     `session_kind_shape`, names another fixture, renders with an error or performs a forbidden
     operation, which would otherwise surface only in the one timed run.*
   - `bash scripts/test-console-benchmark.sh` passes, every positional mutation still refused on
     its intended record, and the new cases behave as deliverable 8 states.
     *Test value (D5 case): it turns red if the aggregate accepts a sends row that rendered the
     standing console's bits.*
   - `bash scripts/operator/preflight-console-benchmark.sh --step bus-send-preflight` passes and
     prints `records_required: 62` with the two new sha256 fields; it launches no workload
     (`workload_launches: 0`) and creates no `artifacts/steps/bus-send-preflight/`.
3. **The standing rows are untouched.** `every_standing_workload_folds_one_route_per_track` and
   the strip-row tests pass; the only change to `the_console_strip_rows_*` tests is the
   stated-facts lines deliverable 3 adds (the bench's short-run test of the strip rows is not
   edited).
4. **Policy and boundaries.**
   - `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`
     (`timed_subjects` unchanged)
   - `bash scripts/check-conformance-boundaries.sh`
   - `bash scripts/check-workspace-policy.sh` and `bash scripts/test-workspace-policy.sh`
   - `cargo fmt --all -- --check`
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`

## Evidence

- The derive script's diff of the fixture against the intended fixture: additions and the
  re-pointed main routes only.
- The facts test's printed line, and each asserted number with its derivation.
- The short-run record printed by the record test.
- The preflight JSON.

### Attempt 1 record (Terra, on `f4cb3dc34`)

Anchors had moved by the brief's own spec commit only; every one was found by symbol (the test
module now starts at `lib.rs:2350`, the metered-row test at `:2897`, `STRIP_ROWS` at `:3125`).

- **Fixture diff against the intended fixture** (computed over the parsed documents): only
  `session_id`, `submixes` and `routes` differ. The 64 `chNN-main` routes change in
  `destination` (into `bus-<NN div 8>`) and `follows_mute` (now `true`, D1's route flags) and in
  nothing else; 138 routes are added (128 sends, 10 returns) and none removed. The fixture is the
  validator's canonical output; `check-console-fixtures.sh` regenerates it byte for byte.
- **Facts test** (`the_bus_send_rows_plan_carries_every_route_unmuted_and_uncompensated`) printed
  `bus_send_plan_facts route_transforms=202 reduction_nodes=11 bank_route_folds=0
  route_ops_per_block=202 delayed_route_edges=0`. Derivations, as asserted: transforms
  `64 + 2 x 64 + 10 = 202` (one per route); reductions `10 + 1 = 11` (each submix input sums 8 or
  64 routes, `main-out` sums 10); 202 timing rows, none compensated (only the limiter has latency
  and every strip carries it once, every tap read lies after `post_insert`); no `route-mute` and no
  `route-follow-zeroed` row (nothing muted). Folds are printed, not asserted.
- **Short-run record** (`the_bus_send_row_prints_its_facts_and_the_validator_pins_them`, 8
  observations, timings not to be read): `workload_kind sixty_four_track_console_sends`, `tracks
  64`, `synthetic_fixture false`, `strip_content eq+compressor+limiter`, `strip_layout
  pre_insert:eq+compressor,post_insert:limiter`, `input_signal tone`, `source_feed bound`,
  `fixture_id fixtures/session/v1/console-sixty-four-track-sends.json`, `render_errors 0`,
  `render_total_forbidden_operations 0`, no meter or bypass group; refused as shortened, accepted
  at `.observations = 1000`, refused there when it names the intended fixture.
- **Preflight** (`--step bus-send-preflight`): PASS, `workload_launches: 0`,
  `records_required: 62`, `sends_fixture_sha256:
  22904bc6d889a4e5224b1470066028f60fba07062a18ae78c66f279e3c8c4712`,
  `sends_fixture_generator_sha256:
  faef2d6c003e77e6822540149cba418e69b84813166c4a9b257ceb00a6a8e4b7` (on the uncommitted tree over
  `f4cb3dc34`); `artifacts/steps/bus-send-preflight/` was not created. No timed step was run.
- **Gates.** 1: release build; `check-console-fixtures.sh target/release/session_validator` ok;
  `cargo test --locked -p session-validator` ok (fixture in the five-stage corpus). 2: `cargo test
  --locked --release -p audit -p bench -p console-workload` 113 passed, 0 failed (floor parity,
  facts, short-run, `every_standing_workload_folds_one_route_per_track` and the strip-row tests
  included); `test-console-benchmark.sh` PASS; preflight PASS. 4: `check-bench-policy`,
  `test-bench-policy`, `check-conformance-boundaries`, `check-workspace-policy`,
  `test-workspace-policy`, `cargo fmt --all -- --check` and workspace clippy `-D warnings` all
  pass. Every positional mutation was checked to land on its intended record (`.[31]` round-two
  session 0, `.[56..61]` round two's meters, observation, placement, automation, mono and mixing
  records; `.[17]` is the sends row).
- **Test value and mutations run** (each red, then restored):
  - row facts test: chaining `BUS_SEND_WORKLOADS` after the strip rows turns it red (a row emitted
    in the wrong position, which would also shift every positional record);
  - facts test: dropping the `console_model` arm (renders the intended fixture), muting
    `ch05-fx-a`, and muting `ch03`'s left fader (a follow-zeroed send) each turn it red; no other
    test compiles this row;
  - short-run record test: pointing the shape branch at the intended fixture, and dropping the
    row's `fixture_id` arm, each turn it red (a record the validator refuses, or one naming the
    wrong fixture, otherwise found only in the timed run);
  - aggregate D5 rule: removing it makes the suite's "a bus-and-send row that rendered the
    standing console row bits" case fail;
  - fixture witnesses: deriving the `fx-b` sends from `pre_fader` fails the `post_fader` witness.
- **Deviations.** The record-validator single-record section also gains three refusals beside
  `session_sends` (the sends kind naming the standing fixture, reported as synthetic, and the
  standing kind naming the sends fixture), within the native part. `scripts/check-cross-targets.sh`
  was not run: it builds no touched crate.

### Verdict follow-up record (after the attempt 1 PASS verdict)

Applied in `2f6b62628` (on `02feade46`, branch `codex/batch-bench`):

- **MINOR-1.** The facts test's doc and gate 2's test-value line say "a muted send or a muted
  track strip", and name the fixture witness that holds a muted submix strip (the verdict's R2: a
  muted submix strip leaves the test green, because its returns into `main-out` carry
  `follows_mute: false` and a strip mute is not a route mute). No model assertion was added: it
  would duplicate the witness.
- **MINOR-2.** `tools/bench/src/console.rs`'s module doc gains `# The bus-and-send row (issue
  #1227)` (the committed fixture, `PlanConfig::BASELINE` with bound sources as the C ABI's
  fan-playback plan, no floor, the in-run `SENDS_PAIR` digest inequality), and the console-strip
  rows are emitted after the bus-and-send row.
- **NIT-1.** The stated-facts test's doc names its unique catch: a non-removal strip edit
  (`HalfMono`-style) or a warmup attached to the row, which its record cannot show.
- **NIT-2.** The facts test's failure message prints only the compensated timing rows.
- **NIT-3.** The `rows` comment in `main` names every `native_session_rows()` list.

## Verdict

- **Attempt 1** (`e7bd95f88`): Sol PASS. Two MINORs and three NITs, all applied above.
  `docs/handoffs/submix-sends-2026-10-02/verdicts/1227-attempt1.md`.

## Dependencies

- None open. Batch K3 (*Let a send follow its source strip's mute live in the browser*, #1224) and
  the VCA batch are on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Freeze the row, fixture and validators here. Nothing is timed in this issue; never invoke a
  timed step.
- Do not quote a projected saving.
- A test that greps source or prose is refused.
- Commit on its own branch from synchronized `main`.
- Attempt budget: five attempts, one adversarial verdict each (`AGENTS.md`).
