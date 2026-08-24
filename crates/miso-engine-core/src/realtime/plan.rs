//! Structurally immutable prepared plan and bounded silence reference renderer.

#![allow(dead_code)] // State fields are intentionally prepared now for later issue-owned kernels.

use super::{
    BufferArena, BufferArenaError, ParameterEventBuffer, ParameterValues, PlanarBufferMut,
    PlanarBufferRef,
};
use crate::{QuantumFrames, SampleRateHz, is_launch_sample_rate};
use core::{cell::Cell, num::NonZeroUsize};

/// Exact rate, quantum, and external I/O shape accepted by a plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderEnvelope {
    /// Caller-selected engine rate; no implicit sample-rate conversion occurs.
    pub sample_rate: SampleRateHz,
    /// Fixed number of frames in every full render block.
    pub quantum: QuantumFrames,
    /// Optional external planar input channel count.
    pub input_channels: Option<NonZeroUsize>,
    /// Required external planar PCM output channel count.
    pub output_channels: NonZeroUsize,
}
impl RenderEnvelope {
    /// Validate a launch-supported rate and nonzero quantum.
    pub fn validate(self) -> Result<Self, RenderError> {
        if !is_launch_sample_rate(self.sample_rate) {
            return Err(RenderError::UnsupportedRate);
        }
        if self.quantum.0 == 0 {
            return Err(RenderError::InvalidEnvelope);
        }
        Ok(self)
    }
}
/// Immutable input accepted by the prepare phase.
pub struct PrepareRenderPlan<'a> {
    /// Control-plane identity used in deterministic traces.
    pub plan_id: u64,
    /// Immutable external render shape.
    pub envelope: RenderEnvelope,
    /// Internal planar scratch buffers allocated during preparation.
    pub scratch: &'a [super::PlanarBufferSpec],
    /// Initial values for pre-resolved parameter slots.
    pub parameter_defaults: &'a [f32],
    /// Fixed event capacity for this plan.
    pub event_capacity: usize,
}
/// Immutable topology/schema placeholder; future graph compilation owns its contents.
#[derive(Debug)]
pub struct PreparedProgram {
    plan_id: u64,
    envelope: RenderEnvelope,
}
impl PreparedProgram {
    /// Control-plane plan identity.
    #[must_use]
    pub const fn plan_id(&self) -> u64 {
        self.plan_id
    }
    /// Immutable render envelope.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.envelope
    }
}
/// Render inputs and outputs. This only owns borrowed scalar/slice references.
pub struct RenderIo<'a> {
    /// Optional external planar input.
    pub input: Option<PlanarBufferRef<'a>>,
    /// External planar PCM output.
    pub output: PlanarBufferMut<'a>,
}

/// Engine-internal prepared-plan dispatch seam.
///
/// Implementations are constructed off the render thread and own all state needed to render a
/// complete block.  It is deliberately not a host callback interface.  Production
/// implementations are policy-limited to `miso-engine-graph`.
#[doc(hidden)]
pub trait PreparedPlanExecutor: Send {
    /// Render one already-validated block using only preallocated state.
    fn render(
        &mut self,
        arena: &mut BufferArena,
        input: Option<PlanarBufferRef<'_>>,
        output: PlanarBufferMut<'_>,
        time: RenderTime,
    ) -> Result<(), RenderError>;
    /// Bounded implementation-owned qualification counters, read only after rendering is
    /// disarmed. Production executors that do not expose counters retain the zero default.
    #[doc(hidden)]
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }
    /// Cumulative planar/AoSoA round-trips performed by every bank chain, read only after
    /// rendering is disarmed. Master plan §4.5 fixes this at one per bank chain per block.
    #[doc(hidden)]
    fn bank_transposes(&self) -> u64 {
        0
    }
    /// Copy cumulative auxiliary-worker audit snapshots after render is disarmed.
    #[doc(hidden)]
    fn copy_worker_audit_snapshots(&self, _output: &mut [super::audit::AuditSnapshot]) -> usize {
        0
    }
    /// Give up an executor-owned resource at the block-boundary swap.
    ///
    /// This exists so a persistent auxiliary worker pool outlives the plan that used it: the
    /// retiring executor hands its worker lease to the replacement instead of stopping and
    /// respawning threads on every structural change. It runs on the render thread, so it is a
    /// move and nothing else: no allocation, no drop, no wait.
    #[doc(hidden)]
    fn take_handover(&mut self) -> Option<ExecutorHandover> {
        None
    }
    /// Accept a hand-over, or refuse it by returning it unchanged.
    ///
    /// A refused hand-over is given back to the retiring executor by
    /// [`RealtimePlanOwner::enter_block`], so it is dropped only when the retired plan is
    /// reclaimed off the render thread. Implementations never drop the value here.
    #[doc(hidden)]
    fn accept_handover(&mut self, handover: ExecutorHandover) -> Option<ExecutorHandover> {
        Some(handover)
    }
    /// Bounded implementation-owned dispatch counters, read only after rendering is disarmed.
    #[doc(hidden)]
    fn dispatch_counters(&self) -> [u64; 4] {
        [0; 4]
    }
}

