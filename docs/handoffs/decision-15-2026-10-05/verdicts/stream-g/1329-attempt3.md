FAIL

# #1329 attempt 3 -- adversarial verdict (Sol verifier, 2026-10-06)

Reviewed: `git diff 0725a8949 6a2c5216a` on `codex/d15-stream-g` (1df306d97 spec addendum,
11f59fa42 implementation, 6a2c5216a evidence), against the spec with Amendments 1-3 and the
Amendment 3 addendum, AGENTS.md, decision 15 D15-4 and the owner's no-shortcuts rule. Work was done
on an export of 6a2c5216a (`/tmp/claude-1002/v1329/tree`) and a scratch copy for probes and
mutations; nothing was edited, built or committed in the worktree. Small evidence (scripts, logs,
dumps) is kept in `/tmp/claude-1002/v1329/evidence/`.

One MAJOR finding fails the attempt. Everything else is MINOR, NIT or an item for root. The fix for
the MAJOR finding is small (one function plus re-recorded figures) and moves no pinned byte.

## BLOCKER

None.

## MAJOR

**M1. The fixed-design certificate rests on an `f64` operator norm that is not certified.**
`crates/math/src/tail.rs:174-179` (`spectral_norm`, used by `v_operator_norm` and so by
`SectionConstants::of`, which supplies `q` for every fixed design) computes
`sqrt(0.5 (F + sqrt(F^2 - 4 det^2)))`. Every design's `R A R^-1` is close to a scaled rotation,
which makes the two singular values almost equal. `F^2 - 4 det^2` then cancels catastrophically,
and the computed `q` can come out *below* the true `||A||_V` by more than the `1 + 2^-30` inflation
(`SLACK`) is meant to cover. Independent 60-digit `Decimal` evaluation
(`evidence/verify_norms.py`), on the kernel's own `f32` words:

| section (words) | exact `||A||_V` | shipped `q` (after `SLACK`) | shipped minus exact |
|---|---|---|---|
| 44.1 kHz HPF max-1ulp (`3f800000 381b3976 3f7ffc92`) | 0.999947675414709 | 0.999947669423110 | **-5.99e-9** (6.4x `SLACK`) |
| 44.1 kHz LPF 1 kHz | 0.904163928707630 | 0.904163927641833 | **-1.07e-9** |
| 44.1 kHz LPF max | 0.999947865737763 | 0.999947867939241 | +2.2e-9 |

When the module is re-run with a numerically stable closed form
(`s_max = (sqrt((a+d)^2 + (c-b)^2) + sqrt((a-d)^2 + (b+c)^2)) / 2`), 179 printed figures change.
Many go **up**, so the shipped values are below the derivation's own bound for those designs
(`evidence/exp-stable-norm.log` against `evidence/tail-release.log`). Examples:

| design | shipped | stable norm |
|---|---|---|
| 44.1 kHz HPF 22049.48 alone, +24 dB, `T_decay` | 397,389 | 397,448 (+59) |
| 44.1 kHz LPF 22049.48 alone, +24 dB, `T_decay` | 397,249 | 397,307 (+58) |
| 44.1 kHz HPF 10 Hz -> LPF 22049.48, +24 dB, `T_decay` | 416,064 | 416,125 (+61) |
| same, `T_rest` = `R(P*)` | 550,795 | 550,873 (+78) |
| 44.1 kHz top pair, +24 dB, `T_decay` / `T_rest` | 478,315 / 485,928 | 478,345 / 485,954 |

Why this is MAJOR and not cosmetic: D3 makes the reported value a certified upper bound, and the
derivation states that the `f64` rounding is "far" inside the `1 + 2^-30` inflation (derivation
line 147). For near-top fixed designs at 44.1 and 88.2 kHz the certificate does not hold as
computed. No gate can catch this. Gate 1(a)'s soundness line has 2-10 % slack, and gate 1(b)
compares designs using the same formula. The real kernel is far inside the bound (for example
last `|y| >= P eps` at 361,775 against 416,064), so no audible or observable D1 violation follows.
The defect is in the proof object this slice delivers.

Not affected:
- The live bound (D5) uses the closed-form `rho(g)` and is unchanged. `input_section_live_bound`
  is bit-identical under the stable norm at every rate.
