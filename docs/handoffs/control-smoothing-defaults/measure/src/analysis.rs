//! Spectral measures of what a gain change adds to a signal.
//!
//! Every engine output measured here is `y = g * x`: the fader, mute and matrix ramp kernels
//! multiply and do nothing else (`measure.rs` asserts the engine's samples equal `f32(g * x)` bit
//! for bit). A slow `g` scales the programme; a fast one also *spreads* it, because the spectrum
//! of `y` is the spectrum of `x` convolved with the spectrum of `g`. The spread is the click (a
//! mute) or the zipper (a stepped fader). Every ratio below is relative to the energy of the
//! *unmodified* input `x` over the same analysis frames ("relative to the signal"):
//!
//! * **Out-of-band energy** (`oob`, band-limited material only). Energy of `y` at or above an edge
//!   the source provably does not reach (1.5 kHz for the bass, whose partials stop at 1 kHz; 1 kHz
//!   for the kick body). The same sum over `x` is the measure's floor. This is the brief's
//!   "out-of-band energy relative to the signal", exactly: everything counted is added by the gain.
//! * **Click-to-threshold ratio** (`ctr`, band-limited material only). The largest ratio, over
//!   frames and Zwicker critical bands above the out-of-band edge, of `y`'s band energy to
//!   Terhardt's threshold in quiet at [`CALIBRATION_DB_SPL`]. Above 0 dB the out-of-band click is
//!   above the absolute threshold at that playback level. It compares ~21 ms frame energies with
//!   long-tone thresholds, which over-predicts audibility of a click shorter than a frame (temporal
//!   integration), and it ignores masking by the programme: a screen that errs towards "audible".
//! * **Splatter** (`splatter`, every material, including the dense mix, which has no out-of-band
//!   region). See [`Splatter`]: the energy the gain moved more than half a critical bandwidth away
//!   from its source frequency. `hf` is the same restricted to 1.5-20 kHz and taken relative to the
//!   *programme's own* energy in 1.5-20 kHz, i.e. how the click compares with what could mask it.
//!
//! Frames are periodic-Hann, ~21-23 ms (1024 samples at 44.1/48 kHz, 2048 at 88.2/96 kHz), 75 %
//! overlap, and only 0-20 kHz counts at every rate.

use std::f64::consts::PI;

/// Upper edge of every measure: nothing above 20 kHz counts, at any sample rate.
pub const AUDIBLE_TOP_HZ: f64 = 20_000.0;

/// Playback calibration for the threshold in quiet: a signal at 0 dBFS RMS plays at this SPL.
/// Katz's K-20 monitoring calibration as his updated text states it (pink noise at -20 dBFS RMS ->
/// 83 dB SPL C-weighted per speaker) gives 103 dB; SMPTE RP 200's 85 dBC would give 105 dB.
pub const CALIBRATION_DB_SPL: f64 = 103.0;

/// Critical-band edges (Hz) as tabulated by Johnston (1988, Table II) and Painter and Spanias (2000,
/// Table 1) after Zwicker/Scharf, closed at 20 kHz; everything above is one more
/// (inaudible) band for the decomposition only.
const BARK_EDGES: [f64; 26] = [
    0.0, 100.0, 200.0, 300.0, 400.0, 510.0, 630.0, 770.0, 920.0, 1080.0, 1270.0, 1480.0, 1720.0,
    2000.0, 2320.0, 2700.0, 3150.0, 3700.0, 4400.0, 5300.0, 6400.0, 7700.0, 9500.0, 12000.0,
    15500.0, 20000.0,
];
/// Audible critical bands.
const BANDS: usize = BARK_EDGES.len() - 1;

pub fn db(ratio: f64) -> f64 {
    if ratio <= 0.0 {
        f64::NEG_INFINITY
    } else {
        10.0 * ratio.log10()
    }
}

/// Band of a frequency: `0..BANDS` audible, `BANDS` above 20 kHz.
fn band_of(f: f64) -> usize {
    if f >= AUDIBLE_TOP_HZ {
        BANDS
    } else {
        BARK_EDGES.iter().rposition(|&e| e <= f).expect("edge 0")
    }
}

/// The high-frequency region of the `hf` splatter ratio.
pub const HF_EDGE_HZ: f64 = 1500.0;

/// Terhardt's (1979) threshold in quiet, dB SPL, `f` in Hz, as restated by Painter and Spanias
/// (Proc. IEEE 2000) eq. (1).
fn threshold_in_quiet(f: f64) -> f64 {
    let k = f / 1000.0;
    3.64 * k.powf(-0.8) - 6.5 * (-0.6 * (k - 3.3) * (k - 3.3)).exp() + 1e-3 * k.powi(4)
}

