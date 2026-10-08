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
  satisfies `P* <= P^ < 2 P*`. Nothing #1329 certified changes.
- **`dev_core` and `dev_loud`.** `dev_core` is #1329's deviation supremum from `T_decay` on; it
  feeds `P*`. `dev_loud` is the output deviation while the input is loud (`Deviation::at_end`'s
  fixed point), per unit of `|trim| P`; it bounds the deviation at every frame. `G_p` uses
  `dev_loud`, never `dev_core`.

## Decisions frozen for this slice

- **F-D1. The contract** is H1 above: (N1), (N2) with the share `3/4`, (N3).
- **F-D2. `D` by the binding construction.** Compute the certified crossing `T(k)` of the floor
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
- `docs/derivations/1379-graph-tail-composition.md` (new),
  `docs/derivations/1329-input-section-tail-and-rest.md` (a pointer to the new note),
  `docs/EFFECT_CONTRACT_V1.md` (the fixed section's statement)
- This spec, and its row in `docs/handoffs/decision-15-2026-10-05/STREAMS.md`

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
  calibrate` and restates the frame-equivalent (the slowest frame class) and
  `builtins::INPUT_BOUND_SECTION_CHARGE` from it (#1457 Amendments 3 and 4; the per-design charge
  was removed in Amendment 4). The restated slowest class is a robust statistic, never the maximum
  of single-shot samples (#1457 attempt 3 verdict n3): each point's ns per frame is the median of
  repeated runs within the one invocation, stated with its spread (minimum to maximum), and
  confirmed against gate 2's band families (their design work divided by their charged
  frame-equivalents).
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
  exact suffix, 35,731 samples against a mean of about 45,500 at the 44.1 kHz top pair, which falls
  about 52,000 samples short of the brute force by `k = 8`) is red; nothing tests a decade law
  today.
- F1(b): a needlessly loose `D` from a certificate rate halfway between the floor rate and 1
  (`lambda' = (1 + max_s (q_s + mu_s)) / 2`, so `D_inf` is about 2 `D_floor`) exceeds the 1.5 line
  on every row, whatever certificate the implementation builds.
- F1(c): a `D` with no transient term (the asymptotic rate alone) falls below the module's own
  crossing at `k = 1`, where the module's majorant takes its longest step (the polynomial factor of
  near-equal poles; steps 48,531, 48,136, 47,816, ... at the top pair); the brute force cannot see
  it, because the certified `T`'s slack absorbs it.
- F1(d): a certificate whose rate omits the `f32` state rounding (`lambda` from `q_s` alone, about
  2 % of the decade length at the top of the domain) is below the floor; F1(a) and F1(c) cannot see
  it, because the per-decade transient allowance absorbs it.
- F2(a): a gain that drops the trim, or the cascade's second section (10 Hz HPF into the top LPF:
  `l1` 3.51 against the HPF's alone), falls below the brute-force `l1`.
- F2(b): a gain from the two sections' state norms (`||c|| ||b|| / (1 - q) + |d|` each) exceeds the
  line by about 1.07 dB at the top pair and about 1.06 dB at the HPF at 10 Hz into the LPF one `f32`
  above 10 Hz.
- F2(c): a trim-only section that states 0 dB for a non-power-of-two trim (the rounding dropped),
  drops the trim, or states `D > 0`.
- F2(d): a `G_p` built on `dev_core` (#1329's deviation from `T_decay` on) instead of `dev_loud` is
  about 59 mB low at the top pair and fails the equality; a `CascadeBound` that returns `dev_core`
  as `dev_loud` falls below the recomputation's lower edge. F2(a) has about 14 dB of slack at the
  top pair and F2(b) is an upper line, so neither can see it.
- F3: a stall of zero, or one rounded down, gives `P^` below the module's `P*`.
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
