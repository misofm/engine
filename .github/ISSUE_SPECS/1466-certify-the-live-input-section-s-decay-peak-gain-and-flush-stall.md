# Certify the live input section's decay, peak gain and flush stall

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order: slice B1 of *Define how node tails compose through gain in the graph
extent* (#1379), Amendment 1 (H1, H3's live row; root ruling 4 split the live slice in two). Code
anchors verified on `main` at `7e8379523` and on `codex/d15-stream-g2` at `f3956e63c`; re-verify
every anchor at start.

## Product outcome

For an input section with a live input lane, the engine computes three certified composition
values over every history of live trim, polarity and filter targets: the decay `D`, the peak gain
`G_p` and the flush stall `sigma`. They are exposed through test-support accessors and checked on
the real kernel. The live bound keeps `CompositionBound::Unstated` until *Derive the live input
section's tail gain and state its composition* adds the fourth value (`G_t`), because the four
values are stated together or not at all. No reported tail and no rendered bit moves.

## Context

- **The carrier** is `NodeTailBound { tail, tail_every_peak, rest, composition }` (slice A1, #1464);
  the fixed-design values and the construction of `D` come from slice A2 (#1465).
- **`math::tail`'s live cascade** (#1433, `live_cascade`, `LiveCascade`, `live_zones`) computes the
  live `T_decay`, stall, `P*` and rests from the zone state bounds (`Phi`), the LPF's state bound,
  the 65-frame window and every settled group; its relative share is `eps / 2` and the stall's
  `eps / 2` (`crates/math/src/tail.rs:1494-1496`, `:1675`). The live bound costs about 250 us and
  is cached per rate by #1457.
- **`builtins::input_section_live_bound`** (`crates/builtins/src/tail.rs:211`) returns it as
  `NodeTailBound` with `composition: Unstated` (A1).
- **`live_bound_carries_every_term_an_independent_recomputation_requires`** in
  `crates/builtins/tests/tail_contract.rs` recomputes the live bound's terms in plain `f64`.
- **The live words.** A live retarget moves the recursion only through designs and their mixtures,
  within 64 half-ulps plus `u D` per word of their convex hull, and completes by `N + 64` (#1407).
  The trim ramp stays inside its endpoints, at most +24 dB (#1408).
- **Retarget transient (`f64` probe, #1379 Amendment 1, H9).** An HPF at the 48 kHz maximum driven
  by an alternating `+-1` input builds `ic1 = -2.70e4` with `|y| <= 1.04`; a 64-frame ramp to
  10 Hz with zero input then peaks at `|y| = 2.17e3` (+67 dB before the trim). The settled designs'
  `l1` (10 Hz HPF into the top LPF: 3.51) is far below that.

## The contract (#1379 Amendment 1, H1)

`eps = 10^(-144/20)`; `u = 2^-24`; `N` the first sample of silence; no control event at or after
`N`; `g_p = 10^(G_p/2000)`.

- **(N1) Peak.** For every input with `|x[n]| <= X` for all `n`, under any admitted history:
  `|y[n]| <= g_p X + sigma` for every `n`.
- **(N2) Tail at every decade.** If `|x[n]| <= X` for all `n` and `|x[n]| <= epsilon` for every
  `n >= M` (some `M >= N`), then for every integer `k >= 0` and every `n >= M + L + T + k D`:
  `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`. (The live bound's own split is
  `eps / 2` relative and `eps / 2` stall; (N2) at `3/4` stays valid for it.)
- **`D` by the binding construction** of slice A2 (F-D2): the certified crossings `T(k)` for
  `k = 1..16` from one extended pass, a contraction certificate (`M v <= lambda v`, `lambda < 1`)
  for `k > 16`, and `D = max(max_{k <= 16} ceil((T(k) - T) / k), D_inf + ceil((T_lambda - T) / 17))`.

## Decisions frozen for this slice

- **L-D1. `D`** by the construction above, the maximum over the settled groups; the accessors as
  A2 (`T(k)`, `D_inf`, `lambda`, `T_lambda`).
- **L-D2. `G_p`:** the supremum over every admitted history, through the output row and the
  feedthrough, over the 65-frame window and every settled group, with the trim word of +24 dB,
  plus the live loud-input deviation; rounded up to millibels with `math::log`.
- **L-D3. `sigma`:** the live stall rounded up to millibels.
- **L-D4. No partial statement.** `input_section_live_bound` keeps `CompositionBound::Unstated`;
  the three values are reachable only through the accessors until the tail-gain slice states all
  four.
- **L-D5. Cost** within #1457's D1 budget (the live bound depends only on the rate and is cached per
  rate).

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade under #1407's live words (#1433's zone analysis); L-D1-L-D3.
- **Coefficient and update rules:** #1407's retarget through designs and their mixtures; #1408's
  trim ramp.
- **Numerical limits:** `f64` with #1433's certified rounding allowances; every millibel rounding
  upward.
- **Latency and tail:** latency 0; `T`, `T_rest` and both rests unchanged.
- **Units and smoothing:** samples at the plan's rate; gains in millibels.
- **Denormal/NaN:** unchanged (sanitized input; per-block reset of a non-finite state).
- **Citations:** as #1329 and #1433 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP],
  [SMITH-SASP]; Higham for the rounding model).
- **Fixtures and objective tests:** L1, L3-L6. **Benchmarks:** #1457's gates (L6).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. `math::tail`'s live cascade: `D`, `G_p` and `sigma`, with test-support accessors.
2. The derivation note's live part for `D`, `G_p` and `sigma`
   (`docs/derivations/1379-graph-tail-composition.md`).
3. Gates L1, L3-L6, and the mutation table.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs` (the accessors only), `crates/builtins/tests/tail_contract.rs`
  (named exceptions, stream A's)
- `docs/derivations/1379-graph-tail-composition.md` (the live part)
- The browser's memory fixture (the exact `memoryBytes` pin #1433 names), only if the live bound's
  peak allocation moves, with its reason
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Non-goals

- The live tail gain `G_t` and the switch to `Stated` (*Derive the live input section's tail gain
  and state its composition*).
- Fixed designs (A2). Any graph or report change (#1379).

## Hazards

- **#1433's values must not move**: live `T_decay`, `T_rest`, both rests and `P*` (L5).
- **`G_p` is not the settled designs' `l1`.** The retarget transient after a Nyquist drive exceeds
  it by tens of dB; L1 exists for that.
- **Cost.** The extended horizon over every settled group lengthens the live computation; a miss of
  #1457's budget stops the slice.

## Objective gates

`crates/builtins/tests/tail_contract.rs`, release, every launch rate.

- **L1. Live peak gain on the real kernel.** A real `InputBuiltins` with a live input lane, trim
  +24 dB, both sections designed at the worst-case pair (`input_section_worst_case_pair(rate)`),
  driven by an alternating `+-1` input for 1,000,000 frames, then the HPF target moved to 10 Hz with
  the input still running: the largest `|y|` over the run is at most `g_p + sigma` (accessors). The
  measured ratio is recorded.
- **L3. Independent recomputation.** The existing
  `live_bound_carries_every_term_an_independent_recomputation_requires` gains `D`, `G_p` and
  `sigma`: each lies between a plain-`f64` recomputation of the derivation in the test and that
  value plus 0.01 % and 64 frames (`D`) or 1 mB (gain, stall).
- **L4. Live decade identity.** The module's directly searched live crossing `T(k)` is at most
  `T + k D` for `k = 0..64`, and `D_inf` is at least the live floor recomputed as in A2's F1(d).
- **L5. Nothing certified moves.** Every existing `tail_contract` assertion passes unchanged; the
  live bound still states `Unstated`; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm
  G5 digests, the builtins PCM fixtures).
- **L6. Budget.** #1457's gates 2 and 8 pass after this change, within #1457's D1 budget.
- **Commands:** as slice A2 (`cargo test --locked --all-targets -p lane -p math -p builtins -p
  dsp-reference --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked
  --release -p builtins --features builtins/test-support --test tail_contract`; #1457's gate 2 and
  gate 8 commands (#1457 Amendment 3: gate 2 is `cargo build --locked --release -p builtins
  --features test-support --example input_bound_budget` and one invocation of `taskset -c <core>
  target/release/examples/input_bound_budget`, descriptive; gate 8 is `cargo test --locked -p
  builtins-compiler --features test-support --lib
  every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`); `audit capi`;
  `bash scripts/check-builtins-fixtures.sh . target/release/audit`; `bash
  scripts/run-wasm-gates.sh`; `bash scripts/check-workspace-policy.sh`; clippy with `-D warnings`;
  `cargo fmt --all -- --check`).

The attempt record carries a mutation table with at least the mutants below; each is red.

## Test value

- L1: a live `G_p` taken from the settled designs' `l1` (about +35 dB with the trim: the 10 Hz HPF
  into the top LPF has `l1` 3.51) is beaten by the retarget transient by tens of dB (one HPF alone:
  +67 dB before the trim); no other test drives a retarget after a Nyquist drive.
- L3: a live value that drops a ramp-window or zone term falls below the recomputation; the real
  kernel's slack (about 8.5 % in `R`) cannot see it.
- L4: a live `D` with no transient term falls below the module's own crossing at small `k`; a
  certificate rate that omits the state rounding falls below the floor (as A2's F1(c) and F1(d)).
- L6: reuses #1457's gates; a live construction that runs one pass per `k` is red there.

## Dependencies

- *State a fixed input section's decay, gains and flush stall* (slice A2, #1465): the construction
  of `D`, the accessors' shape and the shared files.
- *Carry every node's tail bound in one node-neutral struct* (slice A1, #1464).
- *Tighten the cascade exact-rest bound with a frequency-aware cascade analysis* (#1433, passed),
  *Retarget a live input filter only through its designs and their mixtures* (#1407), *Keep every
  trim, fader and matrix ramp inside its endpoints* (#1408).
- *Cache design bounds across preparations within a stated preparation budget* (#1457): the gated
  budget.
- Named exceptions: `STREAMS.md` (#1379 Amendment 1, H8).

## Attempt record
