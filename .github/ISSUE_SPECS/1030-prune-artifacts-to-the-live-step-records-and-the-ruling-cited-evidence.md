# Prune `artifacts/` to the live step records and the ruling-cited evidence

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

Scoping study: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 6. No code ruling
is needed. `artifacts/` is history, and git already keeps it. This draft proposes a retention rule
you can accept or change: step 1 below.

## Context

- **Size.** `artifacts/` holds 275 folders and one loose file: 62.4 MB and 10,685 files. That is 84% of the
  repository's tracked files and 61% of its bytes.
  - The largest folders are `issue537-candidate-lowering` (11.2 MB), `steps` (4.0 MB),
    `issue539-candidate-lowering` (3.6 MB) and `issue534-production-wasm` (2.4 MB).
  - By type: `.txt` 18.4 MB, `.jsonl` 15.2 MB, `.log` 11.2 MB, assembly dumps 4.2 MB, and 1,772
    `.gz` files.
- **Nothing reads it at test or CI time.** There is no `include_str!`, `include_bytes!` or file
  read of any `artifacts/` path in `crates/`, `hosts/`, `tools/`, `scripts/`, `sdk/`, `fuzz/` or the
  workflows.
  - The remaining mentions are doc comments: `crates/lane/src/fpenv.rs:75`,
    `crates/lane/src/lib.rs:185`, `crates/lane/tests/b2_interleave.rs:7`,
    `crates/capi/src/ffi.rs:743` and `tools/console-workload/src/lib.rs:218`.
  - Also the output paths of the used-up runner arms, which `04a-…`, `04b-…`, `04c-…` and `R9-…`
    remove.
- **Links into it.**
  - 28 links in 12 rulings, citing 17 folders totalling 2.93 MB:
    `compressor-round1{,-baseline}`, `issue149{,-phase2,-phase3}`,
    `issue163-phase{0,1,2,4}`, `issue163-phase2-wasm-baseline`, `issue175`,
    `issue183{,-post-round2}`, `issue184`, `issue-loop-eq-r1`, `round1-composed`, `strip4`.
  - 23 links in 9 live docs. 19 of them are in `docs/issue880-*.md`, which `08-…` removes.
  - `docs/C_ABI_V1_QUALIFICATION.md` has 2, which go with `R2-…`.
- **CI routing.** Any change under `artifacts/` routes CI to `full`, because
  `scripts/ci-path-router.py:17` treats only `docs/` and `.github/ISSUE_SPECS/` as evidence.
- **Clone size does not shrink.** The git pack is 82 MiB, and AGENTS.md forbids rewriting
  history. The gain is working-tree size, grep noise and review load.

## Smallest closable slice

1. **Retention rule.** Keep:
   - `artifacts/steps/`, the live `--step` records of the console and V8 benchmarks;
   - the 17 ruling-cited folders (2.93 MB).

   Delete everything else. If you prefer, re-point the ruling links to commit permalinks
   (`https://github.com/misofm/engine/tree/a9414c0c/artifacts/…`) and delete those folders too.
2. **Delete the other 257 folders and the loose file** `artifacts/issue470-wasm-resource-derivation.md`.
   - Some names look alike but are unrelated, so leave them alone: the `/artifacts/…` URL routes
     in host-web's qualification JavaScript (a served build-output directory), and
     `check-workspace-policy.sh:247`, which scans the tracked-path list for `artifacts/*.ll`.
3. **Re-point or drop** every remaining link from a live doc (outside `docs/handoffs/`, and not
   removed by `08-…`) that names a deleted folder. Use a commit permalink, because the target no
   longer exists in the tree.
4. **Edit the five doc comments** above to cite the issue number instead of the folder. These are
   comment-only edits in `lane` and `capi`. `lane` is in the shipped closure, so see gate 3.
5. **State the rule in `scripts/operator/README.md`:** records live in `artifacts/steps/` while an
   optimisation batch is open, and older records live in git history.

## Objective gates

1. **Build:** `cargo check --locked --workspace --all-targets --all-features` passes, and
   `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`
   passes.
