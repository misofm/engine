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
