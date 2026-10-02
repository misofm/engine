# #1197 *Record the submix, send and VCA ruling*: Sol verdict, attempt 2

- **Reviewed:** `git diff fe8ac679 45c1a342` and `git show 45c1a342` only. The commit is on branch
  `codex/batch-submix-k0`, worktree `/home/bl/misofm/wt-submix-k0`. This review changed nothing in
  the worktree, the branch or GitHub. The branch has since moved on to `0a769a63` (#1198, another
  implementer), and I did not review that commit.
- **Binding documents:** `AGENTS.md` at `45c1a342` and
  `.github/ISSUE_SPECS/1197-record-the-submix-send-and-vca-ruling.md` at `45c1a342`.
- **Authority source:** I checked the owner's words against the parent session transcript
  (`~/.claude/projects/-home-bl-misofm-engine/258223fb-….jsonl`):
  - the owner's messages at 16:03:30Z, 16:11:56Z and 16:14:28Z;
  - the planner's messages to the owner at 16:04:49Z and 16:12:11Z.

## Verdict: PASS

There are no BLOCKER or MAJOR findings. Every attempt-1 MAJOR and MINOR is resolved. There are two
new MINOR findings and five NITs. None of them blocks the PASS, and each can be fixed in K0 or
carried into #1205. All four objective gates pass.

## Gates (re-run on an exact snapshot of `45c1a342`)

I created the snapshot with `git worktree add --detach <scratchpad>/v1197-a2 45c1a342` and removed
it afterwards with `git worktree remove`.

1. `bash scripts/check-workspace-policy.sh` printed `workspace policy: ok` and exited 0.
   `bash scripts/test-workspace-policy.sh` exited 0 (`workspace policy mutation tests: ok`).
2. GitHub sync:
   - `gh issue list --state open` shows 123 issues, and `.github/ISSUE_SPECS/` holds 123 numbered
     specs. The two sets are equal.
   - For all 34 issues #1196-#1229:
     - each one is OPEN;
     - its GitHub title equals the spec's H1;
     - its GitHub body equals the spec file after trimming. The edited bodies of #1196, #1197 and
       #1205 were re-synced at 20:27Z.
   - The umbrella has no GitHub sub-issues, so BM1-BM3 are not listed as children.
   - The 20 title mismatches that remain across all specs are old specs without an H1, and none is in
     scope.
   - #210 and #1053 still differ from GitHub only by the annotations. #1053 also predates the earlier
     #1095 rename, as attempt 1 recorded. Both are expected to sync at the batch push. Neither
     changed in attempt 2.
3. `python3 -B scripts/ci-path-router.py --event push --path AGENTS.md --path docs/rulings/x.md`
   prints `full`.
4. Docs and specs only. The changed paths are `.github/ISSUE_SPECS/`, `AGENTS.md`, `docs/rulings/`
   and `docs/handoffs/submix-sends-2026-10-02/`. No test is added.

## Attempt-1 findings: status

- **MAJOR-1 (planner decisions presented as approved): resolved.**
  - `AGENTS.md` now labels each point with its authority:
    - "Approved by decision 13" appears only on the owner's decision O1 and on the owner-delegated
      answers (a) and (b);
    - "Planned under decision 13 …, subject to owner review" appears on the inserts, mute and
      pan/matrix of a submix strip, the seven submix taps (P2), route mute (P3) and follow-mute (P11).
  - The parenthetical now reads "decision 13 …, which marks each point's authority".
  - #1196's first sentence and its Authority section are honest, and the ruling's H1 now reads
    "owner rulings and planner decisions".
  - The D9 sum is now marked "Design consequence, not the owner's words".
  - The spec's D5 template states that "A qualifier never calls a planner decision 'approved'".
- **MAJOR-2 (insert contradiction): resolved.**
  - The text now reads "Inserts are per strip … a strip's inserts", with a planned-under qualifier
    that lands with #1200.
  - #1205 deliverable 6 removes the qualifier at K1 and changes "track-locally" to "strip-locally".
    `AGENTS.md` is authorized for those two changes only.
- **MINOR-1 (decision record): resolved.**
  - The spec's Context, deliverables 1 and 2, the Dependencies section and a new Decision record now
    say what the owner accepted.
  - #1196 row 00 reads "the owner's answers of 2026-10-02 (met)".
- **MINOR-2 (VCA mute wording): resolved.** The text reads "its dB offset adds to its members'
  faders, and its mute ORs into their mutes", which matches ruling (a).
- **MINOR-3 (VCA qualifier owner): resolved.**
  - The ruling names V4, and V0's table shows that V4 closes the batch V1-V4.
  - The V4 draft has deliverable 6 and the path "`AGENTS.md` (that qualifier only)".
  - `ISSUE-MAP.md` records the obligation for the filed V4.
- **MINOR-4 (evidence folder unauthorized): resolved.** The handoff folder is now an authorized
  path.
- **MINOR-5 (owner words): resolved.**
  - I checked W1-W5 against the transcript. W1-W4 are exact, typos included. W5 is exact up to
    "then implement" (see NIT-1).
  - O4 and O5 are relabelled as owner direction read by the planner, subject to owner review.
  - The preface's account of what the owner was told before W5 ("large-format desks give buses the
    same EQ and dynamics …; each track has its own settings") is accurate (16:12:11Z).
- **NIT-1 (summation order): resolved.** `AGENTS.md` no longer pins the order. Only the ruling's
  design-consequence line cites D9.
