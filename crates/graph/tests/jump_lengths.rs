//! Issue #1639 gate 1: the graph reads the plan's jump-length cell once at block entry and holds
//! the three lengths for that block.
//!
//! The plan is rendered through the exchange (`RealtimePlanOwner`), so a block's snapshot is what
//! render read from the plan's own revision gate, as on every host.
#![cfg(feature = "test-support")]

use core::num::NonZeroUsize;
use effect_contract::LatencySamples;
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        self, PlanExchangeConfig, PlanarBufferMut, PreparedRenderPlan, RealtimePlanOwner,
        RenderEnvelope, RenderError, plan_exchange_at_revision,
    },
};
use graph::*;

const FRAMES: usize = 4;

const PREPARED: GraphJumpLengths = GraphJumpLengths {
    fader_samples: 960,
    mute_samples: 480,
    pan_samples: 960,
};

/// A source of a constant, nonzero block.
struct Source;
impl GraphRuntimeProcessor for Source {
    fn retained_bytes(&self) -> u64 {
        0
    }
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        block.left.fill(0.25);
        block.right.fill(-0.25);
        Ok(())
    }
}

fn estimate() -> GraphResourceEstimate {
    GraphResourceEstimate {
        logical_nodes: 0,
        materialized_nodes: 0,
        edges: 0,
        schedule_items: 0,
        dependency_levels: 0,
        reductions: 0,
        routes: 0,
        effects: 0,
        audio_buffer_samples: 0,
        total_delay_samples: 0,
        delay_bytes: 0,
        graph_metadata_bytes: 0,
        declared_effect_bytes: 0,
        effect_bank_count: 0,
        effect_bank_scratch_bytes: 0,
        effect_bank_runtime_buffer_bytes: 0,
        effect_bank_metadata_bytes: 0,
        effect_bank_processor_bytes: 0,
        builtin_bank_bytes: 0,
        builtin_bank_scratch_bytes: 0,
        builtin_bank_count: 0,
        largest_allocation_bytes: 0,
        incremental_plan_bytes: 0,
        session_plus_plan_bytes: 0,
    }
}

/// One track's input feeding the output: the smallest bound graph plan.
fn bound() -> PreparedRenderPlan {
    let envelope = RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(FRAMES as u32),
        output_channels: NonZeroUsize::new(2).expect("stereo"),
    };
    let input = GraphNodeId::TrackStage {
        track_id: StableGraphId::parse("track").expect("track"),
        stage: TrackStage::Input,
    };
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("output"),
    };
    let node = |id: &GraphNodeId| GraphNode {
        id: id.clone(),
        latency: LatencySamples(0),
        bound: effect_contract::NodeTailBound::ZERO,
    };
    let mut nodes = vec![node(&input), node(&output)];
    nodes.sort_by(|a, b| a.id.cmp(&b.id));
    let edge = GraphEdge {
        id: GraphEdgeId::RouteSource {
            route_id: StableGraphId::parse("out").expect("route"),
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
        path: "$.jump_lengths".to_owned(),
    };
    let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 1639,
        spec: GraphSpec {
            nodes,
            ports: vec![],
            edges: vec![edge],
        },
        sequential_schedule: vec![input.clone(), output.clone()],
        dependency_levels: vec![
            DependencyLevel {
                level: 0,
                nodes: vec![input.clone()],
            },
            DependencyLevel {
                level: 1,
                nodes: vec![output.clone()],
            },
        ],
        route_timings: vec![],
        inserted_delays: vec![],
        buffer_assignments: vec![],
        estimate: estimate(),
        envelope,
        required_bindings: vec![input.clone(), output.clone()],
        routes: vec![],
        track_delays: vec![],
        effects: vec![],
        effect_controls: vec![],
        effect_observations: vec![],
        banks: vec![],
        builtin_banks: vec![],
        observers: vec![],
    });
    graph
        .bind(GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(input, Box::new(Source)),
                GraphNodeBinding::identity(output),
            ],
            observers: vec![],
        })
        .unwrap_or_else(|failure| panic!("bind {}", failure.code))
}

/// The exchange around `plan`, whose gate handle the caller keeps. The publisher and the retirer
/// stay alive with the owner.
type Exchange = (
    realtime::PlanRevisionGate,
    RealtimePlanOwner,
    (realtime::PlanPublisher, realtime::PlanRetirer),
);

