//! The per-track channel-symmetry witness: is this track's left plane bit-identical work to its
//! right plane?
//!
//! # What this is for
//!
//! Mono-collapse (the later phase) processes **one** plane for a track whose two channels are
//! doing identical work, and duplicates the result at the fader/matrix seam. That is only sound
//! when every stage *upstream of the seam* would have computed the same bits on both channels.
//! This module is the decision infrastructure for that: an authoritative, event-maintained bit
//! per track saying "the two channels are still doing identical work". **Nothing reads it to
//! decide anything rendered.** It is control-plane state and a query surface, and it is
//! deliberately inert.
//!
//! # The five terms, and why the witness is a set rather than a `bool`
//!
//! A `bool` cannot say *why* a track declined, and the phases that follow have to distinguish the
//! reasons: one term is recomputed by a recompile, one is monotone within a plan, one is
//! reversible block to block. So the witness is a set of named terms and `eligible` is their
//! conjunction.
//!
//! | term | set by | cleared by |
//! |---|---|---|
//! | [`SOURCE`](ChannelSymmetryWitnessV1::SOURCE) | preparation: the track's two channels read one source channel, or a one-channel source | a recompile with a different source mapping |
//! | [`DESIGNED`](ChannelSymmetryWitnessV1::DESIGNED) | preparation: every designed per-lane word the stage's kernel reads compares bit-equal between the channels | a recompile with asymmetric parameters |
//! | [`RESTORED`](ChannelSymmetryWitnessV1::RESTORED) | restore: the left and right payload sections compared byte-equal | a restore whose sections differ |
//! | [`LIVE`](ChannelSymmetryWitnessV1::LIVE) | preparation | an admitted record that writes one channel's upstream word |
//! | [`UNBYPASSED`](ChannelSymmetryWitnessV1::UNBYPASSED) | preparation | a live bypass on an upstream stage; **restored** when it is lifted |
//!
//! # The hook rule is structural, not a list of kinds
//!
//! The rule is *"every record admitted onto a live-console queue declares what it does to an
//! upstream per-lane word"*, and it is carried by [`LiveConsoleRecordV1`] rather than by an
//! `if` over today's record kinds. A new live-console record type -- the builtins trim/polarity
//! liveness drain, an automation span, anything a later issue adds -- cannot be drained into a
//! witness-carrying stage without implementing that trait, and the trait's two obligations are
//! exactly the two facts the witness needs: which side of the seam the record's stage sits on,
//! and what the record does to the two channels. Enumerating kinds would have made every future
//! kind a silent hole; this makes it a compile error.
//!
//! # The two seams this phase deliberately leaves open
//!
//! Both are marked here rather than in the crates that will grow them, because the obligation is
//! this module's to state:
//!
//! 1. **Builtins liveness.** `polarity_invert`, `trim_db`, `hpf_hz` and `lpf_hz` are `PreparedOnly`
//!    today, so the input-builtins stage has no queue and no drain, and its witness is decided
//!    once at preparation. When a live trim/polarity record type arrives it must implement
//!    [`LiveConsoleRecordV1`] with `SEAM = UpstreamOfSeam`, and its drain must call
//!    [`ChannelSymmetryWitnessV1::admit`] exactly as `EffectControlLane::stage` does. The trait
//!    bound is what makes that an obligation rather than a reminder.
//! 2. **Session automation spans.** A prepared automation span carries a
//!    [`ParameterChannel`](crate::ParameterChannel) exactly as a live record does, so the rule is
//!    already written: [`ParameterChannel::writes_one_channel`]. What does not exist yet is the
//!    admission point -- a compiled session's spans are not drained through any queue -- so when
//!    one appears, the span source becomes a [`LiveConsoleRecordV1`] implementor and nothing else
//!    changes.
//!
//! # The static-bypass convention, and its asymmetry
//!
//! `UNBYPASSED` is seeded from the **prepared** bypass ([`EffectControlLane::new`]) and then
//! maintained by the drain, so a session that declares an effect bypassed declines that lane from
//! the moment the plan exists rather than only after a `Bypass(true)` record arrives -- but only
//! where a live-console channel exists to hold the term, because a console-free plan builds no
//! [`EffectControlLane`] at all and its witness is the designed-word comparison alone
//! (`miso_engine_rack::EffectBankStage::lane_symmetry`, `runtime::NodeKind::Effect`). The
//! asymmetry is deliberate and safe in this phase (nothing reads the witness to decide anything
//! rendered) and it is a **seam the collapse must close**: a statically bypassed stage is a dry
//! shunt that copies both planes, so a phase that collapses on this witness has to decide the term
//! for console-free plans too rather than inherit an unconditional `true`.
//!
//! # Why the seam side is a `const` on the record type
//!
//! Fader, mute, pan and matrix are **seam-side by design**: the collapse duplicates the single
//! plane *into* them, so their per-channel words are free to differ and must not gate anything.
//! That is not an opinion a drain should re-derive per record; it is a property of the record
//! type, so it is a `const` and the seam-side arm compiles away.

