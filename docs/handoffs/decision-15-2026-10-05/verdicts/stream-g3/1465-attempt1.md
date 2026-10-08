PASS

# #1465 attempt 1: adversarial verdict

Commits reviewed: `ee1ef4222` (stop note), `f3e6b1539` (implementation), `32f623059` (fold-in under
root's rulings), as `git diff bc2299a08 32f623059` (13 files). Built and tested from an export of
`32f623059` (`/tmp/claude-1002/v1465/tree`, `CARGO_TARGET_DIR=/tmp/claude-1002/v1465/target`). The
base `bc2299a08` and a scratch copy of the attempt (verifier-only probes, never shipped) were
exported beside it. The worktree was not touched.

Verdict: the closed form for `D` is sound, and the code implements it. I checked the proof step by
step (below). The main chain holds: the certificate, its verification under rounding, the
logarithms, the crossings and the claim `T + k D >= T(k)` for every `k >= 1`. Two steps of the
written proof are wrong or do not match the code: the underflow paragraph, and the anchor
condition when `t0 = 0`. Neither moves a stated value for any builtin design. A third statement,
`P^ < 2 P*`, is false on 16 of 112 gate rows. I found no counterexample: on 224 rows (gate-2
representatives, gate 1(a) rows, trims down to -144 dB, 96 pseudo-random designs, four rates), the
independent brute force of the exact half and the module's own walked crossings stay at or below
`T + k D` for `k = 0..16`. #1329's values and the frames walked are bit-identical to the base on
1,560 designs I chose. Every gate is green. Every mutant I re-ran goes red. One mutant in the
record (M2) was not the mutant it names. There is no BLOCKER and no MAJOR. The four MINORs are text
and evidence fixes plus one line of code, for a follow-ups commit.

## BLOCKER

None.

## MAJOR

None.

## MINOR

1. **The underflow step of the proof does not prove what it says.**
   `docs/derivations/1379-graph-tail-composition.md:178-185` says that each rounding loss
   (`<= 2^-1075`) reaches a carried component through at most one resolvent, so the error is
   below `2^12 2^220 2^-1075 < 2^-840`. This is not correct. A loss in an entry of an
   intermediate power `G^(2^s)` reaches the result as `G^a E G^b z`. That has two power factors,
   each up to the resolvent, and it is proportional to the state `z`. With the note's own
   constants (`R < 2^220`, `n <= 4`, `< 2^12` sites), the bound is about `2^-617 max_p z_p`.
   `tau = 2^-600` covers that only while `max z < 2^17`, but the per-unit states at the top pair
   are about `2^15-2^16`. The conclusion is still true by a large margin, and the fix is in the
   text only. For every certificate that is verified, `G_pp SLACK^3 < lambda < 1`, so
   `1 / (1 - G_pp) < 2^29`. This gives resolvent entries below about `2^122` (`L`) and `2^60`
   (`M`), and so a loss below about `2^-800 max z`. Restate the paragraph with both factors and
   `z`, and bound the resolvent by the verified gap, not by `2^53`. (The `2^53` bound also fails
   for `L`, whose diagonal `rho SLACK` can be at or above 1. No certificate can exist then, so the
   corrected argument only needs verified designs.)
2. **The anchor condition in the code is not the one in the proof.** The derivation
   (`:117-119`) anchors the reference half at the pass end `H` "if `t0 = 0`".
   `crates/math/src/tail.rs:2646` tests `first_block == 0` instead. A forward block sum
   (`suffix[0]`) and the replay's backward running sum can fall on the two sides of the threshold
   (a rounding tie). Then `first_block = 1` and `t0 = 0`, and the anchor is the record of frame 0.
   That record is the state before the impulse (all zero), so `remainder_sums` returns 0 (the
   remainder needs `now >= 1`), and the certificate states the reference as at most
   `tau lambda^i`. That is unsound. For builtin designs this cannot occur. The threshold is at
   most about 0.5 (the -144 dB trim floor), and `O >= 1.078` on every row I measured (one section:
   `O >= ||h_1||_1 >= 1`), so `first_block >= 1` and `t0 >= 256` always. (So the `H` branch is
   also unreachable today.) Fix: branch on `t0 == 0`, or anchor at `max(t0, 1)`.
3. **`P* <= P^ < 2 P*` is false as written** (derivation `:26`, spec `:50`). The millibel rounding
   of `sigma` can add up to a factor of `10^(1/2000)`. When `g dev_core` is very small,
   `P* -> 2 F a / eps`, and `P^` goes slightly above `2 P*`. The F3 output of the release
   `tail_contract` run shows `P^ >= 2 P*` on 16 of 112 rows, with a maximum of `2.0016`
   (88.2 kHz, HPF 1 kHz, 0 dB: `P^` 3.4085e-11, `P*` 1.7029e-11). Soundness only needs
   `P^ >= P*`, which holds on every row. Restate it as `P* <= P^ < 2 * 10^(1/2000) P*`.
4. **M2 in the mutation table and the F1(b) test-value text describe a mutant that was not
   run** (spec `:253-257`, `:412`). The mutant as named ("rates only at `lambda = (1 + rho) / 2`",
   `D_inf` about `2 D_floor`) goes red at the first row, which is the 10 Hz LPF at 0 dB and
   44.1 kHz: `D` 5,422 > 1.5 x 2,301.875. The numbers in the record (red at the 10 Hz HPF +24 dB,
   `D` 3,758 > 1.5 x 2,299.75, after five rows pass) come only from a weaker variant. That variant
   replaces the coarse grid with `j = 2` but keeps the refinement, so it searches
   `j in {1, 2, 3, 5}`. I ran both, and the variant gives exactly 3,758. The claim ("a needlessly
   loose `D` exceeds the 1.5 line") is true, but root's ruling 3 asked for this text to match what
   was measured. Correct the mutant's description, or state the halfway mutant's own result.

## NIT

- F3's test-value text (spec `:278`) says that a stall of zero "gives `P^` below the module's
  `P*`" (M13). As measured, M13 (`FlushStall::Zero`) goes red in `composition_values`'s `Level`
  match (`tail_contract.rs:2092`), before any `P^` comparison. The `P^ >= P*` line is what
  catches a raw stall that is too small. My mutant M13m (the raw stall times `1e-3`) goes red
  there: `P^` 1.65e-12 < `P*` 8.26e-10. State the two mechanisms as they are.
- M12's row in the table (spec `:426`) says "top pair, 773". The first row that fails is the LPF
  one `f32` below the maximum at 0 dB and 44.1 kHz, with 773 against 792. The spec's F2(d)
  figure ("about 59 mB low at the top pair") is correct: 55.52 against 51.86 is 59.2 mB.
- F1(a)'s illustrative figures (spec `:249-252`, root's original text) do not reproduce. At the
  44.1 kHz top pair, the exact suffix's shortest decade step for `k <= 8` is 36,270 at 0 dB
  (33,502 at +24 dB), the mean is 45,467 (45,382), and a `D` equal to the shortest step falls
  47,424 (67,574) short by `k = 8`, not 35,731 and about 52,000. The claim itself holds: M1 and M3
  are red at (a). F1(c)'s steps (48,531, 48,136, 47,816) reproduce within one frame (48,530,
  48,135, 47,815 at the top pair, 0 dB).
