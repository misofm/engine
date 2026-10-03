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
cargo test --locked -p lane --test <test binary>
# and revert
```

| # | mutation | file | test binary | result |
|---|---|---|---|---|
| 1 | `Lane::select` swaps its value operands: `m.select(a, b)` becomes `m.select(b, a)` | `src/wide_impl.rs` | `g1_op_identity` | RED |
| 2 | `Lane::max`/`min` forward to `wide`'s `max`/`min` instead of the D8 rule | `src/wide_impl.rs` | `g1_op_identity` | RED |
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
| 18 | `Lane::max`/`min` take wasm's `pmax`/`pmin` operand order on x86: `self.fast_max(b)` becomes `b.fast_max(self)` | `src/wide_impl.rs` | `g1_op_identity` | RED |

## Recorded failures

### 1 — `select` operand swap (G1)

Re-run after round 2 moved `Lane::select` from `bitselect` to `wide`'s `select` (one `blendv` on
x86, the identical `v128.bitselect`/`vbsl` call on wasm and NEON). Every comparison in the pool is
read back through a `select`, so the swap fails on `lt` before it reaches an arithmetic case.

```
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
G1 lt at Simd4: lane 0: a=0x00000000 b=0x00000000 c=0x00000000 oracle=0x00000000 actual=0x3f800000
assertion `left == right` failed: G1: lt differs from the scalar oracle at Simd4 in 1232 of 1232 lanes
  left: 1232
 right: 0
```

### 2 — `wide`'s `max`/`min` instead of the D8 rule (G1)

Only the NaN clause moves on x86 (`maxps` happens to agree with D8 on signed zeros); on NEON
`vmaxnmq` would move the signed-zero clause too, which is why both are in the pool. Applied to the
round-2 lowering, this is `self.fast_max(b)` becoming `self.max(b)` — the fix-up `wide` wraps
`maxps` in is exactly what D8 does not want.

```
test g1_nan_max_and_min_follow_d8 ... FAILED
test g1_max_and_min_lowerings_match_the_oracle ... FAILED
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
G1 max at Simd4: lane 4: a=0x00000000 b=0x7fc00000 c=0x00000000 oracle=0x7fc00000 actual=0x00000000
assertion `left == right` failed: G1: max differs from the scalar oracle at Simd4 in 18 of 104 lanes
```

The `min` half of the same row (`self.fast_min(b)` becoming `self.min(b)`) fails the mirror cases:

```
test g1_nan_max_and_min_follow_d8 ... FAILED
test g1_max_and_min_lowerings_match_the_oracle ... FAILED
G1 min at Simd4: lane 4: a=0x00000000 b=0x7fc00000 c=0x00000000 oracle=0x7fc00000 actual=0x00000000
assertion `left == right` failed: G1: min differs from the scalar oracle at Simd4 in 18 of 104 lanes
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
  crates/lane/tests/g2_kernel_identity.rs:214:17:
assertion `left == right` failed: svf_step band-pass tap at width 1, lane 0, frame 0
```

The gate's oracle is a transcription of Simper's recurrence written inside the test, not
`svf_block`: `svf_block` is now *defined* by `svf_step`, so comparing the two would agree with any
mutation of `svf_step` and prove nothing. The first version of this gate did exactly that and
survived this mutation; the transcription is what makes it red.

### 12 — `svf_step` swaps `a2` and `a3` in `d2`

```
thread 'g2_svf_step_yields_both_taps_of_one_state' panicked at
  crates/lane/tests/g2_kernel_identity.rs:220:17:
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
graph lives in `soft-clip`, and the proof is its `tests/polyphase_identity.rs`
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
  `cargo test -p capi --lib fp_environment` fails 2 of 2 with
  `a refused descriptor leaked MXCSR`, and
  `cargo test -p host-core --test fp_environment` fails 3 of 3.

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

### 18 — wasm's `pmax`/`pmin` operand order used on x86 (G1)

