PASS

# #1055 attempt 2: adversarial verdict (decision-15 stream E)

Subject: `codex/d15-stream-e`, base `8be19c86e`. Attempt 1: `d541cc11c`, `9d2d26ab2`. Attempt 2: `21a1d5af4` (evidence) and `a47df1c46` (spec Attempt record). I reviewed the whole slice `8be19c86e..a47df1c46` and read `9d2d26ab2..a47df1c46` line by line.
Verified on an export of `a47df1c46` at `/tmp/claude-1002/v1055b/tree`. Builds used `CARGO_TARGET_DIR=/tmp/claude-1002/v1055b/target` (gates) and `.../target-mut` (mutations, on a second copy). I did not edit, build or check out anything in the worktree.

Verdict: PASS. Every attempt-1 finding is fixed, and fixed correctly. The MAJOR-1 explanation is correct: I derived it again and checked it with my own model. Every number I checked in 9.2-9.6 and in the Attempt record matches the CSVs. All five gates pass, and the 11 CSVs regenerate byte for byte. There is one new MINOR finding: the 9.8 carry list misses three places in the dependent specs that pin polarity to the mute length. There are also three NITs. None of them changes a key, a default or the decision.

## Attempt-1 findings: status

| finding | status | evidence |
|---|---|---|
| MAJOR-1 (parity "on every measure") | fixed | 9.2 now gives parity only on OOB, worst OOB, CTR and HF splatter. It reports total splatter for each material and length, with an explanation that is correct (see "MAJOR-1 DSP check" below). 9.6 is re-justified by section 1's decision measures. The Attempt-record sentence is corrected in place and marked as corrected (spec, attempt 1 Results). |
| MINOR-1 (0.03 dB) | fixed, and more correct than my fix | The worker compares `bypass` with `mute` and `unbypass` with `unmute`. On that pairing the OOB gap is 0.20 dB (48 kHz bass unbypass 20 ms, -83.39 against -83.19). My 0.13 dB compared every row with `mute`. I reproduced both numbers: 0.20 (paired) and 0.13 (mute only); worst OOB 0.09, CTR 0.12, HF splatter 0.01, splatter 0.00. |
| MINOR-2 (link glide covers four effects) | fixed | 9.5 "Scope: the compressor only" and a 9.7 bullet. I checked the gate anchors: `gate-expander/src/kernel.rs:240-245` is the curve `(ratio-1)(L-T)` clamped by the range; `:217-238` is the hysteresis; ratio 1-20 and attack 0.1-50 ms, default 1 ms (`lib.rs:137-184`). The statement "No #1370 gate measures a click" matches #1370's gates 1-6 and its listening item (`1370-*.md:72-73`). |
| MINOR-3 (matrix through zero) | fixed | The 9.1 paragraph and the pointers in both table rows. Anchors checked: `B:88-102` (`checked`, [-1, 1]), `B:3019` (`set_target_over` calls it), `graph-compiler/src/ids.rs:288-304` (`route_transform`: finite, not subnormal) and `:341`. |
| MINOR-4 (alternatives, cost of the dip) | fixed | 9.2 weighs four choices and the dip. Dip arithmetic checked: \|1-2u\| < 0.5 for half the ramp and < 0.1 for a tenth, so at 20 ms that is 10 ms and 2 ms. |
| NIT-1 | fixed, and corrects me | Worst OOB -0.01 dB at 44.1 kHz 2/10 ms and 48 kHz 0/10 ms. My "48 kHz 2 ms" is -0.02 (unrounded rows: 48 kHz 2 ms bypass and unbypass are both -0.02). |
| NIT-2 | fixed | 36.8-41.2 dB and 26.2-40.8 dB over all rates. The lowest, 26.2 dB, is the transient shaper on the mix at 44.1 kHz. The transient shaper on the kick is reported apart: step -60.3 to -62.5 dB, OOB -105.7 to -107.3 dB, reduction 10.2-11.2 dB. All of these match my own script. |
| NIT-3 | fixed | 16.8 dB. |
| NIT-4 | fixed | [REISS-COMP] is removed and no reference to it is left. |
| NIT-5 | fixed | `rate_coefficient`, `compressor/src/design.rs:129-143`, with its reason at `:18-30`. `envelope.rs:163-176` is kept as the recurrence `ballistic` calls. All three anchors read as claimed. |
| NIT-6 | fixed | The new test (see mutations). |
| NIT-7 | fixed | 9.7 lists #1339 and #1340. |

