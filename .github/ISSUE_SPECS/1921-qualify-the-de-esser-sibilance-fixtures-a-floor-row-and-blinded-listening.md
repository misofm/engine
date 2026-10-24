# Qualify the de-esser: sibilance fixtures, a floor row and blinded listening

Filed by root as #15's slice S5 (backlog sprint, 2026-10-24). Stateless qualification work, split from the product slices per AGENTS.md.

## Smallest closable slice

1. Sibilance fixtures and response plots (split, wideband and listen modes; band-pass and high-pass detectors) in `dsp-research/de-esser.md`.
2. A class-A floor row for the de-esser in `docs/rulings/effect-floor-accounting.md` with a measured `isolated_cycles_per_lane_sample` from a console record (descriptive benchmark rules).
3. Blinded listening through #26's session.

## Paths

`dsp-research/de-esser.md`, the floor accounting doc, the listening record, this spec. After #15 S2 merges. Optional slice S4 (a lookahead quality with fixed latency) is not filed: the launch decision is latency 0.
