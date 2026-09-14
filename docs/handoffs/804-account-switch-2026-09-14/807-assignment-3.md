# #807 assignment3: apply, reset cache and464-byte lane state

User workflow AstraXHIGH design/LunaXHIGH bounded implementation/finalAstraMEDIUM. ReadAGENTS. Rootowns checkpoints/spec. Do not edituntilrootexplicitlyauthorizes afterassignment2 checkpoint.

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


## Render application, symmetry and retained state

`EffectControlLane` folds each PreparedTarget into the existing `LiveConsoleRecord` symmetry hook when it drains. Left/Right clears LIVE; Both preserves it. Observe and bypass retain their existing semantics. Draining stages data and updates the witness; it does **not** yet call the EQ apply hook.

For scalar graph nodes: drain, then apply the validated prepared targets, then call process with the ordinary non-EQ automation staging as appropriate. For banks: `ConsoleEffectBankStage::begin_block/drain` stages per-lane targets and folds witnesses; the chain then makes its collapse decision and calls `desymmetrize_channels` if required; only inside `process_inner`, **after that restoration**, apply the staged per-lane targets, then render. This order is mandatory: current EQ desymmetrization copies coef/target/step/countdown from L to R and would erase a new R target applied in begin_block. Keep that proven copy intact rather than adding a new pending-target exception to it.

The EQ apply hook validates the small internal record shape before any write, updates the addressed stored BandTarget(s), withdraws `silent_fixed_point`, and calls existing `start_ramp` with predesigned words. It does not call `BandTarget::words`, `design_svf`, pow, tan, allocation, parsing or descriptor validation. Both starts both channels at the same application boundary; only lane-local words change. Coefficient arithmetic remains `(target-current)*2^-6`, the existing process-current-then-advance convention and exact final snap (`crates/lane/src/kernels.rs::svf_block_ramped` is the authority: sample A uses the current words, then advances; the target is used at A+64). This differs from the input trim ramp, which advances before applying its sample. Retarget uses current words. Existing ramp-aware identity gating must process identity-to-enabled transitions; no disabled elision may hide an in-flight ramp. Preserve existing NaN/denormal/signed-zero and integrator-reset rules.

Remove EQ's semantic-to-coefficient `automate` call from scalar and bank `process`. A factory opting into prepared targets declares that raw `EffectProcessBlock.automation` semantic points must first be lowered through its control preparer; malformed/unlowered direct EQ spans are counted/rejected without applying or designing. Change only EQ-specific direct conformance helpers and actual EQ producer callers to use the new seam; do not weaken other effects' automation gates or advertise this as an all-roster migration. External console semantics remain preserved through lowering. Native direct users can invoke the stateless preparer outside render and then apply targets at their scheduled block boundary.

Precompute/cache initial designed words per lane at prepare. `FullToDefaults` restores those words, initial semantic targets and cleared integrators without `Channel::new -> design_svf`; `DiscontinuityKeepParameters` retains the accepted semantic target, snaps existing ramps to target and clears integrators. A caller performing a full plan reset must reset its control shadow to the same initial configuration; browser/headless session replacement already creates a new owner/generation. No outstanding old-generation payload survives replacement.

The prepared-child state payload has 114 words/456 bytes per lane, but its 19-word section record omits enabled/kind and reconstructs them from preparation. Live dedicated enable makes that insufficient. Append exactly two u32 words after the six existing section records: current target HPF-enabled and LPF-enabled, encoded0/1. Live payload is **116 words/464 bytes per lane**, common8 bytes unchanged, prelaunch layout identity1 unchanged. Explicitly reject old456-byte lane payloads and malformed enable words before mutation. General-band kind/enabled remain prepared-only, so no further payload expansion is needed. Restore uses saved cut enable with saved numeric targets and validates designed target/state off render. Test a session initially disabled, live-enable, mid-ramp snapshot/restore, and later disable. Recount initial-word cache, real runtime storage and serialized storage separately.

