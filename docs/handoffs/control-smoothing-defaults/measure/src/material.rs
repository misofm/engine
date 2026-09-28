//! Deterministic, repository-owned test material (no third-party audio, no licence question).
//!
//! * `bass`: a sustained A1 (55 Hz) additive bass tone. Every partial is at or below 1 kHz, so the
//!   source has no energy above 1 kHz at all and any energy a mute or fader puts there is exactly
//!   the splatter. A sustained low note is the textbook worst case for a gain click: nothing in the
//!   programme masks the broadband energy of a discontinuity.
//! * `kick`: a pitched-sine kick (glide 150 -> 45 Hz, long body) with a short noise beater. The
//!   measurement places every transition in the body, 110-190 ms after an onset.
//! * `mix`: a dense 120 BPM stereo arrangement (kick, snare, closed/open hats, shaker, bass line and
//!   a detuned saw-pad chord up to 12 kHz). It is full-band, so it masks what the two others expose.
//!
//! Everything is synthesised in `f64` from closed-form oscillators and a SplitMix64 noise source
//! with fixed seeds, then rounded once to `f32`.

use std::f64::consts::TAU;

/// SplitMix64 (Steele, Lea and Flood, OOPSLA 2014), the repository's listening-randomiser family.
pub struct SplitMix64(pub u64);

impl SplitMix64 {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// Uniform in `[0, 1)`.
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1_u64 << 53) as f64
    }
    /// Uniform in `[-1, 1)`.
    pub fn bipolar(&mut self) -> f64 {
        2.0 * self.uniform() - 1.0
    }
}

