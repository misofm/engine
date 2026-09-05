Found while verifying #305 (the FLAC decoder's move into `sidecars/`). Not a live failure today — but it is the shape that made #305's gate holes invisible, and it is repo-wide.

## The bug

Gate scripts guard violations with:

```sh
if rg -n 'pattern' --glob '...' crates hosts tools; then
  fail "violation"
fi
```

`rg` exits **2** when any listed search path does not exist. That shape reads exit 2 as "no violation" and the gate passes while having searched nothing.

Proven directly against `scripts/check-workspace-policy.sh` on a fixture root containing two real violations and no `sidecars/` directory:

```
rg: sidecars: No such file or directory (os error 2)
crates/miso-engine-lib/Cargo.toml:9:avx2 = []
crates/miso-engine-lib/src/bad.rs:1:pub const MAX_TRACKS: usize = 8;
workspace policy: ok            <-- rc=0
```

It **prints the violations it found and exits 0**. Creating an empty `sidecars/` makes the same tree go red.

## Scope

- ~40 conditional-`rg` sites across `check-effect-runtime-policy.sh`, `check-conformance-boundaries.sh`, `check-lane-policy.sh`, `check-realtime-policy.sh`, `check-builtins-policy.sh`, `check-graph-policy.sh`, `check-rack-policy.sh`, `check-effect-interchange-qualification.sh`, `check-native-pcm-runner-v1.sh`, `check-fast-db-seal.sh`, `check-unfused-seal.sh` and others.
- 9 `done < <(find …)` loops with no non-vacuity assertion — a `find` over a missing or renamed path yields zero rows and the loop body never runs.

**No path is missing on `main` today**, so there is no live silent pass. The risk is entirely prospective: any refactor that renames or adds a top-level tree turns these green-and-blind. #305 is the first change to demonstrate it, and it was caught by mutation testing rather than by any gate.

## Why it matters more than it looks

`sweep.sh` printed **101/101 PASS** while five separate gates were provably blind to a whole tree. A green sweep is not evidence that the gates ran — only that they exited zero.

## Fix

Explicit exit-code handling everywhere the pattern appears:

```sh
set +e
matches=$(rg -n 'pattern' --glob '...' "${roots[@]}")
rc=$?
set -e
case $rc in
  0) fail "violation: $matches" ;;
  1) : ;;                       # clean
  *) fail "search failed (rc=$rc) — a scan root is missing" ;;
esac
```

For the `find` loops, assert non-vacuity: if a scan is expected to visit at least one file, fail when it visits none.

Worth pairing with a mutation-suite row that deletes a scan root and asserts the gate goes **red**, so the property is itself gated rather than re-derived by the next person.

`check-workspace-policy.sh` is being fixed as part of #305 because that PR creates the missing path. This issue covers the remaining sites.

## Root-approved bounded execution program — 2026-09-05

# Astra #306 program brief, conditional on merged #371

**APPROVED for issue splitting, not an all-21 implementation pass.** Root must number and synchronize the four attached child specs before assigning implementation. #306 remains the umbrella for the original repository-wide scan-error/non-vacuity defect and TOOL-11 step 1; it is NOT closed by the first child. #349 TOOL-11 stays open until all children satisfy the original acceptance. No original claim is silently removed.

Condition: #371 must be merged, remotely CLOSED and synchronized; inspect its actual merged gate to confirm the expected 12 marked files/42 regions and preserve all its named mutations. Current user authorizes blocked-work recovery and model roles override historical Qwen/Fable instructions: Astra scope/review, Luna one attempt, Sol at most two retries, then hard stop/rescope. Root owns issue numbers, claims, branches, checkpoints, pushes and merges. No work occurs in another owner's existing worktree.

## Why split

The fixed wrapper exists in workspace policy. But the 21 scripts now have 25–445 lines, include direct bans, positive-presence checks, filtered pipelines, process substitutions and bespoke artifact consumers; several have no matching standalone mutation harness. Some contain zero rg sites and do not need an invented search. Their all-at-once rewrite is neither one edit per file nor a safe bounded Luna pass. Four independently useful outcomes preserve the complete roster:

