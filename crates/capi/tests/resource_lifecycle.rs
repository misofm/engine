//! Exported-C retained-allocation and disposal ownership evidence.

#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
    mem::{MaybeUninit, size_of},
    ptr,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize},
};
use std::alloc::{GlobalAlloc, System};
use std::{sync::Mutex, thread::JoinHandle};

use capi::*;
use engine::realtime::{PlanEpoch, PreparedRenderPlan, Producer, QueueGeneration};
use lane::Backend;
use protocol::{
    AUTOMATION_BATCH_RECORDS, AutomationBatchSlot, AutomationRecord, CommandPayload,
    ControlCommandSlot, CounterId, CounterTelemetryRecord, CounterValue, ExpectedRevision,
    ProtocolCodec, ReliableSlot, RequestId, SessionEdit, SessionRevision, StatusCode,
    TelemetryRecord, TypedCommandFrame,
};
use session::StableId;
use source::HostChunkProvider;

struct LifecycleAllocator;

#[global_allocator]
static LIFECYCLE_ALLOCATOR: LifecycleAllocator = LifecycleAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static ALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
}

fn record_allocation(bytes: usize) {
    ACTIVE.with(|active| {
        if active.get() {
            ALLOCATIONS.set(ALLOCATIONS.get() + 1);
            ALLOCATED_BYTES.set(ALLOCATED_BYTES.get() + bytes as u64);
        }
    });
}

fn record_deallocation(bytes: usize) {
    ACTIVE.with(|active| {
        if active.get() {
            DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
            DEALLOCATED_BYTES.set(DEALLOCATED_BYTES.get() + bytes as u64);
        }
    });
}

// SAFETY: Every operation delegates the original pointer/layout unchanged to `System`; the
// thread-local counters are observational and enabled only around this isolated test thread.
unsafe impl GlobalAlloc for LifecycleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The allocator-provided layout is forwarded unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The allocator-provided layout is forwarded unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            record_allocation(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        record_deallocation(layout.size());
        // SAFETY: The original pointer and layout are forwarded unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: The original allocation arguments and requested size are forwarded unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            record_deallocation(layout.size());
            record_allocation(new_size);
        }
        replacement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Snapshot {
    allocations: u64,
    deallocations: u64,
    allocated_bytes: u64,
    deallocated_bytes: u64,
}

impl Snapshot {
    fn delta(self, earlier: Self) -> Self {
        Self {
            allocations: self.allocations - earlier.allocations,
            deallocations: self.deallocations - earlier.deallocations,
            allocated_bytes: self.allocated_bytes - earlier.allocated_bytes,
            deallocated_bytes: self.deallocated_bytes - earlier.deallocated_bytes,
        }
    }

    fn assert_balanced(self, label: &str) {
        assert_eq!(self.allocations, self.deallocations, "{label} owners");
        assert_eq!(
            self.allocated_bytes, self.deallocated_bytes,
            "{label} bytes"
        );
    }
}

fn snapshot() -> Snapshot {
    Snapshot {
        allocations: ALLOCATIONS.get(),
        deallocations: DEALLOCATIONS.get(),
        allocated_bytes: ALLOCATED_BYTES.get(),
        deallocated_bytes: DEALLOCATED_BYTES.get(),
    }
}

/// Initialize the process-lifetime statics that the JSON frontend's dependencies create lazily,
/// so the first observed window is not charged for them.
///
/// `json-syntax` 0.12.5 indexes every object through `hashbrown` 0.12's `DefaultHashBuilder`,
/// which is `ahash` 0.7's `RandomState`. Its first construction in a process boxes three
/// `once_cell::race::OnceBox` statics (`RAND_SOURCE`, its inner `Box<dyn RandomSource>`, and the
/// `SEEDS` array: 8 + 16 + 64 bytes) that live until process exit and belong to no capi owner.
/// ahash's `build.rs` forces `runtime-rng` on every hosted target, so no Cargo feature removes
/// them. They land on whichever thread parses the first object, which under the parallel
/// harness is a race between this file's tests; parsing a trivial object here makes every
/// window start after that initialization on this thread, independent of sibling scheduling.
fn warm_process_lifetime_statics() {
    let _ = session::parse_session_json(r#"{"warm":0}"#);
}

fn begin() {
    warm_process_lifetime_statics();
    // Initialize the thread-local keys before observation is armed.
    ACTIVE.set(false);
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ALLOCATED_BYTES.set(0);
    DEALLOCATED_BYTES.set(0);
    ACTIVE.set(true);
}

fn finish() -> Snapshot {
    ACTIVE.set(false);
    snapshot()
}

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.json");

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_document_bytes: 1_000_000,
        maximum_diagnostic_bytes: 4_096,
        maximum_tracks: 100,
        maximum_sources: 100,
        maximum_routes: 100,
        maximum_effects: 100,
        maximum_graph_session_plus_plan_bytes: 100_000_000,
        maximum_source_total_bytes: 10_000_000,
        maximum_source_overhead_bytes: 10_000_000,
        maximum_effect_state_bytes: 100_000_000,
        maximum_effect_scratch_bytes: 100_000_000,
        maximum_builtin_retained_bytes: 100_000_000,
        maximum_capi_retained_bytes: 10_000_000,
        maximum_named_allocation_bytes: 100_000_000,
        maximum_meter_streams: 1,
        maximum_meter_items: 1,
        maximum_meter_bytes: 1,
        maximum_control_frame_bytes: 4_096,
        maximum_replay_bytes: 8_192,
        maximum_replay_entries: 16,
        reserved: [0; 4],
    }
}

fn command(request_id: u64, revision: u64, session_id: &'static str) -> Vec<u8> {
    let edit = SessionEdit::SetSessionId {
        session_id: StableId::parse(session_id).expect("static session ID"),
    };
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(request_id).expect("nonzero request"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(revision)),
                payload: CommandPayload::SessionTransactionApply(core::slice::from_ref(&edit)),
            },
            &mut bytes,
        )
        .expect("structural command");
    bytes.truncate(len);
    bytes
}

fn capability_command() -> Vec<u8> {
    let mut bytes = vec![0_u8; 4_096];
    let len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("request"),
                expected_revision: ExpectedRevision::Any,
                payload: CommandPayload::CapabilitiesGet,
            },
            &mut bytes,
        )
        .expect("capability command");
    bytes.truncate(len);
    bytes
}

unsafe fn submit(session: *mut Session, request: &[u8], response: &mut [u8; 4_096]) -> u32 {
    let mut output = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: response.as_mut_ptr(),
        capacity_bytes: response.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: All pointers identify live handles or complete caller-owned buffers for this call.
    unsafe {
        miso_engine_v1_submit_command(session, request.as_ptr(), request.len() as u64, &mut output)
    }
}

fn lifecycle(plan_first: bool) -> (Snapshot, Snapshot, Snapshot, Snapshot) {
    let capability = capability_command();
    let first = command(2, 42, "lifecycle-first");
    let second = command(3, 43, "lifecycle-second");
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut diagnostics_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostics_storage.as_mut_ptr(),
        capacity_bytes: diagnostics_storage.len() as u64,
        required_bytes: 0,
    };
    let mut response = [0_u8; 4_096];
    let mut pcm = [f32::NAN; 256];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();

    begin();
    // SAFETY: All ABI arguments remain live and uniquely owned for the complete lifecycle.
    let codes = unsafe {
        let create = miso_engine_v1_engine_create(&config, &mut engine);
        let compile = miso_engine_v1_compile_session(
            engine,
            SESSION.as_ptr(),
            SESSION.len() as u64,
            &limits(),
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        let immediate = submit(session, &capability, &mut response);
        let before_cached = snapshot();
        let cached = submit(session, &capability, &mut response);
        let cached_delta = snapshot().delta(before_cached);
        let first_structural = submit(session, &first, &mut response);
        let before_full = snapshot();
        let full = submit(session, &second, &mut response);
        let full_delta = snapshot().delta(before_full);
        let before_render = snapshot();
        let render = miso_engine_v1_render_f32_planar(plan, 0, &output);
        let render_delta = snapshot().delta(before_render);
        let retry = submit(session, &second, &mut response);
        let second_render = miso_engine_v1_render_f32_planar(plan, 128, &output);
        if plan_first {
            miso_engine_v1_plan_destroy(plan);
            miso_engine_v1_session_destroy(session);
        } else {
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }
        miso_engine_v1_engine_destroy(engine);
        (
            [
                create,
                compile,
                immediate,
                cached,
                first_structural,
                full,
                render,
                retry,
                second_render,
            ],
            cached_delta,
            full_delta,
            render_delta,
        )
    };
    let total = finish();
    assert_eq!(
        codes.0,
        [
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
            RESULT_BACKPRESSURE,
            RESULT_OK,
            RESULT_OK,
            RESULT_OK,
        ]
    );
    (total, codes.1, codes.2, codes.3)
}

fn rejected_compile_lifecycle() -> Snapshot {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut constrained = limits();
    constrained.maximum_capi_retained_bytes = 1;
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::dangling_mut();
    let mut plan = ptr::dangling_mut();

    begin();
    // SAFETY: All ABI arguments remain live for each call and no rejected child is published.
    let codes = unsafe {
        let create = miso_engine_v1_engine_create(&config, &mut engine);
        let compile = miso_engine_v1_compile_session(
            engine,
            SESSION.as_ptr(),
            SESSION.len() as u64,
            &constrained,
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        miso_engine_v1_engine_destroy(engine);
        (create, compile)
    };
    let total = finish();
    assert_eq!(codes, (RESULT_OK, RESULT_COMPILE_REJECTED));
    assert!(session.is_null());
    assert!(plan.is_null());
    assert!(diagnostics.required_bytes > 0);
    total
}

#[test]
fn exported_c_candidates_replay_render_and_both_destroy_orders_balance_exactly() {
    rejected_compile_lifecycle().assert_balanced("rejected compile provisional owners");

    let plan_first = lifecycle(true);
    plan_first
        .0
        .assert_balanced("plan-first complete lifecycle");
    plan_first.1.assert_balanced("cached replay");
    plan_first
        .2
        .assert_balanced("publication-full canceled candidate");
    assert_eq!(
        plan_first.3,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
        }
    );

    let session_first = lifecycle(false);
    session_first
        .0
        .assert_balanced("session-first complete lifecycle");
    session_first.1.assert_balanced("cached replay");
    session_first
        .2
        .assert_balanced("publication-full canceled candidate");
    assert_eq!(session_first.3, plan_first.3);
    assert_eq!(session_first.0, plan_first.0, "destroy-order ownership");
}

