//! The host's mirror of what the render plane was last told about each strip's own fader, matrix,
//! trim and polarity (#1560, decision 17 C1: a behaviour that exists on one host only is a
//! defect).
//!
//! The core never writes a record for a value the render plane already holds: a transaction that
//! rewrites an equal value is `model_only` (`classify_live_delta`, "a record is emitted only when
//! the value the render plane holds changes"). A write of an equal target is not free on the
//! render side: it restarts the lane's ramp over the new length, so a host that writes it renders
//! different bits while an earlier ramp is still in flight. A host that has no committed model
//! keeps this mirror instead and compares each command to it, by the same bits the core
//! compares: the fader's and the trim's *gain* (so `0.0` and `-0.0` dB are one value), the lowered
//! matrix's coefficients, and the polarity flag.
//!
//! The mute, the send and the VCA-reached fader keep their own mirrors (`LiveControlSoloState`,
//! `LiveRouteState`, `LiveVcaState`); this one holds the rest. A strip some VCA reaches takes its
//! fader from `LiveVcaState`, whose emitted value is the effective one; the fader kept here is
//! that of a strip no VCA reaches, where the own value is the effective one.
//!
//! This module is temporary: #1565 (D5) deletes it with the other mirrors when the browser's own
//! live-command path goes and the browser admits its edits through the committed model.
//!
//! # The transaction
//!
//! Like its siblings it carries a shadow: the first mutation of a transaction copies the entries
//! aside, [`LiveStripValues::rollback`] restores them on a refusal, and
//! [`LiveStripValues::commit`] closes the transaction. Every array is allocated at preparation;
//! nothing here allocates afterwards, and the host-web `command_submit` that drives it runs on the
//! worklet thread between quanta, never inside `process()`.

use std::collections::TryReserveError;

use builtins::{BuiltinLaneSelector, checked_fader_gain, checked_trim_gain};
use builtins_compiler::lower_matrix_or_pan;
use session::SessionModel;

use crate::solo::covers;

/// One strip's mirrored values, as the bits the render plane compares.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StripValues {
    /// The fader gain's bits per lane (`checked_fader_gain` of the own dB value).
    pub fader_gain: [u32; 2],
    /// The lowered matrix's coefficient bits, `[ll, lr, rl, rr]`.
    pub matrix: [u32; 4],
    /// The trim gain's bits per lane (`checked_trim_gain`; the polarity sign is its own value).
    pub trim_gain: [u32; 2],
    /// The polarity flag per lane.
    pub polarity: [bool; 2],
}

/// The fader gain bits a dB value compares as: the render-side gain, so `0.0` and `-0.0` are one
/// value. A value outside the domain (the host refused it before asking) compares by its own bits.
#[must_use]
pub fn fader_gain_bits(db: f32) -> u32 {
    checked_fader_gain(db).map_or_else(|_| db.to_bits(), f32::to_bits)
}

/// The trim gain bits a dB value compares as; see [`fader_gain_bits`].
#[must_use]
pub fn trim_gain_bits(db: f32) -> u32 {
    checked_trim_gain(db).map_or_else(|_| db.to_bits(), f32::to_bits)
}

/// Whether a lane selector addresses lane `lane` (0 left, 1 right).
#[must_use]
pub const fn covers_lane(lanes: BuiltinLaneSelector, lane: usize) -> bool {
    covers(lanes, lane)
}

/// The records a per-lane change mask owes: one `Both` when both lanes changed to one value, else
/// one record per changed lane, left first. Each entry names the lane whose value the record
/// carries. The core's rule (`lane_records`), so the two hosts stage the same records.
pub fn changed_lane_records(
    changed: [bool; 2],
    same_value: bool,
) -> impl Iterator<Item = (BuiltinLaneSelector, usize)> {
    crate::prepare::lane_records(changed, same_value)
}

/// Why [`LiveStripValues::try_new`] failed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveStripValuesError {
    /// An array could not be reserved.
    Allocation(TryReserveError),
    /// A strip's matrix or pan does not lower. A validated model never has one.
    Matrix,
}

/// The mirror of every strip's values, with a transaction shadow.
#[derive(Debug)]
pub struct LiveStripValues {
    values: Box<[StripValues]>,
    shadow: Box<[StripValues]>,
    open: bool,
}

impl LiveStripValues {
    /// Seed one entry per strip of `model` (a normalized model), tracks then submixes: the values
    /// preparation baked, which the render plane holds before any command.
    ///
    /// # Errors
    ///
    /// [`LiveStripValuesError::Allocation`] when an array cannot be reserved and
    /// [`LiveStripValuesError::Matrix`] when a strip's matrix does not lower.
    pub fn try_new(model: &SessionModel) -> Result<Self, LiveStripValuesError> {
        let count = model.tracks.len() + model.submixes.len();
        let mut values = Vec::new();
        values
            .try_reserve_exact(count)
            .map_err(LiveStripValuesError::Allocation)?;
        for strip in model.strips() {
            let (matrix, _) = lower_matrix_or_pan(strip.matrix_or_pan)
                .map_err(|_| LiveStripValuesError::Matrix)?;
            let lanes = [&strip.builtins.left, &strip.builtins.right];
            values.push(StripValues {
                fader_gain: [
                    fader_gain_bits(strip.fader.left_db),
                    fader_gain_bits(strip.fader.right_db),
                ],
                matrix: [matrix.ll, matrix.lr, matrix.rl, matrix.rr].map(f32::to_bits),
                trim_gain: lanes.map(|lane| trim_gain_bits(lane.trim_db)),
                polarity: lanes.map(|lane| lane.polarity_invert),
            });
        }
        let mut shadow = Vec::new();
        shadow
            .try_reserve_exact(count)
            .map_err(LiveStripValuesError::Allocation)?;
        shadow.extend_from_slice(&values);
        Ok(Self {
            values: values.into_boxed_slice(),
            shadow: shadow.into_boxed_slice(),
            open: false,
        })
    }

