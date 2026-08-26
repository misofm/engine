//! Issue-007 graph-backed, one-million-render realtime and lifecycle audit.
//!
//! This is deliberately separate from the scalar-kernel audit. It exercises the production
//! session/effect/builtin/graph lowering path, a `RealtimePlanOwner`, and the bounded retirement
//! exchange while all allocation and forbidden-operation hooks are armed by the render entrypoint.

use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
use miso_engine_bench_support::alloc as bench_alloc;
use miso_engine_graph_compiler::Backend;
use std::{
    collections::BTreeSet,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread::ThreadId,
};

use miso_engine_builtins::{MeterConfig, MeterHandle, MeterSnapshot, MeterTap};
use miso_engine_builtins_compiler::{
    BuiltinCompileCaps, MeterConsumer, MeterRequest, prepare_session_builtins,
};
use miso_engine_conformance::DualAccumulatorDelayFactory;
use miso_engine_core::realtime::audit::{self, ForbiddenOperation};
use miso_engine_core::realtime::{
    Consumer, PlanExchangeConfig, PlanRetirer, PlanarBufferMut, QueueGeneration, RealtimePlanOwner,
    RealtimeRenderReport, RenderError, RenderIo, RenderTime, SwapOutcome, bounded_spsc_move,
    plan_exchange,
};
use miso_engine_effect_compiler::{EffectCompileCaps, prepare_native_session_effects};
use miso_engine_effect_contract::{NativeEffectFactory, NativeEffectRegistry};
use miso_engine_graph::{
    GraphBindingBlock, GraphEdgeId, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, PreparedGraphPlan, TrackStage,
};
use miso_engine_graph_compiler::{GraphBuiltinsCompileRequest, GraphCompiler};
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
const ACCEPTED_MANIFEST_SHA256: &str =
    "ddb4b201dcd4cc00ad445013c9a1b29d9d5f6071f018e649748963c74af4c55b";
const ACCEPTED_GRAPH_PCM_SHA256: &str =
    "508c8e94244b99ae1ee59e4863088ba69c6462127eb0256f85ec72e775a17a19";
const ACCEPTED_GRAPH_METERS_SHA256: &str =
    "958a702612b76353ae2dbb0f8a03a2e41aafbd90ed72857bc0c39a10b5d1935f";
const ACCEPTED_GRAPH_PCM: &[u8] =
    include_bytes!("../../../fixtures/builtins/v1/pcm/graph-taps.f32le");
const ACCEPTED_GRAPH_METERS: &str =
    include_str!("../../../fixtures/builtins/v1/meters/graph-taps.jsonl");

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

#[derive(Debug)]
enum RetirementCommand {
    Reclaim,
}

