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
