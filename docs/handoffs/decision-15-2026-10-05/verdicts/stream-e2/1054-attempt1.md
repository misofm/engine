PASS

# #1054 attempt 1: verifier verdict (stream E2, decision 15)

Range: `8c0deee2c..adf234b19` (14 commits) on `codex/d15-stream-e2`. I reviewed the export of `adf234b19`. I built it with my own target dir and did not touch the worktree.

There are no BLOCKER or MAJOR findings. Root must correct one evidence claim in the spec's attempt record before close (MINOR 1).

## Findings

### MINOR 1: the attempt record says the `audit capi` pcm_digest did not move, but it did
- Spec, Attempt record, "Digests that moved": "no render digest moved (... `audit capi` `pcm_digest a9738fa800afcab3` ...)".
- Measured with the same release `audit capi` run at each commit:
  - base `8c0deee2c`: `cb10fbface44a3a4`
  - `da9f2ff50` (after the D1a migration, before D4): `cb10fbface44a3a4`
  - final `adf234b19`: `a9738fa800afcab3`, the same on a rerun
- The digest moves at `a575e74fd` (D4/D5). This is expected. `tools/audit/src/capi.rs` commits a live fader/mute/pan edit every 64 calls (`LIVE_EDIT_PERIOD`) on a session that says `default`, so those edits now ramp where they used to step.
- No gate pins this digest (`tools/audit/src/capi.rs:474-563` only prints it). The audit still reports 0 allocations, 0 syscalls and 0 violations.
- The code is correct. The record is not: it gives the final value as "unchanged". The record must say that this digest moved because of D4, on purpose.
- For root: gate 7 says "No render digest moves ... Any other digest that moves is a defect". Its own reason is "preparation is unchanged". So the gate covers prepared renders, not renders that contain live edits, which must change under the product outcome ("live ... edits start to ramp; today they step"). I read this as a gap in the spec, not a defect in the code. If root reads gate 7 literally, root must rule on it.

### MINOR 2: some edits are outside the authorized paths, and only one of them is in the record
- `crates/capi/tests/resource_lifecycle.rs` (deviation b) is recorded and justified. See "Deviations" below.
- These are not recorded:
  - `crates/host-core/src/lib.rs`: the `LiveRampRow` re-export, which D4 needs.
  - `crates/host-core/tests/launch_registry_once.rs`: the `LiveRamps` literal gains `pan_samples`/`link_samples`, which the compiler forces.
  - `.claude/skills/author-session/SKILL.md`: the root-key list and a short `control_smoothing` section.
  - `scripts/check-browser-expected-resources.py`: the self-test's one-byte mutation, which is the companion of the gate-7 re-pin.
- Each edit is necessary or harmless. None of them weakens a check. The spec's path list and the record should name them.

### NIT 1: the `for_row` exhaustiveness claim is wider than the code
- `crates/host-core/tests/live_delta.rs`, `for_row_returns_the_key_field_of_every_row`, says that "the exhaustive `match` fails to compile when a row is added without one".
- That holds for `expected()`. It does not hold for the hand-written `rows` array: a new variant that is left out of `rows` compiles and goes untested.

### NIT 2: comment out of chronological order
- `tools/audit/src/fixture_builtins.rs` (around line 5313): the #1054 note sits between the #1240 and #1328 notes.

## Deviations (a)-(g)
- **(a) Gate 4 allocations: accepted.**
  - The commit is control-plane work. `classify_live_delta` documents its allocations, and AGENTS.md allows allocation on the control thread.
  - The test brackets each `test_render` call that carries the commit and the ramp with `bench_support::alloc` thread counters, after 4 warm-up blocks. It asserts exactly `(0, 0)` allocations and frees, and `assert_installed()` proves that the counters are live.
  - Recommendation for root: change gate 4's text to "every render call that carries the commit and the ramp allocates nothing". Counting the commit itself would be a different claim, about the control plane.
- **(b) `race_settle_blocks`: accepted. No comparison was weakened.**
  - The bit-identity of the final block against a fresh plan is unchanged, and so is the per-call allocation/free assertion. Only the settle length grows, by the session's longest ramp.
  - The race keeps the `default` table, so ramps now start and retarget under swaps. That is more coverage, not less.
  - I reverted the settle in the mutation tree: `live_edits_racing_a_rendering_plan_and_its_swaps_stay_exact_and_allocation_free` goes RED.
