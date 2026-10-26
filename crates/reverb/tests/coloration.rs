//! Coloration of the late tail (the note's test 4), against a perfectly diffuse reference.

mod common;

use common::{DAMPING, DECAY, DIFFUSION, PRE_DELAY, SIZE, WIDTH, defaults, impulse_response};
use dsp_reference::{ReferenceReverb, ReferenceReverbParameters};

const RATE: u32 = 48_000;
const DECAY_SECONDS: f64 = 1.8;
const TAIL_START: f64 = 0.2;
const TAIL_END: f64 = 1.7;
/// Index of `low_cut_hz` in descriptor order.
const LOW_CUT: usize = 4;
/// Index of `mix` in descriptor order.
const MIX: usize = 7;

/// The reference's trials (seeds `1..=32`); each draws four independent noise paths.
const REFERENCE_SEEDS: u64 = 32;
/// The reference keeps each bin's decay-envelope spectrum over `|m| <= KERNEL_HALF_WIDTH` bins.
const KERNEL_HALF_WIDTH: usize = 128;
/// Largest share of an input bin's expected energy the kept kernel may drop.
const KERNEL_DROP_LIMIT: f64 = 0.01;

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

/// The tail of one response from 0.2 to 1.7 s, multiplied by `10^(3 t / T)` to flatten the
/// nominal decay.
fn flattened_tail(h: &[f32], fs: f64) -> Vec<f64> {
    let start = (TAIL_START * fs) as usize;
    let end = (TAIL_END * fs) as usize;
    h[start..end]
        .iter()
        .enumerate()
        .map(|(n, x)| {
            let t = n as f64 / fs + TAIL_START;
            f64::from(*x) * math::pow(10.0, 3.0 * t / DECAY_SECONDS)
        })
        .collect()
}

/// The note's smoothing bands: for centres `200 * 2^(i / 24)` Hz up to 8 kHz, the `[lo, hi)` bins
/// of the 1/24-octave band and of the 1-octave band at the same centre, a band's bins being the
/// first bin at or above each edge, over `bins` bins of `bin_hz`.
fn bands(bins: usize, bin_hz: f64) -> Vec<[(usize, usize); 2]> {
    // The first `k` with `k * bin_hz >= f` (or `bins`), found from the quotient and corrected.
    let first_bin_at_or_above = |f: f64| {
        let mut k = (math::floor(f / bin_hz) as usize).min(bins);
        while k > 0 && (k - 1) as f64 * bin_hz >= f {
            k -= 1;
        }
        while k < bins && (k as f64 * bin_hz) < f {
            k += 1;
        }
        k
    };
    let edges = |centre: f64, fraction: f64| {
        (
            first_bin_at_or_above(centre * math::exp2(-fraction / 2.0)),
            first_bin_at_or_above(centre * math::exp2(fraction / 2.0)),
        )
    };
    let last = math::floor(math::log2(8_000.0 / 200.0) * 24.0) as usize;
    (0..=last)
        .map(|i| {
            let centre = 200.0 * math::exp2(i as f64 / 24.0);
            [edges(centre, 1.0 / 24.0), edges(centre, 1.0)]
        })
        .collect()
}

/// The note's coloration of one power spectrum: each 1/24-octave band's mean power divided by the
/// 1-octave band's at the same centre, in dB. Returns the peak-to-median and the (population)
/// standard deviation of those ratios. Only the bins inside `bands` are read.
fn coloration(spectrum: &[f64], bands: &[[(usize, usize); 2]]) -> (f64, f64) {
    let mut cumulative = vec![0.0_f64; spectrum.len() + 1];
    for (k, power) in spectrum.iter().enumerate() {
        cumulative[k + 1] = cumulative[k] + power;
    }
    let mean_power =
        |(i0, i1): (usize, usize)| (cumulative[i1] - cumulative[i0]) / (i1 - i0).max(1) as f64;
    let mut ratios: Vec<f64> = bands
        .iter()
        .map(|[narrow, octave]| 10.0 * math::log10(mean_power(*narrow) / mean_power(*octave)))
        .collect();
    let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
    let std =
        (ratios.iter().map(|r| (r - mean) * (r - mean)).sum::<f64>() / ratios.len() as f64).sqrt();
    let peak = ratios.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    ratios.sort_by(f64::total_cmp);
    let median = ratios[ratios.len() / 2];
    (peak - median, std)
}

