//! The one read of observation windows by stable address (issue #1586 D3, D4).
//!
//! A host reads the latest window of up to [`MAXIMUM_OBSERVATION_SELECTIONS`] observation taps,
//! each named by its strip ID, its section (a console slot or an insert), the console slot's or
//! insert's ID and the effect-local tap ID. The read never consumes anything: it takes each tap's
//! slot through a read-only [`ObservationView`], which has no `acknowledge`: the `consumed` word
//! belongs to the meter fold's [`ObservationReader`].
//!
//! # The index holds read-only endpoints (#1332 S6b, root ruling R-S6b)
//!
//! [`ObservationIndex`] is built once per plan on the control thread. Each entry is one effect
//! instance's stable key, the position of its [`EffectObservationHandle`] in the plan's handle list
//! (`None` for an effect that declares no tap) and its descriptor. Beside the entries it keeps one
//! [`ObservationView`] per tap of every handle, in handle order: one pointer each, cloned from the
//! handle's readers, so the read needs no handle list and runs on whatever thread holds the index,
//! while the readers themselves stay with whoever acknowledges (the C ABI's epoch, or the browser's
//! render half). A carried plan swap exchanges reader sets between two plans' handles at the same
//! positions (`pair_carried_observations`); the control side exchanges the two indexes' views at
//! the same positions from the swap record ([`ObservationIndex::pair_carried`]), so the read
//! follows the slot that renders on: an index whose views were not exchanged would keep reading
//! the successor's fresh slot, which nothing publishes to after a carry (#1586a).
//!
//! # Status (#1585, as merged)
//!
//! The status of a row comes from one locked read of the tap's slot and from the tag and kind of
//! the last record the control plane issued for the tap, which the host supplies through the
//! `issued` lookup (host-core does not own the control plane's table):
//!
//! - **Unarmed** when the last issued record is a disarm (it is in effect when it is accepted), or
//!   when the slot reads unarmed with a tag at or above the issued one.
//! - **Ready** when the slot is armed with a tag at or above the issued one and its window opened
//!   at or after the arm sample ([`window_since_arm`]).
//! - **Pending** otherwise: an arm or re-arm render has not applied (the slot's tag is below the
//!   issued tag; tags are monotone), an armed tap with no window since its arm, or a read that tore
//!   on every attempt.

use effect_contract::{EffectDescriptor, ObservationCost, ObservationDescriptor};
use engine::realtime::{
    ObservationReader, ObservationState, ObservationView, ObservationWindow, ObserverCarry,
};

use crate::meters::window_since_arm;
use crate::{EffectObservationHandle, LiveEffectRack};

/// The most selections one read takes.
pub const MAXIMUM_OBSERVATION_SELECTIONS: usize = 256;

/// The lanes one selection asks for.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationReadChannels {
    /// Return the owner-published left value.
    Left,
    /// Return the owner-published right value.
    Right,
    /// Return both owner-published lane values independently.
    Both,
}

/// Availability of one selected observation at the time of a read (the rule is the module's).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ObservationReadStatus {
    /// The tap's last subscription is an arm render has not applied yet, or it is armed and no
    /// complete window has been published since its arm (#1585 D4).
    #[default]
    Pending,
    /// The tap exists, and its last subscription is a disarm, in effect when it is accepted, or
    /// it has none (#1585 D3, D4).
    Unarmed,
    /// A complete resident window was read without consuming it.
    Ready,
}

/// One selection: a tap by its stable address, and the lanes to return.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationSelection<'a> {
    /// The ID of the track or submix.
    pub strip_id: &'a str,
    /// The console slot or the insert section.
    pub section: LiveEffectRack,
    /// The console slot's or the insert's ID.
    pub effect_id: &'a str,
    /// The effect-local tap ID.
    pub tap_id: u32,
    /// The lanes to return.
    pub channels: ObservationReadChannels,
}