- **(c) Deliverable 2 is empty: confirmed.** The one `SessionModel {` outside `crates/session` is a destructuring pattern (`crates/protocol/src/model.rs:1042`). The workspace clippy run with `--all-targets --all-features` is green.
- **(d) Gate 3 needs `--features control-provider`: confirmed.**
  - With `--features test-support` alone, 0 tests run.
  - With `test-support,control-provider`, 33 tests pass.
  - The test-debug-a leg unifies the feature.
- **(e) Rounding: confirmed with exact rational arithmetic and a Rust probe.**

  | ms at 44.1 kHz | f32 spelling `ms*rate/1000` | f32 spelling `ms*(rate/1000)` | f64 (crate) | half-even |
  |---|---|---|---|---|
  | 5 | 221 | 221 | 221 | 220 |
  | 45 | 1985 | 1984 | 1985 | 1984 |
  | 263.39 | 11616 | 11615 | 11615 | 11615 |

  - The exact value of f32(263.39) x 44.1 is 11615.4996.
  - Both f32 mutants go RED in my runs.
- **(f) Six session-document digests: confirmed.**
  - Each is re-pinned in its own commit.
  - I recomputed each:
    - `canonical.json` sha256: `dc19e091…` -> `4c575634…`
    - `MANIFEST.tsv` sha256: `0834666e…` -> `d8d3d6a3…`
    - only the two `prepare_256_tracks` rows and their `session_template_sha256` line change
    - `browser-v1/session.json`: 1969 -> 2021 bytes
    - the C ABI snapshot length: 0x3771 -> 0x37a5, +52
  - Every pinned render digest is unchanged and passes on the final tree: `ACCEPTED_GRAPH_PCM_SHA256`, `_METERS_`, the browser parity digests through `check-browser-expected-resources.py --artifacts`, and `resources.jsonl` through `check-builtins-fixtures.sh`.
  - The exception is the unpinned `audit capi` digest (MINOR 1).
- **(g) Stale `artifacts/steps/*` sha references: accepted.** They are records of past runs. No gate reads old records; the scripts only write new ones.

## Other checks
- **D1a migration (`351650a5b`): only the key is added.** I stripped every inserted `control_smoothing` default spelling from each of the 30 changed files with a script, and each result was byte-identical to its parent. All 21 migrated JSON documents pass `session_validator validate`. The exception is `fixtures/session/v1/canonical.json`, which stops at stage 5 with `effect.native.unavailable`, as `tools/session-validator/tests/validate.rs` expects.
- **Defaults.**
  - `CONTROL_SMOOTHING_DEFAULT` is mute 10, fader 20, pan 20. This matches `FINDINGS.md` section 1 and sections 9.1/9.8 exactly.
  - The D3 key table matches section 9.1 row by row: polarity is `2 x mute_samples`, and the link glide uses `fader_ms`.
  - The rounding formula matches section 1.
- **Existing C ABI live tests are not weakened.**
  - `live_pcm_shape` now runs under the default table and also under `HARD_SWITCH`. The `HARD_SWITCH` run keeps the original windows (ramps 0), so the original claim stays.
  - The carrying-swap test authors `HARD_SWITCH` (D6).
  - The pending-candidate and submix tests still compare 3 blocks after a window that now includes the ramp.
- **SDK.**
  - `check-sdk-headless`: 363 evals pass, and the corpus loop reproduces the Rust writer corpus. The corpus has 2 documents: one `default` by path, and the full-surface one `explicit` with `-0.0`, 12.5 and 1000.
  - SDK mutations, all RED:
    - the writer drops the root key: 24 fails
    - `explicit` key order swapped: 1 fail
    - the builder omits the key: 24 fails