/// One executor-owned resource moved between prepared plans at a block boundary.
///
/// The box is allocated at bind and only ever *moved* on the render thread. `Box<dyn Any>` moves
/// and `downcast` are pointer operations; neither allocates nor frees.
#[doc(hidden)]
pub type ExecutorHandover = Box<dyn core::any::Any + Send>;
/// Absolute sample time supplied by the host; no wall clock is used.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderTime {
    /// Absolute sample index of this block's first frame.
    pub absolute_sample: u64,
}
/// Bounded render failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderError {
    /// The caller selected a sample rate outside the launch engine set.
    UnsupportedRate,
    /// The prepared envelope contains a zero or otherwise invalid dimension.
    InvalidEnvelope,
    /// External input is absent or does not match the prepared envelope.
    InputShape,
    /// External output does not match the prepared envelope.
    OutputShape,
    /// Advancing the absolute sample clock would overflow `u64`.
    TimeOverflow,
    /// A contiguous render was asked to start at a sample other than the one that follows the
    /// last rendered block. `expected` is the sample the plan is waiting for.
    TimeDiscontinuity {
        /// The absolute sample this plan's next contiguous block must start at.
        expected: u64,
    },
    /// Internal or external planar storage validation failed.
    Buffer(BufferArenaError),
}
impl From<BufferArenaError> for RenderError {
    fn from(value: BufferArenaError) -> Self {
        Self::Buffer(value)
    }
}
/// Fixed report returned after every invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RenderReport {
    /// Identity of the plan that rendered the complete block.
    pub plan_id: u64,
    /// Absolute sample immediately after the rendered block.
    pub next_absolute_sample: u64,
    /// Number of PCM frames rendered.
    pub frames: u32,
}
/// Exclusive realtime state. It is `Send` but intentionally `!Sync`.
///
/// ```compile_fail
/// use miso_engine_core::realtime::PreparedRenderPlan;
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<PreparedRenderPlan>();
/// ```
pub struct PreparedRenderPlan {
    program: PreparedProgram,
    arena: BufferArena,
    values: ParameterValues,
    events: ParameterEventBuffer,
    rendered_blocks: u64,
    next_absolute_sample: u64,
    executor: Option<Box<dyn PreparedPlanExecutor>>,
    _not_sync: Cell<()>,
    #[cfg(test)]
    drop_observer: Option<DropObserver>,
}

#[cfg(test)]
struct DropObserver {
    plan_id: u64,
    observer: std::sync::Arc<std::sync::Mutex<Vec<(u64, std::thread::ThreadId)>>>,
}

