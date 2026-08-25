//! Terminal scalar/bank migration execution and ownership coverage.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell};
use std::alloc::{GlobalAlloc, System};
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use miso_engine_effect_compiler::*;
use miso_engine_effect_contract::*;
use miso_engine_effect_package::*;
use miso_engine_lane::Backend;
use sha2::{Digest, Sha256};

struct QualificationAllocator;

#[global_allocator]
static QUALIFICATION_ALLOCATOR: QualificationAllocator = QualificationAllocator;

thread_local! {
    static ALLOCATION_ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static ALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static LIVE_BYTES: Cell<i64> = const { Cell::new(0) };
    static PEAK_LIVE_BYTES: Cell<i64> = const { Cell::new(0) };
}

fn track_allocation(size: usize) {
    ALLOCATION_ACTIVE.with(|active| {
        if active.get() {
            let live = LIVE_BYTES.get() + size as i64;
            ALLOCATIONS.set(ALLOCATIONS.get() + 1);
            ALLOCATED_BYTES.set(ALLOCATED_BYTES.get() + size as u64);
            LIVE_BYTES.set(live);
            PEAK_LIVE_BYTES.set(PEAK_LIVE_BYTES.get().max(live));
        }
    });
}

fn track_deallocation(size: usize) {
    ALLOCATION_ACTIVE.with(|active| {
        if active.get() {
            DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
            DEALLOCATED_BYTES.set(DEALLOCATED_BYTES.get() + size as u64);
            LIVE_BYTES.set(LIVE_BYTES.get() - size as i64);
        }
    });
}

// SAFETY: every operation delegates the original pointer/layout unchanged to `System`; the
// thread-local counters are observational and enabled only around isolated test-thread calls.
unsafe impl GlobalAlloc for QualificationAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            track_allocation(layout.size());
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            track_allocation(layout.size());
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        track_deallocation(layout.size());
        // SAFETY: delegates the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegates the original pointer/layout and requested size unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            track_deallocation(layout.size());
            track_allocation(new_size);
        }
        replacement
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AllocationSnapshot {
    allocations: u64,
    deallocations: u64,
    allocated_bytes: u64,
    deallocated_bytes: u64,
    peak_live_bytes: i64,
    live_bytes: i64,
}

fn measure_allocations<T>(operation: impl FnOnce() -> T) -> (T, AllocationSnapshot) {
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ALLOCATED_BYTES.set(0);
    DEALLOCATED_BYTES.set(0);
    LIVE_BYTES.set(0);
    PEAK_LIVE_BYTES.set(0);
    ALLOCATION_ACTIVE.set(true);
    let value = operation();
    ALLOCATION_ACTIVE.set(false);
    (
        value,
        AllocationSnapshot {
            allocations: ALLOCATIONS.get(),
            deallocations: DEALLOCATIONS.get(),
            allocated_bytes: ALLOCATED_BYTES.get(),
            deallocated_bytes: DEALLOCATED_BYTES.get(),
            peak_live_bytes: PEAK_LIVE_BYTES.get(),
            live_bytes: LIVE_BYTES.get(),
        },
    )
}

fn without_allocation_tracking<T>(operation: impl FnOnce() -> T) -> T {
    let was_active = ALLOCATION_ACTIVE.replace(false);
    let value = operation();
    ALLOCATION_ACTIVE.set(was_active);
    value
}

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("id"),
    }
}
const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("port"),
    }
}

static PARAMETERS: [ParameterDescriptorV1; 1] = [ParameterDescriptorV1 {
    id: ParameterId(1),
    display_name: "value",
    display_unit: "linear",
    unit: ParameterUnit::Linear,
    domain: ParameterDomain::Continuous,
    minimum: Some(-1.0),
    maximum: Some(1.0),
    default_value: 0.0,
    mapping: ParameterMapping::Linear,
    automation_rate: AutomationRate::Block,
    channel_policy: ParameterChannelPolicy::PerLane,
    smoothing: SmoothingRule::None,
    smoothing_samples: 0,
    readable: true,
    automatable: true,
    enum_choices: &[],
    nudge: None,
}];
static PORTS: [PortDescriptorV1; 2] = [
    PortDescriptorV1 {
        id: port_id("main-in"),
        role: PortRole::MainInput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
    PortDescriptorV1 {
        id: port_id("main-out"),
        role: PortRole::MainOutput,
        required: true,
        layout: PortLayout::DualMonoPlanar,
    },
];
const fn sizes(layout: u32) -> StatePayloadSizes {
    StatePayloadSizes {
        common_bytes: layout,
        left_bytes: layout + 1,
        right_bytes: layout + 1,
    }
}
const fn quality(rate: u32, layout: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate: rate,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
        maximum_state: sizes(layout),
        scratch_fixed_bytes: 2,
        scratch_bytes_per_frame: 0,
    }
}
static Q1: [QualityDescriptorV1; 4] = [
    quality(44_100, 1),
    quality(48_000, 1),
    quality(88_200, 1),
    quality(96_000, 1),
];
static Q2: [QualityDescriptorV1; 4] = [
    quality(44_100, 2),
    quality(48_000, 2),
    quality(88_200, 2),
    quality(96_000, 2),
];
static Q3: [QualityDescriptorV1; 4] = [
    quality(44_100, 3),
    quality(48_000, 3),
    quality(88_200, 3),
    quality(96_000, 3),
];
static D1: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.migration-terminal"),
    display_name: "Migration terminal",
    contract_major: 1,
    contract_minor: 0,
    state_layout_version: 1,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &PARAMETERS,
    ports: &PORTS,
    qualities: &Q1,
    observations: &[],
};
static D2: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 2,
    qualities: &Q2,
    ..D1
};
static D3: EffectDescriptorV1 = EffectDescriptorV1 {
    state_layout_version: 3,
    qualities: &Q3,
    ..D1
};
static D3_CLONE: EffectDescriptorV1 = EffectDescriptorV1 { ..D3 };
static INITIAL: [InitialParameterValue; 2] = [
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Left,
        value: -0.25,
    },
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Right,
        value: 0.75,
    },
];

#[derive(Default)]
struct Calls {
    prepare: AtomicUsize,
    bind: AtomicUsize,
    restore: AtomicUsize,
    bank_restore: AtomicUsize,
    bank_drop: AtomicUsize,
    wrong_key: AtomicBool,
    fail_prepare: AtomicBool,
    fail_restore: AtomicBool,
    fail_bank_restore: AtomicBool,
    wrong_metadata: AtomicBool,
    descriptor_drift: AtomicBool,
    wrong_width: AtomicBool,
}

struct Factory {
    descriptor: &'static EffectDescriptorV1,
    calls: Arc<Calls>,
}
struct Scalar {
    metadata: PreparedEffectMetadata,
    calls: Arc<Calls>,
    payload: Vec<u8>,
}
impl Drop for Scalar {
    fn drop(&mut self) {
        without_allocation_tracking(|| drop(core::mem::take(&mut self.payload)));
    }
}
struct Bank {
    metadata: PreparedBankMetadata,
    calls: Arc<Calls>,
    payloads: Vec<Vec<u8>>,
}
impl Drop for Bank {
    fn drop(&mut self) {
        self.calls.bank_drop.fetch_add(1, Ordering::SeqCst);
    }
}

