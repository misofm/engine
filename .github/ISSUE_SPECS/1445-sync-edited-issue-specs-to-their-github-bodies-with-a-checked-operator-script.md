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

## Amendment 1 (root, 2026-10-11): the decision-D2 safeguards

Root's decision D2 of 2026-10-11 (owner direction of the same day: the owner approved syncing
issue titles and bodies from committed specs under these safeguards) amends D1, D2 and D4. Where
this amendment and D1-D4 differ, this amendment holds. Recorded by root, 2026-10-11.

- **A1. Source: the blob on `origin/main`, nothing else.** The script reads every spec with
  `git cat-file blob <commit>:<path>` from one source commit. The source commit is
  `refs/remotes/origin/main`, and the script refuses to start (exit 2, nothing read from GitHub)
  unless that ref equals the hash `git ls-remote origin refs/heads/main` returns now (a stale or
  locally moved ref is refused). It never reads the working tree or the index. If a `--range A..B`
  is given, `B` must resolve to the source commit, or the script refuses. Selection is one of:
  `--all` (every top-level spec at the source), `--range A..B` (D1's top-level filter, with
  `--diff-filter=AM`), or explicit issue numbers. D1's top-level rule stands: a path with a
  directory below `.github/ISSUE_SPECS/` is never read as an issue, and a file name that is not
  `<digits>-<slug>.md` is ignored. The issue number is the leading digits without leading zeros.
- **A2. Title.** The spec's title is its first line when that line is `# <title>`; the title is
  `<title>`. A spec whose first line is not an H1 has no title, and the script never changes that
  issue's title.
- **A3. Classification (three-way, per open issue).** "Matches" means byte equality after allowing
  exactly one trailing newline: the GitHub body `g` matches a blob `b` when `g == b` or
  `g + "\n" == b`. Nothing else is normalized (no CRLF, whitespace or Unicode folding). The
  *history* of an issue is the set of blobs that any top-level spec file of that issue number held
  in any commit reachable from the source commit (renames keep their history because the key is the
  number). Each selected spec gets exactly one class:
  - `closed-skipped`: the issue is not `OPEN`. Never edited.
  - `no-issue`: no issue has that number. Never edited.
  - `oversize`: the source blob is above the ceiling (A6) or its title is above 256 characters.
    Never edited.
  - `in-sync`: the body matches the source blob, and the title equals the spec's title (or the spec
    has no title).
  - `fast-forward`: not `in-sync`; the body matches the source blob or a blob in the history; and
    the GitHub title equals the spec's title, or equals the title of a blob in the history, or the
    spec has no title.
  - `unmatched`: everything else. Never edited. The script stops on these issues and reports them;
    it never overwrites a body or title that is not an earlier committed state of the spec. With
    `--reconcile-dir DIR` it writes, per unmatched issue, the GitHub body, the source blob and the
    nearest history blob (fewest changed lines by `git diff --no-index --numstat`; ties to the
    newest commit), plus a one-line header naming the nearest blob's commit and path and the line
    counts of the two diffs. DIR must not exist.
- **A4. Modes.** `--check` (the default) is read-only: it classifies and prints one line per
  selected spec, `<n> <class>` (for `unmatched`, also the nearest blob's commit and the diff line
  counts). It makes no `gh` call except `gh issue view`. `--apply --backup-dir DIR` writes the
  `fast-forward` set only, and nothing else. `--apply` without `--backup-dir`, or with a DIR that
  already holds a file the run would write, is refused before any GitHub call.
