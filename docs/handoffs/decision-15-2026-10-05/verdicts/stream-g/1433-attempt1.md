PASS

# #1433 attempt 1 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: `git diff 52fed320a d7f593851` on `codex/d15-stream-g` (26ba151d7, 1a74a9edd,
563395413, d7f593851; 7 files, every one inside the spec's authorized paths). I checked it against
the spec (D0-D4, the two-level `P*` candidate, gates 1-4), AGENTS.md, decision 15 D15-4 and the
owner's no-shortcuts rule. All work was done on exports (`tree` = d7f593851, `base` = 52fed320a);
nothing was edited, built or committed in the worktree. Small evidence is in
`/tmp/claude-1002/v1433/evidence/`.

**Verdict.** I re-derived the frequency-aware cascade step by step and found no unsound step. An
independent 50-digit recomputation (my own script, written from the derivation) reproduces every
certified figure at all four rates, with the module above it by the stated inflation. Adversarial
real-kernel histories that release the HPF's resonant state into the LPF (LPF state at `N` up to
2.31 times the top pair's) rest at most at 986,624, below every certified `peak_plus_24_dbfs`.
#1329's `omega_state` finding is real, and #1329's shipped bound absorbs it (verified). Every spec
gate and every repository gate I was asked to run is green. 15 of 17 mutations are red. There is no
BLOCKER and no MAJOR finding.

There are two MINOR findings:
- **m1.** #1329's envelope row `output_state` still uses `omega_state` at the largest words, which
  this slice proved is not a supremum. The value is sound only through slack that no document
  states.
- **m2.** No test defends the joint-flush `X` terms or the use of the pole step in the neighbour
  sets. Both mutations stay green under the whole `tail_contract`.

## BLOCKER

None.

## MAJOR

None.

## MINOR

**m1. The envelope's output row keeps the `omega_state` that #1433 showed is not a supremum; its
soundness rests on an unstated slack argument.**

- *Where.* `crates/builtins/src/tail.rs:382` computes `omega_state` at the largest words, and
  `:387` uses it in `output_state: gamma_design + row_correction + omega_state`. The doc comment at
  `:290-292` says "the rounding counts take every word at its largest magnitude plus `E + h`". The
  derivation's D5 output-row bullet (`docs/derivations/1329-input-section-tail-and-rest.md:142-145`)
  does not mention the output rounding at all. The fix for #1433's own term is at `:409-418`
  (`first_output_rounding = omega(largest) + omega(corner)`). The finding is recorded only in the
  spec's attempt record.
- *The bug is real (verified).* `output_rounding` has `U |1 - c1|` and `U |1 - a3|` terms, which
  are largest at small `c1`, `a3`. On a 200,001-point grid of `f32` designs per rate
  (`evidence/omega_check.py`), designs exceed #1329's value (8.036e-7) from 2,848 Hz to 14,711 Hz
  at 44.1 kHz (3,100-16,013 Hz at 48 kHz, scaled at 88.2/96 kHz). The largest is 8.708e-7 for an
  8,583.5 Hz HPF.
- *The shipped #1329 bound is still sound (verified).* For fixed mix words,
  `||c(w)||_V* + omega_state(w)` is convex in the recursion words (a norm of non-negative convex
  terms), so over the designs' hull and the `E + h` box it peaks at a design vertex. Over the grid
  the peak is the 10 Hz HPF, 1.951e-5 below `output_state` at every rate. That margin is
  `row_correction` (about 1.94e-5) plus `omega(largest) - omega(10 Hz) > 0`. Where `omega` exceeds
  #1329's value, `||c||_V*` lies at least 1e-3 below its 10 Hz value. So this is not an Item for
  ROOT about an unsound shipped bound, but the shipped value depends on an argument that no document
  states.
- *Why MINOR.* This slice owns `builtins/src/tail.rs` and the derivation, found the defect, and
  left a false proof statement in place. #1433's own bound reads `output_state` (the LPF output
  row, `gamma mu` in `c_d`, `a_F`). Either fix is cheap: use `omega(largest) + omega(corner)` in
  `output_state` too (about 1e-6 on 1.4135; the figures should move by at most a frame), or write
  the absorption argument into the derivation and the doc comment.