fn initial_payload(layout: u32, seed: u8) -> Vec<u8> {
    (0..sizes(layout).total().unwrap() as u8)
        .map(|value| seed.wrapping_add(value))
        .collect()
}
fn copy_out(payload: &[u8], sizes: StatePayloadSizes, output: StatePayloadOutput<'_>) {
    let common = sizes.common_bytes as usize;
    let left = sizes.left_bytes as usize;
    output.common.copy_from_slice(&payload[..common]);
    output.left.copy_from_slice(&payload[common..common + left]);
    output.right.copy_from_slice(&payload[common + left..]);
}
fn copy_in(input: StatePayloadInput<'_>) -> Vec<u8> {
    [input.common, input.left, input.right].concat()
}

impl NativeEffectFactory for Factory {
    fn descriptor(&self) -> &'static EffectDescriptorV1 {
        if self.calls.descriptor_drift.load(Ordering::SeqCst) {
            &D2
        } else {
            self.descriptor
        }
    }
    fn prepare(
        &self,
        request: PrepareEffectRequest<'_>,
    ) -> Result<Box<dyn PreparedNativeEffect>, EffectPrepareError> {
        self.calls.prepare.fetch_add(1, Ordering::SeqCst);
        if self.calls.fail_prepare.load(Ordering::SeqCst) {
            return Err(EffectPrepareError { code: "prepare" });
        }
        let mut metadata =
            without_allocation_tracking(|| expected_prepared_metadata(self.descriptor, request))?;
        if self.calls.wrong_metadata.load(Ordering::SeqCst) {
            metadata.descriptor = &D3_CLONE;
        }
        Ok(Box::new(Scalar {
            metadata,
            calls: Arc::clone(&self.calls),
            payload: without_allocation_tracking(|| {
                initial_payload(self.descriptor.state_layout_version, 0x10)
            }),
        }))
    }
    fn bind_homogeneous_bank(
        &self,
        request: PrepareEffectBankRequest<'_>,
    ) -> Result<Option<Box<dyn PreparedNativeEffectBank>>, EffectPrepareError> {
        self.calls.bind.fetch_add(1, Ordering::SeqCst);
        let metadata = expected_prepared_metadata(self.descriptor, request.requests[0])?;
        Ok(Some(Box::new(Bank {
            metadata: PreparedBankMetadata {
                width: request.width,
                program_key: metadata.program_key(),
            },
            calls: Arc::clone(&self.calls),
            payloads: (0..request.width.lanes())
                .map(|index| {
                    initial_payload(
                        self.descriptor.state_layout_version,
                        0x20 + index as u8 * 0x10,
                    )
                })
                .collect(),
        })))
    }
}
impl PreparedNativeEffect for Scalar {
    fn metadata(&self) -> PreparedEffectMetadata {
        self.metadata
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process(&mut self, _: EffectProcessBlock<'_>) -> ProcessReport {
        for byte in &mut self.payload {
            *byte = byte.wrapping_add(1);
        }
        ProcessReport::default()
    }
    fn snapshot_state_payload(
        &self,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        copy_out(&self.payload, self.metadata.state_sizes, output);
        Ok(())
    }
    fn restore_state_payload(
        &mut self,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.calls.restore.fetch_add(1, Ordering::SeqCst);
        if self.calls.fail_restore.load(Ordering::SeqCst) {
            self.payload[0] = 0xff;
            return Err(StatePayloadError { code: "restore" });
        }
        if version != self.metadata.descriptor.state_layout_version {
            return Err(StatePayloadError { code: "version" });
        }
        without_allocation_tracking(|| {
            self.payload = copy_in(input);
        });
        Ok(())
    }
}
impl PreparedNativeEffectBank for Bank {
    fn metadata(&self) -> PreparedBankMetadata {
        let mut metadata = self.metadata.clone();
        if self.calls.wrong_key.load(Ordering::SeqCst) {
            metadata.program_key.quantum += 1;
        }
        if self.calls.wrong_width.load(Ordering::SeqCst) {
            metadata.width = BankWidth::Eight;
        }
        metadata
    }
    fn reset(&mut self, _: ResetKind) {}
    fn process_bank(&mut self, block: EffectBankProcessBlock<'_>) -> BankProcessReport {
        for payload in &mut self.payloads {
            for byte in payload {
                *byte = byte.wrapping_add(1);
            }
        }
        BankProcessReport::empty(block.width)
    }
    fn snapshot_track_state_payload(
        &self,
        index: u32,
        output: StatePayloadOutput<'_>,
    ) -> Result<(), StatePayloadError> {
        copy_out(
            &self.payloads[index as usize],
            self.metadata.program_key.state_sizes,
            output,
        );
        Ok(())
    }
    fn restore_track_state_payload(
        &mut self,
        index: u32,
        version: u32,
        input: StatePayloadInput<'_>,
    ) -> Result<(), StatePayloadError> {
        self.calls.bank_restore.fetch_add(1, Ordering::SeqCst);
        if self.calls.fail_bank_restore.load(Ordering::SeqCst) {
            self.payloads[index as usize][0] = 0xff;
            return Err(StatePayloadError {
                code: "bank-restore",
            });
        }
        if version != 3 {
            return Err(StatePayloadError { code: "version" });
        }
        without_allocation_tracking(|| {
            self.payloads[index as usize] = copy_in(input);
        });
        Ok(())
    }
}

#[derive(Default)]
struct StepControl {
    calls: AtomicUsize,
    reject: AtomicBool,
    partial: AtomicBool,
    report_fault: AtomicUsize,
}
struct GrowStep {
    control: Arc<StepControl>,
}
impl EffectStateMigrationStepV1 for GrowStep {
    fn scratch_bytes(&self) -> u64 {
        3
    }
    fn migrate(
        &self,
        source_layout: u32,
        target_layout: u32,
        source: StatePayloadInput<'_>,
        target: StatePayloadOutput<'_>,
        scratch: &mut [u8],
    ) -> Result<EffectStateMigrationStepReportV1, EffectStateMigrationStepFailureV1> {
        self.control.calls.fetch_add(1, Ordering::SeqCst);
        assert_eq!(target_layout, source_layout + 1);
        assert_eq!(scratch.len(), 3);
        scratch.fill(target_layout as u8);
        if self.control.partial.load(Ordering::SeqCst) {
            target.common[0] = 1;
        }
        if self.control.reject.load(Ordering::SeqCst) {
            return Err(EffectStateMigrationStepFailureV1::Rejected);
        }
        for (from, to) in [
            (source.common, target.common),
            (source.left, target.left),
            (source.right, target.right),
        ] {
            to[..from.len()].copy_from_slice(from);
            to[from.len()..].fill(0x80 | target_layout as u8);
        }
        let sizes = sizes(target_layout);
        let fault = self.control.report_fault.load(Ordering::SeqCst);
        Ok(EffectStateMigrationStepReportV1 {
            common_bytes: sizes.common_bytes - u32::from(fault == 1),
            left_bytes: sizes.left_bytes - u32::from(fault == 2),
            right_bytes: sizes.right_bytes - u32::from(fault == 3),
            reserved: u32::from(fault == 4),
        })
    }
}

fn wire(descriptor: &'static EffectDescriptorV1) -> &'static [u8] {
    let required = effect_descriptor_wire_v1_required_size(descriptor, 1 << 20).unwrap();
    let mut bytes = vec![0; required as usize];
    encode_effect_descriptor_wire_v1(descriptor, 1 << 20, &mut bytes).unwrap();
    Box::leak(bytes.into_boxed_slice())
}
fn bound(descriptor: &'static EffectDescriptorV1) -> BoundEffectDescriptorWireV1<'static> {
    bind_effect_descriptor_wire_v1(descriptor, wire(descriptor), 1 << 20).unwrap()
}
fn capability(
    descriptor: &'static EffectDescriptorV1,
    calls: &Arc<Calls>,
) -> WireBoundNativeEffectFactoryV1<'static> {
    bind_native_effect_factory_state_v1(
        Arc::new(Factory {
            descriptor,
            calls: Arc::clone(calls),
        }),
        wire(descriptor),
        1 << 20,
    )
    .unwrap()
}
fn shared_capabilities(
    descriptor: &'static EffectDescriptorV1,
    calls: &Arc<Calls>,
) -> (
    WireBoundNativeEffectFactoryV1<'static>,
    WireBoundNativeEffectFactoryV1<'static>,
) {
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(Factory {
        descriptor,
        calls: Arc::clone(calls),
    });
    (
        bind_native_effect_factory_state_v1(Arc::clone(&factory), wire(descriptor), 1 << 20)
            .unwrap(),
        bind_native_effect_factory_state_v1(factory, wire(descriptor), 1 << 20).unwrap(),
    )
}
fn shared_capabilities_three(
    descriptor: &'static EffectDescriptorV1,
    calls: &Arc<Calls>,
) -> [WireBoundNativeEffectFactoryV1<'static>; 3] {
    let factory: Arc<dyn NativeEffectFactory> = Arc::new(Factory {
        descriptor,
        calls: Arc::clone(calls),
    });
    [
        bind_native_effect_factory_state_v1(Arc::clone(&factory), wire(descriptor), 1 << 20)
            .unwrap(),
        bind_native_effect_factory_state_v1(Arc::clone(&factory), wire(descriptor), 1 << 20)
            .unwrap(),
        bind_native_effect_factory_state_v1(factory, wire(descriptor), 1 << 20).unwrap(),
    ]
}
fn replay() -> EffectBankPreparationV1 {
    EffectBankPreparationV1 {
        sample_rate: 48_000,
        quantum: 32,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: INITIAL.into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 64,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 8,
        },
    }
}
fn admission() -> EffectStateRestoreAdmissionV1 {
    EffectStateRestoreAdmissionV1 {
        sample_rate: 48_000,
        quantum: 32,
        maximum_total_state_bytes: 64,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 8,
    }
}
fn migration_admission() -> EffectStateMigrationAdmissionV1 {
    EffectStateMigrationAdmissionV1 {
        maximum_chain_steps: 2,
        maximum_intermediate_envelope_bytes: 1 << 20,
        maximum_migration_scratch_bytes: 1 << 20,
    }
}
fn envelope(descriptor: &'static EffectDescriptorV1, payload: &[u8]) -> Vec<u8> {
    let bound = bound(descriptor);
    let r = replay();
    let req = effect_state_v1_requirements(
        bound,
        r.state_replay(descriptor.id),
        EffectStateLimitsV1::default(),
    )
    .unwrap();
    let s = sizes(descriptor.state_layout_version);
    let c = s.common_bytes as usize;
    let l = s.left_bytes as usize;
    let mut out = vec![0; req.envelope_bytes as usize];
    encode_effect_state_v1(
        bound,
        r.state_replay(descriptor.id),
        &payload[..c],
        &payload[c..c + l],
        &payload[c + l..],
        EffectStateLimitsV1::default(),
        &mut out,
    )
    .unwrap();
    out
}
fn snapshot_scalar(
    cap: &WireBoundNativeEffectFactoryV1<'_>,
    replay: &EffectBankPreparationV1,
    processor: &dyn PreparedNativeEffect,
) -> Vec<u8> {
    let req =
        scalar_effect_state_v1_requirements(cap, replay, EffectStateLimitsV1::default()).unwrap();
    let mut scratch = vec![0; req.payload_snapshot_scratch_bytes as usize];
    let mut out = vec![0; req.envelope_bytes as usize];
    snapshot_scalar_effect_state_v1(
        cap,
        replay,
        processor,
        EffectStateLimitsV1::default(),
        &mut scratch,
        &mut out,
    )
    .unwrap();
    out
}
fn snapshot_bank(cap: &UnpublishedEffectBankStateV1<'_>, index: u32) -> Vec<u8> {
    let r = &cap.replays()[index as usize];
    let req =
        scalar_effect_state_v1_requirements(cap.bound_factory(), r, EffectStateLimitsV1::default())
            .unwrap();
    let mut scratch = vec![0; req.payload_snapshot_scratch_bytes as usize];
    let mut out = vec![0; req.envelope_bytes as usize];
    snapshot_unpublished_effect_bank_track_state_v1(
        cap,
        index,
        EffectStateLimitsV1::default(),
        &mut scratch,
        &mut out,
    )
    .unwrap();
    out
}
fn bank_cap(
    capability: WireBoundNativeEffectFactoryV1<'static>,
) -> UnpublishedEffectBankStateV1<'static> {
    bank_cap_with_replays(capability, (0..4).map(|_| replay()).collect::<Vec<_>>())
}
fn bank_cap_with_replays(
    capability: WireBoundNativeEffectFactoryV1<'static>,
    replays: Vec<EffectBankPreparationV1>,
) -> UnpublishedEffectBankStateV1<'static> {
    prepare_unpublished_effect_bank_state_v1(
        capability,
        Backend::Simd4,
        BankWidth::Four,
        replays.into_boxed_slice(),
        admission(),
    )
    .unwrap()
}