- F2(c)'s wording "states 0 dB for a non-power-of-two trim" (spec `:272`) does not match the
  measured mutant (M8 states 600 instead of 601 mB at +6 dB).
- Derivation `:202`: "`T(k)`, `D` and every `j_k` are integers below `HORIZON_LIMIT`". Only `D`
  (and so `j_1`) is bounded. `T(k)` grows without limit in `k`.
- Derivation `:155-157`: "`max_k ceil((T(k) - T) / k) = D` is reached at `k = 1`" holds only when
  the half that binds `D` has `a >= 0`.
- `docs/EFFECT_CONTRACT_V1.md:106-108`: a fixed design that goes over the budget (it reports the
  live bound) or that has no verified certificate still states `Unstated`. The sentence reads as
  if every fixed design states its values.

## The proof, step by step (`docs/derivations/1379-graph-tail-composition.md`, "`D`: the closed form")

- **The systems.** Reference: `S(t + j) <= alpha . M^j w(t)`, `w = (I - M)^-1 m(t)`, `t >= 1`.
  `majorant_step` builds `M` (lower triangular, `M_ii = q_i`, `M_ij = beta_i alpha_ij SLACK`) and
  `alpha = alpha_{K+1}` with every product inflated, so its entries are at or above the products
  of the computed constants. `remainder_sums` is #1329's code, moved and unchanged. Deviation:
  `dev(m + j) <= row . L^j z(m)`, where `L` and `row` are `linear_map`'s and `z` is the floored
  walk state. The orders are triangular (`M`: section order; `L`: `x_1, E_1, x_2, E_2`, checked
  against `Deviation::step`), but soundness does not need this, because the check uses the full
  row.
