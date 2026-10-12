PASS

# #1519 attempt 3 verdict (last attempt; range 7d586f674..0436b10e7: 1ce10f872 spec, 0436b10e7 code; earlier slice commits d6b1d1e35, d5b26071f, 09edb4bb8 re-checked)

Verifier: fresh Opus 5.5 agent, 2026-10-12. Tree: `git archive 0436b10e7` into /tmp/claude-1002/v1519c/tree,
CARGO_TARGET_DIR=/tmp/claude-1002/v1519c/target, rustc 1.97.1, Python 3.12.3. No edit, build or commit in the worktree.
Evidence: /home/bl/misofm/submix-verdicts/evidence/1519-attempt3/.

Root's Amendment 2 is built as written. D1 (d) and the manifest-target-path rule are correct for their text. Every
recorded mutation goes red where the record says. The real tree refuses only `crates/effect-contract/src/live.rs:366`.
The findings below are a false claim in two sentences, and gaps that need rule text from root (see "Spec problems").

## BLOCKER

None.

## MAJOR

None.

## MINOR

**MINOR-1. The corrected #1438 note and the new walk.rs doc comment are still false.** The new texts are
`tools/realtime-policy/src/walk.rs:13-16` ("which keeps rustc out of them too") and
`.github/ISSUE_SPECS/1438-...md:164-166` ("rustc can reach them only through a `mod` item named like them, which
#1519 D1 (d) refuses"). Amendment 2 asked for this note to be corrected. The new sentence says more than the tool
does. Cargo puts rustc into a skipped directory with no `mod` item and no written `path =`:
- Target autodiscovery: `src/bin/target/main.rs`, `tests/node_modules/main.rs`, `examples/<skipped>/main.rs` and
  `benches/<skipped>/main.rs`, and `[[bin]] name = "target"` with no `path`. Measured:
  - `crates/lane/tests/target/main.rs` that holds `unsafe { *p }` is compiled by `cargo test -p lane --test target`.
  - The release tool does not read it: it reports only live.rs:366. The same file in `tests/visible/` is refused
    as unsafe outside ownership.
  - `scripts/check-workspace-policy.sh` gives `ok`.
- A path dependency into a skipped directory: `dep = { path = "node_modules/dep" }` builds. The manifest checker
  exits 0, as its text requires.

The rules that #1519 built are complete for their text. The sentences are not. The defect is only in the text, so
it is MINOR. Root decides the gap itself (spec problem 1). Before merge, change both sentences so that they claim
only what the rules refuse: `mod` items (D1 (d)), `#[path]` (D1 (a)), `include!` (D1 (b)) and manifest target `path`
and `build` keys. Each sentence must point to the successor slice for autodiscovered targets and dependency paths.

## NIT

- **No self-test for the inline array of tables (`bin = [{ .. }]`).** The record (spec line 378) says that no
  mutation of this checker can separate it from `[[bin]]`. In `tomllib` that is true: I measured that it is refused
  through the same code. The defect it would catch is a parser change, for example a merge into #1442's line-based
  awk pass in the same file. The #1442 attempt-3 verifier found that awk pass misses inline tables. Amendment 2 asks
  for one case per table kind, and that is met, so the drop is acceptable. Recommend that the slice that rebases
  #1442's rule adds `bin = [{ path = "../x.rs" }]` and `lib.path = ".."` as cases.
- **Three defensive branches have no test.** No self-test goes red when the Windows drive-letter check is dropped,
  when `\` is no longer a separator, or when an unreadable constant falls back to an empty list. None of these is a
  false pass on a supported build host: on Linux and macOS, `\` and `C:` are ordinary file-name characters inside the
  package. I measured fail-closed by hand (see below). The comments state these behaviours correctly.
- **The `name ! group` list is not complete** (`source_reads.rs:28-32`). `in !(..)` (`for _ in !(..)`) is matched
  too. The sentence does not claim the list is complete, and the over-refusal statement is true.
- **The case comparison is not full case folding.** The module doc (`source_reads.rs:43-46`) records that `ſ`
  (U+017F) folds to `s` on APFS but `to_lowercase` keeps it. This is not a limit that hides a CI false pass:
  - The walk skips only exact names (`walk.rs:83`), so on Linux CI `mod node_moduleſ { .. }` reads a directory the
    walk does read.
  - Folding only matters on a case-insensitive macOS dev build.
  - A one-line fold (`ſ` to `s`) closes it if root wants this.

## Checked and correct

- **D1 (d)** (`source_reads.rs:186-193`, `:274-280`) is refused in each of these forms:
  - inline and out-of-line modules
  - `pub(crate)` modules
  - nested modules
  - modules in a `macro_rules!` body, in a macro call and in a function body
  - `r#target`, `Target`, `Node_Modules`, `r#TARGET`
  - a `mod` keyword and its name on different lines (the finding is on the name's line)
  - `r#mod` is not a keyword, and it is correctly not matched (proc_macro2 `Ident == "mod"`)

  Rustc derives each child-module directory from module names, from a crate root's directory, or from an accepted
  plain-`.rs` `#[path]`. D1 (a) and D1 (d) together close the `mod`/`#[path]` route. The accepted forms do not
  over-refuse: `targets`, `target_dir`, `node_modules_x`, `my_target`, `use crate::target`, `fn target`,
  `struct Target`. The real tree has 0 refusals. A substring mutation would refuse
  `effect-contract/src/lib.rs:8 mod prepared_target;` and two more, and `the_accepted_forms_pass` catches it.
- **D1 (a) now uses the same constant and compares ignoring case.** The walk exports `SKIPPED_DIRECTORIES`
  `pub(crate)`, and nothing is copied.
- **The manifest rule** (`scripts/check-workspace-policy.sh:295-373`) refuses each of these forms (evidence item 5):
  - for the requested classes: a top-level inline `lib = {..}`, dotted `lib.path`, quoted `["lib"]`, spaced
    `[ lib ]`, the escaped key `"pa\u0074h"`, inline `bin = [{..}]`, dotted `package.build`
  - `build.workspace = true`: `build` is not inheritable, and the rule refuses it as "not a non-empty string"
  - `./src/./target/x.rs`, backslash paths, `C:\..`, `c:x.rs`, UNC paths, `TARGET`, `NODE_MODULES`, `src/..`
  - an empty value, a non-string value, a `[bin]` that is not an array, `lib = "<string>"`, invalid TOML

  These forms are correctly accepted:
  - `lib = {..}` written under `[package]`: that is `package.lib`, not a target
  - `build = true` and `build = false`
  - `~/x.rs` and `src/.../x.rs`: on Linux these are ordinary components inside the package
  - `[workspace.package] build`

  For a `path` through a symlink in a package under the roots, the walk refuses a symlink to a directory
  (`walk.rs:70-81`). The real tree has 43 tracked manifests and 20 target `path` keys (13 under the roots), with no
  `build` key, and all are accepted.
- **Reading `SKIPPED_DIRECTORIES` with a regex.** This is acceptable. It is a gate that reads configuration and acts
  on the value. It is not a test that asserts source text, and AGENTS.md refuses only the second kind (#1438 Hazards
  draws the same line). It fails closed, which I measured: with the constant rewritten as `&[&str] = &[..]` or
  `[&str; 0] = []`, the gate reports "manifest target path scan failed (status 1) ... cannot read
  SKIPPED_DIRECTORIES", which is an execution failure, not a pass. `test-workspace-policy.sh` copies walk.rs beside
  its mutant scripts, and no other script copies the policy script.
- **python3 3.11 or later (`tomllib`).** This is acceptable. Every CI job runs on `ubuntu-24.04` or
  `ubuntu-24.04-arm`, which have Python 3.12, and four existing lint scripts already import `tomllib`
  (`check-test-support-ci.py`, `check-release-shape.py`, `lib/gate-rules.py`, `test-test-support-ci.py`).
- **MINOR-1 of attempt 2 is fixed.** The accepted rows `assert_eq!(entry.path, path)`, `json!({"path": path})` and
  `($path:literal)` are added. With PATHALL, only `the_accepted_forms_pass` goes red.
- **MINOR-2 of attempt 2 is fixed.** The module doc, the Cases bullet and the attempt-2 bullet are now true. I checked
  the claim that a macro-tree group only refuses more, in `Scan::ident` and `attribute`.
- **The attempt-2 NITs are fixed:**
  - The attempt-1 mutation-table correction (D 6, L 4, M 4, P/R 3, Q 2) matches attempt 2's re-run.
  - The 223/233/220 counts are recorded.
  - Line references are replaced by function names.
  - The case-insensitive-file-system NIT is closed by the case-insensitive compare.
- **No regressions in the earlier slice commits.** I re-ran attempt 2's A, D, C and E mutations: they are red on the
  same tests as attempt 2's re-run (A on 7, D on 6, C on 1, E on 2).
- **Authorised paths.** The commit touches only:
  - `tools/realtime-policy/**`
  - the two named scripts, for the manifest rule only
  - the #1519 spec
  - the one #1438 sentence

## Mutations re-run (each alone; red = tests or self-test cases that fail)

Rust, on `source_reads.rs`, `cargo test -p realtime-policy --test policy` plus the release tool on the real tree:

| Mutation | Red | Real tree |
|---|---|---|
| D-drop | module, case and line tests (3) | live.rs:366 only |
| D-not-in-macro | module test only | live.rs:366 only |
| D-raw (`to_string`, no `r#` strip) | module test, case test | live.rs:366 only |
| D-case-sensitive at the `mod` site | case test only | live.rs:366 only |
| D-line (finding on `mod`) | line test only | live.rs:366 only |
| A-case-helper (no lower-casing) | case test, `a_path_to_a_file_outside_the_workspace_is_a_finding` | live.rs:366 only |
| D out-of-line only | module test, case test | live.rs:366 only |
| D inline only | module, case and line tests | live.rs:366 only |
| D `target` only | module test, case test | live.rs:366 only |
| D substring | `the_accepted_forms_pass` only | 3 real `mod *target*` refusals |
| PATHALL | `the_accepted_forms_pass` only | builtins-compiler/src/lib.rs:12468 and more |
| A (a) dropped | 7 path tests | live.rs:366 only |
| D (b) dropped | 6 include tests | live.rs:366 only |
| C exception in macro trees | macro test only | live.rs:366 only |
| E (c) dropped | use test, raw test | live.rs:366 only |

Manifest checker, on `check-workspace-policy.sh`. I used a copy of the self-test that does not stop at the first
failure; the real tree is in the last column:

| Mutation | Red | Real tree |
|---|---|---|
| lib, bin, test, example, bench, build dropped (each alone) | its own `target-path-*` case only | ok |
| `..` check dropped | `-lib`, `-build` | ok |
| absolute check dropped | `-test` only | ok |
| skipped check dropped | `-bin`, `-example`, `-bench` | ok |
| case-sensitive compare | `-bench` only | ok |
| substring compare | `allow-target-path-lookalikes` only | fails |
| dependency `path` keys also scanned | `allow-target-path-lookalikes` only | fails |
| violations never reported | all six `target-path-*` cases | ok |
| drive-letter check dropped, `\` not a separator, `build` with any non-string accepted | none (NIT) | ok |

All rows match the attempt-3 record.

## Test value (one sentence per new test or self-test case)

- `a_module_named_like_a_skipped_directory_is_a_finding`: (d) dropped, or limited to inline or out-of-line
  modules, to code outside macro token trees, or to one name, or `r#` not stripped; the attempt-2 false pass
  `mod target { #[path = "x.rs"] mod y; }` returns.
- `a_skipped_directory_name_is_matched_ignoring_case`: a case-sensitive compare at the `mod` site.
- `the_finding_names_the_line_of_the_module`: the finding placed on the `mod` keyword's line.
- New rows in `a_path_to_a_file_outside_the_workspace_is_a_finding` (`Target/x.rs`, `a/Node_Modules/x.rs`): a
  case-sensitive component check in (a).
- New rows in `the_accepted_forms_pass`: PATHALL (a `path` refused outside a written attribute) and a substring or
  non-`mod` match of a skipped name.
- `target-path-lib`: `[lib]` not read. `-bin`, `-example`, `-bench` and `-build`: that kind not read.
  `-test`: that kind not read, or the absolute check dropped. `-bench` also: a case-sensitive compare.
- `allow-target-path-lookalikes`: a checker that reads dependency `path` keys or matches skipped names as
  substrings. Today the real-tree gate catches these too, but the case does not depend on the tree's current
  manifests.

## Gates run (on the exported 0436b10e7 tree)

1. Release `realtime_policy .`: refuses exactly `crates/effect-contract/src/live.rs:366` (expected before #1345)
   and nothing else.
2. `cargo test --locked -p realtime-policy`: 186 passed.
3. `cargo fmt --all --check`: exit 0. `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`: clean.
4. The lint job's x86_64 steps that touch these files all exit 0:
   - `bash scripts/check-workspace-policy.sh`: `ok`
   - `bash scripts/test-workspace-policy.sh`: `ok`
   - `check-realtime-policy.sh`: `ok (94 marked regions in 26 files)`
   - `test-realtime-policy.sh`, `check-realtime-audit-leak.sh`, `test-realtime-audit-leak.sh`
   - `check-ci-path-routing.py`, `test-ci-path-routing.py`
   - `check-script-reachability.py`: 120 reached
   - `test-script-reachability.py`
5. The mutations in the tables above.

## Spec problems for root

1. **Cargo puts rustc into source that the walk does not read, and no rule refuses it** (MINOR-1 evidence). This
   is outside the text of D1 (a)-(d) and of the manifest-target-path rule, and the implementer could not close it
   inside the authorised paths. The product outcome ("refuses every way rustc reads source that the walk does not
   read") is therefore not fully met. The forms:
   - (a) Autodiscovered bin, test, example and bench targets in a directory named `target` or `node_modules`, and a
     `[[bin]] name` that names one, with no `path`.
   - (b) Dependency `path` keys (all dependency tables, `[target.*]` dependency tables, `[workspace.dependencies]`,
     `[patch]`) that resolve outside the scan roots or through a skipped component. A library under a skipped
     directory is linked into a shipped crate, so this is the serious form.
   - (c) Workspace members outside the roots or in a skipped directory.

   Real tree: 45 dependency path keys, all under crates, hosts or tools with no skipped component; every member is
   under the roots; no tracked file sits under a directory named `target` or `node_modules`. A rule costs 0 real
   refusals.

   One candidate rule closes (a), (b) and (c) and the `mod` route together: refuse every tracked or untracked
   non-ignored path under crates, hosts or tools that has a `target` or `node_modules` component. Add a manifest
   rule that every dependency path and member resolves under the roots. Following Amendment 2 ("split that rule into
   its own slice"), I recommend a successor slice beside the manifest rule, not a reopening of #1519.
2. **Doctests are source that rustc reads and the tool does not tokenize.** CI runs `cargo test --workspace --doc`
   (qualification.yml:639). There are 36 doc-comment code blocks under the roots that rustdoc compiles (bare, `rust`, `no_run` or `compile_fail` fences; `text`/`ignore` and similar left out). Rustdoc
   resolves `include!` and `#[path]` there relative to the source file: I measured that `include!("hidden.txt")`
   reads `src/hidden.txt`. No #1438 limit covers this. Root decides between a limit (test-only code) and a rule (for
   example, refuse a doctest fence that is not `text`/`ignore`, or require `doctest = false`).
3. **MINOR-1 wording.** The #1438 sentence was root-ordered text. Root should approve the replacement wording when
   it fixes the sentence at merge.
