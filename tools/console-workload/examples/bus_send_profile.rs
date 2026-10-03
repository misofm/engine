//! Issue #1229 D2: the untimed phase profile of the native bus-and-send row,
//! `sixty_four_track_console_sends`, at [`PlanConfig::BASELINE`] (the static plan: no meters, no
//! live controls, `Concurrent` builtins).
//!
//! An instrument, not a test: it gates nothing and asserts nothing about time. It drives
//! `graph::test_only_phase_profile` (test-support only, absent from every production build; this
//! crate's dev-dependency on `graph` enables it for examples as well as tests) the way
//! `tests/gain_pan_profile.rs`'s `phase_profile` does: 512 warmup blocks, then three repeats of
//! 4000 blocks with the probes off and on, the in-situ probe cost derived from the two, and every
//! graph phase and bank sub-phase reported net of the probes as the per-line median of the
//! repeats. Unlike that harness it prints every line, zero rows included, assumes no strip shape
//! (the slot positions are printed as `bank::slot_names()` reports them), derives no cycles, and
//! prints the plan census #1229 D5 reads: the bank shape, the route folds and, per bank chain, its
//! lanes, the submix lanes among them and the pad lanes of the bank width.
//!
//! Build it with the native console runner's profile overrides and run it once, pinned, under the
//! operator's timing lock (#1229 D2):
//!
//! ```text
//! CARGO_PROFILE_RELEASE_OPT_LEVEL=3 CARGO_PROFILE_RELEASE_LTO=false \
//!     CARGO_PROFILE_RELEASE_CODEGEN_UNITS=16 CARGO_INCREMENTAL=0 \
//!     cargo build --locked --release -p console-workload --example bus_send_profile
//! flock -w 7200 "$LOCK" taskset -c <cpu> target/release/examples/bus_send_profile
//! ```

use std::collections::BTreeSet;
use std::time::Instant;

use console_workload::{PlanConfig, QUANTUM, SessionRuntime, Workload};
use graph::test_only_phase_profile as profile;
use profile::bank;

/// The row under profile.
const ROW: Workload = Workload::SixtyFourTrackConsoleSends;
const WARMUP_BLOCKS: u64 = 512;
const PROFILED_BLOCKS: u64 = 4000;
const REPEATS: usize = 3;

/// Graph phase labels, in `test_only_phase_profile` index order.
const PHASES: [&str; profile::COUNT] = [
    "enter",
    "source",
    "bound",
    "route",
    "output",
    "identity-copy",
    "identity-alias",
    "other",
    "bank",
    "exit",
];

/// Median cost of one `Instant::now()` pair, the unit of probe overhead.
fn clock_read_ns() -> f64 {
    let mut deltas: Vec<u64> = (0..20_000)
        .map(|_| {
            let a = Instant::now();
            let b = Instant::now();
            u64::try_from(b.duration_since(a).as_nanos()).unwrap_or(u64::MAX)
        })
        .collect();
    deltas.sort_unstable();
    deltas[deltas.len() / 2] as f64
}

/// Renders one block and drains any meter it published, outside any clock.
fn render(runtime: &mut SessionRuntime, observation: &mut u64) {
    runtime.render(*observation).expect("console render");
    runtime.drain_meters();
    *observation += 1;
}

/// Every chain sub-phase row of the report, zero rows included: `(label, sub-phase index)`.
fn sub_phase_rows() -> Vec<(String, usize)> {
    let mut rows = vec![
        ("prologue".to_owned(), bank::PROLOGUE),
        ("gather".to_owned(), bank::GATHER),
    ];
    rows.extend(
        (0..bank::SLOTS).map(|position| (format!("slot {position}"), bank::SLOT + position)),
    );
    rows.extend([
        ("seam".to_owned(), bank::SEAM),
        ("scatter".to_owned(), bank::SCATTER),
        ("fold".to_owned(), bank::FOLD),
        ("aux".to_owned(), bank::AUX),
    ]);
    rows
}

