# Share checked scan helpers and adopt them in workspace and rack policy

Parent: #306 / #349 TOOL-11 step 1. Issue #400; local spec `400-tool-11-shared-gate-foundation.md`. Depends on merged #371 as program ordering; does not edit its gate. The complete standing contract is embedded below.

Smallest closable outcome: the proven workspace scan wrapper has one shared owner, and all three rack policy scans now distinguish matches/no-match/scan error. Workspace's existing violation/search-error behavior and exact dependency policies remain intact.

Allowed implementation paths:
- scripts/lib/gate.sh (new, small shared library)
- scripts/check-workspace-policy.sh
- scripts/check-rack-policy.sh
- scripts/test-workspace-policy.sh
- scripts/test-rack-policy.sh
- scripts/test-gate-lib.sh (new focused helper tests)
- this numbered issue's spec/evidence; no other scripts or Rust.

Implement:
1. Extract current workspace `scan_forbidden` and label-aware `fail` into the shared library; retain the existing explicit if-capture status handling and diagnostic details. A no-glob request must preserve current rg traversal exactly. Source location is based on BASH_SOURCE, not the fixture root. Do not modify unrelated workspace positive queries or manifest walkers in this child; inventory those for the parent before closure.
2. Move rack's production-dependency extraction into one named library helper usable by later children; preserve expected sorted package-name output and scoped [dependencies] parsing. Handle extractor read/execution failures explicitly. No TOML parser framework or dependency.
3. Replace rack's three if-rg bans: unsafe over rack/rack-compiler Rust; forbidden control/I/O/threading names over rack/src and Cargo.toml; runtime feature detection/specialization over rack/rack-compiler. Preserve patterns, globs, paths and failure prefixes. No new allowed dependencies or bans.

Acceptance:
- `bash scripts/check-workspace-policy.sh` and `bash scripts/check-rack-policy.sh` pass the real tree.
- Existing workspace and rack mutation suites remain green.
- New real deletion of `crates/rack/src` in a valid fixture goes red through the second scan, although manifests remain valid; make the first scan clean so the intended error is discriminated. Existing positive fixture and actual forbidden code still behave correctly.
- Helper tests cover clean=1, match=0, missing root=2, and an injected non-path execution error; include a match plus a missing path in one scan to reproduce the original “prints violations then passes” defect. Verify no shell-option/cwd changes, diagnostic labels and invocation from a foreign working directory with fixture root.
- Dependency helper on existing representative manifests preserves output, ignores dev-dependencies, and rejects unreadable/missing input rather than comparing an empty result as success.
- `bash scripts/test-gate-lib.sh`, `bash scripts/test-workspace-policy.sh`, `bash scripts/test-rack-policy.sh`, shell syntax/diff checks and proportional CI pass. Full workspace candidate/main counts unchanged; no timed workloads.

Pause after a coherent focused-green tranche for root's exact-path commit/push; Astra reviews actual PR. Luna one attempt, Sol at most two on failure. Parent #306 stays OPEN after this child closes.

## Standing contract


- A search result has THREE outcomes: matches, clean no-match, execution failure. A required path/read/parse error must never be interpreted as clean. Preserve stdout/stderr needed to distinguish them.
- Explicit conditionals capture command status; do not rely on set-e inside functions invoked from conditionals, pipelines/process substitutions, or standalone `! command`. Helpers must not toggle caller shell options or install caller traps/change cwd.
- Resolve sourced library by the script's own physical location before cd into a fixture root. A fixture-root argument selects data to inspect, never a different helper implementation. Preserve existing script CLI/environment and diagnostic prefixes.
- Preserve regex/glob/allowlist semantics. Filtering legitimate exceptions happens AFTER a successful checked source scan; an empty filtered result is allowed, failed source traversal is not. Do not use `--glob '*'` as a blind replacement for “no glob,” because it can alter ignored-file traversal.
- Known required roots remain required; no blanket filter-to-existing-directories or mkdir workaround. If an optional root is currently legitimate, document that specific policy and retain missing-required-root red cases.
- Expected discovery must be non-vacuous. Capture producer failures before a consumer loop and assert nonempty output when the policy requires at least one input. Record any legitimate empty-set case explicitly. All original #306 nine-loop debt must be assigned in the frozen per-child call-site inventory; if a remaining original site lies outside the 21 roster, record a stateless bounded successor before parent closure rather than silently omitting it.
- Every migrated gate has a clean positive control, retained old violation mutations and a new missing-root red case. Prove the changed helper is actually reached, not only that an unrelated earlier manifest check fails. Where deletion is intercepted by prior checks, additionally inject a controlled rg failure at the relevant scan while all required metadata remains valid.
- Red helpers explicitly reject unexpected success and distinguish intended predicate failure from missing tools/syntax errors. Each new helper-level failure class gets at least one counter-mutation demonstrating the assertion is live.
- No Cargo tests for prose/shell implementation mirrors; existing full workspace unchanged-count requirement is retained at coherent child boundary. Run all existing affected shell suites and applicable current required CI. No artifact byte regeneration, benchmark launches or publication solely for gate extraction.


