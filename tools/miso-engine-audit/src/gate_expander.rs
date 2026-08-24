//! Prepared gate/expander realtime audit (#89 gate 7.10).
//!
//! Drives the two production shapes -- an eight-lane homogeneous bank and a scalar instance with a
//! connected sidechain -- for [`BLOCKS`] blocks inside an armed render scope, and reports whether
//! anything allocated, locked, logged or reached the operating system. The markers are what
//! `strace` brackets: nothing between `MISO_ENGINE_GATE_EXPANDER_RT_BEGIN` and `..._RT_END` may be a
//! syscall.
//!
//! Zero allocation is the claim under audit. The gate allocates exactly two `Box<[f32]>` rings per
//! instance at preparation (four when a sidechain is connected) and nothing afterwards; the
//! boundary check, the lane recovery and the parameter smoothing all work in place.

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_core::realtime::audit;
use miso_engine_effect_contract::{
    BankWidth, EffectBankProcessBlock, EffectProcessBlock, EffectQuality, InitialParameterValue,
    LinkMode, NativeEffectFactory, ParameterChannel, PortId, PrepareEffectBankRequest,
    PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect, PreparedNativeEffectBank,
    PreparedPortsV1, PreparedSidechainPort,
};
use miso_engine_gate_expander::{GATE_EXPANDER_DESCRIPTOR_V1, GateExpanderFactory};
use miso_engine_lane::Backend;

/// Frozen block count, matching the other realtime audits.
const BLOCKS: u64 = 100_000;

/// Frozen render quantum.
const QUANTUM: u32 = 128;

/// Lanes in the audited bank.
const WIDTH: usize = 8;

/// Holds the prepared shapes so their destruction can be asserted to happen off render.
struct OffRenderDrop {
    bank: Option<Box<dyn PreparedNativeEffectBank>>,
    scalar: Box<dyn PreparedNativeEffect>,
}

impl Drop for OffRenderDrop {
    fn drop(&mut self) {
        assert!(
            !audit::is_render_scope_active(),
            "prepared gate destruction must stay off render"
        );
    }
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(
        parse_blocks(),
        BLOCKS,
        "the audit is frozen at 100,000 blocks"
    );
    let mut prepared = OffRenderDrop {
        bank: prepare_bank(),
        scalar: prepare_scalar(true),
    };
    let bank_available = prepared.bank.is_some();

    let mut bank_left = [0.0_f32; QUANTUM as usize * WIDTH];
    let mut bank_right = [0.0_f32; QUANTUM as usize * WIDTH];
    let mut left = [0.0_f32; QUANTUM as usize];
    let mut right = [0.0_f32; QUANTUM as usize];
    let mut sidechain_left = [0.0_f32; QUANTUM as usize];
    let mut sidechain_right = [0.0_f32; QUANTUM as usize];
    let offsets = [0_u32; WIDTH + 1];
    let bank_left_address = bank_left.as_ptr() as usize;
    let left_address = left.as_ptr() as usize;

    audit::warm_up();
    audit::reset();
    eprintln!("MISO_ENGINE_GATE_EXPANDER_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 0..BLOCKS {
            // A 1 kHz tone that is gated on and off every 100 ms, so both one-pole rates, the hold
            // and the identity select are all exercised inside the audited region.
            let first = block * u64::from(QUANTUM);
            let gated = (first / 4_800) % 2 == 0;
            for frame in 0..QUANTUM as usize {
                let phase = (first as usize + frame) % 48;
                let tone = if gated {
                    0.251_188_64 * SINE[phase]
                } else {
                    0.000_251_188_64 * SINE[phase]
                };
                left[frame] = tone;
                right[frame] = -tone;
                sidechain_left[frame] = tone;
                sidechain_right[frame] = tone;
                for lane in 0..WIDTH {
                    bank_left[frame * WIDTH + lane] = tone;
                    bank_right[frame * WIDTH + lane] = -tone;
                }
            }
            if let Some(bank) = prepared.bank.as_mut() {
                let report = bank.process_bank(
                    EffectBankProcessBlock::new(
                        &mut bank_left,
                        &mut bank_right,
                        None,
                        QUANTUM,
                        BankWidth::Eight,
                        first,
                        &[],
                        &offsets,
                        QUANTUM,
                    )
                    .expect("prepared 128-frame bank block"),
                );
                for lane in 0..WIDTH {
                    assert_eq!(report.reports[lane].nonfinite_left_blocks, 0);
                    assert_eq!(report.reports[lane].nonfinite_right_blocks, 0);
                    assert_eq!(report.reports[lane].invalid_spans, 0);
                }
                assert!(bank_left.iter().all(|sample| sample.is_finite()));
            }
            let report = prepared.scalar.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    Some((&sidechain_left, &sidechain_right)),
                    first,
                    &[],
                    QUANTUM,
                )
                .expect("prepared 128-frame block"),
            );
            assert_eq!(report.nonfinite_left_blocks, 0);
            assert_eq!(report.nonfinite_right_blocks, 0);
            assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
        }
    });
    eprintln!("MISO_ENGINE_GATE_EXPANDER_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(bank_left.as_ptr() as usize, bank_left_address);
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(snapshot.total(), 0);
    drop(prepared);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue089_gate_expander_realtime_audit\",",
            "\"blocks\":100000,\"quantum_frames\":128,\"bank_width\":8,",
            "\"bank_available\":{},\"connected_scalar\":true,",
            "\"destruction_off_render\":true,\"bank_left_address\":{},\"left_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        bank_available,
        bank_left_address,
        left_address,
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.total(),
    );
}

