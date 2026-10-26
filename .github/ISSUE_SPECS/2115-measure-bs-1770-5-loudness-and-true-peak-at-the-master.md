# Measure BS.1770-5 loudness and true peak at the master

Filed by root for the console sprint (2026-10-26). The owner chose loudness metering in the engine.

## Product outcome
An engine observation at the master (and any output) that gives ITU-R BS.1770-5 loudness: momentary (400 ms), short-term (3 s), integrated (gated) and loudness range (EBU Tech 3342), plus true-peak (BS.1770-5 Annex 2, 4x oversampling) in dBTP. It is readable by the console, by voice control and by future fan mixes on both hosts, through the core's observation frames.

## Smallest slice (the scope note decides)
K-weighting filters at the four launch rates, the 400 ms and 3 s windows with 75 % overlap, the absolute (-70 LUFS) and relative (-10 LU) gates, LRA by EBU Tech 3342, and true-peak via the existing Annex 2 detector code where possible. Reset and integration control (start or reset the integrated measure).

## Gates
EBU Tech 3341 and 3342 conformance signals (the minimum requirements), with each listed case within its tolerance; tests at all four launch rates; zero render-path allocation; the observation frames on both hosts.

## Decision record (root, 2026-10-26; scope note `scope/2115.md` accepted)
- **Shape.** A session-level output loudness meter, the output peak meter's twin, at the rendered master pair; not an
  effect tap, not a strip `MeterMetricSet` bit. Render half and control half live in the new `crates/loudness`; the
  control-plane wiring, arm gate and SPSC are S2; a new `LOUDNESS` (`0x0014`) monitor command is S2; the SDK is S3.
- **S1 paths.** `crates/loudness` (new), `crates/dsp-reference/src/{loudness.rs,lib.rs}`, `crates/true-peak-limiter/src/lib.rs`
  (`pub mod annex2`, visibility only: the limiter keeps calling the same code and no rendered bit moves), `Cargo.toml`,
  `Cargo.lock`, `dsp-research/{loudness.md,BIBLIOGRAPHY.md}`, this spec.
- **Hop = 10 ms (rate/100 samples), changed from the scope note's 100 ms.** EBU Tech 3341 cases 13/14 put a 400 ms
  burst at offsets of i x 20 ms and require Max M within 0.1 LU; a 100 ms grid can miss 50 ms of the burst
  (-0.58 LU). Every Tech 3341/3342 recipe duration is a multiple of 10 ms, and on any programme a 10 ms grid misses
  at most 5 ms of a window. M and S therefore update every 10 ms; gating blocks (I) and S-blocks (LRA) keep the
  100 ms step. S2 sizes its record queue for 100 records/s.
- **K-weighting.** Designed at the session rate (44.1/48/88.2/96 kHz only) by bilinear transform with prewarping, in
  `f64`, from analog prototype constants that reproduce the BS.1770-5 48 kHz table to about 1e-15. Stage 2 keeps
  BS.1770's 48 kHz passband gain (+0.0433 dB, numerator exactly 1, -2, 1 at 48 kHz) at every rate, so a signal reads
  the same at every rate; an unnormalized 1, -2, 1 at every rate would spread 0.025 dB across the four rates (44.1 kHz +0.0471 dB to 96 kHz +0.0217 dB).
- **True peak.** The limiter's Annex-2 detector (`true_peak_limiter::annex2`), `f32`, L/R as two lanes of one
  native-width bank, the same 4x table at all four rates.
- **Integrated and LRA.** Bounded 0.1 LU histograms over [-70, +30) LUFS (above full scale: the `f32` master can
  exceed it) with `u64` counts and rounded `f64` energy sums; the gate-straddling bin is the only approximation, bounded by the oracle test (|dI| <= 0.05 LU,
  |dLRA| <= 0.1 LU).
- **Conformance scope.** Recipe-defined cases only: Tech 3341 1-5, 9-23; Tech 3342 1-4. Excluded: Tech 3341 case 6
  (5.0 layout; the engine measures stereo), Tech 3341 7-8 and Tech 3342 5-6 (authentic programme files), the
  BS.2217 file set. These are successors, filed by root.

## Evidence
S1 attempt 1 (implementer; root's verifier has not reviewed it yet).
- **Debug cost.** The production observer runs about 3 us per frame unoptimised, because the Annex-2 FIR uses the generic
  `wide` lane ops, and the suite feeds about 160M frames (13 min 22 s in debug). Root decision: `[profile.dev.package.loudness]
  opt-level = 3` in the root `Cargo.toml` (debug assertions and overflow checks stay on), and the recipe tests check
  the oracle against the expected values at 48 kHz only. The histogram test cross-checks the oracle at 44.1 and 96 kHz
  against production. Debug `cargo test -p loudness` now takes 25 s.
- **Tech 3341, production (the oracle agrees to 4 decimals), LUFS, at 44.1/48/88.2/96 kHz, all within +-0.1 LU:**
  case 1 -22.994/-22.993/-22.989/-22.989; 2 -32.994/-32.993/-32.989/-32.989; 3 and 4 -23.015/-23.014/-23.010/-23.010;
  5 -22.980/-22.979/-22.975/-22.974; 9 (S after 3 s) -22.987/-22.986/-22.982/-22.982; 12 (M after 1 s)
  -22.962/-22.960/-22.956/-22.956; 10 (Max S, 20 files) -22.994/-22.993/-22.989/-22.989; 13 (Max M, 20 files)
  -22.995/-22.994/-22.990/-22.989; 11 and 14 (20 successive maxima, -38 to -19): worst deviation +0.011 LU.
- **Tech 3342 cases 1-4:** LRA 10.000/5.000/20.000/15.000 LU at every rate (tolerance +-1 LU).
- **True peak, Tech 3341 cases 15-23, in dBTP, the same at every rate (tolerance +0.2/-0.4):** -6.021, -5.976, -6.316,
  -6.028, +3.029, -0.130, -0.079, -0.204, -0.079. The largest difference from the f64 Annex-2 reference is 9.6e-8 linear
  (the bound is 1e-6). Case 17 (fs/6 at 60 degrees) is 0.084 dB inside its lower limit; that is how a 4x Annex-2
  detector reads it, and the oracle reads the same value.
- **Histogram against exact gating (150 s of stepped noise):** dI +0.0056 / -0.0113 LU and dLRA -0.0070 / +0.0131 LU at
  44.1 / 96 kHz (bounds 0.05 / 0.1).
- **Defaults.** M and S read -120 LUFS (the floor) for silence. The maxima count only windows whose hops all come after
  the reset. A reset replays the held hops at or after its sample. `gated_block_count` counts the blocks that pass
  both gates.
- **Mutants.** Each of the 15 mutants turned its own gate red (implementer's table, in the hand-back). The true-peak
  limiter's full debug suite passes after the `annex2` visibility change.

## Review (S1 attempt 1)
- Root's verifier passed S1. Root accepts D1 (the 10 ms hop), D2 (the 48 kHz stage-2 gain at every rate), D3 (the
  `loudness` dev-profile opt-level) and D4 (the defaults: a silent window reads -120 LUFS; the maxima count only windows
  wholly after the reset; a reset replays the held hops; `gated_block_count` counts the blocks that pass both gates).
- Folded in: a continuity test (a block that does not continue the last one restarts the observer), a test of the
  production percentile rank rule, the histogram ceiling raised to +30 LUFS, the REALTIME_POLICY region extended over
  `State::rest`, and corrections to the research note (a silent window reads -120 LUFS; subnormal states are flushed at the
  hop end). The S2 note about a non-monotone absolute sample is left for S2.
