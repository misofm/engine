//! Realtime-only ownership, storage, and block-boundary publication primitives.
//!
//! `PreparedRenderPlan::render` is deliberately a small, allocation-free surface.  All capacity
//! decisions are made by `prepare`; objects returned here are not synchronizable so exclusive
//! render ownership remains apparent in the type system.

pub mod audit;
mod buffer;
mod disjoint;
mod latest_cell;
mod observe;
mod plan;
mod plan_exchange;
mod seqlock;
mod spsc;
mod watermark;

pub use buffer::{BufferArena, BufferArenaError, BufferIndex, PlanarBufferMut, PlanarBufferSpec};
pub use disjoint::{
    ARENA_SILENCE_BUFFER, ArenaStereoPair, ArenaStereoPlanes, DisjointArena, DisjointArenaError,
};
pub use latest_cell::{
    CellRead, CellReader, CellWriter, DirtyReader, DirtyWord, GATE_REVISION_MAX, GateKey, GatePin,
    LatestCell, LiveSnapshot, LiveStamp, LiveValuesSuperseded, RevisionGate, SharedAllocation,
    StampRefusal, StampRefused,
};
pub use observe::{
    ObservationPublisher, ObservationReader, ObservationSlot, ObservationState, ObservationView,
    ObservationWindow, observation_slot, observation_slot_retained_bytes,
};
pub use plan::{
    CarryOutcome, ExecutorBlockReport, HostAttachment, ObserverCarry, ObserverKind,
    PlanRevisionGate, PlanUnitEligibility, PrepareRenderPlan, PreparedPlanExecutor,
    PreparedProgram, PreparedRenderPlan, RenderEnvelope, RenderError, RenderIo, RenderReport,
    RenderTime, ResponseSnapshotAvailability, ResponseSnapshotCapture, ResponseSnapshotError,
    ResponseSnapshotOwnerInfo, ResponseSnapshotRequest, ResponseSnapshotSection,
    ResponseSnapshotSink, plan_revision_gate_retained_bytes, retained,
};
pub use plan_exchange::{
    PlanEpoch, PlanExchangeConfig, PlanExchangeError, PlanExchangeResourceReport, PlanPublisher,
    PlanReplacementReservation, PlanReplacementReservationError, PlanRetirer, PublishError,
    RealtimePlanOwner, RealtimeRenderReport, RealtimeResponseSnapshot, SwapOutcome,
    UnadoptedCandidate, Withdrawal, plan_exchange, plan_exchange_at_revision,
    plan_exchange_resource_report,
};
pub use seqlock::{
    MAXIMUM_READ_ATTEMPTS, PublishedWords, PublishedWordsBusy, PublishedWordsReader,
    PublishedWordsWriter, published_words, published_words_retained_bytes,
};
pub use spsc::{
    Consumer, InPlaceProducer, PlanAdoption, Producer, QueueEmpty, QueueFull, QueueGeneration,
    SpscError, SpscRetainedPayload, bounded_spsc, bounded_spsc_move, bounded_spsc_prefilled,
    bounded_spsc_retained_payload,
};
pub use watermark::{
    CandidateOutcome, OUTCOME_EXACT, OUTCOME_SUPERSEDED, OUTCOME_TRANSITION_FALLBACK,
    PlanWatermark, PlanWatermarkReader, WatermarkBusy,
};

#[cfg(test)]
mod tests {
    use super::spsc::MailboxCellState;
    use super::*;
    use crate::{QuantumFrames, SampleRateHz};
    use core::num::NonZeroUsize;

    struct SnapshotExecutor;

    impl PreparedPlanExecutor for SnapshotExecutor {
        fn retained_bytes(&self) -> u64 {
            0
        }
        fn copy_response_snapshot(
            &self,
            _track_id: &str,
            captured_sample: u64,
            sink: &mut dyn ResponseSnapshotSink,
        ) -> Result<u32, ResponseSnapshotError> {
            let section = ResponseSnapshotSection {
                id: 1,
                kind: 1,
                enabled: true,
                word_count: 1,
                words: [captured_sample as u32; 7],
            };
            sink.copy_owner(
                ResponseSnapshotOwnerInfo {
                    track_id: "track",
                    native_id: "miso.test",
                    stable_id: "owner",
                    rack: 2,
                    slot: 0,
                    kind: 1,
                    bypassed: false,
                    availability: ResponseSnapshotAvailability::Provided,
                },
                &[section],
                &[section],
            )?;
            Ok(1)
        }

        fn render(
            &mut self,
            _arena: &mut BufferArena,
            mut output: PlanarBufferMut<'_>,
            _time: RenderTime,
        ) -> Result<ExecutorBlockReport, RenderError> {
            output.plane_mut(0)?.fill(0.0);
            output.plane_mut(1)?.fill(0.0);
            Ok(ExecutorBlockReport::default())
        }
    }

    struct SnapshotSink {
        calls: usize,
        first_word: u32,
    }

    impl ResponseSnapshotSink for SnapshotSink {
        fn copy_owner(
            &mut self,
            _owner: ResponseSnapshotOwnerInfo<'_>,
            left: &[ResponseSnapshotSection],
            right: &[ResponseSnapshotSection],
        ) -> Result<(), ResponseSnapshotError> {
            assert_eq!(left, right);
            assert_eq!(left.len(), 1);
            self.calls += 1;
            self.first_word = left[0].words[0];
            Ok(())
        }
    }

