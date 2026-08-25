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

pub use buffer::{
    BufferArena, BufferArenaError, BufferIndex, PlanarBufferMut, PlanarBufferRef, PlanarBufferSpec,
};
pub use disjoint::{
    ARENA_SILENCE_BUFFER, ArenaLeaseSetBuilder, ArenaLeaseV1, ArenaStereoPair, DisjointArena,
    DisjointArenaError,
};
pub use observe::{
    ObservationPublisherV1, ObservationReaderV1, ObservationSlotV1, ObservationWindowV1,
    observation_slot, observation_slot_retained_bytes,
};
pub use plan::{
    ExecutorHandover, PrepareRenderPlan, PreparedPlanExecutor, PreparedProgram, PreparedRenderPlan,
    RenderEnvelope, RenderError, RenderIo, RenderReport, RenderTime,
};
pub use plan_exchange::{
    PlanEpoch, PlanExchangeConfig, PlanExchangeResourceReport, PlanPublisher,
    PlanReplacementReservation, PlanReplacementReservationError, PlanRetirer, PublishError,
    RealtimePlanOwner, RealtimeRenderReport, SwapOutcome, plan_exchange,
    plan_exchange_resource_report,
};
pub use spsc::{
    Consumer, LocalRing, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError,
    SpscRetainedPayload, bounded_spsc, bounded_spsc_move, bounded_spsc_retained_payload,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{QuantumFrames, SampleRateHz};
    use core::num::NonZeroUsize;

    /// The plan owns the clock, and a contiguous render is the only caller that has to know the
    /// rule. Rendering the same block twice is a discontinuity, and the error names the sample the
    /// plan is waiting for.
    #[test]
    fn render_contiguous_rejects_stale_and_accepts_next() {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
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
            .render_contiguous(
                RenderIo {
                    input: None,
                    output,
                },
                0,
            )
            .expect("first block");
        assert_eq!(report.next_absolute_sample, 4);
        assert_eq!(plan.next_absolute_sample(), 4);

        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        assert_eq!(
            plan.render_contiguous(
                RenderIo {
                    input: None,
                    output,
                },
                0,
            ),
            Err(RenderError::TimeDiscontinuity { expected: 4 })
        );

        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        plan.render_contiguous(
            RenderIo {
                input: None,
                output,
            },
            4,
        )
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
    fn plan_exchange_resource_projection_covers_both_queues_and_checks_overflow() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let report = plan_exchange_resource_report(config).expect("projection");
        assert!(report.retained_payload_bytes > report.largest_allocation_bytes);
        assert!(report.largest_allocation_bytes > 0);
        assert_eq!(
            plan_exchange_resource_report(PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(usize::MAX).expect("maximum is nonzero"),
                ..config
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
    fn move_only_spsc_preserves_full_value_and_drops_once_after_transfer() {
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

    #[test]
    fn local_ring_supports_capacity_one_and_wraparound() {
        let mut ring = LocalRing::new(NonZeroUsize::new(1).expect("one"), QueueGeneration(4))
            .expect("local ring");
        assert_eq!(ring.capacity(), 1);
        assert_eq!(ring.generation(), QueueGeneration(4));
        ring.try_push(7_u32).expect("push");
        assert!(matches!(ring.try_push(8), Err(QueueFull { value: 8, .. })));
        assert_eq!(ring.try_pop().expect("pop"), 7);
        assert!(ring.try_pop().is_err());
        ring.try_push(9).expect("wrapped push");
        assert_eq!(ring.try_pop().expect("wrapped pop"), 9);
        assert_eq!(ring.full_count(), 1);
        assert_eq!(ring.empty_count(), 1);
    }

    fn prepared(id: u64) -> PreparedRenderPlan {
        PreparedRenderPlan::prepare(PrepareRenderPlan {
            plan_id: id,
            envelope: RenderEnvelope {
                sample_rate: SampleRateHz(48_000),
                quantum: QuantumFrames(2),
                input_channels: None,
                output_channels: NonZeroUsize::new(1).expect("one"),
            },
            scratch: &[],
        })
        .expect("plan")
    }
    /// A token executor that owns one hand-over resource and can be told to refuse one.
    struct HandoverExecutor {
        token: Option<ExecutorHandover>,
        accepts: bool,
        accepted: std::sync::Arc<core::sync::atomic::AtomicU64>,
    }
    impl PreparedPlanExecutor for HandoverExecutor {
        fn render(
            &mut self,
            _arena: &mut BufferArena,
            _input: Option<PlanarBufferRef<'_>>,
            mut output: PlanarBufferMut<'_>,
            _time: RenderTime,
        ) -> Result<(), RenderError> {
            output.plane_mut(0)?.fill(0.0);
            Ok(())
        }
        fn take_handover(&mut self) -> Option<ExecutorHandover> {
            self.token.take()
        }
        fn accept_handover(&mut self, handover: ExecutorHandover) -> Option<ExecutorHandover> {
            if self.accepts && self.token.is_none() {
                self.accepted
                    .fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                self.token = Some(handover);
                None
            } else {
                Some(handover)
            }
        }
    }
    fn handover_plan(
        id: u64,
        token: Option<u64>,
        accepts: bool,
        accepted: &std::sync::Arc<core::sync::atomic::AtomicU64>,
    ) -> PreparedRenderPlan {
        PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id: id,
                envelope: RenderEnvelope {
                    sample_rate: SampleRateHz(48_000),
                    quantum: QuantumFrames(2),
                    input_channels: None,
                    output_channels: NonZeroUsize::new(1).expect("one"),
                },
                scratch: &[],
            },
            Box::new(HandoverExecutor {
                token: token.map(|value| Box::new(value) as ExecutorHandover),
                accepts,
                accepted: std::sync::Arc::clone(accepted),
            }),
        )
        .expect("plan")
    }

    /// The block-boundary swap moves an executor-owned resource to the replacement, and gives it
    /// back to the retiring executor when the replacement refuses it.
    ///
    /// Red mutation: delete the hand-over block in `enter_block` -- the replacement never
    /// receives the token and the retiring plan never gets it back.
    #[test]
    fn enter_block_moves_the_executor_handover_and_returns_a_refused_one() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(2).expect("two"),
        };
        let accepted = std::sync::Arc::new(core::sync::atomic::AtomicU64::new(0));
        let load = |value: &std::sync::Arc<core::sync::atomic::AtomicU64>| {
            value.load(core::sync::atomic::Ordering::Relaxed)
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(handover_plan(1, Some(0xfeed), true, &accepted), config)
                .expect("exchange");
        assert!(
            publisher
                .publish(handover_plan(2, None, true, &accepted))
                .is_ok()
        );
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert_eq!(load(&accepted), 1, "the replacement took the hand-over");
        let (_epoch, mut retired) = retirer.try_reclaim().expect("reclaim");
        assert!(
            retired
                .executor_mut()
                .and_then(PreparedPlanExecutor::take_handover)
                .is_none(),
            "the retiring plan gave its resource away"
        );

        // Now a replacement that refuses: the token must come back to the retiring plan.
        assert!(
            publisher
                .publish(handover_plan(3, None, false, &accepted))
                .is_ok()
        );
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::Applied);
        // The refusing replacement never took it; the retiring executor re-accepted its own.
        assert_eq!(load(&accepted), 2);
        let (_epoch, mut refused) = retirer.try_reclaim().expect("reclaim");
        let returned = refused
            .executor_mut()
            .and_then(PreparedPlanExecutor::take_handover)
            .expect("a refused hand-over returns to the retiring executor");
        assert_eq!(*returned.downcast::<u64>().expect("token"), 0xfeed);
    }

    #[test]
    fn exchange_defers_without_retirement_capacity_then_applies() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        assert!(publisher.publish(prepared(2)).is_ok());
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert!(publisher.publish(prepared(3)).is_ok());
        assert_eq!(
            render_once(&mut realtime, 2).swap,
            SwapOutcome::DeferredRetirementFull
        );
        assert_eq!(realtime.active_plan_id(), 2);
        let _old = retirer.try_reclaim().expect("control reclamation");
        let report = render_once(&mut realtime, 4);
        assert_eq!(report.swap, SwapOutcome::Applied);
        assert_eq!(report.active_epoch, PlanEpoch(2));
        assert_eq!(report.render.plan_id, 3);
    }

    #[test]
    fn reserved_replacement_preowns_publication_epoch_and_retirement_credit() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let reservation = publisher
            .reserve_replacement(prepared(2))
            .expect("complete reservation");
        assert_eq!(reservation.epoch(), PlanEpoch(1));
        assert_eq!(reservation.commit(), PlanEpoch(1));
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::Applied);
        assert_eq!(realtime.active_plan_id(), 2);

        assert!(matches!(
            publisher.reserve_replacement(prepared(3)),
            Err(PlanReplacementReservationError::RetirementFull(returned))
                if returned.program().plan_id() == 3
        ));
        let retired = retirer.try_reclaim().expect("reserved retirement");
        assert_eq!(retired.0, PlanEpoch(0));
        let reservation = publisher
            .reserve_replacement(prepared(3))
            .expect("reclaimed credit");
        assert_eq!(reservation.epoch(), PlanEpoch(2));
        reservation.commit();
        assert_eq!(render_once(&mut realtime, 2).swap, SwapOutcome::Applied);
        assert_eq!(realtime.active_plan_id(), 3);
    }

    #[test]
    fn replacement_cancel_releases_both_credits_without_consuming_epoch() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        };
        let (mut publisher, mut realtime, _retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let returned = publisher
            .reserve_replacement(prepared(2))
            .expect("reservation")
            .cancel();
        assert_eq!(returned.program().plan_id(), 2);
        let replacement = publisher
            .reserve_replacement(returned)
            .expect("credits released");
        assert_eq!(replacement.epoch(), PlanEpoch(1));
        drop(replacement);
        assert_eq!(render_once(&mut realtime, 0).swap, SwapOutcome::None);

        let replacement = publisher
            .reserve_replacement(prepared(3))
            .expect("drop released credits");
        assert_eq!(replacement.commit(), PlanEpoch(1));
        assert_eq!(render_once(&mut realtime, 2).active_epoch, PlanEpoch(1));
    }

    #[test]
    fn replacement_reservation_freezes_failure_precedence_and_serial_order() {
        let config = PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(2).expect("two"),
            retirement_capacity: NonZeroUsize::new(2).expect("two"),
        };
        let (mut publisher, mut realtime, mut retirer) =
            plan_exchange(prepared(1), config).expect("exchange");
        let incompatible = PreparedRenderPlan::prepare(PrepareRenderPlan {
            plan_id: 99,
            envelope: RenderEnvelope {
                sample_rate: SampleRateHz(44_100),
                quantum: QuantumFrames(2),
                input_channels: None,
                output_channels: NonZeroUsize::new(1).expect("one"),
            },
            scratch: &[],
        })
        .expect("other envelope");
        assert!(matches!(
            publisher.reserve_replacement(incompatible),
            Err(PlanReplacementReservationError::Incompatible(returned))
                if returned.program().plan_id() == 99
        ));

        publisher
            .reserve_replacement(prepared(2))
            .expect("first")
            .commit();
        publisher
            .reserve_replacement(prepared(3))
            .expect("second")
            .commit();
        assert!(matches!(
            publisher.reserve_replacement(prepared(4)),
            Err(PlanReplacementReservationError::PublicationFull(returned))
                if returned.program().plan_id() == 4
        ));
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        assert_eq!(render_once(&mut realtime, 2).render.plan_id, 3);
        assert_eq!(retirer.try_reclaim().expect("initial").0, PlanEpoch(0));
        assert_eq!(retirer.try_reclaim().expect("second").0, PlanEpoch(1));
    }

    #[test]
    fn reservation_never_strands_a_queued_legacy_predecessor() {
        let (mut publisher, mut realtime, mut retirer) = plan_exchange(
            prepared(1),
            PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(2).expect("two"),
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            },
        )
        .expect("exchange");
        assert!(matches!(publisher.publish(prepared(2)), Ok(PlanEpoch(1))));
        let candidate = match publisher.reserve_replacement(prepared(3)) {
            Err(PlanReplacementReservationError::RetirementFull(candidate)) => candidate,
            _ => panic!("queued legacy predecessor must retain the retirement credit"),
        };
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        assert!(matches!(
            publisher.reserve_replacement(candidate),
            Err(PlanReplacementReservationError::RetirementFull(_))
        ));
        assert_eq!(retirer.try_reclaim().expect("initial").0, PlanEpoch(0));
        publisher
            .reserve_replacement(prepared(3))
            .expect("credit after predecessor reclaim")
            .commit();
        assert_eq!(render_once(&mut realtime, 2).render.plan_id, 3);
    }

    #[test]
    fn reservation_never_strands_a_pending_legacy_predecessor() {
        let (mut publisher, mut realtime, mut retirer) = plan_exchange(
            prepared(1),
            PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(2).expect("two"),
                retirement_capacity: NonZeroUsize::new(1).expect("one"),
            },
        )
        .expect("exchange");
        assert!(publisher.publish(prepared(2)).is_ok());
        assert_eq!(render_once(&mut realtime, 0).render.plan_id, 2);
        assert!(publisher.publish(prepared(3)).is_ok());
        assert_eq!(
            render_once(&mut realtime, 2).swap,
            SwapOutcome::DeferredRetirementFull
        );
        let candidate = match publisher.reserve_replacement(prepared(4)) {
            Err(PlanReplacementReservationError::RetirementFull(candidate)) => candidate,
            _ => panic!("pending legacy predecessor must retain FIFO progress"),
        };
        assert_eq!(retirer.try_reclaim().expect("initial").0, PlanEpoch(0));
        assert_eq!(render_once(&mut realtime, 4).render.plan_id, 3);
        assert!(matches!(
            publisher.reserve_replacement(candidate),
            Err(PlanReplacementReservationError::RetirementFull(_))
        ));
        assert_eq!(retirer.try_reclaim().expect("plan two").0, PlanEpoch(1));
        publisher
            .reserve_replacement(prepared(4))
            .expect("credit after pending predecessor")
            .commit();
        assert_eq!(render_once(&mut realtime, 6).render.plan_id, 4);
        assert_eq!(retirer.try_reclaim().expect("plan three").0, PlanEpoch(2));
        let canceled = publisher
            .reserve_replacement(prepared(5))
            .expect("no leaked predecessor or retirement credit");
        drop(canceled);
    }

    fn render_once(owner: &mut RealtimePlanOwner, sample: u64) -> RealtimeRenderReport {
        let mut output = [1.0_f32; 2];
        let io = RenderIo {
            input: None,
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

    #[test]
    fn launch_sample_rates_prepare_and_render() {
        for (index, rate) in crate::LAUNCH_SAMPLE_RATES.into_iter().enumerate() {
            let mut plan = PreparedRenderPlan::prepare(PrepareRenderPlan {
                plan_id: index as u64,
                envelope: RenderEnvelope {
                    sample_rate: rate,
                    quantum: QuantumFrames(1),
                    input_channels: None,
                    output_channels: NonZeroUsize::new(1).expect("one"),
                },
                scratch: &[],
            })
            .expect("launch rate");
            let mut output = [1.0];
            plan.render(
                RenderIo {
                    input: None,
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
        for rate in crate::EXTENDED_COMPATIBILITY_SAMPLE_RATES
            .into_iter()
            .chain([SampleRateHz(0), SampleRateHz(32_000), SampleRateHz(192_001)])
        {
            assert!(matches!(
                PreparedRenderPlan::prepare(PrepareRenderPlan {
                    plan_id: 0,
                    envelope: RenderEnvelope {
                        sample_rate: rate,
                        quantum: QuantumFrames(1),
                        input_channels: None,
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
                publication_capacity: NonZeroUsize::new(4).expect("publication"),
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
}
