# Check realtime regions with a Rust syntax-tree tool: the drain-bound rule's outer loops, attributes and constructors

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice B1b of nine (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b, B2a (#1442), C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what each
holds):

- B1a ported the drain rule's mapping, scope resolver, pops, counts, reads and innermost bound,
  and listed the ported cases that need a later rule.
- **B1b (this issue)** ports the rest of `main`'s drain rule: finite outer loops, attributes and
  items, and constructors. It commits B1a's deferred cases, a synthetic twin of each real drain
  shape, and the marker on `drain_controls`.
- B2a then refuses pops in closures, macros and function values.

No production code changes, except D5's two marker comments, which replace two blank lines.

## Problem (verified on `origin/main` at `6d28a80ec` and on `codex/d15-stream-b` at `b8392df66`)

Lines of `scripts/check-realtime-policy.sh` are `main`'s. On `b8392df66`, which lands first, every
line from `:70` on is two lower: the floors are `:75-76` and the floor comment ends at `:74`.
Lines of `scripts/test-realtime-policy.sh` are `b8392df66`'s: the pad loop is `:445-454` and the
three floor messages are `:1408`, `:1410` and `:1414`.

- **B1a leaves outer loops unjudged** (B1a-D9), and does not port `main`'s attribute, item and
  constructor rows. The awk gate covers them until C2.
- **The real outer-loop drains** (B1a, Problem): `drain_fader_controls` and
  `drain_matrix_controls` loop `for (lane, control) in controls.iter_mut().enumerate()` and take
  `let Some(control) = control.as_mut() else { continue; };`
  (`crates/builtins-compiler/src/lib.rs:1068-1071`, `:1112-1115`).
- **An unmarked real drain.** `BuiltinBankProcessor::drain_controls`
  (`crates/builtins-compiler/src/lib.rs:460-506`) is the body of `begin_block` (`:527-530`). Its
  shapes:
  - outer loop `:468`;
  - `let Some(control) = control.as_mut() else { continue; };` at `:469-471`;
  - count `:472`, bound `:473`, pop `:474`.

  It sits outside every marked region. The file's regions are `:532-595`, `:1063-1105` and
  `:1107-1135`.
- Stream B's cell slices (#1312, #1346, #1347, #1345) may remove these drains first (B1a,
  Problem). Every real-tree claim is re-measured.

## Decisions

- **D1. Every other loop around the pop is finite** (`:105-112`, `:278-318`). A loop around the
  innermost bounding loop (B1a-D8) is a bounded loop as in B1a-D8, or one of these:
  - `for P in 0..<bound>`, with `<bound>` an integer literal, or an identifier followed by
    `.field` or `::segment` steps;
  - `&<fields>`, `&mut <fields>`, `<fields>.iter()` or `<fields>.iter_mut()`, where `<fields>` is
    an identifier followed by `.field` steps;
  - a slice or array parameter of the enclosing `fn` that nothing names before the loop, and that
    no `static`, `const` or `use` in the region names;
  - any of these, optionally followed by `.enumerate()` and by one or more
    `.zip(<literal | fields | &fields | &mut fields>)` steps (the D8b pass loop has two);
  - the closure of `core::array::from_fn` or `std::array::from_fn`.

  `while` and `loop` around a bounded drain are refused. The loop and the slice parameter's `fn`
  signature must lie in the pop's region (B1a-D1). This row is `main`'s rule as it stands. The
  tool's `<bound>` is stricter than the awk's `path`, which also accepts zero-argument method
  calls and bracket indexes (`0..self.controls.len()`, `0..a[i]`). Both are refused: they run code
  the tool cannot see. This over-refusal is accepted. Widening it needs a ruling. (root Amendment 1,
  SP-4; test value: red if the outer bound grammar accepts a method call or an index without a
  ruling.) #1418 (Amendment 1) adds "and selects a different queue" to it.
  - **In a B1a-D8b per-queue form,** the pass loop and every loop between it and the set loop
    must be one of these finite forms. `for _ in 0..passes`, with `passes` the `1 + Σ` sum of the
    counters, is the `0..<bound>` form. The loops that enclose the set loop are the drain's outer
    loops, judged as above.
