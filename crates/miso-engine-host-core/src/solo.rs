//! Console solo-in-place state, shared by every host that attaches a live console (issue #210).
//!
//! # Solo is 100% control plane
//!
//! The render plane already carries everything solo-in-place needs: a per-lane declicked gate
//! whose target is `0.0` or the lane's fader gain, fed by a per-track bounded queue of
//! `TrackFaderRecord` records. SIP therefore adds **no render-thread code at all**. It is a
//! state machine at command admission that composes
//!
//! ```text
//! effective_mute(track, lane) = user_mute(track, lane) || (any_solo && !solo(track))
//! ```
//!
//! and emits the *existing* mute records into the *existing* queues. Nothing below admission
//! changes, and the render thread cannot tell a solo-derived mute from a user mute. That is what
//! buys the whole feature its realtime properties for free: no allocation, no cross-lane audio
//! coupling (the `||` is computed over booleans on the control plane, never from audio), and the
//! existing per-sample linear declick with the caller's own `smoothing_samples`.
//!
//! # Why the host has to mirror user mute
//!
//! Once solo exists, the render side's `muted` flag holds the **effective** mute, and there is no
//! host readback of it. So the host keeps [`ConsoleSoloState::user_mute`] -- the user's *intent*,
//! initialized at preparation from the compiled session's baked fader mutes and updated on every
//! admitted mute command. Un-soloing restores exactly that set, which is what makes
//! snapshot/restore correct by construction: solo and user mute never overwrite each other.
//!
//! Restore is **per lane**. `TrackFaderRecord::Mute` carries one `muted` bool, so a track whose
//! user mute is `[true, false]` needs two records, not one; the worst case for a whole console is
//! `2 * track_count` records. [`ConsoleSoloState::track_delta`] is what states that bound.
//!
//! # Never emit a redundant mute record
//!
//! Re-muting a lane that is already *settled* muted is not free and not invisible. The fader
//! stage's `set_mute` unconditionally retargets, so a redundant `set_mute(true)` with
//! `smoothing_samples > 0` restarts a ramp whose step is `0.0`, which drives the block through the
//! ramp kernel (multiply by the current gain) instead of the settled kernel (`fill(0.0)`). For a
//! negative input that is the difference between an exact `+0.0` and a `-0.0` -- **digest
//! visible**. So the emission rule is *not* an optimization:
//!
//! > a solo-derived record is emitted for exactly those lanes whose effective mute **changed**.
//!
//! [`ConsoleSoloState::emitted_mute`] is the mirror of what the render plane was last told, and
//! [`ConsoleSoloState::track_delta`] is the difference between it and the composed effective mute.
//!
//! # The transaction
//!
//! Command admission is all-or-nothing across every queue in a submission. Solo state is mutated
//! while a submission is still being validated, so this type carries its own shadow: the first
//! mutation of a transaction copies the live arrays aside, [`ConsoleSoloState::rollback`] restores
//! them on any refusal, and [`ConsoleSoloState::commit`] closes the transaction once the records
//! are actually in their queues. A refused submission therefore leaves host state exactly as it
//! was -- the same contract the queues already keep.
//!
//! The shadow is allocated at preparation like every other array here. Nothing in this module
//! allocates, and nothing in it runs on a render thread.

use std::collections::TryReserveError;

use miso_engine_builtins::BuiltinLaneSelector;

/// Whether a lane selector addresses one lane index.
///
/// `BuiltinLaneSelector::covers` is private to its own crate; this is the same two-line rule and
/// is exercised by every test in this module.
const fn covers(lanes: BuiltinLaneSelector, lane: usize) -> bool {
    matches!(
        (lanes, lane),
        (BuiltinLaneSelector::Left, 0)
            | (BuiltinLaneSelector::Right, 1)
            | (BuiltinLaneSelector::Both, _)
    )
}

/// The net mute records one track still owes the render plane.
///
/// At most two, because a lane selector carries one `muted` bool and a track has two lanes. Both
/// lanes changing to the *same* value is one `Both` record -- which is exactly the record an
/// explicit `mute` command with `channel = 2` lowers to, so a soloed console and an explicitly
/// muted one put the same bytes in the same queue.
pub type ConsoleMuteDelta = [Option<(BuiltinLaneSelector, bool)>; 2];

