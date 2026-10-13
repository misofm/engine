FAIL

# #1441 attempt 2 verdict (B1b: D9 region ends, D2 every item kind, D10 item macros and attributes)

Range: `1a33b2f6d..aa2f613ed` on `codex/d15-stream-j3`.
- `f9a26b706` is root Amendment 1. It changes spec text only.
- `aa2f613ed` is the attempt. It changes `tools/realtime-policy/**` and the #1441 spec.

Method:
- I exported `aa2f613ed` with `git archive` into `/tmp/claude-1002/v1441b/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1441b/target`.
- I ran the probes on a copy of that tree. Each probe was appended to `crates/engine/src/realtime/buffer.rs`, then the release tool and `scripts/check-realtime-policy.sh` ran.
- I compiled and ran each claimed bypass with rustc 1.97.1 (edition 2024).
- I did not touch the worktree.

There is one MAJOR, so the attempt fails. The MAJOR is a D10 false pass for a slice parameter. The ruled D10 text has the same hole, so root must also amend the spec (SP-A). The measured fix is two lines.

## MAJOR-1: D10 misses an item macro or attribute after the slice loop in the `fn` body

Where: `tools/realtime-policy/src/drain/mod.rs:309-327` (`read_span`, `BindingKind::Param`).

The defect:
- For a slice parameter, the span ends at the end of the body statement that holds the pop.
- But an item in the `fn` body shadows the parameter for the whole body, wherever the item sits.
- So a statement macro, or an item with a derive or attribute macro, at body level *after* the slice loop is not read by D10.
- D2's literal item scan sees a written `const lanes` there. That is why the attempt's own rebuilt rows (`slice_shadowed_by`, `tests/policy/drain/outer.rs:37-45`) put the item at exactly this place. D10 cannot see an expansion.

Probe `b04_slice_bare_rangefrom_after.rs` passes both gates (tool 0, awk 0):
```rust
macro_rules! shadow { () => { const lanes: core::ops::RangeFrom<u64> = 0..; }; }
// REALTIME_POLICY_BEGIN
fn probe(control: &mut Consumer<Record>, lanes: &[u8]) {
    for _ in lanes {
        let available = control.available_at_entry();
        for _ in 0..available { let Ok(record) = control.try_pop() else { break; }; apply(record); }
    }
    shadow!();
}
// REALTIME_POLICY_END
```
- The same program in rustc (`rs/p9.rs`) has a one-element slice and one record available. It pops without end: my guard exits it at 5000 pops.
- `rs/p3.rs` shows that a written `const lanes` after the loop shadows the parameter (3 passes, not 1).
- `b05` passes both gates too. It has `#[derive(MakeLanes)] struct Unrelated;` after the loop at body level.
- `b02` and `b03` are finite variants of the same hole.

Three texts claim more than the tool does:
- The `declares_an_item` doc and the D10 bullet of the module documentation (`drain/mod.rs:27-37`, `:281-287`) say "an item in the binding's own block cannot shadow it".
- The `read_span` doc (`:303-304`) says "a parameter ... its block is that body".
- The record says "The SP-2 and SP-3 limits are gone".

These are true for a `let` (`rs/p8.rs`: E0308). They are false for a parameter.

Measured fix (applied to my export, then reverted). For a `Param`, return `(body.full, body.end)`, which is the whole `fn` body. Results:
- 149 of 149 tests pass;
- the real tree passes (94 regions in 26 files);
- `b02`, `b03`, `b04` and `b05` are refused;
- `s01` stays refused.

Also needed: a new case (the `b04` shape) and a mutation, "param span ends at the body statement", that turns only that case red.

## MINOR-1: no test defends the read rewind

Where: `drain/count.rs:17-28`, `drain/mod.rs:137-150`.

Two mutations turn nothing red:
- **No rewind** (`m10`). The real tree still passes. The only effect is extra refusals.
- **Rewind to zero** (`m11`). This one also opens a false pass. In `pq_shadow.rs`, a per-queue form sits inside a `for _ in rounds` slice-parameter loop. A `shadow!();` at body start redefines `rounds` as `0..`. The correct code refuses the probe, but the tool accepts it under this mutation.

The fix is to add that shape as a refused row. It is the only shape where the rewind decides the result.

## NIT-1: a D9 row that defends nothing new

`a_region_that_ends_in_a_deeper_block_after_a_slice_parameter_is_refused` (`tests/policy/boundary.rs:261-267`) adds nothing over the count-form case:
- D9 is a boundary rule and never reads the region's contents.
- No plausible defect turns only this row red. Under `m1` both rows go red.

Delete it, or make it a D10 row.

## The implementer's open items (facts; root is ruling)

