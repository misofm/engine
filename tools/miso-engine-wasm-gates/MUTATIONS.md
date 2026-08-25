# Red mutations for gate G5

## Full-corpus FTZ/DAZ (issue #144 item 1, closed by issue #146)

`tests/g6_full_corpus_ftz.rs` landed with #144 as an ignored reproducer. It asserted that hardware
FTZ+DAZ is inert over the whole corpus, and it was not: the unmodified corpus had **70 divergent
rows of 331**, including raw math and feed-forward subnormal-input cases outside D7's
recursive-state law. No mutation can discriminate against an already-red baseline, so the file was
carried ignored with that blocker written down rather than by excluding rows or narrowing the claim.

Issue #146 fixed the defect at the render boundary -- every native render entry pins the canonical
floating-point environment and restores the caller's exact control word -- and the file is now a
standing gate for the true claim, with the old reproducer kept as its control arm:

| arm | caller word | entry guard | asserts |
|---|---|---|---|
| canonical | FTZ/DAZ clear | none | matches the pins |
| guarded | FTZ+DAZ set | entered | matches the pins |
| control | FTZ+DAZ set | none | **differs** from the pins, in at least `CONTROL_ARM_FLOOR` rows |

**Red mutation, applied and reverted on the delivery host.** Delete the `CanonicalFpEnv::enter()`
line from `guarded_report`, which is exactly "remove the guard at one render entry":

```
issue #146: a render entry's canonical environment did not normalise 70 of 331 comparisons under a
caller's FTZ+DAZ:
case 106 (effect/soft_clip/subnormal) at simd4: expected 275722f6… got 4fe7b59a…
case 116 (effect/gate_expander/dual_mono/subnormal) at scalar: expected 521e86ea… got de2f2560…
case 120 (builtins/input_stage/subnormal) at scalar: expected fedb98cc… got d6df8037…
case 128 (effect/true_peak_limiter/dual_mono/subnormal_release_flush) at scalar: expected 546b4a87… got af2731df…
test result: FAILED. 1 passed; 1 failed
```

The count is the #144 figure to the row: 70 of 331. The same 70 rows are what the control arm
reports every run, so the gate cannot pass by failing to set FTZ.

The `FLUSH_EPS` mutation #144 wanted is still not this file's discriminator, and now for a
different reason: the D7 flush law is the *state* law and gate `g6_ftz_inert` in
`crates/miso-engine-lane` owns its red mutation. This file's subject is the boundary.

Every mutation below was applied, run, recorded and reverted on the delivery host
(x86-64-v3, rustc 1.97.1, wasmtime 47.0.3). Master plan #83 principle 6: a gate lands together
with the one-line change that makes it fail.

The corpus is `tools/miso-engine-wasm-gate-corpus`; the runner is
`bash scripts/run-wasm-gates.sh`. Its 92 cases are this crate's own 51 lane cases plus the 32
`miso-engine-math` M3 cases and the 9 `miso-engine-effect-runtime` D1 cases, which keep their own
crates' pins.

## 1. Relaxed SIMD in the wasm `Lane::fma` (master plan §10, G5)

`crates/miso-engine-lane/src/softfma.rs`, wasm `simd128` body: replace the round-to-odd
software FMA with `f32x4_relaxed_madd`, and build the guest with
`-C target-feature=+simd128,+relaxed-simd`.

Two independent rejections, and the order matters:

* `scripts/check-lane-policy.sh` fails on the source, naming both the import and the call:
  `lane policy failure: relaxed SIMD is forbidden on every target (D3)`.
* The gate refuses the built artifact before executing a single case:
  `WebAssembly translation error … relaxed SIMD support is not enabled`, because the runner
  configures `Config::wasm_relaxed_simd(false)`.

**Recorded because it is the interesting part:** with the runner temporarily reconfigured to
*allow* relaxed SIMD, the digests still matched. Wasmtime 47 lowers `f32x4.relaxed_madd` to a
hardware `vfmadd` on this x86 host, so on *this* runtime the relaxed instruction happens to agree
with the exact software FMA. That is exactly why D3 forbids the instruction rather than testing its
result: the agreement is a property of one runtime's lowering choice, not of the program. A digest
comparison alone would have passed this mutation. Rejecting the opcode is the load-bearing check.

## 2. Unconditional `| 1` round-to-odd in the wasm `v128` body (plan hazard H1)

`softfma.rs`, wasm `simd128` body:
`let direction = v128_bitselect(i64x2_splat(-1), i64x2_splat(1), toward_zero);`
→ `let direction = i64x2_splat(1);`

```
wasm mismatch: case 48 (lane_fma) at simd4: expected 98207067…4636, got 4aa63b8d…95f7
wasm mismatch: case 48 (lane_fma) at simd8: expected 98207067…4636, got 4aa63b8d…95f7
```

Red only on the wasm leg, and only at `Simd4`/`Simd8` — the scalar width uses the scalar software
FMA, which the mutation does not touch, and no native gate executes this code at all. This is the
single clearest reason gate G5 exists: it is the only gate in the workspace that runs the `v128`
software FMA.

**This mutation was green on the first version of the corpus**, whose FMA operands were the fused/
unfused witness triples and near-total cancellations. Both families produce an *exact* `f64` sum,
so the round-to-odd adjustment never fired and the corpus could not see the direction at all. The
corpus now includes a midpoint family (`a` an odd mantissa in `[1, 4/3)`, `b = 1.5`, `c = ±2^-60`),
where the `f64` sum is inexact and the adjustment alone decides which of two neighbouring `f32`
values the demote produces. The pins were regenerated from the scalar oracle after that correction.

## 3. A moved pin

One byte of `tools/miso-engine-wasm-gate-corpus/src/lane_digests.in` set to `0x00`.

`g5_native_digests_match_pins` and `g5_idle_ramped_svf_equals_the_plain_svf` fail immediately and
name the case; the wasm leg reports the same case as a mismatch. The pins are load-bearing on both
legs, not decoration on one.

## 4. Standing checks that were red before they were satisfied

These are not applied-and-reverted mutations: they are assertions in
`tests/g5_native_corpus.rs` that failed against the corpus as first written and were fixed by
changing the corpus, not the assertion.

* `g5_case_digests_are_distinct` failed with
  `cases 'ramp_block/impulse' and 'one_pole_block/subnormal' have the same pinned digest …`.
  The gain ramp started at `0.0`, so on the impulse signal — whose only non-zero sample is the
  first frame — the case digested 8,192 zeros and would have agreed across every target and every
  width while proving nothing about `ramp_block`. The ramp now starts at `0.25`.
* `g5_no_case_is_vacuously_zero` keeps that from returning: every lane case must produce a non-zero
  value, except the one enumerated case (`one_pole_block/subnormal`) whose correct output *is*
  `+0.0` because D7 flushes a subnormal recurrence on every target.
* `g5_fma_case_separates_fused_from_unfused` compares the pinned `lane_fma` digest against the same
  case evaluated with a multiply and an add. If the operands ever stop separating the two forms,
  the wasm leg would go on being green while proving nothing, and this fails instead.
