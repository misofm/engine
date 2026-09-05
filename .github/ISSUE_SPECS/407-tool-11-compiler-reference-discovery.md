# Complete compiler and reference source discovery before policy checks

Parent: #401 (kept open until #406, this child and Session successor #417 all land); grandparents #306/#349 TOOL-11. Depends on merged #400 and merged #406, so it extends the actual shared helper shape rather than predicting it. This is queued issue #407; implementation waits for its merged prerequisites.

## Smallest closable outcome and exact roster

Two existing gates reject failed/incomplete source discovery while retaining graph ownership and the independent conformance/reference boundary:

- `scripts/check-graph-policy.sh`
- `scripts/check-conformance-boundaries.sh`

No other gate, runtime/Rust source, benchmark, artifact, or manifest changes belong here. Astra invoked the pre-implementation half-day split: Session policy and its entire fixture contract now belong to #417, after this issue.

## Allowed paths

- the two gates above;
- two small direct hermetic suites: `scripts/test-graph-policy.sh` and `scripts/test-conformance-boundaries.sh` (use an existing exact filename if one is found after merge, never a generic mutation framework);
- `scripts/lib/gate.sh` and `scripts/test-gate-lib.sh`, only for minimal backward-compatible checked producer and dependency declaration modes required here;
- `.github/workflows/qualification.yml` only to add each new suite immediately after its existing checker (convert the scalar conformance run to a two-line block); preserve all job/router/trigger/expectation behavior and helper wiring;
- this child's numbered issue spec/evidence.

Do not use or edit `scripts/test-builtins-benchmark.sh` or the Session checker/suite. Session CLI/fixture behavior is wholly assigned to #417.

## Frozen dependency declaration modes and outputs

These modes differ from child A and #400 and must remain deliberate:

| gate / manifest | mode | frozen sorted output |
|---|---|---|
| graph / `crates/graph/Cargo.toml` | exact `[dependencies]` only; preserve actual selection regex `^[a-zA-Z0-9_-]+[.]workspace` (no equals requirement); return full `$1` with `.workspace` retained | `effect-contract.workspace`, `engine.workspace`, `lane.workspace`, `rack.workspace` |
| conformance / `crates/conformance/Cargo.toml` | exact `[dependencies]` plus every `[target.*.dependencies]`; full-line key extraction before `=` accepts compact declarations; strip `.workspace`; ignore dev/build/features | `dsp-reference`, `effect-contract`, `engine`, `lane` |
| conformance / `tools/bench/Cargo.toml` | same plain-plus-target mode; target dependencies are mandatory inputs to the union | `bench-support`, `builtins`, `builtins-compiler`, `conformance`, `console-workload`, `effect-compiler`, `effect-contract`, `effect-package`, `engine`, `flatbuffers`, `graph`, `graph-compiler`, `lane`, `protocol`, `rack`, `session`, `sha2` |

Directed fixtures cover compact-key acceptance for conformance, and preserve graph's original `$1` behavior including rejection by its exact-output gate of compact `name.workspace=true`; graph output keeps `.workspace`. Do not normalize graph keys under the other modes. Cover conformance bare/`.workspace` keys, target inclusion, dev/build exclusion, and independent extraction/sort failures including partial output. Do not replace these with “all dependency-like tables.”

## Per-gate frozen semantics

### Graph

Both graph manifests are required. Preserve the exact dependency output above, required compiler `sha2.workspace = true` positive query, and render-graph control-plane ban over `crates/graph/src` plus its manifest. The first discovery producer at lines 32–34 must successfully and non-vacuously enumerate Rust files from both `crates/graph/src` and `crates/graph-compiler/src` before comment-stripped concatenation and the publication/I/O/threading ban. The second producer at lines 44–52 must successfully and non-vacuously enumerate workspace Rust candidates before per-file comment stripping; its exact final production implementation set remains only `crates/graph/src/lib.rs`. Preserve regexes, test-module truncation, roots, CLI fixture root, trap behavior, and `graph policy failure:` diagnostics.

### Conformance

`crates/dsp-reference/Cargo.toml` is required and its exact existing `[dependencies]`-heading ban stays intact, including an otherwise-empty heading; do not reinterpret it as dependency counting. Workspace library discovery must successfully traverse all four required roots `crates hosts tools sidecars`; an individual root and an individual manifest may contribute no `[lib]` row (bin-only packages are legitimate), but the aggregate sorted unique library-name set must be nonempty.

