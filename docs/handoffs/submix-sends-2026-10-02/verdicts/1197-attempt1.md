# #1197 *Record the submix, send and VCA ruling*: Sol verdict, attempt 1

- Reviewed: `git diff fe8ac679 f622dad1` only (commit `f622dad1`, branch `codex/batch-submix-k0`,
  worktree `/home/bl/misofm/wt-submix-k0`). Nothing in the worktree, the branch or GitHub was changed
  by this review. The worktree's uncommitted crate edits belong to another implementer and were
  ignored.
- Binding: `AGENTS.md` and `.github/ISSUE_SPECS/1197-record-the-submix-send-and-vca-ruling.md`.
- Owner authority facts I was given: on 2026-10-02 the owner decided O1 (submixes carry every
  console slot plus inserts, fader/mute and pan/matrix) and O2 (reverb out of scope), and delegated
  VCA and the submix channel model/input section to the adversarially verified opinion. The owner
  has **not** reviewed P1-P17 one by one, and Q1-Q4 are open.

## Verdict: FAIL

Two MAJOR findings, both small text fixes. Every objective gate passes, and GitHub is in exact sync.
Attempt 2 should be a short docs-only revision.

## What passed (verified)

- **Gate 1.** I ran `scripts/check-workspace-policy.sh` (prints `workspace policy: ok`, exit 0) and
  `scripts/test-workspace-policy.sh` (exit 0) on an exact snapshot of `f622dad1`
  (`git archive` into scratch, then `git init`).