- **The anchors and the carry.** The replay records the state before each step and after the
  last one. `checkpoints[b]` is frame `b * 256`, so `record[(t0 - block * 256) * width]` is
  frame `t0` (correct). The deviation anchor is `deviation.state()` after `frame` steps, which is
  frame `frame`. `at(later)` uses the same convention. `T = max(t0, tail_deviation) >= t0`, so the
  reference offset is 0 whenever `t0 >= 1`. On all 224 rows both offsets are 0. The carry is
  `power_apply` plus `tau`. It is an upper bound when no product underflows, and with underflow
  it is an upper bound after the correction in MINOR 1.
- **The certificate and its verification.** `v_p = max(z_p, ...)`, so `v >= z` and
  `v_p >= tau`. The check `fl(dot(G_p, v) SLACK) <= fl(lambda v_p)` gives
  `dot <= lambda v_p (1 + 2^-53) / (SLACK (1 - 2^-53)) <= lambda v_p (1 - 2^-31)`. Also `dot` is
  at least the exact `(G v)_p` up to `2K 2^-1075` of underflow, against a margin of at least
  `2^-31 2^-16 2^-600`. Correct. NaN or infinite states fail the check or the `finite` test, so
  the composition is unstated.
- **The logarithms.** `rate = -log(lambda) / SLACK <= -ln lambda`. `b = LN_10 SLACK / rate SLACK`
  is an upper bound. For `a < 0`, the two `SLACK` factors cancel, and the bound rests on
  `LOG_MARGIN = 2^-30` against at most about `6 * 2^-53 |N|` with `|N| <= ~1500`. That holds
  (the note's "inflated or deflated by its sign" is not the reason, but the result is correct).
- **The crossings and `D`.** `D_h = o + floor(ru(max(a, 0) + b)) + 1`, so `k (D_h - o)` is an
  integer above `k (max(a, 0) + b) >= a + k b`, and so at least `j_k - o` for every `k >= 1`. For
  real-valued upper bounds `a`, `b`, this gives `T + k D >= T(k)`. Correct.
- **`tau` floors.** The certificate does not floor. Its inputs are floored upper bounds. Correct.
- **Gains.** `G_p = G_t = ceil_mB(g (O + dev_loud))`, `g = |trim| (1 + u)` (the module's `gain`
  already carries `1 + U`), `O = input_sup[K]`, `dev_loud = at_end.loud_output`. (N1) follows
  from `|y_ref| <= g X O` plus the relative deviation at its loud fixed point plus `F a`. The
  split at `M` for (N2) is linear and non-negative, and `G_t = G_p` is the fixed design's
  operator. `ceil_millibels` adds `|x| 2^-30 (+ 2^-30)`, which covers the evaluation. Correct.
- **Disabled filters.** `fl(x trim)` is exact for a normal power-of-two trim up to underflow
  (`2^-150 <= sigma = 2^-126`). Otherwise it is within `(1 + u)`. The trim domain (-144..+24 dB,
  `db_gain` refuses subnormal values) excludes 0 and subnormal trims, so `ceil_mB` never sees 0.
  `D = 0`, rest `ZERO`, `sigma = Level(-75,859)`. Correct.
- **`sigma`, `P^ >= P*`.** `P^ = 4 sigma / eps >= 4 F a / eps >= P*`, because
  `eps / 2 - g dev_core >= eps / 4`. Correct. The upper relation is MINOR 3.

## Independent evidence

- **The walk to `T` is unchanged.** A harness added to both exports printed `tail`,
  `tail_every_peak`, both rests, `rest_at_flush_floor`, `tail_reference`, the bits of
  `flush_floor` and the frames walked of `fixed_cascade_within` for 1,560 designs: 11 cutoffs per
  section (10 Hz to the maximum, with 1 and 7 `f32` below it), trims -144, -61.3, -6, 0, 3.3 and
  +24 dB, four rates. Base and attempt are byte-identical.
- **Counterexample search.** For 224 rows (see above) I checked three things for `k = 0..16`:
  the brute-force exact-half crossing `T_b(eps 10^-k / (2 |trim| (1 + u)))` in plain `f64`;
  the module's own majorant pass and deviation walked frame by frame to the `k = 16` floor (a
  scratch function, never shipped); and each walked half's crossing against the certificate's
  half crossing. There was no violation on any row.
