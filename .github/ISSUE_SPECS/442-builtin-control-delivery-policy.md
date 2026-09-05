# Carry explicit between-render builtin control delivery into prepared bank ownership

This is the smallest policy-plumbing prerequisite for #430 under audit #349, based on delivered main `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`. Planning only until Astra approves this numbered stateless scope.

## Product contract

Carry an immutable, unversioned builtin control-delivery policy from host preparation to concrete prepared bank owners. Existing general preparation APIs default to Concurrent. A dedicated preparation entry lets WebEngine declare BetweenRenderCalls because it privately retains the producer endpoints and exclusively owns both submit_commands and render_next. Only that proven production host opts in. This is an explicit caller contract, not runtime enforcement for arbitrary external hosts. Do not infer it from backend, queue emptiness or observed use.

## Scope and exclusions

Limit edits to graph prepared-builtin metadata in `crates/graph/src/lib.rs`, builtins-compiler preparation/session/bank owners, host-core preparation and host-web private preparation, their existing focused tests and this spec. Permit only necessary concrete-owner layout mirror corrections in `crates/capi/tests/resource_lifecycle.rs`. GraphCompiler already moves the prepared session and needs no new policy argument or second source of truth. The exact route and proof below are frozen. Do not fuse processors, add a queue/record field, move any drain, alter command acknowledgements, application samples, backpressure, callbacks or DSP arithmetic. No graph optimizer, rack renderer changes, new unsafe/dependency, timing or generic test framework.

## Acceptance

A typed preparation test must distinguish default/raw Concurrent from the dedicated BetweenRenderCalls path and show the policy reaches both concrete fader/matrix bank owners. Verify all current general call sites retain Concurrent; no raw producer-exporting API silently changes contract. Existing real browser tests must retain exact applied_at_sample, atomic all-or-nothing admission, saturation/backpressure and block application behavior. A deliberate wrong default or missing host opt-in must fail the same focused assertion. Preserve existing resource/realtime/lifecycle gates.

After focused source PASS, qualify proportional workspace and supported targets/current artifact if affected, then exact actual PR-head Astra review and required CI. No performance claim: no arithmetic changes.

## Workflow

Astra approves the numbered source-level brief; Luna attempt 1; Sol attempts 2/3 only after Astra FAIL; third FAIL requires explicit rebrief. Root owns checkpoint/push/issue synchronization and delivery. #430 implements pairing only after this prerequisite closes. Scalar and concurrent-native outcomes remain separate retained issues; #349 RT-4 remains open.

## Numbered accounting

This is #442. #442 owns the immutable delivery-policy prerequisite; #430 owns serialized live bank pairing; #443 retains scalar pairing; #444 retains concurrent-native admission and pairing. #431 owns separately briefed measurement. None alone closes audit RT-4/#349.

## Exact source route, metadata and resource proof

# Astra #430 numbered split review

**PASS for scope accounting at `1a7177f842968333f22850ab6dda9d87f64badd9`; #442 is not yet ready for implementation assignment.** The remaining work is one finite source-level amendment described below, not further investigation of admission semantics. #435 remains the sole feature; #412's tooling work is independent.

Read the numbered #442/#443/#444 specs, #430's complete refreshed brief/ruling and numbered disposition, and the actual preparation route on delivered `4b352b36ba33334ea2e0c6847c0e3ecf6e8ab33a`. No implementation, builds, source edits or Git/GitHub mutation performed; no remote queries were made in this review.

## Accounting is correct

#442 owns immutable delivery-policy plumbing only. #430 retains actual serialized live W4/W8 bank pairing; #443 explicitly retains scalar/nonbanked pairing; #444 explicitly retains concurrent-native admission and resulting bank pairing, with a required split decision before that broader implementation. #431 remains separately briefed measurement. The #430 disposition supersedes historical pending-number statements and preserves the original live goal. None of these children alone closes RT-4/#349. Defaults, both queue consumers, exact application time, observable post-fader boundaries, unchanged arithmetic and fallback remain required. No source waiver is hidden in the split.

## Exact #442 propagation route now established

The actual production host path is:

