PASS

# #1409 attempt 2 -- adversarial verdict

Commit under review: `f6f599d84` (branch `codex/d15-stream-g`), parent `ba05f2887`. Attempt 1 was
`8deee2c5e` (verdict `1409-attempt1.md`, FAIL on one MAJOR). I reviewed the attempt-2 diff
`ba05f2887..f6f599d84` line by line, and the whole #1409 change as it stands at `f6f599d84`
(`8deee2c5e^..f6f599d84` on #1409's paths), against the spec at `f6f599d84`, AGENTS.md, decision 15
(D15-4 and the root decisions that file #1408/#1409) and the no-shortcuts principle. The other
commits in that range (#1411 `4e41b6296`, #1428 `a124b40be`, the #1328 follow-up checkpoint
`4264bb190`, spec-only commits) do not touch a #1409 render site: `4264bb190` edits
`lane/src/kernels{,/builtins}.rs` but not `ramp_toward`, `ramp_block` or `IndexedRamp`.

All builds and mutations ran in a private `git archive` export (`/tmp/claude-1002/v1409/tree`,
`CARGO_TARGET_DIR=/tmp/claude-1002/v1409/target`). Mutations were applied in place to the export
and reverted; the two mutated files were checked against `git show f6f599d84:` by SHA-256 after
every batch. I did not build, edit or check out anything in `/home/bl/misofm/wt-d15-g`.

Attempt 2 fixes the MAJOR. With the delay time at its 1 ms minimum, each site-7 word that attempt 1
could not see (left and right feedback, left and right damping `g`, cross position) now turns the
delay's gate-2 test red when it alone is reverted, and no other delay or conformance test catches
it. The multiband deviation now has exactly the width of its justification. The probe docs no longer
point at the slack. No compiled code changed. Every gate is green. The remaining findings are one
MINOR (the line-width NIT is not fixed, and the record says it is) and four NITs.

## Findings

### BLOCKER

None.

### MAJOR

None.

### MINOR

1. **The attempt-1 line-width NIT is not fixed, and the attempt record says that it is.** The
   record says "Rewrapped to 100 columns" (spec, Attempt 2, NITs). `rustfmt.toml` sets
   `max_width = 100`, and `rustfmt` does not wrap comments, so `cargo fmt --check` cannot see these
   lines. At `f6f599d84`, these #1409 lines are over 100 columns (characters, not bytes):
   - `crates/lane/src/kernels.rs:1561` (111): the `IndexedRamp` rewrap moved the overflow to the
     next line;
   - `crates/multiband-compressor/tests/ramp_endpoint.rs:13` (149): new in attempt 2;
   - `crates/multiband-compressor/src/lib.rs:901` (102) and `:1129` (101);
   - `crates/compressor/src/kernel.rs:961` (101): the joined `advance_where` line;
   - `docs/EFFECT_CONTRACT_V1.md:145` (126): from attempt 1, which the attempt-1 verdict did not
     list.

   The defect is cosmetic, but a record must not state a fix that was not made. Rewrap the six lines
   and correct the sentence. This does not fail the attempt.

### NIT

1. **The delay test's docs say more than the window shows**
   (`crates/delay/tests/ramp_endpoint.rs:7-8`, `:154-157`).
   - The module doc says that "every word reaches the ring". The mix word does not: it writes only
     the output.
   - The `DELAY_TIME_MS` doc says that feedback, damping `g` and cross feedback "reach the output and
     the ring while they move". Within the 72-frame window they reach only the ring and the damping
     state. Their first live writes are at frames 48-63, and these are read back 48 frames later,
     after the window ends.
   - The mutation evidence agrees: each of the five per-word mutants is red at the final-snapshot
     comparison (`:323`), never at the output comparisons (`:321-322`).
   - Say "the ring" for these three words, and "the output" for mix.
2. **The six probe caller docs claim more reach than the probe has**
   (`crates/{compressor/tests/randomized.rs:152, delay/tests/randomized.rs:23,
   gate-expander/tests/randomized.rs:31, multiband-compressor/tests/randomized.rs:75,
   transient-shaper/tests/randomized.rs:31, true-peak-limiter/tests/randomized.rs:34}`).
   - The docs say the probe is "Red when a render site leaves a ramp word outside its endpoints at
     render".
   - The probe sees only the site that writes the word in the scalar instance's snapshot. With all
     seven delay site-7 words reverted to `value + step`, the delay probe stays green (its gate-2
     test is red). The #1411 verdict (NIT 2) found the same for compressor site 2.
   - The intent of the root-authorized fix holds: a red probe points at a clamp or a validation,
     never at a slack. A clause such as "the site whose word the snapshot holds" would make the
     docs exact.
3. **The conformance probe's summary line is still the old premise**
   (`crates/conformance/src/randomized.rs:2660-2661`). It says that the probe samples "a ramp that
   rounds past its parameter's domain edge". Since #1409, no engine ramp does this; only the
   probe's unclamped model walk does. The paragraph under it is now correct. This line is inside
   the root-authorized doc.
4. **Gate 1's 100,000-ramp size still runs in no CI job** (attempt-1 NIT 2). This is recorded
   candidly under Open items as a root decision, because the fix is a workflow change outside
   #1409's paths. I ran the full size by hand again: it passes in 5.98 s.

### Attempt-1 findings: status

| Attempt-1 finding | Status at `f6f599d84` |
| --- | --- |
| MAJOR 1, delay site 7 not reached | Fixed and verified (mutation table below) |
| MINOR 1, multiband `whole` too wide | Fixed and verified (both directions) |
| MINOR 2, soft clip line clause | Closed by #1411 (its verifier confirmed the clause is gone) |
| NIT 1, stale doc text | #1411's own docs, verified there; #1408's NIT 4 statements remain outside #1409 |
| NIT 2, gate 1 size in CI | Open, recorded for root (NIT 4 above) |
| NIT 3, long lines | Not fixed (MINOR 1 above) |
| NIT 4, `Segment` doc split by derive | Fixed (`multiband-compressor/src/lib.rs:647-651`) |
| NIT 5, "first quality row" vs `qualities[1]` | Fixed in all seven copies, with an `assert_eq!(quality.sample_rate, 48_000)` |
| NIT 6, soft clip settled-path cost | One descriptive measurement recorded, honestly caveated (no tuning, no speedup claim) |
| NIT 7, wasm effect-ramp corpus case | Optional follow-up, recorded |
| #1411 MINOR 1, probe docs | Fixed in all seven places (NITs 2-3 above refine it) |

## Review points

1. **No compiled code changed.** Every changed `src` line in `ba05f2887..f6f599d84` is a comment.
   The one non-comment movement is `#[derive(Clone, Copy)]` moving below the `Segment` doc, which
   has no effect on the code. A grep of `git diff -U0 -- 'crates/*/src'` for lines that are not
   comments finds none.
2. **Authorized paths.** Every attempt-2 path is in the spec's list. The seven probe docs are
   recorded as root-authorized (doc comments only), and each hunk stays inside the doc it names:
   `the_effects_own_edge_ramp_snapshots_restore` in the six callers, and
   `EffectDifferential::edge_ramp_restore_violations` in `conformance`. The delay `src` edit is the
   doc of `a_carried_ramp_is_refused_unless_its_whole_path_is_valid`, which is D6's refusal-row
   test.
3. **The delay fix is sound.**
   - Parameter 0 is the per-lane delay time. Its domain is `[1, 2000]` ms (`delay/src/lib.rs:182-192`),
     so 1 ms is its in-domain minimum: 48 samples at 48 kHz.
   - The per-frame endpoint check and the reach check ("the unclamped law leaves the interval") are
     unchanged, and they still pass at 1 ms.
   - Why the D4-wiring mutants are green here: with a 48-sample delay, the frames where the taps
     are live (48-63) start a new chunk, and that chunk is seeded from site 2's clamped word. On the
     feedback move, that word is already at its target, so a clamp toward the wrong target gives the
     same bits there. The existing `tests::partition_invariance_over_1_7_64_128_512`
     (`delay/src/lib.rs:2250`) still catches both D4-wiring mutants, as in attempt 1. No coverage is
     lost.
4. **The multiband deviation now matches its justification.**
   - Set `whole: true`, high ratio and high attack are each red at `left output`
     (`multiband-compressor/tests/ramp_endpoint.rs:315`).
   - The four restored moves (low ratio, low attack, low release, high release) pass the full
     comparison.
   - The spec sentence, the module doc and all seven `Move::whole` docs now name those two moves
     only.
5. **Probe docs.** They now say that since #1409 every ramp word stays between its start and its
   target, so the strict #1411 restore admits each snapshot. They say a red probe means a missing
   or reverted clamp at render, or a restore that refuses a valid snapshot, and that the fix is
   never a slack. This is correct, with the precision limits in NITs 2 and 3.
6. **GitHub.** The #1409 body equals its spec at `f6f599d84` byte for byte (48,504 bytes, OPEN).
7. **Realtime, naming, digests.**
   - No render code changed.
   - The one new name, `DELAY_TIME_MS`, is unversioned and test-only.
   - No digest, byte count or source-grep test was added.
   - Acked-batch question: no queue is touched.
8. **Bits.** `g5_native_digests_match_pins` passes, and the browser `expected.json` digests agree.
   The iOS `memset_pattern16` counts for #1409's crates are the same as at attempt 1: compressor
   970, gate 91, multiband 566, soft clip 22, transient 268, limiter 104, delay 0.

## Mutation evidence (re-done by the verifier)

Each mutant was applied alone, the named targets were run, and the mutant was reverted.

| Mutant (site 7 in `delay_chunk`, unless named) | `-p delay --all-targets` | `-p conformance --all-targets` |
| --- | --- | --- |
| left damping `g`: `gain_left + left.damping.1` | red only in `ramp_endpoint` (`:323`, damping coefficient, final snapshot) | -- |
| left feedback | red only in `ramp_endpoint` (`:323`, feedback) | green |
| right damping `g` | red only in `ramp_endpoint` (`:323`, damping coefficient) | -- |
| right feedback | red only in `ramp_endpoint` (`:323`, feedback) | green |
| cross position | red only in `ramp_endpoint` (`:323`, cross feedback) | green |
| left mix (`--test ramp_endpoint` only) | red (`:321`, left output) | -- |
| all seven words on `value + step` | `ramp_endpoint` red (`:323`, feedback); the delay probe `the_effects_own_edge_ramp_snapshots_restore` green (NIT 2) | -- |
| D4 wiring: feedback clamps toward the mix target | `ramp_endpoint` green; `partition_invariance_over_1_7_64_128_512` red | -- |
| D4 wiring: cross clamps toward its start | `ramp_endpoint` green; `partition_invariance_over_1_7_64_128_512` red | -- |
| multiband high ratio `whole: true` | red (`:315`, left output) | -- |
| multiband high attack `whole: true` | red (`:315`, left output) | -- |

Attempt 1 found all five per-word site-7 mutants green in every delay suite, at the 250 ms default.
The mutated files were confirmed identical to `f6f599d84` after each batch.

## Test value (new or rewritten tests)

- **`crates/delay/tests/ramp_endpoint.rs`, `every_ramped_word_stays_inside_its_endpoints`
  (rewritten).**
  - Plausible defect: one site-7 word (left or right feedback, left or right damping `g`, or the
    cross position) is left on `value + step` in the ramping shape, so that word passes its target
    on a one-block render.
  - The test is red at the final-snapshot ring comparison. Every other delay target and the
    conformance package survive that defect, as verified per word above.
  - The mix word is red at the output comparison.
- **`crates/multiband-compressor/tests/ramp_endpoint.rs`, `every_ramped_word_stays_inside_its_endpoints`
  (four `whole` flags restored).**
  - This is not a new test, and not new coverage I can claim. The restored flags narrow the
    deviation to what is justified, and I verified that both ways (two moves red when set true, four
    green).
  - The test's own value is unchanged from attempt 1: site 6 per word and per channel, and the D4
    bank lane.
  - I did not find a defect that only the four restored output comparisons catch.
- **The `request` precondition (seven copies).** This is an assertion inside the harness, not a new
  test. It turns red if the second quality row stops being 48 kHz, the rate the moves were scanned
  at.
- **The probe and probe-caller docs.** These are doc changes only, and no test logic changed.

## Gates run (export, fresh target dir)

| Gate | Result |
| --- | --- |
| Spec debug command (`cargo test --locked --all-targets -p lane ... -p conformance --features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`) | pass: 169 result lines, 0 failed; seven gate-2 tests, six bank tests and six probes ok |
| `cargo test --locked --release -p lane -p math -p wasm-gates --features math/lane` | pass (`g5_native_digests_match_pins` ok) |
| `--release --test ramp_endpoint` for the seven effects and `effect-runtime` (gate 1 at 100,000) | pass (gate 1 5.98 s) |
| `test-debug-a` workspace command (from `qualification.yml`) | pass (122 result lines, 0 failed) |
| `bash scripts/run-wasm-gates.sh` | pass (native, simd128, V8 spill gate ok) |
| `conformance_fixtures -- --check` | pass |
| Worklet chain (`build-web-audioworklet.sh --named-twin`, `check-web-audioworklet.sh --without-metadata-regeneration`, `check-browser-expected-resources.py`, `test-web-audioworklet.sh`) | pass |
| `check-cross-targets.sh` | pass (memset counts unchanged) |
| `check-lane-policy.sh`, `check-effect-runtime-policy.sh`, `test-effect-runtime-policy.sh .`, `check-effect-contract.sh`, `check-realtime-policy.sh`, `check-workspace-policy.sh` | pass |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | pass |
| `cargo fmt --all -- --check` | pass |

Disk fell to 17 GB free during the run, because another agent was building at the same time. While
it was below 25 GB, I built only the crates I needed. Once it rose again, I ran the remaining gates
behind an 18 GB guard, and no gate was skipped. #1301 gate 5 was not re-measured: attempt 2 changed
no render code, and #1411's verifier measured the probes on this law.

Evidence (logs, mutation script and log): `/tmp/claude-1002/v1409/evidence/`.
