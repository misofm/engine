//! Deterministic prestarted native dependency-wave scheduling.
//!
//! This crate owns only move-only parcels and dedicated worker lifecycle.  It deliberately knows
//! nothing about graph topology or DSP: a graph binder lowers immutable dependency levels into
//! [`RenderWaveV1`] parcels before publication.  Browser/Wasm preparation is explicitly the same
//! parcel representation driven sequentially, so no browser worker or shared-memory claim leaks
//! into the artifact.
//!
//! # Ownership (issue 100)
//!
//! Threads belong to a [`NativeWorkerPoolV1`], which is control-plane state and **plan
//! independent**: a structural change publishes a new plan without spawning or joining a single
//! thread. The coordinator endpoints of that pool travel with the active plan as a
//! [`WorkerLeaseV1`], handed from the retiring executor to its replacement at the block-boundary
//! swap. A [`NativeSchedulerV1`] owns no thread at all; it borrows the lease for one block.
//!
//! # Idle policy and the single wake
//!
//! Between blocks the workers park. The coordinator issues **at most one**
//! [`std::thread::unpark`] per rendered block (`wake_root`), and workers wake their binary-tree
//! children, so one wake reaches every issued worker. That wake is the only syscall any
//! render-reachable coordinator code performs, and it is the documented exception to the
//! render prohibitions in `docs/REALTIME_DEPENDENCY_POLICY.md`. Inside a block the workers spin
//! for a calibrated budget so no further wake is needed.
//!
//! # Bounded recovery
//!
//! A worker that does not return its parcel within [`RecoveryBudgetV1::recovery_iterations`] is
//! declared dead for the life of the lease. Its parcel stays *trapped* (never touched again until
//! the worker returns it at a later block boundary), the remaining units of the block render on
//! the coordinator, and the audio callback has a bounded worst case. Recovery is never a wedge
//! and never a drop on the render thread.

use core::num::NonZeroUsize;

/// Explicit control-plane worker-pool configuration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NativeWorkerPoolConfigV1 {
    /// Auxiliary worker count. `None` asks for `available_parallelism() - 1`.
    pub requested_workers: Option<NonZeroUsize>,
    /// Bounded startup fault, compiled out of production builds.
    #[cfg(feature = "fault-injection")]
    pub fault: FaultInjectionV1,
}

/// Address-free description of a started worker pool.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NativeWorkerPoolShapeV1 {
    /// Exact number of prestarted auxiliary workers.
    pub worker_count: usize,
    /// Measured cost of one idle spin iteration, in nanoseconds, rounded up, never zero.
    ///
    /// Callers turn a wall-clock budget into an iteration count with it; the render path never
    /// reads a clock.
    pub spin_ns: u32,
}

/// Bounded budgets a plan gives its scheduler, derived from the render quantum at bind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryBudgetV1 {
    /// Completion-spin iterations before a worker is declared dead (about half a quantum).
    pub recovery_iterations: u64,
    /// Idle-spin iterations a worker burns inside an open block before parking (about a quantum).
    pub idle_spin_iterations: u64,
}

impl Default for RecoveryBudgetV1 {
    fn default() -> Self {
        Self {
            recovery_iterations: 1 << 14,
            idle_spin_iterations: 1 << 14,
        }
    }
}

/// Explicit control-plane scheduler configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSchedulerConfigV1 {
    /// Render lanes including lane zero, the callback coordinator.
    pub render_lanes: NonZeroUsize,
    /// Host policy permission to arm native auxiliary workers.
    pub enabled: bool,
    /// Shape of the pool whose lease this plan expects to hold.
    pub pool: NativeWorkerPoolShapeV1,
    /// Bounded test-only fault at the real scheduler protocol boundary.
    #[cfg(feature = "fault-injection")]
    pub fault: FaultInjectionV1,
}

