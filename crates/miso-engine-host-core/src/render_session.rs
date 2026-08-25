//! The started render session: the render thread's own attestation, and the guarded render call.
//!
//! # Why a second handle exists
//!
//! [`PreparedHost`] is prepared on a control thread and moved once. Nothing in its type says
//! *which* thread it landed on, and nothing made the engine look at that thread's floating-point
//! environment before the first block. Issue #144 showed why that matters: under a caller's
//! hardware FTZ+DAZ, 69-70 of 331 corpus rows render off-pin, and every DAW audio callback arrives
//! with FTZ+DAZ set.
//!
//! Legacy engine-v2-old solved the same problem with `MountedSession::start`: the mount is one
//! thing, *starting* it on the render thread is another, and the started object is the only thing
//! that can render. [`StartedRenderSessionV1`] is that shape.
//!
//! * [`StartedRenderSessionV1::start`] runs on the render thread and re-attests there --
//!   `miso_engine_lane::attest_fp_environment` proves the canonical word can be installed *and*
//!   the caller's word restored bit-exactly on this thread -- rather than trusting the boot-time
//!   attestation of whatever thread happened to call `prepare_host_session`.
//! * The handle is **neither `Send` nor `Sync`**. An attestation is a statement about one thread,
//!   so a handle that could be moved or shared would let a host launder it onto a thread that was
//!   never attested. [`PreparedHost`] stays `Send` (moving preparation to the render thread is the
//!   supported hand-off); what cannot move is the *started* session.
//! * [`StartedRenderSessionV1::render`] is the guarded entry: it pins the canonical environment for
//!   the block and restores the caller's exact word on every path out, success or rejection.
//! * There is no `plan_mut`. A host cannot borrow the plan out of a started session and render it
//!   unguarded; it calls [`StartedRenderSessionV1::stop`], which consumes the handle and gives the
//!   plan back for a control-thread teardown or a plan replacement.
//!
//! `stop` is also what makes the failure path of `start` honest: an attestation failure hands the
//! plan back rather than dropping it, because the render thread frees nothing.

use core::marker::PhantomData;

use miso_engine_core::realtime::{
    PlanarBufferMut, PreparedRenderPlan, RenderError, RenderIo, RenderReport, RenderTime,
};
use miso_engine_lane::CanonicalFpEnv;
use miso_engine_lane::fpenv::FpEnvironmentRejection;

use crate::prepare::{HostPrepareReport, PreparedHost};
use crate::source::SourceControlSet;

/// A render plan that has been started on -- and pinned to -- the calling render thread.
///
/// `Send` and `Sync` are both refused. The `PhantomData<*const ()>` field is what refuses `Send`;
/// `Sync` is refused twice over, because `PreparedRenderPlan` is already `!Sync` and this owns one
/// (red mutation M-146-4 in `tests/MUTATIONS.md` shows exactly which claim the marker carries):
///
/// ```compile_fail
/// fn requires_send<T: Send>() {}
/// requires_send::<miso_engine_host_core::StartedRenderSessionV1>();
/// ```
///
/// ```compile_fail
/// fn requires_sync<T: Sync>() {}
/// requires_sync::<miso_engine_host_core::StartedRenderSessionV1>();
/// ```
pub struct StartedRenderSessionV1 {
    plan: PreparedRenderPlan,
    _not_send_not_sync: PhantomData<*const ()>,
}

