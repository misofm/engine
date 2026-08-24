//! Allocation boundary proof for package-native operations.

#![allow(unsafe_code)]

use core::{alloc::Layout, cell::Cell};
use std::alloc::{GlobalAlloc, System};

use miso_engine_effect_contract::*;
use miso_engine_effect_package::{
    ArtifactSelectionRequestV1, EffectArtifactAuthoringV1, EffectArtifactKindV1, EffectCid,
    EffectDescriptorEnumChoiceRecordV1, EffectDescriptorParameterRecordV1,
    EffectDescriptorPortRecordV1, EffectDescriptorQualityRecordV1, EffectDescriptorSummaryV1,
    EffectDescriptorWireDiagnosticCodeV1, EffectDescriptorWireDiagnosticV1,
    EffectPackageAuthoringV1, EffectPackageDiagnosticCodeV1, EffectPackageLimitsV1,
    EffectStateLimitsV1, EffectStateReplayViewV1, bind_effect_descriptor_wire_v1,
    effect_descriptor_identity_v1, effect_package_cid_v1, effect_package_v1_required_size,
    effect_state_v1_requirements, encode_effect_package_v1, encode_effect_state_v1,
    inspect_effect_state_selector_v1, miso_engine_effect_descriptor_v1_inspect,
    select_effect_package_artifact_v1, validate_effect_state_current_layout_v1,
    validate_effect_state_replay_v1, verify_effect_descriptor_wire_v1, verify_effect_package_v1,
    verify_effect_state_v1,
};

const fn effect_id(value: &'static str) -> EffectId {
    match EffectId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("state effect id"),
    }
}

const fn port_id(value: &'static str) -> PortId {
    match PortId::new(value) {
        Ok(value) => value,
        Err(_) => panic!("state port id"),
    }
}

static STATE_PARAMETERS: [ParameterDescriptorV1; 2] = [
    ParameterDescriptorV1 {
        id: ParameterId(1),
        display_name: "Shared",
        display_unit: "linear",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(-1.0),
        maximum: Some(1.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::Shared,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
    ParameterDescriptorV1 {
        id: ParameterId(2),
        display_name: "Per lane",
        display_unit: "linear",
        unit: ParameterUnit::Linear,
        domain: ParameterDomain::Continuous,
        minimum: Some(-2.0),
        maximum: Some(2.0),
        default_value: 0.0,
        mapping: ParameterMapping::Linear,
        automation_rate: AutomationRate::Block,
        channel_policy: ParameterChannelPolicy::PerLane,
        smoothing: SmoothingRule::None,
        smoothing_samples: 0,
        readable: true,
        automatable: true,
        nudge_ladder: None,
        enum_choices: &[],
    },
];
static STATE_PORTS: [PortDescriptorV1; 3] = [
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
    PortDescriptorV1 {
        id: port_id("detector"),
        role: PortRole::SidechainInput,
        required: false,
        layout: PortLayout::DualMonoPlanar,
    },
];
const fn state_quality(sample_rate: u32) -> QualityDescriptorV1 {
    QualityDescriptorV1 {
        quality: EffectQuality::Normal,
        sample_rate,
        latency: LatencySamples(9),
        tail: TailSamples::Finite(17),
        maximum_state: StatePayloadSizes {
            common_bytes: 3,
            left_bytes: 5,
            right_bytes: 5,
        },
        scratch_fixed_bytes: 11,
        scratch_bytes_per_frame: 2,
    }
}
static STATE_QUALITIES: [QualityDescriptorV1; 4] = [
    state_quality(44_100),
    state_quality(48_000),
    state_quality(88_200),
    state_quality(96_000),
];
static STATE_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
    id: effect_id("test.state"),
    display_name: "State test",
    contract_major: 1,
    contract_minor: 7,
    state_layout_version: 3,
    supported_link_modes: LinkModeSet::ALL,
    parameters: &STATE_PARAMETERS,
    ports: &STATE_PORTS,
    qualities: &STATE_QUALITIES,
};
static STATE_INITIAL: [InitialParameterValue; 3] = [
    InitialParameterValue {
        parameter_index: 0,
        channel: ParameterChannel::Both,
        value: 0.25,
    },
    InitialParameterValue {
        parameter_index: 1,
        channel: ParameterChannel::Left,
        value: -0.5,
    },
    InitialParameterValue {
        parameter_index: 1,
        channel: ParameterChannel::Right,
        value: 1.5,
    },
];

struct TrackingAllocator;

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator;

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATIONS: Cell<u64> = const { Cell::new(0) };
    static ALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static DEALLOCATED_BYTES: Cell<u64> = const { Cell::new(0) };
    static LIVE_BYTES: Cell<i64> = const { Cell::new(0) };
    static PEAK_LIVE_BYTES: Cell<i64> = const { Cell::new(0) };
}

