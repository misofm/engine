PASS

# #1408 attempt 1 -- adversarial verdict

Commit under review: `1f333e98d` (branch `codex/d15-stream-g`), parent `f44cf54bf`. Reviewed
`git diff f44cf54bf 1f333e98d` against the spec body at `1f333e98d`, AGENTS.md, decision 15
(D15-4(b)) and the no-shortcuts principle. Every gate and mutation below ran in a private export
(`/tmp/claude-1002/v1408/`), never in the shared worktree.

No BLOCKER or MAJOR finding. The law is D1/D3 exactly, the eight sites are the only D11 builtin
ramp updates in the tree, no rendered bit of any existing test, pin or fixture moved, every gate
is green, and every new test turns red on its named defect.

## Findings

### MINOR

1. **The gate-2 test-value claim is still false in the spec body**
   (`.github/ISSUE_SPECS/1408-...md:264-265`, and gate 2 at `:223`). The text says the long flip
   catches "a kernel site left on the old law (any of the trim bodies, dual or banked)". It reaches
   only `identity_chain_ramp_block` (site 7). I re-ran each site mutation alone. For sites 3, 4, 5,
   6 and 8, both gate-2 tests stay green and only the added `every_trim_body_keeps_a_flip_inside_its_trim`
   is red. The attempt record says this. The stateless body's Objective gates and Test value do not,
   so a later reader would believe gate 2 alone guards the five other bodies. Amend gate 2 (or add a
   gate 2b) and the Test value line so they name the added test as the guard for sites 3-6 and 8.
   This is the implementer's point 2: the added test does reach all five, and the spec claim should
   be amended.

### NIT

1. **The D2 wording names the wrong bound.** `crates/lane/src/kernels/builtins.rs:154` says "the
   lower bound is therefore never the active side", and the spec at `:102` says "the `low` bound".
   For a downward ramp `low = min(current, target)` *is* the target, and it is the active side. The
   0 dB to mute fade is one example. The intended statement is "the bound at `current` (the start
   side) is never active". The mathematics holds. A probe over 200,000 random consistent ramps
   (5.0e8 updates) found the start-side bound engaged 0 times, and `low` engaged as the target on
   downward ramps 51,222 times.
