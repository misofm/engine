# Add the dogfood sparse console rows

Draft, slice S2 of the silence architecture issue (A0). Tooling: adds rows, moves no engine bit.
Evidence: `docs/handoffs/silence-2026-09-27/DESIGN.md` sections 3 and 6. Lands before S4 so that
S4's movement is measured on a real-world shape (the owner's rule).

## Product outcome

The console benchmark gains two rows that render a real session's silence pattern through the
production feed, with builtins:

* `sixty_four_track_console_sparse`: the standing console strip (fixture
  `console-sixty-four-track-intended.json`, as `sixty_four_track_console`);
* `sixty_four_track_builtins_sparse`: the same strip with every rack emptied (as
  `sixty_four_track_builtins_only`), which is the shape of the real dogfood session (builtins only).

Each track follows the per-block activity of one dogfood stem: the track's frozen tone block on a
live block, a silent `Some` block (all `+0.0`) on digital silence, and no block (`None`) where the
stem has ended. Tracks take the first 64 stems of `/home/bl/misofm/agents/stems` in file-name byte
order (8.9 % of their track-blocks are live). The rows are the benchmark evidence for S4-S7 and the
place the owner reads a sparse-session number.

## The activity fixture

* `fixtures/activity/dogfood-first-listen-64.json`: for each of the 64 stems, its file name, its
  content hash (the `sha256:` content identity from the mix plan's metadata), its block count at 128
  frames, and its activity as run lengths of `live` / `silent` / `absent` blocks. No audio. Derived by
  a checked-in program (a `tools/stem-hasher` subcommand or a new `tools/activity-masks` binary; the
  handoff's `stem-silence-banking.rs.txt` is the reference implementation) that reads the stems
  read-only and classifies a block as silent iff every PCM byte of both channels is zero.
* **The rendered excerpt.** 6,144 consecutive blocks: 2,048 untimed lead-in blocks (so filters, tails
  and latches are in their real state), then the 4,096 timed blocks. The derivation program chooses
  the excerpt start (a multiple of 512) whose timed window's silent track-block share is closest to
  the whole song's (90.9 % over all 81 stems; state the 64-stem share) and records it in the fixture.
* The derivation program is the fixture's only writer; a test re-derives nothing (the stems are not
  in the repository) but validates the fixture's structure, totals and the excerpt rule.

## Deliverables

1. `Workload::SixtyFourTrackConsoleSparse` and `Workload::SixtyFourTrackBuiltinsSparse` in
   `tools/console-workload/src/lib.rs`, fed through a prepared source set (the feed of #965; a
   driver like `FrozenSourceDriver` that plays tone, a silent block or `None` per claim per block
   from the fixture), `input_signal: "dogfood-activity"`, warm-up = the 2,048-block lead-in.
2. Registration in `tools/bench/src/console.rs`; `scripts/console-benchmark-record-lib.jq`
   (`session_kinds`, the per-kind facts, a driver-fed list), `scripts/console-benchmark-validator.jq`
   (record counts), `scripts/test-console-benchmark.sh` (kind table, tolerance table, reject cases);
   `tools/bench/src/floor.rs` excludes both rows (data-dependent, no fixed floor).
3. **Mean** in the record beside the percentiles (a sparse row's cost varies block to block; the
   mean is the number that states CPU saved). If the runner has no mean field, add
   `mean_us_per_block` to these two rows' records only.
4. Native only. The wasm console arm addresses rows by index (`WORKLOADS` is append-only); carrying
   these rows into it is a separate change.

## Authorized paths

`tools/console-workload/src/lib.rs`, `tools/console-workload/tests/` (new pins),
`tools/bench/src/console.rs`, `tools/bench/src/floor.rs`, the three console-benchmark scripts above,
`fixtures/activity/` (new), the derivation program's crate or script, this spec.

## Objective gates

1. **Pins.** Each row's 64-block and full-excerpt digests are pinned; its output is audible (nonzero
   energy in the timed window); its source-plane counters show claims read in place, none copied, and
   `None` exactly where the fixture says `absent`.
2. **Feed equivalence.** Each row renders the same bits as a bound-feed twin built from the same
   fixture (harness-only, not a row), which proves the source set serves exactly the fixture.
3. **Fixture.** Structure, totals (block counts, run sums), the 64-stem live share, and the excerpt
   rule are validated by a test; the fixture's stem hashes match the mix plan's.
4. **Scripts.** `bash scripts/test-console-benchmark.sh`, the preflight and the validator pass; no
   timed run by the implementer.
5. fmt, clippy with `-D warnings`, `cargo test -p console-workload -p bench`.

## Console benchmark rows

Adds the two rows. No existing row or digest changes.

## Dependencies

#965 (every row fed through a source set). S1 is optional: with it the driver sets the silence bit
on silent blocks; without it the silent blocks are plain `Some` zero blocks.

## Rulings needed

Checking in derived activity masks of the private dogfood stems (no audio): allowed?

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Do not run the timed runner; do not quote a projected saving.
- Commit on `codex/<issue>-<slug>`.
