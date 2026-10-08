//! #1457 gate 2: the preparation cost of the input-section design bounds, descriptive.
//!
//! One invocation, release, one pinned core:
//!
//! ```text
//! CARGO_INCREMENTAL=0 cargo build --release -p builtins --features test-support \
//!     --example input_bound_budget
//! taskset -c 7 target/release/examples/input_bound_budget            # gate 2
//! taskset -c 7 target/release/examples/input_bound_budget calibrate  # the charge constants
//! ```
//!
//! Each workload runs one warmup round (round 0, not reported as a measurement) and two measured
//! rounds; nothing is retried or tuned. The numbers describe the box they ran on; the issue's
//! attempt record states which box stood in for the CI-class runner.
//!
//! * **Gate 2** (no argument). For each design family at the four launch rates: a first
//!   preparation (`input_section_bounds` with a fresh `InputBoundCache`), a rebuild with the same
//!   cache warm, and a preparation with no cache. It prints each one's milliseconds, the frames
//!   the walks really took, the bounds computed and the strips left on their exact bound. It
//!   asserts what the gate states: the three results are identical, a preparation walks at most
//!   the budget, and a warm rebuild walks no frame and computes no bound.
//!   It also prints each preparation's charge and its design work per frame-equivalent of budget
//!   consumed, which confirms the frame-equivalent against the families' real work: the consumed
//!   amount is the charge when every design is exact, and otherwise the budget, which a
//!   preparation that leaves a design on its live bound has spent to within one design's section
//!   charges.
//! * **Calibrate.** Two fixed-cost measurements, separate from gate 2, that set the charge
//!   constants. First, per design class (one or two channel cascades, one or two sections each),
//!   the least-squares line of one design's computation time against the frames it walks, over a
//!   grid of designs: its intercept is the class's fixed cost, and `INPUT_BOUND_SECTION_CHARGE` is
//!   the largest fixed cost per section in frame-equivalents. Then the frame classes (#1474 root
//!   ruling: every class): the time per walked frame, net of the fixed cost of one cascade of two
//!   sections, of long near-top walks (an HPF at 0.50-0.99 of the maximum, and one `f32` below
//!   it, into the LPF at the maximum, at 0 and +24 dB), of typical designs (an HPF at 20-80 Hz into
//!   an LPF at 16-20 kHz, at 0 and +24 dB) and of gate 2's cheap two-section designs (at 0, +12 and
//!   +24 dB); the slowest class defines the frame-equivalent (`FRAME_EQUIVALENT_NS`). Each point's
//!   time is the median of `CLASS_RUNS` measured sweeps after one warmup sweep, reported with its
//!   spread (minimum to maximum), so one noisy sample cannot move the class.
#![allow(missing_docs)]

use builtins::test_support::{
    fixed_input_bounds_computed, fixed_input_charged, fixed_input_frames_walked,
    input_section_bounds_within,
};
use builtins::{
    BuiltinParameters, ChannelParameters, INPUT_BOUND_BUDGET_FRAMES, InputBoundCache,
    builtin_filter_cutoff_maximum_hz, input_section_bounds, input_section_live_bound_table,
};
use effect_contract::NodeTailBound;
use std::time::Instant;

const RATES: [u32; 4] = [44_100, 48_000, 88_200, 96_000];
/// The frame-equivalent in ns: the time of one frame of the slowest frame class measured (#1457
/// Amendment 4, restated by #1474), as `builtins::INPUT_BOUND_SECTION_CHARGE`'s doc states it.
const FRAME_EQUIVALENT_NS: f64 = 17.0;
const ROUNDS: usize = 3;

fn channel(trim_db: f32, hpf_hz: f32, lpf_hz: f32) -> ChannelParameters {
    ChannelParameters {
        trim_db,
        hpf_hz,
        lpf_hz,
        ..ChannelParameters::default()
    }
}

fn strip(left: ChannelParameters, right: ChannelParameters) -> BuiltinParameters {
    BuiltinParameters {
        left,
        right,
        ..BuiltinParameters::default()
    }
}

fn same(trim_db: f32, hpf_hz: f32, lpf_hz: f32) -> BuiltinParameters {
    let both = channel(trim_db, hpf_hz, lpf_hz);
    strip(both, both)
}

