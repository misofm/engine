use core::num::NonZeroUsize;
use effect_contract::{
    EffectControlLane, EffectControlRecord, EffectDescriptor, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, NativeEffectRegistry, ObservationLane, ParameterChannel,
    ParameterChannelPolicy, ParameterUnit, PrepareEffectLimits, PrepareEffectRequest,
    PreparedEffectMetadata, PreparedNativeEffect, PreparedPorts, PreparedSidechainPort,
    RegistryError, expected_prepared_metadata,
};
use engine::realtime::{
    ObservationReader, Producer, QueueFull, QueueGeneration, bounded_spsc, observation_slot,
};
use session::{
    CompiledSession, EffectIdentity, LinkMode as SessionLinkMode,
    ParameterChannel as SessionChannel, ParameterUnit as SessionUnit, SidechainDeclaration,
};
use std::collections::BTreeMap;
use std::sync::Arc;

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
        }
    }
}

/// The internal rack an effect instance was lowered into (decision 12, class A by lowering):
/// `Simd1` holds the session's `console.pre_insert` slots, `Dynamic` the track's `inserts` and
/// `Simd2` the `console.post_insert` slots.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum EffectRack {
    Simd1,
    Dynamic,
    Simd2,
}
pub struct EffectPreparedSession {
    pub session: CompiledSession,
    pub entries: Vec<EffectPreparedEntry>,
}

