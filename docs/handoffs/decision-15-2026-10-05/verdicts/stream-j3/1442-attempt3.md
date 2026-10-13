FAIL

# #1442 attempt 3 verdict (B2a, last attempt): pops in closures, macros and function values; trusted names; manifest rule

Range `09edb4bb8..7d586f674` on `codex/d15-stream-j3` (`0761d16d9` code, tests and the manifest rule;
`7d586f674` Attempt record and the #1426 deletions). Spec text: root Amendment 4 (`1d1dc57fe`) under
ruling rj3-e. I exported `7d586f674` to `/tmp/claude-1002/v1442c/tree` and built it with
`CARGO_TARGET_DIR=/tmp/claude-1002/v1442c/target`. I did not touch the worktree. Scripts, probes, manifests
and mutation logs are in `/home/bl/misofm/submix-verdicts/evidence/1442-attempt3/`.

The Rust tool part of the slice is correct. I reproduced every gate, every Amendment-4 mutation, the parity,
the fuzz counts and the real-tree claims, and no fresh direct form of R1, R1b, (q), F2, (u), (n), R3, R4 or R5
passes. The attempt fails on the manifest rule: five families of direct written `Cargo.toml` forms give a
dependency the extern name `core`, `std` or `alloc`, Cargo accepts each one, and
`scripts/check-workspace-policy.sh` says `ok`.

## MAJOR

- **M1. The manifest rule passes direct TOML forms of a `core`/`std`/`alloc` dependency key**
  (`scripts/check-workspace-policy.sh:262-286`).
  - The awk rule reads a manifest line by line. It treats a line as a dependency key only under a
    `[..]` header whose text, with no spaces, ends in `dependencies` or `dependencies.<name>`. TOML has
    other spellings of the same table, and the rule does not see them:
    1. inline tables: `dependencies = { core = { .. } }` before `[package]`, or under `[target.'cfg(unix)']`;
    2. dotted keys: `dependencies.core = { .. }` or `target.'cfg(unix)'.dependencies.core = ..` before
       `[package]`, and `dependencies.core.path = ..` under `[target.'cfg(unix)']`;
    3. whitespace around the dots of a header: `[ dependencies . core ]`, `[target . 'cfg(unix)' . dependencies]`;
    4. quoted table names: `["dependencies"]`, `["dependencies".core]`;
    5. escapes in a quoted key: `"core" = { .. }`, `[dependencies."core"]`, `["dependencies"]`.
  - Evidence (`manifest/cargo-results.txt`, `manifest/rule-results.txt`). Cargo 1.97.1, editions 2021 and
    2024: each form passes `--extern core=<fake>` and `core::hint::black_box` resolves to the fake crate
    (prints 4242). 13 of the 23 manifests I wrote pass the extracted awk rule. End to end on the export,
    with the form appended to `crates/math/Cargo.toml`: the target-inline, quoted-header and spaced-header
    forms give `workspace policy: ok`; the control `[dependencies.core]` is refused.
  - Effect (`std-probe/`). A `std` key in the target-inline form, pointing at a `no_std` facade whose prelude
    exports a `debug_assert` that rebinds its first identifier, makes `debug_assert!(available <= 1)` rebind
    the count. The drain pops 1000 records for a count of 1 (`cargo run -p b`). B1a-D7 trusts that macro.
    With the facade in `vendor/fakestd` (outside the scan roots) and this form on `crates/graph/Cargo.toml`,
    the workspace policy says `ok` and the tool refuses only `live.rs:366`.
  - Why this is a FAIL and not a limit. Root Amendment 4 closes "a Cargo dependency key or rename to
    `core`, `std` or `alloc`" with this rule, and the brief asked for these forms (quotes, inline tables,
    target tables). Each one is a direct written form in a tracked manifest. No limit of #1438 Hazards
    covers it.
  - A fix I measured, for the successor: read each tracked `Cargo.toml` with Python's `tomllib` (3.12 on
    ubuntu-24.04) and refuse the keys `core`, `std`, `alloc` of `dependencies`, `dev-dependencies`,
    `build-dependencies` (and the `_` spellings), of each `target.<cfg>` table, and of
    `workspace.dependencies`. A 15-line reader refuses all 23 of my manifests, the plain forms included.
    Or use the `rename` field of `cargo metadata --no-deps` for the members. The parser reads the
    file the way Cargo does, so no spelling escapes it, and `test = false` cannot match.

