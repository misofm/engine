//! Browser/Wasm ownership: there is none.
//!
//! The artifact contains no worker, no atomics beyond the shared parcel types, and no
//! shared-memory claim. [`WorkerLeaseV1`] exists so the block API has one signature on every
//! target; it cannot be constructed here, so every wave takes the sequential driver.

use crate::{
    NativeSchedulerJobV1, NativeSchedulerV1, NativeWorkerPoolConfigV1, NativeWorkerPoolShapeV1,
    RenderWaveV1, SchedulerDispatchErrorV1, SchedulerDispatchReportV1, SchedulerPrepareErrorV1,
    execute_sequential,
};

/// The browser has no auxiliary worker pool.
pub struct NativeWorkerPoolV1<J: NativeSchedulerJobV1> {
    _job: core::marker::PhantomData<J>,
}

/// The browser has no worker lease; this type is uninhabited in practice.
pub struct WorkerLeaseV1<J: NativeSchedulerJobV1> {
    _job: core::marker::PhantomData<J>,
    _never: core::convert::Infallible,
}

impl<J: NativeSchedulerJobV1> NativeWorkerPoolV1<J> {
    /// Starting a pool is not supported on Wasm.
    ///
    /// # Errors
    /// Always returns [`SchedulerPrepareErrorV1::WorkerStart`].
    pub fn start(
        _config: NativeWorkerPoolConfigV1,
    ) -> Result<(Self, WorkerLeaseV1<J>), SchedulerPrepareErrorV1> {
        Err(SchedulerPrepareErrorV1::WorkerStart)
    }

    /// The browser pool is always empty.
    #[must_use]
    pub const fn shape(&self) -> NativeWorkerPoolShapeV1 {
        NativeWorkerPoolShapeV1 {
            worker_count: 0,
            spin_ns: 1,
        }
    }

    /// There is never a lease to recover.
    pub const fn recover_lease(&mut self) -> Option<WorkerLeaseV1<J>> {
        None
    }

    /// There is nothing to stop.
    pub fn stop_and_join(self) {}
}

impl<J: NativeSchedulerJobV1> WorkerLeaseV1<J> {
    /// The browser lease carries no worker.
    #[must_use]
    pub const fn worker_count(&self) -> usize {
        0
    }

    /// The browser lease never publishes an idle budget.
    pub const fn set_idle_spin(&mut self, _iterations: u64) {}
}

pub(crate) const fn retained_queue_bytes<J: NativeSchedulerJobV1>(
    _worker_count: usize,
) -> Result<usize, SchedulerPrepareErrorV1> {
    Ok(0)
}

impl<J: NativeSchedulerJobV1> NativeSchedulerV1<J> {
    /// Open a block. The browser has no worker to wake and no parcel to reap.
    pub fn begin_block(
        &mut self,
        _lease: Option<&mut WorkerLeaseV1<J>>,
        _waves: &mut [RenderWaveV1<J>],
        _reaped: &mut [Option<(usize, usize)>],
    ) -> usize {
        0
    }

    /// Render one wave on the calling lane.
    ///
    /// # Errors
    /// Propagates the first parcel failure in stable partition order.
    // REALTIME_POLICY_BEGIN
    pub fn render_wave(
        &mut self,
        _lease: Option<&mut WorkerLeaseV1<J>>,
        wave: &mut RenderWaveV1<J>,
    ) -> Result<SchedulerDispatchReportV1, SchedulerDispatchErrorV1<J::Error>> {
        execute_sequential(wave)
    }
    // REALTIME_POLICY_END

    /// Close a block.
    pub fn end_block(&mut self, _lease: Option<&mut WorkerLeaseV1<J>>) {}

    /// Copy cumulative per-worker realtime audit snapshots in stable worker order.
    pub fn copy_worker_audit_snapshots(
        &self,
        _lease: Option<&WorkerLeaseV1<J>>,
        _output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
    ) -> usize {
        0
    }
}
