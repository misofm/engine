//! The host's observation subscriptions (#1584): which effect taps a host armed and with which
//! window, kept apart from the monitor overlay (solo and subscriptions are different things).
//!
//! A subscription is host-local state. It is never saved and never in a snapshot, and it needs no
//! revision. A rebuild must not lose it: the host prepares each successor plan from the bindings
//! the transaction leaves ([`ObservationBindings::after`]), so a tap the host acknowledged is armed
//! in the successor from its first rendered block. A binding whose target the next model no longer
//! has ends, and [`ObservationChange::ended`] names it, so the host tells its subscribers.

use core::fmt;
use std::collections::TryReserveError;

use effect_contract::{ObservationCost, ObservationDescriptor};
use session::{EffectIdentity, Section, SessionModel, StableId};

use crate::effect_rows::resident_observation_tap;
use crate::registry::strip_index;

/// The longest window an `Observe` edit may ask for, in render blocks (#1584 D2). `0` asks for the
/// plan's meter window.
pub const MAXIMUM_OBSERVATION_WINDOW_BLOCKS: u32 = 4096;

/// Why an `Observe` edit was refused (#1584 D2, D3, D7a). The codes are stable; a client matches on
/// [`Self::code`]. Each refusal is the transaction's `VALIDATION_FAILED` at the edit's `edits[i]`,
/// before anything is written.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObserveError {
    /// `observation.address.unknown`: no such strip, or the strip has no console slot or insert
    /// of that ID.
    AddressUnknown,
    /// `observation.tap.unknown`: the effect declares no tap with that ID (`0` included; a `cid`
    /// effect declares none).
    TapUnknown,
    /// `observation.tap.unsupported`: the tap is declared but its cost is not resident, and V1 binds
    /// no lane for it.
    TapUnsupported,
    /// `observation.window`: a window above [`MAXIMUM_OBSERVATION_WINDOW_BLOCKS`].
    Window,
    /// `observation.window_conflict`: an arm of a tap another subscriber holds with another window.
    /// The window compared is the one asked for (`0` is one value), not the plan's resolved one.
    WindowConflict,
    /// `observation.not_subscribed`: a disarm of a tap that is not armed, or armed with another
    /// window.
    NotSubscribed,
    /// `observation.subscribers`: an arm of a tap that already has `u32::MAX` subscribers.
    Subscribers,
    /// `observation.unavailable`: the host does not serve the edit (its adapter's
    /// `OBSERVE_EDIT` is off: the browser until #1589), or the endpoint has no host to apply it.
    Unavailable,
}

impl ObserveError {
    /// The stable machine code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::AddressUnknown => "observation.address.unknown",
            Self::TapUnknown => "observation.tap.unknown",
            Self::TapUnsupported => "observation.tap.unsupported",
            Self::Window => "observation.window",
            Self::WindowConflict => "observation.window_conflict",
            Self::NotSubscribed => "observation.not_subscribed",
            Self::Subscribers => "observation.subscribers",
            Self::Unavailable => "observation.unavailable",
        }
    }
}

impl fmt::Display for ObserveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ObserveError {}

/// The tap check of an `Observe` edit against an effect's declared menu (#1584 D2): `Ok` for a
/// resident tap with ID `tap_id`.
///
/// # Errors
///
/// [`ObserveError::TapUnknown`] when the menu declares no such tap, [`ObserveError::TapUnsupported`]
/// when it is declared and not resident.
pub fn observe_tap(
    observations: &[ObservationDescriptor],
    tap_id: u32,
) -> Result<(), ObserveError> {
    let tap = observations
        .iter()
        .find(|tap| tap.id.0 == tap_id)
        .ok_or(ObserveError::TapUnknown)?;
    if tap.cost == ObservationCost::Resident {
        Ok(())
    } else {
        Err(ObserveError::TapUnsupported)
    }
}

/// The declared observation menu of the effect at `section` of strip `strip` (#1584 D2): `None`
/// when the strip has no such console slot or insert, and an empty menu for an instance with no
/// launch descriptor (a `cid` effect, an unknown one).
fn observation_menu(
    model: &SessionModel,
    strip: &StableId,
    section: &TapSection,
) -> Option<&'static [ObservationDescriptor]> {
    let owner = model.strips().find(|candidate| candidate.id == strip)?;
    let identity = match section {
        TapSection::Console(slot) => model
            .console
            .slots()
            .find(|candidate| candidate.slot == *slot)
            .map(|candidate| &candidate.identity),
        TapSection::Inserts(effect) => owner
            .inserts
            .effects
            .iter()
            .find(|candidate| candidate.id == *effect)
            .map(|candidate| &candidate.identity),
    }?;
    let EffectIdentity::Native { effect_id } = identity else {
        return Some(&[]);
    };
    Some(
        effect_compiler::launch_native_effect_registry()
            .ok()
            .and_then(|registry| registry.get_ascii(effect_id.as_str()))
            .map_or(&[], |factory| factory.descriptor().observations),
    )
}

