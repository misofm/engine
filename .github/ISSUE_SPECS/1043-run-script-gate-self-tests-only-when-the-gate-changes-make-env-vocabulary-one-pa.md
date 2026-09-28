# Run script-gate self-tests only when the gate changes; make env-vocabulary one pass

Source: `docs/handoffs/test-value-2026-09-28/TEST-VALUE-AUDIT.md`, verified in `VERIFY-TEST-VALUE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`. Related cleanup issues: #1017-#1042.

Draft, not a GitHub issue. From the 2026-09-28 test-value audit
([`../TEST-VALUE-AUDIT.md`](../TEST-VALUE-AUDIT.md) §5 item 2, §7). Base `a9414c0c`. Paths starting
`../` are relative to the audit's handoff folder. No ruling needed.

## Problem

In the per-PR lint job, script gates take about 230 s of 294 s. Three steps take 148 s of that, and
in each step the gate itself is cheap while its **self-test suite** is the cost. Figures are from PR
#1016's run and `../data/script-gates-verify.md` §1.

| step | gate | self-test | what the self-test re-proves |
|---|---:|---:|---|
| env vocabulary (71 s) | 15.5 s: 12,128 `grep` spawns, 88 % over `artifacts/`; a single `git grep -ho` does it in 0.7 s | 53.7 s: 22,929 greps; a tool-fault matrix of 43.6 s; pins the documented-row count `134` (`test-env-vocabulary.sh:222-223`), which has caused a red | that `grep`/`sort`/`comm` failures are propagated |
| benchmark validators (47 s) | `check-bench-preconditions.sh` | `test-console-benchmark.sh` (4,187 `jq` processes), rack, builtins-current, wasm-kernel-timing suites | that validators of **descriptive** benchmark records reject mutated records |
| conformance boundaries (30 s) | 0.35 s | 29.3 s, 19 s of which re-runs the whole suite twice against mutated copies of itself (`test-conformance-boundaries.sh:141-165`) | that `find`/`awk`/`sort`/`paste`/`rg` failures are propagated |
| SDK job | `check-sdk-deletions.py` 0.7 s | `--self-test`, 20 s (37 in-memory mutations) | the gate's regexes |

A self-test can only fail when its gate script changes. Running it on every PR costs about 165 s of
runner time and never discriminates a product change. Of the 9 env-vocabulary reds in September, 8 were
hygiene and the ninth was the self-test's own row-count pin (`../data/ci-red-jobs.tsv`).

## Outcome

- `scripts/check-env-vocabulary.sh` does one `git grep -hoE 'MISO_[A-Z0-9_]+'` pass over the same
  path set, with the same three failure classes. The comparison is unchanged.
- The self-test suites run **when their gate script, its `lib/gate.sh`, or its fixture changes** (a
  new router step condition), and in `nightly.yml`. They do not run on every PR. The suites are:
  - `test-env-vocabulary.sh`;
  - `test-conformance-boundaries.sh`;
  - `test-console-benchmark.sh`, `test-rack-benchmark.sh`, `test-builtins-current-benchmark.sh`,
    `test-wasm-kernel-timing.sh` and `test-wasm-console-benchmark.sh`;
  - `check-sdk-deletions.py --self-test`;
  - `test-dsp-research.sh`.
- The recursive self-mutation of `test-conformance-boundaries.sh` (`:141-165`) and the row-count pin
  in `test-env-vocabulary.sh` are deleted.
- Every real gate still runs on every code PR.

## Scope

Authorized paths:
- `scripts/check-env-vocabulary.sh`, `scripts/test-env-vocabulary.sh`;
- `scripts/test-conformance-boundaries.sh`;
- `scripts/ci-path-router.py` and its mirror `scripts/check-ci-path-routing.py` /
  `scripts/test-ci-path-routing.py`;
- `.github/workflows/qualification.yml` and `.github/workflows/nightly.yml`;
- this issue's spec.

No gate's pass/fail rule changes.

## Gates

