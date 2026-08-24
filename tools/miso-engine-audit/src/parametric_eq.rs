//! Prepared parametric-EQ realtime audit.
//!
//! Issue #87 replaced the endpoint-conditioned delta recurrence this audit was written for with a
//! cascade of TPT state-variable sections, and the state payload with it (version 2 is 616 bytes,
//! not 512). Only the admission limit moved here; the JSON `kind` label stays `issue042_...`
//! because the evidence trail that consumes it is issue #104's to rename.

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_core::realtime::audit;
use miso_engine_effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedNativeEffect,
    PreparedPortsV1, PreparedSidechainPort,
};
use miso_engine_parametric_eq::ParametricEqFactory;

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;

struct OffRenderDrop {
    effect: Box<dyn PreparedNativeEffect>,
}

impl Drop for OffRenderDrop {
    fn drop(&mut self) {
        assert!(
            !audit::is_render_scope_active(),
            "prepared EQ destruction must stay off render"
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
        "Issue-042 audit is frozen at 100,000 blocks"
    );
    let mut effect = OffRenderDrop {
        effect: prepare_eq(),
    };
    let mut left = [0.125_f32; QUANTUM as usize];
    let mut right = [-0.25_f32; QUANTUM as usize];
    let left_address = left.as_ptr() as usize;
    let right_address = right.as_ptr() as usize;

    audit::warm_up();
    audit::reset();
    eprintln!("MISO_ENGINE_PARAMETRIC_EQ_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 0..BLOCKS {
            left.fill(if block & 1 == 0 { 0.125 } else { -0.375 });
            right.fill(if block & 1 == 0 { -0.25 } else { 0.5 });
            let report = effect.effect.process(
                EffectProcessBlock::new(
                    &mut left,
                    &mut right,
                    None,
                    block
                        .checked_mul(u64::from(QUANTUM))
                        .expect("frozen sample time"),
                    &[],
                    QUANTUM,
                )
                .expect("prepared 128-frame block"),
            );
            assert_eq!(report.sanitized_main_samples, 0);
            assert_eq!(report.invalid_spans, 0);
            assert_eq!(report.nonfinite_left_blocks, 0);
            assert_eq!(report.nonfinite_right_blocks, 0);
            assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
        }
    });
    eprintln!("MISO_ENGINE_PARAMETRIC_EQ_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(right.as_ptr() as usize, right_address);
    assert_eq!(snapshot.total(), 0);
    drop(effect);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue042_parametric_eq_realtime_audit\",",
            "\"blocks\":100000,\"quantum_frames\":128,",
            "\"destruction_off_render\":true,\"left_address\":{},\"right_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        left_address,
        right_address,
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

fn prepare_eq() -> Box<dyn PreparedNativeEffect> {
    let factory = ParametricEqFactory;
    let mut initial_values = Vec::with_capacity(factory.descriptor().parameters.len() * 2);
    for (index, parameter) in factory.descriptor().parameters.iter().enumerate() {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            initial_values.push(InitialParameterValue {
                parameter_index: u32::try_from(index).expect("frozen descriptor count"),
                channel,
                value: parameter.default_value,
            });
        }
    }
    for channel in [ParameterChannel::Left, ParameterChannel::Right] {
        set(&mut initial_values, 0, channel, 1.0);
        set(&mut initial_values, 2, channel, 1_000.0);
        set(&mut initial_values, 3, channel, 6.0);
    }
    factory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: QUANTUM,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &initial_values,
            limits: PrepareEffectLimits {
                // Version-2 state is 616 bytes; production admits 100 MB over the C ABI.
                maximum_total_state_bytes: 1_024,
                maximum_scratch_bytes: 1,
                maximum_automation_spans_per_block: 48,
            },
        })
        .expect("prepared parametric EQ")
}

fn set(
    values: &mut [InitialParameterValue],
    parameter_index: usize,
    channel: ParameterChannel,
    value: f32,
) {
    values[parameter_index * 2
        + match channel {
            ParameterChannel::Left => 0,
            ParameterChannel::Right => 1,
            ParameterChannel::Both => panic!("per-lane prepared values"),
        }]
    .value = value;
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
