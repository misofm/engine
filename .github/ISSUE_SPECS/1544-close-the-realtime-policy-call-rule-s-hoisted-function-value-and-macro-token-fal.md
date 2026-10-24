# Close the realtime-policy call rule's hoisted-function-value and macro-token false passes

Stream J successor of #1444 (B2b-2, decision 15 D15-0, D15-2). The call rule shipped in #1444 passes
two groups of direct, written forms in a marked region. Neither group is in a listed limit of the
#1438 threat model. The code follows the #1444 text, so these are gaps in the spec. This issue widens
the rule. Sprint mode: short note, no mutation runs; the gates are the cases below.

## Problem (verified at `e9255f196`)

**MAJOR-1: a hoisted function value outside a `let` initializer.** `tools/realtime-policy/src/calls.rs:238-248`
makes a local hot only from its `let` initializer. At `:314-336` a single-segment callee that is not a
local is matched only against the set of function names. These pass, with the helper marked in its
own region and the loop in a region:
- Assignment: `let mut next: fn(&mut Lane) -> Option<u32> = Lane::quiet;` and `next = Lane::next_record;`
  above BEGIN, then `while let Some(r) = next(self)` in the region.
- Deferred `let`: `let next; next = Lane::next_record;` above BEGIN, then the same loop.
- Item: `const NEXT: fn(&mut Lane) -> Option<u32> = Lane::next_record;` (or a `static`) outside every
  region, then `while let Some(r) = NEXT(self)` in a marked `impl`.

**MAJOR-2: a loop, closure or path value written inside a macro token tree.** `calls.rs:172-196`
refuses `ident(..)` in a token tree only when the macro call itself stands in a loop or closure that
`syn` sees. It never applies the path-value rule to tokens. These pass in a marked region:
- `assert!({ while self.next_record().is_some() {} true });` (a written loop)
- `assert!((0..4).all(|_| self.next_record().is_some()));` (a written closure)
- `for _ in 0..4 { let n = count!(Self::next_record); }` (a written path value)

## Decisions

- **D1. A local is hot when any assignment to it names a set member.** `x = ..` and `x op= ..` count,
  as does a `let` with no initializer that is assigned later. A hot local is a local that the `let`
  rule already treats as hot, or that a set name or a hot local reaches by assignment. The check is
  flow-insensitive.
- **D2. Items.** Every non-test `const` or `static` whose initializer names a set member is added to
  the set (by its own name). A call to it, as `NEXT(self)`, is then judged by the existing rule.
- **D3. Macro tokens in a region.** Refuse every set name in a macro token tree that does not resolve
  to a local, in or out of a loop. The call class applies when a `( .. )` group follows the name. The
  path-value class applies otherwise. This covers the written loop, the written closure and the
  written path value above without parsing the tree into a loop or closure.
- **D2b. `use` renames (root, 2026-10-24).** `use path::next_record as quiet;` adds `quiet` to the
  set, as D2 does for an item. Items and renames are read token by token in every file (test code
  and macro token trees included), to a fixpoint. In a `const`/`static` initializer, a field name of
  a struct literal (`meters: true`) and a field read (`.meters` with no `( .. )`) do not count:
  without that filter the real tree's `meters` field cascaded through `ALL`, `SPEC` and `_` into
  hundreds of consts and at least 20 false findings.
- **D4. One refused case per form** in `tools/realtime-policy` tests: assignment, deferred `let`,
  `const`, `static`, loop in tokens, closure in tokens, path value in tokens. A case where a local of
  the same name shadows a set name stays accepted. Root: two grouped tests in
  `tests/policy/calls.rs`; the `calls-macro-once` accept row is deleted (D3 supersedes it).

## Measured cost

- MAJOR-2 fix (D3): the reviewer applied it to a scratch export and the real tree stays
  `ok (126 marked regions in 27 files)`.
