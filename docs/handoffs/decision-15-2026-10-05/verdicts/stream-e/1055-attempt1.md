FAIL

# #1055 attempt 1: adversarial verdict (decision-15 stream E)

Subject: `codex/d15-stream-e`, base `8be19c86e`, commits `d541cc11c` (evidence) and `9d2d26ab2` (spec Attempt record).
Verified on an export of `9d2d26ab2` at `/tmp/claude-1002/v1055/tree`. Builds used `CARGO_TARGET_DIR=/tmp/claude-1002/v1055/target*`. I did not edit, build or check out anything in the worktree.

Verdict: FAIL on one MAJOR finding. The fix is to the text of the record only. The harness, the data and every gate are sound. The recommendations (keys, 10/20/20 ms, polarity at twice `muteMs`) survive. But the record states a parity claim that its own CSV contradicts. That claim is the basis for the one deviation from #1054 and for the no-contrast decision under D3.

## MAJOR

**MAJOR-1. The polarity parity claim "within 0.8 dB on every measure and rate" is false against `polarity_click.csv` and `mute_click.csv`.**
Where the claim appears: `docs/handoffs/control-smoothing-defaults/FINDINGS.md:378` (9.2), `:537` (9.6), and the spec Attempt record (`.github/ISSUE_SPECS/1055-...md:160`).

Both files are written by the same `click_rows`, with the same columns. On the total-splatter column `splatter_db`, the 20 ms flip does not match the 10 ms mute:

| material | 20 ms flip minus 10 ms mute (`splatter_db`) | rates |
|---|---|---|
| bass | +2.11 to +2.20 dB | every rate |
| kick | -0.94 to -0.99 dB | every rate |
| mix | +0.94 dB | 48 kHz |

For example, at 48 kHz the bass flip is -37.81 dB and the bass mute is -39.92 dB. On bass, total-splatter parity would need about 2.5x the mute length, not 2x. The claim holds only on OOB, worst OOB, CTR and HF splatter (largest difference 0.80 dB).

Why it matters:
- 9.6 justifies "no polarity contrast" with "its click equals the mute's ... on every measure, so the M block's mute decision carries over". The measure that differs is the near-source spreading. This is also where the midpoint dip that 9.7 leaves unverified would show up.
- D2 makes section 9 the default record that #1054, #1261, #1364 and #1394 copy. The claim must be correct before root carries the deviation into those issues.

