//! The objective measurements of issue #1055. Deterministic: nothing is timed and the only
//! randomness is fixed-seed SplitMix64, so a rerun on the same commit reproduces the CSVs.
//!
//! Every case renders the material through the engine's own ramp (`strip.rs`) and asserts that the
//! engine's samples equal `f32(g * x)` for the gain trajectory `g` the same engine produces from a
//! unit probe. The spectral measures then run on `g` and `x` in `f64` (`analysis.rs`).

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use builtins::ChannelParameters;

use crate::analysis::{Analyzer, Splatter, Tally, db, ms_to_samples, taper};
use crate::material::{self, SplitMix64, Stereo};
use crate::strip::{Control, Event, QUANTUM, Strip, probe, probe_with};

pub const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
pub const MUTE_RAMPS_MS: [f32; 8] = [0.0, 1.0, 2.0, 3.0, 5.0, 10.0, 20.0, 50.0];
pub const DRAG_RAMPS_MS: [f32; 9] = [0.0, 2.0, 5.0, 10.0, 15.0, 20.0, 30.0, 40.0, 50.0];
pub const UPDATE_HZ: [f64; 2] = [30.0, 60.0];
/// Duration of one fader or pan move: a moderate 0.8 s drag and a fast 0.2 s flick.
pub const MOVES_S: [f64; 2] = [0.8, 0.2];
/// Depth of a fader move (0 dB -> this -> 0 dB): 30 dB/s moderate, 120 dB/s fast.
pub const FADER_LOW_DB: f64 = -24.0;
/// Pan moves go from this position to its negation and back.
pub const PAN_FROM: f64 = -0.5;

pub(crate) const BASS_OOB_HZ: f64 = 1500.0;
const KICK_OOB_HZ: f64 = 1000.0;
/// Analysis frames around a mute: centres from 40 ms before the change to 90 ms after it.
pub(crate) const MUTE_BEFORE_S: f64 = 0.040;
pub(crate) const MUTE_AFTER_S: f64 = 0.090;
/// Excerpt around a mute: the change sits `MUTE_PRE_S` in.
pub(crate) const MUTE_PRE_S: f64 = 0.20;
pub(crate) const MUTE_POST_S: f64 = 0.30;
/// Taper at both excerpt ends, far from every analysis frame.
pub(crate) const TAPER_S: f64 = 0.02;

pub(crate) fn align_up(sample: usize) -> usize {
    sample.div_ceil(QUANTUM) * QUANTUM
}

pub(crate) fn fmt(v: f64) -> String {
    if v.is_finite() {
        format!("{v:.2}")
    } else if v.is_sign_negative() {
        "-inf".to_owned()
    } else {
        "inf".to_owned()
    }
}

pub(crate) fn widen(v: &[f32]) -> Vec<f64> {
    v.iter().map(|&s| f64::from(s)).collect()
}

/// The engine's samples must be exactly `f32(g * x)`: the ramp kernels multiply and nothing else.
fn assert_engine_is_gain(x: &[f32], g: &[f32], y: &[f32]) {
    for i in 0..x.len() {
        let want = (f64::from(x[i]) * f64::from(g[i])) as f32;
        assert!(
            y[i] == want,
            "engine sample {i} is {} but f32(g * x) is {want}",
            y[i]
        );
    }
}

pub struct Material {
    pub(crate) name: &'static str,
    pub(crate) source: Stereo,
    /// Transition sample positions (block-aligned) for the mute measurement.
    pub(crate) events: Vec<usize>,
    pub(crate) oob_hz: Option<f64>,
}

