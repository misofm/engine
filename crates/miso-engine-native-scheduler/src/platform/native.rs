//! Persistent worker pool, its lease, and the block protocol (issue 100).
//!
//! Threads are owned by [`NativeWorkerPoolV1`], which the control plane starts once. The
//! coordinator endpoints travel with the active plan as a [`WorkerLeaseV1`], so a structural
//! change hands the lease over at the block-boundary swap instead of spawning and joining a
//! thread per plan.

#[cfg(feature = "fault-injection")]
use crate::FaultInjectionV1;
use crate::{
    NativeSchedulerJobV1, NativeSchedulerV1, NativeWorkerPoolConfigV1, NativeWorkerPoolShapeV1,
    RenderWaveV1, SchedulerDispatchErrorV1, SchedulerDispatchReportV1, SchedulerJobFailureV1,
    SchedulerPrepareErrorV1, SchedulerSelectionV1, execute_sequential,
};
use core::{
    cell::Cell,
    num::NonZeroUsize,
    sync::atomic::{AtomicBool, AtomicU64, Ordering, fence},
};
use miso_engine_core::realtime::{
    Consumer, Producer, QueueGeneration, audit, bounded_spsc_move, bounded_spsc_retained_payload,
};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, mpsc},
    thread::{self, JoinHandle, Thread},
};

struct WorkerCommand<J> {
    generation: u64,
    wave_id: u64,
    partition_id: usize,
    parcel: J,
    #[cfg(feature = "fault-injection")]
    fault: FaultInjectionV1,
}

/// What a worker did with its parcel.
enum WorkerOutcome<E> {
    Completed(Result<(), E>),
    Panicked,
}

struct WorkerCompletion<J: NativeSchedulerJobV1> {
    generation: u64,
    wave_id: u64,
    partition_id: usize,
    outcome: WorkerOutcome<J::Error>,
    audit: audit::AuditSnapshot,
    parcel: J,
}

enum WorkerMessage<J> {
    Run(WorkerCommand<J>),
}

/// Per-worker state both sides of the wake protocol touch.
#[repr(align(64))]
struct WorkerShared {
    /// `true` while the worker is committed to parking; see the Dekker pair in `worker_loop`.
    parked: AtomicBool,
    thread: Thread,
}

/// Pool-wide state, shared with every worker for the life of the pool.
struct PoolShared {
    workers: Box<[WorkerShared]>,
    /// `true` between [`NativeSchedulerV1::begin_block`] and `end_block`: workers spin, they do
    /// not park, so a multi-wave block costs exactly one wake.
    block_open: AtomicBool,
    stop: AtomicBool,
    /// Idle spin iterations, published by the lease holder at bind (about one render quantum).
    idle_spin: AtomicU64,
}

impl PoolShared {
    // REALTIME_POLICY_BEGIN
    /// Wake this worker's binary-tree children. Called by a worker that has just been issued a
    /// command, so one coordinator wake reaches every issued worker.
    fn wake_children(&self, id: usize) {
        let mut child = 2 * id + 1;
        let last = child + 1;
        while child <= last {
            if let Some(worker) = self.workers.get(child) {
                fence(Ordering::SeqCst);
                if worker.parked.load(Ordering::SeqCst) {
                    worker.thread.unpark();
                }
            }
            child += 1;
        }
    }
    // REALTIME_POLICY_END
}

/// One coordinator endpoint: the two queues of one worker plus its liveness.
struct CoordinatorEndpoint<J: NativeSchedulerJobV1> {
    commands: Producer<WorkerMessage<J>>,
    completions: Consumer<WorkerCompletion<J>>,
    /// Set once this worker misses a recovery deadline; never cleared for the life of the lease.
    dead: bool,
    /// The trapped assignment this worker still owns: `(wave_id, partition_id, generation)`.
    outstanding: Option<(u64, usize, u64)>,
    /// A wrong-generation parcel a dead worker returned. Moved here, never dropped on render.
    stale: Option<J>,
    audit: audit::AuditSnapshot,
}

