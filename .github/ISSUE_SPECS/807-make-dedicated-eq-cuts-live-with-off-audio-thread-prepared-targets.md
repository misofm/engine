# Make dedicated EQ cuts live with off-audio-thread prepared targets

## Parent, dependency and bounded delivery

Delivery child of #804, after prepared-dedicated-EQ child #805. The user authorized Astra XHIGH design, bounded sequential Luna XHIGH implementation and Astra MEDIUM adversarial review; root owns numbered local/GitHub issue creation, checkpoints and delivery. Maximum five coherent attempts, each with one adversarial verdict. This is one product slice: existing browser and headless consoles can enable, sweep and disable the dedicated cuts of their one existing EQ, and their acknowledged PCM and copied target response agree. No dependency on #774, no builtin filter control, no all-effect migration, no new response product, no second host/graph, and no new benchmark framework.

Baseline inspected: engine `551f6d7e`, `/tmp/miso-engine-804`, plus `/tmp/804-prepared-eq-spec.md`. Implementation rebases onto the prepared child and the #147 metadata checkpoint. This spec deliberately corrects two misleading source claims: `PreparedParametricEq::automate` currently calls `candidate.words -> design_svf` from both process paths, and browser `submit_commands` runs on the AudioWorklet thread even though it is called between callbacks. Moving design into that function alone is insufficient.

## Frozen public behavior

Retain the prepared child's single physical cascade HPF -> four original bands -> LPF, fixed second-order 12 dB/oct TPT SVFs, original IDs and response-section mapping, independent channels, zero latency, infinite tail, zero scratch, existing cutoff/Q domains and exact disabled identity. HPF IDs 65/66/67 remain `hpf-enabled`/`hpf-frequency`/`hpf-q`; LPF IDs 81/82/83 are analogous. No slope parameter. Cutoffs may overlap.

The six new rows become `AutomationRate::Block`, `automatable=true`, live-updatable, with the existing linear 64-sample **coefficient-word** ramp. Boolean values are stepped semantic targets; their audible transition ramps coefficients, not a fractional boolean. The existing 16 general-band numeric rows move through the same prepared-control path. Original general-band enabled/kind remain prepared-only; this issue does not add eight unrelated controls. All 30 rows nevertheless belong to one coherent configuration authority, including immutable/default values used by design. Both addresses both lanes simultaneously.

Public `miso.command.v1` command IDs, the 48-byte record, command report and `EffectParam` semantic command stay unchanged. Add a bounded companion payload and additive host-web exports for preparing/submitting it. Do not put coefficient bits into the four public semantic value slots or renumber command kinds. SDK numeric and generated-object overloads from #147 both reach the same lowering. The supported raw-record facade still accepts only semantic records and privately runs the preparation/companion sequence. Direct companion submission is an internal trusted-host ABI with the provenance precondition below, not a public arbitrary-coefficient API. An EQ semantic record lacking its required target payload is refused atomically, never redesigned on the audio thread. Existing non-EQ submissions retain their current behavior.

## Small contract seam

Add an optional factory capability and two defaulted application hooks in `effect-contract`. Only parametric EQ opts in. Other effect implementation files are untouched.

```rust
// Internal Rust identities are unversioned; these are not repr(C) wire types.
pub const PREPARED_EFFECT_TARGET_WORDS: usize = 12;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PreparedEffectTarget {
    pub slot: u32,                 // Effect-owned logical section, EQ physical order 0..6.
    pub channel: ParameterChannel,
    pub words: [u32; 12],          // Validated semantic configuration + designed words.
}
pub struct EffectTargetRequest<'a> {
    pub sample_rate: u32,
    pub values: &'a [InitialParameterValue], // Complete final candidate, descriptor order.
    pub changed: &'a [bool],                 // One bit per entry of values, touched this batch.
}
pub trait NativeEffectTargetPreparation: Send + Sync {
    fn maximum_targets(&self) -> usize;
    // Control/main/worker plane only: may call the effect's designer.
    fn prepare_targets(&self, request: EffectTargetRequest<'_>,
        out: &mut [PreparedEffectTarget]) -> Result<usize, EffectTargetError>;
    // No design, allocation or mutation: shape, expected semantic contents and numerical safety.
    fn validate_targets(&self, request: EffectTargetRequest<'_>,
        targets: &[PreparedEffectTarget]) -> Result<(), EffectTargetError>;
}
// NativeEffectFactory, default None:
fn target_preparation(&self) -> Option<&dyn NativeEffectTargetPreparation>;
// PreparedNativeEffect, default Unsupported, rejection leaves state unchanged:
fn apply_prepared_target(&mut self, target: &PreparedEffectTarget)
    -> Result<(), EffectTargetError>;
// PreparedNativeEffectBank, default Unsupported:
fn apply_prepared_target_lane(&mut self, lane: usize, target: &PreparedEffectTarget)
    -> Result<(), EffectTargetError>;
// Append to EffectControlRecord:
PreparedTarget(PreparedEffectTarget),
```

