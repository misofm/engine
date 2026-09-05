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

## Luna attempt 1 compiling checkpoint

Luna implemented prepared metadata, private session-to-owner propagation, preserved Concurrent wrappers, dedicated host preparation and WebEngine opt-in, plus capi owner-layout mirrors. Reported targeted check, actual builtins owner metadata test, builtins library17/17, graph52/52, host-core7/7, host-web60 passed/1 ignored, capi lifecycle4/4, policies/fmt/diff passing in `/tmp/luna-442-*`. This is a coherent checkpoint, not source acceptance. Astra must verify the entire frozen contract, including actual host-to-owner causal proof and the same-assertion wrong-wrapper mutation; no such evidence is inferred merely from the existing host suite passing. No full workspace, artifact/browser qualification or timing is claimed.

## Astra attempt 1 FAIL and Sol attempt 2

# Astra #442 Luna attempt 1 review — FAIL

Exact reviewed checkpoint: `74ef8d5ed62c4562c1e9072a7d8de63c9c0c87e7`, `/home/bl/misofm/engine-430-plan`, against approved `b3185b15`. Source checkpoint `8394abab` plus the evidence correction comprise this attempt.

FAIL. Three finite blocking groups remain within the frozen scope. Luna attempt 1 is consumed; a bounded Sol attempt 2 may correct these together. No full workspace/artifact qualification should precede a coherent source PASS.

## 1. Restore accidentally damaged existing graph tests and qualify the actual checkpoint

`crates/graph/src/lib.rs` still defines GraphNodeId::TrackStage with required `track_id` AND `stage` fields (lines 73–77), but three existing test constructors at approximately 1984, 2028 and 2071 now omit `stage` entirely. The cumulative diff deletes `stage: TrackStage::PostInputBuiltins` from all three. This is a definite Rust missing-field compile error in the graph unit-test configuration, independent of any run environment. The same change broadens the existing stage-specific membership filter around 1975 to all TrackStage nodes. These edits are unrelated to policy metadata and must be restored exactly; do not weaken the graph fixture or enum to make them compile.

Root independently confirmed this on the exact checkpoint: `cargo check --locked -p graph --tests` exited 101; `/tmp/engine-442-exact-checkpoint-check.log` reports the missing stage fields at 1984/2026/2071. The retained `/tmp/luna-442-graph.log` says graph's 52 tests passed, but cannot qualify these actual malformed constructors. Preserve that record candidly as earlier-source evidence and regenerate proportional graph/fmt/check evidence on the final corrected source. This is not a request for a broad build or new graph tests.

## 2. Complete the frozen actual-host causal proof and controls

The only added policy test is builtins-compiler's `control_delivery_metadata_reaches_both_sealed_strip_bank_owners` around line 4337. It does lower real banks and observes owner-sourced metadata, which is useful accepted proof. However, all three preparations pass an empty control request slice, and the test directly selects builtin preparation functions. It never goes through host-core's policy selection or the actual WebEngine `compile_ready` call.

No new host policy assertion, private host construction observation or wrong-wrapper mutation evidence exists in the source/logs. Existing host-web behavior tests cannot discriminate the missing opt-in because #442 intentionally does not read this metadata during render. Replacing the WebEngine call with the old Concurrent wrapper leaves the new direct-builtin metadata test untouched. Thus the frozen mandatory host-to-owner assertion and same-assertion wrong-wrapper control are absent, not merely under-documented.

Finish the existing bounded proof: prepare actual default/raw and explicit live-console owners (populated control requests), retaining the no-console/default control; observe both PostFader and PostMatrix real sealed bank metadata. Add the actual private WebEngine construction assertion using a preparation-only observation before binding, as already authorized. Demonstrate that changing only its dedicated production entry call back to the old Concurrent wrapper makes that same assertion fail for the expected policy mismatch, while the original passes. Keep the mutation well-formed and verify its exact target/hunk; syntax or setup failures are not the control. Retain an appropriate default-path discrimination so changing the raw/general default does not silently opt exported producers in. No render report field, duplicate simulated policy, test-only request-value assertion or new generic framework is warranted.

## 3. State the explicit caller contract in the public dedicated entries