fn migrated_payload(mut payload: Vec<u8>, mut layout: u32, target: u32) -> Vec<u8> {
    while layout < target {
        let source_sizes = sizes(layout);
        let common = source_sizes.common_bytes as usize;
        let left = source_sizes.left_bytes as usize;
        let sections = [
            payload[..common].to_vec(),
            payload[common..common + left].to_vec(),
            payload[common + left..].to_vec(),
        ];
        layout += 1;
        payload.clear();
        for mut section in sections {
            payload.append(&mut section);
            payload.push(0x80 | layout as u8);
        }
    }
    payload
}

#[test]
fn two_step_scalar_and_bank_restore_publish_identical_current_state() {
    let controls = Arc::new(Calls::default());
    let s1 = Arc::new(StepControl::default());
    let s2 = Arc::new(StepControl::default());
    let registry = StateMigrationRegistryV1::new(
        2,
        vec![
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound(&D1),
                bound(&D2),
                Arc::new(GrowStep {
                    control: Arc::clone(&s1),
                }),
            ),
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound(&D2),
                bound(&D3),
                Arc::new(GrowStep {
                    control: Arc::clone(&s2),
                }),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let old_scalar_cap = capability(&D1, &controls);
    let old_scalar = old_scalar_cap
        .factory()
        .prepare(replay().request())
        .unwrap();
    let source = snapshot_scalar(&old_scalar_cap, &replay(), old_scalar.as_ref());
    let expected = envelope(&D3, &migrated_payload(initial_payload(1, 0x10), 1, 3));
    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize - 1];
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize - 1];
    let error = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateMigrationDiagnosticCodeV1::BufferTooSmall, 3)
    );

    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0xcc; req.first_envelope_bytes as usize + 7];
    let mut second = vec![0xdd; req.second_envelope_bytes as usize + 7];
    let mut scratch = vec![0xee; req.migration_scratch_bytes as usize + 7];
    let initial_sentinel = InitialParameterValue {
        parameter_index: 99,
        channel: ParameterChannel::Both,
        value: 0.125,
    };
    let mut initial = vec![initial_sentinel; req.scalar_initial_value_scratch_slots as usize + 1];
    let restored = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap();
    let suffix = initial.last().unwrap();
    assert_eq!(
        (
            suffix.parameter_index,
            suffix.channel,
            suffix.value.to_bits()
        ),
        (
            initial_sentinel.parameter_index,
            initial_sentinel.channel,
            initial_sentinel.value.to_bits()
        )
    );
    let scalar_state = snapshot_scalar(
        restored.bound_factory(),
        restored.replay(),
        restored.processor(),
    );
    assert_eq!(scalar_state, expected);
    assert!(
        first[req.first_envelope_bytes as usize..]
            .iter()
            .all(|b| *b == 0xcc)
    );
    assert!(
        second[req.second_envelope_bytes as usize..]
            .iter()
            .all(|b| *b == 0xdd)
    );
    assert!(
        scratch[req.migration_scratch_bytes as usize..]
            .iter()
            .all(|b| *b == 0xee)
    );
    let (current, bank_factory) = shared_capabilities(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let bank = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        2,
        &mut first,
        &mut second,
        &mut scratch,
    )
    .unwrap();
    assert_eq!(snapshot_bank(&bank, 2), scalar_state);
    assert_ne!(snapshot_bank(&bank, 1), scalar_state);

    let old_bank = prepare_unpublished_effect_bank_state_v1(
        capability(&D1, &controls),
        Backend::Simd4,
        BankWidth::Four,
        (0..4)
            .map(|_| replay())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        admission(),
    )
    .unwrap();
    let bank_source = snapshot_bank(&old_bank, 2);
    let expected_bank = envelope(&D3, &migrated_payload(initial_payload(1, 0x40), 1, 3));
    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &bank_source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
    let restored = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap();
    assert_eq!(
        snapshot_scalar(
            restored.bound_factory(),
            restored.replay(),
            restored.processor()
        ),
        expected_bank
    );
    assert_eq!(s1.calls.load(Ordering::SeqCst), 3);
    assert_eq!(s2.calls.load(Ordering::SeqCst), 3);
}

