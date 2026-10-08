# State a fixed input section's decay, gains and flush stall

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Filed 2026-10-06 by root order: slice A2 of *Define how node tails compose through gain in the graph
extent* (#1379), Amendment 1 (H1, H3's fixed rows, H9). Code anchors verified on `main` at
`7e8379523` and on `codex/d15-stream-g2` at `f3956e63c`; re-verify every anchor at start.

## Product outcome

A fixed input section (no live input lane; #1329 D4), and one with both filters disabled, states
the composition values of `NodeTailBound` as `CompositionBound::Stated { decay, peak_gain,
tail_gain, stall }`, each certified on the designed `f32` words, so that #1379 can carry a strip's
tail through the gain after it. No reported tail moves (the graph does not read the values until
#1379), and no rendered bit moves.

## Context

- **The carrier** is `NodeTailBound` with `composition: CompositionBound` (slice A1, #1464, *Carry
  every node's tail bound in one node-neutral struct*); after A1 every builtin bound states `Unstated`.
- **`math::tail::fixed_cascade`** (`crates/math/src/tail.rs`) computes #1329's `T_decay`, the
  stall, the flush floor `P*` (`:2250-2251`, from `dev_sup_from_core`, the deviation supremum from
  `T_decay` on, `:2245-2249`) and the rests; `CascadeBound` returns them. Its output majorant `o(t)`
  is summed with a `SLACK accumulation(n)` factor for the `f64` summation.
- **`Deviation::at_end`** (`tail.rs:1947-1961`) computes the output deviation's fixed point while
  the input is loud (the last section's `difference`), per unit of `|trim| P` (`:1935`), and
  discards it. Before `N` the drives are at their suprema and the deviation is at most this fixed
  point (`:1929-1932`); after `N` the drives only fall.
- **`crates/builtins/src/tail.rs`** rounds the module's values into the bound.
- **`crates/builtins/tests/tail_contract.rs`** holds #1329's gates: gate 1(a)'s enabled pairs and
  the independent `f64` brute force of the designed cascade's impulse response.
- **Preparation refuses `hpf_hz >= lpf_hz`** with both filters enabled (`FilterOrder`,
  `crates/builtins/src/lib.rs:3452-3454`, `crates/builtins/src/filter_control.rs:43-45`), so every
  gate row below has its HPF below its LPF.

## The contract (#1379 Amendment 1, H1)

Notation: `eps = 10^(-144/20)`; `u = 2^-24`; latency `L`; `N` the first sample of silence;
`g_p = 10^(G_p/2000)`, `g_t = 10^(G_t/2000)`.

- **(N1) Peak.** If `|x[n]| <= X` for all `n`: `|y[n]| <= g_p X + sigma` for every `n`.
- **(N2) Tail at every decade.** With no control event at or after `N`: if `|x[n]| <= X` for all
  `n` and `|x[n]| <= epsilon` for every `n >= M` (some `M >= N`), then for every integer `k >= 0`
  and every `n >= M + L + T + k D`: `|y[n]| <= (3/4) eps 10^(-k) X + g_t epsilon + sigma`.
  For a fixed design `G_t = G_p`: its words never change, so an input after `N` meets the same
  operator as one before it.
- **(N3) Rest.** #1329 D2's `RestSamples`, read with "zero from `M`" (`M >= N`).
- **The share `3/4`** is #1329's split: `T_decay` certifies `eps / 2` for the exact reference and
  `eps / 4` for the relative rounding; the last quarter is the stall's. So (N2) at `k = 0`,
  `epsilon = 0` is #1329 D1 for every peak `P >= P^ := 4 sigma / eps`, and #1329's own floor `P*`
  satisfies `P* <= P^ < 2 * 10^(1/2000) (1 + 2^-29) P* < 2.0024 P*` (the millibel rounding of
  `sigma` can lift `P^` above `2 P*`: 16 of the 112 F3 rows, at most 2.0016). Nothing #1329
  certified changes.
- **`dev_core` and `dev_loud`.** `dev_core` is #1329's deviation supremum from `T_decay` on; it
  feeds `P*`. `dev_loud` is the output deviation while the input is loud (`Deviation::at_end`'s
  fixed point), per unit of `|trim| P`; it bounds the deviation at every frame. `G_p` uses
  `dev_loud`, never `dev_core`.

## Decisions frozen for this slice

- **F-D1. The contract** is H1 above: (N1), (N2) with the share `3/4`, (N3).
- **F-D2 (superseded by root's amendment of 2026-10-08, below; the original text is kept for the
  record). `D` by the binding construction.** Compute the certified crossing `T(k)` of the floor
  `(3/4) eps 10^(-k)` directly, by the machinery that computes `T` (the exact share
  `eps/2 10^(-k)`, the rounding share `eps/4 10^(-k)`, the horizon extended), for `k = 1..16`, all
  17 crossings in **one extended pass** (horizon `T(16)`, about 2.8 `T` at the top pair; one run
  per `k` costs about 16-30 times and is not allowed). Cover every `k > 16` by a contraction
  certificate on #1329's non-negative majorant and deviation propagations: a positive `v` and a
  rate `lambda < 1` with `M v <= lambda v`, so the bound from any frame `t0 >= T` falls by `lambda`
  per frame. Let `D_inf = ceil(ln 10 / -ln lambda)` and `T_lambda >= T` the first frame at or
  after `T` from which the certificate's bound is below the `k = 0` floor. Then
  `D = max(max_{k <= 16} ceil((T(k) - T) / k), D_inf + ceil((T_lambda - T) / 17))`.
  For `k > 16`: `T + k D >= T_lambda + k D_inf >= T(k)`. A slope alone is not a certificate: the
  exact suffix of the top pair falls by 36k-51k samples per decade at 44.1 kHz. `T(k)`, `D_inf`,
  `lambda` and `T_lambda` are exposed through test-support accessors.
- **F-D2, amended by root (2026-10-08, after attempt 1's stop; binding).** The walk goes only to
  `T`, the existing crossing; there is no extended pass to `T(16)`. Every `k >= 1` after `T` is
  covered by a certified closed form: either the contraction certificate (a positive `v`,
  `lambda < 1` with `M v <= lambda v` on #1329's non-negative majorant and deviation propagations)
  extended from `k > 16` down to `k >= 1`, or powers of the non-negative step; the form that gives
  the tighter sound `D` is chosen, with the reason. The closed form and its full proof (rounding and
  #1474's `tau` floors included) are written in the derivation before the code, and the verifier
  checks them as a proof. The looseness of `D` against the walked `T(k)` (computed off-line, for
  measurement only, never shipped) is measured on gate 2's families and recorded; if `D` is more
  than 2x the walked value on any family, the report says so and root files a tightening
  successor (the implementer files nothing). The budget and gate 8 are unchanged: gate 8 stays
  exact at every rate. **Applied:** the certificate, anchored at `T` (each half's state carried to
  `T` by powers of its step), covers every `k >= 1`; `D = max_h (o_h + floor(max(a_h, 0) + b_h) +
  1)` (`docs/derivations/1379-graph-tail-composition.md`, "`D`: the closed form"). The accessors
  are `CascadeComposition::certificate` (`crossing(k)` = `T(k) - T`, `asymptotic_decay()` =
  `D_inf`, `lambda()`, `floor_crossing()` = `T_lambda - T`), `output_majorant` (`O`) and
  `dev_loud`, public fields of `math::tail::CascadeBound::composition`.
- **F-D3. Gains.** Let `O = SLACK accumulation(n) sum_t o(t)` be the module's certified output
  majorant sum (it bounds the cascade's `l1` norm for every reset pattern: a reset only drops
  non-negative terms). Enabled filters: `G_p = G_t = ceil_mB(|trim| (1 + u) (O + dev_loud))`.
  `CascadeBound` returns `dev_loud`, never `dev_core`. Both filters disabled: `D = 0`,
  `G_p = G_t = ceil_mB(|trim| (1 + u))`, or `ceil_mB(|trim|)` where the `f32` product is exact (a
  power-of-two trim, 0 dB included), `sigma` one underflow (`2^-126`, rounded up into
  `FlushStall::Level`), rest `ZERO`. `ceil_mB(g) = ceil(2000 log10 g)`, computed with
  `math::log` so that every target gives the same value.
- **F-D4. Stall.** `sigma` is the module's stall (`F` times the absolute output's fixed point, which
  holds at every frame) rounded up to millibels; `P^ = 4 sigma / eps`.
- **F-D5. Channels.** Each value is the maximum over left and right.
- **F-D6. Cost.** Within #1457's D1 budget (the worst case, binding both hosts).
- **F-D7. Not tightened here.** The per-section triangle inequality inside `O` is sound but about
  14 dB loose at the top pair (`O + dev_loud` = 3.50 against an exact cascade `l1` of 0.697). The
  successor *Tighten the fixed input section's peak gain past the cascade triangle inequality* owns
  that; this slice must not tighten it, because removing it would also move #1329's `T_decay`
  (F4).

## DSP evidence (AGENTS.md)

- **Equations:** TPT SVF sections `s[n+1] = A s[n] + b v[n]`, `y[n] = c s[n] + d v[n]` (#1329);
  F-D2 and F-D3 above.
- **Coefficient and update rules:** the designed `f32` words of #1329 D4; no change.
- **Numerical limits:** `f64` evaluation with #1329's `SLACK` and `accumulation(n)` factors and its
  `f32` rounding model; every rounding to millibels is upward.
- **Latency and tail:** latency 0; `T`, `T_rest` and both rests unchanged; `D` new.
- **Units and smoothing:** samples at the plan's rate; gains in millibels; no smoothing.
- **Denormal/NaN:** the stall carries the per-word flush and the underflow allowance; non-finite
  input is sanitized upstream (unchanged).
- **Citations:** as #1329: [SIMPER-SVF], [ZAVALISHIN-TPT], [ORFANIDIS-ISP], [SMITH-SASP]; N. J.
  Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., for the rounding model.
- **Fixtures and objective tests:** gates F1-F5. **Benchmarks:** #1457's gates 2 and 8 (F5).
  **Listening:** none; no rendered bit moves.

## Deliverables

1. `math::tail`: `CascadeBound` gains `decay`, `dev_loud`, the linear gains and the stall's raw
   value; the test-support accessors (`T(k)`, `D_inf`, `lambda`, `T_lambda`, `O`, `dev_loud`).
2. `builtins`: the fixed-design and disabled-filter `Stated` values.
3. `docs/derivations/1379-graph-tail-composition.md` (new): H1's contract and the fixed-design
   values with their proofs; a pointer from the #1329 note.
4. Gates F1-F5, and the mutation table.

## Authorized paths (named exceptions are marked)

- `crates/math/src/tail.rs`
- `crates/builtins/src/tail.rs`, `crates/builtins/tests/tail_contract.rs` (named exceptions, stream
  A's)
- `crates/builtins/src/lib.rs`, `input_section_bounds_within`, the both-disabled branch only
  (named exception, root's ruling of 2026-10-08: the branch reaches `tail.rs`, so that
  `input_section_bound` and `input_section_bounds` agree for a strip with no filters)
- `docs/derivations/1379-graph-tail-composition.md` (new),
  `docs/derivations/1329-input-section-tail-and-rest.md` (a pointer to the new note),
  `docs/EFFECT_CONTRACT_V1.md` (the fixed section's statement)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`
- Named exceptions, root's rulings of 2026-10-08 (after the restarted attempt 1):
  - `crates/builtins-compiler/src/lib.rs`, the assertion in
    `tests::live_input_lane_reports_the_live_bound_and_plain_input_its_own` that expected
    `NodeTailBound::ZERO` for a disabled-filter strip (`:12011-12014` at `f3e6b1539`): its expected
    value is now `input_section_bound`'s for that strip;
  - `crates/builtins/src/lib.rs`, the doc comment of `input_section_bounds` (`:3520-3521` at
    `f3e6b1539`, "reports the zero bound"), made true;
  - `crates/builtins/examples/input_bound_budget.rs`, `calibrate` and the frame-equivalent
    (root's frame-equivalent statistic, F5);
  - the stale budget figures in the local specs of #1468 (`:152`), #1470 (`:28-31`, `:48`) and
    #1471 (`:21`), as under #1474's authorization.
- Root's second ruling of 2026-10-08 (the frame-equivalent from per-design medians): the same
  example, `crates/builtins/src/tail.rs`'s constants and docs, the same stale figures, and a note
  in #1474's record that #1465 restated its figures.

## Non-goals

- The live input section (*Certify the live input section's decay, peak gain and flush stall* and
  *Derive the live input section's tail gain and state its composition*).
- Any graph or report change (#1379). Effects (#1372-#1376).
- Tightening `O`'s triangle inequality (F-D7).

## Hazards

- **#1329's values must not move.** `T_decay`, `T_rest`, both rests and `P*` stay exactly as
  today (F4). The new code only adds values.
- **Preparation cost.** The extended horizon lengthens the fixed computation (8.8 ms to about
  25 ms per design for the top pair, by estimate, in one pass). A miss of #1457's budget stops the
  slice; there is no fallback value.
- **The tightness lines are calibrated on legal rows only.** Do not add an HPF-above-LPF row: it is
  refused at preparation (`FilterOrder`).

## Objective gates

`crates/builtins/tests/tail_contract.rs`; every launch rate in release, 48 kHz in debug.

- **F1. Decade law.** For every enabled pair of #1329 gate 1(a) at trims {0, +24 dB}:
  - (a) *soundness of the exact half:* `T_b(eps 10^(-k) / (2 |trim|)) <= T + k D` for `k = 0..8`,
    on the unity-trim impulse response, against the independent `f64` brute force of #1329 gate
    1(a) (4,000,000 samples; the top pair at +24 dB needs about 0.81M for `k = 8`);
  - (b) *tightness:* `D <= 1.5 max(D_mean, D_floor)`, with
    `D_mean = (T_b(eps 10^(-8) / 2) - T_b(eps / 2)) / 8` the exact mean decade length and `D_floor`
    (d)'s floor; the measured ratio is recorded per row (probe: a correct construction gives at
    most about 1.12x on every row);
  - (c) *identity:* the module's directly searched crossing `T(k)` (accessor) is at most `T + k D`
    for `k = 0..64`;
  - (d) *asymptotic floor:* `D_inf` (accessor) is at least
    `D_floor = ceil(ln 10 / -ln(max_s (q_s + mu_s)))`, with each enabled section's contraction
    `q_s` and state rounding `mu_s` recomputed in the test from the designed words by #1329's closed
    forms (plain `f64`, no code shared with `math::tail`).
- **F2. Gains.**
  - (a) *soundness:* `|trim| ||h||_1` of the brute-force cascade is at most `g_t`, and
    `g_t <= g_p`;
  - (b) *tightness:* `G_p <= 20 log10(|trim| ||h_HPF||_1 ||h_LPF||_1) + 6.02` dB (the enabled
    sections' own brute-force norms) on gate 1(a)'s enabled pairs and, at every launch rate, on the
    top pair (HPF one `f32` below the maximum into the LPF at the maximum) and on the HPF at 10 Hz
    into the LPF one `f32` above 10 Hz; the margin (with `dev_loud`, `(1 + u)` and the millibel
    rounding included) is recorded per row (probe: +0.94 to +1.16 dB on gate 1(a)'s worst row,
    +3.6 dB on the top pair, +0.74 to +0.75 dB on the 10 Hz row);
  - (c) *filters disabled:* `Stated` with `D = 0`, `G_p = G_t = ceil_mB(|trim| (1 + u))`, or
    `ceil_mB(|trim|)` where the product is exact (checked at 0 dB, +6.02 dB (`|trim|` = 2) and a
    non-power-of-two trim such as +3 dB), and `sigma` the underflow allowance;
  - (d) *`dev_loud` and its use:* the module's `dev_loud` lies between a plain-`f64` evaluation of
    `Deviation::at_end`'s fixed point in the test and that value plus 1 mB. The test takes the
    section constants from the designed words and the first section's output supremum
    `input_sup[1]` as the brute-force `l1` of the first section's impulse response (the module's
    majorant sums `|h_1(t)|`; a closed form of the section constants there is several dB high), with
    `state_sup[0] = beta_1 / (1 - q_1)` and the later `state_sup[i] = beta_i input_sup[i] / (1 - q_i)`;
    no code shared with `math::tail`. And `G_p` equals `ceil_mB(|trim| (1 + u) (O + dev_loud))`
    computed in the test from the accessors.
- **F3. Stall.** `sigma` is the module's stall rounded up to millibels; `P^ = 4 sigma / eps` is at
  least the module's `P*` (`flush_floor`) on every row; every enabled section has `sigma > 0`.
- **F4. Nothing certified moves.** Every existing `tail_contract` assertion passes unchanged
  (`T_decay`, `T_rest`, both rests, `P*`, the live figures); no rendered bit moves (`audit capi`'s
  `pcm_digest`, the wasm G5 digests, the builtins PCM fixtures).
- **F5. Budget.** #1457's gates 2 (the worst case across its design families, the top pair at +24 dB
  and the 64 near-top designs among them, first preparation and rebuild) and 8 (every 64-track
  console document bounded exactly at every launch rate) pass after this change, within #1457's D1
  budget. A change to the walk's cost also reruns `target/release/examples/input_bound_budget
  calibrate` and restates the frame-equivalent and
  `builtins::INPUT_BOUND_SECTION_CHARGE` (#1457 Amendments 3 and 4; the per-design charge was
  removed in Amendment 4). *(Superseded by root's second ruling below:)* ~~Root's ruling of
  2026-10-08 replaces the per-frame statistic: the frame-equivalent is the smallest value, rounded
  up to 0.5 ns, such that every recorded calibration sample and every gate-2 sample satisfies:
  measured design work <= (frames walked + `INPUT_BOUND_SECTION_CHARGE` x sections) x
  frame-equivalent.~~ **Root's second ruling of 2026-10-08 (in force):** each design (a
  calibration design, a frame-class point, a gate-2 family at a rate) is measured R >= 5 times
  within the one invocation, and its cost is the **median** of its repeats. The frame-equivalent
  is the smallest value, rounded up to 0.5 ns, such that every design satisfies: median work <=
  (frames walked + `INPUT_BOUND_SECTION_CHARGE` x sections) x frame-equivalent, gate 2's families
  included (their per-family medians). The section charge comes from the largest per-design-median
  fixed cost per section, in frame-equivalents, rounded up to a ten, found jointly with the
  frame-equivalent. Reason: the computation is deterministic and interference only adds time, so
  the per-design median bounds the cost the budget states, while a single preemption does not set
  the constant. The raw maximum sample and its ratio to its design's median are recorded as
  evidence. `calibrate` computes it directly and prints the binding design; gate 2 prints what its
  own medians need. The per-frame medians and spreads stay descriptive. *(The second fold-in's
  sampling, one walk a calibration sample, is superseded by root's ruling (c) below.)* **Root's
  ruling (c) of 2026-10-08 (in force), the sample:** the median rule stands, on a
  better-conditioned sample. `calibrate` times each short design class as a **batch** of N
  back-to-back walks, so that a sample lasts at least about 1 ms, and takes the median over R >= 5
  batches' samples. A batch must not make a sample cheaper than a real preparation, which walks
  each distinct design once: so each batch walks N **different** designs of the same class, in
  the order a preparation walks them (one preparation, fresh cache, no budget limit); a class that
  cannot supply N different designs shows by measurement that the batch's per-walk cost is not
  below the same designs' single-walk cost, and records it. The fit and the joint search run on the
  per-batch medians, normalized per design. **"Every design", batched:** every batch's median work
  per design is at most (its frames walked per design + `INPUT_BOUND_SECTION_CHARGE` x its sections
  per design) x frame-equivalent, that is, the whole batch's median work against the whole batch's
  charged frame-equivalents; a long walk that already lasts over 1 ms (the near-top frame-class
  points) is a batch of one; each gate-2 family at a rate is one design at its own median. **The
  figure of record** is the batch verdict's single calibration and gate-2 invocation, run by the
  batch verifier; the constants are restated in the batch follow-up if it differs. Gate 2 (real
  preparations, charged against measured) is the final check: a gate-2 family whose median exceeds
  the stated budget means the constant is wrong.
- **Commands:** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
  --features math/lane,builtins/test-support,lane/test-support`; `cargo test --locked --release -p
  builtins --features builtins/test-support --test tail_contract`; #1457's gate 2 and gate 8
  commands (#1457 Amendment 3: gate 2 is `cargo build --locked --release -p builtins --features
  test-support --example input_bound_budget` and one invocation of `taskset -c <core>
  target/release/examples/input_bound_budget`, descriptive; gate 8 is `cargo test --locked -p
  builtins-compiler --features test-support --lib
  every_sixty_four_track_console_document_is_bounded_exactly_at_every_launch_rate`); `cargo build
  --locked --release -p audit && ./target/release/audit capi`; `bash
  scripts/check-builtins-fixtures.sh . target/release/audit`; `bash scripts/run-wasm-gates.sh`;
  `bash scripts/check-workspace-policy.sh`; `cargo clippy --locked --workspace --all-targets -- -D
  warnings`; `cargo fmt --all -- --check`.

The attempt record carries a mutation table (each defect applied, the named test run, the file
restored) with at least the mutants below; each is red.

## Test value

- F1(a): a decade law whose exact half is short (for example the shortest one-decade step of the
  exact suffix for `k <= 8` at the 44.1 kHz top pair, 36,270 samples at 0 dB and 33,502 at +24 dB,
  against means of 45,467 and 45,382, which falls 47,424 and 67,574 samples short of the brute
  force by `k = 8`; the attempt-1 verifier's figures) is red; nothing tests a decade law today.
- F1(b): a needlessly loose `D` exceeds the 1.5 line. Two mutants, each red at (b) at its own
  first failing row (the test stops at its first failure, so no claim is made for later rows):
  the certificate rate halfway between the floor rate and 1 alone (`lambda = (1 + rho) / 2`, the
  grid's `j = 2`, so `D_inf` is about 2 `D_floor`; M2) is red at the first row, the 10 Hz LPF at
  0 dB and 44.1 kHz, `D` 5,422 against 1.5 x 2,301.875 (the attempt-1 verifier's run); the coarse
  grid cut to `j = 2` with the refinement kept, so the search covers `j in {1, 2, 3, 5}` (M2r, the
  mutant attempt 1 ran), passes five rows and is red at the 10 Hz HPF at +24 dB, `D` 3,758 against
  1.5 x 2,299.75.
- F1(c): a `D` with no transient term (the asymptotic rate alone) falls below the module's own
  crossing at `k = 1`, where the module's majorant takes its longest step (the polynomial factor of
  near-equal poles; steps 48,531, 48,136, 47,816, ... at the top pair). It is caught first by (a)
  (mutation M3: the 10 Hz LPF, `T_b` at `k = 1` 19,895 against 17,408 + 2,286); (c) alone still
  catches it with (a) disabled (M3b: `T(1)` 22,066 against the same line).
- F1(d): a certificate whose rate omits the `f32` state rounding (`lambda` from `q_s` alone, about
  2 % of the decade length at the top of the domain) is below the floor; F1(a) and F1(c) cannot see
  it, because the per-decade transient allowance absorbs it.
- F2(a): a gain that drops the trim, or the cascade's second section (10 Hz HPF into the top LPF:
  `l1` 3.51 against the HPF's alone), falls below the brute-force `l1`.
- F2(b): a gain from the two sections' state norms (`||c|| ||b|| / (1 - q) + |d|` each) exceeds the
  line. With (d)'s equality disabled it first fires at the HPF at 10 Hz into the 1 kHz LPF
  (mutation M7b: 1,516 mB against the line's 14.497 dB), not at the top pair; with (d) in place,
  (d)'s equality catches it first (M7).
- F2(c): a trim-only section that drops the `(1 + u)` rounding for a non-power-of-two trim (M8:
  600 instead of 601 mB at +6 dB), drops the trim, or states `D > 0`.
- F2(d): a `G_p` built on `dev_core` (#1329's deviation from `T_decay` on) instead of `dev_loud` is
  about 59 mB low at the top pair and fails the equality; a `CascadeBound` that returns `dev_core`
  as `dev_loud` falls below the recomputation's lower edge. F2(a) has about 14 dB of slack at the
  top pair and F2(b) is an upper line, so neither can see it.
- F3: a stall of zero (M13, `FlushStall::Zero`) is red in `composition_values`'s `Level` match,
  before any `P^` comparison; a raw stall too small to cover the flush floor (M13m, the stall
  times `1e-3`) is red at `P^ >= P*` (`P^` 1.65e-12 against `P*` 8.26e-10). A stall rounded down
  is caught by the equality with `ceil_mB(stall)`, not by `P^ < P*` (M14).
- F5: reuses #1457's gates; a decade law that extends the horizon past the budget (for example one
  run per `k`) is red there.

## Dependencies

- *Carry every node's tail bound in one node-neutral struct* (slice A1, #1464): the carrier.
- *Cache design bounds across preparations within a stated preparation budget* (#1457): the gated
  budget.
- *Remove the data-dependent cost of the near-top tail walk* (#1474, #1457 Amendment 4): it edits
  `crates/math/src/tail.rs`, `crates/builtins/src/tail.rs` and `tail_contract` before this slice,
  and restates the frame-equivalent and the section charge this slice's F5 reruns against.
- *State a bounded tail and an exact-rest bound for every node* (#1329, passed): the module and the
  gates this slice extends.
- Named exceptions: `STREAMS.md` (#1379 Amendment 1, H8).

## Attempt record

### Attempt 1 (2026-10-08): stopped before implementation, spec problems

Anchors re-verified on `codex/d15-stream-g3` at `bc2299a08`. The `math::tail` line anchors moved
with #1474 (`Deviation::at_end` is now `tail.rs:2004-2029`, `dev_sup_from_core` and `P*`
`:2376-2386`); their content is as the spec states. Two problems stop the attempt; no code changed.

1. **F-D2's extended pass cannot fit F5 (gate 8).** The extended horizon `T(16)` is about 3.07 `T`
   for a typical design, not only for the top pair. An `f64` probe of the designed cascades
   (exact Butterworth words, brute-force suffix sums): HPF 40 Hz into LPF 18 kHz at 96 kHz, `T_b`
   9,510, `T_b(eps/2 10^-16)` 29,231; HPF 20 Hz into LPF 16 kHz at 48 kHz, 9,509 and 29,230; HPF
   80 Hz into LPF 20 kHz at 96 kHz, 4,756 and 14,616 (ratio 3.07 on each). #1457's gate 8 record
   walks 1,254,016 frames for the 64 designs of the 64-track console documents at 96 kHz (charge
   about 1.30M of 1,510,000 at 380 a section). The majorant pass is most of each walk, so walking it
   to `T(16)` takes about 3.5M frames at 96 kHz (and about 3.9M at 88.2 kHz's ratio of today's
   figures), over twice the budget. The spec's cost hazard (8.8 ms to about 25 ms per design)
   counts one design, not a 64-design session. Under the spec this stops the slice ("a miss of
   #1457's budget stops the slice; there is no fallback value"); the miss is certain by this
   estimate, so the construction (or the budget) needs a root ruling before implementation.
2. **The disabled-filter values do not reach preparation inside the authorized paths.**
   `input_section_bounds_within` (`crates/builtins/src/lib.rs:3557-3566`) answers a design with
   both filters disabled on both channels with `NodeTailBound::ZERO` itself (composition
   `Unstated`), before `tail.rs` is called. F-D3's trim-dependent `Stated` values for that design
   need that branch to call `tail.rs` (a named exception for those lines); without it
   `input_section_bound` and `input_section_bounds` disagree for the most common strip. Also,
   `tail.rs`'s own memoryless branch (`fixed_input_walk`, `:186-191`) reads only the left channel;
   with a trim-dependent gain it must take both channels' maximum (inside the authorized file).

### Root's rulings (2026-10-08, after attempt 1's stop)

1. **F-D2 amended** (the amendment note beside F-D2 above): walk only to `T`; cover every `k >= 1`
   by a certified closed form (the contraction certificate extended down to `k >= 1`, or powers of
   the non-negative step), the tighter sound one, with its reason; write the closed form and its
   full proof (rounding and #1474's `tau` floors) in the derivation before the code; measure `D`'s
   looseness against the walked `T(k)` (off-line, never shipped) on gate 2's families, and report it
   if `D` exceeds 2x the walked value on any family (root then files a tightening successor). Budget
   and gate 8 unchanged; gate 8 exact at every rate.
2. **Named exception, authorized:** `crates/builtins/src/lib.rs`, `input_section_bounds_within`,
   the both-disabled branch only, reaches `tail.rs`, so that `input_section_bound` and
   `input_section_bounds` agree for a strip with no filters; and `tail.rs`'s memoryless branch
   (`fixed_input_walk`) takes the larger of the two channels' values, not the left channel's. Each
   with a test and a mutation run.

### Attempt 1, restarted under root's rulings (2026-10-08, implementer)

**The closed form** (`docs/derivations/1379-graph-tail-composition.md`, "`D`: the closed form",
written before the code). Each half (the reference against `eps / 2`, the relative deviation
against `eps / 4`) is a non-negative lower-triangular linear system: `S(t + j) <= alpha . M^j w(t)`
for the reference (`M` the majorant step for `t >= 1`, `w` the remainder's sums), `dev(m + j) <=
ell . L^j z(m)` for the deviation (`L` = `linear_map`). Each half's state is carried to `T` by
powers of its step (no frame walked: the reference from `t0`, whose majorant state the replay of
`t0`'s block records; the deviation from the walk's last frame), rounded up by `tau`. From `T` the
least contraction certificate `v >= z`, `G v <= lambda v` (forward substitution, then verified on the
computed values) gives `B lambda^i`; with `a = ln(B / h) / -ln lambda`, `b = ln 10 / -ln lambda`,
`D_h = o + floor(max(a, 0) + b) + 1`, `D = max(D_ref, D_dev)`, and `T + k D >= T(k)` for every
`k >= 1` (proof in the note). **Why this form:** powers alone give each `T(k)` only for finitely
many `k` (16 bisections of a `4 x 4` power, more than a typical design's walk) and still need a
rate for the rest; the certificate covers every `k` in one formula. Powers are used once, to
anchor the certificate at `T`: anchored at the walk's last frame it must cover the near-equal-pole
factor over the whole of `T` with one rate (up to about 0.43 decade looser at the top pair).

**Values (44.1 kHz, `probe` records; gate F1-F3 prints every row):** top pair +24 dB: `T` 478,335,
`D` 49,403, `G_p = G_t` = 3,489 mB (`g (O + dev_loud)` = 55.52, `O` 3.2719, `dev_loud` 0.2310),
`sigma` = `Level(ceil_mB(1.519e-19))`; typical 20 Hz into 20 kHz, 0 dB: `T` 9,272, `D` 1,271, `O`
6.3842, `dev_loud` 0.0004; 1 kHz LPF, 0 dB: `T` 175, `D` 27. Disabled filters: `D = 0`, `G` 0 mB
at 0 dB, 603 mB at `|trim| = 2`, 601 mB at +6 dB (`ceil_mB(|trim| (1 + u))`; 600 without the
`(1 + u)`), `sigma = Level(-75,859)` (`2^-126`).

**Looseness against the walked `T(k)` (measurement only, never shipped).** A temporary function
walked the module's own majorant pass and deviation frame by frame to the `k = 16` floor and took
`D_walked = max_{k <= 16} ceil((T_walked(k) - T) / k)`; it and its probe test were deleted before the
commit. 110 rows, the four launch rates: every gate-2 family's representatives (top pair 0 and
+24 dB, LPF at the maximum, near-top `i = 1, 32, 64`, 65,537 near-top `j = 100, 65,537`, band +1 and
+4,096, typical 20 and 23.15 Hz into 20 kHz, the cheap one-section, two-section and two-cascade
designs) and the gate 1(a) style rows (10 Hz HPF and LPF, 10 Hz into 1 kHz and into the maximum, 10
Hz into one `f32` above, 1 kHz at +24 dB). `D / D_walked`: top pair 1.018-1.028, LPF at the maximum
1.010, near-top 1.012-1.028, band 1.010, typical 1.085-1.108, cheap one-section 1.080-1.120, cheap
two-section 1.100-1.222 (a 9- or 5-sample decade: 11 against 9), 10 Hz HPF +24 dB 1.217, 1 kHz HPF
+24 dB 1.231. **Worst 1.231, below root's 2x line** (no successor needed by that rule). Every row
also checked `T_walked(k) <= T + k D` and `T(k) <= T + k D` for `k <= 16`. `T_walked(0)` is within
0-77 frames below `T` (the walk's longer summation factor), as expected.

**F4, nothing certified moves.** A one-time comparison (PR evidence, not a committed test) of
`fixed_cascade_within`'s `tail`, `tail_every_peak`, both rests, `P*` (bits), `R(P*)`,
`tail_reference` and frames walked on 3,024 designs (19 cutoffs per section, HPF-below-LPF pairs,
trims -12, 0, +12, +24 dB, four rates) between `ee1ef4222` (an export, deleted after) and this change:
identical. Every existing `tail_contract` assertion passes unchanged; the live table is unchanged
(the live bound states `Unstated`).

**Gates.**

- F1 (release, every rate): `D <= 1.5 max(D_mean, D_floor)`, ratio 1.046-1.391 (worst: 1 kHz HPF
  +24 dB at 44.1 kHz, `D` 32 against `D_mean` 23; the certificate's transient there is the closed
  remainder's `gamma ||s||` against `|c . s|`; the walked construction's probe was about 1.12x); (a)
  and (c) for `k = 0..8` and `0..64`; (d) per half.
- F2: margins under the line +0.743 to +0.752 dB on the HPF 10 Hz into one `f32` above 10 Hz row
  (the least), every row non-negative; (a), (d) on every row.
- F3: on every row.
- F2(c) and the exception: `a_design_with_filters_disabled_states_its_trim_gain`.
- F5: below.

**F1(d) reworded.** `D_inf` is the larger of the two halves' rates, so a too-fast deviation rate
hides behind the reference half's slower one (mutant M4c: green against `D_inf >= D_floor` alone).
(d) now checks each half against its own propagation's spectral radius, recomputed from the words:
the deviation's frames per decade at least `ceil(ln 10 / -ln max_s (q_s + mu_s))`, the reference's
at least that of `max_s q_s`. The spec's mutant ("a rate from `q` alone") is not reachable in this
construction: the certificate is verified on `L`, whose diagonal holds `q_s + mu_s`, so any rate
below it fails the verification (M4, M4c green or caught elsewhere); (d) defends the stated rate
against a defect after the verification (M4d).

**Mutation table** (release, `tail_contract`, each mutant applied, the named test run, the files
restored from a pristine copy; `crates/math/src/tail.rs` unless stated):

| mutant | test | result |
|---|---|---|
| M1 `D` three quarters of the certified | F1 | red at (a): 10 Hz LPF, `T_b` at `k = 1` 19,895 > 17,408 + 2,033 |
| M2r coarse grid cut to `j = 2`, refinement kept (`j in {1, 2, 3, 5}`; the mutant run here, recorded first as "rates only at `lambda = (1 + rho) / 2`") | F1 | red at (b): 10 Hz HPF +24 dB, `D` 3,758 > 1.5 x 2,299.75 (first failing row; every row not checked) |
| M2 rates only at `lambda = (1 + rho) / 2` (`j = 2` alone; the attempt-1 verifier's run) | F1 | red at (b), first row: 10 Hz LPF 0 dB 44.1 kHz, `D` 5,422 > 1.5 x 2,301.875 |
| M3 `D` without the transient | F1 | red at (a) (10 Hz LPF, 19,895 > 17,408 + 2,286) |
| M3b M3 with (a) disabled | F1 | red at (c): `T(1)` 22,066 > 17,408 + 2,286 |
| M4 deviation certificate on `L` without `mu` on its diagonal | F1 | **green**: the rate search keeps `lambda` above `q + mu` on every row |
| M4c M4 at the rate nearest `q` | F1 | red at (b) (`D` 5,822), (d) green before the rewording |
| M4d stated rate from `rho - 0.05 (1 - rho)` | F1 | red at (d): deviation half 2,176.97 < `D_floor` 2,286, (a)-(c) green on the row |
| M5 trim dropped from `G_p` | F2 | red at (a): 17.28 > 1.09 |
| M6 `O` from the first section | F2 | red at (a): 10 Hz into 1 kHz, 2.443 > 2.435 |
| M7 `G_p` from the sections' state norms | F2 | red at (d)'s equality |
| M7b M7 with (d)'s equality disabled | F2 | red at (b): 10 Hz into 1 kHz, 1,516 mB above 14.497 dB |
| M8 `(1 + u)` dropped for a non-power-of-two trim | F2(c) | red: 600 at +6 dB |
| M9 trim dropped (disabled) | F2(c) | red: 0 at `|trim| = 2` |
| M10 `D = 1` (disabled) | F2(c) | red |
| M11 `dev_core` returned as `dev_loud` | F2 | red at (d): 1.29e-10 against 2.88e-4 |
| M12 `G_p` on `dev_core` | F2 | red at (d)'s equality (LPF one `f32` below the maximum, 0 dB, 44.1 kHz: 773 against 792) |
| M13 stall `Zero` | F3 | red in `composition_values`'s `Level` match |
| M14 stall one millibel low | F3 | red |
| M15 `lib.rs` branch states `NodeTailBound::ZERO` | exception test | red: `input_section_bounds` `Unstated` |
| M16 `fixed_input_walk`'s memoryless branch the left channel only | exception test | red: (0, 0) against the louder channel |
| M17 `memoryless_input_bound` the left channel only | exception test | red |

Test-value notes against the spec's list: F1(c)'s "the brute force cannot see it" does not hold
here (M3 is red at (a) first); (c) is still red alone (M3b). F2(b)'s mutant is red at (b) at 10 Hz
into 1 kHz before the top pair. F3's "rounded down gives `P^` below `P*`" is not what catches it: the
equality with `ceil_mB(stall)` does (M14).

**F5, cost.** The closed form adds a fixed cost per walk (about 3.3 us for a two-section cascade,
2.3 us for one, measured before the calibration; no frame is walked, so frames walked and gate 8's
frames are unchanged). One build (`CARGO_INCREMENTAL=0 cargo build --locked --release -p builtins
--features test-support --example input_bound_budget`), then one invocation of each, `taskset -c 7`,
one warmup and two measured rounds, no retry:

- *Calibration* (load 1.22 -> 1.37): the largest fixed cost per section is **9.49 us** (96 kHz, round
  1; the measured rounds' intercepts: one section at most 9.20 us, two at most 18.97 us), 512.7
  frame-equivalents of 18.5 ns: **`INPUT_BOUND_SECTION_CHARGE` = 520** (was 380), restated in
  `crates/builtins/src/tail.rs` with the cache cap's arithmetic (at least 257 + 520 = 777 a design,
  at most 1,943 designs a preparation; the cap of 8,192 holds them). **The frame classes net of the
  fixed cost rose above the 18.5 ns frame-equivalent:** slowest class median 19.64 ns (cheap
  two-section, 1 kHz into 1.28 kHz, +24 dB, 44.1 kHz, spread 19.53-19.82; 513-frame walks, whose
  per-frame residual carries the new design-dependent fixed cost the rate's mean intercept does not
  model), and the largest single sample over every point 34.18 ns (cheap two-section, 5.12 into
  10.24 kHz, +24 dB, 88.2 kHz, that point's median 7.99 ns). Under root's rule (the maximum over every
  recorded point, rounded up) the frame-equivalent would be 34.5 ns, which restates
  `FRAME_EQUIVALENT_NS` in `examples/input_bound_budget.rs` (not an authorized path) and moves the
  budget's milliseconds; **not restated, open for root.**
- *Gate 2 on charge 520 and the committed 18.5 ns* (load 0.96 -> 1.04): every family's three results
  identical, every preparation within its budget, every warm rebuild walking nothing. Worst design
  work **24.93 ms** (4,096 cheap two-section designs, 1 kHz into 1.28 kHz, +24 dB, 96 kHz, round 2)
  of the 27.94 ms budget; worst design work per consumed frame-equivalent **17.61 ns** (64 typical
  designs, 48 kHz, round 1), then 17.27 ns (top pair, 44.1 kHz), both below 18.5 ns.
- *Gate 8*: passed, every 64-track console document exact at every rate. Stereo documents: charged
  (frames walked, margin) 44.1 kHz 743,552 (610,432, 766,448), 48 kHz 791,936 (658,816, 718,064),
  88.2 kHz 1,290,880 (1,157,760, 219,120), 96 kHz 1,387,136 (1,254,016, **122,864**); mono 44.1 kHz
  383,040, 48 kHz 408,384, 88.2 kHz 667,712, 96 kHz 717,376. Frames walked equal #1474's.

**Other gates.** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
--features math/lane,builtins/test-support,lane/test-support`: pass. Release `tail_contract`: 17
passed. `audit capi`: 0 allocations, 0 locks, 0 syscalls, `pcm_digest` `cb10fbface44a3a4`
(unchanged). `check-builtins-fixtures.sh`: ok (50 files). `run-wasm-gates.sh`: ok.
`check-workspace-policy.sh`: ok. `cargo clippy --locked --workspace --all-targets -- -D warnings`:
clean. `cargo fmt --all -- --check`: clean.

**Stopped: one test outside the authorized paths is red.** `cargo test --locked -p
builtins-compiler --features test-support`: 56 passed, **1 failed**,
`tests::live_input_lane_reports_the_live_bound_and_plain_input_its_own`
(`crates/builtins-compiler/src/lib.rs:12011-12014`) pins a strip with both filters disabled to
`NodeTailBound::ZERO` (composition `Unstated`); root's named exception makes that strip state its
composition (`Stated { decay: 0, peak_gain: 0 mB, tail_gain: 0 mB, stall: Level(-75,859) }` at
0 dB), so the expected value must change to `input_section_bound`'s. The file is stream A's (#1464)
and not in this slice's paths: a named exception for that one assertion is needed. Not edited.

**Open for root.** (1) The builtins-compiler assertion above. (2) The frame-equivalent (F5): 18.5 ns
kept; the calibration's frame classes measured above it, though gate 2's real work per consumed
frame-equivalent stays below it. (3) The doc comment of `builtins::input_section_bounds`
(`crates/builtins/src/lib.rs:3520-3521`, "reports the zero bound") is outside the named exception's
branch; it now reads loosely (the bound has a zero tail and a stated composition).

### Root's rulings (2026-10-08, after the restarted attempt 1's stop)

1. **Named exceptions, authorized** (also in the authorized-paths list): the assertion in
   `builtins-compiler`'s `tests::live_input_lane_reports_the_live_bound_and_plain_input_its_own`
   (`crates/builtins-compiler/src/lib.rs:12011-12014` at `f3e6b1539`), whose expected value
   becomes `input_section_bound`'s for the disabled-filter strip; and the doc comment of
   `builtins::input_section_bounds` (`crates/builtins/src/lib.rs:3520-3521`), made true.
2. **The frame-equivalent statistic** replaces the per-frame maximum: the smallest value, rounded
   up to 0.5 ns, such that every recorded calibration sample and every gate-2 sample satisfies
   measured design work <= (frames walked + `INPUT_BOUND_SECTION_CHARGE` x sections) x
   frame-equivalent; the section charge from the largest fixed cost per section.
   `examples/input_bound_budget.rs` (authorized) computes and prints it. One calibration and one
   gate-2 invocation, each after a 1-minute load below about 2. Restate `FRAME_EQUIVALENT_NS`, the
   budget's milliseconds, the worst case's share, the cache-cap doc and the stale figures of
   #1468, #1470 and #1471. Gate 8 stays exact. A gate-2 family over the budget stops the slice.
3. **The spec's test-value text** is corrected to what was measured (F1(b), F1(c), F2(b), F3).

### Attempt 1, fold-in under root's rulings (2026-10-08, implementer)

**The exception test.** The disabled-filter strip's expected input bound is now
`builtins::input_section_bound` of the strip's own parameters (a zero tail; composition `Stated {
decay: 0, peak_gain: 0 mB, tail_gain: 0 mB, stall: Level(-75,859) }` at 0 dB). Green at this
commit. Mutation: `input_section_bounds_within`'s both-disabled branch returning
`NodeTailBound::ZERO` again: red (`composition: Unstated` against the `Stated` value); restored:
green. The `input_section_bounds` doc now says a memoryless design reports its own bound (zero tail,
trim-only composition), whatever the remaining budget, and charges nothing.

**Why the per-frame maximum is ill-conditioned for short walks.** It reads each sample as
`(time - fixed) / frames`: the timing noise of one sample (a preemption, a cache miss) is divided
by the walk's frame count, which is 513 for gate 2's cheap designs. The restarted attempt's
calibration recorded 34.18 ns a frame for a 513-frame walk (cheap two-section, 5.12 into
10.24 kHz, +24 dB, 88.2 kHz) whose point's median was 7.99 ns; this run's largest such sample is
30.51 ns (cheap two-section, 1 kHz into 1.28 kHz, 0 dB, 48 kHz; median 13.79 ns). Root's statistic
charges the whole sample, fixed cost included, against frames plus section charges.

**The statistic in the example.** `one_design` now also returns the sections charged (the charge
net of the frames, divided by the section charge, read from the test-support counter); `fixed_costs`
and `frame_classes` record every measured sample whole (warmup round and sweep excluded, as before:
they are not measurements); `frame_equivalent` searches the half-nanosecond grid upward for the
first value `F` at which, with the section charge `C(F) = ceil_10(P / F)` (`P` the largest
measured fixed cost per section), every sample's work is at most `(frames + C(F) sections) F`.
The two are found together because each follows from the other. Gate 2 prints the value its own
measured samples need (its largest design work per consumed frame-equivalent, rounded up to
0.5 ns; the consumed amount is the charge when every design is exact, and otherwise the budget,
which a preparation that stops a design has spent to within one channel's section charges).

**Calibration** (one invocation, `taskset -c 7`, one warmup and two measured rounds, no retry;
`/proc/loadavg` before 0.92 1.35 1.86, after 1.15 1.37 1.84; built with charge 520):

- 44,592 recorded samples. Largest measured fixed cost per section **9.86 us** (88.2 kHz, round 1,
  one cascade of two sections; the measured rounds' per-section intercepts 9.26-11.21 us of the
  one/two-section difference, the per-design term -2.70 to -1.56 us).
- **Frame-equivalent 48.5 ns, section charge 210** (9.86 us / 48.5 ns = 203.3, rounded up to a
  ten; 210 x 48.5 ns = 10.19 us). **Binding sample:** 44.1 kHz, round 2, one cascade of two
  sections, grid design 3059: work 57,119 ns, 769 frames + 210 x 2 sections = 1,189 charged
  frame-equivalents, 48.04 ns per charged frame-equivalent. It is one sample: that class's
  measured slope at 44.1 kHz is 16.7-16.8 ns a frame and its intercept 17.1 us, about 30 us for
  769 frames, so the sample carries about 27 us of interference. Under root's rule (a bound does
  not pick which outlier to believe) it binds.
- Descriptive (net of the fixed cost): each class's slowest point's median 13.88-17.70 ns a frame
  (slowest: cheap two-section, 1 kHz into 1.28 kHz, +24 dB, 44.1 kHz, spread 16.72-17.87 ns).

**Gate 2** (one invocation on charge 210 and 48.5 ns, `taskset -c 7`, one warmup and two measured
rounds, no retry; `/proc/loadavg` before 1.38 1.40 1.83, after 1.33 1.38 1.82): exit 0, every
family's three results identical, every preparation within its budget in frames, every warm
rebuild walking nothing. The budget is 1,510,000 x 48.5 ns = **73.235 ms**. Worst design work
**45.11 ms** (4,096 cheap two-section designs, 1 kHz into 1.28 kHz, +24 dB, 48 kHz, round 2; 1,269
of 4,096 exact): **61.6 %** of the budget. Gate 2's samples need **30.0 ns** (largest design work
per consumed frame-equivalent 29.87 ns, the same row), below 48.5 ns, so the calibration binds. No
family is over the budget. Per family (design work, measured rounds, every rate): top pair
7.26-7.45 ms; LPF at the maximum 3.72-3.76; 64 near-top 21.02-21.46; 65,537 near-top 20.90-21.70
(whole preparation up to 60.78 ms, the O(strips) keying of 65,537 strips outside the budget, as in
#1457); 64 band 20.25-23.17; 4,096 band 20.11-20.59; 64 typical 11.23-21.62 (64 of 64 exact); 256
typical 22.95-25.29; cheap one-section 23.72-29.77; cheap two-section 32.73-45.11; cheap
two-cascade 24.00-26.48. The lower section charge lets more cheap designs fit (1,044-1,618
two-section designs exact a preparation, against fewer at 520), so the worst design work rose from the restarted attempt's
24.93 ms with the budget.

**Gate 8** (charge 210): passed, every 64-track console document exact at every rate. Stereo
documents (charged, frames walked, margin): 44.1 kHz 664,192 (610,432, 845,808); 48 kHz 712,576
(658,816, 797,424); 88.2 kHz 1,211,520 (1,157,760, 298,480); 96 kHz 1,307,776 (1,254,016,
**202,224**, the least). Mono: 343,360 (margin 1,166,640), 368,704 (1,141,296), 628,032 (881,968),
677,696 (832,304). Frames walked unchanged.

**Restated.** `FRAME_EQUIVALENT_NS` = 48.5 (`examples/input_bound_budget.rs`, with its statistic
in the module doc); `INPUT_BOUND_SECTION_CHARGE` = 210 and its doc; `INPUT_BOUND_BUDGET_FRAMES`'s
doc (73.235 ms); the cache cap's doc (a computed design charges at least 257 + 210 = 467, so at
most 1,510,000 / 467 = 3,233 designs a preparation; the cap of 8,192 holds them); the stale figures
of #1468 (`:152`), #1470 (`:28-31`, `:48`) and #1471 (`:21`).

**Gates.** `cargo test --locked --all-targets -p lane -p math -p builtins -p dsp-reference
--features math/lane,builtins/test-support,lane/test-support` (includes `math`): pass. Release
`tail_contract`: 17 passed. `cargo test --locked -p builtins-compiler --features test-support
--lib`: 57 passed (the formerly red test green). Gate 8: pass. `cargo fmt --all -- --check`: clean.
`cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` (CI's flags) and
without `--all-features`: clean. `check-workspace-policy.sh`: ok. `check-realtime-policy.sh`: ok.
`check-cross-targets.sh`: PASS. `audit capi`: 0 allocations, 0 locks, 0 syscalls, `pcm_digest`
`cb10fbface44a3a4` (unchanged). `check-builtins-fixtures.sh`: ok (50 files).

**Open for root** (answered by root's second ruling below; this fold-in's 48.5 ns, 210 and
73.235 ms are superseded). The frame-equivalent rose from 18.5 to 48.5 ns and the section charge fell from
520 to 210 on one interference sample of 44,592; the budget's milliseconds rose from 27.94 to
73.235 ms, and the real worst case with it (45.11 ms), because a lower charge admits more cheap
designs. This is the ruling's statistic applied as written; it is reported for root's view.

### Root's second ruling (2026-10-08, after the fold-in; supersedes the fold-in's statistic)

Root's ruling, recorded as given: each design is measured R >= 5 times within the one invocation,
and its cost is the **median** of its repeats. The frame-equivalent is the smallest value, rounded
up to 0.5 ns, such that every design satisfies: median work <= (frames walked +
`INPUT_BOUND_SECTION_CHARGE` x sections) x frame-equivalent, gate 2's families included (per-design
medians, enough repeats that each has R >= 5). The section charge comes from the largest
per-design-median fixed cost per section (the joint search kept, the rounding stated). **Reason:**
the computation is deterministic and interference only adds time, so the per-design median bounds
the cost the budget states, while a single preemption does not set the constant. The raw maximum
sample and its ratio to its design's median are recorded as evidence. One calibration and one
gate-2 invocation, each after a 1-minute load below about 2; gate 8 stays exact; a gate-2 family
over the budget stops the slice. The fold-in's statistic, 48.5 ns, 210 and 73.235 ms stay above
as superseded.

### Attempt 1, second fold-in under root's second ruling (2026-10-08, implementer)

*(Its sampling, one walk a calibration sample, and its 21.5 ns, 470 and 32.465 ms are superseded
by root's ruling (c) and follow-up 2 below.)*

**The statistic in the example** (`crates/builtins/examples/input_bound_budget.rs`). Every
workload now runs one warmup and **five** measured rounds (`REPEATS` = 5, `ROUNDS` = 6; was two
measured rounds), so every design has R = 5 samples: each calibration grid design (each round
measures every design once, so a burst of interference reaches one sample of many designs), each
frame-class point (five sweeps, as before) and each gate-2 family at a rate. A design's cost is the
median of its five samples (the middle one). `fixed_costs` fits each class's line on the designs'
medians (each round's own line is still printed, descriptive); the section charge is the largest
median-fit fixed cost per section over the classes and rates. `frame_equivalent` keeps the
fold-in's joint search: on the half-nanosecond grid upward, the charge at `F` is
`C(F) = ceil_10(P / F)` (`P` the largest per-design-median fixed cost per section, rounded up to a
ten), and the first `F` at which every design's median is at most `(frames + C(F) sections) F` is
the frame-equivalent. Gate 2's design is a family at a rate: its five measured rounds' design work
(the no-cache preparation less the warm rebuild, as before) gives the median; its consumed
frame-equivalents (the charge when every design is exact, otherwise the budget) are asserted equal
in every round (the computation is deterministic), and gate 2 prints the frame-equivalent its
medians need and each family's median, largest sample, their ratio and its share of the budget.

**Calibration** (one invocation, `taskset -c 7`, built with charge 210 (the sections counted do not
depend on it), one warmup and five measured rounds, no retry; 48.7 s; `/proc/loadavg` before
1.94 3.07 2.84, after 2.48 3.05 2.84; the wait for the 1-minute load below 2 was not a retry):

- 21,504 designs (the fixed-cost grid and the frame-class points of the four rates), five samples
  each. Largest per-design-median fixed cost per section **9.93 us** (88.2 kHz, one cascade of two
  sections, median-fit intercept 19.86 us); the median fits' per-section costs 8.54-9.93 us over
  the classes' largest at each rate, and their per-design terms -1.01 to -0.09 us. (Each measured
  round's own fit: largest per section 8.65-14.17 us, the 14.17 us of 44.1 kHz round 1 an
  interference round; descriptive.)
- **Frame-equivalent 21.5 ns, section charge 470** (9.93 us / 21.5 ns = 461.8, rounded up to a
  ten; 470 x 21.5 ns = 10.11 us). **Binding design:** 96 kHz, one cascade of two sections, grid
  design 1864: median work 41,890 ns, 1,025 frames + 470 x 2 sections = 1,965 charged
  frame-equivalents, 21.318 ns per charged frame-equivalent.
- **Raw maximum sample (evidence):** 386,907 ns, 44.1 kHz, two cascades of one section, grid design
  156, **13.26 x** its design's median of 29,176 ns (174.1 ns per charged frame-equivalent of the
  2,222). It is the largest raw sample per charged frame-equivalent and also the largest ratio of a
  raw sample to its design's median.
- Descriptive (net of the fixed cost): the slowest frame class's slowest point's median 19.40 ns a
  frame (cheap two-section, 1 kHz into 1.28 kHz, +24 dB, 44.1 kHz, spread 18.93-20.32); the
  classes' slowest points' medians 13.30-19.40 ns.

**Gate 2** (one invocation on charge 470 and 21.5 ns, `taskset -c 7`, one warmup and five measured
rounds, no retry; 16.1 s; `/proc/loadavg` before 1.95 2.60 2.72, after 2.12 2.60 2.72): exit 0,
every family's three results identical in every round, every preparation within its budget in
frames, every warm rebuild walking nothing. The budget is 1,510,000 x 21.5 ns = **32.47 ms**
(32.465). Gate 2's medians need **19.0 ns** (largest median design work per consumed
frame-equivalent 18.523 ns: 4,096 cheap two-section designs, 1 kHz into 1.28 kHz, +24 dB, 88.2
kHz), below 21.5 ns, so the calibration binds. **Worst median design work 27.97 ms, 86.2 % of the
budget** (the same family and rate). No family's median is over the budget. Medians per family
over the four rates: top pair 7.31-7.36 ms; LPF at the maximum 3.74-3.79; 64 near-top
21.09-21.34; 65,537 near-top 21.02-21.37; 64 band 20.27-20.52; 4,096 band 20.32-20.52; 64 typical
11.24-21.83 (64 of 64 exact); 256 typical 22.52-23.79; cheap one-section 18.99-19.29; cheap
two-section 21.06-27.97; cheap two-cascade 19.04-19.45. Largest-to-median ratio 1.000-1.055 for
every family and rate but two. **Raw samples (evidence):** the largest raw design work is
**46.77 ms** (4,096 cheap two-section designs, 1.28 into 5.12 kHz, +12 dB, 44.1 kHz), **1.893 x**
its median of 24.71 ms and 144.1 % of the budget (30.98 ns per consumed frame-equivalent); the
other outlier is 45.78 ms, 2.174 x its median of 21.06 ms (65,537 near-top, 48 kHz). Each is one
of five samples whose other four lie near the median; under root's ruling they do not set the
constant. The whole preparation of 65,537 strips reached 69.36 ms in that interfered round (59.06-
61.16 ms otherwise; the O(strips) keying outside the budget, as in #1457).

**Gate 8** (charge 470): passed, every 64-track console document exact at every rate. Stereo
documents (`console-sixty-four-track`, `-app`, `-intended`, `-sends`, identical; charged, frames
walked, margin): 44.1 kHz 730,752 (610,432, 779,248); 48 kHz 779,136 (658,816, 730,864); 88.2 kHz
1,278,080 (1,157,760, 231,920); 96 kHz 1,374,336 (1,254,016, **135,664**, the least). Mono:
376,640 (margin 1,133,360), 401,984 (1,108,016), 661,312 (848,688), 710,976 (799,024). Frames
walked unchanged.

**Restated.** `FRAME_EQUIVALENT_NS` = 21.5 (`examples/input_bound_budget.rs`, with the statistic
in its module doc); `INPUT_BOUND_SECTION_CHARGE` = 470 and its doc; `INPUT_BOUND_BUDGET_FRAMES`'s
doc (32.465 ms; it bounds the median work); the cache cap's doc (a computed design charges at
least 257 + 470 = 727, so at most 1,510,000 / 727 = 2,077 designs a preparation; the cap of 8,192
holds them); the figures of #1468 (`:152-154`), #1470 (`:28-36`, `:50`) and #1471 (`:21`); a note in
#1474's record that #1465 restated them.


### Attempt 1 follow-up: verdict MINOR 1-4 and NITs (2026-10-08, implementer)

Attempt 1 PASSed (`/home/bl/misofm/submix-verdicts/1465-attempt1.md`). This follow-up applies its
four MINORs and seven NITs.

- **MINOR 1, the underflow step** (`docs/derivations/1379-graph-tail-composition.md`, "Rounding").
  Rewritten. A loss in an intermediate power reaches the carry as `G^a E G^b z`: two resolvent
  factors, proportional to `z`. The resolvent is now bounded by the verified gap
  (`G_pp SLACK^3 < lambda <= 1 - 2^-53`, so `1 / (1 - G_pp) < 2^29`; entries of `R` below `2^123`
  for `L`, `2^60` for `M`), not by `2^53`. With fewer than `2^12` roundings, the carried error is
  below `2^-814 max z + 2^-939`. From #1329's builtin constants and the gap, `max z < 2^94` on both
  halves, so the error is below `2^-719`, and `tau = 2^-600` covers it by a factor above `2^119`.
- **MINOR 2, the anchor condition** (`crates/math/src/tail.rs`). **Choice: test `t0 == 0`** and
  anchor at the pass end `H` (the proof's text as written), not `max(t0, 1)`. Reasons: (1) it
  makes the code and the proof test the same quantity; (2) `H` is sound for every `t0`, because
  the pass's remainder sums bound the half from `H` on and the offset `H - T` claims nothing before
  it; (3) no builtin design reaches `t0 = 0`, so the extra tightness of `max(t0, 1)` has no value.
  The derivation now explains the tie. **The case is reachable through `math::tail`'s own
  `gain`** below the builtin trim floor. A temporary probe found ties for two of seven designs
  (BAND_HPFS[1]: 7 of the 64 gains below the `t0` boundary; LOW_LPF into TOP_LPF: 1). New `math`
  test `the_reference_certificate_is_sound_where_the_crossing_is_at_frame_zero`: for four designs,
  bisect to the least gain with `t0 > 0`, check the 64 gains below it, and require that from
  `T + crossing(k)` (`k = 1..3`) the walked majorant suffix is below `h 10^-k`. **Mutation**
  (`first_block == 0` restored): red, `gain 2.854376257044066e-8: the walked suffix 1.105e0 from
  frame 0 (k = 1) is not below 1.105e-1`; green when reverted. Test value: an anchor that reads
  frame 0's pre-impulse record when the crossing block is the first but `t0 = 0` (a certificate
  that states the reference as `tau`) turns it red; no other test reaches `t0 = 0`. Builtin bounds
  do not move: every printed row of the release `tail_contract` run (`--nocapture`,
  `--test-threads=1`) is identical between `55a92ec11`'s `tail.rs` and this change.
- **MINOR 3.** `P* <= P^ < 2 P*` is now `P* <= P^ < 2 * 10^(1/2000) (1 + 2^-29) P* < 2.0024 P*`
  (derivation, contract and "The stall `sigma`"; this spec's contract section). Measured on this
  change's release `tail_contract` run: `P^ >= 2 P*` on 16 of 112 F3 rows, maximum 2.0016 (88.2 kHz,
  1 kHz HPF at 0 dB, and 1 kHz LPF at +24 dB: `P^` 3.4085e-11, `P*` 1.7029e-11).
- **MINOR 4.** F1(b)'s test-value text and the mutation table now name both mutants, each with its
  own first failing row. M2r (coarse grid cut to `j = 2`, refinement kept, so `j in {1, 2, 3, 5}`)
  is the mutant attempt 1 ran: red at the 10 Hz HPF +24 dB, `D` 3,758. M2 as named (`j = 2` alone)
  is the verifier's run: red at the first row, the 10 Hz LPF 0 dB, `D` 5,422 > 1.5 x 2,301.875.
- **NITs.** F3's text: M13 is red in `composition_values`'s `Level` match, and the verifier's M13m
  (raw stall x `1e-3`) is red at `P^ >= P*`. M12's row: the LPF one `f32` below the maximum, 773
  against 792. F1(a)'s figures: the verifier's reproduced 36,270 and 47,424 (0 dB), 33,502 and
  67,574 (+24 dB), means 45,467 and 45,382. F2(c): the measured M8 (600 instead of 601 mB at +6 dB).
  Derivation: only `D` is below `HORIZON_LIMIT`; "reached at `k = 1`" holds when the half that sets
  `D` has `a >= 0`, with the `a < 0` case stated. `docs/EFFECT_CONTRACT_V1.md`: a fixed design over
  the budget or with no verified certificate states `Unstated`.

**F1(b) at a -144 dB trim (evidence only; temporary probe, not committed; the gate is
unchanged).** One release run of `fixed_design_decay_law_is_sound_tight_and_above_its_floor` with
the trims `[0, 24, -144]` and the 1.5 assertion skipped at -144 dB (assertions (a), (c), (d) kept).
All 56 rows at -144 dB pass (a), (c) and (d). **No row reaches the 1.5 line: the largest ratio is
1.358.** At the 44.1 kHz top pair: `T` 59,596, `D` 70,794, `D_mean` 52,147.1, `D_floor` 45,036,
ratio 1.358 (88.2 kHz the same; 48 and 96 kHz: `D` 70,414, `D_mean` 51,863.4, 1.358). Next: the
10 Hz HPF into the 1 kHz LPF, 1.331-1.338. The verifier's prediction compared `D` with the +24 dB row's
`D_mean` (45,382). At -144 dB, `T` is early (59,596 against 478,335 at +24 dB), and the exact
suffix's first decades after it are longer (the polynomial factor of the near-equal poles), so
that trim's own `D_mean` is 52,147 and the ratio stays below 1.5.

**Gates.** `cargo test --locked --release -p builtins --features builtins/test-support --test
tail_contract`: 17 passed. `cargo test --locked -p math`: all passed (lib 8, including the new
test). `cargo test --locked -p builtins --features test-support --lib`: 15 passed. `cargo test
--locked -p builtins-compiler --features test-support --lib`: 57 passed. `cargo fmt --all --
--check`: clean. `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
and without `--all-features`: clean. `scripts/check-workspace-policy.sh`: ok.
`scripts/check-realtime-policy.sh`: ok. `scripts/check-cross-targets.sh`: PASS.

### Root's ruling (c) (2026-10-08, after the follow-up; supersedes the second fold-in's sampling)

Root's ruling, recorded as given. The verifier's descriptive calibration (load 2.15-2.33) gave
23.5 ns and a charge of 420 (binding: 96 kHz grid design 3238, 513 frames, median 31.68 us),
against the second fold-in's 21.5 ns and 470: the frame-equivalent is a maximum over about 45k
per-design medians of short (about 30 us) samples, so it moves about 10 % with the box's load.
**(c) A better-conditioned sample under the same median rule:** `calibrate` times each short
design class as a batch of N back-to-back walks, so that a sample lasts at least about 1 ms, and
takes the median over R >= 5 batches. **Condition:** a batch must not make the sample cheaper than
a real preparation, which walks each distinct design once (warm caches and branch predictors from
repeating one design would understate the cost), so each batch walks N different designs of the
same class, in the order a preparation would; a class that cannot supply N different designs shows
by measurement that the batch's per-walk cost is not below the single-walk cost of the same
designs, and records it. The fit and the joint search (the frame-equivalent on the 0.5 ns grid,
the charge from the largest per-section fixed cost) run on the per-batch medians, normalized per
design. "Every design" for the batched form: every batch's per-design median work satisfies the
inequality with its own frames and sections (F5). **(a) The figure of record** is the batch
verdict's single calibration and gate-2 invocation, run later by the batch verifier; the constants
are restated in the batch follow-up if it differs. Gate 2 (real preparations, charged against
measured) is the final check: a gate-2 family whose median exceeds the stated bound means the
constant is wrong. The second fold-in's sampling (one walk a sample) is superseded; its figures
stay above.

**Every run's value so far** (frame-equivalent / section charge):

| Run | Statistic | Value |
| --- | --- | --- |
| Fold-in 1 | every sample | 48.5 ns / 210 |
| Fold-in 2 | per-design median, one walk a sample | 21.5 ns / 470 |
| The verifier's descriptive run (load 2.15-2.33) | as fold-in 2 | 23.5 ns / 420 |
| This follow-up (below) | per-batch median, 48 different designs a batch | **17.0 ns / 550** |

### Attempt 1 follow-up 2: root's ruling (c) (2026-10-08, implementer)

**The sample** (`crates/builtins/examples/input_bound_budget.rs`). A calibration sample is a
batch: one preparation (`input_section_bounds_within`, a fresh cache, no budget limit) of **N = 48
different designs** of one class, walked back to back in strip order, as a real preparation walks
each distinct design once. The example asserts that every design of a batch is computed (none is
served from the cache), that the batch walks exactly the frames its designs walk alone, and that
every round charges the same. How the designs are chosen:

- *The fixed-cost grid* (four classes, every rate): the class's grid designs ordered by the frames
  each walks, cut into batches of 48 consecutive designs, the remainder folded into the last batch
  (48-95 designs), so a batch's designs are alike and the fit reads each batch at one length.
  Batches per rate: one cascade of one section 6 (288 designs), of two sections 70 (3,384), two
  cascades of one section 24 (1,152), of two sections 8 (420).
- *The short frame-class points* (typical and cheap, 513-24,913 frames a design): each point a
  batch of 48 designs next to it, its HPF (typical) or LPF (cheap) raised by 0.05 Hz per design,
  as gate 2's families step them.
- *The near-top points* (397,826-528,640 frames, about 5-7 ms a walk) are already over 1 ms:
  each is a batch of one, its own single walk.

960 batches, 27,144 designs, five measured samples each after one warmup. Each round also walks
every design of a multi-design batch alone, right after the batch (evidence only). The fit is the
least-squares line of each batch's median per design against its frames per design (batches whose
longest design walks at most 16,384 frames); the joint search reads each batch's median against
its own charged frame-equivalents. Two invocations did not complete and gave no figure: the first
aborted on the batch assertion (two near-top designs one `f32` step apart designed the same words,
so a batch computed 47 of 48), and the second, with the near-top points batched as 48 designs, was
stopped after about 10 minutes (a batch of 48 walks of over 5 ms each, about 22 minutes in all);
the near-top points then became batches of one, as above. Neither printed a frame-equivalent.

**Calibration** (one invocation, `taskset -c 7`, built with charge 470 (the sections counted do not
depend on it), one warmup and five measured rounds, no retry; 93 s; `/proc/loadavg` before
1.14 1.27 1.62, after 1.27 1.29 1.60; the wait for the 1-minute load below 2 was not a retry):

- Largest per-batch-median fixed cost per section **9.34 us** (one cascade of two sections, 88.2
  and 96 kHz; median-fit intercepts 18.69 and 18.68 us). The median fits' largest per section at
  each rate 8.37-9.34 us, their per-design terms -1.93 to -1.44 us. Each measured round's own fit:
  largest per section 8.35-9.68 us (the second fold-in's single walks: 8.65-14.17 us).
- **Frame-equivalent 17.0 ns, section charge 550** (9.34 us / 17.0 ns = 549.7, rounded up to a
  ten; 550 x 17.0 ns = 9.35 us). **Binding batch:** 48 kHz, one cascade of two sections, grid batch
  34 (48 designs): median work 44,723 ns a design, 1,537.0 frames + 550 x 2 sections = 2,637.0
  charged frame-equivalents a design, 16.960 ns per charged frame-equivalent.
- **Sample length.** The shortest batch sample is **0.584 ms** (44.1 kHz, one cascade of one
  section, batch 1: 48 designs of about 12 us each), below the ruling's "about 1 ms"; the binding
  class's batches last about 2.1 ms. The one-section batches are far from binding (at 17.0 ns their
  charge of 513 + 550 frame-equivalents is about 18 us against about 12 us measured). A batch of 96
  would give that class three batches a rate, too few for its fit; not changed in this run.
- **Raw maximum sample (evidence):** 19.104 ns per charged frame-equivalent, 1.468 x its batch's
  median (96 kHz, near-top, +24 dB, HPF 0.51: 8.69 ms against 5.92 ms); it is also the largest
  ratio of a raw sample to its batch's median.
- **Batch against single walks (evidence, the ruling's condition).** Every class supplies 48
  different designs, so the measurement is not required; it is recorded. A batch's median per walk
  over the sum of its designs' single-walk medians (the same rounds): **min 0.9842, median 1.0000,
  max 1.0367**; 133 of the 960 batches are below 1 (the least: 44.1 kHz, two cascades of two
  sections, batch 3, 53,798 ns a design batched against 54,663 ns alone). Per class over the rates:
  one cascade of one section 1.0010-1.0367; of two sections 0.9844-1.0255; two cascades of one
  section 1.0001-1.0194, of two sections 0.9842-1.0192; typical 0.9861-1.0225; cheap
  0.9917-1.0337; near-top 1 (single walks). So the batch's per-walk cost is not systematically
  below the single walk: it is within -1.6 % to +3.7 % of it, centred on 1. And the single walks
  give the same constant: the summed single-walk medians need at most **16.933 ns** per charged
  frame-equivalent at charge 550, which rounds up to the same 17.0 ns.
- Descriptive (net of the fixed cost): the slowest frame class's slowest point's median 16.65 ns a
  frame (cheap two-section, 1 kHz into 1.28 kHz, +24 dB, 88.2 kHz, spread 16.05-17.01).

**Gate 2** (one invocation on charge 550 and 17.0 ns, `taskset -c 7`, one warmup and five measured
rounds, no retry; 15 s; `/proc/loadavg` before 0.91 1.18 1.54, after 1.07 1.20 1.54): exit 0,
every family's three results identical in every round, every preparation within its budget in
frames, every warm rebuild walking nothing. The budget is 1,510,000 x 17.0 ns = **25.67 ms**.
Gate 2's medians need **16.5 ns** (largest median design work per consumed frame-equivalent
16.289 ns: 4,096 cheap two-section designs, 1 kHz into 1.28 kHz, +24 dB, 88.2 kHz), below 17.0 ns.
**Worst median design work 24.60 ms, 95.8 % of the budget** (the same family and rate; 807 of
4,096 designs exact). **No family's median is over the budget.** Medians per family over the four
rates: top pair 7.13-7.33 ms; LPF at the maximum 3.71-3.74; 64 near-top 20.85-21.22; 65,537
near-top 20.64-20.97; 64 band 20.28-20.40; 4,096 band 20.20-20.45; 64 typical 11.08-21.08 (64 of
64 exact); 256 typical 22.05-22.88; cheap one-section 17.99-18.08; cheap two-section 18.60-24.60;
cheap two-cascade 17.80-17.95. Largest-to-median ratio 1.000-1.054. **Raw samples (evidence):**
the largest raw design work is 24.83 ms, 96.7 % of the budget, 1.010 x its median (the same family
and rate; 16.445 ns per consumed frame-equivalent). The whole preparation of 65,537 strips took
61.20-62.64 ms (the O(strips) keying outside the budget, as in #1457). The margin is thin: the
cheap 1 kHz into 1.28 kHz family sits at 95.8 % of the budget at 88.2 kHz and 93.5 % at 96 kHz.

**Gate 8** (charge 550): passed, every 64-track console document exact at every rate. Stereo
documents (`console-sixty-four-track`, `-app`, `-intended`, `-sends`, identical; charged, frames
walked, margin): 44.1 kHz 751,232 (610,432, 758,768); 48 kHz 799,616 (658,816, 710,384); 88.2 kHz
1,298,560 (1,157,760, 211,440); 96 kHz 1,394,816 (1,254,016, **115,184**, the least). Mono:
386,880 (margin 1,123,120), 412,224 (1,097,776), 671,552 (838,448), 721,216 (788,784). Frames
walked unchanged.

**Restated.** `FRAME_EQUIVALENT_NS` = 17.0 (`examples/input_bound_budget.rs`, with the batched
sample in its module doc); `INPUT_BOUND_SECTION_CHARGE` = 550 and its doc;
`INPUT_BOUND_BUDGET_FRAMES`'s doc (25.67 ms); the cache cap's doc (a computed design charges at
least 257 + 550 = 807, so at most 1,510,000 / 807 = 1,871 designs a preparation; the cap of 8,192
holds them); the figures of #1468 (`:153-154`), #1470 (`:28-36`, `:50`), #1471 (`:21`) and the
#1474 note. The verifier NIT (59.06-61.16 ms, not 60.36-61.16 ms, in the second fold-in's gate 2)
is fixed above.
