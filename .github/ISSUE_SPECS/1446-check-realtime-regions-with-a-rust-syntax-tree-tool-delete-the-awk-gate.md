# Check realtime regions with a Rust syntax-tree tool: delete the awk gate

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice C2 of ten (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a (#1442), #1519, C2, B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what each
holds).
- A1 to B2a moved every check of `scripts/check-realtime-policy.sh` into `tools/realtime-policy`.
  The fourth review measured that A1 to B2a cover the awk gate (its BLOCKER-1, option B), so C2
  comes before B2b-1. Root's ruling 2 on that review (option (C)) puts the #1448 guard commit
  after this slice, and B2b-1 and B2b-2 after the guard; none of them edits the awk gate.
- **C2 (this issue)** deletes the awk script and its self-test. CI then runs only the tool, the
  routing checker holds the tool's step as the one owner of the claim, and docs, comments and open
  specs that use either script as a gate are brought up to date. C1's script syncs the edited
  specs to GitHub.

No production code changes. Rust source changes are comment lines only, and each edited comment
keeps its line count.

## Problem (verified on `origin/main` at `6d28a80ec`)

- **Two owners per check.** After B2a, the tool runs every check of the script. The script and
  its self-test still run in `lint`, step "Realtime source policy and mutation tests"
  (`.github/workflows/qualification.yml:527-534`, the awk lines `:529-530`). #1044 removed this
  kind of duplication elsewhere and pinned the one owner that remains (`DEDUPLICATED_OWNERS`,
  `scripts/check-ci-path-routing.py:741-753`; its self-test iterates the table at
  `scripts/test-ci-path-routing.py:754-756`). Nothing pins the tool's `audit-native` step, so a
  later edit could delete it with every job green.
- **Docs and comments name the script as the gate:**
  - docs: `docs/REALTIME_DEPENDENCY_POLICY.md:52`, `:84`, `:120` and `:132`, and
    `docs/REALTIME_MEMORY.md:83`;
  - Rust comments: `crates/capi/tests/resource_lifecycle.rs:2988`,
    `crates/engine/src/realtime/observe.rs:18`, `crates/host-core/src/lib.rs:75`,
    `crates/lane/src/fpenv.rs:63`, `crates/lane/src/softfma.rs:16` and `:29`, and
    `tools/bench-support/src/lib.rs:12`;
  - a script comment: `scripts/check-bench-policy.sh:210`.

  Every line above is the same on `codex/d15-stream-b` at `b8392df66` (stream B batch 1). Batch 1
  adds one more: `crates/engine/src/realtime/watermark.rs:31-32` ("this module is inside the
  realtime root that `scripts/check-realtime-policy.sh` holds to its approved-unsafe list"), on
  `b8392df66` and on the merge tree. Other slices edit these files; find each line by its text at
  implementation time.
- **Open specs name the script, with or without `bash`.**
  - 84 top-level specs (`.github/ISSUE_SPECS/[0-9]*-*.md`, not `BRIEFS/`) on `main` name one of the
    two scripts. This count leaves out #1302 and #1418. Two files under `BRIEFS/` (`022-..` and
    `036-..`) also name them; the README keeps briefs as history (D5).
  - 63 of them use the `bash scripts/...` command form: 64 times for the check script and 15 times
    for the self-test.
  - The rest name the bare path. The second review counted the mentions by section (then over 84
    specs): Objective gates 77, Standing rules 15, Attempt and Evidence records 6, Context 3 (for
    example #1312:54 and `c45f48691:.github/ISSUE_SPECS/1343-let-the-control-thread-withdraw-an-unadopted-candidate-plan.md:28`, which name `check-realtime-policy.sh:29` as the unsafe
    authority), ABI rules to implement literally 2, Findings index 2, and one each in "Gates every
    slice inherits", "Exact ordered gates", "Allowed paths and representative gates", "Decisions
    frozen for this slice", "Authority and baseline", "How to use this issue" and "Product outcome"
    (#948).
  - After deletion, the instructions among them name a gate that does not exist.
- **GitHub bodies.** STREAMS says each spec "equals its GitHub body", so every edited spec needs
  its GitHub body refreshed (C1's script).

## Decisions

- **D1. Delete both scripts.**
  - Remove `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh`.
  - Keep `scripts/lib/gate.sh`. 12 other shell scripts source it, and 5 more copy or link it.
- **D2. CI.**
  - In `lint`, step "Realtime source policy and mutation tests" (`:527-534`), delete the two
    script lines. Rename the step "Realtime audit and artifact evidence leak gates". The step keeps
    its four leak-gate lines.
  - The tool keeps running in `audit-native` (A1-D12). Its tests keep running in `test-debug-a`.
  - **Pin the one owner.** Add `"audit-native": ("./target/release/realtime_policy",)` to
    `DEDUPLICATED_OWNERS` in `scripts/check-ci-path-routing.py`, with a comment line in the
    table's header that names the claim (the realtime source policy, whose awk copy this issue
    removed). `check_qualification_deduplicated_owners` (`:774`) then requires `audit-native` to
    run on exactly the full route and to run the binary unconditionally. If `audit-native`'s `if:`
    is not the full-route condition, stop and report.
  - `scripts/test-ci-path-routing.py`'s owner loop (`:754-756`) builds a mutation for every table
    entry. Confirm that the new entry's mutation (the step deleted, or conditioned) is red, and add
    an explicit case if the loop does not cover it.
  - `scripts/check-script-reachability.py` checks only files that still exist under `scripts/`.
- **D3. No wrapper script.** No `scripts/check-realtime-policy.sh` stays behind as a shim that runs
  the tool. AGENTS.md has no rule against wrappers, so this decision rests on the owner's
  principle of no interim shortcuts:
  - A wrapper would exist only so that stale spec text keeps working. That is the shortcut, and D5
    fixes the text instead.
  - A `bash scripts/...` name that runs `cargo run` hides a build. `lint`'s comment at
    `qualification.yml:507-514` records that the repository removed such hidden builds. A1 already
    runs the release binary in `audit-native` for the same reason.
  - The self-test has no wrapper form. Its replacement is `cargo test`. One package spelling,
    `-p realtime-policy`, serves both commands.
- **D4. Prose.**
  - Each docs and comment line in the Problem now names `tools/realtime-policy` (the gate) in
    place of the script. Do not change the rest of these sentences, except where the sentence is
    already false.
  - `observe.rs:18`'s "an approved-unsafe list of exactly two files" becomes "its approved-unsafe
    list".
  - `watermark.rs:31-32` names `tools/realtime-policy` in place of the script, in its two lines.
  - **Each edited Rust comment keeps its line count**, so no line of a shipped crate moves (gate 5).
    Reflow within the comment's lines; if a sentence cannot fit, shorten it.
  - Historical records stay as they are: verdicts, handoffs, `crates/graph/tests/MUTATIONS.md`, and
    the patches under `docs/handoffs/`.
- **D5. Open specs, by role.**
  - **Scope.** Every spec under `.github/ISSUE_SPECS/` on `main` at implementation time whose
    issue is open. In each, a mention is in scope by its role, not its section:
    - an instruction to run or pass the gate (a gate list, a standing rule, an ordered gate step,
      an inherited gate); or
    - an authority the implementer must follow (an allowlist, a floor or a pattern list that the
      spec cites as binding, wherever it sits: Context, ABI rules, Authority and baseline,
      Decisions).
  - **Leave as history:** attempt records, evidence records, verdict text, findings indexes, and
    the specs of #1302 (closed, or delivered by its attempt 3; it has no amendment), #1418 and
    #1426 (their amendments state their own gates). A closed issue's spec (#948 after A2) is not
    edited. The nine tool specs of this batch (C1, A1, A2, B1a, B1b, B2a, this one, B2b-1 and
    B2b-2) are not edited either: they are still open when this commit is made, and their gates
    name the awk gate on purpose, for the commits before this one. Nothing under
    `.github/ISSUE_SPECS/BRIEFS/` is edited.
  - **Replace every in-scope mention, with or without `bash`:**
    - `bash scripts/check-realtime-policy.sh` becomes
      `cargo run --locked --release -q -p realtime-policy`;
    - `bash scripts/test-realtime-policy.sh` becomes `cargo test --locked -p realtime-policy`;
    - a bare `scripts/check-realtime-policy.sh` (or `check-realtime-policy.sh`) used as a gate or
      an authority becomes `tools/realtime-policy`;
    - a line anchor into the script (for example `check-realtime-policy.sh:29`, the allowlist)
      becomes the tool's matching item, named in prose: its unsafe allowlist, its floors, its
      forbidden patterns or its control-side allowlist.
  - **The command form is a mechanical substitution** (63 specs). Do it with one scripted pass and
    review its diff; spend the judgment on the bare-path mentions only.
  - **Outside gate sections** (Context, ABI rules, Authority and baseline, Decisions and the
    like), root's ruling (2026-10-05) limits the edit to mentions of the script path: the spelled
    path `scripts/check-realtime-policy.sh` or `check-realtime-policy.sh` (with or without a line
    anchor) or the self-test's path. The sentence around the path is not rewritten, except that a
    line anchor becomes the tool item it pointed at.
  - **List every mention left unchanged** in the Evidence, with its reason.
- **D6. GitHub sync.** After the batch push puts this commit on `main`, run C1's
  `bash scripts/operator/sync-spec-bodies.sh <base>..<batch tip>` once, then with `--check`. Both
  outputs go in the Evidence. If C1 is not on `main`, this slice waits for it.
- **D7. STREAMS.** Root names C2's cross-stream edits as D15-0 exceptions (STREAMS).

### Root Amendment 1 (2026-10-11): gate 5 and `.llvmbc`

Authority: root decision under the owner's delegation, after the #1441 attempt-1 verdict (SP-1) and
an adversarial review (ACCEPT WITH CHANGES). Root adopted the review's text verbatim. Gate 5 carries
the same text as #1441 gate 6, because D4 edits comments in shipped crates: any byte change to a
source file changes the `.llvmbc` section of the `libcapi.a` members and nothing else. Marked
"(root Amendment 1)" in place.

## Authorized paths

- `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh` (deleted). Stream J
  owns these after A1's STREAMS row.
- `.github/workflows/qualification.yml` (D2's `lint` step only). A cross-stream exception: H #1334
  edits the workflows, and J #1422, J #1429 and J #1435 edit other steps of this file.
- `scripts/check-ci-path-routing.py` (`DEDUPLICATED_OWNERS` and its header comment only) and
  `scripts/test-ci-path-routing.py` (D2's case, if needed). #1434 also edits the routing checker's
  tables; the later slice rebases.
- `docs/REALTIME_DEPENDENCY_POLICY.md` and `docs/REALTIME_MEMORY.md` (D4's lines only). No stream
  owns them; J takes them by this listing.
- Comment lines only (D4), each named in STREAMS:
  - `crates/engine/src/realtime/observe.rs` and `crates/engine/src/realtime/watermark.rs` (stream
    B owns `crates/engine/src/realtime`);
  - `crates/lane/src/fpenv.rs` and `crates/lane/src/softfma.rs` (stream G owns `crates/lane`);
  - `crates/capi/tests/resource_lifecycle.rs` (stream F owns `crates/capi` tests; stream B owns
    `crates/capi`);
  - `crates/host-core/src/lib.rs` and `tools/bench-support/src/lib.rs` (no stream owns them; J
    takes them by this listing).
- `scripts/check-bench-policy.sh` (the comment at `:210` only; J by this listing).
- `.github/ISSUE_SPECS/*.md` (D5's substitutions only). S0 owns these specs, and the edit touches
  specs of every stream.
- This spec

## Non-goals

- Any change to the tool's rules. B2b-1, B2b-2, #1418 and #1426 change the tool after this
  slice.
- Historical records (D4, D5).
- The sync script itself (C1).

## Hazards

- **Specs that are not yet on `main`.** Specs on other streams' branches still name the old
  commands. The root agent's issue-boundary audit (AGENTS.md, Delivery-control rules) fixes them
  when they land. A rebase conflict in a spec is resolved by applying D5 again.
- **Hot files.** Comment lines in `crates/engine/src/realtime/*`, `crates/lane/*` and
  `crates/capi/*` sit in files that streams B, F, G and others edit. Find each line by its text,
  not by its number.
- **Re-measure.** Each later slice re-measures the floors (the realtime-policy floors row in
  STREAMS). After this slice the floors live only in `Policy::workspace()`.
- **Every later pop or region edits `Policy::workspace()`** (B2b-1, Hazards): the STREAMS standing
  exception covers it.
- **Threat model (root Amendment 2, 2026-10-11).** Deleting the awk gate leaves this tool as the only
  check. Its threat model, with its closed list of limits, is the "Threat model (root ruling
  2026-10-11)" bullet in #1438 Hazards. A finding inside a listed limit is recorded, not a FAIL, and
  only a root ruling adds or widens a limit.

## Objective gates

1. `cargo run --locked --release -q -p realtime-policy` prints the same line as B2a's gate 1, on
   the same tree.
2. `cargo test --locked -p realtime-policy` passes.
3. **Neither deleted script is named as a gate (PR evidence; not a committed test).**
   `git grep -n -e 'check-realtime-policy' -e 'test-realtime-policy'` lists only the mentions that
   D4 and D5 leave as they are. Each one is in the Evidence list with its role and reason.
4. These all exit 0:
   - `python3 -B scripts/check-ci-path-routing.py` and `python3 -B scripts/test-ci-path-routing.py`
     (with D2's owner mutation shown red);
   - `python3 -B scripts/check-script-reachability.py` and
     `python3 -B scripts/test-script-reachability.py`;
   - `bash scripts/check-workspace-policy.sh`;
   - `cargo fmt --all -- --check`.
5. **Shipped artifacts unchanged.** Every edited comment keeps its line count. The
   `artifact-identity` job reports the worklet module UNCHANGED. PR evidence, built as A1's gate 6
   builds them (parent twice first, then the change, in one checkout):
   - the sha256 of `libcapi.so` is identical;
   - `libcapi.a` holds the same members in the same order;
   - each member is byte-identical after `objcopy --remove-section=.llvmbc`.

   `.llvmbc` is rustc's embedded bitcode. It carries each source file's checksum, so any byte
   change to a source file changes it, and no linker copies it into a shipped image. Any other
   difference fails. (root Amendment 1, SP-1)
6. **CI.** The batch's `qualification` run shows `audit-native` running the tool, and `lint`
   running no awk gate.
7. **GitHub.** After the batch push, C1's script exits 0 on `<base>..<batch tip>`, and its
   `--check` then exits 0 with no `MISMATCH` line.

*Test value.*
- D2's owner entry is red (in the routing checker) if a later edit deletes or conditions the tool's
  CI step, which today would leave every job green.
- This slice deletes the self-test that slices A1 to B1b superseded: AGENTS.md says a change that
  supersedes a test deletes it in the same PR. Every case of that self-test is now one of these:
  - a committed case of the tool (A1-D10, A2-D4, B1a-D10, B1b-D8);
  - a case retired with root's acceptance (A1-D10).

## Evidence

- Gates 1-7 output.
- Gate 3's list of remaining mentions, each with its role and reason.
- D6's sync output and `--check` output.

## Dependencies

- After (same stream): B2a (#1442), #1519 and C1. If any of them spends its budget without PASS, C2 does not land in this batch.
- Before (same stream, in the same batch): the commit "H #1448's guard, landed by J after C2"
  (B2b-1's first commit, spec in its "#1448 guard" section), then the rest of B2b-1, then B2b-2. #1418 (Amendment 1) and #1426 (Amendment 1) come after the
  whole batch is on `main`.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day. The command-form edits are one scripted pass (D5); the judgment is in about
  21 specs that name the bare path.

## Attempt 1 record

Base `b45d4836c` (`codex/d15-stream-j3`). Everything below was measured on this branch.

**Re-measured, differing from the Problem.** The Problem's line numbers moved: `qualification.yml`
step at `:527-534` (awk lines `:529-530`) is the same; `resource_lifecycle.rs` comment is at `:3022`
(not `:2988`); `REALTIME_DEPENDENCY_POLICY.md` mentions were at `:55`, `:81`, `:150-153`, `:368-370`
(not `:52`, `:84`, `:120`, `:132`); `REALTIME_MEMORY.md:83`. Real tree: 94 marked regions in 26 files
(`git grep -c '^\s*// REALTIME_POLICY_BEGIN'`), equal to `Policy::workspace()`'s floors. Open specs:
80 numbered specs name a script (gh open set read once); 67 were edited, 13 mention lines were left
(below). `.github/ISSUE_SPECS/` also holds 58 specs of closed issues, 180 mentions, none edited.

**What changed**
- Deleted `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh` (D1). `scripts/lib/gate.sh` kept.
- `qualification.yml`, `lint`: deleted the two awk lines, step renamed "Realtime audit and artifact
  evidence leak gates" (four leak-gate lines kept). `audit-native` (`if:` is the full-route condition)
  still runs `./target/release/realtime_policy`; the tool's tests run in `test-debug-a`'s workspace test.
- `scripts/check-ci-path-routing.py`: `"audit-native": ("./target/release/realtime_policy",)` in
  `DEDUPLICATED_OWNERS`, with a header comment line naming the claim.
- `scripts/test-ci-path-routing.py`: the owner loop covers the deletion mutation of the new entry; two
  explicit cases added: the tool's step given a step-level `if:`, and `audit-native` moved off the full route.
- Comment-only edits (each keeps its line count; `git diff --numstat` is 1/1 or 2/2 per file):
  `crates/engine/src/realtime/observe.rs:18` ("an approved-unsafe list of exactly two files" becomes "its
  approved-unsafe list"), `crates/engine/src/realtime/watermark.rs:32`, `crates/lane/src/fpenv.rs:63`,
  `crates/lane/src/softfma.rs:16` and `:30`, `crates/capi/tests/resource_lifecycle.rs:3022`,
  `crates/host-core/src/lib.rs:75`, `tools/bench-support/src/lib.rs:12`, `scripts/check-bench-policy.sh:210`.
- Docs: `REALTIME_DEPENDENCY_POLICY.md` six lines (the allowlist authority now says `tools/realtime-policy`
  (`UNSAFE_ALLOWLIST` in `src/policy.rs`) in place of the false "`unsafe source exclusions` line"; the
  mutation suite is now `cargo test -p realtime-policy`, which has the third-lane-file and `other.rs` cases
  in `tests/policy/unsafe_ownership.rs`); `REALTIME_MEMORY.md:83`.
- Specs (D5): one scripted pass for the command form (`bash scripts/check-realtime-policy.sh` to
  `cargo run --locked --release -q -p realtime-policy`, `bash scripts/test-realtime-policy.sh` to
  `cargo test --locked -p realtime-policy`), bare path to `tools/realtime-policy`; the diff was reviewed
  line by line. Hand edits: #1447's line anchor `:208-228` became "the count form of the drain-bound rule
  of `tools/realtime-policy`"; #1482's "(or the tool)" was dropped with the script name.

**Mentions left unchanged in open specs (D5 history or time-bound), with reason**
- #1345:219, :228, :229, :233, :234, :238 (D9 and its "added" paths, gate 9 and gate 10): they authorise
  and gate edits to the awk scripts for stream B3's commits that land before this one, and say "Stream J's
  tool is not on `main` yet". Same reason as the nine tool specs. Root: after the batch merges, #1345's
  gate 9 has no awk script to run. Its D9 floors move to `Policy::workspace()`.
- #1447:27: a measurement the awk gate gave on `origin/main` (Problem, history).
- #1422:290: evidence record of its `plan.rs` pair.
- #349:205 and :315: finding rows (`RT-16`, `IO-14`) with line anchors into the old script (findings index).
- #881:122, :179, :398: attempt 1 and attempt 2 evidence records.
- #948:5, :16, :31: #948 closes with A2 (D5).
- #1443:37, #1444:27, the nine tool specs, #1519 (6) and #1418, #1426: D5 lists them; #1519 is the tenth
  tool slice and its mentions are its own attempt records and gates for commits before this one.
- `crates/graph/tests/MUTATIONS.md` (12) and `docs/handoffs/**`, `BRIEFS/` (3): history (D4, D5).
- Specs of closed issues (58 files, 180 mentions): not edited.

**Gates**
1. `cargo run --locked --release -q -p realtime-policy` exits 1 with exactly one finding:
   `marked realtime unbounded try_pop drain ... crates/effect-contract/src/live.rs:366` (pre-#1345;
   stream B3's #1345 rewrites that function; J rebases onto it before the batch gate). No exception added,
   `live.rs` not touched. Nothing else is refused.
2. `cargo test --locked -p realtime-policy`: 186 passed, 0 failed.
3. `git grep` for either script name lists only the mentions above, plus none in code, workflows, scripts or
   docs outside `docs/handoffs/` and `MUTATIONS.md`.
4. exit 0: `check-ci-path-routing.py`, `test-ci-path-routing.py`, `check-script-reachability.py` (118
   files reached), `test-script-reachability.py` (20 cases), `check-workspace-policy.sh`,
   `check-bench-policy.sh`, `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy
   --all-targets -- -D warnings`; `cargo check` of engine, lane, host-core, bench-support and `capi --tests`.
   *Mutation:* with the new `DEDUPLICATED_OWNERS` entry removed, `test-ci-path-routing.py` exits 1
   (`#1044 mutation was accepted ... Realtime source policy (tools/realtime-policy)`); restored, it exits 0.
   Test value: the new cases defend the tool's `audit-native` step and job `if:` being edited off the full
   route or conditioned, which no other table entry covers.
5. Shipped artifacts: `libcapi.so` sha256 `0213cd820e2715de...` at the parent (two builds) and at the
   change. `libcapi.a`: parent `2aa32d808165aef3...` twice (reproducible), change `fd2c8d49063a2f2c...`; 387
   members, same names and order; 5 members differ raw and 0 differ after
   `objcopy --remove-section=.llvmbc`. The worklet module, built with
   `scripts/build-web-audioworklet.sh --module-only`, is `9784d3509dba6bfd...` at the parent and at the change.
   (The `artifact-identity` job is CI only.)
6, 7. CI and GitHub: after the batch push (C1's script and its `--check`).
