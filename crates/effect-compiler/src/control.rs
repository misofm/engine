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
    /// The candidate's rows or touched mask could not be allocated; nothing changed.
    Allocation,
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
    committed_revision: u64,
    /// The transaction phase, which carries the open transaction's candidate. A candidate exists
    /// exactly in the phases that may use it, so the phase and the candidate cannot disagree; it
    /// is built by `begin` and dropped by `commit`, `discard` and a publication fault, so an idle
    /// owner retains only `committed` (#1624).
    state: OwnerState,
}

/// The owner's phase, with the candidate in the phases that have one.
enum OwnerState {
    Idle,
    Open(OpenCandidate),
    Poisoned(OpenCandidate),
    Faulted,
    /// Every target was published; `next_revision` is the revision the one commit takes.
    Published {
        candidate: OpenCandidate,
        next_revision: u64,
    },
}

impl OwnerState {
    const fn phase(&self) -> EffectControlOwnerPhase {
        match self {
            Self::Idle => EffectControlOwnerPhase::Idle,
            Self::Open(_) => EffectControlOwnerPhase::Open,
            Self::Poisoned(_) => EffectControlOwnerPhase::Poisoned,
            Self::Faulted => EffectControlOwnerPhase::Faulted,
            Self::Published { .. } => EffectControlOwnerPhase::Published,
        }
    }

    const fn candidate(&self) -> Option<&OpenCandidate> {
        match self {
            Self::Open(candidate)
            | Self::Poisoned(candidate)
            | Self::Published { candidate, .. } => Some(candidate),
            Self::Idle | Self::Faulted => None,
        }
    }
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
            .field("phase", &self.state.phase())
            .finish_non_exhaustive()
    }
}

impl EffectControlOwner {
    /// Builds an owner from the values accepted by effect preparation.
    pub fn new(
        factory: Arc<dyn NativeEffectFactory>,
        preparation: &EffectBankPreparation,
    ) -> Result<Self, EffectControlOwnerError> {
        Self::validate(factory.as_ref(), preparation)?;
        let committed = preparation.initial_values.clone();
        Ok(Self {
            factory,
            sample_rate: preparation.sample_rate,
            committed,
            committed_revision: 0,
            state: OwnerState::Idle,
        })
    }

    /// [`Self::new`]'s refusals, without building the owner (issue #2199).
    fn validate(
        factory: &dyn NativeEffectFactory,
        preparation: &EffectBankPreparation,
    ) -> Result<(), EffectControlOwnerError> {
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
        Ok(())
    }

    /// What [`Self::new`] would build for `factory` and `preparation` retains, before it is built
    /// (issue #2199): [`Self::new`]'s refusals, and else the owner's
    /// [`resource_facts`](Self::resource_facts) as it would report them. An idle owner retains only
    /// its box and its committed values.
    pub(crate) fn projected_resources(
        factory: &Arc<dyn NativeEffectFactory>,
        preparation: &EffectBankPreparation,
    ) -> Result<
        Result<EffectControlOwnerResources, EffectControlResourceError>,
        EffectControlOwnerError,
    > {
        Self::validate(factory.as_ref(), preparation)?;
        Ok(owner_resources(factory, preparation.initial_values.len()))
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
        self.state.phase()
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
        self.state.candidate().map_or(&[], |open| &open.rows)
    }

    /// Touched mask for the current candidate, empty while no transaction is open.
    #[must_use]
    pub fn dirty(&self) -> &[bool] {
        self.state.candidate().map_or(&[], |open| &open.dirty)
    }

    /// Computes the actual dynamic allocations retained by this owner. The factory allocation is
    /// returned separately so a producer-table walk can count shared `Arc` identities once.
    pub(crate) fn resource_facts(
        &self,
    ) -> Result<EffectControlOwnerResources, EffectControlResourceError> {
        owner_resources(&self.factory, self.committed.len())
    }