2. **Console digests:** the `gain_pan_profile digests` output is byte-identical on base and
   change.
3. **Shipped artifact:** build base and change on one machine with
   `scripts/build-web-audioworklet.sh --module-only EMPTY_DIR`.
   - The comment edits in `crates/lane` can shift panic line numbers. Prove with a
     function-by-function `wasm-objdump -d` comparison that nothing else changes, and re-pin with
     that reason.
   - Or do step 4 without changing any line count, for example by editing the comment text in
     place.
4. **CI routing:**
   - `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
   - `check-artifact-evidence-leak.sh`, `test-artifact-evidence-leak.sh`, `check-dsp-research.sh`,
     `check-builtins-listening.sh` and `check-bench-preconditions.sh` pass.
   - `rg -n 'artifacts/' crates hosts tools scripts sdk fuzz .github/workflows` names only `steps/`,
     kept folders, or build-output directories under `target/`.
5. **No live claim lost:** no test or check reads a deleted file. This is shown by the `rg` in
   gate 4 and by a green run of the full CI route.

## Dependencies

`04a-…`, `04b-…`, `04c-…` and `05-…` first, so no runner still names a deleted folder. Batch this
with `08-…`, so the full CI route runs once.

## Standing rules for the implementer

- Delete only. Do not rewrite history, do not move records to another path in the same change, and
  do not touch `artifacts/steps/`.
- Commit on `codex/<issue>-prune-artifacts`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F15. The central claim holds.

1. **Reproduced:** 275 folders plus one loose file, 10,685 of 12,789 tracked files, 62,445,918 of
   102,503,598 bytes; 28 ruling links in 12 rulings naming 17 folders (2,931,608 bytes); 23 live-doc
   links in 9 files, 19 in `docs/issue880-*`. No script, test, jq validator,
   `check-step-vocabulary.py`, `run-console-benchmark.sh`, its preflight or
   `run-web-mixing-automation-benchmark.sh` reads a sealed record outside `artifacts/steps/`; the
   only readers of real folders are three test scripts deleted together with their folders by
   04b/04c. On a scratch copy with this draft, 06 and 08 applied, `check-dsp-research.sh`,
   `test-dsp-research.sh`, `check-builtins-listening.sh`, `check-session-policy.sh`,
   `test-session-policy.sh`, `check-artifact-evidence-leak.sh`, `check-bench-preconditions.sh`,
   `check-step-vocabulary.py`, `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
2. **Missed: citations that live only on GitHub.** Open #560 (no local spec) points readers at
   `artifacts/issue470-delivery-qualification`, the loose `artifacts/issue470-wasm-resource-derivation.md`,
   and the `issue555-*`, `issue557-qualification`, `issue558-*`, `issue563-*` and `issue570-*`
   folders as "durable raw evidence". Gate 4's `rg` cannot see issue bodies. Add a step: grep the
   bodies of all open issues (`gh issue list --state open --json number,body`) for `artifacts/`
   and either keep those folders or post a comment on each issue with the commit permalink.
3. **CI cost of step 4:** the doc-comment edits in `crates/lane` also turn on the router's
   `math_closure` (the M1 and F1 exhaustive sweeps in `test-release`), not only the `full` route.
   Prefer the in-place comment edit that keeps line counts, and batch it with 08 as drafted.
4. **Keep `artifacts/compressor-round1/`** (already in the 17): the wasm floor rule of
   `docs/rulings/effect-floor-accounting.md:815-840` derives its residual from it (R9 amendment).
5. Mobile scope: no effect.

## Attempt 1 evidence

Terra, 2026-09-28, on `codex/1030-prune-artifacts` from `codex/batch-slim-2` at `c69736c1`.
Commits: `df8cebb3` (the deletion and the rule), `8349f020` (the re-pointed citations) and this
record.

### The retention rule (step 1)

