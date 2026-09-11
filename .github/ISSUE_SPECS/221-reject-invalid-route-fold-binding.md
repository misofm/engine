# apply_route_fold's debug_assert arms must be hard bind errors

## Approved bounded scope — 2026-09-11

Prerequisite: #220 delivered with independent differential coverage and required
PR/main CI. Astra XHIGH scoped the following ownership-safe correction.

Return hard bind errors for invalid route-fold installation before ownership
transfer. GraphBindFailure/GraphSourceBindFailure must return every original
caller-owned input; simply making build_sequential fallible after consuming
RuntimeParts is forbidden. Use a borrowed preflight before bind_optional_source_set
moves observers/bindings. Reuse the same run/fold planning logic through a narrow
borrowed metadata view; avoid a second independent routing algorithm. Validate
fold run mappings, bank-unit ownership, exact prefix masks against first bank
width/active lanes, and emitted plain-singleton master mapping. Include missing
mappings currently silently skipped. The consuming executor must use the same private validated planning result/proof.
All recoverable route-fold installation errors return before any observer,
binding or source ownership transfer, pairing-factory consumption, route
retirement or fold arming. Application of that validated result is infallible;
it must not recompute an unchecked fold plan or replace assertions with panics.

Do not add generic post-consumption recovery, change route-fold admission or PCM,
or weaken any transactional bind promise. If the smallest implementation cannot
reuse existing planning without broad architecture expansion, pause and report.
Own crates/graph/src/lib.rs, runtime.rs and directly relevant graph tests;
console-workload helper/callsites replace GraphIdentity with GraphNodeBinding::identity
as already requested by this issue. Root owns artifact qualification/pinning.

Gates: bounded fault injection for each former assertion/missing mapping through
preflight/public bind; exact typed rejection and original owners/source set returned,
then successful retry. Preserve #220 decisions, fold counts and bitwise PCM in
debug/release; strict focused Clippy; zero-input identity poison-buffer control.
No new performance framework or compiler-IR captures. Coordinate shared artifact
qualification with accepted #162 compiler borrowing if both remain frozen.

Astra LOW implements; Astra XHIGH independently reviews. Five attempts maximum;
root checkpoints/pushes exact paths promptly on focused green, before more work.
At most two active issues (#221 and #162 once #220 closes). Isolated worktrees.
Record actual commands/env/source/exits/logs externally; stop unexpected failure.
Required reviewed-head PR and main qualification precede upstream issue closure
and clean delivered worktree removal. Historical model names below are superseded.

## Historical issue body

Required follow-up F2 from strip Job 3's verification: the `debug_assert!(false)` arms in `apply_route_fold` are commented as inert, but by the time any arm runs the route ops are already retired from unit emission — a reachable instance in RELEASE would be silent wrong audio (stale route buffers / vanished contribution), not a safe decline. Each arm was verified unreachable today (routes are singleton plain units; membership⇒Bank; active masks are planner prefixes), and the code runs at bind on the control plane — so the fix is cheap: make them hard bind errors. Also (F6, same area): console-workload's `GraphIdentity` do-nothing Bound processor should become `GraphNodeBinding::identity` per the new zero-input contract.