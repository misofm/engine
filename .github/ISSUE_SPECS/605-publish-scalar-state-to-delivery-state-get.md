# Publish scalar makeup state to delivery StateGet

Status: proposed IO5 child of audit #349 and lane-B handoff #560, based on delivered main `6fe8676e1537bc2c952ac87ee2fe31c545438474` after #598 / PR #604 and successful post-main qualification `34189229062`. This advances the last lane-B original partial finding; it does not start an original open finding. Lane-A #603 is the only other active issue and owns disjoint native capture scripts/evidence. Sol HIGH coordinates, owns checkpoints, GitHub synchronization, delivery, and any AudioWorklet artifact qualification/pinning. Luna HIGH or XHIGH implements. Astra LOW performs every scope, source, artifact-applicability, and exact-head/current-base verification.

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

`crates/host-core/src/control_provider.rs` may change only if the exact-head scope review proves the fixed inline facade slot cannot preserve ordinary provider behavior; any such amendment must stay allocation-free and two-handle-specific and must be recorded before Luna edits it.

Exclude delivery-core/queue/wire/schema changes, other host-core endpoints/providers, C ABI and browser hosts, SDK/generated surfaces, graph/session/compiler/effect implementations, banks, other effects or parameters, segment execution, manifests/lockfiles, policies/workflows, #603 capture paths/evidence, AudioWorklet artifacts/pins, and timed benchmarks. The `ParameterStatePage` wire schema and flag values are frozen.

## Objective gates

1. **Actual state and time.** Real asymmetric admitted makeup Points pass through typed, B1b, and caller-buffer ingress to PCM/native state. After explicit publication at a quiescent boundary, each ingress's StateGet returns exact current-value bits, correct valid/automation-active flags, requested handle order, and the scalar snapshot's `observed_sample`. It never substitutes `next_sample` or a newer provider clock.
2. **Publication coherence.** A complete publication replaces both records together without allocation. Wrong/reversed/duplicate/zero handles, invalid flags, nonfinite values, or other malformed publication reject atomically and leave the last accepted page byte-identical. Re-publishing the same snapshot is idempotent. A later valid snapshot replaces both records and sample together.
3. **Unavailable and request behavior.** Before first publication and on ordinary/non-opted-in delivery facades, typed, B1b, and caller-buffer StateGet stay `Unavailable`. Unknown handles retain the existing typed not-found status, requested order is exact, and existing request cardinality/schema checks remain authoritative. The ordinary controller's provider-backed StateGet is unchanged.
4. **Lifecycle truth.** Cancellation-only progress cannot advance published `observed_sample` or claim newly observed DSP. Publication neither advances endpoint/provider time nor performs render work. Faulted snapshots retain their actual last observed sample and state. No automatic scheduling, cross-thread synchronization, or host lifecycle claim is introduced.
5. **Delivery and replay law.** State reads/publication do not admit, hand off, collect, cancel, consume credit, alter resident/outstanding counts, or emit reliable events. Typed/B1b/caller-buffer revision and error precedence, exact replay, changed-byte request-ID reuse, zero/short/exact output reservation, and the rule that an acknowledgment cannot precede a drop remain discriminated.
6. **Realtime and resource invariants.** Render source/arithmetic and the prepared render owner are unchanged. Existing scalar endpoint allocation/syscall/realtime gates pass; publication and reads are control-side. Resource reporting includes any inline slot exactly and adds no heap allocation after preparation.
7. **Proportional qualification.** Focused protocol and host-core debug/release tests, unchanged #460 ownership and #528/#532/#575/#578 regressions, strict affected Clippy/rustdoc, supported scalar/SIMD Wasm checks, formatting/diff, workspace/protocol-control/host-core/realtime policies and mutation suites pass. No timed benchmark receives credit.

## Artifact qualification and delivery

After source PASS, root runs the ordinary six-file AudioWorklet identity probe against the current delivered pin. Byte identity permits retained-artifact qualification with the existing attribution. Any drift requires a separately authorized scratch build/ABI/resources/PCM and browser qualification before root changes a pin or generated consumer. Luna and lane A must not generate, pin, or qualify the shipped artifact.

Root records each coherent checkpoint, pushes promptly, and synchronizes this spec and GitHub issue. Astra LOW must pass the exact pushed source and final decision-record head against current main before PR creation. Recheck live head/base/merge-base, #603 path ownership, required PR qualification, guarded merge, post-main qualification, GitHub closure, and clean worktree removal. IO5 remains partial after this child until a post-delivery Astra LOW residual audit determines the next bounded obligation or closure.

## Stop and split triggers

Stop before adding automatic render clocks, a mailbox/lock/atomic snapshot transport, host/CAPI/browser activation, arbitrary provider mutation, a generic parameter-state framework, more than two handles, graph/bank/effect/parameter expansion, automation segments, structural session mutation, another queue/ledger/codec, schema changes, benchmark machinery, or #603 paths. Preserve a useful checkpoint and brief a separate successor. One Luna HIGH or XHIGH attempt receives one Astra LOW adversarial verdict; after three failed attempts, stop and rebrief without weakening gates.

## Preliminary residual audit

Astra LOW returned **PASS** on delivered main `6fe8676e1537bc2c952ac87ee2fe31c545438474`. `ScalarPointSnapshot` already carries actual Left/Right current/target state plus distinct observed and next samples; `SessionControlProvider` still answers from its prepared state catalog and current sample source; and the delivery-context controller branch explicitly refuses StateGet. #575 and #578 already deliver B1b and complete caller-buffer ingress, so older documentation calling framed ingress unavailable is historical rather than a remaining implementation requirement.

Explicit two-handle quiescent publication is the smallest useful successor. Host activation spans resource and lifecycle contracts; automatic publication requires synchronization; graph/bank/effect/parameter/segment rollout adds separate execution semantics. This child is disjoint from #603, advances original partial IO5, and starts no original open finding. Preserve #370's deferral attribution, #460 ownership, #524/#528/#530/#532 execution lineage, #575/#578 ingress, and the #542 to #567 plus #552/#558 delivery order. Exact numbered current-base scope review remains required before implementation.
