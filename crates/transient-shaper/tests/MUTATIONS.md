# Red-mutation record for the transient-shaper gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile unless noted.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p transient-shaper --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | `DB_PER_OCTAVE = 20.0` — the `log10`/`log2` confusion in the contrast scale | `src/lib.rs` | `oracle` | RED (both tests) |
| 2 | `OCTAVES_PER_DB = 0.05` — the `exp2`/`pow10` confusion in the gain law | `src/lib.rs` | `oracle` | RED (both tests) |
| 3 | identity mask drops its `shape == 0` term | `src/lib.rs` | `contract` | RED |
| 4 | the restored ramp derives `step` from `RAMP_SAMPLES` instead of `remaining` | `src/lib.rs` | `contract` | RED |
| 5 | `LinkMode::Maximum` dispatched to the `LINK_AVERAGE` instantiation | `src/lib.rs` | `contract` | RED |
| 6 | `finish_block` skipped after the frame loop (D7 boundary check gone) | `src/lib.rs` | `boundary` | RED (both tests) |
| 7 | `replace_lane` writes lane `0` instead of `lane` | `src/lib.rs` | `bank` | RED |
| 8 | the ramp prefix is capped at one frame, so parameters become block rate | `src/lib.rs` | `partition` | RED (both tests) |
| 9 | `DB_PER_OCTAVE` perturbed by one ulp (`0x40c0a8c2`) | `src/lib.rs` | `cross_target` | RED |
| 10 | `Ramps::advance` packs lanes in reverse (`packed[index][W - 1 - lane]`) | `src/lib.rs` | `cross_target` | RED |
| 11 | `Vec::with_capacity(1)` inside `Shaper::process_block` | `src/lib.rs` | `allocation` | RED (2,000 allocations) |
| 12 | the scalar side of the bank comparison is rendered with `bypass = true` | `tests/bank.rs` | `bank` | RED |

Rows 13–15 are the three mutations that prove `effect_runtime::envelope::ar_one_pole_step`
— added from this job — and are recorded in that crate's `tests/MUTATIONS.md` as rows 16–18: the
strict `u > e` switch, the two-product form against the one-rounding one, and the D7 flush on the
recurrence. Row 15 (no flush) is what `boundary::a_subnormal_input_renders_and_flushes_the_envelope`
also catches from this side.

## Recorded failures

### 1 and 2 — the two dB scale constants

```
thread 'scalar_matches_the_independent_f64_oracle' panicked at tests/oracle.rs:62:
thread 'impulse_step_and_decay_cover_both_attack_and_sustain_signs' panicked at tests/oracle.rs:85:
test result: FAILED. 0 passed; 2 failed
```

Both constants are the whole content of the transcendental swap: `DB_PER_OCTAVE` turns `log2` into
dB and `OCTAVES_PER_DB` turns dB back into an `exp2` argument. Getting either wrong by the factor
`log2(10)` leaves the 0.01 dB gate immediately, which is why the two are pinned as bit patterns and
not as decimal literals.

### 3 — the signed-zero identity select

```
thread 'identity_rules_are_bit_exact_and_the_followers_still_warm' panicked at tests/contract.rs:343
```

`exp2_lane(0)` is exactly `1`, so a `shape == 0` lane already computes `gain == 1` and the mix law
`fma(mix, x * 1 - x, x)` is *numerically* the identity — but it maps `-0.0` to `+0.0`. The select is
what preserves the sign bit, and the `-0.0` rows of the defaults case are what catch its removal.

### 4 — deriving the ramp step on restore

```
thread 'automation_updates_one_sixty_three_sixty_four_retargets_and_restores_exactly'
    panicked at tests/contract.rs:242
```

The persisted layout is eleven words and does not carry the D11 `step`, so a restore has to derive
it. `(target - current) / remaining` is the only derivation that resumes the ramp the writer was
running; `(target - current) / RAMP_SAMPLES` restarts it, and the restored continuation stops
matching the uninterrupted one.

### 5 — link-mode dispatch

```
thread 'link_modes_drive_the_detector_as_specified' panicked at tests/contract.rs
```

