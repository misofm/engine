FAIL

# #1441 attempt 3 verdict (B1b: D10 span of a slice parameter, read-rewind test, texts)

Range: `418c9c343..cfa2e9540` on `codex/d15-stream-j3`.
- `0f8852cba` holds root's rulings. It changes spec text only.
- `cfa2e9540` is the attempt. It changes `tools/realtime-policy/**` and the #1441 spec.
- I also read `git diff 1e03edf19 cfa2e9540 -- tools/realtime-policy` for regressions against attempts 1 and 2.

Method:
- I exported `cfa2e9540` with `git archive` into `/tmp/claude-1002/v1441c/tree` and built with `CARGO_TARGET_DIR=/tmp/claude-1002/v1441c/target`.
- I ran each probe on a copy of that tree. The probe was appended to `crates/engine/src/realtime/buffer.rs`, then the release tool and `scripts/check-realtime-policy.sh` ran.
- I compiled and ran each claimed bypass with rustc 1.97.1 (edition 2024). The harness is a `Consumer` that has 1 record available and panics after 5000 pops.
- I did not touch the worktree.

**Every attempt-2 finding is closed, and the attempt-3 code is correct.** The attempt fails on one MAJOR only. It is a fresh false pass in D3's `array::from_fn` exception. It has been in the code since attempt 1, so it is not a regression, and the awk gate also passes it. It is in the same class as the collection-root hole that root sent to #1442 attempt 2. The threat model does not let a verifier put it in a limit. If root rules it into #1442 attempt 2 (limit 3), then no BLOCKER or MAJOR stays.

## MAJOR-1: an item named `core` or `std` redirects the `array::from_fn` outer loop (false pass in both gates)

Where:
- the code is `tools/realtime-policy/src/drain/outer.rs:60-74` (`array_from_fn`) and `src/drain/tree.rs` (`array_from_fn_call`);
- the rule is spec D1 and D3, which make `core::array::from_fn` and `std::array::from_fn` exceptions with no shadow condition.

The defect:
- The rule trusts the path roots `core` and `std`.
- A workspace module or rename with that name shadows the extern prelude in an expression path, and rustc gives no ambiguity error.
- The closure is then an outer loop that never ends, around a counted drain.

Each probe below gets tool 0 and awk 0. Each one compiles, and the harness stops it at 5001 pops with 1 record available:
- `f02`: `mod core { pub mod array { pub fn from_fn<..>(mut f: F) -> [T; N] { loop { f(0); } } } }` at file level, outside the region;
- `f03`: `mod std { .. }` *after* the region;
- `f04`: `mod spin { .. } use self::spin as core;`;
- `f05`: `mk!(core);` at file level, where `mk` is a macro that emits a `mod $n`.

Each probe holds this drain:
```rust
// REALTIME_POLICY_BEGIN
fn probe(control: &mut Consumer<Record>) {
    let _: [(); 1] = core::array::from_fn(|_| {
        let available = control.available_at_entry();
        for _ in 0..available { let Ok(record) = control.try_pop() else { break; }; apply(record); }
    });
}
// REALTIME_POLICY_END
```
The rustc programs are `rs/f02_user_mod_core.rs`, `rs/f03_user_mod_std_inner.rs`, `rs/f04_use_as_core.rs`, `rs/f05_macro_emitted_core.rs`, `rs/q12.rs` and `rs/q13.rs`. `q13` shows that a `mod core` inside the `fn` body also shadows the path.

Why this is not inside a limit:
- Limit 1 does not apply: the code that loops is workspace source, not std.
- Limit 2 does not apply: this is path resolution of a function call, not a method impl or a deref.
- Limit 3 does not apply: no spec lists it for a later slice. #1442's trusted-name scan covers only `map`, `as_mut`, `try_pop` and `debug_assert`.

The #1438 threat model says that where a rule trusts a name, a definition, rename or macro-emitted use of that name must be refused.

Real-tree cost of a fix:
- No item under `crates/`, `hosts/` or `tools/` is named `core` or `std` in the type namespace (no `mod`, no `as core`/`as std`, no `extern crate .. as`).
- The only hit is a method `fn core(&self)` (`crates/rack/tests/mono_reengage.rs:176`). It cannot shadow a path root.

Possible fixes, for root to choose:
- add `core` and `std` to the trusted names of #1442 D3, as type-namespace definitions, renames and macro-emitted names;
- or route this hole to #1442 attempt 2 by ruling, with the collection-root hole.

## Spec problems for root