Every named production crate (`engine session protocol capi target-smoke effect-contract effect-compiler effect-package lane math`) must resolve to one exact directory with a required manifest and readable source root; remove the current silent `[[ -f ]] || continue`. Preserve manifest harness bans, comment exemptions, and the local same-named `mod conformance`/`mod dsp_reference` exemption only after a successful checked module probe. An empty module match is valid; unreadable/error is not. The filtered harness-use scan may validly be empty only after successful source scanning and comment filtering.

Hosts and sidecars are both required roots, but each may cleanly contain no harness match. Their combined producer must distinguish no match from traversal error and partial matches plus error. The reference-use ban requires a successfully derived nonempty production-library pattern and readable `crates/dsp-reference/src`. Preserve the exact dependency modes/outputs above, production roster, allowlists, roots, and `conformance boundary failure:` prefix.

## Tests

Create two direct disposable fixture suites. Each has a clean positive control, existing policy violations, intended required-root/surface deletion, injected producer error with valid metadata, and partial-output-then-failure. Graph covers both discovery producers, per-file sed reads, exact parser quirk, compiler SHA presence and failed sort after useful output. Conformance covers bin-only manifest success, nonempty aggregate library discovery, all mandatory named crate manifests/source roots, local-module exemptions at their original probe scope, empty hosts/sidecars success, target dependencies, and source/extractor/filter/paste partial failures. Assertions discriminate the named error class. Counter-mutations must run actual acceptance assertions against faulty implementations and demonstrate rejection; constructing bad controls alone is insufficient. No Cargo/build or benchmark invocation is needed.

## Common acceptance

For each changed gate: real-tree positive check, all existing relevant violations, explicit required-root/required-surface deletion, clean optional-empty positive, injected producer error with otherwise-valid metadata, and failure AFTER valid partial output. Check producer status before filtering, counting or looping. Test direct/no-match/positive queries separately; filters may validly leave nothing. Error assertions require the intended class, explicit rejection of unexpected success and one counter-mutation per new helper failure mechanism. Preserve physical-script library sourcing, CLI defaults, diagnostics, caller shell state, exact roots/globs/allowlists and no runtime/source changes.

Final gates are affected shell suites, bash syntax, real policy scripts, existing workspace unchanged-count comparison and required CI. No benchmark, artifact regeneration or publication. Root checkpoints one coherent pass; Luna first attempt, Sol only following Astra FAIL (three total maximum), Astra actual PR review before merge. #401 closes only after #406, #407 and #417 and all seven gate/extractor obligations are upstream/closed; broad #306/TOOL-11 remain open for the rest of their program.

## Numbered program and approved assignment

Astra approved the exact graph/conformance scope after the pre-code Session split on 2026-09-05. Base is merged #406 commit `882277b65ff64780f57c4df33dee127abc6a33e2`, with its actual shared-helper API. #406 is verified CLOSED; #401 remains OPEN until this issue and #417 close, retaining all seven original gates and extractor obligations. Luna gets one coherent implementation pass, then Astra supplies a verdict before any Sol retry. Root owns checkpoints and synchronization.

## Luna attempt 1 source checkpoint — pending adversarial verdict

Luna implemented checked graph/conformance discovery and source consumers, narrow find/sort and plain-target helper extensions, two new focused suites and their required-CI wiring. Graph retains its workspace-prefix/$1 declaration behavior; Session is untouched. Root preserved focused evidence from the paused tree: both real gates, both new suites and existing helper suite exit 0 (`/tmp/engine-407-luna-real-graph.log`, `/tmp/engine-407-luna-real-conformance.log`, `/tmp/engine-407-luna-test-graph.log`, `/tmp/engine-407-luna-test-conformance.log`, `/tmp/engine-407-luna-test-helper.log`). Bash syntax and diff hygiene pass.

This is a recoverable source checkpoint, not acceptance. Astra must inspect the complete declared producer/partial-output/empty-result/parser/counter-mutation contract before any further implementation or full-workspace promotion. No Cargo, runtime, manifest, Session, artifact or benchmark work was performed.

## Astra attempt 1 verdict — FAIL; Sol attempt 2

# Astra #407 attempt 1 review

**FAIL at exact pushed `cb2dc9a354d713125c63d24f8d8b5f9b124759ca`. Luna's one attempt is consumed; route one coherent bounded revision to Sol.** The amended two-gate scope remains appropriate, but several assigned producers are unchanged and the promised direct fixture suites are mostly absent.