A. Shared helper plus workspace/rack adoption — smallest closable first product slice, fixes three real rack scans and preserves the proven workspace guard.
B. Compiler/runtime boundary gates — 7 gates, shared dependency extraction and checked filtered searches.
C. Realtime/numeric/contract gates — 6 gates, including the post-#371 discovery/error boundary.
D. Qualification/evidence gates — 6 gates, preserving every artifact/workload preflight and avoiding timed execution.

These are child issues with exact rosters in the local numbered specs for #400–#403. Root creates all four GitHub issues/local numbered mirrors in a planning checkpoint and adds reciprocal links to #306/#349 before implementation. Children B/C/D depend only on A, and C additionally on merged #371 (already a program prerequisite). Only one implementation tranche at a time; independent read-only briefing/review may run alongside it. No declarative policy TOML/runner (TOOL-11 step 2), general mutation harness (TOOL-12), new semantic bans, runtime changes or benchmark work.

## Standing contract shared by children

- A search result has THREE outcomes: matches, clean no-match, execution failure. A required path/read/parse error must never be interpreted as clean. Preserve stdout/stderr needed to distinguish them.
- Explicit conditionals capture command status; do not rely on set-e inside functions invoked from conditionals, pipelines/process substitutions, or standalone `! command`. Helpers must not toggle caller shell options or install caller traps/change cwd.
- Resolve sourced library by the script's own physical location before cd into a fixture root. A fixture-root argument selects data to inspect, never a different helper implementation. Preserve existing script CLI/environment and diagnostic prefixes.
- Preserve regex/glob/allowlist semantics. Filtering legitimate exceptions happens AFTER a successful checked source scan; an empty filtered result is allowed, failed source traversal is not. Do not use `--glob '*'` as a blind replacement for “no glob,” because it can alter ignored-file traversal.
- Known required roots remain required; no blanket filter-to-existing-directories or mkdir workaround. If an optional root is currently legitimate, document that specific policy and retain missing-required-root red cases.
- Expected discovery must be non-vacuous. Capture producer failures before a consumer loop and assert nonempty output when the policy requires at least one input. Record any legitimate empty-set case explicitly. All original #306 nine-loop debt must be assigned in the frozen per-child call-site inventory; if a remaining original site lies outside the 21 roster, record a stateless bounded successor before parent closure rather than silently omitting it.
- Every migrated gate has a clean positive control, retained old violation mutations and a new missing-root red case. Prove the changed helper is actually reached, not only that an unrelated earlier manifest check fails. Where deletion is intercepted by prior checks, additionally inject a controlled rg failure at the relevant scan while all required metadata remains valid.
- Red helpers explicitly reject unexpected success and distinguish intended predicate failure from missing tools/syntax errors. Each new helper-level failure class gets at least one counter-mutation demonstrating the assertion is live.
- No Cargo tests for prose/shell implementation mirrors; existing full workspace unchanged-count requirement is retained at coherent child boundary. Run all existing affected shell suites and applicable current required CI. No artifact byte regeneration, benchmark launches or publication solely for gate extraction.

## Closure semantics

Each child can close on its own concrete gated outcome after Astra PASS and upstream evidence, even while #306 remains open. Its checkpoint/tracker note says exactly which scripts now reject scan failures. At each boundary root verifies every migrated row against the frozen original roster/call-site inventory. Close #306 only when all four children (and any explicitly discovered original-scope successor) are CLOSED/upstream, all 21 scripts use the shared applicable helper(s), all five original dependency-extractor copies are consolidated, every original vulnerable site/non-vacuity obligation is fixed or explicitly ruled inapplicable with evidence, every required mutation passes and workspace count remains unchanged. Then close the broad TOOL-11 wave-0 finding on #349. Do not close #306 simply because the foundation child lands.

No repository/GitHub edits, claims, implementations or tests were performed while preparing these briefs. Counts refer to read-only current-tree inspection; exact merged source revisions are frozen in the child issue at assignment.

### Numbered child roster and current readiness

- #400: shared helper and workspace/rack adoption (2 gates).
- #401: compiler/runtime boundary gates (7 gates), depends on #400.
- #402: realtime/numeric/contract gates (6 gates), depends on #400 and merged #371.
- #403: qualification/evidence gates (6 gates), depends on #400.

