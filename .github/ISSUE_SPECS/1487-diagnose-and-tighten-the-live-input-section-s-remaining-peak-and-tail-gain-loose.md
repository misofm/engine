# Diagnose and tighten the live input section's remaining peak and tail gain looseness

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-08 by root order: root's ruling of 2026-10-08 on *Tighten the live input section's
peak gain toward the real-kernel history peak* (#1485; its amended title, pending GitHub sync, is
*Tighten the live input section's peak and tail gains toward the real kernel's supremum*)
attempt 1. That slice closes with a
partial tightening (the live `G_p` 22.0 dB and the live `G_t` 34.1 dB tighter); this issue owns the
looseness that stays. **Low priority: it starts after *Define how node tails compose through gain
in the graph extent* (#1379) has landed.** The code this issue reads is created by #1485; verify
every anchor on the branch where #1485 and #1379 have landed before starting.

## Product outcome

First, a diagnosis: the attempt record names, per launch rate, which part of each live gain bound
carries the remaining gap to its exact reference. Then, if a tighter certified bound exists, the
live input section states a `G_p` closer to the real kernel's largest measured history peak and a
`G_t` closer to the adjoint oracle's exact row-`l1` supremum, never below either, with the same
gates as #1485. Every decibel of `G_p` raises `up(v)`, `k(v)` and `X*(v)` in #1379's composition,
and every decibel of `G_t` raises the residual #1379 carries through every live strip. No kernel,
law, render path or rendered bit changes.

## Context

- **What #1485 ships** (its Attempt record, attempt 1): `math::tail::LiveBound::{exposure,
  peak_gain, tail_gain}` in `crates/math/src/tail.rs`; derivation in
  `docs/derivations/1379-graph-tail-composition.md`, "(N1): `G_p` and `sigma_p`" and "(R) The late
  part along the in-flight ramps" / "`G_t`". Stated values (millibels), at 44.1 / 48 / 88.2 /
  96 kHz: `G_p` 12,224 / 12,215 / 12,224 / 12,215; `G_t` 5,824 / 5,825 / 5,829 / 5,830. `D`,
  `T_decay`, `T_rest`, both rests, `P*`, `sigma_p` and `sigma_t` are #1433's, #1466's and #1467's.
- **The remaining gaps** (#1485 gates 2 and 2t, release, every launch rate, ratio of the stated
  gain to its reference): `r_p` = 21.66 / 21.56 / 21.66 / 21.56 (+26.7 dB) against `g_meas`, the
  largest real-kernel peak over gate 1's seven histories (5.9750e4 at 44.1 kHz, per unit of the
  input's peak at the +24 dB trim word, from the three-event worst-sign history that #1485's
  follow-ups 2 added to gate 1; a finite search found it, so `g_meas` is a measured lower bound of
  the true worst-case peak); `r_t` = 13.57 / 13.59 / 13.66 / 13.68 (+22.7 dB) against
  `s_scan`, the largest trim x exact row-`l1` supremum over #1467 L2's scan (6.016e1 at 44.1 kHz).
  #1485's factors: `F_p = 22.75`, `F_t = 14.37` (`F_P`, `F_T` in
  `crates/builtins/tests/tail_contract.rs`).
- **The parts of each bound** (#1485's L3 and L3' recomputation, 44.1 kHz). `G_p`, per unit of the
  trim word: `delta A` 7.27e3 (the HPF's largest output `A` through the LPF's feedthrough), the
  fast part 6.07e3, the potential term 3.02e4 (`1/2 |m| Gammabar(m0) sup Psi Phi`,
  `V = sup Psi Phi = 1.60e5`) and the slow charges 3.82e4 (`C = 5.25` weighted by
  `sum Gamma_s`), at `m0 = 16`, against the measured 3.77e3. `G_t`, with the trim: the window
  part `W* g` 433.7 and the settled part `Z* g` 815.8 (the settled part sets `G_t` at every rate),
  against `s_scan` 60.16. These are the candidates; nothing yet says which of them is loose
  against its own exact counterpart.
- **The references.** For `G_p`: `peak_history` and the gate 1 and 2 test
  `live_peak_gain_bounds_the_real_kernel_on_its_worst_histories` (`tail_contract.rs`). For `G_t`:
  #1467 L2, `live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`,
  with `exact_row_supremum`, `forward_response`, `record_history` and `l2_scan`.
- **The contract** each value keeps: #1379 Amendment 1 H1 (N1) and (N2), as amended by *Split a
  node's flush stall into a peak stall and a tail stall* (#1484), for every admitted history of
  live trim, polarity and filter targets (#1407's retarget words, #1408's trim ramp), every reset
  pattern of the joint flush and every channel. The registry rule (f) of #1464: `G_t <= G_p`.
- **Cost** (#1485, root's cost condition): the live bound depends only on the rate; preparation
  reads the compiled constant `input_section_live_bound_table` (`crates/builtins/src/tail.rs`),
  and the test `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` holds it equal to
  the computed bound. The construction (about 244 ms per rate in release, about 5 s in debug) is
  test time only.

## Decisions frozen for this issue

- **T-D1. Diagnosis first.** Before any bound changes, the attempt record states, per rate, each
  additive part of `g_p` and of `g_t` next to the exact or measured value of the quantity that part
  bounds, and its ratio:
  - `G_p`: the HPF's largest output `A` against the real kernel's largest first-section output on
    gate 1's histories; the first-section state supremum `V` against the largest state the real
    kernel reaches in the same norm; the LPF's exposure `sum Gamma` against the exact `l1` of the
    LPF's time-varying response over the same histories' words; and so for the fast part, the
    potential term and the slow charges.
  - `G_t`: the window part `W* g` against the exact supremum of the late input restricted to the
    window frames (`M ..= M + 63`), and the settled part `Z* g` against the exact supremum of the
    input after the window, both by the adjoint oracle on L2's histories.
  The record names the part (or parts) that carries most of `r_p` and of `r_t`, with the numbers.
  The diagnosis also runs an exhaustive or optimised worst-schedule search for the real kernel's
  peak, because #1485's `g_meas` comes from a finite grid of event schedules.
  The diagnosis probes are one-off evidence (one invocation, recorded); a probe is committed only
  if it becomes a gate under T-D4.
- **T-D2. Method.** If the diagnosis points to a tighter bound, the implementer's, proven in the
  derivation note before the code, for every admitted history and every reset pattern, computed on
  the control thread in `f64` with `math::log`/`math::exp`, rounded up to millibels.
- **T-D3. Never below the reference.** A certified `g_p` below `g_meas`, or a certified `g_t`
  below `s_scan` at any launch rate, is unsound and stops the issue.
- **T-D4. Factors from measurement, confirmed by root** (as #1485 V-D3): the new `F_p` and `F_t`
  are each the measured `r_max` times `1 + 5 %` (or another stated margin with its reason), rounded
  up to two decimals; root confirms each before its gate is committed.
- **T-D5. Only the live `G_p` and `G_t` move.** The live `T_decay`, `T_rest`, both rests, `P*`,
  `D`, `sigma_p` and `sigma_t`, and every fixed-section value stay as they are. `G_p >= G_t` raw
  and in millibels.
- **T-D6. Stop rule.** If the derivation cannot prove a tighter bound for a gain, that gain stays
  as #1485 states it; the diagnosis record closes the issue for it. There is no fallback value
  and no gate change.
- **T-D7. Cost.** The table stays a compiled constant checked by the table test; nothing computes
  the bound at boot or preparation. The record states the construction time per rate (release,
  one invocation) and the debug and release `tail_contract` times, and that every gate still runs
  in a required `qualification.yml` job within its timeout.

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade under #1407's live words (#1433's zone analysis); (N1), (N2);
  #1485's derivation; T-D2.
- **Coefficient and update rules:** #1407's retarget through designs and their mixtures; #1408's
  trim ramp; no change.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; no tail or rest value moves (T-D5).
- **Units and smoothing:** gains in millibels; no smoothing.
- **Denormal/NaN:** unchanged.
- **Citations:** as #1329, #1433 and #1485 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP],
  [SMITH-SASP]; Higham for the rounding model).
- **Fixtures and objective tests:** gates below. **Benchmarks:** none (T-D7 is descriptive).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. The T-D1 diagnosis in the attempt record.
2. If a tighter bound is proven: the live `G_p` and/or `G_t` in `crates/math/src/tail.rs`, rounded
   in `crates/builtins/src/tail.rs`, `input_section_live_bound_table`'s `peak_gain` and/or
   `tail_gain` restated, the derivation updated, and the recomputation tests (#1485 L3 and L3')
   restated to the new formulas.
3. The graph digests and asserted tails that #1379 prints and that a tighter value moves, each
   re-pinned one at a time with its reason.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate. Gates 1 to 3 apply only to a
gain that moves.

0. **Diagnosis.** The attempt record carries T-D1's table for every launch rate and names the part
   that carries the gap, for each gain.
1. **Sound on the real kernel's worst histories** (#1485 gate 1, its seven histories, widened with
   any history the diagnosis finds worse, each with its reason, never narrowed).
1t. **`G_t` sound against the adjoint oracle** (#1467 L2, its scan unchanged or widened, never
   narrowed).
2. **Within the new factor of measured, never below it:** `g_meas <= g_p <= F_p g_meas`.
2t. **Within the new factor of the supremum, never below it:** `s_scan <= g_t <= F_t s_scan`.
3. **Nothing else moves.** Every other `tail_contract` assertion passes unchanged; `G_p >= G_t`
   raw and in millibels; the table test passes with only the moved gains restated; no rendered bit
   moves (`audit capi`'s `pcm_digest`, the wasm G5 digests, the builtins PCM fixtures).
4. **Commands:** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
   --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked --release -p
   builtins --features builtins/test-support --test tail_contract`; the `test-debug-a` workspace
   command of `qualification.yml`; `cargo build --locked --release -p audit &&
   ./target/release/audit capi`; `bash scripts/check-builtins-fixtures.sh . target/release/audit`;
   `bash scripts/run-wasm-gates.sh`; `bash scripts/check-graph-determinism.sh`; `cargo run --locked
   -p graph-compiler --bin graph_fixture -- --check`; `cargo test --locked --release -p host-core
   --test tail_composition -- --include-ignored`; `bash scripts/check-workspace-policy.sh`;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

