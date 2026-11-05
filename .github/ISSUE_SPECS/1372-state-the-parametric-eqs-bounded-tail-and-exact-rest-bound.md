# State the parametric EQ's bounded tail and exact-rest bound

Stream G of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-4(b)).
Code anchors verified on `main` at `6fb211594`.

## Product outcome

A plan with a parametric EQ, as an insert or a console slot, reports a finite tail at its rate. The
EQ also states when its cascade reaches exact rest. Today it declares `Infinite`, so every console
with an EQ slot reports `Infinite`.

## Context

- **The cascade.** Six TPT SVF sections per channel: HPF, four bands, LPF (`EQ_SECTION_COUNT = 6`,
  `crates/parametric-eq/src/lib.rs:77-84`). Band domain: frequency 10 Hz to 20 kHz, gain ±24 dB,
  Q 0.1 to 18, shelf slope 0.1 to 1 (`MINIMA`/`MAXIMA`, `:471-478`); kinds bell, low/high shelf,
  low/high pass, notch (`EqBandKind`, `:275-288`). Design in `f64`, one rounding to `f32`:
  `design_svf_words_f64` and `design_svf` (`:747`, `:853`). Unlike the builtin input section, `k`
  varies with Q, gain and slope.
- **Live edits.** Block-rate parameters ramp the words over `RAMP_SAMPLES = 64` (`:117`).
  *Make a parametric EQ band's enabled and kind live* (#1337) adds live kind and enable.
- **Declared tail.** `Infinite` at every rate (`:648-653`), moved into `tail_and_rest` by
  *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377).
- **Evidence (decision 15, round 2 B4(a)).** With the joint flush of #1328, the EQ's slowest exact
  rest is about 89 s at every rate for a +24 dBFS peak (bell 10 Hz, Q 18, +24 dB). Its -144 dB tail
  has not been computed. Every SVF step matrix is non-expansive in the 2-norm (norm exactly 1), so
  bounding the state under arbitrary earlier edits needs a strict-decay argument.
- *State a bounded tail and an exact-rest bound for every node* (#1329) builds the certified
  machinery for one fixed-`k` section family; this slice extends it to varying `k` and six sections.

## Decisions frozen for this slice

- **D1. Same contract and method as #1329 D1-D3**: certified `T` (exact tail sum below `eps / 2`,
  `f32` deviation below the other half), certified `R` for the two peak cases, `TailDecay` and
  `PeakGain` (#1379), all from the designer, per rate, over the whole parameter domain, on the
  control thread inside `tail_and_rest`.
- **D2. Varying `k`.** The shared-eigenvector argument of #1329 D5 does not apply. The derivation
  (`docs/derivations/1372-parametric-eq-tail-and-rest.md`) must give a common quadratic Lyapunov
  function (a matrix `Q` with `A^T Q A <= q^2 Q`, `q < 1`) over every designed step and every
  64-sample ramp step in the domain at each rate, or another certified strict-decay bound, and
  identify the domain extreme production evaluates. A ramp may run between *any* two designed
  points (an edit jumps anywhere in the domain), not only neighbours. The derivation covers this
  by convexity: the SVF step matrix is affine in the stored words (`d1 = -c1*ic1 + a2*v3`,
  `d2 = a3*v3 + a2*ic1`), so `||Q^(1/2) A(w) Q^(-1/2)||_2 <= q` is a convex constraint in the word
  vector `w`; it holds on every interpolated ramp word once it holds on every designed word, and
  the `f32` rounding of an interpolated word is absorbed by the D1 rate inflation. The note proves
  the affine form and states a Lipschitz constant of the designer in each parameter, so a check on
  a finite grid plus that margin covers the continuous domain. If no such bound exists with a
  finite value, the slice stops and reports to Sol; there is no fallback value.
- **D3. Cascade.** Six sections compose by the union bound of #1329 D3, generalized to `k` terms
  (`a_1 + ... + a_k >= H` implies some `a_i >= H / k`).
- **D3a. Rest** is #1329 D2's definition: output `±0.0` and the twelve SVF integrator words per
  channel equal to a freshly reset EQ's (all `+0.0`) under `f32` `==`, with #1328's joint flush.
- **D4. Expected magnitude.** `R` near 89 s (about 8.5M samples at 96 kHz) and `T` shorter. Hosts cap
  tails at `u64::MAX` (`crates/host-core/src/prepare.rs:1559`); `graph_fixture`'s 10,000,000 cap
  holds them.

## Deliverables

1. `tail_and_rest` for the EQ, with its computation in `crates/parametric-eq/src/tail.rs` (new),
   built on #1329's `math::tail` section bounds (`crates/math/src/tail.rs`, extended there for
   varying `k` if needed).
