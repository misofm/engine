//! Deterministic prestarted native dependency-wave scheduling.
//!
//! This crate owns only move-only parcels and dedicated worker lifecycle.  It deliberately knows
//! nothing about graph topology or DSP: a graph binder lowers immutable dependency levels into
//! [`RenderWaveV1`] parcels before publication.  Browser/Wasm preparation is explicitly the same
//! parcel representation driven sequentially, so no browser worker or shared-memory claim leaks
//! into the artifact.

use core::num::NonZeroUsize;

/// Explicit control-plane scheduler configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeSchedulerConfigV1 {
    /// Render lanes including lane zero, the callback coordinator.
    pub render_lanes: NonZeroUsize,
    /// Host policy permission to arm native auxiliary workers.
    pub enabled: bool,
    /// Fixed test-only order in which the coordinator dequeues and accepts real worker
    /// completions. This field is absent from normal production builds.
    #[cfg(feature = "test-support")]
    test_completion_acceptance_order: [u16; 3],
    /// Test-only bounded ownership-protocol fault selected before the prepared scheduler is
    /// published. This field is absent from normal production builds.
    #[cfg(feature = "test-support")]
    test_protocol_injection: SchedulerTestProtocolInjectionV1,
}

/// Test-only scheduler ownership-protocol fault injection.
///
/// Every variant preserves the real move-only command/completion parcel transport. It is compiled
/// out of normal production builds and has no product retry or recovery behavior.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum SchedulerTestProtocolInjectionV1 {
    /// No injected protocol fault.
    None,
    /// Withhold one worker-ready acknowledgement so preparation cannot publish the scheduler.
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
}

#[cfg(feature = "test-support")]
impl SchedulerTestProtocolInjectionV1 {
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
    pub const fn new(render_lanes: NonZeroUsize, enabled: bool) -> Self {
        Self {
            render_lanes,
            enabled,
            #[cfg(feature = "test-support")]
            test_completion_acceptance_order: [0, 1, 2],
            #[cfg(feature = "test-support")]
            test_protocol_injection: SchedulerTestProtocolInjectionV1::None,
        }
    }

    /// Configure a bounded test-only worker-completion acceptance order.
    ///
    /// The test configuration is compiled only with the scheduler's `test-support` feature. It
    /// selects only the order in which real SPSC completion parcels are dequeued and recovered;
    /// it cannot affect worker execution, parcel ownership, arithmetic, or observer order. The
    /// supplied values must be a permutation of the first three zero-based worker IDs.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    #[must_use]
    pub const fn with_test_completion_acceptance_order(mut self, order: [u8; 3]) -> Self {
        self.test_completion_acceptance_order = [order[0] as u16, order[1] as u16, order[2] as u16];
        self
    }

    /// Configure one bounded test-only fault at the real scheduler protocol boundary.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    #[must_use]
    pub const fn with_test_protocol_injection(
        mut self,
        injection: SchedulerTestProtocolInjectionV1,
    ) -> Self {
        self.test_protocol_injection = injection;
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
}

impl<J> RenderPartitionV1<J> {
    /// Prepare one owned parcel in a checked stable partition range.
    #[must_use]
    pub fn new(range: RenderPartitionRangeV1, parcel: J) -> Self {
        Self {
            range,
            parcel: Some(parcel),
        }
    }

    /// Whether the coordinator owns this parcel at a wave boundary.
    #[must_use]
    pub const fn is_recovered(&self) -> bool {
        self.parcel.is_some()
    }
}

/// One immutable dependency level with move-only execution parcels.
pub struct RenderWaveV1<J> {
    layout: RenderWaveLayoutV1,
    partitions: Box<[RenderPartitionV1<J>]>,
}

impl<J> RenderWaveV1<J> {
    /// Construct a wave after graph preparation has formed each disjoint parcel.
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

/// Divide `unit_count` stable units into contiguous near-even partitions without padding.
pub fn partition_stable_units_v1(
    unit_count: NonZeroUsize,
    render_lanes: NonZeroUsize,
) -> Box<[RenderPartitionRangeV1]> {
    let count = unit_count.get();
    let partitions = count.min(render_lanes.get());
    let base = count / partitions;
    let remainder = count % partitions;
    let mut next = 0_usize;
    (0..partitions)
        .map(|partition_id| {
            let width = base + usize::from(partition_id < remainder);
            let range = RenderPartitionRangeV1 {
                partition_id,
                first_unit: next,
                end_unit: next + width,
            };
            next += width;
            range
        })
        .collect()
}

/// Preparation rejected an invalid scheduler shape before workers were armed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SchedulerPrepareErrorV1 {
    /// A wave cannot be empty.
    EmptyWave,
    /// Partition IDs or unit coverage are not canonical contiguous ranges.
    InvalidPartition,
    /// A native worker could not be started and no prepared scheduler was returned.
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
}

/// A prepared generic native scheduler.  It contains no graph or DSP type knowledge.
pub struct NativeSchedulerV1<J: NativeSchedulerJobV1> {
    selection: SchedulerSelectionV1,
    generation: u64,
    platform: platform::PlatformScheduler<J>,
}

impl<J: NativeSchedulerJobV1> NativeSchedulerV1<J> {
    /// Prestart every required auxiliary worker before the enclosing plan is published.
    pub fn prepare(
        config: NativeSchedulerConfigV1,
        largest_wave_width: usize,
        generation: u64,
    ) -> Result<Self, SchedulerPrepareErrorV1> {
        Self::prepare_with_fallback(config, largest_wave_width, generation, None)
    }