- **Gate 2.**
  - `gh issue list --state open` has 123 issues, and `.github/ISSUE_SPECS/` at `f622dad1` has 123
    numbered specs. The two sets are equal.
  - For **all 34** new issues (#1196-#1229), I compared GitHub against the local file:
    - the state is OPEN;
    - the GitHub title equals the file's H1;
    - the GitHub body equals the file byte for byte, after trimming.
  - The umbrella #1196 lists slices 00-28 (18a = #1215, 18b = #1216) with their numbers. #1227-#1229
    are standalone successors. The umbrella has no GitHub sub-issues, and #1227 has no parent.
- **Gate 3.** `python3 -B scripts/ci-path-router.py --event push --path AGENTS.md --path
  docs/rulings/x.md` prints `full`. The same router on `--base fe8ac679 --head f622dad1` also prints
  `full`.
- **Gate 4.** No test or code is changed. The changed files are ISSUE_SPECS, `AGENTS.md`,
  `docs/IMPLEMENTATION_PLAN.md`, `docs/rulings/` and `docs/handoffs/`.
- **Cross-references.**
  - Every `*Title* (#N)` and `*Title*, #N` in the 53 changed files resolves to the exact GitHub
    title of #N.
  - Every italic title in each Dependencies section is an exact published title, or the title of a
    VCA draft.
  - The dependency graph matches the umbrella table.
- **Ruling content.** Deliverable 1 is present in the decision-12 format:
  - O1-O5;
  - delegated answers (a) and (b), with the VCA sequencing, and with only V5 waiting on #1053 and
    #1226;
  - P1-P17 recorded as **subject to owner review**, with P11, P16 and P17 spelled out;
  - Q1-Q4 recorded as open;
  - BM1-BM3 as successors, and route fusion left to the weekly pass;
  - the batches, and the slice that removes each qualifier.

  The verdict summaries of VERIFY-1/2/3 and APPLIED-3 match those documents. Its code anchors
  (`model.rs:577`, `runtime.rs:22-24`) are correct on `fe8ac679`.
- **#210 and #1053 annotations.**
  - Both are append-only.
  - The #1053 text carries every clause of the spec's quoted paragraph, numbered.
  - `live_builtin_delta` (#1053 `:25`) and the host-core fields `tracks` and `track_controls`
    (`prepare.rs:344`, `:349`) exist.
- **Qualifier removal.**
  - #1205 deliverable 6 covers the dual-mono, chain, console-slot and seven-tap sentences, and
    authorizes `AGENTS.md` (those qualifiers only).
  - #1224 deliverable 5 does the same for the route-mute and follow-mute sentences.
- **Evidence folder.** `docs/handoffs/submix-sends-2026-10-02/` holds `DESIGN` (revision 2),
  `VERIFY-1..3`, `REVISION-1..2`, `APPLIED-3`, the V0-V5 drafts and `ISSUE-MAP.md`, which maps plan
  labels to issues. `ISSUE-MAP.md` states candidly that the probe sources were never committed.
- **Index line.** The index line is in `docs/IMPLEMENTATION_PLAN.md`'s Index table.

## Findings

### MAJOR-1: `AGENTS.md` presents planner decisions as owner-approved

`AGENTS.md` is binding on every agent. The ruling file separates the three kinds of authority with
care, but the `AGENTS.md` text does not:

- The route-mute and follow-mute sentences read: "A route may be muted, and a route into a submix
  may follow its source strip's mute; a route into the output never follows. **Approved by decision
  13** ...". Route mute is planner decision **P3**. Follow-mute, and its illegality on output
  routes, is planner decision **P11**. The ruling says both are "subject to owner review: the owner
  has not reviewed them one by one".
- The new parenthetical reads: "submix strips, sends and VCA groups are **owner decision 13**". This
  labels the send machinery (P2-P11, P13) as owner decisions.
- Related overstatements outside `AGENTS.md`:
  - The umbrella #1196 opens with "**The owner decided this** on 2026-10-02". "This" is the whole
    umbrella, including live sends, route mute and follow-mute. Its Authority section then corrects
    this.
  - The ruling's H1 is "owner rulings of 2026-10-02".
  - The ruling's **Owner decision** bullet for O1 includes "Its input is the master-plan D9 sum ...".
    That clause is not in DESIGN 2.1's O1 row. It comes from the design summary.
- **Root cause.** The spec's D5 template mandates the wording "Approved by decision 13", and its
  Dependencies line assumes "the owner's acceptance of DESIGN.md revision 2". That acceptance
  happened for O1, O2 and the delegation, not for P1-P17. The implementer followed the spec, but the
  result overstates the owner's authority in the binding file.
- **Fix.** Do one of the following:
  - Reword the route qualifier, for example: "Planned under decision 13 (#1196) as planner
    decisions P3 and P11, subject to owner review; landing with *Mute a route in the session*
    (#1216) for route mute and ...". Change the parenthetical to "... are decision 13 (owner,
    delegated and planner authority as marked there), ...". Then:
    - change #1196's first sentence to "The owner's decisions of 2026-10-02 and the planner's
      verified design are recorded as decision 13";
    - retitle the ruling H1 "owner rulings and planner decisions of 2026-10-02", or add a "(see each
      bullet's authority)" note;
    - move "Its input is the D9 sum" out of the owner bullet, or mark it as the design's
      consequence of O1;
    - amend spec #1197 D5 and deliverable 2 so the template allows this wording. The GitHub body
      syncs at the batch push.
  - Or obtain the owner's explicit acceptance of P3 and P11, record it verbatim in the ruling, and
    leave "Approved" in place.

### MAJOR-2: "Inserts are per track" now contradicts the amended paragraph, and the plan keeps the contradiction

- **Where.** In the amended console paragraph (`AGENTS.md:30` at `f622dad1`), the sentences
  "Inserts are per track and ordered. Either console section and a track's inserts may be empty"
  are unchanged.
- **The contradiction.** O1 gives every submix ordered inserts. The new text says "The chain applies
  to every strip", and that chain includes `inserts`.
- **Why it persists.** #1205 deliverable 6 removes the K1 qualifiers "changing nothing else". After
  the K1 push, binding `AGENTS.md` would therefore say both "every strip runs the chain (with
  inserts)" and "Inserts are per track", with no slice scheduled to fix it. The spec's own hazard,
  "Paraphrase drift", names this risk.
- **Fix.** Do one of the following:
  - Change the sentences to "Inserts are per strip and ordered. Either console section and a strip's
    inserts may be empty", and extend the K1 qualifier to cover them: "the submix half lands with
    *Render a submix strip on its summed input*".
  - Or extend #1205 deliverable 6 to make this change at K1, and sync its GitHub body at the batch
    push.

  Optionally, apply the same to "A native effect may run track-locally as an insert" in Effects and
  plugins.

### MINOR-1: The spec's decision record does not show what the implementation learned

- Spec #1197 still says:
  - "Dependencies: ... It needs the owner's acceptance of DESIGN.md revision 2";
  - Deliverable 1: "every planner decision P1-P17 the owner accepts".
- The umbrella's row 00 says "owner acceptance of the design".
- The implementation correctly recorded P1-P17 as unreviewed, but the spec does not record:
  - what the owner actually accepted (O1, O2, the delegation O3, on 2026-10-02);
  - how the dependency is therefore met;
  - who authorized "the slices implement them as written until the owner rules otherwise".
- `AGENTS.md` requires the spec's decision record to be updated as implementation learns facts.
- **Fix.** Amend the Dependencies line and deliverable 1 of #1197 (and #1196's row 00) to match the
  ruling. Sync both at the batch push.

### MINOR-2: The VCA sentence misstates how VCA mute works

- `AGENTS.md` says a VCA "offsets its members' faders and mutes its members". This reads as an
  unconditional mute.
- The ruling (delegated (a)) says the VCA's per-lane mute **ORs into** each member's effective mute.
- **Fix.** "A VCA group is control-only: it carries no audio; its dB offset adds to its members'
  faders, and its mute ORs into their mutes."

### MINOR-3: No slice owns removing the VCA qualifier

- The ruling says "the VCA batch's closing slice removes the one on the VCA sentence". None of the
  drafts V0-V5 has that deliverable, or an `AGENTS.md` path.
- "Closing slice" is also ambiguous. V4 closes the browser batch, while V5 waits on #1053 and #1226.
- **Fix.**
  - Name the slice in the ruling (V4 or V5).
  - Add the deliverable and the path "`AGENTS.md` (that qualifier only)" to that draft. If the
    handoff drafts are frozen, record the obligation in `ISSUE-MAP.md`'s "Not filed" note, so it
    is applied when the umbrella is filed.

### MINOR-4: The spec does not authorize the evidence folder the commit adds

- The commit adds 14 files under `docs/handoffs/submix-sends-2026-10-02/`.
- The spec's Context presumes that record "is committed", but Authorized paths lists only
  `docs/rulings/`, `AGENTS.md`, `.github/ISSUE_SPECS/` and `docs/IMPLEMENTATION_PLAN.md`.
- **Fix.** Add `docs/handoffs/submix-sends-2026-10-02/` (the design record, the verifications, the
  revisions, `ISSUE-MAP.md` and the unfiled V0-V5 drafts) to Authorized paths.

### MINOR-5: The owner's words are not recorded anywhere

- The ruling attributes O1, O2 and O3, and, as "owner direction", O4 and O5, to the owner. Neither
  the ruling nor the evidence folder quotes the owner or names a source. A grep finds no verbatim
  owner text in the folder.
- I could confirm O1, O2 and O3 against the facts I was given. I could not confirm **O4** ("any tap,
  a full 2x2 per send, fan-out") or **O5**.
- O4 is what makes the `AGENTS.md` sentence "every strip has all seven [taps]" owner-backed rather
  than planner-backed: the submix tap source is P2.
- **Fix.** Quote the owner's 2026-10-02 answers in the ruling, as the memory files do. Otherwise,
  relabel O4 and O5 as standing direction and cite their source: `AGENTS.md` "arbitrary track
  counts" and the SIMD principles.

### NIT-1: `AGENTS.md` pins the D9 summation order

- "(stable edge-ID order, left to right)" writes D9's order into binding text.
- The owner's standing summation-order ruling allows a measured class-B reorder. Q5 anticipates
  exactly that for route order.
- **Fix.** Cite "master plan #83 D9" (matching the existing "master plan D4" style) without
  restating the order.

### NIT-2: Five spec filenames are not slugs of their titles

- #1203 `tap-a-submix-strip-at-any-send-point`
- #1205 `build-submix-strips-in-the-sdk-...` (the title includes "and bus taps")
- #1208 `meter-and-designate-submix-strips-in-host-core`
- #1210 (the title includes "the SDK")
- #1212 `add-the-not-soloable-command-reason`

There is precedent (#1102). However, the delivery rule asks that the number and title match the
filename. Rename the files if cheap.

### NIT-3: An unowned stale line in the implementation plan

`docs/IMPLEMENTATION_PLAN.md:31` ("Tracks are dual-mono ... SIMD racks 1 and 2 ... dynamic rack") is
already stale and no slice owns it. Consider a line in #1205 or a housekeeping issue.

### NIT-4: The probe is not reproducible from the repository

The leak #1196 cites (0.3155/0.3549) comes from probe sources that were never committed.
`ISSUE-MAP.md` says so, and #1218 re-measures it as PR evidence. No action is needed beyond that
candour.

## GitHub divergences expected to sync at the batch push

- **#210.** The GitHub body equals the base spec. It lacks only the new 3-line annotation.
- **#1053.** The GitHub body diverged from the local spec **before** this commit: it predates
  #1095's "console → live controls" rename (local lines 15, 23-24, 30-31, 54, 69-70, 122-124, 215,
  plus a trailing blank line). It also lacks the new A2 D1 coordination block. Push the whole local
  body at the batch push, not only the annotation.
- **The 34 new issues.** All are already in exact sync. Any attempt-2 edit to #1196, #1197 or #1205
  must be re-synced to GitHub at the batch push.

## Test value

No test is added or changed (gate 4), so no test-value question applies.
