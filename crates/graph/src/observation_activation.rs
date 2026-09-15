//! Control-side admission and block-boundary transport for prepared graph observers.
//!
//! The executor integration is deliberately left to the next tranche. This module owns only the
//! fixed snapshot storage, bounded move-only queues, and the control-side admission ledger that
//! make that integration safe to add without changing an audio plan.

use core::{
    alloc::{Layout, LayoutError},
    cmp::Ordering,
    num::NonZeroUsize,
    sync::atomic::{AtomicBool, Ordering as AtomicOrdering},
};
use std::sync::Arc;

use engine::realtime::{
    Consumer, Producer, QueueGeneration, bounded_spsc_move, bounded_spsc_retained_payload,
};

/// Preparation limits for controlled graph observation activation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObservationActivationConfig {
    /// Maximum number of controlled observers in one admitted snapshot.
    pub maximum_active_observers: usize,
    /// Inclusive retained-byte budget for the prepared activation transport.
    pub maximum_retained_bytes: u64,
}

/// Exact retained storage and transition limits for one prepared activation transport.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObservationActivationResources {
    /// Sum of every prepared activation allocation, including queue payloads, endpoints, and the
    /// containing executor's inline observation state. The latter overlaps baseline graph runtime
    /// metadata and is identified separately by [`Self::runtime_state_bytes`].
    pub retained_bytes: u64,
    /// Largest activation-owned standalone heap allocation. Inline controller and executor state
    /// are retained in [`Self::retained_bytes`] but are not standalone allocations.
    pub largest_allocation_bytes: u64,
    /// The containing graph executor layout delta occupied by observation activation state. A
    /// combined graph and activation estimate subtracts this once when the graph estimate already
    /// includes runtime metadata.
    pub runtime_state_bytes: u64,
    /// The configured controlled-observer population limit.
    pub maximum_active_observers: usize,
    /// Checked bound for one ordinary transition or two transitions at one boundary.
    pub maximum_transition_entry_visits_per_block: u64,
}

/// Admission refusal preserving the controller's last complete admitted set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphObservationAdmissionError {
    /// A requested handle is absent from the prepared catalog.
    UnknownHandle,
    /// A complete requested set contains one handle more than once.
    DuplicateHandle,
    /// A requested set exceeds the configured controlled population.
    ActiveCapacity,
    /// Preparation would exceed the exact retained-byte budget.
    RetainedBytes,
    /// A removal names a handle outside the last complete admitted set.
    InvalidRemoval,
    /// The bounded publication or retirement credit is already reserved.
    Backpressure,
    /// No further monotonic revision can be represented.
    RevisionExhausted,
    /// The realtime owner has already been closed.
    OwnerClosed,
}

/// Control-plane acknowledgement that a snapshot was reserved and published.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObservationAccepted {
    /// Monotonic activation revision assigned to the candidate.
    pub revision: u64,
}

/// Control-plane acknowledgement that a published snapshot was applied at a boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObservationApplied {
    /// Revision of the candidate that became active.
    pub revision: u64,
    /// Exclusive first sample of the block at whose boundary it became active.
    pub first_sample: u64,
}

/// One validated location in the immutable lowered observer layout.
///
/// The indices are intentionally `Copy`: the realtime snapshot carries no observer trait objects,
/// reference counts, or other resources. `member` is absent for a plain runtime operation.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ActivationEntry {
    pub(crate) unit: usize,
    pub(crate) member: Option<usize>,
    pub(crate) observer: usize,
    pub(crate) ordinal: usize,
}

/// One immutable stable-handle lookup row prepared by graph lowering.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) struct ActivationBinding {
    pub(crate) handle: u64,
    pub(crate) entry: ActivationEntry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PublicationKind {
    Ordinary,
    Removal,
}

/// A candidate snapshot owns exactly one of the three prepared backing arrays.
struct PublishedSnapshot {
    kind: PublicationKind,
    revision: u64,
    len: usize,
    entries: Box<[ActivationEntry]>,
}

/// A displaced snapshot awaiting control-side recycling.
///
/// The `kind` is the kind of the applied candidate, deliberately independent of the displaced
/// box's previous physical role. Recycling by this tag replenishes the exact credit consumed by
/// admission.
struct RetiredSnapshot {
    kind: PublicationKind,
    revision: u64,
    first_sample: u64,
    entries: Box<[ActivationEntry]>,
}

#[repr(C)]
struct SharedRendererAlive {
    strong: core::sync::atomic::AtomicUsize,
    weak: core::sync::atomic::AtomicUsize,
    alive: AtomicBool,
}

