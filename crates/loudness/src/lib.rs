//! ITU-R BS.1770-5 loudness, EBU Tech 3342 loudness range and Annex-2 true peak of a rendered
//! stereo pair (issue #2115).
//!
//! The meter has two halves:
//!
//! * [`LoudnessObserver`], the **render half**, allocation-free: per 10 ms hop it sums the squared
//!   K-weighted samples of each channel and keeps the largest Annex-2 true-peak estimate, and
//!   emits one heap-free [`HopRecord`] per hop.
//! * [`LoudnessAnalysis`], the **control half**: it allocates once in `new` and turns hop records
//!   into momentary (400 ms), short-term (3 s), integrated (gated) loudness, loudness range and
//!   true peak since reset. Every logarithm is here.
//!
//! # The hop
//!
//! A hop is 10 ms, `rate / 100` samples (441, 480, 882 or 960), exact at every launch rate. Every
//! window and step of BS.1770-5, Tech 3341 and Tech 3342 is a whole number of hops, so M and S
//! update every 10 ms, and gating blocks (400 ms) and short-term blocks (3 s) step every 10 hops.
//! A coarser 100 ms hop would miss up to 50 ms of a 400 ms burst that starts off its grid, which
//! Tech 3341 cases 13 and 14 test (a -0.58 LU error against a 0.1 LU tolerance).
//!
//! The K-weighting is designed at the session rate by [`KWeighting`]; the true-peak detector is
//! the true-peak limiter's own (`true_peak_limiter::annex2`), with the same 4x table at every rate
//! (at 96 kHz Annex 2 allows 2x, and 4x is a superset).

mod analysis;
mod k_weighting;
mod observer;

pub use analysis::{
    BINS_PER_LU, BLOCK_STEP_HOPS, HISTOGRAM_BINS, HISTOGRAM_FLOOR_LUFS, LOUDNESS_FLOOR_LUFS,
    LoudnessAnalysis, MOMENTARY_HOPS, SHORT_TERM_HOPS,
};
pub use k_weighting::{
    HIGHPASS_GAIN_RATE, HIGHPASS_HZ, HIGHPASS_Q, KWeighting, SHELF_BAND_EXPONENT, SHELF_GAIN_DB,
    SHELF_HZ, SHELF_Q,
};
pub use observer::{HopRecord, LoudnessObserver};

/// The launch rates the meter measures at, Hz.
pub const SUPPORTED_RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

/// Why a meter could not be built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoudnessError {
    /// The rate is not one of [`SUPPORTED_RATES`].
    UnsupportedRate(u32),
}

impl core::fmt::Display for LoudnessError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedRate(rate) => write!(
                formatter,
                "loudness meter rate {rate} Hz is not 44100, 48000, 88200 or 96000"
            ),
        }
    }
}

impl core::error::Error for LoudnessError {}

/// `Ok` for a launch rate.
fn supported_rate(rate: u32) -> Result<(), LoudnessError> {
    if SUPPORTED_RATES.contains(&rate) {
        Ok(())
    } else {
        Err(LoudnessError::UnsupportedRate(rate))
    }
}