`host-web private preparation (lib.rs:2630)` -> `host-core::prepare_host_runtime_with_console (prepare.rs:479)` -> `builtins-compiler::prepare_session_builtins_with_console (lib.rs:2298)` -> `PreparedBuiltinsSession` -> `GraphCompiler::compile_with_builtins (graph-compiler/src/compile.rs:67)` -> `PreparedBuiltinsSession::into_graph_artifact_with_banks (builtins-compiler/src/lib.rs:1634)` -> concrete FaderBankProcessor and MatrixBankProcessor construction (`:1714-1762`).

GraphCompiler already moves the opaque PreparedBuiltinsSession through to lowering. It does NOT need a duplicate policy argument, a second source of truth or a compiler-wide rewrite. Store the policy privately in that session and copy it into both concrete bank owners at the existing construction sites. Their drains/process methods remain unchanged. A scalar fallback remains unfused and is not an implementation of #443; no policy-driven scalar behavior is needed here.

Preserve all current public entry signatures and Concurrent defaults. Add a dedicated explicitly documented host preparation entry for BetweenRenderCalls, sharing the existing implementation through a private policy-bearing helper. Its sole production caller is WebEngine's private preparation function; its contract requires that caller to retain endpoints and admit only between exclusive render calls. At the builtin boundary preserve `prepare_session_builtins_with_console` as a Concurrent wrapper and add the corresponding explicit policy-bearing preparation function used by the host helper. Do not add the policy to ordinary HostConsoleRequest and silently opt existing callers in. Read-only caller discovery and tests should prove these wrapper defaults, not rely on the enum's derived Default alone.

## Finite amendment required before #442 approval

1. **Freeze policy type location and owner-observation seam.** Recommended minimal route: declare unversioned `BuiltinControlDelivery::{Concurrent, BetweenRenderCalls}` in graph's prepared-builtin metadata vocabulary, where builtins-compiler can import it without a dependency cycle. Add a default-Concurrent preparation metadata query to GraphPreparedBuiltinBankProcessor; the two concrete bank processors return their stored field. Include the read-only value in existing GraphPreparedBuiltinBankInfo enumeration so tests can observe the ACTUAL sealed prepared owners through PreparedBuiltinsGraphArtifact::prepared_builtin_banks. The production render path must not read the new policy in #442. This supports an end-to-end assertion without test-only constructors, reflection or a new telemetry subsystem. The spec's current allowed paths omit `crates/graph/src/lib.rs`; expressly add this narrow metadata definition/query scope before coding. Do not implement the #430 Any/factory handshake in #442.
2. **Freeze the small resource-accounting consequence.** New stored fields can change concrete owner sizes even though processing is unchanged. Existing `strip_processor_bytes` already uses size_of the concrete owners, so preserve that calculation rather than pin a guessed byte count. `crates/capi/tests/resource_lifecycle.rs:1081-1096` independently mirrors both owner layouts; explicitly allow only corresponding mirror/layout expectation corrections if the new enum field changes size. Do not discover this downstream requirement during implementation and expand the issue then. No capi runtime or ABI change is authorized. Retain no-console/default-versus-opt-in resource parity except for the explicitly accounted common metadata storage.
3. **Record a concrete end-to-end proof.** In builtins-compiler's existing artifact tests, prepare one default/raw and one explicit delivery plan, lower real banks, and inspect PostFader AND PostMatrix owner metadata for Concurrent versus BetweenRenderCalls. Include several members and non-console/default preparation. In host-web's existing private unit-test context, assert its actual construction uses the dedicated entry/policy; changing only that call back to the old wrapper must fail the same assertion. The host path may need its private construction test to inspect the artifact before binding; keep that observation preparation-only and do not add a production render report field. Keep existing ack/application-sample, atomic admission and backpressure tests unchanged. Exact test placement can be a routine implementation choice once this causal endpoint is frozen; do not replace it with assertions only about request values.

With these three items incorporated, #442 is a bounded metadata/plumbing issue with resolved ownership and admission premises. Root can then request one final numbered-scope approval and queue Luna after #435 delivery. No additional queue/admission design research is needed for this prerequisite, and no fusion or benchmark is authorized by it. If root prefers a different type/observation placement, it must have the same dependency direction, actual-owner proof and finite resource scope before assignment.