pub(crate) fn materials(rate: u32) -> Vec<Material> {
    let r = f64::from(rate);
    let at = |seconds: f64| align_up((seconds * r).round() as usize);
    let bass = Stereo::dual_mono(&material::bass(rate, (6.0 * r) as usize));
    let bass_events = (0..16).map(|i| at(0.6 + 0.2937 * f64::from(i))).collect();
    let kick = Stereo::dual_mono(&material::kick(rate, (8.0 * r) as usize, 0.25, 0.5));
    let kick_events = (1..=12)
        .map(|k| {
            let onset = 0.25 + 0.5 * f64::from(k);
            let golden = (f64::from(k) * 0.618_033_988_75).fract();
            at(onset + 0.11 + 0.08 * golden)
        })
        .collect();
    let mix = material::mix(rate, (16.0 * r) as usize);
    let mut rng = SplitMix64(0x4D55_5445_0000_0032);
    let mix_events = (0..24).map(|_| at(1.0 + 14.0 * rng.uniform())).collect();
    vec![
        Material {
            name: "bass",
            source: bass,
            events: bass_events,
            oob_hz: Some(BASS_OOB_HZ),
        },
        Material {
            name: "kick",
            source: kick,
            events: kick_events,
            oob_hz: Some(KICK_OOB_HZ),
        },
        Material {
            name: "mix",
            source: mix,
            events: mix_events,
            oob_hz: None,
        },
    ]
}

/// One analysed lane: the tapered excerpt and its sub-band decomposition.
type Lane = (Vec<f64>, Vec<Vec<f64>>);

/// The lanes worth analysing: one when both carry identical samples, else both.
fn lanes(x: &Stereo) -> Vec<&[f32]> {
    if x.is_dual_mono() {
        vec![&x.left]
    } else {
        vec![&x.left, &x.right]
    }
}

/// One transition of a click measurement: its CSV name, the state both lanes of the strip are
/// prepared in, and the record that changes it.
pub(crate) struct Transition {
    pub(crate) name: &'static str,
    pub(crate) initial: ChannelParameters,
    pub(crate) control: Control,
}

fn mute_click(rate: u32, m: &Material) -> String {
    let lane = |muted| ChannelParameters {
        muted,
        ..ChannelParameters::default()
    };
    click_rows(
        rate,
        m,
        &[
            Transition {
                name: "mute",
                initial: lane(false),
                control: Control::Mute(true),
            },
            Transition {
                name: "unmute",
                initial: lane(true),
                control: Control::Mute(false),
            },
        ],
        &MUTE_RAMPS_MS,
    )
}

/// The click of one gain switch per transition and ramp length, over every event of `m`: the
/// `mute_click.csv` row format, also used for the polarity flip (`polarity_click.csv`).
pub(crate) fn click_rows(
    rate: u32,
    m: &Material,
    transitions: &[Transition],
    ramps: &[f32],
) -> String {
    let mut out = String::new();
    let a = Analyzer::new(rate);
    let pre = align_up(a.samples(MUTE_PRE_S));
    let len = pre + a.samples(MUTE_POST_S);
    let edge = a.samples(TAPER_S);
    let s = Splatter::new(rate, len, a.samples(0.08));
    let (first, last) = (pre - a.samples(MUTE_BEFORE_S), pre + a.samples(MUTE_AFTER_S));
    // Decompose each event's excerpt once; every transition and ramp reuses it.
    let prepared: Vec<(Stereo, Vec<Lane>)> = m
        .events
        .iter()
        .map(|&e| {
            let x = m.source.slice(e - pre, e + len - pre);
            let parts = lanes(&x)
                .into_iter()
                .map(|lane| {
                    let xt = taper(lane, edge);
                    let parts = s.decompose(&xt);
                    (xt, parts)
                })
                .collect();
            (x, parts)
        })
        .collect();
    for transition in transitions {
        for &ms in ramps {
            let n = ms_to_samples(ms, rate);
            let events = [Event {
                at: pre,
                control: transition.control,
                smoothing_samples: n,
            }];
            let (gain, _) = probe_with(rate, transition.initial, len, &events);
            let g = widen(&gain);
            let mut sum = Tally::default();
            let mut worst_oob = f64::NEG_INFINITY;
            let mut above = 0_u32;
            for (x, parts) in &prepared {
                let mut y = x.clone();
                Strip::with(rate, transition.initial).render(&mut y.left, &mut y.right, &events);
                assert_engine_is_gain(&x.left, &gain, &y.left);
                assert_engine_is_gain(&x.right, &gain, &y.right);
                let mut t = Tally::default();
                for (xt, bands) in parts {
                    let yt: Vec<f64> = xt.iter().zip(&g).map(|(v, g)| v * g).collect();
                    let c = s.splatter(bands, &g);
                    t.add(a.span((xt, &yt, &c), first, last, m.oob_hz));
                }
                worst_oob = worst_oob.max(t.oob_db());
                above += u32::from(t.ctr_max > 1.0);
                sum.add(t);
            }
            let (oob, oob_worst, floor, ctr, above, edge_hz) = match m.oob_hz {
                Some(edge) => (
                    fmt(sum.oob_db()),
                    fmt(worst_oob),
                    fmt(sum.floor_db()),
                    fmt(sum.ctr_db()),
                    above.to_string(),
                    format!("{edge:.0}"),
                ),
                None => Default::default(),
            };
            let _ = writeln!(
                out,
                "{rate},{},{},{ms},{n},{},{},{},{oob},{oob_worst},{floor},{edge_hz},{ctr},{above}",
                m.name,
                transition.name,
                prepared.len(),
                fmt(sum.splatter_db()),
                fmt(sum.hf_db()),
            );
        }
    }
    out
}

