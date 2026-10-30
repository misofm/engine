use effect_contract::{
    EffectCellWriter, EffectControlLane, EffectControlRecord, EffectDescriptor, EffectLaneLayout,
    EffectLaneShape, EffectLaneSpec, EffectQuality, EffectWriteRefusal, InitialParameterValue,
    LinkMode, NativeEffectFactory, NativeEffectRegistry, ObservationLane, ObservePushError,
    ParameterChannel, ParameterChannelPolicy, ParameterUnit, PrepareEffectLimits,
    PrepareEffectRequest, PreparedEffect, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, RegisteredTailBound, RegistryError, effect_control_lane,
    expected_prepared_metadata,
};
use engine::realtime::{LiveStamp, LiveValuesSuperseded, ObservationReader, observation_slot};
use session::{
    CompiledSession, ConsoleLinkMode, ConsoleSlot, Effect, EffectIdentity,
    LinkMode as SessionLinkMode, ParameterChannel as SessionChannel, ParameterUnit as SessionUnit,
    SessionModel, SidechainDeclaration, StripRef,
};
use std::collections::BTreeMap;
use std::sync::{Arc, OnceLock};

use crate::control::{EffectControlOwner, EffectControlOwnerError, EffectControlResourceError};
use crate::{EffectDiagnostic, EffectDiagnosticSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectCompileCaps {
    /// Ceiling on one instance's declared state ([`PrepareEffectLimits`]). A session's sum is a
    /// host's aggregate budget, projected with [`project_session_effects`] (issue #2194).
    pub maximum_total_state_bytes: u64,
    /// Ceiling on one instance's declared scratch.
    pub maximum_scratch_bytes: u64,
    /// Ceiling on the descriptor-derived per-instance capacity; never a requested window size.
    pub maximum_automation_spans_per_block: u32,
}
/// One native effect instance of a session, resolved, validated and designed, with no processor
/// prepared (issue #2199): what the graph compiler projects a session's graph from and plans its
/// banks over. [`EffectPreparedEntry`] is this with the instance's prepared processor.
pub struct EffectProjectedEntry {
    pub track_id: String,
    /// The owning strip's compiler path prefix (`StripRef::path_prefix`): `$.tracks[id=<id>]` for
    /// a track. Diagnostics about this instance are `"{strip_path}.effects[id={effect}]"`.
    pub strip_path: Box<str>,
    pub rack: EffectRack,
    pub effect_id: String,
    /// The metadata the instance's request designs ([`expected_prepared_metadata`]): the prepared
    /// processor's, which [`Self::prepare`] holds it to (`effect.metadata.mismatch`).
    pub metadata: PreparedEffectMetadata,
    /// The factory's [`designed_channel_symmetry`](NativeEffectFactory::designed_channel_symmetry)
    /// for [`Self::bank_preparation`]'s request: the `DESIGNED` term of the instance's pool class,
    /// read before any processor exists (issue #2199). [`Self::prepare`] holds the prepared
    /// processor's `channel_symmetry` to it.
    pub designed_symmetry: bool,
    /// Factory retained only for transactional off-render bank binding.
    pub factory: Arc<dyn effect_contract::NativeEffectFactory>,
    /// Exact owned request inputs used to prepare the scalar processor.
    ///
    /// Its `bypass` is always `false`: every session bypass is lowered to
    /// [`Self::initial_bypass`] (issues #1087, #1339, #1340).
    pub bank_preparation: EffectBankPreparation,
    /// The session's `bypass` for this instance (issue #1087).
    ///
    /// It is lowered to per-lane shunt state: the effect is prepared with `bypass = false`, so a
    /// bypassed and an enabled instance of one effect share one `EffectProgramKey` and one bank,
    /// and this bit is the initial state of the instance's [`EffectControlLane`], and so of the
    /// rack's latency-preserving shunt. A bypassed
    /// instance runs its wet path and emits its input delayed by its declared latency. It also
    /// seeds a live-control lane ([`attach_effect_live_controls`]) for every effect.
    pub initial_bypass: bool,
    /// The consumer half of this instance's live-control channel (issue #140 A), or the
    /// channel-less lane that carries a lowered session bypass (issue #1087).
    ///
    /// [`prepare_native_session_effects`] sets
    /// [`EffectControlLane::without_channel`]`(true)` on every bypassed instance and `None` on
    /// every other, and [`attach_effect_live_controls`]
    /// replaces it with a live channel seeded from [`Self::initial_bypass`]; nothing else creates
    /// one. It travels with the entry into `GraphPreparedEffect`, so the plan that renders the
    /// effect is the one that reads its cells, drains its observation queue and applies its shunt,
    /// and a session with no live controls and no bypassed instance carries a `None` that the
    /// runtime turns back into the byte-identical live-control-free path.
    pub control: Option<Box<EffectControlLane>>,
    /// This instance's observation taps (issue #143 D3, level 1).
    ///
    /// `None` unless [`attach_effect_observation`] was called, which is the only way one is
    /// ever created. A session whose live-control request named no observation capacity carries
    /// `None` here, and the runtime turns that back into the byte-identical unobserved path: there
    /// is no lane, no slot and no vector anywhere in the compiled plan.
    pub observation: Option<Box<ObservationLane>>,
    /// The layout of the live-control lane [`plan_projected_effect_live_controls`] planned for this
    /// instance, before the lane is attached (issue #2199), or `None` when none was planned. The
    /// graph's estimate charges it in place of [`Self::control`]; the lane attached afterwards must
    /// have exactly this layout.
    pub planned_lane: Option<EffectLaneLayout>,
}

impl EffectProjectedEntry {
    /// The live-control lane the graph's estimate charges for this instance: the planned one when
    /// a lane was planned, else the attached one's ([`EffectControlLane::layout`]), else none.
    #[must_use]
    pub fn lane_layout(&self) -> Option<EffectLaneLayout> {
        self.planned_lane
            .or_else(|| self.control.as_deref().map(EffectControlLane::layout))
    }

    /// What this instance's prepared processor retains beyond its declared state and scratch, at
    /// most: its factory's inline bytes and its undeclared heap at the instance's quality (root
    /// ruling R1 of #2194; `effect-compiler/tests/prepared_state_ceiling.rs`). A host charges it
    /// only for an instance its bank plan leaves per node, since a banked member gets no processor
    /// (issue #2219).
    /// `None` when the descriptor lacks the instance's rate and quality (its projection resolved
    /// them, so a projected entry never answers `None`) or the sum overflows.
    #[must_use]
    pub fn processor_bytes(&self) -> Option<u64> {
        let declared = effect_contract::declared_instance_resources(
            self.factory.descriptor(),
            self.bank_preparation.quality,
            self.bank_preparation.sample_rate,
            self.bank_preparation.quantum,
        )
        .ok()?;
        self.factory.prepared_inline_bytes().checked_add(
            self.factory
                .prepared_undeclared_heap_bytes(&declared.quality),
        )
    }
}

/// One prepared native effect instance: its [`EffectProjectedEntry`] and its prepared processor.
///
/// It dereferences to the projected entry, so its identity, metadata, factory, request and lanes
/// read and write as the projected entry's.
pub struct EffectPreparedEntry {
    pub projected: EffectProjectedEntry,
    pub processor: Box<dyn PreparedNativeEffect>,
}

impl core::ops::Deref for EffectPreparedEntry {
    type Target = EffectProjectedEntry;

    fn deref(&self) -> &EffectProjectedEntry {
        &self.projected
    }
}

impl core::ops::DerefMut for EffectPreparedEntry {
    fn deref_mut(&mut self) -> &mut EffectProjectedEntry {
        &mut self.projected
    }
}

impl EffectProjectedEntry {
    /// The diagnostic path of this instance: `"{strip_path}.effects[id={effect}]"`.
    #[must_use]
    pub fn path(&self) -> String {
        format!("{}.effects[id={}]", self.strip_path, self.effect_id)
    }

    /// Prepare this instance's processor from its request, off the render thread (issue #2199).
    ///
    /// The processor's metadata must be [`Self::metadata`], field by field, and its
    /// `channel_symmetry` must be [`Self::designed_symmetry`]; either difference is
    /// `effect.metadata.mismatch` at [`Self::path`], as is a factory refusal of a request its own
    /// projection accepted (with the factory's code).
    ///
    /// # Errors
    ///
    /// The factory's refusal, or `effect.metadata.mismatch`, at this instance's path.
    pub fn prepare(&self) -> Result<Box<dyn PreparedNativeEffect>, EffectDiagnostic> {
        let PreparedEffect {
            processor,
            metadata,
        } = self
            .factory
            .prepare(self.bank_preparation.request())
            .map_err(|error| EffectDiagnostic {
                code: error.code,
                path: self.path(),
            })?;
        let expected = &self.metadata;
        // Issue #1461: the prepare result carries the metadata beside the processor, which holds
        // no copy of it; this compares that returned value, field by field.
        if metadata.descriptor.id != expected.descriptor.id
            || metadata.descriptor.contract_major != expected.descriptor.contract_major
            || metadata.descriptor.state_layout_version != expected.descriptor.state_layout_version
            || metadata.sample_rate != expected.sample_rate
            || metadata.quantum != expected.quantum
            || metadata.quality != expected.quality
            || metadata.bypass != expected.bypass
            || metadata.link_mode != expected.link_mode
            || metadata.ports != expected.ports
            || metadata.latency != expected.latency
            || metadata.tail != expected.tail
            || metadata.tail_every_peak != expected.tail_every_peak
            || metadata.rest != expected.rest
            || metadata.composition != expected.composition
            || metadata.state_sizes != expected.state_sizes
            || metadata.scratch_bytes != expected.scratch_bytes
            || metadata.automation_capacity != expected.automation_capacity
            || processor.channel_symmetry() != self.designed_symmetry
        {
            return Err(EffectDiagnostic {
                code: "effect.metadata.mismatch",
                path: self.path(),
            });
        }
        Ok(processor)
    }

    /// This instance with `processor`, prepared from its request.
    #[must_use]
    pub fn with_processor(self, processor: Box<dyn PreparedNativeEffect>) -> EffectPreparedEntry {
        EffectPreparedEntry {
            projected: self,
            processor,
        }
    }
}

/// Owned replayable portion of an accepted prepare request. It never crosses into render.
#[derive(Clone, Debug)]
pub struct EffectBankPreparation {
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPorts,
    pub initial_values: Box<[InitialParameterValue]>,
    pub limits: PrepareEffectLimits,
    /// The registry's tail-bound entry for this rate and quality (issue #1462 D1), read once
    /// here and replayed into every request, so binding a bank never evaluates the descriptor.
    pub tail_bound: RegisteredTailBound,
}

impl EffectBankPreparation {
    #[must_use]
    pub fn request(&self) -> PrepareEffectRequest<'_> {
        PrepareEffectRequest {
            sample_rate: self.sample_rate,
            quantum: self.quantum,
            quality: self.quality,
            bypass: self.bypass,
            link_mode: self.link_mode,
            ports: self.ports,
            initial_values: &self.initial_values,
            limits: self.limits,
            tail_bound: self.tail_bound,
        }
    }
}

/// The internal rack an effect instance was lowered into (decision 12, class A by lowering):
/// `Simd1` holds the session's `console.pre_insert` slots, `Dynamic` the track's `inserts` and
/// `Simd2` the `console.post_insert` slots.
///
/// This is the graph's identity for a prepared instance, never a live-control address: live
/// controls name an instance by its [`LiveEffectAddress`], in the session's own terms, and
/// [`LiveEffectAddress::lower`] is the one translation back (issue #1096).
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EffectRack {
    Simd1,
    Dynamic,
    Simd2,
}

/// The rack a live control names, in the session's vocabulary (decision 12, issue #1096).
///
/// The session addresses a console slot as `rack: "console"` and an insert as
/// `rack: "inserts"`. The builtins chassis has no effect index and is not a live effect rack.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LiveEffectRack {
    /// A session console slot, in either section.
    Console,
    /// One of the track's inserts.
    Inserts,
}

/// Where a live control addresses one effect instance of a track (decision 12, issue #1096).
///
/// - A console slot is `(console, slot index)`. The slot index is the slot's position in the
///   session's slot order, `pre_insert` then `post_insert`, which is exactly its index in the
///   track's `console` array. Like the automation target `(track, console, slot)`, the address
///   does not name the section (L6).
/// - An insert is `(inserts, index)`, its position in the track's `inserts`.
///
/// This is the `(rack, effect_index)` a host looks a channel up by ([`EffectControlProducer::address`]); a `SET_CONTROL` edit or an
/// `Observe` edit names the instance by its stable ID, which the core resolves to it. Internally
/// it maps through the lowering ([`Self::lower`]).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LiveEffectAddress {
    /// The session rack.
    pub rack: LiveEffectRack,
    /// The console slot index or the insert index.
    pub index: u32,
}

impl LiveEffectAddress {
    /// A console slot, by its position in the session's slot order (`pre_insert`, then
    /// `post_insert`).
    #[must_use]
    pub const fn console(slot_index: u32) -> Self {
        Self {
            rack: LiveEffectRack::Console,
            index: slot_index,
        }
    }

    /// One of the track's inserts, by its position in `inserts`.
    #[must_use]
    pub const fn insert(index: u32) -> Self {
        Self {
            rack: LiveEffectRack::Inserts,
            index,
        }
    }