## MAJOR-1 DSP check (derived independently)

- **Same length.** At the same `N`, the flip's gain is `2 g_mute - 1`. A constant gain moves no energy, so every spectral measure rises by exactly 6.02 dB. Measured: splatter +6.02 to +6.03 and HF splatter +6.00 to +6.03 (see NIT-1 below); OOB, worst OOB and CTR +5.45 to +6.17.
- **The flip over `2N` is two mutes back to back.** `g_f(t) = g_m(t) + g_m(t-T) - 1`, with `T = N/fs`. For `f != 0`, `G_f = G_m (1 + e^{-j2 pi f T})`, so `|G_f| = |G_m| |2 cos(pi f T)|`. With `|G_m| = |sin(pi f T)|/(2 pi^2 f^2 T)` this gives `|sin(2 pi f T)|/(2 pi^2 f^2 T)`. FINDINGS 9.2 has exactly these. The flip's per-sample step `2/(2N)` rounds to the same `f32` as `1/N`, because both are the correctly rounded value of the same real number.
- **Far from the source.** `4cos^2 sin^2 = sin^2(2x)`, which averages 1/2, the same as `sin^2 x`. So the band energies far from the source are equal: the two slope corners of each ramp are the same. This is why OOB, worst OOB, CTR and HF splatter match.
- **Near the source.** The factor goes from 2, to a null at `1/(2T)`, and back to 2 at `1/T`.
- **The splatter edge.** From `analysis.rs:171-200` I computed each bass partial's window (the quarter-critical-band sub-band plus half a critical bandwidth on each side). The nearest edge is 55-82 Hz from each partial at or below 600 Hz. That matches FINDINGS' "56-82 Hz".
- **My own model.** I weighted each partial by its amplitude (`material.rs:83-92`) and used both window edges (`/tmp/claude-1002/v1055b/ev/model.py`). Flip minus mute: **+5.01, -3.12, +1.93, -1.34 dB** at 2/1, 10/5, 20/10 and 40/20. Measured on the bass: **+4.46..+4.75, -3.23..-3.14, +2.11..+2.20, -1.06..-0.75**. The sign matches at all four lengths, and the size is within 0.6 dB.
- **The worker's model.** `flip_model_db` treats one partial over a range of edges, so it is looser: at 20/10 and 40/20 its range spans zero. FINDINGS states only what it shows: the ranges hold the measured values at three lengths and miss them by 0.2 dB at 2/1.
- **No single factor equalises total splatter.** I scanned factors from 1 to 4 in the weighted model. At 1 ms the excess stays at +2.6 dB or more even at 4x, but at 5 ms it is negative for every factor of 2 or more. So "no length factor gives parity on total splatter at every length" holds. The materials also disagree in sign at 20/10 (bass +2.1, kick -1.0, mix +0.3 to +0.9).
- **The decision rule.** Section 1 (`FINDINGS.md:40-46`) decides `muteMs` by OOB, CTR and HF splatter, never by total splatter. So the factor stands on section 1's own rule.

## "No amendment" against D3

D3 lets this slice add contrasts for questions 4-6 only "if section 9 recommends them". The argument holds:
- On the measures section 1 decides by, the flip at twice the mute length is the mute: within 0.80 dB at the default, 1.19 dB on the mean measures and 2.40 dB on the per-event peaks at the rules' other outcomes.
- D15-1 lets a listening result change values only, and the factor is a rule. So a polarity contrast could only move `muteMs`, which the M block already decides on the mute itself.
- The one open perceptual question is whether the dip through zero is heard. It is stated plainly in 9.2 and 9.7, kept away from #1388, and given to root (9.8, "Root confirms one choice").

`listening/` is unchanged (0 diff lines over the whole slice). No listening result is claimed or simulated, and #1388 is clearly owner-pending (9.7, Attempt record "Open items").

## D4

Over `8be19c86e..a47df1c46`, the only file changed outside `docs/handoffs/control-smoothing-defaults/` is the #1055 spec. In attempt 2, its one edit to earlier text is the corrected Results sentence, marked as corrected, as my attempt-1 verdict asked. Everything else is appended.

## MINOR

