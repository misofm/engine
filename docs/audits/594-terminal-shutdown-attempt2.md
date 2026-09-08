# Issue 594 attempt 2 implementation record

Luna HIGH implemented attempt 2 within the two authorized files. Root committed the source/race
checkpoint as `1a99a968` and the complete evidence checkpoint as `c59b961d` before documentation.
No protocol, engine, DSP, manifest, lockfile, host, browser, SDK, artifact, policy, workflow, or lane-B
path changed.

## Source corrections

- Pending lifecycle intent with an empty generic cancellation queue is treated as the legal
  intent-before-publication window; the current block renders normally and the later boundary consumes
  the message.
- Both reusable cancellation and terminal shutdown roll every lifecycle field back to Idle if generic
  begin fails.
- Shutdown refuses while an ordinary cancellation is still active or its cached generic completion
  has not received final endpoint publication.
- A test-only rendezvous pauses after generic acknowledgement publication and before endpoint
  lifecycle classification. Ordinary control finalization waits for render to publish Idle, so a new
  shutdown cannot attach to the old boundary.
- Sticky render faults publish the invalid lifecycle byte and prevent shutdown certification while
  preserving accepted-ticket ownership.

## Direct behavior and resource evidence

Debug integration now has 20 passing tests and endpoint unit coverage has nine. Tests cover the legal
intent-before-message race and rollback, post-ack/pre-classification race, cached ordinary completion,
stale/duplicate shutdown operations, permanent admission refusal, and exact Applied-uncollected plus
claimed-future plus queued dispositions/prefixes/sample metadata. Native paired-bank and private
forced-scalar runs preserve output bits, plan time, addressed scalar state, raw drains, and pair
process/member counters across acknowledgement and repeated quiescent calls.

The shutdown acknowledgement and repeated terminal calls report zero render allocations/frees.
Thread-scoped allocator counters independently observe the `Arc<AtomicU8>` allocation and its
requested bytes against the resource report; exact and one-below retained/largest caps pass, and
`stop()` retains the final render-side owner until off-render drop. An initial process-wide allocator
assertion was nondeterministic under parallel tests; Luna replaced it with the repository's delivered
thread-scoped counter before this checkpoint.

## Commands

All returned exit 0:

- `cargo test -p host-core --features control-provider --test builtin_batch_endpoint --no-fail-fast`
  — 20 passed.
- `cargo test -p host-core --features control-provider builtin_batch_endpoint --lib --no-fail-fast`
  — 9 passed.
- `cargo test --release -p host-core --features control-provider --test builtin_batch_endpoint --no-fail-fast`
  — 20 passed.
- `cargo clippy -p host-core --features control-provider --tests -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core --features control-provider --no-deps`
- `cargo fmt --all -- --check`
- `git diff --check`

Cargo regenerated its known dependency-order drift during verification; root restored it. The final
checkpoint contains only the two authorized source/test paths.

## Restored behavioral mutations

Each mutation was applied alone, the named command exited 101 at the intended assertion, and source
was restored before the next mutation.

1. `poll_shutdown` temporarily synthesized `ShutdownAcknowledged` plus a cached completion before
   generic polling. The stale-token shutdown test failed because the pre-ack poll returned a
   completion instead of `Ok(None)`.
2. `poll_shutdown` temporarily removed the nonzero-outstanding gate. The terminal reconciliation test
   failed because completion appeared before final collection.
3. `poll_cancel_boundary` temporarily removed the Idle lifecycle requirement. The post-ack
   classification-rendezvous unit test failed because ordinary completion published during the hold.
4. `try_publish` temporarily allowed `ShutdownAcknowledged`. The stale-token/admission test failed its
   permanent post-completion refusal assertion.
5. `StartedBuiltinBatchRender::render` temporarily disabled the terminal early return. The selected
   pair test's repeated quiescent call failed with `DiscontinuousTime { expected: 768 }`, proving graph
   execution was attempted.

Focused commands respectively targeted
`shutdown_rejects_stale_token_before_and_after_completion`,
`terminal_shutdown_acknowledges_once_then_quiesces_and_reconciles`,
`ordinary_ack_before_classification_cannot_be_overtaken_by_shutdown`, the stale-token test again, and
`endpoint_selects_existing_pair_factories_without_observer_barriers`. Final status/diff checks exited
0 with only the authorized files staged in `c59b961d` and no `Cargo.lock` diff.