1. **D10 span widened past the end of the loop.**
   - The false pass is real. In `rs/p7.rs`, `{ for _ in 0..available {..} shadow!(); }` runs 3 times.
   - `c01` is refused only because of the widening. `m7` (the span ends at the innermost loop, as the ruling says) turns `item-macro-after-the-loop-in-an-inner-block` red, and nothing else.
   - The real tree passes under both spans. The widened span refuses everything the ruled span refuses, plus more, so it fails closed.
   - Recommendation: ratify it, extended to the whole `fn` body for a parameter (MAJOR-1).
2. **An item shadows a collection root.**
   - This is a **real false pass in both gates**:
     - `r01`: `struct controls;` with an `iter_mut` that repeats, in an inner block;
     - `r02`: the same item emitted by a macro before the loop;
     - `b06`: a written `struct lanes;` after a `lanes.iter()` loop over a slice *parameter*;
     - `b01`: the macro form of `b06`.
   - `rs/p6.rs` (the `r01` shape) pops without end.
   - It is not a regression: the awk passes all four.
   - It is outside the three threat-model limits. It is name resolution in workspace source, not type-level behaviour, and no spec stages it.
   - It also covers the per-queue roots. `per_queue::bounds` records no reads, so D10 never covers `<Q>` or `<K>`.
   - The module documentation lists it as a limit. Under the threat model, only root can do that.
3. **`debug_assert_eq!` and `debug_assert_ne!` are not exempt.**
   - This is an over-refusal, not a false pass (`g04` is refused). No drain span holds one.
   - Refusing them fails closed. B2a-D2's trust scan checks only the name `debug_assert`, so exempting the other two would need a wider trust scan first.

## Spec problems for root

- **SP-A (MAJOR-1).** D10's ruled span is "start of the slice parameter's `fn` body to the end of the innermost bounding loop". It has the same hole as the implementation, and also open item 1's hole for a `let`. Root should amend D10:
  - for a `let`, the span runs to the end of the statement of the binding's block that holds the pop (this ratifies item 1);
  - for a parameter, the span is the whole `fn` body.
- **SP-B.** Open item 2 needs a ruling: a rule, for example `names_item(root)` with a D10 span from the root binding, or a recorded limit.
- **SP-C (NIT).** `f9a26b706` copied the review's #1441 gate text into #1446 gate 5 word for word. So #1446 now says "There is no fallback: if a marker would move a line, D5 is skipped and B2b-1 places it." That does not fit #1446: its D5 is "Open specs, by role", and C2 places no marker. Drop the sentence there.
- **SP-D (no action).** #1441 D10 in `f9a26b706` adds a sentence that is not in the review's D10 text: "A trailing `m!()` with no semicolon is an expression...". It comes from the review's own SP-3 item 2, and rustc confirms it (`rs/p2.rs`: "expected expression, found keyword `const`").

I compared the rest of `f9a26b706` with `rj3-sp-review.md`, and it matches the review word for word:
- SP-1: #1441 gate 6, #1446 gate 5 and the #1443 gate 5 insertion;
- SP-2: D9 and its cases and mutations;
- NEW-1: D2;
- SP-3: D10, its cases and mutations;
- SP-4: the D1 append and the #1442 D5 line;
- #1442 (i) to (iv): u19, `u19-swapped`, the 88/32 counts, the gate-5 exemption, the D3 receiver text, the Hazards line and the two D6 cases.

## Checked and correct

**D9:**
- The SP-2 probe (`d01`) is refused with the new class on END's line.
- The other placements I tried are refused too:
  - END in a later `fn` (`d04`), in a closure block (`d05`) or in a nested `mod` (`d08`): refused for where the region ends;
  - END between match arms, in a struct body or in an array (`d03`, `d07`, `d09`): refused as a split.
- END in an enclosing block (`d02`) and BEGIN in an `impl` with END at file level (`d06`) stay accepted.
- Containers are every `syn::Block`, plus `mod`, `impl`, `trait`, `extern` and the macro groups. A block that holds both a pop and code after END must hold END, so the rule is sound.

**D2:** each item kind is refused:
- unit, tuple and `pub(crate)` structs, `r#`, `const fn`, `enum` with a glob `use`, `union`, `static mut` and an `extern` static (the unsafe rule also fires on that one);
- an item in a macro token tree.

An unrelated struct is accepted.

**D10:** these refuse the drain:
- `self::shadow!();`;
- a derive after the loop;
- a trailing braced `shadow! {}`;
- a macro in a nested `fn` or a match arm;
- an attributed `impl`;
- `#[allow] shadow!();`;
- a path derive;
- `cfg_attr`;
- `assert!`.

These are accepted:
- an expression macro (`shadow!().len()`);
- a trusted `debug_assert!`, `core::debug_assert!` or `r#debug_assert!`.

Declarative attribute and derive macros are unstable on 1.97.1 (`rs/p5.rs`: E0658).

**Gate 1:** the tool and `check-realtime-policy.sh` both print `ok (94 marked regions in 26 files)`.

**Gate 2:** 149 tests pass. That is 140 plus 3 for D9, 1 for D2 and 5 for D10.