use crate::{EffectControlRecordV1, ParameterChannel};

/// Which side of the fader/matrix seam a live-console record's stage sits on.
///
/// The seam is the earliest genuinely cross-channel operation in the strip -- the 2x2 matrix --
/// and the fader immediately before it. Everything before that pair is per-channel arithmetic
/// that a collapsed track would run once; everything from the fader on reads the duplicated
/// plane and may legitimately differ between the channels.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SeamSideV1 {
    /// The record writes a word a collapsed track would have computed once. It gates collapse.
    UpstreamOfSeam,
    /// The record writes a fader, mute, pan or matrix word. It never gates collapse.
    SeamSide,
}

/// What one admitted live-console record does to the two channels' agreement.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SymmetryEventV1 {
    /// The record leaves the channels doing identical work.
    ///
    /// A `ParameterChannel::Both` retarget is this case and not a hedge: both channels' ramps are
    /// given the same target over the same window at the same block boundary, so they advance
    /// through bit-identical values. Collapse survives both-channel automation.
    Preserve,
    /// The record writes one channel's word and not the other's.
    Desymmetrize,
    /// The record sets this stage's live bypass to the carried value.
    Bypass(bool),
}

/// The channel-symmetry witness for one track, or for one stage of one track.
///
/// A stage-level witness carries the terms that stage can speak to and leaves the rest set; a
/// track-level witness is the conjunction ([`and`](Self::and)) of its stages' witnesses with the
/// structural terms preparation decided. That is why the type is one type and not two: the
/// combinator is the whole relationship.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChannelSymmetryWitnessV1(u8);

impl ChannelSymmetryWitnessV1 {
    /// The track's two channels are fed by one source channel (or by a one-channel source).
    pub const SOURCE: u8 = 1 << 0;
    /// Every designed per-lane word this stage's kernel reads compares bit-equal between the
    /// channels.
    pub const DESIGNED: u8 = 1 << 1;
    /// No admitted record has written an upstream per-lane word for one channel only.
    pub const LIVE: u8 = 1 << 2;
    /// No upstream stage of this track is live-bypassed.
    pub const UNBYPASSED: u8 = 1 << 3;
    /// Every restored state payload's left and right sections compared byte-equal.
    pub const RESTORED: u8 = 1 << 4;