    /// Lower this address to the internal rack and the position within it.
    ///
    /// `lengths` are one track's lowered rack lengths in chain order, `[pre_insert, inserts,
    /// post_insert]`, as [`session::LoweredRacks::in_chain_order`] gives them. Console slot `k`
    /// is `(Simd1, k)` while `k` is below the `pre_insert` length and `(Simd2, k - pre_insert)`
    /// from there; insert `i` is `(Dynamic, i)`. `None` when the address names no instance.
    #[must_use]
    pub const fn lower(self, lengths: [u32; 3]) -> Option<(EffectRack, u32)> {
        let [pre_insert, inserts, post_insert] = lengths;
        match self.rack {
            LiveEffectRack::Inserts if self.index < inserts => {
                Some((EffectRack::Dynamic, self.index))
            }
            LiveEffectRack::Console if self.index < pre_insert => {
                Some((EffectRack::Simd1, self.index))
            }
            LiveEffectRack::Console if self.index - pre_insert < post_insert => {
                Some((EffectRack::Simd2, self.index - pre_insert))
            }
            LiveEffectRack::Console | LiveEffectRack::Inserts => None,
        }
    }
}
pub struct EffectPreparedSession {
    pub session: CompiledSession,
    pub entries: Vec<EffectPreparedEntry>,
}

/// A session's native effect instances, projected and not prepared (issue #2199): the first half
/// of [`prepare_native_session_effects`], in its entry order.
pub struct EffectProjectedSession {
    pub session: CompiledSession,
    pub entries: Vec<EffectProjectedEntry>,
}

impl EffectProjectedSession {
    /// Prepare every instance's processor ([`EffectProjectedEntry::prepare`]), in entry order.
    ///
    /// # Errors
    ///
    /// Every instance's preparation diagnostic, sorted.
    pub fn prepare(self) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
        let mut diagnostics = Vec::new();
        let mut prepared = Vec::with_capacity(self.entries.len());
        for entry in self.entries {
            match entry.prepare() {
                Ok(processor) => prepared.push(entry.with_processor(processor)),
                Err(diagnostic) => diagnostics.push(diagnostic),
            }
        }
        if diagnostics.is_empty() {
            Ok(EffectPreparedSession {
                session: self.session,
                entries: prepared,
            })
        } else {
            Err(EffectDiagnosticSet::sorted(diagnostics))
        }
    }
}

impl EffectPreparedSession {
    /// The projected session and each entry's processor, in entry order (issue #2199).
    #[must_use]
    pub fn into_projected(self) -> (EffectProjectedSession, Vec<Box<dyn PreparedNativeEffect>>) {
        let (entries, processors) = self
            .entries
            .into_iter()
            .map(|entry| (entry.projected, entry.processor))
            .unzip();
        (
            EffectProjectedSession {
                session: self.session,
                entries,
            },
            processors,
        )
    }
}

/// The launch native-effect registry, with its tail-bound table (issue #1469, root ruling R2).
///
/// The registry is a process-lifetime, control-plane-only value: it is built once, on the first
/// call (the first session preparation, live classification or response preview of the process,
/// or of the module instance in the browser), and every later call returns the same immutable
/// registry. No render path reaches it. Session preparation, live classification and the
/// response preview read it here and inject it into [`prepare_native_session_effects`] and its
/// peers; prepared plans keep their own clones of the factories they use.
///
/// A concurrent first call on another control thread waits for the one build and then reads the
/// same value. A failed build is cached as well and is permanent for the process: the registry's
/// inputs are compiled in, so a retry cannot succeed, and every call returns a clone of the same
/// [`RegistryError`].
pub fn launch_native_effect_registry() -> Result<&'static NativeEffectRegistry, RegistryError> {
    LAUNCH_NATIVE_EFFECT_REGISTRY
        .get_or_init(build_launch_native_effect_registry)
        .as_ref()
        .map_err(Clone::clone)
}

/// The one launch registry of the process (issue #1469 D1). Filled at most once, off render.
static LAUNCH_NATIVE_EFFECT_REGISTRY: OnceLock<Result<NativeEffectRegistry, RegistryError>> =
    OnceLock::new();

/// Whether `factory` is the launch registry's own factory allocation (issue #1469 Amendment 1).
///
/// The process-lifetime registry owns that allocation, so a plan's clone of it allocates nothing
/// and no plan is charged for it; the registry's bytes are process-level. Reads the registry only
/// if it is already built: before the first build no factory can be the registry's, and this
/// never builds it. Control thread only; it clones and drops one `Arc` (no allocation).
#[must_use]
pub fn launch_registry_owns_factory(factory: &Arc<dyn NativeEffectFactory>) -> bool {
    let Some(Ok(registry)) = LAUNCH_NATIVE_EFFECT_REGISTRY.get() else {
        return false;
    };
    registry
        .get_shared_ascii(factory.descriptor().id.as_str())
        .is_some_and(|shared| Arc::ptr_eq(&shared, factory))
}

// Issue #1469 D4: shared reads from several control threads need a `Send + Sync` registry.
const _: () = {
    const fn shared_across_threads<T: Send + Sync>() {}
    shared_across_threads::<NativeEffectRegistry>();
    shared_across_threads::<RegistryError>();
};

/// The eleven launch factories, in registry order. Called only by the static's one build.
fn build_launch_native_effect_registry() -> Result<NativeEffectRegistry, RegistryError> {
    NativeEffectRegistry::new([
        Box::new(parametric_eq::ParametricEqFactory) as Box<dyn NativeEffectFactory>,
        Box::new(compressor::CompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(gate_expander::GateExpanderFactory) as Box<dyn NativeEffectFactory>,
        Box::new(multiband_compressor::MultibandCompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(true_peak_limiter::TruePeakLimiterFactory) as Box<dyn NativeEffectFactory>,
        Box::new(soft_clip::SoftClipFactory) as Box<dyn NativeEffectFactory>,
        Box::new(transient_shaper::TransientShaperFactory) as Box<dyn NativeEffectFactory>,
        Box::new(delay::DelayFactory) as Box<dyn NativeEffectFactory>,
        Box::new(de_esser::DeEsserFactory) as Box<dyn NativeEffectFactory>,
        Box::new(dynamic_eq::DynamicEqFactory) as Box<dyn NativeEffectFactory>,
        Box::new(reverb::ReverbFactory) as Box<dyn NativeEffectFactory>,
    ])
}

/// The native effects a session console slot may name (owner decision 12, Sol's L2).
///
/// A console slot always banks, so it must be an effect whose homogeneous bank kernel the console
/// can rely on: the parametric EQ, compressor, gate/expander, soft-clip, transient shaper,
/// true-peak limiter and multiband compressor (admitted by #1893 once #1069 closed). The delay
/// and the reverb never bank, the de-esser is excluded until the owner rules on its eligibility (#1920), and the
/// dynamic EQ is an insert only at launch (#17 S2). Anything else is refused with
/// `console.slot.ineligible_effect` where native identities resolve, in
/// [`prepare_native_session_effects`]; the session schema refuses a third-party slot before this
/// runs.
pub const CONSOLE_ELIGIBLE_EFFECTS: [&str; 7] = [
    "miso.parametric-eq",
    "miso.compressor",
    "miso.gate-expander",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.true-peak-limiter",
    "miso.multiband-compressor",
];

/// Launch effects whose factory binds no homogeneous bank at any width (issue #1087).
///
/// This list says only that the effect declines a bank; it does not decide its bypass. A session
/// bypass on a never-banked effect is lowered to per-lane shunt state like any other
/// (issue #1339), and the effect renders per node as a live-control
/// effect whose shunt emits the dry block while it is bypassed. The delay and the reverb (#2114)
/// both decline every bank and render per node. The delay's block-boundary check
/// (D7) still runs on its wet path when a runaway feedback ring trips it, and the shunt then
/// passes the dry block instead of the zeros a prepared bypass emitted (decision 15 E4).
///
/// `graph-compiler`'s `bypass_shunt_identity` test holds this list to exactly the launch factories
/// that decline a well-formed bank at the host's width.
pub const NEVER_BANKED_EFFECTS: [&str; 2] = ["miso.delay", "miso.reverb"];

pub fn prepare_native_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
    project_with_console_eligibility(session, registry, caps, &CONSOLE_ELIGIBLE_EFFECTS)?.prepare()
}

/// Resolve, validate and design `session`'s native effect instances without preparing any
/// processor (issue #2199): [`prepare_native_session_effects`]' every refusal, in its order, and
/// its entries less their processors. The graph compiler projects the session's graph from them
/// and prepares a processor only for an instance its bank plan leaves per node.
///
/// # Errors
///
/// The diagnostics [`prepare_native_session_effects`] would return.
pub fn project_native_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
) -> Result<EffectProjectedSession, EffectDiagnosticSet> {
    project_with_console_eligibility(session, registry, caps, &CONSOLE_ELIGIBLE_EFFECTS)
}

/// [`prepare_native_session_effects`] with the console eligibility list supplied by the caller.
///
/// Every host prepares through [`prepare_native_session_effects`], which admits exactly
/// [`CONSOLE_ELIGIBLE_EFFECTS`]. This entry exists for test and audit registries whose test double
/// (the conformance crate's `conformance.delay`, which no production registry carries) must occupy
/// a lowered console rack to exercise the graph's internal stages. It changes the list for that
/// caller and nothing else: identity resolution, parameters and every other refusal are the same.
///
/// Compiled only for this crate's tests and under `test-support` (#1093 verdict L2), so no
/// production build can reach a console slot the fixed list refuses.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn prepare_native_session_effects_with_console_eligibility(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
    console_eligible: &[&str],
) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
    project_with_console_eligibility(session, registry, caps, console_eligible)?.prepare()
}

/// [`project_native_session_effects`] with the console eligibility list supplied by the caller,
/// as [`prepare_native_session_effects_with_console_eligibility`] supplies it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn project_native_session_effects_with_console_eligibility(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
    console_eligible: &[&str],
) -> Result<EffectProjectedSession, EffectDiagnosticSet> {
    project_with_console_eligibility(session, registry, caps, console_eligible)
}