Adopted as amended, and written where the next agent looks: `scripts/operator/README.md`,
"Where records live (#1030)". Step records live in `artifacts/steps/` while an optimisation batch
is open; older records live in git history. Anything else under `artifacts/` stays only while
something live reads or cites it: a workflow or script (including a runner arm that names the folder
as its output, whose overwrite refusal depends on the record), a test or tool (including a crate doc
comment), a ruling, a surviving doc outside `docs/handoffs/`, a handoff note an open issue cites, an
open issue spec, or an open GitHub issue body. A surviving doc that cites a removed record as
history cites the removing commit instead. A retired runner arm takes the folders only it named.

### What was removed

227 of the 269 top-level entries: 226 folders and the loose
`artifacts/issue470-wasm-resource-derivation.md`. **10,132 files, 47,769,797 bytes.** Tracked
files in the repository: 12,551 before, 2,419 after. `artifacts/`: 10,482 files and 61,184,281
bytes before, 350 files and 13,414,484 bytes after. (The spec's 275 folders had already shrunk to
268 through #1025-#1037.)

**How each removal was proven.** A script (kept in the scratch, not committed) scanned every
tracked file outside `artifacts/`, plus the bodies of all 107 open GitHub issues
(`gh issue list --state open --json number,body`), for `artifacts/<name>` including brace and glob
forms, and classified each citer by the definition above. A folder was removed only when every
citer was a closed issue spec (removed by R10, #1040), a `docs/handoffs/` note that no open issue
relies on, a note that #1031 deletes, an open spec's own deletion instruction, or a GitHub-only body
handled below. Pure wildcards (`artifacts/**`, `artifacts/**/*.ll`) were not counted as citations
of a folder. A second grep for path construction without the literal (`"artifacts"`, joins,
`$root/artifacts/$x`) found only build-output URL routes, fuzz output directories, synthetic test
repositories and the `steps/` runners.

### What was kept, by category (42 folders, 350 files, 13,414,484 bytes)

| category | folders | files | bytes | why |
|---|---:|---:|---:|---|
| `steps/` | 1 | 68 | 3,981,382 | the live `--step` records (standing rule: not touched) |
| ruling-cited | 17 | 98 | 2,931,608 | `compressor-round1{,-baseline}`, `issue149{,-phase2,-phase3}`, `issue163-phase{0,1,2,4}`, `issue163-phase2-wasm-baseline`, `issue175`, `issue183{,-post-round2}`, `issue184`, `issue-loop-eq-r1`, `round1-composed`, `strip4`; exactly the spec's 17 and 2,931,608 bytes; includes Amendment 4's `compressor-round1` |
| named by the wasm console runner | 21 | 174 | 6,231,870 | `audit-chain-merge{,-baseline}`, `issue182`, `mono2`, `mono3{,-baseline}`, `round2-{comp,eqrack,lane,lim}{,-baseline}`, `round2-composed`, `strip{1,2,3}{,-baseline}`: `scripts/operator/run-wasm-console-benchmark.sh:159-217` and `preflight-wasm-console-benchmark.sh:89-147` name them as output and refuse to overwrite them. R9 (#1039) has not landed |
| cited by crate doc comments | 2 | 7 | 95,928 | `issue146` (`crates/lane/src/fpenv.rs:75`, `crates/capi/src/ffi.rs:743`), `issue163-phase3` (`crates/lane/src/lib.rs:216`, `crates/lane/tests/b2_interleave.rs:7`) |
| cited by a surviving handoff an open issue relies on | 1 | 3 | 173,696 | `plumbing-floor`: `docs/handoffs/plumbing-floor-2026-09-26/DIAGNOSIS-2.md:5` takes its rows from it, and open #938 cites that note (#1031 Amendment 4 keeps it) |

Also cited, and kept under another category: `mono2/README.md` by
`docs/handoffs/dual-mono-2026-09-27/DUAL-MONO.md:363` (kept by #1031), `issue183-post-round2` by
`docs/handoffs/silence-2026-09-27/DESIGN.md:126`, `issue163-phase2/README.md` by
`docs/audits/test-usefulness-2026-09-04/01-foundation.md:25`, `issue163-phase2` by
`tools/console-workload/src/lib.rs:218`, `issue149` by `scripts/operator/run-console-benchmark.sh:27`.

**Step 4 needed no edit.** The five doc comments the body lists cite `issue146`, `issue163-phase3`
and `issue163-phase2`, all kept under the amended rule, so they stay true. No file under `crates/`,
`hosts/`, `tools/`, `sdk/` or `fuzz/` changed, which keeps the change off `math_closure`
(Amendment 3) and leaves the shipped artifact's inputs untouched.

### Citations re-pointed

- `docs/C_ABI_V1_QUALIFICATION.md:134` (`artifacts/issue429-qualification`) and `:140`
  (`artifacts/issue435-qualification`): both now say the evidence was retained there until #1030
  removed it in `df8cebb3`, and give `git show df8cebb3^:artifacts/<folder>/<file>`. These were the
  only surviving-doc citations of removed files. (#1033 edits the head of the same file; the hunks
  do not overlap.)

Deliberately not re-pointed:

- `docs/issue880-mc1.md:39`, `docs/issue880-mc2.md:56-57`, `docs/issue880-mq2.md:43-54` cite the
  removed `artifacts/issue880-mq2/` (16,866 bytes; open #1027's spec line 311 left it to #1030).
  #1031 deletes these notes (spec step 3 exempts them); if #1031 keeps any, its step 4 re-points it
  to `df8cebb3`.
- `docs/handoffs/**` notes (the dead-code audit drafts, `plumbing-floor-2026-09-26/PLAN.md`): history,
  excluded by step 3 and pruned by #1031.
- Closed issue specs: history, removed locally by R10 (#1040).
- Open specs #1027, #1028, #1030 and #1037 name removed folders only in their own deletion
  instructions.

### GitHub-only citations (Amendment 2): action for root

Open #560 lists `artifacts/issue555-*`, `issue557-qualification`, `issue558-attempt1`,
`issue558-review`, `issue563-*`, `issue570-*`, `issue470-delivery-qualification` and
`issue470-wasm-resource-derivation.md` as "durable raw evidence" (33 entries, 847 files, 1,408,515 bytes). This
attempt removed them and did not edit GitHub (brief). **Post this comment on #560 when the batch
carrying `df8cebb3` is pushed:**

> #1030 removed the raw evidence this issue lists (`artifacts/issue555-*`, `issue557-qualification`,
> `issue558-attempt1`, `issue558-review`, `issue563-*`, `issue570-*`,
> `issue470-delivery-qualification`, `issue470-wasm-resource-derivation.md`) from the working tree in
> `df8cebb3`. Git keeps it: https://github.com/misofm/engine/tree/a9414c0c/artifacts (the last
> `main` commit that holds all of them), or `git show df8cebb3^:artifacts/<folder>/<file>`.

Every other open issue that names a folder was checked: #1039 (`compressor-round1`, `mono3`) and
#1038 (`issue183`) name kept folders; #1027 and #1028 name folders their own work deleted; #559:1042
("retains all 37 `artifacts/**/*.s` files") is a dated record of #625's delivery, not a live claim.

### Hand-off to R9 (#1039)

#1039's spec says "Leave `artifacts/` to `07-…`", but the 21 runner-named folders above stay only
because the wasm console runner still names them. Under the rule, **#1039 deletes them when it
retires the runner** (6,231,870 bytes, 174 files). The nine other folders the runner names
(`issue163-phase2`, `issue163-phase2-wasm-baseline`, `issue175`, `issue183`, `issue-loop-eq-r1`,
`compressor-round1{,-baseline}`, `round1-composed`, `strip4`) are ruling-cited and stay.

### Gates (all run on this machine, `CARGO_INCREMENTAL=0`, under `nice`)

What changed outside `artifacts/`: `docs/C_ABI_V1_QUALIFICATION.md` (+5 -2),
`scripts/operator/README.md` (+22 -1) and this spec. No Rust, wasm, fixture, workflow or script
input changed.

1. **Build.** `cargo check --locked --workspace --all-targets --all-features`: pass.
   `RUSTFLAGS='-C target-feature=+simd128' cargo check --locked --target wasm32-unknown-unknown -p host-web`:
   pass.
2. **Console digests.** `cargo test --locked --release -p console-workload --test gain_pan_profile -- --ignored --exact digests --nocapture`
   on the change, then with the tree checked out at base `c69736c1`: the 17 digest rows are
   byte-identical (`cmp`), and the base run recompiled nothing (no `Compiling` line), which is
   itself the proof that no build input changed.
3. **Shipped artifact.** `scripts/build-web-audioworklet.sh --module-only` on the change and on
   base: both `miso-engine-v1-audio-worklet.simd128.wasm` are 3,485,631 bytes with SHA-256
   `6c952a2c26f4f78b047d9578bf39f85c2874e11eb82622f5a7e238772d57c2e7`. No crate edit, so no
   objdump comparison and no re-pin. (The committed pin, `f7bd75ca…`, was set at the slim-1 batch
   boundary and matches neither base nor change; that is the batch's pending re-pin, not #1030's.)
4. **CI routing and the named checks.** `check-ci-path-routing.py` and `test-ci-path-routing.py`
   pass; `ci-path-router.py --flags` on `c69736c1..HEAD` gives `route=full`,
   `math_closure=false`, `release_inputs=false`. `check-artifact-evidence-leak.sh`,
   `test-artifact-evidence-leak.sh`, `check-dsp-research.sh`, `test-dsp-research.sh`,
   `check-builtins-listening.sh` and `check-bench-preconditions.sh` pass.
   `rg -n 'artifacts/' crates hosts tools scripts sdk fuzz .github/workflows` names, besides
   `steps/`, only kept folders (every literal `artifacts/<name>` resolves to a folder that
   exists), plus the host-web `/artifacts/` URL routes, `target/ci/qualification-artifacts`, fuzz
   `artifact_prefix` directories, synthetic repositories in `test-workspace-policy.sh` and
   `test-console-benchmark.sh`, and `$variable` paths.
5. **No live claim lost.** No test or check reads a removed file (the grep above, and no
   `include_str!`/`include_bytes!`/file read of `artifacts/` exists anywhere). The only test file
   that mentions `artifacts/`, `crates/lane/tests/b2_interleave.rs`, passes
   (`cargo test --locked -p lane --test b2_interleave`); its folder is kept.

Every other policy gate, each Python one with `python3 -B`, passes (74 of 74): the whole lint
job's hermetic list (`test-web-audioworklet.mjs`; workspace, session, env-vocabulary, bench
(`check-bench-policy.sh` and `test-bench-policy.sh`), test-support, script-reachability, host-core,
protocol-control, realtime, realtime-audit-leak, lane, rack, builtins, graph, effect-runtime,
native-pcm-runner and conformance policies with their mutation suites; the console fixture check;
`test-builtins-fixtures.sh`; `test-wasm-console-benchmark.sh`; `test-console-benchmark.sh`;
`test-realtime-trace-validator.sh`; the unfused seal and its self-test;
`check-step-vocabulary.py` and its self-test; `check-parametric-eq-render-contract.sh`;
`check-release-shape.py --self-test`; `test-npm-publish-modes.py`; `check-stem-store-v1.mjs`),
plus `check-command-kind-vocabulary.py`, `check-command-reason-vocabulary.py` (each with
`--self-test`), `check-parameter-metadata-v1.py --self-test`, `check-session-map-shape.py`,
`check-sdk-deletions.py` (and `--self-test`, 37 mutations), `check-abi-layout-v1.py --self-test`,
`check-web-audioworklet-callgraph.py --self-test`, `test-gate-lib.sh`,
`test-protocol-benchmark.sh`, `test-native-vectorization-report.sh` and
`test-wasm-realtime-atomics.sh`. Not run: the gates that need a built artifact directory or a
browser (`check-web-audioworklet.sh`, `check-browser-expected-resources.py --artifacts`, the
Playwright matrix); their inputs are unchanged (gate 3).