struct ActiveSnapshot {
    revision: u64,
    len: usize,
    entries: Box<[ActivationEntry]>,
}

impl ActiveSnapshot {
    fn entries(&self) -> &[ActivationEntry] {
        &self.entries[..self.len]
    }
}

/// Realtime owner of the active observation snapshot and its bounded publication consumers.
///
/// This is crate-private until the graph executor consumes it in tranche B. Its methods perform
/// only fixed-capacity queue operations, slice visits, and ownership swaps at the block boundary.
pub(crate) struct RealtimeObservationActivation {
    active: Option<ActiveSnapshot>,
    ordinary: Option<Consumer<PublishedSnapshot>>,
    removal: Option<Consumer<PublishedSnapshot>>,
    retirement: Option<Producer<RetiredSnapshot>>,
    pending: Option<PublishedSnapshot>,
    renderer_alive: Arc<AtomicBool>,
    resources: GraphObservationActivationResources,
}

/// Unique control-side owner of the prepared activation catalog and publication credits.
pub struct GraphObservationController {
    catalog: Box<[ActivationBinding]>,
    permanent: Box<[ActivationEntry]>,
    accepted_handles: Box<[u64]>,
    accepted_len: usize,
    ordinary_free: Option<Box<[ActivationEntry]>>,
    removal_free: Option<Box<[ActivationEntry]>>,
    ordinary: Option<Producer<PublishedSnapshot>>,
    removal: Option<Producer<PublishedSnapshot>>,
    retirement: Option<Consumer<RetiredSnapshot>>,
    next_revision: u64,
    ordinary_outstanding: bool,
    removal_outstanding: bool,
    renderer_alive: Arc<AtomicBool>,
    resources: GraphObservationActivationResources,
}

/// Prepare the fixed activation transport for one lowered observer catalog.
///
/// `catalog` must contain controlled observer rows only. Permanent rows are copied into every
/// prepared snapshot and are retained separately so the executor can keep their base dispatch
/// semantics. The helper is crate-private because only graph lowering can validate these indices.
pub(crate) fn prepare_activation(
    catalog: Box<[ActivationBinding]>,
    permanent: &[ActivationEntry],
    config: GraphObservationActivationConfig,
) -> Result<
    (GraphObservationController, RealtimeObservationActivation),
    GraphObservationAdmissionError,
> {
    let mut catalog = catalog.into_vec();
    catalog.sort_unstable_by_key(|binding| binding.handle);
    if catalog.windows(2).any(|pair| {
        pair[0].handle == pair[1].handle || pair[0].entry.ordinal == pair[1].entry.ordinal
    }) {
        return Err(GraphObservationAdmissionError::DuplicateHandle);
    }

    let mut permanent = permanent.to_vec();
    permanent.sort_unstable_by_key(|entry| entry.ordinal);
    if permanent
        .windows(2)
        .any(|pair| pair[0].ordinal == pair[1].ordinal)
    {
        return Err(GraphObservationAdmissionError::DuplicateHandle);
    }
    if catalog.iter().any(|binding| {
        permanent
            .binary_search_by_key(&binding.entry.ordinal, |entry| entry.ordinal)
            .is_ok()
    }) {
        return Err(GraphObservationAdmissionError::DuplicateHandle);
    }
    if config.maximum_active_observers == 0 {
        return Err(GraphObservationAdmissionError::ActiveCapacity);
    }

    // An empty controlled catalog has no activation pool. Permanent observers remain in the
    // executor's independent dispatch base and are retained here only for the prepared resource
    // projection and for callers that later add controlled rows through a fresh bind.
    let controlled_capacity = if catalog.is_empty() {
        0
    } else {
        config.maximum_active_observers
    };
    let capacity = controlled_capacity
        .checked_add(permanent.len())
        .ok_or(GraphObservationAdmissionError::RetainedBytes)?;
    let capacity = if catalog.is_empty() { 0 } else { capacity };
    let resources = activation_resources(
        catalog.len(),
        permanent.len(),
        controlled_capacity,
        config.maximum_active_observers,
    )?;
    if resources.retained_bytes > config.maximum_retained_bytes {
        return Err(GraphObservationAdmissionError::RetainedBytes);
    }

    let catalog = catalog.into_boxed_slice();
    let permanent = permanent.into_boxed_slice();
    let permanent_len = permanent.len();
    let accepted_handles = zeroed_u64_storage(controlled_capacity);
    let renderer_alive = Arc::new(AtomicBool::new(true));

    if capacity == 0 {
        let controller = GraphObservationController {
            catalog,
            permanent,
            accepted_handles,
            accepted_len: 0,
            ordinary_free: None,
            removal_free: None,
            ordinary: None,
            removal: None,
            retirement: None,
            next_revision: 1,
            ordinary_outstanding: false,
            removal_outstanding: false,
            renderer_alive: Arc::clone(&renderer_alive),
            resources,
        };
        let realtime = RealtimeObservationActivation {
            active: None,
            ordinary: None,
            removal: None,
            retirement: None,
            pending: None,
            renderer_alive,
            resources,
        };
        return Ok((controller, realtime));
    }

    let active = snapshot_storage(capacity, &permanent);
    let ordinary_free = snapshot_storage(capacity, &permanent);
    let removal_free = snapshot_storage(capacity, &permanent);
    let one = NonZeroUsize::new(1).expect("one is nonzero");
    let (ordinary, ordinary_consumer) = bounded_spsc_move(one, QueueGeneration(1))
        .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
    let (removal, removal_consumer) = bounded_spsc_move(one, QueueGeneration(2))
        .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
    let retirement_capacity = NonZeroUsize::new(2).expect("two is nonzero");
    let (retirement, retirement_consumer) =
        bounded_spsc_move(retirement_capacity, QueueGeneration(3))
            .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;

    let controller = GraphObservationController {
        catalog,
        permanent,
        accepted_handles,
        accepted_len: 0,
        ordinary_free: Some(ordinary_free),
        removal_free: Some(removal_free),
        ordinary: Some(ordinary),
        removal: Some(removal),
        retirement: Some(retirement_consumer),
        next_revision: 1,
        ordinary_outstanding: false,
        removal_outstanding: false,
        renderer_alive: Arc::clone(&renderer_alive),
        resources,
    };
    let realtime = RealtimeObservationActivation {
        active: Some(ActiveSnapshot {
            revision: 0,
            len: permanent_len,
            entries: active,
        }),
        ordinary: Some(ordinary_consumer),
        removal: Some(removal_consumer),
        retirement: Some(retirement),
        pending: None,
        renderer_alive,
        resources,
    };
    Ok((controller, realtime))
}