`EffectTargetError` has the small required vocabulary `Unsupported`, `Shape`, `Domain`, `Coefficients`, `Capacity`. Map it to the existing host refusal vocabulary; no new public error taxonomy is needed merely for these hooks. Validation failure must leave output/count/state unchanged. The factory owns decoding and validating its payload; neither graph nor rack knows EQ parameter IDs or coefficient equations. Retain the factory `Arc` only for opted-in producer owners so the borrowed companion capability has a valid lifetime.

EQ target words 0..5 encode `enabled` (u32 0/1), `kind` (existing EqBandKind discriminant), then normalized f32 bits for frequency/gain/Q/shelf slope. Words 6..11 encode existing `c1,a2,a3,m0,m1,m2` rounded f32 bits. The dedicated section's fixed kind/gain/slope are checked against its contract. Complete configurations use existing `InitialParameterValue`, 30 descriptors * two lanes = 60 rows; no new semantic alias table.

`maximum_targets()` is 12 for EQ. Coalesce by final `(section, lane)`: a section touched multiple times in a batch is designed once per final lane configuration. A single Both record is legal only if both lanes were touched and their entire final section configurations and designed words are bit-equal. Otherwise emit the required Left/Right records. A Both edit to frequency can still require two records when other values of that section already differ. Records are in section/channel order, with no overlapping selectors for one section. A batch touching N semantic EQ records emits at most 2N records and at most 12 per EQ; only touched sections emit records. Do not restart unrelated sections' ramps by sending all six on every edit. A disabled section's numeric edit still updates its semantic target even though its designed words remain identity.

`PreparedEffectTarget` is 56 bytes with its stated u32 representation; require the enclosing internal `EffectControlRecord` to remain <=64 bytes. It carries no Vec, Box, Arc, pointer, index into a recyclable external slab, or destructor. The simple enlarged queue record is accepted here to avoid a second publication/retirement protocol. Charge its actual size for every existing effect queue: today's 16-byte record becomes approximately 60 bytes, with `(depth+1)` physical slots; do not pretend this cost is EQ-only. Allocate target staging only for opted-in owners. A later compact tagged queue is optimization scope, not a prerequisite.

## Owner shadow and whole-batch admission

`EffectControlProducer` gains an optional control-side owner for this capability, seeded from its accepted `EffectBankPreparation.initial_values`, not from rendered current coefficients or an SDK default table. Implement the owner in `effect-compiler` (a small adjacent `control.rs` is appropriate): retained factory Arc, sample rate, committed and candidate boxed `InitialParameterValue` slices, same-length preallocated dirty flags, and u64 committed revision. For EQ these are exactly 60 rows. Keep one candidate tranche per owner; there is no speculative queue of mutable shadows. The semantic values are typed session parameters and can be projected through existing parameter/session facilities; do not parse/update JSON during render. This issue adds no general session snapshot API.

For the exclusive host-web owner, admission is:

1. Decode/validate **all** original wire records using existing shape/address/domain/channel/lattice rules. Preserve original wire indexes for refusals. Clone touched EQ committed values into preallocated candidate storage, apply the edits in wire order, and mark touched entries. Validate every supplied value even if a later value would replace it. Existing non-EQ and solo lowering participates in the same transaction.
2. Require exactly the corresponding companion target records for every touched EQ, and none for untargeted owners/sections. Check host generation, owner address and base revision, payload shape and numerical safety, and byte-equality of each target's six semantic words with the final candidate it claims to realize. No coefficient design happens here. Validate all targets before pushing any record. Validated full config and touched masks prevent omitted-target and overwrite-of-untouched-parameter attacks.
3. Count the coalesced prepared targets, ordinary records and existing solo fanout against every destination queue and the preallocated decoded staging. Existing `2*MAXIMUM_COMMAND_RECORDS + 2*track_count` decoded capacity remains sufficient: prepared EQ expansion is <=2 per original command. Check revision increment and every other fallible condition before publication.
4. Push the fully validated records under the existing **exclusive, between-block host ownership**. Consumer/render cannot interleave with this host-web call; all producers are private to it. Commit all EQ candidates/revisions and solo state only after all pushes succeed; publish the success report last. Expected failure changes no queue, committed shadow, revision or observation state. The existing internal push-failure branch is an invariant failure, not normal backpressure; it must never report partial success. Tests must prove preflight makes it unreachable for accepted input.

