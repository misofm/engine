# Retire the issue-specific measurement scripts

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 4, rows B5-B10,
and section 5.1. No ruling is needed. This draft also restores the rule in
`scripts/operator/README.md`: "Every script under `scripts/` is reachable from a GitHub workflow."

Today 18 files outside `scripts/operator/` break that rule, counting jq's extension-less
`include "name"` as a reference.

- 17 are deleted here.
- The 18th is the live `run-console-benchmark.sh`. Move it to `scripts/operator/`, and update its
  callers' paths and its own root computation. If the move is too noisy for this slice, record it
  as the one known exception.

## Context

| family | files | lines | status |
|---|---|---:|---|
| Issue #880 MQ-1/MQ-2 | `scripts/run-issue880-mq{1,2}-benchmark.sh`, `scripts/test-issue880-mq{1,2}-benchmark.sh`, `scripts/issue880-mq1-benchmark-lib.sh`, `scripts/issue880-mq{1,2}-record-validator.jq`, `scripts/fixtures/issue880-mq{1,2}-record.json`, `crates/transient-shaper/tests/bench.rs`, `crates/compressor/tests/bench_ramp.rs` | 2,170 (761 Rust) | reached by no workflow; #880 closed |
| #746/#748 "active" benchmark | `scripts/run-gate-active-benchmark.py`, `scripts/test-gate-active-benchmark.py`, `tools/bench/src/{gate_active,multiband_active}.rs` | 3,225 (1,831 Rust) | reached by no workflow |
| #650 allocation records | `scripts/check-prepared-effect-allocation-records.py`, `tools/audit/src/prepared_effect_allocations.rs` (the `prepared-effect-allocations` subject) | 890 (645 Rust) | the validator is reached by nothing; the subject is one-shot |
| #600-#606 input-trim capture (misnamed "input symmetry"; not dual-mono content detection) | `scripts/{run,preflight,test}-input-symmetry-capture.sh`, `scripts/input-symmetry-capture-validator.py`, `tools/bench/src/input_symmetry{,_capture}.rs` | 1,720 (823 Rust) | reached by no workflow |
| #163 phase-0 wasm kernel timing | `scripts/operator/run-wasm-kernel-timing.sh`, `scripts/test-wasm-kernel-timing.sh`, `scripts/wasm-kernel-timing-validator.jq`, and the `--native-timing`/`--wasm-timing` modes of `tools/wasm-gates` (around `src/main.rs:24-100` and the "#163 phase 0b" block from `src/lib.rs:444`) | 373 script lines + about 200 Rust | its folder exists; the operator script cannot run (wrong root); CI lint still runs its validator test |
| Unreached or superseded | `scripts/check-parametric-eq-targets.sh` (a wrapper with no caller), `scripts/operator/probe-opfs-move-v1.cjs` (a one-off probe), `scripts/operator/seal-web-audioworklet-browser-correctness.sh` (cannot run; superseded by the CI `browser` and `artifact-gates` jobs) | about 290 | — |

**Not in this draft:** `scripts/protocol-benchmark-record-validator.jq` looks unreferenced by
file name, but it is **live**. jq pulls it in with an extension-less
`include "protocol-benchmark-record-validator"` from `protocol-benchmark-validator.jq:1`,
`test-protocol-benchmark.sh:22` and `run-protocol-benchmark.sh:21`, and
`test-protocol-benchmark.sh` runs in the required audit-native job
(`qualification.yml:742`). It goes only with `R3-…`.

**Tests that guard live claims and must survive.** Some tests inside these files assert product
behaviour without timing:

- **`crates/compressor/tests/bench_ramp.rs`:**
  `mq2_preflight_payloads_prove_ramps_restart_on_each_block` (`:374`) asserts the both-channel
  per-block ramp restart and the rate-coefficient call counts `[128, 256, 0]`.
  - No surviving test covers this. `crates/compressor/tests/ramps.rs:119` asserts
    restart-from-current for one point only. So **port** it.
  - `crates/compressor/tests/MUTATIONS.md:327` (row 1006-M3) credits it as one of 16 red tests.
    Edit that row to name the ported test.
- **`tools/bench/src/input_symmetry.rs`, all four tests:**
  - `connected_runtime_oracle_covers_retargeted_both_channel_ramp`;
  - `separate_capacity_sixteen_drain_witness_has_no_pending_records` (`:523`);
  - `owners_are_w8_nonzero_and_phase_results_match` (`:587`);
  - `qualification_phase_constants_and_zero_render_allocations`.
