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

## Evidence: 64-block digests, pre (9ddfe1fb8) and post (attempt 1)

`SessionRuntime::hash_output` over 64 blocks, release, this host's backend. Equal on all 22
surviving rows; the retired `gain_pan_only` read `01e465a7...`, the ring row's digest.

| row | pre = post sha256 |
|---|---|
| `nine_track_baseline` | `e7c6ef01770ab7da98d4b793a6a817bd50a4dfe682321ecfc023ddc275285a80` |
| `nine_track_ragged_strip` | `33995dfc62a9158d4c90e5cb07d5e379bbb75337234886ab2c3719c1895a210c` |
| `one_twenty_eight_track_stretch` | `8fa5b08d370bef83f9fbbbb655f844ec304e237184750db59b5819072216c73a` |
| `sixteen_track_strip` | `11af3f2b1e36125abf1bb07755d0861d7319c9caaaa4d29d5b20e412198854b8` |
| `sixty_four_track_app_shape` | `33b53571775dbce4884b5829d26ecbca42859dfde6169f900328467c761d48b0` |
| `sixty_four_track_builtins_only` | `b63eccd09c19eb7a6e0608144024ac5b14c7d5f7d1c56012cbbd49d6aad8f7f0` |
| `sixty_four_track_compressor_only` | `3890551c423c0fa1feca22d1fbf57c56b395c2254f6d0fc1fa653743696749ea` |
| `sixty_four_track_console` | `b6671122275a7829fdd0dc653e5930637c6453023599822820bb1bcbc564fa67` |
| `sixty_four_track_console_half_mono` | `1f4c885f3811075cd5166d2405ee03c67b307253db876cc9680a31a8055c836f` |
| `sixty_four_track_console_legacy` | `add57c85031ac996ef80497ba73207302a2fde29a6c438858deb8aa8c5613791` |
| `sixty_four_track_console_metered` | `b6671122275a7829fdd0dc653e5930637c6453023599822820bb1bcbc564fa67` |
| `sixty_four_track_console_mono` | `0db1c16c8ba93d645acd5a3f1221112c5c93888ac205b24a155029f6b0aafc65` |
| `sixty_four_track_console_mono_dual` | `0db1c16c8ba93d645acd5a3f1221112c5c93888ac205b24a155029f6b0aafc65` |
| `sixty_four_track_console_sends` | `5a79110007f66da1239a302ea6846e72a58e579c795db51b072b18db16ec5814` |
| `sixty_four_track_console_sparse` | `a728ee5bf4a400df245128edadc4d32e44ce836c6ace715662c1eac1bc5d7b48` |
| `sixty_four_track_dispatch_only` | `15688888612d161e507bc400b9eed356fc1776797c8c66ca52d1e7c9114d3a2d` |
| `sixty_four_track_eq_comp_simd1` | `add57c85031ac996ef80497ba73207302a2fde29a6c438858deb8aa8c5613791` |
| `sixty_four_track_eq_only` | `9b2c56a1da62ebda8aef595973870ea077477d209aacdcd6321637e949d5c828` |
| `sixty_four_track_gain_pan_ring` | `01e465a797036fb4267e895d9319a911bc108d554705d268d9a84a2e2e2dfdb4` |
| `sixty_four_track_idle` | `de2f256064a0af797747c2b97505dc0b9f3df0de4f489eac731c23ae9ca9cc31` |
| `ten_track_ragged_strip` | `4e334d3d4e545968e480c605cf64af2a65b0011e667524b5e7505e8d95811aec` |
| `thirteen_track_ragged_strip` | `7a7b1a6db67bdd59dc1529bf2cf1470798d9e9cd60c6b070fbfd7c652e079bae` |
