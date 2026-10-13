PASS

# #1441 attempt 1 verdict (B1b: the drain rule's outer loops, attributes and constructors)

Range: `1e03edf19..31bfd0271` on `codex/d15-stream-j3` (993b85745, 10f867f65, 42ceb625a, 9ed790619, 31bfd0271).
Exported with `git archive 31bfd0271` into `/tmp/claude-1002/v1441/tree`, built with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1441/target`. The worktree was not touched.

No BLOCKER and no MAJOR against the attempt. The attempt meets the spec as written. Four
findings are spec problems for root: SP-1 (gate 6) and SP-2 (a D2 false pass) need a ruling
before the issue closes.

## Spec problems for root

- **SP-1 (gate 6 cannot be met as written).** D5 requires two source comments in
  `crates/builtins-compiler/src/lib.rs`. Gate 6 requires the sha256 of `libcapi.a` to stay the same.
  Both cannot be true. I measured this in one checkout, with only the two marker lines reverted:
  - `libcapi.so` is identical: `3609d28b...1e75` with and without the markers.
  - `libcapi.a` differs: `303680f3...811c` at head and `d6d91ec6...4089` at parent.
  - The archive has 387 objects. Three differ: `builtins_compiler`, `graph_compiler` and `host_core`.
    After `objcopy --strip-debug --remove-section=.llvmbc`, all three are byte-identical. A
    per-section dump shows that **only `.llvmbc` differs**. Every `.debug_*` section is identical,
    and the `rawline` line tables are identical.
  - The embedded bitcode holds the source file's checksum. Any byte change in a source file
    changes `.a`, even when no line moves.
  - The worklet module is the same at parent and at head:
    `AudioWorklet module 9784d3509dba6bfd8188b00fc48f9ae4f20020ef9583cbc409cea1d079878d9c`, from
    `build-web-audioworklet.sh --module-only` in both states.

  So the purpose of gate 6 holds: no shipped code or data changed. Its literal text ("libcapi digests
  identical") does not hold for `.a`, and no change that follows D5 can make it hold. B2b-1 (#1443)
  adds markers in the same file and will hit the same problem. Root should amend gate 6, for example:
  "`.so` identical; `.a` objects identical after removing `.llvmbc`".

- **SP-2 (D2 false pass when a region ends inside a block; both gates).** The item scan reads only
  the region's leaves (`items.rs` `names_item`, called on `boundaries.leaves_between(region
  bounds)`). A2-D1 accepts a region whose END marker is inside an inner block. A block `const`,
  `static` or `use` placed after that END marker shadows the count, or the slice parameter, for the
  whole block. Neither gate sees it:
  ```rust
  fn probe(q: &mut Consumer) {
      // REALTIME_POLICY_BEGIN
      let available = q.available_at_entry();
      {
          for _ in 0..available { let _ = q.try_pop(); }
      // REALTIME_POLICY_END
          const available: usize = usize::MAX;
      }
  }
  ```
  Result: tool exit 0 and awk exit 0. I compiled a copy with rustc: the loop runs over the `const`
  (5 passes, not 1). The slice-parameter form with a `const`/`static lanes` after END also passes
  both gates.

  D2 says "in the region" and also "wherever it sits in the block". The two do not agree when the
  block continues past the region. The implementer followed "in the region", which is the awk rule.
  This is not a regression, but it is a false pass in a rule that this slice owns. Possible fixes:
  - scan in full every block on the path from the binding to the loop; or
  - make A2-D1 refuse a region whose two markers are in different containers.

  Root should choose between attempt 2 and a named successor.

- **SP-3 (an item made by a macro shadows the count; both gates; not in D4).** A `macro_rules!`
  defined outside the region expands to `const available: usize = usize::MAX;`. It is invoked as
  `shadow!();` in the loop's block, inside the region. Both gates pass. `macro_rules!` items are
  not hygienic, and I compiled a copy to confirm that the macro's item shadows the `let`. D4 lists
  "a macro defined elsewhere that expands to a loop" but not a macro that expands to an item. B2a
  refuses pops in macros, not item macros. Root should add this to D4's limits, or give it to B2a
  or B2b.

- **SP-4 (deviation 1, `<bound>` grammar).** The tool refuses `0..x.len()` and `0..a[i]`, which the
  awk accepts. I confirmed this: `for _ in 0..self.lanes.len()` gives tool 1 and awk 0. This follows
  D1's explicit grammar, but D1 also says "main's rule as it stands", and main's comment names
  `self.controls.len()`. The tool refuses more than the awk, and the spec permits that. No real
  region and no ported case uses this form. Root decides which reading D1 means. This does not
  block the attempt.

## Findings against the attempt

- **MINOR-1: a known false pass is not documented.** `tools/realtime-policy/src/drain/outer.rs:1-17`
  and the module documentation in `drain/mod.rs` do not carry the awk's documented hole: `&mut` of
  an iterator in a field or a local is infinite and passes (`let mut rounds = 0_u32..; for _ in &mut rounds`).
  I confirmed it passes both gates. C2 deletes the awk and its comment, so this limit would then be
  recorded nowhere. Add one sentence to the outer-loop documentation.
- **MINOR-2: SP-2 and SP-3 are not documented either.** The module documentation states D4's limits
  but not these two shadowing holes. Name them as limits until root rules.
- **NIT-1: gate 6 evidence overstates what differs.** The Attempt record (deviation 8) says that
  "`.debug*` and `.llvmbc`" differ. Only `.llvmbc` differs; every `.debug_*` section is identical.
  The conclusion is the same.
- **NIT-2: the floor comment is outside the spec's authorized lines.** `scripts/check-realtime-policy.sh:74-75`
  gets two new comment lines, but the spec authorizes "the region floor line only". STREAMS (stream J
  owns the script until #1446) and the floor comment's own instruction cover this edit. The
  Attempt record says so openly.

## Checked and correct

- **Gate 1:** `cargo run --locked --release -q -p realtime-policy` and `bash scripts/check-realtime-policy.sh`
  both print `realtime policy: ok (94 marked regions in 26 files)`. The floor is 94 in all of these:
  `Policy::workspace()`, the awk floor line, the awk self-test (`pad_1` 15 and "ninety-four"), the
  tool's base fixture (`pad_1.rs.txt` 15) and `BASE_REGIONS` 94.
  - `markers.rs` and `run.rs` changed with the floor.
  - STREAMS' "ninety-three" text is root's file and was left alone, which is correct.
- **D5:** the two markers replace the blank lines before and after `impl BuiltinBankProcessor`.
  No line moves (the hunks are `-1 +1`). The region passes both gates.
- **Gate 2:** 131 tests pass.
- **Gate 3 (parity, re-run):** I instrumented a copy of `test-realtime-policy.sh` to also run the
  tool on every `expect_failure` tree, with a clean release binary.
  - All 96 trees fail the tool, and the awk self-test passes.
  - Of the 57 drain-class (ported) cases, 55 report the drain class from the tool. The other two
    are the D10 class changes, `drain-open-tail-last-file` and `drain-region-ends-in-comment`.
  - No non-drain case reports the drain class.
- **The 14 deferred cases:** all 14 on B1a's deferred list are rows of `ported.rs`, and each expects
  the drain class.
- **Gate 4 (mutations re-run, each alone, then reverted):**

  | Mutation | Turns red | Real tree |
  |---|---|---|
  | cfg check off | ported `count-under-cfg` and 4 rows | ok |
  | `names_item` returns false | ported `count-shadowed-by-block-static` and `outer-over-slice-parameter-shadowed-by-block-const`, and 5 rows | ok |
  | every `for` is finite | ported `outer-repeat` and `paren-open-range-outer`, and others | ok |
  | an enclosing loop outside the region judged as an outer loop | `finite-loop-around-the-region` only | ok |
  | constructor scan off | ported `from-fn`, `bare-from-fn`, `repeat-with` and 9 rows | ok |
  | `array::from_fn` closure not a loop | ported `array-from-fn-pop`, `std-array-from-fn-pop` and the generic-argument row | ok |
  | D1 off entirely | 16 rows, 8 ported cases, 2 per-queue tests | ok |
  | signature-in-region check off | `slice-parameter-outside-the-region` only | ok |
  | mention-before-loop check off | `outer-slice-parameter-rebound-by-a-macro` only | ok |
  | any `static`/`const` refuses | `unrelated-const-after-the-loop`, `unrelated-static-after-a-slice-loop` | fails |

  Deviation 2 holds: the token scan and the tree rule each have a case that only it turns red.
- **D6 twins:**
  - The `drain_controls` twin and the route twin match the real shapes: the `let Self { .. } = self`
    destructure, `controls.iter_mut().enumerate()`, `let .. else` with `.as_mut()`, and
    `self.control.consumer` with `debug_assert!`.
  - Mutation "outer collection root must be a parameter": the base tree fails at the twin
    (`lib.rs:32`, while `drain_fader_records:10` is not affected), and the real tree fails at
    `lib.rs:482`.
  - Mutation "entry receiver limited to one field": the route twin fails, and the real tree fails
    at `runtime.rs:900`.
- **Fresh bypasses of D1-D3:** both gates refuse each of these:
  - a labelled `loop`, `while let`, `.cycle()`, `.chain(repeat)`, `.cycle().zip(..)` and `(0..).take(4)`;
  - an `array::from_fn` inside a `loop`, a generic `::std::array::from_fn` as an outer loop, and `Gen::from_fn`;
  - `use core::iter::from_fn as gen`;
  - `#[core::prelude::v1::cfg(any())]` and a `cfg` on an alias source;
  - a const after the loop in the same block, and a `static` in a macro token tree;
  - a closure parameter that shadows a slice parameter;
  - an async block with a loop: the forbidden-body rule refuses it, and the drain rule refuses it too.

  These pass both gates, as limits that are known or that this verdict now reports:
  - closure forms (`(0..).for_each(|_| ..)`, `repeat(()).map_while(..)`): B2a;
  - `&mut rounds`: MINOR-1;
  - SP-2 and SP-3.

  `0..usize::MAX` passes by design ("finite is not small").
