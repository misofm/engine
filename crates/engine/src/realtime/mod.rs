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
    RealtimePlanOwner, RealtimeRenderReport, RealtimeResponseSnapshot, SwapOutcome, plan_exchange,
    plan_exchange_resource_report,
};
pub use spsc::{
    Consumer, Producer, QueueEmpty, QueueFull, QueueGeneration, SpscError, SpscRetainedPayload,
    bounded_spsc, bounded_spsc_move, bounded_spsc_retained_payload,
};

#[cfg(test)]
mod tests {
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
                publication_capacity: NonZeroUsize::new(2).expect("two"),
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
                        .reserve_replacement(oscillator(1, carry, &hooks))
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
                .reserve_replacement(oscillator(1, true, &hooks))
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
        fn hand_over_runs_only_on_the_applied_block() {
            let reference = reference();
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, mut retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            let mut blocks = Vec::new();
            for block in 0..BLOCKS {
                match block {
                    1 => assert!(publisher.publish(oscillator(1, true, &hooks)).is_ok()),
                    2 => assert!(publisher.publish(oscillator(2, true, &hooks)).is_ok()),
                    5 => assert_eq!(retirer.try_reclaim().expect("plan zero").0, PlanEpoch(0)),
                    _ => {}
                }
                let before = hooks.load(Ordering::Relaxed);
                let (output, report) = owner_block(&mut owner);
                blocks.push(output);
                let (swap, carry, calls) = match block {
                    1 | 5 => (SwapOutcome::Applied, CarryOutcome::Carried, 1),
                    2..=4 => (
                        SwapOutcome::DeferredRetirementFull,
                        CarryOutcome::NotRequested,
                        0,
                    ),
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
            assert_eq!(owner.active_epoch(), PlanEpoch(2));
            assert_eq!((owner.carried_count(), owner.deferred_count()), (2, 3));
        }

        #[test]
        fn dropping_the_owner_never_hands_over_to_an_unapplied_candidate() {
            let hooks = Arc::new(AtomicUsize::new(0));
            let (mut publisher, mut owner, _retirer) =
                plan_exchange(oscillator(0, true, &hooks), config(1)).expect("exchange");
            owner_block(&mut owner);
            publisher
                .reserve_replacement(oscillator(1, true, &hooks))
                .expect("reserve")
                .commit();
            drop(owner);
            assert_eq!(hooks.load(Ordering::Relaxed), 0);
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