/// One period of a 1 kHz sine at 48 kHz, so the render loop needs no transcendental.
static SINE: [f32; 48] = {
    let mut table = [0.0_f32; 48];
    // A twelve-point quarter wave, mirrored: exact enough for an audit signal and constant.
    let quarter = [
        0.0,
        0.130_526_19,
        0.258_819_04,
        0.382_683_43,
        0.5,
        0.608_761_4,
        0.707_106_77,
        0.793_353_3,
        0.866_025_4,
        0.923_879_5,
        0.965_925_8,
        0.991_444_9,
    ];
    let mut index = 0;
    while index < 12 {
        table[index] = quarter[index];
        table[index + 12] = quarter[11 - index];
        table[index + 24] = -quarter[index];
        table[index + 36] = -quarter[11 - index];
        index += 1;
    }
    table
};

/// The eight-lane bank, or `None` when this build has no eight-lane backend.
fn prepare_bank() -> Option<Box<dyn PreparedNativeEffectBank>> {
    let values = active_values();
    let requests: Vec<PrepareEffectRequest<'_>> =
        (0..WIDTH).map(|_| request(&values, false)).collect();
    GateExpanderFactory
        .bind_homogeneous_bank(PrepareEffectBankRequest {
            backend: Backend::Simd8,
            width: BankWidth::Eight,
            requests: &requests,
        })
        .expect("valid bank request")
}

/// The scalar instance, with a sidechain connected so the connected kernel variant is audited too.
fn prepare_scalar(connected: bool) -> Box<dyn PreparedNativeEffect> {
    let values = active_values();
    GateExpanderFactory
        .prepare(request(&values, connected))
        .expect("prepared gate/expander")
}

fn request(values: &[InitialParameterValue], connected: bool) -> PrepareEffectRequest<'_> {
    let quality = GATE_EXPANDER_DESCRIPTOR_V1
        .qualities
        .iter()
        .find(|quality| quality.sample_rate == 48_000)
        .expect("launch rate");
    let id = PortId::new("sidechain-in").expect("static port id");
    PrepareEffectRequest {
        sample_rate: 48_000,
        quantum: QUANTUM,
        quality: EffectQuality::Normal,
        bypass: false,
        link_mode: LinkMode::DualMono,
        ports: PreparedPortsV1 {
            sidechain: if connected {
                PreparedSidechainPort::Connected {
                    id,
                    required: false,
                }
            } else {
                PreparedSidechainPort::Unconnected {
                    id,
                    required: false,
                }
            },
        },
        initial_values: values,
        limits: PrepareEffectLimits {
            maximum_total_state_bytes: quality.maximum_state.total().expect("state total"),
            maximum_scratch_bytes: 64,
            maximum_automation_spans_per_block: 16,
        },
    }
}

/// The parameter set the audit renders with: a high threshold, a steep ratio and a short hold, so
/// the gate is actually working rather than resting in its identity path.
fn active_values() -> Vec<InitialParameterValue> {
    let chosen = [-20.0_f32, 20.0, 48.0, 6.0, 1.0, 0.0, 5.0, 10.0];
    let mut values = Vec::with_capacity(chosen.len() * 2);
    for (index, value) in chosen.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            values.push(InitialParameterValue {
                parameter_index: u32::try_from(index).expect("frozen descriptor count"),
                channel,
                value: *value,
            });
        }
    }
    values
}

fn parse_blocks() -> u64 {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        None => BLOCKS,
        Some("--blocks") => arguments
            .next()
            .expect("--blocks value")
            .parse()
            .expect("integer block count"),
        Some(_) => panic!("unknown audit argument"),
    }
}
