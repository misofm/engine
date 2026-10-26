//! Independent offline `f64` oracle for the `miso.reverb` 16-line feedback delay network (#2114).
//!
//! Written from `dsp-research/reverb.md` ("Algorithm and equations" R1-R12 and "Coefficients and
//! update rules") and nothing else: the oracle imports no production code. It models the
//! *settled* kernel only, with static parameters, so it has no ramps and no crossfades (R12 and the
//! crossfade forms of R1 and R3 are identities at static parameters).
//!
//! Independence choices, so that a disagreement with the kernel points at the kernel:
//!
//! - The feedback matrix is the dense 16 x 16 product `H16 P / 4`, built from the closed form
//!   `H16[i][j] = (-1)^popcount(i & j)` and the permutation `(P f)_j = f_{(14 - j) mod 16}`, and
//!   applied as a matrix-vector product. It is deliberately not the kernel's four-stage butterfly.
//! - The filter states are indexed by line, not by slot. Both filters are elementwise with shared
//!   coefficients, so filtering a line and then permuting it into its slot is the same map.
//! - Every coefficient is computed in `f64` from the `f64` parameters. Only the values that the
//!   kernel derives through an integer rule (line lengths, pre-delay samples, diffuser lengths) use
//!   the control as the kernel sees it: the `f32` control value widened to `f64`.

use crate::FLUSH_EPS;

/// The 16 base line lengths at 48 kHz and size 1, line order 0..15 (the note's `BASE`).
const BASE: [u32; 16] = [
    953, 1171, 1433, 1753, 2141, 2617, 3203, 3907, 1061, 1297, 1583, 1933, 2371, 2897, 3533, 4327,
];

/// Left diffuser lengths at 48 kHz, stages 0..3 (the note's `DIFF_L`).
const DIFF_L: [u32; 4] = [229, 173, 613, 449];

/// Right diffuser lengths at 48 kHz, stages 0..3 (the note's `DIFF_R`).
const DIFF_R: [u32; 4] = [233, 179, 617, 457];

/// Input gain per line (the note's `B`).
const INPUT_GAIN: f64 = 0.5;

/// The matrix output rows that feed the left and right outputs (the note's `OUT`).
const OUTPUT_ROWS: (usize, usize) = (14, 15);

/// The damping cutoff cap in hertz (#1374; the second term of `min(19,845, fs / 4)`).
const DAMPING_CAP_HZ: f64 = 19_845.0;

/// The rate the base lengths are stated at.
const BASE_RATE_HZ: f64 = 48_000.0;

/// The largest size, which sets the ring capacity.
const MAX_SIZE: f64 = 2.0;

/// The crossfade length that the kernel's rings leave room for (`TRANSITION_SAMPLES`).
const TRANSITION_SAMPLES: usize = 128;

/// Static controls of the reverb, in the units of the note's parameter table.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReferenceReverbParameters {
    /// Pre-delay in milliseconds, `0..=250`.
    pub pre_delay_ms: f64,
    /// Reverberation time in milliseconds, `100..=20_000`.
    pub decay_ms: f64,
    /// Room-size ratio that scales the line lengths, `0.25..=2.0`.
    pub size: f64,
    /// In-loop low-pass cutoff in hertz, `1_000..=20_000` (capped at `min(19,845, fs / 4)`).
    pub damping_hz: f64,
    /// In-loop high-pass cutoff in hertz, `20..=500`.
    pub low_cut_hz: f64,
    /// Input-diffuser amount, `0..=1`; the allpass coefficient is `0.7 * diffusion`.
    pub diffusion: f64,
    /// Stereo width of the wet signal, `0..=1`.
    pub width: f64,
    /// Wet/dry mix, `0..=1`; exact dry at 0 and exact wet at 1.
    pub mix: f64,
}

/// A construction input of [`ReferenceReverb`] was outside the frozen launch domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReferenceReverbError {
    /// The sample rate is not 44,100, 48,000, 88,200 or 96,000 Hz.
    UnsupportedSampleRate(u32),
    /// The named parameter is non-finite or outside its inclusive domain.
    ParameterOutOfDomain(&'static str),
}

