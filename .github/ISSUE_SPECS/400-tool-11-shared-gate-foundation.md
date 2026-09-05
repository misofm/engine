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