/// What-if, NOT the engine: the same mute with a raised-cosine ramp of the same length, beside the
/// engine's linear ramp, on the band-limited materials. It sizes a possible kernel successor; no
/// default in FINDINGS.md rests on it.
fn mute_shape_whatif(rate: u32, m: &Material) -> String {
    let mut out = String::new();
    let Some(oob) = m.oob_hz else {
        return out;
    };
    let a = Analyzer::new(rate);
    let pre = align_up(a.samples(MUTE_PRE_S));
    let len = pre + a.samples(MUTE_POST_S);
    let edge = a.samples(TAPER_S);
    let (first, last) = (pre - a.samples(MUTE_BEFORE_S), pre + a.samples(MUTE_AFTER_S));
    let silent = vec![0.0; len];
    for &ms in &MUTE_RAMPS_MS[1..] {
        let n = ms_to_samples(ms, rate) as usize;
        let events = [Event {
            at: pre,
            control: Control::Mute(true),
            smoothing_samples: n as u32,
        }];
        let (linear, _) = probe(rate, false, len, &events);
        let linear = widen(&linear);
        let cosine: Vec<f64> = (0..len)
            .map(|i| match i.checked_sub(pre) {
                None => 1.0,
                Some(k) if k < n => 0.5 + 0.5 * (std::f64::consts::PI * (k + 1) as f64 / n as f64).cos(),
                Some(_) => 0.0,
            })
            .collect();
        for (shape, g) in [("engine-linear", &linear), ("raised-cosine", &cosine)] {
            let mut sum = Tally::default();
            for &e in &m.events {
                let x = m.source.slice(e - pre, e + len - pre);
                let xt = taper(&x.left, edge);
                let y: Vec<f64> = xt.iter().zip(g).map(|(v, g)| v * g).collect();
                sum.add(a.span((&xt, &y, &silent), first, last, Some(oob)));
            }
            let _ = writeln!(
                out,
                "{rate},{},{ms},{n},{shape},{},{}",
                m.name,
                fmt(sum.oob_db()),
                fmt(sum.ctr_db()),
            );
        }
    }
    out
}

/// Time from the block boundary that applies the record to the first sample at `level` or past it.
fn time_to(gain: &[f32], from: usize, rate: u32, reached: impl Fn(f32) -> bool) -> f64 {
    let index = (from..gain.len())
        .find(|&i| reached(gain[i]))
        .expect("level reached");
    (index - from) as f64 * 1000.0 / f64::from(rate)
}

fn mute_timing(rate: u32) -> String {
    let mut out = String::new();
    let j = align_up(rate as usize / 10);
    let frames = j + rate as usize;
    for &ms in &MUTE_RAMPS_MS {
        let n = ms_to_samples(ms, rate);
        let at = |muted: bool| {
            [Event {
                at: j,
                control: Control::Mute(muted),
                smoothing_samples: n,
            }]
        };
        let (down, _) = probe(rate, false, frames, &at(true));
        let (up, _) = probe(rate, true, frames, &at(false));
        let l = |v: f64| 10.0_f64.powf(v / 20.0) as f32;
        let _ = writeln!(
            out,
            "{rate},{ms},{n},{},{},{},{},{},{}",
            fmt(time_to(&down, j, rate, |g| g <= l(-20.0))),
            fmt(time_to(&down, j, rate, |g| g <= l(-40.0))),
            fmt(time_to(&down, j, rate, |g| g == 0.0)),
            fmt(time_to(&up, j, rate, |g| g >= l(-3.0))),
            fmt(time_to(&up, j, rate, |g| g >= l(-0.5))),
            fmt(time_to(&up, j, rate, |g| g == 1.0)),
        );
    }
    out
}

