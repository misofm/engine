# State exact-rest bounds for the gate, transient shaper and soft clip

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

The three effects that already declare a finite tail get it checked against the tail contract, and
each states when its whole state is exactly zero after silence. Silence skipping can then use all
three.

## Context

- **Gate/expander.** Hysteretic peak gate; latency 0, `Finite(0)`
  (`crates/gate-expander/src/lib.rs:237-244`). Output is a gain times the current input; the
  detector and gain state are one-pole recurrences with the `1e-20` flush
  (`crates/dsp-reference/src/gate_expander.rs:161`), plus integer hold counters.
- **Transient shaper.** Two switched attack/release one-pole followers (fast 0.5 ms / 20 ms, slow
  10 ms / 100 ms, `crates/transient-shaper/src/lib.rs:1-12`) through `ar_one_pole_step`
  (`crates/effect-runtime/src/envelope.rs:130-161`); latency 0, `Finite(0)` (`:147-152`).
- **Soft clip.** 2x oversampling, 63-tap Blackman half-band for interpolation and decimation, cubic
  `c(u) = u - u^3/3` clamped to `±2/3`, dry path delayed 31 samples, latency 31, tail 29
  (`crates/soft-clip/src/lib.rs:1-7`, `:174-179`). `c(0) = 0`, and `flush` guards the two history
  inputs.
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375) adds
  `effect_runtime::tail::one_pole_rest_samples` and the stall check this slice reuses.

## Decisions frozen for this slice

- **D1. Tails.** Gate and transient shaper: `Finite(0)`, `TailDecay(0)`. Soft clip: the tail is
  the number of samples after `N + latency` until the decimator's history holds only zeros, from
  the half-band length and the polyphase layout. Exact zero, so `TailDecay(0)`. The test recomputes
  it from the filter length and the latency; if the recomputation is not 29, the declared value
  changes and the commit says why.