/// The target check of an `Observe` edit against `model` (#1584 D2), the transaction's candidate
/// at the edit's position, so an edit after an `UpsertTrack` of the same transaction can address
/// the new effect: the strip, its console slot or insert, a resident tap with that ID, and a window
/// of at most [`MAXIMUM_OBSERVATION_WINDOW_BLOCKS`]. It writes nothing.
///
/// # Errors
///
/// The first [`ObserveError`] that applies, in the order address, tap, window.
pub fn observe_target(
    model: &SessionModel,
    key: &TapKey,
    window_blocks: u32,
) -> Result<(), ObserveError> {
    if strip_index(model, &key.strip).is_none() {
        return Err(ObserveError::AddressUnknown);
    }
    let menu =
        observation_menu(model, &key.strip, &key.section).ok_or(ObserveError::AddressUnknown)?;
    observe_tap(menu, key.tap_id)?;
    if window_blocks > MAXIMUM_OBSERVATION_WINDOW_BLOCKS {
        return Err(ObserveError::Window);
    }
    Ok(())
}

/// One record a transaction's `Observe` edits leave for a tap (#1584 D3): at most one per tap, the
/// net change of the table. An arm of a tap that was not armed, a disarm of a tap the transaction
/// left with no subscriber, or a re-arm with the new window when the transaction changed it (a
/// disarm and an arm with another window). Sharing an arm, or ending one subscriber of several,
/// queues nothing.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObserveWrite {
    /// The tap.
    pub key: TapKey,
    /// `true` arms (or re-arms) the tap, `false` disarms it.
    pub armed: bool,
    /// The window asked for; `0` for a disarm.
    pub window_blocks: u32,
}

/// The effect section a tap belongs to: a console slot or an insert of one strip.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TapSection {
    /// A session console slot, by its slot ID.
    Console(StableId),
    /// One of the strip's inserts, by its effect ID.
    Inserts(StableId),
}

impl TapSection {
    /// The ID of the console slot or the insert.
    #[must_use]
    pub fn id(&self) -> &StableId {
        match self {
            Self::Console(id) | Self::Inserts(id) => id,
        }
    }

    /// The control section this names.
    #[must_use]
    pub fn section(&self) -> Section {
        match self {
            Self::Console(id) => Section::Console(id.clone()),
            Self::Inserts(id) => Section::Inserts(id.clone()),
        }
    }

    /// Whether this is a console slot.
    #[must_use]
    pub const fn is_console(&self) -> bool {
        matches!(self, Self::Console(_))
    }
}

/// The stable key of one observation tap: the strip, the effect section and the effect-local tap
/// ID. It names no plan position, so it survives every rebuild while the effect exists.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TapKey {
    /// The strip (a track or a submix).
    pub strip: StableId,
    /// The console slot or insert.
    pub section: TapSection,
    /// The effect-local tap ID (`ObservationDescriptor::id`).
    pub tap_id: u32,
}

/// One armed tap, the window it was armed with and how many subscribers share the arm.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBinding {
    /// The tap.
    pub key: TapKey,
    /// The window length in blocks as it was asked for; `0` is the plan's meter window, which each
    /// plan resolves for itself.
    pub window_blocks: u32,
    /// The subscribers that share the arm (#1584 D3): each `Observe` arm adds one and each disarm
    /// removes one, and the binding goes at `0`. An absolute [`ObservationBindings::set`] (the
    /// browser's observe kinds 7 and 8) gives a new binding one and keeps an existing one's.
    pub refs: u32,
}

/// The taps a host has armed, sorted by key.
///
/// The browser's observe kinds 7 and 8 write it absolutely ([`Self::set`], [`Self::clear`]); the
/// core's `Observe` edit shares an arm among its subscribers with reference counts
/// ([`Self::observe`]).
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ObservationBindings {
    /// Sorted by key, with no duplicate key.
    bindings: Vec<ObservationBinding>,
}

/// What [`ObservationBindings::after`] returns.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationChange {
    /// The bindings whose target the next model still has.
    pub bindings: ObservationBindings,
    /// The keys of the bindings the change ended, in key order: the strip, the insert or the slot
    /// is gone, or the effect behind the ID has no resident tap with that ID.
    pub ended: Vec<TapKey>,
}

impl ObservationBindings {
    /// No binding. `const`, so a host may keep one in a `static`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    /// The bindings, sorted by key.
    #[must_use]
    pub fn bindings(&self) -> &[ObservationBinding] {
        &self.bindings
    }

    /// How many taps are armed.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Whether no tap is armed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// The window `key` is armed with, or `None` when it is not armed.
    #[must_use]
    pub fn window_of(&self, key: &TapKey) -> Option<u32> {
        self.search(key)
            .ok()
            .map(|index| self.bindings[index].window_blocks)
    }

    /// Arm `key` with `window_blocks`, replacing any earlier window. An existing binding keeps its
    /// subscriber count; a new one has one.
    pub fn set(&mut self, key: TapKey, window_blocks: u32) {
        match self.search(&key) {
            Ok(index) => self.bindings[index].window_blocks = window_blocks,
            Err(index) => self.bindings.insert(
                index,
                ObservationBinding {
                    key,
                    window_blocks,
                    refs: 1,
                },
            ),
        }
    }

