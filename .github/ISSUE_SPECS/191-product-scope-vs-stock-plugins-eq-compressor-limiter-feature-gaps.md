Round-2 research surveyed each effect against the stock plugins the owner named as the scope bar (Pro Tools, Logic, Ableton). These are OBSERVATIONS for owner prioritization — none are proposed for implementation, and several would change the #184 floor inventories if adopted (band count, filter slopes, detector modes are recount triggers).

## Parametric EQ (vs EQ III 7-band / Channel EQ 8-band / EQ Eight 8-band; ours: 4 sections)
- Band count: all three stock EQs ship 7–8 bands
- Variable-slope HPF/LPF (12/24/36/48 dB/oct) and first-order (6 dB) shelf/filter types
- Band-pass band type (EQ III has it; we have notch only)
- Global output trim/gain parameter
- M/S processing mode (EQ Eight); per-channel L/R already exists via ParameterChannel
- Band solo / audition ("listen") mode
- Spectrum analyzer / frequency-response curve data for the UI (engine observation surface exists; EQ declares no taps)
- Oversampling / hi-quality mode (EQ Eight)
- Proportional/adaptive Q options (Logic)

## Compressor (present: threshold/ratio/knee/attack/release/makeup/mix, 3 link modes, external sidechain port, GR tap)
- RMS (and switchable peak/RMS) detection — current detector is rectified peak into a dB-domain one-pole
- Program-dependent / auto release
- Sidechain detector filtering (HPF at minimum) and sidechain listen
- Auto-makeup
- Hold
- Fixed 20 ms PDC regardless of lookahead setting: latency = Fs/50 always (lib.rs:27, design.rs:192-202) — a lookahead-0 instance still reports a full 20 ms where stock plugins report only what they use; also why both rings are 961 rows

## True-peak limiter
- Ceiling accuracy: the frozen −1.0 dB internal estimator guard (limit_coefficient) puts the effective ceiling a full dB under the setting; stock true-peak limiters hold ~0.1–0.3 dBTP (#90 F7 / #49 own the measurement path). Related: gain applied at 1× while detection is 4×, so ramp-reintroduced inter-sample overs are covered only by the guard
- Auto / program-dependent release (single fixed-rate one-pole today, 10–2000 ms)
- No input drive / output (makeup) gain stage (Maxim, Ableton, Logic all have one)
- Metering: only the per-lane GR tap; no input/output true-peak meter, no overs counter
- Docs: lookahead 0–10 ms is preparation-only while latency is always pinned at N+6 — correct and standard, but worth stating user-facing
- Output dither (Maxim) — likely out of scope for this engine layer

Full detail in the round-2 research reports (linked from the #163 loop trail).
