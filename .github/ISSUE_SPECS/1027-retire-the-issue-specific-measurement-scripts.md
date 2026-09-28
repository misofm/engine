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

## Attempt 2 evidence

Implementer attempt 2 (Terra), 2026-09-28, on the same branch.

Commits:
- `863df03d`: merge of the batch head `92ef396f`.
- `d788e613`: the drain port and the LOW fixes.
- `0e92893b`: the env-vocabulary suite re-pointed at the moved runner.

No timed workload ran.

### Merge of `codex/batch-slim-1` (`92ef396f`)

Each of the six conflicts was resolved by keeping both issues' removals:

- `tools/bench/src/input_symmetry.rs` was deleted here and edited by #1024. It stays deleted.
- `tools/bench/src/main.rs` keeps neither side's subjects: #1026 retired graph, rack and builtins, and this issue retired the other four.
- `scripts/check-builtins-targets.sh` was deleted by #1026 and had a comment edit here. It stays deleted.
- `scripts/check-cross-targets.sh`: one comment now names both retired wrappers.
- The `qualification.yml` lint job keeps the reachability step. #1026's rack and builtins-current lines and this issue's kernel-timing line are all gone.
- `test-env-vocabulary.sh` pins the merged tree's documented-name count, 97.

`cargo check --locked --workspace --all-targets --all-features` passes on the merge.

One break only showed after the merge. #1026 had pointed `test-env-vocabulary.sh`'s injections at `scripts/run-console-benchmark.sh`, and this issue moved that runner, so the suite failed with "env checker unexpectedly succeeded". `0e92893b` points its five occurrences at `scripts/operator/run-console-benchmark.sh`, which the checker still scans, and the suite passes.

### HIGH: the full-capacity drain, ported

The new test is `crates/builtins-compiler/tests/input_drain.rs` `a_block_drains_every_input_record_queued_at_its_entry`. It is gated on `test-support` and runs in test-debug-a.

It runs on two fixtures:
- the SIMD fixture (`test_only_prepared_pair_graph`, capacity 8), whose eight-lane cohort and one-lane tail both use `BuiltinBankProcessor::begin_block`;
- the scalar-dispatch fixture, which uses `ConsoleInputProcessor::process`.

What it does:
1. Fill every track's queue to capacity with a walk of immediate trims ending at -40 dB. A further push must be refused.
2. Render one block.
3. Refill, ending at -20 dB. By now the ring's cursors have wrapped.
4. Render a second block.

What it asserts:
- **Applied:** each block's bits equal those of a twin that was sent only that block's last record.
- **Non-trivial:** a third, uncommanded twin differs, so a trim is visible in the output.
- **Nothing pending:** after each block, every queue accepts another full capacity of records.

Each mutation below was applied to a saved copy, then restored and checked with `cmp`.

| mutation | result |
|---|---|
| Sol's: banked drain `for _ in 0..available.min(2)` (`lib.rs:451`) | red: `banked block 0: the block did not apply every queued record` |
| the same cap on the scalar drain (`lib.rs:4147`) | red: `scalar block 0: …` |
| Sol's cap, with the bits assertion removed | red on the pending check alone: `banked t00: a record from before the block is still pending` |
| `available_at_entry` reads a wrapped count as empty (`spsc.rs`, wrapped branch returns `0`) | red: `banked block 1: …`; also `engine` `available_at_entry_is_bounded_and_handles_wrapped_cursors` |
| none (restored) | green |

### The other matched claims, checked against their bounds

For each claim below, I asked what bound the retired test enforced. Then I applied a mutation that breaks only that bound, and listed which surviving tests (not my ports) go red. The runs were `--no-fail-fast` on saved copies, each restored and checked with `cmp`.

