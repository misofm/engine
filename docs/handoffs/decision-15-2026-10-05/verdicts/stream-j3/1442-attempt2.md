FAIL

# #1442 attempt 2 verdict (B2a: pops in closures, macros and function values; probe corpus)

Range `c8d971814..49f02e984` on `codex/d15-stream-j3` (436bdd853 code and tests, 49f02e984 Attempt
record). I exported the head to `/tmp/claude-1002/v1442b/tree` and built it with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1442b/target`. I also built the parent `c8d971814` for gate 4. I
did not touch the worktree. The probes, the rustc programs, the scripts and the mutation logs are in
`/home/bl/misofm/submix-verdicts/evidence/1442-attempt2/`.

D1 (Option A), D2 with F2, D3 (iv), (b), (d), the corpus, the parity, the fuzz counts, gates 1, 2, 6
and 7, and every named mutation are correct, and I reproduced them. Every probe of the three earlier
verdicts is refused. The attempt fails on two false passes that are not in a listed limit of the
threat model (#1438 Hazards). Each compiles and pops without end, and the tool says `ok`:

- F1: a `map` or `as_mut` method that a macro defines, in three forms that the narrowed D3 (a) does
  not see. One form (F1c) also passes root's literal text of (a) and (c), so root must amend (c).
- F2: a written item outside the region that shadows the collection root of D9.

## MAJOR

- **F1. A macro can define the `map` and `as_mut` that D3 (iv) trusts, and the scan does not see it**
  (`src/drain/trusted.rs:156` and `:250-261` `defines_a_name`; `:282-289` `holds_fn_dollar`; module
  documentation `src/drain/mod.rs:71-74` and `:81-97`).
  - Reproducer (evidence `rs/k1.rs`, `rs/k2.rs`, `rs/k5.rs`, rustc 1.97.1, edition 2024). A user
    trait `Spin` on the field's type has an `as_mut` that returns `self` and a `map` whose body is
    `while f(self).is_ok() {}`. The drain is `let _ = self.control.as_mut().map(Consumer::try_pop);`,
    the form that D3 (iv) allows. Each program prints `popped 1000 of 1000`.
  - The trait comes from a macro in one of these forms. The tool gives `ok` with the drain in a
    marked region, with no loop and in a counted loop (evidence `probes/k1`-`k5`, `m04`-`m06`):
    - **F1a.** The keyword arrives through a fragment: `macro_rules! mk { ($k:tt) => { trait Spin {
      $k as_mut(..); $k map(..); } .. } } mk!(fn);`. In the body, `map` follows `k`, which is not
      a keyword of `defines_a_name`, and the body holds no `fn $`, so (c) is silent.
    - **F1b.** The name is not after a keyword in the invocation: `mk!(as_mut map fn)` with
      `($a:ident $m:ident $k:tt) => { .. $k $a(..) .. $k $m(..) .. }`, `mk!([fn] map)`, or an outer
      macro that forwards the name, `macro_rules! name { ($c:ident) => { $c!(map); } } name!(def);`
      (`def` emits `fn $n`). The macro `name` invokes `$c`, so the closure of (c) does not reach `def`.
    - **F1c.** `mk!(fn x.as_mut, x.map)` with `($k:tt $x:ident . $a:ident, $y:ident . $m:ident) =>
      { .. $k $a(..) .. $k $m(..) .. }`. The names follow `.`, so (a) skips them in both texts, and
      the body holds no `fn $`, so the macro is not name-emitting by (c)'s text.
  - **Root's literal (a) closes F1a and F1b.** With `defines_a_name` removed (literal (a)), the tool
    refuses k1-k4 and m04-m06. It also refuses the 17 `let map` locals that the Attempt record
    lists. I confirmed the 17 lines exactly (`mutations/mut-a-literal.log`).
  - **F1c passes root's literal text too.** With literal (a), F1c gives no finding in its file
    (`mutations/mut-a-literal-k5.log`, plus an isolated run: 0 findings in the probe's file). The
    `fn` keyword can reach a body through a `tt` fragment, so "a body that holds `fn` directly
    followed by `$`" does not close the set of name-emitting macros. This is a spec problem (SP-1).
  - This is not limit 1 or limit 2. The threat model says that the scan refuses every definition,
    import, rename or macro-emitted use of a trusted name under the roots that the rule's text does
    not allow. These definitions are in the workspace and come from a workspace macro.
  - The module documentation overclaims. `mod.rs:71-74` says that `trusted` refuses "a definition,
    import, rename or macro-emitted use" of a trusted name. `mod.rs:81-97` says that the false passes
    that remain are only the threat-model limits. It also says that a field whose type has its own
    looping `as_mut` or `map` is outside the check only "when the type is not defined in the
    workspace".
  - Pending root ruling on (a). I do not fail on the wording. I fail on the false pass that the
    narrowed wording lets through (F1a, F1b), and on F1c, which no text of (a) or (c) refuses.
  - A candidate, measured on the real tree (`opt2.pl`, `mutations/mut-option-c2.log`). Keep (a) as
    implemented outside macros. Add: refuse `map` or `as_mut` that does not follow `.` or `::` in
    any `macro_rules!` body. Then widen (c): refuse a trusted name in the invocation of any macro
    that is defined under the roots, not only a name-emitting one. This refuses k1-k5 and m04-m06.
    The suite stays at 163 passing. The real tree has one finding: `per_width!` at
    `crates/builtins/src/lib.rs:5851`. That is the one workspace-macro invocation with a trusted name
    that the spec measured. Root decides whether that site is rewritten or allowed.
  - Out of the stable build: on the pinned nightly `nightly-2026-08-20`, with
    `#![feature(macro_metavar_expr_concat)]`, `fn ${concat(ma, p)}` makes `map` with no `map` token
    anywhere (evidence `rs/cc.rs`, which pops 1000 of 1000). The stable pin 1.97.1 refuses it with
    E0658. I record it for root (SP-1). It is not part of the FAIL.

