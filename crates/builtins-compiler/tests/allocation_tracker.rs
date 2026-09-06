//! Independent phase-two layout accounting for the sealed builtin artifact.

#![cfg(feature = "test-support")]
#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    cell::Cell,
    num::{NonZeroU32, NonZeroU64, NonZeroUsize},
};
use std::alloc::{GlobalAlloc, System};
use std::sync::Mutex;

use builtins::{BuiltinLaneSelector, Matrix2x2, MeterConfig, MeterTap};
use builtins_compiler::{
    BuiltinCompileCaps, MeterRequest, TestOnlyFaderMatrixPair, TrackControlRecord,
    TrackFaderRecord, prepare_session_builtins, test_only_fader_matrix_witness,
    test_only_observed_scalar_pair_binding, test_only_phase_two_allocation_snapshot,
    test_only_record_phase_two_allocation, test_only_record_phase_two_deallocation,
    test_only_reset_fader_matrix_witness, test_only_reset_phase_two_allocation_tracker,
    test_only_scalar_owner_drops, test_only_scalar_owner_layouts,
};
use engine::realtime::{PlanarBufferMut, RenderIo, RenderTime};
use session::{CompileCaps, RouteSource, SendTap, StableId, compile_session, parse_session_json};

/// Serializes the two tests below (each measures the process-global phase-two tracker) so no
/// other test in this binary's use of `TEST_PHASE_TWO_ACTIVE`/`TEST_PHASE_TWO_LAYOUTS` interleaves
/// with either measurement session. `builtins_compiler` exposes no session primitive of its own
/// (that global state is intentionally private -- only the reset/record/snapshot triplet is
/// `test_only_*` public), so ownership of this lock is entirely test-file-local. Poison-tolerant:
/// a panicking test must not permanently deadlock the other test in this binary.
static SESSION: Mutex<()> = Mutex::new(());

struct TrackingAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: TrackingAllocator = TrackingAllocator;

thread_local! {
    /// Set true only on the measuring thread, for the span of one `armed` call (one warm-up or
    /// measured `prepare_session_builtins`), by [`ArmedGuard`]. `TrackingAllocator` forwards an
    /// allocation to `test_only_record_phase_two_allocation` only when the *current* thread's
    /// flag is set, so an allocation made concurrently by any other thread (a background worker
    /// spawned by a dependency, for example) is never attributed to this measurement -- without
    /// calling `std::thread::current()` from inside the allocator hook: that lazily allocates a
    /// `Thread` handle on first use per thread and clones an `Arc` on every subsequent call,
    /// which would itself be a reentrant allocation from inside a `GlobalAlloc::alloc` hook.
    static ARMED: Cell<bool> = const { Cell::new(false) };
    static LIVE_ALLOCS: Cell<u64> = const { Cell::new(0) };
    static LIVE_FREES: Cell<u64> = const { Cell::new(0) };
}

/// Guards one armed span on the current thread; clears the flag on drop (including on panic), so
/// a failed assertion inside `body` can never leave a later, unrelated allocation on the same
/// thread attributed to a stale window.
struct ArmedGuard;

impl ArmedGuard {
    fn new() -> Self {
        ARMED.with(|flag| flag.set(true));
        Self
    }
}

impl Drop for ArmedGuard {
    fn drop(&mut self) {
        ARMED.with(|flag| flag.set(false));
    }
}

/// Runs `body` -- a single `prepare_session_builtins` call -- with this thread armed, so
/// `TrackingAllocator` attributes only this thread's allocations to the phase-two tracker for the
/// span of the call.
fn armed<T>(body: impl FnOnce() -> T) -> T {
    let _guard = ArmedGuard::new();
    body()
}