    /// Starts one candidate at the caller's checked base revision.
    ///
    /// The candidate's rows and touched mask are allocated fallibly: a failed allocation refuses
    /// with [`EffectControlOwnerError::Allocation`] and leaves the owner idle and unchanged.
    pub fn begin(&mut self, base_revision: u64) -> Result<(), EffectControlOwnerError> {
        if !matches!(self.state, OwnerState::Idle)
            || base_revision != self.committed_revision
            || self.committed_revision.checked_add(1).is_none()
        {
            return Err(EffectControlOwnerError::Revision);
        }
        let length = self.committed.len();
        let mut rows = Vec::new();
        let mut dirty = Vec::new();
        if rows.try_reserve_exact(length).is_err() || dirty.try_reserve_exact(length).is_err() {
            return Err(EffectControlOwnerError::Allocation);
        }
        // Within the reserved capacity: neither extension reallocates, and each boxes in place.
        rows.extend_from_slice(&self.committed);
        dirty.resize(length, false);
        self.state = OwnerState::Open(OpenCandidate {
            rows: rows.into_boxed_slice(),
            dirty: dirty.into_boxed_slice(),
        });
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
        let OwnerState::Open(candidate) = &mut self.state else {
            return Err(EffectControlOwnerError::Revision);
        };
        let result = Self::edit_inner(
            self.factory.as_ref(),
            candidate,
            parameter_index,
            channel,
            value,
        );
        if result.is_err() {
            // The candidate moves into the poisoned phase unchanged; only discard may follow.
            if let OwnerState::Open(candidate) =
                core::mem::replace(&mut self.state, OwnerState::Faulted)
            {
                self.state = OwnerState::Poisoned(candidate);
            }
        }
        result
    }

    fn edit_inner(
        factory: &dyn NativeEffectFactory,
        candidate: &mut OpenCandidate,
        parameter_index: u32,
        channel: ParameterChannel,
        value: f32,
    ) -> Result<(), EffectControlOwnerError> {
        let parameter = factory
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
        let OwnerState::Open(open) = &self.state else {
            return Err(EffectControlOwnerError::Revision);
        };
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
        if matches!(self.state, OwnerState::Open(_)) {
            Ok(())
        } else {
            Err(EffectControlOwnerError::Revision)
        }
    }

    /// Discards an open or poisoned candidate.
    pub fn discard(&mut self) -> Result<(), EffectControlOwnerError> {
        if !matches!(self.state, OwnerState::Open(_) | OwnerState::Poisoned(_)) {
            return Err(EffectControlOwnerError::Revision);
        }
        self.state = OwnerState::Idle;
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
                self.state = OwnerState::Faulted;
                return Err(EffectControlOwnerError::Invariant);
            }
        }
        // The preflight held the open phase, so the candidate moves into the published phase.
        match core::mem::replace(&mut self.state, OwnerState::Faulted) {
            OwnerState::Open(candidate) => {
                self.state = OwnerState::Published {
                    candidate,
                    next_revision,
                };
                Ok(())
            }
            _ => Err(EffectControlOwnerError::Invariant),
        }
    }

    /// Commits the candidate after a complete publication.
    pub fn commit(&mut self) -> Result<u64, EffectControlOwnerError> {
        let OwnerState::Published {
            candidate,
            next_revision,
        } = &self.state
        else {
            return Err(EffectControlOwnerError::Revision);
        };
        self.committed.copy_from_slice(&candidate.rows);
        self.committed_revision = *next_revision;
        self.state = OwnerState::Idle;
        Ok(self.committed_revision)
    }
}

/// Arithmetic failure while projecting retained effect-control storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectControlResourceError {
    Arithmetic,
}

/// [`EffectControlOwner::resource_facts`] of an idle owner over `factory` holding
/// `committed_values` committed values: its box, its committed values and the factory's `Arc`
/// allocation.
fn owner_resources(
    factory: &Arc<dyn NativeEffectFactory>,
    committed_values: usize,
) -> Result<EffectControlOwnerResources, EffectControlResourceError> {
    let owner_box = u64::try_from(core::mem::size_of::<EffectControlOwner>())
        .map_err(|_| EffectControlResourceError::Arithmetic)?;
    let committed = checked_slice_bytes::<InitialParameterValue>(committed_values)?;

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
        .extend(Layout::for_value(factory.as_ref()))
        .map_err(|_| EffectControlResourceError::Arithmetic)?;
    let factory_allocation_bytes = u64::try_from(arc_layout.pad_to_align().size())
        .map_err(|_| EffectControlResourceError::Arithmetic)?;
    Ok(EffectControlOwnerResources {
        owned_payload_bytes,
        largest_owned_allocation_bytes,
        factory_allocation_bytes,
    })
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
            committed,
            committed_revision: u64::MAX,
            state: OwnerState::Idle,
        };
        let committed_before = owner.committed.clone();
        assert_eq!(
            owner.begin(u64::MAX),
            Err(EffectControlOwnerError::Revision)
        );
        assert_eq!(owner.phase(), EffectControlOwnerPhase::Idle);
        assert_eq!(owner.committed, committed_before);
        assert!(
            owner.candidate().is_empty(),
            "a refused begin builds nothing"
        );
    }
}
