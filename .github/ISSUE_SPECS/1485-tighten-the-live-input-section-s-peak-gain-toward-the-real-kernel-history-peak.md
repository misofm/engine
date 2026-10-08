# Tighten the live input section's peak gain toward the real-kernel history peak

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-08 by root order: root's ruling of 2026-10-08 on #1466 attempt 1, a tightening
successor for the live `G_p`, on the model of *Tighten the fixed input section's peak gain past the
cascade triangle inequality* (#1468). The code this slice changes is created by #1466 (and its
statement by #1467); verify every anchor on the branch where both have landed before starting.

## Product outcome

An input section with a live input lane states a peak gain `G_p` within a measured factor of the
real kernel's largest measured history peak, and never below it. #1466 certifies a sound live
`G_p` of about +144.24 dB (1.63e7 per unit of the input's peak at 44.1 kHz), about 53.5 dB above
the real kernel's peak under #1466's L1 history (+90.75 dB, 3.446e4). Every decibel of `G_p` feeds
#1379's composition: it raises `up(v)` downstream of every live strip, and with it `k(v)` (about one
more decade of the downstream node's `D` per 20 dB) and the input-peak bound `X*(v)` of the rest
branch. No kernel, law, render path or rendered bit changes.

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
- **The registry rule** (f) of #1464: `G_t <= G_p`. #1467 derives the live `G_t`; the tightened
  `G_p` must stay at or above it.

## Decisions frozen for this slice

- **V-D1. Method.** The implementer's, proven in the derivation note before the code. The
  candidate the evidence points to is a tighter pre-`N` second-section state bound than
  `sup Psi Phi` (#1433's zone analysis), or a bound on the output over the retarget window that
  does not pass through the state supremum. It holds for every admitted history and every reset
  pattern, is computed on the control thread in `f64` with `math::log`/`math::exp`, and rounds up to
  millibels.
- **V-D2. Never below the measured value.** A certified `G_p` below the real kernel's largest
  measured history peak is unsound and stops the slice.
- **V-D3. The factor is set from a measurement, confirmed by root (as #1468 P-D3).**
  - *Step 1, before the gate is frozen.* On gate 2's histories, measure the real kernel's peak
    `g_meas` per unit of the input's peak, and compute the certified `g_p` of the method V-D1
    chooses. Record both and the ratio `r = g_p / g_meas` per history and rate in the attempt
    record.
  - *Step 2, the factor.* `F = r_max (1 + m)`, with `r_max` the largest measured ratio over the
    gated histories and rates, and `m` a stated margin (default 5 %, with its reason), rounded up
    to two decimals. Root confirms `F` in the attempt record before gate 2 is committed.
  - No ceiling on `F` is frozen in advance; root rules on the measured `F` (a sound method that
    closes only part of the 53.5 dB is still a tightening, and root decides whether it closes the
    slice or needs a successor).
- **V-D4. Only the live `G_p` moves.** The live `T_decay`, `T_rest`, both rests, `P*`, `D`,
  `sigma_p`, `sigma_t` and `G_t` stay exactly as #1433, #1466 and #1467 state them; the fixed
  section's values do not move. `G_p >= G_t` holds after the change (else the slice stops).
- **V-D5. Stop rule.** If the derivation cannot prove a tighter bound, stop and report to root.
  #1466's value stays in production; it is sound. There is no fallback value and no gate change.
- **V-D6. Cost.** The live bound depends only on the rate and preparation reads it from
  `input_section_live_bound_table`, so the construction's cost is not a preparation cost; it is
  recorded descriptively (one invocation, ms per rate).

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade under #1407's live words (#1433's zone analysis); (N1); V-D1.
- **Coefficient and update rules:** #1407's retarget through designs and their mixtures; #1408's
  trim ramp; no change.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; no tail value moves (V-D4).
- **Units and smoothing:** gains in millibels; no smoothing.
- **Denormal/NaN:** unchanged.
- **Citations:** as #1329 and #1433 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP],
  [SMITH-SASP]; Higham for the rounding model).
- **Fixtures and objective tests:** gates 1-3. **Benchmarks:** none (V-D6). **Listening:** none; no
  rendered bit moves.

## Deliverables

1. V-D3's step 1 record and the chosen `F` with its margin, confirmed by root before gate 2 is
   committed.
2. The tightened live `G_p` in `crates/math/src/tail.rs`, rounded in `crates/builtins/src/tail.rs`,
   and `input_section_live_bound_table`'s live `peak_gain` restated.
3. The derivation in `docs/derivations/1379-graph-tail-composition.md` (the live `G_p` section).
4. Gates 1-3 in `crates/builtins/tests/tail_contract.rs`; #1466's L3 recomputation of `G_p`
   restated to the new formula.
5. If #1379 has landed: each moved graph digest re-pinned one at a time with its reason.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

1. **Sound on the real kernel's worst histories.** On #1466 L1's history and on every further
   history the attempt names with its reason (candidates: the LPF retargeted after a drive that
   builds its state, the HPF retarget of L1 at other block sizes, a polarity or trim change inside
   the retarget window), each at trim +24 dB with both sections at `input_section_worst_case_pair(rate)` or the history's own
   designs: the largest `|y|` over the run is at most `g_p + sigma_p` per unit of the input's peak.
2. **Within the factor of measured, never below it.** With `g_meas` the largest measured peak over
   gate 1's histories at the rate: `g_meas <= g_p <= F g_meas`, with `F` from V-D3. Record
   `g_meas`, `g_p` and the ratio per history and rate.
3. **Nothing else moves.** Every other `tail_contract` assertion passes unchanged (the live
   `T_decay`, `T_rest`, both rests, `P*`, `D`, both stalls and `G_t`, and every fixed value);
   `live_bound_table_is_the_computed_live_bound_at_every_launch_rate` passes with only `peak_gain`
   restated; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm G5 digests, the builtins
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
- Gate 2: a step that does not tighten (#1466's `sup Psi Phi` bound left in place, about 473x) misses
  the factor `F`; no other test compares the live certified gain with the measured one from above.
- Gate 3: a change that moves `T_decay`, `D` or a stall with the gain (a shared state bound
  tightened in place) is red on #1433's, #1466's and #1484's pins.

## Non-goals

- The live `G_t` (#1467), the live stalls (#1466), the fixed `G_p` (#1468).
- Any kernel, flush or law change. Any change to `INPUT_BOUND_BUDGET_FRAMES` or the section charge.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions, stream
  A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live `G_p` section)
- Only if #1379 has landed: `crates/graph-compiler/tests/track_delay.rs` (the digest),
  `crates/graph-compiler/tests/tail_composition.rs` (an asserted value that moves, with its reason),
  `fixtures/graph/v1/direct-route.*` (regenerated by `graph_fixture`), `fixtures/graph/MANIFEST.tsv`,
  `crates/host-core/tests/live_lanes.rs` and `crates/host-core/tests/tail_composition.rs` (an
  asserted tail that moves, with its reason) (named exceptions, stream A's)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Hazards

- **Every admitted history.** Gate 1 measures a few worst histories; the bound must hold for all of
  them. A bound that matches the measured histories but drops a zone or window term is unsound; the
  derivation, not the gate, carries the proof, and the verifier checks it as a proof.
- **Shared files.** `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs` and `tail_contract.rs`
  are shared with #1466, #1467 and #1468 (`STREAMS.md`); this slice follows #1467 and lands in
  either order with #1468, the later slice rebases.
- **Canonical digests.** If #1379 has landed, the graph's canonical text prints each node's gains,
  so a tighter value moves digests and some reported tails (shorter only); any other moved byte
  stops the slice.

## Dependencies

- *Certify the live input section's decay, peak gain and flush stall* (#1466): the value this slice
  tightens and its L1 history.
- *Derive the live input section's tail gain and state its composition* (#1467): the live `G_t`
  that bounds `G_p` from below, the live statement whose `peak_gain` this slice restates, and the
  shared files, landed first.
- *Split a node's flush stall into a peak stall and a tail stall* (#1484): (N1)'s `sigma_p`.
- In either order with *Tighten the fixed input section's peak gain past the cascade triangle
  inequality* (#1468), the later slice rebases. It may land before or after #1379; if after, it
  re-pins as Deliverable 5 states.

## Attempt record

(none yet)
