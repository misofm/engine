# Issue #580 attempt 1 evidence

This bounded successor preserves the accepted #579 endpoint source and addresses only its two
residual qualification defects.

## Cap preflight and retention

`endpoint_queue_reports` computes the delivery and outcome projections without allocating. Endpoint
retained bytes and largest allocation are checked against caps before host preparation and before
`prepare_endpoint_queues`; the actual queue helper receives the accepted reports only after that
preflight. The existing composed host-plus-endpoint checks remain after host preparation.

The focused cap test uses warmed current-thread allocator counters. One-below endpoint retained
and one-below endpoint-largest caps both return `BuiltinBatchPrepareError::ResourceLimit` with
zero allocations, frees, reallocations, and requested-byte traffic. The accepted exact/composed
cap cases remain green. The successful retention test still compares requested bytes/count against
the independent delivery/outcome layouts, keeps zero frees/reallocations while owners live, and
verifies matching off-render reclamation.

A mutation that calls `prepare_endpoint_queues` before the retained-cap check failed the exact
current-thread assertion:

```
assertion `left == right` failed
left: Counters { allocations: 11, deallocations: 11, reallocations: 0, requested_bytes: 23256 }
right: Counters { allocations: 0, deallocations: 0, reallocations: 0, requested_bytes: 0 }
```

The mutation was restored immediately.

## Bitwise PCM

The native-bank and forced-scalar endpoint/reference test maps both PCM arrays through `f32::to_bits`
before comparison, while retaining the scalar target/ramp/state, PostFader, and zero paired-witness
checks accepted by #579. The signed-zero discriminator maps `[+0.0]` and `[-0.0]` to distinct bits.
A mutation back to ordinary `f32` array inequality failed as required:

```
assertion `left != right` failed
left: [0.0]
right: [-0.0]
```

The mutation was restored.

## Gates

Debug host-core all-target tests pass: 15 unit tests and 14 endpoint integration tests, plus the
complete host-core integration suite. The endpoint integration suite passes in release mode.
Strict Clippy, formatting, diff checks, rustdoc, host/workspace policy, CI routing, and Wasm scalar
and `simd128` checks pass. Cargo.lock is restored and all changes remain uncommitted for root's
checkpoint and final synchronization.
