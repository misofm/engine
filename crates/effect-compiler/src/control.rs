//! Control-plane ownership for prepared effect targets.
//!
//! An owner is deliberately a small transaction shadow. It retains the factory capability so
//! that preparation can borrow it, but no part of this type is reachable from render. A failed
//! edit poisons the one candidate tranche until the caller discards it.

use core::alloc::Layout;
use core::sync::atomic::AtomicUsize;
use effect_contract::{
    AutomationRate, EffectTargetError, EffectTargetRequest, InitialParameterValue,
    NativeEffectFactory, ParameterChannel, ParameterChannelPolicy, PreparedEffectTarget,
    normalize_zero, parameter_value_valid, validate_initial_values,
};
use effect_contract::{EffectCellWriter, EffectControlRecord};
use engine::is_launch_sample_rate;
use engine::realtime::LiveStamp;
use std::sync::Arc;

use crate::EffectBankPreparation;

/// State of one owner transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectControlOwnerPhase {
    /// No candidate is open.
    Idle,
    /// Edits may be made and targets may be prepared.
    Open,
    /// An invalid edit poisoned this candidate; only discard is allowed.
    Poisoned,
    /// Publication violated its invariant; this owner cannot be reused.
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
    /// The capability rejected a candidate or target prefix.
    Target(EffectTargetError),
    /// A target the preflight accepted had no cell to take it.
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
    /// The open transaction's rows and touched mask. Built by `begin`, dropped by `commit`,
    /// `discard` and a publication fault, so an idle owner retains only `committed` (#1624).
    candidate: Option<OpenCandidate>,
    committed_revision: u64,
    pending_revision: Option<u64>,
    phase: EffectControlOwnerPhase,
}

/// The rows an open transaction edits and the mask of those it touched; both are exactly as long
/// as the owner's committed rows.
struct OpenCandidate {
    rows: Box<[InitialParameterValue]>,
    dirty: Box<[bool]>,
}