/// The one projection behind every entry: the console slots whose native identity is not in
/// `console_eligible` are refused, and everything else is resolved, validated and designed.
fn project_with_console_eligibility(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
    console_eligible: &[&str],
) -> Result<EffectProjectedSession, EffectDiagnosticSet> {
    let mut diagnostics = Vec::new();
    let mut entries = Vec::new();
    if caps.maximum_total_state_bytes == 0
        || caps.maximum_scratch_bytes == 0
        || caps.maximum_automation_spans_per_block == 0
    {
        return Err(EffectDiagnosticSet::sorted(vec![EffectDiagnostic {
            code: "effect.resource.limit",
            path: "$.effect_compile_caps".to_owned(),
        }]));
    }
    let model = session.normalized_model();
    for (section, slots) in [
        ("pre_insert", &model.console.pre_insert),
        ("post_insert", &model.console.post_insert),
    ] {
        for slot in slots {
            if let EffectIdentity::Native { effect_id } = &slot.identity
                && !console_eligible.contains(&effect_id.as_str())
            {
                diagnostics.push(EffectDiagnostic {
                    code: "console.slot.ineligible_effect",
                    path: format!("$.console.{section}[slot={}].identity", slot.slot),
                });
            }
        }
    }
    // Issue #2194 (root ruling R4): pass 2 of the one walk. Pass 1 is
    // [`project_session_effects`], which a host runs against its aggregate budget before it calls
    // this; each instance below reads the same quality-table cell, and `effect.metadata.mismatch`
    // holds every prepared instance to it.
    for_each_session_effect(model, |site| {
        let EffectSite {
            strip,
            strip_path,
            rack,
            index,
            effect,
            console_slots,
        } = site;
        let path = format!("{strip_path}.effects[id={}]", effect.id);
        // #1369 D2: a console entry that overrides its slot's link mode is diagnosed at the
        // entry's own `link_mode`; an entry on `slot` keeps the effect path.
        let console_override = console_slots.and_then(|(offset, slots)| {
            let entry = strip.console.get(offset + index)?;
            let declared = slots.get(index)?.link_mode;
            (entry.link_mode != ConsoleLinkMode::Slot).then(|| {
                (
                    format!("{strip_path}.console[slot={}].link_mode", entry.slot),
                    declared,
                )
            })
        });
        let EffectIdentity::Native { effect_id } = &effect.identity else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.third_party.unavailable_at_launch",
                path,
            });
            return;
        };
        let Some(factory) = registry.get_shared_ascii(effect_id.as_str()) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.native.unavailable",
                path,
            });
            return;
        };
        let descriptor = factory.descriptor();
        let quality = contract_quality(effect.quality);
        let link_mode = match effect.link_mode {
            SessionLinkMode::DualMono => LinkMode::DualMono,
            SessionLinkMode::Maximum => LinkMode::Maximum,
            SessionLinkMode::Average => LinkMode::Average,
        };
        if !descriptor.supported_link_modes.contains(link_mode) {
            diagnostics.push(EffectDiagnostic {
                code: "effect.link_mode.unsupported",
                path: console_override.map_or(path, |(entry_path, _)| entry_path),
            });
            return;
        }
        // #1369 R3: an effect whose bank does not hold the link mode per lane would split
        // the slot's bank into two cohorts on a differing override, and a console slot
        // binds one bank (decision 12). Refused until that effect holds it per lane.
        // An unsupported resolved mode refuses first; the bank guard applies only to a
        // supported mode.
        if let Some((entry_path, declared)) = &console_override
            && effect.link_mode != *declared
            && !descriptor.lane_link
        {
            diagnostics.push(EffectDiagnostic {
                code: "console.link_mode.override_unbanked",
                path: entry_path.clone(),
            });
            return;
        }
        // Issue #1462 D1: the registry's table entry, not a call of the descriptor's
        // `tail_and_rest`. A rate or quality the effect does not declare has no entry and
        // is `effect.quality.unsupported`, as it always was.
        let tail_bound = match registry.tail_bound(descriptor.id, session.sample_rate().0, quality)
        {
            Ok(entry) => entry,
            Err(error) => {
                diagnostics.push(EffectDiagnostic {
                    code: error.code,
                    path,
                });
                return;
            }
        };
        let initial = match resolve_initial_values(descriptor, &effect.params) {
            Ok(initial) => initial,
            Err(code) => {
                diagnostics.push(EffectDiagnostic { code, path });
                return;
            }
        };
        let declared_sidechain = descriptor
            .ports
            .iter()
            .find(|port| port.role == effect_contract::PortRole::SidechainInput);
        let ports = match (&effect.sidechain, declared_sidechain) {
            (SidechainDeclaration::None, None) => PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            (SidechainDeclaration::None, Some(port)) if !port.required => PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: port.id,
                    required: false,
                },
            },
            (SidechainDeclaration::None, Some(_)) => {
                diagnostics.push(EffectDiagnostic {
                    code: "effect.sidechain.missing",
                    path,
                });
                return;
            }
            (SidechainDeclaration::Routed(_), None) => {
                diagnostics.push(EffectDiagnostic {
                    code: "effect.sidechain.unexpected",
                    path,
                });
                return;
            }
            (SidechainDeclaration::Routed(sidechain), Some(_)) => {
                match descriptor.ports.iter().find(|port| {
                    port.role == effect_contract::PortRole::SidechainInput
                        && port.id.as_str() == sidechain.port_id.as_str()
                }) {
                    Some(port) => PreparedPorts {
                        sidechain: PreparedSidechainPort::Connected {
                            id: port.id,
                            required: port.required,
                        },
                    },
                    None => {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.sidechain.unknown_port",
                            path,
                        });
                        return;
                    }
                }
            }
        };
        // Issue #1087: a session's bypass is per-lane shunt state, not a prepared
        // program. The effect is prepared enabled, so a banking effect shares its program
        // key and its bank with every enabled instance, and the bit rides the instance's
        // control lane to the shunt, which emits the latency-matched dry signal for this
        // lane. An effect that never banks lowers it too and renders per node (issue
        // #1339), and the multiband's bank recovers per lane (issue #1340), so every
        // effect lowers it.
        let automation_capacity = u32::try_from(EffectLaneShape::live_parameter_cells(descriptor))
            .unwrap_or(u32::MAX)
            .max(1);
        if automation_capacity > caps.maximum_automation_spans_per_block {
            diagnostics.push(EffectDiagnostic {
                code: "effect.resource.limit",
                path,
            });
            return;
        }
        let bank_preparation = EffectBankPreparation {
            sample_rate: session.sample_rate().0,
            quantum: session.quantum().0,
            quality,
            bypass: false,
            link_mode,
            ports,
            initial_values: initial.into_boxed_slice(),
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: caps.maximum_total_state_bytes,
                maximum_scratch_bytes: caps.maximum_scratch_bytes,
                // Decision 15, D15-2 and #1345 D4: the window holds a span for every live
                // parameter cell, so the effect accepts that many. The bound depends on the
                // descriptor alone, so every instance of one effect has one capacity and
                // attaching a live lane changes no program key. The parametric EQ is
                // raised too, though its lane holds target cells in place of parameter
                // cells and so stages no parameter span: whether an instance is attached
                // to a lane, or to an owner, must not change its capacity.
                maximum_automation_spans_per_block: automation_capacity,
            },
            tail_bound,
        };
        let request = bank_preparation.request();
        let metadata = match expected_prepared_metadata(descriptor, request) {
            Ok(metadata) => metadata,
            Err(error) => {
                diagnostics.push(EffectDiagnostic {
                    code: error.code,
                    path,
                });
                return;
            }
        };
        // Issue #2199: the factory answers from the request what `prepare` would, refusing it
        // with `prepare`'s own code, so a projected session is refused exactly where a prepared
        // one was, and its pool class is known before any processor exists.
        let designed_symmetry = match factory.designed_channel_symmetry(request) {
            Ok(symmetric) => symmetric,
            Err(error) => {
                diagnostics.push(EffectDiagnostic {
                    code: error.code,
                    path,
                });
                return;
            }
        };
        entries.push(EffectProjectedEntry {
            track_id: strip.id.as_str().to_owned(),
            strip_path: strip_path.into(),
            rack,
            effect_id: effect.id.as_str().to_owned(),
            metadata,
            designed_symmetry,
            factory,
            bank_preparation,
            initial_bypass: effect.bypass,
            control: effect
                .bypass
                .then(|| Box::new(EffectControlLane::without_channel(true))),
            observation: None,
            planned_lane: None,
        });
    });
    if diagnostics.is_empty() {
        entries.sort_by(|a, b| {
            (&a.track_id, a.rack, &a.effect_id).cmp(&(&b.track_id, b.rack, &b.effect_id))
        });
        Ok(EffectProjectedSession {
            session: session.clone(),
            entries,
        })
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// One effect instance of a session's strips, as [`for_each_session_effect`] visits it.
struct EffectSite<'a> {
    strip: StripRef<'a>,
    /// The strip's compiler path prefix ([`StripRef::path_prefix`]).
    strip_path: &'a str,
    rack: EffectRack,
    /// The instance's position in its lowered rack.
    index: usize,
    effect: &'a Effect,
    /// For a console rack: the rack's first slot index in the strip's `console` array and the
    /// session's slots of that section.
    console_slots: Option<(usize, &'a [ConsoleSlot])>,
}

/// The one walk of a session's effect instances (issue #2194, root ruling R4): every strip in
/// model order, its three lowered racks in chain order (decision 12, class A by lowering:
/// `pre_insert` is the first internal rack, the strip's inserts the second and `post_insert` the
/// third, each console entry an ordinary effect), and each rack's instances in order. The
/// session projection and the preparation are two passes over it, so they cannot visit different
/// instances. Control thread only; it allocates one strip's lowering at a time.
fn for_each_session_effect(model: &SessionModel, mut visit: impl FnMut(EffectSite<'_>)) {
    for strip in model.strips() {
        let lowered = model.lower_strip(&strip);
        let strip_path = strip.path_prefix();
        let [pre_insert, inserts, post_insert] = lowered.in_chain_order();
        let pre_slots = model.console.pre_insert.len();
        for (rack, effects, console_slots) in [
            (
                EffectRack::Simd1,
                pre_insert,
                Some((0, model.console.pre_insert.as_slice())),
            ),
            (EffectRack::Dynamic, inserts, None),
            (
                EffectRack::Simd2,
                post_insert,
                Some((pre_slots, model.console.post_insert.as_slice())),
            ),
        ] {
            for (index, effect) in effects.iter().enumerate() {
                visit(EffectSite {
                    strip,
                    strip_path: &strip_path,
                    rack,
                    index,
                    effect,
                    console_slots,
                });
            }
        }
    }
}

/// The session quality's effect-contract twin.
const fn contract_quality(quality: session::EffectQuality) -> EffectQuality {
    match quality {
        session::EffectQuality::Draft => EffectQuality::Draft,
        session::EffectQuality::Normal => EffectQuality::Normal,
        session::EffectQuality::High => EffectQuality::High,
    }
}

/// What a session's native effect instances declare, summed before any of them is prepared
/// (issue #2194).
///
/// Every row is read from the registry's descriptors, never from a prepared instance: the
/// declared state and scratch are the instance's quality-table row
/// ([`effect_contract::declared_instance_resources`]), which preparation then holds every
/// prepared instance to (`effect.metadata.mismatch`). The processors' own bytes are not here:
/// only an instance its bank plan leaves per node gets a processor, so a host charges them per
/// entry of the projected graph ([`EffectProjectedEntry::processor_bytes`], issue #2219).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectStateProjection {
    /// Native instances whose identity, rate and quality resolve.
    pub instances: u64,
    /// The sum of their declared `maximum_state` totals: the report's `effect_scalar_state_bytes`.
    pub state_bytes: u64,
    /// The sum of their declared scratch: the report's `effect_scalar_scratch_bytes`.
    pub scratch_bytes: u64,
}

/// Project `session`'s native effect instances against `registry` without preparing any
/// (issue #2194): the first pass of the walk [`prepare_native_session_effects`] prepares in, for a
/// host that admits the session's effect bytes against its aggregate budget before it prepares.
///
/// An instance that does not resolve (a third-party identity, an effect the registry lacks, a rate
/// or quality its descriptor does not declare) adds nothing: preparation refuses it. `None` when a
/// sum overflows `u64`.
#[must_use]
pub fn project_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
) -> Option<EffectStateProjection> {
    let model = session.normalized_model();
    let (sample_rate, quantum) = (session.sample_rate().0, session.quantum().0);
    let mut total = Some(EffectStateProjection::default());
    for_each_session_effect(model, |site| {
        let EffectIdentity::Native { effect_id } = &site.effect.identity else {
            return;
        };
        let Some(factory) = registry.get_shared_ascii(effect_id.as_str()) else {
            return;
        };
        let Ok(declared) = effect_contract::declared_instance_resources(
            factory.descriptor(),
            contract_quality(site.effect.quality),
            sample_rate,
            quantum,
        ) else {
            return;
        };
        total = total.and_then(|sum| {
            Some(EffectStateProjection {
                instances: sum.instances.checked_add(1)?,
                state_bytes: sum
                    .state_bytes
                    .checked_add(declared.quality.maximum_state.total()?)?,
                scratch_bytes: sum.scratch_bytes.checked_add(declared.scratch_bytes)?,
            })
        });
    });
    total
}

/// One prepared live-control lane for one effect instance, as control holds it (issue #140 A;
/// decision 15, D15-2; issue #1345).
///
/// The writer is control's end of the instance's latest-target cells and observation queue. The
/// render end rode into the plan inside the entry; the cells are an `Arc` the two ends share, so
/// either may drop first and the last owner frees them: `PreparedHost` drops its plan first, and
/// capi drops a reclaimed plan before the provider epoch that keeps its producers (#1263 D5), both
/// on the control thread.
pub struct EffectControlProducer {
    /// Session-stable identity of the strip (a track or a submix) this channel addresses.
    pub track_id: Box<str>,
    /// The instance's live address within its strip (a track or a submix): a console slot by its
    /// slot index, an insert by its index (decision 12, issue #1096).
    ///
    /// This is the `(rack, effect_index)` a host looks a channel up by; the core's `SET_CONTROL`
    /// and `Observe` edits name the instance by stable ID instead. It is derived from
    /// the normalized session model here rather than from `EffectPreparedSession::entries`, which
    /// is sorted by effect id.
    pub address: LiveEffectAddress,
    /// Session-stable effect instance identity.
    pub effect_id: Box<str>,
    /// The effect's declared parameter table, so an admitting host can map a wire `parameter_id`
    /// to the `parameter_index` the render side stages, and check the value's domain, without a
    /// second copy of the registry.
    pub descriptor: &'static EffectDescriptor,
    /// The instance's cell writers and observation queue. A write cannot fail on a value; a
    /// record the lane has no cell for is refused by [`Self::preflight`] before the first write
    /// of its transaction.
    writer: EffectCellWriter,
    /// Optional off-audio-thread owner for a factory's prepared-target capability.
    ///
    /// The box is private so callers must use the checked transaction methods below; no raw
    /// writer or reusable "validated" flag can bypass candidate/revision admission.
    owner: Option<Box<EffectControlOwner>>,
}

/// Checked native allocations retained by one effect-control producer table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EffectControlResources {
    /// The actual backing allocation of the native producer `Vec`, using its capacity.
    pub producer_table_bytes: u64,
    /// Retained strings, owner boxes, owner row/dirty backings, the writers' cell-writer arrays
    /// and distinct factory `Arc`s.
    pub owned_payload_bytes: u64,
    /// Largest individual allocation in `owned_payload_bytes` (the producer table is separate).
    pub largest_owned_allocation_bytes: u64,
}

impl EffectControlResources {
    /// Total native effect-control bytes retained by the table and its transferred payload.
    #[must_use]
    pub const fn total_bytes(self) -> Option<u64> {
        self.producer_table_bytes
            .checked_add(self.owned_payload_bytes)
    }

    /// Largest individual allocation including the producer table backing.
    #[must_use]
    pub const fn largest_allocation_bytes(self) -> u64 {
        if self.producer_table_bytes > self.largest_owned_allocation_bytes {
            self.producer_table_bytes
        } else {
            self.largest_owned_allocation_bytes
        }
    }
}