fn scratch_session() -> String {
    let mut model = session::parse_session_json(SESSION).expect("oracle fixture");
    for track in &mut model.tracks {
        let effect = &mut track.simd1.effects[0];
        effect.id = StableId::parse("soft-clip").expect("effect slot");
        effect.identity = session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.soft-clip").expect("effect ID"),
        };
        effect.params = vec![
            session::EffectParam {
                parameter_id: 1,
                channel: session::ParameterChannel::Left,
                unit: session::ParameterUnit::Db,
                value: -6.0,
            },
            session::EffectParam {
                parameter_id: 1,
                channel: session::ParameterChannel::Right,
                unit: session::ParameterUnit::Db,
                value: -6.0,
            },
        ];
    }
    session::canonical_session_json(&model).expect("oracle canonical fixture")
}

/// The single-plan resource report of the scratch fixture.
///
/// Issue #181 moved four of these fields by eight bytes: `size_of::<GraphPreparedEffectBank>()`
/// went 88 -> 96 when a bound bank started carrying the cohort chain it is a slot of, and the
/// fixture binds one bank. `effect_bank_metadata_bytes` is where it lands; the three graph totals
/// carry it upward.
fn frozen_scratch_report(capi_retained_bytes: u64) -> PlanResourceReport {
    PlanResourceReport {
        struct_size: PLAN_RESOURCE_REPORT_SIZE,
        abi_version: ABI_VERSION,
        sample_rate_hz: 48_000,
        quantum_frames: 128,
        source_count: 1,
        track_count: 9,
        latency_samples: 31,
        tail_kind: TAIL_INFINITE,
        tail_samples: 0,
        // Strip round job 1: `InputStage` gained the elision plan, and the field changed the
        // struct's layout such that `size_of::<BuiltinBankProcessor>()` went 1248 -> 1216. This
        // fixture binds two banks, so every absolute figure below that carries the processor
        // payload moves by -64. The double-live oracle above does not move: it is a difference
        // between two live compiles, and a uniform shift cancels in it. That asymmetry is the
        // point of holding both -- the oracle proves the model tracks the tree, and these
        // literals prove the absolute report is still the one that was reviewed.
        //
        // Strip round job 2 banked the strip's own fader and matrix, so this nine-track fixture
        // now binds **six** builtin banks where it bound two: `9.div_ceil(8) == 2` per bankable
        // stage, times three stages. Each figure below moves for a stated reason.
        //
        // * `builtin_bank_scratch_bytes` 16_384 -> 49_152 is exactly three times the old value.
        //   One `AoSoaScratch` is charged per bound *slot* -- 128 frames x 8 lanes x 2 planes x 4
        //   bytes = 8_192 each -- and there are three times as many slots. The estimate now
        //   over-states what the plan retains by more than it did, because a merged chain keeps
        //   its first slot's scratch and drops the rest at bind (`runtime::chain_for`). Over-
        //   stating is the safe direction for a memory ceiling and is deliberate: whether a merge
        //   is admissible is not knowable before the lowered program exists.
        // * `builtin_bank_bytes` 2_963 -> 7_865 adds the four new banks' member arrays, member
        //   strings and processors -- the fader and matrix processors each carrying an eight-entry
        //   array of optional console consumers, charged whether or not a console is attached.
        // * `builtin_processor_payload_bytes` 7_974 -> 8_406 is +432, exactly 9 tracks x 48: two
        //   `GraphNodeBinding`s (2 x 72) and the boxed `FaderProcessor` (16) and `MatrixProcessor`
        //   (136) left preparation, and the 344-byte `StripPreparation` vector entry replaced them.
        // * The three `graph_*` figures carry the four extra banks' plan-side metadata.
        //
        // Issue #210 phase 3 (live `trim_db` / `polarity_invert`) moves three of these figures and
        // no others. The session model is untouched -- this phase adds no schema key -- so
        // `graph_metadata_bytes`, every source row, every effect row and `capi_retained_bytes`
        // stand.
        //
        // * `builtin_bank_bytes` 7_865 -> 9_209 is +1_344 = 2 input banks x 672. Each input bank
        //   grew by 352 in `BuiltinBankProcessor` itself -- `InputStage<Simd8>`'s trim ramp (256),
        //   its `[[u32; 8]; 2]` countdown (64), the `ramping` flag, the third drain's
        //   `Box<[Option<Consumer<_>>]>` (16) and the eight-byte live-witness array -- plus 320
        //   for that consumer array's eight-entry heap, charged whether or not a console is
        //   attached, exactly as the fader and matrix banks' arrays already were.
        // * `builtin_processor_payload_bytes` 8_406 -> 9_963 is +1_557 = 9 tracks x 173. See
        //   `builtin_owners` for the five-term restatement that sums to 173.
        // * The two `graph_*` figures carry the same +1_344 the banks moved by, which is what it
        //   means for the growth to be entirely plan-side.
        // #241 deletes 64 x 64 = 4_096 control-queue bytes and 1_024 x 2 x 4 = 8_192
        // declarative source-ring bytes from the session compiler's runtime projection. The host
        // still reports the chosen ring exactly in the source rows below.
        graph_session_plus_plan_bytes: 226_164,
        graph_incremental_plan_bytes: 226_164,
        graph_metadata_bytes: 50_295,
        graph_delay_bytes: 0,
        effect_bank_scratch_bytes: 8_192,
        effect_bank_runtime_buffer_bytes: 8_192,
        effect_bank_metadata_bytes: 648,
        builtin_bank_bytes: 9_337,
        builtin_bank_scratch_bytes: 49_152,
        source_pcm_payload_bytes: 8_192,
        source_overhead_bytes: 2_862,
        source_total_bytes: 11_054,
        effect_scalar_state_bytes: 7_560,
        effect_scalar_scratch_bytes: 216,
        builtin_processor_payload_bytes: 9_963,
        builtin_meter_payload_bytes: 0,
        builtin_retained_payload_bytes: 9_963,
        capi_retained_bytes,
        largest_named_allocation_bytes: 49_167,
        reserved: [0; 4],
    }
}

#[derive(Clone, Copy)]
struct PrimitiveReplacementOracle {
    graph: u64,
    source_total: u64,
    source_overhead: u64,
    effect_state: u64,
    effect_scratch: u64,
    builtin: u64,
    capi: u64,
    largest: u64,
}

/// #84 phase B: each ring cursor sits alone on a 64-byte line, mirroring core's `CachePadded`.
#[repr(C, align(64))]
struct RingCursorMirror(AtomicUsize);

#[repr(C)]
struct RingMirror<T> {
    slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
    slots_len: usize,
    logical_capacity: usize,
    generation: QueueGeneration,
    producer: RingCursorMirror,
    consumer: RingCursorMirror,
}

#[repr(C)]
struct SharedRingMirror<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    ring: RingMirror<T>,
}

#[repr(C)]
struct SharedCounterMirror {
    strong: AtomicUsize,
    weak: AtomicUsize,
    value: AtomicUsize,
}

#[repr(C)]
struct SharedArcMirror<T> {
    strong: AtomicUsize,
    weak: AtomicUsize,
    value: T,
}

#[allow(dead_code)]
enum RetainedDiagnosticSlotMirror {
    Empty,
    Owned(protocol::Diagnostic),
}

#[allow(dead_code)]
struct RenderDiagnosticSlotMirror {
    diagnostic: protocol::Diagnostic,
    reservation: Option<protocol::ReliableEventReservation>,
    protocol_events_before: u64,
    revision: SessionRevision,
    occupied: bool,
}

#[allow(dead_code)]
struct CompiledIndexNodeMirror {
    entries: [(StableId, u64); 4],
}

#[allow(dead_code)]
#[repr(C)]
struct CanonicalEscapedByteMirror {
    escape: u8,
    unicode_tag: u8,
    opening_brace: u8,
    digits: [u8; 6],
    closing_brace: u8,
}

#[allow(dead_code)]
#[repr(C)]
struct CanonicalFieldScratchMirror {
    key: [u8; 32],
    separator: [u8; 8],
    value: [u8; 72],
    terminator: [u8; 16],
}

#[allow(dead_code)]
struct CanonicalStructuralItemMirror {
    fields: [CanonicalFieldScratchMirror; 8],
}

#[allow(dead_code)]
struct CanonicalDocumentPreludeMirror {
    frames: [CanonicalStructuralItemMirror; 4],
}

#[allow(dead_code)]
struct SharedPlanStateMirror {
    plan_alive: AtomicBool,
    active_epoch: AtomicU64,
    reports: Mutex<Vec<(u64, PlanResourceReport)>>,
    render_sequence: AtomicU64,
    render_sample: AtomicU64,
    render_peak_bits: AtomicU32,
}

#[repr(C)]
struct DensityMirror {
    block: u64,
    starts: usize,
    occupied: bool,
}

#[repr(C)]
struct IntervalMirror {
    record: AutomationRecord,
    occupied: bool,
}

#[allow(dead_code)]
struct ReplayEntryMirror {
    request_id: RequestId,
    request_offset: usize,
    request_bytes: usize,
    response_offset: usize,
    response_bytes: usize,
    response_status: StatusCode,
    response_revision: SessionRevision,
    framed: bool,
}

#[allow(dead_code)]
struct PublishedPlanMirror {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
    retirement_reserved: bool,
}

#[allow(dead_code)]
struct RetiredPlanMirror {
    epoch: PlanEpoch,
    plan: PreparedRenderPlan,
}

/// Mirror of `host_core`'s private control-source endpoint (audit #103 W4-2 moved it
/// out of capi). The oracle restates the layout independently and never reads a runtime figure:
/// the facade's own `control_table_bytes` is what capi's pre-flight uses, and this mirror is the
/// second, independent witness that the pre-flight charges the right number of bytes.
#[allow(dead_code)]
struct ControlSourceMirror {
    id_offset: usize,
    id_bytes: usize,
    sample_rate_hz: u32,
    channel_count: u32,
    region_start: u64,
    region_end: u64,
    provider: HostChunkProvider,
}

/// Mirror of the facade's `SourceControlSet`: the ID arena and the endpoint table.
#[allow(dead_code)]
struct SourceControlSetMirror {
    ids: Box<[u8]>,
    sources: Box<[ControlSourceMirror]>,
}