/// In-place iterative radix-2 complex FFT (forward, unnormalised).
struct Fft {
    n: usize,
    cos: Vec<f64>,
    sin: Vec<f64>,
    reverse: Vec<usize>,
}

impl Fft {
    fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        let bits = n.trailing_zeros();
        let reverse = (0..n)
            .map(|i| i.reverse_bits() >> (usize::BITS - bits))
            .collect();
        let cos = (0..n / 2)
            .map(|k| (2.0 * PI * k as f64 / n as f64).cos())
            .collect();
        let sin = (0..n / 2)
            .map(|k| -(2.0 * PI * k as f64 / n as f64).sin())
            .collect();
        Self {
            n,
            cos,
            sin,
            reverse,
        }
    }

    fn forward(&self, re: &mut [f64], im: &mut [f64]) {
        let n = self.n;
        for i in 0..n {
            let j = self.reverse[i];
            if j > i {
                re.swap(i, j);
                im.swap(i, j);
            }
        }
        let mut size = 2;
        while size <= n {
            let half = size / 2;
            let stride = n / size;
            for start in (0..n).step_by(size) {
                for k in 0..half {
                    let (wr, wi) = (self.cos[k * stride], self.sin[k * stride]);
                    let (a, b) = (start + k, start + k + half);
                    let tr = re[b] * wr - im[b] * wi;
                    let ti = re[b] * wi + im[b] * wr;
                    re[b] = re[a] - tr;
                    im[b] = im[a] - ti;
                    re[a] += tr;
                    im[a] += ti;
                }
            }
            size *= 2;
        }
    }

    /// Inverse transform, normalised, via the conjugate trick.
    fn inverse(&self, re: &mut [f64], im: &mut [f64]) {
        for v in im.iter_mut() {
            *v = -*v;
        }
        self.forward(re, im);
        let scale = 1.0 / self.n as f64;
        for (r, i) in re.iter_mut().zip(im.iter_mut()) {
            *r *= scale;
            *i *= -scale;
        }
    }
}

/// Critical bandwidth in Hz at `f` Hz: Painter and Spanias (Proc. IEEE 2000) eq. (2), after Zwicker
/// and Fastl.
pub fn critical_bandwidth(f: f64) -> f64 {
    let k = f / 1000.0;
    25.0 + 75.0 * (1.0 + 1.4 * k * k).powf(0.69)
}

/// Sliding critical-band decomposition on one long FFT, for excerpts of up to `capacity` samples.
///
/// The input is cut into narrow sub-bands a quarter of a critical bandwidth wide. Each sub-band
/// `[a, b]` owns the window `[a - CB(a)/2, b + CB(b)/2]`: whatever the gain moves outside that
/// window has moved more than half a critical bandwidth away from where it was, wherever in the
/// sub-band the component sat. Unlike a fixed band partition, a partial on a band edge is not
/// "splattered" by a slow fade.
pub struct Splatter {
    fft: Fft,
    m: usize,
    /// Sub-band of each positive-frequency bin `0..=m/2`.
    subband: Vec<usize>,
    /// Kept bin range `[lo, hi]` (positive frequencies) of each sub-band's window.
    window: Vec<(usize, usize)>,
}

impl Splatter {
    /// The transform is at least `capacity` long with `margin` samples of zero padding.
    pub fn new(rate: u32, capacity: usize, margin: usize) -> Self {
        let m = (capacity + margin).next_power_of_two();
        let bin_hz = f64::from(rate) / m as f64;
        let nyquist = f64::from(rate) / 2.0;
        let mut edges = vec![0.0];
        while *edges.last().expect("edge") < AUDIBLE_TOP_HZ {
            let f = *edges.last().expect("edge");
            edges.push((f + critical_bandwidth(f) / 4.0).min(AUDIBLE_TOP_HZ));
        }
        edges.push(nyquist.max(AUDIBLE_TOP_HZ) + bin_hz);
        let subband = (0..=m / 2)
            .map(|k| {
                let f = k as f64 * bin_hz;
                edges.iter().rposition(|&e| e <= f).expect("edge 0")
            })
            .collect();
        let window = edges
            .windows(2)
            .map(|w| {
                let lo = (w[0] - critical_bandwidth(w[0]) / 2.0).max(0.0);
                let hi = w[1] + critical_bandwidth(w[1]) / 2.0;
                ((lo / bin_hz).ceil() as usize, (hi / bin_hz).floor() as usize)
            })
            .collect();
        Self {
            fft: Fft::new(m),
            m,
            subband,
            window,
        }
    }