## Concrete failures

1. **Graph still accepts failed publication and executor predicates.** The temporary concatenation is scanned by the original unchecked `if rg ...`; each executor probe is also still an unchecked `if rg -q`. I independently made a minimal valid graph fixture containing a real `std::fs` violation and a shim failing only the publication predicate with status 7. The gate printed the error followed by `graph policy: PASS`, exit 0. `/tmp/astra-407-attempt1-reproduction.log` records it. A failed per-file executor probe can likewise omit an extra executor from the final owner set. Check both consumers, not only the enumerations, before interpreting absence. Keep complete capture before predicate analysis and preserve the exact sole-owner result.

2. **Both fixture-root CLIs resolve the library too late.** They cd to the supplied workspace before deriving script_directory from BASH_SOURCE. A standard relative invocation such as `bash scripts/check-graph-policy.sh /tmp/fixture` now tries to resolve `scripts` in that fixture. The independent reproduction exits 1 at `cd: scripts: No such file or directory`. Derive/source the physical library before changing cwd in both gates; retain existing CLI/defaults. The new suites use absolute script paths and miss this regression.

3. **Conformance source policy changes and unhandled predicates remain.** Its module exemption was explicitly frozen to the immediate `$crate_dir/src/*.rs` probe. Using recursive rg with `--glob '*.rs' "$crate_dir/src"` now lets a nested unrelated same-named module exempt top-level harness references throughout the crate. Restore the original top-level scope while checking enumeration/read status. The production manifest harness ban is still the old unchecked conditional rg. The reference heading is scanned once with a checked collector whose result is discarded, then again with unchecked rg; use the successful captured result directly so a second failed scan cannot masquerade as no heading. Preserve the actual heading ban, including an empty dependency table.

4. **Complete source resolution and producer classification are unfinished.** `workspace_crate_dir` returns the first directory instead of proving the assigned exact unique resolution. Required source roots should be directories with readable inputs, not merely an arbitrary readable path. Library aggregation still pipes checked sort into awk, and pattern construction uses an unclassified paste pipeline; graph's final executor sort is also outside the checked helper. These may fail under current pipefail, but do not satisfy the assigned explicit producer-status/error-class contract. Capture each externally produced result/status before consuming it, including valid partial output followed by error. Preserve valid bin-only manifests, empty individual discovery roots and nonempty aggregate names/pattern.

5. **New suites are not the specified acceptance fixtures.** Graph's suite runs the real tree, deletes all manifests together, tests the find helper directly, and tests the *rack default parser*, which is not graph's local retained-suffix parser. Conformance's suite runs the real tree, tests plain-target parsing and the same helper error only. Neither constructs a valid isolated policy tree with the named policy mutations, missing-source/read/filter/paste cases, intended empty populations or actual gate-level partial discovery. No new find/sort/parser counter-mutation executes the acceptance suite against a faulty implementation. Existing #406 controls do not qualify these new mechanisms.

## Bounded Sol revision

Complete the existing amended #407 contract in one pass; no Session/#417, runtime, manifests, artifacts, benchmark, new policy or generic harness. Keep the two CI suite calls and narrowly useful helpers. Keep graph's local exact regex/$1 output (including compact rejection) and conformance's plain-plus-target table union; preserve merged rack/plain helper modes.

Build the two SMALL standalone valid fixtures promised in the issue. Graph needs each required manifest/root, compiler SHA, actual graph parser, control/publication violations, sole executor plus a second executor, both discovery pipelines, sed failures and their predicate/sort failures. Conformance needs required reference heading behavior, the production roster's missing manifests/source roots, top-level local-module positive and nested-module nonexemption, clean empty hosts/sidecars, bin-only/no-lib manifest positive with nonempty aggregate, true harness/reference violation, target dependency union, and source/extractor/filter/aggregation errors. Use loops over equivalent roster cases rather than duplicating large fixtures. A real minimal fixture must pass first, then each red case must reach the named operation with unrelated metadata still valid.

For each materially distinct changed producer, inject both error-only and useful partial output followed by nonzero status. Include real gate-level find errors rather than only calling the helper. Assert diagnostic class and explicit unexpected-success refusal. Exercise new helper find/sort with caller pipefail on/off and conditional calls. Run actual assertions against disposable fail-open mutants and record the intended failing assertion; constructing a bad control is insufficient. Test relative checker invocation with foreign fixture cwd to pin library sourcing. Retain #406 tests and deliberate parser modes without unrelated widening.