/// Test-only scheduler fault injection.
///
/// Every variant preserves the real move-only command/completion parcel transport. The feature is
/// enabled only from `[dev-dependencies]`, which `scripts/check-scheduler-policy.sh` enforces, so
/// it is absent from every production, host and C-ABI build.
#[cfg(feature = "fault-injection")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FaultInjectionV1 {
    /// No injected fault.
    #[default]
    None,
    /// Withhold one worker-ready acknowledgement so the pool cannot start.
    StartupHandshakeFailure,
    /// Report one command publication as capacity-full before it moves its parcel.
    CommandQueueFull {
        /// Zero-based auxiliary worker selected at the real command publication boundary.
        worker_id: usize,
    },
    /// Return one real completion with a stale generation token while preserving its parcel.
    StaleGeneration {
        /// Zero-based auxiliary worker selected at completion publication.
        worker_id: usize,
    },
    /// Return one real completion with another worker's partition token while preserving its
    /// unique parcel, so coordinator validation observes a duplicate completion identity.
    DuplicateCompletion {
        /// Zero-based auxiliary worker selected at completion publication.
        worker_id: usize,
    },
    /// Spin one worker before it executes, so it misses its recovery deadline.
    StallWorker {
        /// Zero-based auxiliary worker to stall.
        worker_id: usize,
        /// Dependency level on which to stall.
        wave_id: u64,
        /// Spin iterations to burn before executing.
        iterations: u64,
    },
    /// Panic inside one worker's parcel execution scope.
    PanicWorker {
        /// Zero-based auxiliary worker to panic.
        worker_id: usize,
        /// Dependency level on which to panic.
        wave_id: u64,
    },
}

#[cfg(feature = "fault-injection")]
impl FaultInjectionV1 {
    const fn command_queue_is_full_for(self, worker_id: usize) -> bool {
        matches!(self, Self::CommandQueueFull { worker_id: target } if target == worker_id)
    }

    const fn completion_tokens(
        self,
        worker_id: usize,
        generation: u64,
        partition_id: usize,
    ) -> (u64, usize) {
        match self {
            Self::StaleGeneration { worker_id: target } if target == worker_id => {
                (generation.wrapping_add(1), partition_id)
            }
            Self::DuplicateCompletion { worker_id: target }
                if target == worker_id && partition_id > 1 =>
            {
                (generation, partition_id - 1)
            }
            _ => (generation, partition_id),
        }
    }
}

impl NativeSchedulerConfigV1 {
    /// Construct an ordinary production scheduler configuration.
    #[must_use]
    pub const fn new(
        render_lanes: NonZeroUsize,
        enabled: bool,
        pool: NativeWorkerPoolShapeV1,
    ) -> Self {
        Self {
            render_lanes,
            enabled,
            pool,
            #[cfg(feature = "fault-injection")]
            fault: FaultInjectionV1::None,
        }
    }

    /// Configure one bounded test-only fault at the real scheduler protocol boundary.
    #[cfg(feature = "fault-injection")]
    #[must_use]
    pub const fn with_fault(mut self, fault: FaultInjectionV1) -> Self {
        self.fault = fault;
        self
    }
}

/// Immutable reason a prepared scheduler uses the sequential parcel driver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FallbackReasonV1 {
    /// The session explicitly requested the compatibility single-thread render mode.
    SingleThread,
    /// The host explicitly disabled worker execution.
    DisabledByHost,
    /// One configured lane leaves no auxiliary worker.
    OneLane,
    /// No accepted dependency wave has two independent parcels.
    InsufficientWaveWidth,
    /// The host started no auxiliary workers.
    NoWorkers,
    /// Browser/Wasm deliberately has no native scheduler workers.
    UnsupportedTarget,
}

/// Frozen scheduler selection stored in the prepared plan metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerSelectionV1 {
    /// Dedicated auxiliary workers are armed before plan publication.
    Parallel,
    /// The exact prepared parcel representation is rendered by lane zero.
    Sequential(FallbackReasonV1),
}

/// Address-free prepared scheduler resource report.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSchedulerResourceReportV1 {
    /// Selected coordinator plus auxiliary lane count.
    pub selected_lanes: usize,
    /// Exact number of prestarted auxiliary workers.
    pub worker_count: usize,
    /// Number of immutable prepared waves.
    pub wave_count: usize,
    /// Number of stable prepared job units.
    pub unit_count: usize,
    /// Number of stable prepared partitions.
    pub partition_count: usize,
    /// Exact scheduler queue payload bytes, excluding allocator headers/page rounding.
    pub retained_queue_bytes: usize,
}

