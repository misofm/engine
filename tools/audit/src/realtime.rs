//! Deterministic issue-003 realtime audit and bounded descriptive benchmark.

use crate::record::{json_f64_array, json_integer_array, metadata};
use bench_support::alloc as bench_alloc;
use core::num::NonZeroUsize;
use engine::realtime::audit::{self, AuditSnapshot, ForbiddenOperation};
use engine::realtime::{
    PlanExchangeConfig, PlanarBufferMut, PrepareRenderPlan, PreparedRenderPlan, RenderEnvelope,
    RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use engine::{QuantumFrames, SampleRateHz};
use std::env;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
struct RoundEvidence {
    elapsed: Duration,
    swaps: u64,
    deferred: u64,
    output_address: usize,
    audit: AuditSnapshot,
}

enum Mode {
    Audit,
    Benchmark(u8),
    Probe(ForbiddenOperation),
}

fn prepared_plan(id: u64) -> PreparedRenderPlan {
    PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: id,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(1),
            input_channels: None,
            output_channels: NonZeroUsize::new(1).expect("one output channel"),
        },
        scratch: &[],
    })
    .expect("valid prepared audit plan")
}

fn run_round(blocks: u64, trace_markers: bool) -> RoundEvidence {
    assert!(blocks >= 3, "audit requires at least three blocks");
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        prepared_plan(1),
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("publication capacity"),
            retirement_capacity: NonZeroUsize::new(1).expect("retirement capacity"),
        },
    )
    .expect("valid plan exchange");
    publisher
        .publish(prepared_plan(2))
        .unwrap_or_else(|_| panic!("first audit plan must publish"));
    let mut third_plan = Some(prepared_plan(3));
    let mut first_retired = None;
    let mut second_retired = None;
    let mut output = [1.0_f32];
    let output_address = output.as_ptr() as usize;
    let mut swaps = 0_u64;
    let mut deferred = 0_u64;

    audit::warm_up();
    audit::reset();
    let started = Instant::now();
    if trace_markers {
        eprintln!("MISO_ENGINE_RT_BEGIN");
    }
    for block in 0..blocks {
        if block == 1 {
            let candidate = third_plan.take().expect("third plan exists");
            publisher
                .publish(candidate)
                .unwrap_or_else(|_| panic!("third audit plan must publish"));
        }
        let io = RenderIo {
            input: None,
            output: PlanarBufferMut::try_new(&mut output, 1, 1, 1).expect("fixed output view"),
        };
        let report = owner
            .render(
                io,
                RenderTime {
                    absolute_sample: block,
                },
            )
            .expect("bounded reference render");
        match report.swap {
            SwapOutcome::Applied => swaps = swaps.saturating_add(1),
            SwapOutcome::DeferredRetirementFull => {
                deferred = deferred.saturating_add(1);
                first_retired = Some(retirer.try_reclaim().expect("first retired plan"));
            }
            SwapOutcome::None => {}
        }
        assert_eq!(report.render.plan_id, owner.active_plan_id());
        assert_eq!(report.active_epoch, owner.active_epoch());
        assert_eq!(output, [0.0]);
        assert_eq!(output.as_ptr() as usize, output_address);
    }
    if trace_markers {
        eprintln!("MISO_ENGINE_RT_END");
    }
    let elapsed = started.elapsed();
    let audit = audit::snapshot();
    second_retired = second_retired.or_else(|| retirer.try_reclaim().ok());
    assert!(first_retired.is_some());
    assert!(second_retired.is_some());
    assert_eq!(swaps, 2);
    assert_eq!(deferred, 1);
    assert_eq!(audit.total(), 0);

    RoundEvidence {
        elapsed,
        swaps,
        deferred,
        output_address,
        audit,
    }
}

