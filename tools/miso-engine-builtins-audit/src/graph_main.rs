//! Issue-007 graph-backed, one-million-render realtime and lifecycle audit.
//!
//! This is deliberately separate from the scalar-kernel audit. It exercises the production
//! session/effect/builtin/graph lowering path, a `RealtimePlanOwner`, and the bounded retirement
//! exchange while all allocation and forbidden-operation hooks are armed by the render entrypoint.

#![allow(unsafe_code)]

use core::num::{NonZeroU32, NonZeroUsize};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::{Arc, Mutex, mpsc},
    thread::ThreadId,
};

use miso_engine_builtins::{MeterConfig, MeterTap};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, prepare_session_builtins,
};
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RealtimePlanOwner, RealtimeRenderReport, RenderError,
    RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use miso_engine_effect_compiler::EffectPreparedSession;
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings, GraphRuntimeProcessor,
    TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

const BLOCKS: u64 = 1_000_000;
const QUANTUM: usize = 128;
const OBSERVERS: usize = 7;
const PLAN_INITIAL: u64 = 6;
const PLAN_APPLIED: u64 = 7;
const PLAN_DEFERRED: u64 = 8;

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every operation forwards unchanged arguments to the system allocator. The armed path
// aborts rather than unwinding through `GlobalAlloc`.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forward the supplied valid layout unchanged.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forward the supplied valid layout unchanged.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: forward the original pointer/layout unchanged.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: forward the original pointer/layout and requested size unchanged.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

type DropRecords = Arc<Mutex<Vec<(u64, ThreadId)>>>;

struct ExternalSource {
    plan_id: u64,
    drops: Option<DropRecords>,
}

impl GraphRuntimeProcessor for ExternalSource {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let base = self.plan_id as f32 * 0.01;
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            let offset = index as f32 * 0.0001;
            *left = base + 0.125 + offset;
            *right = -(base + 0.25 + offset);
        }
        Ok(())
    }
}

impl Drop for ExternalSource {
    fn drop(&mut self) {
        if let Some(drops) = &self.drops {
            drops
                .lock()
                .expect("drop record lock")
                .push((self.plan_id, std::thread::current().id()));
        }
    }
}

struct ExternalOutput;

impl GraphRuntimeProcessor for ExternalOutput {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

enum RetirementCommand {
    Reclaim,
    Stop,
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        None => run_audit(),
        Some("--probe") => run_probe(
            std::env::args()
                .nth(2)
                .as_deref()
                .map(parse_operation)
                .expect("--probe requires a known operation"),
        ),
        Some(argument) => panic!("unknown graph audit argument: {argument}"),
    }
}