/// The coordinator half of a started pool, owned by exactly one prepared plan at a time.
///
/// The lease is `Send` (it is handed over at a block boundary) and deliberately `!Sync`.
pub struct WorkerLeaseV1<J: NativeSchedulerJobV1> {
    shared: Arc<PoolShared>,
    endpoints: Option<Box<[CoordinatorEndpoint<J>]>>,
    give_back: mpsc::Sender<Box<[CoordinatorEndpoint<J>]>>,
    _not_sync: Cell<()>,
}

impl<J: NativeSchedulerJobV1> WorkerLeaseV1<J> {
    /// Auxiliary workers this lease drives.
    #[must_use]
    pub fn worker_count(&self) -> usize {
        self.endpoints
            .as_ref()
            .map_or(0, |endpoints| endpoints.len())
    }

    /// Publish the idle-spin budget the plan derived from its render quantum.
    ///
    /// Control plane: called at bind and at hand-over, never inside a block.
    pub fn set_idle_spin(&mut self, iterations: u64) {
        self.shared.idle_spin.store(iterations, Ordering::Release);
    }

    /// Whether a worker has been declared dead for the life of this lease.
    #[must_use]
    pub fn is_worker_dead(&self, worker_id: usize) -> bool {
        self.endpoints
            .as_ref()
            .and_then(|endpoints| endpoints.get(worker_id))
            .is_some_and(|endpoint| endpoint.dead)
    }

    /// Whether worker `worker_id` is committed to parking. Test observation only.
    #[cfg(test)]
    pub(crate) fn test_is_parked(&self, worker_id: usize) -> bool {
        self.shared
            .workers
            .get(worker_id)
            .is_some_and(|worker| worker.parked.load(Ordering::SeqCst))
    }

    fn endpoints_mut(&mut self) -> &mut [CoordinatorEndpoint<J>] {
        self.endpoints.as_deref_mut().unwrap_or_default()
    }
}

impl<J: NativeSchedulerJobV1> Drop for WorkerLeaseV1<J> {
    fn drop(&mut self) {
        debug_assert!(
            !audit::is_render_scope_active(),
            "a worker lease is released off the render thread"
        );
        if let Some(endpoints) = self.endpoints.take() {
            // The pool may already be gone (it is stopped before its lease is reclaimed); then
            // the endpoints are simply dropped here, on the control thread.
            let _ = self.give_back.send(endpoints);
        }
    }
}

/// Control-plane owner of the auxiliary render threads.
///
/// The pool outlives every plan that borrows it. Dropping it stops and joins every worker.
pub struct NativeWorkerPoolV1<J: NativeSchedulerJobV1> {
    shared: Arc<PoolShared>,
    handles: Vec<JoinHandle<()>>,
    returned: mpsc::Receiver<Box<[CoordinatorEndpoint<J>]>>,
    give_back: mpsc::Sender<Box<[CoordinatorEndpoint<J>]>>,
    shape: NativeWorkerPoolShapeV1,
}

