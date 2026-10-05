//! The decision-15 addition to #1055 (questions 4-6): the live rows beyond fader, mute and pan.
//!
//! * `polarity_click.csv`: the polarity flip through the engine's own input stage
//!   (`InputBuiltins::set_polarity_invert`, i.e. `InputStage::set_trim_signed`, the D11 retarget of
//!   the signed trim coefficient through zero), measured exactly as `mute_click.csv` measures a
//!   mute (same events, frames and columns).
//! * `law_transfer.csv`: the live route's indexed ramp (`lane::kernels::IndexedRamp` and the
//!   shipped `route_mix_ramp_block`, driven block by block as `LiveRoute::drain` and
//!   `LiveRoute::mix` drive them), and the input trim's D11 retarget (`set_trim_db`), against the
//!   D11 fader, mute and matrix ramps the earlier measurements used, over the same moves and every
//!   measured length.
//! * `bypass_crossfade.csv`: #1341's crossfade (`crossfade.rs`) between the engine's shunt dry
//!   signal and each shunted effect's wet output, with the effects' own kernels.
//! * `link_glide.csv`: the same crossfade between the compressor's `dual_mono` and `maximum`
//!   outputs, the emulation of #1370's detector blend.
//!
//! A crossfade is `y = dry + m (wet - dry)`: the dry plane times a constant and the difference
//! `d = wet - dry` times the ramp `m`. A constant gain moves nothing, so the click is the splatter
//! of `m d` alone, `c = sum_k (I - W_k)(m d_k)` (`analysis::Splatter`). Every crossfade column is a
//! measure of `c`: `oob_db` and `ctr_max_db` read `c` above the out-of-band edge (for a mute, whose
//! `d` is the programme, this is the out-of-band energy of `y` that `mute_click.csv` reports, to
//! within frame leakage; the `mute-reference` rows show the two agree), `splatter_db` all of it, and
//! `hf_splatter_db` its 1.5-20 kHz part against the programme's own 1.5-20 kHz energy. Ratios are
//! relative to the reference programme: the effect's latency-matched input (the dry signal), or for
//! the link glide the compressor's input.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use builtins::{ChannelParameters, checked_fader_gain, pan_matrix};
use effect_contract::LinkMode;
use lane::kernels::{IndexedRamp, route_mix_ramp_block};

use crate::analysis::{Analyzer, Splatter, Tally, ms_to_samples, taper};
use crate::crossfade::{Toward, crossfade, mix_trajectory};
use crate::effects::{Effect, Setting, channel_correlation, link_name, shunt_dry};
use crate::material::Stereo;
use crate::measure::{
    self, BASS_OOB_HZ, Job, MUTE_AFTER_S, MUTE_BEFORE_S, MUTE_POST_S, MUTE_PRE_S, Material, Move,
    PAN_FROM, TAPER_S, Transition, align_up, click_rows, fmt, materials, widen,
};
use crate::strip::{Control, Event, QUANTUM, probe};

/// The brief's lengths plus 40 ms: twice each `muteMs` the preregistered rules can choose (5, 10
/// or 20 ms), so the flip at twice the mute length is measured for every one of them.
pub const POLARITY_RAMPS_MS: [f32; 7] = [0.0, 2.0, 5.0, 10.0, 20.0, 40.0, 50.0];
pub const BYPASS_RAMPS_MS: [f32; 5] = [0.0, 2.0, 5.0, 10.0, 20.0];
pub const LINK_RAMPS_MS: [f32; 6] = [0.0, 2.0, 5.0, 10.0, 20.0, 50.0];
/// Every length `mute_click.csv`, `fader_drag.csv` and `pan_drag.csv` measured.
pub const TRANSFER_RAMPS_MS: [f32; 11] =
    [0.0, 1.0, 2.0, 3.0, 5.0, 10.0, 15.0, 20.0, 30.0, 40.0, 50.0];
/// The right channel of `mix-wide` is the mix's right channel this much later: off the beat
/// grid, so the two channels' kicks alternate and the channels are uncorrelated.
const WIDE_OFFSET_S: f64 = 0.37;

