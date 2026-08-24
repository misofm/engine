//! Prepared dual-mono / ping-pong delay realtime audit (issue #93 eval E11).
//!
//! Two prepared delays -- one dual mono, one with the feedback matrix engaged -- render
//! 100_000 blocks inside one render scope, with automation points every 64 blocks so that tap
//! crossfades, parameter ramps and the one-frame chunks that carry a D11 snap all execute inside
//! the audited region. Any allocation, deallocation, lock, log or syscall between the markers is a
//! violation; the process aborts on the first one.

use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_core::realtime::audit;
use miso_engine_delay::{DELAY_PARAMETERS_V1, DelayFactory};
use miso_engine_effect_contract::{
    AutomationSpanKind, EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode,
    NativeEffectFactory, ParameterChannel, PrepareEffectLimits, PrepareEffectRequest,
    PreparedAutomationSpan, PreparedNativeEffect, PreparedPortsV1, PreparedSidechainPort,
    ProcessReport,
};

const BLOCKS: u64 = 100_000;
const QUANTUM: u32 = 128;
const SAMPLE_RATE: u32 = 48_000;
/// Blocks between automation events, so a 128-update crossfade is always in flight.
const EVENT_BLOCKS: u64 = 64;

struct OffRenderDrop {
    effects: Vec<Box<dyn PreparedNativeEffect>>,
}

impl Drop for OffRenderDrop {
    fn drop(&mut self) {
        assert!(
            !audit::is_render_scope_active(),
            "prepared delay destruction must stay off render"
        );
    }
}

fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    assert_eq!(
        parse_blocks(),
        BLOCKS,
        "issue-093 audit is frozen at 100,000 blocks"
    );
    let mut effects = OffRenderDrop {
        effects: vec![prepare_delay(0.0), prepare_delay(0.5)],
    };
    let mut left = [0.125_f32; QUANTUM as usize];
    let mut right = [-0.25_f32; QUANTUM as usize];
    let left_address = left.as_ptr() as usize;
    let right_address = right.as_ptr() as usize;

    audit::warm_up();
    audit::reset();
    eprintln!("MISO_ENGINE_DELAY_RT_BEGIN");
    audit::in_render_scope(|| {
        for block in 0..BLOCKS {
            let first_sample = block
                .checked_mul(u64::from(QUANTUM))
                .expect("frozen sample time");
            let spans = automation(block, first_sample);
            let automation: &[PreparedAutomationSpan] = if block % EVENT_BLOCKS == 0 && block > 0 {
                &spans
            } else {
                &[]
            };
            for effect in &mut effects.effects {
                left.fill(if block & 1 == 0 { 0.125 } else { -0.375 });
                right.fill(if block & 1 == 0 { -0.25 } else { 0.5 });
                let report = effect.process(
                    EffectProcessBlock::new(
                        &mut left,
                        &mut right,
                        None,
                        first_sample,
                        automation,
                        QUANTUM,
                    )
                    .expect("prepared 128-frame block"),
                );
                assert_eq!(report, ProcessReport::default());
                assert!(left.iter().chain(&right).all(|sample| sample.is_finite()));
            }
        }
    });
    eprintln!("MISO_ENGINE_DELAY_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(left.as_ptr() as usize, left_address);
    assert_eq!(right.as_ptr() as usize, right_address);
    assert_eq!(snapshot.total(), 0);
    drop(effects);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue093_delay_realtime_audit\",",
            "\"blocks\":100000,\"quantum_frames\":128,\"effects\":2,",
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

/// The automation event of one block: a delay time, a damping value and the matrix position, so
/// every control-rate path -- the integer tap mapping, the `tan`/`log` coefficient design and the
/// ramp retarget -- runs inside the render scope.
fn automation(block: u64, first_sample: u64) -> [PreparedAutomationSpan; 3] {
    let step = (block / EVENT_BLOCKS % 4) as usize;
    let delay = [3.0_f32, 11.0, 2.0, 7.0];
    let damping = [0.0_f32, 0.25, 0.995, 0.5];
    let cross = [0.0_f32, 0.5, 1.0, 0.25];
    let point = |parameter_index: u32, channel, value| PreparedAutomationSpan {
        kind: AutomationSpanKind::Point,
        channel,
        parameter_index,
        start_sample: first_sample,
        end_sample: first_sample,
        start_value: value,
        end_value: value,
    };
    [
        point(0, ParameterChannel::Left, delay[step]),
        point(2, ParameterChannel::Right, damping[step]),
        point(4, ParameterChannel::Both, cross[step]),
    ]
}

fn prepare_delay(cross: f32) -> Box<dyn PreparedNativeEffect> {
    let factory = DelayFactory;
    let mut initial_values: Vec<InitialParameterValue> = Vec::with_capacity(9);
    for (index, parameter) in DELAY_PARAMETERS_V1.iter().enumerate().take(4) {
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            initial_values.push(InitialParameterValue {
                parameter_index: u32::try_from(index).expect("frozen descriptor count"),
                channel,
                value: parameter.default_value,
            });
        }
    }
    initial_values.push(InitialParameterValue {
        parameter_index: 4,
        channel: ParameterChannel::Both,
        value: cross,
    });
    factory
        .prepare(PrepareEffectRequest {
            sample_rate: SAMPLE_RATE,
            quantum: QUANTUM,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::None,
            },
            initial_values: &initial_values,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 768_168,
                maximum_scratch_bytes: 36,
                maximum_automation_spans_per_block: 16,
            },
        })
        .expect("prepared two-second delay")
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
