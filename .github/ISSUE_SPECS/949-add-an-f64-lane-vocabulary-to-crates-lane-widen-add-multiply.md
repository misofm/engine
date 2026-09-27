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

## Attempt 1 evidence

Implementer: attempt 1, 2026-09-27. Branch `codex/949-f64-lane-vocabulary`, base `edc9273a` (the
#943 branch tip plus this brief). Code commit `6d8867e9`; the follow-up commit carries this record.
Host AMD EPYC 7313P (Zen 3), rustc 1.97.1, `.cargo/config.toml` pin `+avx2,+fma`, wasmtime 47.0.3,
wabt `wasm-objdump` 1.0.34, node 22.23.2, `CARGO_INCREMENTAL=0`, the worktree's own `target/`. The
console benchmark was not run, and no speed-up is quoted: this slice has no caller.

### Design

- **Traits** (`crates/lane/src/f64_lane.rs`, re-exported as `lane::{LaneF64, Widen}`): exactly the
  brief's two signatures. `LaneF64: Copy + Send + Sync + 'static` with `WIDTH`, `load`, `store`,
  `add` and `mul`; `Widen: Lane` with `type F64: LaneF64` and `widen`. No other operation.
- **Implementations**, every method `#[inline(always)]`:
  - `LaneF64 for f64` (`WIDTH = 1`, `src[0]`, `dst[0]`, `self + b`, `self * b`): the oracle.
  - `LaneF64 for wide::f64x4` and `wide::f64x8` from one macro, `impl_lane_f64_for_wide!`, in
    `wide_impl.rs`'s style: `load` copies into `[f64; W]` and calls `new`, `store` uses `to_array`,
    and `add`/`mul` forward `+`/`*`. None of `wide`'s fused spellings is called.
  - `Widen for f32` (`f64::from`), `for wide::f32x4` and `for wide::f32x8`, the vector ones exactly
    `<F64>::new(self.to_array().map(f64::from))`. No `core::arch`.
  - Three `const _: () = assert!(...)` lines pin `F64::WIDTH == Self::WIDTH` at compile time. They
    are not trait surface.
- **Docs.** The numeric contract sits on the traits: widen is exact off NaN and a NaN gives some
  NaN; add and mul are IEEE binary64 round-to-nearest-even with the NaN payload unspecified; there
  is no fusion, for `Lane::fma`'s two reasons; the environment dependence is `wide_impl.rs`'s
  "The one precondition", met by `fpenv`. The module docs cover what is and is not forwarded, the
  SLP-at-fat-LTO lowering and its pin, and the rlib-assembly trap. `lib.rs` gains a `# f64 lanes`
  paragraph naming the 2026-09-26 owner ruling and #949, and saying the `f32` `Lane` contract is
  unchanged.
- **Tests** (`crates/lane/tests/f64_lane.rs`). Gates 1 to 3 are as briefed, using the brief's
  integer-construction widen oracle and square oracle. Every vector result is stored through
  `black_box`, and the output array is `black_box`ed again before it is compared.
  - A known-value test checks the widen oracle against hand-derived bits before the oracle is
    trusted.
  - A tie test proves on scalar `f64` that the pool's rounding members produce exact ties:
    `1 + 2^-53`, `(1 + 2^-52) + 2^-53`, `2^53 + 1` and `1.5 * (1 + 2^-52)`.
- **wasm differential** (`tools/wasm-gate-corpus`). `pub fn f64_lane_mismatches(width) -> u32` is
  modeled on `minmax_lowering_mismatches`: a count with no pin and no case index, so no digest index
  moves. It runs sets (a), (b) and (c) at `f32`, `Simd4` and `Simd8` through `<L as Widen>::F64`,
  and the tools crates never name `wide`.
- **Guest** (`tools/wasm-gate-guest`): `miso_gate_f64_lane_mismatches(width)` and
  `#[inline(never)] miso_gate_f64_lane_probe(seed)`, exactly the brief's probe shape.
- **Host** (`tools/wasm-gates`): a `Guest` field, the loader's `get_typed_func` and a summing method;
  a `Report` field, and the JSON key `f64_lane_mismatches` after `minmax_lowering_mismatches`. The
  native leg is `(0..corpus::WIDTHS).map(corpus::f64_lane_mismatches).sum()`. In `main.rs` both legs
  fail on a nonzero count, with `report_f64_lane` naming `crates/lane/src/f64_lane.rs`.
- **Lowering pin** (`scripts/run-wasm-gates.sh`, `check_f64_lane_lowering`). It is modeled on
  `check_detector_residency` and runs on the simd128 guest only. It takes the census inside the
  `func[N] <miso_gate_f64_lane_probe>:` body of `wasm-objdump -d`, and it fails:
  - if the probe is not found exactly once;
  - if any of `f64x2.promote_low_f32x4`, `f64x2.mul` or `f64x2.add` is missing;
  - if any of `f64.promote_f32`, `f64.mul` or `f64.add` is present.

  It prints the census on stdout, not in the JSONL, so every evidence line keeps one schema.

### Deviations and additions (none weakens a gate)

1. **Pools are supersets.**
   - The widen pool adds a third NaN, the signalling `0x7F80_0001`, and `-(1 ± ulp)`.
   - The binary64 pool adds two NaN payloads and `2^53`, which with `1.0` makes a third additive
     tie.
   - The tie-rounding "pair" is carried by `2^-53` and `1.5` against `1.0` and `1 + 2^-52`.
2. **Gate 2's random pairs are two families.** Half are arbitrary bit patterns, every class
   including NaN. The other half have exponents within 60 binades of each other, so the addition
   actually rounds instead of returning the larger operand. The wasm set (b) keeps the brief's
   4,096 arbitrary pairs.
3. **Extra widths and inputs.**
   - The release run of gate 1 also sweeps `Widen for f32` over all 2^32 patterns; the brief asked
     only to cover it.
   - Gate 3 also runs at `f32` and adds both zeros and the negative powers.
   - Every list is read at every lane rotation, so each directed value sits in every lane.
4. **Set (c) compares at every step**, not only at the end of each chain, so a one-ulp divergence
   that a later large term would absorb is still counted. The seeds are random positive values in
   `[2^-64, 2^65)`. The inputs are random finite `f32` of every magnitude: the exponent field is
   redrawn from `0..=254`, so subnormals are included.
5. **`check_f64_lane_lowering` also fails if the probe is missing.** Without that clause, a missing
   export would census as zero of everything and fail only on the "vector op absent" clause, with a
   misleading message.

### Gates

| # | gate | command | result |
|---|---|---|---|
| 1 | widen identity | `cargo test --locked --release -p lane --test f64_lane -- --nocapture` | PASS. All 2^32 patterns, 0 mismatches at every width: `f32` 3.78 s, `Simd4` 2.09 s, `Simd8` 1.78 s on 4 threads (a folded sweep measured 5 ms). Dev: sparse sweep plus pool at every rotation, 0 mismatches. Width assertions hold at both widths and for `f32`. |
| 2 | `add`/`mul` identity | same binary; the dev run too | PASS. Every ordered pair of the 20-value pool at every rotation, plus 1,000,000 random pairs in release and 20,000 in dev, at `f64x4` and `f64x8` (the pool also at `f64`): 0 mismatches. The tie test passes. |
| 3 | square exactness witness | same binary | PASS. Pool plus 100,000 seeded finite values at `f32`, `Simd4` and `Simd8`: 0 mismatches against `(M*M) * 2^(2E)`. |
| 4 | wasm differential | `bash scripts/run-wasm-gates.sh` | PASS. `f64_lane_mismatches: 0` on the native, scalar-wasm (backend 0) and simd128-wasm (backend 1) legs, in `target/ci/wasm-gates/wasm-gates.jsonl` after `minmax_lowering_mismatches`. All 358 digest comparisons match their pins on every leg. Under V8 the export takes 0.4 to 1.8 ms per width on both guests, well under a second. |
| 5 | wasm lowering pin | same script | PASS. `f64x2.promote_low_f32x4=2 f64x2.mul=2 f64x2.add=2 f64.promote_f32=0 f64.mul=0 f64.add=0`, the prototype's census exactly. The native census is below. |
| 6 | suites and policy | see below | PASS |

Gate 6, run on the committed tree:

- `cargo fmt --all --check`: clean.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean. The first
  pass flagged a `f64::powi` (disallowed by D6) in a test assertion, a manual `is_multiple_of` and
  an assign-op; all three were fixed.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`: clean.
  `lane/trait.LaneF64.html` and `trait.Widen.html` render.
- `cargo test --locked -p lane`, dev and `--release`: every binary passes, `f64_lane` 8 of 8.
- `cargo test --locked -p wasm-gates`: 7 plus 2 passed, in 1 min 27 s.
- `scripts/check-lane-policy.sh`, `scripts/test-lane-policy.sh`, `scripts/check-realtime-policy.sh`
  (56 marked regions in 16 files), `scripts/check-workspace-policy.sh` and
  `scripts/check-unfused-seal.sh`: all ok. `--self-test` gives 62 passed, 0 failed.

### Contraction evidence (no fusion)

- **Native guest, linked fat-LTO module.**
  - Command: `cargo rustc --release -p wasm-gate-guest --lib -- --emit=llvm-ir,link`. The emitted
    module is the merged one, with 513 definitions.
  - It has 0 floating-point instructions carrying `contract`, `fast` or `reassoc`
    (`grep -E '= (fadd|fmul|fsub|fdiv|frem|fneg|call)( [a-z]+)* (contract|fast|reassoc)\b'`), 0
    `llvm.fmuladd` and 0 `llvm.fma.*`.
  - A bare `grep contract` finds 5 hits. All of them are the `effect-contract` file names, not
    flags.
  - The probe's IR is `fpext <4 x float> %10 to <4 x double>`, `fmul <4 x double> %11, %11` and
    `fadd <4 x double> %energy, %12`.
- **wasm simd128 guest** (same command, `--target wasm32-unknown-unknown`,
  `RUSTFLAGS=-Ctarget-feature=+simd128`). It has 0 flagged instructions, 0 `fmuladd` and 0
  `llvm.fma`. The probe's IR is two `fpext <2 x float> to <2 x double>`, two `fmul <2 x double>`
  and two `fadd <2 x double>`.
- **`lane` release test binary** (`cargo rustc --release -p lane --test f64_lane -- --emit=llvm-ir,link`).
  - IR: 0 flagged instructions, 0 `fmuladd` and 0 `llvm.fma`, with 9 vector `fpext`, 11
    `fmul <4 x double>` and 8 `fadd <4 x double>`.
  - Linked binary: 0 `vfmadd` anywhere (`objdump -d | grep -c vfmadd`).
  - The exhaustive sweeps: the `Simd8` closure holds 2 `vcvtps2pd`, `Simd4` holds 1, and `f32`
    holds 1 `vcvtss2sd`, which is the scalar arm the sweep is meant to exercise.
- **Native probe census.**
  - Build: `cargo build --release -p wasm-gate-guest` natively, which gives
    `target/release/libwasm_gate_guest.so`, a linked cdylib at fat LTO.
  - The loop body of `miso_gate_f64_lane_probe` is `vcvtps2pd 0x8(%rsp,%rax,4),%ymm1`,
    `vmulpd %ymm1,%ymm1,%ymm1` and `vaddpd %ymm1,%ymm0,%ymm0`: 1 of each on `ymm`.
  - It has 0 `vcvtss2sd`, and 1 `vextractf128` in the final fold.
  - It has 0 `vfmadd`, and so does the whole 5.7 MB `.so`.
- **Cross-target value.** `miso_gate_f64_lane_probe(1)` is `967085629` natively (called through
  `ctypes`) and in both wasm guests under V8.

### wasm opcode findings

- The simd128 probe body is 2 `f64x2.promote_low_f32x4`, 2 `f64x2.mul` and 2 `f64x2.add`, with no
  scalar `f64.*` at all.
  - The rest of the body is the xorshift fill (`i32.*`, `memory.fill`), 1 `v128.load` and 1
    `v128.const` (the 0.5 seed), 1 `i8x16.shuffle`, and 8 `i32x4.extract_lane` for the fold.
  - Two of each `f64x2` op is the expected shape: `Simd4`'s companion is two `f64x2` halves.
- Under F-3, `(self + b) + 0.0`, the probe census is unchanged. LLVM folds the `+ 0.0` in the probe,
  because `e + w*w` can never be `-0.0` there. That fold is why gate 4, and not gate 5, is F-3's
  discriminator.

### Red mutations

Each mutation was applied alone, run, and reverted with a `cmp`-clean restore. The full rows are in
`crates/lane/tests/MUTATIONS.md` ("Issue #949"), and the wasm-leg lines are in
`tools/wasm-gates/MUTATIONS.md`.

| # | mutation | observed |
|---|---|---|
| F-1 | `Widen for f32x8` puts lanes 4..8 first | RED. Gate 1 at `Simd8` (522,496) and gate 3 at `Simd8` (100,096). Gate 4: 81,676 on native, scalar wasm and simd128 wasm. |
| F-2 | vector `widen` maps subnormal inputs to `+0.0` | RED. Gate 1 at `Simd4` (1,047) and gate 3 (369). Gate 4: 518 on every leg, which is exactly the widen set's subnormal rows: 255 sparse patterns plus 4 pool entries, at the two vector widths. |
| F-3 | vector `add` becomes `(self + b) + 0.0` | RED. Gate 2 pool (4, the `-0.0 + -0.0` pair at four rotations). Gate 4: 2 on every leg. |
| F-4 | the probe widens through a `black_box`ed scalar loop | RED. Gate 5: `f64x2.promote_low_f32x4=0 … f64.promote_f32=4`. Gate 4 stays 0, as it should. |
| F-green | vector `add` becomes `b + self` | GREEN in gates 1 to 3 (dev and release) and gate 4 (0 on every leg), as expected. |

Under F-1 to F-3, all 358 digest comparisons stayed green on every leg. Nothing in production calls
the new surface, which is why the count exists beside the digests.

### Class A

- **What changed.** Outside `crates/lane/src/f64_lane.rs`, the diff touches `crates/lane/src/lib.rs`
  only by a module line, a re-export and a doc paragraph. The rest is the new test, tooling, the
  script and records.
- **Who uses it.** `LaneF64`/`Widen` are referenced only by `crates/lane` and
  `tools/wasm-gate-{corpus,guest}`/`tools/wasm-gates`, per `grep -rln` over `crates hosts tools`.
  No kernel, pin, digest or production crate changed.
- **Digests.** The G5 corpus's 358 comparisons match their pins on all three legs. Console workload
  digests cannot move, because no render path changed. The console benchmark was not run.

### Not measured

- AArch64: this host has no aarch64 target installed.
- `wasm_gates::native_report()` now also computes the count. `g6_full_corpus_ftz.rs` calls it under
  FTZ+DAZ and asserts only the digests, so the count, which DAZ is expected to make nonzero, is
  ignored there, as the verification record predicted. The test passes.

## Sol attempt 1 verdict: PASS

Reviewer: Sol, 2026-09-27, adversarial review of `git diff edc9273a..a7f7705f` on
`codex/949-f64-lane-vocabulary`. Host AMD EPYC 7313P, rustc 1.97.1, pin `+avx2,+fma`, wasmtime
47.0.3, wabt 1.0.34, node 22.23.2, `CARGO_INCREMENTAL=0`, a fresh worktree `target/`, which was
deleted afterwards. Every gate was re-run from scratch. Each mutation was applied alone as an
exact-text replacement and restored with `git checkout`, and `git status` was clean after each one.
The console benchmark was not run.

### Gates re-run

| # | result |
|---|---|
| 1 | PASS. Release sweeps all 2^32 patterns: `f32` 3.84 s, `Simd4` 2.07 s, `Simd8` 1.66 s on 4 threads, 0 mismatches. The dev sparse sweep and pool are 0. |
| 2, 3 | PASS, dev and release: 8 of 8 tests. |
| 4 | PASS. `f64_lane_mismatches: 0` on native, scalar wasm and simd128 wasm. All 358 digest comparisons match on every leg. Under V8 the export takes 0.3 to 1.6 ms per width on both guests. |
| 5 | PASS. `f64x2.promote_low_f32x4=2 f64x2.mul=2 f64x2.add=2`, scalar `f64.*` 0. The native `libwasm_gate_guest.so` probe has 1 each of `vcvtps2pd`, `vmulpd`, `vaddpd` and `vextractf128`, 0 `vcvtss2sd`, and 0 `vfmadd` in the whole `.so`. `probe(1) = 967085629` natively (ctypes) and on both guests. |
| 6 | PASS: fmt; clippy `-D warnings` over the workspace; `cargo doc` `-D warnings`; `cargo test -p wasm-gates` (7 + 2); lane policy and its mutation test; the unfused seal, whose `--self-test` gives 62/0; workspace policy; realtime policy (56 regions). |

### Adversarial checks

- **The tests are not vacuous.**
  - In the release `f64_lane` binary, the `Simd4` sweep closure holds 1 `vcvtps2pd` and the `Simd8`
    closure holds 2. Gate 2's closures hold `vaddpd`/`vmulpd` on `ymm`. So the tests call the real
    `lane::{LaneF64, Widen}` implementations, and they lower the same way the release caller does.
  - F-2 in **release** gives 16,777,214 mismatches at `Simd4` in the exhaustive sweep, which is
    every nonzero subnormal. The 2^32 gate therefore executes and discriminates.
- **No contraction.** I wrote an out-of-tree caller shaped like #950's step 6:
  `w = L::load(frame).abs().widen(); e = e.add(w.mul(w))`, plus a general `acc + widen(x) * widen(y)`,
  at `f32`, `Simd4` and `Simd8`, as a linked fat-LTO cdylib.
  - Native IR has 0 `contract`/`fast`/`reassoc` (or other FMF) flags, 0 `llvm.fmuladd` and 0
    `llvm.fma`. The linked `.so` has 0 `vfmadd`. `Simd8` gives 2 `vcvtps2pd`, 2 `vmulpd` and
    2 `vaddpd`.
  - The simd128 IR is also 0/0/0, with only `f64x2` ops for `Simd4`/`Simd8` and no scalar `f64`.
  - A build with `+simd128,+relaxed-simd` emits no `relaxed_madd` either.
- **The lowering pin catches a real library scalarisation.** These two mutations go beyond the
  recorded rows:
  - X-1: vector `mul` computed per lane through `black_box`.
  - X-2: `Widen for f32x4` converts through a `black_box`ed lane.

  Both leave gates 1 to 4 green and turn gate 5 red, with `f64.promote_f32=4`. F-3 staying green
  on gate 5 is expected: it changes values, not the lowering, and LLVM folds the `+ 0.0` there.
- **Mutations reproduce exactly as recorded.**
  - F-1: 522,496 and 100,096 in gates 1 and 3, and 81,676 on all three gate-4 legs.
  - F-2: 1,047 and 369, and 518 on every leg.
  - F-3: 4, and 2 on every leg.
  - F-4: gate 5 red with the recorded census, and gate 4 at 0.
  - F-green: green in dev, in release, on all three legs and on the pin.
- **API and scope.** `LaneF64` and `Widen: Lane` are separate traits, so `Observed<L>` is untouched.
  The tools name only `<L as Widen>::F64` and `lane::Simd4`, never `wide`. No `core::arch`, no
  `mul_add(` and no relaxed SIMD. Every changed path is authorized, and no production crate, kernel,
  pin or digest changed.

### Findings (severity-ranked; none blocks)

1. **Low (doc).** `crates/lane/src/f64_lane.rs:64-65`. `LaneF64`'s environment bullet names only
   `MXCSR.FTZ` ("a subnormal result would flush"). It cites `wide_impl.rs`'s "The one precondition",
   which is about **DAZ**, and DAZ would also zero subnormal *operands* of `add`/`mul`. The next
   sentence ("FTZ and DAZ clear") makes the precondition complete, so only the hazard wording is
   partial. Fix it when the file is next touched.
2. **Low (coverage; outside this slice's paths).** `tools/wasm-gates/tests/g6_full_corpus_ftz.rs:93-140`
   computes `f64_lane_mismatches` in its hostile-FTZ+DAZ unguarded and guarded arms but asserts
   neither. Asserting `guarded.f64_lane_mismatches == 0` would make `Widen`'s documented DAZ
   precondition executable. This is a one-line follow-up.
3. **Info (known).** Gate 5 pins the guest probe only. A caller-specific scalarisation, such as
   #950's `meter_block` or any native build, stays invisible. This is F64-VERIFY §1's robustness
   gap and the brief's named non-goal. #950 should census its own kernel.
4. **Info (CI cost).** `qualification.yml:526` runs `cargo test --release -p lane`, which now does
   3 × 2^32 sweeps: 7.6 s on 4 threads here, and about twice that on a 2-core runner. The `f32` arm
   (3.8 s) exceeds the brief's "cover". It is acceptable, and it is the first thing to trim if that
   job's budget tightens.