/// A checked contiguous stable-unit range in one render wave.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderPartitionRangeV1 {
    /// Stable partition identifier, starting at zero in every wave.
    pub partition_id: usize,
    /// Inclusive stable-unit index.
    pub first_unit: usize,
    /// Exclusive stable-unit index.
    pub end_unit: usize,
}

/// Immutable partitioning of a dependency level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderWaveLayoutV1 {
    /// Compiler-issued immutable dependency-level identifier.
    pub level_id: u64,
    /// Stable unit ranges in coordinator/worker partition order.
    pub partitions: Box<[RenderPartitionRangeV1]>,
}

/// A move-only prepared parcel for exactly one partition.
pub struct RenderPartitionV1<J> {
    /// Stable partition range selected at preparation.
    pub range: RenderPartitionRangeV1,
    parcel: Option<J>,
    trapped: bool,
}

impl<J> RenderPartitionV1<J> {
    /// Prepare one owned parcel in a checked stable partition range.
    #[must_use]
    pub fn new(range: RenderPartitionRangeV1, parcel: J) -> Self {
        Self {
            range,
            parcel: Some(parcel),
            trapped: false,
        }
    }

    /// Whether the coordinator owns this parcel at a wave boundary.
    #[must_use]
    pub const fn is_recovered(&self) -> bool {
        self.parcel.is_some()
    }

    /// Whether a worker that missed its deadline still owns this parcel.
    ///
    /// A trapped partition is never executed and its outputs are never read; the executor mutes
    /// every edge sourced from it until the parcel is reaped at a later block boundary.
    #[must_use]
    pub const fn is_trapped(&self) -> bool {
        self.trapped
    }
}

/// One immutable dependency level with move-only execution parcels.
pub struct RenderWaveV1<J> {
    layout: RenderWaveLayoutV1,
    partitions: Box<[RenderPartitionV1<J>]>,
}

impl<J> RenderWaveV1<J> {
    /// Construct a wave after graph preparation has formed each disjoint parcel.
    ///
    /// # Errors
    /// Rejects an empty wave or a non-canonical partition cover.
    pub fn new(
        level_id: u64,
        partitions: Box<[RenderPartitionV1<J>]>,
    ) -> Result<Self, SchedulerPrepareErrorV1> {
        if partitions.is_empty() {
            return Err(SchedulerPrepareErrorV1::EmptyWave);
        }
        let mut expected_unit = 0_usize;
        for (index, partition) in partitions.iter().enumerate() {
            if partition.range.partition_id != index
                || partition.range.first_unit != expected_unit
                || partition.range.end_unit <= partition.range.first_unit
            {
                return Err(SchedulerPrepareErrorV1::InvalidPartition);
            }
            expected_unit = partition.range.end_unit;
        }
        Ok(Self {
            layout: RenderWaveLayoutV1 {
                level_id,
                partitions: partitions.iter().map(|value| value.range).collect(),
            },
            partitions,
        })
    }

    /// Immutable address-free partition transcript.
    #[must_use]
    pub const fn layout(&self) -> &RenderWaveLayoutV1 {
        &self.layout
    }

    /// Count stable job units without inspecting any parcel address.
    #[must_use]
    pub fn unit_count(&self) -> usize {
        self.layout
            .partitions
            .last()
            .map_or(0, |partition| partition.end_unit)
    }

    /// Count partitions selected before publication.
    #[must_use]
    pub const fn partition_count(&self) -> usize {
        self.partitions.len()
    }

    /// Check that every parcel was returned at the wave boundary.
    #[must_use]
    pub fn all_recovered(&self) -> bool {
        self.partitions.iter().all(RenderPartitionV1::is_recovered)
    }

    /// Whether a worker that missed its deadline still owns this partition's parcel.
    #[must_use]
    pub fn is_trapped(&self, partition_id: usize) -> bool {
        self.partitions
            .get(partition_id)
            .is_some_and(RenderPartitionV1::is_trapped)
    }

    /// Borrow one recovered parcel by its stable partition identifier.
    #[must_use]
    pub fn recovered_parcel(&self, partition_id: usize) -> Option<&J> {
        self.partitions.get(partition_id)?.parcel.as_ref()
    }

