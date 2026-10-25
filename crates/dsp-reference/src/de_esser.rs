//! Independent offline `f64` reference for the de-esser's output law (issue #15), on the shared
//! dynamic band reference: one channel, its own detector (dual mono, no sidechain).

use crate::dynamic_band::{
    ReferenceBandTap, ReferenceDynamicBand, ReferenceDynamicBandLaw, ReferenceRbjBiquad,
    reference_level_db,
};

/// Which arm the reference renders.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceDeEsserOutput {
    /// `g x`.
    Wideband,
    /// `x - (1 - g) f`, `f` the selected filter tap.
    Split,
    /// `f` itself.
    Listen,
}

/// One de-esser channel: the selected RBJ tap, the dynamic band's downward hard-knee law and the
/// output arm.
#[derive(Clone, Debug)]
pub struct ReferenceDeEsser {
    filter: ReferenceRbjBiquad,
    band: ReferenceDynamicBand,
    output: ReferenceDeEsserOutput,
}

impl ReferenceDeEsser {
    /// A channel at rest, or `None` for a filter outside `0 < f < fs/2`, `q > 0`.
    #[must_use]
    pub fn new(
        rate_hz: f64,
        frequency_hz: f64,
        q: f64,
        tap: ReferenceBandTap,
        law: ReferenceDynamicBandLaw,
        output: ReferenceDeEsserOutput,
    ) -> Option<Self> {
        Some(Self {
            filter: ReferenceRbjBiquad::new(rate_hz, frequency_hz, q, tap)?,
            band: ReferenceDynamicBand::new(rate_hz, law),
            output,
        })
    }

    /// One sample: `(output, smoothed change in dB)`.
    pub fn process(&mut self, x: f64) -> (f64, f64) {
        let tap = self.tap(x);
        self.finish(x, tap, tap)
    }

    /// The first half of one sample: the selected filter tap of `x`, which is also the
    /// channel's own detector word.
    pub fn tap(&mut self, x: f64) -> f64 {
        self.filter.process(x)
    }

    /// The second half of one sample, from the channel's own `tap` and the detector word
    /// `detector` its gain law reads (`tap` itself for dual mono; a linked pair's blend, issue
    /// #1969): `(output, smoothed change in dB)`.
    pub fn finish(&mut self, x: f64, tap: f64, detector: f64) -> (f64, f64) {
        let change = self.band.step(reference_level_db(detector));
        let gain = 10.0_f64.powf(change / 20.0);
        let y = match self.output {
            ReferenceDeEsserOutput::Wideband => gain * x,
            ReferenceDeEsserOutput::Split => x - (1.0 - gain) * tap,
            ReferenceDeEsserOutput::Listen => tap,
        };
        (y, change)
    }
}
