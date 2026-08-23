# Red-mutation record for the multiband-compressor gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test was run, the failure was
recorded, and the mutation was reverted in the same session.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked --release -p miso-engine-multiband-compressor <test>
# and revert
```

| # | mutation | file | test | result |
|---|---|---|---|---|
| 1 | high band sign flipped: `high = low - ap` instead of `ap - low` | `tests/lr4_two_section_mapping_f64.rs` | `lr4_two_section_mapping_f64` | RED |
| 2 | all-pass tap halved: `ap = x - k*v1` instead of `x - 2k*v1` (`nk2 = -K`) | `tests/lr4_two_section_mapping_f64.rs` | `lr4_two_section_mapping_f64` | RED |

## Recorded failures

### 1 — high band sign flipped

```
thread 'two_section_bands_match_the_four_section_reference' panicked at
  crates/miso-engine-multiband-compressor/tests/lr4_two_section_mapping_f64.rs:189:5:
high band deviates by 3.6359248112641387e0
thread 'the_band_sum_is_the_butterworth_allpass' panicked at
  crates/miso-engine-multiband-compressor/tests/lr4_two_section_mapping_f64.rs:227:17:
rate=44100 crossover=80 probe=20 flatness=-6.785610296840339e-2 dB
```

The sum becomes `2*low - ap` — flat neither in magnitude nor in phase, and exactly zero at the
crossover where `2*LP4 = AP`. This is the mutation the one-sixth-octave sweep exists for: a sign
error here leaves DC and Nyquist untouched.

### 2 — all-pass tap halved

```
thread 'two_section_bands_match_the_four_section_reference' panicked at
  crates/miso-engine-multiband-compressor/tests/lr4_two_section_mapping_f64.rs:189:5:
high band deviates by 1.1839204465966064e0
thread 'the_band_sum_is_the_butterworth_allpass' panicked at
  crates/miso-engine-multiband-compressor/tests/lr4_two_section_mapping_f64.rs:227:17:
rate=44100 crossover=80 probe=20 flatness=-5.774936116386891e-1 dB
```

`x - k*v1` is the notch mix, not the all-pass mix; the sum stops being unit-magnitude.
