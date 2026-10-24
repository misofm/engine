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
    /// Blocks the executor failed after the live snapshot (#1502 D7), mirrored from the plan
    /// owner on the render thread when such a block happens.
    pub(crate) live_block_errors: AtomicU64,
    /// Blocks rendered in which a source underran (#1537 D1), mirrored from the plan owner on the
    /// render thread after each rendered block.
    pub(crate) source_underrun_blocks: AtomicU64,
    /// The applied-revision watermark render publishes (#1314 D4): any thread reads it.
    pub(crate) watermark: PlanWatermarkReader,
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
    /// The output meter's render half (#1581 D4): made with the session's first plan and part of
    /// no plan, so a plan swap does not touch it. The session hands out its reader and gate.
    pub(crate) output_meter: host_core::OutputPeakObserver,
}

impl<A: ControlAdapter> PlanState<A> {
    pub(crate) fn new(
        owner: RealtimePlanOwner,
        shared: Arc<SharedPlanState<A::Row>>,
        output_meter: host_core::OutputPeakObserver,
    ) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.current_plan_constructed += 1);
        Self {
            owner,
            shared,
            fp_env_attested: core::cell::Cell::new(false),
            output_meter,
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

    /// Blocks the executor failed after the block's live snapshot (#1502 D7): such a block may
    /// have applied live values and rendered no output. A defect path, normally 0.
    pub fn live_block_errors(&self) -> u64 {
        self.shared.live_block_errors.load(Ordering::Acquire)
    }

    /// Blocks rendered in which a source played zeros because the host had not delivered PCM it
    /// owed (#1537 D1): graph-wide, saturating, never reset.
    pub fn source_underrun_blocks(&self) -> u64 {
        self.shared.source_underrun_blocks.load(Ordering::Acquire)
    }

    /// Test seam (#1521 D6): stores `count` into the session's `live_block_errors` mirror, as
    /// render does on an error block. No valid plan reaches a counted error, so the adapters'
    /// tests drive the mirror-to-wire link through this. The next error block overwrites it.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_store_live_block_errors(&self, count: u64) {
        self.shared
            .live_block_errors
            .store(count, Ordering::Release);
    }

    /// The applied-revision watermark (#1314): which committed revision is in effect, since which
    /// render sample, with its outcome flags and counters. Pure and bounded: a read that keeps
    /// landing inside render's publication gives up with [`WatermarkBusy`], and the caller
    /// retries.
    pub fn watermark(&self) -> Result<PlanWatermark, WatermarkBusy> {
        self.shared.watermark.read()
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

    /// The bound candidate, unpublished (#1310 D4).
    pub(crate) fn plan(&self) -> &PreparedRenderPlan {
        self.inner.as_ref().expect("reservation is live").plan()
    }

    /// The bound candidate, unpublished, for the donation after the commit (#1310 D5).
    pub(crate) fn plan_mut(&mut self) -> &mut PreparedRenderPlan {
        self.inner.as_mut().expect("reservation is live").plan_mut()
    }

    /// The revisions the candidate completes as `superseded` when render adopts it (#1310 D6).
    pub(crate) fn set_superseded(&mut self, superseded: u64) {
        self.inner
            .as_mut()
            .expect("reservation is live")
            .set_superseded(superseded);
    }

    /// The outcome the candidate's own revisions complete with (#1314 D1): the transition
    /// fallback for a rebuild that carries a following route into a six-word route (#1713).
    pub(crate) fn set_outcome(&mut self, outcome: engine::realtime::CandidateOutcome) {
        self.inner
            .as_mut()
            .expect("reservation is live")
            .set_outcome(outcome);
    }

    /// Publish the candidate with its own structural revision (#1482 R1).
    pub(crate) fn commit(mut self, revision: u64) {
        self.inner
            .take()
            .expect("reservation commits once")
            .commit(revision);
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

    /// The active plan's host attachment as a `T` (#1381a D3), or `None` when the plan carries
    /// none or carries another type. Render side: a downcast of the attachment that
    /// [`RealtimePlanOwner::active_attachment_mut`] reaches, allocating and dropping nothing.
    pub fn active_attachment_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.owner.active_attachment_mut::<T>()
    }

    /// Prevalidate source preparation before a single-instance host admits the producer-side seek
    /// (#1381a): the active plan's [`RealtimePlanOwner::can_prepare_source_seek`].
    #[must_use]
    pub fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.owner.can_prepare_source_seek(source_index)
    }

    /// Prepare an admitted seek on the active plan between blocks (#1381a): the owner's
    /// [`RealtimePlanOwner::prepare_source_seek`].
    pub fn prepare_source_seek(
        &mut self,
        source_index: usize,
        generation: u64,
        frame: u64,
    ) -> bool {
        self.owner
            .prepare_source_seek(source_index, generation, frame)
    }

    /// Copy one track's retained response owners at the current completed block boundary
    /// (#1381a): the owner's [`RealtimePlanOwner::copy_response_snapshot`].
    ///
    /// # Errors
    ///
    /// The plan's typed refusal.
    pub fn copy_response_snapshot(
        &mut self,
        request: engine::realtime::ResponseSnapshotRequest<'_>,
    ) -> Result<engine::realtime::ResponseSnapshotCapture, engine::realtime::ResponseSnapshotError>
    {
        self.owner.copy_response_snapshot(request).map(|capture| {
            engine::realtime::ResponseSnapshotCapture {
                captured_sample: capture.captured_sample,
                owners: capture.owners,
            }
        })
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
    /// nothing. A block that advanced the applied-revision watermark returns the record render
    /// just published (#1349 D1); every other block returns `None`.
    pub fn render(
        &mut self,
        absolute_sample: u64,
        output: PlanarBufferMut<'_>,
    ) -> Result<Option<PlanWatermark>, RenderError> {
        let report = match self
            .owner
            .render_contiguous(RenderIo { output }, absolute_sample)
        {
            Ok(report) => report,
            Err(error) => {
                // Only an errored block can move the count (#1502 D7).
                self.shared
                    .live_block_errors
                    .store(self.owner.live_block_errors(), Ordering::Release);
                return Err(error);
            }
        };
        self.shared
            .active_epoch
            .store(report.active_epoch.0, Ordering::Release);
        self.shared
            .source_underrun_blocks
            .store(self.owner.source_underrun_blocks(), Ordering::Release);
        Ok(report.watermark)
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

    /// Publish the activity of one successfully rendered block, `[first_sample, next_sample)`,
    /// from its two output planes.
    ///
    /// The output meter observes the block first (#1581 D4): one gate load while it is inactive,
    /// otherwise a vector fold of each plane's sanitized peak ([`host_core::OutputPeakObserver`]).
    ///
    /// Issue #163 phase 4 item 2: the scan runs only for an armed observer. When
    /// [`Self::render_peak_observed`] is true the peak starts at `0.0` and folds
    /// `peak.max(sample.abs())` over plane 0 and then plane 1 in frame order; otherwise the block
    /// publishes `NaN`, which the consumer reads as "not measured", so an unarmed block never
    /// fabricates a peak of `0.0`. It costs at most one load per output sample and allocates
    /// nothing, so it is render-safe.
    pub fn observe_rendered_block(
        &mut self,
        planes: [&[f32]; 2],
        first_sample: u64,
        next_sample: u64,
    ) {
        self.output_meter
            .observe(planes[0], planes[1], first_sample, next_sample);
        let peak = if self.render_peak_observed() {
            let mut peak = 0.0_f32;
            for plane in planes {
                for &sample in plane {
                    peak = peak.max(sample.abs());
                }
            }
            peak
        } else {
            f32::NAN
        };
        self.publish_render_observation(peak);
    }

    /// Publish this block's observation. A `NaN` peak means "not measured this block".
    fn publish_render_observation(&self, peak: f32) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::NonZeroUsize;
    use engine::realtime::{
        BufferArena, ExecutorBlockReport, PrepareRenderPlan, PreparedPlanExecutor, RenderEnvelope,
        RenderTime, plan_exchange,
    };
    use engine::{QuantumFrames, SampleRateHz};

    struct FailAfterSnapshot;

    impl PreparedPlanExecutor for FailAfterSnapshot {
        fn render(
            &mut self,
            _arena: &mut BufferArena,
            _output: PlanarBufferMut<'_>,
            _time: RenderTime,
        ) -> Result<ExecutorBlockReport, RenderError> {
            Err(RenderError::InvalidEnvelope)
        }
    }

    struct TestAdapter;

    impl ControlAdapter for TestAdapter {
        type Row = PlanResources;
        const ADAPTER_ALLOCATIONS: &'static [u64] = &[];
    }

    fn test_state() -> PlanState<TestAdapter> {
        let plan = PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id: 1,
                envelope: RenderEnvelope {
                    sample_rate: SampleRateHz(48_000),
                    quantum: QuantumFrames(2),
                    output_channels: NonZeroUsize::new(1).expect("one"),
                },
                scratch: &[],
            },
            Box::new(FailAfterSnapshot),
        )
        .expect("plan");
        let (publisher, owner, _retirer) = plan_exchange(
            plan,
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            },
        )
        .expect("exchange");
        let shared = Arc::new(SharedPlanState {
            plan_alive: AtomicBool::new(true),
            active_epoch: AtomicU64::new(0),
            reports: Mutex::new(Vec::new()),
            render_sequence: AtomicU64::new(0),
            render_sample: AtomicU64::new(0),
            render_peak_bits: AtomicU32::new(0),
            render_peak_observed: AtomicBool::new(false),
            live_block_errors: AtomicU64::new(0),
            source_underrun_blocks: AtomicU64::new(0),
            watermark: publisher.watermark_reader(),
        });
        let (output_meter, _reader, _gate) =
            host_core::output_peak_meter(core::num::NonZeroU32::MIN).expect("output meter");
        PlanState::<TestAdapter>::new(owner, shared, output_meter)
    }

    /// #1570a D3: an armed observer publishes the largest absolute sample over both planes, and an
    /// unarmed block publishes `NaN` ("not measured") instead of a fabricated `0.0`; either way
    /// the render sequence advances once.
    #[test]
    fn a_rendered_block_publishes_the_peak_over_both_planes_only_when_armed() {
        let mut state = test_state();
        let left = [0.25_f32, -0.5];
        let right = [0.75_f32, -1.5];

        state.observe_rendered_block([&left, &right], 0, 2);
        assert!(f32::from_bits(state.shared.render_peak_bits.load(Ordering::Acquire)).is_nan());
        assert_eq!(state.shared.render_sequence.load(Ordering::Acquire), 1);

        state
            .shared
            .render_peak_observed
            .store(true, Ordering::Relaxed);
        state.observe_rendered_block([&left, &right], 0, 2);
        assert_eq!(
            f32::from_bits(state.shared.render_peak_bits.load(Ordering::Acquire)),
            1.5
        );
        assert_eq!(state.shared.render_sequence.load(Ordering::Acquire), 2);

        state.observe_rendered_block([&[0.0; 2], &[0.0; 2]], 0, 2);
        assert_eq!(
            f32::from_bits(state.shared.render_peak_bits.load(Ordering::Acquire)),
            0.0
        );
    }

    /// #1502 D7: a block the executor fails after the live snapshot is mirrored to the any-thread
    /// query, and a block the shape check refuses is not.
    #[test]
    fn the_live_block_error_count_reaches_the_any_thread_query() {
        let mut state = test_state();
        let queries = state.queries();

        let mut wide = [0.0_f32; 4];
        let refused = state.render(
            0,
            PlanarBufferMut::try_new(&mut wide, 2, 2, 2).expect("output"),
        );
        assert_eq!(refused, Err(RenderError::OutputShape));
        assert_eq!(queries.live_block_errors(), 0);

        let mut output = [0.0_f32; 2];
        let failed = state.render(
            0,
            PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
        );
        assert_eq!(failed, Err(RenderError::InvalidEnvelope));
        assert_eq!(queries.live_block_errors(), 1);
    }
}
