# Issue #880 MA-2 and MA-5 evidence

## MA-2 — fast-tier evidence and monotonicity

- Exhaustive refitted-candidate F1 sweep: P degree 3 measured `3.210375e-5` dB on `[-160, -0]` and `2.662959e-5` dB on `[0, 24]`; Q degree 4 measured `1.005557e-4` dB on `[1e-8, 16]`. All exceed their F1 gates. Coefficients and the research-harness lower bounds are recorded in `docs/rulings/fast-db-tier-boundaries.md`.
- Exhaustive shipped-body F1 sweep: `fast_level_db` has 77 decreasing adjacent steps (maximum reversal `1.526e-5` dB); `fast_gain_from_db` has 0 on each gain domain. Worker boundary pairs are included in the count.
- Red mutation: `LOG2_Q[0]` word `0x3fb8_a595` → `0x3fb8_a8dc` produced 95 reversals and failed the pinned count of 77. The production coefficient was restored.
- The subtraction claim in `fast_db.rs` now records the measured `1,048,576,000` rounded fractions and their range; no function body or digest pin moved.
- Checks passed: package default math tests, the full ignored F1 release sweep, package clippy, package fmt, and `git diff --check`.

## MA-5 — prospective transient-shaper R2 evidence

The F1 tests restate `FLOOR` as `0x322b_cc77`, contrast limit as 24 dB, shape limit as 18 dB, and parameter IDs 1/2 as `[-1, 1]`, matching `crates/transient-shaper/src/lib.rs`. The unsaturated ratio range from ±24 dB is approximately `[0.0630957, 15.8489]`, inside F1's `[1e-8, 16]` level domain. Every shape corner is clamped inside `[-18, 18]` dB, within F1's `[-160, 24]` gain domain.

The exhaustive release rail sweep covered 841,731,191 nonnegative `f32` values below the floor, including zero and all positive subnormals, and 1,040,187,392 finite `f32` values from 16 through `f32::MAX`. For both exact and fast level functions, all low values clamp to exactly -24 dB and all high values clamp to exactly +24 dB; positive infinity also reaches the high rail.

The four amount-sign combinations were swept over all 257,176,458 ratios in `[1e-8, 16]`. Maximum post-clamp contrast difference was `1.525879e-5` dB. Maximum fast-versus-exact applied-gain difference was `1.654115e-5` dB (ratio bits `0x40f8633d`, attack `+1`, sustain `-1`). Against the independent f64 `20*log10`/`pow(10, dB/20)` pipeline, worst absolute gain error on unit input was `1.287460e-5` for fast and `2.199415e-6` for exact. The fast error fits the existing `2.0e-5` oracle row tolerance with 1.55x margin. The measured end-to-end tier difference is below the derived `5.8e-5` dB bound. These results provide evidence for an R2 owner ruling; they do not approve the crossing.

Focused gates passed: `cargo test --locked -p math --features lane --test f1_fast_db_bounds` (12 passed, 6 ignored; crossing container remains six), the full ignored release F1 suite with `--test-threads=1` (6 passed, including both exhaustive rail ranges and the pipeline sweep), package clippy, package fmt, and `git diff --check`. The exhaustive MA-5 sweeps are prospective and ignored by default; the domain/container checks remain active. No pin moved.