/// Projects actual native effect-control allocations from the producer Vec.
pub fn effect_control_resources(
    producers: &Vec<EffectControlProducer>,
) -> Result<EffectControlResources, EffectControlResourceError> {
    let producer_table_bytes = producers
        .capacity()
        .checked_mul(core::mem::size_of::<EffectControlProducer>())
        .and_then(|bytes| u64::try_from(bytes).ok())
        .ok_or(EffectControlResourceError::Arithmetic)?;
    let mut owned_payload_bytes = 0_u64;
    let mut largest_owned_allocation_bytes = 0_u64;
    let mut factory_payload_bytes = 0_u64;
    for (index, producer) in producers.iter().enumerate() {
        for text in [&producer.track_id, &producer.effect_id] {
            let bytes =
                u64::try_from(text.len()).map_err(|_| EffectControlResourceError::Arithmetic)?;
            owned_payload_bytes = owned_payload_bytes
                .checked_add(bytes)
                .ok_or(EffectControlResourceError::Arithmetic)?;
            largest_owned_allocation_bytes = largest_owned_allocation_bytes.max(bytes);
        }
        let writer = producer.writer.retained_resources();
        owned_payload_bytes = owned_payload_bytes
            .checked_add(writer.total_bytes)
            .ok_or(EffectControlResourceError::Arithmetic)?;
        largest_owned_allocation_bytes =
            largest_owned_allocation_bytes.max(writer.largest_allocation_bytes);
        if let Some(owner) = producer.owner.as_deref() {
            let facts = owner.resource_facts()?;
            owned_payload_bytes = owned_payload_bytes
                .checked_add(facts.owned_payload_bytes)
                .ok_or(EffectControlResourceError::Arithmetic)?;
            largest_owned_allocation_bytes =
                largest_owned_allocation_bytes.max(facts.largest_owned_allocation_bytes);
            let shared_factory = producers[..index]
                .iter()
                .filter_map(|prior| prior.owner.as_deref())
                .any(|prior| Arc::ptr_eq(prior.factory(), owner.factory()));
            // #1469 Amendment 1: a factory the launch registry owns is process-level memory that
            // the plan's clone does not allocate, so no plan is charged for it.
            if !shared_factory && !launch_registry_owns_factory(owner.factory()) {
                factory_payload_bytes = factory_payload_bytes
                    .checked_add(facts.factory_allocation_bytes)
                    .ok_or(EffectControlResourceError::Arithmetic)?;
                largest_owned_allocation_bytes =
                    largest_owned_allocation_bytes.max(facts.factory_allocation_bytes);
            }
        }
    }
    // Factory allocations are counted once by identity. Add them after walking owner-local
    // payload so shared Arc clones never multiply the retained factory bytes.
    owned_payload_bytes = owned_payload_bytes
        .checked_add(factory_payload_bytes)
        .ok_or(EffectControlResourceError::Arithmetic)?;
    Ok(EffectControlResources {
        producer_table_bytes,
        owned_payload_bytes,
        largest_owned_allocation_bytes,
    })
}

impl EffectControlProducer {
    /// Check delivery capability without writing: `Ok` when [`Self::write`] would take `record`.
    ///
    /// A record the lane has no cell for is refused here: a parameter that is not `Block`-rate or
    /// whose channel does not match its policy, a parameter of a prepared-target owner (its
    /// parameters ride its targets), an observation subscription (it rides the queue,
    /// [`Self::try_push_observe`]), and any prepared target: a target reaches its cells only
    /// through the owner's transaction ([`Self::publish_candidate_targets`]), so no caller can
    /// bypass the candidate and revision admission. The transaction that writes preflights every
    /// record before its first write (D15-2 condition 3).
    ///
    /// # Errors
    ///
    /// [`EffectWriteRefusal`].
    pub fn preflight(&self, record: EffectControlRecord) -> Result<(), EffectWriteRefusal> {
        if matches!(record, EffectControlRecord::PreparedTarget(_)) {
            return Err(EffectWriteRefusal::Target);
        }
        self.writer.check(&record)
    }

    /// Write `record` as the newest value of its cell(s), stamped with the revision of the
    /// transaction that commits it. The caller publishes that revision on the plan's gate after
    /// every write of the transaction.
    ///
    /// # Errors
    ///
    /// [`EffectWriteRefusal`], with nothing written. The caller has preflighted the record, so a
    /// refusal here is an internal fault.
    pub fn write(
        &mut self,
        stamp: LiveStamp<'_>,
        record: EffectControlRecord,
    ) -> Result<(), EffectWriteRefusal> {
        self.preflight(record)?;
        self.writer.write(stamp, record)
    }

    /// Queue one observation subscription for the lane's next drain, in order (D15-2: an
    /// observation record stays a FIFO).
    ///
    /// # Errors
    ///
    /// [`ObservePushError`], with nothing queued.
    ///
    /// `arm_tag` is the edit's tag (#1585 D4), `0` for none; render publishes it with the arm.
    pub fn try_push_observe(
        &mut self,
        tap_index: u32,
        armed: bool,
        window_blocks: u32,
        arm_tag: u64,
    ) -> Result<(), ObservePushError> {
        self.writer
            .try_push_observe(tap_index, armed, window_blocks, arm_tag)
    }

    /// The observation queue's capacity: the number of taps the effect declares, `0` for a lane
    /// without a queue.
    #[must_use]
    pub fn observe_capacity(&self) -> usize {
        self.writer.observe_capacity()
    }

    /// Free slots of the observation queue.
    #[must_use]
    pub fn observe_available_capacity(&self) -> usize {
        self.writer.observe_available_capacity()
    }

    /// Producer-local count of observation records queued.
    #[must_use]
    pub fn observe_success_count(&self) -> u64 {
        self.writer.observe_success_count()
    }

    /// Producer-local count of observation records refused because the queue was full.
    #[must_use]
    pub fn observe_full_count(&self) -> u64 {
        self.writer.observe_full_count()
    }

    /// The instance's cell writer, for inspection.
    #[must_use]
    pub fn writer(&self) -> &EffectCellWriter {
        &self.writer
    }

    /// Whether this effect has an opted-in prepared-target owner.
    #[must_use]
    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    /// Borrows the checked owner for inspection of its committed state.
    #[must_use]
    pub fn owner(&self) -> Option<&EffectControlOwner> {
        self.owner.as_deref()
    }

    /// Starts the owner transaction at a checked committed revision.
    pub fn begin_owner(&mut self, base_revision: u64) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .begin(base_revision)
    }

    /// Applies one checked semantic edit to the owner candidate.
    pub fn edit_owner(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .edit(parameter_index, channel, value)
    }

    /// Writes a validated target set into the owner's target cells, stamped with the revision of
    /// the transaction that commits it, and marks the owner ready for its one commit. A write
    /// cannot fail on room: only the revision and the targets are checked, by
    /// [`Self::preflight_candidate_targets`], which the caller ran before its first write.
    pub fn publish_candidate_targets(
        &mut self,
        stamp: LiveStamp<'_>,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.publish(&mut self.writer, stamp, base_revision, targets)
    }

    /// Preflights this owner's exact target set against its open candidate and its revision. The
    /// cells, candidate and owner phase remain unchanged.
    pub fn preflight_candidate_targets(
        &self,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.preflight_publication(base_revision, targets)?;
        // The publication writes each target to the cell the lane's writer addresses it by, so
        // the preflight asks the writer about every one: a set the owner accepts and the lane has
        // no cell for is refused here, before the first write (D7), not discovered by a write.
        for target in targets {
            self.writer
                .check(&EffectControlRecord::PreparedTarget(*target))
                .map_err(|_| EffectControlOwnerError::Shape)?;
        }
        Ok(())
    }

    /// Commits the candidate after successful complete publication.
    pub fn commit_owner(&mut self) -> Result<u64, EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .commit()
    }

    /// Discards an open or poisoned candidate.
    pub fn discard_owner(&mut self) -> Result<(), EffectControlOwnerError> {
        self.owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .discard()
    }
}

#[cfg(test)]
mod metadata_mismatch_tests {
    use super::{EffectCompileCaps, prepare_native_session_effects};
    use effect_contract::{
        CompositionBound, EffectDescriptor, EffectId, EffectPrepareError, EffectQuality,
        FlushStall, LatencySamples, LinkMode, NativeEffectFactory, NativeEffectRegistry, PeakGain,
        PortId, PrepareEffectBankRequest, PrepareEffectRequest, PreparedEffect, PreparedEffectBank,
        PreparedEffectMetadata, PreparedSidechainPort, RestBound, RestSamples, TailDecay,
        TailSamples,
    };
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use session::{CompileCaps, compile_session, parse_session_json};

    /// The production EQ, whose prepare result's metadata `forge` then edits. The processor is
    /// the EQ's own; only the metadata handed out beside it differs (issue #1461).
    struct ForgingEq(Forge);
    impl NativeEffectFactory for ForgingEq {
        fn descriptor(&self) -> &'static EffectDescriptor {
            &PARAMETRIC_EQ_DESCRIPTOR
        }
        fn prepared_inline_bytes(&self) -> u64 {
            ParametricEqFactory.prepared_inline_bytes()
        }
        fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
            0
        }
        fn designed_channel_symmetry(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<bool, EffectPrepareError> {
            self.prepare(request)
                .map(|prepared| prepared.processor.channel_symmetry())
        }

        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<PreparedEffect, EffectPrepareError> {
            let mut prepared = ParametricEqFactory.prepare(request)?;
            (self.0)(&mut prepared.metadata);
            Ok(prepared)
        }
        fn bind_homogeneous_bank(
            &self,
            _: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
            Ok(None)
        }
    }

    /// A descriptor equal to the prepared one except for what `edit` changes.
    fn forged_descriptor(metadata: &mut PreparedEffectMetadata, edit: fn(&mut EffectDescriptor)) {
        let mut descriptor = *metadata.descriptor;
        edit(&mut descriptor);
        metadata.descriptor = Box::leak(Box::new(descriptor));
    }

    fn other_tail(tail: TailSamples) -> TailSamples {
        match tail {
            TailSamples::Infinite => TailSamples::Finite(0),
            TailSamples::Finite(samples) => TailSamples::Finite(samples.wrapping_add(1)),
        }
    }

    /// One edit of a prepare result's metadata.
    type Forge = fn(&mut PreparedEffectMetadata);

    /// One forgery per field the mismatch check compares, each a value other than the one the
    /// descriptor and request state.
    const FORGERIES: [(&str, Forge); 17] = [
        ("descriptor.id", |m| {
            forged_descriptor(m, |d| {
                d.id = EffectId::new("forged.effect").expect("a valid effect id");
            });
        }),
        ("descriptor.contract_major", |m| {
            forged_descriptor(m, |d| d.contract_major = d.contract_major.wrapping_add(1));
        }),
        ("descriptor.state_layout_version", |m| {
            forged_descriptor(m, |d| {
                d.state_layout_version = d.state_layout_version.wrapping_add(1);
            });
        }),
        ("sample_rate", |m| {
            m.sample_rate = m.sample_rate.wrapping_add(1)
        }),
        ("quantum", |m| m.quantum = m.quantum.wrapping_add(1)),
        ("quality", |m| {
            m.quality = if m.quality == EffectQuality::High {
                EffectQuality::Draft
            } else {
                EffectQuality::High
            };
        }),
        ("bypass", |m| m.bypass = !m.bypass),
        ("link_mode", |m| {
            m.link_mode = if m.link_mode == LinkMode::Maximum {
                LinkMode::Average
            } else {
                LinkMode::Maximum
            };
        }),
        ("ports", |m| {
            m.ports.sidechain = match m.ports.sidechain {
                PreparedSidechainPort::None => PreparedSidechainPort::Unconnected {
                    id: PortId::new("forged-sidechain").expect("a valid port id"),
                    required: false,
                },
                _ => PreparedSidechainPort::None,
            };
        }),
        ("latency", |m| {
            m.latency = LatencySamples(m.latency.0.wrapping_add(1));
        }),
        ("tail", |m| m.tail = other_tail(m.tail)),
        ("tail_every_peak", |m| {
            m.tail_every_peak = other_tail(m.tail_every_peak);
        }),
        ("rest", |m| {
            m.rest = match m.rest {
                RestBound::Unstated => RestBound::Bounded(RestSamples::ZERO),
                RestBound::Bounded(_) | RestBound::FixedPoint(_) => RestBound::Unstated,
            };
        }),
        ("composition", |m| {
            m.composition = match m.composition {
                CompositionBound::Unstated => CompositionBound::Stated {
                    decay: TailDecay(0),
                    peak_gain: PeakGain::Zero,
                    tail_gain: PeakGain::Zero,
                    peak_stall: FlushStall::Zero,
                    tail_stall: FlushStall::Zero,
                },
                CompositionBound::Stated { .. } => CompositionBound::Unstated,
            };
        }),
        ("state_sizes", |m| {
            m.state_sizes.common_bytes = m.state_sizes.common_bytes.wrapping_add(1);
        }),
        ("scratch_bytes", |m| {
            m.scratch_bytes = m.scratch_bytes.wrapping_add(1);
        }),
        ("automation_capacity", |m| {
            m.automation_capacity = m.automation_capacity.wrapping_add(1);
        }),
    ];

    /// Gate 3 of #1461 (extending #1377's gate 2 to every field): a prepare result whose metadata
    /// differs from `expected_prepared_metadata` in any compared field is refused with
    /// `effect.metadata.mismatch`, and prepares no partial session; the same factory with the
    /// metadata unedited prepares.
    ///
    /// The processor holds no copy of its metadata, so this check is the one thing that ties the
    /// metadata every control-side reader uses to what the descriptor and the request state.
    ///
    /// Red mutations: drop any one comparison from the mismatch check, or compare the returned
    /// metadata with itself instead of with the expected value.
    #[test]
    fn a_prepare_result_whose_metadata_differs_in_any_compared_field_is_refused() {
        let model = parse_session_json(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("accepted fixture");
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compiled fixture");
        let caps = EffectCompileCaps {
            maximum_total_state_bytes: 1 << 20,
            maximum_scratch_bytes: 1 << 20,
            maximum_automation_spans_per_block: 128,
        };
        let prepare = |forge: Forge| {
            let registry = NativeEffectRegistry::new([
                Box::new(ForgingEq(forge)) as Box<dyn NativeEffectFactory>
            ])
            .expect("registry");
            prepare_native_session_effects(&session, &registry, caps)
        };

        let prepared = prepare(|_| {}).expect("an unforged prepare result prepares");
        assert_eq!(prepared.entries.len(), 9);
        for (field, forge) in FORGERIES {
            let diagnostics = prepare(forge)
                .err()
                .unwrap_or_else(|| panic!("a forged `{field}` prepared"));
            assert_eq!(diagnostics.0.len(), 9, "{field}");
            assert!(
                diagnostics
                    .0
                    .iter()
                    .all(|diagnostic| diagnostic.code == "effect.metadata.mismatch"),
                "{field}: {:?}",
                diagnostics.0
            );
        }
    }
}

