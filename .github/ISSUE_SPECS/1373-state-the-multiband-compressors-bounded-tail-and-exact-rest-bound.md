# State the multiband compressor's bounded tail and exact-rest bound

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan with a multiband compressor reports a finite tail at its rate, and the effect states when
its crossover and detector state reach exact rest. Today it declares `Infinite`.

## Context

- **Signal path.** Per channel, per track: `(v1, lp1) = svf(x)`, `ap = x - 2k v1`,
  `(_, low) = svf(lp1)`, `high = ap - low`, `k = sqrt(2)` (crate doc,
  `crates/multiband-compressor/src/lib.rs:1-21`; `lr4_step`, `:503-508`). Each band is multiplied by
  a gain from a Giannoulis-Massberg-Reiss curve, a branching smoother and makeup
  (`band amplitude`, `:785-793`), and the two gained bands are summed. Latency 0.
- **Design.** `design_lr4` (`:519-561`), crossover 80 Hz to 8 kHz (`:176-189`). Prepared-only today;
  *Make the multiband compressor's crossover live* (#1338) makes it live with a 64-sample glide.
  Makeup parameters per band (`low_makeup` `:245`, `high_makeup` `:310`).
- **Declared tail.** `Infinite` (`:343-349`), moved into `tail_and_rest` by #1377.
- **Evidence (decision 15, round 2 B4(a)).** With #1328's joint flush, the LR4 crossover's slowest
  exact rest fell from 517k to 28k samples.
- Both crossover sections use `k = sqrt(2)`, the same fixed-`k` family as the builtin input section,
  so #1329 D5's shared-eigenvector argument applies to crossover edits as it stands.

## Decisions frozen for this slice

- **D1. Contract and method** as in *State a bounded tail and an exact-rest bound for every node*
  (#1329) D1-D3, over the whole crossover domain at each rate, with a crossover glide in flight
  (#1338's 64 samples). This slice owns the glide-in-flight case: #1338 D7 leaves the tail to
  stream G, so the derivation, the bound and gate 2's glide run are all here, and this slice lands
  after #1338.
- **D2. Gains.** The node's floor is relative to its input peak, so the crossover residual is
  bounded at `eps / G_max`, with `G_max` the largest band amplitude (`gain_from_db` of the maximum
  makeup with no reduction) times the all-pass split's ℓ1 gain. The band gains are memoryless
  multipliers of the band signals, so they add no tail of their own.
- **D3. Rest.** #1329 D2's definition: output `±0.0` and every signal-state word (the crossover's
  integrator words, each band's detector and its dB smoother) equal under `f32` `==` to the rest
  state `Z` (a freshly reset instance with the same parameters, fed `R` zeros). The bound is the maximum of the crossover's certified
  rest (#1329 machinery, two sections in series) and each detector/smoother word's
  `one_pole_rest_samples` (#1375) toward its value in `Z`, with #1375 D3's stall check.
- **D4. Reuse.** Call #1329's SVF bounds in `math::tail` (`crates/math/src/tail.rs`); no copy.

## Deliverables

1. `tail_and_rest` for the effect, built on `math::tail` (D4).
2. Tests (gates 1-3). Docs: `dsp-research/multirate-crossovers.md` (tail and rest lines).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (descriptor and `tail_and_rest` only),
  `crates/multiband-compressor/tests/tail_contract.rs` (new)
- `docs/derivations/1373-multiband-compressor-tail-and-rest.md` (new): the glide-in-flight bound
- `crates/graph-compiler/src/lib.rs` (the multiband fixture's `Infinite` assertion at `:12735`,
  rewritten to the computed value)
- `dsp-research/multirate-crossovers.md`, this spec

## Non-goals

- Changing the crossover design, the curve or any rendered bit. Stream A owns payload code here.

## Objective gates

1. **Recompute**: for each crossover in {the D1 extreme, 80 Hz, 1 kHz, 8 kHz}, a brute force
   computes the `f64` impulse responses `h_low`, `h_high` of the two-section split (designed `f32`
   words, `f64` arithmetic) to 4,000,000 samples and
   `S_b(j) = G_max * sum_{j <= m < 4e6} (|h_low[m]| + |h_high[m]|)`, `G_max` from D2; `T_b(e)` is
   the smallest `j` with `S_b(j) < e`. Soundness at every crossover: `T_b(eps / 2) <= T`.
   Tightness at the extreme only: `T <= T_b(eps * 2^-5)`, the same 30 dB margin as #1329 gate 1
   and for the same reasons (fixed `k = sqrt(2)`, so the shared-eigenvector bound applies); a
   value above it is a finding for Sol, not a gate change.
2. **Soundness, real kernel** (release, every rate): extreme crossover, maximum makeup, worst-sign
   input (`P * sign` of the reversed `h_low + h_high` over the last 1,000,000 samples before `N`),
   two runs: crossover prepared, and a crossover glide from 8 kHz to the extreme started 32 samples
   before `N` (in flight at `N`). `|y| < P * eps` from `N + T`; D3's rest from `N + R`, for
   `P = 10^(24/20)` and `1e29`.
3. **Rest recompute** for the detector words, as #1375 gate 3.
4. Commands: the `test-debug-b` command; the conformance fixtures check;
   `cargo clippy --locked --workspace --all-targets -- -D warnings`; `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail that forgets the second section or the makeup gain disagrees with brute force.
- Gate 2: a bound that ignores the glide, `f32` rounding or the flush stall is violated by the kernel.
- Gate 3: a stalled or mis-bounded detector word is red.

## Dependencies

- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Build the launch effect registry once per process and share it* (#1469): the shared registry
  the tail derivations run behind, once per process
- *Define how node tails compose through gain in the graph extent* (#1379)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  one-pole helper
- *Make the multiband compressor's crossover live* (#1338): the glide this slice bounds
