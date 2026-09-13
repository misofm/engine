//! Explicit, stopped response previews owned by the native effect providers.
//!
//! This module is deliberately a small bridge.  It does not parse a session, inspect a graph or
//! retain a render plan: a caller supplies one native effect configuration (or one pair of input
//! filters), the owner prepares it through its existing response factory, and a query writes into
//! caller-owned buffers.  The browser adapter uses the same entry point, so native and Wasm
//! previews cannot grow separate numerical implementations.

use builtins::{BuiltinParameters, prepare_input_filter_response};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, ParameterChannel, PreparedPorts,
    PreparedResponseAnalysis, ResponseAnalysisError, ResponseOutput, ResponsePrepareLimits,
    ResponseQuery, ResponseSummary, default_initial_values,
};
use math::{exp, log};

/// The explicit target supplied by a stopped preview caller.
#[derive(Clone, Copy, Debug)]
pub enum ResponsePreviewTarget<'a> {
    /// One native effect owner, identified by its generated effect ID.
    Effect {
        /// Generated native effect identity.
        effect_id: &'a str,
        /// Numeric parameter/channel overrides over descriptor defaults.
        overrides: &'a [ResponseParameterOverride],
        /// Native preparation quality.
        quality: EffectQuality,
        /// Effect-wide bypass.
        bypass: bool,
        /// Native channel-link mode.
        link_mode: LinkMode,
    },
    /// The builtin input HPF/LPF pair. Other builtin sections remain at their defaults.
    InputFilters {
        /// Left high-pass cutoff, in Hz.
        left_hpf_hz: f32,
        /// Left low-pass cutoff, in Hz.
        left_lpf_hz: f32,
        /// Right high-pass cutoff, in Hz.
        right_hpf_hz: f32,
        /// Right low-pass cutoff, in Hz.
        right_lpf_hz: f32,
    },
}

/// One numeric override in descriptor parameter ID/channel space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseParameterOverride {
    /// Native descriptor parameter ID.
    pub parameter_id: u32,
    /// Native parameter channel.
    pub channel: ParameterChannel,
    /// The descriptor-domain value, before owner validation.
    pub value: f32,
}

/// Bounded preparation resources for one response preview.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponsePreviewLimits {
    /// Maximum concrete owner payload retained by preparation.
    pub maximum_prepared_bytes: usize,
    /// Maximum ordinary native effect state admitted while validating the request.
    pub maximum_total_state_bytes: u64,
    /// Maximum ordinary native effect scratch admitted while validating the request.
    pub maximum_scratch_bytes: u64,
    /// Maximum ordinary automation span capacity admitted while validating the request.
    pub maximum_automation_spans_per_block: u32,
}

impl Default for ResponsePreviewLimits {
    fn default() -> Self {
        Self {
            maximum_prepared_bytes: 1 << 20,
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 30,
            maximum_automation_spans_per_block: 4096,
        }
    }
}

/// A generated response frequency axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponsePreviewGrid {
    /// Evenly spaced inclusive endpoints.
    Linear {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
    /// Logarithmically spaced inclusive positive endpoints.
    Logarithmic {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive positive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
}

impl ResponsePreviewGrid {
    /// Number of requested points.
    #[must_use]
    pub const fn points(self) -> usize {
        match self {
            Self::Linear { points, .. } | Self::Logarithmic { points, .. } => points,
        }
    }

    fn endpoints(self) -> (f32, f32) {
        match self {
            Self::Linear {
                minimum_hz,
                maximum_hz,
                ..
            }
            | Self::Logarithmic {
                minimum_hz,
                maximum_hz,
                ..
            } => (minimum_hz, maximum_hz),
        }
    }
}

/// A stopped response preparation request.
#[derive(Clone, Copy, Debug)]
pub struct ResponsePreviewRequest<'a> {
    /// Opaque caller correlation identity.
    pub configuration_id: u64,
    /// Explicit launch sample rate.
    pub sample_rate_hz: u32,
    /// Explicit render quantum used by native effect validation.
    pub quantum_frames: u32,
    /// One owner-local target.
    pub target: ResponsePreviewTarget<'a>,
    /// Preparation/resource caps.
    pub limits: ResponsePreviewLimits,
}

