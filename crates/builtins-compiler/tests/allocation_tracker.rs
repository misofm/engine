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
    BuiltinCompileCaps, LiveValuesSuperseded, MeterRequest, TEST_ONLY_PAIR_GRAPH_TRACKS,
    TrackControlRecord, TrackControlRequest, TrackFaderRecord, TrackInputRecord,
    prepare_session_builtins, prepare_session_builtins_with_live_controls,
    test_only_begin_phase_two_allocation_observation, test_only_fader_matrix_witness,
    test_only_phase_two_allocation_snapshot, test_only_record_phase_two_allocation,
    test_only_record_phase_two_deallocation, test_only_reset_fader_matrix_witness,
    test_only_reset_phase_two_allocation_tracker,
};
use engine::realtime::{LiveSnapshot, PlanarBufferMut, RenderIo, RenderTime};
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
    static LIVE_BYTES: Cell<u64> = const { Cell::new(0) };
    static LARGEST_REQUESTED_BYTES: Cell<u64> = const { Cell::new(0) };
    static LARGEST_REALLOC_BYTES: Cell<u64> = const { Cell::new(0) };
    static BYTE_COUNTER_FAILED: Cell<bool> = const { Cell::new(false) };
}

fn reset_byte_counters() {
    LIVE_BYTES.set(0);
    LARGEST_REQUESTED_BYTES.set(0);
    LARGEST_REALLOC_BYTES.set(0);
    BYTE_COUNTER_FAILED.set(false);
}

fn observe_allocation_bytes(bytes: usize) {
    let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
    match LIVE_BYTES.get().checked_add(bytes) {
        Some(live) => LIVE_BYTES.set(live),
        None => BYTE_COUNTER_FAILED.set(true),
    }
    LARGEST_REQUESTED_BYTES.set(LARGEST_REQUESTED_BYTES.get().max(bytes));
}