fn below(value: f32, steps: u32) -> f32 {
    f32::from_bits(value.to_bits() - steps)
}

fn maximum(rate: u32) -> f32 {
    builtin_filter_cutoff_maximum_hz(rate).expect("a launch rate")
}

fn milliseconds<R>(run: impl FnOnce() -> R) -> (f64, R) {
    let start = Instant::now();
    let result = run();
    (start.elapsed().as_secs_f64() * 1e3, result)
}

/// One design family: a name and its strips at a rate.
struct Family {
    name: &'static str,
    strips: fn(u32) -> Vec<BuiltinParameters>,
}

/// `count` distinct designs: `base`'s LPF (or HPF, when it has no LPF) raised by 0.05 Hz per strip.
fn distinct(count: u32, trim_db: f32, hpf_hz: f32, lpf_hz: f32) -> Vec<BuiltinParameters> {
    (0..count)
        .map(|index| {
            let step = index as f32 * 0.05;
            if lpf_hz > 0.0 {
                same(trim_db, hpf_hz, lpf_hz + step)
            } else {
                same(trim_db, hpf_hz + step, lpf_hz)
            }
        })
        .collect()
}

/// `count` distinct designs in the slowest frame class (#1457 Amendment 4): the HPF at 0.778 of
/// the maximum, raised by 1 to `count` `f32` steps, into the LPF at the maximum, +24 dB.
fn band(rate: u32, count: u32) -> Vec<BuiltinParameters> {
    let hpf = 0.778 * maximum(rate);
    (1..=count)
        .map(|index| same(24.0, f32::from_bits(hpf.to_bits() + index), maximum(rate)))
        .collect()
}

const FAMILIES: [Family; 14] = [
    Family {
        name: "top pair (HPF one f32 below the maximum into the LPF at it, +24 dB)",
        strips: |rate| vec![same(24.0, below(maximum(rate), 1), maximum(rate))],
    },
    Family {
        name: "LPF at the maximum, +24 dB",
        strips: |rate| vec![same(24.0, 0.0, maximum(rate))],
    },
    Family {
        name: "64-design near-top family (HPF 10-640 Hz into the LPF 1-64 f32 below the maximum, +24 dB)",
        strips: |rate| {
            (1..=64)
                .map(|index| same(24.0, 10.0 * index as f32, below(maximum(rate), index)))
                .collect()
        },
    },
    Family {
        name: "65,537 distinct near-top designs (HPF 1-65,537 f32 below the maximum into the LPF at it, +24 dB)",
        strips: |rate| {
            (1..=65_537)
                .map(|index| same(24.0, below(maximum(rate), index), maximum(rate)))
                .collect()
        },
    },
    Family {
        name: "64 band designs (HPF at 0.778 of the maximum + 1-64 f32 steps into the LPF at it, +24 dB)",
        strips: |rate| band(rate, 64),
    },
    Family {
        name: "4,096 band designs (HPF at 0.778 of the maximum + 1-4,096 f32 steps into the LPF at it, +24 dB)",
        strips: |rate| band(rate, 4_096),
    },
    Family {
        name: "64 typical designs (20 Hz HPF + 0.05 Hz steps into a 20 kHz LPF, 0 dB)",
        strips: |_| {
            (0..64)
                .map(|index| same(0.0, 20.0 + index as f32 * 0.05, 20_000.0))
                .collect()
        },
    },
    Family {
        name: "256 typical designs (as above)",
        strips: |_| {
            (0..256)
                .map(|index| same(0.0, 20.0 + index as f32 * 0.05, 20_000.0))
                .collect()
        },
    },
    Family {
        name: "4,096 cheap one-section designs (1 kHz LPF + 0.05 Hz steps, 0 dB)",
        strips: |_| distinct(4_096, 0.0, 0.0, 1_000.0),
    },
    Family {
        name: "4,096 cheap two-section designs (1 kHz HPF into a 1.28 kHz LPF + steps, +24 dB)",
        strips: |_| distinct(4_096, 24.0, 1_000.0, 1_280.0),
    },
    Family {
        name: "4,096 cheap two-section designs (1.28 kHz HPF into a 5.12 kHz LPF + steps, +12 dB)",
        strips: |_| distinct(4_096, 12.0, 1_280.0, 5_120.0),
    },
    Family {
        name: "4,096 cheap two-section designs (2.56 kHz HPF into a 5.12 kHz LPF + steps, +24 dB)",
        strips: |_| distinct(4_096, 24.0, 2_560.0, 5_120.0),
    },
    Family {
        name: "4,096 cheap two-section designs (5.12 kHz HPF into a 10.24 kHz LPF + steps, 0 dB)",
        strips: |_| distinct(4_096, 0.0, 5_120.0, 10_240.0),
    },
    Family {
        name: "4,096 cheap two-cascade designs (left 1 kHz LPF + steps, right 1 kHz HPF, 0 dB)",
        strips: |_| {
            (0..4_096)
                .map(|index| {
                    strip(
                        channel(0.0, 0.0, 1_000.0 + index as f32 * 0.05),
                        channel(0.0, 1_000.0, 0.0),
                    )
                })
                .collect()
        },
    },
];

