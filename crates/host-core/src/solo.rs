//! Live-control solo-in-place state, shared by every host that attaches live controls (issue #210).
//!
//! # Solo is 100% control plane
//!
//! The render plane already carries everything solo-in-place needs: a per-lane declicked gate
//! whose target is `0.0` or the lane's fader gain, fed by a per-track bounded queue of
//! `TrackFaderRecord` records. SIP therefore adds **no render-thread code at all**. It is a
//! state machine at command admission that composes
//!
//! ```text
//! effective_mute(strip, lane) = user_mute(strip, lane)
//!     || (any_solo && !solo_safe(strip) && !solo(strip))
//! ```
//!
//! and emits the *existing* mute records into the *existing* queues. Nothing below admission
//! changes, and the render thread cannot tell a solo-derived mute from a user mute. That is what
//! buys the whole feature its realtime properties for free: no allocation, no cross-lane audio
//! coupling (the `||` is computed over booleans on the control plane, never from audio), and the
//! existing per-sample linear declick with the caller's own `smoothing_samples`.
//!
//! [`LiveControlSoloState::effective_mute`] is that composition, and it is the **one** place it is
//! written: a host that needs an effective mute calls it rather than spelling the formula again.
//!
//! # One mute owner per strip
//!
//! The state is sized per strip (issue #1211 D2, DESIGN P7): the session's tracks, then its
//! submixes, in the order `HostLiveControlHandles::strips` lists them. Every strip's mute -- a bus
//! included -- lives here and nowhere else. Only tracks are soloable. A submix entry is
//! *solo-safe*: its solo bit never engages, it never counts toward `any_solo`, and a solo never
//! mutes it, so a soloed track stays audible through every bus and return it feeds. A bus that
//! took a solo-derived mute would silence the soloed track's own bus path.
//!
//! # Why the host has to mirror user mute
//!
//! Once solo exists, the render side's `muted` flag holds the **effective** mute, and there is no
//! host readback of it. So the host keeps [`LiveControlSoloState::user_mute`] -- the user's
//! *intent*, initialized at preparation from the compiled session's baked fader mutes and updated
//! on every admitted mute command. Un-soloing restores exactly that set, which is what makes
//! snapshot/restore correct by construction: solo and user mute never overwrite each other.
//!
//! Restore is **per lane**. `TrackFaderRecord::Mute` carries one `muted` bool, so a track whose
//! user mute is `[true, false]` needs two records, not one; the worst case for a whole session is
//! `2 * strip_count` records. [`LiveControlSoloState::strip_delta`] is what states that bound.
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
//! [`LiveControlSoloState::emitted_mute`] is the mirror of what the render plane was last told, and
//! [`LiveControlSoloState::strip_delta`] is the difference between it and the composed effective
//! mute.
//!
//! # The transaction
//!
//! Command admission is all-or-nothing across every queue in a submission. Solo state is mutated
//! while a submission is still being validated, so this type carries its own shadow: the first
//! mutation of a transaction copies the live arrays aside, [`LiveControlSoloState::rollback`]
//! restores them on any refusal, and [`LiveControlSoloState::commit`] closes the transaction once
//! the records are actually in their queues. A refused submission therefore leaves host state
//! exactly as it was -- the same contract the queues already keep.
//!
//! The shadow is allocated at preparation like every other array here. Nothing in this module
//! allocates, and nothing in it runs on a render thread.

use std::collections::TryReserveError;

use builtins::BuiltinLaneSelector;

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

/// The net mute records one strip still owes the render plane.
///
/// At most two, because a lane selector carries one `muted` bool and a strip has two lanes. Both
/// lanes changing to the *same* value is one `Both` record -- which is exactly the record an
/// explicit `mute` command with `channel = 2` lowers to, so a solo and an explicit mute put
/// the same bytes in the same queue.
pub type LiveControlMuteDelta = [Option<(BuiltinLaneSelector, bool)>; 2];

/// One strip's starting point for [`LiveControlSoloState::try_new`] (issue #1211 D2).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StripMuteSeed {
    /// `[left_mute, right_mute]` as the compiled session declares the strip's fader.
    pub mutes: [bool; 2],
    /// Whether the strip is solo-safe: never soloable and never solo-muted. Tracks are not; every
    /// submix is.
    pub solo_safe: bool,
}