fn observe_deallocation_bytes(bytes: usize) {
    let bytes = u64::try_from(bytes).unwrap_or(u64::MAX);
    match LIVE_BYTES.get().checked_sub(bytes) {
        Some(live) => LIVE_BYTES.set(live),
        None => BYTE_COUNTER_FAILED.set(true),
    }
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
            observe_allocation_bytes(layout.size());
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if ARMED.with(Cell::get) {
            LIVE_ALLOCS.set(LIVE_ALLOCS.get() + 1);
            observe_allocation_bytes(layout.size());
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the allocator-provided layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if ARMED.with(Cell::get) {
            LIVE_FREES.set(LIVE_FREES.get() + 1);
            observe_deallocation_bytes(layout.size());
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
            observe_deallocation_bytes(layout.size());
            let new_layout = Layout::from_size_align(size, layout.align())
                .expect("GlobalAlloc::realloc receives a valid new layout");
            let new_bytes = u64::try_from(size).unwrap_or(u64::MAX);
            LARGEST_REALLOC_BYTES.set(LARGEST_REALLOC_BYTES.get().max(new_bytes));
            observe_allocation_bytes(new_layout.size());
            test_only_record_phase_two_deallocation(layout);
            test_only_record_phase_two_allocation(layout);
        }
        // SAFETY: forwards the original allocation arguments unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
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

struct RuntimeBankPlanes {
    left: Vec<Vec<f32>>,
    right: Vec<Vec<f32>>,
}

impl RuntimeBankPlanes {
    fn new(lanes: usize) -> Self {
        Self {
            left: (0..lanes).map(|lane| vec![lane as f32 + 1.0]).collect(),
            right: (0..lanes).map(|lane| vec![lane as f32 + 1.0]).collect(),
        }
    }
}

impl rack::BankMembers for RuntimeBankPlanes {
    fn plane(&self, lane: usize) -> (&[f32], &[f32]) {
        (&self.left[lane], &self.right[lane])
    }

    fn plane_mut(&mut self, lane: usize) -> (&mut [f32], &mut [f32]) {
        (&mut self.left[lane], &mut self.right[lane])
    }
}

fn audit_runtime_bank_render(
    chain: &mut rack::BankChain,
    planes: &mut RuntimeBankPlanes,
    first_sample: u64,
) {
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    armed(|| {
        chain
            .run(planes, 1, first_sample, LiveSnapshot::ALL)
            .expect("runtime bank render")
    });
    assert_eq!(
        (LIVE_ALLOCS.get(), LIVE_FREES.get()),
        (0, 0),
        "actual mono-collapsed/fallback bank render allocation/free gate"
    );
}

/// Fader and matrix values drained into a running queued graph allocate and free nothing: a
/// settled block, a fader and matrix ramp on the one-lane tail, a fader retarget inside it, and
/// the block after it.
#[test]
fn actual_queued_graph_phases_allocate_and_free_nothing() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut output = [0.0_f32; 128];
    let mut graph = builtins_compiler::test_only_prepared_unpaired_graph();
    // The graph is one full bank of the build's own width and a one-lane tail: nine tracks on an
    // 8-lane (AVX2) build, five on a 4-lane one (issue #1112).
    let tail_id = format!("t{:02}", TEST_ONLY_PAIR_GRAPH_TRACKS - 1);

    audit_graph_render(&mut graph, &mut output, 0);

    graph.test_only_write(|controls, stamp| {
        let tail = controls
            .iter_mut()
            .find(|control| control.track_id.as_ref() == tail_id)
            .expect("tail controls");
        tail.fader.write(
            stamp,
            TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Both,
                db: -12.0,
                smoothing_samples: 3,
            },
        );
        tail.producer.write(
            stamp,
            TrackControlRecord {
                matrix: Matrix2x2 {
                    ll: 0.5,
                    lr: 0.25,
                    rl: -0.25,
                    rr: 0.75,
                },
                smoothing_samples: 3,
            },
        );
    });
    let ramp = audit_graph_render(&mut graph, &mut output, 64);
    assert_eq!(
        (ramp.fader_values_applied, ramp.matrix_values_applied),
        (1, 1),
        "the ramp block drained both values"
    );
    audit_graph_render(&mut graph, &mut output, 128);

    graph.test_only_write(|controls, stamp| {
        let tail = controls
            .iter_mut()
            .find(|control| control.track_id.as_ref() == tail_id)
            .expect("tail controls");
        tail.fader.write(
            stamp,
            TrackFaderRecord::FaderDb {
                lanes: BuiltinLaneSelector::Left,
                db: -3.0,
                smoothing_samples: 2,
            },
        );
    });
    let retarget = audit_graph_render(&mut graph, &mut output, 192);
    assert_eq!(retarget.fader_values_applied, 1, "the retarget was drained");
    audit_graph_render(&mut graph, &mut output, 256);
}