1. **Same verdicts.** In a scratch copy of the tree, seed each violation class the removed per-PR
   runs covered. Each must still fail the real gate on a PR:
   - env: an undocumented `MISO_ENGINE_X` name in `tools/`; a documented name no longer used; a
     `MISO-FOO` name without the prefix; (quoted names are written `MISO-…` so the env-vocabulary gate does not read them as live identifiers)
   - conformance: a production crate depending on `conformance` or `dsp-reference`, and a production
     `use dsp_reference::`;
   - SDK: a numeric ABI byte offset outside `src/generated/`.
2. **The one-pass env gate is equivalent.** On the unmodified tree, and on the three seeded trees,
   the old and new `check-env-vocabulary.sh` print the same name sets and exit codes.
3. **Routing.**
   - `test-ci-path-routing.py` proves that a change to any listed gate script (or `lib/gate.sh`)
     turns on its self-test step;
   - a product-code-only change leaves the step off but keeps every gate on;
   - a docs-only change keeps today's route.
4. **Nightly** runs every moved suite and fails the nightly issue on a red.
5. **Cost.** On a product-code PR, the lint job's script steps are at most 90 s, measured from the
   job's step timings.

## Saving and risk

- **Saving:** about 165 s of runner time per full PR, from lint and the SDK job.
- **Risk:** a gate script broken by an edit that the router misses would be found at the next
  nightly. Gate 3 bounds this.

## Amendments (Sol verification, 2026-09-28)

See [`../VERIFY-TEST-VALUE.md`](../VERIFY-TEST-VALUE.md). **These amendments supersede the body wherever they conflict.**

