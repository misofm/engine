//! Fixed 10,000-callback audit of the shared Issue-039 q128 native graph fixture.

use core::num::NonZeroUsize;
use miso_engine_bench_support::alloc as bench_alloc;
use std::sync::mpsc;

use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RenderIo, RenderTime, SwapOutcome,
    audit::{self, AuditSnapshot},
    plan_exchange,
};
use miso_engine_graph::{NativeGraphWorkerPoolV1, NativeWorkerPoolConfigV1, SchedulerSelectionV1};
use miso_engine_scheduler_fixture::{
    PoolChoice, PreparedQ128Fixture, Q128_QUANTUM_FRAMES, Q128RenderMode,
    prepare_q128_fixture_with_pool,
};

const CALLBACKS: u64 = 10_000;
const OBSERVERS_PER_CALLBACK: usize = 2;
const WORKERS: usize = 3;
/// One q128 block at 48 kHz, in nanoseconds: the pacing interval of the paced mode.
const BLOCK_PERIOD_NS: u128 = 2_666_666;

fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(std::env::args_os().count(), 1, "audit accepts no arguments");

    // Paced mode renders at the real 2.667 ms cadence so the workers actually park between
    // blocks; unpaced (steady) mode renders back to back so they never do. The pacing spin count
    // is calibrated here, on the control plane: nothing inside the armed interval reads a clock.
    let paced = std::env::var_os("MISO_ENGINE_SCHEDULER_AUDIT_PACED").is_some();

    // One pool serves both plans: the initial plan holds its lease and the replacement is
    // prepared without one, so the block-boundary hand-over is what arms the replacement.
    let (pool, lease) = NativeGraphWorkerPoolV1::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(WORKERS),
        ..NativeWorkerPoolConfigV1::default()
    })
    .unwrap_or_else(|_| panic!("q128 audit worker pool"));
    let shape = pool.shape();
    let pacing_iterations =
        u64::try_from(BLOCK_PERIOD_NS / u128::from(shape.spin_ns.max(1))).unwrap_or(u64::MAX);
    let initial = q128_fixture(39_901, PoolChoice::External(shape, Some(lease)));
    let replacement = q128_fixture(39_902, PoolChoice::External(shape, None));
    assert_eq!(initial.metadata, replacement.metadata);
    assert_eq!(initial.graph_sha256, replacement.graph_sha256);
    assert!(initial.pdc_samples > 0);
    assert!(initial.prepared_builtin_bank_count > 0);
    // #86 F3: every post-input node is a bank member on a vector host; the last bank of the
    // level is padded with identity lanes, so the audit exercises a padded bank and no scalar
    // post-input tail survives.
    assert_eq!(initial.scalar_builtin_tail_count, 0);
    assert!(
        initial.prepared_builtin_bank_lanes > 0
            && !initial
                .prepared_builtin_bank_member_count
                .is_multiple_of(initial.prepared_builtin_bank_lanes),
        "the audited layout must contain a padded bank"
    );
    assert_eq!(initial.metadata.selection, SchedulerSelectionV1::Parallel);
    assert_eq!(initial.metadata.resources.scheduler.selected_lanes, 4);
    assert_eq!(initial.metadata.resources.scheduler.worker_count, WORKERS);

    let fixture_id = miso_engine_scheduler_fixture::Q128_FIXTURE_ID;
    let pdc_samples = replacement.pdc_samples;
    let preparation_hash = replacement.metadata.test_preparation_transcript.hash;
    let replacement_observers = replacement.observer_transcript();
    assert_eq!(replacement_observers.record_count(), 0);

    // This marker is emitted after the pool is prepared and before any render scope.
    eprintln!("MISO_ENGINE_SCHEDULER_PHASE_PREPARED");
    let worker_tids = worker_thread_ids();
    assert_eq!(worker_tids.len(), WORKERS, "one TID per prepared worker");
    let (mut publisher, mut owner, retirer) = plan_exchange(
        initial.plan,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("plan exchange");
    publisher
        .publish(replacement.plan)
        .unwrap_or_else(|_| panic!("replacement publication"));

    let mut output = vec![0.0_f32; Q128_QUANTUM_FRAMES * 2];
    let output_address = output.as_ptr() as usize;
    let mut output_hash = 0xcbf2_9ce4_8422_2325_u64;
    audit::warm_up();
    audit::reset();
    let cpu_before = worker_cpu_ticks(&worker_tids);
    let wall_before = std::time::Instant::now();

    // The armed interval is delimited outside `RealtimePlanOwner::render` and worker dispatch.
    eprintln!("MISO_ENGINE_SCHEDULER_PHASE_ARMED");
    for block in 0..CALLBACKS {
        if paced {
            // A calibrated busy-wait, never a clock read: some kernels trace `clock_gettime`.
            let mut spins = 0_u64;
            while spins < pacing_iterations {
                core::hint::spin_loop();
                spins += 1;
            }
        }
        let report = owner
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(
                        &mut output,
                        2,
                        Q128_QUANTUM_FRAMES,
                        Q128_QUANTUM_FRAMES,
                    )
                    .expect("fixed q128 output"),
                },
                RenderTime {
                    absolute_sample: block * Q128_QUANTUM_FRAMES as u64,
                },
            )
            .expect("q128 native graph render");
        assert_eq!(report.render.plan_id, 39_902);
        assert_eq!(
            report.swap,
            if block == 0 {
                SwapOutcome::Applied
            } else {
                SwapOutcome::None
            }
        );
        for sample in &output {
            output_hash =
                (output_hash ^ u64::from(sample.to_bits())).wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    // All coordinator/worker audit reads occur only after every render scope returned.
    eprintln!("MISO_ENGINE_SCHEDULER_PHASE_DISARMED");
    let wall_seconds = wall_before.elapsed().as_secs_f64();
    let cpu_after = worker_cpu_ticks(&worker_tids);
    let worker_cpu_fraction: Vec<f64> = cpu_before
        .iter()
        .zip(&cpu_after)
        .map(|(before, after)| {
            let ticks = after.saturating_sub(*before) as f64;
            ticks / clock_ticks_per_second() / wall_seconds.max(f64::MIN_POSITIVE)
        })
        .collect();
    let dispatch = owner.dispatch_counters();
    assert_eq!(
        dispatch[3], 0,
        "the replacement rendered every block with the handed-over lease"
    );
    assert_eq!(dispatch[1], 0, "no worker missed its recovery deadline");
    assert_eq!(output.as_ptr() as usize, output_address);
    let coordinator = audit::snapshot();
    assert_eq!(coordinator.total(), 0);
    let mut workers = [AuditSnapshot::default(); WORKERS];
    assert_eq!(owner.copy_worker_audit_snapshots(&mut workers), WORKERS);
    assert!(workers.iter().all(|snapshot| snapshot.total() == 0));
    assert_eq!(
        replacement_observers.record_count(),
        CALLBACKS as usize * OBSERVERS_PER_CALLBACK
    );
    let observer_hash = replacement_observers.stable_hash();

    drop(publisher);
    let (sender, receiver) = mpsc::sync_channel(0);
    let retirement = std::thread::spawn(move || {
        let mut retirer = retirer;
        let retired = retirer.try_reclaim().expect("one displaced plan");
        drop(retired);
        drop(owner);
        // The active replacement and its scheduler are destroyed off the render thread.
        eprintln!("MISO_ENGINE_SCHEDULER_PHASE_RETIRED");
        sender
            .send(std::thread::current().id())
            .expect("retirement result");
    });
    let retirement_thread_id = receiver.recv().expect("retirement thread ID");
    assert_eq!(retirement.join().expect("retirement join"), ());
    pool.stop_and_join();
    println!(
        concat!(
            "{{\"schema_version\":3,\"kind\":\"native_scheduler_realtime_audit\",",
            "\"fixture_id\":\"{}\",\"callbacks\":{},\"sample_rate_hz\":48000,",
            "\"quantum_frames\":{},\"render_lanes\":4,\"worker_count\":3,",
            "\"paced\":{},\"plan_swaps\":1,\"pdc_samples\":{},\"preparation_hash\":{},",
            "\"observer_records\":{},\"observer_hash\":{},",
            "\"coordinator_wakes\":{},\"workers_lost\":{},",
            "\"dead_partitions_executed\":{},\"blocks_without_lease\":{},",
            "\"worker_cpu_fraction\":[{:.6},{:.6},{:.6}],",
            "\"retired_on_thread\":\"{:?}\",\"output_address\":{},",
            "\"output_hash\":{},\"coordinator_forbidden_total\":{},",
            "\"worker_forbidden_totals\":[{},{},{}]}}"
        ),
        fixture_id,
        CALLBACKS,
        Q128_QUANTUM_FRAMES,
        paced,
        pdc_samples,
        preparation_hash,
        replacement_observers.record_count(),
        observer_hash,
        dispatch[0],
        dispatch[1],
        dispatch[2],
        dispatch[3],
        worker_cpu_fraction[0],
        worker_cpu_fraction[1],
        worker_cpu_fraction[2],
        retirement_thread_id,
        output_address,
        output_hash,
        coordinator.total(),
        workers[0].total(),
        workers[1].total(),
        workers[2].total(),
    );
}

