# Add a sparse plumbing ring row to the console benchmark

**Ruled** (coordinator, 2026-09-26): the companion tooling issue of "Skip unplayed source claims in
the fused Output reduction", defined in that issue's "Console benchmark rows" section. Benchmark
tooling only; no engine source changes.

## Product outcome

Every standing console row feeds all tracks with signal, so a skip that fires on silent tracks shows
only its detection cost. A real session is mostly silent: on the 81-stem dogfood session, 91 % of
track-blocks are exact silence and 28 % play no block at all because the stem has ended. Add one
driver-fed row where 48 of 64 claims play no block, so the silence skip has a row that can move.

## Smallest closable slice

Authorized paths: `tools/console-workload/src/lib.rs` and `tests/`, `tools/bench/src/console.rs`
(row registration), `tools/bench/src/floor.rs`, `scripts/run-console-benchmark.sh`,
`scripts/operator/preflight-console-benchmark.sh`, `scripts/console-benchmark-record-lib.jq`,
`scripts/console-benchmark-validator.jq` and `scripts/test-console-benchmark.sh` (record count and the
new row's floor pin only), and this spec. #928 (`ed1ce679`) had to touch the same four scripts and
`floor.rs` to add its row; follow that commit.

1. Add `Workload::SixtyFourTrackPlumbingRingSparse`, kind `sixty_four_track_plumbing_ring_sparse`,
   to `DRIVER_FED_WORKLOADS` (`tools/console-workload/src/lib.rs:415`), the same pattern #928 used
   for the ring row. Same session and feed as `sixty_four_track_plumbing_ring`, with
   `input_signal: "tone+absent"`.
2. Its `FrozenSourceDriver` answers `played_planes` with `None` for the claims of tracks `ch16..ch63`
   (48 of 64, contiguous), and its `copy_track_input` fills `+0.0` for them, as the driver contract
   requires (`crates/graph/src/lib.rs:1818-1828`). The claims table itself is untouched.
3. Register the row in `tools/bench/src/console.rs`. It has no fixed floor, since its cost depends on
   the data: pin its floor record as `[null, 1, "none", "not_derived"]` in both `floor_row`
   (`tools/bench/src/floor.rs`, an exhaustive match; its `None` arm asserts `NineTrackBaseline`
   today) and `scripts/console-benchmark-record-lib.jq`. Update the record count everywhere it is
   pinned: the runner, the preflight's `records_required`, the validator (which hard-codes `== 48`)
   and the runner self-test.

## Non-goals

No engine change, and no skip. On `main` before the silence slice lands, the row renders the absent
claims through the silence buffer at full cost; that is its baseline.

## Objective gates

1. The row's 64-block digest is pinned in a test, and its output is audible (not all zeros).
2. Source-plane counters on the row read `[copied, played, silence] = [0, 16 x blocks, 48 x blocks]`.
3. Its digest equals a variant whose 48 claims play `Some` blocks of all `+0.0` (the "`None` renders
   as zeros" contract).
4. Every other workload's digest is unchanged.
5. `scripts/operator/preflight-console-benchmark.sh` passes. Do not run the timed runner.
6. fmt, clippy with `-D warnings`, `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace
   --no-deps`, `cargo test -p console-workload -p bench`.

## Dependencies

None. Land it before the silence slice so the slice's benchmark has a baseline.

## Standing rules for the implementer

- Work only from this body. Do not survey the workspace.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not run or tune timings.
- The dense rows remain the budget authority; this row is descriptive only.