/// Mirror of capi's `ProviderEpoch`: the epoch tag plus the facade's set.
#[allow(dead_code)]
struct ProviderEpochMirror {
    epoch: u64,
    sources: SourceControlSetMirror,
}

#[allow(dead_code)]
struct TransferBlockMirror {
    generation: source::SourceGeneration,
    start_frame: source::SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
    samples: Box<[f32]>,
}

#[allow(dead_code)]
struct NativeSourceWorkerMirror {
    join: Option<JoinHandle<source::NativeSourceWorkerExit>>,
    stopped: bool,
    stop: Producer<()>,
    not_sync: PhantomData<Cell<()>>,
}

/// #124: the entry is consumer-only — planes were deleted (graph fan-out copies the retained
/// block directly) and retirement ownership moved onto the driver so workers stop before
/// consumers drop.
#[allow(dead_code)]
struct GraphSourceEntryMirror {
    consumer: source::PcmSourceConsumer,
}

/// #124: `_retirement_workers` is declared first for drop order; `copied_claims` is the per-block
/// copied-claim count the fan-out report exposes.
#[allow(dead_code)]
struct SourceGraphSourceSetDriverMirror {
    retirement_workers: Box<[NativeSourceWorkerMirror]>,
    sources: Box<[GraphSourceEntryMirror]>,
    mappings: Box<[source::SourceGraphTrackMapping]>,
    quantum_frames: u32,
    copied_claims: usize,
}

/// `<f32 as lane::Lane>::Mask`: an all-zero or all-one word.
#[allow(dead_code)]
struct ScalarLaneMaskMirror(u32);

#[allow(dead_code)]
struct SvfCoefMirror {
    c1: f32,
    a2: f32,
    a3: f32,
    m0: f32,
    m1: f32,
    m2: f32,
}

#[allow(dead_code)]
struct SvfStateMirror {
    ic1: f32,
    ic2: f32,
}

/// `InputStage<f32>`: the whole of `InputBuiltins` since #85 -- the prepared record lives in the
/// coefficient words, not beside them.
#[allow(dead_code)]
struct InputBuiltinsMirror {
    members: usize,
    active: ScalarLaneMaskMirror,
    trim: [f32; 2],
    coef: [[SvfCoefMirror; 2]; 2],
    state: [[SvfStateMirror; 2]; 2],
    /// The strip round's prepared-identity elision plan, `[channel][section]`. Restated here
    /// since #210 phase 3: at the scalar width it used to fit entirely inside the struct's tail
    /// padding, so omitting it changed no byte; the trim ramp's `bool` now shares that tail and
    /// the two together no longer do.
    plan: [[bool; 2]; 2],
    /// #210 phase 3's live trim ramp: `InputTrimRamp<f32>`, four words per channel.
    ramp: [[f32; 2]; 4],
    /// The authoritative per-lane countdown, `MAX_BANK_LANES` wide at every width.
    ramp_remaining: [[u32; 8]; 2],
    /// The one-`bool` off gate.
    ramping: bool,
    lifetime_recovered: [u64; 2],
}

/// `FaderStage<f32>`.
#[allow(dead_code)]
struct FaderBuiltinsMirror {
    gain: [f32; 2],
    mute: [ScalarLaneMaskMirror; 2],
}

#[allow(dead_code)]
struct Matrix2x2Mirror {
    ll: f32,
    lr: f32,
    rl: f32,
    rr: f32,
}

#[allow(dead_code)]
struct Matrix2x2CoefMirror {
    ll: f32,
    lr: f32,
    rl: f32,
    rr: f32,
    identity: ScalarLaneMaskMirror,
}

#[allow(dead_code)]
struct Matrix2x2RampMirror {
    current: [f32; 4],
    target: [f32; 4],
    step: [f32; 4],
    remaining: f32,
}

/// `MatrixStage<f32>`: the per-lane bookkeeping is sized for the widest bank (#96 banks it).
#[allow(dead_code)]
struct MatrixBuiltinsMirror {
    coef: Matrix2x2CoefMirror,
    ramp: Matrix2x2RampMirror,
    smoothing_samples: [u32; 8],
    remaining: [u32; 8],
}

#[allow(dead_code)]
enum BuiltinTailMirror {
    FiniteZero,
    Infinite,
}

/// `SvfCoef<Simd8>` / `SvfState<Simd8>`: one `__m256` per word.
#[allow(dead_code)]
#[repr(align(32))]
struct SvfCoefEightMirror {
    words: [[f32; 8]; 6],
}

#[allow(dead_code)]
#[repr(align(32))]
struct SvfStateEightMirror {
    words: [[f32; 8]; 2],
}

/// `InputStage<Simd8>`, the eight-lane arm of `InputStageKernel`.
#[allow(dead_code)]
#[repr(align(32))]
struct InputStageEightMirror {
    members: usize,
    active: [u32; 8],
    trim: [[f32; 8]; 2],
    coef: [[SvfCoefEightMirror; 2]; 2],
    state: [[SvfStateEightMirror; 2]; 2],
    /// The strip round's prepared-identity elision plan, `[channel][section]`.
    plan: [[bool; 2]; 2],
    /// #210 phase 3's live trim ramp: `InputTrimRamp<Simd8>`, four words per channel.
    ramp: [[[f32; 8]; 2]; 4],
    /// The authoritative per-lane countdown, `[channel][lane]`.
    ramp_remaining: [[u32; 8]; 2],
    /// The one-`bool` off gate.
    ramping: bool,
    lifetime_recovered: [u64; 2],
}

/// `InputStage<Simd4>`, the four-lane arm.
#[allow(dead_code)]
#[repr(align(16))]
struct InputStageFourMirror {
    members: usize,
    active: [u32; 4],
    trim: [[f32; 4]; 2],
    coef: [[[f32; 4]; 6]; 4],
    state: [[[f32; 4]; 2]; 4],
    /// The strip round's prepared-identity elision plan, `[channel][section]`.
    plan: [[bool; 2]; 2],
    /// #210 phase 3's live trim ramp: `InputTrimRamp<Simd4>`, four words per channel.
    ramp: [[[f32; 4]; 2]; 4],
    /// The authoritative per-lane countdown, `[channel][lane]`; `MAX_BANK_LANES` wide at
    /// every width, exactly as `FaderRampStage`'s is.
    ramp_remaining: [[u32; 8]; 2],
    /// The one-`bool` off gate.
    ramping: bool,
    lifetime_recovered: [u64; 2],
}

/// `InputStageKernel`: an enum over the two widths, sized by the larger. Mirroring production
/// means mirroring its size decision too, `large_enum_variant` included.
#[allow(dead_code, clippy::large_enum_variant)]
enum InputStageKernelMirror {
    Simd4(InputStageFourMirror),
    Simd8(InputStageEightMirror),
}

#[allow(dead_code)]
struct BuiltinInputBankMirror {
    backend: Backend,
    width: effect_contract::BankWidth,
    members: usize,
    stage: InputStageKernelMirror,
}

#[allow(dead_code)]
struct BuiltinBankProcessorMirror {
    bank: BuiltinInputBankMirror,
    /// #210 phase 3: the third drain's per-lane console consumers, and the per-lane live
    /// channel-symmetry terms its `admit` fold maintains. The consumer array is a `Box<[..]>`
    /// here and its heap is charged separately, exactly as the fader and matrix banks' are.
    controls: (usize, usize),
    live: [u8; 8],
    process_calls: u64,
    tpt_kernel_calls: u64,
}

/// `GainMuteRamp<Simd8>`: four eight-lane words plus the mute mask.
#[allow(dead_code)]
#[repr(align(32))]
struct GainMuteRampEightMirror {
    current: [f32; 8],
    target: [f32; 8],
    step: [f32; 8],
    remaining: [f32; 8],
    mute: [u32; 8],
}

/// `GainMuteRamp<Simd4>`: the four-lane arm.
#[allow(dead_code)]
#[repr(align(16))]
struct GainMuteRampFourMirror {
    current: [f32; 4],
    target: [f32; 4],
    step: [f32; 4],
    remaining: [f32; 4],
    mute: [u32; 4],
}

/// `FaderRampStage<Simd8>`, the eight-lane arm of `FaderStageKernel`.
#[allow(dead_code)]
#[repr(align(32))]
struct FaderRampStageEightMirror {
    ramp: [GainMuteRampEightMirror; 2],
    fader_gain: [[f32; 8]; 2],
    muted: [[bool; 8]; 2],
    remaining: [[u32; 8]; 2],
}

/// `FaderRampStage<Simd4>`, the four-lane arm.
#[allow(dead_code)]
#[repr(align(16))]
struct FaderRampStageFourMirror {
    ramp: [GainMuteRampFourMirror; 2],
    fader_gain: [[f32; 8]; 2],
    muted: [[bool; 8]; 2],
    remaining: [[u32; 8]; 2],
}

#[allow(dead_code, clippy::large_enum_variant)]
enum FaderStageKernelMirror {
    Simd4(FaderRampStageFourMirror),
    Simd8(FaderRampStageEightMirror),
}

#[allow(dead_code)]
struct BuiltinFaderBankMirror {
    backend: Backend,
    width: effect_contract::BankWidth,
    members: usize,
    stage: FaderStageKernelMirror,
}

/// `Matrix2x2Coef<Simd8>` / `Matrix2x2Ramp<Simd8>` and the per-lane bookkeeping around them.
#[allow(dead_code)]
#[repr(align(32))]
struct MatrixStageEightMirror {
    coef: [[f32; 8]; 5],
    ramp_current: [[f32; 8]; 4],
    ramp_target: [[f32; 8]; 4],
    ramp_step: [[f32; 8]; 4],
    ramp_remaining: [f32; 8],
    smoothing_samples: [u32; 8],
    remaining: [u32; 8],
}

#[allow(dead_code)]
#[repr(align(16))]
struct MatrixStageFourMirror {
    coef: [[f32; 4]; 5],
    ramp_current: [[f32; 4]; 4],
    ramp_target: [[f32; 4]; 4],
    ramp_step: [[f32; 4]; 4],
    ramp_remaining: [f32; 4],
    smoothing_samples: [u32; 8],
    remaining: [u32; 8],
}

#[allow(dead_code, clippy::large_enum_variant)]
enum MatrixStageKernelMirror {
    Simd4(MatrixStageFourMirror),
    Simd8(MatrixStageEightMirror),
}