- **D2. Attributes and items** (`:100-102`, `:214-217`, `:263-265`).
  - A count binding with a `cfg`-family attribute (`cfg`, `cfg_attr`, or a path ending in either)
    is refused.
  - An item of any kind in the region that declares `<count>`, its alias source or the slice
    parameter as its name (`const`, `static [mut]`, `struct`, `enum`, `union`, `fn`, `type`,
    `trait`, `mod`), or a `use` that can name it (a glob `use` included), is refused, wherever it
    sits in the block. A unit struct with comparison and `SubAssign` impls redirects an alias as a
    `const` does. (root Amendment 1, NEW-1)
- **D3. Constructors** (`:115-117`, `:290-291`, `:370-371`, `:384`).
  - `from_fn`, `repeat_with` and `successors`, path-qualified or bare, are refused anywhere in a
    region. `core::array::from_fn` and `std::array::from_fn` are the exceptions; `array::from_fn`
    after `use core::array` is refused, as today.
  - A pop directly in an `array::from_fn` closure is refused.
- **D4. Limits that stay** (stated in the drain rule's module documentation, which replaces
  `:79-141`):
  - recursion;
  - a pop in a helper function (`begin_block` calling `drain_controls` is one);
  - a loop in a caller, outside the region (a loop that encloses the region inside the same
    function is refused by B1a-D1, not a limit);
  - a macro defined elsewhere that expands to a loop.

  B2a closes the closure, macro and function-value limits. B2b-1 closes a pop outside every
  region, and B2b-2 the helper limit inside marked regions.
