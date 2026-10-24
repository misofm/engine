# Hold doctests to the realtime-policy tool's claims (#1527 rule iv)

Split from #1527 (root decision, 2026-10-24): rule (iv), doctests.

## Problem

#1527's rule (iv) asks the realtime-policy tool to treat Rust doctests the same way as the other build components it reads, so that a doctest cannot hide code from the realtime claims. Rules (i)-(iii), D1-D2 and D4-D6 stay in #1527 and live in `scripts/check-workspace-policy.sh`'s Python `tomllib` pass. Rule (iv) belongs in the Rust tool (`tools/realtime-policy`), because only the tool parses Rust sources. The text of rule (iv), copied from #1527's spec (whose file leaves the tree), is below.

## Smallest closable slice

Implement rule (iv) in the tool as #1527's spec states it, with one table-driven test row per form. Sequence it after #1528, because both add `report.rs` classes.

## Gates

1. The tool's real-tree run is unchanged, or each new finding is a real defect that is fixed in the same PR.
2. Each new row is refused by the tool and accepted by the parent commit's tool (PR evidence).


## Rule (iv), copied from #1527's spec (root amendment 1, 2026-10-12)

Doctests (iv): in compiled doctest code blocks in doc comments under the roots, refuse `unsafe` and
`include!`, the same as the tool's unsafe-ownership and include rules. A doctest is its own test crate: it
cannot reach a realtime region, but it can hold unsafe code that no ownership rule sees.

Measure 0 refusals on the real tree. Each part gets a case with its test-value sentence. If any real doctest
block holds unsafe or include!, report to root before refusing it.

- (iv) Doctests. "Compiled doctest code block": a markdown fenced block (3+ backticks or tildes, also behind a
  `>` or a list marker) or an indented (4 spaces) code block, in a `///`, `//!`, `/** */`, `/*! */` or
  `#[doc = ".."]` text, whose rustdoc language string makes it Rust and not `ignore`. Rustdoc's rule (LangString
  parse): tokens split on `,`/space; `rust`, `should_panic`, `no_run`, `test_harness`, `compile_fail`,
  `standalone_crate`, `editionNNNN`, `EnnnN` keep it Rust; `ignore*` makes it not compiled; any other word
  (`text`, `toml`, `sh`, ...) makes it not Rust; an empty string is Rust. `compile_fail` and `no_run` count as
  compiled (rustdoc compiles both, and a `compile_fail` block that wrongly compiles still held the text).
  Hidden lines (`# ` prefix) are part of the block. The scan does not honour `#[cfg]`: a block on an item that
  is cfg-disabled on this host is still compiled on another target.
  - Unsafe. A doctest block follows the same unsafe-ownership rule as the file that holds it. The `unsafe`
    test is the tool's unsafe-ownership rule applied to the block's tokens (`unsafe` followed by `impl`, `fn`,
    `extern` or a brace group), with the same allowlist: a block in a file on `UNSAFE_ALLOWLIST`
    (`tools/realtime-policy/src/policy.rs`) is not refused, because that file may hold the same `unsafe` as
    code. The 4 lane blocks (`crates/lane/src/fpenv.rs:185`, `:256`, `:322` and `crates/lane/src/softfma.rs:132`;
    #1422's plain twins of the `compile_fail` guarantees, each with a SAFETY comment; both files are
    allowlisted) stay accepted.
  - Include. `include!`, `include_str!` and `include_bytes!` in a doctest block are refused outright, with no
    literal-file allowance (rustdoc resolves the path relative to the source file, measured in 1519; the real
    tree has no site, so the stricter form costs nothing). The test refuses the identifiers `include`,
    `include_str` and `include_bytes` anywhere in a block.
  - Doctests in `*.md` files and `#[doc = include_str!("x.md")]` are out of the rule's text; the real tree has
    neither (0 `include_str!` in a doc attribute, 0 doc `.md` includes).