    /// Every term, in declaration order, with its stable name. Evidence and diagnosis only.
    pub const TERMS: [(u8, &'static str); 5] = [
        (Self::SOURCE, "source"),
        (Self::DESIGNED, "designed"),
        (Self::LIVE, "live"),
        (Self::UNBYPASSED, "unbypassed"),
        (Self::RESTORED, "restored"),
    ];

    const ALL: u8 = Self::SOURCE | Self::DESIGNED | Self::LIVE | Self::UNBYPASSED | Self::RESTORED;

    /// Every term holds: this track (or stage) is collapse-eligible.
    pub const SYMMETRIC: Self = Self(Self::ALL);

    /// No term holds. The conservative value for a stage that has not derived a witness from its
    /// kernel's read surface, which is what makes an unclassified stage decline rather than
    /// silently claim eligibility.
    pub const DECLINED: Self = Self(0);

    /// A witness with exactly `terms` set.
    #[must_use]
    pub const fn from_terms(terms: u8) -> Self {
        Self(terms & Self::ALL)
    }

    /// A witness with every term set except those in `terms`.
    #[must_use]
    pub const fn symmetric_except(terms: u8) -> Self {
        Self(Self::ALL & !terms)
    }

    /// The set of terms that hold.
    #[must_use]
    pub const fn terms(self) -> u8 {
        self.0
    }

    /// The set of terms that do **not** hold. Empty exactly when [`eligible`](Self::eligible).
    #[must_use]
    pub const fn declined(self) -> u8 {
        Self::ALL & !self.0
    }

    /// Whether every term holds.
    #[must_use]
    pub const fn eligible(self) -> bool {
        self.0 == Self::ALL
    }

    /// Whether one named term holds.
    #[must_use]
    pub const fn holds(self, term: u8) -> bool {
        self.0 & term == term
    }

    /// Set or clear one named term.
    pub const fn set(&mut self, term: u8, value: bool) {
        if value {
            self.0 |= term & Self::ALL;
        } else {
            self.0 &= !term;
        }
    }

    /// Clear one named term.
    pub const fn clear(&mut self, term: u8) {
        self.0 &= !term;
    }

    /// The conjunction: a term holds in the result exactly when it holds in both operands.
    ///
    /// This is how a track's witness is built out of its stages', and a cohort's out of its
    /// lanes': a single declining stage declines the track, and a single declining lane declines
    /// the bank.
    #[must_use]
    pub const fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Apply one admitted record to this witness, by the one structural rule.
    ///
    /// Seam-side records are a compile-time no-op: the collapse duplicates the plane *into* the
    /// fader and the matrix, so their per-channel words never gate it.
    pub fn admit<R: LiveConsoleRecordV1>(&mut self, record: &R) {
        if R::SEAM == SeamSideV1::SeamSide {
            return;
        }
        match record.symmetry_event() {
            SymmetryEventV1::Preserve => {}
            SymmetryEventV1::Desymmetrize => self.clear(Self::LIVE),
            SymmetryEventV1::Bypass(value) => self.set(Self::UNBYPASSED, !value),
        }
    }
}

impl Default for ChannelSymmetryWitnessV1 {
    /// [`DECLINED`](Self::DECLINED): a witness nobody has spoken for claims nothing.
    fn default() -> Self {
        Self::DECLINED
    }
}

/// Every record type that may be admitted onto a live-console queue declares its effect on the
/// channel-symmetry witness.
///
/// # The obligation, and why it is a trait
///
/// The de-symmetrising surface is not "the kinds someone remembered": it is *every admission that
/// writes an upstream per-lane word*. A drain that folds records into a witness takes
/// `R: LiveConsoleRecordV1`, so a record type that has not answered the two questions cannot be
/// drained there at all. That is the whole mechanism -- the future kinds this has to cover
/// (builtins trim/polarity liveness, automation spans) are covered because the compiler will not
/// let them past, not because a list was kept up to date.
pub trait LiveConsoleRecordV1: Copy {
    /// Which side of the fader/matrix seam the stage this record addresses sits on.
    const SEAM: SeamSideV1;

    /// What admitting this record does to the two channels' agreement.
    fn symmetry_event(&self) -> SymmetryEventV1;
}

impl ParameterChannel {
    /// Whether a write addressed to this channel leaves the other channel's word behind.
    ///
    /// [`Both`](ParameterChannel::Both) is the preserving case: one record, one target, one ramp
    /// window, applied to both channels at one block boundary.
    #[must_use]
    pub const fn writes_one_channel(self) -> bool {
        match self {
            Self::Left | Self::Right => true,
            Self::Both => false,
        }
    }
}

impl LiveConsoleRecordV1 for EffectControlRecordV1 {
    /// Every prepared effect instance sits in `simd1`, `dynamic` or `simd2` -- all three racks are
    /// upstream of the fader (`TrackStage` order), so every record on this queue is upstream.
    const SEAM: SeamSideV1 = SeamSideV1::UpstreamOfSeam;