/// Input trim values drained into a running graph allocate and free nothing, through the banked
/// drain (`BuiltinBankProcessor::begin_block`) of the fixture's full cohort (eight lanes on an
/// 8-lane (AVX2) build, four on a 4-lane one) and its one-lane tail.
///
/// Every track's input cells receive a `TrimDb { Both, -6 / -12 dB, 256 }` on each of four
/// blocks, so each drain retargets a ramp still in flight; the uncommanded twin shows the records
/// took effect, so an allocation-free pass cannot come from a drain that did nothing.
///
/// Ported by #1027 from the #600 input-trim qualification (`tools/bench/src/input_symmetry.rs`,
/// `qualification_phase_constants_and_zero_render_allocations`), retired with its capture
/// tooling. The fader and matrix cells are gated by the test above; before this, the input
/// cells' `TrimDb` arm had no allocation gate in CI.
///
/// Red mutation: allocate in the banked drain's `TrimDb` arm (`vec![smoothing_samples][0]` for
/// the window) -> the first ridden block counts nine allocations and nine frees, one per track.
#[test]
fn actual_input_trim_cell_drain_allocates_and_frees_nothing() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut ridden = builtins_compiler::test_only_prepared_unpaired_graph();
    let mut quiet = builtins_compiler::test_only_prepared_unpaired_graph();
    assert_eq!(
        ridden.track_controls.len(),
        TEST_ONLY_PAIR_GRAPH_TRACKS,
        "a full cohort and a one-lane tail"
    );
    let mut ridden_output = [0.0_f32; 128];
    let mut quiet_output = [0.0_f32; 128];
    let mut moved = false;
    for block in 0..4_u64 {
        let db = if block % 2 == 0 { -6.0 } else { -12.0 };
        ridden.test_only_write(|controls, stamp| {
            for control in controls {
                control.input.as_mut().expect("input lane requested").write(
                    stamp,
                    TrackInputRecord::TrimDb {
                        lanes: BuiltinLaneSelector::Both,
                        db,
                        smoothing_samples: 256,
                    },
                );
            }
        });
        audit_graph_render(&mut ridden, &mut ridden_output, block * 64);
        audit_graph_render(&mut quiet, &mut quiet_output, block * 64);
        moved |= ridden_output != quiet_output;
    }
    assert!(
        moved,
        "the trim records never moved the output, so the drain they went through proves nothing"
    );
}

/// #1765 gate 1: a render-time swap of two strips' builtin lanes moves each strip's whole state
/// ([`builtins_compiler::test_only_builtin_lane_swap`] holds the scenario and its bit checks).
///
/// Red if a swap leaves a per-lane field behind (stage state or ramp, cell reader, live symmetry
/// term, stored automation), if a builtin stage refuses a swap it should make, or if an automated
/// lane is let into a fader bank without automation slots. The allocation claim is
/// `tests/lane_swap_alloc.rs`.
#[test]
fn a_builtin_lane_swap_moves_the_whole_strip() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let swaps = builtins_compiler::test_only_builtin_lane_swap(&mut |swap| swap());
    assert!(
        swaps >= 16,
        "four passes of four swaps at one width at least"
    );
}

