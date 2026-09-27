# Limiter: drop the four frame operations the output never depends on


Limiter slice 5 of 5 (research 2026-09-27, base `codex/batch-plumbing-floor-2` at `6ca203f8`; every
`file:line` below was read on that tree). Evidence:
`docs/handoffs/effects-2026-09-27/LIMITER-DIAGNOSIS.md`, section 3 (the recount) and section 4
item 5. The prototypes are `V_MAXDIV`, `V_HOIST` and `V_NOZERO` in
`docs/handoffs/effects-2026-09-27/limiter-diagnosis-prototypes.patch`. They are evidence only; do
not commit them.

## Product outcome

The recount finds 7 lane-ops per lane-sample in the frame and detector bodies that no rendered bit
depends on:

| site | today | proposed | lane-ops saved |
|---|---|---|---:|
| gain computer (`channel_frame_uniform`, `crates/true-peak-limiter/src/lib.rs:1635`; `channel_frame`, `:1391`) | `select(p > l, l / p, 1)` | `l / max(p, l)` | 1 |
| stereo link (`:2022-2024`, `:1843-1845`) | `select(link, max(p_R, p_L), p_X)` on the prepared `coef.link_max` | a block-level arm on `coef.link_max` | 1 |
| output (`:1665`, `:1422`) | `select(bypass, z, z * g)` on the prepared `coef.bypass` | a block-level arm on `coef.bypass` | 1 |
| Annex-2 seed (`annex2_phases`, `:1063`) | each accumulator starts `+0.0` and adds its first product | start at the first product | 4 |

The measured saving is small natively: -0.1 to -0.17 cycles per lane-sample at `Simd8` and -0.6 to
-0.8 at `Simd4`. That was measured together with the ramp-scatter skip, which `limiter-4` owns.
Under V8 it is -0.14 to -0.23 (fat build: -1.7), where each `v128.bitselect` is three instructions.
The saving applies to every block, and it takes the implemented inventory from 132.5 to the
class-A minimum of 125.5 (`limiter-4` removes no floor op). These figures are descriptive only.

## Proofs (each is the class-A argument the doc comment must carry)

1. **`select(p > l, l/p, 1) == l / max(p, l)`**, where `max` is the D8 `select(p > l, p, l)`:
   * If `p > l`, both sides are `l / p`.
   * Otherwise `max` is `l`, and `l / l` is exactly `1.0`. `l` is finite and nonzero: it lies in
     `[10^(-25/20), 10^(-1/20)]`, the domain of `limit_coefficient`, `:880`.
   * A NaN `p` fails `p > l` in both forms and gives `1.0` in both.
   * A `+inf` `p` gives `l / inf = +0.0` in both.
   * `p = +/-0.0` gives `1.0` in both.

   So every word is equal, NaN included.
2. **Prepared-boolean arms.** `select(all, a, b) == a` and `select(none, a, b) == b` by bits. A
   branch on the same prepared boolean, taken once per block, is the same function.
3. **Annex-2 seed.** Let `S_k` be the accumulator with the `+0.0` seed and `T_k` without it. Then
   `S_0 = +0.0 + a_0` and `T_0 = a_0` differ only when `a_0 = -0.0`, giving `+0.0` against `-0.0`.
   By induction, `S_k` and `T_k` are equal, or are zeros of opposite sign:
   * `x + a` equals `y + a` whenever `a` is nonzero or NaN;
   * when `a` is a zero, both results are zeros.

   The phases feed only `peak.max(phase.abs())` (`:1045`), and `abs` erases the sign. A NaN
   propagates through the same operands in the same order in both forms.

## Invariants

* **Class A.** Every output word, state payload and report is unchanged, at `f32`, `Simd4` and
  `Simd8`, on every target. The phase values themselves may differ in the sign of a zero. They are
  not state, and nothing but `abs` reads them.
* The arms are chosen once per block, never per lane or per frame. They are monomorphised
  (`const LINK: bool`, `const BYPASS: bool`), or taken as a loop-invariant branch that the release
  disassembly shows unswitched. If it is not unswitched, monomorphise.
* Both bodies take the substitutions: uniform (`:1621`) and per-lane (`:1373`), dual and mono.
* Allocation-free, no `unsafe`, no layout change.

