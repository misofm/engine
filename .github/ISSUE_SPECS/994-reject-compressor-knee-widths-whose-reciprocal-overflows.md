# Reject compressor knee widths whose reciprocal overflows

## Product outcome

A compressor knee of 2.8e-45 passes parameter validation (`crates/compressor/src/params.rs:128`), `1/(2W)` then overflows to +inf (`dynamics.rs:69`), and a sample exactly at threshold computes NaN (`dynamics.rs:120`). Today the gain-reduction clamp in the kernel (`kernel.rs:353-355`, `max(-100)`) turns that NaN into a -100 dB target, so the defect is masked rather than absent: the output is a hard -100 dB duck on that sample, which is wrong. Found by the compressor optimisation verification.

## Smallest closable slice

Reject (or snap to zero, if a zero knee is the documented hard-knee case) any knee width below the smallest value whose `1/(2W)` is finite and whose knee polynomial stays finite over the admissible level range, with a typed parameter diagnostic, at every entry that sets the parameter (session, control, automation). State the bound and derive it.

## Objective gates

- A test at the old failing knee shows the diagnostic (or the snap) and no NaN at threshold.
- A randomized sweep over admissible knees and levels produces no NaN or infinity in the gain computer.
- Every console digest unchanged.

## Attempt 1 evidence

Implementer: Claude (Opus 5.5), branch `codex/994-reject-overflowing-knee` from
`codex/batch-plumbing-floor-2` at `8eebf17b`. Fix commit `bb7fbb82`.

### Chosen fix: (a), the hard knee at design time

`effect_runtime::dynamics::knee_coefficients(knee_db) -> (W/2, 1/(2W))` is a new `const fn`, and
`GainComputerCoef::new` now takes its knee words from it. It returns the hard knee `(+0.0, +0.0)`
for `knee_db <= 0` and NaN (as before) **and for a positive width whose computed `1 / (2 W)` is not
finite**. Every other width returns `(0.5 * W, 1.0 / (2.0 * W))`, the pre-#994 expression.

Why not (b), a raised parameter minimum with a typed diagnostic:

* At the bound this issue derives, it cannot close the defect. The compressor redesigns the curve
  from every sample of a `Linear 64` knee ramp (`kernel.rs` `advance_ramps` -> `design_lane` ->
  `GainComputerCoef::new`). A ramp from 0 to `1e-38` has two endpoints that a minimum of
  `2^-129 + 2^-149` admits. Its step is `111504 * 2^-149`, so its first nine values (and the last
  nine of the ramp back, k = 55..63) lie in the overflow band. The test
  `an_automated_knee_ramp_through_the_overflow_band_does_not_duck` is red on the pre-#994 design.
* A much coarser minimum would keep ramps out of the band, since a ramp from 0 to `a` never comes
  closer to 0 than about `a / 64`. That needs `a > 64 * 2^-129`, for example the 0.1 dB lattice
  step. But it is a product-domain change, not the derived bound, and its safety would rest on the
  ramp's geometry rather than on the design.
* The minimum cannot simply be raised: 0 is the documented hard knee and must stay admissible, so
  (b) would need a split domain `{0} U [bound, 24]`, a new domain kind in the contract, the
  session, the control plane and automation.
* (a) sits at the one function every entry reaches (prepare, automation ramps, restore, both
  resets), so no entry point can bypass it.

The stored parameter value is not changed: a knee of 2.8e-45 still reads back and snapshots as
2.8e-45. Only its design is the hard knee.

### The bound, derived

* `2 W` is exact for every finite `W <= 24`: doubling does not round, subnormals included.
* Under round-to-nearest-even, `1 / (2 W)` is `+inf` exactly when the true quotient is at least
  `f32::MAX + ulp/2 = 2^128 - 2^103`. The tie rounds to the even significand, which is the
  overflow.
* So the reciprocal overflows exactly when `2 W <= 2^-128 / (1 - 2^-25)`. That value lies strictly
  between `2^-128` and the next `f32`, `2^-128 + 2^-149`. For an `f32` the condition is therefore
  `2 W <= 2^-128`, that is `W <= 2^-129`.
* The overflowing widths are exactly `(0, 2^-129]`, bits `0x0000_0001..=0x0010_0000`. The
  narrowest soft knee is `2^-129 + 2^-149` = `f32::from_bits(0x0010_0001)`, about `1.469e-39`,
  exported as `MIN_SOFT_KNEE_DB`.
