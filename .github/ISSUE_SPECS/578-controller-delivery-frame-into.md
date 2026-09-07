# Expose caller-buffer framing on automation delivery facade

Status: numbered child #578 of audit #349 IO5 and lane-B handoff #560, based on delivered main `254b9d86d720da8463739a407b055b4a7596f001` after #575 / PR #577 and post-main qualification `34157315893`. This continues one of the eight original partial findings; it does not start an original open finding. Sol HIGH coordinates and owns checkpoints, artifact qualification, GitHub synchronization and delivery. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, artifact and exact-head/current-base verification assignment under the user's latest routing; historical model provenance is unchanged.

## Smallest closable outcome

Expose the existing complete command-frame caller-buffer contract through `ControllerAutomationDelivery` so one actual encoded `AutomationEnqueue` Point frame can enter through the same full framing boundary used by ordinary `ProtocolController`, dispatch through the facade's existing delivery context, and reach the already delivered scalar compressor PCM/native snapshot endpoint.

Add `ControllerAutomationDelivery::process_command_frame_into(&mut self, input: &[u8], scratch: &mut DecodeScratch<'_>, output: &mut [u8]) -> Result<usize, CommandFrameProcessError>`. Factor only the controller's existing caller-buffer machinery needed to supply an optional `DeliveryContext`. The ordinary public controller method must preserve its current structural planning, provider behavior, replay, response encoding and errors. The facade must apply its fixed restrictions before any structural preparation or commit path can mutate provider/session state.

Retain one codec and command mapping, one replay cache, one protocol queue set, one delivery state, one event/response sequence authority and one scalar endpoint. This child adds no second frame encoder, decoder, queue, ledger, provider or lifecycle owner.

This is a Rust facade seam only. It does not activate C ABI or browser hosts, publish automatic render clocks or lifecycle, make `ParameterStateGet` live, adopt structural session mutations, bind a graph or bank, add effects/parameters, or execute automation segments. IO5 remains partial after this slice.

## Frozen semantics

Preserve the existing `ProtocolController::process_command_frame_into` distinctions:

- a malformed outer header is `CommandFrameProcessError::Uncorrelatable` because no safe request identity exists;
- a malformed payload after a valid complete header produces the existing correlatable non-OK response frame;
- a new request proves caller output capacity against `maximum_cached_response_bytes` before replay admission, provider mutation, queue admission or event publication;
- an exact cached replay copies only its already-known response length and does not dispatch or admit again;
- request-ID reuse, expired replay and replay backpressure retain their existing uncached status frames and precedence;
- insufficient output for a cached response retains the cached request and all ownership so a later adequate retry returns the original exact bytes.

For a new facade automation request, decode and map once, then call the delivered controller-owned automation service exactly once through the existing context. A successful response may acknowledge only retained work. Invalid revision/domain/handle/time, queue saturation or output-reservation failure publishes no accepted owner and no success acknowledgment. The standing review question remains mandatory: an acknowledgment can never precede a drop.

The facade continues to refuse `ParameterStateGet`, session transactions and transport locate as `Unavailable`. A pending cancellation retains its existing non-locate transport restriction. These refusals must occur without structural planning/commit side effects, provider mutation, revision change or events. Unsupported segment batches remain whole `PendingUnsupported` work after successful admission and remain cancellable without partial application.

The ordinary controller's complete framing and #575 B1b facade methods remain independent regressions. The scalar render half, native compressor arithmetic, caller PCM, explicit test clock publication, cancellation boundary and terminal collection remain unchanged.

## Exact ownership

Allowed implementation and focused test paths:

- `crates/protocol/src/controller.rs`
- `crates/protocol/src/controller_delivery.rs`
- `crates/protocol/src/controller/tests.rs`
- `crates/host-core/tests/scalar_point_endpoint.rs`
- this numbered spec and bounded `artifacts/issue578-*` evidence

Excluded paths include:

- `crates/protocol/src/delivery.rs`, `crates/protocol/src/lib.rs` and `crates/protocol/tests/delivery_ownership.rs`;
- queue algorithms and provider production implementations;
- every production `crates/host-core` file;
- C ABI, browser, SDK, graph, effect, bank and session production code;
- manifests, lockfiles, policies, workflows and generic fixture frameworks.

