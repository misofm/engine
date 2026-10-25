//! The one read of observation windows by stable address (issue #1586 D3, D4).
//!
//! A host reads the latest window of up to [`MAXIMUM_OBSERVATION_SELECTIONS`] observation taps,
//! each named by its strip ID, its section (a console slot or an insert), the console slot's or
//! insert's ID and the effect-local tap ID. The read never consumes anything: it takes each tap's
//! slot through [`ObservationReader::read_state`] and never calls
//! [`ObservationReader::acknowledge`], whose word belongs to the meter fold.
//!
//! # The index holds positions, not readers (#1586a, root ruling on finding 1)
//!
//! [`ObservationIndex`] is built once per plan on the control thread. Each entry is one effect
//! instance's stable key, the position of its [`EffectObservationHandle`] in the plan's handle list
//! (`None` for an effect that declares no tap) and its descriptor. The read takes that list as an
//! argument and reads through it. A carried plan swap exchanges reader sets between two plans'
//! handles at the same positions (`pair_carried_observations`), so the positions stay right and
//! the read follows the slot that renders on; an index that held copies of the readers would keep
//! reading the successor's fresh slot, which nothing publishes to after a carry.
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
use engine::realtime::{ObservationState, ObservationWindow};

use crate::meters::window_since_arm;
use crate::{EffectObservationHandle, LiveEffectRack};

#[cfg(doc)]
use engine::realtime::ObservationReader;

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

/// A second entry named a key an earlier one named.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DuplicateObservationKey;

/// One plan's effect instances, sorted by `(strip, section, effect)`, so a selection resolves by a
/// binary search and a read allocates nothing.
#[derive(Debug, Default)]
pub struct ObservationIndex {
    entries: Box<[ObservationIndexEntry]>,
}

impl ObservationIndex {
    /// Sort `entries` by key and keep them. Allocates only when `entries` has spare capacity (the
    /// conversion to a boxed slice shrinks it). Control thread.
    ///
    /// # Errors
    ///
    /// Two entries with the same key.
    pub fn new(mut entries: Vec<ObservationIndexEntry>) -> Result<Self, DuplicateObservationKey> {
        entries.sort_unstable_by(|left, right| key_of(left).cmp(&key_of(right)));
        if entries
            .windows(2)
            .any(|pair| key_of(&pair[0]) == key_of(&pair[1]))
        {
            return Err(DuplicateObservationKey);
        }
        Ok(Self {
            entries: entries.into_boxed_slice(),
        })
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

    /// The heap the index holds: its entry table and the IDs it copies. `None` on overflow.
    #[must_use]
    pub fn retained_bytes(&self) -> Option<u64> {
        let table = u64::try_from(self.entries.len())
            .ok()?
            .checked_mul(u64::try_from(size_of::<ObservationIndexEntry>()).ok()?)?;
        self.entries.iter().try_fold(table, |total, entry| {
            total.checked_add(u64::try_from(entry.strip_id.len() + entry.effect_id.len()).ok()?)
        })
    }
}

fn key_of(entry: &ObservationIndexEntry) -> (&str, LiveEffectRack, &str) {
    (&entry.strip_id, entry.section, &entry.effect_id)
}

/// Read one batch into `rows`, one row per selection in order (#1586 D3).
///
/// `handles` is the plan's handle list the index was built from, as it is now: after a carried
/// swap it holds the carried readers. `issued` returns the tag of the last record the control plane
/// issued for a tap and whether that record arms it (`(0, false)` for none). The whole batch is
/// checked before any row is written (D4). Allocates nothing and never acknowledges.
///
/// # Errors
///
/// The first refusal, in selection order; `rows` is untouched.
pub fn read_into(
    index: &ObservationIndex,
    handles: &[EffectObservationHandle],
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
        handles,
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
    handles: &[EffectObservationHandle],
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
        let found = resolve(index, handles, &selection).map_err(|refusal| {
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
        let row = read_row(index, handles, found, &mut issued);
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
    handles: &[EffectObservationHandle],
    selection: &ObservationSelection<'_>,
) -> Result<Resolved, ObservationRefusal> {
    let wanted = (selection.strip_id, selection.section, selection.effect_id);
    let entry = index
        .entries
        .binary_search_by(|entry| key_of(entry).cmp(&wanted))
        .map_err(|_| ObservationRefusal::UnknownAddress)?;
    let found = &index.entries[entry];
    let position = found.handle.ok_or(ObservationRefusal::Unbound)?;
    // A handle at another address means the list is not the one the index was built from.
    let handle = handles
        .get(position as usize)
        .filter(|handle| {
            handle.track_id == found.strip_id
                && handle.address.rack == found.section
                && handle.effect_id == found.effect_id
        })
        .ok_or(ObservationRefusal::Unbound)?;
    let (tap, declared) = handle
        .descriptor
        .observations
        .iter()
        .enumerate()
        .find(|(_, tap)| tap.id.0 == selection.tap_id)
        .ok_or(ObservationRefusal::UnknownTap)?;
    if declared.cost != ObservationCost::Resident {
        return Err(ObservationRefusal::UnsupportedTap);
    }
    if tap >= handle.readers.len() {
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
    handles: &[EffectObservationHandle],
    found: &Resolved,
    issued: &mut impl FnMut(&ObservationKey<'_>) -> (u64, bool),
) -> ObservationRow {
    let entry = &index.entries[found.entry as usize];
    let handle = &handles[found.handle as usize];
    let tap = &handle.descriptor.observations[found.tap as usize];
    let (issued_tag, issued_arms) = issued(&ObservationKey {
        strip_id: &entry.strip_id,
        section: entry.section,
        effect_id: &entry.effect_id,
        tap_id: tap.id.0,
    });
    let disarmed = issued_tag != 0 && !issued_arms;
    let state = handle.readers[found.tap as usize]
        .read_state()
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
