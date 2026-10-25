# Qualify the de-esser: sibilance fixtures, a floor row and blinded listening

Filed by root as #15's slice S5 (backlog sprint, 2026-10-24). Stateless qualification work, split from the product slices per AGENTS.md.

## Smallest closable slice

1. Sibilance fixtures and response plots (split, wideband and listen modes; band-pass and high-pass detectors) in `dsp-research/de-esser.md`.
2. A class-A floor row for the de-esser in `docs/rulings/effect-floor-accounting.md` with a measured `isolated_cycles_per_lane_sample` from a console record (descriptive benchmark rules).
3. Blinded listening through #26's session.

## Paths

`dsp-research/de-esser.md`, the floor accounting doc, the listening record, this spec. After #15 S2 merges. Optional slice S4 (a lookahead quality with fixed latency) is not filed: the launch decision is latency 0.

## Decision record

- 2026-10-25, root (scope note, binding; the #1943 pattern): the slice is O1 only. Slice 1 is
  delivered as measured tables, not plots (no research note in the repo carries a plot): the
  band-pass and high-pass detector taps at f_c and +-1 octave for three centres, the split,
  wideband and listen outputs on an 8 kHz burst over a 1 kHz tone at three burst levels across the
  threshold (gain at f_c and at 1 kHz, held change, static curve), and the attack (90 %) and
  release (10 %) crossings, all at 48 kHz, in `dsp-research/de-esser.md` (Fixtures). The harness
  was a throwaway on `tests/support`, run once at 4448e90b3 and not committed. No new test: the
  output law is held by `tests/output_law.rs` and the band step by `effect-runtime`'s
  `tests/dynamic_band.rs` (one test per claim).
- Evidence: both taps are within `2.9e-6` dB of the cookbook at all 18 points; the change is the
  same in every output mode for a given detector; split with the band-pass detector cuts the 8 kHz
  burst by the held change and moves the 1 kHz tone at most 0.013 dB; the release 10 % crossing is
  115.15 ms against the one-pole 115.13 ms. Split with the high-pass detector boosts the burst
  (+5.66 dB at a -14.84 dB change), the defect #1984 records; its three rows are re-recorded after
  the owner's class B ruling on #1984. No other measured number contradicts the note, the crate
  docs or the tests.
- #1921 closes on O1 plus this record. Slices 2 (the class-A floor row) and 3 (blinded listening)
  leave this issue as owner items on the owner list (row 1921/1943, still open), as #1943's did:
  no post-limiter effect has a floor row, a row needs a controlled console record with a measured
  core clock that this host does not produce, and a listening session needs a person.