    /// The mirror of a plan without live controls: it holds nothing and refuses every write.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            values: Box::new([]),
            shadow: Box::new([]),
            open: false,
        }
    }

    /// One strip's values, or `None` for an index past the last strip.
    #[must_use]
    pub fn get(&self, strip: usize) -> Option<&StripValues> {
        self.values.get(strip)
    }

    /// Bytes this mirror retains, its shadow included.
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        (size_of_val(&*self.values) + size_of_val(&*self.shadow)) as u64
    }

    /// The largest single allocation this mirror made.
    #[must_use]
    pub fn largest_allocation_bytes(&self) -> u64 {
        size_of_val(&*self.values) as u64
    }

    /// Record the fader gain bits the render plane was told for the lanes `lanes` covers. `false`,
    /// changing nothing, for an unknown strip.
    pub fn set_fader_gain(&mut self, strip: usize, lanes: BuiltinLaneSelector, bits: u32) -> bool {
        self.update(strip, |entry| {
            for lane in 0..2 {
                if covers(lanes, lane) {
                    entry.fader_gain[lane] = bits;
                }
            }
        })
    }

    /// Record the lowered matrix bits the render plane was told. `false` for an unknown strip.
    pub fn set_matrix(&mut self, strip: usize, bits: [u32; 4]) -> bool {
        self.update(strip, |entry| entry.matrix = bits)
    }

    /// Record the trim gain bits the render plane was told for the lanes `lanes` covers. `false`
    /// for an unknown strip.
    pub fn set_trim_gain(&mut self, strip: usize, lanes: BuiltinLaneSelector, bits: u32) -> bool {
        self.update(strip, |entry| {
            for lane in 0..2 {
                if covers(lanes, lane) {
                    entry.trim_gain[lane] = bits;
                }
            }
        })
    }

    /// Record the polarity the render plane was told for the lanes `lanes` covers. `false` for an
    /// unknown strip.
    pub fn set_polarity(
        &mut self,
        strip: usize,
        lanes: BuiltinLaneSelector,
        inverted: bool,
    ) -> bool {
        self.update(strip, |entry| {
            for lane in 0..2 {
                if covers(lanes, lane) {
                    entry.polarity[lane] = inverted;
                }
            }
        })
    }

    /// Close the transaction: the staged records reached their cells.
    pub const fn commit(&mut self) {
        self.open = false;
    }

    /// Undo everything this transaction did. A refused submission leaves no trace.
    pub fn rollback(&mut self) {
        if !self.open {
            return;
        }
        self.values.copy_from_slice(&self.shadow);
        self.open = false;
    }

    /// Whether a transaction has mutated anything since the last commit or rollback.
    #[must_use]
    pub const fn transaction_open(&self) -> bool {
        self.open
    }

    fn update(&mut self, strip: usize, change: impl FnOnce(&mut StripValues)) -> bool {
        if strip >= self.values.len() {
            return false;
        }
        if !self.open {
            self.shadow.copy_from_slice(&self.values);
            self.open = true;
        }
        change(&mut self.values[strip]);
        true
    }
}

#[cfg(test)]
mod tests {
    use session::parse_session_json;

    use super::*;

    fn model() -> SessionModel {
        parse_session_json(include_str!(
            "../../../fixtures/session/v1/observation-frame-shape.json"
        ))
        .expect("observation fixture")
    }

    /// A rolled-back transaction restores every entry, and a committed one keeps them.
    #[test]
    fn rollback_restores_and_commit_keeps() {
        let mut state = LiveStripValues::try_new(&model()).expect("mirror");
        let seeded = *state.get(0).expect("strip 0");
        assert!(state.set_matrix(0, [1; 4]));
        assert!(state.set_polarity(0, BuiltinLaneSelector::Right, true));
        assert_eq!(state.get(0).expect("strip 0").matrix, [1; 4]);
        state.rollback();
        assert_eq!(*state.get(0).expect("strip 0"), seeded);
        assert!(state.set_trim_gain(0, BuiltinLaneSelector::Both, 7));
        state.commit();
        state.rollback();
        assert_eq!(state.get(0).expect("strip 0").trim_gain, [7, 7]);
        assert!(!state.set_matrix(usize::MAX, [0; 4]));
    }

    /// The compared value is the gain, so a dB spelling of the same gain is not a change.
    #[test]
    fn a_zero_is_one_gain() {
        assert_eq!(fader_gain_bits(0.0), fader_gain_bits(-0.0));
        assert_eq!(trim_gain_bits(0.0), trim_gain_bits(-0.0));
        assert_ne!(fader_gain_bits(-6.0), fader_gain_bits(-9.0));
    }

    /// Seeds are the session's baked values, one entry per strip.
    #[test]
    fn seeds_are_the_sessions_baked_values() {
        let model = model();
        let state = LiveStripValues::try_new(&model).expect("mirror");
        let first = model.strips().next().expect("a strip");
        let seed = state.get(0).expect("strip 0");
        assert_eq!(
            seed.fader_gain[0],
            fader_gain_bits(first.fader.left_db),
            "the fader seed is the strip's own"
        );
        assert!(
            state
                .get(model.tracks.len() + model.submixes.len())
                .is_none()
        );
    }
}
