//! Plumbing-floor diagnosis harness (`.github/ISSUE_SPECS/DRAFTS/PLAN.md`).
//!
//! Ignored by default: it prints measurements and asserts only bit-identity. Run it pinned:
//!
//! ```text
//! CARGO_INCREMENTAL=0 taskset -c 31 cargo test --release -p console-workload \
//!     --test plumbing_profile -- --ignored --nocapture --test-threads 1
//! ```
//!
//! Two instruments, both in-process because `perf` is unavailable on the diagnosis host:
//!
//! * `phase_profile` renders `sixty_four_track_plumbing_only` through the real plan with the
//!   `graph::test_only_phase_profile` probes on, and reports nanoseconds and derived core cycles
//!   per block per phase, beside the same row timed with the probes off.
//! * `kernel_replicas` times each phase's kernel work standalone over the row's own working set
//!   (sixty-four frozen stereo blocks, a plane-major arena, two host planes), then the fused
//!   one-pass candidates at group sizes 1, 2, 4 and 8, hoisted and broadcast coefficients, and
//!   asserts every candidate bit-identical to `mix2x2_block` then the grouped reduction.
//!
//! Cycles are derived, not counted: the core clock is calibrated from a dependent chain of scalar
//! `f32` adds (three cycles each on Zen 3 and Zen 4; the harness states the assumption) timed by
//! the same monotonic clock, so every "cycles" figure is `ns * core_hz` under that assumption.

use std::hint::black_box;
use std::time::Instant;

use console_workload::{PlanConfig, QUANTUM, SessionRuntime, Workload, source_block};
use graph::test_only_phase_profile as profile;
use lane::kernels::mix2x2_block;
use lane::{Lane, Simd8};

const WARMUP_BLOCKS: u64 = 512;
const PROFILED_BLOCKS: u64 = 4000;
const REPEATS: usize = 3;
const TRACKS: usize = 64;
/// Silence buffer, sixty-four inputs, one output: the arena the row reserves.
const ARENA_BUFFERS: usize = TRACKS + 2;
const REDUCE_GROUP: usize = 8;
const FADD_LATENCY_CYCLES: f64 = 3.0;

