# Check realtime regions with a Rust syntax-tree tool: refuse every source rustc reads that the walk does not

Stream J3 of decision 15 (`docs/rulings/live-updates-seamless-swaps-and-one-control-plane-2026-10-05.md`,
D15-0). Split out of *B2a* (#1442) by a root ruling of 2026-10-11 (#1442 had grown past one attempt; this
part had never had an attempt). It is a slice of the realtime-policy tool program (#1438 lists the slices).

## Product outcome

The realtime-policy walk refuses every way rustc reads source that the walk does not read, so every rule of
the tool sees all the source it judges. #1443's path resolver then meets only `#[path]` forms it can
resolve.

## Context

- rustc reads source through `#[path = ".."]` module attributes (also through `cfg_attr` arguments and
  from inside macro bodies and invocations, including a `$p:literal` metavariable) and through `include!`
  (also renamed with `use std::include as inc;`). The walk follows plain module files only, so code reached
  any other way is invisible to every rule (a shadow, a looping trait, a pop).
- Reproduced with rustc 2024 (review evidence, 2026-10-11): `#[path = "x.txt"] mod m;`,
  `#[cfg_attr(all(), path = "x.txt")]`, `#[path = "../../scripts/x.rs"]`, `#[path = $p]`,
  `use std::include as inc; inc!("z.txt")` all compile; `#[path = concat!(..)]` does not.
- The #1442 attempt-1 verifier showed that a `#[cfg_attr(feature = "x", path = "tests.rs")] mod prod;` or a
  `#[path]` inside a `macro_rules!` body makes rustc build a test file as production code, which #1443's
  rule 3 cannot see; #1443's sentence "D8 refuses the rest before this resolver runs" was therefore false.
- Real tree (measured on `main` and the J3 branch): one `path` attribute,
  `crates/host-core/tests/successor_swap.rs:24` (`#[path = "support/successor.rs"]`, file level), and four
  `include!` sites, each of a digest array whose identifiers are only in `//` comments:
  `crates/gate-expander/src/corpus.rs:294`, `crates/multiband-compressor/src/corpus.rs:600`,
  `crates/soft-clip/src/corpus.rs:212`, `tools/wasm-gate-corpus/src/lib.rs:1865`. No `path` attribute is
  inside a `cfg_attr` or a macro; no `include` appears in a `use`.

## Decisions

- D1. Over the tokens of every `.rs` file under the roots, at any depth, refuse:
  - (a) an identifier `path` in an attribute (`#[..]` or `#![..]`). It is accepted only if it is directly
    followed by `=` and a plain string literal whose value ends in `.rs`, is relative, and has no `..`,
    `target` or `node_modules` component, **and** the attribute is not inside `cfg_attr` arguments and not
    inside a macro token tree (a `macro_rules!` or `macro` body, or a macro invocation's token tree). A
    `path` in `cfg_attr` arguments or in a macro token tree is refused, whatever its value;
  - (b) an `include!` invocation (any path prefix), unless its one argument is a string literal naming a
    file, relative to the invoking file's directory, whose `proc_macro2` tokens (comments dropped) are
    literals and punctuation only;
  - (c) an identifier `include` in a `use` item.
  `include_str!` and `include_bytes!` produce a literal and stay allowed.
- D2. In #1443 D1 rule 3, the sentence becomes: "D8 refuses every other `#[path]` (every `path` in
  `cfg_attr` arguments or in a macro token tree, and every `$p` metavariable, included) and every
  `include!` of source before this resolver runs, so the resolver never has to resolve one."
- D3. Bookkeeping in the same PR: #1442's D8, its six D6 cases, its D8 gate-5 mutation and the SP-B line of
  its Amendment 2 become a one-line pointer to this issue; #1438's slice list goes from nine to ten (and
  each "slice X of nine" header follows); #1446 Dependencies read "After (same stream): B2a (#1442), this
  issue and C1. If any of them spends its budget without PASS, C2 does not land in this batch."; #1443
  Dependencies add "After: this issue". The existing `cfg_attr(.., path = "x.txt")` case is superseded by
  the `.rs` case below and is deleted.

## Authorized paths

`tools/realtime-policy/**`, this spec, and only the sentences and lines named in D2 and D3 of the #1442,
#1443, #1446 and #1438 specs.

## Cases (each refused) and their test value

