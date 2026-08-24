//! Scheduler unit tests. Every gate names its red mutation in `tests/MUTATIONS.md` terms.

use super::*;
#[cfg(feature = "fault-injection")]
use std::sync::atomic::AtomicUsize;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicU64, Ordering},
};

struct Job {
    id: usize,
    transcript: Arc<Mutex<Vec<usize>>>,
    fail: bool,
}

impl NativeSchedulerJobV1 for Job {
    type Error = usize;

    fn execute(&mut self) -> Result<(), Self::Error> {
        self.transcript
            .lock()
            .expect("test transcript")
            .push(self.id);
        if self.fail { Err(self.id) } else { Ok(()) }
    }
}

fn even_ranges(count: usize, lanes: usize) -> Box<[RenderPartitionRangeV1]> {
    partition_weighted_units_v1(
        &vec![1_u64; count],
        NonZeroUsize::new(lanes).expect("lanes"),
        None,
    )
    .ranges
}

fn wave(transcript: Arc<Mutex<Vec<usize>>>, count: usize, lanes: usize) -> RenderWaveV1<Job> {
    let partitions = even_ranges(count, lanes)
        .iter()
        .map(|range| {
            RenderPartitionV1::new(
                *range,
                Job {
                    id: range.partition_id,
                    transcript: Arc::clone(&transcript),
                    fail: false,
                },
            )
        })
        .collect();
    RenderWaveV1::new(7, partitions).expect("canonical wave")
}

fn pool_shape(worker_count: usize) -> NativeWorkerPoolShapeV1 {
    NativeWorkerPoolShapeV1 {
        worker_count,
        spin_ns: 2,
    }
}

fn budget() -> RecoveryBudgetV1 {
    RecoveryBudgetV1 {
        recovery_iterations: 1 << 22,
        idle_spin_iterations: 1 << 12,
        linger_spin_iterations: 1 << 8,
    }
}

/// E9. Longest-processing-time-first balances one heavy bank unit against four scalar tails,
/// which a count split cannot do.
///
/// Red mutation: restore `partition_stable_units_v1`'s count split -- bin 0 then carries weight
/// 8 + 1 = 9 and the assertion on the maximum bin load fails.
#[test]
fn lpt_balances_a_heavy_bank_against_scalar_tails() {
    let weights = [8_u64, 1, 1, 1, 1];
    let split = partition_weighted_units_v1(&weights, NonZeroUsize::new(4).expect("lanes"), None);
    assert_eq!(split.unit_order.as_ref(), &[0, 1, 4, 2, 3]);
    let widths: Vec<usize> = split
        .ranges
        .iter()
        .map(|range| range.end_unit - range.first_unit)
        .collect();
    assert_eq!(widths, vec![1, 2, 1, 1]);
    let loads: Vec<u64> = split
        .ranges
        .iter()
        .map(|range| {
            split.unit_order[range.first_unit..range.end_unit]
                .iter()
                .map(|unit| weights[*unit])
                .sum()
        })
        .collect();
    assert_eq!(loads, vec![8, 2, 1, 1]);
    assert_eq!(loads.iter().copied().max(), Some(8));
}

/// E9. The cover stays canonical for arbitrary weights: contiguous, complete, no empty partition,
/// every unit exactly once. `RenderWaveV1::new` rejects anything else.
#[test]
fn lpt_is_contiguous_complete_and_never_empty() {
    let mut state = 0x9e37_79b9_u64;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    for units in 1_usize..=64 {
        for lanes in 1_usize..=16 {
            let weights: Vec<u64> = (0..units).map(|_| next() % 97 + 1).collect();
            let split = partition_weighted_units_v1(
                &weights,
                NonZeroUsize::new(lanes).expect("lanes"),
                None,
            );
            assert_eq!(split.ranges.len(), units.min(lanes));
            assert_eq!(split.unit_order.len(), units);
            let mut seen = vec![false; units];
            for unit in split.unit_order.iter() {
                assert!(!seen[*unit], "unit {unit} placed twice");
                seen[*unit] = true;
            }
            assert!(seen.into_iter().all(|value| value));
            let mut expected = 0_usize;
            for (index, range) in split.ranges.iter().enumerate() {
                assert_eq!(range.partition_id, index);
                assert_eq!(range.first_unit, expected);
                assert!(range.end_unit > range.first_unit, "empty partition");
                expected = range.end_unit;
            }
            assert_eq!(expected, units);
        }
    }
}