fn run_probe(operation: ForbiddenOperation) -> ! {
    audit::warm_up();
    audit::reset();
    match operation {
        ForbiddenOperation::Allocation => audit::in_render_scope(|| {
            let values = vec![1_u8];
            std::hint::black_box(values);
        }),
        ForbiddenOperation::Deallocation => {
            let value = Box::new(1_u8);
            audit::in_render_scope(|| drop(value));
        }
        other => audit::in_render_scope(|| audit::forbidden(other)),
    }
    panic!("mutation probe unexpectedly survived")
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let (mode, blocks, trace_markers) = parse_arguments();
    match mode {
        Mode::Probe(operation) => run_probe(operation),
        Mode::Audit => {
            let evidence = run_round(blocks, trace_markers);
            println!(
                concat!(
                    "{{\"schema_version\":1,\"kind\":\"realtime_audit\",",
                    "\"blocks\":{},\"swaps_accepted\":{},\"swaps_deferred\":{},",
                    "\"output_address\":{},\"allocations\":{},\"deallocations\":{},",
                    "\"locks\":{},\"logs\":{},\"file_io\":{},\"network_io\":{},",
                    "\"syscalls\":{},\"total_violations\":{}}}"
                ),
                blocks,
                evidence.swaps,
                evidence.deferred,
                evidence.output_address,
                evidence.audit.allocations,
                evidence.audit.deallocations,
                evidence.audit.locks,
                evidence.audit.logs,
                evidence.audit.file_io,
                evidence.audit.network_io,
                evidence.audit.syscalls,
                evidence.audit.total(),
            );
        }
        Mode::Benchmark(rounds) => {
            let mut durations = Vec::with_capacity(usize::from(rounds));
            for _ in 0..rounds {
                durations.push(run_round(blocks, false).elapsed);
            }
            let ns_per_block = durations
                .iter()
                .map(|duration| duration.as_nanos() as f64 / blocks as f64)
                .collect::<Vec<_>>();
            println!(
                concat!(
                    "{{\"schema_version\":1,\"benchmark\":\"realtime_plan_lifetime\",",
                    "\"cpu\":\"{}\",\"os\":\"{}\",\"power_mode\":\"{}\",",
                    "\"compiler\":\"{}\",\"llvm_version\":\"{}\",",
                    "\"target_triple\":\"{}\",\"compile_target_features\":\"{}\",",
                    "\"runtime_or_browser\":\"{}\",\"sample_rate_hz\":48000,",
                    "\"quantum_frames\":1,\"fixture\":\"bounded_plan_swap_silence\",",
                    "\"warmup_blocks\":0,\"blocks_per_round\":{},\"rounds\":{},",
                    "\"round_duration_ns\":{},\"ns_per_block\":{},",
                    "\"statistical_method\":\"per-round ns/block; descriptive only; no threshold\"}}"
                ),
                metadata("MISO_ENGINE_BENCH_CPU_MODEL"),
                env::consts::OS,
                metadata("MISO_ENGINE_BENCH_GOVERNOR_OR_POWER_MODE"),
                metadata("MISO_ENGINE_BENCH_RUST_VERSION"),
                metadata("MISO_ENGINE_BENCH_LLVM_VERSION"),
                metadata("MISO_ENGINE_BENCH_TARGET_TRIPLE"),
                metadata("MISO_ENGINE_BENCH_TARGET_FEATURES"),
                metadata("MISO_ENGINE_BENCH_RUNTIME_OR_BROWSER"),
                blocks,
                rounds,
                json_integer_array(durations.iter().map(Duration::as_nanos)),
                json_f64_array(ns_per_block.iter().copied()),
            );
        }
    }
}

fn parse_arguments() -> (Mode, u64, bool) {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    let mut blocks = 1_000_000_u64;
    let mut mode = None;
    let mut trace_markers = false;
    let mut index = 0;
    while index < arguments.len() {
        match arguments[index].as_str() {
            "--blocks" => {
                index += 1;
                blocks = arguments
                    .get(index)
                    .and_then(|value| value.parse().ok())
                    .expect("--blocks requires an integer");
            }
            "--audit" => mode = Some(Mode::Audit),
            "--trace-markers" => trace_markers = true,
            "--benchmark-rounds" => {
                index += 1;
                let rounds = arguments
                    .get(index)
                    .and_then(|value| value.parse::<u8>().ok())
                    .filter(|rounds| matches!(rounds, 1 | 2))
                    .expect("--benchmark-rounds requires 1 or 2");
                mode = Some(Mode::Benchmark(rounds));
            }
            "--probe" => {
                index += 1;
                mode = Some(Mode::Probe(parse_operation(
                    arguments.get(index).expect("--probe requires an operation"),
                )));
            }
            _ => panic!("unknown argument: {}", arguments[index]),
        }
        index += 1;
    }
    (mode.unwrap_or(Mode::Audit), blocks, trace_markers)
}

fn parse_operation(value: &str) -> ForbiddenOperation {
    match value {
        "allocation" => ForbiddenOperation::Allocation,
        "deallocation" => ForbiddenOperation::Deallocation,
        "lock" => ForbiddenOperation::Lock,
        "log" => ForbiddenOperation::Log,
        "file-io" => ForbiddenOperation::FileIo,
        "network-io" => ForbiddenOperation::NetworkIo,
        "syscall" => ForbiddenOperation::Syscall,
        _ => panic!("unknown probe operation: {value}"),
    }
}
