## Why

The owner asked for an audit of `crates/math` — the foundation every DSP crate builds on — to make it as efficient as the engine's contracts allow, leaning on vectorization, and to use the result to shape future effects as well as current ones.

**Plain-language verdict.** The SIMD math already compiles to the minimum: one machine instruction per math operation on x86 AVX2 and on wasm, with no spills and no redundant work. The fast dB tier used per sample by the compressor, gate and multiband is proven to be at the lowest polynomial degree its accuracy gate allows, so EQ and compression have nothing left to gain from this crate under the current rules. The remaining wins are narrower:

- a software `sqrt` that is an order of magnitude or more slower than the hardware instruction and gives identical results;
- a cheaper `exp2_lane` with bit-identical output;
- a faster **and** more accurate `log2_lane`;
- moving the transient shaper, the last per-sample user of the slower exact tier, onto the fast tier;
- a compressor automation-ramp CPU spike.

Three pieces of recorded evidence are also wrong and need correcting.

This is an umbrella issue. It is broken into bounded tasks (IDs `MA-*`, `MQ-*`, `MB-*`, `MC-*`, `MZ-*`) that a coordination agent can dispatch one per agent. Every task names its files, its gates as runnable commands, and whether it moves pinned bits.

**Review record.** The draft was written by an Opus 5.5 (xhigh) scope agent from the audit's evidence. A Fable 5.1 (high) adversarial verifier then re-ran every exhaustive number through the real `math`/`lane` crates, checked every cited path, test, digest constant, script and command, and ran the math crate's default tests and clippy. Its findings (1 blocking, 9 should-fix, 9 nits) were resolved in this body. Two corrections came from that review: the inexact-floor count (F-5) and the `sqrt` speed range (MA-1). Timings marked [isolated] were not re-measured by the verifier.

Base commit for every reference below: `e09302ad` (`origin/main`, 2026-09-17). The audit ran at `1dca662a`; no path cited here changed between the two. Toolchain: `main` pins rustc 1.97.1 and the open #877 branch moves the pin to 1.98.1. Every toolchain-dependent claim below was checked on both (and on nightly-2026-08-20, i.e. rustc 1.100.0-nightly of 2026-08-19, where stated).

## Audit findings (technical)

### F-1. Codegen is already one instruction per `Lane` op
Measured by disassembly of `exp2_lane`, `log2_lane`, `fast_gain_from_db` and `fast_level_db` at `Simd8` (x86-64-v3) and `Simd4` (wasm32 `+simd128`):
- Every `Lane` operation lowers to exactly one instruction, with no stack spills, no calls and no redundant moves in the isolated function bodies.
- x86 arithmetic-instruction counts (loads, stores and constant broadcasts excluded), reproduced independently by the verifier: `fast_gain_from_db` 18, `fast_level_db` 20, `exp2_lane` 24 (the two selects are `vblendvps`), `log2_lane` 39.
- wasm `+simd128` (inspected with `wasm2wat`): likewise one wasm SIMD op per `Lane` op. Exact wasm counts depend on the counting convention (whether `i32x4.shl/shr_u` and `v128.and/or` count, and constant materialisation) and on the toolchain, and the two independent counts disagreed, so none are quoted.
- **Counting convention used below.** "Lane ops" means `Lane` trait calls in the source. x86 instruction counts can differ by ±2: today's `log2_lane` is 40 lane ops and 39 instructions; L3 is 27 lane ops and 29 instructions, because LLVM lowers one select as `vpcmpgtd`/`vpand`.
- There is nothing to reclaim at the compiler level. Remaining gains are algorithmic only.

### F-2. Where math runs
Call-site survey of every non-test `math::` call, including unqualified `use math::{..}` imports. Only the T0 (per-sample) sites are listed precisely here.

| T0 site (per lane-sample) | Function | Rate |
|---|---|---|
| `crates/compressor/src/kernel.rs` `curve_target` (crossing X1) | `fast_db::fast_level_db` | 1 |
| `crates/compressor/src/kernel.rs` `gain_mix` (X2) | `fast_db::fast_gain_from_db` | 1 |
| `crates/gate-expander/src/kernel.rs` `channel_step` (X3/X4) | both fast-tier functions | 1 each |
| `crates/multiband-compressor/src/lib.rs` `band_amplitude` (X5/X6) | both fast-tier functions | 2 each (two bands) |
| `crates/transient-shaper/src/lib.rs` `frame` | `log2_lane`, then `exp2_lane` | 1 each |

Other notable sites:
- **Render thread, per frame during a ramp:** `crates/compressor/src/design.rs` `rate_coefficient` calls `math::exp` (scalar `f64`) from `kernel.rs` `advance_ramps` on every frame of a `Linear 64` attack or release ramp (`SMOOTHING_SAMPLES = 64`), per ramping lane per channel.
  - [derived] One parameter ramping on all 8 lanes of a `Simd8` bank, both channels, is 64 × 8 × 2 = 1,024 `exp` calls per event.
  - [measured, isolated] ≈ 6.6 ns each, ≈ 6.8 µs per event per bank, and ≈ 54 µs if all eight banks of a 64-track session ramp in the same block. The 64-track console block is ≈ 222 µs (`docs/rulings/fast-db-tier-boundaries.md`).
- **Render thread, per edit:** `crates/builtins/src/filter_control.rs` `validate_prepared_input_filter_target` makes 2 `math::sqrt` calls per `PreparedFilter` record.
- **Off the render thread, high volume:** `crates/host-core/src/spectrum.rs` `fft_magnitude` calls `math::sqrt` about 1,023 times per channel per analysis frame, and the frame-to-dB path calls `math::log10` per bin.
- **Everything else** is per-event, plan-compile or UI rate.
- No other per-sample kernel computes a transcendental.
- `effect_runtime::dynamics::{level_db, gain_from_db}` (the exact tier) have no render-path caller; they are used by `effect_runtime::corpus` and tests only.

### F-3. The fast dB tier is at its minimum degree for gate F1
Refitted, not truncated. Method: LP minimax, then f32 coefficient rounding with refit, then an f32 coordinate search. Measured exhaustively over F1's exact domains and metric in a scalar-f32 copy of `fast_db.rs`.

The copy reproduces the shipped rows exactly: 7.4311e-6 and 2.1832e-6 dB for gain, 2.8103e-5 dB for level. The FMA rows use `f32::mul_add`; they are hypothetical, since fusion is banned.

**`fast_gain_from_db`, `2^f = 1 + f·P(f)`**

| P degree | x86 instructions (with FMA) | max dB, [-160, -0] | max dB, [0, 24] | gate `1.0e-5` |
|---|---|---|---|---|
| 4 (shipped) | 18 (13) | 7.431e-6 | 2.183e-6 | pass |
| 3, refit | 16 (12) | 3.210e-5 | 2.663e-5 | **fail 3.2×** |
| 2, refit | 14 (11) | 7.503e-4 | 7.45e-4 | fail 75× |

**`fast_level_db`, `log2(1+t) = t·Q(t)`**

| Q degree | x86 instructions (with FMA) | max dB, [1e-8, 16] | gate `4.0e-5` |
|---|---|---|---|
| 5 (shipped) | 20 (14) | 2.810e-5 | pass |
| 4, refit | 18 (13) | 1.006e-4 | **fail 2.5×** |
| 3, refit | 16 (12) | 6.304e-4 | fail 16× |