impl GraphObservationController {
    /// Admit a complete controlled-handle set as one ordinary replacement.
    #[allow(clippy::result_large_err)]
    pub fn replace(
        &mut self,
        handles: &[u64],
    ) -> Result<GraphObservationAccepted, GraphObservationAdmissionError> {
        if self.is_closed() {
            return Err(GraphObservationAdmissionError::OwnerClosed);
        }
        if self.removal_outstanding || self.ordinary_outstanding {
            return Err(GraphObservationAdmissionError::Backpressure);
        }
        self.validate_handles(handles)?;
        if self.next_revision == u64::MAX {
            return Err(GraphObservationAdmissionError::RevisionExhausted);
        }
        let Some(mut entries) = self.ordinary_free.take() else {
            return Err(GraphObservationAdmissionError::Backpressure);
        };
        let Some(ordinary) = self.ordinary.as_mut() else {
            self.ordinary_free = Some(entries);
            return Err(GraphObservationAdmissionError::OwnerClosed);
        };
        if ordinary.available_capacity() == 0 {
            self.ordinary_free = Some(entries);
            return Err(GraphObservationAdmissionError::Backpressure);
        }
        if !self.renderer_alive.load(AtomicOrdering::Acquire) {
            self.ordinary_free = Some(entries);
            return Err(GraphObservationAdmissionError::OwnerClosed);
        }
        let len = write_snapshot(&mut entries, &self.catalog, &self.permanent, handles);
        let revision = self.next_revision;
        let candidate = PublishedSnapshot {
            kind: PublicationKind::Ordinary,
            revision,
            len,
            entries,
        };
        match ordinary.try_push(candidate) {
            Ok(()) => {
                self.record_accepted(handles);
                self.ordinary_outstanding = true;
                self.next_revision += 1;
                Ok(GraphObservationAccepted { revision })
            }
            Err(full) => {
                self.ordinary_free = Some(full.value.entries);
                Err(GraphObservationAdmissionError::Backpressure)
            }
        }
    }

