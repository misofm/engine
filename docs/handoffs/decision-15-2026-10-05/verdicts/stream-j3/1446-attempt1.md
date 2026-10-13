PASS

# #1446 attempt 1: adversarial verdict

Verifier: opus (adversarial). Commit `df9dd6b75` (parent `b45d4836c`), branch `codex/d15-stream-j3`.
The commit was exported to `/tmp/claude-1002/v1446/tree`, with `CARGO_TARGET_DIR=/tmp/claude-1002/v1446/target`.
The worktree was not touched.

Result: no BLOCKER and no MAJOR. Two MINOR and three NIT findings, plus notes for root.

## Findings

### MINOR-1: a sentence about history in `docs/REALTIME_DEPENDENCY_POLICY.md:81` is now false

The line now says: "(`crates/engine/src/arch` was a third owner until #84 phase A deleted it ... the exemption was
removed from `tools/realtime-policy` in the same change.)"

This is false. #84 phase A removed the exemption from `scripts/check-realtime-policy.sh`, and the tool did not exist
at that time.

Spec problem: D4 itself listed this line (Problem `:52` on `6d28a80ec`), so the implementer did what the spec said.
The sentence is history and should name the gate as it was at that time. Suggested wording: "removed from the
realtime policy gate (then `scripts/check-realtime-policy.sh`) in the same change". Fix this at the J rebase or in a
follow-up. The line is not code.

### MINOR-2: D5 edited the gates of specs for stream B3 issues that land before the J batch

The commit changed the gates of #1345 (gate 7), #1346, #1347 and #1447 (D4, gate 3, gate 6). These issues are
delivered on `codex/d15-stream-b3`, which uses the awk gate. The J batch boundary comes after #1447 and #1345 are
on `main` (STREAMS).

- #1447 gate 3 now reads: "`cargo run --locked --release -q -p realtime-policy` prints `realtime policy: ok (..)`.
  Stream J's tool is not on `main` while this issue runs". This contradicts itself, and nobody can run it on
  #1447's tree.
- The implementer left #1345's D9 and gates 9-10 unchanged for exactly this reason ("for stream B3's commits that
  land before this one"). The same reason applies to #1345 gate 7 and to #1447, but the implementer did not apply
  it there.

D5's literal scope ("open on `main` at implementation time") includes these specs, so this is mainly a spec problem.
See the spec problems for root, below.

### NIT-1: two edited comments still make claims that were already false

- `tools/bench-support/src/lib.rs:12`: "`tools/realtime-policy` names `src/alloc.rs` as the single file under
  `tools/` allowed to contain `allow(unsafe_code)`". This is false: `UNSAFE_ALLOWLIST` has three `tools/` files
  (`bench-support/src/alloc.rs`, `wasm-gates/tests/g6_full_corpus_ftz.rs` and `audit/src/capi.rs`), and the tool
  checks `unsafe` code, not the attribute.
- `scripts/check-bench-policy.sh:210`: "holds the matching list for `crates/` and `hosts/`". The list also covers
  `tools/`.

The awk gate had the same list, so both claims were false before this commit. D4 allowed the implementer to fix
them but did not require it.

### NIT-2 (existing gap, for root): the `#1044` owner check accepts `continue-on-error`

Measured on a copy of the tree: `check_qualification_deduplicated_owners` refuses each of these mutations:

- the step deleted;
- the step given a step-level `if:`;
- `|| true` added to the command;
- the job moved off the full route.

It accepts `continue-on-error: true` on the tool's step or on the `audit-native` job. Either one makes a failure
look like success while every job stays green. This gap applies to every `DEDUPLICATED_OWNERS` entry and is not
caused by this slice. I recommend a follow-up issue.

### NIT-3 (no regression, for root): no check pins where the tool's tests run

The tool's test suite replaces the deleted self-test. It runs only in `test-debug-a`'s
`cargo test --workspace --all-targets`. No check pins it, so a later `--exclude realtime-policy` would remove it
with every job green. The awk self-test in `lint` was not pinned either, so this is not a regression. D2 asked only
for the binary to be pinned.

## What was verified

### Coverage of the deleted awk gate

I read `check-realtime-policy.sh` and `test-realtime-policy.sh` from git at `b45d4836c`. A delegated mapping pass,
which I reviewed, looked at every case of the self-test.