    /// Mutably borrow one recovered parcel by its stable partition identifier.
    #[must_use]
    pub fn recovered_parcel_mut(&mut self, partition_id: usize) -> Option<&mut J> {
        self.partitions.get_mut(partition_id)?.parcel.as_mut()
    }

    /// Visit every coordinator-owned parcel in stable partition order.
    pub fn recovered_parcels(&self) -> impl Iterator<Item = &J> {
        self.partitions
            .iter()
            .filter_map(|partition| partition.parcel.as_ref())
    }

    /// Visit every coordinator-owned parcel mutably in stable partition order.
    pub fn recovered_parcels_mut(&mut self) -> impl Iterator<Item = &mut J> {
        self.partitions
            .iter_mut()
            .filter_map(|partition| partition.parcel.as_mut())
    }
}

/// A cost-weighted partitioning of one dependency level.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightedPartitionV1 {
    /// Original unit indices in the order the partitions cover them.
    pub unit_order: Box<[usize]>,
    /// Contiguous partition ranges over `unit_order`.
    pub ranges: Box<[RenderPartitionRangeV1]>,
}

/// Split one dependency level into `render_lanes` cost-balanced contiguous partitions.
///
/// Splitting by unit *count* makes one eight-track bank unit weigh the same as a scalar tail, so
/// the coordinator waits on the heaviest lane every wave (issue 100 F2). This is longest
/// processing time first: units sorted by `(weight desc, index asc)` are assigned one at a time to
/// the least-loaded bin, ties to the lowest bin id, and `pinned_to_zero` -- the session output's
/// unit, which must stay coordinator-owned -- is placed in bin 0 first. Bins are then laid out
/// contiguously in bin order, each bin's units in ascending original index, so
/// [`RenderWaveV1::new`]'s canonical-cover check still holds and the lowering can reorder its
/// units to match.
///
/// The result is a pure function of `weights`, `render_lanes` and `pinned_to_zero`: it is frozen
/// at bind and never re-derived on the render path.
#[must_use]
pub fn partition_weighted_units_v1(
    weights: &[u64],
    render_lanes: NonZeroUsize,
    pinned_to_zero: Option<usize>,
) -> WeightedPartitionV1 {
    let count = weights.len();
    debug_assert!(count > 0, "a wave has at least one unit");
    let bins = count.min(render_lanes.get()).max(1);
    let mut load = vec![0_u64; bins];
    let mut assignment = vec![0_usize; count];
    let mut assigned = vec![false; count];
    if let Some(pinned) = pinned_to_zero.filter(|index| *index < count) {
        assignment[pinned] = 0;
        assigned[pinned] = true;
        load[0] = load[0].saturating_add(weights[pinned].max(1));
    }
    let mut order: Vec<usize> = (0..count).filter(|index| !assigned[*index]).collect();
    order.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    // Every bin must receive at least one unit: the cover is contiguous and no partition may be
    // empty. Seed the still-empty bins in descending weight order before balancing the rest.
    let mut next_seed = if pinned_to_zero.is_some_and(|index| index < count) {
        1
    } else {
        0
    };
    for unit in order {
        let bin = if next_seed < bins {
            let bin = next_seed;
            next_seed += 1;
            bin
        } else {
            let mut best = 0_usize;
            for candidate in 1..bins {
                if load[candidate] < load[best] {
                    best = candidate;
                }
            }
            best
        };
        assignment[unit] = bin;
        load[bin] = load[bin].saturating_add(weights[unit].max(1));
    }
    let mut unit_order = Vec::with_capacity(count);
    let mut ranges = Vec::with_capacity(bins);
    for bin in 0..bins {
        let first_unit = unit_order.len();
        for (unit, target) in assignment.iter().enumerate() {
            if *target == bin {
                unit_order.push(unit);
            }
        }
        ranges.push(RenderPartitionRangeV1 {
            partition_id: bin,
            first_unit,
            end_unit: unit_order.len(),
        });
    }
    WeightedPartitionV1 {
        unit_order: unit_order.into_boxed_slice(),
        ranges: ranges.into_boxed_slice(),
    }
}

