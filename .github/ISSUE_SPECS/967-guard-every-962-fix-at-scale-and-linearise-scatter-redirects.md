# Guard every #962 fix at scale and linearise scatter_redirects

## Product outcome

#962 made compile and bind linear in track count (65,537 tracks: compile 164 s to 10 s, bind 983 s to 5 s). Its scale gate (`crates/graph-compiler/tests/scale.rs`) runs a session with no effects and one route at `Simd8`, so only the bank-membership scan and the bind scans that session reaches are guarded; #962's `effect_control_resource`, `Backend::Scalar` interval-scan and route-fold metadata fixes are not exercised at scale, and a regression in them would pass. The #962 verification also left `scatter_redirects` (bind) superlinear: 2.3 ms at 8,192 tracks and 47 ms at 65,537.

## Smallest closable slice

1. Add scale sessions that reach every #962 fix: effects with controls on every track, one route per track, and whatever shape reaches the remaining fixes, at `Backend::current()`; bound them so each fits CI's debug job with room.
2. Make `scatter_redirects` linear (or state why its growth is bounded) with identical results.
3. Time sources, per-track observers and meters, and sidechains at scale (#962 did not), and fix any quadratic found.

## Objective gates

- Every compiled and bound result byte-identical before and after (the #962 fingerprint and randomized probes).
- A timing table at 8,192 and 65,537 tracks for each shape.
