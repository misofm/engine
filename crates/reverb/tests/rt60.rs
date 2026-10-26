//! Reverberation time per octave band against the design prediction (the note's test 1).

mod common;

use common::{DAMPING, DECAY, PRE_DELAY, SIZE, defaults, impulse_response};
use dsp_reference::{ReferenceReverbParameters, reference_reverb_rt60_seconds};

/// A complex number, enough for a Butterworth band-pass design.
#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    fn add(self, o: Self) -> Self {
        Self::new(self.re + o.re, self.im + o.im)
    }
    fn sub(self, o: Self) -> Self {
        Self::new(self.re - o.re, self.im - o.im)
    }
    fn mul(self, o: Self) -> Self {
        Self::new(
            self.re * o.re - self.im * o.im,
            self.re * o.im + self.im * o.re,
        )
    }
    fn scale(self, k: f64) -> Self {
        Self::new(self.re * k, self.im * k)
    }
    fn div(self, o: Self) -> Self {
        let d = o.re * o.re + o.im * o.im;
        Self::new(
            (self.re * o.re + self.im * o.im) / d,
            (self.im * o.re - self.re * o.im) / d,
        )
    }
    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
    fn sqrt(self) -> Self {
        let r = self.abs();
        let re = ((r + self.re) / 2.0).sqrt();
        let im = ((r - self.re) / 2.0).sqrt();
        Self::new(re, if self.im < 0.0 { -im } else { im })
    }
}

/// One second-order section `(1 - z^-2) / (1 + a1 z^-1 + a2 z^-2)`, times `gain`.
#[derive(Clone, Copy, Debug)]
struct Section {
    gain: f64,
    a1: f64,
    a2: f64,
}

/// The third-order Butterworth band-pass with edges `low` and `high` Hz: the bilinear design with
/// pre-warped edges (what `scipy.signal.butter(3, [low, high], btype="band", fs=fs)` designs),
/// as three sections, unit gain at the centre `t0^2 = t_lo t_hi`.
fn butterworth_band(low: f64, high: f64, fs: f64) -> [Section; 3] {
    let t_lo = math::tan(core::f64::consts::PI * low / fs);
    let t_hi = math::tan(core::f64::consts::PI * high / fs);
    let bandwidth = t_hi - t_lo;
    let centre_sq = t_lo * t_hi;
    let one = Complex::new(1.0, 0.0);
    // Upper-half-plane band-pass poles from the prototype poles at 120 and 180 degrees.
    // Each prototype pole `p` maps to the two roots of `s^2 - p B s + t0^2`. The 120-degree pole
    // gives two poles (their conjugates come from the 240-degree pole); the real pole -1 gives one
    // conjugate pair, of which the upper member is kept. Each kept pole and its conjugate are one
    // section.
    let band_poles = |angle: f64| {
        let p = Complex::new(math::cos(angle.to_radians()), math::sin(angle.to_radians()));
        let pb = p.scale(bandwidth);
        let root = pb.mul(pb).sub(Complex::new(4.0 * centre_sq, 0.0)).sqrt();
        [pb.add(root).scale(0.5), pb.sub(root).scale(0.5)]
    };
    let [a, b] = band_poles(120.0);
    let [c, d] = band_poles(180.0);
    assert!(c.im.abs() > 0.0 && (c.im + d.im).abs() < 1e-12 * c.im.abs());
    let poles = [a, b, if c.im > 0.0 { c } else { d }];
    let omega0 = 2.0 * math::atan(centre_sq.sqrt());
    let z_inverse = Complex::new(math::cos(omega0), -math::sin(omega0));
    let mut sections = [Section {
        gain: 1.0,
        a1: 0.0,
        a2: 0.0,
    }; 3];
    let mut response = one;
    for (section, s) in sections.iter_mut().zip(&poles) {
        let z = one.add(*s).div(one.sub(*s));
        section.a1 = -2.0 * z.re;
        section.a2 = z.re * z.re + z.im * z.im;
        let z2 = z_inverse.mul(z_inverse);
        let numerator = one.sub(z2);
        let denominator = one
            .add(z_inverse.scale(section.a1))
            .add(z2.scale(section.a2));
        response = response.mul(numerator.div(denominator));
    }
    sections[0].gain = 1.0 / response.abs();
    sections
}

/// Runs `x` through the sections forward from zero state (transposed direct form II).
fn filter(sections: &[Section; 3], x: &[f32]) -> Vec<f64> {
    let mut y: Vec<f64> = x.iter().map(|v| f64::from(*v)).collect();
    for section in sections {
        let (mut s1, mut s2) = (0.0_f64, 0.0_f64);
        for v in &mut y {
            let input = *v * section.gain;
            let output = input + s1;
            s1 = -section.a1 * output + s2;
            s2 = -input - section.a2 * output;
            *v = output;
        }
    }
    y
}

/// The fit window `[i0, i1)` on the EDC (dB, relative to its first value): from the later of
/// -5 dB and 0.2 s (past the mixing time), over 40 dB (T40).
fn fit_window(edc: &[f64], fs: f64) -> (usize, usize) {
    let first_5 = edc
        .iter()
        .position(|v| *v <= -5.0)
        .expect("EDC reaches -5 dB");
    let start = first_5.max((0.2 * fs).floor() as usize);
    let end = start
        + edc[start..]
            .iter()
            .position(|v| *v <= edc[start] - 40.0)
            .expect("EDC falls 40 dB after the start");
    (start, end)
}