// SAFETY: this delegates every request unchanged and records only a fixed atomic counter in the
// compiler crate while its explicit test-only phase-two guard is active.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if ARMED.with(Cell::get) {
            LIVE_ALLOCS.set(LIVE_ALLOCS.get() + 1);
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if ARMED.with(Cell::get) {
            LIVE_ALLOCS.set(LIVE_ALLOCS.get() + 1);
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if ARMED.with(Cell::get) {
            LIVE_FREES.set(LIVE_FREES.get() + 1);
            test_only_record_phase_two_deallocation(layout);
        }
        // SAFETY: forwards the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if ARMED.with(Cell::get) {
            // Count realloc as allocation activity as well. A realloc may release the old
            // layout internally, so the free counter is conservatively incremented too; this
            // keeps the zero gate from being bypassed by a direct realloc call.
            LIVE_ALLOCS.set(LIVE_ALLOCS.get() + 1);
            LIVE_FREES.set(LIVE_FREES.get() + 1);
            test_only_record_phase_two_deallocation(layout);
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

#[test]
fn actual_serialized_composite_render_allocates_and_frees_nothing() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut pair = TestOnlyFaderMatrixPair::new_ramping();
    let mut left = [0.25_f32; 4 * 8];
    let mut right = [-0.5_f32; 4 * 8];

    let mut probe = Vec::<u8>::with_capacity(core::hint::black_box(64));
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| probe.reserve_exact(core::hint::black_box(128)));
    assert!(LIVE_ALLOCS.get() > 0, "realloc allocation liveness");
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| drop(probe));
    assert_eq!(LIVE_ALLOCS.get(), 0, "drop is not allocation activity");
    assert!(LIVE_FREES.get() > 0, "independent free liveness");

    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| {
        for _ in 0..32 {
            pair.process(&mut left, &mut right, 8);
        }
    });
    assert_eq!((LIVE_ALLOCS.get(), LIVE_FREES.get()), (0, 0));
}

fn audit_graph_render(
    bound: &mut builtins_compiler::PreparedBuiltinsGraphBound,
    output: &mut [f32],
    absolute_sample: u64,
) -> builtins_compiler::TestOnlyFaderMatrixWitness {
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    test_only_reset_fader_matrix_witness();
    armed(|| {
        bound
            .plan
            .render(
                RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(output, 2, 64, 64).expect("output"),
                },
                RenderTime { absolute_sample },
            )
            .expect("actual graph render");
    });
    let activity = (LIVE_ALLOCS.get(), LIVE_FREES.get());
    let witness = test_only_fader_matrix_witness();
    assert_eq!(
        activity,
        (0, 0),
        "actual queued graph render allocation/free gate"
    );
    witness
}