/// One repeat, per block.
struct Repeat {
    /// Graph phases, probed nanoseconds.
    phases: [f64; profile::COUNT],
    /// Graph phase entries (probes that began each phase), derived from `profile::runs()`.
    phase_entries: [f64; profile::COUNT],
    /// Chain sub-phases, probed nanoseconds.
    sub_phases: [f64; bank::COUNT],
    /// Chain sub-phase entries.
    sub_entries: [f64; bank::COUNT],
    /// Clock reads the chain probes took.
    chain_reads: f64,
    /// Clock reads every probe took, the graph's and the chain's.
    reads: f64,
    /// In-situ cost of one probe: the probed mean less the unprobed mean, over the clock reads.
    probe_ns: f64,
}

impl Repeat {
    fn net_phase(&self, index: usize) -> f64 {
        let nested = if index == profile::BANK {
            self.chain_reads
        } else {
            0.0
        };
        self.phases[index] - (self.phase_entries[index] + nested) * self.probe_ns
    }

    fn net_sub_phase(&self, index: usize) -> f64 {
        self.sub_phases[index] - self.sub_entries[index] * self.probe_ns
    }

    /// What the bank phase holds outside the chain runs: the unit's dispatch, member setup and
    /// observation (and the chain probes' closing reads).
    fn outside_chain(&self, net: bool) -> f64 {
        if net {
            self.net_phase(profile::BANK)
                - (0..bank::COUNT)
                    .map(|index| self.net_sub_phase(index))
                    .sum::<f64>()
        } else {
            self.phases[profile::BANK] - self.sub_phases.iter().sum::<f64>()
        }
    }
}

fn median(mut values: Vec<f64>) -> f64 {
    values.sort_by(f64::total_cmp);
    values[values.len() / 2]
}

/// Renders `PROFILED_BLOCKS` blocks and returns the mean nanoseconds per block.
fn timed_loop(runtime: &mut SessionRuntime, observation: &mut u64) -> f64 {
    let start = Instant::now();
    for _ in 0..PROFILED_BLOCKS {
        render(runtime, observation);
    }
    start.elapsed().as_nanos() as f64 / PROFILED_BLOCKS as f64
}