impl<J: NativeSchedulerJobV1> NativeWorkerPoolV1<J> {
    /// Start every auxiliary worker and return the pool with its first lease.
    ///
    /// This is control-plane work: it spawns threads, calibrates the spin cost with a clock, and
    /// completes a rendezvous handshake with every worker before returning.
    ///
    /// # Errors
    /// Returns [`SchedulerPrepareErrorV1::WorkerStart`] if a thread could not be started or a
    /// worker did not acknowledge, and [`SchedulerPrepareErrorV1::ResourceOverflow`] if the exact
    /// queue accounting overflows. Nothing is left running on either path.
    pub fn start(
        config: NativeWorkerPoolConfigV1,
    ) -> Result<(Self, WorkerLeaseV1<J>), SchedulerPrepareErrorV1> {
        let worker_count = config.requested_workers.map_or_else(
            || {
                thread::available_parallelism()
                    .map(NonZeroUsize::get)
                    .unwrap_or(1)
                    .saturating_sub(1)
            },
            NonZeroUsize::get,
        );
        let spin_ns = calibrate_spin_ns();
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(worker_count);
        let mut senders: Vec<mpsc::SyncSender<Arc<PoolShared>>> = Vec::with_capacity(worker_count);
        let mut readies: Vec<mpsc::Receiver<()>> = Vec::with_capacity(worker_count);
        let mut endpoints: Vec<CoordinatorEndpoint<J>> = Vec::with_capacity(worker_count);
        // Phase 1: spawn every thread and take its `Thread` handle. A worker blocks on its
        // rendezvous channel until phase 2 hands it the shared state.
        for worker_id in 0..worker_count {
            let worker_id_u64 =
                u64::try_from(worker_id).map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
            let (commands, worker_commands) = bounded_spsc_move(
                NonZeroUsize::new(1).expect("nonzero queue capacity"),
                QueueGeneration(2 * worker_id_u64),
            )
            .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
            let (worker_completions, completions) = bounded_spsc_move(
                NonZeroUsize::new(1).expect("nonzero queue capacity"),
                QueueGeneration(2 * worker_id_u64 + 1),
            )
            .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
            #[cfg(feature = "fault-injection")]
            let withhold_ready =
                worker_id == 0 && config.fault == FaultInjectionV1::StartupHandshakeFailure;
            let (shared_sender, shared_receiver) = mpsc::sync_channel::<Arc<PoolShared>>(0);
            let (ready_sender, ready_receiver) = mpsc::channel();
            let spawned = thread::Builder::new()
                .name(format!("miso-scheduler-{worker_id}"))
                .spawn(move || {
                    audit::warm_up();
                    audit::reset();
                    let Ok(shared) = shared_receiver.recv() else {
                        return;
                    };
                    #[cfg(feature = "fault-injection")]
                    if withhold_ready {
                        return;
                    }
                    if ready_sender.send(()).is_ok() {
                        worker_loop(worker_id, shared, worker_commands, worker_completions);
                    }
                });
            match spawned {
                Ok(handle) => handles.push(handle),
                Err(_) => {
                    drop(senders);
                    for handle in handles {
                        let _ = handle.join();
                    }
                    return Err(SchedulerPrepareErrorV1::WorkerStart);
                }
            }
            senders.push(shared_sender);
            readies.push(ready_receiver);
            endpoints.push(CoordinatorEndpoint {
                commands,
                completions,
                dead: false,
                outstanding: None,
                stale: None,
                audit: audit::AuditSnapshot::default(),
            });
        }
        // Phase 2: every `Thread` handle exists, so the wake tree can be built and published.
        let shared = Arc::new(PoolShared {
            workers: handles
                .iter()
                .map(|handle| WorkerShared {
                    parked: AtomicBool::new(false),
                    thread: handle.thread().clone(),
                })
                .collect(),
            block_open: AtomicBool::new(false),
            stop: AtomicBool::new(false),
            idle_spin: AtomicU64::new(1 << 14),
        });
        let mut started = true;
        for sender in &senders {
            started &= sender.send(Arc::clone(&shared)).is_ok();
        }
        for ready in &readies {
            started &= ready.recv().is_ok();
        }
        if !started {
            shared.stop.store(true, Ordering::Release);
            for worker in &shared.workers {
                worker.thread.unpark();
            }
            for handle in handles {
                let _ = handle.join();
            }
            return Err(SchedulerPrepareErrorV1::WorkerStart);
        }
        let (give_back, returned) = mpsc::channel();
        let lease = WorkerLeaseV1 {
            shared: Arc::clone(&shared),
            endpoints: Some(endpoints.into_boxed_slice()),
            give_back: give_back.clone(),
            _not_sync: Cell::new(()),
        };
        Ok((
            Self {
                shared,
                handles,
                returned,
                give_back,
                shape: NativeWorkerPoolShapeV1 {
                    worker_count,
                    spin_ns,
                },
            },
            lease,
        ))
    }