#[test]
fn actual_queued_graph_phases_allocate_and_free_nothing() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut output = [0.0_f32; 128];
    test_only_reset_fader_matrix_witness();
    let mut eligible = builtins_compiler::test_only_prepared_pair_graph(false);
    let prepared = test_only_fader_matrix_witness();
    assert_eq!((prepared.factory_calls, prepared.factory_members), (1, 1));

    let settled = audit_graph_render(&mut eligible, &mut output, 0);
    assert_eq!((settled.fused_calls, settled.fallback_calls), (1, 0));
    assert_eq!(settled.process_members, 1);

    {
        let tail = eligible
            .track_controls
            .iter_mut()
            .find(|control| control.track_id.as_ref() == "t08")
            .expect("eligible tail controls");
        tail.fader
            .try_push(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: -12.0,
                smoothing_samples: 3,
            })
            .unwrap();
        tail.producer
            .try_push(TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.5,
                    lr: 0.25,
                    rl: -0.25,
                    rr: 0.75,
                },
                smoothing_samples: 3,
            })
            .unwrap();
    }
    let ramp_a = audit_graph_render(&mut eligible, &mut output, 64);
    assert_eq!((ramp_a.fused_calls, ramp_a.fallback_calls), (0, 1));
    let ramp_b = audit_graph_render(&mut eligible, &mut output, 128);
    assert_eq!((ramp_b.fused_calls, ramp_b.fallback_calls), (1, 0));

    eligible
        .track_controls
        .iter_mut()
        .find(|control| control.track_id.as_ref() == "t08")
        .expect("eligible tail controls")
        .fader
        .try_push(TrackFaderRecord::FaderDb {
            lanes: BuiltinLaneSelector::Left,
            db: -3.0,
            smoothing_samples: 2,
        })
        .unwrap();
    let retarget = audit_graph_render(&mut eligible, &mut output, 192);
    assert_eq!((retarget.fused_calls, retarget.fallback_calls), (0, 1));
    let resettled = audit_graph_render(&mut eligible, &mut output, 256);
    assert_eq!((resettled.fused_calls, resettled.fallback_calls), (1, 0));

    test_only_reset_fader_matrix_witness();
    let mut observed = builtins_compiler::test_only_prepared_pair_graph(true);
    let observed_prepared = test_only_fader_matrix_witness();
    assert_eq!(
        (
            observed_prepared.factory_calls,
            observed_prepared.factory_members
        ),
        (1, 1),
        "the observed full cohort declines while the tail is prepared"
    );
    let observed_call = audit_graph_render(&mut observed, &mut output, 0);
    assert_eq!(
        (observed_call.fused_calls, observed_call.fallback_calls),
        (1, 0)
    );
    assert_eq!(observed_call.process_members, 1);
    let meter = observed
        .meter_consumers
        .first_mut()
        .expect("observed cohort meter");
    assert!(
        meter.consumer.try_pop().is_ok(),
        "meter drains only after disarming"
    );
}

#[test]
fn actual_queued_scalar_graph_allocates_and_frees_nothing() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    let mut liveness = Vec::<u8>::with_capacity(core::hint::black_box(32));
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| liveness.reserve_exact(core::hint::black_box(64)));
    assert!(LIVE_ALLOCS.get() > 0, "scalar audit allocation liveness");
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| drop(liveness));
    assert!(LIVE_FREES.get() > 0, "scalar audit free liveness");

    let mut output = [0.0_f32; 128];
    test_only_reset_fader_matrix_witness();
    let mut selected = builtins_compiler::test_only_prepared_scalar_pair_graph(false);
    assert_eq!(
        (
            test_only_fader_matrix_witness().factory_calls,
            test_only_fader_matrix_witness().factory_members
        ),
        (1, 1)
    );
    let settled = audit_graph_render(&mut selected, &mut output, 0);
    assert_eq!((settled.fused_calls, settled.fallback_calls), (1, 0));
    {
        let scalar = selected
            .track_controls
            .iter_mut()
            .find(|control| control.track_id.as_ref() == "t01")
            .expect("selected scalar controls");
        scalar
            .fader
            .try_push(TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: -12.0,
                smoothing_samples: 96,
            })
            .unwrap();
        scalar
            .producer
            .try_push(TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.5,
                    lr: 0.25,
                    rl: -0.25,
                    rr: 0.75,
                },
                smoothing_samples: 96,
            })
            .unwrap();
    }
    for (sample, expected) in [(64, (0, 1)), (128, (0, 1)), (192, (1, 0))] {
        let witness = audit_graph_render(&mut selected, &mut output, sample);
        assert_eq!((witness.fused_calls, witness.fallback_calls), expected);
        assert_eq!(witness.process_members, 1);
    }

    test_only_reset_fader_matrix_witness();
    let mut observed = builtins_compiler::test_only_prepared_scalar_pair_graph(true);
    let observed_call = audit_graph_render(&mut observed, &mut output, 0);
    assert_eq!(
        (observed_call.process_calls, observed_call.factory_calls),
        (0, 0),
        "the actual observed scalar graph retains separate owners"
    );
    assert!(
        observed.meter_consumers[0].consumer.try_pop().is_ok(),
        "observed fallback publishes its meter window"
    );
}