/// Preparation rejected an invalid scheduler shape before workers were armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerPrepareErrorV1 {
    /// A wave cannot be empty.
    EmptyWave,
    /// Partition IDs or unit coverage are not canonical contiguous ranges.
    InvalidPartition,
    /// A native worker could not be started and no prepared pool was returned.
    WorkerStart,
    /// Exact retained-byte accounting overflowed `usize`.
    ResourceOverflow,
}

/// A move-only parcel operation; implementations must be bounded and panic-free.
pub trait NativeSchedulerJobV1: Send + 'static {
    /// Bounded deterministic job failure selected after all issued parcels return.
    type Error: Send + 'static;
    /// Execute this parcel once while it is owned by one lane.
    fn execute(&mut self) -> Result<(), Self::Error>;
}

/// A stable worker-completion/job error selected only after recovery.
#[derive(Debug)]
pub struct SchedulerJobFailureV1<E> {
    /// Stable partition identity, never completion-arrival order.
    pub partition_id: usize,
    /// The bounded parcel error.
    pub error: E,
}

/// A dispatch fault; callers retain the wave and can inspect recovered parcel ownership.
#[derive(Debug)]
pub enum SchedulerDispatchErrorV1<E> {
    /// A coordinator boundary was missing its owned parcel.
    MissingParcel {
        /// Stable partition with no coordinator-owned parcel at its boundary.
        partition_id: usize,
    },
    /// Command publication failed before a parcel could be transferred.
    CommandQueueFull {
        /// Sole worker whose capacity-one command queue was unexpectedly occupied.
        worker_id: usize,
    },
    /// A completion did not match the issued wave, generation, or worker partition.
    CompletionMismatch {
        /// Sole worker that returned an invalid completion token.
        worker_id: usize,
    },
    /// One or more jobs failed; the first is selected by stable partition order.
    Job(SchedulerJobFailureV1<E>),
    /// A parcel unwound. Only reachable under an unwinding profile (D12 aborts in release).
    JobPanicked {
        /// Stable partition whose parcel unwound.
        partition_id: usize,
    },
    /// A worker missed its bounded recovery deadline and is dead for the life of the lease.
    ///
    /// Its parcel is trapped: the caller must mute every edge sourced from `partition_id` and
    /// finish the block without it. The parcel returns at a later [`NativeSchedulerV1::begin_block`].
    WorkerLost {
        /// Sole worker declared dead.
        worker_id: usize,
        /// Stable partition whose parcel is trapped with that worker.
        partition_id: usize,
    },
}

/// Fixed per-wave dispatch counters, readable after the scheduler is disarmed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SchedulerDispatchReportV1 {
    /// Coordinator-owned parcel executions.
    pub coordinator_jobs: u64,
    /// Exactly-once auxiliary command publications.
    pub worker_commands: u64,
    /// Exactly-once auxiliary parcel recoveries.
    pub worker_completions: u64,
    /// `Thread::unpark` calls the coordinator issued (at most one per block).
    pub coordinator_wakes: u64,
    /// Partitions of a dead worker executed inline by the coordinator.
    pub dead_partitions_executed: u64,
}

// REALTIME_POLICY_BEGIN
/// Execute every untrapped partition of a wave on the calling lane, in stable partition order.
fn execute_sequential<J: NativeSchedulerJobV1>(
    wave: &mut RenderWaveV1<J>,
) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
    let mut report = SchedulerDispatchReportV1::default();
    for (partition_id, partition) in wave.partitions.iter_mut().enumerate() {
        if partition.trapped {
            continue;
        }
        let Some(mut parcel) = partition.parcel.take() else {
            return Err(SchedulerDispatchErrorV1::MissingParcel { partition_id });
        };
        let result = parcel.execute();
        partition.parcel = Some(parcel);
        report.coordinator_jobs = report.coordinator_jobs.saturating_add(1);
        if let Err(error) = result {
            return Err(SchedulerDispatchErrorV1::Job(SchedulerJobFailureV1 {
                partition_id,
                error,
            }));
        }
    }
    Ok(report)
}
// REALTIME_POLICY_END

