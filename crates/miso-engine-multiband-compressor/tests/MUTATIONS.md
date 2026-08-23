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

---

## Wave 2

Same procedure: apply, run, record, revert. Every row below was executed once in this session.

| # | mutation | file | test | result |
|---|---|---|---|---|
| 3 | high band sign flipped in production: `high = low - ap` | `src/lib.rs` (`lr4_step`) | `crossover` | RED |
| 4 | all-pass tap halved: `NEGATIVE_TWO_K = -sqrt(2)` instead of `-2*sqrt(2)` | `src/lib.rs` | `crossover` | RED |
| 5 | `a2` and `a3` swapped in `design_lr4` | `src/lib.rs` | `crossover` | RED |
| 6 | `segment_length` drops the D11 `- 1`, so a ramp snaps inside a vectorised run | `src/lib.rs` | `identity` | RED |
| 7 | `detector_tap` gains a `if W == 1` shortcut that skips the wrap | `src/lib.rs` | `identity` | RED |
| 8 | the state payload writes both rings in raw slot order instead of oldest-first | `src/lib.rs` | `identity` | RED |
| 9 | the unity-gain path returns half the band sum (a stand-in for the deleted F1 identity branch) | `src/lib.rs` | `product` | RED |
| 10 | the dB path's gain is perturbed by one part in `1e7` | `src/lib.rs` | `cross_target_digest` | RED |
| 11 | `discontinuity_reset` reallocates its rings instead of filling them (the version-1 F9 behaviour) | `src/lib.rs` | `no_alloc_render` | RED |
| 12 | the once-per-block boundary check's verdict is ignored | `src/lib.rs` | `nonfinite` | RED |
| 13 | the restore drops `LinearRamp`'s invariant, so a resting ramp may carry a live step | `src/lib.rs` | `product` | RED |
| 14 | `branching_smooth` reverses the direction: `select(target.gt(y), ..)` | `src/shim.rs` | `--lib` | RED |
| 15 | `branching_smooth` rounds three times: `c * y + (1 - c) * target` | `src/shim.rs` | `--lib` | RED |
| 16 | `link_levels` maximum becomes minimum | `src/shim.rs` | `--lib` | RED |
| 17 | `link_levels` drops the `abs` | `src/shim.rs` | `--lib` | RED |
| 18 | `link_levels` average halves the sum: `0.5 * (l + r)` | `src/shim.rs` | `--lib` | RED |

## Recorded failures, wave 2

### 3 — high band sign flipped in production

```
thread 'the_two_stage_split_matches_the_reference_and_recombines_flat' panicked at
  crates/miso-engine-multiband-compressor/tests/crossover.rs:65:17
```

### 4 — all-pass tap halved

```
thread 'the_design_guards_its_domain' panicked at .../tests/crossover.rs:131:13:
assertion `left == right` failed
thread 'the_two_stage_split_matches_the_reference_and_recombines_flat' panicked at
  crates/miso-engine-multiband-compressor/tests/crossover.rs:65:17
```

### 5 — `a2` and `a3` swapped in the design

All three `crossover` tests fail; the design's own half-power self-check rejects the triple, so
`lr4_coefficients` returns `None`.

### 6 — `segment_length` drops the D11 `- 1`

```
thread 'partition_invariance' panicked at .../tests/identity.rs:268:13:
assertion `left == right` failed: partition=1 frame=960
```

Frame 960 is the first sample the ramp's history can reach the output at, which is exactly the
latency: the snap landed one sample early in the one-frame partition and nowhere else.

### 7 — `detector_tap` gains a width shortcut

```
thread 'lane_identity_across_widths' panicked at .../tests/identity.rs:178:21:
assertion `left == right` failed: link=DualMono width=Four channel=2 frame=960
thread 'partition_invariance' panicked at .../tests/identity.rs:268:13:
assertion `left == right` failed: partition=1 frame=960
```

### 8 — the rings are written in raw slot order