/// Construct the caller-injected native registry for the V1 launch effect set.
///
/// Registry construction is control-plane work. Callers retain and inject the immutable registry
/// into [`prepare_native_session_effects`]; there is no render-reachable global catalog.
pub fn launch_native_effect_registry() -> Result<NativeEffectRegistry, RegistryError> {
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
    for track in &model.tracks {
        // Decision 12, class A by lowering: `pre_insert` is the first internal rack, the track's
        // inserts the second and `post_insert` the third, each console entry an ordinary effect.
        let lowered = model.lower_track(track);
        let [pre_insert, inserts, post_insert] = lowered.in_chain_order();
        for (rack, effects) in [
            (EffectRack::Simd1, pre_insert),
            (EffectRack::Dynamic, inserts),
            (EffectRack::Simd2, post_insert),
        ] {
            for effect in effects {
                let path = format!("$.tracks[id={}].effects[id={}]", track.id, effect.id);
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
                if !descriptor.qualities.iter().any(|item| {
                    item.quality == quality && item.sample_rate == session.sample_rate().0
                }) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.quality.unsupported",
                        path,
                    });
                    continue;
                }
                let mut initial = Vec::new();
                let mut invalid = false;
                for (index, parameter) in descriptor.parameters.iter().enumerate() {
                    let matching: Vec<_> = effect
                        .params
                        .iter()
                        .filter(|item| item.parameter_id == parameter.id.0)
                        .collect();
                    if matching
                        .iter()
                        .any(|item| !same_unit(item.unit, parameter.unit))
                    {
                        diagnostics.push(EffectDiagnostic {
                            code: "effect.parameter.unit_mismatch",
                            path: path.clone(),
                        });
                        invalid = true;
                        break;
                    }
                    match parameter.channel_policy {
                        ParameterChannelPolicy::Shared => {
                            let values: Vec<_> = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Both)
                                .collect();
                            if matching.len() != values.len() || values.len() > 1 {
                                diagnostics.push(EffectDiagnostic {
                                    code: "effect.parameter.channel",
                                    path: path.clone(),
                                });
                                invalid = true;
                                break;
                            }
                            initial.push(InitialParameterValue {
                                parameter_index: index as u32,
                                channel: ParameterChannel::Both,
                                value: effect_contract::normalize_zero(
                                    values
                                        .first()
                                        .map_or(parameter.default_value, |item| item.value),
                                ),
                            });
                        }
                        ParameterChannelPolicy::PerLane => {
                            let both_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Both)
                                .count();
                            let left_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Left)
                                .count();
                            let right_count = matching
                                .iter()
                                .filter(|item| item.channel == SessionChannel::Right)
                                .count();
                            if both_count > 1
                                || left_count > 1
                                || right_count > 1
                                || (both_count == 1 && (left_count != 0 || right_count != 0))
                            {
                                diagnostics.push(EffectDiagnostic {
                                    code: "effect.parameter.duplicate_channel",
                                    path: path.clone(),
                                });
                                invalid = true;
                                break;
                            }
                            let both = matching
                                .iter()
                                .find(|item| item.channel == SessionChannel::Both)
                                .map(|item| item.value);
                            for (channel, requested) in [
                                (
                                    ParameterChannel::Left,
                                    matching
                                        .iter()
                                        .find(|item| item.channel == SessionChannel::Left)
                                        .map(|item| item.value),
                                ),
                                (
                                    ParameterChannel::Right,
                                    matching
                                        .iter()
                                        .find(|item| item.channel == SessionChannel::Right)
                                        .map(|item| item.value),
                                ),
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
                if invalid {
                    continue;
                }
                if let Some(item) = initial.iter().find(|item| {
                    !effect_contract::parameter_value_valid(
                        &descriptor.parameters[item.parameter_index as usize],
                        item.value,
                    )
                }) {
                    let _ = item;
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.parameter.domain",
                        path,
                    });
                    continue;
                }
                if effect.params.iter().any(|item| {
                    !descriptor
                        .parameters
                        .iter()
                        .any(|parameter| parameter.id.0 == item.parameter_id)
                }) {
                    diagnostics.push(EffectDiagnostic {
                        code: "effect.parameter.unknown",
                        path,
                    });
                    continue;
                }
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
                let processor = match factory.prepare(request) {
                    Ok(value) => value,
                    Err(error) => {
                        diagnostics.push(EffectDiagnostic {
                            code: error.code,
                            path,
                        });
                        continue;
                    }
                };
                let metadata = processor.metadata();
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
                    track_id: track.id.as_str().to_owned(),
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
/// entry. A producer must be dropped before the plan that owns its consumer, which is why
/// `PreparedHost`'s field order puts the plan first.
pub struct EffectControlProducer {
    /// Session-stable track identity this channel addresses.
    pub track_id: Box<str>,
    /// Which rack the effect sits in.
    pub rack: EffectRack,
    /// Zero-based position of the effect **within its rack, in session declaration order**.
    ///
    /// This is the `effect_index` the `miso.command.v1` wire addresses, and it is derived from the
    /// normalized session model here rather than from `EffectPreparedSession::entries`, which is
    /// sorted by effect id.
    pub effect_index: u32,
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
            if !shared_factory {
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
mod owner_tests {
    use super::*;
    use crate::EffectControlOwnerPhase;
    use core::alloc::Layout;
    use core::mem::size_of;
    use core::num::NonZeroUsize;
    use core::sync::atomic::AtomicUsize;
    use effect_contract::{
        EffectControlRecord, EffectPrepareError, NativeEffectFactory, ParameterChannel,
        PrepareEffectBankRequest, PreparedEffectTarget, PreparedNativeEffect,
        PreparedNativeEffectBank, default_initial_values,
    };
    use engine::realtime::{QueueGeneration, bounded_spsc};
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use std::sync::Arc;

    /// Test-only wrapper for retained factory ownership and attachment accounting.
    /// Production ParametricEqFactory now exposes the same prepared-target capability.
    struct OptInEqFactory;

    impl NativeEffectFactory for OptInEqFactory {
        fn descriptor(&self) -> &'static EffectDescriptor {
            &PARAMETRIC_EQ_DESCRIPTOR
        }

        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            ParametricEqFactory.prepare(request)
        }

        fn target_preparation(
            &self,
        ) -> Option<&dyn effect_contract::NativeEffectTargetPreparation> {
            Some(&ParametricEqFactory)
        }

        fn bind_homogeneous_bank(
            &self,
            request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            ParametricEqFactory.bind_homogeneous_bank(request)
        }
    }

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
        }
    }

    fn owner_and_queue() -> (
        EffectControlOwner,
        EffectControlProducerHandle,
        engine::realtime::Consumer<EffectControlRecord>,
    ) {
        let preparation = preparation();
        let factory: Arc<dyn NativeEffectFactory> = Arc::new(OptInEqFactory);
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
            rack: EffectRack::Dynamic,
            effect_index: 0,
            effect_id: effect_id.into(),
            descriptor: &PARAMETRIC_EQ_DESCRIPTOR,
            producer: EffectControlProducerHandle::new(producer, owner.is_some()),
            owner,
        }
    }

    #[test]
    fn effect_control_resources_charge_actual_tables_and_shared_owner_once() {
        let factory: Arc<dyn NativeEffectFactory> = Arc::new(OptInEqFactory);
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
            EffectControlOwner::new(
                Arc::new(OptInEqFactory) as Arc<dyn NativeEffectFactory>,
                &preparation,
            )
            .expect("launch rate owner");
        }
        let mut unsupported = preparation();
        unsupported.sample_rate = 176_400;
        assert!(matches!(
            EffectControlOwner::new(
                Arc::new(OptInEqFactory) as Arc<dyn NativeEffectFactory>,
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
    let declared = declared_effect_indices(&prepared.session);
    let mut producers = Vec::with_capacity(prepared.entries.len());
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
        let path = format!(
            "$.tracks[id={}].effects[id={}]",
            entry.track_id, entry.effect_id
        );
        let Some(capacity) = NonZeroUsize::new(entry.metadata.automation_capacity as usize) else {
            diagnostics.push(EffectDiagnostic {
                code: "effect.control.capacity",
                path,
            });
            continue;
        };
        let Some(&effect_index) =
            declared.get(&(entry.track_id.clone(), entry.rack, entry.effect_id.clone()))
        else {
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
                        path: format!(
                            "$.tracks[id={}].effects[id={}]",
                            entry.track_id, entry.effect_id
                        ),
                    });
                    continue;
                }
            }
        } else {
            None
        };
        producers.push(EffectControlProducer {
            track_id: entry.track_id.as_str().into(),
            rack: entry.rack,
            effect_index,
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
/// Addressed exactly as an [`EffectControlProducer`] is -- by `(track_id, rack, effect_index)` --
/// because a subscription and the parameter commands it correlates with address the same instance
/// through the same numbers. `readers[i]` belongs to `descriptor.observations[i]`.
pub struct EffectObservationHandle {
    /// Normalized track identity this instance belongs to.
    pub track_id: Box<str>,
    /// Which rack of that track.
    pub rack: EffectRack,
    /// Declared position within the rack, in session declaration order.
    pub effect_index: u32,
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
    let declared = declared_effect_indices(&prepared.session);
    let mut handles = Vec::new();
    let mut diagnostics = Vec::new();
    for entry in &mut prepared.entries {
        let descriptor = entry.factory.descriptor();
        if descriptor.observations.is_empty() {
            continue;
        }
        let path = format!(
            "$.tracks[id={}].effects[id={}]",
            entry.track_id, entry.effect_id
        );
        if descriptor.observations.len() > maximum_taps as usize {
            diagnostics.push(EffectDiagnostic {
                code: "effect.observation.taps",
                path,
            });
            continue;
        }
        let Some(&effect_index) =
            declared.get(&(entry.track_id.clone(), entry.rack, entry.effect_id.clone()))
        else {
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
            rack: entry.rack,
            effect_index,
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

/// Declared position within each `(track, rack)`, from the normalized model.
///
/// The same order the `miso.command.v1` `effect_index` names and the same order the browser host
/// counts, extracted once so the live-control attach and the observation attach cannot disagree
/// about what "effect 2 of the dynamic rack" means.
fn declared_effect_indices(
    session: &CompiledSession,
) -> BTreeMap<(String, EffectRack, String), u32> {
    let mut declared: BTreeMap<(String, EffectRack, String), u32> = BTreeMap::new();
    let model = session.normalized_model();
    for track in &model.tracks {
        let lowered = model.lower_track(track);
        let [pre_insert, inserts, post_insert] = lowered.in_chain_order();
        for (rack, effects) in [
            (EffectRack::Simd1, pre_insert),
            (EffectRack::Dynamic, inserts),
            (EffectRack::Simd2, post_insert),
        ] {
            for (index, effect) in effects.iter().enumerate() {
                declared.insert(
                    (track.id.to_string(), rack, effect.id.to_string()),
                    index as u32,
                );
            }
        }
    }
    declared
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