- **Looseness.** I reproduced the record's figures on gate 2's families: the worst is 1.231 (HPF
  1 kHz, +24 dB, 44.1 kHz; `D` 32 against walked 26). The top pair is 1.028, the 10 Hz HPF at
  +24 dB 1.217, and the cheap two-section designs up to 1.222. The spec's values also reproduce:
  top pair +24 dB `T` 478,335, `D` 49,403, `O` 3.2719, `dev_loud` 0.2310; typical `T` 9,272,
  `D` 1,271, `O` 6.3842; 1 kHz LPF `T` 175, `D` 27. **For root (outside gate 2's families):** at
  trim -144 dB, the top pair's `D` is 70,794 against a walked 53,432 (1.325). `T` is early there,
  and the anchored certificate carries a transient `a` of 11,906 frames. Pseudo-random designs
  with 3- or 4-frame decades give 1.333. All rows are below root's 2x line. F1(b)'s 1.5 line runs
  only at 0 and +24 dB. At -144 dB the top pair's `D` is above 1.5 times the +24 dB row's exact
  mean decade (70,794 against 1.5 x 45,382). I did not measure `D_mean` at that trim.
- **Named exceptions are confined.** `crates/builtins/src/lib.rs` changes only the doc lines of
  `input_section_bounds` and the both-disabled branch of `input_section_bounds_within`.
  `crates/builtins-compiler/src/lib.rs` changes only the disabled strip's assertion in
  `live_input_lane_reports_the_live_bound_and_plain_input_its_own`. Its expected value is now
  `input_section_bound`'s, and one `tail == Finite(0)` assert was added. The example and the
  three specs of #1468, #1470 and #1471 change only the authorized figures.
- **The memoryless branch uses both channels.** `memoryless_input_bound` is the maximum of the
  two channels, and both memoryless paths call it (M16 and M17 are red).
- **Render-owned memory does not grow.** `effect-contract` has no change, so `NodeTailBound` has
  the same size. Only the transient `math::tail::CascadeBound` grows, and nothing stores it
  (`bound_from_cascade` converts it at once). `InputBoundCache` holds `ChargedInputBound`, whose
  size does not change. There is no change to a render path.
- **Gate 8 and the calibration code.** Gate 8 passes at every rate (it asserts no strip on the
  live bound and the charge within budget). `frame_equivalent` searches the half-nanosecond grid
  upward with `C(F) = ceil_10(P / F)` and returns the first `F` that bounds every sample's work
  by `(frames + C(F) sections) F`. That is root's statistic as written. It terminates, because
  every recorded design has an enabled section (no `0 / 0`). `one_design` reads the sections from
  the test-support charge counter (and asserts divisibility). Gate 2's `need` uses the charge when
  every design is exact, and otherwise the budget. The budget is spent to within one channel's
  section charges, so this is correct. Excluded per the brief: I did not judge the 48.5 ns and
  210 values.

## Mutation runs (release `tail_contract` unless stated; each applied to the scratch copy, the named test run, the file restored)

| mutant | result |
|---|---|
| M2 as named (`j = 2` only) | red at (b), first row: 10 Hz LPF 0 dB, `D` 5,422 > 1.5 x 2,301.875 |
| M2 variant (coarse grid to `j = 2`, refinement kept) | red at (b): 10 Hz HPF +24 dB, `D` 3,758 > 1.5 x 2,299.75 (the record's numbers) |
| M3 `D` without the transient | red at (a): 10 Hz LPF, `T_b` 19,895 > 17,408 + 2,286 (as recorded) |
| M3b M3, (a) disabled | red at (c): `T(1)` 22,066 (as recorded) |
| M4d stated rate from `rho - 0.05 (1 - rho)` | red at (d): 2,176.97 < 2,286 (as recorded) |
| M4q (mine) deviation's stated decade from `max q` after verification | red at (d) only: LPF one `f32` below max, 44,004.6 < 44,869 (about 2 %, as F1(d)'s text says) |
| M5 trim dropped from `G_p` | red at (a): 17.28 > 1.09 |
| M6 `O` from the first section | red at (a): 10 Hz into 1 kHz, 2.443 > 2.435 |
| M7 `G_p` from state norms | red at (d)'s equality |
| M7b M7, (d)'s equality disabled | red at (b): 10 Hz into 1 kHz, 1,516 mB above 14.497 dB |
| M8 `(1 + u)` dropped (disabled) | red at +6 dB |
| M11 `dev_core` as `dev_loud` | red at (d): 1.29e-10 against 2.88e-4 |
| M12 `G_p` on `dev_core` | red at (d)'s equality: LPF one `f32` below max, 0 dB, 773 against 792 |
| M13 `FlushStall::Zero` | red in `composition_values` (not at `P^ < P*`) |
| M13m (mine) raw stall x `1e-3` | red at `P^ >= P*` |
| M14 `sigma` one mB low | red at the `ceil_mB` equality |
| M15 both-disabled branch states `ZERO` | red: `input_section_bounds` `Unstated` |
| M16 `fixed_input_walk` memoryless, left only | red: `(0, 0)` against `(1201, 1201)` |
| M17 `memoryless_input_bound`, left only | red: the louder channel's gain |
| BC M15, builtins-compiler `live_input_lane_reports_the_live_bound_and_plain_input_its_own` | red: `Unstated` against `Stated { 0, 0, 0, Level(-75859) }` |

Before this slice no test read a composition value. The only exception is the rewritten
builtins-compiler assertion (it pinned `ZERO`, and it is superseded in place). So no existing
test caught these defects.

## Test value (one sentence per new or rewritten test)

- `fixed_design_decay_law_is_sound_tight_and_above_its_floor`: a `D` that drops the certificate's
  transient (M3, red at (a) against the brute force, and at (c) alone), a stated rate faster than
  the propagation's spectral radius (M4d, M4q, red at (d)), or a needlessly slow rate (M2, red at
  (b)) turns it red, and no other test reads `D`.
