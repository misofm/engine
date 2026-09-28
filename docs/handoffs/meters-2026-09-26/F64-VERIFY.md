# Adversarial verification of #949 (f64 lane vocabulary) and #950 (banked full meter pass)

Base: `origin/main` = `252622b6` (detached scratch worktree, removed afterwards). Host: AMD EPYC
7313P, x86-64-v3, rustc 1.97.1, node 22.23.2. Release profile (fat LTO, cgu 1). Timings pinned
with `taskset` to a single core, one runtime, A/B in the same process. `CARGO_INCREMENTAL=0`.
Prototype of both briefs, written independently from the brief text (not from the research
patch): `f64-verify-prototype.patch` next to this file (throwaway; nothing committed).

## 1. #949 exactness

| check | result |
|---|---|
| widen, all 2^32 `f32` patterns, `Simd4` and `Simd8`, native, vs the brief's integer-construction oracle, `black_box` on the vector result | 0 mismatches; 1.63 s / 1.61 s per width on 4 threads |
| same sweep with F-2 (subnormal widen -> +0) | 16,777,214 mismatches at `Simd4` (red) |
| F-1 (`Simd8` lanes 4..8 first) | red in the meter-block identity |
| widen, all 2^32 patterns, both widths, **under V8 `simd128`** (guest cdylib, in-module integer oracle) | 0 / 0 |
| widen, scalar wasm leg (`-simd128`), 1/256 sample | 0 |
| `add`/`mul`: every ordered pair of a signed-zero/subnormal/MIN_POSITIVE/1+eps/MAX/inf/NaN/tie pool + 1,000,000 random bit-pattern pairs, `f64x4`, `f64x8`, `f64` | 0 |
| square witness (`widen(x)*widen(x)` vs `M*M * 2^(2E)`), pool + 100,000 finite values | 0 |
| `meter_block::<Simd4/Simd8>` vs independent scalar ALL oracle and vs `::<f32>`; frames 1,2,3,127,128,129; 64 carried blocks; hostile, tone, random-bits input; `peak == meter_sample_peak_block` | 0 |
| `meter_block::<Simd4>` under V8, 2,000 streams x 64 carried blocks | 0 (simd128 and scalar legs) |

### Fusion
* No `vfmadd*` in any linked fat-LTO kernel (`zz_pass8`, `zz_pass4`, `zz_energy_only8`).
* LLVM IR (release, `-C lto=off`) of the same kernels: vector `fpext <4 x float> to <4 x double>`,
  `fmul <4 x double>`, and **zero** `contract` flags, `llvm.fmuladd` or `llvm.fma`. rustc leaves
  `AllowFPOpFusion` at `Standard`, under which the x86 DAG combiner and the AArch64 machine combiner
  fuse only contract-flagged or `fmuladd` nodes. That is the mechanism `Lane::fma` already relies on
  (`wide_impl.rs` writes `(self * b) + c`; `check-unfused-seal.sh` + `lane-source.toml` keep
  `mul_add`/fused intrinsics out). It carries over unchanged to `f64`. `lane-source.toml`'s fusion
  regex (`_mm256_fmadd|vfmaq|mul_add`) already matches the `_pd`/`_f64` spellings as prefixes.
* The energy is doubly safe: `w*w` is exact, so even a fused `fma(w, w, e)` would give the same bits.

### Lowering
* x86 fat LTO, per `Simd8` frame: 2 `vcvtps2pd`, 1 `vextractf128`, 2 `vmulpd`, 2 `vaddpd`;
  `Simd4`: 1/1/1 on `ymm`. The non-LTO pipeline vectorises too (IR above).
* wasm `simd128` guest: brief's probe = 2 `f64x2.promote_low_f32x4`, 2 `f64x2.mul`,
  2 `f64x2.add`, zero scalar `f64.*`. Real `meter_block::<Simd4>` probe: same f64 census plus
  `f32x4.abs/ge/ne/pmax/add`, `v128.bitselect`, zero scalar `f32`/`f64` arithmetic.
* **host-web AudioWorklet artifact** (built with the delivery flags, prototype `bank_meter_pass`
  wired into `observe_unit`): 12 each of `promote_low`/`f64x2.mul`/`f64x2.add` (Simd4 arm 4 + dead
  Simd8 arm 8), no forbidden scalar op. `--kernel-shape ... --kernel-min 11` still passes.
