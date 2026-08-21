//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `miso-engine-graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

use core::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

use miso_engine_core::{
    KernelBackendV1, QuantumFrames,
    realtime::{
        BufferArena, PlanarBufferMut, PlanarBufferRef, PrepareRenderPlan, PreparedPlanExecutor,
        PreparedRenderPlan, RenderEnvelope, RenderError,
    },
};
use miso_engine_effect_contract::{
    EffectProcessBlock, LatencySamples, PreparedEffectMetadata, PreparedNativeEffect, TailSamples,
};
#[cfg(not(target_arch = "wasm32"))]
pub use miso_engine_native_scheduler::{
    FallbackReasonV1, NativeSchedulerConfigV1, NativeSchedulerResourceReportV1,
    SchedulerSelectionV1,
};
#[cfg(not(target_arch = "wasm32"))]
use miso_engine_native_scheduler::{
    NativeSchedulerJobV1, NativeSchedulerV1, RenderPartitionV1, RenderWaveV1,
    SchedulerDispatchErrorV1, SchedulerPrepareErrorV1, partition_stable_units_v1,
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
    /// Exact prepared post-input builtin bank payload retained by the graph.
    pub builtin_bank_bytes: u64,
    /// Exact AoSoA scratch payload retained by post-input builtin banks.
    pub builtin_bank_scratch_bytes: u64,
    /// Number of full retained post-input builtin banks.
    pub builtin_bank_count: u64,
    pub largest_allocation_bytes: u64,
    pub incremental_plan_bytes: u64,
    pub session_plus_plan_bytes: u64,
}

/// Checked retained storage added by sealed post-input builtin-bank preparation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphBuiltinBankResourceEstimate {
    pub bank_count: u64,
    pub payload_bytes: u64,
    pub scratch_bytes: u64,
    pub scratch_samples: u64,
    pub metadata_bytes: u64,
    pub largest_allocation_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphBuiltinBankAttachError {
    InvalidMembers,
    IncompatibleMembers,
    ResourceMismatch,
    ResourceOverflow,
}

impl GraphResourceEstimate {
    /// Fold exact prepared builtin-bank storage into the graph estimate before publication.
    pub fn checked_add_builtin_banks(
        &mut self,
        resource: GraphBuiltinBankResourceEstimate,
    ) -> Option<()> {
        let mut next = self.clone();
        next.builtin_bank_count = next.builtin_bank_count.checked_add(resource.bank_count)?;
        next.builtin_bank_bytes = next
            .builtin_bank_bytes
            .checked_add(resource.payload_bytes)?;
        next.builtin_bank_scratch_bytes = next
            .builtin_bank_scratch_bytes
            .checked_add(resource.scratch_bytes)?;
        next.audio_buffer_samples = next
            .audio_buffer_samples
            .checked_add(resource.scratch_samples)?;
        next.graph_metadata_bytes = next
            .graph_metadata_bytes
            .checked_add(resource.metadata_bytes)?;
        let retained = resource.payload_bytes.checked_add(resource.scratch_bytes)?;
        next.incremental_plan_bytes = next.incremental_plan_bytes.checked_add(retained)?;
        next.incremental_plan_bytes = next
            .incremental_plan_bytes
            .checked_add(resource.metadata_bytes)?;
        next.session_plus_plan_bytes = next
            .session_plus_plan_bytes
            .checked_add(retained)?
            .checked_add(resource.metadata_bytes)?;
        next.largest_allocation_bytes = next
            .largest_allocation_bytes
            .max(resource.largest_allocation_bytes);
        *self = next;
        Some(())
    }
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
    banks: Vec<GraphPreparedEffectBank>,
    builtin_banks: Vec<GraphPreparedBuiltinBank>,
    observers: Vec<GraphNodeObserverBinding>,
    _not_sync: Cell<()>,
}
pub struct GraphPreparedEffect {
    pub id: EffectNodeId,
    pub metadata: PreparedEffectMetadata,
    pub processor: Box<dyn PreparedNativeEffect>,
}
/// A prepared homogeneous native bank and its original graph member identities.
pub struct GraphPreparedEffectBank {
    pub members: Box<[EffectNodeId]>,
    pub processor: Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>,
    pub scratch: miso_engine_rack::AoSoaScratch,
}
/// A compiler-owned homogeneous post-input-builtin bank.  Unlike effect banks, this is a
/// fixed graph stage and therefore has no automation or sidechain surface.
pub struct GraphPreparedBuiltinBank {
    pub backend: KernelBackendV1,
    pub members: Box<[GraphNodeId]>,
    pub active_mask: Box<[bool]>,
    pub processor: Box<dyn GraphPreparedBuiltinBankProcessor>,
    pub scratch: miso_engine_rack::AoSoaScratch,
}

/// Address-free prepared builtin-bank metadata available before render binding.
pub struct GraphPreparedBuiltinBankInfo<'a> {
    pub backend: KernelBackendV1,
    pub width: miso_engine_effect_contract::BankWidth,
    pub members: &'a [GraphNodeId],
    pub active_mask: &'a [bool],
}
/// Render contract for an already-prepared builtin bank.
pub trait GraphPreparedBuiltinBankProcessor: Send {
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError>;
    /// Cumulative `[process_calls, architecture_tpt_kernel_calls]` after render is disarmed.
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }
}
impl PreparedGraphPlan {
    /// Number of prepared homogeneous banks retained for off-render-selected execution.
    #[must_use]
    pub const fn prepared_bank_count(&self) -> usize {
        self.banks.len()
    }
    /// Number of retained production post-input builtin banks.
    #[must_use]
    pub const fn prepared_builtin_bank_count(&self) -> usize {
        self.builtin_banks.len()
    }
    /// Compiler-owned node members replaced by fixed post-input builtin banks.
    pub fn builtin_bank_members(&self) -> impl Iterator<Item = &GraphNodeId> {
        self.builtin_banks
            .iter()
            .flat_map(|bank| bank.members.iter())
    }
    /// Address-free semantic membership retained by production builtin banks.
    pub fn builtin_bank_info(&self) -> impl Iterator<Item = GraphPreparedBuiltinBankInfo<'_>> {
        self.builtin_banks
            .iter()
            .map(|bank| GraphPreparedBuiltinBankInfo {
                backend: bank.backend,
                width: bank.scratch.width(),
                members: &bank.members,
                active_mask: &bank.active_mask,
            })
    }
    /// Attach sealed fixed-stage banks before binding.  The graph compiler remains responsible
    /// for deciding eligibility; this validates only the immutable graph shape.
    pub fn with_builtin_banks(
        mut self,
        banks: Vec<GraphPreparedBuiltinBank>,
        resource: GraphBuiltinBankResourceEstimate,
    ) -> Result<Self, GraphBuiltinBankAttachError> {
        let mut seen = BTreeSet::new();
        let level_by_node: BTreeMap<_, _> = self
            .dependency_levels
            .iter()
            .flat_map(|level| {
                level
                    .nodes
                    .iter()
                    .cloned()
                    .map(move |node| (node, level.level))
            })
            .collect();
        for bank in &banks {
            if bank.members.len() != bank.scratch.width().lanes() as usize
                || bank.active_mask.len() != bank.members.len()
                || !bank.scratch.width().matches_backend(bank.backend)
                || bank.members.iter().any(|node| {
                    !matches!(
                        node,
                        GraphNodeId::TrackStage {
                            stage: TrackStage::PostInputBuiltins,
                            ..
                        }
                    ) || !seen.insert(node.clone())
                })
            {
                return Err(GraphBuiltinBankAttachError::InvalidMembers);
            }
            let Some(level) = bank
                .members
                .first()
                .and_then(|member| level_by_node.get(member))
                .copied()
            else {
                return Err(GraphBuiltinBankAttachError::IncompatibleMembers);
            };
            if bank.members.iter().any(|member| {
                level_by_node.get(member).copied() != Some(level)
                    || !self.required_bindings.contains(member)
            }) {
                return Err(GraphBuiltinBankAttachError::IncompatibleMembers);
            }
        }
        if resource.bank_count != u64::try_from(banks.len()).unwrap_or(u64::MAX) {
            return Err(GraphBuiltinBankAttachError::ResourceMismatch);
        }
        self.estimate
            .checked_add_builtin_banks(resource)
            .ok_or(GraphBuiltinBankAttachError::ResourceOverflow)?;
        self.builtin_banks = banks;
        Ok(self)
    }
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
            banks: parts.banks,
            builtin_banks: parts.builtin_banks,
            observers: parts.observers,
            _not_sync: Cell::new(()),
        }
    }
    pub fn bind(
        self,
        mut bindings: GraphRuntimeBindings,
    ) -> Result<PreparedRenderPlan, GraphBindFailure> {
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let builtin_bank_members: BTreeSet<_> = self.builtin_bank_members().cloned().collect();
        let required: BTreeSet<_> = self
            .required_bindings
            .iter()
            .filter(|node| !builtin_bank_members.contains(*node))
            .cloned()
            .collect();
        let duplicate_binding = supplied.len() != bindings.nodes.len();
        let valid_observers = self
            .observers
            .iter()
            .chain(bindings.observers.iter())
            .all(|binding| matches!(binding.node, GraphNodeId::TrackStage { .. }))
            && {
                let mut pairs: BTreeSet<_> = BTreeSet::new();
                self.observers
                    .iter()
                    .chain(bindings.observers.iter())
                    .all(|binding| pairs.insert((binding.node.clone(), binding.handle)))
            };
        if bindings.envelope != self.envelope
            || supplied != required
            || duplicate_binding
            || !valid_observers
        {
            let envelope_mismatch = bindings.envelope != self.envelope;
            return Err(GraphBindFailure {
                plan: Box::new(self),
                bindings,
                code: if !valid_observers {
                    "graph.plan.observer"
                } else if envelope_mismatch {
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
            self.buffer_assignments,
            self.routes,
            self.effects,
            self.banks,
            self.builtin_banks,
            {
                let mut observers = self.observers;
                observers.append(&mut bindings.observers);
                observers
            },
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

    /// Transactionally bind the ownership-split native dependency-wave executor.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::result_large_err)]
    pub fn bind_native(
        self,
        mut bindings: GraphRuntimeBindings,
        config: NativeGraphBindConfigV1,
    ) -> Result<PreparedNativeGraphPlanV1, GraphNativeBindFailure> {
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let builtin_bank_members: BTreeSet<_> = self.builtin_bank_members().cloned().collect();
        let required: BTreeSet<_> = self
            .required_bindings
            .iter()
            .filter(|node| !builtin_bank_members.contains(*node))
            .cloned()
            .collect();
        let duplicate_binding = supplied.len() != bindings.nodes.len();
        let valid_observers = self
            .observers
            .iter()
            .chain(bindings.observers.iter())
            .all(|binding| matches!(binding.node, GraphNodeId::TrackStage { .. }))
            && {
                let mut pairs: BTreeSet<_> = BTreeSet::new();
                self.observers
                    .iter()
                    .chain(bindings.observers.iter())
                    .all(|binding| pairs.insert((binding.node.clone(), binding.handle)))
            };
        if bindings.envelope != self.envelope
            || supplied != required
            || duplicate_binding
            || !valid_observers
        {
            let envelope_mismatch = bindings.envelope != self.envelope;
            return Err(GraphNativeBindFailure {
                plan: Box::new(self),
                bindings,
                config,
                code: if !valid_observers {
                    "graph.plan.observer"
                } else if envelope_mismatch {
                    "graph.plan.envelope_mismatch"
                } else {
                    "graph.plan.binding"
                },
            });
        }
        let blueprint = match NativeGraphBlueprint::prepare(&self, config, bindings.observers.len())
        {
            Ok(blueprint) => blueprint,
            Err(code) => {
                return Err(GraphNativeBindFailure {
                    plan: Box::new(self),
                    bindings,
                    config,
                    code,
                });
            }
        };
        let explicit_fallback = (config.render_mode == NativeGraphRenderModeV1::SingleThread)
            .then_some(FallbackReasonV1::SingleThread);
        let scheduler = match NativeSchedulerV1::prepare_with_fallback(
            config.scheduler,
            blueprint.largest_wave_width,
            self.plan_id,
            explicit_fallback,
        ) {
            Ok(scheduler) => scheduler,
            Err(error) => {
                let code = match error {
                    SchedulerPrepareErrorV1::WorkerStart => "graph.scheduler.worker_start",
                    SchedulerPrepareErrorV1::ResourceOverflow => "graph.scheduler.resource",
                    SchedulerPrepareErrorV1::EmptyWave
                    | SchedulerPrepareErrorV1::InvalidPartition => "graph.scheduler.layout",
                };
                return Err(GraphNativeBindFailure {
                    plan: Box::new(self),
                    bindings,
                    config,
                    code,
                });
            }
        };
        let scheduler_resources = scheduler.resource_report(
            blueprint.waves.len(),
            blueprint.unit_count,
            blueprint.partition_count,
        );
        let Some(total_retained_bytes) = blueprint
            .graph_job_bytes
            .checked_add(scheduler_resources.retained_queue_bytes)
        else {
            return Err(GraphNativeBindFailure {
                plan: Box::new(self),
                bindings,
                config,
                code: "graph.scheduler.resource",
            });
        };
        if total_retained_bytes > config.maximum_retained_bytes {
            return Err(GraphNativeBindFailure {
                plan: Box::new(self),
                bindings,
                config,
                code: "graph.scheduler.cap",
            });
        }
        let envelope = self.envelope;
        let plan_id = self.plan_id;
        let mut graph = self;
        graph.observers.append(&mut bindings.observers);
        let resources = NativeGraphResourceReportV1 {
            scheduler: scheduler_resources,
            graph_job_bytes: blueprint.graph_job_bytes,
            total_retained_bytes,
        };
        #[cfg(feature = "test-support")]
        let test_preparation_transcript = blueprint.test_preparation_transcript();
        let (executor, metadata) = NativeGraphExecutor::new(
            graph,
            bindings.nodes,
            blueprint,
            scheduler,
            resources,
            #[cfg(feature = "test-support")]
            test_preparation_transcript,
        );
        let plan = PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id,
                envelope,
                scratch: &[],
                parameter_defaults: &[],
                event_capacity: 0,
            },
            Box::new(executor),
        )
        .expect("prevalidated native graph plan");
        Ok(PreparedNativeGraphPlanV1 { plan, metadata })
    }
}

/// Frozen session execution choice supplied to native graph binding.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeGraphRenderModeV1 {
    /// Compatibility execution on the callback coordinator.
    SingleThread,
    /// Execute independent dependency-wave partitions on armed native workers when supported.
    DependencyWaves,
}

/// Explicit, checked native graph binding inputs.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeGraphBindConfigV1 {
    pub render_mode: NativeGraphRenderModeV1,
    pub scheduler: NativeSchedulerConfigV1,
    /// Maximum graph-job plus scheduler-queue payload bytes retained by this execution layout.
    pub maximum_retained_bytes: usize,
}