    /// Admit a complete removal/subset of the last accepted controlled-handle set.
    #[allow(clippy::result_large_err)]
    pub fn remove_to(
        &mut self,
        remaining_handles: &[u64],
    ) -> Result<GraphObservationAccepted, GraphObservationAdmissionError> {
        if self.is_closed() {
            return Err(GraphObservationAdmissionError::OwnerClosed);
        }
        if self.removal_outstanding {
            return Err(GraphObservationAdmissionError::Backpressure);
        }
        self.validate_handles(remaining_handles)?;
        if remaining_handles.iter().any(|handle| {
            self.accepted_handles[..self.accepted_len]
                .binary_search(handle)
                .is_err()
        }) {
            return Err(GraphObservationAdmissionError::InvalidRemoval);
        }
        if self.next_revision == u64::MAX {
            return Err(GraphObservationAdmissionError::RevisionExhausted);
        }
        let Some(mut entries) = self.removal_free.take() else {
            return Err(GraphObservationAdmissionError::Backpressure);
        };
        let Some(removal) = self.removal.as_mut() else {
            self.removal_free = Some(entries);
            return Err(GraphObservationAdmissionError::OwnerClosed);
        };
        if removal.available_capacity() == 0 {
            self.removal_free = Some(entries);
            return Err(GraphObservationAdmissionError::Backpressure);
        }
        if !self.renderer_alive.load(AtomicOrdering::Acquire) {
            self.removal_free = Some(entries);
            return Err(GraphObservationAdmissionError::OwnerClosed);
        }
        let len = write_snapshot(
            &mut entries,
            &self.catalog,
            &self.permanent,
            remaining_handles,
        );
        let revision = self.next_revision;
        let candidate = PublishedSnapshot {
            kind: PublicationKind::Removal,
            revision,
            len,
            entries,
        };
        match removal.try_push(candidate) {
            Ok(()) => {
                self.record_accepted(remaining_handles);
                self.removal_outstanding = true;
                self.next_revision += 1;
                Ok(GraphObservationAccepted { revision })
            }
            Err(full) => {
                self.removal_free = Some(full.value.entries);
                Err(GraphObservationAdmissionError::Backpressure)
            }
        }
    }

    /// Recycle one applied backing array and return its sample-accurate receipt.
    pub fn try_applied(&mut self) -> Option<GraphObservationApplied> {
        let retirement = self.retirement.as_mut()?;
        let retired = retirement.try_pop().ok()?;
        let receipt = GraphObservationApplied {
            revision: retired.revision,
            first_sample: retired.first_sample,
        };
        match retired.kind {
            PublicationKind::Ordinary => {
                debug_assert!(self.ordinary_free.is_none());
                self.ordinary_free = Some(retired.entries);
                self.ordinary_outstanding = false;
            }
            PublicationKind::Removal => {
                debug_assert!(self.removal_free.is_none());
                self.removal_free = Some(retired.entries);
                self.removal_outstanding = false;
            }
        }
        Some(receipt)
    }

    /// Return the exact prepared activation resource projection.
    #[must_use]
    pub const fn resources(&self) -> GraphObservationActivationResources {
        self.resources
    }

    /// Whether the realtime endpoint has been disposed off the render thread.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        !self.renderer_alive.load(AtomicOrdering::Acquire)
    }

    fn validate_handles(&self, handles: &[u64]) -> Result<(), GraphObservationAdmissionError> {
        for (index, handle) in handles.iter().enumerate() {
            if handles[index + 1..].contains(handle) {
                return Err(GraphObservationAdmissionError::DuplicateHandle);
            }
        }
        if handles.len() > self.resources.maximum_active_observers {
            return Err(GraphObservationAdmissionError::ActiveCapacity);
        }
        for handle in handles {
            if self
                .catalog
                .binary_search_by_key(handle, |binding| binding.handle)
                .is_err()
            {
                return Err(GraphObservationAdmissionError::UnknownHandle);
            }
        }
        Ok(())
    }

    fn record_accepted(&mut self, handles: &[u64]) {
        self.accepted_len = handles.len();
        self.accepted_handles[..self.accepted_len].copy_from_slice(handles);
        self.accepted_handles[..self.accepted_len].sort_unstable();
    }
}