The new host-core entry at prepare.rs:488 and builtin entry at builtins-compiler/lib.rs:2324 have only one-line descriptions saying producers/control ownership are “exclusively retained between render calls.” That does not expressly require what the frozen contract depends on: the caller must retain the producer endpoints and admit records only outside exclusive render calls, with no concurrent enqueue during render. The enum variants also carry no explanation of that distinction.

Document those obligations directly on the dedicated public entry (and its builtin counterpart/reference), and state that this is a caller declaration rather than runtime synchronization/enforcement. Existing general APIs remain Concurrent. This is the previously required documented policy contract, not a request for locks, ownership API redesign, unsafe, scheduling enforcement or a native cutoff guarantee.

## Accepted implementation and preserved boundaries

The graph-owned unversioned enum/default query is dependency-safe. PreparedBuiltinsSession privately carries one policy to both concrete fader/matrix owners. GraphPreparedBuiltinBankInfo obtains the value from `bank.processor.control_delivery()`, not from a separate request/default; the added stage observation identifies the actual member stage. Host-core's private helper chooses existing/default versus explicit builtin preparation, and GraphCompiler gains no duplicate policy. The WebEngine production call currently selects the dedicated entry. Existing raw/general wrappers select Concurrent; no ordinary HostConsoleRequest field/default changes.

No production queue drain, record, acknowledgement, application sample, callback, arithmetic, factory/Any, scalar behavior or render policy query is changed. Both concrete owners merely store/query preparation metadata. Source resource accounting still uses real size_of, and capi changes are limited to matching owner mirrors and their resulting expectation deltas. No new capi runtime/ABI change appears. The supplied focused builtins/host/capi/policy records are useful, subject to accurate final-source provenance; immutable workspace, supported targets/current artifact and actual-head PR/required-CI gates remain pending.

Reviewed the full numbered contract/frozen-base approval, cumulative source changes and existing evidence. No source changes, builds/tests, benchmarks or Git/GitHub mutations were performed. Only this `/tmp` verdict was written. #442 remains the sole feature; #430/#443/#444/#431 retain their separate outcomes. This verdict is one consolidated adversarial review, not authorization for additional Luna sub-attempts.

Root records that the exact committed graph test configuration fails compilation (exit101), superseding Luna’s reported compiling checkpoint. Earlier PASS logs are not evidence for those later malformed constructors. Sol attempt 2 is authorized for the three finite groups above; no other feature or API expansion is authorized. Preserve exact failed-source evidence and produce fresh checks for the corrected coherent checkpoint.

## Sol attempt 2 correction evidence

Sol attempt 2 restored the three damaged `TrackStage` fixtures and their original
`PostInputBuiltins` membership filter. The policy proof now uses populated eight-track live-console
requests for both the general/raw Concurrent and explicit BetweenRenderCalls preparations, retains
the no-console Concurrent control, and observes both real multi-member PostFader and PostMatrix
owners. Host preparation separately reads both stage policies back from the sealed artifact owners
before binding; absence defaults to Concurrent and therefore cannot satisfy WebEngine's
BetweenRenderCalls assertion. WebEngine's private production construction asserts both stage
observations in its existing test build.
The dedicated host and builtin entry documentation now requires retained producer endpoints,
admission only between exclusive render calls, and no concurrent enqueue during render, while
stating that selection is a caller declaration rather than runtime enforcement.

Focused causal control: changing only WebEngine `compile_ready`'s production preparation call to
the well-formed old Concurrent wrapper compiled, then the same
`source_backpressure_seek_render_and_stable_output_are_bounded` assertion failed with
`WebEngine construction must seal between-render-call post-fader owners`. The source was restored
before final checks. Complete records are `/tmp/sol442-production-wrong-wrapper-full-diff.patch`,
`/tmp/sol442-host-web-production-policy-mutant.log` and its exit record. An earlier invocation used
`--exact` without the module-qualified name and ran zero tests; it is retained but does not
qualify any claim.

Final-source focused records use `CARGO_TARGET_DIR=/tmp/sol442-target` and are listed in the Sol
handoff. This local evidence is not an Astra PASS, pushed evidence, GitHub synchronization, target
qualification, artifact qualification, or required CI.

