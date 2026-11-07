//! A bank's mono-collapse record (#1894, #973), as one type (issue #2352).
//!
//! A collapsed block (`PreparedNativeEffectBank::process_bank_mono`) advances the left channel's
//! state alone and leaves the right channel frozen at the moment the collapse engaged. Until the
//! disengage copy (`PreparedNativeEffectBank::desymmetrize_channels`) runs, the right channel's
//! state is stale, so a rest scan and a clock advance read the left channel alone, and a dual
//! body must not read the right channel at all.
//!
//! [`MonoCollapse`] keeps its word private, so the record moves only through these functions:
//!
//! * [`MonoCollapse::enter`]`::<true>`, at the top of a one-plane body, engages it;
//! * [`MonoCollapse::disengage`], the disengage copy, copies the left channel's whole per-channel
//!   state onto the right ([`CollapsibleChannels::copy_left_channel_onto_right`]) and then clears
//!   it. No other write clears it, so the record is never cleared with the right channel stale;
//! * [`MonoCollapse::enter`]`::<false>` takes that copy itself when the record is engaged. A bank
//!   calls it at the top of every entry that reads or writes channel state as real state: its
//!   dual body, before the block's automation reaches either channel, and every writer (a reset,
//!   a lane reset, a prepared target, a link retarget, a restore, a lane copy or swap, a defaults
//!   exchange). Such an entry therefore never reads the frozen right channel and never has a
//!   write it made overwritten by a later copy: it starts from the counterfactual dual run's
//!   state, which is what the copy writes.
//!
//! A whole-bank reset takes the copy too rather than clearing the record: a reset that keeps
//! parameters keeps each channel's ramps and targets, and the right channel's are the frozen ones.
//!
//! The chain (`rack::BankChain`) always takes the disengage copy before the first dual block and
//! before a carry or a lane swap, so from the chain the dual entry is one untaken branch; a writer
//! the chain calls on a collapsed bank (a lane reset, a target, a link retarget, all before the
//! block's mode is decided) takes the copy, which the next collapsed block's frozen right channel
//! never reads. The bank's own public surface does not enforce the order, and before this type a
//! caller that skipped the copy got a dual block rendered over the frozen right channel.
//!
//! `builtins::InputStage` keeps its own flag under the same rule (issue #2274): its dual body and
//! its lane import take the copy, and a reset, which rewrites both channels from one state, clears
//! the flag.

/// Whether a collapsed block has rendered since the last disengage copy. See the
/// [module documentation](self).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MonoCollapse {
    /// A collapsed block has rendered and no disengage copy has run since.
    engaged: bool,
}

/// The owner of a [`MonoCollapse`]: the effect state whose right channel a collapsed block freezes.
pub trait CollapsibleChannels {
    /// The owner's record.
    fn mono_collapse(&mut self) -> &mut MonoCollapse;

    /// Copies the left channel's **whole** per-channel render state onto the right, every lane:
    /// the body of the disengage copy, without the record. Called only by
    /// [`MonoCollapse::disengage`].
    fn copy_left_channel_onto_right(&mut self);
}

impl MonoCollapse {
    /// Not engaged: a bank is bound with two live channels.
    #[must_use]
    pub const fn new() -> Self {
        Self { engaged: false }
    }

    /// Whether the right channel is frozen: a rest scan or a clock advance reads the left channel
    /// alone while this is `true`.
    #[must_use]
    pub const fn engaged(self) -> bool {
        self.engaged
    }

    /// The top of a render body. `MONO` engages the record; a dual body on an engaged record takes
    /// the disengage copy first, so it renders over the state a never-collapsed run would hold.
    #[inline(always)]
    pub fn enter<const MONO: bool>(owner: &mut (impl CollapsibleChannels + ?Sized)) {
        if MONO {
            owner.mono_collapse().engaged = true;
        } else if owner.mono_collapse().engaged {
            Self::disengage(owner);
        }
    }

    /// The disengage copy: the left channel's whole state onto the right, then the record cleared.
    /// Always sound: while the record is engaged the counterfactual right channel is the left one,
    /// and while it is not the two channels' state is the dual run's.
    pub fn disengage(owner: &mut (impl CollapsibleChannels + ?Sized)) {
        owner.copy_left_channel_onto_right();
        owner.mono_collapse().engaged = false;
    }
}