* **Robustness gap**: the only required pin is #949's guest probe. #950's M5 census is "record".
  Callgraph rule 3 counts only `f32`. And the render allocation/trap callgraph closure is 8
  functions: the graph executor is reached through `call_indirect`
  (`PreparedPlanExecutor::render`), so no required gate inspects any graph code in the artifact.
  Native has no pin at all. A toolchain that scalarised production `bank_meter_pass` (or the
  native widen) while the energy-only probe stayed vector would pass every required gate. This is
  perf only; the bits are the same either way.

## 2. #950 exactness (randomised differential)

Arm A: `observe_input`. Arm B: `banked_seed` (first `Some` in binding order), `meter_block` per
plane, `observe_input_banked`. All snapshot fields are compared by bits (`to_bits` for floats).
Configurations: periods {1,64,127,128,129,300,512,1536,4096} x 9 metric sets x 4 hold/decay x
{hostile, tone} x {fixed 48x128 with a 7-frame skip, random 150-event stream: skips, restarts,
both reset kinds, zero-frame blocks, sizes 1..511}.

| arm | snapshots | banked commits | mismatches |
|---|---:|---:|---:|
| `f32` | 1,667,802 | 37,183 | 0 |
| `Simd4` | 6,815,288 | 148,588 | 0 |
| `Simd8` | 13,608,560 | 298,416 | 0 |
| `Simd8`, two identical meters per member (twins) | 27,217,120 | 596,832 | 0 |
| `Simd8`, two different meters per member | 13,775,160 | 887,954 | 0 |
| `Simd4`, two different meters per member | 6,897,588 | 443,066 | 0 |

* Counters (8 ALL meters, 64 x 128 frames, tone): 512 at /512, 0 at /64, 512 at /1536, 296 at
  /300. These match the brief exactly.
* Liveness: K-1 (seed + zero-seeded partial) gives 12,055 mismatches. A pass run from zero seeds
  but reporting the true seed gives 20,054 mismatches.
* **Twins.** A second meter on the same member commits a result computed from the *first*
  meter's seed whenever the bits are equal. It is value-exact (596,832 commits, 0 mismatches).
* **Seed pollution.** A member holding `[SAMPLE_PEAK meter, ALL meter]`: the peak meter answers
  `banked_seed` with its never-accumulated energy (+0.0). Graph stops at that first `Some`, so the
  ALL meter commits only at window starts: 640 commits of 1,024 (the ALL meter gets 128 of 512).
  With the order reversed, all 1,024 commit. The result is exact but silently falls back.
* The bit check validates the seed only, not the lane, plane or frames the result came from. A
  lane-mapping bug with equal seeds (for example both +0.0 at a window start) would commit wrong
  words. G-1 and G-2 in M3 are the only guard, which is adequate but should be stated.
* Held (hold 0, decay off), counts and peak: exact by the domain argument (sanitised values are
  `{+0} U normals`, so equal values have equal bits; `>=`/`>` select-max is associative there;
  `hold_remaining` is always 0; `decay_enabled` is false for both `0.0` and `-0.0`). The
  differential confirms it.
* `settled_silence`: the banked commit adds `+0.0` to `energy >= +0.0`, so the bits are identical.
* **Brief inconsistency.** Item 7 routes every `block.meter == Some` to `observe_input_banked`, and
  item 5 hands `meter: Some` to *every* final member. So `SAMPLE_PEAK`-only meters on a
  full-pass unit commit through the banked path, not P1's merge. M3's mixed control ("P1 fast
  merges are above 0") is then unachievable as written.

## 3. #714

The text in #714's spec (`.github/ISSUE_SPECS/714-*.md`):
* line 68: "batch observers across failure boundaries, precompute or publish another observer's
  state" sits in the list of slice prohibitions, next to "insert metering as an executing DSP
  stage";
* line 167: "scalar `f64` square/add energy ... No energy reassociation, `f32` substitution, new
  `f64` lanes";
* lines 250-253: "Across-track vector meter state ... belong to separately scoped successors. New
  `f64` lane arithmetic requires an owner ruling."

Code contract: `GraphRuntimeObserver::observe_resident` (graph `lib.rs:2141-2148`) says "None
declines without mutation ... accept once". `observe()` (`runtime.rs:3602`) runs observers in
binding order with `?` short-circuit.

**Verdict: #950 respects the rule.** The seed is a `&self` read of the meter's own state. The
kernel is a pure function of that seed and the unit's final resident words. Each meter re-checks
window, eligibility and seed bits after its own preamble and commits its own lane, in binding
order. Nothing is published before the observer runs, and a failure stops every later commit. P1
(#943) already uses the same "graph computes, owner merges" pattern while listing the #714 clause
as kept. #714 itself names across-track vector meter work as a successor and gates `f64` lanes on
an owner ruling, which the 2026-09-26 ruling now supplies. What #950 must keep from line 167 (no
reassociation, per-lane sample order) it keeps, and K-1 proves the gate discriminates.

Gaps: #950 does not cite that the ruling supersedes #714 line 167. I4 ("a failing observer leaves
later observers untouched") has no gate. The twin case commits a value seeded from a sibling's
state; it is bit-equal, but if the letter matters, the smallest fix is: graph records the binding
index that supplied lane `l`'s seed, and only that observer consumes `energy` (others fall back
for energy). That costs nothing in shipped configurations (one meter per track).