#[test]
fn actual_scalar_prepare_and_bind_retain_the_charged_owner_layouts() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let [fader, matrix, outer] = test_only_scalar_owner_layouts();
    let owner_allowance = 2 * fader.size_bytes + 2 * matrix.size_bytes + outer.size_bytes;

    test_only_reset_phase_two_allocation_tracker();
    let (bound, admitted, scalar_allowance, preparation, binding) =
        armed(test_only_observed_scalar_pair_binding);
    assert!(!preparation.overflowed);
    let retained_owner_bytes: u64 = [fader, matrix]
        .iter()
        .map(|expected| {
            preparation
                .layouts
                .iter()
                .find(|observed| {
                    observed.size_bytes == expected.size_bytes
                        && observed.align_bytes == expected.align_bytes
                })
                .map_or(0, |observed| {
                    observed.size_bytes * observed.allocation_count
                })
        })
        .sum();
    assert_eq!(
        retained_owner_bytes,
        2 * fader.size_bytes + 2 * matrix.size_bytes
    );
    for expected in [fader, matrix] {
        assert!(
            preparation.layouts.iter().any(|observed| {
                observed.size_bytes == expected.size_bytes
                    && observed.align_bytes == expected.align_bytes
                    && observed.allocation_count == 2
            }),
            "both original owners are actual retained preparation allocations: expected={expected:?}, observed={:?}",
            preparation.layouts
        );
    }
    assert!(!binding.overflowed);
    for retained in [fader, matrix] {
        assert!(
            !binding.deallocation_layouts.iter().any(|released| {
                released.size_bytes == retained.size_bytes
                    && released.align_bytes == retained.align_bytes
            }),
            "binding retains each original owner without free/reallocate substitution"
        );
    }
    assert!(
        binding.layouts.iter().any(|observed| {
            observed.size_bytes == outer.size_bytes
                && observed.align_bytes == outer.align_bytes
                && observed.allocation_count >= 1
        }),
        "binding allocates the charged two-pointer scalar outer: {:?}",
        binding.layouts
    );
    let bound_outer_bytes = binding
        .layouts
        .iter()
        .find(|observed| {
            observed.size_bytes == outer.size_bytes && observed.align_bytes == outer.align_bytes
        })
        .map_or(0, |observed| {
            observed.size_bytes * observed.allocation_count
        });
    assert!(bound_outer_bytes >= outer.size_bytes);
    assert_eq!(retained_owner_bytes + outer.size_bytes, owner_allowance);
    assert!(binding.largest_allocation_bytes >= outer.size_bytes);
    assert!(outer.size_bytes <= admitted.largest_allocation_bytes);
    assert!(owner_allowance <= scalar_allowance.total_bytes);
    assert!(scalar_allowance.total_bytes <= admitted.session_plus_plan_bytes);

    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    test_only_reset_fader_matrix_witness();
    armed(|| drop(bound));
    assert_eq!(LIVE_ALLOCS.get(), 0, "off-render release does not allocate");
    assert!(
        LIVE_FREES.get() > 0,
        "bound owners release only during off-render drop"
    );
    assert_eq!(test_only_scalar_owner_drops(), [2, 2, 1]);
}

fn session(track_count: u32) -> session::CompiledSession {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("fixture parse");
    let mut template = model.tracks[0].clone();
    template.simd1.effects.clear();
    template.dynamic.effects.clear();
    template.simd2.effects.clear();
    model.automation.clear();
    model.tracks.clear();
    model
        .tracks
        .reserve(usize::try_from(track_count).expect("u32 fits usize on supported targets"));
    for index in 0..track_count {
        let mut track = template.clone();
        track.id = StableId::parse(&format!("track-{index}")).expect("generated stable ID");
        model.tracks.push(track);
    }
    model.routes[0].source = RouteSource::Track {
        track_id: StableId::parse("track-0").expect("route track"),
        tap: SendTap::PostMatrix,
    };
    compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("fixture compile")
}