**m2. Two soundness terms of the new bound have no test: the joint-flush one-off terms `X` and the
neighbour radius (the pole step's use).**

- *Mutations* (`evidence/mutations_verifier.log`, `evidence/mutations_full_v2v3.log`):
  - V2 sets `settled_start`'s `X_0 = 2 c_y H` to zero (`crates/math/src/tail.rs:1378`).
  - V3 computes `first_neighbour` and `last_neighbour` without `step` (`:971-978`), so only touching
    zones are neighbours.
  - Both are GREEN under the oracle test and under the whole release `tail_contract` with the
    ignored gate (9/9 pass).
- *Why no test sees them.*
  - The oracle compares only the final figures, inside a band of `+0.01 % + 64` frames above a value
    that the module already exceeds by 17-43 frames by construction.
  - At the launch configuration these terms move the figures by at most a frame. In my Decimal
    variants, touching-only neighbours give -1 frame. Dropping `X` gives 0 frames on `T_decay` and
    the rests. Dropping `X` and its constant moves only the 96 kHz `R(P*)` (-415) and `P*` (-2 %),
    and both stay below `T_decay`.
  - M2 defends the step's *value* (kernel moves against `zones.step`). Nothing defends its *use*.
- *Why MINOR.* These terms are soundness steps of a helper that the EQ and crossover slices may
  reuse (#1372, #1373). There, a dropped `X` or a narrowed neighbourhood can matter. The spec's test
  value names "a telescoping step that loses a boundary term". A cheap remedy: have the oracle also
  compare intermediate quantities at a tight relative tolerance (`sup Psi Phi`, `K`, `K_L`,
  `sup Phi`, per-group `sigma` at the HPF's rest frame, `X_0`). It already computes them. Another
  remedy: a `math::tail` unit test on a constructed `LiveCascade` where the HPF's settled output row
  is large.

## NIT

- **n1.** `crates/math/src/tail.rs:1223` says "lie in one octave of `1 - r`". The code groups by
  `4 log2(1 - r)` (quarter octaves), as the derivation (`:209`) and the spec record say.
- **n2.** `builtins::input_section_live_envelope` (`crates/builtins/src/tail.rs:313`) now returns
  `LiveCascade` (the envelope plus the pole domain, the mix row and the output rounding). The name
  was kept because `builtins/src/lib.rs` is outside the paths. The doc comment covers the new
  meaning, and "envelope" in the wide sense (suprema over reachable words) still fits, so this is
  not misleading. A rename belongs to a slice that owns `lib.rs` (Items for ROOT).
- **n3.** #1433's new gate-2 assertion (`tail_contract.rs:1504-1518`) has no unique catch:
  - Its lower side `R_meas <= R` follows from the same loop's existing `measured <= bound` (`:1503`;
    the 64-frame block rest is at least the exact frame).
  - Its upper side `R <= 1.15 R_meas` is weaker at every rate than gate 3's new
    `peak_plus_24_dbfs <= 1_075_000` (`:1595`; that is a ratio of 1.093-1.099).
  - The spec requires the assertion, and its value is the recorded `R_meas` and ratio. I note this
    for future briefs rather than ask for a change.
- **n4 (inherited from #1329 D5, harmless).** "An `f32` design is within the half-ulp box `h` of the
  exact design" ignores the `f64` evaluation error of `SvfSection::design` (`t1 / den` and
  `g^2 / den` carry about 4 ulp of `f64`, about 4.4e-16 next to `c1`, `a3` near 1). The zone pads
  (`1e-15` in `reach`, `POLE_PAD = 1e-12`) cover the `Re p` shift. The `|p|` and `kappa` images use
  only `SLACK` (about 6e-17 at this size). The effect is far below every other margin, and gate
  1(b) checks every scanned design's certified norms against its zone. The premise wording should
  read "within `h` plus the design's `f64` error" (or the box should be widened by a few `u64`).

## What I re-derived (soundness)

- **Pole form.**
  - I checked `R A R^-1 = (Re p) I + (Im p) J + kappa K0` entry by entry with
    `Re p = 1 - c1 - a3`, `Im p = 2 sqrt2 a2 - c1 + a3` and `kappa = c1 - a3 - sqrt2 a2`.
  - `K0` has eigenvalues `+-sqrt2` and `K0^2 = 2I`. This gives the bounds on `A`, `I + A` and
    `I - A`. For `A^2 - I` it adds `2 |p| sqrt2 |kappa| + 2 kappa^2`.
  - For an exact Butterworth design, `kappa = 0` and `|p + i|^2 = 2` hold symbolically.
  - On the arc, `|p|^2 = 1 - 2 Im p`, so every hull point has `|p0| <= 1` (this is used in
    `square_norm`).
  - `hull_sum` increases on `(-1, 1)` (derivative `2 + 2x / sqrt(2 - x^2) > 0`). `hull_difference`
    decreases, and `hull_radius^2 = 3 - 2 sqrt(2 - x^2)` increases in `|x|`. So each zone constant
    taken at one end of the zone is a supremum.
  - The map from the words to `(Re p, Im p, kappa)` is linear and invertible (determinant
    `2 sqrt2`). So the hull of the designs maps exactly, and a reachable word is a hull point moved
    by the `E + h` box (#1407; the `f32` designs' hull lies within `h` of the exact one by
    convexity).
- **Pole step, worst case.** From the kernel's word update (`lane/src/kernels/builtins.rs:1182`
  `filter_ramp_words`: `c + s` on leading frames, `t - s * remaining` after, `t` when done):
  `|dRe p| <= |Re p(t) - Re p(c)| (1 + u/2) / 64 + about 10u` for every reachable start `c` and
  design target `t`. That is at most `(high - low + 2 eta) / 64 + 32u`.
  - This covers a retarget in mid-ramp (a new ramp starts from the current word).
  - It covers rule 2 (step 0, then a jump to the identity with the integrators cleared,
    `builtins.rs:1119-1125`) and rule 3 (a jump with zero state).
  - The mono-collapse disengage (`builtins/src/lib.rs:2070`) copies the whole channel's integrators
    and its silence counter, and the words are mirrored. Channel 1 therefore takes channel 0's
    whole history, which is a legal history.
  - Within a plan, identity words carry zero state.
- **`Phi`, `Psi`, Abel summation.**
  - `q_k E_j [not direct] <= Psi_k E_j - Psi_k' E_(j+1) + Psi_k' (beta_k |x_j| + F)` follows from
    `E' <= rho_k E + beta_k |x| + F` and from `Psi_k >= q_k + rho_k Psi_i` for every neighbour `i`.
  - On a reset or an identity frame, `Psi E = 0`.
  - The weights `W_j = prod rho_L <= rho_ramp^(n-1-j)` do not decrease, so the telescoped sum is at
    most `W_(n-1) sup(Psi Phi) = V`. An LPF reset restarts the sum with the same bound.
  - `c = 1/2 (m1, m2)(I + A)` holds exactly for any words, and the dual norm gives
    `|c s| <= 1/2 |m| q_k E`.
  - The neighbour sets are symmetric, and `step` carries `SLACK`, which covers the `f64` comparisons.
- **Settled phase.**
  - `b = (I - B) e2` exactly, so `tau' = B (tau - e2 dy) + rounding`. The rounding
    `mu ||sigma|| <= mu (||tau|| + |y_prev|)` is absorbed into `rho = q + mu`.
  - `a_h`, `a_F`, `c_y` and `c_d = 1/2 |m| ||A^2 - I|| + gamma mu + omega (1 + r)`, with `F` weight
    `gamma + 2 + omega`, match term by term.
  - The `X` terms are correct. An LPF flush restarts `tau` at `-e2 y_prev`; only the last restart
    counts, because the recursion is linear with the same drives. The HPF flush makes one output
    jump. Each term is at most `max(rho, r)^(n - n0) c_y H_0 + c_y H_stall + F`.
- **Grouping.** The `covers` reduction is sound because the systems are non-negative and increase
  in every term. The covered zone's HPF rests no later, and its LPF bound is pointwise smaller at
  the covering zone's HPF rest frame.
- **Tail and stall.**
  - The only non-linear operation is a min of per-component bounds
    (`H_0 = min(window energy, Phi_k)`). Path-wise linear majorants (`e^G`, `e^F`) make each
    component's min valid, so `P rel + stall` is a valid bound.
  - The falling test and the bisection are sound for a non-negative `M`.
  - The fixed point `U` plus `M_h^m u0` bounds the driven system.
  - `P*` is about 1.2e-7 and `R(P*) < T_decay`, so `T_rest = T_decay`, and the two-level `P*` is not
    needed.
- **Window.** A control event at `N - 1` reaches its exact target at `N + 63` (the done branch), so
  the words are fixed from `N + 64`, as the window assumes.

## Independent recomputation (50-digit `Decimal`, `evidence/recompute_1433.py`, no inflation)

I wrote this script from the derivation, without the implementer's scripts. Its inputs are #1329's
envelope and rounding counts, read from the module as exact bit patterns (`evidence/dump.txt`).
The script itself recomputes the pole domain (its own 50-digit `tan`), `first_mix_row`,
`first_output_rounding`, the zones, `Phi` and `Psi` (least fixed points), the window, the settled
systems (exact first-falling frame and crossing), the stall, `P*` and the rests.

| rate | `T_decay` recomputed / module | `R(P*)` | `peak_plus_24_dbfs` | `any_sanitized_input` | `P*` recomputed / module |
|---|---|---|---|---|---|
| 44.1 kHz | 703,991 / 704,010 | 693,172 / 693,189 | 1,067,188 / 1,067,207 | 2,384,954 / 2,384,997 | 1.2024693e-7 / 1.2024694e-7 |
| 48 kHz | 699,934 / 699,952 | 689,416 / 689,433 | 1,061,479 / 1,061,497 | 2,371,966 / 2,372,008 | 1.1958752e-7 / 1.1958753e-7 |
| 88.2 kHz | 704,000 / 704,018 | 697,067 / 697,084 | 1,071,038 / 1,071,057 | 2,388,757 / 2,388,800 | 1.2037480e-7 / 1.2037489e-7 |
| 96 kHz | 699,942 / 699,960 | 694,052 / 694,069 | 1,065,670 / 1,065,688 | 2,376,105 / 2,376,147 | 1.2213093e-7 / 1.2213103e-7 |

- The module is 17-43 frames above at every rate, as its `1 + 2^-30` inflations predict.
- 146 zones; `step` 0.031235 (44.1 kHz).
- `first_mix_row` matches to `2^-30`, and `omega` matches exactly.
- Without grouping (per-zone systems), `T_decay` is about 1,040 lower and the rests are unchanged.
  This confirms the record's "about 1,100 frames".

## Adversarial real-kernel histories (release, `evidence/adv1433.rs`, `adversarial_*.log`)

- **Setup.** Top pair, +24 dB trim, alternating +24 dBFS input. Baseline (no retarget): block rest
  983,424, LPF V-norm at `N` 6.81e6.
- **Single release.** HPF retargeted from the top to 15 targets (`top (1 - 1e-6 .. 0.9)`, 1 kHz,
  100 Hz, 10 Hz, disabled) at 21 leads (0-30,000 frames before `N`), with and without input after
  the retarget (1,596 runs at 44.1 and 96 kHz). The LPF state at `N` reaches 2.31 times the
  baseline (HPF to 21,388 Hz, 65 frames before `N`). The worst rest is 970,816 (44.1 kHz) and
  965,632 (96 kHz).
- **Pumping.** Release and rebuild every 1,000-30,000 frames for 300,000 frames, then a release and
  0-60,000 frames back at the top (192 runs, 44.1 kHz). The worst rest is 986,624.
- **Result.** Every rest lies below the certified 1,067,207 / 1,065,688; the certified value is at
  least 1.0817 times the worst rest found. This is evidence, not a proof. The proof is above.

## Cost and memory

- **Native.** Release, one pinned core, best of 50 (`evidence/cost.log`): 236.5 / 241.6 / 245.8 /
  249.5 us per `input_section_live_bound` at 44.1 / 48 / 88.2 / 96 kHz. This confirms the record's
  240-254 us. It is computed once per preparation (`builtins-compiler` `live_input_bound`).
- **Browser.** I did not re-measure the boot times (2.22 to 2.47 ms for 9 tracks, 20.2 to 20.6 ms
  for 64 tracks). They agree with the native cost.
- **Module size.** Base 3,056,452 B (`--module-only` from the base export) and head 3,073,274 B:
  +16,822 B, as recorded.
- **Memory pins.** No pin moved: the diff touches no fixture, and
  `hosts/host-web/tests/browser-v1/expected.json` is unchanged. `check-browser-expected-resources.py
  --artifacts` is green against the built module ("digests and exact rows agree"), and so is the
  boot high-water gate. The bound's peak allocation (one `Powers4` table of at most 26 `4x4` entries
  per group, dropped per group) is what the `memoryBytes` pin now observes, as the record says.
