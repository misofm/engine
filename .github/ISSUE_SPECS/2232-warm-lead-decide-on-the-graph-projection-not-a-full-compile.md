# warm_lead: decide on the graph projection, not a full compile

Filed by root from the #1360 attempt-1 diagnosis (2026-11-01, `scratchpad/d1360/`). #1360 waits for this issue.

## Problem
`warm_lead` decides whether a latency-growing edit can take a warm successor. For each pass it calls `compile_for_arrivals` (`crates/host-core/src/prepare.rs:2168`), which prepares every native effect (`:2178`) only to read arrivals, edges and the schedule. That allocation is covered by no admission. On the 256-track console bus add (browser qualification `console-limits`, #2191) one pass adds a 217 MB transient natively, and in Wasm the module traps at the 512 MiB cap (`RuntimeError: unreachable`, reported as `internal`). The pass count is bounded (`prepare.rs:2428`, 2 × strips + 1); the cost of each pass is not.

`GraphCompiler::project` (#2199) already holds the arrivals, inserted delays and track delays, and the plan copies its schedule, edges and delays from the same front unchanged (`crates/graph-compiler/src/compile.rs:1092-1097`), so a projection answers the same by construction.

## Scope
1. `GraphProjection` exposes `schedule()` and `edges()`. `compile_for_arrivals` builds the projection, and `first_late_node`, `walk_back`, `strips_reaching_output`, `compiled_sidechain_lines` and `prime_bytes_of` read its slices. No effect processor is prepared during `warm_lead`.
2. Measure the transient of `plan_strip_transition` over restarted strips (41 MB at 256 tracks, covered by no admission) at 64, 256 and 1,024 tracks. If it grows with strips, charge it in the pre-check; otherwise record the bound.

## Gates
- A host-core test with `bench_support`'s thread-scoped peak: `warm_lead`'s heap peak on the 64-track console bus add stays below the session's projected effect-state bytes (a ceiling). Red on today's code.
- The `warm_successor` gates 1-9 pass unchanged; `warm_lead`'s answers are equal before and after on every existing test.
- With #1360's S2 applied, browser `console-limits` passes on Chromium and WebKit, single and worker.
- No rendered bit moves (one-time comparison, PR evidence only).
