# Close the remaining live-lane cost on an unedited console

Weekly optimisation pass. Successor of #1763 (2026-10-23).

## Problem

#1763 cut the cost of product live lanes on a block nobody edits from +7.6 / +7.9 % to +2.99 / +1.76 % on the standing 64-track console, and the app-shape session stays at +2.93 / +2.19 % (`artifacts/steps/opt5-1763/`, uncontrolled run, round-to-round spread about 0.7 us). Named but unmeasured reasons:

1. The strip input, fader and matrix banks still walk every lane's dirty words on every block (step 1 measured fader/matrix plus strip input at +0.75 / +0.93 us). #1763's quiet-publication skip covers only the live effect stage.
2. The observation-queue drain loads one producer cursor per lane per block; it cannot skip, because the push is the acknowledgement (#1584 D6a).
3. The app shape did not move under #1763 step 2, so its +1.8-2.4 us is not the effect cell walk; it is not yet explained.

## Smallest closable slice

1. **Measure first.** Split the app-shape arm by lane group (as #1763 step 1 did for the console) and record which group carries its gap.
2. Apply #1763's quiet-publication skip to the builtin banks' drains (`crates/builtins-compiler/src/lib.rs`), with the same argument (a write above the snapshot's publication waits in its dirty word or deferred mask) and the same carry and swap clears.
3. If the observation-queue drain is measurable, find an exact cheaper form (for example one bank-level "any queue non-empty" word) without changing when a push is acknowledged.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (the builtin drains), `crates/rack/src/lib.rs` (the observation drain only)
- `tools/bench/src/console.rs`, `tools/console-workload/src/lib.rs` and the console-benchmark scripts (the app-shape split arms)
- this spec

## Objective gates

1. The app-shape split, recorded before any engine change.
2. Class A: every console digest unchanged.
3. Render allocates nothing; realtime-policy.
4. One descriptive run: the product-lane row and the app-shape arm within 2 % of their standing rows, or the remaining gap recorded with a measured reason.

## Test value

One assertion in #1763's quiet-slot test form per new skip: it turns red when a skipped builtin bank misses a write under a newer publication.

## Dependencies

#1763 and #1765 merged.

## Decision and evidence record

### Scope decisions (root, 2026-10-24)

- **Step 1 and gate 1 retired** (the app-shape split arms). Measured reason: the app-shape gap is inside the round-to-round spread since #1107. opt8-1072 reads the app shape at +0.52 / +1.93 %, opt6-1107-fix at +0.70 / +0.38 %, and the fader/matrix arm alone moves from -0.48 to +1.45 us between rounds of one run, so a lane-group split cannot resolve sub-1 us groups on this host. This also removes the path overlap with #1783 (`tools/bench`, `tools/console-workload`, `scripts/`), which are no longer authorized here.
- **Item 3 retired** (the observation-queue word). A bank-level "non-empty" word needs a producer-side store in `crates/effect-contract/src/live.rs` and a new cross-thread protocol, for a gain below the noise floor (one `available_at_entry` Acquire load per effect lane per block). The owner's rule is that small gains do not justify fragile machinery. No successor.
- **Gate 4's 2 % line is reporting, not the close condition.** The no-lane row itself moves about 0.7-1.3 us between rounds.
- Class A: no rendered bit moves, no summation order changes, no effect kernel changes.

### Implementation (item 2)

`BuiltinBankProcessor`, `FaderBankProcessor` and `MatrixBankProcessor` (`crates/builtins-compiler/src/lib.rs`) carry `quiet_at: Option<u64>`. Each reader's `apply_latest` returns `Ok(true)` when its `DirtyReader::pending` was 0. A block drain that walks every lane and finds every lane quiet marks the snapshot's publication. The next block drain under the same publication skips the lane walk (`quiet_walk_skips`); `LiveSnapshot::ALL` never skips. The fader's `read_fader_offsets` still runs every block. The mark is cleared by `swap_builtin_lanes` (both banks), lane adoption and the carry drains (`drain_for_carry`, `apply_pending`).

Exactness is #1763's argument: a write at or below publication `S` is stored and marked before `S` is published, so the marking walk under `S` saw it; a write above `S` waits in its dirty word or the deferred mask for the first block under a newer publication, which walks. `CarryMarks::begin` already ran in the marking walk, so a skipped walk changes no carried count. The clears on swap and adoption are defensive: on reachable paths a swapped or adopted reader has nothing pending at or below the current publication either, but the mark is a fact about this bank's readers, and these methods change the readers.

### Evidence (attempt 1)

- Test: one assertion block added to `a_drain_applies_the_newest_value_its_snapshot_covers_and_counts_the_rest` (two blocks under one publication, then a monitor write, a new publication under the same revision, applies at the next block; a write between two `ALL` blocks applies at the second). Mutants: mark compared by revision, red; `ALL` check removed from the skip, red.
- The processors grow by `Option<u64>`, so `fixtures/graph/v1/direct-route.resources.json` and `fixtures/graph/MANIFEST.tsv` change (`builtin_bank_bytes` 7223 -> 7255, plan bytes 75191 -> 75223). `fixtures/graph` is outside this issue's authorized paths; see the hand-back.
- Descriptive run (gate 4): `scripts/operator/run-console-benchmark.sh --step opt9-1778` under the exclusive lock refused before any timing, `loadavg_above_ceiling` (loadavg 3.64 5.94 7.32, ceiling 0.50; up to eight streams share the host). Recorded and not retried, so there are no opt9-1778 records or digests. A first invocation refused earlier at the clean-tree check (an uncommitted spec edit), also before timing. The console digests are covered by `graph-compiler` and `console-workload` tests, which pass.
