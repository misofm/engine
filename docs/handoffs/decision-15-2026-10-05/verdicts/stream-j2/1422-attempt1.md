PASS

# #1422 *Run doctests in CI*: verdict, attempt 1 (under Amendment 1)

Commit `f2dccfdd9` on `codex/d15-stream-j2`, reviewed as `git diff f2dccfdd9^ f2dccfdd9` against the
committed spec (Amendment 1), AGENTS.md, decision 15 and the owner principle. I exported the commit to
`/tmp/claude-1002/v1422/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1422/target`. I did
not write to the worktree.

Root-sequenced and not a defect: the `crates/engine/src/realtime/plan.rs:539` pair (twin and comment).
`plan.rs` is not in the diff. Its `compile_fail` already runs in the new `test-debug-a` step (and in
the AArch64 leg). The issue cannot close until that pair lands.

No BLOCKER. No MAJOR.

## Points the coordinator asked about

- **Shared twin (render_session.rs, fpenv.rs).** It meets D2. Each `compile_fail` has a plain doctest
  that differs from it in exactly one construct: its bound (`T: Send` or `T: Sync` against `T`). The
  twin sits directly after the pair, and the prose names it as the twin of both. It does not weaken gate
  3. Two dedicated twins would be the same text, byte for byte, so the second one could catch nothing
  that the first does not catch. A typo in only one fence is out of reach of any twin design, because
  the twin is separate text. A rename of the type or path turns the single twin red in the same way. I
  measured this: with the same typo in all three snippets, line 63 (`render_session.rs`) and line 293
  (`fpenv.rs`) are red, and each `compile_fail` stays green. StartedRenderSession and CanonicalFpEnv
  are neither `Send` nor `Sync`, so a twin with no bound is the only possible shape.
- **Gate 2 `#[allow(unsafe_code)]`.** This is recorded correctly in the spec's Gate 2 paragraph,
  with the reason. I reproduced both halves:
  - Without the `allow`, `host-core` (lib) fails with "implementation of an `unsafe` trait ...
    requested on the command line with `-D unsafe-code`" before any doctest runs. The workspace sets
    `Cargo.toml:90 unsafe_code = "deny"`.
  - With the `allow`, the step exits 101 on `prepare::PreparedHost (line 515) - compile fail ...
    FAILED`.
  - The lane mutation needs no `allow`, because `fpenv.rs:78` has `#![allow(unsafe_code)]`. It is red
    at line 285.

  The spec's gate 2 text still shows the literal line (NIT 3, for root).
- **`scripts/test-test-support-ci.py`.** The anchor scoping is necessary. The parent's self-test, run
  against the new workflow, stops with "mutation anchor must occur once in qualification.yml: '--features
  builtins-compiler/...'". The scoping also checks itself: when I pointed `DEBUG_A_COMMAND` at the
  doctest step, 5 cases went red. The two new cases do not catch anything unique at this commit (MINOR
  1).
