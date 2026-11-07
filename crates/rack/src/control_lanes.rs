//! A live bank stage's control lanes and the one cache derived from them (issue #1763's quiet
//! walk; tracker #2258 GR-5).
//!
//! [`LiveControlEffectBankStage`](crate::LiveControlEffectBankStage)'s drain skips its cell walk
//! when the last walk found the bank quiet under the same snapshot publication. That skip is exact
//! only while the lanes are what that walk left: a lane moved in by a lane swap or a plan swap's
//! carry, or a carried slot installed, brings work the skipped walk would not see. The cache
//! therefore lives here, beside the lanes, behind the only mutable access to them: every
//! [`ControlLanes::lanes_mut`] forgets it, and only [`ControlLanes::walk`], which hands the walk
//! its lanes, can set it. A mutator outside the drain cannot keep a stale mark, because it has no
//! other way to reach a lane.

use core::ops::Deref;

use effect_contract::EffectControlLane;
use engine::realtime::LiveSnapshot;

/// One control lane per bank lane, and the publication of the last walk over them that found the
/// bank quiet.
pub(crate) struct ControlLanes {
    lanes: Box<[Option<EffectControlLane>]>,
    /// The snapshot publication of the last [`Self::walk`] that reported quiet, or `None`.
    /// Cleared by every [`Self::lanes_mut`]; set only by [`Self::walk`].
    quiet_at: Option<u64>,
}

impl ControlLanes {
    /// The lanes as bind leaves them, with no walk recorded.
    pub(crate) fn new(lanes: Box<[Option<EffectControlLane>]>) -> Self {
        Self {
            lanes,
            quiet_at: None,
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Whether a drain under `snapshot` may skip the walk: the last walk found the bank quiet
    /// under this same publication, and nothing has reached a lane mutably since.
    /// [`LiveSnapshot::ALL`] is not a publication and never skips.
    pub(crate) fn quiet_under(&self, snapshot: LiveSnapshot) -> bool {
        snapshot != LiveSnapshot::ALL && self.quiet_at == Some(snapshot.publication())
    }

    /// The lanes, mutably, for anything but the drain's walk (a lane swap, a carry, the process
    /// body's crossfade advance). The quiet mark is forgotten, so the next drain walks.
    pub(crate) fn lanes_mut(&mut self) -> &mut [Option<EffectControlLane>] {
        self.quiet_at = None;
        &mut self.lanes
    }

    /// The drain's walk under `snapshot`: `walk` reads every lane and returns its result and
    /// whether it found the bank quiet; the mark is that verdict, at `snapshot`'s publication.
    #[inline(always)]
    pub(crate) fn walk<R>(
        &mut self,
        snapshot: LiveSnapshot,
        walk: impl FnOnce(&mut [Option<EffectControlLane>]) -> (R, bool),
    ) -> R {
        let (result, quiet) = walk(&mut self.lanes);
        self.quiet_at = quiet.then_some(snapshot.publication());
        result
    }
    // REALTIME_POLICY_END
}

impl Deref for ControlLanes {
    type Target = [Option<EffectControlLane>];

    fn deref(&self) -> &Self::Target {
        &self.lanes
    }
}
