# Add a driver-fed plumbing row to the console benchmark

Tooling only. Drafted from `DRAFTS/PLAN.md`, Part 3; coordinator ruling: `sixty_four_track_plumbing_only`
is not refed, a new row is added beside it.

## Product outcome

Every console row binds its track inputs to `FrozenGraphSource`, a host-supplied
`GraphRuntimeProcessor` that copies a frozen block into the arena. The production feed is the
source set: a `GraphPreparedSourceSetDriver` whose `copy_track_input` fills the claim's arena
buffer, or, since #917/#918, whose `played_planes` lends the played block for an in-place read.
No row measures that feed, so neither #918 nor "Read plain-strip sources in place from the played
transfer block" can show on the benchmark. Add `sixty_four_track_plumbing_ring`: the same
builtins-less session as `sixty_four_track_plumbing_only`, fed through a `FrozenSourceDriver`
that lends played planes, with its digest pinned equal to the bound-feed row's. The sealed row is
untouched: `tools/bench/src/floor.rs` asserts it is the floor of the whole table and the paired
arms compare it across commits.

## Root evidence

- `tools/console-workload/src/lib.rs:1513-1575` `FrozenGraphSource` and `:1620` its binding to
  every `TrackStage::Input`; `:869-873` the builtins-less compile path; `:783` `build`, `:821`
  `build_full`; `:242`, `:336`, `:428`, `:485`, `:504`, `:533` the `SixtyFourTrackPlumbingOnly`
  registrations (kind, strip, content, layout strings) the new row mirrors.
- `crates/graph/src/lib.rs:1778` `GraphPreparedSourceSetDriver` (`claim_count`, `begin_block`,
  `copy_track_input`, `provides_played_planes`, `played_planes`); `:1843`
  `GraphPreparedSourceSet::new`; the bind entry `into_bound_with_source_set` (production call
  site `crates/host-core/src/prepare.rs:1405`); `crates/graph/src/lib.rs:2200-2231` the
  lent-claims path that decides which claims are read in place.
- `crates/graph/tests/rt10_source_in_place_alloc.rs:38-110`: the `Played` driver, about 70
  lines, is the shape `FrozenSourceDriver` takes (per-claim frozen planes, `copy_track_input`
  copies them, `played_planes` lends them).
- `tools/bench/src/console.rs` registers the rows and prints the records; the validators are
  `scripts/console-benchmark-validator.jq`, `scripts/console-benchmark-record-validator.jq`,
  `scripts/console-benchmark-record-lib.jq`, exercised by `scripts/test-console-benchmark.sh`;
  `tools/bench/src/floor.rs:85-111`, `:249-255` the plumbing row's floor entry and its
  no-control rule.
- Measured (`DRAFTS/PLAN.md` table A): on the bound-feed row the 64 `FrozenGraphSource` units are
  8,025 cycles per block; on the driver-fed row that work is the `source set` phase (the copy
  loop), which draft C removes for claims read in place.

## Smallest closable slice

Authorized paths: `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` (the driver
and the builder only), `scripts/console-benchmark-validator.jq`,
`scripts/console-benchmark-record-validator.jq`, `scripts/console-benchmark-record-lib.jq`,
`scripts/test-console-benchmark.sh`, `tools/bench/src/floor.rs` (only if the row needs a floor
entry), and this spec.

1. `FrozenSourceDriver` in `tools/console-workload/src/lib.rs`: one `(left, right)` frozen
   block per claim in claim order (built from `source_block` and `channel_mappings` exactly as
   `FrozenGraphSource::from_block` is), `claim_count`, `begin_block` (no-op beyond the frame
   check), `copy_track_input` (copies the planes), `provides_played_planes() == true`,
   `played_planes` (lends them). Realtime: no allocation, lock or syscall in any of the five.
2. `Workload::SixtyFourTrackPlumbingRing` with kind `sixty_four_track_plumbing_ring`, strip
   `PlumbingOnly`, the same content/layout strings, and a `feed` discriminator the record carries
   (`"source_feed": "bound"` on every existing row, `"played_planes"` on this one). `build_full`
   binds the row's `Input` nodes through `GraphPreparedSourceSet::new` and
   `into_bound_with_source_set` instead of `FrozenGraphSource`; everything else on the
   builtins-less path is shared.