impl RealtimeObservationActivation {
    /// Apply at most the two snapshots admitted before this block entry.
    // REALTIME_POLICY_BEGIN
    pub(crate) fn apply_boundary<F>(&mut self, first_sample: u64, mut on_changed: F)
    where
        F: FnMut(ActivationEntry, bool, u64, u64),
    {
        let pending_count = usize::from(self.pending.is_some());
        let removal_count = self
            .removal
            .as_ref()
            .map_or(0, Consumer::available_at_entry)
            .min(1);
        // A removal can only be published after any ordinary candidate it supersedes. Observe its
        // queue first, then sample ordinary availability: if removal is visible, every older
        // ordinary publication is visible to this bounded entry snapshot as well.
        let ordinary_count = self
            .ordinary
            .as_ref()
            .map_or(0, Consumer::available_at_entry)
            .min(1);
        let candidate_count = pending_count + ordinary_count + removal_count;
        if candidate_count == 0 || candidate_count > 2 {
            return;
        }
        if self
            .retirement
            .as_ref()
            .is_none_or(|retirement| retirement.available_capacity() < candidate_count)
        {
            return;
        }

        // The control owner can admit an ordinary candidate before a removal candidate, but it
        // refuses ordinary starts while a removal is outstanding. Therefore an ordinary row, when
        // present, is always the lower revision and can be applied before reading the removal
        // queue. Keeping the second queue unread until the first succeeds preserves its box if a
        // defensive retirement check ever defers the first candidate.
        if let Some(candidate) = self.pending.take()
            && !self.apply_candidate(candidate, first_sample, &mut on_changed)
        {
            return;
        }
        if ordinary_count != 0 {
            let Some(candidate) = self
                .ordinary
                .as_mut()
                .and_then(|queue| queue.try_pop().ok())
            else {
                return;
            };
            if !self.apply_candidate(candidate, first_sample, &mut on_changed) {
                return;
            }
        }
        if removal_count != 0 {
            let Some(candidate) = self.removal.as_mut().and_then(|queue| queue.try_pop().ok())
            else {
                return;
            };
            self.apply_candidate(candidate, first_sample, &mut on_changed);
        }
    }
    // REALTIME_POLICY_END

    /// Borrow the currently active immutable entry slice for the executor's dispatch cursor.
    pub(crate) fn entries(&self) -> &[ActivationEntry] {
        self.active.as_ref().map_or(&[], ActiveSnapshot::entries)
    }

    /// Return the same exact prepared resource projection held by the control owner.
    pub(crate) const fn resources(&self) -> GraphObservationActivationResources {
        self.resources
    }

    // REALTIME_POLICY_BEGIN
    fn apply_candidate<F>(
        &mut self,
        candidate: PublishedSnapshot,
        first_sample: u64,
        on_changed: &mut F,
    ) -> bool
    where
        F: FnMut(ActivationEntry, bool, u64, u64),
    {
        let Some((active_revision, active_len)) = self
            .active
            .as_ref()
            .map(|active| (active.revision, active.len))
        else {
            self.pending = Some(candidate);
            return false;
        };
        if candidate.revision <= active_revision {
            self.pending = Some(candidate);
            return false;
        }

        // `available_capacity` above is a monotonic control-side observation: only the retirement
        // consumer can increase it. The checked push therefore cannot reject in the normal
        // ownership protocol. Keep the explicit check so a defensive failure retains the entire
        // candidate and never claims its revision as applied.
        let Some(retirement) = self.retirement.as_mut() else {
            self.pending = Some(candidate);
            return false;
        };
        if retirement.available_capacity() == 0 {
            self.pending = Some(candidate);
            return false;
        }

        {
            let Some(active) = self.active.as_ref() else {
                self.pending = Some(candidate);
                return false;
            };
            notify_changes(
                active.entries(),
                &candidate.entries[..candidate.len],
                candidate.revision,
                first_sample,
                on_changed,
            );
        }
        let Some(old) = self.active.take() else {
            self.pending = Some(candidate);
            return false;
        };
        let retired = RetiredSnapshot {
            kind: candidate.kind,
            revision: candidate.revision,
            first_sample,
            entries: old.entries,
        };
        match retirement.try_push(retired) {
            Ok(()) => {
                self.active = Some(ActiveSnapshot {
                    revision: candidate.revision,
                    len: candidate.len,
                    entries: candidate.entries,
                });
                true
            }
            Err(full) => {
                // This can only be reached if a peer changed the retirement cursor between the
                // bounded preflight and publication. Restore both owners and defer the candidate.
                let old = full.value;
                self.active = Some(ActiveSnapshot {
                    revision: active_revision,
                    len: active_len,
                    entries: old.entries,
                });
                self.pending = Some(candidate);
                false
            }
        }
    }
    // REALTIME_POLICY_END
}

impl Drop for RealtimeObservationActivation {
    fn drop(&mut self) {
        self.renderer_alive.store(false, AtomicOrdering::Release);
    }
}