fn requests(count: usize) -> Vec<MeterRequest> {
    let config = MeterConfig {
        period_frames: NonZeroU32::new(128).expect("constant"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(4).expect("constant"),
        reset_generation: 7,
    };
    [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ][..count]
        .iter()
        .copied()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: builtins::MeterHandle(
                NonZeroU64::new(u64::try_from(index).expect("bounded") + 1).expect("nonzero"),
            ),
            track_id: "track-0".to_owned(),
            tap,
            config,
        })
        .collect()
}

fn assert_zero_phase_two_allocations() {
    let snapshot = test_only_phase_two_allocation_snapshot();
    assert_eq!(snapshot.total_bytes, 0);
    assert_eq!(snapshot.largest_allocation_bytes, 0);
    assert_eq!(snapshot.allocation_count, 0);
    assert!(snapshot.layouts.is_empty());
    assert_eq!(snapshot.deallocation_count, 0);
    assert!(snapshot.deallocation_layouts.is_empty());
    assert!(!snapshot.overflowed);
}

fn assert_rejects_in_phase_one(
    session: &session::CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) {
    test_only_reset_phase_two_allocation_tracker();
    let error = armed(|| prepare_session_builtins(session, requests, caps))
        .err()
        .expect("one-below cap must reject");
    assert!(
        error
            .0
            .iter()
            .all(|diagnostic| diagnostic.code == "builtin.resource.limit")
    );
    assert_zero_phase_two_allocations();
}

fn caps() -> BuiltinCompileCaps {
    BuiltinCompileCaps {
        maximum_total_state_bytes: u64::MAX,
        maximum_total_retained_payload_bytes: u64::MAX,
        maximum_total_meter_items: u64::MAX,
        maximum_total_meter_bytes: u64::MAX,
        maximum_single_allocation_bytes: u64::MAX,
        maximum_meter_streams: u64::MAX,
        maximum_period_frames: u32::MAX,
        maximum_peak_hold_frames: u32::MAX,
        maximum_smoothing_samples: u32::MAX,
    }
}

/// Settle first-touch process/thread state (lazily-initialised std machinery reachable from
/// [`prepare_session_builtins`], such as thread handles and `HashMap` `RandomState` seeding)
/// with one full, discarded prepare, so that one-shot cost can never land inside a *measured*
/// phase-two window. See the diagnosis on `TEST_PHASE_TWO_LAYOUTS` in `src/lib.rs`: the #358 CI
/// failure showed four extra retained layouts, one allocation each, present only on the first and
/// only prepare in the process.
///
/// The discarded snapshot is returned, not thrown away: callers must characterise it with
/// [`assert_first_touch_delta_is_bounded`] against the first steady-state measured snapshot taken
/// with the same session/requests/caps, so a retained engine allocation of meaningful size hiding
/// on the first prepare still fails the test instead of being silently absorbed by the warm-up.
fn settle_phase_two_first_touch(
    session: &session::CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) -> builtins_compiler::TestPhaseTwoAllocationSnapshot {
    test_only_reset_phase_two_allocation_tracker();
    let _ = armed(|| prepare_session_builtins(session, requests, caps)).expect("prepare (warm-up)");
    test_only_phase_two_allocation_snapshot()
}