/// One preparation's figures.
struct Run {
    ms: f64,
    frames: u64,
    computed: u64,
    charged: u64,
    bounds: Vec<NodeTailBound>,
}

fn prepare(rate: u32, strips: &[BuiltinParameters], cache: Option<&mut InputBoundCache>) -> Run {
    let frames = fixed_input_frames_walked();
    let computed = fixed_input_bounds_computed();
    let charged = fixed_input_charged();
    let (ms, bounds) =
        milliseconds(|| input_section_bounds(rate, strips.iter().copied(), cache).expect("bounds"));
    Run {
        ms,
        frames: fixed_input_frames_walked() - frames,
        computed: fixed_input_bounds_computed() - computed,
        charged: fixed_input_charged() - charged,
        bounds,
    }
}

fn gate_two() {
    println!(
        "budget {INPUT_BOUND_BUDGET_FRAMES} frame-equivalents of {FRAME_EQUIVALENT_NS} ns ({:.2} ms); section charge {}",
        INPUT_BOUND_BUDGET_FRAMES as f64 * FRAME_EQUIVALENT_NS * 1e-6,
        builtins::INPUT_BOUND_SECTION_CHARGE
    );
    println!(
        "family | rate | round | first ms | rebuild ms | no-cache ms | design work ms | frames | computed | charged | ns per consumed frame-equivalent | exact"
    );
    let mut worst: Vec<(f64, String)> = Vec::new();
    let mut work: Vec<(f64, String)> = Vec::new();
    let mut per_charge: Vec<(f64, String)> = Vec::new();
    for family in &FAMILIES {
        for rate in RATES {
            let strips = (family.strips)(rate);
            let live = input_section_live_bound_table(rate).expect("a launch rate");
            for round in 0..ROUNDS {
                let mut cache = InputBoundCache::new();
                let first = prepare(rate, &strips, Some(&mut cache));
                let rebuild = prepare(rate, &strips, Some(&mut cache));
                let none = prepare(rate, &strips, None);
                assert_eq!(
                    first.bounds, none.bounds,
                    "{}: first and no cache",
                    family.name
                );
                assert_eq!(rebuild.bounds, none.bounds, "{}: rebuild", family.name);
                assert_eq!(
                    (rebuild.frames, rebuild.computed),
                    (0, 0),
                    "{}: rebuild",
                    family.name
                );
                assert_eq!(first.frames, none.frames, "{}: frames", family.name);
                assert!(
                    first.frames <= INPUT_BOUND_BUDGET_FRAMES,
                    "{}: walked",
                    family.name
                );
                let exact = none.bounds.iter().filter(|bound| **bound != live).count();
                let label = if round == 0 {
                    "warmup".to_owned()
                } else {
                    round.to_string()
                };
                let design_work = none.ms - rebuild.ms;
                let consumed = if exact == strips.len() {
                    none.charged
                } else {
                    INPUT_BOUND_BUDGET_FRAMES
                };
                let ns_per_charge = design_work * 1e6 / consumed as f64;
                println!(
                    "{} | {rate} | {label} | {:.3} | {:.3} | {:.3} | {design_work:.3} | {} | {} | {} | {ns_per_charge:.2} | {exact} / {}",
                    family.name,
                    first.ms,
                    rebuild.ms,
                    none.ms,
                    none.frames,
                    none.computed,
                    none.charged,
                    strips.len()
                );
                if round > 0 {
                    let what = format!("{} at {rate} Hz, round {round}", family.name);
                    worst.push((first.ms.max(none.ms), what.clone()));
                    work.push((design_work, what.clone()));
                    per_charge.push((ns_per_charge, what));
                }
            }
        }
    }
    for (heading, unit, mut list) in [
        ("preparation", "ms", worst),
        ("design work", "ms", work),
        (
            "design work per consumed frame-equivalent",
            "ns",
            per_charge,
        ),
    ] {
        list.sort_by(|a, b| b.0.total_cmp(&a.0));
        for (value, what) in list.iter().take(5) {
            println!("slowest {heading}: {value:.3} {unit}, {what}");
        }
    }
}