#[allow(dead_code)]
struct BuiltinMatrixBankMirror {
    backend: Backend,
    width: effect_contract::BankWidth,
    members: usize,
    stage: MatrixStageKernelMirror,
}

/// `Consumer<T>`: an `Arc` to the shared ring plus the consumer's own cursors and counters.
///
/// The ring itself is not here -- it is charged where it is created, per controlled track -- so
/// this mirrors only the handle a bank lane holds.
#[allow(dead_code)]
struct ConsumerMirror {
    /// `Arc<Ring<T>>`. Non-null, so `Option<Consumer<T>>` takes the pointer's niche and costs a
    /// lane nothing for being unaddressed -- which is why the array below is charged flat.
    ring: core::ptr::NonNull<()>,
    local: usize,
    cached_producer: usize,
    successes: u64,
    empty: u64,
}

/// `ChannelParameters`: one dual-mono side of a track's declared builtin parameters.
#[allow(dead_code)]
struct ChannelParametersMirror {
    polarity_invert: bool,
    trim_db: f32,
    hpf_hz: f32,
    lpf_hz: f32,
    fader_db: f32,
    muted: bool,
}

/// `BuiltinParameters`: both sides, the declared matrix and its window.
#[allow(dead_code)]
struct BuiltinParametersMirror {
    left: ChannelParametersMirror,
    right: ChannelParametersMirror,
    matrix: Matrix2x2Mirror,
    smoothing_samples: u32,
}

/// `StripControlConsumers`: one track's three live-console consumers.
#[allow(dead_code)]
struct StripControlConsumersMirror {
    /// The input trim/polarity consumer (#210 phase 3).
    input: Option<ConsumerMirror>,
    fader: Option<ConsumerMirror>,
    matrix: Option<ConsumerMirror>,
}

/// `StripPreparation`: a track's three sections before their binding form is chosen.
///
/// Held inline in the strip vector since issue #212, which is why the boxed `FaderProcessor` and
/// `MatrixProcessor` rows are gone from `builtin_owners` -- the same sections, one indirection
/// fewer, and the console's consumers alongside them so that whichever owner ends up rendering the
/// track gets them. Issue #210 phase 3 moved the **input** section in for the same reason: once it
/// has a console channel, which owner drains it is a lowering decision, so the boxed
/// `InputProcessor` row is gone from `builtin_owners` too and its section lives here.
#[allow(dead_code)]
struct StripPreparationMirror {
    track_id: Box<str>,
    graph_id: graph::StableGraphId,
    parameters: BuiltinParametersMirror,
    input: InputBuiltinsMirror,
    fader: FaderBuiltinsMirror,
    matrix: MatrixBuiltinsMirror,
    control: Option<StripControlConsumersMirror>,
}

/// `FaderBankProcessor`: the bank plus one optional console consumer per lane.
///
/// The consumer array is a `Box<[Option<Consumer<T>>]>` of exactly `lanes` entries, allocated
/// whether or not a console is attached -- so it is a row of its own below rather than part of
/// this struct's own size.
#[allow(dead_code)]
struct FaderBankProcessorMirror {
    bank: BuiltinFaderBankMirror,
    controls: (usize, usize),
    process_calls: u64,
    frames_processed: u64,
    control_delivery: graph::BuiltinControlDelivery,
}

#[allow(dead_code)]
struct MatrixBankProcessorMirror {
    bank: BuiltinMatrixBankMirror,
    controls: (usize, usize),
    process_calls: u64,
    frames_processed: u64,
    control_delivery: graph::BuiltinControlDelivery,
}

#[derive(Clone, Copy)]
struct PrimitiveOwner {
    name: &'static str,
    bytes: u64,
}

fn bytes<T>(count: usize) -> u64 {
    Layout::array::<T>(count).expect("primitive layout").size() as u64
}

fn spsc<T>(capacity: usize, name: &'static str) -> [PrimitiveOwner; 2] {
    [
        PrimitiveOwner {
            name,
            bytes: bytes::<SharedRingMirror<T>>(1),
        },
        PrimitiveOwner {
            name,
            bytes: bytes::<UnsafeCell<MaybeUninit<T>>>(capacity + 1),
        },
    ]
}

fn owner_total(rows: &[PrimitiveOwner]) -> u64 {
    rows.iter().map(|owner| owner.bytes).sum()
}

fn fixture_usize(key: &str) -> usize {
    let model = session::parse_session_json(SESSION).expect("oracle fixture");
    match key {
        "quantum_frames" => model.quantum_frames as usize,
        _ => panic!("missing fixture numeric field {key}"),
    }
}

fn assert_effective_owner_mutations(rows: &[PrimitiveOwner], production: u64, group: &str) {
    assert_eq!(owner_total(rows), production, "{group} authority");
    for index in 0..rows.len() {
        let mut omitted = rows.to_vec();
        let removed = omitted.remove(index);
        assert_ne!(
            owner_total(&omitted),
            production,
            "{group} omitted {}",
            removed.name
        );
        let mut miscounted = rows.to_vec();
        miscounted[index].bytes = miscounted[index]
            .bytes
            .checked_add(1)
            .expect("one-byte mutation");
        assert_ne!(
            owner_total(&miscounted),
            production,
            "{group} miscounted {}",
            rows[index].name
        );
    }
}

fn complete_capi_owners(
    current_canonical: usize,
    candidate_canonical: usize,
) -> (u64, u64, u64, u64) {
    let configuration_items = 4_096 / size_of::<u16>();
    let mut active = Vec::new();
    for owner in spsc::<ControlCommandSlot>(1, "control queue")
        .into_iter()
        .chain(spsc::<AutomationBatchSlot>(1, "automation queue"))
        .chain(spsc::<ReliableSlot>(1, "response queue"))
        .chain(spsc::<ReliableSlot>(2, "event queue"))
        .chain(spsc::<TelemetryRecord>(1, "meter queue"))
        .chain(spsc::<CounterTelemetryRecord>(1, "counter queue"))
    {
        active.push(owner);
    }
    active.extend([
        PrimitiveOwner {
            name: "pending meter",
            bytes: bytes::<Option<TelemetryRecord>>(1),
        },
        PrimitiveOwner {
            name: "pending counter",
            bytes: bytes::<Option<CounterTelemetryRecord>>(1),
        },
        PrimitiveOwner {
            name: "automation density",
            bytes: bytes::<DensityMirror>(AUTOMATION_BATCH_RECORDS),
        },
        PrimitiveOwner {
            name: "automation intervals",
            bytes: bytes::<IntervalMirror>(AUTOMATION_BATCH_RECORDS),
        },
        PrimitiveOwner {
            name: "queue telemetry counter Arc",
            bytes: bytes::<SharedCounterMirror>(1),
        },
    ]);
    let replay_rows = [
        PrimitiveOwner {
            name: "current replay entries",
            bytes: bytes::<ReplayEntryMirror>(16),
        },
        PrimitiveOwner {
            name: "current replay bytes",
            bytes: 8_192,
        },
    ];
    active.extend(replay_rows);
    for owner in spsc::<PublishedPlanMirror>(1, "publication queue")
        .into_iter()
        .chain(spsc::<RetiredPlanMirror>(1, "retirement queue"))
    {
        active.push(owner);
    }
    active.extend([
        PrimitiveOwner {
            name: "retirement credit Arc",
            bytes: bytes::<SharedCounterMirror>(1),
        },
        PrimitiveOwner {
            name: "legacy credit Arc",
            bytes: bytes::<SharedCounterMirror>(1),
        },
        PrimitiveOwner {
            name: "session diagnostics",
            bytes: 4_096,
        },
        PrimitiveOwner {
            name: "decode fields",
            bytes: 4_096,
        },
        PrimitiveOwner {
            name: "response scratch",
            bytes: 4_096,
        },
        PrimitiveOwner {
            name: "structural generation Arc",
            bytes: bytes::<SharedArcMirror<AtomicU64>>(1),
        },
        PrimitiveOwner {
            name: "shared plan-state Arc",
            bytes: bytes::<SharedArcMirror<SharedPlanStateMirror>>(1),
        },
        PrimitiveOwner {
            name: "diagnostic retained slots",
            bytes: bytes::<RetainedDiagnosticSlotMirror>(2),
        },
        PrimitiveOwner {
            name: "CAPI render diagnostic slots",
            bytes: bytes::<RenderDiagnosticSlotMirror>(2),
        },
        PrimitiveOwner {
            name: "CAPI render diagnostic code payloads",
            bytes: 2 * "capi.render.activity".len() as u64,
        },
        PrimitiveOwner {
            name: "provider meter config",
            bytes: bytes::<u32>(configuration_items),
        },
        PrimitiveOwner {
            name: "provider counter config",
            bytes: bytes::<CounterId>(configuration_items),
        },
        PrimitiveOwner {
            name: "provider counter values",
            bytes: bytes::<CounterValue>(configuration_items),
        },
        PrimitiveOwner {
            name: "provider parameter descriptor arena",
            bytes: 9_072,
        },
        PrimitiveOwner {
            name: "provider parameter state arena",
            bytes: 864,
        },
        PrimitiveOwner {
            name: "provider parameter text payloads",
            bytes: 864,
        },
        PrimitiveOwner {
            name: "provider diagnostic projection arena",
            bytes: 240,
        },
        PrimitiveOwner {
            name: "provider diagnostic occupancy arena",
            bytes: 2,
        },
        PrimitiveOwner {
            name: "provider diagnostic code payloads",
            bytes: 40,
        },
        PrimitiveOwner {
            name: "controller meter config",
            bytes: bytes::<u32>(configuration_items),
        },
        PrimitiveOwner {
            name: "controller counter config",
            bytes: bytes::<CounterId>(configuration_items),
        },
        PrimitiveOwner {
            name: "provider epoch arena",
            bytes: bytes::<ProviderEpochMirror>(2),
        },
        PrimitiveOwner {
            name: "plan report arena",
            bytes: bytes::<(u64, PlanResourceReport)>(2),
        },
        PrimitiveOwner {
            name: "session handle",
            bytes: bytes::<Session>(1),
        },
        PrimitiveOwner {
            name: "plan handle",
            bytes: bytes::<Plan>(1),
        },
        PrimitiveOwner {
            name: "current canonical JSON",
            bytes: current_canonical as u64,
        },
        PrimitiveOwner {
            name: "current source controls",
            bytes: bytes::<ControlSourceMirror>(1),
        },
        PrimitiveOwner {
            name: "current source IDs",
            bytes: 14,
        },
    ]);
    let active_total = owner_total(&active);
    // #84 re-pin (+1,336 net): phase B grew every ring header 72 -> 256 (one cache line for the
    // read-mostly header plus one per cursor) and each endpoint by 8 (cached peer cursor); phase C
    // deleted the plan's unused parameter/event store (-96 per live plan row). The rows above are
    // sized from the live layouts, and `resources_c` at the frozen-scratch comparison must agree.
    //
    // #146 re-pin (+8): `PlanState` gained the render-thread floating-point attestation flag, a
    // `Cell<bool>` that costs a machine word of padding inside the `Plan` handle. It is the whole
    // of the change: the first block a plan renders proves the canonical environment took on that
    // thread, and every block after it reads an already-set flag. Eight bytes per live plan handle,
    // once, and no per-block or per-track cost anywhere.
    //
    // #210 phase 2 re-pin (+342): the active session's canonical JSON row is the fixture's own
    // byte count, and every one of its nine tracks gained `", delay_samples = 0"` on both lanes.
    // #241 re-pin (-195): the canonical session is 171 bytes shorter and the session handle's
    // protocol controller shrinks by 24 bytes after its deleted edit variants leave, so
    // #338: canonical JSON adds 8,082 retained bytes to the active session model.
    // #369: the production provider retains the fixture's 864 descriptor/state rows and its
    // descriptor-owned strings/enumerations, plus two bounded render-diagnostic projections.
    assert_effective_owner_mutations(&active, 160_933, "active CAPI");

    let candidate_epoch_rows = [
        PrimitiveOwner {
            name: "candidate canonical JSON",
            bytes: candidate_canonical as u64,
        },
        PrimitiveOwner {
            name: "candidate source controls",
            bytes: bytes::<ControlSourceMirror>(1),
        },
        PrimitiveOwner {
            name: "candidate source IDs",
            bytes: 14,
        },
    ];
    let candidate_epoch = owner_total(&candidate_epoch_rows);
    let prepared_rows = [
        PrimitiveOwner {
            name: "prepared response",
            bytes: 4_096,
        },
        PrimitiveOwner {
            name: "prepared affine token",
            bytes: bytes::<protocol::PreparedStructuralCommand>(1),
        },
        PrimitiveOwner {
            name: "candidate replay entries",
            bytes: bytes::<ReplayEntryMirror>(16),
        },
        PrimitiveOwner {
            name: "candidate replay bytes",
            bytes: 8_192,
        },
        PrimitiveOwner {
            name: "candidate provider parameter descriptor arena",
            bytes: 9_072,
        },
        PrimitiveOwner {
            name: "candidate provider parameter state arena",
            bytes: 864,
        },
        PrimitiveOwner {
            name: "candidate provider parameter text payloads",
            bytes: 864,
        },
    ];
    let prepared = owner_total(&prepared_rows);
    // #84 phase B re-pin (+24): `ControlSourceMirror` carries three spsc endpoints, each +8 for
    // its cached peer cursor.
    // #210 phase 2 re-pin (+342): the candidate's canonical JSON row, same key on the same tracks.
    // #338: canonical JSON adds 8,082 retained bytes to the candidate session model.
    assert_effective_owner_mutations(&candidate_epoch_rows, 18_706, "candidate CAPI epoch");
    // #241: `PreparedStructuralCommand` loses the same deleted edit payload (-24).
    assert_effective_owner_mutations(&prepared_rows, 24_736, "prepared protocol");
    let largest = active
        .iter()
        .chain(candidate_epoch_rows.iter())
        .chain(prepared_rows.iter())
        .map(|owner| owner.bytes)
        .max()
        .expect("named owner");
    (active_total, candidate_epoch, prepared, largest)
}