2. The derivation note; `dsp-research/filters.md` "Latency and tail".
3. Tests (gates 1-3).

## Authorized paths

- `crates/parametric-eq/src/tail.rs` (new), `crates/parametric-eq/src/lib.rs` (descriptor,
  `tail_and_rest` and module line only), `crates/parametric-eq/tests/tail_contract.rs` (new)
- `crates/math/src/tail.rs` (varying-`k` extension only)
- `docs/derivations/1372-parametric-eq-tail-and-rest.md` (new), `dsp-research/filters.md`, this spec
- Tests that assert the EQ's `Infinite` tail, each rewritten to the computed value with the reason:
  `crates/parametric-eq/tests/bank.rs:203` (`program_key.tail`) and
  `crates/host-core/tests/live_lanes.rs:202-211` (the EQ fixture's lanes-free tail). The
  `graph-compiler` fixtures at `crates/graph-compiler/src/lib.rs:12321` and `:12735` are the
  limiter's and the multiband compressor's, not the EQ's; #1375 and #1373 own them.
- Canonical plan fixtures whose graph holds an EQ, if any moves: `fixtures/graph/v1/` and the
  digests that pin them in `crates/graph-compiler/tests/`, re-pinned one at a time

## Non-goals

- Any kernel or designer change. Stream A owns payload code in the same crate.

## Objective gates

1. **Recompute**: for each design, a brute force computes the `f64` cascade impulse response `h`
   (designed `f32` words, `f64` arithmetic) to 12,000,000 samples and `S_b(j) = sum_{j <= m < 1.2e7} |h[m]|`;
   `T_b(e)` is the smallest `j` with `S_b(j) < e`. Points: the D2 extreme and 24 single-band points
   (each kind at 10 Hz and 20 kHz, Q 0.1 and 18, gain +24 dB, slope 0.1). Soundness at every point:
   `T_b(eps / 2) <= T` (`T` is one value over the domain, #1377 D5). Tightness at the extreme only:
   `T <= 2 * T_b(eps / 2)`. The factor 2 is the slack a domain-wide `q` and the condition number of
   `Q` may cost against the extreme's own pole; a certified `T` above it is a finding for Sol with
   the measured ratio, not a gate change. Release at every rate; debug at 48 kHz only.
2. **Domain scan** (closed form, no rendering), checking the derivation's claim
   `||Q^(1/2) A(w) Q^(-1/2)||_2 <= q - m` (`m` the stated Lipschitz margin): (a) on a grid of kind ×
   frequency (100 log-spaced) × Q (20 log-spaced) × gain (13) × slope (5); (b) on 100,000 seeded
   uniform random domain points; (c) on the interpolated words at steps 1, 32 and 63 of a 64-sample
   ramp between 100,000 seeded random pairs of domain points of any kinds, far apart included.
   (c) is the regression check of D2's convexity argument, not its proof.
3. **Soundness, real kernel** (release): the extreme with a worst-sign input and a live target 32
   samples before `N`: `|y| < P * eps` from `N + T`; from `N + R` on, D3a's rest (output `±0.0`,
   integrator words equal to a freshly reset EQ's under `==`), for `P = 10^(24/20)` and `1e29`.
4. Commands: the `test-debug-b` command; `cargo test --locked --release -p parametric-eq --features parametric-eq/test-support --test tail_contract`;
   the `test-debug-a` command; `bash scripts/check-graph-determinism.sh`;
   `bash scripts/check-dsp-research.sh`; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
   `cargo fmt --all -- --check`.

## Test value

- Gate 1: a tail computed from the wrong section or ignoring the cascade disagrees with brute force.
- Gate 2: a Lyapunov constant fitted on grid points only, which an off-grid design or a ramp
  between distant points violates, is red.
- Gate 3: a bound that ignores `f32` rounding, the ramp in flight or the flush stall is violated by
  the production kernel.

## Dependencies

- *Flush the SVF jointly so builtin and EQ filters reach exact rest* (#1328)
- *State a bounded tail and an exact-rest bound for every node* (#1329)
- *Carry each effect's tail and exact-rest bound in its prepared metadata* (#1377)
- *Define how node tails compose through gain in the graph extent* (#1379)

## 1372a record (attempt 1, stopped under R4)

Tranche #1372a (the word-hull Lyapunov cascade in `math::tail`) stopped before code, under root
decision R4: the identity-vertex argument does not close, so no uniform hull bound is stated.

- **The gap R4 names (flush).** With a cross term `Q12 = eps`, a per-word flush can raise
  `||s||_Q^2` by up to `2 eps |n_i| |n_j|`. Near the identity the decay per frame is
  `delta (1 - q)` (`delta` the design weight of the affine split), so it does not pay for a flush
  event that the input re-arms frame after frame. The crude charge (bump against the increments
  `|2 d_i| <= 2 delta C Z` since the last flush) needs `4 eps C kappa_Q < 1 - q`, and with
  `C ~ 2 g`, `1 - q ~ 2 eps g` (scope F2) that fails by a factor of about `4 kappa_Q`.
- **A second gap the scope note does not list (final rounding).** The kernel's last step
  `n = fl(ic + 2 d)` errs by up to `u |n|` per word (the `KAPPA * U * q` term of
  `state_rounding`). That error does not scale with `delta`: it is `min(u |n|, |2 d|)`, and
  `|2 d| >> u |n|` in the zone. So at a chord word `w = (1 - delta) I + delta w'` the kernel
  contraction is at least `1 - delta (1 - q) + u`, and it is not below 1 for
  `delta < delta* = u / (1 - q)`. No metric certifies `1 - q` above the extreme pole's gap
  (bell 10 Hz, Q 18, +24 dB, `k = 0.0140`), so `delta*` is at least 0.0060, 0.0065, 0.0120 and
  0.0131 at 44.1, 48, 88.2 and 96 kHz, and about twice that with the certified rate of scope F2.
  One frame of ramp toward any design moves `delta` by 1/64 = 0.0156.
- **Reachability.** `quantum_frames` only has to be nonzero (`crates/session/src/validate.rs:63`),
  and a retarget restarts the ramp from the words in force (`start_ramp`). So a history that sends
  the band toward a design for one or two frames and then toward the identity again holds `delta`
  in or near `(0, delta*)` for as long as it likes. At 48 kHz and `B = 1`, one cycle (one frame
  toward a design, then a 64-frame ramp back to the identity) has a worst-case rounding gain of
  about `64 u = 3.8e-6` against an exact decay of about `0.5 (1 - q) = 2.3e-6`. So the certified
  pre-`N` state bound grows without limit. An exact identity (`delta = 0`) is frozen with no
  rounding, which is why the gap occurs only on the chord.
- **Consequence.** Without a further lemma on the ramp law and the minimum block size, the
  pre-`N` state of a section whose band can ramp toward the identity has no certified
  peak-relative bound. Such a lemma depends on fine margins (at `B = 2` the cycle balance is
  within about 20%). Its alternative is scope R4's class-B kernel snap (for example, snap a ramp
  word below a stated weight to the exact identity), which only the owner can rule on. The bound
  and the gates stay as frozen.
- **What still holds.** Hulls without the identity (the de-esser's and the dynamic EQ's
  frequency and Q live section, if neither ramps to the identity) are not affected. There the
  uniform `q < 1` over the design hull is the whole argument, and a slice limited to
  `identity_reachable = false` closes on the scope note's plan.

### Root ruling after the stop (2026-10-24)

- #1372a takes option (b), which moves no bit. The norm constants are parametrised, and
  `common_lyapunov` and `hull_cascade` (`crates/math/src/tail/hull.rs`) cover design hulls only.
  `HullSection::identity_reachable = true` is refused with `TailBoundError::IdentityReachable`.
  A hull with an identity vertex cannot certify `q < 1`, so `common_lyapunov` refuses it as
  `NotContracting`.
- Options (a), a class-B kernel snap, and (c), a ramp-law and minimum-block lemma, go to the
  owner. #1372b (the EQ's own bound) needs the identity case, so the parametric EQ keeps its
  current `Infinite`/`Unstated` declaration until the owner rules.
- Which callers qualify (read on `main` 6172dc4e6 and PR #1919 head 5316681de):
  - **De-esser qualifies.** Its SVF words are always the EQ's 0 dB high-pass design (`targets`).
    They ramp only between designs on frequency or Q changes. The mode, detector and listen
    switches crossfade the output arms, not the SVF words, and there is no enable or kind ramp.
  - **Dynamic EQ does not qualify.** A disabled band's design is `[0.0; 9]` (`band_design`), and
    its words ramp to the identity section. It stays `UNBOUNDED` for now.

### 1372a evidence (option b, attempt 1)

- Derivation: `docs/derivations/1372-parametric-eq-tail-and-rest.md`, part 1.
- Gates: a1 (`contraction_bounds_every_hull_word` and `cascade_bound_covers_every_hull_member`),
  a2 (`one_vertex_tail_is_tight`), the composition check (`composition_covers_crossings`), and
  a3 (the builtins' and the multiband's `tail_contract` tables, unchanged).
- Tightness (a2): with a one-vertex hull at the extreme (bell 10 Hz, Q 18, +24 dB, 48 kHz),
  `T = 2,262,795` and `T_b = 2,212,870`, a ratio of 1.0226.
- Evidence, not a gate: a 25 x 25 `(g, k)` grid hull over the whole EQ domain certifies
  `q = 0.99999095` at 48 kHz, with `l12 ~ k_min / 2`. The extreme's `T / T_b` under that hull is
  1.032 at 44.1 and 48 kHz, far inside R3's factor of 2.

### 1372a attempt 2 (verifier FAIL on attempt 1; the math module itself was sound)

- Added a feedthrough row to `cascade_bound_covers_every_hull_member`: two one-vertex sections
  at `g = 0.05`, `k = 1.0` and `k = 0.1`, with section 1's mix `(1, 0, 0)` and section 2's
  `(0, 0, 1)`. Removing `section.output_input * v` from `invariant` turns it red.
- Caller obligation, restated in the derivation and `HullSection`: every word triple the kernel
  can load under its own per-word ramp and retarget rule must lie in hull + margin. The state
  rounding is taken at word magnitudes that are suprema over hull + margin. `identity_reachable`
  is the caller's attestation, and each caller crate tests its own claim.
- The de-esser qualifies only with the joint retarget of all three SVF words. At 6172dc4e6,
  `lib.rs:723` skipped a word whose target bits were unchanged; root's fix is in PR #1922.
  `effect_runtime::svf::RAMP_PATH_NORM_TOLERANCE` makes the same lockstep assumption; #1928
  checks it for the parametric EQ.
- The margin box for `f32` ramp walks and retarget restarts cites the bounded-excess inductions
  of `multiband-compressor/src/tail.rs` and `RAMP_PATH_NORM_TOLERANCE`.

## The hold design (owner rulings of 2026-11-04 and 2026-11-05)

- **Rulings.** The hold (class B, 2026-11-04): on a ramp toward the identity, the recursion
  words `(c1, a2, a3)` stop at weight `delta_s = 1/16`; the mix words finish the 64-frame ramp;
  all six words are the identity at frame 64
  (`docs/derivations/2091-eq-switched-system-strict-decay.md` §6.3). Rest (class B, 2026-11-05):
  a band that finishes turning off at frame 64 has its state cleared to `+0.0` (§7.3). These
  rulings replace the non-goal "any kernel or designer change" for this issue.
- **Root rulings on the plan.** S5 is in #1372. The weight is one `f32` word, walked like the
  words; the payload moves to layout 2. `T` uses `L*` only. `delta_s = 1/8` only if the walk
  loss exceeds 0.68 at 96 kHz. Rule 3 stays.

### Hold attempt 1 record (2026-11-05): S2, S5 and S4 delivered; stopped at S1

- **S2, the hold** (`crates/parametric-eq/src/lib.rs`, `RecursionHold`). The writes come after
  #2250's equal-target return and the stationary hoist in `start_ramp`. `process_section` cuts
  a segment where a lane's recursion words stop and zeroes their increments (`hold_ended`, one
  mask). The weight is evaluated lazily from its sync point. The payload is layout 2: 21 words
  per band, the weight and its increment at words 19 and 20. The step count is derived on
  restore, and a weight that would hold below the floor is refused. The kernel
  (`lane::kernels`) is unchanged.
- **S5, rest.** `snap_ended` sets `ic1, ic2` to `+0.0` on every ended lane whose target is the
  identity. The #979 elision gate's non-vacuity leg (a frozen non-zero state after a switch-off)
  is deleted, because that state is no longer reachable.
- **Tests** (`crates/parametric-eq/src/hold_tests.rs`, `W = 1` and every vector width):
  `a_disable_holds_the_recursion_words_at_one_sixteenth` (red on base at frame 61);
  `a_disable_within_three_frames_of_an_enable_rises_to_the_floor` (red on base at frame
  `d + 1`); `a_restore_refuses_a_weight_held_below_the_floor` (red when `decode_hold` drops its
  floor check); `a_finished_disable_reaches_exact_rest` (red on base: the integrators are not
  `+0.0`).
- **Bit-move evidence (S4, scratch only).** Every preparable `fixtures/session/v1` session,
  plain and live, at quanta 128, 64, 32 and 16 (`"quantum_frames"` rewritten, 38,400 frames):
  136 digests, equal at base `8f46ab035` and head. The 34 digests at 128 frames equal the #2250
  head's. A scratch EQ harness renders 180 band designs (6 kinds, 10 Hz to 20 kHz, -24 to +24
  dB, Q 0.1 to 18, slope 0.1 and 1) with a unit sine at the design frequency and at 3 kHz,
  3,072 frames after the event:
  - disable from a settled design, quanta 64, 16, 3 and 1: only frames 61 to 63 move, largest
    difference 0.0172 of the input amplitude (+24 dB high shelf at 20 kHz), nothing after;
  - enable from rest, then disable 1, 2 or 3 frames later: frames `d + 1` to `d + 63` move,
    largest difference 0.084, nothing after frame 66;
  - re-enable at frame 61, 62 or 63: all later output moves, largest difference 0.80, 1.63 and
    2.49 (a high pass at 1 kHz, Q 18, at its resonance), decaying at the band's rate;
  - a `-0.0` input after a finished disable: no zero sign moved.
  All are inside the plan's scope (0.036, 0.155, 2.80). Nothing else moved.
- **Why S1 and S3 stopped.**
  1. *The walk lemma of the plan is not a proof.* `phi = box(E_1) / m(v)` "independent of
     `omega`" is false on a ramp toward the identity: the excess made while the weight was
     high stays when the weight is at the floor. From a settled design it is about
     `31.4 u |v|` at frame 60 against the margin `delta_s m(v)`, a ratio of about `502 u nu`,
     8 times `box(E_1) / m(v)`. A retarget also carries the start's excess with the old mass:
     a 63-frame ramp away from a heavily damped design adds about `2,079 u |v|` per unit of
     that design's mass while the mass falls by 64 times. A sound bound must count the whole
     chord. A per-piece lemma with a one-dimensional stale recursion is sound but gives
     `phi` about 2.5 at 44.1 kHz, because it compares every stale excess with
     `delta_s m_D`; a tight version needs the weight as a second dimension of the recursion.
  2. *`T` is about 10 times `T_b`, not 1.55 times.* With a provisional `phi = 0.3` (not
     proven), the six-section hull cascade in `L*` gives `T` = 20,718,306 (44.1 kHz) and
     22,646,179 (48 kHz), against `T_b` = 2,212,870 at 48 kHz. `P*` is 1.6e32 and 2.8e32,
     above the sanitizer limit, so `tail_every_peak = R(P*)` = 34,735,490 and 38,016,906. At
     88.2 and 96 kHz the bound passes `HORIZON_LIMIT` (`TailBoundError::Horizon`). The cause:
     each section's state invariant is about `6.2e5` per unit of its input
     (`delta_s input / (1 - q_s)`), a band's output row is about 90 (the high shelf's `m1`
     reaches 75), and six sections compound it, with a `t^5` coupling after `N`.
  3. *Gate 1's tightness cannot hold for a sound domain-wide `T`.* `T` is one value over the
     domain, and the domain holds cascades with repeated slow poles. Measured by brute force at
     48 kHz (`f32` words, `f64` arithmetic, 24,000,000 frames): one extreme bell has
     `T_b` = 2,212,870; four in series 4,104,275 (1.85 times); the HPF at 10 Hz and Q 18, four
     extreme bells and the LPF at 10 Hz and Q 18 in series 4,848,611 (2.19 times). So every
     sound `T` is above `2 T_b` of the single extreme band.
- **Checks** (head `19a8f7c94` plus the capi budget): `cargo fmt --all --check`; workspace
  clippy with CI's flags; release and debug tests of `parametric-eq`, `math` and
  `effect-contract` (`--all-features`); release tests of `host-core`, `graph` and `capi`
  (`--all-features`); `scripts/check-workspace-policy.sh`; `realtime_policy`; `audit capi`
  (0 violations); `scripts/check-web-audioworklet.sh` (it builds the module); the aarch64 check
  of `parametric-eq`. All pass. The capi `effect_scalar_state_bytes` ceiling rises from 9,280 to
  9,936 for layout 2.
- **Not done.** S1 (`math::tail::hull` weight floor, walk lemma, held contraction), S3 (the
  EQ's `tail.rs`, the pinned table, `tests/tail_contract.rs`), the derivation's part 2 and
  `dsp-research/filters.md`. `tail_and_rest` still declares `Infinite` and `Unstated`, so the
  `live_lanes.rs` pin is unchanged. The S1/S3 work in progress is kept outside the tree as a
  patch for the next attempt.