- **`builtins/src/tail.rs`.** Returning the whole `LiveCascade` through the existing export (no new
  exports) is acceptable within the paths (n2).

## Mutations (on the exported tree, release, the named test; tree restored and confirmed byte-equal)

| mutation | test | result |
|---|---|---|
| M1 zone contraction at the inner end only | live scan | RED (11,077 Hz HPF above its zone) |
| M2 pole step halved | live scan | RED (0.0312334 > 0.0156187) |
| M3 `\|1 + p\|` at the zone's low end | live scan | RED (10.28 Hz HPF) |
| M4 `omega_state` at the largest words only | live scan | RED (2,848.5 Hz HPF) |
| M5 potential charge dropped | oracle | RED (`T_decay` 703,120 < 703,991) |
| M6 second `N_SILENCE` dropped | oracle | RED (`R(P*)` 689,425 < 693,172) |
| M7 settled drive by the output, not its change | oracle | RED (`P*` 4.6e-5) |
| M8 rest stops after the HPF | gate 2 | RED (not at rest at `N + R = 972,956`) |
| M10 settled decay at `rho_ramp` | gate 3 | RED (`<= 1_075_000`) |
| M11 `T_rest` from the +24 dBFS rest | live `T_rest` identity | RED |
| V1 direct-zone charge dropped (mine) | oracle | RED (`T_decay` 703,380) |
| V2 `X_0` dropped (mine) | oracle; whole `tail_contract` | **GREEN** (m2) |
| V3 neighbours ignore the step (mine) | oracle; whole `tail_contract` | **GREEN** (m2) |
| V4 stall fixed point dropped (mine) | oracle | RED (`P*` 1.8e-20) |
| V5 input column at the zone's high end (mine) | live scan | RED (10.17 Hz HPF) |
| V6 window output rounding dropped (mine) | oracle | RED (`T_decay` 703,989 < 703,991) |
| V7 `Phi` without neighbours (mine) | oracle | RED (bound refused by the final check) |

