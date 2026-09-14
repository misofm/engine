//! Bounded, stateless EQ and builtin input-filter target preparation used by browser hosts.

use builtins::{
    BuiltinLaneSelector, PreparedInputFilterTarget, prepare_input_filter_pair,
    validate_input_filter_pair,
};
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

/// Number of semantic builtin input-filter values in lane order: LH, LL, RH, RL.
pub const INPUT_FILTER_VALUE_COUNT: usize = 4;
/// Maximum input-filter edits in one bounded request.
pub const INPUT_FILTER_EDIT_CAPACITY: usize = 256;
/// Maximum coalesced input-filter section targets.
pub const INPUT_FILTER_TARGET_CAPACITY: usize = 4;

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

/// One semantic input-filter edit before target design.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputFilterEdit {
    /// Stable builtin parameter ID: 0 for an atomic pair, 3 for HPF, 4 for LPF.
    pub parameter_id: u32,
    /// Addressed lane(s).
    pub channel: ParameterChannel,
    /// HPF or pair value.
    pub value0: f32,
    /// LPF value for an atomic pair; must be +0 for one-sided edits.
    pub value1: f32,
}

/// Refusal kinds for one original input-filter edit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFilterEditErrorKind {
    /// Unsupported parameter ID.
    Parameter,
    /// Invalid cutoff, ordering, rate or non-finite value.
    Domain,
    /// Candidate row shape or one-sided payload was not representable.
    Shape,
}

/// Why bounded input-filter preparation was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputFilterPreparerError {
    /// Sample rate is outside the launch set.
    Rate,
    /// Request, seed or output shape is outside the fixed bound.
    Shape,
    /// A seed pair is outside the filter domain.
    Domain,
    /// A semantic edit failed at its original wire index.
    Edit {
        /// Original zero-based edit index.
        index: u32,
        /// Refusal kind.
        kind: InputFilterEditErrorKind,
    },
    /// Final target design or safety validation failed.
    Design,
}

impl InputFilterPreparerError {
    /// Returns the original edit index or `u32::MAX`.
    #[must_use]
    pub const fn index(self) -> u32 {
        match self {
            Self::Edit { index, .. } => index,
            _ => u32::MAX,
        }
    }
}

/// Apply one semantic edit to a candidate without designing coefficients.
///
/// The operation validates every addressed lane before mutating either the candidate or its
/// dirty-section mask. This is the host-admission authority; final target design is separate.
pub fn apply_input_filter_edit(
    sample_rate_hz: u32,
    candidate: &mut [f32; INPUT_FILTER_VALUE_COUNT],
    dirty: &mut [bool; INPUT_FILTER_VALUE_COUNT],
    edit: InputFilterEdit,
) -> Result<(), InputFilterEditErrorKind> {
    if !matches!(edit.parameter_id, 0 | 3 | 4) {
        return Err(InputFilterEditErrorKind::Parameter);
    }
    if (edit.parameter_id == 3 || edit.parameter_id == 4)
        && edit.value1.to_bits() != 0.0_f32.to_bits()
    {
        return Err(InputFilterEditErrorKind::Shape);
    }
    let mut next = *candidate;
    let mut next_dirty = *dirty;
    let lanes = match edit.channel {
        ParameterChannel::Left => [Some(0_usize), None],
        ParameterChannel::Right => [Some(2_usize), None],
        ParameterChannel::Both => [Some(0_usize), Some(2_usize)],
    };
    for lane_start in lanes.into_iter().flatten() {
        let (hpf, lpf) = match edit.parameter_id {
            0 => (edit.value0, edit.value1),
            3 => (edit.value0, candidate[lane_start + 1]),
            4 => (candidate[lane_start], edit.value0),
            _ => return Err(InputFilterEditErrorKind::Parameter),
        };
        validate_input_filter_pair(sample_rate_hz, hpf, lpf)
            .map_err(|_| InputFilterEditErrorKind::Domain)?;
        next[lane_start] = if hpf == 0.0 { 0.0 } else { hpf };
        next[lane_start + 1] = if lpf == 0.0 { 0.0 } else { lpf };
        next_dirty[lane_start] |= edit.parameter_id == 0 || edit.parameter_id == 3;
        next_dirty[lane_start + 1] |= edit.parameter_id == 0 || edit.parameter_id == 4;
    }
    *candidate = next;
    *dirty = next_dirty;
    Ok(())
}

