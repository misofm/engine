# Pair the fader and matrix banks under concurrent control delivery

Found by the #1750 implementer (2026-10-22), checked by root.

## Problem

The fused fader and matrix stage (#466; its settled pass #881 and its ramping pass #890) forms only when both banks use `BuiltinControlDelivery::BetweenRenderCalls` (`make_fader_matrix`, `crates/builtins-compiler/src/lib.rs` near `:5505-5525`). No product host prepares that delivery:

- the C ABI prepares through `host_core::prepare_host_runtime_with_live_lanes(_successor)` (`crates/capi/src/runtime/mod.rs` near `:103-106`), which passes `between_render_calls = false` (`crates/host-core/src/prepare.rs` near `:1666`);
- the browser's `prepare_web_host` calls no `_between_render_calls` entry since #1400b (13fd9b491; `hosts/host-web/src/lib.rs` near `:5545-5550`).

So every product plan keeps the fader and the matrix as two stages, and #881's and #890's fused passes run only in tests and in the bench's metered row. The metered row's doc (`tools/bench/src/console.rs` near `:103-118`) and its prepare comment (`tools/console-workload/src/lib.rs` near `:1376-1383`) still say the browser fuses; that is stale.

The rule came from #444: under the September control queues, a fader or matrix record could arrive between the two stages' drains (`Consumer::try_pop` refreshed the producer cursor while draining), so one traversal could apply a record to one stage and not the other. Decision 15 replaced that delivery: live values come from one committed snapshot per block (`apply_latest(snapshot, ...)`), and every live value of one committed transaction takes effect in the same block (D15-2).

## Smallest closable slice

1. **Diagnose (record in this spec first).** Under today's concurrent delivery, can a fused traversal observe a different set of fader, mute or matrix values than the two separate stages would in the same block? Trace both stages' control reads for one block on both hosts: what they read, when, and from which snapshot or queue. Answer with file:line evidence. Also answer the acked-batch question: can an ack ever precede a drop on the fused path?
2. **If the answer is "no":** let `make_fader_matrix` pair `Concurrent` banks, with the control reads of both banks taken at the same block boundary the separate stages use today. Move the bench's metered row to the product preparation and correct the stale comments.
3. **If the answer is "yes":** stop and record the hazard. Root then decides between a delivery change and removing the BetweenRenderCalls-only fused stage.

## Authorized paths

- `crates/builtins-compiler/src/lib.rs` (pairing and its tests), `crates/host-core/src/prepare.rs` (only if a prepare entry must change)
- `tools/console-workload/src/lib.rs`, `tools/bench/src/console.rs` (the metered row's preparation and docs)
- this spec

## Objective gates

1. The diagnosis, with evidence, recorded before any code change.
2. A builtins-compiler test: on a Concurrent plan, the paired stage renders PCM and state bit-identical to the separate stages, with live fader, mute and matrix values committed between blocks, across a ramp start, middle and end.
3. host-core and host-web control parity tests pass unchanged (`hosts/host-web/tests/control_parity.rs`); render allocates nothing; `tools/realtime-policy` passes.
4. One descriptive run of the fader-ride row (#1750) before and after.

## Dependencies

#1750 (the baseline row on the product preparation).
