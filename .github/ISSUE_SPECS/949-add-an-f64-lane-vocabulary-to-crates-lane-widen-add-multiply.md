# Add an `f64` lane vocabulary to `crates/lane`: widen, add, multiply

Research on `252622b6`, 2026-09-27. **Owner ruling** (2026-09-26): "We shouldn't leave
scalar arithmetic where vector arithmetic is possible." That ruling settles #943 ruling R4: `crates/lane`
gains `f64` lanes. This slice adds the vocabulary only. Its first caller is
#950, which also holds the shared research (its section "Research
findings", R1 and R5). Class A: nothing in production calls the new surface yet, so no rendered bit
and no published meter word can move.

## Product outcome

The meter's energy (RMS) sum is a scalar `f64` loop: `energy += f64::from(x) * f64::from(x)`, one
lane and one sample at a time (`crates/builtins/src/lib.rs`, `observe_segment` and
`observe_selected_segment`). It is the one ALL-metric statistic that `f32` lanes cannot bank. It
cannot be banked today, because `Lane` is `f32`-only and `crates/lane` is the only crate allowed to
name `wide`.

This slice adds the smallest `f64` surface that a lane-parallel, sample-serial energy sum needs:

- `LaneF64`: `W` lanes of `f64`, with `load`, `store`, `add` and `mul`.
- `Widen`: turns an `f32` lane value into its `W`-lane `f64` companion.

The contract is pinned per operation, as `Lane`'s is: exact IEEE binary64, never fused, with scalar
`f64` as the oracle. It is proven bit-identical at every width, natively and under wasm, and its wasm
lowering is pinned to vector opcodes.

## Root evidence

- **`Lane` cannot grow.** `Lane` (`crates/lane/src/lib.rs:139`) is `f32`-only and "deliberately
  minimal (master plan §3.1)". It is also implemented outside the crate by the test wrapper
  `Observed<L>` (`crates/builtins/src/corpus.rs:605`). A new required item on `Lane` would break that
  impl, so the `f64` surface must be separate traits.
- **What `wide` 1.6.1 offers.** Source: `~/.cargo/registry/src/*/wide-1.6.1/src/`.
  - `f64x2` is `m128d` on sse2, `v128` on wasm `simd128` (`f64x2_add`/`f64x2_mul`), `float64x2_t`
    on AArch64 NEON (`vaddq_f64`/`vmulq_f64`), and `[f64; 2]` elsewhere (`f64x2_.rs:353-404`).
  - `f64x4` is one `m256d` under `avx` and two `f64x2` otherwise (`f64x4_.rs:3-24`).
  - `f64x8` is two `f64x4` without `avx512f` (`f64x8_.rs:3-24`).
  - So a `W`-lane companion lowers to exactly the shapes wanted:
    - `Simd8` on x86-64-v3 becomes two `__m256d`.
    - `Simd4` on wasm and NEON becomes two `f64x2`.
    - `Simd4` on x86 becomes one `__m256d`.
  - `wide` has **no** `f32`-to-`f64` conversion. Grepping `f64x*_.rs` for
    `f32x4|f32x8|cvtps|promote`, and `f32x4_.rs`/`f32x8_.rs` for `f64x2|f64x4|cvtps_pd`, finds
    nothing.
- **`wide`'s `mul_add` is fused on some targets.** `f64x4::mul_add` is `fused_mul_add_m256d` under
  `avx`+`fma` (`f64x4_.rs:763`), and NEON fuses too. It must never be forwarded, which is the same
  rule `Lane::fma` follows. The `+`/`*` operators are `vaddpd`/`vmulpd`, `f64x2.add`/`f64x2.mul`
  and `fadd`/`fmul`.
- **Rust never contracts.** rustc emits no `contract` fast-math flag and no `llvm.fmuladd` for
  `a * b + c`. The `+fma` pin therefore makes the instruction available but never licenses fusing a
  separate multiply and add.
  - Measured below: no `vfmadd*pd` in the fat-LTO kernel.
  - Relaxed SIMD (`f64x2_relaxed_madd`) is already refused by `scripts/policies/lane-source.toml`
    rule `relaxed`.