3. Register the row in the bench beside `sixty_four_track_plumbing_only`; extend the validators'
   row list and the record schema for `source_feed`; `floor.rs`: the new row costs at the plumbing
   inventory (4 lane-ops) with `floor_control_row: none`, and the floor-of-the-table assertion
   stays on `sixty_four_track_plumbing_only`.

## Non-goals

No change to `sixty_four_track_plumbing_only` or to any other row's binding, record or digest. No
change to `crates/graph` or `crates/source`. No arm registration in `run-console-benchmark.sh`
(the plumbing-floor batch's arm pair is registered at the batch boundary, per `PLAN.md`).

## Objective gates

1. console-workload test: `sixty_four_track_plumbing_ring` and `sixty_four_track_plumbing_only`
   render byte-identical digests over 64 blocks (`hash_output`), bank shape `[0, 0]` on both,
   `bank_transposes() == 0` on both; the new row's plan reports `test_only_source_plane_counts()`
   with 64 in-place bindings per block once draft C lands and 0 before it (pin the pre-C value
   now; C's brief moves the pin).
2. `scripts/test-console-benchmark.sh` (the validator suite) green with the new row in every
   record shape; `scripts/check-console-benchmark-fixture.sh`; `scripts/check-bench-policy.sh`.
3. `cargo test -p bench -p console-workload`; the bench's shortened-run self-test prints the new
   row and the validator refuses a record missing `source_feed`.
4. The realtime audit on the new row: `render_total_forbidden_operations == 0` in the short run.

## Console benchmark rows

Adds `sixty_four_track_plumbing_ring`. Moves nothing.

## Dependencies

None. Independent of drafts A and B; run in parallel with A. Must be merged before the
batch-boundary measurement so that C has a row to show on.

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- The driver's five methods are on the render path: allocation-free, lock-free, syscall-free
  (`scripts/check-realtime-policy.sh` is mandatory); no `unsafe`.
- The two plumbing rows must render the same bits; a differing digest is a harness defect, never
  a finding.
- Run `cargo fmt --all --check`, `cargo clippy --locked --workspace --all-targets --all-features
  -- -D warnings`, and the gates above before every checkpoint. Commit on a
  `codex/<issue>-<slug>` branch from synchronized `main`.
- Do not quote a projected saving; the row is measured once at the batch boundary.
- Source of these findings: `.github/ISSUE_SPECS/DRAFTS/PLAN.md`,
  `.github/ISSUE_SPECS/918-gather-banked-source-inputs-from-the-played-transfer-block.md`.

## What the implementer will hit

- `WORKLOADS` (`lib.rs:336`) drives every "all rows" test in `tools/console-workload/tests/`;
  adding a row there means every census/shape test iterates it, and the chain-shape gates that
  name the plumbing row explicitly (`the_plumbing_row_binds_no_strip_at_all`) need the ring row
  added beside it, not folded into the iteration.
- `build_full`'s `plumbing_only` assertion (`lib.rs:834-838`) must cover the new row (no console
  facility on a builtins-less plan).
- The mono fixture's channel mapping is honoured in `FrozenGraphSource::from_block`; the driver
  must honour it the same way or the digest gate fails on nothing but the harness.
- `floor.rs` pins that no row is costed below the plumbing row; a second row at the same
  inventory is equal, not below, and the assertion must be written as `>=`, which it should
  already be -- check before adding the entry.

## Attempt 1 evidence

Terra, branch `codex/928-driver-fed-plumbing-row` from `3c93469d`. Implementation checkpoint
`ed1ce679`; this record is the commit after it. Local only: nothing pushed, no GitHub change.

### Design

- **The row.** `Workload::SixtyFourTrackPlumbingRing`, kind `sixty_four_track_plumbing_ring`
  (`tools/console-workload/src/lib.rs`). `strip()` is `Strip::PlumbingOnly`, so it takes the same
  strip edit, the same `build_full` assertion (no console facility on a builtins-less plan) and the
  same `GraphCompiler::compile` path as `sixty_four_track_plumbing_only`; `strip_content` and
  `strip_layout` are `plumbing`, fixture the intended one, `synthetic_fixture: true`, 64 tracks,
  tone. The feed discriminator is `pub enum SourceFeed { Bound, PlayedPlanes }` with
  `Workload::source_feed()`; `PlayedPlanes` for this row only. `SourceFeed::name()` is the record
  string (`bound` / `played_planes`).
- **The driver.** `FrozenSourceDriver { claims: Box<[FrozenGraphSource]> }`, one frozen block per
  claim in claim order. Each block comes from `frozen_track_source`, extracted from
  `source_binding` so the bound feed's processors and the driver's claims are built by the one
  function (track id -> `SourceSignal::block` -> `FrozenGraphSource::from_block` with the track's
  `channel_mappings` entry). `claim_count` is the slice length; `begin_block` refuses a frame count
  other than `QUANTUM` and does nothing else; `copy_track_input` refuses an out-of-range claim or a
  destination that is not one quantum, then copies both planes; `provides_played_planes` is
  `true`; `played_planes` lends `(&left[..], &right[..])` or `None` past the end. The impl is a
  `REALTIME_POLICY_BEGIN`/`END` region, so `scripts/check-realtime-policy.sh` scans it (53 regions
  in 15 files at `3c93469d`, 54 in 16 now). No `unsafe`. Resource report: the one boxed slice
  (64 x 1 KiB) as `overhead_bytes` = `total_engine_owned_bytes` = `largest_allocation_bytes`,
  `pcm_payload_already_charged_bytes: 0` (no session declaration charges the frozen tone).
- **The bind.** In `build_full`'s builtins-less branch, `SourceFeed::PlayedPlanes` collects every
  `TrackStage::Input` of `required_bindings` as a `GraphSourceInputClaim`, sorts them (the set
  requires strictly ascending claims), builds the driver over them, wraps it in
  `GraphPreparedSourceSet::new(graph.envelope, claims, report, driver)` and binds with
  `PreparedGraphPlan::bind_with_source_set`. Every other required node goes through the same
  `source_binding` the bound feed uses (identity), so the feed is the only difference in the two
  rows' bindings. The builtins branch asserts `source_feed() == Bound`.
- **The bench.** `tools/bench/src/console.rs` measures `WORKLOADS` then `DRIVER_FED_WORKLOADS` as
  one list (the floor control lookup searches that list), asserts in-run that the two plumbing rows'
  `output_sha256` agree before emitting any session record, and emits `"source_feed":"<name>"`
  after `input_signal` on every `console_session` record. `SessionMeasurement` gained
  `run_for(workload, observations)` and an `observations` field the record now prints (the #914
  pattern), so a shortened run says it was shortened; the runner's run is unchanged at 1000.
- **The floor.** `tools/bench/src/floor.rs`: the ring row shares the plumbing row's `FloorRow`
  (4 lane-ops, width 1, `control: None`, basis `docs/rulings/effect-floor-accounting.md:
  plumbing`). The floor-of-the-table test was already `>=` and stays anchored on
  `SixtyFourTrackPlumbingOnly`. The floor tests now iterate `WORKLOADS` chained with
  `DRIVER_FED_WORKLOADS` (`session_rows()`), so the Rust/jq floor-table parity covers the new key.

### The digest pin

Four statements of "the driver-fed row renders the bound-feed row's bits":

1. `console_workload::tests::the_driver_fed_plumbing_row_renders_the_bound_rows_bits`: 64 blocks
   of both rows via `hash_output`, equal digests, and both audible (a silent pair would be equal
   for nothing).
2. `bench` `main`: the in-run `assert_eq!` over the runner's 1000 blocks, before any record is
   printed.
3. `scripts/console-benchmark-validator.jq`: the four `console_session` digests of the two
   plumbing kinds (two rounds each) must be one value.
4. Scratch measurement (a temporary release test, deleted, not committed): the 1000-block digest of
   every row, computed the way `SessionMeasurement` computes it (warm-up blocks unhashed, then
   observations 0..999). All sixteen `WORKLOADS` digests equal round 1 of
   `artifacts/copy-removal/console-benchmark.accepted.jsonl` byte for byte (so the
   `frozen_track_source` extraction moved no row), and `sixty_four_track_plumbing_ring` is
   `38ebb48908b62b9770ac7df1a8f6f2427bfd15eb849429219f8705a3926084c3`, the plumbing row's sealed
   digest. Host: the EPYC 7313P, `x86-64-v3`, `Simd8`.

### Validators

- `scripts/console-benchmark-record-lib.jq`: `source_feed` added to `session_keys` (so it is
  required on both session shapes, `session_floor_keys` included); `session_kinds` gains the ring
  kind (seventeen); `floor_pins` gains it at `[plumbing_lane_ops, 1, "none", "...: plumbing"]`;
  `session_kind_shape` gains its branch (the plumbing row's six facts); new
  `driver_fed_kinds` / `session_source_feed` pin the feed per kind (`played_planes` for the ring
  kind, `bound` for every other), called from `session_record_valid`.
- `scripts/console-benchmark-validator.jq`: forty-eight records, thirty-four session records,
  forty-eight unique `[record, kind, round]`, and the plumbing digest-pair pin above.
- `scripts/console-benchmark-record-validator.jq`: unchanged (it is `include` + the library entry).
- `scripts/test-console-benchmark.sh`: the base session carries `source_feed: "bound"`; new bases
  `session_ring` and `session_floor_ring` accepted; `session_ring` joins the per-key deletion/null
  sweep; feed mutations (a record from before `source_feed`, a floor record without it, the console
  or plumbing row claiming `played_planes`, an unknown feed, the ring claiming `bound` or missing
  its feed, the ring claiming the builtins layout or content, a checked-in fixture, nine tracks,
  the mono fixture, silence), floor mutations (the ring at the identity inventory, citing it,
  isolating against the plumbing row, the plumbing row isolating against the ring); the synthetic
  aggregate set carries the ring row (digest equal to the plumbing row's) and the index map shifts
  by one per round (verified by printing the set: 16 and 40 are the ring rows, 24 is round two's
  first record, 43-47 its meters/observation/placement/automation/mono records); new aggregate
  cases: a set missing the ring row, the ring's digest moved in both rounds, the plumbing row's
  digest moved in both rounds (both refused by the pair pin alone), and the pair moving together
  (accepted).
- `scripts/check-console-benchmark-fixture.sh` checks the retired fixture's shape only and needed
  no change; it passes.

### Tests (all green)

- `console-workload` (lib): `the_driver_fed_row_states_the_plumbing_rows_facts_and_its_own_feed`;
  `the_driver_fed_plumbing_row_renders_the_bound_rows_bits` (gate 1: equal digests over 64 blocks,
  `bank_shape [0, 0]`, `bank_transposes 0`, `bank_route_folds 0`, collapse `[0, 0]`, transitions
  `[0, 0, 0]` on both; `test_only_source_plane_counts` `[0, 0, 0]` on the bound row and
  `[64 * 64, 0, 0]` on the ring row -- 64 claims copied per block, **0 read in place: the pre-#927
  pin**, which #927 moves to `[0, 64 * BLOCKS, 0]`; zero forbidden operations under the audit on
  both); `the_frozen_source_driver_lends_the_words_it_copies` (the five methods called directly
  inside `audit::in_render_scope` over the half-mono model, whose even tracks map `(0, 0)`: zero
  forbidden operations, lent planes equal copied planes, both equal the claimed track's
  `source_block` through its mapping computed from the model position, 32 mono claims with
  `left == right` and 32 stereo with `left != right`, the four refusals).
- `bench`: `console::tests::the_driver_fed_plumbing_row_prints_its_feed_and_the_validator_pins_it`
  (gates 3 and 4: eight-block runs of both plumbing rows through `SessionMeasurement::run_for`,
  equal digests, `render_errors 0`, `render_total_forbidden_operations 0`; prints the ring record;
  each record states its feed exactly once and `observations: 8`; through `jq` and the real
  record validator: the shortened record refused, both records accepted at the frozen count, both
  refused without `source_feed`, each refused with the other's feed);
  `floor::tests::the_driver_fed_plumbing_row_is_costed_at_the_plumbing_floor_and_isolates_nothing`;
  the existing floor tests over `session_rows()`, including
  `rust_and_jq_floor_tables_have_exact_key_value_parity`.
- `cargo test -p bench -p console-workload`: 64 + 3 + 4 + 22 + 3 passed (2 ignored, the
  pre-existing `plumbing_profile` diagnostics). `chain_shape`'s 22 are unchanged and still pass.

### Gates

- `cargo fmt --all --check`: clean. `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`: clean. `RUSTDOCFLAGS='-D warnings' cargo doc --locked -p
  console-workload -p bench --no-deps`: clean.
- `bash scripts/test-console-benchmark.sh`: PASS. `bash scripts/check-console-benchmark-fixture.sh`:
  ok. `bash scripts/check-realtime-policy.sh`: ok (54 regions, 16 files).
- `bash scripts/check-bench-policy.sh` and `bash scripts/test-bench-policy.sh`: **fail at
  `3c93469d` already**, before this change: `tools/console-workload/tests/plumbing_profile.rs:41`
  (added by `f49fdbde`, the cycle's diagnostic commit) defines `fn percentile(`, a second
  nearest-rank percentile owner. Confirmed on a pristine `git archive 3c93469d`. With that one
  function renamed in a scratch copy of this tree, `check-bench-policy.sh` reports ok (1
  percentile owner, 3 timer subjects) and `test-bench-policy.sh` reports its mutations ok, so
  nothing in this change trips either. The file is outside this issue's paths.
- Not run, per the brief: `scripts/run-console-benchmark.sh`. The row's first record is the batch
  boundary capture; its in-run assertion refuses the run if the two plumbing digests differ.

### Mutations (each applied alone, restored, all killed)

- Validators (`scripts/test-console-benchmark.sh` after each): drop `session_source_feed` from
  `session_record_valid` (9 cases red: every swapped, unknown or nulled feed); accept the
  pre-`source_feed` key set (8 red: every removed or nulled feed); drop the aggregate pair pin (the
  two moved-digest cases red, 2); drop the ring's `floor_pins` entry (its floor record red, then
  the suite's floor-aggregate builder errors on the missing pin, exit 5); aggregate count back to
  46 (3 red); empty `driver_fed_kinds` (7 red); drop the ring's `strip_layout` pin (2 red).
- Rust: `played_planes` lends the left plane twice (driver test red -- the digest gate cannot see
  it before #927); `copy_track_input` swaps the planes (driver test and digest gate red); an
  allocation in `begin_block` (SIGABRT from the audited allocator in the gate-1 test, and
  `check-realtime-policy.sh` refuses a `.to_vec()` in the region); `provides_played_planes` false
  (driver test red); the ring row reporting `SourceFeed::Bound` (facts test, and the gate-1 pin
  reads `[0, 0, 0]`); the record always printing `bound` (bench test red); the record printing
  `OBSERVATIONS` instead of `self.observations` (bench test red).

### Deviations and follow-ups

1. **The row is not in `WORKLOADS`.** It is `DRIVER_FED_WORKLOADS`, emitted by the bench after the
   sixteen. `WORKLOADS` is the wasm console arm's address space (`miso_console_prepare(index)`,
   `tools/wasm-console` iterates it, `scripts/wasm-console-benchmark-validator.jq` pins sixteen
   kinds); appending there would have changed the wasm arm and broken its next capture, and would
   have required `tools/console-workload/tests/chain_shape.rs` edits
   (`every_standing_workload_folds_one_route_per_track` asserts 64 folds,
   `the_folded_master_is_the_reductions_own_bits` builds a metered arm the builtins-less path
   refuses) -- all outside this issue's paths. The census facts those tests state of every row are
   asserted of the ring row in gate 1. Carrying the row into the wasm arm, or into `WORKLOADS`, is
   a separate change to that arm's host and validator.
2. **`bind_with_source_set`, not `into_bound_with_source_set`.** The latter exists only on the
   builtins artifact (`crates/builtins-compiler/src/lib.rs`), which a builtins-less plan does not
   have; it delegates to `PreparedGraphPlan::bind_with_source_set` after appending the artifact's
   private builtin bindings, of which this path has none.
3. **Sealed records.** `source_feed` is required (gate 3), so a pre-#928 `console_session` record
   no longer passes the current record validator. Every sealed record was validated at its own
   capture by its own tree's validators; no gate re-validates `artifacts/` with the current ones
   (`artifacts/issue368-floor-recount/verify-historical-repricing.sh` pins its validators at a
   revision). The batch-boundary capture validates against these.
4. **Out-of-path follow-ups.** (a) `plumbing_profile.rs`'s `fn percentile` (bench policy, above).
   (b) `scripts/check-realtime-policy.sh`'s floors (12 files, 41 regions) could be raised to 16 and
   54; they are `>=` floors and pass as they are. (c) `docs/rulings/effect-floor-accounting.md`
   does not yet name the ring row; its floor basis reuses the `plumbing` inventory string.
