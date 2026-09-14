# #807 bounded compiling checkpoint assignments

Planning refinement only. Authority: the frozen numbered spec at `/tmp/miso-engine-804/.github/ISSUE_SPECS/807-make-dedicated-eq-cuts-live-with-off-audio-thread-prepared-targets.md` and established source findings in `/tmp/804-live-eq-spec.md`. No new source audit or design expansion. These assignments replace the broad T1/T2 ownership lists and subdivide T3's existing work; they remain one product issue and one coherent implementation attempt, with one final Astra MEDIUM verdict. A checkpoint is not an extra implementation attempt or release claim.

## Sequencing rule that keeps intermediate checkpoints honest

Build complete components in small checkpoints, then connect their existing production entry points in assignment 10. Before that cutover:

- The six dedicated-cut descriptors retain their prepared-only capability. The 16 existing numeric rows retain their current public live behavior through the existing semantic path. Its remaining render-time design is an explicitly **unresolved #807 gate**, never described as fixed by an earlier checkpoint.
- Add the real EQ companion implementation, application hooks, owners and bridges, but defer the production EQ factory's `target_preparation() -> Some(...)` registration until cutover. Its default `None` is honest until the whole route is available. Do not introduce a runtime feature flag, fake-success hook, shadow-only implementation or fallback from a failed prepared admission to semantic admission.
- Browser `host.command()` and headless `WasmBoundary.submitCommands` keep their original routing until the prepared route is ready to replace it. New private helper functions/modules can be complete and unit-tested before these entry points call them. A private companion call whose owner is not prepared-capable returns a typed refusal with zero admitted records and no mutation.
- The factory opt-in, owner creation, rejection of raw/unlowered EQ semantics, removal of scalar/bank `automate` design, browser/headless entry-point lowering, and six descriptor capability changes land together in the final wiring checkpoint. Do not remove the old processing path earlier and leave previously advertised numeric controls ineffective. Do not flip the cut rows merely because Rust host tests pass while the supported browser/headless facades still lack lowering.
- The final tree contains no temporary compatibility branch, test-only production enable switch or permissive route for an opted-in EQ's unlowered semantic span. The existing reset and apply paths already have their final no-design implementation before cutover.

Each assignment has two to four primary files where practical. Re-exports, exhaustive matches, constructor arguments, generated artifacts and directly affected test expectations are mandatory compilation glue, not a second design task. Root provides the listed files and the relevant frozen-spec sections to Luna; Luna does not need the complete surrounding repository conversation. One uncommitted assignment at a time; after its stated focused gates, Luna stops and root checkpoints exact paths before the next assignment.

## Former T1: four assignments

### 1. Fixed target contract and bounded queue snapshot

Primary files: `crates/effect-contract/src/prepared_target.rs` (new small module), `crates/effect-contract/src/lib.rs`, `crates/engine/src/realtime/spsc.rs`.

Implement the frozen 12-word Copy target/request/error types and defaulted factory/scalar/bank hooks, plus `Consumer::available_at_entry()` or the equivalent frozen-count primitive. Keep internal names unversioned and existing non-EQ implementations on real `None`/`Unsupported` defaults. **Do not append `EffectControlRecord::PreparedTarget` yet**: introduce that variant with its complete staging and application consumers in assignment 4, avoiding an interim exhaustive arm that drops it.

Glue: existing contract exports and default-hook conformance tests only. No other effect implementation files. Gate: effect-contract and SPSC focused tests, target-size/Copy assertions and native/Wasm checks of the affected crates. Prove the snapshot count is bounded and unaffected by later publication. Deliberately incomplete: enlarged queue accounting, target staging and any production prepared-target admission.

### 2. Real EQ preparation and numerical validation

Primary files: `crates/parametric-eq/src/control.rs` (new), `crates/parametric-eq/src/lib.rs`.

Implement the actual factory companion trait and six-word semantic/six-word coefficient target preparation, whole final configuration validation, touched-section coalescing, ordered Left/Right/Both output and the rounded-SVF numerical validator shared with the existing designer. Keep the production factory getter unregistered until assignment 10; direct tests invoke the concrete companion implementation. Derive the accepted finite output-mix bound from the legal EQ domains and record that decision in the numbered issue through root.

