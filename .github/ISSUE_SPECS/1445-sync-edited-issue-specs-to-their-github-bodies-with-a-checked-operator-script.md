# Sync edited issue specs to their GitHub bodies with a checked operator script

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). This is
slice C1 of nine of *Check realtime regions with a Rust syntax-tree tool* (C1, A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441),
B2a (#1442), C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what each holds). It is operator tooling, split from the
awk gate's deletion (C2) because it is independently useful and has no dependency:
- A2 syncs #948's body before it closes #948 (A2-D6);
- C2 edits about 84 open specs and must refresh each GitHub body.

No Rust source and no workflow change.

## Problem (verified on `origin/main` at `6d28a80ec`)

- STREAMS says each spec "equals its GitHub body". Every slice that edits specs refreshes their
  bodies by hand today, with `gh issue edit <n> --body-file <spec>`, and nothing checks the result.
- C2 edits the specs that name the awk scripts: 84 top-level specs under `.github/ISSUE_SPECS/` on
  `main`, leaving out #1302 and #1418 and the two files under `BRIEFS/`. A hand sync of that many
  bodies is where a body is missed or a closed issue is edited.
- `scripts/operator/README.md` describes its tools as browser workloads with one CI-facing
  exception. It is a `dsp-research` self-test input (`scripts/ci-path-router.py:88`).

## Decisions

- **D1. The script.** Add `scripts/operator/sync-spec-bodies.sh`. It is an operator script, not a
  workflow step: it needs a token with issue write access, which CI does not hold.
  `scripts/operator/` is outside `check-script-reachability.py`'s workflow rule.
  - **Input:** a git revision range. The script lists the changed top-level spec files with
    `git diff --name-only --diff-filter=M <range> -- '.github/ISSUE_SPECS/[0-9]*-*.md'`, keeps only
    paths with no directory below `.github/ISSUE_SPECS/` (so `BRIEFS/036-..` is never read as
    issue #36), and takes the issue number from each file name.
  - **For each spec,** it does four things:
    1. It reads the issue state with `gh issue view <n> --json state`. A closed issue is listed
       and skipped.
    2. It runs `gh issue edit <n> --body-file <spec>`.
    3. It fetches the body with `gh issue view <n> --json body --jq .body` and compares it byte for
       byte with the spec. The comparison is against the spec with exactly one trailing newline
       removed, on the assumption that GitHub drops one. Nothing else is normalized.
    4. It prints one line: `<n> synced`, `<n> MISMATCH` or `<n> closed-skipped`.
  - **Exit status:** the script exits 1 if any edit fails or any body mismatches, and 0 otherwise.
    Its `--check` mode does steps 1, 3 and 4 only, so it can re-verify without editing.
  - **A systematic MISMATCH** (every edited body differs the same way) means the trailing-newline
    assumption is wrong. Stop and report; do not loosen the comparison without root.
- **D2. Self-test.** `scripts/operator/test-sync-spec-bodies.sh` puts a stub `gh` on `PATH`. The
  stub records edits and serves bodies from a temporary directory. The self-test proves three
  things:
  - a body that differs by one byte fails;
  - a body that differs only by one trailing newline passes;
  - a closed issue is skipped and never edited;
  - a changed `BRIEFS/036-x.md` in the range is not read, so issue #36 is never edited.

  The self-test removes its temporary directory, as #1421 and #1429 require.
- **D3. README.** Add one sentence and one entry for the sync tool to `scripts/operator/README.md`:
  it is a GitHub operator script that edits issue bodies, run by hand after a batch push. The edit
  selects the `dsp-research` suite on the batch's run. That is harmless; record it.
- **D4. First use.** After the batch push that puts this slice on `main`, run the script once on
  that batch's range (it may list no spec), then `--check`. Both outputs go in the Evidence.

## Authorized paths

- `scripts/operator/sync-spec-bodies.sh` and `scripts/operator/test-sync-spec-bodies.sh` (new)
- `scripts/operator/README.md` (D3's sentence and entry). No stream owns it; J takes it by this
  listing.
- This spec

## Non-goals

- Running the script from CI.
- Any spec edit (C2).

## Hazards

- **The stub must not reach the network.** The self-test fails if the real `gh` would be called.
- **GitHub state is part of the deliverable** (AGENTS.md). The script never closes or reopens an
  issue.

## Objective gates

1. `bash scripts/operator/test-sync-spec-bodies.sh` exits 0.
2. **Red on a defect (PR evidence).** Apply each alone; the self-test fails:
   - make the comparison always succeed: the one-byte assertion;
   - compare without removing the trailing newline: the trailing-newline assertion;
   - drop step 1: the closed-issue assertion;
   - drop the top-level filter: the `BRIEFS/` assertion.
3. `bash scripts/check-workspace-policy.sh`, `python3 -B scripts/check-script-reachability.py` and
   `python3 -B scripts/test-script-reachability.py` exit 0.
4. **D4's run.** After the batch push, the script and `--check` exit 0.

*Test value.* The self-test is red if the sync script accepts a body that differs from its spec,
edits a closed issue, or maps a brief under `BRIEFS/` to an issue number. No other check guards the spec-equals-GitHub rule.

## Evidence

- Gates 1-4 output.

## Dependencies

- None. It may land in any J batch. A2 and C2 use it when it is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: two hours.
