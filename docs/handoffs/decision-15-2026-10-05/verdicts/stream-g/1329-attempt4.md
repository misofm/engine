FAIL

# #1329 attempt 4 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: `git diff 30cfece25 c7f7bbdfb` on `codex/d15-stream-g` (b9de38e70 Amendment 4, feefbe166
implementation, c7f7bbdfb record), and the whole #1329 change as it stands (attempt 3's 1df306d97,
11f59fa42, 6a2c5216a on 0725a8949). Checked against the spec with Amendments 1-4 and the Amendment 3
addendum, AGENTS.md, decision 15 D15-4 and the owner's no-shortcuts rule. All work was done on
exports (`/tmp/claude-1002/v1329b/tree` = c7f7bbdfb, `impl` = feefbe166, `pre` = 0725a8949) and a
scratch copy for probes and mutations. Nothing was edited, built or committed in the worktree.
Small evidence is kept in `/tmp/claude-1002/v1329b/evidence/`.

**The mathematics is now right.** M1 is fixed and I could not break the new operator norm: a
bit-exact replica of `v_operator_norm` matches the module on all 661,869 distinct launch-rate
designs, and every `q` lies above the 110-digit exact `||A||_V`. Every gate is green. One MAJOR
finding fails the attempt. It is a measured regression of the shipped browser path, it does not touch
correctness, and the fix is a few lines.

## BLOCKER

None.

## MAJOR

**MJ1. Preparation computes every strip's design bound, also for a strip whose input lane is live,
and then discards it. In the browser every strip is live, so all of this work is wasted. It adds
about 15 ms (+75 %) to each 64-track browser boot, on the AudioWorklet rendering thread.**

- *Where.* `crates/builtins-compiler/src/lib.rs:3572` computes
  `builtins::input_section_bounds(...)` over **every** strip of the session. Then `:3628-3636`
  replaces the result with the live bound for each strip that has a live input lane. The seal check
  (`expected_tails`, `:3285`) also uses the sealed live bound for those strips, so it never reads the
  design bounds either.
- *Why every browser strip is affected.* The browser prepares with `HostLiveLanes::ALL`
  (`crates/host-core/src/prepare.rs:381-387`, `:1441`), so every browser strip is live.
- *Measured on the shipped path.* I timed `miso_engine_web_v1_boot` on the shipped simd128 module
  with `scripts/web-mixing-automation-benchmark.mjs rebuild-round MODULE 1`. This is the #1289
  rebuild-cost proxy: Node 22 V8 with `--no-liftoff`, pinned to one CPU, 25 boots per document, two
  rounds, every boot rendered an audible block (`evidence/rebuild-*.json`). Boot p50 in ms:

  | module | 9-track EQ | 64-track console | 64-track app shape | 64-track sends |
  |---|---|---|---|---|
  | pre-#1329 `0725a8949` (`10a3c816…`) | 2.15 / 2.15 | 20.20 / 20.25 | 19.70 / 19.69 | 34.04 / 33.74 |
  | **#1329 attempt 4 `c7f7bbdfb` (`d7665dbb…`)** | 2.46 / 2.49 | **35.41 / 35.86** | **35.02 / 35.34** | **49.20 / 50.53** |
  | c7f7bbdfb, design bounds skipped for live strips (`e7ad6e05…`) | 2.25 / 2.23 | 20.34 / 20.48 | 19.81 / 20.03 | 33.82 / 34.03 |

  The patch used for the last row is in `evidence/measurement-patch-and-seal-probe.diff`. It gives a
  live strip the memoryless design before `input_section_bounds`, so nothing is computed for it.
  With the patch, the browser cost is back to the pre-#1329 figures, within noise. Natively
  (x86-64-v3, release, `evidence/probe-cost.txt`), `input_section_bounds` costs 11.7 ms for the
  app-shape fixture's 64 strips. It costs 559 ms for 64 distinct near-top designs (8.73 ms each,
  which agrees with the record). From the V8/native ratio of the app shape (about 1.3x), such a
  browser session would hold the audio rendering thread for about 0.7 s at boot. That figure is an
  estimate, not a measurement. All of this work computes values that the browser never reads.
