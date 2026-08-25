//! Render-thread plan ownership, and the any-thread projection queries read.

use super::*;

pub(crate) struct SharedPlanState {
    pub(crate) plan_alive: AtomicBool,
    pub(crate) active_epoch: AtomicU64,
    pub(crate) reports: Mutex<Vec<(u64, PlanResourceReport)>>,
    pub(crate) render_sequence: AtomicU64,
    pub(crate) render_sample: AtomicU64,
    pub(crate) render_peak_bits: AtomicU32,
    /// Whether anything can currently consume the per-block output peak (#163 phase 4 item 2).
    ///
    /// The render thread computed a 2 x `frames` scalar `abs`/`max` scan on every successful
    /// render and published the result unconditionally. Its only consumer is
    /// `SessionState::collect_render_activity`, which turns it into `MeterRecord`s and stages
    /// them through `ProtocolController::stage_meter_batch_event` -- and that call refuses
    /// outright unless the endpoint has configured meter handles and a nonzero meter period. With
    /// no meters configured, which is every session that has not asked for them, the scan ran 256
    /// times per block to produce a number nothing could read.
    ///
    /// This flag is that consumer condition, hoisted to where the render thread can see it. It is
    /// deliberately a *superset* of the refusal condition in `stage_meter_batch_event`: it omits
    /// the `provider_features.meters` term, so it can only ever cause the scan to run when it was
    /// not strictly needed, never to be skipped when it was.
    pub(crate) render_peak_observed: AtomicBool,
}

pub(crate) fn active_resources(shared: &SharedPlanState) -> PlanResourceReport {
    let active = shared.active_epoch.load(Ordering::Acquire);
    shared
        .reports
        .lock()
        .expect("plan resource report lock is not poisoned")
        .iter()
        .find_map(|(epoch, report)| (*epoch == active).then_some(*report))
        .expect("active plan epoch retains its resource report")
}

pub(crate) struct PlanState {
    pub(crate) owner: RealtimePlanOwner,
    pub(crate) shared: Arc<SharedPlanState>,
    /// Issue #146: the render thread's floating-point environment has been attested once.
    ///
    /// The C ABI has no "the render thread starts now" call, so the plan's first render *is* its
    /// session start on that thread: the first block verifies that
    /// [`miso_engine_lane::fpenv::CanonicalFpEnv`] actually installed the canonical word, and every
    /// block after it takes an untaken branch. A plan is render-thread-exclusive, so a plain `Cell`
    /// is the whole synchronisation this needs.
    pub(crate) fp_env_attested: core::cell::Cell<bool>,
}

impl PlanState {
    pub(crate) fn new(owner: RealtimePlanOwner, shared: Arc<SharedPlanState>) -> Self {
        #[cfg(test)]
        update_test_owners(|owners| owners.current_plan_constructed += 1);
        Self {
            owner,
            shared,
            fp_env_attested: core::cell::Cell::new(false),
        }
    }
}

/// Any-thread projection of a plan's frozen resource accounting.
///
/// Held in its own [`crate::Plan`] field, disjoint from the render-thread-exclusive
/// [`PlanState`], so `miso_engine_v2_plan_resources` can run concurrently with a render call.
pub(crate) struct PlanQueries {
    pub(crate) shared: Arc<SharedPlanState>,
}

impl PlanQueries {
    /// Copies the resource report of the currently active plan epoch.
    pub(crate) fn resources(&self) -> PlanResourceReport {
        active_resources(&self.shared)
    }
}

pub(crate) struct ObservedCandidatePlan {
    pub(crate) inner: Option<PreparedRenderPlan>,
}

pub(crate) struct ObservedRetiredPlan {
    pub(crate) inner: Option<PreparedRenderPlan>,
}

impl ObservedRetiredPlan {
    pub(crate) fn new(plan: PreparedRenderPlan) -> Self {
        Self { inner: Some(plan) }
    }
}

impl Drop for ObservedRetiredPlan {
    fn drop(&mut self) {
        if let Some(plan) = self.inner.take() {
            drop(plan);
            #[cfg(test)]
            update_test_owners(|owners| owners.current_plan_disposed += 1);
        }
    }
}

impl ObservedCandidatePlan {
    pub(crate) fn new(plan: PreparedRenderPlan) -> Self {
        #[cfg(test)]
        update_test_owners(|owners| owners.candidate_plan_constructed += 1);
        Self { inner: Some(plan) }
    }