    /// Address-free description of this pool.
    #[must_use]
    pub const fn shape(&self) -> NativeWorkerPoolShapeV1 {
        self.shape
    }

    /// Take back a lease that a retired plan released.
    pub fn recover_lease(&mut self) -> Option<WorkerLeaseV1<J>> {
        self.returned
            .try_recv()
            .ok()
            .map(|endpoints| WorkerLeaseV1 {
                shared: Arc::clone(&self.shared),
                endpoints: Some(endpoints),
                give_back: self.give_back.clone(),
                _not_sync: Cell::new(()),
            })
    }

    /// Stop and join every worker. This works whether or not the lease has come back.
    pub fn stop_and_join(mut self) {
        self.shutdown();
    }

    fn shutdown(&mut self) {
        self.shared.stop.store(true, Ordering::SeqCst);
        for worker in &self.shared.workers {
            worker.thread.unpark();
        }
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

impl<J: NativeSchedulerJobV1> Drop for NativeWorkerPoolV1<J> {
    fn drop(&mut self) {
        self.shutdown();
    }
}

/// Measure the cost of one idle spin iteration, so a wall-clock budget becomes an iteration count.
///
/// Control plane only: the render path never reads a clock. `pause` latency varies by an order of
/// magnitude across CPUs, so this is measured rather than assumed; the result is clamped to at
/// least one nanosecond so a garbage measurement cannot produce an unbounded budget.
fn calibrate_spin_ns() -> u32 {
    const ITERATIONS: u64 = 1_000_000;
    let flag = AtomicBool::new(false);
    let start = std::time::Instant::now();
    let mut index = 0_u64;
    while index < ITERATIONS {
        let _ = core::hint::black_box(flag.load(Ordering::Acquire));
        core::hint::spin_loop();
        index += 1;
    }
    let elapsed = start.elapsed().as_nanos();
    let per_iteration = elapsed.div_ceil(u128::from(ITERATIONS));
    u32::try_from(per_iteration).unwrap_or(u32::MAX).max(1)
}

pub(crate) fn retained_queue_bytes<J: NativeSchedulerJobV1>(
    worker_count: usize,
) -> Result<usize, SchedulerPrepareErrorV1> {
    if worker_count == 0 {
        return Ok(0);
    }
    let capacity = NonZeroUsize::new(1).expect("nonzero queue capacity");
    let command_bytes = bounded_spsc_retained_payload::<WorkerMessage<J>>(capacity)
        .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?
        .total_bytes()
        .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
    let completion_bytes = bounded_spsc_retained_payload::<WorkerCompletion<J>>(capacity)
        .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?
        .total_bytes()
        .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
    command_bytes
        .checked_add(completion_bytes)
        .and_then(|per_worker| per_worker.checked_mul(worker_count))
        .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)
}

// REALTIME_POLICY_BEGIN
impl<J: NativeSchedulerJobV1> NativeSchedulerV1<J> {
    /// Open a block: let the workers spin, and reap any parcel a dead worker has finally returned.
    ///
    /// Reaped assignments are written into `reaped` as `(wave index, partition id)` and the count
    /// is returned; the caller un-mutes the edges sourced from those partitions.
    pub fn begin_block(
        &mut self,
        lease: Option<&mut WorkerLeaseV1<J>>,
        waves: &mut [RenderWaveV1<J>],
        reaped: &mut [Option<(usize, usize)>],
    ) -> usize {
        let Some(lease) = lease else {
            return 0;
        };
        lease.shared.block_open.store(true, Ordering::Release);
        let generation = self.generation;
        let mut count = 0_usize;
        for endpoint in lease.endpoints_mut() {
            if !endpoint.dead || endpoint.outstanding.is_none() {
                continue;
            }
            let Ok(completion) = endpoint.completions.try_pop() else {
                continue;
            };
            let (wave_id, partition_id, issued_generation) =
                endpoint.outstanding.take().unwrap_or((0, 0, generation));
            endpoint.audit = completion.audit;
            if completion.generation != issued_generation
                || completion.wave_id != wave_id
                || completion.partition_id != partition_id
                || issued_generation != generation
            {
                // A parcel from a plan this lease no longer serves, or a mismatched token: park
                // it on the endpoint. It is dropped when the lease is released, off render.
                debug_assert!(endpoint.stale.is_none(), "a dead worker holds one parcel");
                if endpoint.stale.is_none() {
                    endpoint.stale = Some(completion.parcel);
                }
                continue;
            }
            let mut parcel = Some(completion.parcel);
            let mut restored = None;
            for (index, wave) in waves.iter_mut().enumerate() {
                if wave.layout().level_id != wave_id {
                    continue;
                }
                if let Some(partition) = wave.partitions.get_mut(partition_id) {
                    partition.parcel = parcel.take();
                    partition.trapped = false;
                    restored = Some(index);
                }
                break;
            }
            match restored {
                Some(wave_index) => {
                    if let Some(slot) = reaped.get_mut(count) {
                        *slot = Some((wave_index, partition_id));
                        count += 1;
                    }
                }
                None => {
                    if endpoint.stale.is_none() {
                        endpoint.stale = parcel;
                    }
                }
            }
        }
        count
    }