#371 is merged at `2a18b315067898a94fdc02e8f8b80f07b788ff89`, verified CLOSED, and retains 42 marked regions in 12 files. Children are queued; no implementation has started. Root approves this split without weakening original acceptance. This parent stays OPEN until every child and any original-scope successor is upstream and remotely closed. Child specs carry the complete standing contract.

### Preserved parent accounting from #400 inventory

#400 intentionally leaves workspace positive queries, filtered ISA/retired-codec scans and discovery loops unchanged. Before parent closure, assign or explicitly rule the five current workspace find-backed loop groups and map them against the original nine-loop debt. The historical `check-fast-db-seal.sh` name has no current file: establish removal/rename provenance or create a stateless successor if an obligation survives. These are open accounting requirements, not implicit passes and not blockers for the bounded foundation.

## Final accounting ruling and bounded successor #404

# Astra #306 accounting ruling — 2026-09-05

Read-only review of `/tmp/sol-306-parent-accounting.md`, parent #306 and children #400–#403, and current `/home/bl/misofm/engine-400` scripts. No tests or implementation performed; #400 remains unblocked and unchanged.

1. **Fast-dB historical row: retired/replaced, not an omitted migration.** Repository commit `c41adf56ee4f57e4bf36e509ea0f7588ef440197` explicitly replaces `check-fast-db-seal.sh` (and math-policy) with Clippy resolved `disallowed-methods`; its record covers new-crate, missing-expect and stale-expect mutations. Cite this provenance in #306. The old private-visibility constraint was consciously not reproduced there; do not resurrect it as a #306 scanner task.

2. **Nine non-workspace find-loop mapping confirmed:** #401 owns graph-policy's two and conformance's one; #402 owns lane-policy's two; #403 owns bench-policy's one and realtime-audit-leak's two. The ninth is `check-wasm-realtime-atomics.sh`'s object loop. These are distinct from the five additional workspace-policy find loops. Symbol/call-site identity is authoritative because #400 changes line numbers.

3. **Correction to Sol's ninth-loop disposition:** `object_count > 0` solves only successful-but-empty discovery. `done < <(find ... | sort)` discards producer status. If find prints one legitimate object and then fails, the body can inspect that object, increment the counter and print success. The observation-presence precheck does not cure this: it is a separate pipeline, and its source-level ObservationSlot fallback can also accept after object search failure. Therefore the original ninth-loop completeness/error-propagation obligation survives. Record it as unresolved, not already fixed. This is a static shell-control-flow conclusion, not a claim that an injected mutation was executed during this review.

4. **Root's empty-set policy is sound and adopted:** npm package/lock discovery is optional (clean empty sets pass, preserving Rust-only fixture roots); workspace Cargo manifest discovery across required crates/hosts/tools/sidecars is required and nonempty in aggregate; successful empty retired-directory and root-fingerprint searches are the healthy result. Optional *results* never make producer failure optional. Do not require a package in every individual root; empty sidecars is legitimate. `.cargo` retains its existing optional-directory policy, but any scan actually attempted there must report errors. Known required Cargo/license files stay required.

5. **One bounded successor is warranted**, attached at `.github/ISSUE_SPECS/404-tool-11-complete-discovery.md`. It owns the five workspace discovery loops plus the expressly deferred workspace positive/filtered query producers and the demonstrably surviving ninth Wasm producer. This is the remainder of two gate scripts, not another broad roster. Grouping them closes the parent accounting gap while keeping #400's foundation frozen. It depends on #400; no new dependency or pause is imposed on #400. Root must number/synchronize it before implementation and before closing #306; #401–#403 remain responsible for their eight loops. Parent closure still requires all original scanner obligations, including the existing tracked_paths/awk/filter pipelines, to be resolved with evidence.


#404 now owns the remaining workspace producers/queries and the surviving ninth Wasm loop, depends on merged #400, and is required before parent closure alongside #400–#403. It is queued; no implementation has started. The earlier zero-count-only disposition is corrected explicitly above.

### #401 execution refinement

Astra split its seven-gate implementation into #406 (four public/runtime gates, after #400) and #407 (three compiler/reference gates, after #406). #401 remains their umbrella and cannot close after only one child. This refines execution without dropping any gate, extractor or producer obligation. Both children are queued.