- **Every `f32` is exactly representable in binary64**, subnormals and infinities included. So
  widening is exact for every non-NaN input.
  - A NaN widens to *a* NaN. Its payload is not pinned: wasm `f64.promote_f32` is nondeterministic on
    NaN, and x86 quiets signalling NaNs.
  - On x86, DAZ would flush subnormal inputs of `vcvtps2pd`. Every native render entry installs
    `CANONICAL_MXCSR`, with DAZ and FTZ clear (`crates/lane/src/fpenv.rs`), and AArch64 FPCR is
    cleared too. wasm has no flush mode.
- **Measured lowering** (throwaway probe; `x86-64-v3`; release profile, which is fat LTO).
  - x86: the portable spelling `f64x8::new(self.to_array().map(f64::from))` becomes two
    `vcvtps2pd` plus one `vextractf128` per `Simd8` value.
  - x86: `mul`/`add` become `vmulpd`/`vaddpd` on `ymm`, with zero `vcvtss2sd` and zero `vfmadd`.
  - wasm `simd128`: the same source becomes `f64x2.promote_low_f32x4`, `f64x2.mul` and
    `f64x2.add`, with zero scalar `f64.*`.
  - **The rlib assembly is misleading.** Cargo builds workspace rlibs with `-C linker-plugin-lto`,
    and that pre-link pipeline runs no SLP vectorizer. `--emit asm` on the rlib therefore shows eight
    scalar `vcvtss2sd`. Always census the linked artifact.
  - AArch64 was not measured: the research host has no aarch64 standard library.
- **Measured exactness** (throwaway harnesses; all of them reverted).
  - **Widen, native.** All 2^32 `f32` bit patterns at `Simd4` and `Simd8`, against an independent
    integer-construction oracle: 0 mismatches.
  - **Widen, under V8** (node 22, `simd128`). All 2^32 patterns: 0 mismatches against in-module
    `f64::from` at both widths, and 0 against a JS exact-conversion oracle (`Float32Array` to
    `Float64Array`).
  - **Carried chains, under V8.** `e = e + widen(x) * widen(x)`, 20,000 chains × 256 steps at both
    widths, with random finite `f32` inputs (extremes included) and random positive seeds: 0
    mismatches.
- **Measured hazard: a native identity test can be vacuous.** `widen` is literally `fpext`, the same
  instruction `f64::from` is, so LLVM folds `widen(x) == f64::from(x)` to `true`. The exhaustive
  2^33-comparison test finished in **5 ms**. With `core::hint::black_box` on the vector result and
  an independent oracle, it took 9.7 s and was real.

## Smallest closable slice

Authorized paths:

- `crates/lane/src/f64_lane.rs` (new) and `crates/lane/src/lib.rs` (module, re-exports, one doc
  paragraph).
- `crates/lane/tests/f64_lane.rs` (new) and `crates/lane/tests/MUTATIONS.md`.
- `tools/wasm-gate-corpus/src/lib.rs`, `tools/wasm-gate-guest/src/lib.rs`,
  `tools/wasm-gates/src/lib.rs`, `tools/wasm-gates/src/main.rs` and `tools/wasm-gates/MUTATIONS.md`.
- `scripts/run-wasm-gates.sh`, for the lowering pin only.
- This spec.

1. **The traits** (`crates/lane/src/f64_lane.rs`, re-exported as `lane::{LaneF64, Widen}`):

   ```rust
   pub trait LaneF64: Copy + Send + Sync + 'static {
       const WIDTH: usize;
       fn load(src: &[f64]) -> Self;      // first WIDTH values; panics if shorter (debug aid)
       fn store(self, dst: &mut [f64]);   // first WIDTH values; panics if shorter
       fn add(self, b: Self) -> Self;     // IEEE binary64 round-to-nearest-even, per lane
       fn mul(self, b: Self) -> Self;     // IEEE binary64 round-to-nearest-even, per lane
   }
   pub trait Widen: Lane {
       type F64: LaneF64;                 // F64::WIDTH == Self::WIDTH, lane i <-> lane i
       fn widen(self) -> Self::F64;       // exact per lane for every non-NaN input
   }
   ```

   Nothing else goes in. There is no `sub`, `div`, `sqrt`, compare, select, `splat` or fused
   operation. Each future need is its own owner-visible addition.
