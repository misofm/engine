use core::num::NonZeroUsize;
use effect_contract::{
    AutomationRate, EffectControlLane, EffectControlRecord, EffectDescriptor, EffectQuality,
    InitialParameterValue, LinkMode, NativeEffectFactory, NativeEffectRegistry, ObservationLane,
    ParameterChannel, ParameterChannelPolicy, ParameterUnit, PrepareEffectLimits,
    PrepareEffectRequest, PreparedEffect, PreparedEffectMetadata, PreparedNativeEffect,
    PreparedPorts, PreparedSidechainPort, RegisteredTailBound, RegistryError,
    expected_prepared_metadata,
};
use engine::realtime::{
    ObservationReader, Producer, QueueFull, QueueGeneration, bounded_spsc, observation_slot,
};
use session::{
    CompiledSession, EffectIdentity, LinkMode as SessionLinkMode,
    ParameterChannel as SessionChannel, ParameterUnit as SessionUnit, RackName, SessionModel,
    SidechainDeclaration,
};
use std::collections::BTreeMap;
use std::sync::{Arc, OnceLock};

use crate::control::{EffectControlOwner, EffectControlOwnerError, EffectControlResourceError};
use crate::{EffectDiagnostic, EffectDiagnosticSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_scratch_bytes: u64,
    pub maximum_automation_spans_per_block: u32,
}
pub struct EffectPreparedEntry {
    pub track_id: String,
    /// The owning strip's compiler path prefix (`StripRef::path_prefix`): `$.tracks[id=<id>]` for
    /// a track. Diagnostics about this instance are `"{strip_path}.effects[id={effect}]"`.
    pub strip_path: Box<str>,
    pub rack: EffectRack,
    pub effect_id: String,
    pub processor: Box<dyn PreparedNativeEffect>,
    pub metadata: PreparedEffectMetadata,
    /// Factory retained only for transactional off-render bank binding.
    pub factory: Arc<dyn effect_contract::NativeEffectFactory>,
    /// Exact owned request inputs used to prepare the scalar processor.
    ///
    /// Its `bypass` is `false` for every effect whose session bypass is lowered to
    /// [`Self::initial_bypass`] (issue #1087), and the session's bypass for an effect in
    /// [`NEVER_BANKED_EFFECTS`] or [`PREPARED_BYPASS_EFFECTS`] ([`lowers_session_bypass`]).
    pub bank_preparation: EffectBankPreparation,
    /// The session's `bypass` for this instance (issue #1087).
    ///
    /// For an effect that [`lowers_session_bypass`] it is lowered to per-lane shunt state: the
    /// effect is prepared with `bypass = false`, so a bypassed and an enabled instance of one
    /// effect share one `EffectProgramKey` and one bank, and this bit is the initial state of the
    /// instance's [`EffectControlLane`], and so of the rack's latency-preserving shunt. A bypassed
    /// instance runs its wet path and emits its input delayed by its declared latency. It also
    /// seeds a live-control lane ([`attach_effect_live_controls`]) for every effect.
    pub initial_bypass: bool,
    /// The consumer half of this instance's live-control channel (issue #140 A), or the
    /// channel-less lane that carries a lowered session bypass (issue #1087).
    ///
    /// [`prepare_native_session_effects`] sets
    /// [`EffectControlLane::without_channel`]`(true)` on every bypassed instance of an effect that
    /// [`lowers_session_bypass`] and `None` on every other, and [`attach_effect_live_controls`]
    /// replaces it with a live channel seeded from [`Self::initial_bypass`]; nothing else creates
    /// one. It travels with the entry into `GraphPreparedEffect`, so the plan that renders the
    /// effect is the one that drains its queue and applies its shunt, and a session with no live
    /// controls and no bypassed instance carries a `None` that the runtime turns back into the
    /// byte-identical live-control-free path.
    pub control: Option<Box<EffectControlLane>>,
    /// This instance's observation taps (issue #143 D3, level 1).
    ///
    /// `None` unless [`attach_effect_observation`] was called, which is the only way one is
    /// ever created. A session whose live-control request named no observation capacity carries
    /// `None` here, and the runtime turns that back into the byte-identical unobserved path: there
    /// is no lane, no slot and no vector anywhere in the compiled plan.
    pub observation: Option<Box<ObservationLane>>,
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
/// This is what the browser's `miso.command.v1` record carries in its `rack` byte and
/// `effect_index` word, what the observation records carry, and what a native host looks a
/// channel up by ([`EffectControlProducer::address`]). Internally it maps through the lowering
/// ([`Self::lower`]).
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

/// The eight launch factories, in registry order. Called only by the static's one build.
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
    ])
}

/// The native effects a session console slot may name (owner decision 12, Sol's L2).
///
/// A console slot always banks, so it must be an effect whose homogeneous bank kernel the console
/// can rely on: the parametric EQ, compressor, gate/expander, soft-clip, transient shaper and
/// true-peak limiter. The delay never banks. The multiband compressor is excluded until #1069
/// closes. Anything else is refused with `console.slot.ineligible_effect` where native identities
/// resolve, in [`prepare_native_session_effects`]; the session schema refuses a third-party slot
/// before this runs.
pub const CONSOLE_ELIGIBLE_EFFECTS: [&str; 6] = [
    "miso.parametric-eq",
    "miso.compressor",
    "miso.gate-expander",
    "miso.soft-clip",
    "miso.transient-shaper",
    "miso.true-peak-limiter",
];