/// Engine-owned allocations retained by one prepared-target owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct EffectControlOwnerResources {
    pub(crate) owned_payload_bytes: u64,
    pub(crate) largest_owned_allocation_bytes: u64,
    pub(crate) factory_allocation_bytes: u64,
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
        // Transient: the owner keeps no mask between transactions.
        let untouched = vec![false; preparation.initial_values.len()];
        capability.validate_targets(
            EffectTargetRequest {
                sample_rate: preparation.sample_rate,
                values: preparation.initial_values.as_ref(),
                changed: &untouched,
            },
            &[],
        )?;
        let committed = preparation.initial_values.clone();
        Ok(Self {
            factory,
            sample_rate: preparation.sample_rate,
            candidate: None,
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

    /// The current candidate rows, empty while no transaction is open. The slice is borrowed and
    /// cannot be retained by preparation.
    #[must_use]
    pub fn candidate(&self) -> &[InitialParameterValue] {
        self.candidate.as_ref().map_or(&[], |open| &open.rows)
    }

    /// Touched mask for the current candidate, empty while no transaction is open.
    #[must_use]
    pub fn dirty(&self) -> &[bool] {
        self.candidate.as_ref().map_or(&[], |open| &open.dirty)
    }

    /// Computes the actual dynamic allocations retained by this owner. The factory allocation is
    /// returned separately so a producer-table walk can count shared `Arc` identities once.
    pub(crate) fn resource_facts(
        &self,
    ) -> Result<EffectControlOwnerResources, EffectControlResourceError> {
        let owner_box = u64::try_from(core::mem::size_of::<Self>())
            .map_err(|_| EffectControlResourceError::Arithmetic)?;
        let committed = checked_slice_bytes::<InitialParameterValue>(self.committed.len())?;
        // An open transaction's candidate is transient (built by `begin`, dropped by `commit` or
        // `discard`), so it is no retained allocation.
        let owned_payload_bytes = owner_box
            .checked_add(committed)
            .ok_or(EffectControlResourceError::Arithmetic)?;
        let largest_owned_allocation_bytes = owner_box.max(committed);

        let header = Layout::new::<AtomicUsize>();
        let (strong_weak, _) = header
            .extend(header)
            .map_err(|_| EffectControlResourceError::Arithmetic)?;
        let (arc_layout, _) = strong_weak
            .extend(Layout::for_value(self.factory.as_ref()))
            .map_err(|_| EffectControlResourceError::Arithmetic)?;
        let factory_allocation_bytes = u64::try_from(arc_layout.pad_to_align().size())
            .map_err(|_| EffectControlResourceError::Arithmetic)?;
        Ok(EffectControlOwnerResources {
            owned_payload_bytes,
            largest_owned_allocation_bytes,
            factory_allocation_bytes,
        })
    }

    /// Starts one candidate at the caller's checked base revision.
    pub fn begin(&mut self, base_revision: u64) -> Result<(), EffectControlOwnerError> {
        if self.phase != EffectControlOwnerPhase::Idle
            || base_revision != self.committed_revision
            || self.committed_revision.checked_add(1).is_none()
        {
            return Err(EffectControlOwnerError::Revision);
        }
        self.candidate = Some(OpenCandidate {
            rows: self.committed.clone(),
            dirty: vec![false; self.committed.len()].into_boxed_slice(),
        });
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
        let candidate = self
            .candidate
            .as_mut()
            .ok_or(EffectControlOwnerError::Revision)?;
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
                let row = candidate
                    .rows
                    .iter()
                    .position(|item| {
                        item.parameter_index == parameter_index && item.channel == channel
                    })
                    .ok_or(EffectControlOwnerError::Shape)?;
                candidate.rows[row].value = value;
                candidate.dirty[row] = true;
                Ok(())
            }
            (ParameterChannelPolicy::PerLane, ParameterChannel::Both) => {
                for lane in [ParameterChannel::Left, ParameterChannel::Right] {
                    let row = candidate
                        .rows
                        .iter()
                        .position(|item| {
                            item.parameter_index == parameter_index && item.channel == lane
                        })
                        .ok_or(EffectControlOwnerError::Shape)?;
                    candidate.rows[row].value = normalize_zero(value);
                    candidate.dirty[row] = true;
                }
                Ok(())
            }
            _ => Err(EffectControlOwnerError::Parameter),
        }
    }

    /// Borrows the complete candidate request for off-audio preparation.
    pub fn preparation_request(&self) -> Result<EffectTargetRequest<'_>, EffectControlOwnerError> {
        self.require_open()?;
        let open = self
            .candidate
            .as_ref()
            .ok_or(EffectControlOwnerError::Invariant)?;
        Ok(EffectTargetRequest {
            sample_rate: self.sample_rate,
            values: &open.rows,
            changed: &open.dirty,
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
        self.candidate = None;
        self.pending_revision = None;
        self.phase = EffectControlOwnerPhase::Idle;
        Ok(())
    }

    /// Validates the candidate, the revision and the target set without mutation. The targets
    /// are cells: they want no room, so there is no capacity to check (decision 15, D15-2).
    pub(crate) fn preflight_publication(
        &self,
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
        Ok(())
    }

    /// Writes a completely validated target set into the writer's target cells under `stamp`.
    pub(crate) fn publish(
        &mut self,
        writer: &mut EffectCellWriter,
        stamp: LiveStamp<'_>,
        base_revision: u64,
        targets: &[PreparedEffectTarget],
    ) -> Result<(), EffectControlOwnerError> {
        self.preflight_publication(base_revision, targets)?;
        let next_revision = self
            .committed_revision
            .checked_add(1)
            .ok_or(EffectControlOwnerError::Revision)?;
        for target in targets {
            if writer
                .write(stamp, EffectControlRecord::PreparedTarget(*target))
                .is_err()
            {
                self.phase = EffectControlOwnerPhase::Faulted;
                self.candidate = None;
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
        let open = self
            .candidate
            .take()
            .ok_or(EffectControlOwnerError::Invariant)?;
        self.committed.copy_from_slice(&open.rows);
        self.committed_revision = next_revision;
        self.phase = EffectControlOwnerPhase::Idle;
        Ok(self.committed_revision)
    }
}

/// Arithmetic failure while projecting retained effect-control storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectControlResourceError {
    Arithmetic,
}

fn checked_slice_bytes<T>(length: usize) -> Result<u64, EffectControlResourceError> {
    let bytes = length
        .checked_mul(core::mem::size_of::<T>())
        .ok_or(EffectControlResourceError::Arithmetic)?;
    u64::try_from(bytes).map_err(|_| EffectControlResourceError::Arithmetic)
}

#[cfg(test)]
mod tests {
    use super::*;
    use effect_contract::default_initial_values;
    use parametric_eq::{PARAMETRIC_EQ_DESCRIPTOR, ParametricEqFactory};
    use std::sync::Arc;

    #[test]
    fn revision_overflow_is_refused_before_candidate_mutation() {
        let committed: Box<[InitialParameterValue]> =
            default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        let mut owner = EffectControlOwner {
            factory: Arc::new(ParametricEqFactory),
            sample_rate: 48_000,
            candidate: None,
            committed,
            committed_revision: u64::MAX,
            pending_revision: None,
            phase: EffectControlOwnerPhase::Idle,
        };
        let committed_before = owner.committed.clone();
        assert_eq!(
            owner.begin(u64::MAX),
            Err(EffectControlOwnerError::Revision)
        );
        assert_eq!(owner.phase, EffectControlOwnerPhase::Idle);
        assert_eq!(owner.committed, committed_before);
        assert!(owner.candidate.is_none(), "a refused begin builds nothing");
    }

    /// An idle owner retains only its committed rows: the candidate and the touched mask exist
    /// from `begin` to `commit` or `discard` and nowhere else (#1624).
    #[test]
    fn the_candidate_lives_only_between_begin_and_commit_or_discard() {
        let committed: Box<[InitialParameterValue]> =
            default_initial_values(&PARAMETRIC_EQ_DESCRIPTOR).collect();
        let rows = committed.len();
        let mut owner = EffectControlOwner {
            factory: Arc::new(ParametricEqFactory),
            sample_rate: 48_000,
            candidate: None,
            committed,
            committed_revision: 0,
            pending_revision: None,
            phase: EffectControlOwnerPhase::Idle,
        };
        let idle_bytes = owner.resource_facts().expect("facts").owned_payload_bytes;
        assert!(owner.candidate().is_empty() && owner.dirty().is_empty());

        owner.begin(0).expect("begin");
        assert_eq!(owner.candidate(), &owner.committed[..]);
        assert_eq!(owner.dirty().len(), rows);
        assert!(owner.dirty().iter().all(|touched| !touched));
        owner.edit(2, ParameterChannel::Both, 100.0).expect("edit");
        assert!(owner.dirty().iter().any(|touched| *touched));
        owner.discard().expect("discard");
        assert!(owner.candidate().is_empty() && owner.dirty().is_empty());

        // A second candidate starts from the committed rows, not from the discarded edit.
        owner.begin(0).expect("begin again");
        assert_eq!(owner.candidate(), &owner.committed[..]);
        assert!(owner.dirty().iter().all(|touched| !touched));
        // A poisoned candidate is still the open one until discarded.
        assert!(owner.edit(9_999, ParameterChannel::Both, 0.0).is_err());
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Poisoned);
        assert_eq!(owner.dirty().len(), rows);
        owner.discard().expect("discard poisoned");
        assert!(owner.candidate().is_empty());
        assert_eq!(
            owner.resource_facts().expect("facts").owned_payload_bytes,
            idle_bytes,
            "the retained facts do not depend on a candidate"
        );
    }
}