/// The four-path figures the bounds are on: the mean peak-to-median and the worst std.
fn pooled(figures: &[(f64, f64)]) -> (f64, f64) {
    let mean = figures.iter().map(|f| f.0).sum::<f64>() / figures.len() as f64;
    let worst_std = figures.iter().map(|f| f.1).fold(0.0, f64::max);
    (mean, worst_std)
}

/// A seeded standard-normal generator: SplitMix64 words, Box-Muller pairs.
struct Gaussian {
    state: u64,
}

impl Gaussian {
    /// Uniform in `(0, 1]`: never 0, so the logarithm is finite.
    fn uniform(&mut self) -> f64 {
        self.state = self.state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^= z >> 31;
        ((z >> 11) + 1) as f64 / (1_u64 << 53) as f64
    }

    /// Two independent standard normals, as one complex value.
    fn pair(&mut self) -> Complex {
        let radius = (-2.0 * math::log(self.uniform())).sqrt();
        let angle = 2.0 * core::f64::consts::PI * self.uniform();
        (radius * math::cos(angle), radius * math::sin(angle))
    }
}

/// A perfectly diffuse tail with the design's per-frequency decay, as power spectra of the
/// flattened segment.
///
/// The tail is white Gaussian noise whose every frequency decays at the design's predicted rate:
/// `s[n] = (1/N) sum_k W_k e_k(n) exp(2 pi i k n / N)` over the segment's `N` samples, `W` the
/// DFT of white Gaussian noise (independent complex normals on the positive bins) and
/// `e_k(n) = 10^(-3 t (1 / T(f_k) - 1 / T))`, `t = 0.2 s + n / fs`: the bin's own decay after the
/// same `10^(3 t / T)` flattening. `T(f)` is the note's test-1 prediction,
/// `T(f) = -3 sum D_i / (fs sum log10 |g_i H(f)|)` with Jot's gains and `H` the designed in-loop
/// pair (`|H|^2 = G1^2 / (G1^2 + t^2) * t^2 / (G2^2 + t^2)`, `t = tan(pi f / fs)`). The gains give
/// `T` exactly where `H = 1`, so the extra amplitude decay per sample is
/// `a(f) = -ln |H(f)| / mean(D_i)`.
///
/// The segment's DFT is `S_j = (1/N) sum_k W_k E_k[j - k]`, with the closed form
/// `E_k[m] = c_k (1 - r_k^N) / (1 - r_k exp(-2 pi i m / N))`, `c_k = e_k(0)`, `r_k = exp(-a(f_k))`:
/// the same rectangular segment and the same DFT bins as the design's measurement, without
/// transforming. The sum keeps `|j - k| <= KERNEL_HALF_WIDTH`; the dropped share of each input
/// bin's energy (by Parseval, `sum_m |E_k[m]|^2 = N sum_n e_k(n)^2`) is checked to be at most
/// [`KERNEL_DROP_LIMIT`].
struct DiffuseReference {
    /// The output bins `[first, end)` the bands read.
    first: usize,
    end: usize,
    /// `kernel[(j - first) * (2 h + 1) + (m + h)]` is input bin `k = j - m`'s `E_k[m] / N`.
    kernel: Vec<Complex>,
    bins: usize,
}

