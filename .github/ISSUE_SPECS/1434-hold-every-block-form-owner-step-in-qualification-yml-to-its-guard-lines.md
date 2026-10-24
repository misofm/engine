# Hold every block-form owner step in qualification.yml to its guard lines

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0). Found by
the attempt-1 verdict of *Fail the web AudioWorklet test step when it leaves anything in its
temporary directory* (#1429, `/home/bl/misofm/submix-verdicts/1429-attempt1.md`, NIT); root filed
it. CI tooling only; no engine code changes.

## Problem (verified on `codex/d15-stream-j2` at `2944fc63f`, which contains #1429 and its follow-ups)

- **The owner lint checks the owner command, not the lines that make its step a guard.**
  `scripts/check-ci-path-routing.py` `check_qualification_deduplicated_owners` (`:774-787`) requires
  each command in `DEDUPLICATED_OWNERS` (`:741-753`) among the job's unconditional step commands
  (`unconditional_step_commands`, `:916-917`, over `step_commands`, `:892-913`, which splits a
  `run: |` block into its lines). A line in the same block that is not an owner command is never
  checked.
- **#1429's guard is such a block.** `artifact-gates`' step "Hermetic browser host and worklet
  tests" (`.github/workflows/qualification.yml:351-364`) runs `bash scripts/test-web-audioworklet.sh`
  (the owner command) between `set -o pipefail`, a private `TMPDIR` and an emptiness check. Deleting
  the emptiness check (`:360-364`), the `export TMPDIR` (`:358`) or `set -o pipefail` (`:355`)
  leaves the lint green, and #1429's guarantee is gone with every job green.
- **The other block-form owners have the same gap.** `lint`'s step "Effect runtime
  dependency-boundary and fixture policy and mutation tests" (`qualification.yml:553-558`) runs its
  four owner commands as one block. A `set +e` line before them masks every failure while each
  owner command is still present verbatim, so the lint stays green. Step-level `continue-on-error`
  is refused for some jobs (`:577`, `:1042`), not for the owner steps.

## Decisions

- **D1. Each block-form owner step is named and holds its guard lines.** Extend
  `DEDUPLICATED_OWNERS` (or a sibling table beside it) so each owner whose step is a `run: |` block
  names the step (by its `name:`) and the lines that must stay in that block, in order, besides the
  owner commands: for `artifact-gates`' step, `set -o pipefail`, the `mktemp -d` line, the `trap`
  line, `export TMPDIR="$scratch"`, the owner command, the `find` line and the `if [[ -n
  "$leftover" ]]` check with its `exit 1`. A missing, reordered or edited guard line fails the lint
  with a message naming the line and the issue that owns it (#1429, #1044).
- **D2. No owner step may mask a failure.** In every owner step (block or single line): no
  step-level `if:` (as today), no step-level `continue-on-error`, and no line that disables or
  swallows failure: `set +e`, `set +o errexit`, `set +o pipefail`, `|| true`, `|| :`, a trailing
  `&`, or `exit 0` before an owner command. The lint refuses each by name.
- **D3. Self-test cases.** In `scripts/test-ci-path-routing.py`, one mutation case per D1 guard
  line of #1429's step (delete it: red), one that moves the emptiness check before the owner
  command (red), and one per D2 construct on an owner step (red). The existing #1044 cases stay.

## Authorized paths

- `scripts/check-ci-path-routing.py`
- `scripts/test-ci-path-routing.py`
- This spec

## Non-goals

- Changing any workflow step. If D1 or D2 finds an owner step that masks a failure today, stop and
  report it.
- Owners outside `qualification.yml`.

## Hazards

- `.github/workflows/qualification.yml` is a hot file; this issue reads it and does not edit it. A
  slice that edits an owner step after this lands must update the D1 table in the same change, and
  the lint says so in its message.
- Pinning whole step bodies byte for byte would be ceremony (AGENTS.md); D1 pins only the lines
  that carry a claim, each named with its owning issue.

## Objective gates

1. `python3 -B scripts/check-ci-path-routing.py` and `python3 -B scripts/test-ci-path-routing.py`
   exit 0 on the branch's `qualification.yml`.
2. **Red on revert (PR evidence).** The new self-test cases, run against the checker at this
   issue's parent commit, are accepted (no failure); with the change, each is refused.
3. **Each D1/D2 rule has a catch (PR evidence).** Remove each rule from the checker in turn: at
   least one D3 case turns red, and no existing case already catches it. The same holds for A1-D1
   and A1-D2 (Amendment 1).
4. `bash scripts/check-workspace-policy.sh` exits 0.
5. **The doctest steps are held (Amendment 1).** Each A1-D3 case is refused by the checker with
   the change and accepted by the checker at this issue's parent commit (PR evidence, as gate 2).

*Test value.* Each D3 case is red when an edit to `qualification.yml` removes a guard line from an
owner step, or masks an owner command's failure, which today leaves every job green while the
claim the step owns is no longer checked. Each A1-D3 case is red when an edit deletes a doctest
step or narrows its packages or features below its job's whole-package step, which today leaves
every job green while the `compile_fail` fences in the dropped packages no longer run.

## Evidence

- Gates 1 to 5; the D1 table with each line's owning issue; the A1-D1 pairing table.

## Dependencies

- *Fail the web AudioWorklet test step when it leaves anything in its temporary directory*
  (#1429): D1 holds its guard lines.
- *Run doctests in CI* (#1422, on `codex/d15-stream-j2`): Amendment 1 holds the
  two doctest steps it added.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused; the checker reads the workflow, which is its input.
- Attempt budget: three attempts, one adversarial verdict each.

## Amendment 1 (root, 2026-10-05)

Source: the stream-J batch-2 verdict, MINOR-2
(`/home/bl/misofm/submix-verdicts/stream-j2-batch-verdict.md`). #1422 added two doctest steps to
`qualification.yml`. After #1422 they are the only CI check of the compile-time guarantees (#1422's
fence and twin pairs, #1416's three construction fences in `crates/host-core/src/route_controls.rs`),
and nothing holds them in place. This amendment widens the issue to them. The decisions, gates and
paths above stay; these are added.

**Problem (verified on `codex/d15-stream-j2` at `4387b935e`).**
- `test-debug-a` (`.github/workflows/qualification.yml:601`): the whole-package step "Workspace
  debug tests (excluding DSP crates, audit, and wasm evidence tools)" (`:624-634`) runs
  `cargo test --locked --workspace --all-targets` with 20 `--exclude` packages and
  `--features builtins-compiler/test-support,graph/test-support,host-web/test-support,host-core/test-support,effect-compiler/test-support,protocol/test-support,engine/realtime-audit`.
  The doctest step "Workspace doctests (same packages and features as the debug tests)"
  (`:637-647`) runs `cargo test --locked --workspace --doc` with the same `--exclude` list and
  `--features`.
- `test-debug-b` (`:649`): the whole-package step "DSP crates debug tests (lane feature unification
  pinned explicitly)" (`:668-674`) runs `cargo test --locked --all-targets` over 14 `-p` packages
  with `--features math/lane,parametric-eq/test-support,builtins/test-support,lane/test-support`.
  The doctest step "DSP crates doctests (same packages and features as the debug tests)"
  (`:676-682`) runs `cargo test --locked --doc` with the same `-p` list and `--features`.
- The verdict probed the export: deleting `test-debug-a`'s doctest step (probe E), or removing
  `host-core/test-support` from its `--features` only (probe F), leaves
  `scripts/check-ci-path-routing.py` and `scripts/check-test-support-ci.py` both at exit 0.

**Decisions.**
- **A1-D1. Each whole-package test step has a doctest twin.** The checker names the two pairs
  above (job, whole-package step name, doctest step name). In each pair, the doctest step must be
  present in the same job, unconditional (no step-level `if:`, no `continue-on-error`, and D2's
  failure-masking constructs refused as on an owner step), and its command must be `cargo test
  --locked --doc` (plus `--workspace` where its twin has it).
- **A1-D2. Same packages and features.** The doctest command's package selection (the set of
  `--exclude` values with `--workspace`, or the set of `-p` values) and its `--features` set must
  equal its twin's, compared as sets parsed from the step's `run:` lines. A missing, extra or
  changed package or feature fails the lint with a message naming the step, the difference and
  #1422.
- **A1-D3. Self-test cases** in `scripts/test-ci-path-routing.py`, each red:
  - delete `test-debug-a`'s "Workspace doctests" step;
  - delete `test-debug-b`'s "DSP crates doctests" step;
  - drop one `--exclude` value from `test-debug-a`'s doctest step only (the doctest step then runs
    a package its twin does not; the set differs);
  - drop `host-core/test-support` from `test-debug-a`'s doctest `--features` only (probe F);
  - drop one `-p` package from `test-debug-b`'s doctest step only;
  - drop `lane/test-support` from `test-debug-b`'s doctest `--features` only;
  - add a step-level `if: false` to one doctest step.
  A change that edits a twin and its doctest step together stays green (the pairing, not the
  values, is the claim).

**Non-goals (added).** The AArch64 doctest line (`scripts/run-aarch64-tests.sh:147`) is outside
`qualification.yml` and stays out, as the Non-goals above say for owners outside it. Root files it
separately if wanted.

## Amendment 2 (root, 2026-10-12)

Source: the attempt-1 verdict of *Check realtime regions with a Rust syntax-tree tool ...* (#1446,
`/home/bl/misofm/submix-verdicts/1446-attempt1.md`, NIT-2 and NIT-3). Root ruled on 2026-10-12 to fold
the drafted successor into this issue instead of filing a new one. The decisions, gates and paths
above, and Amendment 1, stay as written; these are added. CI tooling only: the checker and its
self-test, no workflow edit.

**Problem (measured on `codex/d15-stream-j3` at `e6dd440e0`, with #1446's `realtime_policy` owner
entry).** Each case below was applied to a scratch copy of `qualification.yml`; `python3 -B
scripts/check-ci-path-routing.py --root <copy>` exits 0 (accepted) for every one.
- **`continue-on-error` on an owner is not refused (NIT-2).** `check_qualification_deduplicated_owners`
  (`scripts/check-ci-path-routing.py:777-791`) over `DEDUPLICATED_OWNERS` (`:743-756`, four jobs:
  `artifact-gates`, `lint`, `test-release`, `audit-native`) refuses a deleted owner command, a job
  `if:` other than the full route and a step-level `if:`. It accepts `continue-on-error: true` (and
  `${{ true }}`) on the `audit-native` step "Realtime source policy (tools/realtime-policy)" and
  `continue-on-error: true` on the `audit-native` job. Either makes a failing owner pass with every
  job green. The same holds for every owner. `UNMASKABLE` (`:577`) already refuses job-level
  `continue-on-error` for `artifact`, the record job, `artifact-identity` and two self-test jobs, but
  not for these four.
- **Nothing pins where the tool's tests run (NIT-3).** `tools/realtime-policy` is a workspace member
  (`Cargo.toml:40`). Its tests run only in `test-debug-a`'s step "Workspace debug tests ..."
  (`qualification.yml:624-634`): `cargo test --locked --workspace --all-targets` with 20 `--exclude`
  values, none of them `realtime-policy`. No other job runs `cargo test` for it (`audit-native`
  builds the binary only). Adding `--exclude realtime-policy` there (to that step alone, or to it and
  its doctest twin) leaves the checker at exit 0, and the suite that defends the realtime source
  policy no longer runs.
- **The census of other `continue-on-error` lines.** Two exist in `qualification.yml`, at `:1209` and
  `:1218`: the best-effort telemetry steps of the `verdict` job (`if: always()`, design section 11).
  They are not owners and stay. `nightly.yml` has none.

**Relation to the items above (no rewrite).**
- *D2 (step-level `continue-on-error` on owner steps)* covers the step half of NIT-2. A2-D1 extends
  D2 to the job level, and fixes the step test as presence-based (any value), so D2's helper serves
  both. D2's other masking constructs (`set +e`, `|| true`, ...) stay D2's alone; A2 adds none.
- *Amendment 1 (A1-D2, `test-debug-a`'s two steps compared for equal packages)* treats an `--exclude`
  added to both steps as green ("the pairing, not the values, is the claim"). That is the blind spot
  of NIT-3: A2-D2 pins a value, namely that `realtime-policy` is in `test-debug-a`'s resolved
  package set. A1-D2 stays as written; the two checks run side by side and neither replaces the
  other.

**Decisions.**
- **A2-D1. Any `continue-on-error` key on an owner job or step is refused, whatever its value.** In
  `check_qualification_deduplicated_owners`, for each owner job: no `continue-on-error` key at job
  level, and none in the step whose `run:` holds an owner command. `false` is refused too: it is a
  no-op nobody needs, a `${{ ... }}` value can turn truthy later, and a presence test is the
  fail-closed rule that needs no expression parser. The message names the job, the step and #1044.
- **A2-D2. `test-debug-a` must run the realtime-policy tests.** The checker reads the commands of
  `test-debug-a` (`step_commands`, `:895-916`, which already joins `\` continuations) and requires a
  `cargo test` command that is either `--workspace` with no `--exclude realtime-policy`, or names `-p
  realtime-policy`, with `--all-targets` and none of `NON_INTEGRATION_TARGET_SELECTORS` (`:884`) or
  `--`. The doctest twin is out of scope: the tool has no doctest claim. A command that fails this
  rule fails the checker, naming #1446.

**Added gates.**
- **A2-G1. Six mutation cases**, each `workflow_mutation_fails("qualification.yml", ...)`, each
  accepted by the checker at this issue's parent commit and refused with the change:
  1. step-level `continue-on-error: true` on `audit-native`'s "Realtime source policy" step;
  2. job-level `continue-on-error: true` on `audit-native`;
  3. `continue-on-error: false` on that step (the presence rule); one loop covers the other owners;
  4. `--exclude realtime-policy` added to `test-debug-a`'s "Workspace debug tests" step alone;
  5. the same flag added to that step and its doctest step (the A1-D2 blind spot);
  6. `--workspace` replaced by `-p engine` on that step (`realtime-policy` not in the resolved set).
- **A2-G2. Each rule has a catch (PR evidence).** Remove A2-D1 and A2-D2 from the checker in turn: at
  least one A2-G1 case turns red each time, and no existing case already catches it.
- **A2-G3.** `python3 -B scripts/check-script-reachability.py` exits 0 (with gate 4's
  `check-workspace-policy.sh`).

*Test value (Amendment 2).* Cases 1 to 3 are red when an edit makes a failing owner step or job
report success while every job stays green, which no existing case catches (D2 refuses only the
step-level form on the steps it names, and `UNMASKABLE` omits these four jobs). Cases 4 to 6 are red
when an edit drops `realtime-policy`'s own suite from CI, which today leaves the checker at exit 0;
case 5 is the one A1-D2 passes by design.

**Evidence (added).** For each A2-G1 case, the exit status at the parent commit and with the change;
the `continue-on-error` census above, re-run at the PR head.

**Dependencies (added).** The `realtime_policy` owner entry in `DEDUPLICATED_OWNERS` and the
`tools/realtime-policy` crate land with the J3 batch (#1446). This issue therefore lands after the J3
batch is on `main`, and the re-measurement of the line numbers above is part of the implementation.

## Decision record (implementation, attempt 1, 2026-10-24)

Root accepted the scope note's recommendations; anchors re-measured at `7255ebc37`.
- D1 matches the pinned lines as an ordered subsequence of the step's comment-stripped `run:` lines
  (`GUARDED_OWNER_STEPS`), not whole-body equality: the `printf` message and `fi` carry no claim.
  Pinned (artifact-gates, "Hermetic browser host and worklet tests"): `set -o pipefail`, the `mktemp -d`
  line, the `trap` line, `export TMPDIR="$scratch"`, `exit 1` and the `find` and `if [[ -n "$leftover" ]]`
  lines (#1429), with the owner command (#1044) between them.
- D2 scans comment-stripped lines (`FAILURE_MASKS`). It also refuses, fail-closed, a line starting
  with `}` or `)` then `||` or `&&`, and a line that is only `if`, `while`, `until` or `!`. Before the
  step's last owner command it refuses `exit 0` (also inside quotes, as in `trap 'exit 0' EXIT`) and a
  bare `exit` with no `||` before it (`cmd || exit` passes the failure on and is allowed).
- Stated limits (closing them needs a shell parser): a mask through a condition, loop or negation
  written in other ways (for example `if cmd; then` on one line, or `cmd || echo`) is accepted.
  D1's subsequence rule accepts inserted lines that undo a guard (for example a second `export
  TMPDIR=` after the pinned one); the threat model is an accidental edit, not a hostile one.
  Known false refusals, all fail-closed: `|| true` inside quotes, a mask pattern inside a
  trailing comment (`cmd # || true`), and a trailing `&` inside a heredoc. A2-D1 is a presence test (any value) at step and job level, on all four
  owner jobs; the same helper holds the doctest twins (A1-D1).
- A1-D1/A1-D2: `DOCTEST_TWINS`; apart from its `--exclude`, `-p` and `--features` sets, the doctest
  command must equal its twin's other words with `--all-targets` turned into `--doc`, so a flag such
  as `--all-features` on one step only is refused and on both is accepted.
- A2-D2: `test-debug-a` must run an unconditional `cargo test --all-targets` that is `--workspace` with
  no `-p` and no `--exclude realtime-policy`, or names `-p realtime-policy`.
- Masking census outside scope: the one `set +e` in `qualification.yml` is in `audit-native`'s non-owner
  step "Issue-544 C ABI runtime caller audit" (it captures `PIPESTATUS` and re-raises). Not touched.
  The `continue-on-error` census is unchanged: only the two `verdict` telemetry steps.
- No owner step masks a failure today; the Non-goal stop did not trigger.

## Evidence (attempt 1)

- 42 mutation cases and 2 green controls in `test_generated_once_mutations`. With the parent
  commit's checker every case is accepted; with the change every case is refused; the controls
  (`--exclude foo`, `--all-features` on both twins) pass with both.
- Rule removal, each in turn: D1 -> the 8 #1429 cases accepted; D2 masks -> the 7 mask cases;
  D2 `exit 0` -> 1; A2-D1 step -> 5; A2-D1 job -> 5; A1-D1 step presence -> 2 deletions; A1-D1
  unmasked -> `if: false`; A1-D1 command shape -> `--lib`; A1-D2 -> 4 narrowing cases; A2-D2 ->
  the both-twins `--exclude realtime-policy` case. Verifier follow-ups: the quote characters in
  `EXIT_ZERO` -> the `trap 'exit 0'` case; the group-closer rule -> `} && echo ok`; the lone-keyword
  rule -> a lone `!`; the twin-derived doctest flags -> `--no-default-features` on the twin only
  (and the both-twins control turns red); the `- ` branch of `run_lines` -> the unnamed
  `- run:` owner step with `continue-on-error`. The `{ ... } || true` and `|| :` cases are also
  refused by the group-closer rule, so those two labels overlap; every other rule's cases are its
  own.
