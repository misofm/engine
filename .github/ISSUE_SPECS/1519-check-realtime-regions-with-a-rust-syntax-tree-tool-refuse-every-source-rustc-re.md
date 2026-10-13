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
  - (d) (Root Amendment 2) A `mod` item, inline or out of line, written anywhere under the roots (macro token trees included), whose name (r# included), compared case-insensitively, equals a directory name the walk skips, is refused. The skip list is read from the walk's own constant (walk.rs), so the rule follows it by construction. Such a module makes rustc resolve child modules and `#[path]` values inside a directory the walk does not read.
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

## Root Amendment 1 (2026-10-11)

Root's text, verbatim (it closes the three open items that attempt 1 reproduced; it amends D1, and rule 4 is conditional on a measurement):

>
> They apply the threat model (fail closed, a false pass is not acceptable) to the three gaps.
> 1. (Replaced by root's ruling rj3-e, 2026-10-12; the earlier text of this rule 1, which refused any identifier `path` in a macro token tree and any attribute whose content starts with a `$` metavariable, is withdrawn. The rule now reads:)
>    Inside macro token trees the tool applies D1 (a) to WRITTEN attributes: a written `#[..]`/`#![..]` whose content holds the identifier `path` is refused (inside a macro token tree, whatever its value, as D1 (a) already says). A `path` identifier that is not inside a written attribute is not refused (limit 4 covers a path attribute assembled by expansion). Amendment 1 rule 1's second bullet (`#[$m]`/`#![$m]`) is withdrawn too (limit 4). Rules 2-4 (d5b26071f) stand; source_reads.rs:200-216 already implements this rule (0 real refusals).
> 2. Outside a `use` item, an identifier `include` (r# included) not directly followed by `!` is refused, anywhere. This is the same idea as F2 for debug_assert, and it closes `inc!(include)`.
> 3. Inside any macro token tree, refuse `include!` (any path prefix). Its file is read relative to the expanding file, which the tool cannot judge. Also refuse an `include` identifier there in any form.
> 4. include_str!/include_bytes! stay allowed outside macro token trees. Inside a macro token tree, refuse them too, but only after the implementer measures the real tree. If any real site has include_str!/include_bytes! (or their identifiers) inside a macro token tree, do NOT refuse them. Report the sites to me first.
> Each rule gets a refused case with a test-value sentence and a mutation, re-measured to zero refusals on the real tree. Reading `path` and `include` to cover the r# forms is correct; keep it. Matching `if !(..)` as a macro shape is harmless over-refusal, so record it as such.

## Root Amendment 2 (2026-10-12)

Root's text, verbatim (it adds D1 (d) above, and gives the manifest-target-path rule to this issue by a named exception to the authorized paths):

> Both close direct written forms under the reviewed threat model, so they apply that model and are not new policy.
>
> MAJOR-1, the rule text for D1, new item (d):
> "(d) A `mod` item, inline or out of line, written anywhere under the roots (macro token trees included), whose name (r# included), compared case-insensitively, equals a directory name the walk skips, is refused. The skip list is read from the walk's own constant (walk.rs), so the rule follows it by construction. Such a module makes rustc resolve child modules and `#[path]` values inside a directory the walk does not read."
> - Correct #1438's false note "Neither holds a compiled .rs file" in the same commit.
> - Cases (refused): `mod target { #[path = "x.rs"] mod y; }`; `mod r#target;`; `mod node_modules { mod y; }`; a case-variant `mod Target;`. Each case gets its test-value sentence. Mutations: drop (d) (red), and compare case-sensitively (the case-variant row is red).
> - Measure: 0 real refusals expected.
>
> Spec problem 2, manifest target paths: give it to #1519 attempt 3, by named exception in scripts/check-workspace-policy.sh (the same file as #1442's manifest rule; the later slice rebases).
> - The rule: every `path =` under `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]` and `[[bench]]`, and every `build =`, in every workspace Cargo.toml, must be a relative path inside its package directory, with no `..` component and no component that the realtime walk skips.
> - One self-test case per table kind that turns red when the check is dropped.
> - Measure: the 13 real keys stay accepted.
>
> MINOR-1, MINOR-2 and the NITs (from /home/bl/misofm/submix-verdicts/1519-attempt2.md): fold them into attempt 3. If attempt 3 fails only on the manifest rule, split that rule into its own slice; do not let it sink #1519.

The named exception: the manifest-target-path rule is written in `scripts/check-workspace-policy.sh` (the file that holds #1442's
manifest rule) and its self-test cases in `scripts/test-workspace-policy.sh`. Those two files are authorized for this rule only.

## Authorized paths

`tools/realtime-policy/**`, this spec, and only the sentences and lines named in D2 and D3 of the #1442,
#1443, #1446 and #1438 specs.
Root Amendment 2 adds, by named exception, `scripts/check-workspace-policy.sh` and `scripts/test-workspace-policy.sh` for the
manifest-target-path rule only, and the one #1438 sentence it corrects.

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
- Placement (Root Amendment 1; the case list follows the amendment's rules, not rule text of this spec). Each is
  refused, each has a one-sentence test value and a mutation in the Attempt record:
  - Rule 1 (root ruling rj3-e, 2026-10-12; implemented in `Scan::stream` (attribute detection) and `Scan::attribute`/`attribute_tokens` of `source_reads.rs`): a written `#[..]` or `#![..]` inside a macro
    token tree (a `macro_rules!` or `macro` body, or a macro invocation's token tree) that holds an identifier `path` (also
    `r#path`) at any depth is refused, whatever follows it and whatever its value: `macro_rules! m { () => { #[path = "ok.rs"] mod x; } } m!();`,
    `m!(#[path = "ok.rs"] mod x;);`, and `#[cfg_attr(.., path = ..)]` there. A `path` identifier that is not inside a written
    attribute is not refused (for example `assert_eq!(entry.path, x)`, `json!({"path": path})` and a `$path:literal` metavariable).
    A path attribute that an expansion assembles (`m!(path = "y.rs")` into `#[$m]`) is limit 4 (#1438 Hazards), not refused here.
  - Rule 2: `macro_rules! inc { ($i:ident) => { $i!("z.txt") } } inc!(include);` and `r#include` not followed by
    `!` outside a `use` item.
  - Rule 3: `include!` (any path prefix) and an `include` identifier in any form inside a macro token tree.
  - Rule 4 (only if the measurement of the real tree finds no `include_str!`/`include_bytes!` inside a macro
    token tree): both refused inside a macro token tree, still allowed outside. If a real site exists, rule 4 is
    not implemented and the sites go to root.
  - D1 (d) (Root Amendment 2; each refused, in `a_module_named_like_a_skipped_directory_is_a_finding` unless noted):
    `mod target { #[path = "x.rs"] mod y; }` (test value: a (d) dropped lets the plain `.rs` literal that (a) accepts read
    `target/x.rs`, the attempt-2 false pass), `mod r#target;` (a name compared without `r#`), `mod node_modules { mod y; }`
    and `mod target;` (the same gap with no `#[path]`), the forms in a macro body and a macro call and in a function (a check
    that skips macro token trees or function bodies), and in `a_skipped_directory_name_is_matched_ignoring_case` the case
    variants `mod Target;`, `mod Node_Modules {..}` and `mod r#TARGET;` (a case-sensitive compare: `Target` is the directory
    `target` on a case-insensitive file system). `the_finding_names_the_line_of_the_module`: a finding placed on the `mod`
    keyword's line instead of the name's. The accepted forms gain `mod targets;`, `mod target_dir;`, `mod node_modules_x;`,
    `mod my_target;`, `use crate::target;`, `fn target()`, `struct Target;` and a local `target` (a substring or non-`mod`
    match) and, for MINOR-1, `assert_eq!(entry.path, path)`, `json!({"path": path})` and `($path:literal)` in a macro body.
    The `#[path]` table of (a) gains the case variants `Target/x.rs` and `a/Node_Modules/x.rs`.
  - Manifest target paths (Root Amendment 2; `scripts/check-workspace-policy.sh`, self-tests in
    `scripts/test-workspace-policy.sh`): one self-test per table kind, each red when that kind is dropped from the check, each with a
    different violation (`[lib]` with `..`, `[[bin]]` with `target`, `[[test]]` absolute, `[[example]]` with `node_modules`,
    `[[bench]]` with the case variant `Target`, `[package] build = "../build.rs"`), and one accepted case: a dependency `path`
    that leaves the package, `./src/lib.rs`, a single-quoted `tests/targets/node_modules_x.rs`, `build = "build.rs"`.
  - Recorded as over-refusal (corrected in attempt 3; the verifier measured the earlier "harmless" wording false): the macro shape `name ! group` is
    purely syntactic, so it also matches `if !(..)`, `while !(..)`, `match !(..)`, `return !(..)` and `break !(..)`; the group is then judged as a
    macro token tree, which over-refuses a written `#[path]` (whatever its value) and every `include`, `include_str` or `include_bytes` word inside
    the parentheses. Only over-refusal, and the real tree has no such form.

## Gates

1. Real tree: the tool and `check-realtime-policy.sh` give the same counts as before; D1 refuses nothing
   (re-measured).
2. `cargo test --locked -p realtime-policy` passes.
3. Mutations (PR evidence), each alone, with the real tree still passing; each turns red only the case
   named for it: (a) dropped; the (a) exception applied inside `cfg_attr`; the (a) exception applied inside
   macro token trees; (b) dropped; (c) dropped.
   Placement (Root Amendment 1): one mutation per amendment rule (1: a written attribute inside a macro token tree
   no longer refuses its `path`, which turns the rule 1 case red; 2: bare `include`; 3: `include!` and `include` in a macro tree; 4 if implemented), each alone, red only
   on its own case.
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

Corrections after attempt 2 (re-run by the attempt-2 verifier; each mutation is still red, but on more tests than the rows above
say, because attempt 2 added tests that use the same code): D is red on 6 tests (adds `an_absolute_include_is_a_finding`), L on 4
(the `r#` rows of the rule 2/3/4 tests), M on 4 (adds the raw-identifier test), P and R on 3 and Q on 2 (the `include` rows in macro
bodies). The "only" in the rows P/Q/R above means the macro-tree tests, not one test.

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

## Attempt 2 (J batch branch `codex/d15-stream-j3`, head `d6b1d1e35`, then the Root Amendment 1 commit `d681c036a`)

Attempt 1 got no separate verdict: root ordered Root Amendment 1 carried in attempt 2 without waiting.

**Measured first (root's rule 4 condition).** `include_str!`/`include_bytes!` (or their identifiers) inside a macro
token tree: none on the real tree. The measurement ran the new walk with those identifiers refused in macro trees
over `crates hosts tools`; the only two findings were the tool's own method named `include` (rule 2, renamed
`judge_include`). So rule 4 is implemented. The 223 `include_str`/`include_bytes` lines of the tree (at `d6b1d1e35`; 233 at `09edb4bb8`, of which 220 are outside
`tools/realtime-policy`) all stand outside macro token trees (argument of a call, a `const` initialiser).

**Rules 2, 3 and 4 are built** in `tools/realtime-policy/src/source_reads.rs` (and one new class,
`RefusedIncludeLiteralInMacro`, plus a wider `RefusedInclude` message, in `report.rs`):
- Rule 2: an identifier `include` (r# read as `include`) that is not directly followed by `!` is refused anywhere
  outside a `use` item (inside one, (c) refuses it as `RefusedIncludeImport`, only).
- Rule 3: inside a macro token tree an identifier `include` in any form is refused (`include!` with any path
  prefix, `include != x`, `include ! x`). The (b) content judgement now runs outside macro trees only.
- Rule 4: inside a macro token tree `include_str` and `include_bytes` (r# included) are refused in any form, class
  `RefusedIncludeLiteralInMacro`; outside a macro tree they stay allowed.
- The `use` scan now marks the item's tokens (up to its `;`, carried into groups) instead of a second walk.
- `if !(..)` matches the macro shape `name ! group`. (This bullet said "harmless over-refusal: it can only refuse a
  condition that names `include`, `include_str` or `include_bytes`". Attempt 3 corrects it: the shape also matches
  `return`, `while`, `match` and `break` before `!(..)`, and it also refuses a written `#[path]` inside the
  parentheses. Over-refusal only; none on the tree.)

(Superseded by root's ruling rj3-e, 2026-10-12: rule 1 is now the written-attribute rule stated in Root Amendment 1 above, and
`Scan::attribute` and `attribute_tokens` of `source_reads.rs` implement it with 0 real refusals. The paragraph below is the attempt-2 record of the withdrawn text.)

**Rule 1 (the withdrawn text) is NOT built: it fails its own gate on the real tree.** Root Amendment 1 rule 1 refuses any identifier `path`
in any macro token tree, "whatever follows it", and any `#[$m]`/`#![$m]` there. Measured over the J3 branch tree
(`crates hosts tools`): 132 refused lines in 54 files (103 outside `tools/realtime-policy`, 29 inside). 124 are an
ordinary identifier `path` in a macro invocation or definition (`assert_eq!(entry.path, ..)`, `write!`,
`vec![..]`, `json!({"path": path})`, a `path: ..` field in `vec![Diagnostic { .. }]`, a `$path:expr` or
`$path:literal` metavariable in `crates/session/src/estimate.rs:130-137` and
`crates/session/tests/diagnostic_parity.rs:81-85`), and 8 are the `$(#[$enum_meta])*` / `#[$variant_meta:meta]`
shape of the enum-forwarding macros at `crates/session/src/model.rs:11,12,14,16,50,51,53,55`. The tree cannot be
edited here (the authorised paths are the tool and this spec), the amendment names no exception, and any narrowing
(a `path` after `.` or `$`, before `:`) is rule text that is root's. The `m!(path = "y.rs")` bypass (open item 1)
therefore stays open. Rule 1 needs a root ruling before it can land. The 132 lines reproduce by refusing a `path` identifier in
macro token trees in `source_reads.rs` and running the tool on the tree.

**Tests** (`tests/policy/source_reads.rs`, 180 in the crate, 163 before attempt 1, +14, +3 here; one row deleted):
- `an_include_word_without_a_bang_is_a_finding` (rule 2): the `inc!(include)` splice, a function, call, binding,
  method and raw identifier named `include`. Test value: a macro-spliced `include!("z.txt")`, which reads a
  file no rule judges, passes a check that only looks at `include!`.
- `an_include_in_a_macro_token_tree_is_a_finding_even_for_a_file_of_literals` (rule 3): `digests.in` is a file of
  literals that the same `include!` outside a macro passes, so only the macro tree makes the row red. Test value:
  an `include!` in a macro body, read relative to the expanding file, judged against the file beside the body.
  Rows `include != 1` and `include ! x` defend the word in any form.
- `include_str_and_include_bytes_in_a_macro_token_tree_are_findings` (rule 4). Test value: the same
  relative-to-expander read for a literal macro; the accepted-forms test (`include_str!`/`include_bytes!` as a
  constant and as a call argument) defends that they stay allowed outside.
- Deleted row: `m!(include!("looping.in"))` of the cannot-judge test, superseded by the rule 3 test (it is refused
  by rule 3 whatever the file holds). Accepted-forms test: the `fn include()` lines are gone (rule 2 refuses
  them); `include_nothing` and `include_more` stay, to defend the word boundary.

**Mutations** (each alone on `source_reads.rs`, then reverted; red = the tests that fail):
| Mutation | Red | Real tree |
|---|---|---|
| R2 bare `include` allowed | `an_include_word_without_a_bang_is_a_finding` only | ok |
| R3 `include` in a macro tree judged like outside (word and `include!`) | the rule 3 test only | ok |
| R3w only the word form (no group follows) dropped in macros | the rule 3 test only (rows `!=`, `! x`) | ok |
| R4 `include_str`/`include_bytes` allowed in macros | the rule 4 test only | ok |
| R4b `include_str`/`include_bytes` refused outside macros too | `the_accepted_forms_pass` only | fails on the tree |
| V `use` item mask runs past `;` | `the_accepted_forms_pass` only | fails on the tree (4 digest `include!`) |
| O `use` mask not carried into brace groups | `naming_include_in_a_use_item_is_a_finding` only | ok |

**Gates.** Real tree: zero refusals; the release tool and `scripts/check-realtime-policy.sh` give `ok (94 marked
regions in 26 files)`. `cargo test --locked -p realtime-policy` 180 passed. `cargo fmt --check`, `cargo clippy
--locked -p realtime-policy --all-targets -- -D warnings`, `scripts/check-workspace-policy.sh` (`ok`) and
`scripts/test-realtime-policy.sh` (`ok`) exit 0. Gate 4 (shipped artifacts): as attempt 1, `realtime-policy` has no
dependent crate; no artifact built.

**Open items.** Rule 1 (above): a root ruling is needed. (Closed by root's ruling rj3-e, 2026-10-12: see Root Amendment 1, rule 1.) Of attempt 1's open items, 2 (`inc!(include)`) and 3
(`include!` in a macro body read relative to the expanding file) are closed by rules 2 and 3; 1 stays open.

## Attempt 3 (J batch branch `codex/d15-stream-j3`, base `7d586f674`; spec commit `1ce10f872` records Root Amendment 2)

Closes the attempt-2 verdict (FAIL: MAJOR-1, MINOR-1, MINOR-2, NITs) and builds root's Amendment 2.

**Built.**
- D1 (d), `tools/realtime-policy/src/source_reads.rs`: a `mod` item (keyword `mod`, then an identifier) whose name, `r#`
  stripped and lower-cased, equals a name in the walk's own constant is class `RefusedSkippedModule`, wherever the tokens
  are (inline or out of line, macro token trees, function bodies). `walk.rs` exports `SKIPPED_DIRECTORIES` (`pub(crate)`);
  nothing is copied. D1 (a)'s component check now reads the same constant and compares ignoring case (the verifier's
  case-insensitive-file-system NIT: `#[path = "Target/x.rs"]` is refused). The walk's doc comment no longer says "Neither
  holds a compiled `.rs` file".
- Manifest target paths, `scripts/check-workspace-policy.sh` (named exception): one `python3` (`tomllib`) pass over the tracked
  manifests. It reads `[lib]`, `[[bin]]`, `[[test]]`, `[[example]]`, `[[bench]]` `path` and `[package] build` as TOML
  tables, so the inline and dotted spellings are the same tables. A path must be a string, relative, with no `..` component and
  no component (compared ignoring case, `/` and `\` both separators) in the skipped list; `build = false` is accepted.
  Dependency `path` keys are not target paths and are not read. **Single source (choice):** the script reads
  `SKIPPED_DIRECTORIES` from `tools/realtime-policy/src/walk.rs` with one regex and fails closed (execution failure, not a
  pass) if the constant is not found or empty, so the list is not duplicated and no agreement test is needed. The price is a
  source read in a policy script; the alternatives were a copied list plus a pinning test (a test that reads source), or a
  new tool command (a new public surface). `scripts/test-workspace-policy.sh` copies `walk.rs` beside its `mutants/scripts/`
  copies, because they run from another directory. Status 3 is a violation; any other non-zero status is an execution failure.
- MINOR-1: `the_accepted_forms_pass` gains `assert_eq!(entry.path, path)`, `json!({"path": path})` and `($path:literal)`.
- MINOR-2 and NITs: the module doc, the Cases bullet and the attempt-2 bullet now state what the tool does (`name ! group`
  also matches `if`, `while`, `match`, `return` and `break` before `!(..)`; it over-refuses a written `#[path]` and the
  `include*` words there; over-refusal only, none on the tree). The attempt-1 mutation table has a correction note (D 6 tests, L 4,
  M 4, P/R 3, Q 2). The include counts read 223 at `d6b1d1e35`, 233 at `09edb4bb8` (220 outside the tool). The rule 1 line
  references are function names. The case-insensitive-FS limit is in the module doc (Unicode lower-casing, not the full
  folding of APFS/HFS+; Linux CI has none).

**Real tree.** `./target/release/realtime_policy .` refuses `crates/effect-contract/src/live.rs:366` and nothing else
(expected pre-#1345, from #1442 attempt 3); (d) refuses nothing (no `mod target`/`mod node_modules` on the tree). The manifest
rule reads 20 target `path =` keys and no `build =` in the 43 tracked manifests: the 13 under the roots plus 7 outside them
(`fuzz/Cargo.toml` 6, `docs/handoffs/control-smoothing-defaults/measure/Cargo.toml` 1), all accepted.

**Gates.** `cargo test --locked -p realtime-policy`: 186 passed. `cargo fmt --check`, `cargo clippy --locked -p realtime-policy
--all-targets -- -D warnings` clean. `scripts/check-workspace-policy.sh` ok; `scripts/test-workspace-policy.sh` ok;
`scripts/check-realtime-policy.sh` ok (94 marked regions in 26 files); `scripts/test-realtime-policy.sh` ok. Shipped artifacts:
`realtime-policy` has no dependent crate, and the commit touches only the tool, two scripts and specs.

**Tests and mutations** (each alone; red = tests / self-test cases that fail; the real tree stays at live.rs:366 only under all D
mutations):
| Test | Test value (which defect turns it red that no other test catches) | Mutation | Red |
|---|---|---|---|
| `a_module_named_like_a_skipped_directory_is_a_finding` | (d) dropped, or limited to out-of-line modules, outside macro trees or function bodies, or comparing `r#target` raw | D-drop (3 tests: this, the case test, the line test); D-not-in-macro; D-raw | D-not-in-macro: this test only; D-raw: this and the case test (its `r#TARGET` row) |
| `a_skipped_directory_name_is_matched_ignoring_case` | a case-sensitive compare at the `mod` site (`Target` is `target` on APFS) | D-case-sensitive | this test only |
| `the_finding_names_the_line_of_the_module` | the finding on the `mod` keyword's line | D-line | this test only |
| `a_path_to_a_file_outside_the_workspace_is_a_finding` (rows `Target/x.rs`, `a/Node_Modules/x.rs`) | a case-sensitive component check in (a) | A-case-helper (also turns the case test red, which shares the helper) | this test and the case test |
| `the_accepted_forms_pass` (MINOR-1 rows) | a rule refusing every `path` identifier in a macro token tree (the attempt-2 mutation PATHALL, 180/180 green then) | PATHALL | `the_accepted_forms_pass` only; the real tree fails at `builtins-compiler/src/lib.rs:12468` and more |
| self-test `target-path-lib` | `[lib]` not read | lib-dropped | `target-path-lib` only |
| `target-path-bin`, `-test`, `-example`, `-bench`, `-build` | the kind not read (`bin`, `test`, `example`, `bench`, `package.build`) | one drop each | its own case only |
| (violation rows) | `..`, absolute, skipped component, case-insensitive | dotdot-dropped | `target-path-lib`, `-build` (both use `..`); absolute-dropped: `-test` only; skipped-dropped: `-bin`, `-example`, `-bench`; case-sensitive: `-bench` only |
| `allow_target_path_lookalikes` (accepted) | a check that reads dependency `path` keys, or matches skipped names as substrings | dependencies-scanned; substring | red at the accepted case only (all refusal cases stay red-for-the-right-reason) |

The inline-table spelling has no self-test of its own: it was written and removed, because no mutation of this checker separates it
from the `[[bin]]` case (a TOML parser reads both as the same table). The claim "an inline table is the same table" rests on
`tomllib`, not on a test.

**Open items.** None for #1519's rules. Observation, not charged: manifests outside the scan roots (`fuzz/`, `docs/handoffs/...`)
have target sources the realtime walk never reads; the new rule keeps their paths inside their own directories, which is all the
amendment asks. Rule 1's limit 4 (a path attribute assembled by macro expansion) is unchanged.

**Verdict follow-ups (doc only, 2026-10-12, root-approved wording).** MINOR-1 of the attempt-3 verdict: the #1438 D3 note and the
`walk.rs` doc comment of `SKIPPED_DIRECTORIES` said that rustc reaches the skipped directories only through a `mod` item, which is
false (Cargo target autodiscovery, `[[bin]] name` without `path`, and dependency or member paths reach them). Both now say that the
walk reads no `.rs` file under them, that D1 (d) refuses a `mod` item that reaches them, and that the other routes are staged for
#1527 (#1438 limit 3). NITs recorded, none fixed here:
- The inline `bin = [{ .. }]` and `lib.path = ".."` self-tests, and tests for the drive-letter branch, the `\`-separator branch and
  the fail-closed constant-read branch of the manifest target-path rule, are deferred to #1527, which replaces the manifest reader
  (its Root Amendment 1 extends the one `tomllib` pass).
- The `name ! group` list in `source_reads.rs` is not complete: `in !(..)` (`for _ in !(..)`) matches the macro shape too. Over-refusal
  only; the module doc does not claim the list is complete.
- APFS/HFS+ folds `ſ` (U+017F) to `s`, but `to_lowercase` keeps it, so the case-insensitive compare of D1 (a) and (d) misses
  `mod node_moduleſ`. Not a hidden CI false pass: the walk skips only exact names and Linux CI has no folding, so the file there is
  read. It matters only on a case-insensitive macOS dev build; a one-line fold would close it if root wants it.