- **F2. A written item outside the region shadows a collection root, and the drain passes (D9)**
  (`src/drain/outer.rs:99` `read_root`; `src/drain/mod.rs:318` asks `names_item`, which reads only
  the leaves of the region, `mod.rs:175` and `:239`).
  - D9's text is: "an item, of any kind or `use`, that declares the name of ANY binding a drain rule
    reads is refused, wherever it sits in the block".
  - The count rule requires its binding inside the region (`count.rs`, `ctx.region.holds`), so an
    item that can shadow the count must be in the region too. The probe `r08` (struct before BEGIN,
    count let outside) is refused.
  - `read_root` does not require this. It notes any binding the root resolves to, but the item check
    reads only the region's leaves. D10's span check reads the whole file, but only for macros and
    attributed items. So a plain item in the same block, outside the markers, is not seen.
  - Reproducers. Each probe passes the tool (evidence `probes/r04`, `r05`, `r07`, `r11`):
    - `r04`: `let State { controls, .. } = state; { struct controls; impl controls { fn
      iter_mut(..) } // BEGIN for control in controls.iter_mut() { <counted drain> } // END }`.
    - `r05`: a parameter `controls: &mut [Consumer]`, with the region inside the body and
      `struct controls;` after END.
    - `r07`: the `&controls` form of `r04`.
    - `r11`: `use spin::controls;` after END.
  - The same struct inside the region (`r12`) is refused, and so is the macro form before BEGIN
    (`r10`).
  - rustc (evidence `rs/r04.rs`). `r04` and `r05` compile. The outer loop runs over the struct's
    endless iterator: it stopped only at my cap of 5001 passes, over an empty slice, and popped
    1000 of 1000 records. This is the r01 hole that D9 is for, moved outside the markers.
  - A per-queue root has the same cause. `pq_norounds_2` has a `static ready` before BEGIN, with the
    region inside the body, and it passes. I did not build a rustc program that shows it unbounded.
  - Fix: require each read root (and `<Q>`, `<K>`) to be bound inside the region, as the count is,
    or read the items of the binding's whole block. Add a refused case with the item outside the
    markers.

## MINOR

- **F3. The test-value sentence of `u19-swapped` is false as a single defect**
  (`tests/policy/drain/forms.rs:145-152`).
  - The doc comment says the test is "red if the resolver accepts a decrement that a body `let` has
    rebound".
  - When `resolved_uses` resolves by name only (`mutations/mut-resolver-by-name.log`), the case
    stays green. Only `form-shadow-in-a-closed-block` goes red.
  - It is red only when the first-statement rule is also relaxed
    (`mutations/mut-first-relaxed-and-resolver.log`), which agrees with the Attempt record.
  - The record says this plainly. The test's doc comment does not.
  - Under AGENTS.md's test-value rule, no single plausible defect turns this case red. Root
    mandated it (Amendment 1 (i)), so I record it for root (SP-2) and do not fail on it.

## NIT / recorded

- **Not covered before this slice.** When the first-statement rule of `while_bounds` is relaxed to
  "the first binary statement", all 163 tests stay green (`mutations/mut-first-relaxed.log`). That
  relaxation accepts `while n != 0 { <pop>; if x { continue; } n -= 1; }`. This gap is from B1a, not
  this slice. I record it for root as a candidate replacement for `u19-swapped` (SP-2).
