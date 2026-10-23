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