* The code tests the computed reciprocal rather than comparing with the constant. Under IEEE
  arithmetic the two are the same predicate; `the_overflow_bound_is_exact_and_only_it_moves` checks
  that exhaustively. The reciprocal test also stays right under FTZ, where `2 W` could flush to
  zero; a constant compare would then design an infinite reciprocal. Render sessions run with
  FTZ/DAZ clear in any case (`host-core` `render_session`).

### Exactness for every finite-safe knee (class A)

* For every width whose reciprocal is finite, `knee_coefficients` takes the same branch and
  evaluates the same two expressions as before. The words are therefore identical.
* Every downstream consumer (`design_lane`'s four curve words, `gain_delta_db`) is a pure function
  of those words, so every output bit is unchanged.
* The test compares the new words with a transcription of the old design:
  * exhaustively, for all 8,388,609 bit patterns from `+0.0` to `f32::MIN_POSITIVE`;
  * for `-0`, `-1e-45`, `-1`, `-24`, NaN and 24;
  * for one million random bit patterns over `[0, 24]`.
* The words differ only for the widths in `(0, 2^-129]`.
* The console digests, the compressor, multiband and effect-runtime corpus digest pins, and the
  compressor's settled-body differentials are all green.

The knee arm is finite whenever the reciprocal is. `gain_delta_db` selects the knee arm only for
`-(W/2) < d <= W/2`, where `0 < v = d + W/2 <= 2 (W/2)`:

* `(v * v) * (1/(2W))` is at most about `W/2`.
* Subnormal rounding of `v * v` adds at most `2^-150 / (2W) < 2^-22`.
* The result is scaled by `1/R - 1`, which lies in `[-0.95, 0]` on the ratio domain.

The bound therefore depends on no level at all.

### The pathological widths now

The hard knee is the limit of the soft curve as `W -> 0`. It differs from the paper's curve by at
most `|1/R - 1| W / 8`, reached at `x = T`, which is below `2^-132` dB for `W <= 2^-129`. The
result is finite, is never NaN, and matches the independent `f64` reference (which designs these
widths as genuine soft knees) to 1e-6 in every rendered sample. A full-scale sample at a 0 dB
threshold is no longer ducked; before, it was ducked 18.8 dB on the first sample with a 0.1 ms
attack.

### Each user of the curve

| user | what the fix means there |
|---|---|
| `compressor` | No source change. Its knee parameter (`[0, 24]`) reaches `GainComputerCoef::new` through `design_lane` at prepare, reset, restore and every knee-ramp sample, so every entry is covered. The reduction clamp at `kernel.rs:353-355` is untouched. |
| `multiband-compressor` | The knee is the product constant `KNEE_DB = 6.0`: no session, control or automation input reaches an overflowing width. Its knee words now come from `knee_coefficients(KNEE_DB)` at compile time (`BAND_KNEE`), bit-identical to the old inline `0.5 * KNEE_DB` and `1.0 / (2.0 * KNEE_DB)` (`the_fixed_knee_words_are_unchanged`). Step 2 of `band_amplitude` is factored into `#[inline(always)] band_target`, which takes the knee words as an argument so the rule can be tested; the render path passes the constant. |
| `effect-runtime` corpus / `dsp-reference` | The corpus uses knees 0, 6 and 12 (unchanged, pins green). The `f64` reference needs no change: `1/(2W)` is finite in `f64` for every `f32` width. |

### Gates

Each gate ran with `CARGO_INCREMENTAL=0` in the worktree's own `target/`, debug profile, x86_64 (AVX2 + FMA pin).

| gate | command | result |
|---|---|---|
| old failing knee and bound, compressor | `cargo test --locked -p compressor --test knee_overflow` | 5/5 pass. The knees 1.4e-45, 2.8e-45, `0x000F_FFFF` and `2^-129` go through prepare, automation, restore and an 8-lane bank. Each renders bit-identical to knee 0 and within 1e-6 of the `f64` reference. The narrowest soft knees (`0x0010_0001`, `0x0010_0002`, `1e-38`) do the same. |
| old failing knee and bound, multiband | `cargo test --locked -p multiband-compressor --lib knee_tests` | 2/2 pass. Six widths around the bound are tested, with thresholds `{0, -0, -1e-45, -18, -80}` and ratios `{1, 1.5, 4, 20}`. Levels are the threshold itself and points a few subnormal steps and ulps either side of it. Every target is finite, `> -1e-3`, and the same at W1/W4/W8; each overflowing width gives the hard-knee target bit for bit. |
| shared curve at the bound | `cargo test --locked -p effect-runtime --test dynamics` | 13/13 pass, including `the_overflow_bound_is_exact_and_only_it_moves`, `a_knee_whose_reciprocal_overflows_is_a_hard_knee` and `the_narrowest_soft_knees_are_finite_at_the_threshold` |
| randomized sweep | `a_randomized_sweep_never_leaves_the_finite_curve` (same binary) | 100,000 cases and 3,200,000 levels. Knees are drawn by bit pattern over `[0, 24]` and over the neighbourhood of the bound, and uniformly. Thresholds are drawn over `[-80, 0]`, including `0`, `-0` and `-1e-45`, and ratios over `[1, 20]`. Levels are drawn over `[-800, 30]` dB, plus the silent floor, the threshold, its subnormal and ulp neighbours, and the knee edges. There is no NaN or infinity in `gain_delta_db` or `gain_computer_db`, and W1, W4 and W8 are bit-identical. The worst deviation from the `f64` eq. 4 is 8.723e-6 dB, against a 1e-4 gate. |
| console digests | `cargo test --locked -p console-workload --test gain_pan_profile -- --ignored --nocapture --exact digests`, on the base (`git stash -u`) and on the fix | all 17 rows identical |
| formatting | `cargo fmt --all --check` | clean |
| lint | `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | clean |
| docs | `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps` | clean |
| crate tests | `cargo test --locked -p effect-runtime -p compressor -p multiband-compressor -p math` | 273 passed, 0 failed |
| console | `cargo test --locked -p console-workload` | 39 passed, 0 failed, 2 ignored (the `digests` and `phase_profile` harnesses, run above) |
| builtins | `cargo test --locked -p builtins-compiler --features test-support` | 79 passed, 0 failed |
| policy | `scripts/check-lane-policy.sh`, `scripts/check-realtime-policy.sh`, `scripts/check-effect-runtime-policy.sh` | ok (57 marked realtime regions in 16 files) |
| wasm | `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p effect-runtime -p compressor -p multiband-compressor` | clean |

The timed benchmark was not run, as the brief required. The only new render-path work is one float
compare in `knee_coefficients`, paid only while a curve parameter ramps. The multiband's words are
compile-time constants, as the old inline expressions already were after constant folding.

### Red mutations

These are recorded in `crates/{effect-runtime,compressor,multiband-compressor}/tests/MUTATIONS.md`.
Each was applied, run, and then restored byte for byte from a saved copy.

* **994-R1, 994-C1, 994-M1:** drop the `is_finite` test, which is the pre-#994 design. Red in
  every #994 test that uses an overflowing width. The whole of the three crates was run with
  `--no-fail-fast`: 229 passed, and exactly the 8 new tests failed. No gate that existed before
  #994 saw the defect.
* **994-R2:** a constant bound one ulp high. Red in 2 tests.
* **994-R3, 994-C2, 994-M3:** a constant bound one ulp low. Red.
* **994-M2:** `BAND_KNEE` from a doubled knee. Red.

### Deviations and notes for the verifier

* The spec's slice says "reject ... with a typed parameter diagnostic, at every entry". This
  attempt takes the spec's other permitted path: snap to the documented hard knee. It does so at
  design time rather than on the stored value, for the ramp reason above. No diagnostic is added.
  Validation, session, control and automation are untouched.
* The multiband compressor has no knee parameter, so its "test at the old failing knee" exercises
  its own `band_target` with `knee_coefficients(2.8e-45)` and the widths around the bound. It does
  not test through a session.
* `0x0000_0001` alone is not a witness: `0.5 * 2^-149` rounds to `+0.0`, so the knee interval is
  empty even without the rule. The lists also carry `0x0000_0002`, `0x000F_FFFF` and `2^-129`.
* The existing `half_knee_db` field doc ("Exact: halving does not round") is not strictly true for
  subnormal widths, as the point above shows. It is harmless: after this change the only subnormal
  soft knees are `(2^-129, 2^-126)`, where `v * v` underflows to zero and the knee arm is `+-0`. It
  is left unchanged as outside this issue.
