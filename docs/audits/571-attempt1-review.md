# Issue 571 attempt 1 review

Reviewed head: `1716c5fb6dce228ef969411d6f7bc5c757203317`

Reviewer: Astra LOW

Verdict: **FAIL**

## Blocking findings

1. Automation cancellation publishes staged `PendingUnsupported` and newly owned queued batches before the generic cancellation request is visible to render. A concurrent `begin_boundary` can therefore expose an unvalidated batch for application. The repair must keep unsupported/control-retained work non-applicable throughout cancellation and add a deterministic barrier regression around this publication window.
2. The adapter removed the prior serial-plus-queued identity preflight. At exhaustion it can dequeue accepted work and reach `own(batch).expect(...)`. Restore every identity preflight before reliable reservation, dequeue, or publication and prove refusal preserves queued work, reservations, and credits.
3. The generic tests are sequential and do not prove the required two-thread publication/request/acknowledgement/collection boundaries, publication refusal between acknowledgement and collection, required production mutations, generic resource/overflow accounting, repeated cancellation allocation behavior, or complete stale-token/invalid-prefix failure atomicity.
4. Strict Clippy fails at `delivery.rs:466` on `clippy::question_mark`.

## Independent checks

- delivery ownership integration tests without `test-support`: 4 passed;
- delivery ownership integration tests with `test-support`: 5 passed;
- protocol library tests: 144 passed;
- formatting, whitespace, workspace policy, CI routing contract and classifier tests: passed;
- `cargo clippy -p protocol --all-targets -- -D warnings`: failed on the lint above.

The generic core retains one ticket authority and separate prepared cancellation capacity. Its render loop is capacity-bounded and showed no allocation, lock, syscall, callback, or reclamation defect in this review. Those accepted parts remain subject to the corrected exact-head review.

## Attempt 2 boundary

Keep the existing three protocol path claim. Repair cancellation ordering/representation so unpublished automation remains control-owned and is canceled without becoming render-applicable, restore checked preflight, add the missing discriminating gates and mutations, and fix lint. No builtin, controller, queue, SPSC, graph, host, C ABI, artifact, workflow, or dependency expansion is authorized.