**Gate 3 (parity, re-run):** an instrumented copy of the self-test ran the release tool on all 96 `expect_failure` trees (`parity.tsv`):
- the tool refuses all 96;
- 57 rows expect the drain class, and 55 of them report it from the tool;
- the other two are B1a-D10's `drain-open-tail-last-file` and `drain-region-ends-in-comment`;
- no other row reports the drain class.

**Gate 4 (each mutation alone, then reverted):**

| Mutation | Red | Real tree |
|---|---|---|
| m1: END in any container | `boundary-spans-two-blocks`, `region-ends-in-a-deeper-block` (and the slice variant), `boundary-only-end-splits` | ok |
| m2: same container only | `boundary-spans-impl-end` and 4 more | fails at `live.rs:537` and `transient-shaper/src/lib.rs:726` |
| m3: scan only `const` and `static` (with `use`) | `unit-struct-shadows-the-alias-source` and 8 kind rows | ok |
| m12: `names_item` is false | all 4 `slice-shadowed-*` rows now, plus count rows and probes | ok |
| m4: statement macros allowed | `item-macro-shadows-the-count`, `braced-item-macro-shadows-the-slice-parameter` and 4 more | ok |
| m5: `debug_assert!` always exempt | `untrusted-debug-assert-in-the-loop` and the B2a shadow rows | ok |
| m6: `debug_assert!` refused too | base route twin (100 tests) | fails at `graph/src/runtime.rs:900`, the pop under `:903` |
| m7: span ends at the innermost loop | `item-macro-after-the-loop-in-an-inner-block` only | ok |
| m8: slice parameter not recorded as a read | `braced-item-macro-shadows-the-slice-parameter` only | ok |
| m9: attributes never emit items | the 5 refused attribute rows | ok |
| m10 / m11: rewind | nothing (MINOR-1) | ok |

**Gate 5:** the release run takes 1.37 s.

**Gate 6:** neither commit touches `crates`, `hosts`, `scripts`, `Cargo.toml` or `Cargo.lock`. The D5 markers are unchanged since attempt 1. That attempt's measurement meets the amended text: three members differ, and only in `.llvmbc`.

**Gate 7:** these all exit 0:
- `cargo fmt --all -- --check`;
- `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`;
- `check-realtime-policy.sh`;
- `test-realtime-policy.sh`;
- `check-workspace-policy.sh`.

**Superseded tests:**
- `boundary-spans-two-blocks` is rewritten as refused, and `boundary-only-end-splits` expects both findings, as ruled.
- The two B2a tests that now expect the route twin are right: an untrusted `debug_assert!` in the twin's loop body is a statement macro.
- The three rebuilt `slice-shadowed-*` rows now go red under m12. Before, they went red for an unrelated reason: a pop with no count.

**Repository rules:**
- AGENTS.md naming holds: `RegionEndsInsideAContainer` is unversioned.
- No test greps source.
- No production code changed.
- There is no queue, so the acked-batch question does not apply.

## Test value (one sentence per new test)

- `a_region_that_ends_in_another_block_is_refused`: red if D9 refuses only a deeper END and accepts END in a sibling block.
- `a_region_that_ends_in_a_deeper_block_is_refused`: red if a region may end inside a block that continues past END (m1).
- `..._after_a_slice_parameter_is_refused`: no unique defect (NIT-1).
- `a_region_may_span_the_end_of_an_impl` (rewritten): red if D9 is read as "same container" (m2).
- `either_marker_alone_can_split_a_node` (rewritten): red if END's container is not judged.
- `an_item_that_can_name_the_count_refuses_the_drain` (extended): red if the scan misses an item kind, or misses items that shadow the slice parameter (m3, m12).
- `unit_struct_shadows_the_alias_source`: red if the scan reads only `static`, `const` and `use` (m3).
- `a_statement_macro_between_the_count_and_the_end_of_its_block_is_refused`: red if statement macros are allowed (m4), or if the span stops at the loop (m7).
- `a_statement_macro_in_the_body_of_a_function_with_a_slice_loop_is_refused`: red if a slice parameter is not recorded as a read (m8). It does not cover a macro after the loop (MAJOR-1).
- `an_attribute_that_can_emit_an_item_refuses_the_drain`: red if attributes never count (m9), or if `cfg`, a lint level or `doc` is refused.
- `a_macro_that_cannot_declare_an_item_in_the_span_is_not_refused`: red if expression macros, macros outside the span or a trusted `debug_assert!` are refused (m6 and the implementer's span-at-file mutation).
- `an_untrusted_debug_assert_in_the_loop_refuses_the_drain`: red if the exemption ignores B2a-D2's trust (m5).

## Evidence kept

All paths are under `/tmp/claude-1002/v1441b/`:
- `probes/**` (the `.rs` probes; `probe.sh` runs them);
- `rs/p1-p9.rs` (rustc programs);
- `parity.tsv`;
- `mut-*.log` and `mut.sh`.
