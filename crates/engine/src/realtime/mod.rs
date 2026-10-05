//! Realtime-only ownership, storage, and block-boundary publication primitives.
//!
//! `PreparedRenderPlan::render` is deliberately a small, allocation-free surface.  All capacity
//! decisions are made by `prepare`; objects returned here are not synchronizable so exclusive
//! render ownership remains apparent in the type system.

pub mod audit;
mod buffer;
mod disjoint;
mod observe;
mod plan;
mod plan_exchange;
mod spsc;
mod watermark;

pub use buffer::{BufferArena, BufferArenaError, BufferIndex, PlanarBufferMut, PlanarBufferSpec};
pub use disjoint::{
    ARENA_SILENCE_BUFFER, ArenaStereoPair, ArenaStereoPlanes, DisjointArena, DisjointArenaError,
};
pub use observe::{
    ObservationPublisher, ObservationReader, ObservationSlot, ObservationWindow, observation_slot,
    observation_slot_retained_bytes,
};
pub use plan::{
    CarryOutcome, PlanUnitEligibility, PrepareRenderPlan, PreparedPlanExecutor, PreparedProgram,
    PreparedRenderPlan, RenderEnvelope, RenderError, RenderIo, RenderReport, RenderTime,
    ResponseSnapshotAvailability, ResponseSnapshotCapture, ResponseSnapshotError,
    ResponseSnapshotOwnerInfo, ResponseSnapshotRequest, ResponseSnapshotSection,
    ResponseSnapshotSink,
};
pub use plan_exchange::{
    PlanEpoch, PlanExchangeConfig, PlanExchangeResourceReport, PlanPublisher,
    PlanReplacementReservation, PlanReplacementReservationError, PlanRetirer, PublishError,
    RealtimePlanOwner, RealtimeRenderReport, RealtimeResponseSnapshot, RevisionTarget, SwapOutcome,
    UnadoptedCandidate, Withdrawal, plan_exchange, plan_exchange_at_revision,
    plan_exchange_resource_report,
};
pub use spsc::{
    Consumer, PlanAdoption, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError,
    SpscRetainedPayload, bounded_spsc, bounded_spsc_move, bounded_spsc_retained_payload,
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
        ) -> Result<(), RenderError> {
            output.plane_mut(0)?.fill(0.0);
            output.plane_mut(1)?.fill(0.0);
            Ok(())
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

    #[test]
    fn native_spsc_preserves_value_on_full_and_fifo_wrap() {
        let (mut producer, mut consumer) =
            bounded_spsc::<u32>(NonZeroUsize::new(2).expect("two"), QueueGeneration(9))
                .expect("queue");
        producer.try_push(1).expect("one");
        producer.try_push(2).expect("two");
        assert!(matches!(
            producer.try_push(3),
            Err(QueueFull {
                value: 3,
                generation: QueueGeneration(9),
                ..
            })
        ));
        assert_eq!(consumer.try_pop().expect("first"), 1);
        producer.try_push(3).expect("wrap");
        assert_eq!(consumer.try_pop().expect("second"), 2);
        assert_eq!(consumer.try_pop().expect("third"), 3);
        assert!(matches!(
            consumer.try_pop(),
            Err(QueueEmpty {
                generation: QueueGeneration(9),
                empty_count: 1,
            })
        ));
        assert_eq!(producer.capacity(), 2);
        assert_eq!(producer.success_count(), 3);
        assert_eq!(producer.full_count(), 1);
        assert_eq!(consumer.success_count(), 3);
    }

    #[test]
    fn move_only_spsc_preserves_full_value_and_transfers_to_consumer() {
        #[derive(Debug)]
        struct MoveOnly(u32);
        let (mut producer, mut consumer) =
            bounded_spsc_move::<MoveOnly>(NonZeroUsize::new(1).expect("one"), QueueGeneration(10))
                .expect("queue");
        producer.try_push(MoveOnly(7)).expect("first transfer");
        let full = producer
            .try_push(MoveOnly(9))
            .expect_err("full queue returns move-only item");
        assert_eq!(full.value.0, 9);
        assert_eq!(consumer.try_pop().expect("single consumer transfer").0, 7);
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
        assert_eq!(reservation.commit(), PlanEpoch(1));
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
        reservation.commit();
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
        assert_eq!(replacement.commit(), PlanEpoch(1));
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
            .commit();
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
            .commit();
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
        assert!(matches!(publisher.publish(prepared(2)), Ok(PlanEpoch(1))));
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
        assert!(matches!(publisher.publish(prepared(3)), Ok(PlanEpoch(2))));
    }

    /// #1343 gate 1: once render has claimed the candidate, withdrawal reports `Taken` and the
    /// candidate is the active plan.
    #[test]
    fn withdrawal_after_render_claimed_reports_taken() {
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), one_retirement()).expect("exchange");
        assert!(publisher.publish(prepared(2)).is_ok());
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
        assert!(publisher.publish(prepared(2)).is_ok());
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
        assert!(matches!(publisher.publish(prepared(3)), Ok(PlanEpoch(2))));
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
        assert!(publisher.publish(prepared(2)).is_ok());
        assert_eq!(publisher.mailbox_cell_states(), [Active, Full]);
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        assert_eq!(publisher.mailbox_cell_states(), [Empty, Active]);
        assert!(publisher.publish(prepared(3)).is_ok());
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
        assert!(publisher.publish(prepared(2)).is_ok());
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert_eq!(publisher.free_retirement_credits(), 0);
        let returned = match publisher.publish(prepared(3)) {
            Err(PublishError::Full(returned)) => returned,
            _ => panic!("a credit-less publication must fail Full"),
        };
        assert_eq!(returned.program().plan_id(), 3);
        assert_eq!(publisher.mailbox_cell_states(), [Empty, Active]);
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::None);
        assert!(matches!(publisher.withdraw(), Withdrawal::Taken));

        let _ = retirer.try_reclaim().expect("plan one");
        assert_eq!(publisher.free_retirement_credits(), 1);
        assert!(matches!(publisher.publish(returned), Ok(PlanEpoch(2))));
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

    /// #1314: the applied-revision watermark, read the way any thread reads it.
    mod watermark {
        use super::*;

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

        fn publish_at(publisher: &mut PlanPublisher, id: u64, revision: u64) {
            let mut reservation = publisher
                .reserve_replacement(prepared(id), PlanAdoption::Next)
                .expect("room");
            reservation.set_revision(revision);
            reservation.commit();
        }

        /// #1314 gate 1: a revision completes when render adopts the plan that carries it, never
        /// at publication, and a withdrawn candidate takes its revision out of the cell it left.
        #[test]
        fn a_revision_completes_at_adoption_not_at_publication() {
            let (mut publisher, mut realtime, mut retirer) =
                plan_exchange_at_revision(prepared(1), 1, one_retirement()).expect("exchange");
            assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 0);

            publish_at(&mut publisher, 2, 7);
            assert_eq!(
                publisher.mailbox_cell_states(),
                [MailboxCellState::Active, MailboxCellState::Full]
            );
            assert_eq!(block(&mut realtime, 0).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (7, 0, OUTCOME_EXACT));
            let _ = retirer.try_reclaim().expect("plan one");

            // B lands in cell 0, P0's old cell, and leaves it again with its revision.
            publish_at(&mut publisher, 3, 8);
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
            let _ = retirer.try_reclaim().expect("plan A");

            publish_at(&mut publisher, 4, 9);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
        }

        /// #1314 gate 4: a candidate's `superseded` count and its outcome complete once, at the
        /// first advance after its claim, and a withdrawn candidate takes its outcome with it.
        #[test]
        fn superseded_and_fallback_revisions_complete_once_per_adoption() {
            let (mut publisher, mut realtime, mut retirer) =
                plan_exchange_at_revision(prepared(1), 1, one_retirement()).expect("exchange");
            let a = read(&publisher).revision;
            let mut reservation = publisher
                .reserve_replacement(prepared(2), PlanAdoption::Next)
                .expect("room");
            reservation.set_revision(a + 3);
            reservation.set_superseded(2);
            reservation.commit();
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
            assert_eq!(publisher.set_revision(a + 4), RevisionTarget::Active);
            let _ = block(&mut realtime, 1);
            let live = read(&publisher);
            assert_eq!((live.revision, live.outcome_flags), (a + 4, OUTCOME_EXACT));
            assert_eq!(
                (live.superseded, live.exact, live.transition_fallback),
                (2, 2, 0)
            );

            let b = live.revision;
            let mut reservation = publisher
                .reserve_replacement(prepared(3), PlanAdoption::Next)
                .expect("room");
            reservation.set_revision(b + 2);
            reservation.set_outcome(CandidateOutcome::TransitionFallback);
            reservation.commit();
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

            assert_eq!(publisher.set_revision(b + 3), RevisionTarget::Active);
            let _ = block(&mut realtime, 3);
            let live = read(&publisher);
            assert_eq!((live.revision, live.outcome_flags), (b + 3, OUTCOME_EXACT));
            assert_eq!(
                (live.superseded, live.exact, live.transition_fallback),
                (2, 3, 2)
            );
        }

        /// #1314 gate 8: a revision committed while a candidate is pending, held by the control
        /// thread or published, goes to that candidate and never to the running plan's cell.
        #[test]
        fn a_revision_follows_a_held_or_published_candidate() {
            let (mut publisher, mut realtime, mut retirer) =
                plan_exchange_at_revision(prepared(1), 1, one_retirement()).expect("exchange");
            publish_at(&mut publisher, 2, 7);
            let mut held = withdrawn(&mut publisher);
            // A model-only commit while control holds A.
            held.set_revision(8);
            for index in 0..=2 {
                assert_eq!(block(&mut realtime, index).swap, SwapOutcome::None);
                assert_eq!(level(&publisher), (1, 0, OUTCOME_EXACT), "block {index}");
            }
            let _ = publisher.republish(held);
            assert_eq!(block(&mut realtime, 3).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
            let _ = retirer.try_reclaim().expect("plan one");

            publish_at(&mut publisher, 3, 9);
            // A model-only commit while B is published.
            assert_eq!(publisher.set_revision(10), RevisionTarget::Pending);
            assert_eq!(level(&publisher), (8, 3 * QUANTUM, OUTCOME_EXACT));
            assert_eq!(block(&mut realtime, 4).swap, SwapOutcome::Applied);
            assert_eq!(level(&publisher), (10, 4 * QUANTUM, OUTCOME_EXACT));
            assert_eq!(read(&publisher).exact, 9);
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
            ) -> Result<(), RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(())
            }
        }

        /// An executor that keeps the default readiness hook.
        struct Unchecked;

        impl PreparedPlanExecutor for Unchecked {
            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<(), RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(())
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
                .commit();
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
                    match publisher.publish(candidate) {
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
            fn as_any_mut(&mut self) -> Option<&mut dyn core::any::Any> {
                Some(self)
            }

            fn adopt_predecessor(
                &mut self,
                predecessor: &mut dyn PreparedPlanExecutor,
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
            ) -> Result<(), RenderError> {
                for sample in output.plane_mut(0)? {
                    // Offset so the test signal has no exact-zero sample.
                    *sample = (0.5 + 0.25 * self.phase.1) as f32;
                    let (cos, sin) = self.phase;
                    self.phase = (
                        cos * ROTATION.0 - sin * ROTATION.1,
                        sin * ROTATION.0 + cos * ROTATION.1,
                    );
                }
                Ok(())
            }
        }

        /// An executor of another shape: it offers nothing to a successor.
        struct Stranger;

        impl PreparedPlanExecutor for Stranger {
            fn render(
                &mut self,
                _arena: &mut BufferArena,
                mut output: PlanarBufferMut<'_>,
                _time: RenderTime,
            ) -> Result<(), RenderError> {
                output.plane_mut(0)?.fill(0.0);
                Ok(())
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
                        .commit();
                }
                let (output, report) = owner_block(&mut owner);
                blocks.push(output);
                reports.push(report);
            }
            (blocks, reports, owner, hooks.load(Ordering::Relaxed))
        }

        #[test]
        fn successor_continues_the_predecessor_state_gap_free() {
            let reference = reference();
            let (blocks, reports, owner, hooks) = swap_at_block_five(true);
            assert_eq!(blocks, reference);
            assert_eq!(hooks, 1);
            for (block, report) in reports.iter().enumerate() {
                let (swap, carry) = if block == 5 {
                    (SwapOutcome::Applied, CarryOutcome::Carried)
                } else {
                    (SwapOutcome::None, CarryOutcome::NotRequested)
                };
                assert_eq!((report.swap, report.carry), (swap, carry), "block {block}");
            }
            assert_eq!(
                (owner.carried_count(), owner.carry_mismatch_count()),
                (1, 0)
            );

            let (blocks, reports, owner, _) = swap_at_block_five(false);
            assert_eq!(blocks[..5], reference[..5]);
            assert_ne!(blocks[5], reference[5]);
            assert_eq!(reports[5].carry, CarryOutcome::NotRequested);
            assert_eq!(owner.carried_count(), 0);
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
                .commit();
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
                .commit();
            drop(owner);
            assert_eq!(hooks.load(Ordering::Relaxed), 0);
        }

        /// Two carrying swaps on one owner, with a withdrawn candidate between them: each
        /// applied block hands over exactly once, the withdrawn block hands over nothing, and the
        /// output stays gap-free against the uninterrupted reference.
        #[test]
        fn hand_over_runs_only_on_the_applied_block() {
            let reference = reference();
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            let mut blocks = Vec::new();
            let mut withdrawn = None;
            for block in 0..BLOCKS {
                match block {
                    1 => assert!(publisher.publish(oscillator(1, true, &hooks)).is_ok()),
                    3 => {
                        let _ = retirer.try_reclaim().expect("plan zero");
                        assert!(publisher.publish(oscillator(2, true, &hooks)).is_ok());
                        withdrawn = match publisher.withdraw() {
                            Withdrawal::Withdrawn(candidate) => Some(candidate),
                            _ => panic!("an unclaimed candidate is withdrawn"),
                        };
                    }
                    5 => {
                        let _ = publisher.republish(withdrawn.take().expect("candidate"));
                    }
                    _ => {}
                }
                let before = hooks.load(Ordering::Relaxed);
                let (output, report) = owner_block(&mut owner);
                blocks.push(output);
                let (swap, carry, calls) = match block {
                    1 | 5 => (SwapOutcome::Applied, CarryOutcome::Carried, 1),
                    _ => (SwapOutcome::None, CarryOutcome::NotRequested, 0),
                };
                assert_eq!((report.swap, report.carry), (swap, carry), "block {block}");
                assert_eq!(
                    hooks.load(Ordering::Relaxed) - before,
                    calls,
                    "block {block}"
                );
            }
            assert_eq!(blocks, reference);
            assert_eq!(owner.carried_count(), 2);
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
                        .commit();
                }
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