fn allocated(size: usize) {
    let live = LIVE_BYTES.get() + size as i64;
    ALLOCATIONS.set(ALLOCATIONS.get() + 1);
    ALLOCATED_BYTES.set(ALLOCATED_BYTES.get() + size as u64);
    LIVE_BYTES.set(live);
    PEAK_LIVE_BYTES.set(PEAK_LIVE_BYTES.get().max(live));
}

fn deallocated(size: usize) {
    DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
    DEALLOCATED_BYTES.set(DEALLOCATED_BYTES.get() + size as u64);
    LIVE_BYTES.set(LIVE_BYTES.get() - size as i64);
}

fn when_active(action: impl FnOnce()) {
    ACTIVE.with(|active| {
        if active.get() {
            action();
        }
    });
}

// SAFETY: every operation delegates its original pointer/layout unchanged to `System`. The
// thread-local counters are observational and active only around a single test-thread call.
unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc(layout) };
        if !pointer.is_null() {
            when_active(|| {
                allocated(layout.size());
            });
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        // SAFETY: delegates the allocator-provided layout unchanged.
        let pointer = unsafe { System.alloc_zeroed(layout) };
        if !pointer.is_null() {
            when_active(|| {
                allocated(layout.size());
            });
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        when_active(|| {
            deallocated(layout.size());
        });
        // SAFETY: delegates the original pointer and layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: delegates the original pointer/layout and requested size unchanged.
        let replacement = unsafe { System.realloc(pointer, layout, new_size) };
        if !replacement.is_null() {
            when_active(|| {
                ALLOCATIONS.set(ALLOCATIONS.get() + 1);
                DEALLOCATIONS.set(DEALLOCATIONS.get() + 1);
                ALLOCATED_BYTES.set(ALLOCATED_BYTES.get() + new_size as u64);
                DEALLOCATED_BYTES.set(DEALLOCATED_BYTES.get() + layout.size() as u64);
                LIVE_BYTES.set(LIVE_BYTES.get() - layout.size() as i64 + new_size as i64);
                PEAK_LIVE_BYTES.set(PEAK_LIVE_BYTES.get().max(LIVE_BYTES.get()));
            });
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
    live_bytes: i64,
    peak_live_bytes: i64,
}

fn measure<T>(operation: impl FnOnce() -> T) -> (T, Snapshot) {
    ALLOCATIONS.set(0);
    DEALLOCATIONS.set(0);
    ALLOCATED_BYTES.set(0);
    DEALLOCATED_BYTES.set(0);
    LIVE_BYTES.set(0);
    PEAK_LIVE_BYTES.set(0);
    ACTIVE.set(true);
    let result = operation();
    ACTIVE.set(false);
    (
        result,
        Snapshot {
            allocations: ALLOCATIONS.get(),
            deallocations: DEALLOCATIONS.get(),
            allocated_bytes: ALLOCATED_BYTES.get(),
            deallocated_bytes: DEALLOCATED_BYTES.get(),
            live_bytes: LIVE_BYTES.get(),
            peak_live_bytes: PEAK_LIVE_BYTES.get(),
        },
    )
}

fn fixture() -> Vec<u8> {
    let compact: Vec<_> =
        include_str!("../../../fixtures/effect-package/v1/comprehensive-a.package.hex")
            .bytes()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("lowercase hex fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn hex_bytes(text: &str) -> Vec<u8> {
    let compact: Vec<_> = text
        .bytes()
        .filter(|byte| !byte.is_ascii_whitespace())
        .collect();
    compact
        .chunks_exact(2)
        .map(|pair| {
            let digit = |byte: u8| match byte {
                b'0'..=b'9' => byte - b'0',
                b'a'..=b'f' => byte - b'a' + 10,
                _ => panic!("lowercase hex fixture"),
            };
            digit(pair[0]) << 4 | digit(pair[1])
        })
        .collect()
}

fn state_replay() -> EffectStateReplayViewV1<'static> {
    EffectStateReplayViewV1 {
        effect_id: STATE_DESCRIPTOR.id,
        request: PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 8,
            quality: EffectQuality::Normal,
            bypass: true,
            link_mode: LinkMode::Maximum,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::Connected {
                    id: port_id("detector"),
                    required: false,
                },
            },
            initial_values: &STATE_INITIAL,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 64,
                maximum_scratch_bytes: 128,
                maximum_automation_spans_per_block: 23,
            },
        },
    }
}