## Readiness and assignment

Prerequisite #371 is merged as `2a18b315067898a94fdc02e8f8b80f07b788ff89` and verified CLOSED. Its actual realtime policy has 42 regions in 12 files. This issue is a queued brief, not an implementation claim. Freeze the exact base and per-site inventory at assignment. Current roles: Astra scope/final PR review, Luna one implementation attempt, Sol at most two retries on failure, then preserve evidence and rescope; root owns checkpoints, pushes and GitHub synchronization. Do not edit another owner’s worktree.

## Program closure

Parent #306 and the broad #349 TOOL-11 finding remain OPEN until #400, #401, #402 and #403 are upstream and all original 21-gate, five-extractor and nine-loop obligations are resolved. Each child closes only its named outcome; any discovered original-scope site outside the roster requires a numbered successor before parent closure.

## Frozen implementation inventory — 2026-09-05

# #400 foundation call-site inventory

Read-only inventory at `cf4fcf64` in `/home/bl/misofm/engine-306-plan`. This freezes the bounded #400 implementation surface. It does not authorize changes to the parent-accounting sites below.

## Exact #400 production sites

### `scripts/check-workspace-policy.sh`

The local `fail` at lines 10–13 prints `workspace policy failure: <message>` to stderr and exits 1. The local `scan_forbidden` at lines 20–51 captures `rg` output and status explicitly: 0 prints matches and fails with the supplied description; 1 is clean; every other status prints rg output and fails, distinguishing missing roots (`scan could not run ... missing search path(s)`) from other execution errors (`scan errored`). Extraction must preserve that contract and the workspace diagnostic prefix.

Its five call sites are:

| line | description | pattern / glob | required roots |
|---|---|---|---|
| 182 | hardware ISA Cargo features are forbidden | `^[[:space:]]*(simd128|neon|avx2|fma)[[:space:]]*=` / `Cargo.toml` | `crates hosts tools sidecars` |
| 249 | retired delivery-codec Cargo identity is forbidden in the lockfile | `$retired_delivery_codec_pattern` / `Cargo.lock` | `Cargo.lock` |
| 253 | compiled track-capacity identifiers are forbidden | `\b(MAX_TRACKS\|MAX_TRACK_COUNT\|DEFAULT_MAX_TRACKS\|TRACK_LIMIT)\b` / `*.rs` | `crates hosts tools sidecars` |
| 261 | prelaunch live-product identities must not claim a later generation | `$prelaunch_later_generation_pattern` / `*` | `crates hosts tools sidecars` |
| 268 | AudioWorklet processor implementation classes must be unversioned | `$versioned_worklet_implementation_pattern` / `*.js` | `crates hosts tools sidecars` |

The library API therefore needs a real optional-glob representation: these calls all supply a glob, while rack calls below do not. Omitting a glob must omit `--glob` entirely; using `--glob '*'` changes traversal. Resolve `scripts/lib/gate.sh` from the policy script's physical `BASH_SOURCE` before `cd "$workspace_root"`.

### `scripts/check-rack-policy.sh`

The local `fail` at lines 9–12 prints `rack policy failure: <message>`. The production dependency extractor at lines 14–26 parses only the exact `[dependencies]` section, accepts keys matching `[A-Za-z0-9_-]+` with optional `.workspace`, strips `.workspace`, and sorts output. Frozen expectations are:

- `crates/rack/Cargo.toml`: `effect-contract`, `engine`
- `crates/rack-compiler/Cargo.toml`: `effect-contract`, `engine`, `rack`

The shared extractor must retain those sorted newline-separated values, ignore later sections including `[dev-dependencies]`, and surface missing, unreadable, or awk/execution failure. A failed extraction must not become an empty string that merely fails an equality assertion ambiguously.

The three direct bans to migrate are:

| line | pattern / glob | required roots | failure text |
|---|---|---|---|
| 36 | `\bunsafe\b` / `*.rs` | `crates/rack crates/rack-compiler` | `rack source has unsafe code` |
| 39 | `\b(session\|effect_compiler\|graph\|builtins)::\|std::(fs\|net\|thread\|sync)\|log::\|tracing::` / no glob | `crates/rack/src crates/rack/Cargo.toml` | `control-plane, I/O, threading, synchronization, or logging leaked into rack render code` |
| 43 | `target_feature\|is_x86_feature_detected!` / no glob | `crates/rack crates/rack-compiler` | `feature detection or target-feature specialization leaked out of core dispatch` |

