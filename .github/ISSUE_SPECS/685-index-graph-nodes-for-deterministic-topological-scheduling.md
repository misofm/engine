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