1. **Drop four suites from the move list.** Filed issues delete them outright:
   - `test-rack-benchmark.sh` and `test-builtins-current-benchmark.sh` (#1026);
   - `test-wasm-kernel-timing.sh` (#1027);
   - `test-wasm-console-benchmark.sh` (#1039).

   Land this draft after #1026, which also re-points `test-env-vocabulary.sh`'s mutation target and
   would conflict with the one-pass rewrite.
2. **Measured saving.**
   - Across 8 full-route runs the lint self-test parts take a median 142.5 s, and the SDK
     `--self-test` 16.4 s: about 159 s in all.
   - After the deletions above, about **137 s** is attributable to this draft.
   - Gate 5's 90 s bound must be re-derived after #1026.
3. **Where `test-dsp-research.sh` runs.** It runs in the "docs and research evidence gates" job
   (median 17 s), not in lint. Keep it in the list, with the right job named.
4. **The router key is every file a suite reads, not only its gate script.** That includes the jq
   libraries (for example `console-benchmark-record-lib.jq`), the validators, `lib/gate.sh` and any
   fixture the suite copies.
   - Gate 3 adds one case: a change to a jq library only must turn on its suite.
   - `conformance-boundaries` builds a synthetic tree, so it depends on the gate alone. The env
     suite read live docs only through the row-count pin this draft deletes.
5. **The router pins this job.** `scripts/check-ci-path-routing.py` pins `SDK_CLOSURE_LINES`,
   including `check-sdk-deletions.py --self-test` in the `sdk` job. Update it and its mutation test
   in the same change. Both are already in scope.

## Attempt 1 evidence

Terra, 2026-09-28, branch `codex/1043-self-tests-on-gate-change` from `a509b681` (main plus #1031,
#1030, #1033, #1061). CI was not run: every CI figure below is an estimate from the job logs of
five full-route runs (36382065641, 36451079452, 36452187631, 36484715483, 36486469924). Local
figures were taken at a 1-minute load of 28-68 on 32 cores, so local wall times are secondary.

### Where the base moved since the spec

- #1026 and #1027 deleted `test-rack-benchmark.sh`, `test-builtins-current-benchmark.sh` and
  `test-wasm-kernel-timing.sh` (amendment 1) and re-pointed `test-env-vocabulary.sh`'s mutation
  targets at live files. The documented-name count it pinned is 66 on this base (68 in the last CI
  run, 134 when the audit was written).
- #1030 pruned `artifacts/`, where 88 % of the per-file `grep` spawns went. On this base the old
  env gate takes 3.4 s locally, so the one-pass rewrite saves about 3 s, not the audit's 15 s.
- #1027 added the script-reachability lint. The moved suites stay reached (from
  `gate-self-tests` and nightly). The new key rule reuses that lint's own mention scanner.
- #1048's G5 pairing rules and #1061's artifact-identity/record rules are unchanged, and pass.
- #1017 and #1061 took the leaf job count to 17. `gate-self-tests` makes it 18.
- #1033 and #1037 removed two lint steps (native PCM runner, 8 s; effect interchange, 14 s). The
  base totals below leave them out.
- `test-wasm-console-benchmark.sh` still runs in lint (1-2 s). Amendment 1 leaves it to #1039.

### What changed

- **One-pass env gate.** `scripts/check-env-vocabulary.sh` replaces the listing, `tr`, `sed`,
  two `grep -v` exclusions and the per-file `grep` loop with one `git grep -hoE`:
  - in a work tree, `--untracked` (tracked files plus untracked, non-ignored ones: the old
    `ls-files --cached --others --exclude-standard` set);
  - outside one, `--no-index` excluding the top-level `target/` (the old `find` set; `.git/` and
    symlinks are skipped by both);
  - both exclude the vocabulary and `.github/ISSUE_SPECS/` by literal pathspec;
  - `--no-color --no-line-number --no-column` keeps a user's `grep.*`/`color.*` config out of the
    output.

  `git grep` reports an unreadable file on stderr and still exits 0 or 1 (checked with git
  2.43), so any stderr output fails the scan. The three failure classes and the comparison are
  unchanged.
- **The count pin is gone.** `scripts/test-env-vocabulary.sh` is hermetic: it builds a small
  tree and never copies the live docs/, scripts/ and tools/, so it reads only the gate and itself.
  The pinned `66` is replaced by the fixture's own count, `${#documented[@]}`. It is asserted in
  the baseline report line and as the partial output of the `wc` and `tr` faults, the same claim
  the pin made. The rule cases are kept: stray prefix in scripts, tools and docs; the two-path
  exemption; undocumented, unused, deleted and malformed rows; the synonym; missing, symlinked and
  empty inputs; an invalid `GIT_DIR`; both counter-mutants. Added cases prove the path set: in a
  work tree, untracked, ignored and tracked files; outside one, top-level and nested `target/`;
  issue specs; a name used only in a crate; a fragment. The fault matrix covers every external
  command the gate still runs, in both trees, including `git grep`'s stderr failure.
- **Conformance.** The two recursive `MUTANT_RUN` counter-mutant re-runs (the spec's `:141-165`) and the
  `CHECKER` override they needed are deleted.
- **Routing.** `scripts/ci-path-router.py` prints a fourth `--flags` line: `self_tests`, a
  compact JSON list of the suites whose key (`SELF_TEST_INPUTS`) a changed path hits.
  - Every suite is selected, fail-safe, when the path list is unavailable, empty or untrusted.
  - Every suite is selected when `.github/workflows/qualification.yml` changes, because it hosts
    the job.
- **Workflow.** A new `gate-self-tests` job (`needs: route`,
  `if: needs.route.outputs.self_tests != '[]'`) runs each suite in its own step, conditioned
  `contains(fromJSON(needs.route.outputs.self_tests), '<suite>')`.
  - The verdict validates the list's shape and expects the job to be `success` exactly when the
    list is not `[]`, and `skipped` otherwise, on any route.
  - The self-tests are removed from lint, sdk and docs-gates. Each gate stays there,
    unconditional.
  - Nightly's `moved-mutation-suites` runs all five suites, each step `if: ${{ !cancelled() }}` so
    one red suite hides no other. The job is already in `failure-notice`'s `needs`.
- **Deviation 1.** The spec asked for a router step condition inside the existing jobs. A job
  instead lets the verdict enforce the result in both directions, as it does for `release-shape`
  and `release_inputs`. A step condition would only be pinned statically.

### New triggers (the router key) for each self-test

| suite | runs on a PR when any of these changes (and nightly) |
|---|---|
| `test-env-vocabulary.sh` | `scripts/check-env-vocabulary.sh`, `scripts/test-env-vocabulary.sh` |
| `test-conformance-boundaries.sh` | `scripts/check-conformance-boundaries.sh`, `scripts/lib/gate.sh`, `scripts/test-conformance-boundaries.sh` |
| `test-console-benchmark.sh` | the suite, `check-bench-preconditions.sh`, `console-benchmark-record-lib.jq`, `console-benchmark-record-validator.jq`, `console-benchmark-validator.jq`, `web-mixing-automation-lib.jq`, `web-mixing-automation-validator.jq`, `run-web-mixing-automation-benchmark.sh`, `web-mixing-automation-benchmark.mjs`, `build-web-audioworklet.sh` (all under `scripts/`) |
| `check-sdk-deletions.py --self-test` | `scripts/check-sdk-deletions.py`, anything under `sdk/` |
| `test-dsp-research.sh` | `scripts/check-dsp-research.sh`, `scripts/lib/gate.sh`, `scripts/test-dsp-research.sh`, `scripts/operator/README.md` |
| all five | `.github/workflows/qualification.yml`; a `workflow_dispatch`; a missing, malformed or empty diff; an untrusted path |

How the keys were derived:
- **Traced reads.** `strace -f` of each suite on this base: the files it opened under the repo
  root are a subset of its key. The conformance and research suites open only their three
  scripts, since the other paths are inside their synthetic trees. The console suite opens eight
  scripts. The SDK self-test opens the whole `sdk/` tree, because `load()` reads every SDK source
  file, so its key is the prefix (amendment 4).
- **Checker rule.** `check-ci-path-routing.py` now requires each key to cover every file under
  scripts/ that the suite mentions, transitively, by #1027's mention rule (basename on a
  non-comment line, jq `include`/`import`).
- **Over-inclusions.** The rule adds three files the trace did not open:
  `build-web-audioworklet.sh` and `web-mixing-automation-benchmark.mjs`, which the runner names
  but the suite stubs, and `scripts/operator/README.md`, which the research fixture's `README.md`
  basename matches. Each is safe: it only runs a suite more often.
- **How often keys fire.** Over the 284 first-parent commits on main since 2026-09-01:
  - 80 select at least one suite. 17 of those are `qualification.yml` edits, and 46 select only
    the SDK suite.
  - 141 of the 203 full-route commits (69 %) select none.

### Gates

1. **Same verdicts: PASS.** The real gates fail on seeded scratch copies of the tree:
   - env, all three classes, in both a git and a non-git copy: `MISO-ENGINE-X` in
     `tools/bench/src/main.rs`; the one use of `MISO-ENGINE-BENCH-CORE-CLOCK-DRIFT-CEILING`
     renamed; `MISO-FOO` in `crates/engine/src/lib.rs`;
   - conformance: `conformance` added to engine's dependencies, `dsp-reference` added to
     session's, and `use dsp_reference::` in engine: each exit 1;
   - SDK: `view.getUint32(24, true)` in `sdk/src/core/abi.ts`: exit 1.

   The seeded paths route full (tools/, crates/) or sdk (sdk/). The checker pins each gate to an
   unconditional step of lint, sdk or docs-gates.
2. **The one-pass gate is equivalent: PASS.** Old and new gates ran on the unmodified tree and
   the three seeded trees, in a git copy and a non-git copy (8 cases). In every case these were
   byte-identical: stdout, stderr, exit status, and the stray, used, documented, undocumented and
   unused name sets. Local time: 3.5 s old, 0.1 s new.

   It differs only on degenerate trees where the old gate failed with a tool error rather than a
   rule:
   - a tracked file deleted from the work tree;
   - a tracked symlink to a directory.

   The old gate failed on both (`grep status 2`); `git grep` reads neither and passes. `git grep`
   does not follow a symlink to a file; the one tracked symlink points inside the tree. A binary
   file containing a name fails both.

   An 11-mutant pass over the new gate was all killed by the new suite: each exclusion dropped,
   the stderr rule dropped, the untracked files dropped, `target/` handled wrongly, each
   comparison dropped, the scan status ignored, the count wrong.
3. **Routing: PASS.** `scripts/test-ci-path-routing.py`:
   - A change to every listed file selects its suite, including a jq library alone (amendment 4)
     and a rename away.
   - Product code, an unrelated gate and the routing files select none and route full. Docs-only
     and research-note changes keep the evidence route with none selected.
   - An SDK-only change keeps the sdk route and selects the SDK suite. The fail-safe cases select
     all five.
   - The verdict's own bash is extracted and run: it passes only when the job's result matches
     the list. It fails on a red suite, on a skip when a suite was selected, on a run when none
     was, and on a malformed list, and it passes end to end for routed changes.
   - 76 new mutants, each caught by the intended rule:
     - key and suite-table mutants;
     - four router-selection mutants, judged in-process;
     - each suite's step emptied, unconditioned, re-keyed, masked or given another command;
     - each gate dropped from, or conditioned in, its per-PR job;
     - the job's condition changed or masked;
     - the route output or `tail -n 4` dropped;
     - each verdict line dropped or re-pointed;
     - each nightly step dropped, conditioned or masked;
     - the job dropped from the failure notice or put off the schedule.
4. **Nightly: PASS (static).** `moved-mutation-suites` runs all five suites unconditionally
   (`!cancelled()`) and is in `failure-notice`'s `needs`, which opens the issue on any failure.
   The checker pins this and its mutants are red. It has not run on a schedule yet.
5. **Cost: projected, re-derived.** The lint job's script steps on this base run 207-281 s over
   the five runs. Removing the three moved self-test parts (step medians: env 66.1 s, console
   42.1 s, conformance 32.5 s) and the old env scan projects **71, 89, 74, 99 and 98 s (median
   89 s)**. The re-derived bound:
   - at most 90 s at the median of the first three full-route runs after merge;
   - at most 100 s on any single run.

   The spec's single-run 90 s holds at the audit run's speed (71 s), but not on the two slowest
   runners, whose steps all ran 20-40 % slower. To be confirmed from the batch's first CI run.

### CI time saved per pull request (estimate)

- **Full-route PR that hits no key** (69 % of full-route commits): about **180 s of runner time**.
  - lint: 140.7 s of self-tests, plus about 3 s of env scan;
  - sdk: 20.0 s, the `--self-test`;
  - docs-gates: 15.5 s, the research suite.

  Lint wall time falls by about 144 s.
- **Evidence PR:** 15.5 s.
- **PR that hits a key:** the selected suites run in the parallel `gate-self-tests` job, at about
  20 s of setup (checkout and ripgrep).
  - The env suite now costs 5 s of CPU against 108 s for the old one, measured back to back
    locally.
  - The conformance suite costs 23 s of CPU against 61 s, the recursion gone.
  - An sdk-route PR therefore spends about 20 s more runner time in parallel, with the same wall
    time.
- **Route job**, on every PR's critical path: `test-ci-path-routing.py` costs about 13 % more CPU
  (30.2 s against 26.8 s of user time locally). That is about 3 s on its 20-22 s CI step.

### Other gates run

All pass on this tree:
- `check-ci-path-routing.py` and `test-ci-path-routing.py`;
- actionlint 1.7.12 on both workflows (no shellcheck on this host, so `run:` bodies were not
  shell-linted);
- `check-script-reachability.py` and `test-script-reachability.py`;
- `check-env-vocabulary.sh` (66 names) and `test-env-vocabulary.sh`;
- every lint, docs-gates and `gate-self-tests` script command in `qualification.yml`: 58
  commands, the hermetic policy gates and their self-tests with Python under `-B`;
- `check-sdk-deletions.py` and its `--self-test`.

No Rust changed, so `cargo check` was not needed.

### Risks

- **Nightly is the backstop for missed edits.** A gate a suite reads without mentioning it by
  basename (a tree read wholesale, like `sdk/`) must be keyed by hand. The key rule cannot derive
  it, so a miss is found at the next nightly.
- **Conflict with #1039.** `nightly.yml`'s `failure-notice` row label and the
  `moved-mutation-suites` job are edited here, and #1039 also edits `nightly.yml`. The conflict,
  if any, is mechanical.