- **D5 precedence.** The model's non-zero window wins, else `pan_samples`, for both pan and raw matrix.
- **Production caps.** The production `maximum_smoothing_samples` is `u32::MAX`, so a live window up to 192,000 is accepted.
- **The browser is unaffected.** host-web carries its own record windows (#1364).
- **Acked-batch question:** there are no queue changes, so no ack can precede a drop.
- **Wording:** there is no producer, fan or listener wording, and nothing relies on decision 16.

## Test value (one sentence per new or rewritten test)
- `default_and_explicit_parse_to_the_model_field`: red if the parser confuses `default` with `explicit`, or normalizes `-0.0` or the lengths.
- `every_malformed_control_smoothing_is_refused_at_its_path`: red if the parser defaults a missing key, accepts a value outside [0, 1000], or lets a kind carry foreign keys; no test read the key before.
- `both_kinds_round_trip_through_canonical_json_to_identical_bytes`: red if the Rust writer drops, reorders or invents the object, or loses `-0.0`.
- `a_built_model_with_a_length_outside_the_domain_is_refused`: red if the validator stops holding a built (not parsed) model to the domain, NaN and infinity included.
- `control_smoothing_samples_round_half_up_in_f64`: red if the conversion truncates, rounds half to even, or uses either f32 spelling. I re-ran the f32 mutants and they were RED.
- `the_default_table_is_the_researched_one`: red if the default stops being the #1055 table at any launch rate.
- `the_default_pan_ramp_is_at_least_twice_the_mute_ramp`: red if a future default change (#1388) breaks `pan_ms >= 2 x mute_ms` even after the table test is updated with it.
- `records_carry_the_session_ramps_and_the_models_own_pan_window` (rewritten): red if `for_session` returns 0, if a row reads the wrong key, or if a session 0 is read as the default. I re-ran the for_session-zero and zero-is-default mutants and they were RED.
- `a_raw_matrix_without_a_window_takes_the_session_pan_ramp`: red if a raw matrix without its own window keeps 0. I re-ran the matrix-ignores-pan mutant and it was RED; no C ABI test catches it.
- `for_row_returns_the_key_field_of_every_row`: red if polarity gets the plain mute length or a row maps to the wrong key. I re-ran the polarity-plain mutant and it was RED, and only this test catches it.
- `a_live_mute_ramps_over_the_session_mute_ramp_and_lands_on_the_muted_control` (gate 4): red if the mute still steps, if its ramp is cut short, or if a ramp block allocates. I re-ran for_session-zero and quarter-mute and both were RED; it was the only C ABI test red under for_session-zero, so it is unique.
- `a_live_mute_under_an_explicit_zero_is_the_muted_control_from_the_first_block` (gate 5): red if a session 0 is read as "use the default". I re-ran zero-is-default and it was RED.

## Gates run (on the final tree)
- **lint job, every step:** PASS.
  - fmt, clippy with `-D warnings`, `cargo doc` with `-D warnings`.
  - Policy checks and their mutation tests: workspace, session, env-vocabulary, bench, test-support CI, scalar-oracle self-test, script reachability, console benchmark fixture, builtins-fixtures mutations, bench preconditions, host-core, protocol-control, realtime/audit-leak/artifact-evidence, realtime trace validator, lane, unfused seal, rack/builtins/graph, effect-runtime, conformance boundaries, PEQ seal, release-shape, npm-publish, stem store, stem identity.
  - The three x86 lane-guard probes.
  - Script reachability first failed because my export had no git repository. It passes after a `git init` of the export (2,192 files, the same file list as `adf234b19`).
- **test-release job, every step:** PASS. This covers lane/math/wasm-gates, filter_liveness, tail_contract, ramp_endpoint, designer_total `--ignored`, the M3 FMA cfg check, and the loom SPSC leg. The math-closure steps did not run, because the route does not trigger them for this diff.
- **test-debug-a (tests and doctests, CI feature list):** PASS.
- **audit-native subset:** PASS.
  - Release build of audit, bench, capi and session-validator.
  - `cargo test --release -p audit -p bench -p console-workload`, which covers the re-pinned `ACCEPTED_MANIFEST_SHA256` and the joined-corpus identity.
  - `audit capi`: 0 allocations, 0 syscalls, 0 violations.
  - `check-builtins-fixtures.sh` and `check-console-fixtures.sh`.
- **Worklet chain:** PASS.
  - `build-web-audioworklet.sh --named-twin`, the `strip-wasm-names` self-test and check, `check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, and `check-scalar-oracle-absent --wasm`.
  - `test-web-audioworklet.sh` with a private TMPDIR, which was empty afterwards.
- **SDK job:** PASS. This covers `npm ci`, `check-sdk-generated`, `check-sdk-deletions`, `check-sdk-types`, `check-sdk-headless` (363/363) and `sdk-package.sh check`.
- **Other gate-6 checks:** `check-cross-targets.sh` passes, and `session_validator validate` was run on every migrated document.
- **Not run:** the Chromium browser legs, which are not required at slice level, and anything that runs only on AArch64.
