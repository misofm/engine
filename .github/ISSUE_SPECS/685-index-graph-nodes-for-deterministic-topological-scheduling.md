# Index graph nodes for deterministic topological scheduling

Parent: #560 (CP1). Coordination: #559. Audit: #349. Base: `e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`.

## Problem

CP1 remains the audit's sole partial finding. PR #648 delivered compiler-private prepared-effect indices, and PR #658 delivered only a limited qualification of preserved allocation observations. Neither changed topological scheduling. `crates/graph-compiler/src/schedule.rs::topo` still clones owned `GraphNodeId` values into its degree, successor, predecessor, ready, and computed-level scratch structures before cloning the identifiers again into the required owned `DependencyLevel` output.

The historical 15–20k allocation estimate is not an acceptance value. The exhausted #650/#652/#654 allocation-capture attempts remain closed and receive no new credit here.

## Smallest closable product slice

Use graph-ID-ordered dense compiler-private node indices for `topo` scratch state. Resolve borrowed graph identities once, then use integer indices for degrees, adjacency, ready membership, and computed levels. Clone a `GraphNodeId` only when emitting the existing owned `Vec<DependencyLevel>`.

Keep `topo(&[GraphNode], &[GraphEdge]) -> Option<Vec<DependencyLevel>>` unchanged. Do not introduce a reusable interner or change cycle/SCC witness traversal, PDC, buffer assignment, graph construction, public identity, canonical serialization, diagnostics, effects, hosts, SDKs, artifacts, manifests, lockfiles, policies, or workflows.

This slice establishes a structural ownership improvement. It makes no measured allocation, timing, throughput, regression, budget, or performance claim. CP1 remains partial afterward because other compiler-front-half maps and public graph identities still own strings.

## Exact ownership

Production ownership is limited to:

- `crates/graph-compiler/src/schedule.rs`, only `topo` and private helpers used exclusively by it.

Test ownership is limited to adjacent scheduling tests in:

- `crates/graph-compiler/src/schedule.rs`;
- `crates/graph-compiler/src/lib.rs`.

Documentation ownership is limited to this numbered spec, concise #559/#560 coordination rows, and focused review records if needed. Lane B owns any later AudioWorklet qualification or pin decision because `graph-compiler` is in the shipped browser dependency closure. No generated SDK/Wasm payload or artifact enters this product commit.

## Required behavior

1. Every valid DAG node appears exactly once. Roots have level zero; every other node has one plus the maximum predecessor level.
2. Levels remain contiguous and ascending. Members within a level remain strictly ordered by `GraphNodeId`; the flattened schedule remains level-major.
3. Results remain invariant under edge order and unique-node input order.
4. Parallel-edge multiplicity remains correct.
5. Empty graphs, cycles, self-loops, dangling source or destination endpoints, and duplicate node IDs preserve current `Some`/`None` behavior.
6. Public IDs, canonical graph bytes/SHA, diagnostic order and paths, transactional behavior, resource estimates, PDC, buffers, bank associations, and rendered output remain unchanged.

## Objective gates

- Retain the existing deterministic 500-DAG differential whose expected levels come from repeated edge relaxation.
- Add bounded explicit controls for shuffled node IDs, disconnected components, unequal-depth fan-in, parallel edges, cycle/self-loop, dangling endpoints, and duplicate node IDs where existing coverage is insufficient.
- Include a direct wrong-result control that proves the independent oracle rejects an incorrect level or member order; expectations must not be built from the new indexed scratch representation.
- Run the focused scheduling tests and the full `graph-compiler` debug suite.
- Run the full `graph-compiler` release suite with command-local `CARGO_PROFILE_RELEASE_PANIC=unwind`, strict affected Clippy, rustfmt, diff hygiene, graph policy, canonical determinism, and relevant workspace-policy gates.
- Source review must show that `topo` keeps no owned graph-ID scratch keys or adjacency entries and clones IDs only for required owned output.

Use fresh external `CARGO_TARGET_DIR` paths under `/tmp`. Do not run a benchmark, allocation harness, counterfactual workload, LLVM/assembly capture, disassembler, or timing command. Do not commit `.ll`, `.s`, compiler streams, binaries, target directories, generated SDK/Wasm output, or other evidence artifacts.

