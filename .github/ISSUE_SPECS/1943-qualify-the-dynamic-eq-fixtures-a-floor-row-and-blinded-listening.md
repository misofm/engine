# Qualify the dynamic EQ: fixtures, a floor row and blinded listening

Filed by root as #17's slice S5 (backlog sprint, 2026-10-24). Stateless qualification work, split from the product slices per AGENTS.md.

## Smallest closable slice

1. Fixtures and response plots (static and dynamic modes, down and up, internal and external detector) in `dsp-research/dynamic-eq.md`.
2. A class-A floor row in `docs/rulings/effect-floor-accounting.md` with a measured `isolated_cycles_per_lane_sample` from a console record (descriptive benchmark rules).
3. Blinded listening through #26's session.

## Paths

`dsp-research/dynamic-eq.md`, the floor accounting doc, the listening record, this spec. After #17 S2 merges.

## Decision record

- 2026-10-24, root (scope note, binding): the slice is O1 only. Slice 1 is delivered as measured
  tables, not plots (no research note in the repo carries a plot): the static bell at f_c and +-1
  octave for the four default bands, the down and up change against level in 6 dB steps, the
  internal versus external detector, and the attack and release crossings, all at 48 kHz with the
  parameters and levels stated, in `dsp-research/dynamic-eq.md` (Fixtures). The harness was a
  throwaway on `tests/support`, run once at 39247834b and not committed. No new test: the claims
  are already held by `tests/oracle.rs`'s three rows (one test per claim).
- Evidence: the static bell is within `1.5e-6` dB of the cookbook at all 36 points; the 1 kHz
  output gain equals the static gain plus the observed change within 0.001 dB at every level; the
  release 10 % crossing is 230.4 ms against the one-pole 230.3 ms. No measured number contradicts
  the note, the crate docs or the oracle rows.
- #1943 closes on O1 plus this record. Slices 2 (the class-A floor row) and 3 (blinded listening)
  leave this issue as owner items, tracked on the owner list and not kept open here: no
  post-limiter effect has a floor row, a row needs a controlled console record with a measured
  core clock that this host does not produce (its records are `uncontrolled`), and a listening
  session needs a person. Owner questions: (1) floor rows for the post-limiter effects, one
  cross-effect successor or none; (2) one combined de-esser and dynamic EQ listening packet now or
  at the DAW sprint.
