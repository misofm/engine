# Compare pooled compiler scratch through a tooling-only allocator adapter

GitHub: https://github.com/misofm/engine/issues/874

## Outcome and dependencies

Depends on [#871](https://github.com/misofm/engine/issues/871). Implement a descriptive allocator candidate for exactly the same eight numeric scratch arrays, keeping Rust 1.97.1 and the production workspace implementation intact.

## Exact implementation slice

Own graph-compiler's workspace/test-support modules and manifest; `tools/bench/Cargo.toml` for explicit opt-in; the corresponding lockfile dependency edges; and only the narrow feature-isolation policy check needed to prove the candidate cannot enter shipped hosts. Reuse locked `bumpalo = 3.20.3` and `allocator-api2 = 0.2.21`. Verify their exact resolved compatibility before edits. Neither library becomes an enabled default production dependency. All-features checks must still compile on the pinned stable toolchain; no nightly feature or toolchain upgrade.

Supply the existing typed borrowed-slice algorithm views using four named storage policies: fresh std Vecs; retained std Vecs; fresh allocator-api2 Vecs with Global (collection control); and allocator-api2 Vecs backed by a reused Bump, with reset between compilations. Keep phase order, initialization, element types, lengths, input data and final outputs identical. Select a policy at a phase boundary, not inside a per-node loop. Use a small private/test-support adapter with a feature-gated experimental workspace constructor, not a public generic allocator framework or copied topology/coloring implementation. The normal host workspace entry accepts that owner unchanged; host production code needs no pool-specific entry. If the shared seam cannot be kept within these files, stop and rebrief instead of cloning the algorithm.

Retain only numeric scratch in the pool. End all borrowed slice/vector lifetimes before reset. Returned graphs and DSP/source owners remain independently allocated. Apply the same explicit post-call retention cap using actual retained backing bytes; count chunk metadata/padding and report occupancy separately. A Bump allocation limit is not itself the logical retention policy. Reset may retain a chunk, so measure the retained backing and release/recreate the pool when the cap is exceeded.

## Objective gates

- Compare all four policies with pre-series semantic manifests, existing independent graph tests, and repeated/grow/shrink/failure-then-success cases.
- Prove returned artifacts remain valid after reset/drop and that production host dependency graphs resolve without the experimental feature and optional dependencies enabled. Keep dependency/feature isolation gates effective under Cargo feature unification.
- Test cap enforcement and backing-versus-occupancy accounting with known requests. Use safe upstream collection APIs; no new custom unsafe allocator.
- Run focused tests for default and experimental features, affected strict Clippy, graph/bench/workspace/feature-isolation policy and Wasm compilation for the production path. The separately scoped [#875](https://github.com/misofm/engine/issues/875) and [#876](https://github.com/misofm/engine/issues/876) own latency, allocation and active/replacement memory measurements. No timed benchmark or claim of equivalence to the future std 1.100 allocator implementation.

Sources: https://docs.rs/bumpalo/3.20.3/bumpalo/ and https://docs.rs/allocator-api2/0.2.21/allocator_api2/. This is a storage-policy experiment using a stable compatibility API.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