/// Run only the audit-local, off-render retirement ownership handoff.
///
/// Before control disarms the graph markers this loop can reach only the move-SPSC poll, atomic
/// loads/stores, and a processor spin hint. Reclamation, destruction, and thread exit occur only
/// after control sends its sole command outside that armed lifetime.
fn run_retirement_worker(
    mut commands: Consumer<RetirementCommand>,
    mut retirer: PlanRetirer,
    ready: &AtomicBool,
    reclaimed_epoch_plus_one: &AtomicU64,
    stop: &AtomicBool,
) -> ThreadId {
    ready.store(true, Ordering::Release);
    loop {
        if stop.load(Ordering::Acquire) {
            return std::thread::current().id();
        }
        match commands.try_pop() {
            Ok(RetirementCommand::Reclaim) => {
                let (epoch, plan) = retirer.try_reclaim().expect("retired A is available");
                assert_eq!(epoch.0, 0, "only epoch-zero A may retire here");
                drop(plan);
                reclaimed_epoch_plus_one.store(
                    epoch.0.checked_add(1).expect("epoch plus one"),
                    Ordering::Release,
                );
            }
            Err(_) => core::hint::spin_loop(),
        }
    }
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
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
    let drops = Arc::new(Mutex::new(Vec::with_capacity(3)));
    let control_thread_id = std::thread::current().id();
    let (initial, mut initial_meters) = prepare_graph_plan(PLAN_A, Some(Arc::clone(&drops)));
    let (applied, mut applied_meters) = prepare_graph_plan(PLAN_B, Some(Arc::clone(&drops)));
    let (deferred, _deferred_meters) = prepare_graph_plan(PLAN_C, Some(Arc::clone(&drops)));
    let (mut publisher, mut owner, retirer) = plan_exchange(
        initial,
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("publication capacity"),
            retirement_capacity: NonZeroUsize::new(1).expect("retirement capacity"),
        },
    )
    .expect("plan exchange");
    let (mut command_sender, command_receiver) = bounded_spsc_move(
        NonZeroUsize::new(1).expect("one reclaim command"),
        QueueGeneration(70),
    )
    .expect("prepare reclaim command queue");
    let ready = AtomicBool::new(false);
    let reclaimed_epoch_plus_one = AtomicU64::new(0);
    let stop = AtomicBool::new(false);
    let (retirement_thread_id, audit) = std::thread::scope(|scope| {
        let ready_ref = &ready;
        let reclaimed_ref = &reclaimed_epoch_plus_one;
        let stop_ref = &stop;
        let retirement_thread = scope.spawn(move || {
            run_retirement_worker(
                command_receiver,
                retirer,
                ready_ref,
                reclaimed_ref,
                stop_ref,
            )
        });
        while !ready.load(Ordering::Acquire) {
            core::hint::spin_loop();
        }

        let mut output = [0.0_f32; QUANTUM * 2];
        let left_address = output.as_ptr() as usize;
        let right_address = output[QUANTUM..].as_ptr() as usize;
        audit::warm_up();
        audit::reset();

        let first = traced_render(&mut owner, &mut output, 0);
        assert_eq!(first.swap, SwapOutcome::None);
        assert_eq!(first.render.plan_id, PLAN_A);
        assert_pcm_fixture(&output);
        let first_meter_values =
            drain_fixture(&mut initial_meters, ACCEPTED_GRAPH_METERS, "A fixture");
        let first_epoch = match publisher.publish(applied) {
            Ok(epoch) => epoch,
            Err(_) => panic!("B publication"),
        };
        assert_eq!(first_epoch.0, 1);
        let second = traced_render(&mut owner, &mut output, 1);
        assert_applied(&second, PLAN_B, 1);
        assert_pcm_fixture(&output);
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
        let second_meter_values = drain_values(&mut applied_meters, "B first window");
        assert_eq!(second_meter_values, first_meter_values);

        assert_eq!(owner.deferred_count(), BLOCKS - 2);
        assert_eq!(output.as_ptr() as usize, left_address);
        assert_eq!(output[QUANTUM..].as_ptr() as usize, right_address);
        let audit = audit::snapshot();
        assert_eq!(audit.total(), 0);

        command_sender
            .try_push(RetirementCommand::Reclaim)
            .expect("one empty reclaim command slot");
        while reclaimed_epoch_plus_one.load(Ordering::Acquire) == 0 {
            core::hint::spin_loop();
        }
        assert_eq!(reclaimed_epoch_plus_one.load(Ordering::Acquire), 1);
        assert_eq!(command_sender.success_count(), 1);
        stop.store(true, Ordering::Release);
        (retirement_thread.join().expect("retirement owner"), audit)
    });
    drop(owner);
    let drops = drops.lock().expect("drop records");
    assert_eq!(drops.len(), 3);
    assert_eq!(
        drops
            .iter()
            .filter(|row| **row == (PLAN_A, retirement_thread_id))
            .count(),
        1
    );
    assert_eq!(
        drops
            .iter()
            .filter(|row| **row == (PLAN_B, control_thread_id))
            .count(),
        1
    );
    assert_eq!(
        drops
            .iter()
            .filter(|row| **row == (PLAN_C, control_thread_id))
            .count(),
        1
    );

    let queue_success_windows = 2 * OBSERVERS as u64;
    let queue_full_windows = (BLOCKS - 2) * OBSERVERS as u64;
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"issue069_graph_realtime_lifecycle_audit\",",
            "\"renders\":{},\"sample_rate_hz\":48000,\"quantum_frames\":{},\"observers\":{},",
            "\"render_count_by_plan\":{{\"A\":1,\"B\":999999,\"C\":0}},",
            "\"swaps_applied\":1,\"swaps_deferred\":999998,",
            "\"prior_plan_renders_on_deferred\":999998,\"drained_blocks\":2,",
            "\"observer_windows_per_drained_block\":7,",
            "\"queue_success_windows\":{},\"queue_full_windows\":{},",
            "\"pdc_samples\":9,\"distinct_taps\":7,",
            "\"accepted_manifest_sha256\":\"{}\",\"accepted_graph_pcm_sha256\":\"{}\",",
            "\"accepted_graph_meters_sha256\":\"{}\",",
            "\"retirement_owner_destroyed\":1,\"control_owner_destroyed\":2,",
            "\"render_owner_destroyed\":0,\"stable_left_address\":true,",
            "\"stable_right_address\":true,",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},\"feature_detection\":{},\"logs\":{},",
            "\"file_io\":{},\"network_io\":{},\"syscalls\":{},\"panic_unwinds\":{},\"total_violations\":{}}}"
        ),
        BLOCKS,
        QUANTUM,
        OBSERVERS,
        queue_success_windows,
        queue_full_windows,
        ACCEPTED_MANIFEST_SHA256,
        ACCEPTED_GRAPH_PCM_SHA256,
        ACCEPTED_GRAPH_METERS_SHA256,
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
    eprintln!("MISO_ENGINE_BUILTINS_GRAPH_RT_BEGIN");
    for block in first..end_exclusive {
        let report = render(owner, output, block);
        assert_eq!(report.swap, SwapOutcome::DeferredRetirementFull);
        assert_eq!(report.active_epoch.0, 1);
        assert_eq!(report.render.plan_id, plan_id);
    }
    eprintln!("MISO_ENGINE_BUILTINS_GRAPH_RT_END");
}