#[test]
fn workspace_and_step_failures_precede_hooks_and_drop_banks() {
    let controls = Arc::new(Calls::default());
    let step = Arc::new(StepControl::default());
    let registry = StateMigrationRegistryV1::new(
        1,
        vec![EffectStateMigrationRegistrationV1::from_bound_descriptors(
            bound(&D2),
            bound(&D3),
            Arc::new(GrowStep {
                control: Arc::clone(&step),
            }),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let source = envelope(&D2, &initial_payload(2, 0x42));
    for detail in 1..=3 {
        let (current, bank_factory) = shared_capabilities(&D3, &controls);
        let bank = bank_cap(bank_factory);
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0xa1; req.first_envelope_bytes as usize];
        let mut second = vec![0xa2; req.second_envelope_bytes as usize];
        let mut scratch = vec![0xa3; req.migration_scratch_bytes as usize];
        match detail {
            1 => {
                first.pop();
            }
            2 => {}
            3 => {
                scratch.pop();
            }
            _ => {}
        }
        if detail == 2 {
            continue;
        }
        let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
            resolved,
            bank,
            0,
            &mut first,
            &mut second,
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.detail),
            (EffectStateMigrationDiagnosticCodeV1::BufferTooSmall, detail)
        );
        assert_eq!(step.calls.load(Ordering::SeqCst), 0);
    }
    for partial in [false, true] {
        step.partial.store(partial, Ordering::SeqCst);
        step.reject.store(true, Ordering::SeqCst);
        let current = capability(&D3, &controls);
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0; req.first_envelope_bytes as usize];
        let mut scratch = vec![0; req.migration_scratch_bytes as usize];
        let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
        let prepares = controls.prepare.load(Ordering::SeqCst);
        let error = restore_scalar_effect_state_with_migration_v1(
            resolved,
            current,
            &mut first,
            &mut [],
            &mut scratch,
            &mut initial,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.detail),
            (
                EffectStateMigrationDiagnosticCodeV1::Step,
                if partial { 2 } else { 1 }
            )
        );
        assert_eq!(controls.prepare.load(Ordering::SeqCst), prepares);
    }

    step.partial.store(true, Ordering::SeqCst);
    step.reject.store(true, Ordering::SeqCst);
    let (current, bank_factory) = shared_capabilities(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let drops = controls.bank_drop.load(Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        0,
        &mut first,
        &mut second,
        &mut scratch,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateMigrationDiagnosticCodeV1::Step, 2)
    );
    assert!(first.iter().all(|byte| *byte == 0));
    assert_eq!(controls.bank_restore.load(Ordering::SeqCst), 0);
    assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);

    for (reject, report_fault, detail) in [
        (true, 0_usize, 1_u32),
        (false, 1, 3),
        (false, 2, 3),
        (false, 3, 3),
        (false, 4, 3),
    ] {
        step.partial.store(false, Ordering::SeqCst);
        step.reject.store(reject, Ordering::SeqCst);
        step.report_fault.store(report_fault, Ordering::SeqCst);
        let (current, bank_factory) = shared_capabilities(&D3, &controls);
        let bank = bank_cap(bank_factory);
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0; req.first_envelope_bytes as usize];
        let mut scratch = vec![0; req.migration_scratch_bytes as usize];
        let drops = controls.bank_drop.load(Ordering::SeqCst);
        let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
            resolved,
            bank,
            0,
            &mut first,
            &mut [],
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.detail),
            (EffectStateMigrationDiagnosticCodeV1::Step, detail)
        );
        assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);
    }
}

