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
   its ruling (c) of the same day (per-batch medians), as 550 frame-equivalents of 17.0 ns; and,
   with root's measurement margin of the same day (the computed bound x 1.05, rounded up to
   0.5 ns), as 520 frame-equivalents of 18.0 ns).
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

### Attempt 1 (2026-10-08, implementer): stopped under P-D5

**Result.** P-D3 step 1 measured `r_max = 5.0056` (gate 2's top pair, 48 kHz, +24 dB), above the
1.5 ceiling. P-D5 applies: the slice stops, A2's value stays in production, and no gate changes.
The code on `codex/d15-stream-g3` is unchanged. The attempt (derivation, code, gates 1 and 2 with
the soundness half of gate 2 only) is kept on the local branch `codex/d15-1468-attempt1`, commit
`49c86926b`, for root's ruling.

**Anchors verified** on `56fe81fcb`: `math::tail::fixed_cascade_walk` (the pass, `O = input_sup[K]`,
`Deviation::at_end`'s `loud_output`), `CascadeComposition::{output_majorant, dev_loud, peak_gain}`,
`builtins::tail::stated_composition` (`ceil_millibels(peak_gain)`), F2(a)-(d) in
`tail_contract.rs`. #1379 is not on the branch, so Deliverable 5 does not apply.

**The resets the proof must cover** (checked in code): the joint flush zeroes one section after
the step of a frame `r` that is armed only when the channel's raw input was exactly zero on frames
`r - N_SILENCE + 1 ..= r` (`lane::silence_step`, `lane::kernels::builtins::input_chain_block`);
the two sections flush independently. The block recovery (`|y| >= NONFINITE_LIMIT` anywhere in a
block) zeroes the block's output and both sections together.

**Method of the attempt (P-D1).** `L = sum_{t < W} |h(t)| + sum_{t >= W} o(t)`, `W = N_SILENCE + 1`:
no joint flush can cut a lag below `W` (Lemma 1 in the derivation on the side branch), the
recovery only removes whole terms, and `o` covers every cut from `W` on. `|h(t)|` over the window
is the cascade in signed `f64` with certified radii, in the walk's pass (no frame added). Sound:
gate 1 (release, every rate, the 20 x 20 grid plus the gated rows, trims 0, +24 and +7.3 dB) gives a
smallest `g_t / (|trim| ||h||_1)` of 1.00003, 1.00002, 1.00006 and 1.00001 (44.1, 48, 88.2,
96 kHz); gate 2's soundness half holds on all 120 rows.

**P-D3 step 1** (gate 2, release, one run; `r = g_p / g_meas`; "A2 r" is A2's
`ceil_mB(|trim| (1 + u) (O + dev_loud))` against the same `g_meas`; `L` and `dev_loud` per unit of
`|trim|`; the +24 dB `g_meas` is the 0 dB value times the trim to within the `f32` output):

| rate | HPF | LPF | g_meas (0 dB) | g_p (0 dB) | r (0 dB) | r (+24 dB) | L | dev_loud | A2 r (0 dB) |
|---|---|---|---|---|---|---|---|---|---|
| 44100 | 0 | 10 | 1.090331 | 1.091440 (76 mB) | 1.0010 | 1.0010 | 1.090331 | 0.000288 | 1.0010 |
| 44100 | 0 | 1000 | 1.090982 | 1.091440 (76 mB) | 1.0004 | 1.0004 | 1.090982 | 0.000005 | 1.0004 |
| 44100 | 0 | 22049.48 | 2.435547 | 2.488857 (792 mB) | 1.0219 | 1.0221 | 2.434529 | 0.052138 | 1.0219 |
| 44100 | 0 | 22049.482 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0221 | 2.434317 | 0.052341 | 1.0227 |
| 44100 | 10 | 0 | 2.432523 | 2.435006 (773 mB) | 1.0010 | 1.0010 | 2.432523 | 0.000289 | 1.0010 |
| 44100 | 10 | 1000 | 2.442938 | 2.466039 (784 mB) | 1.0095 | 1.0095 | 2.464254 | 0.000565 | 1.9015 |
| 44100 | 10 | 22049.48 | 3.528320 | 6.706563 (1653 mB) | 1.9008 | 1.9007 | 6.571196 | 0.127710 | 2.1059 |
| 44100 | 10 | 22049.482 | 3.528320 | 6.706563 (1653 mB) | 1.9008 | 1.9007 | 6.574339 | 0.128203 | 2.1059 |
| 44100 | 1000 | 0 | 2.244230 | 2.246467 (703 mB) | 1.0010 | 1.0010 | 2.244230 | 0.000006 | 1.0010 |
| 44100 | 1000 | 22049.48 | 3.671875 | 6.397348 (1612 mB) | 1.7423 | 1.7434 | 6.278874 | 0.117029 | 1.8669 |
| 44100 | 1000 | 22049.482 | 3.669922 | 6.404718 (1613 mB) | 1.7452 | 1.7454 | 6.281251 | 0.117484 | 1.8679 |
| 44100 | 22049.48 | 0 | 1.091774 | 1.148154 (120 mB) | 1.0516 | 1.0518 | 1.090402 | 0.056968 | 1.0516 |
| 44100 | 22049.48 | 22049.482 | 0.698486 | 3.495425 (1087 mB) | 5.0043 | 5.0043 | 3.263191 | 0.230996 | 5.0158 |
| 44100 | 22049.482 | 0 | 1.091574 | 1.148154 (120 mB) | 1.0518 | 1.0520 | 1.090199 | 0.057162 | 1.0518 |
| 44100 | 10 | 10.000001 | 0.694134 | 0.890225 (-101 mB) | 1.2825 | 1.2825 | 0.888909 | 0.001280 | 7.0074 |
| 48000 | 0 | 10 | 1.090333 | 1.091440 (76 mB) | 1.0010 | 1.0010 | 1.090331 | 0.000314 | 1.0010 |
| 48000 | 0 | 1000 | 1.090875 | 1.091440 (76 mB) | 1.0005 | 1.0005 | 1.090875 | 0.000006 | 1.0005 |
| 48000 | 0 | 23999.432 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0225 | 2.434162 | 0.051890 | 1.0227 |
| 48000 | 0 | 23999.434 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0229 | 2.434138 | 0.052075 | 1.0227 |
| 48000 | 10 | 0 | 2.432687 | 2.435006 (773 mB) | 1.0010 | 1.0010 | 2.432687 | 0.000314 | 1.0010 |
| 48000 | 10 | 1000 | 2.442832 | 2.466039 (784 mB) | 1.0095 | 1.0095 | 2.464332 | 0.000616 | 1.9081 |
| 48000 | 10 | 23999.432 | 3.507812 | 6.645077 (1645 mB) | 1.8944 | 1.8955 | 6.517611 | 0.127192 | 2.1182 |
| 48000 | 10 | 23999.434 | 3.507812 | 6.652732 (1646 mB) | 1.8965 | 1.8966 | 6.520076 | 0.127643 | 2.1182 |
| 48000 | 1000 | 0 | 2.258113 | 2.259436 (708 mB) | 1.0006 | 1.0006 | 2.258113 | 0.000006 | 1.0006 |
| 48000 | 1000 | 23999.432 | 3.682617 | 6.397348 (1612 mB) | 1.7372 | 1.7364 | 6.279003 | 0.117193 | 1.8743 |
| 48000 | 1000 | 23999.434 | 3.683594 | 6.404718 (1613 mB) | 1.7387 | 1.7384 | 6.280782 | 0.117612 | 1.8738 |
| 48000 | 23999.432 | 0 | 1.091425 | 1.146833 (119 mB) | 1.0508 | 1.0509 | 1.090051 | 0.056648 | 1.0508 |
| 48000 | 23999.432 | 23999.434 | 0.697998 | 3.491403 (1086 mB) | 5.0020 | 5.0056 | 3.260399 | 0.229743 | 5.0193 |
| 48000 | 23999.434 | 0 | 1.091400 | 1.148154 (120 mB) | 1.0520 | 1.0521 | 1.090028 | 0.056848 | 1.0520 |
| 48000 | 10 | 10.000001 | 0.694134 | 0.891251 (-100 mB) | 1.2840 | 1.2840 | 0.889091 | 0.001392 | 7.0155 |
| 88200 | 0 | 10 | 1.090331 | 1.091440 (76 mB) | 1.0010 | 1.0010 | 1.090331 | 0.000574 | 1.0010 |
| 88200 | 0 | 1000 | 1.090519 | 1.091440 (76 mB) | 1.0008 | 1.0008 | 1.090519 | 0.000008 | 1.0008 |
| 88200 | 0 | 44098.96 | 2.435547 | 2.488857 (792 mB) | 1.0219 | 1.0221 | 2.434529 | 0.052138 | 1.0219 |
| 88200 | 0 | 44098.965 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0221 | 2.434317 | 0.052341 | 1.0227 |
| 88200 | 10 | 0 | 2.433529 | 2.435006 (773 mB) | 1.0006 | 1.0006 | 2.433530 | 0.000575 | 1.0006 |
| 88200 | 10 | 1000 | 2.442410 | 2.466039 (784 mB) | 1.0097 | 1.0097 | 2.464751 | 0.001143 | 1.9462 |
| 88200 | 10 | 44098.96 | 3.372070 | 6.194411 (1584 mB) | 1.8370 | 1.8367 | 6.058665 | 0.128635 | 2.2035 |
| 88200 | 10 | 44098.965 | 3.373047 | 6.194411 (1584 mB) | 1.8364 | 1.8372 | 6.062720 | 0.129128 | 2.2054 |
| 88200 | 1000 | 0 | 2.336531 | 2.338837 (738 mB) | 1.0010 | 1.0010 | 2.336531 | 0.000009 | 1.0010 |
| 88200 | 1000 | 44098.96 | 3.755859 | 6.237348 (1590 mB) | 1.6607 | 1.6610 | 6.111234 | 0.121850 | 1.9002 |
| 88200 | 1000 | 44098.965 | 3.753906 | 6.237348 (1590 mB) | 1.6616 | 1.6610 | 6.114756 | 0.122324 | 1.9012 |
| 88200 | 44098.96 | 0 | 1.091774 | 1.148154 (120 mB) | 1.0516 | 1.0518 | 1.090402 | 0.056968 | 1.0516 |
| 88200 | 44098.96 | 44098.965 | 0.698486 | 3.443499 (1074 mB) | 4.9299 | 4.9299 | 3.211992 | 0.230996 | 5.0158 |
| 88200 | 44098.965 | 0 | 1.091574 | 1.148154 (120 mB) | 1.0518 | 1.0520 | 1.090199 | 0.057162 | 1.0518 |
| 88200 | 10 | 10.000001 | 0.694134 | 0.892278 (-99 mB) | 1.2855 | 1.2855 | 0.889120 | 0.002547 | 7.0155 |
| 96000 | 0 | 10 | 1.090333 | 1.091440 (76 mB) | 1.0010 | 1.0010 | 1.090331 | 0.000625 | 1.0010 |
| 96000 | 0 | 1000 | 1.090472 | 1.091440 (76 mB) | 1.0009 | 1.0009 | 1.090471 | 0.000009 | 1.0009 |
| 96000 | 0 | 47998.863 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0225 | 2.434162 | 0.051890 | 1.0227 |
| 96000 | 0 | 47998.867 | 2.433594 | 2.488857 (792 mB) | 1.0227 | 1.0229 | 2.434138 | 0.052075 | 1.0227 |
| 96000 | 10 | 0 | 2.433614 | 2.435006 (773 mB) | 1.0006 | 1.0006 | 2.433612 | 0.000625 | 1.0006 |
| 96000 | 10 | 1000 | 2.442375 | 2.468880 (785 mB) | 1.0109 | 1.0109 | 2.464810 | 0.001245 | 1.9507 |
| 96000 | 10 | 47998.863 | 3.350586 | 6.109420 (1572 mB) | 1.8234 | 1.8232 | 5.977122 | 0.128190 | 2.2201 |
| 96000 | 10 | 47998.867 | 3.351562 | 6.109420 (1572 mB) | 1.8229 | 1.8232 | 5.980649 | 0.128641 | 2.2195 |
| 96000 | 1000 | 0 | 2.344209 | 2.344229 (740 mB) | 1.0000 | 1.0000 | 2.344209 | 0.000010 | 1.0000 |
| 96000 | 1000 | 47998.863 | 3.761719 | 6.187283 (1583 mB) | 1.6448 | 1.6446 | 6.060150 | 0.121670 | 1.9038 |
| 96000 | 1000 | 47998.867 | 3.761719 | 6.187283 (1583 mB) | 1.6448 | 1.6446 | 6.063164 | 0.122105 | 1.9038 |
| 96000 | 47998.863 | 0 | 1.091425 | 1.146833 (119 mB) | 1.0508 | 1.0509 | 1.090051 | 0.056648 | 1.0508 |
| 96000 | 47998.863 | 47998.867 | 0.697998 | 3.427678 (1070 mB) | 4.9107 | 4.9142 | 3.195257 | 0.229743 | 5.0193 |
| 96000 | 47998.867 | 0 | 1.091400 | 1.148154 (120 mB) | 1.0520 | 1.0521 | 1.090028 | 0.056848 | 1.0520 |
| 96000 | 10 | 10.000001 | 0.694136 | 0.892278 (-99 mB) | 1.2855 | 1.2855 | 0.889191 | 0.002771 | 7.0154 |

`r_max = 5.0056` (+24 dB; 5.0043 at 0 dB), at the top pair of every rate (4.91 to 5.01). The HPF
at 10 Hz or 1 kHz into the LPF at or one `f32` below the maximum gives 1.64 to 1.90, and the HPF
at 10 Hz into the LPF one `f32` above it 1.28 to 1.29. Every other row is 1.00 to 1.05. By P-D3,
`F = min(1.5, 5.0056 x 1.05) = 1.50` with the default `m = 5 %`, and the top pair and the four
LPF-at-the-maximum rows per rate exceed it.

**Why the window method stops there.** The sections near the maximum cutoff ring far past `W`.
Brute force (`f64`, 4,000,000 frames): the HPF one `f32` below the maximum has
`sum_{t >= W} |h_HPF(t)| = 1.056, 1.050, 0.972, 0.952` (44.1, 48, 88.2, 96 kHz) of its
`||h_HPF||_1 = 1.090`; the cascade's tail past `W` is 0.667, 0.661, 0.608 and 0.596 of its 0.697.
`o` then bounds that tail by the product of the sections.

**A tighter bound (derived, not implemented).** For two sections, a cut of either section at a lag
of `W` or more gives `|c(n, i) - s h(t)| <= (|h_2| * T_1)(t)` with `s` in `{0, 1}` and
`T_1(l) = |h_1(l)| [l >= W]`, so `sum_i |c(n, i)| <= ||h||_1 + ||h_2||_1 sum_{t >= W} |h_1(t)|`
(and the induction `E_(k+1) = |h_(k+1)| * (T_k + E_k)` for more sections). Brute-force values of
it, with the attempt's `dev_loud`: the HPF at 10 Hz or 1 kHz into the LPF at the maximum,
`r = 1.03` to 1.06; the HPF at 10 Hz into the LPF one `f32` above it, `r = 1.04`. At the top pair
it gives 3.27 (44.1 kHz), `r = 5.0`: it does not help there, because the HPF's own tail past `W`
is almost all of its norm.

**No sound bound meets gate 2 at the top pair.** In the reset model (every reset pattern, the
spec's hazard and A2's model), take inputs that end `N_SILENCE` frames before an armed frame `r`,
with the signs that maximise the HPF's output at `r + 1`, and a joint flush of the LPF alone at
`r`. Then `y[r + 1] = d_LPF y_HPF[r + 1]` and the gain is `|d_LPF| sum_{t >= W} |h_HPF(t)|` =
1.0564, 1.0497, 0.9720 and 0.9517 (brute force). Against gate 2's `g_meas` (0.698486, 0.697998,
0.698486, 0.697998), every sound `g_p` has `r >= 1.512, 1.504, 1.392, 1.363` before any deviation
term, and `r >= 1.84` at 44.1 kHz with A2's `dev_loud` (0.231). So gate 2 at `F <= 1.5`, measured
with an uninterrupted input, cannot pass at the 44.1 kHz and 48 kHz top pair for any sound value.
Not shown: that the real kernel reaches this flush pattern (the LPF's words both below `REST_EPS`
while the HPF's are not); the proof cannot exclude it without a new argument.

**Spec problem.** P-D3's estimate of about 1.33 at the top pair uses the uninterrupted `l1` (0.697)
as the reference's bound, which the spec's own hazard ("every reset pattern") rules out.

**For root.** (a) Keep A2 (P-D5's default). (b) Rule a different gate 2 for the near-Nyquist rows
(for example a reset-aware measurement on the real kernel, or a factor per row) and implement the
tighter bound above, which brings every other gated row to about 1.06 or lower. (c) A new argument
that excludes the LPF-only flush while the HPF rings (a property of the flush law), which would
admit the uninterrupted `l1` past `W`.

**Commands run** (side branch, release unless stated): gate 2 and gate 1 (above); `cargo test
--locked --release -p builtins --features builtins/test-support --test tail_contract --
--include-ignored`, 24 passed (F1-F3, F2(d) restated to `L`, every #1329, #1433, #1466 and #1467
assertion); `cargo clippy --locked -p math -p builtins --all-targets --features
math/lane,builtins/test-support -- -D warnings` clean; `cargo fmt --all -- --check` clean. Not run:
the debug command, gate 4 (the window's signed steps raise the pass's cost per frame), the PCM
checks, and the mutation runs (the slice stopped before its gates were final).
