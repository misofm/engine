# Measure outstanding requested heap bytes in benchmark support

GitHub: https://github.com/misofm/engine/issues/873

## Outcome and dependencies

Independent tooling issue; may be implemented alongside [#870](https://github.com/misofm/engine/issues/870) in an isolated worktree. Add explicit benchmark measurement modes to `tools/bench-support`: a direct-System timing mode and an audited mode with live/high-water requested-heap-byte accounting. Preserve default allocation counter semantics and realtime violation behavior.

## Exact implementation slice

Own `tools/bench-support/Cargo.toml`, `src/alloc.rs`, and focused tests in that module. Use explicit nondefault measurement features and the single existing global allocator registration. The timing executable selects System directly, without existing event counters, TLS/atomic accounting or new byte hooks: otherwise the pool benefits partly by bypassing instrumentation. Its records must say allocation auditing is unavailable, and this executable must not run rendering or correctness gates; those run in the separately audited executable. The separate correctness/memory executable retains the audited allocator and render violation guards. Do not register a second global allocator or add a generic profiler. Give all-features builds an explicit audited precedence; the timing runner must reject an instrumented/mixed-feature binary rather than silently timing it. Verify default, timing-only, byte-accounting, and combined-feature selection with focused tests; do not disable the default counter/abort tests globally.

Count only successful alloc/alloc_zeroed requests as newly live bytes; subtract the original layout on deallocation; for successful realloc replace old requested size with new requested size, and retain the old live amount on failure. Handle overflow with an invalid-measurement state rather than an apparently valid wrapped result. Preserve current event counter definitions, including their established realloc accounting. Process-wide live accounting is needed because allocation and release may occur on different threads. Provide a high-water observation window that starts at current live bytes without zeroing the live balance. Sample/report outside timed regions.

Label the result `live_requested_heap_bytes` / `peak_live_requested_heap_bytes`. It excludes allocator headers, page rounding, stacks, and internal allocator transients during realloc; it is not RSS or total resident memory. Record fresh-process RSS high-water separately where supported. Allocation totals/requested-byte totals are not a peak-memory substitute. Pool backing allocations count once in the global ledger; pool suballocation/occupancy is a separate metric, never added again to global live bytes.

## Objective gates

- Exercise real successful allocations, zeroed allocations, deallocation, successful shrink/grow realloc, live-baseline windows, and cross-thread release. Test failure/overflow transitions through the accounting helper with independent expected arithmetic; avoid relying on an OS OOM experiment.
- Known allocate/hold/free controls distinguish cumulative traffic from outstanding/peak bytes. A before-window allocation freed in-window cannot underflow an incorrectly reset counter.
- Existing thread-scoped counters and render abort tests remain unchanged in meaning. The new hook cannot allocate, lock, log, or unwind.
- Run focused bench-support tests, strict Clippy with and without the feature, benchmark/workspace policy and formatting. No timed workload.

Actual byte measurements use a fresh dedicated process with unrelated worker activity excluded or explicitly reported. Only the verified direct-System binary supplies comparative latency records. Instrumented passes supply counts and bytes, not performance numbers. The final comparison describes the benchmark System allocator and does not assume an embedding host uses the same allocator.

## Delegation and checkpoint contract

Assign one fresh `gpt-6-luna` agent with `reasoning_effort=max` to this bounded issue. Root briefs/reviews the issue and owns exact-path commits, integration and GitHub synchronization. This user-selected model workflow supersedes historical Terra defaults for this series. Each attempt is one coherent implementation pass plus one root adversarial verdict; maximum five attempts, then preserve evidence and split/rebrief. Root commits every compiling, focused-green tranche before any further implementation layers on it. No implementation slice should exceed half a working day; split before expanding beyond the named boundary. Independent tooling may run in its own worktree/target; overlapping source owners are sequential. Final independent verification remains the user's fresh Astra XHIGH review after implementation and capture. Status: scoped; implementation and official measurements have not started.