2. **Implementations.** Every method is `#[inline(always)]`.
   - `LaneF64 for f64`, `WIDTH = 1`: the oracle, written as `self + b` and `self * b`, with `src[0]`
     and `dst[0]`.
   - `LaneF64 for wide::f64x4` (4) and `wide::f64x8` (8): one macro, in the same style as
     `wide_impl.rs`.
     - `load` copies into `[f64; W]` and calls `new`. `store` uses `to_array`.
     - `add` and `mul` forward the `+` and `*` operators.
     - `mul_add`, `mul_sub`, `mul_neg_add` and `mul_neg_sub` are never called.
   - `Widen for f32` (`F64 = f64`, `f64::from(self)`), `for wide::f32x4` (`F64 = wide::f64x4`) and
     `for wide::f32x8` (`F64 = wide::f64x8`).
     - The two vector impls are exactly `<F64>::new(self.to_array().map(f64::from))`.
     - Do not use `core::arch`: the lane policy confines it to `softfma.rs`/`fpenv.rs`.
3. **Docs.** Put the numeric contract on the traits:
   - Widen is exact for non-NaN inputs; a NaN input gives some NaN.
   - Add and multiply are exact IEEE binary64 round-to-nearest-even; a NaN result's payload is
     unspecified.
   - There is no fusion, for the same two reasons `Lane::fma` gives: Rust never contracts, and
     `wide`'s `mul_add` is not forwarded.
   - The environment dependence is the one `wide_impl.rs` "The one precondition" records for `f32`:
     native render pins DAZ/FTZ clear through `fpenv`.
   - The vector lowering is a codegen outcome (SLP at fat LTO). Correctness never depends on it,
     and gate 5 pins it.

   Add one paragraph to `lib.rs`'s crate docs. It names the owner ruling and this issue, and says
   the `f32` `Lane` contract is unchanged.
4. **wasm differential** (`tools/wasm-gate-corpus`). Add `pub fn f64_lane_mismatches(width: usize)
   -> u32`, modeled line for line on `minmax_lowering_mismatches` (`lib.rs:566`): a count, with no
   pin. At width 0 (`f32`), 1 (`Simd4`) and 2 (`Simd8`) it counts the lanes that disagree with the
   scalar oracle over three sets:
   - **(a) widen.** Every 65,537th `f32` bit pattern, plus a directed pool:
     - `±0.0`, the smallest and largest subnormal, `±MIN_POSITIVE`, `±1.0`, `1 ± ulp`, `±MAX`,
       `±inf`, and two NaN payloads;
     - a non-NaN input must match the independent oracle of gate 1 bit for bit;
     - a NaN input must give a NaN.
   - **(b) `add` and `mul`.** Every ordered pair of a directed `f64` pool (`±0.0`, `±` the smallest
     and largest subnormal, `±MIN_POSITIVE`, `±1.0`, `1 + 2^-52`, `±MAX`, `±inf`, a tie-rounding
     pair), plus 4,096 seeded random pairs, against scalar `+`/`*`. A non-NaN result must match bit
     for bit; a NaN result must be a NaN.
   - **(c) the energy shape.** 64 chains × 256 steps of `e = e.add(w.mul(w))` with `w =
     widen(x)`, on random finite `f32` of any magnitude and random positive seeds.

   Pass every vector result through `core::hint::black_box` before comparing it, or the native leg is
   vacuous (Root evidence). The whole function must run in well under a second on the scalar wasm
   leg.
5. **Guest exports and the lowering pin.**
   - `tools/wasm-gate-guest` exports `miso_gate_f64_lane_mismatches(width: u32) -> u32`, like
     `miso_gate_minmax_lowering_mismatches` (`lib.rs:97`).
   - It also exports `#[inline(never)] miso_gate_f64_lane_probe(seed: u32) -> u32`. The probe:
     - fills `[f32; 4 * 64]` from an xorshift of `seed`, masking the bits with `& 0xBFFF_FFFF` so
       every value is finite;
     - runs `e = e.add(w.mul(w))` with `w = Simd4::load(frame).widen()` over the 64 frames;
     - returns a fold of `e`'s bits.
   - In `scripts/run-wasm-gates.sh`, add `check_f64_lane_lowering`, modeled on
     `check_detector_residency` (`:65-83`). It runs on the **simd128 leg only** and fails unless the
     probe's body contains `f64x2.promote_low_f32x4`, `f64x2.mul` and `f64x2.add`, and none of
     `f64.promote_f32`, `f64.mul` or `f64.add`. This probe shape was prototyped. Its census was two of
     each vector op and zero scalar `f64`.