Glue: module declarations and narrowly affected existing EQ designer tests. Gate: representative launch-rate/domain grid, invalid-edit and unchanged-output-on-error tests, identity exactness, section coalescing, asymmetric Both cases, capacity tests and unsafe finite target rejection. Retain the explicit stable-forgery negative-contract test; do not claim the numerical validator proves cutoff/Q equivalence. Deliberately incomplete: a producer cannot yet obtain this capability through the production registry, and no host live-cut claim follows.

### 3. EQ application, reset cache and 464-byte state

Primary files: `crates/parametric-eq/src/lib.rs`, `crates/parametric-eq/src/response.rs`; `control.rs` only if small shared decoding belongs there.

Implement the real scalar/bank prepared-target application hooks, retained semantic enable state, initial coefficient cache for reset, ramp retargeting and response copying. Extend the per-lane state to the frozen 116 words/464 bytes, with atomic refusal of old/malformed payloads and correct live-enable restore. Keep the existing semantic `automate` process route temporarily reachable for existing production numeric controls; its removal belongs to cutover. Direct tests prepare outside render, apply the real hooks at a block boundary and render the resulting targets.

Glue: existing EQ-specific direct DSP/response/state fixtures and state-size expectations. Gate: direct scalar/bank target application, exact identity transitions, A/current-then-advance and A+64 settling, retarget/partition checks, independent lanes, cached reset without design, retained response and mid-ramp state roundtrip. Designer instrumentation must distinguish this new route from the still-present legacy numeric route. Deliberately incomplete: graph/rack queue delivery, production lowering and the global no-design-in-process claim.

### 4. Complete queue staging and graph/rack consumption

Primary files: `crates/effect-contract/src/live.rs`, `crates/effect-contract/src/symmetry.rs`, `crates/graph/src/runtime.rs`, `crates/rack/src/lib.rs`.

Now append `PreparedTarget` to `EffectControlRecord`; implement concrete preallocated target staging, both staged counts, bounded entry-count drain and symmetry folding. Wire actual scalar application before process and actual bank application only after desymmetrization inside `process_inner`. Preserve the existing EQ desymmetrization copy. A prepared record cannot be consumed by an ignore/no-op arm. A producer/admission route that cannot deliver prepared targets must refuse them **before publication**, never after issuing an admission report. No production prepared-capable EQ owner is published before cutover.

Required exhaustive-match glue: all `EffectControlRecord` and `Staged` consumers in the four files, existing LiveConsoleRecord symmetry construction, constructor/preparation calls supplying separate preallocated span/target slices, and the existing producer preflight where it checks supported record delivery. Non-EQ effects keep their normal semantic path and default unsupported hooks. Update actual queue-size/resource accounting and directly affected budget tests at this checkpoint, including the existing calculation in `crates/host-core/src/prepare.rs` if it embeds record size: the larger record affects **every queue** using the enum, with `(depth+1)` slots. Do not postpone an otherwise false memory budget until assignment 5.

Gate: contract live-control/SPSC tests, graph/rack focused tests, native/Wasm builds of affected runtime crates, `EffectControlRecord <= 64` assertion, no producer-chasing drain, bounded staging without drop, and collapsed-to-dual target survival against an always-dual oracle. Direct fixture admission can exercise real completed EQ hooks under exclusive ownership; it is not evidence for the still-unconnected public host route. Deliberately incomplete: owner shadow, public companion admission and final integration gates.

## Former T2: three assignments

### 5. Compiler-owned candidate transaction and resource accounting

Primary files: `crates/effect-compiler/src/control.rs` (new), `crates/effect-compiler/src/prepare.rs`, `crates/effect-compiler/src/lib.rs`, `crates/host-core/src/prepare.rs` only for actual new owner/staging charges.

Implement the real optional owner seeded from accepted `EffectBankPreparation.initial_values`: retained factory, rate, committed/candidate values, dirty flags and revision. Implement finite candidate editing/validation, target matching, revision preflight and commit/discard operations. Owner construction follows the production factory capability, so existing EQ owners remain on their old route until assignment 10. Keep staging allocated only for opted-in owners; do not make all effects carry EQ shadows. Finish actual owner/cache/staging accounting separately from serialized-state and queue storage.

Glue: exact existing producer construction/attachment sites and narrowly affected resource tests; no all-roster constructor rewrites. Unit tests may directly construct an owner with the real companion implementation or an explicit fixture registry. Such tests exercise the component; they must not masquerade as production EQ registration. Gate: focused compiler/control/resource tests and native/Wasm checks; 60-row authoritative seeding, sparse/default/asymmetric values, full validation before candidate changes become committed, coalescing, revision overflow and failure rollback. Deliberately incomplete: cross-destination host publication and browser/headless lowering.

