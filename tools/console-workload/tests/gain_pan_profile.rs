//! Issue #960: the phase profile of the native pure-audio-path target,
//! `sixty_four_track_gain_pan_ring`.
//!
//! It replaces the deleted `plumbing_profile.rs`, whose builtins-less rows no host compiles, with a
//! profile of the row a host actually feeds: sixty-four gain/pan strips compiled with builtins and
//! fed through a prepared source set whose claims every bank gather reads in place.
//!
//! Two instruments, both test-support only and both absent from every production build:
//!
//! * `graph::test_only_phase_profile` charges each block to the executor's phases (entry, source,
//!   the unit kinds, exit), taking a probe only where the unit kind changes;
//! * its `bank` module splits every bank unit's chain run at the chain's own boundaries: the
//!   prologue (queue drains and the collapse decision), the gather, each slot by its chain
//!   position, the collapse seam, the scatter, and the route/master fold the members perform.
//!   What the `BANK` phase holds beyond those is the unit's dispatch, member setup and
//!   observation, reported as "outside chain".
//!
//! `phase_profile` is ignored by default. It prints measurements and asserts nothing about time:
//! the numbers are descriptive, taken on whatever host runs it, and are not the console benchmark.
//! Run it pinned, on an otherwise quiet core:
//!
//! ```text
//! CARGO_INCREMENTAL=0 cargo test --release -p console-workload --test gain_pan_profile --no-run
//! taskset -c 31 target/release/deps/gain_pan_profile-<hash> --ignored --nocapture \
//!     --test-threads 1 phase_profile
//! ```
//!
//! A builtin slot shows the graph's `BuiltinStage` wrapper as its type. On this row every chain
//! is one cohort's builtin strip in its fixed order (AGENTS.md: input section, fader, pan
//! matrix), so slot 0 is the input section, slot 1 the fader and slot 2 the pan matrix.
//!
//! `digests` (also ignored) prints the 64-block digest of every native session row with the probes
//! off and on, so a change to the probes can be checked against a base commit's bits in one
//! command. `probes_split_the_bank_units_and_move_no_bit` is the standing, untimed gate: the
//! probes fire where they claim to and render the row's bits.
//!
//! Cycles are derived, not counted: the core clock is calibrated from a dependent chain of scalar
//! `f32` adds at [`FADD_LATENCY_CYCLES`] cycles each (Zen 3), timed by the same monotonic clock the
//! probes read.

use std::hint::black_box;
use std::time::Instant;

use bench_support::digest::Sha256Sink;
use console_workload::{PlanConfig, QUANTUM, SessionRuntime, Workload, native_session_rows};
use graph::test_only_phase_profile as profile;
use profile::bank;

/// The row under profile.
const ROW: Workload = Workload::SixtyFourTrackGainPanRing;
const WARMUP_BLOCKS: u64 = 512;
const PROFILED_BLOCKS: u64 = 4000;
const REPEATS: usize = 3;
/// Dependent `addss` latency on the Zen 3 core this harness was first run on.
const FADD_LATENCY_CYCLES: f64 = 3.0;

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

/// Core clock from a dependent `f32` add chain: [`FADD_LATENCY_CYCLES`] cycles per add.
fn core_hz() -> f64 {
    const ADDS: u64 = 300_000_000;
    let mut best = f64::MAX;
    for _ in 0..3 {
        let mut x = black_box(0.0_f32);
        let d = black_box(1.0e-3_f32);
        let start = Instant::now();
        for _ in 0..ADDS {
            x += d;
        }
        let elapsed = start.elapsed().as_secs_f64();
        black_box(x);
        best = best.min(elapsed);
    }
    FADD_LATENCY_CYCLES * ADDS as f64 / best
}

/// Renders one block and drains any meter it published, outside any clock.
fn render(runtime: &mut SessionRuntime, observation: &mut u64) {
    runtime.render(*observation).expect("console render");
    runtime.drain_meters();
    *observation += 1;
}