/// Launch effects whose factory binds no homogeneous bank at any width (issue #1087).
///
/// A session bypass is lowered to per-lane shunt state so that a bypassed track keeps its effect
/// bank, except on the effects this list and [`PREPARED_BYPASS_EFFECTS`] name
/// ([`lowers_session_bypass`]). An effect that never banks has no bank to keep, so its session bypass stays a prepared
/// bypass and it renders the per-node path it always did. For the delay that is also the only
/// exact choice: its block-boundary check (D7) reads its ring and damping state as well as its
/// output, so on a block where a runaway feedback ring trips it, its prepared bypass zeroes the dry
/// block, which a shunt would pass through unchanged.
///
/// `graph-compiler`'s `bypass_shunt_identity` test holds this list to exactly the launch factories
/// that decline a well-formed bank at the host's width.
pub const NEVER_BANKED_EFFECTS: [&str; 1] = ["miso.delay"];

/// Launch effects that bank, but whose session bypass stays a prepared bypass (issue #1100).
///
/// A lowered bypass keeps a bypassed lane in a bank beside enabled lanes, and a bank's
/// block-boundary recovery (D7) zeroes the whole bank when any lane trips it. A bypassed lane still
/// runs its wet path, so a bypassed multiband lane fed a legal but extreme input (about `6e29` at
/// its defaults) would trip that recovery and silence its enabled bank-mates. That couples bits
/// across lanes, which decision 12 forbids. The other bankable effects make their recovery per lane
/// in their own slices (#1089, #1090, #1092); nothing does so for the multiband yet.
///
/// So a session bypass on an effect listed here is prepared exactly as it was before #1087:
/// `bypass = true`, no lane, and a program key that differs from an enabled instance's, so a mixed
/// bypass cohort declines a bank and a uniform one still binds. This is a separate list from
/// [`NEVER_BANKED_EFFECTS`] because these effects do bank. The slice that makes the multiband
/// console-eligible, after #1069, owns its per-lane D7 and removes it from this list.
pub const PREPARED_BYPASS_EFFECTS: [&str; 1] = ["miso.multiband-compressor"];

/// Whether a session bypass on `effect_id` is lowered to per-lane shunt state (issue #1087), or
/// stays a prepared bypass: for an effect that never banks, or one in
/// [`PREPARED_BYPASS_EFFECTS`].
#[must_use]
pub fn lowers_session_bypass(effect_id: &str) -> bool {
    !NEVER_BANKED_EFFECTS.contains(&effect_id) && !PREPARED_BYPASS_EFFECTS.contains(&effect_id)
}

pub fn prepare_native_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
    prepare_with_console_eligibility(session, registry, caps, &CONSOLE_ELIGIBLE_EFFECTS)
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
    prepare_with_console_eligibility(session, registry, caps, console_eligible)
}

/// Refuses every stored automation whose effect target the plan keeps prepared (decision 15 E1,
/// #1335 D1, D2).
///
/// An automation whose rack is `inserts` or `console` is accepted only when the target instance is
/// a native effect whose descriptor parameter with that ID is `automatable` and has
/// [`AutomationRate::Block`]. Anything else is one `effect.automation.rate` diagnostic at
/// `$.automation[id=<automation id>].target.parameter_id`. A `Sample` parameter has no rendering
/// contract yet, so it is refused too.
///
/// - An `inserts` target is the target strip's insert whose `id` is the target's `effect_id`.
/// - A `console` target's identity is the session console slot whose `slot` is the `effect_id`,
///   never the strip's entry, which carries no identity.
/// - A `builtins` target is not an effect and is not checked here.
///
/// Skipped, because each has its own diagnostic elsewhere and must not be reported twice: a target
/// strip or instance the session does not declare (session validation), a third-party identity
/// (`effect.third_party.unavailable_at_launch`), an effect the registry lacks
/// (`effect.native.unavailable`) and a parameter ID the descriptor lacks
/// (`effect.parameter.unknown`, since session validation makes the target one of the instance's
/// declared `params`).
///
/// Control plane only: it allocates its result.
#[must_use]
pub fn effect_automation_diagnostics(
    model: &SessionModel,
    registry: &NativeEffectRegistry,
) -> Vec<EffectDiagnostic> {
    let mut diagnostics = Vec::new();
    for automation in &model.automation {
        let target = &automation.target;
        let Some(strip) = model.strips().find(|strip| strip.id == &target.entity_id) else {
            continue;
        };
        let identity = match target.rack {
            RackName::Inserts => strip
                .inserts
                .effects
                .iter()
                .find(|effect| effect.id == target.effect_id)
                .map(|effect| &effect.identity),
            RackName::Console => model
                .console
                .slots()
                .find(|slot| slot.slot == target.effect_id)
                .map(|slot| &slot.identity),
            RackName::Builtins => None,
        };
        let Some(EffectIdentity::Native { effect_id }) = identity else {
            continue;
        };
        let Some(factory) = registry.get_ascii(effect_id.as_str()) else {
            continue;
        };
        let Some(parameter) = factory
            .descriptor()
            .parameters
            .iter()
            .find(|parameter| parameter.id.0 == target.parameter_id)
        else {
            continue;
        };
        if !parameter.automatable || parameter.automation_rate != AutomationRate::Block {
            diagnostics.push(EffectDiagnostic {
                code: "effect.automation.rate",
                path: format!(
                    "$.automation[id={}].target.parameter_id",
                    automation.id.as_str()
                ),
            });
        }
    }
    diagnostics
}

