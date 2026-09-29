# Retire the wasmtime console benchmark and decide the nightly descriptive benchmarks

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved: retire the wasmtime console benchmark, and the nightly descriptive benchmarks if nothing actively uses them. The native console `--step` rows and the V8 rows on the shipped artifact stay.

**Blocked on an owner ruling.** Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`,
section 7, R9. The ruling to record: "Only real host paths are benchmarked. They are the native
console `--step` rows (the host-core compile path at the native width) and the V8 rows on the
shipped `host_web.wasm`. The wasmtime console benchmark is retired. The nightly descriptive
benchmarks are {kept | retired}."

## Context

**The wasmtime console benchmark** (about 2,400 lines):

- **Files:** `tools/wasm-console` (824 lines), `tools/wasm-console-guest` (272),
  `scripts/operator/run-wasm-console-benchmark.sh` (496), `scripts/operator/preflight-wasm-console-benchmark.sh`
  (269), `scripts/wasm-console-benchmark-validator.jq` and `scripts/test-wasm-console-benchmark.sh`.
  - It has 29 named arms and a default, and all 30 output folders already exist.
- **It has not been able to run since 2026-09-01.** Both operator scripts compute the repository
  root as `scripts/` (`$(dirname "$0")/..`) since they moved into `scripts/operator/` (commit
  `f0509c3f`, #319), so they source `scripts/scripts/check-bench-preconditions.sh`.
  - The last record is `artifacts/mono3` (2026-08-28).
  - The runner was still being edited on 2026-09-27 (#956).
- **It measures the wrong engine.** It runs Cranelift ahead-of-time on wasmtime, not V8. The
  records themselves say `comparable_with_console_records: false` and
  `browser_field_measurement: false`.
- **The real-host replacement already exists.** `scripts/run-web-mixing-automation-benchmark.sh …
  --step NAME` renders the shipped module under Node's V8 through `prepared-control.js`, exactly
  as the SDK drives it (#1003). Recent handoffs time other rows with ad-hoc V8 harnesses, for
  example `docs/handoffs/effects-2026-09-27/limiter-diagnosis-wasm-console.mjs.txt`.
- **CI cost:**
  - lint "Wasm console benchmark validator mutation tests";
  - nightly "Wasm console benchmark guest compiles for Wasm";
  - `check-release-shape.py`, which pins `wasm-console-guest` in the cdylib set.

**The nightly descriptive benchmarks:**

- **What they are.** The `benchmark` job in `nightly.yml` runs `run-conformance-benchmark.sh`,
  `run-realtime-benchmark.sh`, `run-session-benchmark.sh`, `run-effect-contract-benchmark.sh` and
  `run-fp-environment-benchmark.sh`, about 30 minutes a night.
  - They measure native kernels and synthetic realtime loops, not a host render path.
  - They assert no threshold, and upload JSONL that nothing reads.
- **They do not measure the shipped build either.** Every benchmark links `bench-support`, which
  enables `engine/realtime-audit` and installs an audited `#[global_allocator]`
  (`tools/bench-support/src/alloc.rs:156-157`).
  - So the console numbers are taken with the audit scope guard in place.
  - That is acceptable for a relative `--step` series, but the figures are not "the shipped
    build".

## Smallest closable slice

1. **Wasmtime console.**
   - Delete `tools/wasm-console`, `tools/wasm-console-guest`, both operator scripts, the validator
     and its test.
   - Remove the lint step line, the nightly guest check, and the guest from
     `check-release-shape.py`'s expected set.
   - Leave `artifacts/` to `07-…`.
2. **Nightly benchmarks (your choice).**
   - **If retired:** delete the `benchmark` job and its five runner scripts, and update
     `failure-notice`.
     - Delete the `bench` `session` subject, which only `run-session-benchmark.sh` launches.
     - **Keep** the `bench` `conformance` and `effect-contract` subjects and the `audit realtime`
       subject. audit-native's `check-effect-contract.sh` runs the first two as *conformance*
       gates, and `trace-realtime-audit.sh` runs `audit realtime`.
     - The `audit fp-env` subject is launched only by `run-fp-environment-benchmark.sh` and has no
       unit tests (audit grep). Delete it with its runner, unless it guards a live claim; the FP
       environment itself stays guarded by `lane`'s `fp_env` tests.
     - Remove only the timing-only code paths that nothing else reaches, proven by compile.
   - **If kept:** no change.
3. **Optional follow-up** (not in this slice): promote a multi-row V8 console harness from the
   handoffs to `scripts/`, with `--step NAME` like the mixing-automation arm, so browser numbers
   exist for every console row.