    fn fold(&self, k: usize) -> usize {
        k.min(self.m - k)
    }

    /// `x_k` for every sub-band, zero-padded to the FFT size; they sum back to `x`.
    pub fn decompose(&self, x: &[f64]) -> Vec<Vec<f64>> {
        let mut re = vec![0.0; self.m];
        re[..x.len()].copy_from_slice(x);
        let mut im = vec![0.0; self.m];
        self.fft.forward(&mut re, &mut im);
        (0..self.window.len())
            .map(|b| {
                let mut br = vec![0.0; self.m];
                let mut bi = vec![0.0; self.m];
                for k in 0..self.m {
                    if self.subband[self.fold(k)] == b {
                        br[k] = re[k];
                        bi[k] = im[k];
                    }
                }
                self.fft.inverse(&mut br, &mut bi);
                br
            })
            .collect()
    }

    /// The splatter `c = sum_k (I - W_k)(g * x_k)`. `g` shorter than the transform is extended
    /// with its last value (the gain a settled ramp holds).
    pub fn splatter(&self, parts: &[Vec<f64>], g: &[f64]) -> Vec<f64> {
        let last = *g.last().expect("gain");
        let gain = |n: usize| if n < g.len() { g[n] } else { last };
        let mut cr = vec![0.0; self.m];
        let mut ci = vec![0.0; self.m];
        let mut re = vec![0.0; self.m];
        let mut im = vec![0.0; self.m];
        for (b, part) in parts.iter().enumerate() {
            // A sub-band the material does not reach is rounding noise; it cannot splatter.
            if part.iter().map(|v| v * v).sum::<f64>() < 1e-20 {
                continue;
            }
            for n in 0..self.m {
                re[n] = gain(n) * part[n];
                im[n] = 0.0;
            }
            self.fft.forward(&mut re, &mut im);
            let (lo, hi) = self.window[b];
            for k in 0..self.m {
                let f = self.fold(k);
                if f < lo || f > hi {
                    cr[k] += re[k];
                    ci[k] += im[k];
                }
            }
        }
        self.fft.inverse(&mut cr, &mut ci);
        cr
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Tally {
    /// Energy of `y` in the out-of-band region.
    pub oob_y: f64,
    /// Energy of `x` in the same region: the floor of the out-of-band measure.
    pub oob_x: f64,
    /// Largest out-of-band band energy of `y` over the threshold in quiet, over every frame.
    pub ctr_max: f64,
    /// Energy of the splatter `c`.
    pub splatter: f64,
    /// Energy of the splatter in 1.5-20 kHz.
    pub hf_splatter: f64,
    /// Energy of the unmodified input in 1.5-20 kHz.
    pub hf_signal: f64,
    /// Energy of the unmodified input `x`, the reference every ratio is taken against.
    pub signal: f64,
}

impl Tally {
    pub fn add(&mut self, other: Tally) {
        self.oob_y += other.oob_y;
        self.oob_x += other.oob_x;
        self.ctr_max = self.ctr_max.max(other.ctr_max);
        self.splatter += other.splatter;
        self.hf_splatter += other.hf_splatter;
        self.hf_signal += other.hf_signal;
        self.signal += other.signal;
    }
    pub fn oob_db(&self) -> f64 {
        db(self.oob_y / self.signal)
    }
    pub fn floor_db(&self) -> f64 {
        db(self.oob_x / self.signal)
    }
    pub fn ctr_db(&self) -> f64 {
        db(self.ctr_max)
    }
    pub fn splatter_db(&self) -> f64 {
        db(self.splatter / self.signal)
    }
    pub fn hf_db(&self) -> f64 {
        db(self.hf_splatter / self.hf_signal)
    }
}

/// Short-time analysis: band energies, the threshold in quiet, and the out-of-band sums.
pub struct Analyzer {
    pub rate: u32,
    pub len: usize,
    pub hop: usize,
    window: Vec<f64>,
    fft: Fft,
    band: Vec<usize>,
    bin_hz: f64,
    /// Threshold in quiet per band, as frame energy.
    quiet: [f64; BANDS],
}

impl Analyzer {
    pub fn new(rate: u32) -> Self {
        let len = if rate <= 48_000 { 1024 } else { 2048 };
        let window: Vec<f64> = (0..len)
            .map(|n| 0.5 - 0.5 * (2.0 * PI * n as f64 / len as f64).cos())
            .collect();
        let bin_hz = f64::from(rate) / len as f64;
        let band: Vec<usize> = (0..=len / 2).map(|k| band_of(k as f64 * bin_hz)).collect();
        // A signal at RMS `a` has one-sided frame energy `a^2 * sum(w^2) * len / 2`.
        let window_power: f64 = window.iter().map(|w| w * w).sum();
        let full_scale = window_power * len as f64 / 2.0;
        let mut quiet = [f64::INFINITY; BANDS];
        for (k, &b) in band.iter().enumerate() {
            let f = k as f64 * bin_hz;
            if b < BANDS && f >= 20.0 {
                let level = threshold_in_quiet(f) - CALIBRATION_DB_SPL;
                quiet[b] = quiet[b].min(full_scale * 10.0_f64.powf(level / 10.0));
            }
        }
        Self {
            rate,
            len,
            hop: len / 4,
            window,
            fft: Fft::new(len),
            band,
            bin_hz,
            quiet,
        }
    }

    pub fn samples(&self, seconds: f64) -> usize {
        (seconds * f64::from(self.rate)).round() as usize
    }

    fn power(&self, signal: &[f64], start: usize) -> Vec<f64> {
        let mut re: Vec<f64> = (0..self.len)
            .map(|n| signal[start + n] * self.window[n])
            .collect();
        let mut im = vec![0.0; self.len];
        self.fft.forward(&mut re, &mut im);
        (0..=self.len / 2)
            .map(|k| re[k] * re[k] + im[k] * im[k])
            .collect()
    }

    fn bands(&self, power: &[f64]) -> [f64; BANDS] {
        let mut e = [0.0; BANDS];
        for (k, p) in power.iter().enumerate() {
            if self.band[k] < BANDS {
                e[self.band[k]] += p;
            }
        }
        e
    }

    /// One frame: `x` the input, `y` the output, `c` the splatter.
    pub fn frame(&self, x: &[f64], y: &[f64], c: &[f64], start: usize, oob: Option<f64>) -> Tally {
        let px = self.power(x, start);
        let py = self.power(y, start);
        let pc = self.power(c, start);
        let mut t = Tally::default();
        for k in 0..=self.len / 2 {
            let f = k as f64 * self.bin_hz;
            if f >= AUDIBLE_TOP_HZ {
                break;
            }
            t.signal += px[k];
            t.splatter += pc[k];
            if f >= HF_EDGE_HZ {
                t.hf_signal += px[k];
                t.hf_splatter += pc[k];
            }
            if let Some(edge) = oob
                && f >= edge
            {
                t.oob_x += px[k];
                t.oob_y += py[k];
            }
        }
        if let Some(edge) = oob {
            let ey = self.bands(&py);
            for b in (0..BANDS).filter(|&b| BARK_EDGES[b] >= edge) {
                t.ctr_max = t.ctr_max.max(ey[b] / self.quiet[b]);
            }
        }
        t
    }

    /// Every frame whose centre lies in `[first_centre, last_centre]`, hop-spaced.
    pub fn span(
        &self,
        signals: (&[f64], &[f64], &[f64]),
        first_centre: usize,
        last_centre: usize,
        oob: Option<f64>,
    ) -> Tally {
        let (x, y, c) = signals;
        let mut tally = Tally::default();
        let mut centre = first_centre;
        while centre <= last_centre {
            let start = centre - self.len / 2;
            if start + self.len <= x.len() {
                tally.add(self.frame(x, y, c, start, oob));
            }
            centre += self.hop;
        }
        tally
    }
}

/// Raised-cosine taper of `edge` samples at both ends, so the long-FFT decomposition sees an
/// excerpt that starts and ends at zero. Analysis frames stay well inside the flat part.
pub fn taper(x: &[f32], edge: usize) -> Vec<f64> {
    let n = x.len();
    (0..n)
        .map(|i| {
            let d = i.min(n - 1 - i);
            let w = if d >= edge {
                1.0
            } else {
                0.5 - 0.5 * (PI * d as f64 / edge as f64).cos()
            };
            f64::from(x[i]) * w
        })
        .collect()
}

/// `smoothing_samples` from milliseconds by the rule this research recommends:
/// `round_half_up(ms * rate / 1000)`, evaluated in `f64` from the `f32` session value.
pub fn ms_to_samples(ms: f32, rate: u32) -> u32 {
    (f64::from(ms) * f64::from(rate) / 1000.0 + 0.5).floor() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fft_round_trips_and_keeps_parseval() {
        let fft = Fft::new(1024);
        let x: Vec<f64> = (0..1024).map(|n| ((n * 7919) % 101) as f64 / 50.0 - 1.0).collect();
        let (mut re, mut im) = (x.clone(), vec![0.0; 1024]);
        fft.forward(&mut re, &mut im);
        let energy: f64 = x.iter().map(|v| v * v).sum();
        let spectral: f64 = re.iter().zip(&im).map(|(r, i)| r * r + i * i).sum();
        assert!((spectral / 1024.0 - energy).abs() < 1e-9 * energy);
        fft.inverse(&mut re, &mut im);
        assert!(re.iter().zip(&x).all(|(a, b)| (a - b).abs() < 1e-12));
    }

    #[test]
    fn bands_sum_back_to_the_input() {
        let s = Splatter::new(48_000, 4000, 1000);
        let x: Vec<f64> = (0..4000).map(|n| ((n * 7919) % 101) as f64 / 50.0 - 1.0).collect();
        let parts = s.decompose(&x);
        for n in 0..4000 {
            let sum: f64 = parts.iter().map(|p| p[n]).sum();
            assert!((sum - x[n]).abs() < 1e-10);
        }
    }

    #[test]
    fn a_constant_gain_splatters_nothing_and_a_step_splatters_a_lot() {
        let rate = 48_000;
        let s = Splatter::new(rate, 24_000, 4000);
        let raw: Vec<f32> = (0..24_000)
            .map(|n| (2.0 * PI * 55.0 * n as f64 / f64::from(rate)).sin() as f32)
            .collect();
        let x = taper(&raw, 960);
        let parts = s.decompose(&x);
        let constant = s.splatter(&parts, &vec![0.5; 24_000]);
        let peak = constant.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        assert!(peak < 1e-9, "{peak}");
        let step: Vec<f64> = (0..24_000).map(|n| if n < 12_000 { 1.0 } else { 0.0 }).collect();
        let clicked = s.splatter(&parts, &step);
        let peak = clicked[11_000..13_000]
            .iter()
            .fold(0.0_f64, |m, v| m.max(v.abs()));
        assert!(peak > 1e-3, "{peak}");
    }

    #[test]
    fn a_slow_fade_on_a_band_edge_partial_barely_splatters() {
        // 770 Hz sits exactly on a Zwicker band edge; a sliding window must not count its fade.
        let rate = 48_000;
        let s = Splatter::new(rate, 24_000, 4000);
        let raw: Vec<f32> = (0..24_000)
            .map(|n| (2.0 * PI * 770.0 * n as f64 / f64::from(rate)).sin() as f32)
            .collect();
        let x = taper(&raw, 960);
        let parts = s.decompose(&x);
        let fade: Vec<f64> = (0..24_000)
            .map(|n| (1.0 - (n as f64 - 12_000.0) / 2400.0).clamp(0.0, 1.0))
            .collect();
        let c = s.splatter(&parts, &fade);
        let ec: f64 = c[9_000..16_000].iter().map(|v| v * v).sum();
        let ex: f64 = x[9_000..16_000].iter().map(|v| v * v).sum();
        assert!(db(ec / ex) < -45.0, "{}", db(ec / ex));
    }

    /// A steady 1 kHz sine at -83 dBFS RMS plays at 20 dB SPL under the K-20 calibration. Its
    /// critical band is 920-1080 Hz, where Terhardt's threshold is lowest at the top bin (about
    /// 3.1 dB SPL), so the click-to-threshold ratio must read about 16.9 dB.
    #[test]
    fn click_to_threshold_ratio_is_calibrated_in_spl() {
        let rate = 48_000;
        let a = Analyzer::new(rate);
        let amplitude = 10.0_f64.powf(-83.0 / 20.0) * std::f64::consts::SQRT_2;
        let y: Vec<f64> = (0..4096)
            .map(|n| amplitude * (2.0 * PI * 1000.0 * n as f64 / f64::from(rate)).sin())
            .collect();
        let silent = vec![0.0; 4096];
        let t = a.frame(&y, &y, &silent, 1024, Some(900.0));
        let top_bin_hz = (1079.0 / a.bin_hz).floor() * a.bin_hz;
        let want = 20.0 - threshold_in_quiet(top_bin_hz);
        assert!((t.ctr_db() - want).abs() < 0.2, "{} vs {want}", t.ctr_db());
    }

    #[test]
    fn threshold_in_quiet_is_lowest_near_3_to_4_khz() {
        assert!(threshold_in_quiet(3300.0) < threshold_in_quiet(1000.0));
        assert!(threshold_in_quiet(100.0) > 20.0);
    }

    #[test]
    fn rounding_rule_rounds_ties_up() {
        assert_eq!(ms_to_samples(5.0, 44_100), 221);
        assert_eq!(ms_to_samples(5.0, 48_000), 240);
        assert_eq!(ms_to_samples(20.0, 44_100), 882);
        assert_eq!(ms_to_samples(0.0, 96_000), 0);
    }
}