    pub(crate) fn take(mut self) -> PreparedRenderPlan {
        self.inner.take().expect("candidate plan transfers once")
    }

    pub(crate) fn returned(plan: PreparedRenderPlan) -> Self {
        Self { inner: Some(plan) }
    }
}

impl Drop for ObservedCandidatePlan {
    fn drop(&mut self) {
        if let Some(plan) = self.inner.take() {
            drop(plan);
            #[cfg(test)]
            update_test_owners(|owners| owners.candidate_plan_disposed += 1);
        }
    }
}

pub(crate) struct ObservedReservation<'a> {
    pub(crate) inner: Option<PlanReplacementReservation<'a>>,
}

impl<'a> ObservedReservation<'a> {
    pub(crate) fn new(inner: PlanReplacementReservation<'a>) -> Self {
        #[cfg(test)]
        update_test_owners(|owners| owners.reservation_constructed += 1);
        Self { inner: Some(inner) }
    }

    pub(crate) fn epoch(&self) -> u64 {
        self.inner.as_ref().expect("reservation is live").epoch().0
    }

    pub(crate) fn commit(mut self) {
        self.inner
            .take()
            .expect("reservation commits once")
            .commit();
        #[cfg(test)]
        update_test_owners(|owners| {
            owners.candidate_plan_published += 1;
            owners.reservation_committed += 1;
        });
    }
}

impl Drop for ObservedReservation<'_> {
    fn drop(&mut self) {
        if let Some(inner) = self.inner.take() {
            drop(inner);
            #[cfg(test)]
            update_test_owners(|owners| {
                owners.candidate_plan_disposed += 1;
                owners.reservation_canceled += 1;
            });
        }
    }
}

impl PlanState {
    #[cfg(test)]
    pub(crate) fn resources(&self) -> PlanResourceReport {
        active_resources(&self.shared)
    }

    /// Clones the any-thread query projection installed in [`crate::Plan::queries`].
    pub(crate) fn queries(&self) -> PlanQueries {
        PlanQueries {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Render the block that must start at the plan's own next absolute sample.
    ///
    /// Continuity, output shape and clock overflow are core's rules now, reported as typed
    /// [`RenderError`] variants; capi maps each one to its own diagnostic code and adds nothing.
    pub(crate) fn render(
        &mut self,
        absolute_sample: u64,
        output: PlanarBufferMut<'_>,
    ) -> Result<(), u32> {
        let report = self
            .owner
            .render_contiguous(
                RenderIo {
                    input: None,
                    output,
                },
                absolute_sample,
            )
            .map_err(|error| match error {
                RenderError::OutputShape => plan_error::OUTPUT_SHAPE,
                RenderError::TimeDiscontinuity { .. } => plan_error::TIME_DISCONTINUITY,
                RenderError::TimeOverflow => plan_error::TIME_OVERFLOW,
                _ => plan_error::PLAN_REJECTED,
            })?;
        self.shared
            .active_epoch
            .store(report.active_epoch.0, Ordering::Release);
        Ok(())
    }

    /// Whether the peak scan has a consumer this block (#163 phase 4 item 2).
    ///
    /// `Relaxed` is the correct ordering and not a shortcut: the flag guards no other memory, the
    /// value it gates is published through its own `Release` store below, and a render that
    /// straddles the control thread's `refresh_render_peak_gate` is allowed either answer. Taking
    /// the stale `false` for one block costs one dropped record on the lossy telemetry lane --
    /// which that lane documents as permitted -- and `publish_render_observation` marks that block
    /// so the consumer drops it rather than reading a `0.0` that was never measured.
    pub(crate) fn render_peak_observed(&self) -> bool {
        self.shared.render_peak_observed.load(Ordering::Relaxed)
    }

    /// Publish this block's observation. A `NaN` peak means "not measured this block".
    pub(crate) fn publish_render_observation(&self, peak: f32) {
        self.shared
            .render_sample
            .store(self.owner.next_absolute_sample(), Ordering::Release);
        self.shared
            .render_peak_bits
            .store(peak.to_bits(), Ordering::Release);
        self.shared.render_sequence.fetch_add(1, Ordering::AcqRel);
    }
}

impl Drop for PlanState {
    fn drop(&mut self) {
        self.shared.plan_alive.store(false, Ordering::Release);
        #[cfg(test)]
        update_test_owners(|owners| owners.current_plan_disposed += 1);
    }
}