Patterns, paths and messages are contract data. The required deletion mutation is specifically `crates/rack/src`: keep both manifests valid and the first unsafe scan clean so failure comes from the second scan's missing required root.

## #400 test mapping

- `scripts/test-workspace-policy.sh` already has a valid fixture/control, the established policy mutations (including `MAX_TRACKS`, later-generation identity and versioned worklet class), and a missing-`sidecars` regression at lines 460–467. Preserve these.
- `scripts/test-rack-policy.sh` currently has a valid fixture, unsafe mutation, and dependency-boundary mutation only. It lacks direct mutations for the second control/I/O scan, the third feature-detection scan, missing `crates/rack/src`, dev-dependency exclusion, and extractor read/execution failures.
- New `scripts/test-gate-lib.sh` must directly prove rg statuses 0/1/2 plus injected non-path execution error; the combined real match + missing root case; labels; foreign-CWD sourcing; and unchanged caller cwd/options. Negative assertions must discriminate the intended status/error rather than accept any nonzero command.
- Extractor tests should use representative manifests, add a forbidden-looking dev dependency that remains ignored, and separately prove missing/unreadable and injected execution errors. An unreadable test must remain meaningful under privileged/root execution (a mode-bit-only test can still be readable).

These map all #400 acceptance requirements without adding a parser framework or touching another gate.

## Workspace sites intentionally left for parent accounting

The #400 brief expressly excludes unrelated positive queries and manifest walkers. Current exact sites are:

- Positive-presence queries: root and fuzz Apache licenses (94, 97), third-party inventory reference (100), per-package inherited license inside the manifest loop (120), the required x86 target table when ISA directives exist (287), and approved ISA pin presence through the filtered directive logic (277–291).
- Required/possibly-required discovery loops: `package.json` (108), `package-lock.json` (114), workspace manifests below `crates hosts tools sidecars` (180), whole-tree retired-directory discovery (208), and root spill `.fingerprint` discovery (311). #400 must not silently classify their empty-set semantics. Parent #306 must assign or explicitly rule each against the frozen nine-loop debt before closure.
- Bespoke scans that are not simple #400 helper replacements: comment-stripped per-manifest retired-codec scan (243), filtered `.cargo` ISA scan/presence logic (277–291), and direct `[build]` ban (294). Leave them unchanged in #400 and account for their producer/error behavior at parent closure.

## Original #306 obligations and roster check

The four children cover exactly 21 named gates: #400 has 2; #401 has 7; #402 has 6; #403 has 6. All gates explicitly named in the original #306 body map into that roster, allowing for the current filename `check-native-pcm-runner.sh`: effect-runtime, conformance, lane, realtime, builtins, graph, rack, effect-interchange qualification, native PCM runner, and unfused seal are present. The original phrase `check-fast-db-seal.sh` has no current file and no named child row. Before closing #306, root must establish whether it was renamed/removed before this freeze or create a stateless successor for any surviving obligation; it is not a #400 blocker.

Current `done < <(find ...)` sites extend beyond #400 (notably graph, conformance, lane, bench-policy, and realtime-audit-leak) and are already in the #401–#403 gate roster. The five workspace loops above remain the immediate explicit parent-accounting exception. A full nine-loop provenance mapping should be frozen before parent closure; it need not expand or delay this foundation slice.

## Readiness verdict

#400 is implementation-ready on the clean `codex/400-shared-gate` branch at `cf4fcf64`. No source blocker was found. The implementation should stay confined to its seven allowed paths, with the workspace/rack semantics and tests above as the acceptance map. Parent accounting for the deferred workspace sites and the historical `check-fast-db-seal.sh` name remains open and should not be represented as completed by #400.


Root assigns Luna attempt 1 on the post-#371 source at `2a18b315067898a94fdc02e8f8b80f07b788ff89`, with planning checkpoint `cf4fcf64`. #388 has only immutable qualification outstanding and no overlapping implementation paths. No production Rust, artifacts, or other gate migrations are authorized in this slice.

## Attempt 1 checkpoint — pending adversarial verdict

