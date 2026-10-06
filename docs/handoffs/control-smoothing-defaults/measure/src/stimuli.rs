//! Listening stimuli for the #1055 blinded comparison, rendered through the engine's own ramps.
//!
//! Every condition is one 48 kHz stereo 32-bit-float WAV named by its condition (so the directory
//! is private: `listening.py prepare` turns it into anonymous trial files). `manifest.tsv` lists
//! each condition with its frame count, peak and RMS. Stimulus edges get a 20 ms raised-cosine
//! fade applied identically to every condition, after the engine, so no edge is a click.
//!
//! * `mute-<material>-<ms>ms`: 2.0 s; a mute admitted at 0.80 s and an unmute at 1.30 s. The kick
//!   pattern has onsets at 0.10 + 0.5 k s, so both transitions fall 200 ms into a kick body.
//! * `drag-bass-<hz>hz-<ms>ms`: 2.4 s; the fader moves 0 -> -24 dB over 0.8 s from 0.3 s, holds
//!   0.2 s and returns over 0.8 s, sampled by a UI at `hz`.
//! * `flick-bass-30hz-<ms>ms`: 1.6 s; the same move over 0.2 s each way (120 dB/s), twice.
//! * `chop-mix-<ms>ms`: 2.0 s; from 0.25 s to 1.75 s the mute toggles every 125 ms (a 16th-note
//!   stutter at 120 BPM), as a hand or a sequencer driving the mute would.
//!
//! * `polarity-<material>-<flip>` (listening Amendment 1, block P): 2.0 s; the input polarity
//!   inverts at 0.80 s and restores at 1.30 s through the engine's
//!   `InputBuiltins::set_polarity_invert` (the trim coefficient carried through zero, FINDINGS
//!   9.2), each over the flip's length; `none` is the same strip with no record. The shipped flip
//!   is the polarity row's rule, twice the shipped mute's samples (FINDINGS 9.8): 20 ms. On the
//!   bass, `-inband` and `-oob` split the flip's change `d = y - x` (`y` the flipped render, `x`
//!   the render with no record) at the measures' out-of-band edge (`split.rs`): `-inband` is
//!   `x + L d`, the note with only the in-band change (the dip through zero), and `-oob` is
//!   `y - L d`, the unflipped note with only the out-of-band change (the click). The bass is
//!   rendered over its full 2.4 s, split in `f64`, then cut to 2.0 s and faded.
//!
//! `--mix-wav` replaces the synthetic mix with an owner-supplied 48 kHz excerpt (read from
//! `--mix-offset-s`), for the mute, chop and polarity conditions.

use std::f64::consts::PI;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::analysis::ms_to_samples;
use crate::material::{self, Stereo};
use crate::measure::BASS_OOB_HZ;
use crate::split::BandSplit;
use crate::strip::{Control, Event, Strip};
use crate::wav;

pub const RATE: u32 = 48_000;

pub const MUTE_RAMPS_MS: [f32; 6] = [0.0, 2.0, 5.0, 10.0, 20.0, 50.0];
pub const DRAG_RAMPS_MS: [f32; 5] = [0.0, 10.0, 20.0, 40.0, 50.0];
pub const CHOP_RAMPS_MS: [f32; 4] = [5.0, 10.0, 20.0, 50.0];

/// The shipped `muteMs` (FINDINGS section 1, `CONTROL_SMOOTHING_DEFAULT`).
pub const SHIPPED_MUTE_MS: f32 = 10.0;
/// Block P's slow flip, the dip control: below -6 dB for 100 ms, below -20 dB for 20 ms.
pub const SLOW_FLIP_MS: f32 = 200.0;
/// Design stopband attenuation of the block-P band split.
pub const SPLIT_ATTENUATION_DB: f64 = 120.0;
/// Admission times of block P's invert and restore records (the mute stimuli's times).
const POLARITY_TIMES_S: [(f64, bool); 2] = [(0.80, true), (1.30, false)];

/// Block P's flips, `(name in ms, length in samples)`: the hard flip, the shipped rule (twice the
/// shipped mute's samples, as `LiveRamps::for_row(PolarityInvert)` resolves the polarity row) and
/// the slow flip.
pub fn polarity_flips() -> [(f32, u32); 3] {
    [
        (0.0, 0),
        (
            2.0 * SHIPPED_MUTE_MS,
            2 * ms_to_samples(SHIPPED_MUTE_MS, RATE),
        ),
        (SLOW_FLIP_MS, ms_to_samples(SLOW_FLIP_MS, RATE)),
    ]
}

fn seconds(s: f64) -> usize {
    (s * f64::from(RATE)).round() as usize
}