/// E9. The session output's unit is pinned to partition zero, which the coordinator always owns,
/// so the host copy-out can never read a trapped parcel.
#[test]
fn a_pinned_unit_is_always_in_partition_zero() {
    for pinned in 0_usize..6 {
        let weights = [1_u64, 9, 3, 4, 5, 2];
        let split = partition_weighted_units_v1(
            &weights,
            NonZeroUsize::new(3).expect("lanes"),
            Some(pinned),
        );
        let zero = &split.unit_order[..split.ranges[0].end_unit];
        assert!(
            zero.contains(&pinned),
            "pinned {pinned} left partition zero"
        );
    }
}

#[test]
fn preparation_without_workers_selects_sequential() {
    let scheduler = NativeSchedulerV1::<Job>::prepare(
        NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true, pool_shape(0)),
        4,
        9,
        budget(),
    )
    .expect("scheduler");
    assert_eq!(
        scheduler.selection(),
        SchedulerSelectionV1::Sequential(FallbackReasonV1::NoWorkers)
    );
    assert_eq!(scheduler.expected_workers(), 0);
}

/// Exact retained-byte accounting is checked before publication, never on the render path.
#[test]
fn an_impossible_pool_shape_is_rejected_before_publication() {
    let prepared = NativeSchedulerV1::<Job>::prepare(
        NativeSchedulerConfigV1::new(
            NonZeroUsize::new(usize::MAX).expect("lanes"),
            true,
            NativeWorkerPoolShapeV1 {
                worker_count: usize::MAX,
                spin_ns: 1,
            },
        ),
        4,
        9,
        budget(),
    );
    assert!(
        matches!(prepared, Err(SchedulerPrepareErrorV1::ResourceOverflow)),
        "an unbounded worker count cannot be accounted for"
    );
}

#[test]
fn disabled_and_narrow_preparation_select_the_same_sequential_parcels() {
    let transcript = Arc::new(Mutex::new(Vec::new()));
    let mut scheduler = NativeSchedulerV1::prepare(
        NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), false, pool_shape(3)),
        4,
        9,
        budget(),
    )
    .expect("scheduler");
    assert_eq!(
        scheduler.selection(),
        SchedulerSelectionV1::Sequential(FallbackReasonV1::DisabledByHost)
    );
    let mut rendered = wave(Arc::clone(&transcript), 4, 4);
    let report = scheduler
        .render_wave(None, &mut rendered)
        .expect("sequential render");
    assert_eq!(report.coordinator_jobs, 4);
    assert_eq!(report.coordinator_wakes, 0);
    assert!(rendered.all_recovered());
    assert_eq!(*transcript.lock().expect("transcript"), vec![0, 1, 2, 3]);
}

#[test]
fn native_workers_recover_move_only_parcels_and_select_errors_stably() {
    let transcript = Arc::new(Mutex::new(Vec::new()));
    let partitions = even_ranges(4, 4)
        .iter()
        .map(|range| {
            RenderPartitionV1::new(
                *range,
                Job {
                    id: range.partition_id,
                    transcript: Arc::clone(&transcript),
                    fail: range.partition_id == 2,
                },
            )
        })
        .collect();
    let mut rendered = RenderWaveV1::new(8, partitions).expect("wave");
    let (pool, mut lease) = NativeWorkerPoolV1::<Job>::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(3),
        #[cfg(feature = "fault-injection")]
        fault: FaultInjectionV1::None,
    })
    .expect("pool");
    let mut scheduler = NativeSchedulerV1::prepare(
        NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true, pool.shape()),
        4,
        10,
        budget(),
    )
    .expect("scheduler");
    assert_eq!(scheduler.selection(), SchedulerSelectionV1::Parallel);
    let mut reaped = [None; 3];
    scheduler.begin_block(
        Some(&mut lease),
        core::slice::from_mut(&mut rendered),
        &mut reaped,
    );
    let result = scheduler.render_wave(Some(&mut lease), &mut rendered);
    match result {
        Err(SchedulerDispatchErrorV1::Job(error)) => assert_eq!(error.partition_id, 2),
        _ => panic!("stable worker failure was not returned"),
    }
    scheduler.end_block(Some(&mut lease));
    assert!(rendered.all_recovered());
    let mut audits = [miso_engine_core::realtime::audit::AuditSnapshot::default(); 3];
    assert_eq!(
        scheduler.copy_worker_audit_snapshots(Some(&lease), &mut audits),
        3
    );
    assert!(audits.into_iter().all(|snapshot| snapshot.total() == 0));
    drop(lease);
    pool.stop_and_join();
}