/// Designs that walk more frames than this are left out of the calibration's fit: the intercept
/// (the fixed cost) is read where the walk is short, so the per-frame slope barely moves it.
const CALIBRATION_FRAMES: f64 = 16_384.0;

/// A design class of the calibration (its name and the sections it walks) and its grid at a rate:
/// cutoffs on a 48-step geometric grid from 40 Hz to the maximum, trims of 0, +12 and +24 dB.
fn calibration_classes(rate: u32) -> [(&'static str, u64, Vec<BuiltinParameters>); 4] {
    let top = maximum(rate);
    let cutoffs: Vec<f32> = (0..48)
        .map(|step| (40.0 * math::powf(top / 40.0, step as f32 / 47.0)).min(below(top, 1)))
        .collect();
    let at = |index: usize| cutoffs[index % cutoffs.len()];
    let mut one = Vec::new();
    let mut two = Vec::new();
    let mut split_one = Vec::new();
    let mut split_two = Vec::new();
    for trim in [0.0_f32, 12.0, 24.0] {
        for index in 0..cutoffs.len() {
            let cutoff = at(index);
            one.push(same(trim, 0.0, cutoff));
            one.push(same(trim, cutoff, 0.0));
            for upper in index + 1..cutoffs.len() {
                two.push(same(trim, cutoff, at(upper)));
            }
            for step in [1, 5, 11, 23] {
                let other = at(index + step);
                split_one.push(strip(channel(trim, 0.0, cutoff), channel(trim, 0.0, other)));
                split_one.push(strip(channel(trim, cutoff, 0.0), channel(trim, other, 0.0)));
                let pair = |low: usize, high: usize| {
                    let (low, high) = (at(low), at(high));
                    channel(trim, low.min(high), low.max(high))
                };
                if index + step + 3 < cutoffs.len() {
                    split_two.push(strip(
                        pair(index, index + 3),
                        pair(index + step, index + step + 3),
                    ));
                }
            }
        }
    }
    [
        ("one cascade of one section", 1, one),
        ("one cascade of two sections", 2, two),
        ("two cascades of one section", 2, split_one),
        ("two cascades of two sections", 4, split_two),
    ]
}

/// The least-squares line `ms = intercept + slope * frames`.
fn line(points: &[(f64, f64)]) -> (f64, f64) {
    let count = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(x, y), p| (x + p.0, y + p.1));
    let (mx, my) = (sx / count, sy / count);
    let (sxy, sxx) = points.iter().fold((0.0, 0.0), |(xy, xx), p| {
        (xy + (p.0 - mx) * (p.1 - my), xx + (p.0 - mx) * (p.0 - mx))
    });
    let slope = sxy / sxx;
    (my - slope * mx, slope)
}

/// One design's computation alone: a one-strip preparation with a fresh cache and no budget limit
/// (its keying, the walk and the cache insertion). Milliseconds and frames walked.
fn one_design(rate: u32, design: BuiltinParameters) -> (f64, u64) {
    let mut cache = InputBoundCache::new();
    let frames = fixed_input_frames_walked();
    let (ms, _) = milliseconds(|| {
        input_section_bounds_within(rate, [design], u64::MAX, Some(&mut cache)).expect("bounds")
    });
    (ms, fixed_input_frames_walked() - frames)
}

/// Measured sweeps of the frame classes after the warmup sweep: each point's ns per frame is the
/// median of these, reported with its spread (#1474).
const CLASS_RUNS: usize = 5;

