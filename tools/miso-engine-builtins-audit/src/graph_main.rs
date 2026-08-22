//! Issue-007 graph-backed, one-million-render realtime and lifecycle audit.
//!
//! This is deliberately separate from the scalar-kernel audit. It exercises the production
//! session/effect/builtin/graph lowering path, a `RealtimePlanOwner`, and the bounded retirement
//! exchange while all allocation and forbidden-operation hooks are armed by the render entrypoint.

#![allow(unsafe_code)]

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use std::{
    alloc::{GlobalAlloc, Layout, System},
    sync::{Arc, Mutex, mpsc},
    thread::ThreadId,
};

use miso_engine_builtins::{MeterConfig, MeterHandle, MeterTap};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, prepare_session_builtins,
};
use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, RealtimePlanOwner, RealtimeRenderReport, RenderError,
    RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use miso_engine_effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
use miso_engine_effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use miso_engine_graph::{
    GraphBindingBlock, GraphEdgeId, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompileReport, GraphCompiler};
use miso_engine_session::{
    ChannelMatrix, CompileCaps, EffectIdentity, RouteSource, SendTap, StableId, compile_session,
    parse_session_toml,
};

const BLOCKS: u64 = 1_000_000;
const QUANTUM: usize = 128;
const OBSERVERS: usize = 7;
const PLAN_A: u64 = 1;
const PLAN_B: u64 = 2;
const PLAN_C: u64 = 3;

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
        for (index, (left, right)) in block
            .left
            .iter_mut()
            .zip(block.right.iter_mut())
            .enumerate()
        {
            (*left, *right) = if index == 0 {
                (1.0, -0.5)
            } else {
                (0.125 + index as f32 * 0.001, -0.25 - index as f32 * 0.002)
            };
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
    let (initial, _initial_meters) = prepare_graph_plan(PLAN_A, Some(Arc::clone(&drops)));
    let (applied, mut applied_meters) = prepare_graph_plan(PLAN_B, Some(Arc::clone(&drops)));
    let (deferred, _deferred_meters) = prepare_graph_plan(PLAN_C, Some(Arc::clone(&drops)));
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        initial,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("publication capacity"),
            retirement_capacity: NonZeroUsize::new(1).expect("retirement capacity"),
        },
    )
    .expect("plan exchange");
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
    assert_eq!(first.swap, SwapOutcome::None);
    assert_eq!(first.render.plan_id, PLAN_A);
    let first_epoch = match publisher.publish(applied) {
        Ok(epoch) => epoch,
        Err(_) => panic!("B publication"),
    };
    assert_eq!(first_epoch.0, 1);
    let second = traced_render(&mut owner, &mut output, 1);
    assert_applied(&second, PLAN_B, 1);
    let second_epoch = match publisher.publish(deferred) {
        Ok(epoch) => epoch,
        Err(_) => panic!("C publication"),
    };
    assert_eq!(second_epoch.0, 2);
    let third = traced_render(&mut owner, &mut output, 2);
    assert_eq!(third.swap, SwapOutcome::DeferredRetirementFull);
    assert_eq!(third.active_epoch.0, 1);
    assert_eq!(third.render.plan_id, PLAN_B);
    traced_range(&mut owner, &mut output, 3, BLOCKS, PLAN_B);
    drain_exact(&mut applied_meters, BLOCKS - 2, "B final");

    assert_eq!(owner.deferred_count(), 1);
    assert_eq!(output.as_ptr() as usize, left_address);
    assert_eq!(output[QUANTUM..].as_ptr() as usize, right_address);
    let audit = audit::snapshot();
    assert_eq!(audit.total(), 0);

    command_sender
        .send(RetirementCommand::Reclaim)
        .expect("reclaim A");
    assert_eq!(result_receiver.recv().expect("A epoch").0, 0);
    command_sender
        .send(RetirementCommand::Stop)
        .expect("stop retirement owner");
    let retirement_thread_id = retirement_thread.join().expect("retirement owner");
    drop(owner);
    let drops = drops.lock().expect("drop records");
    assert!(drops.contains(&(PLAN_A, retirement_thread_id)));
    assert!(
        drops
            .iter()
            .any(|(plan, thread)| *plan == PLAN_B && *thread != retirement_thread_id)
    );
    assert!(
        drops
            .iter()
            .any(|(plan, thread)| *plan == PLAN_C && *thread != retirement_thread_id)
    );

    let queue_success_windows = OBSERVERS as u64;
    let queue_full_windows = (BLOCKS - 2) * OBSERVERS as u64;
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue057_graph_realtime_lifecycle_audit\",",
            "\"renders\":{},\"sample_rate_hz\":48000,\"quantum_frames\":{},\"observers\":{},",
            "\"render_count_by_plan\":{{\"A\":1,\"B\":999999,\"C\":0}},",
            "\"swaps_applied\":1,\"swaps_deferred\":1,",
            "\"prior_plan_renders_on_deferred\":1,\"drained_blocks\":1,",
            "\"observer_windows_per_drained_block\":7,",
            "\"queue_success_windows\":{},\"queue_full_windows\":{},",
            "\"retired_destroyed_off_render\":1,\"left_address\":{},\"right_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"feature_detection\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"panic_unwinds\":{},\"total_violations\":{}}}"
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
        audit.feature_detection,
        audit.logs,
        audit.file_io,
        audit.network_io,
        audit.syscalls,
        audit.panic_unwinds,
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
    let mut fixture_effect = model.tracks[0].dynamic.effects[0].clone();
    fixture_effect.identity = EffectIdentity::Native {
        effect_id: StableId::parse("conformance.delay").expect("fixture effect ID"),
    };
    fixture_effect.params.clear();
    fixture_effect.id = StableId::parse("fixture-simd1").expect("fixture effect ID");
    model.tracks[0].simd1.effects = vec![fixture_effect.clone()];
    fixture_effect.id = StableId::parse("fixture-dynamic").expect("fixture effect ID");
    model.tracks[0].dynamic.effects = vec![fixture_effect.clone()];
    fixture_effect.id = StableId::parse("fixture-simd2").expect("fixture effect ID");
    model.tracks[0].simd2.effects = vec![fixture_effect];
    model.tracks[0].fader.left_db = -6.0;
    model.tracks[0].fader.right_db = 3.0;
    model.automation.clear();
    let mut early = model.routes[0].clone();
    early.id = StableId::parse("to-main-early").expect("fixture route ID");
    early.source = RouteSource::Track {
        track_id: model.tracks[0].id.clone(),
        tap: SendTap::PostInputBuiltins,
    };
    early.channel_matrix = ChannelMatrix {
        ll: 0.25,
        lr: -0.5,
        rl: 0.75,
        rr: 0.125,
    };
    model.routes.push(early);
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
    .enumerate()
    .map(|(index, tap)| MeterRequest {
        handle: MeterHandle(
            NonZeroU64::new(
                plan_id
                    .checked_mul(100)
                    .and_then(|value| value.checked_add(u64::try_from(index).expect("bounded") + 1))
                    .expect("bounded plan meter handle"),
            )
            .expect("nonzero"),
        ),
        track_id: "vocal".to_owned(),
        tap,
        config,
    })
    .collect();
    let registry = NativeEffectRegistry::new([
        Box::new(DualAccumulatorDelayFactory::correct()) as Box<dyn NativeEffectFactory>
    ])
    .expect("fixture registry");
    let effects = prepare_native_session_effects(
        &session,
        &registry,
        EffectCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_scratch_bytes: u64::MAX,
            maximum_automation_spans_per_block: u32::MAX,
        },
    )
    .expect("fixture effects");
    let builtins = prepare_session_builtins(
        &effects.session,
        &requests,
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
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
        effects,
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
    assert_graph_fixture_pdc(artifact.report());
    assert_eq!(artifact.external_binding_nodes().count(), 2);
    let envelope = artifact.envelope();
    let nodes = artifact
        .external_binding_nodes()
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
    let bound = artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|_| panic!("graph bind"));
    (bound.plan, bound.meter_consumers)
}