const FILES: [(&str, &str); 4] = [
    (
        "polarity_click.csv",
        "rate_hz,material,transition,ramp_ms,ramp_samples,events,splatter_db,hf_splatter_db,oob_db,\
         oob_worst_db,oob_floor_db,oob_edge_hz,ctr_max_db,events_above_threshold",
    ),
    (
        "law_transfer.csv",
        "rate_hz,law,case,update_hz,ramp_ms,ramp_samples,records,max_abs_diff,max_ulp_diff,\
         both_settle_exactly,d11_bass_oob_db,law_bass_oob_db,d11_bass_ctr_max_db,\
         law_bass_ctr_max_db",
    ),
    (
        "bypass_crossfade.csv",
        "rate_hz,effect,setting,material,latency_samples,transition,ramp_ms,ramp_samples,events,\
         step_db,wet_oob_db,splatter_db,hf_splatter_db,oob_db,oob_worst_db,oob_edge_hz,ctr_max_db,\
         events_above_threshold",
    ),
    (
        "link_glide.csv",
        "rate_hz,material,channel_correlation,setting,transition,ramp_ms,ramp_samples,events,\
         step_db,splatter_db,hf_splatter_db,oob_db,oob_worst_db,oob_edge_hz,ctr_max_db,\
         events_above_threshold",
    ),
];

// ---- question 4: polarity ------------------------------------------------------------------

fn polarity_click(rate: u32, m: &Material) -> String {
    let lane = |polarity_invert| ChannelParameters {
        polarity_invert,
        ..ChannelParameters::default()
    };
    click_rows(
        rate,
        m,
        &[
            Transition {
                name: "invert",
                initial: lane(false),
                control: Control::Polarity(true),
            },
            Transition {
                name: "restore",
                initial: lane(true),
                control: Control::Polarity(false),
            },
        ],
        &POLARITY_RAMPS_MS,
    )
}

// ---- question 4: the route and trim laws against D11 ----------------------------------------------

/// One live route record: admitted at `at`, ramping to `target` over `length` samples.
type RouteRecord = (usize, [f32; 4], u32);

/// A unit probe (`left = 1`, `right = 0`) through one live route, driven as `LiveRoute` drives
/// it (`crates/graph/src/runtime.rs:896-930`): records drained at the top of the first block at or
/// after their admission, each starting an `IndexedRamp` from the coefficients the route is at;
/// `route_mix_ramp_block` at the native lane width; `position` advanced and saturated per block.
/// The outputs are the `ll` and `rl` coefficient trajectories.
fn route_probe(frames: usize, initial: [f32; 4], records: &[RouteRecord]) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![1.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    let mut ramp = IndexedRamp::settled(initial);
    let mut position = 0_u32;
    let mut next = 0;
    let mut start = 0;
    while start < frames {
        while next < records.len() && records[next].0 <= start {
            let (_, target, length) = records[next];
            ramp = IndexedRamp::new(ramp.coefficients_at(position), target, length);
            position = 0;
            next += 1;
        }
        let end = (start + QUANTUM).min(frames);
        route_mix_ramp_block::<lane::Native>(
            &mut left[start..end],
            &mut right[start..end],
            &ramp,
            position,
        );
        let advanced = u32::try_from(end - start).expect("block");
        position = position.saturating_add(advanced).min(ramp.length);
        start = end;
    }
    (left, right)
}

/// Distance in units in the last place between two `f32` values (`+0.0` and `-0.0` coincide).
fn ulps(a: f32, b: f32) -> u64 {
    let key = |x: f32| {
        let bits = i64::from(x.to_bits() & 0x7FFF_FFFF);
        if x.is_sign_negative() { -bits } else { bits }
    };
    key(a).abs_diff(key(b))
}

fn quad(m: builtins::Matrix2x2) -> [f32; 4] {
    [m.ll, m.lr, m.rl, m.rr]
}

/// Bass out-of-band energy and click-to-threshold ratio under one gain trajectory switching at
/// `j`, over the bass's 16 events: `mute_click.csv`'s method for those two columns, on the
/// trajectory's first `j` + 300 ms.
fn bass_switch_click(rate: u32, bass: &Material, gain: &[f32], j: usize) -> (f64, f64) {
    let a = Analyzer::new(rate);
    let edge = a.samples(TAPER_S);
    let (first, last) = (j - a.samples(MUTE_BEFORE_S), j + a.samples(MUTE_AFTER_S));
    let gain = &gain[..(j + a.samples(MUTE_POST_S)).min(gain.len())];
    let g = widen(gain);
    let silent = vec![0.0; gain.len()];
    let mut sum = Tally::default();
    for &e in &bass.events {
        let x = bass.source.slice(e - j, e - j + gain.len());
        let xt = taper(&x.left, edge);
        let y: Vec<f64> = xt.iter().zip(&g).map(|(v, g)| v * g).collect();
        sum.add(a.span((&xt, &y, &silent), first, last, bass.oob_hz));
    }
    (sum.oob_db(), sum.ctr_db())
}