Required fix:
- State the parity on the measures where it holds (OOB, worst OOB, CTR, HF splatter).
- Report the total-splatter differences and explain them. The flip's total change is twice a mute's at any length, so its below-edge spreading is not set by the slope corners. Then say whether this changes the factor (it should not, given section 1's decision rule) and re-justify 9.6 on that basis.
- Correct the Attempt-record sentence.

## MINOR

**MINOR-1. "mute-reference rows reproduce `mute_click.csv` within 0.03 dB" is wrong (`FINDINGS.md:441`, `:560`).**
Twenty-six cells differ by more than 0.03 dB. The largest differences are 0.13 dB OOB (96 kHz bass 20 ms: -83.42 against -83.55) and 0.12 dB CTR (48 kHz kick 10 ms: 23.20 against 23.32). The conclusion does not change. Change the number to 0.13 dB.

**MINOR-2. The link-glide answer covers four effects, but only the compressor was assessed (`:495-531`, `:563-568`).**
#1054 D3's "detector link glide" row and #1370 cover the compressor, gate-expander, transient shaper and true-peak limiter. The bound in 9.5 depends on the compressor's attack smoother being about 1 ms or longer. Neither 9.5 nor 9.7 says that the other three effects (for example the gate's steep static curve) were not assessed.

`:511` says "#1370's own gates and its listening item cover the real glide". This is overstated: no #1370 gate measures a click; only its listening item does.

The spec scoped the measurement to the compressor, and 20 ms is the safer direction, so this is MINOR. 9.7 should state the limit.

**MINOR-3. The raw-matrix and send-matrix rows (`:340`, `:345`) omit the through-zero case.**
These rows argue from pan's measurements. But a raw matrix coefficient lies in [-1, 1] (`B:3019` `target.checked()`), and a send matrix coefficient may be any finite value (`crates/graph-compiler/src/ids.rs:341`, `route_coefficients`). Both can be negative, so a matrix record can carry a coefficient through zero (a change of 2): the 9.2 polarity case, which pan (coefficients in [0, 1]) never reaches.
- At the defaults, `panMs` = 20 ms = 2 x `muteMs`, so a matrix flip lands on the 10 ms mute's click.
- A session with `panMs` < 2 x `muteMs` gets a louder matrix flip.

Neither row says this. Add one sentence that points to 9.2.

**MINOR-4. The polarity answer does not weigh its alternatives fully (`:382-395`).**
- Only `faderMs` is explicitly rejected. A separate key (`polarityMs`) is argued against only by implication ("parity is a property of the law").
- The cost of doubling the ramp is not weighed in the answer. The level dip through zero lasts twice as long: below -6 dB for 10 ms at the 20 ms default, against 5 ms. 9.7 mentions the dip only as not verified.

The DSP is sound: the flip gain is exactly `2 g_mute - 1`, with corners `2/N`. Over `2N` the corners match the mute's. Root should still see the trade-off and the rejected fourth key in 9.2.

## NIT

- **NIT-1.** `:471` says "largest excess -0.04 dB". That is the CTR value. On `oob_worst_db` the largest excess is -0.01 dB (48 kHz, gate, kick, 0 and 2 ms). It is still below zero.
- **NIT-2.** `:481` and `:484` say "37-41 dB ... (29-41 dB in the mix's HF splatter)". These are the 48 kHz values. Over all four rates the ranges are 36.8-41.2 dB and 26.2-40.8 dB (44.1 kHz transient shaper on the mix). The transient shaper on the kick (step -62 dB, 10-11 dB) is excluded without saying so. Qualify the rate and the exclusion.
- **NIT-3.** `:543` says "17 dB or more". The CTR margin is 16.8 dB at 44.1 and 88.2 kHz.
- **NIT-4.** `[REISS-COMP]` (`:312`) says 9.5 "cites this key for context only", but 9.5 never cites it. It is an orphan entry.
- **NIT-5.** `:499` cites `crates/effect-runtime/src/envelope.rs:163-176` for `c = 1 - exp(-1/(tau fs))`. That anchor is `rms_follow`. The compressor's coefficient (the `f64` form) is in `crates/compressor/src/design.rs:18-30`.
- **NIT-6.** The route-probe unit test covers one record from a settled state. A probe that forgot `position = 0` on a retarget passes all 16 tests (mutation run). Only the drag rows of `law_transfer.csv` would expose it.
- **NIT-7.** 9.7 does not list the delay and multiband shunts (#1339, #1340) as unassessed. #1341 says its law will cover them with no further edit.

## Polarity recommendation: fit with decision 15 and #1054

**Sound DSP.** The flip uses the input stage's real D11 ramp through zero (`B:1436-1439`, `B:1502-1519`). At the same length, every measure is the mute's plus 6.01-6.04 dB (I checked this from the CSVs). Over `2N` the slope corners equal the mute's.

**Fits the contract.** #1055 D2 lets section 9 change a length, and the dependent issues follow it. D15-1 allows it: the keys are unchanged, and a listening result still moves only values; the factor follows `muteMs`. It is not a spec problem.

**Root must carry it explicitly in the closing comment**, because it changes #1054's wording, not only a number:
- #1054 D3: a note in the table.
- #1054 D4: "for_row ... returns that row's key field" and "No row has a separate default".
- #1054 gate 3: "for_row returns the D3 key's field for every LiveRampRow".
- The schema document.
- #1261 D2, Ramps bullet: PolarityInvert carries `mute_samples`.
- #1364 row map and its gate-1 example "polarity on the fader key". At the defaults `2 x mute == fader`, so that test needs non-default values.
- #1394 `resolve`.

Root should also confirm the worker's choice of a derived rule inside the three-key schema over a fourth key.

## Requirement checks (all confirmed)

**Section 9.1 rows.** All 15 product-outcome rows are present, in order, each with a key, a default in ms and a CSV or a source argument. I opened every anchor on the tree, and each reads as claimed:
- B:1434-1478, 1436-1439, 1480-1500, 1502-1519, 2730-2750, 2773-2794, 3008-3049, 4345 and 4363.
- K:159-202, 422-470, 618-726 and 638-650.
- `runtime.rs:861-931`.
- `kernels.rs:1073-1168` and 1177-1263.
- `graph/src/lib.rs:759-812` and 922-967.
- `route_controls.rs:82-107`, `live_route_state.rs:46-47`, `vca.rs:1-13`, `solo.rs:255-263` and 311.
- W:5179-5214, 5215-5273 and 5274-5364.
- `live.rs:822-990` and 841-843.
- Compressor `kernel.rs:317-340`, 349-361 and 364-369.
- The follows_mute and VCA source arguments agree with #1226 D5, #1342 and #1247 D5.

**Real kernels.**
- Polarity and trim run through `InputBuiltins::set_polarity_invert` and `set_trim_db`.
- The route law uses the shipped `IndexedRamp` and `route_mix_ramp_block`. The drive loop mirrors `LiveRoute::drain` and `mix`; `LiveRoute` is `pub(crate)`.
- Wet comes from the six shipped factories, prepared as `effect-compiler` prepares them. Dry comes from the engine's `BypassShunt`.
- The law transfer compares two real kernels (D11 builtins against the indexed route). It is not a tautology: the route law differs by up to 1.352e-4, within D11's N·2^-25 accumulation. The trim and fader trajectories are bit-identical.

**Crossfade.** It follows #1341's law: `m` is word 0 of the shipped `IndexedRamp`, frame `f` is at `k = f+1` as in the route kernel, the arithmetic is separate subtract, multiply and add, and both ends are copied. A length of 0 is today's whole-block step.

**Link-glide bound.** The derivations check out:
- Initial-slope factor `a(rho-1)/(1-rho^-a)` = 8.2 for ratio 4 and 20 dB.
- `2 pi f tau` = 94 at 1.5 kHz and 10 ms.
- The stated 1 ms / 0.1 ms limit is honest for the compressor (see MINOR-2 for the other three effects).

**Citations.** All are primary or official and paraphrased. There is no legacy-engine material.

**Listening.** No listening result is fabricated. #1388 is clearly owner-pending, and `listening/` is unchanged. The D3 pointers are present in section 1 (`:52-54`), section 6 (`:261-262`) and section 9.

**D4.** The only file changed outside `docs/handoffs/control-smoothing-defaults/` is the spec, and that change is append-only (the Attempt record).

**Other numbers I spot-checked against the CSVs, all correct:**
- The 9.2 table.
- The 9.4 table.
- The 9.5 table and its margins.
- `law_transfer` (352 route rows; all settle on the same bits; bass OOB and CTR within 0.03 and 0.08 dB).
- Bypass against unbypass, and link against unlink: 0.00 dB.
- OOB minus (mute + step) in -1.12 to +1.77 dB.
- The calibrated settings.

## Test value (one sentence per new test)

1. `crossfade::the_mix_follows_the_indexed_law_from_the_first_frame_after_the_record`: turns red if the emulation accumulates `m` (D11) instead of `IndexedRamp`'s `c(k)`, or numbers the first ramp frame `k = 0`. No other test checks the crossfade law (the mutation reds it and test 3 only).
2. `crossfade::both_ends_are_exact_copies_of_the_settled_planes`: turns red if either settled end, before the record or after the ramp, is computed through the formula instead of copied. It is the only test that reds on each of the two mutations.
3. `crossfade::a_ramping_frame_is_dry_plus_the_scaled_difference`: turns red if the in-ramp arithmetic changes form, for example `wet*m + dry*(1-m)`, which test 1's 0/1 planes cannot see (mutation: only this test reds).
4. `live_rows::the_route_probe_mixes_by_the_indexed_law_from_the_applying_block`: turns red if the probe applies a record in the block that contains its admission rather than the first block at or after it (mutation: only this test reds). It does not catch a missed position reset (NIT-6).
5. `live_rows::a_polarity_flip_is_the_trim_ramp_through_zero`: turns red if the strip does not render the input section, or wires polarity with the wrong length. Mutations: the input section removed (red together with test 6); the length halved (only this test reds).
6. `live_rows::trim_and_fader_ramps_are_bit_identical`: turns red if the `TrimDb` path is not rendered, or if the engine's trim retarget stops being the fader's D11 bit for bit. It is the only check of the trim wiring and of the law_transfer `input-trim` claim.
7. `effects::the_shunt_is_the_latency_matched_input_and_the_calibration_lands`: turns red if `shunt_dry` delays by the wrong latency, or if the calibration bisection runs the wrong way. Each was a mutation, and only this test reds.

## Mutation runs (on a copy of the export; each restored and confirmed identical with `cmp`)

| mutation | result |
|---|---|
| accumulated `m` (`acc += step`) instead of `coefficients_at(k)` | red: tests 1 and 3 (frame 1, bits differ by 1 ulp) |
| settled end after the ramp computed (`dry + (wet-dry)*target`) | red: test 2 ("after"), only |
| settled end before the record computed (`dry + (wet-dry)*start`) | red: test 2 ("before"), only |
| two-multiply mix `wet*m + dry*(1-m)` | red: test 3, only |
| route record applied in the admitting block | red: test 4, only |
| route `position = 0` removed | green, all 16 tests (NIT-6) |
| input section not rendered | red: tests 5 and 6 |
| polarity length halved | red: test 5, only |
| shunt latency minus 1 | red: test 7, only |
| calibration direction inverted | red: test 7, only |

## Gates run on the export

1. **Harness tests.** From `measure/`, `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/tmp/claude-1002/v1055/target cargo test --release --offline`: 16 passed, 0 failed. `cargo clippy --release --offline --all-targets`: 0 warnings.
2. **Reproduction.**
   - `control_smoothing_measure live --out <scratch>`, all four rates, 13 min 21 s: all four new CSVs are byte-identical (`cmp`) to the committed ones. Their SHA-256 values equal the Attempt record's (a5fe06dd..., 89eb4b73..., 98d30ca0..., e22ff0d6...). The run's own assertions also held: engine output is the gain, the `f32` crossfade is the analysed one, and every effect block reports clean.
   - `measure --out <scratch>`, all four rates, 6 min 26 s: all seven earlier CSVs are byte-identical with the extended harness.
3. **Policy scripts.** `bash scripts/check-workspace-policy.sh`: ok. `bash scripts/check-env-vocabulary.sh`: ok (59 names). `bash scripts/check-dsp-research.sh`: ok.
4. **Section 9.1 table.** 15 rows; each has an anchor, a key, a default in ms and evidence (checked by hand above).
5. **Listening packet.** `TMPDIR=/tmp/claude-1002/v1055/tmp python3 docs/handoffs/control-smoothing-defaults/listening/listening.py self-test`: passed (68 trials).

Not verified by me: the primary text of [REISS-COMP] (the worker could not open it either; 9.5 does not rest on it). The real #1341 and #1370 kernels do not exist yet.
