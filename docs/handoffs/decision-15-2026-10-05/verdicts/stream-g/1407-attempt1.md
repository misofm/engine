FAIL

# #1407 attempt 1 -- adversarial verdict

Commit under review: `f44cf54bf` (`codex/d15-stream-g`). Diff reviewed: `git diff a18a2652d f44cf54bf`.
I read the code only with git at that commit. I built and ran everything in exports under
`/tmp/claude-1002/v1407/`. Evidence: `/tmp/claude-1002/v1407/evidence/` (repro tests, before/after
bit dumps, mutation logs, the gate-4 release log).

Summary: rules 1-4 match D1-D4 exactly, gates 1-4 are real, and all spec gates are green.
The verdict is FAIL because of one new interaction: under the mono collapse, rule 3's integrator
predicate reads the right channel's frozen state. This moves rendered bits (BLOCKER 1). There is
also one spec-level numerical gap that gate 4 does not reach (MAJOR 2).

## BLOCKER

### 1. Under the mono collapse, rule 3 reads stale right-channel integrators. A collapsed track then renders bits that a never-collapsed run does not, and L != R on a mono source.

`crates/builtins/src/lib.rs:1420-1423` (`settled_disabled` reads `self.state.section[channel][section]`
for each channel). In a collapsed block, `process_mono` (`lib.rs:1832`) advances channel 0 only.
Channel 1's integrators stay frozen at their values when the collapse engaged. `mirror_filter_ramp`
(`lib.rs:1666`) copies only coefficients, target, step and countdown. Only `desymmetrize`
(`lib.rs:1995`) repairs the state, and the chain calls it *after* the drain has applied the record.
The ruling paragraph that follows the one this slice edits says the same thing: "disengagement
restores only integrators".

The sequence:
1. The collapse engages over non-zero integrators. Both of these paths do it: a plan swap that
   carries a filtered mono-source lane (`disengage_for_carry`, then import, then the successor
   inherits agreement and collapses at once), and an M3 re-engage after a dual window.
2. A symmetric (`Both`) disable completes while collapsed. The mono kernel's identity completion
   clears channel 0's integrators. Channel 1 keeps its stale, non-zero values.
3. A symmetric enable arrives. Channel 0 is "settled disabled" and takes rule 3 (recursion jumps,
   mix crossfades). Channel 1 fails the predicate on its stale integrators and takes rule 4 (all
   six words sweep up from the identity). The DESIGNED term then declines, the chain disengages,
   and `desymmetrize` zeroes channel 1's state. But channel 1 still renders the rule-4 sweep from
   the identity, which is the near-identity word class this slice exists to remove.

Reproduced at two levels. Both are red on `f44cf54bf` and green on the parent `a18a2652d` (the old
law does not read state):
- Host level, through the real chain and plan swap
  (`evidence/successor_swap_zz_verifier.diff`, added to `crates/host-core/tests/successor_swap.rs`).
  The mono `filtered_session(true)` swaps to `with_muted_track(..)`. Then `eq2`'s LPF gets a `Both`
  disable at block 8 and a `Both` enable at block 10. The oracle is the same successor with
  `force_mono_collapse_off`, which the repo itself uses as the class-A oracle
  (`a_lane_from_a_delayed_strip_keeps_its_chain_dual_after_the_swap`). On `f44cf54bf`, Simd4 and
  Simd8 both first differ at block 10, max |diff| 1.76e-1, with an extra disengage (transitions
  `[1,0,0]`). The parent gives no difference at either width, and the chain stays collapsed.
- Builtins level, with the bank and the chain's dispatch order (`evidence/zz_verifier_collapse.rs`).
  First divergence at the enable block. Lane 0 L != R by up to 1.24e-1 in that block.

Why this is a BLOCKER: the mono collapse is class A, so it must never move a bit. This diff
introduces the regression, in a production path: any structural edit that carries a filtered mono
track, followed by a live disable and re-enable of its filter. It also contradicts the slice's own
D5 premise ("a lane whose channels hold equal words keeps equal words") and the ruling's new
sentence "Every recursion word the kernel can load is then a design, the identity at rest, or a
linear mixture of designs".