/// The stable key of one tap, as the `issued` lookup of [`read_into`] receives it: the index
/// entry's own IDs, so it borrows nothing from the caller's selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationKey<'a> {
    /// The ID of the track or submix.
    pub strip_id: &'a str,
    /// The console slot or the insert section.
    pub section: LiveEffectRack,
    /// The console slot's or the insert's ID.
    pub effect_id: &'a str,
    /// The effect-local tap ID.
    pub tap_id: u32,
}

/// One row of a read, for the selection at the same position.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObservationRow {
    /// The row's status.
    pub status: ObservationReadStatus,
    /// The slot's published state the status was taken from: `None` when the read tore on every
    /// attempt or when the slot's tag is below the issued tag (render has not applied the tap's
    /// last record).
    pub state: Option<ObservationState>,
    /// The window, present exactly when the status is [`ObservationReadStatus::Ready`].
    pub window: Option<ObservationWindow>,
    /// The left lane, present when ready and asked for.
    pub left: Option<f32>,
    /// The right lane, present when ready and asked for.
    pub right: Option<f32>,
    /// The tap's declaration, which names its native unit: always present in a row a read wrote,
    /// `None` only in a default row.
    pub tap: Option<&'static ObservationDescriptor>,
}

/// Why a read refused its whole batch, and which selection it names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObservationReadError {
    /// The first selection that failed the check; the selection count for a batch-level refusal.
    pub selection: usize,
    /// What was wrong with it.
    pub refusal: ObservationRefusal,
}

/// The refusals of one read (#1586 D4). Nothing is written when one is returned.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationRefusal {
    /// More than [`MAXIMUM_OBSERVATION_SELECTIONS`] selections.
    TooManySelections,
    /// Fewer rows than selections.
    RowsTooShort,
    /// The plan has no such strip, console slot or insert (`observation.address.unknown`).
    UnknownAddress,
    /// The effect declares no such tap (`observation.tap.unknown`).
    UnknownTap,
    /// The tap is computed, not resident (`observation.tap.unsupported`).
    UnsupportedTap,
    /// The effect has no observation lane for the tap (`observation.unbound`).
    Unbound,
    /// An earlier selection names the same address and tap (`observation.selection.duplicate`).
    Duplicate,
}

/// One effect instance of a plan, as the index keeps it.
#[derive(Debug)]
pub struct ObservationIndexEntry {
    /// The ID of the track or submix.
    pub strip_id: Box<str>,
    /// The console slot or the insert section.
    pub section: LiveEffectRack,
    /// The console slot's or the insert's ID.
    pub effect_id: Box<str>,
    /// The position of the instance's [`EffectObservationHandle`] in the plan's handle list, or
    /// `None` for an instance that declares no tap.
    pub handle: Option<u32>,
    /// The instance's declared effect.
    pub effect: &'static EffectDescriptor,
}

/// Why an index was not built.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObservationIndexError {
    /// A second entry named a key an earlier one named.
    DuplicateKey,
    /// An entry names a handle position outside the handle list, or a count does not fit a `u32`.
    Handle,
    /// The view table could not be allocated.
    Allocation,
}

/// One plan's effect instances, sorted by `(strip, section, effect)`, so a selection resolves by a
/// binary search and a read allocates nothing, and the read-only endpoint of every tap.
#[derive(Debug, Default)]
pub struct ObservationIndex {
    entries: Box<[ObservationIndexEntry]>,
    /// One view per reader of every handle, in handle order, then reader order.
    taps: Box<[ObservationView]>,
    /// Where each handle's views start in `taps`, one per handle and one past the last: handle
    /// `h`'s views are `taps[starts[h]..starts[h + 1]]`. Empty for an index of no handle.
    starts: Box<[u32]>,
}

