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
//! Each workload runs one warmup round (round 0, not reported as a measurement) and five measured
//! rounds, so that every design (a calibration batch, a frame-class batch or a gate-2 family at a
//! rate) has `REPEATS` = 5 measured samples and a median (root's ruling of 2026-10-08 on #1465);
//! nothing is retried or tuned. The numbers describe the box they ran on; the issue's attempt
//! record states which box stood in for the CI-class runner.
//!
//! * **Gate 2** (no argument). For each design family at the four launch rates: a first
//!   preparation (`input_section_bounds` with a fresh `InputBoundCache`), a rebuild with the same
//!   cache warm, and a preparation with no cache. It prints each one's milliseconds, the frames
//!   the walks really took, the bounds computed and the strips left on their exact bound. It
//!   asserts what the gate states: the three results are identical, a preparation walks at most
//!   the budget, and a warm rebuild walks no frame and computes no bound.
//!   It also prints each preparation's charge and its design work (the no-cache preparation's
//!   milliseconds less the warm rebuild's), and per family and rate the median design work of the
//!   five measured rounds per frame-equivalent of budget consumed, which confirms the
//!   frame-equivalent against the families' real work: the consumed amount is the charge when
//!   every design is exact, and otherwise the budget, which a preparation that leaves a design on
//!   its live bound has spent to within one design's section charges.
//! * **Calibrate.** Two fixed-cost measurements, separate from gate 2, that set the charge
//!   constants. Every sample is a **batch** (root's ruling (c) of 2026-10-08 on #1465): one
//!   preparation, with a fresh cache and no budget limit, of `BATCH` = 48 *different* designs of
//!   one class walked back to back in strip order, as a real preparation walks each distinct
//!   design once, so that a sample of the shortest designs (about 25-30 us a walk) lasts at least
//!   about 1 ms; the example asserts that every design of a batch is computed. A sample of one
//!   short walk moved the constant by about 10 % with the box's load. First, per design class (one
//!   or two channel cascades, one or two sections each), the grid's designs ordered by the frames
//!   each walks and cut into batches of 48 consecutive designs (the remainder folded into the last
//!   batch), and the least-squares line of each batch's median time per design against its frames
//!   per design: its intercept is the class's fixed cost, and `INPUT_BOUND_SECTION_CHARGE` is the
//!   largest fixed cost per section in frame-equivalents. Then the frame classes (#1474 root
//!   ruling: every class), each short point (typical and cheap) a batch of 48 different designs
//!   next to it (its HPF or LPF raised by 0.05 Hz a design, as gate 2's families step them) and
//!   each near-top point (a walk of over 5 ms) its own single walk: the time per walked frame, net of the
//!   fixed cost of one cascade of two sections, of long near-top walks (an HPF at 0.50-0.99 of the
//!   maximum, and one `f32` below it, into the LPF at the maximum, at 0 and +24 dB), of typical
//!   designs (an HPF at 20-80 Hz into an LPF at 16-20 kHz, at 0 and +24 dB) and of gate 2's cheap
//!   two-section designs (at 0, +12 and +24 dB), each point's median with its spread
//!   (descriptive). For evidence, each round also walks every design of a batch alone right after
//!   the batch, and calibrate prints the batch's median per walk over its designs' summed
//!   single-walk medians: a batch must not be cheaper per walk than a real preparation.
//!
//! The frame-equivalent (`FRAME_EQUIVALENT_NS`, root's ruling of 2026-10-08 on #1465, which
//! supersedes the same day's every-sample statistic) is the smallest value on a half-nanosecond
//! grid such that every design's median work is at most its charged frame-equivalents (frames
//! walked plus the section charge per section) times the frame-equivalent. For the batched
//! calibration, "every design" is every batch: its median work per design is at most its frames
//! per design plus the section charge times its sections per design, times the frame-equivalent
//! (the whole batch's median against the whole batch's charge); gate 2's families count each as
//! one design, at their own median. The section charge is the largest per-batch-median fixed cost
//! per section in frame-equivalents, rounded up to a ten, so the two are found together. The
//! computation is deterministic and interference only adds time, so a median bounds the cost the
//! budget states, while one preempted sample does not set the constant. Calibrate prints the
//! binding batch, its median work and its charged frame-equivalents per design, the shortest batch
//! sample, and the largest raw sample with its ratio to its batch's median (evidence); gate 2
//! prints the value its own medians need, and the committed value is the larger.
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
/// The frame-equivalent in ns: the smallest half nanosecond that bounds every design's median work
/// by its charged frame-equivalents, of the calibration and of gate 2 (root's ruling of 2026-10-08,
/// #1465), as `builtins::INPUT_BOUND_SECTION_CHARGE`'s doc states it.
const FRAME_EQUIVALENT_NS: f64 = 17.0;
/// Measured samples of each design, after one warmup (root's ruling of 2026-10-08, #1465: at
/// least five, and a design's cost is their median).
const REPEATS: usize = 5;
const ROUNDS: usize = 1 + REPEATS;