I found no production writer that leaves non-zero integrators on an identity section other than
this stale channel 1. `set_lane_state_words` is test-support only. A carry desymmetrizes first.
Identity sections keep `+0.0` under the kernel. So the D3 integrator clause has no production case
where it fires correctly, and it has this one case where it fires wrongly.

A fix needs a root/spec decision, because D5 limits the change to `apply_prepared_filter`. Options:
- (a) In the collapsed body, mirror the identity-completion integrator clear onto channel 1. In the
  counterfactual dual run, channel 1's state there is `+0.0`.
- (b) Give the predicate the integrators of the channel the stage actually advanced.
- (c) Revisit D3's integrator clause.

Add a collapsed-versus-dual gate for disable then enable.

## MAJOR

### 2. (Spec-level; the old law has it too.) In `f32`, a ramp can leave the hull of the designs by up to 3x the gate-4 inflation bound. Gate 4's generator never reaches this case.

Probe: `evidence/zz_verifier_stall.rs`, release, on pristine `f44cf54bf`. Retarget chains among
*close but distinct* designs, restarted before completion every q frames. This is a cutoff knob
dragged near the maximum cutoff, or alternated with a cutoff 1 Hz lower. 404 of 1,512
(rate, section, quantum, pattern) cases exceed `q_design + 6*2^-24*kappa`, at every launch rate
and in both sections. The worst excess is 2.569e-6, against a bound of 8.633e-7. The norm is
convex, so these `f32` words lie outside the convex hull of the designs used.

The cause is rule 4's accumulated per-component `current += step` in `f32`. The components round,
or stall, independently, so the word drifts off the segment by tens of ulps in coarse-ulp
components near 1. The parent gives the identical 2.569e-6, so this diff did not introduce it.
Stability itself survives: the reached norms stay at most 1 - 4.98e-5.

But the slice's frozen claims are wrong as stated:
- the product outcome "provably stable in f32 under every control history";
- the DSP evidence "sampled f32 within 2.5e-7 of exact";
- the ruling's "every recursion word ... is a design, the identity at rest, or a linear mixture of
  designs";
- gate 4's test-value claim "any rule that lets a word leave the convex hull of designs exceeds the
  bound".

#1329 is to build its tail bound on these claims. Gate 4 draws designs log-uniformly, so it almost
never sends consecutive close designs and stays green. Root must decide one of:
- add a per-component rounding term to the bound and to #1329's premise;
- change the ramp arithmetic so it cannot leave the segment (e.g. `start + n*step` with an exact
  endpoint);
- extend gate 4's generator to close retargets.

## MINOR

3. **Rule 2's unconditional countdown restart is correct but no test pins it.**
   `lib.rs:1444` (`let mut changed = freeze_recursion;`). Mutation M2c (`changed = false`, so a
   restart happens only when the mix differs) passes every committed test, debug and release.
   Probe `evidence/zz_verifier_samedrain.rs`: an enable from rest and a disable in the same drain
   (two records at one boundary).
   - With the code as committed, the section completes, clears and elides.
   - Under M2c it settles at `[design recursion, identity mix]` with target identity, live
     integrators, and never elides.

   This is D2's "starts the 64-frame countdown" in the one case where it matters. Add the case to
   gate 1 or gate 2.
4. **Gate 4's 1/8 endpoint draws make every history's bound the global maximum design norm.**
   `filter_liveness.rs:664-668`. The chance that a section's history never draws the maximum is
   about (31/32)^512, roughly 1e-7. So `q_design` is about 1 - 5.24e-5 everywhere. Without the
   endpoint draws, it is about the lowest drawn cutoff (10 Hz: 1 - 9.25e-4 at 48 kHz, 1 - 4.63e-4
   at 96 kHz). The bound's margin resolution therefore drops by about 9-18x. A hull violation with
   a margin between those values passes.
   - The named defects are still red: full revert, and rules 1, 2 and 3 removed.
   - The draws add reach: the most extreme design itself.
   - The reported maxima ("excess 0") are just the maximum-cutoff design's own norm, so they say
     nothing about interpolated words.

   Acceptable as a deviation. Better: report and bound the excess per word against the designs
   that word's ramps actually interpolated, or put the endpoint draws in separate histories.

## NIT

