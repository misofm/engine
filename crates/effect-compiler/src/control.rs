//! Control-plane ownership for prepared effect targets.
//!
//! An owner is deliberately a small transaction shadow. It retains the factory capability so
//! that preparation can borrow it, but no part of this type is reachable from render. A failed
//! edit poisons the one candidate tranche until the caller discards it.

use effect_contract::{
    AutomationRate, EffectTargetError, EffectTargetRequest, InitialParameterValue,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, PreparedEffectTarget,
    normalize_zero, parameter_value_valid, validate_initial_values,
};
use engine::is_launch_sample_rate;
use std::sync::Arc;

use crate::{EffectBankPreparation, EffectControlProducerHandle};

/// State of one owner transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectControlOwnerPhase {
    /// No candidate is open.
    Idle,
    /// Edits may be made and targets may be prepared.
    Open,
    /// An invalid edit poisoned this candidate; only discard is allowed.
    Poisoned,
    /// Queue publication violated its prefix invariant; this owner cannot be reused.
    Faulted,
    /// All targets were published; exactly one commit is allowed.
    Published,
}

/// Why a prepared-target owner operation was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectControlOwnerError {
    /// The factory does not expose prepared-target preparation.
    Unsupported,
    /// The accepted initial rows or a target prefix has the wrong shape.
    Shape,
    /// The owner was prepared at a non-launch rate.
    Rate,
    /// A parameter index or channel does not match the descriptor.
    Parameter,
    /// A parameter cannot be changed through the live control path.
    NotAutomatable,
    /// A value is outside its descriptor domain or is non-finite.
    Domain,
    /// The transaction revision or phase does not permit this operation.
    Revision,
    /// The queue cannot hold the complete target prefix.
    Capacity,
    /// The capability rejected a candidate or target prefix.
    Target(EffectTargetError),
    /// The checked queue publication unexpectedly became full after the prefix check.
    Invariant,
}

impl From<EffectTargetError> for EffectControlOwnerError {
    fn from(value: EffectTargetError) -> Self {
        Self::Target(value)
    }
}

/// A control-side owner for one prepared effect instance.
pub struct EffectControlOwner {
    factory: Arc<dyn NativeEffectFactory>,
    sample_rate: u32,
    committed: Box<[InitialParameterValue]>,
    candidate: Box<[InitialParameterValue]>,
    dirty: Box<[bool]>,
    committed_revision: u64,
    pending_revision: Option<u64>,
    phase: EffectControlOwnerPhase,
}

impl core::fmt::Debug for EffectControlOwner {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EffectControlOwner")
            .field("sample_rate", &self.sample_rate)
            .field("value_count", &self.committed.len())
            .field("committed_revision", &self.committed_revision)
            .field("phase", &self.phase)
            .finish_non_exhaustive()
    }
}

impl EffectControlOwner {
    /// Builds an owner from the values accepted by effect preparation.
    pub fn new(
        factory: Arc<dyn NativeEffectFactory>,
        preparation: &EffectBankPreparation,
    ) -> Result<Self, EffectControlOwnerError> {
        let descriptor = factory.descriptor();
        if !is_launch_sample_rate(engine::SampleRateHz(preparation.sample_rate)) {
            return Err(EffectControlOwnerError::Rate);
        }
        validate_initial_values(descriptor, preparation.initial_values.as_ref())
            .map_err(|_| EffectControlOwnerError::Shape)?;
        let Some(capability) = factory.target_preparation() else {
            return Err(EffectControlOwnerError::Unsupported);
        };
        let dirty = vec![false; preparation.initial_values.len()].into_boxed_slice();
        capability.validate_targets(
            EffectTargetRequest {
                sample_rate: preparation.sample_rate,
                values: preparation.initial_values.as_ref(),
                changed: &dirty,
            },
            &[],
        )?;
        let committed = preparation.initial_values.clone();
        Ok(Self {
            factory,
            sample_rate: preparation.sample_rate,
            candidate: committed.clone(),
            dirty,
            committed,
            committed_revision: 0,
            pending_revision: None,
            phase: EffectControlOwnerPhase::Idle,
        })
    }

