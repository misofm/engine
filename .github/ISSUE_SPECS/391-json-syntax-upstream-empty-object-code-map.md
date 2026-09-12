# json-syntax upstream fix for the empty-object CodeMap volume bug (#387 option 3): upstream PR, optional [patch] ruling, ignored regression test

# Issue 391 scoped bugfix plan — 2026-09-12

Scope verdict: APPROVE the bounded upstream correction and engine dependency sentinel, revalidated on main 644756ffb59a2950fc3fa7b2c91b7f9f7435b485 after #307 delivery; root may now create/synchronize the numbered #391 scope-only spec, with implementation waiting for #758's current tranche checkpoint. No engine production change or dependency migration is justified. Amendment: upstream contribution policy requires human submission; this agent may prepare, test and review the patch and PR body, but must not submit an upstream PR or push an upstream contribution.

## Read-only findings

Engine baseline inspected: c2c1368dcf4b8984fa7bdf8add5807e4336492ae, clean at inspection. Matching GitHub #391 is OPEN; no local numbered 391 spec exists. Root must create/synchronize that spec before implementation, retaining its original GitHub title/number and superseding the historical implementer/branch assignment with the user's Astra medium scope, Luna max implementation, Astra medium verification workflow.

`crates/session/Cargo.toml:17` pins registry json-syntax exactly 0.12.5 with defaults disabled. Cargo.lock checksum is 044a68aba3f96d712f492b72be25e10f96201eaaca3207a7d6e68d6d5105fda9. `crates/session/src/json_preflight.rs::Scanner::object` refuses immediate closing brace after whitespace before dependency Value parsing; #387 grammar regression tests exist. Preserve these production bytes and existing tests. Dependency policy's session-parser section already requires the exact audited parser choice. The issue's original no-patch default remains appropriate: explicitly record no `[patch]`, no registry bump, no guard removal.

Current upstream main is d77cc66527968b1fa36ceed2c0d050da9a14608e, last push reported 2026-06-21. The empty-object branch still consumes `}` and returns without end_fragment; its caller does not finalize the missing fragment elsewhere. CodeMap reserve still initializes zero volume and empty span. This is source evidence, not an executed reproduction in this scope turn.

Upstream main now uses `JsonValue` and `ParseJson` and Rust edition 2024 / rust-version 1.85.0, despite Cargo.toml still saying version 0.12.5. Its number/object dependencies have changed. A git patch against current main is not API-compatible with the engine pin. The registry sparse index's latest record is non-yanked 0.12.5 (2024-07-03), matching engine checksum. GitHub has no releases and latest tag is v0.12.5 at 603af1dc4aa7782ecbd05c570a450f25500436b9. Do not confuse unreleased main's manifest version with the released artifact.

All available issues/PRs were examined for duplicates: issues 9 and 8 open; 3,2,1 closed; PRs 10 and 7 open, 6,5,4 merged. #9/#10 discuss CodeMap provenance helpers/documentation, not empty-object finalization; PR10 changes only src/lib.rs. PR7 is dependency refresh and does not modify parse/object.rs. No existing fix/report was found. Repeat this small read-only audit immediately before preparing the final human submission packet to avoid a race/duplicate.

Sources:
- https://github.com/misofm/engine/issues/391
- https://github.com/timothee-haudebourg/json-syntax/blob/d77cc66527968b1fa36ceed2c0d050da9a14608e/src/parse/object.rs
- https://github.com/timothee-haudebourg/json-syntax/blob/d77cc66527968b1fa36ceed2c0d050da9a14608e/src/parse/value.rs
- https://github.com/timothee-haudebourg/json-syntax/blob/d77cc66527968b1fa36ceed2c0d050da9a14608e/src/code_map.rs
- https://github.com/timothee-haudebourg/json-syntax/blob/d77cc66527968b1fa36ceed2c0d050da9a14608e/Cargo.toml
- https://github.com/timothee-haudebourg/json-syntax/pull/10
- https://github.com/timothee-haudebourg/json-syntax/pull/7
- https://index.crates.io/js/on/json-syntax
The crates.io REST endpoint returned 403; the official sparse-index endpoint succeeded. No uncertainty about latest release is concealed by that REST failure.

## Smallest closable slice and allowed paths