    /// Prepare with an explicit control-plane sequential selection.
    ///
    /// Graph binding uses this for a session whose frozen render mode is single-threaded.  Host
    /// and target fallbacks continue to be selected by [`Self::prepare`].
    pub fn prepare_with_fallback(
        config: NativeSchedulerConfigV1,
        largest_wave_width: usize,
        generation: u64,
        fallback: Option<FallbackReasonV1>,
    ) -> Result<Self, SchedulerPrepareErrorV1> {
        let selection = fallback.map_or_else(
            || select_scheduler(config, largest_wave_width),
            SchedulerSelectionV1::Sequential,
        );
        let platform = platform::PlatformScheduler::prepare(selection, config, generation)?;
        Ok(Self {
            selection,
            generation,
            platform,
        })
    }

    /// Frozen driver selection.
    #[must_use]
    pub const fn selection(&self) -> SchedulerSelectionV1 {
        self.selection
    }

    /// Report the exact queue payload retained by this scheduler plus graph-supplied wave counts.
    ///
    /// `wave_count`, `unit_count`, and `partition_count` come from the immutable graph lowering;
    /// this generic boundary can account only for the workers and queues that it owns itself.
    #[must_use]
    pub fn resource_report(
        &self,
        wave_count: usize,
        unit_count: usize,
        partition_count: usize,
    ) -> NativeSchedulerResourceReportV1 {
        NativeSchedulerResourceReportV1 {
            selected_lanes: self.platform.selected_lanes(),
            worker_count: self.platform.worker_count(),
            wave_count,
            unit_count,
            partition_count,
            retained_queue_bytes: self.platform.retained_queue_bytes(),
        }
    }

    /// Execute each auxiliary parcel once, execute lane zero, then recover in partition order.
    // REALTIME_POLICY_BEGIN
    pub fn render_wave(
        &mut self,
        wave: &mut RenderWaveV1<J>,
    ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
        self.platform.render_wave(wave, self.generation)
    }
    // REALTIME_POLICY_END

    /// Stop and join dedicated workers after the enclosing prepared plan is retired off render.
    pub fn stop_and_join(&mut self) {
        self.platform.stop_and_join();
    }

    /// Copy cumulative per-worker realtime audit snapshots in stable worker order.
    ///
    /// Callers read this only after rendering is disarmed. The returned count is bounded by the
    /// supplied slice and never allocates.
    pub fn copy_worker_audit_snapshots(
        &self,
        output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
    ) -> usize {
        self.platform.copy_worker_audit_snapshots(output)
    }

