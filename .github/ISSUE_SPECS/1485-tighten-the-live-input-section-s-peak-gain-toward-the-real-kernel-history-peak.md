# Tighten the live input section's peak and tail gains toward the real kernel's supremum

*(Title amended by root's ruling of 2026-10-08, "Amendment (root, 2026-10-08)" below; the issue
number is unchanged. Was: "Tighten the live input section's peak gain toward the real-kernel
history peak". The GitHub title and body are pending sync, and the spec's filename and its
`STREAMS.md` row keep the old title until that sync.)*

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-08 by root order: root's ruling of 2026-10-08 on #1466 attempt 1, a tightening
successor for the live `G_p`, on the model of *Tighten the fixed input section's peak gain past the
cascade triangle inequality* (#1468). The code this slice changes is created by #1466 (and its
statement by #1467); verify every anchor on the branch where both have landed before starting.
Amended 2026-10-08 by root: the live `G_t` tightening is folded in, so both live gains are
tightened against one oracle.

## Product outcome

An input section with a live input lane states a peak gain `G_p` within a measured factor of the
real kernel's largest measured history peak, and never below it, and a tail gain `G_t` within a
measured factor of the exact row-`l1` supremum of the real kernel over #1467's scanned ramp
histories, and never below it. #1466 certifies a sound live
`G_p` of about +144.24 dB (1.63e7 per unit of the input's peak at 44.1 kHz), about 53.5 dB above
the real kernel's peak under #1466's L1 history (+90.75 dB, 3.446e4). Every decibel of `G_p` feeds
#1379's composition: it raises `up(v)` downstream of every live strip, and with it `k(v)` (about one
more decade of the downstream node's `D` per 20 dB) and the input-peak bound `X*(v)` of the rest
branch. #1467 states a sound live `G_t` of 9,242 mB (+92.42 dB; `g_t` 4.1783e4 per unit of the
input's peak at the +24 dB trim word, at every launch rate), against the largest trim x exact
supremum over its scan of +35.58 dB (60.12 at 44.1 kHz; 60.08 / 59.62 / 59.62 at 48 / 88.2 /
96 kHz): about 57 dB loose (56.84-56.91 dB). Every decibel of `G_t` raises the residual that
#1379 carries through every live strip (its `g_t epsilon` term). No kernel, law, render path or
rendered bit changes.

## Context

- **What #1466 ships** (its Attempt record, prototype figures at `d12fd3940`): `G_p` is
  `ceil_mB(L)` with `L = outputs[0] + delta^2 g`: the window's first frame bounds the output at any
  frame of any admitted history with the current input zero (the pre-`N` analysis holds at every
  frame), and the current input adds `delta^2 g` through the two feedthroughs. Per unit of the
  input's peak at the +24 dB trim word: `L` = 1.629e7 (+144.24 dB) / 1.613e7 / 1.632e7 / 1.615e7
  at 44.1 / 48 / 88.2 / 96 kHz. What sets it is the pre-`N` second-section state bound
  `sup Psi Phi` of #1433.
- **The real-kernel history peak** (#1466 L1's history): a real `InputBuiltins` with a live input
  lane, trim +24 dB, both sections at `input_section_worst_case_pair(rate)`, driven by an
  alternating `+-1` input for 1,000,000 frames, then the HPF target moved to 10 Hz with the input
  running: peak `|y|` **3.446e4 (+90.75 dB)** / 3.428e4 / 3.446e4 / 3.429e4 (3.19 before the
  retarget). The ratio `g_p / g_meas` is about 473 (+53.5 dB).
- **The contract** the value must keep is #1379 Amendment 1 H1's (N1), as amended by *Split a
  node's flush stall into a peak stall and a tail stall* (#1484): for every input with
  `|x[n]| <= X` for all `n`, under any admitted history of live trim, polarity and filter targets
  (#1407's retarget words, #1408's trim ramp), `|y[n]| <= g_p X + sigma_p` for every `n`, for every
  reset pattern of the joint flush and every channel.
- **What #1467 ships** (its Attempt record, "Attempt 1, restarted under root's amendment", at
  `ae054b277`): `G_t = ceil_mB((g W* + g L_1 L_2) SLACK)` (`LiveComposition::tail_gain`,
  `crates/math/src/tail.rs`; derivation "The live tail gain `G_t` and the statement"), where `W*`
  is the window input's part (the late input on `M ..= M + 63`: a finite-horizon zone bound from
  zero state, then #1433's relative settled system from `M + 65`, per group) and `g L_1 L_2` the
  settled input's part. At 44.1 kHz the window frames give 2.7809e4 (+88.9 dB), the continuation
  4.1494e4 (+92.36 dB, which sets `W*` at every rate), the settled input 2.5126e2 (+48.0 dB).
  The continuation's zone and state supremum is where the looseness sits.
- **The adjoint oracle** (#1467 L2, `live_tail_gain_covers_the_exact_supremum_of_a_late_input_over_the_scanned_ramps`
  in `crates/builtins/tests/tail_contract.rs`, with `exact_row_supremum`, `forward_response` and
  `free_response_bounds`): the exact row-`l1` supremum of the time-varying live cascade over
  inputs `|x| <= 1` from `N` on, by the backward (adjoint) recursion with #1407's word mixtures,
  times the trim word. Its scan, per rate (336 histories): (a) the HPF ramped between every
  ordered pair of `{10 Hz, 100 Hz, 1 kHz, 10 kHz, the worst-case HPF, the identity}` with the LPF
  at the maximum, and the LPF between every ordered pair of `{10 Hz, 100 Hz, 1 kHz, 10 kHz, the
  maximum, the identity}` with the HPF at 10 Hz (60 ramps); (b) each with its control event at
  `N - s`, `s` in `{1, 16, 32, 48, 62}`; (c) the 36 settled pairs. It covers those histories in
  exact arithmetic, one section moving at a time, every input `|x| <= 1` from `N` (so every
  `M >= N`), every frame; it does not cover other endpoints, both sections moving, restarts,
  rule-3 resets, `f32` word rounding or the rounding deviation, and claims no more. The
  derivation, not the scan, carries the proof for every admitted history.
- **The registry rule** (f) of #1464: `G_t <= G_p`. Both gains move in this slice; the tightened
  `G_p` must stay at or above the tightened `G_t`.

## Decisions frozen for this slice

- **V-D1. Method.** The implementer's, for each gain, proven in the derivation note before the
  code. For `G_p`, the candidate the evidence points to is a tighter pre-`N` second-section state
  bound than `sup Psi Phi` (#1433's zone analysis), or a bound on the output over the retarget
  window that does not pass through the state supremum. For `G_t`, the candidate is a tighter
  bound on the window input's part `W*` (its continuation through #1433's relative settled system
  sets it at every rate), for example one that follows the late input's own response rather than
  a zone supremum. Each holds for every admitted history and every reset
  pattern, is computed on the control thread in `f64` with `math::log`/`math::exp`, and rounds up to
  millibels.
- **V-D2. Never below the reference value.** A certified `G_p` below the real kernel's largest
  measured history peak is unsound and stops the slice. A certified `g_t` below the trim word
  times #1467's adjoint row-`l1` supremum over the stated scan (at any launch rate) is unsound and
  stops the slice.
- **V-D3. Each factor is set from a measurement, confirmed by root (as #1468 P-D3).** Two
  factors: `F_p` for `G_p` and `F_t` for `G_t`.
  - *Step 1, before the gates are frozen.* On gate 2's histories, measure the real kernel's peak
    `g_meas` per unit of the input's peak, and compute the certified `g_p` of the method V-D1
    chooses; record both and the ratio `r_p = g_p / g_meas` per history and rate. With
    `s_scan(rate)` the largest trim x exact supremum over #1467's scan (or the widened scan, gate
    1t) at the rate, compute the certified `g_t` and record `g_t`, `s_scan` and `r_t = g_t /
    s_scan` per rate. All in the attempt record.
  - *Step 2, the factors.* `F_p = r_p,max (1 + m_p)` and `F_t = r_t,max (1 + m_t)`, each `r_max`
    the largest ratio over its gated histories and rates, each `m` a stated margin (default 5 %,
    with its reason), rounded up to two decimals. Root confirms each factor in the attempt record
    before its gate is committed.
  - No ceiling on either factor is frozen in advance; root rules on each measured factor (a sound
    method that closes only part of the 53.5 dB (`G_p`) or of the about 57 dB (`G_t`) is still a
    tightening, and root decides whether it closes the slice or needs a successor).
- **V-D4. Only the live `G_p` and `G_t` move.** The live `T_decay`, `T_rest`, both rests, `P*`,
  `D`, `sigma_p` and `sigma_t` stay exactly as #1433, #1466 and #1467 state them; the fixed
  section's values do not move. `G_p >= G_t` holds after the change, raw and in millibels (else
  the slice stops).
- **V-D5. Stop rule.** If the derivation cannot prove a tighter bound for either gain, stop and
  report to root. #1466's `G_p` and #1467's `G_t` stay in production; they are sound. There is no
  fallback value and no gate change.
- **V-D6. Cost.** The live bound depends only on the rate and preparation reads it from
  `input_section_live_bound_table`, so the construction's cost is not a preparation cost; it is
  recorded descriptively (one invocation, ms per rate).

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade under #1407's live words (#1433's zone analysis); (N1); the
  `G_t` contract of #1379 Amendment 1 H1 (#1467's "The contract for `G_t`"); V-D1.
- **Coefficient and update rules:** #1407's retarget through designs and their mixtures; #1408's
  trim ramp; no change.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; no tail or rest value moves (V-D4); only the two gains.
- **Units and smoothing:** gains in millibels; no smoothing.
- **Denormal/NaN:** unchanged.
- **Citations:** as #1329 and #1433 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP],
  [SMITH-SASP]; Higham for the rounding model).
- **Fixtures and objective tests:** gates 1, 1t, 2, 2t and 3. **Benchmarks:** none (V-D6). **Listening:** none; no
  rendered bit moves.

## Deliverables

1. V-D3's step 1 record and the chosen `F_p` and `F_t`, each with its margin, each confirmed by
   root before its gate is committed.
2. The tightened live `G_p` and `G_t` in `crates/math/src/tail.rs`, rounded in
   `crates/builtins/src/tail.rs`, and `input_section_live_bound_table`'s live `peak_gain` and
   `tail_gain` restated.
3. The derivation in `docs/derivations/1379-graph-tail-composition.md` (the live `G_p` section and
   "The live tail gain `G_t` and the statement").
4. Gates 1, 1t, 2, 2t and 3 in `crates/builtins/tests/tail_contract.rs`; #1466's L3
   recomputation of `G_p` and #1467's L3' recomputation of `G_t` restated to the new formulas.
5. If #1379 has landed: each moved graph digest re-pinned one at a time with its reason.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

1. **Sound on the real kernel's worst histories.** On #1466 L1's history and on every further
   history the attempt names with its reason (candidates: the LPF retargeted after a drive that
   builds its state, the HPF retarget of L1 at other block sizes, a polarity or trim change inside
   the retarget window), each at trim +24 dB with both sections at `input_section_worst_case_pair(rate)` or the history's own
   designs: the largest `|y|` over the run is at most `g_p + sigma_p` per unit of the input's peak.
1t. **`G_t` sound against the adjoint oracle.** #1467's L2 stays, its scan unchanged or widened
   (each added history with its reason; never narrowed), and passes against the tightened `g_t`
   at every launch rate: `g_t` is at least the trim word times the exact row-`l1` supremum over
   every scanned history. The record states the scan as #1467's does: which ramps, which start
   offsets, which designs, what it covers and what it does not.
2. **Within the factor of measured, never below it.** With `g_meas` the largest measured peak over
   gate 1's histories at the rate: `g_meas <= g_p <= F_p g_meas`, with `F_p` from V-D3. Record
   `g_meas`, `g_p` and the ratio per history and rate.
2t. **Within the factor of the supremum, never below it.** With `s_scan` the largest trim x exact
   supremum over gate 1t's scan at the rate: `s_scan <= g_t <= F_t s_scan`, with `F_t` from V-D3.
   Record `s_scan`, `g_t` and the ratio per rate.
3. **Nothing else moves.** Every other `tail_contract` assertion passes unchanged (the live
   `T_decay`, `T_rest`, both rests, `P*`, `D` and both stalls, and every fixed value), `G_p >= G_t`
   holds raw and in millibels (#1467 L5');
   `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` passes with only `peak_gain`
   and `tail_gain` restated; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm G5 digests, the builtins
   PCM fixtures).
4. **Commands:** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
   --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked --release -p
   builtins --features builtins/test-support --test tail_contract`; `cargo build --locked --release
   -p audit && ./target/release/audit capi`; `bash scripts/check-builtins-fixtures.sh .
   target/release/audit`; `bash scripts/run-wasm-gates.sh`; `bash
   scripts/check-workspace-policy.sh`; `cargo clippy --locked --workspace --all-targets -- -D
   warnings`; `cargo fmt --all -- --check`; if #1379 has landed, also the `test-debug-a` workspace
   command, `bash scripts/check-graph-determinism.sh`, `cargo run --locked -p graph-compiler --bin
   graph_fixture -- --check` and `cargo test --locked --release -p host-core --test tail_composition
   -- --include-ignored`.

The attempt record carries a mutation table (each defect applied, the named test run, the file
restored) with at least the mutants below; each is red.

## Test value

- Gate 1: a tightening that drops the retarget transient (for example a `G_p` from the settled
  designs' `l1`, about +35 dB with the trim) or the trim's rounding falls below the real kernel's
  measured peak.
- Gate 1t: a tightening of `G_t` that drops the window input's ramp or the trim falls below the
  exact supremum (#1467's M2 mutant shows the gate red for it).
- Gate 2: a step that does not tighten (#1466's `sup Psi Phi` bound left in place, about 473x) misses
  the factor `F_p`; no other test compares the live certified gain with the measured one from above.
- Gate 2t: a step that does not tighten `G_t` (#1467's window term left in place, about 57 dB, a
  ratio of about 695) misses `F_t`; no other test bounds the live `g_t` from above against the
  oracle.
- Gate 3: a change that moves `T_decay`, `D` or a stall with a gain (a shared state bound
  tightened in place) is red on #1433's, #1466's and #1484's pins.

## Non-goals

- The live stalls (#1466), the fixed `G_p` (#1468).
- Any kernel, flush or law change. Any change to `INPUT_BOUND_BUDGET_FRAMES` or the section charge.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions, stream
  A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live `G_p` section and "The live tail gain
  `G_t` and the statement")
- Only if #1379 has landed: `crates/graph-compiler/tests/track_delay.rs` (the digest),
  `crates/graph-compiler/tests/tail_composition.rs` (an asserted value that moves, with its reason),
  `fixtures/graph/v1/direct-route.*` (regenerated by `graph_fixture`), `fixtures/graph/MANIFEST.tsv`,
  `crates/host-core/tests/live_lanes.rs` and `crates/host-core/tests/tail_composition.rs` (an
  asserted tail that moves, with its reason) (named exceptions, stream A's)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Hazards

- **Every admitted history.** Gate 1 measures a few worst histories; the bound must hold for all of
  them. A bound that matches the measured histories but drops a zone or window term is unsound; the
  derivation, not the gate, carries the proof, and the verifier checks it as a proof. The same
  holds for `G_t`: gate 1t's scan covers only the histories it states (#1467 L2), and a `G_t` that
  clears the scan but drops a window or settled-group term elsewhere is unsound.
- **Shared files.** `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs` and `tail_contract.rs`
  are shared with #1466, #1467 and #1468 (`STREAMS.md`); this slice follows #1467 and lands in
  either order with #1468, the later slice rebases.
- **Canonical digests.** If #1379 has landed, the graph's canonical text prints each node's gains,
  so a tighter value moves digests and some reported tails (shorter only); any other moved byte
  stops the slice.

## Dependencies

- *Certify the live input section's decay, peak gain and flush stall* (#1466): the value this slice
  tightens and its L1 history.
- *Derive the live input section's tail gain and state its composition* (#1467), landed first:
  the live `G_t` this slice tightens and that bounds `G_p` from below, the adjoint oracle and its
  scan (#1467 L2) that gates 1t and 2t read, the L3' recomputation this slice restates, the live
  statement whose `peak_gain` and `tail_gain` this slice restates, and the shared files.
- *Split a node's flush stall into a peak stall and a tail stall* (#1484): (N1)'s `sigma_p`.
- In either order with *Tighten the fixed input section's peak gain past the cascade triangle
  inequality* (#1468), the later slice rebases. It may land before or after #1379; if after, it
  re-pins as Deliverable 5 states.

## Amendment (root, 2026-10-08)

Root's ruling, recorded as given: **the live `G_t` tightening is folded into #1485, so both live
gains are tightened against one oracle.** (#1467's own Amendment, point 3, moved the tightening of
its window term out of #1467 and into this issue.) #1467 states `G_t` = 9,242 mB (+92.4 dB)
against the scanned supremum's +35.6 dB, about 57 dB loose.

- The outcome, scope and gates now cover `G_p` **and** `G_t` (title, Product outcome, Context,
  V-D1 to V-D5, Deliverables, gates 1t and 2t, Test value, Non-goals, Authorized paths).
- Each stated gain is at most a measured factor times its exact real-kernel reference, and
  **never below it**: `G_p` against the real kernel's measured history peak (gate 2, `F_p`), `G_t`
  against #1467's adjoint row-`l1` supremum over the stated scan (gate 2t, `F_t`).
- Root confirms each factor, as in #1468 (V-D3).
- #1467 is a dependency (Dependencies).
- **Pending GitHub sync:** the issue's title ("Tighten the live input section's peak and tail
  gains toward the real kernel's supremum") and body. The spec's filename and its `STREAMS.md`
  row keep the old title until then; the row's dependencies (#1466, #1467, #1484) already include
  #1467.

## Attempt record

### Attempt 1, phase A (implementer, 2026-10-08)

Phase A per root's order: the tighter, sound live `G_p` and `G_t` (V-D1, V-D2, V-D4, the
derivation), every gate except the tightness gates 2 and 2t, and V-D3 step 1 (the measurements and
the proposed factors). Gates 2 and 2t wait for root's confirmation of `F_p` and `F_t`. #1379 has
not landed, so Deliverable 5 and the "only if #1379 has landed" commands do not apply.

**Method (V-D1).** Both proofs are in `docs/derivations/1379-graph-tail-composition.md`, "(N1):
`G_p` and `sigma_p`" and "(R) The late part along the in-flight ramps" / "`G_t`"; the code is
`math::tail::LiveBound::{exposure, peak_gain, tail_gain}`.

* `G_p`: #1466's `W_0(g, 0) + delta^2 g` charged every frame of the HPF's output to the LPF
  frequency-blind (`gamma_2 iota rho_ramp^m`, summing to `7.6e4` per unit). The new bound follows
  the LPF along its own zones (both sections share #1433's pole domain, so the zone constants hold
  for the LPF's words): `|y[n]| <= delta a_n + sum_m Gamma(m) a_(n-m)`, `a_j` #1433's HPF output
  majorant and `Gamma(m)` the LPF's largest exposure `m` frames later over every neighbour path
  (a DP over 1,024 frames and a checked geometric tail at `rho_ramp (1 + 2^-40)`). Its envelope is
  split at every `m0`: the fast part on the HPF's largest output `A = max_k (h q_k + omega) Phi_k +
  delta`, the slow part telescoped through #1433's potential (Abel summation with the
  non-decreasing weights) with each frame's charge `C` at one zone. Reset, rule 3, disable,
  identity and channel copy only lower or are covered (the derivation). The split into the input
  part and the flush part is (P3)'s, so `sigma_p` (W_0(0, F) by (P2)) is untouched. No window, no
  cap: the bound holds at every frame of every admitted history.
* `G_t`: (N2) has no control event at or after `N <= M`, so on the window's frames each section's
  words follow one in-flight ramp of #1407 (rule 4: all six words from a reachable word toward a
  design; rule 2: a frozen recursion, then zero state and the identity; rule 3: the design with
  zero state). The word at ramp update `i` has `Re p` within `eta` of `(1 - i/R) Re p(c) + (i/R)
  Re p(t)` (`Re p` is linear in the words), so it lies in a zone meeting that interval. The module
  enumerates every start zone, target zone and update count left at `M` (and every frozen word),
  runs the HPF from zero with a unit input and the LPF from zero on the HPF's largest output per
  window frame, each along its own zones; then per pair of settled zones a closed form from the
  window's end state. #1467's `Phi^(j)` (any move every frame), frequency-blind window and `tau`
  continuation are gone, and so are `arrival_window`, `arrival_continuation` and
  `settled_input_gain`.

**Values (V-D2, V-D4).** Raw (`live_cascade_composition` at the +24 dB trim word) and stated
(millibels); `D`, `T_decay`, `T_rest`, both rests, `P*`, `sigma_p` and `sigma_t` are unchanged at
every rate (`live_bound_table_is_the_computed_live_bound_at_every_launch_rate` passes with only
`peak_gain` and `tail_gain` restated; L3, L4 and L5' unchanged otherwise).

| rate | old `G_p` mB | new `G_p` raw | new `G_p` mB | old `G_t` mB | new `G_t` raw | new `G_t` mB | `G_p - G_t` |
|---|---|---|---|---|---|---|---|
| 44.1 kHz | 14,424 | 1.294092e6 | 12,224 | 9,242 | 8.157736e2 | 5,824 | 6,400 mB |
| 48 kHz | 14,416 | 1.280357e6 | 12,215 | 9,242 | 8.166610e2 | 5,825 | 6,390 mB |
| 88.2 kHz | 14,426 | 1.294087e6 | 12,224 | 9,242 | 8.212877e2 | 5,829 | 6,395 mB |
| 96 kHz | 14,417 | 1.280352e6 | 12,215 | 9,242 | 8.217405e2 | 5,830 | 6,385 mB |

`G_p` is 22.0 dB tighter at every rate and `G_t` 34.1 to 34.2 dB tighter; `G_p >= G_t` holds raw
and in millibels (L5'). Parts (gate L3's recomputation, per unit of `G` at 44.1 kHz): `G_p`'s
`m0 = 16`, `Gamma(1) = 0.870`, `Gammabar(16) = 0.267`, `sum Gamma = 7.27e3`, `A = 7.27e3`,
`C = 5.25`, `V = sup Psi Phi = 1.60e5`; terms `delta A` 7.27e3, fast 6.07e3, potential 3.02e4,
slow charges 3.82e4. `G_t`'s window part `W* g` 433.7 and settled part `Z* g` 815.8 (the settled
part sets it at every rate). L3 and L3' recompute both in plain `f64` with no shared code (the
LPF exposure DP, the split over every `m0`, the ramp paths written per path, not as the module's
vectorised sweep) and agree within 1.2e-6 (`G_p`) and 7e-8 (`G_t`) relative.

**V-D3 step 1.**

*`G_p`, gate 1's histories* (`live_peak_gain_bounds_the_real_kernel_on_its_worst_histories`;
release; trim +24 dB, both sections at `input_section_worst_case_pair(rate)`, an alternating `+-1`
drive for 1,000,000 frames, the events at the drive's end, 200,000 frames more). Peak `|y|` with
the trim (`g_meas` is per unit of the input's peak, `X = 1`), `g_p` raw, `r_p = g_p / peak`:

| history (reason) | 44.1 kHz | 48 kHz | 88.2 kHz | 96 kHz |
|---|---|---|---|---|
| L1: HPF to 10 Hz (#1466 L1) | 3.4456e4, r_p 37.56 | 3.4281e4, 37.35 | 3.4460e4, 37.55 | 3.4285e4, 37.34 |
| L1 in blocks of 1 (ramp allowance largest at block 1) | 3.4456e4, 37.56 | 3.4281e4, 37.35 | 3.4460e4, 37.55 | 3.4285e4, 37.34 |
| L1, polarity flipped 8 frames in (a change inside the window) | 3.4456e4, 37.56 | 3.4281e4, 37.35 | 3.4460e4, 37.55 | 3.4285e4, 37.34 |
| LPF to 10 Hz (the LPF's top state the slow exposure charges) | 1.4162e1, 91,376 | 1.3173e1, 97,194 | 7.8891e0, 164,035 | 7.4403e0, 172,084 |
| HPF to 10 Hz, LPF to 10 Hz 4 frames later (both moving) | 3.1115e4, 41.59 | 3.0958e4, 41.36 | 3.1122e4, 41.58 | 3.0964e4, 41.35 |
| **`g_meas`** (largest) | 3.445581e4 | 3.428141e4 | 3.445984e4 | 3.428506e4 |
| **`g_p`** raw / stated | 1.294092e6 / 1.294196e6 | 1.280357e6 / 1.280855e6 | 1.294087e6 / 1.294196e6 | 1.280352e6 / 1.280855e6 |
| **`r_p`** raw / stated | 37.558 / 37.561 | 37.348 / 37.363 | 37.554 / 37.557 | 37.344 / 37.359 |

Before (#1466's `g_p`): `r_p` 472.8 / 470.5 / 473.5 / 471.2. Histories measured during the
search and not gated (all at or below L1's peak at 44.1 kHz): the HPF retargeted to 0.05 fs to
0.45 fs (1.837e4 to 3.263e4), the LPF to the same (2.355e3 to 7.642e3), a drive at the top design's
pole frequency with the HPF, the LPF or both to 10 Hz (4.60e3, 1.93e3, 3.10e3), L1 then the HPF
back to the maximum at +2 to +48 (1.83e4 to 3.4456e4), L1 then the LPF to 10 Hz at +2 to +48 (2.69e4
to 3.4456e4), L1 with a polarity flip at 0 to +48 (3.4427e4 to 3.4456e4), twenty alternating
restarts every 4 frames (3.11e4), the HPF to 1 kHz and to 100 Hz (3.365e4, 3.438e4).

*`G_t`, gate 1t's scan* (L2 unchanged, neither narrowed nor widened; release). The scan, as L2's
doc states it: per launch rate 3,213 histories whose words the real kernel records frame by frame
(#1407's rules 1 to 4 and its event timing), on the cutoffs `{identity, 10 Hz, 100 Hz, 200 Hz,
500 Hz, 1 kHz, 10 kHz, one f32 below the maximum, the maximum}`: (a) one section retargeted between
every ordered pair (a disable, an enable from rest or an all-six ramp), the other fixed, the event
before `N - s`, `s` in `{1, 2, 8, 16, 32, 48, 63}`; (b) every settled pair; (c) one section
retargeted twice on five cutoffs at four event-time pairs; (d) both sections retargeted on four
cutoffs each at four event-time pairs. It covers those histories only, the input from `N` in
exact arithmetic on the kernel's `f32` words; not other endpoints, rule-3 resets, the rounding
deviation (the derivation's) or the flush part (`sigma_t`). `s_scan` the largest trim x exact
supremum, `g_t` raw / stated (L2 compares the stated value), `r_t = g_t / s_scan`:
`s_scan` the largest trim x exact supremum, `g_t` raw / stated (L2 compares the stated value),
`r_t = g_t / s_scan`:

| rate | `s_scan` (history) | `g_t` raw / stated | `r_t` raw / stated |
|---|---|---|---|
| 44.1 kHz | 6.016420e1 (HPF 0 -> 200 Hz at N - 1, LPF 22049.482 -> 22049.48 Hz at N - 1) | 8.157736e2 / 8.165824e2 | 13.559 / 13.573 |
| 48 kHz | 6.017214e1 (section 0 0 -> 23999.432 Hz at N - 2 -> 200 Hz at N - 1) | 8.166610e2 / 8.175230e2 | 13.572 / 13.586 |
| 88.2 kHz | 6.012041e1 (HPF 0 -> 200 Hz at N - 1, LPF 44098.965 -> 44098.96 Hz at N - 1) | 8.212877e2 / 8.212966e2 | 13.661 / 13.661 |
| 96 kHz | 6.012030e1 (section 0 0 -> 500 Hz at N - 1, other 47998.867 Hz) | 8.217405e2 / 8.222426e2 | 13.668 / 13.677 |

Before (#1467's `g_t`, 9,242 mB): `r_t` 694.5 / 694.4 / 695.0 / 695.0.

*Proposed factors (V-D3 step 2), for root's confirmation.* Each ratio taken on the stated
(millibel) gain, which is what the statement carries and is at least the raw one, so a gate on
either reading holds:

* `F_p = r_p,max (1 + m_p) = 37.561 x 1.05 = 39.439`, rounded up: **`F_p = 39.44`**
  (`r_p,max` at 44.1 kHz; +31.5 dB).
* `F_t = r_t,max (1 + m_t) = 13.677 x 1.05 = 14.360`, rounded up: **`F_t = 14.37`**
  (`r_t,max` at 96 kHz; +22.7 dB).
* Margins `m_p = m_t = 5 %` (the default). Reason: every quantity in both gates is deterministic
  (the kernel's `f32` peak and the bound under the canonical floating-point environment, the
  oracle's exact arithmetic), so the margin covers no run-to-run noise; it is headroom for the
  millibel rounding of the statement (at most 1 mB, 0.012 %) and for a later restatement of a
  constant at rounding level, while a step that does not tighten (#1466's `G_p` at `r_p` about
  473, #1467's `G_t` at `r_t` about 695) stays more than 12 times outside either factor.

**Cost (V-D6, descriptive).** One `live_cascade_composition` per rate, release, x86-64-v3
(AMD EPYC 7313P), one invocation: 243.9 / 243.5 / 243.7 / 245.1 ms at 44.1 / 48 / 88.2 / 96 kHz
(#1466 and #1467 together: about 0.5 ms). Almost all of it is `tail_gain`'s path sweep (146 start
zones x 146 target zones x 65 update counts x 64 frames, per section, vectorisable over the update
counts); `peak_gain`'s exposure DP (1,024 frames x 146 zones) is a few ms. A debug build takes
5.13 s per rate. Preparation never computes it (`input_section_live_bound_table`). The debug
`tail_contract` binary now takes 63.6 s, its longest test L3 (both the module's composition and
the oracle's per-path recomputation at four rates); the debug gate command took 3 min 47 s.

**Gates run (phase A), all green.**

* `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference --features
  math/lane,builtins/test-support,lane/test-support` (debug): pass (tail_contract 20 passed,
  2 ignored release-scale).
* `cargo test --locked --release -p builtins --features builtins/test-support --test
  tail_contract`: 22 passed (gate 1, gate 1t = L2, L3/L3', L4, L5', the table test, every #1329,
  #1433, #1465, #1466, #1467 and #1484 assertion).
* Gate 3: the table test passes with only `peak_gain` and `tail_gain` restated; `cargo build
  --locked --release -p audit && ./target/release/audit capi`: 0 allocations, 0 deallocations,
  0 locks, 0 syscalls, 0 violations, `pcm_digest` `cb10fbface44a3a4` (unchanged);
  `scripts/check-builtins-fixtures.sh . target/release/audit`: ok (50 files);
  `scripts/run-wasm-gates.sh`: ok (native + wasm simd128 + V8 EQ loops, G5 digests).
* `cargo clippy --locked --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`,
  `scripts/check-workspace-policy.sh`, `scripts/check-realtime-policy.sh` (89 regions),
  `scripts/check-cross-targets.sh` (PASS): green.
* The worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh` with an empty-afterwards `TMPDIR`): green.

