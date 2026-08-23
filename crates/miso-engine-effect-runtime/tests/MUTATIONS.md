# Red-mutation record for the effect-runtime gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing in this file is a claim about
code that was not run.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p miso-engine-effect-runtime --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | ramp snap off by one: `ramp_frames = min(remaining, frames)` instead of `min(remaining - 1, frames)` | `src/ramp.rs` | `ramp` | RED |
| 2 | the ramp divides per sample again: `current += (target - current) / remaining as f32` | `src/ramp.rs` | `ramp` | RED |
| 3 | envelope coefficient sign: `expf(1 / tau)` instead of `expf(-1 / tau)` | `src/envelope.rs` | `envelope` | RED |
| 4 | `peak_follow` rounds twice: `c * y + (1 - c) * x` instead of `fma(c, y - x, x)` | `src/envelope.rs` | `envelope` | RED |
| 5 | hysteresis drops the hold: `close = below` instead of `below AND expired` | `src/envelope.rs` | `envelope` | RED |
| 6 | knee-width halving dropped: `half_knee_db = knee_db` instead of `0.5 * knee_db` | `src/dynamics.rs` | `dynamics` | RED |
| 7 | the `knee_db > 0.0` guard dropped, so a non-positive knee is designed literally | `src/dynamics.rs` | `dynamics` | RED |
| 8 | boundary-check threshold raised: `BLOCK_LIMIT = 1e33` instead of `1e30` | `src/bank.rs` | `bank` | RED |
| 9 | boundary check loses its NaN case: `mask_not(abs.lt(limit))` becomes `abs.ge(limit)` | `src/bank.rs` | `bank` | RED |
| 10 | state payload byte order: `to_be_bytes` instead of `to_le_bytes` | `src/state_payload.rs` | `state_payload` | RED |
| 11 | state payload length check accepts trailing bytes: `<` instead of `!=` | `src/state_payload.rs` | `state_payload` | RED |
| 12 | parameter domain becomes exclusive: `>` / `<` instead of `>=` / `<=` | `src/params.rs` | `params` | RED |
| 13 | the corpus loop becomes width-dependent: `point(index + offset % 2)` | `src/corpus.rs` | `lane_identity` | RED |
| 15 | the lower knee edge is compared against `+W/2`: `d.le(half_knee_db)` instead of `d.le(-(W/2))` | `src/dynamics.rs` | `dynamics` | RED |
| 16 | `ar_one_pole_step` switches on `u.ge(e)` instead of `u.gt(e)` | `src/envelope.rs` | `envelope` | RED |
| 17 | `ar_one_pole_step` uses the one-rounding release `c.fma(e - u, u)` instead of the two-product `c * e + k * u` | `src/envelope.rs` | `envelope` | RED |
| 18 | `ar_one_pole_step` drops `miso_engine_lane::flush` on the recurrence | `src/envelope.rs` | `envelope` | RED |

## Recorded failures

### 1 — ramp snap off by one

`advance_block` hands the kernel one stepping frame too many, so the sample that should be an exact
assignment of the target is `current + step` instead.

```
test block_driving_matches_the_scalar_sequence ... FAILED
assertion `left == right` failed: scalar, target 0.1, 7 samples, blocks of 7: sample 6: 0.09999999 vs 0.1
```

### 2 — the ramp divides per sample again

This is the D11 law itself. The discriminating value is the second sample of a three-sample ramp
from `0.0` to `1.0`: the precomputed step gives `1/3 + 1/3`, re-deriving gives `0.5`.

```
test the_step_is_computed_once ... FAILED
test block_driving_matches_the_scalar_sequence ... FAILED
```

### 3 — envelope coefficient sign

```
test coefficients_are_in_range_and_correctly_signed ... FAILED
0.1 ms at 44100 Hz: retention 1 left [0, 1)
```

The clamp saturates the coefficient at `1.0`, which the range assertion catches; without the clamp
the coefficient would be `e^{+1/tau}` and every follower built on it would diverge.

### 4 — `peak_follow` rounds twice

```
test peak_follow_rounds_once ... FAILED
assertion `left == right` failed: step 4334: 0.9999921 vs 0.99999213
```

One ulp, on step 4334 of 20000 — which is exactly why this is asserted by bits against an `f64`
single-rounding evaluation rather than by a tolerance.

### 5 — hysteresis drops the hold

```
test the_hold_delays_closing ... FAILED
assertion `left == right` failed: the gate must stay open for the hold
  left: 0
 right: 3
```

### 6 — knee-width halving dropped

```
test below_the_knee_is_the_exact_identity ... FAILED
test the_knee_is_continuous_at_both_edges ... FAILED
test the_curve_matches_the_paper ... FAILED
assertion `left == right` failed: -21.0001 dB
```

A 6 dB knee spans 12 dB, so a level 3 dB below the threshold is inside the knee instead of below it
and the "exact identity below the knee" property goes first.

### 7 — the non-positive knee guard dropped

```
test a_non_positive_knee_is_a_hard_knee ... FAILED
assertion `left == right` failed: T 0 R 1.5 W -1 at 0.5 dB
```

A negative width gives a negative `half_knee_db`, the `under` arm swallows the first `|W|/2` dB
above the threshold, and a signal that should be compressed is passed through.

### 8 — boundary-check threshold raised

`1e33` is chosen because `1e40` does not fit an `f32` and is rejected by `overflowing_literals` at
compile time — a real red, but a compile error rather than a behavioural one.

```
test the_lane_mask_names_the_failing_lanes ... FAILED
test the_check_rejects_at_the_threshold ... FAILED
```

### 9 — boundary check loses its NaN case

