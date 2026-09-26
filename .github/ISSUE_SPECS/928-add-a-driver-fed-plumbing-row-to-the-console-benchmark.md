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