/// A there-and-back move: `from` -> `to` over `duration`, hold, `to` -> `from` over `duration`.
#[derive(Clone, Copy)]
pub(crate) struct Move {
    from: f64,
    to: f64,
    t0: f64,
    duration: f64,
    hold: f64,
}

impl Move {
    pub(crate) fn new(from: f64, to: f64, duration: f64) -> Self {
        Self {
            from,
            to,
            t0: 0.2,
            duration,
            hold: 0.2,
        }
    }
    pub(crate) fn value(&self, t: f64) -> f64 {
        let (t0, d, h) = (self.t0, self.duration, self.hold);
        let shape = if t <= t0 {
            0.0
        } else if t <= t0 + d {
            (t - t0) / d
        } else if t <= t0 + d + h {
            1.0
        } else if t <= t0 + 2.0 * d + h {
            1.0 - (t - t0 - d - h) / d
        } else {
            0.0
        };
        self.from + (self.to - self.from) * shape
    }
    fn segments(&self) -> [(f64, f64); 2] {
        let second = self.t0 + self.duration + self.hold;
        [
            (self.t0, self.t0 + self.duration),
            (second, second + self.duration),
        ]
    }
    fn end(&self) -> f64 {
        self.t0 + 2.0 * self.duration + self.hold
    }
    /// Excerpt length: the move plus 300 ms of tail.
    pub(crate) fn frames(&self, rate: u32) -> usize {
        ((self.end() + 0.3) * f64::from(rate)).round() as usize
    }
    /// The UI's updates: the hand position sampled every `1 / hz` from `t0` until the first
    /// sample at or after the end, each admitted at the next whole sample.
    pub(crate) fn updates(
        &self,
        rate: u32,
        hz: f64,
        n: u32,
        control: impl Fn(f64) -> Control,
    ) -> Vec<Event> {
        let mut events = Vec::new();
        let mut k = 0_u32;
        loop {
            let t = self.t0 + f64::from(k) / hz;
            events.push(Event {
                at: (t * f64::from(rate)).ceil() as usize,
                control: control(self.value(t)),
                smoothing_samples: n,
            });
            if t >= self.end() {
                break events;
            }
            k += 1;
        }
    }
    /// Analysis frame centres: each move plus 60 ms for a lagging ramp to finish.
    pub(crate) fn frame_spans(&self, a: &Analyzer) -> Vec<(usize, usize)> {
        self.segments()
            .iter()
            .map(|&(s, e)| (a.samples(s), a.samples(e + 0.06)))
            .collect()
    }
    fn tracking_windows(&self) -> Vec<(f64, f64)> {
        self.segments().iter().map(|&(a, b)| (a, b + 0.1)).collect()
    }
    /// The steady part of each move once a lag of `tau` is removed.
    fn inner_windows(&self, tau: f64) -> Vec<(f64, f64)> {
        self.segments()
            .iter()
            .map(|&(a, b)| (a + tau + 0.05, b + tau - 0.05))
            .collect()
    }
}

/// Least-squares delay of `track` (per-sample values) behind `reference(t)` over `windows`.
fn lag(track: &[f64], reference: impl Fn(f64) -> f64, windows: &[(f64, f64)], rate: u32) -> f64 {
    let r = f64::from(rate);
    let cost = |tau: f64, stride: usize| {
        let mut sum = 0.0;
        for &(a, b) in windows {
            let mut i = (a * r) as usize;
            while (i as f64) < b * r && i < track.len() {
                let e = track[i] - reference(i as f64 / r - tau);
                sum += e * e;
                i += stride;
            }
        }
        sum
    };
    let coarse_stride = (rate / 2000) as usize;
    let mut best = (f64::INFINITY, 0.0);
    let mut step = 0_u32;
    while f64::from(step) * 0.00025 <= 0.150 {
        let tau = f64::from(step) * 0.00025;
        let c = cost(tau, coarse_stride);
        if c < best.0 {
            best = (c, tau);
        }
        step += 1;
    }
    let centre = best.1;
    let mut fine = (f64::INFINITY, centre);
    let span = (0.0005 * r) as i64;
    for offset in -span..=span {
        let tau = (centre + offset as f64 / r).max(0.0);
        let c = cost(tau, 2);
        if c < fine.0 {
            fine = (c, tau);
        }
    }
    fine.1
}

