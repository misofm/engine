# Connect BTLV automation ingress to scalar Point PCM

Status: numbered child of audit #349 IO5 and lane-B handoff #560 on delivered main `bf882a84ec630fb559690d010a65baf77dc45734`, after #572 / PR #574 and post-main qualification `34153096917`. This is the next partial-finding slice; no original open finding has started. Luna HIGH implements attempt 1, Astra LOW performs every adversarial scope/source/exact-head review, and the Sol HIGH coordinator owns the issue brief, checkpoints, artifact qualification/pinning, GitHub synchronization and delivery.

## Smallest closable outcome

Accept one actual schema-closed BTLV command frame through `ControllerAutomationDelivery`, decode it once with the existing `ProtocolController` command mapping, and dispatch it through that facade's existing delivery context. In the already delivered #532 combined endpoint, an encoded `AutomationEnqueue` Point batch must therefore reach the same controller-owned queue, scalar compressor application, PCM and native snapshot without the caller reconstructing a trusted typed `ControllerRequest`.

Add `ControllerAutomationDelivery::process_b1b_btlv(&mut self, input: &[u8], scratch: &mut DecodeScratch<'_>) -> Result<ControllerResponse, DecodeError>`. Factor the existing `ProtocolController::process_b1b_btlv` implementation into one crate-private contextual path: the ordinary public method passes no delivery context and preserves current behavior, while the facade supplies its existing `DeliveryContext`. Retain one decoder, one command mapping, one replay cache, one protocol queue set, one delivery state and one response/event sequence authority.

This method returns the existing complete `ControllerResponse`. It does not add or claim the separate `process_command_frame_into` malformed-header/output-reservation contract, C ABI or browser activation, automatic render scheduling or clock publication, live `ParameterStateGet`, structural-session adoption, graph binding, banks, other effects/parameters, or segment execution. IO5 remains partial after this slice.

## Frozen semantics and ownership

Use the controller's existing limited typed decoder and exact command construction. Preserve request ID, expected revision, canonical input bytes, decode errors, validation order, status mapping, cached complete response bytes and revision behavior. Exact-byte replay must return the cached response without a second admission or application. Reusing a request ID with different bytes must retain the existing refusal. Malformed or truncated input must return the existing typed decode error before any admission.

For `AutomationEnqueue`, the contextual path must call the delivered controller-owned automation service exactly once. A successful response may acknowledge only work retained by that service. Invalid revision, domain, handle, time or queue saturation must publish no accepted owner and no successful acknowledgment. Preserve the standing queue question: an acknowledgment can never precede a drop. Existing unsupported segment batches may be accepted as whole controller-owned work but must retain their delivered `PendingUnsupported` handoff and cancellation behavior; do not translate or partially execute them.

The facade restrictions remain frozen: `ParameterStateGet`, session transactions and transport locate return `Unavailable`; a pending cancellation retains the existing non-locate transport restriction. Metadata, counters, diagnostics, transport observation and telemetry retain their current provider behavior. The ordinary `ProtocolController::process_b1b_btlv` path must continue using its ordinary queue rather than silently acquiring a delivery context.

The render half, scalar endpoint, native compressor, caller PCM, caller-owned sample source and off-render lifecycle remain unchanged. Tests publish the actual endpoint `next_sample` through the existing fixture clock before later admission. Cancellation still requires the delivered boundary acknowledgment and terminal/event reconciliation before reclamation. No new queue, ledger, copy, decoder table, provider interface, allocator or dependency is permitted.

## Exact ownership

Allowed implementation paths:

- `crates/protocol/src/controller.rs`
- `crates/protocol/src/controller_delivery.rs`
- `crates/protocol/src/controller/tests.rs` only if a focused ordinary-path regression cannot live beside the contextual facade fixture
- `crates/host-core/tests/scalar_point_endpoint.rs`
- this numbered spec and bounded `artifacts/issue575-*` evidence

Do not edit `crates/protocol/src/delivery.rs`, `crates/protocol/src/lib.rs`, `crates/protocol/tests/delivery_ownership.rs`, queue algorithms, manifests/lockfiles, effect code, graph, hosts outside the existing host-core test, C ABI, browser, SDK, policies or workflows. The #572 delivery core and ownership regression are immutable prerequisites. Coordinate any newly discovered shared path with #559 before editing.