fn percentile(sorted: &[u64], p: f64) -> u64 {
    let rank = ((p / 100.0) * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

/// Median cost of one `Instant::now()` pair, the unit of probe overhead.
fn probe_cost_ns() -> f64 {
    let mut deltas: Vec<u64> = (0..20_000)
        .map(|_| {
            let a = Instant::now();
            let b = Instant::now();
            b.duration_since(a).as_nanos() as u64
        })
        .collect();
    deltas.sort_unstable();
    deltas[deltas.len() / 2] as f64
}

/// Core clock from a dependent `f32` add chain: `FADD_LATENCY_CYCLES` cycles per add.
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

const PHASE_NAMES: [&str; profile::COUNT] = [
    "enter (shape check, begin_observation_block)",
    "source set (begin_block + copy loop)",
    "bound units (FrozenGraphSource copies)",
    "route units (mix2x2 in place)",
    "output unit (64-input reduction)",
    "identity copy units (fan-in one, not in place)",
    "identity alias units (in place: dispatch only)",
    "other plain units",
    "bank units",
    "exit (loop end to return)",
];

const PHASE_SHORT: [&str; profile::COUNT] = [
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

#[test]
#[ignore = "measurement harness; run pinned with --ignored --nocapture"]
fn phase_profile() {
    let hz = core_hz();
    let probe_ns = probe_cost_ns();
    println!(
        "core clock (dependent-add calibration, {FADD_LATENCY_CYCLES} cycles/add): {:.3} GHz",
        hz / 1e9
    );
    println!("probe cost (one Instant::now pair): {probe_ns:.1} ns");
    let mut runtime =
        SessionRuntime::build(Workload::SixtyFourTrackPlumbingOnly, PlanConfig::BASELINE);
    let mut observation = 0_u64;
    profile::enable(false);
    for _ in 0..WARMUP_BLOCKS {
        runtime.render(observation).expect("warmup block");
        observation += 1;
    }
    for repeat in 0..REPEATS {
        // Probes off: the row as the benchmark sees it, one clock read per block.
        let mut samples = Vec::with_capacity(PROFILED_BLOCKS as usize);
        for _ in 0..PROFILED_BLOCKS {
            let start = Instant::now();
            let result = runtime.render(observation);
            let elapsed = start.elapsed().as_nanos() as u64;
            result.expect("block");
            observation += 1;
            samples.push(elapsed);
        }
        samples.sort_unstable();
        let min = samples[0];
        let p50 = percentile(&samples, 50.0);
        let p95 = percentile(&samples, 95.0);
        println!(
            "repeat {repeat}: probes off  min {min} ns  p50 {p50} ns  p95 {p95} ns  \
             (p50 = {:.0} cycles/block, {:.3} cycles/lane-sample)",
            p50 as f64 * hz / 1e9,
            p50 as f64 * hz / 1e9 / (TRACKS * QUANTUM * 2) as f64
        );
        // Probes on: phase totals.
        profile::enable(true);
        profile::reset();
        let start = Instant::now();
        for _ in 0..PROFILED_BLOCKS {
            runtime.render(observation).expect("block");
            observation += 1;
        }
        let total_ns = start.elapsed().as_nanos() as f64 / PROFILED_BLOCKS as f64;
        profile::enable(false);
        let (nanos, blocks, probes) = profile::snapshot();
        assert_eq!(blocks, PROFILED_BLOCKS);
        let runs = profile::runs();
        if repeat == 0 {
            let runs: Vec<String> = runs
                .iter()
                .filter(|(kind, units)| *kind < profile::COUNT && *units > 0)
                .map(|(kind, units)| format!("{units} x {}", PHASE_SHORT[*kind]))
                .collect();
            println!("unit runs in schedule order: {}", runs.join(", "));
        }
        let probes_per_block = probes as f64 / blocks as f64;
        let sum: u64 = nanos.iter().sum();
        println!(
            "repeat {repeat}: probes on   mean {total_ns:.0} ns/block, {probes_per_block:.1} probes/block \
             (~{:.0} ns probe cost/block), phases sum to {:.0} ns",
            probes_per_block * probe_ns,
            sum as f64 / blocks as f64
        );
        println!(
            "  {:<46} {:>10} {:>12} {:>7}",
            "phase", "ns/block", "cycles/block", "share"
        );
        for (index, name) in PHASE_NAMES.iter().enumerate() {
            let ns = nanos[index] as f64 / blocks as f64;
            if ns == 0.0 {
                continue;
            }
            println!(
                "  {:<46} {:>10.1} {:>12.0} {:>6.1}%",
                name,
                ns,
                ns * hz / 1e9,
                100.0 * nanos[index] as f64 / sum as f64
            );
        }
    }
}

/// The row's working set, laid out as the executor lays it out.
struct Replica {
    /// Sixty-four frozen stereo blocks: the `FrozenGraphSource` members.
    sources: Vec<([f32; QUANTUM], [f32; QUANTUM])>,
    /// Plane-major arena: `[plane][buffer][frame]`, buffer 0 the silence slot, `1..=64` the inputs.
    arena: Vec<f32>,
    coefficients: Vec<[f32; 4]>,
    host_left: Vec<f32>,
    host_right: Vec<f32>,
}

impl Replica {
    fn new() -> Self {
        let sources = (0..TRACKS)
            .map(|track| {
                let block = source_block(track, false);
                let mut left = [0.0; QUANTUM];
                let mut right = [0.0; QUANTUM];
                left.copy_from_slice(&block[..QUANTUM]);
                right.copy_from_slice(&block[QUANTUM..2 * QUANTUM]);
                (left, right)
            })
            .collect();
        // Deterministic non-trivial 2x2s of unit-ish magnitude from an integer hash (no libm, so
        // the same words on every target); the arithmetic cost is data independent and no
        // coefficient is a signed zero.
        let unit = |seed: u32| -> f32 {
            let hashed = seed.wrapping_mul(2_654_435_761).rotate_left(13) ^ 0x9E37_79B9;
            (hashed >> 8) as f32 / (1u32 << 24) as f32
        };
        let coefficients = (0..TRACKS)
            .map(|track| {
                let t = track as u32;
                [
                    0.5 + 0.4 * unit(4 * t),
                    0.2 * unit(4 * t + 1) - 0.1,
                    0.2 * unit(4 * t + 2) - 0.1,
                    0.5 + 0.4 * unit(4 * t + 3),
                ]
            })
            .collect();
        Self {
            sources,
            arena: vec![0.0; 2 * ARENA_BUFFERS * QUANTUM],
            coefficients,
            host_left: vec![0.0; QUANTUM],
            host_right: vec![0.0; QUANTUM],
        }
    }

    fn buffer_range(buffer: usize) -> core::ops::Range<usize> {
        buffer * QUANTUM..(buffer + 1) * QUANTUM
    }

    /// Phase: sixty-four `FrozenGraphSource::process` copies into the arena.
    fn copy_phase(&mut self) {
        let (left, right) = self.arena.split_at_mut(ARENA_BUFFERS * QUANTUM);
        for (track, (source_left, source_right)) in self.sources.iter().enumerate() {
            let range = Self::buffer_range(track + 1);
            left[range.clone()].copy_from_slice(source_left);
            right[range].copy_from_slice(source_right);
        }
    }

    /// Phase: sixty-four in-place `mix2x2_block` route passes.
    fn route_phase(&mut self) {
        let (left, right) = self.arena.split_at_mut(ARENA_BUFFERS * QUANTUM);
        for track in 0..TRACKS {
            let range = Self::buffer_range(track + 1);
            mix2x2_block::<Simd8>(
                &mut left[range.clone()],
                &mut right[range],
                self.coefficients[track],
            );
        }
    }

    /// Phase: the Output op's reduction into the host planes, `reduce_many_into`'s shape: groups
    /// of eight, the first group storing, each later group reloading the running sum.
    fn reduce_phase(&mut self) {
        let (left, right) = self.arena.split_at(ARENA_BUFFERS * QUANTUM);
        for (plane, host) in [(left, &mut self.host_left), (right, &mut self.host_right)] {
            for group in 0..TRACKS / REDUCE_GROUP {
                let sources: [&[f32]; REDUCE_GROUP] = core::array::from_fn(|i| {
                    &plane[Self::buffer_range(group * REDUCE_GROUP + i + 1)]
                });
                accumulate_run_replica::<Simd8, REDUCE_GROUP>(host, sources, group == 0);
            }
        }
    }
}

/// `accumulate_run::<L, N>` as `crates/graph/src/runtime.rs` writes it: every source walked by its
/// own `chunks_exact` iterator, the first group storing `s0 + s1 + ...`, later groups
/// `((out + s0) + s1) + ...`.
#[inline(never)]
fn accumulate_run_replica<L: Lane, const N: usize>(
    output: &mut [f32],
    sources: [&[f32]; N],
    initial_store: bool,
) -> bool {
    let mut chunks = sources.map(|source| source.chunks_exact(L::WIDTH));
    if initial_store {
        let Some((first, rest)) = chunks.split_first_mut() else {
            return false;
        };
        for out in output.chunks_exact_mut(L::WIDTH) {
            let Some(acc) = first
                .next()
                .and_then(|chunk| add_chunks_replica(L::load(chunk), rest))
            else {
                return false;
            };
            acc.store(out);
        }
    } else {
        for out in output.chunks_exact_mut(L::WIDTH) {
            let Some(acc) = add_chunks_replica(L::load(out), &mut chunks) else {
                return false;
            };
            acc.store(out);
        }
    }
    true
}

#[inline(always)]
fn add_chunks_replica<L: Lane>(
    mut acc: L,
    chunks: &mut [core::slice::ChunksExact<'_, f32>],
) -> Option<L> {
    for chunk in chunks {
        acc = acc.add(L::load(chunk.next()?));
    }
    Some(acc)
}

/// The fused one-pass candidate: per group of `G` tracks, per chunk, load both planes of each
/// track, apply its 2x2 in registers (`lr.fma(r, ll.mul(l))`, `rr.fma(r, rl.mul(l))`), and
/// accumulate in edge order -- `value = mix(in0)` for the first group, `load(out)` after -- then
/// store. `HOIST` splats the group's coefficients once per group; otherwise each chunk broadcasts
/// them from the table. `inputs` are the `(left, right)` planes read, in edge order. Written
/// with const-generic arrays, as `accumulate_run::<L, N>` is, so `G` is a compile-time shape.
#[inline(never)]
fn fused_route_reduce<const G: usize, const HOIST: bool>(
    inputs: &[(&[f32], &[f32])],
    coefficients: &[[f32; 4]],
    host_left: &mut [f32],
    host_right: &mut [f32],
) {
    let mut first_group = true;
    for (group, group_coefficients) in inputs.chunks_exact(G).zip(coefficients.chunks_exact(G)) {
        let group: &[(&[f32], &[f32]); G] = group.try_into().expect("group");
        let group_coefficients: &[[f32; 4]; G] = group_coefficients.try_into().expect("group");
        let hoisted: [[Simd8; 4]; G] = core::array::from_fn(|i| {
            let c = group_coefficients[i];
            if HOIST {
                [
                    Simd8::splat(c[0]),
                    Simd8::splat(c[1]),
                    Simd8::splat(c[2]),
                    Simd8::splat(c[3]),
                ]
            } else {
                [Simd8::splat(0.0); 4]
            }
        });
        let mut lefts: [core::slice::ChunksExact<'_, f32>; G] =
            core::array::from_fn(|i| group[i].0.chunks_exact(Simd8::WIDTH));
        let mut rights: [core::slice::ChunksExact<'_, f32>; G] =
            core::array::from_fn(|i| group[i].1.chunks_exact(Simd8::WIDTH));
        let coefficients_of = |i: usize| -> [Simd8; 4] {
            if HOIST {
                hoisted[i]
            } else {
                let c = group_coefficients[i];
                [
                    Simd8::splat(c[0]),
                    Simd8::splat(c[1]),
                    Simd8::splat(c[2]),
                    Simd8::splat(c[3]),
                ]
            }
        };
        for (out_left, out_right) in host_left
            .chunks_exact_mut(Simd8::WIDTH)
            .zip(host_right.chunks_exact_mut(Simd8::WIDTH))
        {
            let l = Simd8::load(lefts[0].next().expect("chunk"));
            let r = Simd8::load(rights[0].next().expect("chunk"));
            let [ll, lr, rl, rr] = coefficients_of(0);
            let mixed_left = lr.fma(r, ll.mul(l));
            let mixed_right = rr.fma(r, rl.mul(l));
            let (mut acc_left, mut acc_right) = if first_group {
                (mixed_left, mixed_right)
            } else {
                (
                    Simd8::load(out_left).add(mixed_left),
                    Simd8::load(out_right).add(mixed_right),
                )
            };
            for i in 1..G {
                let l = Simd8::load(lefts[i].next().expect("chunk"));
                let r = Simd8::load(rights[i].next().expect("chunk"));
                let [ll, lr, rl, rr] = coefficients_of(i);
                acc_left = acc_left.add(lr.fma(r, ll.mul(l)));
                acc_right = acc_right.add(rr.fma(r, rl.mul(l)));
            }
            acc_left.store(out_left);
            acc_right.store(out_right);
        }
        first_group = false;
    }
}

/// Minimum per-iteration nanoseconds over `batches` batches of `per_batch` iterations.
fn time_min(batches: usize, per_batch: usize, mut body: impl FnMut()) -> f64 {
    let mut best = f64::MAX;
    for _ in 0..batches {
        let start = Instant::now();
        for _ in 0..per_batch {
            body();
        }
        best = best.min(start.elapsed().as_nanos() as f64 / per_batch as f64);
    }
    best
}

#[test]
#[ignore = "measurement harness; run pinned with --ignored --nocapture"]
fn kernel_replicas() {
    let hz = core_hz();
    println!(
        "core clock (dependent-add calibration): {:.3} GHz",
        hz / 1e9
    );
    let lane_samples = (TRACKS * QUANTUM * 2) as f64;
    let report = |name: &str, ns: f64| {
        println!(
            "  {:<58} {:>9.0} ns {:>9.0} cycles {:>7.3} cycles/lane-sample",
            name,
            ns,
            ns * hz / 1e9,
            ns * hz / 1e9 / lane_samples
        );
    };
    let mut replica = Replica::new();
    // Reference bits: copy, route, reduce.
    replica.copy_phase();
    replica.route_phase();
    replica.reduce_phase();
    let reference = (replica.host_left.clone(), replica.host_right.clone());
    // Bit-identity of every fused candidate, from the arena (post-copy, pre-route words) and
    // straight from the frozen sources (no copy at all).
    macro_rules! check {
        ($g:literal, $hoist:literal) => {{
            replica.copy_phase();
            let mut host_left = vec![0.0; QUANTUM];
            let mut host_right = vec![0.0; QUANTUM];
            {
                let (left, right) = replica.arena.split_at(ARENA_BUFFERS * QUANTUM);
                let inputs: Vec<(&[f32], &[f32])> = (1..=TRACKS)
                    .map(|b| {
                        (
                            &left[Replica::buffer_range(b)],
                            &right[Replica::buffer_range(b)],
                        )
                    })
                    .collect();
                fused_route_reduce::<$g, $hoist>(
                    &inputs,
                    &replica.coefficients,
                    &mut host_left,
                    &mut host_right,
                );
            }
            assert!(
                host_left
                    .iter()
                    .zip(&reference.0)
                    .all(|(a, b)| a.to_bits() == b.to_bits())
                    && host_right
                        .iter()
                        .zip(&reference.1)
                        .all(|(a, b)| a.to_bits() == b.to_bits()),
                "fused G={} hoist={} from the arena is not bit-identical",
                $g,
                $hoist
            );
            let inputs: Vec<(&[f32], &[f32])> = replica
                .sources
                .iter()
                .map(|(l, r)| (&l[..], &r[..]))
                .collect();
            fused_route_reduce::<$g, $hoist>(
                &inputs,
                &replica.coefficients,
                &mut host_left,
                &mut host_right,
            );
            assert!(
                host_left
                    .iter()
                    .zip(&reference.0)
                    .all(|(a, b)| a.to_bits() == b.to_bits())
                    && host_right
                        .iter()
                        .zip(&reference.1)
                        .all(|(a, b)| a.to_bits() == b.to_bits()),
                "fused G={} hoist={} from the sources is not bit-identical",
                $g,
                $hoist
            );
        }};
    }
    check!(1, true);
    check!(2, true);
    check!(4, true);
    check!(8, true);
    check!(1, false);
    check!(2, false);
    check!(4, false);
    check!(8, false);
    println!("every fused candidate is bit-identical to mix2x2_block then the grouped reduction");

    const BATCHES: usize = 30;
    const PER_BATCH: usize = 200;
    for repeat in 0..REPEATS {
        println!("repeat {repeat}:");
        let copy = time_min(BATCHES, PER_BATCH, || replica.copy_phase());
        report("copy phase: 64 x 2 x 128-word copy_from_slice", copy);
        // The route pass must not be timed over its own output words (it is in place and would
        // grow), so each iteration recopies first and the copy is subtracted.
        let copy_route = time_min(BATCHES, PER_BATCH, || {
            replica.copy_phase();
            replica.route_phase();
        });
        report(
            "route phase: 64 x mix2x2_block::<Simd8> in place (copy subtracted)",
            copy_route - copy,
        );
        let reduce = time_min(BATCHES, PER_BATCH, || replica.reduce_phase());
        report(
            "reduce phase: 8 groups of 8 into the host planes, both planes",
            reduce,
        );
        let all = time_min(BATCHES, PER_BATCH, || {
            replica.copy_phase();
            replica.route_phase();
            replica.reduce_phase();
        });
        report("copy + route + reduce, one iteration", all);
        macro_rules! fused_timing {
            ($g:literal, $hoist:literal) => {{
                let arena_ns = time_min(BATCHES, PER_BATCH, || {
                    let (left, right) = replica.arena.split_at(ARENA_BUFFERS * QUANTUM);
                    let inputs: [(&[f32], &[f32]); TRACKS] = core::array::from_fn(|t| {
                        (
                            &left[Replica::buffer_range(t + 1)],
                            &right[Replica::buffer_range(t + 1)],
                        )
                    });
                    fused_route_reduce::<$g, $hoist>(
                        &inputs,
                        &replica.coefficients,
                        &mut replica.host_left,
                        &mut replica.host_right,
                    );
                });
                let source_ns = time_min(BATCHES, PER_BATCH, || {
                    let inputs: [(&[f32], &[f32]); TRACKS] = core::array::from_fn(|t| {
                        (&replica.sources[t].0[..], &replica.sources[t].1[..])
                    });
                    fused_route_reduce::<$g, $hoist>(
                        &inputs,
                        &replica.coefficients,
                        &mut replica.host_left,
                        &mut replica.host_right,
                    );
                });
                report(
                    &format!(
                        "fused route+reduce G={} hoist={} from the arena",
                        $g, $hoist
                    ),
                    arena_ns,
                );
                report(
                    &format!(
                        "fused route+reduce G={} hoist={} from the sources (no copy)",
                        $g, $hoist
                    ),
                    source_ns,
                );
            }};
        }
        fused_timing!(1, true);
        fused_timing!(2, true);
        fused_timing!(4, true);
        fused_timing!(8, true);
        fused_timing!(1, false);
        fused_timing!(2, false);
        fused_timing!(4, false);
        fused_timing!(8, false);
    }
    black_box(&replica.host_left);
}
