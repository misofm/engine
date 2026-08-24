//! One-million-block allocation and forbidden-operation audit for a bound scalar graph.

use core::num::NonZeroUsize;
use miso_engine_bench_support::alloc as bench_alloc;
use std::{
    sync::{Arc, Mutex, mpsc},
    thread::ThreadId,
};

use miso_engine_core::realtime::audit;
use miso_engine_core::realtime::{
    PlanExchangeConfig, PlanarBufferMut, PublishError, RealtimePlanOwner, RealtimeRenderReport,
    RenderEnvelope, RenderError, RenderIo, RenderTime, SwapOutcome, plan_exchange,
};
use miso_engine_core::{QuantumFrames, SampleRateHz};
use miso_engine_effect_contract::{LatencySamples, TailSamples};
use miso_engine_graph::{
    GraphBindFailure, GraphBindingBlock, GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding,
    GraphNodeId, GraphPortId, GraphPortKind, GraphResourceEstimate, GraphRuntimeBindings,
    GraphRuntimeProcessor, GraphSpec, PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId,
    TrackStage,
};

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

    let mut output = [1.0_f32; 2];
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
    assert_eq!(output, [0.0, 0.0]);
    assert_eq!(output.as_ptr() as usize, output_address);
    assert_eq!(snapshot.total(), 0);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"graph_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":1,",
            "\"swaps_accepted\":{},\"swaps_deferred\":{},",
            "\"displaced_plans_destroyed_off_render\":{},\"output_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},",
            "\"logs\":{},\"file_io\":{},\"network_io\":{},",
            "\"syscalls\":{},\"total_violations\":{}}}"
        ),
        blocks,
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
    output: &mut [f32; 2],
    block: u64,
) -> RealtimeRenderReport {
    let output_view = PlanarBufferMut::try_new(output, 2, 1, 1).expect("fixed output");
    owner
        .render(
            RenderIo {
                input: None,
                output: output_view,
            },
            RenderTime {
                absolute_sample: block,
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
) -> miso_engine_core::realtime::PreparedRenderPlan {
    let input = GraphNodeId::TrackStage {
        track_id: StableGraphId::parse("audit").expect("ID"),
        stage: TrackStage::Input,
    };
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("ID"),
    };
    let edge = GraphEdge {
        id: GraphEdgeId::TrackMain {
            target: output.clone(),
        },
        source: GraphPortId {
            node: input.clone(),
            kind: GraphPortKind::MainOutput,
            effect_port: None,
        },
        destination: GraphPortId {
            node: output.clone(),
            kind: GraphPortKind::MainInput,
            effect_port: None,
        },
        path: "$.audit".to_owned(),
    };
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(1),
        input_channels: None,
        output_channels: NonZeroUsize::new(2).expect("two"),
    };
    let node = |id| GraphNode {
        id,
        latency: LatencySamples(0),
        tail: TailSamples::Finite(0),
    };
    let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id,
        spec: GraphSpec {
            nodes: vec![node(input.clone()), node(output.clone())],
            ports: Vec::new(),
            edges: vec![edge],
        },
        sequential_schedule: vec![input.clone(), output.clone()],
        dependency_levels: vec![
            miso_engine_graph::DependencyLevel {
                level: 0,
                nodes: vec![input.clone()],
            },
            miso_engine_graph::DependencyLevel {
                level: 1,
                nodes: vec![output.clone()],
            },
        ],
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: GraphResourceEstimate {
            logical_nodes: 2,
            materialized_nodes: 2,
            edges: 1,
            schedule_items: 2,
            dependency_levels: 2,
            reductions: 0,
            routes: 0,
            effects: 0,
            audio_buffer_samples: 6,
            total_delay_samples: 0,
            delay_bytes: 0,
            graph_metadata_bytes: 0,
            declared_effect_bytes: 0,
            effect_bank_count: 0,
            effect_bank_scratch_bytes: 0,
            effect_bank_runtime_buffer_bytes: 0,
            effect_bank_metadata_bytes: 0,
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 4,
            incremental_plan_bytes: 24,
            session_plus_plan_bytes: 24,
        },
        envelope,
        required_bindings: vec![input.clone(), output.clone()],
        routes: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        banks: Vec::new(),
        builtin_banks: Vec::new(),
        observers: Vec::new(),
    });
    match graph.bind(GraphRuntimeBindings {
        #[cfg(not(target_arch = "wasm32"))]
        worker_lease: None,
        envelope,
        nodes: vec![
            GraphNodeBinding::new(
                input,
                Box::new(Silence {
                    plan_id,
                    drops: drop_records,
                }),
            ),
            GraphNodeBinding::new(
                output,
                Box::new(Silence {
                    plan_id,
                    drops: None,
                }),
            ),
        ],
        observers: Vec::new(),
    }) {
        Ok(plan) => plan,
        Err(GraphBindFailure { code, .. }) => panic!("graph bind: {code}"),
    }
}
