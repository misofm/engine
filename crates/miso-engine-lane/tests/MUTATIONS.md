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
