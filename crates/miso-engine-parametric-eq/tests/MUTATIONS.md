# Red-mutation record for the parametric-EQ gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure was
recorded, and the mutation was reverted in the same session. Nothing here is a claim about code that
was not run — including the two mutants that survived, which are recorded with the reason.

This record matters more than usual for this crate. The gate it replaces
(`endpoint_conditioned_delta_matches_the_independent_oracle_on_the_complete_grid`) was green while
the shipped kernel was 12.4859 dB out on 483 of its 1,488 rows, because it evaluated a rational
function of the stored words rather than the transfer the kernel computed (#87 F1). A gate that
cannot be made red by breaking the thing it claims to certify is the failure mode this file exists
to rule out.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile.

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p miso-engine-parametric-eq --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | `a1` storage instead of `c1`: `c1 = 1 - f32(1 / (1 + t))`, the master plan's pre-A1 text | `src/lib.rs` | `analytic` | RED |
| 1b | the same mutation, measured by the 48 one-second impulses | `src/lib.rs` | `time_domain` | SURVIVED |
| 2 | low shelf without the `sqrt(A)` prewarp: `g = tan(pi f0/fs)` | `src/lib.rs` | `analytic` | RED |
| 3 | high-shelf `m1` sign flipped | `src/lib.rs` | `analytic` | RED |
| 4 | bell damping without the gain factor: `k = 1/Q` instead of `1/(Q A)` | `src/lib.rs` | `analytic` | RED |
| 5 | the snap does not assign the target, it only stops the ramp | `src/lib.rs` | `contract` | RED |
| 6 | the ramped segment loses its pre-advance, so frame 0 uses the pre-event words | `src/lib.rs` | `contract` | RED |
| 7 | `ramp_frames = length` instead of `length - 1`, so the block steps once too often | `src/lib.rs` | `contract` | RED |
| 8 | a block is never cut at a lane's ramp end, so lanes of different ramp ages share one segment | `src/lib.rs` | `bank` | RED |
| 9 | the ramp increment is not stored in the payload (word 8..14 written as zero) | `src/lib.rs` | `contract` | RED |
| 10 | a rejected block is zeroed but the integrators are not reset | `src/lib.rs` | `time_domain` | RED |
| 11 | the identity section is `m0 = 1 - 2^-24` instead of `1.0` | `src/lib.rs` | `contract` | RED |
| 12 | the ramp increment is `2^-6 * (1 + 1e-7)` instead of exactly `2^-6` | `src/lib.rs` | `determinism` | RED |
| 13 | the D7 flush is the identity | `../miso-engine-lane/src/lib.rs` | `time_domain` | RED |
| 14 | `design_svf_v1` drops its spectral-norm guard | `src/lib.rs` | `analytic` | SURVIVED |
| 15 | the automation domain check always validates against the frequency spec | `src/lib.rs` | `contract` | RED |
| 16 | a settled band accepts stored words that disagree with its stored parameters | `src/lib.rs` | `contract` | RED |
| 17 | the payload length check accepts trailing bytes (`<` instead of `!=`) | `src/lib.rs` | `contract` | RED |
| 18 | `word_spectral_norm` drops the off-diagonal term of `M^T M` | `src/lib.rs` | `analytic` | RED |
| 19 | the corpus reads a lane back from the mirrored AoSoA offset | `src/corpus.rs` | `determinism` | RED |
| 20 | the bypass path renders instead of copying the dry block | `src/lib.rs` | `contract` | RED |
| 21 | the corpus stops staggering its per-lane ramp ends | `src/corpus.rs` | `determinism` | RED |
| 22 | a `-0.0` automation value is not normalised to `+0.0` on the way in | `src/lib.rs` | `contract` | RED |

## Recorded failures

### 1 — `a1` storage instead of `c1`

The amendment A1 evidence, reproduced as a mutation.
`svf_words_match_the_independent_oracle_on_the_complete_grid` fails at `analytic.rs:226`: the
low-frequency, high-Q rows exceed the frozen 0.005 dB tolerance, because `t = g(g + k)` is about
4.7e-6 at 10 Hz / Q = 18 / 88.2 kHz and storing `1 - t` instead of `t` spends the whole `f32`
mantissa on the leading one.

### 1b — the same mutation, measured on the impulses: **SURVIVED**

Recorded rather than quietly dropped. The 48 frozen impulse rows are the two parameter *corners*
(10 Hz / -24 dB / Q = 0.1 / S = 0.1 and 20 kHz / +24 dB / Q = 18 / S = 1.0), and neither is where
`a1` storage hurts: at 10 Hz the corner uses Q = 0.1, so `k = 10` and `t = 3.6e-3` — a relative
error of 1.7e-5, not the 0.3 % the Q = 18 row carries; at 20 kHz `g` is large and `t` is order one.
The row that separates the two storages is 10 Hz **with Q = 18**, which lives in the analytic grid
(row 1) and not in the impulse set. The impulse gate is therefore a gate on the realization and the
flush, not on the storage, and row 1 is what protects the storage. The impulse set is deliberately
left as issue #42 froze it.

### 2, 3, 4 — mapping mutations

Each fails `the_f64_mapping_reproduces_the_verified_reference_mapping` at `analytic.rs:144` on the
first shelf or bell row, before any `f32` rounding is involved; 4 additionally fails the `f32` grid
and the PCM-fixture rows. These are the three mutations issue #105 used against its own oracle
(its M1/M2/M3), applied to this crate's side of the mapping.

### 5, 6, 7 — the D11 word ramp

All three fail `automation_starts_a_64_sample_word_ramp`. 5 leaves the words at
`start + 64 * step` instead of exactly `target`; 6 makes frame 0 use the pre-event words, so the
first ramped word is `start`; 7 advances twice on the first frame. 7 also fails
`automation_is_partition_invariant`, because the number of additions then depends on how the block
was cut.

### 8 — one segment for lanes of different ramp ages

`every_width_matches_the_scalar_instantiation` fails at block 2, track 0, frame 0. This is the
mutation that matters for the bank: at `WIDTH = 1` there is only one lane, so no cut is ever needed
and the scalar path is unaffected; at `WIDTH = 8` the lanes whose ramps ended mid-block keep
stepping past their target. A width difference that only appears when ramps of different ages
coexist is exactly what the corpus's staggered case (21) and this gate are for.

### 9 — the increment is not stored

`state_restore_continues_active_ramp_bit_exactly` fails on the continuation block: the restored
effect re-derives an increment from the remaining distance, which is the pre-D11 law, and the two
renders diverge within one block.

### 10 — a rejected block does not reset

`a_non_finite_input_block_is_zeroed_counted_once_and_leaves_the_next_block_clean` fails on the
snapshot: the NaN that entered the integrators survives the zeroing of the output, so the *next*
block is poisoned too and the counter under-reports the fault.

### 11 — a near-identity identity

`disabled_and_zero_db_sections_return_dry_bits_with_zero_state_growth` fails on the first sample.
`1 - 2^-24` is inaudible and still wrong: a disabled slot must be the input bit for bit, or a
cohort's membership becomes observable in its output.

### 12, 19, 21 — the cross-target corpus

12 moves the `cascade/ramped_noise` digest away from its pin (the increment is no longer an exact
power of two, so the ramp accumulates differently); 19 breaks width agreement itself
(`simd4 vs scalar` fails before the pin is even consulted); 21 removes the per-lane stagger, which
moves the ramped digest and would have left mutation 8 undetected by this gate.

### 13 — the flush is the identity

`flush_keeps_decaying_state_out_of_the_subnormal_range` fails inside the impulse loop: the retained
words of the 44.1 kHz / 20 kHz / +24 dB / Q = 18 bell — the row that "recovered" at sample 39,223
under the old predicate — enter the subnormal range as the tail decays. With the flush they are
either exactly `+0.0` or at least `1e-20`.

### 14 — the spectral-norm guard is dropped: **SURVIVED**

Also recorded rather than dropped. `word_spectral_norm(words) > NORM_TOLERANCE` is a guard on a
region that no in-domain parameter set reaches: `word_ramps_are_contractive_on_every_grid_row`
measures a worst norm of `1 + 1.03e-7` over 372 rows and 207,018 convex combinations, and the
10,000 seeded designs reach `1 + 6.7e-8`, both inside the `1 + 2^-22` tolerance. Removing the guard
therefore cannot fail a test that only feeds it legal parameters — which is the *point* of the
measurement, not a hole in it. What must be discriminating is the predicate, and that is mutation
18: `the_spectral_norm_predicate_separates_contractive_from_expansive_words` pins a triple whose
spectral radius is 1 and whose operator norm is 1.4, and dropping the off-diagonal term of `M^T M`
scores it 1.0 and fails. The guard stays because a future parameter-domain change is exactly the
event that would make the region reachable.

### 15, 16, 17, 20 — contract surfaces

15 makes every automation value validate against the frequency domain, so the Q point at 1.0 is
rejected and `malformed_automation_rejects_each_span_without_losing_valid_targets` counts seven
invalid spans instead of six. 16 accepts a settled band whose stored coefficient words disagree
with its stored parameters — a payload that would render something the session does not describe.
17 accepts a payload longer than the layout, whose surplus is either another layout's data or
uninitialised memory. 20 renders the
signal on the bypass path, and `bypass_copies_dry_bits_and_leaves_the_state_alone` fails because
bypass must preserve the dry bits *and* the latency, not approximate them.

### 22 — `-0.0` is not normalised

`a_negative_zero_automation_value_is_accepted_as_zero` fails on the stored target: the payload
carries `-0.0` and a later restore has to decide what to do with a value five of the eight effect
crates used to reject outright. 83c decision 3 settled it — accepted as a way of writing zero,
normalised on the way in — so nothing downstream of the validator ever sees a negative zero.