**MINOR-1. The 9.8 carry list misses three places in the dependent specs that pin polarity to the plain mute length (`FINDINGS.md:727-740`).**
9.8 says the change "changes wording ... in each dependant" and lists the items. My attempt-1 list had the same gaps; the worker copied it. Three more places need the polarity exception:
- **#1261 gate 1** (`1261-*.md:146-147`): "a TrimDb record carries `LiveRamps::for_session(next).fader_samples` and a PolarityInvert record its `mute_samples`". 9.8 names only #1261 D2's Ramps bullet. If gate 1 is left as it is, it tests the rule this record removes, and an implementer who follows the gate drops the factor.
- **#1364 gate 1** (`1364-*.md:131-132`): "480 for the mute rows" at 48 kHz. Its row map (`:36`) counts polarity as a mute row, but under the derived rule polarity resolves to 960. #1364 D2 (`:62`) also says the sentinel is replaced by "the field that #1054 D3 names". 9.8's note that the test "needs non-default keys" is right, but it names only the Test-value example (`:170`), not gate 1's expected value or D2.
- **#1394 context** (`1394-*.md:26-27`): "`for_row(row)` returns the D3 key field for a row". 9.8 says #1394 needs "no change of its own". That is true of its decisions and gates, but this sentence becomes stale.

Why it matters: under D2, 9.8 is the list root copies into the closing comment. Fix: add the three items, and say that #1394 changes only its background sentence.

## NIT

- **NIT-1.** `FINDINGS.md:377` says "+6.02 to +6.03 dB on total and HF splatter". HF splatter is +6.00 to +6.03; only total splatter is +6.02 to +6.03.
- **NIT-2.** `FINDINGS.md:446` says total splatter "is not a measure of the click". But 9.4 counts it among the click measures ("on any measure ... -0.69 dB on total splatter", `:570-574`), 9.5 quotes a total-splatter margin (`:650`), and 9.6 says "on every measure" (`:675`), which includes it. Better: "it is not a measure section 1 decides by". The argument does not need the stronger wording.
- **NIT-3.** `FINDINGS.md:363-366` (matrix through zero) says the matrix flip matches the 10 ms mute "at the defaults", and that a session with `panMs` below twice `muteMs` gets a louder flip. But one preregistered #1388 outcome (`muteMs` 20 ms with `panMs` 20 or 35 ms) makes that the case at the defaults themselves. This is the same argument 9.2 uses against `faderMs`. One clause would cover it. The decision to keep `panMs`, because a row's length is per row (#1054 D4), is sound.

## Number checks (independent scripts, `/tmp/claude-1002/v1055b/ev/check.py`, `check2.py`)

