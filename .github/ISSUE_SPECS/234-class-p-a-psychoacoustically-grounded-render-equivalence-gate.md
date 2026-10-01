**Scope: research audit (no engine code changes).** Companion to #233 (GPU backend architecture research), which gates every non-bit-exact outcome — the wgpu baseline backend, scan-form IIR, and the speculative-cache splice — behind "the class-P psychoacoustic instrument" this issue defines. GPU renders other than (possibly) native CUDA cannot be bit-identical to the CPU legs: FMA contraction, reduction ordering, transcendental implementations, and fast-math defaults all diverge (WGSL licenses reassociation and fusion outright; Metal compiles fast-math by default). Today's correctness bar is class-A: identical `output_sha256` across backends and legs. This issue derives a second, principled equivalence class:

> **Class-P (perceptually identical):** two sample-aligned renders of the same prepared plan whose difference signal stays below a conservatively-modeled threshold of audibility in every analysis window and every auditory band, under a stated worst-case playback calibration.

Class-A remains the bar for all CPU legs (bit-identical across Scalar/Simd4/Simd8/wasm and across machines, #197), for each GPU backend against itself (intra-backend determinism), and for any GPU backend that proves bit-exact (CUDA is a live candidate, #233 §4.3). Class-P qualifies the genuinely non-bit-exact pairs. Nothing is replaced or weakened.

Method note: numeric results below come from f32-exact emulations of the engine's actual algorithms (the TPT SVF EQ topology plus a DF1-biquad worst-case proxy, and the `miso-engine-compressor` dB-domain gain-word recursion) and from a prototype metric run against a calibrated perturbation corpus; scripts are summarized in the appendix. Since scalar and SIMD CPU legs render bit-identically today, cross-backend divergence was synthesized by construction (FMA-contracted/reordered arms, 1-ulp transcendental perturbation) at known magnitudes. Every threshold number carries a citation or an explicit derivation; engineering judgment is labeled as such.

## 1. Survey: what the literature offers, and what actually transfers

The problem is narrower than anything the standard metrics were built for: two renders of the *same program* through the *same prepared plan*, sample-aligned, whose difference is bounded numerical noise that is spectrally correlated with the signal and typically 40–120 dB below it. That is not codec evaluation. The survey below therefore ends each entry with an explicit transfer verdict.

### 1.1 PEAQ — ITU-R BS.1387 (ODG scale)
PEAQ (Rec. ITU-R BS.1387, 1998; current revision BS.1387-2, 05/2023) is the ITU objective method for perceived audio quality: an ear model plus 11 (Basic) / 5 (Advanced) model output variables (MOVs) mapped by a trained neural network to an Objective Difference Grade (ODG, 0 to -4), anchored to Subjective Difference Grades from BS.1116 listening tests. Its calibration corpus is codecs: per Thiede et al.'s account of the TG10/4 process, the databases were built from MPEG/ITU codec listening tests 1990-97 (MPEG-1 Layer 2/3, AC-2/AC-3, ATRAC, AAC, etc.; >600 coded excerpts), "predominantly items of medium or good quality" — i.e., *perceptible* artifacts.

Documented failure modes, each disqualifying for this problem:
- **Underspecification**: Kabal's McGill analysis concludes "the description in BS.1387 is inadequate by itself to allow for a conforming implementation" and "a conforming implementation is not possible without access to additional information." Open implementations disagree with the ITU conformance vectors at RMSE ~0.2 ODG — larger than the entire effect size a near-transparent comparison must resolve.
- **Near-transparent region unvalidated**: Torcoli et al. (2021) state plainly that "a generalization of the results on estimating BAQ of signals with small impairments should not be done without further research"; no published PEAQ validation exists for differences orders of magnitude below codec artifacts.
- **False-positive band on non-codec transparent changes**: Neubauer & Herre (AES preprint 4823) measured ODG -0.24 to -0.43 for watermarked items that were subjectively transparent — PEAQ does not return ~0 for inaudible non-codec modifications.

**Transfer verdict: the ODG scale and any ODG threshold do not transfer.** What does transfer is PEAQ's *architecture* (masked-threshold ear model; NMR-family MOVs — see 1.4) and its listening-level convention (full-scale sine = 92 dB SPL by default, per Kabal's reading of the standard), which this proposal adopts in stricter form.

### 1.2 PEMO-Q
PEMO-Q (Huber & Kollmeier 2006, IEEE TASLP 14(6)) correlates internal auditory representations (Dau et al. perception model) of reference and test into PSM (overall similarity) and PSMt (fifth-percentile of 10 ms instantaneous quality), with a codec-calibrated ODG mapping. The official manual is explicit that the ODG mapping "has especially been designed and optimized... for the evaluation of audio codecs," recommends the modulation-filterbank mode for "very small quality distortions," assumes amplitude 1.0 = 100 dB SPL — a precedent this proposal's calibration matches exactly — and requires pre-aligned signals. It is distributed closed-source (MATLAB P-files, Univ. Oldenburg EULA).

**Transfer verdict: no published PSM value is validated as an inaudibility threshold; PSM saturation behavior for differences at -100 dB is uncharacterized; closed source makes numerical audit impossible. Not usable as a CI gate; its 100 dB SPL full-scale convention is adopted here.**

### 1.3 ViSQOL / ViSQOLAudio
ViSQOL v3 (Chinen et al., QoMEX 2020; Apache-2.0, github.com/google/visqol) computes gammatone-spectrogram NSIM mapped to MOS-LQO. Its audio mode is trained on codec degradations (down to ~24 kbps) and its own README caps the audio-mode output at ~4.75 MOS even for identical inputs, warning that "single scores are not very meaningful." Production usage (SoundStream, Lyra) is regression *tracking* during codec development, with MUSHRA listening tests for actual claims.

**Transfer verdict: worst fit of the surveyed metrics — for near-identical renders the entire comparison lives inside the mapping's saturation zone. Nothing transfers except the division of labor it exemplifies (objective metric for CI iteration, listening test for ground truth).**

### 1.4 The masking-model tradition from perceptual coding (NMR / psychoacoustic models 1 & 2)
The masking-model tradition asks this issue's exact question. Brandenburg's NMR (AES preprint 2433, 1987; Brandenburg & Sporer, AES 11th Int. Conf. 1992) is the ratio of error energy to the masked threshold derived from the reference, per critical band; the **masking flag** marks any band-frame where error exceeds the masked threshold (predicted audible). NMR survives inside PEAQ as the Total/Segmental NMR MOVs. The verified inaudibility criterion, from Fraunhofer's own application of the tool (Neubauer & Herre, AES 4823): "as a rule of thumb a NMR below -10 dB is considered to indicate 'imperceptible' distortions" (their NMRtot, an average). By construction, NMR < 0 dB in *every* band and frame — masking-flag rate zero — means the error is below the masked threshold everywhere, i.e., predicted inaudible with no averaging caveat.