API consequence for Astra review: the two stage-specific observations are retained as private
fields of the public `HostConsoleHandles` aggregate and exposed only through hidden read-only
queries. Because the aggregate previously had only public fields, adding private fields prevents
external struct-literal construction. No repository caller constructs it outside host-core, but
this source-compatibility consequence is real and is not claimed as pre-approved by this attempt.

## Astra attempt 2 verdict and final Sol attempt 3

# Astra #442 Sol attempt 2 review — FAIL

Exact checkpoint: `ea575f1aa46353e2e291ba43cb39b4a89e60a5cb`, `/home/bl/misofm/engine-430-plan`.

FAIL for one remaining bounded scope/API blocker: the host proof introduces retained production fields and changes external construction of HostConsoleHandles. The other attempt1 corrections are accepted. One final Sol attempt3 remains; a further FAIL requires the prescribed hard stop/rescope.

## Accepted corrections and evidence

The three graph TrackStage constructor fields and original PostInputBuiltins filter are restored. Fresh graph records show successful check and all 52 tests. The builtin test now lowers populated eight-track live-console raw/explicit plans plus the no-console control and observes actual PostFader/PostMatrix owner metadata. The default/live and no-console host checks pass. Both dedicated entries now expressly require retaining producer endpoints, admission only between exclusive render calls and no concurrent enqueue, while distinguishing caller declaration from runtime enforcement.

The host causal proof is meaningful: the normal and restored named WebEngine test each run one test and pass. The well-formed old-wrapper replacement compiles, runs `tests::source_backpressure_seek_render_and_stable_output_are_bounded`, and exits101 at the expected `WebEngine construction must seal between-render-call post-fader owners` assertion. The earlier zero-test invocation is explicitly nonqualifying. The provided full diff is against the earlier checkpoint and includes assertion additions; comparing its assertion text with final source establishes that the counter-case changes the production preparation call, not the expected policy. Preserve these candid records and retain an exact original-to-mutant one-call diff for the final probe.

Capi lifecycle reports 4/4. Root additionally ran the whole host-web library suite on this exact checkpoint with exit0 in `/tmp/engine-442-root-host-web-attempt2.log`. There is no observed current-host-suite regression to report. No additional uniformity refusal remains. Accepted enum/session/concrete-owner propagation, Concurrent wrappers, actual owner-sourced graph metadata and unchanged queue/ack/DSP/render behavior remain intact.

## Remaining blocker

`crates/host-core/src/prepare.rs` adds two private fields to public HostConsoleHandles (around312), hidden public queries (around342), captures the first owner policy for each stage before binding (around821), and retains the results in the returned aggregate (around988). Before this change the aggregate's fields were all public, so external struct literals and exhaustive destructuring were possible. The private additions now prevent them. Hidden documentation does not make those fields or methods an implementation-only change. The spec candidly acknowledges this consequence but does not authorize it; it explicitly says the consequence is not pre-approved, and root's attempt2 authorization prohibits other API expansion.

The frozen route authorized metadata on the graph/session/concrete bank owners and a preparation-only causal observation. It did not authorize a second retained host observation payload, new host query surface, or retirement of public aggregate construction. Absence of repository struct-literal callers is not authorization to break external callers. This repeats the public-API distinction deliberately made explicit for #435; #442 has no corresponding retirement ruling.

The unconditional cfg(test) assertions in compile_ready also interpret absence of a bank as a Concurrent mismatch, because stage_delivery defaults absent stages to Concurrent. Current host fixtures pass, so this is not an observed failure; nevertheless the intended proof should target its known bank-producing fixture and require actual owners there, not impose a global bank-presence condition on all valid preparations or future scalar/no-bank tests. Removing this global proof instrumentation belongs to the same bounded correction, not a new runtime feature.

## Frozen final Sol attempt3 correction

Remove the two HostConsoleHandles fields, their hidden queries, the production stage-policy collection and associated initializers. Restore that public aggregate's original source shape. Remove the unconditional host test assertions and host tests' reliance on those queries. Preserve all accepted policy storage on actual prepared bank owners, the populated/default builtin metadata test, explicit public entry documentation, original wrapper signatures/defaults and resource-owner accounting.

