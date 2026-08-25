# Red-mutation record for the lane gates

Master plan for issue #83, §1.6: *every gate is proven red*. A test that has never failed is not a
gate. Each row below was applied to the working tree, the named test binary was run, the failure
output was recorded, and the mutation was reverted in the same session. Nothing in this file is
a claim about code that was not run.

Host: `x86_64` (Zen 5 class), `rustc 1.97.1`, workspace `.cargo/config.toml` pin
`-C target-feature=+avx2,+fma`, debug profile (the release profile only widens the random sweeps).

Reproduce one row with:

```
# apply the "mutation" edit, then
cargo test --locked -p miso-engine-lane --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | `Lane::select` swaps its operands: `m.bitselect(a, b)` becomes `m.bitselect(b, a)` | `src/wide_impl.rs` | `g1_op_identity` | RED |
| 2 | `Lane::max`/`min` forward to `wide`'s `max`/`min` instead of the D8 default | `src/wide_impl.rs` | `g1_op_identity` | RED |
| 3 | `Lane::neg` becomes `zero - self` instead of a sign-bit flip | `src/wide_impl.rs` | `g1_op_identity` | RED |
| 4 | `Lane::fma` at the vector widths becomes the unfused `(self * b) + c` | `src/wide_impl.rs` | `g2_kernel_identity` | RED |
| 5 | round-to-odd becomes unconditional (`s_bits \| 1`), losing the direction | `src/softfma.rs` | `g3_softfma` | RED |
| 6 | the `finite` guard is dropped from the round-to-odd adjustment | `src/softfma.rs` | `g3_softfma` | RED |
| 7 | the demotion becomes the naive `s as f32` (double rounding) | `src/softfma.rs` | `g3_softfma` | RED |
| 8 | `flush` compares with `le` instead of `lt` | `src/lib.rs` | `g4_flush` | RED |
| 9 | `FLUSH_EPS` drops to `1e-40`, below the top of the subnormal range | `src/lib.rs` | `g6_ftz_inert` | RED |
| 10 | `svf_block` drops the `s.ic1 = ic1` state write-back | `src/kernels.rs` | `p1_partition` | RED |
| 11 | `svf_step` returns its taps swapped: `(v2, v1)` instead of `(v1, v2)` | `src/kernels.rs` | `g2_kernel_identity` | RED |
| 12 | `svf_step` swaps `a2` and `a3` in `d2`: `fma(a2, v3, a3 * ic1)` | `src/kernels.rs` | `g2_kernel_identity` | RED |
| 13 | `history_push` writes only the low copy, dropping the mirror at `row + 32` | `src/kernels/halfband.rs` | `halfband` | RED |
| 14 | two even taps are swapped (`h[14]` and `h[16]`), breaking the half-band symmetry | `src/kernels/halfband.rs` | `halfband` | RED |
| 15 | `HALFBAND63_CENTER_SPLIT` moves from 15 to 16, putting the centre tap one position late | `src/kernels/halfband.rs` | `halfband` | RED |
| 16 | `Drop for CanonicalFpEnv` stops writing `self.saved` back | `src/fpenv.rs` | `fp_env` | RED (4 of 8) |
| 17 | `CanonicalFpEnv::enter` installs the caller's word instead of the canonical one | `src/fpenv.rs` | `fp_env` | RED (3 of 8) |

## Recorded failures

### 1 — `select` operand swap (G1)

```
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_signed_zero_max_and_min_follow_d8 ... FAILED
test g1_nan_max_and_min_follow_d8 ... FAILED
test g1_exp2_int_is_exact_on_the_integer_range ... FAILED
test g1_frexp_reconstructs_positive_normals ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
assertion `left == right` failed: Simd4: exp2_int(-126)
  left: 2130706432
 right: 8388608
