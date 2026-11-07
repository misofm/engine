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
//! * [`MonoCollapse::enter`]`::<false>`, at the top of a dual body, takes that copy itself when the
//!   record is engaged. A dual body entered collapsed therefore never reads the frozen right
//!   channel: it reads the counterfactual dual run's state, which is what the copy writes.
//!
//! The chain (`rack::BankChain`) always takes the disengage copy before the first dual block, so
//! from the chain the dual entry is one untaken branch. The bank's own public surface does not
//! enforce that order, and before this type a caller that skipped the copy got a dual block
//! rendered over the frozen right channel. `builtins::InputStage` follows the same rule with its
//! own flag (issue #2274).

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