M9 (the universal ball) was not re-run. Its effect is a ratio of 1.21, so gate 3's ceiling (M10) and
the oracle's upper band also catch it. All five named tests are green on the restored tree.

## Test value (new or rewritten tests)

- `live_bound_covers_every_scanned_design_and_ramp_word` (extended): a zone constant evaluated at
  the wrong end of its range (M1, M3, V5), a stated pole step below the kernel's real per-frame move
  (M2), or an HPF output rounding that is not a supremum (M4) turns it red. The oracle shares the
  derivation's formulas, and the figures barely move, so no other test sees these.
- `live_bound_carries_every_term_an_independent_recomputation_requires` (rewritten oracle): a
  dropped or reweighted term that lowers a figure below the independent recomputation (M5, M6, M7,
  V1, V4, V6) turns it red. The real kernel, with 8.5 % slack, cannot. It cannot see terms worth
  less than the inflation band (m2).
- `live_bound_holds_on_the_real_kernel_at_the_domain_extreme` (extended): #1433's assertion adds no
  unique catch (n3). The test keeps #1329's unique value: a live bound below the real kernel's rest
  under the adversarial and block-size-1 histories.
- `live_rest_bounds_are_within_the_restated_contract_figures` (ceilings restated): a loosening that
  moves the module and the oracle together turns it red (the oracle takes #1329's envelope from the
  module). Examples: the envelope's `rho_settled`, or figures drifting above 1,075,000 / 2,390,000
  or above #1329's `T_decay` (M10).