/// Time after `end` until `track` stays within `tolerance` of `target`.
fn settle(track: &[f64], target: f64, tolerance: f64, end: f64, rate: u32) -> f64 {
    let last_out = (0..track.len())
        .rev()
        .find(|&i| (track[i] - target).abs() > tolerance)
        .map_or(0, |i| i + 1);
    (last_out as f64 / f64::from(rate) - end).max(0.0) * 1000.0
}

fn fader_drag(rate: u32, m: &Material, hz: f64) -> String {
    let mut out = String::new();
    let a = Analyzer::new(rate);
    let r = f64::from(rate);
    let edge = a.samples(TAPER_S);
    let offset = if m.name == "mix" { a.samples(2.0) } else { 0 };
    for &duration in &MOVES_S {
        let mv = Move::new(0.0, FADER_LOW_DB, duration);
        let frames = mv.frames(rate);
        let s = Splatter::new(rate, frames, a.samples(0.1));
        let x = m.source.slice(offset, offset + frames);
        let prepared: Vec<(Vec<f64>, Vec<Vec<f64>>)> = lanes(&x)
            .into_iter()
            .map(|lane| {
                let xt = taper(lane, edge);
                let parts = s.decompose(&xt);
                (xt, parts)
            })
            .collect();
        let spans = mv.frame_spans(&a);
        let tally = |g: &[f64]| {
            let mut t = Tally::default();
            for (xt, parts) in &prepared {
                let y: Vec<f64> = xt.iter().zip(g).map(|(v, g)| v * g).collect();
                let c = s.splatter(parts, g);
                for &(first, last) in &spans {
                    t.add(a.span((xt, &y, &c), first, last, m.oob_hz));
                }
            }
            t
        };
        let ideal_gain: Vec<f64> = (0..frames)
            .map(|i| 10.0_f64.powf(mv.value(i as f64 / r) / 20.0))
            .collect();
        let ideal = tally(&ideal_gain);
        let mut zero_lag = 0.0;
        for &ms in &DRAG_RAMPS_MS {
            let n = ms_to_samples(ms, rate);
            let events = mv.updates(rate, hz, n, |v| Control::FaderDb(v as f32));
            let (gain, _) = probe(rate, false, frames, &events);
            let mut y = x.clone();
            Strip::new(rate, false).render(&mut y.left, &mut y.right, &events);
            assert_engine_is_gain(&x.left, &gain, &y.left);
            assert_engine_is_gain(&x.right, &gain, &y.right);
            let g = widen(&gain);
            let t = tally(&g);
            let gdb: Vec<f64> = g.iter().map(|&v| 20.0 * v.log10()).collect();
            let hand = |t: f64| mv.value(t);
            let tau = lag(&gdb, hand, &mv.tracking_windows(), rate);
            if ms == 0.0 {
                zero_lag = tau;
            }
            let (mut sum, mut count, mut peak) = (0.0, 0.0, 0.0_f64);
            for &(s0, s1) in &mv.inner_windows(tau) {
                let mut i = (s0 * r) as usize;
                while (i as f64) < s1 * r {
                    let e = gdb[i] - hand(i as f64 / r - tau);
                    sum += e * e;
                    count += 1.0;
                    peak = peak.max(e.abs());
                    i += 1;
                }
            }
            let (oob, ideal_oob, ctr, ideal_ctr) = match m.oob_hz {
                Some(_) => (
                    fmt(t.oob_db()),
                    fmt(ideal.oob_db()),
                    fmt(t.ctr_db()),
                    fmt(ideal.ctr_db()),
                ),
                None => Default::default(),
            };
            let _ = writeln!(
                out,
                "{rate},{},{hz},{},{ms},{n},{},{},{},{},{oob},{ideal_oob},{ctr},{ideal_ctr},{:.3},\
                 {:.3},{},{},{},{}",
                m.name,
                duration * 1000.0,
                fmt(t.splatter_db()),
                fmt(ideal.splatter_db()),
                fmt(t.hf_db()),
                fmt(ideal.hf_db()),
                (sum / count).sqrt(),
                peak,
                fmt(tau * 1000.0),
                fmt((tau - zero_lag) * 1000.0),
                fmt(settle(&gdb, 0.0, 0.5, mv.end(), rate)),
                fmt(settle(&gdb, 0.0, 0.1, mv.end(), rate)),
            );
        }
    }
    out
}