/// The frame-class points at a rate, grouped by class and trim (#1474 root ruling: every class).
/// Every point is one cascade of two sections, the same on both channels.
fn frame_class_points(rate: u32) -> Vec<(String, Vec<(String, BuiltinParameters)>)> {
    let top = maximum(rate);
    let mut groups = Vec::new();
    for trim in [0.0_f32, 24.0] {
        let points = (50..100)
            .map(|percent| (percent as f32 / 100.0, percent as f32 / 100.0 * top))
            .chain([(1.0, below(top, 1))])
            .map(|(fraction, hpf)| (format!("HPF {fraction:.2}"), same(trim, hpf, top)))
            .collect();
        groups.push((
            format!("near-top (HPF 0.50-0.99 and one f32 below the maximum into the LPF at it), {trim} dB"),
            points,
        ));
    }
    for trim in [0.0_f32, 24.0] {
        let mut points = Vec::new();
        for hpf in [20.0_f32, 40.0, 80.0] {
            for lpf in [16_000.0_f32, 18_000.0, 20_000.0] {
                points.push((format!("{hpf} Hz into {lpf} Hz"), same(trim, hpf, lpf)));
            }
        }
        groups.push((
            format!("typical (HPF 20-80 Hz into LPF 16-20 kHz), {trim} dB"),
            points,
        ));
    }
    for trim in [0.0_f32, 12.0, 24.0] {
        let points = [
            (1_000.0_f32, 1_280.0_f32),
            (1_280.0, 5_120.0),
            (2_560.0, 5_120.0),
            (5_120.0, 10_240.0),
        ]
        .into_iter()
        .map(|(hpf, lpf)| (format!("{hpf} Hz into {lpf} Hz"), same(trim, hpf, lpf)))
        .collect();
        groups.push((
            format!("cheap two-section (gate 2's families), {trim} dB"),
            points,
        ));
    }
    groups
}

/// The frame classes: the ns per walked frame of each point, net of its fixed cost (`fixed_ns`,
/// the rate's measured intercept of one cascade of two sections), so that short walks are compared
/// frame for frame with long ones. Each sweep measures every point once, so a burst of
/// interference reaches one sample of many points, not every sample of one; sweep 0 is the
/// warmup. Returns the slowest point's median.
fn frame_classes(fixed_ns: &[f64; 4]) -> f64 {
    println!(
        "rate | class | slowest point | its ns per frame net of the fixed cost, median (min-max) | fastest median | frames"
    );
    let mut slowest = (0.0_f64, String::new());
    for (slot, rate) in RATES.into_iter().enumerate() {
        for (class, points) in frame_class_points(rate) {
            let mut samples = vec![Vec::with_capacity(CLASS_RUNS); points.len()];
            let mut frames = vec![0_u64; points.len()];
            for sweep in 0..=CLASS_RUNS {
                for (point, (_, design)) in points.iter().enumerate() {
                    let (ms, walked) = one_design(rate, *design);
                    frames[point] = walked;
                    if sweep > 0 {
                        samples[point].push((ms * 1e6 - fixed_ns[slot]) / walked as f64);
                    }
                }
            }
            let medians: Vec<(usize, f64, f64, f64)> = samples
                .iter_mut()
                .enumerate()
                .map(|(point, runs)| {
                    runs.sort_by(f64::total_cmp);
                    (point, runs[CLASS_RUNS / 2], runs[0], runs[CLASS_RUNS - 1])
                })
                .collect();
            let high = medians
                .iter()
                .copied()
                .fold((0, 0.0_f64, 0.0, 0.0), |a, b| if b.1 > a.1 { b } else { a });
            let low = medians.iter().fold(f64::MAX, |low, p| low.min(p.1));
            let (low_frames, high_frames) = frames
                .iter()
                .fold((u64::MAX, 0), |(l, h), f| (l.min(*f), h.max(*f)));
            println!(
                "{rate} | {class} | {} | {:.2} ({:.2}-{:.2}) | {low:.2} | {low_frames}-{high_frames}",
                points[high.0].0, high.1, high.2, high.3
            );
            if high.1 > slowest.0 {
                slowest = (
                    high.1,
                    format!(
                        "{rate} Hz, {class}, {}, spread {:.2}-{:.2}",
                        points[high.0].0, high.2, high.3
                    ),
                );
            }
        }
    }
    println!(
        "slowest frame class measured: {:.2} ns per frame (median of {CLASS_RUNS}), {}",
        slowest.0, slowest.1
    );
    slowest.0
}

