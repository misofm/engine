//! Issue #1271: the swap block that runs a graph plan's carry program allocates and frees nothing
//! on the render thread. The successor takes a heap-owning source slot from its predecessor by a
//! swap; a hand-over that copied, boxed or dropped anything would move the allocator counters.
//!
//! This binary holds only this test: the audited allocator it links aborts on any render-scope
//! allocation, so no unrelated test may share the process.

use bench_support::alloc::{Mode, assert_installed, current_thread_counters, mode, set_mode};
use core::num::NonZeroUsize;
use effect_contract::{LatencySamples, TailSamples};
use engine::{
    QuantumFrames, SampleRateHz,
    realtime::{
        CarryOutcome, PlanExchangeConfig, PlanarBufferMut, PreparedRenderPlan, RealtimePlanOwner,
        RenderEnvelope, RenderError, RenderIo, SwapOutcome, plan_exchange,
    },
};
use graph::*;
use std::any::Any;

struct RestoreMode(Mode);
impl Drop for RestoreMode {
    fn drop(&mut self) {
        set_mode(self.0);
    }
}

struct Noop;
impl GraphRuntimeProcessor for Noop {
    fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        Ok(())
    }
}

/// A heap-owning source slot, moved between plans by the swap-block carry.
struct CarriedSlot {
    slot: Option<Box<[f32]>>,
}

impl GraphPreparedSourceSetDriver for CarriedSlot {
    fn claim_count(&self) -> usize {
        1
    }

    fn begin_block(&mut self, _first_sample: u64, _frames: u32) -> Result<(), RenderError> {
        Ok(())
    }

    fn copy_track_input(
        &mut self,
        _claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        let value = self.slot.as_ref().map_or(0.0, |slot| slot[0]);
        left.fill(value);
        right.fill(value);
        Ok(())
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn source_vacancy(&self, source_index: usize) -> Option<bool> {
        (source_index == 0).then_some(self.slot.is_none())
    }

    fn adopt_sources(
        &mut self,
        predecessor: &mut dyn GraphPreparedSourceSetDriver,
        moves: &[(u32, u32)],
    ) -> bool {
        let Some(predecessor) = predecessor
            .as_any_mut()
            .and_then(|any| any.downcast_mut::<Self>())
        else {
            return false;
        };
        if moves != [(0, 0)] {
            return false;
        }
        core::mem::swap(&mut self.slot, &mut predecessor.slot);
        true
    }
}

fn envelope() -> RenderEnvelope {
    RenderEnvelope {
        sample_rate: SampleRateHz(48_000),
        quantum: QuantumFrames(1),
        output_channels: NonZeroUsize::new(2).expect("two"),
    }
}

/// One track input routed to the output; the input is claimed by a `CarriedSlot` source set.
fn bound(slot: Option<Box<[f32]>>) -> PreparedRenderPlan {
    let input = GraphNodeId::TrackStage {
        track_id: StableGraphId::parse("track").expect("ID"),
        stage: TrackStage::Input,
    };
    let output = GraphNodeId::Output {
        output_id: StableGraphId::parse("main").expect("ID"),
    };
    let mut nodes: Vec<GraphNode> = [input.clone(), output.clone()]
        .into_iter()
        .map(|id| GraphNode {
            id,
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
        })
        .collect();
    nodes.sort_by(|left, right| left.id.cmp(&right.id));
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
        path: "$.test".to_owned(),
    };
    let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
        plan_id: 1,
        spec: GraphSpec {
            nodes,
            ports: Vec::new(),
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
        route_timings: Vec::new(),
        inserted_delays: Vec::new(),
        buffer_assignments: Vec::new(),
        estimate: GraphResourceEstimate {
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
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        },
        envelope: envelope(),
        required_bindings: vec![input.clone(), output.clone()],
        routes: Vec::new(),
        track_delays: Vec::new(),
        effects: Vec::new(),
        effect_controls: Vec::new(),
        effect_observations: Vec::new(),
        banks: Vec::new(),
        builtin_banks: Vec::new(),
        observers: Vec::new(),
    });
    let set = GraphPreparedSourceSet::new(
        envelope(),
        vec![GraphSourceInputClaim { node: input }],
        GraphSourceSetResourceReport {
            pcm_payload_already_charged_bytes: 0,
            overhead_bytes: 0,
            total_engine_owned_bytes: 0,
            largest_allocation_bytes: 0,
        },
        Box::new(CarriedSlot { slot }),
    );
    let bindings = GraphRuntimeBindings {
        envelope: envelope(),
        nodes: vec![GraphNodeBinding::new(output, Box::new(Noop))],
        observers: Vec::new(),
    };
    match plan.bind_with_source_set(bindings, set) {
        Ok(plan) => plan,
        Err(failure) => panic!("bind failed: {}", failure.code),
    }
}

fn block(owner: &mut RealtimePlanOwner) -> ([f32; 2], engine::realtime::RealtimeRenderReport) {
    let mut output = [f32::NAN; 2];
    let sample = owner.next_absolute_sample();
    let report = owner
        .render_contiguous(
            RenderIo {
                output: PlanarBufferMut::try_new(&mut output, 2, 1, 1).expect("output"),
            },
            sample,
        )
        .expect("render");
    (output, report)
}

/// Red if the swap block's hand-over clones, boxes or drops anything on the render thread (the
/// moved slot owns heap storage, so a copy instead of a move allocates).
#[test]
fn the_swap_block_carry_allocates_and_frees_nothing() {
    assert_installed();
    let _restore = RestoreMode(mode());
    set_mode(Mode::Count);

    let mut predecessor = bound(Some(vec![0.75; 64].into_boxed_slice()));
    let identity = plan_identity(&mut predecessor).expect("graph plan");
    let mut successor = bound(None);
    install_carry_program(
        &mut successor,
        GraphCarryProgram {
            predecessor: identity,
            sources: vec![(0, 0)].into_boxed_slice(),
        },
    )
    .expect("install");
    let (mut publisher, mut owner, _retirer) = plan_exchange(
        predecessor,
        PlanExchangeConfig {
            retirement_capacity: NonZeroUsize::new(1).expect("one"),
        },
    )
    .expect("exchange");
    // Warm the render path's process statics before measuring.
    for _ in 0..4 {
        assert_eq!(block(&mut owner).0, [0.75; 2]);
    }
    publisher
        .reserve_replacement(successor, engine::realtime::PlanAdoption::Next)
        .expect("reserve")
        .commit();
    let mark = current_thread_counters();
    let (output, report) = block(&mut owner);
    let after = current_thread_counters();
    assert_eq!(
        (report.swap, report.carry),
        (SwapOutcome::Applied, CarryOutcome::Carried)
    );
    assert_eq!(output, [0.75; 2]);
    assert_eq!(
        (
            after.allocations - mark.allocations,
            after.deallocations - mark.deallocations
        ),
        (0, 0)
    );
}