#[test]
fn every_workspace_and_bank_destination_failure_is_pre_hook_and_exact() {
    let live_controls = Arc::new(Calls::default());
    let live_cap = capability(&D3, &live_controls);
    let live_processor = live_cap.factory().prepare(replay().request()).unwrap();
    let live_before = snapshot_scalar(&live_cap, &replay(), live_processor.as_ref());
    let live_bank = bank_cap(capability(&D3, &live_controls));
    let live_bank_before = snapshot_bank(&live_bank, 2);
    let controls = Arc::new(Calls::default());
    let one = Arc::new(StepControl::default());
    let two = Arc::new(StepControl::default());
    let registry = StateMigrationRegistryV1::new(
        2,
        vec![
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound(&D1),
                bound(&D2),
                Arc::new(GrowStep {
                    control: Arc::clone(&one),
                }),
            ),
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                bound(&D2),
                bound(&D3),
                Arc::new(GrowStep {
                    control: Arc::clone(&two),
                }),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let source = envelope(&D1, &initial_payload(1, 0x51));

    for detail in [1_u32, 2, 3] {
        let (current, bank_factory) = shared_capabilities(&D3, &controls);
        let bank = bank_cap(bank_factory);
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0; req.first_envelope_bytes as usize];
        let mut second = vec![0; req.second_envelope_bytes as usize];
        let mut scratch = vec![0; req.migration_scratch_bytes as usize];
        match detail {
            1 => {
                first.pop();
                second.pop();
                scratch.pop();
            }
            2 => {
                second.pop();
                scratch.pop();
            }
            3 => {
                scratch.pop();
            }
            _ => unreachable!(),
        }
        let drops = controls.bank_drop.load(Ordering::SeqCst);
        let live_sentinel = [0x5a_u8; 8];
        let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
            resolved,
            bank,
            0,
            &mut first,
            &mut second,
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (error.code, error.detail),
            (EffectStateMigrationDiagnosticCodeV1::BufferTooSmall, detail)
        );
        assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);
        assert_eq!(live_sentinel, [0x5a; 8]);
    }
    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize - 1];
    let error = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail, error.required_bytes),
        (
            EffectStateMigrationDiagnosticCodeV1::BufferTooSmall,
            4,
            req.scalar_initial_value_scratch_bytes
        )
    );
    assert_eq!(one.calls.load(Ordering::SeqCst), 0);
    assert_eq!(two.calls.load(Ordering::SeqCst), 0);

    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
    let error = restore_scalar_effect_state_with_migration_v1(
        resolved,
        capability(&D3, &controls),
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap_err();
    assert_eq!(
        (error.code, error.detail),
        (EffectStateMigrationDiagnosticCodeV1::Chain, 3)
    );
    assert_eq!(one.calls.load(Ordering::SeqCst), 0);

    let (current, execution) = shared_capabilities(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
    controls.descriptor_drift.store(true, Ordering::SeqCst);
    let error = restore_scalar_effect_state_with_migration_v1(
        resolved,
        execution,
        &mut first,
        &mut second,
        &mut scratch,
        &mut initial,
    )
    .unwrap_err();
    controls.descriptor_drift.store(false, Ordering::SeqCst);
    assert_eq!(
        (error.code, error.detail),
        (EffectStateMigrationDiagnosticCodeV1::Chain, 3)
    );
    assert_eq!(one.calls.load(Ordering::SeqCst), 0);

    let (current, bank_factory) = shared_capabilities(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut second = vec![0; req.second_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let drops = controls.bank_drop.load(Ordering::SeqCst);
    controls.descriptor_drift.store(true, Ordering::SeqCst);
    let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        0,
        &mut first,
        &mut second,
        &mut scratch,
    )
    .unwrap_err();
    controls.descriptor_drift.store(false, Ordering::SeqCst);
    assert_eq!(
        (error.code, error.detail, error.nested_state.detail),
        (EffectStateMigrationDiagnosticCodeV1::Restore, 2, 4)
    );
    assert_eq!(one.calls.load(Ordering::SeqCst), 0);
    assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);

    for expected_nested in [1_u32, 2, 3, 4] {
        controls.wrong_key.store(false, Ordering::SeqCst);
        controls.wrong_width.store(false, Ordering::SeqCst);
        let current = capability(&D3, &controls);
        let mut replays = (0..4).map(|_| replay()).collect::<Vec<_>>();
        if expected_nested <= 2 {
            replays[0].initial_values[0].value = 0.5;
        }
        let bank = bank_cap_with_replays(capability(&D3, &controls), replays);
        if expected_nested <= 3 {
            controls.wrong_key.store(true, Ordering::SeqCst);
        }
        if expected_nested <= 2 {
            controls.wrong_width.store(true, Ordering::SeqCst);
        }
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0; req.first_envelope_bytes as usize];
        let mut second = vec![0; req.second_envelope_bytes as usize];
        let mut scratch = vec![0; req.migration_scratch_bytes as usize];
        let index = if expected_nested == 1 { 99 } else { 0 };
        let drops = controls.bank_drop.load(Ordering::SeqCst);
        let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
            resolved,
            bank,
            index,
            &mut first,
            &mut second,
            &mut scratch,
        )
        .unwrap_err();
        assert_eq!(
            (
                error.code,
                error.detail,
                error.nested_state.code,
                error.nested_state.detail
            ),
            (
                EffectStateMigrationDiagnosticCodeV1::Restore,
                2,
                EffectStateDiagnosticCodeV1::Restore,
                expected_nested
            )
        );
        assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);
    }
    controls.wrong_key.store(false, Ordering::SeqCst);
    controls.wrong_width.store(false, Ordering::SeqCst);
    assert_eq!(one.calls.load(Ordering::SeqCst), 0);
    assert_eq!(two.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        snapshot_scalar(&live_cap, &replay(), live_processor.as_ref()),
        live_before
    );
    assert_eq!(snapshot_bank(&live_bank, 2), live_bank_before);
}

