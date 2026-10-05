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
  (#1338's 64 samples).
- **D2. Gains.** The node's floor is relative to its input peak, so the crossover residual is
  bounded at `eps / G_max`, with `G_max` the largest band amplitude (`gain_from_db` of the maximum
  makeup with no reduction) times the all-pass split's ℓ1 gain. The band gains are memoryless
  multipliers of the band signals, so they add no tail of their own.
- **D3. Rest.** The maximum of the crossover's certified rest (#1329 machinery, two sections in
  series) and each detector/smoother word's `one_pole_rest_samples` (#1375), with #1375 D3's stall
  check.
- **D4. Reuse.** Call #1329's SVF bounds in `lane::tail`; no copy.

## Deliverables

1. `tail_and_rest` for the effect, built on `lane::tail` (D4).
2. Tests (gates 1-3). Docs: `dsp-research/multirate-crossovers.md` (tail and rest lines).

## Authorized paths

- `crates/multiband-compressor/src/lib.rs` (descriptor and `tail_and_rest` only),
  `crates/multiband-compressor/tests/tail_contract.rs` (new)
- `dsp-research/multirate-crossovers.md`, this spec

## Non-goals

- Changing the crossover design, the curve or any rendered bit. Stream A owns payload code here.

## Objective gates

1. **Recompute**: brute-force `f64` impulse response of the two-section split (to 4,000,000
   samples) at the D1 extreme and at 80 Hz, 1 kHz and 8 kHz: `T_b <= T`, and at the extreme
   `T <= T_b + T / 100`.
2. **Soundness, real kernel** (release, every rate): extreme crossover, maximum makeup, worst-sign
   input, crossover prepared (#1338 adds the glide-in-flight case to this test when it makes the
   crossover live; D1's derivation already covers glides):
   `|y| < P * eps` from `N + T`; all state words `+0.0` from `N + R`, for `P = 10^(24/20)` and `1e29`.
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
- *Define how node tails compose through gain in the graph extent* (#1379)
- *Report a zero tail beyond latency for the compressor and the true-peak limiter* (#1375), for the
  one-pole helper
