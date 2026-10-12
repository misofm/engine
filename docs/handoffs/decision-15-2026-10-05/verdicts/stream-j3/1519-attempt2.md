FAIL

# #1519 attempt 2 verdict (whole slice: d6b1d1e35, d681c036a, d5b26071f, 1d1dc57fe, 09edb4bb8; base 49f02e984)

Verifier: fresh Opus 5.5 agent, 2026-10-12. Tree: `git archive 09edb4bb8` into /tmp/claude-1002/v1519/tree,
CARGO_TARGET_DIR=/tmp/claude-1002/v1519/target, rustc 1.97.1. No edit, build or commit in the worktree.

## BLOCKER

None.

## MAJOR

**MAJOR-1. An accepted `#[path]` reads a file that the walk skips: the enclosing inline module adds a `target`
or `node_modules` component.** `tools/realtime-policy/src/source_reads.rs:259-281` (`names_a_plain_file`)
checks the components of the written value only. rustc resolves a `#[path]` inside an inline module relative
to that module's directory, and the walk skips every directory named `target` or `node_modules`
(`walk.rs:15`, `:83`). Measured (rustc 1.97.1 compiles each one; the release tool on the exported tree with
the file in place says `ok (94 marked regions in 26 files)`):
- `mod target { #[path = "x.rs"] mod y; }` in `crates/lane/src/zz.rs` reads `crates/lane/src/target/x.rs`
  (that file held the looping `map` trait). `mod r#target { .. }` is the same.
- `src/target.rs` (reached by `mod target;`) holding `pub mod a { #[path = "x.rs"] pub mod b; }` reads
  `src/target/a/x.rs`.