| retired claim | its bound | mutation | surviving tests red |
|---|---|---|---|
| `owners_are_w8_nonzero_and_phase_results_match` | 4,608 blocks of 8 records each, so the queues cycle through thousands of ring wraps; and independent preparations agree | wrapped count read as empty (above) | `engine` spsc wrap test, plus this port's second block. Independent-preparation bit equality is asserted by `input_liveness_console.rs:538`, `:242` and this port. The frozen 4,608-block digest is not re-pinned: it is a change detector over the trim ramp, which the host-core port pins |
| `gate_active` preflight, loud plateaus | the gate opens: output/input > 0.9 | `target = curve` (the open state ignored, `kernel.rs:245`) | `oracle_pcm_within_derived_tolerance_scalar`, `…_w8`, `every_case_agrees_at_every_width_and_matches_its_pin`, `production_hold_is_k_plus_one_and_retrigger_is_current_sample` |
| `gate_active` preflight, quiet plateaus | < 0.01 and nonzero | range clamp removed | attempt 1's port (`contract.rs`), red |
| `multiband_active` preflight | both bands compress: band-gain witnesses < -3 dB | high band summed uncompressed (`lib.rs` near-channel `high_near.mul(amplitude_near_high)` becomes `high_near`) | `active_causal_oracle_engages_releases_and_starts_on_current_sample`, `isolated_low_and_high_band_compression_reduce_only_the_selected_band` (and the dual-mono port) |
| `multiband_active` `descriptor_has_all_frozen_active_values` | 11 parameters × 2 channels | not run | `product.rs:50` asserts the exact list of parameter IDs, so any addition or removal breaks it |
| `prepared_effect_allocations` `banks64_…` (non-bypass half) | at scale, every full group binds and the tail falls back | `bind_group_banks` binds only group 0 (`banks.rs`, `if group_index > 0 { return Ok(bound) }`) | 29 tests across graph-compiler and rack-compiler, including `console_sixty_four_track_fixture_banks_its_dynamic_compressor_bit_identically`, `mixed_twelve_track_plan_binds_renders_full_banks_and_scalar_tails_without_graph_changes` and `the_sixty_four_track_console_less_one_eq_binds_at_every_width`. rack-compiler's 200-case randomized `single_slot_programs_reproduce_exact_equal_chunking` holds the chunking |
| `prepared_effect_allocations` `crossed_small_…` (order half) | the entry order it asserted comes after the subject's own `reverse()` | not applicable | this is a harness fact. The product claim, that membership does not depend on entry order, is `bank_membership_is_independent_of_entry_order` |

Only `separate_capacity_sixteen_…` had the weakness Sol found: a surviving test that exercises the path but not the bound.

### LOW fixes

- The attempt 1 evidence is corrected in place, and each correction is marked "attempt 2 correction":
  - 17 bench tests + 5 audit tests = 22;
  - 17 digest lines;
  - docs and CI = 50 lines (`ENGINE_ENV_VOCABULARY.md` -49, `qualification.yml` -1);
  - the step-vocabulary note now says the batch head already fixed it.
- The host-core port's doc now describes the oracle as a linear ramp restarted each block, with one step `(target - g0) / 256` added per frame.

### Test lists (gate 5)

I compared `cargo test --locked --workspace --all-targets --all-features -- --list` on the batch head (`92ef396f`, built in my own scratch worktree and target directory) against `0e92893b`:

- **Left:** exactly 26 tests. They are 17 bench-subject tests, 5 audit-subject tests, `mq1_transient_shaper_ns_per_lane_sample` and the 3 `bench_ramp` tests. Every one is defined in a deleted file.
- **Arrived:** exactly 7 tests, the six ports and `a_block_drains_every_input_record_queued_at_its_entry`.
- Nothing else changed.

### Gates (on `0e92893b` unless noted)

| gate | result |
|---|---|
| `cargo check` / `clippy -D warnings`, `--workspace --all-targets --all-features`; `cargo fmt --check` | pass |
| wasm `+simd128` check of `host-web` | pass |
| test-debug-a's full command | pass (1,448; the new port included) |
| test-debug-b's full command | pass (809) |
| test-release (`-p lane -p math -p wasm-gates --features math/lane`, release) | pass (115) |
| audit-native (`-p audit -p bench -p console-workload`, release) | pass (134) |
| `gain_pan_profile digests` | 17 digest lines byte-identical to the batch head |
| `bash scripts/run-wasm-gates.sh` | pass |
| shipped module, `--module-only` on the batch head and on the change | byte-identical, `f7bd75ca…`, which equals the batch's new pin |
| every `scripts/check-*.sh` (41 after the merge) | 40 pass; `check-sdk-types.sh` exits 2 because `sdk/node_modules` is absent (it needs `npm ci`). `check-web-audioworklet.sh` and `check-sdk-headless.sh` now pass against the refreshed pin |
| every argument-free `scripts/check-*.py` | all 10 pass, including `check-step-vocabulary.py`, `check-script-reachability.py` (155 reached, 8 operator files exempt) and `check-ci-path-routing.py`. The other 6 require arguments |
| `test-ci-path-routing.py`, `test-console-benchmark.sh`, `test-bench-policy.sh`, `check-/test-env-vocabulary.sh` (97 names), `test-script-reachability.py` | pass (`test-env-vocabulary.sh` after `0e92893b`) |
| router, `--base codex/batch-slim-1 --head HEAD` | `route=full`, `math_closure=false`, `release_inputs=true` |

### Totals against the current batch head

127 files changed, +1,403 / -17,348 lines. The additions include this evidence and Sol's verdict.

