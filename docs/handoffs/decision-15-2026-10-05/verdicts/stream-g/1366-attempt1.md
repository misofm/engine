PASS

# #1366 attempt 1 -- adversarial verdict

Commit `31a2132c6` on `codex/d15-stream-g` (parent `8439972be`, #1328 attempt 1). Reviewed from
an export of the commit (`git archive`), built and tested in `/tmp/claude-1002/v1366/`. Nothing in
`/home/bl/misofm/wt-d15-g` was touched.

D1-D4 are implemented as frozen. Only authorized paths are touched. Gate 1 holds when I re-run
it, and gate 2 holds too: no rendered bit and no shipped code byte moves. The test-value claim
survives my own mutation runs across the whole CI debug workspace. No BLOCKER or MAJOR finding.

## Findings

### MINOR

1. **The shared module states only part of `RAMP_PATH_NORM_TOLERANCE`'s precondition.**
   `crates/effect-runtime/src/svf.rs:3-5` says the multiband's words "satisfy the same
   preconditions (`c1, a2, a3 < 1`), so one definition serves both". The derivation at `:46-55`
   also depends on two things:
   - a walk of at most `RAMP_SAMPLES` = 64 steps (the `65 * 4 * 2^-25` term at `:48`);
   - an exact power-of-two step scale ("the scaled step lands within about `u` of the target").

   `RAMP_SAMPLES` is a private EQ constant (`crates/parametric-eq/src/lib.rs:118`). From
   `effect-runtime` it is now an unexplained code span (`:54`). With an N-sample ramp the bound
   needs `2^-22 + (N+1) * 2^-23 <= 2^-12`. That holds up to about 2,000 samples, but the stated
   30x margin holds only at 64.

   This is not a defect today. The only consumers are the EQ and #1338, and #1338 D4 adopts
   64-sample power-of-two ramps. But this is now a public shared constant, and a third consumer
   with a longer session ramp would read the module doc and wrongly conclude the bound applies.

   The spec asked for a verbatim move and itself asserted "the derivation uses only
   `c1, a2, a3 < 1`", so this is a spec-level gap, not an implementer error. **Fix:** state the
   64-step, power-of-two-scale precondition in the module and constant docs (fold into #1338 or a
   doc follow-up).

2. **No automated job runs the exhaustive sweep.** `crates/multiband-compressor/tests/designer_total.rs:89`
   is `#[ignore]`, and neither `qualification.yml` nor `nightly.yml` runs it. Per PR, only the
   1/1024 stride guards the totality that licenses #1338's failure-free render designer. That
   stride is `:84`, run by test-debug-b.

   The repo has a precedent: `qualification.yml` test-release runs the math M1 and F1 exhaustive
   sweeps with `--ignored` under `math_closure` routing. This sweep costs 5.7 s in release.

   Today's margins are large:

   | Check | Worst value | Limit |
   |---|---|---|
   | Check-back dB error | 8.2e-7 | 0.005 |
   | a3/a2 relative error | 2.0e-7 | 1e-4 |
   | Norm excess | 5.4e-8 | 2.4e-7 |

   So the risk is low. The spec chose `#[ignore]`, and workflows are outside the authorized paths.
   **Recommend:** a follow-up that wires the release sweep into test-release, routed on
   `math`/multiband changes, so the proof re-runs when `math::tan` or the designer moves (owner
   principle: no shortcuts).

### NIT

3. `crates/effect-runtime/src/svf.rs:42`: the moved text cites "(`tests/carry.rs`)". In
   `effect-runtime` that path resolves to a file that does not exist; the file is
   `crates/parametric-eq/tests/carry.rs`. This is a consequence of the verbatim-move requirement.
4. `crates/parametric-eq/tests/MUTATIONS.md:49`: row 18 names `src/lib.rs` as the home of
   `word_spectral_norm`'s body, which now lives in `crates/effect-runtime/src/svf.rs`. It is a
   historical record of a past run and outside the authorized paths, so leaving it is acceptable.

## Implementer-declared deviations

- **(a) `design_lr4` made `pub`: accepted.** The spec puts gate 1 in an integration test
  (`tests/designer_total.rs`), which can only call public items. The signature and behaviour are
  unchanged. The crate already exposes `lr4_coefficients`, `lr4_step` and `Lr4State` publicly for
  tests and has no `test-support` feature. `#[must_use]` is harmless, and clippy is clean.
- **(b) The intra-doc link became a code span: accepted and correct.** rustdoc cannot resolve a
  private constant of the downstream `parametric-eq` from `effect-runtime`. A diff of the moved
  doc block against the original shows this as the only changed token. Prose cleanup is NITs 1
  and 3.
- **(c) The worklet module sha256 changed: verified, handled correctly, no committed update
  needed.** I built both modules with `build-web-audioworklet.sh --module-only --named-twin`:
  - Parent `89a909d0...` vs commit `748efd3d...`. Both are 2,903,816 B, and both named twins are
    3,323,129 B.
  - `wasm-objdump -d` of the two named twins: 1,266,805 lines each, zero diff.
  - Section headers are identical. All 44 differing bytes (offsets 2,840,206-2,852,210) fall
    inside the Data section (2,803,338-2,903,577), as expected for panic-location line numbers.

  The committed pin `hosts/host-web/web/miso-engine-v1-audio-worklet-artifact.sha256`
  (`6c952a2c...`) is the release fingerprint. Per #1061 and `docs/RELEASE.md` ("Nothing re-pins
  the pin ... between releases"), it must not be re-pinned per change. It is correctly untouched,
  and the disassembly comparison is one-time PR evidence, as AGENTS.md requires.
- **(d) `crates/parametric-eq/tests/carry.rs:13` names `RAMP_PATH_NORM_TOLERANCE`: no action
  needed.** The constant still exists under that exact name (`effect_runtime::svf`, imported by the
  EQ at `crates/parametric-eq/src/lib.rs:57` and used at `:2739`), so the prose stays accurate.
  The file is also outside the authorized paths.

## Test value

`designer_total.rs`, both tests: a designer edit that makes the checked `design_lr4` refuse a
legal interior crossover at some launch rate turns it red, and no existing test catches it.
Existing tests sample only 80, 1000 and 8000 Hz, and the digest corpus samples only 1 kHz at
48 kHz. Without this test, #1338's render path would apply words that prepare and restore refuse.

My own mutation runs:

| Mutation | `designer_total` strided | Existing tests |
|---|---|---|
| M2: `a3/a2` consistency tolerance `1e-4` -> `1.5e-7` (passes all 12 sampled points, max 9.1e-8) | RED, "design_lr4 refused 170.625 Hz at 48000 Hz" | Full CI debug workspace (test-debug-a + test-debug-b command lines, `--no-fail-fast`): `designer_total` is the **only** failure |
| M3: check-back dB tolerance `0.005` -> `6e-7` (sampled max 5.7e-7, interior max 8.2e-7) | RED, "refused 2856.0 Hz at 44100 Hz" | test-debug-b set: `designer_total` is the only failure |
| Divergence: `design_lr4` returns `a3` one ulp off `design_lr4_words` | RED, "checked and infallible designs differ at 80.0 Hz" | not run |
| Norm half: stored `c1` word x 1.00001 | RED, "norm 1.00000024 exceeds the tolerance at 2981.0 Hz" | Also caught by `crossover.rs::the_two_stage_split_matches_the_reference_and_recombines_flat`, `wasm-gates` `g5_native_digests_match_pins` and `g6`, so the norm half's unique value is forward-looking (#1338 render; no other test computes crossover contractivity), not a unique catch today |
| `transition_norm` `a11 = 1 - 1.9 a3` | RED | 6 `parametric-eq` `analytic` tests red (D3's delegation is guarded, as the spec says) |

Every mutation was reverted afterwards.

My sweep also re-derived the domain's own margins in release over all 224,919,556 designs:

| Rate | Worst dB error | Worst a3/a2 relative error |
|---|---|---|
| 44.1 kHz | 8.17e-7 | 2.04e-7 |
| 48 kHz | 8.17e-7 | 2.04e-7 |
| 88.2 kHz | 6.12e-7 | 1.74e-7 |
| 96 kHz | 6.12e-7 | 1.74e-7 |

## Other checks

- **D2 bit identity.** `math::sqrt` is literally `f64::sqrt` (`crates/math/src/vendored/sqrt.rs`).
  `transition_norm` is operation-for-operation the old body.
- **Same transition matrix.** The multiband and the EQ both iterate `lane::kernels::svf_step`, so
  the norm is the right predicate for the crossover words.
- **D1.** `design_lr4_words` is byte-identical to the old inline design, and every caller still
  uses `design_lr4`.
- **Realtime.** No render-path change, and `transition_norm` is used only at prepare and restore.
  `effect-runtime` stays `no_std`.
- **Naming.** No versioned or renamed identities.
- **Test policy.** No source grep, no digest re-pin, and `corpus_digests.in` and the EQ render
  contract are untouched.
- **Sweep numbers match the Attempt record.** Exhaustive: 224,919,556 designs, largest excess
  5.402e-8 at 7943.455 Hz, 44.1 kHz. Strided: 219,652 designs, 5.292e-8 at 7984 Hz, 44.1 kHz.

## Gates run (in the export, own `CARGO_TARGET_DIR`)

| Gate | Result |
|---|---|
| `cargo test --locked --all-targets -p effect-runtime -p multiband-compressor -p parametric-eq --features math/lane,parametric-eq/test-support,lane/test-support` | PASS |
| `cargo test --locked --release -p multiband-compressor --test designer_total -- --ignored` | PASS, 224,919,556 designs, 5.67 s |
| `bash scripts/check-parametric-eq-render-contract.sh` | PASS |
| `bash scripts/check-effect-runtime-policy.sh` | PASS |
| `bash scripts/check-cross-targets.sh` | PASS. The "lower its row" notes for builtins and parametric-eq match the parent `8439972be` output exactly, so they do not come from this change |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | PASS, no warnings |
| `cargo fmt --all -- --check` | PASS |
| `bash scripts/check-workspace-policy.sh` | PASS |
| Extra: `cargo test --locked --release -p wasm-gates --features math/lane` | PASS. The native digest owner (G5/G6) is unchanged, so no rendered bit moved |
| Extra: worklet module build at parent and at commit, disassembly diff | Code identical, data-only diff |
| Not run: `run-wasm-gates.sh` V8 spill gate | This is #1328's known red and not a gate of this issue, and the module's code bytes are identical to the parent's |