    #[test]
    fn response_capture_uses_next_boundary_and_refuses_failed_render() {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let mut plan = PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id: 8,
                envelope,
                scratch: &[],
            },
            Box::new(SnapshotExecutor),
        )
        .expect("plan");
        let mut sink = SnapshotSink {
            calls: 0,
            first_word: u32::MAX,
        };
        let initial = plan
            .copy_response_snapshot(ResponseSnapshotRequest {
                track_id: "track",
                sink: &mut sink,
            })
            .expect("initial boundary");
        assert_eq!(initial.captured_sample, 0);
        assert_eq!(sink.first_word, 0);

        let mut samples = [0.0_f32; 8];
        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        plan.render_contiguous(RenderIo { output }, 0)
            .expect("render");
        let after = plan
            .copy_response_snapshot(ResponseSnapshotRequest {
                track_id: "track",
                sink: &mut sink,
            })
            .expect("next boundary");
        assert_eq!(after.captured_sample, 4);
        assert_eq!(sink.first_word, 4);

        let mut malformed = [0.0_f32; 4];
        let output = PlanarBufferMut::try_new(&mut malformed, 1, 4, 4).expect("shape");
        assert_eq!(
            plan.render_contiguous(RenderIo { output }, 4,),
            Err(RenderError::OutputShape)
        );
        assert_eq!(
            plan.copy_response_snapshot(ResponseSnapshotRequest {
                track_id: "track",
                sink: &mut sink,
            }),
            Err(ResponseSnapshotError::Owner)
        );
    }

    /// The plan owns the clock, and a contiguous render is the only caller that has to know the
    /// rule. Rendering the same block twice is a discontinuity, and the error names the sample the
    /// plan is waiting for.
    #[test]
    fn render_contiguous_rejects_stale_and_accepts_next() {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            output_channels: NonZeroUsize::new(2).expect("two"),
        };
        let mut plan = PreparedRenderPlan::prepare(PrepareRenderPlan {
            plan_id: 7,
            envelope,
            scratch: &[],
        })
        .expect("plan");
        assert_eq!(plan.next_absolute_sample(), 0);

        let mut samples = [0.0_f32; 8];
        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        let report = plan
            .render_contiguous(RenderIo { output }, 0)
            .expect("first block");
        assert_eq!(report.next_absolute_sample, 4);
        assert_eq!(plan.next_absolute_sample(), 4);

        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        assert_eq!(
            plan.render_contiguous(RenderIo { output }, 0,),
            Err(RenderError::TimeDiscontinuity { expected: 4 })
        );

        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        plan.render_contiguous(RenderIo { output }, 4)
            .expect("second block");
        assert_eq!(plan.next_absolute_sample(), 8);
    }

    /// A silent executor that records the source-read offset its plan hands it.
    struct OffsetExecutor {
        offset: std::sync::Arc<std::sync::atomic::AtomicU64>,
    }

    impl PreparedPlanExecutor for OffsetExecutor {
        fn retained_bytes(&self) -> u64 {
            crate::realtime::retained::arc(&self.offset)
        }
        fn set_source_read_offset(&mut self, offset: u64) {
            self.offset
                .store(offset, std::sync::atomic::Ordering::Relaxed);
        }

        fn render(
            &mut self,
            _arena: &mut BufferArena,
            mut output: PlanarBufferMut<'_>,
            _time: RenderTime,
        ) -> Result<ExecutorBlockReport, RenderError> {
            output.plane_mut(0)?.fill(0.0);
            Ok(ExecutorBlockReport::default())
        }
    }

    /// #1396 gate 4: through 8 blocks and two swaps (offset 384, then 0), after each block the
    /// publisher's render clock is the active plan's next sample and its source-read clock is that
    /// plus the active plan's offset; before the first block both read the initial plan's.
    ///
    /// Test value: red if render stores the outgoing plan's offset after a swap, stores the clocks
    /// before the block renders, or never stores them; or if a plan does not forward its offset
    /// to its executor.
    #[test]
    fn the_publisher_reads_both_clocks_of_the_active_plan_after_each_block() {
        const QUANTUM: u64 = 128;
        let offset_plan = |id: u64, offset: u64| {
            let seen = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(u64::MAX));
            let mut plan = PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: id,
                    envelope: RenderEnvelope {
                        sample_rate: SampleRateHz(48_000),
                        quantum: QuantumFrames(QUANTUM as u32),
                        output_channels: NonZeroUsize::new(1).expect("one"),
                    },
                    scratch: &[],
                },
                Box::new(OffsetExecutor {
                    offset: std::sync::Arc::clone(&seen),
                }),
            )
            .expect("plan");
            plan.set_source_read_offset(offset);
            assert_eq!(plan.source_read_offset(), offset);
            assert_eq!(seen.load(std::sync::atomic::Ordering::Relaxed), offset);
            plan
        };
        let mut initial = offset_plan(1, 0);
        initial.set_source_read_offset(QUANTUM);
        let (mut publisher, mut realtime, mut retirer) = plan_exchange(
            initial,
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(2).expect("two"),
            },
        )
        .expect("exchange");
        assert_eq!(publisher.render_clock(), 0);
        assert_eq!(publisher.source_read_clock(), QUANTUM);

        let mut active_offset = QUANTUM;
        let mut samples = [0.0_f32; QUANTUM as usize];
        for block in 0..8_u64 {
            let swap = match block {
                2 => Some(3 * QUANTUM),
                5 => Some(0),
                _ => None,
            };
            if let Some(offset) = swap {
                let _ = publisher
                    .publish(offset_plan(block + 2, offset), 0)
                    .map_err(|_| "publish")
                    .expect("publish");
            }
            let output =
                PlanarBufferMut::try_new(&mut samples, 1, QUANTUM as usize, QUANTUM as usize)
                    .expect("output");
            let report = realtime
                .render_contiguous(RenderIo { output }, block * QUANTUM)
                .expect("block");
            if let Some(offset) = swap {
                assert_eq!(report.swap, SwapOutcome::Applied);
                active_offset = offset;
            }
            let next = (block + 1) * QUANTUM;
            assert_eq!(publisher.render_clock(), next, "block {block}");
            assert_eq!(
                publisher.source_read_clock(),
                next + active_offset,
                "block {block}"
            );
            while retirer.try_reclaim().is_ok() {}
        }
    }

    #[test]
    fn arena_is_fixed_and_disjoint() {
        let spec = PlanarBufferSpec {
            channels: NonZeroUsize::new(2).expect("two"),
            frame_capacity: QuantumFrames(4),
        };
        let mut arena = BufferArena::try_new(&[spec]).expect("arena");
        arena.plane_mut(BufferIndex(0), 0).expect("left")[0] = 1.0;
        assert_eq!(arena.plane(BufferIndex(0), 1).expect("right")[0], 0.0);
        assert!(matches!(
            BufferArena::try_new(&[PlanarBufferSpec {
                channels: NonZeroUsize::new(1).expect("one"),
                frame_capacity: QuantumFrames(0)
            }]),
            Err(BufferArenaError::ZeroFrames)
        ));
    }

    #[test]
    fn plan_exchange_resource_projection_covers_mailbox_and_retirement_and_checks_overflow() {
        let config = PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let report = plan_exchange_resource_report(config).expect("projection");
        assert!(report.retained_payload_bytes > report.largest_allocation_bytes);
        assert!(report.largest_allocation_bytes > 0);
        assert_eq!(
            plan_exchange_resource_report(PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
            }),
            Err(SpscError::CapacityOverflow)
        );
    }

    fn prepared(id: u64) -> PreparedRenderPlan {
        PreparedRenderPlan::prepare(PrepareRenderPlan {
            plan_id: id,
            envelope: RenderEnvelope {
                sample_rate: SampleRateHz(48_000),
                quantum: QuantumFrames(2),
                output_channels: NonZeroUsize::new(1).expect("one"),
            },
            scratch: &[],
        })
        .expect("plan")
    }
    #[test]
    fn reserved_replacement_preowns_publication_epoch_and_retirement_credit() {
        let config = PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let reservation = publisher
            .reserve_replacement(prepared(2), PlanAdoption::Next)
            .expect("complete reservation");
        assert_eq!(reservation.epoch(), PlanEpoch(1));
        assert_eq!(reservation.commit(0), PlanEpoch(1));
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert_eq!(realtime.active_plan_id(), 2);

        assert!(matches!(
            publisher.reserve_replacement(prepared(3), PlanAdoption::Next),
            Err(PlanReplacementReservationError::RetirementFull(returned))
                if returned.program().plan_id() == 3
        ));
        let retired = retirer.try_reclaim().expect("reserved retirement");
        assert_eq!(retired.0, PlanEpoch(0));
        let reservation = publisher
            .reserve_replacement(prepared(3), PlanAdoption::Next)
            .expect("reclaimed credit");
        assert_eq!(reservation.epoch(), PlanEpoch(2));
        reservation.commit(0);
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::Applied);
        assert_eq!(realtime.active_plan_id(), 3);
    }

    #[test]
    fn replacement_cancel_releases_both_credits_without_consuming_epoch() {
        let config = PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let (mut publisher, mut realtime, _retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let returned = publisher
            .reserve_replacement(prepared(2), PlanAdoption::Next)
            .expect("reservation")
            .cancel();
        assert_eq!(returned.program().plan_id(), 2);
        let replacement = publisher
            .reserve_replacement(returned, PlanAdoption::Next)
            .expect("credits released");
        assert_eq!(replacement.epoch(), PlanEpoch(1));
        drop(replacement);
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::None);

        let replacement = publisher
            .reserve_replacement(prepared(3), PlanAdoption::Next)
            .expect("drop released credits");
        assert_eq!(replacement.commit(0), PlanEpoch(1));
        assert_eq!(render_once(&mut realtime, 2).active_epoch, PlanEpoch(1));
    }

    #[test]
    fn replacement_reservation_freezes_failure_precedence_and_serial_order() {
        let config = PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(2).expect("two"),
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let incompatible = PreparedRenderPlan::prepare(PrepareRenderPlan {
            plan_id: 99,
            envelope: RenderEnvelope {
                sample_rate: SampleRateHz(44_100),
                quantum: QuantumFrames(2),
                output_channels: NonZeroUsize::new(1).expect("one"),
            },
            scratch: &[],
        })
        .expect("other envelope");
        assert!(matches!(
            publisher.reserve_replacement(incompatible, PlanAdoption::Next),
            Err(PlanReplacementReservationError::Incompatible(returned))
                if returned.program().plan_id() == 99
        ));

        publisher
            .reserve_replacement(prepared(2), PlanAdoption::Next)
            .expect("first")
            .commit(0);
        // The mailbox holds one published candidate: a second waits for render's claim, even
        // with a retirement credit free.
        assert!(matches!(
            publisher.reserve_replacement(prepared(3), PlanAdoption::Next),
            Err(PlanReplacementReservationError::PublicationFull(returned))
                if returned.program().plan_id() == 3
        ));
        assert_eq!(publisher.free_retirement_credits(), 1);
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        publisher
            .reserve_replacement(prepared(3), PlanAdoption::Next)
            .expect("second")
            .commit(0);
        assert_eq!(render_once(&mut realtime, 2).render.plan_id, 3);
        assert_eq!(retirer.try_reclaim().expect("initial").0, PlanEpoch(0));
        assert_eq!(retirer.try_reclaim().expect("second").0, PlanEpoch(1));
    }

    fn one_retirement() -> PlanExchangeConfig {
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        }
    }

    fn withdrawn(publisher: &mut PlanPublisher) -> UnadoptedCandidate {
        match publisher.withdraw() {
            Withdrawal::Withdrawn(candidate) => candidate,
            other => panic!("expected a withdrawn candidate, got {other:?}"),
        }
    }

    /// #1343 gate 1: a candidate withdrawn before any render comes back whole, render then applies
    /// nothing, and republished it is adopted at the next block with its own epoch and credit.
    #[test]
    fn withdrawn_candidate_returns_whole_and_republishes_with_its_epoch() {
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), one_retirement()).expect("exchange");
        assert_eq!(publisher.free_retirement_credits(), 1);
        assert!(matches!(
            publisher.publish(prepared(2), 0),
            Ok(PlanEpoch(1))
        ));
        assert_eq!(publisher.free_retirement_credits(), 0);

        let candidate = withdrawn(&mut publisher);
        assert_eq!((candidate.epoch(), candidate.plan_id()), (PlanEpoch(1), 2));
        assert_eq!(
            publisher.mailbox_cell_states(),
            [MailboxCellState::Active, MailboxCellState::Empty]
        );
        // The withdrawn candidate keeps its credit.
        assert_eq!(publisher.free_retirement_credits(), 0);
        for sample in [0, 2] {
            let report = render_once(&mut realtime, sample);
            assert_eq!(
                (report.swap, report.active_epoch, report.render.plan_id),
                (SwapOutcome::None, PlanEpoch(0), 1)
            );
        }

        assert_eq!(publisher.republish(candidate), PlanEpoch(1));
        let report = render_once(&mut realtime, 4);
        assert_eq!(
            (report.swap, report.active_epoch, report.render.plan_id),
            (SwapOutcome::Applied, PlanEpoch(1), 2)
        );
        assert!(matches!(publisher.withdraw(), Withdrawal::Taken));
        // The credit moved with the displaced plan into retirement and returns at reclamation.
        assert_eq!(publisher.free_retirement_credits(), 0);
        assert_eq!(retirer.try_reclaim().expect("plan one").0, PlanEpoch(0));
        assert_eq!(publisher.free_retirement_credits(), 1);
        // The next publication takes the next epoch: republication consumed none.
        assert!(matches!(
            publisher.publish(prepared(3), 0),
            Ok(PlanEpoch(2))
        ));
    }

    /// #1343 gate 1: once render has claimed the candidate, withdrawal reports `Taken` and the
    /// candidate is the active plan.
    #[test]
    fn withdrawal_after_render_claimed_reports_taken() {
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), one_retirement()).expect("exchange");
        assert!(publisher.publish(prepared(2), 0).is_ok());
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert!(matches!(publisher.withdraw(), Withdrawal::Taken));
        assert_eq!(
            (realtime.active_epoch(), realtime.active_plan_id()),
            (PlanEpoch(1), 2)
        );
        assert!(matches!(publisher.withdraw(), Withdrawal::Nothing));
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::None);
        assert_eq!(publisher.free_retirement_credits(), 0);
        let _ = retirer.try_reclaim().expect("plan one");
        assert_eq!(publisher.free_retirement_credits(), 1);
    }

    /// #1343 gate 1: with nothing published, withdrawal reports `Nothing`; a withdrawn candidate
    /// dropped on the control thread returns its credit and is never adopted.
    #[test]
    fn withdrawal_with_nothing_published_reports_nothing() {
        let (mut publisher, mut realtime, _retirer) =
            plan_exchange(prepared(1), one_retirement()).expect("exchange");
        assert!(matches!(publisher.withdraw(), Withdrawal::Nothing));
        assert!(publisher.publish(prepared(2), 0).is_ok());
        let candidate = withdrawn(&mut publisher);
        assert!(matches!(publisher.withdraw(), Withdrawal::Nothing));
        drop(candidate);
        assert_eq!(publisher.free_retirement_credits(), 1);
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::None);
        assert_eq!(realtime.active_plan_id(), 1);
        let plan = withdrawn_then_given_up(&mut publisher);
        assert_eq!(plan.program().plan_id(), 3);
        assert_eq!(publisher.free_retirement_credits(), 1);
    }

    fn withdrawn_then_given_up(publisher: &mut PlanPublisher) -> PreparedRenderPlan {
        assert!(matches!(
            publisher.publish(prepared(3), 0),
            Ok(PlanEpoch(2))
        ));
        withdrawn(publisher).into_plan()
    }

    /// #1343 gate 1: a second publication lands in the cell the first claim left `Empty`, never in
    /// the `Active` one.
    #[test]
    fn second_publication_lands_in_the_cell_the_claim_left_empty() {
        use MailboxCellState::{Active, Empty, Full};
        let (mut publisher, mut realtime, mut retirer) = plan_exchange(
            prepared(1),
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(2).expect("two"),
            },
        )
        .expect("exchange");
        assert_eq!(publisher.mailbox_cell_states(), [Active, Empty]);
        assert!(publisher.publish(prepared(2), 0).is_ok());
        assert_eq!(publisher.mailbox_cell_states(), [Active, Full]);
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        assert_eq!(publisher.mailbox_cell_states(), [Empty, Active]);
        assert!(publisher.publish(prepared(3), 0).is_ok());
        assert_eq!(publisher.mailbox_cell_states(), [Full, Active]);
        let report = render_once(&mut realtime, 2);
        assert_eq!(
            (report.swap, report.active_epoch, report.render.plan_id),
            (SwapOutcome::Applied, PlanEpoch(2), 3)
        );
        assert_eq!(publisher.mailbox_cell_states(), [Active, Empty]);
        assert_eq!(publisher.free_retirement_credits(), 0);
        assert_eq!(retirer.try_reclaim().expect("plan one").0, PlanEpoch(0));
        assert_eq!(retirer.try_reclaim().expect("plan two").0, PlanEpoch(1));
        assert_eq!(publisher.free_retirement_credits(), 2);
    }

    /// #1343 gate 1 and gate 4's replacement: a publication without a free retirement credit fails
    /// `Full` on the control side and publishes nothing, so render never holds a candidate it
    /// cannot retire for.
    #[test]
    fn publication_without_a_credit_fails_full() {
        use MailboxCellState::{Active, Empty};
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), one_retirement()).expect("exchange");
        assert!(publisher.publish(prepared(2), 0).is_ok());
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert_eq!(publisher.free_retirement_credits(), 0);
        let returned = match publisher.publish(prepared(3), 0) {
            Err(PublishError::Full(returned)) => returned,
            _ => panic!("a credit-less publication must fail Full"),
        };
        assert_eq!(returned.program().plan_id(), 3);
        assert_eq!(publisher.mailbox_cell_states(), [Empty, Active]);
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::None);
        assert!(matches!(publisher.withdraw(), Withdrawal::Taken));

        let _ = retirer.try_reclaim().expect("plan one");
        assert_eq!(publisher.free_retirement_credits(), 1);
        assert!(matches!(publisher.publish(returned, 0), Ok(PlanEpoch(2))));
        let report = render_once(&mut realtime, 4);
        assert_eq!(
            (report.swap, report.active_epoch, report.render.plan_id),
            (SwapOutcome::Applied, PlanEpoch(2), 3)
        );
    }

    fn render_once(owner: &mut RealtimePlanOwner, sample: u64) -> RealtimeRenderReport {
        let mut output = [1.0_f32; 2];
        let io = RenderIo {
            output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
        };
        let report = owner
            .render(
                io,
                RenderTime {
                    absolute_sample: sample,
                },
            )
            .expect("render");
        assert_eq!(output, [0.0, 0.0]);
        report
    }

    /// #1314, #1502: the applied-revision watermark, read the way any thread reads it, and the
    /// per-plan revision gate that feeds it.
    mod watermark {
        use super::*;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

        const QUANTUM: u64 = 2;

        fn read(publisher: &PlanPublisher) -> PlanWatermark {
            publisher
                .watermark_reader()
                .read()
                .expect("a read with no render in flight is never busy")
        }

        /// `(revision, first sample, flags)`.
        fn level(publisher: &PlanPublisher) -> (u64, u64, u64) {
            let watermark = read(publisher);
            (
                watermark.revision,
                watermark.first_sample,
                watermark.outcome_flags,
            )
        }

        fn block(owner: &mut RealtimePlanOwner, index: u64) -> RealtimeRenderReport {
            render_once(owner, index * QUANTUM)
        }

        /// A plan with its gate handle taken, as the control plane takes it before publication.
        fn plan_with_gate(id: u64) -> (PreparedRenderPlan, PlanRevisionGate) {
            let mut plan = prepared(id);
            let gate = plan.take_revision_gate().expect("a fresh plan's handle");
            (plan, gate)
        }

        type Exchange = (
            PlanRevisionGate,
            PlanPublisher,
            RealtimePlanOwner,
            PlanRetirer,
        );

        /// The exchange with `plan` (and its handle) at `revision`; the handle stays with the
        /// caller.
        fn exchange_with(
            plan: PreparedRenderPlan,
            mut gate: PlanRevisionGate,
            revision: u64,
        ) -> Exchange {
            let (publisher, realtime, retirer) =
                plan_exchange_at_revision(plan, &mut gate, revision, one_retirement())
                    .expect("exchange");
            (gate, publisher, realtime, retirer)
        }

        /// The exchange with plan 1 at `revision`.
        fn exchange_at(revision: u64) -> Exchange {
            let (plan, gate) = plan_with_gate(1);
            exchange_with(plan, gate, revision)
        }

        /// Publish plan `id` carrying `revision`: the revision goes on the candidate's own gate
        /// before the commit (#1502 D6).
        fn publish_at(publisher: &mut PlanPublisher, id: u64, revision: u64) -> PlanRevisionGate {
            let (plan, mut gate) = plan_with_gate(id);
            gate.publish(revision);
            publisher
                .reserve_replacement(plan, PlanAdoption::Next)
                .expect("room")
                .commit(revision);
            gate
        }

        /// #1314 gate 1 on plan gates: a revision completes when render adopts the plan that
        /// carries it, never at publication, and a withdrawn candidate takes its revision with it.
        #[test]
        fn a_revision_completes_at_adoption_not_at_publication() {
            let (_gate, mut publisher, mut realtime, mut retirer) = exchange_at(1);
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 0);

            let _b = publish_at(&mut publisher, 2, 7);
            assert_eq!(
                publisher.mailbox_cell_states(),
                [MailboxCellState::Active, MailboxCellState::Full]
            );
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            assert_eq!(block(&mut realtime, 0).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (7, 0, OUTCOME_EXACT));
            let _ = retirer.try_reclaim().expect("plan one");

            // C lands in cell 0, plan one's old cell, and leaves it again with its revision.
            let _c = publish_at(&mut publisher, 3, 8);
            assert_eq!(
                publisher.mailbox_cell_states(),
                [MailboxCellState::Full, MailboxCellState::Active]
            );
            let candidate = withdrawn(&mut publisher);
            for index in 1..=2 {
                assert_eq!(block(&mut realtime, index).swap, SwapOutcome::None);
                assert_eq!(level(&publisher), (7, 0, OUTCOME_EXACT), "block {index}");
            }

            assert_eq!(publisher.republish(candidate), PlanEpoch(2));
            assert_eq!(block(&mut realtime, 3).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 7);
            let _ = retirer.try_reclaim().expect("plan C");

            let _d = publish_at(&mut publisher, 4, 9);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
        }

        /// #1349 gate 2: a block's report carries the watermark exactly when the block advanced
        /// it, with the values render just published, and carries nothing on any other block.
        #[test]
        fn the_render_report_carries_the_watermark_only_on_the_advancing_block() {
            let (_gate, mut publisher, mut realtime, _retirer) = exchange_at(1);
            assert_eq!(block(&mut realtime, 0).watermark, None);
            assert_eq!(block(&mut realtime, 1).watermark, None);

            let mut live = publish_at(&mut publisher, 2, 5);
            let adopted = block(&mut realtime, 2);
            assert_eq!(adopted.swap, SwapOutcome::Applied);
            assert_eq!(adopted.watermark, Some(read(&publisher)));
            assert_eq!(
                adopted
                    .watermark
                    .map(|w| (w.revision, w.first_sample, w.outcome_flags)),
                Some((5, 2 * QUANTUM, OUTCOME_EXACT))
            );
            assert_eq!(block(&mut realtime, 3).watermark, None);

            live.publish(6);
            let edited = block(&mut realtime, 4);
            assert_eq!(edited.swap, SwapOutcome::None);
            assert_eq!(edited.watermark, Some(read(&publisher)));
            assert_eq!(
                edited.watermark.map(|w| (w.revision, w.first_sample)),
                Some((6, 4 * QUANTUM))
            );
            assert_eq!(block(&mut realtime, 5).watermark, None);
        }

        /// #1314 gate 4 on plan gates: a candidate's `superseded` count and its outcome complete
        /// once, at the first advance after its claim, and a withdrawn candidate takes its outcome
        /// with it.
        #[test]
        fn superseded_and_fallback_revisions_complete_once_per_adoption() {
            let (_gate, mut publisher, mut realtime, mut retirer) = exchange_at(1);
            let a = read(&publisher).revision;
            let (plan, mut b_gate) = plan_with_gate(2);
            b_gate.publish(a + 3);
            let mut reservation = publisher
                .reserve_replacement(plan, PlanAdoption::Next)
                .expect("room");
            reservation.set_superseded(2);
            reservation.commit(a + 3);
            assert_eq!(block(&mut realtime, 0).swap, SwapOutcome::Applied);
            let adopted = read(&publisher);
            assert_eq!(
                (adopted.revision, adopted.outcome_flags),
                (a + 3, OUTCOME_EXACT | OUTCOME_SUPERSEDED)
            );
            assert_eq!(
                (
                    adopted.superseded,
                    adopted.exact,
                    adopted.transition_fallback
                ),
                (2, 1, 0)
            );
            let _ = retirer.try_reclaim().expect("plan one");

            // A live edit on the adopted plan.
            b_gate.publish(a + 4);
            let _ = block(&mut realtime, 1);
            let live = read(&publisher);
            assert_eq!((live.revision, live.outcome_flags), (a + 4, OUTCOME_EXACT));
            assert_eq!(
                (live.superseded, live.exact, live.transition_fallback),
                (2, 2, 0)
            );

            let b = live.revision;
            let (plan, mut c_gate) = plan_with_gate(3);
            c_gate.publish(b + 2);
            let mut reservation = publisher
                .reserve_replacement(plan, PlanAdoption::Next)
                .expect("room");
            reservation.set_outcome(CandidateOutcome::TransitionFallback);
            reservation.commit(b + 2);
            let candidate = withdrawn(&mut publisher);
            let _ = publisher.republish(candidate);
            assert_eq!(block(&mut realtime, 2).swap, SwapOutcome::Applied);
            let fallback = read(&publisher);
            assert_eq!(
                (
                    fallback.revision,
                    fallback.first_sample,
                    fallback.outcome_flags
                ),
                (b + 2, 2 * QUANTUM, OUTCOME_TRANSITION_FALLBACK)
            );
            assert_eq!(
                (
                    fallback.superseded,
                    fallback.exact,
                    fallback.transition_fallback
                ),
                (2, 2, 2)
            );

            c_gate.publish(b + 3);
            let _ = block(&mut realtime, 3);
            let live = read(&publisher);
            assert_eq!((live.revision, live.outcome_flags), (b + 3, OUTCOME_EXACT));
            assert_eq!(
                (live.superseded, live.exact, live.transition_fallback),
                (2, 3, 2)
            );
        }

        /// #1314 gate 8 on plan gates: a revision published on the gate of a candidate that is
        /// pending, held by the control thread or published, leaves the watermark unchanged until
        /// that candidate is republished and adopted.
        #[test]
        fn a_revision_follows_a_held_or_published_candidate() {
            let (_gate, mut publisher, mut realtime, mut retirer) = exchange_at(1);
            let mut a_gate = publish_at(&mut publisher, 2, 7);
            let held = withdrawn(&mut publisher);
            // A model-only commit while control holds A: it publishes on A's gate.
            a_gate.publish(8);
            for index in 0..=2 {
                assert_eq!(block(&mut realtime, index).swap, SwapOutcome::None);
                assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT), "block {index}");
            }
            let _ = publisher.republish(held);
            assert_eq!(block(&mut realtime, 3).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
            let _ = retirer.try_reclaim().expect("plan one");

            let mut b_gate = publish_at(&mut publisher, 3, 9);
            // A model-only commit while B is published: B's gate again.
            b_gate.publish(10);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
            assert_eq!(block(&mut realtime, 4).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (10, 4 * QUANTUM, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 9);
        }

        /// An executor that records the snapshot `begin_live_block` hands it, and fails a block
        /// on demand. Atomics only: it runs inside the render scope.
        #[derive(Clone, Default)]
        struct SnapshotProbe {
            last: Arc<AtomicU64>,
            calls: Arc<AtomicU64>,
            fail: Arc<AtomicBool>,
        }

        impl SnapshotProbe {
            fn last(&self) -> u64 {
                self.last.load(Ordering::Relaxed)
            }
            fn calls(&self) -> u64 {
                self.calls.load(Ordering::Relaxed)
            }
        }

        struct Snapshotting(SnapshotProbe);

        impl PreparedPlanExecutor for Snapshotting {
            fn retained_bytes(&self) -> u64 {
                crate::realtime::retained::arc(&self.0.last)
                    + crate::realtime::retained::arc(&self.0.calls)
                    + crate::realtime::retained::arc(&self.0.fail)
            }
            fn begin_live_block(&mut self, snapshot: LiveSnapshot) {
                self.0.last.store(snapshot.revision(), Ordering::Relaxed);
                self.0.calls.fetch_add(1, Ordering::Relaxed);
            }

            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                if self.0.fail.load(Ordering::Relaxed) {
                    return Err(RenderError::InvalidEnvelope);
                }
                output.plane_mut(0)?.fill(0.0);
                Ok(ExecutorBlockReport::default())
            }
        }

        fn probed(id: u64, probe: &SnapshotProbe) -> (PreparedRenderPlan, PlanRevisionGate) {
            let mut plan = PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: id,
                    envelope: prepared(id).envelope(),
                    scratch: &[],
                },
                Box::new(Snapshotting(probe.clone())),
            )
            .expect("plan");
            let gate = plan.take_revision_gate().expect("a fresh plan's handle");
            (plan, gate)
        }

        /// #1502 gate 1: each block's snapshot is the revision published on the gate of the plan
        /// that renders it. A revision published on the running plan's gate after a candidate was
        /// published is not the candidate's `S` at its adoption block, and the candidate's is not
        /// the running plan's before it.
        #[test]
        fn each_block_takes_its_snapshot_from_its_own_plans_gate() {
            let (first, second) = (SnapshotProbe::default(), SnapshotProbe::default());
            let (plan, gate) = probed(1, &first);
            let (mut first_gate, mut publisher, mut realtime, _retirer) =
                exchange_with(plan, gate, 1);

            let _ = block(&mut realtime, 0);
            assert_eq!(first.last(), 1);
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            first_gate.publish(2);
            let _ = block(&mut realtime, 1);
            assert_eq!(first.last(), 2);
            assert_eq!(level(&publisher), (2, QUANTUM, OUTCOME_EXACT));

            let (plan, mut second_gate) = probed(2, &second);
            second_gate.publish(5);
            publisher
                .reserve_replacement(plan, PlanAdoption::Next)
                .expect("room")
                .commit(5);
            // The running plan's gate moves while B is published; B's block must not see it.
            first_gate.publish(3);
            assert_eq!(block(&mut realtime, 2).swap, SwapOutcome::Applied);
            assert_eq!((second.last(), second.calls()), (5, 1));
            assert_eq!(first.calls(), 2, "the displaced plan rendered the block");
            assert_eq!(level(&publisher), (5, 2 * QUANTUM, OUTCOME_EXACT));
            second_gate.publish(6);
            let _ = block(&mut realtime, 3);
            assert_eq!(second.last(), 6);
            assert_eq!(level(&publisher), (6, 3 * QUANTUM, OUTCOME_EXACT));
        }

        /// #1502 gate 1, the direct path: `PreparedRenderPlan::render` and `render_contiguous`
        /// read their own gate too, with and without an executor, and no render path uses
        /// `LiveSnapshot::ALL`.
        #[test]
        fn the_direct_render_paths_take_their_own_gates_snapshot() {
            let probe = SnapshotProbe::default();
            let (mut plan, mut gate) = probed(1, &probe);
            gate.publish(4);
            let mut output = [1.0_f32; 2];
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let report = plan
                .render(io, RenderTime { absolute_sample: 0 })
                .expect("render");
            assert_eq!(report.live_snapshot.revision(), 4);
            assert_eq!(probe.last(), 4);

            gate.publish(5);
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let report = plan.render_contiguous(io, QUANTUM).expect("render");
            assert_eq!(report.live_snapshot.revision(), 5);
            assert_eq!(probe.last(), 5);

            // No executor: the snapshot is still the plan's own gate's.
            let (mut bare, mut bare_gate) = plan_with_gate(2);
            bare_gate.publish(3);
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let report = bare
                .render(io, RenderTime { absolute_sample: 0 })
                .expect("render");
            assert_eq!(report.live_snapshot.revision(), 3);
        }

        /// A probe executor over one cell: each block drains the cell the way a stage drains its
        /// lane, and publishes the newest applied value as `(revision << 32 | publication) + 1` (0 for
        /// none). Atomics only: it runs inside the render scope.
        struct CellProbe {
            cell: Arc<LatestCell<2>>,
            dirty: Arc<DirtyWord>,
            reader: CellReader,
            stage: DirtyReader,
            applied: Arc<AtomicU64>,
        }

        impl PreparedPlanExecutor for CellProbe {
            fn retained_bytes(&self) -> u64 {
                crate::realtime::retained::arc(&self.cell)
                    + crate::realtime::retained::arc(&self.dirty)
                    + crate::realtime::retained::arc(&self.applied)
            }
            fn begin_live_block(&mut self, snapshot: LiveSnapshot) {
                let pending = self.stage.pending(&self.dirty);
                if pending & 1 == 0 {
                    return;
                }
                if let CellRead::Applied { words, .. } =
                    self.reader.read(&self.cell, snapshot, &mut self.stage, 0)
                {
                    self.applied.store(
                        (u64::from(words[0]) << 32 | u64::from(words[1])) + 1,
                        Ordering::Relaxed,
                    );
                }
            }

            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(ExecutorBlockReport::default())
            }
        }

        /// What control keeps of the newest plan: its handle, cell and writer.
        struct Newest {
            gate: PlanRevisionGate,
            cell: Arc<LatestCell<2>>,
            dirty: Arc<DirtyWord>,
            writer: CellWriter<2>,
        }

        /// A probe plan whose cell holds the model value of `revision`, stamped on the plan's held
        /// gate as a successor's carry retargets are, with its handle taken and published.
        fn probe_plan_at(
            id: u64,
            revision: u64,
            applied: &Arc<AtomicU64>,
        ) -> (PreparedRenderPlan, Newest) {
            let cell = Arc::new(LatestCell::<2>::new());
            let dirty = Arc::new(DirtyWord::new());
            let mut plan = PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: id,
                    envelope: prepared(id).envelope(),
                    scratch: &[],
                },
                Box::new(CellProbe {
                    cell: Arc::clone(&cell),
                    dirty: Arc::clone(&dirty),
                    reader: CellReader::new(),
                    stage: DirtyReader::new(),
                    applied: Arc::clone(applied),
                }),
            )
            .expect("plan");
            let mut writer = CellWriter::<2>::new();
            let stamp = plan
                .stamp_held_gate(revision)
                .expect("held")
                .expect("a revision above 0");
            writer.write(&cell, &dirty, 0, stamp, [revision as u32, 1]);
            let gate = plan.take_revision_gate().expect("a fresh plan's handle");
            (
                plan,
                Newest {
                    gate,
                    cell,
                    dirty,
                    writer,
                },
            )
        }

        fn seeded(state: &mut u64) -> u64 {
            *state ^= *state << 13;
            *state ^= *state >> 7;
            *state ^= *state << 17;
            *state
        }

        /// #1603 gate 4: with monitor writes under a revision, model commits and successor plans
        /// racing a rendering plan on another thread, the watermark is never above the revision
        /// control had begun to publish, the block that reports a revision applies exactly that
        /// revision's newest value, the watermark counts one exact revision per model revision,
        /// and the last block reads the last value written.
        #[test]
        fn the_watermark_counts_model_revisions_only_under_monitor_writes() {
            for seed in 1..=20_u64 {
                monitor_run(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1);
            }
        }

        fn monitor_run(mut rng: u64) {
            use std::sync::atomic::AtomicBool;
            const STEPS: u32 = 10_000;
            const FIRST: u64 = 1;
            let applied = Arc::new(AtomicU64::new(0));
            let announced = Arc::new(AtomicU64::new(FIRST));
            let done = AtomicBool::new(false);
            let finished = AtomicBool::new(false);
            let final_word = AtomicU64::new(0);
            let (plan, mut newest) = probe_plan_at(1, FIRST, &applied);
            let (mut publisher, mut realtime, mut retirer) =
                plan_exchange_at_revision(plan, &mut newest.gate, FIRST, one_retirement())
                    .expect("exchange");
            let reader = publisher.watermark_reader();
            std::thread::scope(|scope| {
                scope.spawn(|| {
                    let mut sample = 0_u64;
                    let mut reported = FIRST;
                    let mut last_applied = 0_u64;
                    let mut blocks = 0_u64;
                    loop {
                        // Read `done` first: a block after it sees every write.
                        let stop = done.load(Ordering::Acquire);
                        let report = realtime
                            .render(
                                RenderIo {
                                    output: PlanarBufferMut::try_new(&mut [1.0_f32; 2], 1, 2, 2)
                                        .expect("output"),
                                },
                                RenderTime {
                                    absolute_sample: sample,
                                },
                            )
                            .expect("render");
                        let snapshot = report.render.live_snapshot;
                        let watermark = reader.read().expect("no render in flight");
                        assert_eq!(watermark.revision, snapshot.revision());
                        assert!(
                            watermark.revision <= announced.load(Ordering::Relaxed),
                            "watermark {} above the revision control began to publish",
                            watermark.revision
                        );
                        let word = applied.load(Ordering::Relaxed);
                        assert!(word > 0, "the plan's first block applies its model value");
                        let (value_revision, value_publication) =
                            ((word - 1) >> 32, (word - 1) & 0xffff_ffff);
                        assert_eq!(
                            value_revision,
                            snapshot.revision(),
                            "the block reports the revision whose value it applied"
                        );
                        assert_eq!(
                            value_publication,
                            snapshot.publication(),
                            "every publication writes the cell, so the block applied its own"
                        );
                        assert!(word >= last_applied, "an applied value went backwards");
                        last_applied = word;
                        if watermark.revision > reported {
                            assert_eq!(watermark.first_sample, sample);
                            reported = watermark.revision;
                        }
                        assert_eq!(
                            watermark.exact,
                            watermark.revision - FIRST,
                            "one exact revision per model revision"
                        );
                        sample += QUANTUM;
                        blocks += 1;
                        assert!(blocks < 200_000_000, "render never converged");
                        if stop && word == final_word.load(Ordering::Relaxed) {
                            break;
                        }
                    }
                    finished.store(true, Ordering::Release);
                });

                let mut revision = FIRST;
                let mut publication = 1_u64;
                let mut plan_id = 1_u64;
                for step in 0..STEPS {
                    let _ = retirer.try_reclaim();
                    match seeded(&mut rng) % 100 {
                        0..=39 => {
                            announced.fetch_max(revision + 1, Ordering::Relaxed);
                            let stamp = newest.gate.stamp(revision + 1).expect("next revision");
                            let key = stamp.key();
                            newest.writer.write(
                                &newest.cell,
                                &newest.dirty,
                                0,
                                stamp,
                                [(revision + 1) as u32, key.publication() as u32],
                            );
                            newest.gate.publish_key(key);
                            revision += 1;
                            publication = key.publication();
                        }
                        40..=84 => {
                            let stamp = newest
                                .gate
                                .stamp_monitor(revision)
                                .expect("a monitor stamp");
                            let key = stamp.key();
                            assert_eq!(
                                (key.revision(), key.publication()),
                                (revision, publication + 1),
                                "a monitor write spends a publication and no revision"
                            );
                            newest.writer.write(
                                &newest.cell,
                                &newest.dirty,
                                0,
                                stamp,
                                [revision as u32, key.publication() as u32],
                            );
                            newest.gate.publish_key(key);
                            publication = key.publication();
                        }
                        _ => {
                            announced.fetch_max(revision + 1, Ordering::Relaxed);
                            plan_id += 1;
                            let (plan, mut candidate) =
                                probe_plan_at(plan_id, revision + 1, &applied);
                            candidate.gate.publish(revision + 1);
                            if let Ok(reservation) =
                                publisher.reserve_replacement(plan, PlanAdoption::Next)
                            {
                                reservation.commit(revision + 1);
                                newest = candidate;
                                revision += 1;
                                publication = 1;
                            }
                        }
                    }
                    if step % 64 == 0 {
                        std::thread::yield_now();
                    }
                }
                final_word.store((revision << 32 | publication) + 1, Ordering::Relaxed);
                done.store(true, Ordering::Release);
                while !finished.load(Ordering::Acquire) {
                    let _ = retirer.try_reclaim();
                    std::thread::yield_now();
                }
                assert_eq!(read(&publisher).revision, revision);
            });
        }

        /// A plan hands out its gate handle once: a second handle would be a second writer with
        /// its own pin (#1502 D1).
        #[test]
        fn a_plan_hands_out_its_gate_handle_once() {
            let mut plan = prepared(1);
            assert!(plan.take_revision_gate().is_some());
            assert!(plan.take_revision_gate().is_none());
        }

        /// A plan stamps on its own pin while it holds its handle, and the open stamp moves with
        /// the pin into the handle (#1277 D11): the holder stamps the same revision again and
        /// publishes it, the plan's next block covers it, and once the handle is out the plan
        /// stamps nothing.
        #[test]
        fn a_held_gate_stamps_on_the_pin_the_handle_inherits() {
            let mut plan = prepared(1);
            assert!(
                plan.stamp_held_gate(0).expect("held").is_err(),
                "0 is not above 0"
            );
            assert!(
                plan.stamp_held_gate(GATE_REVISION_MAX + 1)
                    .expect("held")
                    .is_err()
            );
            let stamp = plan
                .stamp_held_gate(5)
                .expect("the pin is held")
                .expect("legal");
            assert_eq!(stamp.revision(), 5);
            let mut gate = plan.take_revision_gate().expect("the handle");
            assert!(
                plan.stamp_held_gate(5).is_none(),
                "the pin left with the handle"
            );
            gate.stamp(5).expect("the open revision stamps again");
            gate.publish(5);
            let mut output = [0.0_f32; 2];
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let report = plan
                .render(io, RenderTime { absolute_sample: 0 })
                .expect("render");
            assert_eq!(report.live_snapshot.revision(), 5);
        }

        /// The stamp a plan took before hand-out is open on the handle's pin: a different revision
        /// stamped on the handle while it is open breaks the publish-before-stamp rule (#1432).
        #[cfg(debug_assertions)]
        #[test]
        #[should_panic(expected = "is stamped and not yet published")]
        fn the_handle_inherits_the_open_stamp_of_a_held_gate() {
            let mut plan = prepared(1);
            let _ = plan
                .stamp_held_gate(5)
                .expect("the pin is held")
                .expect("legal");
            let mut gate = plan.take_revision_gate().expect("the handle");
            let _ = gate.stamp(6);
        }

        /// The initial revision is published on the initial plan's own gate: a handle of another
        /// plan is refused (#1502 D5).
        #[test]
        #[should_panic(expected = "the initial plan's own gate")]
        fn the_initial_revision_is_published_on_the_initial_plans_gate() {
            let (plan, _own) = plan_with_gate(1);
            let (_other, mut stranger) = plan_with_gate(2);
            let _ = plan_exchange_at_revision(plan, &mut stranger, 1, one_retirement());
        }

        /// One block whose output has two channels where the plan has one, so `render_inner`
        /// refuses it with `OutputShape`; `contiguous` picks the render path.
        fn refused_block(owner: &mut RealtimePlanOwner, sample: u64, contiguous: bool) {
            let mut output = [1.0_f32; 4];
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, 2, 2).expect("output"),
            };
            let result = if contiguous {
                owner.render_contiguous(io, sample)
            } else {
                owner.render(
                    io,
                    RenderTime {
                        absolute_sample: sample,
                    },
                )
            };
            assert!(
                matches!(result, Err(RenderError::OutputShape)),
                "{result:?}"
            );
        }

        /// #1314 D3: a block that errors publishes nothing, on either render path, even when the
        /// block adopted a candidate or a live edit is pending; the next block that renders
        /// publishes the revision with its own first sample. A block the shape check refuses takes
        /// no snapshot and is not a live-block error (#1502 D2, D7).
        #[test]
        fn a_block_that_errors_publishes_nothing() {
            let probe = SnapshotProbe::default();
            let (plan, gate) = probed(1, &probe);
            let (_first_gate, mut publisher, mut realtime, _retirer) = exchange_with(plan, gate, 1);
            // Plan one's own refused block takes no snapshot: its executor is never called.
            refused_block(&mut realtime, 0, false);
            assert_eq!(probe.calls(), 0);
            let mut b_gate = publish_at(&mut publisher, 2, 7);

            // `render`: the refused block claims B but renders nothing.
            refused_block(&mut realtime, QUANTUM, false);
            assert_eq!(realtime.active_plan_id(), 2);
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 0);
            let _ = block(&mut realtime, 2);
            assert_eq!(level(&publisher), (7, 2 * QUANTUM, OUTCOME_EXACT));

            // `render_contiguous`: a live edit on the running plan.
            b_gate.publish(8);
            let next = realtime.next_absolute_sample();
            refused_block(&mut realtime, next, true);
            assert_eq!(level(&publisher), (7, 2 * QUANTUM, OUTCOME_EXACT));
            let mut output = [1.0_f32; 2];
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let _ = realtime.render_contiguous(io, next).expect("render");
            assert_eq!(level(&publisher), (8, next, OUTCOME_EXACT));

            assert_eq!(realtime.live_block_errors(), 0);
            assert_eq!(probe.calls(), 0, "plan one never rendered a block");
        }

        /// A block the plan renders on either path, expecting the executor's failure.
        fn failing_block(owner: &mut RealtimePlanOwner, contiguous: bool) {
            let sample = owner.next_absolute_sample();
            let mut output = [1.0_f32; 2];
            let io = RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
            };
            let result = if contiguous {
                owner.render_contiguous(io, sample)
            } else {
                owner.render(
                    io,
                    RenderTime {
                        absolute_sample: sample,
                    },
                )
            };
            assert!(
                matches!(result, Err(RenderError::InvalidEnvelope)),
                "{result:?}"
            );
        }

        /// #1502 gate 5 (D7): a block the executor fails after the snapshot is counted, on both
        /// render paths, publishes nothing, and the next rendered block publishes at its own first
        /// sample.
        #[test]
        fn a_block_the_executor_fails_after_the_snapshot_is_counted() {
            let probe = SnapshotProbe::default();
            let (plan, gate) = probed(1, &probe);
            let (mut gate, publisher, mut realtime, _retirer) = exchange_with(plan, gate, 1);
            let _ = block(&mut realtime, 0);
            gate.publish(2);
            probe.fail.store(true, Ordering::Relaxed);

            failing_block(&mut realtime, false);
            assert_eq!(realtime.live_block_errors(), 1);
            assert_eq!(probe.last(), 2, "the executor saw the snapshot");
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            failing_block(&mut realtime, true);
            assert_eq!(realtime.live_block_errors(), 2);
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));

            probe.fail.store(false, Ordering::Relaxed);
            let next = realtime.next_absolute_sample();
            assert_eq!(next, QUANTUM, "a failed block did not advance the clock");
            let _ = render_once(&mut realtime, next);
            assert_eq!(level(&publisher), (2, next, OUTCOME_EXACT));
            assert_eq!(realtime.live_block_errors(), 2);
        }
    }

    /// #1311: a candidate adopted no earlier than a scheduled sample, or primed once the running
    /// plan is ready. Quantum 2; every block renders through `render_once`'s explicit time.
    mod scheduled_adoption {
        use super::*;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};

        /// What one plan's readiness hook answered and was asked. Atomics only: the hook runs
        /// inside the render scope.
        #[derive(Clone, Default)]
        struct Probe {
            ready: Arc<AtomicBool>,
            calls: Arc<AtomicUsize>,
            block_start: Arc<AtomicU64>,
            lead_blocks: Arc<AtomicU32>,
        }

        impl Probe {
            fn set_ready(&self, ready: bool) {
                self.ready.store(ready, Ordering::Relaxed);
            }
            fn calls(&self) -> usize {
                self.calls.load(Ordering::Relaxed)
            }
            /// `(block_start, lead_blocks)` of the last call.
            fn last(&self) -> (u64, u32) {
                (
                    self.block_start.load(Ordering::Relaxed),
                    self.lead_blocks.load(Ordering::Relaxed),
                )
            }
        }

        /// An executor whose readiness is the probe's flag.
        struct Gated(Probe);

        impl PreparedPlanExecutor for Gated {
            fn retained_bytes(&self) -> u64 {
                crate::realtime::retained::arc(&self.0.ready)
                    + crate::realtime::retained::arc(&self.0.calls)
                    + crate::realtime::retained::arc(&self.0.block_start)
                    + crate::realtime::retained::arc(&self.0.lead_blocks)
            }
            fn prime_ready(&self, block_start: u64, lead_blocks: u32) -> bool {
                self.0.calls.fetch_add(1, Ordering::Relaxed);
                self.0.block_start.store(block_start, Ordering::Relaxed);
                self.0.lead_blocks.store(lead_blocks, Ordering::Relaxed);
                self.0.ready.load(Ordering::Relaxed)
            }

            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(ExecutorBlockReport::default())
            }
        }

        /// An executor that keeps the default readiness hook.
        struct Unchecked;

        impl PreparedPlanExecutor for Unchecked {
            fn retained_bytes(&self) -> u64 {
                0
            }
            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(ExecutorBlockReport::default())
            }
        }

        fn with_executor(id: u64, executor: Box<dyn PreparedPlanExecutor>) -> PreparedRenderPlan {
            PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: id,
                    envelope: prepared(id).envelope(),
                    scratch: &[],
                },
                executor,
            )
            .expect("plan")
        }

        fn gated(id: u64, probe: &Probe) -> PreparedRenderPlan {
            with_executor(id, Box::new(Gated(probe.clone())))
        }

        fn reserve(
            publisher: &mut PlanPublisher,
            plan: PreparedRenderPlan,
            adoption: PlanAdoption,
        ) {
            publisher
                .reserve_replacement(plan, adoption)
                .expect("room")
                .commit(0);
        }

        /// Render the block at `sample` and return `(swap, plan that rendered it)`.
        fn at(owner: &mut RealtimePlanOwner, sample: u64) -> (SwapOutcome, u64) {
            let report = render_once(owner, sample);
            (report.swap, report.render.plan_id)
        }

        /// Reclaim every retired plan and check that each credit is back.
        fn credits_balance(publisher: &PlanPublisher, retirer: &mut PlanRetirer, capacity: usize) {
            while retirer.try_reclaim().is_ok() {}
            assert_eq!(publisher.free_retirement_credits(), capacity);
        }

        fn two_retirements() -> PlanExchangeConfig {
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(2).expect("two"),
            }
        }

        /// Gate 1, `Next` and `NoEarlierThan`: a `Next` candidate adopts at the next block, and
        /// one scheduled at 7 (between block starts) renders the predecessor at 2, 4 and 6 and
        /// adopts at 8. Neither asks the running plan's readiness hook.
        #[test]
        fn no_earlier_than_adopts_at_the_first_block_at_or_past_its_sample() {
            let (first, second, third) = (Probe::default(), Probe::default(), Probe::default());
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(gated(1, &first), two_retirements()).expect("exchange");
            reserve(&mut publisher, gated(2, &second), PlanAdoption::Next);
            assert_eq!(at(&mut owner, 0), (SwapOutcome::Applied, 2));
            reserve(
                &mut publisher,
                gated(3, &third),
                PlanAdoption::NoEarlierThan(7),
            );
            for sample in [2, 4, 6] {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 2),
                    "block {sample}"
                );
            }
            assert_eq!(at(&mut owner, 8), (SwapOutcome::Applied, 3));
            assert_eq!(owner.active_epoch(), PlanEpoch(2));
            assert_eq!([first.calls(), second.calls(), third.calls()], [0, 0, 0]);
            assert!(matches!(publisher.withdraw(), Withdrawal::Taken));
            credits_balance(&publisher, &mut retirer, 2);
        }

        /// Gate 1, hazard: `render` schedules by the host's explicit time. After a block at 0 the
        /// plan's clock stands at 2, but the host's next block starts at 100, past the due 50.
        #[test]
        fn render_schedules_by_the_hosts_time_not_the_plan_clock() {
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(prepared(1), one_retirement()).expect("exchange");
            reserve(&mut publisher, prepared(2), PlanAdoption::NoEarlierThan(50));
            assert_eq!(at(&mut owner, 0), (SwapOutcome::None, 1));
            assert_eq!(owner.next_absolute_sample(), 2);
            assert_eq!(at(&mut owner, 100), (SwapOutcome::Applied, 2));
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// #1597: before any block, `adopt_before_first_block` adopts the published candidate in
        /// place and retires the displaced plan; once `render_contiguous` or `render` has run a
        /// block it adopts nothing. Test value: red if either render call stops closing the window
        /// (the call would then swap a running plan off the render thread), or if the in-place
        /// adoption does not retire the plan it displaces.
        #[test]
        fn adopt_before_first_block_adopts_only_before_a_render_call() {
            for contiguous in [true, false] {
                let (mut publisher, mut owner, mut retirer) =
                    plan_exchange(prepared(1), one_retirement()).expect("exchange");
                reserve(&mut publisher, prepared(2), PlanAdoption::Next);
                assert_eq!(owner.adopt_before_first_block(), SwapOutcome::Applied);
                assert_eq!(owner.active_epoch(), PlanEpoch(1));
                assert_eq!(owner.active_plan_id(), 2);
                let (retired, plan) = retirer.try_reclaim().expect("the displaced plan retires");
                assert_eq!((retired, plan.program().plan_id()), (PlanEpoch(0), 1));
                drop(plan);

                let mut output = [1.0_f32; 2];
                let io = RenderIo {
                    output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
                };
                let report = if contiguous {
                    owner.render_contiguous(io, 0)
                } else {
                    owner.render(io, RenderTime { absolute_sample: 0 })
                }
                .expect("render");
                assert_eq!((report.swap, report.render.plan_id), (SwapOutcome::None, 2));
                reserve(&mut publisher, prepared(3), PlanAdoption::Next);
                assert_eq!(
                    owner.adopt_before_first_block(),
                    SwapOutcome::None,
                    "contiguous {contiguous}: a render call closed the window"
                );
                assert_eq!(
                    (owner.active_epoch(), owner.active_plan_id()),
                    (PlanEpoch(1), 2)
                );
                assert!(retirer.try_reclaim().is_err(), "nothing retired");
                assert!(matches!(publisher.withdraw(), Withdrawal::Withdrawn(_)));
            }
        }

        /// Gate 1 on the production path: `render_contiguous`, which the C ABI drives, schedules
        /// by the running plan's clock. A candidate due at 7 (between block starts) renders the
        /// predecessor at 0, 2, 4 and 6 and adopts at 8.
        #[test]
        fn render_contiguous_adopts_no_earlier_than_at_the_first_block_at_or_past_it() {
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(prepared(1), one_retirement()).expect("exchange");
            reserve(&mut publisher, prepared(2), PlanAdoption::NoEarlierThan(7));
            for sample in (0..=8).step_by(2) {
                assert_eq!(owner.next_absolute_sample(), sample);
                let mut output = [1.0_f32; 2];
                let io = RenderIo {
                    output: PlanarBufferMut::try_new(&mut output, 1, 2, 2).expect("output"),
                };
                let report = owner.render_contiguous(io, sample).expect("render");
                let expected = if sample < 8 {
                    (SwapOutcome::None, 1)
                } else {
                    (SwapOutcome::Applied, 2)
                };
                assert_eq!(
                    (report.swap, report.render.plan_id),
                    expected,
                    "block {sample}"
                );
            }
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// D5 for `Primed`: a `not_before` of 5, between block starts, is not reached by the
        /// block at 4, whose hook is not asked, and adopts at 6, where the hook saw 6.
        #[test]
        fn an_off_grid_primed_candidate_adopts_at_the_first_block_past_not_before() {
            let running = Probe::default();
            running.set_ready(true);
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(gated(1, &running), one_retirement()).expect("exchange");
            let primed = PlanAdoption::Primed {
                not_before: 5,
                lead_blocks: 1,
            };
            reserve(&mut publisher, prepared(2), primed);
            for sample in [0, 2, 4] {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 1),
                    "block {sample}"
                );
            }
            assert_eq!(running.calls(), 0, "asked before not_before");
            assert_eq!(at(&mut owner, 6), (SwapOutcome::Applied, 2));
            assert_eq!((running.calls(), running.last()), (1, (6, 1)));
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// Gate 1, `Primed` ready: not adopted before `not_before`, where the hook is not asked;
        /// adopted at `not_before`, where the running plan's hook saw that block's start and the
        /// lead, and the candidate's own hook was never asked.
        #[test]
        fn a_ready_primed_candidate_adopts_at_not_before() {
            let (running, candidate) = (Probe::default(), Probe::default());
            running.set_ready(true);
            candidate.set_ready(true);
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(gated(1, &running), one_retirement()).expect("exchange");
            let primed = PlanAdoption::Primed {
                not_before: 6,
                lead_blocks: 3,
            };
            reserve(&mut publisher, gated(2, &candidate), primed);
            for sample in [0, 2, 4] {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 1),
                    "block {sample}"
                );
            }
            assert_eq!(running.calls(), 0, "asked before not_before");
            assert_eq!(at(&mut owner, 6), (SwapOutcome::Applied, 2));
            assert_eq!(running.calls(), 1);
            assert_eq!(running.last(), (6, 3));
            assert_eq!(candidate.calls(), 0, "the candidate's hook was asked");
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// Gate 1, `Primed` unready: blocks 6, 8 and 10 render the predecessor and leave the
        /// candidate `Full`; it adopts at 12, the first block after readiness turns `true`.
        #[test]
        fn an_unready_primed_candidate_waits_published_until_ready() {
            use MailboxCellState::{Active, Full};
            let running = Probe::default();
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(gated(1, &running), one_retirement()).expect("exchange");
            let primed = PlanAdoption::Primed {
                not_before: 6,
                lead_blocks: 2,
            };
            reserve(&mut publisher, prepared(2), primed);
            for sample in [0, 2, 4] {
                assert_eq!(at(&mut owner, sample), (SwapOutcome::None, 1));
            }
            for sample in [6, 8, 10] {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 1),
                    "block {sample}"
                );
                assert_eq!(running.last(), (sample, 2));
                assert_eq!(publisher.mailbox_cell_states(), [Active, Full]);
            }
            assert_eq!(running.calls(), 3);
            running.set_ready(true);
            assert_eq!(at(&mut owner, 12), (SwapOutcome::Applied, 2));
            assert_eq!(running.last(), (12, 2));
            assert!(matches!(publisher.withdraw(), Withdrawal::Taken));
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// Gate 1, withdrawn and republished: an unready `Primed` candidate withdrawn before its
        /// `not_before` comes back with its epoch, plan ID and schedule; republished with the
        /// running plan now ready, it still waits for its `not_before` of 10.
        #[test]
        fn a_withdrawn_primed_candidate_republishes_with_its_schedule() {
            let running = Probe::default();
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(gated(1, &running), one_retirement()).expect("exchange");
            let primed = PlanAdoption::Primed {
                not_before: 10,
                lead_blocks: 2,
            };
            reserve(&mut publisher, prepared(2), primed);
            assert_eq!(at(&mut owner, 0), (SwapOutcome::None, 1));
            assert_eq!(at(&mut owner, 2), (SwapOutcome::None, 1));
            let candidate = withdrawn(&mut publisher);
            assert_eq!(
                (candidate.epoch(), candidate.plan_id(), candidate.adoption()),
                (PlanEpoch(1), 2, primed)
            );
            assert_eq!(at(&mut owner, 4), (SwapOutcome::None, 1));
            running.set_ready(true);
            assert_eq!(publisher.republish(candidate), PlanEpoch(1));
            for sample in [6, 8] {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 1),
                    "block {sample}"
                );
            }
            assert_eq!(at(&mut owner, 10), (SwapOutcome::Applied, 2));
            assert_eq!(running.last(), (10, 2));
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// Gate 1: a `NoEarlierThan` candidate withdrawn before its sample is never adopted, and
        /// withdrawal finds it published (not held by render) until then.
        #[test]
        fn a_no_earlier_than_candidate_withdrawn_before_its_sample_is_never_adopted() {
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(prepared(1), one_retirement()).expect("exchange");
            reserve(&mut publisher, prepared(2), PlanAdoption::NoEarlierThan(6));
            assert_eq!(at(&mut owner, 0), (SwapOutcome::None, 1));
            assert_eq!(at(&mut owner, 2), (SwapOutcome::None, 1));
            let candidate = withdrawn(&mut publisher);
            assert_eq!(candidate.adoption(), PlanAdoption::NoEarlierThan(6));
            drop(candidate);
            for sample in (4..=12).step_by(2) {
                assert_eq!(
                    at(&mut owner, sample),
                    (SwapOutcome::None, 1),
                    "block {sample}"
                );
            }
            assert!(matches!(publisher.withdraw(), Withdrawal::Nothing));
            credits_balance(&publisher, &mut retirer, 1);
        }

        /// Gate 1, default hook: a running plan that keeps the default readiness hook, or has no
        /// executor at all, never adopts a `Primed` candidate, which stays withdrawable.
        #[test]
        fn a_plan_without_a_readiness_check_never_adopts_a_primed_candidate() {
            let primed = PlanAdoption::Primed {
                not_before: 0,
                lead_blocks: 1,
            };
            for running in [with_executor(1, Box::new(Unchecked)), prepared(1)] {
                let (mut publisher, mut owner, mut retirer) =
                    plan_exchange(running, one_retirement()).expect("exchange");
                reserve(&mut publisher, prepared(2), primed);
                for sample in (0..=20).step_by(2) {
                    assert_eq!(
                        at(&mut owner, sample),
                        (SwapOutcome::None, 1),
                        "block {sample}"
                    );
                }
                let candidate = withdrawn(&mut publisher);
                assert_eq!(candidate.adoption(), primed);
                drop(candidate);
                credits_balance(&publisher, &mut retirer, 1);
            }
        }
    }

    #[test]
    fn launch_sample_rates_prepare_and_render() {
        for (index, rate) in crate::LAUNCH_SAMPLE_RATES.into_iter().enumerate() {
            let mut plan = PreparedRenderPlan::prepare(PrepareRenderPlan {
                plan_id: index as u64,
                envelope: RenderEnvelope {
                    sample_rate: rate,
                    quantum: QuantumFrames(1),
                    output_channels: NonZeroUsize::new(1).expect("one"),
                },
                scratch: &[],
            })
            .expect("launch rate");
            let mut output = [1.0];
            plan.render(
                RenderIo {
                    output: PlanarBufferMut::try_new(&mut output, 1, 1, 1).expect("output"),
                },
                RenderTime { absolute_sample: 0 },
            )
            .expect("render");
            assert_eq!(output, [0.0]);
        }
    }

    #[test]
    fn extended_and_unrelated_rates_reject_before_plan_publication() {
        for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001].map(SampleRateHz) {
            assert!(matches!(
                PreparedRenderPlan::prepare(PrepareRenderPlan {
                    plan_id: 0,
                    envelope: RenderEnvelope {
                        sample_rate: rate,
                        quantum: QuantumFrames(1),
                        output_channels: NonZeroUsize::new(1).expect("one"),
                    },
                    scratch: &[],
                }),
                Err(RenderError::UnsupportedRate)
            ));
        }
    }

    #[test]
    fn concurrent_plan_publication_is_complete_and_retirement_drops_off_render() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::{Arc, Mutex};

        const REPLACEMENTS: u64 = 64;
        let observer = Arc::new(Mutex::new(Vec::new()));
        let mut initial = prepared(0);
        initial.set_drop_observer(Arc::clone(&observer));
        let (mut publisher, mut owner, mut retirer) = plan_exchange(
            initial,
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(2).expect("retirement"),
            },
        )
        .expect("exchange");

        let mut replacements = Vec::new();
        for plan_id in 1..=REPLACEMENTS {
            let mut plan = prepared(plan_id);
            plan.set_drop_observer(Arc::clone(&observer));
            replacements.push(plan);
        }

        let publisher_done = Arc::new(AtomicBool::new(false));
        let publisher_done_thread = Arc::clone(&publisher_done);
        let publisher_thread = std::thread::spawn(move || {
            for mut candidate in replacements {
                loop {
                    match publisher.publish(candidate, 0) {
                        Ok(_) => break,
                        Err(PublishError::Full(returned)) => {
                            candidate = returned;
                            std::thread::yield_now();
                        }
                        Err(PublishError::Incompatible(_)) => panic!("compatible plan rejected"),
                        Err(PublishError::EpochExhausted(_)) => panic!("epoch exhausted"),
                    }
                }
            }
            publisher_done_thread.store(true, Ordering::Release);
        });

        let stop_retirer = Arc::new(AtomicBool::new(false));
        let stop_retirer_thread = Arc::clone(&stop_retirer);
        let retirement_thread = std::thread::spawn(move || {
            let thread_id = std::thread::current().id();
            while !stop_retirer_thread.load(Ordering::Acquire) {
                match retirer.try_reclaim() {
                    Ok(retired) => drop(retired),
                    Err(_) => std::thread::yield_now(),
                }
            }
            while let Ok(retired) = retirer.try_reclaim() {
                drop(retired);
            }
            thread_id
        });

        let mut previous_epoch = PlanEpoch(0);
        let mut iterations = 0_u64;
        while !publisher_done.load(Ordering::Acquire)
            || owner.active_epoch() != PlanEpoch(REPLACEMENTS)
        {
            let report = render_once(&mut owner, iterations.saturating_mul(2));
            assert!(report.active_epoch >= previous_epoch);
            assert_eq!(report.render.plan_id, report.active_epoch.0);
            previous_epoch = report.active_epoch;
            iterations += 1;
            assert!(iterations < 1_000_000, "bounded publication stress stalled");
            std::thread::yield_now();
        }
        publisher_thread.join().expect("publisher");

        let mut retirement_wait = 0;
        while observer.lock().expect("observer").len() < REPLACEMENTS as usize {
            retirement_wait += 1;
            assert!(retirement_wait < 1_000_000, "retirement stress stalled");
            std::thread::yield_now();
        }
        stop_retirer.store(true, Ordering::Release);
        let retirement_thread_id = retirement_thread.join().expect("retirer");
        let observed = observer.lock().expect("observer");
        assert_eq!(observed.len(), REPLACEMENTS as usize);
        assert!(
            observed
                .iter()
                .all(|(_, thread_id)| *thread_id == retirement_thread_id)
        );
    }

    /// Slice #1270: the successor's one chance to take state from the plan it displaces.
    mod carry {
        use super::*;
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        const FRAMES: u32 = 16;
        const BLOCKS: usize = 10;
        /// One sample's rotation, `(cos, sin)` of 0.0731 rad: the oscillator advances its phase
        /// as a unit phasor, so the test needs no transcendental call.
        const ROTATION: (f64, f64) = (0.9973293845450933, 0.07303491441020102);

        std::thread_local! {
            /// Whether the last hand-over on this thread ran inside a render scope.
            static HOOK_IN_RENDER_SCOPE: core::cell::Cell<Option<bool>> =
                const { core::cell::Cell::new(None) };
        }

        /// A sine oscillator whose phase (a unit phasor `(cos, sin)`) is its only state.
        struct Oscillator {
            phase: (f64, f64),
            carry: bool,
            hooks: Arc<AtomicUsize>,
        }

        impl PreparedPlanExecutor for Oscillator {
            fn retained_bytes(&self) -> u64 {
                crate::realtime::retained::arc(&self.hooks)
            }
            fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
                Some(self)
            }

            fn adopt_predecessor(
                &mut self,
                predecessor: &mut dyn PreparedPlanExecutor,
                _block_start: u64,
            ) -> CarryOutcome {
                self.hooks.fetch_add(1, Ordering::Relaxed);
                HOOK_IN_RENDER_SCOPE.set(Some(audit::is_render_scope_active()));
                if !self.carry {
                    return CarryOutcome::NotRequested;
                }
                match predecessor
                    .as_any_mut()
                    .and_then(|any| any.downcast_mut::<Self>())
                {
                    Some(predecessor) => {
                        self.phase = predecessor.phase;
                        CarryOutcome::Carried
                    }
                    None => CarryOutcome::PredecessorMismatch,
                }
            }

            fn copy_response_snapshot(
                &self,
                _track_id: &str,
                _captured_sample: u64,
                _sink: &mut dyn ResponseSnapshotSink,
            ) -> Result<u32, ResponseSnapshotError> {
                Ok(0)
            }

            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                for sample in output.plane_mut(0)? {
                    // Offset so the test signal has no exact-zero sample.
                    *sample = (0.5 + 0.25 * self.phase.1) as f32;
                    let (cos, sin) = self.phase;
                    self.phase = (
                        cos * ROTATION.0 - sin * ROTATION.1,
                        sin * ROTATION.0 + cos * ROTATION.1,
                    );
                }
                Ok(ExecutorBlockReport::default())
            }
        }

        /// An executor of another shape: it offers nothing to a successor.
        struct Stranger;

        impl PreparedPlanExecutor for Stranger {
            fn retained_bytes(&self) -> u64 {
                0
            }
            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<ExecutorBlockReport, RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(ExecutorBlockReport::default())
            }
        }

        fn envelope() -> RenderEnvelope {
            RenderEnvelope {
                sample_rate: SampleRateHz(48_000),
                quantum: QuantumFrames(FRAMES),
                output_channels: NonZeroUsize::new(1).expect("one"),
            }
        }

        fn plan(id: u64, executor: Box<dyn PreparedPlanExecutor>) -> PreparedRenderPlan {
            PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: id,
                    envelope: envelope(),
                    scratch: &[],
                },
                executor,
            )
            .expect("plan")
        }

        fn oscillator(id: u64, carry: bool, hooks: &Arc<AtomicUsize>) -> PreparedRenderPlan {
            plan(
                id,
                Box::new(Oscillator {
                    phase: (1.0, 0.0),
                    carry,
                    hooks: Arc::clone(hooks),
                }),
            )
        }

        fn config(retirement: usize) -> PlanExchangeConfig {
            PlanExchangeConfig {
                retirement_capacity: NonZeroUsize::new(retirement).expect("retirement"),
            }
        }

        fn owner_block(
            owner: &mut RealtimePlanOwner,
        ) -> ([f32; FRAMES as usize], RealtimeRenderReport) {
            let mut output = [0.0_f32; FRAMES as usize];
            let sample = owner.next_absolute_sample();
            let report = owner
                .render_contiguous(
                    RenderIo {
                        output: PlanarBufferMut::try_new(
                            &mut output,
                            1,
                            FRAMES as usize,
                            FRAMES as usize,
                        )
                        .expect("output"),
                    },
                    sample,
                )
                .expect("render");
            (output, report)
        }

        fn plan_block(plan: &mut PreparedRenderPlan) -> [f32; FRAMES as usize] {
            let mut output = [0.0_f32; FRAMES as usize];
            let sample = plan.next_absolute_sample();
            plan.render_contiguous(
                RenderIo {
                    output: PlanarBufferMut::try_new(
                        &mut output,
                        1,
                        FRAMES as usize,
                        FRAMES as usize,
                    )
                    .expect("output"),
                },
                sample,
            )
            .expect("render");
            output
        }

        /// One unswapped oscillator rendered for `BLOCKS` blocks.
        fn reference() -> Vec<[f32; FRAMES as usize]> {
            let hooks = Arc::new(AtomicUsize::new(0));
            let mut plan = oscillator(0, true, &hooks);
            let blocks: Vec<_> = (0..BLOCKS).map(|_| plan_block(&mut plan)).collect();
            assert!(blocks.iter().flatten().all(|sample| *sample != 0.0));
            blocks
        }

        /// Plan A for five blocks, B published, five more: returns the blocks and the reports.
        #[cfg(feature = "realtime-audit")]
        fn swap_at_block_five(
            carry: bool,
        ) -> (
            Vec<[f32; FRAMES as usize]>,
            Vec<RealtimeRenderReport>,
            RealtimePlanOwner,
            usize,
        ) {
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            let mut blocks = Vec::new();
            let mut reports = Vec::new();
            for block in 0..BLOCKS {
                if block == 5 {
                    publisher
                        .reserve_replacement(oscillator(1, carry, &hooks), PlanAdoption::Next)
                        .expect("reserve")
                        .commit(0);
                }
                let (output, report) = owner_block(&mut owner);
                blocks.push(output);
                reports.push(report);
            }
            (blocks, reports, owner, hooks.load(Ordering::Relaxed))
        }

        #[test]
        fn a_mismatched_predecessor_is_reported_not_carried() {
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(plan(0, Box::new(Stranger)), config(1)).expect("exchange");
            owner_block(&mut owner);
            publisher
                .reserve_replacement(oscillator(1, true, &hooks), PlanAdoption::Next)
                .expect("reserve")
                .commit(0);
            let (output, report) = owner_block(&mut owner);
            assert_eq!(report.carry, CarryOutcome::PredecessorMismatch);
            assert_eq!(output, reference()[0]);
            assert_eq!(
                (owner.carried_count(), owner.carry_mismatch_count()),
                (0, 1)
            );
        }

        #[test]
        fn dropping_the_owner_never_hands_over_to_an_unapplied_candidate() {
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            owner_block(&mut owner);
            publisher
                .reserve_replacement(oscillator(1, true, &hooks), PlanAdoption::Next)
                .expect("reserve")
                .commit(0);
            drop(owner);
            assert_eq!(hooks.load(Ordering::Relaxed), 0);
        }

        fn capture_epoch(owner: &mut RealtimePlanOwner) -> PlanEpoch {
            owner
                .copy_response_snapshot(ResponseSnapshotRequest {
                    track_id: "track",
                    sink: &mut Stranger,
                })
                .expect("capture")
                .active_epoch
        }

        #[test]
        fn the_non_contiguous_render_reports_the_hand_over() {
            let reference = reference();
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            let mut blocks = Vec::new();
            for block in 0..BLOCKS {
                if block == 5 {
                    publisher
                        .reserve_replacement(oscillator(1, true, &hooks), PlanAdoption::Next)
                        .expect("reserve")
                        .commit(0);
                }
                let before = capture_epoch(&mut owner);
                let mut output = [0.0_f32; FRAMES as usize];
                let report = owner
                    .render(
                        RenderIo {
                            output: PlanarBufferMut::try_new(
                                &mut output,
                                1,
                                FRAMES as usize,
                                FRAMES as usize,
                            )
                            .expect("output"),
                        },
                        RenderTime {
                            absolute_sample: block as u64 * u64::from(FRAMES),
                        },
                    )
                    .expect("render");
                let carry = if block == 5 {
                    CarryOutcome::Carried
                } else {
                    CarryOutcome::NotRequested
                };
                assert_eq!(report.carry, carry, "block {block}");
                // A capture names the plan that rendered: the predecessor until the swap block,
                // the successor from it on.
                let after = capture_epoch(&mut owner);
                assert_eq!(after, owner.active_epoch(), "block {block}");
                assert_eq!(before == after, block != 5, "block {block}");
                blocks.push(output);
            }
            assert_eq!(blocks, reference);
            assert_eq!(owner.carried_count(), 1);
        }

        /// The hand-over is part of the swap block: the realtime audit must see it, in the
        /// exchange path and in the synchronous form a host calls outside any render.
        #[cfg(feature = "realtime-audit")]
        #[test]
        fn the_hand_over_runs_inside_a_render_scope() {
            let hooks = Arc::new(AtomicUsize::new(0));
            let mut predecessor = oscillator(0, true, &hooks);
            let mut successor = oscillator(1, true, &hooks);
            HOOK_IN_RENDER_SCOPE.set(None);
            assert!(!audit::is_render_scope_active());
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::Carried
            );
            assert_eq!(HOOK_IN_RENDER_SCOPE.get(), Some(true));

            HOOK_IN_RENDER_SCOPE.set(None);
            let (_, reports, _, _) = swap_at_block_five(true);
            assert_eq!(reports[5].carry, CarryOutcome::Carried);
            assert_eq!(HOOK_IN_RENDER_SCOPE.get(), Some(true));
        }

        #[test]
        fn a_synchronous_predecessor_of_another_envelope_is_refused() {
            let hooks = Arc::new(AtomicUsize::new(0));
            let mut predecessor = PreparedRenderPlan::prepare_with_executor(
                PrepareRenderPlan {
                    plan_id: 0,
                    envelope: RenderEnvelope {
                        quantum: QuantumFrames(FRAMES * 2),
                        ..envelope()
                    },
                    scratch: &[],
                },
                Box::new(Oscillator {
                    phase: (1.0, 0.0),
                    carry: true,
                    hooks: Arc::clone(&hooks),
                }),
            )
            .expect("plan");
            let mut output = [0.0_f32; 2 * FRAMES as usize];
            predecessor
                .render_contiguous(
                    RenderIo {
                        output: PlanarBufferMut::try_new(
                            &mut output,
                            1,
                            2 * FRAMES as usize,
                            2 * FRAMES as usize,
                        )
                        .expect("output"),
                    },
                    0,
                )
                .expect("render");
            let mut successor = oscillator(1, true, &hooks);
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::PredecessorMismatch
            );
            assert_eq!(successor.next_absolute_sample(), 0);
            assert_eq!(hooks.load(Ordering::Relaxed), 0);
            assert_eq!(plan_block(&mut successor), reference()[0]);
        }

        #[test]
        fn synchronous_hand_over_continues_state_and_clock() {
            let reference = reference();
            let hooks = Arc::new(AtomicUsize::new(0));
            let mut predecessor = oscillator(0, true, &hooks);
            let mut blocks: Vec<_> = (0..5).map(|_| plan_block(&mut predecessor)).collect();
            let mut successor = oscillator(1, true, &hooks);
            assert_eq!(
                successor.adopt_predecessor_plan(&mut predecessor),
                CarryOutcome::Carried
            );
            let mut sink = Stranger;
            let capture = successor
                .copy_response_snapshot(ResponseSnapshotRequest {
                    track_id: "track",
                    sink: &mut sink,
                })
                .expect("capture");
            assert_eq!(capture.captured_sample, 5 * u64::from(FRAMES));
            blocks.extend((5..BLOCKS).map(|_| plan_block(&mut successor)));
            assert_eq!(blocks, reference);
            assert_eq!(
                successor.next_absolute_sample(),
                (BLOCKS as u64) * u64::from(FRAMES)
            );
        }

        impl ResponseSnapshotSink for Stranger {
            fn copy_owner(
                &mut self,
                _owner: ResponseSnapshotOwnerInfo<'_>,
                _left: &[ResponseSnapshotSection],
                _right: &[ResponseSnapshotSection],
            ) -> Result<(), ResponseSnapshotError> {
                Ok(())
            }
        }
    }
}