#[cfg(test)]
mod live_address_tests {
    use super::{EffectRack, LiveEffectAddress};

    /// Decision 12's lowering of a live address (issue #1096), on a track with two `pre_insert`
    /// slots, three inserts and one `post_insert` slot.
    ///
    /// Red mutations: lower console slot `k >= pre_insert` into `Simd1` (the `post_insert` slot
    /// addressed as `pre_insert`), or into `Simd2` at `k` rather than `k - pre_insert`; let an
    /// insert index reach past the inserts; or accept a console index past the last slot.
    #[test]
    fn a_live_address_lowers_through_the_section_split() {
        let lengths = [2, 3, 1];
        let cases = [
            (LiveEffectAddress::console(0), Some((EffectRack::Simd1, 0))),
            (LiveEffectAddress::console(1), Some((EffectRack::Simd1, 1))),
            (LiveEffectAddress::console(2), Some((EffectRack::Simd2, 0))),
            (LiveEffectAddress::console(3), None),
            (LiveEffectAddress::console(u32::MAX), None),
            (LiveEffectAddress::insert(0), Some((EffectRack::Dynamic, 0))),
            (LiveEffectAddress::insert(2), Some((EffectRack::Dynamic, 2))),
            (LiveEffectAddress::insert(3), None),
        ];
        for (address, lowered) in cases {
            assert_eq!(address.lower(lengths), lowered, "{address:?}");
        }
        // An empty console lowers no console address; an empty `pre_insert` puts slot 0 after the
        // inserts.
        assert_eq!(LiveEffectAddress::console(0).lower([0, 1, 0]), None);
        assert_eq!(
            LiveEffectAddress::console(0).lower([0, 1, 1]),
            Some((EffectRack::Simd2, 0))
        );
    }
}

#[cfg(test)]
mod owner_tests {
    use super::*;
    use crate::EffectControlOwnerPhase;
    use effect_contract::{
        NativeEffectFactory, ParameterChannel, PreparedEffectTarget, default_initial_values,
    };
    use engine::realtime::{GatePin, RevisionGate};
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use std::sync::Arc;

    /// The EQ's six target slots.
    const EQ_SLOTS: usize = 6;

    fn preparation() -> EffectBankPreparation {
        EffectBankPreparation {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect(),
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: u64::MAX,
                maximum_scratch_bytes: u64::MAX,
                maximum_automation_spans_per_block: 64,
            },
            tail_bound: NativeEffectRegistry::new([
                Box::new(ParametricEqFactory) as Box<dyn NativeEffectFactory>
            ])
            .expect("the EQ is admitted")
            .tail_bound(PARAMETRIC_EQ_DESCRIPTOR.id, 48_000, EffectQuality::Normal)
            .expect("a declared row"),
        }
    }

    /// An EQ owner, the writer of its target cells, the render end of those cells and the plan's
    /// gate.
    struct OwnerRig {
        owner: EffectControlOwner,
        writer: EffectCellWriter,
        lane: EffectControlLane,
        gate: RevisionGate,
        pin: GatePin,
        superseded: LiveValuesSuperseded,
    }

    impl OwnerRig {
        fn new() -> Self {
            let factory: Arc<dyn NativeEffectFactory> = Arc::new(ParametricEqFactory);
            let owner = EffectControlOwner::new(factory, &preparation()).expect("valid EQ owner");
            let superseded = LiveValuesSuperseded::new();
            let (writer, lane) = effect_control_lane(
                EffectLaneSpec {
                    shape: EffectLaneShape::from_descriptor(&PARAMETRIC_EQ_DESCRIPTOR, EQ_SLOTS),
                    bypass: false,
                    observation_depth: 0,
                    link: None,
                },
                &superseded,
            )
            .expect("an EQ lane");
            let (gate, pin) = RevisionGate::pair(0);
            Self {
                owner,
                writer,
                lane,
                gate,
                pin,
                superseded,
            }
        }

        /// Publish `targets` as one transaction: stamp the next revision, write the cells, publish
        /// the revision.
        fn publish(
            &mut self,
            base_revision: u64,
            targets: &[PreparedEffectTarget],
        ) -> Result<(), EffectControlOwnerError> {
            let revision = self.pin.last_published() + 1;
            let stamp = self.pin.stamp(revision).expect("a legal revision");
            let result = self
                .owner
                .publish(&mut self.writer, stamp, base_revision, targets);
            self.pin.publish(&self.gate, revision);
            result
        }

        /// The targets one block applies.
        fn block(&mut self) -> Vec<PreparedEffectTarget> {
            let snapshot = self.gate.snapshot();
            self.lane.stage(&mut [], 0, snapshot);
            self.lane.prepared_targets().to_vec()
        }
    }

    fn resource_producer(
        track_id: &str,
        effect_id: &str,
        factory: Option<Arc<dyn NativeEffectFactory>>,
    ) -> EffectControlProducer {
        let owner = factory.map(|factory| {
            Box::new(EffectControlOwner::new(factory, &preparation()).expect("valid EQ owner"))
        });
        let (writer, _lane) = effect_control_lane(
            EffectLaneSpec {
                shape: EffectLaneShape::from_descriptor(
                    &PARAMETRIC_EQ_DESCRIPTOR,
                    if owner.is_some() { EQ_SLOTS } else { 0 },
                ),
                bypass: false,
                observation_depth: 0,
                link: None,
            },
            &LiveValuesSuperseded::new(),
        )
        .expect("an EQ lane");
        EffectControlProducer {
            track_id: track_id.into(),
            address: LiveEffectAddress::insert(0),
            effect_id: effect_id.into(),
            descriptor: &PARAMETRIC_EQ_DESCRIPTOR,
            writer,
            owner,
        }
    }

    fn blank_targets() -> [PreparedEffectTarget; 12] {
        [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12]
    }

    /// An idle owner holds no candidate: the candidate and its touched mask exist from `begin`
    /// to `commit` or `discard` and nowhere else, and each new candidate starts from the
    /// committed rows (#1624).
    #[test]
    fn the_candidate_lives_only_between_begin_and_commit_or_discard() {
        let mut rig = OwnerRig::new();
        let rows = rig.owner.committed().len();
        assert!(rig.owner.candidate().is_empty() && rig.owner.dirty().is_empty());

        rig.owner.begin(0).expect("begin");
        assert_eq!(rig.owner.candidate(), rig.owner.committed());
        assert_eq!(rig.owner.dirty().len(), rows);
        assert!(rig.owner.dirty().iter().all(|touched| !touched));
        rig.owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("edit");
        assert!(rig.owner.dirty().iter().any(|touched| *touched));
        rig.owner.discard().expect("discard");
        assert!(rig.owner.candidate().is_empty() && rig.owner.dirty().is_empty());

        // A second candidate starts from the committed rows, not from the discarded edit.
        rig.owner.begin(0).expect("begin again");
        assert_eq!(rig.owner.candidate(), rig.owner.committed());
        assert!(rig.owner.dirty().iter().all(|touched| !touched));
        // A poisoned candidate is still the open one until discarded.
        assert!(rig.owner.edit(9_999, ParameterChannel::Both, 0.0).is_err());
        assert_eq!(rig.owner.phase(), EffectControlOwnerPhase::Poisoned);
        assert_eq!(rig.owner.dirty().len(), rows);
        rig.owner.discard().expect("discard poisoned");
        assert!(rig.owner.candidate().is_empty());

        // A published candidate lives until its one commit, which drops it.
        rig.owner.begin(0).expect("begin a committed transaction");
        rig.owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("edit");
        let edited = rig.owner.candidate().to_vec();
        let mut targets = blank_targets();
        let count = rig
            .owner
            .prepare_targets_into(&mut targets)
            .expect("prepare");
        rig.publish(0, &targets[..count]).expect("publish");
        assert_eq!(rig.owner.phase(), EffectControlOwnerPhase::Published);
        assert_eq!(rig.owner.candidate(), &edited[..]);
        assert_eq!(rig.owner.commit(), Ok(1));
        assert_eq!(rig.owner.phase(), EffectControlOwnerPhase::Idle);
        assert!(rig.owner.candidate().is_empty() && rig.owner.dirty().is_empty());
        assert_eq!(rig.owner.committed(), &edited[..]);
    }

    /// Red if two producers that share one factory are charged that factory twice (the charge is
    /// then no smaller than for two producers with a factory each), or if the producer table is
    /// charged by its length and not by the capacity its vector holds.
    #[test]
    fn effect_control_resources_charge_actual_tables_and_shared_owner_once() {
        let shared: Arc<dyn NativeEffectFactory> = Arc::new(ParametricEqFactory);
        let charge = |factories: [Arc<dyn NativeEffectFactory>; 2], capacity: usize| {
            let mut producers = Vec::with_capacity(capacity);
            for ((track, effect), factory) in [("track-a", "effect-a"), ("track-b", "effect-b")]
                .into_iter()
                .zip(factories)
            {
                producers.push(resource_producer(track, effect, Some(factory)));
            }
            effect_control_resources(&producers).expect("resource facts")
        };
        let once = charge([Arc::clone(&shared), Arc::clone(&shared)], 2);
        let twice = charge(
            [Arc::new(ParametricEqFactory), Arc::new(ParametricEqFactory)],
            2,
        );
        assert!(
            once.owned_payload_bytes < twice.owned_payload_bytes,
            "a shared factory is charged once: {} against {}",
            once.owned_payload_bytes,
            twice.owned_payload_bytes
        );
        let spare = charge([Arc::clone(&shared), Arc::clone(&shared)], 5);
        assert!(once.producer_table_bytes > 0);
        assert_eq!(
            2 * spare.producer_table_bytes,
            5 * once.producer_table_bytes,
            "the table is charged by its capacity"
        );
        assert_eq!(spare.owned_payload_bytes, once.owned_payload_bytes);
    }

    /// #1469 Amendment 1. Red if a plan is charged for a factory the process-lifetime launch
    /// registry owns: two owners that share the registry's EQ factory are charged less than two
    /// owners that share a factory the plan owns.
    #[test]
    fn effect_control_resources_charge_no_registry_owned_factory() {
        let registry_factory = launch_native_effect_registry()
            .expect("launch registry")
            .get_shared_ascii("miso.parametric-eq")
            .expect("launch EQ");
        let plan_factory: Arc<dyn NativeEffectFactory> = Arc::new(ParametricEqFactory);
        assert!(launch_registry_owns_factory(&registry_factory));
        assert!(!launch_registry_owns_factory(&plan_factory));
        let charge = |factory: &Arc<dyn NativeEffectFactory>| {
            let mut producers = Vec::with_capacity(2);
            for (track, effect) in [("track-a", "effect-a"), ("track-b", "effect-b")] {
                producers.push(resource_producer(track, effect, Some(Arc::clone(factory))));
            }
            effect_control_resources(&producers).expect("resource facts")
        };
        assert!(
            charge(&registry_factory).owned_payload_bytes
                < charge(&plan_factory).owned_payload_bytes,
            "the registry's factory is not charged to the plan"
        );
    }

    #[test]
    fn opted_in_eq_owner_prepares_both_and_commits_once() {
        let mut rig = OwnerRig::new();
        let initial = rig.owner.committed().to_vec();
        rig.owner.begin(0).expect("base revision");
        rig.owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("numeric Both edit");
        let mut targets = blank_targets();
        let count = rig
            .owner
            .prepare_targets_into(&mut targets)
            .expect("off-audio preparation");
        assert_eq!(count, 1, "equal Both lanes coalesce");
        assert_eq!(targets[0].channel, ParameterChannel::Both);
        assert_eq!(
            rig.owner.committed(),
            initial,
            "preparation leaves committed rows unchanged"
        );
        let final_rows = rig.owner.candidate().to_vec();
        let changed: Vec<_> = final_rows
            .iter()
            .filter(|row| row.parameter_index == 2)
            .collect();
        assert_eq!(changed.len(), 2);
        assert!(changed.iter().all(|row| row.value == 100.0));
        rig.publish(0, &targets[..count])
            .expect("complete target publication");
        assert_eq!(
            rig.owner.committed(),
            initial,
            "publication precedes shadow commit"
        );
        assert!(rig.owner.edit(2, ParameterChannel::Both, 200.0).is_err());
        assert!(rig.owner.discard().is_err());
        assert_eq!(rig.owner.commit().expect("one commit"), 1);
        assert_eq!(rig.owner.committed(), final_rows);
        assert!(rig.owner.commit().is_err(), "repeat commit refused");
        assert_eq!(
            rig.block(),
            vec![targets[0]],
            "the block applies the target once"
        );
    }

    #[test]
    fn opted_in_owner_accepts_two_transactions_before_render() {
        let mut rig = OwnerRig::new();
        let initial = rig.owner.committed().to_vec();
        let mut first = blank_targets();
        rig.owner.begin(0).expect("first base revision");
        rig.owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("first Both edit");
        let first_count = rig
            .owner
            .prepare_targets_into(&mut first)
            .expect("first prepare");
        assert_eq!(first_count, 1);
        rig.publish(0, &first[..first_count])
            .expect("first publication");
        assert_eq!(rig.owner.commit(), Ok(1));

        let mut second = blank_targets();
        rig.owner
            .begin(1)
            .expect("second base revision before render");
        rig.owner
            .edit(2, ParameterChannel::Right, 200.0)
            .expect("second asymmetric edit");
        let second_count = rig
            .owner
            .prepare_targets_into(&mut second)
            .expect("second prepare");
        assert_eq!(second_count, 1);
        assert_eq!(second[0].channel, ParameterChannel::Right);
        rig.publish(1, &second[..second_count])
            .expect("second publication");
        assert_eq!(rig.owner.commit(), Ok(2));
        // The cells hold the latest write of each channel: the left cell the first transaction's
        // `Both`, the right cell the second's `Right`. The block applies them as two lone targets.
        let applied = rig.block();
        assert_eq!(applied.len(), 2);
        assert_eq!(
            (applied[0].slot, applied[0].channel, applied[0].words),
            (first[0].slot, ParameterChannel::Left, first[0].words)
        );
        assert_eq!(
            (applied[1].slot, applied[1].channel, applied[1].words),
            (second[0].slot, ParameterChannel::Right, second[0].words)
        );
        assert_eq!(
            rig.superseded.get(),
            1,
            "the first transaction's right-channel write was replaced before any block read it"
        );
        assert_eq!(rig.owner.committed_revision(), 2);
        assert_eq!(
            rig.owner
                .committed()
                .iter()
                .find(|row| { row.parameter_index == 2 && row.channel == ParameterChannel::Left })
                .expect("left committed row")
                .value,
            100.0
        );
        assert_eq!(
            rig.owner
                .committed()
                .iter()
                .find(|row| { row.parameter_index == 2 && row.channel == ParameterChannel::Right })
                .expect("right committed row")
                .value,
            200.0
        );
        assert_ne!(
            rig.owner.committed(),
            initial.as_slice(),
            "both transactions changed the seed"
        );
    }

    #[test]
    fn opted_in_owner_accepts_launch_rates_and_rejects_unsupported_rate() {
        for sample_rate in [44_100, 48_000, 88_200, 96_000] {
            let mut preparation = preparation();
            preparation.sample_rate = sample_rate;
            preparation.tail_bound = NativeEffectRegistry::new([
                Box::new(ParametricEqFactory) as Box<dyn NativeEffectFactory>
            ])
            .expect("the EQ is admitted")
            .tail_bound(
                PARAMETRIC_EQ_DESCRIPTOR.id,
                sample_rate,
                EffectQuality::Normal,
            )
            .expect("a declared row");
            EffectControlOwner::new(
                Arc::new(ParametricEqFactory) as Arc<dyn NativeEffectFactory>,
                &preparation,
            )
            .expect("launch rate owner");
        }
        let mut unsupported = preparation();
        unsupported.sample_rate = 176_400;
        assert!(matches!(
            EffectControlOwner::new(
                Arc::new(ParametricEqFactory) as Arc<dyn NativeEffectFactory>,
                &unsupported,
            ),
            Err(EffectControlOwnerError::Rate)
        ));
    }

    #[test]
    fn invalid_edit_poison_survives_valid_overwrite_until_discard() {
        let mut rig = OwnerRig::new();
        rig.owner.begin(0).expect("base revision");
        assert!(rig.owner.edit(2, ParameterChannel::Left, f32::NAN).is_err());
        assert!(rig.owner.edit(2, ParameterChannel::Left, 100.0).is_err());
        assert!(rig.owner.prepare_targets_into(&mut []).is_err());
        rig.owner.discard().expect("discard poisoned candidate");
        rig.owner.begin(0).expect("restart at unchanged revision");
    }

    /// A target set wants no room: any number of transactions publish before a block reads, each
    /// is accepted, and the block applies the newest write of each cell.
    ///
    /// Test value: a publication path that still refuses for room (the removed `Capacity` error),
    /// which would turn a paused host's edits into `BACKPRESSURE` or a rebuild.
    #[test]
    fn a_target_set_wants_no_room_and_the_newest_write_of_each_cell_is_applied() {
        let mut rig = OwnerRig::new();
        let mut last = blank_targets();
        let mut last_count = 0;
        for revision in 0..40_u64 {
            rig.owner.begin(revision).expect("base revision");
            rig.owner
                .edit(2, ParameterChannel::Both, 100.0 + revision as f32)
                .expect("numeric edit");
            rig.owner
                .edit(2, ParameterChannel::Right, 200.0 + revision as f32)
                .expect("asymmetric target");
            last_count = rig.owner.prepare_targets_into(&mut last).expect("prepare");
            assert_eq!(last_count, 2);
            rig.publish(revision, &last[..last_count])
                .expect("a publication never refuses for room");
            rig.owner.commit().expect("commit");
        }
        assert_eq!(rig.owner.phase(), EffectControlOwnerPhase::Idle);
        let applied = rig.block();
        assert_eq!(applied, last[..last_count].to_vec());
        assert_eq!(
            rig.superseded.get(),
            2 * 39,
            "39 replaced writes on each of the two cells"
        );
    }

    #[test]
    fn stale_base_and_commit_before_publish_are_refused() {
        let mut rig = OwnerRig::new();
        assert!(rig.owner.begin(1).is_err(), "stale base");
        rig.owner.begin(0).expect("current base");
        assert!(rig.owner.commit().is_err(), "commit before publication");
        rig.owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("edit");
        let mut targets = blank_targets();
        let count = rig
            .owner
            .prepare_targets_into(&mut targets)
            .expect("prepare");
        rig.publish(1, &targets[..count])
            .expect_err("stale publication");
        assert_eq!(rig.owner.phase(), EffectControlOwnerPhase::Open);
        assert!(
            rig.block().is_empty(),
            "a refused publication wrote no cell"
        );
    }
}