4. **Docs.** Update `scripts/operator/README.md` and `docs/ENGINE_ENV_VOCABULARY.md`.

## Objective gates

1. **Native and wasm build.**
   - `cargo check --locked --workspace --all-targets --all-features` and
     `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings` pass.
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
     passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change. `console-workload`, which the native benchmark and `bench` still use, is unchanged.
3. **Shipped artifact: unchanged.** No crate in its closure changes. Show
   `git diff --stat -- crates hosts`, which should be empty (audit section 11).
4. **CI routing.**
   - `check-release-shape.py` and its `--self-test` pass, as do `check-bench-policy.sh`,
     `test-bench-policy.sh`, `check-ci-path-routing.py` and `test-ci-path-routing.py`.
   - `nightly.yml`'s `failure-notice` `needs:` list and body match its jobs.
   - The `verdict` table is unchanged.
5. **No live claim lost.**
   - `check-effect-contract.sh`'s conformance record still runs in audit-native.
   - Every deleted test belongs to a deleted tool or runner. List them from the `-- --list` diff
     (audit section 11).

## Dependencies

The owner ruling. It absorbs the tool half of `R7-…` if it lands first.

## Standing rules for the implementer

- Launch no timed workload.
- Commit on `codex/<issue>-retire-wasmtime-console`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F12. The recommendation stands, with one missed dependency.

1. **Holds:** `isolated_cycles_per_lane_sample` comes only from the native console
   (`tools/bench/src/floor.rs:315`), and the exit-report clause names
   `scripts/run-console-benchmark.sh` (`docs/rulings/effect-floor-accounting.md:896-902`).
   `tools/wasm-console` produces no floor field. The V8 benchmark covers one row
   (`console_mixing_automation`).
2. **Missed:** the **wasm floor rule** (`docs/rulings/effect-floor-accounting.md:815-840`) derives
   its per-row residual from `artifacts/compressor-round1/wasm-console-benchmark.accepted.jsonl`,
   and `docs/rulings/compressor-identity-mask-hoist-wasm-null.md:55-65` cites the same record. The
   wasmtime console is the only committed source of per-row wasm numbers (Cranelift, not V8). Step 4
   must append a dated history note to both rulings: the tool that produced those residuals is
   retired, and a future wasm floor comes from V8 rows or is not stated. `artifacts/compressor-round1/`
   is one of the 17 ruling-cited folders draft 07 keeps, so the record itself survives.
3. **If this ruling waits or is "keep",** the wasm console scripts stay broken (finding F2). Draft
   `00b-…` repairs their root in the meantime.
4. **Mobile scope:** nothing in the repository measures AArch64/NEON performance
   (`scripts/check-cross-targets.sh:124` prints "native aarch64 unsupported, see #378"). Deleting
   the wasmtime console does not change that; a NEON console leg belongs to R1's recommended
   aarch64 CI work, not to this draft.
5. **A lint ratchet names two files this draft deletes.** `scripts/check-bench-policy.sh:184-185`
   holds a `timed_subjects` list that "never shrinks": `tools/bench/src/rack.rs`,
   `tools/audit/src/fp_env.rs` and `tools/wasm-console/src/main.rs`. Deleting
   `tools/wasm-console` (step 1) or the `audit fp-env` subject (step 2, "if retired") fails lint
   with `converted subject is missing`. The draft must edit that list, and its "never shrinks"
   comment, with an explicit reason, and update `scripts/test-bench-policy.sh`'s cases that write
   into those files.

## Attempt 1 evidence

Terra, attempt 1. Branch `codex/1039-retire-wasmtime-console-bench` from the batch-2 head
`52016391`; implementation commit `9530ee3d`. Every comparison below is against `52016391`. No timed
workload was launched.

### Decision: the nightly descriptive benchmarks are retired

The owner ruling retires them "if nothing actively uses them". Nothing does:

- The five runners are named only by `nightly.yml`'s `benchmark` job (`git grep`, outside closed
  specs).
- They assert no threshold, and nothing downloads the `conformance-benchmark` upload: the only
  `download-artifact` steps fetch `audioworklet-*` and `engine-sdk-qualify-*`.

### One deviation from the body: `bench conformance` is deleted, not kept

The body keeps the `bench conformance` subject because "audit-native's `check-effect-contract.sh`
runs the first two as conformance gates". That premise is wrong for `conformance`:

- `check-effect-contract.sh:76` runs `bench effect-contract --conformance`. Its failure text, "bench
  conformance output", names that mode, not the `conformance` subject.