fn compiled_model_owners(session_id: &str, canonical: &str) -> Vec<PrimitiveOwner> {
    let sources = 1_u64;
    let tracks = 9_u64;
    let outputs = 1_u64;
    let effects = 9_u64;
    let parameters = 18_u64;
    vec![
        PrimitiveOwner {
            name: "source declarations",
            bytes: bytes::<session::Source>(1),
        },
        PrimitiveOwner {
            name: "track declarations",
            bytes: bytes::<session::Track>(tracks as usize),
        },
        PrimitiveOwner {
            name: "output declarations",
            bytes: bytes::<session::Output>(1),
        },
        PrimitiveOwner {
            name: "route declarations",
            bytes: bytes::<session::Route>(9),
        },
        PrimitiveOwner {
            name: "effect declarations",
            bytes: bytes::<session::Effect>(effects as usize),
        },
        PrimitiveOwner {
            name: "effect parameter declarations",
            bytes: bytes::<session::EffectParam>(parameters as usize),
        },
        PrimitiveOwner {
            name: "session ID",
            bytes: session_id.len() as u64,
        },
        PrimitiveOwner {
            name: "render profile ID",
            bytes: "native".len() as u64,
        },
        PrimitiveOwner {
            name: "output profile ID",
            bytes: "main".len() as u64,
        },
        PrimitiveOwner {
            name: "source ID",
            bytes: "fixture-source".len() as u64,
        },
        PrimitiveOwner {
            name: "source content identity",
            bytes: "sha256:7e945c107a97cd24135e85dc2f407c5ecd39663a8737bf5b92114ccce38f1ab8".len()
                as u64,
        },
        PrimitiveOwner {
            name: "track IDs",
            bytes: tracks * 3,
        },
        PrimitiveOwner {
            name: "track source IDs",
            bytes: tracks * "fixture-source".len() as u64,
        },
        PrimitiveOwner {
            name: "effect slot IDs",
            bytes: effects * "soft-clip".len() as u64,
        },
        PrimitiveOwner {
            name: "native effect IDs",
            bytes: effects * "miso.soft-clip".len() as u64,
        },
        PrimitiveOwner {
            name: "output IDs",
            bytes: "main-out".len() as u64,
        },
        PrimitiveOwner {
            name: "route IDs",
            bytes: 9 * "eq0-main".len() as u64,
        },
        PrimitiveOwner {
            name: "route source track IDs",
            bytes: tracks * 3,
        },
        PrimitiveOwner {
            name: "route output IDs",
            bytes: 9 * "main-out".len() as u64,
        },
        PrimitiveOwner {
            name: "compiled source index node storage",
            bytes: bytes::<CompiledIndexNodeMirror>(sources as usize),
        },
        PrimitiveOwner {
            name: "compiled graph-entity index node storage",
            bytes: bytes::<CompiledIndexNodeMirror>((tracks + outputs) as usize),
        },
        PrimitiveOwner {
            name: "canonical session snapshot",
            bytes: canonical.len() as u64,
        },
    ]
}

