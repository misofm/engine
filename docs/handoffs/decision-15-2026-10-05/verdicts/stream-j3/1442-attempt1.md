FAIL

# #1442 attempt 1 verdict (B2a: pops in closures, macros and function values; probe corpus)

Range `d08217cef..1a33b2f6d` on `codex/d15-stream-j3`. I exported the head to
`/tmp/claude-1002/v1442/tree` and built it with `CARGO_TARGET_DIR=/tmp/claude-1002/v1442/target`.
I did not touch the worktree. The evidence scripts and probes are in
`/home/bl/misofm/submix-verdicts/evidence/1442-attempt1/`.

D1, D2 as enumerated, the corpus, the dedup, u19, the fuzz counts, the gates and the mutations are
all correct, and I reproduced them. The attempt fails on F1. The tool accepts a pop through a
function value that pops until the queue is empty. The code compiles today, and the documentation
says it cannot happen. The cause is in the spec: D3 allows `.map(<path>)` on any receiver. The
correction needs a D3 amendment from root before attempt 2 (SP-A).

## MAJOR

- **F1. D3's `.map(..)` exception trusts the method name `map`. A user-defined `map` drains without
  a bound and passes** (`tools/realtime-policy/src/drain/mod.rs:241-254` `called_in_place`, and the
  module doc at `mod.rs:39-43`).
  - Reproducer (evidence `m.rs`, rustc 2024): `trait Spin<T> { fn map<R,E>(self, f: fn(&mut T) -> Result<R,E>) -> usize; }`,
    with `impl<T> Spin<T> for &mut T` whose body is `while f(self).is_ok() {}`. A drain body of
    `control.map(Consumer::try_pop)` then pops 1000 of 1000 records with the real `&mut self`
    signature.
  - The tool accepts the region in both cases: with the trait in another file, and with the trait
    in the region itself (rows `user-map-no-loop` and `user-map-trait-in-region` in
    `zz_verifier.rs.txt`).
  - The row `function-value-as-an-argument` (`repeat_until_empty(control, Consumer::try_pop)`) is
    refused for exactly this shape. The only difference is the callee's name.
  - The module doc says the `.map` argument is "called at once and the loops around it bound it".
    This is false for a user `map`, and for a lazy `Iterator::map`, whose consumer does the
    looping.
  - The Attempt record's open item says "no infinite form is reachable without `from_fn`,
    `repeat_with` or `successors`". This is false. It looks only at iterator sources.
  - Root's question (iv). The workspace has exactly one `try_pop`:
    `crates/engine/src/realtime/spsc.rs:441` `pub fn try_pop(&mut self)`.
    - `iter::repeat(&mut c).map(Consumer::try_pop)` does not compile: rustc gives E0277,
      `&mut Consumer: Clone` (evidence `s2.rs`). `.cycle()` fails for the same reason.
    - The tool still accepts the text of four shapes with no finding: `repeat(control).map(..)`,
      `[control].into_iter().cycle().map(..)`, the same form in a counted loop, and a `zip` with a
      `repeat`. Only `for x in repeat(..).map(..)` is refused.
    - With a `&self` `try_pop` (evidence `s.rs`), `repeat` pops 1000 of 1000 and `cycle` pops 500
      of 500.
    - So D3 is sound today only because of a receiver type the tool cannot see, and in any case it
      is broken by the user-`map` form above.
  - **Answer: yes, D3 must restrict `.map(path)`** (see SP-A).
  - Measured on the head tree: no `fn map` and no `fn as_mut` is defined anywhere under the roots.
    The only `.map(<path>::try_pop)` is `live.rs:366`, `self.control.as_mut().map(Consumer::try_pop)`.

## MINOR