fn run_audit() {
    let drops = Arc::new(Mutex::new(Vec::with_capacity(2)));
    let (initial, _initial_meters) = prepare_graph_plan(PLAN_INITIAL, Some(Arc::clone(&drops)));
    let (applied, mut applied_meters) = prepare_graph_plan(PLAN_APPLIED, Some(Arc::clone(&drops)));
    let (deferred, mut deferred_meters) = prepare_graph_plan(PLAN_DEFERRED, None);
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        initial,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("publication capacity"),
            retirement_capacity: NonZeroUsize::new(1).expect("retirement capacity"),
        },
    )
    .expect("plan exchange");
    let first_epoch = match publisher.publish(applied) {
        Ok(epoch) => epoch,
        Err(_) => panic!("first publication"),
    };
    assert_eq!(first_epoch.0, 1);

    let (command_sender, command_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();
    let retirement_thread = std::thread::spawn(move || {
        loop {
            match command_receiver.recv().expect("retirement command") {
                RetirementCommand::Reclaim => {
                    let (epoch, plan) = retirer.try_reclaim().expect("retired plan");
                    drop(plan);
                    result_sender.send(epoch).expect("retirement result");
                }
                RetirementCommand::Stop => return std::thread::current().id(),
            }
        }
    });

    let mut output = [0.0_f32; QUANTUM * 2];
    let left_address = output.as_ptr() as usize;
    let right_address = output[QUANTUM..].as_ptr() as usize;
    audit::warm_up();
    audit::reset();

    let first = traced_render(&mut owner, &mut output, 0);
    assert_applied(&first, PLAN_APPLIED, 1);
    let second = traced_render(&mut owner, &mut output, 1);
    assert_eq!(second.swap, SwapOutcome::None);
    assert_eq!(second.render.plan_id, PLAN_APPLIED);
    drain_exact(&mut applied_meters, 0, "applied-first");

    let third = traced_render(&mut owner, &mut output, 2);
    assert_eq!(third.swap, SwapOutcome::None);
    assert_eq!(third.render.plan_id, PLAN_APPLIED);
    drain_exact(&mut applied_meters, 1, "applied-drop");

    let second_epoch = match publisher.publish(deferred) {
        Ok(epoch) => epoch,
        Err(_) => panic!("second publication"),
    };
    assert_eq!(second_epoch.0, 2);
    let fourth = traced_render(&mut owner, &mut output, 3);
    assert_eq!(fourth.swap, SwapOutcome::DeferredRetirementFull);
    assert_eq!(fourth.active_epoch.0, 1);
    assert_eq!(fourth.render.plan_id, PLAN_APPLIED);
    drain_exact(&mut applied_meters, 1, "deferred-prior-plan");

    command_sender
        .send(RetirementCommand::Reclaim)
        .expect("reclaim initial");
    assert_eq!(result_receiver.recv().expect("initial epoch").0, 0);

    let fifth = traced_render(&mut owner, &mut output, 4);
    assert_applied(&fifth, PLAN_DEFERRED, 2);
    drain_exact(&mut deferred_meters, 0, "deferred-first");

    traced_range(&mut owner, &mut output, 5, BLOCKS - 1, PLAN_DEFERRED);
    // The window emitted at block five is intentionally retained while the queue is full. Drain
    // it off render before the final block so that the final snapshot reports every exact drop.
    drain_exact(&mut deferred_meters, 0, "deferred-pre-final");
    let last = traced_render(&mut owner, &mut output, BLOCKS - 1);
    assert_eq!(last.swap, SwapOutcome::None);
    assert_eq!(last.render.plan_id, PLAN_DEFERRED);
    drain_exact(&mut deferred_meters, BLOCKS - 7, "deferred-final");

    assert_eq!(owner.deferred_count(), 1);
    assert_eq!(output.as_ptr() as usize, left_address);
    assert_eq!(output[QUANTUM..].as_ptr() as usize, right_address);
    let audit = audit::snapshot();
    assert_eq!(audit.total(), 0);

    command_sender
        .send(RetirementCommand::Reclaim)
        .expect("reclaim applied");
    assert_eq!(result_receiver.recv().expect("applied epoch").0, 1);
    command_sender
        .send(RetirementCommand::Stop)
        .expect("stop retirement owner");
    let retirement_thread_id = retirement_thread.join().expect("retirement owner");
    let drops = drops.lock().expect("drop records");
    assert_eq!(
        drops.as_slice(),
        &[
            (PLAN_INITIAL, retirement_thread_id),
            (PLAN_APPLIED, retirement_thread_id)
        ]
    );

    // Successes: three windows from plan seven and three from plan eight, each with seven taps.
    // Fulls: one on plan seven, then blocks 6..=999_998 on plan eight, each at seven taps.
    let queue_success_windows = 6_u64 * OBSERVERS as u64;
    let queue_full_windows = (1 + (BLOCKS - 7)) * OBSERVERS as u64;
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue007_graph_realtime_lifecycle_audit\",",
            "\"renders\":{},\"quantum_frames\":{},\"observers\":{},",
            "\"render_count_by_epoch\":{{\"1\":4,\"2\":999996}},",
            "\"swaps_applied\":2,\"swaps_deferred\":1,",
            "\"prior_plan_renders_on_deferred\":1,\"drained_blocks\":6,",
            "\"observer_windows_per_drained_block\":7,",
            "\"queue_success_windows\":{},\"queue_full_windows\":{},",
            "\"retired_destroyed_off_render\":2,\"left_address\":{},\"right_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"total_violations\":{}}}"
        ),
        BLOCKS,
        QUANTUM,
        OBSERVERS,
        queue_success_windows,
        queue_full_windows,
        left_address,
        right_address,
        audit.allocations,
        audit.deallocations,
        audit.locks,
        audit.logs,
        audit.file_io,
        audit.network_io,
        audit.syscalls,
        audit.total(),
    );
}

fn assert_applied(report: &RealtimeRenderReport, plan_id: u64, epoch: u64) {
    assert_eq!(report.swap, SwapOutcome::Applied);
    assert_eq!(report.render.plan_id, plan_id);
    assert_eq!(report.active_epoch.0, epoch);
}

fn traced_range(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; QUANTUM * 2],
    first: u64,
    end_exclusive: u64,
    plan_id: u64,
) {
    eprintln!("MISO_ISSUE007_GRAPH_RT_BEGIN");
    for block in first..end_exclusive {
        let report = render(owner, output, block);
        assert_eq!(report.swap, SwapOutcome::None);
        assert_eq!(report.render.plan_id, plan_id);
    }
    eprintln!("MISO_ISSUE007_GRAPH_RT_END");
}

