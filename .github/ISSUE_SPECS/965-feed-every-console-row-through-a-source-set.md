# Feed every console row through a source set

## Product outcome

Every console benchmark row except the ring-fed one is bound-feed: `SourceFeed::Bound` means one dispatched unit per track per block that copies the track's frozen block into its arena buffer (`tools/console-workload/src/lib.rs:426-427`). No host pays that: the C ABI, the native host and the browser all feed through source sets, where #918 gathers in place and #936 skips the input unit. So every row except the ring row measures a per-track copy and 64 dispatches that production never runs. The owner's rule is that benchmarks measure only real-world paths. Found by the builtins-less removal verification (`docs/handoffs/builtins-less-removal-2026-09-27/VERIFY.md` section 8).

## Smallest closable slice

Make the production feed (a `FrozenSourceDriver` source set bound with `into_bound_with_source_set`, as the re-based ring row does after #956) the feed of every console row, and remove the bound feed from the rows. For each row the 64-block digest must equal its bound-feed digest (the bound feed and the source set serve the same words). Update record shapes, validators, floor pins and counts as #881 and #928 did.

## Objective gates

- Every row's digest equals its pre-change digest; every row's source-plane counters show claims read in place by bank gathers, none copied.
- `test-console-benchmark.sh`, the wasm console validator and self-test, and the preflight pass; no timed run by the implementer.

## Dependencies

#956.


## Decision record (attempt 1, 2026-10-24)

- Scope note accepted by root: every row binds `into_bound_with_source_set` through `FrozenSourceDriver`;
  `SourceFeed::Bound`, the bound arm and its helpers are deleted. `sixty_four_track_gain_pan_only`
  (bit-identical twin of `gain_pan_ring`, a feed no host runs) is retired, with its in-run pair
  assertion and pair tests. Records 74 -> 72, session records 46 -> 44; `source_feed` stays and is
  pinned `played_planes` on every session record; a `bound` record is refused.
- The wasm console arm went with #1039 (R9), so the "wasm validator" gate is moot and the
  `WORKLOADS` append-only comments were deleted.
- Fact: every one of the 22 surviving rows reads every claim in place (`[0, tracks * 64, 0]`
  source-plane counts over 64 blocks), except `sixty_four_track_idle`, whose all-silent banks skip
  their gather (`[0, 0, 0]`). No row copies a claim, so the zero-copy gate is exact on every row.
- Fact: the 64-block digest of all 22 surviving rows is unchanged (pre/post table in the PR).
- Fact: the mono census moves from `[65, 129]` to `[129, 129]` (half-mono `[193, 193]`): the
  claimed inputs lower to the engine's `SourceInput` kind, which reports `SYMMETRIC`. The bank
  premise (`bank_symmetry_counters`, `[64, 64]`) and every record field are unchanged.
- Paths beyond the scope note's list, forced by retiring the row: `tools/bench/src/floor.rs`,
  `tools/console-workload/tests/chain_shape.rs`, `scripts/operator/run-console-benchmark.sh`
  (record count 74 -> 72). `docs/rulings/effect-floor-accounting.md` still names `gain_pan_only`
  (left for root).