- **F2. A `debug_assert` macro defined as a workspace proc macro is not seen** (`src/drain/shadow.rs`).
  - Reproducer (evidence `pm.rs` and `e.rs`): `#[proc_macro] pub fn debug_assert(..)` in a crate
    under the roots, with `#[macro_use] extern crate pm;` in the crate of the drain, rebinds the
    count. The program prints `usize::MAX`.
  - The scan sees no `macro_rules!`, no `macro` and no `use`, so the tool stays silent (row
    `proc-macro-fn`).
  - D2's stated limit covers a *registry* crate only. The module doc says trust lasts "only while
    no file under the scan roots defines or imports a macro of that name", and a workspace proc
    macro is such a definition.
  - No proc-macro crate exists today (no `proc-macro = true`).
  - A cheap fix that also covers every enumerated form: count as a shadow any `debug_assert`
    identifier (bare or `r#`) that a `!` does not follow. On the head tree the only such text is in
    `//` comments, which are not tokens, so the real tree stays ok.
- **F3. The row `closure-in-a-nested-closure-in-array-from-fn` never turns red** (`tests/policy/drain/indirect.rs`).
  - Mutation: `in_closure` exempts every closure when any `array::from_fn` closure is on the path.
    This is the obvious mis-scoping of the exemption, and all 140 tests stay green under it.
  - The row's pop is under no counted loop, so the "no count" rule refuses it either way.
  - A row that turns red under that mutation is a nested closure that pops in a `while` inside a
    counted `for` inside the `from_fn` closure. I confirmed that this row goes red under the
    mutation and is refused on the head.
  - Replace the row with that one, or delete it. As written, it defends nothing.

## Verified correct (deviations and claims)

- **v14 and v14b not committed.** The tool refuses both with {drain} alone (the bound is
  `0..self.controls.len()`), and the awk gate passes them. This is SP-4. Root's draft ruling
  (commit both as refused) can be done in attempt 2 with no rule change.
- **u19 committed as accepted.** This is correct. Under rustc, `let remaining = 5;` shadows only
  the rest of the loop body. The header and `remaining -= 1` read the outer binding: evidence
  `u19.rs` runs exactly 3 iterations for `available = 3`. The awk gate over-refuses it.
- **e02, e03 and e06 not committed.** Each is byte-identical, after the marker lines are removed,
  to the `text` of the ported cases `count-under-cfg`,
  `outer-over-slice-parameter-shadowed-by-block-const` and `count-in-raw-identifier-macro`. Each
  has the same path and anchor. No other committed probe duplicates an existing case text.
- **Corpus.** 118 files: 84 `drain`, 2 multi-class and 32 `accepted`, so 86 refused and 32
  accepted.
  - 114 files equal their bundle source with the markers removed.
  - u03 differs only in `//!` changed to `//`.
  - The three `a2-rc` files are the three methods of `rc/shapes.rs`, each in its own `impl`.
  - The accepted set is the spec's 33, without v14 and v14b, plus u19.
- **Gate 3 (parity), re-run on the 118 probes plus v14 and v14b** (evidence `par.py`).
  - The awk gate (gawk) agrees with the TSV on every probe.
  - The tool differs from the awk on exactly 9: a06, a07, b02, n09, e07 and e11 (D1, D2), v14 and
    v14b (SP-4), and u19.
  - u03 is refused by both.
- **Gate 4 (fuzz), re-run** (evidence `fz.py`, which removes regions the parser rejects one at a
  time).
  - The unbounded files hold 3767 regions: 3762 refused, 1 parse failure (`fz_2` s125) and 4
    accepted (`fz_1` s249, `fz_3` s63, `fz_11` s170, `fz_14` s229).
  - In all 4, the header is `<expr>.. { .. }` in a `for` or `while` header. rustc's
    no-struct-literal rule gives the `{..}` to the loop as its body, so the pop block is a bare
    block after the loop.
  - Confirmed with rustc: `for _ in 0.. {} { break; }` gives E0268, "`break` outside of a loop".
    The pop is under no loop, and the regions are not even valid Rust. The tool's acceptance is
    correct.
  - The bounded files hold 800 regions, with 0 refused by the drain class. 125 are refused by the
    unsafe class (66 + 59).
- **The `.map(<path>)`-refused mutation necessarily fails the real tree.** Confirmed: the tool
  refuses `crates/effect-contract/src/live.rs:366`, and 95 tests turn red because the base tree
  holds the same form.