/// The one preparation behind both entries: the console slots whose native identity is not in
/// `console_eligible` are refused, and everything else is prepared.
fn prepare_with_console_eligibility(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
    console_eligible: &[&str],
) -> Result<EffectPreparedSession, EffectDiagnosticSet> {
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
    for strip in model.strips() {
        // Decision 12, class A by lowering: `pre_insert` is the first internal rack, the strip's
        // inserts the second and `post_insert` the third, each console entry an ordinary effect.
        let lowered = model.lower_strip(&strip);
        let strip_path = strip.path_prefix();
        let [pre_insert, inserts, post_insert] = lowered.in_chain_order();
        for (rack, effects) in [
            (EffectRack::Simd1, pre_insert),
            (EffectRack::Dynamic, inserts),
            (EffectRack::Simd2, post_insert),
        ] {
            for effect in effects {
                let path = format!("{strip_path}.effects[id={}]", effect.id);
                let EffectIdentity::Native { effect_id } = &effect.identity else {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.third_party.unavailable_at_launch",
                        path,
                    });
                    continue;
                };
                let Some(factory) = registry.get_shared_ascii(effect_id.as_str()) else {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.native.unavailable",
                        path,
                    });
                    continue;
                };
                let descriptor = factory.descriptor();
                let quality = match effect.quality {
                    session::EffectQuality::Draft => EffectQuality::Draft,
                    session::EffectQuality::Normal => EffectQuality::Normal,
                    session::EffectQuality::High => EffectQuality::High,
                };
                let link_mode = match effect.link_mode {
                    SessionLinkMode::DualMono => LinkMode::DualMono,
                    SessionLinkMode::Maximum => LinkMode::Maximum,
                    SessionLinkMode::Average => LinkMode::Average,
                };
                if !descriptor.supported_link_modes.contains(link_mode) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.link_mode.unsupported",
                        path,
                    });
                    continue;
                }
                // Issue #1462 D1: the registry's table entry, not a call of the descriptor's
                // `tail_and_rest`. A rate or quality the effect does not declare has no entry and
                // is `effect.quality.unsupported`, as it always was.
                let tail_bound =
                    match registry.tail_bound(descriptor.id, session.sample_rate().0, quality) {
                        Ok(entry) => entry,
                        Err(error) => {
                            diagnostics.push(EffectDiagnostic {
                                code: error.code,
                                path,
                            });
                            continue;
                        }
                    };
                let initial = match resolve_initial_values(descriptor, &effect.params) {
                    Ok(initial) => initial,
                    Err(code) => {
                        diagnostics.push(EffectDiagnostic { code, path });
                        continue;
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
                        continue;
                    }
                    (SidechainDeclaration::Routed(_), None) => {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.sidechain.unexpected",
                            path,
                        });
                        continue;
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
                                continue;
                            }
                        }
                    }
                };
                // Issue #1087: a session's bypass is per-lane shunt state, not a prepared
                // program. An effect that can bank is prepared enabled, so it shares its program
                // key and its bank with every enabled instance, and the bit rides the instance's
                // control lane to the rack's shunt, which emits the latency-matched dry signal for
                // this lane. An effect that never banks keeps its prepared bypass, and so does one
                // whose bank recovery is still whole-bank (issue #1100).
                let lowered = lowers_session_bypass(effect_id.as_str());
                let bank_preparation = EffectBankPreparation {
                    sample_rate: session.sample_rate().0,
                    quantum: session.quantum().0,
                    quality,
                    bypass: effect.bypass && !lowered,
                    link_mode,
                    ports,
                    initial_values: initial.into_boxed_slice(),
                    limits: PrepareEffectLimits {
                        maximum_total_state_bytes: caps.maximum_total_state_bytes,
                        maximum_scratch_bytes: caps.maximum_scratch_bytes,
                        maximum_automation_spans_per_block: caps.maximum_automation_spans_per_block,
                    },
                    tail_bound,
                };
                let request = bank_preparation.request();
                let expected = match expected_prepared_metadata(descriptor, request) {
                    Ok(metadata) => metadata,
                    Err(error) => {
                        diagnostics.push(EffectDiagnostic {
                            code: error.code,
                            path,
                        });
                        continue;
                    }
                };
                // Issue #1461: the prepare result carries the metadata beside the processor,
                // which holds no copy of it; this compares that returned value, field by field.
                let PreparedEffect {
                    processor,
                    metadata,
                } = match factory.prepare(request) {
                    Ok(value) => value,
                    Err(error) => {
                        diagnostics.push(EffectDiagnostic {
                            code: error.code,
                            path,
                        });
                        continue;
                    }
                };
                if metadata.descriptor.id != expected.descriptor.id
                    || metadata.descriptor.contract_major != expected.descriptor.contract_major
                    || metadata.descriptor.state_layout_version
                        != expected.descriptor.state_layout_version
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
                    || metadata.state_sizes != expected.state_sizes
                    || metadata.scratch_bytes != expected.scratch_bytes
                    || metadata.automation_capacity != expected.automation_capacity
                {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.metadata.mismatch",
                        path,
                    });
                    continue;
                }
                entries.push(EffectPreparedEntry {
                    track_id: strip.id.as_str().to_owned(),
                    strip_path: strip_path.as_str().into(),
                    rack,
                    effect_id: effect.id.as_str().to_owned(),
                    processor,
                    metadata,
                    factory,
                    bank_preparation,
                    initial_bypass: effect.bypass,
                    control: (effect.bypass && lowered)
                        .then(|| Box::new(EffectControlLane::without_channel(true))),
                    observation: None,
                });
            }
        }
    }
    // Decision 15 E1 (#1335 D3): an effect automation whose target the plan keeps prepared is
    // refused here, so every host and `session-validator` stage 5 refuse it.
    diagnostics.extend(effect_automation_diagnostics(model, registry));
    if diagnostics.is_empty() {
        entries.sort_by(|a, b| {
            (&a.track_id, a.rack, &a.effect_id).cmp(&(&b.track_id, b.rack, &b.effect_id))
        });
        Ok(EffectPreparedSession {
            session: session.clone(),
            entries,
        })
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// One prepared live-control channel for one effect instance (issue #140 A).
///
/// The producer half stays on the control plane; the consumer half rode into the plan inside the
/// entry. The ring is an `Arc` the two halves share, so either may drop first and the last owner
/// frees it: `PreparedHost` drops its plan first, and capi drops a reclaimed plan before the
/// provider epoch that keeps its producers (#1263 D5), both on the control thread.
pub struct EffectControlProducer {
    /// Session-stable identity of the strip (a track or a submix) this channel addresses.
    pub track_id: Box<str>,
    /// The instance's live address within its strip (a track or a submix): a console slot by its
    /// slot index, an insert by its index (decision 12, issue #1096).
    ///
    /// This is the `(rack, effect_index)` the `miso.command.v1` wire addresses. It is derived from
    /// the normalized session model here rather than from `EffectPreparedSession::entries`, which
    /// is sorted by effect id.
    pub address: LiveEffectAddress,
    /// Session-stable effect instance identity.
    pub effect_id: Box<str>,
    /// The effect's declared parameter table, so an admitting host can map a wire `parameter_id`
    /// to the `parameter_index` the render side stages, and check the value's domain, without a
    /// second copy of the registry.
    pub descriptor: &'static EffectDescriptor,
    /// Checked bounded producer endpoint; unsupported prepared targets are refused before queue
    /// mutation, while ordinary records retain the queue's full-result retry semantics.
    producer: EffectControlProducerHandle,
    /// Optional off-audio-thread owner for a factory's prepared-target capability.
    ///
    /// The box is private so callers must use the checked transaction methods below; no raw
    /// producer or reusable "validated" flag can bypass candidate/revision admission.
    owner: Option<Box<EffectControlOwner>>,
}

/// Checked native allocations retained by one effect-control producer table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectControlResources {
    /// The actual backing allocation of the native producer `Vec`, using its capacity.
    pub producer_table_bytes: u64,
    /// Retained strings, owner boxes, owner row/dirty backings and distinct factory `Arc`s.
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

/// Why an effect-control publication failed.
#[derive(Debug)]
pub enum EffectControlPushError {
    /// This owner has no prepared-target capability at this checkpoint. The record was not
    /// published and is returned for the caller's refusal report.
    Unsupported { record: EffectControlRecord },
    /// The bounded queue was full. The underlying queue retains its generation and full counter.
    Full(QueueFull<EffectControlRecord>),
}

/// Checked control-plane endpoint for one effect queue.
///
/// The underlying producer is intentionally private: callers cannot bypass target capability
/// checks or manufacture a production prepared-target route by setting a flag. Component tests
/// that need to exercise the target consumer construct an exclusively owned low-level queue and
/// lane directly.
pub struct EffectControlProducerHandle {
    inner: Producer<EffectControlRecord>,
    requires_prepared_targets: bool,
}

impl EffectControlProducerHandle {
    fn new(inner: Producer<EffectControlRecord>, requires_prepared_targets: bool) -> Self {
        Self {
            inner,
            requires_prepared_targets,
        }
    }

    /// Exact usable queue capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Producer-side capacity snapshot used for complete target-prefix admission.
    #[must_use]
    pub fn available_capacity(&self) -> usize {
        self.inner.available_capacity()
    }

    /// Producer-local successful publication count.
    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.inner.success_count()
    }

    /// Producer-local full/overflow count.
    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.inner.full_count()
    }

    /// Check delivery capability without touching the queue, counters, or record ownership.
    ///
    /// A prepared-capable owner will eventually require all EQ parameter records to arrive with
    /// their companion targets. Assignment4 keeps production owners unregistered, so this path
    /// remains fail-closed until the owner transaction is available.
    pub fn preflight(&self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        if matches!(record, EffectControlRecord::PreparedTarget(_))
            || (self.requires_prepared_targets
                && matches!(record, EffectControlRecord::Parameter { .. }))
        {
            return Err(EffectControlPushError::Unsupported { record });
        }
        Ok(())
    }

    /// Publish one record after checking whether this owner can deliver prepared targets.
    pub fn try_push(&mut self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.preflight(record)?;
        self.inner
            .try_push(record)
            .map_err(EffectControlPushError::Full)
    }

    pub(crate) fn try_push_prepared(
        &mut self,
        target: effect_contract::PreparedEffectTarget,
    ) -> Result<(), EffectControlPushError> {
        self.inner
            .try_push(EffectControlRecord::PreparedTarget(target))
            .map_err(EffectControlPushError::Full)
    }
}

