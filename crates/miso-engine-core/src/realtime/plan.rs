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
}
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
        Ok(RenderReport {
            plan_id: self.program.plan_id,
            next_absolute_sample: next,
            frames: envelope.quantum.0,
        })
    }
    // REALTIME_POLICY_END
}