- **Refused probes cannot catch a skipped tree node.** True: an unreached pop is refused. A
  facts-visitor skip did not turn any probe red either; when `Facts::visit_attribute` records
  nothing, only two `per_queue` cfg cases go red. The test docs are accurate.
- **The two superseded B1a tests are deleted.** `form-pop-token-in-a-macro-in-a-counted-loop` and
  `a_pop_in_a_macro_or_a_closure_is_judged_by_the_loops_around_it` are gone. Their refused shapes
  are still covered by `macro-call-in-an-unbounded-loop` and the closure rows.
- **The closure exemption covers only `core::array::from_fn` and `std::array::from_fn`.** It needs
  exactly 3 segments with no qself, and the closure must be the direct argument
  (`tree.rs:103-125`). When I removed the exemption, the base tree and the `outer-*-array-from-fn`
  rows went red, and the real tree stayed ok.
- **Macro tokens.** Every `try_pop` identifier in any macro token tree is a `MacroToken` site and
  is refused first. My mutation that removed this check turned these red: `macro-call-in-a-counted-loop`,
  `macro-call-with-no-loop`, `debug-assert-argument` and u09.
- **Shadow scan bypasses.** All of these are refused, correctly:
  - `use std::debug_assert as d` with `d!(..)`;
  - a `pub use .. as debug_assert` re-export;
  - `#[macro_export] macro_rules! debug_assert`;
  - `macro_rules! r#debug_assert`;
  - `use .. as r#debug_assert`;
  - a shadow in a file under `tools/`.

  A macro-generated `macro_rules! $n` (`mk!($ debug_assert)`) passes the tool, but rustc refuses
  it with E0659 (macro-expanded name against the prelude), with and without `#[macro_use] mod`
  (evidence `a.rs`, `g.rs`). `include!` of a shadowing file also gives E0659. None of these is an
  escape.
- **Fresh D1-D3 bypasses that are refused, correctly:**
  - `let pop = <_>::try_pop;`;
  - `<Consumer<Record>>::try_pop(control)` in a `loop`;
  - UFCS `Queue::try_pop(&mut *control)` in a closure;
  - `?` on a pop in a closure;
  - a `const` or `static` fn pointer in the region;
  - `.flat_map(Consumer::try_pop)`;
  - `.map::<..>(Consumer::try_pop)` in a `loop`;
  - `.map(&Consumer::try_pop)`, `.map((Consumer::try_pop))` and `.map({ Consumer::try_pop })`;
  - an `async` closure.

  These pass, and stay pass, as the stated limits allow:
  - a nested `fn` in the region that pops, called in a `while` (helper limit; B2b-2);
  - a fn value or closure created *outside* the region and called in it (that pop is outside every
    region; B2b-1).

## Spec problems for root

- **SP-A (D3).** `.map(<path>)` on any receiver is a name trust with no shadow check (F1).
  - Option 1: allow the argument only for a receiver of the form `<expr>.as_mut()`, the one real
    form.
  - Option 2: allow `.map(<path>)` only while no file under the roots defines a function or method
    named `map`, a check like D2's `debug_assert` scan. Today there are none.
  - Option 3: both.
  - A registry trait that adds a looping `map` would then be a stated limit, like D2's.
- **SP-B (walk).** `#[macro_use] #[path = "shadow.txt"] mod shadow;` loads a non-`.rs` file as a
  module, and rustc accepts it. A `macro_rules! debug_assert` in it rebinds the count (evidence
  `c2.rs`, which prints `usize::MAX`).
  - The walk reads only `.rs` files, so D2's scan cannot see it.
  - More widely, every rule is blind to code in such a file, marked regions and pops included.
    This crosses slices (the walk, B2b-1).
  - Suggested: refuse a `#[path = ..]` whose target does not end in `.rs`. The one `#[path]` in
    the tree, `crates/host-core/tests/successor_swap.rs:24`, names a `.rs` file.