5. The spec's DSP evidence line still says "-5.21e-5 ... which is the slowest design"
   (`1407-...md:130`), and so does gate 4's doc comment (`filter_liveness.rs:632`). Deviation (3)
   is right: the extreme is the maximum cutoff (about fs/2 - 0.6 Hz), not 10 Hz (measured in
   `evidence/zz_verifier_gate4.rs`). The text was not corrected.
6. "For designs it equals the spectral radius" holds only approximately. `||A||_V - rho` is
   2.0e-8 (44.1/88.2 kHz) and 4.0e-8 (48/96 kHz) at the maximum cutoff, because the f32 design
   words are not exactly `k = sqrt(2)`-consistent. This does not affect the bound.
7. Rule 3's predicate uses bitwise `+0.0` as D3 states. A `== 0.0` mutation (M3z) survives every
   test. There is no production path to a `-0.0` integrator on an identity section, so this is
   noted only.
8. Ruling edit (`docs/rulings/builtins-input-liveness-d2.md:18,31`): two new lines exceed 100
   columns. The old #808 tail ("Settled all-disabled filters execute no SVF recurrences ... Mixed
   banks keep the existing fallback.") now trails the new #1407 rules paragraph.
9. The full release gate-4 scan (all rates, quanta 1-63) runs in no workflow. CI runs only the
   debug subset (test-debug-b). This is the spec's design, but the per-rate claim rests on a local
   run.

## Rules 1-4 against D1-D4 (item 1)

- **Rule 1** (`lib.rs:1405-1408`) applies only when `remaining != 0` and all six in-flight target
  words are bit-equal. It `continue`s before any write. That matches D1. M1s (rule 1 on settled
  lanes) turns only the D1 canary `tests::trim_refresh_preserves_asymmetric_settled_filter_steps`
  red. The canary is unchanged and green.
- **Rule 2** freezes `c1`, `a2`, `a3` (step `+0.0`), ramps the mix, and always restarts
  (`changed = freeze_recursion`). That matches D2.
- **Rule 3** reads the countdown, the six identity words (bitwise) and both integrators at bits 0,
  then writes the recursion into `coef` before the step loop. That matches D3, apart from
  BLOCKER 1.
- **Rule 4** is unchanged.
- Each covered channel decides from its own words and writes its own `+0.0` steps. The function ends
  with a full `refresh_channel_symmetry`.
- No allocation, lock or syscall was added. `check-builtins-policy` and `check-workspace-policy`
  are ok. No sealed size moved.

Interactions checked:
- Reset (both kinds): snaps and clears; rule-3 predicate consistent.
- Snapshot: reads target words; unaffected.
- Swap export/import: the full rule state is carried, and the drain runs after `disengage_for_carry`,
  so it reads real state.
- Bank versus scalar: same `InputStage` code.
- Both sections.
- Atomic pair edits: a re-sent unchanged section takes rule 1 while in flight and rule 4 with no
  change when settled.
- Mono mirror: see BLOCKER 1.

Acked-batch question: no acked target is dropped. Rule 1 skips only a target bit-equal to the one
already in flight. That target is reached exactly at the first send's A+64, which is at or before
the re-send's A+64. Rules 2 and 3 reach their target exactly at A+64. Host-core admission and ack
are unchanged.

## Deviations (item 2)

- **(1) Gate 1 on the bank** uses the integrators plus `bank_elision_plan`. Acceptable and strictly
  stronger. `section_is_identity` checks all six words and both integrators on every lane, and
  `refresh_filter_plan` forces in-flight sections non-elided.
- **(2) Gate 4 design.**
  - q one-frame calls: verified. My `evidence/zz_verifier_gate4.rs` ran the committed generator
    against single q-frame calls. Output bits and block-end words and integrators are identical over
    all 2,064,384 blocks (4 rates x 63 quanta x 16 histories x 512 blocks).
  - One command per section per block: an acceptable reading.
  - Zero-recursion words excluded but held to `+0.0` integrators: acceptable.
  - Endpoint draws: see MINOR 4.
- **(3) The maximum cutoff is the extreme**: correct (NIT 5).

## Bit moves (item 3)