Then run both real gates, both complete fixture suites and shared helper suite, syntax/diff; root checkpoints/pushes and Astra supplies one attempt-2 verdict before workspace/PR qualification. Sol has at most two attempts remaining; final attempt failure means stop/rescope. Pause all checks when root reserves the #415 timing window.

Review used source and two tiny disposable shell probes only. No Cargo, timing, Git, repository or GitHub mutation occurred.

The #415 measurement window is terminal and its complete capture is checkpointed. Root authorizes the bounded Sol attempt 2 above; no Session or other scope expansion.

## Sol attempt 2 source checkpoint — pending adversarial verdict

Sol completed the bounded graph/conformance revision after Astra's attempt-1 FAIL. Both gates now source the physical helper before changing directory; graph checks publication and per-source executor predicates before interpreting absence and checks the final owner sort; conformance reuses its checked reference-heading result, proves unique crate-directory resolution, requires readable immediate Rust sources, restores the top-level-only module probe, and checks manifest, unique-name and pattern-join producers. The shared additions are limited to checked text scanning, nonempty uniqueness filtering and line joining.

The two focused suites now build standalone valid policy trees and exercise the real checkers across required surfaces, graph parser behavior, both graph discovery populations, publication/executor predicates, sole ownership, conformance target/bin-only behavior, unique production resolution, top-level module exemption versus nested nonexemption, clean empty host/sidecar scans, harness/reference violations, and injected find/read/sort/extractor/filter/join errors with useful partial output. Shared helper tests cover the added producer status paths with caller pipefail modes.

Focused evidence is green: both real gates, both direct suites, the shared helper suite, Bash syntax, and diff hygiene. Logs are `/tmp/engine-407-sol-real-graph.log`, `/tmp/engine-407-sol-real-conformance.log`, `/tmp/engine-407-sol-test-graph.log`, `/tmp/engine-407-sol-test-conformance.log`, and `/tmp/engine-407-sol-test-helper.log`. No Cargo, timing, Session, runtime, manifest, artifact, benchmark, Git, GitHub, or workflow change was made. Astra must supply the attempt-2 verdict before any broader qualification.

## Astra attempt 2 verdict — FAIL; final Sol attempt 3

# Astra #407 attempt 2 review

**FAIL at exact pushed `3942e96c882ba482345e2d47ba1da20dcd2d75aa`: source blockers are resolved, but the explicitly frozen error-path/counter-mutation evidence remains incomplete. One final bounded Sol attempt remains.** Preserve this useful checkpoint; no full-workspace promotion yet.

## Resolved source findings

Both gates source the helper before changing cwd, and the new relative-invocation positives cover that regression. Graph now checks the publication predicate, each executor predicate and final owner sort. Conformance reuses one checked heading result, checks production manifest predicates, resolves unique crate directories, restores immediate-source module probes and explicitly checks library sorting/uniqueness/pattern joining. Exact graph workspace-prefix/$1 behavior, reference dependency-heading ban and conformance target union remain. The standalone fixtures are materially improved: they exercise actual gates, publication/executor violations, partial discovery, top-level versus nested module behavior, bin-only manifests, empty host/sidecar roots and reference violations. No additional source bug was established by this review.

## Evidence still does not satisfy the assigned contract

The conformance fixture's generic failing `rg` always stops at the INITIAL reference-heading scan. It never exercises production manifest, local-module, recursive harness-source or hosts/sidecars search execution failure. Its generic failing `find` stops at the FIRST engine immediate-source enumeration; it does not qualify the later workspace-library manifest population. Its generic `awk` stops at the first library-name extractor, not the target-dependency extraction or uniqueness-filter call. This distinction matters because these are separate conditionals/consumers, and the issue explicitly requires partial-output failures to reach each materially different producer with other metadata valid.

Concrete independent counter-proof: I copied the checker/helper/suite to a disposable directory and changed ONLY the local-module probe error arm from `|| exit $?` to `|| true`. The complete conformance suite still exited 0 and printed `conformance boundary fixtures: ok`. `/tmp/astra-407-attempt2-counter.log` records it. Thus the assigned module-probe failure rule is not enforced by the suite; the existing nested-module policy mutation cannot substitute for an execution-error test.