/// Exact address-free native execution storage report.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeGraphResourceReportV1 {
    pub scheduler: NativeSchedulerResourceReportV1,
    pub graph_job_bytes: usize,
    pub total_retained_bytes: usize,
}

/// Frozen metadata returned alongside the publishable prepared render plan.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NativeGraphPreparedMetadataV1 {
    pub selection: SchedulerSelectionV1,
    pub resources: NativeGraphResourceReportV1,
    /// Exact immutable native wave/unit/partition summary for qualification-only tests. This is
    /// absent from normal production builds.
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub test_preparation_transcript: NativeGraphPreparationTranscriptV1,
}

/// Address-free qualification summary of the actual native preparation blueprint.
#[cfg(all(not(target_arch = "wasm32"), feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct NativeGraphPreparationTranscriptV1 {
    /// FNV-1a transcript over every immutable wave, unit key/kind/member sequence and partition.
    pub hash: u64,
    /// Largest immutable wave width before partitioning.
    pub largest_wave_width: usize,
    /// Number of indivisible retained bank units in the native layout.
    pub retained_bank_units: usize,
    /// Total nodes represented by indivisible retained bank units.
    pub retained_bank_members: usize,
    /// Number of indivisible retained builtin-bank units in the native layout.
    pub retained_builtin_bank_units: usize,
    /// Total builtin-bank members represented by those native units.
    pub retained_builtin_bank_members: usize,
    /// `true` only when every prepared partition is contiguous, nonempty, and unpadded.
    pub partitions_are_canonical: bool,
}

/// A native graph preparation result; the contained plan remains the ordinary publication unit.
#[cfg(not(target_arch = "wasm32"))]
pub struct PreparedNativeGraphPlanV1 {
    pub plan: PreparedRenderPlan,
    pub metadata: NativeGraphPreparedMetadataV1,
}

#[cfg(not(target_arch = "wasm32"))]
impl PreparedNativeGraphPlanV1 {
    #[must_use]
    pub fn into_plan(self) -> PreparedRenderPlan {
        self.plan
    }
}

