# Check realtime regions with a Rust syntax-tree tool: refuse pops in closures, macros and function values, and commit the probe corpus

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice B2a of ten (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a, #1519, C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what
each holds).
- B1a and B1b ported `main`'s drain rule as it stands.
- **B2a (this issue)** closes three of that rule's documented limits, which a syntax tree makes
  cheap to close: pops in closures, in macro token trees, and through function values. It also
  commits every compiled verdict probe as a case, as root ruled ("every compiled probe from
  #1302's and #1418's verdicts that is refused today becomes a committed case"), except a08, which
  B2b-2's call rule closes.
- C2 then deletes the awk gate (A1 to B2a cover every check it makes). After the #1448 guard
  commit, B2b-1 requires every non-test pop to be in a marked region or on a control-side
  allowlist, and B2b-2 refuses a call to a popping function in a marked loop or closure.

No production code changes. No dependency on another stream's open work.

## Problem (verified on `origin/main` at `6d28a80ec`)

The probe results were measured on `6b9067ede`. Between it and `6d28a80ec` the awk script and its
self-test did not change, and stream B batch 1 (`b8392df66`) changes only their floors and pad
files, which every probe leaves alone. The awk lines cited are `main`'s; on `b8392df66` every line
from `:70` on is two lower (the limits `:131-134` are `:133-136`, `:236-260` is `:238-262`), and
the self-test's lines from `:440` on are one lower.

- **Documented limits of `main`'s rule.** `scripts/check-realtime-policy.sh:131-134` lists these
  as outside the rule:
  - a pop in a closure that a loop calls, or that a repeating adapter holds;
  - a pop through a function value;
  - a pop in a macro.

  B1a ports them as limits (B1a-D4: closures and macros are transparent). These compiled probes
  pass `main`'s gate and pop until the queue is empty:
  - `v1302/adv/a06` (`iter::repeat(()).map_while(|()| q.try_pop().ok())`);
  - `v1302/adv/a07` (`(0..).map_while(..)`);
  - `v1302/adv/b02` (a local closure called in a `while let`);
  - `v1302/a2/n/n09` (`repeat` with `for_each`);
  - `vbatchj/rt/shapes/e11` (a closure pop called in a `while`);
  - `vbatchj/rt/shapes/e07` (a local `macro_rules! debug_assert` that rebinds the count).
- **`debug_assert!` is taken at its word** (`:236-260`; B1a-D7): a count may be named inside it.
  e07 shadows `debug_assert` with a local `macro_rules!` in the same file. The second review
  showed the same escape across files: a `macro_rules! debug_assert` in `lib.rs`, then `mod drain;`,
  with e07's drain in `drain.rs`, which defines no macro and has no `use`. `#[macro_use] mod`,
  and a glob `use` of a `#[macro_export]` macro, do the same. On `main`, 41 `debug_assert` lines sit
  in marked regions and none names a count binding. The self-test's base tree holds one that does
  (`scripts/test-realtime-policy.sh:284`, `debug_assert!(available <= MAX_RECORDS_PER_BLOCK);`, in
  a valid drain).
- **The probe corpus is not committed.** It is bundled at
  `/home/bl/misofm/submix-verdicts/evidence/1418-probes/probe-corpus.tar.gz`. Its paths are
  `v1302/{adv,a2/c,a2/n,a2/rc,a2/v,a3/shapes,a3/t,a3/v}/`, `vbatchj/rt/shapes/`, `v1418/rs/` and
  `v1418b/evidence/`. Each probe's result on `main`'s gate is in
  `probe-results-origin-main-6b9067ede.tsv` (same folder), produced with each probe in place of
  `fn drain_matrix_controls() {}` in the base tree, under gawk and mawk, with identical results.
  Probes are named by directory below, because `e01` and `e04` exist in both `v1302/a2/c` and
  `vbatchj/rt/shapes`.
  - **82 files (84 shapes) are refused:**
    - `v1302/adv`: a01-a05, a11-a15, a17, b03;
    - `v1302/a2/c`: c01-c18, d01-d05, d07, d08, e01-e04;
    - `v1302/a2/n`: n01, n02, n04, n05, n08;
    - `v1302/a2/rc/shapes.rs`: three shapes;
    - `v1302/a3/shapes`: s00-s03, s08, s10_block_static_shadow;
    - `v1302/a3/t`: u01-u20;
    - `v1302/a3/v`: v18;
    - `vbatchj/rt/shapes`: e02, e03, e05, e06, e09, e10, e12, u00.

    Two of them are refused by another class, so the TSV does not show that `main`'s drain pass
    refuses them: `v1302/a3/t/u13` by the unsafe class, and `v1302/a3/t/u08` by the forbidden-body
    class.
  - **33 bounded probes pass:**
    - `v1302/adv`: a09, a10, a16, b01;
    - `v1302/a2/n`: n06, n07;
    - `v1302/a2/v`: v01-v07;
    - `v1302/a3/v`: v01, v02, v03, v03b, v04, v04b, v05, v07, v08, v09, v11, v13, v14, v14b, v15,
      v16, v19, v20, v21;
    - `vbatchj/rt/shapes`: b00.
  - **15 unbounded probes pass:**
    - the six limits above (a06, a07, b02, n09, e11, e07);
    - `v1302/adv/a08` (a pop in a helper called in a loop; B2b-2);
    - eight outer-loop escapes, which #1418 owns: `v1302/a2/n/n03`, `v1302/a3/shapes/s04`-`s07`,
      `vbatchj/rt/shapes/e01`, `e04`, `e08`.
  - **The fuzz corpora** (`v1302/a3/fuzz/fz_*.rs`, with their generators `gen*.py`):
    - fz_1-4 and fz_11-14 hold 3767 unbounded regions, all refused;
    - fz_21 and fz_22 hold 800 bounded regions, all accepted.
  - **Not valid Rust:**
    - `v1302/a2/c/d06` (its verdict says so);
    - `v1302/a3/shapes/s10_check.rs` (the rustc check program for s10, not a probe);
    - `v1302/a3/t/u03_nested_comment`: rustc gives E0753, a `//!` after an outer doc comment, and
      `syn` refuses it;
    - `s09_frame0_let_b` (E0530; it is in `/tmp/claude-1002/v1302/a3/cc` only, not in the
      bundle).

    `v1302/adv/case7.rs.txt` is the shape of the ported case `loop_inside_bounded_for` (#1302
    attempt-1 verdict, "case 7").

## Decisions

(Placement, root Amendment 4, 2026-10-12: Amendment 4 below supersedes D3's `.map(<path>)` allowance, root Amendment 1 (iv) and root Amendment 2 SP-A (a), (b) and (c) with their D6 cases and gate 5 mutations, `u19-swapped`, and D10's span for a `let`; D9, F2 and B1a-D7 are amended there. Where an older bullet below disagrees, Amendment 4 governs.)

- **D1. Closures.**
  - (root Amendment 3, MAJOR-1; replaces the former first bullet, "A pop inside a closure body is
    refused, except in an `array::from_fn` closure, where B1b-D1 and B1b-D3 apply") "A pop inside a
    closure body is refused, the closure of `core::array::from_fn` and `std::array::from_fn` included.
    The B1b-D1/D3 outer-loop exception for that closure is withdrawn (root, 2026-10-11, #1441 attempt-3
    MAJOR-1: it trusted the path roots `core`/`std`, which a workspace `mod`, a `use .. as`, a
    macro-made module or a Cargo dependency key can redirect). B1b-D3's token allowance for the two
    paths stays. No marked region in `crates/` or `hosts/` pops in such a closure."
  - This refuses a06, a07, b02, n09 and e11.
  - No marked region on `main` pops inside a closure. On `6b9067ede` there were 143 closures in
    marked regions, and none holds `try_pop` (the second review measured it with `syn`).
- **D2. Macros.**
  - A `try_pop` identifier in any macro token tree is refused. That includes `macro_rules!`
    bodies and `debug_assert!` arguments.
  - **`debug_assert!` is taken at its word (B1a-D7) only if no file under the scan roots shadows
    it.** The tool collects, over every `.rs` file under the roots: any `macro_rules! debug_assert`
    (at any depth, local ones included), and any `use` whose leaf or rename is `debug_assert`
    (`use x::debug_assert;`, `use x::y as debug_assert;`). If it finds one, a count named inside
    any `debug_assert!` is refused everywhere, as in any other macro.
  - This refuses e07 and its cross-file form. It keeps the base tree's valid drain at `:284` and
    the accepted probe `v1302/a3/v/v13_debug_assert`.
  - **Stated limit.** A registry crate that exports a `debug_assert` macro and is imported with
    `#[macro_use] extern crate` or a glob `use` is outside the check. No crate in the workspace
    does this; the tool's module documentation says so.
  - **(root Amendment 2, F2)** This text replaces the enumeration in the second bullet above:
    "Count as a shadow any `debug_assert` identifier (bare or `r#`) under the roots that is not
    directly followed by `!`. This one rule replaces D2's enumeration of `macro_rules!`, `macro` and
    `use` forms; delete the enumeration code. Proc macros are refused by D3's (d)."
    Measured on the tree: 122 `debug_assert` identifiers, all directly followed by `!`, so it refuses
    nothing. A macro-made `macro_rules! $n` or `use .. as $n` is not an escape: rustc gives E0659.
    A workspace proc macro is one (`mk!(debug_assert!)` in a proc-macro crate rebinds the count), and
    D3's (d) closes it.
- **D3. Function values.**
  - A path expression whose last segment is `try_pop` is allowed only in two positions:
    - as the callee of a call (`Consumer::try_pop(control)`);
    - as the only argument of a `.map(..)` method call whose receiver is `<fields>.as_mut()`,
      where `<fields>` is an identifier (or `self`) followed by `.field` steps (`live.rs:366`,
      #1426 D2's form). `Option::as_mut` yields at most one value, so the pop runs at most once
      per evaluation and is judged by the loops around it. On any other receiver, `.map` may be an
      iterator adapter that pops once per item with no count, or without end through a
      `try_pop(&self)` and `iter::repeat`. (root Amendment 1, iv)
  - Any other use is refused, for example a `let pop = Consumer::try_pop;`.
  - **(root Amendment 2, SP-A)** The trusted-name scan below amends D3 and is adopted alongside
    root Amendment 1 (iv). Its text:
    "The trusted-name scan reads the tokens of every `.rs` file under the roots, at any depth, and refuses:
    (a) an identifier `map` or `as_mut` (bare or `r#`) that does not directly follow `.` or `::`;
    (b) an identifier `try_pop` that does not directly follow `.` or `::`, except in `fn try_pop(&mut self`, with the receiver tokens exactly `&`, `mut`, `self`;
    (c) an identifier `map`, `as_mut`, `try_pop` or `debug_assert` in the token tree of an invocation of a name-emitting macro. A name-emitting macro is a `macro_rules!` or `macro` under the roots whose body holds `fn` directly followed by `$`, or that invokes a name-emitting macro (the set is closed transitively). An invocation is matched by its last path segment. An invocation through a name that a `use` item under the roots renames (`as n`) counts as one;
    (d) any identifier `proc_macro`, `proc_macro_attribute` or `proc_macro_derive`. A workspace proc macro can emit any name, pop or shadow that the scan cannot see.
    A registry trait or type method named `map` or `as_mut` is a stated limit."
    Why the draft's narrower forms fail: `macro_rules! mk { ($a:ident . $n:ident) => { trait Spin<T>{fn $n..} impl<T> Spin<T> for &mut T { fn $n.. { while f(self).is_ok(){} } } } } mk!(x.map);`
    compiles, and `c.map(Q::try_pop)` then pops 1000 of 1000, so a scan for the text `fn map` does not
    see it; `self: &Self`, or `impl Pop for &Q { fn try_pop(self) }`, escape a `&self` receiver check.
    **Measured** on `main` and on the branch head `1a33b2f6d`, read-only (Python token scan, comments
    and strings removed, over `crates/`, `hosts/` and `tools/`): the rule refuses nothing.
    - (a): 2429 `.map(`, 332 `.as_mut(` and 6 `Option::as_mut`. All follow `.` or `::`; there is no
      `fn map` and no `fn as_mut`.
    - (b): 102 `.try_pop(`, 1 `Consumer::try_pop`, and the one definition at
      `crates/engine/src/realtime/spsc.rs:441` (`fn try_pop(&mut self)`).
    - (c): five `fn $` macros exist (`session/src/estimate.rs:363`, `builtins/src/corpus.rs:599/602/605`,
      `conformance/src/randomized.rs:2760`) and none receives a trusted name. The only trusted name in
      any workspace-macro invocation is `.map(` inside `per_width!` (`crates/builtins/src/lib.rs:5851`),
      and `per_width`'s body emits no `fn $`.
    - (d): there is no `proc_macro` identifier and no `proc-macro = true`.
- **D4. The allow-list changes.** B1a-D4 makes `Expr::Closure` and `Expr::Macro` transparent.
  D1 and D2 replace that with refusal on a pop's path. The rest of B1a-D4 is unchanged.
- **D5. Committed probe cases.**
  - Use A1's case format. Each probe replaces `fn drain_matrix_controls() {}` in the base tree,
    with its own marker lines removed, so A1-D6 does not refuse nested markers.
  - Split `v1302/a2/rc/shapes.rs` into its three shapes.
  - The expected class sets are stated here, not taken from the tool's output:
    - every probe that the TSV shows refused with the drain class: {drain}, except
      `v1302/a3/t/u19`, which is bounded (the body's `let remaining = 5;` follows the decrement
      and shadows only the rest of the body) and is committed as accepted. The awk over-refuses
      it. (root Amendment 1, i)
    - `v1302/a3/v/v14_len_outer` and `v14b_len_outer` are committed as refused, expecting
      {drain} (B1b-D1's `<bound>`). Gate 3 names them as differences from the TSV. Test value:
      red if the outer bound grammar accepts a method call or an index without a ruling. (root
      Amendment 1, SP-4)
    - `v1302/a3/t/u08`: {forbidden-body, drain};
    - `v1302/a3/t/u13`: {unsafe, drain};
    - `v1302/a3/t/u03`: committed in a valid form (`//` in place of `//!`), expecting {drain};
    - a06, a07, b02, n09, e11: {drain} (D1);
    - e07: {drain} (D2).
  - The corpus is **88 refused and 32 accepted (120 files)**: refused are 84 shapes plus 6 newly
    refused, minus 3 duplicates, minus u19, plus v14 and v14b; accepted are 33, minus v14 and v14b,
    plus u19. (root Amendment 1, ii)
  - The 33 bounded probes become accepted cases, except v14 and v14b (above), and u19 is added. A probe whose shape the base tree or a B1a or B1b case
    already holds is not committed twice. The Evidence maps it to the case that holds it.
  - Not committed here:
    - a08, which B2b-2 commits with its call rule;
    - d06, s10_check, s09 and case7, for the reasons in the Problem;
    - the eight outer-loop escapes, which #1418 owns.
- **D6. New cases.** One each:
  - a pop in a closure stored in a `let` and called in a loop;
  - `let pop = Consumer::try_pop;` (D3);
  - a pop inside a local `macro_rules!` body;
  - e07's drain in `drain.rs`, with `macro_rules! debug_assert` in the parent `lib.rs` before
    `mod drain;` (refused, D2);
  - a `use core::debug_assert as debug_assert;` anywhere under the roots, with the base tree's
    valid drain unchanged (refused at `:284`'s line, D2);
  - `Consumer::try_pop(control)` in a counted loop (accepted, D3);
  - `self.control.as_mut().map(Consumer::try_pop)` in a counted loop (accepted, D3).
  - (root Amendment 1, i) `u19-swapped`: u19 with the inner `let mut remaining = 5;` before the
    decrement (refused). Test value: red if the resolver accepts a decrement that a body `let`
    has rebound, which would make u19's acceptance blind to order.
  - (root Amendment 1, iv) `function-value-in-map-on-an-iterator`
    (`qs.iter_mut().map(Consumer::try_pop).count()`, refused). Test value: red if
    `.map(<path>)` is allowed on a receiver other than `<fields>.as_mut()`.
  - (root Amendment 1, iv) `function-value-in-map-on-repeat`
    (`std::iter::repeat(q).map(Queue::try_pop).take_while(Result::is_ok).count()`, refused).
    Test value: red if the restriction is a blacklist of collection receivers, not an allow-list.
  - (root Amendment 2, SP-A) Five cases for D3's trusted-name scan, each refused. Test value, one
    sentence each:
    - `user-map-trait-in-a-nested-module`: F1's reproducer (a user trait with a looping `map`)
      inside `mod a { mod b { .. } }`. F1's source shape is a user trait `Spin` with a `map` method
      whose `impl` calls its argument in a `while f(self).is_ok() {}` loop, applied as
      `control.map(Consumer::try_pop)` (or `x.map(Consumer::try_pop)`) in a marked drain; it pops
      1000 of 1000 records (`/home/bl/misofm/submix-verdicts/1442-attempt1.md` F1 and `evidence/1442-attempt1/m.rs`). Red if the scan reads only top-level items or one file; this
      shape passes today.
    - `try-pop-by-value-on-a-reference-impl`: `impl Pop for &Q { fn try_pop(self) }`. Red if (b)
      checks only for `&self`, which was the draft's wording.
    - `macro-made-map-name`: `mk!(x.map)` where `mk!` emits `fn $n` (the shape above). Red if (c) is
      dropped or matches only the text `fn map`. Reproduced: 1000 of 1000 pops.
    - `macro-passes-name-to-an-inner-name-emitting-macro`: an outer macro passes `$n` to an inner
      `fn $n` macro. Red if the name-emitting set is not closed transitively.
    - `macro-made-proc-macro`: `macro_rules! mk { ($n:ident !) => { #[proc_macro] pub fn $n(..) } }`
      with `mk!(debug_assert!);` in a workspace proc-macro crate. Red if (d) is dropped.
  - (root Amendment 2, F2) The 8 existing shadow rows defend the replacement rule. Keep the F2
    `proc-macro-fn` row: it is red only if both the F2 rule and D3's (d) are dropped.
    The `proc-macro-fn` row's source shape: a file `crates/shadowing/src/lib.rs` that defines
    `#[proc_macro] pub fn debug_assert(input: TokenStream) -> TokenStream { input }` (no `macro_rules!`,
    no `macro`, no `use`), beside a drain that names its count inside `debug_assert!`
    (`/home/bl/misofm/submix-verdicts/1442-attempt1.md` F2 and `evidence/1442-attempt1/pm.rs`, `zz_verifier.rs.txt`).
  - (root Amendment 2, SP-B; removed by root Amendment 3) The six D8 cases moved to #1519.
  - (root Amendment 3, MAJOR-1) `from-fn-closure-drain-is-refused` (f02's drain: a counted drain in a
    `core::array::from_fn` closure, no shadow; refused). Test value: red if the `array::from_fn`
    outer-loop exception for a closure returns.
  - (root Amendment 3, SP-2) `unit-struct-shadows-the-collection-root` (`struct controls;` before a
    repeating `controls.iter_mut()` outer loop around a counted drain, as the attempt-2 verdict's r01;
    refused). Test value: red if the item rule reads only the count, its alias source and the slice
    parameter, and not the collection root of an outer `for`.
  - (root Amendment 3, SP-2) The cases r01, r02, b01 and b06 (see D9), each refused, and the same
    shadow of a `<Q>` or `<K>` root of a per-queue form.
  - (root Amendment 3, F3) The row `closure-in-a-nested-closure-in-array-from-fn` never turns red
    (`tests/policy/drain/indirect.rs`). Mutation: `in_closure` exempts every closure when any
    `array::from_fn` closure is on the path; all 140 tests stay green under it. The row's pop is under
    no counted loop, so the "no count" rule refuses it either way. A row that turns red under that
    mutation is a nested closure that pops in a `while` inside a counted `for` inside the `from_fn`
    closure. Replace the row with that one, or delete it.
- **D7. Documentation.** The drain rule's module documentation drops the closure,
  function-value and macro limits from B1b-D4, states D1 to D3, and states D2's limit.
- **D8.** Moved to #1519 (root Amendment 3, rj3-c R2). #1443 D1 rule 3 and Amendment 2 name "#1442 D8"; that is #1519's D1.
- **D9. Collection-root shadowing (root Amendment 3, SP-2).** Root's text: "Collection-root shadowing
  (`struct controls;` with a repeating iter_mut): a false pass, not a listed limit. Fix it in #1442
  attempt 2, not as a #1441 attempt 3. Generalize the item rule: an item, of any kind or `use`, that
  declares the name of ANY binding a drain rule reads is refused, wherever it sits in the block. That
  covers the count, its alias source, the slice parameter and the collection root of an outer `for`.
  Add the spec line, a refused case (`unit-struct-shadows-the-collection-root`, with its test-value
  sentence and a mutation), and a real-tree re-measure. It must land before #1446 removes the awk."
  Root's extension: "Keep collection-root shadowing in #1442 attempt 2, with the verifier's extension to
  the per-queue <Q>/<K> roots and the macro forms (r01, r02, b01, b06)." Probe shapes (the #1441 attempt-2
  verdict, `/home/bl/misofm/submix-verdicts/1441-attempt2.md`, open item 2): `r01` `struct controls;` with a
  repeating `iter_mut`, in an inner block; `r02` the same struct emitted by a macro before the loop; `b06` a
  written `struct lanes;` after a `lanes.iter()` loop over a slice parameter; `b01` the macro form of `b06`.
  The implementer re-measures the rule on the real tree (gate 1).

### Root Amendment 1 (2026-10-11): the attempt-1 open items

Authority: root decisions under the owner's delegation, after the #1441 attempt-1 verdict and an
adversarial review (ACCEPT WITH CHANGES; root adopted its texts verbatim). Each is placed where it
belongs and marked "(root Amendment 1)":

- **i** u19 is accepted, with D5's text, and `u19-swapped` is added (refused).
- **ii** The three duplicates (e02, e03, e06) are not committed. The corpus is 88 refused and 32
  accepted (120 files).
- **iii** Gate 5's `.map(<path>)` mutation is exempt from "the real tree still passes".
- **iv** D3 allows `.map(<path>)` only on a `<fields>.as_mut()` receiver, with its Hazards line,
  the two D6 cases and the gate 5 mutation. `Consumer::try_pop` takes `&mut self` today, so the
  coordinator's `repeat` bypass does not compile with that type; the rule still matched by name
  and would accept a `try_pop(&self)` type or a per-item pop through `qs.iter_mut()`.
- **SP-4** v14 and v14b are committed as refused, expecting {drain} (B1b-D1's `<bound>`).

### Root Amendment 2 (2026-10-11): the trusted-name scan, F2, source rustc reads, and the threat model

Authority: root decisions under the owner's delegation, after the attempt-1 verdict (SP-A, F2, SP-B)
and a fresh adversarial review (item 1, 2, 3 ACCEPT WITH CHANGES; item 4 REJECT as drafted, ACCEPT
with a replacement text). Root adopted the review's texts verbatim; the draft threat model is
rejected. Each is placed where it belongs and marked "(root Amendment 2)":

- **SP-A** D3's trusted-name scan (a) to (d), with five D6 cases and four gate 5 mutations.
- **F2** One rule replaces D2's enumeration: any `debug_assert` identifier not directly followed by
  `!` is a shadow. The enumeration code is deleted; the `proc-macro-fn` row stays.
- **SP-B** Moved to #1519 (root Amendment 3).
- **Threat model** The text is in #1438 Hazards; this spec's Hazards refer to it.

All three rules were measured at zero refusals on the real tree (`main` and `1a33b2f6d`).

### Root Amendment 3 (2026-10-11): MAJOR-1 (Option A), the collection-root item rule, F3 and the D8 split

Authority: root (ruling rj3-c, under the owner's delegation), after the #1441 attempt-3 verdict
(`/home/bl/misofm/submix-verdicts/1441-attempt3.md`) and the #1442 attempt-1 verdict
(`/home/bl/misofm/submix-verdicts/1442-attempt1.md`). Root adopted the fresh review's texts verbatim and chose
its Option A for MAJOR-1. This spec-only commit comes before attempt 2; the implementer does not write rule
text. Each item is placed where it belongs and marked "(root Amendment 3)":

- **MAJOR-1 (Option A)** D1's first bullet (text above), with these items: the D6 case
  `from-fn-closure-drain-is-refused`; the gate 5 mutation "restore the exception"; B1b-D1's
  `array::from_fn` outer-loop entry (the "bounding-loop entry" of the review's D10 wording) is removed; the
  base tree's `drain_lane_pairs` (`tests/fixtures/base/crates/builtins/src/lib.rs.txt`) is rewritten or its row
  flipped to refused. Option B (shadow scans of `core`/`std`) is refused by root: it cannot see a Cargo
  dependency key (`core = { package = "spin", .. }`, built and confirmed) and has an open splice gap.
- **SP-2** New D9: the collection-root item rule, the D6 case `unit-struct-shadows-the-collection-root`, the
  cases r01, r02, b01, b06 and the `<Q>`/`<K>` roots, and a gate 5 mutation.
- **F3** The D6 row of the nested closure (text in D6).
- **SP-B split** D8, its six D6 cases, its gate 5 mutation bullet and Amendment 2's SP-B line moved to #1519
  (a new issue, filed by root; three attempts). #1442 keeps (iv), SP-A, F2, D9, u19-swapped, F3, the counts and
  MAJOR-1.
- **#1418** is amended in the same commit: a drain inside any closure, `from_fn` included, is refused.
- **Module documentation.** The list of known false passes in the realtime-policy tool's module documentation
  (`tools/realtime-policy`, drain rule) is updated in attempt 2 so that it does not overclaim: the
  collection-root hole is a known false pass until D9 lands (rj3-c R1).

### Root Amendment 4 (2026-10-12): attempt 3 (the last), the owner's limit 4, and the retired trust rules

Authority: root ruling rj3-e (2026-10-12), after a fresh adversarial review of the attempt-2 head `d5b26071f`
(`rj3-d-review`), under the owner's decision of 2026-10-11 ("Add one stated limit": the tool refuses every
direct form, and names and file paths built by macro metaprogramming become one written limit, like registry
crates; runtime audits (0 allocations, 0 syscalls) stay the backstop). The limit is item 4 of the threat model
in #1438 Hazards. Root adopted the review's texts, with the cuts that limit 4 allows. This spec-only commit
comes before attempt 3; the implementer does not write rule text. Phrases marked "(placement)" only connect
root's text to this spec.

**Superseded by this amendment (placement).**
- D3's `.map(<path>)` allowance and the `<recv>.map_or(0, <Type>::available_at_entry)` count form (R1, R1b).
- Root Amendment 1 (iv), and root Amendment 2's SP-A (a), (b) and (c) (the earlier D3 (a) ratification is withdrawn),
  with their D6 cases and gate 5 mutations; SP-A (d) stays.
- D10's span for a `let` (R3), and `u19-swapped` (R5).
- The name-emitting set, its transitive closure and the rename tracking: delete that code.

**R1 and R1b. Text.**
> **D3 (root Amendment 4).** Function values: a path whose last segment is `try_pop` or `available_at_entry` is allowed only as the callee of a call. The `.map(<path>)` allowance and the `<recv>.map_or(0, <Type>::available_at_entry)` count form are withdrawn (root, 2026-10-11, #1442 attempt-2 F1, t5). A count is `<recv>.available_at_entry()[.min(<cap>)]`, and a pop is `<recv>.try_pop()`.

Facts the review measured: with R1 applied, `.map(|c| c.try_pop())` is refused by D1; the real tree refuses only
`crates/effect-contract/src/live.rs:366`; the 17 `let map` locals and F1a-c become moot; `per_width!`
(`crates/builtins/src/lib.rs:5851`) is not affected. Probe `t5` is a user trait `map_or` that returns `usize::MAX`;
the attempt-2 tool passes it.

**The `live.rs` rewrite belongs to #1345.** #1345 D8 is amended in the same commit with a pointer; #1426 D2 keeps
only `<recv>.try_pop()` and its `pop_receiver_differs_map` case is deleted in the same commit. J does not touch
`live.rs`. J's gate 1 refuses exactly `live.rs:366` until J rebases onto #1345, and the batch-merge gate then passes
with no exception. (Placement: the real-tree claim of gate 1 and of the rows below is measured on the tree rebased onto #1345.)

**Trusted names.** Q = {`try_pop`, `available_at_entry`}. The scan reads the tokens of every `.rs` file under the roots and refuses:
- (q) a Q identifier (bare or `r#`) that does not directly follow `.` or `::`, except exactly one written `fn` token each for `try_pop` and `available_at_entry`, in `crates/engine/src/realtime/spsc.rs` (a second one, in `spsc.rs` too, is refused; every other written definition is refused; a definition built by expansion is limit 4; t2, t7);
- (s) F2 for `debug_assert` only: a `debug_assert` identifier (bare or `r#`) not directly followed by `!` withdraws the trust from that name everywhere, and also withdraws B1a-D7. It is not generalized to other assert macros: with (m) cut, nothing consumes that trust. (Placement: this is the review's (s) with its set S reduced to `debug_assert`.)
- (u) a `macro_use` identifier, except the whole attribute `#[macro_use]` directly followed by `mod`;
- (n) any `no_implicit_prelude` identifier;
- (d) unchanged.

**B1a-D7 (amended):** only a single-identifier `debug_assert!` (no path, no leading `::`) is taken at its word (t6).

Macro-made shadows inside one crate, and globs under the implicit prelude, are E0659 (built). A Cargo dependency key or rename to `core`, `std` or `alloc` is closed by the manifest rule below. (Placement: in #1438 Hazards the trusted-name parenthetical reads "Q and `debug_assert`".)

**Manifest rule (named exception to this spec's authorized paths).** `scripts/check-workspace-policy.sh:161` refuses only package names `core|std|alloc|proc_macro|test`. It does not refuse dependency keys. No Cargo.toml has such a key. A naive regex hits `test = false` in `fuzz/Cargo.toml`, so restrict the rule to dependency tables. Close it with a rule in that script for dependency-table keys and `[*dependencies.core|std|alloc]` headers, by this named exception. No other edit to that script.

**R3. Text.**
> **D9 (amended).** For each binding a drain rule reads (count, alias source, slice parameter, collection root, `<Q>`, `<K>`), an item of any kind or a `use` that names it, and D10's statement or item macro or attributed item, is refused anywhere in its scope: for a `let`, the whole block that holds the `let`, from its `{`, at any depth; for a parameter, the whole function body; for a pattern, the whole node that binds it. This replaces D10's span for a `let`. A `let` whose identifier pattern names a unit struct or unit variant in scope binds nothing. Its value then has that item's type, which is limit 2 (as for a `let` of that type).

Measured by the review: with the scopes above, r04, r05, r07, r09 (also a head false pass), r11 and `pq_norounds_2` are refused;
`pq_norounds_1` stays accepted; real tree 0. D10's comment "An item in the let's own block cannot shadow it" is false for r13
(`struct controls; impl..; let controls = controls;`, region after) and r14 (the same struct made by a `mk!()` before the `let`):
both compile, loop 5001 times at the review's cap, pop 1000, and passed the attempt-2 tool. r15 (a unit struct at module level) is limit 2 in effect.

**R4. Text.** Refuse an identifier `feature` directly followed by a `(` group anywhere under the roots. Also refuse `$` directly followed by a `{` group (a metavariable expression), so that a later stabilization of `${concat}` on a toolchain bump is still refused.
The review confirmed that `scripts/check-workspace-policy.sh` does not forbid `#![feature`, and that `#![feature(` as a token sequence misses `#![cfg_attr(x, feature(..))]`. Real tree: 0 for both.

**R5. Case `decrement-after-a-continue`** (refused):
`let mut remaining = control.available_at_entry(); while remaining != 0 { let Ok(record) = control.try_pop() else { break; }; if skip { continue; } remaining -= 1; apply(record); }`
The attempt-2 tool refuses it. Under the mutation "first binary statement" it is accepted (built). The control form, with the decrement first, is accepted.
Test value: red if `while_bounds` accepts a decrement that is not the body's first statement, which lets a `continue` before it skip the decrement and pop without bound. The existing `while-decrement-not-first` puts the decrement in an `if`, so it stays green under that mutation.
Delete `u19-swapped`, because this case supersedes it.

**Spec items to delete or replace.**
- The accepted D6 `.map` case flips to refused.
- Delete the (a) tests, the (c) name-emitting and rename tests, `try-pop-by-value-on-a-reference-impl` (superseded by (q)), and the (a), (b) and (c) mutations, plus gate 5's ".map refused" and "any .map receiver".
- Rewrite `the_trusted_names_in_their_allowed_forms_pass`: drop its `impl Consumer { fn try_pop }`, `.map` and `let map`.
- Keep the 8 F2 shadow rows and `proc-macro-fn`.
- Delete the name-emitting set, the transitive closure and the rename tracking.

**New D6 cases.** Each is refused unless it says "accepted". Each has its test value.
- **C1** `map-path-on-as-mut` (k0): red if `.map(<path>)` on `<fields>.as_mut()` is accepted again.
- **C2** `map-or-count-form`: red if the `map_or(0, <Type>::available_at_entry)` count is accepted again.
- **C4 / C5** (t2 / t7): red if a definition of `try_pop` / `available_at_entry` outside spsc.rs is allowed.
- **C10** (t6): red if D7 matches by the last segment.
- **C11** `#[macro_use] extern crate x;` is refused. Red if (u) is dropped. Its accepted half, `#[macro_use] mod`, is red if the exemption is dropped (the real tree, `crates/math/src/lib.rs:52`).
- **C12** `#[no_implicit_prelude]` is refused. Red if (n) is dropped.
- **C13** the `cfg_attr` form of `feature(..)` is refused. Red if the rule matches only `#![feature(`.
- **C14** r04, r05, r07, r09, r11 and `pq_norounds_2`. Red if the scan stops at the region. r13 and r14 are red if a `let`'s scope starts at the `let`.
- **C15** the R5 case above.

**Gate 5 mutations (amended).** Apply each alone. The real tree must pass, measured on the tree rebased onto #1345. (The numbers are the review's; 5-8 are cut with their cases.)
1. Restore `.map(<path>)`: C1 goes red.
2. Restore the `map_or` count form: C2 goes red.
3. (q) allows any `fn try_pop(&mut self`: C4 goes red.
4. `available_at_entry` is not in Q: C5 goes red.
9. D7 matches the last segment: C10 goes red.
10. (u) is dropped: C11's refused half goes red. The exemption dropped: C11's accepted half goes red, and so does the real tree at `math:52`.
11. (n) is dropped: C12 goes red.
12. `#![feature(` matched only as a token sequence: C13 goes red.
13. The `let` scope runs from the `let` (today's D10 span): r13 and r14 go red. Region leaves only: r04, r05, r07, r09, r11 and `pq_norounds_2` go red.
14. The first-statement rule relaxed to the first binary statement: C15 goes red.

**Real tree, measured** (the review's rows, on the attempt-2 head; the implementer re-measures on the rebased tree, gate 1).
| Rule | Refusals |
|---|---|
| Head tool (`d5b26071f`) | 0 (94 regions in 26 files) |
| R1 alone | 1 (`live.rs:366`); 0 after #1345's rewrite |
| R1b (`map_or` withdrawn) | 0 after #1345's rewrite |
| Replacement R2 ((q), (s), (u), (n), the single-segment D7) | 0 |
| R3 (whole scope) | 0 |
| R4 | 0 |
| R5 | 0 |

Also measured: only 1 `fn try_pop` (`spsc.rs:441`) and 1 `fn available_at_entry` (`spsc.rs:421`) exist; there are 0 `macro_rules! $`, 0 `#![feature`, 0 `no_implicit_prelude` and 0 `${`.

**Limit 4 evidence (recorded; none of these is a FAIL).** These probes build a name or a path by expansion, and the owner's limit 4 (#1438 Hazards, item 4) puts that outside the tool's claim:
- t1: `mkp!(fn x.try_pop());` with `($k:tt $x:ident . $n:ident ()) => { .. $k $n(&mut self) {while ..} }`: a looping `try_pop` made by expansion (rustc pops 1000 of 1000).
- t3a: `mk!($ ::debug_assert!)`, then `#[macro_export] macro_rules! $n`.
- t4: a macro made by `make!($ innocent)`, whose name is in no set the scan reads (rustc pops 1000 of 1000).
- The stable `path` splice vectors of the #1519 rule-1 review: V3 `m!(path, =, "y.rs")` into `#[$a $b $c]`; V4 `outer!(path, "y.rs")` with `inner!($p = $l)` into `#[$m]`; V8 `m!(#, [path = "y.rs"])` into `$h $g mod x;`; W3 a transcriber `$h [path = "y.rs"] mod x;` with `m!(#)`; R1/R2 a group first in a repetition, `# $( [path = "y.rs"] mod $a; )*`.
- The nightly vectors N1 to N3 of the same review (`${concat(..)}`, `macro_attr`, `macro_derive`) are refused by R4: each needs `${` or `#![feature(..)]`.

## Authorized paths

- `tools/realtime-policy/**`
- This spec
- `scripts/check-workspace-policy.sh`, for the manifest rule only (named exception, root Amendment 4)

## Non-goals

- B2b-1's non-test pop rule, allowlist and markers; B2b-2's call rule.
- #1418's and #1426's rules.
- Committing the fuzz files (PR evidence only; root accepted this, 2026-10-05).

## Hazards

- **The probe sources are in the bundle.** Copy their content into case fixtures. Do not read
  them from `/tmp`.
- **Over-refusal is acceptable, a false pass is not.**
- **Threat model (root Amendment 2, 2026-10-11).** The tool's threat model, with its closed list of
  limits, is the "Threat model (root ruling 2026-10-11)" bullet in #1438 Hazards. D3's trusted-name
  scan and D2's replacement rule are how this slice meets it (D8 moved to #1519); a registry trait or type method
  named `map` or `as_mut` is a stated limit (limit 1). A finding inside a listed limit is recorded,
  not a FAIL, and only a root ruling adds or widens a limit.
- A type with its own `as_mut` that returns an iterator would let `.map` pop once per item; the
  tool cannot see types. No `fn as_mut` is defined in the workspace. (root Amendment 1, iv)
- As in A1: the awk gate scans the tool's own sources until C2.

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints the same counts
   as before this slice. `bash scripts/check-realtime-policy.sh` passes on the same tree.
   (root Amendment 2) D2's replacement rule and D3's trusted-name scan refuse nothing on the real
   tree (measured on `main` and on `1a33b2f6d`); the implementer re-measures with the tool and the
   full suite.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every earlier case and
   every D5 and D6 case.
3. **Parity (PR evidence).** A throwaway script outside the repository runs `main`'s awk gate
   (gawk) and the tool on each D5 probe. The verdicts agree with the TSV, except for the D1 and D2
   probes, u03's repaired form, u19 and v14/v14b (root Amendment 1). Name each difference.
4. **Fuzz (PR evidence).** Run the tool's drain rule over the ten fuzz files: 3767 regions are
   refused and 800 are accepted.
5. **Mutations (PR evidence).** Apply each alone. Each turns red the named cases, and the real
   tree still passes:
   - closures transparent again: a06, a07, b02, n09, e11 and the D6 closure case;
   - macro token trees skipped: the D6 macro case;
   - `debug_assert` always taken at its word: e07, the cross-file case and the `use` case;
   - the shadow check limited to the file of the drain: the cross-file case and the `use` case;
   - any `try_pop` path value allowed: the `let pop` case;
   - `.map(<path>)` refused: turns the accepted `.map` case red. This mutation is exempt from
     "the real tree still passes": the real tree (`live.rs:366`) and the base tree hold the same
     form, so their failure is further evidence that the mutation bites. (root Amendment 1, iii)
   - any `.map` receiver: turns `function-value-in-map-on-an-iterator` and
     `function-value-in-map-on-repeat` red; the real tree passes. (root Amendment 1, iv)
   - (root Amendment 2, SP-A) One mutation for each of D3's (a) to (d), each alone, the real tree
     still passing:
     - (a) dropped: turns `user-map-trait-in-a-nested-module` red;
     - (b) limited to a `&self` receiver: turns `try-pop-by-value-on-a-reference-impl` red;
     - (c) dropped, or matching only the text `fn map`, or not closed transitively: turns
       `macro-made-map-name` and `macro-passes-name-to-an-inner-name-emitting-macro` red;
     - (d) dropped: turns `macro-made-proc-macro` red.
   - (root Amendment 2, F2) The replacement rule removed: turns the 8 shadow rows red; the real tree
     passes.
   - (root Amendment 2, SP-B; removed by root Amendment 3) D8's mutations moved to #1519.
   - (root Amendment 3, MAJOR-1) Restore the exception (a closure of `core::array::from_fn` or
     `std::array::from_fn` is again a finite outer loop around a pop): turns `from-fn-closure-drain-is-refused`
     red; the real tree passes (no marked region pops in such a closure).
   - (root Amendment 3, SP-2; the mutation is the placement of the rule's own words) The item rule limited to the
     count, its alias source and the slice parameter, not the collection root or the `<Q>`/`<K>` roots: turns
     `unit-struct-shadows-the-collection-root` and r01, r02, b01, b06 red; the real tree passes.
6. **Shipped artifacts unchanged.** As A1's gate 6.
7. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings` and
   `bash scripts/check-workspace-policy.sh` exit 0.

*Test value.*
- The D1, D2 and D3 cases are red if a pop in a closure, a macro token tree or a function value
  passes again. No B1a or B1b case reaches these: they port them as limits.
- The cross-file and `use` cases are red if the `debug_assert` shadow check looks at one file
  only, which is how e07's shape still passes across files.
- The refused probes are red if the visitor skips the construct each one holds:
  - a literal form, a let chain, an or-pattern or a struct pattern;
  - a labelled, `const` or `unsafe` block, or a closure in a header;
  - a `cfg` attribute.

  Those are the positions where a `syn` walk that misses a `visit_*` override is silent.
- The accepted probes are red if the port refuses a bounded form that `main` accepts:
  - an early return, a capped count or a typed binding;
  - `while >`;
  - a `zip` or slice-parameter outer loop;
  - an indexed receiver or an attributed loop;
  - a `debug_assert!` that names the count, with no shadow (`v13_debug_assert`).
- Lexer-era probes that a `syn` tool cannot get wrong (u01, u02, u04 and u15-u18) are committed
  as a regression corpus. Root's ruling of 2026-10-05 ("every compiled probe ... becomes a
  committed case") is the authority for them.

## Evidence

- Gates 1-7 output. Gate 3's table and the probe-to-case map. Gate 4's counts. Gate 5's runs.

## Dependencies

- After (same stream): B1b.
- Before: C2.

## Standing rules for the implementer

- Work only from this body. Read the cited lines first; do not survey the workspace.
- A test that greps source or prose is refused. The cases are the tool's own input.
- Attempt budget: three attempts, one adversarial verdict each.
- Size: half a day. The rules (D1-D3) are small. The probe cases are mechanical: a throwaway
  script outside the repository writes each case file from the bundle (PR evidence, not committed),
  and the expected class sets are fixed by D5, not judged case by case.

## Attempt record (attempt 1, J batch branch `codex/d15-stream-j3`, base `d08217cef`)

Commits: `bfed03af6` (rules, cases, corpus) and the evidence commit that holds this record. Edits:
`tools/realtime-policy/**` and this spec only. No production code, `Cargo.toml` or `Cargo.lock` change; no
shipped crate depends on `realtime-policy` (`cargo tree -i realtime-policy` lists only itself), so gate 6 holds
trivially.

**What is where.**
- `src/drain/mod.rs` (`bounded`): a `PopKind::MacroToken` site is refused first (D2). `in_closure` refuses a pop
  below any closure step that is not the closure of `core::array::from_fn` or `std::array::from_fn` (D1; that
  closure stays B1b's finite outer loop). `called_in_place` allows a `PopKind::Path` site only as operand 0 of a
  `Call`, or as operand 1 of a `.map(..)` method call with one argument (D3). Module documentation: the closure,
  macro and function-value limit is gone; D1 to D3 and D2's stated limit are in (D7).
- `src/drain/shadow.rs` (new, D2): a token scan of one file for `macro_rules! debug_assert`, `macro debug_assert`
  and any `use` item that holds the identifier `debug_assert` (before its `;`, at any depth of the tokens; `use<..>`
  precise capturing is skipped). `src/lib.rs` runs it on every lexed `.rs` file under the roots, then judges the
  files that need the syntax tree (the marked files and `capi/src/ffi.rs`), which now wait for the whole walk.
  `src/drain/facts.rs`: `Facts::of(file, trust_debug_assert)`; untrusted, a `debug_assert!` is any other macro
  (its identifiers are macro uses, so a count named in it is refused).
- Deleted in the same commit (superseded, as the earlier slices' records say):
  `form-pop-token-in-a-macro-in-a-counted-loop` (a row of `the_forms_that_d6_and_d8_state_pass`) and
  `a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it`.
- `tests/policy/drain/probes.rs` + `tests/fixtures/probes/<group>/<name>.<verdict>.rs.txt` (118 files): the
  corpus. The verdict in the file name is the expected class set: `drain`, `accepted`, `forbidden-body+drain` (u08),
  `unsafe+drain` (u13). Each file replaces `fn drain_matrix_controls() {}` in the base tree with its own marker
  lines removed. `a2-rc/` holds the three shapes of `rc/shapes.rs`. u03 is committed with `//` in place of `//!`.
- `tests/policy/drain/indirect.rs`: the D6 cases (8 tests).

**Real tree (re-measured).** `cargo run --locked --release -q -p realtime-policy` and
`bash scripts/check-realtime-policy.sh`: `realtime policy: ok (94 marked regions in 26 files)`, as before the
slice. No marked region pops in a closure, a macro or through a function value (the tool passes). No file under
the roots defines or imports a `debug_assert` macro. Release run time 1.27 s (B1b: 1.05 s; the second pass and
the token scan of every file add about 0.2 s). Not tuned.

**Deviations from the spec's numbers and decisions.**
1. **Corpus size.** Committed: 86 refused and 32 accepted probes (118). The spec's 84 + 6 refused and 33 accepted
   change as follows. (a) `e02_cfg_count`, `e03_slice_param_shadowed_by_block_const` and `e06_raw_keyword_macro`
   are token-for-token the ported cases `count-under-cfg`, `outer-over-slice-parameter-shadowed-by-block-const`
   and `count-in-raw-identifier-macro`: not committed twice (D5). (b) `v14_len_outer` and `v14b_len_outer` are
   not committed: the tool refuses `0..self.controls.len()` as an outer bound where the awk gate passes (B1b
   deviation 1, still an open item for root). Committing them as accepted needs `outer::range_to_a_bound` widened;
   committing them as refused would pin an over-refusal. Open item below. (c) `u19_while_count_alias_twice` is
   committed as `accepted`, not `drain`: the drain is bounded (`let remaining = 5;` after the decrement shadows
   only the rest of the loop body; the header and `remaining -= 1` read the outer binding) and the awk refuses it
   by over-counting aliases. D5's {drain} for u19 is wrong; the tool is right.
2. **Gate 5, `.map(<path>)` refused.** The real tree does not pass under this mutation: `live.rs:366` is exactly
   that form. The base tree fails too (its `effect-contract/src/live.rs`), so the mutation is red in the whole
   suite, not in one case. The D6 `.map` case is among the red rows.
3. The refused probes do not catch a visitor that skips a node: an unreached pop is refused (B1a-D4, fail-closed).
   The spec's test-value sentence says they do. The accepted probes catch the skip (mutation C3 below); the
   refused probes catch a rule that accepts too much (C4, C5). The test docs say so.
4. `Tree::apply` is unchanged; `each_row` takes `&'static str`, so the probe names are leaked in the test.

**Gates.**
1. Real tree: both gates `ok (94 marked regions in 26 files)`.
2. `cargo test --locked -p realtime-policy`: 140 passed (131 before; -1 test, -1 row, +2 corpus, +8 indirect).
3. Parity (throwaway script outside the repository: base tree + each probe, `scripts/check-realtime-policy.sh`
   under gawk, and the release binary), run on the 123 probes generated from the bundle (the 118 committed, the
   three duplicates and v14, v14b). The gawk verdicts equal the TSV on all 123. The tool differs on 9:
   a06, a07, b02, n09, e07, e11 (awk passes, tool refuses: D1 and D2, as intended); v14 and v14b (awk passes, tool
   refuses: B1b deviation 1); u19 (awk refuses, tool accepts: above). u03 repaired: both refuse.
4. Fuzz (throwaway; each file split into its regions, regions the parser rejects removed one at a time, the rest
   judged in one file): fz_1-4 and fz_11-14 hold 3767 regions: 3762 refused with the drain class, 1 does not parse
   (`fz_2`, refused as a parse error), 4 accepted. The 4 (`fz_1` s249, `fz_11` s170, `fz_14` s229, `fz_3` s63) are
   `for/while <header>.. { body } { <pop block> }`: the generator wrote the pop in a bare block that follows the
   loop, because a range's `..` before `{` takes no block as its end; the pop is under no loop, so it is bounded.
   (The awk's lexer reads that block as the loop's body.) fz_21 and fz_22 hold 800 regions: 0 refused with the
   drain class; 125 are refused as unsafe code (the generator uses `unsafe { }` in headers), the class `main`'s
   unsafe pass also reports.
5. Mutations: below.
6. Shipped artifacts: unchanged (above).
7. `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `scripts/check-workspace-policy.sh`, `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh`
   exit 0.

**Mutations** (each alone, `cargo test` and the real tree after each, source restored by the harness; 140 green and
the real tree ok after every revert). Red = tests and rows.
- Closures transparent again: `closure-in-a-let-called-in-a-loop`, `-called-once`, `-in-a-counted-loop`,
  `-argument-of-an-adapter`, and probes a06, a07, b02, n09, e11. Real tree ok.
- Macro token trees transparent (the `MacroToken` refusal removed): `macro-call-in-a-counted-loop`,
  `macro-call-with-no-loop`, `debug-assert-argument`, probe u09. Real tree ok.
- Macro token trees skipped (pop finder ignores macros): the five macro rows (the body, both calls, no loop,
  `debug_assert`), u09, and three B1a/B1b macro cases. Real tree ok.
- `debug_assert` always taken at its word: probe e07, all 8 `a_debug_assert_that_a_file_shadows..` rows and the
  cross-file case. Real tree ok.
- Shadow check limited to the file of the drain: the cross-file case and the 8 shadow rows (e07 stays green: its
  macro is in the drain's file). Real tree ok.
- Any `try_pop` path value allowed: the 7 `function-value-*` rows (the `let pop` row first). Real tree ok.
- `.map(<path>)` refused: see deviation 2 (the `.map` case, every test that builds the base tree, and the
  real tree at `live.rs:366`).
- Shadow internals, one red test each: braced `use` group ignored (`use-in-a-group`); no recursion into groups
  (`macro-rules-in-a-function-body`, `macro-rules-in-a-macro-body`); `use<..>` not skipped
  (`precise-capturing-bound`); decl macro arm removed (`declarative-macro-2`); `use` scan runs past `;`
  (`use-then-a-call-of-debug-assert`).
- Path rule internals, one red row each: `.map` with any argument count (`function-value-in-map-with-another-argument`);
  any method name for operand 1 (`function-value-in-and-then`); any call operand (`function-value-as-an-argument`).
- `array::from_fn` exemption removed from the closure rule: the base tree and the `outer-*-array-from-fn` rows.
- Corpus: `match` arms not walked: probes v01 and v07 (accepted); `let` expressions not walked and collection
  outer loops never finite: the base tree and all 32 accepted probes; any loop bounds a pop
  (C4): 57 refused probe rows; unreached pops accepted (C5): the u08 probe. The real tree stays ok except under
  "collection outer loops never finite", which refuses `lib.rs:1128`.

**Test value (one sentence per new test).**
- `a_pop_in_a_closure_is_refused`: red if a pop that runs wherever its closure is called passes (a closure in a
  `let` called by a loop, called once, in a counted loop, or held by an adapter). B1a's transparency passed them.
- `a_pop_in_a_macro_token_tree_is_refused`: red if a `try_pop` token in a macro call, a `macro_rules!` body or
  `debug_assert!` arguments is judged by the loops around the call.
- `a_function_value_that_pops_is_refused_unless_it_is_called_in_place` and
  `a_function_value_called_in_place_is_judged_by_its_loops`: red if a stored, passed or parenthesised `try_pop`
  value passes, or if the callee and the `.map(..)` argument are refused or stop being judged by their loops.
- `a_debug_assert_that_a_file_shadows_is_not_taken_at_its_word` and
  `a_debug_assert_shadowed_in_the_parent_module_is_shadowed_in_the_child`: red if the shadow check misses a form
  (`use`, rename, group, nested or local `macro_rules!`, decl macro) or looks at one file only.
- `text_that_does_not_shadow_debug_assert_leaves_it_trusted`: red if a string, a comment, a longer name or a
  `use<..>` bound withdraws the trust from the valid drain.
- `the_refused_probes_are_refused`, `the_bounded_probes_pass`: red if a rule accepts a drain the verdicts refused,
  or refuses a bounded form `main` accepts, in the positions the probes hold (C3, C4, C5, M1, M2, M3 above).

**Open items.**
- Root: B1b deviation 1 (`0..x.len()` and bracket-index outer bounds). When widened, commit `v14` and `v14b`
  (`/tmp` copies are not needed: they are `v1302/a3/v/v14_len_outer.rs` and `v14b_len_outer.rs` in the bundle)
  as accepted cases.
- Root: u19 (deviation 1c) and the 4 accepted fuzz regions (gate 4) are awk over-refusals the tool does not repeat.
- A `.map(Consumer::try_pop)` on an iterator of `&mut Consumer` is allowed by D3 as written. Such an iterator
  yields each consumer once, so it is finite; no infinite form is reachable without `from_fn`, `repeat_with` or
  `successors`, which stay refused. (Attempt 2: this sentence is false, as the attempt-1 verdict F1 showed. A user
  `map` or a `try_pop(&self)` type with `repeat` pops without end. Root Amendment 1 (iv) and Amendment 2 (SP-A)
  replace the rule.)

## Attempt record (attempt 2, J batch branch `codex/d15-stream-j3`, base `c8d971814`)

Commits: the code commit and the evidence commit that holds this record. Edits: `tools/realtime-policy/**` and this
spec only. No production code, `Cargo.toml` or `Cargo.lock` change (gate 6 holds as in attempt 1).

**Deviation for root (D3 (a), read first).** The text of (a) refuses every identifier `map` or `as_mut` that does not
directly follow `.` or `::`. On the real tree that refuses 17 local variables named `map`
(`tools/console-workload/tests/paired_spans.rs:2061,2070,2071,2074,2075,2123,2124,2125,2129,2402,2403,2413` and
`tools/session-validator/src/lib.rs:793,794,807,808,821`), so the gate fails with the text as worded. The spec's
measurement counted `.map(` and `.as_mut(` occurrences, not every `map` identifier. I did not edit those two files
(outside the authorized paths). I implemented (a) for the forms that define or import a name: after `fn`, `const`,
`static`, `struct`, `enum`, `union`, `type`, `trait`, `mod`, `macro`, `as` or `macro_rules!`, and anywhere in a `use`
item, in each case when no `.` or `::` directly precedes it (`trusted.rs`, `defines_a_name` and `imported_names`).
A local variable cannot change what `.map(..)` resolves to. Root may (1) accept this, or (2) rule the literal text and
have the 17 locals renamed in a production-adjacent change, which flips one accepted row
(`the_trusted_names_in_their_allowed_forms_pass`, the `let map` line) and deletes `defines_a_name`. Rules (b), (c),
(d) and F2 are implemented as worded and refuse nothing on the real tree.

**What changed.**
- D1 (Option A). `in_closure` refuses a pop below any closure. The `array::from_fn` closure exception is deleted
  from `mod.rs`, `outer.rs` and `tree.rs` (`array_from_fn_call`, `is_loop_at`); `constructor_lines` keeps the token
  allowance for the two paths. The base tree's `drain_lane_pairs` is a plain `for lane in 0..2` loop (same count
  comment between the binding and the loop). The probe `a2-n/n06_array_from_fn_bounded` is a counted drain in a
  `from_fn` closure: it is committed as refused now (`.drain`), which the spec's counts did not foresee.
- D2 F2 and D3 SP-A. `src/drain/shadow.rs` and its enumeration are deleted. `src/drain/trusted.rs` is one token
  walk over every `.rs` file: a `debug_assert` that no `!` follows is a shadow; (a), (b), (d) are findings of the
  drain class at the identifier's line; (c) records every invocation `name!(..)` whose tree holds `map`, `as_mut`,
  `try_pop` or `debug_assert`, and `finish` reports those whose macro is name-emitting (a body with `fn $`, closed
  over the macros it invokes and over `use .. as n` renames). A hit is a standalone finding of the drain class (the
  threat model says the scan refuses the definition), not a trust withdrawal; F2 stays a trust withdrawal, so the
  existing shadow rows keep their two-finding assertion.
- D3 (iv). `called_in_place` allows `.map(<path>)` only when the receiver is `<fields>.as_mut()`, where `<fields>` is
  an identifier or `self` followed by zero or more `.field` steps (`Place::of`), and the call has no turbofish.
- D9. `Ctx::note_read` stores the name with the binding. `shadowed_by_an_item` (renamed from `declares_an_item`)
  asks `names_item(name)` for every read, then D10's macro span. New reads: the root of an outer `for`'s collection
  (`outer::read_root`; it must resolve to a binding, `self` is skipped), and `<Q>` and `<K>` of a per-queue form
  (`per_queue::Pass::holds`, with a rewind per failed form). `BindingKind::Pattern` carries its owner node so that a
  pattern-bound root has a span (the binding node). The scattered `names_item` calls in `count.rs` and `outer.rs`
  are deleted: the one check covers them.
- Module documentation of `drain/mod.rs`, `outer.rs`, `items.rs`, `scope.rs`, `trusted.rs`: the closure exception,
  the collection-root hole and the `.map(..)` on any receiver are gone; the list of known false passes keeps only the
  threat-model limits (never-ending iterator in a field or local, registry crates, auto-deref and impl resolution).
- Tests: 149 before, 163 now (+15 tests, -1). Deleted as superseded: the three `outer-*-array-from-fn` accepted
  rows, the two `outer-array-from-fn-*` refused rows, `a_pop_directly_in_an_array_from_fn_closure_is_refused`.

**Real tree (re-measured).** `cargo run --locked --release -q -p realtime-policy` and `bash scripts/check-realtime-policy.sh`:
`realtime policy: ok (94 marked regions in 26 files)`, as before. Release run 1.31 s (attempt 1: 1.27 s), 214 MB.

**Gates.**
1. Real tree: above. 2. `cargo test --locked -p realtime-policy`: 163 passed.
3. Parity (throwaway script outside the repository, base tree + each of the 120 committed probes, awk under gawk and the
   release tool). The tool equals the expected class set of every file name. Awk and tool differ on 10, all named:
   a06, a07, b02, n09, e11 (D1), e07 (D2), v14 and v14b (SP-4: awk passes, tool refuses), n06 (D1 Option A: awk
   passes, tool refuses), u19 (awk refuses, tool accepts). u03's repaired form is refused by both.
   **Corpus: 120 files = 89 refused + 31 accepted**, not 88 + 32: n06 moves from accepted to refused (D1).
4. Fuzz (throwaway, as in attempt 1): fz_1-4 and fz_11-14: 3767 regions, 3762 refused with the drain class, 1 parse
   failure (`fz_2` s125), 4 accepted (`fz_1` s249, `fz_3` s63, `fz_11` s170, `fz_14` s229: the pop is in a bare block
   after the loop; attempt 1 explained them). fz_21 and fz_22: 800 regions, **106 refused with the drain class** (58 +
   48), 694 accepted, and 125 also refused as unsafe code. The spec says 800 accepted. All 106 hold a statement macro
   `m! { x }` or an attribute in the span, so they are refused by #1441's D10. I ran the same script on the release
   tool built from `299cdafbd` (this slice's base): the numbers are identical (58 and 48), so this slice moves none.
5. Mutations, below. 6. Shipped artifacts: unchanged. 7. `cargo fmt --all -- --check`,
   `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`, `scripts/check-workspace-policy.sh`,
   `scripts/check-realtime-policy.sh` and `scripts/test-realtime-policy.sh` exit 0.

**Mutations** (each alone; after each the suite is green again on revert; real tree `ok` unless stated).
- Closures transparent again: the five closure rows (F3's row among them), the five `from_fn` rows, `array-from-fn-pop`,
  a06, a07, b02, n09, e11, n06.
- Restore the exception (a `core`/`std::array::from_fn` closure is a finite outer loop again): the four `from_fn`
  drain rows and n06. Restore it as "any `from_fn` closure on the path exempts every closure": those and F3's
  `closure-in-a-nested-closure-in-array-from-fn`, which is red only here (the old mutation of F3 is gone with the
  exception; this is its Option A form).
- Macro token trees allowed: `macro-call-with-no-loop`, `debug-assert-argument`, u09.
- `debug_assert` always trusted: the 8 shadow rows, the parent-module case, `untrusted-debug-assert-in-the-loop`, e07.
  Shadow check limited to the drain's file: the same, e07 excepted.
- F2 dropped: the same 8 rows, parent-module case, untrusted case, e07. F2 dropped alone, and (d) dropped alone:
  `a_proc_macro_named_debug_assert_is_refused` stays green; both dropped: it is red.
- Any `try_pop` path value allowed: the 12 function-value rows.
- `.map(<path>)` refused: 113 tests red (the base tree holds `live.rs:366`'s form) and the real tree refuses
  `live.rs:366` (exempt, root Amendment 1 iii). Any `.map` receiver: the iterator, `repeat`, `cycle`, `as_ref` and
  call-result rows. Receiver method `as_ref` accepted besides `as_mut`: the `as_ref` row.
- (a) dropped: `user-map-trait-in-a-nested-module`, `fn-as-mut`, `fn-map-in-an-impl`, `const-named-as-mut`,
  `macro-named-map`. (a) after `fn` only: `const-named-as-mut`, `macro-named-map`. `use` items not read:
  `use-of-map-in-a-group`.
- (b) dropped, and (b) as "refuse only `&self`": `try-pop-by-value-on-a-reference-impl`. (The test panics at that
  first assertion, so the three rows after it were not run under these two mutations.)
- (c) dropped, and (c) as the text `fn map` only: `macro-made-map-name`, the inner-macro case, the rename case. Not
  transitive: the inner-macro case. Renames not followed: the rename case.
- (d) dropped: `macro-made-proc-macro`. (d) for `proc_macro` only: `proc-macro-attribute`, `proc-macro-derive`.
- D9, collection roots not noted: `unit-struct-shadows-the-collection-root` (r01), `slice-parameter-collection-root-...`
  (b06), the `&controls`, `&cfg.controls` and pattern rows, and the macro rows r02 and b01. Only the `.iter_mut()`
  arm not noted: r01, b06, r02, b01. Only the `&` arm not noted: the const, `cfg` and pattern rows. `<Q>` and `<K>`
  not noted: the per-queue item test and both per-queue macro rows. Pattern binding has no span: the accepted
  `collection-root-of-a-pattern` row. The item check dropped: 6 tests including the count rows.
- u19-swapped: red only when both "the loop body opens with the decrement" and the binding-aware use resolution are
  dropped (`while-first-stmt-any-position` alone leaves it green: the resolver still refuses the decrement of the
  inner `remaining`). One defect alone does not turn it red.

**Deviations from the spec's numbers and cases.**
1. D3 (a): above.
2. Corpus 89 refused + 31 accepted (n06).
3. `user-map-trait-in-a-nested-module` applies the trait beside the accepted form `self.control.as_mut().map(..)`, not
   F1's `control.map(..)` (which D3 (iv) refuses on its own). `macro-made-proc-macro` invokes `mk!(emit !)`, not
   `mk!(debug_assert !)`: with `debug_assert` in the invocation, (c) refuses it as well and the (d) mutation would stay
   green. F3's row is the spec's second form (a counted drain in a nested closure in the `from_fn` closure), under the
   same name.
4. Gate 4 bounded fuzz: 694 accepted, not 800 (#1441's D10; identical on `299cdafbd`).
5. A trusted-name hit is a standalone drain-class finding at the identifier; the spec says "refuses" and names no
   class. The message text is the drain class's.
6. The unit-struct-shadows-the-collection-root row is the r01 shape (the spec calls them the same shape); rows that
   no single mutation turns red (`use`, `fn`, after-the-loop, glob-use collection-root rows; `free-fn-map`,
   `use-renamed-to-map`) were deleted.

**Test value (one sentence per new test).**
- `a_drain_in_the_closure_of_array_from_fn_is_refused`: red if the `array::from_fn` outer-loop exception returns, in any
  path spelling (restore-the-exception mutation).
- `a_pop_in_a_closure_is_refused` (F3 row): red if a closure inside the `from_fn` closure is exempted with it.
- `a_function_value_that_pops_is_refused_unless_it_is_called_in_place` (new rows): red if `.map(<path>)` is allowed on a
  receiver other than `<fields>.as_mut()`, or on `as_ref`, or on a call result.
- `a_proc_macro_named_debug_assert_is_refused`: red only if F2 and (d) are both dropped.
- `a_user_trait_with_a_map_in_a_nested_module_is_refused`: red if the scan reads one file or only top-level items, or drops (a).
- `a_definition_import_or_rename_of_map_or_as_mut_is_refused`: red if (a) names only `map`, looks after `fn` only, or skips `use` items.
- `a_try_pop_taken_by_value_on_a_reference_impl_is_refused`: red if (b) allows a receiver other than `&mut self`.
- `a_map_name_that_a_macro_makes_is_refused`, `a_name_passed_to_an_inner_name_emitting_macro_is_refused`,
  `a_name_emitting_macro_invoked_through_a_rename_is_refused`: red if (c) is dropped, matches only `fn map`, is not
  transitive, or does not follow renames.
- `a_proc_macro_that_a_macro_makes_is_refused`: red if (d) is dropped or names only `proc_macro`.
- `the_trusted_names_in_their_allowed_forms_pass`: red if the scan refuses `.map`, `.as_mut`, the `Consumer::try_pop`
  definition, a macro that emits no name, or (deviation 1) a local variable named `map`.
- `an_item_that_declares_the_collection_root_refuses_the_drain`: red if the item rule reads the count, its alias source
  and the slice parameter but not the collection root of an outer `for` (either arm, any binding kind).
- `a_collection_root_that_no_item_names_passes`: red if the root rule over-refuses an unshadowed root, a pattern root or `self`.
- `a_statement_macro_that_can_declare_a_collection_root_refuses_the_drain`: red if a root, or `<Q>` or `<K>`, is not a
  read for D10's span (r02, b01 and the per-queue macro forms).
- `an_item_that_declares_a_per_queue_root_refuses_the_form`: red if the item rule skips `<Q>` and `<K>`.
- `a_decrement_that_a_body_let_has_rebound_is_refused`: red if a body `let` that rebinds the count is not read as a
  rebinding by the use resolution (only together with a relaxed first-statement rule; see the mutation note).
- `the_refused_probes_are_refused`, `the_bounded_probes_pass` (v14, v14b, n06 moved to refused): red if the outer
  bound accepts a method call or an index without a ruling, or the `from_fn` closure exception returns.

**Open items.**
- Root: D3 (a) as worded (deviation 1).
- Root: spec counts 88/32 and gate 4's "800 accepted" are stale (deviations 2 and 4).
- The scripts/test-realtime-policy.sh base tree still holds the awk-era `drain_lane_pairs` closure (outside this
  slice's paths; the awk accepts it until C2 deletes the script).