- **`tools/bench/src/gate_active.rs` and `multiband_active.rs`:** the `untimed_preflight_runs_*`
  tests.
- **`tools/audit/src/prepared_effect_allocations.rs`:**
  - `crossed_small_proves_reversed_distinct_prepared_programs`;
  - `banks64_proves_current_backend_cohort_and_heterogeneous_fallback`.

**Also here: repair `prepare-builtins-listening.sh`.** `scripts/operator/prepare-builtins-listening.sh`
cannot run either: its root resolves to `scripts/`. It prepares the blinded listening packets that
AGENTS.md requires, so it is **repaired** here (`/..` becomes `/../..`), not deleted. Of the 7
operator shell scripts, only `preflight-console-benchmark.sh` resolves the root correctly today.

## Smallest closable slice

1. **Keep the live-claim tests.** For every test above:
   - move the untimed assertion into the owning crate's permanent tests (for example
     `crates/compressor/tests/ramps.rs`, a `graph-compiler` or `builtins-compiler` cohort test, or
     `effect-contract` symmetry tests); or
   - show in the evidence the surviving test that already asserts the same claim, with its
     file:line.

   Do this **before** deleting anything.
2. **Delete the files in the table**, and `artifacts/issue880/` and
   `artifacts/issue-60{2,3,6}-input-symmetry-capture/`.
   - Keep `artifacts/issue163-phase0/`: rulings cite it, and `07-…`'s retention rule owns it.
   - Append a dated history note to `docs/rulings/wasm-kernel-timing-interim.md` (which names that
     folder and `run-wasm-kernel-timing.sh` at `:55-56`) and to `docs/rulings/wasm-simd8-survey.md`
     (which cites `run-wasm-kernel-timing.sh:183-184` at `:107`). The note says the runner was
     retired, and that the record and the runner remain in git history. The rulings README asks
     for exactly this kind of note.
   - Remove the `gate-active`, `multiband-active`, `input-symmetry` and `input-symmetry-capture`
     subjects from `tools/bench/src/main.rs`.
   - Remove the `prepared-effect-allocations` subject from `tools/audit/src/main.rs`.
   - Remove the two timing modes from `wasm-gates`, and any code only they reach (prove by
     compile).
3. **Remove the lint line `bash scripts/test-wasm-kernel-timing.sh`** from `qualification.yml`.
4. **Repair `scripts/operator/prepare-builtins-listening.sh`'s root** (`../..`). Run its argument
   and usage path to show it resolves `scripts/check-builtins-listening-033.py`.
5. **Update docs that describe these tools as current.** Remove `docs/ENGINE_ENV_VOCABULARY.md`
   entries only they read (`check-env-vocabulary.sh` decides). `docs/issue880-*.md` belongs to
   `08-…`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
   - `cargo test --locked --release -p audit -p bench -p wasm-gates` and
     `cargo test --locked --all-targets -p compressor -p transient-shaper --features math/lane`
     pass. That is the feature set CI's `test-debug-b` pins.
2. **Console and wasm digests.**
   - The `gain_pan_profile digests` output is byte-identical on base and change.
   - `bash scripts/run-wasm-gates.sh` passes: the digest legs of `wasm-gates` are untouched.
3. **Shipped artifact: unchanged.** No crate in its closure changes; show `git diff --stat`.
4. **CI routing.**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - The `verdict` table is unchanged.
   - A reachability check shows every remaining file under `scripts/` (excluding
     `scripts/operator/`) is named by a workflow or by a script a workflow runs. It must count jq's
     extension-less `include "name"` as a reference, or it misreports live validators as dead.
5. **No live claim lost.**
   - Every test that step 1 lists is either ported, with its new location and a red run of the
     mutation it kills, or matched to a surviving test.
   - The `-- --list` diff contains nothing else.

## Dependencies

`00-…`. Land it in the same CI-conscious batch as `04a-…`, `04b-…` and `05-…`.

## Standing rules for the implementer

