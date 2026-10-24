# Add a signed gain-change observation for effects that boost

Filed by root from the #17 S1 verification (backlog sprint, 2026-10-24). Blocks #17 S2 (registering the dynamic EQ).

## Finding

The dynamic EQ's upward mode boosts a band, but the only observation kind it can declare is `GainReductionDb` with the `PeakMagnitude` fold (`crates/dynamic-eq/src/lib.rs` near line 493). An upward band therefore reads as a reduction of the same size on any meter a UI subscribes to: a 6 dB boost shows as "6 dB GR".

## Smallest closable slice

1. Add a signed observation kind (for example `GainChangeDb`, the next free value) and a signed fold (the value of largest magnitude in the window, sign kept) to `crates/effect-contract`, `docs/EFFECT_OBSERVATION_V1.md` and the host fold on both hosts (decision 17: one core function).
2. The dynamic EQ declares the signed kind; the compressor, gate, de-esser and multiband keep `GainReductionDb` (no change, class A).
3. One test in effect-contract for the signed fold (red if the fold drops the sign or picks the wrong extreme), and one host parity row.

## Paths

`crates/effect-contract/src/**`, `docs/EFFECT_OBSERVATION_V1.md`, the host fold in `crates/host-core` (and its browser and C ABI surfaces if a kind list is exported), `crates/dynamic-eq/src/lib.rs` (the declaration), this spec. After #17 S1 merges.

## Decision record (attempt 1, 2026-10-24)

- Root: `ObservationKind::GainChangeDb = 2`, `ObservationFold::PeakSigned = 3`. `PeakSigned` keeps the value of
  largest `|x|` in the window with its sign; equal magnitude keeps the earlier value (strict `>`, as
  `PeakMagnitude`); the `0.0` window reset is beaten by any nonzero reading.
- Root: the fold arm is in `ObservationLane::accumulate` (`crates/effect-contract/src/live.rs`), the one function
  the per-node and bank paths call on both hosts. Paths narrowed from "host-core" accordingly (scope note).
- Root: additive to V1. No reserved value, no wire, protocol, C ABI or meter-frame change. The C ABI has no
  observation read surface, so nothing is exported there.
- Root: host-web keeps `observed_decibels` (its clamp honours a negative minimum) and adds
  `strip_reduction_decibels`, the per-strip frame-slot contribution: a `GainReductionDb` window as before
  (larger lane, then convert), a `GainChangeDb` window `max(0, -x)` of the lower lane. `trackGrDb`/`submixGrDb`
  stay non-negative per the `.d.ts`; the signed value is on the `readObservations` path.
- Not in this PR: the dynamic EQ's declaration (its crate is in open PR #1919); it moves to #17 S2. Compressor,
  gate, de-esser, multiband and limiter are untouched (class A: the fold `match` gains an arm only).

## Evidence (attempt 1)

Recorded in the PR body by root from the implementer's report.