- MAJOR-1 fix (D1, D2): to be measured; expected 0 new findings. If one appears, it is a real
  hoisted-value pop and root names the restructure (#1444 D3).

## Stated limits (root, 2026-10-24; not implemented)

- A name that a macro builds from parts (`paste!`, `concat_idents!`, a `$f` fragment spliced into a
  call): limit 4 of the #1438 threat model (owner ruling 2026-10-11).
- `let mut it = std::iter::from_fn(|| lane.next_record());` above BEGIN, then `for r in it` in a
  region: #1444 D4 and limit 1. Closing it needs a hot local refused as a value in a region, and
  `let record = self.next_record();` is hot too, so it would over-refuse every real drain result.
- A helper two calls deep, a value that enters through a field or a return, `Drop`/`Deref`: #1444 D4.
- Aliasing through `&mut` (`*r = ..`, `mem::swap`, `mem::replace`): root ruling 2026-10-24, the same
  family as a value through a field or a return.

## Hazards

- Limit 4 of the #1438 threat model says written tokens in macro trees are judged by the rules the
  tool applies to written tokens in that place. D3 makes the tool do that. Forms that a macro builds
  from parts stay outside the limit.
- The reviewer also recorded `let mut it = std::iter::from_fn(|| lane.next_record());` above BEGIN
  then `for r in it` in a region. It is inside limits 1 and D4 (standard-library iterator, closure as
  a parameter). Root decides whether to close it here; this issue does not.
- Over-refusal is acceptable, a false pass is not. A new name collision is resolved by restructure,
  never by an exemption or a rename that dodges the set.

## Authorized paths

`tools/realtime-policy/**` and this spec. No production code changes.

## Objective gates

1. `cargo run --locked --release -q -p realtime-policy` prints the same line as a run on `main`
   before the change (`ok (192 marked regions in 34 files)` at `4ebb72ae0`).
2. `cargo test --locked -p realtime-policy` passes with the D4 cases.
3. `cargo fmt --all -- --check` and `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`.

## Dependencies

Stream J batch 4 (`codex/d15-stream-j4`) on `main`. Attempt budget: 3.

## Attempt record

- Attempt 1 (2026-10-24, branch `codex/batch-b10-1544`). D1: a per-function pre-pass in
  `src/calls.rs` collects every local that a `let`, an assignment, a compound assignment or an
  `x = ..` written in a macro token tree reaches from a set name, to a fixpoint; `bind` marks it hot.
  D2/D2b: `read_aliases` (already run on every file by `src/lib.rs`) now also reads `const`/`static`
  items and `use` renames, so a fact in a file that no rule parses counts; `calls::check` extends the
  set to a fixpoint. This replaces the scope note's walk in `src/unmarked/`, which sees only parsed
  files, so `src/unmarked/**` is unchanged. D3: `tokens()` refuses a set name in a region with no
  loop test. Real tree: `ok (192 marked regions in 34 files)` before and after. Mutants (PR
  evidence): no pre-pass, the `repeats` test back in `tokens()`, no item/rename facts: each turns
  its group test red.
- Attempt 2 (2026-10-24), after verifier FAIL on attempt 1. BLOCKER-1: a hot local called in a token
  tree is now refused anywhere in a region (a loop or closure written in the tokens is not seen).
  MAJOR-1: the D1 pre-pass also binds the patterns of `if let`/`while let`, each `match` arm and a
  `for` from the identifiers of their value. MAJOR-2: recorded as a limit above. MINOR-1: in
  `tokens()`, a field read (`.name` with no `( .. )`) and a struct-literal field name are not values.
  MINOR-2: in an item initializer, a module segment (`name::item`, not `name::<T>`) does not count.
  Rows added: `calls-macro-hot-local`, `calls-hoisted-pattern` (red on attempt 1), and the accept rows
  `calls-macro-field`, `calls-item-field-and-module` (each red when its brake is removed). Real tree:
  `ok (192 marked regions in 34 files)`.
- Post-PASS fold (verifier MINOR): rows `calls-hoisted-match` and `calls-hoisted-for`. A mutant that
  removes both pattern edges turns each red (run once, restored). NIT-1: `tokens()` shares the
  module-segment brake with item initializers. Real tree: `ok (192 marked regions in 34 files)`.