- Typical designs keep their values. The canonical fixture's 48 kHz 20 Hz -> 20 kHz design stays
  `finite:10048`, so the graph re-pins stand (`evidence/p2-shipped.txt` = `p2-stable.txt`).

Required: replace the formula (or add an explicit a-priori error term), re-record the affected
gate 1(a)/1(c) figures, and correct derivation line 147.

## MINOR

**m1. Gate 2 is weaker than specified, its `P = 1e29` sign-pattern run is vacuous, and the record
misdescribes the run.** `crates/builtins/tests/tail_contract.rs:664` starts every non-quantum-1
run with the HPF at **1 kHz** (`start_hpf = 1_000.0`). Only 32 samples before `N` does a target
retarget it to max-2ulp. For 999,968 of the 1,000,000 history samples the run is therefore not
the D5 domain extreme, yet the spec's record (line 729) calls it the "extreme pair".

At `P = 1e29` this start overflows the state. A probe shows `lifetime_recovered_state` going
(0,0) -> (1,1) -> (2,2) across the two drive blocks, and the state at `N` is all `+0.0`. The
"at rest by `N + 64`" the record leaves "not investigated" is the per-block non-finite recovery,
and the run tests nothing after `N`.

Driving the extreme pair as specified (start max-1ulp, retarget max-2ulp at `N - 32`;
`evidence/probe3.txt`, 44.1 kHz) keeps the state finite (about 4.7e34). It is also harsher than
the shipped runs:

| run | last `|y| >= P eps` | `T_decay` | rest by | bound |
|---|---|---|---|---|
| `P = 1e29` | 437,772 | 904,785 | 2,234,624 | `any_sanitized_input` 2,583,197 |
| `P = +24 dBFS` | 437,809 (shipped 366,584) | 904,785 | 982,592 (shipped 926,865) | `peak_plus_24_dbfs` 1,264,736 |

48 kHz is similar. So the claim holds, but the committed test does not encode the specified run.
Fix: `start_hpf = hpf` (keep the retarget), and correct the record. The alternating +-1e29 run
that was added is non-vacuous and is red on mutation M7 below.

**m2. The live `T_rest` has no test.** `crates/math/src/tail.rs:766`
(`tail_every_peak: tail_relative.max(rest_star)` in `envelope_cascade`) can drop its `R(P*)` term
with all seven `tail_contract` tests green (mutation M9). Gate 1(c)(i) checks the formula only for
fixed pairs, and 1(b)'s sampled-design comparison stays satisfied because fixed `T_rest` values are
far below the live `T_decay`. Add the 1(c)(i)-style identity for the live cascade
(`T_rest == max(T_decay, rest at P*)`, `T_rest >= N_SILENCE`).

**m3. Derivation and research-note inaccuracies.**
- `docs/derivations/1329-input-section-tail-and-rest.md:53` says the native x86-64-v3 and AArch64
  kernels fuse multiply-add. They do not: `Lane::fma` rounds twice on every backend
  (`crates/lane/src/wide_impl.rs:220`, `scalar.rs:86`). The count matches the kernel exactly, so
  the bound is unaffected.
- Line 147 says the `2^-30` inflation is "far below one sample". It compounds through
  `power_apply`. An independent frame-by-frame recompute of the live bound
  (`evidence/verify_live.py`) gives `T_decay` 904,768 against the shipped 904,785, and every rest
  figure 14-42 frames lower, all on the conservative side.
- `dsp-research/filters.md:25` cites the 44.1 kHz top pair at 0 dB as 420,121. The shipped value
  is 420,130.

## NIT

- The gate 2 `-0.0` item cannot be met at this node: every section's output mix (and the
  identity's trailing `+0.0`) normalizes `-0.0`. A probe of polarity-inverted HPF-only, LPF-only and
  pair runs saw no `-0.0` after rest. The test accepts `+-0.0`, which is all D2 needs. The spec
  wording should change (see ROOT-4).
- `CascadeBound::tail_reference` (the exact half) equals `tail` for the live bound, where no
  reference split exists. It is evidence only; a doc line would avoid confusion.

## Rulings on the six points