    /// Close a block: the workers may park again.
    pub fn end_block(&mut self, lease: Option<&mut WorkerLeaseV1<J>>) {
        if let Some(lease) = lease {
            lease.shared.block_open.store(false, Ordering::Release);
        }
    }

    /// Issue every auxiliary parcel once, execute lane zero, then recover in partition order.
    ///
    /// # Errors
    /// Returns the first fault in stable partition order. [`SchedulerDispatchErrorV1::WorkerLost`]
    /// takes precedence over a job error: the block is degraded either way and the caller must
    /// apply its mutes before continuing.
    pub fn render_wave(
        &mut self,
        lease: Option<&mut WorkerLeaseV1<J>>,
        wave: &mut RenderWaveV1<J>,
    ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
        let Some(lease) = lease.filter(|_| self.selection == SchedulerSelectionV1::Parallel) else {
            return execute_sequential(wave);
        };
        if wave.partition_count() == 1 {
            return execute_sequential(wave);
        }
        let worker_partitions = wave.partition_count() - 1;
        if worker_partitions > lease.worker_count() {
            return Err(SchedulerDispatchErrorV1::CompletionMismatch {
                worker_id: lease.worker_count(),
            });
        }
        let generation = self.generation;
        let wave_id = wave.layout().level_id;
        let mut report = SchedulerDispatchReportV1::default();
        let mut issued = 0_usize;
        let mut issue_error: Option<SchedulerDispatchErrorV1<J::Error>> = None;
        #[cfg(feature = "fault-injection")]
        let fault = self.fault;
        for partition_index in 1..wave.partition_count() {
            let worker_id = partition_index - 1;
            if wave.partitions[partition_index].trapped {
                continue;
            }
            if lease.endpoints_mut()[worker_id].dead {
                continue;
            }
            #[cfg(feature = "fault-injection")]
            if fault.command_queue_is_full_for(worker_id) {
                issue_error = Some(SchedulerDispatchErrorV1::CommandQueueFull { worker_id });
                break;
            }
            let Some(parcel) = wave.partitions[partition_index].parcel.take() else {
                issue_error = Some(SchedulerDispatchErrorV1::MissingParcel {
                    partition_id: partition_index,
                });
                break;
            };
            let command = WorkerMessage::Run(WorkerCommand {
                generation,
                wave_id,
                partition_id: partition_index,
                parcel,
                #[cfg(feature = "fault-injection")]
                fault,
            });
            if let Err(full) = lease.endpoints_mut()[worker_id].commands.try_push(command) {
                let WorkerMessage::Run(command) = full.value;
                wave.partitions[partition_index].parcel = Some(command.parcel);
                issue_error = Some(SchedulerDispatchErrorV1::CommandQueueFull { worker_id });
                break;
            }
            lease.endpoints_mut()[worker_id].outstanding =
                Some((wave_id, partition_index, generation));
            issued += 1;
            report.worker_commands = report.worker_commands.saturating_add(1);
        }
        // Every issued parcel must be woken before it can be recovered, on the error path too.
        if issued > 0 {
            wake_root(lease, &mut report);
        }
        if let Some(error) = issue_error {
            let recovered = recover_issued(
                lease,
                wave,
                issued,
                generation,
                wave_id,
                self.budget.recovery_iterations,
                &mut report,
            );
            if let Some((worker_id, partition_id)) = recovered.lost {
                return Err(SchedulerDispatchErrorV1::WorkerLost {
                    worker_id,
                    partition_id,
                });
            }
            if let Some(worker_id) = recovered.mismatch {
                return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
            }
            return Err(error);
        }
        // Lane zero, then the partitions of any dead worker, inline and in partition order.
        let mut coordinator_result = Ok(());
        let mut coordinator_failure: Option<usize> = None;
        for partition_index in 0..wave.partition_count() {
            let is_coordinator = partition_index == 0;
            if !is_coordinator {
                if wave.partitions[partition_index].trapped {
                    continue;
                }
                if !lease.endpoints_mut()[partition_index - 1].dead {
                    continue;
                }
            }
            let Some(mut parcel) = wave.partitions[partition_index].parcel.take() else {
                let recovered = recover_issued(
                    lease,
                    wave,
                    issued,
                    generation,
                    wave_id,
                    self.budget.recovery_iterations,
                    &mut report,
                );
                if let Some(worker_id) = recovered.mismatch {
                    return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                }
                return Err(SchedulerDispatchErrorV1::MissingParcel {
                    partition_id: partition_index,
                });
            };
            let result = parcel.execute();
            wave.partitions[partition_index].parcel = Some(parcel);
            report.coordinator_jobs = report.coordinator_jobs.saturating_add(1);
            if !is_coordinator {
                report.dead_partitions_executed = report.dead_partitions_executed.saturating_add(1);
            }
            if result.is_err() && coordinator_failure.is_none() {
                coordinator_failure = Some(partition_index);
                coordinator_result = result;
            }
        }
        let recovered = recover_issued(
            lease,
            wave,
            issued,
            generation,
            wave_id,
            self.budget.recovery_iterations,
            &mut report,
        );
        if let Some((worker_id, partition_id)) = recovered.lost {
            return Err(SchedulerDispatchErrorV1::WorkerLost {
                worker_id,
                partition_id,
            });
        }
        if let Some(worker_id) = recovered.mismatch {
            return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
        }
        if let Some(partition_id) = recovered.panicked {
            return Err(SchedulerDispatchErrorV1::JobPanicked { partition_id });
        }
        if let (Some(partition_id), Err(error)) = (coordinator_failure, coordinator_result) {
            let earlier = recovered.first_error.as_ref().is_none_or(
                |current: &SchedulerJobFailureV1<J::Error>| partition_id < current.partition_id,
            );
            if earlier {
                return Err(SchedulerDispatchErrorV1::Job(SchedulerJobFailureV1 {
                    partition_id,
                    error,
                }));
            }
        }
        if let Some(error) = recovered.first_error {
            return Err(SchedulerDispatchErrorV1::Job(error));
        }
        Ok(report)
    }

