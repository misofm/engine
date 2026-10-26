//! Normalised echo density of the impulse response (the note's test 2) [ABEL-HUANG-NED].

mod common;

use common::{DAMPING, DECAY, DIFFUSION, PRE_DELAY, SIZE, defaults, impulse_response};

const RATE: u32 = 48_000;

/// `erfc(1 / sqrt 2)`: the fraction of a Gaussian's samples outside one standard deviation, the
/// normalisation that makes a Gaussian signal's echo density 1.
const ERFC_ONE_OVER_SQRT_TWO: f64 = 0.317_310_507_862_914_1;

/// Abel and Huang's normalised echo density `eta(t)` of `h`: at each centre `c` (1 ms hop), the
/// unit-sum 20 ms Hann window's weight of the samples whose magnitude exceeds the window's
/// weighted standard deviation, divided by [`ERFC_ONE_OVER_SQRT_TWO`]. The window is the note's
/// (numpy's `hanning(963)[1:-1]`, 961 points at 48 kHz); returns `(centre seconds, eta)`.
fn echo_density(h: &[f32], fs: u32) -> Vec<(f64, f64)> {
    let half = (20.0 * f64::from(fs) / 2000.0).round() as usize;
    let points = 2 * half + 3;
    let mut window: Vec<f64> = (1..points - 1)
        .map(|n| {
            0.5 - 0.5 * math::cos(2.0 * core::f64::consts::PI * n as f64 / (points - 1) as f64)
        })
        .collect();
    let sum: f64 = window.iter().sum();
    for w in &mut window {
        *w /= sum;
    }
    let hop = (fs / 1000) as usize;
    (half..h.len() - half)
        .step_by(hop)
        .map(|c| {
            let segment = &h[c - half..=c + half];
            let variance: f64 = window
                .iter()
                .zip(segment)
                .map(|(w, x)| w * f64::from(*x) * f64::from(*x))
                .sum();
            let sigma = variance.sqrt();
            let eta = if sigma == 0.0 {
                0.0
            } else {
                window
                    .iter()
                    .zip(segment)
                    .filter(|(_, x)| f64::from(x.abs()) > sigma)
                    .map(|(w, _)| w)
                    .sum::<f64>()
                    / ERFC_ONE_OVER_SQRT_TWO
            };
            (c as f64 / f64::from(fs), eta)
        })
        .collect()
}

/// The note's mixing time: the centre time of the first `eta` point from which the mean of the
/// next 50 points (50 ms) reaches `0.9`.
fn mixing_time(eta: &[(f64, f64)]) -> f64 {
    const POINTS: usize = 50;
    eta.windows(POINTS)
        .find(|run| run.iter().map(|(_, v)| v).sum::<f64>() / POINTS as f64 >= 0.9)
        .map_or(f64::INFINITY, |run| run[0].0)
}

/// The L-to-L impulse response becomes Gaussian-dense quickly: the 50 ms mean of the normalised
/// echo density reaches 0.9 by 150 ms at diffusion 0.7 and by 215 ms at diffusion 0 (48 kHz,
/// size 1, decay 1.8 s, damping 6 kHz, pre-delay 0, other controls at their defaults; one second
/// rendered). At diffusion 0 the allpasses are pure delays, so that setting measures the feedback
/// matrix alone. The note's `f64` model measured 104 ms and 193 ms; a Gaussian-noise reference's
/// 50 ms mean never falls below 0.946.
///
/// Red for: a feedback matrix or slot permutation that under-mixes (the note's candidates B, C,
/// Householder-16 and N = 8 take 228-341 ms at diffusion 0), a butterfly stage dropped or
/// paired on the wrong slots, or diffusers that stop diffusing at 0.7. The RT60 and correlation
/// tests measure energy decay and the relation of the two outputs, never the arrival density.
#[test]
fn echo_density_reaches_gaussian_within_the_mixing_time_bounds() {
    for (diffusion, bound) in [(0.7_f32, 0.150_f64), (0.0, 0.215)] {
        let mut values = defaults();
        values[SIZE] = 1.0;
        values[DECAY] = 1_800.0;
        values[DAMPING] = 6_000.0;
        values[PRE_DELAY] = 0.0;
        values[DIFFUSION] = diffusion;
        let (left, _) = impulse_response(&values, RATE, (1.0, 0.0), RATE as usize);
        let eta = echo_density(&left, RATE);
        let t_mix = mixing_time(&eta);
        println!(
            "diffusion {diffusion}: mixing time {:.0} ms",
            t_mix * 1000.0
        );
        assert!(
            t_mix <= bound,
            "diffusion {diffusion}: mixing time {t_mix} s exceeds {bound} s"
        );
    }
}