The required all-roster manifest/source deletions remain represented only by engine/session manifest and math source examples. Aggregate-empty library/pattern cases and the source-only empty/read-error boundaries remain missing. The new helper tests check find failure with pipefail settings by using a nonexistent path (their PATH directory contains an rg stub, not a find stub); they do not exercise partial find output under those invocation modes. There is no direct gate_sort_lines on/off/conditional error test. New-mechanism counter-mutation results are not recorded: the inherited #406 controls exercise only old functions and do not run the actual new acceptance assertions against changed implementations.

## Final Sol pass: tests and truthful evidence, no expansion

Keep the current gate implementation unless a directed case demonstrates a concrete defect. Complete only the existing two suites/shared helper suite and issue record:

- Target conformance's manifest, module, harness-source and hosts/sidecars producer errors separately; delegate unrelated invocations to the real tool. For the module case emit a plausible matching `mod` row then fail, proving it cannot establish an exemption. For later manifest discovery, delegate immediate-source find calls and fail only the workspace manifest population after valid manifest paths. Cover library extraction/uniqueness and target-dependency stages distinctly where their consumer differs, plus partial pattern join/filter failure. Assert each intended diagnostic, not a generic `scan errored` that can be satisfied earlier.
- Loop over the frozen mandatory roster for manifest/source deletion using the existing small fixture. Add aggregate no-library/no-production-pattern red controls while preserving bin-only and individually empty-root positives. Keep exact top-level module semantics and avoid adding new source-policy rules.
- Exercise gate_find_collect and gate_sort_lines directly with real plausible stdout then nonzero status, both pipefail settings and direct/conditional calls. Keep empty successful discovery valid at the helper and required/nonempty decisions in the callers. Preserve all merged parser-mode tests; add only the target-mode distinctions needed here.
- Actually run the focused acceptance assertions against disposable fail-open mutants for newly introduced mechanisms. At minimum the escaped module-consumer mutant above must now go red at its directed case; new find/sort/text/unique/join and target-mode mutations must be rejected at meaningful assertions. Record the concrete failing assertion and status. Constructing controls alone is insufficient; do not reuse the old printed-control label as proof.

Do not add a generic fixture/runner framework, more gates, Session/#417, Rust, manifest changes, artifacts or timing. A small operation table and loops inside these existing suites are enough; no repeated large fixture corpus. After one coherent pass, run both real gates, both direct suites, helper suite, syntax/diff, then root checkpoint/push and ONE final Astra verdict. Attempt 3 FAIL is a hard stop/rescope, not another correction round. Workspace and actual PR/CI follow only focused acceptance.

Review used source, completed logs and one small disposable shell counter-mutation. No Cargo, timing, Git, repository or GitHub mutation occurred.

## Sol attempt 3 final focused revision — pending final Astra verdict

Attempt 3 preserves the attempt-2 gate implementation and completes the missing directed acceptance evidence. The conformance suite now reaches manifest, top-level module, recursive harness-source, hosts/sidecars, workspace-manifest discovery, library extraction, uniqueness, harness/reference filters, pattern joining, and plain-target extraction/sort failures separately while delegating earlier operations to real tools. Error-only and useful-partial-then-error modes are exercised at these materially distinct producers. It loops over every frozen production crate for missing manifests, missing source roots, and present-but-empty source roots; retains the bin-only and individually empty discovery positives; and adds aggregate no-library and no-production-pattern controls.

The helper suite now exercises actual partial-output `gate_find_collect` and `gate_sort_lines` failures under pipefail on/off and direct/conditional invocation. Disposable fail-open or semantic mutants for module consumption, find, sort, text scanning, uniqueness, joining, and plain-target parsing are run through the real focused assertions; each is required to fail at its named assertion and records its nonzero status. Graph producer tests now distinguish error-only from plausible-partial failure for discovery, sed, sort, publication, and executor predicates.

Final focused evidence is captured in `/tmp/engine-407-sol3-real-graph.log`, `/tmp/engine-407-sol3-real-conformance.log`, `/tmp/engine-407-sol3-test-graph.log`, `/tmp/engine-407-sol3-test-conformance.log`, and `/tmp/engine-407-sol3-test-helper.log`. Bash syntax and diff hygiene are also required before checkpoint. No gate source, shared helper implementation, workflow, Session, Rust, manifest, artifact, benchmark, Cargo, timing, Git, or GitHub work belongs to this revision. Attempt 3 is the hard-stop attempt and requires Astra's final verdict before broader qualification.