/// The median of a design's measured samples.
fn median(samples: &[f64]) -> f64 {
    let mut sorted = samples.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        sorted[middle]
    } else {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    }
}

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
    let budget_ms = INPUT_BOUND_BUDGET_FRAMES as f64 * FRAME_EQUIVALENT_NS * 1e-6;
    println!(
        "budget {INPUT_BOUND_BUDGET_FRAMES} frame-equivalents of {FRAME_EQUIVALENT_NS} ns ({budget_ms:.2} ms); section charge {}; {REPEATS} measured rounds",
        builtins::INPUT_BOUND_SECTION_CHARGE
    );
    println!(
        "family | rate | round | first ms | rebuild ms | no-cache ms | design work ms | frames | computed | charged | ns per consumed frame-equivalent | exact"
    );
    let mut worst: Vec<(f64, String)> = Vec::new();
    // Per family and rate: the median and the largest of the measured rounds' design work (ms),
    // the frame-equivalents consumed and what it is.
    let mut designs: Vec<(f64, f64, u64, String)> = Vec::new();
    for family in &FAMILIES {
        for rate in RATES {
            let strips = (family.strips)(rate);
            let live = input_section_live_bound_table(rate).expect("a launch rate");
            let mut works = Vec::with_capacity(REPEATS);
            let mut consumed_each = None;
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
                // The computation is deterministic: every round consumes the same amount.
                assert_eq!(
                    *consumed_each.get_or_insert(consumed),
                    consumed,
                    "{}: consumed",
                    family.name
                );
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
                    worst.push((first.ms.max(none.ms), what));
                    works.push(design_work);
                }
            }
            let largest = works.iter().fold(0.0_f64, |a, b| a.max(*b));
            designs.push((
                median(&works),
                largest,
                consumed_each.expect("a measured round"),
                format!("{} at {rate} Hz", family.name),
            ));
        }
    }
    println!(
        "family | median design work ms of {REPEATS} | largest ms | largest / median | ns per consumed frame-equivalent (median) | share of the budget (median)"
    );
    for (work, largest, consumed, what) in &designs {
        println!(
            "{what} | {work:.3} | {largest:.3} | {:.3} | {:.3} | {:.1} %",
            largest / work,
            work * 1e6 / *consumed as f64,
            work / budget_ms * 1e2
        );
    }
    let binding = designs
        .iter()
        .max_by(|a, b| (a.0 / a.2 as f64).total_cmp(&(b.0 / b.2 as f64)))
        .expect("a family");
    let need = binding.0 * 1e6 / binding.2 as f64;
    println!(
        "frame-equivalent gate 2's medians need: {:.1} ns (largest median design work per consumed frame-equivalent {need:.3} ns, {}, rounded up to a half nanosecond; committed {FRAME_EQUIVALENT_NS} ns)",
        (need * 2.0).ceil() / 2.0,
        binding.3
    );
    let heaviest = designs
        .iter()
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .expect("a family");
    println!(
        "worst median design work: {:.3} ms, {:.1} % of the budget's {budget_ms:.2} ms, {}",
        heaviest.0,
        heaviest.0 / budget_ms * 1e2,
        heaviest.3
    );
    let raw = designs
        .iter()
        .max_by(|a, b| (a.1 / a.2 as f64).total_cmp(&(b.1 / b.2 as f64)))
        .expect("a family");
    println!(
        "largest raw sample per consumed frame-equivalent (evidence): {:.3} ns, design work {:.3} ms, {:.3} x its median {:.3} ms, {}",
        raw.1 * 1e6 / raw.2 as f64,
        raw.1,
        raw.1 / raw.0,
        raw.0,
        raw.3
    );
    let raw_work = designs
        .iter()
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .expect("a family");
    println!(
        "largest raw design work (evidence): {:.3} ms, {:.1} % of the budget, {:.3} x its median, {}",
        raw_work.1,
        raw_work.1 / budget_ms * 1e2,
        raw_work.1 / raw_work.0,
        raw_work.3
    );
    worst.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (value, what) in worst.iter().take(5) {
        println!("slowest preparation: {value:.3} ms, {what}");
    }
}

