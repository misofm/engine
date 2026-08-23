//! Exported-C retained-allocation and disposal ownership evidence.

#![allow(unsafe_code)]

use core::{
    alloc::Layout,
    cell::{Cell, UnsafeCell},
    mem::{MaybeUninit, size_of},
    ptr,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize},
};
use std::alloc::{GlobalAlloc, System};
use std::sync::Mutex;

use miso_engine_capi::*;
use miso_engine_core::realtime::{PlanEpoch, PreparedRenderPlan, QueueGeneration};
use miso_engine_protocol::{
    AUTOMATION_BATCH_RECORDS, AutomationBatchSlot, AutomationRecord, CommandPayload,
    ControlCommandSlot, CounterId, CounterTelemetryRecord, CounterValue, ExpectedRevision,
    ProtocolCodec, ReliableSlot, RequestId, RetainedDiagnosticSlot, SessionEditV1, SessionRevision,
    StatusCode, TelemetryRecord, TypedCommandFrame,
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
        graph_session_plus_plan_bytes: 210_331,
        graph_incremental_plan_bytes: 198_043,
        graph_metadata_bytes: 49_943,
        graph_delay_bytes: 0,
        effect_bank_scratch_bytes: 16_384,
        effect_bank_runtime_buffer_bytes: 8_192,
        effect_bank_metadata_bytes: 648,
        builtin_bank_bytes: 1_536,
        builtin_bank_scratch_bytes: 16_384,
        source_pcm_payload_bytes: 8_192,
        source_overhead_bytes: 3_366,
        source_total_bytes: 11_558,
        effect_scalar_state_bytes: 12_168,
        effect_scalar_scratch_bytes: 216,
        builtin_processor_payload_bytes: 6_678,
        builtin_meter_payload_bytes: 0,
        builtin_retained_payload_bytes: 6_678,
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

fn complete_capi_owners() -> (u64, u64, u64, u64) {
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
            name: "plan diagnostics",
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
            bytes: bytes::<RetainedDiagnosticSlot>(2),
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
            bytes: 10_200,
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
    let active_total = active.iter().map(|owner| owner.bytes).sum::<u64>();
    assert_eq!(active_total, 144_121, "complete active CAPI owner sum");
    for index in 0..active.len() {
        let omitted = active
            .iter()
            .enumerate()
            .filter_map(|(other, owner)| (other != index).then_some(owner.bytes))
            .sum::<u64>();
        assert_ne!(omitted, active_total, "omitted {}", active[index].name);
        let miscounted = active_total - active[index].bytes + active[index].bytes + 1;
        assert_ne!(miscounted, active_total, "miscount {}", active[index].name);
    }

    let candidate_epoch_rows = [
        PrimitiveOwner {
            name: "candidate canonical TOML",
            bytes: 10_191,
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
    let candidate_epoch = candidate_epoch_rows
        .iter()
        .map(|owner| owner.bytes)
        .sum::<u64>();
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
    let prepared = prepared_rows.iter().map(|owner| owner.bytes).sum::<u64>();
    for rows in [&candidate_epoch_rows[..], &prepared_rows[..]] {
        let total = rows.iter().map(|owner| owner.bytes).sum::<u64>();
        for index in 0..rows.len() {
            assert_ne!(
                rows.iter()
                    .enumerate()
                    .filter_map(|(other, owner)| (other != index).then_some(owner.bytes))
                    .sum::<u64>(),
                total,
                "omitted {}",
                rows[index].name
            );
        }
    }
    let largest = active
        .iter()
        .chain(candidate_epoch_rows.iter())
        .chain(prepared_rows.iter())
        .map(|owner| owner.bytes)
        .max()
        .expect("named owner");
    (active_total, candidate_epoch, prepared, largest)
}

fn owner_sum(rows: &[PrimitiveOwner], expected: u64) -> u64 {
    let total = rows.iter().map(|owner| owner.bytes).sum::<u64>();
    assert_eq!(total, expected);
    for index in 0..rows.len() {
        let omitted = rows
            .iter()
            .enumerate()
            .filter_map(|(other, owner)| (other != index).then_some(owner.bytes))
            .sum::<u64>();
        assert_ne!(omitted, total, "omitted {}", rows[index].name);
        let miscounted = total - rows[index].bytes + rows[index].bytes + 1;
        assert_ne!(miscounted, total, "miscounted {}", rows[index].name);
    }
    total
}

fn primitive_replacement_oracle() -> PrimitiveReplacementOracle {
    // These are independently pinned owner rows for this exact canonical fixture. They do not
    // invoke a compiled session, plan/resource report, queue/replay/exchange projection, or CAPI
    // admission helper. Both compiled models remain separate admission-only owners.
    let graph_owners = [
        PrimitiveOwner {
            name: "compiled graph session envelope",
            bytes: 12_288,
        },
        PrimitiveOwner {
            name: "plan audio buffers",
            bytes: 93_220,
        },
        PrimitiveOwner {
            name: "declared effect state",
            bytes: 12_168,
        },
        PrimitiveOwner {
            name: "declared effect scratch",
            bytes: 216,
        },
        PrimitiveOwner {
            name: "graph nodes/edges/schedule/levels/reductions metadata",
            bytes: 49_943,
        },
        PrimitiveOwner {
            name: "effect-bank scratch",
            bytes: 16_384,
        },
        PrimitiveOwner {
            name: "effect-bank runtime buffers",
            bytes: 8_192,
        },
        PrimitiveOwner {
            name: "builtin bank",
            bytes: 1_536,
        },
        PrimitiveOwner {
            name: "builtin-bank scratch",
            bytes: 16_384,
        },
    ];
    let current_graph = owner_sum(&graph_owners, 210_331);
    let prospective_graph = owner_sum(&graph_owners, 210_331);
    let current_model = owner_sum(
        &[PrimitiveOwner {
            name: "current compiled model",
            bytes: 16_631,
        }],
        16_631,
    );
    let prospective_model = owner_sum(
        &[PrimitiveOwner {
            name: "prospective compiled model",
            bytes: 16_613,
        }],
        16_613,
    );
    let source_owners = [
        PrimitiveOwner {
            name: "source PCM ring",
            bytes: 8_192,
        },
        PrimitiveOwner {
            name: "source ring/control overhead",
            bytes: 3_366,
        },
    ];
    let current_source_total = owner_sum(&source_owners, 11_558);
    let prospective_source_total = owner_sum(&source_owners, 11_558);
    let current_source_overhead = owner_sum(&source_owners[1..], 3_366);
    let prospective_source_overhead = owner_sum(&source_owners[1..], 3_366);
    let effect_state_owners = [PrimitiveOwner {
        name: "nine scalar effect states",
        bytes: 9 * 1_352,
    }];
    let effect_scratch_owners = [PrimitiveOwner {
        name: "nine scalar effect scratch regions",
        bytes: 9 * 24,
    }];
    let current_effect_state = owner_sum(&effect_state_owners, 12_168);
    let prospective_effect_state = owner_sum(&effect_state_owners, 12_168);
    let current_effect_scratch = owner_sum(&effect_scratch_owners, 216);
    let prospective_effect_scratch = owner_sum(&effect_scratch_owners, 216);
    let builtin_owners = [PrimitiveOwner {
        name: "nine builtin processor payloads",
        bytes: 9 * 742,
    }];
    let current_builtin = owner_sum(&builtin_owners, 6_678);
    let prospective_builtin = owner_sum(&builtin_owners, 6_678);

    // Candidate CAPI epoch is canonical bytes + one ControlSource + source-ID bytes:
    // 10_191 + 224 + 14 = 10_429. Prepared protocol is a 4 KiB response, a 776-byte
    // affine token, and the independently pinned 9_088-byte candidate replay arena.
    let (current_capi, candidate_epoch, prepared_protocol, capi_largest) = complete_capi_owners();
    assert_eq!(candidate_epoch, 10_429);
    assert_eq!(prepared_protocol, 13_960);

    let aggregate = |rows: &[u64]| rows.iter().copied().sum::<u64>();
    let graph = aggregate(&[
        current_graph,
        prospective_graph,
        current_model,
        prospective_model,
    ]);
    let source_total = aggregate(&[current_source_total, prospective_source_total]);
    let source_overhead = aggregate(&[current_source_overhead, prospective_source_overhead]);
    let effect_state = aggregate(&[current_effect_state, prospective_effect_state]);
    let effect_scratch = aggregate(&[current_effect_scratch, prospective_effect_scratch]);
    let builtin = aggregate(&[current_builtin, prospective_builtin]);
    let capi = aggregate(&[current_capi, candidate_epoch, prepared_protocol]);
    let largest = [49_167_u64, 49_167, capi_largest, 58_694, 58_694]
        .into_iter()
        .max()
        .expect("nonempty primitive max");

    // Effective mutations: every omitted owner and aggregate/max confusion changes authority.
    for (index, _) in [
        current_graph,
        prospective_graph,
        current_model,
        prospective_model,
    ]
    .iter()
    .enumerate()
    {
        let mut rows = [
            current_graph,
            prospective_graph,
            current_model,
            prospective_model,
        ];
        rows[index] = 0;
        assert_ne!(aggregate(&rows), graph, "graph/model owner {index}");
    }
    assert_ne!(
        [current_capi, candidate_epoch, prepared_protocol]
            .into_iter()
            .max()
            .expect("CAPI max"),
        capi,
        "aggregate cannot be replaced by max-single"
    );
    assert_ne!(
        [49_167_u64, 49_167, 32_768, 58_694, 58_694]
            .into_iter()
            .sum::<u64>(),
        largest,
        "max-single cannot be replaced by aggregate"
    );

    PrimitiveReplacementOracle {
        graph,
        source_total,
        source_overhead,
        effect_state,
        effect_scratch,
        builtin,
        capi,
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
fn external_primitive_double_live_oracle_drives_exact_and_one_below_c_caps() {
    let session_toml = scratch_session();
    let oracle = primitive_replacement_oracle();
    assert_eq!(oracle.graph, 453_906);
    assert_eq!(oracle.source_total, 23_116);
    assert_eq!(oracle.source_overhead, 6_732);
    assert_eq!(oracle.effect_state, 24_336);
    assert_eq!(oracle.effect_scratch, 432);
    assert_eq!(oracle.builtin, 13_356);
    assert_eq!(oracle.capi, 168_510);
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
            assert_eq!(resources_c(plan), frozen_scratch_report(144_121));
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
            assert_eq!(resources_c(plan), frozen_scratch_report(144_112));
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
