//! Independent offline `f64` reference for the dynamic band step shared by the de-esser (#15) and
//! the dynamic EQ (#17): an RBJ band-pass detector, the dB level, the static curve as a change, an
//! upward or downward direction, the range clamp and a branching one-pole in the dB domain.
//!
//! It imports no production code. The filters are RBJ cookbook biquads (bilinear transform with
//! the cutoff prewarped), which have the transfer function of the production TPT SVF with
//! `g = tan(pi f / fs)` and `k = 1/Q`, realised differently.

/// One RBJ cookbook biquad in transposed direct form II.
#[derive(Clone, Copy, Debug)]
pub struct ReferenceRbjBiquad {
    b: [f64; 3],
    a: [f64; 2],
    z: [f64; 2],
}

/// The two RBJ responses the dynamic band uses.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReferenceBandTap {
    /// Band-pass with a constant 0 dB peak at the centre frequency.
    BandPass,
    /// Second-order high-pass at the centre frequency with damping `1/Q`.
    HighPass,
}

impl ReferenceRbjBiquad {
    /// Designs `tap` at `frequency_hz` and `q`, or `None` outside `0 < f < fs/2`, `q > 0`.
    #[must_use]
    pub fn new(rate_hz: f64, frequency_hz: f64, q: f64, tap: ReferenceBandTap) -> Option<Self> {
        if !(frequency_hz > 0.0 && frequency_hz < 0.5 * rate_hz && q > 0.0) {
            return None;
        }
        let omega = core::f64::consts::TAU * frequency_hz / rate_hz;
        let (sine, cosine) = omega.sin_cos();
        let alpha = sine / (2.0 * q);
        let b = match tap {
            ReferenceBandTap::BandPass => [alpha, 0.0, -alpha],
            ReferenceBandTap::HighPass => {
                [(1.0 + cosine) * 0.5, -(1.0 + cosine), (1.0 + cosine) * 0.5]
            }
        };
        let a0 = 1.0 + alpha;
        Some(Self {
            b: b.map(|word| word / a0),
            a: [-2.0 * cosine / a0, (1.0 - alpha) / a0],
            z: [0.0; 2],
        })
    }

    /// Filters one sample.
    pub fn process(&mut self, x: f64) -> f64 {
        let y = self.b[0] * x + self.z[0];
        self.z[0] = self.b[1] * x - self.a[0] * y + self.z[1];
        self.z[1] = self.b[2] * x - self.a[1] * y;
        y
    }
}

/// The gain law of one dynamic band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceDynamicBandLaw {
    /// Threshold in dB.
    pub threshold_db: f64,
    /// Ratio, at least 1.
    pub ratio: f64,
    /// Soft-knee width in dB; 0 is a hard knee.
    pub knee_db: f64,
    /// The largest change in dB.
    pub range_db: f64,
    /// Attack time in ms.
    pub attack_ms: f64,
    /// Release time in ms.
    pub release_ms: f64,
    /// `true` for a boost above the threshold, `false` for a cut.
    pub upward: bool,
}

/// `20 log10(max(|d|, 1e-8))`, clamped to `[-160, 24]` dB.
#[must_use]
pub fn reference_level_db(detector: f64) -> f64 {
    (20.0 * detector.abs().max(1.0e-8).log10()).clamp(-160.0, 24.0)
}

/// The smoothed change in dB of one band, one sample at a time.
#[derive(Clone, Debug)]
pub struct ReferenceDynamicBand {
    law: ReferenceDynamicBandLaw,
    attack: f64,
    release: f64,
    gain_db: f64,
}

impl ReferenceDynamicBand {
    /// A band at rest (no change) at `rate_hz`.
    #[must_use]
    pub fn new(rate_hz: f64, law: ReferenceDynamicBandLaw) -> Self {
        let rate = |ms: f64| 1.0 - (-1.0 / (0.001 * ms * rate_hz)).exp();
        Self {
            law,
            attack: rate(law.attack_ms),
            release: rate(law.release_ms),
            gain_db: 0.0,
        }
    }

    /// The static curve's change at `level_db`, signed by the direction and clamped by the range.
    #[must_use]
    pub fn target_db(&self, level_db: f64) -> f64 {
        let law = &self.law;
        let over = level_db - law.threshold_db;
        let slope = 1.0 / law.ratio - 1.0;
        let cut = if 2.0 * over <= -law.knee_db {
            0.0
        } else if 2.0 * over > law.knee_db {
            slope * over
        } else {
            slope * (over + 0.5 * law.knee_db).powi(2) / (2.0 * law.knee_db)
        };
        let change = if law.upward { -cut } else { cut };
        change.clamp(-law.range_db, law.range_db)
    }

    /// One step from a detector level in dB: the new smoothed change in dB.
    pub fn step(&mut self, level_db: f64) -> f64 {
        let target = self.target_db(level_db);
        let rate = if target.abs() > self.gain_db.abs() {
            self.attack
        } else {
            self.release
        };
        self.gain_db += rate * (target - self.gain_db);
        self.gain_db
    }
}