- **SP-1 (the #1443 sentence from `418c9c343`: not accurate).** The sentence says D8 refuses every other `#[path]`, "`cfg_attr` forms and `$p` metavariables included". The `$p` part is true, because D8 (a) needs a plain literal. The `cfg_attr` part is false:
  - D8 (a) scans `cfg_attr` arguments with the same exception, so it *accepts* `#[cfg_attr(feature = "x", path = "tests.rs")] mod prod;`.
  - D8 (a) also accepts a plain `#[path = "tests.rs"]` inside a `macro_rules!` body, because it reads tokens at any depth.
  - #1443 rule 3 cannot see the macro form. Rule 2 says "`cfg_attr` is ignored", and rule 3 does not say how it resolves a `cfg_attr` path.
  - Thus `tests.rs`, which `#[cfg(test)] mod tests;` declares, can also be production code. rustc confirms this (`rs/pathm/main.rs`, which prints `7 7`). Pops in that file would then count as test code: a false pass.
  - Fix: either D8 (a) refuses every `path` identifier inside `cfg_attr` arguments and inside macro token trees (the real tree has neither), or rule 3 treats both forms as declarations that reach their file. Then correct the sentence.
- **SP-2.** Root assigned the collection-root hole (r01, r02, b01, b06) to #1442 attempt 2. But only #1441's record says so. The #1442 spec has no item or case for it, so threat-model limit 3 ("a limit that a slice's spec lists for a later slice") is not yet met in writing. Add it to #1442, with MAJOR-1 if root routes MAJOR-1 there.

## Attempt-2 findings: closed

**MAJOR-1 (b02 to b05):** closed. I re-ran all of my predecessor's probes from `/tmp/claude-1002/v1441b/probes` on the new binary:
- `b02`, `b03`, `b04` and `b05` are now refused.
- Every other result is the same as attempt 2. The `d9` cases, the `d2` and `d2b` kinds, the `g` macro and attribute rows, `pq_shadow`, `s01` and `c01` are refused. `a00`, `e11`, `f00`, `g03`, `g05`, `g10`, `g16`, `pq_ok`, `s02` and `r03` are accepted.
- `b01`, `b06`, `r01` and `r02` still pass. These are the collection-root hole, which root routed to #1442. I recorded them and do not fail on them.

`read_span` for `Param` is `(body.full, body.end)` (`drain/mod.rs:326-330`). The `let` arm keeps attempt 2's semantics.

**The new b04 and b05 rows:** present in `a_statement_macro_or_a_derive_in_the_body_of_a_function_with_a_slice_loop_is_refused` (`tests/policy/drain/declarations.rs:97-124`).
- Mutation m13 restores attempt 2's statement-end span for a parameter. It turns exactly `item-macro-after-the-slice-loop-shadows-the-slice-parameter` and `derive-after-the-slice-loop-shadows-the-slice-parameter` red (148 of 149 pass). The real tree stays ok.

**MINOR-1 (rewind):** closed. `a_loop_that_does_not_bound_takes_back_only_its_own_reads` (`declarations.rs:219-247`, the test at `:231`). Each mutation is red alone:
- m10, no rewind: only `failed-alias-read-does-not-extend-the-span` is red.
- m11, `truncate(0)`: only `per-queue-in-a-slice-loop-with-a-shadowing-macro` is red.

The real tree stays ok under both mutations.

The accepted row is not a hidden false pass:
- Its `shadow!()` declares `const available`. That makes the inner `let available` refutable, so the program cannot compile.
- Its outer loop `0..lanes` is finite by its form.

**NIT-1:** closed. The D9 slice row is deleted, and the count-form D9 rows remain.

**The three false texts:** corrected, and the new texts are true.
- The module documentation (`drain/mod.rs:26-41`) and the `declares_an_item` and `read_span` docs (`:286-333`) now separate the `let` span from the parameter span.
- These claims are true:
  - an item in the `let`'s own block cannot shadow the `let`: for a `const` or a unit struct, rustc refuses the program (E0005);
  - an item anywhere in the body shadows a parameter (`rs/p3.rs` from attempt 2).
- The attempt-2 record sentence has a correction note.
- The documentation's list of known false passes does not name MAJOR-1. It claims nothing false about it.

**The spec commit `0f8852cba`:** it carries root's rulings faithfully.
- D10's span: for a `let`, the span ends at the end of the statement, in the binding's own block, that holds the pop (this ratifies the widened span); for a slice parameter, the span is the whole `fn` body (b04 and b05).
- The `debug_assert_eq!` and `debug_assert_ne!` over-refusal line.
- #1446 gate 5: SP-C's sentence is removed, and nothing else changes.
- #1442: the F1 reproducer and the F2 `proc-macro-fn` source shapes are added.
- SP-D (the trailing `m!()` sentence) stays, as ruled.

## Fresh bypass attempts (all refused unless noted; `probes/n/`)

- **D10:**
  - a statement macro after a slice loop with the count in the loop header (`x01`, `x02`, `x03`);
  - a macro in a nested `fn` body (`n13`) or a closure body (`n14`) after the loop: over-refusal, which is acceptable;
  - a macro-generated `macro_rules!` used after the loop (`n09`; rustc `q10b` spins, so this refusal is needed);
  - a `shadow!()` in a block in the loop header (`n11`);
  - a macro in pattern position that rebinds the slice or the count (`p01`, `p02`, `p03`; rustc `q11` spins);
  - an alias source shadowed by a macro after the loop in an inner block (`n16`).
- **`debug_assert` made by a generating macro (`n10`):** passes the tool, but rustc refuses it with E0659, ambiguous (`q10`). This is not a false pass.
- **D2:**
  - `use self::FOREVER as lanes;` after the loop (`n07`);
  - `use forever::*;` (`n08`);
  - an associated `const lanes` in an `impl` in the body (`n12`: over-refusal).
- **D9:** END inside each of these is refused:
  - a `const _` block, an `impl`, a nested `fn`, a `trait`, a `const { }` expression or a `let`-`else` in the body (`w04` to `w06`, `w08`, `w10`, `w13`);
  - a `macro_rules!` body (`w07`: split token tree);
  - a closure parameter list, an `enum`, an attribute, a call or a struct (`w09`, `w11`, `w12`, `w14`, `w15`).

  Regions that begin inside a `fn` body with their binding outside the region are refused, which fails closed (`n00` to `n06`, `w01`, `w03`). `w02` (`let` and loop both in the region, inside a block) is accepted correctly.
- **The receiver:** `n15` shows a receiver that an item could redirect, which is the same as a receiver of a workspace type. That is limit 2 ("which impl a method call resolves to"). I recorded it and do not fail on it.

## Gates (re-run on the export)

1. Real tree: the tool and `check-realtime-policy.sh` both print `realtime policy: ok (94 marked regions in 26 files)`.
2. `cargo test --locked -p realtime-policy`: 149 passed. That is attempt 2's 149, minus the deleted D9 row, plus the rewind test.
3. Parity: the instrumented copy of `test-realtime-policy.sh` gives a `parity.tsv` byte-identical to attempt 2's:
   - all 96 trees are refused;
   - 57 rows expect the drain class, and 55 of them report it;
   - the other two rows are `drain-open-tail-last-file` and `drain-region-ends-in-comment`.
4. Mutations, each alone, with the source restored after each:

   | Mutation | Red | Real tree |
   |---|---|---|
   | m10 | the rewind test, only the `failed-alias-read-does-not-extend-the-span` case | ok |
   | m11 | the rewind test, only the `per-queue-in-a-slice-loop-with-a-shadowing-macro` case | ok |
   | m13 | the slice test, only the b04 and b05 rows | ok |
   | m8 (slice parameter not noted) | the three slice rows and the per-queue rewind row | ok |

5. Run time: the release binary on the real tree, at load average 6 to 7. I ran the attempt-2 binary (`aa2f613ed`, built separately) and the attempt-3 binary interleaved, 6 runs each:
   - attempt 3: 1.280 to 1.297 s;
   - attempt 2: 1.271 to 1.296 s.

   The change did not slow the tool. The implementer's 2.6 to 3.1 s came from the machine's load (33).
6. `git diff 418c9c343 cfa2e9540 -- crates hosts scripts Cargo.toml Cargo.lock` is empty.
7. These all exit 0:
   - `cargo fmt --all -- --check`;
   - `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`;
   - `check-realtime-policy.sh`;
   - `test-realtime-policy.sh`;
   - `check-workspace-policy.sh`.

Repository rules:
- No test greps source.
- No production code changed.
- The new names are unversioned.
- There is no queue, so the acked-batch question does not apply.

## Test value (one sentence per new or changed test)

- `a_statement_macro_or_a_derive_in_the_body_of_a_function_with_a_slice_loop_is_refused`: red if a slice parameter's D10 span ends at the body statement that holds the pop (m13), starts after the body opens (the braced row), or if a slice parameter is not noted as a read (m8).
- `a_loop_that_does_not_bound_takes_back_only_its_own_reads`: red if a loop that fails to bound keeps its stale reads (m10, an over-refusal) or forgets reads an earlier rule noted (m11, a false pass through `shadow!()` redeclaring `rounds` as `0..`).

## Evidence kept

All paths are under `/tmp/claude-1002/v1441c/`:
- `probes/n/*.rs` (the fresh probes);
- `rs/*.rs` and `rs/pathm/` (rustc programs);
- `parity.tsv`;
- `mut-*.log`, `mut.sh`, `muts.sh` and `probe.sh`.