## 4. Saving (in-process, pinned core 27, 3 rounds x 192,000 blocks, tone, ALL, hold 0, decay 0)

| configuration | scalar ns/bank/block | seeds + pass + commits |
|---|---:|---:|
| `Simd8`, period 512 | 3,355 / 3,366 / 3,356 | 711 / 703 / 701 |
| `Simd8`, period 1536 | 3,368 / 3,330 / 3,329 | 687 / 687 / 687 |
| `Simd4`, period 512 | 1,745 / 1,674 / 1,676 | 485 / 486 / 489 |

The direction is reproduced (4.7-4.9x at `Simd8`, 3.4-3.6x at `Simd4`); absolute numbers run
about 7 % faster on this run. Estimate: 8 x (3.35 - 0.70) = ~21 us per 64-track block, against
#950's 22 us. #943 F2 has the scalar per-bank cost x 8 = 29-30 us of the 31 us in-situ delta, so
the estimate is plausible at about 20-23 us. The row that moves is `console_meters`: its arm uses
`prepare_session_builtins` (ALL, `PostMatrix`, 4x128, hold 0, decay 0, permanent observers, so
`observe_unit`). `sixty_four_track_console_metered` (`SAMPLE_PEAK`) must not move.

## 5-6. Scope, layering and realtime

* `Observed<L>` (`crates/builtins/src/corpus.rs:605`) is the only `Lane` impl outside
  `crates/lane`, so separate `LaneF64`/`Widen` traits are correct. Associated-type defaults are
  unstable, so a defaulted method on `Lane` is not an option.
* `lane-source.toml`'s fusion rule forbids `wide::` under `tools/`, so the corpus must name the
  companions as `<Simd4 as Widen>::F64`. That works, and graph does the same.
* `UnitIdentity`: 4 flag bytes + P1 + #950 = 6, + 8 (two u32) + 16 (`Box<[_]>`) = 30 <= 32.
  `rt9_identity_metadata_has_no_retained_or_peak_layout_delta` compares against a 32-byte `Before`.
* The graph dependency list is unchanged (`lane` is already there). No `unsafe` is needed.
  `GraphResidentObservationBlock` is only constructed at `runtime.rs:3684/3758`.
* The G6 FTZ test (`tools/wasm-gates/tests/g6_full_corpus_ftz.rs`) calls `native_report()` under
  DAZ, but it asserts only digest mismatches. The new `f64_lane_mismatches` field (nonzero under
  DAZ) is ignored there, so it cannot go red. Fine.
* The prototype `#[inline(never)] bank_meter_pass` in the host-web artifact carries 2
  `slice_index_fail` + `unreachable` from `words[..frames * W]`. It is invisible to the trap gate
  (see §1), but it is avoidable: `let Some(w) = words.get(..frames*W) else { return };` with
  `frames = w.len() / W` measured panic-free and still vectorised. P1 has the same shape.
* Counts in `f32` are exact only for `frames <= 2^24`. Session validation checks only
  `quantum_frames != 0` (`crates/session/src/validate.rs:52`), and I found no quantum ceiling
  elsewhere. This is inherited from `sanitize_gain_block`'s documented premise.
* AArch64 is unmeasured (no aarch64 std/rust-src on the host, and no aarch64 CI job at all). The
  meter widens only sanitised values, so it is immune to FPCR.FZ. NEON `vaddq/vmulq_f64` are IEEE,
  and the contraction argument above holds there too.