/// Stateless preparation authority for live builtin input-filter targets.
#[derive(Clone, Copy, Debug, Default)]
pub struct InputFilterPreparer;

impl InputFilterPreparer {
    /// Validate every original edit in wire order, then design only final touched sections.
    pub fn prepare(
        &self,
        sample_rate_hz: u32,
        seeds: &[f32],
        edits: &[InputFilterEdit],
        out: &mut [PreparedInputFilterTarget],
    ) -> Result<([f32; INPUT_FILTER_VALUE_COUNT], usize), InputFilterPreparerError> {
        if !is_launch_sample_rate(engine::SampleRateHz(sample_rate_hz)) {
            return Err(InputFilterPreparerError::Rate);
        }
        if seeds.len() != INPUT_FILTER_VALUE_COUNT
            || edits.len() > INPUT_FILTER_EDIT_CAPACITY
            || out.len() < INPUT_FILTER_TARGET_CAPACITY
        {
            return Err(InputFilterPreparerError::Shape);
        }
        let mut candidate = [0.0_f32; INPUT_FILTER_VALUE_COUNT];
        candidate.copy_from_slice(seeds);
        for lane in [0_usize, 2_usize] {
            validate_input_filter_pair(sample_rate_hz, candidate[lane], candidate[lane + 1])
                .map_err(|_| InputFilterPreparerError::Domain)?;
            candidate[lane] = if candidate[lane] == 0.0 {
                0.0
            } else {
                candidate[lane]
            };
            candidate[lane + 1] = if candidate[lane + 1] == 0.0 {
                0.0
            } else {
                candidate[lane + 1]
            };
        }
        let mut dirty = [false; INPUT_FILTER_VALUE_COUNT];
        for (index, edit) in edits.iter().copied().enumerate() {
            apply_input_filter_edit(sample_rate_hz, &mut candidate, &mut dirty, edit).map_err(
                |kind| InputFilterPreparerError::Edit {
                    index: index as u32,
                    kind,
                },
            )?;
        }

        let mut designed = [[None; 2]; 2];
        for lane in [0_usize, 2_usize] {
            if dirty[lane] || dirty[lane + 1] {
                let pair =
                    prepare_input_filter_pair(sample_rate_hz, candidate[lane], candidate[lane + 1])
                        .map_err(|_| InputFilterPreparerError::Design)?;
                designed[lane / 2] = [Some(pair.targets[0]), Some(pair.targets[1])];
            }
        }
        let mut count = 0;
        for section in 0..2 {
            let left_dirty = dirty[section];
            let right_dirty = dirty[2 + section];
            let left = designed[0][section];
            let right = designed[1][section];
            if left_dirty && right_dirty {
                let (Some(left), Some(right)) = (left, right) else {
                    return Err(InputFilterPreparerError::Design);
                };
                if target_words_equal(left, right) {
                    out[count] = PreparedInputFilterTarget {
                        lanes: BuiltinLaneSelector::Both,
                        ..left
                    };
                    count += 1;
                    continue;
                }
            }
            if left_dirty {
                out[count] = PreparedInputFilterTarget {
                    lanes: BuiltinLaneSelector::Left,
                    ..left.ok_or(InputFilterPreparerError::Design)?
                };
                count += 1;
            }
            if right_dirty {
                out[count] = PreparedInputFilterTarget {
                    lanes: BuiltinLaneSelector::Right,
                    ..right.ok_or(InputFilterPreparerError::Design)?
                };
                count += 1;
            }
        }
        Ok((candidate, count))
    }
}