| deleted | files | lines |
|---|---|---|
| scripts | 22 | 4,535 |
| Rust files | 7 | 4,057 (#1024 had shortened `input_symmetry.rs` by 3 lines) |
| artifacts | 68 | 8,298 |

## Sol verdict, attempt 2

**PASS.** Attempt 1's HIGH is closed: the new port covers the full-capacity drain claim. The merge with the batch is sound, and every gate I re-ran passes.

Reviewer: Sol, 2026-09-28, on `ba02e7cc`, in a scratch detached checkout with its own target
directory. No timed workload was launched.

### Findings, by severity

1. **LOW: the port's bound is smaller than the one it replaces.**
   `a_block_drains_every_input_record_queued_at_its_entry` fills queues of capacity 8. The retired
   test used 16. A drain capped at a constant of 8 or more would therefore pass the port, where the
   original caught any constant below 16. Neither drain has such a constant (both loop to
   `available_at_entry()`), so no claim is lost. A fixture with a larger capacity would make the
   port as strong as the original.
2. **LOW: evidence wording.** "127 files changed, +1,403 / -17,348 … The additions include this
   evidence and Sol's verdict." +1,403 is the count at `0e92893b`. It includes the attempt 1
   evidence and Sol's attempt 1 verdict, but not the attempt 2 evidence. At `ba02e7cc` the count is
   +1,519.

### The HIGH, re-checked

I applied each mutation below to the scratch copy, ran
`cargo test -p builtins-compiler --features builtins-compiler/test-support,graph/test-support --test input_drain`,
and then restored the file (`git status` clean). The suite was green before and after (7 passed).

| mutation | result |
|---|---|
| mine from attempt 1: banked drain `0..available.min(2)` (`lib.rs:451`) | red: `banked block 0: … the last one did not decide it` |
| banked drain capped at capacity − 1: `available.min(control.capacity() - 1)` | red: `banked block 0: …` |
| scalar drain capped at capacity − 1: `available.min(self.control.capacity() - 1)` (`:4147`) | red: `scalar block 0: …` |
| banked and scalar drains stop one short: `available.saturating_sub(1)` | red on both paths. The one-record twin also stops applying, so the failing assertion is "the trim moves no bit" |
| `spsc.rs` `available_at_entry`: the wrapped branch counts one fewer (`… + producer - 1`) | red: `banked block 1: …`, so the wrapped second block holds |

### The "other six", spot-checked on their bounds

I made three mutations of the kind the coordinator asked for, each changing a bound rather than
only a path:

- **Gate quiet floor.** The range floor is moved by 1 dB: `.max(range.neg().sub(one))`. Red:
  - `contract.rs` `a_quiet_plateau_settles_on_the_range_floor_not_silence`;
  - `determinism.rs` `every_case_agrees_at_every_width_and_matches_its_pin`.
- **Multiband compression depth.** Gain reduction is halved:
  `inv_ratio_minus_one = 0.5 * (1.0 / ratio - 1.0)`. Red:
  `product.rs` `active_causal_oracle_engages_releases_and_starts_on_current_sample`.
- **Bank-group fullness.** A group one lane short counts as full:
  `is_full = active_count() + 1 >= lanes`. Red:
  - graph-compiler `a_single_odd_track_no_longer_strands_a_pool_remainder`;
  - rack-compiler `single_slot_programs_reproduce_exact_equal_chunking`.

  Binding itself stays correct under this mutation, because `bindable_slot_members` has a second
  guard: every lane must run the slot, and padding lanes run none. This agrees with the evidence
  that the binding bound is held.

### Merge with `92ef396f`

I re-merged `2c85ef0d` into `92ef396f` myself. Every file outside the six conflicts is identical to
the implementer's `863df03d`, and each of the six is resolved as the evidence says:
- the reachability step is kept;
- #1026's rack and builtins-current lines and this issue's kernel-timing line are all gone;
- the documented-name count is 97;
- `tools/bench/src/main.rs` keeps neither side's retired subjects.

On the new head, nothing live names a deleted file: the three remaining mentions of
`check-parametric-eq-targets.sh` are comments. `0e92893b`'s re-pointing of
`test-env-vocabulary.sh` at `scripts/operator/run-console-benchmark.sh` is correct, and its suite
passes.

### Gates on `ba02e7cc`, all passing

- `cargo check` and `clippy -D warnings`, `--workspace --all-targets --all-features`; fmt.
- test-debug-a's full command: 1,448 passed, including the new port.
- test-debug-b's full command: 809 passed.
- audit-native: 134 passed.
- Wasm `+simd128` check of `host-web`, and `check-web-audioworklet.sh` (the module matches the
  batch's refreshed pin).
- Reachability: `check-script-reachability.py` reports 155 files reached and 8 operator files
  exempt, and `test-script-reachability.py` passes all 18 cases.
- `check-`/`test-ci-path-routing.py`.
- `check-`/`test-env-vocabulary.sh` (97 names).
- `check-`/`test-bench-policy.sh` (5 operator scripts rooted at the workspace).
- `check-step-vocabulary.py` and `test-console-benchmark.sh`.
- 22 static `check-*.sh` policy scripts, plus `check-`/`test-workspace-policy.sh`.