/// The pool outlives its lease: a released lease comes back and can drive another plan, and the
/// pool joins cleanly whether or not the lease has been returned.
///
/// Red mutation: drop the `give_back` send in `WorkerLeaseV1::drop` -- `recover_lease` is `None`.
#[test]
fn a_released_lease_returns_to_its_pool() {
    let (mut pool, lease) = NativeWorkerPoolV1::<Job>::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(2),
        #[cfg(feature = "fault-injection")]
        fault: FaultInjectionV1::None,
    })
    .expect("pool");
    assert_eq!(pool.shape().worker_count, 2);
    assert!(pool.shape().spin_ns >= 1);
    assert!(pool.recover_lease().is_none(), "the lease is still out");
    assert_eq!(lease.worker_count(), 2);
    drop(lease);
    let recovered = pool.recover_lease().expect("released lease");
    assert_eq!(recovered.worker_count(), 2);
    drop(recovered);
    pool.stop_and_join();
}

#[test]
fn a_pool_stops_without_its_lease() {
    let (pool, lease) = NativeWorkerPoolV1::<Job>::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(2),
        #[cfg(feature = "fault-injection")]
        fault: FaultInjectionV1::None,
    })
    .expect("pool");
    pool.stop_and_join();
    drop(lease);
}

/// E6's mechanism: workers park once the block closes and are back at work after one wake.
///
/// Red mutation: never store `block_open = false` in `end_block` -- the workers keep spinning and
/// the parked assertion fails.
#[test]
fn workers_park_between_blocks_and_one_wake_brings_them_back() {
    let transcript = Arc::new(Mutex::new(Vec::new()));
    let (pool, mut lease) = NativeWorkerPoolV1::<Job>::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(3),
        #[cfg(feature = "fault-injection")]
        fault: FaultInjectionV1::None,
    })
    .expect("pool");
    lease.set_idle_spin(RecoveryBudgetV1 {
        recovery_iterations: 1 << 22,
        idle_spin_iterations: 1 << 10,
        linger_spin_iterations: 1 << 6,
    });
    let mut scheduler = NativeSchedulerV1::prepare(
        NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true, pool.shape()),
        4,
        10,
        budget(),
    )
    .expect("scheduler");
    let mut wakes = 0_u64;
    for block in 0..8 {
        let mut rendered = wave(Arc::clone(&transcript), 4, 4);
        let mut reaped = [None; 3];
        scheduler.begin_block(
            Some(&mut lease),
            core::slice::from_mut(&mut rendered),
            &mut reaped,
        );
        let report = scheduler
            .render_wave(Some(&mut lease), &mut rendered)
            .expect("parallel render");
        assert_eq!(report.worker_commands, 3);
        assert_eq!(report.worker_completions, 3);
        assert!(report.coordinator_wakes <= 1, "block {block}");
        wakes += report.coordinator_wakes;
        scheduler.end_block(Some(&mut lease));
        assert!(rendered.all_recovered());
        // Long enough for every worker to exhaust its idle spin and commit to parking.
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
        while std::time::Instant::now() < deadline
            && !(0..3).all(|worker| lease.test_is_parked(worker))
        {
            std::thread::yield_now();
        }
        assert!(
            (0..3).all(|worker| lease.test_is_parked(worker)),
            "block {block}: every worker parks once the block closes"
        );
    }
    assert!(
        wakes >= 7,
        "a parked pool needs a wake per block, saw {wakes}"
    );
    assert_eq!(transcript.lock().expect("transcript").len(), 8 * 4);
    drop(lease);
    pool.stop_and_join();
}

#[cfg(feature = "fault-injection")]
struct ProtocolAudit {
    executions: [AtomicUsize; 4],
    drops: [AtomicUsize; 4],
}