## MINOR

- **m1. The module documentation gives registry crates the wrong limit number.**
  `tools/realtime-policy/src/drain/trusted.rs:37-39` says a registry crate's macros and the standard
  library "are limit 3". In #1438 Hazards, code outside the workspace is limit 1. Limit 3 is "a limit a
  later slice closes".
- **m2. Two module comments say more than the scan does about imports.** `trusted.rs:13-15` ("every
  other written definition, import, binding or field" is refused) and `drain/mod.rs:69-71` ("refuses a
  written definition, import or rename"). An import by a `::` path passes (q) as the spec's text allows:
  `pub use crate::loops::try_pop;` gives `ok` (probe `q04`). This is harmless: such an import can only
  name a written definition that (q) refuses, a registry item (limit 1) or an expansion (limit 4). But
  the sentences must say that.

## NIT

- `scripts/test-workspace-policy.sh` is edited (four refused cases and one lookalike). Authorized paths in
  the #1442 spec name only `scripts/check-workspace-policy.sh`. The Attempt record discloses it, and the
  self-test belongs with the rule. Root should add the path to the exception.
- `try-pop-by-value-on-a-reference-impl` stays as a row of C4. The spec said to delete it (superseded by
  (q)). The record states it moved. It catches nothing that `try-pop-looping-on-a-ring` does not.

## Checks against the spec and root's items

- **Gate 1, real tree.** At `7d586f674` the release tool refuses exactly
  `crates/effect-contract/src/live.rs:366` and nothing else. With the review's rewrite copied over
  `live.rs`: `ok (94 marked regions in 26 files)`. With the real #1345 attempt-2 `live.rs`
  (`wt-d15-b3` at `01f859ba1`) copied over: `ok (94 marked regions in 26 files)`. The whole B3 export
  under J's tool: no drain finding over 106 regions in 27 files. The awk gate: `ok (94 / 26)`.
- **Gate 2.** `cargo test --locked -p realtime-policy`: 183 passed.
- **Gate 3, parity.** 120 probes: 89 refused + 31 accepted, the tool equals each file name's classes, and
  the awk and the tool differ on exactly a06, a07, b02, n09, e11, e07, v14, v14b, n06 and u19.
- **Gate 4, fuzz.** fz_1-4 and fz_11-14: 3767 regions, 3762 refused, 1 parse failure (fz_2 s125), 4 accepted
  (fz_1 s249, fz_3 s63, fz_11 s170, fz_14 s229). fz_21 + fz_22: 106 refused (58 + 48), 694 accepted. As recorded.
- **Gate 5, mutations** (each alone, my own harness: `mut.sh`, `muts.sh`). Each is red alone, on the named case only:
  M1 `map-path-on-as-mut`; M2 `the_map_or_count_form_is_not_a_count`; M3 C4's `try-pop-looping-on-a-ring` (and the
  second-definition test, because my M3 also allows the second `fn` in spsc.rs); **M3b** (the one-per-name count
  dropped) `a_second_definition_in_the_defining_file_is_refused` only; M4 C5 and the second-definition test;
  M9 `path-debug-assert`, `rooted-debug-assert`; M10a the three refused rows of C11; M10b C11 and the allowed-forms
  test, and the real tree also refuses `crates/math/src/lib.rs:52`; M11 C12; M12 `cfg-attr-feature`; M12b
  `metavariable-expression`; M13a r13 and r14 only; M13b r04, r07, r09, r05, r11, `pq_norounds_2`, r13 and
  `a_region_that_ends_in_a_deeper_block_is_refused` (r14 stays green: D10 sees its macro); M14 C15 only.
  Under every mutation the pre-#1345 tree shows only `live.rs:366` (plus `math:52` under M10b), and the tree
  with #1345's `live.rs` is `ok` (except `math:52` under M10b). Carried over and re-run: F2 dropped (4 tests),
  any `try_pop` path value (1 test), closures transparent (4 tests). The manifest self-tests: dropping the header
  form, the key form, or the table guard each turns its row red.