/// Attach one live-control lane to every prepared effect of the session: the instance's
/// latest-target cells and, for an effect that declares observation taps when the plan has
/// observation capacity, its observation queue.
///
/// # The capacity rule that makes the render-side drain exact
///
/// A block stages one span for every live parameter cell it reads, in the block that reads it
/// (decision 15, D15-2 and #1345 D4), so an instance's staging window, which is exactly its
/// `automation_capacity`, must hold every live parameter cell of its effect.
/// [`prepare_native_session_effects`] raises each instance's capacity to at least
/// [`EffectLaneShape::live_parameter_cells`] of its descriptor, so this is an invariant of
/// preparation, not a runtime check on the render thread.
///
/// `observation_taps` is the observation capacity the host asked for (zero for none): with it, an
/// effect that declares taps gets a queue of one slot per declared tap, which holds the
/// subscriptions of one block in order.
///
/// # Errors
///
/// `effect.control.prepare` if a lane cannot be built, `effect.control.capacity` if an effect's
/// automation capacity is below its live parameter cells (which preparation makes unreachable),
/// and `effect.control.owner` if a prepared-target owner cannot be built.
pub fn attach_effect_live_controls(
    prepared: &mut EffectPreparedSession,
    superseded: &LiveValuesSuperseded,
    observation_taps: u32,
) -> Result<Vec<EffectControlProducer>, EffectDiagnosticSet> {
    attach_live_controls(
        &prepared.session,
        prepared
            .entries
            .iter_mut()
            .map(|entry| &mut entry.projected),
        superseded,
        observation_taps,
    )
}

/// [`attach_effect_live_controls`] on a projected session (issue #2199): the lanes attach to the
/// instances before any processor is prepared, and travel with each into the graph compile.
///
/// # Errors
///
/// As [`attach_effect_live_controls`].
pub fn attach_projected_effect_live_controls(
    projected: &mut EffectProjectedSession,
    superseded: &LiveValuesSuperseded,
    observation_taps: u32,
) -> Result<Vec<EffectControlProducer>, EffectDiagnosticSet> {
    attach_live_controls(
        &projected.session,
        projected.entries.iter_mut(),
        superseded,
        observation_taps,
    )
}

