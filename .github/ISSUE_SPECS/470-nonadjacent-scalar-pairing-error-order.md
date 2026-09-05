# Complete nonadjacent serialized scalar pairing with preserved execution-error order

# Astra #443 delivered scheduling preflight

**Decision: adjacency does NOT cover the full applicable serialized scalar population. Number a retained successor before assigning #443.** Read-only delivered source `aba905c0a5ae0bc747a65d1052ba76811fcee3c5` through `engine-456-plan`, and the complete queued #443 spec at `1d3f8d16`. Its appended root ruling is binding. No tests, builds, timing or repository/GitHub mutations were performed.

## Concrete population proof

`crates/graph-compiler/src/schedule.rs:9–65` assigns each node one plus its maximum predecessor level, sorts nodes within levels, and returns ascending levels. `crates/graph/src/program.rs:531–550` enforces that the execution schedule is the concatenation of those levels. This is live single-thread scheduling; it is not the removed worker scheduler.

`crates/builtins-compiler/src/lib.rs:1946–1959` takes the real scalar route when `BankWidth::for_backend(dispatch)` is None, or when no planned banks exist. `into_graph_artifact` then calls `strip_bindings` (`1831–1893`), creating separate actual ConsoleFaderProcessor and ConsoleMatrixProcessor boxes and moving their separate queue consumers. The list's per-track insertion order does not override the graph's level-major execution order.

A concrete source-derived counterexample is two independent, same-depth tracks A and B, both lowered with Backend::Scalar, each with its ordinary unobserved sole-reader PostFader→PostMatrix edge, no sends/delays or other structural incompatibility, and BetweenRenderCalls ownership. At those two levels the real schedule is `F_A, F_B, M_A, M_B` in track/node-ID order. Neither matching pair is adjacent. Both have the same valid scalar dataflow and delivery population targeted by #443; no observer or Concurrent exemption explains their exclusion. Extra zero-work stages between earlier strip stages do not alter this final-level argument. This is a constructive source proof, not an executed fixture claim.

Runtime `units_of` and `build_sequential` (`runtime.rs:1782,1828–1861`) preserve plain scalar units; cohort runs merge bank membership, not these scalar owners. `finish_unit` at2791 returns a plain RuntimeUnit::Op for empty membership. Delivered #430's bank pairing therefore does not collapse the scalar counterexample or repair its scheduling. Current scalar lowering still lacks propagation of the immutable delivery declaration into its two concrete owners; #443 retains that approved task.

The existing proposed pre-build_op adjacent insertion remains appropriate for its bounded slice. It cannot be described as complete serialized scalar delivery. `execute_op` still invokes bound processors at their existing positions and propagates their errors. Eagerly moving M_A to F_A crosses F_B: if F_B fails, old M_A has not consumed its queue or changed state, whereas an eager pair has. Conversely delaying F_A until M_A would change already-completed fader state/queue effects before F_B fails. BetweenRenderCalls freezes producer admission; it does not make failures or state effects commute.

## Required numbered successor body

Suggested title: **Complete nonadjacent serialized scalar pairing with preserved execution-error order**.

Parent #443; retained audit #349 RT-4 population. Depends on delivered #443 adjacent scalar ownership/bridge and its actual-base review. Concurrent scalar ownership remains #444; measurement remains #431. No implementation authority until the finite preparation decision below is resolved and Astra approves an amended executable scope.

Retained product obligation: cover otherwise compatible, unobserved, nonadjacent serialized scalar fader/matrix pairs produced by the existing level-major schedule. Do not close this obligation by renaming all scalar pairing as adjacent, treating BetweenRenderCalls as infallibility, or pointing to bank-tail execution. Preserve original schedule, reductions, exact arithmetic, both queue ownership/drain effects, first error and state at every original execution boundary. Observed, delayed, fan-out, noncompatible and Concurrent paths retain original separate execution.

Smallest first closable slice is a bounded scheduling/error-order design decision on the existing two-track scalar fixture, not an unbriefed general optimizer. Inspect the actual owned scalar stages and intervening operation kinds after #443 delivery. Freeze either (a) a concrete preparation-only eligibility proof and execution mechanism that preserves those boundaries and earns a useful nonadjacent population, with every excluded compatible population explicitly retained; or (b) a precise impossibility/architecture decision explaining which original side effect prevents fusion and what owner decision is needed. A design decision alone does not close the retained product obligation or audit row. Do not authorize speculative public fallibility flags, shared mutable consumers, rollback of arbitrary processors, execution reordering or a new scheduler inside this brief. If a new architecture is required, amend/split before code rather than allowing Luna to choose one mid-attempt.

Finite decision evidence: use the existing compiler/graph fixture helpers to spell out the two-track schedule and identities; enumerate the actual fader and matrix queue/setter failure points and intervening bound-processor return. Provide a before/after trace for successful settled execution, an intervening error, fader error and later matrix error, including queued prefix and state effects. Existing APIs and source references suffice for the initial decision; no builds, corpus or benchmark are required merely to number the retained work.

Before eventual product implementation, freeze a small actual nonbanked two-track prepared fixture and old separate reference. It must prove scheduled nonadjacency, actual selected mechanism, exact PCM and per-boundary state/queue/error order, plus observed/nonunity-send and Concurrent declines using the existing compact fixture conventions. Reuse #443's accepted arithmetic, resource and live allocation proof mechanisms rather than repeat its entire corpus. One actual selection-to-separate SAME-assertion control is sufficient for this new dispatch; exact commands/allowed files must be frozen against delivered #443 before assignment. No additional measurement invocation is implied.

## Reciprocal accounting and #443 assignment

Before #443 assignment, create/synchronize this successor and add its number to #443, #349 and the RT-4 retention record. #443 may then deliver its explicitly adjacent serialized product with all original finite correctness, host, resource, allocation and qualification gates, while this successor remains open for nonadjacent serialized coverage and #444 for Concurrent coverage. Neither child nor parent wording may claim all scalar integration is finished. Root should preserve the existing public Any/static decision and approved identity substitution; this finding does not reopen those decisions.

This is the smallest honest split: the adjacent capability is independently useful, while nonadjacent error ordering is a separate unresolved architecture boundary. It should not be hidden inside a half-day implementation attempt or waived as a performance-only detail. #460 remains the only active feature; no #443 implementation is authorized by this report.


## Numbered retained obligation

This is #470. #443 retains adjacent serialized scalar integration; #470 retains otherwise compatible nonadjacent serialized scalar scheduling/error-order completion; #444 retains Concurrent admission and scalar/bank rollout. #431 retains measurement. Numbering supplies no implementation or timing authority. The actual delivered #443 base and explicit design decision remain prerequisites.
