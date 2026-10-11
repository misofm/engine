//! Deterministic issue-003 realtime audit.

use bench_support::alloc as bench_alloc;
use core::num::NonZeroUsize;
use engine::realtime::audit::{self, AuditSnapshot, ForbiddenOperation};
use engine::realtime::{
    PlanEpoch, PlanExchangeConfig, PlanReplacementReservationError, PlanarBufferMut,
    PrepareRenderPlan, PreparedRenderPlan, RealtimePlanOwner, RealtimeRenderReport, RenderEnvelope,
    RenderIo, RenderTime, SwapOutcome, Withdrawal, plan_exchange,
};
use engine::{QuantumFrames, SampleRateHz};
use std::env;

#[derive(Clone, Copy)]
struct RoundEvidence {
    swaps: u64,
    reservations_refused: u64,
    prior_plan_renders_while_refused: u64,
    withdrawals: u64,
    republished_adoptions: u64,
    output_address: usize,
    audit: AuditSnapshot,
}

enum Mode {
    Audit,
    Probe(ForbiddenOperation),
}

fn prepared_plan(id: u64) -> PreparedRenderPlan {
    PreparedRenderPlan::prepare(PrepareRenderPlan {
        plan_id: id,
        envelope: RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(1),
            output_channels: NonZeroUsize::new(1).expect("one output channel"),
        },
        scratch: &[],
    })
    .expect("valid prepared audit plan")
}

/// One rendered block, checked against the owner's own view of the active plan.
fn render_block(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; 1],
    output_address: usize,
    block: u64,
) -> RealtimeRenderReport {
    let io = RenderIo {
        output: PlanarBufferMut::try_new(output, 1, 1, 1).expect("fixed output view"),
    };
    let report = owner
        .render(
            io,
            RenderTime {
                absolute_sample: block,
            },
        )
        .expect("bounded reference render");
    assert_eq!(report.render.plan_id, owner.active_plan_id());
    assert_eq!(report.active_epoch, owner.active_epoch());
    assert_eq!(*output, [0.0]);
    assert_eq!(output.as_ptr() as usize, output_address);
    report
}

/// Block 0 adopts plan 2 (epoch 1), which fills the one-plan retirement queue with plan 1.
///
/// Refused-reservation round, blocks `1..blocks - 2`: before each block control tries to reserve
/// plan 3 and is refused `RetirementFull`, and render keeps rendering plan 2.
///
/// Withdraw-and-republish round: control reclaims plan 1, publishes plan 3 (epoch 2) and
/// withdraws it before render claims it; block `blocks - 2` renders plan 2 and applies nothing;
/// control republishes plan 3, and block `blocks - 1` adopts it with epoch 2.
fn run_round(blocks: u64, trace_markers: bool) -> RoundEvidence {
    assert!(blocks >= 4, "audit requires at least four blocks");
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        prepared_plan(1),
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("retirement capacity"),
        },
    )
    .expect("valid plan exchange");
    publisher
        .publish(prepared_plan(2))
        .unwrap_or_else(|_| panic!("first audit plan must publish"));
    let mut third_plan = Some(prepared_plan(3));
    let mut output = [1.0_f32];
    let output_address = output.as_ptr() as usize;
    let mut swaps = 0_u64;
    let mut reservations_refused = 0_u64;
    let mut prior_plan_renders_while_refused = 0_u64;
    let mut withdrawals = 0_u64;
    let mut republished_adoptions = 0_u64;

    audit::warm_up();
    audit::reset();
    if trace_markers {
        eprintln!("MISO_ENGINE_RT_BEGIN");
    }
    let report = render_block(&mut owner, &mut output, output_address, 0);
    assert_eq!(report.swap, SwapOutcome::Applied);
    assert_eq!(
        (report.active_epoch, report.render.plan_id),
        (PlanEpoch(1), 2)
    );
    swaps += 1;

    for block in 1..blocks - 2 {
        let candidate = third_plan.take().expect("third plan is control-owned");
        match publisher.reserve_replacement(candidate, engine::realtime::PlanAdoption::Next) {
            Err(PlanReplacementReservationError::RetirementFull(returned)) => {
                third_plan = Some(returned);
                reservations_refused += 1;
            }
            _ => panic!("a reservation must be refused while the retirement queue is full"),
        }
        let report = render_block(&mut owner, &mut output, output_address, block);
        assert_eq!(report.swap, SwapOutcome::None);
        assert_eq!(
            (report.active_epoch, report.render.plan_id),
            (PlanEpoch(1), 2)
        );
        prior_plan_renders_while_refused += 1;
    }

    let first_retired = retirer.try_reclaim().ok();
    let epoch = publisher
        .publish(third_plan.take().expect("third plan is control-owned"))
        .unwrap_or_else(|_| panic!("third audit plan must publish after reclamation"));
    assert_eq!(epoch, PlanEpoch(2));
    let candidate = match publisher.withdraw() {
        Withdrawal::Withdrawn(candidate) => candidate,
        other => panic!("an unclaimed candidate must withdraw, got {other:?}"),
    };
    assert_eq!((candidate.epoch(), candidate.plan_id()), (PlanEpoch(2), 3));
    withdrawals += 1;
    let report = render_block(&mut owner, &mut output, output_address, blocks - 2);
    assert_eq!(report.swap, SwapOutcome::None);
    assert_eq!(
        (report.active_epoch, report.render.plan_id),
        (PlanEpoch(1), 2)
    );
    assert_eq!(publisher.republish(candidate), PlanEpoch(2));
    let report = render_block(&mut owner, &mut output, output_address, blocks - 1);
    assert_eq!(report.swap, SwapOutcome::Applied);
    assert_eq!(
        (report.active_epoch, report.render.plan_id),
        (PlanEpoch(2), 3)
    );
    swaps += 1;
    republished_adoptions += 1;
    if trace_markers {
        eprintln!("MISO_ENGINE_RT_END");
    }
    let audit = audit::snapshot();
    let second_retired = retirer.try_reclaim().ok();
    assert_eq!(first_retired.map(|(epoch, _)| epoch), Some(PlanEpoch(0)));
    assert_eq!(second_retired.map(|(epoch, _)| epoch), Some(PlanEpoch(1)));
    assert_eq!(swaps, 2);
    assert_eq!(reservations_refused, blocks - 3);
    assert_eq!(prior_plan_renders_while_refused, blocks - 3);
    assert_eq!((withdrawals, republished_adoptions), (1, 1));
    assert_eq!(audit.total(), 0);

    RoundEvidence {
        swaps,
        reservations_refused,
        prior_plan_renders_while_refused,
        withdrawals,
        republished_adoptions,
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
                    "\"blocks\":{},\"swaps_accepted\":{},\"reservations_refused\":{},",
                    "\"prior_plan_renders_while_refused\":{},\"withdrawals\":{},",
                    "\"republished_adoptions\":{},",
                    "\"output_address\":{},\"allocations\":{},\"deallocations\":{},",
                    "\"locks\":{},\"logs\":{},\"file_io\":{},\"network_io\":{},",
                    "\"syscalls\":{},\"total_violations\":{}}}"
                ),
                blocks,
                evidence.swaps,
                evidence.reservations_refused,
                evidence.prior_plan_renders_while_refused,
                evidence.withdrawals,
                evidence.republished_adoptions,
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