Finish the actual-host evidence through a bounded DISPOSABLE preparation probe, without retained production telemetry or new public APIs: in a temporary copy/instrumented source, immediately after GraphCompiler returns the real artifact and before binding, inspect its actual prepared_builtin_banks metadata. Under execution of the existing named WebEngine bank-producing fixture, require at least one PostFader bank and at least one PostMatrix bank, require nonempty members on each inspected bank, and require every inspected owner in each of those populations to report BetweenRenderCalls. Do not require more than one member here: the named real host fixture has one track, while the accepted builtin test separately supplies the multi-member proof. Place this immediately after `GraphCompiler::compile_with_builtins(...).map_err(...)?` in the private host preparation helper, where `artifact` is available, before the existing `let graph_report = artifact.report().clone()` and before `artifact.into_bound`. This assertion must use actual owner metadata, not the requested enum or wrapper name. Run that same named real host test with the correct production call, then change only `compile_ready`'s `prepare_host_runtime_between_render_calls(&session, caps, &console)` call to `host_core::prepare_host_runtime_with_console(&session, caps, &console)`; the same PostFader policy assertion must fail after successful compilation. Restore/remove the disposable probe and mutation before the final source checkpoint. Retain the small reproducible probe patch, exact one-call mutation diff, original/mutant exit+assertion logs and restored-source checks in the decision record. This is the existing causal gate implemented without a permanent API change, not a new framework or a weakened source-only grep.

Default/raw/no-console discrimination remains supplied by the actual sealed-owner tests; any supplemental host-default observation can use the same bounded disposable pre-bind seam. Do not add a callback API, feature framework, report field, unsafe downcast, duplicate policy cache or runtime refusal to implement the evidence.

Run only proportional final-source checks for the correction: restored graph/host configurations and existing focused policy/resource assertions; root's current full host-library result is useful history, but source changes require the applicable final checks. Full workspace/targets/current artifact remain after source PASS. Preserve all historical failed/zero-test evidence honestly. A final PASS will still require actual pushed PR-head Astra review and required CI before delivery.

Reviewed full numbered contract/attempt history, final changes, actual source and supplied logs. No source edits, builds/tests, timing or Git/GitHub mutations by this review; only this `/tmp` verdict was written. Root's independently executed host-suite result is credited as such.

Root authorizes the one final bounded correction above. Remove the proof-only public aggregate/API change and retain the same actual-host causal gate through the specified disposable pre-bind probe. No new production API, callback, framework, refusal, or arithmetic change is authorized. This is final attempt 3; further FAIL requires hard stop and explicit rescope.

## Sol attempt 3 correction evidence

Final source removes the proof-only fields and hidden queries from `HostConsoleHandles`, removes
their collection and initializers, and removes all tests that depended on them. This restores the
public aggregate's original construction and destructuring shape. No replacement production
telemetry, callback, feature, refusal or runtime assertion remains.

The bounded disposable probe was inserted immediately after the real `GraphCompiler` preparation
returned its artifact and before report collection or binding. For the named one-track WebEngine
fixture it required at least one actual PostFader owner and one actual PostMatrix owner, nonempty
members for every inspected owner, and BetweenRenderCalls on every owner in both stage populations.
The normal production call compiled and the named test passed. Changing only the unique
`compile_ready` call to the old Concurrent wrapper compiled and made the same PostFader owner
assertion fail with left `Concurrent` and right `BetweenRenderCalls` (exit 101). The production
call and probe were both removed/restored before final-source checks.

Reproducible evidence is `/tmp/sol442-attempt3-disposable-prebind-probe.patch`, the exact one-call
diff `/tmp/sol442-attempt3-production-call-mutant.patch`, normal and mutant logs
`/tmp/sol442-attempt3-probe-original.log` and `/tmp/sol442-attempt3-probe-mutant.log`, and their
`.exit` records. The accepted populated eight-track owner test remains the multi-member proof and
the default/raw/no-console discrimination. Final-source records are listed in the Sol handoff;
they do not claim Astra PASS, pushed evidence, GitHub synchronization, target/artifact
qualification or required CI.

## Final source PASS and immutable qualification boundary

# Astra #442 final Sol attempt3 source review — PASS

Exact checkpoint: `1c9d0d7368bdbaa3ad4eba9444e020d9f4d5eaf2`, `/home/bl/misofm/engine-430-plan`.

