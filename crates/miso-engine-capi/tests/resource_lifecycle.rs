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

use miso_engine_capi::*;
use miso_engine_core::realtime::{PlanEpoch, PreparedRenderPlan, Producer, QueueGeneration};
use miso_engine_protocol::{
    AUTOMATION_BATCH_RECORDS, AutomationBatchSlot, AutomationRecord, CommandPayload,
    ControlCommandSlot, CounterId, CounterTelemetryRecord, CounterValue, ExpectedRevision,
    ProtocolCodec, ReliableSlot, RequestId, SessionEditV1, SessionRevision, StatusCode,
    TelemetryRecord, TypedCommandFrame,
};
use miso_engine_session::StableId;
use miso_engine_source::HostChunkProvider;

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

fn begin() {
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

const SESSION: &str = include_str!("../../../fixtures/session/v1/parametric-eq-nine-track.toml");

fn limits() -> CompileLimits {
    CompileLimits {
        struct_size: COMPILE_LIMITS_SIZE,
        source_ring_frames: 1_024,
        maximum_automation_spans_per_block: 128,
        reserved0: 0,
        maximum_toml_bytes: 1_000_000,
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
    let edit = SessionEditV1::SetSessionId {
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
        miso_engine_v2_submit_command(session, request.as_ptr(), request.len() as u64, &mut output)
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
        let create = miso_engine_v2_engine_create(&config, &mut engine);
        let compile = miso_engine_v2_compile_session(
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
        let render = miso_engine_v2_render_f32_planar(plan, 0, &output);
        let render_delta = snapshot().delta(before_render);
        let retry = submit(session, &second, &mut response);
        let second_render = miso_engine_v2_render_f32_planar(plan, 128, &output);
        if plan_first {
            miso_engine_v2_plan_destroy(plan);
            miso_engine_v2_session_destroy(session);
        } else {
            miso_engine_v2_session_destroy(session);
            miso_engine_v2_plan_destroy(plan);
        }
        miso_engine_v2_engine_destroy(engine);
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
        let create = miso_engine_v2_engine_create(&config, &mut engine);
        let compile = miso_engine_v2_compile_session(
            engine,
            SESSION.as_ptr(),
            SESSION.len() as u64,
            &constrained,
            &mut diagnostics,
            &mut session,
            &mut plan,
        );
        miso_engine_v2_engine_destroy(engine);
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
    let mut model = miso_engine_session::parse_session_toml(SESSION).expect("oracle fixture");
    for track in &mut model.tracks {
        let effect = &mut track.simd1.effects[0];
        effect.id = StableId::parse("soft-clip").expect("effect slot");
        effect.identity = miso_engine_session::EffectIdentity::Native {
            effect_id: StableId::parse("miso.soft-clip").expect("effect ID"),
        };
        effect.params = vec![
            miso_engine_session::EffectParam {
                parameter_id: 1,
                channel: miso_engine_session::ParameterChannel::Left,
                unit: miso_engine_session::ParameterUnit::Db,
                value: -6.0,
            },
            miso_engine_session::EffectParam {
                parameter_id: 1,
                channel: miso_engine_session::ParameterChannel::Right,
                unit: miso_engine_session::ParameterUnit::Db,
                value: -6.0,
            },
        ];
    }
    miso_engine_session::canonical_session_toml(&model).expect("oracle canonical fixture")
}

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
        graph_session_plus_plan_bytes: 205_723,
        graph_incremental_plan_bytes: 193_435,
        graph_metadata_bytes: 49_943,
        graph_delay_bytes: 0,
        effect_bank_scratch_bytes: 16_384,
        effect_bank_runtime_buffer_bytes: 8_192,
        effect_bank_metadata_bytes: 648,
        builtin_bank_bytes: 1_728,
        builtin_bank_scratch_bytes: 16_384,
        source_pcm_payload_bytes: 8_192,
        source_overhead_bytes: 3_366,
        source_total_bytes: 11_558,
        effect_scalar_state_bytes: 7_560,
        effect_scalar_scratch_bytes: 216,
        builtin_processor_payload_bytes: 7_974,
        builtin_meter_payload_bytes: 0,
        builtin_retained_payload_bytes: 7_974,
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

#[repr(C)]
struct RingMirror<T> {
    slots: Box<[UnsafeCell<MaybeUninit<T>>]>,
    slots_len: usize,
    logical_capacity: usize,
    generation: QueueGeneration,
    producer: AtomicUsize,
    consumer: AtomicUsize,
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
    Owned(miso_engine_protocol::Diagnostic),
}

#[allow(dead_code)]
struct RenderDiagnosticSlotMirror {
    diagnostic: miso_engine_protocol::Diagnostic,
    reservation: Option<miso_engine_protocol::ReliableEventReservation>,
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
struct CompiledControlQueueItemMirror {
    request_id: RequestId,
    revision: SessionRevision,
    command_sequence: u64,
    absolute_sample: u64,
    payload_offset: usize,
    payload_bytes: usize,
    admitted_bytes: u64,
    provider_sequence: u64,
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

#[allow(dead_code)]
struct ProviderEpochMirror {
    epoch: u64,
    source_ids: Box<[u8]>,
    sources: Box<[ControlSourceMirror]>,
}

#[allow(dead_code)]
struct TransferBlockMirror {
    generation: miso_engine_source::SourceGeneration,
    start_frame: miso_engine_source::SourceFrame,
    frames: u32,
    end_of_region: bool,
    native_decoder_sanitized_samples: u64,
    samples: Box<[f32]>,
}

#[allow(dead_code)]
struct NativeSourceWorkerMirror {
    join: Option<JoinHandle<miso_engine_source::NativeSourceWorkerExit>>,
    stopped: bool,
    stop: Producer<()>,
    not_sync: PhantomData<Cell<()>>,
}

#[allow(dead_code)]
struct GraphSourceEntryMirror {
    consumer: miso_engine_source::PcmSourceConsumer,
    channel_count: u32,
    planes: Box<[f32]>,
    retirement_worker: Option<NativeSourceWorkerMirror>,
}

#[allow(dead_code)]
struct SourceGraphSourceSetDriverMirror {
    sources: Box<[GraphSourceEntryMirror]>,
    mappings: Box<[miso_engine_source::SourceGraphTrackMapping]>,
    quantum_frames: u32,
}

/// `<f32 as miso_engine_lane::Lane>::Mask`: an all-zero or all-one word.
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
    lifetime_recovered: [u64; 2],
}

/// `InputStageKernel`: an enum over the two widths, sized by the larger.
#[allow(dead_code)]
enum InputStageKernelMirror {
    Simd4(InputStageFourMirror),
    Simd8(InputStageEightMirror),
}

#[allow(dead_code)]
struct BuiltinInputBankMirror {
    backend: miso_engine_core::KernelBackendV1,
    width: miso_engine_effect_contract::BankWidth,
    members: usize,
    stage: InputStageKernelMirror,
}

#[allow(dead_code)]
struct BuiltinBankProcessorMirror {
    bank: BuiltinInputBankMirror,
    process_calls: u64,
    tpt_kernel_calls: u64,
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
    SESSION
        .lines()
        .flat_map(|line| line.split([',', '{', '}']))
        .find_map(|field| {
            let (name, value) = field.split_once('=')?;
            (name.trim() == key).then(|| {
                value
                    .trim()
                    .parse::<usize>()
                    .expect("fixture numeric field")
            })
        })
        .unwrap_or_else(|| panic!("missing fixture numeric field {key}"))
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
            name: "automation track ID",
            bytes: 4,
        },
        PrimitiveOwner {
            name: "automation effect ID",
            bytes: 7,
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
            name: "current canonical TOML",
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
    assert_effective_owner_mutations(&active, 140_425, "active CAPI");

    let candidate_epoch_rows = [
        PrimitiveOwner {
            name: "candidate canonical TOML",
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
            bytes: bytes::<miso_engine_protocol::PreparedStructuralCommand>(1),
        },
        PrimitiveOwner {
            name: "candidate replay entries",
            bytes: bytes::<ReplayEntryMirror>(16),
        },
        PrimitiveOwner {
            name: "candidate replay bytes",
            bytes: 8_192,
        },
    ];
    let prepared = owner_total(&prepared_rows);
    assert_effective_owner_mutations(&candidate_epoch_rows, 10_429, "candidate CAPI epoch");
    assert_effective_owner_mutations(&prepared_rows, 13_960, "prepared protocol");
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
            bytes: bytes::<miso_engine_session::Source>(1),
        },
        PrimitiveOwner {
            name: "track declarations",
            bytes: bytes::<miso_engine_session::Track>(tracks as usize),
        },
        PrimitiveOwner {
            name: "output declarations",
            bytes: bytes::<miso_engine_session::Output>(1),
        },
        PrimitiveOwner {
            name: "route declarations",
            bytes: bytes::<miso_engine_session::Route>(9),
        },
        PrimitiveOwner {
            name: "effect declarations",
            bytes: bytes::<miso_engine_session::Effect>(effects as usize),
        },
        PrimitiveOwner {
            name: "effect parameter declarations",
            bytes: bytes::<miso_engine_session::EffectParam>(parameters as usize),
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
            bytes: "sha256:parametric-eq-nine-track".len() as u64,
        },
        PrimitiveOwner {
            name: "source locator",
            bytes: "host:parametric-eq-nine-track".len() as u64,
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
    let control_queue_items = fixture_usize("control_queue_messages");
    let source_ring_frames = fixture_usize("pcm_ring_frames") as u64;
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
    let builtin_bank_processor = bytes::<BuiltinBankProcessorMirror>(1);
    assert_eq!(
        builtin_bank_processor, 1_248,
        "primitive builtin bank processor"
    );
    vec![
        PrimitiveOwner {
            name: "control queue typed item storage",
            bytes: bytes::<CompiledControlQueueItemMirror>(control_queue_items),
        },
        PrimitiveOwner {
            name: "session source PCM runtime envelope",
            bytes: source_ring_frames * 2 * size_of::<f32>() as u64,
        },
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
            bytes: nodes * size_of::<miso_engine_graph::GraphNode>() as u64,
        },
        PrimitiveOwner {
            name: "graph edge array",
            bytes: edges * size_of::<miso_engine_graph::GraphEdge>() as u64,
        },
        PrimitiveOwner {
            name: "graph schedule array",
            bytes: schedule * size_of::<miso_engine_graph::GraphNodeId>() as u64,
        },
        PrimitiveOwner {
            name: "graph dependency levels",
            bytes: dependency_levels * size_of::<miso_engine_graph::DependencyLevel>() as u64,
        },
        PrimitiveOwner {
            name: "graph buffer assignments",
            bytes: buffers * size_of::<miso_engine_graph::BufferAssignment>() as u64,
        },
        PrimitiveOwner {
            name: "graph route timings",
            bytes: route_timings * size_of::<miso_engine_graph::RouteTiming>() as u64,
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
            bytes: bytes::<miso_engine_graph::GraphPreparedEffectBank>(1),
        },
        PrimitiveOwner {
            name: "effect-bank member IDs",
            bytes: bytes::<miso_engine_graph::EffectNodeId>(bank_lanes as usize),
        },
        PrimitiveOwner {
            name: "effect-bank member strings",
            bytes: bank_lanes * (3 + "soft-clip".len() as u64),
        },
        PrimitiveOwner {
            name: "effect-bank four-plane scratch",
            bytes: effect_bank_plane * 4,
        },
        PrimitiveOwner {
            name: "effect-bank two-plane runtime",
            bytes: effect_bank_plane * 2,
        },
        PrimitiveOwner {
            name: "builtin-bank descriptor array",
            bytes: bytes::<miso_engine_graph::GraphPreparedBuiltinBank>(1),
        },
        PrimitiveOwner {
            name: "builtin-bank member IDs",
            bytes: bytes::<miso_engine_graph::GraphNodeId>(bank_lanes as usize),
        },
        PrimitiveOwner {
            name: "builtin-bank active mask",
            bytes: bytes::<bool>(bank_lanes as usize),
        },
        PrimitiveOwner {
            name: "builtin-bank member strings",
            bytes: bank_lanes * 3,
        },
        PrimitiveOwner {
            name: "builtin-bank processor",
            bytes: builtin_bank_processor,
        },
        PrimitiveOwner {
            name: "builtin-bank four-plane scratch",
            bytes: effect_bank_plane * 4,
        },
    ]
}

fn source_owners() -> Vec<PrimitiveOwner> {
    let blocks = 1_024_usize / 128;
    let channels = 2_usize;
    let mappings = 9_usize;
    let data = spsc::<Box<TransferBlockMirror>>(blocks, "source data queue");
    let recycle = spsc::<Box<TransferBlockMirror>>(blocks, "source recycle queue");
    let command = spsc::<miso_engine_source::SourceCommand>(1, "source command queue");
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
            bytes: bytes::<miso_engine_source::SourceGraphTrackMapping>(mappings),
        },
        PrimitiveOwner {
            name: "graph source claims",
            bytes: bytes::<miso_engine_graph::GraphSourceInputClaim>(mappings),
        },
        PrimitiveOwner {
            name: "graph source driver",
            bytes: bytes::<SourceGraphSourceSetDriverMirror>(1),
        },
        PrimitiveOwner {
            name: "graph source coordinator planes",
            bytes: bytes::<f32>(channels * 128),
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
    vec![
        PrimitiveOwner {
            name: "builtin graph bindings",
            bytes: bytes::<miso_engine_graph::GraphNodeBinding>(processors),
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
            bytes: bytes::<(Box<str>, miso_engine_graph::TrackStage)>(processors),
        },
        PrimitiveOwner {
            name: "builtin cloned tail seal",
            bytes: bytes::<(Box<str>, BuiltinTailMirror)>(tracks),
        },
        PrimitiveOwner {
            name: "builtin stable-ID payload copies",
            bytes: (tracks * 10 * 3) as u64,
        },
        PrimitiveOwner {
            name: "builtin input processors",
            bytes: bytes::<InputBuiltinsMirror>(tracks),
        },
        PrimitiveOwner {
            name: "builtin fader processors",
            bytes: bytes::<FaderBuiltinsMirror>(tracks),
        },
        PrimitiveOwner {
            name: "builtin matrix processors",
            bytes: bytes::<MatrixBuiltinsMirror>(tracks),
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
        .take(14)
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
    assert_effective_owner_mutations(&graph, 444_690, "double-live graph/model");

    let source = source_owners();
    assert_eq!(owner_total(&source), 11_558, "primitive source total");
    let source_overhead_rows = source[1..].to_vec();
    assert_effective_owner_mutations(&source_overhead_rows, 3_366, "source overhead");
    let mut source_total_rows = source.clone();
    source_total_rows.extend(source.clone());
    assert_effective_owner_mutations(&source_total_rows, 23_116, "double-live source total");
    let mut double_source_overhead = source_overhead_rows.clone();
    double_source_overhead.extend(source_overhead_rows);
    assert_effective_owner_mutations(
        &double_source_overhead,
        6_732,
        "double-live source overhead",
    );

    // Soft-clip state layout 2 (issue #91): 104 effect-owned words per channel, plus the two
    // header words `miso-engine-effect-runtime`'s payload codec stamps into the common section.
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
    assert_effective_owner_mutations(&builtin, 7_974, "current builtin payload");
    let mut double_builtin = builtin.clone();
    double_builtin.extend(builtin);
    assert_effective_owner_mutations(&double_builtin, 15_948, "double-live builtin payload");

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
    assert_effective_owner_mutations(&capi_rows, 164_814, "double-live CAPI");

    let graph_rows = graph_owners();
    let graph_largest = owner_total(&graph_rows[7..15]);
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
    assert_eq!(largest, 58_694, "primitive maximum-single authority");
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
        Some(164_814),
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
    session_toml: &str,
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
            miso_engine_v2_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v2_compile_session(
                engine,
                session_toml.as_ptr(),
                session_toml.len() as u64,
                compile_limits,
                &mut diagnostics,
                &mut session,
                &mut plan,
            ),
            RESULT_OK,
            "{}",
            String::from_utf8_lossy(&diagnostic_storage[..diagnostics.required_bytes as usize])
        );
        miso_engine_v2_engine_destroy(engine);
    }
    (session, plan)
}

unsafe fn resources_c(plan: *const Plan) -> PlanResourceReport {
    // SAFETY: The caller supplies a live plan and this is a complete writable report.
    unsafe {
        let mut report: PlanResourceReport = core::mem::zeroed();
        report.struct_size = PLAN_RESOURCE_REPORT_SIZE;
        assert_eq!(miso_engine_v2_plan_resources(plan, &mut report), RESULT_OK);
        report
    }
}

unsafe fn compile_rejected_c(session_toml: &str, compile_limits: &CompileLimits) {
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
            miso_engine_v2_engine_create(&config, &mut engine),
            RESULT_OK
        );
        assert_eq!(
            miso_engine_v2_compile_session(
                engine,
                session_toml.as_ptr(),
                session_toml.len() as u64,
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
        miso_engine_v2_engine_destroy(engine);
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
    let configuration = miso_engine_protocol::TelemetryConfiguration {
        meter_handles: Vec::new(),
        meter_period_blocks: 0,
        counter_ids: Vec::new(),
        counter_period_blocks: 0,
        diagnostics_enabled: true,
        minimum_diagnostic_severity: miso_engine_protocol::DiagnosticSeverity::Info,
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
            miso_engine_v2_render_f32_planar(plan, 0, &output),
            miso_engine_v2_dequeue_event(session, EVENT_LANE_RELIABLE, &mut event),
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
        miso_engine_v2_session_destroy(session);
        miso_engine_v2_plan_destroy(plan);
    }
}

#[test]
fn external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps() {
    let session_toml = scratch_session();
    let prospective_toml = session_toml.replacen(
        "session_id = \"parametric-eq-nine-track\"",
        "session_id = \"double-live-cap\"",
        1,
    );
    assert_eq!(session_toml.len(), 10_200, "current canonical fixture");
    assert_eq!(
        prospective_toml.len(),
        10_191,
        "prospective canonical fixture"
    );
    let oracle = primitive_replacement_oracle(&session_toml, &prospective_toml);
    assert_eq!(oracle.graph, 444_690);
    assert_eq!(oracle.source_total, 23_116);
    assert_eq!(oracle.source_overhead, 6_732);
    assert_eq!(oracle.effect_state, 15_120);
    assert_eq!(oracle.effect_scratch, 432);
    assert_eq!(oracle.builtin, 15_948);
    assert_eq!(oracle.capi, 164_814);
    assert_eq!(oracle.largest, 58_694);

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
            let (session, plan) = compile_c(&session_toml, &exact_limits);
            assert_eq!(resources_c(plan), frozen_scratch_report(140_425));
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
                miso_engine_v2_render_f32_planar(plan, 0, &output),
                RESULT_OK
            );
            assert_eq!(resources_c(plan), frozen_scratch_report(140_416));
            miso_engine_v2_session_destroy(session);
            miso_engine_v2_plan_destroy(plan);
        }

        let mut below_limits = limits();
        set_cap(&mut below_limits, required - 1);
        if row == "largest" {
            // The same named compiled-model owner is already live during initial construction,
            // so one-below is atomically rejected before either child handle can be published.
            // SAFETY: The helper owns every handle through rejection and destroys the engine.
            unsafe { compile_rejected_c(&session_toml, &below_limits) };
            continue;
        }
        // SAFETY: These handles are uniquely owned until their matching destroy calls.
        unsafe {
            let (session, plan) = compile_c(&session_toml, &below_limits);
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
            miso_engine_v2_session_destroy(session);
            miso_engine_v2_plan_destroy(plan);
        }
    }
}