    /// The factory's immutable capability is retained for the owner lifetime.
    #[must_use]
    pub fn factory(&self) -> &Arc<dyn NativeEffectFactory> {
        &self.factory
    }

    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[must_use]
    pub const fn committed_revision(&self) -> u64 {
        self.committed_revision
    }

    #[must_use]
    pub const fn phase(&self) -> EffectControlOwnerPhase {
        self.phase
    }

    /// The last committed canonical rows.
    #[must_use]
    pub fn committed(&self) -> &[InitialParameterValue] {
        &self.committed
    }

    /// The current candidate rows. The slice is borrowed and cannot be retained by preparation.
    #[must_use]
    pub fn candidate(&self) -> &[InitialParameterValue] {
        &self.candidate
    }

    /// Touched mask for the current candidate.
    #[must_use]
    pub fn dirty(&self) -> &[bool] {
        &self.dirty
    }

    /// Starts one candidate at the caller's checked base revision.
    pub fn begin(&mut self, base_revision: u64) -> Result<(), EffectControlOwnerError> {
        if self.phase != EffectControlOwnerPhase::Idle
            || base_revision != self.committed_revision
            || self.committed_revision.checked_add(1).is_none()
        {
            return Err(EffectControlOwnerError::Revision);
        }
        self.candidate.copy_from_slice(&self.committed);
        self.dirty.fill(false);
        self.phase = EffectControlOwnerPhase::Open;
        Ok(())
    }