- **Gate 5:** the release binary runs on the real tree in 1.01 s (twice).
- **Gate 6:** see SP-1. `.so` and the worklet module are identical. `.a` differs only in `.llvmbc`.
- **Gate 7:** these all exit 0: `cargo fmt --all -- --check`,
  `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
  `check-realtime-policy.sh`, `test-realtime-policy.sh` and `check-workspace-policy.sh`.
- AGENTS.md naming and version-suffix rules hold. No test greps source or prose: the cases are the
  tool's own input. There is no realtime or production code change except the two comment lines.
  There is no queue, so the acked-batch question does not apply.

## Test value (one sentence per new test)

- `the_finite_outer_loops_pass`: goes red if any D1 finite form is refused (bound, reference,
  `iter`, enumerate and zip, slice or array parameter, bounded loop, `array::from_fn`). No earlier
  case pins these.
- `the_outer_loops_that_may_not_end_are_refused`: goes red if `loop`, `while`, an open, inclusive or
  non-zero range, `rev`/`take`/`cycle`, a by-value field or vector, or an iterator parameter is
  accepted as an outer loop. Confirmed by the D1-off and every-`for` mutations.
- `a_slice_parameter_of_a_function_outside_the_region_is_not_read`: goes red if a signature outside
  the region qualifies a slice parameter. Confirmed: it alone goes red.
- `an_item_that_can_name_the_count_refuses_the_drain`: goes red if a block `const`, `static mut` or
  `use` (glob, name or nested) is not seen, or if an unrelated item refuses. Both directions
  confirmed.
- `a_count_binding_that_may_not_be_compiled_is_refused`: goes red if `cfg_attr`, a path ending in
  `cfg`, a `cfg` after another attribute, or a `cfg` alias source is missed. Confirmed.
- `a_constructor_that_loops_is_refused_anywhere_in_a_region`: goes red if a constructor with no pop,
  a raw spelling, a longer or foreign path, a `use`d `array`, or a macro token is not refused.
  Confirmed (9 rows).
- `a_pop_directly_in_an_array_from_fn_closure_is_refused`: goes red if the closure of
  `core::`/`std::array::from_fn` is not treated as a loop. Confirmed.
- `finite-loop-around-the-region` (a row of an existing test): goes red if an enclosing loop outside
  the region is judged as an outer loop. Confirmed: it alone goes red.
- `per_queue_inside_a_finite_outer_loop`, `per_queue_inside_an_unbounded_outer_loop` and
  `per_queue_pass_count_loop_may_not_end`: they go red if the loops around a D8b form are left
  unjudged. Confirmed by the D1-off mutation.
- The 14 ported deferred rows: each goes red under the mutation of its rule, as listed under gate 4.
- The D6 twins (base tree): they go red if the port refuses the `drain_controls` or route shape.
  Confirmed by the two twin mutations, which leave the older base drains green.

## Evidence kept

`/tmp/claude-1002/v1441/probes/*.rs` (bypass probes), `/tmp/claude-1002/v1441/parity.out`,
`/tmp/claude-1002/v1441/mut-*.log`, `/tmp/claude-1002/v1441/shadow.rs`, `shadow2.rs`.