- **Unsafe allowlist.** The 19 paths are the same and in the same order (`Policy::workspace()`).
- **Floors.** 26 files and 94 regions are unchanged.
- **Forbidden patterns.** All 36 awk patterns are there, plus `.collect::<` and the `async` keyword token.
- **Other checks.** Each of these maps to tool source:
  - the realtime-module directory check;
  - BEGIN/END pairing;
  - every drain-bound sub-rule;
  - the `ffi.rs` whole-plan rule, including the missing-file check and the "no production render entry" check.
- **Self-test cases.** Every case maps to a tool test, or to a retirement that root accepted:
  - the 9 forbidden-body cases;
  - 16 rows for the 17 `unsafe-*` cases (one exact duplicate was dropped, 1438:573-575);
  - all 56 `mutate_*` drain cases, in `drain/ported.rs`, with text byte-identical except for one re-indent;
  - the floor, unmatched-marker and whole-plan cases;
  - the 22 tool-failure injections and 5 counter-mutants, which A1-D10 retired (1438:287-289, :583).

  The class changes (an open tail is now misordered, a region that ends in a comment is now malformed) are recorded
  in 1438 and 1440.
- **Gaps.** None.
- **Unrecorded narrowings.** All are benign:
  - an identifier that ends in `unsafe` or `async` no longer matches;
  - `&*plan` in a trailing comment or a literal in `ffi.rs` is no longer a finding. This belongs to #1439's D3 and
    is not recorded there.

### References to the deleted scripts

`git grep` for both script names at `df9dd6b75` finds nothing in code, workflows, scripts or docs, except under
`docs/handoffs/`, in `crates/graph/tests/MUTATIONS.md` and under `BRIEFS/`. Those are history (D4, D5).

The mentions left in open specs match the Attempt 1 record exactly, and each one is justified:

- #1345 D9 and gates 9-10: they authorise B3's edits to the awk floors.
- #1447:27: a measurement in the Problem.
- #1422:290: an evidence row.
- #349:205 and :315: rows of the findings index.
- #881: three evidence records.
- #948: it closes with A2's D6 at the batch push. It is still OPEN on GitHub, which is correct for now. Root must
  confirm that it closes at the sync, or an open spec will name a deleted gate.
- #1443:37, #1444:27, the nine tool specs, #1519's own records, #1418 and #1426: D5 lists them.

### D5 substitution

I checked the scripted substitution against the parent and the live GitHub state:

- **Counts.** 67 specs were edited, and all 67 are OPEN on GitHub. 80 open numbered specs name a script at the
  parent, so 13 were left unchanged.