    /// Copy cumulative per-worker realtime audit snapshots in stable worker order.
    ///
    /// Callers read this only after rendering is disarmed. The returned count is bounded by the
    /// supplied slice and never allocates.
    pub fn copy_worker_audit_snapshots(
        &self,
        lease: Option<&WorkerLeaseV1<J>>,
        output: &mut [audit::AuditSnapshot],
    ) -> usize {
        let Some(endpoints) = lease.and_then(|lease| lease.endpoints.as_deref()) else {
            return 0;
        };
        let count = output.len().min(endpoints.len());
        for (target, endpoint) in output[..count].iter_mut().zip(endpoints) {
            *target = endpoint.audit;
        }
        count
    }
}

/// The single permitted coordinator syscall: at most one `unpark` per rendered block.
///
/// Worker 0 always has a command when any worker does (partitions fill workers in order), and
/// every worker's tree path passes only through lower ids, so this one wake reaches the whole
/// issued set through `PoolShared::wake_children`.
fn wake_root<J: NativeSchedulerJobV1>(
    lease: &mut WorkerLeaseV1<J>,
    report: &mut SchedulerDispatchReportV1,
) {
    let Some(root) = lease.shared.workers.first() else {
        return;
    };
    fence(Ordering::SeqCst);
    if root.parked.load(Ordering::SeqCst) {
        root.thread.unpark();
        report.coordinator_wakes = report.coordinator_wakes.saturating_add(1);
    }
}

struct Recovery<E> {
    mismatch: Option<usize>,
    first_error: Option<SchedulerJobFailureV1<E>>,
    panicked: Option<usize>,
    lost: Option<(usize, usize)>,
}

/// Recover every issued parcel, each within its own bounded budget.
///
/// A worker that does not answer inside `budget` spin iterations is declared dead for the life of
/// the lease and its partition is left trapped. Recovery continues with the remaining workers, so
/// one late worker cannot make the block wait `n` budgets in sequence for the others.
fn recover_issued<J: NativeSchedulerJobV1>(
    lease: &mut WorkerLeaseV1<J>,
    wave: &mut RenderWaveV1<J>,
    issued: usize,
    generation: u64,
    wave_id: u64,
    budget: u64,
    report: &mut SchedulerDispatchReportV1,
) -> Recovery<J::Error> {
    let mut recovery = Recovery {
        mismatch: None,
        first_error: None,
        panicked: None,
        lost: None,
    };
    let mut remaining = issued;
    let mut worker_id = 0_usize;
    while remaining > 0 {
        let endpoint = match lease.endpoints_mut().get_mut(worker_id) {
            Some(endpoint) if endpoint.outstanding.is_some() && !endpoint.dead => endpoint,
            Some(_) => {
                worker_id += 1;
                continue;
            }
            None => break,
        };
        let mut spins = 0_u64;
        let completion = loop {
            match endpoint.completions.try_pop() {
                Ok(completion) => break Some(completion),
                Err(_) => {
                    if spins >= budget {
                        break None;
                    }
                    spins += 1;
                    core::hint::spin_loop();
                }
            }
        };
        remaining -= 1;
        let expected_partition = worker_id + 1;
        let Some(completion) = completion else {
            endpoint.dead = true;
            if let Some(partition) = wave.partitions.get_mut(expected_partition) {
                partition.trapped = true;
            }
            if recovery.lost.is_none() {
                recovery.lost = Some((worker_id, expected_partition));
            }
            worker_id += 1;
            continue;
        };
        endpoint.outstanding = None;
        endpoint.audit = completion.audit;
        report.worker_completions = report.worker_completions.saturating_add(1);
        if completion.generation != generation
            || completion.wave_id != wave_id
            || completion.partition_id != expected_partition
            || wave.partitions[expected_partition].parcel.is_some()
        {
            recovery.mismatch.get_or_insert(worker_id);
        }
        if wave.partitions[expected_partition].parcel.is_none() {
            wave.partitions[expected_partition].parcel = Some(completion.parcel);
        }
        match completion.outcome {
            WorkerOutcome::Panicked => {
                if recovery
                    .panicked
                    .is_none_or(|current| expected_partition < current)
                {
                    recovery.panicked = Some(expected_partition);
                }
            }
            WorkerOutcome::Completed(Err(error)) => {
                let replaces = recovery.first_error.as_ref().is_none_or(
                    |current: &SchedulerJobFailureV1<J::Error>| {
                        expected_partition < current.partition_id
                    },
                );
                if replaces {
                    recovery.first_error = Some(SchedulerJobFailureV1 {
                        partition_id: expected_partition,
                        error,
                    });
                }
            }
            WorkerOutcome::Completed(Ok(())) => {}
        }
        worker_id += 1;
    }
    recovery
}
// REALTIME_POLICY_END

/// The auxiliary worker: spin while a block is open, park otherwise, execute exactly one parcel
/// per command, and publish it back with its audit snapshot.
///
/// `ORDER 1-3` is a Dekker pair with `wake_root`/`PoolShared::wake_children`: the worker commits
/// to parking, fences, and only then makes its last check of the command queue. Removing either
/// fence, or weakening one to Acquire/Release, is a lost-wake bug.
fn worker_loop<J: NativeSchedulerJobV1>(
    id: usize,
    shared: Arc<PoolShared>,
    mut commands: Consumer<WorkerMessage<J>>,
    mut completions: Producer<WorkerCompletion<J>>,
) {
    let mut spins = 0_u64;
    loop {
        let message = loop {
            if let Ok(message) = commands.try_pop() {
                break message;
            }
            if shared.stop.load(Ordering::Acquire) {
                return;
            }
            if shared.block_open.load(Ordering::Acquire)
                && spins < shared.idle_spin.load(Ordering::Relaxed)
            {
                spins += 1;
                core::hint::spin_loop();
                continue;
            }
            shared.workers[id].parked.store(true, Ordering::SeqCst); // ORDER 1
            fence(Ordering::SeqCst); // ORDER 2
            let late = commands.try_pop(); // ORDER 3
            if let Ok(message) = late {
                shared.workers[id].parked.store(false, Ordering::SeqCst);
                break message;
            }
            if shared.stop.load(Ordering::SeqCst) {
                shared.workers[id].parked.store(false, Ordering::SeqCst);
                return;
            }
            thread::park();
            shared.workers[id].parked.store(false, Ordering::SeqCst);
            spins = 0;
        };
        let WorkerMessage::Run(mut command) = message;
        shared.wake_children(id);
        #[cfg(feature = "fault-injection")]
        if let FaultInjectionV1::StallWorker {
            worker_id,
            wave_id,
            iterations,
        } = command.fault
            && worker_id == id
            && wave_id == command.wave_id
        {
            let mut spin = 0_u64;
            while spin < iterations {
                core::hint::spin_loop();
                spin += 1;
            }
        }
        #[cfg(feature = "fault-injection")]
        let panic_here = matches!(
            command.fault,
            FaultInjectionV1::PanicWorker { worker_id, wave_id }
                if worker_id == id && wave_id == command.wave_id
        );
        #[cfg(not(feature = "fault-injection"))]
        let panic_here = false;
        let outcome = audit::in_render_scope(|| {
            catch_unwind(AssertUnwindSafe(|| {
                assert!(!panic_here, "injected worker panic");
                command.parcel.execute()
            }))
        });
        let outcome = match outcome {
            Ok(result) => WorkerOutcome::Completed(result),
            Err(_) => WorkerOutcome::Panicked,
        };
        #[cfg(feature = "fault-injection")]
        let (generation, partition_id) =
            command
                .fault
                .completion_tokens(id, command.generation, command.partition_id);
        #[cfg(not(feature = "fault-injection"))]
        let (generation, partition_id) = (command.generation, command.partition_id);
        let mut pending = Some(WorkerCompletion {
            generation,
            wave_id: command.wave_id,
            partition_id,
            outcome,
            audit: audit::snapshot(),
            parcel: command.parcel,
        });
        while let Some(value) = pending.take() {
            pending = match completions.try_push(value) {
                Ok(()) => None,
                Err(full) => Some(full.value),
            };
            if pending.is_some() {
                if shared.stop.load(Ordering::Acquire) {
                    return;
                }
                core::hint::spin_loop();
            }
        }
        spins = 0;
    }
}