const ZERO_ALLOCATION: Snapshot = Snapshot {
    allocations: 0,
    deallocations: 0,
    allocated_bytes: 0,
    deallocated_bytes: 0,
    live_bytes: 0,
    peak_live_bytes: 0,
};

#[test]
fn each_package_publication_has_one_nested_descriptor_pass_and_no_native_allocation() {
    let bytes = fixture();
    let descriptor_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;
    let descriptor = &bytes[96..96 + descriptor_len];
    let (_, descriptor_pass) = measure(|| effect_descriptor_identity_v1(descriptor, 4_194_304));
    assert!(descriptor_pass.allocations > 0);
    assert_eq!(descriptor_pass.live_bytes, 0);
    assert_eq!(
        descriptor_pass.allocated_bytes,
        descriptor_pass.deallocated_bytes
    );
    let (_, descriptor_verify_pass) =
        measure(|| verify_effect_descriptor_wire_v1(descriptor, 4_194_304));
    assert_eq!(descriptor_verify_pass, descriptor_pass);

    let verified = verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()).unwrap();
    let artifacts: Vec<_> = verified
        .artifacts()
        .map(|artifact| EffectArtifactAuthoringV1 {
            kind: artifact.kind(),
            path: artifact.path(),
            target: artifact.target(),
            features: artifact.features(),
            content: artifact.content(),
        })
        .collect();
    let authoring = EffectPackageAuthoringV1 {
        descriptor,
        artifacts: &artifacts,
    };
    let mut output = vec![0; bytes.len()];

    let (required, required_snapshot) =
        measure(|| effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default()));
    assert_eq!(required.unwrap(), bytes.len() as u64);
    assert_eq!(required_snapshot, descriptor_pass);

    let (encoded, encode_snapshot) = measure(|| {
        encode_effect_package_v1(&authoring, EffectPackageLimitsV1::default(), &mut output)
    });
    assert_eq!(encoded.unwrap(), bytes.len());
    assert_eq!(encode_snapshot, descriptor_pass);

    let (verified, verify_snapshot) =
        measure(|| verify_effect_package_v1(&bytes, EffectPackageLimitsV1::default()));
    let verified = verified.unwrap();
    assert_eq!(verify_snapshot, descriptor_pass);

    let (cid, cid_snapshot) =
        measure(|| effect_package_cid_v1(&bytes, EffectPackageLimitsV1::default()));
    let cid = cid.unwrap();
    assert_eq!(cid_snapshot, descriptor_pass);

    let (selected, select_snapshot) = measure(|| {
        select_effect_package_artifact_v1(
            &verified,
            ArtifactSelectionRequestV1 {
                kind: EffectArtifactKindV1::CoreWasm,
                target: "wasm32-unknown-unknown",
                capabilities: &["bulk-memory", "simd128"],
            },
        )
    });
    assert_eq!(selected.unwrap().path(), "wasm/bulk.wasm");
    assert_eq!(
        select_snapshot,
        Snapshot {
            allocations: 0,
            deallocations: 0,
            allocated_bytes: 0,
            deallocated_bytes: 0,
            live_bytes: 0,
            peak_live_bytes: 0,
        }
    );

    let (_, iteration_snapshot) = measure(|| {
        let mut count = 0;
        for artifact in verified.artifacts() {
            count += usize::from(!artifact.content().is_empty());
        }
        count
    });
    assert_eq!(iteration_snapshot, select_snapshot);

    let (_, cid_boundary_snapshot) = measure(|| {
        let binary = EffectCid::from_binary(cid.as_binary()).unwrap();
        let mut text = [0xa5; 64];
        let written = binary.write_text(&mut text).unwrap();
        let parsed: EffectCid = core::str::from_utf8(&text[..written])
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(parsed, cid);
        assert!(text[written..].iter().all(|byte| *byte == 0xa5));
    });
    assert_eq!(cid_boundary_snapshot, select_snapshot);
    println!(
        "issue081_package_allocation descriptor={descriptor_pass:?} package={verify_snapshot:?} postverify={select_snapshot:?}"
    );
}