- *Why this is MAJOR and not a follow-up.*
  - The slice itself causes the regression, on the primary product path (D7: "nothing is computed
    ... on the render thread"). In the browser, preparation runs on the AudioWorklet rendering
    thread (`hosts/host-web/qualification/rebuild-cost.mjs`, #1457 "Where preparation runs").
  - The waste is 100 %, so this is not diminishing-return optimization.
  - R7 asked that each bound is computed once, that nothing is computed twice and that the
    worst-case preparation cost is measured and recorded. A bound that no consumer reads is worse
    than a bound computed twice. The attempt record and #1457 give per-design costs but do not say
    that the browser discards every one.
  - Attempt 3 had the same waste (`prepare_input_bounds` over every strip). Its verdict did not flag
    it.
- *Fix (attempt 5).*
  - Compute design bounds only for strips without a live input lane. The seal, the seal check,
    `input_bounds()` and every fixture stay unchanged.
  - Add a counted test that is red if a live strip's design bound is computed. For example, a
    test-support counter of `fixed_input_bound` calls: zero when every strip is live, the number of
    distinct designs when no strip is.
  - Re-measure `rebuild-round` against pre-#1329 and record it.
  - Restate #1457's context: the browser computes no design bound, and the C ABI control thread
    carries #1457's worst case.

## MINOR

**m1. The seal check's live-strip rule has no test, but a test is feasible.**
`crates/builtins-compiler/src/lib.rs:3298` (`if live { live_bound.ok_or(())? }`). The record says
"no test reaches it". That is a choice, not a limit. A unit test in builtins-compiler's own
`mod tests` (an authorized path) can do this:
- forge the live strip's entry to its design bound in both `tails` and `seal.tails`;
- expect a diagnostic from `validate_for_session`.

I added exactly this (6 lines, `evidence/measurement-patch-and-seal-probe.diff`). It is green on the
shipped code and red on mutation E2, "the seal check skips its live rule" (`a live strip carrying
its design bound passed the seal check`). Every existing test is green on E2. R3's intent (every
claim discriminated where a test is possible) applies.

**m2. `each_strip_is_bounded_by_its_own_design_when_designs_are_shared` does not see the right
channel.**
- `crates/builtins/tests/tail_contract.rs:301`. Every strip in the test has `left == right`.
- A key that drops the right channel (mutation C3: `channels: [lanes[0], lanes[0]]` in
  `crates/builtins/src/tail.rs:103`) leaves all nine `tail_contract` tests green.
- No other fixture catches it either: every left-channel key of the 64-track app session is unique.
- So the spec's claim (a key that "omits ... a section's words" is red) holds for the left channel's
  HPF and LPF words and for the trim (C1, C2, C4 red), not for the right channel.
- Fix: add a strip that differs from the first only in its right channel.

**m3. One Amendment 4 test-value line is false (R3: every claim true).** Spec line 586 says of
`live_tail_every_peak_is_the_rest_at_the_flush_floor`: "no other test computes the live `R(P*)`".
`live_bound_carries_every_term_an_independent_recomputation_requires` computes it
(`rest_at_p_star`), and it is red on the same defect: B6, "live `T_rest` drops `R(P*)`", is red on
both tests. The identity test catches only one thing the oracle cannot: a `T_rest` that is off by
less than the oracle's 0.01 % + 64 frames. Restate the line.

## NIT

- **The `nu` proof text does not derive its own constant.**
  - Where: `crates/math/src/tail.rs:844-852` and the derivation's "Numerical limits".
  - The text says "at most `4.01 u` ... per word, which is at most `nu = 8 u ||R||_2` ... (every
    entry of `|A|` and `|b|` is at most `2`)".
  - From those premises the result is `||R||_2 sqrt(2) 2 (4.01 u) = 11.3 u ||R||_2`, not `8 u`.
    The text leaves out the `sqrt(2)` of the 2-norm of a 2-vector.
  - The constant is still sound, with margin about 1.9: for a Butterworth (any `k >= 0`) TPT step,
    `|1 - 2 c1|`, `|2 a2|` and `|1 - 2 a3|` are at most 1, and one step rounds about `3 u` per word.
    That gives `4.25 u ||R||_2`.
  - Correct the text before #1372 reuses it.