#[test]
fn actual_runtime_bank_slot_owners_fit_retained_largest_and_conversion_reservation() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);

    // Positive liveness control for the NEW realloc request. The phase-two recorder intentionally
    // retains its old-layout semantics; this test-local counter observes the requested new size.
    let old = Layout::from_size_align(8, 8).expect("old realloc layout");
    let new = Layout::from_size_align(64, 8).expect("new realloc layout");
    reset_byte_counters();
    LIVE_ALLOCS.set(0);
    LIVE_FREES.set(0);
    // SAFETY: `old` is nonzero and valid; the returned pointer is checked before reuse.
    let pointer = armed(|| unsafe { std::alloc::alloc(old) });
    assert!(!pointer.is_null(), "controlled realloc allocation");
    // SAFETY: `pointer` came from `alloc(old)`, remains live, and `new` preserves its alignment.
    let pointer = armed(|| unsafe { std::alloc::realloc(pointer, old, new.size()) });
    assert!(!pointer.is_null(), "controlled realloc");
    assert_eq!(LARGEST_REALLOC_BYTES.get(), new.size() as u64);
    // SAFETY: the successful realloc returned a live allocation with layout `new`.
    armed(|| unsafe { std::alloc::dealloc(pointer, new) });
    assert!(!BYTE_COUNTER_FAILED.get());
    assert_eq!(
        LIVE_BYTES.get(),
        0,
        "controlled realloc releases its new request"
    );

    let count = |layouts: &[builtins_compiler::BuiltinRetainedLayout], expected: Layout| {
        layouts
            .iter()
            .find(|layout| {
                layout.size_bytes == expected.size() as u64
                    && layout.align_bytes == expected.align() as u64
            })
            .map_or(0, |layout| layout.allocation_count)
    };
    let mut prepared_unit_layout = None;
    for &width in effect_contract::BankWidth::ALL {
        let lanes = width.lanes() as usize;
        for slot_count in [0_usize, 1, 3, 9] {
            // Stage storage, chain mask and scratch are created before the measured conversion.
            // The graph test seam keeps this chain ragged (padded), so construction also makes
            // the chain's own planes (issue #887): two staging planes, which every chain owns for
            // its tiled scatter and which at quantum 1 have the scratch plane's layout, and the
            // one-word `+0.0` plane only a padded chain owns, which its tiled gather reads for the
            // inactive lane. They are the three allocations, and the bytes, this test adds below.
            let stage_layout =
                Layout::array::<Box<dyn rack::BankStage>>(slot_count).expect("stage layout");
            let slot_layout =
                Layout::array::<rack::BankSlot>(slot_count).expect("public slot layout");
            let mask_layout = Layout::array::<bool>(lanes).expect("one lane mask layout");
            let scratch_layout = Layout::array::<f32>(lanes).expect("one scratch plane");
            let zero_layout = Layout::array::<f32>(1).expect("one quantum-1 zero plane");
            let chain_plane_bytes = (2 * scratch_layout.size() + zero_layout.size()) as u64;
            let inputs = graph::test_only_prepare_bank_chain_inputs(width, slot_count);
            test_only_reset_phase_two_allocation_tracker();
            reset_byte_counters();
            LIVE_ALLOCS.set(0);
            LIVE_FREES.set(0);
            LIVE_BYTES.set((stage_layout.size() + mask_layout.size()) as u64);
            let observation = test_only_begin_phase_two_allocation_observation();
            let mut ownership = armed(|| graph::test_only_bank_chain_ownership(inputs));
            drop(observation);
            let snapshot = test_only_phase_two_allocation_snapshot();
            assert!(!snapshot.overflowed);
            assert!(!BYTE_COUNTER_FAILED.get());
            assert_eq!(ownership.slot_count, slot_count);
            assert_eq!(ownership.requested_stage_capacity, slot_count);
            assert_eq!(
                ownership.runtime_slot_capacity, slot_count,
                "incoming public BankSlot Vec capacity"
            );
            assert_eq!(ownership.inferred_retained_slot_count, slot_count);
            assert_eq!(ownership.stage_pointer_bytes, stage_layout.size());
            assert_eq!(ownership.slot_bytes, slot_layout.size());
            assert_eq!(ownership.mask_bytes, mask_layout.size());

            // The chain's own planes, on every slot count: two staging planes and the zero plane.
            assert_eq!(count(&snapshot.layouts, scratch_layout), 2);
            assert_eq!(count(&snapshot.layouts, zero_layout), 1);
            let prepared_layout = if slot_count == 0 {
                assert_eq!(snapshot.allocation_count, 3);
                assert_eq!(snapshot.deallocation_count, 0);
                assert_eq!(snapshot.layouts.len(), 2);
                assert!(snapshot.deallocation_layouts.is_empty());
                None
            } else {
                let observed = snapshot
                    .layouts
                    .iter()
                    .find(|layout| {
                        layout.allocation_count == 1
                            && !(layout.size_bytes == zero_layout.size() as u64
                                && layout.align_bytes == zero_layout.align() as u64)
                            && !(layout.size_bytes == slot_layout.size() as u64
                                && layout.align_bytes == slot_layout.align() as u64)
                            && !(layout.size_bytes == mask_layout.size() as u64
                                && layout.align_bytes == mask_layout.align() as u64)
                    })
                    .expect("actual private prepared-slot destination request");
                let layout = Layout::from_size_align(
                    usize::try_from(observed.size_bytes).expect("native layout size"),
                    usize::try_from(observed.align_bytes).expect("native layout alignment"),
                )
                .expect("observed private destination layout");
                let unit = (
                    layout
                        .size()
                        .checked_div(slot_count)
                        .expect("nonzero private destination slot count"),
                    layout.align(),
                );
                if let Some(expected) = prepared_unit_layout {
                    assert_eq!(unit, expected, "one private target layout across S/W cases");
                } else {
                    prepared_unit_layout = Some(unit);
                    eprintln!(
                        "actual-private-prepared-slot-layout size={} align={}",
                        unit.0, unit.1
                    );
                }
                // `+ 3`: the two staging planes and the zero plane are never freed here.
                assert_eq!(snapshot.allocation_count, slot_count as u64 + 2 + 3);
                assert_eq!(snapshot.deallocation_count, slot_count as u64 + 2);
                assert_eq!(count(&snapshot.layouts, slot_layout), 1);
                assert_eq!(count(&snapshot.layouts, layout), 1);
                assert_eq!(count(&snapshot.layouts, mask_layout), slot_count as u64);
                assert_eq!(count(&snapshot.deallocation_layouts, stage_layout), 1);
                assert_eq!(count(&snapshot.deallocation_layouts, slot_layout), 1);
                assert_eq!(
                    count(&snapshot.deallocation_layouts, mask_layout),
                    slot_count as u64
                );
                Some(layout)
            };

            let n = slot_count as u64;
            let f = core::mem::size_of::<Box<dyn rack::BankStage>>() as u64;
            let b = core::mem::size_of::<rack::BankSlot>() as u64;
            let w = mask_layout.size() as u64;
            let p = prepared_layout.map_or(0, |layout| {
                (layout.size() as u64)
                    .checked_div(n)
                    .expect("private layout exists only for a nonzero slot count")
            });
            assert_eq!(core::mem::size_of::<bool>(), 1);
            assert!(p <= b, "actual private slot layout fits public slot layout");
            let retained = n * p + w + chain_plane_bytes;
            let staging_plane = scratch_layout.size() as u64;
            if n == 0 {
                assert_eq!(LARGEST_REQUESTED_BYTES.get(), staging_plane);
                assert_eq!(
                    LIVE_BYTES.get(),
                    w + chain_plane_bytes,
                    "direct caller retains its chain mask and the chain its own planes"
                );
            } else {
                let c = n * (f + 3 * b + 3 * w);
                let l = (n * f).max(n * b).max(w).max(staging_plane);
                let conversion_coexistence = n * f + n * b + 2 * n * p + 2 * n * w;
                assert!(n * p + n * w <= n * (b + 2 * w));
                assert!(conversion_coexistence <= c);
                for request in [n * f, n * b, n * p, w, staging_plane] {
                    assert!(request <= l);
                }
                assert_eq!(LARGEST_REQUESTED_BYTES.get(), l);
                assert_eq!(LIVE_BYTES.get(), retained);

                let mut planes = RuntimeBankPlanes::new(lanes);
                let chain = ownership.test_only_chain_mut();
                chain.arm_mono_collapse(true);
                for sample in 0..4 {
                    audit_runtime_bank_render(chain, &mut planes, sample);
                }
                assert_eq!(chain.collapses(), 4, "actual mono-collapsed path is live");
                chain.force_mono_collapse_off(true);
                for sample in 4..8 {
                    audit_runtime_bank_render(chain, &mut planes, sample);
                }
                assert_eq!(chain.collapses(), 4, "forced fallback remains dual");
                assert!(!chain.is_collapsed());
            }

            let release_bytes = retained + 2 * scratch_layout.size() as u64;
            test_only_reset_phase_two_allocation_tracker();
            reset_byte_counters();
            LIVE_ALLOCS.set(0);
            LIVE_FREES.set(0);
            LIVE_BYTES.set(release_bytes);
            let observation = test_only_begin_phase_two_allocation_observation();
            armed(|| drop(ownership));
            drop(observation);
            let release = test_only_phase_two_allocation_snapshot();
            assert!(!release.overflowed);
            assert!(!BYTE_COUNTER_FAILED.get());
            assert_eq!(release.allocation_count, 0);
            // `+ 3` more than before #887: the two staging planes and the zero plane.
            assert_eq!(
                release.deallocation_count,
                u64::from(slot_count != 0) + 3 + 3
            );
            if let Some(prepared_layout) = prepared_layout {
                assert_eq!(count(&release.deallocation_layouts, prepared_layout), 1);
            }
            assert_eq!(count(&release.deallocation_layouts, mask_layout), 1);
            // Two scratch planes and two staging planes share the layout at quantum 1.
            assert_eq!(count(&release.deallocation_layouts, scratch_layout), 4);
            assert_eq!(count(&release.deallocation_layouts, zero_layout), 1);
            assert_eq!(LIVE_BYTES.get(), 0, "owned slot chain releases off render");
            assert_eq!(LIVE_ALLOCS.get(), 0);
            assert_eq!(LIVE_FREES.get(), u64::from(slot_count != 0) + 3 + 3);
        }
    }
    // The full-mask row (issue #887): a full chain owns its two staging planes and no `+0.0`
    // plane, which only a padded chain's gather reads. Red if a full chain allocates one.
    for &width in effect_contract::BankWidth::ALL {
        let lanes = width.lanes() as usize;
        let staging_layout = Layout::array::<f32>(lanes).expect("one quantum-1 staging plane");
        let scratch = rack::AoSoaScratch::new(width, 1).expect("test width and quantum");
        let active = vec![true; lanes].into_boxed_slice();
        test_only_reset_phase_two_allocation_tracker();
        reset_byte_counters();
        let observation = test_only_begin_phase_two_allocation_observation();
        let chain =
            armed(|| rack::BankChain::new(scratch, active, Vec::new())).expect("a full chain");
        drop(observation);
        let snapshot = test_only_phase_two_allocation_snapshot();
        assert!(!snapshot.overflowed);
        assert_eq!(snapshot.allocation_count, 2, "{width:?}: staging only");
        assert_eq!(count(&snapshot.layouts, staging_layout), 2, "{width:?}");
        drop(chain);
    }
    assert!(
        prepared_unit_layout.is_some(),
        "nonzero cases expose the actual private target layout"
    );

    // This direct-attachment fixture is separate from compiler admission. It proves the actual
    // runtime R/S bounds against the calculated allowance.
    let mut output = [0.0_f32; 128];
    graph::test_only_reset_bank_chain_construction_facts();
    let mut unpaired = builtins_compiler::test_only_prepared_unpaired_graph();
    let unpaired_facts = graph::test_only_bank_chain_construction_facts();
    let unpaired_allowance = graph::GraphBankSlotResourceEstimate::checked_for(
        unpaired_facts.prepared_memberships as u64,
        effect_contract::BankWidth::for_backend(lane::Backend::current()),
    )
    .expect("unpaired allowance");
    assert!(
        unpaired_facts.maximum_runtime_slots > 1,
        "actual unpaired S > 1"
    );
    assert!(unpaired_facts.runtime_slots <= unpaired_facts.run_memberships);
    assert!(unpaired_facts.run_memberships <= unpaired_facts.prepared_memberships);
    assert_eq!(
        unpaired_allowance.bank_count,
        unpaired_facts.prepared_memberships as u64
    );
    for sample in [0, 64] {
        let _ = audit_graph_render(&mut unpaired, &mut output, sample);
    }
    drop(unpaired);
}