/// Per design class, the least-squares line of one design's time against its frames walked
/// (warmup round 0, then two measured rounds). Returns each rate's mean measured intercept of one
/// cascade of two sections (ns) and the largest measured fixed cost per section (ns).
fn fixed_costs() -> ([f64; 4], f64) {
    println!(
        "rate | round | class | designs | frames min-max | intercept us | ns per frame (slope) | intercept us per section"
    );
    let mut two_sections = [0.0_f64; 4];
    let mut per_section = 0.0_f64;
    for (slot, rate) in RATES.into_iter().enumerate() {
        for round in 0..ROUNDS {
            let label = if round == 0 {
                "warmup".to_owned()
            } else {
                round.to_string()
            };
            let mut intercepts = [0.0_f64; 4];
            let mut sections = [0_u64; 4];
            let mut largest = 0.0_f64;
            for (class, (name, walked, designs)) in
                calibration_classes(rate).into_iter().enumerate()
            {
                // Each design is a one-strip preparation with a fresh cache and no budget limit:
                // everything a preparation spends on one distinct design (its keying, the walk,
                // the cache insertion), however many strips share it.
                let points: Vec<(f64, f64)> = designs
                    .iter()
                    .map(|design| {
                        let (ms, frames) = one_design(rate, *design);
                        (frames as f64, ms)
                    })
                    .collect();
                let short: Vec<(f64, f64)> = points
                    .iter()
                    .copied()
                    .filter(|point| point.0 <= CALIBRATION_FRAMES)
                    .collect();
                let (intercept, slope) = line(&short);
                intercepts[class] = intercept;
                sections[class] = walked;
                let (low, high) = short.iter().fold((f64::MAX, 0.0_f64), |(low, high), p| {
                    (low.min(p.0), high.max(p.0))
                });
                println!(
                    "{rate} | {label} | {name} | {} of {} | {low:.0}-{high:.0} | {:.2} | {:.2} | {:.2}",
                    short.len(),
                    points.len(),
                    intercept * 1e3,
                    slope * 1e6,
                    intercept * 1e3 / walked as f64
                );
                largest = largest.max(intercept * 1e6 / walked as f64);
            }
            // One cascade of one section is `D + S`, of two sections `D + 2S`; the two-cascade
            // classes check the model (`D + 2S`, `D + 4S`).
            let section = intercepts[1] - intercepts[0];
            let design = intercepts[0] - section;
            let predicted: Vec<String> = (2..4)
                .map(|class| {
                    format!(
                        "{:.2} us predicted against {:.2} us",
                        (design + section * sections[class] as f64) * 1e3,
                        intercepts[class] * 1e3
                    )
                })
                .collect();
            println!(
                "{rate} | {label} | per design {:.2} us; per section {:.2} us; two cascades: {}; largest fixed cost per section {:.2} us",
                design * 1e3,
                section * 1e3,
                predicted.join(", "),
                largest * 1e-3
            );
            if round > 0 {
                two_sections[slot] += intercepts[1] * 1e6 / (ROUNDS - 1) as f64;
                per_section = per_section.max(largest);
            }
        }
    }
    (two_sections, per_section)
}

fn calibrate() {
    // The fixed costs first: the frame classes are read net of them.
    let (two_sections, per_section) = fixed_costs();
    let slowest = frame_classes(&two_sections);
    println!(
        "frame-equivalent {FRAME_EQUIVALENT_NS} ns (committed); slowest measured {slowest:.2} ns"
    );
    println!(
        "largest fixed cost per section {:.2} us: {:.1} frame-equivalents of the slowest class measured, {:.1} of the committed frame-equivalent",
        per_section * 1e-3,
        per_section / slowest,
        per_section / FRAME_EQUIVALENT_NS
    );
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        None => gate_two(),
        Some("calibrate") => calibrate(),
        Some(other) => {
            eprintln!("unknown argument {other:?}: expected none (gate 2) or `calibrate`");
            std::process::exit(2);
        }
    }
}
