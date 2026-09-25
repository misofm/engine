# Issue #880 MA-1 evidence

MA-1 replaces the integer square-root implementation with the standard library's IEEE 754
`f64::sqrt` and `f32::sqrt` methods. The old `u128::isqrt` implementation and its rounding rule
were moved into `scalar_accuracy.rs` as independent `software_sqrt`/`software_sqrtf` test oracles;
their algorithm is unchanged. NaN results are compared by class because target-specific sign and
payload bits are outside the determinism contract. Every non-NaN result is compared bit-for-bit.

## Oracle runs

Toolchain: `rustc 1.97.1 (8bab26f4f 2026-07-14)`. Both runs used the release test binary:

```text
cargo test --locked --release -p math --test scalar_accuracy \
  sqrtf_is_correctly_rounded_exhaustive -- --ignored --exact --nocapture
test sqrtf_is_correctly_rounded_exhaustive ... ok
test result: ok. 1 passed; 0 failed; finished in 64.59s
```

The test visits every raw `f32` bit pattern from `0x00000000` through `0xffffffff`. It records
2,139,095,042 non-NaN comparisons and 2,155,872,254 inputs where both results are NaN; all
non-NaN values matched bit-for-bit and all NaN classes agreed.

```text
cargo test --locked --release -p math --test scalar_accuracy \
  sqrt_is_correctly_rounded_large_oracle -- --ignored --exact --nocapture
test sqrt_is_correctly_rounded_large_oracle ... ok
test result: ok. 1 passed; 0 failed; finished in 2.07s
```

The large `f64` run checks 100,000,000 deterministic xorshift raw patterns against the exact
integer oracle. It starts from `0x1319_8a2e_0370_7344`, applies shifts `12`, `25`, and `27`, then
multiplies by `0x2545_f491_4f6c_dd1d` for each sample. This records the run performed for MA-1; it
does not claim the historical 1,878,905,041-comparison sample cited in the issue audit.

The default `sqrt_is_correctly_rounded` sample checks 2,000,000 xorshift raw patterns from seed
`0x243f_6a88_85a3_08d3`, including both signs. It also checks 100,000 integer perfect squares,
each square, and its four neighboring bit patterns on either side; 52 groups of subnormal edge
cases; 199,999 additional deterministic subnormals; and 30,690 inputs adjacent to squared root
midpoints (1,023 root exponents × 6 significands × 5 input neighbors). For nonnegative samples it
also compares against the host platform `sqrt`; negative samples use NaN-class comparison against
the independent oracle. The existing default `sqrtf_is_correctly_rounded` platform wiring sample
remains in place.

`cargo test --locked --release -p math` passed: 8 passed, 2 ignored. This includes
`m3_corpus_digests_match_pins`; no digest moved. The integrated wasm gate is left for the batch
owner.

## Code generation

Before and after probes compiled the exact `vendored/sqrt.rs` bodies in a tiny `#![no_std]`
wrapper that exports no-inline `math_sqrt_probe` and `math_sqrtf_probe` functions:

```rust
#![no_std]
extern crate std;
mod sqrt { include!("<vendored-sqrt.rs>"); }
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn math_sqrt_probe(x: f64) -> f64 { sqrt::sqrt(x) }
#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn math_sqrtf_probe(x: f32) -> f32 { sqrt::sqrtf(x) }
```

For the before probe, `<vendored-sqrt.rs>` was extracted from commit `6dcd9070`; for the after
probe, it was the MA-1 file. Commands used rustc 1.97.1 with `-C opt-level=3`; native also used
`-C target-cpu=x86-64-v3`:

```text
rustc --edition=2024 -C opt-level=3 -C target-cpu=x86-64-v3 \
  --crate-type=lib --emit=asm codegen_probe.rs -o x86-after-sqrt.s
rustc --edition=2024 --target wasm32-unknown-unknown -C opt-level=3 \
  --crate-type=lib --emit=asm codegen_probe.rs -o wasm-after-sqrt.s
```

The same commands with the probe's include pointing to the source extracted from commit `6dcd9070`
produced the before listings. Before, the native `f64` body was a branchy bit-decomposition and
integer-square-root routine (393 assembly lines); wasm emitted the equivalent software routine
(769 lines). After, native emits `vsqrtsd` / `vsqrtss` followed by `ret`; wasm emits `f64.sqrt` /
`f32.sqrt`. The raw listings are preserved in `/tmp/issue880-ma1/` on the task host.

MA-4 adds the forward implementation rule in the `math` crate docs: render-rate functions use
unfused `Lane` operations with M1/F1 and M2-style numeric evidence, while control-plane helpers
remain scalar. It names the sealed fast-tier precedent and links the upcoming De-esser, Dynamic EQ,
and engine-owned analysis consumers.