/// Caller-owned output buffers for one response query.
#[derive(Debug)]
pub struct ResponsePreviewOutput<'a> {
    /// Engine-generated frequency axis.
    pub frequencies_hz: &'a mut [f32],
    /// Left total curve.
    pub total_left_db: &'a mut [f32],
    /// Right total curve.
    pub total_right_db: &'a mut [f32],
    /// Optional section-major left curves.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional section-major right curves.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Why an explicit preview request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponsePreviewError {
    /// The requested launch rate/quantum or grid shape is invalid.
    InvalidShape,
    /// The requested native effect ID is absent.
    UnknownEffect,
    /// The effect exists but has no response provider.
    UnsupportedEffect,
    /// A parameter ID/channel row is not valid for this descriptor.
    InvalidParameter,
    /// A parameter ID/channel row was supplied more than once.
    DuplicateParameter,
    /// A shared parameter was addressed as a lane or a lane parameter as both.
    ConflictingChannel,
    /// The owner rejected the explicit preparation.
    Owner(ResponseAnalysisError),
    /// The generated axis is not finite, increasing and inside Nyquist.
    InvalidGrid,
    /// Caller buffers do not exactly match the requested shape.
    OutputShape,
}

/// An immutable native response owner prepared for repeated queries.
pub struct PreparedResponsePreview {
    owner: Box<dyn PreparedResponseAnalysis>,
    configuration_id: u64,
}

impl PreparedResponsePreview {
    /// The owner descriptor used by this preparation.
    #[must_use]
    pub fn descriptor(&self) -> &'static effect_contract::ResponseAnalysisDescriptor {
        self.owner.analysis_descriptor()
    }

    /// The owner's normalized immutable configuration view.
    #[must_use]
    pub fn configuration(&self) -> effect_contract::ResponseConfigurationView<'_> {
        self.owner.configuration()
    }

    /// Concrete owner payload bytes retained by this preview.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.owner.retained_bytes()
    }

    /// Query a generated axis into caller-owned buffers.
    pub fn query_into(
        &self,
        grid: ResponsePreviewGrid,
        output: ResponsePreviewOutput<'_>,
    ) -> Result<ResponseSummary, ResponsePreviewError> {
        let points = grid.points();
        if points < 2 || output.frequencies_hz.len() != points {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        let sample_rate_hz = self.configuration().sample_rate_hz;
        generate_response_grid(grid, sample_rate_hz, output.frequencies_hz)?;
        if output.total_left_db.len() != points || output.total_right_db.len() != points {
            return Err(ResponsePreviewError::OutputShape);
        }
        let sections = self.descriptor().sections.len();
        for values in [
            output.sections_left_db.as_deref(),
            output.sections_right_db.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if values.len()
                != sections
                    .checked_mul(points)
                    .ok_or(ResponsePreviewError::OutputShape)?
            {
                return Err(ResponsePreviewError::OutputShape);
            }
        }
        self.owner
            .query_into(
                ResponseQuery {
                    configuration_id: self.configuration_id,
                    frequencies_hz: output.frequencies_hz,
                    maximum_points: points,
                },
                ResponseOutput {
                    total_left_db: output.total_left_db,
                    total_right_db: output.total_right_db,
                    sections_left_db: output.sections_left_db,
                    sections_right_db: output.sections_right_db,
                },
            )
            .map_err(ResponsePreviewError::Owner)
    }
}