Round 2 lowers `Lane::max`/`Lane::min` to one instruction per backend, and the two backends want
opposite operand orders: x86's `maxps(a, b)` is `a > b ? a : b`, while wasm's `f32x4.pmax(z1, z2)`
is `z1 < z2 ? z2 : z1` and therefore has to be called as `pmax(b, a)`. Writing the wasm order on
x86 is the one mistake this shape can make, and it is silent on ordinary values: it moves only the
tie and the unordered cases. Both halves were applied and reverted; `max` first:

```
test g1_exp2_int_is_exact_on_the_integer_range ... FAILED
test g1_nan_max_and_min_follow_d8 ... FAILED
test g1_max_and_min_lowerings_match_the_oracle ... FAILED
test g1_signed_zero_max_and_min_follow_d8 ... FAILED
test g1_directed_edge_pool_is_lane_identical ... FAILED
test g1_random_vectors_are_lane_identical ... FAILED
G1 max at Simd4: lane 1: a=0x00000000 b=0x80000000 c=0x00000000 oracle=0x80000000 actual=0x00000000
assertion `left == right` failed: G1: max differs from the scalar oracle at Simd4 in 36 of 104 lanes
```

then `min`, whose signed-zero clause is the clearest statement of what a swapped order costs:

```
test g1_signed_zero_max_and_min_follow_d8 ... FAILED
assertion `left == right` failed: Simd4: min(-0.0, +0.0)
  left: 2147483648
 right: 0
```

The wasm half of the same claim cannot be run here. It is executed instead by gate G5's two wasm
legs, which call `miso_gate_minmax_lowering_mismatches` inside the guest over the same ordered pool
and require zero (`tools/wasm-gate-corpus`, `scripts/run-wasm-gates.sh`).

## M-146-V1 (verifier-added, #146 review): the scheduling barrier is deliberately undiscriminated
Mutation: empty `scheduling_barrier` entirely (no asm block). Observed: `fp_env` release suite
green (its cases self-anchor by design — the recorded register-only limit), and release
`g6_full_corpus_ftz` green (current codegen does not hoist the corpus render's memory-dependent
operations even unaided). Conclusion: no deterministic red exists for the barrier by construction —
it is defense-in-depth against future codegen, its necessity evidenced by the recorded pre-barrier
release failure of the un-anchored lane case, not by a standing gate. Do not delete it on the
strength of a green sweep; this record is the reason the sweep is green without it.

## G2 `g2_interleaved_cascade_equals_a_chain_of_blocks` (issue #163 phase 3)

The gate asserts that `svf_cascade_interleaved` is bit-for-bit a chain of `svf_block` calls -- the
shape the parametric EQ ran before phase 3 -- in both the audio it writes and the integrator words
it leaves behind, at every width and at every depth that divides a four-section cascade.

Red mutations run against it, each reverted after it was seen to fail:

