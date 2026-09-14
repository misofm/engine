//! Bounded, stateless EQ target preparation used by browser hosts.

use core::alloc::Layout;
use core::sync::atomic::AtomicUsize;
use effect_contract::{
    AutomationRate, EffectTargetError, EffectTargetRequest, InitialParameterValue,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, PreparedEffectTarget,
    default_initial_values, normalize_zero, parameter_value_valid, validate_initial_values,
};
use engine::is_launch_sample_rate;
use std::sync::Arc;

/// Number of canonical EQ parameter rows.
pub const EQ_VALUE_COUNT: usize = 60;
/// Maximum edits in one request.
pub const EQ_EDIT_CAPACITY: usize = 256;
/// Maximum prepared EQ target records.
pub const EQ_TARGET_CAPACITY: usize = 12;

/// One descriptor-checked semantic edit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EqTargetEdit {
    /// Stable descriptor parameter ID.
    pub parameter_id: u32,
    /// Channel selector.
    pub channel: ParameterChannel,
    /// Candidate value.
    pub value: f32,
}

/// Why a bounded EQ preparation request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EqTargetPreparerError {
    /// The factory has no target-preparation capability.
    Unsupported,
    /// The sample rate is outside the launch set.
    Rate,
    /// The seed or target shape is malformed.
    Shape,
    /// The parameter ID or channel is not declared.
    Parameter,
    /// The parameter is immutable through this path.
    NotAutomatable,
    /// The value is outside the declared domain.
    Domain,
    /// The native target capability rejected the request.
    Target(EffectTargetError),
    /// A semantic edit was refused at its original batch index.
    Edit {
        /// Original zero-based edit row.
        index: u32,
        /// Semantic refusal kind.
        kind: EditErrorKind,
    },
}

/// Semantic refusal kinds carried with an original edit index.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditErrorKind {
    /// Unknown parameter or invalid channel policy.
    Parameter,
    /// Immutable parameter.
    NotAutomatable,
    /// Out-of-domain or non-finite value.
    Domain,
    /// Canonical row shape was not found.
    Shape,
}

impl From<EffectTargetError> for EqTargetPreparerError {
    fn from(error: EffectTargetError) -> Self {
        Self::Target(error)
    }
}

impl EqTargetPreparerError {
    /// Returns the original edit index or `u32::MAX`.
    #[must_use]
    pub const fn index(self) -> u32 {
        match self {
            Self::Edit { index, .. } => index,
            _ => u32::MAX,
        }
    }
}

/// Stateless fixed-storage facade retaining only the opted-in factory capability.
pub struct EqTargetPreparer {
    factory: Arc<dyn NativeEffectFactory>,
}

impl EqTargetPreparer {
    /// Retain one factory after checking its canonical 60-row capability.
    pub fn new(factory: Arc<dyn NativeEffectFactory>) -> Result<Self, EqTargetPreparerError> {
        if factory.target_preparation().is_none()
            || default_initial_values(factory.descriptor()).count() != EQ_VALUE_COUNT
        {
            return Err(EqTargetPreparerError::Unsupported);
        }
        Ok(Self { factory })
    }