This is the browser/headless owner contract, not a claim of atomic cross-queue publication while an unrelated native render thread runs concurrently. Native users of the producer/preparer seam must admit while they exclusively own the plan between blocks. A concurrent native command endpoint requires its own admission envelope/scheduler and is outside this issue.

All runtime queues stay bounded at preparation. Add `Consumer::available_at_entry()` (one acquire cursor snapshot, modular occupancy <=capacity, no counter changes) or an equivalent frozen-drain API in `engine/realtime/spsc.rs`. `EffectControlLane::stage` freezes that count once and pops at most that many records. Never use the present producer-chasing `while try_pop` for the extended path. Later publications wait for the next stage entry. The caller supplies separate preallocated semantic-span and prepared-target staging; `Staged` reports both counts. Their combined count cannot exceed the queue's admitted entry count. An opted-in EQ queue may not stage a raw semantic Parameter record as a fallback.

## Browser and headless coefficient location

Both shipped frontends ultimately render the **host-web Wasm artifact**. `sdk/src/headless/engine.ts` uses `core/boundary.ts`; browser `miso-engine-v1-audio-worklet.js::receiveCommand` currently calls the same Rust admission export inside the worklet. Only headless's surrounding JS call is safely outside an audio callback thread.

Reuse #719's retained verified `WebAssembly.Module`, and the accepted response adapter's **analysis-only Wasm export pattern**. #719 deliberately terminates its rehearsal worker and disposes its scratch instance before returning; it cannot serve live edits. For the minimum browser implementation, instantiate that same compiled module once on the **main realm**, do not call boot/render, and use only the new stateless EQ target-preparation export. This is a small preparation workspace, not a second host: no session compile, DSP plan, source rings, rehearsal, response grid, audio processing, or effect roster is constructed. No new worker lifecycle is needed. The existing optional response Worker can consume the export later, but making a control path depend on an optional response subscription is excluded.

The default low-level `createMisoAudioWorkletHost` owns this helper, so SDK calls and direct low-level `host.command()` calls share it. Keep the small transport helper in one host-web JS module and ship/import it through the normal SDK asset pipeline; do not implement independent DSP or duplicate lowering in TypeScript. Main-realm preparation finishes before posting `miso.command.v1`. Worklet messages carry original 48-byte records plus the prepared companion; their renderer only validates/copies/adopts it. Neither `receiveCommand` nor Rust submit, `process`, or any bank process may call the design export.

Headless uses the identical stateless export on its existing Wasm instance **before** calling prepared admission, while it exclusively owns the instance between render calls. It needs no second Wasm instance and keeps its synchronous submit API. Both numeric and semantic console entry points and raw `WasmBoundary.submitCommands` perform this lowering. Host-web Rust tests can call the same preparation function directly outside render.

### Exact bounded preparation/companion bridge

Add a stateless `EqTargetWorkspace` in the existing host-web FFI workspace style, reached through a small `host-core` facade over the registry's EQ companion capability. Retain/prewarm the factory once in the helper; never reconstruct an all-effect registry per gesture. No effect-crate dependency is added to host-web. Root can place the facade in `crates/host-core/src/control_preparation.rs` and re-export it.

The FFI operation takes one EQ owner at a time: explicit launch sample rate, 60 seed f32 values in descriptor/lane order, and <=256 edit rows `{ parameter_id:u32, channel:u32, value:f32 }` (wire channel 0/1/2, translated to Rust enums). It validates seed and every edit through EQ's existing descriptors, applies edits in order in a stack/fixed candidate, and returns 60 final values plus <=12 `PreparedEffectTarget` records. No committed shadow is held by this operation. Require exact struct size/ABI version/counts/reserved zeros. Input/output buffers have fixed published capacities, pointer/byte-count accessors and explicit result counts. Define their exact C layouts in the ordinary generated ABI source when implemented; no JS offset literals outside that generated/shared boundary helper.