    /// One `Observe` edit's subscriber change (#1584 D3): an arm with no binding binds `key` with one
    /// subscriber, an arm with the same window adds one, a disarm with the same window removes one
    /// and the binding goes at `0`. The window compared is the one asked for.
    ///
    /// # Errors
    ///
    /// [`ObserveError::WindowConflict`] for an arm of a tap bound with another window,
    /// [`ObserveError::Subscribers`] for an arm of a tap that has `u32::MAX` subscribers, and
    /// [`ObserveError::NotSubscribed`] for a disarm of a tap that is not bound or bound with another
    /// window. A refusal changes nothing.
    pub fn observe(
        &mut self,
        key: &TapKey,
        armed: bool,
        window_blocks: u32,
    ) -> Result<(), ObserveError> {
        match (self.search(key), armed) {
            (Err(index), true) => {
                self.bindings.insert(
                    index,
                    ObservationBinding {
                        key: key.clone(),
                        window_blocks,
                        refs: 1,
                    },
                );
                Ok(())
            }
            (Err(_), false) => Err(ObserveError::NotSubscribed),
            (Ok(index), _) if self.bindings[index].window_blocks != window_blocks => {
                Err(if armed {
                    ObserveError::WindowConflict
                } else {
                    ObserveError::NotSubscribed
                })
            }
            (Ok(index), true) => {
                let binding = &mut self.bindings[index];
                binding.refs = binding
                    .refs
                    .checked_add(1)
                    .ok_or(ObserveError::Subscribers)?;
                Ok(())
            }
            (Ok(index), false) => {
                let binding = &mut self.bindings[index];
                binding.refs = binding.refs.saturating_sub(1);
                if binding.refs == 0 {
                    self.bindings.remove(index);
                }
                Ok(())
            }
        }
    }

    /// The records that take this table to `after` for the taps in `touched` (#1584 D3): at most
    /// one per tap, in key order. A tap bound in both with the same window has none, whatever its
    /// subscriber count did.
    #[must_use]
    pub fn writes_to(&self, after: &Self, touched: &[TapKey]) -> Vec<ObserveWrite> {
        let mut keys: Vec<&TapKey> = touched.iter().collect();
        keys.sort_unstable();
        keys.dedup();
        keys.into_iter()
            .filter_map(|key| match (self.window_of(key), after.window_of(key)) {
                (None, Some(window)) => Some((key, true, window)),
                (Some(before), Some(window)) if before != window => Some((key, true, window)),
                (Some(_), None) => Some((key, false, 0)),
                _ => None,
            })
            .map(|(key, armed, window_blocks)| ObserveWrite {
                key: key.clone(),
                armed,
                window_blocks,
            })
            .collect()
    }

    /// Disarm `key`. Returns whether it was armed.
    pub fn clear(&mut self, key: &TapKey) -> bool {
        match self.search(key) {
            Ok(index) => {
                self.bindings.remove(index);
                true
            }
            Err(_) => false,
        }
    }

    /// Whether the tap named by its parts is armed: the strip, the effect section (a console slot
    /// when `console`, else an insert), the section's ID and the tap ID. It compares text and
    /// allocates nothing, so a caller that holds only borrowed IDs can ask.
    #[must_use]
    pub fn is_armed_by_parts(&self, strip: &str, console: bool, effect: &str, tap_id: u32) -> bool {
        self.bindings.iter().any(|binding| {
            let key = &binding.key;
            key.tap_id == tap_id
                && key.strip.as_str() == strip
                && key.section.is_console() == console
                && key.section.id().as_str() == effect
        })
    }

    fn search(&self, key: &TapKey) -> Result<usize, usize> {
        self.bindings
            .binary_search_by(|binding| binding.key.cmp(key))
    }

    /// Whether the next model still has the target of `key`: the strip, the section's instance
    /// and a resident tap of that ID.
    #[must_use]
    pub fn has_target(model: &SessionModel, key: &TapKey) -> bool {
        strip_index(model, &key.strip).is_some()
            && resident_observation_tap(model, &key.strip, &key.section.section(), key.tap_id)
                .is_some()
    }

    /// The bindings on a session whose strips and effects are `next`'s: each binding whose target
    /// `next` no longer has is dropped and its key returned in [`ObservationChange::ended`].
    ///
    /// Both lists are reserved exactly before anything is copied, so a refusal leaves `self`
    /// untouched and a caller can refuse the transaction before it commits anything.
    ///
    /// # Errors
    ///
    /// The allocation failed.
    pub fn after(&self, next: &SessionModel) -> Result<ObservationChange, TryReserveError> {
        let mut kept = Vec::new();
        kept.try_reserve_exact(self.bindings.len())?;
        let mut ended = Vec::new();
        ended.try_reserve_exact(self.bindings.len())?;
        for binding in &self.bindings {
            if Self::has_target(next, &binding.key) {
                kept.push(binding.clone());
            } else {
                ended.push(binding.key.clone());
            }
        }
        Ok(ObservationChange {
            bindings: Self { bindings: kept },
            ended,
        })
    }
}