#[cfg(test)]
impl Drop for DropObserver {
    fn drop(&mut self) {
        self.observer
            .lock()
            .expect("drop observer lock")
            .push((self.plan_id, std::thread::current().id()));
    }
}
impl PreparedRenderPlan {
    /// Compile fixed storage and immutable program outside the render plane.
    pub fn prepare(request: PrepareRenderPlan<'_>) -> Result<Self, RenderError> {
        let envelope = request.envelope.validate()?;
        Ok(Self {
            program: PreparedProgram {
                plan_id: request.plan_id,
                envelope,
            },
            arena: BufferArena::try_new(request.scratch)?,
            values: ParameterValues::new(request.parameter_defaults),
            events: ParameterEventBuffer::with_capacity(request.event_capacity),
            rendered_blocks: 0,
            next_absolute_sample: 0,
            executor: None,
            _not_sync: Cell::new(()),
            #[cfg(test)]
            drop_observer: None,
        })
    }
    /// Construct a plan with an already validated internal executor.
    ///
    /// The executor is owned by the plan, so it is destroyed only when the retired plan is
    /// reclaimed off the render thread.  No publication behavior is changed here.
    #[doc(hidden)]
    pub fn prepare_with_executor(
        request: PrepareRenderPlan<'_>,
        executor: Box<dyn PreparedPlanExecutor>,
    ) -> Result<Self, RenderError> {
        let mut plan = Self::prepare(request)?;
        plan.executor = Some(executor);
        Ok(plan)
    }
    /// The absolute sample the next *contiguous* block must start at.
    ///
    /// Zero before the first render; after every successful render it is the previous block's
    /// `next_absolute_sample`. The clock lives here, in the plan, so that no host has to keep its
    /// own copy and drift from it -- both the C ABI and the browser host used to.
    #[must_use]
    pub const fn next_absolute_sample(&self) -> u64 {
        self.next_absolute_sample
    }
    /// Adopt a continuing timeline, for a plan that replaces a running one.
    ///
    /// Only [`super::RealtimePlanOwner`] calls this, at a block boundary, so a plan swap does not
    /// restart the host's clock at zero.
    pub(crate) const fn adopt_absolute_sample(&mut self, absolute_sample: u64) {
        self.next_absolute_sample = absolute_sample;
    }
    /// Immutable structural program.
    #[must_use]
    pub const fn program(&self) -> &PreparedProgram {
        &self.program
    }
    /// Immutable external render envelope.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.program.envelope
    }
    /// Preallocated internal planar arena.
    #[must_use]
    pub fn arena(&self) -> &BufferArena {
        &self.arena
    }
    /// Read bounded executor qualification counters outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn qualification_counters(&self) -> [u64; 2] {
        assert!(
            !super::audit::is_render_scope_active(),
            "qualification counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0, 0], PreparedPlanExecutor::qualification_counters)
    }
    /// Read bounded executor dispatch counters outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn dispatch_counters(&self) -> [u64; 4] {
        assert!(
            !super::audit::is_render_scope_active(),
            "dispatch counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or([0; 4], PreparedPlanExecutor::dispatch_counters)
    }
    /// The plan's internal executor, for the block-boundary hand-over in `plan_exchange`.
    pub(crate) fn executor_mut(&mut self) -> Option<&mut (dyn PreparedPlanExecutor + 'static)> {
        self.executor.as_deref_mut()
    }
    /// Read the cumulative bank transpose count outside the render scope.
    #[doc(hidden)]
    #[must_use]
    pub fn bank_transposes(&self) -> u64 {
        assert!(
            !super::audit::is_render_scope_active(),
            "bank transpose counters are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, PreparedPlanExecutor::bank_transposes)
    }
    /// Copy cumulative auxiliary-worker audit snapshots in stable worker order.
    #[doc(hidden)]
    pub fn copy_worker_audit_snapshots(&self, output: &mut [super::audit::AuditSnapshot]) -> usize {
        assert!(
            !super::audit::is_render_scope_active(),
            "worker audit snapshots are sealed until the render audit is disarmed"
        );
        self.executor
            .as_deref()
            .map_or(0, |executor| executor.copy_worker_audit_snapshots(output))
    }
    #[cfg(test)]
    pub(crate) fn set_drop_observer(
        &mut self,
        observer: std::sync::Arc<std::sync::Mutex<Vec<(u64, std::thread::ThreadId)>>>,
    ) {
        self.drop_observer = Some(DropObserver {
            plan_id: self.program.plan_id,
            observer,
        });
    }
    // REALTIME_POLICY_BEGIN
    /// Render a fixed full quantum. The reference implementation writes silence only.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        super::audit::in_render_scope(|| self.render_inner(io, time))
    }

    /// Render the block that must start at [`Self::next_absolute_sample`].
    ///
    /// The continuity rule is the engine's, not each host's transcription of it: a host that
    /// renders a quantum at a time calls this and never compares sample counters itself.
    pub fn render_contiguous(
        &mut self,
        io: RenderIo<'_>,
        absolute_sample: u64,
    ) -> Result<RenderReport, RenderError> {
        if absolute_sample != self.next_absolute_sample {
            return Err(RenderError::TimeDiscontinuity {
                expected: self.next_absolute_sample,
            });
        }
        self.render(io, RenderTime { absolute_sample })
    }

    pub(crate) fn render_inner(
        &mut self,
        mut io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        let envelope = self.program.envelope;
        let frames = envelope.quantum.0 as usize;
        if io.output.frames() != frames || io.output.channels() != envelope.output_channels.get() {
            return Err(RenderError::OutputShape);
        }
        match (io.input, envelope.input_channels) {
            (None, None) => {}
            (Some(input), Some(channels))
                if input.frames() == frames && input.channels() == channels.get() => {}
            _ => return Err(RenderError::InputShape),
        }
        let next = time
            .absolute_sample
            .checked_add(u64::from(envelope.quantum.0))
            .ok_or(RenderError::TimeOverflow)?;
        if let Some(executor) = &mut self.executor {
            executor.render(&mut self.arena, io.input, io.output, time)?;
        } else {
            let mut channel = 0;
            while channel < envelope.output_channels.get() {
                io.output.plane_mut(channel)?.fill(0.0);
                channel += 1;
            }
        }
        self.rendered_blocks = self.rendered_blocks.saturating_add(1);
        self.next_absolute_sample = next;
        Ok(RenderReport {
            plan_id: self.program.plan_id,
            next_absolute_sample: next,
            frames: envelope.quantum.0,
        })
    }
    // REALTIME_POLICY_END
}
