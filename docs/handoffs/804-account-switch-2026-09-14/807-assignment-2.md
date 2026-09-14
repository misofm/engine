# #807 assignment2: actual EQ preparation and numerical validation

Read applicable AGENTS.md. User approved Astra XHIGH design and Luna XHIGH bounded implementation. Root owns commits/spec updates; stop at focused green. No production getter registration untilassignment10.

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


### Prepared-payload safety and trust boundary

The internal companion decoder checks every transported byte; trusted origin does not excuse malformed data. Transported targets are not accepted merely because all floats are finite. Factor the current rounded-SVF numerical validator out of `design_svf` so the same coefficient authority is used without invoking trigonometry. Require canonical zeros, finite bounded output-mix words, the current allowed `c1/a2/a3` shape and `word_spectral_norm <= NORM_TOLERANCE`; explicitly admit only the exact identity tuple for disabled targets. Require fixed-cut kind and mix invariants where directly known. Reject NaN/Inf, arbitrary huge finite mix words, unstable matrices, malformed bool/kind values, wrong sample rate/generation/revision, duplicate/overlapping selectors and altered untouched semantic fields. Derive and freeze a conservative output-mix bound from the existing legal EQ domains in the implementation record (the six-word designer's domain is bounded); no renderer transcendental is needed for this check. Test a domain grid of designer output against this validator and hostile finite payloads that violate it.

The official Rust preparer is the coefficient/semantic mapping authority. Audio-thread validation proves shape, equality of the supplied semantic header with the candidate shadow, and numerical safety. It **cannot prove that arbitrary supplied coefficient words implement that header's cutoff/Q**. A stable forged filter with the expected header can pass those checks. Do not describe `validate_targets` as coefficient/parameter equivalence validation, and do not claim that it rejects every semantic forgery.

The accepted boundary is therefore an **opaque engine-owned prepared batch**, not arbitrary raw prepared input. The public SDK, low-level `host.command()`, and supported raw-semantic-record facade accept semantic records only; their helper invokes the matched Rust preparer, owns the returned payload privately, binds it to the original records/host generation/revision, and transfers it unchanged. No caller-supplied sidecar, coefficients, callback-produced target, or imported "prepared batch" object is accepted by these entry points. The helper may represent the batch as a class with private fields or equivalent module-private storage; it must not return mutable target arrays to callers. The existing internal wire buffer is a transport detail. This needs no new cryptography, token service or coefficient framework.

Document the low-level companion export as an **internal trusted-host integration ABI**: the caller must supply unchanged output of the exact running artifact's Rust preparer for the same final candidate, generation and revision. This is a logical provenance precondition, not a Rust memory-safety `unsafe` condition and not a security boundary against code that can invoke arbitrary Wasm exports, mutate engine memory or replace the trusted host. A raw caller forging stable wrong words violates that precondition; the numerical validator does not turn its report into proof of a valid semantic command. If arbitrary caller-authored coefficient payloads are to become supported input, this design is insufficient and must be separately briefed; do not quietly broaden the public contract to them.

For every supported semantic path, ack and semantic shadow refer to the original records that produced the private payload; apply consumes that exact payload; response copies the actual retained applied target words and enabled flags, never a curve recomputed from the semantic shadow. This is the mechanism that prevents a silent semantic/readback/response split in this product contract. Preserve payload identity through the real browser/headless integration tests and compare captured words with the trusted preparer's output. A hash or echoed header alone would prove neither coefficient correctness nor DSP application and is not a substitute for those checks.


### 2. Real EQ preparation and numerical validation

Primary files: `crates/parametric-eq/src/control.rs` (new), `crates/parametric-eq/src/lib.rs`.

Implement the actual factory companion trait and six-word semantic/six-word coefficient target preparation, whole final configuration validation, touched-section coalescing, ordered Left/Right/Both output and the rounded-SVF numerical validator shared with the existing designer. Keep the production factory getter unregistered until assignment 10; direct tests invoke the concrete companion implementation. Derive the accepted finite output-mix bound from the legal EQ domains and record that decision in the numbered issue through root.

Glue: module declarations and narrowly affected existing EQ designer tests. Gate: representative launch-rate/domain grid, invalid-edit and unchanged-output-on-error tests, identity exactness, section coalescing, asymmetric Both cases, capacity tests and unsafe finite target rejection. Retain the explicit stable-forgery negative-contract test; do not claim the numerical validator proves cutoff/Q equivalence. Deliberately incomplete: a producer cannot yet obtain this capability through the production registry, and no host live-cut claim follows.


Assignment1 supplies real types/default hooks. Use actual signatures from effect-contract new prepared_target module, not duplicate types. Whole final candidate60values and changed60; per-row descriptor/lane canonical order; derive mixedBoth coalescing from all six semantic fields. Do not mutate out on any error; design into fixed stack temp then copy once valid/capacitychecked. All sample rates supported exactly44.1/48/88.2/96k. Preserve existing designer arithmetic while extracting numerical validator; primary citations alreadyinEQdocs. No applyhook, reset/state, host/shadow/SDK edits in this slice. Keep source primarycontrol.rs and minimal lib.rs extraction; tests in module or narrow existing EQ tests only. Evidence /tmp/807-assignment-2-evidence.md.

Root preliminary bound derivation to verify independently: gain ±24 gives 1/4 < A < 4 and 1/A <4; Q>=.1, slope>=.1. Conservative shelf k<sqrt((4+4)*9+2)=sqrt74<9. Hence high-shelf |m1|=k*|1-A|*A<9*3*4=108, low-shelf |m1|<27, bell |m1|=|A-1/A|/Q<40, and other mix magnitudes<=16. A frozen128 f32 mix bound leaves ample rounding room without accepting huge finite coefficients. Check all cases and preserve disabled exactidentityexception separately; document any correction to this derivation.
