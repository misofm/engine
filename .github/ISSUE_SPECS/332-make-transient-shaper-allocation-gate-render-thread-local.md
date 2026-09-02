# Make the transient-shaper allocation gate render-thread-local

## Objective

Make the transient-shaper realtime allocation gate measure only the thread executing the render
workload. Eliminate process-global test-runner allocations from the armed window without weakening
the zero-allocation/zero-free assertion, scalar-plus-bank workload, or existing production red
mutation. Produce a trustworthy passing `engine qualification` aggregate so path-aware branch
protection can be completed under issue #331.

## Current evidence

Engine qualification run <https://github.com/misofm/engine/actions/runs/33651977993> built and passed
all x86 scalar/SIMD, Wasm, cross-target, and policy work, but its host job failed only
`transient-shaper --test allocation`: the render loop reported 4 allocations instead of 0.
The exact same release test binary name/hash and unchanged production/test/Cargo.lock blobs passed
twice on the same Ubuntu image and Rust toolchain in runs
<https://github.com/misofm/engine/actions/runs/33634215401> and
<https://github.com/misofm/engine/actions/runs/33625893647>.

The harness currently arms process-global `AtomicBool`/`AtomicU64` state around a test running
under Rust's parallel libtest process. Any allocation performed concurrently by the libtest parent
or another test thread increments the same counters. The production render code is not implicated
by the varying count, and sibling effect allocation gates already use const-initialized
thread-local counters specifically to prevent cross-test contamination.

## Decision

Change only the allocation test harness. Use const-initialized thread-local `Cell` state for armed,
allocation, and deallocation counts. Every global-allocator hook consults the current thread with
`try_with`, never panics during thread teardown, and forwards every allocation request unchanged to
`System`. The measurement helper resets both counts, arms only the caller, runs the supplied body,
disarms it, and returns both counts.

Add discriminating harness tests:

- a same-thread allocation and free inside the measured body must be observed; and
- a helper thread started before measurement must allocate and free while the caller is armed, yet
  contribute zero to the caller's counts.

Coordinate the foreign-thread test with preallocated atomic state and a prestarted thread; perform
thread creation/join and other setup outside the measured region. Preserve the production gate's
1,000 blocks at quantum 128, per-block automation schedule, scalar effect, available native bank,
and separate zero-allocation and zero-deallocation assertions. Re-run the recorded
`Vec::with_capacity(1)` production mutation in an isolated copy and update its evidence, but do not
change production code in the committed result.

## Scope

- `crates/transient-shaper/tests/allocation.rs`;
- `crates/transient-shaper/tests/MUTATIONS.md` only for the refreshed allocation-harness mutation
  evidence; and
- this issue specification.

No transient-shaper production code, other crate/test, workflow, router, manifest, lockfile,
benchmark, generated file, SDK file, digest, or branch-protection change belongs to this slice.

## Objective gates

1. The allocator's armed state and both counters are const-initialized thread-local `Cell`s; no
   process-global armed/counter atomic remains.
2. All alloc, alloc-zeroed, realloc, and dealloc hooks use teardown-safe thread-local access and
   forward the original pointer/layout/size unchanged.
3. A same-thread self-test proves allocation and deallocation are counted, so a permanently
   disarmed or vacuous harness cannot pass.
4. A prestarted foreign thread allocates and frees while the caller is armed, and the caller's
   result remains exactly `(0, 0)`; the test proves overlap rather than merely running sequentially.
5. The production gate preserves the exact 1,000-by-128 scalar and native-bank workload,
   automation cadence, and independent zero-allocation/zero-free assertions.
6. The existing `Vec::with_capacity(1)` inside `Shaper::process_block` mutation is red under the
   revised harness and the clean tree is green.
7. The focused allocation binary passes repeatedly under ordinary parallel libtest execution and
   under explicit test-thread concurrency; the complete transient-shaper test target passes.
8. The exact scope passes Rust formatting, Clippy/compile policy relevant to the test, workspace
   policy, environment vocabulary, routing checker/mutations, and `git diff --check`.
9. Fresh Sol/high adversarial review of the exact implementation checkpoint returns PASS before
   push, with no claim that historical green runs alone prove current correctness.