- **A5. The write, serial, one issue at a time.** For each `fast-forward` issue, in ascending number:
  1. Fetch `gh issue view <n> --json number,state,title,body` again, raw. Refuse (stop the run) if
     the state is not `OPEN` or the title or body bytes differ from the classification fetch.
  2. Save that raw JSON to `DIR/<n>.json` and read the file back; refuse if it differs. This is the
     rollback copy (restore with `jq -j .body DIR/<n>.json` into a file and
     `gh issue edit <n> --body-file`).
  3. Write the source blob to a file and run `gh issue edit <n> --body-file <file>`, with
     `--title <title>` only when the title differs. No other flag and no other `gh` write
     (no close, reopen, comment, label, assignee, milestone or `gh api`). The body never passes
     through an argument, an environment variable or a shell command substitution.
  4. Re-read `gh issue view <n> --json number,state,title,body` raw and compare: the body must
     match the source blob (A3's one-newline allowance), the title must equal the spec's title (or
     be unchanged when the spec has none), and the state must be `OPEN`.
  5. Print `<n> synced`. On any failure in 1-4, print `<n> MISMATCH` (or `<n> refused`), make no
     further write, and exit 1 naming the backup file.
- **A6. Size ceiling, fail closed: 261,693 UTF-8 bytes per body.** Measured on 2026-10-11 without
  writing: the largest body GitHub holds intact in `misofm/engine` is #559's, 261,384 characters
  and 261,693 UTF-8 bytes (its spec blob plus one newline). GitHub's API error names a 65,536
  "character" maximum, but that body shows the stored limit is in bytes; community measurements put
  the storage limit at 262,144 bytes (65,536 four-byte characters). The ceiling is the largest size
  observed stored intact, counted in bytes, which can only over-count characters. A larger blob is
  `oversize` and is never written; raising the ceiling needs new evidence and root. Titles: 256
  characters.
- **A7. Exit status.** 0: every selected spec is `in-sync` or `closed-skipped` at the end of the
  run (for `--apply`, after its writes). 3: the run completed but `fast-forward` (in `--check`),
  `unmatched`, `oversize` or `no-issue` specs remain. 1: a write, read-back or race check failed. 2:
  a refusal before any write (source, arguments, backup dir, `gh` or `git` failure).
- **A8. Self-test (replaces D2's list).** `scripts/operator/test-sync-spec-bodies.sh` builds a
  temporary git repository with a bare `origin` (so `ls-remote` is local) and a stub `gh` on `PATH`
  that serves issues from files, logs every argv, and fails the test on any command it does not
  model (so the real `gh` and the network are never reached). It proves each safeguard:
  - a stale `origin/main` ref (remote main moved) is refused before any `gh` call, and a
    `--range` ending at a commit other than `origin/main` is refused;
  - the source is the `origin/main` blob, not a different working-tree copy of the spec;
  - an unmatched body (equal to neither the source blob nor any history blob, for example a body
    edited on GitHub or one equal to an unmerged branch's blob) is reported `unmatched` and never
    edited, and `--reconcile-dir` holds its three files;
  - a body equal to an older blob of a renamed spec file is `fast-forward`;
  - the old body is saved before the edit and equals the served raw JSON; an existing backup file
    refuses the run before any edit;
  - a blob above the ceiling is `oversize` and never edited (the test may lower the ceiling with an
    environment variable that only the self-test sets; the default stays A6's);
  - a closed issue is never edited;
  - every edit is exactly `issue edit <n> --body-file <f>` plus an optional `--title <t>`, and no
    other `gh` write occurs;
  - a read-back that differs by one byte fails (exit 1) and stops later writes; one trailing newline
    of difference passes;
  - a body that changed between classification and the write is refused;
  - `BRIEFS/036-x.md` is never read as issue #36;
  - `--check` makes no edit.
  It removes its temporary directory on exit, as #1421 and #1429 require.
- **A9. First use (replaces D4).** Before this slice is on `main`, root's D2 run uses the script
  from the branch against `origin/main`: `--check --all` first (its output is evidence), then
  `--apply --all` with the backup directory root names, which writes only the `fast-forward` set.
  The `unmatched` set is reconciled by hand from `--reconcile-dir`, never by the script.

## Amendment 2 (root, 2026-10-11): rulings on the first run's `unmatched` set

Root's rulings on the 31 `unmatched` issues of A9's run (owner-approved syncs, 2026-10-11) need two
additions. Recorded by root, 2026-10-11; the mechanism in A11 is the coordinator's, so that every
ruling's sync still goes through A5's backup and read-back.

- **A10. One normalization: a body without its H1.** Nineteen bodies (#948-#993, #1470, #1471)
  hold an earlier committed spec with its title line removed. A blob *with its H1 removed* is the
  blob minus its first two lines, defined only when the first line is `# <title>` (A2) and the second
  line is empty. A GitHub body that matches (A3, one trailing newline allowed) the H1-removed form of
  the source blob or of a history blob counts as a body match for `fast-forward`. It never makes an
  issue `in-sync`: `in-sync` still needs the full blob, so the sync restores the title line. Nothing
  else is normalized; any other difference stays `unmatched`.
- **A11. A reviewed body: `--reviewed DIR`.** Some rulings sync an issue whose body is not an earlier
  committed state (a body from an unmerged branch, a body with extra trailing blank lines, a title
  set on GitHub). Root rules on each after a three-way reconcile. `--reviewed DIR` takes a directory
  that an earlier `--reconcile-dir` run wrote. For an `unmatched` issue that is named explicitly on
  the command line (never through `--all` or `--range`), the issue becomes `fast-forward` (printed
  as `<n> fast-forward reviewed`) only when the current GitHub body is byte-equal to
  `DIR/<n>.github.md` and the current GitHub title is byte-equal to `DIR/<n>.github-title.txt`. Any
  change since the review leaves it `unmatched`. `--reconcile-dir` therefore also writes
  `<n>.github-title.txt`, the title with no trailing newline. `--reviewed` changes nothing else: the
  write is A5's, with the backup, the race check and the read-back, and the spec is still the
  `origin/main` blob.
- **A12. Self-test additions.** A body equal to a history blob with its H1 removed is
  `fast-forward` and the read-back has the full blob. A body equal to the H1-removed form but with
  any other one-byte change is `unmatched`. A body with the H1 line removed but the blank line kept,
  or a blob whose second line is not empty, is `unmatched`. With `--reviewed`: a body and title equal
  to the reviewed files sync; a one-byte change to either since the review is `unmatched`; an issue
  selected with `--all` is not overridden. Each new assertion has a mutation run in the Evidence.

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
   - (Amendment 1) skip the `ls-remote` comparison: the stale-source assertion;
   - read the spec from the working tree: the source assertion;
   - treat any body as history (no `unmatched` class): the unmatched assertion;
   - key the history by path instead of issue number: the renamed-spec assertion;
   - skip the backup write: the backup assertion;
   - remove the ceiling check: the oversize assertion;
   - skip the pre-write re-fetch comparison: the race assertion;
   - skip the read-back comparison: the one-byte read-back assertion.
3. `bash scripts/check-workspace-policy.sh`, `python3 -B scripts/check-script-reachability.py` and
   `python3 -B scripts/test-script-reachability.py` exit 0.
4. **A9's run** (replaces D4's): its `--check` and `--apply` output and the backup directory's file list.

*Test value.* The self-test is red if the sync script accepts a body that differs from its spec,
edits a closed issue, or maps a brief under `BRIEFS/` to an issue number. No other check guards the spec-equals-GitHub rule.

## Evidence

Attempt 1 (2026-10-11). Gate 4 (A9's run) is root's and is not recorded here.

- **Gate 1.** `bash scripts/operator/test-sync-spec-bodies.sh` exits 0: 74 `ok - NAME` lines and
  `all assertions passed` (about 5 s; temporary directory removed by trap).
- **Gate 2, mutations.** Each applied alone to `sync-spec-bodies.sh`, the self-test run, then the
  file restored (byte-identical to the unmutated copy). Every run exited 1. Count of red
  assertions, and the assertion(s) the spec names:
  - comparison always succeeds (`matches_blob` returns 0): 30 red, including `readback-one-byte-fails`;
  - no trailing-newline allowance (`matches_blob` ends `return 1`): 29 red, including
    `readback-trailing-newline-dropped-passes`;
  - drop the closed-issue check (classify `state != OPEN`): 13 red, including `class-closed-skipped`.
    `closed-never-edited` stays green because the write phase's own re-fetch (A5 step 1) refuses a
    non-OPEN issue (`12 refused`, exit 1) before any edit; the two checks back each other;
  - drop the top-level filter (`[[ $base != */* ]] || continue`): 3 red, `range-selects-changed-top-level-specs`,
    `range-reads-brief-as-nothing`, `range-apply-never-reads-issue-36`;
  - skip the `ls-remote` comparison: 5 red, `stale-origin-ref-refused`, `stale-origin-ref-no-gh-call`,
    `stale-origin-ref-no-backup-dir-written`, `locally-moved-origin-ref-refused`,
    `locally-moved-origin-ref-no-gh-call`;
  - read the spec from the working tree (`read_source` uses `cat`): 1 red,
    `source-is-origin-main-blob-not-working-tree`;
  - no `unmatched` class (the fallback class is `fast-forward`): 16 red, including
    `class-unmatched-branch-only-blob`, `unmatched-never-edited` and the four `reconcile-*` file assertions;
  - history keyed by path instead of issue number: 33 red, including `renamed-spec-body-is-fast-forward`;
  - skip the backup write (`save_backup` returns 0): 5 red, `backup-saved-before-each-edit`,
    `backup-equals-served-raw-json-10`, `backup-equals-served-raw-json-16`,
    `backup-holds-fast-forward-set-only`, `readback-failure-keeps-the-backup`;
  - remove the ceiling check (`size > ceiling`): 3 red, `oversize-class`, `oversize-never-edited`,
    `oversize-other-fast-forwards-still-written`;
  - skip the pre-write re-fetch comparison: 4 red, `race-refused-exit-1`, `race-refused-line`,
    `race-no-edit`, `race-no-backup-written`;
  - skip the read-back body comparison: 4 red, `readback-one-byte-fails`, `readback-mismatch-line`,
    `readback-stops-later-writes`, `readback-names-the-backup-file`.
- **Gate 3.** `bash scripts/check-workspace-policy.sh` (`workspace policy: ok`),
  `python3 -B scripts/check-script-reachability.py` (`script reachability: ok`, 8 files under
  `scripts/operator/` exempt) and `python3 -B scripts/test-script-reachability.py` (20 cases) exit 0.
  `README.md` is a `dsp-research` input of the path router, so `python3 -B scripts/check-ci-path-routing.py`
  and `python3 -B scripts/test-ci-path-routing.py` were run and exit 0. `shellcheck` is not installed
  on the build host; `bash -n` passes.
- **Smoke test, read-only.** `bash scripts/operator/sync-spec-bodies.sh --check --all` against the real
  repository and GitHub on `origin/main` `1b55de30a` (about 3 minutes, no `gh` write): exit 3, 305 top-level
  specs, `summary in-sync=176 fast-forward=17 closed-skipped=81 no-issue=0 oversize=0 unmatched=31`.
  The `unmatched` lines say which half failed (`body=no-committed-state-matches` or
  `title=no-committed-title-matches`) and give the nearest history blob; for example #291 matches a
  committed body and differs in the title. The trailing-newline assumption holds (176 bodies match).

Attempt 1 verdict follow-ups (2026-10-11, on `342ab06a5` PASS; verdict
`submix-verdicts/1445-attempt1.md`). A1-A9 semantics are unchanged. Implementation notes to
Amendment 1, each inside the amendment's contract:

- **MINOR 3, repository binding (A4/A5).** Every `gh issue view` and `gh issue edit` now carries
  `--repo OWNER/REPO`, derived from `git remote get-url origin` (`https://`, `ssh://` and
  `git@github.com:` forms of github.com; any other URL exits 2 before any `gh` call). The script also
  sets `GH_REPO` to the same value, so an ambient `GH_REPO` cannot redirect it. `git remote get-url`
  applies `url.*.insteadOf`, so the derived repository is the one `ls-remote` proves.
- **MINOR 2, directories (A3/A4).** `--backup-dir` and `--reconcile-dir` are made absolute against
  the caller's directory before the `cd` to the repository root; the path that is checked is the
  path that is used, and messages print the absolute path.
- **NIT 1, title sanity (A6/A7).** A spec title with a control character (CR included) or leading or
  trailing whitespace is class `oversize` (exit 3, never edited), because GitHub may normalize such a
  title and the read-back would fail after the body write. The line says why:
  `<n> oversize title=control-character-or-edge-whitespace` (also `size=above-ceiling`,
  `title=above-256-characters`). No new class and no new exit code; A7 is unchanged. The 305 specs on
  `origin/main` `1b55de30a` have no such title (verdict).
- **NIT 2, 3, 4, 6, 7.** The refusal message names a rollback copy only when this run saved it. The
  rollback copy is written with `noclobber` (O_EXCL), so a file that appears after the pre-check is
  never overwritten (`<n> refused`, exit 1). `--apply` exits 1 if synced differs from the
  fast-forward count (a guard against a later early exit; no fixture can reach it without a code
  defect, so it has no assertion). The history `git log` has `--root`, so `log.showRoot=false` cannot
  drop the root commit's blobs. The backup-directory pre-check now runs before `git ls-remote`
  (selection reads only the local `origin/main` ref; the source is still proven current before any
  `gh` call). NIT 5 (pull requests) is not changed.
- **Gate 1.** The self-test now has 122 `ok - NAME` lines (was 74) and prints `all assertions passed`.
  New fixtures: issues 21-30 (titles with trailing space, CR, leading space, embedded control
  character, 257 and 256 characters; a body that is another issue's blob; a body that is a
  `BRIEFS/` blob; a spec with no H1), a git wrapper on `PATH` that answers `remote get-url origin`
  and logs `ls-remote`, stub switches for a state change between fetch and write and after the
  edit, a read-back title corruption and a backup file that appears during the run.
- **Mutations of the new assertions** (each applied alone to `sync-spec-bodies.sh`; self-test exit 1;
  then restored, byte-identical). Red count and the assertion(s) that go red:
  - `history_of` returns every issue's rows (not keyed by number): 7 red, `history-is-keyed-by-issue-number`,
    `summary-counts`, `unmatched-never-edited`;
  - history path reduced to its basename (so `BRIEFS/029-y.md` counts as issue 29): 7 red,
    `brief-blob-is-not-history`, `summary-counts`, `unmatched-never-edited`. The verdict's mutation
    (only the `name ~ /\//` line removed) stays green and is an equivalent mutant: the next line,
    `name !~ /^[0-9]+-[A-Za-z0-9._-]+\.md$/`, rejects `BRIEFS/...` on its own, so both lines have to go
    (the basename mutation does that);
  - read-back title comparison removed: 3 red, `readback-title-differs-fails` and two more;
  - "title unchanged" read-back comparison removed: 2 red, `title-changed-though-spec-has-none-fails`,
    `title-changed-though-spec-has-none-says-so`;
  - write-phase `OPEN` check removed: 2 red, `closed-before-the-write-refused-line`,
    `closed-before-the-write-no-edit`;
  - read-back `OPEN` check removed: 2 red, `closed-by-the-edit-fails-the-read-back`,
    `closed-by-the-edit-mismatch-line`;
  - 256-character title limit removed: 2 red, `class-title-257-characters-is-oversize`,
    `summary-counts`; limit off by one (`>=`): 2 red, `class-title-256-characters-is-in-sync`,
    `summary-counts`;
  - duplicate-spec refusal removed: 2 red, `duplicate-spec-for-one-issue-refused`,
    `duplicate-spec-for-one-issue-no-gh-call`;
  - `--repo` dropped from `gh issue view`: 105 red (the stub refuses the argv); dropped from
    `gh issue edit`: 29 red; `GH_REPO` export removed: 1 red, `ambient-gh-repo-cannot-redirect`;
    underivable-URL refusal removed: 6 red, `underivable-origin-url-no-gh-call-*`;
  - `--backup-dir` not made absolute: 4 red, `relative-backup-dir-lands-in-the-callers-directory`
    and three more; `--reconcile-dir` not made absolute: 2 red,
    `relative-reconcile-dir-lands-in-the-callers-directory`,
    `relative-reconcile-dir-leaves-the-repo-root-copy-alone`;
  - title unsafe checks removed one at a time: control character 2 red
    (`class-title-embedded-control-character-is-oversize`), leading whitespace 8 red
    (`class-title-leading-space-is-oversize`), trailing whitespace 9 red
    (`class-title-trailing-space-is-oversize`);
  - refusal message always names the backup: 1 red, `refusal-before-the-backup-names-no-backup-file`;
  - `save_backup` back to a plain `cp`: 3 red, `backup-never-overwrites-a-file-that-appeared`,
    `backup-that-appeared-is-left-as-it-was`, `backup-that-appeared-no-edit`;
  - `--root` removed: 1 red, `root-commit-blobs-are-history-despite-showroot-false`;
  - an `ls-remote` call before the backup pre-check: 2 red, `existing-backup-file-refuses-before-ls-remote`,
    `ls-remote-runs-once-in-a-normal-run`;
  - synced-equals-fast-forward guard removed: 0 red (not reachable by a fixture, see above).
- **Gate 3** re-run after the follow-ups: `check-workspace-policy.sh` (`workspace policy: ok`),
  `check-script-reachability.py` (ok), `test-script-reachability.py` (20 cases), `check-ci-path-routing.py` and
  `test-ci-path-routing.py` exit 0; `bash -n` passes on both scripts.
- **Smoke test, read-only.** `bash scripts/operator/sync-spec-bodies.sh --check 1438 1445 291` against
  the real repository and GitHub on `origin/main` `1b55de30a`: `1438 fast-forward`, `1445 in-sync`,
  `291 unmatched` (title), no `gh` write.
- **Gate 4: A9's run (root, 2026-10-11)**, on `origin/main` `1b55de30a`:
  - `--check --all` exited 3 with `summary in-sync=176 fast-forward=17 closed-skipped=81 no-issue=0
    oversize=0 unmatched=31 synced=0`.
  - `--apply --backup-dir /home/bl/misofm/submix-verdicts/body-sync-backup-2026-10-11 763 881 887 888 889
    1008 1010 1019 1373 1374 1375 1376 1378 1379 1422 1438 1487` exited 0 with 17 `synced`.
  - A following `--check` of those 17 issues exited 0 with `in-sync=17`.
  - The backup directory holds 17 files, `<n>.json`, one per synced issue.
  - The 31 `unmatched` issues were not written: 26, 124, 291, 559, 560, 948, 951, 952, 955, 961, 968, 969,
    972, 973, 974, 975, 987, 988, 989, 991, 992, 993, 1309, 1311, 1312, 1314, 1343, 1348, 1468, 1470, 1471.
    They are reconciled by hand, with a recommendation each, in
    `/home/bl/misofm/submix-verdicts/body-sync-unmatched.md`.

## Dependencies

- None. It may land in any J batch. A2 and C2 use it when it is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: two hours.
