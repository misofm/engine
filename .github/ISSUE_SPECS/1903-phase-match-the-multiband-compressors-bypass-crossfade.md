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

## Decision record

- 2026-10-25, owner ruling: option 1, accept and record. No code or test change; no rendered bit moves.
- Why option 2 was not adopted: a dry path all-passed only during the fade must still settle to the raw delayed input (the bypass null, `graph-compiler/tests/bypass_shunt_identity.rs`), so the phase step moves to the fade end as a hard step (option 2 as filed). The consistent forms are 2a, a permanent all-pass on the bypassed output, which breaks the null and the identity gates, and 2b, a crossover glide to a band edge during the fade, a new class-B design. Neither is worth a 0.8-2.3 dB HF-splatter margin that is under a 5 ms mute's click.
- Recorded in `dsp-research/multirate-crossovers.md`, "Known gaps and follow-up". The scope note named `dsp-research/multiband-compressor.md`, which does not exist; root authorized the existing crossover note, which already documents the multiband compressor, in its place.
- Reopening condition: a listening complaint about the multiband bypass, or a product need for a phase-matched bypass.
- No test: a test that pins a documented margin is a prose/number ledger (AGENTS.md test-value rule).
