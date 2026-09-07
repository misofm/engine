# Issue 572 deterministic cancellation proof

Issue #572 preserves the accepted #571 runtime source at `d2e16082` and changes
only `crates/protocol/tests/delivery_ownership.rs`, this evidence, and the numbered
spec. No protocol runtime or public API file is changed.

The former zero/partial/full schedule used barriers that released render before the
control-side pre-ack assertion. It could legally observe an acknowledgement first,
and an assertion failure could strand the render worker at a later barrier.

The replacement uses bounded `sync_channel` rendezvous with sender-drop exit:

1. Render performs its selected zero, partial, or full pre-application and sends a
   fallible readiness result.
2. Control begins cancellation, receives readiness, and polls before sending the
   release signal. Render is blocked before `cancel_boundary`, so the assertion that
   poll returns `None` is deterministic.
3. Render performs `cancel_boundary`, sends its result and allocation counts, and
   exits. Control then checks the acknowledgement and retains the exact frontier,
   disposition, prefix, remainder, sample, delayed collection, and reuse assertions.

If either side exits early, the corresponding receiver or sender is dropped and the
other side returns instead of waiting on an unmatched barrier.

The ordering mutation moved the pre-ack assertion after the render completion
rendezvous. The focused test failed without hanging, exactly at:

```
assertion `left == right` failed
left: Some(CoreCancelComplete { token: CoreCancelToken { generation: 1 }, frontier: Some(2), acknowledged_sample: SampleTime(200) })
right: None
```

The mutation was restored. Final validation used the unchanged accepted #571 runtime
source and the corrected test schedule.