fn graph_owners() -> Vec<PrimitiveOwner> {
    let tracks = 9_u64;
    let nodes = tracks * 8 + 9 + 1;
    let edges = tracks * 7 + 9 * 2;
    let schedule = nodes;
    let dependency_levels = 10_u64;
    let buffers = nodes;
    let route_timings = 9_u64;
    let quantum = fixture_usize("quantum_frames") as u64;
    let colored_outputs = 10_u64;
    let maximum_inputs = 9_u64;
    let bank_lanes = 8_u64;
    let node_text_bytes = tracks
        * ([
            "input",
            "post-input-builtins",
            "post-simd1",
            "post-dynamic",
            "post-simd2-pre-fader",
            "post-fader",
            "post-matrix",
        ]
        .iter()
        .map(|stage| "track:".len() as u64 + 3 + 1 + stage.len() as u64)
        .sum::<u64>()
            + "effect:".len() as u64
            + 3
            + 1
            + "simd1".len() as u64
            + 1
            + "soft-clip".len() as u64
            + "route:".len() as u64
            + "eq0-main".len() as u64)
        + "output:main-out".len() as u64;
    let track_edge_text = {
        let stage_text = |stage: &str| "track:".len() as u64 + 3 + 1 + stage.len() as u64;
        let effect_text = "effect:eq0:simd1:soft-clip".len() as u64;
        let chain = [
            stage_text("input"),
            stage_text("post-input-builtins"),
            effect_text,
            stage_text("post-simd1"),
            stage_text("post-dynamic"),
            stage_text("post-simd2-pre-fader"),
            stage_text("post-fader"),
            stage_text("post-matrix"),
        ];
        (0..7)
            .map(|index| {
                chain[index]
                    + chain[index + 1]
                    + if index == 1 {
                        "$.tracks[id=eq0].simd1.effects[id=soft-clip]".len() as u64
                    } else {
                        "$.tracks".len() as u64
                    }
            })
            .sum::<u64>()
    };
    let route_edge_text = "track:eq0:post-matrix".len() as u64
        + "route:eq0-main".len() as u64
        + "$.routes[id=eq0-main].source".len() as u64
        + "route:eq0-main".len() as u64
        + "output:main-out".len() as u64
        + "$.routes[id=eq0-main].destination".len() as u64;
    let audio_samples = (colored_outputs + edges) * 2 * quantum + maximum_inputs;
    // Soft-clip state layout 2 (issue #91): 104 effect-owned words per channel, plus the two
    // header words the shared payload codec stamps into the common section.
    let effect_lane_state = 104_u64 * size_of::<f32>() as u64;
    let effect_common_state = 2_u64 * size_of::<f32>() as u64;
    let effect_bank_plane = quantum * bank_lanes * size_of::<f32>() as u64;
    let builtin_banks = tracks.div_ceil(bank_lanes);
    let builtin_bank_processor = bytes::<BuiltinBankProcessorMirror>(1);
    // Strip round job 1: `InputStage` gained the elision plan. The field is four bytes and the
    // struct got *smaller* -- 1248 -> 1216 -- because the four align-1 bytes let the layout
    // algorithm pack the tail differently. That is exactly why this mirror exists rather than a
    // `size_of` of the real type: the model has to restate the field list, and a restatement that
    // disagreed with production would be caught by the one-below cap arms below rather than
    // silently absorbed.
    //
    // Issue #210 phase 3 moved it 1_216 -> 1_568, and every one of the 352 bytes is a restated
    // field. `InputStageKernel` is an enum sized by its **larger** variant, so the growth is
    // `InputStage<Simd8>`'s whether or not the host selects eight lanes: the trim ramp is four
    // `[f32; 8]` words per channel (256) and the authoritative countdown is `[[u32; 8]; 2]` (64).
    // The processor itself gained the third drain's `Box<[Option<Consumer<_>>]>` (16), the
    // eight-byte per-lane live-witness array and the `ramping` flag, and eight bytes of tail
    // padding: 256 + 64 + 16 + 8 + 8 = 352. The consumer array's *heap* is charged separately
    // below, by `strip_control_array`, which the input row now takes exactly as the fader and
    // matrix rows already did.
    assert_eq!(
        builtin_bank_processor, 1_568,
        "primitive builtin bank processor"
    );
    // Strip round job 2: the fader and the matrix are bankable stages too, so this fixture binds
    // `3 * 9.div_ceil(8) == 6` builtin banks. Each stage groups the same nine tracks the same way,
    // so the descriptor, member-ID, member-string and scratch rows are all three times what they
    // were, while the processor row splits into three per-kind rows.
    let strip_stages = 3_u64;
    let strip_banks = builtin_banks * strip_stages;
    let fader_bank_processor = bytes::<FaderBankProcessorMirror>(1);
    let matrix_bank_processor = bytes::<MatrixBankProcessorMirror>(1);
    // One `Option<Consumer<_>>` per lane, allocated whether or not a console is attached: a
    // banked session's retained payload does not depend on whether the host leased one. All
    // **three** strip stages carry one since #210 phase 3 gave the input bank its drain; the three
    // record types are all 12 bytes, so one term serves all three rows.
    let strip_control_array = bytes::<Option<ConsumerMirror>>(bank_lanes as usize);
    vec![
        // #241 deletes the session control queue and declarative source-ring projection. Their
        // absence is the assertion here: the chosen ring is charged exactly by `source_owners`,
        // outside the graph/model cap. Zero-byte rows would defeat the effective-owner check.
        PrimitiveOwner {
            name: "graph planar audio buffers",
            bytes: audio_samples * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "effect left state words",
            bytes: tracks * effect_lane_state,
        },
        PrimitiveOwner {
            name: "effect right state words",
            bytes: tracks * effect_lane_state,
        },
        PrimitiveOwner {
            name: "effect common state words",
            bytes: tracks * effect_common_state,
        },
        PrimitiveOwner {
            name: "effect fixed scratch",
            bytes: tracks * 24,
        },
        PrimitiveOwner {
            name: "graph node array",
            bytes: nodes * size_of::<graph::GraphNode>() as u64,
        },
        PrimitiveOwner {
            name: "graph edge array",
            bytes: edges * size_of::<graph::GraphEdge>() as u64,
        },
        PrimitiveOwner {
            name: "graph schedule array",
            bytes: schedule * size_of::<graph::GraphNodeId>() as u64,
        },
        PrimitiveOwner {
            name: "graph dependency levels",
            bytes: dependency_levels * size_of::<graph::DependencyLevel>() as u64,
        },
        PrimitiveOwner {
            name: "graph buffer assignments",
            bytes: buffers * size_of::<graph::BufferAssignment>() as u64,
        },
        PrimitiveOwner {
            name: "graph route timings",
            bytes: route_timings * size_of::<graph::RouteTiming>() as u64,
        },
        PrimitiveOwner {
            name: "graph node stable-ID text",
            bytes: node_text_bytes,
        },
        PrimitiveOwner {
            name: "graph edge path and endpoint text",
            bytes: tracks * track_edge_text + 9 * route_edge_text,
        },
        PrimitiveOwner {
            name: "effect-bank descriptor array",
            bytes: bytes::<graph::GraphPreparedEffectBank>(1),
        },
        PrimitiveOwner {
            name: "effect-bank member IDs",
            bytes: bytes::<graph::EffectNodeId>(bank_lanes as usize),
        },
        PrimitiveOwner {
            name: "effect-bank member strings",
            bytes: bank_lanes * (3 + "soft-clip".len() as u64),
        },
        PrimitiveOwner {
            name: "effect-bank active mask",
            bytes: bytes::<bool>(bank_lanes as usize),
        },
        PrimitiveOwner {
            name: "effect-bank two-plane scratch",
            bytes: effect_bank_plane * 2,
        },
        PrimitiveOwner {
            name: "effect-bank two-plane runtime",
            bytes: effect_bank_plane * 2,
        },
        // #86 F3/F4: the nine post-input nodes are one full eight-lane bank plus a one-member
        // bank padded with seven identity lanes -- `9.div_ceil(8) == 2` -- and each bank owns
        // two main planes, not four (a fixed stage has no sidechain surface). No lane mask is
        // stored anywhere: membership is the mask.
        PrimitiveOwner {
            name: "builtin-bank descriptor array",
            bytes: bytes::<graph::GraphPreparedBuiltinBank>(strip_banks as usize),
        },
        PrimitiveOwner {
            name: "builtin-bank member IDs",
            bytes: bytes::<graph::GraphNodeId>((tracks * strip_stages) as usize),
        },
        PrimitiveOwner {
            name: "builtin-bank member strings",
            bytes: tracks * 3 * strip_stages,
        },
        PrimitiveOwner {
            name: "builtin-bank post-input processors",
            bytes: (builtin_bank_processor + strip_control_array) * builtin_banks,
        },
        PrimitiveOwner {
            name: "builtin-bank fader processors",
            bytes: (fader_bank_processor + strip_control_array) * builtin_banks,
        },
        PrimitiveOwner {
            name: "builtin-bank matrix processors",
            bytes: (matrix_bank_processor + strip_control_array) * builtin_banks,
        },
        PrimitiveOwner {
            name: "builtin-bank two-plane scratch",
            bytes: effect_bank_plane * 2 * strip_banks,
        },
    ]
}

fn source_owners() -> Vec<PrimitiveOwner> {
    let blocks = 1_024_usize / 128;
    let channels = 2_usize;
    let mappings = 9_usize;
    let data = spsc::<Box<TransferBlockMirror>>(blocks, "source data queue");
    let recycle = spsc::<Box<TransferBlockMirror>>(blocks, "source recycle queue");
    let command = spsc::<source::SourceCommand>(1, "source command queue");
    vec![
        PrimitiveOwner {
            name: "source PCM transfer blocks",
            bytes: bytes::<f32>(1_024 * channels),
        },
        data[0],
        data[1],
        recycle[0],
        recycle[1],
        command[0],
        command[1],
        PrimitiveOwner {
            name: "source transfer-block metadata",
            bytes: bytes::<TransferBlockMirror>(blocks),
        },
        PrimitiveOwner {
            name: "graph source entries",
            bytes: bytes::<GraphSourceEntryMirror>(1),
        },
        PrimitiveOwner {
            name: "graph source mappings",
            bytes: bytes::<source::SourceGraphTrackMapping>(mappings),
        },
        PrimitiveOwner {
            name: "graph source claims",
            bytes: bytes::<graph::GraphSourceInputClaim>(mappings),
        },
        PrimitiveOwner {
            name: "graph source driver",
            bytes: bytes::<SourceGraphSourceSetDriverMirror>(1),
        },
        PrimitiveOwner {
            name: "graph source mapping stable IDs",
            bytes: (mappings * 2 * 3) as u64,
        },
    ]
}

fn builtin_owners() -> Vec<PrimitiveOwner> {
    let tracks = 9_usize;
    let processors = tracks * 3;
    // The "builtin graph bindings" and "builtin input processors" rows this list used to open and
    // close with are **gone**, and their absence is the statement: since #210 phase 3 all three of
    // a track's stages ride the strip vector below until lowering decides whether they bind per
    // node or as bank lanes, so preparation allocates no binding vector and boxes no processor at
    // all. `assert_effective_owner_mutations` requires every row to be load-bearing, so a row that
    // charges nothing cannot be left standing to say so; this comment says it instead.
    vec![
        PrimitiveOwner {
            name: "builtin strip preparations",
            bytes: bytes::<StripPreparationMirror>(tracks),
        },
        PrimitiveOwner {
            name: "builtin bank-input table",
            bytes: bytes::<(Box<str>, InputBuiltinsMirror)>(tracks),
        },
        PrimitiveOwner {
            name: "builtin tail table",
            bytes: bytes::<(Box<str>, BuiltinTailMirror)>(tracks),
        },
        PrimitiveOwner {
            name: "builtin track seal",
            bytes: bytes::<Box<str>>(tracks),
        },
        PrimitiveOwner {
            name: "builtin processor seal",
            bytes: bytes::<(Box<str>, graph::TrackStage)>(processors),
        },
        PrimitiveOwner {
            name: "builtin cloned tail seal",
            bytes: bytes::<(Box<str>, BuiltinTailMirror)>(tracks),
        },
        PrimitiveOwner {
            // Nine copies per track since #210 phase 3, not ten: the post-input binding's node ID
            // went with the binding. Each of this fixture's nine track IDs is three characters.
            name: "builtin stable-ID payload copies",
            bytes: (tracks * 9 * 3) as u64,
        },
    ]
}

fn canonical_writer_owners(session_id: &str) -> Vec<PrimitiveOwner> {
    let tracks = 9;
    let effects = 9;
    let parameters = 18;
    let mut owners = vec![PrimitiveOwner {
        name: "canonical document prelude frames",
        bytes: bytes::<CanonicalDocumentPreludeMirror>(1),
    }];
    for owner in compiled_model_owners(session_id, "")
        .into_iter()
        .skip(6)
        // #241 leaves thirteen retained-string rows after deleting `source.locator`.
        .take(13)
    {
        owners.push(PrimitiveOwner {
            name: owner.name,
            bytes: bytes::<CanonicalEscapedByteMirror>(owner.bytes as usize),
        });
    }
    owners.extend([
        PrimitiveOwner {
            name: "canonical source structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(1),
        },
        PrimitiveOwner {
            name: "canonical track structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(tracks),
        },
        PrimitiveOwner {
            name: "canonical output structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(1),
        },
        PrimitiveOwner {
            name: "canonical route structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(tracks),
        },
        PrimitiveOwner {
            name: "canonical effect structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(effects),
        },
        PrimitiveOwner {
            name: "canonical parameter structural storage",
            bytes: bytes::<CanonicalStructuralItemMirror>(parameters),
        },
    ]);
    owners
}

