# Measure the delay's live bypass crossfade in the click harness

Scope now also covers the multiband compressor: #1340 (PR #1869) moved its gate 7 here, as #1339 did (comment on #1830).

Split from #1339 (root decision, 2026-10-24): #1339's gate 8.

## Problem

#1339 makes the delay's session bypass a live shunt with the session's bypass crossfade. Its gate 8 asked for a click measurement of a live delay bypass toggle with the standalone harness under `docs/handoffs/control-smoothing-defaults/measure/`. One run of that harness takes about 14 minutes. #1341 already proved the crossfade law on the effects that had a live shunt before #1339, and the delay has no latency line. So #1339 merged without the delay row of the harness.

## Smallest closable slice

Add the delay's live bypass to the harness's live rows and run it once. Record the measured click margin beside the other effects' rows in `docs/handoffs/control-smoothing-defaults/FINDINGS.md`.

## Gates

1. The delay row's click margin is inside the bound that #1341's rows meet, or the issue records the margin and the reason it differs.
2. The multiband compressor's row likewise (#1340's gate 7).

## Decision record (root, 2026-10-24, on the scope note)

- Step 0: the harness no longer built on main (`PrepareEffectRequest.tail_bound`, #1462 D1; `prepare` returns `PreparedEffect`). Repaired in its own commit; `Cargo.lock` reseeded from the workspace lock (the engine's `cfg(loom)` dependency).
- R1: the multiband's shared low/high threshold is bisected until the peak of its own resident `Gain Reduction` observation reaches 6 dB, because its unprocessed output is the crossover's all-pass of the input.
- R2: the delay runs at its shipped defaults (250 ms, feedback 0.35, damping 0.25, mix 0.35).
- R3: older rows that moved since they were committed are reported, not refolded.

## Evidence (attempt 1)

- Step 0 moves nothing: on the repaired harness `bypass_crossfade.csv`, `link_glide.csv`, `polarity_click.csv` and five of the seven `measure` files are byte-identical to the committed ones; the six older effects' bypass rows are unchanged after the two new effects are added.
- R3: `fader_drag.csv` (45/432 rows), `pan_drag.csv` (21/144) and `law_transfer.csv` (31/484) moved since they were committed (2026-09-28 and later). All moved rows are 30/60 Hz drags with ramps of 10 ms or more; largest change 1.0 dB on one HF splatter cell, every other dB cell 0.14 dB or less. Recorded in FINDINGS 9.7, not refolded.
- Gate 1, delay: at or below the mute reference on every measure, rate, length and material; smallest margin 2.62 dB (HF splatter, mix, 88.2 kHz, 2 ms).
- Gate 1, multiband: below on every measure but the mix's HF splatter, where it is +0.82 to +2.33 dB above at 2-20 ms (48 kHz, 10 ms: +1.72 dB; 0 ms: 4.6-6.3 dB under). Reason: the crossover's all-pass is in antiphase with the input at 1 kHz, so `wet - dry` there is up to twice the programme. A one-off 48 kHz run with both ratios at 1 (the all-pass alone, not committed) gives +2.30/+1.86/+2.18/+2.17 dB at 2/5/10/20 ms, at least the calibrated excess. At 10 ms the multiband's click stays 2.7-3.6 dB under the 5 ms mute's. Recorded in FINDINGS 9.4. Making the shunt's dry phase-match the all-pass would move rendered bits (class B) and is not done here.
- `cargo test --release --offline` in `measure/`: 21 passed (the effects test now also checks the multiband calibration lands within 0.01 dB). A mutant reading the observation with the wrong sign turns it red.
- Two four-rate `live` runs of the final harness are byte-identical (cmp), PR evidence only.