10. The reviewed full-route main push produces passing `engine qualification`; all other workflow
    results are recorded without retrying or weakening a gate.
11. Issue #331 observes passing engine, browser, and SDK aggregates plus selected release work
    before branch protection is migrated to the exact three aggregate checks.
12. Local and GitHub issue #332 evidence/state are synchronized upstream before closure.

## Non-goals

- Changing transient-shaper DSP, parameters, automation, banking, allocation behavior, or its
  realtime contract;
- serializing the whole test binary, disabling libtest concurrency, accepting a nonzero budget,
  filtering known allocations, retrying flaky counts, or weakening/removing the production red
  mutation;
- refactoring other allocation harnesses or adopting a shared harness in this slice;
- changing CI workflows, routing, required checks, SDK behavior, or package contents; or
- running descriptive benchmarks or optimization work.

## Rollout order

1. Create this matching local spec and GitHub issue before implementation.
2. Obtain Sol/high approval of the bounded brief.
3. Implement once with Sol medium and attach focused clean/red evidence.
4. Commit the coherent tranche and obtain fresh Sol/high adversarial PASS.
5. Re-read remote main and prove the proposed range routes `full` without unrelated changes.
6. Push the reviewed batch once and observe every resulting workflow without retries.
7. Record the engine aggregate and synchronize/close this issue only after its evidence is upstream.
8. Return to issue #331 for aggregate observation and atomic branch-protection migration.

## Evidence

Drafted on 2026-09-03 from the failed and passing remote runs plus direct harness inspection.
Sol/high approved the brief at `df96edf1`: remote binary/source identity corroborates the harness
race; const thread-local `Cell` state and teardown-safe hooks are appropriate; the positive and
foreign-thread controls are discriminating; the exact workload and red mutation are frozen; and
the scope/rollout are the smallest closable slice. Sol-medium implementation, red mutation,
adversarial verdict, and remote qualification will be appended without weakening the gates above.

### Sol-medium implementation evidence

From clean checkpoint `8b9ed955`, the allocation harness now uses const-initialized thread-local
`Cell` state for its armed flag and separate allocation/deallocation counters. Every allocator hook
uses teardown-safe `try_with` observation and forwards the original allocation arguments unchanged
to `System`; successful reallocations count both the replacement allocation and release of the old
allocation. A shared measurement helper resets both counters, arms only its caller, runs the body,
disarms, and returns both values.

The same-thread positive control performs one explicit non-zero allocation and matching free and
observes exactly `(1, 1)`. The foreign-thread control preallocates all coordination atomics and
starts its scoped worker before measurement. Release/acquire handshakes prove that the worker's own
measured `(1, 1)` allocation/free occurs while the caller remains armed; the caller observes
exactly `(0, 0)`. Thread creation and join remain outside the caller's measured region.

The production gate retains its exact 1,000 blocks at quantum 128, every-fourth-block automation,
scalar processor, available native bank, and separate zero-allocation and zero-deallocation
assertions. Clean focused runs passed with default libtest scheduling, `--test-threads=1`, and two
successive `--test-threads=8` runs. The complete transient-shaper target and
`cargo clippy --locked -p transient-shaper --tests -- -D warnings` passed.

The frozen `Vec::<u8>::with_capacity(1)` mutation was applied only in an isolated source-tree copy
carrying the revised test. It failed the render gate with 2,000 observed allocations versus zero
while both harness controls passed (2 passed / 1 failed). Removing it restored the isolated
production file byte-for-byte and returned the allocation binary to 3/3 green. Production code in
this worktree was never modified.

Formatting, realtime policy, environment vocabulary, path-routing checker/mutations, and
`git diff --check` passed. The workspace-policy checker returned success on macOS but emitted its
existing unsupported BSD `find -printf` diagnostic; its mutation script also cannot complete under
native BSD `sed` (and a narrow `-i` compatibility shim does not cover its GNU `0,addr` extension).
No portability claim is made for those local mutation harnesses. The exact scoped paths classify
`full`, as required for a crate test change.

Only `crates/transient-shaper/tests/allocation.rs`, its allocation row/evidence in
`tests/MUTATIONS.md`, and this specification changed. This evidence does not claim fresh Sol/high
review, remote engine qualification, GitHub synchronization, or issue closure.
