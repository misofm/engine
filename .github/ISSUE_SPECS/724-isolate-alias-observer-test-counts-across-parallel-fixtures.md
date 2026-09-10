## Problem and authority

Delivery repair for #709/#722 after PR #723. Reviewed head
98018830ec9a8462d22dc62d2740dc45386d1aab merged as
41ef9b7667a37427e490bfe3abf3b3cebd3e118f. Required PR qualification
34489225879 passed, but main run34490147028 workspace-debug job102914615009
failed `serialized_alias_observer_is_the_decline_boundary`: observed12 versus
expected3 callbacks. Exact log is `/tmp/rack-meter-main-debug-failure.log`.
Do not rerun away or hide the failed main receipt.

Independent Astra XHIGH diagnosis identifies test-instrumentation interference:
`ALIAS_OBSERVATIONS` is global, the failing test resets/reads it under the old
pair-witness mutex, and the new resident-meter fixture invokes the same alias
observer concurrently without that mutex. PCM and thread-local capture checks
passed. No production duplicate-observation defect is demonstrated. Other
fader witnesses do not require expansion: these fixture owners decline before
fused-factory increments.

User routing: Astra LOW implementation, Astra XHIGH scoping/verification. This
is a separate bounded test-isolation correction, not reopening exhausted #714
or expanding #722's two-expression product scope. #709/#722 are passive pending
this delivery dependency; this issue occupies the active implementation slot.
At most two active slots overall. Lane B retains pin ownership.

## Smallest closable correction and ownership

Only `crates/builtins-compiler/src/lib.rs`, inside its existing tests module:
- Change ALIAS_OBSERVATIONS storage/accesses to thread-local Cell<usize> beside
  the already-thread-local ALIAS_CAPTURE.
- Add one inline deterministic regression using actual AliasObserver calls.

Own this numbered spec. Preserve production bytes, observer behavior, the exact
HARNESS_BLOCKS==3 assertion, all other counters, policies, dependency contracts,
pin and qualification expectations. No production repair, broad test rewrite,
new harness framework, artifact rebuild/repin or performance campaign.

## Discriminating regression and finite gates

Two scoped threads synchronize after both reset their count/capture and again
after making two versus five actual alias-observer calls. Each must see only
its own count and captured samples. Put count/capture assertions after the
second barrier so the negative control cannot strand the other thread.
Restoring the old shared counter must deterministically report the combined
count7 and fail the intended assertion. Use one external physical mutation;
preserve its input/diff/command/exit/output and restore exact source.

Astra LOW implements one coherent pass and runs formatting, the new regression
and existing alias/resident selectors. Stop on the first unexpected failure;
record actual commands, environment, exits and logs externally. Pause when
focused checks pass for root exact-path checkpoint/push and GitHub sync.

Astra XHIGH verifies the test-only scope, deterministic regression and one old-
counter negative control, then runs the full builtins-compiler library suite
with test-support once in debug and release-unwind, plus focused strict all-
target/all-feature Clippy. Reuse prior product/artifact evidence; do not launch
a repeated browser campaign. The existing five-attempt ceiling and candid
stop/rescope rules apply; do not weaken the exact callback assertion.

## Delivery

The current base is origin/main41ef9b76, whose post-main qualification failed;
it is not a delivered PASS baseline. Use an isolated corrective branch and
frequent exact-path checkpoints. Independent XHIGH PASS precedes one PR.
Require exact-head PR qualification and a guarded merge, then successful
qualification of the corrected main. Existing CI must reproduce the unchanged
qualified ea8f843b artifact pin. No special bypass or blind rerun of main41ef.

Only after corrected-main PASS synchronize this issue and #709/#722 closure.
Their records must preserve the original failed main run and name this repair
and both qualification runs. #714 remains superseded without retroactive PASS.
Remove clean delivered worktrees after preserving branches/history and external
evidence. No registry publication, speedup or full-RT10 claim.

## Status

Independent XHIGH scope PASS established before implementation. No correction
or rerun has been performed. Earlier source/shared-artifact PASS remains earned;
#709/#722 delivery is blocked on this test-isolation correction and required CI.

## Attempt 1 implementation checkpoint

Astra LOW changed only the alias count storage/accesses inside the test module
and added the deterministic two-thread 2/5 observer regression. Formatting, new
regression (1), alias selector (3), and resident-meter selector (2) passed.
Actual commands, environment, exit status and logs are preserved externally in
`/tmp/issue724-attempt1-implementation`. Cargo's generated dependency ordering
change was preserved externally and restored; no dependency change is included.
These focused commands lacked --locked; independent review uses --locked.
Root audited the exact-path checkpoint. Independent XHIGH verdict and CI remain.
