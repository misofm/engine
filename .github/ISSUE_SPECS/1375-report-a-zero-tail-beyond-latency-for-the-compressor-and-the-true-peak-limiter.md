# Report a zero tail beyond latency for the compressor and the true-peak limiter

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan whose only recursive effects are compressors and true-peak limiters reports a finite tail:
both effects multiply a (delayed) input by a gain, so their output is exactly zero once the input
has been zero for their latency. Both also state when their detector and gain state reach exact
rest. Today both declare `Infinite`, which makes every plan that holds one report `Infinite`.

## Context

- **Compressor.** Feed-forward peak compressor; zero latency (`crates/compressor/src/lib.rs:291-296`,
  `tail: Infinite`). Its frozen identities include `mix == 0` and `mix == 1` (`:25-30` of the crate
  doc); the gain path is `effect_runtime::envelope::rms_follow` and `lane::flush` on `g` (crate doc
  table, `:11-22`). Output is a gain-and-mix of the current input sample (`gain_mix_block` form).
- **Limiter.** `y[n] = x[n - T] * g[n]`, `T = N + 6`, `N = Fs / 100`
  (`crates/true-peak-limiter/src/lib.rs:11-24`), declared latency `lookahead_maximum + 6`
  (`:235-243`, `tail: Infinite`). The gain recurrence is
  `d[n] = max(1 - s[n], fma(c, (1 - s[n]) - d[n-1], d[n-1]))` (`:21`): the `e + c * (u - e)` form.
- **One-pole followers.** `ar_one_pole_step` is `e' = flush(c * e + k * u)`
  (`crates/effect-runtime/src/envelope.rs:130-161`); its doc records that the `e + k * (u - e)` form
  stalls in `f32` when `|u - e| < ulp(e) / (2k)`. With `u = 0` a stall above `FLUSH_EPS` would be a
  state that never rests.