impl DiffuseReference {
    fn new(lengths: &[usize], damping_cutoff_hz: f64, low_cut_hz: f64, n: usize) -> Self {
        let fs = f64::from(RATE);
        let bin_hz = fs / n as f64;
        let bins = n / 2 + 1;
        let bands = bands(bins, bin_hz);
        let first = bands.iter().map(|b| b[1].0).min().expect("bands");
        let end = bands.iter().map(|b| b[1].1).max().expect("bands");
        let half = KERNEL_HALF_WIDTH;
        assert!(
            first > half && end + half < bins,
            "the kernel stays on positive bins"
        );
        let mean_length = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
        let prewarp = |f: f64| math::tan(core::f64::consts::PI * f / fs);
        let (g1, g2) = (prewarp(damping_cutoff_hz), prewarp(low_cut_hz));
        let width = 2 * half + 1;
        let unit: Vec<Complex> = (0..width)
            .map(|index| {
                let angle = -2.0 * core::f64::consts::PI * (index as f64 - half as f64) / n as f64;
                (math::cos(angle), math::sin(angle))
            })
            .collect();
        let samples = n as f64;
        let mut rows: Vec<Vec<Complex>> = Vec::new();
        let mut worst_drop = 0.0_f64;
        for k in first - half..end + half {
            let t = prewarp(k as f64 * bin_hz);
            let loop_power = g1 * g1 / (g1 * g1 + t * t) * (t * t / (g2 * g2 + t * t));
            let a = -0.5 * math::log(loop_power) / mean_length;
            let c = math::exp(-a * TAIL_START * fs);
            let r = math::exp(-a);
            let scale = c * -math::expm1(-a * samples) / samples;
            let row: Vec<Complex> = unit
                .iter()
                .map(|(wr, wi)| {
                    let (dr, di) = (1.0 - r * wr, -r * wi);
                    let d = dr * dr + di * di;
                    (scale * dr / d, -scale * di / d)
                })
                .collect();
            let kept: f64 = row.iter().map(|(re, im)| re * re + im * im).sum();
            // `sum_n e_k(n)^2 / N`, the whole `sum_m |E_k[m] / N|^2`.
            let whole = c * c * math::expm1(-2.0 * a * samples) / (math::expm1(-2.0 * a) * samples);
            worst_drop = worst_drop.max(1.0 - kept / whole);
            rows.push(row);
        }
        println!("reference kernel: worst dropped energy share {worst_drop:.2e}");
        assert!(
            worst_drop <= KERNEL_DROP_LIMIT,
            "dropped share {worst_drop}"
        );
        let mut kernel = Vec::with_capacity((end - first) * width);
        for j in first..end {
            // Row index `m + h` holds input bin `k = j - m`, whose row is `k - (first - h)`.
            kernel.extend((0..width).map(|index| rows[j + half - index - (first - half)][index]));
        }
        Self {
            first,
            end,
            kernel,
            bins,
        }
    }

    /// One path's power spectrum with `W` drawn from `noise`; bins outside `[first, end)` are 0.
    fn power_spectrum(&self, noise: &mut Gaussian) -> Vec<f64> {
        let half = KERNEL_HALF_WIDTH;
        let width = 2 * half + 1;
        let low = self.first - half;
        let w: Vec<Complex> = (low..self.end + half).map(|_| noise.pair()).collect();
        let mut spectrum = vec![0.0; self.bins];
        for (offset, j) in (self.first..self.end).enumerate() {
            let row = &self.kernel[offset * width..][..width];
            // Row index `m + h` pairs with input bin `j - m`, so the inputs run downwards.
            let inputs = &w[j - half - low..=j + half - low];
            let (mut re, mut im) = (0.0, 0.0);
            for ((er, ei), (xr, xi)) in row.iter().zip(inputs.iter().rev()) {
                re += er * xr - ei * xi;
                im += er * xi + ei * xr;
            }
            spectrum[j] = re * re + im * im;
        }
        spectrum
    }
}