/// The one live-control attach behind [`attach_effect_live_controls`] and
/// [`attach_projected_effect_live_controls`].
fn attach_live_controls<'e>(
    session: &CompiledSession,
    entries: impl ExactSizeIterator<Item = &'e mut EffectProjectedEntry>,
    superseded: &LiveValuesSuperseded,
    observation_taps: u32,
) -> Result<Vec<EffectControlProducer>, EffectDiagnosticSet> {
    let declared = declared_live_addresses(session);
    let mut producers = Vec::with_capacity(entries.len());
    let mut diagnostics = Vec::new();
    for entry in entries {
        let (address, spec) = match live_lane_spec(entry, &declared, observation_taps) {
            Ok(value) => value,
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                continue;
            }
        };
        let Some((writer, lane)) = effect_control_lane(spec, superseded) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path: entry.path(),
            });
            continue;
        };
        let owner = if entry.factory.target_preparation().is_some() {
            match EffectControlOwner::new(Arc::clone(&entry.factory), &entry.bank_preparation) {
                Ok(owner) => Some(Box::new(owner)),
                Err(_) => {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.control.owner",
                        path: entry.path(),
                    });
                    continue;
                }
            }
        } else {
            None
        };
        producers.push(EffectControlProducer {
            track_id: entry.track_id.as_str().into(),
            address,
            effect_id: entry.effect_id.as_str().into(),
            descriptor: entry.factory.descriptor(),
            writer,
            owner,
        });
        entry.control = Some(Box::new(lane));
    }
    if diagnostics.is_empty() {
        Ok(producers)
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// The address and the lane spec of `entry`'s live-control lane: the one derivation behind
/// [`attach_live_controls`] and [`plan_projected_effect_live_controls`], with their per-entry
/// refusals (`effect.control.capacity`, `effect.control.prepare`).
fn live_lane_spec(
    entry: &EffectProjectedEntry,
    declared: &BTreeMap<(&str, EffectRack, &str), LiveEffectAddress>,
    observation_taps: u32,
) -> Result<(LiveEffectAddress, EffectLaneSpec), EffectDiagnostic> {
    let descriptor = entry.factory.descriptor();
    let target_slots = entry
        .factory
        .target_preparation()
        .map_or(0, |capability| capability.target_slots());
    let shape = EffectLaneShape::from_descriptor(descriptor, target_slots);
    if (entry.metadata.automation_capacity as usize) < shape.parameter_cells() {
        return Err(EffectDiagnostic {
            code: "effect.control.capacity",
            path: entry.path(),
        });
    }
    let Some(&address) = declared.get(&(
        entry.track_id.as_str(),
        entry.rack,
        entry.effect_id.as_str(),
    )) else {
        return Err(EffectDiagnostic {
            code: "effect.control.prepare",
            path: entry.path(),
        });
    };
    let observation_depth = if observation_taps == 0 {
        0
    } else {
        descriptor.observations.len()
    };
    // Seeded from the session's bypass, not the prepared one: an effect whose bypass is
    // lowered is prepared enabled (issue #1087), and this lane replaces the channel-less
    // lane that carried the same bit.
    // #1371 D6: the prepared link mode, which a plan-swap carry seats before a restore.
    let link = shape.has_link().then_some(entry.metadata.link_mode);
    Ok((
        address,
        EffectLaneSpec {
            shape,
            bypass: entry.initial_bypass,
            observation_depth,
            link,
        },
    ))
}

/// Lay out the live-control lanes [`attach_projected_effect_live_controls`] would attach, without
/// attaching any (issue #2199): every entry's [`EffectProjectedEntry::planned_lane`] is set to the
/// layout of the lane it would get, and the producers' [`EffectControlResources`] are returned as
/// [`effect_control_resources`] would walk them over the attached producers. A host projects a
/// session's graph and control rows from these before it allocates a lane, and then attaches.
///
/// # Errors
///
/// The diagnostics [`attach_projected_effect_live_controls`] would return.
pub fn plan_projected_effect_live_controls(
    projected: &mut EffectProjectedSession,
    observation_taps: u32,
) -> Result<EffectControlResources, EffectDiagnosticSet> {
    let declared = declared_live_addresses(&projected.session);
    let arithmetic = || {
        EffectDiagnosticSet::sorted(vec![EffectDiagnostic {
            code: "effect.control.resource.arithmetic",
            path: "$.effects".to_owned(),
        }])
    };
    let mut diagnostics = Vec::new();
    let mut owned_payload_bytes = 0_u64;
    let mut largest_owned_allocation_bytes = 0_u64;
    let mut factory_payload_bytes = 0_u64;
    let mut charged_factories: Vec<&Arc<dyn NativeEffectFactory>> = Vec::new();
    let mut planned = Vec::with_capacity(projected.entries.len());
    for entry in &projected.entries {
        let (_, spec) = match live_lane_spec(entry, &declared, observation_taps) {
            Ok(value) => value,
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                planned.push(None);
                continue;
            }
        };
        let Some(lane) = spec.project() else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path: entry.path(),
            });
            planned.push(None);
            continue;
        };
        if entry.factory.target_preparation().is_some() {
            match EffectControlOwner::projected_resources(&entry.factory, &entry.bank_preparation) {
                Ok(facts) => {
                    let facts = facts.map_err(|_| arithmetic())?;
                    owned_payload_bytes = owned_payload_bytes
                        .checked_add(facts.owned_payload_bytes)
                        .ok_or_else(arithmetic)?;
                    largest_owned_allocation_bytes =
                        largest_owned_allocation_bytes.max(facts.largest_owned_allocation_bytes);
                    // #1469 Amendment 1: a factory the launch registry owns is process-level
                    // memory the plan's clone does not allocate; any other is charged once.
                    if !charged_factories
                        .iter()
                        .any(|charged| Arc::ptr_eq(charged, &entry.factory))
                        && !launch_registry_owns_factory(&entry.factory)
                    {
                        charged_factories.push(&entry.factory);
                        factory_payload_bytes = factory_payload_bytes
                            .checked_add(facts.factory_allocation_bytes)
                            .ok_or_else(arithmetic)?;
                        largest_owned_allocation_bytes =
                            largest_owned_allocation_bytes.max(facts.factory_allocation_bytes);
                    }
                }
                Err(_) => {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.control.owner",
                        path: entry.path(),
                    });
                    planned.push(None);
                    continue;
                }
            }
        }
        for text in [&entry.track_id, &entry.effect_id] {
            let bytes = u64::try_from(text.len()).map_err(|_| arithmetic())?;
            owned_payload_bytes = owned_payload_bytes
                .checked_add(bytes)
                .ok_or_else(arithmetic)?;
            largest_owned_allocation_bytes = largest_owned_allocation_bytes.max(bytes);
        }
        owned_payload_bytes = owned_payload_bytes
            .checked_add(lane.writer.total_bytes)
            .ok_or_else(arithmetic)?;
        largest_owned_allocation_bytes =
            largest_owned_allocation_bytes.max(lane.writer.largest_allocation_bytes);
        planned.push(Some(lane.lane));
    }
    if !diagnostics.is_empty() {
        return Err(EffectDiagnosticSet::sorted(diagnostics));
    }
    for (entry, lane) in projected.entries.iter_mut().zip(planned) {
        entry.planned_lane = lane;
    }
    // The producer table is built at exactly one entry per instance (`attach_live_controls`).
    let producer_table_bytes = projected
        .entries
        .len()
        .checked_mul(core::mem::size_of::<EffectControlProducer>())
        .and_then(|bytes| u64::try_from(bytes).ok())
        .ok_or_else(arithmetic)?;
    Ok(EffectControlResources {
        producer_table_bytes,
        owned_payload_bytes: owned_payload_bytes
            .checked_add(factory_payload_bytes)
            .ok_or_else(arithmetic)?,
        largest_owned_allocation_bytes,
    })
}

/// The control-side reader half of one prepared effect instance's observation taps (issue #143).
///
/// Addressed exactly as an [`EffectControlProducer`] is -- by `(track_id, address)` -- because a
/// subscription and the live values it correlates with address the same instance through the same
/// numbers. `readers[i]` belongs to `descriptor.observations[i]`.
pub struct EffectObservationHandle {
    /// Normalized identity of the strip (a track or a submix) this instance belongs to.
    pub track_id: Box<str>,
    /// The instance's live address within its strip, a track or a submix (issue #1096).
    pub address: LiveEffectAddress,
    /// The instance's session-declared identifier.
    pub effect_id: Box<str>,
    /// The effect's declared menu, so an admitting host maps a wire `tap_id` to a `tap_index` and
    /// checks the cost class without a second copy of the registry.
    pub descriptor: &'static EffectDescriptor,
    /// One reader per declared tap, in declaration order. Each tap's arm state is in its slot,
    /// which render writes ([`ObservationReader::read_state`], #1585 D6).
    pub readers: Box<[ObservationReader]>,
}

/// One observation tap a successor plan is prepared with armed (#1584 D6a): the host acknowledged
/// the subscription before the rebuild, and the rebuild must not drop it.
///
/// An instance is addressed by its strip, its section (a console slot or an insert) and its
/// session ID, and the tap by its effect-local ID. Prefer building these from the host's
/// subscription table; an arm whose instance or tap the plan does not have is ignored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationArm<'a> {
    /// The strip (a track or a submix) the instance belongs to.
    pub strip_id: &'a str,
    /// Whether the instance is a console slot (`true`) or an insert (`false`).
    pub console: bool,
    /// The console slot ID or the insert's effect ID.
    pub effect_id: &'a str,
    /// The effect-local tap ID ([`effect_contract::ObservationTapId`]).
    pub tap_id: u32,
    /// The window length in blocks; `0` is the plan's meter window.
    pub window_blocks: u32,
    /// The tag of the record that armed the tap (#1585 D4), which the plan's first block publishes
    /// with the arm; `0` for none.
    pub arm_tag: u64,
}

/// Attach observation capacity to every prepared effect that declares at least one tap.
///
/// # Level 1 of the two-level zero (issue #143 D3)
///
/// This function is the **only** thing that creates an [`ObservationLane`]. A session whose
/// live-control request named no observation capacity never calls it, so its compiled plan contains
/// no lane, no accumulator and no conflating cell -- not a disabled one, none. That is what makes
/// "observation off costs nothing" an identity rather than a claim, and it is what
/// `observation_retained_bytes == 0` reports.
///
/// An effect that declares no tap gets no lane either, for the same reason: `miso.delay` has
/// nothing to observe, so it carries nothing.
///
/// `window_blocks` is the plan's default window length in render blocks. It is the *meter* window,
/// derived by the host from the same `live_control_meter_blocks` the peak meters use, so a
/// gain-reduction value and the peak beside it in one `miso.meter.v1` frame describe the same span
/// of samples.
///
/// # Errors
///
/// `effect.observation.prepare` if an instance cannot be located in the declared order, and
/// `effect.observation.taps` if an effect declares more taps than the request's cap allows.
pub fn attach_effect_observation(
    prepared: &mut EffectPreparedSession,
    maximum_taps: u32,
    window_blocks: u32,
    arms: &[ObservationArm<'_>],
) -> Result<Vec<EffectObservationHandle>, EffectDiagnosticSet> {
    attach_observation(
        &prepared.session,
        prepared
            .entries
            .iter_mut()
            .map(|entry| &mut entry.projected),
        maximum_taps,
        window_blocks,
        arms,
    )
}

/// [`attach_effect_observation`] on a projected session (issue #2199).
///
/// # Errors
///
/// As [`attach_effect_observation`].
pub fn attach_projected_effect_observation(
    projected: &mut EffectProjectedSession,
    maximum_taps: u32,
    window_blocks: u32,
    arms: &[ObservationArm<'_>],
) -> Result<Vec<EffectObservationHandle>, EffectDiagnosticSet> {
    attach_observation(
        &projected.session,
        projected.entries.iter_mut(),
        maximum_taps,
        window_blocks,
        arms,
    )
}

/// The one observation attach behind [`attach_effect_observation`] and
/// [`attach_projected_effect_observation`].
fn attach_observation<'e>(
    session: &CompiledSession,
    entries: impl Iterator<Item = &'e mut EffectProjectedEntry>,
    maximum_taps: u32,
    window_blocks: u32,
    arms: &[ObservationArm<'_>],
) -> Result<Vec<EffectObservationHandle>, EffectDiagnosticSet> {
    let declared = declared_live_addresses(session);
    // The arms by instance, so each entry finds its own in one lookup.
    // (strip, console, instance ID) -> (tap ID, window blocks, arm tag) of each armed tap.
    type ArmedTaps<'a> = BTreeMap<(&'a str, bool, &'a str), Vec<(u32, u32, u64)>>;
    let mut armed = ArmedTaps::new();
    for arm in arms {
        armed
            .entry((arm.strip_id, arm.console, arm.effect_id))
            .or_default()
            .push((arm.tap_id, arm.window_blocks, arm.arm_tag));
    }
    let mut handles = Vec::new();
    let mut diagnostics = Vec::new();
    for entry in entries {
        let descriptor = entry.factory.descriptor();
        if descriptor.observations.is_empty() {
            continue;
        }
        let path = format!("{}.effects[id={}]", entry.strip_path, entry.effect_id);
        if descriptor.observations.len() > maximum_taps as usize {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.taps",
                path,
            });
            continue;
        }
        let Some(&address) = declared.get(&(
            entry.track_id.as_str(),
            entry.rack,
            entry.effect_id.as_str(),
        )) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.prepare",
                path,
            });
            continue;
        };
        let mut publishers = Vec::with_capacity(descriptor.observations.len());
        let mut readers = Vec::with_capacity(descriptor.observations.len());
        for _ in descriptor.observations {
            let (publisher, reader) = observation_slot();
            publishers.push(publisher);
            readers.push(reader);
        }
        let Some(mut lane) =
            ObservationLane::new(descriptor.observations, publishers, window_blocks)
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.prepare",
                path,
            });
            continue;
        };
        // #1584 D6a: a tap the host acknowledged a subscription for is armed here, to open its
        // first window at its first rendered block. An arm for a tap the effect does not declare
        // is ignored: the host drops such a binding before it reaches here.
        let console = !matches!(entry.rack, EffectRack::Dynamic);
        if let Some(taps) = armed.get(&(entry.track_id.as_str(), console, entry.effect_id.as_str()))
        {
            for &(tap_id, window, arm_tag) in taps {
                if let Some(index) = descriptor
                    .observations
                    .iter()
                    .position(|tap| tap.id.0 == tap_id)
                    && let Ok(index) = u32::try_from(index)
                {
                    lane.arm_prepared(index, window, arm_tag);
                }
            }
        }
        handles.push(EffectObservationHandle {
            track_id: entry.track_id.as_str().into(),
            address,
            effect_id: entry.effect_id.as_str().into(),
            descriptor,
            readers: readers.into_boxed_slice(),
        });
        entry.observation = Some(Box::new(lane));
    }
    if diagnostics.is_empty() {
        Ok(handles)
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// The live address of every lowered instance, from the normalized model (issue #1096).
///
/// Keyed by the prepared entry's graph identity `(track, internal rack, effect id)`. A
/// `pre_insert` slot at position `k` is console slot `k`, a `post_insert` slot at position `j` is
/// console slot `pre_insert.len() + j` -- its index in the track's `console` array -- and an insert
/// at position `i` is insert `i`. Extracted once so the live-control attach and the observation
/// attach cannot disagree about what "console slot 2" means.
fn declared_live_addresses(
    session: &CompiledSession,
) -> BTreeMap<(&str, EffectRack, &str), LiveEffectAddress> {
    let mut declared = BTreeMap::new();
    let model = session.normalized_model();
    for strip in model.strips() {
        for (index, slot) in model.console.slots().take(strip.console.len()).enumerate() {
            let rack = if index < model.console.pre_insert.len() {
                EffectRack::Simd1
            } else {
                EffectRack::Simd2
            };
            declared.insert(
                (strip.id.as_str(), rack, slot.slot.as_str()),
                LiveEffectAddress::console(index as u32),
            );
        }
        for (index, effect) in strip.inserts.effects.iter().enumerate() {
            declared.insert(
                (strip.id.as_str(), EffectRack::Dynamic, effect.id.as_str()),
                LiveEffectAddress::insert(index as u32),
            );
        }
    }
    declared
}

/// Resolve one effect instance's session `params` into the initial values it is prepared with
/// (issue #1264 D1): the one value authority that preparation and the live classifier share.
///
/// In order, for each declared parameter: a `params` entry whose unit is not the descriptor's is
/// `effect.parameter.unit_mismatch`; a `Shared` parameter takes only a `Both` value
/// (`effect.parameter.channel` otherwise) and resolves to one [`ParameterChannel::Both`] value; a
/// `PerLane` parameter takes either one `Both` value or per-lane values
/// (`effect.parameter.duplicate_channel` otherwise) and resolves to a `Left` and a `Right` value,
/// the lane's own value before `Both`. A missing value is the descriptor's default, and every value
/// goes through [`effect_contract::normalize_zero`]. Then a resolved value outside its domain
/// ([`effect_contract::parameter_value_valid`]) is `effect.parameter.domain`, and a `params` entry
/// naming no declared parameter is `effect.parameter.unknown`.
///
/// The values come back in descriptor order, `Left` before `Right`, so two resolutions against
/// one descriptor pair up index by index.
///
/// # Errors
///
/// The diagnostic code of the first check that refuses.
pub fn resolve_initial_values(
    descriptor: &EffectDescriptor,
    params: &[session::EffectParam],
) -> Result<Vec<InitialParameterValue>, &'static str> {
    let mut initial = Vec::new();
    for (index, parameter) in descriptor.parameters.iter().enumerate() {
        let mut values = [None; 3];
        let mut duplicate = false;
        let mut unit_mismatch = false;
        for item in params
            .iter()
            .filter(|item| item.parameter_id == parameter.id.0)
        {
            unit_mismatch |= !same_unit(item.unit, parameter.unit);
            let channel = match item.channel {
                SessionChannel::Left => 0,
                SessionChannel::Right => 1,
                SessionChannel::Both => 2,
            };
            duplicate |= values[channel].replace(item.value).is_some();
        }
        if unit_mismatch {
            return Err("effect.parameter.unit_mismatch");
        }
        let [left, right, both] = values;
        match parameter.channel_policy {
            ParameterChannelPolicy::Shared => {
                if duplicate || left.is_some() || right.is_some() {
                    return Err("effect.parameter.channel");
                }
                initial.push(InitialParameterValue {
                    parameter_index: index as u32,
                    channel: ParameterChannel::Both,
                    value: effect_contract::normalize_zero(both.unwrap_or(parameter.default_value)),
                });
            }
            ParameterChannelPolicy::PerLane => {
                if duplicate || (both.is_some() && (left.is_some() || right.is_some())) {
                    return Err("effect.parameter.duplicate_channel");
                }
                for (channel, requested) in [
                    (ParameterChannel::Left, left),
                    (ParameterChannel::Right, right),
                ] {
                    initial.push(InitialParameterValue {
                        parameter_index: index as u32,
                        channel,
                        value: effect_contract::normalize_zero(
                            requested.or(both).unwrap_or(parameter.default_value),
                        ),
                    });
                }
            }
        }
    }
    if initial.iter().any(|item| {
        !effect_contract::parameter_value_valid(
            &descriptor.parameters[item.parameter_index as usize],
            item.value,
        )
    }) {
        return Err("effect.parameter.domain");
    }
    if params.iter().any(|item| {
        !descriptor
            .parameters
            .iter()
            .any(|parameter| parameter.id.0 == item.parameter_id)
    }) {
        return Err("effect.parameter.unknown");
    }
    Ok(initial)
}