// REALTIME_POLICY_BEGIN
fn notify_changes<F>(
    old: &[ActivationEntry],
    new: &[ActivationEntry],
    revision: u64,
    first_sample: u64,
    on_changed: &mut F,
) where
    F: FnMut(ActivationEntry, bool, u64, u64),
{
    let mut old_index = 0;
    let mut new_index = 0;
    while old_index < old.len() || new_index < new.len() {
        match (old.get(old_index), new.get(new_index)) {
            (Some(old_entry), Some(new_entry)) => match old_entry.ordinal.cmp(&new_entry.ordinal) {
                Ordering::Less => {
                    on_changed(*old_entry, false, revision, first_sample);
                    old_index += 1;
                }
                Ordering::Greater => {
                    on_changed(*new_entry, true, revision, first_sample);
                    new_index += 1;
                }
                Ordering::Equal => {
                    if old_entry != new_entry {
                        on_changed(*old_entry, false, revision, first_sample);
                        on_changed(*new_entry, true, revision, first_sample);
                    }
                    old_index += 1;
                    new_index += 1;
                }
            },
            (Some(old_entry), None) => {
                on_changed(*old_entry, false, revision, first_sample);
                old_index += 1;
            }
            (None, Some(new_entry)) => {
                on_changed(*new_entry, true, revision, first_sample);
                new_index += 1;
            }
            (None, None) => break,
        }
    }
}
// REALTIME_POLICY_END

fn snapshot_storage(capacity: usize, permanent: &[ActivationEntry]) -> Box<[ActivationEntry]> {
    let mut storage = vec![ActivationEntry::default(); capacity];
    storage[..permanent.len()].copy_from_slice(permanent);
    storage.into_boxed_slice()
}

fn zeroed_u64_storage(capacity: usize) -> Box<[u64]> {
    vec![0; capacity].into_boxed_slice()
}

fn write_snapshot(
    storage: &mut [ActivationEntry],
    catalog: &[ActivationBinding],
    permanent: &[ActivationEntry],
    handles: &[u64],
) -> usize {
    storage[..permanent.len()].copy_from_slice(permanent);
    for (index, handle) in handles.iter().enumerate() {
        storage[permanent.len() + index] = catalog
            .binary_search_by_key(handle, |binding| binding.handle)
            .map(|index| catalog[index].entry)
            .expect("validated activation handle");
    }
    let len = permanent.len() + handles.len();
    storage[..len].sort_unstable_by_key(|entry| entry.ordinal);
    len
}

fn activation_resources(
    catalog_len: usize,
    permanent_len: usize,
    controlled_capacity: usize,
    maximum_active_observers: usize,
) -> Result<GraphObservationActivationResources, GraphObservationAdmissionError> {
    let transition_population = maximum_active_observers
        .checked_add(permanent_len)
        .ok_or(GraphObservationAdmissionError::RetainedBytes)?;
    let maximum_transition_entry_visits_per_block = u64::try_from(transition_population)
        .ok()
        .and_then(|population| population.checked_mul(4))
        .ok_or(GraphObservationAdmissionError::RetainedBytes)?;

    let snapshot_capacity = if controlled_capacity == 0 {
        0
    } else {
        controlled_capacity
            .checked_add(permanent_len)
            .ok_or(GraphObservationAdmissionError::RetainedBytes)?
    };
    let snapshot_bytes = layout_array::<ActivationEntry>(snapshot_capacity)?;
    let catalog_bytes = layout_array::<ActivationBinding>(catalog_len)?;
    let permanent_bytes = layout_array::<ActivationEntry>(permanent_len)?;
    let accepted_bytes = if controlled_capacity == 0 {
        0
    } else {
        layout_array::<u64>(controlled_capacity)?
    };
    let shared_alive_bytes = layout_array::<SharedRendererAlive>(1)?;

    let mut rows = [0_u64; 13];
    rows[..7].copy_from_slice(&[
        snapshot_bytes,
        snapshot_bytes,
        snapshot_bytes,
        catalog_bytes,
        permanent_bytes,
        accepted_bytes,
        shared_alive_bytes,
    ]);
    if snapshot_bytes != 0 {
        let publication = bounded_spsc_retained_payload::<PublishedSnapshot>(
            NonZeroUsize::new(1).expect("one is nonzero"),
        )
        .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
        let retirement = bounded_spsc_retained_payload::<RetiredSnapshot>(
            NonZeroUsize::new(2).expect("two is nonzero"),
        )
        .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
        rows[7] = u64::try_from(publication.ring_header_bytes)
            .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
        rows[8] = u64::try_from(publication.slot_payload_bytes)
            .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
        rows[9] = rows[7];
        rows[10] = rows[8];
        rows[11] = u64::try_from(retirement.ring_header_bytes)
            .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
        rows[12] = u64::try_from(retirement.slot_payload_bytes)
            .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
    }
    let heap_bytes = rows.iter().try_fold(0_u64, |total, row| {
        total
            .checked_add(*row)
            .ok_or(GraphObservationAdmissionError::RetainedBytes)
    })?;
    let (_, runtime_state_bytes) =
        crate::observation_runtime_layout().ok_or(GraphObservationAdmissionError::RetainedBytes)?;
    let controller_bytes = u64::try_from(core::mem::size_of::<GraphObservationController>())
        .map_err(|_| GraphObservationAdmissionError::RetainedBytes)?;
    let retained_bytes = heap_bytes
        .checked_add(controller_bytes)
        .and_then(|total| total.checked_add(runtime_state_bytes))
        .ok_or(GraphObservationAdmissionError::RetainedBytes)?;
    let largest_allocation_bytes = rows.iter().copied().max().unwrap_or(0);
    Ok(GraphObservationActivationResources {
        retained_bytes,
        largest_allocation_bytes,
        runtime_state_bytes,
        maximum_active_observers,
        maximum_transition_entry_visits_per_block,
    })
}

