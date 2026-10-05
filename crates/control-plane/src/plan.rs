//! Render-thread plan ownership, and the any-thread projection queries read.

use super::*;

/// The state a plan shares with its session and its query projection.
///
/// `reports` holds one adapter row per plan epoch a reader can still ask for (#1309 D4).
pub(crate) struct SharedPlanState<Row> {
    pub(crate) plan_alive: AtomicBool,
    pub(crate) active_epoch: AtomicU64,
    pub(crate) reports: Mutex<Vec<(u64, Row)>>,
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

impl<Row: Send> host_core::PlanSampleSource for SharedPlanState<Row> {
    fn next_absolute_sample(&self) -> u64 {
        self.render_sample.load(Ordering::Acquire)
    }
}

/// Copies the resource report of the epoch the render thread last published.
///
/// The epoch is read under the report lock (issue #1042). The control thread removes a row only
/// under that lock, and only once the atomic has moved past it; the atomic never moves back. So an
/// epoch read under the lock always has its row. Read before the lock, a render could publish the
/// next epoch and a control call remove the old row in between, and the lookup below would panic.
pub(crate) fn active_resources<Row: Copy>(shared: &SharedPlanState<Row>) -> Row {
    let reports = shared
        .reports
        .lock()
        .expect("plan resource report lock is not poisoned");
    let active = shared.active_epoch.load(Ordering::Acquire);
    reports
        .iter()
        .find_map(|(epoch, report)| (*epoch == active).then_some(*report))
        .expect("active plan epoch retains its resource report")
}

/// The render-thread plan: exclusively the render thread's, apart from its shared projection.
pub struct PlanState<A: ControlAdapter> {
    pub(crate) owner: RealtimePlanOwner,
    pub(crate) shared: Arc<SharedPlanState<A::Row>>,
    /// Issue #146: the render thread's floating-point environment has been attested once.
    ///
    /// The C ABI has no "the render thread starts now" call, so the plan's first render *is* its
    /// session start on that thread: the first block verifies that
    /// [`lane::fpenv::CanonicalFpEnv`] actually installed the canonical word, and every
    /// block after it takes an untaken branch. A plan is render-thread-exclusive, so a plain `Cell`
    /// is the whole synchronisation this needs.
    pub(crate) fp_env_attested: core::cell::Cell<bool>,
}

impl<A: ControlAdapter> PlanState<A> {
    pub(crate) fn new(owner: RealtimePlanOwner, shared: Arc<SharedPlanState<A::Row>>) -> Self {
        #[cfg(feature = "test-support")]
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
/// The adapter holds it apart from the render-thread-exclusive [`PlanState`] (capi: in its own
/// `Plan` field), so a resource query can run concurrently with a render call.
pub struct PlanQueries<A: ControlAdapter> {
    pub(crate) shared: Arc<SharedPlanState<A::Row>>,
}

impl<A: ControlAdapter> PlanQueries<A> {
    /// Copies the resource row of the currently active plan epoch.
    pub fn resources(&self) -> A::Row {
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
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| owners.current_plan_disposed += 1);
        }
    }
}

impl ObservedCandidatePlan {
    pub(crate) fn new(plan: PreparedRenderPlan) -> Self {
        #[cfg(feature = "test-support")]
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
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| owners.candidate_plan_disposed += 1);
        }
    }
}

pub(crate) struct ObservedReservation<'a> {
    pub(crate) inner: Option<PlanReplacementReservation<'a>>,
}

impl<'a> ObservedReservation<'a> {
    pub(crate) fn new(inner: PlanReplacementReservation<'a>) -> Self {
        #[cfg(feature = "test-support")]
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
        #[cfg(feature = "test-support")]
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
            #[cfg(feature = "test-support")]
            update_test_owners(|owners| {
                owners.candidate_plan_disposed += 1;
                owners.reservation_canceled += 1;
            });
        }
    }
}

impl<A: ControlAdapter> PlanState<A> {
    /// Copies the resource row of the currently active plan epoch.
    #[cfg(feature = "test-support")]
    pub fn resources(&self) -> A::Row {
        active_resources(&self.shared)
    }

    /// The render thread's plan owner, for reads that need no render in flight.
    pub fn owner(&self) -> &RealtimePlanOwner {
        &self.owner
    }

    /// The render thread's plan owner, so a test can drive a render in two halves.
    #[cfg(feature = "test-support")]
    pub fn test_owner_mut(&mut self) -> &mut RealtimePlanOwner {
        &mut self.owner
    }

    /// The atomic that publishes the epoch the render thread last rendered.
    #[cfg(feature = "test-support")]
    pub fn test_active_epoch(&self) -> &AtomicU64 {
        &self.shared.active_epoch
    }

    /// Whether this plan's render thread has attested its floating-point environment (#146).
    pub fn fp_env_attested(&self) -> bool {
        self.fp_env_attested.get()
    }

    /// Records that this plan's render thread attested its floating-point environment (#146).
    pub fn attest_fp_env(&self) {
        self.fp_env_attested.set(true);
    }

    /// Clones the any-thread query projection the adapter installs beside the plan.
    pub fn queries(&self) -> PlanQueries<A> {
        PlanQueries {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Render the block that must start at the plan's own next absolute sample.
    ///
    /// Continuity, output shape and clock overflow are core's rules now, reported as typed
    /// [`RenderError`] variants; the adapter maps each one to its own diagnostic code and adds
    /// nothing.
    pub fn render(
        &mut self,
        absolute_sample: u64,
        output: PlanarBufferMut<'_>,
    ) -> Result<(), RenderError> {
        let report = self
            .owner
            .render_contiguous(RenderIo { output }, absolute_sample)?;
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
    pub fn render_peak_observed(&self) -> bool {
        self.shared.render_peak_observed.load(Ordering::Relaxed)
    }

    /// Publish this block's observation. A `NaN` peak means "not measured this block".
    pub fn publish_render_observation(&self, peak: f32) {
        self.shared
            .render_sample
            .store(self.owner.next_absolute_sample(), Ordering::Release);
        self.shared
            .render_peak_bits
            .store(peak.to_bits(), Ordering::Release);
        self.shared.render_sequence.fetch_add(1, Ordering::AcqRel);
    }
}

impl<A: ControlAdapter> Drop for PlanState<A> {
    fn drop(&mut self) {
        self.shared.plan_alive.store(false, Ordering::Release);
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.current_plan_disposed += 1);
    }
}