Lane A #576 exclusively owns `crates/host-core/src/builtin_batch_endpoint.rs`, minimal `crates/host-core/src/lib.rs` exports, `crates/host-core/tests/builtin_batch_endpoint.rs`, its numbered spec and focused evidence. This child must not edit or depend on those active paths. Any newly discovered shared path requires a fresh #559 ownership check before editing.

## Objective gates

1. **New-request reservation.** Exercise zero, one-below and exact `maximum_cached_response_bytes` output capacity. A short output admits no request, consumes no replay slot, mutates no provider or delivery owner, changes no revision and emits no event. Retrying exact capacity succeeds once and returns one canonical response.
2. **Cached replay and reuse.** Exact replay into an actual response-sized output returns the original bytes without another decode, admission or scalar application. One byte below the cached response length returns the existing encode error while retaining replay and ownership; an adequate retry still returns the original bytes. Changed valid bytes under the same request ID retain the existing reuse refusal.
3. **Error identity and precedence.** Distinguish a malformed outer header from a correlatable malformed payload. Verify the latter's request/message/revision/status frame and no side effect. Preserve ordinary output reservation, replay and decode precedence.
4. **Real Point-to-PCM.** Send real encoded asymmetric interior and future/block-end Points through the new full facade method into #532's compiled compressor/provider/processor fixture. PCM bits, native current/target state and endpoint snapshot agree with the existing independent span comparator. Exact replay does not apply twice.
5. **Ownership under refusal and cancellation.** Wrong revision/value/handle, past time and bounded saturation admit nothing and never return a success acknowledgment for dropped work. A two-record Point batch applies one prefix then cancels at the actual boundary with exact event, credit and state reconciliation. An unsupported segment batch stays whole `PendingUnsupported` and cancels without partial execution.
6. **Facade restrictions and regressions.** Full-frame StateGet, session edit and locate stay `Unavailable` without structural/provider effects. Ordinary `ProtocolController::process_command_frame_into`, facade B1b ingress and #572 delivery-ownership tests pass unchanged. A pending cancellation preserves its transport rule.
7. **Proportional quality.** Focused and complete affected protocol/host-core suites pass in debug and release, including protocol `test-support`; strict affected Clippy/rustdoc, formatting, workspace/host-core/protocol policy and scalar plus simd128 Wasm compilation pass. No benchmark, performance claim or whole-method allocation-free claim is authorized.

Artifact delivery is identity-first. Root builds the ordinary six-file AudioWorklet artifact and compares every file, resource/PCM witness and pin to delivered main. Unchanged bytes retain pin `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357` and reuse the original #570 browser attribution. Drift requires a bounded root-owned probe and Astra LOW review before any detached qualification or pin change; implementation agents never pin artifacts.

## Stop and split triggers

Stop before widening this child if it needs another decoder or command mapping, another queue/ledger, a delivery-core/export change, provider publication, a shared host-core production edit, structural adoption, CAPI/browser activation, automatic clock/lifecycle, graph/effect/bank work, segment translation, a new fixture corpus or a new allocation/benchmark framework. Preserve the checkpoint and brief the independent outcome.

The slice is one control-plane extraction plus focused existing-fixture extensions. A coherent passing implementation tranche pauses for the root exact-path checkpoint before more work. Each attempt receives one Luna implementation pass and one Astra LOW adversarial verdict. After three failed attempts, preserve evidence and rescope without weakening gates.

## Preliminary residual scope audit

After #575 merged, Astra LOW compared caller-buffer framing, live `ParameterStateGet`, full CAPI/browser activation and automatic clock/lifecycle publication on current main `254b9d86`. Caller-buffer framing is the smallest useful successor because `ProtocolController` already owns output reservation, correlatable error encoding and exact replay, while #575 already supplied the delivery-context B1b seam. Live state requires coherent renderer-state publication; CAPI/browser activation requires host session/provider/replacement integration; automatic clock/lifecycle requires cancellation, quiescence and association rules. Those are separate children.

The audit returned conditional PASS for this exact product shape and judged Astra LOW sufficient because it changes bounded control-plane dispatch/encoding rather than render arithmetic. Activation still requires this numbered brief and GitHub issue to be synchronized, current-base and #576 ownership to be rechecked, and Astra LOW to review this committed brief. No implementation is authorized by the preliminary audit alone.
