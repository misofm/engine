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
