//! Coloration of the late tail (the note's test 4).

mod common;

use common::{DAMPING, DECAY, DIFFUSION, PRE_DELAY, SIZE, defaults, impulse_response};

const RATE: u32 = 48_000;
const DECAY_SECONDS: f64 = 1.8;
const TAIL_START: f64 = 0.2;
const TAIL_END: f64 = 1.7;

/// A complex number as `(re, im)`.
type Complex = (f64, f64);

/// The DFT of `x` by recursive mixed-radix decimation in time over its prime factors, with each
/// radix-`p` combination done directly; `twiddle[j] = exp(-2 pi i j / N)` for the top-level `N`,
/// and `stride = N / x.len()`. Exact length, so the bins are the note's `numpy.fft.rfft` bins.
fn dft(x: &[Complex], twiddle: &[Complex], stride: usize) -> Vec<Complex> {
    let n = x.len();
    if n == 1 {
        return x.to_vec();
    }
    let p = (2..=n).find(|p| n.is_multiple_of(*p)).expect("a factor");
    let m = n / p;
    let subs: Vec<Vec<Complex>> = (0..p)
        .map(|r| {
            let part: Vec<Complex> = x.iter().skip(r).step_by(p).copied().collect();
            dft(&part, twiddle, stride * p)
        })
        .collect();
    let top = twiddle.len();
    let mut out = vec![(0.0, 0.0); n];
    for (index, word) in out.iter_mut().enumerate() {
        let k = index % m;
        let mut acc = (0.0, 0.0);
        for (r, sub) in subs.iter().enumerate() {
            let (wr, wi) = twiddle[(r * index * stride) % top];
            let (sr, si) = sub[k];
            acc.0 += wr * sr - wi * si;
            acc.1 += wr * si + wi * sr;
        }
        *word = acc;
    }
    out
}

/// The power spectrum `|X_k|^2`, `k = 0 ..= N / 2`, of the real signal `x`.
fn power_spectrum(x: &[f64]) -> Vec<f64> {
    let n = x.len();
    let twiddle: Vec<Complex> = (0..n)
        .map(|j| {
            let angle = -2.0 * core::f64::consts::PI * j as f64 / n as f64;
            (math::cos(angle), math::sin(angle))
        })
        .collect();
    let input: Vec<Complex> = x.iter().map(|v| (*v, 0.0)).collect();
    dft(&input, &twiddle, 1)[..=n / 2]
        .iter()
        .map(|(re, im)| re * re + im * im)
        .collect()
}

/// The note's coloration of one response: the tail from 0.2 to 1.7 s, multiplied by
/// `10^(3 t / T)` to flatten the decay, its power spectrum averaged over 1/24-octave bands at
/// centres `200 * 2^(i / 24)` Hz up to 8 kHz, each divided by the 1-octave average at the same
/// centre. A band is the bins `[lo, hi)` by the first bin at or above each edge. Returns the
/// peak-to-median and the (population) standard deviation of those ratios, in dB.
fn coloration(h: &[f32], fs: u32) -> (f64, f64) {
    let fs = f64::from(fs);
    let start = (TAIL_START * fs) as usize;
    let end = (TAIL_END * fs) as usize;
    let segment: Vec<f64> = h[start..end]
        .iter()
        .enumerate()
        .map(|(n, x)| {
            let t = n as f64 / fs + TAIL_START;
            f64::from(*x) * math::pow(10.0, 3.0 * t / DECAY_SECONDS)
        })
        .collect();
    let spectrum = power_spectrum(&segment);
    let bin_hz = fs / segment.len() as f64;
    let mut cumulative = vec![0.0_f64; spectrum.len() + 1];
    for (k, power) in spectrum.iter().enumerate() {
        cumulative[k + 1] = cumulative[k] + power;
    }
    let first_bin_at_or_above = |f: f64| {
        (0..spectrum.len())
            .find(|k| *k as f64 * bin_hz >= f)
            .unwrap_or(spectrum.len())
    };
    let band = |centre: f64, fraction: f64| {
        let i0 = first_bin_at_or_above(centre * math::exp2(-fraction / 2.0));
        let i1 = first_bin_at_or_above(centre * math::exp2(fraction / 2.0));
        (cumulative[i1] - cumulative[i0]) / (i1 - i0).max(1) as f64
    };
    let last = math::floor(math::log2(8_000.0 / 200.0) * 24.0) as usize;
    let mut ratios: Vec<f64> = (0..=last)
        .map(|i| {
            let centre = 200.0 * math::exp2(i as f64 / 24.0);
            10.0 * math::log10(band(centre, 1.0 / 24.0) / band(centre, 1.0))
        })
        .collect();
    let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
    let std =
        (ratios.iter().map(|r| (r - mean) * (r - mean)).sum::<f64>() / ratios.len() as f64).sqrt();
    let peak = ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    ratios.sort_by(f64::total_cmp);
    let median = ratios[ratios.len() / 2];
    (peak - median, std)
}

/// The late tail has no strong modal ringing: over the four paths (L to L, L to R, R to L,
/// R to R) the mean of the 1/24-octave peak-to-median is at most 3.5 dB and every path's
/// standard deviation at most 1.5 dB (48 kHz, size 1, decay 1.8 s, damping 20 kHz, i.e. the
/// `fs / 4` cap, low cut 20 Hz, diffusion 0.7, pre-delay 0 as the note's model, other controls
/// at their defaults). The bound is on the mean because one path's peak-to-median is noisy: a
/// Gaussian-noise reference spans 1.63-2.82 dB over eight seeds. The note's model measured a
/// 2.85 dB mean and a 1.44 dB worst std.
///
/// Red for: a modal-density collapse of the feedback structure, for example the plain `H16`
/// without the slot permutation (4.63 dB). Coprime line lengths are a separate claim, held by
/// `line_lengths_are_pairwise_coprime_at_every_size_and_rate` (one duplicated base length passes
/// this test). The 3.5 / 1.5 dB bounds are the note's attempt-1 values, not yet derived from a
/// stated rule: root's diffuse-noise rule (#2114 attempt 2) fails the design, and the bound is
/// open for root's ruling.
#[test]
fn late_tail_coloration_is_within_the_bounds() {
    let mut values = defaults();
    values[SIZE] = 1.0;
    values[DECAY] = 1_800.0;
    values[DAMPING] = 20_000.0;
    values[DIFFUSION] = 0.7;
    values[PRE_DELAY] = 0.0;
    let frames = (TAIL_END * f64::from(RATE)) as usize;
    let mut figures = Vec::new();
    for impulse in [(1.0_f32, 0.0_f32), (0.0, 1.0)] {
        let (left, right) = impulse_response(&values, RATE, impulse, frames);
        figures.push(coloration(&left, RATE));
        figures.push(coloration(&right, RATE));
    }
    for (path, (peak_to_median, std)) in ["L-L", "L-R", "R-L", "R-R"].iter().zip(&figures) {
        println!("{path}: peak-to-median {peak_to_median:.2} dB, std {std:.2} dB");
    }
    let mean = figures.iter().map(|f| f.0).sum::<f64>() / figures.len() as f64;
    let worst_std = figures.iter().map(|f| f.1).fold(0.0, f64::max);
    println!("mean peak-to-median {mean:.2} dB, worst std {worst_std:.2} dB");
    assert!(mean <= 3.5, "mean peak-to-median {mean} dB");
    assert!(worst_std <= 1.5, "worst std {worst_std} dB");
}
