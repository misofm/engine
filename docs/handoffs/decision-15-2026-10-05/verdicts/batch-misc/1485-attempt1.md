PASS

# #1485 attempt 1: tighten the live input section's peak and tail gains

Commits under review: `ae8b09318` (phase A, parent `a059cdd03`) and `9d7722276` (phase B, parent
`fb5898857`; `fb5898857` is #1481 and out of scope). Worktree `/home/bl/misofm/wt-d15-misc`, not
touched. `9d7722276` exported with `git archive` into `/tmp/claude-1002/v1485/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1485/target`. Verifier: opus-xhigh, 2026-10-08. Small evidence
files: `/tmp/claude-1002/v1485/evidence/`.

No BLOCKER and no MAJOR. Two MINORs and four NITs. The bound is sound on every history I could
construct, and the strongest real-kernel peak I found is still 22.8 times below `g_p`. But that
peak is 1.65 times above gate 1's `g_meas`, so the record's `r_p` of about 37.6 (and #1487's
starting figure) overstates the remaining `G_p` looseness. Root must know this before #1487
starts (MINOR 1).

## (1) Soundness: the derivation checked step by step against `crates/math/src/tail.rs`

### `G_p` ("(N1): `G_p` and `sigma_p`", `LiveBound::{exposure, peak_gain}`)

- First section. `E_j <= Phi_(k_j)` is #1433's zone invariant: `Phi_k >= rho_i Phi_i + beta_i` for
  every neighbour `i` of `k`, itself included (`live_zones`, tail.rs:1223-1293). The neighbour
  relation is symmetric (distance at most `step`). Jumps (rule 3, a completed disable, a reset)
  carry zero state. So `a_j = (h q_(k_j) + omega) E_j + delta |x'_j| <= A` with
  `A = max_k (h q_k + omega) Phi_k + delta` holds at every frame (tail.rs:1747-1763).
- Second section. The state step at the zone's ramp constants (`contraction`, `input`) and the
  output row `(h_2 q_l + omega_2) ||sigma|| + delta |y_1|` hold for any word in zone `l`, because
  both sections' recursion words lie in the one `PoleDomain` and the constants depend only on
  `Re p` and the boxes. I checked the unrolled sum: `Gamma(m)` is
  `beta_(l_0) prod_(i=1..m-1) rho_(l_i) (h_2 q_(l_m) + omega_2)` with consecutive zones as
  neighbours. `B_r` and `Gamma` in `exposure` (tail.rs:1663-1703) match it index by index.
  An identity frame (state zero, output equals input, `delta >= 1`), rule 2 (recursion frozen in a
  zone, mix inside `second_mix_row`), rule 3 (zero state) and the mono-collapse copy (the copied
  channel's past is an admitted history) only drop terms.
- Exposure tail. The check `fl(fl(rho_l max c) STEP_UP) <= fl(fl(lambda c)/STEP_UP)` implies the
  exact `rho_l max_(nb) c <= lambda c(l)`. By induction `B_(H+s) <= lambda^s c`, so
  `Gamma(m) <= lambda^(m-H-1) Gamma(H+1)`. The tail `Gamma(H+1) lambda/(1-lambda)` bounds
  `sum_(m>H+1) Gammabar(m)`. `1 - lambda` is exact (Sterbenz).
- Split. With the computed envelope as `Gammabar` (non-increasing, at least `Gamma`), `Gamma <=
  Gamma_f + Gamma_s` holds for every `m0`. The running `fast` sum
  (`(m0-1)(Gammabar(m0-1) - Gammabar(m0))` per step, tail.rs:1773-1778) equals
  `sum_(m<m0)(Gammabar(m) - Gammabar(m0))`. `slow = (m0-1) Gammabar(m0) + sum_(m>=m0) Gammabar(m)`
  with the tail.
- Potential. With `Psi_k >= q_k [k not direct] + rho_k Psi_i` for every neighbour `i`
  (tail.rs:1254-1277), the inequality `[k_j not direct] q E_j <= P_j - P_(j+1) +
  Psi_(k_(j+1)) beta_(k_j) |x'_j|` holds at every frame, also at a direct zone (right side
  non-negative) and across a jump (`P_(j+1) = 0`). Abel summation with the non-decreasing weights
  `Gamma_s(n - j)` and `0 <= P <= V` gives `Gammabar(m0) V`. The per-frame charge `C`
  (tail.rs:1750-1761) uses the next zone's `Psi` over the neighbours, as the derivation does.
- Result. `g_p = g min_(m0)[delta A + A fast + h Gammabar(m0) V + C slow]`, every product rounded
  up. It does not read the window or (P2), so it holds at every frame of every admitted history.
  `sigma_p` is unchanged.

### `G_t` ("(R)" and "`G_t`", `LiveBound::tail_gain`)

- Words. I read `apply_prepared_filter` (crates/builtins/src/lib.rs:1447-1561) and the kernel's
  `filter_ramp_words` (crates/lane/src/kernels/builtins.rs). A frame runs on the current words, then
  the words update. So an event before frame `e <= N - 1` gives the design from frame `e + 64 <=
  M + 63`. The module's `r` in `0..=64` covers this, and more. #1407's ruling bounds each ramp word
  by `rho_j <= E` from its exact mixture of `c` and `t`. `Re p` is linear in the words, so the
  word lies within `eta = (box_0 + box_2) SLACK` of the interpolation. `over(from, to)` (contiguous
  zones that meet the interval, clamped at the domain ends) takes each constant's largest value.
- Paths. I checked the index arithmetic of `section` (tail.rs:1863-1929). Path `r` starts from zero
  at absolute frame `R - r`. Window frame `f = absolute - R + r`. The ramp word index is
  `i = R - r + f`. The design constants apply from `f = r`. `end` reads `S` at window frame `R`.
  Disable paths hold zone `s` for `r` frames, then the identity. The second section is driven by
  `A_f`, the largest first-section output over every path.
- Settled. I re-derived the bound on `sum_(i<s) r_l^(s-1-i) r_k^i` by `1/(1 - min(r_k, r_l))`. The
  code (tail.rs:1937-1960) matches it term for term. The identity, a reset and a joint flush are
  covered by monotone majorants with `delta >= 1`.
- Premise. (N2) has no control event at or after `N`. So the in-flight ramp at `M` is the last
  event's ramp. Rule 1 leaves the earlier ramp in place. Rule 3 gives the `r = 0` design recursion,
  with the mix inside `first_mix_row`.

### Adversarial histories (`evidence/adversarial.txt`)

- `G_p`, the real f32 kernel with the oracle's own worst-sign input `sign h(n, m)`, on 11 schedules
  at every launch rate plus a 108-schedule grid at 44.1 kHz. The largest peak is **5.678e4**
  (HPF to 10 Hz, then LPF to 10 Hz 16 frames later), and `g_p / peak >= 22.79` everywhere. The f32
  kernel agrees with the f64 exact row within 0.03 %.
- `G_t`, the adjoint oracle on 8 histories outside L2's scan (both sections moving at `N - 1` to
  the worst settled pair, a three-event restart chain, a disable then an enable, staggered moves).
  The largest trim x supremum is 60.19, and `g_t / sup >= 13.55` at every rate.
- No history goes below either certified gain.

Observation, not a finding. `apply_prepared_filter`'s doc says that an identity section that holds
restored non-zero integrators takes rule 4. That ramp would start from the identity recursion
(`Re p = 1`), which is outside the pole domain. #1407's ruling says every loadable word is "the
identity at rest with `+0.0` integrators" or near the hull. I did not find a path that restores
non-zero integrators under identity words. This premise is #1407/#1433's and #1485 does not add to
it. #1467's verifier listed the same case as outside L2's scan.

## (2) V-D4

Only `peak_gain` and `tail_gain` change. In the table, only those two entries move
(crates/builtins/src/tail.rs:579-599). `composition` keeps `stall(0)`, `stall(T)`, the
certificates and `D` exactly as before (diff checked). `live_bound_table_is_the_computed_live_bound_at_every_launch_rate`,
L3 (`D`, `sigma_p`, `sigma_t`), L4 and L5' pass. `G_p - G_t` is 6,385 to 6,400 mB, and raw
63.85 to 64.01 dB. No runtime reader of the live gains exists before #1379
(`grep` over `crates/*/src`, `hosts/*/src`).

## (3) Gates and factors

Release `tail_contract`: 22 passed, 11.2 s. My figures are the same as the record to the last
printed digit: `g_meas` 3.445581e4 / 3.428141e4 / 3.445984e4 / 3.428506e4; `r_p` stated 37.561 /
37.363 / 37.557 / 37.359; `s_scan` 6.016420e1 / 6.017214e1 / 6.012041e1 / 6.012030e1; `r_t`
stated 13.573 / 13.586 / 13.661 / 13.677. `F_P = 39.44` (37.561 x 1.05 = 39.439, rounded up) and
`F_T = 14.37` (13.677 x 1.05 = 14.361, rounded up). These are the values root confirmed. L3 and
L3' agree with the module within 9.2e-7 and 6.8e-8 relative, the module above. L2's scan is the
same as at `a059cdd03` (only a doc line is added).

## (4) Root's cost condition

- `input_section_live_bound_table` is a `pub const fn` that returns literals.
- Outside test code, only preparation's fallback (crates/builtins/src/lib.rs:3548) and the strip
  bound (crates/builtins-compiler/src/lib.rs:3279) read the table.
- `input_section_live_bound`, `live_cascade_composition`, `input_section_live_cascade` and
  `input_section_live_envelope` have callers only in tests. The builtins-compiler caller at :12044
  is inside `mod tests` (:5219). The host-core caller is in `tests/live_lanes.rs` and in
  `prepare.rs`'s `mod tests`.
- The required jobs are `test-release` (step "tail_contract in release", 15 min timeout),
  `test-debug-b` and `aarch64-debug`. Each is in `verdict`'s expectation table on the `full`
  route.
- `gh run view 37823829719` (main at `a059cdd03`) confirms the record's base durations: 10 min 43 s,
  7 min 54 s and 13 min 38 s. The added time (+39 s debug and +4.7 s release here) leaves each job
  inside its timeout.

## (5) Successor #1487

- The local spec `.github/ISSUE_SPECS/1487-diagnose-and-tighten-the-live-input-section-s-remaining-peak-and-tail-gain-loose.md`
  exists.
- GitHub #1487 is OPEN with the same title, and its body is the same as the spec except for one
  trailing newline.
- The STREAMS.md stream G row 53 is present.
- The spec carries root's ruling: diagnosis first, then a tighter certified bound if one exists,
  the same gates, low priority, after #1379.

## (6) Mutation runs (release `tail_contract`, files restored and checked by SHA-256)

| mutant | defect | red | green |
|---|---|---|---|
| MP | `g_p = (W_0(g,0) + delta^2 g) SLACK` in `composition` | gate 2 (ratio 472.87), table, L3 | 19 others |
| MT | #1467's `arrival_window`, `arrival_continuation`, `settled_input_gain` restored | gate 2t (694.39 to 694.99, every rate), table, L3' | 19 others |
| MC | `a059cdd03`'s math tail, builtins tail and tail_contract, with only gates 2 and 2t transplanted (one import added) | gate 2 (472.87), gate 2t (every rate) | the other 20 |
| L3P (phase A, my variant) | exposure reads its own zone, check included; `g_p` 4.43e5 | L3, table | gate 1, L2 |
| L3T (phase A, my variant) | settled pair drops `E*`; `g_t` 585 | L3', table | gate 1, L2 |

All confirm the record.

## Test value (one sentence per new or rewritten test)

- Gate 2 (in `live_peak_gain_bounds_the_real_kernel_on_its_worst_histories`): a bound that does not
  tighten, restored together with its table and its recomputation (#1466's `W_0` bound, ratio
  about 473), turns only gate 2 red (MC).
- Gate 2t (in `l2_at_rate`): #1467's window term restored coherently (ratio about 695) turns only
  gate 2t red (MC).
- Gate 1 (#1466 L1, rewritten): a derivation that drops the history's state (G1, `g_p = g delta^2`)
  is red only here, because L3 recomputes the same derivation. The four added histories have no
  unique catch (MINOR 1).
- L3 and L3' restated: a module that departs from the derivation while it stays above every
  measurement (L3P, L3T) is red only on them and on the table test.

## Findings

**MINOR 1. Gate 1's histories under-measure the real kernel's worst peak, so `r_p` about 37.6
overstates the remaining `G_p` looseness. The four added histories defend nothing that L1 does not
defend.** crates/builtins/tests/tail_contract.rs:2974-3065.
- The real f32 kernel reaches 5.678e4 with the oracle's own worst-sign input. That is 1.65 times
  (4.3 dB) above `g_meas` 3.4456e4, with `g_p / peak` 22.8, not 37.6. So the remaining looseness
  is at most 27.2 dB, not 31.5 dB.
- The alternating drive also beats L1 at every rate: HPF to 10 Hz, then LPF to 1 kHz 32 frames
  later, gives 3.5189e4 / 3.4946e4 / 3.4794e4 / 3.4584e4.
- The added histories (blocks of 1, polarity flip, LPF to 10 Hz, both moving at +4) peak at or below
  L1 at every rate. So none of them can go red while L1 stays green, and they add test time. The
  test name "on its worst histories" and the derivation's "Looseness" paragraph say more than the
  evidence shows.
- Soundness is not affected. Gate 2's absolute ceiling `F_p g_meas` = 1.359e6 does not depend on
  the reference.
- Root's ruling and #1487's Context quote `r_p` about 37.6. Fix in #1487 (gate 1 "widened with any
  history the diagnosis finds worse"): add the worst-sign-input histories to gate 1, then restate
  `g_meas`, `r_p` and its factor there. Correct #1487's Context figure before it starts.

**MINOR 2. Two clauses of the spec's Test value section are not true of the shipped bound** (spec
lines 188-192; root's rule: every test-value claim in a spec must be true).
- Gate 1, "or the trim's rounding". Dropping `(1 + u)` moves `g_p` by about 6e-8 relative, and
  `g_p` is 37 times above the measured peak. Gate 1 cannot see it.
- Gate 1t, "drops the window input's ramp". `Z*` sets `G_t` at every rate, so dropping `W*` leaves
  `g_t` unchanged. Even without the window's end states, `g_t` stays far above `s_scan` (L3T gives
  585 against 60). L2 cannot see it.
- The "or the trim" clauses are true (G1T is red on L2). Fix the text: remove the two false
  clauses, or name the test that catches each one.

**NIT 1.** GitHub #1485 still has the old title and body. The Amendment marks the sync as pending.
Synchronize it at close.

**NIT 2.** The derivation's "Gates" section (docs/derivations/1379-graph-tail-composition.md:761-782)
is outside the two authorized sections. It was edited, but it names #1485's gate 1 and not gates 2
and 2t. Lines 570 and 766 are new lines of 161 and 121 characters.

**NIT 3.** STREAMS.md's hot-file row (line 92) does not put #1487 in sequence, but #1487 edits the
same three hot files.

**NIT 4.** The attempt record repeats one sentence ("`s_scan` the largest trim x exact supremum,
`g_t` raw / stated ...", spec lines 360-362).

## Gates run (on the export of `9d7722276`)

- `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract`:
  22 passed.
- `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features
  math/lane,builtins/test-support,lane/test-support` (debug): 45 binaries, no failure, 3 min 52 s.
- `cargo build --locked --release -p audit && audit capi`: 0 allocations, 0 locks, 0 syscalls,
  0 violations, `pcm_digest` `cb10fbface44a3a4`.
- `scripts/check-builtins-fixtures.sh`: ok (50 files).
- `scripts/check-realtime-policy.sh`: ok (89 regions).
- `scripts/check-cross-targets.sh`: PASS.
- `scripts/run-wasm-gates.sh`: ok (native + wasm simd128 + V8 EQ loops, G5 digests).
- `scripts/check-workspace-policy.sh`: ok.
- `cargo fmt --all -- --check`: ok.
- Workspace clippy: not rerun. The aarch64 clippy `-D warnings` rows of `check-cross-targets.sh`
  cover the product crates, builtins and math included, with all targets.
- Authorized paths: `ae8b09318` touches the spec, the math and builtins tail modules,
  `tail_contract.rs` and the derivation. `9d7722276` touches the spec, `tail_contract.rs`,
  STREAMS.md and the new #1487 spec, which root ordered. The acked-batch question does not apply:
  no queue changes.