fn pan_gains(p: f64) -> (f64, f64) {
    let theta = (p + 1.0) * std::f64::consts::FRAC_PI_4;
    (theta.cos(), theta.sin())
}

fn pan_drag(rate: u32, hz: f64) -> String {
    let mut out = String::new();
    let a = Analyzer::new(rate);
    let r = f64::from(rate);
    let edge = a.samples(TAPER_S);
    for &duration in &MOVES_S {
        let mv = Move::new(PAN_FROM, -PAN_FROM, duration);
        let frames = mv.frames(rate);
        let s = Splatter::new(rate, frames, a.samples(0.1));
        let bass: Vec<f32> = material::bass(rate, frames)
            .iter()
            .map(|&v| v as f32)
            .collect();
        let xt = taper(&bass, edge);
        let parts = s.decompose(&xt);
        let spans = mv.frame_spans(&a);
        let tally = |ll: &[f64], rl: &[f64]| {
            let mut t = Tally::default();
            for g in [ll, rl] {
                let y: Vec<f64> = xt.iter().zip(g).map(|(v, g)| v * g).collect();
                let c = s.splatter(&parts, g);
                for &(first, last) in &spans {
                    t.add(a.span((&xt, &y, &c), first, last, Some(BASS_OOB_HZ)));
                }
            }
            t
        };
        let (ideal_ll, ideal_rl): (Vec<f64>, Vec<f64>) = (0..frames)
            .map(|i| pan_gains(mv.value(i as f64 / r)))
            .unzip();
        let ideal = tally(&ideal_ll, &ideal_rl);
        let mut zero_lag = 0.0;
        for &ms in &DRAG_RAMPS_MS {
            let n = ms_to_samples(ms, rate);
            let mut events = vec![Event {
                at: 0,
                control: Control::Pan(PAN_FROM as f32),
                smoothing_samples: 0,
            }];
            events.extend(mv.updates(rate, hz, n, |v| Control::Pan(v as f32)));
            let (ll, rl) = probe(rate, false, frames, &events);
            let mut left = bass.clone();
            let mut right = vec![0.0_f32; frames];
            Strip::new(rate, false).render(&mut left, &mut right, &events);
            assert_engine_is_gain(&bass, &ll, &left);
            assert_engine_is_gain(&bass, &rl, &right);
            let (ll, rl) = (widen(&ll), widen(&rl));
            let t = tally(&ll, &rl);
            let position: Vec<f64> = ll
                .iter()
                .zip(&rl)
                .map(|(&c, &s)| s.atan2(c) / std::f64::consts::FRAC_PI_4 - 1.0)
                .collect();
            let hand = |t: f64| mv.value(t);
            let tau = lag(&position, hand, &mv.tracking_windows(), rate);
            if ms == 0.0 {
                zero_lag = tau;
            }
            // Per-channel gain error in dB against the continuous law, after the lag.
            let (mut sum, mut count, mut peak) = (0.0, 0.0, 0.0_f64);
            for &(s0, s1) in &mv.inner_windows(tau) {
                let mut i = (s0 * r) as usize;
                while (i as f64) < s1 * r {
                    let (gl, gr) = pan_gains(hand(i as f64 / r - tau));
                    for (actual, wanted) in [(ll[i], gl), (rl[i], gr)] {
                        let err = 20.0 * (actual / wanted).log10();
                        sum += err * err;
                        count += 1.0;
                        peak = peak.max(err.abs());
                    }
                    i += 1;
                }
            }
            let power_dev = (0..frames)
                .map(|i| db(ll[i] * ll[i] + rl[i] * rl[i]).abs())
                .fold(0.0_f64, f64::max);
            let _ = writeln!(
                out,
                "{rate},bass,{hz},{},{ms},{n},{},{},{},{},{},{},{:.3},{:.3},{},{},{:.3}",
                duration * 1000.0,
                fmt(t.splatter_db()),
                fmt(ideal.splatter_db()),
                fmt(t.oob_db()),
                fmt(ideal.oob_db()),
                fmt(t.ctr_db()),
                fmt(ideal.ctr_db()),
                (sum / count).sqrt(),
                peak,
                fmt(tau * 1000.0),
                fmt((tau - zero_lag) * 1000.0),
                power_dev,
            );
        }
    }
    out
}