- **`STEP_UP` covers two roundings, but `out.input[1]` has three.** The `STEP_UP` doc covers "two
  rounded products and one rounded addition". `out.input[1]` (`:932-935`) has three summands and a
  product, so `(1 - u)^4 (1 + 4 u)` is `1 - 10 u^2`, just below 1. Other margins absorb this
  (`8 u` against about `6 u` in the output rounding, and `nu`'s 1.9x). Use `1 + 5 u`, or state the
  argument.
- **The record names the wrong module digest.**
  - The attempt record (spec line 1104) names `1a833326…c397d7`. I rebuilt it: that is
    **feefbe166's** module.
  - c7f7bbdfb ships `d7665dbb…6465f5` (3,052,846 B). Its doc-comment edits in `math/src/tail.rs`
    move panic-location line numbers.
  - No audio bit moves. Record the final digest at the batch.
- **Superseded test-value lines are not marked.** The original Gate 2, Gate 3 and Gate 7 lines (spec
  lines 555-565) still stand above their R3 restatements, with no strike or "amended" marker.
- **A host-core doc comment is stale.** `crates/host-core/src/prepare.rs:372` still says a live
  input lane "makes the strip's builtin tail infinite". That has been false since attempt 3. The
  file is outside #1329's paths (ROOT-B).

## Attempt-3 findings: status

| # | finding | status | evidence |
|---|---|---|---|
| M1 | cancelling `f64` operator norm | **Fixed** | Non-cancelling closed form plus a Frobenius bound on the a-priori entry errors, times `1 + 16 u64`. Re-derived (below). Bit-exact replica = module on 661,869 designs; all `q > exact` (110 digits, textbook form cross-checked with the stable form): min margin 4.64e-15, max relative over-estimate 1.26e-14 (`verify_q_sweep.py`, `verify_q_replica.py`). Box norms `P(h)`, `P(E)` above exact by 7.8e-22 and 5.2e-20. The unit test is red on attempt 3's form; all 9 `tail_contract` tests green under it (A1). |
| m1 | gate 2 not at the extreme; vacuous `1e29` run | **Fixed** | Whole history at the worst-case pair. Recovery `(0,0)` and `above > 0` asserted. D1 (1 kHz start) red. `P = 1e29`: last above 437,772, rest by 2,234,641 <= 2,583,197 (44.1 kHz). |
| m2 | live `T_rest` untested | **Fixed** | Identity test plus oracle. B6 red on both (but see m3). |
| m3 | derivation and research-note errors | **Fixed** | FMA sentence corrected. "Far below one sample" replaced. `filters.md` 420,148 = gate 1(a) row. |
| NIT | gate 2 `-0.0` wording | **Fixed** | R4 in the spec and the test doc. |
| NIT | `tail_reference` of the live bound | **Fixed** | Doc line added. |
| ROOT-1 | outside-path edits | **Done** | `filter_response.rs` byte-identical to 0725a8949. The other three ratified (R1). |
| ROOT-2 | gate 5 and `maximum_single_allocation_bytes` | **Done** | +0 measured. `resources.jsonl` +96 B/track in both payload counts (2 tail vectors x 48 B), all nine rows checked field by field. |
| ROOT-3 | test-value claims | **Partly** | Gate 2, 3 and 7 claims restated and true. m2 and m3 above are new overclaims. The seal rule (m1) is untested. |
| ROOT-4 | `-0.0` infeasible | **Done** | R4. |
| ROOT-5 | bounds in render-owned memory | **Done** | `InputBuiltins { stage }` only. No `OnceCell`, no accessors. Bounds in `PreparedBuiltinsSession.tails`. |
| ROOT-6 | CI release scale | **Done** | `test-release` step added. `check-ci-path-routing.py` green. |
| ROOT-7 | preparation cost | **Done as ruled, but see MJ1** | Once per preparation, reused by the seal. Cost reproduced (8.73 ms per near-top design). #1457 filed. |
| ROOT-8 | D2 across a swap | **Done** | #1269 spec section. GitHub body contains it (checked with `gh`). |

## Mathematics checked independently

- **Spectral bound.**
  - For `m = [[a, b], [c, d]]`, `s1 = (sqrt((a+d)^2 + (c-b)^2) + sqrt((a-d)^2 + (b+c)^2)) / 2`
    holds for either sign of `det`: the two roots swap, and their half-sum is still `s1`.
  - Each operation rounds relative to the `f64` inputs, so the closed form is within `(1 +- u)^4`
    of `s1(m)`.
  - Weyl gives `s1(M) <= s1(m) + ||E||_2 <= s1(m) + ||E||_F`.
  - The worst entry `sqrt(2) be + de - al - r ga` has at most 5 roundings per term (`gamma_6`
    covers them), against the stated 16 u.
  - Final factor `(1 - u)^6 (1 + 16 u) > 1`.
  - `R A R^-1 = [[al + r ga, sqrt(2) be + de - al - r ga], [r ga, de - r ga]]` checked by hand.
  - `state_rounding`'s zero-error call: a non-negative matrix's norm is monotone in its entries, and
    `SLACK` covers the `O(u)` entry rounding.
- **Majorant recursions.**
  - Later sections: `(1 - u)^3 (1 + 4 u) >= 1`, an upper bound by induction.
  - First section: error radius and output rounding hold (NITs above on the text).
  - Long sums: recursive summation is at least `(1 - (n-1) u)` times exact. `1 + 2 n u` with
    `n = frame + 1` covers every chain (blocks, suffixes, replay, suprema).
  - `1 - q` is exact for `q` in `[0.5, 1]`.
  - `free_decay_frames`'s `+2` covers the `log` rounding (under 1e-5 frames at 2^26).
- **Other `f64` steps.** Every other step is a short non-negative evaluation, and the per-step
  `2^-30` covers it. The compounding is conservative only. Live module minus my recomputation:
  +17 (`T_decay`), +14 (`R(P*)`), +18 / +42 (rests). The expected `n 2^-30 / (1 - rho)` is 16-47
  frames.
- **Fixed figures move as predicted.** Example: HPF max-1ulp +24 dB is 397,440. The attempt-3
  verifier predicted 397,448 with the stable norm and `SLACK` on `q`; removing `2^-30` from `q`
  accounts for the -8 frames.

## Implementer's open items: rulings

1. **Seal live rule untested.** A test is feasible and cheap: m1, demonstrated. Required in the
   spirit of R3.
2. **Gate 7 has no discriminating claim.** Accepted. The test reaches 10M only through a
   rho_settled margin cut about 4x, and larger cuts give `NotContracting` or `Horizon` (Infinite,
   caught by the finiteness checks). B5 (ramp contraction for the whole decay, 1,263,135) is red only
   on the oracle's upper side, as the restated line says.
3. **The oracle's lower side mirrors the derivation.** Accepted as described. It is an independent
   re-implementation of the derivation's propagation from the module's envelope. It catches
   implementation slips, not derivation errors. The derivation is covered by 1(a)'s brute force,
   gate 2's real kernel and the checks above.
   - Its envelope check uses only `kappa u rho`, which is 1/7 of `mu_state`. A partial rounding
     undercount is still caught by 1(b): B1 is red on 1(b) too.
4. **Oracle tolerance.** The lower side is exact (`module >= recomputed`), so an optimistic defect of
   one frame is red. The upper side is 0.01 % + 64 frames, about 5x the derived `2^-30` compounding.
   It admits only a looser, still sound, bound.
   - Red only on the oracle: drop ramp allowance (B2), drop stall (B3, `P*` 0), ramp rho for the
     whole decay (B5).
   - Red on the oracle and elsewhere: drop rounding (B1, also 1(b)), drop A9 (B4, also 1(c)(i)).
5. **Preparation cost and realtime.**
   - No bound is computed in any render callback: `audit capi` 0 allocations, deallocations and
     syscalls; `check-realtime-policy.sh` green.
   - In the browser the whole preparation runs on the AudioWorklet rendering thread, outside
     `process()`. There the design bounds are pure waste (MJ1).
   - On the C ABI, the cost lands on the caller's control thread. #1457 owns that budget; its 9.6 min
     extrapolation agrees with my 8.73 ms per design.
6. **Error-bound re-derivation and sweep.** Above. Sound. Two proof-text NITs.
7. **Re-pins.**
   - `resources.jsonl`: +96 B/track in both `engine_owned_*` counts, `maximum_single_allocation_bytes`
     +0 (1,080 / 4,320 / 70,779,960 unchanged).
   - `fixture_builtins.rs`: constants 712/696/1080 are pre-#1329's. The tail entry is 72:
     `(Box<str>, InputSectionBound)` = 16 + 2x16 + 24.
   - `MANIFEST.tsv` and the manifest identity `1a8fd9a1…`: `check-builtins-fixtures.sh` (50 files)
     and the audit tests are green.
   - Graph fixture `finite:10048` is unchanged; `graph_fixture --check` is green.
   - Each reason is correct.
8. **Worklet bytes.**
   - The shipped module changes: `d7665dbb…` at c7f7bbdfb, not the recorded `1a833326…` (NIT).
   - No audio bit moves: wasm G5 corpus 0 mismatches (143 cases, 252 comparisons), browser
     expected digests and rows agree, builtins PCM fixtures green, `audit capi` `pcm_digest`
     `cb10fbface44a3a4` the same as attempt 3's.

## Test-value sentences (new and rewritten tests)

- `math::tail::tests::the_operator_norm_bounds_the_exact_norm_of_every_near_top_design`: an
  operator norm whose `f64` evaluation cancels falls below the exact norm of the 44.1 kHz HPF
  max-1ulp design (A1: `0.99994766942311031` against `0.99994767541470919`). All nine
  `tail_contract` tests stay green under A1. It does not guard the perturbation term (A2 green),
  which is not load-bearing on launch-rate designs: without it the margin is still 6.6e-16.
- `each_strip_is_bounded_by_its_own_design_when_designs_are_shared`: a design key that drops the
  trim magnitude or the left channel's HPF or LPF words hands a strip another design's bound
  (C1, C2, C4 red). It is blind to a key without the right channel (C3, m2).
- `live_tail_every_peak_is_the_rest_at_the_flush_floor`: a live `T_rest` that is not exactly
  `max(T_decay, R(P*))` fails. Its catch that no other test makes is an offset smaller than the
  oracle's tolerance. Dropping `R(P*)` (B6) is also caught by the oracle (m3).
- `live_bound_carries_every_term_an_independent_recomputation_requires`: a live bound without the
  ramp allowance (B2), without the flush stall (B3) or with the ramp contraction for the whole decay
  (B5) is red here and nowhere else. It also catches B1, B4 and B6.
- `live_bound_holds_on_the_real_kernel_at_the_domain_extreme` (rewritten): a gate-2 history that the
  non-finite recovery resets (attempt 3's 1 kHz start at `1e29`) is refused (D1 red). Its other
  claims are as stated in attempt 3 (M6, M7).
- builtins-compiler `live_input_lane_reports_the_live_bound_and_plain_input_its_own` (rewritten): a
  preparation that keeps a live strip's design bound reports it through `input_bounds()` (E1 red).
  Its new `validate_for_session(...).is_empty()` line discriminates nothing (E2 green, m1).
- builtins-compiler `prepares_three_sections_and_each_named_meter_tap` (rewritten): unchanged in
  value. Plain strips that report `Finite(0)` fail (attempt 3).
- `a_prepared_bound_is_stored_only_for_its_own_design` was deleted with its mechanism. That is
  correct (superseded test deleted in the same change).

### Mutations run here (scratch copy, release unless noted; `evidence/mutations.txt`, `evidence/mut/`)

| # | mutation | result |
|---|---|---|
| A1 | attempt 3's cancelling norm | unit test RED; all 9 `tail_contract` GREEN |
| A2 | perturbation term dropped | unit test GREEN (term not load-bearing, verified) |
| A3 | first-section error radius dropped | all GREEN (expected: no test can see it) |
| B1 | live envelope drops state rounding | oracle RED, 1(b) RED, others GREEN |
| B2 | live envelope drops ramp allowance | oracle RED only |
| B3 | live bound drops flush stall | oracle RED only (`P*` 0 against 1.7349e-3) |
| B4 | A9 dropped | oracle RED (1,077,903 against 1,081,750), 1(c) RED |
| B5 | ramp contraction for the whole decay | oracle RED only (1,263,135 against 904,768); gate 7 GREEN |
| B6 | live `T_rest` drops `R(P*)` | identity RED, oracle RED |
| C1 / C2 / C4 | key drops trim / LPF / HPF words | `each_strip_…` RED |
| C3 | key drops the right channel | **all GREEN** (m2) |
| D1 | gate 2 history starts at 1 kHz | gate 2 RED (history overflowed at `1e29`) |
| E1 | preparation: live strip keeps design bound (debug) | `live_input_lane_…` RED |
| E2 | seal check skips its live rule (debug) | **GREEN**; RED with my 6-line probe (m1) |

## Gates run (all green unless stated)

- **Release `tail_contract`.** `cargo test --locked --release -p builtins --features
  builtins/test-support --test tail_contract -- --include-ignored`: 9/9, 12.7 s.
  - 1(a): 112 rows, max ratio 1.0977 (96 kHz 10 Hz into the max, +24 dB). Tightest headroom 4.07 %
    below `T_b(eps/32)`.
  - 1(c): 112 rows plus the live identity at four rates.
  - Gate 2 and gate 3 figures as recorded. Gate 7 headroom 74.1-91.0 %.
  - CI's step (without `--include-ignored`) runs gate 2 too: it is ignored only in debug.
- **Debug suites.**
  - `test-debug-a`: 1,441 passed, 0 failed, 10 ignored.
  - `test-debug-b`: 887 passed, 0 failed, 25 ignored. It includes the new `math` unit test, and
    `tail_contract` debug runs 8 tests with 1 ignored.
  - `conformance_fixtures --check`; `cargo test -p builtins-compiler --no-run` (default features).
- **Lint and docs.** `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets
  -- -D warnings`; `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- **Policy scripts.** `check-workspace-policy.sh`, `check-realtime-policy.sh` (89 regions),
  `check-builtins-policy.sh`, `check-dsp-research.sh`, `check-effect-contract.sh`,
  `check-lane-policy.sh`, `check-ci-path-routing.py`.
- **Release audit and fixtures.**
  - `cargo build --release -p audit` and `-p capi`.
  - `audit capi`: 0 allocations, 0 deallocations, 0 syscalls, 0 violations, `pcm_digest`
    `cb10fbface44a3a4`.
  - `check-builtins-fixtures.sh` (50 files); `graph_fixture --check`; `check-graph-determinism.sh`
    (100/100).
  - `check-capi-abi.sh` and `--self-test`; `check-scalar-oracle-absent.py --native`.
  - `cargo test --release -p audit -p bench -p console-workload`: 113 passed, 0 failed.
- **Cross targets.** `check-cross-targets.sh`: PASS. builtins `memset_pattern16` 5 calls, which is
  the expected #1018 row.
- **Worklet chain.**
  - `build-web-audioworklet.sh --named-twin`: shipped module `d7665dbb…`, 3,052,846 B.
  - `strip-wasm-names.py --self-test` and `check`.
  - `check-web-audioworklet.sh --without-metadata-regeneration`.
  - `check-browser-expected-resources.py --artifacts`.
  - `check-scalar-oracle-absent.py --wasm`.
  - `test-web-audioworklet.sh`.
- **Wasm gates.** `run-wasm-gates.sh --without-v8-spill --without-native`: G5 0 mismatches.
- **Extra.** The 661,869-design `q` sweep (four rates), the box-norm check, the bit-exact replica,
  native preparation-cost probes, and V8 boot timings for three modules.
- **Not run.** AArch64 execution (CI only); the `artifact-identity` CI comparison.

## What a fifth (last) attempt needs

1. **MJ1.**
   - Compute design bounds only for strips without a live input lane.
   - Add a counted test: with every strip live, no design bound is computed.
   - Re-measure the browser boot against pre-#1329 (`rebuild-round`, the three 64-track documents)
     and record it.
   - Restate #1457's context.
2. **m1.** Add the seal live-rule test: forge the payload and seal entry of a live strip; expect
   `builtin.prepared.tail_set`.
3. **m2.** Add a strip to `each_strip_…` that differs only in its right channel.
4. **m3.** Correct the live-identity test-value line.
5. **NITs.** The `nu` and `STEP_UP` proof text, the final module digest in the record, and the
   superseded test-value lines.

No other change. The bounds, figures, fixtures and re-pins are verified and should not move. If
attempt 5 touches nothing in `math` or `builtins`, gates 1-3 and 7 re-run unchanged.

## Items for ROOT

- **ROOT-A.** MJ1 changes the premise of #1457. With design bounds skipped for live strips, the
  browser (every strip live) computes no design bound, and the pathological browser stall goes.
  #1457's worst case is then the C ABI's control thread. Amend #1457's "Where preparation runs".
  Also confirm that MJ1 is in #1329's scope rather than #1457's: it is a regression this slice
  introduces on the browser path, +75 % per 64-track boot.
- **ROOT-B.** `crates/host-core/src/prepare.rs:372` (the `HostLiveLanes::strip_input` doc) is stale
  since #1329 and outside its authorized paths. Authorize the one-line fix in attempt 5, or a
  follow-up.
- **ROOT-C.** Spec lines 555-565 keep the original Gate 2, 3 and 7 test-value claims unmarked above
  their R3 restatements. Mark them superseded (root's document).
- **ROOT-D.** `artifact-identity` will report `d7665dbb…` for c7f7bbdfb. The record's
  `1a833326…` is feefbe166's. A record-only commit that moves source line numbers changes the
  shipped bytes, through panic locations.
- **ROOT-E (optional).** `docs/rulings/effect-floor-accounting.md:473` cites
  `InputBuiltins::tail()`, which no longer exists. It is historical text; it was already inexact
  before #1329.
- **ROOT-F.** This is attempt 4 of 5. Scope attempt 5 to the list above so that it cannot become a
  disguised sixth retry.
