//! Listening stimuli for the #1055 blinded comparison, rendered through the engine's own ramps.
//!
//! Every condition is one 48 kHz stereo 32-bit-float WAV named by its condition (so the directory
//! is private: `listening.py prepare` turns it into anonymous trial files). `manifest.tsv` lists
//! each condition with its frame count, peak and RMS. Stimulus edges get a 20 ms raised-cosine
//! fade applied identically to every condition, after the engine, so no edge is a click.
//!
//! * `mute-<material>-<ms>ms`: 2.0 s; a mute admitted at 0.80 s and an unmute at 1.30 s. The kick
//!   pattern has onsets at 0.10 + 0.5 k s, so both transitions fall 150 ms into a kick body.
//! * `drag-bass-<hz>hz-<ms>ms`: 2.4 s; the fader moves 0 -> -24 dB over 0.8 s from 0.3 s, holds
//!   0.2 s and returns over 0.8 s, sampled by a UI at `hz`.
//! * `flick-bass-30hz-<ms>ms`: 1.6 s; the same move over 0.2 s each way (120 dB/s), twice.
//! * `chop-mix-<ms>ms`: 2.0 s; from 0.25 s to 1.75 s the mute toggles every 125 ms (a 16th-note
//!   stutter at 120 BPM), as a hand or a sequencer driving the mute would.
//!
//! `--mix-wav` replaces the synthetic mix with an owner-supplied 48 kHz excerpt (read from
//! `--mix-offset-s`), for the mute and chop conditions.

use std::f64::consts::PI;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::analysis::ms_to_samples;
use crate::material::{self, Stereo};
use crate::strip::{Control, Event, Strip};
use crate::wav;

pub const RATE: u32 = 48_000;

pub const MUTE_RAMPS_MS: [f32; 6] = [0.0, 2.0, 5.0, 10.0, 20.0, 50.0];
pub const DRAG_RAMPS_MS: [f32; 5] = [0.0, 10.0, 20.0, 40.0, 50.0];
pub const CHOP_RAMPS_MS: [f32; 4] = [5.0, 10.0, 20.0, 50.0];

fn seconds(s: f64) -> usize {
    (s * f64::from(RATE)).round() as usize
}

/// Renders `source` through a fresh strip with `events`, then fades the stimulus edges.
fn render(source: &Stereo, events: &[Event]) -> Stereo {
    let mut out = source.clone();
    Strip::new(RATE, false).render(&mut out.left, &mut out.right, events);
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
    fs::write(out.join("manifest.tsv"), manifest).expect("manifest");
}