- `fixed_design_gains_bound_the_cascade_and_stay_near_its_section_norms`: a peak gain that drops
  the trim or the second section (M5, M6, red at (a)), uses `dev_core` for `dev_loud` (M11, M12,
  red at (d)), or comes from the sections' state norms (M7b, red at (b)) turns it red.
- `fixed_design_stall_is_the_module_stall_rounded_up_and_covers_the_flush_floor`: a `sigma`
  rounded down (M14) or a raw stall too small to cover the flush floor (M13m) turns it red.
- `a_design_with_filters_disabled_states_its_trim_gain`: the preparation branch that states
  `ZERO` (M15), a memoryless path that reads only the left channel (M16, M17), or a
  non-power-of-two trim without `(1 + u)` (M8) turns it red, on both entry points.
- builtins-compiler `live_input_lane_reports_the_live_bound_and_plain_input_its_own` (rewritten
  assertion): a compiled session that reports `ZERO` for a disabled strip (BC) turns it red.

## Gates run (export of `32f623059`)

- `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`:
  17 passed (F1 ratios 1.046-1.391, F2 least margin +0.743 dB, F3 on every row).
- `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features
  math/lane,builtins/test-support,lane/test-support`: 45 test binaries, all pass (the `math` lib
  tests are included).
- `cargo test --locked -p builtins-compiler --features test-support --lib`: 57 passed, gate 8
  (`every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`) included.
- `cargo fmt --all -- --check`: clean. `cargo clippy --locked --workspace --all-targets
  --all-features -- -D warnings`, and without `--all-features`: clean.
