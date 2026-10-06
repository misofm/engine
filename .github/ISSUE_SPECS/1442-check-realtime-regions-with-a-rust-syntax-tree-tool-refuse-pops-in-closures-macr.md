# Check realtime regions with a Rust syntax-tree tool: refuse pops in closures, macros and function values, and commit the probe corpus

Stream J follow-up of decision 15
(`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`, D15-0, D15-2).
This is slice B2a of nine (C1 (#1445), A1 (#1438), A2 (#1439), B1a (#1440), B1b (#1441), B2a, C2 (#1446), B2b-1 (#1443), B2b-2 (#1444); A1's spec lists what
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

- **D1. Closures.**
  - A pop inside a closure body is refused, except in an `array::from_fn` closure, where B1b-D1
    and B1b-D3 apply.
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
- **D3. Function values.**
  - A path expression whose last segment is `try_pop` is allowed only in two positions:
    - as the callee of a call (`Consumer::try_pop(control)`);
    - as the only argument of a `.map(..)` method call (`live.rs:366`, #1426 D2's form).
  - Any other use is refused, for example a `let pop = Consumer::try_pop;`.
- **D4. The allow-list changes.** B1a-D4 makes `Expr::Closure` and `Expr::Macro` transparent.
  D1 and D2 replace that with refusal on a pop's path. The rest of B1a-D4 is unchanged.
- **D5. Committed probe cases.**
  - Use A1's case format. Each probe replaces `fn drain_matrix_controls() {}` in the base tree,
    with its own marker lines removed, so A1-D6 does not refuse nested markers.
  - Split `v1302/a2/rc/shapes.rs` into its three shapes.
  - The expected class sets are stated here, not taken from the tool's output:
    - every probe that the TSV shows refused with the drain class: {drain};
    - `v1302/a3/t/u08`: {forbidden-body, drain};
    - `v1302/a3/t/u13`: {unsafe, drain};
    - `v1302/a3/t/u03`: committed in a valid form (`//` in place of `//!`), expecting {drain};
    - a06, a07, b02, n09, e11: {drain} (D1);
    - e07: {drain} (D2).
  - The 33 bounded probes become accepted cases. A probe whose shape the base tree or a B1a or B1b case
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
- **D7. Documentation.** The drain rule's module documentation drops the closure,
  function-value and macro limits from B1b-D4, states D1 to D3, and states D2's limit.

## Authorized paths

- `tools/realtime-policy/**`
- This spec

## Non-goals

- B2b-1's non-test pop rule, allowlist and markers; B2b-2's call rule.
- #1418's and #1426's rules.
- Committing the fuzz files (PR evidence only; root accepted this, 2026-10-05).

## Hazards

- **The probe sources are in the bundle.** Copy their content into case fixtures. Do not read
  them from `/tmp`.
- **Over-refusal is acceptable, a false pass is not.**
- As in A1: the awk gate scans the tool's own sources until C2.

## Objective gates

1. **The real tree.** `cargo run --locked --release -q -p realtime-policy` prints the same counts
   as before this slice. `bash scripts/check-realtime-policy.sh` passes on the same tree.
2. **The cases.** `cargo test --locked -p realtime-policy` passes, with every earlier case and
   every D5 and D6 case.
3. **Parity (PR evidence).** A throwaway script outside the repository runs `main`'s awk gate
   (gawk) and the tool on each D5 probe. The verdicts agree with the TSV, except for the D1 and D2
   probes and u03's repaired form. Name each difference.
4. **Fuzz (PR evidence).** Run the tool's drain rule over the ten fuzz files: 3767 regions are
   refused and 800 are accepted.
5. **Mutations (PR evidence).** Apply each alone. Each turns red the named cases, and the real
   tree still passes:
   - closures transparent again: a06, a07, b02, n09, e11 and the D6 closure case;
   - macro token trees skipped: the D6 macro case;
   - `debug_assert` always taken at its word: e07, the cross-file case and the `use` case;
   - the shadow check limited to the file of the drain: the cross-file case and the `use` case;
   - any `try_pop` path value allowed: the `let pop` case;
   - `.map(<path>)` refused: the accepted `.map` case.
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