Seed the browser helper from **Rust's accepted owner shadow**. Add a read-only fixed-size addressed config-copy export: input `(handle, track_index, rack, effect_index)`; output sample rate, opaque current host-generation token, owner revision, and exactly 60 canonical f32 values. It performs only a bounded copy and rejects non-EQ/invalid owners. Bootstrap lazily on an owner's first EQ edit through one bounded worklet request using the existing request machinery; retain that accepted seed thereafter. One fixed 60-value output buffer serves all owners, so boot cost does not require a new all-owner snapshot allocation. Do not reparse the session in JS to guess sparse defaults or instance order.

A companion frame carries `{struct_size, abi_version, host_generation, target_count, reserved=0}` and <=512 fixed records (because the public batch ceiling is256). Each record carries address `(track_index, rack, effect_index)`, base owner revision, and the target `(slot, channel, words[12])`; use explicit-width fields, zero reserved/padding bytes, and the generated layout. Every owner's records have the same base revision. Total record cap <=512 and per-owner cap <=12 are enforced before copying or indexing. Define the former as `2 * MAXIMUM_COMMAND_RECORDS`, never as an independent magic limit: one semantic command can touch at most two lane/section targets and coalescing only reduces that count. It limits one submission, not tracks, EQ owners or session size; arbitrarily many resource-admitted owners may be addressed across submissions. Account for the exact `sizeof` rather than inventing an unlimited side channel. The worklet submission stages both arrays before one Rust admission call; it never acknowledges a header separately from its data.

Main helper state is a map of accepted seed arrays/revisions, one candidate per touched owner, and **at most one unsettled EQ-containing batch** for that live host. Refuse another EQ batch immediately with existing typed backpressure and zero admitted commands; do not grow an unbounded Promise chain. Non-EQ traffic can retain current behavior. Commit cached candidates only on a matching full successful admission ack, increment revisions once per touched owner, and discard candidates on any refusal. Preparation success is not admission. A transport failure or ambiguous ack invalidates the helper's affected seeds; it must re-copy the live Rust shadow before preparing again, never guess whether a batch applied. Existing fatal-host/timeout behavior remains authoritative. Dispose/reboot invalidates all seeds, pending work and generation tokens. Revision overflow refuses before mutation.

### Prepared-payload safety and trust boundary