fn primitive_replacement_oracle(current: &str, prospective: &str) -> PrimitiveReplacementOracle {
    let mut graph = graph_owners();
    let prospective_graph = graph.clone();
    graph.extend(prospective_graph);
    let current_model = compiled_model_owners("parametric-eq-nine-track", current);
    let prospective_model = compiled_model_owners("double-live-cap", prospective);
    graph.extend(current_model);
    graph.extend(prospective_model);
    // Issue #181 moved this by 16 bytes: `size_of::<GraphPreparedEffectBank>()` went 88 -> 96
    // when the bank started carrying the cohort chain it is a slot of, the fixture binds one
    // bank, and this oracle holds **two** plans live at once -- so eight bytes are counted twice.
    // (`frozen_scratch_report`, which describes a single plan, moves by eight.) This is a real
    // retained byte and it is reported rather than absorbed: the whole point of the double-live
    // oracle is that a struct that grew says so, in both the model and the measurement.
    //
    // The strip round moved it by 128 in the other direction: `InputStage` gained the elision plan
    // and `size_of::<BuiltinBankProcessor>()` went 1248 -> 1216, the fixture binds two banks, and
    // this oracle holds two plans live -- so 2 x 2 x 32. A struct that *shrank* says so too.
    //
    // Strip round job 2 moved it by +75_980: the fader and the matrix became bankable stages, so
    // the fixture binds six builtin banks where it bound two, and the descriptor, member-ID,
    // member-string and two-plane-scratch rows all tripled while two per-kind processor rows --
    // each carrying its eight-entry array of optional console consumers -- joined the post-input
    // one. Held over two live plans, as everything in this oracle is -- and `+75_980` is exactly
    // twice the `236_980 - 198_990` that `frozen_scratch_report` records for a single plan, which
    // is the cross-check that this model tracks the tree rather than having been tuned to it.
    // Issue #210 phase 2 moved it by +828, all of it model rather than plan: `ChannelBuiltins`
    // gained a required `delay_samples: u32`, so `size_of::<Track>()` grew by eight (two lanes,
    // 16 -> 20 bytes each with the trailing padding reused) and the canonical text of each session
    // grew by 342 (9 tracks x 2 lanes x `", delay_samples = 0"`). Held over two live plans:
    // 2 x (9 x 8) + 2 x 342 = 828.
    //
    // Issue #210 phase 3 moved it by +2_688, all of it plan and none of it model: the input bank
    // grew by 672 -- 352 in `BuiltinBankProcessor` itself (the trim ramp, the countdown, the
    // `Box<[Option<Consumer<_>>]>` and the live-witness array) and 320 for the eight-lane consumer
    // array's heap -- and the fixture binds two input banks. Held over two live plans:
    // 2 x 2 x 672 = 2_688, which is exactly twice the `9_209 - 7_865` this phase records for
    // `builtin_bank_bytes` in `frozen_scratch_report`. The session model does not move: this
    // phase adds no schema key.
    // #241: the two plans lose 4_096 queue + 8_192 ring projection each (-24_576), and the two
    // compiled models each shrink by 200 bytes (-400): 510_720 - 24_576 - 400 = 485_744.
    // #338: canonical JSON adds 8,082 retained bytes to each of the two live models.
    assert_effective_owner_mutations(&graph, 502_164, "double-live graph/model");

    let source = source_owners();
    assert_eq!(owner_total(&source), 11_054, "primitive source total");
    let source_overhead_rows = source[1..].to_vec();
    assert_effective_owner_mutations(&source_overhead_rows, 2_862, "source overhead");
    let mut source_total_rows = source.clone();
    source_total_rows.extend(source.clone());
    assert_effective_owner_mutations(&source_total_rows, 22_108, "double-live source total");
    let mut double_source_overhead = source_overhead_rows.clone();
    double_source_overhead.extend(source_overhead_rows);
    assert_effective_owner_mutations(
        &double_source_overhead,
        5_724,
        "double-live source overhead",
    );

    // Soft-clip state layout 2 (issue #91): 104 effect-owned words per channel, plus the two
    // header words `effect-runtime`'s payload codec stamps into the common section.
    let effect_state_rows = vec![
        PrimitiveOwner {
            name: "current effect left state",
            bytes: 9 * 104 * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "current effect right state",
            bytes: 9 * 104 * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "current effect common state",
            bytes: 9 * 2 * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "prospective effect left state",
            bytes: 9 * 104 * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "prospective effect right state",
            bytes: 9 * 104 * size_of::<f32>() as u64,
        },
        PrimitiveOwner {
            name: "prospective effect common state",
            bytes: 9 * 2 * size_of::<f32>() as u64,
        },
    ];
    assert_effective_owner_mutations(&effect_state_rows, 15_120, "double-live effect state");
    let effect_scratch_rows = vec![
        PrimitiveOwner {
            name: "current effect fixed scratch",
            bytes: 9 * 24,
        },
        PrimitiveOwner {
            name: "prospective effect fixed scratch",
            bytes: 9 * 24,
        },
    ];
    assert_effective_owner_mutations(&effect_scratch_rows, 432, "double-live effect scratch");
    let builtin = builtin_owners();
    // Strip round job 2: +48 bytes per track, and nine tracks makes +432. Two `GraphNodeBinding`s
    // (2 x 72) and the boxed fader (16) and matrix (136) sections left preparation, and the
    // 344-byte `StripPreparation` entry replaced them.
    //
    // Issue #210 phase 3: +173 bytes per track on this fixture, and nine tracks makes +1_557.
    // Every term is a restated field: the `GraphNodeBinding` vector leaves (-72), the boxed
    // `InputProcessor` leaves (-168), one of the ten track-ID copies leaves with the binding
    // (-3 at this fixture's three-character IDs), `StripPreparation` gains the input section and
    // a third console consumer (344 -> 656, +312), and the bank-input table entry grows with
    // `InputBuiltins` (168 -> 272, +104). -72 - 168 - 3 + 312 + 104 = +173.
    //
    // This total is reached by restating the field lists, and it agrees with
    // `frozen_scratch_report`'s independently measured `builtin_processor_payload_bytes` -- which
    // is the whole point of holding both.
    assert_effective_owner_mutations(&builtin, 9_963, "current builtin payload");
    let mut double_builtin = builtin.clone();
    double_builtin.extend(builtin);
    assert_effective_owner_mutations(&double_builtin, 19_926, "double-live builtin payload");

    let (current_capi, candidate_epoch, prepared_protocol, capi_largest) =
        complete_capi_owners(current.len(), prospective.len());
    let capi_rows = [
        PrimitiveOwner {
            name: "current CAPI retained owners",
            bytes: current_capi,
        },
        PrimitiveOwner {
            name: "candidate CAPI epoch",
            bytes: candidate_epoch,
        },
        PrimitiveOwner {
            name: "prepared protocol owner",
            bytes: prepared_protocol,
        },
    ];
    // #338: canonical JSON adds 8,082 retained bytes to each live session model.
    assert_effective_owner_mutations(&capi_rows, 204_375, "double-live CAPI");

    let graph_rows = graph_owners();
    // The eight graph-metadata rows begin after the five audio/effect rows. #241 removed the
    // declarative control-queue and source-ring owners which formerly occupied indices zero/one.
    let graph_largest = owner_total(&graph_rows[5..13]);
    assert_eq!(graph_largest, 49_167, "primitive graph metadata allocation");
    let source_largest = source
        .iter()
        .skip(1)
        .map(|owner| owner.bytes)
        .chain(core::iter::once(bytes::<f32>(2 * 128)))
        .max()
        .expect("source owner");
    let current_canonical_writer = canonical_writer_owners("parametric-eq-nine-track");
    let prospective_canonical_writer = canonical_writer_owners("double-live-cap");
    let current_canonical_maximum = owner_total(&current_canonical_writer);
    let prospective_canonical_maximum = owner_total(&prospective_canonical_writer);
    let largest_candidates = [
        graph_largest,
        source_largest,
        capi_largest,
        current_canonical_maximum,
        prospective_canonical_maximum,
    ];
    let largest = largest_candidates.into_iter().max().expect("largest owner");
    // #241 canonical scratch: remove 29 locator bytes x 10, add 40 content-identity bytes x 10.
    // 58_694 - 290 + 400 = 58_804. #369's provider descriptor arena is 9,072 bytes here and does
    // not displace the canonical-writer maximum.
    assert_eq!(largest, 58_804, "primitive maximum-single authority");
    assert_effective_owner_mutations(
        &current_canonical_writer,
        largest,
        "current canonical writer maximum",
    );
    for index in 0..current_canonical_writer.len() {
        let mut omitted = current_canonical_writer.clone();
        omitted.remove(index);
        let actual = [
            graph_largest,
            source_largest,
            capi_largest,
            owner_total(&omitted),
            prospective_canonical_maximum,
        ]
        .into_iter()
        .max();
        assert_ne!(
            actual,
            Some(largest),
            "omitting canonical primitive owner {index} reaches final production cap comparison"
        );
        let mut miscounted = current_canonical_writer.clone();
        miscounted[index].bytes += 1;
        let actual = [
            graph_largest,
            source_largest,
            capi_largest,
            owner_total(&miscounted),
            prospective_canonical_maximum,
        ]
        .into_iter()
        .max();
        assert_ne!(
            actual,
            Some(largest),
            "miscounting canonical primitive owner {index} reaches final production cap comparison"
        );
    }
    let mut omitted_maximum = largest_candidates.to_vec();
    omitted_maximum.remove(3);
    assert_ne!(
        omitted_maximum.into_iter().max(),
        Some(largest),
        "omitting the current compiled-model maximum reaches the production comparison"
    );
    let mut miscounted_maximum = largest_candidates;
    miscounted_maximum[3] += 1;
    assert_ne!(
        miscounted_maximum.into_iter().max(),
        Some(largest),
        "miscounting the current compiled-model maximum reaches the production comparison"
    );
    assert_ne!(
        largest_candidates.into_iter().sum::<u64>(),
        largest,
        "maximum is not aggregate"
    );
    assert_ne!(
        capi_rows.iter().map(|owner| owner.bytes).max(),
        Some(166_882),
        "CAPI aggregate is not max-single"
    );

    PrimitiveReplacementOracle {
        graph: owner_total(&graph),
        source_total: owner_total(&source_total_rows),
        source_overhead: owner_total(&double_source_overhead),
        effect_state: owner_total(&effect_state_rows),
        effect_scratch: owner_total(&effect_scratch_rows),
        builtin: owner_total(&double_builtin),
        capi: owner_total(&capi_rows),
        largest,
    }
}