/// A prepared generic native scheduler.  It contains no graph or DSP type knowledge, and -- since
/// issue 100 -- no thread: it borrows a [`WorkerLeaseV1`] for the duration of one block.
pub struct NativeSchedulerV1<J: NativeSchedulerJobV1> {
    selection: SchedulerSelectionV1,
    generation: u64,
    budget: RecoveryBudgetV1,
    worker_count: usize,
    retained_queue_bytes: usize,
    #[cfg(feature = "fault-injection")]
    fault: FaultInjectionV1,
    _job: core::marker::PhantomData<fn() -> J>,
}

impl<J: NativeSchedulerJobV1> NativeSchedulerV1<J> {
    /// Freeze a driver selection before the enclosing plan is published.
    ///
    /// # Errors
    /// Rejects a configuration whose exact retained-byte accounting overflows.
    pub fn prepare(
        config: NativeSchedulerConfigV1,
        largest_wave_width: usize,
        generation: u64,
        budget: RecoveryBudgetV1,
    ) -> Result<Self, SchedulerPrepareErrorV1> {
        Self::prepare_with_fallback(config, largest_wave_width, generation, budget, None)
    }

    /// Prepare with an explicit control-plane sequential selection.
    ///
    /// Graph binding uses this for a session whose frozen render mode is single-threaded.  Host
    /// and target fallbacks continue to be selected by [`Self::prepare`].
    ///
    /// # Errors
    /// Rejects a configuration whose exact retained-byte accounting overflows.
    pub fn prepare_with_fallback(
        config: NativeSchedulerConfigV1,
        largest_wave_width: usize,
        generation: u64,
        budget: RecoveryBudgetV1,
        fallback: Option<FallbackReasonV1>,
    ) -> Result<Self, SchedulerPrepareErrorV1> {
        let selection = fallback.map_or_else(
            || select_scheduler(config, largest_wave_width),
            SchedulerSelectionV1::Sequential,
        );
        let worker_count = if selection == SchedulerSelectionV1::Parallel {
            config
                .pool
                .worker_count
                .min(config.render_lanes.get().saturating_sub(1))
        } else {
            0
        };
        let retained_queue_bytes = platform::retained_queue_bytes::<J>(worker_count)?;
        Ok(Self {
            selection,
            generation,
            budget,
            worker_count,
            retained_queue_bytes,
            #[cfg(feature = "fault-injection")]
            fault: config.fault,
            _job: core::marker::PhantomData,
        })
    }

    /// Frozen driver selection.
    #[must_use]
    pub const fn selection(&self) -> SchedulerSelectionV1 {
        self.selection
    }

    /// Auxiliary workers this plan expects its lease to carry.
    #[must_use]
    pub const fn expected_workers(&self) -> usize {
        self.worker_count
    }

    /// Bounded recovery and idle budgets frozen at bind.
    #[must_use]
    pub const fn budget(&self) -> RecoveryBudgetV1 {
        self.budget
    }

    /// Report the exact queue payload retained by this scheduler plus graph-supplied wave counts.
    ///
    /// `wave_count`, `unit_count`, and `partition_count` come from the immutable graph lowering;
    /// this generic boundary can account only for the queues that its lease owns.
    #[must_use]
    pub const fn resource_report(
        &self,
        wave_count: usize,
        unit_count: usize,
        partition_count: usize,
    ) -> NativeSchedulerResourceReportV1 {
        NativeSchedulerResourceReportV1 {
            selected_lanes: self.worker_count + 1,
            worker_count: self.worker_count,
            wave_count,
            unit_count,
            partition_count,
            retained_queue_bytes: self.retained_queue_bytes,
        }
    }
}

fn select_scheduler(
    config: NativeSchedulerConfigV1,
    largest_wave_width: usize,
) -> SchedulerSelectionV1 {
    if cfg!(target_arch = "wasm32") {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::UnsupportedTarget)
    } else if !config.enabled {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::DisabledByHost)
    } else if config.render_lanes.get() == 1 {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::OneLane)
    } else if config.pool.worker_count == 0 {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::NoWorkers)
    } else if largest_wave_width < 2 {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::InsufficientWaveWidth)
    } else {
        SchedulerSelectionV1::Parallel
    }
}

mod platform;

pub use platform::{NativeWorkerPoolV1, WorkerLeaseV1};

#[cfg(test)]
mod tests;