- **(q) second `fn try_pop` in spsc.rs** (root item): refused at its own line by
  `a_second_definition_in_the_defining_file_is_refused`; M3b turns only that test red.
- **#1426 strip** (root item). Measured: after #1345's rewrite no count or pop receiver in the tree ends in
  `.as_mut()`/`.as_ref()` (the only chains were `live.rs:349-352` and `:366`; `live.rs:1038` binds `observe` first).
  The base fixture's drains bind `control` first. The only test cases that still spell `.as_mut().map(..)` or
  `.as_ref().map_or(..)` are the refused C1 and C2. So nothing needs the strip. The `7d586f674` edits to #1426
  delete only the strip clause, the `stage` twin of "Still accepted", mutation (b) and its test-value bullet.
- **Placement texts** (root's condition). Checked against `rj3-e-final.md` and `rj3-d-review.md`:
  the #1345 D8 pointer matches "amend #1345 D8 now" and the J gate-1 sentence; #1442's (s) is the review's (s)
  with S = {`debug_assert`} and keeps "also withdraws B1a-D7"; the (q) join is the review's first clause plus
  root's replacement exception, word for word. All three are correct.
- **Other spec items.** No production, `Cargo.toml` or `Cargo.lock` change; `realtime/mod.rs:268` untouched; (m),
  the name-emitting set, its closure and rename tracking are deleted; `u19-swapped` deleted and C15 added; limit-4
  probes t1, t3a, t3b, t3drain and t4 pass and t2, t5, t6drain and t7 are refused, as recorded.

## Fresh direct-form bypass attempts (tool; all refused or bounded)

Probes appended to `crates/engine/src/realtime/buffer.rs` of the base fixture (`probe.sh`), or put in a new file
(`newfile.sh`).
- R1/R1b: `Option::map(self.control.as_mut(), Consumer::try_pop)`, `.map(<Consumer<Record>>::try_pop)`,
  `(Consumer::try_pop)(control)`, `Consumer::try_pop as fn(..)`, `while let .. = Consumer::try_pop(c)`,
  `.and_then(|c| c.try_pop().ok())`, counts `map_or_else(|| 0, Consumer::available_at_entry)`,
  `Consumer::available_at_entry(c)` and `.max(usize::MAX)`: all refused. A UFCS callee in a counted loop is
  accepted (bounded, correct).
- (q): `fn r#try_pop` default method, `use .. as try_pop`, `use other::{try_pop}`, field, type alias, label,
  `macro_rules! try_pop`, closure parameter, `..try_pop`, associated `const available_at_entry`, generic and
  lifetime names, `fn try_pop` in a macro token tree, `fn`/name split over two lines: all refused.
  `pub use crate::loops::try_pop;` passes, as the spec allows (m2).
- F2/B1a-D7: an assignment to the count inside `debug_assert!` args (`for` and `while` forms), a `bump!(count)`
  in the loop, `use core::debug_assert;` (trust withdrawn): all refused. An inner `let` or item in a block in the
  args is accepted and bounded.
- (u)/(n)/R4: `#[cfg_attr(all(), macro_use)] extern crate`, `#[macro_use] mod m { #[macro_use] extern crate .. }`,
  `#[macro_use] pub mod`, `#[r#no_implicit_prelude]`, `#![r#feature(..)]`, `#![feature\n(..)]`,
  `#![cfg_attr(.., r#feature(..))]`, `fn $ { concat(..) }`: all refused. `feature[..]` and `feature{..}` are
  rejected by rustc ("wrong meta list delimiters"), so they enable nothing.
- R3: items in an inner block after the loop, a sibling block before the `let`, a statement macro after the
  region, a derive before the `let`, a glob `use` before the `let`, an alias source shadowed by a later `fn`,
  a `static` in a closure after the region, `extern crate .. as <count>`: all refused (some are over-refusals,
  which the spec accepts).
- R5: a `continue` before the decrement, `-= 0`, a shadowing `let`, a `&mut` of the count, `wrapping_sub`, a
  decrement in a block, nested pops: all refused. `#[cfg(any())] remaining -= 1;` passes the tool, but rustc
  1.97.1 rejects it ("attributes on expressions are experimental"), so it is not a form (`fresh/cfgdec.rs`).

## Test value (new or rewritten tests)

The implementer's sentences are true; my mutation runs confirm each named defect turns its test red and no
other test catches it:
- C1 (`map-path-on-as-mut`): red if `.map(<path>)` on `<fields>.as_mut()` is accepted again.
- C2 (`the_map_or_count_form_is_not_a_count`): red if the `map_or(0, <Type>::available_at_entry)` count returns.
- C4, C5: red if a written definition of `try_pop` / `available_at_entry` outside spsc.rs is allowed.
- `a_second_definition_in_the_defining_file_is_refused`: red if the spsc.rs allowance is not one `fn` per name (M3b).
- C10: red if D7 matches the last path segment.
- C11: red if (u) is dropped; its accepted half is red if the `#[macro_use] mod` exemption is dropped.
- C12: red if (n) is dropped. C13: red if `feature(` is matched only at the head of an attribute, or `${` is not refused.
- C14: red if the item scan stops at the region, or if a `let`'s scope starts at the `let`.
- C15: red if the decrement need not be the body's first statement.
- `the_trusted_names_in_their_allowed_forms_pass`: red if the scan refuses the allowed forms (`#[macro_use] mod`
  among them, M10b).
- Manifest self-tests (`sysroot-*`): red if the header form, the key form or the table guard is dropped. They do
  not reach the forms of M1.

## Gates run on the export (x86_64)

Every x86_64 step of the `lint` job and the `test-release` job in `.github/workflows/qualification.yml` exited 0
(`logs/summary.txt`): fetch, fmt, clippy (`--all-targets --all-features -D warnings`), doc, workspace policy and its
mutation tests, session, env-vocabulary, bench, test-support-ci, scalar-oracle, script reachability, console fixture,
builtins fixture tests, bench preconditions, host-core, protocol-control, realtime policy (awk) with audit-leak and
artifact-leak, trace validator, lane, unfused seal, rack/builtins/graph, effect-runtime, conformance, PEQ seal,
release shape, npm modes, stem store, stem identity, the two sub-v3 refusal probes and the AVX2+FMA probe; release
lane/math/wasm-gates, filter_liveness, tail_contract, ramp_endpoint, designer_total, the FMA cfg check, the loom leg,
and (route-gated in CI, run here anyway) M1 exhaustive and F1.

## Notes for root

- **Spec problem / rescope.** This was attempt 3 of 3. Everything except the manifest rule meets Amendment 4. A
  bounded successor can own the manifest rule alone: a TOML-parser reader (above), its refused cases (the five
  families) and the `test = false` lookalike, in `scripts/check-workspace-policy.sh` and its self-test.
- **Scan roots versus path crates.** A path dependency outside `crates/`, `hosts/` and `tools/` (my `vendor/fakestd`)
  passes the workspace policy, and the trusted-name scan does not read it. #1438 limit 1 names registry crates, not
  in-repo path crates outside the roots. With the manifest rule fixed, such a crate still reaches the drain only by a
  written `use`, `#[macro_use]` or rename, which the scan sees; but root may want to state which limit covers it.
- **Rebase item (not this slice).** `crates/capi/tests/design_bound_cache.rs` (#1471, already on `main`) holds
  `unsafe` that the awk gate allows and J's tool does not. On the B3 tree the tool reports 7 unsafe findings there.
  J's rebase onto `main` must add it to the tool's ownership list.
- **R4 scope.** `feature(` is refused in `.rs` tokens. A crate attribute given by `-Zcrate-attr` in RUSTFLAGS or
  `.cargo/config.toml` is outside the scanned files. `${` is still refused in tokens; the other nightly macro
  features are limit 4 anyway. Recorded, not a finding.
