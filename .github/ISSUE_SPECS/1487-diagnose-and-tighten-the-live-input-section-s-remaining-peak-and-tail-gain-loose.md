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
  gain to its reference): `r_p` = 22.80 / 22.70 / 22.80 / 22.70 (+27.2 dB) against `g_meas`, the
  largest real-kernel peak over gate 1's six histories (5.6766e4 at 44.1 kHz, per unit of the
  input's peak at the +24 dB trim word, from the worst-sign history that #1485's follow-ups added
  to gate 1); `r_t` = 13.57 / 13.59 / 13.66 / 13.68 (+22.7 dB) against
  `s_scan`, the largest trim x exact row-`l1` supremum over #1467 L2's scan (6.016e1 at 44.1 kHz).
  #1485's factors: `F_p = 23.94`, `F_t = 14.37` (`F_P`, `F_T` in
  `crates/builtins/tests/tail_contract.rs`).
- **The parts of each bound** (#1485's L3 and L3' recomputation, 44.1 kHz). `G_p`, per unit of the
  trim word: `delta A` 7.27e3 (the HPF's largest output `A` through the LPF's feedthrough), the
  fast part 6.07e3, the potential term 3.02e4 (`1/2 |m| Gammabar(m0) sup Psi Phi`,
  `V = sup Psi Phi = 1.60e5`) and the slow charges 3.82e4 (`C = 5.25` weighted by
  `sum Gamma_s`), at `m0 = 16`, against the measured 3.58e3. `G_t`, with the trim: the window
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
1. **Sound on the real kernel's worst histories** (#1485 gate 1, its six histories, widened with
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

(none yet)