### 6. Stateless preparation workspace and exact additive ABI

Primary files: `crates/host-core/src/control_preparation.rs` (new), `hosts/host-web/src/control_targets.rs` (new small workspace/codec module), `tools/parameter-metadata/src/abi_layout.rs`.

Implement the frozen single-owner stateless preparation facade and fixed buffers/layouts: explicit rate, 60 seeds, <=256 edit rows, 60 final values and <=12 targets. Define companion structs/capacities, generated offsets and strict size/version/reserved/count checks. The record cap is `2 * MAXIMUM_COMMAND_RECORDS`, hence 512 per submission, never a track limit. Retain/prewarm the existing registry factory once; no all-roster reconstruction per edit and no direct effect dependency in host-web.

Required export glue: `crates/host-core/src/lib.rs`, `hosts/host-web/src/lib.rs` and `hosts/host-web/src/ffi.rs` for module/exports and fixed accessors; ordinary ABI-generated artifacts. This glue exposes only implemented operations. The not-yet-registered production EQ capability returns the existing typed refusal, not success with zero/missing targets. Do not add a placeholder prepared-admission export before assignment 7 implements it.

Gate: workspace/codec native tests, exact layout/capacity and malformed buffer tests, deterministic direct preparation tests with the real capability, additive ABI generation/check and host-web Wasm build. Deliberately incomplete: production config-copy seed availability, cross-queue admission and the SDK-owned opaque transport. Preparation success is not admission.

### 7. Actual host transaction and accepted-shadow copy

Primary files: `hosts/host-web/src/lib.rs`, `hosts/host-web/src/control_targets.rs`, `hosts/host-web/src/ffi.rs`, `hosts/host-web/src/tests.rs`.

Implement addressed config-copy from the accepted owner, complete companion validation, mixed-command lowering/preflight, capacity checks for every destination, exclusive between-block publication and post-publication shadow/revision/solo commit. Return the report only after real admission. Preserve original command indexes and generation ownership; all input failures are atomic. An absent prepared-capable owner is a refusal, so this additive internal route remains fail-closed before EQ registration. Keep existing ordinary semantic commands unchanged while production opt-in is absent.

Glue: only the existing host ownership/constructor fields and generated export declarations introduced in assignment 6. Gate: focused host/transaction tests using explicit complete opted-in fixtures, no-allocation/prewarmed admission checks, native/Wasm host build, missing/extra/unsafe payload cases, late invalid records, stale generations/revisions, full unrelated queue and two successive accepted candidate transactions. Include non-EQ regression gates through the actual existing route. Record that the actual production EQ registry/host/browser proof is deferred to assignment 10; fixture injection alone does not close it.

## Existing T3: separate helper, SDK and final cutover contexts

### 8. Shared browser preparation helper and worklet transport

Primary files: `hosts/host-web/web/prepared-control.js`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`, `hosts/host-web/web/miso-engine-v1-audio-worklet.js`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.d.ts`.

Implement the complete private helper and worklet protocol: retained verified module, main-realm no-boot/no-render preparation workspace, lazy accepted-shadow request, opaque payload ownership, one pending EQ batch, immutable transfer, matching-ack commit and stale/ambiguous/dispose invalidation. The worklet's companion branch calls actual prepared admission; it never designs coefficients. Ordinary command routing remains unchanged until cutover. Provide the real prepared submission function without installing it as the default semantic entry point yet; no coefficient injection API and no public mutable prepared object.

Glue: existing host-web JS unit/protocol tests and normal asset inclusion for the new module. Gate: helper lifecycle/request/byte-identity tests, zero speculative commit on failure, typed busy refusal, no boot/render on preparation instance and unchanged existing host defaults. Deliberately incomplete: production default-host EQ lowering and actual browser PCM/response parity.

### 9. Headless lowering and SDK package plumbing

Primary files: `sdk/src/core/boundary.ts`, the existing SDK asset/build inclusion module(s); edit only the known existing modules, not a new packaging subsystem.

Implement the headless route using the same shared helper and its existing Wasm instance under exclusive between-render ownership. Add real preparation/config-copy/admission calls and bind both numeric and semantic entry points to the same lowering function; install the default call-site routing at assignment 10. Complete normal helper asset inclusion and generated ABI/type linkage. The production submit path remains the previous working path until cutover, with no new public coefficient facade or parallel TypeScript designer.