fn target_words_equal(left: PreparedInputFilterTarget, right: PreparedInputFilterTarget) -> bool {
    left.pair
        .iter()
        .zip(right.pair.iter())
        .all(|(left, right)| left.to_bits() == right.to_bits())
        && left
            .coefficients
            .iter()
            .zip(right.coefficients.iter())
            .all(|(left, right)| left.to_bits() == right.to_bits())
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

    #[test]
    fn input_filter_pair_edit_coalesces_only_equal_whole_sections() {
        let preparer = InputFilterPreparer;
        let seeds = [100.0, 1_000.0, 100.0, 1_000.0];
        let mut targets = [PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Left,
            section: 9,
            pair: [9.0; 2],
            coefficients: [9.0; 6],
        }; INPUT_FILTER_TARGET_CAPACITY];
        let (values, count) = preparer
            .prepare(
                48_000,
                &seeds,
                &[InputFilterEdit {
                    parameter_id: 0,
                    channel: ParameterChannel::Both,
                    value0: 200.0,
                    value1: 2_000.0,
                }],
                &mut targets,
            )
            .unwrap();
        assert_eq!(values, [200.0, 2_000.0, 200.0, 2_000.0]);
        assert_eq!(count, 2);
        assert_eq!(targets[0].lanes, BuiltinLaneSelector::Both);
        assert_eq!(targets[1].lanes, BuiltinLaneSelector::Both);

        let seeds = [100.0, 1_000.0, 200.0, 2_000.0];
        let (values, count) = preparer
            .prepare(
                48_000,
                &seeds,
                &[InputFilterEdit {
                    parameter_id: 3,
                    channel: ParameterChannel::Both,
                    value0: 300.0,
                    value1: 0.0,
                }],
                &mut targets,
            )
            .unwrap();
        assert_eq!(values, [300.0, 1_000.0, 300.0, 2_000.0]);
        assert_eq!(count, 2);
        assert_eq!(targets[0].lanes, BuiltinLaneSelector::Left);
        assert_eq!(targets[1].lanes, BuiltinLaneSelector::Right);
        assert_eq!(targets[0].section, 0);
        assert_eq!(targets[1].section, 0);

        let (values, count) = preparer
            .prepare(
                48_000,
                &seeds,
                &[InputFilterEdit {
                    parameter_id: 4,
                    channel: ParameterChannel::Right,
                    value0: 3_000.0,
                    value1: 0.0,
                }],
                &mut targets,
            )
            .unwrap();
        assert_eq!(values, [100.0, 1_000.0, 200.0, 3_000.0]);
        assert_eq!(count, 1);
        assert_eq!(targets[0].lanes, BuiltinLaneSelector::Right);
        assert_eq!(targets[0].section, 1);
    }

    #[test]
    fn input_filter_refuses_crossing_before_a_later_overwrite() {
        let preparer = InputFilterPreparer;
        let mut targets = [PreparedInputFilterTarget {
            lanes: BuiltinLaneSelector::Left,
            section: 9,
            pair: [9.0; 2],
            coefficients: [9.0; 6],
        }; INPUT_FILTER_TARGET_CAPACITY];
        let error = preparer
            .prepare(
                48_000,
                &[100.0, 1_000.0, 100.0, 1_000.0],
                &[
                    InputFilterEdit {
                        parameter_id: 3,
                        channel: ParameterChannel::Left,
                        value0: 2_000.0,
                        value1: 0.0,
                    },
                    InputFilterEdit {
                        parameter_id: 4,
                        channel: ParameterChannel::Left,
                        value0: 5_000.0,
                        value1: 0.0,
                    },
                ],
                &mut targets,
            )
            .unwrap_err();
        assert_eq!(
            error,
            InputFilterPreparerError::Edit {
                index: 0,
                kind: InputFilterEditErrorKind::Domain,
            }
        );
        assert_eq!(targets[0].section, 9);
    }

    #[test]
    fn input_filter_rejects_negative_zero_in_unused_one_sided_word() {
        let mut candidate = [100.0, 1_000.0, 100.0, 1_000.0];
        let mut dirty = [false; INPUT_FILTER_VALUE_COUNT];
        assert_eq!(
            apply_input_filter_edit(
                48_000,
                &mut candidate,
                &mut dirty,
                InputFilterEdit {
                    parameter_id: 3,
                    channel: ParameterChannel::Left,
                    value0: 200.0,
                    value1: -0.0,
                },
            ),
            Err(InputFilterEditErrorKind::Shape)
        );
        assert_eq!(candidate, [100.0, 1_000.0, 100.0, 1_000.0]);
        assert_eq!(dirty, [false; INPUT_FILTER_VALUE_COUNT]);
    }
}
