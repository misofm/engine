# Phase-match the multiband compressor's bypass crossfade

Filed by root from #1830's measurement (backlog sprint, 2026-10-24). **Class B: this moves rendered bits and needs an owner ruling before implementation.**

## Finding

#1830 measured the multiband compressor's live bypass crossfade (#1340) in the click harness (`docs/handoffs/control-smoothing-defaults/FINDINGS.md` 9.4). On the mix stem its HF splatter is +0.82 to +2.33 dB above the mute click reference at 2-20 ms crossfades (+1.72 dB at 48 kHz, 10 ms). Every other measure is at least 3.3 dB under. At 10 ms the click is still 2.7-3.6 dB under a 5 ms mute's.

Cause: the shunt crossfades between the processed path, which passes through the Linkwitz-Riley crossover (an all-pass sum, H = -1 at the crossover), and a dry path that skips it. Near the crossover, `wet - dry` is up to twice the programme. A ratio-1 control run (the all-pass alone) gives the same excess.

## Options for the owner

1. Accept: record the margin as the multiband's known bypass behaviour (no code change).
2. Phase-match the dry path: run the dry signal through the same all-pass sum while the shunt crossfades, so `wet - dry` is only the compression. This costs one all-pass sum per lane during a crossfade and moves the bypassed and crossfading bits (class B).
3. Another design the owner prefers.

## Paths (option 2)

`crates/multiband-compressor/src/lib.rs`, `crates/effect-contract/src/live.rs` (if the shunt needs a hook), the click harness row, this spec.
