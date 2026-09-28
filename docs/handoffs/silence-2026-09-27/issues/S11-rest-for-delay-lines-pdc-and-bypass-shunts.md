# Rest for delay lines, PDC compensation and bypass shunts

Draft, slice S11 of the silence architecture issue (A0). Class A. Evidence:
`docs/handoffs/silence-2026-09-27/DESIGN.md` section 4.9 (#940's successor S6).

## Product outcome

Pure delay lines are ring swaps with no arithmetic: PDC `CompensationDelay` and `TrackDelayLine`
(`crates/graph/src/runtime.rs:652`, `:733`, `delay_lane` `:785`) and the console stage's bypass shunt
line (`effect_contract::BypassShunt`, `crates/effect-contract/src/live.rs:688`). A line of `D` samples
whose input has been silent for `r` consecutive blocks of `Q` frames holds only `+0.0` once
`r * Q >= D + Q`; from then on its output block is silent and the swap can be skipped, with the cursor
advance deferred and applied once on the next live block (all observable words bit-identical).

* each line keeps a run-length counter of consecutive silent input blocks (the input fact comes from
  the silence table or an early-exit scan) and exposes its output fact;
* a console stage whose shunt line is at rest no longer declines S4's latch on that account
  (`advance_silent` defers the shunt cursor).

The delay *effect* (feedback, damping; per-node, never banked) needs its own rule, not in this slice:
its tap reads have been `+0.0` for at least the delay length, its input is `+0.0` and its damping state
is `+0.0`, so the active ring window is `+0.0`. Its payload (768 KB per instance at 48 kHz, cursor
included) rules out the generic latch.

## Authorized paths

`crates/graph/src/runtime.rs` (the two line types), `crates/effect-contract/src/live.rs`
(`BypassShunt`), `crates/rack/src/lib.rs` (the console stage's claim), tests, this spec.

## Objective gates

1. Partition invariance: every line, at every `D` in 0..=4,097 and block partitions 1..=128, renders
   bit-identically with and without the rest skip over seeded on/off patterns (extend
   `a_delay_line_is_a_pure_shift_at_every_partitioning` and
   `compensation_delay_is_partition_invariant_and_matches_per_sample_reference`).
2. Red mutations: rest after `r * Q >= D` (one block early); forget the deferred cursor; skip on a
   `-0.0` input block.
3. A session with track delays and a bypassed latent effect, sparse, bit-identical to declined.
4. Realtime and allocation-free.

## Console benchmark rows

None: no fixture carries a delay (`delay_samples` is 0 everywhere). A delay-bearing sparse variant is
the evidence.

## Dependencies

S4 (and S10 for input facts across units).

## Standing rules for the implementer

- Work only from this body. Read the cited functions first; do not survey the workspace.
- Class A: every gate that says "bit-identical" or "unchanged" is a hard stop.
- Commit on `codex/<issue>-<slug>`. Do not run the timed runner; do not quote a projected saving.