    /// Applies one descriptor-indexed semantic edit to the candidate.
    ///
    /// An invalid edit poisons the entire candidate. This is intentional: a later valid overwrite
    /// cannot turn an originally refused wire batch into an admitted one.
    pub fn edit(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EffectControlOwnerError> {
        if self.phase != EffectControlOwnerPhase::Open {
            return Err(EffectControlOwnerError::Revision);
        }
        let result = self.edit_inner(parameter_index, channel, value);
        if result.is_err() {
            self.phase = EffectControlOwnerPhase::Poisoned;
        }
        result
    }

    fn edit_inner(
        &mut self,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EffectControlOwnerError> {
        let parameter = self
            .factory
            .descriptor()
            .parameters
            .get(parameter_index as usize)
            .ok_or(EffectControlOwnerError::Parameter)?;
        if !parameter.automatable || parameter.automation_rate == AutomationRate::None {
            return Err(EffectControlOwnerError::NotAutomatable);
        }
        if !parameter_value_valid(parameter, value) {
            return Err(EffectControlOwnerError::Domain);
        }
        match (parameter.channel_policy, channel) {
            (ParameterChannelPolicy::Shared, ParameterChannel::Both)
            | (ParameterChannelPolicy::PerLane, ParameterChannel::Left)
            | (ParameterChannelPolicy::PerLane, ParameterChannel::Right) => {
                let value = normalize_zero(value);
                let row = self
                    .candidate
                    .iter()
                    .position(|item| {
                        item.parameter_index == parameter_index && item.channel == channel
                    })
                    .ok_or(EffectControlOwnerError::Shape)?;
                self.candidate[row].value = value;
                self.dirty[row] = true;
                Ok(())
            }
            (ParameterChannelPolicy::PerLane, ParameterChannel::Both) => {
                for lane in [ParameterChannel::Left, ParameterChannel::Right] {
                    let row = self
                        .candidate
                        .iter()
                        .position(|item| {
                            item.parameter_index == parameter_index && item.channel == lane
                        })
                        .ok_or(EffectControlOwnerError::Shape)?;
                    self.candidate[row].value = normalize_zero(value);
                    self.dirty[row] = true;
                }
                Ok(())
            }
            _ => Err(EffectControlOwnerError::Parameter),
        }
    }

    /// Borrows the complete candidate request for off-audio preparation.
    pub fn preparation_request(&self) -> Result<EffectTargetRequest<'_>, EffectControlOwnerError> {
        self.require_open()?;
        Ok(EffectTargetRequest {
            sample_rate: self.sample_rate,
            values: &self.candidate,
            changed: &self.dirty,
        })
    }

    /// Designs targets into caller-owned preallocated storage.
    pub fn prepare_targets_into(
        &self,
        out: &mut [PreparedEffectTarget],
    ) -> Result<usize, EffectControlOwnerError> {
        let request = self.preparation_request()?;
        self.factory
            .target_preparation()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .prepare_targets(request, out)
            .map_err(Into::into)
    }

    /// Validates a caller-owned target prefix against this exact candidate and touched mask.
    pub fn validate_targets(
        &self,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        let request = self.preparation_request()?;
        self.factory
            .target_preparation()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .validate_targets(request, targets)
            .map_err(Into::into)
    }

    fn require_open(&self) -> Result<(), EffectControlOwnerError> {
        if self.phase == EffectControlOwnerPhase::Open {
            Ok(())
        } else {
            Err(EffectControlOwnerError::Revision)
        }
    }

    /// Discards an open or poisoned candidate.
    pub fn discard(&mut self) -> Result<(), EffectControlOwnerError> {
        if !matches!(
            self.phase,
            EffectControlOwnerPhase::Open | EffectControlOwnerPhase::Poisoned
        ) {
            return Err(EffectControlOwnerError::Revision);
        }
        self.candidate.copy_from_slice(&self.committed);
        self.dirty.fill(false);
        self.pending_revision = None;
        self.phase = EffectControlOwnerPhase::Idle;
        Ok(())
    }

    /// Validates the candidate, revision and complete queue capacity without mutation.
    pub(crate) fn preflight_publication(
        &self,
        producer: &EffectControlProducerHandle,
        base_revision: u64,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        self.require_open()?;
        self.committed_revision
            .checked_add(1)
            .ok_or(EffectControlOwnerError::Revision)?;
        if base_revision != self.committed_revision {
            return Err(EffectControlOwnerError::Revision);
        }
        let maximum_targets = self
            .factory
            .target_preparation()
            .ok_or(EffectControlOwnerError::Unsupported)?
            .maximum_targets();
        if targets.len() > maximum_targets {
            return Err(EffectControlOwnerError::Shape);
        }
        self.validate_targets(targets)?;
        let available = producer.available_capacity();
        if targets.len() > available {
            return Err(EffectControlOwnerError::Capacity);
        }
        Ok(())
    }

    /// Publishes a completely validated target prefix through the private producer.
    pub(crate) fn publish(
        &mut self,
        producer: &mut EffectControlProducerHandle,
        base_revision: u64,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        self.preflight_publication(producer, base_revision, targets)?;
        let next_revision = self
            .committed_revision
            .checked_add(1)
            .ok_or(EffectControlOwnerError::Revision)?;
        for target in targets {
            if producer.try_push_prepared(*target).is_err() {
                self.phase = EffectControlOwnerPhase::Faulted;
                self.pending_revision = None;
                return Err(EffectControlOwnerError::Invariant);
            }
        }
        self.pending_revision = Some(next_revision);
        self.phase = EffectControlOwnerPhase::Published;
        Ok(())
    }

    /// Commits the candidate after a complete publication.
    pub fn commit(&mut self) -> Result<u64, EffectControlOwnerError> {
        if self.phase != EffectControlOwnerPhase::Published {
            return Err(EffectControlOwnerError::Revision);
        }
        let next_revision = self
            .pending_revision
            .take()
            .ok_or(EffectControlOwnerError::Invariant)?;
        self.committed.copy_from_slice(&self.candidate);
        self.dirty.fill(false);
        self.committed_revision = next_revision;
        self.phase = EffectControlOwnerPhase::Idle;
        Ok(self.committed_revision)
    }
}
