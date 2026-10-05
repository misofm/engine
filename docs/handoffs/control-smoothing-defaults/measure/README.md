# #1055 measurement harness

A standalone Cargo package (its own `[workspace]`, so not a member of the engine workspace and not
part of any gate). It links `crates/builtins` by path and drives the shipped live-control ramps:
`InputBuiltins::set_polarity_invert` / `set_trim_db`, `FaderMuteRampBuiltins::set_fader_db` /
`set_mute` and `MatrixBuiltins::set_target_smoothed` with `pan_matrix`, in 128-frame blocks, applying
every record at the top of the first block at or after its admission sample, as the console drains
do (`src/strip.rs`; chain order input, fader, matrix). Every case asserts that the engine's output
samples equal `f32(g * x)` for the gain trajectory `g` that the same engine produces from a unit
probe, so the spectral analysis in `f64` measures exactly what the engine renders.

The decision-15 addition (`src/live_rows.rs`, FINDINGS section 9) also links, by path only, `lane`
(the live route's `IndexedRamp` and its kernel `route_mix_ramp_block`), `effect-contract`
(`BypassShunt` and the factory contract) and the six launch effects that have a bypass shunt
(compressor, gate-expander, parametric EQ, soft clip, transient shaper, true-peak limiter). The
bypass and link-glide rows emulate a crossfade the engine does not have yet (#1341, #1370) with the
law #1341 freezes (`src/crossfade.rs`): `y = dry + (wet - dry) * m(k)`, `m` on the indexed law,
both ends exact copies. Dry is the engine's shunt; wet is the effect's own scalar instance.

Nothing is timed. The only randomness is fixed-seed SplitMix64, so a rerun at the same commit
reproduces the CSVs (on 12 threads of a shared machine, `measure` took about 5 minutes of wall time
and `live` about 14).

## Commands

Run from this directory. `CARGO_TARGET_DIR` must point outside the repository.

```text
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<scratch>/target cargo test --release --offline
CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=<scratch>/target cargo build --release --offline
<scratch>/target/release/control_smoothing_measure measure --out ../data     # sections 5-6, all four launch rates
<scratch>/target/release/control_smoothing_measure live --out ../data        # section 9, all four launch rates
<scratch>/target/release/control_smoothing_measure stimuli --out <private dir> [--mix-wav F --mix-offset-s S]
python3 summarise.py                                                          # the FINDINGS tables
```

`measure` writes the seven files of sections 5-6 and `live` the four of section 9; neither touches
the other's files. `--rates 48000` narrows either to one rate for a quick look. To check
reproduction, run a command twice into two scratch directories and `cmp` each file.

`Cargo.lock` was seeded from the workspace lock so `--offline` resolves the same crate versions.

## Materials (`src/material.rs`)

Repository-owned synthesis, no third-party audio. `bass`: sustained A1 (55 Hz), additive, every
partial at or below 1 kHz, -15.4 dBFS RMS. `kick`: sine glide 150 -> 45 Hz, 350 ms body, short noise
beater, 120 BPM, -11.7 dBFS RMS. `mix`: dense 120 BPM stereo arrangement (kick, snare, hats,
shaker, bass line, detuned saw-pad chord up to 12 kHz), peak -3 dBFS, about -19 dBFS RMS.

## Output files (`../data`)

All levels in dB. Every ratio is relative to the energy of the unmodified input over the same
analysis frames. Frames: periodic Hann, 1024 samples at 44.1/48 kHz, 2048 at 88.2/96 kHz, 75 %
overlap, only 0-20 kHz counted.

| file | one row per | columns |
|---|---|---|
| `materials.csv` | rate x material | RMS and peak (dBFS) and RMS in dB SPL at the K-20 calibration used by `ctr` |
| `mute_click.csv` | rate x material x transition x ramp | `splatter_db`: energy moved more than half a critical bandwidth from its source frequency; `hf_splatter_db`: the same in 1.5-20 kHz relative to the programme's own 1.5-20 kHz energy; `oob_db`: energy of the output at or above `oob_edge_hz` (bass 1.5 kHz, kick 1 kHz), mean over events, and `oob_worst_db` the worst event; `oob_floor_db`: the same sum on the unmodified source; `ctr_max_db`: largest out-of-band critical-band energy over Terhardt's threshold in quiet at 0 dBFS RMS = 103 dB SPL, and `events_above_threshold` how many events exceed it. Frames centred 40 ms before to 90 ms after the change. Bass 16 events, kick 12, mix 24. |
| `mute_timing.csv` | rate x ramp | time from the applying block boundary to -20 dB, -40 dB and exact zero (mute), and to -3 dB, -0.5 dB and unity (unmute) |
| `fader_drag.csv` | rate x material x update rate x move x ramp | a UI samples a 0 -> -24 -> 0 dB move (800 ms or 200 ms each way) at 30 or 60 Hz; the same spectral columns for the engine and for a continuous per-sample drag (`ideal_*`, the floor); `ripple_*_db`: RMS and peak dB error of the engine's gain against the continuous move after removing the best lag, over the steady part of each move; `lag_ms`: least-squares delay of the engine gain behind the hand (includes the UI's sample-and-hold and block quantisation), `added_lag_ms` the ramp's share (relative to 0 ms); `settle_*`: time after the hand stops until the gain stays within 0.5 or 0.1 dB |
| `pan_drag.csv` | rate x update rate x move x ramp | bass in the left lane, pan -0.5 -> +0.5 -> -0.5; spectral columns summed over both outputs; `gain_error_*`: per-output dB error against the continuous sin/cos law after the lag; `power_dev_max_db`: largest deviation of `ll^2 + rl^2` from 0 dB |
| `pan_jump.csv` | rate x jump x ramp | one pan record over a whole range: minimum total power and time below -1 dB during the linear coefficient ramp |
| `polarity_click.csv` | rate x material x transition x ramp | `mute_click.csv`'s columns, for a polarity flip through the input trim (`invert`: +1 -> -1, `restore`: -1 -> +1); same events and frames |
| `law_transfer.csv` | rate x law x case x update rate x ramp | another law against the D11 ramp the earlier files measured, over the same move: `law` is `route-indexed` (the live route's `IndexedRamp` through `route_mix_ramp_block`) or `input-trim` (`set_trim_db`); `case` a mute, unmute, 0 -> -24 dB jump, full pan jump, or the 800 ms fader or pan drag at `update_hz`; `max_abs_diff` and `max_ulp_diff`: the largest difference between the two coefficient trajectories (every frame, both outputs); `both_settle_exactly`: both end on the same bits; `d11_bass_*` and `law_bass_*`: the bass out-of-band energy and click-to-threshold ratio under each trajectory (`mute_click.csv`'s method for a jump, `fader_drag.csv`'s for a drag; empty for pan) |
| `bypass_crossfade.csv` | rate x effect x material x transition x ramp | the bypass switch of each shunted effect at a representative `setting` (calibrated values in the column), as #1341's crossfade: `bypass` wet -> dry, `unbypass` dry -> wet, ramp 0 ms = today's whole-block step. `mute-reference` rows crossfade the programme to silence (a mute) under the same measure. `latency_samples`: the shunt's delay; `step_db`: energy of `wet - dry` around the switch relative to the dry programme; `wet_oob_db`: the wet programme's own energy above the out-of-band edge. The click columns measure the splatter `c` of `m (wet - dry)` (see `src/live_rows.rs`): `splatter_db`, `hf_splatter_db` (relative to the programme's 1.5-20 kHz energy), `oob_db`/`oob_worst_db` (energy of `c` above `oob_edge_hz`), `ctr_max_db`, `events_above_threshold` |
| `link_glide.csv` | rate x material x transition x ramp | the compressor at 6 dB peak gain reduction (under `maximum`) switching `dual_mono` -> `maximum` (`link`) and back (`unlink`), as the same crossfade between the two modes' outputs: the emulation of #1370's detector blend. `channel_correlation`: Pearson correlation of the input's channels; materials `mix`, `mix-wide` (right channel 0.37 s later, uncorrelated) and `bass-kick` (bass left, kick right, band-limited); the other columns as `bypass_crossfade.csv` |