fn traced_render(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; QUANTUM * 2],
    block: u64,
) -> RealtimeRenderReport {
    eprintln!("MISO_ISSUE007_GRAPH_RT_BEGIN");
    let report = render(owner, output, block);
    eprintln!("MISO_ISSUE007_GRAPH_RT_END");
    report
}

fn render(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; QUANTUM * 2],
    block: u64,
) -> RealtimeRenderReport {
    owner
        .render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(output, 2, QUANTUM, QUANTUM)
                    .expect("fixed planar output"),
            },
            RenderTime {
                absolute_sample: block.checked_mul(QUANTUM as u64).expect("audit time"),
            },
        )
        .expect("graph render")
}

fn drain_exact(meters: &mut [MeterConsumer], expected_dropped: u64, label: &str) {
    assert_eq!(meters.len(), OBSERVERS, "{label}: all seven consumers");
    for meter in meters {
        let snapshot = meter.consumer.try_pop().expect("one observer window");
        assert_eq!(snapshot.frames as usize, QUANTUM, "{label}: quantum window");
        assert_eq!(
            snapshot.cumulative_dropped_snapshots, expected_dropped,
            "{label}: exact drops"
        );
        assert!(
            meter.consumer.try_pop().is_err(),
            "{label}: exactly one window"
        );
    }
}

fn prepare_graph_plan(
    plan_id: u64,
    drops: Option<DropRecords>,
) -> (
    miso_engine_core::realtime::PreparedRenderPlan,
    Vec<MeterConsumer>,
) {
    let mut model = parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
        .expect("canonical session");
    model.tracks[0].simd1.effects.clear();
    model.tracks[0].dynamic.effects.clear();
    model.tracks[0].simd2.effects.clear();
    model.automation.clear();
    let session = compile_session(
        &model,
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        },
    )
    .expect("compiled session");
    let config = MeterConfig {
        period_frames: NonZeroU32::new(QUANTUM as u32).expect("quantum"),
        peak_hold_frames: 0,
        peak_decay_db_per_second: 0.0,
        queue_capacity: NonZeroUsize::new(1).expect("queue"),
        reset_generation: plan_id,
    };
    let requests: Vec<_> = [
        MeterTap::Input,
        MeterTap::PostInputBuiltins,
        MeterTap::PostSimd1,
        MeterTap::PostDynamic,
        MeterTap::PostSimd2PreFader,
        MeterTap::PostFader,
        MeterTap::PostMatrix,
    ]
    .into_iter()
    .map(|tap| MeterRequest {
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect();
    let builtins = prepare_session_builtins(
        &session,
        &requests,
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        },
    )
    .expect("sealed builtins");
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        plan_id,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: miso_engine_graph::GraphCompileCaps {
            maximum_nodes: 10_000,
            maximum_edges: 10_000,
            maximum_schedule_items: 10_000,
            maximum_dependency_levels: 10_000,
            maximum_audio_buffer_samples: 10_000_000,
            maximum_delay_samples_per_edge: 1_000_000,
            maximum_total_delay_samples: 10_000_000,
            maximum_graph_bytes: 10_000_000,
            maximum_plan_bytes: 100_000_000,
            maximum_single_allocation_bytes: 10_000_000,
            maximum_finite_tail_samples: 10_000_000,
        },
    })
    .unwrap_or_else(|_| panic!("graph compile"));
    assert_eq!(artifact.graph.required_bindings.len(), 2);
    let envelope = artifact.graph.envelope;
    let nodes = artifact
        .graph
        .required_bindings
        .iter()
        .cloned()
        .map(|node| match node {
            GraphNodeId::TrackStage {
                stage: TrackStage::Input,
                ..
            } => GraphNodeBinding::new(
                node,
                Box::new(ExternalSource {
                    plan_id,
                    drops: drops.clone(),
                }) as Box<dyn GraphRuntimeProcessor>,
            ),
            GraphNodeId::Output { .. } => GraphNodeBinding::new(
                node,
                Box::new(ExternalOutput) as Box<dyn GraphRuntimeProcessor>,
            ),
            _ => panic!("only external source and output may be bound"),
        })
        .collect();
    let meters = artifact.meter_consumers;
    let plan = artifact
        .graph
        .bind(GraphRuntimeBindings { envelope, nodes })
        .unwrap_or_else(|_| panic!("graph bind"));
    (plan, meters)
}

fn run_probe(operation: ForbiddenOperation) -> ! {
    audit::warm_up();
    audit::reset();
    audit::in_render_scope(|| audit::forbidden(operation));
    panic!("forbidden-operation probe unexpectedly survived")
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
        _ => panic!("unknown forbidden operation"),
    }
}