/// Batches whose designs walk more frames than this are left out of the calibration's fit: the
/// intercept (the fixed cost) is read where the walk is short, so the per-frame slope barely moves
/// it.
const CALIBRATION_FRAMES: u64 = 16_384;

/// The designs of one calibration batch (root's ruling (c) of 2026-10-08 on #1465): `BATCH`
/// different designs of one class, walked back to back by one preparation, so that one sample of
/// the shortest designs (about 25-30 us a walk) lasts at least about 1 ms. A class whose design
/// count is not a multiple of `BATCH` folds the remainder into its last batch (`BATCH` to
/// `2 BATCH - 1` designs).
const BATCH: usize = 48;

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

/// The least-squares line `y = intercept + slope * x`.
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

/// One preparation of `designs` with a fresh cache and no budget limit: its keying, every walk and
/// every cache insertion, in strip order, as a real preparation spends them. Nanoseconds, frames
/// walked and sections charged (the charge net of the frames, in units of
/// `INPUT_BOUND_SECTION_CHARGE`). It asserts that every design is computed: the designs are
/// different, so none is served from the cache.
fn preparation(rate: u32, designs: &[BuiltinParameters]) -> (f64, u64, u64) {
    let mut cache = InputBoundCache::new();
    let frames = fixed_input_frames_walked();
    let charged = fixed_input_charged();
    let computed = fixed_input_bounds_computed();
    let (ms, _) = milliseconds(|| {
        input_section_bounds_within(rate, designs.iter().copied(), u64::MAX, Some(&mut cache))
            .expect("bounds")
    });
    assert_eq!(
        fixed_input_bounds_computed() - computed,
        designs.len() as u64,
        "every design of a batch is different and computed once"
    );
    let frames = fixed_input_frames_walked() - frames;
    let fixed = fixed_input_charged() - charged - frames;
    assert_eq!(
        fixed % builtins::INPUT_BOUND_SECTION_CHARGE,
        0,
        "section charges"
    );
    (
        ms * 1e6,
        frames,
        fixed / builtins::INPUT_BOUND_SECTION_CHARGE,
    )
}

/// One calibration batch: `BATCH` (or up to `2 BATCH - 1`) different designs of one class, each
/// sample a back-to-back preparation of all of them, and for evidence each design's single walk
/// (a one-strip preparation of it alone) measured in the same round, right after the batch.
struct Batch {
    designs: Vec<BuiltinParameters>,
    /// Batch samples, whole batch (ns), measured rounds only.
    ns: Vec<f64>,
    /// Per design, its single-walk samples (ns), measured rounds only.
    singles: Vec<Vec<f64>>,
    /// The frames walked and sections charged by the whole batch (deterministic).
    frames: u64,
    sections: u64,
    /// The most frames one design of the batch walks.
    longest: u64,
    class: String,
    what: String,
}