The threshold construction underneath it is standard and primary-verifiable (Painter & Spanias, Proc. IEEE 88(4) 2000, reproduces the equations): Bark-domain decomposition (z(f) = 13 arctan(0.00076 f) + 3.5 arctan((f/7500)^2), Zwicker & Fastl); the Schroeder spreading function SF(x) = 15.81 + 7.5(x+0.474) - 17.5 sqrt(1+(x+0.474)^2) dB with +25/-10 dB/Bark asymptotic slopes (Schroeder, Atal & Hall, JASA 66(6) 1979); masker-type offsets — Johnston (IEEE JSAC 6(2) 1988): tone-masking-noise threshold "14.5 + z dB below" the spread masker energy, noise-masking-tone "5.5 dB below"; MPEG-1 model 1's masking indices are -0.275z - 6.025 dB (tonal) and -0.175z - 2.025 dB (noise) relative to masker SPL; PEAQ's spreading uses a 27 dB/Bark lower slope and a level-dependent upper slope (Kabal eq. 22). The underlying asymmetry data: tone-masking-noise minimum SMR ~21-28 dB (Schroeder/Atal/Hall; Hellman 1972), noise-masking-tone ~4-5 dB (Egan & Hake 1950).

**Transfer verdict: this is the tradition that transfers — definitionally.** The error-to-mask construction makes no assumption that the error is codec quantization noise; the only codec inheritance is in tuning (tonality estimation, spreading), which S3 removes by always assuming the weakest-masking case. The gate proposed below is a worst-window, worst-band NMR-style measure held at masking-flag-rate zero — strictly tighter than the verified mean-NMR < -10 dB rule of thumb.

### 1.5 Absolute threshold of hearing and playback calibration
The threshold in quiet is modeled by the Terhardt approximation Tq(f) = 3.64 (f/kHz)^-0.8 - 6.5 e^(-0.6 (f/kHz - 3.3)^2) + 1e-3 (f/kHz)^4 dB SPL (Terhardt 1979; equation primary-verified via Painter & Spanias 2000, eq. 1), consistent with the ISO 226:2003 free-field threshold table (minimum about -6 dB SPL at 3.15 kHz). Since a digital comparison has no intrinsic SPL, every psychoacoustic model must posit a playback calibration: MPEG-1 model 1 normalizes so a full-scale sinusoid sits near **90 dB SPL** (PN = 90.302 dB, Painter & Spanias §II-F); PEAQ defaults a full-scale sine to **92 dB SPL** (Kabal); PEMO-Q assumes amplitude 1.0 = **100 dB SPL** (official manual). The empirical anchor for what a calibration buys: Meyer & Moran (JAES 55(9) 2007; 554 double-blind trials, 49.8% correct) found a 16-bit A/D/A loop (TPDF-dithered floor ~ -93 dBFS, per Lipshitz, Wannamaker & Vanderkooy JAES 40(5) 1992) undetectable at normal-to-loud levels; its noise floor became reliably audible only at gains >= +14 dB above their 85 dB SPL reference — i.e., ~115 dB SPL full scale — in silence. Reiss's meta-analysis (JAES 64(6) 2016; 12,500+ trials) tempers overconfidence: trained listeners discriminate high-resolution formats slightly but significantly (~60% with training), so margins must assume trained ears.

This proposal calibrates **0 dBFS sine = 100 dB SPL** (matching PEMO-Q, 8-10 dB stricter than MPEG/PEAQ), with the sensitivity to this knob quantified in S3.4.

### 1.6 JND literature: level, and correlated error signals
The smallest verified level JNDs: **0.41 dB** for wideband noise >= 30 dB SL (Miller, JASA 19(4) 1947); **0.5-0.93 dB** for pure tones at 80-40 dB SL (derived from Jesteadt, Wier & Green's fitted Weber function ΔI/I = 0.463 (I/I0)^-0.072, JASA 61(1) 1977); and the most sensitive published detection figure of merit for a *spectral* change: **~0.25 dB** for a just-detectable low-Q resonance on pink noise by trained listeners (Toole & Olive, JAES 36(3) 1988; music program is more forgiving, and Bücklein JAES 29(3) 1981 shows peaks are far more audible than dips).

Why this matters here: a cross-backend error that is *correlated with and spectrally shaped like the signal* is, in the limit, a pure level change — error at relative level x changes level by ~8.686x dB, so a -40 dB-relative correlated error is a 0.087 dB level change, a factor of ~5 below the smallest verified JND; the crossover where the correlated-level cue reaches the best-case 0.4 dB JND is an error at ~ **-26.5 dB relative** (derivation from the cited JNDs). But a correlated error *concentrated* in one band or window is a local spectral change governed by the much stricter masked thresholds of S1.4 (in the worst case, ~5 dB below an in-band noise masker). The gate therefore uses masked thresholds as the primary criterion and the JND literature as an independent cross-check: its implied full-band level-discrimination limit measures ~0.1 dB (S4.1), 8-12 dB of energy margin below every verified JND.

### 1.7 Roundoff noise in recursive filters (why divergence is bounded but amplified)

The propagation of per-sample rounding differences through a stable IIR filter is classical finite-wordlength theory: injected roundoff acts as a noise source filtered by the system, with output variance scaled by the filter's noise gain (the L2 norm of the impulse response from the injection point) — large for high-Q, low-frequency poles (Jackson, "On the Interaction of Roundoff Noise and Dynamic Range in Digital Filters," Bell Syst. Tech. J., 1970 — noise expressed through Lp norms of the error-injection-to-output responses; Weinstein & Oppenheim, Proc. IEEE 1969 for the floating-point case; textbook treatment in Oppenheim & Schafer's finite-word-length chapters: output noise variance = (q^2/12)·Σ h_e^2[n]. For a two-pole resonator the noise gain grows like 1/(1-r^2) as the pole radius r approaches 1; audio-specific treatments: Wilson "Filter Topologies," AES UK DSP Conf. 1992; JOS Smith, *Introduction to Digital Filters*, numerical-robustness sections). Two consequences carry over directly to cross-backend divergence: it is **bounded and stationary for stable filters** (the difference between two arms is itself the output of a stable system driven by bounded per-op rounding discrepancies), and its level is program- and topology-dependent, spanning tens of dB across EQ settings — confirmed empirically in §2.

### 1.8 Engineering precedent: how shipping systems gate "same audio" across implementations

- **Web Platform Tests / Web Audio API** conformance (`webaudio/resources/audit-util.js`) compares rendered buffers with per-sample mixed abs/rel tolerances, max-ULP differences, and per-test SNR thresholds — shipped tests require SNR >= 110-130 dB for oscillator waveforms and per-sample error <= 9.79e-8 (~1.6 ulp at unit scale) for biquads. This is exactly the regime of a cross-backend render gate, but the thresholds are empirically tuned per node, not psychoacoustically derived.
- **Opus (RFC 6716 §6)** is the closest published relative: decoder conformance explicitly abandons bit-exactness and gates on `opus_compare`, a frequency-weighted windowed spectral comparison whose zero point "was calibrated... [to] correspond to additive white noise with a 48 dB SNR (similar to what can be obtained on a cassette deck)," with implementations "RECOMMENDED" to score quality > 90/100 — a calibrated floor plus a large margin. Precedent for: windowed spectral comparison, a physically-anchored pass line, and margin-above-floor; but its threshold is a codec conformance bar far laxer than inaudibility and does not transfer numerically. The RFC also concedes a low score can still be inaudible "unless this is verified by listening tests" — the same metric-vs-ground-truth split adopted here.
- **ViSQOL in production codec work** (SoundStream/Lyra): used as a development regression metric, demonstrating the "objective metric as CI signal, listening tests as ground truth" division of labor this issue adopts.
- **glibc libm** gates its own math-accuracy regressions on per-architecture max-ulp tables (`libm-test-ulps`) — the reference-implementation-plus-frozen-tolerance discipline S5 adopts.
- **No prior art in GPU audio**: the GPU-audio literature and commercial offerings (GPU Audio Inc., convolution-reverb ports) evaluate speed and latency, never psychoacoustic output equivalence; the survey found no published psychoacoustically-grounded CPU-vs-GPU render qualification. This gate would be first of its kind, which raises the evidence bar on its calibration (S4).