fn q128_fixture(plan_id: u64, pool: PoolChoice) -> PreparedQ128Fixture {
    prepare_q128_fixture_with_pool(
        48_000,
        4,
        Q128RenderMode::DependencyWaves,
        plan_id,
        CALLBACKS as usize * OBSERVERS_PER_CALLBACK,
        pool,
    )
    .unwrap_or_else(|error| panic!("q128 audit preparation failed: {error}"))
}

/// Thread IDs of the prepared workers, matched by the names the pool gives them.
///
/// Control plane only: this reads `/proc` before the armed interval and again after it.
fn worker_thread_ids() -> Vec<u32> {
    let mut tids: Vec<u32> = Vec::new();
    let Ok(entries) = std::fs::read_dir("/proc/self/task") else {
        return tids;
    };
    for entry in entries.flatten() {
        let Ok(tid) = entry.file_name().to_string_lossy().parse::<u32>() else {
            continue;
        };
        let Ok(comm) = std::fs::read_to_string(format!("/proc/self/task/{tid}/comm")) else {
            continue;
        };
        if comm.trim().starts_with("miso-scheduler-") {
            tids.push(tid);
        }
    }
    tids.sort_unstable();
    tids
}

/// Cumulative user+system ticks of each worker thread, from `/proc/<tid>/stat` fields 14 and 15.
///
/// The `comm` field can contain spaces and parentheses, so parsing starts after the last `)`.
fn worker_cpu_ticks(tids: &[u32]) -> Vec<u64> {
    tids.iter()
        .map(|tid| {
            let Ok(stat) = std::fs::read_to_string(format!("/proc/self/task/{tid}/stat")) else {
                return 0;
            };
            let Some(tail) = stat.rsplit_once(')').map(|(_, tail)| tail) else {
                return 0;
            };
            let fields: Vec<&str> = tail.split_whitespace().collect();
            // After the closing parenthesis, field 0 is `state` (stat field 3), so utime (14)
            // and stime (15) are indices 11 and 12.
            let utime = fields.get(11).and_then(|value| value.parse::<u64>().ok());
            let stime = fields.get(12).and_then(|value| value.parse::<u64>().ok());
            utime.unwrap_or(0).saturating_add(stime.unwrap_or(0))
        })
        .collect()
}

/// `CLK_TCK`. The workspace forbids `libc`, and every Linux target this audit runs on uses 100.
const fn clock_ticks_per_second() -> f64 {
    100.0
}