Engine implementation: only `crates/session/src/lib.rs`, inside its existing cfg(test) module. Add one direct-dependency unit sentinel named `json_syntax_empty_object_code_map_regression`, using local `use json_syntax::Parse as _;` and `json_syntax::Value::parse_str(r#"{"a":{},"b":1}"#)`. Assert slot 3 volume == 1 and span endpoints == (5,7). Mark `#[ignore = "json-syntax 0.12.5 empty-object CodeMap defect; prepared patch tracked at https://github.com/misofm/engine/issues/391; rerun on dependency updates"]` (use engine issue https://github.com/misofm/engine/issues/391 as the durable pointer until a human submits the upstream PR). Do not use should_panic or assert the defective behavior as success. The sentinel intentionally fails when explicitly run against today's pin; normal tests must report it ignored. Its future pass only triggers dependency/guard review; it does not authorize automatic guard removal because schema policy remains independent.

Root-controlled evidence paths: `.github/ISSUE_SPECS/391-json-syntax-upstream-empty-object-code-map.md` (or root-chosen matching filename), and the existing issue index only if this repository's actual indexing convention requires a row. Store concise decision and command/verdict evidence there. No new generic harness, benchmark, fixture corpus, policy rewrite, Cargo edit, or runtime code. Avoid overlap with #307; root owns all repo checkpoints and remote writes.

Upstream separate fresh checkout, never engine's shared implementation tree: exactly `src/parse/object.rs` (add `parser.end_fragment(i);` immediately after consuming the empty closing brace) and `src/code_map.rs` (small existing unit-test module additions). Use upstream JsonValue/ParseJson APIs. One regression should assert the complete 7-entry map for `{"a":{},"b":1}`: spans/volumes [(0..14,7),(1..7,3),(1..4,1),(5..7,1),(8..13,3),(8..11,1),(12..13,1)]. Add compact empty root `{}` and whitespace `{ \n }` checks to prove full closure span; existing nonempty-object tests provide controls. No generated parser corpus or unrelated cleanup. If actual code inspection changes these APIs/offsets, correct expectations from source positions, never weaken the defect assertion.

## Execution and discriminating gates

1. Root revalidates latest engine base: pin, guard, test module, policy, #387 delivery and #391 state. #307 is delivered. Confirm #758's current sole implementation tranche is checkpointed before any #391 implementation edit; integrate latest main before #391 verification and revalidate intervening relevant changes. Re-audit upstream main and issues/PRs; if fixed meanwhile, cite exact fix and skip duplicate correction/PR, retain sentinel while engine pin remains defective.
2. In the separate upstream checkout, add the named regression tests first and run a focused `cargo test --lib code_map` on the recorded unmodified-production base. Require actual assertion failure at empty-object volume/span, not build failure. Add the one-line fix; run the same focused tests successfully, then proportional upstream full `cargo test` and `cargo test --no-default-features`, plus the upstream contribution gates `cargo check`, `cargo clippy` and `cargo fmt --check`. Compilation must have no warnings; capture Clippy and formatter diagnostics without expanding into unrelated cleanup. These gates are mandatory under the discovered contribution policy, not optional touched-file substitutes. If unrelated pre-existing failures occur, preserve evidence and do not expand into upstream cleanup; verifier assesses focused proof and candid delivery limitations.
3. In engine, run `cargo test --locked -p session --lib json_syntax_empty_object_code_map_regression -- --ignored --exact` with the actual fully qualified `tests::...` name, so exactly one test runs. Expected: one assertion failure for volume 0 versus 1. Capture real exit status; a zero-test success is invalid. Prefer final exact command `cargo test --locked -p session --lib tests::json_syntax_empty_object_code_map_regression -- --ignored --exact`.
4. Run `cargo test --locked -p session` and touched-file rustfmt check. Normal run must pass and visibly list the new sentinel ignored with explanation. Existing #387 parser grammar cases must pass. Assert diff shows unchanged Cargo manifests/locks, production parser/preflight and guard tests. No benchmark, fuzz campaign, full workspace DSP matrix or browser rebuild is required for this cfg(test)-only engine delta; ordinary delivery CI still applies.
5. Astra medium independently reviews exact engine and upstream heads plus before/after evidence. Maximum five total coherent attempts with one adversarial verdict per attempt; attempt 1 Luna max, root assigns bounded revisions under user's workflow. Stop/rescope at fifth failure. No retries that silently broaden scope or conceal baseline tooling defects.

## Upstream contribution-policy amendment

Root discovered `/home/bl/misofm/json-syntax-issue391` at d77cc665 and followed its AGENTS -> CONTRIBUTING chain to https://github.com/timothee-haudebourg/admin/blob/main/files/CONTRIBUTING.md, read here from `/tmp/json-syntax-upstream-contribution-guidelines.md`. It explicitly states: “Bots are not allowed to submit changes directly, unless managed by the owner of the repository.” It also requires the closest human manager be made aware. Root reports the user has already been informed via commentary. This is an actual external project rule; no evidence establishes the owner-managed exception.

Accordingly, prepare and test the complete patch, obtain Astra review, preserve a patch artifact and exact PR title/body, and stop only at human upstream submission. Do not create an upstream bot PR, issue, fork/push contribution or comment as a substitute. User submission is the remaining external boundary, not permission to skip the useful local work. A general confirmation alone does not establish the upstream-owner exception. Use neutral branch names, conventional commits, small coherent commits, and no advertisements/AI branding in branches, messages, PR prose or coauthor trailers. Do not add an AI company coauthor. The prepared PR must summarize the change and disclose actual validation results; no breaking change is introduced by this fix.

## Delivery

Root owns the concrete human submission packet: reviewed patch against current upstream main, concise conventional-commit title, PR body explaining missing finalization, minimal input, expected CodeMap slot, and baseline failing/corrected passing commands. Preserve files and commit identities outside any disposable checkout. Do not file a redundant upstream issue solely for bookkeeping. If a duplicate appears, record the existing PR/fix instead. Upstream scope requires a submitted PR, not maintainer merge/release; this agent's packet alone does not satisfy that remaining deliverable.

Root may commit and deliver the engine sentinel/evidence in an engine PR, following its ordinary authorized workflow and reporting partial status honestly. Keep GitHub #391 OPEN until the human upstream submission deliverable is observed. Before submission, the sentinel's ignore reason and engine record point to engine #391 and the prepared patch, never a nonexistent PR. Once submitted, record the actual upstream URL and commit and synchronize evidence. Close engine #391 only after submission, PASS, engine evidence upstream and required GitHub synchronization. Describe an unmerged upstream PR as submitted/pending. No maintainer merge or fixed registry release may be claimed until observed. Registry adoption/guard reconsideration is future work, not hidden in this bugfix. Remove completed clean pushed worktrees only after retained evidence/delivery conditions are met; retain branches/history and any unique work.

No tests/builds, implementation, commits, remote writes or repository edits were performed during this scope task. Only this /tmp plan was written.


## Current-base readiness amendment — main 644756ff

APPROVED for root to create `.github/ISSUE_SPECS/391-json-syntax-upstream-empty-object-code-map.md` (matching existing GitHub #391, not a duplicate issue) and synchronize its scope/evidence now. This is scope-only authorization; #391 Luna max implementation waits until root has committed #758's current implementation tranche. Integrate latest main before Astra medium verification.

Read-only revalidation observed clean primary main `644756ffb59a2950fc3fa7b2c91b7f9f7435b485`, which contains merged #307 via PR #757. Relative to original c2c1368d baseline, `crates/session/` and Cargo.lock are unchanged; dependency-policy changes only remove retired sidecars from an unrelated package enumeration. Exact json-syntax 0.12.5 pin, empty-object refusal, existing grammar fixtures and cfg(test) sentinel insertion point remain valid. No numbered #391 spec was present in primary checkout at inspection.

#755 worktree is at `173cbfc6` (root reports Astra PASS and PR #759 CI). Local changed paths are its spec, `.github/workflows/npm-publish.yml`, `.github/workflows/qualification.yml` and `scripts/test-npm-publish-modes.py`. #758 worktree is at `e97f8e41`; its spec permits audit/vectorization source, vectorization mutation script and two audit docs, and inspection observed uncommitted `tools/audit/src/vectorization.rs`. Neither scope intersects #391's engine unit-test-only path or numbered spec. No changes, tests or commands were run in those worktrees beyond read-only status/spec inspection. Root's reported remote review/CI state was not independently queried in this no-remote-action revalidation.

The upstream contribution-policy amendment remains binding and was re-read from the retained actual policy: fully prepare/test/review a patch and human PR packet, no bot upstream submission, conventional commits and neutral branches with no branding/coauthor advertisements; run required check/test/fmt/clippy gates. Sentinel ignore reason points to engine #391/prepared patch until an actual human-submitted upstream PR exists. Engine sentinel/evidence may ship once green, but #391 remains OPEN until the human upstream submission deliverable is observed; report engine-only delivery as partial. Neither #755 nor #758 delivery changes this external boundary or authorizes a dependency patch/upgrade or guard removal.

This amendment changed only this `/tmp` plan. No repository edits, tests/builds, commits or remote actions were performed.

## Original issue report and decision history

Follow-up to #387 (option 3, which that issue asked for **in addition to** the preflight refusal, not instead). #390 lands the in-tree refusal so no `{}` document can reach the parser's broken CodeMap through `parse_session_json`. The parser defect itself remains: json-syntax 0.12.5 never calls `parser.end_fragment(i)` for an empty object (`src/parse/object.rs:26-29`; the empty-array branch in `src/parse/array.rs` does), so the reserved CodeMap entry keeps `span = p..p, volume = 0` (`src/code_map.rs:15-22`) and `IterMapped::next` advances `2 + 0` instead of `2 + 1` (`src/object/mod.rs:795`). Any future direct consumer of `json_syntax::Value::parse_str` in this workspace would hit it.

## Scope

1. Open an upstream PR against `timothee-haudebourg/json-syntax` adding the `end_fragment` call for the empty-object branch, with a regression test built from the evidence in #387 (`{"a":{},"b":1}` must give `[3] span=5..7 volume=1`). Link it here.
2. Until it ships, decide whether to carry a `[patch.crates-io] json-syntax = { git = ..., rev = ... }` pointing at the fixed commit. This is a Cargo dependency change on a shipped crate and needs an owner ruling (docs/REALTIME_DEPENDENCY_POLICY.md session-parser entry). Default recommendation: do NOT patch before launch; the preflight refusal makes the bug unreachable, and a git dependency adds supply-chain surface a week out. Revisit when upstream releases.
3. Either way, add a `crates/session` unit test that parses `{"a":{},"b":1}` through `json_syntax::Value::parse_str` directly and asserts the CodeMap volume, marked `#[ignore]` with the reason until the upstream fix is in, so the day the dependency is bumped the test flips and the preflight guard can be reconsidered.

## Assignment

| | |
|---|---|
| Implementer | Sonnet |
| Verifier | Fable 5.1 |
| Branch | `sonnet/391-json-syntax-upstream` off `main` |
| Land after | #390 |
| Class | N/A (control plane) |

Not scheduled before launch except item 1 (the upstream PR costs nothing in-tree).

## Attempt 1 implementation checkpoint

Luna max completed the bounded engine sentinel and upstream patch, then paused. Upstream local commit `64446eeb74b715dc49c6cbb78682404d8705dc18` on base `d77cc66527968b1fa36ceed2c0d050da9a14608e` adds the one missing end_fragment call and three CodeMap regression cases. Baseline focused tests failed on the intended zero volume/empty spans; corrected focused, full default and no-default-feature tests pass. Cargo check and Clippy exit successfully with pre-existing number.rs warnings. Whole-repository fmt check reports unrelated baseline differences; the two touched files pass direct rustfmt. No unrelated formatting or dependency changes were made.

Engine explicit ignored sentinel runs exactly one test and fails at volume 0 versus 1 (exit 101), proving the current registry pin remains defective. Normal locked session tests and package formatting pass with the sentinel ignored. Production guard, dependency manifests/lock and existing grammar tests are unchanged. Logs are `/tmp/issue391-attempt1-*`; prepared diff SHA-256 is `8c00065ac479c8d85fc49a03c3b6cdca564f933a6a7701d8b84c3ab5225c4536`.

No upstream PR has been submitted. Upstream contribution rules require human submission; root will preserve a reviewed patch and PR text. The decision is explicitly no git patch, no dependency bump and no guard removal. Astra medium verification is pending. #391 remains open until its upstream submission deliverable exists, even if the engine sentinel ships.

## Attempt 1 adversarial verdict: implementation PASS

Astra medium PASS for engine `e01dd6167dcf2e1cf354a9194d65ac8ab9c80ebd` and upstream `64446eeb74b715dc49c6cbb78682404d8705dc18`. Independent upstream red reproduction failed exactly the three new cases; corrected focused cases passed. Independent engine sentinel ran exactly one test and failed on the intended 0-versus-1 assertion. The earlier implementation log's private-field compile failure is not counted as red evidence; its corrected assertion run is. Baseline exports independently confirmed identical pre-existing upstream check/Clippy warnings and whole-tree formatting differences; touched files pass formatting. Production engine and dependency bytes are unchanged.

Integration confirmation: reviewed engine content is unchanged at `2d3345a71c3ba4286a64294a2b291e78c33c7527`, which incorporates main `96415c2f`; only independent #755 files were added. This remains the same successful attempt.

The reviewed human submission packet is preserved at `/home/bl/misofm/engine-upstream-contributions/issue391/`, including standalone patch, accurate PR body, verdict, baseline/final logs and validation lockfile (not part of the patch). Upstream main and issue/PR history were rechecked after review: no duplicate fix, main still d77cc665. No upstream writes or submission have occurred. Engine delivery is partial completion only; #391 stays OPEN pending human submission required by upstream contribution rules.
