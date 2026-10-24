# Ramp the detector link of the de-esser and the dynamic EQ between modes

Filed by root as #1370's R5 successor (backlog sprint, 2026-10-24).

## Context

#1370 adds `retarget_link` and the shared detector-link glide for the compressor, the gate-expander, the transient shaper and the true-peak limiter. The de-esser and the dynamic EQ are also `lane_link: true` with `LinkModeSet::ALL`, so a live link change (#1371) needs the same glide in them.

## Smallest closable slice

1. `retarget_link` and the glide (w_link, w_avg per lane and channel, the D2 blend, exact arrival, mul plus add) in both effects, through the shared helpers #1370a added in `effect-contract`.
2. The D6 payload words and the restore rule as #1370a ruled them.
3. Per effect: the f64-oracle glide test, the exact-arrival and resting-lane law test, the bank-mate test and the refusal test, in the form #1370a used.

## Order

After #1370 (both slices). Before #1371 flips the link row to live for these two effects.

## Paths

`crates/de-esser/`, `crates/dynamic-eq/` (`src/` and `tests/`), `crates/dsp-reference/` if the oracle needs a new law, this spec.