PASS for source and the frozen causal proof. The one remaining attempt2 blocker is resolved. Root may now freeze the immutable source and perform required full-workspace, supported-target/current-artifact qualification. This is not final PR, CI or delivery acceptance.

Read the full numbered contract/attempt history, prior attempt2 ruling, final four-path correction and retained probe/control/final-check evidence. HostConsoleHandles has its original public field shape again: both private observation fields, hidden queries, collection and initializers are removed. Host-core tests no longer depend on them and are restored to their earlier form. The unconditional host construction test assertions are removed. There is no replacement retained host telemetry, new public probe API, global bank-presence refusal, callback, feature or runtime assertion.

The disposable proof is at the exact accepted boundary: immediately after the real GraphCompiler artifact returns and before `let graph_report = artifact.report().clone()` or binding. It enumerates actual prepared_builtin_banks separately for PostFader and PostMatrix, requires nonempty members for EVERY inspected bank, checks every owner's control_delivery equals BetweenRenderCalls, and requires a positive bank count for each stage. Thus an absent stage cannot pass vacuously and the policy is read from actual sealed owners rather than the requested enum. The known one-track fixture is valid here; the preserved eight-track builtin test separately proves multi-member/default/raw/no-console behavior.

The original probe log compiles and runs the real named WebEngine `tests::source_backpressure_seek_render_and_stable_output_are_bounded` test once, with exit0. The retained exact one-call diff changes only compile_ready's dedicated preparation call to `host_core::prepare_host_runtime_with_console(&session, caps, &console)`. That mutant compiles, runs the SAME named test and exits101 at the SAME PostFader owner equality assertion, displaying left Concurrent/right BetweenRenderCalls. Its unused-import warning is incidental; the failure is the intended policy mismatch, not compilation, zero tests or setup. The original disposable probe file has a bare `@@` insertion header, so it is an annotated insertion, not itself an applicable unified patch. Root mechanically normalized the identical added lines against the unique frozen-source anchor in `/tmp/engine-442-attempt3-normalized-prebind-probe.patch` and reports `git apply --check` PASS with the worktree still clean. This is evidence packaging only; no execution of an unrun probe variant is credited. The executed original insertion and exact call diff establish the causal result. The final source contains neither disposable assertion nor mutant call.

Accepted implementation remains: graph-owned unversioned metadata/default query; a private PreparedBuiltinsSession policy copied to both concrete fader/matrix owners; owner-sourced prepared-bank metadata; existing general wrappers remain Concurrent; WebEngine's private preparation is the sole production dedicated opt-in; entry documentation explicitly states producer retention/no concurrent admission and caller responsibility. GraphCompiler gains no policy copy. No queue/drain/ack/application-time, scalar processing, arithmetic, factory/Any or render policy query changes appear. Resource accounting still uses actual concrete owner sizes, with the accepted capi mirror/expectation corrections.

Final-source records show graph check and 52 tests, the populated builtin policy test, host default/live and no-console checks, restored host test, and capi lifecycle4 passing, with fmt/workspace-policy records retained. The prior full host-web library PASS remains historical evidence; immutable qualification now supplies the broader final-source gate. No full workspace, supported-target, artifact/browser or benchmark result is inferred from this source verdict. Earlier malformed-source and zero-test records remain candidly historical, not reused as final proof.

No further source correction or scope expansion is required. Root must retain the proof patches/logs with immutable delivery evidence, complete mandatory qualification, obtain actual pushed PR-head Astra review and required CI, then merge and synchronize closure. #430/#443/#444/#431 and broad RT-4/#349 remain separate outcomes; #427 tooling remains independent.

Read-only review; no source changes, builds/tests, benchmarks or Git/GitHub mutations. Only this `/tmp` verdict was written.

Root integrated delivered tooling-only PR #449 (main `39da065507beb822ef70a1552ff5dcc363938dd4`) before qualification. That upstream change leaves runtime/artifact build inputs unchanged; the accepted policy source is preserved. This checkpoint freezes source for full locked workspace including doctests and supported targets, followed by current artifact/browser qualification. No implementation, source pin, or consumer changes may be layered during the workspace run. #427 remains an isolated tooling attempt.