impl EffectControlProducer {
    /// Read-only checked endpoint for capacity and accounting inspection.
    #[must_use]
    pub fn producer(&self) -> &EffectControlProducerHandle {
        &self.producer
    }

    /// Publishes one ordinary semantic/observation record through the checked endpoint.
    pub fn try_push(&mut self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.producer.try_push(record)
    }

    /// Checks one ordinary record without queue mutation.
    pub fn preflight(&self, record: EffectControlRecord) -> Result<(), EffectControlPushError> {
        self.producer.preflight(record)
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.producer.capacity()
    }

    #[must_use]
    pub const fn success_count(&self) -> u64 {
        self.producer.success_count()
    }

    #[must_use]
    pub const fn full_count(&self) -> u64 {
        self.producer.full_count()
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

    /// Publishes a validated target prefix and marks the owner ready for its one commit.
    pub fn publish_candidate_targets(
        &mut self,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref_mut()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.publish(&mut self.producer, base_revision, targets)
    }

    /// Preflights this owner's exact target prefix, including revision and complete queue room.
    /// The queue, candidate and owner phase remain unchanged.
    pub fn preflight_candidate_targets(
        &self,
        base_revision: u64,
        targets: &[effect_contract::PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let owner = self
            .owner
            .as_deref()
            .ok_or(EffectControlOwnerError::Unsupported)?;
        owner.preflight_publication(&self.producer, base_revision, targets)
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
mod control_producer_tests {
    use super::{EffectControlProducerHandle, EffectControlPushError};
    use core::num::NonZeroUsize;
    use effect_contract::{EffectControlRecord, ParameterChannel, PreparedEffectTarget};
    use engine::realtime::{QueueGeneration, bounded_spsc};

    fn target(slot: u32) -> EffectControlRecord {
        EffectControlRecord::PreparedTarget(PreparedEffectTarget {
            slot,
            channel: ParameterChannel::Left,
            words: [slot; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        })
    }

    fn parameter(value: f32) -> EffectControlRecord {
        EffectControlRecord::Parameter {
            parameter_index: 3,
            channel: ParameterChannel::Left,
            value,
        }
    }

    #[test]
    fn unsupported_target_is_returned_without_touching_queue_or_full_counter() {
        let (producer, consumer) = bounded_spsc::<EffectControlRecord>(
            NonZeroUsize::new(2).expect("capacity"),
            QueueGeneration(7),
        )
        .expect("queue");
        let mut producer = EffectControlProducerHandle::new(producer, false);
        producer
            .try_push(parameter(0.25))
            .expect("first queue slot");
        producer
            .try_push(parameter(0.5))
            .expect("second queue slot");
        let record = target(11);

        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 0);
        let refusal = producer.try_push(record).expect_err("unsupported target");
        match refusal {
            EffectControlPushError::Unsupported { record: returned } => {
                assert_eq!(returned, record);
            }
            EffectControlPushError::Full(_) => panic!("unsupported is distinct from full"),
        }
        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 0);
        assert_eq!(consumer.available_at_entry(), 2);
    }

    #[test]
    fn ordinary_semantic_record_publishes_and_full_refusal_preserves_original_record() {
        let (producer, mut consumer) = bounded_spsc::<EffectControlRecord>(
            NonZeroUsize::new(1).expect("capacity"),
            QueueGeneration(9),
        )
        .expect("queue");
        let mut producer = EffectControlProducerHandle::new(producer, false);
        let first = parameter(0.25);
        assert!(producer.try_push(first).is_ok());
        assert_eq!(producer.success_count(), 1);
        assert_eq!(producer.full_count(), 0);
        assert_eq!(consumer.try_pop().expect("semantic record"), first);

        let second = parameter(0.5);
        let third = parameter(0.75);
        producer.try_push(second).expect("room after pop");
        let refusal = producer.try_push(third).expect_err("full queue");
        match refusal {
            EffectControlPushError::Full(full) => {
                assert_eq!(full.value, third);
                assert_eq!(full.generation, QueueGeneration(9));
                assert_eq!(full.full_count, 1);
            }
            EffectControlPushError::Unsupported { .. } => panic!("ordinary record is supported"),
        }
        assert_eq!(producer.success_count(), 2);
        assert_eq!(producer.full_count(), 1);
        assert_eq!(consumer.available_at_entry(), 1);
        assert_eq!(
            consumer.try_pop().expect("retained semantic record"),
            second
        );
    }
}

#[cfg(test)]
mod metadata_mismatch_tests {
    use super::{EffectCompileCaps, prepare_native_session_effects};
    use effect_contract::{
        EffectDescriptor, EffectId, EffectPrepareError, EffectQuality, LatencySamples, LinkMode,
        NativeEffectFactory, NativeEffectRegistry, PortId, PrepareEffectBankRequest,
        PrepareEffectRequest, PreparedEffect, PreparedEffectBank, PreparedEffectMetadata,
        PreparedSidechainPort, RestBound, RestSamples, TailSamples,
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
    const FORGERIES: [(&str, Forge); 16] = [
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
                RestBound::Bounded(_) => RestBound::Unstated,
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
            maximum_automation_spans_per_block: 32,
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
    use core::alloc::Layout;
    use core::mem::size_of;
    use core::num::NonZeroUsize;
    use core::sync::atomic::AtomicUsize;
    use effect_contract::{
        EffectControlRecord, NativeEffectFactory, ParameterChannel, PreparedEffectTarget,
        default_initial_values,
    };
    use engine::realtime::{QueueGeneration, bounded_spsc};
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use std::sync::Arc;

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

    fn owner_and_queue() -> (
        EffectControlOwner,
        EffectControlProducerHandle,
        engine::realtime::Consumer<EffectControlRecord>,
    ) {
        let preparation = preparation();
        let factory: Arc<dyn NativeEffectFactory> = Arc::new(ParametricEqFactory);
        let owner = EffectControlOwner::new(factory, &preparation).expect("valid EQ owner");
        let (producer, consumer) = bounded_spsc(
            NonZeroUsize::new(12).expect("capacity"),
            QueueGeneration(31),
        )
        .expect("queue");
        (
            owner,
            EffectControlProducerHandle::new(producer, true),
            consumer,
        )
    }

    fn resource_producer(
        track_id: &str,
        effect_id: &str,
        factory: Option<Arc<dyn NativeEffectFactory>>,
    ) -> EffectControlProducer {
        let owner = factory.map(|factory| {
            Box::new(EffectControlOwner::new(factory, &preparation()).expect("valid EQ owner"))
        });
        let (producer, _consumer) = bounded_spsc(
            NonZeroUsize::new(12).expect("capacity"),
            QueueGeneration(33),
        )
        .expect("queue");
        EffectControlProducer {
            track_id: track_id.into(),
            address: LiveEffectAddress::insert(0),
            effect_id: effect_id.into(),
            descriptor: &PARAMETRIC_EQ_DESCRIPTOR,
            producer: EffectControlProducerHandle::new(producer, owner.is_some()),
            owner,
        }
    }

    #[test]
    fn effect_control_resources_charge_actual_tables_and_shared_owner_once() {
        let factory: Arc<dyn NativeEffectFactory> = Arc::new(ParametricEqFactory);
        let mut opted = Vec::with_capacity(5);
        opted.push(resource_producer(
            "track-a",
            "effect-a",
            Some(Arc::clone(&factory)),
        ));
        opted.push(resource_producer(
            "track-b",
            "effect-b",
            Some(Arc::clone(&factory)),
        ));
        let resources = effect_control_resources(&opted).expect("resource facts");
        let values = preparation().initial_values.len();
        let owner_box = size_of::<EffectControlOwner>() as u64;
        let row_backing = (values * size_of::<InitialParameterValue>()) as u64;
        let dirty_backing = (values * size_of::<bool>()) as u64;
        let owner_payload = owner_box + (2 * row_backing) + dirty_backing;
        let factory_layout = Layout::new::<AtomicUsize>()
            .extend(Layout::new::<AtomicUsize>())
            .expect("Arc header layout")
            .0
            .extend(Layout::for_value(factory.as_ref()))
            .expect("factory layout")
            .0
            .pad_to_align()
            .size() as u64;
        let expected_payload = "track-a".len() as u64
            + "effect-a".len() as u64
            + "track-b".len() as u64
            + "effect-b".len() as u64
            + (2 * owner_payload)
            + factory_layout;
        let expected_table = 5 * size_of::<EffectControlProducer>() as u64;
        let expected_largest = [
            "track-a".len() as u64,
            "effect-a".len() as u64,
            "track-b".len() as u64,
            "effect-b".len() as u64,
            owner_box,
            row_backing,
            dirty_backing,
            factory_layout,
        ]
        .into_iter()
        .max()
        .expect("nonempty resource rows");
        assert_eq!(resources.producer_table_bytes, expected_table);
        assert_eq!(resources.owned_payload_bytes, expected_payload);
        assert_eq!(resources.largest_owned_allocation_bytes, expected_largest);
        assert_eq!(
            resources.total_bytes(),
            Some(expected_table + expected_payload)
        );
        assert_eq!(
            resources.largest_allocation_bytes(),
            expected_table.max(expected_largest)
        );

        let mut production = Vec::with_capacity(3);
        production.push(resource_producer("track", "effect", None));
        let production_resources =
            effect_control_resources(&production).expect("production resource facts");
        assert_eq!(
            production_resources.producer_table_bytes,
            3 * size_of::<EffectControlProducer>() as u64
        );
        assert_eq!(
            production_resources.owned_payload_bytes,
            "track".len() as u64 + "effect".len() as u64
        );
        assert_eq!(
            production_resources.largest_owned_allocation_bytes,
            "effect".len() as u64
        );
    }

    /// #1469 Amendment 1. Red if a plan is charged for a factory the process-lifetime launch
    /// registry owns: the two owners share the registry's EQ factory, so the charge is their
    /// strings and owner payloads alone.
    #[test]
    fn effect_control_resources_charge_no_registry_owned_factory() {
        let factory = launch_native_effect_registry()
            .expect("launch registry")
            .get_shared_ascii("miso.parametric-eq")
            .expect("launch EQ");
        assert!(launch_registry_owns_factory(&factory));
        assert!(!launch_registry_owns_factory(
            &(Arc::new(ParametricEqFactory) as Arc<dyn NativeEffectFactory>)
        ));
        let mut producers = Vec::with_capacity(2);
        for (track, effect) in [("track-a", "effect-a"), ("track-b", "effect-b")] {
            producers.push(resource_producer(track, effect, Some(Arc::clone(&factory))));
        }
        let values = preparation().initial_values.len();
        let owner_payload = size_of::<EffectControlOwner>() as u64
            + (2 * (values * size_of::<InitialParameterValue>()) as u64)
            + (values * size_of::<bool>()) as u64;
        let strings = ["track-a", "effect-a", "track-b", "effect-b"]
            .map(|text| text.len() as u64)
            .into_iter()
            .sum::<u64>();
        let resources = effect_control_resources(&producers).expect("resource facts");
        assert_eq!(resources.owned_payload_bytes, strings + 2 * owner_payload);
    }

    #[test]
    fn opted_in_eq_owner_prepares_both_and_commits_once() {
        let (mut owner, mut producer, mut consumer) = owner_and_queue();
        let initial = owner.committed().to_vec();
        owner.begin(0).expect("base revision");
        owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("numeric Both edit");
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner
            .prepare_targets_into(&mut targets)
            .expect("off-audio preparation");
        assert_eq!(count, 1, "equal Both lanes coalesce");
        assert_eq!(targets[0].channel, ParameterChannel::Both);
        assert_eq!(
            owner.committed(),
            initial,
            "preparation leaves committed rows unchanged"
        );
        let final_rows = owner.candidate().to_vec();
        let changed: Vec<_> = final_rows
            .iter()
            .filter(|row| row.parameter_index == 2)
            .collect();
        assert_eq!(changed.len(), 2);
        assert!(changed.iter().all(|row| row.value == 100.0));
        owner
            .publish(&mut producer, 0, &targets[..count])
            .expect("complete prefix publication");
        assert_eq!(
            owner.committed(),
            initial,
            "publication precedes shadow commit"
        );
        assert!(owner.edit(2, ParameterChannel::Both, 200.0).is_err());
        assert!(owner.discard().is_err());
        assert_eq!(owner.commit().expect("one commit"), 1);
        assert_eq!(owner.committed(), final_rows);
        assert!(owner.commit().is_err(), "repeat commit refused");
        assert_eq!(
            consumer.try_pop(),
            Ok(EffectControlRecord::PreparedTarget(targets[0]))
        );
    }

    #[test]
    fn opted_in_owner_accepts_two_transactions_before_render() {
        let (mut owner, mut producer, mut consumer) = owner_and_queue();
        let initial = owner.committed().to_vec();
        let mut first = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        owner.begin(0).expect("first base revision");
        owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("first Both edit");
        let first_count = owner
            .prepare_targets_into(&mut first)
            .expect("first prepare");
        assert_eq!(first_count, 1);
        owner
            .publish(&mut producer, 0, &first[..first_count])
            .expect("first publication");
        assert_eq!(owner.commit(), Ok(1));

        let mut second = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        owner.begin(1).expect("second base revision before render");
        owner
            .edit(2, ParameterChannel::Right, 200.0)
            .expect("second asymmetric edit");
        let second_count = owner
            .prepare_targets_into(&mut second)
            .expect("second prepare");
        assert_eq!(second_count, 1);
        assert_eq!(second[0].channel, ParameterChannel::Right);
        owner
            .publish(&mut producer, 1, &second[..second_count])
            .expect("second publication");
        assert_eq!(owner.commit(), Ok(2));
        assert_eq!(
            consumer.try_pop(),
            Ok(EffectControlRecord::PreparedTarget(first[0]))
        );
        assert_eq!(
            consumer.try_pop(),
            Ok(EffectControlRecord::PreparedTarget(second[0]))
        );
        assert_eq!(owner.committed_revision(), 2);
        assert_eq!(
            owner
                .committed()
                .iter()
                .find(|row| { row.parameter_index == 2 && row.channel == ParameterChannel::Left })
                .expect("left committed row")
                .value,
            100.0
        );
        assert_eq!(
            owner
                .committed()
                .iter()
                .find(|row| { row.parameter_index == 2 && row.channel == ParameterChannel::Right })
                .expect("right committed row")
                .value,
            200.0
        );
        assert_ne!(
            owner.committed(),
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
        let (mut owner, _producer, _consumer) = owner_and_queue();
        owner.begin(0).expect("base revision");
        assert!(owner.edit(2, ParameterChannel::Left, f32::NAN).is_err());
        assert!(owner.edit(2, ParameterChannel::Left, 100.0).is_err());
        assert!(owner.prepare_targets_into(&mut []).is_err());
        owner.discard().expect("discard poisoned candidate");
        owner.begin(0).expect("restart at unchanged revision");
    }

    #[test]
    fn full_prefix_refusal_preserves_queue_and_candidate() {
        let (mut owner, mut producer, mut consumer) = owner_and_queue();
        for slot in 0..11 {
            producer
                .try_push(EffectControlRecord::Bypass(slot % 2 == 0))
                .expect("fill queue");
        }
        let initial = owner.committed().to_vec();
        owner.begin(0).expect("base revision");
        owner
            .edit(2, ParameterChannel::Both, 100.0)
            .expect("numeric edit");
        owner
            .edit(2, ParameterChannel::Right, 200.0)
            .expect("asymmetric target");
        let candidate = owner.candidate().to_vec();
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner.prepare_targets_into(&mut targets).expect("prepare");
        assert_eq!(count, 2);
        assert_eq!(producer.available_capacity(), 1);
        let successes = producer.success_count();
        let full = producer.full_count();
        assert_eq!(
            owner.publish(&mut producer, 0, &targets[..count]),
            Err(EffectControlOwnerError::Capacity)
        );
        assert_eq!(owner.committed(), initial);
        assert_eq!(owner.candidate(), candidate);
        assert_eq!(owner.committed_revision(), 0);
        assert_eq!(consumer.available_at_entry(), 11);
        assert_eq!(
            (producer.success_count(), producer.full_count()),
            (successes, full)
        );
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Open);
        for slot in 0..11 {
            assert_eq!(
                consumer.try_pop(),
                Ok(EffectControlRecord::Bypass(slot % 2 == 0))
            );
        }
        assert_eq!(consumer.available_at_entry(), 0);
    }

    #[test]
    fn stale_base_and_commit_before_publish_are_refused() {
        let (mut owner, mut producer, _consumer) = owner_and_queue();
        assert!(owner.begin(1).is_err(), "stale base");
        owner.begin(0).expect("current base");
        assert!(owner.commit().is_err(), "commit before publication");
        owner.edit(2, ParameterChannel::Both, 100.0).expect("edit");
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Left,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; 12];
        let count = owner.prepare_targets_into(&mut targets).expect("prepare");
        owner
            .publish(&mut producer, 1, &targets[..count])
            .expect_err("stale publication");
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Open);
    }
}

/// Attach one bounded live-control channel to every prepared effect of the session.
///
/// # The capacity rule that makes the render-side drain exact
///
/// A channel is prepared at `min(depth, automation_capacity)` records. Each drained
/// [`EffectControlRecord::Parameter`] produces at most one span and duplicates collapse, so a
/// full drain can never stage more spans than the effect's own
/// [`PreparedEffectMetadata::automation_capacity`](effect_contract::PreparedEffectMetadata).
/// "The staging window cannot overflow" is therefore an invariant of preparation, not a runtime
/// check on the render thread.
///
/// # Errors
///
/// `effect.control.prepare` if a bounded queue cannot be built, and
/// `effect.control.capacity` if an effect declares a zero automation capacity, which no launch
/// effect does and which would leave the channel unable to deliver anything.
pub fn attach_effect_live_controls(
    prepared: &mut EffectPreparedSession,
    depth: NonZeroUsize,
) -> Result<Vec<EffectControlProducer>, EffectDiagnosticSet> {
    let declared = declared_live_addresses(&prepared.session);
    let mut producers = Vec::with_capacity(prepared.entries.len());
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
        let path = format!("{}.effects[id={}]", entry.strip_path, entry.effect_id);
        let Some(capacity) = NonZeroUsize::new(entry.metadata.automation_capacity as usize) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.capacity",
                path,
            });
            continue;
        };
        let Some(&address) = declared.get(&(
            entry.track_id.as_str(),
            entry.rack,
            entry.effect_id.as_str(),
        )) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path,
            });
            continue;
        };
        let Ok((producer, consumer)) =
            bounded_spsc::<EffectControlRecord>(depth.min(capacity), QueueGeneration(0))
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.prepare",
                path,
            });
            continue;
        };
        let target_capable = entry.factory.target_preparation().is_some();
        let owner = if target_capable {
            match EffectControlOwner::new(Arc::clone(&entry.factory), &entry.bank_preparation) {
                Ok(owner) => Some(Box::new(owner)),
                Err(_) => {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.control.owner",
                        path,
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
            producer: EffectControlProducerHandle::new(producer, owner.is_some()),
            owner,
        });
        // Seeded from the session's bypass, not the prepared one: an effect whose bypass is
        // lowered is prepared enabled (issue #1087), and this channel replaces the channel-less
        // lane that carried the same bit. For the delay and the multiband (`lowers_session_bypass`
        // is false) the prepared bypass stays in force, so a live toggle cannot lift it.
        entry.control = Some(Box::new(if target_capable {
            EffectControlLane::new_with_target_staging(consumer, entry.initial_bypass)
        } else {
            EffectControlLane::new(consumer, entry.initial_bypass)
        }));
    }
    if diagnostics.is_empty() {
        Ok(producers)
    } else {
        Err(EffectDiagnosticSet::sorted(diagnostics))
    }
}

