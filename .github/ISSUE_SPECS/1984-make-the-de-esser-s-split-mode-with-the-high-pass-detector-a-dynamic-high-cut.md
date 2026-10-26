# Make the de-esser's split mode with the high-pass detector a dynamic high cut

Filed by root from the #1921 qualification (backlog sprint, 2026-10-24). Class B: the fix moves rendered bits on the split + high-pass path, so it needs the owner's ruling before implementation.

## Defect (measured)

`crates/de-esser/src/lib.rs:143-144` and `dsp-research/de-esser.md` say that split mode with the high-pass detector is a dynamic high cut. It is not. Split computes `y = x - (1 - g) h`, where `h` is the SVF high-pass tap. That tap has the value `jQ` at f_c, so `|1 - (1 - g) jQ| > 1` there, and below f_c its phase is near 180 degrees, so that region is boosted too.

Measured at 48 kHz, f_c 6.5 kHz, the gain held at -20 dB (every row matches the analytic `|1 - (1 - g) H_hp|` to 0.01 dB):

| Q | -2 oct | -1 | -0.5 | f_c | +0.5 | +1 | +1.5 |
|---|---|---|---|---|---|---|---|
| 0.5 | +0.36 | +0.93 | +1.11 | +0.80 | -0.46 | -3.25 | -9.24 |
| 1 | +0.42 | +1.57 | +2.63 | +2.58 | -1.39 | -7.31 | -14.40 |
| 2 | +0.44 | +1.90 | +4.10 | +6.27 | -2.86 | -12.43 | -18.82 |
| 4 | +0.45 | +2.00 | +4.79 | +11.45 | -3.93 | -17.36 | -21.45 |

At the defaults with the high-pass detector, on the 5-9 kHz sibilance fixture, the detector reports -15.2 dB held, but the 5-9 kHz band energy rises by 3.88 dB and a 1 kHz tone moves +0.151 dB (above the 0.1 dB isolation claim).

No test catches it: `every_output_arm_matches_the_reference` compares with an oracle that has the same law, and `split_mode_cuts_the_band...` uses the band-pass detector only.

## Proposed fix (root's recommendation)

A complementary high split: `y = g x + (1 - g) v2`, where `v2` is the SVF's low-pass state. This is a dynamic high shelf: unity below f_c, and the held gain above it, with no boost anywhere. Equivalently, split the signal into the SVF's low-pass and high-pass parts that sum to the input exactly, and apply `g` to the high part only.

## Gates

1. A response test on the product (not the oracle): split + high-pass, held gain, |H| <= 1 + epsilon at every frequency, and the held gain reached above f_c within a stated bound, at Q 0.5, 1, 2 and 4.
2. The sibilance fixture: the 5-9 kHz band energy falls with the high-pass detector, and the 1 kHz tone stays within 0.1 dB.
3. The oracle (dsp-reference) changes to the same law; every other mode's bits are unchanged (class A outside this path).
4. Docs and the research note restated.

## Paths

`crates/de-esser/src/` (the split arm and its doc), `crates/de-esser/tests/`, `crates/dsp-reference/` (the oracle arm), `dsp-research/de-esser.md`, this spec.

## Decision record

- **Owner approval (2026-10-26, "Yes for 1984").** Split mode with the high-pass detector becomes a
  dynamic high shelf with no boost at any frequency; bits move only on that path (class B).
- **The proposed formula is not boost-free (scoper's proof).** With `D = s^2 + k s + 1` (`k = 1/Q`,
  `w` normalised to f_c after prewarp), any numerator `g s^2 + b s + 1` over `D` has
  `|H|^2 - 1 = -w^2 ((1 - g^2) w^2 + k^2 - b^2 - 2 (1 - g)) / |D|^2`, so `|H| <= 1` for all
  frequencies iff `k^2 >= b^2 + 2 (1 - g)`. `g x + (1 - g) v2` on the detector SVF has `b = g k`:
  boost-free only for `Q <= sqrt((1 + g)/2)`; +5.1 dB at f_c at the default Q 2 and -20 dB held,
  +11.1 dB at Q 4.
- **Root ruling: a dedicated shelf SVF.** Per channel, a third SVF on the main input at the same
  frequency with fixed Butterworth damping `k = sqrt 2`, and `y = g x + (1 - g) v2_shelf`. Then
  `k^2 = 2 >= 2/(1 + g)` for every `g` in `[0, 1]`. Butterworth because a fixed damping must meet
  the bound for every held gain, and `2/(1 + g)` reaches 2 as `g -> 0`: `k = sqrt 2` is the least
  damping, so the sharpest second-order (12 dB per octave) shelf, that never boosts, and it does
  not change shape with the detector's Q. Q now shapes only the detector in
  high mode. Rejected: the formula on the detector SVF (boosts), `Q_shelf = min(Q, 1/sqrt 2)`
  (shape follows a detector parameter), a first-order shelf (6 dB per octave; 1 kHz at about
  -0.1 dB), an RBJ moving-pole shelf (per-sample redesign).
- **Known limit (root tells the owner).** 1.5 to 3 octaves above f_c the response dips deeper than
  `g`: -25.8 dB at +2 octaves (-27.3 dB minimum at +1.74 octaves) with -20 dB held; never a boost.
  At f_c itself the cut is `|g - j (1 - g)/sqrt 2|`, between -3.1 and -4.8 dB for any held gain
  (measured -4.755 dB at 8 kHz with -8.842 dB held), so the held gain is reached about an octave
  above f_c.
- **Class A elsewhere.** The split arm is `blend(x - (1 - g) f, g (x - l) + l, D)`; `blend` is
  exact at 0 and 1, so wideband, split + band-pass and listen keep their bits. Bits move at
  `D = 1` in split mode and during a detector-switch ramp in split mode.
- **State.** The shelf adds two integrators and three ramps per channel (`RAMPS` 12 -> 15); the
  integrators follow the ramps in the channel section so every earlier payload word keeps its
  index. `STATE_LAYOUT_VERSION` stays 1: the payload is the in-memory plan-swap carry, not
  persisted state (R6b). The shelf steps in every mode with the main input's flush threshold
  (#1328), so a detector or mode switch never starts it cold, and it is part of the rest check and
  the D7 state scan.

## Evidence (attempt 1)

- Gate 1, `split_with_the_high_pass_detector_is_a_shelf_without_boost` (new): red on main (first
  failure: Q 0.5, -2 octaves, +0.390 dB); green on the branch, every probe equal to the analytic
  shelf to 0.001 dB (f_c -3.820 dB, +1 octave -17.053 dB, +2 octaves -23.167 dB at 96 kHz).
- Gate 2, `split_mode_cuts_the_band_and_leaves_an_out_of_band_tone` over both detectors: red on
  main (high-pass detector: 1 kHz moved +0.142 dB); green on the branch.
- Gate 3, `every_output_arm_matches_the_reference`: the (Split, HighPass) row against the new
  oracle law (RBJ Butterworth low-pass in `dsp_reference::de_esser`), tolerance unchanged.
- Class A: a one-time base (f62e4312d) against branch render diff (scalar and native eight-lane
  bank, dual and collapsed, static and ramped parameters, live mode, listen and detector switches,
  two link modes): every wideband, split + band-pass and listen output bit-identical; only the
  split + high-pass renders and the split detector switch moved.
- Gate 4: `dsp-research/de-esser.md` (topology, equations, the shelf bound and its limit, the
  three re-recorded rows, rejected alternatives) and the detector doc in `lib.rs`.