    /// Return the actual partition-completion acceptance order from the last successful wave.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    #[must_use]
    pub fn test_last_completion_acceptance_order(&self) -> ([u8; 3], u8) {
        self.platform.test_last_completion_acceptance_order()
    }
}

impl<J: NativeSchedulerJobV1> Drop for NativeSchedulerV1<J> {
    fn drop(&mut self) {
        self.stop_and_join();
    }
}

fn select_scheduler(
    config: NativeSchedulerConfigV1,
    largest_wave_width: usize,
) -> SchedulerSelectionV1 {
    if !cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::UnsupportedTarget)
    } else if !config.enabled {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::DisabledByHost)
    } else if config.render_lanes.get() == 1 {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::OneLane)
    } else if largest_wave_width < 2 {
        SchedulerSelectionV1::Sequential(FallbackReasonV1::InsufficientWaveWidth)
    } else {
        SchedulerSelectionV1::Parallel
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod platform {
    #[cfg(feature = "test-support")]
    use super::SchedulerTestProtocolInjectionV1;
    use super::{
        NativeSchedulerConfigV1, NativeSchedulerJobV1, RenderWaveV1, SchedulerDispatchErrorV1,
        SchedulerDispatchReportV1, SchedulerJobFailureV1, SchedulerPrepareErrorV1,
        SchedulerSelectionV1,
    };
    use core::num::NonZeroUsize;
    use miso_engine_core::realtime::{Consumer, Producer, QueueGeneration, bounded_spsc_move};
    use std::{
        sync::mpsc,
        thread::{self, JoinHandle},
    };

    struct WorkerCommand<J> {
        generation: u64,
        wave_id: u64,
        partition_id: usize,
        parcel: J,
        #[cfg(feature = "test-support")]
        protocol_injection: SchedulerTestProtocolInjectionV1,
    }

    struct WorkerCompletion<J: NativeSchedulerJobV1> {
        generation: u64,
        wave_id: u64,
        partition_id: usize,
        result: Result<(), J::Error>,
        audit: miso_engine_core::realtime::audit::AuditSnapshot,
        parcel: J,
    }

    enum WorkerMessage<J> {
        Run(WorkerCommand<J>),
        Stop,
    }

    struct Worker<J: NativeSchedulerJobV1> {
        commands: Producer<WorkerMessage<J>>,
        completions: Consumer<WorkerCompletion<J>>,
        handle: Option<JoinHandle<()>>,
        audit: miso_engine_core::realtime::audit::AuditSnapshot,
    }

    /// Target-native worker implementation.  Its threads are created only by `prepare`.
    pub(super) struct PlatformScheduler<J: NativeSchedulerJobV1> {
        workers: Vec<Worker<J>>,
        parallel: bool,
        retained_queue_bytes: usize,
        #[cfg(feature = "test-support")]
        completion_acceptance_order: [u16; 3],
        #[cfg(feature = "test-support")]
        last_completion_acceptance_order: [u8; 3],
        #[cfg(feature = "test-support")]
        last_completion_acceptance_count: u8,
        #[cfg(feature = "test-support")]
        protocol_injection: SchedulerTestProtocolInjectionV1,
    }

    impl<J: NativeSchedulerJobV1> PlatformScheduler<J> {
        pub(super) fn prepare(
            selection: SchedulerSelectionV1,
            config: NativeSchedulerConfigV1,
            generation: u64,
        ) -> Result<Self, SchedulerPrepareErrorV1> {
            if selection != SchedulerSelectionV1::Parallel {
                return Ok(Self {
                    workers: Vec::new(),
                    parallel: false,
                    retained_queue_bytes: 0,
                    #[cfg(feature = "test-support")]
                    completion_acceptance_order: config.test_completion_acceptance_order,
                    #[cfg(feature = "test-support")]
                    last_completion_acceptance_order: [0; 3],
                    #[cfg(feature = "test-support")]
                    last_completion_acceptance_count: 0,
                    #[cfg(feature = "test-support")]
                    protocol_injection: config.test_protocol_injection,
                });
            }
            let worker_count = config.render_lanes.get() - 1;
            #[cfg(feature = "test-support")]
            if !test_completion_acceptance_order_is_valid(config.test_completion_acceptance_order) {
                return Err(SchedulerPrepareErrorV1::InvalidPartition);
            }
            let command_bytes = miso_engine_core::realtime::bounded_spsc_retained_payload::<
                WorkerMessage<J>,
            >(NonZeroUsize::new(1).expect("nonzero queue capacity"))
            .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?
            .total_bytes()
            .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
            let completion_bytes =
                miso_engine_core::realtime::bounded_spsc_retained_payload::<WorkerCompletion<J>>(
                    NonZeroUsize::new(1).expect("nonzero queue capacity"),
                )
                .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?
                .total_bytes()
                .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
            let retained_queue_bytes = command_bytes
                .checked_add(completion_bytes)
                .and_then(|per_worker| per_worker.checked_mul(worker_count))
                .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
            let worker_count_u64 = u64::try_from(worker_count)
                .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
            generation
                .checked_mul(2)
                .and_then(|value| {
                    worker_count_u64
                        .checked_mul(2)
                        .and_then(|count| value.checked_add(count))
                })
                .ok_or(SchedulerPrepareErrorV1::ResourceOverflow)?;
            let mut workers = Vec::with_capacity(worker_count);
            for worker_id in 0..worker_count {
                #[cfg(feature = "test-support")]
                let startup_handshake_failure = worker_id == 0
                    && config.test_protocol_injection
                        == SchedulerTestProtocolInjectionV1::StartupHandshakeFailure;
                let worker_id_u64 = u64::try_from(worker_id)
                    .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
                let command_generation = QueueGeneration(generation * 2 + worker_id_u64);
                let completion_generation = QueueGeneration(
                    command_generation
                        .0
                        .checked_add(worker_count_u64)
                        .and_then(|value| value.checked_add(1))
                        .expect("preflighted queue generation arithmetic"),
                );
                let (commands, worker_commands) = bounded_spsc_move(
                    NonZeroUsize::new(1).expect("nonzero queue capacity"),
                    command_generation,
                )
                .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
                let (worker_completions, completions) = bounded_spsc_move(
                    NonZeroUsize::new(1).expect("nonzero queue capacity"),
                    completion_generation,
                )
                .map_err(|_| SchedulerPrepareErrorV1::ResourceOverflow)?;
                let (ready_sender, ready_receiver) = mpsc::sync_channel(0);
                let handle = match thread::Builder::new()
                    .name(format!("miso-scheduler-{worker_id}"))
                    .spawn(move || {
                        miso_engine_core::realtime::audit::warm_up();
                        miso_engine_core::realtime::audit::reset();
                        #[cfg(feature = "test-support")]
                        if startup_handshake_failure {
                            return;
                        }
                        if ready_sender.send(()).is_ok() {
                            worker_loop(worker_commands, worker_completions);
                        }
                    }) {
                    Ok(handle) => handle,
                    Err(_) => {
                        stop_workers(&mut workers);
                        return Err(SchedulerPrepareErrorV1::WorkerStart);
                    }
                };
                if ready_receiver.recv().is_err() {
                    let _ = handle.join();
                    stop_workers(&mut workers);
                    return Err(SchedulerPrepareErrorV1::WorkerStart);
                }
                workers.push(Worker {
                    commands,
                    completions,
                    handle: Some(handle),
                    audit: miso_engine_core::realtime::audit::AuditSnapshot::default(),
                });
            }
            Ok(Self {
                workers,
                parallel: true,
                retained_queue_bytes,
                #[cfg(feature = "test-support")]
                completion_acceptance_order: config.test_completion_acceptance_order,
                #[cfg(feature = "test-support")]
                last_completion_acceptance_order: [0; 3],
                #[cfg(feature = "test-support")]
                last_completion_acceptance_count: 0,
                #[cfg(feature = "test-support")]
                protocol_injection: config.test_protocol_injection,
            })
        }

        // REALTIME_POLICY_BEGIN
        pub(super) fn render_wave(
            &mut self,
            wave: &mut RenderWaveV1<J>,
            generation: u64,
        ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
            if !self.parallel || wave.partition_count() == 1 {
                return execute_sequential(wave);
            }
            let worker_partitions = wave.partition_count() - 1;
            if worker_partitions > self.workers.len() {
                return Err(SchedulerDispatchErrorV1::CompletionMismatch {
                    worker_id: self.workers.len(),
                });
            }
            let wave_id = wave.layout().level_id;
            let mut report = SchedulerDispatchReportV1::default();
            let mut issued = 0_usize;
            #[cfg(feature = "test-support")]
            let completion_acceptance_order = self.completion_acceptance_order;
            for partition_index in 1..wave.partition_count() {
                #[cfg(feature = "test-support")]
                if self
                    .protocol_injection
                    .command_queue_is_full_for(partition_index - 1)
                {
                    let recovered = recover_issued(
                        &mut self.workers,
                        wave,
                        issued,
                        generation,
                        wave_id,
                        &mut report,
                        completion_acceptance_order,
                    );
                    if let Some(worker_id) = recovered.mismatch {
                        return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                    }
                    return Err(SchedulerDispatchErrorV1::CommandQueueFull {
                        worker_id: partition_index - 1,
                    });
                }
                let Some(parcel) = wave.partitions[partition_index].parcel.take() else {
                    let recovered = recover_issued(
                        &mut self.workers,
                        wave,
                        issued,
                        generation,
                        wave_id,
                        &mut report,
                        #[cfg(feature = "test-support")]
                        completion_acceptance_order,
                    );
                    if let Some(worker_id) = recovered.mismatch {
                        return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                    }
                    return Err(SchedulerDispatchErrorV1::MissingParcel {
                        partition_id: partition_index,
                    });
                };
                let command = WorkerMessage::Run(WorkerCommand {
                    generation,
                    wave_id,
                    partition_id: partition_index,
                    parcel,
                    #[cfg(feature = "test-support")]
                    protocol_injection: self.protocol_injection,
                });
                if let Err(full) = self.workers[partition_index - 1].commands.try_push(command) {
                    let WorkerMessage::Run(command) = full.value else {
                        let recovered = recover_issued(
                            &mut self.workers,
                            wave,
                            issued,
                            generation,
                            wave_id,
                            &mut report,
                            #[cfg(feature = "test-support")]
                            completion_acceptance_order,
                        );
                        if let Some(worker_id) = recovered.mismatch {
                            return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                        }
                        return Err(SchedulerDispatchErrorV1::CommandQueueFull {
                            worker_id: partition_index - 1,
                        });
                    };
                    wave.partitions[partition_index].parcel = Some(command.parcel);
                    let recovered = recover_issued(
                        &mut self.workers,
                        wave,
                        issued,
                        generation,
                        wave_id,
                        &mut report,
                        #[cfg(feature = "test-support")]
                        completion_acceptance_order,
                    );
                    if let Some(worker_id) = recovered.mismatch {
                        return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                    }
                    return Err(SchedulerDispatchErrorV1::CommandQueueFull {
                        worker_id: partition_index - 1,
                    });
                }
                issued += 1;
                report.worker_commands = report.worker_commands.saturating_add(1);
            }
            let Some(mut coordinator) = wave.partitions[0].parcel.take() else {
                let recovered = recover_issued(
                    &mut self.workers,
                    wave,
                    issued,
                    generation,
                    wave_id,
                    &mut report,
                    #[cfg(feature = "test-support")]
                    completion_acceptance_order,
                );
                if let Some(worker_id) = recovered.mismatch {
                    return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
                }
                return Err(SchedulerDispatchErrorV1::MissingParcel { partition_id: 0 });
            };
            let coordinator_result = coordinator.execute();
            wave.partitions[0].parcel = Some(coordinator);
            report.coordinator_jobs = 1;
            let recovered = recover_issued(
                &mut self.workers,
                wave,
                issued,
                generation,
                wave_id,
                &mut report,
                #[cfg(feature = "test-support")]
                completion_acceptance_order,
            );
            #[cfg(feature = "test-support")]
            {
                self.last_completion_acceptance_order = recovered.test_acceptance_order;
                self.last_completion_acceptance_count = recovered.test_acceptance_count;
            }
            if let Some(worker_id) = recovered.mismatch {
                return Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id });
            }
            if let Err(error) = coordinator_result {
                return Err(SchedulerDispatchErrorV1::Job(SchedulerJobFailureV1 {
                    partition_id: 0,
                    error,
                }));
            }
            if let Some(error) = recovered.first_error {
                return Err(SchedulerDispatchErrorV1::Job(error));
            }
            Ok(report)
        }
        // REALTIME_POLICY_END

        pub(super) fn stop_and_join(&mut self) {
            for worker in &mut self.workers {
                if worker.handle.is_none() {
                    continue;
                }
                loop {
                    match worker.commands.try_push(WorkerMessage::Stop) {
                        Ok(()) => break,
                        Err(full) => {
                            let _ = full.value;
                            let _ = worker.completions.try_pop();
                        }
                    }
                }
            }
            for worker in &mut self.workers {
                if let Some(handle) = worker.handle.take() {
                    let _ = handle.join();
                }
            }
            self.parallel = false;
        }

        pub(super) const fn worker_count(&self) -> usize {
            self.workers.len()
        }

        pub(super) fn selected_lanes(&self) -> usize {
            self.worker_count() + 1
        }

        pub(super) const fn retained_queue_bytes(&self) -> usize {
            self.retained_queue_bytes
        }

        pub(super) fn copy_worker_audit_snapshots(
            &self,
            output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
        ) -> usize {
            let count = output.len().min(self.workers.len());
            for (target, worker) in output[..count].iter_mut().zip(&self.workers) {
                *target = worker.audit;
            }
            count
        }

        #[cfg(feature = "test-support")]
        pub(super) const fn test_last_completion_acceptance_order(&self) -> ([u8; 3], u8) {
            (
                self.last_completion_acceptance_order,
                self.last_completion_acceptance_count,
            )
        }
    }

    fn execute_sequential<J: NativeSchedulerJobV1>(
        wave: &mut RenderWaveV1<J>,
    ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
        let mut report = SchedulerDispatchReportV1::default();
        for (partition_id, partition) in wave.partitions.iter_mut().enumerate() {
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

    fn stop_workers<J: NativeSchedulerJobV1>(workers: &mut [Worker<J>]) {
        for worker in workers.iter_mut() {
            if worker.handle.is_some() {
                let _ = worker.commands.try_push(WorkerMessage::Stop);
            }
        }
        for worker in workers.iter_mut() {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }

    struct Recovery<E> {
        mismatch: Option<usize>,
        first_error: Option<SchedulerJobFailureV1<E>>,
        #[cfg(feature = "test-support")]
        test_acceptance_order: [u8; 3],
        #[cfg(feature = "test-support")]
        test_acceptance_count: u8,
    }

    fn recover_issued<J: NativeSchedulerJobV1>(
        workers: &mut [Worker<J>],
        wave: &mut RenderWaveV1<J>,
        issued: usize,
        generation: u64,
        wave_id: u64,
        report: &mut SchedulerDispatchReportV1,
        #[cfg(feature = "test-support")] completion_acceptance_order: [u16; 3],
    ) -> Recovery<J::Error> {
        let mut first_error = None;
        let mut mismatch = None;
        #[cfg(feature = "test-support")]
        let mut test_acceptance_order = [0_u8; 3];
        #[cfg(feature = "test-support")]
        let mut test_acceptance_count = 0_u8;
        for acceptance_index in 0..issued {
            #[cfg(feature = "test-support")]
            let worker_id = test_completion_acceptance_worker_id(
                completion_acceptance_order,
                issued,
                acceptance_index,
            );
            #[cfg(not(feature = "test-support"))]
            let worker_id = acceptance_index;
            let worker = &mut workers[worker_id];
            let completion = loop {
                if let Ok(completion) = worker.completions.try_pop() {
                    break completion;
                }
                core::hint::spin_loop();
            };
            #[cfg(feature = "test-support")]
            if usize::from(test_acceptance_count) < test_acceptance_order.len() {
                let index = usize::from(test_acceptance_count);
                test_acceptance_order[index] = (worker_id + 1) as u8;
                test_acceptance_count = test_acceptance_count.saturating_add(1);
            }
            let expected_partition = worker_id + 1;
            if completion.generation != generation
                || completion.wave_id != wave_id
                || completion.partition_id != expected_partition
                || wave.partitions[expected_partition].parcel.is_some()
            {
                mismatch.get_or_insert(worker_id);
            }
            if wave.partitions[expected_partition].parcel.is_none() {
                wave.partitions[expected_partition].parcel = Some(completion.parcel);
            }
            worker.audit = completion.audit;
            report.worker_completions = report.worker_completions.saturating_add(1);
            if let Err(error) = completion.result {
                let replaces =
                    first_error
                        .as_ref()
                        .is_none_or(|current: &SchedulerJobFailureV1<J::Error>| {
                            expected_partition < current.partition_id
                        });
                if replaces {
                    first_error = Some(SchedulerJobFailureV1 {
                        partition_id: expected_partition,
                        error,
                    });
                }
            }
        }
        Recovery {
            mismatch,
            first_error,
            #[cfg(feature = "test-support")]
            test_acceptance_order,
            #[cfg(feature = "test-support")]
            test_acceptance_count,
        }
    }

    #[cfg(feature = "test-support")]
    const fn test_completion_acceptance_order_is_valid(order: [u16; 3]) -> bool {
        let mut seen = 0_u8;
        let mut index = 0;
        while index < order.len() {
            let worker = order[index];
            if worker >= 3 || seen & (1 << worker) != 0 {
                return false;
            }
            seen |= 1 << worker;
            index += 1;
        }
        seen == 0b111
    }

    #[cfg(feature = "test-support")]
    fn test_completion_acceptance_worker_id(
        order: [u16; 3],
        issued: usize,
        acceptance_index: usize,
    ) -> usize {
        if issued > order.len() && acceptance_index >= order.len() {
            return acceptance_index;
        }
        order
            .into_iter()
            .map(usize::from)
            .filter(|worker_id| *worker_id < issued)
            .nth(acceptance_index)
            .expect("validated test completion acceptance permutation")
    }

    fn worker_loop<J: NativeSchedulerJobV1>(
        mut commands: Consumer<WorkerMessage<J>>,
        mut completions: Producer<WorkerCompletion<J>>,
    ) {
        loop {
            let message = loop {
                if let Ok(message) = commands.try_pop() {
                    break message;
                }
                core::hint::spin_loop();
            };
            match message {
                WorkerMessage::Stop => return,
                WorkerMessage::Run(mut command) => {
                    let result = miso_engine_core::realtime::audit::in_render_scope(|| {
                        command.parcel.execute()
                    });
                    #[cfg(feature = "test-support")]
                    let (completion_generation, completion_partition_id) =
                        command.protocol_injection.completion_tokens(
                            command.partition_id.saturating_sub(1),
                            command.generation,
                            command.partition_id,
                        );
                    #[cfg(not(feature = "test-support"))]
                    let (completion_generation, completion_partition_id) =
                        (command.generation, command.partition_id);
                    let completion = WorkerCompletion {
                        generation: completion_generation,
                        wave_id: command.wave_id,
                        partition_id: completion_partition_id,
                        result,
                        audit: miso_engine_core::realtime::audit::snapshot(),
                        parcel: command.parcel,
                    };
                    let mut completion = Some(completion);
                    while let Some(value) = completion.take() {
                        completion = match completions.try_push(value) {
                            Ok(()) => None,
                            Err(full) => Some(full.value),
                        };
                        if completion.is_some() {
                            core::hint::spin_loop();
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod platform {
    use super::{
        NativeSchedulerConfigV1, NativeSchedulerJobV1, RenderWaveV1, SchedulerDispatchErrorV1,
        SchedulerDispatchReportV1, SchedulerJobFailureV1, SchedulerPrepareErrorV1,
        SchedulerSelectionV1,
    };

    /// Browser-only sequential driver; it contains no worker, atomics, or shared queue.
    pub(super) struct PlatformScheduler<J: NativeSchedulerJobV1> {
        _job: core::marker::PhantomData<J>,
    }

    impl<J: NativeSchedulerJobV1> PlatformScheduler<J> {
        pub(super) fn prepare(
            _selection: SchedulerSelectionV1,
            _config: NativeSchedulerConfigV1,
            _generation: u64,
        ) -> Result<Self, SchedulerPrepareErrorV1> {
            Ok(Self {
                _job: core::marker::PhantomData,
            })
        }

        pub(super) fn render_wave(
            &mut self,
            wave: &mut RenderWaveV1<J>,
            _generation: u64,
        ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
            let mut report = SchedulerDispatchReportV1::default();
            for (partition_id, partition) in wave.partitions.iter_mut().enumerate() {
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

        pub(super) fn stop_and_join(&mut self) {}

        pub(super) const fn worker_count(&self) -> usize {
            0
        }

        pub(super) const fn selected_lanes(&self) -> usize {
            1
        }

        pub(super) const fn retained_queue_bytes(&self) -> usize {
            0
        }

        pub(super) fn copy_worker_audit_snapshots(
            &self,
            _output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
        ) -> usize {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;
    #[cfg(feature = "test-support")]
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

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

    fn wave(transcript: Arc<Mutex<Vec<usize>>>, count: usize) -> RenderWaveV1<Job> {
        let ranges = partition_stable_units_v1(
            NonZeroUsize::new(count).expect("units"),
            NonZeroUsize::new(4).expect("lanes"),
        );
        let partitions = ranges
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

    #[test]
    fn stable_partitioning_has_no_padding_or_track_ceiling() {
        for count in [1_usize, 3, 4, 5, 12, 17] {
            let ranges = partition_stable_units_v1(
                NonZeroUsize::new(count).expect("count"),
                NonZeroUsize::new(4).expect("lanes"),
            );
            assert_eq!(ranges.first().expect("first").first_unit, 0);
            assert_eq!(ranges.last().expect("last").end_unit, count);
            assert!(
                ranges
                    .windows(2)
                    .all(|pair| pair[0].end_unit == pair[1].first_unit)
            );
            assert!(ranges.iter().all(|range| range.end_unit > range.first_unit));
            assert_eq!(ranges.len(), count.min(4));
            let minimum_width = ranges
                .iter()
                .map(|range| range.end_unit - range.first_unit)
                .min()
                .expect("one partition");
            let maximum_width = ranges
                .iter()
                .map(|range| range.end_unit - range.first_unit)
                .max()
                .expect("one partition");
            assert!(maximum_width - minimum_width <= 1, "track count {count}");
        }
    }

    #[test]
    fn disabled_and_narrow_preparation_select_the_same_sequential_parcels() {
        let transcript = Arc::new(Mutex::new(Vec::new()));
        let mut scheduler = NativeSchedulerV1::prepare(
            NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), false),
            4,
            9,
        )
        .expect("scheduler");
        assert_eq!(
            scheduler.selection(),
            SchedulerSelectionV1::Sequential(FallbackReasonV1::DisabledByHost)
        );
        let mut rendered = wave(Arc::clone(&transcript), 4);
        let report = scheduler
            .render_wave(&mut rendered)
            .expect("sequential render");
        assert_eq!(report.coordinator_jobs, 4);
        assert!(rendered.all_recovered());
        assert_eq!(*transcript.lock().expect("transcript"), vec![0, 1, 2, 3]);
    }

    #[test]
    fn native_workers_recover_move_only_parcels_and_select_errors_stably() {
        let transcript = Arc::new(Mutex::new(Vec::new()));
        let ranges = partition_stable_units_v1(
            NonZeroUsize::new(4).expect("units"),
            NonZeroUsize::new(4).expect("lanes"),
        );
        let partitions = ranges
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
        let mut scheduler = NativeSchedulerV1::prepare(
            NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true),
            4,
            10,
        )
        .expect("scheduler");
        let result = scheduler.render_wave(&mut rendered);
        match result {
            Err(SchedulerDispatchErrorV1::Job(error)) => assert_eq!(error.partition_id, 2),
            _ => panic!("stable worker failure was not returned"),
        }
        assert!(rendered.all_recovered());
        let mut audits = [miso_engine_core::realtime::audit::AuditSnapshot::default(); 3];
        assert_eq!(scheduler.copy_worker_audit_snapshots(&mut audits), 3);
        assert!(audits.into_iter().all(|snapshot| snapshot.total() == 0));
        scheduler.stop_and_join();
    }

    #[cfg(feature = "test-support")]
    struct ProtocolAudit {
        executions: [AtomicUsize; 4],
        drops: [AtomicUsize; 4],
    }

    #[cfg(feature = "test-support")]
    impl ProtocolAudit {
        fn new() -> Self {
            Self {
                executions: std::array::from_fn(|_| AtomicUsize::new(0)),
                drops: std::array::from_fn(|_| AtomicUsize::new(0)),
            }
        }

        fn assert_counts(&self, expected_executions: [usize; 4], expected_drops: [usize; 4]) {
            assert_eq!(
                self.executions
                    .each_ref()
                    .map(|count| count.load(Ordering::SeqCst)),
                expected_executions
            );
            assert_eq!(
                self.drops
                    .each_ref()
                    .map(|count| count.load(Ordering::SeqCst)),
                expected_drops
            );
        }
    }

    #[cfg(feature = "test-support")]
    struct ProtocolJob {
        partition_id: usize,
        audit: Arc<ProtocolAudit>,
        fail: bool,
    }

    #[cfg(feature = "test-support")]
    impl Drop for ProtocolJob {
        fn drop(&mut self) {
            self.audit.drops[self.partition_id].fetch_add(1, Ordering::SeqCst);
        }
    }

    #[cfg(feature = "test-support")]
    impl NativeSchedulerJobV1 for ProtocolJob {
        type Error = usize;

        fn execute(&mut self) -> Result<(), Self::Error> {
            self.audit.executions[self.partition_id].fetch_add(1, Ordering::SeqCst);
            if self.fail {
                Err(self.partition_id)
            } else {
                Ok(())
            }
        }
    }

    #[cfg(feature = "test-support")]
    fn protocol_wave(audit: Arc<ProtocolAudit>, failures: [bool; 4]) -> RenderWaveV1<ProtocolJob> {
        let ranges = partition_stable_units_v1(
            NonZeroUsize::new(4).expect("units"),
            NonZeroUsize::new(4).expect("lanes"),
        );
        let partitions = ranges
            .iter()
            .map(|range| {
                RenderPartitionV1::new(
                    *range,
                    ProtocolJob {
                        partition_id: range.partition_id,
                        audit: Arc::clone(&audit),
                        fail: failures[range.partition_id],
                    },
                )
            })
            .collect();
        RenderWaveV1::new(19, partitions).expect("canonical protocol wave")
    }

    #[cfg(feature = "test-support")]
    fn protocol_scheduler(
        injection: SchedulerTestProtocolInjectionV1,
    ) -> NativeSchedulerV1<ProtocolJob> {
        NativeSchedulerV1::prepare(
            NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true)
                .with_test_protocol_injection(injection),
            4,
            41,
        )
        .expect("parallel protocol scheduler")
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_startup_handshake_failure_never_publishes_a_scheduler() {
        let config = NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true)
            .with_test_protocol_injection(
                SchedulerTestProtocolInjectionV1::StartupHandshakeFailure,
            );
        let original = config;
        assert!(matches!(
            NativeSchedulerV1::<ProtocolJob>::prepare(config, 4, 41),
            Err(SchedulerPrepareErrorV1::WorkerStart)
        ));
        assert_eq!(
            config, original,
            "prepare must not mutate its configuration"
        );
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_command_queue_full_preserves_unmoved_parcels() {
        let audit = Arc::new(ProtocolAudit::new());
        let mut wave = protocol_wave(Arc::clone(&audit), [false; 4]);
        let mut scheduler =
            protocol_scheduler(SchedulerTestProtocolInjectionV1::CommandQueueFull { worker_id: 2 });

        assert!(matches!(
            scheduler.render_wave(&mut wave),
            Err(SchedulerDispatchErrorV1::CommandQueueFull { worker_id: 2 })
        ));
        assert!(wave.all_recovered());
        audit.assert_counts([0, 1, 1, 0], [0; 4]);

        drop(scheduler);
        drop(wave);
        audit.assert_counts([0, 1, 1, 0], [1; 4]);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_completion_acceptance_order_genuinely_reorders_real_parcels() {
        for order in [[0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]] {
            let audit = Arc::new(ProtocolAudit::new());
            let mut wave = protocol_wave(Arc::clone(&audit), [false; 4]);
            let mut scheduler = NativeSchedulerV1::prepare(
                NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true)
                    .with_test_completion_acceptance_order(order),
                4,
                41,
            )
            .expect("parallel completion-order scheduler");

            let report = scheduler
                .render_wave(&mut wave)
                .expect("completion-order wave");
            assert_eq!(report.worker_commands, 3);
            assert_eq!(report.worker_completions, 3);
            let (accepted, accepted_count) = scheduler.test_last_completion_acceptance_order();
            assert_eq!(accepted_count, 3);
            assert_eq!(
                accepted,
                order.map(|worker_id| worker_id + 1),
                "reported partition acceptance order must match actual dequeues"
            );
            assert_ne!(accepted, [1, 2, 3]);
            assert!(wave.all_recovered());
            audit.assert_counts([1; 4], [0; 4]);

            drop(scheduler);
            drop(wave);
            audit.assert_counts([1; 4], [1; 4]);
        }
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_completion_acceptance_order_rejects_invalid_permutations_before_startup() {
        for order in [[0, 0, 1], [0, 1, 3]] {
            assert!(matches!(
                NativeSchedulerV1::<ProtocolJob>::prepare(
                    NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true,)
                        .with_test_completion_acceptance_order(order),
                    4,
                    41,
                ),
                Err(SchedulerPrepareErrorV1::InvalidPartition)
            ));
        }
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_stale_generation_recovers_each_parcel_once() {
        let audit = Arc::new(ProtocolAudit::new());
        let mut wave = protocol_wave(Arc::clone(&audit), [false; 4]);
        let mut scheduler =
            protocol_scheduler(SchedulerTestProtocolInjectionV1::StaleGeneration { worker_id: 1 });

        assert!(matches!(
            scheduler.render_wave(&mut wave),
            Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id: 1 })
        ));
        assert!(wave.all_recovered());
        audit.assert_counts([1; 4], [0; 4]);

        drop(scheduler);
        drop(wave);
        audit.assert_counts([1; 4], [1; 4]);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_duplicate_completion_recovers_each_parcel_once() {
        let audit = Arc::new(ProtocolAudit::new());
        let mut wave = protocol_wave(Arc::clone(&audit), [false; 4]);
        let mut scheduler =
            protocol_scheduler(SchedulerTestProtocolInjectionV1::DuplicateCompletion {
                worker_id: 2,
            });

        assert!(matches!(
            scheduler.render_wave(&mut wave),
            Err(SchedulerDispatchErrorV1::CompletionMismatch { worker_id: 2 })
        ));
        assert!(wave.all_recovered());
        audit.assert_counts([1; 4], [0; 4]);

        drop(scheduler);
        drop(wave);
        audit.assert_counts([1; 4], [1; 4]);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_returns_worker_errors_in_stable_partition_order() {
        let audit = Arc::new(ProtocolAudit::new());
        let mut wave = protocol_wave(Arc::clone(&audit), [false, true, true, false]);
        let mut scheduler = NativeSchedulerV1::prepare(
            NativeSchedulerConfigV1::new(NonZeroUsize::new(4).expect("lanes"), true)
                .with_test_completion_acceptance_order([2, 1, 0]),
            4,
            41,
        )
        .expect("parallel protocol scheduler");

        match scheduler.render_wave(&mut wave) {
            Err(SchedulerDispatchErrorV1::Job(error)) => {
                assert_eq!(error.partition_id, 1);
                assert_eq!(error.error, 1);
            }
            _ => panic!("first worker error was not selected in partition order"),
        }
        assert!(wave.all_recovered());
        audit.assert_counts([1; 4], [0; 4]);

        drop(scheduler);
        drop(wave);
        audit.assert_counts([1; 4], [1; 4]);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn test_protocol_exactly_once_drops_hold_for_either_owner_endpoint_order() {
        for scheduler_drops_first in [false, true] {
            let audit = Arc::new(ProtocolAudit::new());
            let mut wave = protocol_wave(Arc::clone(&audit), [false; 4]);
            let mut scheduler = protocol_scheduler(SchedulerTestProtocolInjectionV1::None);

            let report = scheduler
                .render_wave(&mut wave)
                .expect("completed protocol wave");
            assert_eq!(report.coordinator_jobs, 1);
            assert_eq!(report.worker_commands, 3);
            assert_eq!(report.worker_completions, 3);
            assert!(wave.all_recovered());
            audit.assert_counts([1; 4], [0; 4]);

            if scheduler_drops_first {
                drop(scheduler);
                drop(wave);
            } else {
                drop(wave);
                drop(scheduler);
            }
            audit.assert_counts([1; 4], [1; 4]);
        }
    }
}
