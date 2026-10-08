# Tighten the fixed input section's peak gain past the cascade triangle inequality

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order (root ruling m6 on *Define how node tails compose through gain in the
graph extent*, #1379 Amendment 1): the tightening successor of slice A2 (#1465). Code anchors
verified on `main` at `7e8379523`; the code this slice changes is created by A2, so verify every
anchor on the branch where A2 has landed before starting. Amended the same day (root ruling,
#1379 Amendment 1, third round): the gate factor is set from a first measurement, not frozen in
advance (P-D3).

## Product outcome

A fixed input section (no live input lane) states a peak gain `G_p` (and so `G_t = G_p`) within a
stated factor of the real kernel's measured peak gain, and never below it. Today's certified value
(slice A2) bounds the HPF→LPF cascade section by section with the triangle inequality, which is
sound but about 14 dB above the cascade's real `l1` norm at the top of the cutoff domain. Every
decibel of `G_p` feeds #1379's composition: it raises `up(v)` of a filtered submix section fed by a
filtered track (up to about one more decade, about 45k samples at the top pair) and the input-peak
bound `X*` of the rest branch. No kernel, law, render path or rendered bit changes.

## Context

- **Root ruling m6 (2026-10-06).** The decision-15 root coordinator, under the owner's
  no-shortcuts delegation (`no-shortcuts-correctness-first`), ruled the triangle inequality sound:
  its looseness is a tightening, not a correctness gap, so it is a successor and not a defect of
  A2. A2 must not tighten it, because the same majorant also feeds #1329's `T_decay`.
- **What A2 ships.** `G_p = G_t = ceil_mB(|trim| (1 + u) (O + dev_loud))`, where
  `O = SLACK accumulation(n) sum_t o(t)` is `math::tail::fixed_cascade`'s output majorant summed
  over all frames (`crates/math/src/tail.rs`), and `dev_loud` is the loud-input deviation fixed
  point of `Deviation::at_end` (`tail.rs:1947-1961`), per unit of `|trim| P`. `ceil_mB(g) =
  ceil(2000 log10 g)`.
- **How loose it is** (#1379 Amendment 1, H9, `f64` probes): top pair (HPF one `f32` below the
  maximum into the LPF at the maximum) at 44.1 kHz: `O` = 3.2719, `dev_loud` = 0.2310, against an
  exact cascade `l1` of 0.6967 (0.6959 at 48 kHz). The cascade's `l1` is 11.6 dB below the product
  of its sections' `l1` norms (1.09 x 2.43 at 48 kHz), and `O` exceeds that product by up to
  +5.27 dB (LPF at 10 Hz). At low cutoffs `dev_loud` is at most +0.005 dB of `O`.
- **The contract** the value must keep is #1379 Amendment 1 H1's (N1): for every input with
  `|x[n]| <= X` for all `n`, under any admitted history, `|y[n]| <= g_p X + sigma_p` (the peak stall), with
  `g_p = 10^(G_p/2000)`, for every reset pattern of the joint flush, every channel, and the `f32`
  rounding of the trim product included.
- **Gates already in place** (A2, `crates/builtins/tests/tail_contract.rs`): F2(a) (the brute-force
  `|trim| ||h||_1` at most `g_t`), F2(b) (`G_p` at most the product of the sections' brute-force
  norms plus 6.02 dB), F2(d) (`G_p` equals the A2 formula from the accessors).

## Decisions frozen for this slice

- **P-D1. Method.** The implementer's, proven in the derivation note. Candidates: a joint
  four-state majorant of the cascade instead of the section-by-section bound; or a certified `l1`
  of the designed cascade's `f64` reference response (a brute-force sum to a horizon plus a
  certified remainder from #1329's contraction) plus a deviation term tightened the same way. It
  holds for every reset pattern (a reset only drops non-negative terms) and is computed on the
  control thread in `f64` with `math::log`/`math::exp`.
- **P-D2. Never below the measured value.** A certified `G_p` below the real kernel's measured peak
  gain is unsound and stops the slice.
- **P-D3. The factor is set from a measurement, at most 1.5 (root ruling, 2026-10-06).**
  - *Step 1, before the gate is frozen.* On gate 2's rows, measure the real kernel's peak gain
    `g_meas` and compute the certified `g_p` of the method P-D1 chooses. Record both and the ratio
    `r = g_p / g_meas` per row and rate in the attempt record.
  - *Step 2, the factor.* `F = min(1.5, r_max (1 + m))`, with `r_max` the largest measured ratio and
    `m` a stated margin (default 5 %, so that a sound bound is not failed on calibration while the
    gate still binds; the attempt record states `m` and its reason), rounded up to two decimals.
    Root confirms `F` in the attempt record before gate 2 is committed, as #1433 set 1.15 from its
    estimate.
  - *Ceiling.* If `r_max` exceeds 1.5 on a gated row, P-D5 applies: stop and report. Today's value
    is about 5 (+14 dB) at the top pair; a sound method that keeps today's deviation term lands
    near 1.33 there by estimate (`dev_loud` 0.231 against an exact `l1` of 0.697) and near 1.0 at
    low cutoffs.
- **P-D4. Only `G_p` and `G_t` move.** `T_decay`, `T_rest`, both rests, `P*`, `D`, `sigma_p` and `sigma_t`
  stay exactly as A2 and #1329 state them (the tightened gain is a separate computation from the
  majorant that feeds `T`).
- **P-D5. Stop rule.** If the derivation cannot prove a tighter bound, or the measured ratio
  exceeds 1.5 on a gated row (P-D3), stop and report the measured ratios to root. A2's value
  stays in production; it is sound. There is no fallback value and no gate change.
- **P-D6. Cost** within #1457's D1 budget.

## DSP evidence (AGENTS.md)

- **Equations:** the TPT SVF cascade `s[n+1] = A s[n] + b v[n]`, `y[n] = c s[n] + d v[n]` per
  section, the cascade input `v_L = y_H`; (N1); P-D1.
- **Coefficient and update rules:** the designed `f32` words of #1329 D4; no change.
- **Numerical limits:** `f64` with #1329's `SLACK` and `accumulation(n)` factors and its `f32`
  rounding model; every millibel rounding upward.
- **Latency and tail:** latency 0; no tail value moves (P-D4).
- **Units and smoothing:** gains in millibels; no smoothing.
- **Denormal/NaN:** unchanged.
- **Citations:** as #1329 ([SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP], [SMITH-SASP]); Higham,
  *Accuracy and Stability of Numerical Algorithms*, 2nd ed.
- **Fixtures and objective tests:** gates 1-4. **Benchmarks:** #1457's gates 2 and 8 (gate 4).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. P-D3's step 1 record (`g_meas`, the certified `g_p`, the ratio per row and rate) and the chosen
   `F` with its margin, confirmed by root before gate 2 is committed.
2. The tightened fixed `G_p` in `crates/math/src/tail.rs`, rounded in `crates/builtins/src/tail.rs`.
3. The derivation in `docs/derivations/1379-graph-tail-composition.md` (the fixed `G_p` section).
4. Gates 1-4 in `crates/builtins/tests/tail_contract.rs`; A2's F2(d) equality restated to the new
   formula.
5. If #1379 has landed: each moved graph digest re-pinned one at a time with its reason.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions, stream
  A's)
- `docs/derivations/1379-graph-tail-composition.md`
- Only if #1379 has landed: `crates/graph-compiler/tests/track_delay.rs` (the digest),
  `crates/graph-compiler/tests/tail_composition.rs` (an asserted value that moves, with its reason),
  `fixtures/graph/v1/direct-route.*` (regenerated by `graph_fixture`), `fixtures/graph/MANIFEST.tsv`,
  `crates/host-core/tests/live_lanes.rs` and `crates/host-core/tests/tail_composition.rs` (an
  asserted tail that moves, with its reason) (named exceptions, stream A's)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

## Non-goals

- The live section's gains (slices B1 and B2). `T_decay`, the rests and `P*` (P-D4).
- Tightening `dev_loud` for its own sake: it moves only as far as the method needs.
- Any kernel, flush or law change.

## Hazards

- **Every reset pattern.** A bound that holds only for an uninterrupted response is unsound: the
  joint flush can reset a section mid-response. Gate 1 scans the domain for that reason.
- **Canonical digests.** If #1379 has landed, the graph's canonical text prints each node's gains,
  so a tighter value moves digests and some reported tails (shorter only); any other moved byte
  stops the slice.

## Objective gates

`crates/builtins/tests/tail_contract.rs`; every launch rate in release, 48 kHz in debug.

1. **Sound over the domain.** On a 20 x 20 log grid of legal pairs (HPF below LPF, each over its
   whole cutoff domain) plus gate 1(a)'s enabled pairs, the top pair and the HPF at 10 Hz into the
   LPF one `f32` above 10 Hz, at trims {0, +24 dB, a non-power-of-two trim}: the brute-force
   `|trim| ||h||_1` of the designed cascade (#1329 gate 1(a)'s independent `f64` brute force) is at
   most `g_t`, and `g_t <= g_p`.
2. **Within the factor of measured, never below it** (release, every launch rate). On gate 1(a)'s
   enabled pairs, the top pair and the HPF at 10 Hz into the LPF one `f32` above 10 Hz, trims
   {0, +24 dB}: drive the real kernel (`InputBuiltins`, fixed design) with the worst-sign input for
   the designed cascade's `f64` impulse response (`x[N - 1 - i] = sign(h[i])` over 4,000,000
   samples, peak `P = 1` before the trim), and take the measured peak gain `g_meas = max |y| / P`.
   Assert `g_meas <= g_p <= F g_meas`, with `F` from P-D3 (at most 1.5). Record `g_meas`, `g_p`
   and the ratio per row.
3. **Nothing else moves.** Every other `tail_contract` assertion passes unchanged (`T_decay`,
   `T_rest`, both rests, `P*`, `D`, `sigma_p`, `sigma_t`, the live figures); A2's F2(a)-(c) pass; F2(d)'s
   equality holds for the new formula; no rendered bit moves (`audit capi`'s `pcm_digest`, the wasm
   G5 digests, the builtins PCM fixtures).
4. **Budget.** #1457's gates 2 and 8 pass, within #1457's D1 budget. A change to the walk's cost
   also reruns `target/release/examples/input_bound_budget calibrate` and restates the
   frame-equivalent (the slowest frame class) and `builtins::INPUT_BOUND_SECTION_CHARGE` from it
   (#1457 Amendments 3 and 4; Amendment 4 removed the per-design charge and set the section charge
   to 290 frame-equivalents of 23.5 ns; #1474 restated them as 380 frame-equivalents of 18.5 ns;
   #1465, under root's second frame-equivalent statistic of 2026-10-08 (per-design medians) and
   its ruling (c) of the same day (per-batch medians), as 550 frame-equivalents of 17.0 ns).
5. **Commands:** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
   --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked --release -p
   builtins --features builtins/test-support --test tail_contract`; #1457's gate 2 and gate 8
   commands (#1457 Amendment 3: gate 2 is `cargo build --locked --release -p builtins --features
   test-support --example input_bound_budget` and one invocation of `taskset -c <core>
   target/release/examples/input_bound_budget`, descriptive; gate 8 is `cargo test --locked -p
   builtins-compiler --features test-support --lib
   every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`); `bash
   scripts/check-builtins-fixtures.sh . target/release/audit`; `bash
   scripts/check-workspace-policy.sh`; `cargo clippy --locked --workspace --all-targets -- -D
   warnings`; `cargo fmt --all -- --check`; if #1379 has landed, also the `test-debug-a` workspace
   command, `bash scripts/check-graph-determinism.sh`, `cargo run --locked -p graph-compiler --bin
   graph_fixture -- --check` and `cargo test --locked --release -p host-core --test tail_composition
   -- --include-ignored`.

## Test value

- Gate 1: a tightening that holds at the gated rows but not over the domain (for example one that
  drops a reset pattern, or bounds the cascade by the HPF's `l1` alone) falls below a scanned row's
  brute-force `l1`.
- Gate 2: an unsound tightening (the trim's rounding dropped, the deviation term dropped) falls
  below the real kernel's measured peak gain; a step that does not tighten (A2's section-by-section
  bound left in place: about 5x, +14 dB, at the top pair) misses the factor `F` (at most 1.5). No test compares the
  certified gain with the measured one from above.
- Gate 3: a change that moves `T_decay` with the gain (the shared majorant tightened in place) is
  red on #1329's pins.

## Dependencies

- *State a fixed input section's decay, gains and flush stall* (slice A2, #1465): the value this
  slice tightens.
- *Derive the live input section's tail gain and state its composition* (slice B2, #1467): the
  shared files `crates/math/src/tail.rs` and `crates/builtins/src/tail.rs`, landed first.
- *Cache design bounds across preparations within a stated preparation budget* (#1457).
- It may land before or after #1379; if after, it re-pins as Deliverable 5 states.

## Attempt record
