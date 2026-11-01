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

## Decision record (attempt 1)
- D1. `project_for_arrivals` replaces `compile_for_arrivals`: it projects the effects (`project_native_session_effects`), prepares the builtins as before, and calls `GraphCompiler::project` with the lead's floors. `GraphProjection` gains `edges()` and `schedule()`. `first_late_node`, `walk_back`, `strips_reaching_output` and `compiled_sidechain_lines` take slices, so the warm successor's own C1 re-check after its compile reads the plan's rows through the same functions. `prime_bytes_of` reads the track delays only (no effect state), so it reads `GraphProjection::track_delays()`.
- D2. A refusal that only the back half of a compile can return (a bank bind or a processor prepare) is no longer reachable from `warm_lead`. The ordinary successor of the same session was already prepared, so this changes no answer on any test (D4).
- D3. `plan_strip_transition` grows with strips: 12.1 MB at 64 tracks, 48.4 MB at 256, 193.5 MB at 1,024 (bus-add shape, every strip restarted, thread peak). Almost all of it was `controls::changed_controls`, which held both models' whole control lists (about 190 KB per console strip). It now walks one owner at a time through one buffer, as the catalog does since #1400b, with the same rows compared in the same order. After: 0.63 MB, 2.53 MB, 10.1 MB (about 9.9 KB per strip: the model copies and the two canonical documents of the classifier).
- D4. The residual is not charged in the replacement pre-check. That admission charges exact rows only, so that it never refuses a candidate the exact admission after preparation admits (`projected_plan`); a transient ceiling there must also be charged by the exact admission. The residual is at most 1.06 × the committed model's `canonical_upper_bound_bytes` and 2.6 × its `compiled_model_bytes` at all three sizes. Root ruling (2026-11-01): accepted as a stated limit here; #2234 charges it in both admissions.

## Evidence (attempt 1)
- Gate (`crates/host-core/tests/warm_lead_peak.rs`): 64-track console bus add, answer `Unavailable(Misaligned)` at base and head. Base: peak 55,857,768 B over the 50,066,072 B projected effect state (red). Head: 3,588,608 B. At 256 tracks (scratch): base 222,707,552 B, head 13,970,621 B, projected 200,224,664 B.
- Answers: a scratch differential ran the old full compile beside each projection and compared arrivals, schedule, edges, nodes, inserted delays and track delays: 147 passes equal (host-core 98, control-plane 49) over every test of host-core, control-plane, capi and host-web. `warm_successor` 22 of 22 pass unchanged.
- No rendered bit moved: the 34 digests over `fixtures/session/v1` (C ABI, 300 blocks, plain and live, scratch harness) are identical at base 222411be4 and at head.
- Browser, with #1360's S2 (3fbe53670) cherry-picked onto head in a scratch worktree only: `--check-matrix` passes on Chromium 151 and WebKit 26.5, single and worker; `console-limits` replies `ok` on the rebuild path, revision 0 to 1, render allocations 0, on all four legs.
- Checks: `cargo fmt --all --check`; workspace clippy `--all-targets --all-features -D warnings`; release tests of graph-compiler, host-core, control-plane, capi, host-web and controls with `--all-features`; `check-workspace-policy.sh`; `realtime_policy`; `check-test-support-ci.py`; the web build and `check-web-audioworklet.sh`: all pass.
