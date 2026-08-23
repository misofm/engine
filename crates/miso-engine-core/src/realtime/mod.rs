//! Realtime-only ownership, storage, and block-boundary publication primitives.
//!
//! `PreparedRenderPlan::render` is deliberately a small, allocation-free surface.  All capacity
//! decisions are made by `prepare`; objects returned here are not synchronizable so exclusive
//! render ownership remains apparent in the type system.

pub mod audit;
mod buffer;
mod parameter;
mod plan;
mod plan_exchange;
mod spsc;

pub use buffer::{
    BufferArena, BufferArenaError, BufferIndex, PlanarBufferMut, PlanarBufferRef, PlanarBufferSpec,
};
pub use parameter::{
    ParameterEvent, ParameterEventBuffer, ParameterEventError, ParameterSlot, ParameterValues,
    PlanEpoch,
};
pub use plan::{
    PrepareRenderPlan, PreparedPlanExecutor, PreparedProgram, PreparedRenderPlan, RenderEnvelope,
    RenderError, RenderIo, RenderReport, RenderTime,
};
pub use plan_exchange::{
    PlanExchangeConfig, PlanPublisher, PlanReplacementReservation, PlanReplacementReservationError,
    PlanRetirer, PublishError, RealtimePlanOwner, RealtimeRenderReport, SwapOutcome, plan_exchange,
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

    #[test]
    fn arena_and_events_are_fixed_and_disjoint() {
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
        let event = ParameterEvent {
            plan_epoch: PlanEpoch(1),
            absolute_sample: 4,
            slot: ParameterSlot(0),
            value: 0.5,
        };
        let mut events = ParameterEventBuffer::with_capacity(1);
        assert!(events.try_push_ordered(event).is_ok());
        assert!(
            matches!(events.try_push_ordered(event), Err(ParameterEventError::Full(returned)) if returned == event)
        );
        assert_eq!(events.overflow_count(), 1);
        assert_eq!(events.capacity(), 1);
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
            parameter_defaults: &[],
            event_capacity: 0,
        })
        .expect("plan")
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
            parameter_defaults: &[],
            event_capacity: 0,
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
                parameter_defaults: &[],
                event_capacity: 0,
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
                    parameter_defaults: &[],
                    event_capacity: 0,
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