- `scripts/check-workspace-policy.sh`: ok. `scripts/check-realtime-policy.sh`: ok (89 regions).
- `scripts/check-cross-targets.sh`: PASS.
- `cargo build --locked --release -p audit && audit capi`: 0 allocations, 0 locks, 0 syscalls,
  `pcm_digest` `cb10fbface44a3a4` (unchanged).
- `scripts/check-builtins-fixtures.sh . <target>/release/audit`: ok (50 files).
- Not run: gate 2 (descriptive timing, and its statistic is excluded), `run-wasm-gates.sh` (not in
  the brief; no render or browser-compiled code changed).

Evidence files are kept in `/tmp/claude-1002/v1465/` (`tail_contract_release.log`, `exp.log`,
`mut/*.log`). The targets and the trees are deleted.

## Addendum: fold-in 2 (`55a92ec11`, `git diff 0133fd8f6 55a92ec11`), root's median ruling

The coordinator extended this review after root's second frame-equivalent ruling, and lifted the
exclusion on the statistic. I reviewed `git diff 0133fd8f6 55a92ec11` (7 files). I ignored the
parent `0133fd8f6`, a #1464 follow-up, as instructed. I built an export of `55a92ec11`
(`/tmp/claude-1002/v1465/tree2`, own target). The verdict stays **PASS**. Fold-in 2 adds no
BLOCKER, no MAJOR and no MINOR.

**The calibrate code implements the ruling.**
- `REPEATS = 5` measured rounds after one warmup (`ROUNDS = 6`) in one invocation. Every
  calibration grid design, every frame-class point and every gate-2 family at a rate has five
  samples. Its cost is the `median` (the middle one of five).
- `fixed_costs` fits each class's line on the designs' medians. The section charge input `P` is
  the largest median-fit intercept per section over the classes and rates (9.93 us). The
  per-round lines are printed only. This covers every class's whole intercept, because
  `C n >= intercept` for `n = 1, 2, 2, 4`. The `D + nS` model's slope (10.43 us at 88.2 kHz) is
  higher only because the per-design term is negative, so the budget does not need it.
- `frame_equivalent` searches the 0.5 ns grid upward with `C(F) = ceil_10(P / F)` and returns the
  first `F` at which every design's median is at most `(frames + C(F) sections) F`. That is the
  ruling. It terminates, because every recorded design has an enabled section. A misfit line
  cannot make it unsound: the search bounds every design's median one by one.
- Gate 2: each family at a rate keeps five design-work samples and their median. `consumed` is
  asserted the same in every round. The need is the largest median per consumed
  frame-equivalent. When a design is left on the live bound, the budget is used as `consumed`.
  This understates the need by at most one channel's section charges (940 of 1,510,000, 0.06 %),
  against a headroom of 16 %.
- The raw maximum and its ratio to its design's median are printed in both modes. Units are
  consistent (ms to ns).
- The calibration binary was built with charge 210. This is valid: the frames and the sections
  counted (`fixed / charge`) do not depend on the charge.

**The record matches the raw outputs** (`g3-cal2.txt`, `g3-g2b.txt` and their `.load` and `.time`
files):
- Calibration. 21,504 designs. 9.93 us, then 461.8, then a charge of 470 (10.11 us). Binding
  design: 96 kHz, one cascade of two sections, design 1864, median 41,890 ns,
  1,025 + 470 x 2 = 1,965, 21.318 ns. Raw maximum 386,907 ns, 13.261 x the median of 29,176 ns,
  174.126 ns per charged frame-equivalent (2,222). Per-design terms -1.01 to -0.09 us. Median-fit
  per-section costs 8.54-9.93 us. Per-round largest 8.65-14.17 us (44.1 kHz round 1). Slowest
  frame class 19.40 ns (18.93-20.32), classes 13.30-19.40 ns. Load before 1.94, 48.7 s. All
  match.