```

### 2 — `wide`'s `max`/`min` instead of the D8 default (G1)

Only the NaN clause moves on x86 (`maxps` happens to agree with D8 on signed zeros); on NEON
`vmaxnmq` would move the signed-zero clause too, which is why both are in the pool.

```
test g1_nan_max_and_min_follow_d8 ... FAILED
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
G1 max at Simd4: lane 8: a=0x00000000 b=0x7fc00000 c=0x7fc00000 oracle=0x7fc00000 actual=0x00000000
G1 max at Simd4: lane 43: a=0x80000000 b=0x7fc00000 c=0xffc00000 oracle=0x7fc00000 actual=0x80000000
```

### 3 — `neg` as `zero - self` (G1)

```
G1 neg at Simd4: lane 0: a=0x00000000 b=0x00000000 c=0x00000000 oracle=0x80000000 actual=0x00000000
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
```

### 4 — unfused vector `fma` (G2)

This is the revision-4 form of the master plan's "reassociate one `fma` into mul+add in the AVX2
wrapper only": with no `#[target_feature]` wrapper left, the width-specific mutation is to unfuse
the `wide` widths and leave the scalar oracle fused.

```
test g2_kernels_are_bit_identical_at_every_width ... FAILED
assertion `left == right` failed: G2 svf_block/low / noise at Simd4: lane 0, frame 2:
  0xbbe1cd0f != oracle 0xbbe1cd10
```

### 5 — unconditional round-to-odd (G3)

```
test g3_soft_fma_equals_hardware_fma_on_the_edge_pool ... FAILED
test g3_soft_fma_equals_hardware_fma_on_the_midpoint_family ... FAILED
G3 edges: fma(1.5e0, 1e30, -1e0) = 1.5000001e30 (0x71977618), hardware 1.5e30 (0x71977617)
assertion `left == right` failed: G3: 142 edge-pool mismatches
G3 midpoint: fma(2.7923584e-1, 4.08128e5, -5.684342e-14) = 1.1396397e5 (0x47de95fc),
             hardware 1.1396396e5 (0x47de95fb)
```

### 6 — no `finite` guard (G3)

```
test g3_soft_fma_equals_hardware_fma_on_the_edge_pool ... FAILED
G3 edges: fma(0e0, 0e0, inf) = NaN (0x7fc00000), hardware inf (0x7f800000)
assertion `left == right` failed: G3: 2523 edge-pool mismatches
```

### 7 — naive `(p + c) as f32` (G3)

```
test g3_soft_fma_equals_hardware_fma_on_the_edge_pool ... FAILED
test g3_soft_fma_equals_hardware_fma_on_the_midpoint_family ... FAILED
assertion `left == right` failed: G3: 144 edge-pool mismatches
```

### 8 — `flush` with `le` (G4)

```
test g4_flush_law_holds_at_every_width ... FAILED
test g4_flush_is_lane_wise ... FAILED
assertion `left == right` failed: f32: flush(1e-20) must be unchanged
  left: 0
 right: 507307272
```

### 9 — `FLUSH_EPS = 1e-40` (G6)

With the threshold below the top of the subnormal range, subnormal state words survive the flush,
hardware FTZ starts to matter, and the FTZ-on and FTZ-off digests separate — which is exactly the
property D7 exists to prevent.

```
test g6_flush_makes_hardware_ftz_inert ... FAILED
assertion `left == right` failed: G6: the flush law must be FTZ-inert
```

### 10 — dropped state write-back (P1)

```
test p1_every_kernel_is_partition_invariant ... FAILED
assertion `left == right` failed: P1 svf_block/low / noise at f32: partition 1 differs from the
one-shot run at sample Some(1)
```

## Policy-script mutations

The lane policy script has its own mutation test, `scripts/test-lane-policy.sh`, run in CI next to
`scripts/check-lane-policy.sh`. It proves each clause separately: `mul_add` outside the lane crate,
`wide::` outside the lane crate, `core::arch` outside `softfma.rs`, an unmarked `wide` method call
inside the lane crate, a lockfile with an unpinned `wide`, and a lane dependency that is neither
`wide` nor a workspace crate.

## Not proven here

* The wasm `v128` soft-FMA body (`softfma.rs`, `wasm_simd128`) is compile-checked for
  `wasm32-unknown-unknown` with `+simd128` but not executed: running it needs the `wasmtime` gate
  crate, which is job 83d. Its scalar twin is the code G3 proves, and the vector body is the same
  operations in the same order.
* Cross-target digests (G5) and the AArch64 leg of G1 are 83d/CI-runner work for the same reason.