/// A stereo buffer in the engine's planar layout.
#[derive(Clone)]
pub struct Stereo {
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

impl Stereo {
    pub fn dual_mono(mono: &[f64]) -> Self {
        let lane: Vec<f32> = mono.iter().map(|&v| v as f32).collect();
        Self {
            left: lane.clone(),
            right: lane,
        }
    }
    pub fn slice(&self, start: usize, end: usize) -> Self {
        Self {
            left: self.left[start..end].to_vec(),
            right: self.right[start..end].to_vec(),
        }
    }
    /// Whether both lanes carry identical samples (then one lane is analysed, not two).
    pub fn is_dual_mono(&self) -> bool {
        self.left == self.right
    }
}

fn seconds(rate: u32, frames: usize) -> impl Iterator<Item = (usize, f64)> {
    (0..frames).map(move |i| (i, i as f64 / f64::from(rate)))
}

/// Raised-cosine gate: 0 before `start`, rising over `attack`, 1 until `stop`, falling over `release`.
fn gate(t: f64, start: f64, attack: f64, stop: f64, release: f64) -> f64 {
    if t < start || t >= stop + release {
        0.0
    } else if t < start + attack {
        0.5 - 0.5 * (std::f64::consts::PI * (t - start) / attack).cos()
    } else if t < stop {
        1.0
    } else {
        0.5 + 0.5 * (std::f64::consts::PI * (t - stop) / release).cos()
    }
}

/// Partials of the additive bass: `(frequency, amplitude, phase)`, all at or below `top_hz`.
fn bass_partials(f0: f64, top_hz: f64, rng: &mut SplitMix64) -> Vec<(f64, f64, f64)> {
    let mut partials = Vec::new();
    let mut n = 1_u32;
    while f64::from(n) * f0 <= top_hz {
        let k = f64::from(n);
        let amplitude = (1.0 / k) * (-(k - 1.0) / 8.0).exp();
        partials.push((k * f0, amplitude, rng.uniform() * TAU));
        n += 1;
    }
    partials
}

/// The highest partial of [`bass`]; the out-of-band edge sits well above it.
pub const BASS_TOP_HZ: f64 = 1000.0;

/// A sustained 55 Hz bass tone, strictly band-limited to [`BASS_TOP_HZ`], peak <= 0.5.
pub fn bass(rate: u32, frames: usize) -> Vec<f64> {
    let mut rng = SplitMix64(0x0B45_5000_0000_0055);
    let partials = bass_partials(55.0, BASS_TOP_HZ, &mut rng);
    let norm = 0.5 / partials.iter().map(|p| p.1).sum::<f64>();
    seconds(rate, frames)
        .map(|(_, t)| {
            let fade_in = gate(t, 0.0, 0.01, f64::INFINITY, 0.0);
            fade_in
                * norm
                * partials
                    .iter()
                    .map(|&(f, a, p)| a * (TAU * f * t + p).sin())
                    .sum::<f64>()
        })
        .collect()
}

/// One-pole low-pass coefficient for a -3 dB corner at `hz`.
fn one_pole(rate: u32, hz: f64) -> f64 {
    (-TAU * hz / f64::from(rate)).exp()
}

/// Adds one kick voice at `onset` seconds. The body is a sine gliding 150 -> 45 Hz.
fn add_kick(out: &mut [f64], rate: u32, onset: f64, decay: f64, level: f64, rng: &mut SplitMix64) {
    let first = (onset * f64::from(rate)).ceil() as usize;
    let lp = one_pole(rate, 6000.0);
    let mut click = 0.0;
    for (i, sample) in out.iter_mut().enumerate().skip(first) {
        let t = i as f64 / f64::from(rate) - onset;
        if t > 8.0 * decay {
            break;
        }
        let phase = TAU * (45.0 * t + 105.0 * 0.03 * (1.0 - (-t / 0.03).exp()));
        let envelope = (-t / decay).exp() * (1.0 - (-t / 0.0008).exp());
        let noise = if t < 0.004 {
            rng.bipolar() * (-t / 0.0012).exp()
        } else {
            0.0
        };
        click = lp * click + (1.0 - lp) * noise;
        *sample += level * (envelope * phase.sin() + 0.25 * click);
    }
}

/// A four-on-the-floor kick pattern: onsets at `first + k * period`, peak about 0.7.
pub fn kick(rate: u32, frames: usize, first: f64, period: f64) -> Vec<f64> {
    let mut out = vec![0.0; frames];
    let mut rng = SplitMix64(0x4B49_434B_0000_0001);
    let duration = frames as f64 / f64::from(rate);
    let mut onset = first;
    while onset < duration {
        add_kick(&mut out, rate, onset, 0.35, 0.7, &mut rng);
        onset += period;
    }
    out
}

/// Band-limited sawtooth-like additive note (partials `1/n`) added into `out`.
#[allow(clippy::too_many_arguments)]
fn add_additive(
    out: &mut [f64],
    rate: u32,
    f0: f64,
    top_hz: f64,
    level: f64,
    start: f64,
    attack: f64,
    stop: f64,
    release: f64,
    rng: &mut SplitMix64,
    rolloff: f64,
) {
    let nyquist_guard = 0.45 * f64::from(rate);
    let top = top_hz.min(nyquist_guard);
    let mut partials = Vec::new();
    let mut n = 1_u32;
    while f64::from(n) * f0 <= top {
        let k = f64::from(n);
        partials.push((k * f0, (1.0 / k) * (-(k - 1.0) / rolloff).exp(), rng.uniform() * TAU));
        n += 1;
    }
    let first = (start * f64::from(rate)).floor().max(0.0) as usize;
    let last = (((stop + release) * f64::from(rate)).ceil() as usize).min(out.len());
    for (i, sample) in out.iter_mut().enumerate().take(last).skip(first) {
        let t = i as f64 / f64::from(rate);
        let g = gate(t, start, attack, stop, release);
        if g == 0.0 {
            continue;
        }
        let s: f64 = partials
            .iter()
            .map(|&(f, a, p)| a * (TAU * f * t + p).sin())
            .sum();
        *sample += level * g * s;
    }
}

/// A filtered noise burst: high-passed by a cascade of `hp_poles` one-poles, then low-passed.
/// Returns the hit's samples from its onset (ten decay constants long).
fn noise_hit(
    rate: u32,
    decay: f64,
    hp_hz: f64,
    hp_poles: usize,
    lp_hz: f64,
    rng: &mut SplitMix64,
) -> Vec<f64> {
    let hp = one_pole(rate, hp_hz);
    let lp = one_pole(rate, lp_hz);
    let mut lows = vec![0.0; hp_poles];
    let mut smooth = 0.0;
    let frames = (10.0 * decay * f64::from(rate)).ceil() as usize;
    (0..frames)
        .map(|i| {
            let t = i as f64 / f64::from(rate);
            let mut v = rng.bipolar();
            for low in lows.iter_mut() {
                *low = hp * *low + (1.0 - hp) * v;
                v -= *low;
            }
            smooth = lp * smooth + (1.0 - lp) * v;
            (-t / decay).exp() * (1.0 - (-t / 0.0005).exp()) * smooth
        })
        .collect()
}

/// Adds `hit` at `onset` seconds into both sides with the given gains.
fn place(left: &mut [f64], right: &mut [f64], rate: u32, onset: f64, hit: &[f64], gains: (f64, f64)) {
    let first = (onset * f64::from(rate)).ceil() as usize;
    for (offset, v) in hit.iter().enumerate() {
        let i = first + offset;
        if i >= left.len() {
            break;
        }
        left[i] += gains.0 * v;
        right[i] += gains.1 * v;
    }
}

/// The dense stereo mix: 120 BPM, peak normalised to -3 dBFS.
pub fn mix(rate: u32, frames: usize) -> Stereo {
    let mut left = vec![0.0; frames];
    let mut right = vec![0.0; frames];
    let mut centre = vec![0.0; frames];
    let mut rng = SplitMix64(0x4D49_5800_0000_0120);
    let beat = 0.5;
    let duration = frames as f64 / f64::from(rate);
    let bass_line = [55.0, 55.0, 65.406, 48.999];
    let mut index = 0_usize;
    let mut onset = 0.05;
    while onset < duration {
        let beat_in_bar = index % 4;
        add_kick(&mut centre, rate, onset, 0.22, 0.55, &mut rng);
        if beat_in_bar == 1 || beat_in_bar == 3 {
            let snare = noise_hit(rate, 0.09, 400.0, 1, 8000.0, &mut rng);
            place(&mut left, &mut right, rate, onset, &snare, (0.9, 0.9));
            add_additive(
                &mut centre, rate, 185.0, 400.0, 0.12, onset, 0.001, onset + 0.02, 0.12,
                &mut rng, 2.0,
            );
        }
        let f0 = bass_line[beat_in_bar];
        add_additive(
            &mut centre, rate, f0, 1200.0, 0.22, onset, 0.005, onset + 0.42, 0.03, &mut rng,
            6.0,
        );
        for eighth in 0..2 {
            let t = onset + 0.25 * f64::from(eighth);
            let open = beat_in_bar == 3 && eighth == 1;
            let decay = if open { 0.15 } else { 0.025 };
            let hat = noise_hit(rate, decay, 7000.0, 2, 16000.0, &mut rng);
            place(&mut left, &mut right, rate, t, &hat, (0.48, 0.96));
        }
        for sixteenth in 0..4 {
            let t = onset + 0.125 * f64::from(sixteenth);
            let shaker = noise_hit(rate, 0.04, 5000.0, 2, 14000.0, &mut rng);
            place(&mut left, &mut right, rate, t, &shaker, (0.36, 0.135));
        }
        onset += beat;
        index += 1;
    }
    // Pad: A minor (A3, C4, E4), two voices per note detuned +-6 cents, one voice per side.
    for &f in &[220.0, 261.626, 329.628] {
        let detune = 2.0_f64.powf(6.0 / 1200.0);
        add_additive(
            &mut left, rate, f / detune, 12_000.0, 0.035, 0.0, 0.3, duration, 0.0, &mut rng, 40.0,
        );
        add_additive(
            &mut right, rate, f * detune, 12_000.0, 0.035, 0.0, 0.3, duration, 0.0, &mut rng,
            40.0,
        );
    }
    for i in 0..frames {
        left[i] += centre[i];
        right[i] += centre[i];
    }
    let peak = left
        .iter()
        .chain(right.iter())
        .fold(0.0_f64, |m, v| m.max(v.abs()));
    let norm = 10.0_f64.powf(-3.0 / 20.0) / peak;
    Stereo {
        left: left.iter().map(|v| (v * norm) as f32).collect(),
        right: right.iter().map(|v| (v * norm) as f32).collect(),
    }
}
