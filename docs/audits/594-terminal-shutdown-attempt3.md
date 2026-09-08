# Issue 594 attempt 3 implementation record

Luna HIGH changed only the two authorized endpoint/test paths from review checkpoint `6d05360d`.
Root committed the final evidence tranche as `d49fe6fa`. Production lifecycle behavior accepted in
attempt 2 is unchanged; this final pass corrects only the three evidence blockers.

## Exact lifecycle allocation and retention

The integration test warms allocator state, then uses thread-scoped counters around an independent
`Arc<AtomicU8>`. It proves exactly one allocation, zero reallocation/deallocation before drop,
requested bytes exactly equal `BuiltinBatchResources::lifecycle_heap_bytes`, and exactly one
subsequent deallocation.

An authorized private unit test directly examines the real endpoint owners. The lifecycle Arc strong
count is two before control drop, one on the render owner afterward, and one on
`StoppedBuiltinBatchRender` after `render.stop()`. The test destructures the stopped owner and drops
all non-lifecycle fields before marking thread-scoped allocator counters. Dropping only the retained
lifecycle Arc then produces exactly one off-render deallocation with no allocation or reallocation.
This isolates the lifecycle allocation from plan/queue teardown.

## Failure-safe publication rendezvous

The successful intent-before-generic-publication test now creates the release sender inside
`thread::scope`, so failure unwinding drops it before the scoped worker join. A second explicit path
drops the release sender while the worker is held: generic begin returns `DeliveryError::Empty`, every
endpoint cancellation field and the shared lifecycle roll back to Idle, scope completes without a
hang, and a later ordinary cancellation succeeds.

## Direct graph-execution mutation

The restored fifth mutation disabled the `ShutdownAcknowledged` early return while the selected-pair
test called repeated render at the unchanged valid shutdown sample and correct shape. Command:

`cargo test -p host-core --features control-provider endpoint_selects_existing_pair_factories_without_observer_barriers --lib`

It exited 101. The repeated call returned a graph report and advanced `next_absolute_sample` to 896
instead of returning the stable shutdown report with unchanged clock/state. This directly reaches and
detects forbidden graph execution. Source was restored before the checkpoint.

## Verification

All commands returned exit 0 after restoration:

- `cargo test -p host-core --features control-provider --test builtin_batch_endpoint --no-fail-fast`
  — 20/20.
- `cargo test -p host-core --features control-provider builtin_batch_endpoint --lib --no-fail-fast`
  — 11/11.
- `cargo test --release -p host-core --features control-provider --test builtin_batch_endpoint --no-fail-fast`
  — 20/20.
- `cargo clippy -p host-core --features control-provider --tests -- -D warnings`
- `RUSTDOCFLAGS='-D warnings' cargo doc -p host-core --features control-provider --no-deps`
- `cargo fmt --all -- --check`
- `git diff --check`

`Cargo.lock` has no diff. No other path changed.