### 11 — `svf_step` returns its taps swapped

```
thread 'g2_svf_step_yields_both_taps_of_one_state' panicked at
  crates/miso-engine-lane/tests/g2_kernel_identity.rs:214:17:
assertion `left == right` failed: svf_step band-pass tap at width 1, lane 0, frame 0
```

The gate's oracle is a transcription of Simper's recurrence written inside the test, not
`svf_block`: `svf_block` is now *defined* by `svf_step`, so comparing the two would agree with any
mutation of `svf_step` and prove nothing. The first version of this gate did exactly that and
survived this mutation; the transcription is what makes it red.

### 12 — `svf_step` swaps `a2` and `a3` in `d2`

```
thread 'g2_svf_step_yields_both_taps_of_one_state' panicked at
  crates/miso-engine-lane/tests/g2_kernel_identity.rs:220:17:
assertion `left == right` failed: svf_step low-pass tap at width 1, lane 0, frame 0
```

### 13-15 — the polyphase half-band module (issue #91)

Added with `src/kernels/halfband.rs`. Row 13 (`history_push` writes one copy instead of two)
fails `the_double_written_history_addresses_every_age_at_every_position` at age 17 of position 15,
which is exactly the wrap the mirrored row exists to make contiguous. Row 14 fails
`the_even_tap_table_is_the_symmetric_half_band` with `tap 6 breaks the half-band symmetry
h[2k] = h[62-2k]`. Row 15 fails the same test on `HALFBAND63_CENTER_SPLIT`, which is the position
of the centre tap in the frozen ascending accumulation order; moving it is the mutation that would
silently reassociate the decimator sum.

The bit-identity of these kernels against the 63-tap graph they replace is *not* proved here: that
graph lives in `miso-engine-soft-clip`, and the proof is its `tests/polyphase_identity.rs`
(recorded in that crate's `tests/MUTATIONS.md`).

## Issue #146 — the canonical floating-point environment

Rows 16 and 17 are the guard's own two failure modes, and they fail different halves of the test
binary, which is why both are recorded rather than one standing for the other.

* **Row 16 (no restore).** `a_hostile_caller_word_is_normalised_and_returned_bit_exactly`,
  `the_caller_word_survives_an_unwind_through_the_guard`,
  `attestation_passes_from_a_hostile_caller_word` and
  `sticky_status_flags_do_not_fail_the_attestation_but_a_control_bit_does` all fail:

  ```
  assertion `left == right` failed: every bit of the caller's word must come back, status flags included
  assertion `left == right` failed: the guard must restore the caller's word while unwinding
  test result: FAILED. 4 passed; 4 failed
  ```

  The same mutation is red at both hosts as well:
  `cargo test -p miso-engine-capi --lib fp_environment` fails 2 of 2 with
  `a refused descriptor leaked MXCSR`, and
  `cargo test -p miso-engine-host-core --test fp_environment` fails 3 of 3.

* **Row 17 (canonical word not installed).** The guard becomes a no-op transition:

  ```
  assertion `left == right` failed: the guard must install 0x1F80: masked, round-to-nearest-even, no FTZ, no DAZ
  assertion `left == right` failed: inside the guard the same product must be the exact IEEE subnormal `2^-130`
  test result: FAILED. 5 passed; 3 failed
  ```

  and the full-corpus gate goes red with the #144 divergence, unchanged:

  ```
  issue #146: a render entry's canonical environment did not normalise 70 of 331 comparisons under a caller's FTZ+DAZ
  ```

## M-146-V1 (verifier-added, #146 review): the scheduling barrier is deliberately undiscriminated
Mutation: empty `scheduling_barrier` entirely (no asm block). Observed: `fp_env` release suite
green (its cases self-anchor by design — the recorded register-only limit), and release
`g6_full_corpus_ftz` green (current codegen does not hoist the corpus render's memory-dependent
operations even unaided). Conclusion: no deterministic red exists for the barrier by construction —
it is defense-in-depth against future codegen, its necessity evidenced by the recorded pre-barrier
release failure of the un-anchored lane case, not by a standing gate. Do not delete it on the
strength of a green sweep; this record is the reason the sweep is green without it.