The internal companion decoder checks every transported byte; trusted origin does not excuse malformed data. Transported targets are not accepted merely because all floats are finite. Factor the current rounded-SVF numerical validator out of `design_svf` so the same coefficient authority is used without invoking trigonometry. Require canonical zeros, finite bounded output-mix words, the current allowed `c1/a2/a3` shape and `word_spectral_norm <= NORM_TOLERANCE`; explicitly admit only the exact identity tuple for disabled targets. Require fixed-cut kind and mix invariants where directly known. Reject NaN/Inf, arbitrary huge finite mix words, unstable matrices, malformed bool/kind values, wrong sample rate/generation/revision, duplicate/overlapping selectors and altered untouched semantic fields. Derive and freeze a conservative output-mix bound from the existing legal EQ domains in the implementation record (the six-word designer's domain is bounded); no renderer transcendental is needed for this check. Test a domain grid of designer output against this validator and hostile finite payloads that violate it.

The official Rust preparer is the coefficient/semantic mapping authority. Audio-thread validation proves shape, equality of the supplied semantic header with the candidate shadow, and numerical safety. It **cannot prove that arbitrary supplied coefficient words implement that header's cutoff/Q**. A stable forged filter with the expected header can pass those checks. Do not describe `validate_targets` as coefficient/parameter equivalence validation, and do not claim that it rejects every semantic forgery.

The accepted boundary is therefore an **opaque engine-owned prepared batch**, not arbitrary raw prepared input. The public SDK, low-level `host.command()`, and supported raw-semantic-record facade accept semantic records only; their helper invokes the matched Rust preparer, owns the returned payload privately, binds it to the original records/host generation/revision, and transfers it unchanged. No caller-supplied sidecar, coefficients, callback-produced target, or imported "prepared batch" object is accepted by these entry points. The helper may represent the batch as a class with private fields or equivalent module-private storage; it must not return mutable target arrays to callers. The existing internal wire buffer is a transport detail. This needs no new cryptography, token service or coefficient framework.

Document the low-level companion export as an **internal trusted-host integration ABI**: the caller must supply unchanged output of the exact running artifact's Rust preparer for the same final candidate, generation and revision. This is a logical provenance precondition, not a Rust memory-safety `unsafe` condition and not a security boundary against code that can invoke arbitrary Wasm exports, mutate engine memory or replace the trusted host. A raw caller forging stable wrong words violates that precondition; the numerical validator does not turn its report into proof of a valid semantic command. If arbitrary caller-authored coefficient payloads are to become supported input, this design is insufficient and must be separately briefed; do not quietly broaden the public contract to them.

For every supported semantic path, ack and semantic shadow refer to the original records that produced the private payload; apply consumes that exact payload; response copies the actual retained applied target words and enabled flags, never a curve recomputed from the semantic shadow. This is the mechanism that prevents a silent semantic/readback/response split in this product contract. Preserve payload identity through the real browser/headless integration tests and compare captured words with the trusted preparer's output. A hash or echoed header alone would prove neither coefficient correctness nor DSP application and is not a substitute for those checks.

## Render application, symmetry and retained state

`EffectControlLane` folds each PreparedTarget into the existing `LiveConsoleRecord` symmetry hook when it drains. Left/Right clears LIVE; Both preserves it. Observe and bypass retain their existing semantics. Draining stages data and updates the witness; it does **not** yet call the EQ apply hook.

For scalar graph nodes: drain, then apply the validated prepared targets, then call process with the ordinary non-EQ automation staging as appropriate. For banks: `ConsoleEffectBankStage::begin_block/drain` stages per-lane targets and folds witnesses; the chain then makes its collapse decision and calls `desymmetrize_channels` if required; only inside `process_inner`, **after that restoration**, apply the staged per-lane targets, then render. This order is mandatory: current EQ desymmetrization copies coef/target/step/countdown from L to R and would erase a new R target applied in begin_block. Keep that proven copy intact rather than adding a new pending-target exception to it.

The EQ apply hook validates the small internal record shape before any write, updates the addressed stored BandTarget(s), withdraws `silent_fixed_point`, and calls existing `start_ramp` with predesigned words. It does not call `BandTarget::words`, `design_svf`, pow, tan, allocation, parsing or descriptor validation. Both starts both channels at the same application boundary; only lane-local words change. Coefficient arithmetic remains `(target-current)*2^-6`, the existing process-current-then-advance convention and exact final snap (`crates/lane/src/kernels.rs::svf_block_ramped` is the authority: sample A uses the current words, then advances; the target is used at A+64). This differs from the input trim ramp, which advances before applying its sample. Retarget uses current words. Existing ramp-aware identity gating must process identity-to-enabled transitions; no disabled elision may hide an in-flight ramp. Preserve existing NaN/denormal/signed-zero and integrator-reset rules.

Remove EQ's semantic-to-coefficient `automate` call from scalar and bank `process`. A factory opting into prepared targets declares that raw `EffectProcessBlock.automation` semantic points must first be lowered through its control preparer; malformed/unlowered direct EQ spans are counted/rejected without applying or designing. Change only EQ-specific direct conformance helpers and actual EQ producer callers to use the new seam; do not weaken other effects' automation gates or advertise this as an all-roster migration. External console semantics remain preserved through lowering. Native direct users can invoke the stateless preparer outside render and then apply targets at their scheduled block boundary.

Precompute/cache initial designed words per lane at prepare. `FullToDefaults` restores those words, initial semantic targets and cleared integrators without `Channel::new -> design_svf`; `DiscontinuityKeepParameters` retains the accepted semantic target, snaps existing ramps to target and clears integrators. A caller performing a full plan reset must reset its control shadow to the same initial configuration; browser/headless session replacement already creates a new owner/generation. No outstanding old-generation payload survives replacement.

The prepared-child state payload has 114 words/456 bytes per lane, but its 19-word section record omits enabled/kind and reconstructs them from preparation. Live dedicated enable makes that insufficient. Append exactly two u32 words after the six existing section records: current target HPF-enabled and LPF-enabled, encoded0/1. Live payload is **116 words/464 bytes per lane**, common8 bytes unchanged, prelaunch layout identity1 unchanged. Explicitly reject old456-byte lane payloads and malformed enable words before mutation. General-band kind/enabled remain prepared-only, so no further payload expansion is needed. Restore uses saved cut enable with saved numeric targets and validates designed target/state off render. Test a session initially disabled, live-enable, mid-ramp snapshot/restore, and later disable. Recount initial-word cache, real runtime storage and serialized storage separately.

Copied response uses retained `Section::target` words and the newly applied semantic enabled state at the same application boundary, preserving public response IDs1..6 and the physical-order mapping from the prepared child. Existing response mode is **target**, not instantaneous in-ramp response. Successful ack's `appliedAtSample=A` means the target begins its transition at A; it does not mean it is already settled. Before rendering the admitting block, a capture may correctly show the previous target; it must not be labeled fresh. After the block completes, require a copied snapshot with `capturedSample > A`, then verify its target words/configuration. At A+64 updates PCM reaches the new target exactly. If later submissions share A, last-wins coalescing means the latest acknowledged final target is what that boundary reports; do not promise audible intermediate configurations at the same sample. No #774 endpoint or new response grid is necessary.

## Bounded compiling implementation assignments

Astra XHIGH refined the assignments below to bound each Luna context. This changes sequencing, not the frozen product contract or the one-verdict-per-attempt rule.

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

## Mandatory acceptance evidence

- All four launch rates in inexpensive DSP tests: HPF/LPF/both enabled/disabled; representative frequency/Q extrema; finite output and unchanged four-band behavior. Native scalar, Simd4, Simd8/tails and dynamic/SIMD placement agree under current contracts; last LPF remains effective.
- Live enable from exact identity, disable to identity, sweep, same-target restatement and retarget at non-64-divisible block boundaries; partition invariance and exact64-update snap; unchanged sections' ramps are not restarted. Bypass preserves latency and target response.
- Left/Right/Both while the EQ bank is collapsed, including first asymmetric admission during a ramp. First dual block matches an always-dual oracle and new R targets survive desymmetrization. Both-only symmetric edits can keep the witness; both lanes of every admitted record apply.
- A mixed batch edits both cuts, old numeric rows, another effect and a fader. Invalid late command, stale epoch/revision, missing/extra/corrupted target and a full unrelated destination each refuse the whole batch without any shadow/queue/revision change. Success followed by another pre-render success designs the latter from the first accepted target. Rapid same-section edits coalesce but every original value is validated. Actual wire rejectedIndex remains an original command index.
- Freeze queue publication at entry; a producer publishing during drain cannot extend that drain. No acknowledged target is dropped by staging overflow. Queue/staging/owner/FFI/helper memory is fixed or configured and accounted for; no compiled track cap.
- Instrument the designer in tests: calls occur in main/control preparation; none in worklet admission, scalar/bank processing, mono transition, or reset. Run allocation/free counter gates over admission and render after prewarming. Inspect callback call reachability; a lexical absence of tan in a wrapper is insufficient.
- Malformed/unsafe transported target cases fail before admission: NaN/Inf, huge finite mix, finite unstable pole words, noncanonical disabled words, invalid fixed kind, incomplete/duplicate/overlapping records and altered untouched values. Valid official output across representative domains passes the numerical validator. A focused negative-contract test demonstrates that stable wrong coefficients can pass numerical validation; the public semantic facade still has no way to submit that forged payload. This keeps the stated trust boundary honest rather than overstating the validator.
- Browser and headless send identical semantic edits through the released facade and produce the same target payload, acknowledging exact application samples. Copy the actual retained response after the first applied block and compare target coefficients plus enabled flags; render beyond the ramp and verify PCM effect against the independent existing response/impulse authority. Preserve four general bands and one EQ owner; no hidden effects or builtin activation.
- Direct low-level browser host command and SDK semantic/numeric overloads use the same helper. Prepared module is reused without fetch/recompile when supplied; no host boot/render export is called on the helper instance. Busy/failed/stale submissions, dispose and session reload cannot advance speculative shadows or reuse stale seeds.
- 464-byte lane payload, malformed/old payload atomic refusal, live enabled persistence, mid-ramp restore, both reset kinds and response truthfulness. Native/Wasm builds, generated metadata/ABI and SDK packed artifacts pass. No timing benchmark is mandatory; any descriptive run follows the repository's frozen one-invocation policy.

## Decision/evidence record

Astra XHIGH design: this spec is ready for root to number and brief. Source evidence is the current effect-contract live queue and symmetry seam; parametric-eq automate/start_ramp/desymmetrize/reset/payload/response paths; effect-compiler EffectBankPreparation and producer attachment; graph ConsoleEffect; rack ConsoleEffectBankStage; host-web ReadyOwnership/admit_commands; browser receiveCommand; SDK WasmBoundary; and accepted #719/response analysis ownership. DSP equations and primary citations remain those of the existing rounded TPT SVF authority; this child changes preparation/application ownership, not the filter algorithm. Implementation attempt1 and Astra MEDIUM verdict: pending. Root records checkpoint hashes, gates, upstream issue synchronization and final verdict; no completion claim before remote delivery.

## Driver timing correction discovered during #805

Astra XHIGH found that the existing EQ Channel::process_section driver pre-advances and snaps when remaining==1, although the underlying lane ramp kernel processes current then advances. #805 preserves that existing original-band timing. #807 must explicitly correct the driver to its frozen current-then-advance contract: sample A uses current coefficients, all 64 updates occur, exact target first applies at A+64. Test retarget and uneven block partitions at that boundary. Reuse #805 additive masked helpers for dedicated settled-identity lanes; derive dry masks from exact current identity words and remaining==0, never target enabled while a ramp is in flight.

## Attempt 1 implementation baseline

Prepared-cut child #805 merged at 80f2918b5aba5b2428c5f5cc76c24f46b4e0edde with required qualification34820834881 PASS; #147 is included. The numbered-spec/GitHub boundary audit found no missing numbered issues; #805 is verified closed, and #807/#808/#809 remain open. Root starts bounded assignment1 on codex/807-live-eq from this merged baseline. Request/target/error types may derive Clone and Copy; borrowed request slices do not own heap state. No production capability opt-in until assignment10.

### Assignment 1 checkpoint — fixed handoff and queue snapshot

Luna XHIGH implemented the 56-byte Copy target, borrowed Copy request, small typed errors, optional factory capability and default Unsupported scalar/bank hooks. Consumer::available_at_entry takes one acquire snapshot and computes bounded modular occupancy without mutating cursors, caches or counters. Empty/full/wrapped cursor and later-publication cases are covered. Root reviewed the contract and clarified separation of off-audio design from bounded admission validation; preparation errors leave caller output unchanged.

Focused gates PASS: effect-contract46tests; SPSC6tests; existing Loom release model1test; native and Wasm checks of both affected crates; formatting and diff checks. No effect opts in; no queue enum enlargement, target admission, DSP application or live-cut claim occurs at this checkpoint. Assignments2–10 and the final fresh Astra MEDIUM adversarial verdict remain required.

### Assignment 2 checkpoint — actual EQ preparation and validator

Luna XHIGH implemented the real ParametricEqFactory companion over canonical60rows, retaining the production capability getter None until assignment10. It validates all semantic rows before output mutation, designs only touched final section/lane pairs into fixed stack storage, and emits ordered coalesced targets up to12. Validation checks exact headers, touched coverage, duplicate/overlapping/missing/extra selectors, canonical zero, exact disabled identity and enabled fixed cut mixes without redesigning. Root reviewed the mapping, coalescing, validation and adversarial cases.

The conservative output-mix bound is128: legal gain implies 1/4<A<4 and1/A<4; Q>=.1 and slope>=.1 imply shelf damping<sqrt74<9. High-shelf |m1|<108, low-shelf<27, bell<40, and remaining mix magnitudes<=16. This admits legal rounded outputs while refusing huge finite words. HPF exactmix is(1,round_f32(-1/Q),-1); LPF is(0,0,1). The enabled6dB-bell stable-forgery test deliberately changes cutoff coefficients while preserving the semantic header and demonstrates that numerical validation does not prove coefficient/semantic equivalence. The opaque matched-Rust-preparer provenance requirement remains mandatory.

Gates PASS: complete parametric-eq test suite (31unit plus analytic/bank/conformance/contract/determinism/mono/response/silent/stationary/time-domain tests, existing ignored tests unchanged), effect-contract46tests, Wasm compile, all-target EQ Clippy with warnings denied, fmt and diffcheck. Direct tests cover all6kinds at4launchrates, 12-target asymmetric capacity, shape/domain/selector refusal and sentinel rollback. No queue/apply/state/owner/host/SDK cutover occurs here. Assignments3–10, rebuilt final artifact and the final fresh Astra MEDIUM verdict remain pending.

### Assignment 3 checkpoint — DSP application, reset and state

Luna XHIGH implemented real scalar/bank prepared-target application, instance-local general-band kind/enable immutability, cached initial coefficients for reset, and current-then-advance ramp timing with exact target use at A+64. Dedicated enable state is appended as two u32 words per channel: 464 bytes per channel and 936 bytes total including the common 8-byte header. Old payload lengths and malformed enable words are refused atomically. Existing retained-response copying observes the applied semantic target and actual target words.

Root review corrected integer kind-word decoding and kept full semantic-domain validation on preparation/admission rather than repeating descriptor validation in the render hook. A focused Astra MEDIUM advisory identified restore edge cases: validate only coefficients actually processed before final snap, require exact zero steps on settled lanes, and use exact identity bits. These corrections are implemented; failed restores preserve the fixed-point witness and both channels. This advisory is not the final whole-issue verdict.

The cfg(test)-only designer counter proves prepared apply/render and cached resets do not design; it also detects the still-existing numeric semantic route, which remains intentionally unresolved until assignment10. The intentional timing correction changes only the EQ corpus ramped-noise pin, from c7d053d96d0810cf6eb9ecf820d68a738133d24ae34a27e4e8c90db390d50465 to 452fc6b5dc02f8fdb626aebd3eae8bea508f41d5f6338319828eca204419b79e. Static disabled-cut compatibility remains covered.

Final gates PASS: locked complete EQ tests (32 unit, 7 analytic, 6 bank, 1 conformance, 18 contract, 3 determinism, 3 mono, 18 response, 3 silent, 4 stationary, 5 time-domain; existing ignored tests unchanged); strict all-target EQ Clippy; Wasm compile; fmt and diff checks. Actual native Channel sizes at widths1/4/8 remain 656/2608/5216 bytes; PreparedParametricEq sizes are 2120/7760/15296 bytes. These resident object sizes exclude allocator overhead and are separate from the 936-byte serialized payload. No timed benchmark was run.

#### Independently derived graph accounting pin

The preserved #805 canonical fixture hashes to e9e6b012399cabe3632462622519bebac71e31af17813a31aa84d857209433e4. Nine EQs each add 936-920=16 declared-state bytes, hence exactly144 bytes: declared_effect_bytes 8280→8424, incremental_plan_bytes and session_plus_plan_bytes 150415→150559. Root constructed expected canonical text by changing only those three fields of the preserved final estimate row, retaining every other byte. Its SHA256 is eb3ca77606e93cf9aa13f475415ecbf6e70ee1cdd074a0cca9e46cbb18e0ea10, exactly matching the current compiler's independently observed result. The graph test pin/comment now records this derivation; all eight graph-compiler track_delay tests pass, preserving structural/no-delay/PDC/resource gates. No production graph source changed.

Queue delivery, owner/admission, browser/headless wiring, capability activation, rebuilt final artifact and the final fresh Astra MEDIUM review remain pending. No public live-cut or release completion is claimed at this checkpoint.


### Account-switch pause — 2026-09-14

Implementation is paused at the user's request for another Codex account to resume. All source checkpoints are pushed on `codex/807-live-eq`, through `e81df55b61f563fa26952608e7383d3c060cb956`. #147 and #805 are merged/closed; #807 assignments 1–3 are complete, assignments 4–10 remain. #808, #809, adapter #111 and app #222 have not entered implementation/release. No public live-cut activation, package publication or app deployment is claimed.

The self-contained [resume handoff](https://github.com/misofm/engine/blob/codex/807-live-eq/docs/handoffs/804-account-switch-2026-09-14/README.md) records branches, exact commits, accepted decisions, test evidence, preserved plans, downstream requirements and the next bounded task. All agents are stopped. Before assignment 4, freeze its staging layout and amend the brief using the linked handoff's Astra XHIGH queue/accounting recommendation; no assignment-4 source has been written. #807 remains attempt 1 and still requires its final fresh Astra MEDIUM review. Keep this issue open.