- **NIT-2 (filenames): resolved for the five files.**
  - The five renames are 100% similarity, except #1205's intended edit.
  - No reference to an old name remains anywhere in the tree.
  - `ISSUE-MAP.md` records the renames, and the plan-file names that DESIGN and REVISION-2 cite are
    unaffected.
  - See NIT-2 below for #1198.
- **NIT-3 (stale plan line): partly addressed.** #1205 deliverable 7 is added; see NIT-4 below.

## Qualifier ownership (checked)

Every qualifier in `AGENTS.md` has a named slice whose spec carries the removal and the `AGENTS.md`
path:

| `AGENTS.md` line | Qualifier | Removed by |
|---|---|---|
| 24 | dual-mono strip | #1205 deliverable 6 |
| 28 | chain | #1205 deliverable 6 |
| 30 | console slot | #1205 deliverable 6 |
| 30 | strip insert | #1205 deliverable 6 |
| 32 | seven taps | #1205 deliverable 6 |
| 32 | route mute and follow-mute | #1224 deliverable 5 (generic wording, still accurate) |
| 32 | VCA | V4 draft deliverable 6, and `ISSUE-MAP.md` |

Every "until it lands" clause is true at `fe8ac679`. After each removal, the sentences that remain
stay true on every host at preparation. They make no live C ABI claim, which waits on #1225, #1226
and V5.

## New findings

### MINOR-A: The banking paragraph remains track-only after K1, and no slice owns updating it

- **Where:** `AGENTS.md:34`. It says buffers are "banked AoSoA across tracks", that a vector holds
  "the same dual-mono lane from four Wasm/NEON tracks or eight AVX2 tracks", that "parameters/state
  remain per-track", and that "incompatible tracks form another cohort".
- **Why it is stale:** #1202 D3 puts a bus console lane in the same (slot, pool class, level)
  groups. Lines 30 and 42 together then make submix lanes bank. After K1, the binding description
  of the bank layout therefore omits submix strips.
- **Why no slice fixes it:** #1205 deliverable 6 says "changing nothing else".
- **Why this is MINOR and not a contradiction:**
  - nothing says "only tracks bank";
  - the specific rules (lines 30 and 42) govern;
  - #1202's spec is explicit.
- **Fix:** extend #1205 deliverable 6, the same way as "track-locally". In line 34, change
  "tracks" to "strips" ("banked AoSoA across strips", "lane from four … strips", "per-strip",
  "incompatible strips"). Sync #1205 at the batch push.

### MINOR-B: The ruling's O3 reading of "submix inputs" is inverted relative to the transcript, and it omits a reversal the owner was not told

- **What the ruling says:** "The planner reads that as the submix strip's input section and, with
  it, its channel model … The channel-model half rests on that reading."
- **What the transcript shows:**
  - The question the owner was answering was the planner's question 4 (16:04:49Z): "**Submix
    input:** a bus sums panned tracks, so I'd make it a true L/R stereo strip and not dual-mono.
    Agreed?" The channel model is therefore the direct referent of the delegation, and the input
    section is the inference.
  - At 16:12:11Z the planner told the owner that a submix "drops the source-specific parts of the
    input section, such as trim and polarity". Delegated answer (b) gives all five keys, which
    reverses the last thing the owner heard. The ruling does not record this reversal.
- **Authority impact:**
  - **Line 24** (dual-mono, "Approved … owner-delegated answer (b)") is in fact better founded than
    the ruling says.
  - **Line 28** ("owner-delegated answer (b) (its input section)") rests on the inference. It is
    defensible, since "inputs" plainly covers an input section, so nothing is overstated in binding
    text.
  - **The ruling** misplaces where the inference lies, and that matters for later owner review.
- **Fix:**
  - quote question 4 next to W5 in the ruling;
  - say that the input-section half is the reading;
  - add one line: "(b) keeps trim and polarity, which the planner had told the owner a bus would
    drop".

### NIT-1: W5 is marked "verbatim" but is cut short without a marker

W5 drops the trailing " - use a fresh agent for each step, opus 5.5 please." Either add "[…]" or add
the clause. The clause is a process instruction, so authority is unaffected.

### NIT-2: The #1198 filename is still not the slug of its title

The filename is `1198-iterate-session-strips-not-tracks`. The title ends "…, wherever strip
semantics apply". Attempt 1 missed this file. It is understandable to leave it alone while #1198 is
being implemented in the same worktree. Rename it at a safe point, or note in `ISSUE-MAP.md` why it
is kept.

### NIT-3: The parenthetical names three kinds of authority

The `AGENTS.md:30` parenthetical lists "owner, owner-delegated or planner". The ruling defines four
kinds, and the fourth is owner direction read by the planner. Say "four kinds", or list all four.

### NIT-4: The plan sentence keeps "a track's inserts"

#1205 deliverable 7 makes the `docs/IMPLEMENTATION_PLAN.md:31` sentence begin "Strips (tracks and
submixes) are dual-mono". The same sentence still says "a track's inserts to the dynamic rack", and
it keeps the stale rack wording. Consider "a strip's inserts".

### NIT-5: Generic "approved" and "decided" wording remains

- #1197's Product outcome says `AGENTS.md` states "what the engine has been approved to promise".
- #1196's heading reads "Design (decided; DESIGN section 5)".
- Both read correctly next to the Authority sections. "Approved or planned" or "decided (authority
  per point in decision 13)" would match the new discipline.

## Test value

No test is added or changed (gate 4), so no test-value question applies. The gates that do
discriminate are:

- the policy scripts;
- the exact GitHub body and title comparison;
- the router check.

Each would turn red on a regression that matters here: a forbidden path, a GitHub issue out of sync
with its spec, or `AGENTS.md` misclassified as evidence.
