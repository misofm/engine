# Issue 571 attempt 2 evidence

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