/// One delay ring with its own write cursor.
#[derive(Clone, Debug)]
struct Ring {
    words: Vec<f64>,
    cursor: usize,
}

impl Ring {
    fn new(capacity: usize) -> Self {
        Self {
            words: vec![0.0; capacity],
            cursor: 0,
        }
    }

    /// The index of the word written `delay` samples before the current write position.
    fn index(&self, delay: usize) -> usize {
        let capacity = self.words.len();
        (self.cursor + capacity - delay % capacity) % capacity
    }

    fn read(&self, delay: usize) -> f64 {
        self.words[self.index(delay)]
    }

    /// R3: read the word `delay` samples old, then set it to `+0.0` (clear-on-read).
    fn read_and_clear(&mut self, delay: usize) -> f64 {
        let index = self.index(delay);
        let word = self.words[index];
        self.words[index] = 0.0;
        word
    }

    fn write(&mut self, value: f64) {
        let index = self.cursor;
        self.words[index] = value;
    }

    fn advance(&mut self) {
        self.cursor = (self.cursor + 1) % self.words.len();
    }
}

/// Independent `f64` oracle of the settled `miso.reverb` kernel.
#[derive(Clone, Debug)]
pub struct ReferenceReverb {
    parameters: ReferenceReverbParameters,
    /// R1: one pre-delay ring per channel, `floor(0.25 fs) + 1 + 128` words.
    pre_delay_rings: [Ring; 2],
    pre_delay: usize,
    /// R2: diffuser `(c, s)`, ring of `M_{c,s}` words.
    diffuser_rings: [[Ring; 4]; 2],
    diffuser_lengths: [[usize; 4]; 2],
    /// R2: `y_{c,s}[n - 1]` for `s = 0..2`, the pipeline registers.
    pipeline: [[f64; 3]; 2],
    allpass_k: f64,
    /// R3: line `i`, ring of `D_i(2.0, fs) + 128` words.
    line_rings: [Ring; 16],
    lengths: [usize; 16],
    /// R5 and R6: the low-pass and high-pass states, indexed by line.
    low_pass_states: [f64; 16],
    high_pass_states: [f64; 16],
    low_pass_g: f64,
    high_pass_g: f64,
    /// Jot's absorption gain `g_i` per line.
    gains: [f64; 16],
    /// The dense feedback matrix `H16 P / 4`, row `i` writes line `i`.
    matrix: [[f64; 16]; 16],
}

impl ReferenceReverb {
    /// Static parameters (no ramps, no crossfades): the state a settled kernel reaches.
    ///
    /// # Errors
    ///
    /// [`ReferenceReverbError::UnsupportedSampleRate`] for a rate other than the four launch rates,
    /// and [`ReferenceReverbError::ParameterOutOfDomain`] naming the first parameter that is
    /// non-finite or outside its domain.
    pub fn new(
        sample_rate: u32,
        parameters: ReferenceReverbParameters,
    ) -> Result<Self, ReferenceReverbError> {
        validate(sample_rate, parameters)?;
        let fs = f64::from(sample_rate);
        let lengths = line_lengths_at(parameters.size, sample_rate);
        let capacities = line_lengths_at(MAX_SIZE, sample_rate);
        let pre_delay_capacity = quarter_second(sample_rate) + 1 + TRANSITION_SAMPLES;
        let diffuser_lengths = [
            DIFF_L.map(|base| diffuser_length(base, sample_rate)),
            DIFF_R.map(|base| diffuser_length(base, sample_rate)),
        ];
        Ok(Self {
            parameters,
            pre_delay_rings: [Ring::new(pre_delay_capacity), Ring::new(pre_delay_capacity)],
            pre_delay: pre_delay_samples(parameters.pre_delay_ms, sample_rate),
            diffuser_rings: diffuser_lengths.map(|channel| channel.map(Ring::new)),
            diffuser_lengths,
            pipeline: [[0.0; 3]; 2],
            allpass_k: 0.7 * parameters.diffusion,
            line_rings: capacities.map(|capacity| Ring::new(capacity + TRANSITION_SAMPLES)),
            lengths,
            low_pass_states: [0.0; 16],
            high_pass_states: [0.0; 16],
            low_pass_g: one_pole_g(damping_cutoff_hz(parameters.damping_hz, fs), fs),
            high_pass_g: one_pole_g(parameters.low_cut_hz, fs),
            gains: absorption_gains(&lengths, fs, parameters.decay_ms),
            matrix: feedback_matrix(),
        })
    }

