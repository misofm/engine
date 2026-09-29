//! One-million-block allocation and forbidden-operation audit of the plan exchange lifecycle.
//!
//! The plans are compiled the way every host compiles one (#963): a one-track session through
//! [`GraphCompiler::compile_with_builtins`] at the build's own [`Backend::current`], with the
//! track's input and the session output bound to host processors and the track's builtin strip
//! attached as banks. Each block is one frame, so the million renders are a million block
//! boundaries at which a swap may land.

use bench_support::alloc as bench_alloc;
use core::num::NonZeroUsize;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread::ThreadId,
};

use builtins_compiler::{BuiltinCompileCaps, prepare_session_builtins};
use effect_compiler::EffectPreparedSession;
use engine::realtime::audit;
use engine::realtime::{
    PlanExchangeConfig, PlanarBufferMut, PublishError, RealtimePlanOwner, RealtimeRenderReport,
    RenderError, RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use graph::{
    GraphBindingBlock, GraphCompileCaps, GraphNodeBinding, GraphNodeId, GraphRuntimeBindings,
    GraphRuntimeProcessor, TrackStage,
};
use graph_compiler::{Backend, GraphBuiltinsCompileRequest, GraphCompiler};
use session::{CompileCaps, compile_session, parse_session_json};

/// Frames per block: the audit's block boundaries are its subject, so each block is one frame.
const QUANTUM: u32 = 1;

type DropRecords = Arc<Mutex<Vec<(u64, ThreadId)>>>;

struct Silence {
    plan_id: u64,
    drops: Option<DropRecords>,
}
impl GraphRuntimeProcessor for Silence {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(0.0);
        block.right.fill(0.0);
        Ok(())
    }
}
impl Drop for Silence {
    fn drop(&mut self) {
        if let Some(drops) = &self.drops {
            drops
                .lock()
                .expect("drop record lock")
                .push((self.plan_id, std::thread::current().id()));
        }
    }
}