/// Solo-in-place console state for one prepared session.
///
/// Track indices are the canonical track order (`HostConsoleHandlesV1::tracks`), which is the
/// compiled session's normalized order and the same order every queue and meter slot uses.
#[derive(Debug)]
pub struct ConsoleSoloState {
    solo: Box<[bool]>,
    user_mute: Box<[[bool; 2]]>,
    emitted: Box<[[bool; 2]]>,
    solo_count: u32,
    solo_shadow: Box<[bool]>,
    user_mute_shadow: Box<[[bool; 2]]>,
    emitted_shadow: Box<[[bool; 2]]>,
    solo_count_shadow: u32,
    open: bool,
}

impl ConsoleSoloState {
    /// Allocate console solo state for a session whose baked per-lane fader mutes are `mutes`.
    ///
    /// `mutes[t]` is `[left_mute, right_mute]` of track `t` as the compiled session declares it --
    /// the same words `track_parameters` bakes into the prepared fader section. Solo starts
    /// disengaged, so the effective mute at preparation *is* the user mute, and the emitted mirror
    /// starts equal to it: the render plane has already been told exactly this much.
    ///
    /// # Errors
    ///
    /// Returns the allocator's own error when any of the six arrays cannot be reserved. Every
    /// allocation here happens at preparation; none of them can happen again later.
    pub fn try_new(mutes: &[[bool; 2]]) -> Result<Self, TryReserveError> {
        let solo = try_boxed(mutes.len(), false)?;
        let solo_shadow = try_boxed(mutes.len(), false)?;
        let user_mute = try_boxed_from(mutes)?;
        let user_mute_shadow = try_boxed_from(mutes)?;
        let emitted = try_boxed_from(mutes)?;
        let emitted_shadow = try_boxed_from(mutes)?;
        Ok(Self {
            solo,
            user_mute,
            emitted,
            solo_count: 0,
            solo_shadow,
            user_mute_shadow,
            emitted_shadow,
            solo_count_shadow: 0,
            open: false,
        })
    }

    /// Tracks this console addresses.
    #[must_use]
    pub const fn track_count(&self) -> usize {
        self.solo.len()
    }

    /// The one control-plane global: is any track soloed?
    #[must_use]
    pub const fn any_solo(&self) -> bool {
        self.solo_count > 0
    }

    /// Tracks currently soloed.
    #[must_use]
    pub const fn solo_count(&self) -> u32 {
        self.solo_count
    }

    /// Whether one track's solo bit is engaged. `false` for an index this console has no track for.
    #[must_use]
    pub fn solo(&self, track: usize) -> bool {
        self.solo.get(track).copied().unwrap_or(false)
    }

    /// The user's mute *intent* for one lane, which solo never overwrites.
    #[must_use]
    pub fn user_mute(&self, track: usize, lane: usize) -> bool {
        self.user_mute
            .get(track)
            .and_then(|lanes| lanes.get(lane))
            .copied()
            .unwrap_or(false)
    }

    /// The effective mute the render plane was last told, per lane.
    #[must_use]
    pub fn emitted_mute(&self, track: usize, lane: usize) -> bool {
        self.emitted
            .get(track)
            .and_then(|lanes| lanes.get(lane))
            .copied()
            .unwrap_or(false)
    }

    /// `user_mute || (any_solo && !my_solo)` -- the composition, in one place.
    #[must_use]
    pub fn effective_mute(&self, track: usize, lane: usize) -> bool {
        self.user_mute(track, lane) || (self.any_solo() && !self.solo(track))
    }

    /// Whether a transaction has mutated anything since the last commit or rollback.
    #[must_use]
    pub const fn transaction_open(&self) -> bool {
        self.open
    }

    /// Engage or clear one track's solo bit. `false` when `track` names no track.
    pub fn set_solo(&mut self, track: usize, engaged: bool) -> bool {
        if track >= self.solo.len() {
            return false;
        }
        self.shadow();
        let previous = self.solo[track];
        self.solo[track] = engaged;
        match (previous, engaged) {
            (false, true) => self.solo_count = self.solo_count.saturating_add(1),
            (true, false) => self.solo_count = self.solo_count.saturating_sub(1),
            _ => {}
        }
        true
    }

    /// Record the user's mute intent for the lanes `lanes` covers. `false` when `track` is unknown.
    pub fn set_user_mute(&mut self, track: usize, lanes: BuiltinLaneSelector, muted: bool) -> bool {
        if track >= self.user_mute.len() {
            return false;
        }
        self.shadow();
        for lane in 0..2 {
            if covers(lanes, lane) {
                self.user_mute[track][lane] = muted;
            }
        }
        true
    }

