# Compute the EQ elision gate in its min/max form

EQ optimisation, slice 5 (research 2026-09-27, base `6ca203f8`). Evidence:
`docs/handoffs/effects-2026-09-27/EQ-DIAGNOSIS.md`, sections 1 and 5 (EQ-5); the equivalence and
timing harness is `gate_forms` in `docs/handoffs/effects-2026-09-27/eq-diagnosis-prototypes.patch`
(`tools/console-workload/tests/eq_rows.rs`).

## Product outcome

`block_admits_elision` (`crates/parametric-eq/src/lib.rs:997`) scans both input planes of every
stationary bank block:

```rust
rejected |= u32::from(bits == NEGATIVE_ZERO_BITS);
rejected |= u32::from((bits & MAGNITUDE_MASK) > ELISION_MAGNITUDE_CEILING);
```

LLVM turns each `u32::from(bool)` into a compare plus a shift, so the loop is 7 integer vector
operations per vector of words. It costs 0.22-0.26 cycles per lane-sample natively and 0.71 in the
browser build: 3.2 us of a 64-track block in V8, the largest non-kernel term of the EQ after the
kernel itself.

The same predicate in min/max form is 4 operations per vector:

```rust
nearest = nearest.min(bits ^ NEGATIVE_ZERO_BITS);   // 0 exactly when some word is -0.0
largest = largest.max(bits & MAGNITUDE_MASK);
// admitted iff nearest != 0 && largest <= ELISION_MAGNITUDE_CEILING
```

Replica per two 1,024-word planes: native `Simd8` 431 -> 309 cycles (-28 %), V8 617 -> 402
(-35 %). In process, per 64-track block: wasm -1.0 us, native `Simd8` -0.1 to -0.25 us, `Simd4`
-0.1 us.

## Lessons carried from #944

Dev and release; the equivalence gate below is exhaustive over its bit patterns; mutations recorded
red in `crates/parametric-eq/tests/MUTATIONS.md`.

## Invariants

- **Class A.** The predicate's value is unchanged on every input, so the elision decision, and with
  it every rendered and integrator word, is unchanged.
- `u32::min`/`u32::max` on bit patterns only; no float compare (a float compare would treat `-0.0`
  as `+0.0` and order NaNs away).
- Allocation-free; the function stays scalar source that LLVM vectorises (`i32x4.min_u`/`max_u` in
  `simd128`, `vpminud`/`vpmaxud` on x86).

## Interface contract

`block_admits_elision(io: &[f32]) -> bool` keeps its signature and doc; its body becomes the min/max
form above, and the doc states why `nearest == 0` is exactly "some word is `-0.0`" (`x ^ c == 0`
iff `x == c`) and why `largest <= ceiling` is exactly "every magnitude is finite and within
`BLOCK_LIMIT`" (the existing argument about monotone magnitude bits).

## Smallest closable slice

Authorized paths: `crates/parametric-eq/src/lib.rs` (`block_admits_elision` and the `elision` test
module); `crates/parametric-eq/tests/MUTATIONS.md`; this spec.

## Non-goals

Removing the gate, fusing it with `block_is_positive_zero` or with the previous stage's scan,
changing the ceiling.

## Objective gates

1. **Equivalence (in-crate, exhaustive).** Keep the old body as a `#[cfg(test)]` oracle. For every
   ordered pair `(a, b)` from the edge set `{0, 0x8000_0000, 1, 0x8000_0001, CEILING, CEILING + 1,
   0x7f80_0000, 0xff80_0000, 0x7fc0_0000, 0xffc0_1234, 0x3f80_0000, 0xbf80_0000,
   CEILING | 0x8000_0000, (CEILING + 1) | 0x8000_0000, 0x7fff_ffff, 0xffff_ffff}`, planted at two
   positions of a block of ordinary words, and for block lengths `{0, 1, 7, 8, 9, 1024}`, the new
   body equals the oracle. Dev and release.
2. The existing refusal tests stay green unchanged: `a_negative_zero_input_refuses_elision`,
   `a_non_finite_or_oversized_input_refuses_elision`, `the_negative_zero_refusal_is_load_bearing`.
3. `cargo test -p parametric-eq`, dev and release; every `WORKLOADS` digest unchanged.
4. **Mutations**, each alone, red:
   - M1: `nearest.min(bits)` (no xor): gate 1 (a block holding `+0.0` is refused).
   - M2: `largest < ELISION_MAGNITUDE_CEILING`: gate 1 (`CEILING` itself is admitted today).
   - M3: drop the `& MAGNITUDE_MASK`: gate 1 (a negative ordinary word is refused).
5. **Browser artifact.** As EQ-1 gate 5; record the loop's `i32x4.min_u`/`max_u` in
   `wasm-objdump -d`.
6. Toolchain and policy scripts as EQ-1 gate 7.

## Console benchmark rows

- **Can move** (down, slightly): every row carrying the EQ, most visibly the wasm `eq_only`.
- **Must not move:** rows without the EQ.

## Dependencies

None.

## Standing rules for the implementer

As EQ-1.

## What the implementer will hit

- **The inversion is easy to get backwards.** Today's loop accumulates *rejections*; this one
  accumulates two witnesses and decides at the end. Gate 1's oracle is the guard.
- **Do not reach for `Lane`.** The gate reads raw bits and `Lane` has no integer lanes; plain `u32`
  arithmetic is what vectorises here.