fn layout_array<T>(length: usize) -> Result<u64, GraphObservationAdmissionError> {
    Layout::array::<T>(length)
        .map(|layout| u64::try_from(layout.size()).unwrap_or(u64::MAX))
        .map_err(|_: LayoutError| GraphObservationAdmissionError::RetainedBytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn independent_layout<T>(length: usize) -> u64 {
        u64::try_from(
            Layout::array::<T>(length)
                .expect("independent layout")
                .size(),
        )
        .expect("independent layout fits u64")
    }

    fn catalog() -> Box<[ActivationBinding]> {
        vec![
            ActivationBinding {
                handle: 10,
                entry: ActivationEntry {
                    unit: 1,
                    member: None,
                    observer: 0,
                    ordinal: 1,
                },
            },
            ActivationBinding {
                handle: 20,
                entry: ActivationEntry {
                    unit: 2,
                    member: Some(0),
                    observer: 1,
                    ordinal: 2,
                },
            },
        ]
        .into_boxed_slice()
    }

    fn config() -> GraphObservationActivationConfig {
        GraphObservationActivationConfig {
            maximum_active_observers: 2,
            maximum_retained_bytes: u64::MAX,
        }
    }

    #[test]
    fn admission_preserves_credits_and_applies_revisions_in_order() {
        let (mut controller, mut realtime) =
            prepare_activation(catalog(), &[], config()).expect("activation");
        let first = controller.replace(&[20, 10]).expect("ordinary");
        assert_eq!(first.revision, 1);
        let second = controller.remove_to(&[10]).expect("reserved removal");
        assert_eq!(second.revision, 2);
        assert_eq!(
            controller.remove_to(&[]),
            Err(GraphObservationAdmissionError::Backpressure)
        );
        assert_eq!(
            controller.replace(&[20]),
            Err(GraphObservationAdmissionError::Backpressure)
        );

        let mut changes = Vec::new();
        realtime.apply_boundary(480, |entry, active, revision, sample| {
            changes.push((entry.ordinal, active, revision, sample));
        });
        assert_eq!(realtime.entries().len(), 1);
        assert_eq!(
            changes,
            vec![(1, true, 1, 480), (2, true, 1, 480), (2, false, 2, 480)]
        );
        assert_eq!(
            controller.try_applied(),
            Some(GraphObservationApplied {
                revision: 1,
                first_sample: 480
            })
        );
        assert_eq!(
            controller.try_applied(),
            Some(GraphObservationApplied {
                revision: 2,
                first_sample: 480
            })
        );
        assert_eq!(controller.try_applied(), None);
        controller.replace(&[20]).expect("ordinary credit recycled");
    }

    #[test]
    fn refusal_keeps_last_admitted_set_and_exact_budget_is_inclusive() {
        let mut config = config();
        let (controller, _) = prepare_activation(catalog(), &[], config).expect("activation");
        let resources = controller.resources();
        config.maximum_retained_bytes = resources.retained_bytes - 1;
        assert!(matches!(
            prepare_activation(catalog(), &[], config),
            Err(GraphObservationAdmissionError::RetainedBytes)
        ));

        let (mut controller, mut realtime) = prepare_activation(
            catalog(),
            &[],
            GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            },
        )
        .expect("activation");
        assert_eq!(
            controller.replace(&[10, 20]),
            Err(GraphObservationAdmissionError::ActiveCapacity)
        );
        assert_eq!(
            controller.replace(&[10, 10]),
            Err(GraphObservationAdmissionError::DuplicateHandle)
        );
        assert_eq!(
            controller.replace(&[99]),
            Err(GraphObservationAdmissionError::UnknownHandle)
        );
        controller.replace(&[10]).expect("valid ordinary");
        realtime.apply_boundary(0, |_, _, _, _| {});
        assert_eq!(
            controller.remove_to(&[20]),
            Err(GraphObservationAdmissionError::InvalidRemoval)
        );
        assert_eq!(controller.try_applied().unwrap().revision, 1);
    }

    #[test]
    fn empty_catalog_has_no_activation_pool() {
        let (mut controller, realtime) = prepare_activation(
            Vec::new().into_boxed_slice(),
            &[],
            GraphObservationActivationConfig {
                maximum_active_observers: 32,
                maximum_retained_bytes: u64::MAX,
            },
        )
        .expect("empty activation");
        assert!(controller.resources().retained_bytes > 0);
        assert_eq!(realtime.entries(), &[]);
        assert_eq!(
            controller.replace(&[]),
            Err(GraphObservationAdmissionError::Backpressure)
        );
        drop(realtime);
        assert!(controller.is_closed());
        assert_eq!(
            controller.replace(&[]),
            Err(GraphObservationAdmissionError::OwnerClosed)
        );
    }

    #[test]
    fn resource_report_matches_independent_heap_and_inline_layout_oracle() {
        let permanent = [ActivationEntry {
            unit: 0,
            member: None,
            observer: 9,
            ordinal: 0,
        }];
        let maximum_active_observers = 3;
        let (controller, _) = prepare_activation(
            catalog(),
            &permanent,
            GraphObservationActivationConfig {
                maximum_active_observers,
                maximum_retained_bytes: u64::MAX,
            },
        )
        .expect("activation");
        let resources = controller.resources();
        let snapshot_capacity = maximum_active_observers + permanent.len();
        let catalog_bytes = independent_layout::<ActivationBinding>(2);
        let permanent_bytes = independent_layout::<ActivationEntry>(permanent.len());
        let accepted_bytes = independent_layout::<u64>(maximum_active_observers);
        let snapshot_bytes = independent_layout::<ActivationEntry>(snapshot_capacity);
        let alive_bytes = independent_layout::<SharedRendererAlive>(1);
        let publication =
            bounded_spsc_retained_payload::<PublishedSnapshot>(NonZeroUsize::new(1).expect("one"))
                .expect("publication layout");
        let removal =
            bounded_spsc_retained_payload::<PublishedSnapshot>(NonZeroUsize::new(1).expect("one"))
                .expect("removal layout");
        let retirement =
            bounded_spsc_retained_payload::<RetiredSnapshot>(NonZeroUsize::new(2).expect("two"))
                .expect("retirement layout");
        assert_eq!(
            publication.slot_count, 2,
            "publication includes sentinel slot"
        );
        assert_eq!(removal.slot_count, 2, "removal includes sentinel slot");
        assert_eq!(
            retirement.slot_count, 3,
            "retirement includes sentinel slot"
        );

        let queue_bytes = |payload: engine::realtime::SpscRetainedPayload| {
            u64::try_from(payload.ring_header_bytes)
                .expect("ring header fits u64")
                .checked_add(u64::try_from(payload.slot_payload_bytes).expect("slots fit u64"))
                .expect("queue layout fits u64")
        };
        let heap_rows = [
            snapshot_bytes,
            snapshot_bytes,
            snapshot_bytes,
            catalog_bytes,
            permanent_bytes,
            accepted_bytes,
            alive_bytes,
            u64::try_from(publication.ring_header_bytes).expect("ring header fits u64"),
            u64::try_from(publication.slot_payload_bytes).expect("slots fit u64"),
            u64::try_from(removal.ring_header_bytes).expect("ring header fits u64"),
            u64::try_from(removal.slot_payload_bytes).expect("slots fit u64"),
            u64::try_from(retirement.ring_header_bytes).expect("ring header fits u64"),
            u64::try_from(retirement.slot_payload_bytes).expect("slots fit u64"),
        ];
        let heap_total = heap_rows
            .iter()
            .try_fold(0_u64, |total, row| total.checked_add(*row))
            .expect("heap layout fits u64");
        let controller_bytes = u64::try_from(core::mem::size_of::<GraphObservationController>())
            .expect("controller layout fits u64");
        let (_, runtime_state_bytes) = crate::test_only_observation_runtime_layout();
        let expected_retained = heap_total
            .checked_add(controller_bytes)
            .and_then(|total| total.checked_add(runtime_state_bytes))
            .expect("activation layout fits u64");
        assert_eq!(resources.retained_bytes, expected_retained);
        assert_eq!(resources.runtime_state_bytes, runtime_state_bytes);
        assert_eq!(
            resources.largest_allocation_bytes,
            heap_rows.iter().copied().max().expect("heap rows")
        );
        assert_eq!(
            queue_bytes(publication) + queue_bytes(removal) + queue_bytes(retirement),
            heap_rows[7..].iter().sum::<u64>()
        );
    }
}
