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
- **Integrated and LRA.** Bounded 0.1 LU histograms over [-70, +10) LUFS with `u64` counts and exact `f64` energy
  sums; the gate-straddling bin is the only approximation, bounded by the oracle test (|dI| <= 0.05 LU,
  |dLRA| <= 0.1 LU).
- **Conformance scope.** Recipe-defined cases only: Tech 3341 1-5, 9-23; Tech 3342 1-4. Excluded: Tech 3341 case 6
  (5.0 layout; the engine measures stereo), Tech 3341 7-8 and Tech 3342 5-6 (authentic programme files), the
  BS.2217 file set. These are successors, filed by root.

## Evidence
(S1 attempt 1: filled in at hand-back.)