/// Renders `source` through a fresh strip with `events`.
fn render_raw(source: &Stereo, events: &[Event]) -> Stereo {
    let mut out = source.clone();
    Strip::new(RATE, false).render(&mut out.left, &mut out.right, events);
    out
}

/// Fades the stimulus edges: a 20 ms raised cosine at both ends.
fn fade_edges(mut out: Stereo) -> Stereo {
    let n = out.left.len();
    let edge = seconds(0.02);
    for i in 0..n {
        let d = i.min(n - 1 - i);
        if d < edge {
            let w = (0.5 - 0.5 * (PI * d as f64 / edge as f64).cos()) as f32;
            out.left[i] *= w;
            out.right[i] *= w;
        }
    }
    out
}

/// Renders `source` through a fresh strip with `events`, then fades the stimulus edges.
fn render(source: &Stereo, events: &[Event]) -> Stereo {
    fade_edges(render_raw(source, events))
}

/// Block P's invert and restore records, each over `n` samples.
pub fn polarity_events(n: u32) -> Vec<Event> {
    POLARITY_TIMES_S
        .iter()
        .map(|&(t, inverted)| Event {
            at: seconds(t),
            control: Control::Polarity(inverted),
            smoothing_samples: n,
        })
        .collect()
}

/// One flip's bass presentations, unfaded and as long as the source: `none` is `x`, the render
/// without a record; `full` is `y`, the flipped render; with `d = y - x`, `inband` is `x + L d`
/// (only the in-band change) and `oob` is `y - L d` (only the out-of-band change).
pub struct PolarityParts {
    pub none: Stereo,
    pub full: Stereo,
    pub inband: Stereo,
    pub oob: Stereo,
}

pub fn polarity_parts(source: &Stereo, n: u32, split: &BandSplit) -> PolarityParts {
    let none = render_raw(source, &[]);
    let full = render_raw(source, &polarity_events(n));
    let lane = |x: &[f32], y: &[f32]| {
        let d: Vec<f64> = x
            .iter()
            .zip(y)
            .map(|(&x, &y)| f64::from(y) - f64::from(x))
            .collect();
        let low = split.low(&d);
        let inband: Vec<f32> = x
            .iter()
            .zip(&low)
            .map(|(&x, &l)| (f64::from(x) + l) as f32)
            .collect();
        let oob: Vec<f32> = y
            .iter()
            .zip(&low)
            .map(|(&y, &l)| (f64::from(y) - l) as f32)
            .collect();
        (inband, oob)
    };
    let (inband_left, oob_left) = lane(&none.left, &full.left);
    let (inband_right, oob_right) = lane(&none.right, &full.right);
    PolarityParts {
        none,
        full,
        inband: Stereo {
            left: inband_left,
            right: inband_right,
        },
        oob: Stereo {
            left: oob_left,
            right: oob_right,
        },
    }
}

/// The block-P band split: passes the bass's band, stops from the measures' out-of-band edge.
pub fn polarity_split() -> BandSplit {
    BandSplit::new(
        RATE,
        material::BASS_TOP_HZ,
        BASS_OOB_HZ,
        SPLIT_ATTENUATION_DB,
    )
}

fn mute_events(times: &[(f64, bool)], n: u32) -> Vec<Event> {
    times
        .iter()
        .map(|&(t, muted)| Event {
            at: seconds(t),
            control: Control::Mute(muted),
            smoothing_samples: n,
        })
        .collect()
}

/// Fader-move UI updates: the hand at `value(t)` sampled every `1 / hz` over `[start, end]`.
fn fader_updates(value: impl Fn(f64) -> f64, start: f64, end: f64, hz: f64, n: u32) -> Vec<Event> {
    let mut events = Vec::new();
    let mut k = 0_u32;
    loop {
        let t = start + f64::from(k) / hz;
        events.push(Event {
            at: (t * f64::from(RATE)).ceil() as usize,
            control: Control::FaderDb(value(t) as f32),
            smoothing_samples: n,
        });
        if t >= end {
            break events;
        }
        k += 1;
    }
}

/// A there-and-back move 0 -> `low` -> 0 with `d` seconds each way, `hold` between, from `t0`.
fn there_and_back(t: f64, t0: f64, d: f64, hold: f64, low: f64) -> f64 {
    let shape = if t <= t0 {
        0.0
    } else if t <= t0 + d {
        (t - t0) / d
    } else if t <= t0 + d + hold {
        1.0
    } else if t <= t0 + 2.0 * d + hold {
        1.0 - (t - t0 - d - hold) / d
    } else {
        0.0
    };
    low * shape
}

fn levels(audio: &Stereo) -> (f64, f64) {
    let (mut sum, mut peak, mut count) = (0.0, 0.0_f64, 0.0);
    for &v in audio.left.iter().chain(&audio.right) {
        sum += f64::from(v) * f64::from(v);
        peak = peak.max(f64::from(v).abs());
        count += 1.0;
    }
    (20.0 * peak.log10(), 10.0 * (sum / count).log10())
}

