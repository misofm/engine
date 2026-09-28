# Ruling request: mono pooling versus the route fold

Draft ruling request for the owner, not an implementation issue. Base `6ca203f8`.

## The conflict

* The master bus must be summed in **track order** to keep its bits (a floating-point sum is not
  associative).
* Issue #218's route fold (each track's route and the master sum folded into its chain's epilogue)
  is admissible only when chains render their lanes in exactly that order.
* The mono collapse pools collapse-eligible tracks into their own cohorts. When mono and stereo
  tracks are **interleaved** in track order, the pooled chains no longer render in track order, so
  the whole fold declines (`tools/console-workload/tests/chain_shape.rs:343-373`;
  `crates/graph-compiler/src/lib.rs:8385-8405`).

The tree assumes interleaving is rare ("contiguous ... the shape a real session takes"). The
dogfood session is interleaved: its 18 dual-mono tracks sit at positions 0, 4, 10, 18-20, 22, 35,
36, 38, 40, 42-44, 53, 58, 60, 66 of 81, because track order is the sorted stable id.

## What it costs (dogfood, 81 tracks, 18 folded to mono; `DUAL-MONO.md` §4.4)

| | 8 lanes | 4 lanes |
|---|---:|---:|
| lost route fold (any strip) | +3.3 to +3.9 µs | +2.3 to +2.7 µs |
| collapse saving, 16 mono tracks on an EQ+compressor+limiter strip | -13.4 µs | -23.1 µs |
| collapse saving, 16 mono tracks on a builtins-only strip | -0.3 to -0.7 µs | -2.0 µs |

So with remainder-aware pooling the mixing strip still wins (-5.3% / -7.6%), but a builtins-only
mix loses at 8 lanes (+6%) and breaks even at 4.

## Options

1. **Keep today's rule.** Pool by class always; accept the fold loss. Simple; a net loss on light
   strips.
2. **Planner policy (class A).** Pool by class only when the mono cohorts carry enough upstream
   work to beat the fold (for example: at least one effect slot upstream of the seam per mono
   cohort). Every choice renders the same bits; only speed changes.
3. **Contiguous import order.** When stems are imported (or folded), give mono-sourced tracks stable
   ids that sort together, so pools are contiguous and both the fold and the saving are kept (the
   ~8% ceiling on this session). This changes the master's summation order relative to a session
   imported without it, so the mix differs in the last bits from that session. The display order in
   the app is separate (`app/src/lib/mixer/channel-order.ts`) and need not change.

## Recommendation

Option 2 now (class A, no audible or bit-level change), option 3 if the owner accepts that a
session's track-id order is a performance input at import time.