1. **`t0` one frame conservative: accepted.** D1 needs `T >= T_b(eps/2) - 1`; stating `t0` puts
   the bound on gate 1(a)'s index at a cost of one sample. It is sound, documented in the code and
   the derivation, and not an interim shortcut. Single sections measure ratio 1.0000.
2. **D7 timing and the `OnceCell`: accepted as built, with a design note for root (ROOT-5).**
   - *Reachability.* A grep of `crates/`, `hosts/` and `tools/` finds exactly one production
     caller of `InputBuiltins::{tail, tail_every_peak, rest}` / `BuiltinChain::*`:
     builtins-compiler preparation (`lib.rs:3607`). There the chain is built by
     `with_prepared_bound` with the same parameters, so the key always matches and the lazy path is
     never taken. Render never calls these accessors, and `audit capi` reports 0 allocations and
     0 syscalls. The lazy path is reachable only through `BuiltinChain::new` from tests and tools.
   - *Phase-two account.* The bounds are computed before `TestPhaseTwoAllocationGuard`;
     `test-debug-a` is green.
   - *Shortcut?* The `OnceCell` is a complete mechanism, not a placeholder.
   - *Two residual costs.* (a) A pub accessor on a render-owned type hides an allocating,
     O(T) computation with no structural guard against a future render-side call. (b) The 72 bytes
     sit in both the scalar strip and the bank input: +160 B per track of engine-owned payload,
     +10.49 MB (+7.9 %) at 65,537 tracks, for data render never reads.
   - D7 itself mandates storage in `InputBuiltins`, so I do not charge this to the implementer.
3. **`CanonicalFpEnv` around the bounds: accepted and necessary.** It restores the caller's
   control and status word bit for bit; the host-core `fp_environment` test is green in
   `test-debug-a`. It also makes the bound independent of a host's FTZ/DAZ, which the
   cross-target-identical canonical plan text needs. It is a no-op on Wasm, as intended.
4. **Edits outside the authorized paths: all mechanical, but they need root ratification
   (ROOT-1).**
   - `tools/audit/src/builtins_graph.rs` is **necessary**: the joined-corpus manifest identity moves
     with `resources.jsonl`, and #1451 (969ed73df) edited the same constant.
   - `crates/builtins/src/filter_response.rs` is **not necessary under the shipped design**. With
     the original file restored, all 11 `filter_response` tests (including
     `malformed_grids_shapes_and_budgets_are_atomic_and_allocation_free`) pass in debug and
     release, because `prepare_sections` no longer computes a bound. The edit is
     behaviour-preserving (the fader domain is still checked in `prepare_input_track`). Either
     revert it or ratify it.
   - Two outside-path edits were not flagged: `fixtures/builtins/v1/MANIFEST.tsv` and
     `fixtures/graph/MANIFEST.tsv`. They are generator companions of authorized regenerations.
5. **Gate 2 evidence gaps.**
   - The `-0.0` item is infeasible at this node (NIT; spec wording).
   - The `1e29` sign-pattern run is vacuous because of the test's own 1 kHz start (m1). The
     mechanism is the non-finite recovery, now identified.
   - The added alternating +-1e29 run is real and catches M7.
6. **Weaker test value than the spec claims: confirmed; it is the spec's claim that is wrong, not
   the implementer's work (ROOT-3).**
   - Gate 2 is red only for gross under-bounds (M6 one-section, M7).
   - Gate 3's lower side does not catch a dropped A9 term (M2 green on gates 2 and 3); only
     1(c)(i) does (M2 red).
   - Gate 7 is the only upper bound on the live `T_decay`/`T_rest`, but no single plausible defect
     I tried reaches 10,000,000 (M9 green).

## Re-pins (each checked individually)

- **Graph fixture (`direct-route.*`)**: `infinite` -> `finite:10048` on the post-input-builtins
  `node` and `tail` rows and the DOT label only; `graph_fixture --check` is green.
- **`ZERO_DELAY_CANONICAL_SHA256`**: I dumped the zero-delay canonical text from the shipped code.
  It hashes to `60cae21e...fc6c362`. Exactly 18 `finite:10048` tokens appear (nine `node`, nine
  `tail`, all post-input-builtins). Reverting those 18 tokens alone hashes to the previous pin
  `bf2dfd6c...ad7b10d8b3`, so the reason in the doc comment is correct.
  (`evidence/zero-delay-canonical.txt`)