- **Kinds of change.** The word-level diff holds only these: the command form, the bare path to
  `tools/realtime-policy`, and one hand rewrite of a line anchor (#1447:122).
- **Sections.** Of the edited hunks, 62 are in Objective gates and 15 in Standing rules. One each is in Decisions
  (two in #1447), How to use, Evidence requirements, Authority and baseline, and Allowed paths. All are
  instructions or authorities.
- **History.** The commit edits no attempt record, evidence record or closed spec. The 58 closed specs are untouched.

### Comment-only edits

These files have comment-only changes, 1/1 lines each (2/2 in `softfma.rs`), so no line count changed:

- `observe.rs`, `watermark.rs`;
- `fpenv.rs`, `softfma.rs`;
- `resource_lifecycle.rs`;
- `host-core/src/lib.rs`, `bench-support/src/lib.rs`;
- `check-bench-policy.sh`.

`codex/d15-stream-b3` currently changes none of these lines.

### Documentation lines

- `REALTIME_DEPENDENCY_POLICY.md:55-56`: the `UNSAFE_ALLOWLIST` pointer is true. It is the const in
  `Policy::workspace()`, `src/policy.rs`.
- `:150-153` and `:368-370`: true. `tests/policy/unsafe_ownership.rs` has `crates/lane/src/kernels.rs`,
  `fpenv_extra.rs`, `tools/audit/src/other.rs` and the retired paths of `native-pcm-runner`, `bench-protocol` and
  `engine/src/arch`.
- `REALTIME_MEMORY.md:83`: true. `.expect(` is a forbidden pattern.
- The exceptions are MINOR-1 and NIT-1.

### CI and root's standing ruling

- The tool runs as `./target/release/realtime_policy` in `audit-native` (`qualification.yml:766-767`).
- `audit-native` has `if: needs.route.outputs.route == 'full'`, which is the required workflow.
- `qualification.yml` has no `paths` filter on any trigger.
- The verdict job checks `audit-native` against `$full_expected` (`:1190`).
- No other workflow (`nightly.yml`, `fuzz.yml`) runs the tool.
- `lint`'s step is renamed and keeps its four leak-gate lines.

## Test value

The commit adds two explicit cases to `scripts/test-ci-path-routing.py`, and adds `audit-native` to the
`DEDUPLICATED_OWNERS` loop. Together they defend one thing: removing the `audit-native` entry from
`DEDUPLICATED_OWNERS`, or editing the tool's step or job off the full route.

Measured on a copy of the tree:

- With the entry removed, the checker accepts all three workflow mutations (step deleted, step `if:`, job `if:`
  off the full route), and the self-test exits 1 ("#1044 mutation was accepted ... Realtime source policy").
- With the entry restored, the checker refuses all three, and the self-test exits 0.

The loop alone cannot catch the entry's removal, because it iterates the table. The explicit cases are therefore
what pin the entry. No other existing check catches the job `if:` mutation.

## Gates run (on the export)

1. **The tool on the tree.**
   - `cargo run --locked --release -q -p realtime-policy` exits 1 with exactly one finding: the marked unbounded
     `try_pop` drain at `crates/effect-contract/src/live.rs:366`. This is expected before #1345; root ruled that J
     rebases onto #1345 before the batch gate, and no exception was added.
   - The prebuilt binary, which is `audit-native`'s step, gives the same output.
   - Regions: 94 in 26 files (`git grep` outside the tool's fixtures). This equals the floors, and the tool reports
     no floor finding.
2. **Tool tests and lint.**
   - `cargo test --locked -p realtime-policy`: 186 passed.
   - `cargo clippy --locked -p realtime-policy --all-targets -D warnings`: clean.
3. **Remaining mentions.** `git grep` of the remaining mentions gives the list above.
4. **Every x86_64 step of the `lint` job** passed on the export (the export was turned into a git repository so that
   the scripts that need git work):
   - `cargo fmt --check`;
   - workspace `clippy` with all features, `-D warnings`;
   - `cargo doc` with `-D warnings`;
   - the workspace, session, env, bench, test-support-ci, scalar-oracle, reachability and routing policy scripts
     and their self-tests (`check-ci-path-routing.py` and `test-ci-path-routing.py` included);
   - the console fixture, builtins fixtures, bench-pre, host-core, protocol, leak, trace, lane, unfused, rack,
     builtins, graph, effect-runtime, conformance, peq, release-shape, npm, stem-store and stem-identity steps;
   - the three x86-64 lane-guard probes.
5. **Artifacts.** I built in one checkout path, with fresh mtimes to force rebuilds: the change first, then the
   parent.
   - My first build had a stale-fingerprint problem. Cargo saw the commit-time mtimes as fresh, so the change did
     not rebuild. I discarded that build and built again.
   - `libcapi.so`: sha256 `cc13af25...` at the parent and at the change. The digests are local to this path.
   - `libcapi.a`: parent `8f118f2a...`. This is reproducible: the same digest came from an earlier build and from
     the forced rebuild. Change: `64f96707...`.
   - The archive has the same 387 members in the same order. 5 members differ (builtins, capi, engine, host_core,
     lane), and 0 differ after `objcopy --remove-section=.llvmbc`.
   - The worklet module (`build-web-audioworklet.sh --module-only`) is
     `9784d3509dba6bfd8188b00fc48f9ae4f20020ef9583cbc409cea1d079878d9c` at the parent and at the change.
6. **CI and GitHub sync (gates 6 and 7).** These run only after the batch push, so they were not verified here.

## Spec problems for root

1. **D5's scope does not handle specs whose issues land before the J batch (MINOR-2).** At the J rebase onto
   B3's merge, apply D5 again against the specs that are open at that time:
   - drop the edits to the specs of issues that B3 closes (#1345, #1346, #1347, #1447 if they close);
   - remove the contradiction in #1447 gate 3 if that spec is still open.

   Also, #1345 D9 and gates 9-10 tell B3 to raise the awk floors. After the rebase, those floor changes belong in
   `Policy::workspace()`, as the Hazards say.
2. **D4 listed a history sentence (MINOR-1).** Rule on the wording.
3. **NIT-2 (the `continue-on-error` gap in the `#1044` owner check) and NIT-3 (no check pins the tool's tests)**
   are candidates for a follow-up issue.