impl Batch {
    fn new(designs: Vec<BuiltinParameters>, class: String, what: String) -> Self {
        let count = designs.len();
        Batch {
            designs,
            ns: Vec::with_capacity(REPEATS),
            singles: vec![Vec::with_capacity(REPEATS); count],
            frames: 0,
            sections: 0,
            longest: 0,
            class,
            what,
        }
    }

    /// One round: the batch, then each design alone (a batch of one design is its own single
    /// walk). Returns the batch's ns.
    fn measure(&mut self, rate: u32, measured: bool) -> f64 {
        let (ns, frames, sections) = preparation(rate, &self.designs);
        if self.frames == 0 {
            (self.frames, self.sections) = (frames, sections);
        }
        assert_eq!(
            (self.frames, self.sections),
            (frames, sections),
            "deterministic"
        );
        if self.designs.len() == 1 {
            self.longest = frames;
            if measured {
                self.ns.push(ns);
                self.singles[0].push(ns);
            }
            return ns;
        }
        let mut walked = 0;
        for (index, design) in self.designs.iter().enumerate() {
            let (single, frames, _) = preparation(rate, std::slice::from_ref(design));
            self.longest = self.longest.max(frames);
            walked += frames;
            if measured {
                self.singles[index].push(single);
            }
        }
        assert_eq!(
            walked, self.frames,
            "the batch walks what its designs walk alone"
        );
        if measured {
            self.ns.push(ns);
        }
        ns
    }

    fn count(&self) -> f64 {
        self.designs.len() as f64
    }

    fn design(self) -> Design {
        let single_ns = self.singles.iter().map(|runs| median(runs)).sum();
        Design {
            count: self.designs.len() as u64,
            ns: self.ns,
            frames: self.frames,
            sections: self.sections,
            single_ns,
            class: self.class,
            what: self.what,
        }
    }
}

/// A class's designs at a rate in batches: ordered by the frames each walks (so that a batch's
/// designs are alike and the fit reads each batch at one length), then cut into `BATCH`
/// consecutive designs, the remainder folded into the last batch.
fn batches(rate: u32, name: &str, designs: &[BuiltinParameters]) -> Vec<Batch> {
    let mut walked: Vec<(u64, BuiltinParameters)> = designs
        .iter()
        .map(|design| (preparation(rate, std::slice::from_ref(design)).1, *design))
        .collect();
    walked.sort_by_key(|(frames, _)| *frames);
    assert!(walked.len() >= BATCH, "{name}: fewer than {BATCH} designs");
    let count = walked.len() / BATCH;
    (0..count)
        .map(|index| {
            let end = if index + 1 == count {
                walked.len()
            } else {
                (index + 1) * BATCH
            };
            Batch::new(
                walked[index * BATCH..end]
                    .iter()
                    .map(|(_, design)| *design)
                    .collect(),
                name.to_owned(),
                format!("{rate} Hz, {name}, batch {index}"),
            )
        })
        .collect()
}

/// One calibration batch's record: its measured samples of work (ns, the whole batch, one per
/// measured repeat), its design count, the frames it walks and the sections it charges (the whole
/// batch), the sum of its designs' single-walk medians (evidence) and what it is. Gate 2's
/// statistic reads it per design: the batch's median and charge divided by its design count.
struct Design {
    ns: Vec<f64>,
    count: u64,
    frames: u64,
    sections: u64,
    single_ns: f64,
    class: String,
    what: String,
}

impl Design {
    fn median(&self) -> f64 {
        median(&self.ns)
    }

    fn largest(&self) -> f64 {
        self.ns.iter().fold(0.0_f64, |a, b| a.max(*b))
    }

    fn smallest(&self) -> f64 {
        self.ns.iter().fold(f64::MAX, |a, b| a.min(*b))
    }