- **`resources.jsonl`** (field by field, all nine rows): both `engine_owned_*` payloads move +160
  per track (65,537 tracks: +10,485,920). This is +72 strip preparation, +72 bank input entry and
  +8 x 2 tail entries (`(Box<str>, TailSamples)` is 32 B against 24 B before).
  `maximum_single_allocation_bytes` moves +72 per track (the strip vector). The size delta is
  correct, but gate 5 did not predict it (ROOT-2).
- **`fixture_builtins.rs` constants** (784 / 32 / 768 / 1152) and the **`builtins_graph.rs`
  manifest identity**: consistent with the above; `check-builtins-fixtures.sh` is green (50 files),
  and every PCM, meter, response and benchmark fixture digest is unchanged.
- **`BuiltinTail` deletion**: no remaining reference. builtins-compiler and graph-compiler take
  `TailSamples` directly; the `TailChanged` corruption case is adapted (`Finite(n)` ->
  `Finite(n + 1)`).
- **Shipped worklet module**: its bytes change because the bound code is linked into preparation.
  Audio bits do not move: wasm G5 digests (`run-wasm-gates.sh`), the builtins PCM fixtures and the
  conformance and effect-contract fixtures are all green, and the worklet chain (below) checks the
  browser expected resources and native parity.

## Mathematics checked independently

- **Exact Butterworth V-norm.** `R A R^-1` is a scaled rotation for exact designs (Decimal, g =
  0.01, 1, 1000) and `||A||_V = rho(g) = sqrt(1+g^4)/(1+sqrt2 g+g^2)`. The closed forms
  `||b||_V = 2g/sqrt(1+t)`, `||c||_V* = sqrt2/sqrt(1+t)` (both mixes) and `|d| <= 1` are verified
  by hand.
- **Rounding count.** `mu_state = R_NORM ||G|| R_INV_NORM + kappa u q`, with `fl(n)` contributing
  `2(1+u)|d - d_e| + u|n_e|`, matches the frozen two-rounding kernel; `7 u kappa = 1.0073e-6` at
  the top. The output-mix count is also checked.
- **Reset-aware reference.** For any reset pattern, a reset drops terms of
  `sum_{t>m} Gbar_k(t)`. The second-section majorant equals the triangle bound (substitution
  checked). The deviation recursion and the `P*` budget give `< P eps` from `T_decay`.
- **Rest.** It is sequential, uses `limit = REST_EPS/||R^-1||` with the `F` drive, and is monotone
  in `P`, so `R(P) <= R(P*)` for `P <= P*`. `F` is needed because the per-word flush can raise
  `||.||_V`.
- **D5 envelope.** Recomputed in Decimal: `rho(g_max) - 1` = -5.214238e-5 / -5.242522e-5,
  `P(h)` = 2.1585e-7, `E` = (1.96695e-6, 9.71556e-7, 1.96695e-6), `P(E)` = 1.41884e-5,
  `mu_top` = 1.00728e-6. The **margin `1 - rho_ramp` is 3.67309e-5 (44.1, 88.2 kHz) and
  3.70137e-5 (48, 96 kHz)**, and `rho_settled - 1` = -5.091925e-5 / -5.120209e-5. These match G4
  and the code. Convexity of `||A(w)||_V + mu_state(w)` over the hull is valid.
- **Live figures** recomputed frame by frame (`evidence/verify_live.py`) from the envelope
  constants: `P*` = 1.73489e-3 / 1.70862e-3 / 1.73576e-3 / 1.70941e-3, and `T_decay`, `R(P*)` and
  both rests within 14-42 frames below the shipped figures (the conservative `SLACK` compounding).
  The shipped G4 table (904,785 / 1,081,764 / 1,264,736 / 2,583,197 at 44.1 kHz, and so on) is
  reproduced.
- **Single-section check by hand.** The 1 kHz LPF at 44.1 kHz gives `P*` = 8.80e-12 and `R(P*)` =
  78 + 3,764 = 3,842, matching the module.

## Test-value sentences (new and rewritten tests)