/// Prepare one explicit owner response without compiling a session or instantiating an effect
/// processor.
pub fn prepare_response_preview(
    request: ResponsePreviewRequest<'_>,
) -> Result<PreparedResponsePreview, ResponsePreviewError> {
    if request.sample_rate_hz == 0 || request.quantum_frames == 0 {
        return Err(ResponsePreviewError::InvalidShape);
    }
    let limits = ResponsePrepareLimits {
        maximum_prepared_bytes: request.limits.maximum_prepared_bytes,
    };
    let owner = match request.target {
        ResponsePreviewTarget::InputFilters {
            left_hpf_hz,
            left_lpf_hz,
            right_hpf_hz,
            right_lpf_hz,
        } => {
            let mut configuration = BuiltinParameters::default();
            configuration.left.hpf_hz = left_hpf_hz;
            configuration.left.lpf_hz = left_lpf_hz;
            configuration.right.hpf_hz = right_hpf_hz;
            configuration.right.lpf_hz = right_lpf_hz;
            prepare_input_filter_response(request.sample_rate_hz, configuration, limits)
                .map_err(ResponsePreviewError::Owner)?
        }
        ResponsePreviewTarget::Effect {
            effect_id,
            overrides,
            quality,
            bypass,
            link_mode,
        } => {
            let registry =
                launch_native_effect_registry().map_err(|_| ResponsePreviewError::UnknownEffect)?;
            let factory = registry
                .get_ascii(effect_id)
                .ok_or(ResponsePreviewError::UnknownEffect)?;
            let response = factory
                .response_analysis()
                .ok_or(ResponsePreviewError::UnsupportedEffect)?;
            let descriptor = factory.descriptor();
            let mut initial_values: Vec<InitialParameterValue> =
                default_initial_values(descriptor).collect();
            apply_overrides(descriptor, &mut initial_values, overrides)?;
            response
                .prepare_response(
                    effect_contract::PrepareEffectRequest {
                        sample_rate: request.sample_rate_hz,
                        quantum: request.quantum_frames,
                        quality,
                        bypass,
                        link_mode,
                        ports: PreparedPorts {
                            sidechain: effect_contract::PreparedSidechainPort::None,
                        },
                        initial_values: &initial_values,
                        limits: effect_contract::PrepareEffectLimits {
                            maximum_total_state_bytes: request.limits.maximum_total_state_bytes,
                            maximum_scratch_bytes: request.limits.maximum_scratch_bytes,
                            maximum_automation_spans_per_block: request
                                .limits
                                .maximum_automation_spans_per_block,
                        },
                    },
                    limits,
                )
                .map_err(ResponsePreviewError::Owner)?
        }
    };
    Ok(PreparedResponsePreview {
        owner,
        configuration_id: request.configuration_id,
    })
}