/// The late tail is no more colored than a perfectly diffuse tail with the same decay: over the
/// four paths (L to L, L to R, R to L, R to R), the mean 1/24-octave peak-to-median and the worst
/// path's standard deviation are within the reference's maxima (48 kHz, size 1, decay 1.8 s,
/// damping 20 kHz, i.e. the `fs / 4` cap, low cut 20 Hz, diffusion 0.7, pre-delay 0 as the note's
/// model, other controls at their defaults).
///
/// The rule (root's, fixed before the design was measured against it, never from the design's
/// result): the bounds are the maximum over 32 fixed seeds of the same two four-path figures of a
/// [`DiffuseReference`], four independent noise paths per seed, through the same segment, bins
/// and smoothing. The reference carries the design's predicted per-frequency decay `T(f)` from
/// the oracle's 32 line lengths (their mean length enters the per-bin decay), because the
/// statistic is not tilt-insensitive: the decay difference across the 1-octave band at the top
/// centres moves the ratio (the note's test 4 records the flat-noise figures). The window,
/// 0.2-1.7 s, starts after the mixing-time bound (150 ms at diffusion 0.7, the echo-density
/// test), so it measures the late tail, not the early reflections.
///
/// The claim is only modal-density collapse. Red for: too few modes in the late tail, for example
/// the 16-line design this one replaced (0.75 modes per Hz at size 1), the slot permutation
/// replaced by the identity (the plain `H32` structure), or a feedback matrix that leaves lines
/// uncoupled.
/// Coprime line lengths are not this test's claim (a duplicated base length passes it); the unit
/// test `line_lengths_are_pairwise_coprime_at_every_size_and_rate` owns that. No other test looks
/// at the tail's spectral fine structure.
#[test]
fn late_tail_is_no_more_colored_than_a_diffuse_reference() {
    let mut values = defaults();
    values[SIZE] = 1.0;
    values[DECAY] = 1_800.0;
    values[DAMPING] = 20_000.0;
    values[DIFFUSION] = 0.7;
    values[PRE_DELAY] = 0.0;
    let fs = f64::from(RATE);

    let parameters = ReferenceReverbParameters {
        pre_delay_ms: f64::from(values[PRE_DELAY]),
        decay_ms: f64::from(values[DECAY]),
        size: f64::from(values[SIZE]),
        damping_hz: f64::from(values[DAMPING]),
        low_cut_hz: f64::from(values[LOW_CUT]),
        diffusion: f64::from(values[DIFFUSION]),
        width: f64::from(values[WIDTH]),
        mix: f64::from(values[MIX]),
    };
    let lengths = ReferenceReverb::new(RATE, parameters)
        .expect("oracle")
        .line_lengths();
    // The designed damping cutoff, `min(damping, 19,845 Hz, fs / 4)`.
    let damping_cutoff_hz = parameters.damping_hz.min(19_845.0).min(fs / 4.0);
    let n = ((TAIL_END - TAIL_START) * fs) as usize;
    let reference = DiffuseReference::new(&lengths, damping_cutoff_hz, parameters.low_cut_hz, n);
    let bands = bands(n / 2 + 1, fs / n as f64);
    let (mut bound_mean, mut bound_std) = (0.0_f64, 0.0_f64);
    for seed in 1..=REFERENCE_SEEDS {
        let mut noise = Gaussian { state: seed };
        let figures: Vec<(f64, f64)> = (0..4)
            .map(|_| coloration(&reference.power_spectrum(&mut noise), &bands))
            .collect();
        let (mean, worst_std) = pooled(&figures);
        bound_mean = bound_mean.max(mean);
        bound_std = bound_std.max(worst_std);
    }
    println!(
        "reference, {REFERENCE_SEEDS} seeds: max mean peak-to-median {bound_mean:.2} dB, max worst \
         std {bound_std:.2} dB"
    );

    let frames = (TAIL_END * fs) as usize;
    let mut figures = Vec::new();
    for impulse in [(1.0_f32, 0.0_f32), (0.0, 1.0)] {
        let (left, right) = impulse_response(&values, RATE, impulse, frames);
        for path in [left, right] {
            let tail = flattened_tail(&path, fs);
            assert_eq!(tail.len(), n);
            figures.push(coloration(&power_spectrum(&tail), &bands));
        }
    }
    for (path, (peak_to_median, std)) in ["L-L", "L-R", "R-L", "R-R"].iter().zip(&figures) {
        println!("{path}: peak-to-median {peak_to_median:.2} dB, std {std:.2} dB");
    }
    let (mean, worst_std) = pooled(&figures);
    println!("design: mean peak-to-median {mean:.2} dB, worst std {worst_std:.2} dB");
    assert!(
        mean <= bound_mean,
        "mean peak-to-median {mean} dB exceeds the reference's {bound_mean} dB"
    );
    assert!(
        worst_std <= bound_std,
        "worst std {worst_std} dB exceeds the reference's {bound_std} dB"
    );
}