6. **Host plumbing** (`tools/wasm-gates`). Do this exactly as `minmax_lowering_mismatches` is done:
   - a `Guest` field, the loader's `get_typed_func` and a method summing the three widths;
   - a `Report` field, and a JSON key `f64_lane_mismatches` placed after
     `minmax_lowering_mismatches`;
   - on the native leg, `(0..corpus::WIDTHS).map(corpus::f64_lane_mismatches).sum()`;
   - in `main.rs`, both legs fail on a nonzero value, with a diagnostic naming
     `crates/lane/src/f64_lane.rs`.

## Non-goals

- Any production caller: the meter pass is #950.
- Any change to `Lane`, `Simd4`/`Simd8`, `wide_impl.rs`, a kernel, a digest or a pin.
- `core::arch` conversion intrinsics, or any change to `scripts/policies/lane-source.toml`.
- AArch64 codegen evidence. It is unmeasured. Record a census only if an aarch64 toolchain is at
  hand, and never as a gate.
- A native vectorization-report probe (`tools/audit/src/vectorization.rs`). That is a named
  follow-up: the report is nightly and non-blocking, and gate 5 is the required pin.
- Adding the `_pd`/`_f64` fused spellings to `check-unfused-seal.sh`'s `call_pattern`. The seal
  already catches `mul_add(`, and `core::arch` is confined to two files, so no gap opens. That is a
  one-line hardening follow-up.

## Objective gates

1. **Widen identity** (`crates/lane/tests/f64_lane.rs`).
   - At `Simd4` and `Simd8`, compare every lane of `widen` with an independent oracle that builds the
     binary64 bits with integer arithmetic from the `f32` fields. Take `bits = x.to_bits()`,
     `s = u64::from(bits >> 31) << 63`, `e = (bits >> 23) & 0xFF` and `m = bits & 0x7F_FFFF`:
     - zero (`e == 0 && m == 0`): `s`;
     - subnormal (`e == 0`, `m != 0`): let `k = m.leading_zeros() - 9` on the `u32`. The bits are
       `s | (896 - k) << 52 | ((m << (k + 1)) & 0x7F_FFFF) << 29`.
     - normal (`0 < e < 255`): `s | (e + 896) << 52 | m << 29`;
     - infinity (`e == 255`, `m == 0`): `s | 0x7FF << 52`;
     - NaN: the result must be a NaN.

     This oracle was run in the research: 0 mismatches over all 2^32 patterns at both widths.
   - Store each vector result and pass it through `black_box` first.
   - Release sweeps all 2^32 patterns. Debug sweeps every 65,537th pattern plus the directed pool.
   - Record the release run time in the evidence. A sweep the compiler folded away measured 5 ms, and
     a real one measured 9.7 s on two threads, so the time shows the vector instruction ran.
   - Also assert `<L as Widen>::F64::WIDTH == L::WIDTH` at both widths, and cover `Widen for f32`.
2. **`add`/`mul` identity.** Take gate 4(b)'s pool, all ordered pairs, plus seeded random pairs
   (1,000,000 in release, 20,000 in debug). Compare `f64x4` and `f64x8` against scalar `+`/`*`, bit
   for bit where the result is not NaN. Vector results go through `black_box`.
3. **Square exactness witness.** For every `f32` `x` in the directed pool (`MAX`, `MIN_POSITIVE`, both
   subnormal extremes, `1 ± ulp`, `2^-34..2^5`) and 100,000 seeded finite values, check
   `widen(x).mul(widen(x))` bit for bit against the integer oracle.
   - The oracle is `((M*M) as f64) * f64::from_bits(((2E + 1023) as u64) << 52)`, where `x = ±M ·
     2^E`.
     - A normal `x` has `M = m | 0x80_0000` and `E = e - 150`.
     - A subnormal has `M = m` and `E = -149`.
     - `M*M` is below `2^48`, so the conversion is exact, and `2E` is in `[-298, 208]`, so the power
       of two is a normal `f64` and the product is exact.
   - This is the premise the banked meter's class-A argument rests on: the square is exact, so only
     the order of the additions can move a bit.