/// The frozen 4,096-artifact cap is the authoring default, not a pathological input: encoding at
/// the cap must stay allocation-free relative to the one nested descriptor pass and finish in
/// single-digit milliseconds.
#[test]
fn encode_at_the_frozen_artifact_cap_has_one_nested_descriptor_pass_and_no_native_allocation() {
    let bytes = fixture();
    let descriptor_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;
    let descriptor = &bytes[96..96 + descriptor_len];
    let (_, descriptor_pass) = measure(|| effect_descriptor_identity_v1(descriptor, 4_194_304));
    assert!(descriptor_pass.allocations > 0);

    let count = 4_096usize;
    let paths: Vec<String> = (0..count)
        .rev()
        .map(|index| format!("src/file-{index:04}.rs"))
        .collect();
    let contents: Vec<[u8; 1]> = (0..count).map(|index| [index as u8]).collect();
    let artifacts: Vec<EffectArtifactAuthoringV1<'_>> = (0..count)
        .map(|index| EffectArtifactAuthoringV1 {
            kind: EffectArtifactKindV1::Source,
            path: &paths[index],
            target: "",
            features: "",
            content: &contents[index],
        })
        .collect();
    let authoring = EffectPackageAuthoringV1 {
        descriptor,
        artifacts: &artifacts,
    };

    let started = std::time::Instant::now();
    let (required, required_snapshot) =
        measure(|| effect_package_v1_required_size(&authoring, EffectPackageLimitsV1::default()));
    let required = required.unwrap() as usize;
    let mut output = vec![0; required];
    let (encoded, encode_snapshot) = measure(|| {
        encode_effect_package_v1(&authoring, EffectPackageLimitsV1::default(), &mut output)
    });
    let elapsed = started.elapsed();
    assert_eq!(encoded.unwrap(), required);
    assert_eq!(required_snapshot, descriptor_pass);
    assert_eq!(encode_snapshot, descriptor_pass);
    let budget_ms = if cfg!(debug_assertions) { 500 } else { 10 };
    assert!(
        elapsed.as_millis() < budget_ms,
        "required_size + encode at the artifact cap took {elapsed:?}"
    );

    let verified = verify_effect_package_v1(&output, EffectPackageLimitsV1::default()).unwrap();
    assert_eq!(verified.artifact_count(), count as u32);
    assert!(
        verified
            .artifacts()
            .map(|artifact| artifact.path())
            .is_sorted()
    );

    let extra = [EffectArtifactAuthoringV1 {
        path: "src/zzz.rs",
        ..artifacts[0]
    }];
    let grown: Vec<_> = artifacts.iter().copied().chain(extra).collect();
    let one_over = EffectPackageAuthoringV1 {
        descriptor,
        artifacts: &grown,
    };
    let generous = EffectPackageLimitsV1 {
        maximum_artifacts: u32::MAX,
        ..EffectPackageLimitsV1::default()
    };
    let error = effect_package_v1_required_size(&one_over, generous).unwrap_err();
    assert_eq!(
        (error.code, error.byte_offset),
        (EffectPackageDiagnosticCodeV1::Limit, 48)
    );
    println!(
        "issue097_package_cap artifacts={count} bytes={required} elapsed={elapsed:?} \
         allocations={encode_snapshot:?}"
    );
}