- `#[path = "shadow.txt"]`: red if (a) is dropped.
- `#[cfg_attr(all(), path = "ok.rs")] mod m;`: red if (a) does not read `cfg_attr` arguments or applies the
  plain-literal exception there (the false pass that reaches `tests.rs` as production code).
- `macro_rules! m { () => { #[path = "ok.rs"] mod x; } } m!();`: red if (a) applies the literal exception
  inside a macro token tree, where #1443's resolver cannot see the declaration.
- `#[path = "../../../scripts/x.rs"]`: red if (a) checks only the `.rs` suffix.
- `#[path = $p]` from a `$p:literal`: red if (a) accepts a value that is not a literal.
- `include!("x.in")` whose file holds a looping `map` trait: red if (b) is dropped.
- `use std::include as inc;` with `inc!("z.txt")`: red if (c) is dropped.

## Gates

1. Real tree: the tool and `check-realtime-policy.sh` give the same counts as before; D1 refuses nothing
   (re-measured).
2. `cargo test --locked -p realtime-policy` passes.
3. Mutations (PR evidence), each alone, with the real tree still passing; each turns red only the case
   named for it: (a) dropped; the (a) exception applied inside `cfg_attr`; the (a) exception applied inside
   macro token trees; (b) dropped; (c) dropped.
4. Shipped artifacts unchanged, as A1's gate 6 (`.so` identical; `.a` members identical once `.llvmbc` is
   removed; the worklet module UNCHANGED).
5. `cargo fmt --check`, `cargo clippy -p realtime-policy --all-targets -- -D warnings` and
   `scripts/check-workspace-policy.sh` exit 0.

## Budget, size and order