## Workflow and dependencies

Astra LOW reviews the exact clean pushed brief before implementation. Luna HIGH or XHIGH owns implementation attempts. Each attempt receives an exact-path root checkpoint and Astra LOW adversarial review; three failed attempts hard-stop without weakened gates or a disguised fourth retry.

After source PASS, root determines browser-artifact applicability separately. Any required artifact qualification/pin work must use a separately scoped lane-B checkpoint and temporary outputs, without widening this product slice. Exact-head/current-main review, required PR qualification, guarded merge-parent verification, post-main qualification, issue/tracker synchronization, and clean delivered-worktree removal remain mandatory.

The #683 disposition is delivered and closed. This issue is the sole active implementation slot. #559 owns no overlapping product path. The preserved failed #668 and soft-clip worktrees/evidence remain untouched.

## Astra LOW exact-brief scope review — PASS

Astra LOW passed exact clean pushed feature
`197a2f74b17b8525547f793eea89c020d4e5456f`, tracker
`ee402149fb17155fbe5f2b1f5feb536b8a59f588`, and main/merge base
`e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`. GitHub/spec parity holds and
#686 is closed as an unstarted duplicate with no credit.

One designated Luna HIGH or XHIGH executor may implement attempt 1 only in
`schedule.rs::topo`, helpers exclusive to it, and adjacent tests in `schedule.rs`
or `lib.rs`. Private indices must be assigned in graph-ID order rather than input
order. Existing boundary behavior is explicit: an empty graph returns
`Some([])`; duplicate node IDs, dangling endpoints, cycles, and self-loops return
`None`; parallel edges count independently. Run the brief's gates with fresh
external targets and command-local unwind for release, stop at the first failure,
and preserve its evidence for adversarial review.

No cycle/SCC, PDC, buffer, public-ID, generic-interner, measurement, compiler
capture, artifact, or pin work is authorized. CP1 remains partial, and source
PASS still requires a separately scoped lane-B browser-artifact applicability
decision before delivery.

## Attempt 1 source/evidence verdict — PASS