/// The control-side reader half of one prepared effect instance's observation taps (issue #143).
///
/// Addressed exactly as an [`EffectControlProducer`] is -- by `(track_id, address)` -- because a
/// subscription and the parameter commands it correlates with address the same instance through
/// the same numbers. `readers[i]` belongs to `descriptor.observations[i]`.
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
    /// One reader per declared tap, in declaration order.
    pub readers: Box<[ObservationReader]>,
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
) -> Result<Vec<EffectObservationHandle>, EffectDiagnosticSet> {
    let declared = declared_live_addresses(&prepared.session);
    let mut handles = Vec::new();
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
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
        let Some(lane) = ObservationLane::new(descriptor.observations, publishers, window_blocks)
        else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.prepare",
                path,
            });
            continue;
        };
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
        AutomationRate, BankWidth, EffectDescriptor, EffectId, EffectPrepareError,
        EffectProcessBlock, EffectQuality, EffectTailBound, LatencySamples, LinkModeSet,
        NativeEffectFactory, NativeEffectRegistry, ParameterChannelPolicy, ParameterDescriptor,
        ParameterDomain, ParameterId, ParameterLattice, ParameterMapping, ParameterUnit,
        PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectBankRequest,
        PrepareEffectRequest, PreparedEffect, PreparedEffectBank, PreparedNativeEffect,
        ProcessReport, QualityDescriptor, ResetKind, RestBound, SmoothingRule, StatePayloadError,
        StatePayloadInput, StatePayloadOutput, StatePayloadSizes, TailSamples,
        expected_prepared_metadata,
    };
    use session::{CompileCaps, compile_session, parse_session_json};

    /// Every evaluation of [`counted`]; only this module's effect states it.
    static EVALUATIONS: AtomicUsize = AtomicUsize::new(0);

    const TAIL: EffectTailBound = EffectTailBound {
        tail: TailSamples::Finite(7),
        tail_every_peak: TailSamples::Infinite,
        rest: RestBound::Unstated,
    };

    fn counted(_: u32, _: EffectQuality) -> EffectTailBound {
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
            "tracks":[{}],"submixes":[],"vcas":[],"outputs":[{{"id":"main-out"}}],
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