#[test]
fn zero_step_scalar_and_serial_one_step_bank_restores_are_exact_and_isolated() {
    let controls = Arc::new(Calls::default());
    let current = capability(&D3, &controls);
    let source = envelope(&D3, &initial_payload(3, 0x61));
    let empty = StateMigrationRegistryV1::new(0, Box::new([])).unwrap();
    let resolved = resolve_effect_state_migration_v1(
        &empty,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    assert_eq!(
        (
            req.chain_step_count,
            req.first_envelope_bytes,
            req.second_envelope_bytes,
            req.migration_scratch_bytes
        ),
        (0, 0, 0, 0)
    );
    let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
    let restored = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut [],
        &mut [],
        &mut [],
        &mut initial,
    )
    .unwrap();
    assert_eq!(
        snapshot_scalar(
            restored.bound_factory(),
            restored.replay(),
            restored.processor()
        ),
        source
    );
    let (current, bank_factory) = shared_capabilities(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let resolved = resolve_effect_state_migration_v1(
        &empty,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let mut first = [0xc1_u8; 3];
    let mut second = [0xc2_u8; 3];
    let mut scratch = [0xc3_u8; 3];
    let bank = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        0,
        &mut first,
        &mut second,
        &mut scratch,
    )
    .unwrap();
    assert_eq!(snapshot_bank(&bank, 0), source);
    assert_eq!((first, second, scratch), ([0xc1; 3], [0xc2; 3], [0xc3; 3]));

    let step = Arc::new(StepControl::default());
    let registry = StateMigrationRegistryV1::new(
        1,
        vec![EffectStateMigrationRegistrationV1::from_bound_descriptors(
            bound(&D2),
            bound(&D3),
            Arc::new(GrowStep {
                control: Arc::clone(&step),
            }),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let [resolve_a, resolve_b, bank_factory] = shared_capabilities_three(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let untouched = snapshot_bank(&bank, 1);
    let source_a = envelope(&D2, &initial_payload(2, 0x71));
    let source_b = envelope(&D2, &initial_payload(2, 0x91));
    let current = capability(&D3, &controls);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source_a,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req_scalar = resolved.requirements();
    let mut scalar_first = vec![0; req_scalar.first_envelope_bytes as usize];
    let mut scalar_scratch = vec![0; req_scalar.migration_scratch_bytes as usize];
    let mut scalar_initial =
        vec![INITIAL[0]; req_scalar.scalar_initial_value_scratch_slots as usize];
    let restored = restore_scalar_effect_state_with_migration_v1(
        resolved,
        current,
        &mut scalar_first,
        &mut [],
        &mut scalar_scratch,
        &mut scalar_initial,
    )
    .unwrap();
    assert_eq!(
        snapshot_scalar(
            restored.bound_factory(),
            restored.replay(),
            restored.processor()
        ),
        envelope(&D3, &migrated_payload(initial_payload(2, 0x71), 2, 3))
    );
    let resolved_a = resolve_effect_state_migration_v1(
        &registry,
        &resolve_a,
        &source_a,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved_a.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let bank = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved_a,
        bank,
        0,
        &mut first,
        &mut [],
        &mut scratch,
    )
    .unwrap();
    let resolved_b = resolve_effect_state_migration_v1(
        &registry,
        &resolve_b,
        &source_b,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let bank = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved_b,
        bank,
        2,
        &mut first,
        &mut [],
        &mut scratch,
    )
    .unwrap();
    assert_eq!(
        snapshot_bank(&bank, 0),
        envelope(&D3, &migrated_payload(initial_payload(2, 0x71), 2, 3))
    );
    assert_eq!(
        snapshot_bank(&bank, 2),
        envelope(&D3, &migrated_payload(initial_payload(2, 0x91), 2, 3))
    );
    assert_eq!(snapshot_bank(&bank, 1), untouched);
    assert_eq!(step.calls.load(Ordering::SeqCst), 3);
}

#[test]
fn terminal_factory_and_partial_restore_failures_publish_nothing() {
    let live_controls = Arc::new(Calls::default());
    let live_cap = capability(&D3, &live_controls);
    let live_processor = live_cap.factory().prepare(replay().request()).unwrap();
    let live_scalar_before = snapshot_scalar(&live_cap, &replay(), live_processor.as_ref());
    let live_bank = bank_cap(capability(&D3, &live_controls));
    let live_bank_before = snapshot_bank(&live_bank, 1);
    let controls = Arc::new(Calls::default());
    let step = Arc::new(StepControl::default());
    let registry = StateMigrationRegistryV1::new(
        1,
        vec![EffectStateMigrationRegistrationV1::from_bound_descriptors(
            bound(&D2),
            bound(&D3),
            Arc::new(GrowStep {
                control: Arc::clone(&step),
            }),
        )]
        .into_boxed_slice(),
    )
    .unwrap();
    let source = envelope(&D2, &initial_payload(2, 0xb1));
    for (prepare_failure, restore_failure, metadata_failure, expected_code, expected_detail) in [
        (true, false, false, EffectStateDiagnosticCodeV1::Factory, 3),
        (false, true, false, EffectStateDiagnosticCodeV1::Payload, 3),
        (false, false, true, EffectStateDiagnosticCodeV1::Factory, 4),
    ] {
        controls
            .fail_prepare
            .store(prepare_failure, Ordering::SeqCst);
        controls
            .fail_restore
            .store(restore_failure, Ordering::SeqCst);
        controls
            .wrong_metadata
            .store(metadata_failure, Ordering::SeqCst);
        let current = capability(&D3, &controls);
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            admission(),
        )
        .unwrap();
        let req = resolved.requirements();
        let mut first = vec![0; req.first_envelope_bytes as usize];
        let mut scratch = vec![0; req.migration_scratch_bytes as usize];
        let mut initial = vec![INITIAL[0]; req.scalar_initial_value_scratch_slots as usize];
        let error = restore_scalar_effect_state_with_migration_v1(
            resolved,
            current,
            &mut first,
            &mut [],
            &mut scratch,
            &mut initial,
        )
        .unwrap_err();
        assert_eq!(
            (
                error.code,
                error.detail,
                error.nested_state.code,
                error.nested_state.detail
            ),
            (
                EffectStateMigrationDiagnosticCodeV1::Restore,
                1,
                expected_code,
                expected_detail
            )
        );
    }
    controls.fail_prepare.store(false, Ordering::SeqCst);
    controls.fail_restore.store(false, Ordering::SeqCst);
    controls.wrong_metadata.store(false, Ordering::SeqCst);
    controls.fail_bank_restore.store(true, Ordering::SeqCst);
    let (current, bank_factory) = shared_capabilities(&D3, &controls);
    let bank = bank_cap(bank_factory);
    let drops = controls.bank_drop.load(Ordering::SeqCst);
    let resolved = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        admission(),
    )
    .unwrap();
    let req = resolved.requirements();
    let mut first = vec![0; req.first_envelope_bytes as usize];
    let mut scratch = vec![0; req.migration_scratch_bytes as usize];
    let error = restore_unpublished_effect_bank_track_state_with_migration_v1(
        resolved,
        bank,
        0,
        &mut first,
        &mut [],
        &mut scratch,
    )
    .unwrap_err();
    assert_eq!(
        (
            error.code,
            error.detail,
            error.nested_state.code,
            error.nested_state.detail
        ),
        (
            EffectStateMigrationDiagnosticCodeV1::Restore,
            2,
            EffectStateDiagnosticCodeV1::Payload,
            4
        )
    );
    assert_eq!(controls.bank_drop.load(Ordering::SeqCst), drops + 1);
    assert_eq!(
        snapshot_scalar(&live_cap, &replay(), live_processor.as_ref()),
        live_scalar_before
    );
    assert_eq!(snapshot_bank(&live_bank, 1), live_bank_before);
}

#[test]
fn intermediate_identity_and_replay_mutation_are_unreachable_by_construction() {
    let source = include_str!("../src/migration.rs");
    let executor = source
        .split("fn execute_migration_steps")
        .nth(1)
        .unwrap()
        .split("fn final_envelope")
        .next()
        .unwrap();
    assert!(executor.contains("StatePayloadInput"));
    assert!(executor.contains("StatePayloadOutput"));
    assert!(executor.contains("encode_effect_state_v1"));
    assert!(executor.contains("verify_effect_state_v1"));
    assert!(executor.contains("validate_effect_state_replay_configuration_v1"));
    assert!(!executor.contains("descriptor_identity"));
    assert!(!executor.contains("descriptor_wire"));
}

fn qualification_replay(
    sample_rate: u32,
    step_count: u32,
    historical_bank: bool,
) -> EffectBankPreparationV1 {
    EffectBankPreparationV1 {
        sample_rate,
        quantum: 128,
        quality: EffectQuality::Normal,
        bypass: historical_bank,
        link_mode: match step_count {
            0 => LinkMode::DualMono,
            1 => LinkMode::Maximum,
            2 => LinkMode::Average,
            _ => panic!("qualification step count"),
        },
        ports: PreparedPortsV1 {
            sidechain: PreparedSidechainPort::None,
        },
        initial_values: INITIAL.into(),
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: 64,
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 8,
        },
    }
}

fn qualification_admission(sample_rate: u32) -> EffectStateRestoreAdmissionV1 {
    EffectStateRestoreAdmissionV1 {
        sample_rate,
        quantum: 128,
        maximum_total_state_bytes: 64,
        maximum_scratch_bytes: 64,
        maximum_automation_spans_per_block: 8,
    }
}

fn qualification_bank_cap(
    capability: WireBoundNativeEffectFactoryV1<'static>,
    replay: &EffectBankPreparationV1,
) -> UnpublishedEffectBankStateV1<'static> {
    prepare_unpublished_effect_bank_state_v1(
        capability,
        Backend::Simd4,
        BankWidth::Four,
        (0..4)
            .map(|_| replay.clone())
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        qualification_admission(replay.sample_rate),
    )
    .unwrap()
}

fn qualification_envelope(
    descriptor: &'static EffectDescriptorV1,
    replay: &EffectBankPreparationV1,
    payload: &[u8],
) -> Vec<u8> {
    let descriptor_bound = bound(descriptor);
    let state_replay = replay.state_replay(descriptor.id);
    let requirements = effect_state_v1_requirements(
        descriptor_bound,
        state_replay,
        EffectStateLimitsV1::default(),
    )
    .unwrap();
    let payload_sizes = sizes(descriptor.state_layout_version);
    let common = payload_sizes.common_bytes as usize;
    let left = payload_sizes.left_bytes as usize;
    let mut output = vec![0xa5; requirements.envelope_bytes as usize];
    encode_effect_state_v1(
        descriptor_bound,
        state_replay,
        &payload[..common],
        &payload[common..common + left],
        &payload[common + left..],
        EffectStateLimitsV1::default(),
        &mut output,
    )
    .unwrap();
    output
}

fn run_qualification_rows(
    rates: &[u32],
    step_counts: &[u32],
    historical_owners: &[bool],
    bank_destinations: &[bool],
) -> (usize, [u8; 32]) {
    let mut rows = 0usize;
    let mut matrix_digest = Sha256::new();
    for &sample_rate in rates {
        for &step_count in step_counts {
            for &historical_bank in historical_owners {
                for &bank_destination in bank_destinations {
                    let calls = Arc::new(Calls::default());
                    let first_control = Arc::new(StepControl::default());
                    let second_control = Arc::new(StepControl::default());
                    let registry = StateMigrationRegistryV1::new(
                        2,
                        vec![
                            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                                bound(&D1),
                                bound(&D2),
                                Arc::new(GrowStep {
                                    control: Arc::clone(&first_control),
                                }),
                            ),
                            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                                bound(&D2),
                                bound(&D3),
                                Arc::new(GrowStep {
                                    control: Arc::clone(&second_control),
                                }),
                            ),
                        ]
                        .into_boxed_slice(),
                    )
                    .unwrap();
                    let source_descriptor = match step_count {
                        0 => &D3,
                        1 => &D2,
                        2 => &D1,
                        _ => unreachable!(),
                    };
                    let replay = qualification_replay(sample_rate, step_count, historical_bank);
                    let source = if historical_bank {
                        let old_bank =
                            qualification_bank_cap(capability(source_descriptor, &calls), &replay);
                        snapshot_bank(&old_bank, 2)
                    } else {
                        let old = capability(source_descriptor, &calls);
                        let processor = old.factory().prepare(replay.request()).unwrap();
                        snapshot_scalar(&old, &replay, processor.as_ref())
                    };
                    let source_sha256 = Sha256::digest(&source);
                    let seed = if historical_bank { 0x40 } else { 0x10 };
                    let expected_payload = migrated_payload(
                        initial_payload(source_descriptor.state_layout_version, seed),
                        source_descriptor.state_layout_version,
                        3,
                    );
                    let expected = qualification_envelope(&D3, &replay, &expected_payload);
                    let continued_payload: Vec<_> = expected_payload
                        .iter()
                        .map(|byte| byte.wrapping_add(1))
                        .collect();
                    let expected_next = qualification_envelope(&D3, &replay, &continued_payload);
                    let (resolution_capability, destination_capability) =
                        shared_capabilities(&D3, &calls);
                    let resolved = resolve_effect_state_migration_v1(
                        &registry,
                        &resolution_capability,
                        &source,
                        EffectStateLimitsV1::default(),
                        migration_admission(),
                        qualification_admission(sample_rate),
                    )
                    .unwrap();
                    assert_eq!(resolved.chain_step_count(), step_count as usize);
                    assert_eq!(
                        inspect_effect_state_selector_v1(&source, EffectStateLimitsV1::default())
                            .unwrap()
                            .descriptor_identity(),
                        bound(source_descriptor).identity()
                    );
                    let requirements = resolved.requirements();
                    let mut first = vec![0x31; requirements.first_envelope_bytes as usize + 5];
                    let mut second = vec![0x42; requirements.second_envelope_bytes as usize + 5];
                    let mut scratch = vec![0x53; requirements.migration_scratch_bytes as usize + 5];
                    let (final_state, next_state) = if bank_destination {
                        let destination = qualification_bank_cap(destination_capability, &replay);
                        let unrelated = [
                            snapshot_bank(&destination, 0),
                            snapshot_bank(&destination, 1),
                            snapshot_bank(&destination, 3),
                        ];
                        let mut destination =
                            restore_unpublished_effect_bank_track_state_with_migration_v1(
                                resolved,
                                destination,
                                2,
                                &mut first,
                                &mut second,
                                &mut scratch,
                            )
                            .unwrap();
                        assert_eq!(snapshot_bank(&destination, 0), unrelated[0]);
                        assert_eq!(snapshot_bank(&destination, 1), unrelated[1]);
                        assert_eq!(snapshot_bank(&destination, 3), unrelated[2]);
                        let restored = snapshot_bank(&destination, 2);
                        let mut left = [0.0; 4];
                        let mut right = [0.0; 4];
                        destination.bank_mut().process_bank(
                            EffectBankProcessBlock::new(
                                &mut left,
                                &mut right,
                                None,
                                1,
                                BankWidth::Four,
                                0,
                                &[],
                                &[0; 5],
                                128,
                            )
                            .unwrap(),
                        );
                        let next = snapshot_bank(&destination, 2);
                        (restored, next)
                    } else {
                        let sentinel = InitialParameterValue {
                            parameter_index: 99,
                            channel: ParameterChannel::Both,
                            value: 0.125,
                        };
                        let mut initial =
                            vec![
                                sentinel;
                                requirements.scalar_initial_value_scratch_slots as usize + 2
                            ];
                        let mut restored = restore_scalar_effect_state_with_migration_v1(
                            resolved,
                            destination_capability,
                            &mut first,
                            &mut second,
                            &mut scratch,
                            &mut initial,
                        )
                        .unwrap();
                        for value in
                            &initial[requirements.scalar_initial_value_scratch_slots as usize..]
                        {
                            assert_eq!(value.parameter_index, sentinel.parameter_index);
                            assert_eq!(value.channel, sentinel.channel);
                            assert_eq!(value.value.to_bits(), sentinel.value.to_bits());
                        }
                        let restored_state = snapshot_scalar(
                            restored.bound_factory(),
                            restored.replay(),
                            restored.processor(),
                        );
                        let mut left = [0.0];
                        let mut right = [0.0];
                        restored.processor_mut().process(
                            EffectProcessBlock::new(&mut left, &mut right, None, 0, &[], 128)
                                .unwrap(),
                        );
                        let next = snapshot_scalar(
                            restored.bound_factory(),
                            restored.replay(),
                            restored.processor(),
                        );
                        (restored_state, next)
                    };
                    assert_eq!(final_state, expected);
                    assert_eq!(next_state, expected_next);
                    assert_ne!(next_state, final_state);
                    assert_eq!(Sha256::digest(&source), source_sha256);
                    assert!(
                        first[requirements.first_envelope_bytes as usize..]
                            .iter()
                            .all(|byte| *byte == 0x31)
                    );
                    assert!(
                        second[requirements.second_envelope_bytes as usize..]
                            .iter()
                            .all(|byte| *byte == 0x42)
                    );
                    assert!(
                        scratch[requirements.migration_scratch_bytes as usize..]
                            .iter()
                            .all(|byte| *byte == 0x53)
                    );
                    assert_eq!(
                        first_control.calls.load(Ordering::SeqCst),
                        usize::from(step_count == 2)
                    );
                    assert_eq!(
                        second_control.calls.load(Ordering::SeqCst),
                        usize::from(step_count >= 1)
                    );
                    matrix_digest.update(sample_rate.to_le_bytes());
                    matrix_digest.update(step_count.to_le_bytes());
                    matrix_digest.update([historical_bank as u8, bank_destination as u8]);
                    matrix_digest.update((final_state.len() as u64).to_le_bytes());
                    matrix_digest.update(&final_state);
                    matrix_digest.update((next_state.len() as u64).to_le_bytes());
                    matrix_digest.update(&next_state);
                    rows += 1;
                }
            }
        }
    }
    (rows, matrix_digest.finalize().into())
}