pub fn run(out: &Path, mix_wav: Option<&Path>, mix_offset: f64) {
    fs::create_dir_all(out).expect("output directory");
    let bass = Stereo::dual_mono(&material::bass(RATE, seconds(2.4)));
    let kick = Stereo::dual_mono(&material::kick(RATE, seconds(2.0), 0.10, 0.5));
    let mix = match mix_wav {
        Some(path) => {
            let (rate, audio) = wav::read(path);
            assert_eq!(rate, RATE, "--mix-wav must be {RATE} Hz (there is no implicit SRC)");
            let start = seconds(mix_offset);
            assert!(start + seconds(2.0) <= audio.left.len(), "--mix-wav excerpt too short");
            audio.slice(start, start + seconds(2.0))
        }
        None => material::mix(RATE, seconds(6.0)).slice(seconds(4.0), seconds(6.0)),
    };
    let mut manifest = String::from("condition\tfile\tframes\tpeak_dbfs\trms_dbfs\n");
    let mut write = |name: String, audio: Stereo| {
        let file = format!("{name}.wav");
        wav::write_f32_stereo(&out.join(&file), RATE, &audio);
        let (peak, rms) = levels(&audio);
        assert!(peak < 0.0, "{name} clips");
        let _ = writeln!(manifest, "{name}\t{file}\t{}\t{peak:.3}\t{rms:.3}", audio.left.len());
    };
    for (name, source) in [("bass", &bass), ("kick", &kick), ("mix", &mix)] {
        let excerpt = source.slice(0, seconds(2.0));
        for &ms in &MUTE_RAMPS_MS {
            let events = mute_events(&[(0.80, true), (1.30, false)], ms_to_samples(ms, RATE));
            write(format!("mute-{name}-{ms}ms"), render(&excerpt, &events));
        }
    }
    for hz in [30.0, 60.0] {
        for &ms in &DRAG_RAMPS_MS {
            let n = ms_to_samples(ms, RATE);
            let move_value = |t: f64| there_and_back(t, 0.3, 0.8, 0.2, -24.0);
            let events = fader_updates(move_value, 0.3, 2.1, hz, n);
            write(format!("drag-bass-{hz}hz-{ms}ms"), render(&bass, &events));
        }
    }
    for &ms in &DRAG_RAMPS_MS {
        let n = ms_to_samples(ms, RATE);
        let flick = |t: f64| {
            there_and_back(t, 0.2, 0.2, 0.2, -24.0) + there_and_back(t, 0.9, 0.2, 0.2, -24.0)
        };
        let events = fader_updates(flick, 0.2, 1.5, 30.0, n);
        write(
            format!("flick-bass-30hz-{ms}ms"),
            render(&bass.slice(0, seconds(1.6)), &events),
        );
    }
    for &ms in &CHOP_RAMPS_MS {
        let n = ms_to_samples(ms, RATE);
        let times: Vec<(f64, bool)> = (0..12)
            .map(|k| (0.25 + 0.125 * f64::from(k), k % 2 == 0))
            .collect();
        write(format!("chop-mix-{ms}ms"), render(&mix, &mute_events(&times, n)));
    }
    // Block P (listening Amendment 1): polarity flips. The bass is rendered over its full 2.4 s
    // so the split sees real signal past the 2.0 s cut.
    let split = polarity_split();
    let cut = |audio: &Stereo| fade_edges(audio.slice(0, seconds(2.0)));
    for (i, &(ms, n)) in polarity_flips().iter().enumerate() {
        let parts = polarity_parts(&bass, n, &split);
        if i == 0 {
            write("polarity-bass-none".to_owned(), cut(&parts.none));
        }
        write(format!("polarity-bass-{ms}ms"), cut(&parts.full));
        write(format!("polarity-bass-{ms}ms-inband"), cut(&parts.inband));
        write(format!("polarity-bass-{ms}ms-oob"), cut(&parts.oob));
    }
    let (shipped_ms, shipped_n) = polarity_flips()[1];
    for (name, source) in [("kick", &kick), ("mix", &mix)] {
        let excerpt = source.slice(0, seconds(2.0));
        write(format!("polarity-{name}-none"), render(&excerpt, &[]));
        write(
            format!("polarity-{name}-{shipped_ms}ms"),
            render(&excerpt, &polarity_events(shipped_n)),
        );
    }
    fs::write(out.join("manifest.tsv"), manifest).expect("manifest");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::Fft;
    use crate::measure::align_up;
    use crate::strip::probe;

    /// Block P's shipped stimulus is a polarity flip on the engine's input stage over twice the
    /// shipped mute's samples: the gain leaves `+1` at the applying block, is exactly `-1` from
    /// update `2N` until the restore, and the note is exactly negated there. A stimulus built from
    /// a mute record (gain to zero), at the mute's own length, or at half the window fails it.
    #[test]
    fn the_shipped_polarity_stimulus_flips_over_twice_the_mute_length() {
        let (ms, n) = polarity_flips()[1];
        assert_eq!((ms, n), (20.0, 2 * ms_to_samples(SHIPPED_MUTE_MS, RATE)));
        assert_eq!(n, 960);
        let frames = seconds(2.0);
        let events = polarity_events(n);
        let (g, _) = probe(RATE, false, frames, &events);
        let (invert, restore) = (align_up(seconds(0.80)), align_up(seconds(1.30)));
        let n = n as usize;
        assert!(g[..invert].iter().all(|&v| v == 1.0));
        assert!(g[invert] < 1.0 && g[invert + n - 2] > -1.0);
        assert!(g[invert + n - 1..restore].iter().all(|&v| v == -1.0));
        assert!(g[restore + n - 1..].iter().all(|&v| v == 1.0));
        let bass = Stereo::dual_mono(&material::bass(RATE, frames));
        let (x, y) = (render_raw(&bass, &[]), render_raw(&bass, &events));
        for i in invert + n - 1..restore {
            assert_eq!(y.left[i].to_bits(), (-x.left[i]).to_bits(), "sample {i}");
        }
    }

    /// Energy of `a - b` in `[lo, hi)` Hz, by one exact DFT of the zero-padded difference.
    fn band_energy(a: &[f32], b: &[f32], lo: f64, hi: f64) -> f64 {
        let m = a.len().next_power_of_two();
        let mut re: Vec<f64> = a
            .iter()
            .zip(b)
            .map(|(&a, &b)| f64::from(a) - f64::from(b))
            .collect();
        re.resize(m, 0.0);
        let mut im = vec![0.0; m];
        Fft::new(m).forward(&mut re, &mut im);
        let bin = f64::from(RATE) / m as f64;
        (1..m / 2)
            .filter(|&k| (lo..hi).contains(&(k as f64 * bin)))
            .map(|k| re[k] * re[k] + im[k] * im[k])
            .sum()
    }

    /// The shipped flip's split presentations hold one part of its change each, as rendered to
    /// `f32`: the in-band stimulus's change keeps the in-band part and leaves at least 60 dB of the
    /// out-of-band part (the click) out, the out-of-band stimulus's change the reverse with at
    /// least 100 dB, and the two stimuli rebuild the full flip. A split wired to the wrong signal
    /// (the out-of-band stimulus without the note's own band, `y - L y`, or a sign error on `d`)
    /// fails it. The change is zero outside the flips, so one DFT measures each band exactly.
    #[test]
    fn the_split_stimuli_hold_the_dip_and_the_click_apart() {
        let bass = Stereo::dual_mono(&material::bass(RATE, seconds(2.4)));
        let (_, n) = polarity_flips()[1];
        let p = polarity_parts(&bass, n, &polarity_split());
        let x = &p.none.left;
        let (inband, oob) = (&p.inband.left, &p.oob.left);
        for (i, (&none, &full)) in x.iter().zip(&p.full.left).enumerate() {
            let rebuilt = f64::from(inband[i]) + f64::from(oob[i]);
            let want = f64::from(none) + f64::from(full);
            assert!((rebuilt - want).abs() <= 1e-7, "sample {i}");
        }
        let top = material::BASS_TOP_HZ;
        let edge = BASS_OOB_HZ;
        let nyquist = f64::from(RATE) / 2.0;
        let full_in = band_energy(&p.full.left, x, 0.0, top);
        let full_out = band_energy(&p.full.left, x, edge, nyquist);
        let inband_in = band_energy(&p.inband.left, x, 0.0, top);
        let inband_out = band_energy(&p.inband.left, x, edge, nyquist);
        let oob_in = band_energy(&p.oob.left, x, 0.0, top);
        let oob_out = band_energy(&p.oob.left, x, edge, nyquist);
        let db = |a: f64, b: f64| 10.0 * (a / b).log10();
        let (kept_in, kept_out) = (db(inband_in, full_in), db(oob_out, full_out));
        let (left_out, left_in) = (db(inband_out, full_out), db(oob_in, full_in));
        assert!(kept_in.abs() < 1e-3, "in-band kept: {kept_in} dB");
        assert!(kept_out.abs() < 1e-3, "click kept: {kept_out} dB");
        assert!(left_out < -60.0, "click in -inband: {left_out} dB");
        assert!(left_in < -100.0, "dip in -oob: {left_in} dB");
    }
}