/// The bass through a fader drag's gain trajectory: out-of-band energy and click-to-threshold
/// ratio over the move, `fader_drag.csv`'s method for those two columns.
fn bass_drag_click(rate: u32, bass: &Material, gain: &[f32], mv: &Move) -> (f64, f64) {
    let a = Analyzer::new(rate);
    let edge = a.samples(TAPER_S);
    let x = bass.source.slice(0, gain.len());
    let xt = taper(&x.left, edge);
    let g = widen(gain);
    let y: Vec<f64> = xt.iter().zip(&g).map(|(v, g)| v * g).collect();
    let silent = vec![0.0; gain.len()];
    let mut t = Tally::default();
    for (first, last) in mv.frame_spans(&a) {
        t.add(a.span((&xt, &y, &silent), first, last, bass.oob_hz));
    }
    (t.oob_db(), t.ctr_db())
}

/// One law-transfer case: the D11 reference trajectory (the fader, mute or matrix ramp the
/// earlier measurements used) and the same move under another law.
struct LawCase {
    law: &'static str,
    case: &'static str,
    update_hz: String,
    reference: (Vec<f32>, Vec<f32>),
    other: (Vec<f32>, Vec<f32>),
    records: usize,
    /// How to read the bass click under both trajectories: a switch at `j`, a fader drag, or
    /// nothing (pan, whose two outputs `pan_drag.csv` measures).
    bass: BassRead,
}

enum BassRead {
    Switch,
    Drag(Move),
    None,
}