#[test]
fn one_portable_migration_qualification_row_smoke() {
    let (rows, digest) = run_qualification_rows(&[48_000], &[2], &[true], &[false]);
    assert_eq!(rows, 1);
    assert_ne!(digest, [0; 32]);
}

#[test]
#[ignore = "Issue 081 exact 48-row migration qualification matrix"]
fn exact_portable_migration_qualification_matrix() {
    let (rows, digest) = run_qualification_rows(
        &[44_100, 48_000, 88_200, 96_000],
        &[0, 1, 2],
        &[false, true],
        &[false, true],
    );
    assert_eq!(rows, 48);
    let digest_hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    println!("migration_rows=48 output_sha256={digest_hex}");
}

#[test]
fn registry_resolution_and_bank_execution_allocation_boundaries_are_isolated() {
    let calls = Arc::new(Calls::default());
    let first_control = Arc::new(StepControl::default());
    let second_control = Arc::new(StepControl::default());
    let first_source = bound(&D1);
    let first_target = bound(&D2);
    let second_source = bound(&D2);
    let second_target = bound(&D3);
    let (_, registry_allocations) = measure_allocations(|| {
        let registry = StateMigrationRegistryV1::new(
            2,
            vec![
                EffectStateMigrationRegistrationV1::from_bound_descriptors(
                    first_source,
                    first_target,
                    Arc::new(GrowStep {
                        control: Arc::clone(&first_control),
                    }),
                ),
                EffectStateMigrationRegistrationV1::from_bound_descriptors(
                    second_source,
                    second_target,
                    Arc::new(GrowStep {
                        control: Arc::clone(&second_control),
                    }),
                ),
            ]
            .into_boxed_slice(),
        )
        .unwrap();
        assert_eq!(registry.maximum_entries(), 2);
        drop(registry);
    });
    assert!(registry_allocations.allocations > 0);
    assert_eq!(registry_allocations.live_bytes, 0);
    assert_eq!(
        registry_allocations.allocated_bytes,
        registry_allocations.deallocated_bytes
    );

    let registry = StateMigrationRegistryV1::new(
        2,
        vec![
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                first_source,
                first_target,
                Arc::new(GrowStep {
                    control: Arc::clone(&first_control),
                }),
            ),
            EffectStateMigrationRegistrationV1::from_bound_descriptors(
                second_source,
                second_target,
                Arc::new(GrowStep {
                    control: Arc::clone(&second_control),
                }),
            ),
        ]
        .into_boxed_slice(),
    )
    .unwrap();
    let replay = qualification_replay(48_000, 2, false);
    let old = capability(&D1, &calls);
    let processor = old.factory().prepare(replay.request()).unwrap();
    let source = snapshot_scalar(&old, &replay, processor.as_ref());
    let (current, destination_factory) = shared_capabilities(&D3, &calls);
    let destination = qualification_bank_cap(destination_factory, &replay);

    let (_, resolution_allocations) = measure_allocations(|| {
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            qualification_admission(48_000),
        )
        .unwrap();
        assert_eq!(resolved.chain_step_count(), 2);
        drop(resolved);
    });
    assert!(resolution_allocations.allocations > 0);
    assert_eq!(resolution_allocations.live_bytes, 0);
    assert_eq!(
        resolution_allocations.allocated_bytes,
        resolution_allocations.deallocated_bytes
    );

    let requirements = resolve_effect_state_migration_v1(
        &registry,
        &current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        qualification_admission(48_000),
    )
    .unwrap()
    .requirements();
    let mut first = vec![0xa5; requirements.first_envelope_bytes as usize];
    let mut second = vec![0xa5; requirements.second_envelope_bytes as usize];
    let mut scratch = vec![0xa5; requirements.migration_scratch_bytes as usize];
    let (restored, execution_allocations) = measure_allocations(|| {
        let resolved = resolve_effect_state_migration_v1(
            &registry,
            &current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            qualification_admission(48_000),
        )
        .unwrap();
        restore_unpublished_effect_bank_track_state_with_migration_v1(
            resolved,
            destination,
            1,
            &mut first,
            &mut second,
            &mut scratch,
        )
    });
    let restored = restored.unwrap();
    assert_eq!(execution_allocations, resolution_allocations);
    assert_eq!(
        snapshot_bank(&restored, 1),
        qualification_envelope(
            &D3,
            &replay,
            &migrated_payload(initial_payload(1, 0x10), 1, 3)
        )
    );

    let (scalar_current, scalar_destination) = shared_capabilities(&D3, &calls);
    let scalar_requirements = resolve_effect_state_migration_v1(
        &registry,
        &scalar_current,
        &source,
        EffectStateLimitsV1::default(),
        migration_admission(),
        qualification_admission(48_000),
    )
    .unwrap()
    .requirements();
    let mut scalar_first = vec![0xa5; scalar_requirements.first_envelope_bytes as usize];
    let mut scalar_second = vec![0xa5; scalar_requirements.second_envelope_bytes as usize];
    let mut scalar_scratch = vec![0xa5; scalar_requirements.migration_scratch_bytes as usize];
    let mut initial =
        vec![INITIAL[0]; scalar_requirements.scalar_initial_value_scratch_slots as usize];
    let (scalar_resolved, scalar_resolution_retained) = measure_allocations(|| {
        resolve_effect_state_migration_v1(
            &registry,
            &scalar_current,
            &source,
            EffectStateLimitsV1::default(),
            migration_admission(),
            qualification_admission(48_000),
        )
        .unwrap()
    });
    assert!(scalar_resolution_retained.live_bytes > 0);
    let (scalar, scalar_allocations) = measure_allocations(|| {
        restore_scalar_effect_state_with_migration_v1(
            scalar_resolved,
            scalar_destination,
            &mut scalar_first,
            &mut scalar_second,
            &mut scalar_scratch,
            &mut initial,
        )
    });
    let scalar = scalar.unwrap();
    let replay_initial_bytes = core::mem::size_of_val(INITIAL.as_slice());
    let destination_box_bytes = core::mem::size_of::<Scalar>();
    let retained_bytes = replay_initial_bytes + destination_box_bytes;
    let resolution_survivors =
        scalar_resolution_retained.allocations - scalar_resolution_retained.deallocations;
    let scalar_execution_delta = AllocationSnapshot {
        allocations: scalar_allocations.allocations,
        deallocations: scalar_allocations.deallocations - resolution_survivors,
        allocated_bytes: scalar_allocations.allocated_bytes,
        deallocated_bytes: scalar_allocations.deallocated_bytes
            - u64::try_from(scalar_resolution_retained.live_bytes).unwrap(),
        peak_live_bytes: scalar_allocations.peak_live_bytes,
        live_bytes: scalar_allocations.live_bytes + scalar_resolution_retained.live_bytes,
    };
    assert!(scalar_allocations.peak_live_bytes >= i64::try_from(retained_bytes).unwrap());
    assert_eq!(
        scalar_execution_delta,
        AllocationSnapshot {
            allocations: 2,
            deallocations: 0,
            allocated_bytes: u64::try_from(retained_bytes).unwrap(),
            deallocated_bytes: 0,
            peak_live_bytes: scalar_allocations.peak_live_bytes,
            live_bytes: i64::try_from(retained_bytes).unwrap(),
        },
        "scalar execution adds only its required replay slice and mock destination Box"
    );
    let (_, scalar_drop) = measure_allocations(|| drop(scalar));
    assert_eq!(
        scalar_drop,
        AllocationSnapshot {
            allocations: 0,
            deallocations: 2,
            allocated_bytes: 0,
            deallocated_bytes: u64::try_from(retained_bytes).unwrap(),
            peak_live_bytes: 0,
            live_bytes: -i64::try_from(retained_bytes).unwrap(),
        },
        "dropping the result returns every retained byte to the allocation baseline"
    );
    println!(
        "issue081_migration_allocation registry={registry_allocations:?} resolution={resolution_allocations:?} bank={execution_allocations:?} scalar_prebuilt={scalar_resolution_retained:?} scalar_delta={scalar_execution_delta:?} scalar_drop={scalar_drop:?}"
    );
}