- `fixed_design_tail_is_sound_and_within_thirty_db_of_the_exact_tail`: a fixed bound that drops
  the trim gain (M3) or the cascade's second section (M4) falls below the independent brute-force
  `T_b(eps/2)`, and a Putzer-loose bound exceeds `T_b(eps/32)`; no other test computes a tail
  against a brute force.
- `a_prepared_bound_is_stored_only_for_its_own_design`: a `with_prepared_bound` that stores a bound
  for a different rate, words or trim reports another strip's tail (M8 red).
- `live_bound_covers_every_scanned_design_and_ramp_word`: a live envelope taken at the wrong
  extreme is exceeded by a scanned top design (M5 red).
- `tail_every_peak_is_the_rest_at_the_flush_floor_and_holds_on_the_real_kernel`: a fixed `T_rest`
  that drops `R(P*)` (M1) or the A9 term (M2) turns red; it is the only test that catches the A9
  drop. (Live `T_rest` is not covered; see m2.)
- `live_bound_holds_on_the_real_kernel_at_the_domain_extreme`: a live bound that drops the
  cascade's second section (M6) or states `any_sanitized_input` at +24 dBFS (M7) is beaten by the
  real kernel. It is blind to the A9 drop and to rounding, ramp or stall omissions, and its 1e29
  sign-pattern run is vacuous (m1).
- `live_rest_bounds_are_within_the_restated_contract_figures`: a sound but loose live rest bound
  (for example a Putzer-like log factor) exceeds 1.27M/2.59M; upper side only.
- `live_bounds_leave_headroom_to_the_tail_cap`: the only upper bound on the live `T_decay` and
  `T_rest`. A live tail grown past the 10M cap the tools configure turns red. It is weak (M9
  green), and root mandated it.
- builtins-compiler `prepares_three_sections_and_each_named_meter_tap`: plain strips reporting
  `Finite(0)` at both D7 sites fail (`Finite(0)` against `Finite(10048)`; re-run red here).
- `live_input_lane_reports_the_live_bound_and_plain_input_its_own`: a live lane not reporting the
  live bound, or a disabled plain strip not `Finite(0)`, fails (implementer's run; not re-run).
- graph-compiler `builtins_replace_only_the_three_internal_track_bindings` and the `track_delay`
  digest: a graph that rewrites the prepared builtin tail on the node or the output fails
  (MUTATIONS.md 964-1/964-3 rechecks; not re-run).
- host-core `live_lanes.rs` (`Finite(2 * live T_decay)`): a host path that reports `Infinite` or
  composes the live tail once instead of along the two-strip path fails.
- Superseded `input_tail_is_infinite_while_a_filter_target_is_ramping` is deleted, as gate 4
  requires.

### Mutations re-run here (scratch copy, release unless noted)

| # | mutation | test(s) | result |
|---|---|---|---|
| M1 | fixed `T_rest` drops `R(P*)` | 1(c) | RED (`175` against `3842`) |
| M2 | A9 term dropped from every rest | 1(c) / 2 / 3 | RED / GREEN / GREEN |
| M3 | fixed bound drops trim gain | 1(a) | RED (`T_b 20253 > T 17408`) |
| M4 | fixed bound drops the second section | 1(a) | RED |
| M5 | live top group at `g_min` | 1(b) | RED |
| M6 | live bound with one section | 2 | RED (gate-3 lower-side run) |
| M7 | `any_sanitized_input` at +24 dBFS | 2 | RED (alternating 1e29 run) |
| M8 | `with_prepared_bound` ignores the key | prepared-bound test | RED |
| M9 | live `T_rest` drops `R(P*)` | all seven `tail_contract` tests | **all GREEN** |
| -- | both D7 sites report `Finite(0)` (debug) | `prepares_three_sections_...` | RED |

## Items for ROOT (scope and authorization, not implementer defects)

- **ROOT-1.** Ratify or reject the outside-path edits:
  - `tools/audit/src/builtins_graph.rs` (necessary)
  - `crates/builtins/src/filter_response.rs` (unnecessary under the shipped design;
    behaviour-preserving; revert or ratify)
  - `fixtures/builtins/v1/MANIFEST.tsv` and `fixtures/graph/MANIFEST.tsv` (generator companions;
    not flagged by the implementer)
- **ROOT-2.** Gate 5 said only `engine_owned_*` counts move. `maximum_single_allocation_bytes`
  also moves (+72 per track, the same size delta through the strip vector). Acknowledge, or amend
  the gate.
- **ROOT-3.** The spec's "Test value" lines overclaim for gate 2 (rounding, ramp and stall
  omissions), gate 3's lower side (the A9 drop) and gate 7. Mutations confirm all three. Amend the
  claims, or ask for discriminating tests (the m2 identity is one cheap addition).