This is a false pass of a direct written `#[path]` form, not limit 4 (no macro is involved), and no other
limit covers it (#1443 resolves declarations through inline modules but does not refuse an unwalked file).
D1 (a)'s own intent (no `target`/`node_modules` component) is defeated, and #1443's D2 sentence ("the
resolver never has to resolve one") is untrue for this form.
The same gap without any `#[path]`: `mod target;` with `target/mod.rs`, `mod node_modules { mod y; }`, and
`mod y;` inside a `target.rs` reached by `mod target;` all compile, and the tool passes them. Those are
outside D1's text (a walk defect from #1438: "Neither holds a compiled `.rs` file" is false), but they break
this issue's product outcome and #1438's threat-model claim ("every file rustc reads as source from them").
Fix needs root's rule text (see "Spec problems"). The real tree has no `mod target`, `mod node_modules`,
`target.rs` or `node_modules.rs` (grep), so a refusal costs 0 real lines. Refusing the directories is not
workable: `hosts/host-web/qualification/node_modules` exists in the primary checkout.

## MINOR

**MINOR-1. No test defends that a `path` identifier outside a written attribute in a macro token tree is
accepted.** Mutation PATHALL (refuse every `path` identifier in a macro token tree, in `ident()`): 180/180
tests pass; only the real tree fails (`builtins-compiler/src/lib.rs:12468`, `conformance_fixtures.rs:95`,
...). The spec's rule 1 case names `assert_eq!(entry.path, x)`, `json!({"path": path})` and a `$path:literal`
metavariable as not refused. Add them as rows of `the_accepted_forms_pass`
(`tests/policy/source_reads.rs:463`). Gate 1 catches the defect today, so this is not a false pass.

**MINOR-2. The `if !(..)` sentence is false (module doc `source_reads.rs:28-29`, and the attempt-2 record).**
"That can only over-refuse a condition that names `include`, `include_str` or `include_bytes`." The shape
`ident ! group` also matches `return !(..)`, `while !(..)`, `match !(..)`, `break !(..)`, and it also
over-refuses a written `#[path]`. Measured: `fn f() { if !({ #[path = "x.rs"] mod a; true }) {} }` compiles
under rustc and the tool refuses it (`RefusedPathAttribute`); `return !(include_str!("d.in").is_empty())` is
refused as `RefusedIncludeLiteralInMacro`. Only over-refusal, but the sentence must state it correctly.

## NIT

- Attempt-1 mutation table is stale after attempt 2's tests: D is now red on 6 tests (adds
  `an_absolute_include_is_a_finding`), L on 4 (the r# rows of the rule 2/3/4 tests), M on 4 (adds the raw
  test), P/R on 3 and Q on 2 (the include rows in macro bodies). Each is still red; the rows say "only".
- "The 223 `include_str`/`include_bytes` lines": 223 is the d6b1d1e35 count including the tool's own files;
  at 09edb4bb8 it is 233 with the tool and 220 outside it. The claim (none in a macro tree) holds: R4 alone
  leaves the real tree passing.
- Spec rule 1 "implemented at `source_reads.rs:200-216`": the code is `:91-102` (attribute detection) and
  `:201-226` (`attribute`, `attribute_tokens`) at 09edb4bb8.
- Case-insensitive file systems (macOS APFS default): `#[path = "Target/x.rs"]` passes the component check
  and reads `target/x.rs` there. Linux CI is unaffected. Record it, or compare components without case.

## Checked and correct

- D1 (a): `.txt`, `..` (also escaped `\u{2e}\u{2e}`), `target` (also `targ\x65t`), `node_modules`, absolute,
  no suffix, `x.rs/`, byte string, C string `c"x.rs"`, `concat!`, `$p`, bare `#[path]`, `#[path(..)]`, `r#path`:
  all refused. `cfg_attr` (nested twice, inner `#![cfg_attr]`, inside an inline module, on an inline module,
  inside a `macro_rules!` body): refused. Written attributes in macro token trees (`macro_rules!`, `macro`,
  invocations with path prefix and any delimiter, nested, `#![path]` in `m!(..)`, a macro inside an attribute
  argument `#[foo(bar!(#[path = "x.rs"] mod a;))]`): refused. `#[::core::prelude::v1::cfg_attr(..)]` and
  `#[unsafe(path = ..)]` do not compile (rustc 1.97.1). Inner `#![path = "q.rs"]` on an inline module makes
  rustc read `q.rs/..`, which the walk refuses as a `.rs` directory.
- Final rule 1 (rj3-e) is what the code does; the new doc comment (`:31-34`) is true. Splice forms (V3, V4,
  V8, W3, R1/R2) are limit 4 as root ruled; not judged here.
- D1 (b): `::core::include!`, `std::include!`, `include![..]`, `include!(concat!(..))`, a literals file that
  holds a macro call `[m!()]`, a missing file, a backslash name (rustc reads the same literal name on Linux),
  absolute: refused; `include!("./d.in")` of literals: accepted (same file as rustc). D1 (c): every `use` row
  refused. Rules 2, 3 and 4: as stated. `#[doc = include_str!(..)]` outside macros: accepted (spec).
- D2: the #1443 sentence is D2's text verbatim. D3: already in the tree (#1442 pointers, ten slices, #1446
  and #1443 Dependencies).
- Real tree: the 1 `path` attribute (`crates/host-core/tests/successor_swap.rs:24`) and 4 `include!` sites
  are the only ones; mutation Z (plain-file acceptance off) refuses `successor_swap.rs:24` alone; mutation N
  refuses the `include!` sites.

## Mutations re-run (each alone on `source_reads.rs`; full `--test policy` suite and the real tree)

| Mutation | Red (tests) | Real tree |
|---|---|---|
| A (a) dropped | 7 path tests | ok |
| B exception in cfg_attr | cfg_attr test only | ok |
| C exception in macro trees (rule 1) | macro test only | ok |
| D (b) dropped | 6 include tests (record: 5) | ok |
| E (c) dropped | use test, raw test | ok |
| F .rs suffix | 4 (as recorded) | ok |
| G, H, I, J | table test only, each | ok |
| K non-literal | non-literal test only | ok |
| L raw not normalised | 4 (raw test plus the r# rows of rules 2-4) | ok |
| M content check | 4 | ok |
| N cwd base | include-directory, accepted forms | fails (include! sites) |
| O use braces | use test only | ok |
| P, Q, R macro tree marking | 3, 2, 3 | ok |
| S inner attr, T nested group | cfg_attr test only, each | ok |
| U absolute | absolute test only | ok |
| V use past `;` | accepted forms only | fails |
| R2, R3, R3w, R4 | own rule test only, each | ok |
| R4b | accepted forms only | fails |
| Z plain-file acceptance off | accepted forms only | fails on successor_swap.rs:24 only |
| PATHALL every `path` in macro trees | none (MINOR-1) | fails |

## Test value (one sentence per new test)

- `a_path_to_a_file_that_is_not_rust_is_a_finding`: a dropped (a), or an (a) without the `.rs` suffix check.
- `the_finding_names_the_line_of_the_path`: a finding placed on the item line instead of the `path` token line.
- `a_path_in_cfg_attr_arguments_is_a_finding_even_for_a_plain_rs_file`: the plain-literal exception applied
  inside `cfg_attr`, or inner attributes / nested groups not read (B, S, T).
- `a_path_in_a_macro_token_tree_is_a_finding_even_for_a_plain_rs_file`: the exception applied in macro token
  trees (rule 1, C).
- `a_path_to_a_file_outside_the_workspace_is_a_finding`: a missing `..`, `target`, `node_modules` or
  leading-`/` check (G-J).
- `a_path_that_is_not_a_string_literal_is_a_finding`: a check that accepts a non-literal value (K).
- `raw_identifiers_do_not_hide_a_path_or_an_include`: names compared without `r#` (L).
- `an_include_of_source_is_a_finding`: (b) dropped or its content check dropped.
- `an_include_that_the_tool_cannot_judge_is_a_finding`: an argument check that accepts a non-literal, two
  arguments or a missing file.
- `an_include_word_without_a_bang_is_a_finding`: rule 2 dropped (`inc!(include)` splice) (R2).
- `an_include_in_a_macro_token_tree_is_a_finding_even_for_a_file_of_literals`: rule 3 dropped or limited to
  `include!(..)` with a group (R3, R3w).
- `include_str_and_include_bytes_in_a_macro_token_tree_are_findings`: rule 4 dropped (R4).
- `an_included_file_with_an_identifier_is_a_finding`: a content check that misses a nested identifier or a
  lex error.
- `an_include_is_read_from_the_directory_of_the_invoking_file`: the file read from the working directory (N).
- `an_absolute_include_is_a_finding`: an absolute `include!` accepted (U).
- `naming_include_in_a_use_item_is_a_finding`: (c) dropped or `use` braces not descended (E, O).
- `the_accepted_forms_pass`: over-refusal of the real `#[path]` and `include!` forms, a `use` mask past its
  `;`, `include_str!` refused outside macros (V, R4b, Z).

## Gates run (on the exported 09edb4bb8 tree)

1. Real tree: release tool and `scripts/check-realtime-policy.sh`: `ok (94 marked regions in 26 files)`, 0 refusals.
2. `cargo test --locked -p realtime-policy`: 180 passed.
3. Mutations: table above.
4. `cargo tree -i realtime-policy`: no dependent; the diff touches only `tools/realtime-policy/**` and specs,
   so no shipped artifact changes.
5. `cargo fmt --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
   `scripts/check-workspace-policy.sh` (`ok`), `scripts/test-realtime-policy.sh` (`ok`): exit 0.

## Spec problems for root

1. (MAJOR-1) D1 (a) judges written components only. Proposed rule: refuse a `mod` item whose name is
   `target` or `node_modules` (`r#` included), inline or out-of-line, written anywhere (macro token trees
   included, by the written-token rule). With path-attributed files being mod-rs-like (measured: `#[path =
   "p.rs"] mod c;` with `mod y;` in `p.rs` reads `y.rs` beside `p.rs`), this closes the `#[path]` form and the
   plain `mod target;` walk gap. 0 real refusals. Authorised path: `tools/realtime-policy/**`.
2. Cargo manifest target paths are another way rustc reads source that the walk does not read:
   `[lib] path`, `[[bin]]`/`[[test]]`/`[[example]]`/`[[bench]]` `path`, `build =`. Measured: a package with
   `[lib] path = "../outside/target/lib.rs"` builds. Real manifests under the roots: 13 target `path =` keys, all `src/...`; no `build =` key. No
   gate refuses a target path outside the scan roots or in a skipped directory. Outside #1519's authorised
   paths (the manifest rule would go in `scripts/check-workspace-policy.sh`, as #1442's manifest rule does), so
   not charged to this attempt; root decides the slice or a limit.