/// `law_transfer.csv`: per move and length, the live route's indexed law and the input trim's
/// D11 retarget against the D11 ramp `mute_click.csv`, `fader_drag.csv` and `pan_drag.csv`
/// measured.
fn law_transfer(rate: u32, bass: &Material) -> String {
    let mut out = String::new();
    let j = align_up(rate as usize / 10);
    let jump_frames = j + align_up(rate as usize);
    let unity = [1.0, 0.0, 0.0, 1.0];
    let low_db = measure::FADER_LOW_DB as f32;
    let low = checked_fader_gain(low_db).expect("fader gain");
    let pan = |p: f32| quad(pan_matrix(p, p).expect("pan"));
    for &ms in &TRANSFER_RAMPS_MS {
        let n = ms_to_samples(ms, rate);
        let at = |control| Event {
            at: j,
            control,
            smoothing_samples: n,
        };
        let mut cases = vec![
            LawCase {
                law: "route-indexed",
                case: "mute",
                update_hz: String::new(),
                reference: probe(rate, false, jump_frames, &[at(Control::Mute(true))]),
                other: route_probe(jump_frames, unity, &[(j, [0.0; 4], n)]),
                records: 1,
                bass: BassRead::Switch,
            },
            LawCase {
                law: "route-indexed",
                case: "unmute",
                update_hz: String::new(),
                reference: probe(rate, true, jump_frames, &[at(Control::Mute(false))]),
                other: route_probe(jump_frames, [0.0; 4], &[(j, unity, n)]),
                records: 1,
                bass: BassRead::Switch,
            },
            LawCase {
                law: "route-indexed",
                case: "gain-0-to-minus24db",
                update_hz: String::new(),
                reference: probe(rate, false, jump_frames, &[at(Control::FaderDb(low_db))]),
                other: route_probe(jump_frames, unity, &[(j, [low, 0.0, 0.0, low], n)]),
                records: 1,
                bass: BassRead::Switch,
            },
            LawCase {
                law: "route-indexed",
                case: "pan-minus1-to-plus1",
                update_hz: String::new(),
                reference: probe(
                    rate,
                    false,
                    jump_frames,
                    &[
                        Event {
                            at: 0,
                            control: Control::Pan(-1.0),
                            smoothing_samples: 0,
                        },
                        at(Control::Pan(1.0)),
                    ],
                ),
                other: route_probe(jump_frames, pan(-1.0), &[(j, pan(1.0), n)]),
                records: 1,
                bass: BassRead::None,
            },
            LawCase {
                law: "input-trim",
                case: "gain-0-to-minus24db",
                update_hz: String::new(),
                reference: probe(rate, false, jump_frames, &[at(Control::FaderDb(low_db))]),
                other: probe(rate, false, jump_frames, &[at(Control::TrimDb(low_db))]),
                records: 1,
                bass: BassRead::Switch,
            },
        ];
        for &hz in &measure::UPDATE_HZ {
            let mv = Move::new(0.0, measure::FADER_LOW_DB, measure::MOVES_S[0]);
            let frames = mv.frames(rate);
            let fader = mv.updates(rate, hz, n, |v| Control::FaderDb(v as f32));
            let records: Vec<RouteRecord> = fader
                .iter()
                .map(|e| match e.control {
                    Control::FaderDb(db) => {
                        let g = checked_fader_gain(db).expect("fader gain");
                        (e.at, [g, 0.0, 0.0, g], n)
                    }
                    _ => unreachable!("fader updates"),
                })
                .collect();
            let trim = mv.updates(rate, hz, n, |v| Control::TrimDb(v as f32));
            let reference = probe(rate, false, frames, &fader);
            cases.push(LawCase {
                law: "route-indexed",
                case: "fader-drag-800ms",
                update_hz: format!("{hz}"),
                reference: reference.clone(),
                other: route_probe(frames, unity, &records),
                records: records.len(),
                bass: BassRead::Drag(mv),
            });
            cases.push(LawCase {
                law: "input-trim",
                case: "fader-drag-800ms",
                update_hz: format!("{hz}"),
                reference,
                other: probe(rate, false, frames, &trim),
                records: trim.len(),
                bass: BassRead::Drag(mv),
            });
            let mv = Move::new(PAN_FROM, -PAN_FROM, measure::MOVES_S[0]);
            let frames = mv.frames(rate);
            let mut events = vec![Event {
                at: 0,
                control: Control::Pan(PAN_FROM as f32),
                smoothing_samples: 0,
            }];
            events.extend(mv.updates(rate, hz, n, |v| Control::Pan(v as f32)));
            let records: Vec<RouteRecord> = events[1..]
                .iter()
                .map(|e| match e.control {
                    Control::Pan(p) => (e.at, pan(p), n),
                    _ => unreachable!("pan updates"),
                })
                .collect();
            cases.push(LawCase {
                law: "route-indexed",
                case: "pan-drag-800ms",
                update_hz: format!("{hz}"),
                reference: probe(rate, false, frames, &events),
                other: route_probe(frames, pan(PAN_FROM as f32), &records),
                records: records.len(),
                bass: BassRead::None,
            });
        }
        for c in cases {
            let (reference_l, reference_r) = &c.reference;
            let (other_l, other_r) = &c.other;
            let mut max_abs = 0.0_f64;
            let mut max_ulp = 0_u64;
            for (a, b) in reference_l
                .iter()
                .zip(other_l)
                .chain(reference_r.iter().zip(other_r))
            {
                max_abs = max_abs.max((f64::from(*a) - f64::from(*b)).abs());
                max_ulp = max_ulp.max(ulps(*a, *b));
            }
            let last = |v: &[f32]| v.last().map(|x| x.to_bits());
            let settle = last(reference_l) == last(other_l) && last(reference_r) == last(other_r);
            let clicks = match &c.bass {
                BassRead::Switch => Some((
                    bass_switch_click(rate, bass, reference_l, j),
                    bass_switch_click(rate, bass, other_l, j),
                )),
                BassRead::Drag(mv) => Some((
                    bass_drag_click(rate, bass, reference_l, mv),
                    bass_drag_click(rate, bass, other_l, mv),
                )),
                BassRead::None => None,
            };
            let (reference_oob, other_oob, reference_ctr, other_ctr) = match clicks {
                Some(((a_oob, a_ctr), (b_oob, b_ctr))) => {
                    (fmt(a_oob), fmt(b_oob), fmt(a_ctr), fmt(b_ctr))
                }
                None => Default::default(),
            };
            let _ = writeln!(
                out,
                "{rate},{},{},{},{ms},{n},{},{max_abs:.3e},{max_ulp},{settle},{reference_oob},\
                 {other_oob},{reference_ctr},{other_ctr}",
                c.law, c.case, c.update_hz, c.records,
            );
        }
    }
    out
}

// ---- questions 5 and 6: crossfades ---------------------------------------------------------

/// One switch between two programmes, aligned to the output: the reference the ratios are taken
/// against, and the dry and wet planes the crossfade mixes.
struct Pair<'a> {
    reference: &'a Stereo,
    dry: &'a Stereo,
    wet: &'a Stereo,
}

/// The measures of one transition at one ramp length, summed over events.
struct Crossfade {
    transition: &'static str,
    ms: f32,
    n: u32,
    sum: Tally,
    worst_oob: f64,
    above: u32,
}

/// Per event, both lanes unless all three planes are dual-mono.
fn crossfade_lanes(pair: &Pair<'_>) -> Vec<usize> {
    if pair.reference.is_dual_mono() && pair.dry.is_dual_mono() && pair.wet.is_dual_mono() {
        vec![0]
    } else {
        vec![0, 1]
    }
}