fn traced_render(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; QUANTUM * 2],
    block: u64,
) -> RealtimeRenderReport {
    eprintln!("MISO_ENGINE_BUILTINS_GRAPH_RT_BEGIN");
    let report = render(owner, output, block);
    eprintln!("MISO_ENGINE_BUILTINS_GRAPH_RT_END");
    report
}

fn render(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; QUANTUM * 2],
    block: u64,
) -> RealtimeRenderReport {
    audit::in_render_scope(|| {
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
    })
}

fn drain_values(meters: &mut [MeterConsumer], label: &str) -> Vec<String> {
    assert_eq!(meters.len(), OBSERVERS, "{label}: all seven consumers");
    let mut values = Vec::with_capacity(OBSERVERS);
    for meter in meters {
        let snapshot = meter.consumer.try_pop().expect("one observer window");
        assert_eq!(snapshot.frames as usize, QUANTUM, "{label}: quantum window");
        assert!(
            meter.consumer.try_pop().is_err(),
            "{label}: exactly one window"
        );
        values.push(meter_value_row(meter.tap, snapshot));
    }
    values.sort();
    assert_eq!(values.iter().collect::<BTreeSet<_>>().len(), OBSERVERS);
    values
}

fn drain_fixture(meters: &mut [MeterConsumer], expected: &str, label: &str) -> Vec<String> {
    assert_eq!(meters.len(), OBSERVERS, "{label}: all seven consumers");
    let mut records = Vec::with_capacity(OBSERVERS);
    let mut values = Vec::with_capacity(OBSERVERS);
    for meter in meters {
        let snapshot = meter.consumer.try_pop().expect("one observer window");
        records.push(format!(
            "{{\"tap\":\"{:?}\",\"snapshot\":{}}}",
            meter.tap,
            meter_snapshot_json("graph-taps", snapshot)
        ));
        values.push(meter_value_row(meter.tap, snapshot));
        assert!(
            meter.consumer.try_pop().is_err(),
            "{label}: exactly one window"
        );
    }
    records.sort();
    values.sort();
    assert_eq!(records.join("\n") + "\n", expected);
    assert_eq!(values.iter().collect::<BTreeSet<_>>().len(), OBSERVERS);
    values
}

fn meter_value_row(tap: MeterTap, snapshot: MeterSnapshot) -> String {
    format!(
        "{:?}:{:016x}:{:08x}:{:08x}:{:08x}:{:08x}:{:016x}:{:016x}:{:016x}:{:016x}:{:016x}:{:016x}",
        tap,
        snapshot.handle.0.get(),
        snapshot.left.sample_peak.to_bits(),
        snapshot.right.sample_peak.to_bits(),
        snapshot.left.held_peak.to_bits(),
        snapshot.right.held_peak.to_bits(),
        snapshot.left.energy.to_bits(),
        snapshot.right.energy.to_bits(),
        snapshot.left.rms.to_bits(),
        snapshot.right.rms.to_bits(),
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
    )
}