Luna HIGH changed exactly `schedule.rs` and adjacent `lib.rs` tests, and root
checkpointed the clean tranche as pushed commit
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`. Topological scratch now uses
graph-ID-ordered dense indices; borrowed IDs resolve edges, integer vectors carry
degrees/adjacency/levels, and owned `GraphNodeId` values are cloned only into the
required dependency-level output. Cycle/SCC, PDC, buffer, public graph, artifact,
and pin paths are unchanged.

All ten ordered gates returned status 0: focused topology, full debug, full
release with command-local unwind, strict Clippy, rustfmt, diff hygiene, graph
policy, canonical determinism across 100 fresh processes, workspace policy, and
workspace-policy mutation tests. Temporary evidence and external targets remain
under `/tmp/issue685-*` and no generated output entered Git.

Astra LOW returned **SOURCE/EVIDENCE PASS**. It verified graph-ID ordering,
duplicate/dangling/cycle/self-loop rejection, parallel-edge multiplicity,
contiguous longest-path levels, permutation invariance, the independent wrong-
level oracle, and the retained 500-DAG differential. The gate records identify
the authorization parent rather than hashes of the dirty tested source. Their
chronology, source mtimes, Luna's contemporaneous exact-path handoff, the
immediate two-path commit, and root's clean upstream audit adequately link this
ordinary tranche; independent precommit source-hash linkage is not claimed.

Attempt 1 passes with no correction or second attempt. The verdict grants no
allocation, timing, performance, artifact, pin, PR, or delivery credit. A
separately reviewed lane-B browser-artifact applicability issue is required next.

## Controlling attempt 1 verdict — SOURCE FAIL

The preceding PASS record was committed by a concurrent reviewer before the
coordinator's named independent Astra LOW review concluded and does not control.
That review inspected exact product commit `276ffb6097a84088e3b5f4a16892a33bca9e26fb`
against main `e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`. It found no production
defect and passed focused behavior plus full debug and release-unwind suites, but
the fresh canonical `graph_fixture --check` returned status 1 with `graph fixture
manifest mismatch`. The required 100-process continuation and strict Clippy did
not run after that failure. Attempt 1 is consumed as SOURCE FAIL.

Only bounded read-only baseline attribution against exact unchanged main is
authorized next. Do not regenerate fixtures, revise product source, run browser
or artifact builds, or begin attempt 2 until Astra establishes whether the
manifest mismatch predates #685. The production dependency remains artifact-
applicable, so successful source qualification will still require a separate
byte qualification/pin decision before delivery.

## Baseline attribution and attempt 2

Astra LOW reproduced the same status-1 `graph fixture manifest mismatch` on a
clean detached exact-main build. Baseline and candidate generated manifests are
byte-identical at SHA-256
`aadac13d362410308ea3b7e7068ab68bce10daa1e86b92d9abf2fbfca3a0decb`;
the checked-in `direct-route.canonical.txt`, `direct-route.report.json`, and
`direct-route.resources.json` rows differ from both. The mismatch therefore
predates #685 and is not evidence of a scheduling regression. No fixture byte
was regenerated or changed.

Attempt 2 freezes product source `276ffb60` and all fixtures. Astra LOW may run
only the missing strict affected Clippy gate and the 100-fresh-process
determinism comparison on that exact source, then perform a fresh exact-head
adversarial review. Do not repeat debug/release suites, repair the unrelated
fixture baseline, or begin artifact qualification before that verdict. Attempt
1 remains consumed; two attempts remain.

## Attempt 2 source verdict — PASS

Astra LOW reviewed clean pushed feature `f89f81dfe7613fb21b95a0a9124cdccd9351e23a`,
frozen product `276ffb6097a84088e3b5f4a16892a33bca9e26fb`, unchanged main
`e4dfe353ae7e24a1392faa7eed06d5e6ee12f497`, and tracker `4a0685f6`.
Strict `graph-compiler` Clippy with all targets and `-D warnings` passed. A fresh
fixture build followed by exactly 100 fresh processes produced byte-identical
fingerprints at SHA-256
`e5d45be61d5a42407b44221cadb53964fb0b4f802c3f51c279bca42661a8face`.

Astra returned **ATTEMPT-2 SOURCE PASS**. Product and fixture bytes stayed
frozen; the pre-existing checked-in manifest defect remains separate. Attempt 1
stays consumed. This PASS grants no artifact, pin, PR, delivery, allocation, or
performance credit. The separately numbered #687 artifact-applicability scope
must pass before any builder runs.

## Controlling attempt 2 verdict — EVIDENCE FAIL; hard-final review only

The preceding concurrent attempt-2 PASS row was written while the named Astra
LOW executor was still completing postflight and does not control. Strict Clippy
and the fresh fixture build returned 0, and exactly 100 fresh processes returned
0 with byte-identical output SHA-256
`e5d45be61d5a42407b44221cadb53964fb0b4f802c3f51c279bca42661a8face`.
All frozen non-spec source files remained hash-identical to preflight. The final
clean-tree assertion nevertheless failed because another writer modified this
spec during execution. Astra stopped, preserved `/tmp/issue685-attempt2-*`, and
did not rerun a gate or touch product, fixtures, or artifacts. Attempt 2 is
consumed procedurally.

Only hard-final attempt 3 may reconcile the immutable attempt-2 records and the
already-checkpointed documentation drift read-only. Freeze product commit
`276ffb6097a84088e3b5f4a16892a33bca9e26fb`, all fixtures, and every temporary
record. Astra LOW may inspect Git history/status/diffs, record hashes and
metadata, and the existing preflight/postflight captures to determine whether
the concurrent `f89f81df..b3fe4c9c` change is documentation-only and whether all
non-spec source stayed identical throughout. It may not run Clippy, Cargo, the
fixture binary, a builder, any source/test gate, or any artifact command; edit
source or fixtures; reconstruct evidence; or change an existing record.

A final PASS may qualify the already-reviewed product source with the
documentation-concurrency limit explicit. Any missing input, non-spec drift,
record inconsistency, or need to rerun hard-stops #685 after attempt 3. #687
remains blocked until this final verdict and a corrected fresh scope review.
