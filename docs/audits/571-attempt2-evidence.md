# Issue 571 attempt 2 and attempt 3 evidence

Reviewed implementation paths are `crates/protocol/src/delivery.rs`,
`crates/protocol/src/lib.rs`, and `crates/protocol/tests/delivery_ownership.rs`.

The corrected cancellation path keeps staged and newly admitted automation in the
control owner. The generic core terminalizes unpublished entries as canceled after
the bounded cancel request is queued; they never enter the render data ring. The
staged automation race test uses two barriers and asserts that the render boundary
has no pending application while cancellation is in flight.

The identity preflight test sets the next serial and admission order to values that
would overflow when two queued batches are included. `begin_cancel` returns
`SequenceOverflow` before reliable reservation or queue dequeue, and occupancy,
outstanding count, and event sequence remain unchanged.

The generic threaded test places publication, cancel request, render acknowledgement,
and collection on separate ownership sides. It asserts that publication remains
blocked after the acknowledgement and before both terminal dispositions are
collected. The generic invalid-prefix test confirms rejected regressions do not alter
the pending prefix and cancellation reports the preserved prefix.

Two temporary production mutations were run and restored:

1. The generic control poll's missing-ack branch was changed to return completion
   immediately. `cargo test -p protocol --test delivery_ownership
   generic_boundary_cancel_reports_zero_partial_and_full_without_releasing_credits_early`
   failed at the pre-ack `None` assertion, proving early acknowledgement is gated.
2. Generic publication was temporarily allowed after `completion_reported` but before
   terminal collection. The same focused test failed at its
   `Err(CancellationPending)` assertion, proving credit/publication release remains
   gated by collection. The mutation was reverted before the green run.

Green checks on the restored source:

- `cargo test -p protocol --test delivery_ownership`: 7 passed.
- `cargo test -p protocol --lib delivery::tests`: 21 passed.
- `cargo clippy -p protocol --all-targets -- -D warnings`: passed for protocol;
  repository configuration emits existing invalid-path warnings for the unrelated
  math fast-db allowlist.
- `cargo fmt --all -- --check` and `git diff --check`: passed.

## Attempt 3 evidence

The private inline delivery test now inspects every retained core entry across
`begin_cancel`, the render boundary acknowledgement, and control reconciliation.
Both the staged unsupported batch and the newly owned queued batch remain
`published == false`, and a direct render `begin` remains empty before the cancel
boundary runs.

The parameterized generic schedule uses two separately owned threads and two tickets
for zero, partial, and full pre-cancel application. Each case checks the captured
frontier, pre-ack `None`, acknowledged sample, applied/canceled disposition, exact
prefix and remainder, publication refusal after one collection, and final reuse.
Preparation allocation bytes and largest allocation match the exact generic resource
report. A positive teardown measurement observes off-render frees, and sixteen
cancel/reuse cycles on the same endpoints measure zero render allocations and frees.
Private tests cover serial and generation overflow transactionality, stale old cancel
tokens across a new generation, duplicate finish/collect, and reused-ticket rejection.

A temporary physical-credit mutation inserted `self.entries[ticket.slot] = None`
before `collect` drained its terminal. The focused test failed exactly with:

```
called `Result::unwrap()` on an `Err` value: StaleTicket
```

The mutation was restored. The earlier accepted early-ack and early-publication
mutation records remain above unchanged.

Final attempt-3 checks on the restored source:

- `cargo test -p protocol --all-targets --features test-support`: 149 library
  tests, 10 delivery ownership tests, 1 controller API test, and 3 builtins-rack
  tests passed.
- `cargo clippy -p protocol --all-targets --features test-support -- -D warnings`:
  passed for protocol with the repository's existing unrelated math fast-db
  allowlist warnings.
- `bash scripts/check-workspace-policy.sh`, `bash scripts/test-workspace-policy.sh`,
  and the protocol path router check passed. The workspace mutation harness emits
  its expected directed-mutant diagnostics while returning success.
- `cargo fmt --all -- --check` and `git diff --check`: passed.