Luna extracted the shared helper and migrated the five workspace and three rack calls. Root reran the real workspace/rack gates, both existing mutation suites, and new helper suite: all PASS. This is a useful buildable checkpoint, not acceptance. Root identified unresolved review concerns: the dependency helper still uses an awk/sort pipeline that may depend on caller pipefail; the combined match-plus-missing-root test removes the match first; diagnostic/error-class assertions, dependency extraction failure tests, and the required valid-fixture missing rack/src test are not yet demonstrated. Astra must review against the complete spec before any further implementation. On FAIL, Sol receives the next attempt under the user workflow.

Focused logs: `/tmp/engine-400-luna-gate-lib.log`, `/tmp/engine-400-luna-workspace-mutations.log`, `/tmp/engine-400-luna-rack-mutations.log`, `/tmp/engine-400-luna-workspace.log`, `/tmp/engine-400-luna-rack.log`. No workspace-wide test, artifact rebuild or benchmark is claimed for this attempt.

## Astra attempt 1 verdict — FAIL

# Astra #400 attempt 1 verdict

**FAIL — bounded Sol revision required at exact pushed checkpoint `abbab60889f8989432a7b552504b6b876b6a00e0`.** One Luna attempt is consumed. Preserve this useful checkpoint; do not give Luna an uncounted correction pass.

The shared forbidden-scan helper captures rg status explicitly and preserves no-glob semantics, roots, patterns and diagnostics. Both real callers source the physical script library before fixture-root cd. The five workspace and three rack call migrations are within scope. However green focused suites do not satisfy the frozen acceptance:

1. **Real shared-helper correctness failure.** `gate_toml_dependencies` captures `awk ... | sort` but checks only that pipeline's status, which depends on caller pipefail. I independently sourced the helper in `bash -c` with pipefail disabled and called it in an if/command-substitution condition on a nonexistent manifest: awk printed its missing-file error, yet the helper returned success with empty output. This violates explicit extractor read/error handling and the no-caller-shell-option-dependency contract. A partial output plus failed awk has the same issue. Capture/check extraction and sorting separately (or an equally explicit self-contained status mechanism); do not toggle caller shell options. Preserve stderr and fail diagnostics and existing dependency parsing/sorted output.

2. **Original regression test is absent despite its label.** test-gate-lib.sh deletes match.txt before the combined scan, so its “combined” case exercises clean data plus a missing path, not the original match-plus-error bug. Keep a real match during that test and assert both the match evidence and missing-root/error diagnostic. Current failure cases discard stderr or only check nonzero; syntax/tool failures could satisfy them. Require diagnostic prefix/class and explicit unexpected-success refusal, plus the frozen counter-mutations.

3. **Extractor acceptance tests are missing.** The new suite has no gate_toml_dependencies invocation. Add representative sorted output, dev-dependency exclusion, missing/unreadable or controlled read failure, injected awk failure (including valid partial output) and sorting failure. Run helper error cases with pipefail both off/on and conditional invocation; demonstrate no cwd/options mutation. A mode-bit-only unreadability case is insufficient under privileged execution.

4. **Required real rack mutation is missing.** test-rack-policy.sh is unchanged and still tests only unsafe/dependency bans. Add deletion of crates/rack/src in an otherwise valid fixture, leaving both manifests valid and the first scan clean; assert the second scan's missing-path failure. Add frozen direct control/I/O and feature-detection mutations and dependency-extraction failure classification. Keep the clean fixture and existing red cases. Foreign-cwd coverage should use the valid selected fixture and prove it sources the repository helper, not fixture contents.

Sol attempt 2 is limited to these existing allowed helpers/callers/tests and issue evidence. Do not broaden into deferred workspace walkers/positive scans, the new parent-accounting successor, other gate migrations, parser improvements or generic harness construction. Preserve current parsing, diagnostic prefixes, glob semantics and all existing tests. At one coherent pass completion, root checkpoints and pushes exact paths, then Astra gives one adversarial verdict. The remaining budget is at most two Sol attempts total, with hard stop/rescope after the third failed implementation attempt.

Required next evidence: real workspace/rack gates, all three named mutation/helper suites, shell syntax and diff hygiene, explicit demonstrations of the four missing acceptance groups above. Final full-workspace unchanged-count comparison and actual pushed-PR review/required CI remain delivery gates after focused acceptance. No benchmark or artifact build is warranted. This review inspected source/spec and ran only the tiny non-mutating missing-manifest shell repro; it did not edit the repository or GitHub.


Root assigns the bounded second attempt to Sol. #404 is the separately numbered parent discovery successor and remains outside this implementation.

## Sol attempt 2 focused checkpoint — pending adversarial verdict