- Launch no timed workload.
- Commit on `codex/<issue>-retire-measurement-scripts`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`. The slice is sound; three facts need correcting.

1. **Two of the "unreached" Rust files run in CI today.** `crates/compressor/tests/bench_ramp.rs`
   has two non-ignored tests (`:375`, `:541`), including the MQ-2 preflight test the draft ports;
   `test-debug-b` (`--all-targets -p compressor`) runs them. The four `tools/bench/src/input_symmetry.rs`
   tests run in audit-native's `cargo test --release -p bench`. So "reached by no workflow" is
   true of the scripts only, and the `-- --list` diff in gate 5 will show these tests leaving
   `test-debug-b` and audit-native. The port-first rule in step 1 already covers them.
2. **Operator-script roots.** `prepare-builtins-listening.sh`'s repair (step 4) now also lives in
   the new `00b-repair-the-operator-script-roots.md`; do it in whichever lands first.
3. **Reachability gate (4, last bullet):** the verifier's reachability run (jq `include`, Python
   imports, `source`, and `bash scripts/…` inside scripts) reproduced the 18-file count, and every
   `scripts/operator/` script except the stem-store eval runner is unreached, as the README intends.
   Make the reachability check a committed lint script with a mutation test, or the rule in
   `scripts/operator/README.md` stays unenforced.
4. Mobile scope: no effect.

## Attempt 1 evidence

Implementer attempt 1 (Terra), 2026-09-28. Branch `codex/1027-retire-issue-scripts`, from
`codex/batch-slim-1` at `ed0556a9`. Commits: `7234970c` and `5304d95e` (the ports, before any
deletion), `b5aa74fb` (the retirement and the reachability lint), `632203fc` (merge of the batch
head `7d0d4adf`, which carries #1025's runner rewrite), `f8142178` (the runner move). Host x86_64
(x86-64-v3). No timed workload ran: every runner and preflight call below stops at argument
parsing or at an overwrite or input refusal.

### What was removed

Against the batch head `7d0d4adf`: **126 files changed, +992 / -17,348 lines.**

| kind | files | lines removed |
|---|---:|---:|
| scripts deleted (19 under `scripts/`, 3 under `scripts/operator/`) | 22 | 4,535 |
| Rust files deleted: `compressor/tests/bench_ramp.rs`, `transient-shaper/tests/bench.rs`, bench `gate_active`, `multiband_active`, `input_symmetry`, `input_symmetry_capture`, audit `prepared_effect_allocations` | 7 | 4,060 |
| Rust edits: `wasm-gates` timing arm and modes (lib -274, main -72/+2), bench and audit dispatchers (-19) | 4 | 365 |
| `artifacts/issue880/`, `artifacts/issue-60{2,3,6}-input-symmetry-capture/` | 68 | 8,298 |
| docs and CI: 20 env-vocabulary rows, the kernel-timing lint line | | 50 (attempt 2 correction; was 56: `ENGINE_ENV_VOCABULARY.md` -49, `qualification.yml` -1) |

Added: the ports (+542 lines in 6 test files), the reachability lint and its suite, two dated
history notes, and comment corrections. Every file in the table of the brief is gone, as are the two
artifact families. `artifacts/issue163-phase0/` is kept. `wasm-gates` loses only the
`--native-timing`/`--wasm-timing` modes and the timing arm only they reached (`TimingReport`,
`timing_run`, `native_timing_report`, `wasm_timing_report`). Its `bench-support` dependency stays,
because `digest::hex` still uses it. Clippy `-D warnings` over the workspace reports no dead code left.

### The runner move (the eighteenth file)

#1025 landed its rewrite of `scripts/run-console-benchmark.sh` in the batch while this attempt ran.
I merged the batch head and then moved that version, as the spec asks, to
`scripts/operator/run-console-benchmark.sh`:

- its root is now `../..`, so #1022's operator-root rule covers it (`check-bench-policy.sh`:
  6 operator scripts);
- the preflight hashes it at the new path;
- `check-bench-preconditions.sh` and the standing instructions in `effect-floor-accounting.md`
  (`:901`, `:1164`) name the new path;
- rulings that cite the runner as the provenance of an old record keep the path it had then
  (`d7-check-block-fusion`, `de-versioning-inventory`, and the three rulings #1025 edited).

Argument paths, with no workload:

| invocation | exit |
|---|---|
| `scripts/operator/run-console-benchmark.sh` | 2, usage |
| `… --step base` | 1, `refusing to overwrite console artifact: <repo>/artifacts/steps/base/…` |
| the same, from `/tmp` | 1, the same repository path |
| `scripts/operator/preflight-console-benchmark.sh --step base` | 1, `console artifact already exists` |

No exception is left, so the checker has no exception list.

**Step 4.** `prepare-builtins-listening.sh` already resolves `../..` since #1022. With no arguments
it exits 2 (usage). Given an empty inbox it refuses at `missing inbox member: source.mepcm` after
`cd` to the repository root, from which `scripts/check-builtins-listening-033.py` resolves.

### Reachability lint (amendment 3, gate 4)

`scripts/check-script-reachability.py` (lint job, new step) takes a fixed point over mentions. Its
seeds are the workflows and every `package.json`. Carriers are shell, Python, jq, JavaScript,
TypeScript and YAML files. It counts basenames after one level of brace expansion, jq's
extension-less `include`/`import "name"`, and Python `import`. It never counts comments, Rust
sources or `docs/`.

- On the base tree it reports exactly the 18 files (17 plus the runner). The audit's `reach.py`
  gives the same list.
- On the change: `ok (179 files under scripts/ reached from workflows, 9 under scripts/operator/ exempt)`.
- `scripts/test-script-reachability.py` has 18 cases on a scratch copy of the real carriers. One
  case removes the three jq includes, and `protocol-benchmark-record-validator.jq` is then refused.
  Others cover a dead script, mentions in a comment, a Rust note, a `docs/` tool or a document
  (all refused), transitive reach and its break, Python imports, brace lists, fixtures, the
  operator exemption, and a tree with no workflows.
- Nine mutations of the checker itself were each red under the suite: jq includes ignored, comment
  stripping off, imports ignored, brace expansion off, `docs/` carriers allowed, Rust carriers
  allowed, self-exclusion dropped, workflow guard dropped, and every file treated as reached.

### Live claims (gate 5): ports and matches

Each port was run green, then red under the named mutation on a saved copy, then restored
(`cmp` clean):

| retired test | where the claim lives now | red under |
|---|---|---|
| `bench_ramp.rs` `mq2_preflight_payloads_prove_ramps_restart_on_each_block` | ported: `compressor/tests/ramps.rs` `a_both_channel_point_on_every_bank_lane_restarts_its_ramp_each_block`, at the build's own bank width (was Simd8 only); `MUTATIONS.md` row 1006-M3 now names it | 1006-M3 (output ramps also advanced in pass 1): `lane 0 left: parameter 4's remaining samples`, 62 vs 63, as the original |
| `input_symmetry` `connected_runtime_oracle_covers_retargeted_both_channel_ramp` | ported: `host-core/tests/input_liveness_console.rs` `a_drained_trim_ride_ramps_from_the_value_it_reached_on_both_channels` (twin-times-ramp oracle, 4 retargeted blocks) | bank drain with a zero window: block 0 frame 0, 3.007 vs oracle 5.988 |
| `input_symmetry` `qualification_phase_constants_and_zero_render_allocations` | ported: `builtins-compiler/tests/allocation_tracker.rs` `actual_queued_input_trim_drain_allocates_and_frees_nothing` | allocating `TrimDb` arm: `(9, 9)` vs `(0, 0)` |
| `input_symmetry` `separate_capacity_sixteen_drain_witness_has_no_pending_records` | matched: `input_liveness_console.rs:538` `a_symmetric_ride_renders_the_same_bits_collapsed_or_not` (two records per track must both drain in block 0) | — |
| `input_symmetry` `owners_are_w8_nonzero_and_phase_results_match` | matched: `:538` (two independent preparations, identical bits) and `:242`; bank structure by `graph-compiler` `scalar_dispatch_compiles_without_banks_on_any_host` (`:2543`). The frozen 4,608-block digest is not re-pinned | — |
| `gate_active` `untimed_preflight_runs_the_actual_shapes_without_timing_calls` | matched: `gate-expander/tests/oracle.rs:183`/`:238`, `identity.rs:70`, `graph-compiler` `launch_gate_expander_fixture_retains_width_correct_banks_and_scalar_fallbacks`, `contract.rs:24`. Ported for the range floor, which only a corpus digest crossed: `contract.rs` `a_quiet_plateau_settles_on_the_range_floor_not_silence` | range clamp removed: gain 1.2e-38 vs floor 0.00398 |
| `multiband_active` `untimed_preflight_runs_actual_scalar_and_bank_shapes` | matched for resources, engagement and widths: `product.rs:50`, `:463`, `identity.rs:326`. Ported for dual-mono independence: `product.rs` `dual_mono_bands_compress_each_channel_from_its_own_level`, with a Maximum-link control | `DualMono` dispatched as `LINK_MAXIMUM`: `a loud left moved the quiet right`; the other 10 product tests stayed green |
| `multiband_active` `descriptor_has_all_frozen_active_values` | matched: `product.rs:50` (the 11-parameter descriptor) | — |
| `prepared_effect_allocations` `banks64_…` and `crossed_small_…` | ported: `graph-compiler` `a_prepare_time_bypassed_slot_takes_its_chain_out_of_the_cohort`. Bypass alone separates the program keys, the bypassed chain falls back per node, only the full group binds, and one slot id in two racks gives two programs. Entry order was the subject's own `reverse()` and is proven order-independent by `bank_membership_is_independent_of_entry_order` | `program_key` drops `bypass`: red at the key assertions. With those removed, the cohort half is red on its own: 4 bound slots vs 2 |