Three attempts; about half a day. After B1b (#1441); before C2 (#1446) and B2b-1 (#1443).

## Attempt record (attempt 1, J batch branch `codex/d15-stream-j3`, base `49f02e984`)

**Built.** `tools/realtime-policy/src/source_reads.rs` (new; one token walk over every `.rs` file under the roots,
called from `check` right after the unsafe scan), three classes in `report.rs` (`RefusedPathAttribute`,
`RefusedInclude`, `RefusedIncludeImport`), one line in `lib.rs`, and `tests/policy/source_reads.rs` (14 tests).
No other `src` file of the tool is touched (the drain files are not).
- (a) is read from the tokens: `#` (and `#!`) followed by a bracket group. Every identifier `path` at any depth
  inside is refused unless `=` and a string literal follow it directly (`syn::LitStr`, so a raw string counts; a
  byte string, `concat!`, `$p`, a bare `#[path]` do not), the value ends in `.rs`, does not start with `/` or `\`, and has no
  `..`, `target` or `node_modules` component (split on `/` and `\`), and the attribute is not inside a macro token
  tree and its first token is not `cfg_attr`.
- A macro token tree is the group after `name!`, `path::name!`, `macro_rules! name` and `macro name` (with its
  second group), at any depth below. `ident ! group` also matches `if !(..)`; no attribute can sit there, so the
  over-match refuses nothing real.
- (b) `include` followed by `!` and a group of any delimiter, any path prefix: the group must be exactly one string
  literal, relative, naming a file under the invoking file's directory that is readable, lexes, and holds
  `Literal`, `Punct` and groups of them only. Everything else is a finding, a missing file included.
- (c) any identifier `include` in the tokens of a `use` item (up to its `;`, into braces); `use<..>` is skipped.
- `r#path`, `r#include` read as `path`, `include`: rustc accepts both (checked with rustc 1.97.1, below).
- D2: the sentence in #1443 is replaced with D2's text. D3 was already in the tree (root's rj3-c commit): #1442's
  D8 pointer, SP-B line and gate-5 bullet; "ten" slices in #1438 and each header; #1446 and #1443 Dependencies.
  Re-checked, no edit needed. #1442 holds no `x.txt` case, so there was nothing to delete.

**Real tree (re-measured on the branch).** `grep`: 1 `path` attribute (`crates/host-core/tests/successor_swap.rs:24`),
4 `include!` (`gate-expander/src/corpus.rs:294`, `multiband-compressor/src/corpus.rs:600`,
`soft-clip/src/corpus.rs:212`, `tools/wasm-gate-corpus/src/lib.rs:1865`), none in `cfg_attr` or a macro, none
in a `use`: as the spec says, no difference. Tool and `scripts/check-realtime-policy.sh`: `ok (94 marked regions in
26 files)`, zero refusals. The sites are seen: with the `path` acceptance forced off the tool refuses
`successor_swap.rs:24` alone; with the include base wrong it refuses the four `include!` (mutation N).

**Gates.** 1 real tree: above. 2 `cargo test --locked -p realtime-policy`: 177 passed (163 before this slice +
14). 4 shipped artifacts: `cargo tree -i realtime-policy` shows no dependent, so no shipped input changed; no
artifact was built. 5 `cargo fmt --check`, `cargo clippy --locked -p realtime-policy --all-targets -- -D warnings`,
`scripts/check-workspace-policy.sh` (`ok`), `scripts/check-realtime-policy.sh`, `scripts/test-realtime-policy.sh`
(`ok`): exit 0.

**Gate 3 mutations** (each alone, applied to `source_reads.rs`; the real tree passes under all but N and V; red =
tests of `tests/policy/source_reads.rs`, abbreviated by the claim they defend):
| Mutation | Red |
|---|---|
| A (a) dropped | `.txt` path; the line of the finding; cfg_attr; macro; `..`/target/absolute table; non-literal; raw ident (7) |
| B exception applied inside `cfg_attr` | cfg_attr test only |
| C exception applied inside macro trees | macro test only |
| D (b) dropped | include of a looping trait; include forms the tool cannot judge; included file with identifier; include directory; raw include (5) |
| E (c) dropped | use-item test; raw ident (`use std::r#include`) |
| F `.rs` suffix dropped | `.txt` path; table; line; raw ident |
| G / H / I `..` / `target` / `node_modules` component dropped | the table test, each alone |
| J leading `/` dropped | the table test |
| K a non-literal value accepted | the non-literal test only |
| L raw identifier not normalised | the raw-identifier test only |
| M included-file content check dropped | looping trait; cannot-judge; identifier in included file |
| N include read from the working directory, not the invoking file's | include-directory test, accepted-forms test; the real tree fails on the 4 `include!` sites |
| O `use` braces not descended | use-item test only |
| P / Q / R `macro_rules!` / `macro` / invocation group not marked a macro tree | the macro test, each alone |
| S inner attribute `#![..]` not read | cfg_attr test (its inner row) only |
| T nested group of an attribute not descended | cfg_attr test only |
| U absolute `include!` accepted | `an_absolute_include_is_a_finding` only (first run: no red, because the case named a missing file; the case now writes the file) |
| V `use` scan runs past the `;` | accepted-forms test (a `use` is followed by a legitimate `include!`); the real tree fails too |
| (X, `=` spacing check) | no red: the check was dead (rustc rejects `path ==`); removed |

**Test value** (which defect turns each red that no existing test catches): the `.txt`/table tests, a path
check that tests the suffix only or misses a component; cfg_attr and macro tests, the SP-1 false pass (a plain
`.rs` literal accepted where #1443's resolver cannot see the declaration, rustc then building `tests.rs` as
production); non-literal test, `concat!`/`$p` accepted by a check that looks at the next token only; raw-identifier
test, a name compared without `r#`; include tests, an `include!` of a trait that no rule reads, a content check
that is missing or reads the wrong directory; use-item test, `use std::include as inc` past the `include!` rule;
accepted-forms test, a rule that refuses the real `#[path]` and the digest `include!`s (and a `use` scan that
swallows the next item).

**Deviations.** None from the rule text. `r#` normalisation is a reading of "identifier `path`". The sentence in
#1443 follows D2; the previous text there was root's Amendment 2 wording, which D2 replaces.

**Open items (rule gaps, each reproduced with rustc 1.97.1; not fixed, rule text is root's).**
1. `macro_rules! m { ($m:meta) => { #[$m] mod x; } } m!(path = "y.rs");` compiles and builds `y.rs` as production
   code: the `path` is in an invocation, not in an attribute, so (a) does not see it.
2. `macro_rules! inc { ($i:ident) => { $i!("z.txt") } }` with `inc!(include)` compiles and includes `z.txt`: the
   identifier `include` has no `!` after it and is in no `use` item, so (b) and (c) do not see it.
3. An `include!` inside a `macro_rules!` body reads its file relative to the file that expands the macro, not the
   file that holds the body (`sub/def.rs` defines it, `./w.in` was read, not `sub/w.in`). The tool judges the
   file beside the body. The real tree has no `include!` in a macro.
