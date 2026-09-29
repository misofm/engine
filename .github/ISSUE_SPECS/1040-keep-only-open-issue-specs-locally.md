# Keep only open issue specs locally

Issue key for the footprint cleanup: AArch64 CI = #1017, Darwin memset = #1018, LANE-3 = #1019, live-control research = #1020, 00 = #1021, 00b = #1022, 01 = #1023, 03 = #1024, 04a = #1025, 04b = #1026, 04c = #1027, 05 = #1028, 06 = #1029, 07 = #1030, 08 = #1031, R1 = #1032, R2 = #1033, R3 = #1034, R4 = #1035, R5 = #1036, R6 = #1037, R7 = #1038, R9 = #1039, R10 = #1040.

Source: `docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, verified in `VERIFY-DEAD-CODE.md`. **The Amendments section supersedes the body wherever they conflict.** Owner rulings: `docs/rulings/engine-footprint-2026-09-28.md`.

**Owner ruling (2026-09-28):** Approved, with the amendment's corrections: re-point citations to permalinks and keep `.github/ISSUE_SPECS/BRIEFS/`.

**Blocked on an owner ruling (a workflow policy change).** Scoping study:
`docs/handoffs/dead-code-2026-09-28/DEAD-CODE-AUDIT.md`, section 7, R10. The ruling to record:
"`.github/ISSUE_SPECS/` holds the specs of open issues only. A closed issue's spec and evidence
live on GitHub and in git history."

## Context

- **What exists.** 560 numbered specs, 10.55 MB. 509 are closed on GitHub (10.19 MB) and 51 are
  open (0.35 MB).
  - Every spec has a GitHub issue.
  - 115 GitHub issues have no local spec (95 closed, 20 open).
- **What AGENTS.md says:**
  - "Work only from a stateless issue body in `.github/ISSUE_SPECS/`; update its evidence/decision
    record as implementation learns facts."
  - "At every issue boundary, compare `.github/ISSUE_SPECS/` with `gh issue list --state all` and
    fix missing, stale, or incorrectly closed entries."
  - Deleting closed specs without amending the second rule would make that comparison report 509
    missing specs.
- **Readers of spec content:**
  - `scripts/run-builtins-benchmark.sh:27` and `scripts/preflight-builtins-benchmark.sh:62` read
    spec 068, and `scripts/test-builtins-benchmark.sh:236` (nightly) copies it. `04b-…` deletes all
    three.
  - `scripts/session-policy-historical-allowlist.txt` lists 5 spec paths. Stale lines there are
    harmless, because the check only skips matches.
- **What changes.** Local specs carry evidence sections appended after filing, so they can differ
  from the GitHub body. Git keeps them either way, and a closed spec can be restored by path from
  any commit.
- **CI routing.** `.github/ISSUE_SPECS/` is an evidence path in `scripts/ci-path-router.py`, so
  deleting specs runs only the `docs-gates` job.

## Smallest closable slice

1. Amend AGENTS.md: the local folder holds open issues only; the boundary comparison is between
   the open specs and `gh issue list --state open`; and closing an issue deletes its spec in the
   same change as the closing evidence commit.
2. Delete the 509 closed specs. Before deleting, confirm each one's GitHub state again with
   `gh issue view N --json state`.
3. Remove the allowlist lines that name deleted specs, and update `ISSUE_SPECS/README.md`, which is
   still titled "Engine V2".

## Objective gates

1. **Scripts:** `bash scripts/check-session-policy.sh` and `bash scripts/test-session-policy.sh`
   pass, as do the other docs-gates scripts.
2. **Nothing reads a deleted spec:**
   `rg 'ISSUE_SPECS/[0-9]' crates hosts tools scripts sdk fuzz .github/workflows` names only open
   specs.
3. **Build, console digests and shipped artifact:** unaffected, because only `.md` files and
   AGENTS.md change. Show `git diff --stat`.
4. **CI routing:** the change routes to `evidence`. `check-ci-path-routing.py` and
   `test-ci-path-routing.py` pass.
5. **Workflow consistency:** after the change, the set of local spec numbers equals the set of
   open GitHub issues that have specs. Record both lists.
6. **No live claim lost:** specs are not tests, and gate 2 shows no test or check reads a deleted
   spec.

## Dependencies

The owner ruling, and `04b-…` (spec 068's readers).

## Standing rules for the implementer

- Do not close, reopen or edit any GitHub issue as part of this change.
- Commit on `codex/<issue>-open-specs-only`.

## Amendments (Sol verification, 2026-09-28)

See `../VERIFY-DEAD-CODE.md`, finding F13. Counts confirmed: 560 numbered specs, 509 closed
(10,193,899 bytes), 51 open; 115 GitHub issues have no spec (20 open); no spec lacks an issue.

1. **Gate 2 cannot pass as written.** `rg 'ISSUE_SPECS/[0-9]' crates hosts tools scripts …` finds
   comments that cite closed specs by path: `crates/dsp-reference/src/svf.rs:443` (045),
   `crates/effect-package/Cargo.toml:23` (083), `scripts/check-effect-interchange-qualification.sh:11`
   (081), `scripts/check-effect-interchange-benchmark-108.sh:11` (108),
   `scripts/promote-issue006-graph-benchmark.sh:7` (006). Add a step that re-points each to the
   GitHub issue or a commit permalink, or narrow the gate to machine readers.
2. **Open specs link closed spec paths:** 1010 → 1000, 1008 → 1001, 026 → 114. Re-point them to
   permalinks in the same change.
3. **Machine readers** are only spec 068 (04b's runners) and five allowlist lines, as the draft says.
4. **`.github/ISSUE_SPECS/BRIEFS/` is not closed-spec history and must not be swept up here or in
   `08-…`**: `BRIEFS/019` is the cited normative source of the frozen soft-clip and half-band
   coefficients (`crates/soft-clip/src/lib.rs:3`, `src/kernel.rs:3`,
   `tests/polyphase_identity.rs:5`, `:20`; `crates/lane/src/kernels/halfband.rs:3`, `:58`), and
   `BRIEFS/016` is the source for `crates/dsp-reference/src/true_peak_limiter.rs:57`.
5. **Mobile scope: no effect.**

## Attempt 1 evidence

Terra, 2026-09-29, branch `codex/1040-open-specs-only` from the local batch `codex/batch-slim-5`
at `c04bc8ea`. No GitHub issue was closed, reopened or edited.

**What was deleted.** 551 numbered specs, 11,278,259 bytes (130,476 lines). The set is every
numbered spec whose issue `gh issue list --state all` reports closed, and each one was confirmed
again with `gh issue view N --json state` just before `git rm` (551/551 `CLOSED`). The draft
counted 509; issues have closed and specs have been added since the audit. #1075 falls in the
#1069–#1079 range, but it closed on 2026-09-29 in batch-slim-4, which is on `origin/main` (merged
through PR #1077). The rule deletes it, and keeping it would fail gate 5.

**What was kept.** `README.md`, `BRIEFS/` (77 files, untouched apart from re-pointed citations) and
the 68 specs of open issues: 15, 17, 26, 124, 338, 391, 763, 774, 881, 882, 883, 887, 888, 889,
890, 891, 892, 893, 894, 895, 896, 897, 899, 938, 948, 951, 952, 953, 955, 961, 965, 967, 968, 969,
972, 973, 974, 975, 987, 988, 989, 991, 992, 993, 1008, 1010, 1018, 1019, 1020, 1040, 1045, 1051,
1053, 1054, 1055, 1057, 1058, 1064, 1065, 1069, 1070, 1071, 1072, 1073, 1074, 1076, 1078, 1079.
These include this batch's #1045, #1051, #1064, #1065 and #1076. There is no unnumbered spec, and
every numbered spec has a GitHub issue.

**Permalink commit.** Every citation points at `80c4119b9e6814cb450e87568243d6df9b6be7bc`
(`origin/main`, PR #1077), not at `c04bc8ea`. `git diff origin/main c04bc8ea` changes none of the
551 deleted files, so both commits hold the same bytes, and `80c4119b` resolves on GitHub today
while `c04bc8ea` is not yet pushed. #1031 set this precedent. All 30 distinct permalinks were
checked with `git cat-file` at that commit, and every line anchor was checked against the text it
quotes. `gh api …/contents/…013-compressor.md?ref=80c4119b…` resolves.

**Citations re-pointed (36 edits in 25 files).**
- Open specs: 026 → #114; 1008 → #1001; 1010 → #1000 (amendment 2).
- `BRIEFS/`: 013, 014, 018, 019, 020, 021 → their own issue's spec; 031 → #031; 071 → #071;
  081 → #108.
- Code: `crates/dsp-reference/src/svf.rs` → #045 (amendment 1). This comment is the only code
  change.
- Docs: `docs/README.md` → #013; `docs/research/f64-introduction.md` → #031 and #087. The #087 link
  was already dangling: it named `087-audit-parametric-eq.md`, a file that never existed.
  `docs/rulings/effect-floor-accounting.md` → #013 (×2);
  `docs/rulings/simd-wrapper-around-scalar-inner-loop.md` → #020 and #083;
  `docs/rulings/prefix-strip-inventory.md` names the #84–#107 audit specs by number and records
  their removal.
- Handoffs, as permalinks with the cited line anchors: `builtins-less-removal-2026-09-27/SCOPE.md`
  → #8, #925, #926, #927 (×2), #937, #940, #947; `dead-code-2026-09-28/VERIFY-DEAD-CODE.md` → #1,
  #140; `issues/R1-…` → #1; `silence-2026-09-27/DESIGN.md` → #27, #28; `issues/S7-…` → #944;
  `test-value-2026-09-28/TEST-VALUE-AUDIT.md` → #962; `data/bug-reproducers.md` → #970.
- The amendment's other sites (`effect-package/Cargo.toml`, `check-effect-interchange-*.sh`,
  `promote-issue006-graph-benchmark.sh`) and every spec-068 reader and allowlist line had already
  been removed by earlier issues (04b, #1050). No machine reader of a spec file remains.

**Rule recorded.** AGENTS.md's "Issue-first execution" section gains one sentence: "A closed
issue's spec leaves `.github/ISSUE_SPECS/` at the batch after it closes, and git history keeps it."
The boundary comparison now reads `gh issue list --state open`. With `--state all` it would report
551 missing specs. `ISSUE_SPECS/README.md` gains a short "What this folder holds" section, and
`docs/IMPLEMENTATION_PLAN.md`'s purpose line gains one clause.

**Gates.**
1. `check-session-policy.sh` and `test-session-policy.sh` pass. So do the docs-gates job
   (`check-dsp-research.sh`, `check-builtins-listening.sh`, plus `test-dsp-research.sh`), the
   route job's `check-`/`test-ci-path-routing.py`, every script of the hermetic policy job
   (env-vocabulary, script-reachability, test-support-CI, workspace, realtime, lane, rack,
   builtins, graph, effect-runtime, host-core, protocol-control, bench, unfused-seal, stem-store,
   release-shape and the rest) and the command kind/reason vocabulary checks: 53 invocations, all
   green. All Python ran with `python3 -B`.
2. `rg 'ISSUE_SPECS/[0-9]' crates hosts tools scripts sdk fuzz .github/workflows` finds three
   things: the `svf.rs` permalink, the `0001-fixture.md` paths that `test-env-vocabulary.sh`
   writes into its temporary roots, and the synthetic `1043-x.md` path in
   `test-ci-path-routing.py`, which is a routing input and never read. A repo-wide scan of every
   tracked file (by stem, by `ISSUE_SPECS/NNN-` and by abbreviated `NNN-…`/`NNN-*` form, with
   permalinks and `BRIEFS/` masked) finds only false positives: branch names in
   `ci-red-jobs.tsv`, the fixture paths above and a mutation label `918-*`.
3. `git diff --shortstat c04bc8ea 210f4d3e`: 579 files, +48/−130,476. That is 551 deletions,
   3 open specs, 9 briefs, the README, AGENTS.md, one Rust comment and 13 docs. No build input,
   console digest or shipped artifact changes. `cargo fmt --check` passes, and
   `cargo test --locked -p dsp-reference` passes (31 + 3); it covers the one Rust file touched,
   where only a comment changed. No test reads a spec, so no wider cargo run was needed.
4. Routing: the deletions and doc edits route `evidence`. The change as a whole routes `full`,
   because `AGENTS.md` (slice step 1) and `svf.rs` (amendment 1) are not evidence paths. Measured
   with `ci-path-router.py` on `c04bc8ea..210f4d3e`. The batch routes `full` anyway.
   `check-ci-path-routing.py` and `test-ci-path-routing.py` pass.
5. Local spec numbers (68) = open GitHub issues that have a local spec (68); the diff is empty.
   The open issues without a local spec are unchanged, at 20: 172, 191, 195, 197, 210, 234, 284,
   291, 293, 296, 349, 377, 379, 382, 394, 559, 560, 877, 931, 932. The kept list above is the
   other side.
6. No test or check reads a deleted spec (gate 2).