    /// Its charged frame-equivalents at a section charge (the whole batch).
    fn charged(&self, charge: u64) -> u64 {
        self.frames + charge * self.sections
    }

    /// Its median per walk over its designs' summed single-walk medians per walk (evidence).
    fn batch_over_single(&self) -> f64 {
        self.median() / self.single_ns
    }
}

/// The section charge a frame-equivalent of `frame_ns` gives: the largest per-batch-median fixed
/// cost per section (`per_section_ns`) in frame-equivalents, rounded up to a ten.
fn section_charge(per_section_ns: f64, frame_ns: f64) -> u64 {
    ((per_section_ns / frame_ns / 10.0).ceil() as u64) * 10
}

/// Root's statistic (the second ruling of 2026-10-08 on #1465, sampled as its ruling (c) of the
/// same day states): the smallest frame-equivalent on a half-nanosecond grid such that every
/// batch's per-design median work is at most its per-design charged frame-equivalents (frames
/// walked plus the section charge per section, the charge itself following from the
/// frame-equivalent by [`section_charge`]) times the frame-equivalent. Divided by the batch's
/// design count on both sides, it is the whole batch's median against the whole batch's charge.
/// Returns the frame-equivalent, its section charge and the binding batch's index (the largest
/// median work per charged frame-equivalent).
fn frame_equivalent(designs: &[Design], per_section_ns: f64) -> (f64, u64, usize) {
    let required = |charge: u64| {
        designs
            .iter()
            .enumerate()
            .map(|(index, design)| (design.median() / design.charged(charge) as f64, index))
            .fold((0.0_f64, 0), |a, b| if b.0 > a.0 { b } else { a })
    };
    let mut half_ns = 1_u32;
    loop {
        let frame_ns = f64::from(half_ns) * 0.5;
        let charge = section_charge(per_section_ns, frame_ns);
        let (need, binding) = required(charge);
        if need <= frame_ns {
            return (frame_ns, charge, binding);
        }
        half_ns += 1;
    }
}

/// A frame-class point: its name and the designs of its batch.
type Point = (String, Vec<BuiltinParameters>);

/// The frame-class points at a rate, grouped by class and trim (#1474 root ruling: every class).
/// Every point is one cascade of two sections, the same on both channels. A short point (typical,
/// about 3,000-25,000 frames a walk, and cheap, 513-1,025) is measured as a batch of `BATCH` different designs
/// of its class next to it, its HPF (typical) or LPF (cheap) raised by 0.05 Hz per design, as gate
/// 2's families step them. A near-top walk is long (about 400,000-530,000 frames, over 5 ms), so
/// its point is a batch of one design, its own single walk.
fn frame_class_points(rate: u32) -> Vec<(String, Vec<Point>)> {
    let top = maximum(rate);
    let steps = |count: usize, design: &dyn Fn(u32) -> BuiltinParameters| {
        (0..count as u32).map(design).collect::<Vec<_>>()
    };
    let mut groups = Vec::new();
    for trim in [0.0_f32, 24.0] {
        let points = (50..100)
            .map(|percent| (percent as f32 / 100.0, percent as f32 / 100.0 * top))
            .chain([(1.0, below(top, 1))])
            .map(|(fraction, hpf)| {
                (
                    format!("HPF {fraction:.2}"),
                    steps(1, &|_| same(trim, hpf, top)),
                )
            })
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
                points.push((
                    format!("{hpf} Hz into {lpf} Hz"),
                    steps(BATCH, &|step| same(trim, hpf + step as f32 * 0.05, lpf)),
                ));
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
        .map(|(hpf, lpf)| {
            (
                format!("{hpf} Hz into {lpf} Hz"),
                steps(BATCH, &|step| same(trim, hpf, lpf + step as f32 * 0.05)),
            )
        })
        .collect();
        groups.push((
            format!("cheap two-section (gate 2's families), {trim} dB"),
            points,
        ));
    }
    groups
}