impl ObservationIndex {
    /// Sort `entries` by key and keep them, with a read-only view of every reader of `handles`,
    /// the list the entries' positions name. Allocates the view and start tables, and the entry
    /// table only when `entries` has spare capacity (the conversion to a boxed slice shrinks it).
    /// No slot is allocated: a view shares its reader's. Control thread.
    ///
    /// # Errors
    ///
    /// Two entries with the same key, a position outside `handles`, or an allocation failure.
    pub fn new(
        mut entries: Vec<ObservationIndexEntry>,
        handles: &[EffectObservationHandle],
    ) -> Result<Self, ObservationIndexError> {
        entries.sort_unstable_by(|left, right| key_of(left).cmp(&key_of(right)));
        if entries
            .windows(2)
            .any(|pair| key_of(&pair[0]) == key_of(&pair[1]))
        {
            return Err(ObservationIndexError::DuplicateKey);
        }
        if entries
            .iter()
            .filter_map(|entry| entry.handle)
            .any(|handle| handle as usize >= handles.len())
        {
            return Err(ObservationIndexError::Handle);
        }
        let tap_count: usize = handles.iter().map(|handle| handle.readers.len()).sum();
        let mut taps = Vec::new();
        taps.try_reserve_exact(tap_count)
            .map_err(|_| ObservationIndexError::Allocation)?;
        let mut starts = Vec::new();
        if !handles.is_empty() {
            starts
                .try_reserve_exact(handles.len() + 1)
                .map_err(|_| ObservationIndexError::Allocation)?;
        }
        for handle in handles {
            starts.push(u32::try_from(taps.len()).map_err(|_| ObservationIndexError::Handle)?);
            taps.extend(handle.readers.iter().map(ObservationReader::view));
        }
        if !handles.is_empty() {
            starts.push(u32::try_from(taps.len()).map_err(|_| ObservationIndexError::Handle)?);
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
            taps: taps.into_boxed_slice(),
            starts: starts.into_boxed_slice(),
        })
    }

    /// The views of the handle at `position`, or `None` past the list.
    fn handle_taps(&self, position: u32) -> Option<&[ObservationView]> {
        let range = self.handle_range(position)?;
        self.taps.get(range)
    }

    fn handle_range(&self, position: u32) -> Option<core::ops::Range<usize>> {
        let position = position as usize;
        let start = *self.starts.get(position)? as usize;
        let end = *self.starts.get(position.checked_add(1)?)? as usize;
        Some(start..end)
    }

    /// Exchange the views of each carried observation pair between `successor`'s and
    /// `predecessor`'s tables, from the swap record's `carries` (handle positions, as
    /// [`crate::pair_carried_observations`] reads them): the control side's half of the pairing
    /// the render side made in the swap block, so the successor's views read the slots that render
    /// on (#1332 S6b). Each carried pair is one effect, so the two handles have equal tap counts;
    /// the exchange stops at the shorter. A record out of range of either table is skipped.
    /// Allocation-free; drops nothing.
    pub fn pair_carried(successor: &mut Self, predecessor: &mut Self, carries: &[ObserverCarry]) {
        for carry in carries {
            if carry.kind != engine::realtime::ObserverKind::Observation {
                continue;
            }
            let (Ok(into), Ok(from)) = (
                u32::try_from(carry.successor),
                u32::try_from(carry.predecessor),
            ) else {
                continue;
            };
            let (Some(into), Some(from)) =
                (successor.handle_range(into), predecessor.handle_range(from))
            else {
                continue;
            };
            let (Some(into), Some(from)) =
                (successor.taps.get_mut(into), predecessor.taps.get_mut(from))
            else {
                continue;
            };
            for (into, from) in into.iter_mut().zip(from) {
                core::mem::swap(into, from);
            }
        }
    }

    /// The entry for one effect instance, or `None` when the plan has none at that address.
    #[must_use]
    pub fn find(
        &self,
        strip_id: &str,
        section: LiveEffectRack,
        effect_id: &str,
    ) -> Option<&ObservationIndexEntry> {
        let wanted = (strip_id, section, effect_id);
        self.entries
            .binary_search_by(|entry| key_of(entry).cmp(&wanted))
            .ok()
            .map(|position| &self.entries[position])
    }

    /// The heap the index holds: its entry table, the IDs it copies, and its view and start
    /// tables (one pointer per tap, one `u32` per handle and one more; the views' slots are their
    /// readers', counted with them). `None` on overflow.
    #[must_use]
    pub fn retained_bytes(&self) -> Option<u64> {
        let table = u64::try_from(self.entries.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<ObservationIndexEntry>()).ok()?)?;
        let views = u64::try_from(self.taps.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<ObservationView>()).ok()?)?;
        let starts = u64::try_from(self.starts.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<u32>()).ok()?)?;
        let fixed = table.checked_add(views)?.checked_add(starts)?;
        self.entries.iter().try_fold(fixed, |total, entry| {
            total.checked_add(u64::try_from(entry.strip_id.len() + entry.effect_id.len()).ok()?)
        })
    }
}