Glue: ordinary generated `sdk/src/generated/*` and `sdk/src/browser/shipped-host.d.ts`, existing focused boundary/package tests. Gate: types, generation/asset checks, pure boundary lowering/error tests and unchanged legacy headless submissions. Deliberately incomplete: actual production EQ opt-in, new cut metadata and final browser/headless integration. This assignment must leave only call-site wiring for SDK activation, not an unimplemented candidate/ack algorithm.

### 10. Atomic production cutover and coherent integration proof

Primary implementation files: `crates/parametric-eq/src/lib.rs`, `hosts/host-web/web/miso-engine-v1-audio-worklet-host.js`, `sdk/src/core/boundary.ts`. These are final registration/call-site changes using already implemented components, not another framework tranche.

In this one checkpoint: opt EQ into the real factory capability; its already-implemented compiler path now creates the accepted owner. Remove scalar/bank semantic-to-coefficient processing and ensure direct unlowered EQ spans are counted/refused. Activate actual default browser/headless semantic lowering, including the supported raw-record facade and SDK numeric/object overloads. Change the six cut descriptors to the frozen live capability only when their actual end-to-end tests pass. Old numeric rows move through that same coherent owner with no capability downgrade.

Mandatory integration glue: any exact production owner-registration call site that does not follow the getter automatically; the remaining EQ-specific direct-automation conformance callers; ordinary metadata/ABI/Wasm/SDK asset regeneration. Primary gate files are the existing host-web Rust/JS integration tests and `sdk/test/{console-evals,headless-path-evals,browser-defaults-evals,live-response-evals}.mjs`. Those are acceptance evidence, not permission to add another browser harness. Root supplies the actual build-script path already owned by the SDK work when briefing assignment 9; no rediscovery of unrelated packaging work is required.

Gate: actual production-registry host admission in scalar and bank placements; existing numeric live edits still work; both low-level and SDK browser/headless facades produce identical trusted payloads; one real packed browser episode establishes actual worklet application, acknowledged A, copied target response with `capturedSample > A`, and settled PCM at/after A+64. Run the frozen designer/allocation counters across admission, process, reset and mono transitions, adversarial atomic-batch cases, 464-byte state/response gates and proportional native/Wasm/SDK checks. No ignored or expected-failing new integration tests remain. Metadata must describe the artifact actually built and tested.

If final wiring exposes a missing component rather than a local connection error, stop the cutover, leave the last honest checkpoint intact and finish that already-scoped component in its owning assignment before retrying. Do not hide the gap behind a successful report, fallback or capability lie. Root takes the focused-green final checkpoint and requests the one fresh Astra MEDIUM verdict over the coherent #807 attempt. Broader release/registry/app deployment remains the existing downstream scope.

## Gate accounting at handoff

Every checkpoint report contains: exact changed paths; commands and results for its focused gates; any required glue beyond its primary list; and the numbered acceptance gates still deliberately pending. Record incomplete integration candidly without disabling existing tests or calling the issue PASS. A local foundation checkpoint may compile and pass its component tests while #807's full acceptance remains incomplete; it may not claim a deployable live-cut capability.

Exhaustive-match glue must route, stage, apply or refuse a new case correctly. Prohibited shortcuts include `_ => {}`, `Ok(())` for unsupported application, success with zero generated targets, an accepted semantic shadow without queued/application targets, acknowledgement before sidecar validation/publication, and silently dropping overflowed staging. If required glue cannot be kept mechanical, root moves that implementation into its owning assignment before accepting the checkpoint. This split changes task size and activation order only; every frozen #807 product, trust, bounded-memory, channel, A+64, response and no-design-on-audio acceptance gate remains binding.

## Root forward integration notes from #805 CI
- Assignment3 final464bytes/lane changes declared total920->936; graph-compiler/tests/track_delay.rs current semantic digest includes9 instances declaredstate, so independently derive expected+144 estimate delta and update this existing test pin in the same tranche if changed. CAPI metadata count stays30perEQ/69roster, responsecount stays6. Do not repeat old63/4 assumptions.
- Assignment4 queue-size growth may move resource-lifecycle exact layout oracles; run proportional affected host/capi/graph compiler resource tests with the actual enlarged enum, derive deltas rather than observedrepins.