- `live_tail_every_peak_is_the_rest_at_the_flush_floor` (rewired): a `T_rest` taken from any rest
  other than `R(P*)` recomputed through `live_cascade` at `P*` turns it red (M11). It is exact,
  where the oracle allows a band of `0.01 % + 64` frames.

## Gates run (all green)

- Release `tail_contract` with `--include-ignored` (9/9; the figures and ratios 1.0853 / 1.0850 /
  1.0892 / 1.0893 as recorded).
- The spec's debug command (`-p lane -p math -p builtins -p dsp-reference` with its features).
- `cargo fmt --all -- --check`.
- Workspace clippy `--all-targets -D warnings`, with and without `--all-features`.
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- `test-debug-a` (122 suites, 0 failed) and `test-debug-b` (162 suites, 0 failed), as in
  `qualification.yml`. Both ran with `CARGO_PROFILE_DEV_DEBUG=0` to stay inside the disk limit; no
  test behaviour changes.
- `conformance_fixtures --check`.
- Policy scripts: `check-workspace-policy.sh`, `check-realtime-policy.sh`, `check-lane-policy.sh`,
  `check-builtins-policy.sh`, `check-dsp-research.sh`.
- `graph_fixture --check` and `check-graph-determinism.sh` (100/100).
- Release build of `audit`, `bench`, `capi` and `session-validator`. `audit capi` (0 violations,
  0 allocations).
