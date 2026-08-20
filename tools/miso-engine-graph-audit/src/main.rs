//! One-million-block allocation and forbidden-operation audit for a bound scalar graph.

#![allow(unsafe_code)]

use core::num::NonZeroUsize;
use std::alloc::{GlobalAlloc, Layout, System};

use miso_engine_core::realtime::audit::{self, ForbiddenOperation, record_allocator_violation};
use miso_engine_core::realtime::{
    PlanarBufferMut, RenderEnvelope, RenderError, RenderIo, RenderTime,
};
use miso_engine_core::{QuantumFrames, SampleRateHz};
use miso_engine_effect_contract::{LatencySamples, TailSamples};
use miso_engine_graph::{
    GraphBindFailure, GraphBindingBlock, GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding,
    GraphNodeId, GraphPortId, GraphPortKind, GraphResourceEstimate, GraphRuntimeBindings,
    GraphRuntimeProcessor, GraphSpec, PreparedGraphPlan, PreparedGraphPlanParts, StableGraphId,
    TrackStage,
};

struct AuditedAllocator;

#[global_allocator]
static GLOBAL_ALLOCATOR: AuditedAllocator = AuditedAllocator;

// SAFETY: every operation forwards the unchanged pointer/layout contract to `System`. The audit
// branch aborts instead of unwinding through `GlobalAlloc`.
unsafe impl GlobalAlloc for AuditedAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the caller supplied this valid layout to the global allocator.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the caller supplied this valid layout to the global allocator.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        if record_allocator_violation(ForbiddenOperation::Deallocation) {
            std::process::abort();
        }
        // SAFETY: the pointer/layout pair came from this allocator.
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        if record_allocator_violation(ForbiddenOperation::Allocation) {
            std::process::abort();
        }
        // SAFETY: the allocation and original layout came from this allocator.
        unsafe { System.realloc(pointer, layout, size) }
    }
}

struct Silence;
impl GraphRuntimeProcessor for Silence {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(0.0);
        block.right.fill(0.0);
        Ok(())
    }
}

fn main() {
    let blocks = parse_blocks();
    let mut plan = prepared_graph();
    let mut output = [1.0_f32; 2];
    let output_address = output.as_ptr() as usize;
    audit::warm_up();
    audit::reset();
    eprintln!("MISO_GRAPH_RT_BEGIN");
    for block in 0..blocks {
        let output_view = PlanarBufferMut::try_new(&mut output, 2, 1, 1).expect("fixed output");
        let report = plan
            .render(
                RenderIo {
                    input: None,
                    output: output_view,
                },
                RenderTime {
                    absolute_sample: block,
                },
            )
            .expect("graph render");
        assert_eq!(report.plan_id, 6);
        assert_eq!(output, [0.0, 0.0]);
        assert_eq!(output.as_ptr() as usize, output_address);
    }
    eprintln!("MISO_GRAPH_RT_END");
    let snapshot = audit::snapshot();
    assert_eq!(snapshot.total(), 0);
    println!(
        concat!(
            "{{\"schema_version\":1,\"kind\":\"graph_realtime_audit\",",
            "\"blocks\":{},\"quantum_frames\":1,\"output_address\":{},",
            "\"allocations\":{},\"deallocations\":{},\"locks\":{},",
            "\"logs\":{},\"file_io\":{},\"network_io\":{},",
            "\"syscalls\":{},\"total_violations\":{}}}"
        ),
        blocks,
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

fn prepared_graph() -> miso_engine_core::realtime::PreparedRenderPlan {
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
        plan_id: 6,
        spec: GraphSpec {
            nodes: vec![node(input.clone()), node(output.clone())],
            ports: Vec::new(),
            edges: vec![edge],
        },
        sequential_schedule: vec![input.clone(), output.clone()],
        dependency_levels: Vec::new(),
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
            largest_allocation_bytes: 4,
            incremental_plan_bytes: 24,
            session_plus_plan_bytes: 24,
        },
        envelope,
        required_bindings: vec![input.clone(), output.clone()],
        routes: Vec::new(),
        effects: Vec::new(),
    });
    match graph.bind(GraphRuntimeBindings {
        envelope,
        nodes: vec![
            GraphNodeBinding::new(input, Box::new(Silence)),
            GraphNodeBinding::new(output, Box::new(Silence)),
        ],
    }) {
        Ok(plan) => plan,
        Err(GraphBindFailure { code, .. }) => panic!("graph bind: {code}"),
    }
}