## Objective gates

1. **Real encoded Point-to-PCM.** Use #532's real compiled compressor session/provider/processor fixture and actual resolved Left/Right makeup handles. Encode an `AutomationEnqueue` command with the existing codec, process those exact bytes through the new facade method, hand off, and render at least one interior and one future/block-end asymmetric Point. PCM bits, native current/target state and endpoint snapshot must agree with the existing independent span-based native comparator.
2. **Replay identity.** Replaying the exact command bytes returns the exact cached response and creates no second outstanding owner or native application. The same request ID with different valid bytes returns the existing request-ID-reuse refusal without admission. A later new request ID uses the caller-published nonzero clock and rejects a past Point without ownership.
3. **Atomic rejection and saturation.** Representative truncated/malformed input returns its existing decode error. Wrong revision, invalid value and unknown handle return their existing non-OK controller response with no owner. With a deliberately full bounded automation service, another otherwise valid frame is refused as backpressure, outstanding/occupancy remain exact, and no success response exists for dropped work.
4. **Cancellation and unsupported work.** A real framed two-record batch applies prefix one, cancels at the actual endpoint boundary, emits the sole retained cancellation event, releases credit and preserves exact applied/canceled counts and native state. A framed unsupported segment retains whole-batch `PendingUnsupported` behavior and can be canceled without partial application.
5. **Facade limits and ordinary regression.** Framed `ParameterStateGet`, structural mutation and locate remain `Unavailable`. The ordinary non-facade `ProtocolController::process_b1b_btlv` automation path retains its current queue/replay behavior. Run #572's delivery-ownership regression unchanged against this base.
6. **Proportional quality.** Focused and complete affected protocol/host-core tests pass in debug and release as appropriate, including protocol `test-support`; strict affected Clippy/rustdoc, formatting, workspace/host-core/protocol policies and scalar plus simd128 Wasm compilation pass. No benchmark or performance claim is authorized.

Artifact delivery is identity-first. Build the ordinary six-file AudioWorklet artifact and compare every file, resource/PCM witness and source pin to the delivered artifact at `bf882a84`. If bytes are unchanged, retain the current qualified SHA-256 and cite that identity. Any drift requires one bounded root-owned probe, Astra LOW review, and explicit qualification before a pin change; implementation agents never pin artifacts.

## Stop and split triggers

Stop before widening this child if the implementation needs a delivery-core change, public export edit, a second decoder or command mapping, full response-buffer framing, provider production changes, live StateGet, automatic scheduling/clock/lifecycle publication, graph/effect/bank binding, segment translation, another fixture corpus, or a new allocation/benchmark framework. Preserve evidence and brief the independent outcome instead.

The slice is feasible as one private decoder/context extraction, one facade method and focused integration fixtures. A coherent passing tranche must pause for the root exact-path checkpoint before more implementation. Each attempt receives one implementation pass and one Astra LOW adversarial verdict; after three failed attempts, preserve evidence and rescope without weakening these gates.

## Numbered current-base scope review

Astra LOW reviewed the residual IO5 contract first at `ed1dbf87a611abe535af67ed7c237c04534ce1ce`, then rechecked the delivery-context seam after #572 merged at `bf882a84ec630fb559690d010a65baf77dc45734`. The merge changed only the delivery core, its exports, ownership tests and evidence; `controller.rs`, `controller_delivery.rs` and the scalar endpoint remained unchanged. `DeliveryContext` still supplies the same automation state and the facade still dispatches typed requests through `process_with_delivery_context`.

Scope verdict: **PASS**. The existing B1b decoder can be factored once so ordinary callers retain `None` and the facade supplies its current delivery context. This materially advances actual framed ingress into delivered scalar PCM without requiring a cancellation-core change or a second command mapping. The allowed paths and gates above are the frozen brief. Astra LOW remains appropriate because production work is control-plane decoding/context plumbing; existing scalar DSP and render algorithms remain byte-identical. Implementation may begin only from clean integrated base `bf882a84` after its successful post-main run `34153096917`.

