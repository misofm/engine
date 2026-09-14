# Assignment 1: #807 fixed target contract and queue snapshot

Implement only this bounded slice in the worktree root supplies. Read AGENTS.md and keep source APIs documented. User requests Luna XHIGH implementation; Astra XHIGH approved this spec. No commits or pushes; root owns exact-path checkpoints. Stop after focused gates. No subsequent assignment edits.

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


IMPORTANT OVERRIDE FOR INTERMEDIATE CHECKPOINT: the enum variant in the full spec above is deferred to assignment4. DO NOT add it now.

### 1. Fixed target contract and bounded queue snapshot

Primary files: `crates/effect-contract/src/prepared_target.rs` (new small module), `crates/effect-contract/src/lib.rs`, `crates/engine/src/realtime/spsc.rs`.

Implement the frozen 12-word Copy target/request/error types and defaulted factory/scalar/bank hooks, plus `Consumer::available_at_entry()` or the equivalent frozen-count primitive. Keep internal names unversioned and existing non-EQ implementations on real `None`/`Unsupported` defaults. **Do not append `EffectControlRecord::PreparedTarget` yet**: introduce that variant with its complete staging and application consumers in assignment 4, avoiding an interim exhaustive arm that drops it.

Glue: existing contract exports and default-hook conformance tests only. No other effect implementation files. Gate: effect-contract and SPSC focused tests, target-size/Copy assertions and native/Wasm checks of the affected crates. Prove the snapshot count is bounded and unaffected by later publication. Deliberately incomplete: enlarged queue accounting, target staging and any production prepared-target admission.


Queue snapshot: one acquire producer cursor snapshot, consumer-local cursor load, modular occupancy <= logical capacity, no counter mutation. Later publication cannot enlarge the returned count. Respect existing loom synchronization wrappers. Test empty/full/wrapped cursor cases and frozen returned count after producer pushes additional records; no producer-chasing behavior.

Known trait positions before this slice: effect-contract lib.rs NativeEffectFactory around1470, PreparedNativeEffect1583, PreparedNativeEffectBank1719. Read targeted definitions, not every effect source. Primary files only new prepared_target.rs, contract lib.rs, engine realtime/spsc.rs; necessary direct tests may use existing test module.

Report exact paths, command outputs/logs, no-allocation type facts, and remaining gates candidly. No new effect capability is registered, no enlarged queue published, no live-cut completion claim.