- Home of (iv): the realtime-policy tool. The rule is defined as "the same as the tool's unsafe and include
  rules", whose allowlist (`UNSAFE_ALLOWLIST`, a local constant in `policy.rs`), token scan (`unsafe_scan.rs`)
  and include rules (`source_reads.rs`) already live there. The tool's lexer gives every doc form as one
  `#[doc = "..."]` attribute (`///`, `//!`, `/** */` and written `#[doc]`) and does not see a `///` inside a
  string, and the block text is re-lexed with the same code, so `unsafe` in a comment or string inside a block
  is not mistaken for code and an unlexable block fails closed. The "no Rust code" and "tool untouched" lines
  of this slice (Non-goals "any change to the realtime-policy checks", Objective gate 6, the budget sentence)
  are amended to match. If (iv) alone makes #1527 too big for its budget, split (iv) into its own slice before
  implementation, not after a failure.

(iv) Doctests (tool tests under `tools/realtime-policy/tests/`)
- Refused: a doc comment block ```` ```\nunsafe { f() }\n``` ```` in a file not on the allowlist; the same as
  `compile_fail`, `no_run`, `should_panic`, `edition2021`, and `rust,ignore` is NOT refused; a hidden line
  `# unsafe { f() };`; `include!("x.rs")`, `include_str!("x")`, `include_bytes!("x")`; the block in a `//!`, a
  `/** */` and a `#[doc = "```\nunsafe {}\n```"]` form; a `~~~` fence, a 4-backtick fence, a fence behind `> `
  and behind `- `; an indented (4-space) block. Test value: nothing else in the tool tokenizes doc text, so
  unsafe code or an `include!` written only inside a doctest (rustdoc compiles it, the tool reads it as a
  string) passes every other rule. Mutations, each alone: skip `//!`/block/attribute doc forms, drop the
  hidden-line strip, drop `no_run`/`compile_fail` from "compiled", drop the tilde/blockquote/list/indented
  recognition, drop `include_str`/`include_bytes`: its own row red.
- Not refused: `text`/`toml`/`sh`/`ignore` blocks that hold `unsafe` or `include!`; prose and inline code
  (`` `unsafe` ``) mentioning the words; `#![forbid(unsafe_code)]` inside a block; an `unsafe` block in a
  doctest of an allowlisted file (the four real sites). Test value (allowlist row): refusing every `unsafe`
  doctest refuses the lane twin tests that #1422 added beside each `compile_fail` guarantee. Mutation: ignore
  the allowlist: the allowlisted row red (and, on the real tree, 4 findings).
- A doc text that does not lex as Rust (an unbalanced `"` in a `compile_fail` block) is a finding
  (fail-closed), never a skip; a self-test row.

## Decision record (attempt 1, 2026-10-24)

- Slice (root, from the scope note): new `tools/realtime-policy/src/doctests.rs`, run from `check` beside
  `unsafe_scan::scan`; `unsafe_scan::scan_stream` is `pub(crate)` and reports lines through a callback; three
  classes `UnsafeInDoctest`, `IncludeInDoctest`, `DoctestNotLexable`. No `Cargo.toml` change (`syn::LitStr`
  unescapes the literal). `source_reads.rs` is unchanged: the include test is a plain identifier scan.
- Language string: the tool follows rustdoc's `LangString::parse` order rule, which is stricter than the
  summary above: a Rust tag counts when no other word came before it (`no_run,text` is compiled, `text,no_run`
  is not), and `rust` always counts. Only the exact word `ignore` makes a block not compiled: `ignore-<target>`
  ignores it on one target only, so it is compiled elsewhere and counts as compiled here.
- Doc text: consecutive attributes of one kind (outer or inner) are one item's doc; a block comment is
  beautified as rustc does (blank first and last line, `*` column); the joined text loses its smallest indent,
  as rustdoc's unindent does. The markdown reading is a CommonMark subset, stated in `blocks`; an indented
  block in a block quote in a list item is not recognized.
- Stated limit (owner threat model, 2026-10-11): the tool refuses the written forms; a doc text that a macro
  builds (`concat!`, a `$x` metavariable, `cfg_attr(.., doc = ..)`, `#[doc = include_str!(..)]`) is not read.
- The hidden-line strip is kept for a faithful block text, but no token rule can tell it apart: `#` is a
  token of its own, so the `# unsafe {..}` row passes with or without the strip (an equivalent mutant).

## Evidence (attempt 1)

- Real tree (release tool, temporary count print, not committed): 38 compiled blocks, 4 with unsafe
  (`crates/lane/src/fpenv.rs:185`, `:256`, `:322`, `crates/lane/src/softfma.rs:132`, all allowlisted),
  0 refusals; `realtime policy: ok (193 marked regions in 34 files)`.
