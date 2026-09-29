//! Native-effect conformance and realtime audit driver.
#![allow(missing_docs, unsafe_code)]

use std::{env, hint::black_box};

use bench_support::alloc as bench_alloc;
use conformance::{ConformanceConfig, DualAccumulatorDelayFactory, run_effect_conformance};
use effect_contract::{
    EffectProcessBlock, EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory,
    ParameterChannel, PrepareEffectLimits, PrepareEffectRequest, PreparedPorts,
    PreparedSidechainPort,
};
use effect_contract::{PreparedNativeEffect, ProcessReport};
use engine::realtime::audit;

fn prepared(bypass: bool) -> Box<dyn PreparedNativeEffect> {
    let factory = DualAccumulatorDelayFactory::correct();
    let initial = [
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 1.0,
        },
        InitialParameterValue {
            parameter_index: 0,
            channel: ParameterChannel::Right,
            value: 1.0,
        },
    ];
    factory
        .prepare(PrepareEffectRequest {
            sample_rate: 48_000,
            quantum: 128,
            quality: EffectQuality::Normal,
            bypass,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Unconnected {
                    id: conformance::DUAL_ACCUMULATOR_DELAY_DESCRIPTOR.ports[1].id,
                    required: false,
                },
            },
            initial_values: &initial,
            limits: PrepareEffectLimits {
                maximum_total_state_bytes: 1 << 20,
                maximum_scratch_bytes: 1 << 20,
                maximum_automation_spans_per_block: 8,
            },
        })
        .expect("valid bounded conformance processor")
}

fn audit_process(blocks: u64, markers: bool) {
    let mut effect = prepared(false);
    let mut left = [0.0_f32; 128];
    let mut right = [0.0_f32; 128];
    let mut totals = ProcessReport::default();
    audit::warm_up();
    audit::reset();
    if markers {
        eprintln!("MISO_ENGINE_EFFECT_RT_BEGIN");
    }
    for block_index in 0..blocks {
        let extreme = if block_index & 1 == 0 {
            f32::MAX
        } else {
            -f32::MAX
        };
        left.fill(extreme);
        right.fill(-extreme);
        let block = EffectProcessBlock::new(
            &mut left,
            &mut right,
            None,
            block_index.saturating_mul(128),
            &[],
            128,
        )
        .expect("fixed audit block");
        let report = audit::in_render_scope(|| effect.process(block));
        totals.sanitized_main_samples = totals
            .sanitized_main_samples
            .saturating_add(report.sanitized_main_samples);
        totals.invalid_spans = totals.invalid_spans.saturating_add(report.invalid_spans);
        assert!(left.iter().chain(&right).all(|value| value.is_finite()));
        black_box((&left, &right));
    }
    if markers {
        eprintln!("MISO_ENGINE_EFFECT_RT_END");
    }
    let snapshot = audit::snapshot();
    assert_eq!(snapshot.total(), 0);
    assert_eq!(totals.sanitized_main_samples, 0);
    assert_eq!(totals.invalid_spans, 0);
    println!(
        "{{\"schema_version\":1,\"kind\":\"effect_realtime_audit\",\"blocks\":{blocks},\"frames_per_block\":128,\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}",
        snapshot.allocations,
        snapshot.deallocations,
        snapshot.locks,
        snapshot.logs,
        snapshot.file_io,
        snapshot.network_io,
        snapshot.syscalls,
        snapshot.total()
    );
}

fn conformance() {
    // #105 phase 2: `run_effect_conformance` proves its own allocation detector is live by
    // allocating inside an armed render scope on purpose, so this invocation counts and continues
    // instead of aborting. `--audit` (the mode that asserts zero violations) keeps the default
    // abort policy: it is a different process.
    bench_alloc::set_mode(bench_alloc::Mode::Count);
    let report = run_effect_conformance(
        &DualAccumulatorDelayFactory::correct(),
        ConformanceConfig {
            quantum: 128,
            blocks: 1,
        },
    );
    assert!(
        report.passed(),
        "failed launch gates: {:?}",
        report.launch_gates.failures
    );
    println!(
        "{{\"schema_version\":1,\"kind\":\"effect_conformance\",\"launch_prepared_configurations\":{},\"launch_process_calls\":{},\"launch_failed_gates\":0}}",
        report.launch_gates.prepared_configurations, report.launch_gates.process_calls,
    );
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [mode] if mode == "--conformance" => conformance(),
        [mode, blocks] if mode == "--audit" => {
            audit_process(blocks.parse().expect("block count"), false)
        }
        [mode, blocks, marker] if mode == "--audit" && marker == "--trace-markers" => {
            audit_process(blocks.parse().expect("block count"), true)
        }
        _ => {
            eprintln!(
                "usage: effect_contract_bench --conformance | --audit BLOCKS [--trace-markers]"
            );
            std::process::exit(2);
        }
    }
}