- **ROOT-4.** Gate 2's "polarity inverted, so `-0.0` is reached" is infeasible for the builtin
  input section, whose output normalizes to `+0.0`. D2's `-0.0` remark does not apply to this node.
- **ROOT-5.** D7 stores control-plane-only bounds in a render-owned type: 72 B x 2 per track
  (+7.9 % engine-owned builtins payload at 65,537 tracks), behind a lazy, allocating, O(T)
  accessor. Root may prefer to keep the bounds beside `tails` in `PreparedBuiltinsSession`, which
  #1107 will read anyway, and drop them from `InputBuiltins`.
- **ROOT-6.** No CI job runs `tail_contract` at release scale. Gate 2 and 1(c)(ii) never run in
  CI, and 1(a)/(b) run only at 48 kHz and reduced scale in debug. `qualification.yml` is outside
  this slice's paths. The release step costs about 19 s (as `filter_liveness` has in #1428), and
  under the no-shortcuts rule it should land with this slice.
- **ROOT-7.** Preparation cost: each distinct design costs O(T) (implementer: about 145 us typical,
  up to about 10 ms at the top of the domain). It is paid twice per preparation (`prepare` and the
  seal check's `expected_tails`). In the browser it runs on the AudioWorklet thread at processor
  construction. There is no budget or issue for it.
- **ROOT-8 (carried open item).** D2 across a plan swap that carries non-zero integrators into a
  disabled section (#1407 rule 4) is for #1269 to state.

## Gates run (all green unless stated)

- `cargo test --locked --release -p builtins --features builtins/test-support --test tail_contract
  -- --include-ignored`: 7/7, 6.8 s. 112 gate 1(a) rows, maximum ratio 1.0977 (96 kHz, 10 Hz ->
  max, +24 dB); 112 gate 1(c) rows; gate 2, 3 and 7 figures as recorded.
- `test-debug-b` command: green; `tail_contract` debug 6 passed, 1 ignored.
- `test-debug-a` command: 1,441 passed, 0 failed, 10 ignored (includes `fp_environment`,
  `live_lanes`, `track_delay`, the builtins-compiler and graph-compiler tail tests).
- `cargo fmt --all -- --check`; `cargo clippy --locked --workspace --all-targets -- -D warnings`;
  `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.
- `check-workspace-policy.sh`, `check-realtime-policy.sh` (89 regions), `check-builtins-policy.sh`,
  `check-dsp-research.sh`, `check-effect-contract.sh`.
- Release `audit` + `capi` build; `audit capi` (0 allocations, 0 deallocations, 0 syscalls,
  0 violations); `check-builtins-fixtures.sh . target/release/audit` (50 files);
  `check-graph-determinism.sh` (100/100); `graph_fixture --check`; `check-capi-abi.sh` and
  `--self-test`; `check-scalar-oracle-absent.py --native`.
- `check-cross-targets.sh` (PASS; builtins `memset_pattern16` 5, within the script's expected set);
  `run-wasm-gates.sh --without-v8-spill --without-native`.
- Worklet chain, all green:
  - `build-web-audioworklet.sh --named-twin`: shipped module
    `e25b045d...90d9f652`, 3,049,623 B
  - `strip-wasm-names.py --self-test` and `check`
  - `check-web-audioworklet.sh --without-metadata-regeneration` (boot budget, static and object
    checks)
  - `check-browser-expected-resources.py --artifacts` (expected digests and exact rows agree with
    the built module)
  - `check-scalar-oracle-absent.py --wasm`
  - `test-web-audioworklet.sh`
- Not run: AArch64 execution (CI only); the `artifact-identity` comparison against main's recorded
  digest (needs the CI status).