- **D5. Mark `drain_controls`, if it still pops.**
  - At implementation time, if `BuiltinBankProcessor::drain_controls` still calls `try_pop`, put
    it in its own region, with each marker replacing a blank line, so that no line of the file
    moves:
    - `// REALTIME_POLICY_BEGIN` replaces the blank line before `impl BuiltinBankProcessor {`
      (`:451`; the `impl` is `:452-507`);
    - `// REALTIME_POLICY_END` replaces the blank line after the `impl`'s closing brace (`:508`).

    The `impl` holds only `drain_controls`, so the region is that function. Both markers sit at
    a child boundary of the file (A2-D1). The second review measured this placement on the awk
    gate at `6b9067ede`: `ok (90 marked regions in 25 files)`.
  - **If either blank line is gone** at implementation time, do not insert a line. Skip this
    decision, record it in the Evidence, and B2b-1 places the markers in its marker commit (the
    one commit that root's ruling lets move lines).
  - If #1346 has removed the ring, skip this decision and record that in the Evidence.
  - If the region fails any rule, stop and report the rule and the line. Do not change the
    function.
  - Raise the region floor by one in both gates: `Policy::workspace()` and the awk's floor line
    (`:76` on `b8392df66`), as the awk's floor comment asks of every slice that adds a region.
    The awk self-test's base tree sits exactly on the floors, so raise it with them, as
    `98a2d6bfc` did for #1314: one more region in the pad loop and its comment (re-measured: on
    `b8392df66`, `pad_1.rs` 14 → 15, unless #1345 has raised the pad first) and the region-floor
    message at `:1414` (one more, re-measured; `ninety-three` → `ninety-four` on `b8392df66`). The file floor
    and its two messages (`:1408`, `:1410`) do not change. Without this, `bash
    scripts/test-realtime-policy.sh` exits 1 with "expected at least ninety-three marked realtime
    regions" (fourth review, MAJOR-4, measured on `b8392df66`).
- **D6. Synthetic twins.** Every real drain shape in B1a's Problem and this Problem has a twin in
  the base tree, so each rule stays tested after the cell slices remove the real drains. The base
  tree already holds `drain_fader_records`, `stage_records` and the `plan_exchange` pops. Add any
  shape that is missing:
  - the `drain_controls` shape: a `let Self { .. } = self;` destructure, an outer loop over
    `controls.iter_mut().enumerate()`, and `let .. else` with `.as_mut()`;
  - the route `drain` shape over `self.control.consumer`.
- **D7. Over-refusals that stay** (`:135-141`):
  - a count used other than as the left operand of a comparison: `0 == available`,
    `apply(available)`, `available as u32`;
  - a cap other than `.min(..)`: `cmp::min`, `.clamp(..)`;
  - an outer range other than `0..<bound>`: `1..n`, `0..=n`, `.rev()`, `.take(n)`;
  - a bare local iterated by value;
  - a pop once per pass of a collection loop, with no count.
- **D8. Cases.**
  - Commit every case on B1a's deferred list (B1a-D10), each expecting {drain class}. If one still
    passes, stop and report it.
  - New: a region opened inside a loop's body, around an otherwise counted drain (refused, B1a-D1
    with D1's region test); the D6 twins (accepted).
  - root Amendment 1 adds the cases named in D2 (`unit-struct-shadows-the-alias-source`), D9
    (`region-ends-in-a-deeper-block`, and the rewritten `boundary-spans-two-blocks` and
    `boundary-only-end-splits`) and D10 (four cases), each with its mutation (gate 4).

- **D9. A region ends where it can hold the rest of its block** (amends A2-D1, #1439, which is
  closed; root Amendment 1, SP-2).
  - END's innermost container must be BEGIN's innermost container or one that encloses it.
  - A region whose END sits in a container that does not enclose BEGIN's is refused (boundary
    class), whatever it holds. It would end inside a block that continues past END, and an item
    after END would then name a binding of the region for the whole block unseen.
  - BEGIN inside an `impl` with END after the `impl` (`live.rs:231`,
    `transient-shaper/src/lib.rs:613`) stays accepted.
  - The macro-token-tree refusal is unchanged.
  - The case `boundary-spans-two-blocks` becomes refused, and `boundary-only-end-splits` expects
    both findings (a superseded case is rewritten in the same PR).
  - The awk gate keeps this hole until C2. Both gates run, so the tool's refusal covers it.
  - New case `region-ends-in-a-deeper-block` (the first verdict's probe, refused). Test value: red
    if a region may end inside a block it does not wholly hold, which lets an item after END
    shadow the count.
- **D10. Macros and attributes that can declare an item** (closes a limit the first verdict found;
  root Amendment 1, SP-3).
  - From the first binding the drain rule reads for a pop (the count, its alias source, or the
    start of the slice parameter's `fn` body) to the end of the innermost bounding loop, these
    refuse the drain:
    - every macro invocation in statement position (`name!(..);`, `name![..];`, `name! {..}`) or
      item position;
    - every item that carries an attribute other than `cfg`, `allow`, `expect`, `warn`, `deny`,
      `forbid` or `doc`.
  - A `macro_rules!` item name is not hygienic, and a derive or attribute macro emits items beside
    its item: either can declare a `const`, `static` or unit struct that shadows the binding for
    the whole block.
  - `debug_assert!` is exempt while B2a-D2 trusts it (no file under the roots shadows it).
    Untrusted, it is refused like any other macro.
  - Macros in expression position are not in this rule: they cannot declare an item in the
    enclosing block. (A trailing `m!()` with no semicolon is an expression and cannot expand to an
    item.)
  - The #1440 D8b rule (root Amendment 1) is unchanged.
  - New cases: `item-macro-shadows-the-count` (`shadow!();`, refused; test value: red if a
    statement macro in the drain's span is not refused);
    `braced-item-macro-shadows-the-slice-parameter` (`shadow! {}` against a slice parameter,
    refused; red if braced statement macros, or the span from a slice parameter's `fn` body, are
    missed); `derive-on-an-item-in-the-drain` (refused; red if an item that carries a derive or
    attribute macro is not refused); `untrusted-debug-assert-in-the-loop` (`macro_rules!
    debug_assert` in the file plus `debug_assert!(true);` in a counted loop, refused; red if the
    `debug_assert!` exemption ignores B2a-D2's trust state).
  - Production, measured by a grep of the five real drains and `poll_meters`: the only statement
    macro in a span is `debug_assert!` at `crates/graph/src/runtime.rs:903`; `live.rs:376`
    `matches!` is in expression position; there are no items in any drain span. The implementer
    re-measures with the tool.

## Authorized paths

- `tools/realtime-policy/**`
- `crates/builtins-compiler/src/lib.rs`: D5's two marker comments only. This is a cross-stream
  exception, because stream A owns `crates/builtins*`. STREAMS' hot-file row moves from
  "J #1418" to "J tool slice B1b" (STREAMS).
- `scripts/check-realtime-policy.sh` (the region floor line only, `:76` on `b8392df66`) and
  `scripts/test-realtime-policy.sh` (the pad loop, its comment and the region-floor message only, D5), until C2
  deletes both. STREAMS' standing exception names both.
- This spec

## Non-goals

- B2a's refusals and the probe corpus; B2b-1's and B2b-2's rules and markers.
- #1418's queue-selection rule and #1426's receiver rule.
- Any change to a drain in production code. If a ported rule refuses a real drain, stop and
  report it.

## Hazards

- **Re-measure at implementation time.** The region and file counts, which real drains still pop,
  and whether D5 applies. Each later slice re-measures the floors (the realtime-policy floors row
  in STREAMS).
- **Both gates run until C2.** D5's region must pass the awk gate too.
- **`crates/builtins-compiler/src/lib.rs` is a hot file.** Find `impl BuiltinBankProcessor` and
  `fn drain_controls` by name, not by line.
- **Over-refusal is acceptable, a false pass is not.**

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints
   `realtime policy: ok (R marked regions in F files)`. R is B1a's count plus one if D5 applies.
   `bash scripts/check-realtime-policy.sh` passes on the same tree.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every earlier case and
   every D8 case.
3. **Parity (PR evidence).** As B1a's gate 3, over all 57 ported cases: after this slice the
   verdicts agree on every ported case, except B1a-D10's two class changes.
4. **Mutations (PR evidence).** Apply each alone. Each turns red the named case, and the real
   tree still passes:
   - accept a `cfg` binding: `count_under_cfg`;
   - drop the `static`/`const`/`use` check: `count_shadowed_by_block_static` and
     `outer_over_slice_parameter_shadowed_by_block_const`;
   - accept any `for` as an outer loop: `outer_repeat` and `paren_open_range_outer`;
   - read an enclosing loop outside the region as an outer loop (B1a-D1): the region-in-a-loop
     case passes;
   - drop the constructor check: `from_fn` and `repeat_with`;
   - any mutation that B1a moved here (B1a gate 4);
   - (root Amendment 1) accept END in any container: `region-ends-in-a-deeper-block`;
   - (root Amendment 1) require the same container: `a_region_may_span_the_end_of_an_impl`, and
     the real tree then fails at `live.rs:231` and `transient-shaper/src/lib.rs:613`;
   - (root Amendment 1) restrict the item scan to `static`, `const` and `use`:
     `unit-struct-shadows-the-alias-source`;
   - (root Amendment 1) allow statement macros: `item-macro-shadows-the-count` and
     `braced-item-macro-shadows-the-slice-parameter`;
   - (root Amendment 1) always exempt `debug_assert!`: `untrusted-debug-assert-in-the-loop`;
   - (root Amendment 1) refuse `debug_assert!` too: the base route twin, and the real tree then
     fails at `crates/graph/src/runtime.rs:903`.
5. **Run time (PR evidence).** As B1a's gate 5.
6. **Shipped artifacts unchanged.** D5's markers replace blank lines, so no line moves. The
   `artifact-identity` job reports the worklet module UNCHANGED. PR evidence, built as A1's gate 6
   builds them (parent twice first, then the change, in one checkout):
   - the sha256 of `libcapi.so` is identical;
   - `libcapi.a` holds the same members in the same order;
   - each member is byte-identical after `objcopy --remove-section=.llvmbc`.

   `.llvmbc` is rustc's embedded bitcode. It carries each source file's checksum, so any byte
   change to a source file changes it, and no linker copies it into a shipped image. Any other
   difference fails. There is no fallback: if a marker would move a line, D5 is skipped and B2b-1
   places it. (root Amendment 1, SP-1)
7. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `bash scripts/check-realtime-policy.sh`, `bash scripts/test-realtime-policy.sh` and
   `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The deferred ported cases keep their claims: each is red if the outer-loop, attribute, item or
  constructor row misses the form the case holds.
- The region-in-a-loop case is red if an enclosing loop outside the region is read as bounded.
- The D6 twins are red if the port refuses a real drain shape. They keep that defence after the
  cell slices remove the real drains.

## Evidence

- Gates 1-7 output. Gate 3's table. Gate 4's runs.
- Which real drains still pop at implementation time, and whether D5 applied.

## Dependencies

- After (same stream): B1a.
- After (other streams): stream B batch 1 (#1309, #1343, #1314, #1311, #1348), through A1.
- Not ordered against #1312, #1346, #1347 and #1345 (root's ruling, 2026-10-05).
- Before: B2a.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day.

### Root Amendment 1 (2026-10-11): the first verdict's spec problems

Authority: root decisions under the owner's delegation, after the attempt-1 verdict PASS
(`/home/bl/misofm/submix-verdicts/1441-attempt1.md`: SP-1 to SP-4) and an adversarial review of
root's drafts (ACCEPT WITH CHANGES). Root adopted the review's texts verbatim. Each is placed where
it belongs and marked "(root Amendment 1)":

- **SP-1** Gate 6 compares `libcapi.a` members after removing `.llvmbc`. Only `.llvmbc` differs
  between the parent and D5's markers; the `.so` and the worklet module are identical.
- **SP-2** D9 (END's container encloses BEGIN's), not "same block", because that would refuse
  `live.rs:231` and `transient-shaper/src/lib.rs:613`.
- **NEW-1** D2's item scan covers every item kind, because a unit struct with comparison and
  `SubAssign` impls redirects an alias source as a `const` does. It lands before #1446 deletes the
  awk.
- **SP-3** D10 (statement and item macros, and attributes that emit items).
- **SP-4** D1 keeps the refusal of `0..x.len()` and `0..a[i]`.
- Attempt 2 also closes the first verdict's MINOR-1 (document the `&mut` iterator-in-field or
  local limit in the tool's documentation), MINOR-2 (state the remaining limits) and NIT-1 (gate 6
  evidence: only `.llvmbc` differs).

## Attempt record (attempt 1, J batch branch `codex/d15-stream-j3`, base `1e03edf19`)

Commits: `993b85745` (D5 and both floors), `10f867f65` (rules, cases, twins), and three small follow-ups. Edits:
`tools/realtime-policy/**`, `crates/builtins-compiler/src/lib.rs` (two comment lines),
`scripts/check-realtime-policy.sh` (floor line and comment), `scripts/test-realtime-policy.sh` (pad loop, its
comment, the region-floor message) and this spec. No `Cargo.toml` or `Cargo.lock` change.

**What is where.**
- `src/drain/outer.rs` (D1): `finite` judges every loop of the pop's path between the region's top node and the
  innermost bounding loop: a B1a-D8 bounded loop (`count::bounds`), `for P in 0..<bound>`, the collection forms
  with any `.enumerate()` and `.zip(..)` steps, a slice or array parameter, or the `array::from_fn` closure.
- `src/drain/items.rs` (D2, D3): two token scans over the region's leaves (`names_item`, `constructor_lines`). They
  read macro token trees and `macro_rules!` bodies too.
- `src/drain/count.rs`: `under_cfg` and the item check, in `counted` and in the alias branch.
- `src/drain/tree.rs`: `is_loop_at` and `array_from_fn_call` (the closure argument of `core::array::from_fn` or
  `std::array::from_fn` is a loop of the tree rule); `src/drain/scope.rs`: `BindingKind::Param { ty }`.
- `src/drain/mod.rs`: `check` scans each region for constructors whether or not a pop stands in it; `bounded`
  judges the outer loops; the module documentation states D4's limits and D7's over-refusals.
- `src/boundary.rs`: `leaves_between`, `marker_start`, `RegionNodes::bounds`. `src/policy.rs`: region floor 94.

**Real tree (re-measured).** `drain_controls` is at `crates/builtins-compiler/src/lib.rs:460-515`; the blank lines
were `:459` and `:516` (the spec's `:451`/`:508` are `main`'s). It still pops (`:482`) and now has a `carry` argument
and a `let Self { .. } = self;` destructure. D5 applied: the two markers replace the two blank lines, so no line
moves. `cargo run --locked --release -q -p realtime-policy` and `bash scripts/check-realtime-policy.sh` both print
`realtime policy: ok (94 marked regions in 26 files)`. The cell slices have not landed on this branch: the marked
pops are `lib.rs:482`, `:1084` (fader), `:1128` (matrix), `live.rs:366`, `runtime.rs:900`.
Floors: regions 93 to 94 in `Policy::workspace()` and the awk floor line; files stay 26. The awk self-test's pad file
`pad_1.rs` 14 to 15 regions and its message `ninety-three` to `ninety-four`. The tool's own base tree needed the same
region (`pad_1.rs.txt` 14 to 15, `BASE_REGIONS` 94), because its `run.rs` cases run `Policy::workspace()` on the
base tree.

**Deviations and notes.**
1. **`<bound>` grammar.** D1 states the integer literal or an identifier with `.field` and `::segment` steps (the
   same `cap` as D6), and the tool follows it. The awk's `path` also accepts zero-argument method calls and a
   bracket index (`0..self.controls.len()`, `0..a[i]`), so the tool refuses `0..x.len()` where the awk passes. No
   real region needs it and no ported case holds it. Widening it is one change in `shapes::cap`'s caller
   (`outer::range_to_a_bound`). Open item for root: decide whether D1's `main`-parity wording means the awk's path.
2. **The tree rule treats only the `array::from_fn` closure as a loop.** Every other `from_fn`, `repeat_with` and
   `successors` is refused by its token. With both mechanisms on every `from_fn`, no mutation could turn the
   ported `from_fn` case red alone. Now the token scan owns `from_fn`/`bare_from_fn`/`repeat_with` (gate 4,
   constructor mutation), and the tree rule owns `array_from_fn_pop`.
3. **The item scan is token-based over the whole region** (`static [mut] name`, `const name`, a `use` whose tokens
   up to `;` hold the name or `*`), as the awk's `item()`. The resolver is unchanged; `scope.rs` documents that
   rules which read a binding ask `names_item`. The scan covers the count, its alias source and a slice
   parameter. The per-queue roots are not asked (D2 names `<count>` only).
4. **A `cfg`-family attribute** means the last path segment is `cfg` or `cfg_attr` (the spec's words); the awk
   matched any identifier starting with `cfg`. It is checked on the entry binding and on the alias source.
5. **Slice parameter:** the parameter is a plain name (`name` or `mut name`) of type `&[T]`, `&mut [T]`, `&[T; N]`
   (any lifetime) or `[T; N]`; its `fn` signature and the loop lie in the region (`region.holds(signature.core,
   loop.end)`); and `words_in(name, body_open, loop.full) == 0`.
6. **Base-tree twins (D6).** The base already held `drain_fader_records`, `stage_records`, the `plan_exchange`
   pops, and (A1) `drain_lane_pairs` (an `array::from_fn` drain) and `drain_two_lanes`. Added: the `drain_controls`
   twin (`let Self { .. } = self`, `controls.iter_mut().enumerate()`, `let .. else` with `.as_mut()`) in the first
   `builtins-compiler` region, and the route `drain` twin (`self.control.consumer`, a `debug_assert!` in the loop)
   in `graph/runtime.rs`'s first region. No region was added.
7. **Cases.** The 14 deferred cases are rows of `ported.rs` (kebab names) and all expect {drain class}. The
   region-in-a-loop case existed (`loop-around-the-region`, `loop`); B1b adds `finite-loop-around-the-region`
   (`for _ in 0..4` outside the region), which is the case that tells the mutation "read the enclosing loop as an
   outer loop" apart from the unconditional refusal. 131 tests (121 before).
8. **`libcapi.a` digest.** The parent (`1e03edf19`) builds `libcapi.a` `3431f546ec53477d...` and `libcapi.so`
   `0213cd820e2715de...`; with D5's markers the `.so` is identical and the `.a` is `2aa32d808165aef3...`. Three
   objects of the archive differ (`builtins_compiler`, `graph_compiler`, `host_core`); their code and data sections are
   identical, and only `.debug*` and `.llvmbc` (embedded bitcode) differ, which carry the changed source text of
   `lib.rs`. A comment in place of a blank line cannot leave that unchanged. The worklet module is byte-identical:
   `scripts/build-web-audioworklet.sh --module-only` at the parent and at the change both give `9784d3509dba6bfd...`
   (the CI `artifact-identity` job is not run here). Open item for root: gate 6's "libcapi digests identical" holds
   for the `.so` and for the code of the `.a`, not for the `.a` bytes.
9. Not changed: B1a's per-queue window, terminal-call and macro-and-attribute rules; a doc sentence in
   `per_queue/mod.rs` says the loops around the form are judged by `outer`. STREAMS' hot-file row and its
   "ninety-three" floor message (`docs/handoffs/.../STREAMS.md`) are root's files and are not edited.

**Gates.**
1. Real tree: both gates print `ok (94 marked regions in 26 files)` (above).
2. `cargo test --locked -p realtime-policy`: 131 passed.
3. Parity (the B1a method: an instrumented copy of `scripts/test-realtime-policy.sh` outside the repository runs the
   awk gate and the release binary on every `expect_failure` tree, 96): awk fails on all 96. The tool fails on all
   96: 55 with the drain class (B1a's 41 plus the 14 deferred cases) and 41 without it (A1 and A2 cases,
   `open-tail-last-file` and `region-ends-in-comment`, the two D10 class changes). No tree passes the tool. The
   verdicts agree on all 57 ported cases.
4. Mutations: below.
5. Run time, release binary: real tree 1.05 s, 1.05 s, 1.06 s (B1a: 1.03-1.04 s; the awk gate 0.64 s). The base
   tree plus one marked file of 3,006 lines (`for lane in 0..cfg.lanes` around 300 counted drains): 0.07 s with the
   same count name 300 times and 0.07 s with 300 distinct names. Not tuned.
6. See deviation 8. Outside the authorized paths only the two comment lines of `lib.rs` changed.
7. `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `check-realtime-policy.sh`, `test-realtime-policy.sh` (with the raised pad), `check-workspace-policy.sh` and
   `check-bench-policy.sh` all exit 0.

**Mutations** (each applied alone, `cargo test` and the real tree after each, then reverted by `git checkout`;
131 green and the real tree ok after every revert). "red" lists tests and rows; the real tree still passed under
every one, except where marked *. The spec's named cases are first.
- Accept a `cfg` binding: `count-under-cfg` (ported) and the four `count-under-*`/`alias-source-under-cfg` rows.
- Drop the `static`/`const`/`use` check: `count-shadowed-by-block-static` and
  `outer-over-slice-parameter-shadowed-by-block-const` (ported), with five count rows.
- Accept any `for` as an outer loop: `outer-repeat` and `paren-open-range-outer` (ported), with the rest of the
  outer ported cases and nine refusal rows.
- Read an enclosing loop outside the region as an outer loop: `finite-loop-around-the-region` (alone;
  `loop-around-the-region` stays red because a `loop` is not finite).
- Drop the constructor check: `from-fn`, `bare-from-fn` and `repeat-with` (ported) and nine rows.
- B1a mutations moved here: none (B1a's gate 4 names no deferred case).
- New rules, one red test each unless noted: raw prefix not stripped (`raw-identifier-from-fn`); longer path ending
  in `array::from_fn` accepted; any root for `array::from_fn`; constructors scanned over the whole file
  (`constructors-outside-every-region`); constructors checked only in a region with a pop (nine rows); generic
  arguments ignored by the outer exemption (`outer-array-from-fn-with-generic-arguments`); the `array::from_fn`
  closure not a loop (`array-from-fn-pop`, `std-array-from-fn-pop`, `array-from-fn-block-pop`, the generic-argument
  row); never a finite outer (the base tree and the three `outer-*-array-from-fn` rows); `loop` accepted as an
  outer (`outer-loop`, the per-queue `loop` pass count and `loop` around the form, `bounded-drain-inside-outer-loop`);
  `while` accepted (`outer-while-condition`, `-while-let`, `wrapped-outer-while`); range start, inclusive range and
  open end (one row each); any adapter after the base (`outer-reversed`, `-taken`, `-cycled`,
  `outer-zip-then-cycle`); slice parameter type unchecked (`outer-vec-parameter-by-value`,
  `outer-over-iterator-parameter`); a destructured parameter takes the type
  (`outer-slice-parameter-with-a-pattern`); signature outside the region
  (`slice-parameter-outside-the-region`); mention before the loop unchecked
  (`outer-slice-parameter-rebound-by-a-macro`; `outer-over-rebound-slice-parameter` stays red through the resolver
  alone); `use` arm dropped; `static mut` not skipped; any `static`/`const` refuses; any `use` refuses; `cfg` only
  (`count-under-cfg-attr`); first path segment; any attribute (the two `*-under-allow` rows); first attribute only;
  alias source unchecked; only the nearest outer loop judged (`outer-finite-loop-inside-an-unbounded-one`, the
  `never-ends` row, `per_queue_inside_an_unbounded_outer_loop`); outer loops not judged (B1a's state: the three
  per-queue cases, 16 rows and 8 ported cases); bounded `for` or `while` not an outer
  (`outer-counted-for-header`, `outer-counted-while`); zip reference or literal arm dropped (`outer-zip-chain`);
  impl method body not found (`outer-slice-parameter-of-a-method`).
- D6 twins: receiver of an entry count limited to one field: the route twin (`graph/runtime.rs`) fails in the base
  tree, and the real `runtime.rs:900`*; outer collection root must be a parameter: the `drain_controls` twin fails
  in the base tree, and the real `lib.rs:482`*.

**Test value (the one-sentence answer, per new test).**
- `the_finite_outer_loops_pass`: red if any finite form of D1 (bound, reference, `iter`, adapters, slice or array
  parameter, bounded loop, `array::from_fn`) is refused, which no earlier case or base-tree shape pins.
- `the_outer_loops_that_may_not_end_are_refused`: red if a loop that can run forever, or that D1 and D7 cannot
  prove finite, is accepted as an outer loop.
- `a_slice_parameter_of_a_function_outside_the_region_is_not_read`: red if a signature outside the region is read.
- `an_item_that_can_name_the_count_refuses_the_drain`: red if a `const`, `static mut` or `use` (glob, name, nested)
  that can shadow the count or the slice parameter is not seen, or if an unrelated item refuses.
- `a_count_binding_that_may_not_be_compiled_is_refused`: red if `cfg_attr`, a `cfg` path, a `cfg` after another
  attribute or a `cfg` alias source is missed, or if any attribute refuses.
- `a_constructor_that_loops_is_refused_anywhere_in_a_region`: red if a constructor with no pop, a raw spelling, a
  longer or foreign path or a macro token escapes, or if the exempt `array::from_fn` paths, or a constructor outside
  every region, are refused.
- `a_pop_directly_in_an_array_from_fn_closure_is_refused`: red if the closure of `std::array::from_fn` is no loop.
- `finite-loop-around-the-region` (in `a_loop_that_encloses_the_region_is_refused`): red if an enclosing loop outside
  the region is judged as an outer loop.
- `per_queue_inside_a_finite_outer_loop`, `per_queue_inside_an_unbounded_outer_loop`,
  `per_queue_pass_count_loop_may_not_end`: red if the loops around a D8b form, the pass count loop included, are
  refused when finite or left unjudged when not.
- The D6 twins (base tree): red if the port refuses the `drain_controls` or route shape.

**Open items.**
- Root: deviation 1 (`.len()` and index bounds) and deviation 8 (`libcapi.a` bytes).
- STREAMS: the hot-file row to "J tool slice B1b", and the floor message text, are root's edits.
- B2a: the closure and macro transparency and its two superseded B1a tests, as B1a's record says.

## Attempt record (attempt 2 placement, 2026-10-11)

Coordinator's choice, recorded with root Amendment 1: D10 (SP-3) is implemented in this issue's
attempt 2, not in #1442. B2a-D2's trust state (`src/drain/shadow.rs`, `Facts::of(file,
trust_debug_assert)`) already exists on the branch (`bfed03af6`), so D10 needs no forward reference.