- Gate 2. The budget line is 21.5 ns, charge 470, 5 rounds. Need 19.0 ns (18.523, cheap
  1 k into 1.28 kHz, +24 dB, 88.2 kHz). Worst median 27.970 ms, 86.2 %. Every per-family median
  range in the record matches the summary table. Ratios are 1.000-1.055 except for the two
  outliers (46.772 ms, 1.893 x 24.710, 144.1 %, 30.975 ns; 45.781 ms, 2.174 x 21.058). Load
  before 1.95, 16.1 s. All match.
- Gate 8 (I re-ran it on `55a92ec11`): the charged amounts, the frames walked and the margins
  are exactly the record's. The stereo 96 kHz document is 1,374,336 (1,254,016, margin
  **135,664**). The mono documents are 376,640, 401,984, 661,312 and 710,976. Every document is
  exact at every rate.
- NIT: the record gives the other 65,537-strip preparations as "60.36-61.16 ms otherwise". The
  other 19 measured rounds are 59.06-61.16 ms. 60.36 ms is the fifth-slowest that the
  `slowest preparation` list prints, not the least.

**The restated figures are consistent.** `INPUT_BOUND_SECTION_CHARGE = 470` and its doc,
`FRAME_EQUIVALENT_NS = 21.5`, the budget doc (32.465 ms, "bounds the median real work"), and the
cache cap: 257 + 470 = 727, and 1,510,000 / 727 = 2,077 designs, which the 8,192 cap holds. In the
specs: #1468 (470 of 21.5 ns), #1470 (21.5 ns, 32.47 ms, 27.97 ms worst median, 86.2 %, the
cheap families 18.99-27.97, typical 11.24-23.79, near-top 21.02-21.37, band 20.27-20.52, single
top designs 3.74-7.36, charge 470), #1471 (21.5 ns, 32.47 ms), and #1474's note (21.5, 470,
32.465, 727, 2,077). All agree with the raw outputs and with each other. The #1465 spec marks the
first fold-in's 48.5 ns, 210 and 73.235 ms as superseded and states the ruling as given.

**The statistic, judged.** It is the ruling as written, and it is a sound choice for this
quantity. The walk is deterministic, and interference only adds time, so the median of five
estimates the cost of the work. The frame budget enforces the work itself in frames (gate 2
asserts frames <= budget on every preparation). The ms figure is that work's median cost, and
the doc now says "median". 21.5 ns has 0.9 % headroom over the calibration's binding design and
14 % over gate 2's need. The charge, 470 x 21.5 ns = 10.11 us, covers the largest class fixed
cost of 9.93 us. The coupled change (the charge rose from 210 to 470) lowers the number of cheap
designs a preparation computes exactly. That is why the worst median (27.97 ms) stays at 86 % of
a budget less than half of the first fold-in's.

**The 46.77 ms raw sample: in my view, not a stop.** It is interference, not the cost of the
design:
- In the same round, the identical first preparation (a fresh cache, the same work) took
  33.1 ms against the no-cache preparation's 49.6 ms.
- The warm rebuild of that round took 2.85 ms against 1.65-1.68 ms in the other rounds, so the
  whole round was disturbed.
- The four other samples are 24.31-24.90 ms.
- The 45.78 ms sample (65,537 near-top, 48 kHz) shows the same pattern: first 59.5 ms against
  no-cache 69.4 ms in the same round.
- My own gate-2 run on `55a92ec11` (one invocation, `taskset -c 7`, exit 0, at a higher load of
  3.4-4.2, so descriptive only): largest raw-to-median ratio 1.051, largest raw design work
  28.89 ms (89.0 %), need 19.0 ns (18.887), worst median 28.52 ms. No sample was over the budget.

The ruling defines a family's cost as its median, and no family's median is over the budget. A
deterministic frame charge cannot bound preemption. Optional NIT: per round, `min(first,
no-cache) - rebuild` would halve the interference exposure of each sample. The median already
handles it.

**Gates on `55a92ec11`.**
- `cargo fmt --all -- --check`: clean.
- `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`: clean (the
  example included).
- `check-workspace-policy.sh`: ok.
- `cargo test --locked -p builtins-compiler --features test-support --lib`: 57 passed (gate 8
  included).
- Release `tail_contract`: 17 passed.
- `cargo test --locked --all-targets -p builtins --features builtins/test-support`: every binary
  passes.
- Gate 2: exit 0 (above).
