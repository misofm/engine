# Issue 571 attempt 2 review

Reviewed head: `607352f80205fe88986a7e736bf2cecf6784dbde`

Reviewer: Astra LOW

Verdict: **FAIL**

## Accepted corrections

The attempt-one runtime defects are fixed by inspection: unsupported/control-retained automation no longer enters the render data ring during cancellation, serial-plus-queued overflow is checked before reservation/dequeue, and strict Clippy is repaired. The common ticket authority, independent cancel capacity, bounded render loop, ordinary dispositions, and automation event/reservation behavior remain sound.

## Remaining blocking evidence

1. The staged-automation test calls `begin_cancel` before starting the render thread and cannot fail on the former publication-before-cancel race. Add a deterministic assertion that control-retained entries never become render-published, or an equivalent schedule that exposes the old defect.
2. Generic two-thread coverage exercises only zero application. Add deterministic partial and full application cases around cancellation and assert exact disposition, prefix, remainder, acknowledged sample, frontier, and publication refusal after incomplete collection.
3. Add a measured generic preparation/resource-report comparison, repeated cancel/reuse cycles with zero allocation/free, generic generation and serial overflow refusal, a stale prior cancellation token during a later generation, and complete duplicate-terminal/reused-ticket rejection.
4. The existing early-publication mutation proves the public gate but does not independently prove that physical slot ownership cannot be released before collection. Add a discriminating physical-credit mutation check and preserve its failure.

## Independent checks

- `cargo test -p protocol --all-targets --features test-support`: 157 passed;
- strict protocol Clippy with `test-support`: passed with existing unrelated allowlist warnings;
- formatting, whitespace, workspace policy, CI routing contract and classifier: passed;
- the recorded early-ack and early-publication mutations are discriminating and were accepted.

Attempt 3 is the final allowed attempt. It is limited to the missing focused tests/evidence and any smallest source seam strictly required to make those tests discriminating. A third FAIL triggers the hard stop and rebrief; gates cannot be weakened.