fn session(track_count: u32) -> session::CompiledSession {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("fixture parse");
    let mut template = model.tracks[0].clone();
    template.console.clear();
    template.inserts.effects.clear();
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
        tap: SendTap::PostPan,
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

fn effective_of(session: &session::CompiledSession) -> controls::Effective {
    controls::effective(
        session.normalized_model(),
        &controls::MonitorOverlay::default(),
    )
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

/// Layout-class equality does not depend on the track count, and one and four tracks reach every
/// linear container (issue #1045), so the 65,537-track row runs nightly, in
/// [`phase_two_allocator_layouts_match_the_checked_resource_report_at_65_537_tracks`].
#[test]
fn phase_two_allocator_layouts_match_the_checked_resource_report() {
    assert_phase_two_allocator_layouts_match_the_checked_resource_report(&[1, 4]);
}

/// Issue #1045: nightly, in release with overflow checks on (`nightly.yml`, `release-budgets`).
#[test]
#[ignore = "65,537-track scale gate; runs nightly"]
fn phase_two_allocator_layouts_match_the_checked_resource_report_at_65_537_tracks() {
    assert_phase_two_allocator_layouts_match_the_checked_resource_report(&[65_537]);
}

fn assert_phase_two_allocator_layouts_match_the_checked_resource_report(track_counts: &[u32]) {
    // Excludes every other test in this binary from the process-global tracker for the whole
    // measurement session (reset through snapshot); see `SESSION`.
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    // One discarded prepare, ahead of every measured combination below, settles first-touch
    // process/thread state once so no measured window ever observes it. Its snapshot is not
    // thrown away: the very first measured combination below repeats the same
    // session/requests/caps, and the two are compared by `assert_first_touch_delta_is_bounded`.
    let warmup_snapshot =
        settle_phase_two_first_touch(&session(track_counts[0]), &requests(0), caps());
    let mut first_touch_delta_checked = false;
    for &track_count in track_counts {
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

/// Issue #1254 gate 3: a fader-and-matrix request (`input_lane: false`) on every track charges
/// exactly what preparation allocates -- two cell sets per track, not three -- and an all-lanes
/// request still does. A cell set that is allocated but not charged, or charged but not allocated,
/// turns the layout comparison red.
#[test]
fn live_control_cells_are_charged_exactly_as_allocated_with_and_without_the_input_lane() {
    let _session_guard = SESSION
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let session = session(4);
    let requests = requests(0);
    let controls = |input_lane: bool| -> Vec<TrackControlRequest> {
        (0..4)
            .map(|index| TrackControlRequest {
                track_id: format!("track-{index}"),
                superseded: LiveValuesSuperseded::default(),
                input_lane,
            })
            .collect()
    };
    let prepare = |controls: &[TrackControlRequest]| {
        armed(|| {
            prepare_session_builtins_with_live_controls(
                &session,
                &effective_of(&session),
                &requests,
                controls,
                caps(),
            )
        })
        .expect("prepare")
    };
    // Settles first-touch state for this measurement, as `settle_phase_two_first_touch` does.
    test_only_reset_phase_two_allocation_tracker();
    let _ = prepare(&controls(true));
    let mut retained = Vec::new();
    for input_lane in [false, true] {
        let controls = controls(input_lane);
        test_only_reset_phase_two_allocation_tracker();
        let prepared = prepare(&controls);
        let snapshot = test_only_phase_two_allocation_snapshot();
        let report = prepared.resource_report();
        assert!(!snapshot.overflowed);
        assert_eq!(prepared.track_control_count(), 4);
        assert!(
            prepared.validate_for_session(&session).0.is_empty(),
            "input_lane={input_lane}: the prepared artifact passes its own seal"
        );
        assert_eq!(
            snapshot.layouts,
            report.retained_layouts(),
            "input_lane={input_lane}"
        );
        assert_eq!(
            snapshot.total_bytes, report.engine_owned_retained_payload_bytes,
            "input_lane={input_lane}"
        );
        assert_eq!(
            snapshot.allocation_count, report.retained_allocation_count,
            "input_lane={input_lane}"
        );
        assert_eq!(
            snapshot.largest_allocation_bytes, report.maximum_single_allocation_bytes,
            "input_lane={input_lane}"
        );
        retained.push(report.engine_owned_retained_payload_bytes);
    }
    assert!(
        retained[0] < retained[1],
        "dropping the input lane must drop its cells: {retained:?}"
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