- **D2. Rest.** #1329 D2's definition: output `±0.0` and every signal-state word equal under
  `f32` `==` to the rest state `Z`, which is not "all zero" and not always the reset state. The
  gate resets open (`gain_db = 0`, `open = OPEN_WORD`, `hold` loaded;
  `crates/gate-expander/src/lib.rs:504-515`) but with zero input it closes: its `Z` has the gain
  word at the range floor (`-range` dB), the hysteresis closed and the hold counter expired. The
  derivation proves the gate's gain smoother reaches `-range` exactly (not a stall one ulp away),
  so `Z` is unique; otherwise D3 applies. The transient shaper's `Z` is its reset state (followers
  at `+0.0`). Bounds: gate and transient shaper, the maximum over their one-pole words of
  `one_pole_rest_samples` (#1375) for the distance to `Z` at the slowest release in the domain,
  plus the hold counter's maximum, for #1329 D2's two peak cases. Soft clip: its histories are FIR
  delay lines and its ramps hold no input-driven state, so `Z` is the reset state and `R` is the
  number of samples after `N` until every history word is zero, from the history lengths; it is
  the same for both peak cases.
- **D3. Stall check** as in #1375 D3 for every one-pole word. A stall is reported to Sol as a class-B
  defect; no bound is stated for that effect.
- **D4. Gain** (#1379 D1: a peak gain and an incremental gain after silence). Gate: `0 dB`
  (`g <= 1`). Transient shaper: its maximum attack/sustain boost (output `g * x`). Soft clip: the
  output parameter's maximum plus the half-band's ℓ1 gain twice (interpolator and decimator) and
  the cubic's Lipschitz constant `sup |c'(u)| = 1`, mixed with the dry path's; the Lipschitz
  constant makes it an incremental gain, not only a peak gain.

## Deliverables

1. `tail_and_rest` bodies for the three effects.
2. Tests (gates 1-3). Docs: `dsp-research/dynamics.md` and `dsp-research/nonlinear-antialiasing.md`
   (tail and rest lines).

## Authorized paths

- `crates/gate-expander/src/lib.rs`, `crates/transient-shaper/src/lib.rs`, `crates/soft-clip/src/lib.rs`
  (descriptor and `tail_and_rest` only)
- `crates/gate-expander/tests/tail_contract.rs`, `crates/transient-shaper/tests/tail_contract.rs`,
  `crates/soft-clip/tests/tail_contract.rs` (new)
- `dsp-research/dynamics.md`, `dsp-research/nonlinear-antialiasing.md`, this spec

## Non-goals

- Changing any kernel, coefficient or declared latency.

## Objective gates

1. **Tail, real kernel**: for each effect at every rate, parameters at their domain extremes, 4,096
   samples of full-scale noise then zeros: output exactly zero from `latency + tail` after the last
   non-zero input, and (soft clip) non-zero at `latency + tail - 1` for some input, so 29 is the
   smallest.
2. **Rest, real kernel**: the same runs at `P = 10^(24/20)` and `P = 1e29` reach D2's rest by
   `latency + R`: output `±0.0`, and the state payload snapshot equals, word for word under `f32`
   `==`, that of `Z` (a freshly reset instance with the same parameters and settled ramps, fed the
   same number of zero samples), cursor words excepted and named by the test. Both instances then
   render the same 4,096 samples of seeded noise and agree under `==`. For the gate, a third
   instance fed only `R` zeros from reset must equal `Z` too, so the closed state is the same from
   every history.
3. **Recompute**: soft clip's tail from the filter length and latency in the test; gate and shaper
   rest from a brute-force `f32` iteration of each word toward its value in `Z` (with `B` the
   brute-force count: `B <= helper <= B + max(2, ceil(B / 100))`).
4. Commands: the `test-debug-b` command from `.github/workflows/qualification.yml`; the conformance
   fixtures check; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a soft-clip declared tail shorter than the decimator's flush, or a gate/shaper output term
  that survives silence, is red; nothing renders past these tails today.
- Gate 2: a stalled one-pole word, a gate gain that settles an ulp off `-range` (history-dependent
  rest), or a bound from the wrong release coefficient is red; an all-zero check could never pass
  on the gate.
- Gate 3: a stated value that no longer follows its designer (for example a half-band length change)
  is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Build the launch effect registry once per process and share it* (#1469): the shared registry
  the tail derivations run behind, once per process
- *Carry every node's tail bound in one node-neutral struct* (#1464) and *State a fixed input
  section's decay, gains and flush stall* (#1465): the carrier and the contract (#1379 Amendment 1,
  third round; D4 is restated to its H1: `D`, `G_p`, `G_t` and the two stalls `sigma_p` (every frame, (N1)) and
  `sigma_t` (from the node's tail on, (N2); #1484), each for every sidechain input).
- **Landing constraint (confirmed by root, 2026-10-06):** implemented after #1465, but lands on
  `main` only in the same batch as *Define how node tails compose through gain in the graph extent*
  (#1379), never before it: `main` must never report a plan tail that is not certified.
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  helper

## Evidence and decision record (attempt 1, 2026-10-24, on #1375's f20fd389b)

- **Scope note decisions taken (root).** Two tranches; the shared run-grouped walker in
  `effect_runtime::tail` serves three entry points; `FAST_GAIN_ERROR_DB = 1e-5` re-declared per
  crate with the F1 citation; the soft clip's D4 includes the drive; the soft clip's `R` counts the
  32-row ring; the gate's `Z` is the stall word (D2 amendment below). Paths beyond the spec list,
  from the scope note: `crates/effect-runtime/src/tail.rs`, `crates/transient-shaper/src/corpus.rs`
  (the metadata literal now reads `tail_and_rest`), `.github/workflows/qualification.yml` (one line).
- **Helper.** #1375 shipped `rate_follower_rest_samples`, not D2's `one_pole_rest_samples`. The
  walker is factored into `magnitude_walk` over an exact weight `P / 2^Q`; the rate form passes
  `c (1 - u)`, the new `pole_follower_rest_samples` passes `1 - c` (exact, Sterbenz). No value of
  the compressor or limiter moved.
- **Transient shaper (D1, D2, D4).** Tail `Finite(0)` confirmed by reading (`select(identity, x,
  gain_mix_step(x, g, mix))` of the current input). With a zero detector each follower is
  `snap(fl(c e))` at its release retention, snapped `Below(FOLLOWER_REST = 1e-12)`; `Z` is the reset
  state. Distance: the input peak `U` inflated to `U (1 + 3 u / k_min)` (a convex-step invariant
  proved in the doc comment). D3: slowest retention `1 - 1.04e-4`. Stated `R` (+24 dBFS / any
  sanitized input) at 44.1 / 48 / 88.2 / 96 kHz: 134,039 / 426,481; 145,892 / 464,191;
  268,101 / 853,016; 291,723 / 928,173, each equal to the brute force of the slow word. Gate 2 in
  release, three link modes and a ramp in flight: rest at most 904,192 against 928,173 (96 kHz).
  `G_p = G_t = 1,801` mB (`10^((18 + 1e-5)/20) (1 + u)^4`: the scope note's `(1 + u)^3` undercounts
  `gain_mix_step`'s four roundings); `sigma = 4 UNDERFLOW (1 + u)^3 = -74,655` mB. Measured largest
  `|y| / max|x|` 7.9411 against 7.9433.
- **Soft clip (D1, D2, D4).** `T = max(2 * 30, 31) - 31 = 29` stands. `R = 30 + 32 - 31 = 31`: the
  last non-zero `e` push is 30 samples after the input stops and the 32-row ring clears 32 pushes
  later; gate 2 judges it with `lanes_at_rest` on a bank and the payload against `Z`
  (`tail_every_peak = 31`). `G_p = G_t = 7,025` mB: `g_out max(1, 2 g_drive (l1e^2 + 1/4))
  (1 + u)^129` at the +36 dB drive and +24 dB output words (the spec's D4 omitted the drive; the
  centre tap is carried exactly instead of folding it into `l1(h)^2`); `sigma = 129 UNDERFLOW g_out
  (l1e + 1/2)^2 (1 + u)^129 = -68,344` mB. Measured largest `|y| / max|x|` 16.99 against 2,243.
- **Gate D2 amendment (root, scope note).** The spec's premise that the gain smoother reaches
  `-range` is false: `flush(fma(c, t - e, e))` stops at the first word whose step rounds away, the
  stall word `w*` (measured 0.74 dB short of a 96 dB range at 96 kHz). `Z = (gain = w*, open = +0.0,
  hold = +0.0)`. `w*` is the same for every start in `[w*, 0]` (every history with fixed
  parameters): near the end each step moves one word and no step from farther away lands past the
  first stall (derivation in `rate_follower_stall_samples`; gate 2's three histories and the helper
  test's three starts per target). **Recorded limit:** an edit (range, ratio, threshold, a ramp in
  flight) that leaves the word below `w*` rests on its own fixed point, which `lanes_at_rest`
  accepts but which is not word-equal to `Z`. **Owner question (not blocking, class B):** snap the
  gain word to its target within a stated epsilon, so `Z` is unique for every history and the gate
  lands on `-range`.
- **Gate bound.** `rate_follower_stall_samples` bounds the walk from `+0.0` for every target and
  every release coefficient: majorant integer walks per target class (B: a target just above a
  power of two, whose walk ends on the finer grid; M: mid; A: high in its binade), coefficient
  brackets of 2 % above the slowest, and a coarse bound above them. `c_min` is the 2,000 ms design
  less `2^-24` (`expf` is within one ulp, and every release coefficient is a multiple of `2^-24`).
  `R = H_max + K` (`H_max` = 1,000 ms, above the 64-sample ramp): 44.1 / 48 / 88.2 / 96 kHz:
  562,206 (walk 518,106, longest brute force 516,751), 602,859 (554,859 / 553,347), 1,006,672
  (918,472 / 914,783), 1,082,844 (986,844 / 982,616): within 0.43 %. The class B target is the
  maximum: its stall word is `-32.0` exactly in gate 2. Cost: 1.6-3.3 ms per rate in release.
  `G_p = G_t = 1` mB (`10^(1e-5/20) (1 + u)`), `sigma = UNDERFLOW = -75,859` mB, for every
  sidechain input.
- **No rendered bit moved.** No kernel, coefficient or latency changed; the corpus metadata reads
  `tail_and_rest`.
- **Tests.** One `tests/tail_contract.rs` per effect (gates 1+3, 2, 4; the soft clip folds the tail
  and rest recompute into gate 1); helper tests in `effect_runtime::tail` (pole bound, pole stall,
  stall bound and uniqueness at faster coefficients). #1784's `rest_tests.rs` stay. The three crates
  join #1375's release `tail_contract` step in `qualification.yml`.
