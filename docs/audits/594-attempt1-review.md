# Issue 594 attempt 1 — Astra LOW review

Reviewed exact clean pushed source checkpoint
`111c2b0e` against delivered main `e16cea23` and the approved #594 scope.

Verdict: **FAIL**. Two implementation attempts remain.

## Blocking findings

1. The render `DeliveryError::Empty` arm turns any pending lifecycle state into sticky Fault. Both
   begin methods intentionally store pending intent before publishing the generic cancellation
   message, so a render boundary may legally observe intent while the message is not yet visible.
   This race can strand the cancellation and every accepted ticket. Attempt 2 needs a deterministic
   rendezvous between intent store and generic publication, including failed-begin rollback.
2. `begin_shutdown` does not gate `cancellation_started` or a cached `cancel_complete`. After render
   acknowledges an ordinary cancellation and sets the handshake to Idle, control may cache the
   generic completion, collect the last ticket, then begin shutdown before the final ordinary poll.
   Shutdown overwrites the old endpoint token/completion. Attempt 2 must require ordinary endpoint
   finalization before shutdown begins and directly regress this sequence.
3. The new tests are sequential and omit the approved post-generic-ack/pre-classification
   rendezvous, begin rollback, stale shutdown tokens, and the combined Applied-uncollected,
   claimed-future, and queued ownership case. Sticky-fault shutdown refusal and retained ownership
   are not proved.
4. Shutdown-specific PCM bits, plan time, addressed state, raw drains, pair counters, forced-scalar
   behavior, zero allocation/free, `stop()` lifetime, actual Arc accounting, and exact/one-below
   preparation need direct evidence.
5. None of the five required restored behavioral mutations is recorded.

## Accepted observations

The implementation stayed inside the two authorized source paths. The four-state classification and
ordinary-poll gating otherwise follow the approved design; no protocol or DSP widening is required.
Astra independently passed 16 integration tests, six endpoint unit tests, strict Clippy, formatting,
and diff checks. The generated `Cargo.lock` ordering drift was restored and is not part of the
checkpoint. Release verification did not finish in the reviewer session and is not claimed.