/// Schroeder backward integration and the least-squares fit: `T = -60 / slope`.
fn decay_time(energy: &[f64], fs: f64) -> f64 {
    let mut edc = vec![0.0_f64; energy.len()];
    let mut sum = 0.0;
    for n in (0..energy.len()).rev() {
        sum += energy[n];
        edc[n] = sum;
    }
    let total = edc[0];
    for v in &mut edc {
        *v = 10.0 * math::log10(*v / total);
    }
    let (start, end) = fit_window(&edc, fs);
    let count = (end - start) as f64;
    let (mut st, mut sy, mut stt, mut sty) = (0.0, 0.0, 0.0, 0.0);
    for (n, y) in edc.iter().enumerate().take(end).skip(start) {
        let t = n as f64 / fs;
        st += t;
        sy += y;
        stt += t * t;
        sty += t * y;
    }
    let slope = (count * sty - st * sy) / (count * stt - st * st);
    -60.0 / slope
}

fn reference_parameters(values: &[f32; 8]) -> ReferenceReverbParameters {
    let p = |index: usize| f64::from(values[index]);
    ReferenceReverbParameters {
        pre_delay_ms: p(PRE_DELAY),
        decay_ms: p(DECAY),
        size: p(SIZE),
        damping_hz: p(DAMPING),
        low_cut_hz: p(4),
        diffusion: p(5),
        width: p(6),
        mix: p(7),
    }
}

/// The gating rule: a band is gated when its centre is at or below the effective damping cutoff,
/// `fc <= min(damping_hz, 19,845, fs / 4)`. Above it the loop low-pass removes most of a band's
/// energy per trip and Jot's smooth-decay model does not describe the band (the note's test 1).
fn band_is_gated(values: &[f32; 8], fs: f64, fc: f64) -> bool {
    fc <= f64::from(values[DAMPING]).min(19_845.0).min(fs / 4.0)
}

/// T40 from 0.2 s in the 125 / 500 / 2,000 / 8,000 Hz octave bands, at decay 0.5 / 2 / 8 s and
/// damping 20 k / 6 k / 2 k, 48 and 96 kHz, size 1, diffusion 0.7: within 5 % of the design
/// prediction. Four unit impulse responses (L to L, L to R, R to L, R to R) of
/// `floor((0.2 + 2 T) fs)` samples are band-passed and summed in energy. Gated cells follow
/// [`band_is_gated`]; the others are printed, not gated.
///
/// Red for: a wrong gain law, filter coefficient, line-length or size mapping, which moves the
/// decay of a band; the oracle test sees only 0.75 s of one decay time.
#[test]
#[cfg_attr(
    debug_assertions,
    ignore = "about 30 million frames and their band filters: release only"
)]
fn band_decay_times_match_the_design() {
    const BOUND: f64 = 0.05;
    let mut failures = Vec::new();
    for sample_rate in [48_000_u32, 96_000] {
        let fs = f64::from(sample_rate);
        for decay_s in [0.5_f64, 2.0, 8.0] {
            for damping in [20_000.0_f32, 6_000.0, 2_000.0] {
                let mut values = defaults();
                values[DECAY] = (decay_s * 1000.0) as f32;
                values[DAMPING] = damping;
                values[SIZE] = 1.0;
                let frames = ((0.2 + 2.0 * decay_s) * fs).floor() as usize;
                let from_left = impulse_response(&values, sample_rate, (1.0, 0.0), frames);
                let from_right = impulse_response(&values, sample_rate, (0.0, 1.0), frames);
                for fc in [125.0_f64, 500.0, 2_000.0, 8_000.0] {
                    let low = fc / core::f64::consts::SQRT_2;
                    let high = (fc * core::f64::consts::SQRT_2).min(0.45 * fs);
                    let band = butterworth_band(low, high, fs);
                    let mut energy = vec![0.0_f64; frames];
                    for response in [&from_left.0, &from_left.1, &from_right.0, &from_right.1] {
                        for (e, y) in energy.iter_mut().zip(filter(&band, response)) {
                            *e += y * y;
                        }
                    }
                    let measured = decay_time(&energy, fs);
                    let predicted = reference_reverb_rt60_seconds(
                        sample_rate,
                        reference_parameters(&values),
                        low,
                        high,
                    );
                    let error = measured / predicted - 1.0;
                    println!(
                        "{sample_rate} Hz decay {decay_s} s damping {damping} Hz band {fc} Hz: \
                         T40 {measured:.4} s, predicted {predicted:.4} s, error {:+.2} %{}",
                        error * 100.0,
                        if band_is_gated(&values, fs, fc) {
                            ""
                        } else {
                            " (not gated)"
                        }
                    );
                    let gated = band_is_gated(&values, fs, fc);
                    if gated && error.abs() > BOUND {
                        failures.push((sample_rate, decay_s, damping, fc, error));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "cells outside 5 %: {failures:?}");
}