    /// Record that a mute record for `lanes` has been staged for this track.
    ///
    /// The caller stages the record; this is the mirror update that keeps [`Self::track_delta`]
    /// from staging it a second time in the same submission.
    pub fn record_emitted(&mut self, track: usize, lanes: BuiltinLaneSelector, muted: bool) {
        if track >= self.emitted.len() {
            return;
        }
        self.shadow();
        for lane in 0..2 {
            if covers(lanes, lane) {
                self.emitted[track][lane] = muted;
            }
        }
    }

    /// The records this track still owes -- never a redundant one, at most two.
    #[must_use]
    pub fn track_delta(&self, track: usize) -> ConsoleMuteDelta {
        let left = self.effective_mute(track, 0);
        let right = self.effective_mute(track, 1);
        match (
            left != self.emitted_mute(track, 0),
            right != self.emitted_mute(track, 1),
        ) {
            (false, false) => [None, None],
            (true, false) => [Some((BuiltinLaneSelector::Left, left)), None],
            (false, true) => [Some((BuiltinLaneSelector::Right, right)), None],
            (true, true) if left == right => [Some((BuiltinLaneSelector::Both, left)), None],
            (true, true) => [
                Some((BuiltinLaneSelector::Left, left)),
                Some((BuiltinLaneSelector::Right, right)),
            ],
        }
    }

    /// Close the transaction: the staged records reached their queues.
    pub const fn commit(&mut self) {
        self.open = false;
    }

    /// Undo everything this transaction did. A refused submission leaves no trace.
    pub fn rollback(&mut self) {
        if !self.open {
            return;
        }
        self.solo.copy_from_slice(&self.solo_shadow);
        self.user_mute.copy_from_slice(&self.user_mute_shadow);
        self.emitted.copy_from_slice(&self.emitted_shadow);
        self.solo_count = self.solo_count_shadow;
        self.open = false;
    }

    /// Take the transaction shadow, once, before the first mutation of a submission.
    fn shadow(&mut self) {
        if self.open {
            return;
        }
        self.solo_shadow.copy_from_slice(&self.solo);
        self.user_mute_shadow.copy_from_slice(&self.user_mute);
        self.emitted_shadow.copy_from_slice(&self.emitted);
        self.solo_count_shadow = self.solo_count;
        self.open = true;
    }
}

fn try_boxed<T: Clone>(count: usize, value: T) -> Result<Box<[T]>, TryReserveError> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(count)?;
    buffer.resize(count, value);
    Ok(buffer.into_boxed_slice())
}