fn apply_overrides(
    descriptor: &'static effect_contract::EffectDescriptor,
    initial_values: &mut [InitialParameterValue],
    overrides: &[ResponseParameterOverride],
) -> Result<(), ResponsePreviewError> {
    for (index, override_value) in overrides.iter().enumerate() {
        let parameter = descriptor
            .parameters
            .iter()
            .find(|parameter| parameter.id.0 == override_value.parameter_id)
            .ok_or(ResponsePreviewError::InvalidParameter)?;
        let expected_channel = match parameter.channel_policy {
            effect_contract::ParameterChannelPolicy::Shared => {
                if override_value.channel != ParameterChannel::Both {
                    return Err(ResponsePreviewError::ConflictingChannel);
                }
                ParameterChannel::Both
            }
            effect_contract::ParameterChannelPolicy::PerLane => {
                if override_value.channel == ParameterChannel::Both {
                    return Err(ResponsePreviewError::ConflictingChannel);
                }
                override_value.channel
            }
        };
        let Some(slot) = initial_values.iter_mut().find(|value| {
            value.parameter_index == index_of_parameter(descriptor, parameter)
                && value.channel == expected_channel
        }) else {
            return Err(ResponsePreviewError::InvalidParameter);
        };
        if slot.value.to_bits() != parameter.default_value.to_bits()
            && slot.value.to_bits() != normalize_zero(parameter.default_value).to_bits()
        {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        // The exact slot is still marked below; a separate bitset is not needed because a
        // duplicate is detected by the sentinel's original default value for every descriptor.
        // Defaults can be repeated, so use the preceding rows to make duplicate detection exact.
        if overrides[..index].iter().any(|previous| {
            previous.parameter_id == override_value.parameter_id
                && previous.channel == override_value.channel
        }) {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        slot.value = override_value.value;
    }
    Ok(())
}

fn index_of_parameter(
    descriptor: &'static effect_contract::EffectDescriptor,
    parameter: &effect_contract::ParameterDescriptor,
) -> u32 {
    descriptor
        .parameters
        .iter()
        .position(|candidate| core::ptr::eq(candidate, parameter))
        .expect("parameter came from descriptor") as u32
}

fn normalize_zero(value: f32) -> f32 {
    if value == 0.0 { 0.0 } else { value }
}

/// Generate and validate one explicit frequency axis with portable f64 interpolation.
pub fn generate_response_grid(
    grid: ResponsePreviewGrid,
    sample_rate_hz: u32,
    output: &mut [f32],
) -> Result<(), ResponsePreviewError> {
    let points = grid.points();
    let (minimum_hz, maximum_hz) = grid.endpoints();
    if points < 2
        || output.len() != points
        || !minimum_hz.is_finite()
        || !maximum_hz.is_finite()
        || minimum_hz < 0.0
        || minimum_hz >= maximum_hz
        || maximum_hz > sample_rate_hz as f32 * 0.5
        || (matches!(grid, ResponsePreviewGrid::Logarithmic { .. }) && minimum_hz <= 0.0)
    {
        return Err(ResponsePreviewError::InvalidGrid);
    }
    let denominator = (points - 1) as f64;
    for index in 0..points {
        let t = index as f64 / denominator;
        let result = match grid {
            ResponsePreviewGrid::Linear { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                minimum + (maximum - minimum) * t
            }
            ResponsePreviewGrid::Logarithmic { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                exp(log(minimum) + (log(maximum) - log(minimum)) * t)
            }
        } as f32;
        if !result.is_finite() || (index > 0 && result <= output[index - 1]) {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        output[index] = result;
    }
    output[0] = minimum_hz;
    output[points - 1] = maximum_hz;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use effect_contract::{EffectQuality, LinkMode, ParameterChannel};

    #[test]
    fn explicit_eq_and_input_filter_queries_use_owner_outputs_at_launch_rates() {
        for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
            let eq = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 9_007_199_254_740_993,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::Effect {
                    effect_id: "miso.parametric-eq",
                    overrides: &[],
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("EQ response owner");
            let mut frequencies = [0.0; 4];
            let mut left = [0.0; 4];
            let mut right = [0.0; 4];
            let mut sections_left = [0.0; 16];
            let mut sections_right = [0.0; 16];
            let summary = eq
                .query_into(
                    ResponsePreviewGrid::Linear {
                        points: 4,
                        minimum_hz: 0.0,
                        maximum_hz: sample_rate_hz as f32 * 0.5,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("EQ query");
            assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert_eq!(frequencies[0], 0.0);
            assert_eq!(frequencies[3], sample_rate_hz as f32 * 0.5);

            let filters = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 7,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::InputFilters {
                    left_hpf_hz: 80.0,
                    left_lpf_hz: 18_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                    right_hpf_hz: 120.0,
                    right_lpf_hz: 16_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("input-filter response owner");
            let mut frequencies = [0.0; 3];
            let mut left = [0.0; 3];
            let mut right = [0.0; 3];
            let mut sections_left = [0.0; 6];
            let mut sections_right = [0.0; 6];
            filters
                .query_into(
                    ResponsePreviewGrid::Logarithmic {
                        points: 3,
                        minimum_hz: 20.0,
                        maximum_hz: sample_rate_hz as f32 * 0.45,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("input-filter query");
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert!(frequencies.windows(2).all(|pair| pair[1] > pair[0]));
        }
    }

    #[test]
    fn effect_overrides_reject_unknown_duplicate_and_conflicting_rows() {
        let registry = launch_native_effect_registry().expect("registry");
        let descriptor = registry
            .get_ascii("miso.parametric-eq")
            .expect("EQ")
            .descriptor();
        let id = descriptor.parameters[0].id.0;
        fn request<'a>(overrides: &'a [ResponseParameterOverride]) -> ResponsePreviewRequest<'a> {
            ResponsePreviewRequest {
                configuration_id: 1,
                sample_rate_hz: 48_000,
                quantum_frames: 128,
                target: ResponsePreviewTarget::Effect {
                    effect_id: "miso.parametric-eq",
                    overrides,
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                },
                limits: ResponsePreviewLimits::default(),
            }
        }
        assert!(matches!(
            prepare_response_preview(request(&[ResponseParameterOverride {
                parameter_id: u32::MAX,
                channel: ParameterChannel::Left,
                value: 1.0,
            }]))
            .err(),
            Some(ResponsePreviewError::InvalidParameter)
        ));
        let duplicate = [
            ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Left,
                value: 1.0,
            },
            ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Left,
                value: 0.0,
            },
        ];
        assert!(matches!(
            prepare_response_preview(request(&duplicate)).err(),
            Some(ResponsePreviewError::DuplicateParameter)
        ));
        assert!(matches!(
            prepare_response_preview(request(&[ResponseParameterOverride {
                parameter_id: id,
                channel: ParameterChannel::Both,
                value: 1.0,
            }]))
            .err(),
            Some(ResponsePreviewError::ConflictingChannel)
        ));
    }
}