/// Characterises what the first-touch warm-up in [`settle_phase_two_first_touch`] discards: the
/// warm-up's layout multiset must be a superset of the steady-state measured one (nothing the
/// measured prepare allocated can be missing from the warm-up), and the difference (warm-up minus
/// measured) must consist only of one-off allocations (`allocation_count == 1` per layout class)
/// totalling at most 4096 bytes. This turns "discarded" into "characterised as small first-touch
/// one-offs": a retained engine allocation of meaningful size that only shows up on the first
/// prepare now fails here instead of vanishing with the warm-up snapshot.
fn assert_first_touch_delta_is_bounded(
    warmup: &builtins_compiler::TestPhaseTwoAllocationSnapshot,
    measured: &builtins_compiler::TestPhaseTwoAllocationSnapshot,
) {
    for measured_layout in &measured.layouts {
        let warmup_count = warmup
            .layouts
            .iter()
            .find(|candidate| {
                candidate.size_bytes == measured_layout.size_bytes
                    && candidate.align_bytes == measured_layout.align_bytes
            })
            .map_or(0, |candidate| candidate.allocation_count);
        assert!(
            warmup_count >= measured_layout.allocation_count,
            "the warm-up's layout multiset must be a superset of the measured one: measured has \
             {measured_layout:?} but the warm-up only has {warmup_count} of that layout class \
             (warm-up={:?}, measured={:?})",
            warmup.layouts,
            measured.layouts
        );
    }

    let mut difference = Vec::new();
    let mut difference_bytes = 0_u64;
    for warmup_layout in &warmup.layouts {
        let measured_count = measured
            .layouts
            .iter()
            .find(|candidate| {
                candidate.size_bytes == warmup_layout.size_bytes
                    && candidate.align_bytes == warmup_layout.align_bytes
            })
            .map_or(0, |candidate| candidate.allocation_count);
        let extra = warmup_layout
            .allocation_count
            .saturating_sub(measured_count);
        if extra > 0 {
            difference.push(builtins_compiler::BuiltinRetainedLayout {
                size_bytes: warmup_layout.size_bytes,
                align_bytes: warmup_layout.align_bytes,
                allocation_count: extra,
            });
            difference_bytes += warmup_layout.size_bytes.saturating_mul(extra);
        }
    }

    assert!(
        difference.iter().all(|layout| layout.allocation_count == 1),
        "the discarded first-touch delta must consist only of one-off allocations \
         (allocation_count == 1 per layout class): difference={difference:?}"
    );
    assert!(
        difference_bytes <= 4096,
        "the discarded first-touch delta must total at most 4096 bytes: difference={difference:?} \
         total_bytes={difference_bytes}"
    );
}