fn meter_snapshot_json(case: &str, snapshot: MeterSnapshot) -> String {
    format!(
        "{{\"case\":\"{case}\",\"handle\":\"{:016x}\",\"reset_generation\":\"{:016x}\",\"sequence\":\"{:016x}\",\"start\":\"{:016x}\",\"end\":\"{:016x}\",\"frames\":{},\"left_peak\":\"{:08x}\",\"right_peak\":\"{:08x}\",\"left_held_peak\":\"{:08x}\",\"right_held_peak\":\"{:08x}\",\"left_energy\":\"{:016x}\",\"right_energy\":\"{:016x}\",\"left_rms\":\"{:016x}\",\"right_rms\":\"{:016x}\",\"clipped\":\"{:016x}\",\"sanitized\":\"{:016x}\",\"dropped\":\"{:016x}\",\"discontinuities\":\"{:016x}\"}}",
        snapshot.handle.0.get(),
        snapshot.reset_generation,
        snapshot.window_sequence,
        snapshot.start_sample,
        snapshot.end_sample,
        snapshot.frames,
        snapshot.left.sample_peak.to_bits(),
        snapshot.right.sample_peak.to_bits(),
        snapshot.left.held_peak.to_bits(),
        snapshot.right.held_peak.to_bits(),
        snapshot.left.energy.to_bits(),
        snapshot.right.energy.to_bits(),
        snapshot.left.rms.to_bits(),
        snapshot.right.rms.to_bits(),
        snapshot.cumulative_clipped_samples,
        snapshot.cumulative_sanitized_samples,
        snapshot.cumulative_dropped_snapshots,
        snapshot.cumulative_discontinuities,
    )
}