fn assert_graph_fixture_pdc(report: &GraphCompileReport) {
    let timing = |id: &str| {
        report
            .route_timings
            .iter()
            .find(|row| row.route_id.as_str() == id)
            .unwrap_or_else(|| panic!("missing route timing: {id}"))
    };
    let late = timing("to-main");
    let early = timing("to-main-early");
    assert_eq!(
        (
            late.source_arrival.0,
            late.compensation_delay.0,
            late.destination_arrival.0
        ),
        (9, 0, 9)
    );
    assert_eq!(
        (
            early.source_arrival.0,
            early.compensation_delay.0,
            early.destination_arrival.0,
        ),
        (0, 9, 9)
    );
    let delays: Vec<_> = report
        .inserted_delays
        .iter()
        .filter(|row| {
            matches!(
                &row.edge_id,
                GraphEdgeId::RouteDestination { route_id } if route_id.as_str() == "to-main-early"
            )
        })
        .map(|row| row.samples.0)
        .collect();
    assert_eq!(delays, [9]);
}

fn run_probe(operation: ForbiddenOperation) -> ! {
    audit::warm_up();
    audit::reset();
    audit::in_render_scope(|| {
        if operation == ForbiddenOperation::PanicUnwind {
            panic!("deliberate panic/unwind detector probe");
        }
        audit::forbidden(operation);
    });
    panic!("forbidden-operation probe unexpectedly survived")
}

fn parse_operation(value: &str) -> ForbiddenOperation {
    match value {
        "allocation" => ForbiddenOperation::Allocation,
        "deallocation" => ForbiddenOperation::Deallocation,
        "lock" => ForbiddenOperation::Lock,
        "feature-detection" => ForbiddenOperation::FeatureDetection,
        "log" => ForbiddenOperation::Log,
        "file-io" => ForbiddenOperation::FileIo,
        "network-io" => ForbiddenOperation::NetworkIo,
        "syscall" => ForbiddenOperation::Syscall,
        "panic-unwind" => ForbiddenOperation::PanicUnwind,
        _ => panic!("unknown forbidden operation"),
    }
}