Copied response uses retained `Section::target` words and the newly applied semantic enabled state at the same application boundary, preserving public response IDs1..6 and the physical-order mapping from the prepared child. Existing response mode is **target**, not instantaneous in-ramp response. Successful ack's `appliedAtSample=A` means the target begins its transition at A; it does not mean it is already settled. Before rendering the admitting block, a capture may correctly show the previous target; it must not be labeled fresh. After the block completes, require a copied snapshot with `capturedSample > A`, then verify its target words/configuration. At A+64 updates PCM reaches the new target exactly. If later submissions share A, last-wins coalescing means the latest acknowledged final target is what that boundary reports; do not promise audible intermediate configurations at the same sample. No #774 endpoint or new response grid is necessary.


### 3. EQ application, reset cache and 464-byte state

Primary files: `crates/parametric-eq/src/lib.rs`, `crates/parametric-eq/src/response.rs`; `control.rs` only if small shared decoding belongs there.

Implement the real scalar/bank prepared-target application hooks, retained semantic enable state, initial coefficient cache for reset, ramp retargeting and response copying. Extend the per-lane state to the frozen 116 words/464 bytes, with atomic refusal of old/malformed payloads and correct live-enable restore. Keep the existing semantic `automate` process route temporarily reachable for existing production numeric controls; its removal belongs to cutover. Direct tests prepare outside render, apply the real hooks at a block boundary and render the resulting targets.

Glue: existing EQ-specific direct DSP/response/state fixtures and state-size expectations. Gate: direct scalar/bank target application, exact identity transitions, A/current-then-advance and A+64 settling, retarget/partition checks, independent lanes, cached reset without design, retained response and mid-ramp state roundtrip. Designer instrumentation must distinguish this new route from the still-present legacy numeric route. Deliberately incomplete: graph/rack queue delivery, production lowering and the global no-design-in-process claim.


## Driver timing correction discovered during #805

Astra XHIGH found that the existing EQ Channel::process_section driver pre-advances and snaps when remaining==1, although the underlying lane ramp kernel processes current then advances. #805 preserves that existing original-band timing. #807 must explicitly correct the driver to its frozen current-then-advance contract: sample A uses current coefficients, all 64 updates occur, exact target first applies at A+64. Test retarget and uneven block partitions at that boundary. Reuse #805 additive masked helpers for dedicated settled-identity lanes; derive dry masks from exact current identity words and remaining==0, never target enabled while a ramp is in flight.

## Attempt 1 implementation baseline

Prepared-cut child #805 merged at 80f2918b5aba5b2428c5f5cc76c24f46b4e0edde with required qualification34820834881 PASS; #147 is included. The numbered-spec/GitHub boundary audit found no missing numbered issues; #805 is verified closed, and #807/#808/#809 remain open. Root starts bounded assignment1 on codex/807-live-eq from this merged baseline. Request/target/error types may derive Clone and Copy; borrowed request slices do not own heap state. No production capability opt-in until assignment10.

### Assignment 1 checkpoint — fixed handoff and queue snapshot

Luna XHIGH implemented the 56-byte Copy target, borrowed Copy request, small typed errors, optional factory capability and default Unsupported scalar/bank hooks. Consumer::available_at_entry takes one acquire snapshot and computes bounded modular occupancy without mutating cursors, caches or counters. Empty/full/wrapped cursor and later-publication cases are covered. Root reviewed the contract and clarified separation of off-audio design from bounded admission validation; preparation errors leave caller output unchanged.

Focused gates PASS: effect-contract46tests; SPSC6tests; existing Loom release model1test; native and Wasm checks of both affected crates; formatting and diff checks. No effect opts in; no queue enum enlargement, target admission, DSP application or live-cut claim occurs at this checkpoint. Assignments2–10 and the final fresh Astra MEDIUM adversarial verdict remain required.

Root forward integration note:920->936 total serializedstate changes9-EQ graph canonicalresourceestimate by+144. Existing graph-compiler/tests/track_delay.rs must retainindependentcanonical derivation ifnewstate moves its pin; obtain baseline/currenttext exactdiff and update narrowtest/comment samecheckpoint. Do not touchcatalogcount69/30rows/6response shapes—theyremain. RootwillprovideactualbaselineSHAafterassignment2. Preserveoldnumericproductionroute untilassignment10; nofactorygetter/capabilityadvertisement now.