fn assert_pcm_fixture(output: &[f32; QUANTUM * 2]) {
    assert_eq!(
        ACCEPTED_GRAPH_PCM.len(),
        output.len() * core::mem::size_of::<f32>()
    );
    for (sample, expected) in output.iter().zip(ACCEPTED_GRAPH_PCM.chunks_exact(4)) {
        assert_eq!(
            sample.to_bits(),
            u32::from_le_bytes(expected.try_into().expect("word"))
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
        reset_generation: 0,
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
            NonZeroU64::new(u64::try_from(index).expect("bounded") + 1).expect("nonzero"),
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
        dispatch: Backend::current(),
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
    assert_graph_fixture_pdc(artifact.graph());
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|_| panic!("graph bind"));
    (bound.plan, bound.meter_consumers)
}

fn assert_graph_fixture_pdc(graph: &PreparedGraphPlan) {
    let timing = |id: &str| {
        graph
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
    let delays: Vec<_> = graph
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

#[cfg(test)]
mod tests {
    use super::*;

    fn render_prepared(
        plan: &mut miso_engine_core::realtime::PreparedRenderPlan,
        block: u64,
    ) -> [f32; QUANTUM * 2] {
        let mut output = [0.0; QUANTUM * 2];
        plan.render(
            RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut output, 2, QUANTUM, QUANTUM)
                    .expect("fixed output"),
            },
            RenderTime {
                absolute_sample: block * QUANTUM as u64,
            },
        )
        .expect("render graph proof");
        output
    }

    fn pop_rows(meters: &mut [MeterConsumer]) -> (Vec<String>, Vec<u64>) {
        let mut rows = Vec::with_capacity(OBSERVERS);
        let mut drops = Vec::with_capacity(OBSERVERS);
        for meter in meters {
            let snapshot = meter.consumer.try_pop().expect("meter window");
            rows.push(meter_value_row(meter.tap, snapshot));
            drops.push(snapshot.cumulative_dropped_snapshots);
            assert!(meter.consumer.try_pop().is_err());
        }
        rows.sort();
        (rows, drops)
    }

    #[test]
    fn issue069_two_graph_instances_prove_success_and_saturation_without_duplicates() {
        let (mut success, mut success_meters) = prepare_graph_plan(70, None);
        let (mut saturation, mut saturation_meters) = prepare_graph_plan(71, None);

        let success_first = render_prepared(&mut success, 0);
        let (success_first_rows, success_first_drops) = pop_rows(&mut success_meters);
        assert_eq!(success_first_drops, [0; OBSERVERS]);
        let _ = render_prepared(&mut success, 1);
        let _ = pop_rows(&mut success_meters);
        let success_continuation = render_prepared(&mut success, 2);
        let _ = pop_rows(&mut success_meters);

        let saturation_first = render_prepared(&mut saturation, 0);
        let _saturated = render_prepared(&mut saturation, 1);
        let (saturation_first_rows, saturation_first_drops) = pop_rows(&mut saturation_meters);
        assert_eq!(saturation_first_drops, [0; OBSERVERS]);
        let saturation_continuation = render_prepared(&mut saturation, 2);
        let (post_full_rows, post_full_drops) = pop_rows(&mut saturation_meters);

        assert_eq!(success_first, saturation_first);
        assert_eq!(success_first_rows, saturation_first_rows);
        assert_eq!(post_full_drops, [1; OBSERVERS]);
        assert_eq!(post_full_rows.len(), OBSERVERS);
        assert_eq!(success_continuation, saturation_continuation);
        assert!(
            success_first[..9]
                .iter()
                .all(|sample| sample.to_bits() == 0)
        );
        assert_ne!(success_first[9].to_bits(), 0);
    }

    #[test]
    fn issue070_retirement_worker_is_ready_quiescent_and_owns_only_a() {
        let drops = Arc::new(Mutex::new(Vec::with_capacity(3)));
        let control_thread_id = std::thread::current().id();
        let (initial, _initial_meters) = prepare_graph_plan(PLAN_A, Some(Arc::clone(&drops)));
        let (applied, _applied_meters) = prepare_graph_plan(PLAN_B, Some(Arc::clone(&drops)));
        let (deferred, _deferred_meters) = prepare_graph_plan(PLAN_C, Some(Arc::clone(&drops)));
        let (mut publisher, mut owner, retirer) = plan_exchange(
            initial,
            PlanExchangeConfig {
                publication_capacity: NonZeroUsize::new(1).expect("one publication"),
                retirement_capacity: NonZeroUsize::new(1).expect("one retirement"),
            },
        )
        .expect("exchange");
        let (mut command_sender, command_receiver) = bounded_spsc_move(
            NonZeroUsize::new(1).expect("one reclaim command"),
            QueueGeneration(70),
        )
        .expect("command queue");
        let ready = AtomicBool::new(false);
        let reclaimed_epoch_plus_one = AtomicU64::new(0);
        let stop = AtomicBool::new(false);
        let retirement_thread_id = std::thread::scope(|scope| {
            let ready_ref = &ready;
            let reclaimed_ref = &reclaimed_epoch_plus_one;
            let stop_ref = &stop;
            let retirement_thread = scope.spawn(move || {
                run_retirement_worker(
                    command_receiver,
                    retirer,
                    ready_ref,
                    reclaimed_ref,
                    stop_ref,
                )
            });
            while !ready.load(Ordering::Acquire) {
                core::hint::spin_loop();
            }

            let b_epoch = match publisher.publish(applied) {
                Ok(epoch) => epoch,
                Err(_) => panic!("publish B"),
            };
            assert_eq!(b_epoch.0, 1);
            let mut output = [0.0_f32; QUANTUM * 2];
            assert_applied(&render(&mut owner, &mut output, 0), PLAN_B, 1);
            let c_epoch = match publisher.publish(deferred) {
                Ok(epoch) => epoch,
                Err(_) => panic!("publish C"),
            };
            assert_eq!(c_epoch.0, 2);
            assert_eq!(
                render(&mut owner, &mut output, 1).swap,
                SwapOutcome::DeferredRetirementFull
            );
            assert_eq!(owner.deferred_count(), 1);

            command_sender
                .try_push(RetirementCommand::Reclaim)
                .expect("one reclaim command");
            while reclaimed_epoch_plus_one.load(Ordering::Acquire) == 0 {
                core::hint::spin_loop();
            }
            assert_eq!(reclaimed_epoch_plus_one.load(Ordering::Acquire), 1);
            assert_eq!(command_sender.success_count(), 1);
            stop.store(true, Ordering::Release);
            retirement_thread.join().expect("retirement worker")
        });
        drop(owner);
        let drops = drops.lock().expect("drop records");
        assert_eq!(drops.len(), 3);
        for (plan, expected_thread) in [
            (PLAN_A, retirement_thread_id),
            (PLAN_B, control_thread_id),
            (PLAN_C, control_thread_id),
        ] {
            assert_eq!(
                drops
                    .iter()
                    .filter(|row| **row == (plan, expected_thread))
                    .count(),
                1,
                "plan {plan} has one exact destruction role"
            );
        }
    }

    #[test]
    fn issue070_retirement_worker_source_is_limited_to_nonblocking_primitives() {
        let source = include_str!("builtins_graph.rs");
        let (_, worker) = source
            .split_once("fn run_retirement_worker")
            .expect("worker source");
        let (worker, _) = worker
            .split_once("\npub(crate) fn main")
            .expect("worker boundary");
        assert!(worker.contains(concat!("spin", "_loop")));
        assert!(source.contains(concat!("bounded_spsc", "_move")));
        for forbidden in [
            concat!("m", "psc"),
            concat!(".re", "cv("),
            concat!("pa", "rk"),
            concat!("yield", "_now"),
            concat!("sl", "eep"),
        ] {
            assert!(
                !worker.contains(forbidden),
                "worker must not contain {forbidden}"
            );
        }
    }
}