- `bench conformance` (`tools/bench/src/conformance.rs`) is a timed descriptive benchmark of two
  conformance primitives (fixture decode with CRC32C, and the f32-to-f64 compare). Only
  `run-conformance-benchmark.sh` launched it.

So it is one of the nightly descriptive benchmarks the ruling retires. The two subjects that are
real gates stay whole as gates:

- `bench effect-contract --conformance` and `--audit`;
- `audit realtime --audit` and `--probe`.

Only their timing modes went, because only the retired runners passed them. Then the compiler
proved the rest dead:

| removed | only caller | then flagged dead by `cargo check` and removed |
|---|---|---|
| `bench effect-contract --benchmark-two-rounds` | `run-effect-contract-benchmark.sh` | `benchmark()`, `NoopBank` and its `PreparedNativeEffectBank` impl, `metadata()`, `project_metadata()` and its one test |
| `audit realtime --benchmark-rounds` (`Mode::Benchmark`) | `run-realtime-benchmark.sh` | `RoundEvidence::elapsed` with its `Instant`, and all of `tools/audit/src/record.rs` (its three helpers had no other caller once `fp_env.rs` went) |

`audit fp-env` guarded no live claim. It measured the guard's cost, with no threshold. The #146
record that backs "negligible" stays at `artifacts/issue146/`, which `lane/src/fpenv.rs:75` cites.
The FP environment itself stays guarded by `lane --test fp_env`, `capi --lib fp_environment` and
`host-core --test fp_environment`.

The retired entry points now refuse:

- `bench session` and `bench conformance` print the usage `bench <console|effect-contract|protocol>`
  and exit 2;
- `audit fp-env` prints the usage and exits 2;
- `bench effect-contract --benchmark-two-rounds` prints the usage and exits 2;
- `audit realtime --benchmark-rounds 2` panics with "unknown argument", as any unknown argument
  already did.

### What was deleted, and what it covered

| category | files | lines removed | lines added | bytes removed |
|---|---:|---:|---:|---:|
| wasmtime console tool and scripts | 8 | 2,421 | 0 | 124,317 |
| nightly runners and subjects (`tools/bench`, `tools/audit`, 5 runners) | 13 | 1,733 | 6 | 66,877 |
| `artifacts/` (15 folders) | 117 | 1,902 | 0 | 4,199,320 |
| CI, policy, docs and manifests | 15 | 210 | 112 | 15,268 (10,655 added) |
| **total** | **153** | **6,266** | **118** | **4,405,782** |

Code and scripts alone: 4,364 lines removed, 206,462 bytes.

- **Wasmtime console** (Cranelift ahead-of-time on wasmtime 47.0.3, not V8; broken since
  2026-09-01):
  - `tools/wasm-console` (823 + 37 lines) and `tools/wasm-console-guest` (271 + 23);
  - `scripts/operator/run-wasm-console-benchmark.sh` (496) and
    `preflight-wasm-console-benchmark.sh` (269);
  - `scripts/wasm-console-benchmark-validator.jq` (225) and `scripts/test-wasm-console-benchmark.sh`
    (277).
  - It covered 29 named arms and a default, the #183 W4/W8 leg, and the
    `comparable_with_console_records: false` wasm record family.