Reproduced with `evidence/zz_verifier_bits.rs`, before (`a18a2652d`) and after, at every launch
rate:
- **Disable** (HPF 1 kHz, LPF 1 kHz, LPF 15 Hz): frame A and A+64 onward are bit-identical, and
  only A+1..A+63 move. After the change, the output is within 3.9e-7 (HPF) and 5.5e-8 (LPF) of the
  f64 blend `(1-n/64) y_design + (n/64) x`. Before, it was up to 1.71e-1.
- **Enable from rest**: frame A is identical, and later frames move (the state history differs).
  After, the output is within 3.2e-7 of `(1-n/64) x + (n/64) y_design_from_rest`. Before, up to
  1.87e-1.
- **Design to design** (1 kHz to 2 kHz, 15 Hz to 30 Hz): every frame is bit-identical.

No pinned artifact exercises a live enable or disable:
- host-web live filter tests are design to design;
- builtins fixtures are prepared;
- the browser qualification compares browser and headless under the same law;
- browser `expected.json` digests agree with the built module.

So nothing should have moved, and nothing did. The one invariant that live enable/disable does
reach, collapse equals dual, did move. No gate exercised it (BLOCKER 1).

## Test value (item 4) and my mutation runs

Mutations were applied to `lib.rs` in an export and run debug (whole builtins crate) and release
(`filter_liveness`). Logs are in `evidence/mutations/`.

| Mutation | Result |
|---|---|
| M0, full revert | gates 1-4 red; nothing else red |
| M1, rule 1 removed | gates 1 and 4 red |
| M1s, rule 1 on settled lanes | only the D1 canary red |
| M2, rule 2 removed | gates 2 and 4 red |
| M2c | survives (MINOR 3) |
| M3, rule 3 removed | gates 3 and 4 red |
| M3b, predicate ignores integrators | gate 3 red (restored case) |
| M3z | survives (NIT 7) |

The parent's own suite is green, so no existing test catches M0, M1, M2, M3 or M3b.

Test-value sentences:
- **Gate 1** (`a_disable_re_sent_every_frame_completes_clears_and_elides_on_time`): a law that
  restarts the ramp on a bit-equal in-flight re-send (M1, M0) never completes, clears or elides a
  disable hosts keep re-sending. No other test re-sends a target.
- **Gate 2** (`a_disable_freezes_the_recursion_words_and_moves_only_the_mix`): a disable that ramps
  `c1`, `a2`, `a3` toward the identity instead of freezing them (M2, M0) is red from frame 1, from a
  settled design and from an interior word.
- **Gate 3** (`an_enable_from_rest_jumps_the_recursion_only_when_the_integrators_are_zero`): an
  enable from rest that sweeps the recursion up from the identity (M3, M0) is red, and so is a
  rule-3 predicate that ignores the integrators (M3b, restored case).
- **Gate 4** (`every_reachable_recursion_word_stays_inside_the_hull_of_the_designs`): removing
  rule 1, 2 or 3, or the whole law, lets restart chains and identity ramps reach words beyond the
  largest design norm (M0, M1, M2, M3). It does not reach close-retarget f32 drift (MAJOR 2), and
  its bound is loose (MINOR 4).

## Gates run (item 5), all in a pristine export of `f44cf54bf`, x86-64-v3

| Gate | Result |
|---|---|
| `cargo test --locked --all-targets -p lane -p builtins -p dsp-reference --features builtins/test-support,lane/test-support` | pass, 223 passed, 0 failed |
| `cargo test --locked --release -p builtins --features builtins/test-support --test filter_liveness` | pass, 13 passed; per-rate gate-4 maxima reproduce the spec table exactly (-5.213e-5, -5.241e-5, -5.213e-5, -5.241e-5; excess 0; bound 8.633e-7) |
| `test-debug-a` workspace command | pass, 1,426 passed, 0 failed, 10 ignored |
| `cargo build --locked --release -p audit && check-builtins-fixtures.sh` | `builtins fixtures: ok (50 files)` |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`, `test-web-audioworklet.sh`) | all pass; shipped module sha256 `367479425829a096...` |
| `check-builtins-policy.sh`, `check-workspace-policy.sh` | ok |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | clean |
| `cargo fmt --all -- --check` | clean |
| `scripts/check-cross-targets.sh` | PASS (AArch64 rows check/lint only; AArch64 tests are CI-only) |

Not verified: AArch64 or wasm execution of the new tests, and listening (the spec requires none).