## Interface contract

1. `required_gain<L>(peak, limit) -> L { limit.div(peak.max(limit)) }`, `#[inline(always)]`, used
   by both frame bodies.
2. `annex2_phases` seeds each accumulator with its first product. The old form stays, under
   `#[cfg(test)]`, as the E1 oracle `annex2_phases_seeded`.
3. The link and bypass arms, per the invariants.

## Smallest closable slice

Authorized paths: `crates/true-peak-limiter/src/lib.rs` (the functions named and the tests module),
`crates/true-peak-limiter/tests/MUTATIONS.md`, and this spec. `floor.rs`, the `jq` restatement and
the ruling move only with the owner's recount ruling (diagnosis section 7 item 1). They are not
part of this slice.

Steps:

1. **On the base:** gate 4's scenario and its pinned digest.
2. Contracts 1-3.
3. The gates.

## Objective gates

1. **Gain-computer truth table**, in dev and in release. At `f32`, `Simd4` and `Simd8`, compare
   `select(p > l, l / p, 1)` against `l / max(p, l)` bit for bit:
   * `l` takes every value on the ceiling grid of the parameter domain (-24 to 0 dB in 1/32 dB
     steps, through `limit_coefficient`);
   * `p` takes:
     * `+/-0.0`, `+/-2^-149`, `+/-f32::MIN_POSITIVE`;
     * `l` itself, `l` one ulp either side, and `1.0`;
     * `+/-inf`, and NaN with two payloads;
     * 10^5 seeded values.

   NaN words are compared as "both NaN". The x86 `maxps` and the wasm `pmax` operand order are
   exactly what may commute here.
2. **Seed.** Keep E1 (`phase_outputs_match_the_frozen_scalar_order`, `:3661`) testing the oracle
   `annex2_phases_seeded`. Add E1b: `detector_peak` with the new seed against the seeded oracle's
   peak, by bits. Inputs:
   * the E1 noise;
   * impulses of `+/-0.0` and `+/-1.0` at each tap;
   * histories of mixed signed zeros;
   * subnormals;
   * one NaN.

   E2 (`bs1770_annex2_conformance_is_unchanged`) stays green unchanged.
3. **Existing identity gates,** in dev and in release:
   * `tests/determinism.rs` (D90) and `wasm-gates` G5;
   * the cohort tests at `:4605` and `:4633`;
   * `tests/gain_law.rs`, `tests/mono_collapse.rs`;
   * every console workload's 64-block digest.
4. **Scenario pinned on base.** Bank API, 96 blocks at W8 and W4:
   * `Maximum` and `DualMono` banks;
   * bypass on and off (a bypassed bank is prepared, per `LimiterCoef::new`, `:421`);
   * limiting noise and quiet tone.

   One SHA-256 of output words and payloads, recorded on `6ca203f8`.
5. **Mutations**, each recorded red:
   * M1: `peak.max(limit)` with the operands swapped (`limit.max(peak)`). Gate 1 goes red on a
     NaN peak, which then propagates instead of giving `1.0`.
   * M2: the link arm keyed on `!link_max`. Gates 3 and 4 go red.
   * M3: drop the first product instead of the seed. E1b goes red.
   * M4: the bypass arm inverted. Gate 4 goes red.
6. **Realtime and wasm.** `tests/allocation.rs`, `check-realtime-policy.sh`, `check-lane-policy.sh`,
   and `check-web-audioworklet.sh`: the roster row stays one function, and the rule-3 count does
   not drop. The vector count falls, which the shape gate absorbs.
7. **Codegen (recorded).** The uniform frame loop loses its six `vblendvps` and gains two
   `vmaxps`. The detector loop loses four `vaddps`. Quote both.

## What the implementer will hit

* **`Lane::max` operand order is load-bearing.** It is `SRC1 > SRC2 ? SRC1 : SRC2` on x86, and
  `pmax(b, a)` on wasm (`crates/lane/src/wide_impl.rs`). `peak.max(limit)` returns `limit` for a
  NaN peak; `limit.max(peak)` returns NaN.
* **The measured native saving is within run-to-run noise at `Simd8`.** This slice is justified by
  the floor, the wasm instruction count and every-block applicability, not by a timing claim.

