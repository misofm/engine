use miso_engine_effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory, NativeEffectRegistry,
    ParameterChannel, ParameterChannelPolicy, ParameterUnit, PrepareEffectLimits,
    PrepareEffectRequest, PreparedEffectMetadata, PreparedNativeEffect, PreparedPortsV1,
    PreparedSidechainPort, RegistryError, expected_prepared_metadata,
};
use miso_engine_session::{
    CompiledSession, EffectIdentity, LinkMode as SessionLinkMode,
    ParameterChannel as SessionChannel, ParameterUnit as SessionUnit, SidechainDeclaration,
};
use std::sync::Arc;

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
    pub factory: Arc<dyn miso_engine_effect_contract::NativeEffectFactory>,
    /// Exact owned request inputs used to prepare the scalar processor.
    pub bank_preparation: EffectBankPreparationV1,
}

/// Owned replayable portion of an accepted prepare request. It never crosses into render.
#[derive(Clone, Debug)]
pub struct EffectBankPreparationV1 {
    pub sample_rate: u32,
    pub quantum: u32,
    pub quality: EffectQuality,
    pub bypass: bool,
    pub link_mode: LinkMode,
    pub ports: PreparedPortsV1,
    pub initial_values: Box<[InitialParameterValue]>,
    pub limits: PrepareEffectLimits,
}

impl EffectBankPreparationV1 {
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
pub fn launch_native_effect_registry_v1() -> Result<NativeEffectRegistry, RegistryError> {
    NativeEffectRegistry::new([
        Box::new(miso_engine_parametric_eq::ParametricEqFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_compressor::CompressorFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_gate_expander::GateExpanderFactory) as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_multiband_compressor::MultibandCompressorFactory)
            as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_true_peak_limiter::TruePeakLimiterFactory)
            as Box<dyn NativeEffectFactory>,
        Box::new(miso_engine_soft_clip::SoftClipFactory) as Box<dyn NativeEffectFactory>,
    ])
}

pub fn prepare_native_session_effects(
    session: &CompiledSession,
    registry: &NativeEffectRegistry,
    caps: EffectCompileCaps,
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
    for track in &session.normalized_model().tracks {
        for (rack, effects) in [
            (EffectRack::Simd1, &track.simd1.effects),
            (EffectRack::Dynamic, &track.dynamic.effects),
            (EffectRack::Simd2, &track.simd2.effects),
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
                    miso_engine_session::EffectQuality::Draft => EffectQuality::Draft,
                    miso_engine_session::EffectQuality::Normal => EffectQuality::Normal,
                    miso_engine_session::EffectQuality::High => EffectQuality::High,
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
                                value: normalize_zero(
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
                                    value: normalize_zero(
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
                    !parameter_value_is_valid(
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
                let declared_sidechain = descriptor.ports.iter().find(|port| {
                    port.role == miso_engine_effect_contract::PortRole::SidechainInput
                });
                let ports = match (&effect.sidechain, declared_sidechain) {
                    (SidechainDeclaration::None, None) => PreparedPortsV1 {
                        sidechain: PreparedSidechainPort::None,
                    },
                    (SidechainDeclaration::None, Some(port)) if !port.required => PreparedPortsV1 {
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
                            port.role == miso_engine_effect_contract::PortRole::SidechainInput
                                && port.id.as_str() == sidechain.port_id.as_str()
                        }) {
                            Some(port) => PreparedPortsV1 {
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
                let bank_preparation = EffectBankPreparationV1 {
                    sample_rate: session.sample_rate().0,
                    quantum: session.quantum().0,
                    quality,
                    bypass: effect.bypass,
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

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

fn parameter_value_is_valid(
    descriptor: &miso_engine_effect_contract::ParameterDescriptorV1,
    value: f32,
) -> bool {
    if !value.is_finite() {
        return false;
    }
    match descriptor.domain {
        miso_engine_effect_contract::ParameterDomain::Continuous => descriptor
            .minimum
            .zip(descriptor.maximum)
            .is_some_and(|(minimum, maximum)| value >= minimum && value <= maximum),
        miso_engine_effect_contract::ParameterDomain::Boolean => value == 0.0 || value == 1.0,
        miso_engine_effect_contract::ParameterDomain::Enumeration => {
            descriptor.enum_choices.iter().any(|choice| {
                normalize_zero(choice.value).to_bits() == normalize_zero(value).to_bits()
            })
        }
    }
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