### 1.9 ABX as ground truth
Double-blind ABX (Clark 1982, "High-Resolution Subjective Testing Using a Double-Blind Comparator", JAES 30(5)) is the operational definition of "audibly identical": if trained listeners cannot exceed chance identifying X as A or B, the difference is not audible *in that test*. Standard statistics: 16 trials with >= 12 correct gives exact binomial p = 0.038; Leventhal (1986, "Type 1 and Type 2 Errors in the Statistical Analysis of Listening Tests", JAES) showed that small-N ABX tests have high type-II error — a failed ABX is weak evidence of inaudibility unless powered (on the order of 45 trials for 80% power against a listener with true per-trial detection probability 0.7). ITU-R BS.1116 codifies the small-impairment methodology (double-blind triple-stimulus hidden reference, expert listeners, the 5-grade continuous impairment scale where 5.0 = imperceptible; "transparent" operationally means the hidden reference and the test item are not distinguished). The engine already carries preregistered ABX machinery and facilitator protocol (`dsp-research/listening/`, issues #007/#033/#111); class-P uses it as the appeals court, not the routine gate (S4.3).

**Transfer verdict**: methodology transfers wholesale; it is the ground truth this metric is calibrated against. Its limitation is the reason an objective gate is needed at all: ABX cannot run per-commit, and can never *prove* inaudibility — only bound detection probability under a stated power.

### 1.10 Transfer summary

| Source | What transfers | What does not |
|---|---|---|
| PEAQ / BS.1387 | psychoacoustic architecture (ear model, masked threshold, NMR-family MOVs); the 92 dB SPL listening-level convention | the ODG regression and its thresholds (trained on codec artifacts orders of magnitude above our differences; near-transparent region is exactly where ODG saturates and implementations disagree) |
| PEMO-Q / ViSQOL | validation-domain lesson: correlation with MOS on codec-grade impairments | numeric thresholds; MOS-scale outputs saturate for near-identical inputs |
| MPEG PM1/PM2, Johnston | masking-threshold construction: Bark decomposition, spreading function, masker-type offsets, threshold in quiet | bit-allocation-oriented tunings (they may *under*-estimate audibility for a codec's purposes; we re-bias every choice conservative) |
| JND / intensity discrimination | the 0.5–1 dB level-JND floor as an independent cross-check on correlated (signal-shaped) error; Weber-fraction framing | direct use as a gate (JND is a full-band construct; our errors are band-local) |
| Opus/WPT practice | windowed spectral comparison + calibrated pass line + reference-implementation discipline | their numeric thresholds (conformance bars, not inaudibility bars) |
| ABX / BS.1116 | ground-truth methodology, small-impairment test discipline, statistics | nothing numeric to reuse; ABX proves audibility, never inaudibility |

## 2. The floating-point reality: what cross-backend divergence actually looks like

### 2.1 Scale of a single rounding difference

IEEE 754 binary32 has a 24-bit significand. The unit roundoff is `u = 2^-24 ≈ 5.96e-8`, i.e. every correctly-rounded op carries relative error ≤ `2^-24` = **-144.5 dB**; the ulp spacing at 1.0 is `2^-23` = **-138.5 dB** relative to full scale. A GPU backend diverges from the CPU SIMD backend through four mechanisms, none of which are bugs:

- **FMA contraction** — `a*b+c` fused (one rounding) vs multiply-then-add (two roundings). Base Wasm SIMD is pinned non-fused in this engine (AGENTS.md; `dsp-research/simd-numerics.md`), native x86-64-v3 and every GPU ISA fuse by default.
- **Reduction/association order** — mix-bus and matrix sums re-associated by wide SIMD/warp reductions.
- **Transcendental implementations** — `expf/logf/powf/sinf` differ per math library; GPU vendors document only error bounds, not bit results. The bounds are not uniformly tight: CUDA documents small ulp bounds for `expf`/`logf`, but WGSL (WebGPU) guarantees only **absolute error <= 2^-11 for `sin`/`cos`** (about -66 dB — grossly insufficient for audio if used naively), up to 4096 ulp for `atan2`, and `pow` composed as `exp2(y*log2(x))` with compounding error; WGSL also explicitly licenses reassociation and fusion, and Metal compiles with fast-math by default (survey S1.8/S3). Wasm, by contrast, is strictly IEEE-deterministic for basic ops.
- **Fast-math / flush-to-zero defaults** — the engine already flushes subnormals by policy (`flushed()` in `miso-engine-compressor`), which removes one class of divergence; contraction and reassociation remain.

Two load-bearing design consequences (engineering judgment, both cheap): **(a)** filter/ballistics coefficients are computed once on the control plane in f64 and shipped bit-identically to all backends — never recomputed with backend transcendentals (see the coefficient-path measurement below); **(b)** every per-sample transcendental on a GPU backend uses the engine-owned polynomial kernels the CPU strip already uses (`fast_level_db`/`fast_gain_from_db`) — never a WGSL/Metal builtin — so cross-backend transcendental divergence stays at the few-ulp level modeled here instead of WGSL's -66 dB license. With (a)+(b), the measured per-sample arithmetic divergence below is the realistic ceiling.

### 2.2 Is divergence bounded? Per-effect analysis of the actual engine set

The danger is not the per-op -144.5 dB; it is recursive accumulation of differently-rounded results through stateful DSP. For the engine's launch effect set:

**TPT SVF EQ sections (`miso-engine-parametric-eq`, builtins HPF/LPF).** A stable recursive filter is a contraction on state perturbations: an injected state error decays with the pole radius, so divergence between two arms is the sum of per-sample rounding-difference injections convolved with the (stable) error-propagation impulse response. It is **bounded and stationary — but amplified by the filter's noise gain**, which scales like the L2 norm of the impulse response (large for high-Q, low-frequency sections where the pole radius approaches 1). This is the classical roundoff-noise-accumulation result for fixed/floating IIR filters (see survey §1.7). Measured, f32-exact emulation of a peaking section (DF1 biquad as a same-pole-radius proxy for the production TPT SVF — DF1's noise gain at low frequency is equal or worse, so these numbers are a conservative ceiling; the actual SVF kernel is measured immediately below), non-fused left-to-right arm vs FMA-contracted reverse-order arm, 10 s of music-like program at 48 kHz, peak -1 dBFS:

| Section | pole radius | err RMS (dBFS) | err peak (dBFS) | err rel. to signal |
|---|---|---|---|---|
| 60 Hz, Q=8, +6 dB | 0.999653 | **-89.0** | **-74.4** | -66.3 dB |
| 100 Hz, Q=4, +6 dB | 0.998842 | -99.0 | -81.3 | -76.4 dB |
| 1 kHz, Q=1, +6 dB | 0.954817 | -135.4 | -107.1 | -112.7 dB |
| 10 kHz, Q=2, -9 dB | 0.650448 | -164.4 | -138.5 | -141.8 dB |

Per-second RMS trajectories are flat over 10 s and 60 s runs (no growth; the 60 s run's first-10 s and last-10 s means agree within 1 dB). Naive intuition ("differences live at -140 dB") is wrong for high-Q low-frequency filters in this topology: **divergence reaches -74 dBFS peak**, spectrally concentrated at the resonance — exactly where the reference signal has energy (correlated error).

**The production topology is far better-conditioned.** The same experiment on the engine's actual TPT SVF bell form (Zavalishin trapezoidal integrators) measures err RMS **-147.0 dBFS** (peak -131.4) at 60 Hz/Q8 and -152 to -164 dBFS elsewhere — 40–60 dB better than DF1 at the hard corner, flat trajectories, EMR (§3) around **-72 dB**. This is the known numerical robustness of trapezoidal SVF integrators at low frequency, now measured for the cross-backend divergence question specifically. Consequence: the DF1 table above is retained as the **planning ceiling** — the level a class-P gate must tolerate if a worse-conditioned form ever enters the strip (e.g., the scan-form parallel IIR of #233 §1, whose high-Q audio-band error behavior is unpublished) — while the production strip's realistic divergence is compressor-dominated at ≈ -116 dBFS RMS.

**Coefficient-path divergence is larger and avoidable.** If each backend computes its own filter coefficients with its own `cos/sin`, a 1-ulp difference in `a1` at 60 Hz/Q=8 shifts the center frequency by ~0.2% of itself and produces a **-58.7 dBFS RMS** correlated error (-36 dB relative to signal, ≈ 0.14 dB response deviation at resonance). Recommendation (engineering judgment, but load-bearing): **coefficients must be computed once on the control plane in f64 and shipped bit-identically to every backend**, exactly as the plan-compilation architecture already implies. That removes this term entirely; the per-sample arithmetic divergence (table above) is what remains.

**Compressor gain word (`miso-engine-compressor`).** The engine smooths gain-reduction in dB: `g' = c*g + (1-c)*t` with `c = exp(-1/(τ*fs)) ∈ (0,1)` — a contraction with the same coefficient in both arms up to 1 ulp, so state divergence cannot grow. The attack/release branch (`target < g`) is a hard decision boundary; a diverged state can flip it, but both branches remain contractions toward the same target, so flips are self-healing. Production uses engine-owned polynomial dB conversions (`fast_level_db`/`fast_gain_from_db`, #233 §0), so a GPU port shares the algorithm and diverges only through per-op rounding/contraction inside the polynomial — which is what the emulation models: every transcendental result perturbed by +1 ulp in one arm plus FMA contraction of the smoother. Measured: output error RMS **-115.8 dBFS**, peak -92.1 dBFS, max gain deviation 8.6e-4 dB, **2 branch flips in 480k samples**, flat per-second trajectory.

**True-peak limiter (`miso-engine-true-peak-limiter`).** The 12-tap x 4-phase BS.1770 detector FIR is feed-forward (divergence: a few ulp, non-accumulating); the van Herk sliding minimum is compare-select over nearly identical values; the release ballistic is a one-pole contraction. Bounded by the same arguments. (Not separately simulated; covered by the chain measurement below and by the metric's calibration corpus.)

**Full-chain composition.** Five cascaded high-Q sections followed by the compressor recursion, both arms end-to-end: error RMS **-95.1 dBFS**, peak -78.7 dBFS, flat trajectory. Divergence through a realistic serial chain accumulates roughly as the RMS sum of per-stage contributions filtered by downstream stages — bounded, stationary, correlated with the program.

**The one genuinely unbounded-looking mechanism: threshold-crossing time shifts.** A gate/expander (or any hard decision with memory) converts a level difference δL (dB) in its detector into a *timing* difference of the decision: Δt ≈ δL / (envelope slope in dB/s). With upstream divergence of ~1e-3 dB (compressor case) and a 1 dB/s swell, Δt ≈ 1 ms; with a pathological -36 dB-relative upstream error (unshared coefficient path), Δt could reach ~100 ms. The difference signal during Δt is the gated content itself — at the gate threshold level, i.e. quiet, and perceptually this is "the gate opened 100 ms later on a slow swell", which no listener can attribute — but a naive sample-domain or even worst-window spectral metric can flag it. This is a *metric design constraint*, not an instability: divergence stays bounded in signal level; it does not stay bounded as a fraction of ulp. Consequences for the metric are handled in §3.5 and §7 (open questions).

### 2.3 What this means for the metric's shape

1. A **single full-render statistic hides worst cases** (divergence is stationary but program-dependent); a **per-block statistic at the render quantum (128) is far below auditory temporal integration** and would be dominated by unmaskable single-window noise estimates. The right granularity is the psychoacoustic one: **overlapped analysis windows of ~40 ms** (2048 @ 48 kHz), with the render verdict driven by the **worst window** (and distributional diagnostics).
2. The error is **spectrally correlated with the program** (it lives at resonances and under program energy), so an unweighted broadband level test either fails correct renders (-74 dBFS peaks) or is too lax elsewhere. The metric must compare error to the **frequency-local masked threshold of the reference render** — which is exactly the error-to-mask tradition from perceptual coding (survey §1).
3. Absolute floors matter: in silence the masked threshold is zero, so an **absolute threshold-of-hearing floor under a stated playback-level calibration** is required to avoid failing renders for -120 dBFS differences in digital black.

## 3. Proposed measurement: worst-window, worst-band error-to-mask ratio (EMR)

### 3.1 Definition

Given two sample-aligned renders of the same prepared plan — reference `R` (CPU SIMD leg) and candidate `T` (GPU leg) — compute the difference `e = T - R` in f64 and analyze `e` against a masking model of `R`:

1. **Windowing.** Hann window, N = 2048 at 44.1/48 kHz (N = 4096 at 88.2/96 kHz), 50% overlap → 42.7 ms analysis frames. Rationale: threshold-level temporal integration is ~3 dB per doubling of duration out to ~100-300 ms (Plomp & Bouman 1959), so a brief artifact needs *more* level to be detected than a steady one by ~10·log10(200 ms/T); the worst-case energy dilution of a T-ms artifact inside a 42.7 ms window (10·log10(42.7/T)) is always smaller than that elevation, so the window never hides a brief artifact behind its own averaging — confirmed empirically by the 5 ms burst rows in §4.1. Effective premasking is only 1-2 ms (Painter & Spanias §II-D, after Zwicker & Fastl) — another reason the verdict must come from the worst window, not from any average. The window also gives ~23 Hz resolution below the first Bark edge.
2. **Band decomposition.** Power spectra of `R` and `e` per frame, normalized so a full-scale sine measures 0 dBFS band energy; energies summed into 1-Bark-wide bands over 20 Hz–20 kHz (Zwicker Bark scale; ~25 bands).
3. **Masked threshold of the reference.** Spread the reference band energies with a two-slope, level-independent spreading function: **27 dB/Bark toward lower frequencies and 15 dB/Bark toward higher frequencies**. Both slopes are steeper (stricter) than the classical values — Schroeder/Atal/Hall's function has +25/-10 dB/Bark asymptotic slopes, and MPEG model 1 / PEAQ upper skirts flatten further with level (PEAQ: -24 - 230/f + 0.2L dB/Bark; its lower slope is the same 27) — so masking is never over-credited. Subtract the **tone-masking-noise offset `(14.5 + z) dB`** (Johnston 1988, primary-verified: TMN threshold "14.5 + z dB below" the spread masker) in *every* band — always the weakest masker type, never the 5.5 dB noise-masking-tone offset or MPEG's laxer masking indices (-0.275z - 6.025 tonal). This is the deliberate conservatism: real program material is partly noise-like and masks 8-25 dB better than credited here.
4. **Absolute floor.** Threshold in quiet per band from the Terhardt approximation (primary-verified via Painter & Spanias eq. 1), mapped to dBFS with a **stated playback calibration: 0 dBFS sine = 100 dB SPL** — exactly PEMO-Q's convention, 8-10 dB stricter than PEAQ's 92 dB SPL default and MPEG model 1's ~90 dB; a knob the owner sets (§3.4, §7). Band threshold `M[z] = max(masked[z], quiet[z])`.
5. **EMR.** Per frame and band: `EMR[frame, z] = 10*log10(E_e[frame, z] / M[frame, z])`.

### 3.2 Decision rule

- **Per fixture:** `EMR_max = max over frames and bands`. **PASS iff `EMR_max <= 0 dB`.** Frames where `e` is exactly zero contribute -inf (class-A-identical blocks pass trivially).
- **Warn band:** `-10 dB < EMR_max <= 0 dB` → pass, but the sweep row records the value and worst frame/band for trend tracking; a fixture drifting toward 0 is investigated before it fails.
- **Per corpus:** class-P for a backend pair holds only if every fixture in the corpus passes; the sweep reports the corpus-wide worst row.
- **Preconditions (hard fail before scoring):** identical length, identical PDC/latency (sample alignment is guaranteed by the shared prepared plan, and the metric is *deliberately not* shift-invariant — a 1-sample shift scores EMR_max ≈ +42 dB on the calibration program; two renders that disagree on timing are not class-P equivalent), no NaN/Inf, and class-A short-circuit: byte-identical output passes without analysis.

### 3.3 Why worst-window/worst-band, not averages

PEAQ/PEMO-Q/ViSQOL average distortion over seconds to predict *graded annoyance*; a CI gate must instead bound *detectability of the worst moment*, because ABX listeners are free to focus on any instant (survey §1). A mean-EMR statistic would let a single audible 40 ms glitch hide inside 10 clean seconds. `EMR_p95` and `EMR_mean` are reported as diagnostics only.

### 3.4 Threshold derivation and safety margins

The pass line is `EMR_max <= 0 dB`, i.e. **error energy in every ~43 ms window and every Bark band stays below a deliberately pessimistic estimate of the masked/absolute threshold**. The conservatism lives in the model, and each layer is separately grounded:

| Layer | Choice | Established basis | Margin direction |
|---|---|---|---|
| Masker-type offset | TMN `(14.5+z)` dB everywhere | Johnston 1988 (primary-verified) | 8-25 dB stricter than noise-masker reality (NMT 5.5 dB) |
| Spreading skirts | 27/15 dB/Bark fixed | vs Schroeder +25/-10 (JASA 1979) and PEAQ's level-dependent upper skirt | stricter (less masking credited) |
| Temporal masking | none credited | post-masking 50-300 ms is real (Zwicker & Fastl via Painter & Spanias) | stricter |
| Playback level | 0 dBFS = 100 dB SPL | = PEMO-Q manual convention; PEAQ default 92 (Kabal), MPEG ~90 | 8-10 dB stricter than ITU/ISO practice |
| Statistic | worst window, worst band; masking-flag rate 0 | tighter than the verified Fraunhofer mean-NMR < -10 dB rule (Neubauer & Herre AES 4823) | strictest pooling |

Cross-checks against independent psychoacoustics (§4 calibration table): the gate's implied full-band level-JND is ~0.1 dB — below the smallest verified JNDs (0.41 dB broadband noise, Miller 1947; 0.5-0.93 dB tones, Jesteadt et al. 1977; 0.25 dB trained resonance detection, Toole & Olive 1988) by 8-12 dB of energy — and its implied broadband noise-floor tolerance is ~ -100 dBFS (16-bit TPDF dither at ~ -93 dBFS would *fail*, consistent with Meyer & Moran's finding that the 16-bit floor is audible at >= +14 dB above an 85 dB SPL reference; -120 dBFS passes by 32 dB). Both cross-checks land the gate 5-15 dB on the strict side of the verified literature, which is where a gate should sit.

Measured corridor on the calibration corpus: the worst *planning-ceiling* inaudible case (DF1-topology high-Q divergence, §2.2) scores **-16.6 dB**; the worst *actual-strip* divergence (compressor) scores -44, and the production SVF EQ -72; the first solidly-audible case (+0.5 dB gain) scores **+13.4 dB**. That is a ~30 dB separation corridor around the 0 dB line at cal = 100 dB SPL even against the pessimistic topology ceiling, and >55 dB for the strip as built. Sensitivity: floor-dominated cases move dB-for-dB with the calibration level (at 92 dB SPL the DF1 ceiling case scores -24.6; at a pessimal 110 dB SPL, -8.6); masking-dominated cases are invariant.

### 3.5 Known limitations (stated up front)

- **Decision-timing pathologies** (§2.2): a fixture engineered to hover an envelope within micro-dB of a gate threshold can turn bounded state divergence into a >40 ms decision shift and fail EMR while remaining perceptually unattributable. Policy: corpus fixtures must not sit statically on decision thresholds (they may *cross* them); an EMR failure whose difference energy is confined to a decision window is investigated, never auto-waived.
- **No spectral loudness summation across bands**: many bands each just below threshold could in principle integrate to detectability. The tonal-offset conservatism (8–25 dB) is the budget covering this; flagged as an open question with a listening-verification path (§7).
- **Model, not oracle**: the gate's authority comes from the calibration/validation protocol in §4 plus the engine's existing preregistered ABX machinery (`dsp-research/listening/`), which remains the ground truth for any disputed verdict.

## 4. Validation plan for the metric itself

The gate is only as trustworthy as its own test suite. Three layers:

### 4.1 Calibration corpus (already executed in this research pass, to be frozen as fixtures)

20 s music-like program (tonal chord decays + pink-ish bed + drum transients, peak -1 dBFS, deterministic seed), 48 kHz, metric at cal = 110 dB SPL (the strictest setting; numbers below are the measured worst case):

| Perturbation | Expected | `EMR_max` (dB) | Verdict at 0 dB line |
|---|---|---|---|
| identical | pass | -inf | pass |
| 1-ulp random dither on every sample | pass | -67.0 | pass, 67 dB margin |
| TPDF noise at -120 dBFS | pass | -32.1 | pass |
| realistic FP divergence, compressor (transcendental+FMA) | pass | -44.3 | pass |
| realistic FP divergence, EQ 100 Hz Q4 | pass | -21.3 | pass |
| FP divergence, DF1 EQ 60 Hz Q8 (planning ceiling) | pass | -8.6 | pass (16.6 dB margin at cal=100) |
| FP divergence, production TPT SVF EQ 60 Hz Q8 | pass | -72.6 | pass |
| full chain 5×EQ + compressor divergence | pass | -13.0 | pass |
| white noise -100 dBFS | borderline by design | -4.4 | pass (fails nothing audible; at 110 dB SPL playback this noise sits at ~10 dB SPL in the 2–8 kHz threshold dip) |
| white noise -80 dBFS | fail under loud playback | +15.7 | fail |
| white noise -60 dBFS (clear hiss) | fail | +35.3 | fail |
| 5 ms noise bursts at -40 dBFS | fail | +21.5 | fail |
| static gain +0.1 dB | inaudible per level-JND literature | -0.8 | pass, marginal — consistent with JND ≈ 0.5–1 dB |
| gain step +0.1 dB mid-render | borderline | -0.8 | pass, marginal |
| static gain +0.5 dB (≈ level JND) | fail (conservative) | +13.4 | fail |
| static gain +1.0 dB | fail | +19.6 | fail |
| polarity inversion of the 1–4 kHz band | fail | +44.1 | fail |
| 1-sample time shift | must fail (alignment precondition) | +41.8 | fail |

The separation is clean: every known-inaudible perturbation passes with ≥ 8.6 dB margin even at the pessimal calibration (≥ 16.6 dB at the proposed 100 dB SPL), every known-audible one fails by ≥ +13 dB, and the two deliberately borderline rows (-100 dBFS noise, 0.1 dB gain) land within a few dB of the line on the correct side of the JND/threshold literature.

### 4.2 Red-mutation tests of the gate (CI self-tests, same style as `test-*-policy.sh`)

Each mutation must flip the gate's verdict, or the checker fails its own suite:

- inject -60 dBFS noise into one leg → gate must FAIL;
- apply +0.5 dB gain to one leg → FAIL;
- drop one sample from one leg → FAIL (precondition);
- corrupt the threshold table / spreading matrix constants → checker self-test must detect (golden EMR values on frozen fixture pairs, asserted to ±0.1 dB);
- weaken the statistic (mean instead of max) → the frozen "5 ms burst" fixture must expose it (burst passes on mean, fails on max);
- 1-ulp dither leg → gate must PASS (guards against over-tightening regressions).

### 4.3 Ground-truth anchoring (the step that makes it *psychoacoustic* rather than merely principled)

The engine already has preregistered double-blind ABX machinery (`dsp-research/listening/`, issues #007/#033/#111). One bounded listening session validates the corridor, not every render:

- Take the two borderline calibration rows (+0.5 dB gain fail-side; -100 dBFS noise pass-side) plus the worst realistic divergence pair (EQ 60 Hz Q8 arms A/B) rendered as WAV pairs.
- Preregistered ABX, per Clark's double-blind comparator methodology: 16 trials/listener with >= 12 correct as the p < 0.05 criterion (exact binomial p = 0.038). Expectation: *no discrimination* on pass-side pairs, reliable discrimination on fail-side pairs. A pass-side pair that any listener reliably discriminates falsifies the model and blocks class-P adoption until tightened.
- Power the null claim properly (Leventhal 1986: an underpowered failed ABX is not evidence of inaudibility). For 80% power against a listener whose true per-trial detection probability is 0.7, on the order of 45 trials are needed; the preregistration must state trial counts and the effect size the null is powered against.
- Note the asymmetry honestly: ABX can prove audibility, never inaudibility; for pass-side pairs the claim is bounded ("not discriminable in N trials at power X against effect size Y"), which is why the metric's margins — not the listening test — carry the inaudibility argument.

## 5. Integration sketch

The engine already renders the same fixture on multiple legs and compares digests (rack/builtins bench tools emit `output_sha256` per workload row; `scripts/check-protocol-wasm-parity.sh` runs scalar and simd128 Wasm variants of the same golden assertions; `tools/miso-engine-native-pcm-runner` renders a session TOML to block-planar f32le over the frozen C ABI with digest verification). Class-P slots beside that, not inside it:

- **New offline tool `tools/miso-engine-classp-audit`** (native, not realtime-constrained): takes two rendered f32le outputs (or renders both legs itself via the runner), verifies preconditions, computes windowed EMR, emits one JSON row per fixture pair: `{fixture, ref_leg, test_leg, emr_max_db, emr_p95_db, emr_mean_db, worst_frame, worst_band_hz, cal_db_spl, metric_version, class_p: "pass"|"fail"}`. The DSP is ~25 lines of spec: Hann 2048/50%, rFFT, Bark fold, fixed spreading matrix, fixed offset/floor tables — all constants frozen in the tool and covered by golden self-tests (§4.2).
- **Sweep semantics.** Class-A rows remain authoritative for `{scalar, simd4, simd8, wasm-simd128}` legs: identical `output_sha256` or fail (already machine-independent by design, #197). A cross-class pair `{cpu-reference x gpu-cuda | gpu-wgpu | gpu-metal}` gets a class-P row instead — unless the backend proves class-A (per #233 §4.3, CUDA with unfused IEEE ops is a real class-A candidate, in which case its row stays a digest row and class-P applies only to the genuinely non-bit-exact backends and forms). A GPU leg must additionally be **class-A against itself** (same device + driver renders twice → identical digest), so intra-backend nondeterminism is still caught by the cheap gate.
- **Consumers already queued (#233):** the phase-1 offline GPU bounce go/no-go ("class-A on CUDA, or class-P divergence inside the psychoacoustic gate"), the speculative-cache seam certification (splice continuity of cached-GPU block against CPU-resumed block), and any scan-based IIR reformulation (reassociation breaks class-A by construction; Zhai & Paris measure ~1e-7 relative error for cascaded-SOS parallel forms in f32, but no published audio-band error analysis exists at high Q — the class-P corpus's high-Q fixtures are exactly the missing evidence).
- **Corpus.** Start with the existing fixture sessions (`fixtures/native-pcm-runner/v1`, rack/builtins fixture programs) extended with class-P stress fixtures: high-Q low-frequency EQ (the measured worst case), deep compressor ballistics, limiter at threshold, digital-black tails, and full-chain sessions. Fixture sources must be seconds long, not the current 1024-sample smoke inputs.
- **Runtime cost.** One rFFT-2048 per 1024 samples per channel plus O(bands²) spreading — ~0.5 MFLOP/s of audio, orders of magnitude below render cost; wall-time is dominated by the renders themselves. No realtime-plane involvement.
- **Reference implementation discipline.** The Python metric from this research pass becomes `scripts/`-side reference (like `effect-*-reference.py`); the Rust tool must match it to ±0.1 dB on frozen fixtures — same dual-implementation pattern the repo already uses for CID/state references.

## 6. First customer: relaxed-SIMD FMA on Wasm (owner directive, 2026-08-28)

The owner's directive: "if the psychoacoustic benchmark allows, we might be able to enable the fused FMA on WASM." This makes class-P's first paying customer the *browser CPU path*, not the GPU:

- **Standing state.** The #163 phase-2 ruling made the whole stack unfused (two-rounding multiply-add) because wasm simd128 has no FMA and software-emulated fusion cost ~54 instructions per madd — deleting it bought the browser 5.61x (964 -> 173 µs/block, 64-track console). #172 tracks re-adopting fused arithmetic "when deterministic hardware FMA ships." Relaxed SIMD's `f32x4.relaxed_madd` ships hardware FMA *today* — but deliberately non-deterministically: each client device may fuse or not, which class-A forbids across the install base (outputs are currently machine-independent by design, #197).
- **What class-P changes.** The divergence class "same program, same op order, each madd either fused or not" is the *best-behaved* divergence class in this entire issue: no transcendental variance, no reassociation, no reduction reordering — exactly one rounding difference per madd, bounded by u·|a·b| per op, then shaped by filter noise gain. It is strictly contained in the arm-A-vs-arm-B experiments of §2.2 (which additionally reversed accumulation order): production-topology SVF cascades measure **-147 dBFS RMS divergence (EMR ~ -72 dB)** at the hardest corner, and even the deliberately bad DF1 topology stays 8.6 dB inside the gate at the pessimal calibration. Certifying fused-vs-unfused as class-P-null is comfortably within the measured corridor.
- **Gate mechanics.** The reference leg stays the unfused deterministic build (class-A anchor). A `relaxed_madd` build is one more leg in the sweep: class-A against itself on any single device (fusion choice is fixed per device), class-P against the unfused reference. The calibration corpus (§4.1) gains the fused-vs-unfused arm as a permanent known-inaudible anchor — it is simultaneously the shipping configuration under test, which red-mutation tests must not be able to weaken.
- **Payoff.** Immediate, measurable browser speedup on FMA-dense kernels (SVF cascades were the site of the softfma tax; #163 measured 5.5x on that kernel from the fusion question alone), and native re-fusing recovers its +3.5% unfused overhead — all independent of the GPU roadmap, and a low-risk shakedown cruise for the class-P machinery before a GPU backend exists. #172's re-adoption condition ("deterministic hardware FMA") can then be re-scoped: class-P certification substitutes for determinism on the fused legs, while the unfused build remains the deterministic reference.

## 7. Open questions for the owner

1. **Calibration level.** 100 dB SPL for 0 dBFS sine is proposed (8 dB stricter than BS.1387's 92). Mastering-grade paranoia says 110; that costs the worst realistic EQ-divergence case 8 dB of its margin (-16.6 → -8.6). Where do we set it?
2. **Does class-P also gate future fast-math / relaxed-SIMD CPU optimizations?** The machinery is backend-agnostic; recommending yes, with class-A retained per (backend, opt-level) leg — but that widens the class-P surface and the owner should size the corpus accordingly.
3. **Gate/expander decision-window policy** (§3.5): investigate-on-fail, or an explicit carve-out with its own bound (e.g., decision shift ≤ N ms and shifted-content level ≤ threshold + 6 dB)?
4. **Corpus breadth vs. runtime**: how many program-material fixtures (music-like, sparse/quiet, digital black, pathological high-Q) per backend pair per CI run, and is class-P per-PR or nightly?
5. **Is `EMR_max <= 0` with warn at -10 the right posture, or should the pass line itself carry an explicit numeric margin (e.g., -6 dB) at the cost of headroom against future, larger-divergence effects (reverbs, long feedback delays)?**
6. **Stereo/multichannel pooling**: score L/R independently (proposed — dual-mono architecture) vs. mid/side analysis to catch interaural-difference artifacts; binaural unmasking (BMLD) can lower thresholds for antiphasic errors, which the current model does not credit — worth a bounded literature check before v1 freezes.
7. **Relaxed-FMA rollout shape (§6)**: is class-P certification on the fixture corpus sufficient to ship `relaxed_madd` to all capable devices, or does the owner want a device-lab pass (fused/unfused pairs re-scored on real hardware) first? And does #172 get re-scoped now or after the gate exists?

## 8. References

Standards and primary methods
- Rec. ITU-R BS.1387-2 (05/2023), *Method for objective measurements of perceived audio quality* (PEAQ). https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1387-2-202305-I!!PDF-E.pdf
- Rec. ITU-R BS.1116-3 (02/2015), *Methods for the subjective assessment of small impairments in audio systems*. https://www.itu.int/dms_pubrec/itu-r/rec/bs/R-REC-BS.1116-3-201502-I!!PDF-E.pdf
- ISO 226:2003, *Normal equal-loudness-level contours* (threshold table; 2023 edition exists). https://cdn.standards.iteh.ai/samples/34222/d93363dbdafa470aab734f04d091065b/ISO-226-2003.pdf
- RFC 6716 §6, Opus decoder conformance (`opus_compare`, 48 dB SNR anchor, quality > 90). https://www.rfc-editor.org/rfc/rfc6716.txt
- W3C WGSL spec §15.7.4-15.7.5 (floating-point accuracy; reassociation and fusion). https://www.w3.org/TR/WGSL/#floating-point-accuracy
- WebAssembly core spec, Numerics (IEEE 754 determinism; NaN-bit nondeterminism only). https://webassembly.github.io/spec/core/exec/numerics.html
- CUDA C++ Programming Guide §5.5, mathematical functions ulp tables. https://docs.nvidia.com/cuda/cuda-programming-guide/05-appendices/mathematical-functions.html
- Metal Shading Language Specification v4.1 (fast-math default). https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
- OpenCL C Specification, Table 65 (single-precision ulp bounds). https://registry.khronos.org/OpenCL/specs/unified/html/OpenCL_C.html

Psychoacoustics (primary-verified where noted)
- T. Painter, A. Spanias, "Perceptual Coding of Digital Audio," Proc. IEEE 88(4):451-513, 2000 — primary-verified source for the Terhardt threshold formula (eq. 1), Bark mapping (eq. 3), ERB (eq. 6), Schroeder spreading (eq. 7), MPEG-1 masking indices (eqs. 30-32), SPL normalization (§II-F), temporal masking (§II-D). https://www.cns.nyu.edu/~david/courses/perceptionGrad/Readings/PainterSpanias-ProcIEEE2000.pdf
- E. Terhardt, "Calculating virtual pitch," Hearing Research 1:155-182, 1979 (threshold approximation, via Painter & Spanias).
- J. D. Johnston, "Transform Coding of Audio Signals Using Perceptual Noise Criteria," IEEE JSAC 6(2):314-323, 1988 — primary-verified TMN offset 14.5+z dB, NMT 5.5 dB. https://www.ee.columbia.edu/~dpwe/papers/Johns88-audiocoding.pdf
- M. R. Schroeder, B. S. Atal, J. L. Hall, "Optimizing digital speech coders by exploiting masking properties of the human ear," JASA 66(6):1647-1652, 1979.
- E. Zwicker, H. Fastl, *Psychoacoustics: Facts and Models*, Springer 1990.
- B. R. Glasberg, B. C. J. Moore, "Derivation of auditory filter shapes from notched-noise data," Hearing Research 47:103-138, 1990.
- G. A. Miller, "Sensitivity to changes in the intensity of white noise...," JASA 19(4):609-619, 1947 (0.41 dB broadband level JND).
- W. Jesteadt, C. C. Wier, D. M. Green, "Intensity discrimination as a function of frequency and sensation level," JASA 61(1):169-177, 1977.
- F. E. Toole, S. E. Olive, "The Modification of Timbre by Resonances: Perception and Measurement," JAES 36(3):122-142, 1988. R. Bücklein, "The Audibility of Frequency Response Irregularities," JAES 29(3):126-131, 1981.
- R. Plomp, M. A. Bouman, "Relation between hearing threshold and duration for tone pulses," JASA 31:749-758, 1959.
- Egan & Hake, JASA 22:622-630, 1950; Hellman, Percept. Psychophys. 11:241-246, 1972 (masking asymmetry data, via Painter & Spanias).

Objective metrics and their critique
- P. Kabal, "An Examination and Interpretation of ITU-R BS.1387: Perceptual Evaluation of Audio Quality," McGill MMSP report, 2002 — primary-verified quotes on underspecification; PEAQ spreading slopes; 92 dB SPL default. https://www.mmsp.ece.mcgill.ca/Documents/Reports/2002/KabalR2002v2.pdf
- T. Thiede et al., "PEAQ — der künftige ITU-Standard...," 1998 account of TG10/4 databases. http://elvera.nue.tu-berlin.de/files/0829Thiede1998.pdf
- M. Torcoli, T. Kastner, J. Herre, "Objective Measures of Perceptual Audio Quality Reviewed...," IEEE/ACM TASLP 2021 (small-impairment generalization warning). https://arxiv.org/pdf/2110.11438
- P. M. Delgado, J. Herre, "Can We Still Use PEAQ?...," QoMEX 2020. https://arxiv.org/pdf/2212.01467
- R. Huber, B. Kollmeier, "PEMO-Q...," IEEE TASLP 14(6):1902-1911, 2006; PEMO-Q manual (100 dB SPL convention). https://uol.de/f/6/dept/mediphysik/ag/mediphysik/download/pemo_q/Manual_Pemo-Q.pdf?v=1680700821
- M. Chinen et al., "ViSQOL v3: An Open Source Production Ready Objective Speech and Audio Metric," QoMEX 2020. https://arxiv.org/pdf/2004.09584 ; https://github.com/google/visqol
- K. Brandenburg, AES preprint 2433, 1987; K. Brandenburg, T. Sporer, "'NMR' and 'Masking Flag'...," AES 11th Int. Conf., 1992. https://www.aes.org/e-lib/browse.cfm?elib=6276
- C. Neubauer, J. Herre, "Digital Watermarking and its Influence on Audio Quality," AES preprint 4823 — primary-verified mean-NMR < -10 dB rule of thumb. https://www.iis.fraunhofer.de/content/dam/iis/de/doc/ame/conference/AES4823_Digital_Watermarking_and_its_Influence_on_Audio_Quality.pdf
- N. Zeghidour et al., "SoundStream: An End-to-End Neural Audio Codec," IEEE TASLP 2021 (ViSQOL-for-iteration pattern). https://arxiv.org/abs/2107.03312

Listening methodology and audibility
- D. Clark, "High-Resolution Subjective Testing Using a Double-Blind Comparator," JAES 30(5):330-338, 1982.
- L. Leventhal, "Type 1 and Type 2 Errors in the Statistical Analysis of Listening Tests," JAES 34(6):437-453, 1986.
- J. Boley, M. Lester, "Statistical Analysis of ABX Results Using Signal Detection Theory," AES 127th Conv., 2009.
- S. P. Lipshitz, R. A. Wannamaker, J. Vanderkooy, "Quantization and Dither: A Theoretical Survey," JAES 40(5):355-375, 1992.
- E. B. Meyer, D. R. Moran, "Audibility of a CD-Standard A/D/A Loop...," JAES 55(9):775-779, 2007. https://drewdaniels.com/audible.pdf
- J. D. Reiss, "A Meta-Analysis of High Resolution Audio Perceptual Evaluation," JAES 64(6):364-379, 2016.

Finite-precision and cross-implementation numerics
- L. B. Jackson, "On the Interaction of Roundoff Noise and Dynamic Range in Digital Filters," Bell Syst. Tech. J., 1970. C. Weinstein, A. V. Oppenheim, Proc. IEEE, 1969.
- A. V. Oppenheim, R. W. Schafer, *Discrete-Time Signal Processing*, finite-word-length chapters. R. Wilson, "Filter Topologies," AES UK DSP Conf., 1992. J. O. Smith III, *Introduction to Digital Filters* (CCRMA), round-off sections.
- N. Whitehead, A. Fit-Florea, "Precision & Performance: Floating Point and IEEE 754 Compliance for NVIDIA GPUs," NVIDIA whitepaper, 2011. https://developer.download.nvidia.com/assets/cuda/files/NVIDIA-CUDA-Floating-Point.pdf
- P. Ahrens, J. Demmel, H. D. Nguyen, "Algorithms for Efficient Reproducible Floating Point Summation," ACM TOMS 2020 (ReproBLAS).
- W3C web-platform-tests WebAudio `audit-util.js` and shipped thresholds (SNR 110-130 dB; biquad 9.79e-8/sample). https://github.com/web-platform-tests/wpt/tree/master/webaudio
- X. Zhai, S. Paris (2026), parallel-scan IIR error measurements, arXiv:2607.23763 (via #233). A. Heinsen 2023, arXiv:2311.06281.

Repo grounding: AGENTS.md; `dsp-research/simd-numerics.md`; `docs/rulings/unfused-multiply-add-audit.md`; #163 (unfused contract, softfma ~54 instr), #172 (fused re-adoption), #177/#197 (machine-independent outputs), #233 (GPU architecture research, class-P consumers); `crates/miso-engine-compressor/src/lib.rs` (gain-word recursion), `crates/miso-engine-parametric-eq` (SVF sections), `tools/miso-engine-native-pcm-runner`, `scripts/check-protocol-wasm-parity.sh`, `dsp-research/listening/` (preregistered ABX machinery, #007/#033/#111).

## Appendix: experiment method detail

- **Program material**: deterministic 10–60 s music-like signal (decaying tonal chords, pink-ish noise bed with slow level modulation, 2 Hz drum-like transients with 60 Hz fundamentals), peak -1 dBFS, RMS ≈ -23 dBFS, 48 kHz.
- **f32-exact emulation**: every arithmetic op rounded to binary32 (numpy f32 scalar ops); FMA emulated as `f32(f64(a)*f64(b)+f64(c))` (exact product, one effective rounding; the double-rounding discrepancy vs. hardware FMA is negligible at the magnitudes measured). Arm A: non-fused, left-to-right accumulation (Wasm-simd128-style). Arm B: fused, reversed accumulation (GPU-codegen-style). Arm C: Arm A with one coefficient perturbed 1 ulp (backend-local transcendental in the coefficient path).
- **Compressor emulation**: the exact `miso-engine-compressor` structure — dB detector via `log10`, soft-knee gain computer with clamps, `g' = c*g + (1-c)*t` smoothing with `c = exp(-1/(0.001*τ_ms*fs))`, attack/release branch on `t < g`, linear gain via `10^(0.05*g)`, subnormal flush; Arm B additionally rounds every transcendental result up 1 ulp and fuses the smoother.
- **Metric prototype**: as specified in §3.1 (Hann 2048/50%, 1-Bark bands 20 Hz–20 kHz, 27/15 dB-per-Bark spreading, `14.5+z` offset, Terhardt floor at the stated calibration). All calibration-table numbers in §4.1 are from this prototype at cal = 110 dB SPL (pessimal); §3.4 corridor numbers restate the worst rows at cal = 100 and 92 dB SPL.
- Reproduction scripts (`progmat.py`, `exp1_biquad.py`, `exp2_compressor.py`, `emr_metric.py`, `exp3_calibrate.py`, `exp4_divergence_metric.py`, `exp5_longrun.py`, `exp6_svf.py`) are retained in the research scratchpad and should be committed under `dsp-research/` (or a successor issue's fixture tree) when this issue is briefed for implementation.