/// Issue 078 forbids the package layer validating one descriptor twice; the C `inspect` entry has
/// to obey it too. The oracle is the allocation count of exactly one nested Issue-082 pass.
#[test]
fn c_inspect_performs_exactly_one_nested_descriptor_pass() {
    let bytes = fixture();
    let descriptor_len = u64::from_le_bytes(bytes[24..32].try_into().unwrap()) as usize;
    let descriptor = &bytes[96..96 + descriptor_len];
    let (identity, descriptor_pass) =
        measure(|| effect_descriptor_identity_v1(descriptor, 4_194_304));
    let identity = identity.unwrap();
    assert!(descriptor_pass.allocations > 0);

    let verified = verify_effect_descriptor_wire_v1(descriptor, 4_194_304).unwrap();
    let (parameter_capacity, port_capacity, quality_capacity, choice_capacity) = (
        verified.parameter_count(),
        verified.port_count(),
        verified.quality_count(),
        verified.enum_choice_count(),
    );
    let mut summary = EffectDescriptorSummaryV1::default();
    let mut parameters =
        vec![EffectDescriptorParameterRecordV1::default(); parameter_capacity as usize];
    let mut ports = vec![EffectDescriptorPortRecordV1::default(); port_capacity as usize];
    let mut qualities = vec![EffectDescriptorQualityRecordV1::default(); quality_capacity as usize];
    let mut choices = vec![EffectDescriptorEnumChoiceRecordV1::default(); choice_capacity as usize];
    let (mut required_parameters, mut required_ports) = (0u32, 0u32);
    let (mut required_qualities, mut required_choices) = (0u32, 0u32);
    let mut diagnostic = EffectDescriptorWireDiagnosticV1::new(
        EffectDescriptorWireDiagnosticCodeV1::Ok,
        u32::MAX,
        u32::MAX,
    );
    let (code, snapshot) = measure(|| {
        // SAFETY: every pointer denotes live, correctly typed, mutually nonoverlapping storage of
        // the stated capacity for this call, and none is retained.
        unsafe {
            miso_engine_effect_descriptor_v1_inspect(
                descriptor.as_ptr(),
                descriptor.len(),
                4_194_304,
                &mut summary,
                parameters.as_mut_ptr(),
                parameter_capacity,
                ports.as_mut_ptr(),
                port_capacity,
                qualities.as_mut_ptr(),
                quality_capacity,
                choices.as_mut_ptr(),
                choice_capacity,
                &mut required_parameters,
                &mut required_ports,
                &mut required_qualities,
                &mut required_choices,
                &mut diagnostic,
            )
        }
    });
    assert_eq!(code, EffectDescriptorWireDiagnosticCodeV1::Ok as u32);
    assert_eq!(snapshot, descriptor_pass);
    assert_eq!(summary.identity, *identity.as_bytes());
    assert_eq!(
        (
            required_parameters,
            required_ports,
            required_qualities,
            required_choices
        ),
        (
            parameter_capacity,
            port_capacity,
            quality_capacity,
            choice_capacity
        )
    );
    println!("issue097_ffi_single_pass inspect={snapshot:?} descriptor_pass={descriptor_pass:?}");
}