/// Transactional native binding failure returning every caller-owned input.
#[cfg(not(target_arch = "wasm32"))]
pub struct GraphNativeBindFailure {
    pub plan: Box<PreparedGraphPlan>,
    pub bindings: GraphRuntimeBindings,
    pub config: NativeGraphBindConfigV1,
    pub code: &'static str,
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
    pub banks: Vec<GraphPreparedEffectBank>,
    pub builtin_banks: Vec<GraphPreparedBuiltinBank>,
    pub observers: Vec<GraphNodeObserverBinding>,
}
pub struct GraphRuntimeBindings {
    pub envelope: RenderEnvelope,
    pub nodes: Vec<GraphNodeBinding>,
    /// Ordinary graph observation bindings. Compiler-owned builtins are appended only by their
    /// sealed artifact wrapper, never by a generic internal-attachment capability.
    pub observers: Vec<GraphNodeObserverBinding>,
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
/// Immutable post-node observation input. Observers cannot alter graph audio.
pub struct GraphObservationBlock<'a> {
    pub left: &'a [f32],
    pub right: &'a [f32],
    pub first_sample: u64,
}
/// A bounded observer invoked after its node has completed.
pub trait GraphRuntimeObserver: Send {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError>;
}
/// One immutable prepared observer binding, ordered by its stable meter handle.
pub struct GraphNodeObserverBinding {
    pub node: GraphNodeId,
    pub handle: u64,
    observer: Box<dyn GraphRuntimeObserver>,
}
impl GraphNodeObserverBinding {
    pub fn new(node: GraphNodeId, handle: u64, observer: Box<dyn GraphRuntimeObserver>) -> Self {
        Self {
            node,
            handle,
            observer,
        }
    }
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
    source_buffer: usize,
    sidechain: bool,
    delay: Option<CompensationDelay>,
    contribution: StereoBuffer,
}
enum RuntimeNodeKind {
    Identity,
    Bound(Box<dyn GraphRuntimeProcessor>),
    Effect(GraphPreparedEffect),
    BankMember(usize),
    BuiltinBankMember(usize),
    Route(RouteTransform),
    Reduction,
}
struct RuntimeNode {
    incoming: Vec<RuntimeEdge>,
    output_buffer: usize,
    kind: RuntimeNodeKind,
    observers: Vec<GraphNodeObserverBinding>,
}
struct GraphExecutor {
    nodes: Vec<RuntimeNode>,
    buffers: Vec<StereoBuffer>,
    output_node: usize,
    reduction_scratch: Box<[f32]>,
    banks: Vec<RuntimeBank>,
    bank_rendered: Box<[bool]>,
    builtin_banks: Vec<RuntimeBuiltinBank>,
    builtin_bank_rendered: Box<[bool]>,
    sanitized_samples: u64,
}
struct RuntimeBank {
    members: Box<[usize]>,
    processor: Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>,
    scratch: miso_engine_rack::AoSoaScratch,
}
struct RuntimeBuiltinBank {
    members: Box<[usize]>,
    processor: Box<dyn GraphPreparedBuiltinBankProcessor>,
    scratch: miso_engine_rack::AoSoaScratch,
}
impl GraphExecutor {
    #[allow(clippy::too_many_arguments)]
    fn new(
        spec: GraphSpec,
        schedule: Vec<GraphNodeId>,
        inserted_delays: Vec<InsertedDelay>,
        buffer_assignments: Vec<BufferAssignment>,
        routes: Vec<PreparedRoute>,
        effects: Vec<GraphPreparedEffect>,
        banks: Vec<GraphPreparedEffectBank>,
        builtin_banks: Vec<GraphPreparedBuiltinBank>,
        observer_bindings: Vec<GraphNodeObserverBinding>,
        bindings: Vec<GraphNodeBinding>,
        frames: usize,
    ) -> Self {
        let mut assigned_buffers: BTreeMap<_, _> = buffer_assignments
            .into_iter()
            .map(|assignment| (assignment.port.node, assignment.buffer_index))
            .collect();
        let mut next_buffer = assigned_buffers
            .values()
            .copied()
            .max()
            .and_then(|maximum| maximum.checked_add(1))
            .unwrap_or(0);
        for node in &schedule {
            assigned_buffers.entry(node.clone()).or_insert_with(|| {
                let buffer = next_buffer;
                next_buffer = next_buffer.checked_add(1).expect("validated buffer count");
                buffer
            });
        }
        // Liveness coloring is valid for the scalar schedule, where equal-colored effect outputs
        // are consumed before the next producer reuses that storage. A homogeneous bank makes all
        // of its member outputs live together from gather through scatter, so retain distinct
        // runtime buffers for those original graph nodes. This is an execution-only allocation:
        // the immutable graph's canonical assignment, reductions, PDC and schedule stay intact.
        let mut runtime_buffers = assigned_buffers.clone();
        for bank in &banks {
            for member in &bank.members {
                let node = GraphNodeId::Effect(member.clone());
                runtime_buffers.insert(node, next_buffer);
                next_buffer = next_buffer.checked_add(1).expect("validated buffer count");
            }
        }
        for bank in &builtin_banks {
            for member in &bank.members {
                runtime_buffers.insert(member.clone(), next_buffer);
                next_buffer = next_buffer.checked_add(1).expect("validated buffer count");
            }
        }
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
        let bank_by_node: BTreeMap<_, _> = banks
            .iter()
            .enumerate()
            .flat_map(|(index, bank)| {
                bank.members
                    .iter()
                    .cloned()
                    .map(move |member| (GraphNodeId::Effect(member), index))
            })
            .collect();
        let builtin_bank_by_node: BTreeMap<_, _> = builtin_banks
            .iter()
            .enumerate()
            .flat_map(|(index, bank)| {
                bank.members
                    .iter()
                    .cloned()
                    .map(move |member| (member, index))
            })
            .collect();
        let mut bindings: BTreeMap<_, _> = bindings
            .into_iter()
            .map(|binding| (binding.node, binding.processor))
            .collect();
        let mut observers: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for observer in observer_bindings {
            observers
                .entry(observer.node.clone())
                .or_default()
                .push(observer);
        }
        for values in observers.values_mut() {
            values.sort_by_key(|value| value.handle);
        }
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
        let mut maximum_buffer = 0_u64;
        for node_id in &schedule {
            let incoming: Vec<_> = incoming_by_node
                .remove(node_id)
                .expect("validated schedule node")
                .into_iter()
                .map(|edge| RuntimeEdge {
                    source_buffer: usize::try_from(runtime_buffers[&edge.source.node])
                        .expect("validated buffer index"),
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
            let kind = if let Some(bank) = builtin_bank_by_node.get(node_id) {
                RuntimeNodeKind::BuiltinBankMember(*bank)
            } else if let Some(processor) = bindings.remove(node_id) {
                RuntimeNodeKind::Bound(processor)
            } else if let Some(bank) = bank_by_node.get(node_id) {
                RuntimeNodeKind::BankMember(*bank)
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
            let output_buffer = runtime_buffers[node_id];
            maximum_buffer = maximum_buffer.max(output_buffer);
            nodes.push(RuntimeNode {
                incoming,
                output_buffer: usize::try_from(output_buffer).expect("validated buffer index"),
                kind,
                observers: observers.remove(node_id).unwrap_or_default(),
            });
        }
        let buffer_count = if nodes.is_empty() {
            0
        } else {
            usize::try_from(maximum_buffer)
                .expect("validated buffer index")
                .checked_add(1)
                .expect("validated buffer count")
        };
        let output_node = schedule
            .iter()
            .position(|node| matches!(node, GraphNodeId::Output { .. }))
            .expect("validated single output");
        let node_indices: BTreeMap<_, _> = schedule
            .iter()
            .cloned()
            .enumerate()
            .map(|(index, node)| (node, index))
            .collect();
        let runtime_banks = banks
            .into_iter()
            .map(|bank| RuntimeBank {
                members: bank
                    .members
                    .iter()
                    .cloned()
                    .map(|member| node_indices[&GraphNodeId::Effect(member)])
                    .collect(),
                processor: bank.processor,
                scratch: bank.scratch,
            })
            .collect::<Vec<_>>();
        let runtime_builtin_banks = builtin_banks
            .into_iter()
            .map(|bank| RuntimeBuiltinBank {
                members: bank
                    .members
                    .iter()
                    .map(|member| node_indices[member])
                    .collect(),
                processor: bank.processor,
                scratch: bank.scratch,
            })
            .collect::<Vec<_>>();
        Self {
            nodes,
            buffers: (0..buffer_count)
                .map(|_| StereoBuffer::new(frames))
                .collect(),
            output_node,
            reduction_scratch: vec![0.0; maximum_inputs].into_boxed_slice(),
            bank_rendered: vec![false; runtime_banks.len()].into_boxed_slice(),
            banks: runtime_banks,
            builtin_bank_rendered: vec![false; runtime_builtin_banks.len()].into_boxed_slice(),
            builtin_banks: runtime_builtin_banks,
            sanitized_samples: 0,
        }
    }

    fn prepare_inputs(&mut self, node_index: usize) {
        let current = &mut self.nodes[node_index];
        for edge in &mut current.incoming {
            let source = &self.buffers[edge.source_buffer];
            edge.contribution.left.copy_from_slice(&source.left);
            edge.contribution.right.copy_from_slice(&source.right);
            if let Some(delay) = &mut edge.delay {
                delay.process(&mut edge.contribution.left, &mut edge.contribution.right);
            }
        }
    }

    fn reduce_main_inputs(&mut self, node_index: usize) {
        let current = &mut self.nodes[node_index];
        let output = &mut self.buffers[current.output_buffer];
        let main_inputs = current
            .incoming
            .iter()
            .filter(|edge| !edge.sidechain)
            .count();
        for frame in 0..output.left.len() {
            for (slot, edge) in current
                .incoming
                .iter()
                .filter(|edge| !edge.sidechain)
                .enumerate()
            {
                self.reduction_scratch[slot] = edge.contribution.left[frame];
            }
            output.left[frame] = balanced_pairwise_sum(
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
            output.right[frame] = balanced_pairwise_sum(
                &mut self.reduction_scratch[..main_inputs],
                &mut self.sanitized_samples,
            );
        }
    }

    fn render_bank(&mut self, bank_index: usize, first_sample: u64) -> Result<(), RenderError> {
        let lanes = self.banks[bank_index].members.len();
        for lane in 0..lanes {
            let node_index = self.banks[bank_index].members[lane];
            self.prepare_inputs(node_index);
            self.reduce_main_inputs(node_index);
            let output_buffer = self.nodes[node_index].output_buffer;
            self.banks[bank_index]
                .scratch
                .gather_lane(
                    lane,
                    &self.buffers[output_buffer].left,
                    &self.buffers[output_buffer].right,
                    self.buffers[output_buffer].left.len() as u32,
                )
                .map_err(|_| RenderError::InvalidEnvelope)?;
        }
        let frames = self.buffers[self.nodes[self.banks[bank_index].members[0]].output_buffer]
            .left
            .len() as u32;
        let offsets_four = [0_u32; 5];
        let offsets_eight = [0_u32; 9];
        let offsets = if lanes == 4 {
            &offsets_four[..]
        } else {
            &offsets_eight[..]
        };
        let bank = &mut self.banks[bank_index];
        bank.scratch
            .process(
                bank.processor.as_mut(),
                frames,
                first_sample,
                &[],
                offsets,
                false,
            )
            .map_err(|_| RenderError::InvalidEnvelope)?;
        for lane in 0..lanes {
            let node_index = self.banks[bank_index].members[lane];
            let output_buffer = self.nodes[node_index].output_buffer;
            let buffer = &mut self.buffers[output_buffer];
            self.banks[bank_index]
                .scratch
                .scatter_lane(lane, &mut buffer.left, &mut buffer.right, frames)
                .map_err(|_| RenderError::InvalidEnvelope)?;
        }
        Ok(())
    }

    fn render_builtin_bank(
        &mut self,
        bank_index: usize,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let lanes = self.builtin_banks[bank_index].members.len();
        for lane in 0..lanes {
            let node_index = self.builtin_banks[bank_index].members[lane];
            self.prepare_inputs(node_index);
            self.reduce_main_inputs(node_index);
            let output_buffer = self.nodes[node_index].output_buffer;
            self.builtin_banks[bank_index]
                .scratch
                .gather_lane(
                    lane,
                    &self.buffers[output_buffer].left,
                    &self.buffers[output_buffer].right,
                    self.buffers[output_buffer].left.len() as u32,
                )
                .map_err(|_| RenderError::InvalidEnvelope)?;
        }
        let frames = self.buffers
            [self.nodes[self.builtin_banks[bank_index].members[0]].output_buffer]
            .left
            .len() as u32;
        let bank = &mut self.builtin_banks[bank_index];
        let (left, right) = bank
            .scratch
            .builtin_planes_mut(frames)
            .map_err(|_| RenderError::InvalidEnvelope)?;
        bank.processor.process(left, right, frames, first_sample)?;
        for lane in 0..lanes {
            let node_index = bank.members[lane];
            let output_buffer = self.nodes[node_index].output_buffer;
            let buffer = &mut self.buffers[output_buffer];
            bank.scratch
                .scatter_lane(lane, &mut buffer.left, &mut buffer.right, frames)
                .map_err(|_| RenderError::InvalidEnvelope)?;
        }
        Ok(())
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
        self.bank_rendered.fill(false);
        self.builtin_bank_rendered.fill(false);
        for node_index in 0..self.nodes.len() {
            let bank = match self.nodes[node_index].kind {
                RuntimeNodeKind::BankMember(index) => Some(index),
                _ => None,
            };
            let builtin_bank = match self.nodes[node_index].kind {
                RuntimeNodeKind::BuiltinBankMember(index) => Some(index),
                _ => None,
            };
            if let Some(bank) = bank {
                if !self.bank_rendered[bank] {
                    self.render_bank(bank, time.absolute_sample)?;
                    self.bank_rendered[bank] = true;
                }
            } else if let Some(bank) = builtin_bank {
                if !self.builtin_bank_rendered[bank] {
                    self.render_builtin_bank(bank, time.absolute_sample)?;
                    self.builtin_bank_rendered[bank] = true;
                }
            } else {
                self.prepare_inputs(node_index);
                self.reduce_main_inputs(node_index);
            }
            {
                let current = &mut self.nodes[node_index];
                let output_buffer = current.output_buffer;
                let rendered = &mut self.buffers[output_buffer];
                match &mut current.kind {
                    RuntimeNodeKind::Identity
                    | RuntimeNodeKind::Reduction
                    | RuntimeNodeKind::BuiltinBankMember(_) => {}
                    RuntimeNodeKind::Bound(processor) => processor.process(GraphBindingBlock {
                        left: &mut rendered.left,
                        right: &mut rendered.right,
                        first_sample: time.absolute_sample,
                    })?,
                    RuntimeNodeKind::Route(transform) => {
                        for frame in 0..rendered.left.len() {
                            (rendered.left[frame], rendered.right[frame]) = transform.transform(
                                rendered.left[frame],
                                rendered.right[frame],
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
                            &mut rendered.left,
                            &mut rendered.right,
                            sidechain,
                            time.absolute_sample,
                            &[],
                            effect.metadata.quantum,
                        )
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                        let _ = effect.processor.process(block);
                    }
                    RuntimeNodeKind::BankMember(_) => {}
                }
            }
            let current = &mut self.nodes[node_index];
            let rendered = &self.buffers[current.output_buffer];
            for observer in &mut current.observers {
                observer.observer.observe(GraphObservationBlock {
                    left: &rendered.left,
                    right: &rendered.right,
                    first_sample: time.absolute_sample,
                })?;
            }
        }
        let rendered = &self.buffers[self.nodes[self.output_node].output_buffer];
        output.plane_mut(0)?.copy_from_slice(&rendered.left);
        output.plane_mut(1)?.copy_from_slice(&rendered.right);
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        self.builtin_banks.iter().fold([0, 0], |mut total, bank| {
            let counters = bank.processor.qualification_counters();
            total[0] = total[0].saturating_add(counters[0]);
            total[1] = total[1].saturating_add(counters[1]);
            total
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct NativeNodeLocation {
    wave: usize,
    partition: usize,
    unit: usize,
    member: usize,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
enum NativeUnitBlueprintKind {
    Node,
    EffectBank(usize),
    BuiltinBank(usize),
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeUnitBlueprint {
    key: GraphNodeId,
    members: Box<[GraphNodeId]>,
    kind: NativeUnitBlueprintKind,
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeWaveBlueprint {
    level: u64,
    units: Box<[NativeUnitBlueprint]>,
    partitions: Box<[miso_engine_native_scheduler::RenderPartitionRangeV1]>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
struct NativeStageOperation {
    source: NativeNodeLocation,
    destination: NativeNodeLocation,
    destination_edge: usize,
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphBlueprint {
    waves: Box<[NativeWaveBlueprint]>,
    stage_operations: Box<[Box<[NativeStageOperation]>]>,
    observation_order: Box<[Box<[NativeNodeLocation]>]>,
    output: NativeNodeLocation,
    largest_wave_width: usize,
    unit_count: usize,
    partition_count: usize,
    graph_job_bytes: usize,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
struct NativeBankMembership {
    key: GraphNodeId,
    members: Box<[GraphNodeId]>,
    kind: NativeUnitBlueprintKind,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphBlueprint {
    fn prepare(
        plan: &PreparedGraphPlan,
        config: NativeGraphBindConfigV1,
        runtime_observer_count: usize,
    ) -> Result<Self, &'static str> {
        if config.maximum_retained_bytes == 0 {
            return Err("graph.scheduler.cap");
        }
        let schedule_nodes: BTreeSet<_> = plan.sequential_schedule.iter().cloned().collect();
        if schedule_nodes.len() != plan.sequential_schedule.len() {
            return Err("graph.scheduler.layout");
        }
        let mut level_by_node = BTreeMap::new();
        for (wave_index, level) in plan.dependency_levels.iter().enumerate() {
            if level.nodes.is_empty()
                || level.nodes.windows(2).any(|pair| pair[0] >= pair[1])
                || level
                    .nodes
                    .iter()
                    .any(|node| level_by_node.insert(node.clone(), wave_index).is_some())
            {
                return Err("graph.scheduler.layout");
            }
        }
        if level_by_node.keys().cloned().collect::<BTreeSet<_>>() != schedule_nodes {
            return Err("graph.scheduler.layout");
        }

        let mut membership = BTreeMap::new();
        for (bank_index, bank) in plan.banks.iter().enumerate() {
            let members: Box<[_]> = bank
                .members
                .iter()
                .cloned()
                .map(GraphNodeId::Effect)
                .collect();
            add_native_bank_membership(
                &mut membership,
                &level_by_node,
                members,
                NativeUnitBlueprintKind::EffectBank(bank_index),
            )?;
        }
        for (bank_index, bank) in plan.builtin_banks.iter().enumerate() {
            add_native_bank_membership(
                &mut membership,
                &level_by_node,
                bank.members.clone(),
                NativeUnitBlueprintKind::BuiltinBank(bank_index),
            )?;
        }

        let mut locations = BTreeMap::new();
        let mut waves = Vec::with_capacity(plan.dependency_levels.len());
        let mut largest_wave_width = 0_usize;
        let mut unit_count = 0_usize;
        let mut partition_count = 0_usize;
        let mut bank_member_count = 0_usize;
        for (wave_index, level) in plan.dependency_levels.iter().enumerate() {
            let mut units = Vec::new();
            for node in &level.nodes {
                if let Some(bank) = membership.get(node) {
                    if *node == bank.key {
                        bank_member_count = bank_member_count
                            .checked_add(bank.members.len())
                            .ok_or("graph.scheduler.resource")?;
                        units.push(NativeUnitBlueprint {
                            key: bank.key.clone(),
                            members: bank.members.clone(),
                            kind: bank.kind,
                        });
                    }
                } else {
                    units.push(NativeUnitBlueprint {
                        key: node.clone(),
                        members: vec![node.clone()].into_boxed_slice(),
                        kind: NativeUnitBlueprintKind::Node,
                    });
                }
            }
            units.sort_by(|left, right| left.key.cmp(&right.key));
            if units.is_empty() || units.windows(2).any(|pair| pair[0].key >= pair[1].key) {
                return Err("graph.scheduler.layout");
            }
            let ranges = partition_stable_units_v1(
                core::num::NonZeroUsize::new(units.len()).ok_or("graph.scheduler.layout")?,
                config.scheduler.render_lanes,
            );
            for range in &ranges {
                for (local_unit, unit) in units[range.first_unit..range.end_unit].iter().enumerate()
                {
                    for (member, node) in unit.members.iter().enumerate() {
                        if locations
                            .insert(
                                node.clone(),
                                NativeNodeLocation {
                                    wave: wave_index,
                                    partition: range.partition_id,
                                    unit: local_unit,
                                    member,
                                },
                            )
                            .is_some()
                        {
                            return Err("graph.scheduler.layout");
                        }
                    }
                }
            }
            largest_wave_width = largest_wave_width.max(units.len());
            unit_count = unit_count
                .checked_add(units.len())
                .ok_or("graph.scheduler.resource")?;
            partition_count = partition_count
                .checked_add(ranges.len())
                .ok_or("graph.scheduler.resource")?;
            waves.push(NativeWaveBlueprint {
                level: level.level,
                units: units.into_boxed_slice(),
                partitions: ranges,
            });
        }

        let mut edge_index_by_node: BTreeMap<GraphNodeId, usize> = BTreeMap::new();
        let mut stage_operations = vec![Vec::new(); waves.len()];
        for edge in &plan.spec.edges {
            let source = *locations
                .get(&edge.source.node)
                .ok_or("graph.scheduler.layout")?;
            let destination = *locations
                .get(&edge.destination.node)
                .ok_or("graph.scheduler.layout")?;
            if source.wave >= destination.wave {
                return Err("graph.scheduler.layout");
            }
            let destination_edge = edge_index_by_node
                .entry(edge.destination.node.clone())
                .or_default();
            stage_operations[destination.wave].push(NativeStageOperation {
                source,
                destination,
                destination_edge: *destination_edge,
            });
            *destination_edge = destination_edge
                .checked_add(1)
                .ok_or("graph.scheduler.resource")?;
        }
        let observation_order: Box<[Box<[NativeNodeLocation]>]> = plan
            .dependency_levels
            .iter()
            .map(|level| level.nodes.iter().map(|node| locations[node]).collect())
            .collect();
        let output_node = plan
            .sequential_schedule
            .iter()
            .find(|node| matches!(node, GraphNodeId::Output { .. }))
            .ok_or("graph.scheduler.layout")?;
        let output = locations[output_node];
        let graph_job_bytes = native_graph_job_bytes(
            plan,
            unit_count,
            partition_count,
            bank_member_count,
            &stage_operations,
            runtime_observer_count,
        )?;
        if graph_job_bytes > config.maximum_retained_bytes {
            return Err("graph.scheduler.cap");
        }
        Ok(Self {
            waves: waves.into_boxed_slice(),
            stage_operations: stage_operations
                .into_iter()
                .map(Vec::into_boxed_slice)
                .collect(),
            observation_order,
            output,
            largest_wave_width,
            unit_count,
            partition_count,
            graph_job_bytes,
        })
    }

    #[cfg(feature = "test-support")]
    fn test_preparation_transcript(&self) -> NativeGraphPreparationTranscriptV1 {
        let mut hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut retained_bank_units = 0_usize;
        let mut retained_bank_members = 0_usize;
        let mut retained_builtin_bank_units = 0_usize;
        let mut retained_builtin_bank_members = 0_usize;
        let mut partitions_are_canonical = true;
        for wave in &self.waves {
            hash = test_transcript_u64(hash, wave.level);
            hash = test_transcript_usize(hash, wave.units.len());
            for unit in &wave.units {
                let tag = match unit.kind {
                    NativeUnitBlueprintKind::Node => 1,
                    NativeUnitBlueprintKind::EffectBank(_) => 2,
                    NativeUnitBlueprintKind::BuiltinBank(_) => 3,
                };
                hash = test_transcript_byte(hash, tag);
                hash = test_transcript_node(hash, &unit.key);
                hash = test_transcript_usize(hash, unit.members.len());
                for member in &unit.members {
                    hash = test_transcript_node(hash, member);
                }
                if !matches!(unit.kind, NativeUnitBlueprintKind::Node) {
                    retained_bank_units = retained_bank_units.saturating_add(1);
                    retained_bank_members =
                        retained_bank_members.saturating_add(unit.members.len());
                }
                if matches!(unit.kind, NativeUnitBlueprintKind::BuiltinBank(_)) {
                    retained_builtin_bank_units = retained_builtin_bank_units.saturating_add(1);
                    retained_builtin_bank_members =
                        retained_builtin_bank_members.saturating_add(unit.members.len());
                }
            }
            hash = test_transcript_usize(hash, wave.partitions.len());
            let mut expected_first_unit = 0_usize;
            for partition in &wave.partitions {
                hash = test_transcript_usize(hash, partition.partition_id);
                hash = test_transcript_usize(hash, partition.first_unit);
                hash = test_transcript_usize(hash, partition.end_unit);
                partitions_are_canonical &= partition.first_unit == expected_first_unit
                    && partition.end_unit > partition.first_unit;
                expected_first_unit = partition.end_unit;
            }
            partitions_are_canonical &= expected_first_unit == wave.units.len();
        }
        NativeGraphPreparationTranscriptV1 {
            hash,
            largest_wave_width: self.largest_wave_width,
            retained_bank_units,
            retained_bank_members,
            retained_builtin_bank_units,
            retained_builtin_bank_members,
            partitions_are_canonical,
        }
    }
}

#[cfg(feature = "test-support")]
fn test_transcript_byte(hash: u64, value: u8) -> u64 {
    (hash ^ u64::from(value)).wrapping_mul(0x0000_0100_0000_01b3)
}

#[cfg(feature = "test-support")]
fn test_transcript_u64(mut hash: u64, value: u64) -> u64 {
    for byte in value.to_le_bytes() {
        hash = test_transcript_byte(hash, byte);
    }
    hash
}

#[cfg(feature = "test-support")]
fn test_transcript_usize(hash: u64, value: usize) -> u64 {
    test_transcript_u64(hash, value as u64)
}

#[cfg(feature = "test-support")]
fn test_transcript_bytes(mut hash: u64, value: &[u8]) -> u64 {
    hash = test_transcript_usize(hash, value.len());
    for byte in value {
        hash = test_transcript_byte(hash, *byte);
    }
    hash
}

#[cfg(feature = "test-support")]
fn test_transcript_node(hash: u64, node: &GraphNodeId) -> u64 {
    match node {
        GraphNodeId::TrackStage { track_id, stage } => {
            let hash = test_transcript_byte(hash, 1);
            let hash = test_transcript_bytes(hash, track_id.as_str().as_bytes());
            test_transcript_byte(hash, *stage as u8)
        }
        GraphNodeId::Effect(effect) => {
            let hash = test_transcript_byte(hash, 2);
            let hash = test_transcript_bytes(hash, effect.track_id.as_str().as_bytes());
            let hash = test_transcript_byte(hash, effect.rack as u8);
            test_transcript_bytes(hash, effect.effect_id.as_str().as_bytes())
        }
        GraphNodeId::Route { route_id } => {
            test_transcript_bytes(test_transcript_byte(hash, 3), route_id.as_str().as_bytes())
        }
        GraphNodeId::Submix { submix_id } => {
            test_transcript_bytes(test_transcript_byte(hash, 4), submix_id.as_str().as_bytes())
        }
        GraphNodeId::Output { output_id } => {
            test_transcript_bytes(test_transcript_byte(hash, 5), output_id.as_str().as_bytes())
        }
        GraphNodeId::CompensationDelay { edge_id } => {
            test_transcript_edge(test_transcript_byte(hash, 6), edge_id)
        }
    }
}

#[cfg(feature = "test-support")]
fn test_transcript_edge(hash: u64, edge: &GraphEdgeId) -> u64 {
    match edge {
        GraphEdgeId::TrackMain { target } => {
            test_transcript_node(test_transcript_byte(hash, 1), target)
        }
        GraphEdgeId::RouteSource { route_id } => {
            test_transcript_bytes(test_transcript_byte(hash, 2), route_id.as_str().as_bytes())
        }
        GraphEdgeId::RouteDestination { route_id } => {
            test_transcript_bytes(test_transcript_byte(hash, 3), route_id.as_str().as_bytes())
        }
        GraphEdgeId::EffectSidechain { effect, port } => {
            let hash = test_transcript_byte(hash, 4);
            let hash = test_transcript_bytes(hash, effect.track_id.as_str().as_bytes());
            let hash = test_transcript_byte(hash, effect.rack as u8);
            let hash = test_transcript_bytes(hash, effect.effect_id.as_str().as_bytes());
            test_transcript_bytes(hash, port.as_bytes())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn add_native_bank_membership(
    membership: &mut BTreeMap<GraphNodeId, NativeBankMembership>,
    level_by_node: &BTreeMap<GraphNodeId, usize>,
    members: Box<[GraphNodeId]>,
    kind: NativeUnitBlueprintKind,
) -> Result<(), &'static str> {
    let key = members
        .iter()
        .min()
        .cloned()
        .ok_or("graph.scheduler.layout")?;
    let level = level_by_node
        .get(&key)
        .copied()
        .ok_or("graph.scheduler.layout")?;
    if members.iter().any(|member| {
        level_by_node.get(member).copied() != Some(level) || membership.contains_key(member)
    }) {
        return Err("graph.scheduler.layout");
    }
    let record = NativeBankMembership {
        key,
        members: members.clone(),
        kind,
    };
    for member in members {
        membership.insert(member, record.clone());
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn native_graph_job_bytes(
    plan: &PreparedGraphPlan,
    unit_count: usize,
    partition_count: usize,
    bank_member_count: usize,
    stage_operations: &[Vec<NativeStageOperation>],
    runtime_observer_count: usize,
) -> Result<usize, &'static str> {
    fn add(total: &mut usize, count: usize, size: usize) -> Result<(), &'static str> {
        *total = total
            .checked_add(count.checked_mul(size).ok_or("graph.scheduler.resource")?)
            .ok_or("graph.scheduler.resource")?;
        Ok(())
    }
    let frames = plan.envelope.quantum.0 as usize;
    let mut total = 0_usize;
    add(
        &mut total,
        plan.sequential_schedule.len(),
        2_usize
            .checked_mul(frames)
            .and_then(|samples| samples.checked_mul(core::mem::size_of::<f32>()))
            .ok_or("graph.scheduler.resource")?,
    )?;
    add(
        &mut total,
        plan.spec.edges.len(),
        2_usize
            .checked_mul(frames)
            .and_then(|samples| samples.checked_mul(core::mem::size_of::<f32>()))
            .ok_or("graph.scheduler.resource")?,
    )?;
    let delay_samples = plan
        .inserted_delays
        .iter()
        .try_fold(0_usize, |sum, delay| {
            sum.checked_add(delay.samples.0 as usize)
                .ok_or("graph.scheduler.resource")
        })?;
    add(&mut total, delay_samples, 2 * core::mem::size_of::<f32>())?;
    let reduction_slots = plan.spec.nodes.iter().try_fold(0_usize, |sum, node| {
        let inputs = plan
            .spec
            .edges
            .iter()
            .filter(|edge| {
                edge.destination.node == node.id
                    && edge.destination.kind != GraphPortKind::SidechainInput
            })
            .count();
        sum.checked_add(inputs).ok_or("graph.scheduler.resource")
    })?;
    add(&mut total, reduction_slots, core::mem::size_of::<f32>())?;
    add(
        &mut total,
        plan.spec.edges.len(),
        core::mem::size_of::<NativeRuntimeEdge>(),
    )?;
    add(
        &mut total,
        unit_count,
        core::mem::size_of::<NativeGraphUnit>(),
    )?;
    add(
        &mut total,
        bank_member_count,
        core::mem::size_of::<NativeRuntimeNode>(),
    )?;
    add(
        &mut total,
        partition_count,
        core::mem::size_of::<RenderPartitionV1<NativeGraphPartitionJob>>()
            + core::mem::size_of::<miso_engine_native_scheduler::RenderPartitionRangeV1>(),
    )?;
    add(
        &mut total,
        plan.dependency_levels.len(),
        core::mem::size_of::<RenderWaveV1<NativeGraphPartitionJob>>(),
    )?;
    add(
        &mut total,
        stage_operations.iter().map(Vec::len).sum(),
        core::mem::size_of::<NativeStageOperation>(),
    )?;
    add(
        &mut total,
        plan.sequential_schedule.len(),
        core::mem::size_of::<NativeNodeLocation>(),
    )?;
    add(
        &mut total,
        plan.observers
            .len()
            .checked_add(runtime_observer_count)
            .ok_or("graph.scheduler.resource")?,
        core::mem::size_of::<GraphNodeObserverBinding>(),
    )?;
    Ok(total)
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeRuntimeEdge {
    sidechain: bool,
    delay: Option<CompensationDelay>,
    contribution: StereoBuffer,
}

#[cfg(not(target_arch = "wasm32"))]
enum NativeRuntimeNodeKind {
    Identity,
    Bound(Box<dyn GraphRuntimeProcessor>),
    Effect(GraphPreparedEffect),
    Route(RouteTransform),
    Reduction,
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeRuntimeNode {
    incoming: Box<[NativeRuntimeEdge]>,
    output: StereoBuffer,
    reduction_scratch: Box<[f32]>,
    kind: NativeRuntimeNodeKind,
    observers: Box<[GraphNodeObserverBinding]>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeRuntimeNode {
    fn reduce(&mut self, sanitized_samples: &mut u64) {
        let main_inputs = self.incoming.iter().filter(|edge| !edge.sidechain).count();
        for frame in 0..self.output.left.len() {
            for (slot, edge) in self
                .incoming
                .iter()
                .filter(|edge| !edge.sidechain)
                .enumerate()
            {
                self.reduction_scratch[slot] = edge.contribution.left[frame];
            }
            self.output.left[frame] = balanced_pairwise_sum(
                &mut self.reduction_scratch[..main_inputs],
                sanitized_samples,
            );
            for (slot, edge) in self
                .incoming
                .iter()
                .filter(|edge| !edge.sidechain)
                .enumerate()
            {
                self.reduction_scratch[slot] = edge.contribution.right[frame];
            }
            self.output.right[frame] = balanced_pairwise_sum(
                &mut self.reduction_scratch[..main_inputs],
                sanitized_samples,
            );
        }
    }

    fn execute(
        &mut self,
        first_sample: u64,
        sanitized_samples: &mut u64,
    ) -> Result<(), RenderError> {
        self.reduce(sanitized_samples);
        match &mut self.kind {
            NativeRuntimeNodeKind::Identity | NativeRuntimeNodeKind::Reduction => {}
            NativeRuntimeNodeKind::Bound(processor) => processor.process(GraphBindingBlock {
                left: &mut self.output.left,
                right: &mut self.output.right,
                first_sample,
            })?,
            NativeRuntimeNodeKind::Route(transform) => {
                for frame in 0..self.output.left.len() {
                    (self.output.left[frame], self.output.right[frame]) = transform.transform(
                        self.output.left[frame],
                        self.output.right[frame],
                        sanitized_samples,
                    );
                }
            }
            NativeRuntimeNodeKind::Effect(effect) => {
                let sidechain = self
                    .incoming
                    .iter()
                    .find(|edge| edge.sidechain)
                    .map(|edge| (&*edge.contribution.left, &*edge.contribution.right));
                let block = EffectProcessBlock::new(
                    &mut self.output.left,
                    &mut self.output.right,
                    sidechain,
                    first_sample,
                    &[],
                    effect.metadata.quantum,
                )
                .map_err(|_| RenderError::InvalidEnvelope)?;
                let _ = effect.processor.process(block);
            }
        }
        Ok(())
    }

    fn observe(&mut self, first_sample: u64) -> Result<(), RenderError> {
        for observer in &mut self.observers {
            observer.observer.observe(GraphObservationBlock {
                left: &self.output.left,
                right: &self.output.right,
                first_sample,
            })?;
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
enum NativeGraphUnit {
    Node(NativeRuntimeNode),
    EffectBank {
        members: Box<[NativeRuntimeNode]>,
        processor: Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>,
        scratch: miso_engine_rack::AoSoaScratch,
    },
    BuiltinBank {
        members: Box<[NativeRuntimeNode]>,
        processor: Box<dyn GraphPreparedBuiltinBankProcessor>,
        scratch: miso_engine_rack::AoSoaScratch,
    },
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphUnit {
    fn member(&self, member: usize) -> Option<&NativeRuntimeNode> {
        match self {
            Self::Node(node) => (member == 0).then_some(node),
            Self::EffectBank { members, .. } | Self::BuiltinBank { members, .. } => {
                members.get(member)
            }
        }
    }

    fn member_mut(&mut self, member: usize) -> Option<&mut NativeRuntimeNode> {
        match self {
            Self::Node(node) => (member == 0).then_some(node),
            Self::EffectBank { members, .. } | Self::BuiltinBank { members, .. } => {
                members.get_mut(member)
            }
        }
    }

    fn execute(
        &mut self,
        first_sample: u64,
        sanitized_samples: &mut u64,
    ) -> Result<(), RenderError> {
        match self {
            Self::Node(node) => node.execute(first_sample, sanitized_samples),
            Self::EffectBank {
                members,
                processor,
                scratch,
            } => {
                let frames = members
                    .first()
                    .ok_or(RenderError::InvalidEnvelope)?
                    .output
                    .left
                    .len() as u32;
                for (lane, member) in members.iter_mut().enumerate() {
                    member.reduce(sanitized_samples);
                    scratch
                        .gather_lane(lane, &member.output.left, &member.output.right, frames)
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                }
                let offsets_four = [0_u32; 5];
                let offsets_eight = [0_u32; 9];
                let offsets = if members.len() == 4 {
                    &offsets_four[..]
                } else {
                    &offsets_eight[..]
                };
                scratch
                    .process(
                        processor.as_mut(),
                        frames,
                        first_sample,
                        &[],
                        offsets,
                        false,
                    )
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                for (lane, member) in members.iter_mut().enumerate() {
                    scratch
                        .scatter_lane(
                            lane,
                            &mut member.output.left,
                            &mut member.output.right,
                            frames,
                        )
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                }
                Ok(())
            }
            Self::BuiltinBank {
                members,
                processor,
                scratch,
            } => {
                let frames = members
                    .first()
                    .ok_or(RenderError::InvalidEnvelope)?
                    .output
                    .left
                    .len() as u32;
                for (lane, member) in members.iter_mut().enumerate() {
                    member.reduce(sanitized_samples);
                    scratch
                        .gather_lane(lane, &member.output.left, &member.output.right, frames)
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                }
                let (left, right) = scratch
                    .builtin_planes_mut(frames)
                    .map_err(|_| RenderError::InvalidEnvelope)?;
                processor.process(left, right, frames, first_sample)?;
                for (lane, member) in members.iter_mut().enumerate() {
                    scratch
                        .scatter_lane(
                            lane,
                            &mut member.output.left,
                            &mut member.output.right,
                            frames,
                        )
                        .map_err(|_| RenderError::InvalidEnvelope)?;
                }
                Ok(())
            }
        }
    }

    fn qualification_counters(&self) -> [u64; 2] {
        match self {
            Self::BuiltinBank { processor, .. } => processor.qualification_counters(),
            Self::Node(_) | Self::EffectBank { .. } => [0, 0],
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphPartitionJob {
    units: Box<[NativeGraphUnit]>,
    first_sample: u64,
    sanitized_samples: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphPartitionJob {
    fn node(&self, location: NativeNodeLocation) -> Option<&NativeRuntimeNode> {
        self.units.get(location.unit)?.member(location.member)
    }

    fn node_mut(&mut self, location: NativeNodeLocation) -> Option<&mut NativeRuntimeNode> {
        self.units
            .get_mut(location.unit)?
            .member_mut(location.member)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeSchedulerJobV1 for NativeGraphPartitionJob {
    type Error = RenderError;

    fn execute(&mut self) -> Result<(), Self::Error> {
        for unit in &mut self.units {
            unit.execute(self.first_sample, &mut self.sanitized_samples)?;
        }
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphExecutor {
    waves: Box<[RenderWaveV1<NativeGraphPartitionJob>]>,
    stage_operations: Box<[Box<[NativeStageOperation]>]>,
    observation_order: Box<[Box<[NativeNodeLocation]>]>,
    output: NativeNodeLocation,
    scheduler: NativeSchedulerV1<NativeGraphPartitionJob>,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphExecutor {
    fn new(
        graph: PreparedGraphPlan,
        bindings: Vec<GraphNodeBinding>,
        blueprint: NativeGraphBlueprint,
        scheduler: NativeSchedulerV1<NativeGraphPartitionJob>,
        resources: NativeGraphResourceReportV1,
        #[cfg(feature = "test-support")]
        test_preparation_transcript: NativeGraphPreparationTranscriptV1,
    ) -> (Self, NativeGraphPreparedMetadataV1) {
        let PreparedGraphPlan {
            spec,
            inserted_delays,
            routes,
            effects,
            banks,
            builtin_banks,
            observers,
            envelope,
            ..
        } = graph;
        let frames = envelope.quantum.0 as usize;
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
        let mut observers_by_node: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for observer in observers {
            observers_by_node
                .entry(observer.node.clone())
                .or_default()
                .push(observer);
        }
        for values in observers_by_node.values_mut() {
            values.sort_by_key(|observer| observer.handle);
        }
        let mut incoming_by_node: BTreeMap<_, Vec<_>> = spec
            .nodes
            .iter()
            .map(|node| (node.id.clone(), Vec::new()))
            .collect();
        for edge in spec.edges {
            incoming_by_node
                .get_mut(&edge.destination.node)
                .expect("validated native destination")
                .push(edge);
        }
        let mut banks: Vec<_> = banks.into_iter().map(Some).collect();
        let mut builtin_banks: Vec<_> = builtin_banks.into_iter().map(Some).collect();
        let mut rendered_waves = Vec::with_capacity(blueprint.waves.len());
        for wave in blueprint.waves {
            let mut units: Vec<Option<NativeGraphUnit>> = wave
                .units
                .into_vec()
                .into_iter()
                .map(|unit| {
                    let prepared = match unit.kind {
                        NativeUnitBlueprintKind::Node => NativeGraphUnit::Node(build_native_node(
                            &unit.members[0],
                            false,
                            frames,
                            &mut incoming_by_node,
                            &delays,
                            &mut routes,
                            &mut effects,
                            &mut bindings,
                            &mut observers_by_node,
                        )),
                        NativeUnitBlueprintKind::EffectBank(index) => {
                            let bank = banks[index]
                                .take()
                                .expect("validated effect bank ownership");
                            let members = unit
                                .members
                                .iter()
                                .map(|member| {
                                    build_native_node(
                                        member,
                                        true,
                                        frames,
                                        &mut incoming_by_node,
                                        &delays,
                                        &mut routes,
                                        &mut effects,
                                        &mut bindings,
                                        &mut observers_by_node,
                                    )
                                })
                                .collect();
                            NativeGraphUnit::EffectBank {
                                members,
                                processor: bank.processor,
                                scratch: bank.scratch,
                            }
                        }
                        NativeUnitBlueprintKind::BuiltinBank(index) => {
                            let bank = builtin_banks[index]
                                .take()
                                .expect("validated builtin bank ownership");
                            let members = unit
                                .members
                                .iter()
                                .map(|member| {
                                    build_native_node(
                                        member,
                                        true,
                                        frames,
                                        &mut incoming_by_node,
                                        &delays,
                                        &mut routes,
                                        &mut effects,
                                        &mut bindings,
                                        &mut observers_by_node,
                                    )
                                })
                                .collect();
                            NativeGraphUnit::BuiltinBank {
                                members,
                                processor: bank.processor,
                                scratch: bank.scratch,
                            }
                        }
                    };
                    Some(prepared)
                })
                .collect();
            let partitions = wave
                .partitions
                .iter()
                .map(|range| {
                    let partition_units = (range.first_unit..range.end_unit)
                        .map(|index| units[index].take().expect("one stable unit owner"))
                        .collect();
                    RenderPartitionV1::new(
                        *range,
                        NativeGraphPartitionJob {
                            units: partition_units,
                            first_sample: 0,
                            sanitized_samples: 0,
                        },
                    )
                })
                .collect();
            rendered_waves.push(
                RenderWaveV1::new(wave.level, partitions).expect("validated native wave layout"),
            );
        }
        debug_assert!(incoming_by_node.values().all(Vec::is_empty));
        debug_assert!(bindings.is_empty());
        debug_assert!(observers_by_node.values().all(Vec::is_empty));
        let metadata = NativeGraphPreparedMetadataV1 {
            selection: scheduler.selection(),
            resources,
            #[cfg(feature = "test-support")]
            test_preparation_transcript,
        };
        (
            Self {
                waves: rendered_waves.into_boxed_slice(),
                stage_operations: blueprint.stage_operations,
                observation_order: blueprint.observation_order,
                output: blueprint.output,
                scheduler,
            },
            metadata,
        )
    }

    fn stage_wave(&mut self, wave_index: usize, first_sample: u64) -> Result<(), RenderError> {
        let (previous, current_and_later) = self.waves.split_at_mut(wave_index);
        let current = current_and_later
            .first_mut()
            .ok_or(RenderError::InvalidEnvelope)?;
        for parcel in current.recovered_parcels_mut() {
            parcel.first_sample = first_sample;
        }
        for operation in self.stage_operations[wave_index].iter().copied() {
            let source = previous
                .get(operation.source.wave)
                .and_then(|wave| wave.recovered_parcel(operation.source.partition))
                .and_then(|parcel| parcel.node(operation.source))
                .ok_or(RenderError::InvalidEnvelope)?;
            let destination = current
                .recovered_parcel_mut(operation.destination.partition)
                .and_then(|parcel| parcel.node_mut(operation.destination))
                .ok_or(RenderError::InvalidEnvelope)?;
            let edge = destination
                .incoming
                .get_mut(operation.destination_edge)
                .ok_or(RenderError::InvalidEnvelope)?;
            edge.contribution.left.copy_from_slice(&source.output.left);
            edge.contribution
                .right
                .copy_from_slice(&source.output.right);
            if let Some(delay) = &mut edge.delay {
                delay.process(&mut edge.contribution.left, &mut edge.contribution.right);
            }
        }
        Ok(())
    }

    fn observe_wave(&mut self, wave_index: usize, first_sample: u64) -> Result<(), RenderError> {
        for location in self.observation_order[wave_index].iter().copied() {
            self.waves[wave_index]
                .recovered_parcel_mut(location.partition)
                .and_then(|parcel| parcel.node_mut(location))
                .ok_or(RenderError::InvalidEnvelope)?
                .observe(first_sample)?;
        }
        Ok(())
    }

    fn node(&self, location: NativeNodeLocation) -> Option<&NativeRuntimeNode> {
        self.waves
            .get(location.wave)?
            .recovered_parcel(location.partition)?
            .node(location)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl PreparedPlanExecutor for NativeGraphExecutor {
    // REALTIME_POLICY_BEGIN
    fn render(
        &mut self,
        _arena: &mut BufferArena,
        _input: Option<PlanarBufferRef<'_>>,
        mut output: PlanarBufferMut<'_>,
        time: miso_engine_core::realtime::RenderTime,
    ) -> Result<(), RenderError> {
        for wave_index in 0..self.waves.len() {
            self.stage_wave(wave_index, time.absolute_sample)?;
            match self.scheduler.render_wave(&mut self.waves[wave_index]) {
                Ok(_) => {}
                Err(SchedulerDispatchErrorV1::Job(error)) => return Err(error.error),
                Err(
                    SchedulerDispatchErrorV1::MissingParcel { .. }
                    | SchedulerDispatchErrorV1::CommandQueueFull { .. }
                    | SchedulerDispatchErrorV1::CompletionMismatch { .. },
                ) => return Err(RenderError::InvalidEnvelope),
            }
            self.observe_wave(wave_index, time.absolute_sample)?;
        }
        let rendered = self.node(self.output).ok_or(RenderError::InvalidEnvelope)?;
        output.plane_mut(0)?.copy_from_slice(&rendered.output.left);
        output.plane_mut(1)?.copy_from_slice(&rendered.output.right);
        Ok(())
    }
    // REALTIME_POLICY_END

    fn qualification_counters(&self) -> [u64; 2] {
        self.waves.iter().fold([0_u64, 0_u64], |mut total, wave| {
            for parcel in wave.recovered_parcels() {
                for unit in &parcel.units {
                    let counters = unit.qualification_counters();
                    total[0] = total[0].saturating_add(counters[0]);
                    total[1] = total[1].saturating_add(counters[1]);
                }
            }
            total
        })
    }

    fn copy_worker_audit_snapshots(
        &self,
        output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
    ) -> usize {
        self.scheduler.copy_worker_audit_snapshots(output)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[allow(clippy::too_many_arguments)]
fn build_native_node(
    node_id: &GraphNodeId,
    bank_member: bool,
    frames: usize,
    incoming_by_node: &mut BTreeMap<GraphNodeId, Vec<GraphEdge>>,
    delays: &BTreeMap<GraphEdgeId, u64>,
    routes: &mut BTreeMap<GraphNodeId, RouteTransform>,
    effects: &mut BTreeMap<GraphNodeId, GraphPreparedEffect>,
    bindings: &mut BTreeMap<GraphNodeId, Box<dyn GraphRuntimeProcessor>>,
    observers: &mut BTreeMap<GraphNodeId, Vec<GraphNodeObserverBinding>>,
) -> NativeRuntimeNode {
    let incoming: Vec<_> = incoming_by_node
        .remove(node_id)
        .expect("validated native node")
        .into_iter()
        .map(|edge| NativeRuntimeEdge {
            sidechain: edge.destination.kind == GraphPortKind::SidechainInput,
            delay: delays
                .get(&edge.id)
                .copied()
                .filter(|samples| *samples != 0)
                .map(|samples| CompensationDelay::new(samples as usize)),
            contribution: StereoBuffer::new(frames),
        })
        .collect();
    let main_inputs = incoming.iter().filter(|edge| !edge.sidechain).count();
    let kind = if bank_member {
        NativeRuntimeNodeKind::Identity
    } else if let Some(processor) = bindings.remove(node_id) {
        NativeRuntimeNodeKind::Bound(processor)
    } else if let Some(effect) = effects.remove(node_id) {
        NativeRuntimeNodeKind::Effect(effect)
    } else if let Some(transform) = routes.remove(node_id) {
        NativeRuntimeNodeKind::Route(transform)
    } else if matches!(
        node_id,
        GraphNodeId::Submix { .. } | GraphNodeId::Output { .. }
    ) {
        NativeRuntimeNodeKind::Reduction
    } else {
        NativeRuntimeNodeKind::Identity
    };
    NativeRuntimeNode {
        incoming: incoming.into_boxed_slice(),
        output: StereoBuffer::new(frames),
        reduction_scratch: vec![0.0; main_inputs].into_boxed_slice(),
        kind,
        observers: observers
            .remove(node_id)
            .unwrap_or_default()
            .into_boxed_slice(),
    }
}

pub fn quantum_samples(quantum: QuantumFrames, count: u64) -> Option<u64> {
    u64::from(quantum.0).checked_mul(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_core::LAUNCH_SAMPLE_RATES;
    use miso_engine_effect_contract::{
        EffectDescriptorV1, EffectId, EffectQuality, InitialParameterValue, LinkMode, LinkModeSet,
        NativeEffectFactory, ParameterChannel, PortDescriptorV1, PortId, PortLayout, PortRole,
        PrepareEffectLimits, PrepareEffectRequest, PreparedPortsV1, PreparedSidechainPort,
        ProcessReport, ResetKind, StatePayloadError, StatePayloadInput, StatePayloadOutput,
        StatePayloadSizes,
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

    const SUM_ID: EffectId = match EffectId::new("sidechain-sum") {
        Ok(value) => value,
        Err(_) => panic!("ID"),
    };
    const SUM_MAIN_IN: PortId = match PortId::new("main-in") {
        Ok(value) => value,
        Err(_) => panic!("ID"),
    };
    const SUM_MAIN_OUT: PortId = match PortId::new("main-out") {
        Ok(value) => value,
        Err(_) => panic!("ID"),
    };
    const SUM_SIDECHAIN: PortId = match PortId::new("sidechain-in") {
        Ok(value) => value,
        Err(_) => panic!("ID"),
    };
    static SUM_PORTS: [PortDescriptorV1; 3] = [
        PortDescriptorV1 {
            id: SUM_MAIN_IN,
            role: PortRole::MainInput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptorV1 {
            id: SUM_MAIN_OUT,
            role: PortRole::MainOutput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptorV1 {
            id: SUM_SIDECHAIN,
            role: PortRole::SidechainInput,
            required: false,
            layout: PortLayout::DualMonoPlanar,
        },
    ];
    static SUM_DESCRIPTOR: EffectDescriptorV1 = EffectDescriptorV1 {
        id: SUM_ID,
        display_name: "Sidechain sum fixture",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        parameters: &[],
        ports: &SUM_PORTS,
        qualities: &[],
    };

    struct SidechainSum {
        metadata: PreparedEffectMetadata,
    }
    impl PreparedNativeEffect for SidechainSum {
        fn metadata(&self) -> PreparedEffectMetadata {
            self.metadata
        }
        fn reset(&mut self, _kind: ResetKind) {}
        fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
            let (side_left, side_right) = block.sidechain.expect("fixture sidechain");
            for frame in 0..block.left.len() {
                block.left[frame] += side_left[frame];
                block.right[frame] += side_right[frame];
            }
            ProcessReport::default()
        }
        fn snapshot_state_payload(
            &self,
            _output: StatePayloadOutput<'_>,
        ) -> Result<(), StatePayloadError> {
            Ok(())
        }
        fn restore_state_payload(
            &mut self,
            _state_layout_version: u32,
            _input: StatePayloadInput<'_>,
        ) -> Result<(), StatePayloadError> {
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
            builtin_bank_bytes: 0,
            builtin_bank_scratch_bytes: 0,
            builtin_bank_count: 0,
            largest_allocation_bytes: 0,
            incremental_plan_bytes: 0,
            session_plus_plan_bytes: 0,
        }
    }

    #[test]
    fn builtin_bank_resource_overflow_leaves_the_graph_estimate_unchanged() {
        let mut estimate = empty_estimate();
        estimate.builtin_bank_bytes = 1;
        estimate.audio_buffer_samples = 7;
        estimate.incremental_plan_bytes = 11;
        estimate.session_plus_plan_bytes = 13;
        let before = estimate.clone();
        assert_eq!(
            estimate.checked_add_builtin_banks(GraphBuiltinBankResourceEstimate {
                bank_count: 1,
                payload_bytes: u64::MAX,
                scratch_bytes: 16,
                scratch_samples: 4,
                metadata_bytes: 8,
                largest_allocation_bytes: 16,
            }),
            None
        );
        assert_eq!(
            estimate, before,
            "overflow cannot partially mutate the report"
        );
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
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers: Vec::new(),
            }),
            GraphRuntimeBindings {
                envelope,
                nodes: required
                    .into_iter()
                    .map(|node| GraphNodeBinding::new(node, Box::new(Noop)))
                    .collect(),
                observers: Vec::new(),
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

    #[cfg(not(target_arch = "wasm32"))]
    fn native_parallel_sum_plan(rate: u32) -> (PreparedGraphPlan, GraphRuntimeBindings) {
        let input_a = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("a").expect("ID"),
            stage: TrackStage::Input,
        };
        let input_b = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("b").expect("ID"),
            stage: TrackStage::Input,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let delayed_edge_id = GraphEdgeId::RouteSource {
            route_id: StableGraphId::parse("a").expect("route ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(rate),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let edge = |source: GraphNodeId, route: &str| GraphEdge {
            id: GraphEdgeId::RouteSource {
                route_id: StableGraphId::parse(route).expect("route ID"),
            },
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: format!("$.{route}"),
        };
        let schedule = vec![input_a.clone(), input_b.clone(), output.clone()];
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
            plan_id: u64::from(rate),
            spec: GraphSpec {
                nodes,
                ports: Vec::new(),
                edges: vec![edge(input_a.clone(), "a"), edge(input_b.clone(), "b")],
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input_a.clone(), input_b.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: vec![InsertedDelay {
                node: GraphNodeId::CompensationDelay {
                    edge_id: Box::new(delayed_edge_id.clone()),
                },
                edge_id: delayed_edge_id,
                samples: LatencySamples(2),
            }],
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: vec![input_a.clone(), input_b.clone(), output.clone()],
            routes: Vec::new(),
            effects: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(
                    input_a,
                    Box::new(OneShotSource {
                        emitted: false,
                        left: 1.0,
                        right: 2.0,
                    }),
                ),
                GraphNodeBinding::new(
                    input_b,
                    Box::new(OneShotSource {
                        emitted: false,
                        left: 3.0,
                        right: 5.0,
                    }),
                ),
                GraphNodeBinding::new(output, Box::new(Noop)),
            ],
            observers: Vec::new(),
        };
        (plan, bindings)
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_dependency_waves_match_sequential_state_and_pcm_at_launch_rates() {
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let (sequential_graph, sequential_bindings) = native_parallel_sum_plan(rate);
            let (native_graph, native_bindings) = native_parallel_sum_plan(rate);
            let mut sequential = match sequential_graph.bind(sequential_bindings) {
                Ok(plan) => plan,
                Err(_) => panic!("sequential binding failed"),
            };
            let prepared = match native_graph.bind_native(
                native_bindings,
                NativeGraphBindConfigV1 {
                    render_mode: NativeGraphRenderModeV1::DependencyWaves,
                    scheduler: NativeSchedulerConfigV1::new(
                        core::num::NonZeroUsize::new(2).expect("two lanes"),
                        true,
                    ),
                    maximum_retained_bytes: 1 << 20,
                },
            ) {
                Ok(prepared) => prepared,
                Err(failure) => panic!("native binding failed: {}", failure.code),
            };
            assert_eq!(prepared.metadata.selection, SchedulerSelectionV1::Parallel);
            assert_eq!(prepared.metadata.resources.scheduler.worker_count, 1);
            assert_eq!(prepared.metadata.resources.scheduler.wave_count, 2);
            assert_eq!(prepared.metadata.resources.scheduler.unit_count, 3);
            let mut native = prepared.into_plan();
            for block in 0..3_u64 {
                let mut sequential_pcm = [0.0_f32; 8];
                let mut native_pcm = [0.0_f32; 8];
                sequential
                    .render(
                        miso_engine_core::realtime::RenderIo {
                            input: None,
                            output: PlanarBufferMut::try_new(&mut sequential_pcm, 2, 4, 4)
                                .expect("sequential output"),
                        },
                        miso_engine_core::realtime::RenderTime {
                            absolute_sample: block * 4,
                        },
                    )
                    .expect("sequential render");
                native
                    .render(
                        miso_engine_core::realtime::RenderIo {
                            input: None,
                            output: PlanarBufferMut::try_new(&mut native_pcm, 2, 4, 4)
                                .expect("native output"),
                        },
                        miso_engine_core::realtime::RenderTime {
                            absolute_sample: block * 4,
                        },
                    )
                    .expect("native render");
                assert_eq!(
                    native_pcm.map(f32::to_bits),
                    sequential_pcm.map(f32::to_bits)
                );
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_binding_reports_fallback_and_returns_ownership_on_cap_failure() {
        let (graph, bindings) = native_parallel_sum_plan(48_000);
        let prepared = match graph.bind_native(
            bindings,
            NativeGraphBindConfigV1 {
                render_mode: NativeGraphRenderModeV1::SingleThread,
                scheduler: NativeSchedulerConfigV1::new(
                    core::num::NonZeroUsize::new(4).expect("four lanes"),
                    true,
                ),
                maximum_retained_bytes: 1 << 20,
            },
        ) {
            Ok(prepared) => prepared,
            Err(failure) => panic!("single-thread native binding failed: {}", failure.code),
        };
        assert_eq!(
            prepared.metadata.selection,
            SchedulerSelectionV1::Sequential(FallbackReasonV1::SingleThread)
        );
        assert_eq!(prepared.metadata.resources.scheduler.worker_count, 0);
        drop(prepared);

        let (graph, bindings) = native_parallel_sum_plan(48_000);
        let failure = match graph.bind_native(
            bindings,
            NativeGraphBindConfigV1 {
                render_mode: NativeGraphRenderModeV1::DependencyWaves,
                scheduler: NativeSchedulerConfigV1::new(
                    core::num::NonZeroUsize::new(2).expect("two lanes"),
                    true,
                ),
                maximum_retained_bytes: 1,
            },
        ) {
            Ok(_) => panic!("undersized cap unexpectedly accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.cap");
        assert_eq!(failure.plan.plan_id, 48_000);
        assert_eq!(failure.bindings.nodes.len(), 3);
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn native_startup_handshake_failure_returns_every_bind_input_transactionally() {
        use miso_engine_native_scheduler::SchedulerTestProtocolInjectionV1;

        let (graph, bindings) = native_parallel_sum_plan(48_000);
        let expected_nodes: Vec<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let config = NativeGraphBindConfigV1 {
            render_mode: NativeGraphRenderModeV1::DependencyWaves,
            scheduler: NativeSchedulerConfigV1::new(
                core::num::NonZeroUsize::new(4).expect("four lanes"),
                true,
            )
            .with_test_protocol_injection(
                SchedulerTestProtocolInjectionV1::StartupHandshakeFailure,
            ),
            maximum_retained_bytes: 1 << 20,
        };
        let failure = match graph.bind_native(bindings, config) {
            Ok(_) => panic!("injected startup handshake failure unexpectedly published a plan"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.worker_start");
        assert_eq!(failure.plan.plan_id, 48_000);
        assert_eq!(failure.config, config);
        assert_eq!(failure.bindings.envelope, failure.plan.envelope);
        assert!(failure.bindings.observers.is_empty());
        assert_eq!(
            failure
                .bindings
                .nodes
                .iter()
                .map(|binding| binding.node.clone())
                .collect::<Vec<_>>(),
            expected_nodes
        );

        let recovered_config = NativeGraphBindConfigV1 {
            render_mode: NativeGraphRenderModeV1::DependencyWaves,
            scheduler: NativeSchedulerConfigV1::new(
                core::num::NonZeroUsize::new(4).expect("four lanes"),
                true,
            ),
            maximum_retained_bytes: 1 << 20,
        };
        let recovered = failure
            .plan
            .bind_native(failure.bindings, recovered_config)
            .unwrap_or_else(|retry| panic!("returned inputs were not reusable: {}", retry.code));
        assert_eq!(recovered.metadata.selection, SchedulerSelectionV1::Parallel);
        assert_eq!(recovered.metadata.resources.scheduler.worker_count, 3);
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
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
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
            observers: Vec::new(),
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

    fn sidechain_pdc_plan(delay_main: bool) -> PreparedRenderPlan {
        let main_input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("main-path").expect("ID"),
            stage: TrackStage::Input,
        };
        let sidechain_input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("sidechain-path").expect("ID"),
            stage: TrackStage::Input,
        };
        let effect_id = EffectNodeId {
            track_id: StableGraphId::parse("main-path").expect("ID"),
            rack: RackId::Dynamic,
            effect_id: StableGraphId::parse("sum").expect("ID"),
        };
        let effect_node = GraphNodeId::Effect(effect_id.clone());
        let route_id = StableGraphId::parse("to-main").expect("ID");
        let route_node = GraphNodeId::Route {
            route_id: route_id.clone(),
        };
        let output_node = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let main_edge_id = GraphEdgeId::TrackMain {
            target: effect_node.clone(),
        };
        let sidechain_edge_id = GraphEdgeId::EffectSidechain {
            effect: effect_id.clone(),
            port: SUM_SIDECHAIN.as_str().to_owned(),
        };
        let main_edge = GraphEdge {
            id: main_edge_id.clone(),
            source: GraphPortId {
                node: main_input.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: effect_node.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.sidechain.main".to_owned(),
        };
        let sidechain_edge = GraphEdge {
            id: sidechain_edge_id.clone(),
            source: GraphPortId {
                node: sidechain_input.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: effect_node.clone(),
                kind: GraphPortKind::SidechainInput,
                effect_port: Some(SUM_SIDECHAIN.as_str().to_owned()),
            },
            path: "$.sidechain.aux".to_owned(),
        };
        let route_source = GraphEdge {
            id: GraphEdgeId::RouteSource {
                route_id: route_id.clone(),
            },
            source: GraphPortId {
                node: effect_node.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: route_node.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.sidechain.route".to_owned(),
        };
        let route_destination = GraphEdge {
            id: GraphEdgeId::RouteDestination {
                route_id: route_id.clone(),
            },
            source: GraphPortId {
                node: route_node.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output_node.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.sidechain.output".to_owned(),
        };
        let schedule = vec![
            main_input.clone(),
            sidechain_input.clone(),
            effect_node.clone(),
            route_node.clone(),
            output_node.clone(),
        ];
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let metadata = PreparedEffectMetadata {
            descriptor: &SUM_DESCRIPTOR,
            sample_rate: 48_000,
            quantum: 4,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPortsV1 {
                sidechain: PreparedSidechainPort::Connected {
                    id: SUM_SIDECHAIN,
                    required: false,
                },
            },
            latency: LatencySamples(0),
            tail: TailSamples::Finite(0),
            state_sizes: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_bytes: 0,
            automation_capacity: 0,
        };
        let delayed_edge = if delay_main {
            main_edge_id
        } else {
            sidechain_edge_id
        };
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: if delay_main { 91 } else { 92 },
            spec: GraphSpec {
                nodes: schedule
                    .iter()
                    .cloned()
                    .map(|id| GraphNode {
                        id,
                        latency: LatencySamples(0),
                        tail: TailSamples::Finite(0),
                    })
                    .collect(),
                ports: Vec::new(),
                edges: vec![main_edge, sidechain_edge, route_source, route_destination],
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
            required_bindings: vec![
                main_input.clone(),
                sidechain_input.clone(),
                output_node.clone(),
            ],
            routes: vec![PreparedRoute {
                node: route_node,
                transform: RouteTransform {
                    gain: 1.0,
                    ll: 1.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
            }],
            effects: vec![GraphPreparedEffect {
                id: effect_id,
                metadata,
                processor: Box::new(SidechainSum { metadata }),
            }],
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let (main_left, main_right, side_left, side_right) = if delay_main {
            (
                [1.0, 0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 10.0, 0.0],
            )
        } else {
            (
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 10.0, 0.0],
                [1.0, 0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0, 0.0],
            )
        };
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(
                    main_input,
                    Box::new(FixedSource {
                        left: main_left,
                        right: main_right,
                    }),
                ),
                GraphNodeBinding::new(
                    sidechain_input,
                    Box::new(FixedSource {
                        left: side_left,
                        right: side_right,
                    }),
                ),
                GraphNodeBinding::new(output_node, Box::new(Noop)),
            ],
            observers: Vec::new(),
        };
        match graph.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => panic!("bindings"),
        }
    }

    #[test]
    fn faster_main_and_faster_sidechain_align_on_their_typed_ports() {
        for delay_main in [true, false] {
            let mut plan = sidechain_pdc_plan(delay_main);
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
            assert_eq!(samples, [0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 20.0, 0.0]);
        }
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
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
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
            observers: Vec::new(),
        };
        match graph.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => panic!("bindings"),
        }
    }

    #[test]
    fn enabled_and_bypass_pdc_align_at_launch_rates_and_quanta() {
        for rate in LAUNCH_SAMPLE_RATES.into_iter().map(|rate| rate.0) {
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
