# Run script-gate self-tests only when the gate changes; make env-vocabulary one pass

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