/// A null wire with `wire_len == 0` is a legal argument tuple, so the boundary must report the
/// wire diagnostic the verifier gives for empty input, not `Null`, and must zero the counts.
#[test]
fn c_inspect_reports_a_wire_diagnostic_for_an_empty_null_wire() {
    let expected = verify_effect_descriptor_wire_v1(&[], 4_194_304).unwrap_err();
    assert_ne!(expected.code, EffectDescriptorWireDiagnosticCodeV1::Null);
    let mut summary = EffectDescriptorSummaryV1 {
        abi_version: 0xdead_beef,
        ..EffectDescriptorSummaryV1::default()
    };
    let untouched = summary;
    let (mut required_parameters, mut required_ports) = (0xa5a5_a5a5u32, 0xa5a5_a5a5u32);
    let (mut required_qualities, mut required_choices) = (0xa5a5_a5a5u32, 0xa5a5_a5a5u32);
    let mut diagnostic = EffectDescriptorWireDiagnosticV1::new(
        EffectDescriptorWireDiagnosticCodeV1::Ok,
        u32::MAX,
        u32::MAX,
    );
    // SAFETY: the wire pointer is null exactly with length zero, which the ABI permits; every
    // mandatory output denotes live writable storage and the optional record arrays are null with
    // zero capacity.
    let code = unsafe {
        miso_engine_effect_descriptor_v1_inspect(
            core::ptr::null(),
            0,
            4_194_304,
            &mut summary,
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            0,
            core::ptr::null_mut(),
            0,
            &mut required_parameters,
            &mut required_ports,
            &mut required_qualities,
            &mut required_choices,
            &mut diagnostic,
        )
    };
    assert_eq!(code, expected.code as u32);
    assert_eq!(diagnostic, expected);
    assert_eq!(
        (
            required_parameters,
            required_ports,
            required_qualities,
            required_choices
        ),
        (0, 0, 0, 0)
    );
    assert_eq!(summary, untouched);
}

#[test]
fn prebound_state_selection_verification_replay_requirements_and_encode_allocate_nothing() {
    let wire = hex_bytes(include_str!(
        "../../../fixtures/effect-state/v1/canonical.descriptor.wire.hex"
    ));
    let state = include_bytes!("../../../fixtures/effect-state/v1/canonical.state.bin");
    let state_before = *state;
    let bound = bind_effect_descriptor_wire_v1(&STATE_DESCRIPTOR, &wire, 1 << 20).unwrap();

    let (selector, selector_allocations) =
        measure(|| inspect_effect_state_selector_v1(state, EffectStateLimitsV1::default()));
    assert_eq!(selector.unwrap().descriptor_identity(), bound.identity());
    assert_eq!(selector_allocations, ZERO_ALLOCATION);

    let (verified, verify_allocations) =
        measure(|| verify_effect_state_v1(bound, state, EffectStateLimitsV1::default()));
    let verified = verified.unwrap();
    assert_eq!(verify_allocations, ZERO_ALLOCATION);
    let (_, current_allocations) = measure(|| validate_effect_state_current_layout_v1(verified));
    assert_eq!(current_allocations, ZERO_ALLOCATION);
    let (_, replay_allocations) =
        measure(|| validate_effect_state_replay_v1(verified, state_replay()));
    assert_eq!(replay_allocations, ZERO_ALLOCATION);
    let (requirements, requirement_allocations) = measure(|| {
        effect_state_v1_requirements(bound, state_replay(), EffectStateLimitsV1::default())
    });
    let requirements = requirements.unwrap();
    assert_eq!(requirement_allocations, ZERO_ALLOCATION);

    let (common, left, right) = verified.payloads();
    let mut output = vec![0x6d; requirements.envelope_bytes as usize + 7];
    let (written, encode_allocations) = measure(|| {
        encode_effect_state_v1(
            bound,
            state_replay(),
            common,
            left,
            right,
            EffectStateLimitsV1::default(),
            &mut output,
        )
    });
    assert_eq!(written.unwrap(), requirements.envelope_bytes);
    assert_eq!(encode_allocations, ZERO_ALLOCATION);
    assert_eq!(&output[..state.len()], state);
    assert!(output[state.len()..].iter().all(|byte| *byte == 0x6d));

    let mut short = vec![0xa5; state.len() - 1];
    let short_before = short.clone();
    let (failure, short_allocations) = measure(|| {
        encode_effect_state_v1(
            bound,
            state_replay(),
            common,
            left,
            right,
            EffectStateLimitsV1::default(),
            &mut short,
        )
    });
    assert!(failure.is_err());
    assert_eq!(short, short_before);
    assert_eq!(short_allocations, ZERO_ALLOCATION);
    assert_eq!(state, &state_before);
    println!("issue081_state_allocation zero={verify_allocations:?}");
}