/// The frame classes: the ns per walked frame of each point's batch, per design and net of its
/// fixed cost (`fixed_ns`, the rate's per-batch-median intercept of one cascade of two sections),
/// so that short walks are compared frame for frame with long ones. Each sweep measures every
/// batch once, so a burst of interference reaches one sample of many batches, not every sample of
/// one; sweep 0 is the warmup. Every batch is also recorded whole in `recorded` (the
/// frame-equivalent's statistic, [`frame_equivalent`]). Returns the slowest point's median net of
/// the fixed cost (descriptive).
fn frame_classes(fixed_ns: &[f64; 4], recorded: &mut Vec<Design>) -> f64 {
    println!(
        "rate | class | slowest point | its ns per frame net of the fixed cost, median (min-max) | fastest median | frames per design"
    );
    let mut slowest = (0.0_f64, String::new());
    for (slot, rate) in RATES.into_iter().enumerate() {
        for (class, points) in frame_class_points(rate) {
            let mut batches: Vec<Batch> = points
                .iter()
                .map(|(name, designs)| {
                    Batch::new(
                        designs.clone(),
                        class.clone(),
                        format!("{rate} Hz, {class}, {name}"),
                    )
                })
                .collect();
            for sweep in 0..ROUNDS {
                for batch in &mut batches {
                    batch.measure(rate, sweep > 0);
                }
            }
            let net = |batch: &Batch, ns: f64| {
                (ns / batch.count() - fixed_ns[slot]) / (batch.frames as f64 / batch.count())
            };
            let medians: Vec<(usize, f64, f64, f64)> = batches
                .iter()
                .enumerate()
                .map(|(point, batch)| {
                    let low = batch.ns.iter().fold(f64::MAX, |a, b| a.min(*b));
                    let high = batch.ns.iter().fold(0.0_f64, |a, b| a.max(*b));
                    (
                        point,
                        net(batch, median(&batch.ns)),
                        net(batch, low),
                        net(batch, high),
                    )
                })
                .collect();
            let high = medians
                .iter()
                .copied()
                .fold((0, 0.0_f64, 0.0, 0.0), |a, b| if b.1 > a.1 { b } else { a });
            let low = medians.iter().fold(f64::MAX, |low, p| low.min(p.1));
            let (low_frames, high_frames) = batches.iter().fold((u64::MAX, 0), |(l, h), b| {
                let frames = b.frames / b.designs.len() as u64;
                (l.min(frames), h.max(frames))
            });
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
            recorded.extend(batches.into_iter().map(Batch::design));
        }
    }
    println!(
        "slowest frame class measured: {:.2} ns per frame net of the fixed cost (median of {REPEATS}), {}",
        slowest.0, slowest.1
    );
    slowest.0
}

