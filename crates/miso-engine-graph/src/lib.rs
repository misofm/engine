//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `miso-engine-graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

use core::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use miso_engine_core::{
    QuantumFrames,
    realtime::{
        BufferArena, PlanarBufferMut, PlanarBufferRef, PrepareRenderPlan, PreparedPlanExecutor,
        PreparedRenderPlan, RenderEnvelope, RenderError,
    },
};
use miso_engine_effect_contract::{
    EffectProcessBlock, LatencySamples, PreparedEffectMetadata, PreparedNativeEffect, TailSamples,
};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StableGraphId(String);
impl StableGraphId {
    pub fn parse(value: &str) -> Option<Self> {
        let bytes = value.as_bytes();
        if !(1..=127).contains(&bytes.len()) || !bytes[0].is_ascii_lowercase() {
            return None;
        }
        if bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        }) {
            Some(Self(value.to_owned()))
        } else {
            None
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RackId {
    Simd1 = 1,
    Dynamic = 2,
    Simd2 = 3,
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TrackStage {
    Input = 1,
    PostInputBuiltins = 2,
    PostSimd1 = 3,
    PostDynamic = 4,
    PostSimd2PreFader = 5,
    PostFader = 6,
    PostMatrix = 7,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EffectNodeId {
    pub track_id: StableGraphId,
    pub rack: RackId,
    pub effect_id: StableGraphId,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphNodeId {
    TrackStage {
        track_id: StableGraphId,
        stage: TrackStage,
    },
    Effect(EffectNodeId),
    Route {
        route_id: StableGraphId,
    },
    Submix {
        submix_id: StableGraphId,
    },
    Output {
        output_id: StableGraphId,
    },
    CompensationDelay {
        edge_id: Box<GraphEdgeId>,
    },
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphPortKind {
    MainInput = 1,
    MainOutput = 2,
    SidechainInput = 3,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GraphPortId {
    pub node: GraphNodeId,
    pub kind: GraphPortKind,
    pub effect_port: Option<String>,
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphEdgeId {
    TrackMain { target: GraphNodeId },
    RouteSource { route_id: StableGraphId },
    RouteDestination { route_id: StableGraphId },
    EffectSidechain { effect: EffectNodeId, port: String },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphNode {
    pub id: GraphNodeId,
    pub latency: LatencySamples,
    pub tail: TailSamples,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEdge {
    pub id: GraphEdgeId,
    pub source: GraphPortId,
    pub destination: GraphPortId,
    pub path: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphSpec {
    pub nodes: Vec<GraphNode>,
    pub ports: Vec<GraphPortId>,
    pub edges: Vec<GraphEdge>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyLevel {
    pub level: u64,
    pub nodes: Vec<GraphNodeId>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RouteTiming {
    pub route_id: StableGraphId,
    pub source_arrival: LatencySamples,
    pub compensation_delay: LatencySamples,
    pub destination_arrival: LatencySamples,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsertedDelay {
    pub node: GraphNodeId,
    pub edge_id: GraphEdgeId,
    pub samples: LatencySamples,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReductionRecord {
    pub node: GraphNodeId,
    pub contributions: Vec<GraphEdgeId>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferAssignment {
    pub port: GraphPortId,
    pub buffer_index: u64,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphResourceEstimate {
    pub logical_nodes: u64,
    pub materialized_nodes: u64,
    pub edges: u64,
    pub schedule_items: u64,
    pub dependency_levels: u64,
    pub reductions: u64,
    pub routes: u64,
    pub effects: u64,
    pub audio_buffer_samples: u64,
    pub total_delay_samples: u64,
    pub delay_bytes: u64,
    pub graph_metadata_bytes: u64,
    pub declared_effect_bytes: u64,
    pub largest_allocation_bytes: u64,
    pub incremental_plan_bytes: u64,
    pub session_plus_plan_bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphCompileCaps {
    pub maximum_nodes: u64,
    pub maximum_edges: u64,
    pub maximum_schedule_items: u64,
    pub maximum_dependency_levels: u64,
    pub maximum_audio_buffer_samples: u64,
    pub maximum_delay_samples_per_edge: u64,
    pub maximum_total_delay_samples: u64,
    pub maximum_graph_bytes: u64,
    pub maximum_plan_bytes: u64,
    pub maximum_single_allocation_bytes: u64,
    pub maximum_finite_tail_samples: u64,
}
impl GraphCompileCaps {
    pub fn all_nonzero(self) -> bool {
        [
            self.maximum_nodes,
            self.maximum_edges,
            self.maximum_schedule_items,
            self.maximum_dependency_levels,
            self.maximum_audio_buffer_samples,
            self.maximum_delay_samples_per_edge,
            self.maximum_total_delay_samples,
            self.maximum_graph_bytes,
            self.maximum_plan_bytes,
            self.maximum_single_allocation_bytes,
            self.maximum_finite_tail_samples,
        ]
        .into_iter()
        .all(|v| v != 0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphDiagnostic {
    pub code: &'static str,
    pub path: String,
    pub cycle: Vec<GraphNodeId>,
    pub cycle_edge_paths: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphDiagnosticSet(Vec<GraphDiagnostic>);
impl GraphDiagnosticSet {
    pub fn sorted(mut diagnostics: Vec<GraphDiagnostic>) -> Self {
        diagnostics.sort();
        diagnostics.dedup();
        Self(diagnostics)
    }
    pub fn diagnostics(&self) -> &[GraphDiagnostic] {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteTransform {
    pub gain: f32,
    pub ll: f32,
    pub lr: f32,
    pub rl: f32,
    pub rr: f32,
}
impl RouteTransform {
    pub fn transform(self, left: f32, right: f32, sanitized: &mut u64) -> (f32, f32) {
        let mut l = self.gain * (self.ll * left + self.lr * right);
        let mut r = self.gain * (self.rl * left + self.rr * right);
        if !l.is_finite() || l.is_subnormal() {
            l = 0.0;
            *sanitized = sanitized.saturating_add(1);
        }
        if !r.is_finite() || r.is_subnormal() {
            r = 0.0;
            *sanitized = sanitized.saturating_add(1);
        }
        (l, r)
    }
}

pub fn balanced_pairwise_sum(values: &mut [f32], sanitized: &mut u64) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    let mut length = values.len();
    while length > 1 {
        let mut write = 0;
        let mut read = 0;
        while read + 1 < length {
            let sum = values[read] + values[read + 1];
            values[write] = if sum.is_finite() && !sum.is_subnormal() {
                sum
            } else {
                *sanitized = sanitized.saturating_add(1);
                0.0
            };
            write += 1;
            read += 2;
        }
        if read < length {
            values[write] = values[read];
            write += 1;
        }
        length = write;
    }
    values[0]
}

pub struct CompensationDelay {
    left: Vec<f32>,
    right: Vec<f32>,
    cursor: usize,
}
impl CompensationDelay {
    pub fn new(samples: usize) -> Self {
        Self {
            left: vec![0.0; samples],
            right: vec![0.0; samples],
            cursor: 0,
        }
    }
    pub fn samples(&self) -> usize {
        self.left.len()
    }
    pub fn reset(&mut self) {
        self.left.fill(0.0);
        self.right.fill(0.0);
        self.cursor = 0;
    }
    pub fn process(&mut self, left: &mut [f32], right: &mut [f32]) {
        if self.left.is_empty() {
            return;
        }
        for (l, r) in left.iter_mut().zip(right) {
            let old_l = self.left[self.cursor];
            let old_r = self.right[self.cursor];
            self.left[self.cursor] = *l;
            self.right[self.cursor] = *r;
            *l = old_l;
            *r = old_r;
            self.cursor = (self.cursor + 1) % self.left.len();
        }
    }
}

pub struct PreparedGraphPlan {
    plan_id: u64,
    pub spec: GraphSpec,
    pub sequential_schedule: Vec<GraphNodeId>,
    pub dependency_levels: Vec<DependencyLevel>,
    pub route_timings: Vec<RouteTiming>,
    pub inserted_delays: Vec<InsertedDelay>,
    pub buffer_assignments: Vec<BufferAssignment>,
    pub estimate: GraphResourceEstimate,
    pub envelope: RenderEnvelope,
    pub required_bindings: Vec<GraphNodeId>,
    routes: Vec<PreparedRoute>,
    effects: Vec<GraphPreparedEffect>,
    _not_sync: Cell<()>,
}
pub struct GraphPreparedEffect {
    pub id: EffectNodeId,
    pub metadata: PreparedEffectMetadata,
    pub processor: Box<dyn PreparedNativeEffect>,
}
impl PreparedGraphPlan {
    pub fn new(parts: PreparedGraphPlanParts) -> Self {
        Self {
            plan_id: parts.plan_id,
            spec: parts.spec,
            sequential_schedule: parts.sequential_schedule,
            dependency_levels: parts.dependency_levels,
            route_timings: parts.route_timings,
            inserted_delays: parts.inserted_delays,
            buffer_assignments: parts.buffer_assignments,
            estimate: parts.estimate,
            envelope: parts.envelope,
            required_bindings: parts.required_bindings,
            routes: parts.routes,
            effects: parts.effects,
            _not_sync: Cell::new(()),
        }
    }
    pub fn bind(
        self,
        bindings: GraphRuntimeBindings,
    ) -> Result<PreparedRenderPlan, GraphBindFailure> {
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let required: BTreeSet<_> = self.required_bindings.iter().cloned().collect();
        let duplicate_binding = supplied.len() != bindings.nodes.len();
        if bindings.envelope != self.envelope || supplied != required || duplicate_binding {
            let envelope_mismatch = bindings.envelope != self.envelope;
            return Err(GraphBindFailure {
                plan: Box::new(self),
                bindings,
                code: if envelope_mismatch {
                    "graph.plan.envelope_mismatch"
                } else {
                    "graph.plan.binding"
                },
            });
        }
        let envelope = self.envelope;
        let executor = GraphExecutor::new(
            self.spec,
            self.sequential_schedule,
            self.inserted_delays,
            self.routes,
            self.effects,
            bindings.nodes,
            envelope.quantum.0 as usize,
        );
        Ok(PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id: self.plan_id,
                envelope,
                scratch: &[],
                parameter_defaults: &[],
                event_capacity: 0,
            },
            Box::new(executor),
        )
        .expect("prevalidated graph plan"))
    }
}
pub struct PreparedGraphPlanParts {
    pub plan_id: u64,
    pub spec: GraphSpec,
    pub sequential_schedule: Vec<GraphNodeId>,
    pub dependency_levels: Vec<DependencyLevel>,
    pub route_timings: Vec<RouteTiming>,
    pub inserted_delays: Vec<InsertedDelay>,
    pub buffer_assignments: Vec<BufferAssignment>,
    pub estimate: GraphResourceEstimate,
    pub envelope: RenderEnvelope,
    pub required_bindings: Vec<GraphNodeId>,
    pub routes: Vec<PreparedRoute>,
    pub effects: Vec<GraphPreparedEffect>,
}
pub struct GraphRuntimeBindings {
    pub envelope: RenderEnvelope,
    pub nodes: Vec<GraphNodeBinding>,
}
pub struct GraphBindFailure {
    pub plan: Box<PreparedGraphPlan>,
    pub bindings: GraphRuntimeBindings,
    pub code: &'static str,
}
pub struct GraphNodeBinding {
    pub node: GraphNodeId,
    processor: Box<dyn GraphRuntimeProcessor>,
}
impl GraphNodeBinding {
    pub fn new(node: GraphNodeId, processor: Box<dyn GraphRuntimeProcessor>) -> Self {
        Self { node, processor }
    }
}
pub struct GraphBindingBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub first_sample: u64,
}
pub trait GraphRuntimeProcessor: Send {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError>;
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedRoute {
    pub node: GraphNodeId,
    pub transform: RouteTransform,
}

struct StereoBuffer {
    left: Box<[f32]>,
    right: Box<[f32]>,
}
impl StereoBuffer {
    fn new(frames: usize) -> Self {
        Self {
            left: vec![0.0; frames].into_boxed_slice(),
            right: vec![0.0; frames].into_boxed_slice(),
        }
    }
}
struct RuntimeEdge {
    source: usize,
    sidechain: bool,
    delay: Option<CompensationDelay>,
    contribution: StereoBuffer,
}
enum RuntimeNodeKind {
    Identity,
    Bound(Box<dyn GraphRuntimeProcessor>),
    Effect(GraphPreparedEffect),
    Route(RouteTransform),
    Reduction,
}
struct RuntimeNode {
    incoming: Vec<RuntimeEdge>,
    output: StereoBuffer,
    kind: RuntimeNodeKind,
}
struct GraphExecutor {
    nodes: Vec<RuntimeNode>,
    output_node: usize,
    reduction_scratch: Box<[f32]>,
    sanitized_samples: u64,
}
impl GraphExecutor {
    #[allow(clippy::too_many_arguments)]
    fn new(
        spec: GraphSpec,
        schedule: Vec<GraphNodeId>,
        inserted_delays: Vec<InsertedDelay>,
        routes: Vec<PreparedRoute>,
        effects: Vec<GraphPreparedEffect>,
        bindings: Vec<GraphNodeBinding>,
        frames: usize,
    ) -> Self {
        let indexes: BTreeMap<_, _> = schedule
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, node)| (node, index))
            .collect();
        let delays: BTreeMap<_, _> = inserted_delays
            .into_iter()
            .map(|delay| (delay.edge_id, delay.samples.0))
            .collect();
        let mut routes: BTreeMap<_, _> = routes
            .into_iter()
            .map(|route| (route.node, route.transform))
            .collect();
        let mut effects: BTreeMap<_, _> = effects
            .into_iter()
            .map(|effect| (GraphNodeId::Effect(effect.id.clone()), effect))
            .collect();
        let mut bindings: BTreeMap<_, _> = bindings
            .into_iter()
            .map(|binding| (binding.node, binding.processor))
            .collect();
        let mut incoming_by_node: BTreeMap<_, Vec<_>> = schedule
            .iter()
            .cloned()
            .map(|node| (node, Vec::new()))
            .collect();
        for edge in spec.edges {
            incoming_by_node
                .get_mut(&edge.destination.node)
                .expect("validated destination")
                .push(edge);
        }
        let mut maximum_inputs = 1usize;
        let mut nodes = Vec::with_capacity(schedule.len());
        for node_id in &schedule {
            let incoming: Vec<_> = incoming_by_node
                .remove(node_id)
                .expect("validated schedule node")
                .into_iter()
                .map(|edge| RuntimeEdge {
                    source: indexes[&edge.source.node],
                    sidechain: edge.destination.kind == GraphPortKind::SidechainInput,
                    delay: delays
                        .get(&edge.id)
                        .copied()
                        .filter(|samples| *samples != 0)
                        .map(|samples| CompensationDelay::new(samples as usize)),
                    contribution: StereoBuffer::new(frames),
                })
                .collect();
            maximum_inputs = maximum_inputs.max(incoming.len());
            let kind = if let Some(processor) = bindings.remove(node_id) {
                RuntimeNodeKind::Bound(processor)
            } else if let Some(effect) = effects.remove(node_id) {
                RuntimeNodeKind::Effect(effect)
            } else if let Some(transform) = routes.remove(node_id) {
                RuntimeNodeKind::Route(transform)
            } else if matches!(
                node_id,
                GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
            ) {
                RuntimeNodeKind::Reduction
            } else {
                RuntimeNodeKind::Identity
            };
            nodes.push(RuntimeNode {
                incoming,
                output: StereoBuffer::new(frames),
                kind,
            });
        }
        let output_node = schedule
            .iter()
            .position(|node| matches!(node, GraphNodeId::Output { .. }))
            .expect("validated single output");
        Self {
            nodes,
            output_node,
            reduction_scratch: vec![0.0; maximum_inputs].into_boxed_slice(),
            sanitized_samples: 0,
        }
    }

    fn prepare_inputs(&mut self, node_index: usize) {
        let (before, current_and_after) = self.nodes.split_at_mut(node_index);
        let current = &mut current_and_after[0];
        for edge in &mut current.incoming {
            let source = &before[edge.source].output;
            edge.contribution.left.copy_from_slice(&source.left);
            edge.contribution.right.copy_from_slice(&source.right);
            if let Some(delay) = &mut edge.delay {
                delay.process(&mut edge.contribution.left, &mut edge.contribution.right);
            }
        }
    }

    fn reduce_main_inputs(&mut self, node_index: usize) {
        let current = &mut self.nodes[node_index];
        let main_inputs = current
            .incoming
            .iter()
            .filter(|edge| !edge.sidechain)
            .count();
        for frame in 0..current.output.left.len() {
            for (slot, edge) in current
                .incoming
                .iter()
                .filter(|edge| !edge.sidechain)
                .enumerate()
            {
                self.reduction_scratch[slot] = edge.contribution.left[frame];
            }
            current.output.left[frame] = balanced_pairwise_sum(
                &mut self.reduction_scratch[..main_inputs],
                &mut self.sanitized_samples,
            );
            for (slot, edge) in current
                .incoming
                .iter()
                .filter(|edge| !edge.sidechain)
                .enumerate()
            {
                self.reduction_scratch[slot] = edge.contribution.right[frame];
            }
            current.output.right[frame] = balanced_pairwise_sum(
                &mut self.reduction_scratch[..main_inputs],
                &mut self.sanitized_samples,
            );
        }
    }
}
impl PreparedPlanExecutor for GraphExecutor {
    fn render(
        &mut self,
        _arena: &mut BufferArena,
        _input: Option<PlanarBufferRef<'_>>,
        mut output: PlanarBufferMut<'_>,
        time: miso_engine_core::realtime::RenderTime,
    ) -> Result<(), RenderError> {
        for node_index in 0..self.nodes.len() {
            self.prepare_inputs(node_index);
            self.reduce_main_inputs(node_index);
            let current = &mut self.nodes[node_index];
            match &mut current.kind {
                RuntimeNodeKind::Identity | RuntimeNodeKind::Reduction => {}
                RuntimeNodeKind::Bound(processor) => processor.process(GraphBindingBlock {
                    left: &mut current.output.left,
                    right: &mut current.output.right,
                    first_sample: time.absolute_sample,
                })?,
                RuntimeNodeKind::Route(transform) => {
                    for frame in 0..current.output.left.len() {
                        (current.output.left[frame], current.output.right[frame]) = transform
                            .transform(
                                current.output.left[frame],
                                current.output.right[frame],
                                &mut self.sanitized_samples,
                            );
                    }
                }
                RuntimeNodeKind::Effect(effect) => {
                    let sidechain = current
                        .incoming
                        .iter()
                        .find(|edge| edge.sidechain)
                        .map(|edge| (&*edge.contribution.left, &*edge.contribution.right));
                    let block = EffectProcessBlock::new(
                        &mut current.output.left,
                        &mut current.output.right,
                        sidechain,
                        time.absolute_sample,
                        &[],
                        effect.metadata.quantum,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                    let _ = effect.processor.process(block);
                }
            }
        }
        let rendered = &self.nodes[self.output_node].output;
        output.plane_mut(0)?.copy_from_slice(&rendered.left);
        output.plane_mut(1)?.copy_from_slice(&rendered.right);
        Ok(())
    }
}

pub fn quantum_samples(quantum: QuantumFrames, count: u64) -> Option<u64> {
    u64::from(quantum.0).checked_mul(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_effect_contract::{
        EffectQuality, InitialParameterValue, LinkMode, NativeEffectFactory, ParameterChannel,
        PortId, PrepareEffectLimits, PrepareEffectRequest, PreparedPortsV1, PreparedSidechainPort,
    };

    struct Noop;
    impl GraphRuntimeProcessor for Noop {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct FixedSource {
        left: [f32; 4],
        right: [f32; 4],
    }

    struct OneShotSource {
        emitted: bool,
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for OneShotSource {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            block.left.fill(0.0);
            block.right.fill(0.0);
            if !self.emitted {
                block.left[0] = self.left;
                block.right[0] = self.right;
                self.emitted = true;
            }
            Ok(())
        }
    }
    impl GraphRuntimeProcessor for FixedSource {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            block.left.copy_from_slice(&self.left);
            block.right.copy_from_slice(&self.right);
            Ok(())
        }
    }

    fn empty_estimate() -> GraphResourceEstimate {
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
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        }
    }

    fn binding_plan() -> (PreparedGraphPlan, GraphRuntimeBindings, GraphNodeId) {
        let input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("track").expect("ID"),
            stage: TrackStage::Input,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(1),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let required = vec![input.clone(), output.clone()];
        let graph_nodes = vec![
            GraphNode {
                id: input.clone(),
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            },
            GraphNode {
                id: output.clone(),
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            },
        ];
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
        (
            PreparedGraphPlan::new(PreparedGraphPlanParts {
                plan_id: 42,
                spec: GraphSpec {
                    nodes: graph_nodes,
                    ports: Vec::new(),
                    edges: vec![edge],
                },
                sequential_schedule: vec![input.clone(), output.clone()],
                dependency_levels: Vec::new(),
                route_timings: Vec::new(),
                inserted_delays: Vec::new(),
                buffer_assignments: Vec::new(),
                estimate: empty_estimate(),
                envelope,
                required_bindings: required.clone(),
                routes: Vec::new(),
                effects: Vec::new(),
            }),
            GraphRuntimeBindings {
                envelope,
                nodes: required
                    .into_iter()
                    .map(|node| GraphNodeBinding::new(node, Box::new(Noop)))
                    .collect(),
            },
            input,
        )
    }

    #[test]
    fn delay_is_exact_and_lane_independent() {
        let mut delay = CompensationDelay::new(2);
        let mut l = [1.0, 2.0, 3.0];
        let mut r = [4.0, 5.0, 6.0];
        delay.process(&mut l, &mut r);
        assert_eq!(l, [0.0, 0.0, 1.0]);
        assert_eq!(r, [0.0, 0.0, 4.0]);
    }
    #[test]
    fn reduction_is_fixed_pairwise() {
        let mut values = [1.0, 2.0, 3.0];
        let mut sanitized = 0;
        assert_eq!(balanced_pairwise_sum(&mut values, &mut sanitized), 6.0);
    }

    #[test]
    fn pairwise_reduction_meets_analytic_bound_and_ignores_completion_order() {
        let fixtures = [
            vec![1.0_f32; 257],
            (0..257)
                .map(|index| if index % 2 == 0 { 1.0 } else { -1.0 })
                .collect(),
            (0..257)
                .map(|index| 2.0_f32.powi(-index.min(120)))
                .collect(),
            vec![1.0e20, 1.0, -1.0e20, 3.0, -2.0, 0.5, -0.5],
        ];
        for fixture in fixtures {
            let reference = fixture.iter().map(|value| f64::from(*value)).sum::<f64>();
            let sum_abs = fixture
                .iter()
                .map(|value| f64::from(value.abs()))
                .sum::<f64>();
            let levels = fixture.len().next_power_of_two().ilog2();
            let u = 2.0_f64.powi(-24);
            let gamma = f64::from(levels) * u / (1.0 - f64::from(levels) * u);
            let bound = gamma * sum_abs + fixture.len() as f64 * f64::from(f32::MIN_POSITIVE);
            let mut values = fixture;
            let mut sanitized = 0;
            let actual = balanced_pairwise_sum(&mut values, &mut sanitized);
            assert_eq!(sanitized, 0);
            assert!((f64::from(actual) - reference).abs() <= bound);
        }

        let canonical: Vec<_> = (0..65)
            .map(|index| (index, (index as f32 + 1.0).recip()))
            .collect();
        let mut baseline_values: Vec<_> = canonical.iter().map(|(_, value)| *value).collect();
        let baseline = balanced_pairwise_sum(&mut baseline_values, &mut 0).to_bits();
        let mut state = 0x6d69_736f_6772_6170_u64;
        for _ in 0..100 {
            let mut completed = canonical.clone();
            for index in (1..completed.len()).rev() {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                completed.swap(index, state as usize % (index + 1));
            }
            // Worker completion order is discarded; the frozen semantic ID order controls the
            // reduction tree.
            completed.sort_by_key(|(id, _)| *id);
            let mut values: Vec<_> = completed.iter().map(|(_, value)| *value).collect();
            assert_eq!(
                balanced_pairwise_sum(&mut values, &mut 0).to_bits(),
                baseline
            );
        }
    }

    #[test]
    fn binding_rejects_duplicates_and_returns_all_ownership() {
        let (plan, mut bindings, duplicate) = binding_plan();
        bindings
            .nodes
            .push(GraphNodeBinding::new(duplicate, Box::new(Noop)));
        let failure = match plan.bind(bindings) {
            Ok(_) => panic!("duplicate binding unexpectedly accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.binding");
        assert_eq!(failure.bindings.nodes.len(), 3);
        assert_eq!(failure.plan.plan_id, 42);
    }

    #[test]
    fn compile_request_plan_id_survives_binding() {
        let (plan, bindings, _) = binding_plan();
        let mut plan = match plan.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => panic!("exact bindings rejected"),
        };
        let mut samples = [1.0_f32; 2];
        let output = PlanarBufferMut::try_new(&mut samples, 2, 1, 1).expect("output");
        let report = plan
            .render(
                miso_engine_core::realtime::RenderIo {
                    input: None,
                    output,
                },
                miso_engine_core::realtime::RenderTime { absolute_sample: 9 },
            )
            .expect("render");
        assert_eq!(report.plan_id, 42);
        assert_eq!(report.next_absolute_sample, 10);
    }

    #[test]
    fn executor_applies_exact_pdc_then_fixed_pairwise_reduction() {
        let input_a = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("a").expect("ID"),
            stage: TrackStage::Input,
        };
        let input_b = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("b").expect("ID"),
            stage: TrackStage::Input,
        };
        let route_a = GraphNodeId::Route {
            route_id: StableGraphId::parse("a").expect("ID"),
        };
        let route_b = GraphNodeId::Route {
            route_id: StableGraphId::parse("b").expect("ID"),
        };
        let output_node = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let edge = |id: GraphEdgeId, source: GraphNodeId, destination: GraphNodeId| GraphEdge {
            id,
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: destination,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.test".to_owned(),
        };
        let delayed_edge = GraphEdgeId::RouteDestination {
            route_id: StableGraphId::parse("a").expect("ID"),
        };
        let edges = vec![
            edge(
                GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("a").expect("ID"),
                },
                input_a.clone(),
                route_a.clone(),
            ),
            edge(
                GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("b").expect("ID"),
                },
                input_b.clone(),
                route_b.clone(),
            ),
            edge(delayed_edge.clone(), route_a.clone(), output_node.clone()),
            edge(
                GraphEdgeId::RouteDestination {
                    route_id: StableGraphId::parse("b").expect("ID"),
                },
                route_b.clone(),
                output_node.clone(),
            ),
        ];
        let schedule = vec![
            input_a.clone(),
            input_b.clone(),
            route_a.clone(),
            route_b.clone(),
            output_node.clone(),
        ];
        let identity = RouteTransform {
            gain: 1.0,
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        };
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let nodes = schedule
            .iter()
            .cloned()
            .map(|id| GraphNode {
                id,
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            })
            .collect();
        let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 77,
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: Vec::new(),
            route_timings: Vec::new(),
            inserted_delays: vec![InsertedDelay {
                node: GraphNodeId::CompensationDelay {
                    edge_id: Box::new(delayed_edge.clone()),
                },
                edge_id: delayed_edge,
                samples: LatencySamples(2),
            }],
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: vec![input_a.clone(), input_b.clone(), output_node.clone()],
            routes: vec![
                PreparedRoute {
                    node: route_a,
                    transform: identity,
                },
                PreparedRoute {
                    node: route_b,
                    transform: identity,
                },
            ],
            effects: Vec::new(),
        });
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(
                    input_a,
                    Box::new(FixedSource {
                        left: [1.0, 0.0, 0.0, 0.0],
                        right: [10.0, 0.0, 0.0, 0.0],
                    }),
                ),
                GraphNodeBinding::new(
                    input_b,
                    Box::new(FixedSource {
                        left: [0.0, 0.0, 2.0, 0.0],
                        right: [0.0, 0.0, 20.0, 0.0],
                    }),
                ),
                GraphNodeBinding::new(output_node, Box::new(Noop)),
            ],
        };
        let mut plan = match plan.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => panic!("bindings"),
        };
        let mut samples = [0.0_f32; 8];
        let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
        plan.render(
            miso_engine_core::realtime::RenderIo {
                input: None,
                output,
            },
            miso_engine_core::realtime::RenderTime { absolute_sample: 0 },
        )
        .expect("render");
        assert_eq!(samples, [0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 30.0, 0.0]);
    }

    fn effect_pdc_plan(rate: u32, quantum: u32, bypass: bool) -> PreparedRenderPlan {
        let input_effect = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("effect-path").expect("ID"),
            stage: TrackStage::Input,
        };
        let input_direct = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("direct-path").expect("ID"),
            stage: TrackStage::Input,
        };
        let effect_id = EffectNodeId {
            track_id: StableGraphId::parse("effect-path").expect("ID"),
            rack: RackId::Dynamic,
            effect_id: StableGraphId::parse("delay").expect("ID"),
        };
        let effect_node = GraphNodeId::Effect(effect_id.clone());
        let route_effect = GraphNodeId::Route {
            route_id: StableGraphId::parse("effect-route").expect("ID"),
        };
        let route_direct = GraphNodeId::Route {
            route_id: StableGraphId::parse("direct-route").expect("ID"),
        };
        let output_node = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let factory = DualAccumulatorDelayFactory::correct();
        let initial_values = [
            InitialParameterValue {
                parameter_index: 0,
                channel: ParameterChannel::Left,
                value: 1.0,
            },
            InitialParameterValue {
                parameter_index: 0,
                channel: ParameterChannel::Right,
                value: 1.0,
            },
        ];
        let processor = factory
            .prepare(PrepareEffectRequest {
                sample_rate: rate,
                quantum,
                quality: EffectQuality::Normal,
                bypass,
                link_mode: LinkMode::DualMono,
                ports: PreparedPortsV1 {
                    sidechain: PreparedSidechainPort::Unconnected {
                        id: PortId::new("sidechain-in").expect("port"),
                        required: false,
                    },
                },
                initial_values: &initial_values,
                limits: PrepareEffectLimits {
                    maximum_total_state_bytes: 1_000,
                    maximum_scratch_bytes: 1_000,
                    maximum_automation_spans_per_block: 1,
                },
            })
            .expect("effect");
        let metadata = processor.metadata();
        let direct_destination = GraphEdgeId::RouteDestination {
            route_id: StableGraphId::parse("direct-route").expect("ID"),
        };
        let make_edge =
            |id: GraphEdgeId, source: GraphNodeId, destination: GraphNodeId| GraphEdge {
                id,
                source: GraphPortId {
                    node: source,
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: destination,
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.pdc".to_owned(),
            };
        let edges = vec![
            make_edge(
                GraphEdgeId::TrackMain {
                    target: effect_node.clone(),
                },
                input_effect.clone(),
                effect_node.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("effect-route").expect("ID"),
                },
                effect_node.clone(),
                route_effect.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteDestination {
                    route_id: StableGraphId::parse("effect-route").expect("ID"),
                },
                route_effect.clone(),
                output_node.clone(),
            ),
            make_edge(
                GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse("direct-route").expect("ID"),
                },
                input_direct.clone(),
                route_direct.clone(),
            ),
            make_edge(
                direct_destination.clone(),
                route_direct.clone(),
                output_node.clone(),
            ),
        ];
        let schedule = vec![
            input_direct.clone(),
            input_effect.clone(),
            effect_node.clone(),
            route_direct.clone(),
            route_effect.clone(),
            output_node.clone(),
        ];
        let graph_nodes = schedule
            .iter()
            .cloned()
            .map(|id| GraphNode {
                latency: if id == effect_node {
                    metadata.latency
                } else {
                    LatencySamples(0)
                },
                tail: if id == effect_node {
                    metadata.tail
                } else {
                    TailSamples::Finite(0)
                },
                id,
            })
            .collect();
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(rate),
            quantum: QuantumFrames(quantum),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let identity = RouteTransform {
            gain: 1.0,
            ll: 1.0,
            lr: 0.0,
            rl: 0.0,
            rr: 1.0,
        };
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: u64::from(rate) + u64::from(quantum),
            spec: GraphSpec {
                nodes: graph_nodes,
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: Vec::new(),
            route_timings: Vec::new(),
            inserted_delays: vec![InsertedDelay {
                node: GraphNodeId::CompensationDelay {
                    edge_id: Box::new(direct_destination.clone()),
                },
                edge_id: direct_destination,
                samples: metadata.latency,
            }],
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: vec![
                input_direct.clone(),
                input_effect.clone(),
                output_node.clone(),
            ],
            routes: vec![
                PreparedRoute {
                    node: route_direct,
                    transform: identity,
                },
                PreparedRoute {
                    node: route_effect,
                    transform: identity,
                },
            ],
            effects: vec![GraphPreparedEffect {
                id: effect_id,
                metadata,
                processor,
            }],
        });
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(
                    input_direct,
                    Box::new(OneShotSource {
                        emitted: false,
                        left: 1.0,
                        right: 2.0,
                    }),
                ),
                GraphNodeBinding::new(
                    input_effect,
                    Box::new(OneShotSource {
                        emitted: false,
                        left: 1.0,
                        right: 2.0,
                    }),
                ),
                GraphNodeBinding::new(output_node, Box::new(Noop)),
            ],
        };
        match graph.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => panic!("bindings"),
        }
    }

    #[test]
    fn enabled_and_bypass_pdc_align_at_all_rates_and_quanta() {
        for rate in miso_engine_core::realtime::SUPPORTED_SAMPLE_RATES {
            for quantum in [1, 127, 128, 255, 1024] {
                for bypass in [false, true] {
                    let mut plan = effect_pdc_plan(rate, quantum, bypass);
                    let mut rendered_left = Vec::new();
                    let mut rendered_right = Vec::new();
                    let blocks = 4_u32.div_ceil(quantum);
                    for block in 0..blocks {
                        let frames = quantum as usize;
                        let mut pcm = vec![0.0_f32; frames * 2];
                        let output =
                            PlanarBufferMut::try_new(&mut pcm, 2, frames, frames).expect("output");
                        plan.render(
                            miso_engine_core::realtime::RenderIo {
                                input: None,
                                output,
                            },
                            miso_engine_core::realtime::RenderTime {
                                absolute_sample: u64::from(block) * u64::from(quantum),
                            },
                        )
                        .expect("render");
                        rendered_left.extend_from_slice(&pcm[..frames]);
                        rendered_right.extend_from_slice(&pcm[frames..]);
                    }
                    assert_eq!(&rendered_left[..4], &[0.0, 0.0, 0.0, 2.0]);
                    assert_eq!(&rendered_right[..4], &[0.0, 0.0, 0.0, 4.0]);
                }
            }
        }
    }
}
