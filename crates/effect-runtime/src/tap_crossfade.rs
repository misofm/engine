//! The queued tap crossfade (#2309): one model for every effect that moves a delay tap by
//! crossfading from the tap it reads to the new one.
//!
//! The delay's two lanes, the reverb's pre-delay and the reverb's thirty-two lines (as one
//! `[u32; 32]` tap set, which shares one fade) all hold the same four words:
//!
//! * `active`: the tap read when no crossfade runs, and the tap a running crossfade fades from;
//! * `transition`: the tap a running crossfade fades towards, equal to `active` when idle;
//! * `pending`: the tap the last accepted control value maps to;
//! * `remaining`: the crossfade updates left, `0` when idle.
//!
//! # Phases
//!
//! * **Idle** (`remaining == 0`, `transition == active`). When `pending != active` a crossfade is
//!   queued, and [`TapCrossfade::begin`] starts it.
//! * **Fading** (`1 <= remaining <= TRANSITION_SAMPLES`, `transition != active`). A control change
//!   only moves `pending`: it queues behind the running fade. The update that takes `remaining`
//!   to `0` makes `transition` the active tap.
//!
//! These two are every state live rendering reaches. A fade only starts towards a tap that differs
//! from the active one, so a fade between two equal taps is never reached. The fields are private:
//! only the transitions below and the one validator, [`TapCrossfade::from_words`], build a value,
//! so no other state can be represented.
//!
//! # What stays in the kernels
//!
//! The blend arithmetic, `old + alpha (new - old)` with the new tap's bits on the final update, is
//! not shared. The delay renders in chunks and accumulates `alpha` by exact `1/128` additions; the
//! reverb renders per sample and computes `alpha` in closed form. Both give the exact weight
//! `j / 128`, but in different operation orders, and this module does not change either. Each
//! kernel reads [`TapCrossfade::remaining`] and decides itself when to call
//! [`TapCrossfade::begin`]: the delay at a chunk boundary, the reverb at every sample.

/// Updates in every tap crossfade: the crossfade weight moves in exact steps of `1 / 128`.
pub const TRANSITION_SAMPLES: u32 = 128;

/// One queued tap crossfade over a tap set `T`: a single tap (`u32`) or a set of taps that fade
/// together (`[u32; N]`).
///
/// See the [module documentation](self) for the phases. `queued` caches `pending != active` while
/// idle, so a per-sample [`TapCrossfade::begin`] on a large tap set compares no taps; it is
/// `false` while fading and set again when the fade ends, so it is a function of the other four
/// words and two values with the same words compare equal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TapCrossfade<T> {
    active: T,
    transition: T,
    pending: T,
    remaining: u32,
    queued: bool,
}

impl<T: Copy + PartialEq> TapCrossfade<T> {
    /// Idle at `tap`, with nothing queued: a prepared or reset instance.
    #[must_use]
    pub const fn settled(tap: T) -> Self {
        Self {
            active: tap,
            transition: tap,
            pending: tap,
            remaining: 0,
            queued: false,
        }
    }

    /// The one validator: the crossfade a restored payload's words describe, or `None` if live
    /// rendering cannot reach it.
    ///
    /// Live rendering reaches exactly the states with `remaining <= TRANSITION_SAMPLES` in which
    /// `remaining == 0` holds exactly when `transition == active`. That excludes an idle
    /// crossfade with two different taps, and a running one between two equal taps. Each tap's
    /// own domain is the caller's to check.
    #[must_use]
    pub fn from_words(active: T, transition: T, pending: T, remaining: u32) -> Option<Self> {
        let idle = remaining == 0;
        if remaining > TRANSITION_SAMPLES || idle != (transition == active) {
            return None;
        }
        Some(Self {
            active,
            transition,
            pending,
            remaining,
            queued: idle && pending != active,
        })
    }

    /// The tap read when no crossfade runs; a running crossfade fades from it.
    #[must_use]
    #[inline(always)]
    pub const fn active(&self) -> &T {
        &self.active
    }

    /// The tap a running crossfade fades towards; [`Self::active`] when idle.
    #[must_use]
    #[inline(always)]
    pub const fn transition(&self) -> &T {
        &self.transition
    }

    /// The tap the last accepted control value maps to.
    #[must_use]
    #[inline(always)]
    pub const fn pending(&self) -> &T {
        &self.pending
    }

    /// Crossfade updates left: `0` when idle, otherwise `1..=TRANSITION_SAMPLES`.
    #[must_use]
    #[inline(always)]
    pub const fn remaining(&self) -> u32 {
        self.remaining
    }

    /// Queues `tap`: an accepted control value. It takes effect at the next
    /// [`Self::begin`] that finds the crossfade idle.
    #[inline(always)]
    pub fn queue(&mut self, tap: T) {
        self.pending = tap;
        if self.remaining == 0 {
            self.queued = tap != self.active;
        }
    }

    /// Starts the queued crossfade, if one is queued and none is running.
    #[inline(always)]
    pub fn begin(&mut self) {
        debug_assert!(self.remaining != 0 || self.queued == (self.pending != self.active));
        if self.remaining == 0 && self.queued {
            self.transition = self.pending;
            self.remaining = TRANSITION_SAMPLES;
            self.queued = false;
        }
    }

    /// Consumes `updates` crossfade updates, at most [`Self::remaining`], when a crossfade runs;
    /// nothing when idle. The last update makes the transition tap the active one.
    #[inline(always)]
    pub fn advance(&mut self, updates: u32) {
        if self.remaining != 0 {
            debug_assert!(updates <= self.remaining);
            self.remaining -= updates;
            if self.remaining == 0 {
                self.active = self.transition;
                self.queued = self.pending != self.active;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The validator accepts every reachable phase and refuses the two unreachable ones.
    ///
    /// Red for: a validator that misses either direction of `remaining == 0 <=> transition ==
    /// active` (the reverb's restore missed the second, #2309), or the length bound.
    #[test]
    fn from_words_accepts_exactly_the_reachable_phases() {
        // Idle, with or without a queued tap.
        assert_eq!(
            TapCrossfade::from_words(5_u32, 5, 5, 0),
            Some(TapCrossfade::settled(5))
        );
        assert!(TapCrossfade::from_words(5_u32, 5, 9, 0).is_some());
        // Fading, with the queued tap anywhere, at either end of the count.
        assert!(TapCrossfade::from_words(5_u32, 9, 9, 1).is_some());
        assert!(TapCrossfade::from_words(5_u32, 9, 5, TRANSITION_SAMPLES).is_some());
        // Idle between two different taps.
        assert_eq!(TapCrossfade::from_words(5_u32, 9, 9, 0), None);
        // Fading between two equal taps.
        assert_eq!(TapCrossfade::from_words(5_u32, 5, 9, 64), None);
        assert_eq!(
            TapCrossfade::from_words([5_u32, 7], [5, 7], [5, 8], 64),
            None
        );
        // A tap set fades when any tap differs.
        assert!(TapCrossfade::from_words([5_u32, 7], [5, 8], [5, 8], 64).is_some());
        // Longer than a crossfade.
        assert_eq!(
            TapCrossfade::from_words(5_u32, 9, 9, TRANSITION_SAMPLES + 1),
            None
        );
    }
}