- **The 1441 problems.**
  - SP-3, the item macro that shadows the count, is still a false pass at this head: a
    `macro_rules! shadow` outside the region expands to `const available`, and both gates are ok.
    B2a's macro rule does not touch it, because the macro has no `try_pop`. Root's ruling decides
    whether attempt 2 or another slice takes it.
  - SP-4 accounts for v14 and v14b, as described above.
- **D2's enumeration** omits the `#[proc_macro] fn debug_assert` form (F2). Either widen the scan
  as F2 suggests, or name the form in the stated limit.

## Test value (one sentence per new test)

- `a_pop_in_a_closure_is_refused`: red if a pop in a closure that is stored, called once, called
  in a counted loop or held by an adapter passes again. Confirmed by the closures-transparent
  mutation, which turned 4 rows red. The fifth, nested `from_fn` row is dead (F3).
- `a_pop_in_a_macro_token_tree_is_refused`: red if a `try_pop` token in a macro call or in the
  arguments of `debug_assert!` is judged by the loops around it. Confirmed: 3 rows went red.
  `macro-call-in-an-unbounded-loop` stays refused by the loop rule, and `macro-body-with-a-pop`
  sits under no loop.
- `a_function_value_that_pops_is_refused_unless_it_is_called_in_place`: red if a `try_pop` path
  value in a `let`, an argument, a field, parentheses, `.and_then` or a 2-argument `.map` passes.
  Confirmed: 7 rows went red.
- `a_function_value_called_in_place_is_judged_by_its_loops`: red if the callee or the `.map`
  argument is refused, or no longer judged by its loops. Confirmed: under the map-refused
  mutation, 95 tests went red.
- `a_debug_assert_that_a_file_shadows_is_not_taken_at_its_word`: red if the scan misses a `use`,
  a rename, a group, a nested or local `macro_rules!` or `macro`, or reads one file only.
  Confirmed: 8 rows went red under both trust-always and shadow-one-file.
- `a_debug_assert_shadowed_in_the_parent_module_is_shadowed_in_the_child`: red if the shadow check
  is per file, which is e07's cross-file escape. Confirmed under shadow-one-file, where e07 itself
  stays green.
- `text_that_does_not_shadow_debug_assert_leaves_it_trusted`: red if a string, a comment, a longer
  name or a `use<..>` bound withdraws trust. This is the implementer's claim; I did not re-run its
  mutations.
- `the_refused_probes_are_refused`: red if a rule accepts a drain that pops until the queue is
  empty. Confirmed under closures-transparent, which turned a06, a07, b02, n09 and e11 red.
- `the_bounded_probes_pass`: red if the walk or a rule over-refuses a form that main accepts.
  Confirmed under map-refused, which turned all 32 red.

## Gates run (head `1a33b2f6d`)

1. Real tree: `realtime_policy` (release) and `scripts/check-realtime-policy.sh` both print
   `ok (94 marked regions in 26 files)`. Release run time: 1.273 s and 1.295 s.
2. `cargo test --locked -p realtime-policy`: 140 passed.
3. Parity: re-run as above, 9 named differences.
4. Fuzz: re-run as above, 3762 / 1 / 4 and 800 / 0.
5. Mutations re-run, each alone, with the real tree checked after each:
   - closures transparent;
   - macro token allowed;
   - trust always;
   - shadow one file;
   - any path allowed;
   - map refused (real tree red at `live.rs:366`, as expected);
   - from_fn exemption removed;
   - from_fn any ancestor (all green: F3);
   - facts records no attributes.

   All match the Attempt record where it makes a claim.
6. No production, `Cargo.toml` or `Cargo.lock` change is in the diff.
7. `cargo fmt --all -- --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `scripts/check-workspace-policy.sh` and `scripts/test-realtime-policy.sh`: all exit 0.

## For attempt 2

- Implement root's D3 amendment (SP-A).
- Add a refused user-`map` row and the `repeat`/`cycle` rows.
- Correct the module doc and the open item.
- Fix F2 and F3.
- Commit v14 and v14b as refused if root rules so.
