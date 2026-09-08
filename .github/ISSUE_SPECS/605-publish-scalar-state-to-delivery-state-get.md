# Publish scalar makeup state to delivery StateGet

Status: proposed IO5 child of audit #349 and lane-B handoff #560, based on delivered main `6fe8676e1537bc2c952ac87ee2fe31c545438474` after #598 / PR #604 and successful post-main qualification `34189229062`. This advances the last lane-B original partial finding; it does not start an original open finding. Lane-A #606 is the only other active issue and owns disjoint native capture scripts/evidence. Sol HIGH coordinates, owns checkpoints, GitHub synchronization, delivery, and any AudioWorklet artifact qualification/pinning. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, artifact-applicability, and exact-head/current-base verification.

## Smallest closable outcome

Make the already delivered scalar compressor makeup endpoint's actual native state explicitly publishable to `ControllerAutomationDelivery` at a caller-established quiescent boundary. After publication, typed, B1b, and caller-buffer `ParameterStateGet` requests through that same delivery facade return the two prepared Left/Right makeup handles from one fixed two-record snapshot. Before publication, and for every ordinary delivery facade that did not opt into this exact state surface, `ParameterStateGet` remains `Unavailable`.

The published page uses `ScalarPointSnapshot.observed_sample`, never `next_sample` or an independently sampled provider clock. Each record's `value` is the corresponding native `current_value`; the valid flag is set, and the automation-active flag is set exactly when the native current and target bit patterns differ. Requested handle order is preserved. This slice may expose both handles or any nonempty request subset/ordering made only from those two handles; an unknown, zero, duplicate, or over-limit request retains the existing typed protocol disposition. Publication is an explicit control-side action after the caller has established that no render call can mutate the snapshot.

This child reuses the single delivered controller, codec, replay cache, protocol queues, delivery ledger, reliable sequence authority, scalar endpoint, and native state read. It adds no render-to-control mailbox, lock, automatic clock publication, second provider catalog, or second automation model.

## Frozen opt-in boundary

The protocol delivery facade may retain one inline, allocation-free state slot with exactly two configured nonzero distinct handles. Existing `ControllerAutomationDelivery::prepare` behavior stays state-disabled. Add one narrowly named opt-in preparation path used only by `prepare_controller_scalar_point_endpoint`, and one explicit publication operation accepting an observed sample plus exactly two fixed-width records. The slot begins unpublished. Publication validates exact configured handle identity/order, valid finite values, and legal state flags before atomically replacing the complete inline snapshot; a rejected publication changes nothing.

The delivery-context `ParameterStateGet` branch may read only this slot. Disabled or unpublished slots return `Unavailable`; a request containing a handle outside the configured pair returns the existing `NotFound` provider disposition. The ordinary `ProtocolController` continues to dispatch state reads to its `ControlProvider` exactly as today. Do not expose mutable provider access or a generic publication callback.

Host-core owns the translation from `ScalarPointSnapshot` to the two protocol records. Its publication helper requires the exact prepared handles, copies `observed_sample`, maps `current_value` and derives the automation-active flag from current/target bit inequality. It must not read the caller's `PlanSampleSource`, advance any clock, hand off or collect delivery work, or claim that a cancellation-only boundary observed DSP.

If the implementation cannot preserve this exact fixed two-record, explicit-publication shape without a generic provider framework or concurrent transport, stop and split before editing.

## Exact ownership

Allowed production and test paths are:

- `crates/protocol/src/controller.rs`
- `crates/protocol/src/controller_delivery.rs`
- `crates/host-core/src/scalar_point_endpoint.rs`
- `crates/host-core/tests/scalar_point_endpoint.rs`
- this numbered spec and bounded issue evidence

Exclude `crates/host-core/src/control_provider.rs`, delivery-core/queue/wire/schema changes, other host-core endpoints/providers, C ABI and browser hosts, SDK/generated surfaces, graph/session/compiler/effect implementations, banks, other effects or parameters, segment execution, manifests/lockfiles, policies/workflows, #606 capture paths/evidence, AudioWorklet artifacts/pins, and timed benchmarks. The `ParameterStatePage` wire schema and flag values are frozen.

## Objective gates

