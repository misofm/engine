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