- `check-capi-abi.sh` and its `--self-test`.
- `check-builtins-fixtures.sh . target/release/audit`.
- `check-cross-targets.sh` (PASS, with the #1018 expected failures).
- `run-wasm-gates.sh --without-v8-spill --without-native`.
- The worklet chain:
  - `build-web-audioworklet.sh --named-twin`.
  - `strip-wasm-names.py --self-test` and `check`.
  - `check-web-audioworklet.sh --without-metadata-regeneration` (including the boot-budget
    high-water gate).
  - `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm` and `--native`.
  - `test-web-audioworklet.sh`.

## Items for ROOT

1. **m1.** Decide whether #1433 attempt 2 (or a follow-up) applies the corrected supremum to the
   envelope's `output_state` or records the absorption argument. The shipped #1329 value is sound;
   its stated proof step is not.
2. **m2.** The remedy (intermediate-quantity oracle checks, or a constructed `math::tail` unit
   test) matters most before #1372 and #1373 reuse `live_cascade`.
3. **n2.** Renaming `input_section_live_envelope` (for example to `input_section_live_terms`) needs
   `crates/builtins/src/lib.rs` in a slice's paths.
4. **n4.** #1329 D5's `h`-box premise should mention the `f64` design-evaluation error. This is
   wording; no figure is at risk.
5. **#1329 R8 / #1269, inherited.** Rule 4 from an identity section that holds carried non-zero
   integrators ramps words with `Re p` up to 1. Those words lie outside #1433's pole domain, with a
   contraction up to 1. The live bound, like #1329's, covers histories within one plan. #1269 must
   still state whether a carried state is admitted history.
6. **Cost for #1457.** About +200 us native per preparation with a live input lane. The browser
   live boot is +0.25-0.4 ms (implementer's figures, consistent with my native measurement).
7. **Briefs.** Gate 2's 15 % margin is dominated by gate 3's restated ceiling at every rate (n3).
   Future briefs should not require two assertions where one is strictly stronger.