fn pan_jump(rate: u32) -> String {
    let mut out = String::new();
    let j = align_up(rate as usize / 10);
    let frames = j + rate as usize;
    for (from, to) in [(-1.0_f32, 1.0_f32), (0.0, -1.0), (-0.5, 0.5)] {
        for &ms in &DRAG_RAMPS_MS {
            let n = ms_to_samples(ms, rate);
            let events = [
                Event {
                    at: 0,
                    control: Control::Pan(from),
                    smoothing_samples: 0,
                },
                Event {
                    at: j,
                    control: Control::Pan(to),
                    smoothing_samples: n,
                },
            ];
            let (ll, rl) = probe(rate, false, frames, &events);
            let power: Vec<f64> = (0..frames)
                .map(|i| db(f64::from(ll[i]).powi(2) + f64::from(rl[i]).powi(2)))
                .collect();
            let minimum = power.iter().copied().fold(f64::INFINITY, f64::min);
            let below = power.iter().filter(|&&p| p < -1.0).count();
            let _ = writeln!(
                out,
                "{rate},{from},{to},{ms},{n},{:.3},{}",
                minimum,
                fmt(below as f64 * 1000.0 / f64::from(rate)),
            );
        }
    }
    out
}

/// RMS and peak of each material, so the calibration behind `ctr` can be restated in SPL.
fn material_levels(rate: u32, mats: &[Material]) -> String {
    let mut out = String::new();
    for m in mats {
        let samples = m.source.left.iter().chain(&m.source.right);
        let (mut sum, mut peak, mut count) = (0.0, 0.0_f64, 0.0);
        for &v in samples {
            sum += f64::from(v) * f64::from(v);
            peak = peak.max(f64::from(v).abs());
            count += 1.0;
        }
        let rms_db = db(sum / count);
        let _ = writeln!(
            out,
            "{rate},{},{},{},{}",
            m.name,
            fmt(rms_db),
            fmt(20.0 * peak.log10()),
            fmt(rms_db + crate::analysis::CALIBRATION_DB_SPL),
        );
    }
    out
}

const FILES: [(&str, &str); 7] = [
    (
        "materials.csv",
        "rate_hz,material,rms_dbfs,peak_dbfs,rms_db_spl_at_calibration",
    ),
    (
        "mute_click.csv",
        "rate_hz,material,transition,ramp_ms,ramp_samples,events,splatter_db,hf_splatter_db,oob_db,\
         oob_worst_db,oob_floor_db,oob_edge_hz,ctr_max_db,events_above_threshold",
    ),
    (
        "mute_timing.csv",
        "rate_hz,ramp_ms,ramp_samples,mute_to_minus20db_ms,mute_to_minus40db_ms,mute_to_zero_ms,\
         unmute_to_minus3db_ms,unmute_to_minus0p5db_ms,unmute_to_unity_ms",
    ),
    (
        "fader_drag.csv",
        "rate_hz,material,update_hz,move_ms,ramp_ms,ramp_samples,splatter_db,ideal_splatter_db,\
         hf_splatter_db,ideal_hf_splatter_db,oob_db,ideal_oob_db,ctr_max_db,ideal_ctr_max_db,\
         ripple_rms_db,ripple_peak_db,lag_ms,added_lag_ms,settle_0p5db_ms,settle_0p1db_ms",
    ),
    (
        "pan_drag.csv",
        "rate_hz,material,update_hz,move_ms,ramp_ms,ramp_samples,splatter_db,ideal_splatter_db,\
         oob_db,ideal_oob_db,ctr_max_db,ideal_ctr_max_db,gain_error_rms_db,gain_error_peak_db,\
         lag_ms,added_lag_ms,power_dev_max_db",
    ),
    (
        "pan_jump.csv",
        "rate_hz,from,to,ramp_ms,ramp_samples,power_min_db,time_below_minus1db_ms",
    ),
    (
        "mute_shape_whatif.csv",
        "rate_hz,material,ramp_ms,ramp_samples,shape,oob_db,ctr_max_db",
    ),
];

