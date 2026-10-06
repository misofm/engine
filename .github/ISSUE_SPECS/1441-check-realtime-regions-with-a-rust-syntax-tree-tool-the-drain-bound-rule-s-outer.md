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
  signature must lie in the pop's region (B1a-D1). This row is `main`'s rule as it stands. #1418
  (Amendment 1) adds "and selects a different queue" to it.
  - **In a B1a-D8b per-queue form,** the pass loop and every loop between it and the set loop
    must be one of these finite forms. `for _ in 0..passes`, with `passes` the `1 + Σ` sum of the
    counters, is the `0..<bound>` form. The loops that enclose the set loop are the drain's outer
    loops, judged as above.
- **D2. Attributes and items** (`:100-102`, `:214-217`, `:263-265`).
  - A count binding with a `cfg`-family attribute (`cfg`, `cfg_attr`, or a path ending in either)
    is refused.
  - A `static`, `const` or `use` in the region that can name `<count>` (a glob `use` included) is
    refused, wherever it sits in the block.
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
   - any mutation that B1a moved here (B1a gate 4).
5. **Run time (PR evidence).** As B1a's gate 5.
6. **Shipped artifacts unchanged.** D5's markers replace blank lines, so no line moves. The
   `artifact-identity` job reports the worklet module UNCHANGED, and the `libcapi` digests are
   identical, as A1's gate 6 measures them. There is no fallback: if a marker would move a line,
   D5 is skipped and B2b-1 places it.
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