1. **Actual state and time.** Real asymmetric admitted makeup Points pass through typed, B1b, and caller-buffer ingress to PCM/native state. After explicit publication at a quiescent boundary, each ingress's StateGet returns exact current-value bits, correct valid/automation-active flags, requested handle order, and the scalar snapshot's `observed_sample`. It never substitutes `next_sample` or a newer provider clock.
2. **Publication coherence.** A complete publication replaces both records together without allocation. Wrong/reversed/duplicate/zero handles, invalid flags, nonfinite values, or other malformed publication reject atomically and leave the last accepted page byte-identical. Re-publishing the same snapshot is idempotent. A later valid snapshot replaces both records and sample together.
3. **Unavailable and request behavior.** Before first publication and on ordinary/non-opted-in delivery facades, typed, B1b, and caller-buffer StateGet stay `Unavailable`. Unknown handles retain the existing typed not-found status, requested order is exact, and existing request cardinality/schema checks remain authoritative. The ordinary controller's provider-backed StateGet is unchanged.
4. **Lifecycle truth.** Cancellation-only progress cannot advance published `observed_sample` or claim newly observed DSP. Publication neither advances endpoint/provider time nor performs render work. A sticky fault makes `snapshot()` return the existing error. It produces no new publishable snapshot and leaves the last successfully published records and observed sample unchanged. Tests must not manufacture or publish a new successful snapshot from a faulted owner. No automatic scheduling, cross-thread synchronization, or host lifecycle claim is introduced.
5. **Delivery and replay law.** State reads/publication do not admit, hand off, collect, cancel, consume credit, alter resident/outstanding counts, or emit reliable events. Typed/B1b/caller-buffer revision and error precedence, exact replay, changed-byte request-ID reuse, zero/short/exact output reservation, and the rule that an acknowledgment cannot precede a drop remain discriminated.
6. **Realtime and resource invariants.** Render source/arithmetic and the prepared render owner are unchanged. Existing scalar endpoint allocation/syscall/realtime gates pass; publication and reads are control-side. The inline publication slot and publication operation allocate nothing. StateGet retains the existing bounded control-side `ParameterStatePage`/response/replay allocation behavior; no allocation-free read claim is made. Resource reporting includes the slot's exact inline size.
7. **Proportional qualification.** Focused protocol and host-core debug/release tests, unchanged #460 ownership and #528/#532/#575/#578 regressions, strict affected Clippy/rustdoc, supported scalar/SIMD Wasm checks, formatting/diff, workspace/protocol-control/host-core/realtime policies and mutation suites pass. No timed benchmark receives credit.

## Artifact qualification and delivery

After source PASS, root runs the ordinary six-file AudioWorklet identity probe against the current delivered pin. Byte identity permits retained-artifact qualification with the existing attribution. Any drift requires a separately authorized scratch build/ABI/resources/PCM and browser qualification before root changes a pin or generated consumer. Luna and lane A must not generate, pin, or qualify the shipped artifact.

Root records each coherent checkpoint, pushes promptly, and synchronizes this spec and GitHub issue. Astra LOW must pass the exact pushed source and final decision-record head against current main before PR creation. Recheck live head/base/merge-base, #606 path ownership, required PR qualification, guarded merge, post-main qualification, GitHub closure, and clean worktree removal. IO5 remains partial after this child until a post-delivery Astra LOW residual audit determines the next bounded obligation or closure.

## Stop and split triggers

Stop before adding automatic render clocks, a mailbox/lock/atomic snapshot transport, host/CAPI/browser activation, arbitrary provider mutation, a generic parameter-state framework, more than two handles, graph/bank/effect/parameter expansion, automation segments, structural session mutation, another queue/ledger/codec, schema changes, benchmark machinery, or #606 paths. Preserve a useful checkpoint and brief a separate successor. One Luna HIGH or XHIGH attempt receives one Astra LOW adversarial verdict; after three failed attempts, stop and rebrief without weakening gates.

## Preliminary residual audit

Astra LOW returned **PASS** on delivered main `6fe8676e1537bc2c952ac87ee2fe31c545438474`. `ScalarPointSnapshot` already carries actual Left/Right current/target state plus distinct observed and next samples; `SessionControlProvider` still answers from its prepared state catalog and current sample source; and the delivery-context controller branch explicitly refuses StateGet. #575 and #578 already deliver B1b and complete caller-buffer ingress, so older documentation calling framed ingress unavailable is historical rather than a remaining implementation requirement.

Explicit two-handle quiescent publication is the smallest useful successor. Host activation spans resource and lifecycle contracts; automatic publication requires synchronization; graph/bank/effect/parameter/segment rollout adds separate execution semantics. This child is disjoint from #606, advances original partial IO5, and starts no original open finding. Preserve #370's deferral attribution, #460 ownership, #524/#528/#530/#532 execution lineage, #575/#578 ingress, and the #542 to #567 plus #552/#558 delivery order. Exact numbered current-base scope review remains required before implementation.

## Numbered scope verdict 1

Astra LOW returned **FAIL** at exact clean head/upstream `a4103d878b0f8da60c2aaeea95e605a748e816ec` and current main/merge-base `6fe8676e1537bc2c952ac87ee2fe31c545438474`. GitHub #605 had exact open identity/body and the fixed two-record inline design was otherwise sound, but the brief incorrectly banned all post-preparation heap allocation despite the existing Vec-backed StateGet response, described a successful faulted snapshot even though `snapshot()` returns the sticky error, and expanded the feature SHA incorrectly in #560. #603 also closed after its bounded correction failed.

This correction narrows the allocation claim to the inline publication slot and operation, preserves existing bounded control-side StateGet response/replay allocation, states the real sticky-fault behavior, excludes `control_provider.rs`, and requires tracker synchronization to the exact feature head and stopped #603 state. Luna remains unauthorized until Astra LOW passes the corrected pushed scope.

