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