    /// Validate one complete seed/edit request and design its target prefix into `out`.
    pub fn prepare(
        &self,
        sample_rate: u32,
        seeds: &[f32],
        edits: &[EqTargetEdit],
        out: &mut [PreparedEffectTarget],
    ) -> Result<([f32; EQ_VALUE_COUNT], usize), EqTargetPreparerError> {
        if !is_launch_sample_rate(engine::SampleRateHz(sample_rate)) {
            return Err(EqTargetPreparerError::Rate);
        }
        if seeds.len() != EQ_VALUE_COUNT || edits.len() > EQ_EDIT_CAPACITY {
            return Err(EqTargetPreparerError::Shape);
        }
        if out.len() < EQ_TARGET_CAPACITY {
            return Err(EqTargetPreparerError::Shape);
        }
        let descriptor = self.factory.descriptor();
        let mut candidate = [InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 0.0,
        }; EQ_VALUE_COUNT];
        let mut default_count = 0;
        for value in default_initial_values(descriptor) {
            if default_count == EQ_VALUE_COUNT {
                return Err(EqTargetPreparerError::Shape);
            }
            candidate[default_count] = value;
            default_count += 1;
        }
        if default_count != EQ_VALUE_COUNT {
            return Err(EqTargetPreparerError::Shape);
        }
        for (row, value) in candidate.iter_mut().zip(seeds.iter().copied()) {
            row.value = normalize_zero(value);
        }
        validate_initial_values(descriptor, &candidate)
            .map_err(|_| EqTargetPreparerError::Shape)?;
        let mut dirty = [false; EQ_VALUE_COUNT];
        for (index, edit) in edits.iter().copied().enumerate() {
            self.apply_edit(&mut candidate, &mut dirty, edit)
                .map_err(|kind| EqTargetPreparerError::Edit {
                    index: index as u32,
                    kind,
                })?;
        }
        validate_initial_values(descriptor, &candidate)
            .map_err(|_| EqTargetPreparerError::Shape)?;
        let request = EffectTargetRequest {
            sample_rate,
            values: &candidate,
            changed: &dirty,
        };
        let mut prepared = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Both,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_TARGET_CAPACITY];
        let count = self
            .factory
            .target_preparation()
            .ok_or(EqTargetPreparerError::Unsupported)?
            .prepare_targets(request, &mut prepared)?;
        if count > EQ_TARGET_CAPACITY {
            return Err(EqTargetPreparerError::Shape);
        }
        out[..count].copy_from_slice(&prepared[..count]);
        let values = core::array::from_fn(|index| candidate[index].value);
        Ok((values, count))
    }

    fn apply_edit(
        &self,
        candidate: &mut [InitialParameterValue; EQ_VALUE_COUNT],
        dirty: &mut [bool; EQ_VALUE_COUNT],
        edit: EqTargetEdit,
    ) -> Result<(), EditErrorKind> {
        let index = self
            .factory
            .descriptor()
            .parameters
            .iter()
            .position(|parameter| parameter.id.0 == edit.parameter_id)
            .ok_or(EditErrorKind::Parameter)?;
        let parameter = &self.factory.descriptor().parameters[index];
        if !parameter.automatable || parameter.automation_rate == AutomationRate::None {
            return Err(EditErrorKind::NotAutomatable);
        }
        if !parameter_value_valid(parameter, edit.value) {
            return Err(EditErrorKind::Domain);
        }
        let value = normalize_zero(edit.value);
        match (parameter.channel_policy, edit.channel) {
            (ParameterChannelPolicy::Shared, ParameterChannel::Both)
            | (ParameterChannelPolicy::PerLane, ParameterChannel::Left)
            | (ParameterChannelPolicy::PerLane, ParameterChannel::Right) => {
                Self::set_row(candidate, dirty, index as u32, edit.channel, value)
            }
            (ParameterChannelPolicy::PerLane, ParameterChannel::Both) => {
                Self::set_row(
                    candidate,
                    dirty,
                    index as u32,
                    ParameterChannel::Left,
                    value,
                )?;
                Self::set_row(
                    candidate,
                    dirty,
                    index as u32,
                    ParameterChannel::Right,
                    value,
                )
            }
            _ => Err(EditErrorKind::Parameter),
        }
    }

    fn set_row(
        candidate: &mut [InitialParameterValue; EQ_VALUE_COUNT],
        dirty: &mut [bool; EQ_VALUE_COUNT],
        index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EditErrorKind> {
        let row = candidate
            .iter()
            .position(|item| item.parameter_index == index && item.channel == channel)
            .ok_or(EditErrorKind::Shape)?;
        candidate[row].value = value;
        dirty[row] = true;
        Ok(())
    }

    /// Returns the factory retained by this facade.
    #[must_use]
    pub fn factory(&self) -> &Arc<dyn NativeEffectFactory> {
        &self.factory
    }

    /// Actual retained allocation size of the shared factory `Arc`.
    #[must_use]
    pub fn factory_allocation_bytes(&self) -> usize {
        let header = Layout::new::<AtomicUsize>();
        let Ok((strong_weak, _)) = header.extend(header) else {
            return 0;
        };
        strong_weak
            .extend(Layout::for_value(self.factory.as_ref()))
            .map_or(0, |(layout, _)| layout.pad_to_align().size())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use effect_contract::{
        EffectDescriptor, EffectPrepareError, NativeEffectTargetPreparation,
        PrepareEffectBankRequest, PrepareEffectRequest, PreparedNativeEffect,
        PreparedNativeEffectBank,
    };
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};

    struct OptInEq;
    impl NativeEffectFactory for OptInEq {
        fn descriptor(&self) -> &'static EffectDescriptor {
            &PARAMETRIC_EQ_DESCRIPTOR
        }
        fn prepare(
            &self,
            request: PrepareEffectRequest<'_>,
        ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
            ParametricEqFactory.prepare(request)
        }
        fn target_preparation(&self) -> Option<&dyn NativeEffectTargetPreparation> {
            Some(&ParametricEqFactory)
        }
        fn bind_homogeneous_bank(
            &self,
            request: PrepareEffectBankRequest<'_>,
        ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
            ParametricEqFactory.bind_homogeneous_bank(request)
        }
    }

    #[test]
    fn real_opted_in_eq_seeds_and_both_edit_are_bounded() {
        let descriptor = &PARAMETRIC_EQ_DESCRIPTOR;
        let mut seeds: [f32; EQ_VALUE_COUNT] = core::array::from_fn(|index| {
            default_initial_values(descriptor).nth(index).unwrap().value
        });
        seeds[4] = 120.0;
        seeds[5] = 2_400.0;
        seeds[6] = 6.0;
        seeds[7] = -9.0;
        let preparer = EqTargetPreparer::new(Arc::new(OptInEq)).unwrap();
        let parameter = &descriptor.parameters[2];
        let mut targets = [PreparedEffectTarget {
            slot: 0,
            channel: ParameterChannel::Both,
            words: [0; effect_contract::PREPARED_EFFECT_TARGET_WORDS],
        }; EQ_TARGET_CAPACITY];
        let (values, count) = preparer
            .prepare(
                48_000,
                &seeds,
                &[EqTargetEdit {
                    parameter_id: parameter.id.0,
                    channel: ParameterChannel::Both,
                    value: 100.0,
                }],
                &mut targets,
            )
            .unwrap();
        let mut expected = seeds;
        expected[4] = 100.0;
        expected[5] = 100.0;
        assert_eq!(values, expected);
        assert_eq!(count, 2);
        assert_eq!(targets[0].channel, ParameterChannel::Left);
        assert_eq!(targets[1].channel, ParameterChannel::Right);
        let before = targets;
        let error = preparer
            .prepare(
                48_000,
                &seeds,
                &[EqTargetEdit {
                    parameter_id: 0,
                    channel: ParameterChannel::Both,
                    value: 0.0,
                }],
                &mut targets,
            )
            .unwrap_err();
        assert_eq!(error.index(), 0);
        assert_eq!(targets, before);
    }
}