Swapping the *values* of `LINK_MAXIMUM` and `LINK_AVERAGE` is an equivalent mutant — the constants
are only ever compared against themselves — so the mutation that means anything is at the dispatch,
where the link mode chooses an instantiation. Reading the detector off the envelope state word after
one frame is what makes the three modes distinguishable to the last bit.

### 6 — the boundary check

```
a_nonfinite_block_is_zeroed_and_the_envelopes_are_reset --- FAILED
a_nonfinite_bank_block_is_rejected_as_a_unit --- FAILED
```

With `finish_block` gone the NaN reaches the output. This is the D7 replacement for the seven
per-value `Option` classifications the pre-audit crate ran per lane-sample, so it has to be the
thing that actually catches a non-finite block, not a formality next to a per-value check.

### 7 — lane-local state

```
bank_snapshot_restore_and_resets_are_track_local --- FAILED
```

Envelope words live packed across lanes, so a single-track restore is a read-modify-write of one
lane of a vector. Writing lane 0 for every track passes a round trip of track 0 and corrupts every
other track, which the peer assertions catch.

### 8 — the ramp prefix

```
the_scalar_product_is_partition_invariant --- FAILED
the_bank_is_partition_invariant --- FAILED
```

Capping the prefix at one frame makes the three parameters block rate: a 512-frame render advances
each ramp once, a 512 x 1-frame render advances it 512 times, and the block boundary becomes
audible. Note the *opposite* mutation — `prefix = frames`, evaluating the ramp on every frame of
every block — is **equivalent**, because `LinearRamp::next_value` at `remaining == 0` returns
`current` unchanged; it costs time and changes no bits, and is disclosed below rather than claimed.

### 9 and 10 — the cross-target pins

```
the_pinned_digests_hold --- FAILED          (DB_PER_OCTAVE + 1 ulp)
every_width_produces_the_same_words --- FAILED   (reversed lane packing)
```

Row 9 shows the digest resolves a one-ulp coefficient change: the pin is a real constraint on the
arithmetic, not a checksum of the corpus shape. Row 10 is the width-identity half — reversing the
lane order inside `Ramps::advance` is exactly identity at `W = 1` and wrong at `W = 4` and `W = 8`,
which is the class of bug the whole lane-identity claim exists to exclude.

### 11 — allocation

```
the_render_path_allocates_nothing --- FAILED (left: 2000, right: 0)
test result: FAILED. 2 passed; 1 failed
```

Refreshed for issue #332 on 2026-09-03 in an isolated copy carrying the revised thread-local
harness. The mutation allocates once in each scalar and native-bank `process_block` call across the
1,000-block workload, so the allocation assertion observed 2,000 rather than 0. Both harness
controls passed. Removing the mutation restored the isolated production file byte-for-byte to the
clean tree, and the same allocation binary then passed 3/3.

### 12 — the bank comparison is live

```
bank_matches_scalar_pcm_state_and_reports_for_every_link_mode --- FAILED
```

Lane identity holds by construction — one `#[inline(always)]` generic body, instantiated at `f32`
and at `Simd8` — so the gate can only prove that the comparison is real. Rendering the scalar side
bypassed is that proof.

## Disclosed equivalent mutants

Two mutations were tried, survived, and are recorded here with the arithmetic rather than quietly
dropped.

* **`Average` link written as `0.5 * (|l| + |r|)`** instead of `0.5 * |l| + 0.5 * |r|`. Scaling by
  a power of two is exact and rounding commutes with it, so `fl(0.5l + 0.5r)` and `0.5 * fl(l + r)`
  agree on every pair whose sum is normal. They can differ where `l + r` overflows or where the
  result is subnormal; the block boundary check excludes the first and `flush` clears the second
  before it can enter the recurrence. The two-product form is kept anyway, because it is the form
  the frozen brief writes and because "equivalent under the current guards" is not "equivalent".
* **`prefix = frames`** (evaluate the ramp on every frame rather than only while one is running).
  Bit-identical for the reason given under row 8. It is a performance regression — it puts three
  `LinearRamp::next_value` calls per lane per channel back into the steady-state loop, which is
  audit finding F6 — and is rejected on that ground, not on a numeric one.