- **D1, router and verdict.** I parsed both jobs' steps with the checker's own parser. In each job, the
  doctest step's arguments are identical to the whole-package step's arguments, apart from `--doc` in
  place of `--all-targets`. The AArch64 line reuses the main run's `"${packages[@]}" --features
  "$features"`. There is no new job, so `ci-path-router.py` and the verdict table need no change.
  `check-ci-path-routing.py` and `test-ci-path-routing.py` both exit 0.

## MINOR

1. **`scripts/test-test-support-ci.py:187` and `:190`: the two new "narrowed to --doc" cases catch
   nothing unique at this commit.** I removed `--doc` from `NARROWING_TARGET_FLAGS`
   (`check-test-support-ci.py:44`) and ran a self-test that collects every case. 17 cases are red: the
   2 new ones and 15 rewritten, scoped cases. The 15 cases go red because the real doctest steps carry
   the same features as their whole-package steps. The implementer's own mutation evidence also names
   an existing case as the catch: "host-web/test-support removed from test-debug-a". I also deleted the
   `--features` lines from both doctest steps and applied the same mutation. Then only the 2 new cases
   are red. So they become the only catch only when a doctest step's feature list stops matching its
   job's list, and D1 forbids that today. The attempt record does not answer the test-value question
   for them. Fix: delete both cases and record that the scoped cases defend `--doc` narrowing while D1
   keeps the lists identical. Or record that conditional answer and let root accept it. This severity
   follows the earlier verdicts for rows with no unique catch (#1234 NIT 2, #1335 MINOR 2).
2. **`crates/graph-compiler/src/lib.rs:71`: the construct fence cannot see its own guarantee break.**
   I made every field of `PreparedBuiltinsGraphArtifact` (`crates/builtins-compiler/src/lib.rs:1822`)
   `pub`. Then the struct literal `PreparedGraphBuiltinsArtifact {}` fails with `E0063 missing fields
   builtin_observers, builtin_processors, graph and 3 other fields`, and the `compile_fail` stays green
   for the wrong reason. The fence was flipped to plain to read rustc's error. The guarantee is still
   enforced as a set: construction needs `graph` to be visible, and the mutate fence (line 90) and the
   extract fence (line 104) both go red in that run. So no guarantee is unprotected. But the spec's
   test-value sentence ("each is red if the type-level guarantee it states stops holding") is false for
   this fence alone, and the record says that no snippet has a wrong reason. Fix: record this in the
   Attempt record, and say in the doc comment that the field seals below enforce it. Naming every field
   in the literal is not a durable fix: each new field would bring back `E0063`.

## NIT

1. **Spec Attempt record, line 177 ("no snippet needed a fix"): the record omits some snippet edits.**
   Some `compile_fail` snippets were reshaped, which the spec allows only "where D2 finds a wrong
   reason":
   - The construct fence went from `let _ = PreparedGraphBuiltinsArtifact {};`, with its comment inside
     the snippet, to `fn construct() -> PreparedGraphBuiltinsArtifact { ... }`.
   - Five host-core and lane fences renamed `requires_send`/`requires_sync` to `requires`.

   Both edits are needed for "identical except for exactly the one construct". I checked that neither
   changes rustc's reason. List them with that reason.
2. **The worklet chain was not recorded, and the shipped module's digest moves.** This commit changes
   only doc comments, but it changes the shipped AudioWorklet module. Built locally with 1.97.1, the
   parent gives `c4d170dd...` and this commit gives `e4d822a6...`: 9 bytes differ and the size is
   unchanged. Each differing byte is a panic `Location` line number moved by +3, from the three lines
   added in `crates/host-core/src/prepare.rs`. The chain passes on this commit:
   - `build-web-audioworklet.sh --named-twin` exits 0.
   - `check-web-audioworklet.sh --without-metadata-regeneration` exits 0 (`render: closure=8
     traps=5`, sole owner `render_inner`).
   - `check-browser-expected-resources.py --artifacts` exits 0.

   Expect ARTIFACT CHANGED in the PR's `artifact-identity` summary. Record it in the batch record.
3. **Spec gate 2 text (root).** The literal `unsafe impl Sync for PreparedHost {}` cannot compile
   under `unsafe_code = "deny"`. Root may add `#[allow(unsafe_code)]` to the gate text so that a later
   reader does not run the literal line.

## Test value (one sentence per new or rewritten test)

- **The two CI steps and the AArch64 doctest line (D1).** They turn the merge red when a type-level
  guarantee breaks, and before this commit no CI step ran it:
  - `unsafe impl Sync for PreparedHost`: line 515 red.
  - `unsafe impl Sync for CanonicalFpEnv`: line 285 red, in the `test-debug-b` command.
  - `unsafe impl Send for StartedRenderSession`: line 50 red.
  - Every artifact field `pub`: lines 90 and 104 red.
- **GC construct twin (`lib.rs:79`).** Red if `graph_compiler::PreparedGraphBuiltinsArtifact` is renamed
  or moved and the doc snippet is not updated. I renamed the alias and updated the code, but not the
  docs: line 79 is red and line 71 stays green. Nothing else compiles that snippet's path.
- **GC mutate twin (`:96`), extract twin (`:110`), clone_back twin (`:124`).** The same defect for the
  artifact path in each signature or pattern. The same alias-rename run turns each one red, and their
  `compile_fail` partners stay green.
- **GC back_convert twin (`:139`).** Red if `graph::PreparedGraphPlan` or the artifact path goes stale
  in the snippet. Alias rename: red. Typo `PreparedGraphPlam` in both snippets: line 139 red, line 133
  green.
