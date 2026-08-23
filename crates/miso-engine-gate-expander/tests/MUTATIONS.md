# Red-mutation record for the gate/expander gates (#89)

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing in this file is a claim about
code that was not run.

Host: `x86_64` (Zen 5 class), workspace `.cargo/config.toml` pin `-C target-feature=+avx2,+fma`,
debug profile unless a row says otherwise.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p <crate> --test <test binary>
# and revert
```

| # | mutation | file | crate / binary | result |
|---|---|---|---|---|
| 1 | the hold expiry is tested *after* the decrement instead of before it | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |
| 2 | the opening threshold becomes strict: `level_db > threshold` instead of `>=` | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |
| 3 | the in-band re-arm stops reloading the hold | `dsp-reference/src/gate_expander.rs` | `miso-engine-dsp-reference` / `--lib` | RED |

## Recorded failures

### 1 — the hold expiry is tested after the decrement

```
} else {
    slot.hold_remaining = slot.hold_remaining.saturating_sub(1);
    if slot.hold_remaining == 0 { slot.open = false; }
}
```

closes the gate one sample early, so a hold of `n` keeps it open for `n - 1` samples.

```
test gate_expander::tests::a_level_inside_the_hysteresis_band_keeps_reloading_the_hold ... FAILED
test gate_expander::tests::hold_expiry_closes_exactly_one_sample_after_the_countdown_reaches_zero ... FAILED
test result: FAILED. 5 passed; 2 failed
```

### 2 — the opening threshold becomes strict

`level_db > threshold` leaves a closed gate closed on the one sample whose level is exactly the
threshold, which brief 014 opens on.

```
test gate_expander::tests::a_level_exactly_at_the_threshold_opens_a_closed_gate ... FAILED
test result: FAILED. 6 passed; 1 failed
```

### 3 — the in-band re-arm stops reloading the hold

With the reload removed, a gate whose countdown is already partly spent when the level returns to
the hysteresis band closes from what was left of it instead of from a full hold.

```
test gate_expander::tests::a_level_inside_the_hysteresis_band_keeps_reloading_the_hold ... FAILED
test result: FAILED. 6 passed; 1 failed
```

Note on an *equivalent* mutant found while writing row 3: with a test whose hold is full when the
band is entered, removing the reload changes nothing, because the in-band arm is an `else if` and
short-circuits the decrement. The test was rewritten to spend part of the countdown before the band
is entered rather than accepting the mutant as covered.

## Production gates (#89 §7)

Reproduce with `cargo test --locked -p miso-engine-gate-expander --test <binary>` unless a row
says otherwise. Every row below was applied, run, recorded and reverted in one session.

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 4 | the detector gather takes the partner sample from the neighbouring lane | `src/kernel.rs` | `identity` | RED |
| 5 | the block does not advance the shared ring cursor | `src/kernel.rs` | `identity` | RED |
| 6 | `DB_PER_OCTAVE` perturbed from `6.0206` to `6.03` | `src/kernel.rs` | `oracle` | RED |
| 7 | the dry path is flushed before the identity select | `src/kernel.rs` | `identity` | RED |
| 8 | the ring is snapshotted in physical slot order, not cursor-normalised | `src/lib.rs` | `state` | RED |
| 9 | the boundary check scans only the gain words, not the output block | `src/lib.rs` | `state` | RED |
| 10 | the boundary check scans only the output block, not the gain words | `src/lib.rs` | `--lib` | RED |
| 11 | the detector tap is one sample early (`N - L - 1`) | `src/lib.rs` | `contract` | RED |
| 12 | the one-pole rounds twice instead of fusing (D3) | `src/kernel.rs` | `determinism` | RED |
| 13 | the D7 `flush` is removed from the one recursive word | `src/kernel.rs` | `state` | RED |
| 14 | the render path allocates one `Vec` per block | `src/lib.rs` | audit bin | RED |

### 4 — the detector gather takes the partner from the neighbouring lane

`taps[1][lane] = source_right[(own + 1) % len]`. Scalar is unaffected (`WIDTH = 1` wraps onto
itself), so only the bank moves — which is the point: it is the one place the lane-generic body
touches a lane index.

```
test lane_identity_scalar_w8 ... FAILED
assertion `left == right` failed: Maximum left track 0 sample 480
```

### 5 — the block does not advance the shared ring cursor

Removing `*cursor = base.wrapping_add(frames)` makes every block start writing where the previous
one did, which is invisible at one block size and fatal at another.

```
test partition_invariance ... FAILED
assertion `left == right` failed: scalar track 0 left at partition 1 sample 64
```

### 6 — `DB_PER_OCTAVE` perturbed to 6.03

A 0.16 % error in the dB conversion, about 0.08 dB at the corpus's quiet level, four times the
derived bound.

```
test oracle_pcm_within_derived_tolerance_scalar ... FAILED
test oracle_pcm_within_derived_tolerance_w8 ... FAILED
```

Note on an *equivalent* mutant found while writing this row: with the corpus's quiet level at
-60 dBFS the mutation survived, because `(rho - 1) * (X - T)` was below `-R` and the range clamp
made every closed sample read exactly `-48` however the level was computed. The corpus level was
moved to -50 dBFS and the test now asserts that some attenuated sample is *off* the clamp, rather
than accepting the mutant as covered.

### 7 — the dry path is flushed before the identity select

```
test signed_zero_identity_is_bit_exact ... FAILED
assertion `left == right` failed: scalar: the negative zero survived the ring and the select
```

### 8 — the ring is snapshotted in physical slot order

```
test active_restore_continues_against_uninterrupted ... FAILED
test a_track_restores_into_a_bank_whose_cursor_is_elsewhere ... FAILED
```

### 9 — the boundary check scans only the gain words

```
test nonfinite_input_recovers_lane_locally_at_the_block_boundary ... FAILED
test nonfinite_input_recovers_one_lane_of_a_bank ... FAILED
assertion `left == right` failed: the recovered block is all +0
```

### 10 — the boundary check scans only the output block

The case the gain scan exists for: a NaN `G` yields *finite* output, because `exp2_lane` clamps its
argument with the D8 `max`/`min` and those swallow NaN, so `A` is `2^-126` and `z * A` is finite.
Only `G` itself shows the fault.

```
test tests::injected_nonfinite_gain_has_scalar_w8_parity ... FAILED
```

### 11 — the detector tap is one sample early

```
test the_lookahead_tap_lands_the_detector_exactly_where_the_brief_says ... FAILED
assertion `left == right` failed: lookahead 0 ms: frame 4479 must still be the exact identity
```

### 12 — the one-pole rounds twice

`rate.mul(target.sub(G)).add(G)` instead of `rate.fma(target.sub(G), G)`. Inside the oracle
tolerance and therefore invisible to gate 7.3; the cross-target digest is what catches it, which is
why both gates exist.

```
test every_case_agrees_at_every_width_and_matches_its_pin ... FAILED
assertion `left == right` failed: gate/dual-mono/noise: the scalar oracle moved away from its pin
```

### 13 — the D7 `flush` is removed

```
test the_gain_word_is_flushed_to_zero_once_it_decays_below_the_band ... FAILED
assertion `left == right` failed: G decayed through the flush band and was stored as canonical +0
```

Note on an *equivalent* mutant: the flush is deliberately invisible in *output* bits, and the
kernel's doc comment says why — inside the flush band `exp2_lane(G)` already rounds to exactly
`1.0`, and `z * 1.0 == z`. The first attempt at this row used a PCM assertion and survived. The
observable is the stored state word, and the hazard is a target with hardware FTZ zeroing what a
native render keeps.

### 14 — the render path allocates one `Vec` per block

`core::hint::black_box(Vec::<f32>::with_capacity(frames))` at the top of `run_block`. The audit
allocator aborts on the first armed allocation rather than reporting it, so the bin dies with
SIGABRT (exit 134) instead of printing a report with `allocations > 0`. A first attempt using
`let _ = Vec::with_capacity(1)` survived: the release build elides an unused allocation, which is
worth knowing about any allocation-audit mutation.