    fn symmetry_event(&self) -> SymmetryEventV1 {
        // Exhaustive, with no wildcard arm on purpose: a new variant is a compile error here,
        // which is the structural half of the hook rule.
        match *self {
            Self::Parameter { channel, .. } => {
                if channel.writes_one_channel() {
                    SymmetryEventV1::Desymmetrize
                } else {
                    SymmetryEventV1::Preserve
                }
            }
            // A subscription changes what is *read* after the block, never what the block
            // renders, so it cannot move a designed word.
            Self::Observe { .. } => SymmetryEventV1::Preserve,
            Self::Bypass(value) => SymmetryEventV1::Bypass(value),
        }
    }
}

/// Whether one restored state payload's two channel sections are byte-equal.
///
/// This is the restore half of the witness, and it is a plain byte comparison on purpose: every
/// payload word is little-endian and every `f32` is stored as its raw `to_bits`
/// (`miso_engine_effect_runtime::state_payload`), so byte equality of the sections **is** bitwise
/// equality of the two channels' words, `-0.0` included. The two sections are always the same
/// length (`StateLayout::lane_words` is one number for both), so a length mismatch is a corrupt
/// payload and declines.
///
/// Off the render thread, at restore time, once per restored instance or bank lane.
#[must_use]
pub fn payload_sections_agree(left: &[u8], right: &[u8]) -> bool {
    left.len() == right.len() && left == right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_declined_witness_names_every_term_it_is_missing() {
        let witness = ChannelSymmetryWitnessV1::DECLINED;
        assert!(!witness.eligible());
        assert_eq!(witness.declined(), ChannelSymmetryWitnessV1::ALL);
        for (term, _) in ChannelSymmetryWitnessV1::TERMS {
            assert!(!witness.holds(term));
        }
    }

    #[test]
    fn every_single_missing_term_declines_the_conjunction() {
        for (term, name) in ChannelSymmetryWitnessV1::TERMS {
            let witness = ChannelSymmetryWitnessV1::symmetric_except(term);
            assert!(!witness.eligible(), "{name} did not gate eligibility");
            assert_eq!(witness.declined(), term);
            assert!(
                ChannelSymmetryWitnessV1::SYMMETRIC.eligible(),
                "the all-terms value must be eligible"
            );
        }
    }

    #[test]
    fn the_conjunction_declines_when_either_operand_declines() {
        let symmetric = ChannelSymmetryWitnessV1::SYMMETRIC;
        let no_live = ChannelSymmetryWitnessV1::symmetric_except(ChannelSymmetryWitnessV1::LIVE);
        assert!(symmetric.and(symmetric).eligible());
        assert!(!symmetric.and(no_live).eligible());
        assert!(!no_live.and(symmetric).eligible());
        assert_eq!(
            symmetric.and(no_live).declined(),
            ChannelSymmetryWitnessV1::LIVE
        );
    }

    #[test]
    fn a_left_channel_parameter_desymmetrises_and_a_both_channel_one_does_not() {
        let left = EffectControlRecordV1::Parameter {
            parameter_index: 3,
            channel: ParameterChannel::Left,
            value: 0.5,
        };
        let right = EffectControlRecordV1::Parameter {
            parameter_index: 3,
            channel: ParameterChannel::Right,
            value: 0.5,
        };
        let both = EffectControlRecordV1::Parameter {
            parameter_index: 3,
            channel: ParameterChannel::Both,
            value: 0.5,
        };
        assert_eq!(left.symmetry_event(), SymmetryEventV1::Desymmetrize);
        assert_eq!(right.symmetry_event(), SymmetryEventV1::Desymmetrize);
        assert_eq!(both.symmetry_event(), SymmetryEventV1::Preserve);

        let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
        witness.admit(&both);
        assert!(witness.eligible(), "a both-channel command must preserve");
        witness.admit(&left);
        assert!(!witness.eligible());
        assert_eq!(witness.declined(), ChannelSymmetryWitnessV1::LIVE);
    }

    #[test]
    fn a_subscription_never_moves_the_witness() {
        let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
        witness.admit(&EffectControlRecordV1::Observe {
            tap_index: 0,
            armed: true,
            window_blocks: 4,
        });
        assert!(witness.eligible());
    }

    #[test]
    fn bypass_is_reversible_and_asymmetric_parameters_are_not() {
        let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
        witness.admit(&EffectControlRecordV1::Bypass(true));
        assert_eq!(
            witness.declined(),
            ChannelSymmetryWitnessV1::UNBYPASSED,
            "a live bypass declines the cohort while it is on"
        );
        witness.admit(&EffectControlRecordV1::Bypass(false));
        assert!(witness.eligible(), "lifting the bypass re-earns the term");

        witness.admit(&EffectControlRecordV1::Parameter {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 1.0,
        });
        witness.admit(&EffectControlRecordV1::Bypass(true));
        witness.admit(&EffectControlRecordV1::Bypass(false));
        assert!(
            !witness.eligible(),
            "an asymmetric parameter write is not undone by bypass traffic"
        );
    }

    #[test]
    fn a_seam_side_record_never_moves_the_witness() {
        #[derive(Clone, Copy)]
        struct SeamRecord;
        impl LiveConsoleRecordV1 for SeamRecord {
            const SEAM: SeamSideV1 = SeamSideV1::SeamSide;
            fn symmetry_event(&self) -> SymmetryEventV1 {
                // Deliberately the worst answer: the seam-side const must win regardless.
                SymmetryEventV1::Desymmetrize
            }
        }
        let mut witness = ChannelSymmetryWitnessV1::SYMMETRIC;
        witness.admit(&SeamRecord);
        assert!(witness.eligible());
    }

    #[test]
    fn payload_sections_agree_is_a_bitwise_comparison() {
        assert!(payload_sections_agree(&[1, 2, 3, 4], &[1, 2, 3, 4]));
        assert!(!payload_sections_agree(&[1, 2, 3, 4], &[1, 2, 3, 5]));
        assert!(!payload_sections_agree(&[1, 2, 3], &[1, 2, 3, 4]));
        // +0.0 and -0.0 differ, which a float comparison would have missed.
        assert!(!payload_sections_agree(
            &0.0_f32.to_bits().to_le_bytes(),
            &(-0.0_f32).to_bits().to_le_bytes()
        ));
    }
}