#[cfg(feature = "fault-injection")]
impl ProtocolAudit {
    fn new() -> Self {
        Self {
            executions: std::array::from_fn(|_| AtomicUsize::new(0)),
            drops: std::array::from_fn(|_| AtomicUsize::new(0)),
        }
    }

    fn assert_counts(&self, expected_executions: [usize; 4], expected_drops: [usize; 4]) {
        for partition in 0..4 {
            assert_eq!(
                self.executions[partition].load(Ordering::SeqCst),
                expected_executions[partition],
                "partition {partition} executions"
            );
            assert_eq!(
                self.drops[partition].load(Ordering::SeqCst),
                expected_drops[partition],
                "partition {partition} drops"
            );
        }
    }
}

#[cfg(feature = "fault-injection")]
struct ProtocolJob {
    partition_id: usize,
    audit: Arc<ProtocolAudit>,
    fail: bool,
    stall_until: Option<Arc<AtomicU64>>,
}

#[cfg(feature = "fault-injection")]
impl Drop for ProtocolJob {
    fn drop(&mut self) {
        self.audit.drops[self.partition_id].fetch_add(1, Ordering::SeqCst);
    }
}

#[cfg(feature = "fault-injection")]
impl NativeSchedulerJobV1 for ProtocolJob {
    type Error = usize;

    fn execute(&mut self) -> Result<(), Self::Error> {
        if let Some(gate) = &self.stall_until {
            while gate.load(Ordering::Acquire) == 0 {
                core::hint::spin_loop();
            }
        }
        self.audit.executions[self.partition_id].fetch_add(1, Ordering::SeqCst);
        if self.fail {
            Err(self.partition_id)
        } else {
            Ok(())
        }
    }
}

#[cfg(feature = "fault-injection")]
fn protocol_wave(
    level: u64,
    audit: Arc<ProtocolAudit>,
    failures: [bool; 4],
    stalls: [Option<Arc<AtomicU64>>; 4],
) -> RenderWaveV1<ProtocolJob> {
    let mut stalls = stalls.into_iter();
    let partitions = even_ranges(4, 4)
        .iter()
        .map(|range| {
            RenderPartitionV1::new(
                *range,
                ProtocolJob {
                    partition_id: range.partition_id,
                    audit: Arc::clone(&audit),
                    fail: failures[range.partition_id],
                    stall_until: stalls.next().flatten(),
                },
            )
        })
        .collect();
    RenderWaveV1::new(level, partitions).expect("wave")
}

#[cfg(feature = "fault-injection")]
#[allow(clippy::type_complexity)]
fn protocol_pool(
    fault: FaultInjectionV1,
    recovery_iterations: u64,
) -> Option<(
    NativeWorkerPoolV1<ProtocolJob>,
    WorkerLeaseV1<ProtocolJob>,
    NativeSchedulerV1<ProtocolJob>,
)> {
    let (pool, lease) = NativeWorkerPoolV1::<ProtocolJob>::start(NativeWorkerPoolConfigV1 {
        requested_workers: NonZeroUsize::new(3),
        fault,
    })
    .ok()?;
    let scheduler = NativeSchedulerV1::prepare(
        NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true, pool.shape())
            .with_fault(fault),
        4,
        11,
        RecoveryBudgetV1 {
            recovery_iterations,
            idle_spin_iterations: 1 << 12,
            linger_spin_iterations: 1 << 8,
        },
    )
    .expect("scheduler");
    Some((pool, lease, scheduler))
}

#[cfg(feature = "fault-injection")]
#[test]
fn startup_handshake_failure_never_publishes_a_pool() {
    assert!(
        NativeWorkerPoolV1::<ProtocolJob>::start(NativeWorkerPoolConfigV1 {
            requested_workers: NonZeroUsize::new(3),
            fault: FaultInjectionV1::StartupHandshakeFailure,
        })
        .is_err()
    );
}