## Numbered scope verdict 2

Astra LOW returned **FAIL** at exact clean head/upstream `28d3692841c0278c1e9748a65db0d6e3efbe6d20`. The allocation, sticky-fault, provider exclusion, feature SHA and tracker state were corrected, but the opening status and several live ownership references still named stopped #603 instead of active replacement #606. This correction changes current path ownership and live merge checks to #606 while preserving #603 only as historical failure provenance. Luna remains unauthorized until Astra LOW confirms this exact correction.

## Corrected numbered scope PASS

Astra LOW returned **PASS** at exact clean head/upstream `f50253ab6a7e03b371f922e81b46f148ef0ae23b`, current main/merge-base `6fe8676e1537bc2c952ac87ee2fe31c545438474`, and pushed tracker `11d3241724f317d530190f44c2ab5bd92e1010d4`. GitHub #605 matches this spec, all current coordination/exclusion/merge-check references name active #606, and #603 remains only historical. The allocation, sticky-fault and provider-exclusion corrections are accepted; the fixed inline two-record design is sound and disjoint from #606.

Luna HIGH or XHIGH attempt 1 is authorized within the four listed source/test paths plus this spec/evidence. `crates/host-core/src/control_provider.rs`, #606 paths and artifact work remain excluded. Root may perform the ordinary artifact identity probe only after Astra LOW source PASS.

## Attempt 1 implementation checkpoint

Luna HIGH delivered source checkpoint `6aa9f2fe` within exactly the four authorized source/test paths. `ControllerAutomationDelivery` retains an optional inline two-record slot: ordinary preparation leaves it disabled, while the combined scalar endpoint configures the exact two handles. Explicit publication validates both records before replacing the complete snapshot. Typed, B1b and caller-buffer execution read the same slot; disabled/unpublished returns `Unavailable`, unknown handles return `NotFound`, requested order is preserved, and ordinary provider-backed controller reads remain unchanged.

Host-core translates `ScalarPointSnapshot.observed_sample`, current-value bits and current/target bit inequality into the frozen state records. The focused real asymmetric Point-to-PCM fixture publishes and reads that state through both encoded ingress modes and exercises atomic reversed-handle rejection. Luna reported host-core scalar endpoint tests passing 9/9 in debug and release, protocol library tests passing 160/160, affected Clippy, formatting and diff checks passing. `Cargo.lock` and the briefly touched protocol export were restored before checkpoint; the committed path set is exact. No artifact, browser, listening or timed work ran. Astra LOW attempt-1 source review is required before any artifact identity probe.

## Attempt 1 source verdict

Astra LOW returned **FAIL** at exact clean head/upstream `56417a480a58b6a1de06f48380fe4c0fd88b1e9b`, source `6aa9f2fe9d08eb990f3e8815da546e6b0931db8c`, and base `6fe8676e1537bc2c952ac87ee2fe31c545438474`. The public opt-in constructor accepts zero or duplicate handles; public publication returns prose errors; and the focused additions do not yet discriminate typed success, unpublished/default-disabled behavior, subset/order/unknown requests, readback after rejected publication, replacement/idempotence, both flag outcomes, observed-versus-next time, cancellation/fault preservation, replay across publication, changed-byte/output precedence, delivery-credit/event invariance, or allocation-free publication.

Protocol library tests passed 160/160 and feature-enabled scalar endpoint tests passed 9/9 in debug and release; formatting and diff checks passed. Featureless filtered invocations ran no tests and receive no credit. Astra's Cargo invocations reordered two `Cargo.lock` lines; root restored that generated excluded delta, leaving the pushed worktree clean. Attempt 2 is authorized only for the public pair validation, accessible typed errors and missing publication-specific discriminators within the existing four paths. Any new export requires a prior explicit scope amendment; artifact work remains locked until source PASS.

## Attempt 2 implementation checkpoint

Luna HIGH delivered correction `b5c1a318` within three authorized paths. The existing exported `ControllerAutomationPrepareError` now distinguishes invalid opt-in bindings, a disabled publication slot, and invalid publication records; no export file changed. The public opt-in constructor rejects zero or duplicate handles before preparation. Compact protocol and host-core fixtures cover default-disabled and unpublished state, typed plus both encoded ingress paths, request ordering/unknown handles, rejected-publication preservation, idempotent and replacement publication, both flag states, observed sample, cancellation preservation, exact replay across later publication, changed-byte request-ID reuse, short caller output, delivery ownership invariants, and a thread allocation counter around publication.

Luna reported protocol library tests passing 161/161, scalar endpoint tests passing 9/9 in debug and release, strict affected Clippy, formatting and diff checks passing. `Cargo.lock` was restored and only the three authorized paths were committed. No artifact, browser, listening or timed work ran. Astra LOW attempt-2 source review must verify that the new controls causally prove the full gate, including installed allocation counting, observed-versus-next time and sticky-fault preservation.