    /// Processes one input frame through R1-R11 and returns the output frame `(left, right)`.
    pub fn process_sample(&mut self, left: f64, right: f64) -> (f64, f64) {
        let input = [left, right];

        // R1: pre-delay. Write first, so that P = 0 reads this sample's input.
        let mut pre_delayed = [0.0; 2];
        for (channel, ring) in self.pre_delay_rings.iter_mut().enumerate() {
            ring.write(input[channel]);
            pre_delayed[channel] = ring.read(self.pre_delay);
            ring.advance();
        }

        // R2: four pipelined Schroeder allpasses per channel. Stage s >= 1 reads stage s - 1's
        // output of the previous sample; the registers update after all eight lanes have run.
        let k = self.allpass_k;
        let mut injection = [0.0; 2];
        for channel in 0..2 {
            let mut outputs = [0.0; 4];
            for (stage, output) in outputs.iter_mut().enumerate() {
                let stage_input = if stage == 0 {
                    pre_delayed[channel]
                } else {
                    self.pipeline[channel][stage - 1]
                };
                let ring = &mut self.diffuser_rings[channel][stage];
                let delayed = ring.read(self.diffuser_lengths[channel][stage]);
                let v = k * delayed + stage_input;
                *output = delayed - k * v;
                ring.write(flush(v));
                ring.advance();
            }
            self.pipeline[channel] = [outputs[0], outputs[1], outputs[2]];
            injection[channel] = outputs[3];
        }

        // R3: the 16 taps, each read once and cleared.
        let mut taps = [0.0; 16];
        for (line, tap) in taps.iter_mut().enumerate() {
            *tap = self.line_rings[line].read_and_clear(self.lengths[line]);
        }

        // R4-R6: the TPT low-pass, then the TPT high-pass (low-pass input minus its low-pass), per
        // line. R4's slot permutation lives in the matrix.
        let mut filtered = [0.0; 16];
        for line in 0..16 {
            let low = tpt_low_pass(taps[line], self.low_pass_g, &mut self.low_pass_states[line]);
            let low_of_low = tpt_low_pass(low, self.high_pass_g, &mut self.high_pass_states[line]);
            filtered[line] = low - low_of_low;
        }

        // R7: the dense product (H16 P / 4) F. Row i is the value written to line i.
        let mut mixed = [0.0; 16];
        for (row, value) in self.matrix.iter().zip(mixed.iter_mut()) {
            *value = row
                .iter()
                .zip(filtered.iter())
                .map(|(coefficient, word)| coefficient * word)
                .sum();
        }

        // R8: the two output rows, before the absorption gains.
        let wet_left = mixed[OUTPUT_ROWS.0];
        let wet_right = mixed[OUTPUT_ROWS.1];

        // R9: gain on the written line, plus the injection of its half.
        let q = [INPUT_GAIN * injection[0], INPUT_GAIN * injection[1]];
        for (line, (ring, value)) in self.line_rings.iter_mut().zip(mixed).enumerate() {
            let half = usize::from(line >= 8);
            ring.write(flush(self.gains[line] * value + q[half]));
            ring.advance();
        }

        // R10: the mid/side width matrix.
        let mid = 0.5 * (wet_left + wet_right);
        let side = self.parameters.width * (0.5 * (wet_left - wet_right));
        let wet = [mid + side, mid - side];

        // R11: the mix, with exact selects at 0 and 1.
        let mix = self.parameters.mix;
        let out = [0, 1].map(|channel| {
            if mix == 0.0 {
                input[channel]
            } else if mix == 1.0 {
                wet[channel]
            } else {
                input[channel] + mix * (wet[channel] - input[channel])
            }
        });
        (out[0], out[1])
    }