4. **wasm differential.** `bash scripts/run-wasm-gates.sh` must pass, with `f64_lane_mismatches: 0` on
   the native, scalar-wasm and simd128-wasm legs. The evidence JSONL carries the new key.
5. **wasm lowering pin.** `check_f64_lane_lowering` passes on the simd128 leg. Record its census, and
   the native census of a release build of the same probe run locally (`vcvtps2pd`, `vmulpd`,
   `vaddpd`; no `vcvtss2sd`, no `vfmadd`), in the evidence.
6. **Suites and policy.**
   - `cargo fmt --all --check`.
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`.
   - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
   - `cargo test -p lane`, both debug and `--release`.
   - `cargo test -p wasm-gates`.
   - `scripts/check-lane-policy.sh`, `scripts/check-unfused-seal.sh` (and `--self-test`) and
     `scripts/check-workspace-policy.sh`.
7. **Red mutations**, each applied alone and recorded in the named `MUTATIONS.md`:

   | # | mutation | must go red |
   |---|---|---|
   | F-1 | `Widen for f32x8` puts lanes 4..8 first | gates 1 and 4 |
   | F-2 | `widen` maps subnormal inputs to `+0.0` | gates 1 and 4 (subnormal rows only) |
   | F-3 | vector `add` becomes `(self + b) + zero` | gates 2 and 4 (`-0.0 + -0.0`) |
   | F-4 | the probe widens through a `black_box`ed scalar loop | gate 5 (`f64.promote_f32` appears) |

   Record as expected green: vector `add` becomes `b + self`, which is bitwise the same off NaN.

## Console benchmark rows

None can move: nothing renders through the new surface. Do not run the console benchmark for this
issue.

## Dependencies

- None.
- #950 depends on this issue.

## Standing rules for the implementer

- Work only from this body. Read `wide_impl.rs`, `scalar.rs`, `lib.rs`, and the
  `minmax_lowering_mismatches` plumbing first. Do not survey the workspace.
- Only `crates/lane` names `wide`. No `core::arch`, no `mul_add`, no relaxed SIMD.
- Every "bit-identical" or "0 mismatches" gate is a hard stop, not a tolerance.
- Commit on `codex/<issue>-<slug>` from synchronized `main`. Do not quote a speed-up: this slice has
  no caller.
- Public docs must not link to private items (rustdoc runs with `-D warnings`).

## What the implementer will hit

- **Vacuous native tests.** `widen` and `f64::from` are the same IR instruction, and
  `extractelement` of a vector `fmul` scalarizes, so LLVM can prove a vector-versus-scalar comparison
  true and delete the loop. Measured: 2^33 comparisons in 5 ms. `black_box` every vector result and
  use the independent integer oracle for widen.
- **Misleading rlib assembly.** `cargo rustc --emit asm` on `lane` shows scalar `vcvtss2sd`, because
  Cargo passes `-C linker-plugin-lto` and the pre-link pipeline skips SLP. Census a linked release
  binary or the wasm guest, never the rlib.
- **Case indices.** `wasm-gate-corpus` indexes pinned digests by case number. This slice adds a
  count function, not a case, so no index or pin moves. Do not add a digest case.
- **`f64x8` alignment.** `f64x8` is `#[repr(C, align(64))]`. Load and store through `[f64; 8]`
  copies, as `wide_impl.rs` does for `f32`. Never transmute a slice.
- **The scalar wasm leg.** It builds with `-simd128`, and there `wide`'s `f64x2` is an array. The
  lowering pin applies to the simd128 leg only; the count applies to both.
- **Lane policy.** The policy's marker rule flags `.max(`, `.min(`, `.mul_add(`, `.sqrt(` and
  `f64::sqrt(` in `crates/lane/src`, but not `f64::from(`. If you write any flagged call, it needs a
  `LANE-OP-OK` marker, and none should be needed.