unsafe fn compile_c(
    session_document: &str,
    compile_limits: &CompileLimits,
) -> (*mut Session, *mut Plan) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::null_mut();
    let mut plan = ptr::null_mut();
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location remains live for the complete call.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v1_compile_session(
                engine,
                session_document.as_ptr(),
                session_document.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_OK,
            "{}",
            String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
        );
        miso_engine_v1_engine_destroy(engine);
    }
    (session, plan)
}

unsafe fn resources_c(plan: *const Plan) -> PlanResourceReport {
    // SAFETY: The caller supplies a live plan and this is a complete writable report.
    unsafe {
        let mut report: PlanResourceReport = core::mem::zeroed();
        report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        assert_eq!(miso_engine_v1_plan_resources(plan, &mut report), RESULT_OK);
        report
    }
}

unsafe fn compile_rejected_c(session_document: &str, compile_limits: &CompileLimits) {
    let config = EngineConfig {
        struct_size: ENGINE_CONFIG_SIZE,
        abi_version: ABI_VERSION,
        reserved: [0; 4],
    };
    let mut engine = ptr::null_mut();
    let mut session = ptr::dangling_mut();
    let mut plan = ptr::dangling_mut();
    let mut diagnostic_storage = [0_u8; 4_096];
    let mut diagnostics = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: diagnostic_storage.as_mut_ptr(),
        capacity_bytes: diagnostic_storage.len() as u64,
        required_bytes: 0,
    };
    // SAFETY: Every descriptor and output location remains live for the complete call.
    unsafe {
        assert_eq!(
            miso_engine_v1_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v1_compile_session(
                engine,
                session_document.as_ptr(),
                session_document.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_COMPILE_REJECTED
        );
        assert!(session.is_null());
        assert!(plan.is_null());
        assert!(diagnostics.required_bytes > 0);
        miso_engine_v1_engine_destroy(engine);
    }
}

#[test]
fn render_diagnostic_egress_reuses_eager_capi_storage_without_allocation() {
    // SAFETY: The returned handles are uniquely owned until the matching destroy calls below.
    let (session, plan) = unsafe { compile_c(SESSION, &limits()) };
    let mut pcm = [f32::NAN; 256];
    let output = PlanarOutput {
        struct_size: PLANAR_OUTPUT_SIZE,
        channels: 2,
        samples: pcm.as_mut_ptr(),
        sample_capacity: pcm.len() as u64,
        frames: 128,
        plane_stride_samples: 128,
        reserved: [0; 2],
    };
    let mut event_storage = [0xa5_u8; 4_096];
    let mut event = BytesOut {
        struct_size: BYTES_OUT_SIZE,
        reserved0: 0,
        data: event_storage.as_mut_ptr(),
        capacity_bytes: event_storage.len() as u64,
        required_bytes: 0,
    };
    let configuration = protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: protocol::DiagnosticSeverity::Info,
    };
    let mut configure = vec![0_u8; 4_096];
    let configure_len = ProtocolCodec::default()
        .encode_command_frame_into(
            &TypedCommandFrame {
                request_id: RequestId::new(1).expect("request ID"),
                expected_revision: ExpectedRevision::Exact(SessionRevision(42)),
                payload: CommandPayload::TelemetryConfigure(&configuration),
            },
            &mut configure,
        )
        .expect("telemetry configuration");
    configure.truncate(configure_len);
    let mut response = [0_u8; 4_096];
    // SAFETY: The session and all caller-owned command buffers remain live for the call.
    let configure_result = unsafe { submit(session, &configure, &mut response) };
    assert_eq!(configure_result, RESULT_OK);

    begin();
    // SAFETY: Both live handles and caller-owned buffers remain valid for each complete call.
    let (render_result, event_result) = unsafe {
        (
            miso_engine_v1_render_f32_planar(plan, 0, &output),
            miso_engine_v1_dequeue_event(session, EVENT_LANE_RELIABLE, &mut event),
        )
    };
    let observed = finish();
    assert_eq!(render_result, RESULT_OK);
    assert_eq!(event_result, RESULT_OK);
    assert!(
        event.required_bytes > 0,
        "render diagnostic crossed C egress"
    );
    assert_eq!(
        observed,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
        },
        "render observation and diagnostic egress use only eager retained storage"
    );

    // SAFETY: These are the exact live handles returned by `compile_c` and are destroyed once.
    unsafe {
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
}

#[test]
fn external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps() {
    let session_document = scratch_session();
    let prospective_document = session_document.replacen(
        "\"session_id\": \"parametric-eq-nine-track\"",
        "\"session_id\": \"double-live-cap\"",
        1,
    );
    // #338 re-pin: the canonical JSON fixture is exact; the session-ID replacement removes 9.
    assert_eq!(session_document.len(), 18_453, "current canonical fixture");
    assert_eq!(
        prospective_document.len(),
        18_444,
        "prospective canonical fixture"
    );
    let oracle = primitive_replacement_oracle(&session_document, &prospective_document);
    // Issue #181: `size_of::<GraphPreparedEffectBank>()` went 88 -> 96, the fixture binds one
    // bank, and this oracle is double-live -- so +16 here and +8 in the single-plan report. The
    // live oracle and the primitive model both move, which is the property this pair of pins
    // exists to check: a struct that grew is reported by both or by neither.
    // #210 phase 3: +2_688, the two input banks' growth over two live plans; the immutable
    // builtin control-delivery metadata adds the corresponding concrete-owner layout bytes.
    // See
    // `primitive_replacement_oracle` for the per-bank arithmetic.
    // #241: 510_720 - 2 x (4_096 queue + 8_192 ring) - 2 x 200 = 485_744.
    assert_eq!(oracle.graph, 502_164);
    assert_eq!(oracle.source_total, 22_108);
    assert_eq!(oracle.source_overhead, 5_724);
    assert_eq!(oracle.effect_state, 15_120);
    assert_eq!(oracle.effect_scratch, 432);
    // #210 phase 3: 2 x 9_963 (see `builtin_owners`).
    assert_eq!(oracle.builtin, 19_926);
    assert_eq!(oracle.capi, 204_375);
    // #241: 58_694 - (29 x 10 locator) + (40 x 10 content identity) = 58_804.
    assert_eq!(oracle.largest, 58_804);

    let rows = [
        ("graph", oracle.graph),
        ("source-total", oracle.source_total),
        ("source-overhead", oracle.source_overhead),
        ("effect-state", oracle.effect_state),
        ("effect-scratch", oracle.effect_scratch),
        ("builtin", oracle.builtin),
        ("capi", oracle.capi),
        ("largest", oracle.largest),
    ];
    for (row, required) in rows {
        let set_cap = |compile_limits: &mut CompileLimits, value: u64| match row {
            "graph" => compile_limits.maximum_graph_session_plus_plan_bytes = value,
            "source-total" => compile_limits.maximum_source_total_bytes = value,
            "source-overhead" => compile_limits.maximum_source_overhead_bytes = value,
            "effect-state" => compile_limits.maximum_effect_state_bytes = value,
            "effect-scratch" => compile_limits.maximum_effect_scratch_bytes = value,
            "builtin" => compile_limits.maximum_builtin_retained_bytes = value,
            "capi" => compile_limits.maximum_capi_retained_bytes = value,
            "largest" => compile_limits.maximum_named_allocation_bytes = value,
            _ => unreachable!(),
        };

        let mut exact_limits = limits();
        set_cap(&mut exact_limits, required);
        // SAFETY: These handles are uniquely owned until their matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(&session_document, &exact_limits);
            assert_eq!(resources_c(plan), frozen_scratch_report(160_933));
            let request = command(1, 42, "double-live-cap");
            let mut response = [0xa5_u8; 4_096];
            assert_eq!(submit(session, &request, &mut response), RESULT_OK, "{row}");
            let mut pcm = [f32::NAN; 256];
            let output = PlanarOutput {
                struct_size: PLANAR_OUTPUT_SIZE,
                channels: 2,
                samples: pcm.as_mut_ptr(),
                sample_capacity: pcm.len() as u64,
                frames: 128,
                plane_stride_samples: 128,
                reserved: [0; 2],
            };
            assert_eq!(
                miso_engine_v1_render_f32_planar(plan, 0, &output),
                RESULT_OK
            );
            // The prospective session ID is nine bytes shorter than the current one.
            assert_eq!(resources_c(plan), frozen_scratch_report(160_933 - 9));
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }

        let mut below_limits = limits();
        set_cap(&mut below_limits, required - 1);
        if row == "largest" {
            // The same named compiled-model owner is already live during initial construction,
            // so one-below is atomically rejected before either child handle can be published.
            // SAFETY: The helper owns every handle through rejection and destroys the engine.
            unsafe { compile_rejected_c(&session_document, &below_limits) };
            continue;
        }
        // SAFETY: These handles are uniquely owned until their matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(&session_document, &below_limits);
            let before = resources_c(plan);
            let request = command(1, 42, "double-live-cap");
            let mut response = [0xa5_u8; 4_096];
            assert_eq!(
                submit(session, &request, &mut response),
                RESULT_COMPILE_REJECTED,
                "{row} one-below"
            );
            assert!(response.iter().all(|byte| *byte == 0xa5), "{row} canary");
            assert_eq!(resources_c(plan), before, "{row} atomic report");
            miso_engine_v1_session_destroy(session);
            miso_engine_v1_plan_destroy(plan);
        }
    }
}

#[test]
fn tiny_control_frame_still_accounts_three_provider_counters_exactly() {
    let mut roomy = limits();
    roomy.maximum_control_frame_bytes = 1;
    // SAFETY: Each returned child is uniquely owned and destroyed exactly once below.
    let required = unsafe {
        let (session, plan) = compile_c(SESSION, &roomy);
        let required = resources_c(plan).capi_retained_bytes;
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
        required
    };
    assert_eq!(required, 178_466, "tiny-frame retained authority");
    let mut exact = roomy;
    exact.maximum_capi_retained_bytes = required;
    // SAFETY: Exact admission returns two uniquely owned children.
    unsafe {
        let (session, plan) = compile_c(SESSION, &exact);
        assert_eq!(resources_c(plan).capi_retained_bytes, required);
        miso_engine_v1_session_destroy(session);
        miso_engine_v1_plan_destroy(plan);
    }
    let mut below = roomy;
    below.maximum_capi_retained_bytes = required - 1;
    // SAFETY: The helper verifies atomic rejection without published children.
    unsafe { compile_rejected_c(SESSION, &below) };
}