fn key_of(entry: &ObservationIndexEntry) -> (&str, LiveEffectRack, &str) {
    (&entry.strip_id, entry.section, &entry.effect_id)
}

/// Read one batch into `rows`, one row per selection in order (#1586 D3).
///
/// The read goes through the index's own views ([`ObservationIndex::pair_carried`] keeps them on
/// the slots that render). `issued` returns the tag of the last record the control plane
/// issued for a tap and whether that record arms it (`(0, false)` for none). The whole batch is
/// checked before any row is written (D4). Allocates nothing and never acknowledges.
///
/// # Errors
///
/// The first refusal, in selection order; `rows` is untouched.
pub fn read_into(
    index: &ObservationIndex,
    selections: &[ObservationSelection<'_>],
    issued: impl FnMut(&ObservationKey<'_>) -> (u64, bool),
    rows: &mut [ObservationRow],
) -> Result<(), ObservationReadError> {
    if selections.len() > MAXIMUM_OBSERVATION_SELECTIONS {
        return Err(batch_refusal(
            selections.len(),
            ObservationRefusal::TooManySelections,
        ));
    }
    if rows.len() < selections.len() {
        return Err(batch_refusal(
            selections.len(),
            ObservationRefusal::RowsTooShort,
        ));
    }
    read_with(
        index,
        selections,
        |selection| Ok(*selection),
        issued,
        |position, _, row| rows[position] = row,
    )
}

/// [`read_into`] over a host's own selection records: `select` maps each record to its stable
/// selection, or refuses it with the host's own error, in the same pass as the check, so the first
/// refusal in record order is the one returned; `write` receives each row, with the index entry
/// its selection resolved to, once the whole batch passed. Allocates nothing and never acknowledges.
///
/// # Errors
///
/// The first refusal, in record order: `select`'s, or a [`ObservationReadError`] converted.
pub fn read_with<'s, S, E: From<ObservationReadError>>(
    index: &ObservationIndex,
    selections: &[S],
    mut select: impl FnMut(&S) -> Result<ObservationSelection<'s>, E>,
    mut issued: impl FnMut(&ObservationKey<'_>) -> (u64, bool),
    mut write: impl FnMut(usize, &ObservationIndexEntry, ObservationRow),
) -> Result<(), E> {
    if selections.len() > MAXIMUM_OBSERVATION_SELECTIONS {
        return Err(batch_refusal(selections.len(), ObservationRefusal::TooManySelections).into());
    }
    // The check: every selection resolves, and no two name the same tap. What it resolved to is
    // kept on the stack, so the read below neither resolves again nor allocates.
    let mut resolved = [Resolved::UNUSED; MAXIMUM_OBSERVATION_SELECTIONS];
    for (position, record) in selections.iter().enumerate() {
        let selection = select(record)?;
        let found = resolve(index, &selection).map_err(|refusal| {
            E::from(ObservationReadError {
                selection: position,
                refusal,
            })
        })?;
        if resolved[..position]
            .iter()
            .any(|prior| prior.entry == found.entry && prior.tap == found.tap)
        {
            return Err(ObservationReadError {
                selection: position,
                refusal: ObservationRefusal::Duplicate,
            }
            .into());
        }
        resolved[position] = found;
    }
    for (position, found) in resolved[..selections.len()].iter().enumerate() {
        let row = read_row(index, found, &mut issued);
        write(position, &index.entries[found.entry as usize], row);
    }
    Ok(())
}

const fn batch_refusal(count: usize, refusal: ObservationRefusal) -> ObservationReadError {
    ObservationReadError {
        selection: count,
        refusal,
    }
}

/// What one selection resolved to: the index entry, the handle's position and the tap's index.
#[derive(Clone, Copy)]
struct Resolved {
    entry: u32,
    handle: u32,
    tap: u32,
    channels: ObservationReadChannels,
}

impl Resolved {
    const UNUSED: Self = Self {
        entry: u32::MAX,
        handle: u32::MAX,
        tap: u32::MAX,
        channels: ObservationReadChannels::Both,
    };
}

fn resolve(
    index: &ObservationIndex,
    selection: &ObservationSelection<'_>,
) -> Result<Resolved, ObservationRefusal> {
    let wanted = (selection.strip_id, selection.section, selection.effect_id);
    let entry = index
        .entries
        .binary_search_by(|entry| key_of(entry).cmp(&wanted))
        .map_err(|_| ObservationRefusal::UnknownAddress)?;
    let found = &index.entries[entry];
    let position = found.handle.ok_or(ObservationRefusal::Unbound)?;
    let views = index
        .handle_taps(position)
        .ok_or(ObservationRefusal::Unbound)?;
    let (tap, declared) = found
        .effect
        .observations
        .iter()
        .enumerate()
        .find(|(_, tap)| tap.id.0 == selection.tap_id)
        .ok_or(ObservationRefusal::UnknownTap)?;
    if declared.cost != ObservationCost::Resident {
        return Err(ObservationRefusal::UnsupportedTap);
    }
    if tap >= views.len() {
        return Err(ObservationRefusal::Unbound);
    }
    Ok(Resolved {
        entry: u32::try_from(entry).map_err(|_| ObservationRefusal::UnknownAddress)?,
        handle: position,
        tap: u32::try_from(tap).map_err(|_| ObservationRefusal::Unbound)?,
        channels: selection.channels,
    })
}

/// One row: the module's status rule over one locked read of the slot.
fn read_row(
    index: &ObservationIndex,
    found: &Resolved,
    issued: &mut impl FnMut(&ObservationKey<'_>) -> (u64, bool),
) -> ObservationRow {
    let entry = &index.entries[found.entry as usize];
    let tap = &entry.effect.observations[found.tap as usize];
    let (issued_tag, issued_arms) = issued(&ObservationKey {
        strip_id: &entry.strip_id,
        section: entry.section,
        effect_id: &entry.effect_id,
        tap_id: tap.id.0,
    });
    let disarmed = issued_tag != 0 && !issued_arms;
    let state = index
        .handle_taps(found.handle)
        .and_then(|views| views.get(found.tap as usize))
        .and_then(ObservationView::read_state)
        .filter(|state| state.arm_tag >= issued_tag);
    let window = state
        .filter(|state| state.armed && !disarmed)
        .and_then(|state| {
            state
                .window
                .and_then(|window| window_since_arm(window, state.arm_sample))
        });
    let status = match state {
        _ if disarmed => ObservationReadStatus::Unarmed,
        Some(state) if !state.armed => ObservationReadStatus::Unarmed,
        _ if window.is_some() => ObservationReadStatus::Ready,
        _ => ObservationReadStatus::Pending,
    };
    let (left, right) = match (status, window, found.channels) {
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Left) => {
            (Some(window.left), None)
        }
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Right) => {
            (None, Some(window.right))
        }
        (ObservationReadStatus::Ready, Some(window), ObservationReadChannels::Both) => {
            (Some(window.left), Some(window.right))
        }
        _ => (None, None),
    };
    ObservationRow {
        status,
        state,
        window,
        left,
        right,
        tap: Some(tap),
    }
}
