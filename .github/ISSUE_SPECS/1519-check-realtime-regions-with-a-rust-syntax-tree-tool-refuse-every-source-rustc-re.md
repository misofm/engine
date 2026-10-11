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