fn try_boxed_from(values: &[[bool; 2]]) -> Result<Box<[[bool; 2]]>, TryReserveError> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(values.len())?;
    buffer.extend_from_slice(values);
    Ok(buffer.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(mutes: &[[bool; 2]]) -> ConsoleSoloState {
        ConsoleSoloState::try_new(mutes).expect("solo state")
    }

    /// Solo `S` composes to exactly "mute everything outside `S`", and nothing else moves.
    #[test]
    fn engaging_solo_mutes_the_complement_and_only_the_complement() {
        let mut solo = state(&[[false; 2]; 4]);
        assert!(!solo.any_solo());
        assert!(solo.set_solo(1, true));
        assert!(solo.any_solo());
        for track in 0..4 {
            let expected = track != 1;
            assert_eq!(
                solo.effective_mute(track, 0),
                expected,
                "track {track} left"
            );
            assert_eq!(
                solo.effective_mute(track, 1),
                expected,
                "track {track} right"
            );
        }
        // One `Both` record per muted track; the soloed track owes nothing.
        assert_eq!(solo.track_delta(1), [None, None]);
        assert_eq!(
            solo.track_delta(0),
            [Some((BuiltinLaneSelector::Both, true)), None]
        );
    }

    /// A track whose user mute is asymmetric needs two records to restore, not one.
    #[test]
    fn per_lane_user_mute_restores_as_two_records() {
        let mut solo = state(&[[true, false], [false, false]]);
        assert!(solo.set_solo(1, true));
        // Track 0's left was already muted; only its right lane changed.
        assert_eq!(
            solo.track_delta(0),
            [Some((BuiltinLaneSelector::Right, true)), None]
        );
        solo.record_emitted(0, BuiltinLaneSelector::Right, true);
        assert_eq!(solo.track_delta(0), [None, None]);

        // Disengaging restores the asymmetric set: left stays muted, right unmutes.
        assert!(solo.set_solo(1, false));
        assert!(!solo.any_solo());
        assert_eq!(
            solo.track_delta(0),
            [Some((BuiltinLaneSelector::Right, false)), None]
        );
        solo.record_emitted(0, BuiltinLaneSelector::Right, false);
        assert_eq!(solo.track_delta(0), [None, None]);
    }

    /// Both lanes changing to *different* values is the only two-record case.
    #[test]
    fn a_two_record_delta_is_exactly_the_disagreeing_case() {
        let mut solo = state(&[[false; 2]; 2]);
        // The render plane was last told `[unmuted, muted]` ...
        solo.record_emitted(0, BuiltinLaneSelector::Right, true);
        // ... and the user's intent is now the exact opposite. Both lanes changed, to values that
        // disagree, so one `Both` record cannot carry it.
        assert!(solo.set_user_mute(0, BuiltinLaneSelector::Left, true));
        assert_eq!(
            solo.track_delta(0),
            [
                Some((BuiltinLaneSelector::Left, true)),
                Some((BuiltinLaneSelector::Right, false)),
            ]
        );
    }

    /// User mute and solo are separate states; neither overwrites the other.
    #[test]
    fn mute_while_soloed_survives_the_un_solo() {
        let mut solo = state(&[[false; 2]; 3]);
        assert!(solo.set_solo(0, true));
        assert!(solo.set_user_mute(0, BuiltinLaneSelector::Both, true));
        assert!(
            solo.effective_mute(0, 0),
            "a soloed track can still be muted"
        );
        assert!(solo.set_solo(0, false));
        assert!(
            solo.effective_mute(0, 0),
            "the user mute outlives the solo it was set under"
        );
        assert!(!solo.effective_mute(1, 0), "and nothing else is muted");
    }

    /// `solo_count` is incremental and idempotent under a repeated set.
    #[test]
    fn solo_count_tracks_engaged_bits_exactly() {
        let mut solo = state(&[[false; 2]; 3]);
        assert!(solo.set_solo(0, true));
        assert!(solo.set_solo(0, true));
        assert_eq!(solo.solo_count(), 1);
        assert!(solo.set_solo(2, true));
        assert_eq!(solo.solo_count(), 2);
        assert!(solo.set_solo(0, false));
        assert!(solo.set_solo(0, false));
        assert_eq!(solo.solo_count(), 1);
        assert!(solo.any_solo());
        assert!(solo.set_solo(2, false));
        assert!(!solo.any_solo());
    }

    /// The transactional contract: a rollback restores every word, including `solo_count`.
    #[test]
    fn rollback_restores_every_word() {
        let mut solo = state(&[[true, false], [false, false], [false, true]]);
        assert!(solo.set_solo(1, true));
        solo.record_emitted(0, BuiltinLaneSelector::Both, true);
        solo.commit();

        let before: Vec<[bool; 2]> = (0..3)
            .map(|track| [solo.user_mute(track, 0), solo.user_mute(track, 1)])
            .collect();
        let emitted: Vec<[bool; 2]> = (0..3)
            .map(|track| [solo.emitted_mute(track, 0), solo.emitted_mute(track, 1)])
            .collect();

        assert!(!solo.transaction_open());
        assert!(solo.set_solo(2, true));
        assert!(solo.set_user_mute(0, BuiltinLaneSelector::Both, false));
        solo.record_emitted(2, BuiltinLaneSelector::Left, true);
        assert!(solo.transaction_open());
        assert_eq!(solo.solo_count(), 2);
        solo.rollback();

        assert!(!solo.transaction_open());
        assert_eq!(solo.solo_count(), 1);
        assert!(solo.solo(1) && !solo.solo(2));
        for track in 0..3 {
            assert_eq!(
                [solo.user_mute(track, 0), solo.user_mute(track, 1)],
                before[track]
            );
            assert_eq!(
                [solo.emitted_mute(track, 0), solo.emitted_mute(track, 1)],
                emitted[track]
            );
        }
    }

    /// An out-of-range track is refused rather than silently folded onto track 0.
    #[test]
    fn an_unknown_track_is_refused() {
        let mut solo = state(&[[false; 2]; 2]);
        assert!(!solo.set_solo(2, true));
        assert!(!solo.set_user_mute(7, BuiltinLaneSelector::Both, true));
        assert!(!solo.any_solo());
    }
}
