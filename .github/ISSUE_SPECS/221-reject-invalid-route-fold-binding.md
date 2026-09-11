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

## Bounded rack API amendment — Astra XHIGH, 2026-09-11

Inspection before implementation found no existing infallible fold-installation
seam. Extend exact-path ownership to crates/rack/src/lib.rs and focused rack tests.
Use an opaque prepared fold configuration that validates width, its owned/copied
active mask, and requested fold mask during borrowed graph preflight. A narrow
constructor variant consumes that configuration as the sole source of the new
chain's active/fold masks: no second caller-supplied active mask and no transferable
setter that can apply a proof to an arbitrary chain. Delegate ordinary scratch/slot
shape checks to existing BankChain::new, then install the validated fold through a
private infallible helper shared with arm_fold. Preserve existing arm_fold behavior.
Graph carries each configuration in the same validated run plan into chain_for /
build_chain, paired with the first run bank's exact width/active metadata through
pairing/fallback. All fold-mask errors precede ownership consumption. The existing
bank-shape constructor Result/expect is unchanged; no new post-consumption fold
error or panic is permitted. Gates include width mismatch, inactive armed lane,
all-false disarm, valid partial/full masks, and proof/configuration misuse prevention.
Compare old/new constructor paths at widths 4/8 for identical masks and bitwise
PCM, including prefix subsets with additional active lanes left unfolded, active
mask holes, all-false/disarm, and partial-bank staging allocation before render.
Proof supplies the actual active mask, not a comparison copy. Cover pairing
success/fallback. No unsafe or unchecked public setter, mask truncation/intersection
fallback, or render-path change.
This is a scope correction before source edits, not a failed implementation attempt.

## Historical issue body

Required follow-up F2 from strip Job 3's verification: the `debug_assert!(false)` arms in `apply_route_fold` are commented as inert, but by the time any arm runs the route ops are already retired from unit emission — a reachable instance in RELEASE would be silent wrong audio (stale route buffers / vanished contribution), not a safe decline. Each arm was verified unreachable today (routes are singleton plain units; membership⇒Bank; active masks are planner prefixes), and the code runs at bind on the control plane — so the fix is cheap: make them hard bind errors. Also (F6, same area): console-workload's `GraphIdentity` do-nothing Bound processor should become `GraphNodeBinding::identity` per the new zero-input contract.