/// Per design class, the least-squares line of each batch's median time per design (of the
/// measured rounds) against its frames walked per design; each measured round's own line is
/// printed too (descriptive). Round 0 is the warmup; each round measures every batch once, so a
/// burst of interference reaches one sample of many batches. Returns each rate's median-fit
/// intercept of one cascade of two sections (ns) and the largest median-fit fixed cost per section
/// (ns). Every batch is also recorded in `recorded` (the frame-equivalent's statistic).
fn fixed_costs(recorded: &mut Vec<Design>) -> ([f64; 4], f64) {
    println!(
        "rate | round | class | batches | frames per design min-max | intercept us | ns per frame (slope) | intercept us per section"
    );
    let mut two_sections = [0.0_f64; 4];
    let mut per_section = 0.0_f64;
    for (slot, rate) in RATES.into_iter().enumerate() {
        let classes = calibration_classes(rate);
        let mut measured: Vec<Vec<Batch>> = classes
            .iter()
            .map(|(name, _, designs)| batches(rate, name, designs))
            .collect();
        // Points: the batch's longest design walk, its frames per design, its ns per design.
        let fit = |name: &str, walked: u64, label: &str, points: &[(u64, f64, f64)]| -> f64 {
            let short: Vec<(f64, f64)> = points
                .iter()
                .filter(|point| point.0 <= CALIBRATION_FRAMES)
                .map(|point| (point.1, point.2))
                .collect();
            let (intercept, slope) = line(&short);
            let (low, high) = short.iter().fold((f64::MAX, 0.0_f64), |(low, high), p| {
                (low.min(p.0), high.max(p.0))
            });
            println!(
                "{rate} | {label} | {name} | {} of {} | {low:.0}-{high:.0} | {:.2} | {:.2} | {:.2}",
                short.len(),
                points.len(),
                intercept * 1e-3,
                slope,
                intercept * 1e-3 / walked as f64
            );
            intercept
        };
        let summary = |label: &str, intercepts: &[f64; 4]| {
            // One cascade of one section is `D + S`, of two sections `D + 2S`; the two-cascade
            // classes check the model (`D + 2S`, `D + 4S`).
            let section = intercepts[1] - intercepts[0];
            let design = intercepts[0] - section;
            let predicted: Vec<String> = (2..4)
                .map(|class| {
                    format!(
                        "{:.2} us predicted against {:.2} us",
                        (design + section * classes[class].1 as f64) * 1e-3,
                        intercepts[class] * 1e-3
                    )
                })
                .collect();
            let largest = (0..4)
                .map(|class| intercepts[class] / classes[class].1 as f64)
                .fold(0.0_f64, f64::max);
            println!(
                "{rate} | {label} | per design {:.2} us; per section {:.2} us; two cascades: {}; largest fixed cost per section {:.2} us",
                design * 1e-3,
                section * 1e-3,
                predicted.join(", "),
                largest * 1e-3
            );
            largest
        };
        for round in 0..ROUNDS {
            let label = if round == 0 {
                "warmup".to_owned()
            } else {
                round.to_string()
            };
            let mut intercepts = [0.0_f64; 4];
            for (class, (name, walked, _)) in classes.iter().enumerate() {
                let points: Vec<(u64, f64, f64)> = measured[class]
                    .iter_mut()
                    .map(|batch| {
                        let ns = batch.measure(rate, round > 0);
                        (
                            batch.longest,
                            batch.frames as f64 / batch.count(),
                            ns / batch.count(),
                        )
                    })
                    .collect();
                intercepts[class] = fit(name, *walked, &label, &points);
            }
            summary(&label, &intercepts);
        }
        let mut intercepts = [0.0_f64; 4];
        for (class, (name, walked, _)) in classes.iter().enumerate() {
            let points: Vec<(u64, f64, f64)> = measured[class]
                .iter()
                .map(|batch| {
                    (
                        batch.longest,
                        batch.frames as f64 / batch.count(),
                        median(&batch.ns) / batch.count(),
                    )
                })
                .collect();
            intercepts[class] = fit(name, *walked, "median", &points);
        }
        per_section = per_section.max(summary("median", &intercepts));
        two_sections[slot] = intercepts[1];
        for batches in measured {
            recorded.extend(batches.into_iter().map(Batch::design));
        }
    }
    (two_sections, per_section)
}