## Attempt 1 source verdict

Luna HIGH delivered the single contextual decoder extraction and facade entry point at production checkpoint `7d50c93615cc553a9e57198261c370fc925397ed`. Later checkpoints `e6906954` and `4307c0dc` add only the real scalar PCM and protocol adversarial fixtures. Consolidated evidence is pushed at `fa84469c43b88fa7664806c332aa363d9e6a014c` under `artifacts/issue575-attempt1/`; root verified its 166-entry SHA-256 manifest and every credited exit. The retained malformed Clippy wrapper exited 127 before Cargo ran and is explicitly non-credit; the corrected strict command passed.

Astra LOW returned **PASS** for source acceptance at exact head `fa84469c`. The implementation retains one decoder and command mapping, the ordinary path supplies no delivery context, and the facade supplies its existing delivery state. Replay, revision/status precedence, atomic saturation, and the ack-before-drop rule remain intact. Real encoded Points reach the delivered scalar PCM/native snapshot path; exact replay, changed-byte reuse, truncation, past time, wrong revision/value/handle, saturation, prefix cancellation, unsupported whole-batch cancellation, fixed facade restrictions, the ordinary queue path, and #572 ownership are discriminated. Protocol test-support passes 169 tests per profile; host-core default passes 72 with 2 ignored and control-provider passes 88 with 2 ignored per profile; strict lint/doc/format/policy/conformance and scalar/simd128 Wasm gates pass.

Source acceptance does not complete artifact delivery. Astra LOW authorized exactly one ordinary identity probe with `MISO_ENGINE_WEB_AUDIOWORKLET_REPIN=1`. If all six bytes are unchanged, root may verify affected consumers and reuse the delivered browser evidence with its original attribution. Any drift must be preserved and reviewed before bounded detached qualification; no pin change is authorized by this verdict. IO5 remains partial after #575.

## Artifact and delivery decision

Root ran the single authorized identity probe at documentation-only head `ec992089d1fa1208ceb49ff6ca65e6e2c05bf2ed`. It returned the existing qualified Wasm SHA-256 `0d447edfd651bd1292ffbce81ec8923a8b20c063722b7bfacaf073e3fa36c357`, left the output directory empty as designed, and changed no repository pin or lineage file. No second probe or repin occurred.

The corrected ordinary build produced exactly six files. Every file is byte-identical to the retained #570 qualified artifact: ABI layout, host declaration, host JavaScript, AudioWorklet JavaScript, simd128 Wasm and parameter metadata. Static/object/ABI checks, browser expected resources with 26 red mutations, generated matrix consistency, SDK installation and SDK packaging/self-test all pass. Resource and PCM expectations remain frozen. No browser was launched for #575; the identical bytes reuse #570's Chromium `151.0.7922.34`, Firefox `153.0` and WebKit `26.5` qualification under its original candidate `3871e137b519815540c1b3abcd6cfcec7932efa7`.

The first ordinary-build capture exited 2 before compilation because root's capture helper omitted the required pre-existing output directory. That non-credit preflight failure is preserved; the corrected build is the sole credited ordinary build. Astra's first artifact review accepted every artifact fact but returned delivery FAIL because eight generated command records and one post-context line carried trailing spaces. Their exact original bytes were losslessly packaged as deterministic gzip files, manifests were regenerated, and no command was repeated for that correction.

Astra LOW returned **PASS** for artifact and delivery evidence at exact clean pushed head `585c1de93b7259812ca6b8f0ace1766c7905e2e4`. The 7-entry probe and 44-entry identity manifests verify, all nine compressed records reproduce the originals, and `git diff --check origin/main...HEAD` passes. Source, artifact bytes, pin and lineage remain unchanged. Source and local immutable delivery gates are accepted; exact final head/current-base review, required CI, guarded merge, post-main qualification, GitHub closure and worktree cleanup remain pending. IO5 retains live StateGet, full output-buffer/CAPI/browser activation, automatic clock/lifecycle, graph/bank/effect/parameter and segment-execution successors.