fn same_unit(session: SessionUnit, contract: ParameterUnit) -> bool {
    matches!(
        (session, contract),
        (SessionUnit::Db, ParameterUnit::Db)
            | (SessionUnit::Hz, ParameterUnit::Hz)
            | (SessionUnit::Milliseconds, ParameterUnit::Milliseconds)
            | (SessionUnit::Samples, ParameterUnit::Samples)
            | (SessionUnit::Linear, ParameterUnit::Linear)
            | (SessionUnit::Ratio, ParameterUnit::Ratio)
    )
}

/// Issue #1462 gate 1: an effect's `tail_and_rest` is evaluated once per (rate, quality) at
/// registry build, and never again by preparation or bank binding.
#[cfg(test)]
mod tail_bound_count_tests {
    use super::{EffectCompileCaps, prepare_native_session_effects};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use effect_contract::{
        AutomationRate, BankWidth, CompositionBound, EffectDescriptor, EffectId,
        EffectPrepareError, EffectProcessBlock, EffectQuality, LatencySamples, LinkModeSet,
        NativeEffectFactory, NativeEffectRegistry, NodeTailBound, ParameterChannelPolicy,
        ParameterDescriptor, ParameterDomain, ParameterId, ParameterLattice, ParameterMapping,
        ParameterUnit, PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
        PrepareEffectRequest, PreparedEffect, PreparedEffectBank, PreparedNativeEffect,
        ProcessReport, QualityDescriptor, ResetKind, RestBound, SmoothingRule, StatePayloadError,
        StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
        expected_prepared_metadata,
    };
    use session::{CompileCaps, compile_session, parse_session_json};

    /// Every evaluation of [`counted`]; only this module's effect states it.
    static EVALUATIONS: AtomicUsize = AtomicUsize::new(0);

    const TAIL: NodeTailBound = NodeTailBound {
        tail: TailSamples::Finite(7),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
        composition: CompositionBound::Unstated,
    };

    fn counted(_: u32, _: EffectQuality) -> NodeTailBound {
        EVALUATIONS.fetch_add(1, Ordering::SeqCst);
        TAIL
    }

    const fn port(id: &'static str) -> PortId {
        match PortId::new(id) {
            Ok(id) => id,
            Err(_) => panic!("valid test port ID"),
        }
    }

    const PARAMETERS: [ParameterDescriptor; 1] = [ParameterDescriptor {
        id: ParameterId(1),
        display_name: "Gain",
        name: "gain_db",
        display_unit: "dB",
        unit: ParameterUnit::Db,
        domain: ParameterDomain::Continuous,
        minimum: Some(-24.0),
        maximum: Some(24.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::Linear,
        smoothing_samples: 8,
        readable: true,
        automatable: true,
        enum_choices: &[],
        lattice: ParameterLattice::arithmetic(0.1, 1),
        prepared_reason: None,
    }];
    const PORTS: [PortDescriptor; 2] = [
        PortDescriptor {
            id: port("main-in"),
            role: PortRole::MainInput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptor {
            id: port("main-out"),
            role: PortRole::MainOutput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
    ];
    const fn quality(sample_rate: u32) -> QualityDescriptor {
        QualityDescriptor {
            quality: EffectQuality::Normal,
            sample_rate,
            latency: LatencySamples(0),
            maximum_state: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_fixed_bytes: 0,
            scratch_bytes_per_frame: 0,
        }
    }
    /// Four rows: every launch rate at `Normal`.
    const QUALITIES: [QualityDescriptor; 4] = [
        quality(44_100),
        quality(48_000),
        quality(88_200),
        quality(96_000),
    ];
    static DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        id: match EffectId::new("counted-tail") {
            Ok(id) => id,
            Err(_) => panic!("valid test effect ID"),
        },
        display_name: "Counted Tail",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        lane_link: false,
        parameters: &PARAMETERS,
        ports: &PORTS,
        qualities: &QUALITIES,
        tail_and_rest: counted,
        observations: &[],
    };

    /// Prepares and binds as every launch effect does: each instance's and each bank member's
    /// metadata through `expected_prepared_metadata`. The bank then declines, which is all the
    /// count needs.
    struct Factory;
    impl NativeEffectFactory for Factory {
        fn descriptor(&self) -> &'static EffectDescriptor {
            &DESCRIPTOR
        }
        fn prepared_inline_bytes(&self) -> u64 {
            size_of::<Processor>() as u64
        }
        fn prepared_undeclared_heap_bytes(&self, _: &effect_contract::QualityDescriptor) -> u64 {
            0
        }
        fn designed_channel_symmetry(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<bool, EffectPrepareError> {
            self.prepare(request)
                .map(|prepared| prepared.processor.channel_symmetry())
        }

        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<PreparedEffect, EffectPrepareError> {
            Ok(PreparedEffect {
                processor: Box::new(Processor),
                metadata: expected_prepared_metadata(&DESCRIPTOR, request)?,
            })
        }
        fn bind_homogeneous_bank(
            &self,
            request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<PreparedEffectBank>, EffectPrepareError> {
            for member in request.requests {
                expected_prepared_metadata(&DESCRIPTOR, *member)?;
            }
            Ok(None)
        }
    }
    struct Processor;
    impl PreparedNativeEffect for Processor {
        fn retained_bytes(&self) -> u64 {
            0
        }

        fn reset(&mut self, _: ResetKind) {}
        fn process(&mut self, _: EffectProcessBlock<'_>) -> ProcessReport {
            ProcessReport::default()
        }
        fn snapshot_state_payload(
            &self,
            _: StatePayloadOutput<'_>,
        ) -> Result<(), StatePayloadError> {
            Ok(())
        }
        fn restore_state_payload(
            &mut self,
            _: u32,
            _: StatePayloadInput<'_>,
        ) -> Result<(), StatePayloadError> {
            Ok(())
        }
    }

    /// `tracks` tracks, each with two inserts of the counted effect, all routed to the output.
    fn session_json(tracks: usize) -> String {
        let insert = |id: &str| {
            format!(
                r#"{{"id":"{id}","identity":{{"kind":"native","effect_id":"counted-tail"}},
                "quality":"normal","bypass":false,"link_mode":"dual_mono",
                "params":[{{"parameter_id":1,"channel":"both","unit":"db","value":0.0}}],
                "sidechain":{{"kind":"none"}}}}"#
            )
        };
        let builtins = r#"{"polarity_invert":false,"trim_db":0.0,"hpf_hz":20.0,
            "lpf_hz":20000.0,"delay_samples":0}"#;
        let track = |index: usize| {
            format!(
                r#"{{"id":"t{index}","source_id":"voice","left_source_channel":0,
                "right_source_channel":1,"builtins":{{"left":{builtins},"right":{builtins}}},
                "console":[],"inserts":{{"effects":[{},{}]}},
                "fader":{{"left_db":0.0,"right_db":0.0,"left_mute":false,"right_mute":false}},
                "pan":{{"left":1.0,"right":1.0,"smoothing_samples":16}}}}"#,
                insert("a"),
                insert("b")
            )
        };
        let route = |index: usize| {
            format!(
                r#"{{"id":"r{index}","source":{{"kind":"track","track_id":"t{index}",
                "tap":"post_pan"}},"destination":{{"kind":"output_input","output_id":"main-out"}},
                "channel_matrix":{{"ll":1.0,"lr":0.0,"rl":0.0,"rr":1.0}},"gain_db":0.0,
                "mute":false,"follows_mute":false}}"#
            )
        };
        let join = |items: Vec<String>| items.join(",");
        format!(
            r#"{{"schema_version":1,"session_id":"counted.session","revision":"1",
            "sample_rate_hz":48000,"quantum_frames":128,
            "render_profile":{{"id":"native","mode":"single_thread"}},
            "output_profile":{{"id":"main","channels":2,"sample_format":"f32_planar"}},
            "sources":[{{"id":"voice",
            "content":"blake3:2a97516c354b68848cdbd8f54a226a0a55b21ed138e207ad6c5cbb9c00aa5aea",
            "channels":2,"bit_depth":"32f","frames":"48000"}}],
            "console":{{"pre_insert":[],"post_insert":[]}},
            "tracks":[{}],"submixes":[],"vcas":[],"control_smoothing":{{"kind":"default"}},"outputs":[{{"id":"main-out"}}],
            "routes":[{}],"automation":[]}}"#,
            join((0..tracks).map(track).collect()),
            join((0..tracks).map(route).collect())
        )
    }

    /// Building the registry evaluates the statement once per declared row (four), and preparing
    /// sixteen instances and binding them as four-lane banks evaluates it no further; every
    /// prepared instance carries the stated values.
    ///
    /// Red mutation: `expected_prepared_metadata` calls `tail_and_rest` again instead of reading
    /// the request's table entry (each instance, each compiler check and each bank member then
    /// adds one), or the registry evaluates a row more than once.
    #[test]
    fn preparation_and_bank_binding_never_evaluate_the_tail_bound() {
        const TRACKS: usize = 8;
        let registry =
            NativeEffectRegistry::new([Box::new(Factory) as Box<dyn NativeEffectFactory>])
                .expect("the counted effect is admitted");
        assert_eq!(EVALUATIONS.load(Ordering::SeqCst), QUALITIES.len());

        let model = parse_session_json(&session_json(TRACKS)).expect("a valid session");
        let session = compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("a compiled session");
        let prepared = prepare_native_session_effects(
            &session,
            &registry,
            EffectCompileCaps {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 32,
            },
        )
        .expect("the counted effect prepares");
        assert_eq!(prepared.entries.len(), TRACKS * 2);
        for entry in &prepared.entries {
            assert_eq!(
                (
                    entry.metadata.tail,
                    entry.metadata.tail_every_peak,
                    entry.metadata.rest
                ),
                (TAIL.tail, TAIL.tail_every_peak, TAIL.rest)
            );
        }

        // Bind as the graph compiler does: each bank's requests are its members' replayed
        // preparations (`EffectBankPreparation::request`).
        for members in prepared.entries.chunks(BankWidth::Four.lanes() as usize) {
            let requests: Vec<_> = members
                .iter()
                .map(|entry| entry.bank_preparation.request())
                .collect();
            let bound = members[0]
                .factory
                .bind_homogeneous_bank(PrepareEffectBankRequest {
                    backend: BankWidth::Four.backend(),
                    width: BankWidth::Four,
                    requests: &requests,
                    active_mask: BankWidth::Four.full_mask(),
                })
                .expect("every member's metadata derives");
            assert!(bound.is_none());
        }
        assert_eq!(EVALUATIONS.load(Ordering::SeqCst), QUALITIES.len());
    }
}