2. **The gate-1 wording "returns `target` for any step once `current == target`"** (spec `:218`)
   is false for `step = +0.0` with `target = -0.0`. The result is `+0.0`, which is what D3 requires
   and what the old law produced. This case is reachable: a matrix word held at `-0.0` while its
   siblings ramp gets `step = (-0 - -0)/n = +0.0`. It reads `+0.0` mid-ramp, and the snap then
   assigns `-0.0`. The two values are numerically equal, stay inside the endpoints and are
   unchanged from base, so nothing is affected. Amend the wording to "any nonzero step". (This is
   the implementer's point 3: it does not matter.)
3. **The reach of `every_trim_body_keeps_a_flip_inside_its_trim` is fragile**
   (`crates/builtins/tests/input_liveness.rs:880`). The test reads `trim_signed` only at block ends.
   For sites 3, 4 and 8, its reach depends on one block end (frame 1,044,480) falling inside the
   overshoot stretch of about 7,200 frames. The test has no self-check that the unclamped law
   overshoots at a checked frame. Gate 1 has such a check: "the sample must reach the clamp". If a
   later change moves the dB-to-gain word, the window or the block size, the test can pass without
   checking anything. Suggestion: read the word every frame near the end, or assert reach.
4. **A stale statement of the law remains outside the authorized paths.** The
   `FaderMuteRampBuiltins` doc at `crates/builtins/src/lib.rs:4224` still says "`current += step`
   per sample". Fix it in a follow-up or in #1409.
5. **The `docs/rulings/effect-floor-accounting.md` edit is slightly wider than authorized.** The
   spec authorized "the trim ramp gap term only". The edit also adds one sentence on the fader and
   matrix clamp cost. It matches the spec's DSP-evidence text and is harmless. The term changes
   from 3 to 7 (4 clamp lane-ops: `min`, `max`, `max`, `min`, on top of the old count's convention),
   which is correct.

## The eight points in the brief

1. **Clamp spelling.** The spelling `L::min(high, L::max(low, next))` with `low = L::min(current,
   target)` and `high = L::max(current, target)` is D1 operation for operation. D8 defines
   `Lane::max` as `select(a > b, a, b)` and `Lane::min` as `select(a < b, a, b)` (`lane/src/lib.rs:394-413`).
   The x86 `maxps`/`minps` overrides and the wasm swapped `pmax`/`pmin` overrides reproduce this.
   NEON uses the select. So the inner `max` replaces `next` only when `low > next` (strict), and the
   outer `min` replaces it only when `high < x` (strict). A NaN `next` passes through both. I ran a
   probe at Scalar, Simd4 and Simd8 over 12,167 special-value triples (each of `+-0`, `+-inf`, NaN,
   subnormals, `+-MAX`, the +24 dB trim word and others). It found 0 mismatches against the
   D8-spelled scalar D1. The method spelling and the trait spelling are the same operations.
2. **Gate-2 reach.** See MINOR 1. The added test reaches all five other trim bodies. Sites 3, 4, 5,
   6 and 8 are each red alone, with the legs "filtered dual", "filtered collapsed",
   "filter-retarget dual", "filter-retarget collapsed" and "identity collapsed" respectively. Amend
   the spec.
3. **"Returns target once current == target".** See NIT 2. The claim holds for every nonzero step.
   The one exception is zero, D3-mandated and harmless.
4. **"No bit moved" and production reach.** D6 is confirmed:
   - `audit fixture-builtins --write` produces byte-identical trees (all 50 files) from the base
     kernel and from the clamped kernel, on the same build path.
   - The G5 native pins are unchanged and green.
   - With the old law restored at all eight sites and the new twin kept, a release run of `lane`,
     `builtins`, `dsp-reference`, `graph`, `host-core`, `builtins-compiler` and `engine`
     (`--no-fail-fast`, 73 binaries green) fails only the five new kernel tests.

   The absence of moved pins does not mean production cannot reach the moved frames. The pinned
   windows (37 to 129 frames) never overshoot, but production windows do. I simulated the old law:
   - At the #1055 defaults (10/20 ms, 480 to 1920 samples), 0.06% to 2% of random fader or pan
     moves overshoot by a few ulps. On the 0.1 dB UI grid over [-24, 12] dB at 960 samples,
     454 of 129,960 moves overshoot (for example -23.9 to -23.6 dB, frame 959).
   - At 1 s (48,000 samples), about 49% of moves overshoot, and mute fades reach a negative gain.

   So real sessions and automation do hit the moved frames. The reach lives in the new tests, not
   in the pins. G5 has no case where the clamp engages: NEON is covered by the AArch64 release leg,
   which runs `ramp_endpoint.rs`, and wasm simd128 is covered only through the D8 `pmin`/`pmax`
   lowering. A corpus case would be an optional follow-up, not part of this slice.
5. **The filter-response problem predates this slice. It is a build-determinism defect, not a stale
   fixture.** Out of scope; file a new issue.
   - `--write` is deterministic run to run for a given binary, and only the `impulse_dft_magnitude_db`
     column differs (all 1,630 rows; median 2e-12 dB, max 3.2e-5 dB at -186 dB). `--check` passes
     because its tolerance is 0.05 dB.
   - At `477dc15ee` (the file's last regeneration, #177), `--write` reproduces the committed file on
     this host.
   - Bisect over first-parent `main` (406 merges, one build path): every commit through `6fb211594`
     reproduces the committed bytes.
   - The same source `6fb211594` built at a different checkout path produces the other variant. The
     two binaries differ in machine code under fat LTO: `BuiltinChain::process_dual_mono` has 3,521
     instructions in one and 3,360 in the other, with different vectorization, while
     `measure_response`, and with it the DFT, is identical.
   - `f44cf54bf` and `1f333e98d` give the other variant on both paths.
   - There are exactly two variants (sha256 `ece29117...`, the committed one, and `d577dab4...`).
     The builtin chain's 1 s impulse response therefore depends on codegen at about the
     1-ulp-of-small-samples level. The `pcm/*.f32le` fixtures and the G5 corpus are unaffected.
   - Root cause not isolated.
   - Hazard for D6 and every later re-pin: running `--write` churns this file with no semantic
     change. Never commit that churn as a "moved" artifact.
6. **Runtime.** Acceptable. Debug `input_liveness` takes 54.6 s here; it is the second-slowest debug
   binary after `randomized.rs` at 76 s. The local test-debug-b command took 5:31 including the
   build. On CI, test-debug-b runs 5 to 7 min against a 15 min timeout. AArch64 debug, which also
   runs `builtins` in debug, runs about 10.5 min against a 30 min timeout. Gate 2 freezes "debug runs
   the same case", so trimming it would need a spec change.
7. **Ramp interaction.** Clean. Sites 5 and 6 change only the trim update. The six-word filter
   coefficient updates (`builtins.rs:918-923`, `:1023-1028`) are byte-for-byte unchanged. Expect
   textual merge conflicts with the #1407 revision in the same two functions. The floor term is
   correct (NIT 5).
8. **Gates and mutations.** All gates are green. The mutation table is below.

## Test value (new or rewritten tests)

- `ramp_toward_holds_the_target_and_keeps_in_range_bits_at_every_width` (`crates/lane/tests/ramp_endpoint.rs:103`):
  turns red on a clamp with its outer or inner operand order swapped (an in-range signed zero
  becomes the endpoint's zero), a non-strict select, a one-sided clamp, a target-only clamp, a clamp
  that hides a NaN, or an in-range snap. Each was verified red. No other test calls `ramp_toward`.
- `random_ramps_stay_inside_their_endpoints_at_every_width` (`:204`): turns red on a one-sided or
  target-only clamp and on a clamp that moves in-range words (in-range snap). Each was verified red.
  It is judged by reach: it asserts that the sample overshoots under the old law at every width.
- `a_long_polarity_flip_stays_inside_its_trim` (`input_liveness.rs:609`) and
  `a_long_polarity_flip_on_a_bank_member_stays_inside_its_trim` (`:631`): turn red when site 7,
  `identity_chain_ramp_block`, is left on the old law (`|-15.848933|` passes the trim `15.848932`
  at word 19,414,966). This is #1329's case. The flip also crosses the `2^24` countdown reload.
- `every_trim_body_keeps_a_flip_inside_its_trim` (`:880`): turns red when any one of sites 3, 4, 5,
  6 or 8 is left on the old law. Each was verified red alone. No other test reaches those five. See
  NIT 3 on fragility.
- `a_long_fader_or_mute_move_stays_inside_its_endpoints` (`fader_ramp.rs:331`): turns red when
  site 1, `gain_mute_ramp_block`, is unclamped (frame 47,969 applies `-1.947e-5` on a fade to mute).
- `a_long_matrix_move_stays_inside_its_endpoints` (`matrix.rs:642`): turns red when site 2,
  `matrix2x2_ramp_block`, is unclamped (`ll` reaches `-1.947e-5`).
- `the_trim_ramp_is_bit_identical_to_the_reference_ramp` (`input_liveness.rs:285`, oracle switched
  per D4): keeps its existing claim. A trim word sequence that departs from the D11 twin, for
  example a countdown moved after the done compare, turns it red. The rename and the deleted
  `ParameterSmoother` oracle follow AGENTS.md ("supersedes ... deletes it").
- `ramp::tests::a_long_ramp_never_passes_its_target` (`dsp-reference/src/ramp.rs:120`): turns red
  when the twin is reverted to `+=` (verified). No oracle comparison engages the twin's clamp.
- `ramp::tests::the_clamp_keeps_a_signed_zero_word` (`:132`): turns red when the twin's outer or
  inner comparison is the non-strict mirror (verified, both).

## Mutation runs (release; each alone, then restored)

| Mutation | Red |
| --- | --- |
| site 1 `gain_mute_ramp_block` -> `current.add(step)` | fader gate-3 test |
| site 2 `matrix2x2_ramp_block` | matrix gate-3 test |
| site 3 `input_chain_ramp_block` | `every_trim_body` (filtered dual) |
| site 4 `input_chain_ramp_block_mono` | `every_trim_body` (filtered collapsed) |
| site 5 `input_chain_ramp_block_filter` trim | `every_trim_body` (filter-retarget dual) |
| site 6 `input_chain_ramp_block_filter_mono` trim | `every_trim_body` (filter-retarget collapsed) |
| site 7 `identity_chain_ramp_block` | both gate-2 tests, `every_trim_body` |
| site 8 `identity_chain_ramp_block_mono` | `every_trim_body` (identity collapsed) |
| all eight sites, wide release suite, no fail-fast | only the 5 new kernel tests (73 binaries green) |
| `L::min(L::max(low,next), high)` / `L::min(high, L::max(next,low))` / both | gate 1 fixed |
| one-sided `min(high,next)` / `max(low,next)` / target-only | gate 1 fixed and randomized |
| non-strict outer select / NaN hidden | gate 1 fixed |
| in-range snap (`abs(x - target) < 1e-6`) | gate 1 fixed and randomized |
| twin `+=` / twin outer non-strict / twin inner non-strict | the two twin unit tests |

## Gates run (export of `1f333e98d`)

- test-debug-b command (spec gate 6, line 1): ok (5:31 wall, 16 `input_liveness` tests, 54.6 s in debug)
- `--release -p builtins --test input_liveness`: ok
- `--release -p lane -p math -p wasm-gates --features math/lane`: ok (G5 native pins unchanged)
- test-debug-a workspace command: ok
- `conformance_fixtures -- --check`: ok
- `audit` release and `check-builtins-fixtures.sh`: ok (50 files). `--write` gives the same tree before and after the change
- `run-wasm-gates.sh`: ok (native, simd128, V8 spill gate: no carried stack slot)
- worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh
  --without-metadata-regeneration`, `check-browser-expected-resources.py --artifacts`,
  `test-web-audioworklet.sh`, `check-web-audioworklet-v8-spill.py` with `--self-test`): ok
- `check-cross-targets.sh`: PASS (the #1018 expected failures only)
- `check-lane-policy.sh`, `check-builtins-policy.sh`, `check-graph-determinism.sh`,
  `check-workspace-policy.sh`: ok
- `cargo clippy --workspace --all-targets -D warnings`, `cargo fmt --check`: ok
- Not run: the AArch64 legs (CI only).