    /// The 16 line lengths D_i in samples (note: nearest-prime rule), line order 0..15.
    #[must_use]
    pub fn line_lengths(&self) -> [usize; 16] {
        self.lengths
    }
}

/// The design prediction of the reverberation time (seconds) in a band [low_hz, high_hz], from the
/// note's test-1 rule: Jot's per-line formula combined as T = -3 sum D_i / (fs sum log10 |g_i H_i(f)|),
/// weighted over the band by the band filter's power.
///
/// The rule, step by step (the note's "Objective tests and tolerances", test 1):
///
/// 1. `T(f) = -3 sum_i D_i / (fs sum_i log10 |g_i H(f)|)`, with `D_i` and `g_i` as
///    [`ReferenceReverb`] designs them and `H = HP(LP)` the in-loop pair from the designed `f64`
///    coefficients: `|H|^2 = G1^2 / (G1^2 + t^2) * t^2 / (G2^2 + t^2)`, `t = tan(pi f / fs)`
///    (the bilinear images of the two one-poles).
/// 2. `T(f)` is evaluated on 600 log-spaced frequencies `f_k` from 20 Hz to `0.45 fs` (both ends
///    included).
/// 3. The band filter is a third-order Butterworth band-pass with edges `low_hz` and `high_hz`
///    (the bilinear design with pre-warped edges, as the measurement's analysis filter):
///    `|B(f)|^2 = 1 / (1 + ((t^2 - t_lo t_hi) / (t (t_hi - t_lo)))^6)`. Each frequency's weight is
///    `|B(f_k)|^2 f_k`; the factor `f_k` is the density `df` of the log-spaced grid, so the sum is
///    a rectangle-rule integral over linear frequency.
/// 4. The predicted energy envelope is `env[n] = sum_k w_k 10^(-6 n / (fs T(f_k)))` for
///    `n = 0 .. floor(L fs) - 1`, with the measurement length `L = max(1.5, 0.8 T + 0.4)` seconds
///    and `T = decay_ms / 1000`.
/// 5. The fit is the measured one: Schroeder backward integration of `env`, the EDC normalised to
///    its first value in dB, a start index `i5 = max(first n with EDC <= -5 dB, floor(0.15 fs))`,
///    an end index `i25 =` first `n` with `EDC <= EDC[i5] - 20 dB`, a least-squares line of EDC
///    against `n / fs` over `i5 .. i25` (end excluded), and `RT60 = -60 / slope`.
///
/// # Panics
///
/// On an unsupported rate, an out-of-domain parameter, or a band that is not
/// `0 < low_hz < high_hz < fs / 2`.
#[must_use]
pub fn reference_reverb_rt60_seconds(
    sample_rate: u32,
    parameters: ReferenceReverbParameters,
    low_hz: f64,
    high_hz: f64,
) -> f64 {
    validate(sample_rate, parameters).expect("reference reverb parameters in domain");
    let fs = f64::from(sample_rate);
    assert!(
        low_hz > 0.0 && low_hz < high_hz && high_hz < fs / 2.0,
        "band must satisfy 0 < low_hz < high_hz < fs / 2"
    );

    let lengths = line_lengths_at(parameters.size, sample_rate);
    let gains = absorption_gains(&lengths, fs, parameters.decay_ms);
    let big_g1 = prewarp(damping_cutoff_hz(parameters.damping_hz, fs), fs);
    let big_g2 = prewarp(parameters.low_cut_hz, fs);
    let total_length: f64 = lengths.iter().map(|&length| length as f64).sum();

    let band_low = prewarp(low_hz, fs);
    let band_high = prewarp(high_hz, fs);

    const POINTS: usize = 600;
    let top = 0.45 * fs;
    let mut weights = [0.0; POINTS];
    let mut ratios = [0.0; POINTS];
    for k in 0..POINTS {
        let frequency = 20.0 * (top / 20.0).powf(k as f64 / (POINTS - 1) as f64);
        let t = prewarp(frequency, fs);
        let loop_power =
            big_g1 * big_g1 / (big_g1 * big_g1 + t * t) * (t * t / (big_g2 * big_g2 + t * t));
        let log_sum: f64 = gains
            .iter()
            .map(|gain| gain.log10() + 0.5 * loop_power.log10())
            .sum();
        let decay_seconds = -3.0 * total_length / (fs * log_sum);
        let shape = (t * t - band_low * band_high) / (t * (band_high - band_low));
        weights[k] = frequency / (1.0 + shape.powi(6));
        // The per-sample energy ratio 10^(-6 / (fs T(f))).
        ratios[k] = 10f64.powf(-6.0 / (fs * decay_seconds));
    }

    let seconds = (0.8 * parameters.decay_ms / 1000.0 + 0.4).max(1.5);
    let frames = (seconds * fs).floor() as usize;
    let mut envelope = vec![0.0; frames];
    let mut powers = [1.0; POINTS];
    for value in &mut envelope {
        let mut sum = 0.0;
        for k in 0..POINTS {
            sum += weights[k] * powers[k];
            powers[k] *= ratios[k];
        }
        *value = sum;
    }

    // Schroeder backward integration, then the EDC in dB relative to its first value.
    let mut remaining = 0.0;
    let mut edc = vec![0.0; frames];
    for (energy, level) in envelope.iter().zip(edc.iter_mut()).rev() {
        remaining += energy;
        *level = remaining;
    }
    let first = edc[0];
    for level in &mut edc {
        *level = 10.0 * (*level / first).log10();
    }

    let minus_five = edc.iter().position(|&level| level <= -5.0).unwrap_or(0);
    let start = minus_five.max((0.15 * fs).floor() as usize);
    let target = edc[start] - 20.0;
    let end = edc
        .iter()
        .position(|&level| level <= target)
        .expect("the EDC falls 20 dB below its start inside the measurement length");
    least_squares_rt60(&edc[start..end], start, fs)
}

