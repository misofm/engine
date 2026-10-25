//! Independent offline `f64` reference for one dynamic EQ band (issue #17), on the shared dynamic
//! band reference: a fixed-pole bell whose centre gain the band's gain law moves.
//!
//! The cookbook bell `H(s) = (s^2 + s A/Q + 1) / (s^2 + s/(A Q) + 1)` is
//! `1 + (A^2 - 1) B(s)`, with `B` the 0 dB-peak band-pass at the centre and quality `A Q`. The
//! band keeps `B` fixed at its static gain `A = 10^(G/40)` and scales the band by the smoothed
//! change `g`: `y = x + (A^2 g - 1) b`, `b = B x`. The detector is the 0 dB-peak band-pass at the
//! band's own quality `Q` (the bell's half-gain band, whatever the static gain), on `x` or on the
//! sidechain. One channel, dual mono.

use crate::dynamic_band::{
    ReferenceBandTap, ReferenceDynamicBand, ReferenceDynamicBandLaw, ReferenceRbjBiquad,
    reference_level_db,
};

/// One dynamic EQ band on one channel.
#[derive(Clone, Debug)]
pub struct ReferenceDynamicEqBand {
    main: ReferenceRbjBiquad,
    detector: ReferenceRbjBiquad,
    external: bool,
    band: ReferenceDynamicBand,
    static_gain: f64,
}

impl ReferenceDynamicEqBand {
    /// A band at rest with its bell at `frequency_hz`, `q` and the static `gain_db`. `external`
    /// feeds the detector's band-pass the sidechain input. `None` for a filter outside
    /// `0 < f < fs/2`, `q > 0`.
    #[must_use]
    pub fn new(
        rate_hz: f64,
        frequency_hz: f64,
        q: f64,
        gain_db: f64,
        law: ReferenceDynamicBandLaw,
        external: bool,
    ) -> Option<Self> {
        let amplitude = 10.0_f64.powf(gain_db / 40.0);
        let filter = |quality: f64| {
            ReferenceRbjBiquad::new(rate_hz, frequency_hz, quality, ReferenceBandTap::BandPass)
        };
        Some(Self {
            main: filter(q * amplitude)?,
            detector: filter(q)?,
            external,
            band: ReferenceDynamicBand::new(rate_hz, law),
            static_gain: amplitude * amplitude,
        })
    }

    /// One sample of the main input `x` and the sidechain input `side`: `(output, change in dB)`.
    pub fn process(&mut self, x: f64, side: f64) -> (f64, f64) {
        let (band, detector) = self.taps(x, side);
        self.finish(x, band, detector)
    }

    /// The first half of one sample: the bell's band-pass word and the channel's own detector
    /// word, on `x` or, external, on `side`.
    pub fn taps(&mut self, x: f64, side: f64) -> (f64, f64) {
        let band = self.main.process(x);
        let detector = self.detector.process(if self.external { side } else { x });
        (band, detector)
    }

    /// The second half of one sample, from the bell's `band` word and the detector word
    /// `detector` the gain law reads (the channel's own for dual mono; a linked pair's blend,
    /// issue #1969): `(output, change in dB)`.
    pub fn finish(&mut self, x: f64, band: f64, detector: f64) -> (f64, f64) {
        let change = self.band.step(reference_level_db(detector));
        let gain = 10.0_f64.powf(change / 20.0);
        (x + (self.static_gain * gain - 1.0) * band, change)
    }
}
