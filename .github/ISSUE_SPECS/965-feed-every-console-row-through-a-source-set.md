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