- **Limit 2, recorded.** A second workspace `fn try_pop(&mut self)` on another type, which (b)
  allows by its text, is impl resolution. The receiver item shadow `n15` (#1441) is the same.
- **Limit 3, recorded.** `#[path = "x.txt"] mod` is #1519 (`path-mod-non-rs` still passes here). A
  nested `fn` in the region is B2b-2 (`fn-item-in-region`).

## Verified correct

- **Earlier probes.** All are refused: `f02`-`f05` (and `f01`, now a counted drain in a closure),
  `r01`, `r02`, `b01`-`b07`, and every `d2`, `d2b`, `d9`, `g`, `n` and `pq` probe of #1441.
  `old-probes.out` lists 175 runs. The accepted ones are exactly `a00`, `e11`, `f00`, `g03`, `g05`,
  `g10`, `g16`, `n15` (limit 2), `w02`, `x06`, `d02`, `d06`, `pq_ok`, `s02` and `r03`, as in the
  #1441 verdicts.
- **The attempt-1 verifier rows** (re-run as a temporary test, then deleted):
  - F1 `user-map-no-loop` and `user-map-trait-in-region` are refused.
  - `repeat`, `cycle`, the counted-loop form, `for` over `repeat(..).map`, and `zip` with `repeat`
    are refused.
  - `proc-macro-fn`, the 8 shadow forms, `macro-generated`, `use-in-tools`, the qself, UFCS,
    `?`-in-closure, const and static fn pointer, `flat_map`, turbofish, `&`, paren and block `.map`
    forms, the two `from_fn` closures and the async closure are refused.
- **Real-tree measurement of D3 (a).** The literal (a) refuses exactly the 17 locals listed in the
  record. (b), (d) and F2 refuse nothing on the real tree.
- **Gate 1.** Both gates print `realtime policy: ok (94 marked regions in 26 files)`. Release run
  1.306 s.
- **Gate 2.** 163 passed.
- **Gate 3** (`par.py`, the 120 committed probes on the materialized base tree, gawk and the release
  tool).
  - The tool matches the expected class of every file name.
  - Awk and tool differ on exactly 10: a06, a07, b02, n09, e11 (D1), e07 (D2), v14 and v14b (SP-4),
    n06 (Option A: awk passes, tool refuses), and u19 (awk refuses, tool accepts).
  - The awk agrees with the TSV everywhere.
  - Corpus: 87 `drain`, 1 `forbidden-body+drain`, 1 `unsafe+drain` and 31 `accepted`, which is
    89 + 31. v14, v14b and n06 are byte-equal to the bundle with the markers removed.
- **Gate 4** (`fz.py`).
  - Unbounded: 3767 regions, 3762 refused, 1 parse failure (`fz_2` s125) and 4 accepted (`fz_1`
    s249, `fz_3` s63, `fz_11` s170, `fz_14` s229, explained in attempt 1).
  - Bounded: 58 + 48 = 106 refused with the drain class and 694 accepted. 66 + 59 = 125 are also
    unsafe-class.
  - The parent `c8d971814` gives the same 58 and 48. Each of the 106 holds a `name!` group or an
    attribute (#1441 D10).
- **Gate 5** (each mutation alone, the suite and the real tree after each, logs in `mutations/`):
  - Closures transparent: 4 tests, with the 5 closure rows (F3's row among them), the 5 `from_fn`
    rows, `array-from-fn-pop`, and a06, a07, b02, n06, n09, e11.
  - `from_fn` closure exempt (direct): the `from_fn` rows, `array-from-fn-pop` and n06.
  - `from_fn` on any ancestor: the same, plus F3's `closure-in-a-nested-closure-in-array-from-fn`.
  - Macro token allowed: `macro-call-with-no-loop`, `debug-assert-argument` and u09.
  - Any path value: the 12 function-value rows.
  - Any `.map` receiver: the 5 receiver rows.
  - `.map` refused: 113 tests red, and the real tree refuses `live.rs:366` (exempt).
  - (a) dropped: the nested-module test and its 4 definition rows.
  - (b) `&self` only: `try-pop-by-value-on-a-reference-impl`.
  - (c) dropped: 3 tests. (c) not closed: the inner-macro and rename tests.
  - (d) dropped: `macro-made-proc-macro`.
  - F2 dropped: the 8 shadow rows, the parent-module case, the untrusted case and e07.
    `proc-macro-fn` stays green.
  - F2 and (d) dropped: `proc-macro-fn` red as well.
  - `debug_assert` always trusted: the 8 rows and e07.
  - Root not noted: the 5 D9 item rows and the r02 and b01 macro rows.
  - `<Q>`/`<K>` not noted: the per-queue item test and both per-queue macro rows.
  - Item check dropped: 6 tests.
  - The real tree is `ok` after every mutation except `.map` refused, and after every restore.
- **Gate 6.** The diff touches only `tools/realtime-policy/**` and the spec. There is no
  production, `Cargo.toml` or `Cargo.lock` change.
- **Gate 7.** These exit 0: `cargo fmt --all -- --check`,
  `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
  `scripts/check-workspace-policy.sh`, `scripts/check-realtime-policy.sh` and
  `scripts/test-realtime-policy.sh`.
- **The base fixture.** `drain_lane_pairs` is now a plain `for lane in 0..2`, and the real tree has
  no such function. The from_fn exception is gone from `mod.rs`, `outer.rs` and `tree.rs`. The
  token allowance in `items::constructor_lines` stays.

## Test value (one sentence per new test)

- `a_drain_in_the_closure_of_array_from_fn_is_refused`: red if a closure of `array::from_fn` again
  exempts its pops, in any spelling of the path (confirmed by from-fn-exempt).
- `a_pop_in_a_closure_is_refused` (F3 row): red if every closure under a `from_fn` closure is exempt
  (confirmed by from-fn-any-ancestor).
- `a_function_value_that_pops_is_refused_unless_it_is_called_in_place` (new rows): red if `.map(<path>)`
  is allowed on a receiver other than `<fields>.as_mut()` (confirmed by map-any-receiver).
- `a_proc_macro_named_debug_assert_is_refused`: red only when F2 and (d) are both dropped (confirmed).
  Root kept this row on purpose.
- `a_user_trait_with_a_map_in_a_nested_module_is_refused`, `a_definition_import_or_rename_of_map_or_as_mut_is_refused`:
  red if (a) is dropped or reads only `fn` (confirmed by a-dropped).
- `a_try_pop_taken_by_value_on_a_reference_impl_is_refused`: red if (b) accepts any receiver other
  than `&mut self` (confirmed by b-amp-self-only).
- `a_map_name_that_a_macro_makes_is_refused`, `a_name_passed_to_an_inner_name_emitting_macro_is_refused`,
  `a_name_emitting_macro_invoked_through_a_rename_is_refused`: red if (c) is dropped, is not
  closed, or does not follow renames (confirmed).
- `a_proc_macro_that_a_macro_makes_is_refused`: red if (d) is dropped (confirmed).
- `the_trusted_names_in_their_allowed_forms_pass`: red if the scan refuses `.map`, `.as_mut`, the real
  `try_pop` definition or a local `map` (the literal-(a) mutation refuses that local).
- `an_item_that_declares_the_collection_root_refuses_the_drain`,
  `a_statement_macro_that_can_declare_a_collection_root_refuses_the_drain`,
  `an_item_that_declares_a_per_queue_root_refuses_the_form`: red if the collection root, or `<Q>`
  and `<K>`, is not noted as a read (confirmed).
- `a_collection_root_that_no_item_names_passes`: red if the root rule refuses an unshadowed root, a
  pattern root or `self`.
- `a_decrement_that_a_body_let_has_rebound_is_refused`: no single plausible defect turns it red (F3).
- `the_refused_probes_are_refused`, `the_bounded_probes_pass`: red if a rule accepts a drain that the
  verdicts refused (confirmed under five mutations), or refuses a bounded form that `main` accepts
  (confirmed under `.map` refused).

## Spec problems for root

- **SP-1 (D3 (a) and (c)).**
  - The narrowed (a) is unsound (F1a, F1b). The literal (a) closes those, but it fails the real tree
    on 17 locals.
  - Neither text closes F1c, because "fn directly followed by `$`" is not a closed definition of a
    name-emitting macro. A `tt` fragment can carry `fn`.
  - Root must amend (c). The measured candidate is above: refuse `map`/`as_mut` in every
    `macro_rules!` body, and refuse a trusted name in every invocation of a workspace macro. Its one
    real-tree cost is `per_width!` at `crates/builtins/src/lib.rs:5851`.
  - Also decide whether a nightly `${concat(..)}` name is a listed limit or is refused (for
    example, refuse `concat` after `$` in a macro body).
- **SP-2 (u19-swapped).**
  - Its sentence in Amendment 1 (i) is not true for a single defect.
  - Either accept it as a double-defect pin and correct the doc comment, or add a case that defends
    the first-statement rule. Today no test defends that rule: `while n != 0 { <pop>; if x {
    continue; } n -= 1; }`.
- **Pending rulings (facts confirmed).**
  - The corpus is 89 refused and 31 accepted (n06 flips under Option A).
  - Gate 4's "800 accepted" is stale: 106 are refused by #1441 D10, the same at the parent.

## For attempt 3

- Close F2 (the root binding in the region, or the whole block read) and add an outside-the-markers
  case.
- After root's SP-1 ruling, implement the amended (a)/(c) and add k1, k2, k5 and m06 as refused rows.
- Correct the module documentation (`mod.rs:71-97`) and the `u19-swapped` doc comment.