Refit coefficient words for the two rows that fail closest, so they can be reproduced (f32 bits, **highest order first**; the repo's arrays are ascending, so reverse them):
- P degree 3 (gain, 3.2×): `[0x3c5bf2e2, 0x3d55ffe6, 0x3e7711ca, 0x3f316b63]`
- Q degree 4 (level, 2.5×): `[0x3d3e0145, 0xbe48fcca, 0x3ed5d00c, 0xbf35aca2, 0x3fb89252]`

The other refit rows (P degree 2, Q degree 3) are research-harness figures whose coefficients were not recorded. They show the trend only and must not be cited in F1.

[derived] The real-coefficient minimax error, before any f32 rounding, is **2.52e-5 dB** for P degree 3 and **8.65e-5 dB** for Q degree 4. No refit, reduction or FMA can bring either under its gate. These bounds were derived by the research harness (LP minimax); the verifier found them consistent in magnitude with a Chebyshev-truncation estimate (≈2e-5 and ≈7e-5 dB) but did not re-derive them.

The degree is not slack. But `crates/math/tests/f1_fast_db_bounds.rs` and `docs/rulings/fast-db-tier-boundaries.md` support that with truncation mutations ("drop `EXP2_P[4]`/`LOG2_Q[5]` … same coefficients", "200x and 4000x over the gate"). A truncated minimax polynomial is not a lower-degree fit; `fast_db.rs` says so itself. Task MA-2 corrects this.

### F-4. `fast_level_db` is not monotone
[measured, exhaustive, through `impl Lane for f32`, over every f32 in `[1e-8, 16]`]
- 257,176,458 inputs checked; **77 decreasing steps**.
- The largest reversal is 1.526e-5 dB, a six-way tie (e.g. x = 2.5154717e-8 and 5.0309435e-8). All six ties sit at t = 0.68810. Within each octave, 65 of the 77 sit at t ∈ [0.65, 0.70] and 12 at t ≈ 0.844–0.850.
- The cause is unfused Horner rounding: the same coefficients evaluated with FMA have 0 reversals (not applicable here).
- `fast_gain_from_db` is monotone on both F1 gain domains (0 decreasing steps over 1,103,101,953 and 1,126,170,625 inputs).
- F1 does not check monotonicity, and no document mentions it. The reversals are inaudible, but a future threshold or hysteresis consumer must know about them.

### F-5. A false exactness claim
`crates/math/src/lane_math.rs` ("`x - floor(x)` is exact for `|x| < 2^23`") and `crates/math/src/fast_db.rs` ("The reductions themselves are exact: `x - floor(x)` is exact for `|x| < 2^23`") are wrong.
- [measured, exhaustive, error-free Knuth TwoSum in f32; re-derived analytically] **1,048,576,000** (= 125·2^23) finite f32 with `|x| < 2^23` give an inexact `x - floor(x)`. All of them are negative non-integers of magnitude below 1/2; no positive input rounds. Example: x = −5.551116e-17 (`0xa4800001`) gives `1.0`.
- Analytic count, for negative x: the binade `[2^-k, 2^(1-k))` of |x| contributes 2^23 − 2^(24−k) inexact inputs for k = 2..24. Every binade below 2^-24 (102 normal binades plus the subnormals) is entirely inexact. The sum is 125·2^23.
- **Exactness checks must use an error-free transformation (TwoSum) or integer arithmetic, never an f64 comparison.** An f64 check reports only 436,207,616 (= 52·2^23): for |x| below about 2^-54 the f64 difference also rounds to `1.0`, so 73·2^23 inexact inputs are misread as exact. The audit's first count made exactly this mistake.
- The published error bounds are unaffected, because they were measured exhaustively. Only the stated reasoning is wrong.
- At the recorded `exp2_lane` worst point x = −0.4910151, this rounding contributes 1.0 of the 1.4615 ulp.

### F-6. Price of each numeric constraint (why the contract stays as it is)

| Constraint given up | Native (x86 AVX2) | Browser (V8 wasm) | Verdict |
|---|---|---|---|
| Cross-target bit identity (which forces no FMA) | `artifacts/issue163-phase2/README.md`: 64-track console 92.27 → 95.46 µs from unfusing (+3.5%). [isolated] FMA speeds up the four dB functions by 12–23% | Base `simd128` has no FMA. Relaxed-SIMD `f32x4.relaxed_madd` was mixed: `log2` −15%, `fast_level` −7%, but `exp2` +12% and `fast_gain` +15% **slower**. Relaxed SIMD is also allowed to differ between implementations | keep |
| Monotone output | [isolated] non-monotone exp2 E4 (20 lane ops, 1.059 ulp) is 0.607 vs 0.604 ns: no gain | E4 is 1.134 vs 0.99–1.00 ns: slower | keep; it is free |
| No SIMD tables | [estimate, not built] an 8-entry in-register table (`vpermps`) might save about 10–20% on `exp2` | ≈ 0: wasm has only a 16-byte swizzle | keep |
| ≤ 2 ulp exact tier / F1 fast-tier gates | see F-3: one degree less costs 3.2× / 2.5× the gate, for 2 instructions each | same | keep |

[isolated] = microbenchmark on AMD EPYC 7313P (Zen 3), rustc 1.97.1, `-C target-feature=+avx2,+fma`, fat LTO, `codegen-units = 1`; functions inlined over a 4096-element f32 array; one warmup plus two measured rounds. The wasm column is the same loops in wasm32 `+simd128` under Node 22.23.2 / V8 12.4.254.21, with TurboFan-only (`--no-liftoff`) runs matching the default runs.

Per `docs/rulings/fast-db-tier-boundaries.md` (boundary 4), an isolated loop under-predicted an in-kernel win by 17×. **None of these numbers may be quoted as a render-time win.** In-kernel tasks below produce those.

## Invariants every task inherits

- `AGENTS.md` in full, especially:
  - realtime render rules;
  - "Fusion exists nowhere": no `mul_add`, no fused intrinsic; `scripts/check-unfused-seal.sh` enforces this;
  - no `unsafe` (workspace `unsafe_code = "deny"`);
  - benchmark protocol: freeze workload and validator, one invocation, one warmup and two measured rounds, descriptive only, never tune or retry;
  - five-attempt limit and CI-conscious batching.
- **`crates/math` stays target-unconditional.**
  - `tests/m3_determinism.rs::m3_no_target_conditional_source` bans `target_feature`, `core::arch`, `std::arch`, `mul_add`, `target_arch` and `is_x86_feature` in `src/vendored/`.
  - `m3_no_unsafe_or_force_eval_in_vendored_source` bans `unsafe`, `force_eval!`, `select_implementation!` and `read_volatile` there.
  - The same discipline applies to `lane_math.rs` and `fast_db.rs`, which may use only `Lane` trait operations.
- **Pinned digests.** A **class A** task must not move any digest listed below; a digest that moves means the task failed. A **class B** task moves exactly the pins it names, in the same commit, with the reason recorded. Pins must be regenerated from the scalar `Lane` oracle, never from a wasm or SIMD run (`scripts/run-wasm-gates.sh` header).
  - Pin sets:
    - `math::corpus::M3_DIGESTS`;
    - `EXP2_DIGEST` and `LOG2_DIGEST` in `crates/math/tests/m2_lane_identity.rs`;
    - `effect_runtime::corpus::D1_DIGESTS` (rows `level_db`, `gain_from_db` and seven others);
    - `transient_shaper::corpus::CROSS_TARGET_DIGESTS` (3 rows);
    - `tools/wasm-gate-corpus` `LANE_DIGESTS` (`src/lane_digests.in`, including the `exp2_lane` and `log2_lane` rows);
    - the M1 measured-worst-point pins in `m1_measured_worst_points`.
  - Re-pin switches: `MISO_ENGINE_MATH_PIN=1` (M2, M3), `MISO_ENGINE_REPIN_EFFECT_RUNTIME_CORPUS=1` (D1), `MISO_ENGINE_REPIN_TRANSIENT_SHAPER_CORPUS=1` (transient shaper). `LANE_DIGESTS` has no environment switch: regenerate it with `cargo run --locked --release -p wasm-gates -- --print-pins` (see `tools/wasm-gates/src/main.rs`) and replace the entire contents of `tools/wasm-gate-corpus/src/lane_digests.in` with the printed output (it prints every case, not just changed rows); then confirm only the intended rows differ in `git diff`.
- **CI routing.** `qualification.yml` runs the M1 and F1 exhaustive jobs only when a diff touches `crates/math/` or `crates/lane/` (`route.outputs.math_closure`, `scripts/ci-path-router.py`); `nightly.yml`'s `math-sweeps` job also runs both unconditionally every night. Every math task here touches `crates/math/`, so its exhaustive gates run in qualification. Still run them locally before pushing.
- **Never add points to an existing digest corpus in a class-A task.** A new corpus point changes the digest even when no function changed. Put new coverage in new tests.
- **NaN bit patterns are outside the determinism claim** (master plan D5). The M3 corpus is NaN-free (`m3_corpus_is_nan_free`).

## Owner rulings required

The owner has **not** ruled on any of these. The tasks they gate are marked BLOCKED-ON-RULING and must not start until the ruling is recorded on this issue.

- **R1 — `log2_lane` algorithm.** Replace the Cephes degree-9 form (40 lane ops, 1.4667 ulp) with the atanh form **L3**: 27 lane ops including one `div`, 1.2983 ulp. The alternative is the division-free **L1**: 33 lane ops, 1.3595 ulp. Either one moves the exact tier's pins.
  - [isolated] `Simd8` throughput 0.948 → 0.657 ns/element (L3) or 0.831 (L1); latency 23.8 → 18.0 ns (L3) or 21.6 (L1).
  - V8: 2.12 → 1.27 (L3) or 1.77 (L1).
  - Recommendation: **L3**. It is faster on every measured target and more accurate. Adopt it now, while the exact tier has one per-sample consumer; every future consumer adds pins to the next re-pin.
- **R2 — transient shaper onto the fast tier.** Admit two new named fast-dB crossings, X7 (detector contrast) and X8 (applied gain), in `crates/transient-shaper/src/lib.rs` `frame`. This moves `transient_shaper::corpus::CROSS_TARGET_DIGESTS` and requires re-deriving the oracle bound in `crates/transient-shaper/tests/oracle.rs` (see MB-2).
  - [isolated, sum of the two conversion functions, not the kernel] The shaper's dB math drops from 1.48 to 0.84 ns per lane-sample on `Simd8` and from 3.13 to 1.30 ns in V8. MQ-1 supplies the in-kernel number.
  - [derived] The applied-gain change is ≤ ≈ 5.8e-5 dB; the second adversarial review measured 1.65e-5 dB. MA-5 measures it exhaustively, proves the domain claims, and checks the shaper's oracle tolerance **before** this ruling, so the ruling can be made on evidence.
  - `docs/rulings/fast-db-tier-boundaries.md` already names the shaper "a genuine candidate, ruled out of scope rather than ruled out on evidence".
  - Recommendation: **approve**.
- **R3 — compressor ramp coefficient design.** Choose how `rate_coefficient` is evaluated during attack/release ramps (options in MC-2). Every option that removes the spike moves compressor bits during ramps.
  - Recommendation: ramp in the coefficient domain between exact endpoints, as the true-peak limiter (release) and delay (damping `g`) already do.
- **R4 (optional) — refit `exp2_lane` for accuracy only (E3).** 1.4615 → 1.2970 ulp, 22 lane ops. Not faster: [isolated] 0.569 vs 0.538 ns/element on `Simd8`, because the chain is latency-bound. It moves exp2 pins. Recommendation: **decline** unless a consumer needs the accuracy.

## Tasks

Capability tiers:
- **S:** mechanical, doc or test plumbing.
- **M:** a scoped implementation with a known proof recipe.
- **X:** numerics derivation or a class-B re-pin that needs judgement.

Each task is sized for ≤ ½ working day for one agent.

**Task index.**

| ID | Task | Class | Ruling | Depends on | Tier |
|---|---|---|---|---|---|
| MA-1 | Hardware `sqrt`/`sqrtf` (optional MA-1b: `floor`) | A | — | — | M |
| MA-2 | Correct the fast-tier evidence; pin `fast_level_db`'s 77 reversals | A | — | — | S/M |
| MA-3 | `exp2_lane` magic-constant round (E1), bit-identical | A | — | — | M |
| MA-4 | Doc rule for future per-sample transcendentals | A | — | MA-1 (same file) | S |
| MA-5 | Shaper-domain proofs and error figures for R2 | A (tests) | — | MA-2 (same file) | M |
| MQ-1 | Transient-shaper in-kernel benchmark baseline | A (qual.) | — | — | M |
| MQ-2 | Compressor ramp-spike measurement | A (qual.) | — | — | M |
| MB-1 | `log2_lane` → L3 (or L1) | B | **R1** | MA-3 (same file); MB-2 first if R2 approved | X |
| MB-2 | Transient shaper → fast dB tier (crossings X7/X8) | B | **R2** | MA-5, MQ-1 | X |
| MB-3 | `exp2_lane` refit E3 (optional, accuracy only) | B | **R4** | MA-3, MB-1 | M |
| MC-1 | Compressor ramp design-options brief | — (no code) | feeds R3 | MQ-2 | X |
| MC-2 | Implement the ruled compressor ramp design | B | **R3** | MC-1; before #15/#17 attack/release | X |
| MZ-1 | Delivery sync: local spec, evidence, closure | — | — | — | S |

**Attempt budget.** Each task gets at most two attempts: one implementation and one correction after a single adversarial verdict. A task that fails its second attempt stops, its evidence is preserved, and the coordinator rescopes it. The AGENTS.md five-attempt ceiling is never multiplied across tasks or quietly extended.

---

### MA-1 — Hardware `sqrt`/`sqrtf` in `math` (class A, no ruling) — tier M

**Goal.** Replace the software `u128::isqrt` square root with the IEEE hardware instruction. Results are identical for every non-NaN input.

**Files.** `crates/math/src/lib.rs`, `crates/math/src/vendored/sqrt.rs`, `crates/math/VENDORED.md`, `crates/math/tests/scalar_accuracy.rs` (or a new test file).

**Change.**
- `lib.rs`: replace `#[cfg(test)] extern crate std;` with an unconditional `extern crate std;`. Keep `#![no_std]`, so `Vec`, `String` and the allocating prelude stay unreachable. This follows the precedent in `crates/lane/src/lib.rs` ("`std` is used for exactly two things … the `f32::floor` and `f32::sqrt` inherent methods").
  - It adds no platform requirement: every `math` consumer is a `std` crate, or (`effect-runtime`) enables the `lane` feature, which already links `std` through `lane`.
  - It must stay unconditional: `core::f64::math::sqrt` is still unstable (`core_float_math`, rust-lang/rust#137578) on stable 1.97.1, stable 1.98.1 and nightly-2026-08-20 (rustc 1.100.0-nightly, 2026-08-19). None of these has `f64::sqrt`/`floor` as inherent methods in `no_std`.
- **Speed** [isolated, EPYC 7313P, three independent harnesses]: software `math::sqrt` costs 27–30 ns per independent call and about 38–42 ns in a dependent chain. Hardware `f64::sqrt` is about 5–6× faster in a dependent chain (≈ 6–9 ns), 12–13× per independent call (≈ 2.3 ns), and up to 45× in a store-throughput loop (≈ 0.6 ns). For `sqrtf`: 21× (per call through a function pointer) to 160× (store-throughput loop). The task stands on identical results and simpler code, whatever the exact factor.
- `vendored/sqrt.rs`: the bodies become `f64::sqrt(x)` and `f32::sqrt(x)`, with **no NaN guard**. An `if x < 0.0 { return f64::NAN }` guard is removed by LLVM at `opt-level=3`: measured x86 codegen is `sqrtsd; ret` and wasm is `f64.sqrt`.
  - Document that a negative argument returns a NaN whose bit pattern depends on the target: measured `0xfff8000000000000` on x86-64; ARM's default NaN is positive; wasm's is nondeterministic. This is outside D5.
  - Replace the file header's correct-rounding derivation with the IEEE 754 argument, and keep the old derivation in the test oracle (below).
- `lib.rs` docs:
  - Reword "The crate is `no_std`; `std` is used only by its tests."
  - Reword "No function in this crate branches on target features, calls an intrinsic, or fuses a multiply and an add" so it names the `sqrt` exception the way `lane` does.
  - Update the `sqrt` doc ("it exists because `core` has no `f64::sqrt`").
- `VENDORED.md`: the `sqrt.rs` bullet no longer describes an `isqrt` derivation.

**Tests to commit.**
- Move the current `vendored/sqrt.rs` bodies (`sqrt` via `u128::isqrt` with the `r > q` rounding rule, and `sqrtf` via it) verbatim into the test file as `software_sqrt`/`software_sqrtf`: an independent, proof-carrying correctly-rounded oracle.
- `#[ignore]` exhaustive test: all 2^32 f32 patterns. Where both results are non-NaN they must be bit-equal; NaN-ness must agree.
  - [measured] 2,139,095,042 non-NaN comparisons, 0 mismatches; 2,155,872,254 inputs NaN on both sides.
- Default-run sampled f64 test against `software_sqrt`, plus an `#[ignore]` large run.
  - [measured] 1,878,905,041 comparisons, 0 mismatches. The sample covered raw patterns, subnormals, exact squares with their ±4-ulp neighbours, and near-midpoints.
- Keep the existing `sqrtf_is_correctly_rounded*` and `sqrt_is_correctly_rounded` platform checks as wiring checks.

**Gates.**
- `cargo test --locked --release -p math` passes, with `m3_corpus_digests_match_pins` unchanged and both source-scan tests green.
- `cargo test --locked --release -p math --test scalar_accuracy -- --ignored`, including the new exhaustive tests.
- `scripts/run-wasm-gates.sh` passes all three legs; M3 replays under wasm with **unchanged** pins.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`; `cargo fmt --all -- --check`; `scripts/check-workspace-policy.sh`; `scripts/check-unfused-seal.sh`.

**Evidence.** Before/after disassembly of `math::sqrt` for x86-64 and wasm32. Output of both exhaustive tests. A statement that no digest moved.

**Optional sub-task MA-1b (tier S): `floor`/`floorf`.**
- Same treatment (IEEE `floor` is exact). [isolated] 1.48 → 0.14 ns.
- Only caller: `vendored/rem_pio2_large.rs` (huge trig arguments). Hardware `floor` quiets a signaling NaN input where the software returns it unchanged, which is outside D5.
- Do it only if it rides along cheaply; it has no hot caller.

---

### MA-2 — Correct the fast-tier evidence and pin its monotonicity (class A, no ruling) — tier S/M

**Goal.** Fix F-3's mis-supported claim, F-5's false claim in `fast_db.rs`, and document and pin F-4.

**Files.** `crates/math/tests/f1_fast_db_bounds.rs`, `crates/math/src/fast_db.rs`, `docs/rulings/fast-db-tier-boundaries.md`. Do **not** touch `lane_math.rs`; MA-3 owns it.

**Change.**
- F1's "Red mutations" header and the ruling's "A lower degree still" bullet keep the truncation rows as *mutations* (they still go red). They must stop citing them as the degree argument ("200x and 4000x … The degree is not slack"). Cite instead:
  - the two reproducible refit rows of F-3, with their coefficient words: P degree 3 at 3.210e-5 / 2.663e-5 dB (3.2× over) and Q degree 4 at 1.006e-4 dB (2.5× over). **Re-measure both with F1's own sweep before writing them in**, and label them measured;
  - the real-coefficient lower bounds 2.52e-5 dB and 8.65e-5 dB, labelled *derived (research harness, LP minimax)*.
  - Do not cite the P degree 2 or Q degree 3 rows; their coefficients were not recorded.
- `fast_db.rs`: replace "The reductions themselves are exact: `x - floor(x)` is exact for `|x| < 2^23`" with a correct statement.
  - `x - floor(x)` rounds for 1,048,576,000 inputs, all negative non-integers of magnitude below 1/2 (count method in F-5). The rounded fraction stays in `[0, 1]`, and the result can be exactly `1.0` (then `exp2_int(xi) * (1 + 1·P(1))`).
  - The exhaustive F1 bound includes this effect.
  - `frexp` and `exp2_int` remain exact.
- Document F-4 in `fast_db.rs` ("Behaviour at and beyond the domain"): 77 one-ulp reversals ≤ 1.526e-5 dB, from unfused Horner rounding.
- Pin F-4 in F1: assert exactly 77 decreasing steps for `fast_level_db` over `[1e-8, 16]`, and 0 for `fast_gain_from_db` over each gain domain.
  - **F1's existing `sweep` cannot count them.** It takes `error_db: fn(f32) -> Option<f64>`, never sees the function's value, and keeps no previous point. Add a separate value-tracking sweep, or extend `sweep` to return values.
  - The count must be independent of thread count. Record each thread's first and last `(x, y)` and join the pairs at chunk boundaries after the scope. (`m1_exhaustive.rs`'s memory note documents that M1's sweep drops those pairs; do not copy that behaviour.)
  - Record a red mutation: perturb one `LOG2_Q` word so the count changes.

**Gates.**
- `cargo test --locked --release -p math --features lane --test f1_fast_db_bounds -- --ignored`, which qualification CI runs, is green with the new assertions.
- The default `cargo test -p math --features lane` passes.
- No digest moves; there is no code change to `fast_db.rs` bodies.
- `cargo clippy …`, `cargo fmt …`.

**Evidence.** The sweep output showing 77/0/0. The red-mutation result.

---

### MA-3 — `exp2_lane`: replace the compare/select fold with a magic-constant round (E1) (class A, no ruling) — tier M

**Goal.** One fewer lane op and two fewer selects, with bit-identical output.
- [isolated, V8] 1.013–1.016 → 0.862–0.871 ns/element (−15%).
- [isolated, AVX2] neutral: 0.533–0.538 vs 0.541–0.542.
- Its only current per-sample consumer is the transient shaper; it also benefits every future `exp2_lane` user.

**Files.** `crates/math/src/lane_math.rs` (`exp2_lane` body, its frozen-order doc, and the module doc's false exactness sentence), `crates/math/tests/m1_exhaustive.rs` (header note only), and a new test.

**Change (operation order, frozen after this task).**
```rust
let x = x.max(L::splat(-126.0)).min(L::splat(127.0));
let xi = x.floor();
let f = x.sub(xi);                                   // in [0, 1]; may round up to exactly 1.0
let r = f.add(L::splat(12_582_912.0)).sub(L::splat(12_582_912.0)); // 1.5·2^23: RNE(f) ∈ {0, 1}
let xi = xi.add(r);
let f = f.sub(r);                                    // f − 1 is exact by Sterbenz when r = 1
// unchanged from here: Cephes Horner on f, `1 + f*p`, `p * exp2_int_in_range(xi)`
```

**Why it is the same function.**
- `RNE(f)` for f ∈ [0, 1] equals `f > 0.5` exactly: ties-to-even sends 0.5 to 0, matching the strict `>`.
- `xi + 0.0` can turn `-0.0` into `+0.0`, but `exp2_int_in_range(±0.0)` builds identical bits. The exhaustive proof covers this.
- Fix the `lane_math.rs` module sentence "Every reduction step here is exact. `x - floor(x)` is exact for `|x| < 2^23`" the same way as MA-2. `f - 1` and `0.5 * m - 1` are genuinely exact by Sterbenz and may stay.

**Test to commit.** A new `#[ignore]` exhaustive test (e.g. in `m1_exhaustive.rs` or a new `tests/e1_identity.rs`) containing a test-local copy of the **pre-change** `exp2_lane` body. It asserts bit equality with the new `math::exp2_lane::<f32>` over **all 2^32 patterns**, including NaN, ±inf, the clamps and x ∈ (−0.5, 0).
- [measured] 0 differing patterns.
- Also run the same comparison at `Simd8` and `Simd4` against the scalar oracle: [measured] 0 mismatches on x86-64.

**Gates.**
- Must **not** move: `EXP2_DIGEST` (M2), `M3_DIGESTS`, `D1_DIGESTS`, `CROSS_TARGET_DIGESTS`, `LANE_DIGESTS`, or the M1 worst point (`exp2_lane` 1.4615 ulp at `0xbefb6655`).
- Commands:
  - `cargo test --locked --release -p math --features lane --test m1_exhaustive -- --ignored`
  - `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane`
  - `cargo test --locked --release -p effect-runtime -p transient-shaper`
  - `scripts/run-wasm-gates.sh`
  - clippy, fmt, `scripts/check-lane-policy.sh`, `scripts/check-unfused-seal.sh`
- Do **not** edit the M2 corpus; the fold-boundary points it already carries stay, and new edge coverage goes in the new test only.

**Evidence.** Exhaustive identity output. wasm op counts before and after for `exp2_lane` (e.g. `wasm2wat` of a probe; the counts should go from 2 `v128.bitselect` + 1 `f32x4.gt` to none). Optionally one descriptive V8 or wasmtime run.

**Target note.** No CI leg executes AArch64 NEON (all workflows run `ubuntu-24.04` x86-64). NEON identity is argued: E1 adds only IEEE `add`/`sub` and removes a compare and select. If an AArch64 host is available, run `m2_lane_identity` there and attach the result.

---

### MA-4 — Rule for future per-sample transcendentals (class A, doc only) — tier S — after MA-1 (same file)

**Goal.** Stop future effects from repeating the compressor-ramp pattern: scalar `f64` transcendentals called per lane per frame.

**File.** `crates/math/src/lib.rs` crate docs: a short "Adding a function" section.

**Content.**
- A function needed per sample (T0) or per frame on the render thread gets a `Lane`-generic implementation in this crate.
- It is built only from `Lane` basic operations, unfused.
- It is proven by an exhaustive or full-domain sweep like M1/F1, including a monotonicity statement and red mutations, and bit-identical across `Scalar`/`Simd4`/`Simd8` and wasm (M2 style).
- Scalar functions remain for control-plane (T2/T3) use.
- Name the fast-tier seal (`clippy.toml` `disallowed-methods`) as the model for any reduced-accuracy tier.
- Link upcoming consumers:
  - #15 (De-esser) and #17 (Dynamic EQ), which will need per-sample dB conversions and, for dynamic EQ, possibly per-sample coefficient updates;
  - issue #763 (engine-owned analysis).

**Gates.** `cargo doc -p math --no-deps` has no warnings; `cargo fmt`; clippy (`missing_docs` is deny).

---

### MA-5 — Evidence for R2: the shaper's domains under the fast tier (class A, tests only, no ruling) — tier M — after MA-2 (same file)

**Goal.** Turn MB-2's domain and error claims into committed, exhaustive evidence *before* the owner rules on R2. Nothing moves: no code outside tests changes and no pin moves.

**File.** `crates/math/tests/f1_fast_db_bounds.rs`: new tests only. Keep them in the bound gate so `clippy.toml`'s prose stays true (the fast tier is "admitted only inside `fast_db.rs` itself, the bound gate that proves its error, and six named crossings"). Structurally any test file may opt out per file with `#![allow(clippy::disallowed_methods)]` (`m2_lane_identity.rs` already does, for its fast-tier identity edges), but do not add a second fast-tier test file without saying why.
- **Naming constraint:** `f1_the_container_pins_exactly_six_crossings` counts occurrences of `fn f1_crossing_x`. The new tests must **not** use that prefix, or the count breaks; use e.g. `fn f1_prospective_shaper_*`. MB-2 renames them if R2 is approved.
- `math` cannot depend on `transient-shaper` (it would create a cycle). **Restate the shaper's constants by bit pattern**, with a comment naming their source: `FLOOR = 0x322b_cc77` (≈ 1e-8), `CONTRAST_LIMIT_DB = 24.0`, `SHAPE_LIMIT_DB = 18.0`, and attack/sustain ∈ [−1, 1] (parameters 1 and 2 in `crates/transient-shaper/src/lib.rs`). For the exact-tier side, reuse F1's existing `exact_level_db`/`exact_gain_from_db` helpers: their constants are bit-identical to the shaper's `DB_PER_OCTAVE` (`0x40c0_a8c1`) and `OCTAVES_PER_DB` (`0x3e2a_152d`), so no further constants need restating.

**Claims to prove** (they are claims until these tests pass):
1. Every ratio that survives the ±24 dB contrast clamp, i.e. `ratio ∈ [10^(−24/20), 10^(24/20)] ≈ [0.0631, 15.85]`, lies inside F1's proven level domain `[1e-8, 16]`. This is an arithmetic assertion on the f32 bounds.
2. Exhaustively: for every positive `ratio < 1e-8`, including `+0.0` and subnormals (reachable when `slow` is huge; `fast_level_db` clamps them to `MIN_POSITIVE`), `fast_level_db(ratio) ≤ −24`; for every finite `ratio ≥ 16`, `fast_level_db(ratio) ≥ 24`. Out-of-domain ratios therefore produce exactly the clamp rail, as the exact tier does today. [verified by the adversarial review: 0 violations in both ranges]
   - Ratios below `1e-8` are reachable: `ratio = max(fast, FLOOR) / max(slow, FLOOR)` with `slow > 1`.
3. `shape` is clamped to ±18 dB, inside F1's two gain domains.

**Measure and report** (evidence for the ruling; do not decide anything):
- The end-to-end difference between the two tiers on the shaper's conversion pipeline. Exhaustively over `ratio ∈ [1e-8, 16]`, for each of the four sign combinations (attack, sustain) ∈ {±1}², emulate `contrast = clamp(level_db(ratio), ±24)`, `shape = clamp(A·max(c,0) + S·max(−c,0), ±18)`, `gain = gain_from_db(shape)` with the exact tier (F1's `exact_level_db`/`exact_gain_from_db`) and with the fast tier. Report the maximum over all four combinations of `|20·log10(g_fast / g_exact)|` in dB, plus max |Δcontrast|.
  - [measured by the second adversarial review, to be reproduced here] max |Δgain| = 1.654e-5 dB (A = +1; 1.652e-5 at A = −1), worst near ratio ≈ 7.76 and 6.38; max |Δcontrast| = 1.526e-5 dB.
  - [derived, to be checked] `|Δcontrast| ≤ 2.810e-5 + 1.538e-5` dB; the clamps are 1-Lipschitz and only one of the two shape terms is nonzero, so `|Δshape| ≤ |Δcontrast|`; the gain-conversion difference adds ≤ 7.431e-6 + 7.020e-6 dB. Total ≤ ≈ 5.8e-5 dB (< 1e-4 dB).
- Against an f64 reference of the same pipeline: the fast pipeline's worst **absolute** gain error on a unit-amplitude input, for comparison with the `2.0e-5` absolute row tolerance in `crates/transient-shaper/tests/oracle.rs` (`scalar_matches_the_independent_f64_oracle`).
  - [measured by the second adversarial review, to be reproduced here] The fast pipeline's worst absolute gain error on unit amplitude is 1.29e-5 (exact tier: 2.2e-6). That fits `2.0e-5` with a 1.55× margin; the oracle row's input peaks at 0.7 (`oracle.rs`), giving ≈ 9e-6. **Report the figure and state plainly whether it fits.** If it does not, R2 must also rule on the tolerance.

**Gates.**
- The default run `cargo test --locked --release -p math --features lane --test f1_fast_db_bounds` is green with the container count still **6**. The container count and crossing tests are not `#[ignore]`d, so `-- --ignored` alone would skip them.
- `… --test f1_fast_db_bounds -- --ignored` is green for the new exhaustive tests, which are `#[ignore]`d like F1's other exhaustive sweeps.
- No pin moves. Clippy and fmt.

**Evidence.** The test output and the two reported figures, posted on this issue next to R2.

---

### MQ-1 — Transient-shaper in-kernel descriptive benchmark and baseline (class A, qualification) — tier M

**Goal.** Provide the real-kernel instrument that R2 (and R1, if the shaper stays on the exact tier) must be judged by. The fast-dB ruling forbids sizing per-sample math with an isolated loop.

**Files.** New `crates/transient-shaper/tests/bench.rs`, `#[ignore]`d, modelled on `crates/true-peak-limiter/tests/bench.rs` (the "B1 … descriptive throughput measurement" pattern).

**Workload (freeze before timing).**
- Prepared `TransientShaper` at 48 kHz, 128-frame blocks, both a `Simd8` bank (`BankWidth` per `effect-contract`) and a single scalar instance.
- A deterministic 4-second programme with transients: seeded impulses plus decays.
- Attack 0.75, sustain −0.5, mix 1.0.
- Report ns per lane-sample.

**Protocol.** AGENTS.md benchmark rules: preflight arguments and output persistence without launching the timed workload; one invocation; one warmup; two measured rounds; no tuning or retry; record host, toolchain and commit.

**Deliverable.** The bench file plus a baseline record on the current tree, attached here.

**Gates.** The bench compiles and is ignored by default; `cargo test -p transient-shaper` is unaffected.

---

### MQ-2 — Compressor ramp spike: frozen measurement (class A, qualification) — tier M

**Goal.** Measure the real cost of per-frame `rate_coefficient` recomputation before R3 is decided.

**Files.** New `crates/compressor/tests/bench_ramp.rs`, `#[ignore]`d, same pattern as MQ-1.

**Workload (freeze before timing).**
- A `Simd8` compressor bank at 48 kHz, 128-frame blocks, eight banks (64 tracks).
- **Arm A:** a block with a release point event on every lane of every bank at frame 0, so all lanes ramp for 64 frames.
- **Arm B:** attack and release events on every lane.
- **Arm C:** the same audio with no automation.
- Report µs per block per arm, and the count of `rate_coefficient` calls per block **derived from the ramp state** (ramping lanes × ramping parameters × frames). Do not add `#[cfg(test)]` counters to production render code.

**Protocol.** As in MQ-1. Also report the per-block maximum, since the concern is a spike, not an average.

**Deliverable.** A bench file plus a record. Cross-check against the derived 1,024 `exp` calls per bank per ramping parameter.

---

### MB-1 — `log2_lane` → L3 (or L1) (class B) — **BLOCKED-ON-RULING R1** — tier X

**Files.** `crates/math/src/lane_math.rs` (`log2_lane`, constants, module docs), `crates/math/tests/m1_exhaustive.rs` (header table, red mutations, the "Cephes polynomials under test" sentences, `m1_measured_worst_points`), `crates/math/tests/m2_lane_identity.rs`, `crates/effect-runtime/src/corpus.rs` (`D1_DIGESTS`), `tools/wasm-gate-corpus/src/lane_digests.in`, `crates/math/tests/f1_fast_db_bounds.rs` (exact-tier reference column only), and `crates/math/src/fast_db.rs` (module doc only: its opening sentence calls the exact tier "Cephes polynomials"). Also `crates/transient-shaper/src/corpus.rs` and `tests/oracle.rs` **only if R2 is declined**.
- `docs/rulings/*.md` are historical records ("already ran … Cephes polynomials", "replacing Cephes degree 6 and 9"). They described the tree at the time: **do not rewrite them**. At most, append a dated one-line note pointing to this issue.

**L3 operation order (frozen).**
```rust
let x = x.max(L::splat(f32::MIN_POSITIVE));
let (m, e) = x.frexp();                                        // m in [1, 2)
let fold = m.gt(L::splat(core::f32::consts::SQRT_2));
let t = m.mul(L::select(fold, L::splat(0.5), L::splat(1.0))).sub(L::splat(1.0)); // same bits as today's fold
let e = e.add(L::select(fold, L::splat(1.0), L::splat(0.0)));
let s = t.div(t.add(L::splat(2.0)));
let z = s.mul(s);
let r = z.mul(horner(P3, z));        // P3 highest first: p = P3[0]; p = p*z + P3[1]; p = p*z + P3[2]
let u = s.mul(t.sub(r));             // ln(1+t) = t − u
t.mul(L::splat(A)).sub(u.mul(L::splat(L2E))).add(t).add(e)     // log2(1+t) = t + t·A − u·log2(e)
```
Constants (f32 bits):
- `P3 = [0x3e99004a, 0x3eccaefc, 0x3f2aaab1]`
- `A = 0x3ee2a8ed` (f32 of log2(e) − 1, today's `LOG2EA`)
- `L2E = 0x3fb8aa3b` (f32 of log2(e))

**L1 alternative (division-free).** Same `t`/`e`. Then:
- `g = horner(G, t)` with `G` (highest first) = `[0xbdf874b9, 0x3e41e6b2, 0xbe3fb6ec, 0x3e507cea, 0xbe75c925, 0x3e93cafc, 0xbeb8ab26, 0x3ef637f5, 0xbf38aa3b, 0x3ee2a8ed]`;
- `return t.add(t.mul(g)).add(e)`.

**Measured by the audit** [exhaustive, M1 method, through `impl Lane for f32`, all 2,130,706,432 positive normals]:
- L3: max **1.2983 ulp** at x = 0.7106287 (`0x3f35ebc3`); 0 decreasing steps.
- L1: 1.3595 ulp at x = 0.7078326 (`0x3f353484`); 0 decreasing steps.
- Anchors `log2(1)=0`, `log2(2)=1`, `log2(0.5)=−1` are exact; NaN, 0 and −inf give −126; +inf gives 128 (same as today).
- `Scalar`/`Simd4`/`Simd8` bit-identical over all 2^32 patterns on x86-64.
- Coefficient provenance: LP minimax with sequential f32 rounding and refit, then an f32 coordinate search scored by full two-rounding evaluation. Record this in the module doc as `fast_db.rs` does. The exhaustive sweep is the proof; the fit is only provenance.

**Gates.**
- `m1_log2_lane_exhaustive` ≤ 2 ulp and monotone; re-measure and re-pin the M1 header table.
- **`m1_measured_worst_points` needs a structural edit, not just a re-pin.** It asserts one shared lower floor, `worst >= 1.4`, for both rows in the neighbourhood of each recorded worst point. L3's maximum is 1.2983 ulp, so no bit pattern can satisfy 1.4. In the same commit:
  - make the floor a per-row value;
  - set the `log2_lane` row to L3's worst point (`0x3f35ebc3`) with floor `1.25`;
  - keep the `exp2_lane` row at `0xbefb6655` / `1.4` (unless MB-3 also lands).
  - `MAX_ULP` (2.0) does not change. Lowering a *worst-point floor* to describe a more accurate function is not "changing M1's gate" (see Non-goals).
- **Re-run and record fresh red mutations for the new body**; the current ones name Cephes `LOG2_P`.
- `LOG2_DIGEST` re-pinned via `MISO_ENGINE_MATH_PIN=1`, while `EXP2_DIGEST` is unchanged.
- `D1_DIGESTS` row `level_db` re-pinned. **Every other D1 row must be unchanged**, or its movement explained.
- The `LANE_DIGESTS` `log2_lane` row re-pinned with `cargo run --locked --release -p wasm-gates -- --print-pins` (scalar oracle); every other `LANE_DIGESTS` row unchanged. The compressor, gate and multiband corpora must not move, since those effects use the fast tier.
- `f1_fast_tier_stays_within_twice_the_exact_tier` still passes. Measured with L3 in place, the exact-tier level error is 1.5382e-5 dB (today's `log2_lane`: 1.5383e-5); the fast tier's 2.8103e-5 gives a ratio of 1.83.
- `scripts/run-wasm-gates.sh`; clippy; fmt; `scripts/check-unfused-seal.sh`.
- `Lane::div` is documented as "Audit every render-path use"; record the use in the module doc.

**Evidence.**
- Exhaustive output.
- The list of moved pins with old and new hex.
- The MQ-1 bench re-run, only if the shaper is still on the exact tier (R2 declined); otherwise the shaper does not consume `log2_lane` and no in-kernel number applies.

---

### MB-2 — Transient shaper → fast dB tier (class B) — **BLOCKED-ON-RULING R2** — tier X — depends on MQ-1 and MA-5

**Files.** `crates/transient-shaper/src/lib.rs` (`frame`, imports, module docs), `clippy.toml` (crossing reasons), `crates/math/tests/f1_fast_db_bounds.rs` (new `f1_crossing_x7_*`/`f1_crossing_x8_*`, and `f1_the_container_pins_exactly_six_crossings` becomes eight), `crates/transient-shaper/src/corpus.rs` (`CROSS_TARGET_DIGESTS`), `crates/transient-shaper/tests/oracle.rs`, `contract.rs` and `boundary.rs` (docs that cite `exp2_lane`/`log2_lane`), `docs/rulings/fast-db-tier-boundaries.md` (the transient-shaper boundary becomes adopted).

**Change.** In `frame`:
```rust
let contrast = fast_level_db(ratio);        // was: log2_lane(ratio).mul(L::splat(DB_PER_OCTAVE))
// …clamps and shape unchanged…
let gain = fast_gain_from_db(shape);        // was: exp2_lane(shape.mul(L::splat(OCTAVES_PER_DB)))
```
- This is exact: `DB_PER_OCTAVE` (`0x40c0_a8c1`) and `OCTAVES_PER_DB` (`0x3e2a_152d`) are bit-identical to `fast_db`'s `DB_PER_LOG2` and `LOG2_PER_DB`, and `fast_level_db`/`fast_gain_from_db` apply the same multiply in the same position.
- Add `// FAST-DB-CROSSING X7` and `X8` comments and `#[expect(clippy::disallowed_methods, reason = "FAST-DB-CROSSING X7/X8: …")]`, as the compressor's `curve_target` and `gain_mix` do.

**Domain proofs and bound.** MA-5 supplies them before the ruling. Here the worker:
- renames MA-5's tests to `f1_crossing_x7_*` (level, sweeping `[1e-8, 16]`) and `f1_crossing_x8_*` (gain, sweeping `[−18, 18]`), and raises `f1_the_container_pins_exactly_six_crossings` to 8, renaming the test to match;
- re-derives the header table in `crates/transient-shaper/tests/oracle.rs`, which is built on M1's ≤ 2 ulp, and checks the existing `2.0e-5` absolute row tolerance in `scalar_matches_the_independent_f64_oracle` and the `0.01` dB gates against MA-5's measured figures.
- **If a re-derived bound or measured row exceeds an existing tolerance, stop and bring the new tolerance to the owner as part of R2 (MA-5 should already have flagged it). Never widen a tolerance silently.**

**Gates.**
- `cargo test --locked --release -p transient-shaper`, including oracle, contract, boundary, partition and bank.
- `cargo test --locked --release -p math --features lane --test f1_fast_db_bounds` (default run: the container count and the crossing tests are not `#[ignore]`d) with X7/X8 present and the container count at 8, plus the same command with `-- --ignored` for the exhaustive sweeps.
- `CROSS_TARGET_DIGESTS` re-pinned via `MISO_ENGINE_REPIN_TRANSIENT_SHAPER_CORPUS=1` (all 3 rows are expected to move); `scripts/run-wasm-gates.sh`; clippy (the new `#[expect]`s must be fulfilled); fmt.
- The identity contract still holds: `fast_gain_from_db(+0.0) == 1.0` exactly (F1 `f1_identity_anchors_are_exact`) and the `shape == 0` dry select in `frame` is unchanged.

**Evidence.**
- The MQ-1 bench re-run against the MQ-1 baseline: same workload, same protocol, descriptive only.
- The moved-pin list.
- The re-derived oracle table.
- A null-difference measurement against the pre-change output on the MQ-1 programme (max |Δ| in dB and dBFS).

**Sequencing.** If R1 and R2 are both approved, land MB-2 before or in the same batch as MB-1, so `CROSS_TARGET_DIGESTS` moves exactly once; after MB-2 the shaper no longer calls `log2_lane`.

---

### MB-3 (optional) — `exp2_lane` refit E3 (class B) — **BLOCKED-ON-RULING R4** — tier M — depends on MA-3 and MB-1

**Operation order.**
```rust
clamp to [-126, 127];
r = (x + 12582912.0) - 12582912.0;
f = x - r;
f = (f - 0.5) + 0.5;
p = horner(P, f);
p = 1 + f * p;
p * exp2_int_in_range(r)
```
- The `(f − 0.5) + 0.5` step re-quantises f onto the grid that keeps the function monotone. Without it (variant E4, same coefficients), the research harness measured 83,682 one-ulp reversals, all for x ∈ (−0.5, 0).
- `P` (highest first) = `[0x392035d7, 0x3aaf9e7b, 0x3c1d9896, 0x3d635790, 0x3e75fdea, 0x3f317218]`.
- [measured, exhaustive, 2,247,753,730 inputs in [−126, 127]] max **1.2970 ulp** at x = −0.49866977 (`0xbeff51a5`); 0 decreasing steps; anchors exact.

**Moves:** `EXP2_DIGEST`, D1 `gain_from_db`, the `LANE_DIGESTS` `exp2_lane` row (regenerate with `cargo run --locked --release -p wasm-gates -- --print-pins`), the M1 exp2 header row, F1's exact-tier gain column (ratio still passes at 1.08× / 1.46×), and the transient-shaper pins if R2 is declined.
- **`m1_measured_worst_points`:** same structural edit as MB-1. Use a per-row floor, move the `exp2_lane` row to E3's worst point `0xbeff51a5` with floor `1.25` (measured 1.2970), and leave `MAX_ULP` unchanged.
- **Docs to update:** the `exp2_lane` provenance in `lane_math.rs` (published Cephes set → a committed fit record, as `fast_db.rs` has); `m1_exhaustive.rs`'s "Cephes fold removed" red-mutation row (re-run a fresh red mutation against the new body); and `fast_db.rs`'s opening sentence if MB-1 has not already changed it. Do not rewrite `docs/rulings/`.

**Not a speed change.** Throughput is equal or slightly lower than E1.

---

### MC-1 — Compressor ramp design options (decision brief, no code) — tier X — depends on MQ-2

**Goal.** Give the owner the R3 options, with the MQ-2 numbers.

**Options to cost.**
- **(a) Coefficient-domain ramp.** Compute `rate_coefficient` exactly at the ramp's start and target (2 `exp` per lane per event), then interpolate the coefficient linearly over the 64-sample `Linear 64` window.
  - This is the pattern of the true-peak limiter (release coefficient designed in `f64` at event time, linear ramp) and of the delay's damping `g`.
  - It changes the time-constant trajectory during a ramp: linear in coefficient rather than linear in milliseconds.
- **(b) Decimated recomputation.** Recompute every K frames and hold in between.
- **(c) A new lane-wide `expm1`**, per MA-4. The accuracy for long time constants (coefficients ≈ 1e-5) is the hard part.
- **(d) Status quo.**
- **(e) Per-block hold, the multiband pattern.** `multiband-compressor` refreshes its time-constant coefficients once per block in `band_coefficients` (`BandCache::refresh`, keyed on `[ratio, attack, release]`), not per frame. The compressor could evaluate `rate_coefficient` once per block from the ramp's current value and hold it for the block.

For each option state: rendered-bit impact, accuracy, worst-case block cost, and interaction with `advance_ramps`/`design_lane`'s `changed` mask.

**Deliverable.** A short brief appended to this issue for the R3 ruling.

---

### MC-2 — Implement the ruled compressor ramp design (class B) — **BLOCKED-ON-RULING R3** — tier X — depends on MC-1

**Files.**
- `crates/compressor/src/kernel.rs` (`advance_ramps`, `frames_loop`, `frames_loop_mono`) and `crates/compressor/src/design.rs` (`rate_coefficient`, `design_lane`).
- `crates/compressor/tests/ramps.rs` plus the compressor corpus pins `compressor::corpus::C1_DIGESTS` (`crates/compressor/src/corpus.rs`).

**Gates.**
- `cargo test --locked --release -p compressor`; compressor corpus re-pin with each moved row explained; `scripts/run-wasm-gates.sh`; clippy; fmt.
- Re-run MQ-2: the spike arm's per-block maximum is reported against the baseline.
- **Must land before #15 (De-esser) and #17 (Dynamic EQ) implement automatable attack/release**, so they copy the fixed pattern.

---

### MZ-1 — Delivery sync (per AGENTS.md "Delivery-control rules") — tier S

- Create `.github/ISSUE_SPECS/<this-issue-number>-<kebab-case-title>.md` containing this body, in the first checkpoint of the batch.
- After each pushed checkpoint, add concise evidence comments here.
- Close this issue only when every non-optional task is complete or explicitly declined by a recorded ruling, and GitHub is synchronized.
- At the issue boundary, compare `.github/ISSUE_SPECS/` with `gh issue list --state all`.

## Dependency graph and batch order

```
Batch 1 (class A; tasks in parallel worktrees, merged in this order):
  MA-1  math: lib.rs, vendored/sqrt.rs, VENDORED.md, sqrt tests
  MA-2  math: fast_db.rs docs, tests/f1_fast_db_bounds.rs; docs/rulings/fast-db-tier-boundaries.md
  MA-3  math: lane_math.rs (exp2_lane + module doc), new identity test
  MQ-1  transient-shaper: tests/bench.rs (new)
  MQ-2  compressor: tests/bench_ramp.rs (new)
  then MA-4 (lib.rs; after MA-1)
  then MA-5 (tests/f1_fast_db_bounds.rs; after MA-2) — its evidence goes to the owner with R2
Owner rulings R1–R4 (MA-5 and MQ-1 are the evidence for R2; MQ-2 → MC-1 for R3)
Batch 2 (class B, after rulings, one batch so shaper pins move once):
  MB-2 (R2)  ─┐  MB-2 edits tests/f1_fast_db_bounds.rs; MB-1 edits only F1's exact-tier column
  MB-1 (R1)  ─┘  → run sequentially, MB-2 first
  MB-3 (R4, optional) after MA-3 and MB-1
Batch 3 (class B):
  MC-1 (after MQ-2) → R3 → MC-2
Closing: MZ-1 (spec file in batch 1's first checkpoint; closure last)
```
- **File conflicts to respect:**
  - MA-2, MA-5, MB-1 and MB-2 all touch `f1_fast_db_bounds.rs`, so serialize them.
  - MA-3, MB-1 and MB-3 all touch `lane_math.rs`, so serialize them.
  - MA-1 and MA-4 both touch `lib.rs`.
- **How "parallel" works.** AGENTS.md allows at most one uncommitted implementation tranche in a shared worktree, and several agents editing `crates/math` in one tree would break each other's `cargo test -p math` gates even with disjoint files. So:
  - The coordinator cuts one `codex/batch-*` branch from the synchronized `main`.
  - Each parallel task runs in **its own `git worktree`**, on a task branch cut from the batch branch.
  - A task commits when its focused gates are green.
  - The coordinator merges completed task branches into the batch branch in the order listed above, re-running the affected gates after each merge.
  - If worktrees are not available, run batch 1 sequentially in that order.
- **Batching:** in CI-conscious batch mode, batch 1 accumulates local checkpoints on the batch branch and is pushed once. Per AGENTS.md, remove each task worktree after its branch is merged and its evidence preserved.
- **The main checkout may be in use.** Other agents work in `/home/bl/misofm/engine` (e.g. the #877 toolchain branch). Never switch branches, stash or reset there; create worktrees instead.

## Decision record: evaluated and not recommended

| Item | Evidence | Revisit when |
|---|---|---|
| Lower-degree fast tier | F-3: refits fail 3.2× / 2.5×; derived lower bounds make it impossible under F1 | the owner changes the F1 accuracy budget (errors would still be ~1000× below audibility, but the "≤ 2× the exact tier" rule is worth keeping) |
| Modernise the scalar libm (ARM optimized-routines port of `exp`/`log`/`pow`/`expf`/`powf`…, or CORE-MATH correctly-rounded `f32`) | All vendored functions are FreeBSD/Sun msun (rust `libm` 0.2.16 has no algorithmic change since). [isolated, separate C harness against the real `math` crate, FMA-free builds, busy host, ±20%] ARM optimized-routines ports would be 1.3–6.8× faster: `powf` 6.8×, `pow` 3.2×, `exp` 2.2×, `log`/`log2` 1.6×. The agent reported rust `libm` HEAD has no algorithmic change to these functions since 0.2.16 (source comparison, not re-verified here). But every caller is T2/T3/T4. `powf`'s only non-test call sites are `effect_contract::map_normalized` and `automation_segment_value`, and only tests and the `conformance` harness reach them (`conformance` is a dev-dependency of the effect crates and a dependency of `tools/audit`/`tools/bench` only). A swap also moves `M3_DIGESTS` and every coefficient-derived pin (EQ, compressor, limiter, soft-clip, delay) | a scalar function moves onto a T0/T1 path |
| FMA / relaxed-SIMD | F-6: +3.5% native console cost of unfusing; mixed in V8; relaxed SIMD may differ per implementation | never, under D5 |
| SIMD lookup tables | F-6 (estimate): about 10–20% native `exp2`, ≈ 0 in the browser; needs a new `Lane` permute primitive | a native-only product surface matters more than the browser |
| Drop monotonicity (E4) | F-6: no gain on AVX2, slower in V8 | — |
| E3 for speed | MB-3: not faster; accuracy only | — |
| `frexp_centered` `Lane` primitive (integer-offset reduction to [√2/2, √2)) | derived: L3 from 27 to 22 lane ops, same bits; needs integer `sub`/`shr_s`/convert lane ops, argued deterministic on AVX2/NEON/wasm | after MB-1, as its own `lane` issue with G1 coverage |
| Estrin / second-order Horner | [measured] without FMA they cost accuracy: exp2 1.683 / 1.501 ulp, log2 1.969 / 1.511 ulp; call sites are feed-forward, so op count matters more than latency | — |

## Hazards

- **Isolated microbenchmarks mislead.** They under-predicted the fast-dB win 17×, and here E3 has fewer ops yet is slower because the chains are latency-bound. Only MQ-1 and MQ-2 style in-kernel numbers may be quoted as render wins.
- **LLVM treats NaN payloads as unspecified** and deletes NaN-canonicalising guards around IEEE ops. Do not promise NaN bits.
- **Class A tasks that move a digest have failed.** Do not "fix" them by re-pinning.
- **`scripts/run-wasm-gates.sh`:** "A mismatch is never fixed by re-pinning." Pins come from the scalar oracle.
- **AArch64 NEON has no CI execution leg.** Identity there is argued from IEEE basic operations.
- **Stale comments exist.** `crates/compressor/tests/static_curve.rs`, `crates/compressor/tests/nonfinite.rs`, `crates/compressor/src/corpus.rs`, `crates/gate-expander/tests/oracle.rs` and `crates/gate-expander/src/lib.rs` still describe `log2_lane`/`exp2_lane`, but those effects run the fast tier. `.cargo/config.toml`'s header comment still says x86-64 is pinned to x86-64-v3 "so that `wide` selects its AVX2 backend and `Lane::fma` is the fused `vfmadd` instruction". The second half has been untrue since issue #163 phase 2 unfused the contract. Correct these opportunistically in a class-A doc edit, never in a class-B commit. `scripts/check-workspace-policy.sh` scans `.cargo/` for `target-cpu|target-feature|rustflags|RUSTFLAGS` and exempts only `#` comment lines. So change only the comment, keep it a `#` line, and re-run that script.

## Non-goals

- Changing F1's gate values or M1's `MAX_ULP` (2.0). Updating M1's recorded worst points and their per-row lower floors so they describe a new, more accurate body (MB-1, MB-3) is required bookkeeping, not a gate change.
- Changing the unfused contract.
- Touching EQ, limiter or builtins DSP.
- Third-party math crates.
- New `Lane` primitives (except as a separate follow-up issue).
- Promoting any isolated number to a release claim.

## Target matrix

| Target | Coverage |
|---|---|
| x86-64-v3 native (`Simd8`, plus `Scalar`/`Simd4` oracles) | CI: `qualification.yml` including the M1 and F1 exhaustive jobs |
| wasm32 scalar and `+simd128` | `scripts/run-wasm-gates.sh` |
| AArch64 NEON (`Simd4`) | argued only; attach results if hardware is available |

## Appendix: exhaustive-sweep method (for MA-2, MA-3, MB-1, MB-2, MB-3)

- **ulp metric:** identical to `m1_exhaustive.rs::f32_ulp`. The ulp of `want` is `next_f32(|want|) − |want|` in f64, clamped at the subnormal spacing. Error = `|got − want| / ulp(want)`.
- **Oracle:** `math::exp2`/`math::log2` (f64) applied to `f64::from(x)`.
- **Domains:**
  - exp2: finite x in `[−126, 127]` (2,247,753,730 inputs);
  - log2: finite, positive, normal x (2,130,706,432 inputs).
- **Monotonicity:** walk bit patterns in order. A pair is decreasing if `(x_prev < x && y_prev > y) || (x_prev > x && y_prev < y)`; the second clause covers negative inputs, whose values fall as bit patterns rise.
- **F1 metric:** dB error of the applied gain or of the level, over F1's inclusive bit ranges.
- **Parallelism:** iterate `u32` ranges and never collect patterns. Include thread-boundary pairs whenever a count is pinned.
- **Exactness checks** (e.g. whether an f32 subtraction rounded) use an error-free transformation such as Knuth TwoSum in f32 (`s = a + b; bb = s − a; err = (a − (s − bb)) + (b − bb)`, exact iff `err == 0`) or integer arithmetic. Never compare against an f64 result (see F-5).
- **Measured runtime of the audit harness:** 41 s wall on 32 threads (EPYC 7313P) for the floor-exactness count, the E1 identity, five width-identity sweeps and six M1 sweeps.


## Execution record — 2026-09-25

- Coordinator accepted the bounded task scopes and objective gates above. The user's explicit
  workflow is GPT-6 Luna xhigh implementation followed by GPT-6 Astra xhigh adversarial review;
  corrections repeat that flow, within the two-attempt budget per task.
- Batch branch: `codex/batch-880`, based on synchronized `origin/main` `e09302ad`. The primary
  checkout is untouched. Independent task worktrees preserve class-A gate isolation.
- First implementation tranche: MA-1, MA-2, MA-3 independently. MA-4 and MA-5 follow their
  dependencies; MQ-1/MQ-2 and MC-1 supply concrete owner-decision evidence. Class-B implementation
  remains blocked until the corresponding owner ruling is recorded. Optional MA-1b is omitted.
- Local issue-boundary audit: 445 existing numbered/spec Markdown files compared with 557
  GitHub issues; no numbered local spec lacks a matching remote issue. #880 is open and its
  title matches this spec's filename. No unrelated issue state was changed.
- Checkpoint commits accumulate locally for the issue's declared class-A batch; delivery evidence
  and remote issue state will be synchronized at the coherent batch boundary.

### Integrated class-A checkpoint validation

On `abd2c1e0` (MA-1/2/3/4 integrated), the coordinator ran:

- `cargo test --locked --release -p lane -p math -p wasm-gates -p effect-runtime -p transient-shaper --features math/lane`: PASS.
- `scripts/run-wasm-gates.sh`: PASS, native backend 2 / wasm scalar backend 0 / wasm SIMD backend 1; each leg ran 141 cases and 355 comparisons, with zero digest or min/max-lowering mismatches. Detector residency checks passed.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: PASS.
- `bash scripts/check-workspace-policy.sh`, `bash scripts/check-unfused-seal.sh`, and `bash scripts/check-lane-policy.sh`: PASS.

Task-local exhaustive results and codegen evidence accompany the implementation checkpoints.
MA-5, MQ-1, MQ-2, MC-1 and independent Astra review are still pending; this is not issue closure.

### MQ-1 post-workload tooling failure

The single authorized run at `2a8977f5` completed its warmup and two rounds. Its raw output reports
Simd8 6.281854 / 6.281717 ns per lane-sample and scalar 40.841292 / 40.803620. The runner then
failed to extract the result because libtest prefixed the `MQ1_RESULT` line. No timed retry was
performed. Raw evidence and the failed disposition are preserved; record repair/promotion is
bounded successor [#902](https://github.com/misofm/engine/issues/902), not a DSP blocker. These
are descriptive shared-host measurements, not a claimed render speedup.

### Decision evidence and review handoff

- MA-5 (`fc20ea6e`): all 257,176,458 in-domain ratios and four attack/sustain sign corners
  measured max post-clamp contrast difference `1.525879e-5` dB, max gain difference
  `1.654115e-5` dB, and unit-input fast gain error `1.287460e-5` (exact tier
  `2.199415e-6`). The existing `2.0e-5` oracle tolerance is sufficient. Low/high
  out-of-domain rail proofs pass; named production crossings remain six.
- MQ-2 (`e0c57d6f`, evidence `f16c88cf`): exactly one timed invocation, with one
  warmup and two measured rounds. Mean/max microseconds per block: release-only
  `169.448/175.995`, `169.313/174.312`; attack+release `215.761/220.029`,
  `216.534/220.700`; no automation `40.739/45.918`, `40.833/45.236`. State-derived
  call counts: 8,192 / 16,384 / 0 respectively. Record validation passed.
- MC-1: `docs/issue880-mc1.md` compares all five designs, including state restoration,
  changed-mask behavior, and block-partition constraints. No proposed design has been timed.
- Luna xhigh supplied implementation/evidence. Fresh Astra xhigh review is underway on
  the complete authorized class-A batch and briefs; no verdict is recorded yet.
- R1/R2/R3/R4 were presented to the owner with these concrete findings; no response
  or ruling has been received. MB-1/2/3 and MC-2 remain untouched and blocked. The
  umbrella issue must remain open after class-A delivery unless rulings resolve all
  required work.

### Final class-A adversarial verdict

Astra xhigh attempt 1 passed every reviewed task except MA-3: its new E1 integration
test lacked Cargo's `required-features = ["lane"]` registration, breaking standalone
`cargo test --locked --release -p math`. Luna xhigh corrected that registration in
`51c1fd1b`, with no numeric or pin changes. Astra independently re-ran the default
math suite and all 2^32 E1 patterns and recorded **MA-3 attempt 2 PASS; authorized
class-A batch PASS**, with no remaining review blocker. The full verdict is
`docs/issue880-astra-review.md`. The two-attempt MA-3 budget is exhausted successfully.

Both integrated benchmark preflight self-tests passed without timing; integrated
compressor tests, final workspace clippy/fmt and all policy checks passed. Original
raw logs retain their trailing blank lines byte-for-byte. R1–R4 remain pending,
so this PASS authorizes class-A delivery only, not closure of the umbrella issue.

### Required artifact delivery successor

PR #903's first qualification run `36084071070` passed every test job except the
shipped AudioWorklet artifact check: reviewed code-generation changes moved the
compiled binary identity. Successor #904 qualified the exact accepted source and
new artifact `772b65111a22fa07135dd3f90628774a5587ef6891e14da25f28a2989d3d4d56`
through all existing static/resource/SDK/real-receiver and three-browser gates.
Luna xhigh implemented and Astra xhigh recorded attempt-1 PASS. Numerical corpus
pins are unchanged; only the compiled artifact and live test identity changed.
The qualified delivery is part of the same PR; pending owner rulings still keep
this umbrella open.

## Owner rulings — approved 2026-09-25

The owner replied **“Approved.”** to the concrete recommended choices after class-A
delivery. This records explicit authorization to proceed, not an inferred ruling:

- **R1 APPROVED:** adopt L3 for `log2_lane` (MB-1).
- **R2 APPROVED:** admit X7/X8 and move the transient shaper to the fast tier (MB-2).
- **R3 APPROVED:** coefficient-domain compressor ramps, exact endpoints and 64-sample
  interpolation, with compatible state/restore handling and unchanged accuracy and
  block-partition gates (MC-2).
- **R4 DECLINED:** omit the optional accuracy-only exp2 refit (MB-3).

### Class-B execution brief

Base: merged class-A delivery `a5cb5d8e`, branch `codex/batch-880-class-b`.
User-selected workflow remains Luna xhigh implementation, then Astra xhigh adversarial
review; corrections repeat that flow, within the existing two-attempt budget per task.
MB-2 precedes MB-1 so transient-shaper pins move only once. Compressor work is isolated
from math/shaper changes. Preserve all frozen accuracy gates, unfused arithmetic and
realtime/partition contracts. Coefficient ramp state/restore design must be explicit;
necessary narrowly scoped compressor state/metadata/test integration is part of R3's
approved compatible handling, not permission to silently discard active ramp state.

#902 parser repair/promotion is a separate bounded tooling tranche before the MB-2
comparison: preserve the original baseline bytes and do not retime that baseline.
Each approved new implementation gets only its specified new one-warmup/two-round
measurement after its harness is frozen and preflighted. Final source changes require
new exact-source AudioWorklet qualification through existing gates before delivery.
All checkpoints remain local until a coherent reviewed batch boundary.