fn calibrate() {
    // The fixed costs first: the frame classes are read net of them.
    let mut recorded = Vec::new();
    let (two_sections, per_section) = fixed_costs(&mut recorded);
    let slowest = frame_classes(&two_sections, &mut recorded);
    println!(
        "descriptive, net of the fixed cost: slowest class's median {slowest:.2} ns per frame"
    );
    let (frame_ns, charge, binding) = frame_equivalent(&recorded, per_section);
    let design = &recorded[binding];
    let charged = design.charged(charge);
    let count = design.count as f64;
    println!(
        "largest per-batch-median fixed cost per section {:.2} us: {:.1} frame-equivalents of {frame_ns} ns, section charge {charge} (rounded up to a ten, {:.2} us)",
        per_section * 1e-3,
        per_section / frame_ns,
        charge as f64 * frame_ns * 1e-3
    );
    let designs: u64 = recorded.iter().map(|design| design.count).sum();
    println!(
        "frame-equivalent over {} batches of {designs} designs ({REPEATS} measured samples each, median): {frame_ns} ns (committed {FRAME_EQUIVALENT_NS} ns); binding batch {} ({} designs): median work per design {:.0} ns, {:.1} frames + {charge} x {:.2} sections = {:.1} charged frame-equivalents per design, {:.3} ns per charged frame-equivalent",
        recorded.len(),
        design.what,
        design.count,
        design.median() / count,
        design.frames as f64 / count,
        design.sections as f64 / count,
        charged as f64 / count,
        design.median() / charged as f64
    );
    let shortest = recorded
        .iter()
        .min_by(|a, b| a.smallest().total_cmp(&b.smallest()))
        .expect("a batch");
    println!(
        "shortest batch sample: {:.3} ms, {} ({} designs)",
        shortest.smallest() * 1e-6,
        shortest.what,
        shortest.count
    );
    let raw = recorded
        .iter()
        .max_by(|a, b| {
            (a.largest() / a.charged(charge) as f64)
                .total_cmp(&(b.largest() / b.charged(charge) as f64))
        })
        .expect("a batch");
    println!(
        "largest raw sample per charged frame-equivalent (evidence): {:.3} ns, {}: sample {:.0} ns per design, {:.3} x its batch's median {:.0} ns per design",
        raw.largest() / raw.charged(charge) as f64,
        raw.what,
        raw.largest() / raw.count as f64,
        raw.largest() / raw.median(),
        raw.median() / raw.count as f64
    );
    let ratio = recorded
        .iter()
        .max_by(|a, b| (a.largest() / a.median()).total_cmp(&(b.largest() / b.median())))
        .expect("a batch");
    println!(
        "largest raw sample over its batch's median (evidence): {:.3} x, {}: sample {:.0} ns per design, median {:.0} ns per design, {:.3} ns per charged frame-equivalent",
        ratio.largest() / ratio.median(),
        ratio.what,
        ratio.largest() / ratio.count as f64,
        ratio.median() / ratio.count as f64,
        ratio.largest() / ratio.charged(charge) as f64
    );
    // Evidence for ruling (c)'s condition: a batch walks different designs, so it must not be
    // cheaper per walk than the same designs walked alone.
    let mut ratios: Vec<f64> = recorded.iter().map(Design::batch_over_single).collect();
    ratios.sort_by(f64::total_cmp);
    let below = ratios.iter().filter(|ratio| **ratio < 1.0).count();
    let cheapest = recorded
        .iter()
        .min_by(|a, b| a.batch_over_single().total_cmp(&b.batch_over_single()))
        .expect("a batch");
    println!(
        "batch median per walk over the sum of its designs' single-walk medians (evidence): min {:.4}, median {:.4}, max {:.4}; {below} of {} batches below 1; the least {}: batch {:.0} ns per design, single walks {:.0} ns per design",
        ratios[0],
        median(&ratios),
        ratios[ratios.len() - 1],
        ratios.len(),
        cheapest.what,
        cheapest.median() / cheapest.count as f64,
        cheapest.single_ns / cheapest.count as f64
    );
    let singles_need = recorded
        .iter()
        .map(|design| design.single_ns / design.charged(charge) as f64)
        .fold(0.0_f64, f64::max);
    println!(
        "the summed single-walk medians per charged frame-equivalent at charge {charge} (evidence): largest {singles_need:.3} ns"
    );
    let mut per_class: Vec<(String, f64, f64)> = Vec::new();
    for design in &recorded {
        let value = design.batch_over_single();
        match per_class.iter_mut().find(|entry| entry.0 == design.class) {
            Some(entry) => (entry.1, entry.2) = (entry.1.min(value), entry.2.max(value)),
            None => per_class.push((design.class.clone(), value, value)),
        }
    }
    for (class, low, high) in per_class {
        println!("batch over single walks (every rate), {class}: {low:.4}-{high:.4}");
    }
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