The delivered #429 arithmetic and broad #430 ownership/fallback gates remain intact. #444, not the new enum or host entry, owns any future concurrent-native cutoff guarantee. #443 remains queued/unbriefed for its distinct scalar ownership route.

## Amendment disposition

Root adopts all three recommendations above exactly: graph owns the unversioned metadata enum/default query and existing bank-info observation; PreparedBuiltinsSession privately carries the single policy to both concrete bank owners; preserved Concurrent wrappers and a dedicated documented host entry serve the sole production WebEngine opt-in. Render never reads the policy in this issue. The only added capi scope is necessary owner-layout mirrors, with actual size_of accounting preserved. Actual sealed-owner/default/explicit/host-call discrimination and same-assertion wrong-wrapper control are mandatory. Historical “amendment required” wording above is superseded by this adoption; final numbered Astra approval remains required before assignment, after #435 delivery. No factory/Any handshake, pairing, scalar behavior, queue or ack changes are authorized.

## Final numbered scope approval

# Astra #442 final numbered scope approval

**PASS for planning head `74be64d4f01b3dff3a9344bd0a4e87bc4bc992fe`.** #442 is bounded and ready to queue after #435 delivery, followed by a check of its exact post-merge source base before Luna assignment. This approves scope, not implementation or immediate parallel feature work.

Read the full amendment against the prior numbered checkpoint. Its only change is #442's spec. The explicit disposition adopts all three finite source recommendations and supersedes the embedded historical “not ready” wording. No additional research or scope amendment is required on the inspected base.

Graph owns the unversioned prepared metadata enum/default query and existing bank-info observation. PreparedBuiltinsSession privately carries the single policy through the existing GraphCompiler move into both concrete fader/matrix bank owners. Existing general preparation signatures/wrappers retain Concurrent; a dedicated documented entry supplies WebEngine's sole production BetweenRenderCalls opt-in. No duplicate GraphCompiler policy, ordinary HostConsoleRequest default change or runtime inference is permitted.

Actual sealed-owner/default/explicit/host-call discrimination remains mandatory, including a same-assertion wrong-wrapper control. Resource scope explicitly includes only necessary capi test layout mirrors and preserves actual size_of accounting. Render does not read the policy in this issue; no queue/drain/ack/application-time, arithmetic, factory/Any, pairing or scalar behavior changes are authorized. Existing browser atomic admission, backpressure and application-sample tests plus proportional resource/realtime/workspace/target/artifact and actual-head PR/required-CI gates remain.

#430, #443, #444 and #431 retain respectively serialized bank pairing, scalar pairing, concurrent-native admission/pairing and measurement. #442 does not establish a native cutoff or close RT-4/#349. The caller declaration remains explicit and is not misrepresented as enforced scheduling for arbitrary external hosts.

#435 remains the sole feature; #412 tooling is independent. After #435 delivery root freezes/checks merged source and follows Luna attempt 1, Astra verdict, Sol attempts 2/3 only after FAIL, then hard stop/rescope. No implementation, tests/builds, new research, source edits or Git/GitHub mutation performed for this approval.

## Post-#435 source-base freeze

PR #447 delivered #435 at main `99df5cf6c639f0909f82e116eb776e95c172536c` after exact-head Astra PASS and required CI. Root integrated that delivered base into this planning branch. #442 is next runtime feature only after Astra confirms the approved source route and resource gates on this base; #448 is independent tooling. No implementation is assigned by this checkpoint alone.

## Astra post-#435 approval and Luna attempt 1

# Astra #442 frozen-base review — PASS

Planning checkpoint supplied by root: `83743a3a3ce9eb77cf5e620c1f7e917030f04848`, `/home/bl/misofm/engine-430-plan`, integrating delivered #435 main `99df5cf6c639f0909f82e116eb776e95c172536c`.

PASS: the approved #442 scope remains ready for Luna attempt 1 on this post-#435 base. No finite source fact, additional research, prerequisite or scope amendment is missing. This is source-base readiness, not implementation acceptance.