The attempt record carries a mutation table (each defect applied, the named test run, the file
restored) for every gate that moves: #1485's bound restored for a gain that moves is red on its
gate 2 or 2t, and a bound that drops a term the derivation needs is red on gate 1 or 1t, or on the
recomputation.

## Test value

- Gate 2 / 2t (restated): a step that does not tighten (#1485's bound left in place) misses the new
  factor; no other test bounds the live gains from above against their references.
- Gate 1 / 1t: a tightening that drops a term the history needs falls below the measured peak or
  the exact supremum.
- Gate 3: a change that moves `D`, `T_decay` or a stall with a gain is red on #1433's, #1466's and
  #1484's pins.

## Non-goals

- Any kernel, flush or law change; `INPUT_BOUND_BUDGET_FRAMES` or the section charge.
- The fixed section's `G_p` (#1468); the live stalls, rests and `D`.
- A wider oracle than the diagnosis needs: the scan grows only by histories with a stated reason.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions,
  stream A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live `G_p` section and "The live tail
  gain `G_t` and the statement")
- `crates/graph-compiler/tests/track_delay.rs` (the digest), `crates/graph-compiler/tests/tail_composition.rs`
  (an asserted value that moves, with its reason), `fixtures/graph/v1/direct-route.*` (regenerated
  by `graph_fixture`), `fixtures/graph/MANIFEST.tsv`, `crates/host-core/tests/live_lanes.rs` and
  `crates/host-core/tests/tail_composition.rs` (an asserted tail that moves, with its reason)
  (named exceptions, stream A's)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Hazards

- **Every admitted history.** The gates measure a few histories; the derivation, not the gates,
  carries the proof, and the verifier checks it as a proof. A bound that matches the measured
  histories but drops a zone, window or settled-group term is unsound.
- **The diagnosis is not a bound.** A ratio near 1 for one part on the scanned histories does not
  prove that part tight on every history; it only says where to look.
- **Shared files.** `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs` and `tail_contract.rs`
  are shared with #1468 and #1379 (`STREAMS.md`, hot files); this issue lands after #1485 and
  #1379, in either order with #1468, the later slice rebases.
- **Canonical digests.** #1379's canonical text prints each node's gains, so a tighter value moves
  digests and some reported tails (shorter only); any other moved byte stops the issue.
- **CI time.** The debug `tail_contract` binary takes about 67 s on four cores of an x86-64
  workstation after #1485 (about 28 s before it; 43 s on the `test-debug-b` runner before it); a costlier construction must stay inside the
  required jobs' timeouts (`test-debug-b`, `aarch64-debug`, `test-release`).

## Dependencies

- *Tighten the live input section's peak gain toward the real-kernel history peak* (#1485):
  the bound this issue diagnoses, gates 1, 1t, 2 and 2t, and the recomputation it restates.
- *Define how node tails compose through gain in the graph extent* (#1379): lands first (low
  priority); the digests and tails this issue re-pins.
- *Derive the live input section's tail gain and state its composition* (#1467): the adjoint
  oracle and its scan.
- *Split a node's flush stall into a peak stall and a tail stall* (#1484): (N1)'s `sigma_p`.
- In either order with *Tighten the fixed input section's peak gain past the cascade triangle
  inequality* (#1468), the later slice rebases.

## Attempt record

### Root decision (2026-10-24)

The scoper marked the issue NEEDS-OWNER with three options: (a) close with #1485's values, (b) the
T-D1 diagnosis only, (c) diagnosis plus tightening. Root chose (b): the T-D1 diagnosis only, no
product code, scratch-only probes. Gates 1 to 4 do not apply to this slice.

### T-D1 diagnosis (2026-10-24, on `main` 506196ce5)

**Method.** I used a scratch copy of `tail_contract.rs` (never committed; deleted after the run).
It has four parts: the L3 recomputation oracle, changed to print each term; an exact `f64` adjoint
split of the worst histories into the two sections; #1467 L2's scan with its rows split at the
window's end; and a worst-schedule search. All values are per unit of the trimmed input `G`, and
"x trim" means times the +24 dB trim word 15.849. Release build, one invocation per probe.

**`G_p`, the terms of `g_p / g`** (m0 = 16 at every rate):

| rate | `delta A` | fast | potential `h Gbar(m0) V` | slow `C sum Gamma_s` | sum | exact cascade sup |
|---|---|---|---|---|---|---|
| 44.1 | 7.266e3 (8.9 %) | 6.068e3 (7.4 %) | 3.016e4 (36.9 %) | 3.816e4 (46.7 %) | 8.165e4 | 3.771e3 |
| 48 | 7.210e3 | 6.022e3 | 2.981e4 | 3.774e4 | 8.078e4 | 3.749e3 |
| 88.2 | 7.266e3 | 6.068e3 | 3.016e4 | 3.816e4 | 8.165e4 | 3.771e3 |
| 96 | 7.210e3 | 6.022e3 | 2.981e4 | 3.774e4 | 8.078e4 | 3.749e3 |

The shares are the same to 0.1 % at every rate.

**Each part against its exact counterpart** (44.1 kHz; the other rates are within 1 %). The
"exact" column is the time-varying cascade's adjoint split on gate 1's histories: the LPF's row
`h2(n, m)` weighted by the HPF's exact row `l1` at each input frame. On the worst history (row
+42), this split is 4.523e3, only 1.20 times the cascade's own 3.771e3.

| part | bound | exact | ratio | reason |
|---|---|---|---|---|
| `A` (HPF output) | 7.266e3 | 2.482e3 (L1 history, row +6) | 2.93 | `max_k (h q_k + omega) Phi_k` pairs each zone's largest state with its exposure |
| `Phi` at the top | 5.445e4 | 3.112e4 (`sup` of the HPF state in the `V`-norm at the event) | 1.75 | 1.39 from `rho_ramp` (`1 - rho` 3.67e-5 against the top design's `1 - |p|` 5.23e-5; `rho_settled` gives 5.09e-5), 1.26 because the input signs cannot hold the rotating state aligned |
| `sum Gamma` | 7.265e3 (96 % of it is the tail past `H`) | 1.473e3 (`sum |h2|` over lags of 1 or more) | 4.93 | the long-lag level `Gamma` 0.257 to 0.267 against the exact `|h2|` 0.102 to 0.114 (2.3 to 2.5 times), and the `rho_ramp` mass (1.39) |
| fast (lags below 16) | 6.068e3 | 716.5 | 8.5 | `A` against `Gammabar` differences |
| potential + slow (lags of 16 or more) | 6.832e4 | 3.672e3 | 18.6 | see the next two rows |
| slow: `C` | 5.25 | 1.09 (the top HPF's settled `l1`, the steady output per frame) | 4.8 | `C` charges every input frame at the top with `h Psi beta` (4.15 of the 5.25). This is the mass that a later retarget could expose, but at the top it decays, it is not output |
| potential: `h Psi` per unit of state | 2.08 | 0.83 (the HPF's exposed output summed over 400 frames, per unit of state) | 2.5 | `Psi` is a scalar potential over the zone paths |

**Which part carries `r_p`.** The potential and the slow charges carry 83.6 % of `g_p`, and they
are 18.6 times their exact counterpart. If they were exact, `delta A` would move `G_p` by at most 0.5
dB, the fast part by at most 0.6 dB, and both together (16.3 % of `g_p`) by at most 1.2 dB. The section split is not the gap: the exact split is 1.20
times the cascade (1.6 dB). The gap is the frequency-blindness of the scalar `V`-norm zone
constants at the top of the domain (`Psi`, `C`, `Gamma`'s long-lag level, `Phi`), plus the 1.39
`rho_ramp` mass in `Phi` and in `Gamma`'s tail.

**Worst-schedule search** (44.1 and 48 kHz; 88.2 and 96 kHz repeat them to five digits
everywhere else). The start is the worst-case pair. I ran 3,000 random schedules of 1 to 4 retargets
(frames 0 to 63, 21 cutoffs from 10 Hz to the maximum), then a greedy local refinement of the best
8 (event times, cutoffs, sections, adding and removing events). Polarity and trim are not searched:
a flip changes only the input's sign, which the worst-sign input already sets, and the trim ramp
never goes above its maximum. Best: `(+4 HPF 0.95 max, +7 HPF 0.95 max, +20 HPF 0.9 max, +49 LPF
500 Hz, +52 HPF 10 Hz)`, row +55. Exact x trim 5.9906e4 at 44.1 kHz and 5.9558e4 at 48 kHz, that is
0.26 % above `g_meas` (so `r_p` is 21.60 and 21.50, not 21.66 and 21.55). The five-event history
is a gate 1 widening for any tightening slice. It needs no change while #1485's bound stands.

**`G_t`, the terms of `g_t / g`:**

| rate | window `W*` | settled `Z*` (sets `G_t`) | exact window rows (n < 64) x trim | exact settled rows (n >= 64) x trim |
|---|---|---|---|---|
| 44.1 | 27.36 (433.7 x trim) | 51.47 (815.8) | 41.41 | 60.16 |
| 48 | 27.36 | 51.53 | 41.34 | 60.17 |
| 88.2 | 27.36 | 51.82 | 40.77 | 60.12 |
| 96 | 27.36 | 51.85 | 40.82 | 60.12 |

The window part is 10.5 times its exact counterpart, and the settled part 13.6 times. At every rate,
`Z*` comes from the pair of settled zones `k = l = 121` (`Re p` in [0.948, 0.965]). Its parts at
44.1 kHz are: the exposure `h_2 q_l + omega_2` 1.39 times (`S*_l` 15.39, cross term 9.51, steady
term 8.13), plus `delta` times the first output, 5.56. The shares are 41.6 % `S*`, 25.7 % cross,
22.0 % steady and 10.8 % `delta` output. Exact: at the settled rows, the in-flight states add
nothing measurable. The largest settled row (60.1642) equals the largest settled design's `l1`
(60.1641). The settled bound alone (steady plus `delta` output, 16.86) is 4.4 times the exact
settled cascade `l1` (3.796). The `V`-norm `l1` bound of one settled section is 3.90 for both
sections in zone 121. The exact section `l1`s are 2.39 to 2.43 (HPF at 10 to 200 Hz) and 1.09 to
2.43 (LPF).

**Which part carries `r_t`.** It is the settled part. The in-flight terms `S*` and cross carry 67 %
of `Z*`, and the exact settled rows show no in-flight excess. The module adds the separate suprema
of the three state terms (`S*` decays from the window's end while the steady term builds up). A
scratch time-resolved form gives 27.20 / 27.22 / 27.32 / 27.33: the sup over `m` of the same
majorant recursion's solution, `E_m = Ebar + (E* - Ebar) r_k^m`, `S_(m+1) = r_l S_m + beta_l (c_k
E_m + delta)`, over every settled pair. This is an estimate on a dense grid of `m`, not a proof.
The sup is at the settled phase's first frame. At every rate it is below the window part, so
`G_t` would become `W*`: 433.7 x trim, -5.5 dB, `r_t` about 7.2.

**Candidate tightenings.** None moves a rendered bit unless the table says so. Each one moves
reported tails and #1379's canonical digests.

| id | gain | change | expected effect | cost | class |
|---|---|---|---|---|---|
| C1 | `G_t` | the settled phase time-resolved (the sup over `m` of the recursion's closed form per pair, with its stationary point) in place of the sum of separate suprema | `G_t` 815.8 to about 433.7 (-5.5 dB, `W*` then binds), `r_t` about 7.2, every rate | M (the formula is S; then the derivation, the oracle, the table, `F_T`, and the #1379 digests and tails re-pinned) | A |
| C2 | `G_t` | the window phase (`RampPaths`) frequency-aware, after C1 | below `W*`, which is 10.5 times its exact counterpart; no estimate | L | A |
| C3 | `G_p` | `rho_settled` in place of `rho_ramp` for a long dwell at the top | at most 1.39 times on each term that scales with `Phi` or with `Gamma`'s tail: -2.3 dB (potential and slow only) to -2.9 dB (every term) | M to L. It needs a proof that in-flight words cannot hold the top zone's worst-corner contraction (an in-flight re-send keeps a word in flight indefinitely). Soundness is not shown | A as a bound. It touches render if done by changing the ramp arithmetic |
| C4 | `G_p` | frequency-aware zone constants (a per-zone basis or a vector majorant for `Psi`, `Phi`, `Gamma`) | the 2.3 to 4.8 factors above; not estimated | L+, a research re-derivation of #1433's zones | A |
| C5 | `G_p` | `delta A` or the fast part made exact | at most -0.5 dB and -0.6 dB (-1.2 dB together) | M | A; not worth it |
| C6 | `G_p` | one joint zone DP on `(||s1||, ||s2||)` over pairs of zones, in place of `Gamma` and the potential | scratch result 1.97e6 per unit against the module's 8.17e4: 24 times looser. Rejected | - | - |

**Recommendation.** For `G_p`: keep #1485's values (T-D6). The bounded candidates (C3, C5) move
`G_p` by at most about 2.9 dB and 1.2 dB, C3's soundness is not shown, and the real gap (C4) is research.
For `G_t`: one bounded tightening slice, *#1487-B: time-resolve the live `G_t`'s settled phase*
(C1). Its scope is `LiveBound::tail_gain` and the oracle's `G_t` recomputation, the derivation's
"`G_t`" section, the table's `tail_gain`, `F_T` restated from the new `r_t`, and the #1379 digests
and tails that move. Its gates are 1t, 2t, 3 and 4 as written, plus the five-event history in gate
1. The slice measures how much the reported live tails actually shorten. If root prefers no more
work in this program family, closing with #1485's values for both gains is sound. The cost is
`G_t` staying 5.5 dB looser than C1 would make it.

**Surprises.** (1) The two-section split costs only 1.20 times. An obvious joint DP is 24 times
looser, so #1485's potential is essential, and the gap is frequency-blindness, not the split.
(2) The search moves `g_meas` by only 0.26 %, so the measured reference is robust. (3) At the
settled rows, the in-flight terms add nothing measurable but carry 67 % of `Z*`. After C1 the
window part binds, with a margin of only 0.1 % (96 kHz) to 0.6 % (44.1 kHz) over the time-resolved
settled sup. (4) The LPF at the top has `l1` 2.43, not about 1. The largest settled cascade is the
HPF at 200 Hz into the top LPF.

### Root decision on the diagnosis (2026-10-24)

Root accepted the recommendation. **`G_p` closes with #1485's value under T-D6**: no bounded
tightening has a proof (C3's soundness is not shown, C4 is research, C5 is at most 1.2 dB), so
`G_p` stays 12,224 / 12,215 / 12,224 / 12,215 mB with `F_P = 22.75`, and no gate changes for it.
`G_t` takes slice #1487-B (C1).

### #1487-B attempt 1 (2026-10-24): the time-resolved settled phase of `G_t`

**Change.** `LiveBound::tail_gain`'s settled part per pair `(k, l)` is now `settled_pair_peak`
(`crates/math/src/tail.rs`). It walks the majorant recursion from `(E*_k, S*_l)` frame by frame,
rounded up, and its tail bound `T_H` covers every later frame. #1485's sum of three separate
suprema is gone. The proof is in the derivation note, "Settled, from `M + R`". Gate L3''s oracle
recomputes the settled part in closed form (`settled_pair_sup`: the recursion's solution is
`K0 + K1 r_k^s + K2 r_l^s`, and its derivative has at most one zero). The window part, the
first-section row, `G_p`, `D`, the stalls and the rests are unchanged (T-D5).

**`G_t` per rate** (raw, with the trim; the window part now sets it at every rate):

| rate | `W* g` | `Z* g` (oracle) | margin `W*/Z* - 1` | `G_t` raw | `G_t` mB (was) | `r_t` stated |
|---|---|---|---|---|---|---|
| 44.1 | 433.681 | 431.168 | 0.58 % | 433.681 | 5,275 (5,824) | 7.2138 |
| 48 | 433.679 | 431.457 | 0.51 % | 433.679 | 5,275 (5,825) | 7.2128 |
| 88.2 | 433.681 | 432.962 | 0.17 % | 433.681 | 5,275 (5,829) | 7.2190 |
| 96 | 433.679 | 433.108 | 0.13 % | 433.679 | 5,275 (5,830) | 7.2190 |

The margin is thin, but nothing depends on it. If the settled part ever exceeds the window part,
`G_t` takes the settled part, and gate L3' recomputes both parts.

**Re-pins (old -> new).**
- `input_section_live_bound_table` `tail_gain`: 5,824 / 5,825 / 5,829 / 5,830 -> 5,275 at every rate.
- `F_T` (gate 2t): 14.37 -> 7.58, that is `r_t,max` 7.2190 times `1 + 5 %`, rounded up (T-D4). It
  waits for root's confirmation.
- Gate 1 gains the T-D1 search's best five-event history (row +55). With it, `g_meas` is 5.9890e4 /
  5.9542e4 / 5.9891e4 / 5.9542e4, +0.23 % over the old values, and `r_p` is 21.61 / 21.51 / 21.61 /
  21.51, still within `F_P`.

No #1379 digest and no asserted tail moved. The graph fixtures, `track_delay`'s digest, and
host-core's `tail_composition` and `live_lanes` passed unchanged. So no other pin was restated.

**Reported tails.** I measured with a scratch per-rate copy of host-core's `live_lanes` C8 (the
nine-track live-controls fixture with its bus, every strip with an input lane, never committed).
I ran it with the old table and with the new one:

- `output_tail` and `output_tail_every_peak` are unchanged at every rate (44.1 kHz: 2,388,258 and
  4,769,994 under `ALL`; 2,014,834 and 3,452,204 with the input lanes only).
- Only the graph's flush floor `P*_graph` moved: 3,509 / 3,505 / 3,515 / 3,528 mB -> 2,960 / 2,955 /
  2,961 / 2,973 mB under `ALL`, and -4,889 / -4,893 / -4,883 / -4,870 -> -5,431 / -5,436 / -5,430 /
  -5,418 mB with the input lanes only. That is about -5.5 dB at every rate.
- The EQ fixture's tail stays infinite (its EQ bound is unstated).

The tails do not move because an input section's `G_t` enters only the `down` gain of the nodes
upstream of it, and the extent rounds `A` up to whole decades (`k = ceil(A / 2000)`). On these
fixtures, the 549 mB drop does not cross a decade.

**Class A.** The change is to bound code only, and no render path is touched. Every render digest
gate passed unchanged (gates below). Preparation cost is unchanged: preparation reads the table.

**Red first and test value.**
- With #1485's module (`git show e6cd552d9:crates/math/src/tail.rs`) and the new tests, three tests
  are red at every rate:
  - gate 2t: stated `g_t` 816.6 > `F_T s_scan` 456.0, ratio 13.57 at 44.1 kHz;
  - gate L3': module 815.8 against the recomputed 433.68;
  - the table test.
- Test value: gate 2t and L3' turn red when the settled phase is not time-resolved (#1485's bound
  left in place); no other test bounds `G_t` from above.
- Gate 1's new row is red alone under a `g_p / 21.63` mutant (44.1 kHz: kernel peak 5.9890e4
  against `g_p + sigma_p` 5.9829e4; the seven older rows pass). Test value: a `G_p` that covers the
  three-event history but not the search's five-event one.
- I restored each mutant with `git checkout` and `touch`.

**Render digests.** `audit capi`'s `pcm_digest` is `bffc21e857b5e134` both on this branch and on
the base (e6cd552d9, built in a scratch worktree). `check-builtins-fixtures.sh`, `run-wasm-gates.sh`
(G5), `check-graph-determinism.sh` and `graph_fixture --check` pass unchanged.

**Gates** (all pass):
- `cargo fmt --check`;
- the gate 4 DSP debug tests (`-p lane -p math -p builtins -p dsp-reference`);
- release `tail_contract --include-ignored` (20 tests);
- release host-core `tail_composition --include-ignored`;
- the `test-debug-a` workspace command (on cd27acb60; the later commit changes only a test's
  history list and the docs);
- `audit capi`, `check-builtins-fixtures.sh`, `run-wasm-gates.sh`, `check-web-audioworklet.sh`,
  `check-graph-determinism.sh` and `graph_fixture --check`;
- `clippy -D warnings` on math, builtins, host-core, graph-compiler and builtins-compiler;
- AArch64 `cargo check --tests` on math and builtins;
- `cargo doc -D warnings` on math and builtins;
- `check-workspace-policy.sh` and `lint-checks.sh`.

325 tests passed in the gate run, and none failed. Release `tail_contract` takes about 11 s.

### Verdict and close (2026-10-24)

The verifier's verdict on #1487-B attempt 1 is FAIL. The bound is sound: each step is a one-step
majorant, every step rounds up, and the value is a bound at every stopping frame. But two defects
remain:
- a required CI test goes red: the multiband compressor's pinned live table
  (`crates/multiband-compressor/tests/tail_contract.rs`, `G_t` 6,259 -> 5,735 mB at 44.1 kHz,
  through `live_cascade_composition`);
- no test exercises `settled_pair_peak`: a `return 0.0` mutant passes everything, because the
  window part binds.

Root's decision is to close #1487 as measured, not shipped:
- **`G_p`** closes with #1485's value under T-D6.
- **`G_t`'s C1** is sound and gives -5.5 dB (5,824-5,830 -> 5,275 mB). It is not shipped, because:
  - it moves no reported tail on today's fixtures (a 549 mB drop does not cross a decade of the
    extent);
  - it moves only the graph's flush floor, and no production path reads that value (only
    `GraphCompileReport` and the canonical evidence text read it);
  - a ship would also restate the multiband compressor's table and the #1373 derivation row, and
    would need a direct test of the settled part.

Commits `cd27acb60` and `468ea3234` hold the code, and the branch history keeps them. Commits
`ac60aa2c0` and `559d1691f` revert them. The diagnosis (`e6cd552d9`) and this record stay.

**Reopen condition:** a production path reads the graph's flush floor, or a session's `G_t` drop
would cross a decade in a reported tail.
