# Make generic cancellation race proof deterministic

Bounded successor to hard-stopped #571 under parent #444 and audit lane A #559. #571 reached its three-attempt limit at reviewed source head `d2e16082852daccc8181c87d18f6126400c6d66f`. Astra LOW accepted the production generic cancellation source and every other gate, but found the final threaded pre-ack assertion scheduler-dependent and capable of stranding a scoped thread. This successor owns only that proof correction and delivery of the preserved source.

## Frozen input

Start from #571 branch checkpoint `1422e867`, which preserves the accepted product source, all three attempt records, and the hard-stop ruling. The product implementation and public API are frozen. Do not repeat, relabel, or erase #571's three attempts.

The defective schedule in `crates/protocol/tests/delivery_ownership.rs` releases render through `request_visible` and only then asserts `poll_cancel_boundary(token) == None`. A legal scheduler may let render run `cancel_boundary` and publish its acknowledgement first. The control assertion then fails even though production behavior is correct. Its later barrier can also leave render waiting after a control-side assertion failure.

## Product slice

Replace that one schedule with deterministic, failure-safe rendezvous:

1. Render establishes the requested zero, partial, or full application state and signals readiness.
2. Control calls `begin_cancel` while render is held before `cancel_boundary`.
3. Control asserts the pre-ack poll returns `None` before releasing render.
4. Render performs `cancel_boundary`, returns its result, and signals completion.
5. Control polls the acknowledgement and retains every existing frontier, disposition, prefix, remainder, sample, partial-collection publication-refusal, final collection, and reuse assertion.

Use ownership-safe deterministic primitives with failure-safe exit. A dropped control signal or assertion failure must let the render worker return an error and join; no barrier may wait forever when its peer exits early. Do not use sleeps, deadlines, probabilistic stress, or scheduler assumptions.

## Exact ownership

Allowed implementation paths:

- `crates/protocol/tests/delivery_ownership.rs`
- this numbered issue spec
- focused successor evidence and the #571 closure record under `docs/audits/`
- #571's local spec only for final delivered linkage after PASS

Do not edit `crates/protocol/src/delivery.rs`, `crates/protocol/src/lib.rs`, Cargo manifests/lockfile, controller, queue, SPSC, builtins, graph, engine, hosts, C ABI, artifacts, workflows, or dependencies. If review reproduces a production defect, stop and rebrief instead of widening this successor.

## Objective gates

- The corrected test deterministically covers zero, partial, and full application with separately owned generic control/render endpoints and two tickets.
- The pre-ack `None` assertion executes while render is provably unable to acknowledge.
- After release, each case verifies exact captured frontier, acknowledged sample, applied/canceled disposition, applied prefix, remainder, cancellation sample rules, publication refusal after one incomplete collection, and final slot reuse.
- Failure-safe rendezvous has no unmatched blocking wait after a peer error. A focused test mutation that restores the old release-before-poll ordering, or otherwise removes the hold, must fail the schedule's ordering discriminator without hanging; restore it afterward and preserve the result.
- Rerun #571's unchanged full protocol tests with `test-support`, strict Clippy, formatting, diff, workspace policy, and CI routing/classifier checks. Confirm the branch contains no runtime-source delta after `d2e16082` and no out-of-scope file.

## Attempt 1 evidence

The faulty barrier schedule was replaced with failure-safe bounded channel rendezvous.
Render is held before `cancel_boundary`, control proves pre-ack `None`, and render is
released only afterward. Zero, partial, and full application retain all exact
frontier, disposition, prefix/remainder, sample, collection, and reuse assertions.
The release-before-poll mutation failed deterministically without hanging; its exact
output is preserved in `docs/audits/572-deterministic-cancellation-proof.md`.

## Workflow and completion

Luna HIGH implements attempt 1. Astra LOW performs adversarial exact-head review. Retain at most three successor attempts, though this slice should close in one. Root checkpoints and pushes each coherent tranche, keeps both issues and #559/#560 synchronized, and opens a PR only after source PASS.

On PASS and required CI, merge the complete preserved #571 source plus this successor correction. Close this successor and #571 in the same synchronized workflow, record that #571 delivery was earned by the successor after its hard stop, update #444/#559, verify post-main qualification, and remove both clean delivered worktrees. This successor closes only generic boundary cancellation; #444's typed builtin endpoint and concurrent bank/scalar pairing remain open.