#[cfg(feature = "fault-injection")]
#[test]
fn command_queue_full_preserves_unmoved_parcels() {
    let audit = Arc::new(ProtocolAudit::new());
    let (pool, mut lease, mut scheduler) =
        protocol_pool(FaultInjectionV1::CommandQueueFull { worker_id: 1 }, 1 << 22).expect("pool");
    let mut rendered = protocol_wave(11, Arc::clone(&audit), [false; 4], [None, None, None, None]);
    match scheduler.render_wave(Some(&mut lease), &mut rendered) {
        Err(SchedulerDispatchErrorV1::CommandQueueFull { worker_id }) => {
            assert_eq!(worker_id, 1);
        }
        _ => panic!("command publication fault was not reported"),
    }
    assert!(rendered.all_recovered());
    drop(rendered);
    // Commands are issued highest-partition-first (the wake-tree ordering rule), so partition 3
    // is already on its worker when partition 2's publication is reported full.
    audit.assert_counts([0, 0, 0, 1], [1, 1, 1, 1]);
    drop(lease);
    pool.stop_and_join();
}

#[cfg(feature = "fault-injection")]
#[test]
fn stale_generation_recovers_each_parcel_once() {
    let audit = Arc::new(ProtocolAudit::new());
    let (pool, mut lease, mut scheduler) =
        protocol_pool(FaultInjectionV1::StaleGeneration { worker_id: 2 }, 1 << 22).expect("pool");
    let mut rendered = protocol_wave(11, Arc::clone(&audit), [false; 4], [None, None, None, None]);
    match scheduler.render_wave(Some(&mut lease), &mut rendered) {
        Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id }) => {
            assert_eq!(worker_id, 2);
        }
        _ => panic!("stale generation was not reported"),
    }
    assert!(rendered.all_recovered());
    drop(rendered);
    audit.assert_counts([1, 1, 1, 1], [1, 1, 1, 1]);
    drop(lease);
    pool.stop_and_join();
}

#[cfg(feature = "fault-injection")]
#[test]
fn duplicate_completion_recovers_each_parcel_once() {
    let audit = Arc::new(ProtocolAudit::new());
    let (pool, mut lease, mut scheduler) = protocol_pool(
        FaultInjectionV1::DuplicateCompletion { worker_id: 2 },
        1 << 22,
    )
    .expect("pool");
    let mut rendered = protocol_wave(11, Arc::clone(&audit), [false; 4], [None, None, None, None]);
    match scheduler.render_wave(Some(&mut lease), &mut rendered) {
        Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id }) => {
            assert_eq!(worker_id, 2);
        }
        _ => panic!("duplicate completion was not reported"),
    }
    assert!(rendered.all_recovered());
    drop(rendered);
    audit.assert_counts([1, 1, 1, 1], [1, 1, 1, 1]);
    drop(lease);
    pool.stop_and_join();
}

#[cfg(feature = "fault-injection")]
#[test]
fn worker_errors_return_in_stable_partition_order() {
    let audit = Arc::new(ProtocolAudit::new());
    let (pool, mut lease, mut scheduler) =
        protocol_pool(FaultInjectionV1::None, 1 << 22).expect("pool");
    let mut rendered = protocol_wave(
        11,
        Arc::clone(&audit),
        [false, false, true, true],
        [None, None, None, None],
    );
    match scheduler.render_wave(Some(&mut lease), &mut rendered) {
        Err(SchedulerDispatchErrorV1::Job(failure)) => assert_eq!(failure.partition_id, 2),
        _ => panic!("stable partition order was not applied"),
    }
    assert!(rendered.all_recovered());
    drop(rendered);
    audit.assert_counts([1, 1, 1, 1], [1, 1, 1, 1]);
    drop(lease);
    pool.stop_and_join();
}