fn plane(x: &Stereo, lane: usize) -> &[f32] {
    if lane == 0 { &x.left } else { &x.right }
}

/// The emulated `f32` output must be what the analysis measures: exact copies at both ends and,
/// inside the ramp, `dry + m (wet - dry)` to within the three roundings of the `f32` mix.
fn assert_crossfade_is_measured(dry: &[f32], wet: &[f32], m: &[f32], y: &[f32]) {
    for i in 0..y.len() {
        let (d, w, mix) = (f64::from(dry[i]), f64::from(wet[i]), f64::from(m[i]));
        if mix == 1.0 || mix == 0.0 {
            let settled = if mix == 1.0 { wet[i] } else { dry[i] };
            assert_eq!(y[i].to_bits(), settled.to_bits(), "settled frame {i}");
        } else {
            let want = d + (w - d) * mix;
            let scale = d.abs().max(w.abs()).max(f64::MIN_POSITIVE);
            assert!(
                (f64::from(y[i]) - want).abs() <= scale * 4.0 * f64::from(f32::EPSILON),
                "ramp frame {i}"
            );
        }
    }
}

/// The click of a crossfade per transition and ramp length, over every event at the block-aligned
/// output positions `events`.
fn crossfade_measures(
    rate: u32,
    pair: &Pair<'_>,
    events: &[usize],
    oob: Option<f64>,
    transitions: &[(&'static str, Toward)],
    ramps: &[f32],
) -> (Tally, f64, Vec<Crossfade>) {
    let a = Analyzer::new(rate);
    let pre = align_up(a.samples(MUTE_PRE_S));
    let len = pre + a.samples(MUTE_POST_S);
    let edge = a.samples(TAPER_S);
    let s = Splatter::new(rate, len, a.samples(0.08));
    let (first, last) = (
        pre - a.samples(MUTE_BEFORE_S),
        pre + a.samples(MUTE_AFTER_S),
    );
    let weight = taper(&vec![1.0_f32; len], edge);
    let lanes = crossfade_lanes(pair);
    let silent = vec![0.0; len];
    let mut rows: Vec<Crossfade> = transitions
        .iter()
        .flat_map(|&(transition, _)| {
            ramps.iter().map(move |&ms| Crossfade {
                transition,
                ms,
                n: ms_to_samples(ms, rate),
                sum: Tally::default(),
                worst_oob: f64::NEG_INFINITY,
                above: 0,
            })
        })
        .collect();
    let trajectories: Vec<Vec<f32>> = transitions
        .iter()
        .flat_map(|&(_, toward)| {
            ramps
                .iter()
                .map(move |&ms| mix_trajectory(len, pre, toward, ms_to_samples(ms, rate)))
        })
        .collect();
    let mut step = Tally::default();
    let mut wet_oob = Tally::default();
    for &e in events {
        let (from, to) = (e - pre, e - pre + len);
        assert!(to <= pair.dry.left.len(), "event {e} past the material");
        let mut per_event: Vec<Tally> = rows.iter().map(|_| Tally::default()).collect();
        for &lane in &lanes {
            let reference = &plane(pair.reference, lane)[from..to];
            let dry = &plane(pair.dry, lane)[from..to];
            let wet = &plane(pair.wet, lane)[from..to];
            let rt = taper(reference, edge);
            let wt = taper(wet, edge);
            let dt: Vec<f64> = (0..len)
                .map(|i| (f64::from(wet[i]) - f64::from(dry[i])) * weight[i])
                .collect();
            step.add(a.span((&rt, &dt, &dt), first, last, oob));
            wet_oob.add(a.span((&rt, &wt, &silent), first, last, oob));
            let parts = s.decompose(&dt);
            for (index, row) in rows.iter().enumerate() {
                let m = &trajectories[index];
                let toward = transitions
                    .iter()
                    .find(|t| t.0 == row.transition)
                    .expect("transition")
                    .1;
                let y = crossfade(dry, wet, pre, toward, row.n);
                assert_crossfade_is_measured(dry, wet, m, &y);
                let c = s.splatter(&parts, &widen(m));
                per_event[index].add(a.span((&rt, &c, &c), first, last, oob));
            }
        }
        for (row, t) in rows.iter_mut().zip(per_event) {
            row.worst_oob = row.worst_oob.max(t.oob_db());
            row.above += u32::from(t.ctr_max > 1.0);
            row.sum.add(t);
        }
    }
    (step, wet_oob.oob_db(), rows)
}

/// `(oob_db, oob_worst_db, oob_edge_hz, ctr_max_db, events_above_threshold)`, empty without an
/// out-of-band region.
fn oob_columns(row: &Crossfade, oob: Option<f64>) -> (String, String, String, String, String) {
    match oob {
        Some(edge) => (
            fmt(row.sum.oob_db()),
            fmt(row.worst_oob),
            format!("{edge:.0}"),
            fmt(row.sum.ctr_db()),
            row.above.to_string(),
        ),
        None => Default::default(),
    }
}

const BYPASS_TRANSITIONS: [(&str, Toward); 2] =
    [("bypass", Toward::Dry), ("unbypass", Toward::Wet)];

/// One effect setting (or the mute reference, `effect = None`) on one material.
fn bypass(rate: u32, effect: Option<Effect>, m: &Material) -> String {
    let mut out = String::new();
    let (name, setting, latency, dry, wet) = match effect {
        Some(effect) => {
            let setting = Setting::calibrate(effect, rate, &m.source, LinkMode::DualMono);
            let latency = setting.latency(rate);
            let wet = setting.render(rate, &m.source);
            let dry = shunt_dry(&m.source, latency);
            (effect.name(), setting.describe(), latency, dry, wet)
        }
        None => {
            let frames = m.source.left.len();
            let silence = Stereo {
                left: vec![0.0; frames],
                right: vec![0.0; frames],
            };
            (
                "mute-reference",
                "wet=input dry=silence".to_owned(),
                0,
                silence,
                m.source.clone(),
            )
        }
    };
    let reference = if effect.is_some() { &dry } else { &m.source };
    let pair = Pair {
        reference,
        dry: &dry,
        wet: &wet,
    };
    // The record lands on the output's block boundary at each of the material's events.
    let (step, wet_oob, rows) = crossfade_measures(
        rate,
        &pair,
        &m.events,
        m.oob_hz,
        &BYPASS_TRANSITIONS,
        &BYPASS_RAMPS_MS,
    );
    let wet_oob = if m.oob_hz.is_some() {
        fmt(wet_oob)
    } else {
        String::new()
    };
    for row in &rows {
        let (oob, worst, edge, ctr, above) = oob_columns(row, m.oob_hz);
        let _ = writeln!(
            out,
            "{rate},{name},{setting},{},{latency},{},{},{},{},{},{wet_oob},{},{},{oob},{worst},\
             {edge},{ctr},{above}",
            m.name,
            row.transition,
            row.ms,
            row.n,
            m.events.len(),
            fmt(step.splatter_db()),
            fmt(row.sum.splatter_db()),
            fmt(row.sum.hf_db()),
        );
    }
    out
}

/// The link-glide materials: the mix as synthesised; the mix with its right channel taken
/// [`WIDE_OFFSET_S`] later (uncorrelated channels); and the bass in the left channel with the kick
/// in the right (uncorrelated, band-limited, so the out-of-band columns exist).
fn link_material(rate: u32, name: &'static str, mats: &[Material]) -> Material {
    let find = |wanted: &str| mats.iter().find(|m| m.name == wanted).expect("material");
    match name {
        "mix" => {
            let mix = find("mix");
            Material {
                name: "mix",
                source: mix.source.clone(),
                events: mix.events.clone(),
                oob_hz: None,
            }
        }
        "mix-wide" => {
            let mix = find("mix");
            let offset = (WIDE_OFFSET_S * f64::from(rate)).round() as usize;
            let frames = mix.source.left.len() - offset;
            Material {
                name: "mix-wide",
                source: Stereo {
                    left: mix.source.left[..frames].to_vec(),
                    right: mix.source.right[offset..].to_vec(),
                },
                events: mix.events.clone(),
                oob_hz: None,
            }
        }
        "bass-kick" => {
            let (bass, kick) = (find("bass"), find("kick"));
            let frames = bass.source.left.len().min(kick.source.left.len());
            Material {
                name: "bass-kick",
                source: Stereo {
                    left: bass.source.left[..frames].to_vec(),
                    right: kick.source.left[..frames].to_vec(),
                },
                events: kick
                    .events
                    .iter()
                    .copied()
                    .filter(|&e| e + align_up((MUTE_POST_S * f64::from(rate)) as usize) < frames)
                    .collect(),
                oob_hz: Some(BASS_OOB_HZ),
            }
        }
        _ => unreachable!("link material"),
    }
}

const LINK_TRANSITIONS: [(&str, Toward); 2] = [("link", Toward::Wet), ("unlink", Toward::Dry)];

/// The compressor at 6 dB of gain reduction (under `maximum`) switching between `dual_mono` (the
/// crossfade's dry plane) and `maximum` (its wet plane).
fn link_glide(rate: u32, name: &'static str, mats: &[Material]) -> String {
    let mut out = String::new();
    let m = link_material(rate, name, mats);
    let maximum = Setting::calibrate(Effect::Compressor, rate, &m.source, LinkMode::Maximum);
    let dual = Setting {
        link: LinkMode::DualMono,
        ..maximum.clone()
    };
    assert_eq!(maximum.latency(rate), 0, "the compressor adds no latency");
    let wet = maximum.render(rate, &m.source);
    let dry = dual.render(rate, &m.source);
    let pair = Pair {
        reference: &m.source,
        dry: &dry,
        wet: &wet,
    };
    let correlation = channel_correlation(&m.source, (0.5 * f64::from(rate)) as usize);
    let (step, _, rows) = crossfade_measures(
        rate,
        &pair,
        &m.events,
        m.oob_hz,
        &LINK_TRANSITIONS,
        &LINK_RAMPS_MS,
    );
    let setting = maximum.describe().replace(
        &format!("link={}", link_name(LinkMode::Maximum)),
        "link=dual_mono<->maximum",
    );
    for row in &rows {
        let (oob, worst, edge, ctr, above) = oob_columns(row, m.oob_hz);
        let _ = writeln!(
            out,
            "{rate},{},{correlation:.3},{setting},{},{},{},{},{},{},{},{oob},{worst},{edge},{ctr},\
             {above}",
            m.name,
            row.transition,
            row.ms,
            row.n,
            m.events.len(),
            fmt(step.splatter_db()),
            fmt(row.sum.splatter_db()),
            fmt(row.sum.hf_db()),
        );
    }
    out
}

pub fn run(out_dir: &Path, rates: &[u32]) {
    fs::create_dir_all(out_dir).expect("output directory");
    eprintln!("synthesising material at {rates:?}");
    let per_rate: Vec<Vec<Material>> = std::thread::scope(|scope| {
        let handles: Vec<_> = rates
            .iter()
            .map(|&rate| scope.spawn(move || materials(rate)))
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().expect("material"))
            .collect()
    });
    let mut jobs: Vec<Job<'_>> = Vec::new();
    for (ri, (&rate, mats)) in rates.iter().zip(&per_rate).enumerate() {
        let order = |i: usize| ri * 1000 + i;
        let find = move |name: &str| mats.iter().find(|m| m.name == name).expect("material");
        for (mi, m) in mats.iter().enumerate() {
            jobs.push((0, order(mi), Box::new(move || polarity_click(rate, m))));
        }
        jobs.push((
            1,
            order(0),
            Box::new(move || law_transfer(rate, find("bass"))),
        ));
        for (mi, m) in mats.iter().enumerate() {
            jobs.push((2, order(mi), Box::new(move || bypass(rate, None, m))));
        }
        for (ei, effect) in Effect::ALL.into_iter().enumerate() {
            for (mi, name) in effect.materials().iter().enumerate() {
                let m = find(name);
                jobs.push((
                    2,
                    order(100 + ei * 10 + mi),
                    Box::new(move || bypass(rate, Some(effect), m)),
                ));
            }
        }
        for (mi, name) in ["mix", "mix-wide", "bass-kick"].into_iter().enumerate() {
            jobs.push((3, order(mi), Box::new(move || link_glide(rate, name, mats))));
        }
    }
    // The crossfade jobs are the longest; starting them first keeps the pool busy.
    jobs.sort_by_key(|job| std::cmp::Reverse(usize::from(job.0 >= 2)));
    let results = measure::execute(&jobs);
    measure::write_files(out_dir, &FILES, &results);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The route probe is the shipped kernel's law: every ramping frame is `IndexedRamp`'s
    /// `c(k)`, `k` counted from the applying block's first frame, and the route ends exactly on
    /// its target. A probe that numbered frames from zero, or re-anchored at each block, would
    /// fail it.
    #[test]
    fn the_route_probe_mixes_by_the_indexed_law_from_the_applying_block() {
        let (at, n) = (2 * QUANTUM + 1, 300_u32);
        let applied = 3 * QUANTUM;
        let target = [0.25, 0.5, -0.75, 0.125];
        let (left, right) = route_probe(
            applied + 4 * QUANTUM,
            [1.0, 0.0, 0.0, 1.0],
            &[(at, target, n)],
        );
        let ramp = IndexedRamp::new([1.0, 0.0, 0.0, 1.0], target, n);
        assert!(left[..applied].iter().all(|&v| v == 1.0));
        for f in 0..n as usize {
            let c = ramp.coefficients_at(f as u32 + 1);
            assert_eq!(left[applied + f].to_bits(), c[0].to_bits(), "ll, frame {f}");
            assert_eq!(
                right[applied + f].to_bits(),
                c[2].to_bits(),
                "rl, frame {f}"
            );
        }
        assert!(
            left[applied + n as usize - 1..]
                .iter()
                .all(|&v| v == target[0])
        );
    }

    /// Every record restarts the ramp index from the coefficients the route has reached, as
    /// `LiveRoute::drain` does (`position = 0`): one record lands mid-ramp and one after the
    /// previous ramp has settled. A probe that kept the old position would start the second ramp
    /// part-way through and turn the third into a step to its target.
    #[test]
    fn a_retarget_restarts_the_index_from_where_the_route_is() {
        let unity = [1.0, 0.0, 0.0, 1.0];
        let (t1, n1) = ([0.5, 0.25, -0.5, 0.75], 600_u32);
        let (t2, n2) = ([-0.25, 0.5, 0.125, -1.0], 400_u32);
        let (t3, n3) = (unity, 300_u32);
        // Admitted inside blocks 2 and 8, so applied at the tops of blocks 3 and 9: the second
        // lands at index 384 of the 600-frame ramp, the third 368 frames after the second settled.
        let (a2, a3) = (2 * QUANTUM + 5, 8 * QUANTUM + 23);
        let (s2, s3) = (3 * QUANTUM, 9 * QUANTUM);
        let frames = s3 + n3 as usize + 2 * QUANTUM;
        let (left, right) = route_probe(frames, unity, &[(0, t1, n1), (a2, t2, n2), (a3, t3, n3)]);
        let r1 = IndexedRamp::new(unity, t1, n1);
        let r2 = IndexedRamp::new(r1.coefficients_at(s2 as u32), t2, n2);
        let r3 = IndexedRamp::new(r2.coefficients_at(n2), t3, n3);
        for f in 0..frames {
            let c = if f < s2 {
                r1.coefficients_at(f as u32 + 1)
            } else if f < s3 {
                r2.coefficients_at((f - s2) as u32 + 1)
            } else {
                r3.coefficients_at((f - s3) as u32 + 1)
            };
            assert_eq!(left[f].to_bits(), c[0].to_bits(), "ll, frame {f}");
            assert_eq!(right[f].to_bits(), c[2].to_bits(), "rl, frame {f}");
        }
        assert_eq!(left[frames - 1], t3[0]);
    }

    /// A polarity flip on the engine's input stage is the D11 ramp of the signed trim through
    /// zero: `1 - 2 (k + 1) / N` to within the accumulation's rounding, exactly `-1` from update
    /// `N` on, and twice a mute's slope at the same `N`.
    #[test]
    fn a_polarity_flip_is_the_trim_ramp_through_zero() {
        let n = 480_u32;
        let applied = 2 * QUANTUM;
        let events = [Event {
            at: applied,
            control: Control::Polarity(true),
            smoothing_samples: n,
        }];
        let (g, _) = crate::strip::probe_with(
            48_000,
            ChannelParameters::default(),
            applied + n as usize + QUANTUM,
            &events,
        );
        assert!(g[..applied].iter().all(|&v| v == 1.0));
        for k in 0..n as usize - 1 {
            let want = 1.0 - 2.0 * (k as f64 + 1.0) / f64::from(n);
            assert!((f64::from(g[applied + k]) - want).abs() < 1e-5, "k = {k}");
        }
        assert!(g[applied + n as usize - 1..].iter().all(|&v| v == -1.0));
    }

    /// The input trim and the fader share one ramp law bit for bit: the same moves in dB, over
    /// the same windows, give the same coefficient trajectory, so every fader measurement is a
    /// trim measurement too.
    #[test]
    fn trim_and_fader_ramps_are_bit_identical() {
        let mut fader = Vec::new();
        let mut trim = Vec::new();
        for (k, db) in [-6.0_f32, -24.0, 3.5, 0.0, -60.0, 12.0]
            .into_iter()
            .enumerate()
        {
            let at = k * 300 + 17;
            let n = [480, 960, 37, 0, 4800, 128][k];
            fader.push(Event {
                at,
                control: Control::FaderDb(db),
                smoothing_samples: n,
            });
            trim.push(Event {
                at,
                control: Control::TrimDb(db),
                smoothing_samples: n,
            });
        }
        let frames = 8000;
        let (by_fader, _) = probe(48_000, false, frames, &fader);
        let (by_trim, _) = probe(48_000, false, frames, &trim);
        assert!(
            by_fader
                .iter()
                .zip(&by_trim)
                .all(|(a, b)| a.to_bits() == b.to_bits())
        );
    }
}