/// The chain sub-phase rows of the report: `(label, sub-phase index)`, slots named by the stage
/// type the first profiled chain ran there.
fn sub_phase_rows(names: &[&'static str; bank::SLOTS]) -> Vec<(String, usize)> {
    let mut rows = vec![
        ("prologue".to_owned(), bank::PROLOGUE),
        ("gather".to_owned(), bank::GATHER),
    ];
    for (position, name) in names.iter().enumerate() {
        if !name.is_empty() {
            rows.push((
                format!("slot {position} ({})", short_type_name(name)),
                bank::SLOT + position,
            ));
        }
    }
    rows.extend([
        ("seam".to_owned(), bank::SEAM),
        ("scatter".to_owned(), bank::SCATTER),
        ("fold".to_owned(), bank::FOLD),
        ("aux".to_owned(), bank::AUX),
    ]);
    rows
}

/// `a::b::Stage<c::D>` as `Stage<D>`: the last path segment of every type in the name.
fn short_type_name(name: &str) -> String {
    let mut short = String::new();
    let mut segment = String::new();
    for character in name.chars() {
        if character.is_alphanumeric() || character == '_' || character == ':' {
            segment.push(character);
        } else {
            short.push_str(segment.rsplit("::").next().unwrap_or(""));
            segment.clear();
            short.push(character);
        }
    }
    short.push_str(segment.rsplit("::").next().unwrap_or(""));
    short
}

/// One repeat, per block.
struct Repeat {
    /// Graph phases, probed nanoseconds.
    phases: [f64; profile::COUNT],
    /// Graph phase entries (probes that began each phase).
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

#[test]
#[ignore = "measurement harness; run pinned with --ignored --nocapture"]
fn phase_profile() {
    let hz = core_hz();
    let clock_ns = clock_read_ns();
    let tracks = u64::from(ROW.tracks());
    let lane_samples = (tracks * QUANTUM as u64 * 2) as f64;
    let cycles = |ns: f64| ns * hz / 1e9;
    println!(
        "row {}, backend {:?}, quantum {QUANTUM}, {tracks} tracks",
        ROW.kind(),
        lane::Backend::current()
    );
    println!(
        "core clock {:.3} GHz (derived: dependent f32 add chain at {FADD_LATENCY_CYCLES} cycles), \
         clock read {clock_ns:.1} ns",
        hz / 1e9
    );

    let mut runtime = SessionRuntime::build(ROW, PlanConfig::BASELINE);
    let [chains, slots] = runtime.bank_shape();
    assert_eq!(
        slots,
        3 * chains,
        "the slot labels assume three builtin stages per chain: input section, fader, pan matrix"
    );
    println!(
        "bank_shape [chains, slots] = [{chains}, {slots}], units = {}, route folds = {}",
        runtime.unit_eligibility().len(),
        runtime.bank_route_folds()
    );
    let mut observation = 0_u64;
    profile::enable(false);
    for _ in 0..WARMUP_BLOCKS {
        render(&mut runtime, &mut observation);
    }

    let per_block = |total: u64| total as f64 / PROFILED_BLOCKS as f64;
    let mut repeats = Vec::with_capacity(REPEATS);
    let mut off_p50 = Vec::with_capacity(REPEATS);
    let mut off_mean = Vec::with_capacity(REPEATS);
    let mut names = [""; bank::SLOTS];
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
            "repeat {repeat}: probes off  min {} ns  p50 {p50} ns  p95 {} ns  \
             mean {mean_off:.0} ns  \
             (p50 = {:.0} cycles/block, {:.3} cycles/lane-sample)",
            samples[0],
            bench_support::stats::per_mille(&samples, 950),
            cycles(p50 as f64),
            cycles(p50 as f64) / lane_samples
        );

        // Probes on: the same loop, charged to phases and sub-phases.
        profile::reset();
        profile::enable(true);
        let mean_on = timed_loop(&mut runtime, &mut observation);
        profile::enable(false);
        let (nanos, blocks, graph_reads) = profile::snapshot();
        let chain = bank::snapshot();
        assert_eq!(
            blocks, PROFILED_BLOCKS,
            "every profiled block closed its probe"
        );
        assert_eq!(
            chain.chain_runs,
            PROFILED_BLOCKS * chains,
            "every chain run closed its probe"
        );
        // The graph's phases are entered once per block each (entry, source), and a unit kind
        // once per run of it in the schedule, which is fixed at bind.
        let mut phase_entries = [0.0; profile::COUNT];
        phase_entries[profile::ENTER] = 1.0;
        phase_entries[profile::SOURCE] = 1.0;
        for (kind, units) in profile::runs() {
            if kind < profile::COUNT && units > 0 {
                phase_entries[kind] += 1.0;
            }
        }
        if repeat == 0 {
            let runs: Vec<String> = profile::runs()
                .iter()
                .filter(|(kind, units)| *kind < profile::COUNT && *units > 0)
                .map(|(kind, units)| format!("{units} x {}", PHASES[*kind]))
                .collect();
            println!("unit runs in schedule order: {}", runs.join(", "));
            names = bank::slot_names();
            for (position, name) in names.iter().enumerate() {
                if !name.is_empty() {
                    println!("slot {position}: {name}");
                }
            }
        }
        let reads = per_block(graph_reads + chain.probes);
        let probe_ns = (mean_on - mean_off) / reads;
        println!(
            "repeat {repeat}: probes on   mean {mean_on:.0} ns/block, \
             {reads:.1} clock reads/block, \
             {probe_ns:.1} ns per probe in situ (clock read alone {clock_ns:.1} ns)"
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

    // The report: per row, the median over the repeats. "Probed" is what the probes charged; "net"
    // subtracts one in-situ probe cost per entry (and, for the bank phase, every chain probe nested
    // in it), which is an estimate of the unprobed cost, not a measurement of it.
    let over = |value: &dyn Fn(&Repeat) -> f64| median(repeats.iter().map(value).collect());
    let net_total = over(&|r| (0..profile::COUNT).map(|i| r.net_phase(i)).sum());
    let row = |label: &str, entries: f64, probed: f64, net: f64| {
        println!(
            "| {label:<30} | {entries:>7.1} | {probed:>9.1} | {net:>9.1} \
             | {:>9.0} | {:>7.3} | {:>5.1}% |",
            cycles(net),
            cycles(net) / lane_samples,
            100.0 * net / net_total
        );
    };
    println!();
    println!(
        "median of {REPEATS} repeats, per block of {lane_samples} lane-samples; \
         probes off: p50 {:.0} ns \
         = {:.0} cycles, loop mean {:.0} ns; probe cost {:.1} ns in situ",
        median(off_p50.clone()),
        cycles(median(off_p50)),
        median(off_mean),
        over(&|r| r.probe_ns)
    );
    println!(
        "| {:<30} | {:>7} | {:>9} | {:>9} | {:>9} | {:>7} | {:>6} |",
        "phase", "entries", "probed ns", "net ns", "net cyc", "net c/ls", "share"
    );
    println!(
        "|{}|{}|{}|{}|{}|{}|{}|",
        "-".repeat(32),
        "-".repeat(9),
        "-".repeat(11),
        "-".repeat(11),
        "-".repeat(11),
        "-".repeat(9),
        "-".repeat(8)
    );
    for (index, name) in PHASES.iter().enumerate() {
        let probed = over(&|r| r.phases[index]);
        if probed == 0.0 {
            continue;
        }
        let entries = over(&|r| r.phase_entries[index]);
        let net = over(&|r| r.net_phase(index));
        if index != profile::BANK {
            row(name, entries, probed, net);
            continue;
        }
        row("bank (whole unit)", entries, probed, net);
        for (label, sub) in sub_phase_rows(&names) {
            let probed = over(&|r| r.sub_phases[sub]);
            if probed > 0.0 {
                row(
                    &format!("  {label}"),
                    over(&|r| r.sub_entries[sub]),
                    probed,
                    over(&|r| r.net_sub_phase(sub)),
                );
            }
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

/// Renders `blocks` blocks of `workload` and returns the digest of everything it emitted.
fn digest(workload: Workload, blocks: u64) -> String {
    let mut runtime = SessionRuntime::build(workload, PlanConfig::BASELINE);
    let mut hash = Sha256Sink::new();
    let mut observation = 0;
    for _ in 0..blocks {
        render(&mut runtime, &mut observation);
        runtime.hash_output(&mut hash);
    }
    hash.finish_hex()
}

#[test]
#[ignore = "measurement harness; run with --ignored --nocapture"]
fn digests() {
    for workload in native_session_rows() {
        profile::enable(false);
        let off = digest(workload, 64);
        profile::reset();
        profile::enable(true);
        let on = digest(workload, 64);
        profile::enable(false);
        println!("{:<44} {off}", workload.kind());
        assert_eq!(on, off, "{}: the probes moved a bit", workload.kind());
    }
}

/// The probes fire where they claim to, and a profiled render is the row's own bits.
///
/// Untimed. Every chain run opens and closes one sub-phase profile, gathers once and scatters
/// once; the row folds every route, so every chain run enters the fold; the chain sub-phases nest
/// inside the graph's `BANK` phase; and the digest with every probe on is the digest with every
/// probe off.
#[test]
fn probes_split_the_bank_units_and_move_no_bit() {
    const BLOCKS: u64 = 16;
    profile::enable(false);
    let off = digest(ROW, BLOCKS);

    let mut runtime = SessionRuntime::build(ROW, PlanConfig::BASELINE);
    let [chains, slots] = runtime.bank_shape();
    assert!(chains > 0, "the row binds bank chains");
    assert_eq!(
        slots,
        3 * chains,
        "the slot labels assume three builtin stages per chain: input section, fader, pan matrix"
    );
    assert_eq!(
        runtime.bank_route_folds(),
        u64::from(ROW.tracks()),
        "the row folds every route"
    );
    let mut hash = Sha256Sink::new();
    let mut observation = 0;
    profile::reset();
    profile::enable(true);
    for _ in 0..BLOCKS {
        render(&mut runtime, &mut observation);
        runtime.hash_output(&mut hash);
    }
    profile::enable(false);
    assert_eq!(hash.finish_hex(), off, "the probes moved a bit");

    let (nanos, blocks, _) = profile::snapshot();
    let chain = bank::snapshot();
    let runs = BLOCKS * chains;
    assert_eq!(blocks, BLOCKS);
    assert_eq!(chain.chain_runs, runs, "one closed profile per chain run");
    assert_eq!(
        chain.entries[bank::PROLOGUE],
        runs,
        "one prologue per chain run"
    );
    assert_eq!(
        chain.entries[bank::GATHER],
        runs,
        "one gather per chain run"
    );
    assert_eq!(
        chain.entries[bank::SCATTER],
        runs,
        "one scatter per chain run"
    );
    assert_eq!(
        chain.entries[bank::FOLD],
        runs,
        "one fold per folded chain run"
    );
    assert_eq!(chain.entries[bank::SEAM], 0, "the row does not collapse");
    let slot_entries: u64 = chain.entries[bank::SLOT..bank::SLOT + bank::SLOTS]
        .iter()
        .sum();
    assert!(
        slot_entries > 0 && slot_entries <= BLOCKS * slots,
        "each slot with an active lane is entered once per run ({slot_entries} of at most {})",
        BLOCKS * slots
    );
    for (position, name) in bank::slot_names().iter().enumerate() {
        assert_eq!(
            name.is_empty(),
            chain.entries[bank::SLOT + position] == 0,
            "slot {position} is named exactly when it ran"
        );
    }
    assert!(
        chain.nanos.iter().sum::<u64>() <= nanos[profile::BANK],
        "the chain sub-phases nest inside the bank phase"
    );
}