- **Nightly benchmarks:**
  - `run-conformance-benchmark.sh`, `run-realtime-benchmark.sh`, `run-session-benchmark.sh`,
    `run-effect-contract-benchmark.sh` and `run-fp-environment-benchmark.sh` (269 lines);
  - `tools/bench/src/session.rs` (514; the #004 parse and compile timing of 256 tracks);
  - `tools/bench/src/conformance.rs` (435);
  - `tools/audit/src/fp_env.rs` (178) and `tools/audit/src/record.rs` (43);
  - the timing branches of `effect_contract.rs` (-225, +4) and `realtime.rs` (-52, +1).
- **CI:**
  - the lint step "Wasm console benchmark validator mutation tests";
  - the nightly `benchmark` job (about 30 minutes a night);
  - the nightly step "Wasm console benchmark guest compiles for Wasm", with `math-sweeps`'s wasm32
    target install (the job is renamed "math exhaustive sweeps");
  - the two `--exclude wasm-console*` in `test-debug-a`.

### The 21 runner-named artifact folders: 15 deleted, 6 kept

**Method.** For each of the 21 folders from #1030's hand-off, a scanner looked for `artifacts/<name>`
in every tracked file, expanding brace lists (`round2-{comp,…}`). A second pass matched bare names
(`-w`). Then the bodies of the 107 open issues (`gh issue list --state open`) were scanned for the
same names.

**The rule applied.** A folder stays if any tracked file outside the deleted set cites it, a kept
sealed record included, iterated to a fixed point. This is stricter than #1030's README rule, which
counts only live citers. Under that rule, `issue182`, `round2-comp` and `round2-comp-baseline`
would also go: 3 more folders, 30 files, 828,923 bytes. That call is root's.

**Kept (6 folders, 57 files, 2,032,550 bytes):**

| folder | cited by |
|---|---|
| `mono2` | `docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md:363`, which takes its wasm number from `mono2/README.md`. #1031 keeps that note (checked on `codex/1031-prune-handoffs`), and open #972-#975 and #987 cite it. |
| `mono3`, `mono3-baseline` | `crates/rack/tests/MUTATIONS.md:271`: "the sealed `--mono3` pair measured it at +39-42%". This bare arm name was missed by #1030's path scan, and `crates/` cannot change here (gate 3). |
| `issue182` | the kept, ruling-cited `artifacts/compressor-round1/README.md:31` ("the cause … `artifacts/issue182/README.md` records"). |
| `round2-comp` | the kept `artifacts/mono3/README.md:29` ("for the reason `artifacts/round2-comp/README.md` records"). |
| `round2-comp-baseline` | the kept `artifacts/round2-comp/README.md:23,116` (its paired arm). |

**Deleted (15 folders, 117 files, 4,199,320 bytes):** `audit-chain-merge{,-baseline}`,
`round2-{eqrack,lane,lim}{,-baseline}`, `round2-composed` and `strip{1,2,3}{,-baseline}`.

- On the base tree, their only path citers were the two operator scripts this change deletes. On
  HEAD, the scanner finds none.
- Bare names appear only in deletion lists and descriptions: the dead-code drafts and the #1022,
  #1030 and #1039 specs.
- No open issue body names any of them.

### Ratchets and counts, updated honestly

- **`timed_subjects`** (`check-bench-policy.sh`) goes from three to one. `fp_env.rs` and
  `wasm-console/src/main.rs` leave with their benchmarks. No surviving file under `tools/` calls
  `timing::timed` except `console.rs`, so no slot can be refilled. The ratchet comment says so.
  `test-bench-policy.sh`'s three `later-timed-*` cases now target `console.rs`, renamed
  `timed-*`.
- **Unsafe owners under `tools/`** go from six to five: `wasm-console-guest` left the list in
  `check-bench-policy.sh`, `check-realtime-policy.sh` and `test-bench-policy.sh` (counts 6 to 5).
- **Release shape:** the cdylib set goes from four to three (`capi`, `host-web`,
  `wasm-gate-guest`). The self-test's "missing crate" mutation now removes `wasm-gate-guest`.
- **Env vocabulary:** 68 names become 67. `MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER` was set only by
  the realtime and FP-environment runners and read only by the deleted record paths.
- **#557 rule removed.** Its two subjects (`session.rs`, `conformance.rs`) are gone. The mutation
  cases that wrote into those files now write into `tools/bench/src/protocol.rs`, and the two #557
  cases are dropped.
  - **Follow-up, not done:** `bench-support`'s `sysinfo::HostToolchainFacts`,
    `physical_core_count` and `json::json_string_array` now have no caller outside `bench-support`.
  - They are public items of a library, so the compiler cannot prove them dead.
  - The spec limits this change to what compile proves, so they stay.

### Gates

Gates 1 to 5 and the policy run below were taken on the tree before four of the kept folders were
restored. The amend that restored them changed only `artifacts/`. The artifact-sensitive checks
were then run again on the amended commit.

1. **Native and wasm build.** All pass, with zero warnings in the check logs:
   - `cargo check --locked --workspace --all-targets --all-features`;
   - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`;
   - `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`;
   - `CARGO_PROFILE_RELEASE_PANIC=unwind cargo check --locked --release --workspace --all-targets`
     (the release-shape job);
   - `cargo fmt --all -- --check`.
2. **Console digests.** `gain_pan_profile digests` gives 17 rows on base and change, byte-identical
   (the row file's SHA-256 is `dd0e7194…` on both). The change run recompiled nothing.
   `tools/console-workload` is untouched.
3. **Shipped artifact unchanged.**
   - `git diff --stat 52016391 HEAD -- crates hosts` is empty.
   - `cargo tree --locked -p host-web --target wasm32-unknown-unknown -e normal` is identical on
     base and change.
   - `Cargo.lock` loses only the two workspace-member entries. `wasmtime` stays, for `wasm-gates`.
4. **CI routing.** All pass:
   - `check-release-shape.py` and `--self-test`;
   - `check-bench-policy.sh` and `test-bench-policy.sh`;
   - `check-ci-path-routing.py` and `test-ci-path-routing.py`.

   `failure-notice` was checked by parsing `nightly.yml`:
   - its `needs:` equals the six remaining jobs;
   - every `*_RESULT` env maps to one of them;
   - the body has six rows, in job order, with seven `%s` for seven arguments.

   The `verdict` table is untouched (no hunk in `qualification.yml` touches it).
   `ci-path-router.py --flags` on `52016391..HEAD` gives `route=full`, `math_closure=true` and
   `release_inputs=true`. `math_closure` comes from `Cargo.toml` and `Cargo.lock`, which dropping
   two members must change. actionlint and shellcheck are not installed on this host, so neither
   ran.
5. **No live claim lost.**
   - `bash scripts/check-effect-contract.sh target/release/bench` passes (8 production factories;
     `launch_failed_gates: 0`).
   - So do the other audit-native uses of the two kept subjects:
     - `test-realtime-audit-probes.sh realtime` (7 operations);
     - `trace-realtime-audit.sh … 1000000`;
     - `trace-effect-contract-audit.sh … 1000000`;
     - `cargo test --locked --release -p audit -p bench` (42 and 18 passed).
   - `cargo test --locked --workspace --all-targets --all-features -- --list` goes from 2,260
     tests to 2,252. All 8 removed tests belong to a deleted subject or runner path:

| removed test (`bench` bin) | claim | where it stands now |
|---|---|---|
| `conformance::tests::percentile_is_nearest_rank_and_escape_is_json_safe` | shared percentile and escaper | `bench-support` `stats::tests::*` (8) and `json::tests::*` (7) |
| `conformance::tests::shared_host_toolchain_facts_preserve_conformance_projection` | the deleted record's projection | removed with the record; the collector keeps `sysinfo::tests::*` (13) |
| `conformance::tests::metadata_command_projection_preserves_empty_status_and_unknown_failures` | the deleted record's command projection | removed with the record |
| `conformance::tests::workspace_dirty_projection_preserves_clean_dirty_and_unavailable_states` | the deleted record's `workspace_dirty` field | removed with the record |
| `session::tests::percentile_and_sha256_are_contract_stable` | shared percentile and digest | `stats::tests::*` and `digest::tests::matches_the_published_vectors` |
| `session::tests::representative_fixture_has_the_frozen_workload_and_stable_bytes` | the deleted session benchmark's workload | removed with the benchmark |
| `session::tests::shared_host_toolchain_facts_preserve_session_projection` | the deleted record's projection | removed with the record |
| `effect_contract::tests::metadata_projection_replaces_quotes_without_changing_other_bytes` | the deleted `--benchmark-two-rounds` record's metadata | removed with the mode |

`tools/wasm-console` and `tools/wasm-console-guest` had no tests (0 base entries).

**Brief's extra gates, and every policy script.** 75 of 75 pass on the implementation commit. The
Python ones ran with `python3 -B`. The list is the lint job's whole hermetic list:

- `test-web-audioworklet.mjs`;
- the workspace, session, env-vocabulary, bench, test-support and script-reachability policies,
  each with its mutation suite (`test-script-reachability.py`: 18 cases);
- the console-fixture check, `test-builtins-fixtures.sh`, `check-bench-preconditions.sh` and
  `test-console-benchmark.sh`;
- the host-core, protocol-control, realtime, realtime-audit-leak, artifact-evidence-leak, lane,
  rack, builtins, graph, effect-runtime (with its fixtures), native-pcm-runner and
  conformance-boundaries policies, each with its mutation suite, and
  `test-realtime-trace-validator.sh`;
- the unfused seal and its self-test;
- `check-step-vocabulary.py` and `--self-test`;
- `check-parametric-eq-render-contract.sh`;
- `check-release-shape.py` and `--self-test`;
- `test-npm-publish-modes.py` and `check-stem-store-v1.mjs`.

Beyond that list, these also pass:

- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- `check-dsp-research.sh`, `test-dsp-research.sh` and `check-builtins-listening.sh`;
- the command-kind and command-reason vocabularies, each with `--self-test`;
- `check-parameter-metadata-v1.py --self-test`, `check-session-map-shape.py`, and
  `check-sdk-deletions.py` with `--self-test`;
- `check-abi-layout-v1.py --self-test` and `check-web-audioworklet-callgraph.py --self-test`;
- `test-gate-lib.sh`, `test-protocol-benchmark.sh`,
  `test-native-vectorization-report.sh target/release/audit`, `test-wasm-realtime-atomics.sh` and
  `check-capi-abi.sh --self-test`;
- `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`.

After the six folders were restored, the artifact-sensitive checks were run again, and all pass:
artifact-evidence-leak with its suite, script reachability, CI routing, `check-dsp-research.sh`,
`check-bench-preconditions.sh`, step vocabulary and workspace policy.

Not run:

- the browser and Playwright gates, `check-web-audioworklet.sh` and the artifact build, because no
  input to them changed (gate 3);
- the full `cargo test --workspace`. Only `tools/bench` and `tools/audit` changed; their release
  tests ran, and every other package's test list is identical.

### Notes for root

- **Merging with the parallel branches** (`git merge-tree` against this commit).
  - #1029, #1030 and #1031 merge cleanly.
  - #1033 conflicts in four files:
    - `check-bench-policy.sh`, `check-realtime-policy.sh` and `test-bench-policy.sh`: delete both
      rows (native-pcm-runner and wasm-console-guest), which leaves four unsafe owners;
    - `tools/audit/src/main.rs`: drop `fp_env`, `record` and `source_fixture`.
  - **Silent edits.** Both branches make three identical edits, so git applies each once with no
    conflict, and each leaves a wrong number. Each is caught by its own suite, but should be set by
    hand:
    - `test-bench-policy.sh`'s three count cases (6 to 5) must read 4;
    - the "these five files … A sixth file" comment in `check-bench-policy.sh` must read four and
      fifth;
    - `test-env-vocabulary.sh`'s count (68 to 67) must read 66.
  - #1049 conflicts in `nightly.yml`'s `failure-notice`: keep #1049's `full-size-tests` row, env
    and argument, and drop `benchmark` and `BENCHMARK_RESULT`.
  - #1059 gets modify/delete conflicts on the two wasm-console sources: resolve by deleting them.
  - #1061's conflict (`scripts/test-web-audioworklet.mjs`) exists against the base `52016391`
    already, and is not from this branch.
- **`AGENTS.md:147`** still lists "the descriptive benchmarks" among `nightly.yml`'s jobs. It is now
  stale. It was not edited, because it is the instruction file; the owner or root should amend it.
- **CI cost:**
  - `route=full` and `math_closure=true`, because of `Cargo.toml` and `Cargo.lock`.
  - Nightly loses about 30 minutes (the `benchmark` job) and the guest check.
  - Lint loses one step.

## Sol verdict, attempt 1

**PASS.** Nothing live was retired, no guard was lost, and every gate is green on the merge into
the batch head. The only defects are in the merge prediction (five conflicts and one wrong count),
and root fixes them at merge. One ruling needs a history note.

**How it was checked.** `#1039` (`51d1195d`) was merged into `codex/batch-slim-3` (`a509b681`: main
plus #1031, #1030, #1033 and #1061) in a scratch detached worktree. No timed workload was
launched.

### Findings, by severity

1. **Medium: the merge prediction is wrong against the real batch. Root must apply these fixes.**
   The implementer predicted conflicts against #1033 alone. The batch also carries #1061, which
   removed an env name. The merge gives **five** content conflicts plus two silent wrong counts:
   - **`scripts/check-bench-policy.sh`.**
     - The unsafe-owner comment block conflicts. Keep #1033's line "(#1033 removed
       `tools/native-pcm-runner` and its row.)" and #1039's `wasm-console-guest` paragraph.
     - *Silent:* "these five files" must read "these **four** files", and "A sixth file" must read
       "A **fifth** file".
     - The `printf` list auto-merges to the right four (bench-support `alloc.rs`, audit `capi.rs`,
       bench `protocol.rs`, `wasm-gate-guest`).
   - **`scripts/check-realtime-policy.sh`.** Conflict: one exclusion line with both
     `^tools/native-pcm-runner/src/lib.rs:` and `^tools/wasm-console-guest/src/lib.rs:` removed.
   - **`scripts/test-bench-policy.sh`.**
     - Conflict at `unsafe-owner-grep-error`: the four-file list above.
     - *Silent:* the `count-error`, `count-formatter-error` and `count-formatter-empty-error`
       cases merge to `5`. All five values must read `4`: `output: 4`, then `output: 4; input: 4`,
       then `input: 4`.
     - Checked: leaving `5` turns the suite red (exit 96, `count-error`).
   - **`scripts/test-env-vocabulary.sh`.** Conflict (batch `66` against #1039's `67`). `COUNT` and
     `COUNT_TR` must read **65**, not the predicted 66.
     - `check-env-vocabulary.sh` reports 65 names on the merge.
     - `66` turns the suite red.
   - **`tools/audit/src/main.rs`.** Three conflict hunks; take neither side. Drop `fp_env` and
     `record` (batch side), and `source_fixture`/`fixture-source` (already gone with #1033).

   `qualification.yml`, `nightly.yml` and `AGENTS.md` auto-merge. The workflow change sets are
   line-identical to #1039's own diff, and the `verdict` table is untouched. #1030's `artifacts/`
   changes do not collide.

2. **Low-medium: a ruling still names the retired arm as its reopen path.**
   `docs/rulings/wasm-simd8-null.md:37-43` says "The `--issue183` bench arm and the
   `miso_wasm_simd8` opt-in cfg remain in the tree" and that a future engine "can re-run the same
   paired capture".
   - `--issue183` existed only in the two deleted wasm-console operator scripts, so that sentence
     is now false.
   - `wasm-simd8-survey.md:199-202,229` also names those scripts as the only setter of the cfg.
   - This is the same class as Amendment 2's missed dependency. It is not a FAIL: owner ruling R9
     retires the tool, and nothing live runs it.
   - **Fix, at merge or as a successor:** append a dated #1039 history note to
     `wasm-simd8-null.md`, and optionally the survey, in the shape of the two notes added here. A
     re-measurement comes from V8 rows on the shipped module, or it is not made.
3. **Low, follow-up only.** Two things are left without a caller, and both sit in crates this
   issue must not touch (gate 3):
   - `miso_wasm_simd8` now has no setter in the tree. `Cargo.toml:97-99` and
     `crates/lane/src/backend.rs:51-55` still describe it as the input to the wasmtime console
     record.
   - `bench-support`'s now-callerless public items, which the implementer already recorded.
4. **Info: stale text that predates this change.**
   - `nightly.yml`'s header still says "ONE FILE, THREE JOBS", though the file has six jobs plus
     `failure-notice`.
   - `AGENTS.md`'s parenthetical never listed `moved-mutation-suites` or `release-budgets`.
   - Root's edit is accurate: the `benchmark` job is gone. The remaining jobs are
     `native-vectorization-report`, `deep-fuzz`, `release-link-proof`, `math-sweeps`,
     `moved-mutation-suites`, `release-budgets` and `failure-notice`.
   - `failure-notice` is consistent:
     - its six `needs` match the six `*_RESULT` env entries;
     - the body has six rows;
     - there are seven `%s` for seven arguments.

### The brief's checks

1. **Nothing live was retired.**
   - **The two kept benchmarks still work** (untimed):
     - `scripts/operator/preflight-console-benchmark.sh --step sol-verify-1039` passes with
       workload launches 0. That includes `cargo test -p bench`, workspace clippy, the release
       build and `console --preflight`.
     - `scripts/test-console-benchmark.sh` passes.
     - `scripts/run-web-mixing-automation-benchmark.sh prepare` and `preflight` pass on the merge.
       `host_web.wasm` is `01dd58be…` on both the merge and `a509b681`.
   - **The `timed_subjects` ratchet shrank honestly.** On the merge, only `tools/bench/src/console.rs`
     calls `timing::timed`. The two retired files leave, the survivor stays, and the comment says
     why.
   - **Nothing still calls a retired item.** Outside closed specs, handoffs and `artifacts/`, no
     workflow, script, step-vocabulary entry or operator doc invokes any of these:
     - the 5 runners, `run-/preflight-wasm-console-benchmark.sh`, the validator or its test;
     - `bench session` or `bench conformance`;
     - `audit fp-env`, `--benchmark-two-rounds` or `--benchmark-rounds`;
     - `MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER`.

     The retired entry points refuse. `bench session` and `bench conformance` exit 2, as do
     `audit fp-env` and `effect-contract --benchmark-two-rounds`. `audit realtime
     --benchmark-rounds 2` aborts on "unknown argument".
2. **The `bench conformance` deviation is correct.**
   - `check-effect-contract.sh:74-76` runs only `bench effect-contract --conformance`. Its failure
     text "bench conformance output" and `qualification.yml:493`'s comment name that record.
   - `tools/bench/src/conformance.rs` was a descriptive timing loop. Its only assertions were
     about its own record helpers. The primitives it timed (fixture parse with CRC32C, and
     `compare_f32_to_f64`) keep their own tests in `crates/conformance`.
   - `bash scripts/check-effect-contract.sh target/release/bench` passes on the merge (8 production
     factories).
3. **`audit fp-env`: no guard was lost.** It measured the guard's cost, with no threshold. The
   claim itself is guarded as follows:
   - **The guard:** `crates/lane/tests/fp_env.rs`, with the x86 MXCSR module and #1017's
     `aarch64` FPCR module. Alongside it are `capi` `runtime::tests::fp_environment`,
     `host-core/tests/fp_environment.rs`, and G6 (`tools/wasm-gates/tests/g6_full_corpus_ftz.rs`).
   - **On x86** these run in `test-debug-b` (`qualification.yml:592`) and `test-release` (`:616`).
   - **On AArch64** they run in `aarch64-debug` (`run-aarch64-tests.sh debug`). Its product-crate
     set includes `lane`, `host-core` and `capi`.
   - The #146 cost record stays at `artifacts/issue146/`, which `lane/src/fpenv.rs:76` cites.
4. **The six kept folders: keep all six.**
   - **`mono2` is required by #1030's rule.** `docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md:363`
     cites it, and open #972-#975 and #987 cite that note. Checked with `gh`, read-only.
   - **`mono3` and `mono3-baseline` are kept by a live crate doc.** `crates/rack/tests/MUTATIONS.md:271`
     cites the pair by arm name. Re-pointing it to a commit means editing `crates/`, which is out
     of scope here.
   - **`issue182`, `round2-comp` and `round2-comp-baseline` are cited only by other kept records.**
     - The citers are `compressor-round1/README.md:31`, `mono3/README.md:29` and
       `round2-comp/README.md:23,116`.
     - #1030's scanner did not count such record-to-record citations, so a strict reading would
       delete them (30 files, 828,923 bytes).
     - Deleting them leaves dangling paths inside a ruling-cited sealed record, and sealed records
       should not be edited to re-point them.
     - The cost is small. Recommendation: keep them.
   - The 15 deleted folders have no citer left on the merge, and no open issue body names them.
5. **AGENTS.md** is accurate; see finding 4.
6. **Gates on the merge.** All pass unless noted.
   - **The bench-policy and routing checks from the brief:**
     - `check-bench-policy.sh` (4 unsafe owners, 1 timed subject) and `test-bench-policy.sh`;
     - `test-console-benchmark.sh`;
     - `check-step-vocabulary.py` with `--self-test`;
     - `check-script-reachability.py` (127 reached, 6 exempt) and its 18 cases;
     - `check-ci-path-routing.py` and `test-ci-path-routing.py`;
     - `check-env-vocabulary.sh` (65) and `test-env-vocabulary.sh`.
   - **actionlint is not installed, so it did not run.** As a substitute, all three workflows parse
     as YAML.
   - **Rust:**
     - `cargo check --locked --workspace --all-targets --all-features`: 0 warnings;
     - `cargo clippy … -D warnings`;
     - `cargo fmt --all -- --check`;
     - `cargo test --locked --release -p audit -p bench -p console-workload`: 40 + 18 + the
       console-workload suites.
   - **Console digests** (`gain_pan_profile digests`): 17 rows, byte-identical to `a509b681`.
   - **Test list** (`cargo test --workspace --all-targets --all-features -- --list`): 2,231 tests
     on the batch head, 2,223 on the merge. The difference is exactly the 8 tests listed above.
   - **The audit-native steps that use the trimmed subjects:**
     - `check-effect-contract.sh`;
     - `test-realtime-audit-probes.sh realtime` (7 operations);
     - `trace-realtime-audit.sh … 1000000`;
     - `trace-effect-contract-audit.sh … 1000000`;
     - `test-protocol-benchmark.sh`.
   - **Release shape:** `check-release-shape.py` and `--self-test`. The set is capi, host-web and
     wasm-gate-guest.
   - **Every policy script:**
     - the lint and docs-gates jobs' 53 script commands, Python run with `python3 -B`;
     - 19 more self-contained checks and self-tests: command-kind and command-reason vocabularies,
       parameter metadata, session map, SDK deletions, ABI layout, call graph, V8 spill, identity,
       gate lib, vectorization report, wasm atomics and C ABI.
   - **Scope:** `git diff 52016391 51d1195d -- crates hosts sdk fuzz` is empty. `Cargo.lock` loses
     only the two members. `ci-path-router.py` gives `route=full`, `math_closure=true` and
     `release_inputs=true`.

**Not in this batch.** The implementer's notes on #1049 (`nightly.yml` `failure-notice`) and #1059
(modify/delete on the two wasm-console sources) still apply when those branches merge.