/// One unit of work: which file, its position in that file, and the rows it writes.
pub(crate) type Job<'a> = (usize, usize, Box<dyn Fn() -> String + Send + Sync + 'a>);

/// Runs `jobs` on up to twelve threads, in list order, and returns `(file, order, rows)` sorted by
/// file and order, so the output does not depend on which thread finished first.
pub(crate) fn execute(jobs: &[Job<'_>]) -> Vec<(usize, usize, String)> {
    let workers = std::thread::available_parallelism()
        .map_or(4, |n| n.get())
        .min(12);
    let next = AtomicUsize::new(0);
    let results = Mutex::new(Vec::new());
    eprintln!("{} jobs on {workers} threads", jobs.len());
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some((file, order, job)) = jobs.get(i) else {
                        break;
                    };
                    let rows = job();
                    results
                        .lock()
                        .expect("results")
                        .push((*file, *order, rows));
                    eprintln!("done {}/{}", i + 1, jobs.len());
                }
            });
        }
    });
    let mut results = results.into_inner().expect("results");
    results.sort_by_key(|r| (r.0, r.1));
    results
}

/// Writes each file of `files` (name, header) with the rows `execute` returned for its index.
pub(crate) fn write_files(out_dir: &Path, files: &[(&str, &str)], results: &[(usize, usize, String)]) {
    for (index, (name, header)) in files.iter().enumerate() {
        let mut body = format!("{header}\n");
        for (_, _, rows) in results.iter().filter(|r| r.0 == index) {
            body.push_str(rows);
        }
        fs::write(out_dir.join(name), body).expect("write csv");
    }
}

pub fn run(out_dir: &Path, rates: &[u32]) {
    fs::create_dir_all(out_dir).expect("output directory");
    eprintln!("synthesising material at {rates:?}");
    let per_rate: Vec<Vec<Material>> = std::thread::scope(|scope| {
        let handles: Vec<_> = rates
            .iter()
            .map(|&rate| scope.spawn(move || materials(rate)))
            .collect();
        handles.into_iter().map(|h| h.join().expect("material")).collect()
    });
    let mut jobs: Vec<Job<'_>> = Vec::new();
    for (ri, (&rate, mats)) in rates.iter().zip(&per_rate).enumerate() {
        let order = |i: usize| ri * 100 + i;
        jobs.push((0, order(0), Box::new(move || material_levels(rate, mats))));
        for (mi, m) in mats.iter().enumerate() {
            jobs.push((1, order(mi), Box::new(move || mute_click(rate, m))));
            if rate == 48_000 {
                jobs.push((6, order(mi), Box::new(move || mute_shape_whatif(rate, m))));
            }
            for (hi, &hz) in UPDATE_HZ.iter().enumerate() {
                jobs.push((3, order(mi * 10 + hi), Box::new(move || fader_drag(rate, m, hz))));
            }
        }
        jobs.push((2, order(0), Box::new(move || mute_timing(rate))));
        for (hi, &hz) in UPDATE_HZ.iter().enumerate() {
            jobs.push((4, order(hi), Box::new(move || pan_drag(rate, hz))));
        }
        jobs.push((5, order(0), Box::new(move || pan_jump(rate))));
    }
    // Longest jobs first keeps the pool busy; `execute` restores the output order.
    jobs.sort_by_key(|job| std::cmp::Reverse(usize::from(job.0 == 3)));
    let results = execute(&jobs);
    write_files(out_dir, &FILES, &results);
}