- **9.2 table at twice the length** (all 28 ranges) matches my script. The locations of the extremes also match the text: HF splatter 0.80 dB on the mix at 96 kHz (20/10); +2.40 dB worst OOB on the kick at 48 kHz (10/5); +1.41 and +1.34 dB on the bass at 96 kHz (40/20); mix total splatter +0.94 dB at 48 kHz.
- **9.2, 48 kHz table** including the new 40 ms row (-82.9 / +13.1, -78.5 / +17.0, -62.8, +0.35 / +0.25). The rate spreads: 1.1 dB OOB, 1.4 dB CTR, 2.5 dB mix.
- **Restore against invert**: 0.00 dB on every measure.
- **9.4**: largest excess per measure (-0.69, -0.15, -0.01, -0.04 and -0.14 dB, all the gate on the kick); mute-reference against `mute_click.csv`; reductions over all rates and at 48 kHz; the transient-shaper kick figures.
- **9.5**: margins 24.3-25.9, 22.6-23.7 and 16.8-18.2 dB on bass-kick; 38.2-40.0 / 37.7-38.5 dB on `mix` and 25.8-28.2 / 26.2-27.3 dB on `mix-wide`; events above threshold 1-2 of 10; 10 ms glide CTR margin 10.8-12.3 dB (so 18 dB less leaves 5.7-7.2 dB over the mute's CTR); -95.3 dB and +7.2 dB at 10 ms (48 kHz).
- **`polarity_click.csv`**: with its 24 rows at 40 ms removed, it is byte-identical to attempt 1's file. SHA-256 `801274e7...215e51`, as recorded.

## Test value (one sentence per new test in the slice)

1. `crossfade::the_mix_follows_the_indexed_law_from_the_first_frame_after_the_record`: turns red if the emulation accumulates `m` instead of using `IndexedRamp::coefficients_at`, or numbers the first ramp frame `k = 0`.
2. `crossfade::both_ends_are_exact_copies_of_the_settled_planes`: turns red if either settled end, before the record or after the ramp, is computed through the formula instead of copied. It is the only test that turns red for either mutation.
3. `crossfade::a_ramping_frame_is_dry_plus_the_scaled_difference`: turns red if the arithmetic of a frame inside the ramp changes form (for example `wet*m + dry*(1-m)`). Test 1's 0/1 planes cannot see that.
4. `live_rows::the_route_probe_mixes_by_the_indexed_law_from_the_applying_block`: turns red if the probe applies a record in the block that admits it, rather than at the first block boundary at or after admission.
5. `live_rows::a_polarity_flip_is_the_trim_ramp_through_zero`: turns red if the strip does not render the input section, or wires polarity with the wrong length.
6. `live_rows::trim_and_fader_ramps_are_bit_identical`: turns red if the `TrimDb` path is not rendered, or if the engine's trim retarget stops being the fader's D11 bit for bit.
7. `effects::the_shunt_is_the_latency_matched_input_and_the_calibration_lands`: turns red if `shunt_dry` delays by the wrong latency, or if the calibration bisection runs the wrong way.
8. **New in attempt 2.** `live_rows::a_retarget_restarts_the_index_from_where_the_route_is`: turns red if the route probe keeps its ramp position when a record lands. It catches this both mid-ramp (frame 384) and after the previous ramp has settled (frame 1152). Test 4, which uses one record from a settled state, cannot see it. Only this test protects the drag rows of `law_transfer.csv` against that defect.

## Mutation runs (on a copy of the export; each restored and confirmed identical with `cmp`/`diff -r`)

| mutation | result |
|---|---|
| accumulated `m` (`acc += step`) instead of `coefficients_at(k)` in `crossfade` | red: tests 1 and 3 only |
| end after the ramp computed (`dry + (wet-dry)*target`) | red: test 2 only ("after") |
| end before the record computed (`dry + (wet-dry)*start`) | red: test 2 only ("before") |
| `position = 0;` deleted in `route_probe` | red: test 8 only, "ll, frame 384"; 16 pass (as claimed) |
| variant: reset only while mid-ramp (keeps position after settle) | red: test 8 only, "ll, frame 1152" |

## Gates run on the export

1. **Harness tests.** From `measure/`: `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/claude-1002/v1055b/target cargo test --release --offline`: 17 passed, 0 failed. `cargo clippy --release --offline --all-targets`: 0 warnings.
2. **Reproduction.**
   - `control_smoothing_measure measure`, all four rates (5 min 56 s): all 7 CSVs are `cmp`-identical to the committed files.
   - `control_smoothing_measure live`, all four rates (12 min 43 s; the run's own engine-equals-gain, `f32`-crossfade and clean-effect assertions held, exit 0): all 4 CSVs are `cmp`-identical to the committed files. Their SHA-256 values are `a5fe06dd...`, `89eb4b73...` and `98d30ca0...`, the same as attempt 1 (only `polarity_click.csv` changed), and `801274e7...` for `polarity_click.csv`.
   - So all 11 of 11 CSVs regenerate byte for byte. The hashes are in `/tmp/claude-1002/v1055b/ev/regenerated.sha256`.
3. **Policy scripts** from the tree root: `bash scripts/check-workspace-policy.sh` ok; `bash scripts/check-env-vocabulary.sh` ok (59 names); `bash scripts/check-dsp-research.sh` ok.
4. **15-row table** (`/tmp/claude-1002/v1055b/ev/table.py`): 15 rows, in the product outcome's order. Each has a `path:line` anchor, a key (`muteMs`, `faderMs` or `panMs`), a default in ms and a CSV or source argument.
5. **Listening packet.** `TMPDIR=/tmp/claude-1002/v1055b/tmp python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`: passed (68 trials).
6. **`summarise.py`** runs clean on the committed CSVs, and its output matches 9.2-9.5.

Not verified by me: the real #1341 crossfade and #1370 glide (both are emulated; the kernels do not exist yet); the audibility of the polarity dip (no listening has been done; it is routed to root).