| mutation in `kernels.rs::svf_cascade_interleaved`            | result |
|--------------------------------------------------------------|--------|
| `&mut state[stream][section]` -> `&mut state[stream][section % 1]` (collapse every section's integrators onto slot 0) | FAILED |
| `&c[stream][section]` -> `&c[stream][(section + 1) % D]` (rotate the coefficient sets within a pass) | FAILED |
| output mix `m2.fma(v2, ..)` -> `m2.mul(v2).add(..)` (reassociate one fused multiply-add) | FAILED |

The third is the phase-3 twin of the `svf_block` reassociation mutation above: it is the one that
proves the gate is an identity and not a tolerance.

## Issue #944 — the select-free settled matrix kernel

`matrix2x2_block_without_identity` is `matrix2x2_block`'s second arm with no `L::select` in the
body. Gate 1, `select_free_matrix_matches_the_select_form_when_no_lane_is_identity`
(`tests/fader_matrix.rs`), holds it equal to `matrix2x2_block` with no identity lane over the four
hostile families, frame counts `[1, 3, 8, 9, 128]`, guard words included, at `f32`, `Simd4` and
`Simd8`, in dev and in release. NaN words compare as "both NaN" (amendment 1): in the release
build LLVM commutes the right-output sum differently in the two kernels, and x86 keeps the first
operand's payload. With that clause removed from the comparator the release run is red at
`width=4 frames=1 family=3: R[3] new=7fc00000 old=7fc01234` while dev stays green -- which is why
the clause exists and why the gate runs in both profiles.

Command form: `cargo test -p lane --test fader_matrix select_free` (and with `--release`).

| # | mutation | gate | observed |
| --- | --- | --- | --- |
| M3 | swap `c.lr` and `c.rl` in `matrix2x2_block_without_identity` | gate 1, dev and release | FAILED, `width=1 frames=1 family=0: L[1] new=bfc00000 old=c04c0000` |

The same mutation is red in `builtins` (gate 3) and `console-workload` (gate 4); those rows, and
M1, M2 and M4, are in `crates/builtins/tests/MUTATIONS.md`.

## Issue #943 — the banked sample-peak kernel

`meter_sample_peak_block` (`src/kernels/builtins.rs`) is gate G1's subject:
`tests/meter_peak.rs` holds `Simd8` and `Simd4` against the kernel at `Lane = f32` over the
de-interleaved lane and against the builtin meter's own serial loop, over 64 carried blocks at frames
1, 2, 3, 127, 128 and 129, plus an invalid-only lane and a bit sweep of every exponent's boundary
mantissas. The same mutations were run against gate G2 (`crates/builtins/tests/MUTATIONS.md`) and
against the wasm-gate corpus case `meter_sample_peak_block/hostile` (case 55), whose native leg
compares every width with the pin taken from the scalar `Lane` oracle. Each row was applied alone
as an exact-text replacement (match count one) to `1975fc44` plus the evidence commit's final G2,
run in dev (the corpus in release), and restored. Host AMD EPYC 7313P, rustc 1.97.1, pin `+avx2,+fma`.

| # | mutation | G1 `meter_peak` | corpus `g5_native_digests_match_pins` |
| --- | --- | --- | --- |
| L-1 | `a.ge(low)` becomes `a.ge(L::zero())` (admits subnormals) | RED, 3 of 4: `width 8 frames 1 block 0 lane 3: the vector kernel against the meter's serial loop` (`1` against `0`); the invalid-only lane `frames 127 lane 1` (`8388607` against `0`); the sweep at input `0x00000001` | RED at case 55, every width |
| L-2 | drop the `a.lt(high)` term: `L::select(a.ge(low), a, L::zero())` (admits infinity) | RED, 3 of 4: `width 8 frames 1 block 6 lane 4` (`2139095040`, `+inf`, against `2^1`); `frames 3 lane 1` (`+inf` against `0`); the sweep at `0x7f800000` | RED at case 55 |
| L-green | `L::max(c, peak)` becomes `L::max(peak, c)` | GREEN, 4 of 4 | GREEN |

L-green is the expected-green row: on the sanitized domain `{+0.0} ∪ [MIN_POSITIVE, MAX]` the D8
select form is commutative by bits, so the operand order is free.
`g1_select_max_is_order_free_only_on_the_sanitized_domain` witnesses that argument directly, and
also that it fails off the domain (`max(+0, -0)` and `max(1, NaN)` depend on the order).

## Issue #949 — the `f64` lane vocabulary

`src/f64_lane.rs` adds `LaneF64` and `Widen`. Gates 1 to 3 are `tests/f64_lane.rs`; gate 4 is the
`f64_lane_mismatches` count that `tools/wasm-gates` reads on its native, scalar-wasm and
simd128-wasm legs; gate 5 is `check_f64_lane_lowering` in `scripts/run-wasm-gates.sh`. Each row was
applied alone as an exact-text replacement (match count one per site) on top of the attempt-1 tree,
run, and restored by copying the saved original back (`cmp` clean). Gates 1 to 3 were run in dev
(`cargo test --locked -p lane --test f64_lane`); gate 4 on all three legs with the same build
commands as `scripts/run-wasm-gates.sh`; gate 5 by running that script. Host AMD EPYC 7313P,
rustc 1.97.1, pin `+avx2,+fma`, wasmtime 47.0.3.

| # | mutation | gates 1-3 (`f64_lane`, dev) | gate 4 (`f64_lane_mismatches`) | gate 5 |
| --- | --- | --- | --- | --- |
| F-1 | `Widen for f32x8` puts lanes 4..8 first: `[a[4], a[5], a[6], a[7], a[0], a[1], a[2], a[3]].map(f64::from)` | RED: gate 1 `widen at Simd8 over the directed pool and sparse sweep` (`522496` against `0`); gate 3 `square witness at Simd8` (`100096`) | RED: `81676` on native, scalar wasm and simd128 wasm | green (the probe is `Simd4`) |
| F-2 | both vector `widen`s map a subnormal input to `+0.0`: `map(\|x\| if x.is_subnormal() { 0.0 } else { f64::from(x) })` | RED: gate 1 `widen at Simd4 …` (`1047`); gate 3 `square witness at Simd4` (`369`) | RED: `518` on every leg, which is exactly the subnormal rows of the widen set (255 sparse patterns plus 4 pool entries, at the two vector widths) | green |
| F-3 | vector `add` becomes `(self + b) + <$simd>::splat(0.0)` | RED: gate 2 `add/mul at Simd4 over the pool` (`4`: the `-0.0 + -0.0` pair at four rotations); the random pairs stay green | RED: `2` on every leg (`-0.0 + -0.0` at `Simd4` and `Simd8`) | green |
| F-4 | the guest probe widens through a `black_box`ed scalar loop (`*value = black_box(f64::from(x))`, then `LaneF64::load`) | not applicable | green, `0` on every leg (the values are the same) | RED: `the f64 lane probe is not vectorised on the simd128 leg: f64x2.promote_low_f32x4=0 f64x2.mul=2 f64x2.add=2 f64.promote_f32=4 f64.mul=0 f64.add=0` |
| F-green | vector `add` becomes `b + self` | GREEN, 8 of 8, dev and release | GREEN, `0` on every leg | green |

F-green is the recorded expected-green row: IEEE addition is commutative bit for bit off NaN, and a
NaN result is compared only as "is a NaN", so the operand order is free.

Under F-1, F-2 and F-3 every one of the 358 corpus digest comparisons still matched its pin on all
three legs. Nothing in production calls the new surface, so the frozen corpus cannot see a defect in
it; the count exists beside the digests for the same reason `minmax_lowering_mismatches` does.

## Issue #954 — the select-free fused fader/matrix kernel

`fader_matrix_block_without_identity` is `fader_matrix_block`'s second arm with no `L::select` in
the body. Gate 1, `select_free_fused_fader_matrix_matches_both_oracles_when_no_lane_is_identity`
(`tests/fader_matrix.rs`), holds it equal to two oracles with no identity lane: oracle A is
`fader_matrix_block` with an all-false mask, oracle B is `gain_mute_block` on each plane followed by
`matrix2x2_block_without_identity`. It covers five input families (the four hostile ones and an
overflow family that a gain of up to `15.85` drives to infinity), three gain sets (mixed; unity and
zero; large), three mute sets (none, mixed, all), frame counts `[1, 3, 8, 9, 128]`, guard words
included, at `f32`, `Simd4` and `Simd8`, in dev and in release, with a negative control over a mixed
identity mask. NaN words compare as "both NaN" (amendment 4). Release reports
`[[30, 0], [456, 0], [894, 0]]` NaN-payload differences `[oracle A, oracle B]` at
`[f32, Simd4, Simd8]` and no other difference; dev reports none.

Each row was applied alone as an exact-text replacement (match count one) to `ab9bdbd9`, run in dev
and release, and restored with `git checkout`. Command form:
`cargo test --locked -p lane --test fader_matrix select_free_fused` (and with `--release`).
Host AMD EPYC 7313P, rustc 1.97.1, pin `+avx2,+fma`.

| # | mutation | gate | observed |
| --- | --- | --- | --- |
| M3 | swap `matrix.lr` and `matrix.rl` in `fader_matrix_block_without_identity` | gate 1, dev and release | FAILED, `width=1 frames=1 family=0 gains=0 mutes=0: oracle A L[1] new=c0250000 old=c0688000` |

The same mutation is red in `builtins` (the scenario gates), `builtins-compiler`
(`composite_live_sequence_…`) and `console-workload` (the metered row); those rows, and M1, M2 and
M5, are in `crates/builtins/tests/MUTATIONS.md`.

## Issue #950 — the full meter block pass

`src/kernels/builtins.rs` adds `meter_block` and `MeterBlock`. Gate M1 is `tests/meter_block.rs`
(`Simd8`, `Simd4` and `f32` against the `f32` kernel over the de-interleaved lane and an
independent scalar `ALL` oracle, 64 carried blocks at frames 1, 2, 3, 127, 128 and 129, hostile and
tone input, random positive seeds; the bit sweep of the count boundaries; and the witness that a
zero-seeded partial plus the seed is a different sum). The wasm count is
`wasm_gate_corpus::meter_block_mismatches`, run by `tools/wasm-gates` on its native, scalar-wasm and
simd128-wasm legs, and the lowering pin is `check_f64_lane_lowering`'s census of
`miso_gate_meter_block_probe` in `scripts/run-wasm-gates.sh`. Each row was applied alone as an
exact-text replacement (match count one) to `326607ce`, run, and restored with `git checkout`.
M1 in dev; the count through `cargo run --release -p wasm-gates -- --native` (and, for K-6, both
guest legs built with `run-wasm-gates.sh`'s commands); the pin by running that script. Host AMD EPYC
7313P, rustc 1.97.1, pin `+avx2,+fma`, wasmtime 47.0.3.

| # | mutation | M1 (`meter_block`, dev) | `meter_block_mismatches` | lowering pin |
| --- | --- | --- | --- | --- |
| K-1 | the energy starts from `+0.0` and the seed is added once after the loop (`seed.add(partial)`), the class-B form | RED, 2 of 3: `width 8 Hostile frames 2 block 2 lane 7: energy against the meter's serial loop` (`...022` against `...021`, one ulp); the reassociation witness (the two forms now agree) | RED, `686` on the native leg | not run |
| K-5 | the sanitized count adds `1.0 & !(a >= MIN_POSITIVE & a < INFINITY)`, which also counts both zeros | RED, 2 of 3: `width 8 Hostile frames 1 block 4 lane 5: sanitized against the meter's serial loop` (`1` against `0`); the bit sweep | RED, `1315` native | not run |
| K-6 | clipped counts `c > 1.0` | RED, 2 of 3: `width 8 Hostile frames 1 block 3 lane 3: clipped against the meter's serial loop` (`0` against `1`); the bit sweep | RED, `662` on native, wasm scalar and wasm simd128 | not run |
| M-W | the kernel widens through a `black_box`ed scalar `f64::from` per lane, then `LaneF64::load` | not run (same values) | GREEN, `0` on every leg | RED: `miso_gate_meter_block_probe is not vectorised on the simd128 leg: f64x2.promote_low_f32x4=0 ... f64.promote_f32=4`; the #949 `miso_gate_f64_lane_probe` census stays green, so the new probe is the only pin on the production kernel's lowering |

K-1, K-5 and K-6 are also red on gate M2 (`crates/builtins/tests/MUTATIONS.md`), and K-1 on gate M3
(`crates/graph/tests/MUTATIONS.md`). All 358 corpus digest comparisons stay green under every row:
the frozen digests do not run the new kernel.

## Issue #978 — the skewed (software-pipelined) cascade

`src/kernels.rs` adds `svf_cascade_skewed` and `svf_cascade_skewed_with_dry_masks` (one body,
`svf_cascade_skewed_impl`). Gate 1 is `g2_skewed_cascade_equals_the_interleaved_cascade`
(`tests/g2_kernel_identity.rs`): `f32`, `Simd4` and `Simd8`, one and two streams, depths 1 to 3,
masked and select-free, frames 1, 2, 3, 128 and 1,024, the G2 signals plus a hostile family, 40
carried blocks, outputs and integrators against `svf_cascade_interleaved[_with_dry_masks]` ("both
NaN, or equal bits"), guard words, under `CanonicalFpEnv`. Driver: one mutation at a time as an
exact-text replacement (match count one) in `src/kernels.rs`, then
`cargo test --release -p lane --test g2_kernel_identity --no-fail-fast`, restored byte for byte.
Host AMD EPYC 7313P, rustc 1.97.1, `x86-64-v3`.

| # | mutation | gate 1 | result |
|---|---|---|---|
| 978-M1 | the steady state visits the sections `0..D` ascending, so section 0 overwrites the carry section 1 was about to read | `Scalar S=1 D=2 masked=false frames=2 noise: block 0, stream 0, word 8: 0xbd32fd2c != interleaved 0x3d2f6ceb` | RED |
| 978-M2 | section `D - 1` stores to frame `i` instead of `i - (D - 1)` | the epilogue's store runs off the block (`range end index 3 out of range for slice of length 2`, `S=1 D=2 frames=2`) | RED |
| 978-M2b | the same, clamped to the block's last frame so nothing panics | `Scalar S=1 D=2 masked=false frames=2 noise: block 0, stream 0, word 8: 0x3f7a5c00 != interleaved 0x3d2f6ceb` | RED |
| 978-M3 | the epilogue deleted | `Scalar S=1 D=2 masked=false frames=2 noise: block 0, stream 0, word 9: 0xbf187800 != interleaved 0xbd57c751` (the last frame is never written) | RED |

Each is red at its first case, the scalar two-section cascade over two frames, which is the
shortest shape with a prologue, a steady state and an epilogue; the clamped M2b is added so the
corruption is shown on the bit comparison as well as on the bounds check.

## Issue #999 — the bounded cascade (the §4.4 verdict folded into the store)

`src/kernels.rs` adds `svf_cascade_interleaved_bounded` and
`svf_cascade_interleaved_with_dry_masks_bounded`: the interleaved body with a `StoreBound` observer
that folds the verdict per stream as each word is stored (attempt 1: `ok = ok AND (|y| < limit)` into
a vector mask; attempt 2: `failed |= mask_any(NOT (|y| < limit))` into a `bool`, so V8 keeps it in a
general-purpose register). Gate 1 is
`g2_bounded_cascade_is_the_cascade_and_judges_what_it_stores` (`tests/g2_kernel_identity.rs`):
`f32`, `Simd4` and `Simd8`, one and two streams, depths 1 and 2, masked and select-free, the G2
coefficients and the identity words, limits `1e30`, `1.0` and infinity, frames 1, 2, 3 and 128, 24
carried blocks of the hostile family with edge words (the bound and its neighbours, both
infinities, `NaN`, `-0.0`); outputs and integrators against the unbounded kernel ("both NaN, or
equal bits"), each stream's verdict against `check_block`'s fold over what it stored. Driver: one
mutation at a time as an exact-text replacement in `src/kernels.rs` of a scratch copy, then
`cargo test --release -p lane --test g2_kernel_identity g2_bounded`, restored between rows. Host AMD
EPYC 7313P, rustc 1.97.1, `x86-64-v3`. The same rows are also red on the EQ's gates
(`crates/parametric-eq/tests/MUTATIONS.md`, issue #999).

| # | mutation | gate 1 | result |
|---|---|---|---|
| 999-M1 | the fold ignores the second stream (`observe` updates stream 0 only; attempt 1 `within[0]`, attempt 2 `failed[0]`) | `Scalar S=2 D=1 identity=false masked=false limit=1e30 frames=1: block 2, stream 1: the folded verdict is not the scan of the stored words` (`true` against `false`) | RED |
| 999-M2 | the fold compares with `<=` (`y.abs().le(limit)`) | `Scalar S=1 D=1 identity=false masked=true limit=1e30 frames=1: block 0, stream 0: the folded verdict is not the scan of the stored words` (a dry lane stored `1e30` exactly) | RED |

Re-run on attempt 2 (the `bool` fold), each alone in a scratch copy, release: 999-M1 and 999-M2 red on
gate 1 at the same first case and message as above, and on the EQ gates as recorded in
`crates/parametric-eq/tests/MUTATIONS.md`.

## Issue #1219 — the indexed ramp (`route_mix_ramp_block`, `IndexedRamp`)

`tests/route_ramp.rs`: gate 1 is `the_kernel_is_the_indexed_ramp_law_at_every_width` and
`a_retarget_starts_from_the_exact_current_coefficients`, gate 2
`an_overflowing_difference_is_a_step_to_the_target`, gate 3 `a_settled_ramp_is_the_static_mix`.
Driver: one mutation at a time as an exact-text replacement in `src/kernels.rs` of a scratch copy
of the tree, then `cargo test --locked -p lane --test route_ramp` (debug), restored between rows.
M1 cannot change a native bit, so it was read on a scratch `cdylib` that calls
`route_mix_ramp_block::<Simd4>`, built for `wasm32-unknown-unknown` with `+simd128` and the
workspace's release profile, through `scripts/check-web-audioworklet-callgraph.py --kernel-shape`
(rule 3). Host AMD EPYC 7313P, rustc 1.97.1, `x86-64-v3`.

| # | mutation | gates red (first failure) | result |
|---|---|---|---|
| 1219-M1 | the ramp tail inlined into the generic body (`#[inline(always)]` on `route_mix_ramp_tail`) | rule 3 on the probe: `FAIL kernel ...route_mix_ramp_block...4wide6f32x4...: vector=22 scalar=42` (unmutated: 22/0). The native gates cannot see it. | RED (probe), GREEN (native) |
| 1219-M1b | the settled tail inlined (`#[inline(always)]` on `route_mix_settled_tail`) | probe reads 22/18: still passes rule 3, one edit from failing it, which is why that tail is outlined too | GREEN (recorded) |
| 1219-M2 | the settled frames mix `fma(length, step, start)` instead of `target` | gate 1 (`width 1, length 37, 128 frames from position 0, block 0: left frame 37`, `0x3ed5947d` against `0x3ed5947c`; at length 0, `+0.0` against `-0.0`), retarget, gate 3 (`case 2, length 3830`) | RED |
| 1219-M3a | `k` off by one in the vector body (`position as f32 - 1.0`) | gate 1 (`width 1, length 37 ... left frame 0`), retarget | RED |
| 1219-M3b | `k` off by one in the outlined tail (`first = position + vectored`) | gate 1 (`width 4, length 37, 128 frames from position 34 ... left frame 0`), retarget | RED |
| 1219-M4 | the snap one frame late (`ramping = length - position`) | gate 1 (`width 1, length 37 ... right frame 36`, one ulp), retarget | RED |
| 1219-M5 | `IndexedRamp::new` without its non-finite branch | gate 2 alone (`coefficients_at(0)`) | RED |
| 1219-M6 | `coefficients_at(0)` computed (`fma(0, step, start)`) instead of returning `start` | retarget alone (`coefficients_at(0)[3]`: the `-0.0` start word) | RED |
| 1219-M7 | `coefficients_at` fused (`index.mul_add(step, start)`) | retarget alone (`coefficients_at(4799)[0]`) | RED |
| 1219-M8 | every frame of a vector takes the vector's first `k` (`index = splat(position + 1)`) | gate 1 at width 4 only (`width 4, length 37 ... left frame 1`), retarget | RED |
| 1219-M9 | the ramp body swaps the left plane's coefficient roles (`ll.fma(r, lr * l)`) | gate 1, retarget | RED |
| 1219-M10 | the settled coefficients computed in `f64` and rounded once | gate 1, retarget, gate 3 (`case 2, length 3830 ... left frame 0`) | RED |
| 1219-M11 | the settled frames re-implemented as a private loop, and `mix2x2_block` then flushing its outputs (two edits: a static mix that drifts from the ramp's settled path) | gate 3 alone (`case 2, length 3830 ... left frame 3`, `-0.0` against `mix2x2_block`'s `+0.0`) | RED |