- **GC attach twin (`:154`).** Red if the `graph::PreparedGraphPlan` path goes stale in the snippet.
  This is the implementer's typo run. It correctly stays green on the alias rename, because neither
  snippet of the pair names the alias.
- **PreparedHost twin (`prepare.rs:524`, reshaped).** Red if the `host_core::PreparedHost` path goes
  stale in the snippet (typo in both: line 524 red, line 515 green). It keeps its earlier value: red if
  `PreparedHost` stops being `Send`.
- **StartedRenderSession shared twin (`render_session.rs:63`).** Red if the
  `host_core::StartedRenderSession` path goes stale in the snippets. Typo: line 63 red, lines 50 and 55
  green.
- **CanonicalFpEnv shared twin (`fpenv.rs:293`).** Red if the native `lane::fpenv::CanonicalFpEnv` path
  goes stale. This includes a cfg change that removes the type on x86-64 or AArch64. Typo: line 293
  red, lines 280 and 285 green.
- **Rewritten scoped cases in `test-test-support-ci.py`.** Each keeps its earlier unique value, now aimed
  at the whole-package step. As a set they are red if the checker counts a `--doc` step as
  whole-package coverage. If they pointed at the doctest step instead, 5 of them would be red, so the
  scoping checks itself.
- **New cases "test-debug-a/b narrowed to --doc".** None at this commit (MINOR 1). They are the only
  catch only if a doctest step's features stop matching its job's whole-package step.

## Gates run (export of `f2dccfdd9`, x86-64, toolchain 1.97.1)

- **Gate 1.**
  - `test-debug-a` doctest command: exit 0. 18 doctests: plan.rs 1 `compile_fail`; graph-compiler 6
    `compile_fail` + 6 twins; host-core 3 `compile_fail` + 2 twins.
  - `test-debug-b` doctest command: exit 0. 3 doctests: lane 2 `compile_fail` + 1 twin.
  - The AArch64 doc line, run on x86-64 with the same derived package list (capi closure +
    dsp-reference, conformance, target-smoke) and features: exit 0, 21 doctests. So in CI only
    target-specific behaviour is still open.
  - Real AArch64: open until the PR's `aarch64-debug` job runs. The command and the plan are correct.
- **Gate 2.** PreparedHost (with `allow`): exit 101, line 515 FAILED. The literal line without
  `allow`: the lib fails to build. CanonicalFpEnv: exit 101, line 285 FAILED.
- **Gate 3, redone.**
  - Rename runs:
    - GC alias rename: twins 79, 96, 110, 124 and 139 red; all 6 `compile_fail` green.
    - GC back_convert typo: line 139 red.
    - StartedRenderSession typo: line 63 red.
    - PreparedHost typo: line 524 red.
    - CanonicalFpEnv typo: line 293 red.
  - Delete runs (each red on its own fence only):
    - GC construct (71), mutate (90), extract (104) and attach (148).
    - StartedRenderSession Send (50) and Sync (55).
    - PreparedHost (515).
    - CanonicalFpEnv Send (280) and Sync (285).
  - Every fence flipped to plain once: each fails with exactly the code its comment names. These are no
    code (71), E0616, E0451, E0599, E0277, E0599, and E0277 for all host-core and lane fences.
- **Gate 4.** All exit 0:
  - `check-test-support-ci.py`, `test-test-support-ci.py`
  - `check-ci-path-routing.py`, `test-ci-path-routing.py`
  - `check-workspace-policy.sh`, `test-workspace-policy.sh`
  - `check-script-reachability.py`, `test-script-reachability.py`
- **Other gates.** All exit 0:
  - `cargo fmt --all --check`
  - `cargo clippy --locked --workspace --all-targets --all-features -- -D warnings`
  - `RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps`
  - `bash -n scripts/run-aarch64-tests.sh`
  - Worklet chain (NIT 2).

  shellcheck is not installed.
- **Gate 5.** Open until the batch PR runs, together with the new steps' wall times. Local warm wall
  time is about 20 s (debug-a) and 6 s (debug-b).
- **Other checks.** No `RUSTC_BOOTSTRAP` in the diff. Every changed path is in the spec's authorized
  list. The spec diff changes only the Attempt record.
