# Reuse numeric buffer-coloring scratch through the graph compiler workspace

GitHub: https://github.com/misofm/engine/issues/871

## Outcome and dependencies

Depends on [#870](https://github.com/misofm/engine/issues/870). Extend the accepted workspace to five fixed-length arrays in `schedule.rs::buffer_assignments`: `consumer_counts`, `last_consumers`, `main_input_counts`, `main_input_sources`, and `node_buffers`.

## Exact implementation slice

Own only `crates/graph-compiler/src/{workspace.rs,schedule.rs,compile.rs}` and focused graph-compiler tests. Supply initialized typed mutable slices to the existing coloring algorithm. Include all eight arrays from the two compiler issues in the same retained-byte total and post-call retention policy.

Keep the positions map, free set, `live_until`, nested expirations, output assignments and required owned output ID clones unchanged. Preserve duplicate-key behavior, last-consumer handling, output retention, identity alias eligibility, smallest-free-buffer selection, PDC-related consumers and stable reduction order. No graph representation rewrite, allocator dependency, render change or new public resource-report field belongs here.

## Objective gates

- Compare graph manifests against the predecessor and the pre-series semantic evidence.
- Exercise fanout, parallel edges, sidechains, identity boundaries, output retention and multiple buffer lifetimes using existing independent liveness/coloring oracles. Reusing the same new code twice is not the only oracle.
- Test shrink/grow/rejected-then-valid sequences and zero/under/exact/over retention caps over the combined eight-array owner. A failed compile must neither retain logical state nor invalidate a previous returned plan.
- Require unchanged canonical outputs/resource estimates, representative scalar/native PCM, and render allocation/free gates. Check graph-compiler tests, focused release tests, strict affected Clippy, formatting, policy and scalar/SIMD Wasm builds.

No timing or broad fixture expansion. This issue closes the eight-array product contract; nested scratch and owned outputs still allocate and are explicitly outside the performance claim.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
