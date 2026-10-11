# Restore a green nightly: install ripgrep for the vectorization mutations and give the release link proof timeout headroom

## Product outcome

The nightly is green on `main`. The vectorization red-mutation harness runs to completion on the runner
for the first time, fails loudly and accurately when a tool is missing, and a nightly job cannot be
masked again without a gate failing. The link proof has measured timeout headroom, and its cost
reduction is owned by #1511.

## Context (diagnosis 2026-10-11)

- The "native production-kernel release-probe report" job has failed every night since 2026-10-09
  (alert #1501). The report passes (`"status":"pass","kernel_rules":4,"failures":[]`); the mutation
  script `scripts/test-native-vectorization-report.sh` then fails with `line 65: rg: command not found`.
  The job installs only `llvm` (`.github/workflows/nightly.yml:58-62`). 11 other jobs install `ripgrep`
  inline (qualification.yml 85, 298, 466, 761, 912, 969, 1002, 1029, 1053, 1086; nightly.yml 220).
- The script has used `rg` since 90631be59 (2026-09-12). `continue-on-error: true` hid the failure: the
  mutation checks have never completed in CI since at least 2026-09-02. fb5898857 (#1481) removed the
  mask, correctly. The #1481 and #1490 verdicts ran the script on a workstation, where `rg` exists.
- Missing `rg` showed up as "green control produced no pass report: unmodified-wrapper", which reads like
  an audit regression, not a missing tool.
- The "workspace release link proof" hit its 45-minute timeout once (2026-10-09): build 39m40s, the
  randomized step cancelled 5.3 min in. The same commit 1b55de30a built in 39.6 min on 10-09 and 26.2
  min on 10-10 (slow/fast factor about 1.5). Two job comments are false: "nothing is rebuilt" (line
  170; the randomized step recompiles 2-3.5 min) and "full 21-minute cost" (line 165; measured 25-40 min).
- `scripts/check-ci-path-routing.py`'s `UNMASKABLE` matches only job-level keys and checks step-level
  `continue-on-error` only for `moved-mutation-suites`; nothing stops #1481's mask from coming back.

## Decisions

- D1. Install `ripgrep` in the probe job's install step, like the 11 other jobs; the step name says why
  (rg is not on the runner image). The script is not ported to `grep -E`.
- D1b. In `scripts/test-native-vectorization-report.sh`, beside the objdump preflight (lines 11-18), add
  `command -v rg >/dev/null 2>&1 || { printf 'native vectorization mutations require rg (ripgrep)\n' >&2; exit 2; }`.
- D2. Keep #1481's removal of `continue-on-error`.
- D3. Raise `release-link-proof` `timeout-minutes` from 45 to 60, with the derivation in the job comment
  (slow-runner factor about 1.5; current fast total 30.8 min gives about 46.5 min slow; 60 gives about 29%
  headroom; the timeout guards against hangs and is not the cost budget, which #1511 owns). Add a first
  step that prints `nproc` and the `lscpu` model name. Correct the two false comments: line 170 becomes
  "shares the build above except a measured 2-3.5 min recompile, owned by #1511"; line 165 becomes
  "measured 25-40 min".
- D4. #1511 (reduce the link proof's cost) and #1512 (a shared required-tool preflight, including the 11
  negated `! rg` gate sites) are the filed successors.
- D5. A change to a nightly step's commands or failure mode is proven once on the runner with
  `gh workflow run nightly.yml --ref <branch>` before merge, because no per-PR job runs these jobs.
  Record this rule in the nightly.yml header comment.
- D6. Extend `check_nightly_*` in `scripts/check-ci-path-routing.py` so that no job in `failure-notice`'s
  `needs` list carries `continue-on-error` at job or step level; add its mutation to
  `scripts/test-ci-path-routing.py`.

## Authorized paths

- `.github/workflows/nightly.yml` (the probe install step, the link-proof timeout, comments and hardware
  step, the header comment).
- `scripts/test-native-vectorization-report.sh` (the preflight lines only).
- `scripts/check-ci-path-routing.py`, `scripts/test-ci-path-routing.py` (the D6 rule and its mutation).
- This spec.

## Gates

- G1. A `workflow_dispatch` of nightly.yml on the branch before merge: the probe job is `success`, its
  log has `"status":"pass"` and `native vectorization red mutations: ok`, duration recorded against its
  20-minute limit. Test value: red if the harness cannot run on the runner image, which no workstation
  run catches (the cause of this failure and of about 40 masked nights).
- G2. The same run: the link proof is `success`; build, randomized and total minutes and the CPU model
  are recorded against the 60-minute limit (evidence for the timeout value, not a new test).
- G3. Local, before push, with a PATH shim directory without `rg`: (i) the base script exits 1 at line 65
  with "green control produced no pass report" (the regression reproducer); (ii) the patched script exits
  2 at the preflight with "require rg" before any mutation runs; (iii) the patched script with `rg`
  present prints `red mutations: ok` and exits 0. Test value of (ii): red if a missing tool again shows up
  as a false audit failure.
- G4. `python3 -B scripts/test-ci-path-routing.py` passes, and its new mutation fails the checker when
  `continue-on-error: true` is put on a step of `native-vectorization-report` or `release-link-proof`.
  Test value: catches the step-level mask coming back, the defect that hid this since September; the
  existing `UNMASKABLE` rule cannot.
- G5. The next scheduled nightly on `main` is green; then close #1501.
