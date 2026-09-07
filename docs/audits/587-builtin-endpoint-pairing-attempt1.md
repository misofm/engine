# #587 attempt 1 implementation record

This record covers the uncommitted Luna implementation tranche from the #587 brief.

## Source decision

`prepare_builtin_batch_endpoint_with_backend` now uses a crate-private preparation route with
`BuiltinControlDelivery::BetweenRenderCalls`. The public raw console preparation remains on the
`Concurrent` policy, including the test-only backend-selected reference helper. The public
`prepare_host_runtime_between_render_calls` delegates through the same route with
`Backend::current()`. No compiler, graph, builtin, manifest, lockfile, or public API path changed.

The endpoint test fixture has a private meter-free preparation option so pairing eligibility can be
observed without weakening the separate PostFader fixture. The new endpoint/reference test uses the
existing native bank and forced scalar backends, pushes asymmetric fader and matrix records with
smoothing, compares every output `f32::to_bits()` value, and asserts nonzero factory/process/member
witnesses plus scalar state words. The existing PostFader test remains a separate observation-barrier
case and retains bitwise PCM, state trace, and meter-bit comparisons.

## Focused results

Both focused tests pass in debug, including when all endpoint unit tests run together:

```text
cargo fmt --all -- --check                         PASS
cargo test -p host-core --lib --features control-provider \
  builtin_batch_endpoint::tests:: -- --nocapture   PASS (6 passed)
```

The two named focused tests also pass independently. `git diff --check` passes and `Cargo.lock` was
restored after cargo invocations.

## Direct red mutations

1. The endpoint route's policy boolean was temporarily changed from `true` to `false`, forcing the
   endpoint back to `Concurrent`. The pair discriminator failed at
   `builtin_batch_endpoint.rs:1538` with:

   ```text
   native bank pair factory was not selected: TestOnlyFaderMatrixWitness {
     process_calls: 0, factory_calls: 0, process_members: 0, factory_members: 0,
     fader_records_drained: 2, matrix_records_drained: 2, ...
   }
   ```

2. In `crates/builtins-compiler/src/lib.rs`, the selected `FaderMatrixBankProcessor::process`
   operation was temporarily changed to add `1.0` to the first left output sample. The bitwise
   reference discriminator failed at `builtin_batch_endpoint.rs:1537` with the first output word
   changing from `2763632810` to `1073741824`; the temporary compiler edit was restored.

## Remaining scope

This tranche does not yet provide the full six-block immediate/ramping/settled/mid-ramp-retarget/
mute/unmute schedule or dedicated bank and scalar decline fixtures required by the complete #587
objective gate. Those are explicit follow-up work for review rather than being represented as
completed evidence here.
