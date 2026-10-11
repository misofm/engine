# Check realtime regions with a Rust syntax-tree tool: region boundaries, the forbidden-body predicate and the whole-plan check

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice A2 of nine (C1 (#1445), A1 (#1438), A2, B1a (#1440), B1b (#1441), B2a (#1442), C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what each
holds).
- A1 built `tools/realtime-policy`: the crate, the binary, the walk, unsafe ownership, markers,
  floors and the case harness.
- **A2 (this issue)** adds the syntax rule for regions (where a marker may sit and which nodes a
  region holds), the forbidden-body predicate (C7) and the whole-plan check (C9). After A2 the tool
  runs every check of the awk gate except the drain rule.

No production code changes.

## Problem (verified on `origin/main` at `6d28a80ec`)

The markers and files cited below are the same on `codex/d15-stream-b` at `b8392df66` (stream B
batch 1), which adds four regions and one marked file. Each of its eight new marker lines sits
between two items of its file (measured on the diff). Re-check the marker claims on `main` at
implementation time. The script and self-test lines are `main`'s: on `b8392df66` the script's
lines from `:70` on are two lower (`:78-79`, `:438-461`) and the self-test's lines from `:440` on are
one lower (A1's Problem lists them).

- **The checks.** `scripts/check-realtime-policy.sh` C5 (a region is the lines between markers,
  `:58-63`), C7 (the forbidden-body predicate: 37 regexes over region lines, `:76-77`) and C9 (the
  C ABI never forms a reference to a whole `Plan`, `:436-459`). A1's spec has the full table.
- **The self-test cases:** 9 forbidden-body cases (`scripts/test-realtime-policy.sh:529-541`,
  `:585-596`) and 9 whole-plan cases (`:1422-1433`).
- **The real tree's markers, in syntax-tree terms.** 175 of the 178 marker lines sit at a child
  boundary of the file, an `impl` or a `trait`. Three do not:
  - `crates/gate-expander/src/lib.rs:1014` (BEGIN) and `:1033` (END) sit inside the token tree
    of `macro_rules! bank_impl`, which `:1034-1036` closes;
  - `crates/rack/src/lib.rs:627` (BEGIN) sits between the doc comment of `lane_symmetry_bank`
    (`:621-626`) and its `fn` (`:628`). `syn` puts outer attributes inside the item's span.
- Two regions start inside an `impl` and end after it closes:
  - `crates/effect-contract/src/live.rs:231-537` (`impl EffectControlLane`, `:156-497`);
  - `crates/transient-shaper/src/lib.rs:587-703` (`impl Shaper`, `:457-629`).
- No real region has one marker inside a macro token tree and the other outside it.
- **`crates/capi/src/ffi.rs`.** The awk stops at `mod tests {` (`:448`), so items after the test
  module are not scanned today. No marked region on `main` calls `collect` in any form.

## Decisions

- **D1 (C5, region boundaries).**
  - **Allowed marker positions.** A marker line must sit at a child boundary of its innermost
    container: before the first child, between two children, or after the last child. A container
    is one of:
    - the file;
    - an inline `mod`, an `impl`, a `trait` or an `extern` block;
    - a block (a `{ .. }` statement list);
    - a macro token tree. Its children are the token trees at one nesting level of the group.
  - **Attributes.** For this rule, an item starts at its first token after its outer attributes.
    So a marker between an item's doc comments or attributes and the item is at a child boundary.
  - Any other position is a finding: "region boundary splits a <node kind>".
  - **A region that crosses a macro token tree's boundary** is a finding: "region boundary splits
    a macro token tree". That is a region whose BEGIN and END sit in different containers and one
    of those containers is a macro token tree (or lies inside one). The region would otherwise hold
    some of the macro's tokens and some syntax, and no later rule reads such a mix.
  - **Region nodes.** A region's nodes are the maximal syntax nodes whose span, with the attribute
    rule above, lies wholly between its markers. For a region inside a macro token tree, the nodes
    are the token trees between the markers: the region is tokens, not syntax.
  - The real tree's 178 markers pass this rule. That covers the two macro-body markers, the
    attribute-gap marker and the two `impl`-spanning regions.
- **D2 (C7, forbidden-body predicate).**
  1. **Render the tokens.** Take each region's tokens whose start line is inside the region.
     Render them as text: one space between two adjacent identifier, keyword or literal tokens,
     and no space anywhere else. Render each literal as an empty literal of its kind. Comments
     are not tokens.
  2. **Match the 37 patterns** of `:77`. Each is a literal string after its regex escapes are
     removed, matched as a substring of the rendered text (A1-D2: no regex crate): `Vec::`,
     `vec!`, `Box::`, `String::`, `.to_vec(`, `.collect(`, `Arc::clone`, `Rc::clone`, `drop(`,
     `Mutex`, `RwLock`, `Condvar`, `mpsc`, `sync_channel`, `thread::`, `sleep(`, `yield_now`,
     `spin_loop`, `std::fs`, `std::net`, `std::process`, `println!`, `eprintln!`, `format!`,
     `log::`, `tracing::`, `.await`, `File::`, `Tcp`, `Udp`, `.expect(`, `.unwrap(`, `panic!(`,
     `unreachable!(`, `todo!(`, `unimplemented!(`. Two are not literals and change:
     - `async[[:space:]]` becomes the `async` keyword token itself, because `async {` renders as
       `async{`.
     - `\.collect\(` is joined by `.collect::<`, so the turbofish form is caught too. This is
       #948's slice.
  3. The class is "marked realtime forbidden-body predicate".

  Token rendering keeps the regex semantics. A longer identifier still matches where the regex
  matched it (`SmallVec::`, `CString::`, `do_drop(`). It also catches spellings that the line
  regex missed (`Vec :: new`, a pattern split across lines). It no longer matches text inside
  comments and literals. Macro token trees are scanned like any other tokens.
- **D3 (C9, whole-plan reference).**
  - Parse `crates/capi/src/ffi.rs`. If the file is missing, the finding is "missing
    crates/capi/src/ffi.rs".
  - **Production items** are every item except a top-level `mod tests` whose outer attributes
    include exactly `#[cfg(test)]`. A `#[cfg(any(test, ..))]` module is production code. (This is
    narrower than B2b-1's test-code rule on purpose: it is today's C9 scope, plus the items after
    the test module.)
  - There must be a production `fn` item named `miso_engine_v1_render_f32_planar`. If there is
    none, the finding is "has no production render entry point to scan".
  - **In production items, a finding is** `&` or `&mut` whose operand, after any parentheses, is
    `*plan`, where `plan` is the single-segment path `plan`. That covers `&*plan`, `&mut *plan`,
    `&(*plan)` and `&mut (*plan)`. Whitespace is irrelevant in the tree.
  - **Not findings,** as today: a projection after the dereference (`&(*plan).field`,
    `&*plan.cast::<H>()`) and another name (`&*plan_state(..)`).
  - **In macro token trees,** a finding is the token sequence `&`, optional `mut`, then `*` `plan`
    or the group `( * plan )`, where the next token is absent or is none of `.`, `(`, `[` and `::`.
    That is the tree rule above, read on tokens.
  - The class is "the C ABI forms a reference to a whole Plan; project the field with &raw const
    or &raw mut".
  - Items after the test module are now scanned.
- **D4. Library cases, ported** (A1's case format, A1-D9):
  - **Forbidden body (9):** `allocation`, `lock`, `log`, `panic-path-expect`,
    `panic-path-macro`, `marked-outside-realtime-root`, `marked-runtime-execute-op`,
    `marked-effect-control-lane-stage`, `marked-tools-root-scanned`.
  - **Whole plan (9):** the five forms at `:1422-1425`, `whole-plan-at-line-end`,
    `whole-plan-after-a-comment-line`, `capi-ffi-missing`, `capi-render-entry-only-in-tests`.
- **D5. New cases, one each.**
  - **D1:**
    - a marker inside a `match` arm list;
    - a marker inside a call's argument list;
    - a BEGIN inside a `macro_rules!` body with its END after the macro (refused, macro boundary);
    - the gate-expander shape: a region inside a `macro_rules!` body (passes);
    - the rack shape: BEGIN between a doc comment and its `fn` (passes);
    - a region that spans the end of an `impl` (passes).
  - **D2:**
    - a forbidden spelling inside a user macro's arguments;
    - `.collect::<Vec<_>>()`;
    - `Vec` and `::new` split across two lines;
    - `async{` with no space;
    - a forbidden word only inside a comment and a string literal (passes).
  - **D3:**
    - a whole-plan borrow in an item after `mod tests`;
    - `&*plan` inside a macro call in production code;
    - `&*planner` (passes);
    - `&*plan[0]` inside a macro call (passes: an index, not the whole plan);
    - a `#[cfg(any(test, feature = "x"))] mod tests` that holds a borrow (fails).
- **D6. #948.** This slice closes *Make the realtime policy scan catch turbofish collects* (#948)
  if the verifier confirms the D2 turbofish case. The steps:
  1. The D5 turbofish case is the negative fixture, and gate 1 shows the workspace passes.
     Together they meet #948's objective gates.
  2. Append an Evidence section to `.github/ISSUE_SPECS/948-make-the-realtime-policy-scan-catch-turbofish-collects.md`:
     the case name, the gate-1 line, the verdict path, and "closed by tool slice A2".
  3. After the batch push puts that commit on `main`, run C1's
     `bash scripts/operator/sync-spec-bodies.sh <base>..<batch tip>` (so #948's body matches its
     spec) and its `--check`, then `gh issue comment 948 --body-file <evidence>` and
     `gh issue close 948 --reason completed`. If C1 is not on `main` yet, run
     `gh issue edit 948 --body-file <spec>` and compare the fetched body by hand instead.
  4. Verify the issue state with `gh issue view 948 --json state` and record it.
  5. Remove the spec from `.github/ISSUE_SPECS/` at the next batch, as AGENTS.md says.

## Authorized paths

- `tools/realtime-policy/**`
- `.github/ISSUE_SPECS/948-make-the-realtime-policy-scan-catch-turbofish-collects.md` (its
  Evidence section only; owned by S0)
- This spec

## Non-goals

- The drain rule (B1a, B1b, B2a, B2b-1, B2b-2) and deleting the awk gate (C2).
- New forbidden surfaces. D2 changes only spelling coverage: the turbofish, whitespace, and
  comments and literals. `Arc` method-form `.clone()`, `assert!` and `unsafe trait` stay as they
  are today.
- Any Rust source outside `tools/realtime-policy`.

## Hazards

- As in A1: the two-gates rule, re-measuring on rebase, and the awk gate scanning the tool's own
  sources (build `REALTIME_POLICY_` and the 37 pattern strings that the awk would match with
  `concat!`, or keep them out of the awk's reach as A1's Hazards say).
- **Over-refusal is acceptable, a false pass is not.** A marker the tool cannot place refuses
  (D1).

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints the same counts
   as `bash scripts/check-realtime-policy.sh` on the same tree.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every A1 case and every D4
   and D5 case.
3. **Parity (PR evidence).** As A1's gate 3, over this slice's cases, with these deliberate
   differences (name the case for each):
   - D1: the split findings and the macro-boundary finding;
   - D2: token rendering, the turbofish and `async{`. The script's regex also matched its own
     `path:line:` prefix, and the tool does not;
   - D3: items after `mod tests`, macro token trees and `cfg(any(test, ..))`. The awk flags
     `&*plan(..)`, `&*plan[..]` and `&*plan::..`; the tool does not (the `&*plan[0]` case). In the
     tree these are a call, an index and a path, not a reference to the whole plan.
4. **Mutations (PR evidence).** Apply each alone. Each turns at least the named case red:
   - skip macro token trees in D2: the user-macro case;
   - match D2 on raw line text: the split-`Vec` and comment/literal cases;
   - drop D1's split check: both split cases;
   - drop D1's macro-boundary check: the macro-boundary case;
   - drop D1's macro-tree container: the gate-expander shape (and gate 1);
   - drop D1's attribute rule: the rack shape (and gate 1);
   - narrow D3's production scope to items before `mod tests`: the after-tests case.
5. **Run time (PR evidence).** As A1's gate 5.
6. **Shipped artifacts unchanged.** As A1's gate 6.
7. As A1's gate 7.

*Test value.*
- The ported cases keep the claims that their script rows made.
- The D1 cases are red if a region that splits an expression or crosses a macro's boundary is
  accepted, or if the real tree's macro-body and attribute-gap markers are refused.
- The user-macro case is red if the forbidden scan skips macro token trees. A `syn` visitor skips
  them by default.
- The split-`Vec`, `async{` and comment/literal cases are red if the predicate falls back to line
  text.
- The turbofish case is red if `collect` is matched only in its plain form (#948).
- The D3 cases are red if production scope or the macro-token match is narrowed or widened.
- Ported lexer-era cases that a `syn` tool cannot get wrong are committed as a regression corpus.
  Root's ruling of 2026-10-05 ("every existing self-test case becomes a committed case of the
  tool") is the authority for them.

## Evidence

- Gates 1-7 output. Gate 3's table. Gate 4's mutation runs.
- #948's closing comment and the issue state.

## Dependencies

- After (same stream): A1.
- Before: B1a.
- Closes: #948 (D6).

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.

## Attempt record (attempt 1, J batch branch `codex/d15-stream-j3`)

Implemented on `e5fa8319b` (A1 with its verdict follow-ups). No production code changed. Edits:
`tools/realtime-policy/**` and this spec, plus #948's Evidence section. No `Cargo.toml` or `Cargo.lock` change
(`syn` 2.0.119 and `proc-macro2` 1.0.107 only; no `quote`).

**What is where.**
- `src/leaf.rs`: the token stream flattened to leaves (word, literal, punctuation, group delimiters). A1's marker
  pass now reads it too (its private copy is gone).
- `src/boundary.rs` (D1): a `syn` visitor records the containers (file; the braces of `mod`, `impl`, `trait`,
  `extern`; blocks; every group of a macro's token tree) with the children of each, and every attribute. A child
  starts at its first leaf after its outer attributes. Markers are placed by one sweep over the containers. The
  finding names the outermost item or statement that holds the marker ("a match expression", "a call
  expression", "a let statement", "a struct", ...), or "an attribute".
- `src/forbidden.rs` (D2): the region's leaves rendered as text (one space only between two words, literals
  rendered empty), 37 substring patterns (the awk's 36 literals plus `.collect::<`) and the `async` keyword token.
  One finding per line, on the line where the spelling starts.
- `src/whole_plan.rs` (D3): production items (all but a top-level `mod tests` under exactly `#[cfg(test)]`);
  `ExprReference` over `*plan` through parentheses; the token rule in every macro's tokens (`macro_rules!`
  bodies included).
- New classes: `RegionSplits(&'static str)`, `RegionSplitsMacroTree`, `ForbiddenBody`, `MissingCapiFfi`,
  `NoRenderEntry`, `WholePlanReference`.

**Deviations from the spec's numbers and text.**
- Real-tree line numbers are the branch's, not the spec's: the gate-expander region is `:1022-1041` (spec
  `:1014`/`:1033`), the transient-shaper region `:613-726` (spec `:587-703`), the effect-contract region
  `:231-537` (same). Measured with a temporary print (not committed): of 93 regions, exactly one has a macro-tree
  container (gate-expander, both markers in one group) and exactly two have markers in different containers
  (effect-contract, transient-shaper; neither involves a macro tree). The rack marker (`:628`) passes only by the
  attribute rule.
- `Policy::always_parsed` is gone (a public field of A1). D3 parses `crates/capi/src/ffi.rs` anyway, so the
  library parses that one path by the constant `whole_plan::CAPI_FFI`; the A1 parse-error case
  (`walk-ffi-parse-error`) still holds. "missing crates/capi/src/ffi.rs" is a bare finding, as the missing
  realtime module is.
- D2 runs on any file with regions, even when the file does not parse (the parse error is its own finding).
- D1 names the node by the outermost statement or item that holds the marker, not the innermost expression: the
  tree has no other place to stand once the container's children are the rule.
- The spec's "region nodes" (maximal nodes between the markers) are not built as a type: no A2 rule reads them.
  The container/children layout in `boundary.rs` is what B1a extends (a region's nodes are the children of its
  innermost container that lie between the markers).
- Case counts: D4's nine whole-plan cases are 8 tests rows: the five forms are one test (`every_form_of_...`),
  the other four are `layout_does_not_hide_...` (2) and `a_missing_file_or_a_render_entry_only_in_tests_...` (2).
  The nine forbidden-body cases are one table test. 53 tests became 77.

**Gates.**
1. `cargo run --locked --release -q -p realtime-policy` prints `realtime policy: ok (93 marked regions in 26
   files)`; `bash scripts/check-realtime-policy.sh` prints the same.
2. `cargo test --locked -p realtime-policy`: 77 passed (one `policy` binary).
3. Parity: 185 case trees (every tree the harness builds, kept by a throwaway patch of `Tree::drop`, reverted)
   through gawk `check-realtime-policy.sh` and the release binary: 130 agree (44 both pass, 86 both fail), 55
   differ. 20 are A1's (markers `markers-end-before-begin`, `markers-nested`, `open-tail`; `unsafe-mentions`;
   the walk cases `walk-dir-named-rs`, `-rs-link`, `-fifo`, `-not-utf8`, `-lex-error`, `-parse-error`,
   `-ffi-parse-error`, `-skipped-directories` and the eight `walk-verbatim-*`). The 35 of this slice, each a
   listed deliberate difference:
   - D1 (awk pass, tool fail, 21): `boundary-in-match-arms`, `-in-call-arguments`, `-in-struct-fields`,
     `-in-enum-variants`, `-in-parameters`, `-in-let-initializer`, `-before-else`, `-in-method-chain`,
     `-in-impl-header`, `-in-struct-expression`, `-in-closure-parameters`, `-in-attribute`, `-only-begin-splits`,
     `-only-end-splits`, `-finding-place` (the split findings); `boundary-begin-in-macro-end-after`,
     `-begin-before-macro-end-inside`, `-begin-in-outer-group-end-in-inner`,
     `-begin-in-group-end-two-groups-deeper`, `-between-two-macro-calls`, `-block-to-macro-group` (the
     macro-boundary finding).
   - D2 (7 awk pass, tool fail): `forbidden-turbofish-collect`, `-split-across-lines`, `-split-vec` (the
     `Vec`/`::new` split), `-spaced-path`, `-spaced-call`, `-spaced-macro`, `-async-no-space`; and (awk fail,
     tool pass): `forbidden-words-in-comments-and-literals`. The `path:line:` prefix match of the script's regex
     shows in no case tree.
   - D3 (awk pass, tool fail, 2): `whole-plan-after-tests`, `whole-plan-in-macro-rules`; (awk fail, tool pass,
     4): `whole-plan-macro-call` (`&*plan(1)`), `-macro-index` (`&*plan[0]`), `-macro-path` (`&*plan::DEFAULT`),
     `whole-plan-tree-lookalikes` (the tree forms of the same call, index and path).
   - Agree, though the spec expected a difference: the `cfg(any(test, ..))` and `#[cfg(test)] mod helpers`
     trees. The awk stops only at a `#[cfg(test)]` line followed by `mod tests {`, so it too scans those modules.
     The tool differs from the awk there only in the other direction (an exact attribute is required), which no
     tree separates.
4. Mutations, each applied alone, then reverted (`cargo test` after each; 77 green on the revert). "red" lists
   the tests that went red; the spec's seven first.
   - Skip macro token trees in D2: 1 red, `a_spelling_the_line_scan_missed_is_a_finding` (the user-macro rows).
   - D2 on raw line text: 3 red, `a_finding_names_the_line_where_the_spelling_starts` (split `Vec`),
     `a_forbidden_word_in_a_comment_or_a_literal_passes`, `a_spelling_the_line_scan_missed_is_a_finding`.
   - Drop D1's child split check: 3 red, `a_marker_inside_a_node_is_refused`,
     `either_marker_alone_can_split_a_node`, `a_boundary_finding_names_the_marker_line` (gate 1 stays green: the
     real tree has no split marker). Drop the attribute split check: 1 red, `a_marker_inside_an_attribute_is_refused`.
   - Drop D1's macro-boundary check: 1 red, `a_region_that_crosses_a_macro_token_tree_is_refused`.
   - Drop D1's macro-tree container: 2 red, `a_region_inside_a_macro_body_is_tokens_and_passes` and the crossing
     test; gate 1 refuses `crates/gate-expander/src/lib.rs:1022` and `:1041` ("splits a macro item").
   - Drop D1's attribute rule (a child starts at its first leaf): 2 red,
     `a_marker_between_attributes_and_their_item_is_a_boundary`, `a_marker_inside_an_attribute_is_refused`; gate 1
     refuses `crates/rack/src/lib.rs:628` ("splits a method"). Skip only the first attribute: 1 red (the
     several-attributes row, which the one-attribute row did not catch), gate 1 red on rack.
   - Narrow D3 to items before `mod tests`: 2 red, `a_whole_plan_borrow_after_the_test_module_is_a_finding`,
     `a_whole_plan_borrow_inside_a_macro_is_a_finding` (the macro row sits in the entry point, which is before the
     tests; the second red is the `macro_rules!` row appended after them).
   - Further, one red each unless noted: `.collect::<` dropped (`a_spelling_the_line_scan_missed...`); `async`
     rule dropped (same); literals kept as text (`..._in_a_comment_or_a_literal_passes`); word-boundary
     matching (`a_longer_identifier_still_matches`); whole file scanned (`only_the_tokens_between_the_markers_are_read`);
     only the first region scanned (6 red); no space between words (`adjacent_words_do_not_join_into_a_spelling`,
     which survived all 76 earlier tests until it was added); a finding per match instead of per line
     (`findings_are_one_per_line`); forbidden scan only under the realtime root
     (`a_forbidden_spelling_in_a_region_is_a_finding`); match end taken as the line
     (`a_finding_names_the_line_where_the_spelling_starts`); nested macro groups not containers (the
     two-groups-deeper row; the first version of the row survived); crossing check on one side only (each
     side: the crossing test); crossing flagged for one container (`a_region_inside_a_macro_body_is_tokens_and_passes`);
     any container change a crossing (3 red incl. `a_region_may_span_the_end_of_an_impl`); no `mod`, `impl`,
     `trait`, `extern` or block container (`a_marker_between_the_children_of_a_container_is_a_boundary`; the
     `impl` and block ones also red elsewhere; gate 1 stays green for block); only the begin or only the end marker
     checked (`either_marker_alone_can_split_a_node`; the end-only one 3 red); the finding at the region's begin
     instead of the marker (`either_marker_alone_can_split_a_node`, after its line assertions were added: it
     survived before); the crossing finding at the end marker (the crossing test, after its line assertion was
     added: it survived before); stale-container pop dropped from the sweep (5 red); statement name `an
     expression` (3 red); D3 test module accepted under any `cfg` or any name
     (`only_a_cfg_test_module_named_tests_is_skipped`); no parenthesis peel (`every_form_of_...`); `mut`
     borrows ignored in the tree (`every_form_of_...`) and in macros (`a_whole_plan_borrow_inside_a_macro...`);
     macro group form `(*plan)` off (same); each of `.`, `(`, `[`, `::` as a continuation off
     (`borrows_that_are_not_the_whole_plan_pass`, one red each); a single colon read as `::` (survived until the
     `whole-plan-before-a-colon` row was added); a prefix match on `plan` in macros and in the tree
     (`borrows_that_are_not_the_whole_plan_pass`); render-entry check off or test module counted
     (`a_missing_file_or_a_render_entry_only_in_tests...`); missing file not reported (same); finding line off by
     one (`layout_does_not_hide_a_whole_plan_borrow`).
5. Run time, release binary, on a loaded machine (load average 6.5): real tree 1.02-1.04 s (A1: 0.75-0.86 s;
   with the D1 and D2 calls removed it is 0.75 s here, so the two cost about 0.28 s: 0.12 s is the children's
   `Spanned` spans, the rest the second and third flattening of the marked files' tokens). Base tree plus one
   synthetic marked file of 20,004 lines (3,334 regions): 0.04 s (A1: 0.13 s). The awk gate: 0.58 s. Not tuned.
6. Shipped artifacts: nothing outside `tools/realtime-policy` and `.github/ISSUE_SPECS` changed against the
   parent `e5fa8319b` (`git diff --stat e5fa8319b..HEAD -- . ':!tools/realtime-policy'` is empty before the spec
   commit), `cargo tree -e normal,features -p capi` names no `realtime-policy`, and two clean builds of
   `cargo build --locked --release -p capi` give identical digests: `libcapi.a` `3431f546...a3394`, `libcapi.so`
   `0213cd82...8ca6d`. (The A1 digests differ because #1422 changed `crates/engine/src/realtime/plan.rs` since.)
   The `artifact-identity` job is CI-only.
7. `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `check-workspace-policy.sh`, `check-bench-policy.sh`, `check-realtime-policy.sh`, `test-realtime-policy.sh`,
   `check-ci-path-routing.py`, `test-ci-path-routing.py`, `check-script-reachability.py`: all exit 0. (A first
   draft of `whole_plan.rs` held the text of an unsafe fn in a test row and the awk gate refused it; the rows are
   built from `UNSAFE` now.)

**Test value (the one-sentence answer, per new test).**
- `a_marker_inside_a_node_is_refused` / `either_marker_alone_can_split_a_node`: red if a marker inside a match,
  call, struct, let, if/else, method chain, parameter list or impl header is accepted, or only one marker of a
  region is judged.
- `a_marker_inside_an_attribute_is_refused`: red if an attribute that precedes its item is not whole.
- `a_marker_between_attributes_and_their_item_is_a_boundary`: red if the attribute rule skips fewer than all
  outer attributes (the real rack marker follows six doc lines).
- `a_marker_between_the_children_of_a_container_is_a_boundary`: red if `mod`, `impl`, `trait`, `extern` or a
  block is not a container.
- `a_region_may_span_the_end_of_an_impl`: red if any change of container is refused (the real effect-contract and
  transient-shaper regions).
- `a_region_inside_a_macro_body_is_tokens_and_passes`: red if a macro's groups are not containers or a region
  inside one is refused (the real gate-expander region).
- `a_region_that_crosses_a_macro_token_tree_is_refused`: red if a region from a macro group into syntax, or
  between two groups, is accepted, on either side, at any depth.
- `a_boundary_finding_names_the_marker_line`: red if a finding names the wrong line or text.
- `forbidden::*`: the ported table is red if a forbidden spelling in a marked function is missed in any of the
  nine places; `a_spelling_the_line_scan_missed_is_a_finding` is red if the predicate falls back to line text, skips
  macro trees, drops the turbofish (#948) or `async`; `a_forbidden_word_in_a_comment_or_a_literal_passes` is red
  if literals are scanned as text; `a_longer_identifier_still_matches` is red if word boundaries are added;
  `adjacent_words_do_not_join_into_a_spelling` is red if the rendering joins words; `only_the_tokens_between_the_markers_are_read`
  is red if the scan covers the file or one region; `findings_are_one_per_line` and
  `a_finding_names_the_line_where_the_spelling_starts` are red if a finding is per match or on the wrong line.
- `whole_plan::*`: red if a form of a whole-plan borrow is missed in the tree or a macro, an item after the test
  module is not scanned, `cfg(any(test, ..))` or another module name is skipped, a call, index, path, field or
  `&raw` form is flagged, or the render-entry and missing-file checks go.

**Open items.**
- The region-nodes type is not built (nothing reads it in A2); B1a extends `boundary.rs`'s layout.
- Run time: D1 and D2 add about 0.28 s on the real tree; children could be computed only for containers that hold
  a marker, and the three flattenings of a marked file could be one. Not done (a diminishing return, 0.04 s on the
  synthetic file).
- D6 steps 3-5 (sync of #948's body, the comment, the close, and removing its spec at the next batch) wait for
  the batch push; the verdict path in #948's Evidence is the expected one.
- A string in a macro's tokens that names a forbidden spelling is a literal and passes, as in a function body;
  the same holds for text built at run time. The predicate reads spellings, as the regex did.
