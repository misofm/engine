# Issue #579 attempt 1 qualification evidence

This record qualifies the preserved #576 builtin batch endpoint at the #579 successor head.
The implementation remains limited to `builtin_batch_endpoint.rs` and its focused integration
test. No protocol, graph, engine, manifest, lockfile, or lane-B path was changed.

## Cancellation and ownership

The endpoint retains the one generic cancellation acknowledgement and returns `None` from
`poll_cancel_boundary` until every captured endpoint ticket has passed through `collect`. Applied
metadata is staged before generic collection; after acknowledgement, a missing metadata row is
the never-injected canceled case. An applied prefix followed by a canceled suffix, out-of-order
collection, duplicate collection, stale generation rejection, and next-generation reuse are
covered by the focused integration tests. A private scoped test holds the first generic claim,
publishes a second ticket while the claim is held, then proves the second application is at sample
128 and is late. The control/render rendezvous channels are created inside `thread::scope`, and a
sender-drop panic path joins without a sleep or timeout.

The post-graph fault seam remains sticky. The focused source test attempts cancellation and
collection after the fault and observes `Ok(None)`, `Empty`, and an unchanged outstanding ticket;
the ticket cannot be labeled canceled or reused.

## Audio and resources

`endpoint_drives_nonzero_pcm_through_the_prepared_bank_and_scalar_plan` feeds identical source
planes and fader/matrix records to the endpoint and an independently prepared ordinary console
owner. It compares the complete PCM arrays bitwise and requires nonzero output. The endpoint
report exposes the bank scratch witness; the existing host preparation remains the separate-owner
reference. The private claim test separately proves direct FIFO dispatch ownership.

The requested paired-dispatch counter witness remains blocked by the exact ownership boundary:
`StartedRenderSession` keeps its `PreparedRenderPlan` private and exposes no dispatch-counter
reader, while the #579 allowed paths exclude `crates/host-core/src/render_session.rs` and the
engine/graph production crates. `HostPrepareReport::effect_bank_scratch_bytes` and PCM equality
cannot truthfully substitute for a post-render paired bank/scalar dispatch count. This gate is
therefore reported as an explicit rebrief blocker rather than claimed as satisfied; obtaining it
requires a smallest test-only accessor in the frozen render-session boundary or a separately
authorized successor path.

Resource rows are named by ownership: host builtin payload, endpoint heap, composed heap, largest
endpoint heap, largest host engine allocation, largest composed heap, and prepared/started render
inline sizes. The integration test independently mirrors the concrete SPSC layouts and ledger
array, comparing bytes and largest allocation against the endpoint's report. Exact and one-below
aggregate/largest cap cases pass. The workspace audited allocator supplies the positive
allocation/free liveness control; render application and cancellation snapshots report zero
allocations and frees. A local second global allocator was deliberately not installed because
`bench_support` already owns the workspace global allocator.

## Focused gates

```
cargo test -p host-core --test builtin_batch_endpoint --features control-provider
12 passed; 0 failed
cargo test -p host-core --lib --features control-provider \
  builtin_batch_endpoint::tests::private_post_claim_hold_rejects_same_block_second_claim
1 passed; 0 failed
```

## Direct mutation evidence

Each mutation was applied to the named operation, run with a 30-second safety bound, and restored
from the clean source immediately afterward.

1. Forcing a second `DeliveryCoreRender::begin` while the first claim was pending failed the
   private claim test at `render: Delivery(AlreadyPending)` and the report rendezvous returned
   `RecvError`.
2. Adding a second claim/injection/terminal after the first finish, before the render call
   returned, failed the private claim test at `second application`; the second ticket was
   incorrectly consumed in the current block.
3. Changing `for record in batch.records()` to `batch.records().take(1)` failed the endpoint PCM
   reference at the exact `assert_eq!(endpoint_samples, baseline_samples)` assertion, with the
   left endpoint array missing the matrix transform and the right reference retaining it.
4. Calling generic `delivery.collect(ticket)` before endpoint outcome staging failed the FIFO
   test at `first terminal: StaleTicket`; generic ownership had already been released before
   endpoint reconciliation.
5. Replacing the post-graph sticky `Err(Fault)` with a healthy report failed the source test at
   the exact assertion expecting `Err(Fault)`, showing the observed value as
   `Ok(BuiltinBatchRenderReport { ... applied: None })`.

All five mutations were removed before the green rerun. The mutation records are negative
evidence only; no mutation output is used as a product result.
