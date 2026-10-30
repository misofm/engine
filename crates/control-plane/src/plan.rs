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
    /// The output loudness meter's render half (#2115): made with the session's first plan and
    /// part of no plan, as the output meter is. The session keeps its control half.
    pub(crate) loudness: crate::loudness::LoudnessTap,
    /// The render half of the session's response capture (#1577 D3): made with the session's
    /// first plan and part of no plan, run after every successful block.
    pub(crate) response_capture: host_core::ResponseCaptureServer,
    /// The relay of the output meter's gate (#1583) when the session's meter delivery runs on the
    /// control thread (the C ABI): the delivery writes a requested gate, and [`Self::render`]
    /// applies it before each block, so the gate moves only here, between blocks. `None` when the
    /// delivery shares this thread (the browser), whose gate the delivery moves between blocks.
    pub(crate) output_gate: Option<host_core::meters::OutputPeakGateRelay>,
    /// The epoch of the plan the last rendered block ran (#1587a): a block whose report names
    /// another adopted a successor, and [`Self::render`] restarts the relay's gate there when the
    /// successor carried no strip meter.
    pub(crate) rendered_epoch: u64,
}

impl<A: ControlAdapter> PlanState<A> {
    pub(crate) fn new(
        owner: RealtimePlanOwner,
        shared: Arc<SharedPlanState<A::Row>>,
        output_meter: host_core::OutputPeakObserver,
        output_gate: Option<host_core::meters::OutputPeakGateRelay>,
        response_capture: host_core::ResponseCaptureServer,
        loudness: crate::loudness::LoudnessTap,
    ) -> Self {
        #[cfg(feature = "test-support")]
        update_test_owners(|owners| owners.current_plan_constructed += 1);
        Self {
            owner,
            shared,
            fp_env_attested: core::cell::Cell::new(false),
            output_meter,
            loudness,
            response_capture,
            output_gate,
            rendered_epoch: 0,
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

    /// Prepare every admitted plain seek on the active plan between blocks (#2213): the owner's
    /// [`RealtimePlanOwner::prepare_admitted_seeks`]. Render side: the caller is the plan's
    /// exclusive render owner while render does not run.
    pub fn prepare_admitted_seeks(&mut self) -> usize {
        self.owner.prepare_admitted_seeks()
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

    /// Adopt the published candidate in place before the plan's first block (#1597), for
    /// [`crate::SessionState::command_with_plan`]: the owner's
    /// [`RealtimePlanOwner::adopt_before_first_block`], then the epoch store a rendered block
    /// makes, so the session's next service step promotes the candidate. The adopted plan is the
    /// one the first block renders, so the relay sees no swap at that block, as for a first plan.
    /// Returns whether it adopted.
    pub(crate) fn adopt_before_first_block(&mut self) -> bool {
        if self.owner.adopt_before_first_block() != engine::realtime::SwapOutcome::Applied {
            return false;
        }
        let epoch = self.owner.active_epoch().0;
        self.rendered_epoch = epoch;
        self.shared.active_epoch.store(epoch, Ordering::Release);
        true
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
        // #1583: a requested gate reaches the output meter here, before the block, so this block's
        // strip meter level and its output fold read the one gate value.
        if let Some(relay) = self.output_gate.as_mut() {
            relay.apply(absolute_sample);
        }
        // #882: the strip meters observe exactly the blocks the output meter does, by its gate's
        // own rule, so a block rendered with the meter level off does no strip meter work. A block
        // that adopts a successor carrying no strip meter restarts the grid there, by the gate's
        // restart rule.
        self.owner
            .set_strip_meter_level(self.output_meter.admits(absolute_sample));
        self.owner
            .set_strip_meter_level_at_restart(self.output_meter.admits_restart_block());
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
        // #1587a: a block that adopted a successor whose strip meters were not carried restarted
        // the strip meter grid at its first sample, so the output meter's gate restarts there
        // too, before `observe_rendered_block` folds the block: the rule of the control thread's
        // `MeterDelivery::adopt`, whose request then asks for the same value.
        if report.active_epoch.0 != self.rendered_epoch {
            self.rendered_epoch = report.active_epoch.0;
            if !self.owner.active_strip_meters_carried()
                && let Some(relay) = self.output_gate.as_mut()
            {
                relay.restart_at(absolute_sample);
            }
        }
        self.shared
            .active_epoch
            .store(report.active_epoch.0, Ordering::Release);
        self.shared
            .source_underrun_blocks
            .store(self.owner.source_underrun_blocks(), Ordering::Release);
        // #1577 D3: after the epoch store, so the control side never reads an epoch this step
        // has not yet seen; never after an error.
        self.response_capture.after_block(&mut self.owner);
        Ok(report.watermark)
    }

    /// Publish the activity of one successfully rendered block, `[first_sample, next_sample)`,
    /// from its two output planes.
    ///
    /// The output meter observes the block (#1581 D4): one gate load while it is inactive,
    /// otherwise a vector fold of each plane's sanitized peak ([`host_core::OutputPeakObserver`]).
    /// The loudness meter observes it too (#2115): one gate load while it is disarmed, otherwise
    /// its K-weighted hop sums and Annex-2 true peak. Then the render sample and sequence advance,
    /// which the service step's counters read. The per-block peak scan is gone (#1583 D6): the
    /// meter batches are the output meter's windows.
    pub fn observe_rendered_block(
        &mut self,
        planes: [&[f32]; 2],
        first_sample: u64,
        next_sample: u64,
    ) {
        self.output_meter
            .observe(planes[0], planes[1], first_sample, next_sample);
        // #2115: one gate load while the loudness meter is disarmed.
        self.loudness.observe(planes[0], planes[1], first_sample);
        self.publish_render_observation();
    }

    /// Publish this block's render sample and sequence.
    fn publish_render_observation(&self) {
        self.shared
            .render_sample
            .store(self.owner.next_absolute_sample(), Ordering::Release);
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        test_state_with(Box::new(FailAfterSnapshot)).0
    }

    /// A one-channel, two-frame plan rendering through `executor`, with its output meter's gate.
    fn test_state_with(
        executor: Box<dyn PreparedPlanExecutor>,
    ) -> (PlanState<TestAdapter>, host_core::OutputPeakGate) {
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
            executor,
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
            live_block_errors: AtomicU64::new(0),
            source_underrun_blocks: AtomicU64::new(0),
            watermark: publisher.watermark_reader(),
        });
        let (output_meter, _reader, gate) =
            host_core::output_peak_meter(core::num::NonZeroU32::MIN).expect("output meter");
        (
            PlanState::<TestAdapter>::new(
                owner,
                shared,
                output_meter,
                None,
                host_core::response_capture_channel()
                    .expect("capture channel")
                    .1,
                crate::loudness::loudness_meter(48_000)
                    .expect("loudness meter")
                    .0,
            ),
            gate,
        )
    }

    /// Records the strip meter level each block hands it, and renders silence.
    struct RecordLevel(Arc<Mutex<Vec<bool>>>);

    impl PreparedPlanExecutor for RecordLevel {
        fn retained_bytes(&self) -> u64 {
            0
        }
        fn set_strip_meter_level(&mut self, on: bool) {
            self.0.lock().expect("levels").push(on);
        }
        fn render(
            &mut self,
            _arena: &mut BufferArena,
            _output: PlanarBufferMut<'_>,
            _time: RenderTime,
        ) -> Result<ExecutorBlockReport, RenderError> {
            Ok(ExecutorBlockReport::default())
        }
    }

    /// #882 gate 2: the executor's strip meter level is the output meter gate's answer for the
    /// same block: off while the gate is inactive, off for a block before the activation sample,
    /// on from it, and off again once the gate shuts.
    ///
    /// Test value: red if `PlanState::render` hands the executor a level other than the output
    /// gate's rule for the block (inverted, off by one block, never set, or set after render).
    #[test]
    fn the_strip_meter_level_follows_the_output_meter_gate_per_block() {
        let levels = Arc::new(Mutex::new(Vec::new()));
        let (mut state, gate) = test_state_with(Box::new(RecordLevel(Arc::clone(&levels))));
        let mut block = |sample: u64| {
            let mut output = [0.0_f32; 2];
            state
                .render(
                    sample,
                    PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
                )
                .expect("render");
        };
        block(0);
        gate.activate(4);
        block(2);
        block(4);
        block(6);
        gate.deactivate();
        block(8);
        assert_eq!(
            *levels.lock().expect("levels"),
            [false, false, true, true, false]
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