- The home for the values is `tail_and_rest` (#1377); the contract and `RestSamples` are #1329's;
  the composition values (`D`, `G_p`, `G_t`, `sigma_p`, `sigma_t`) are #1379 Amendment 1's H1
  contract (#1484 split the stall into the peak stall `sigma_p`, every frame, (N1), and the tail stall
  `sigma_t`, from the node's tail on, (N2)), carried
  by `CompositionBound::Stated` in `NodeTailBound` (#1464; contract text in
  `docs/handoffs/decision-15-2026-10-05/1379-amendment1-design.md`, H1).
- **Root ruling (2026-10-06, #1379 Amendment 1, third round).** This slice needs only the carrier
  and the contract from #1379, not the composed graph extent: no gate or deliverable reads
  `output_tail`, and the graph-compiler test it edits checks the effect entry's own
  `metadata.tail`. Its dependency on #1379 narrows to #1464 and #1465, and D4 is restated to H1.
  It lands on `main` in the batch with #1379 (slice C), never before it: before C the extent ignores
  the compressor's makeup gain, so a newly finite plan tail would not yet be composed through gain.
- Round-1 evidence (decision 15): "the compressor and limiter only multiply a delayed input, the
  argument the transient shaper used for `Finite(0)`".

## Decisions frozen for this slice

- **D1. Tail.** Both report `TailSamples::Finite(0)` and `TailDecay(0)` at every rate: output is
  exactly zero (either sign) from `N + latency` on. The implementer confirms
  by reading each kernel that no output term other than `gain * delayed input` exists (dry and wet
  of the mix both read the same delayed input; a sidechain only feeds the detector). If any other
  term exists, the slice stops and reports it.
- **D2. Rest.** `RestBound::Bounded`, with #1329 D2's definition: output `±0.0` and every
  signal-state word equal under `f32` `==` to the rest state `Z`. Rest is not "all zero". For both
  effects `Z` is the reset state (with zero input the reset state is a fixed point: no reduction is
  required). The limiter's reset state is the one its `clear_runtime` writes
  (`crates/true-peak-limiter/src/lib.rs:624-643`): `history`, `main_ring` and `reduction` at
  `0.0`, `required_ring`, `box_ring` and `prefix` at `1.0`, `box_sum` at `Wb`; `phase` is a cursor,
  not signal state. With zero input every required gain is `1.0`, so each ring returns to `1.0`
  after its length, and `box_sum` returns to `Wb` exactly because it is an exact running sum on the
  `2^-14` grid (crate doc, `:19`). The compressor's detector and gain words return to their reset
  values. The bound is the maximum over words: a ring or window word rests after its length plus
  the latency; each one-pole word rests after `ceil(ln(E_max / FLUSH_EPS) / -ln(c_eff))` samples
  (distance to its rest value, `E_max` the largest reachable distance for the two peak cases of
  #1329 D2, `c_eff` the slowest coefficient over the parameter domain at that rate, inflated by
  `f32` rounding). The limiter's `d` (`fma(c, (1 - s) - d, d)`) rests at `+0.0` by its flush once
  `s == 1` exactly. A shared helper `effect_runtime::tail::one_pole_rest_samples(c_max, e_max) -> u64`
  holds the formula for #1373, #1374 and #1376.
- **D3. Stall check.** For every one-pole word, prove (in the doc comment of the helper's caller) that
  no `f32` fixed point exists above `FLUSH_EPS` with zero input at the slowest coefficient. For the
  two-product form the condition is `c < 1 - 2^-24`; for the `e + c (u - e)` form it is
  `c > 2^-24`, scale-free. If a word fails, stop and report it to Sol as a class-B defect, as in
  *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328); do not state a bound.
- **D4. Composition values** (restated 2026-10-06 to #1379 Amendment 1, H1). Both state
  `CompositionBound::Stated` with `decay` `TailDecay(0)` (D1). `peak_gain = tail_gain`: for the
  compressor, `max(makeup_max, 1)` over the parameter domain (its output is
  `(mix g + 1 - mix) x` with `0 <= g <= makeup`), and for the limiter `0 dB` (`g <= 1`), each
  times `(1 + u)` per rounded product on the output's chain (`u = 2^-24`), rounded up to millibels;
  after silence the output is `g x` with the same bound, so `G_t = G_p`. `sigma_p` and `sigma_t`, both from
  one value: the underflow allowance of the output's chain (`2^-126` per rounded operation, rounded
  up into `FlushStall::Level`), as #1379 H3 states for gain-only nodes, so `sigma_t = sigma_p`. Each value holds for every sidechain
  input (H1's quantifier): a sidechain feeds only the detector, so `g <= makeup` and `g <= 1` hold
  whatever it carries.

## Deliverables

1. `tail_and_rest` bodies for both effects; `effect_runtime::tail` (new module) with the helper.
2. Tests (gates 1-3). Docs: `dsp-research/dynamics.md` and `dsp-research/true-peak.md` (tail and rest lines).

## Authorized paths

- `crates/compressor/src/lib.rs`, `crates/true-peak-limiter/src/lib.rs` (descriptor and
  `tail_and_rest` only), `crates/effect-runtime/src/tail.rs` (new), `crates/effect-runtime/src/lib.rs`
  (module line)
- `crates/compressor/tests/tail_contract.rs`, `crates/true-peak-limiter/tests/tail_contract.rs` (new)
- The limiter unit test that asserts an `Infinite` tail (`crates/true-peak-limiter/src/lib.rs:5124`,
  moved onto `tail_and_rest` by #1377) and the limiter fixture's `Infinite` assertion in `crates/graph-compiler/src/lib.rs:12321`,
  rewritten to `Finite(0)`
- `dsp-research/dynamics.md`, `dsp-research/true-peak.md`, this spec

## Non-goals

- Other effects (#1372-#1374, #1376). Any rendered-bit change.

## Objective gates

1. **Zero tail, real kernel**: for each effect, at every rate, parameters at the domain extremes
   (fastest/slowest attack and release, maximum makeup, `mix` 0, 0.5, 1, ratio maximum, limiter
   lookahead 0 and maximum), feed 4,096 samples of full-scale noise then zeros: every output sample
   from `latency` after the last non-zero input is exactly zero.
2. **Rest, real kernel**: the same runs, at `P = 10^(24/20)` and `P = 1e29`, reach D2's rest by
   `latency + R`, `R` the stated bound: from there every output is `±0.0`, and the state payload
   snapshot equals, word for word under `f32` `==`, the snapshot of `Z`: a freshly reset instance
   with the same parameters and settled ramps, fed the same number of zero samples, except the
   cursor words the test names (`phase`, ring write positions). As a cursor-independent check, both instances then render the same 4,096
   samples of seeded noise and their outputs agree under `==`.
3. **Composition** (both effects, every rate): the stated `peak_gain` equals D4's formula
   recomputed in the test, and over gate 1's runs (sidechain connected with full-scale noise, and
   unconnected) every output sample satisfies `|y| <= g_p max|x| + sigma_p` (N1).
4. **Recompute**: the helper's value for each word is checked against a brute-force `f32`
   iteration of that word's recurrence from `E_max` with zero input, counted until it equals its
   value in `Z` (with `B` that count:
   `B <= helper <= B + max(2, ceil(B / 100))`; the two-sample floor covers the `ceil` and rounding
   of short fast-coefficient runs).
5. Commands: the `test-debug-b` command from `.github/workflows/qualification.yml`; the conformance
   fixtures check; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a mix or makeup path that leaks a delayed or filtered term past latency is red; no test
  checks output beyond latency after silence.
- Gate 2: a state word that stalls above the flush (D3), a bound computed from the wrong
  coefficient, or a limiter box sum that drifts off `Wb` is red; an all-zero rest check could not
  pass on the limiter at all.
- Gate 3: a gain stated from `makeup` alone that ignores the dry term of `mix` (below 0 dB makeup the
  dry path dominates), or a gain that depends on the sidechain, is red; nothing else checks the
  composition values.
- Gate 4: a helper formula off by the rounding inflation under-reports and is red.

## Dependencies

- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Build the launch effect registry once per process and share it* (#1469): the shared registry
  the tail derivations run behind, once per process
- *Carry every node's tail bound in one node-neutral struct* (#1464): the carrier.
- *State a fixed input section's decay, gains and flush stall* (#1465): the contract text and the
  derivation note it starts.
- **Landing constraint (confirmed by root, 2026-10-06):** implemented after #1465, but lands on
  `main` only in the same batch as *Define how node tails compose through gain in the graph extent*
  (#1379), never before it: before #1379 a finite effect tail would be composed without the
  compressor's makeup gain, and `main` must never report a plan tail that is not certified.