#[test]
fn phase_two_allocator_layouts_match_the_checked_resource_report() {
    // Excludes every other test in this binary from the process-global tracker for the whole
    // measurement session (reset through snapshot); see `SESSION`.
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // One discarded prepare, ahead of every measured combination below, settles first-touch
    // process/thread state once so no measured window ever observes it. Its snapshot is not
    // thrown away: the very first measured combination below repeats the same
    // session/requests/caps, and the two are compared by `assert_first_touch_delta_is_bounded`.
    let warmup_snapshot = settle_phase_two_first_touch(&session(1), &requests(0), caps());
    let mut first_touch_delta_checked = false;
    for track_count in [1, 4, 65_537] {
        let session = session(track_count);
        for meter_count in [0, 1, 7] {
            let requests = requests(meter_count);
            test_only_reset_phase_two_allocation_tracker();
            let prepared =
                armed(|| prepare_session_builtins(&session, &requests, caps())).expect("prepare");
            let snapshot = test_only_phase_two_allocation_snapshot();
            let report = prepared.resource_report();
            assert!(!snapshot.overflowed);
            if !first_touch_delta_checked {
                assert_first_touch_delta_is_bounded(&warmup_snapshot, &snapshot);
                first_touch_delta_checked = true;
            }
            assert_eq!(
                snapshot.total_bytes,
                report.engine_owned_retained_payload_bytes,
                "tracks={track_count}, meters={meter_count}, observed={:?}, reported={:?}",
                snapshot.layouts,
                report.retained_layouts()
            );
            assert_eq!(
                snapshot.largest_allocation_bytes, report.maximum_single_allocation_bytes,
                "tracks={track_count}, meters={meter_count}"
            );
            assert_eq!(
                snapshot.allocation_count, report.retained_allocation_count,
                "tracks={track_count}, meters={meter_count}"
            );
            assert_eq!(
                snapshot.layouts,
                report.retained_layouts(),
                "tracks={track_count}, meters={meter_count}"
            );

            let mut exact = caps();
            exact.maximum_total_state_bytes = report.engine_owned_processor_payload_bytes;
            exact.maximum_total_retained_payload_bytes = report.engine_owned_retained_payload_bytes;
            exact.maximum_total_meter_items = report.meter_items.max(1);
            exact.maximum_total_meter_bytes = report.engine_owned_meter_payload_bytes.max(1);
            exact.maximum_single_allocation_bytes = report.maximum_single_allocation_bytes;
            exact.maximum_meter_streams = u64::try_from(meter_count).expect("bounded").max(1);
            armed(|| prepare_session_builtins(&session, &requests, exact))
                .expect("equal caps accept");

            let mut below = exact;
            below.maximum_total_state_bytes = report
                .engine_owned_processor_payload_bytes
                .checked_sub(1)
                .expect("processor payload is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            let mut below = exact;
            below.maximum_total_retained_payload_bytes = report
                .engine_owned_retained_payload_bytes
                .checked_sub(1)
                .expect("retained payload is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            let mut below = exact;
            below.maximum_single_allocation_bytes = report
                .maximum_single_allocation_bytes
                .checked_sub(1)
                .expect("largest allocation is nonzero");
            assert_rejects_in_phase_one(&session, &requests, below);

            if meter_count > 0 {
                let mut below = exact;
                below.maximum_total_meter_items = report
                    .meter_items
                    .checked_sub(1)
                    .expect("meter item payload is nonzero");
                assert_rejects_in_phase_one(&session, &requests, below);

                let mut below = exact;
                below.maximum_total_meter_bytes = report
                    .engine_owned_meter_payload_bytes
                    .checked_sub(1)
                    .expect("meter payload is nonzero");
                assert_rejects_in_phase_one(&session, &requests, below);

                let mut below = exact;
                below.maximum_meter_streams = u64::try_from(meter_count)
                    .expect("bounded")
                    .checked_sub(1)
                    .expect("positive meter count");
                if below.maximum_meter_streams > 0 {
                    assert_rejects_in_phase_one(&session, &requests, below);
                }
            }
        }
    }
    assert!(
        first_touch_delta_checked,
        "the first-touch delta check must have run against the first measured combination"
    );
}

/// Repeats a measured prepare three times in-process (after the same first-touch warm-up) and
/// asserts every observed snapshot is byte-for-byte identical. This is the direct in-process
/// analogue of "passes in isolation, fails in the workspace run": if any thread-local or
/// process-global first-touch state, or any cross-thread contamination, remained, one of these
/// three measured prepares would diverge from the others.
#[test]
fn phase_two_allocator_layouts_are_stable_across_repeated_measured_prepares() {
    // Excludes every other test in this binary from the process-global tracker for the whole
    // measurement session (reset through snapshot); see `SESSION`.
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let session = session(4);
    let requests = requests(3);
    let warmup_snapshot = settle_phase_two_first_touch(&session, &requests, caps());

    let mut snapshots = Vec::with_capacity(3);
    for _ in 0..3 {
        test_only_reset_phase_two_allocation_tracker();
        let _ = armed(|| prepare_session_builtins(&session, &requests, caps()))
            .expect("prepare (measured)");
        snapshots.push(test_only_phase_two_allocation_snapshot());
    }

    let first = &snapshots[0];
    assert!(
        !first.overflowed && !first.layouts.is_empty(),
        "measured snapshot must be a real, non-overflowed observation: {first:?}"
    );
    for (index, snapshot) in snapshots.iter().enumerate().skip(1) {
        assert_eq!(
            snapshot, first,
            "measured prepare #{index} observed different phase-two layouts than #0"
        );
    }
    assert_first_touch_delta_is_bounded(&warmup_snapshot, first);
}
