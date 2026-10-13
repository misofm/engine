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
- **D4. One refused case per form** in `tools/realtime-policy` tests: assignment, deferred `let`,
  `const`, `static`, loop in tokens, closure in tokens, path value in tokens. A case where a local of
  the same name shadows a set name stays accepted.

## Measured cost

- MAJOR-2 fix (D3): the reviewer applied it to a scratch export and the real tree stays
  `ok (126 marked regions in 27 files)`.
- MAJOR-1 fix (D1, D2): to be measured; expected 0 new findings. If one appears, it is a real
  hoisted-value pop and root names the restructure (#1444 D3).

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

1. `cargo run --locked --release -q -p realtime-policy` prints `ok (126 marked regions in 27 files)`.
2. `cargo test --locked -p realtime-policy` passes with the D4 cases.
3. `cargo fmt --all -- --check` and `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`.

## Dependencies

Stream J batch 4 (`codex/d15-stream-j4`) on `main`. Attempt budget: 3.