An ordered compare against NaN is false in both directions, so `abs >= limit` accepts every NaN.

```
test a_rejected_block_is_zeroed_reset_and_counted ... FAILED
test the_counter_counts_blocks ... FAILED
test the_check_rejects_at_the_threshold ... FAILED
assertion failed: !finish_block::<Simd4>(&mut left, &mut right, &mut report, ...)
```

### 10 — state payload byte order

```
test words_are_little_endian ... FAILED
test float_words_round_trip_by_bits ... FAILED
test snapshot_then_restore_returns_every_word ... FAILED
test wrong_word_count_is_a_length_error ... FAILED
```

The header words are read back through `read_u32`, so a one-sided byte-order change also breaks the
version and word-count checks — which is the shape a half-migrated codec would have.

### 11 — length check accepts trailing bytes

```
test wrong_section_lengths_are_rejected ... FAILED
common section length must be exact: ()
```

### 12 — parameter domain becomes exclusive

```
test continuous_domains_are_inclusive_and_reject_non_finite ... FAILED
test clamping_always_lands_in_the_domain ... FAILED
test defaults_are_clamped_and_counted ... FAILED
assertion failed: parameter_value_valid(&THRESHOLD, -80.0)
```

A parameter can be set to the end of its own range; clamping to that end and then rejecting it is
the bug this catches.

### 13 — the corpus loop becomes width-dependent

The lane-identity gate has to be able to see a body whose result depends on `L::WIDTH`. Introducing
one in the corpus driver proves the gate is not vacuous.

```
test the_corpus_is_width_independent ... FAILED
gain_delta_db_hard_knee point 2: W=1 0x00000000 vs W=4 0xc1580000
```

### 15 — the lower knee edge compared against `+W/2`

```
test the_knee_is_continuous_at_both_edges ... FAILED
test the_curve_matches_the_paper ... FAILED
T 0 R 1.5 W 1: upper edge 0.5 vs line 0.3333333333333333
```

### Added by issue #88 (`bank::finish_channel`, verifier decision W2-D3)

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 19 | `finish_channel` returns `0` after a rejected block instead of the failing lane mask | `src/bank.rs` | `bank` | RED |
| 20 | `finish_channel` resets the channel but does not zero the rejected block | `src/bank.rs` | `bank` | RED |

## Equivalent mutations (recorded, deliberately not gated)

| # | mutation | why it survives |
|---|---|---|
| 14 | `over = d.ge(half_knee_db)` instead of `d.gt(half_knee_db)` | At `d == W/2` the knee arm and the line arm are the **same `f32` value**, not merely equal analytically: the knee gives `W * W * (1/(2W)) * (1/R - 1)` and the line gives `(W/2) * (1/R - 1)`, and for the corpus's coefficients (`T = -18`, `W = 6`, `R = 4`, upper edge `-15` dB, both `-2.25`) the two round to identical bits. The `determinism` digest — whose edge pool contains `-15.0` precisely so that this point is covered — is also green. The strict form is kept because it is what makes a **hard** knee (`W = 0`) give exactly `x` at `x = T` as the paper requires, and that case is gated by `a_hard_knee_is_exact_at_the_threshold`. |

## Gates in this crate

| binary | proves |
|---|---|
| `state_payload` | codec round trip, exact length rejection, version rejection, little-endian byte order |
| `ramp` | D11: one division, iterated additions, exact snap, block driving matches the scalar sequence at {1, 7, 64, 128, 512} |
| `envelope` | coefficient design against an `f64` oracle, both followers exact against a single-rounding `f64` evaluation, D8 NaN/signed-zero behaviour, hysteresis open/hold/close |
| `dynamics` | Giannoulis, Massberg and Reiss equation 4 to better than 0.01 dB over the frozen grid; knee continuity at both edges; hard and non-positive knees |
| `bank` | master plan §4.4: threshold, NaN, both channels, zeroing, reset, the block counter, the failing-lane mask, and an identity slot that is exact |
| `params` | domain validation for all three kinds, clamping into the domain, mapping exactness and monotonicity, defaults |
| `lane_identity` | every lane-generic function agrees at `W = 1`, 4 and 8 by `to_bits` |
| `partition` | P1: a ramp, a follower and the gain computer composed, rendered in blocks of {1, 7, 64, 128, 512}, bit-identical output and state |
| `determinism` | D1: pinned SHA-256 digests over the nine-case corpus, at all three widths, for 83d's wasm leg |


### 16, 17, 18 — the attack/release one-pole step (added by the #92 job)

`ar_one_pole_step` was added to `envelope` from the #92 (transient shaper) job, which needs a
switched two-coefficient one-pole that `peak_follow` is not. Three mutations, all run on
`cargo test --locked -p miso-engine-effect-runtime --test envelope`:

* **16** — `u.ge(e)`: `ar_one_pole_step_switches_strictly_on_rising` FAILED. The equal case is a
  fixed point in exact arithmetic, so the test pins a witness (`e = u = 0x3c1b4ffb` at the
  0.5 ms / 20 ms 44.1 kHz pair) where the two rounded products sum differently under the two
  coefficients, and asserts that the two do differ before asserting which one is taken. Without the
  witness this mutation is equivalent, which is why the assertion pair is there.
* **17** — the one-rounding form: `ar_one_pole_step_is_the_two_product_form` FAILED. The witness
  `e = 0.7`, `u = e - 5.7220459e-5` at the 100 ms / 96 kHz coefficient is inside the one-rounding
  form's deadband, so the mutated step returns `e` unchanged.
* **18** — no flush: `ar_one_pole_step_flushes_the_recurrence` FAILED, the envelope word decaying
  into the subnormal range instead of reaching `+0.0`.