**Mutation table** (release; each defect applied, the named tests run, the files restored and
checked by digest).

| mutant | defect | red | green |
|---|---|---|---|
| G1 | the derivation keeps only the current input through both feedthroughs (`g_p = g delta^2`), in the module and the oracle | gate 1 (`peak 3.4456e4 exceeds g_p + sigma_p = 1.585e1`) | L3 (both agree) |
| G1T | the derivation drops the trim from `G_t` (`g_t = max(W*, Z*)`), module and oracle | gate 1t = L2 (`the exact supremum 53.1 exceeds g_t 51.9`, 88.2 and 96 kHz) | L3', L5' |
| G3 | a shared state bound tightened in place (`Phi` halved in `live_zones`) | the table test, L3 (`group H_0: module 6.84e6 against the recomputed 1.36e7`) | gate 1, L2 |
| L3P | `G_p`'s exposure reads its own zone, not its neighbours (module only; `g_p` falls to 3.25e5, still above every measured peak) | L3 (`G_p: module 3.248e5 against the recomputed 1.294e6`), the table test | gate 1, L2 |
| L3T | `G_t`'s settled pair drops the window's first-section state (module only; `g_t` falls to 606, still above the scan) | L3' (`G_t: module 6.062e2 against the recomputed 8.158e2`), the table test | gate 1, L2 |

Test value (worker rule): gate 1 is the only test that turns red when the derivation itself drops
the history's state (G1: L3 recomputes the same derivation and stays green); L2 is the only one
for a `G_t` derivation below the exact supremum (G1T); L3/L3' are the only ones for a module that
departs from the derivation while staying above every measurement (L3P, L3T: gate 1 and L2 stay
green, and a table restated with the module's values would leave L3 alone red). The table test
is red for any value change, as it must be until the table is restated.

**Open for phase B.** Root confirms `F_p` and `F_t`; then gate 2 (`g_meas <= g_p <= F_p g_meas`
over gate 1's histories per rate) and gate 2t (`s_scan <= g_t <= F_t s_scan` over L2's scan per
rate) are committed with their mutants (#1466's `W_0` bound restored: red; #1467's window term
restored: red).