impl StartedRenderSessionV1 {
    /// Attest this thread's floating-point environment and take ownership of the plan.
    ///
    /// Call this from the render thread, once, before the first block -- not from the control
    /// thread that prepared the plan.
    ///
    /// # Errors
    ///
    /// Returns the plan unchanged alongside a [`FpEnvironmentRejection`] whose `Display` is the
    /// stable `fp_environment_invalid` token, when the canonical control word cannot be installed
    /// on this thread or the caller's word is not restored bit-exactly. The plan comes back so the
    /// host can move it to a control thread and drop it there: the render thread frees nothing.
    // `allow`, not `expect`: `PreparedRenderPlan` is under the lint's threshold on `wasm32`, where
    // this crate also compiles, so an `expect` would be unfulfilled there.
    #[allow(
        clippy::result_large_err,
        reason = "the plan is the Err payload on purpose: a refused attestation hands it back, so the host drops it on a control thread and the render thread frees nothing"
    )]
    pub fn start(
        plan: PreparedRenderPlan,
    ) -> Result<Self, (PreparedRenderPlan, FpEnvironmentRejection)> {
        match miso_engine_lane::attest_fp_environment() {
            Ok(()) => Ok(Self {
                plan,
                _not_send_not_sync: PhantomData,
            }),
            Err(rejection) => Err((plan, rejection)),
        }
    }

    /// Render the block that must start at this plan's next absolute sample.
    ///
    /// The canonical floating-point environment is pinned for the whole call and the caller's exact
    /// word is restored on every path out, including a rejection and an unwind (issue #146).
    ///
    /// # Errors
    ///
    /// Whatever `PreparedRenderPlan::render_contiguous` rejects: a discontinuous absolute sample, a
    /// mismatched output envelope, or a clock overflow. A [`RenderError`] is sticky and frees
    /// nothing; see the crate-level host callback contract.
    pub fn render_contiguous(
        &mut self,
        io: RenderIo<'_>,
        absolute_sample: u64,
    ) -> Result<RenderReport, RenderError> {
        let _fp_env = CanonicalFpEnv::enter();
        self.plan.render_contiguous(io, absolute_sample)
    }

    /// Render one fixed quantum at an explicit absolute sample time.
    ///
    /// The same guarantee as [`Self::render_contiguous`]; this is the form for a host that owns its
    /// own clock continuity rule.
    ///
    /// # Errors
    ///
    /// Whatever `PreparedRenderPlan::render` rejects.
    pub fn render(
        &mut self,
        io: RenderIo<'_>,
        time: RenderTime,
    ) -> Result<RenderReport, RenderError> {
        let _fp_env = CanonicalFpEnv::enter();
        self.plan.render(io, time)
    }

    /// Render one quantum into caller-owned contiguous planar storage.
    ///
    /// # Errors
    ///
    /// Whatever [`Self::render_contiguous`] rejects, plus the buffer-layout rejection of
    /// `PlanarBufferMut::try_new`.
    pub fn render_planar(
        &mut self,
        samples: &mut [f32],
        channels: usize,
        frames: usize,
        plane_stride: usize,
        absolute_sample: u64,
    ) -> Result<RenderReport, RenderError> {
        let _fp_env = CanonicalFpEnv::enter();
        let output = PlanarBufferMut::try_new(samples, channels, frames, plane_stride)?;
        self.plan.render_contiguous(
            RenderIo {
                input: None,
                output,
            },
            absolute_sample,
        )
    }

    /// Address-free facts about the plan this session renders.
    #[must_use]
    pub fn next_absolute_sample(&self) -> u64 {
        self.plan.next_absolute_sample()
    }

    /// Stop the session and hand the plan back for control-thread teardown or replacement.
    #[must_use]
    pub fn stop(self) -> PreparedRenderPlan {
        self.plan
    }
}

impl core::fmt::Debug for StartedRenderSessionV1 {
    /// Address-free: the plan is not printable, so only its clock position appears.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("StartedRenderSessionV1")
            .field("next_absolute_sample", &self.plan.next_absolute_sample())
            .finish_non_exhaustive()
    }
}

impl PreparedHost {
    /// Split a prepared host into a render-thread session and its control-side halves.
    ///
    /// Call the whole thing on the render thread the session will render on, or move the
    /// [`PreparedHost`] there first: [`StartedRenderSessionV1`] cannot be moved afterwards. The
    /// returned [`SourceControlSet`] and [`HostPrepareReport`] are `Send` and go back to the
    /// control thread.
    ///
    /// # Errors
    ///
    /// Returns the untouched [`PreparedHost`] alongside the [`FpEnvironmentRejection`] when this
    /// thread's floating-point environment cannot be pinned.
    // `allow`, not `expect`: `PreparedRenderPlan` is under the lint's threshold on `wasm32`, where
    // this crate also compiles, so an `expect` would be unfulfilled there.
    #[allow(
        clippy::result_large_err,
        reason = "the plan is the Err payload on purpose: a refused attestation hands it back, so the host drops it on a control thread and the render thread frees nothing"
    )]
    pub fn start_render_session(
        self,
    ) -> Result<
        (StartedRenderSessionV1, SourceControlSet, HostPrepareReport),
        (Self, FpEnvironmentRejection),
    > {
        let Self {
            plan,
            sources,
            report,
        } = self;
        match StartedRenderSessionV1::start(plan) {
            Ok(started) => Ok((started, sources, report)),
            Err((plan, rejection)) => Err((
                Self {
                    plan,
                    sources,
                    report,
                },
                rejection,
            )),
        }
    }
}