The other removed tests check only their own harness: argument parsers, stimulus constants, record
splicing, activity validators, injected clocks, the allocator control, and the MQ-2 marker probe.
The rate-coefficient counts `[128, 256, 0]` were arithmetic on the benchmark's own arm constants,
so they were not ported. The input-trim allocation port covers the banked drain. The fixture's
ninth track is a one-lane bank, not `ConsoleInputProcessor`, which is the coverage the original had.

`-- --list`, base against change, CI feature sets:

- **audit-native** (`-p audit -p bench -p console-workload`, release): exactly 22 tests leave, 17
  of the four bench subjects and 5 of the audit subject (attempt 2 correction; this read "22 …
  and 5"). Nothing else changes.
- **test-debug-b**: the 4 `mq1_`/`mq2_` tests leave, and 3 ports arrive (compressor, gate,
  multiband).
- **test-release** (`-p lane -p math -p wasm-gates`): no change.
- **test-debug-a**: additions only (host-core, builtins-compiler, graph-compiler).

The debug-b base list was captured while the compressor port was being written. Its one line for
that port was removed by hand; no other file had changed.

### Gates

| gate | result |
|---|---|
| `cargo check --locked --workspace --all-targets --all-features` | pass |
| `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` | pass |
| `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web` | pass |
| `cargo fmt --all -- --check` | pass |
| `cargo test --locked --release -p audit -p bench -p wasm-gates` | pass |
| `cargo test --locked --all-targets -p compressor -p transient-shaper --features math/lane` | pass (124) |
| test-debug-b's full command | pass |
| test-debug-a's features over `-p host-core -p builtins-compiler -p graph-compiler` | pass |
| `cargo test --locked --release -p audit -p bench -p console-workload` | pass |
| `gain_pan_profile digests` | every digest line byte-identical to base (only the `finished in` time differs); 17 lines, not 16 (attempt 2 correction) |
| `bash scripts/run-wasm-gates.sh` | pass (native, wasm scalar, simd128, V8 spill) |
| shipped module | `build-web-audioworklet.sh --module-only` on the change, and with `graph-compiler/src/lib.rs` swapped back to base (the only closure `src` change, and it is inside `mod tests`): byte-identical, `3f744b03…` |
| `git diff --stat` for `crates hosts` | test files plus `graph-compiler/src/lib.rs` test module only |
| every `scripts/check-*.sh` | 40 of 43 pass; the 3 failures are below |
| every argument-free `scripts/check-*.py` | 9 of 10 pass; `check-step-vocabulary.py` fails, below |
| `check-ci-path-routing.py`, `test-ci-path-routing.py` | pass; `verdict` table unchanged (one lint step added, one lint line removed, no job added) |
| `test-console-benchmark.sh`, `check-bench-policy.sh`, `test-bench-policy.sh`, `check-env-vocabulary.sh`, `test-env-vocabulary.sh`, `test-wasm-console-benchmark.sh`, `test-script-reachability.py` | pass |
| router, `--base codex/batch-slim-1 --head HEAD` | `route=full`, `math_closure=false`, `release_inputs=true` (the workflow and `wasm-gates/Cargo.toml` changed) |

Failures, none caused by this change:

- `check-web-audioworklet.sh` and `check-sdk-headless.sh` fail on the pin: local module `3f744b03…`
  against pin `476e58ad…`. The module is byte-identical on base (A/B above). The audit (§9, §11)
  says a local build is not compared with the pin; CI's `artifact` job is the authority.
- `check-sdk-types.sh` exits 2: `sdk/node_modules` is missing (it needs `npm ci`).
- `check-step-vocabulary.py` refuses `.github/ISSUE_SPECS/1025-…md:238`, which quotes the retired
  word in #1025's evidence. That line is on the batch head as merged, and this change adds no such
  word. **The batch's lint job will be red on it until #1025's spec is reworded or #1050 retires
  the rule.** (Attempt 2: already reworded on the batch head; it passes on the merge.)
- Six `check-*.py` take required arguments (`abi-layout-v1`, `builtins-listening-033`/`-111`,
  `parameter-metadata-v1`, and the two AudioWorklet checkers). They are not argument-free, so they
  were not run as gates.

### Gates the change had to edit, and why

- `qualification.yml`: the lint line `bash scripts/test-wasm-kernel-timing.sh` is gone with its
  script (step 3), and the new reachability step is added.
- `docs/ENGINE_ENV_VOCABULARY.md`: the 20 `MISO_ENGINE_606_*` and `MISO_ENGINE_CAPTURE_*` rows went.
  `check-env-vocabulary.sh` refused them as unused, and it now reports 114 names.
  `test-env-vocabulary.sh` pins the documented-name count in its `COUNT`/`COUNT_TR` payloads, so
  134 became 114. #1043 removes that pin.
- The rack-chain fixture in `graph-compiler` gains an edit hook for the bypass port. Its existing
  callers are unchanged.

### Left for other issues

- `docs/issue880-*.md` (08, #1031): it now links into the deleted `artifacts/issue880/` and names
  the deleted MQ runners.
- `artifacts/issue880-mq2/` (07, #1030) is not in this brief.
- `docs/audits/60x-*` per-attempt reviews (08).
- Open specs #1030, #1039 and #1050 still cite `scripts/run-console-benchmark.sh` at its old path.

## Sol verdict, attempt 1

**FAIL.** One live claim is lost. Everything else holds, and the fix is one small port.

Reviewer: Sol, 2026-09-28, on `5c72f2c8`, merged into a scratch detached checkout of the
`codex/batch-slim-1` head `d416d8c8` (which carries #1056 and the #1052/#1025 step-vocabulary
rewordings). No timed workload was launched.

### Findings, by severity

1. **HIGH: a lost live claim.** `input_symmetry::tests::separate_capacity_sixteen_drain_witness_has_no_pending_records`
   is listed as "matched" by `input_liveness_console.rs:538`, but that test pushes two records per
   track. The retired test filled every track's input queue to its capacity (16) after a render,
   rendered once, and required all 16 more pushes to be accepted. That proves the banked input
   drain (`BuiltinBankProcessor::begin_block`, `crates/builtins-compiler/src/lib.rs:451`) applies
   every record available at block entry in that block. The proof:
   - Mutation: `for _ in 0..available {` becomes `for _ in 0..available.min(2) {`.
   - On the merge, no surviving test goes red under it. I ran `-p builtins -p builtins-compiler
     -p graph-compiler -p rack -p host-core` (554 passed) and `-p host-web -p capi
     -p effect-contract` (314 passed), with the test-support features. These are every package
     that pushes a `TrackInputRecord`.
   - On base, the retired test goes red under the same mutation, at `input_symmetry.rs:541`
     (`try_push` refused), and green when the mutation is reverted.

   **Fix:** port it. For example, add a test in `builtins-compiler/tests/` on
   `test_only_prepared_pair_graph`: fill each track's input queue to its capacity, render one
   block, then require another capacity's worth of pushes to be accepted. Show it red under the
   mutation above.
2. **LOW, evidence wording.**
   - "exactly the 22 tests of the four bench subjects and the 5 of the audit subject" should say
     17 bench tests plus 5 audit tests, 22 in all.
   - The digest gate prints 17 lines on this merge, not 16. All 17 are byte-identical to base.
   - "docs and CI … 56" does not reproduce: `ENGINE_ENV_VOCABULARY.md` loses 49 lines and
     `qualification.yml` loses 1, so 50. The other rows of the removal table reproduce exactly:
     126 files, +992/-17,348 at `f8142178`; 22 scripts, 4,535 lines; 7 Rust files, 4,060 lines;
     Rust edits 365 lines; 68 artifacts, 8,298 lines; 20 env names.
   - The `check-step-vocabulary.py` failure the evidence reports is already fixed on the batch
     head. It passes on the merge.
3. **LOW, a doc comment.** The host-core port's doc says the gain follows
   `g += (target - g) / 256` per frame. The oracle takes one step per block,
   `(target - g0) / 256`, which is a linear ramp restarted each block. The code is right; the
   sentence reads as a one-pole.

### Merge

There was one conflict. `tools/bench/src/input_symmetry.rs` is deleted here and modified by #1024
(`RenderIo { output }`). It resolves by deletion.

There are no semantic conflicts:
- **#1056:** nothing deleted here is named by it.
- **#1052:** no deleted file held a source-scrape row. `check-workspace-policy.sh` and
  `test-workspace-policy.sh` pass.
- **#1025:** the moved runner is #1025's file. Only the usage comment and the `../..` root differ.
- **#1022:** `check-bench-policy.sh` reports 6 operator scripts rooted at the workspace, and
  `test-bench-policy.sh` passes.

### What holds

- **Nothing live deleted.** No workflow, `package.json`, `Cargo.toml`, script, `tools/`,
  `hosts/`, `sdk/` or operator doc names a deleted script, subject, mode or artifact folder. The
  remaining mentions are rulings history (with the new dated notes) and `docs/issue880-*.md`, which
  belongs to #1031 and is disclosed. `artifacts/issue163-phase0/` stays.
- **The moved runner:**
  - it prints usage and exits 2;
  - `--step base` refuses to overwrite and exits 1, run from the repository and from `/tmp`;
  - the preflight hashes the new path.
- **Test lists.** I diffed `cargo test --workspace --all-targets --all-features -- --list`
  between base and merge, with a separate target directory for each. Exactly 26 tests leave: 17
  bench, 5 audit and the 4 `mq1_`/`mq2_` tests. Every one is defined in a deleted file. The 6
  ports arrive. Nothing else changes. The change touches no Cargo feature, so a default-feature
  list can differ only in the same way.
- **Ports.**
  - All 6 pass.
  - I re-ran two of the mutation proofs, and each went red, then green when reverted:
    - with the range clamp dropped, the gate test fails at `gain 1.18e-38` against the floor
      0.00398;
    - with `DualMono` dispatched as `LINK_MAXIMUM`, only the multiband port fails and the other
      10 tests stay green.
  - The other matches I spot-checked hold at their cited `file:line` on the branch.
- **The lint.**
  - A planted dead script and a script named only in a comment are refused.
  - A script reached through a reached script, and a jq module reached only by `include`, are
    accepted.
  - The lint names exactly the 18 files on base.
  - Two mutations of the checker (jq includes ignored, comment stripping off) turn the suite red.
  - The step runs in the `lint` job, and `check-`/`test-ci-path-routing.py` pass.
- **Gates on the merge.** All of these pass:
  - `cargo check` and `clippy -D warnings`, `--workspace --all-targets --all-features`, and fmt;
  - wasm `+simd128` `host-web`;
  - `aarch64-apple-ios` and `aarch64-linux-android` checks of every product crate, `capi` and
    `host-mobile`;
  - `test-debug-b`'s full command (808 passed);
  - `test-debug-a`'s features over host-core, builtins-compiler and graph-compiler (379);
  - audit-native (154) and test-release;
  - `run-wasm-gates.sh`;
  - `gain_pan_profile digests`, 17 lines identical to base;
  - every argument-free `check-*.py`, and the self-tests of the three that take arguments;
  - `test-console-benchmark.sh`, `test-wasm-console-benchmark.sh`, `check-`/`test-env-vocabulary.sh`
    and `check-sdk-types.sh` (after `npm ci`).
  - 41 of the 43 `check-*.sh`.

  The other two, `check-web-audioworklet.sh` and `check-sdk-headless.sh`, fail only on the pin
  (`476e58ad…` against the local `f7bd75ca…`). The module is identical on base, so this is not
  caused by this change (next item).
- **AudioWorklet module.** It is byte-identical on base and merge (`f7bd75ca…`). The pin
  (`476e58ad…`) is the batch boundary's to refresh.