fn exchange(mut plan: PreparedRenderPlan) -> Exchange {
    let mut gate = plan.take_revision_gate().expect("a fresh plan's gate");
    let (publisher, owner, retirer) = plan_exchange_at_revision(
        plan,
        &mut gate,
        1,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    (gate, owner, (publisher, retirer))
}

/// Renders block `block` and returns the calling thread's allocator events inside the render
/// call alone.
fn render(owner: &mut RealtimePlanOwner, block: usize) -> bench_support::alloc::Counters {
    let mut pcm = vec![1.0_f32; 2 * FRAMES];
    let mark = bench_support::alloc::current_thread_counters();
    owner
        .render(
            realtime::RenderIo {
                output: PlanarBufferMut::try_new(&mut pcm, 2, FRAMES, FRAMES).expect("output"),
            },
            realtime::RenderTime {
                absolute_sample: (block * FRAMES) as u64,
            },
        )
        .expect("render");
    bench_support::alloc::current_thread_delta_since(mark)
}

fn lengths(fader_samples: u32, mute_samples: u32, pan_samples: u32) -> GraphJumpLengths {
    GraphJumpLengths {
        fader_samples,
        mute_samples,
        pan_samples,
    }
}

/// A fresh plan holds its prepared lengths at its first block. A write stamped and published
/// before a block entry is held from that block on, not before; a write whose revision is not yet
/// published is not held; two writes before one block give the last, and the replaced one is
/// counted superseded. Every block allocates and frees nothing, the block that applies a write
/// included (gate 4's render half). The attach allocates exactly the cell's retained row, in two
/// allocations.
///
/// Test value: red if the graph reads the cell only at preparation, under no snapshot (a write
/// held before its revision is published), or applies an older write than the newest, or if the
/// cell's allocations drift from the retained row the hosts charge.
#[test]
fn the_block_entry_holds_the_newest_published_lengths() {
    bench_support::alloc::assert_installed();
    let superseded = realtime::LiveValuesSuperseded::new();
    // Warm any process static the attach touches on a throwaway plan, so the counted attach
    // below sees the cell's own allocations alone.
    let mut warm = bound();
    drop(attach_jump_lengths(&mut warm, PREPARED, &superseded).expect("a graph plan's cell"));
    drop(warm);
    let mut plan = bound();
    let mark = bench_support::alloc::current_thread_counters();
    let mut producer =
        attach_jump_lengths(&mut plan, PREPARED, &superseded).expect("a graph plan's cell");
    let cell = bench_support::alloc::current_thread_delta_since(mark);
    assert_eq!(
        cell.requested_bytes - cell.released_bytes,
        jump_length_retained_bytes(),
        "the cell's live bytes are the retained row"
    );
    assert_eq!(
        (cell.allocations, cell.deallocations, cell.reallocations),
        (2, 0, 0),
        "the cell is two allocations: the shared lane and the plan's reader"
    );
    assert_eq!(
        attach_jump_lengths(&mut plan, PREPARED, &superseded).err(),
        Some(GraphJumpLengthError::AlreadyAttached)
    );
    let (mut gate, mut owner, _exchange) = exchange(plan);
    // Warm the process statics and the thread-local reader before any count.
    let _ = render(&mut owner, 0);
    assert_eq!(test_only_block_jump_lengths(), Some(PREPARED));
    let used = render(&mut owner, 1);
    assert_eq!((used.allocations, used.deallocations), (0, 0));
    assert_eq!(
        test_only_block_jump_lengths(),
        Some(PREPARED),
        "no write: the prepared lengths"
    );

    // A write stamped but not yet published is not held.
    let first = lengths(4_800, 2_400, 4_800);
    let revision = gate.last_published() + 1;
    producer.write(gate.stamp(revision).expect("a legal revision"), first);
    let used = render(&mut owner, 2);
    assert_eq!((used.allocations, used.deallocations), (0, 0));
    assert_eq!(
        test_only_block_jump_lengths(),
        Some(PREPARED),
        "an unpublished write is not held"
    );
    gate.publish(revision);
    let used = render(&mut owner, 3);
    assert_eq!(
        (used.allocations, used.deallocations),
        (0, 0),
        "the block that applies a write allocates and frees nothing"
    );
    assert_eq!(
        test_only_block_jump_lengths(),
        Some(first),
        "held from the first block after the publication"
    );
    assert_eq!(superseded.get(), 0);

    // Two writes before one block: the last is held, and the first is counted superseded.
    let replaced = lengths(1, 2, 3);
    let last = lengths(0, 0, 0);
    for value in [replaced, last] {
        let revision = gate.last_published() + 1;
        producer.write(gate.stamp(revision).expect("a legal revision"), value);
        gate.publish(revision);
    }
    let used = render(&mut owner, 4);
    assert_eq!((used.allocations, used.deallocations), (0, 0));
    assert_eq!(
        test_only_block_jump_lengths(),
        Some(last),
        "the newest write"
    );
    assert_eq!(superseded.get(), 1, "the replaced write is superseded");
    assert_eq!(producer.test_only_write_count(), 3);

    // Held across blocks with no write.
    let _ = render(&mut owner, 5);
    assert_eq!(test_only_block_jump_lengths(), Some(last));
}
