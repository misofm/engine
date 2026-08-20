//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `miso-engine-graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

use core::cell::Cell;
use std::collections::BTreeSet;

use miso_engine_core::{
    QuantumFrames,
    realtime::{
        BufferArena, PlanarBufferMut, PlanarBufferRef, PrepareRenderPlan, PreparedPlanExecutor,
        PreparedRenderPlan, RenderEnvelope, RenderError,
    },
};
use miso_engine_effect_contract::{
    LatencySamples, PreparedEffectMetadata, PreparedNativeEffect, TailSamples,
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
    pub spec: GraphSpec,
    pub sequential_schedule: Vec<GraphNodeId>,
    pub dependency_levels: Vec<DependencyLevel>,
    pub route_timings: Vec<RouteTiming>,
    pub buffer_assignments: Vec<BufferAssignment>,
    pub estimate: GraphResourceEstimate,
    pub envelope: RenderEnvelope,
    pub required_bindings: Vec<GraphNodeId>,
    scratch_buffers: u64,
    effects: Vec<GraphPreparedEffect>,
    _not_sync: Cell<()>,
}
pub struct GraphPreparedEffect {
    pub id: EffectNodeId,
    pub metadata: PreparedEffectMetadata,
    pub processor: Box<dyn PreparedNativeEffect>,
}
impl PreparedGraphPlan {
    pub fn new(
        spec: GraphSpec,
        sequential_schedule: Vec<GraphNodeId>,
        dependency_levels: Vec<DependencyLevel>,
        route_timings: Vec<RouteTiming>,
        buffer_assignments: Vec<BufferAssignment>,
        estimate: GraphResourceEstimate,
        envelope: RenderEnvelope,
        required_bindings: Vec<GraphNodeId>,
        scratch_buffers: u64,
        effects: Vec<GraphPreparedEffect>,
    ) -> Self {
        Self {
            spec,
            sequential_schedule,
            dependency_levels,
            route_timings,
            buffer_assignments,
            estimate,
            envelope,
            required_bindings,
            scratch_buffers,
            effects,
            _not_sync: Cell::new(()),
        }
    }
    pub fn bind(
        self,
        bindings: GraphRuntimeBindings,
    ) -> Result<PreparedRenderPlan, GraphBindFailure> {
        let supplied: BTreeSet<_> = bindings.nodes.iter().cloned().collect();
        let required: BTreeSet<_> = self.required_bindings.iter().cloned().collect();
        if bindings.envelope != self.envelope || supplied != required {
            let envelope_mismatch = bindings.envelope != self.envelope;
            return Err(GraphBindFailure {
                plan: self,
                code: if envelope_mismatch {
                    "graph.plan.envelope_mismatch"
                } else {
                    "graph.plan.binding"
                },
            });
        }
        let specs: Vec<_> = (0..self.scratch_buffers)
            .map(|_| miso_engine_core::realtime::PlanarBufferSpec {
                channels: core::num::NonZeroUsize::new(2).expect("constant nonzero"),
                frame_capacity: self.envelope.quantum,
            })
            .collect();
        // The compiler admitted the envelope and all buffer counts before this point.  Hence the
        // core constructor cannot reject this exact request; keeping the fallible work before the
        // ownership transfer preserves the failure-return contract above.
        let envelope = self.envelope;
        let executor = GraphExecutor {
            _effects: self.effects,
        };
        Ok(PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id: bindings.plan_id,
                envelope,
                scratch: &specs,
                parameter_defaults: &[],
                event_capacity: 0,
            },
            Box::new(executor),
        )
        .expect("prevalidated graph plan"))
    }
}
pub struct GraphRuntimeBindings {
    pub plan_id: u64,
    pub envelope: RenderEnvelope,
    pub nodes: Vec<GraphNodeId>,
}
pub struct GraphBindFailure {
    pub plan: PreparedGraphPlan,
    pub code: &'static str,
}
struct GraphExecutor {
    _effects: Vec<GraphPreparedEffect>,
}
impl PreparedPlanExecutor for GraphExecutor {
    fn render(
        &mut self,
        _arena: &mut BufferArena,
        _input: Option<PlanarBufferRef<'_>>,
        mut output: PlanarBufferMut<'_>,
    ) -> Result<(), RenderError> {
        for channel in 0..output.channels() {
            output.plane_mut(channel)?.fill(0.0);
        }
        Ok(())
    }
}

pub fn quantum_samples(quantum: QuantumFrames, count: u64) -> Option<u64> {
    u64::from(quantum.0).checked_mul(count)
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
