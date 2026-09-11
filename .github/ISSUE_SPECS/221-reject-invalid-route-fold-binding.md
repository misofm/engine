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
## Attempt 1 checkpoint: rack construction

Astra LOW added opaque owned active/fold configuration and a consuming constructor;
arm_fold shares its private installation helper. Formatting and six focused rack
fold tests passed (cargo test --locked -p rack fold --lib). Evidence with actual
argv/env/source/log/exits: /tmp/issue221-attempt1-rack. Graph is unchanged; borrowed
preflight, public ownership controls and full old/new PCM controls remain pending.
Root checkpoints this coherent tranche before further implementation; no PASS
verdict or delivery is claimed.

## Attempt 1 checkpoint: borrowed graph planning

Borrowed preflight precedes ownership transfer and hands the same owned schedule,
mappings and fold configurations into consuming construction. The old
post-consumption apply_route_fold step is removed. Formatting, focused route
controls, the seeded4000-graph corpus and RT1 direct-bank bit-exact/allocation
control passed. An initial unused test-wrapper warning was corrected; final
format/route rebuild passed without it. Receipts: /tmp/issue221-attempt1-graph-plan.
Public ownership/fault matrix, rack PCM/pairing controls, console identity and
strict Clippy remain before independent attempt review.

## Attempt 1 checkpoint: public recovery matrix

Eleven one-shot raw installation faults run through both public bind families
(22 rejection/retry paths). The focused test checks exact returned graph/bank/
bound/observer owner addresses, source driver identity, no source work/drop
before rejection, then successful same-owner retry with four folded lanes and
known three-block PCM. Added folded-op mapping consistency rejection. Formatting
and focused recovery test passed; receipts /tmp/issue221-attempt1-recovery.
Rack PCM/pairing, console identity and strict gates remain before final review.

## Attempt 1 implementation evidence — console identity gate pause

Rack prepared-fold constructor/PCM tests (2), graph accepted/declined pairing test
(1), and the existing poisoned zero-input Identity/Bound control (1) passed.
`cargo test --locked -p console-workload --test chain_shape` exited 101: 20 passed,
`the_half_mono_cohort_banks_like_a_uniform_one` failed at line 472 because the mono
symmetry census was `(64, [65, 129])`, previously expected `(64, [64, 129])`.
The helper now binds the master with the requested `GraphNodeBinding::identity`;
its identity witness appears to explain the additional eligible unit. This is a
hypothesis pending independent review, not a changed acceptance gate. Source edits
stopped at this failure; strict Clippy was not launched by the sequential runner.
Exact source/head/argv/environment/logs/exits are preserved externally in
`/tmp/issue221-attempt1-pcm`. No benchmark or timing claim is made.

## Attempt 1 preserved gate failure

Rack old/new constructor PCM controls, pairing success/fallback and existing
poisoned identity-buffer control passed. The required console helper now uses
GraphNodeBinding::identity for non-source nodes. Console chain_shape compiled
and passed20 tests, but the_half_mono_cohort_banks_like_a_uniform_one failed:
mono eligible counters were [65,129], expected [64,129] at line472. Execution
stopped before Clippy; no retry or correction was made. Preserve this useful
compiling checkpoint and /tmp/issue221-attempt1-pcm logs/argv/env/exits. Astra
XHIGH is reviewing whether the additional master identity explains the counter
and what bounded correction is warranted; attempt PASS is not claimed.