/// `-60 / slope` of the least-squares line through `(n / fs, edc[n - start])`.
fn least_squares_rt60(edc: &[f64], start: usize, fs: f64) -> f64 {
    let count = edc.len() as f64;
    let times = || (0..edc.len()).map(|offset| (start + offset) as f64 / fs);
    let mean_time = times().sum::<f64>() / count;
    let mean_level = edc.iter().sum::<f64>() / count;
    let (covariance, variance) =
        times()
            .zip(edc.iter())
            .fold((0.0, 0.0), |(covariance, variance), (time, &level)| {
                let dt = time - mean_time;
                (covariance + dt * (level - mean_level), variance + dt * dt)
            });
    -60.0 / (covariance / variance)
}

fn validate(
    sample_rate: u32,
    parameters: ReferenceReverbParameters,
) -> Result<(), ReferenceReverbError> {
    if !matches!(sample_rate, 44_100 | 48_000 | 88_200 | 96_000) {
        return Err(ReferenceReverbError::UnsupportedSampleRate(sample_rate));
    }
    let domains: [(&'static str, f64, f64, f64); 8] = [
        ("pre_delay_ms", parameters.pre_delay_ms, 0.0, 250.0),
        ("decay_ms", parameters.decay_ms, 100.0, 20_000.0),
        ("size", parameters.size, 0.25, 2.0),
        ("damping_hz", parameters.damping_hz, 1_000.0, 20_000.0),
        ("low_cut_hz", parameters.low_cut_hz, 20.0, 500.0),
        ("diffusion", parameters.diffusion, 0.0, 1.0),
        ("width", parameters.width, 0.0, 1.0),
        ("mix", parameters.mix, 0.0, 1.0),
    ];
    for (name, value, minimum, maximum) in domains {
        if !(value.is_finite() && (minimum..=maximum).contains(&value)) {
            return Err(ReferenceReverbError::ParameterOutOfDomain(name));
        }
    }
    Ok(())
}

/// A control as the kernel sees it: the `f32` control value widened to `f64`. Used only where the
/// note fixes an integer rule on the control (lengths and pre-delay samples).
fn as_control(value: f64) -> f64 {
    f64::from(value as f32)
}

/// `D_i(size, fs) = Prime(floor(BASE_i * size * fs / 48000 + 0.5))`.
fn line_lengths_at(size: f64, sample_rate: u32) -> [usize; 16] {
    let scale = as_control(size) * f64::from(sample_rate) / BASE_RATE_HZ;
    BASE.map(|base| nearest_prime((f64::from(base) * scale + 0.5).floor() as usize))
}

/// `M_{c,s}(fs) = floor(DIFF * fs / 48000 + 0.5)`; the diffusers do not scale with size.
fn diffuser_length(base: u32, sample_rate: u32) -> usize {
    (f64::from(base) * f64::from(sample_rate) / BASE_RATE_HZ + 0.5).floor() as usize
}

/// `P = floor(pre_delay_ms * fs / 1000 + 0.5)`.
fn pre_delay_samples(pre_delay_ms: f64, sample_rate: u32) -> usize {
    let samples = (as_control(pre_delay_ms) * f64::from(sample_rate) / 1000.0 + 0.5).floor();
    (samples as usize).min(quarter_second(sample_rate))
}

/// `floor(0.25 fs)`, the largest pre-delay in samples.
fn quarter_second(sample_rate: u32) -> usize {
    (sample_rate / 4) as usize
}

/// The nearest prime to `n`; on a tie the smaller.
fn nearest_prime(n: usize) -> usize {
    for distance in 0..=n {
        if is_prime(n - distance) {
            return n - distance;
        }
        if is_prime(n + distance) {
            return n + distance;
        }
    }
    unreachable!("a prime exists within distance n of n (Bertrand)")
}

/// Trial division, written out so the oracle shares no sieve with the kernel.
fn is_prime(n: usize) -> bool {
    if n < 2 {
        return false;
    }
    let mut divisor = 2;
    while divisor * divisor <= n {
        if n.is_multiple_of(divisor) {
            return false;
        }
        divisor += 1;
    }
    true
}

/// Jot: `g_i = exp(-3 ln(10) D_i / (fs T))`, `T = decay_ms / 1000`.
fn absorption_gains(lengths: &[usize; 16], fs: f64, decay_ms: f64) -> [f64; 16] {
    let decay_seconds = decay_ms / 1000.0;
    lengths.map(|length| {
        (-3.0 * core::f64::consts::LN_10 * length as f64 / (fs * decay_seconds)).exp()
    })
}

/// `fc = min(damping_hz, 19,845, fs / 4)`.
fn damping_cutoff_hz(damping_hz: f64, fs: f64) -> f64 {
    damping_hz.min(DAMPING_CAP_HZ).min(fs / 4.0)
}

/// `G = tan(pi f / fs)`.
fn prewarp(frequency: f64, fs: f64) -> f64 {
    (core::f64::consts::PI * frequency / fs).tan()
}

/// `g = G / (1 + G)` of the TPT one-pole.
fn one_pole_g(cutoff_hz: f64, fs: f64) -> f64 {
    let big_g = prewarp(cutoff_hz, fs);
    big_g / (1.0 + big_g)
}

/// R5 (and R6's inner low-pass): `delta = x - s; h = g delta; y = h + s; s' = flush(y + h)`.
fn tpt_low_pass(input: f64, g: f64, state: &mut f64) -> f64 {
    let h = g * (input - *state);
    let output = h + *state;
    *state = flush(output + h);
    output
}

/// `|x| < FLUSH_EPS` becomes `+0.0`; `-0.0` becomes `+0.0`.
fn flush(value: f64) -> f64 {
    if value.abs() < f64::from(FLUSH_EPS) {
        0.0
    } else {
        value
    }
}

/// The note's permutation `pi(j) = (14 - j) mod 16`: slot `j` holds line `pi(j)`.
fn slot_source(slot: usize) -> usize {
    (14 + 16 - slot) % 16
}

/// The dense `H16 P / 4`: entry `(i, l)` is `H16[i][j] / 4` for the slot `j` that holds line `l`.
fn feedback_matrix() -> [[f64; 16]; 16] {
    let mut matrix = [[0.0; 16]; 16];
    for (row, entries) in matrix.iter_mut().enumerate() {
        for slot in 0..16 {
            let sign = if (row & slot).count_ones() % 2 == 0 {
                1.0
            } else {
                -1.0
            };
            entries[slot_source(slot)] += sign * 0.25;
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];

    fn defaults() -> ReferenceReverbParameters {
        ReferenceReverbParameters {
            pre_delay_ms: 20.0,
            decay_ms: 1_800.0,
            size: 1.0,
            damping_hz: 6_000.0,
            low_cut_hz: 20.0,
            diffusion: 0.7,
            width: 1.0,
            mix: 1.0,
        }
    }

    #[test]
    fn feedback_matrix_is_orthogonal_and_not_an_involution() {
        let a = feedback_matrix();
        let product = |left: &[[f64; 16]; 16], right_transposed: bool| {
            let mut out = [[0.0; 16]; 16];
            for i in 0..16 {
                for j in 0..16 {
                    out[i][j] = (0..16)
                        .map(|k| left[i][k] * if right_transposed { a[j][k] } else { a[k][j] })
                        .sum();
                }
            }
            out
        };
        let gram = product(&a, true);
        let square = product(&a, false);
        for (i, row) in gram.iter().enumerate() {
            for (j, &entry) in row.iter().enumerate() {
                assert_eq!(entry, if i == j { 1.0 } else { 0.0 }, "A A^T at {i},{j}");
            }
        }
        assert!(
            (0..16).any(|i| square[i][i] != 1.0),
            "A^2 = I would be the plain-H16 structure the note rejects"
        );
    }

    #[test]
    fn base_lengths_follow_the_ranked_rule_and_are_the_size_one_lengths() {
        let ranked: Vec<usize> = (0..16)
            .map(|k| {
                nearest_prime((960.0 * 4.5f64.powf(f64::from(k) / 15.0) + 0.5).floor() as usize)
            })
            .collect();
        let expected: [usize; 16] = core::array::from_fn(|i| {
            if i < 8 {
                ranked[2 * i]
            } else {
                ranked[2 * (i - 8) + 1]
            }
        });
        assert_eq!(BASE.map(|base| base as usize), expected);
        let reverb = ReferenceReverb::new(48_000, defaults()).expect("defaults");
        assert_eq!(reverb.line_lengths(), expected);
    }

    #[test]
    fn lengths_are_sixteen_distinct_primes_over_the_size_sweep_at_every_rate() {
        for rate in RATES {
            for step in 250..=2_000 {
                let size = f64::from(step) / 1_000.0;
                let lengths = line_lengths_at(size, rate);
                assert!(
                    lengths.iter().all(|&length| is_prime(length)),
                    "{rate} {size}"
                );
                for i in 0..16 {
                    for j in i + 1..16 {
                        assert_ne!(lengths[i], lengths[j], "{rate} {size} lines {i} {j}");
                    }
                }
            }
        }
    }

    #[test]
    fn first_wet_sample_arrives_after_pre_delay_pipeline_and_shortest_left_line() {
        let parameters = ReferenceReverbParameters {
            pre_delay_ms: 10.0,
            ..defaults()
        };
        let mut reverb = ReferenceReverb::new(48_000, parameters).expect("in domain");
        let expected = 480
            + 3
            + reverb.line_lengths()[..8]
                .iter()
                .min()
                .copied()
                .unwrap_or(0);
        let first = (0..expected + 8)
            .position(|n| {
                let input = if n == 0 { 1.0 } else { 0.0 };
                reverb.process_sample(input, 0.0) != (0.0, 0.0)
            })
            .expect("a wet sample");
        assert_eq!(first, expected);
    }

    #[test]
    fn rt60_prediction_is_the_decay_control_where_the_loop_filters_pass() {
        let parameters = ReferenceReverbParameters {
            decay_ms: 2_000.0,
            damping_hz: 20_000.0,
            ..defaults()
        };
        let predicted = reference_reverb_rt60_seconds(
            48_000,
            parameters,
            500.0 / 2f64.sqrt(),
            500.0 * 2f64.sqrt(),
        );
        assert!((predicted / 2.0 - 1.0).abs() < 0.01, "{predicted}");
    }
}