```
thread 'a_restored_track_is_rotated_into_the_receiving_cursor' panicked at .../tests/identity.rs:443:9:
assertion `left == right` failed: frame 260: the restored track must continue the donor's history
```

### 9 — the unity-gain path returns half the band sum

```
thread 'unity_gain_output_is_the_delayed_lr4_sum' panicked at .../tests/product.rs:181:9
thread 'unity_gain_transition_has_no_step_at_crossover' panicked at .../tests/product.rs:128:9
thread 'isolated_low_and_high_band_compression_reduce_only_the_selected_band' panicked at
  .../tests/product.rs:349:9
```

A stand-in for the deleted F1 branch: any output selection that depends on the gain being exactly
unity is a discontinuity, and all three product gates see it.

### 10 — the dB path's gain is perturbed by one part in `1e7`

```
thread 'the_corpus_digests_are_pinned_and_width_independent' panicked at
  .../tests/cross_target_digest.rs:65:13:
assertion `left == right` failed: band_amplitude moved: re-pin only from an oracle, never from a run
```

### 11 — `discontinuity_reset` reallocates its rings

```
thread 'the_scalar_render_path_allocates_nothing' panicked at .../tests/no_alloc_render.rs:121:5:
assertion `left == right` failed: the scalar render path allocated 16 times
thread 'the_bank_render_path_allocates_nothing' panicked at .../tests/no_alloc_render.rs:186:9:
assertion `left == right` failed: Four allocated 16 times
```

### 12 — the boundary check's verdict is ignored

```
thread 'a_nonfinite_block_is_zeroed_reset_and_counted' panicked at .../tests/nonfinite.rs:76:9:
assertion `left == right` failed: NaN: expected exactly one rejected block, got [0, ...]
```

### 13 — the restore drops `LinearRamp`'s invariant

```
thread 'a_rejected_restore_changes_nothing' panicked at .../tests/product.rs:290:14
```

A payload that says `remaining == 0` while carrying a non-zero `step` would have the segment
driver add that step to a resting parameter on every frame for ever. The invariant is
`LinearRamp`'s; the restore enforces it rather than assuming a well-formed writer.

### 14 to 18 — the `src/shim.rs` pieces

Wave-2 decision W2-D3 on #83 gives the stereo-coupled dynamics scaffolding to #88, so these two
functions live here as shims rather than in `effect-runtime`. Their gates and their red mutations
come with them.

```
thread 'shim::tests::the_smoother_rounds_once_and_picks_the_attack_direction' panicked at
  crates/miso-engine-multiband-compressor/src/shim.rs:120:13                              (14)
thread 'shim::tests::the_smoother_rounds_once_and_picks_the_attack_direction' panicked at
  crates/miso-engine-multiband-compressor/src/shim.rs:119:13                              (15)
thread 'shim::tests::the_link_combines_the_pair_per_mode' panicked at .../src/shim.rs:174:13  (16)
thread 'shim::tests::the_link_combines_the_pair_per_mode' panicked at .../src/shim.rs:164:13  (17)
thread 'shim::tests::the_link_combines_the_pair_per_mode' panicked at .../src/shim.rs:191:9   (18)
```

14 is the direction of the smoother's coefficient select: more gain reduction is a *lower* dB
value, so the attack coefficient belongs to `target < y`. 83c found the gate crate using the
opposite compare under the opposite sign convention — correct in both, and exactly what a shared
helper must not paper over.

## Equivalent mutants, recorded rather than hidden

* `link_levels`' maximum written as `select(left.ge(right), left, right)` instead of the D8
  `select(left.gt(right), left, right)` **survives**, and provably must: the two differ only on
  equal lanes, where they choose between `+0.0` and `-0.0`, and both operands have already been
  through `abs`, so `-0.0` cannot reach the compare. The strict form is kept anyway because it is
  the trait's `max` and because a caller that skipped the `abs` would then be wrong for one reason
  instead of two.
