//! Block P's band split (listening Amendment 1): a zero-phase, linear-phase FIR low-pass that
//! divides a polarity flip's change into the part inside the bass's own band and the part above
//! the measures' out-of-band edge.
//!
//! The flip's output is `y = g x`, and its change `d = y - x` holds two things at once: near each
//! partial, the dip of the gain through zero (with the phase reversal it carries), and far from
//! every partial, the click of the gain's two slope corners (FINDINGS 9.2). The bass's partials
//! stop at 1 kHz and every click measure of section 1 reads from 1.5 kHz, so a low-pass `L` that
//! passes 0-1 kHz and stops from 1.5 kHz gives `L d`, the in-band change (the dip), and `d - L d`,
//! the out-of-band change (the click). `L d + (d - L d) = d`: nothing is added or lost.
//!
//! The filter is a Kaiser-windowed ideal low-pass [Kaiser 1974, as given in Oppenheim and Schafer,
//! *Discrete-Time Signal Processing*, 3rd ed., section 7.6]: `beta = 0.1102 (A - 8.7)` for a
//! stopband attenuation `A` above 50 dB, order `M = ceil((A - 8) / (2.285 dw))` for a transition
//! `dw` in radians per sample, cutoff in the middle of the transition. It is applied centred
//! (zero phase), in `f64`, with zeros outside the signal, so `L d` lines up with `d` sample for
//! sample. It runs off the engine, after the render, on the harness's own data only.

use std::f64::consts::PI;

pub struct BandSplit {
    /// `2 half + 1` symmetric taps, centre at `half`, summing to exactly one after normalising.
    taps: Vec<f64>,
    half: usize,
}

impl BandSplit {
    /// Passes up to `pass_hz` and stops from `stop_hz` by about `attenuation_db` (over 50 dB).
    pub fn new(rate: u32, pass_hz: f64, stop_hz: f64, attenuation_db: f64) -> Self {
        assert!(0.0 < pass_hz && pass_hz < stop_hz && attenuation_db > 50.0);
        let fs = f64::from(rate);
        let transition = 2.0 * PI * (stop_hz - pass_hz) / fs;
        let order = ((attenuation_db - 8.0) / (2.285 * transition)).ceil() as usize;
        let half = order.div_ceil(2);
        let beta = 0.1102 * (attenuation_db - 8.7);
        let cutoff = PI * (pass_hz + stop_hz) / fs;
        let window_norm = bessel_i0(beta);
        // One side, mirrored, so the taps are exactly symmetric.
        let side: Vec<f64> = (0..=half)
            .map(|m| {
                let ideal = if m == 0 {
                    cutoff / PI
                } else {
                    (cutoff * m as f64).sin() / (PI * m as f64)
                };
                let r = m as f64 / half as f64;
                ideal * bessel_i0(beta * (1.0 - r * r).sqrt()) / window_norm
            })
            .collect();
        let mut taps: Vec<f64> = side.iter().rev().chain(&side[1..]).copied().collect();
        let sum: f64 = taps.iter().sum();
        for t in &mut taps {
            *t /= sum;
        }
        Self { taps, half }
    }

    /// Number of taps (odd).
    #[cfg(test)]
    pub fn taps(&self) -> usize {
        self.taps.len()
    }

    /// The low part of `x`, centred: `low[i] = sum_k h[k] x[i + k - half]`, zeros outside `x`.
    pub fn low(&self, x: &[f64]) -> Vec<f64> {
        let n = x.len();
        (0..n)
            .map(|i| {
                let first = self.half.saturating_sub(i);
                let end = (n + self.half - i).min(self.taps.len());
                (first..end)
                    .map(|k| self.taps[k] * x[i + k - self.half])
                    .sum()
            })
            .collect()
    }

    /// The (real, zero-phase) amplitude response at `hz`.
    #[cfg(test)]
    pub fn response(&self, rate: u32, hz: f64) -> f64 {
        let w = 2.0 * PI * hz / f64::from(rate);
        self.taps
            .iter()
            .enumerate()
            .map(|(k, &h)| h * (w * (k as f64 - self.half as f64)).cos())
            .sum()
    }
}

/// The modified Bessel function of the first kind, order zero, by its power series.
fn bessel_i0(x: f64) -> f64 {
    let quarter = x * x / 4.0;
    let (mut sum, mut term, mut k) = (1.0, 1.0, 1.0);
    loop {
        term *= quarter / (k * k);
        sum += term;
        if term < sum * 1e-17 {
            return sum;
        }
        k += 1.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: u32 = 48_000;

    fn split() -> BandSplit {
        BandSplit::new(RATE, 1000.0, 1500.0, crate::stimuli::SPLIT_ATTENUATION_DB)
    }

    /// The split passes the bass's whole band unchanged and stops the measures' out-of-band region:
    /// within 2e-5 dB of unity to 1 kHz and at least 115 dB down from 1.5 kHz to Nyquist (5 Hz
    /// grid; on a 0.5 Hz grid the design measures 1.12e-5 dB and -119.4 dB, 751 taps). A wrong cutoff (in hertz where radians are meant, or at an edge instead of the middle
    /// of the transition) or a short or badly windowed kernel leaves either the dip filtered or the
    /// click in the in-band stimulus, and fails here.
    #[test]
    fn the_split_passes_the_bass_band_and_stops_the_out_of_band_region() {
        let s = split();
        assert_eq!(s.taps() % 2, 1);
        for k in 0..=200 {
            let hz = f64::from(k) * 5.0;
            let gain_db = 20.0 * s.response(RATE, hz).abs().log10();
            assert!(gain_db.abs() < 2e-5, "passband {hz} Hz: {gain_db} dB");
        }
        for k in 300..=4800 {
            let hz = f64::from(k) * 5.0;
            let gain_db = 20.0 * s.response(RATE, hz).abs().log10();
            assert!(gain_db < -115.0, "stopband {hz} Hz: {gain_db} dB");
        }
    }

    /// The low part is aligned with its input sample for sample (zero phase): a 440 Hz tone comes
    /// out in place, away from the ends. A causal application (no centring) delays it by `half`
    /// samples and fails.
    #[test]
    fn the_low_part_is_centred_on_its_input() {
        let s = split();
        let x: Vec<f64> = (0..8000)
            .map(|i| (2.0 * PI * 440.0 * f64::from(i) / f64::from(RATE)).sin())
            .collect();
        let low = s.low(&x);
        for i in s.taps()..x.len() - s.taps() {
            assert!((low[i] - x[i]).abs() < 1e-6, "sample {i}");
        }
    }
}