pub(crate) fn main() {
    // #104 F4: prove the shared audited allocator is the one serving this process. A global
    // allocator registered by a dependency that is never named may not be linked at all, and a
    // silently absent audit reports success for every gate below it.
    bench_alloc::assert_installed();
    let blocks = parse_blocks();
    assert!(
        blocks >= 3,
        "graph lifecycle audit requires at least 3 blocks"
    );
    let drops = Arc::new(Mutex::new(Vec::with_capacity(2)));
    let (mut publisher, mut owner, mut retirer) = plan_exchange(
        prepared_graph(6, Some(Arc::clone(&drops))),
        PlanExchangeConfig {
            publication_capacity: NonZeroUsize::new(1).expect("one"),
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("graph plan exchange");
    publisher
        .publish(prepared_graph(7, Some(Arc::clone(&drops))))
        .unwrap_or_else(|_| panic!("first graph replacement must publish"));

    enum RetirementCommand {
        ReclaimOne,
        Stop,
    }
    let (command_sender, command_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();
    let retirement_thread = std::thread::spawn(move || {
        loop {
            match command_receiver.recv().expect("retirement command") {
                RetirementCommand::ReclaimOne => {
                    let (epoch, plan) = retirer.try_reclaim().expect("retired graph plan");
                    drop(plan);
                    result_sender.send(epoch).expect("retirement result");
                }
                RetirementCommand::Stop => return std::thread::current().id(),
            }
        }
    });

    let mut output = [1.0_f32; 2 * QUANTUM as usize];
    let output_address = output.as_ptr() as usize;
    let mut swaps_accepted = 0_u64;
    let mut swaps_deferred = 0_u64;
    audit::warm_up();
    audit::reset();

    eprintln!("MISO_ENGINE_GRAPH_RT_BEGIN");
    let first = render_graph_block(&mut owner, &mut output, 0);
    eprintln!("MISO_ENGINE_GRAPH_RT_END");
    assert_eq!(first.swap, SwapOutcome::Applied);
    assert_eq!(first.render.plan_id, 7);
    swaps_accepted += 1;

    publisher
        .publish(prepared_graph(8, None))
        .unwrap_or_else(|error| match error {
            PublishError::Full(_) => panic!("publication queue unexpectedly full"),
            PublishError::Incompatible(_) => panic!("replacement envelope mismatch"),
            PublishError::EpochExhausted(_) => panic!("replacement epoch exhausted"),
        });
    eprintln!("MISO_ENGINE_GRAPH_RT_BEGIN");
    let deferred = render_graph_block(&mut owner, &mut output, 1);
    eprintln!("MISO_ENGINE_GRAPH_RT_END");
    assert_eq!(deferred.swap, SwapOutcome::DeferredRetirementFull);
    assert_eq!(deferred.render.plan_id, 7);
    swaps_deferred += 1;

    command_sender
        .send(RetirementCommand::ReclaimOne)
        .expect("request first retirement");
    assert_eq!(
        result_receiver.recv().expect("first retirement result").0,
        0
    );

    eprintln!("MISO_ENGINE_GRAPH_RT_BEGIN");
    for block in 2..blocks {
        let report = render_graph_block(&mut owner, &mut output, block);
        if block == 2 {
            assert_eq!(report.swap, SwapOutcome::Applied);
            swaps_accepted += 1;
        } else {
            assert_eq!(report.swap, SwapOutcome::None);
        }
        assert_eq!(report.render.plan_id, 8);
    }
    eprintln!("MISO_ENGINE_GRAPH_RT_END");

    let snapshot = audit::snapshot();
    command_sender
        .send(RetirementCommand::ReclaimOne)
        .expect("request second retirement");
    assert_eq!(
        result_receiver.recv().expect("second retirement result").0,
        1
    );
    command_sender
        .send(RetirementCommand::Stop)
        .expect("stop retirement thread");
    let retirement_thread_id = retirement_thread.join().expect("retirement thread");
    let drop_records = drops.lock().expect("drop records");
    assert_eq!(drop_records.len(), 2);
    assert_eq!(drop_records[0], (6, retirement_thread_id));
    assert_eq!(drop_records[1], (7, retirement_thread_id));
    assert_eq!(swaps_accepted, 2);
    assert_eq!(swaps_deferred, 1);
    assert_eq!(output, [0.0; 2 * QUANTUM as usize]);
    assert_eq!(output.as_ptr() as usize, output_address);
    assert_eq!(snapshot.total(), 0);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"graph_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":{},",
            "\"swaps_accepted\":{},\"swaps_deferred\":{},",
            "\"displaced_plans_destroyed_off_render\":{},\"output_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},",
            "\"logs\":{},\"file_io\":{},\"network_io\":{},",
            "\"syscalls\":{},\"total_violations\":{}}}"
        ),
        blocks,
        QUANTUM,
        swaps_accepted,
        swaps_deferred,
        drop_records.len(),
        output_address,
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

fn render_graph_block(
    owner: &mut RealtimePlanOwner,
    output: &mut [f32; 2 * QUANTUM as usize],
    block: u64,
) -> RealtimeRenderReport {
    let frames = QUANTUM as usize;
    let output_view = PlanarBufferMut::try_new(output, 2, frames, frames).expect("fixed output");
    owner
        .render(
            RenderIo {
                output: output_view,
            },
            RenderTime {
                absolute_sample: block * u64::from(QUANTUM),
            },
        )
        .expect("graph render")
}

fn parse_blocks() -> u64 {
    let mut arguments = std::env::args().skip(1);
    match arguments.next().as_deref() {
        None => 1_000_000,
        Some("--blocks") => arguments
            .next()
            .expect("--blocks value")
            .parse()
            .expect("integer block count"),
        Some(argument) => panic!("unknown argument: {argument}"),
    }
}

fn prepared_graph(
    plan_id: u64,
    drop_records: Option<DropRecords>,
) -> engine::realtime::PreparedRenderPlan {
    let mut model = parse_session_json(include_str!("../../../fixtures/session/v1/canonical.json"))
        .expect("canonical session");
    model.quantum_frames = QUANTUM;
    model.tracks[0].inserts.effects.clear();
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
    let builtins = prepare_session_builtins(
        &session,
        &[],
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
    let dispatch = Backend::current();
    let artifact = GraphCompiler::compile_with_builtins(GraphBuiltinsCompileRequest {
        dispatch,
        plan_id,
        effects: EffectPreparedSession {
            session,
            entries: Vec::new(),
        },
        builtins,
        caps: GraphCompileCaps {
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
    .unwrap_or_else(|failure| panic!("graph compile: {:?}", failure.diagnostics));
    // The strip renders in banks at a SIMD width, as on every host; at scalar width no bank
    // attaches.
    let banks = if dispatch.width() > 1 { 3 } else { 0 };
    assert_eq!(artifact.prepared_builtin_bank_count(), banks);
    let envelope = artifact.envelope();
    assert_eq!(envelope.quantum.0, QUANTUM);
    let mut drop_records = drop_records;
    let nodes: Vec<_> = artifact
        .external_binding_nodes()
        .cloned()
        .map(|node| match node {
            // The displaced plans' destruction is witnessed by the input processor's drop.
            GraphNodeId::TrackStage {
                stage: TrackStage::Input,
                ..
            } => GraphNodeBinding::new(
                node,
                Box::new(Silence {
                    plan_id,
                    drops: drop_records.take(),
                }) as Box<dyn GraphRuntimeProcessor>,
            ),
            GraphNodeId::Output { .. } => GraphNodeBinding::new(
                node,
                Box::new(Silence {
                    plan_id,
                    drops: None,
                }) as Box<dyn GraphRuntimeProcessor>,
            ),
            _ => panic!("only the track input and the session output are external"),
        })
        .collect();
    assert_eq!(nodes.len(), 2, "one track input and the session output");
    artifact
        .into_bound(GraphRuntimeBindings {
            envelope,
            nodes,
            observers: Vec::new(),
        })
        .unwrap_or_else(|failure| panic!("graph bind: {}", failure.code))
        .plan
}