/// The plan census (#1229 D5), printed before anything is timed.
fn print_census(runtime: &SessionRuntime) {
    let [chains, slots] = runtime.bank_shape();
    let units = runtime.unit_eligibility();
    let banked = units.iter().filter(|unit| unit.banked).count();
    let width = lane::Backend::current().width();
    println!(
        "census: bank_shape [chains, slots] = [{chains}, {slots}], units = {} \
         ({banked} bank chains, {} single ops), bank_route_folds = {}, bank width = {width}",
        units.len(),
        units.len() - banked,
        runtime.bank_route_folds()
    );
    // The fixture's tracks are exactly the structural-symmetry rows (keyed by track id, tracks
    // only); a non-empty lane name outside them is a submix strip's lane (since #1200).
    let tracks: BTreeSet<&str> = runtime
        .structural_symmetry()
        .iter()
        .map(|(track, _)| &**track)
        .collect();
    println!("census: {} tracks", tracks.len());
    for unit in units.iter().filter(|unit| unit.banked) {
        let active = unit.lanes() as usize;
        let submix_lanes: Vec<&str> = unit
            .lane_tracks
            .iter()
            .map(|lane| &**lane)
            .filter(|lane| !lane.is_empty() && !tracks.contains(lane))
            .collect();
        let kind = match submix_lanes.len() {
            0 => "track",
            count if count == active => "submix",
            _ => "mixed",
        };
        println!(
            "census chain: unit {} {kind} stages {} active lanes {active} pad lanes {} lanes [{}]",
            unit.unit,
            unit.stages,
            active.div_ceil(width) * width - active,
            unit.lane_tracks
                .iter()
                .map(|lane| &**lane)
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}

fn main() {
    let clock_ns = clock_read_ns();
    let tracks = ROW.tracks();
    println!(
        "row {}, backend {:?}, quantum {QUANTUM}, {tracks} tracks, PlanConfig::BASELINE",
        ROW.kind(),
        lane::Backend::current()
    );
    println!("clock read {clock_ns:.1} ns (median Instant::now() pair)");

    let mut runtime = SessionRuntime::build(ROW, PlanConfig::BASELINE);
    print_census(&runtime);
    let [chains, _] = runtime.bank_shape();
    let mut observation = 0_u64;
    profile::enable(false);
    for _ in 0..WARMUP_BLOCKS {
        render(&mut runtime, &mut observation);
    }

    let per_block = |total: u64| total as f64 / PROFILED_BLOCKS as f64;
    let mut repeats = Vec::with_capacity(REPEATS);
    let mut off_p50 = Vec::with_capacity(REPEATS);
    let mut off_mean = Vec::with_capacity(REPEATS);
    let mut graph_probes_per_block = Vec::with_capacity(REPEATS);
    for repeat in 0..REPEATS {
        // Probes compiled in but off: the row's own cost, block by block, then as one loop.
        let mut samples = Vec::with_capacity(PROFILED_BLOCKS as usize);
        for _ in 0..PROFILED_BLOCKS {
            let start = Instant::now();
            let result = runtime.render(observation);
            let elapsed = u64::try_from(start.elapsed().as_nanos()).unwrap_or(u64::MAX);
            result.expect("console render");
            runtime.drain_meters();
            observation += 1;
            samples.push(elapsed);
        }
        samples.sort_unstable();
        let p50 = bench_support::stats::per_mille(&samples, 500);
        off_p50.push(p50 as f64);
        let mean_off = timed_loop(&mut runtime, &mut observation);
        off_mean.push(mean_off);
        println!(
            "repeat {repeat}: probes off  min {} ns  p50 {p50} ns  p95 {} ns  mean {mean_off:.0} ns",
            samples[0],
            bench_support::stats::per_mille(&samples, 950),
        );

        // Probes on: the same loop, charged to phases and sub-phases.
        profile::reset();
        profile::enable(true);
        let mean_on = timed_loop(&mut runtime, &mut observation);
        profile::enable(false);
        let (nanos, blocks, graph_reads) = profile::snapshot();
        let chain = bank::snapshot();
        // Printed, not asserted: a measurement has already been printed, so a failure here is a
        // finding to record, never a reason to run again.
        if blocks != PROFILED_BLOCKS || chain.chain_runs != PROFILED_BLOCKS * chains {
            println!(
                "repeat {repeat}: NOTE profiled blocks {blocks} (expected {PROFILED_BLOCKS}), \
                 chain runs {} (expected {})",
                chain.chain_runs,
                PROFILED_BLOCKS * chains
            );
        }
        // The graph's phases are entered once per block each (entry, source), and a unit kind
        // once per run of it in the schedule, which is fixed at bind.
        let runs = profile::runs();
        let used: Vec<(usize, u64)> = runs
            .iter()
            .copied()
            .filter(|&(kind, units)| kind < profile::COUNT && units > 0)
            .collect();
        let mut phase_entries = [0.0; profile::COUNT];
        phase_entries[profile::ENTER] = 1.0;
        phase_entries[profile::SOURCE] = 1.0;
        for &(kind, _) in &used {
            phase_entries[kind] += 1.0;
        }
        let graph_probes = per_block(graph_reads);
        graph_probes_per_block.push(graph_probes);
        if repeat == 0 {
            let listed: Vec<String> = used
                .iter()
                .map(|&(kind, units)| format!("{units} x {}", PHASES[kind]))
                .collect();
            println!(
                "unit runs in schedule order ({} of {} entries used): {}",
                used.len(),
                profile::RUNS_CAPACITY,
                listed.join(", ")
            );
            // A block's graph probes are its start, the source entry, one per unit-kind run and
            // the finish, so a runs list that holds every run accounts for all of them.
            println!(
                "graph probes per block {graph_probes:.1}; start + source + runs + finish = {}{}",
                used.len() + 3,
                if used.len() == profile::RUNS_CAPACITY {
                    "; NOTE every runs() entry is used, so the entry counts and the net figures \
                     may be truncated: read the probed figures"
                } else {
                    ""
                }
            );
            for (position, name) in bank::slot_names().iter().enumerate() {
                println!(
                    "slot {position}: {}",
                    if name.is_empty() {
                        "(not reached)"
                    } else {
                        name
                    }
                );
            }
        }
        let reads = per_block(graph_reads + chain.probes);
        let probe_ns = (mean_on - mean_off) / reads;
        println!(
            "repeat {repeat}: probes on   mean {mean_on:.0} ns/block, \
             {reads:.1} clock reads/block ({graph_probes:.1} graph, {:.1} chain), \
             {probe_ns:.1} ns per probe in situ (clock read alone {clock_ns:.1} ns)",
            per_block(chain.probes)
        );
        repeats.push(Repeat {
            phases: nanos.map(per_block),
            phase_entries,
            sub_phases: chain.nanos.map(per_block),
            sub_entries: chain.entries.map(per_block),
            chain_reads: per_block(chain.probes),
            reads,
            probe_ns,
        });
    }

    // The report: per line, the median over the repeats. "Probed" is what the probes charged;
    // "net" subtracts one in-situ probe cost per entry (and, for the bank phase, every chain probe
    // nested in it), an estimate of the unprobed cost, not a measurement of it. Every share has the
    // one denominator `net_total`, the sum of every graph phase's net time: the profiled block time.
    let over = |value: &dyn Fn(&Repeat) -> f64| median(repeats.iter().map(value).collect());
    let net_total = over(&|r| (0..profile::COUNT).map(|i| r.net_phase(i)).sum());
    let row = |label: &str, entries: f64, probed: f64, net: f64| {
        println!(
            "| {label:<30} | {entries:>8.1} | {probed:>10.1} | {net:>10.1} | {:>6.2}% |",
            100.0 * net / net_total
        );
    };
    println!();
    println!(
        "median of {REPEATS} repeats of {PROFILED_BLOCKS} blocks; probes off: p50 {:.0} ns/block, \
         loop mean {:.0} ns/block; probe cost {:.1} ns in situ; {:.1} clock reads/block \
         ({:.1} graph probes/block); profiled block time (net_total) {net_total:.1} ns",
        median(off_p50),
        median(off_mean),
        over(&|r| r.probe_ns),
        over(&|r| r.reads),
        median(graph_probes_per_block)
    );
    println!(
        "| {:<30} | {:>8} | {:>10} | {:>10} | {:>7} |",
        "phase", "entries", "probed ns", "net ns", "share"
    );
    println!(
        "|{}|{}|{}|{}|{}|",
        "-".repeat(32),
        "-".repeat(10),
        "-".repeat(12),
        "-".repeat(12),
        "-".repeat(9)
    );
    for (index, name) in PHASES.iter().enumerate() {
        let entries = over(&|r| r.phase_entries[index]);
        let probed = over(&|r| r.phases[index]);
        let net = over(&|r| r.net_phase(index));
        if index != profile::BANK {
            row(name, entries, probed, net);
            continue;
        }
        row("bank (whole unit)", entries, probed, net);
        for (label, sub) in sub_phase_rows() {
            row(
                &format!("  {label}"),
                over(&|r| r.sub_entries[sub]),
                over(&|r| r.sub_phases[sub]),
                over(&|r| r.net_sub_phase(sub)),
            );
        }
        row(
            "  outside chain",
            entries,
            over(&|r| r.outside_chain(false)),
            over(&|r| r.outside_chain(true)),
        );
    }
    row(
        "total (entries: clock reads)",
        over(&|r| r.reads),
        over(&|r| r.phases.iter().sum()),
        net_total,
    );
}