/// Solo-in-place live-control state for one prepared session.
///
/// Indices are strip indices in the canonical strip order (`HostLiveControlHandles::strips`): the
/// tracks, then the submixes, each in the compiled session's normalized order -- the same order
/// every control queue uses.
#[derive(Debug)]
pub struct LiveControlSoloState {
    /// Fixed at construction; no transaction ever changes it, so it has no shadow.
    solo_safe: Box<[bool]>,
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

impl LiveControlSoloState {
    /// Allocate live-control solo state for a session with one seed per strip.
    ///
    /// `seeds[s].mutes` is `[left_mute, right_mute]` of strip `s` as the compiled session declares
    /// it -- the same words the prepared fader section bakes -- and `seeds[s].solo_safe` says
    /// whether the strip may be soloed (a track) or never (a submix). Solo starts disengaged, so the
    /// effective mute at preparation *is* the user mute, and the emitted mirror starts equal to it:
    /// the render plane has already been told exactly this much.
    ///
    /// # Errors
    ///
    /// Returns the allocator's own error when any of the seven arrays cannot be reserved. Every
    /// allocation here happens at preparation; none of them can happen again later.
    pub fn try_new(seeds: &[StripMuteSeed]) -> Result<Self, TryReserveError> {
        let solo_safe = try_boxed_map(seeds, |seed| seed.solo_safe)?;
        let solo = try_boxed(seeds.len(), false)?;
        let solo_shadow = try_boxed(seeds.len(), false)?;
        let user_mute = try_boxed_map(seeds, |seed| seed.mutes)?;
        let user_mute_shadow = try_boxed_map(seeds, |seed| seed.mutes)?;
        let emitted = try_boxed_map(seeds, |seed| seed.mutes)?;
        let emitted_shadow = try_boxed_map(seeds, |seed| seed.mutes)?;
        Ok(Self {
            solo_safe,
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

    /// Strips these live controls address: the tracks, then the submixes.
    #[must_use]
    pub const fn strip_count(&self) -> usize {
        self.solo.len()
    }

    /// The one control-plane global: is any track soloed? A solo-safe entry never counts.
    #[must_use]
    pub const fn any_solo(&self) -> bool {
        self.solo_count > 0
    }

    /// Tracks currently soloed.
    #[must_use]
    pub const fn solo_count(&self) -> u32 {
        self.solo_count
    }

    /// Whether one strip's solo bit is engaged. `false` for an index these live controls have no
    /// strip for, and always `false` for a solo-safe strip.
    #[must_use]
    pub fn solo(&self, strip: usize) -> bool {
        self.solo.get(strip).copied().unwrap_or(false)
    }

    /// Whether one strip is solo-safe (a submix): never soloable, never solo-muted. `false` for an
    /// index these live controls have no strip for.
    #[must_use]
    pub fn solo_safe(&self, strip: usize) -> bool {
        self.solo_safe.get(strip).copied().unwrap_or(false)
    }

    /// The user's mute *intent* for one lane, which solo never overwrites.
    #[must_use]
    pub fn user_mute(&self, strip: usize, lane: usize) -> bool {
        self.user_mute
            .get(strip)
            .and_then(|lanes| lanes.get(lane))
            .copied()
            .unwrap_or(false)
    }

    /// The effective mute the render plane was last told, per lane.
    #[must_use]
    pub fn emitted_mute(&self, strip: usize, lane: usize) -> bool {
        self.emitted
            .get(strip)
            .and_then(|lanes| lanes.get(lane))
            .copied()
            .unwrap_or(false)
    }

    /// `user_mute || (any_solo && !solo_safe && !soloed)` -- the effective-mute composition, and
    /// the **one** place it is written. Every host that needs an effective mute calls this.
    #[must_use]
    pub fn effective_mute(&self, strip: usize, lane: usize) -> bool {
        self.user_mute(strip, lane)
            || (self.any_solo() && !self.solo_safe(strip) && !self.solo(strip))
    }

    /// Whether a transaction has mutated anything since the last commit or rollback.
    #[must_use]
    pub const fn transaction_open(&self) -> bool {
        self.open
    }

    /// Engage or clear one track's solo bit. `false`, changing nothing, when `strip` names no strip
    /// or a solo-safe one.
    pub fn set_solo(&mut self, strip: usize, engaged: bool) -> bool {
        if strip >= self.solo.len() {
            return false;
        }
        // Source semantics: tracks only.
        // Only tracks are soloable; a submix entry is solo-safe, so its solo bit never engages.
        if self.solo_safe[strip] {
            return false;
        }
        self.shadow();
        let previous = self.solo[strip];
        self.solo[strip] = engaged;
        match (previous, engaged) {
            (false, true) => self.solo_count = self.solo_count.saturating_add(1),
            (true, false) => self.solo_count = self.solo_count.saturating_sub(1),
            _ => {}
        }
        true
    }

    /// Record the user's mute intent for the lanes `lanes` covers. `false` when `strip` is unknown.
    pub fn set_user_mute(&mut self, strip: usize, lanes: BuiltinLaneSelector, muted: bool) -> bool {
        if strip >= self.user_mute.len() {
            return false;
        }
        self.shadow();
        for lane in 0..2 {
            if covers(lanes, lane) {
                self.user_mute[strip][lane] = muted;
            }
        }
        true
    }

    /// Record that a mute record for `lanes` has been staged for this strip.
    ///
    /// The caller stages the record; this is the mirror update that keeps [`Self::strip_delta`]
    /// from staging it a second time in the same submission.
    pub fn record_emitted(&mut self, strip: usize, lanes: BuiltinLaneSelector, muted: bool) {
        if strip >= self.emitted.len() {
            return;
        }
        self.shadow();
        for lane in 0..2 {
            if covers(lanes, lane) {
                self.emitted[strip][lane] = muted;
            }
        }
    }

    /// The records this strip still owes -- never a redundant one, at most two.
    #[must_use]
    pub fn strip_delta(&self, strip: usize) -> LiveControlMuteDelta {
        let left = self.effective_mute(strip, 0);
        let right = self.effective_mute(strip, 1);
        match (
            left != self.emitted_mute(strip, 0),
            right != self.emitted_mute(strip, 1),
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

fn try_boxed_map<T>(
    seeds: &[StripMuteSeed],
    field: impl Fn(&StripMuteSeed) -> T,
) -> Result<Box<[T]>, TryReserveError> {
    let mut buffer = Vec::new();
    buffer.try_reserve_exact(seeds.len())?;
    buffer.extend(seeds.iter().map(field));
    Ok(buffer.into_boxed_slice())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Soloable track seeds only: every test written before submixes existed runs on these.
    fn state(mutes: &[[bool; 2]]) -> LiveControlSoloState {
        let seeds: Vec<StripMuteSeed> = mutes
            .iter()
            .map(|&mutes| StripMuteSeed {
                mutes,
                solo_safe: false,
            })
            .collect();
        LiveControlSoloState::try_new(&seeds).expect("solo state")
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
        assert_eq!(solo.strip_delta(1), [None, None]);
        assert_eq!(
            solo.strip_delta(0),
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
            solo.strip_delta(0),
            [Some((BuiltinLaneSelector::Right, true)), None]
        );
        solo.record_emitted(0, BuiltinLaneSelector::Right, true);
        assert_eq!(solo.strip_delta(0), [None, None]);

        // Disengaging restores the asymmetric set: left stays muted, right unmutes.
        assert!(solo.set_solo(1, false));
        assert!(!solo.any_solo());
        assert_eq!(
            solo.strip_delta(0),
            [Some((BuiltinLaneSelector::Right, false)), None]
        );
        solo.record_emitted(0, BuiltinLaneSelector::Right, false);
        assert_eq!(solo.strip_delta(0), [None, None]);
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
            solo.strip_delta(0),
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

    /// Three soloable tracks, then two solo-safe submixes: `[true, false]` and `[false, false]`.
    fn strips() -> LiveControlSoloState {
        let track = |mutes| StripMuteSeed {
            mutes,
            solo_safe: false,
        };
        let submix = |mutes| StripMuteSeed {
            mutes,
            solo_safe: true,
        };
        LiveControlSoloState::try_new(&[
            track([false; 2]),
            track([false, true]),
            track([false; 2]),
            submix([true, false]),
            submix([false; 2]),
        ])
        .expect("solo state")
    }

    /// Gate 1 (issue #1211): a bus never takes a solo-derived mute, whatever the tracks do.
    #[test]
    fn a_solo_safe_strip_keeps_its_user_mute_through_every_solo_transition() {
        let mut solo = strips();
        let bus_mutes = |solo: &LiveControlSoloState| {
            [3, 4].map(|strip| [solo.effective_mute(strip, 0), solo.effective_mute(strip, 1)])
        };
        let settled = [[true, false], [false, false]];
        assert_eq!(bus_mutes(&solo), settled);
        // Engage and release every track's solo in turn, overlapping, and back to none.
        for (track, engaged) in [
            (0, true),
            (2, true),
            (0, false),
            (1, true),
            (2, false),
            (1, false),
        ] {
            assert!(solo.set_solo(track, engaged));
            assert_eq!(bus_mutes(&solo), settled, "after solo({track}) = {engaged}");
            for strip in [3, 4] {
                assert_eq!(
                    solo.strip_delta(strip),
                    [None, None],
                    "bus {strip} owes nothing"
                );
            }
        }
        // The soloable tracks still took their solo-derived mutes on the way.
        assert!(solo.set_solo(1, true));
        assert!(solo.effective_mute(0, 0) && solo.effective_mute(2, 1));
        assert!(!solo.effective_mute(1, 0) && solo.effective_mute(1, 1));
    }

    /// Gate 1: soloing a bus is refused, moves nothing and opens no transaction.
    #[test]
    fn a_solo_safe_strip_cannot_be_soloed() {
        let mut solo = strips();
        for strip in [3, 4] {
            assert!(solo.solo_safe(strip));
            assert!(!solo.set_solo(strip, true), "bus {strip} accepted a solo");
            assert!(!solo.solo(strip));
        }
        assert!(!solo.any_solo());
        assert_eq!(solo.solo_count(), 0);
        assert!(!solo.transaction_open());
        // Nothing was solo-muted by the refused requests.
        for strip in 0..5 {
            assert_eq!(solo.strip_delta(strip), [None, None], "strip {strip}");
        }
        // A real solo still counts exactly once, and a bus request does not add to it.
        assert!(solo.set_solo(0, true));
        assert!(!solo.set_solo(4, true));
        assert_eq!(solo.solo_count(), 1);
        assert!(!solo.solo_safe(0) && !solo.solo_safe(5));
    }

    /// Gate 1: a refused batch leaves every strip -- the solo-safe ones too -- as it found it.
    #[test]
    fn rollback_restores_every_strip_including_the_solo_safe_ones() {
        let mut solo = strips();
        assert!(solo.set_solo(1, true));
        solo.record_emitted(0, BuiltinLaneSelector::Both, true);
        solo.commit();
        let snapshot = |solo: &LiveControlSoloState| {
            (0..5)
                .map(|strip| {
                    (
                        solo.solo(strip),
                        [solo.user_mute(strip, 0), solo.user_mute(strip, 1)],
                        [solo.emitted_mute(strip, 0), solo.emitted_mute(strip, 1)],
                        [solo.effective_mute(strip, 0), solo.effective_mute(strip, 1)],
                    )
                })
                .collect::<Vec<_>>()
        };
        let before = snapshot(&solo);

        // The refused batch: it touches both buses and two tracks, and tries to solo a bus.
        assert!(solo.set_user_mute(3, BuiltinLaneSelector::Both, false));
        assert!(solo.set_user_mute(4, BuiltinLaneSelector::Right, true));
        solo.record_emitted(4, BuiltinLaneSelector::Right, true);
        solo.record_emitted(3, BuiltinLaneSelector::Left, false);
        assert!(!solo.set_solo(3, true));
        assert!(solo.set_solo(2, true));
        assert!(solo.set_solo(1, false));
        assert!(solo.set_user_mute(0, BuiltinLaneSelector::Left, true));
        solo.rollback();

        assert!(!solo.transaction_open());
        assert_eq!(solo.solo_count(), 1);
        assert_eq!(snapshot(&solo), before);
        assert!(solo.solo_safe(3) && solo.solo_safe(4) && !solo.solo_safe(2));
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