Read the full current #442 spec, its explicit amendment disposition and `/tmp/astra-442-numbered-scope-review.md`. Inspected the actual metadata, ownership/lowering, host preparation, browser ownership/admission, resource mirrors and caller sites. The eight relevant files in graph metadata, builtins-compiler, host-core preparation/exports, host-web implementation/tests, graph-compiler lowering and capi resource mirrors are byte-identical to the previously inspected engine-429 copies. The intervening lease cleanup does not invalidate this route or its proof seams. The supplied Git identities/delivery status were not independently re-queried because this task forbids Git/GitHub operations.

The concrete route remains host-web `compile_ready` (lib.rs:2622, current call :2630) -> host-core `prepare_host_runtime_with_console` (prepare.rs:479, builtin call :705) -> builtins-compiler `prepare_session_builtins_with_console` (lib.rs:2298) -> private PreparedBuiltinsSession (:297) -> GraphCompiler::compile_with_builtins (compile.rs:67) -> into_graph_artifact_with_banks (builtins-compiler :1634) -> FaderBankProcessor/MatrixBankProcessor construction (:1729/:1753). GraphCompiler already moves the session; adding another policy argument there would violate the frozen single-source route.

GraphPreparedBuiltinBankProcessor and GraphPreparedBuiltinBankInfo remain the correct dependency-safe preparation metadata seam (graph/lib.rs:530/:536, enumeration :746). Add the approved unversioned BuiltinControlDelivery enum with default Concurrent query, a private session field copied to both concrete owners, and owner-sourced metadata in existing bank enumeration. Render must not read that policy. This remains metadata plumbing only, with no factory/Any handshake or pairing.

Checked current Rust caller discovery across crates, hosts, tools and sidecars. General host/session preparation wrappers and the raw builtin-console entry remain the preserved Concurrent routes. Introduce the dedicated documented host entry plus private policy-bearing helper and corresponding explicit builtin preparation entry; WebEngine's private construction is the sole production opt-in. Include the routine host-core public re-export needed for that entry. Do not change ordinary HostConsoleRequest or general wrapper signatures/defaults. ReadyOwnership still privately owns the control producers (host-web :546), and submit_commands/render_next still require exclusive mutable host access. This earns the caller declaration for that host; it does not establish a concurrent-native queue cutoff or enforce correct use by arbitrary external callers.

Resource accounting is still actual concrete size_of in strip_processor_bytes (builtins-compiler :674), and the two capi test mirrors remain at resource_lifecycle.rs:1081/:1089. Preserve real size accounting and permit only necessary mirror corrections for the new metadata fields. No capi runtime/ABI change or guessed resource constant is justified.

Keep the frozen causal proof: lower actual multi-member PostFader and PostMatrix owners through default/raw, no-console and explicit preparation; inspect their sealed owner metadata. The real private host construction must select BetweenRenderCalls, with changing only its dedicated call to the old wrapper causing the same focused assertion to fail. A request-value-only test or enum-default-only test is insufficient. The preparation-only observation may be placed in existing private tests before binding; no render report field or generic framework is needed.

Existing exact application-sample, first-block application, atomic admission and typed saturation/backpressure tests remain applicable and must stay unchanged. No queue/drain/record/ack/application-time, callback, scalar or DSP behavior changes are authorized. Preserve proportional resource/realtime/workspace, supported-target/current-artifact and exact-head PR/required-CI delivery gates. No benchmark is authorized.

Root may assign Luna attempt 1 after recording this approval; Astra reviews the coherent attempt, Sol gets attempts 2/3 only after FAIL, and a third FAIL requires hard stop/rebrief. #430, #443, #444 and #431 retain serialized bank pairing, scalar pairing, concurrent-native admission/pairing and measurement. #442 alone closes neither live integration nor RT-4/#349. Independent #448 tooling does not expand this feature scope.

Read-only source/spec/file-comparison review. No implementation, build, test, timing, repository edit or Git/GitHub operation; only this `/tmp` verdict was written.

Root assigns Luna attempt 1. #442 is the sole active runtime feature; #448 runs independent tooling qualification. The exact approved metadata/policy/default/owner-proof/resource boundaries remain binding.