Sol separated dependency extraction and sorting into independently status-checked operations, without changing caller shell options or relying on caller `pipefail`. Focused tests now preserve a real match alongside a missing root and assert both facts; exercise match, clean, missing-path and non-path scan outcomes; cover dependency sorting/section scope plus missing, partial-output awk and sort failures with `pipefail` on/off and conditional invocation; and retain caller cwd/options. Rack mutations now directly reach unsafe, control/I/O, feature detection, missing `crates/rack/src`, manifest and extractor error classes, plus a valid foreign-CWD fixture that cannot substitute its own helper.

Focused PASS logs: `/tmp/engine-400-sol-gate-lib.log`, `/tmp/engine-400-sol-rack-mutations.log`, `/tmp/engine-400-sol-workspace-mutations.log`, `/tmp/engine-400-sol-rack.log`, `/tmp/engine-400-sol-workspace.log`. Shell syntax passed for the shared library, both gates and all three affected suites. This is an uncommitted focused-green checkpoint for root to inspect and commit before Astra review; no full workspace run, benchmark or artifact work is claimed.

## Astra attempt 2 verdict — focused PASS

# Astra #400 attempt 2 verdict

**PASS for focused acceptance at exact pushed head `58e1575038c32696e4114dbbd253b985284793d3`.** Proceed to the full-workspace/final-delivery gates; no further source revision requested. This is Sol's successful second implementation attempt, not final PR approval.

Reviewed the frozen #400 contract and four failures from abbab608. Extraction now checks awk before sorting and separately checks sort failure, independent of caller pipefail. The previous missing-manifest silent-success repro now fails explicitly with pipefail off. Source parsing, sorted output, no-glob behavior, caller cwd/options and existing workspace/rack policies remain unchanged. No deferred #404 scan was pulled into the implementation; parent/successor spec additions are root accounting work.

Independent reruns PASS: test-gate-lib.sh, test-rack-policy.sh, test-workspace-policy.sh and both real policy gates. The combined scan now retains the actual forbidden match alongside the missing path and asserts both evidence and error class. Helper cases cover representative sorted dependencies and dev exclusion, missing input in direct/conditional invocation with pipefail off/on, partial-output awk status 7 and sort status 8. Rack tests now include the actual missing-src fixture, both omitted bans, explicit extraction errors and a foreign-CWD fixture whose fake helper must not be sourced.

I additionally performed reviewer counter-mutations in disposable temporary copies, leaving repository files unchanged: restoring the old extractor is rejected by the missing-manifest assertion; accepting a real forbidden match is rejected as unexpected success; changing the missing-root diagnostic is rejected; accepting sort failure is rejected by the sort-status assertion. These supply the frozen assertion-liveness evidence absent from the implementation's recorded focused logs. Root should record these checks in the final issue/PR evidence; no new test framework or implementation attempt is needed to do so.

Limits: the shell suites intentionally do not solve workspace's deferred positive queries/walkers (#404), other policy gates (#401–#403), or broaden the original dependency parser grammar. A printf feeding already captured text to sort is not the former unchecked external awk producer. No remaining behavior blocker was found within this API/shape.

Root must now complete the unchanged-count full-workspace comparison, synchronize final evidence and open/update the actual PR. Astra exact pushed-head review and required qualification SUCCESS remain mandatory before merge. No Cargo, benchmark, artifact rebuild, repository edit or GitHub mutation was performed by this review.


Root normally merged post-#388 main `0c2b283f86b199351b78be99784def7c614c0320` without conflicts after the focused PASS. The implementation is unchanged; final full-workspace comparison against this synchronized base is pending.

## Final integration evidence — 2026-09-05

Baseline synchronized main `0c2b283f86b199351b78be99784def7c614c0320` and integrated candidate `bc8605f9` both completed `cargo test --locked --workspace` with **1552 passed, 0 failed, 24 ignored**, including doctests. Source candidate was immutable during this qualification. Logs: `/tmp/engine-400-main-baseline.log` and `/tmp/engine-400-candidate-workspace.log`. The final checkpoint adds only this evidence and queued issue briefs to that qualified source.

Astra's attempt-2 focused PASS at `58e1575038c32696e4114dbbd253b985284793d3` independently reran all named shell gates and the old missing-manifest reproduction. Disposable-copy counter-mutations proved the assertions reject the old extractor, accepted forbidden match, altered error class and accepted sort failure. Those checks are assertion-liveness evidence, not production/source mutations. Root's conflict-free current-main merge preserves the accepted helper/caller/test implementation and passed the real workspace/rack gates again.

No runtime Rust, artifact, benchmark or publication change is included. #306/#401/#402/#403/#404 and queued #406/#407 remain open for their separate obligations; this issue closes only shared-helper and workspace/rack adoption. Final actual PR review by Astra and required CI SUCCESS remain merge gates.
