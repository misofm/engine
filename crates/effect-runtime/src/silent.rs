//! The silent fixed-point claim (#163 phase 4, #893, #1745), as one type (#2320).
//!
//! An effect whose state at rest is a fixed point of a block of exact `+0.0` input may skip its
//! kernel on such a block. The claim that the bank sits at that fixed point is a cache of the
//! effect's rest scan (`PreparedNativeEffectBank::lanes_at_rest`): a quiet block trusts it without
//! scanning, so a claim that outlives the state it was proved on freezes a lane that is not at rest
//! and moves rendered bits. [`SilentClaim`] keeps its word private, so the claim moves only through
//! these methods:
//!
//! * [`settled`](SilentClaim::settled), at the end of a rendered block, from that block's quiet
//!   test and the scan's own answer: the one place the claim is earned by observation;
//! * [`settle_skipped`](SilentClaim::settle_skipped), at a whole-bank skip
//!   (`PreparedNativeEffectBank::advance_clocks`), whose caller's mask came from the scan;
//! * [`withdraw`](SilentClaim::withdraw), on every path that writes a lane word between two blocks.
//!
//! Where the claim is trusted it is read through [`trusted`](SilentClaim::trusted), which checks it
//! against the scan in debug builds, so a writer that forgets to withdraw fails the first test that
//! renders a quiet block after it. Release builds never scan there: the claim exists to save that
//! scan.
//!
//! The compressor, the parametric EQ, the multiband compressor and the true-peak limiter use it
//! (#2378 C1 moved the limiter's own copy, #2264, onto it).

/// Whether the previous block proved the bank is at its silent fixed point. See the
/// [module documentation](self).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SilentClaim {
    /// The last settled block proved the fixed point.
    proved: bool,
}

impl SilentClaim {
    /// No claim: a claim is earned by observation, never assumed.
    #[must_use]
    pub const fn new() -> Self {
        Self { proved: false }
    }

    /// The claim a rendered block leaves: earned iff the block was `quiet` and the rest scan then
    /// answers `every` lane. `scan` is called only for a quiet block, as the effects' own `&&`
    /// evaluated it.
    #[inline(always)]
    #[must_use]
    pub fn settled(quiet: bool, scan: impl FnOnce() -> u32, every: u32) -> Self {
        Self {
            proved: quiet && scan() == every,
        }
    }

    /// Withdraws the claim. Always sound: the claim only ever buys speed.
    #[inline(always)]
    pub const fn withdraw(&mut self) {
        self.proved = false;
    }

    /// Settles the claim at a skipped block (issue #1747): a `whole`-bank skip earns it, as the
    /// rendered quiet block would, because the caller's mask came from the rest scan. A partial
    /// skip leaves it as it stood: it moved no lane off rest, and the contract's caller never
    /// passes one.
    #[inline(always)]
    pub const fn settle_skipped(&mut self, whole: bool) {
        if whole {
            self.proved = true;
        }
    }

    /// The claim, where a quiet block is about to trust it. In debug builds a standing claim is
    /// checked against the rest scan, which must answer `every` lane; release builds never call
    /// `scan`.
    ///
    /// # Panics
    ///
    /// In debug builds, when the claim stands and `scan` answers anything but `every`: a
    /// between-block writer moved a lane off rest without withdrawing the claim.
    #[inline(always)]
    #[track_caller]
    pub fn trusted(self, scan: impl FnOnce() -> u32, every: u32) -> bool {
        if self.proved {
            debug_assert_eq!(
                scan(),
                every,
                "a standing silent claim the rest scan refutes"
            );
        }
        self.proved
    }

    /// Whether the claim stands, read without the scan. For gates that compare or hash a bank's
    /// words; a fast path reads [`trusted`](Self::trusted) instead.
    #[must_use]
    pub const fn proved(self) -> bool {
        self.proved
    }
}

#[cfg(test)]
mod tests {
    use super::SilentClaim;

    /// The claim's transition table: earned only by a quiet block the scan confirms or by a
    /// whole-bank skip, kept by a partial skip, lost by a withdrawal or by a rendered block that
    /// does not earn it, and the scan never consulted for a loud block or without a claim.
    ///
    /// Test value: a `settled` that scanned a loud block (a scan per block of music, the cost the
    /// claim exists to save) or a `settle_skipped` that withdrew a standing claim on a partial mask
    /// (the limiter's copy's rule before #2378 C1) turns this red. Neither moves a bit, so no
    /// effect's fast-versus-kernel gate sees either.
    #[test]
    fn the_claim_moves_only_as_the_table_says() {
        let every = 0b1111;
        assert!(!SilentClaim::new().proved());
        assert!(SilentClaim::settled(true, || every, every).proved());
        assert!(!SilentClaim::settled(true, || 0b0111, every).proved());
        assert!(!SilentClaim::settled(false, || panic!("scanned a loud block"), every).proved());

        let mut claim = SilentClaim::new();
        claim.settle_skipped(false);
        assert!(!claim.proved(), "a partial skip earns nothing");
        claim.settle_skipped(true);
        assert!(claim.proved(), "a whole-bank skip earns the claim");
        claim.settle_skipped(false);
        assert!(claim.proved(), "a partial skip keeps a standing claim");
        assert!(claim.trusted(|| every, every));
        claim.withdraw();
        assert!(!claim.proved());
        assert!(!claim.trusted(|| panic!("scanned without a claim"), every));
    }
}