/// E4's scheduler half. A worker that misses its bounded deadline is declared dead for the life of
/// the lease: the wave returns `WorkerLost`, its parcel stays trapped, the next wave executes that
/// partition on the coordinator, and the parcel is reaped at a later block boundary.
///
/// Red mutation: make `recovery_iterations` unbounded (`u64::MAX`) -- the render call never
/// returns and the wall-clock guard in this test fires.
#[cfg(feature = "fault-injection")]
#[test]
fn a_late_worker_is_bounded_marked_dead_and_reaped_later() {
    let audit = Arc::new(ProtocolAudit::new());
    let gate = Arc::new(AtomicU64::new(0));
    // A budget healthy workers beat comfortably and the gated parcel can never meet.
    let (pool, mut lease, mut scheduler) =
        protocol_pool(FaultInjectionV1::None, 1 << 19).expect("pool");
    let mut rendered = protocol_wave(
        11,
        Arc::clone(&audit),
        [false; 4],
        [None, None, Some(Arc::clone(&gate)), None],
    );
    let started = std::time::Instant::now();
    let mut reaped = [None; 3];
    scheduler.begin_block(
        Some(&mut lease),
        core::slice::from_mut(&mut rendered),
        &mut reaped,
    );
    match scheduler.render_wave(Some(&mut lease), &mut rendered) {
        Err(SchedulerDispatchErrorV1::WorkerLost {
            worker_id,
            partition_id,
        }) => {
            assert_eq!((worker_id, partition_id), (1, 2));
        }
        _ => panic!("a late worker must be bounded, not awaited"),
    }
    assert!(
        started.elapsed() < std::time::Duration::from_secs(5),
        "recovery must be bounded"
    );
    assert!(rendered.is_trapped(2));
    assert!(lease.is_worker_dead(1));
    scheduler.end_block(Some(&mut lease));

    // The next block renders the dead worker's partition on the coordinator and issues nothing
    // to worker 1. The trapped partition 2 is still trapped, so nothing touches its parcel.
    let mut reaped = [None; 3];
    scheduler.begin_block(
        Some(&mut lease),
        core::slice::from_mut(&mut rendered),
        &mut reaped,
    );
    let report = scheduler
        .render_wave(Some(&mut lease), &mut rendered)
        .expect("degraded render");
    assert_eq!(report.worker_commands, 2, "worker 1 is never issued again");
    assert_eq!(report.dead_partitions_executed, 0, "partition 2 is trapped");
    scheduler.end_block(Some(&mut lease));

    // Release the stalled parcel and reap it at a later block boundary.
    gate.store(1, Ordering::Release);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    let mut reaped_at = None;
    while std::time::Instant::now() < deadline && reaped_at.is_none() {
        let mut reaped = [None; 3];
        let count = scheduler.begin_block(
            Some(&mut lease),
            core::slice::from_mut(&mut rendered),
            &mut reaped,
        );
        if count == 1 {
            reaped_at = reaped[0];
        }
        scheduler.end_block(Some(&mut lease));
    }
    assert_eq!(reaped_at, Some((0, 2)), "the trapped parcel came back");
    assert!(!rendered.is_trapped(2));
    assert!(rendered.all_recovered());

    // Reaping proves the worker is alive after all, so it is issued to again: a deadline miss
    // costs one degraded block, not the life of the lease.
    assert!(!lease.is_worker_dead(1));
    let mut reaped = [None; 3];
    scheduler.begin_block(
        Some(&mut lease),
        core::slice::from_mut(&mut rendered),
        &mut reaped,
    );
    let report = scheduler
        .render_wave(Some(&mut lease), &mut rendered)
        .expect("recovered render");
    assert_eq!(report.dead_partitions_executed, 0);
    assert_eq!(report.worker_commands, 3);
    scheduler.end_block(Some(&mut lease));
    drop(rendered);
    drop(lease);
    pool.stop_and_join();
}

/// A parcel that unwinds is still returned to the coordinator, and the fault is named by stable
/// partition, not by arrival order. Under D12's release profile `panic = "abort"` makes this
/// unreachable; it exists for unwinding (test) profiles only.
#[cfg(feature = "fault-injection")]
#[test]
fn a_panicking_parcel_is_returned_and_named_by_partition() {
    let audit = Arc::new(ProtocolAudit::new());
    let (pool, mut lease, mut scheduler) = protocol_pool(
        FaultInjectionV1::PanicWorker {
            worker_id: 1,
            wave_id: 11,
        },
        1 << 22,
    )
    .expect("pool");
    let mut rendered = protocol_wave(11, Arc::clone(&audit), [false; 4], [None, None, None, None]);
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let outcome = scheduler.render_wave(Some(&mut lease), &mut rendered);
    std::panic::set_hook(hook);
    match outcome {
        Err(SchedulerDispatchErrorV1::JobPanicked { partition_id }) => {
            assert_eq!(partition_id, 2);
        }
        _ => panic!("an unwinding parcel was not reported"),
    }
    assert!(rendered.all_recovered());
    drop(rendered);
    audit.assert_counts([1, 1, 0, 1], [1, 1, 1, 1]);
    drop(lease);
    pool.stop_and_join();
}
