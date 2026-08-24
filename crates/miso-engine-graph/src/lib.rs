//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `miso-engine-graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

pub mod program;
mod runtime;

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
    LatencySamples, PreparedEffectMetadata, PreparedNativeEffect, TailSamples,
};
#[cfg(not(target_arch = "wasm32"))]
pub use miso_engine_native_scheduler::{
    FallbackReasonV1, NativeSchedulerConfigV1, NativeSchedulerResourceReportV1,
    NativeWorkerPoolConfigV1, NativeWorkerPoolShapeV1, RecoveryBudgetV1, SchedulerSelectionV1,
};
#[cfg(not(target_arch = "wasm32"))]
use miso_engine_native_scheduler::{
    NativeSchedulerJobV1, NativeSchedulerV1, NativeWorkerPoolV1, RenderPartitionRangeV1,
    RenderPartitionV1, RenderWaveV1, SchedulerDispatchErrorV1, SchedulerPrepareErrorV1,
    WorkerLeaseV1, partition_weighted_units_v1,
};
use miso_engine_rack::AoSoaScratch;

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
    /// Number of full homogeneous native-effect banks retained by the graph.
    pub effect_bank_count: u64,
    /// Exact two-plane (L and R) AoSoA scratch payload retained by native-effect banks.
    pub effect_bank_scratch_bytes: u64,
    /// Exact additional per-member output buffers required while a bank is gathered/scattered.
    pub effect_bank_runtime_buffer_bytes: u64,
    /// Checked bank/member metadata retained before render-plan binding.
    pub effect_bank_metadata_bytes: u64,
    /// Exact prepared post-input builtin bank payload retained by the graph.
    pub builtin_bank_bytes: u64,
    /// Exact AoSoA scratch payload retained by post-input builtin banks.
    pub builtin_bank_scratch_bytes: u64,
    /// Number of retained post-input builtin banks.  The last bank of a dependency level may
    /// be padded with identity lanes, so a bank holds `1..=width.lanes()` members.
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
/// The graph's frozen reduction, over one frame's worth of contributions (evidence only).
///
/// Render never calls this: it reduces whole blocks through `miso-engine-lane`'s `sum2_block` and
/// `sum_into_block`. This is the same arithmetic at one frame, exported so the summation-residual
/// fixture and the compiler's reduction tests measure the production order rather than a private
/// copy of it -- master plan #83 D9: stable edge-ID order, left-to-right, `-0.0` preserved by the
/// single-input copy (which is why the reference is `reduce`, never `fold(0.0, +)`).
#[must_use]
pub fn reduce_left_to_right(values: &[f32]) -> f32 {
    match values {
        [] => 0.0,
        [single] => *single,
        [first, second, rest @ ..] => {
            let mut left = [0.0f32];
            miso_engine_lane::kernels::sum2_block::<f32>(&mut left, &[*first], &[*second]);
            for next in rest {
                miso_engine_lane::kernels::sum_into_block::<f32>(&mut left, &[*next]);
            }
            left[0]
        }
    }
}

pub struct PreparedGraphPlan {
    /// The executable form of this plan, derived at construction (#99 F2).
    ///
    /// Not yet consumed by either executor -- that is the step this seam exists for, and #98
    /// owns the executor kernels it feeds. It is validated on every compile (see
    /// `graph_plans_always_lower_to_an_executable_program`), so the shape both executors will be
    /// rebuilt against is proven on every session the test corpus compiles, before anything
    /// depends on it.
    program: Option<program::ExecutionProgram>,
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
    /// `true` for every lane that carries a member. #96 binds only full groups, so this is all
    /// `true` today; the field exists so a padded group can be bound without a second bank shape.
    pub active_mask: Box<[bool]>,
    pub processor: Box<dyn miso_engine_effect_contract::PreparedNativeEffectBank>,
    pub scratch: AoSoaScratch,
}
/// A compiler-owned homogeneous post-input-builtin bank.  Unlike effect banks, this is a
/// fixed graph stage and therefore has no automation or sidechain surface.
///
/// Lane `l` is active if and only if `l < members.len()`; lanes `members.len()..width.lanes()`
/// are identity lanes carried by the bank kernel itself.  Membership is the mask, so no mask is
/// stored here: `members.len()` is in `1..=width.lanes()` and the executor gathers into and
/// scatters from exactly those lanes.
pub struct GraphPreparedBuiltinBank {
    pub backend: KernelBackendV1,
    pub members: Box<[GraphNodeId]>,
    pub processor: Box<dyn GraphPreparedBuiltinBankProcessor>,
    pub scratch: AoSoaScratch,
}

/// Address-free prepared builtin-bank metadata available before render binding.
///
/// Lane `l` is active if and only if `l < members.len()`; lanes `members.len()..width.lanes()`
/// are identity lanes.
pub struct GraphPreparedBuiltinBankInfo<'a> {
    pub backend: KernelBackendV1,
    pub width: miso_engine_effect_contract::BankWidth,
    pub members: &'a [GraphNodeId],
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
    /// Cumulative `[process_calls, frames_processed]` after render is disarmed.
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }
}
impl PreparedGraphPlan {
    fn has_valid_structural_layout(&self) -> bool {
        let graph_nodes: BTreeSet<_> = self.spec.nodes.iter().map(|node| node.id.clone()).collect();
        if graph_nodes.len() != self.spec.nodes.len() {
            return false;
        }

        let mut level_by_node = BTreeMap::new();
        let mut flattened = Vec::with_capacity(graph_nodes.len());
        let mut previous_level = None;
        for level in &self.dependency_levels {
            if level.nodes.is_empty()
                || previous_level.is_some_and(|previous| previous >= level.level)
                || level.nodes.windows(2).any(|pair| pair[0] >= pair[1])
                || level
                    .nodes
                    .iter()
                    .any(|node| level_by_node.insert(node.clone(), level.level).is_some())
            {
                return false;
            }
            previous_level = Some(level.level);
            flattened.extend(level.nodes.iter().cloned());
        }
        if flattened != self.sequential_schedule
            || level_by_node.keys().cloned().collect::<BTreeSet<_>>() != graph_nodes
        {
            return false;
        }

        let positions: BTreeMap<_, _> = self
            .sequential_schedule
            .iter()
            .cloned()
            .enumerate()
            .map(|(position, node)| (node, position))
            .collect();
        if self.spec.edges.iter().any(|edge| {
            match (
                level_by_node.get(&edge.source.node),
                level_by_node.get(&edge.destination.node),
            ) {
                (Some(source), Some(destination)) => source >= destination,
                _ => true,
            }
        }) {
            return false;
        }

        let effect_banks = self.banks.iter().map(|bank| {
            bank.members
                .iter()
                .cloned()
                .map(GraphNodeId::Effect)
                .collect::<Vec<_>>()
        });
        let builtin_banks = self.builtin_banks.iter().map(|bank| bank.members.to_vec());
        let mut bank_members = BTreeSet::new();
        for members in effect_banks.chain(builtin_banks) {
            if members.is_empty() || members.windows(2).any(|pair| pair[0] >= pair[1]) {
                return false;
            }
            let Some(bank_level) = level_by_node.get(&members[0]).copied() else {
                return false;
            };
            if members.iter().any(|member| {
                level_by_node.get(member).copied() != Some(bank_level)
                    || !bank_members.insert(member.clone())
            }) {
                return false;
            }
            let Some(first_member) = members
                .iter()
                .filter_map(|member| positions.get(member))
                .min()
                .copied()
            else {
                return false;
            };
            if self.spec.edges.iter().any(|edge| {
                members.contains(&edge.destination.node)
                    && positions
                        .get(&edge.source.node)
                        .is_none_or(|source| *source >= first_member)
            }) {
                return false;
            }
        }
        true
    }

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
            if bank.members.is_empty()
                || bank.members.len() > bank.scratch.width().lanes() as usize
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
    /// The lowered executable program, or `None` when the plan's schedule, levels and spec
    /// disagree (which bind-time structural validation rejects).
    #[must_use]
    pub fn program(&self) -> Option<&program::ExecutionProgram> {
        self.program.as_ref()
    }
    /// Re-derives the executable program from this plan's *current* semantic fields.
    ///
    /// [`program`](Self::program) is derived once at construction and gated on every compile
    /// (#99 F2). The schedule, levels and inserted delays are public, and the transactional bind
    /// contract hands a rejected plan back for the caller to repair and re-bind, so binding must
    /// lower the plan it now holds rather than the one the constructor saw. Lowering is a pure
    /// function of those fields, so the two can never disagree about an unmodified plan.
    fn lowered(&self) -> Option<program::ExecutionProgram> {
        let program = program::lower(
            &self.spec,
            &self.sequential_schedule,
            &self.dependency_levels,
            &self.inserted_delays,
        )
        .ok()?;
        // A node the lowering elided has no op, so a processor bound to it would never run. The
        // compiler never asks for one -- the three internal rack boundaries are not bindable
        // (`program::is_alias_candidate`) -- and a hand-built plan that does is rejected here
        // rather than silently dropping the binding.
        let elided_binding = self.required_bindings.iter().any(|node| {
            program::node_index(&self.spec, node)
                .is_some_and(|index| program.node_op[index as usize].is_none())
        });
        (!elided_binding).then_some(program)
    }
    /// The prepared route transforms, by shared reference (#99 F5).
    #[must_use]
    pub fn routes(&self) -> &[PreparedRoute] {
        &self.routes
    }
    pub fn new(parts: PreparedGraphPlanParts) -> Self {
        // #99 F2: the executable program is *derived* here, from the plan's own spec, schedule,
        // levels and PDC edges, so it cannot disagree with the semantic graph and no caller has
        // to supply or maintain it. `None` means those four disagree -- a schedule that is not
        // the concatenation of the levels, an edge running backwards, an unsorted spec -- which
        // `has_valid_structural_layout` rejects at bind time anyway. Hand-built plans in tests
        // are the only things that ever produce it, and they keep working exactly as before.
        let program = program::lower(
            &parts.spec,
            &parts.sequential_schedule,
            &parts.dependency_levels,
            &parts.inserted_delays,
        )
        .ok();
        Self {
            program,
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
    /// The failure carries every input back, which is the point; boxing it would only move the
    /// allocation onto the caller's error path.
    #[allow(clippy::result_large_err)]
    pub fn bind(
        self,
        bindings: GraphRuntimeBindings,
    ) -> Result<PreparedRenderPlan, GraphBindFailure> {
        match self.bind_optional_source_set(bindings, None) {
            Ok(plan) => Ok(plan),
            Err((plan, bindings, _, code)) => Err(GraphBindFailure {
                plan: Box::new(plan),
                bindings,
                code,
            }),
        }
    }

    /// Transactionally bind one sealed coordinator-owned source set.
    #[allow(clippy::result_large_err)]
    pub fn bind_with_source_set(
        self,
        bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
    ) -> Result<PreparedRenderPlan, GraphSourceBindFailure> {
        self.bind_optional_source_set(bindings, Some(source_set))
            .map_err(
                |(plan, bindings, source_set, code)| GraphSourceBindFailure {
                    plan: Box::new(plan),
                    bindings,
                    source_set: source_set.expect("source-set bind retains source set"),
                    code,
                },
            )
    }

    #[allow(clippy::result_large_err)]
    fn bind_optional_source_set(
        self,
        mut bindings: GraphRuntimeBindings,
        mut source_set: Option<GraphPreparedSourceSet>,
    ) -> Result<
        PreparedRenderPlan,
        (
            Self,
            GraphRuntimeBindings,
            Option<GraphPreparedSourceSet>,
            &'static str,
        ),
    > {
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
        let source_claims = source_set
            .as_ref()
            .map(GraphPreparedSourceSet::claimed_nodes)
            .unwrap_or_default();
        let source_claim_set: BTreeSet<_> = source_claims.iter().cloned().collect();
        let source_claims_valid = source_set.as_ref().is_none_or(|set| {
            set.envelope == self.envelope
                && set.is_valid()
                && source_claim_set.len() == source_claims.len()
        });
        let mut all_supplied = supplied.clone();
        all_supplied.extend(source_claim_set.iter().cloned());
        let source_overlap = supplied.iter().any(|node| source_claim_set.contains(node));
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
            || all_supplied != required
            || duplicate_binding
            || source_overlap
            || !source_claims_valid
            || !valid_observers
        {
            let envelope_mismatch = bindings.envelope != self.envelope;
            let code = if source_set.is_some()
                && (!source_claims_valid || source_overlap || all_supplied != required)
            {
                "source.graph.binding_mismatch"
            } else if !valid_observers {
                "graph.plan.observer"
            } else if envelope_mismatch {
                "graph.plan.envelope_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err((self, bindings, source_set, code));
        }
        if !self.has_valid_structural_layout() {
            return Err((self, bindings, source_set, "graph.scheduler.layout"));
        }
        let Some(program) = self.lowered() else {
            return Err((self, bindings, source_set, "graph.scheduler.layout"));
        };
        let envelope = self.envelope;
        let plan_id = self.plan_id;
        let mut plan = self;
        let observers = {
            let mut observers = core::mem::take(&mut plan.observers);
            observers.append(&mut bindings.observers);
            observers
        };
        let executor =
            GraphExecutor::new(plan, &program, bindings.nodes, observers, source_set.take());
        Ok(PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id,
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
        bindings: GraphRuntimeBindings,
        config: NativeGraphBindConfigV1,
    ) -> Result<PreparedNativeGraphPlanV1, GraphNativeBindFailure> {
        match self.bind_native_optional_source_set(bindings, config, None) {
            Ok(plan) => Ok(plan),
            Err((plan, bindings, _, config, code)) => Err(GraphNativeBindFailure {
                plan: Box::new(plan),
                bindings,
                config,
                code,
            }),
        }
    }

    /// Transactionally bind the native executor with one coordinator-owned source set.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::result_large_err)]
    pub fn bind_native_with_source_set(
        self,
        bindings: GraphRuntimeBindings,
        config: NativeGraphBindConfigV1,
        source_set: GraphPreparedSourceSet,
    ) -> Result<PreparedNativeGraphPlanV1, GraphNativeSourceBindFailure> {
        self.bind_native_optional_source_set(bindings, config, Some(source_set))
            .map_err(
                |(plan, bindings, source_set, config, code)| GraphNativeSourceBindFailure {
                    plan: Box::new(plan),
                    bindings,
                    source_set: source_set.expect("source-set bind retains source set"),
                    config,
                    code,
                },
            )
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::result_large_err)]
    fn bind_native_optional_source_set(
        self,
        mut bindings: GraphRuntimeBindings,
        config: NativeGraphBindConfigV1,
        mut source_set: Option<GraphPreparedSourceSet>,
    ) -> Result<
        PreparedNativeGraphPlanV1,
        (
            Self,
            GraphRuntimeBindings,
            Option<GraphPreparedSourceSet>,
            NativeGraphBindConfigV1,
            &'static str,
        ),
    > {
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
        let source_claims = source_set
            .as_ref()
            .map(GraphPreparedSourceSet::claimed_nodes)
            .unwrap_or_default();
        let source_claim_set: BTreeSet<_> = source_claims.iter().cloned().collect();
        let source_claims_valid = source_set.as_ref().is_none_or(|set| {
            set.envelope == self.envelope
                && set.is_valid()
                && source_claim_set.len() == source_claims.len()
        });
        let mut all_supplied = supplied.clone();
        all_supplied.extend(source_claim_set.iter().cloned());
        let source_overlap = supplied.iter().any(|node| source_claim_set.contains(node));
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
            || all_supplied != required
            || duplicate_binding
            || source_overlap
            || !source_claims_valid
            || !valid_observers
        {
            let envelope_mismatch = bindings.envelope != self.envelope;
            let code = if source_set.is_some()
                && (!source_claims_valid || source_overlap || all_supplied != required)
            {
                "source.graph.binding_mismatch"
            } else if !valid_observers {
                "graph.plan.observer"
            } else if envelope_mismatch {
                "graph.plan.envelope_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err((self, bindings, source_set, config, code));
        }
        if !self.has_valid_structural_layout() {
            return Err((self, bindings, source_set, config, "graph.scheduler.layout"));
        }
        let Some(program) = self.lowered() else {
            return Err((self, bindings, source_set, config, "graph.scheduler.layout"));
        };
        let blueprint = match NativeGraphBlueprint::prepare(
            &self,
            &program,
            config,
            bindings.observers.len(),
        ) {
            Ok(blueprint) => blueprint,
            Err(code) => {
                return Err((self, bindings, source_set, config, code));
            }
        };
        let explicit_fallback = (config.render_mode == NativeGraphRenderModeV1::SingleThread)
            .then_some(FallbackReasonV1::SingleThread);
        // Turn one render quantum into spin-iteration budgets, on the control plane. A worker
        // idles for about a quantum before parking, and the coordinator waits one quantum (never
        // less than `MINIMUM_RECOVERY_NS`) for a completion before declaring its worker dead for
        // this block. The iteration floors guard against a zero or garbage `spin_ns` measurement.
        let quantum_ns = u128::from(self.envelope.quantum.0)
            .saturating_mul(1_000_000_000)
            .checked_div(u128::from(self.envelope.sample_rate.0.max(1)))
            .unwrap_or(0);
        let spin_ns = u128::from(config.scheduler.pool.spin_ns.max(1));
        let iterations = |budget_ns: u128| -> u64 {
            u64::try_from(budget_ns / spin_ns)
                .unwrap_or(u64::MAX)
                .max(1 << 14)
        };
        let idle_spin_iterations = iterations(quantum_ns);
        let recovery_ns = match config.scheduler.recovery_deadline_ns() {
            0 => quantum_ns.max(miso_engine_native_scheduler::MINIMUM_RECOVERY_NS),
            override_ns => u128::from(override_ns),
        };
        let budget = RecoveryBudgetV1 {
            recovery_iterations: iterations(recovery_ns),
            idle_spin_iterations,
        };
        let scheduler = match NativeSchedulerV1::prepare_with_fallback(
            config.scheduler,
            blueprint.largest_wave_width,
            self.plan_id,
            budget,
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
                return Err((self, bindings, source_set, config, code));
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
            return Err((
                self,
                bindings,
                source_set,
                config,
                "graph.scheduler.resource",
            ));
        };
        if total_retained_bytes > config.maximum_retained_bytes {
            return Err((self, bindings, source_set, config, "graph.scheduler.cap"));
        }
        // A lease of the wrong shape is a control-plane mistake, not a render-time surprise.
        if let Some(lease) = &bindings.worker_lease
            && lease.worker_count() != scheduler.expected_workers()
            && scheduler.selection() == SchedulerSelectionV1::Parallel
        {
            return Err((self, bindings, source_set, config, "graph.scheduler.lease"));
        }
        let worker_lease = (scheduler.selection() == SchedulerSelectionV1::Parallel)
            .then(|| bindings.worker_lease.take())
            .flatten();
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
            &program,
            bindings.nodes,
            blueprint,
            scheduler,
            worker_lease.map(|lease| lease.0),
            idle_spin_iterations,
            resources,
            source_set.take(),
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
    /// `true` only when every wave's partitions are cost-balanced (issue 100 F2).
    ///
    /// The checkable invariant of the greedy longest-processing-time-first split: a unit is only
    /// ever placed on the least-loaded bin, whose load before that placement is at most the mean,
    /// so no bin ends heavier than `ceil(total / bins) + heaviest unit`. A count split fails this
    /// as soon as one wave mixes a wide bank with scalar tails.
    pub weighted_partitions_are_balanced: bool,
    /// Sum of every unit weight the layout balanced, folded across all waves.
    pub total_unit_weight: u64,
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

/// Transactional native source-set binding rejection returning every caller-owned input.
#[cfg(not(target_arch = "wasm32"))]
pub struct GraphNativeSourceBindFailure {
    pub plan: Box<PreparedGraphPlan>,
    pub bindings: GraphRuntimeBindings,
    pub source_set: GraphPreparedSourceSet,
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
    /// The persistent worker pool's lease, when this plan is the one that should hold it.
    ///
    /// Binding takes it; a rejected binding returns it with everything else. A plan bound without
    /// a lease renders sequentially until the block-boundary hand-over gives it one.
    #[cfg(not(target_arch = "wasm32"))]
    pub worker_lease: Option<NativeGraphWorkerLeaseV1>,
}

/// One immutable source-owned claim for a track input node.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GraphSourceInputClaim {
    pub node: GraphNodeId,
}

/// Exact source-set allocations presented to graph binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphSourceSetResourceReport {
    /// PCM retained by the source layer and already charged by the session declaration.
    pub pcm_payload_already_charged_bytes: u64,
    /// Source transfer queues, metadata, and coordinator source-plane copies.
    pub overhead_bytes: u64,
    /// Exact total engine-owned source allocation bytes.
    pub total_engine_owned_bytes: u64,
    /// Largest individual prepared source allocation.
    pub largest_allocation_bytes: u64,
}

impl GraphSourceSetResourceReport {
    fn is_consistent(self) -> bool {
        self.pcm_payload_already_charged_bytes
            .checked_add(self.overhead_bytes)
            == Some(self.total_engine_owned_bytes)
            && self.largest_allocation_bytes <= self.total_engine_owned_bytes
    }
}

/// Render-coordinator implementation behind one sealed prepared source set.
///
/// Implementors own prepared source consumers and source-plane storage. The graph invokes this
/// only on its coordinator before ordinary nodes or native dependency waves begin.
pub trait GraphPreparedSourceSetDriver: Send {
    fn claim_count(&self) -> usize;
    fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError>;
    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError>;
    fn copy_after_disarm_telemetry(&self, _output: &mut [u64]) -> usize {
        0
    }
}

/// A graph-owned, coordinator-only source-set capability.
///
/// Its claims, envelope, resource report, and driver ownership become immutable once prepared;
/// callers can only move it into a transactional graph bind.
pub struct GraphPreparedSourceSet {
    envelope: RenderEnvelope,
    claims: Box<[GraphSourceInputClaim]>,
    resources: GraphSourceSetResourceReport,
    driver: Box<dyn GraphPreparedSourceSetDriver>,
}

impl GraphPreparedSourceSet {
    #[must_use]
    pub fn new(
        envelope: RenderEnvelope,
        claims: Vec<GraphSourceInputClaim>,
        resources: GraphSourceSetResourceReport,
        driver: Box<dyn GraphPreparedSourceSetDriver>,
    ) -> Self {
        Self {
            envelope,
            claims: claims.into_boxed_slice(),
            resources,
            driver,
        }
    }

    #[must_use]
    pub fn claims(&self) -> &[GraphSourceInputClaim] {
        &self.claims
    }

    #[must_use]
    pub const fn resource_report(&self) -> GraphSourceSetResourceReport {
        self.resources
    }

    fn claimed_nodes(&self) -> Vec<GraphNodeId> {
        self.claims.iter().map(|claim| claim.node.clone()).collect()
    }

    fn is_valid(&self) -> bool {
        self.resources.is_consistent()
            && self.driver.claim_count() == self.claims.len()
            && self.claims.windows(2).all(|pair| pair[0] < pair[1])
            && self.claims.iter().all(|claim| {
                matches!(
                    claim.node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                )
            })
    }

    fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError> {
        if frames != self.envelope.quantum.0 {
            return Err(RenderError::InvalidEnvelope);
        }
        self.driver.begin_block(first_sample, frames)
    }

    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError> {
        if claim_index >= self.claims.len()
            || left.len() != self.envelope.quantum.0 as usize
            || right.len() != self.envelope.quantum.0 as usize
        {
            return Err(RenderError::InvalidEnvelope);
        }
        self.driver.copy_track_input(claim_index, left, right)
    }

    /// Copy bounded render-owner telemetry only after the prepared plan is disarmed.
    pub fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
        self.driver.copy_after_disarm_telemetry(output)
    }
}

/// Transactional source-set binding rejection returning every caller-owned input.
pub struct GraphSourceBindFailure {
    pub plan: Box<PreparedGraphPlan>,
    pub bindings: GraphRuntimeBindings,
    pub source_set: GraphPreparedSourceSet,
    pub code: &'static str,
}
pub struct GraphBindFailure {
    pub plan: Box<PreparedGraphPlan>,
    pub bindings: GraphRuntimeBindings,
    pub code: &'static str,
}
pub struct GraphNodeBinding {
    pub node: GraphNodeId,
    pub(crate) processor: Option<Box<dyn GraphRuntimeProcessor>>,
}
impl GraphNodeBinding {
    pub fn new(node: GraphNodeId, processor: Box<dyn GraphRuntimeProcessor>) -> Self {
        Self {
            node,
            processor: Some(processor),
        }
    }
    /// Acknowledge one required external node without supplying a processor.
    ///
    /// The node is still *listed* as bound, so a host that forgets a node it must bind still
    /// fails with `graph.plan.binding`; it is rendered by the executor's own reduction or identity
    /// kind instead of by a host-supplied pass-through processor. Every host that used to hand the
    /// executor a do-nothing `GraphRuntimeProcessor` for a submix or output node uses this
    /// instead, so no host defines one (audit #103 F1).
    #[must_use]
    pub fn identity(node: GraphNodeId) -> Self {
        Self {
            node,
            processor: None,
        }
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

/// The sequential executor: one coloured arena, one pass over the lowered ops.
///
/// It is a *driver* over [`runtime`], not a second implementation of anything: node semantics,
/// reductions, routes, delays and banks all live there and the native executor calls the same
/// functions (#98 F7).
struct GraphExecutor {
    runtime: runtime::Runtime,
    output: u32,
    source_set: Option<GraphPreparedSourceSet>,
    /// `(claim index, arena buffer)` for every track input the coordinator's source set fills.
    source_input_buffers: Box<[(usize, u32)]>,
}

impl GraphExecutor {
    fn new(
        plan: PreparedGraphPlan,
        program: &program::ExecutionProgram,
        bindings: Vec<GraphNodeBinding>,
        observers: Vec<GraphNodeObserverBinding>,
        source_set: Option<GraphPreparedSourceSet>,
    ) -> Self {
        let frames = plan.envelope.quantum.0 as usize;
        let source_inputs: BTreeSet<_> = source_set
            .as_ref()
            .map(|set| set.claimed_nodes().into_iter().collect())
            .unwrap_or_default();
        let source_input_buffers: Box<[(usize, u32)]> = source_set
            .as_ref()
            .map(|set| {
                set.claims()
                    .iter()
                    .enumerate()
                    .map(|(claim, entry)| {
                        let node = program::node_index(&plan.spec, &entry.node)
                            .expect("validated source claim");
                        (
                            claim,
                            program.node_buffer[node as usize].0 + runtime::ARENA_BASE,
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        // The shared arena reserves buffer zero for silence, so every coloured buffer is offset.
        let output = program.output.0 + runtime::ARENA_BASE;
        let parts = runtime::RuntimeParts::new(
            &plan.spec,
            plan.routes,
            plan.effects,
            plan.banks,
            plan.builtin_banks,
            observers,
            bindings,
            source_inputs,
        );
        let runtime = runtime::build_sequential(program, &plan.spec, parts, frames);
        Self {
            runtime,
            output,
            source_set,
            source_input_buffers,
        }
    }
}

impl PreparedPlanExecutor for GraphExecutor {
    // REALTIME_POLICY_BEGIN
    fn render(
        &mut self,
        _arena: &mut BufferArena,
        _input: Option<PlanarBufferRef<'_>>,
        mut output: PlanarBufferMut<'_>,
        time: miso_engine_core::realtime::RenderTime,
    ) -> Result<(), RenderError> {
        let Self {
            runtime,
            output: output_buffer,
            source_set,
            source_input_buffers,
        } = self;
        if let Some(source_set) = source_set {
            source_set.begin_block(time.absolute_sample, source_set.envelope.quantum.0)?;
            for &(claim, buffer) in source_input_buffers.iter() {
                let (left, right) = runtime.buffer_mut(buffer);
                source_set.copy_track_input(claim, left, right)?;
            }
        }
        for unit in 0..runtime.units.len() {
            runtime.execute(unit, time.absolute_sample)?;
            runtime.observe_unit(unit, time.absolute_sample)?;
        }
        let (left, right) = runtime.buffer(*output_buffer);
        output.plane_mut(0)?.copy_from_slice(left);
        output.plane_mut(1)?.copy_from_slice(right);
        Ok(())
    }
    // REALTIME_POLICY_END

    fn qualification_counters(&self) -> [u64; 2] {
        self.runtime.units.iter().fold([0, 0], |mut total, unit| {
            let counters = unit.qualification_counters();
            total[0] = total[0].saturating_add(counters[0]);
            total[1] = total[1].saturating_add(counters[1]);
            total
        })
    }

    fn bank_transposes(&self) -> u64 {
        self.runtime
            .units
            .iter()
            .fold(0_u64, |total, unit| total.saturating_add(unit.transposes()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Copy)]
enum NativeUnitBlueprintKind {
    Node,
    EffectBank,
    BuiltinBank,
}

/// One scheduling unit of the native layout, kept for the preparation transcript and the
/// scheduler's cost-weighted partitioning: a plain op, or a whole homogeneous bank.
#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(not(feature = "test-support"), allow(dead_code))]
struct NativeUnitBlueprint {
    key: GraphNodeId,
    members: Box<[GraphNodeId]>,
    kind: NativeUnitBlueprintKind,
    /// Prepared cost weight, the quantity the partitioner balances (issue 100 F2).
    weight: u64,
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(not(feature = "test-support"), allow(dead_code))]
struct NativeWaveBlueprint {
    level: u64,
    /// Units in the order the partitions cover them, after the weighted split.
    units: Box<[NativeUnitBlueprint]>,
    partitions: Box<[RenderPartitionRangeV1]>,
}

/// Processing stages a post-input builtin bank runs per member, for the cost weight only.
///
/// The chain is polarity/trim, high-pass, low-pass and the trim/mute application (AGENTS.md,
/// "Approved audio architecture"). The bank exposes no stage count, so this is a documented
/// estimate; it never affects rendered bits, only how a wave is split across lanes.
#[cfg(not(target_arch = "wasm32"))]
const BUILTIN_BANK_SLOTS_ESTIMATE: u64 = 4;

/// Processing stages an effect bank runs per member. `runtime::bank_chain` builds exactly one
/// [`miso_engine_rack::BankSlot`] per bank today; #96 owns the chain shape.
#[cfg(not(target_arch = "wasm32"))]
const EFFECT_BANK_SLOTS: u64 = 1;

/// The native executor's layout, derived from the lowered program (#99 F2, #98 F2).
///
/// A dependency level whose nodes are all elided stage boundaries carries no op and therefore no
/// wave; the levels that remain keep their order, so every edge still runs forwards.
#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphBlueprint {
    waves: Box<[NativeWaveBlueprint]>,
    layout: Vec<runtime::WaveLayout>,
    largest_wave_width: usize,
    unit_count: usize,
    partition_count: usize,
    graph_job_bytes: usize,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphBlueprint {
    fn prepare(
        plan: &PreparedGraphPlan,
        program: &program::ExecutionProgram,
        config: NativeGraphBindConfigV1,
        runtime_observer_count: usize,
    ) -> Result<Self, &'static str> {
        if config.maximum_retained_bytes == 0 {
            return Err("graph.scheduler.cap");
        }
        let membership = runtime::bank_membership(&plan.spec, &plan.banks, &plan.builtin_banks);
        let grouped = runtime::waves_of(program, &membership);
        if grouped.is_empty() {
            return Err("graph.scheduler.layout");
        }
        let output_node = plan
            .spec
            .nodes
            .iter()
            .find(|node| matches!(node.id, GraphNodeId::Output { .. }))
            .map(|node| node.id.clone())
            .ok_or("graph.scheduler.layout")?;
        let mut waves = Vec::with_capacity(grouped.len());
        let mut layout = Vec::with_capacity(grouped.len());
        let mut largest_wave_width = 0_usize;
        let mut unit_count = 0_usize;
        let mut partition_count = 0_usize;
        let mut bank_member_count = 0_usize;
        for (level, units) in &grouped {
            let described: Vec<NativeUnitBlueprint> = units
                .iter()
                .map(|(bank, ops)| {
                    let members: Box<[GraphNodeId]> = ops
                        .iter()
                        .map(|op| plan.spec.nodes[program.ops[*op].node as usize].id.clone())
                        .collect();
                    let key = members.iter().min().cloned().expect("unit has a member");
                    let kind = match bank {
                        None => NativeUnitBlueprintKind::Node,
                        Some(runtime::Membership::Effect(_)) => NativeUnitBlueprintKind::EffectBank,
                        Some(runtime::Membership::Builtin(_)) => {
                            NativeUnitBlueprintKind::BuiltinBank
                        }
                    };
                    // The prepared cost of one unit: how much DSP it runs, plus what its
                    // reductions have to fold in. A count split makes an eight-track bank weigh
                    // the same as a scalar tail, which is what caps the production session.
                    let width = u64::try_from(members.len()).unwrap_or(u64::MAX);
                    let slots = match kind {
                        NativeUnitBlueprintKind::EffectBank => EFFECT_BANK_SLOTS,
                        NativeUnitBlueprintKind::BuiltinBank => BUILTIN_BANK_SLOTS_ESTIMATE,
                        NativeUnitBlueprintKind::Node => u64::from(
                            members
                                .first()
                                .is_some_and(|id| matches!(id, GraphNodeId::Effect(_))),
                        ),
                    };
                    let incoming = ops
                        .iter()
                        .map(|op| u64::from(program.ops[*op].input_count()))
                        .sum::<u64>();
                    let weight = width.saturating_mul(slots).max(1).saturating_add(incoming);
                    NativeUnitBlueprint {
                        key,
                        members,
                        kind,
                        weight,
                    }
                })
                .collect();
            if described.windows(2).any(|pair| pair[0].key >= pair[1].key) {
                return Err("graph.scheduler.layout");
            }
            for unit in &described {
                if !matches!(unit.kind, NativeUnitBlueprintKind::Node) {
                    bank_member_count = bank_member_count
                        .checked_add(unit.members.len())
                        .ok_or("graph.scheduler.resource")?;
                }
            }
            if described.is_empty() {
                return Err("graph.scheduler.layout");
            }
            // The session output's unit stays in partition zero: the coordinator always owns that
            // parcel, so the host copy-out can never read a trapped worker's buffers.
            let pinned = described
                .iter()
                .position(|unit| unit.members.contains(&output_node));
            let weights: Vec<u64> = described.iter().map(|unit| unit.weight).collect();
            let split =
                partition_weighted_units_v1(&weights, config.scheduler.render_lanes, pinned);
            let mut ordered: Vec<Option<NativeUnitBlueprint>> =
                described.into_iter().map(Some).collect();
            let units_in_order: Vec<NativeUnitBlueprint> = split
                .unit_order
                .iter()
                .map(|unit| ordered[*unit].take().expect("each unit placed once"))
                .collect();
            largest_wave_width = largest_wave_width.max(units_in_order.len());
            unit_count = unit_count
                .checked_add(units_in_order.len())
                .ok_or("graph.scheduler.resource")?;
            partition_count = partition_count
                .checked_add(split.ranges.len())
                .ok_or("graph.scheduler.resource")?;
            layout.push(runtime::WaveLayout {
                unit_order: split.unit_order.to_vec(),
                ranges: split
                    .ranges
                    .iter()
                    .map(|range| (range.first_unit, range.end_unit, range.partition_id))
                    .collect(),
            });
            waves.push(NativeWaveBlueprint {
                level: *level,
                units: units_in_order.into_boxed_slice(),
                partitions: split.ranges,
            });
        }
        let graph_job_bytes = native_graph_job_bytes(
            plan,
            program,
            unit_count,
            partition_count,
            bank_member_count,
            waves.len(),
            runtime_observer_count,
        )?;
        if graph_job_bytes > config.maximum_retained_bytes {
            return Err("graph.scheduler.cap");
        }
        Ok(Self {
            waves: waves.into_boxed_slice(),
            layout,
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
        let mut weighted_partitions_are_balanced = true;
        let mut total_unit_weight = 0_u64;
        for wave in &self.waves {
            let bins = wave.partitions.len().max(1) as u64;
            let wave_total: u64 = wave.units.iter().map(|unit| unit.weight).sum();
            let heaviest = wave.units.iter().map(|unit| unit.weight).max().unwrap_or(0);
            let bound = wave_total.div_ceil(bins).saturating_add(heaviest);
            for partition in wave.partitions.iter() {
                let load: u64 = wave.units[partition.first_unit..partition.end_unit]
                    .iter()
                    .map(|unit| unit.weight)
                    .sum();
                weighted_partitions_are_balanced &= load <= bound;
            }
            total_unit_weight = total_unit_weight.saturating_add(wave_total);
        }
        for wave in &self.waves {
            hash = test_transcript_u64(hash, wave.level);
            hash = test_transcript_usize(hash, wave.units.len());
            for unit in &wave.units {
                let tag = match unit.kind {
                    NativeUnitBlueprintKind::Node => 1,
                    NativeUnitBlueprintKind::EffectBank => 2,
                    NativeUnitBlueprintKind::BuiltinBank => 3,
                };
                hash = test_transcript_byte(hash, tag);
                hash = test_transcript_node(hash, &unit.key);
                hash = test_transcript_u64(hash, unit.weight);
                hash = test_transcript_usize(hash, unit.members.len());
                for member in &unit.members {
                    hash = test_transcript_node(hash, member);
                }
                if !matches!(unit.kind, NativeUnitBlueprintKind::Node) {
                    retained_bank_units = retained_bank_units.saturating_add(1);
                    retained_bank_members =
                        retained_bank_members.saturating_add(unit.members.len());
                }
                if matches!(unit.kind, NativeUnitBlueprintKind::BuiltinBank) {
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
            weighted_partitions_are_balanced,
            total_unit_weight,
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
#[allow(clippy::too_many_arguments)]
fn native_graph_job_bytes(
    plan: &PreparedGraphPlan,
    program: &program::ExecutionProgram,
    unit_count: usize,
    partition_count: usize,
    bank_member_count: usize,
    wave_count: usize,
    runtime_observer_count: usize,
) -> Result<usize, &'static str> {
    fn add(total: &mut usize, count: usize, size: usize) -> Result<(), &'static str> {
        *total = total
            .checked_add(count.checked_mul(size).ok_or("graph.scheduler.resource")?)
            .ok_or("graph.scheduler.resource")?;
        Ok(())
    }
    let frames = plan.envelope.quantum.0 as usize;
    let stereo_block = 2_usize
        .checked_mul(frames)
        .and_then(|samples| samples.checked_mul(core::mem::size_of::<f32>()))
        .ok_or("graph.scheduler.resource")?;
    let mut total = 0_usize;
    // One plan-wide disjoint arena: the silence buffer, one unique output buffer per op, and one
    // staging buffer per delayed edge (issue 100's pull model -- an ordinary edge is read in
    // place and costs nothing).
    add(&mut total, 1, stereo_block)?;
    add(&mut total, program.ops.len(), stereo_block)?;
    add(&mut total, program.delayed_input_count(), stereo_block)?;
    let delay_samples = program.delays.iter().try_fold(0_usize, |sum, line| {
        sum.checked_add(line.samples as usize)
            .ok_or("graph.scheduler.resource")
    })?;
    add(&mut total, delay_samples, 2 * core::mem::size_of::<f32>())?;
    // The lease access map is one byte per arena buffer per parcel.
    let arena_buffers = program
        .ops
        .len()
        .checked_add(program.delayed_input_count())
        .and_then(|count| count.checked_add(1))
        .ok_or("graph.scheduler.resource")?;
    add(&mut total, partition_count, arena_buffers)?;
    add(
        &mut total,
        program.inputs.len(),
        core::mem::size_of::<u32>(),
    )?;
    add(
        &mut total,
        program.delayed_input_count(),
        core::mem::size_of::<runtime::StagedInput>(),
    )?;
    add(
        &mut total,
        program.inputs.len(),
        core::mem::size_of::<runtime::MutedRead>(),
    )?;
    add(
        &mut total,
        program.ops.len(),
        core::mem::size_of::<runtime::RuntimeOp>(),
    )?;
    add(
        &mut total,
        unit_count.saturating_add(bank_member_count),
        core::mem::size_of::<runtime::RuntimeUnit>(),
    )?;
    add(
        &mut total,
        partition_count,
        core::mem::size_of::<RenderPartitionV1<NativeGraphPartitionJob>>()
            + core::mem::size_of::<RenderPartitionRangeV1>()
            + core::mem::size_of::<runtime::Runtime>(),
    )?;
    add(
        &mut total,
        wave_count,
        core::mem::size_of::<RenderWaveV1<NativeGraphPartitionJob>>(),
    )?;
    add(
        &mut total,
        program.ops.len(),
        core::mem::size_of::<runtime::NodeLocation>(),
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

/// One partition's work: its units over its lease on the plan's shared arena.
///
/// Observers run here too, on the worker that rendered the unit (issue 100 F1): an observer is
/// invoked exactly once per block per node, and the order across nodes in different parcels is
/// unspecified. A host observer that shares state across nodes needs its own synchronisation.
#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphPartitionJob {
    runtime: runtime::Runtime,
    first_sample: u64,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeSchedulerJobV1 for NativeGraphPartitionJob {
    type Error = RenderError;

    // REALTIME_POLICY_BEGIN
    fn execute(&mut self) -> Result<(), Self::Error> {
        for unit in 0..self.runtime.units.len() {
            self.runtime.execute(unit, self.first_sample)?;
            self.runtime.observe_unit(unit, self.first_sample)?;
        }
        Ok(())
    }
    // REALTIME_POLICY_END
}

/// The native dependency-wave executor: the same [`runtime`] model, one shared disjoint arena.
#[cfg(not(target_arch = "wasm32"))]
struct NativeGraphExecutor {
    waves: Box<[RenderWaveV1<NativeGraphPartitionJob>]>,
    /// Per `(wave, partition)`, the consumer reads to mute while that partition is trapped.
    trapped_edges: Box<[runtime::WaveMutedReads]>,
    output: runtime::NodeLocation,
    scheduler: NativeSchedulerV1<NativeGraphPartitionJob>,
    lease: Option<Box<WorkerLeaseV1<NativeGraphPartitionJob>>>,
    expected_workers: usize,
    idle_spin_iterations: u64,
    /// Reap slots for `begin_block`, one per worker; preallocated at bind.
    reaped: Box<[Option<(usize, usize)>]>,
    source_set: Option<GraphPreparedSourceSet>,
    source_input_targets: Box<[(usize, runtime::NodeLocation)]>,
    counters: [u64; 4],
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphExecutor {
    #[allow(clippy::too_many_arguments)]
    fn new(
        plan: PreparedGraphPlan,
        program: &program::ExecutionProgram,
        bindings: Vec<GraphNodeBinding>,
        blueprint: NativeGraphBlueprint,
        scheduler: NativeSchedulerV1<NativeGraphPartitionJob>,
        lease: Option<WorkerLeaseV1<NativeGraphPartitionJob>>,
        idle_spin_iterations: u64,
        resources: NativeGraphResourceReportV1,
        source_set: Option<GraphPreparedSourceSet>,
        #[cfg(feature = "test-support")]
        test_preparation_transcript: NativeGraphPreparationTranscriptV1,
    ) -> (Self, NativeGraphPreparedMetadataV1) {
        let frames = plan.envelope.quantum.0 as usize;
        let source_inputs: BTreeSet<_> = source_set
            .as_ref()
            .map(|set| set.claimed_nodes().into_iter().collect())
            .unwrap_or_default();
        let parts = runtime::RuntimeParts::new(
            &plan.spec,
            plan.routes,
            plan.effects,
            plan.banks,
            plan.builtin_banks,
            plan.observers,
            bindings,
            source_inputs,
        );
        let native = runtime::build_native(program, &plan.spec, parts, frames, &blueprint.layout);
        let source_input_targets = source_set
            .as_ref()
            .map(|set| {
                set.claims()
                    .iter()
                    .enumerate()
                    .map(|(claim, entry)| (claim, native.locations[&entry.node]))
                    .collect()
            })
            .unwrap_or_default();
        let mut waves = Vec::with_capacity(native.parcels.len());
        for (wave, parcels) in native.parcels.into_iter().enumerate() {
            let partitions = blueprint.waves[wave]
                .partitions
                .iter()
                .copied()
                .zip(parcels)
                .map(|(range, runtime)| {
                    RenderPartitionV1::new(
                        range,
                        NativeGraphPartitionJob {
                            runtime,
                            first_sample: 0,
                        },
                    )
                })
                .collect();
            waves.push(
                RenderWaveV1::new(native.levels[wave], partitions)
                    .expect("validated native wave layout"),
            );
        }
        let metadata = NativeGraphPreparedMetadataV1 {
            selection: scheduler.selection(),
            resources,
            #[cfg(feature = "test-support")]
            test_preparation_transcript,
        };
        let expected_workers = scheduler.expected_workers();
        let mut lease = lease.map(Box::new);
        if let Some(lease) = lease.as_mut() {
            lease.set_idle_spin(idle_spin_iterations);
        }
        (
            Self {
                waves: waves.into_boxed_slice(),
                trapped_edges: native
                    .trapped_edges
                    .into_iter()
                    .map(|wave| {
                        wave.into_iter()
                            .map(Vec::into_boxed_slice)
                            .collect::<Box<[_]>>()
                    })
                    .collect(),
                output: native.output,
                scheduler,
                lease,
                expected_workers,
                idle_spin_iterations,
                reaped: vec![None; expected_workers].into_boxed_slice(),
                source_set,
                source_input_targets,
                counters: [0; 4],
            },
            metadata,
        )
    }

    // REALTIME_POLICY_BEGIN
    /// Redirect (or restore) every consumer read sourced from one partition.
    ///
    /// A trapped partition's buffers are radioactive: its worker may still be writing them, and
    /// nobody owns them. Muting turns those reads into the arena's always-zero silence buffer
    /// (invariant I4) so the rest of the block renders correct, defined audio.
    fn set_partition_muted(&mut self, wave: usize, partition: usize, muted: bool) {
        let Some(edges) = self
            .trapped_edges
            .get(wave)
            .and_then(|wave| wave.get(partition))
        else {
            return;
        };
        for index in 0..edges.len() {
            let edge = self.trapped_edges[wave][partition][index];
            if let Some(parcel) = self.waves[edge.wave].recovered_parcel_mut(edge.partition) {
                parcel.runtime.lease.set_muted(edge.buffer, muted);
            }
        }
    }
    // REALTIME_POLICY_END
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
        if self.scheduler.selection() == SchedulerSelectionV1::Parallel && self.lease.is_none() {
            self.counters[3] = self.counters[3].saturating_add(1);
        }
        let reaped_count = self.scheduler.begin_block(
            self.lease.as_deref_mut(),
            &mut self.waves,
            &mut self.reaped,
        );
        for index in 0..reaped_count {
            if let Some((wave, partition)) = self.reaped[index] {
                self.set_partition_muted(wave, partition, false);
            }
        }
        for wave in self.waves.iter_mut() {
            for parcel in wave.recovered_parcels_mut() {
                parcel.first_sample = time.absolute_sample;
            }
        }
        if let Some(source_set) = &mut self.source_set {
            source_set.begin_block(time.absolute_sample, source_set.envelope.quantum.0)?;
            for &(claim, location) in &self.source_input_targets {
                let (left, right) = self.waves[location.wave]
                    .recovered_parcel_mut(location.partition)
                    .ok_or(RenderError::InvalidEnvelope)?
                    .runtime
                    .buffer_mut(location.buffer);
                source_set.copy_track_input(claim, left, right)?;
            }
        }
        let mut degraded = false;
        let mut job_error = None;
        for wave_index in 0..self.waves.len() {
            let lease = if degraded {
                None
            } else {
                self.lease.as_deref_mut()
            };
            match self
                .scheduler
                .render_wave(lease, &mut self.waves[wave_index])
            {
                Ok(report) => {
                    self.counters[0] = self.counters[0].saturating_add(report.coordinator_wakes);
                    self.counters[2] =
                        self.counters[2].saturating_add(report.dead_partitions_executed);
                }
                Err(SchedulerDispatchErrorV1::WorkerLost { partition_id, .. }) => {
                    // The worker keeps the parcel; mute everything sourced from it and finish the
                    // block on the coordinator.
                    self.counters[1] = self.counters[1].saturating_add(1);
                    self.set_partition_muted(wave_index, partition_id, true);
                    degraded = true;
                }
                Err(SchedulerDispatchErrorV1::Job(error)) => {
                    job_error = Some(error.error);
                    break;
                }
                Err(
                    SchedulerDispatchErrorV1::MissingParcel { .. }
                    | SchedulerDispatchErrorV1::CommandQueueFull { .. }
                    | SchedulerDispatchErrorV1::CompletionMismatch { .. }
                    | SchedulerDispatchErrorV1::JobPanicked { .. },
                ) => {
                    job_error = Some(RenderError::InvalidEnvelope);
                    break;
                }
            }
        }
        self.scheduler.end_block(self.lease.as_deref_mut());
        if let Some(error) = job_error {
            return Err(error);
        }
        let (left, right) = self.waves[self.output.wave]
            .recovered_parcel(self.output.partition)
            .ok_or(RenderError::InvalidEnvelope)?
            .runtime
            .buffer(self.output.buffer);
        output.plane_mut(0)?.copy_from_slice(left);
        output.plane_mut(1)?.copy_from_slice(right);
        Ok(())
    }
    // REALTIME_POLICY_END

    fn take_handover(&mut self) -> Option<miso_engine_core::realtime::ExecutorHandover> {
        self.lease
            .take()
            .map(|lease| lease as miso_engine_core::realtime::ExecutorHandover)
    }

    fn accept_handover(
        &mut self,
        handover: miso_engine_core::realtime::ExecutorHandover,
    ) -> Option<miso_engine_core::realtime::ExecutorHandover> {
        if self.lease.is_some() || self.scheduler.selection() != SchedulerSelectionV1::Parallel {
            return Some(handover);
        }
        match handover.downcast::<WorkerLeaseV1<NativeGraphPartitionJob>>() {
            Ok(mut lease) => {
                if lease.worker_count() == self.expected_workers {
                    lease.set_idle_spin(self.idle_spin_iterations);
                    self.lease = Some(lease);
                    None
                } else {
                    Some(lease as miso_engine_core::realtime::ExecutorHandover)
                }
            }
            Err(handover) => Some(handover),
        }
    }

    fn dispatch_counters(&self) -> [u64; 4] {
        self.counters
    }

    fn qualification_counters(&self) -> [u64; 2] {
        self.waves.iter().fold([0_u64, 0_u64], |mut total, wave| {
            for parcel in wave.recovered_parcels() {
                for unit in parcel.runtime.units.iter() {
                    let counters = unit.qualification_counters();
                    total[0] = total[0].saturating_add(counters[0]);
                    total[1] = total[1].saturating_add(counters[1]);
                }
            }
            total
        })
    }

    fn bank_transposes(&self) -> u64 {
        self.waves.iter().fold(0_u64, |mut total, wave| {
            for parcel in wave.recovered_parcels() {
                for unit in parcel.runtime.units.iter() {
                    total = total.saturating_add(unit.transposes());
                }
            }
            total
        })
    }

    fn copy_worker_audit_snapshots(
        &self,
        output: &mut [miso_engine_core::realtime::audit::AuditSnapshot],
    ) -> usize {
        self.scheduler
            .copy_worker_audit_snapshots(self.lease.as_deref(), output)
    }
}

/// Control-plane owner of the native render workers, independent of any plan.
///
/// Hosts start one pool and keep it: a structural change publishes a replacement plan and the
/// lease is handed over at the block-boundary swap, so no thread is spawned or joined for it.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeGraphWorkerPoolV1(NativeWorkerPoolV1<NativeGraphPartitionJob>);

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphWorkerPoolV1 {
    /// Start the pool and take its first lease.
    ///
    /// # Errors
    /// Returns the scheduler's preparation error; nothing is left running on the failure path.
    pub fn start(
        config: NativeWorkerPoolConfigV1,
    ) -> Result<(Self, NativeGraphWorkerLeaseV1), SchedulerPrepareErrorV1> {
        let (pool, lease) = NativeWorkerPoolV1::start(config)?;
        Ok((Self(pool), NativeGraphWorkerLeaseV1(lease)))
    }

    /// Address-free description of this pool.
    #[must_use]
    pub const fn shape(&self) -> NativeWorkerPoolShapeV1 {
        self.0.shape()
    }

    /// Take back a lease a retired plan released.
    pub fn recover_lease(&mut self) -> Option<NativeGraphWorkerLeaseV1> {
        self.0.recover_lease().map(NativeGraphWorkerLeaseV1)
    }

    /// Stop and join every worker.
    pub fn stop_and_join(self) {
        self.0.stop_and_join();
    }
}

/// The coordinator half of a [`NativeGraphWorkerPoolV1`], owned by one prepared plan at a time.
#[cfg(not(target_arch = "wasm32"))]
pub struct NativeGraphWorkerLeaseV1(WorkerLeaseV1<NativeGraphPartitionJob>);

#[cfg(not(target_arch = "wasm32"))]
impl NativeGraphWorkerLeaseV1 {
    /// Auxiliary workers this lease drives.
    #[must_use]
    pub fn worker_count(&self) -> usize {
        self.0.worker_count()
    }
}

pub fn quantum_samples(quantum: QuantumFrames, count: u64) -> Option<u64> {
    u64::from(quantum.0).checked_mul(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::CompensationDelay;
    use miso_engine_conformance::DualAccumulatorDelayFactory;
    use miso_engine_core::LAUNCH_SAMPLE_RATES;
    use miso_engine_effect_contract::{
        BankWidth, EffectDescriptorV1, EffectId, EffectProcessBlock, EffectQuality,
        InitialParameterValue, LinkMode, LinkModeSet, NativeEffectFactory, ParameterChannel,
        PortDescriptorV1, PortId, PortLayout, PortRole, PrepareEffectLimits, PrepareEffectRequest,
        PreparedPortsV1, PreparedSidechainPort, ProcessReport, ResetKind, StatePayloadError,
        StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    };
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
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

    struct ThreeBlockConstant {
        lane: u32,
    }
    impl GraphRuntimeProcessor for ThreeBlockConstant {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let scale = (block.first_sample + 1) as f32;
            block.left.fill(scale * (self.lane + 1) as f32);
            block.right.fill(-scale * (1_u32 << self.lane) as f32);
            Ok(())
        }
    }

    #[derive(Default)]
    struct CountingIdentityBuiltin {
        calls: u64,
    }
    impl GraphPreparedBuiltinBankProcessor for CountingIdentityBuiltin {
        fn process(
            &mut self,
            _left: &mut [f32],
            _right: &mut [f32],
            _frames: u32,
            _first_sample: u64,
        ) -> Result<(), RenderError> {
            self.calls += 1;
            Ok(())
        }

        fn qualification_counters(&self) -> [u64; 2] {
            [self.calls, self.calls]
        }
    }

    struct W4OrderObserver {
        lane: u64,
        order: Arc<AtomicU64>,
    }
    impl GraphRuntimeObserver for W4OrderObserver {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let expected_order = block.first_sample * 4 + self.lane;
            assert_eq!(
                self.order.fetch_add(1, Ordering::SeqCst),
                expected_order,
                "stable member observation order"
            );
            let scale = (block.first_sample + 1) as f32;
            assert_eq!(block.left, [scale * (self.lane + 1) as f32]);
            assert_eq!(block.right, [-scale * (1_u64 << self.lane) as f32]);
            Ok(())
        }
    }

    struct SilentSourceSetDriver {
        claims: usize,
    }
    impl GraphPreparedSourceSetDriver for SilentSourceSetDriver {
        fn claim_count(&self) -> usize {
            self.claims
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
            left.fill(0.0);
            right.fill(0.0);
            Ok(())
        }
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

    /// Hand-built plans list their nodes in whatever order reads best; the compiler always emits
    /// them sorted by id, and `program::lower` interns ids by binary search over that order, so
    /// the helpers sort here rather than making every fixture do it by hand.
    fn sorted_nodes(mut nodes: Vec<GraphNode>) -> Vec<GraphNode> {
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        nodes
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
                    nodes: sorted_nodes(graph_nodes),
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
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
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

    /// A padded bank is the normal shape: the last bank of a level holds `1..=W` members and the
    /// rest of its lanes are identity lanes the executor never touches.  Empty and oversized
    /// member lists stay rejected.
    #[test]
    fn with_builtin_banks_accepts_padded_members_and_rejects_empty_or_oversized() {
        let attach = |member_count: usize| {
            let (plan, _, _) = four_track_builtin_plan(970 + member_count as u64, false, true);
            let members: Vec<_> = plan
                .required_bindings
                .iter()
                .filter(|node| {
                    matches!(
                        node,
                        GraphNodeId::TrackStage {
                            stage: TrackStage::PostInputBuiltins,
                            ..
                        }
                    )
                })
                .cloned()
                .collect();
            assert_eq!(members.len(), 4);
            let members: Vec<_> = (0..member_count)
                .map(|index| {
                    members
                        .get(index)
                        .cloned()
                        .unwrap_or(GraphNodeId::TrackStage {
                            track_id: StableGraphId::parse(&format!("extra{index}")).expect("id"),
                            stage: TrackStage::PostInputBuiltins,
                        })
                })
                .collect();
            plan.with_builtin_banks(
                vec![GraphPreparedBuiltinBank {
                    backend: KernelBackendV1::Aarch64Neon,
                    members: members.into_boxed_slice(),
                    processor: Box::<CountingIdentityBuiltin>::default(),
                    scratch: miso_engine_rack::AoSoaScratch::new(BankWidth::Four, 1)
                        .expect("W4 scratch"),
                }],
                GraphBuiltinBankResourceEstimate {
                    bank_count: 1,
                    ..GraphBuiltinBankResourceEstimate::default()
                },
            )
            .map(|_| ())
        };
        for member_count in 1..=4 {
            assert!(
                attach(member_count).is_ok(),
                "{member_count} of four lanes must attach"
            );
        }
        assert_eq!(attach(0), Err(GraphBuiltinBankAttachError::InvalidMembers));
        // Five distinct members over a four-lane scratch: rejected by the width clause, not by
        // the duplicate-member clause.
        assert_eq!(attach(5), Err(GraphBuiltinBankAttachError::InvalidMembers));
    }

    fn four_track_builtin_plan(
        plan_id: u64,
        banked: bool,
        id_ordered: bool,
    ) -> (PreparedGraphPlan, GraphRuntimeBindings, Arc<AtomicU64>) {
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(1),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let inputs: Vec<_> = (0..4)
            .map(|lane| GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
                stage: TrackStage::Input,
            })
            .collect();
        let members: Vec<_> = (0..4)
            .map(|lane| GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
                stage: TrackStage::PostInputBuiltins,
            })
            .collect();
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output id"),
        };
        let mut level_major_schedule = inputs.clone();
        level_major_schedule.extend(members.iter().cloned());
        level_major_schedule.push(output.clone());
        let schedule = if id_ordered {
            inputs
                .iter()
                .cloned()
                .zip(members.iter().cloned())
                .flat_map(|pair| [pair.0, pair.1])
                .chain(core::iter::once(output.clone()))
                .collect()
        } else {
            level_major_schedule.clone()
        };
        let nodes = level_major_schedule
            .iter()
            .cloned()
            .map(|id| GraphNode {
                id,
                latency: LatencySamples(0),
                tail: TailSamples::Finite(0),
            })
            .collect();
        let mut edges = Vec::new();
        for lane in 0..4 {
            edges.push(GraphEdge {
                id: GraphEdgeId::TrackMain {
                    target: members[lane].clone(),
                },
                source: GraphPortId {
                    node: inputs[lane].clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: members[lane].clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: format!("$.tracks[{lane}].builtin"),
            });
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: StableGraphId::parse(&format!("route{lane}")).expect("route id"),
                },
                source: GraphPortId {
                    node: members[lane].clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: format!("$.routes[{lane}]"),
            });
        }
        let builtin_banks = if banked {
            vec![GraphPreparedBuiltinBank {
                backend: KernelBackendV1::Aarch64Neon,
                members: members.clone().into_boxed_slice(),
                processor: Box::<CountingIdentityBuiltin>::default(),
                scratch: miso_engine_rack::AoSoaScratch::new(BankWidth::Four, 1)
                    .expect("W4 scratch"),
            }]
        } else {
            Vec::new()
        };
        let required_bindings: Vec<_> = inputs
            .iter()
            .chain(&members)
            .chain(core::iter::once(&output))
            .cloned()
            .collect();
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id,
            spec: GraphSpec {
                nodes: sorted_nodes(nodes),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: inputs.clone(),
                },
                DependencyLevel {
                    level: 1,
                    nodes: members.clone(),
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: required_bindings.clone(),
            routes: Vec::new(),
            effects: Vec::new(),
            banks: Vec::new(),
            builtin_banks,
            observers: Vec::new(),
        });
        let nodes = required_bindings
            .into_iter()
            .filter(|node| !banked || !members.contains(node))
            .map(|node| {
                let processor: Box<dyn GraphRuntimeProcessor> = match &node {
                    GraphNodeId::TrackStage {
                        track_id,
                        stage: TrackStage::Input,
                    } => Box::new(ThreeBlockConstant {
                        lane: track_id
                            .as_str()
                            .strip_prefix("track")
                            .expect("track prefix")
                            .parse()
                            .expect("track lane"),
                    }),
                    _ => Box::new(Noop),
                };
                GraphNodeBinding::new(node, processor)
            })
            .collect();
        let observer_order = Arc::new(AtomicU64::new(0));
        let observers = members
            .iter()
            .cloned()
            .enumerate()
            .map(|(lane, node)| {
                GraphNodeObserverBinding::new(
                    node,
                    4 - lane as u64,
                    Box::new(W4OrderObserver {
                        lane: lane as u64,
                        order: Arc::clone(&observer_order),
                    }),
                )
            })
            .collect();
        (
            graph,
            GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes,
                observers,
            },
            observer_order,
        )
    }

    fn render_three_blocks(mut plan: PreparedRenderPlan) -> ([f32; 6], [u64; 2]) {
        let mut pcm = [0.0_f32; 6];
        for block in 0..3 {
            let output = PlanarBufferMut::try_new(&mut pcm[block * 2..block * 2 + 2], 2, 1, 1)
                .expect("output");
            plan.render(
                miso_engine_core::realtime::RenderIo {
                    input: None,
                    output,
                },
                miso_engine_core::realtime::RenderTime {
                    absolute_sample: block as u64,
                },
            )
            .expect("render");
        }
        (pcm, plan.qualification_counters())
    }

    fn silent_source_set(envelope: RenderEnvelope, node: GraphNodeId) -> GraphPreparedSourceSet {
        GraphPreparedSourceSet::new(
            envelope,
            vec![GraphSourceInputClaim { node }],
            GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(SilentSourceSetDriver { claims: 1 }),
        )
    }

    /// A recovery deadline long enough that no descheduled worker is ever mistaken for a dead
    /// one. The determinism harnesses measure bits, not deadlines; the bounded deadline itself is
    /// proved by the dead-worker injection test in `tests/scheduler_recovery.rs`.
    #[cfg(not(target_arch = "wasm32"))]
    const DETERMINISM_DEADLINE_NS: u64 = 5_000_000_000;

    fn single_thread_config() -> NativeGraphBindConfigV1 {
        NativeGraphBindConfigV1 {
            render_mode: NativeGraphRenderModeV1::SingleThread,
            scheduler: NativeSchedulerConfigV1::new(
                core::num::NonZeroUsize::new(4).expect("four lanes"),
                true,
                NativeWorkerPoolShapeV1::default(),
            ),
            maximum_retained_bytes: 1 << 20,
        }
    }

    /// Start a pool for `lanes` render lanes and hand back its shape and first lease.
    ///
    /// The pool is control-plane state that outlives the plan: every caller keeps it alive for as
    /// long as it renders.
    #[cfg(not(target_arch = "wasm32"))]
    fn test_pool(
        lanes: usize,
    ) -> (
        Option<NativeGraphWorkerPoolV1>,
        Option<NativeGraphWorkerLeaseV1>,
        NativeWorkerPoolShapeV1,
    ) {
        #![allow(clippy::type_complexity)]
        match core::num::NonZeroUsize::new(lanes.saturating_sub(1)) {
            None => (None, None, NativeWorkerPoolShapeV1::default()),
            Some(workers) => {
                let (pool, lease) = NativeGraphWorkerPoolV1::start(NativeWorkerPoolConfigV1 {
                    requested_workers: Some(workers),
                    ..NativeWorkerPoolConfigV1::default()
                })
                .expect("test worker pool");
                let shape = pool.shape();
                (Some(pool), Some(lease), shape)
            }
        }
    }

    #[test]
    fn level_major_w4_builtin_bank_is_analytic_for_three_blocks_in_both_executors() {
        let (scalar_graph, scalar_bindings, scalar_observers) =
            four_track_builtin_plan(123_000, false, false);
        assert!(scalar_graph.inserted_delays.is_empty());
        assert!(
            scalar_graph
                .spec
                .nodes
                .iter()
                .all(|node| node.latency == LatencySamples(0))
        );
        let scalar = render_three_blocks(
            scalar_graph
                .bind(scalar_bindings)
                .unwrap_or_else(|failure| panic!("scalar reference bind: {}", failure.code)),
        );

        let (bank_graph, bank_bindings, bank_observers) =
            four_track_builtin_plan(123_001, true, false);
        assert!(bank_graph.inserted_delays.is_empty());
        assert!(
            bank_graph
                .spec
                .nodes
                .iter()
                .all(|node| node.latency == LatencySamples(0))
        );
        let banked = render_three_blocks(
            bank_graph
                .bind(bank_bindings)
                .unwrap_or_else(|failure| panic!("banked bind: {}", failure.code)),
        );

        let (native_graph, native_bindings, native_observers) =
            four_track_builtin_plan(123_002, true, false);
        assert!(native_graph.inserted_delays.is_empty());
        assert!(
            native_graph
                .spec
                .nodes
                .iter()
                .all(|node| node.latency == LatencySamples(0))
        );
        let native = native_graph
            .bind_native(native_bindings, single_thread_config())
            .unwrap_or_else(|failure| panic!("native bind: {}", failure.code));
        assert_eq!(
            native.metadata.selection,
            SchedulerSelectionV1::Sequential(FallbackReasonV1::SingleThread)
        );
        let native = render_three_blocks(native.into_plan());

        let analytic = [10.0, -15.0, 20.0, -30.0, 30.0, -45.0];
        assert_eq!(scalar.0.map(f32::to_bits), analytic.map(f32::to_bits));
        assert_eq!(banked.0.map(f32::to_bits), analytic.map(f32::to_bits));
        assert_eq!(native.0.map(f32::to_bits), analytic.map(f32::to_bits));
        assert_eq!(scalar.1, [0, 0]);
        assert_eq!(banked.1, [3, 3]);
        assert_eq!(native.1, [3, 3]);
        assert_eq!(scalar_observers.load(Ordering::SeqCst), 12);
        assert_eq!(bank_observers.load(Ordering::SeqCst), 12);
        assert_eq!(native_observers.load(Ordering::SeqCst), 12);
    }

    #[test]
    fn id_ordered_bank_plan_rejects_transactionally_and_returned_ownership_is_reusable() {
        let restore_level_major = |plan: &mut PreparedGraphPlan| {
            plan.sequential_schedule = plan
                .dependency_levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect();
        };

        let (invalid, mut bindings, _) = four_track_builtin_plan(123_010, true, true);
        bindings.observers.clear();
        let failure = match invalid.bind(bindings) {
            Ok(plan) => {
                let (pcm, _) = render_three_blocks(plan);
                assert_eq!(
                    [pcm[0], pcm[2], pcm[4]],
                    [10.0, 20.0, 30.0],
                    "ID-ordered bank exposes the auditor's 1/11/21 stale-lane transcript"
                );
                panic!("ID-ordered scalar bank plan accepted")
            }
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.layout");
        assert_eq!(failure.bindings.nodes.len(), 5);
        let mut recovered = *failure.plan;
        restore_level_major(&mut recovered);
        let recovered = recovered
            .bind(failure.bindings)
            .unwrap_or_else(|retry| panic!("scalar ownership retry: {}", retry.code));
        assert_eq!(
            render_three_blocks(recovered).0.map(f32::to_bits),
            [10.0, -15.0, 20.0, -30.0, 30.0, -45.0].map(f32::to_bits)
        );

        let (invalid, bindings, _) = four_track_builtin_plan(123_011, true, true);
        let config = single_thread_config();
        let failure = match invalid.bind_native(bindings, config) {
            Ok(_) => panic!("ID-ordered native bank plan accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.layout");
        assert_eq!(failure.config, config);
        assert_eq!(failure.bindings.nodes.len(), 5);
        let mut recovered = *failure.plan;
        restore_level_major(&mut recovered);
        let recovered = recovered
            .bind_native(failure.bindings, failure.config)
            .unwrap_or_else(|retry| panic!("native ownership retry: {}", retry.code));
        assert_eq!(
            render_three_blocks(recovered.into_plan())
                .0
                .map(f32::to_bits),
            [10.0, -15.0, 20.0, -30.0, 30.0, -45.0].map(f32::to_bits)
        );
    }

    #[test]
    fn structural_layout_rejection_is_shared_after_binding_validation_for_source_families() {
        let (mut scalar, scalar_bindings, _) = binding_plan();
        scalar.sequential_schedule.swap(0, 1);
        let scalar_failure = match scalar.bind(scalar_bindings) {
            Ok(_) => panic!("non-level-major scalar plan accepted"),
            Err(failure) => failure,
        };
        assert_eq!(scalar_failure.code, "graph.scheduler.layout");
        let mut scalar = *scalar_failure.plan;
        scalar.sequential_schedule.swap(0, 1);
        scalar
            .bind(scalar_failure.bindings)
            .unwrap_or_else(|failure| panic!("scalar retry: {}", failure.code));

        let (mut source_graph, mut source_bindings, source_node) = binding_plan();
        source_graph.sequential_schedule.swap(0, 1);
        source_bindings.nodes.remove(0);
        let source_set = silent_source_set(source_graph.envelope, source_node);
        let source_failure = match source_graph.bind_with_source_set(source_bindings, source_set) {
            Ok(_) => panic!("non-level-major scalar source plan accepted"),
            Err(failure) => failure,
        };
        assert_eq!(source_failure.code, "graph.scheduler.layout");
        assert_eq!(source_failure.source_set.claims().len(), 1);
        let mut source_graph = *source_failure.plan;
        source_graph.sequential_schedule.swap(0, 1);
        source_graph
            .bind_with_source_set(source_failure.bindings, source_failure.source_set)
            .unwrap_or_else(|failure| panic!("scalar source retry: {}", failure.code));

        let (mut native, native_bindings) = native_parallel_sum_plan(48_000);
        native.sequential_schedule.swap(0, 1);
        let native_failure = match native.bind_native(native_bindings, single_thread_config()) {
            Ok(_) => panic!("non-level-major native plan accepted"),
            Err(failure) => failure,
        };
        assert_eq!(native_failure.code, "graph.scheduler.layout");
        let mut native = *native_failure.plan;
        native.sequential_schedule.swap(0, 1);
        native
            .bind_native(native_failure.bindings, native_failure.config)
            .unwrap_or_else(|failure| panic!("native retry: {}", failure.code));

        let (mut native_source, mut native_source_bindings) = native_parallel_sum_plan(48_000);
        native_source.sequential_schedule.swap(0, 1);
        let claimed = native_source.dependency_levels[0].nodes[0].clone();
        native_source_bindings
            .nodes
            .retain(|binding| binding.node != claimed);
        let source_set = silent_source_set(native_source.envelope, claimed);
        let native_source_failure = match native_source.bind_native_with_source_set(
            native_source_bindings,
            single_thread_config(),
            source_set,
        ) {
            Ok(_) => panic!("non-level-major native source plan accepted"),
            Err(failure) => failure,
        };
        assert_eq!(native_source_failure.code, "graph.scheduler.layout");
        assert_eq!(native_source_failure.source_set.claims().len(), 1);
        let mut native_source = *native_source_failure.plan;
        native_source.sequential_schedule.swap(0, 1);
        native_source
            .bind_native_with_source_set(
                native_source_failure.bindings,
                native_source_failure.config,
                native_source_failure.source_set,
            )
            .unwrap_or_else(|failure| panic!("native source retry: {}", failure.code));

        let (mut precedence_plan, mut bad_bindings, duplicate) = binding_plan();
        precedence_plan.sequential_schedule.swap(0, 1);
        bad_bindings
            .nodes
            .push(GraphNodeBinding::new(duplicate, Box::new(Noop)));
        let failure = match precedence_plan.bind(bad_bindings) {
            Ok(_) => panic!("invalid binding accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.binding");
    }

    #[test]
    fn structural_layout_rejects_node_level_edge_and_bank_corruptions() {
        for corruption in 0..6 {
            let (mut graph, bindings, _) = binding_plan();
            match corruption {
                0 => {
                    let duplicate = graph.dependency_levels[0].nodes[0].clone();
                    graph.dependency_levels[0].nodes.push(duplicate);
                }
                1 => graph.dependency_levels[1].nodes.clear(),
                2 => graph.dependency_levels[1].level = 0,
                3 => graph.spec.edges[0].source.node = graph.dependency_levels[1].nodes[0].clone(),
                4 => {
                    graph.sequential_schedule.pop();
                }
                // `program::lower` interns node ids by binary search over `spec.nodes`, so an
                // unsorted spec is a malformed plan: bind refuses it with the same code rather
                // than lowering against a binary search that cannot find its nodes.
                5 => graph.spec.nodes.reverse(),
                _ => unreachable!(),
            }
            let failure = match graph.bind(bindings) {
                Ok(_) => panic!("structural corruption accepted"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.scheduler.layout");
        }

        let (mut reversed_bank, bindings, _) = four_track_builtin_plan(123_100, true, false);
        reversed_bank.builtin_banks[0].members.reverse();
        let failure = match reversed_bank.bind(bindings) {
            Ok(_) => panic!("reversed bank members accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.layout");

        let (mut mixed_level_bank, bindings, _) = four_track_builtin_plan(123_101, true, false);
        let moved = mixed_level_bank.dependency_levels[1]
            .nodes
            .pop()
            .expect("member");
        mixed_level_bank.dependency_levels[0].nodes.push(moved);
        mixed_level_bank.dependency_levels[0].nodes.sort();
        mixed_level_bank.sequential_schedule = mixed_level_bank
            .dependency_levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let failure = match mixed_level_bank.bind(bindings) {
            Ok(_) => panic!("mixed-level bank accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.layout");
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
    fn reduction_is_left_to_right() {
        assert_eq!(reduce_left_to_right(&[1.0, 2.0, 3.0]), 6.0);
    }

    #[test]
    fn left_to_right_reduction_meets_analytic_bound_and_ignores_completion_order() {
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
            // D9 is a left-to-right recursive summation, so the classical bound is
            // `gamma_{n-1} * sum|x_i|` with `gamma_k = k u / (1 - k u)`, `u = 2^-24`
            // (Higham, *Accuracy and Stability of Numerical Algorithms*, 2nd ed., eq. 4.4) --
            // `n - 1` additions rather than the balanced tree's `log2 n` levels.
            let steps = (fixture.len() - 1) as f64;
            let u = 2.0_f64.powi(-24);
            let gamma = steps * u / (1.0 - steps * u);
            let bound = gamma * sum_abs + fixture.len() as f64 * f64::from(f32::MIN_POSITIVE);
            let actual = reduce_left_to_right(&fixture);
            assert!((f64::from(actual) - reference).abs() <= bound);
        }

        let canonical: Vec<_> = (0..65)
            .map(|index| (index, (index as f32 + 1.0).recip()))
            .collect();
        let baseline_values: Vec<_> = canonical.iter().map(|(_, value)| *value).collect();
        let baseline = reduce_left_to_right(&baseline_values).to_bits();
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
            // reduction order.
            completed.sort_by_key(|(id, _)| *id);
            let values: Vec<_> = completed.iter().map(|(_, value)| *value).collect();
            assert_eq!(reduce_left_to_right(&values).to_bits(), baseline);
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

    /// `GraphNodeBinding::identity` acknowledges a required node without a host processor: the
    /// node is still listed (so a genuinely missing binding still fails), and the executor renders
    /// it with its own reduction kind, bit-for-bit as a do-nothing host processor did.
    #[test]
    fn identity_binding_acknowledges_without_a_processor() {
        struct Constant;
        impl GraphRuntimeProcessor for Constant {
            fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
                block.left.fill(0.25);
                block.right.fill(-0.5);
                Ok(())
            }
        }
        let render = |identity: bool| {
            let (plan, bindings, input) = binding_plan();
            let bindings = GraphRuntimeBindings {
                envelope: bindings.envelope,
                nodes: bindings
                    .nodes
                    .into_iter()
                    .map(|binding| {
                        if binding.node == input {
                            GraphNodeBinding::new(binding.node, Box::new(Constant))
                        } else if identity {
                            GraphNodeBinding::identity(binding.node)
                        } else {
                            binding
                        }
                    })
                    .collect(),
                observers: bindings.observers,
            };
            let mut plan = match plan.bind(bindings) {
                Ok(plan) => plan,
                Err(failure) => panic!("identity bindings rejected: {}", failure.code),
            };
            let mut samples = [f32::NAN; 2];
            let output = PlanarBufferMut::try_new(&mut samples, 2, 1, 1).expect("output");
            plan.render(
                miso_engine_core::realtime::RenderIo {
                    input: None,
                    output,
                },
                miso_engine_core::realtime::RenderTime { absolute_sample: 0 },
            )
            .expect("render");
            [samples[0].to_bits(), samples[1].to_bits()]
        };
        let identity_bits = render(true);
        assert_eq!(
            identity_bits,
            render(false),
            "an identity binding must render the same bits as a do-nothing host processor"
        );
        assert_eq!(
            identity_bits,
            [0.25_f32.to_bits(), (-0.5_f32).to_bits()],
            "a supplied processor must still run when other nodes bind by identity"
        );

        let (plan, bindings, _) = binding_plan();
        let mut nodes = bindings.nodes;
        nodes.pop();
        let short = GraphRuntimeBindings {
            envelope: bindings.envelope,
            nodes,
            observers: bindings.observers,
        };
        match plan.bind(short) {
            Ok(_) => panic!("a missing binding was accepted"),
            Err(failure) => assert_eq!(failure.code, "graph.plan.binding"),
        }
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
                nodes: sorted_nodes(nodes),
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
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

    // ---- 50 random DAG sessions: the executor bit-identity gate (#98 F2) ----------------------

    /// Adds its sidechain when one is connected, and otherwise trims: the random corpus needs an
    /// effect that is legal both with and without a sidechain edge.
    struct OptionalSidechainSum {
        metadata: PreparedEffectMetadata,
    }
    impl PreparedNativeEffect for OptionalSidechainSum {
        fn metadata(&self) -> PreparedEffectMetadata {
            self.metadata
        }
        fn reset(&mut self, _kind: ResetKind) {}
        fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
            match block.sidechain {
                Some((side_left, side_right)) => {
                    for frame in 0..block.left.len() {
                        block.left[frame] += side_left[frame];
                        block.right[frame] += side_right[frame];
                    }
                }
                None => {
                    for frame in 0..block.left.len() {
                        block.left[frame] *= 0.75;
                        block.right[frame] *= 0.75;
                    }
                }
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

    /// Frozen xorshift64: the generator must not depend on host RNG state.
    fn xorshift(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    /// Seeded noise, so a bound input is a deterministic signal rather than a constant.
    struct SeededSource {
        state: u32,
    }
    impl GraphRuntimeProcessor for SeededSource {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.left.len() {
                self.state = self
                    .state
                    .wrapping_mul(1_664_525)
                    .wrapping_add(1_013_904_223);
                let value = f32::from(((self.state >> 16) & 0xffff) as i16) / 3_276.8;
                block.left[frame] = value;
                block.right[frame] = -value * 0.5;
            }
            Ok(())
        }
    }

    /// A stateful bound stage: proves the executors advance identical state, not just bits.
    struct Recursive {
        coefficient: f32,
        left: f32,
        right: f32,
    }
    impl GraphRuntimeProcessor for Recursive {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.left.len() {
                self.left = miso_engine_lane::softfma::fma_f32_via_f64(
                    self.coefficient,
                    self.left,
                    block.left[frame] * 0.5,
                );
                self.right = miso_engine_lane::softfma::fma_f32_via_f64(
                    self.coefficient,
                    self.right,
                    block.right[frame] * 0.5,
                );
                block.left[frame] = self.left;
                block.right[frame] = self.right;
            }
            Ok(())
        }
    }

    /// Longest-path dependency levels of a hand-built DAG, the way the compiler's `topo` emits
    /// them: level `1 + max(predecessor level)`, nodes ascending within a level, schedule the
    /// concatenation. Test-only -- production has exactly one level derivation, in the compiler.
    fn levels_for(nodes: &[GraphNodeId], edges: &[GraphEdge]) -> Vec<DependencyLevel> {
        let mut level: BTreeMap<GraphNodeId, u64> =
            nodes.iter().cloned().map(|node| (node, 0)).collect();
        for _ in 0..nodes.len() {
            let mut changed = false;
            for edge in edges {
                let source = level[&edge.source.node];
                let destination = level[&edge.destination.node];
                if destination < source + 1 {
                    level.insert(edge.destination.node.clone(), source + 1);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let mut by_level: BTreeMap<u64, Vec<GraphNodeId>> = BTreeMap::new();
        for (node, value) in level {
            by_level.entry(value).or_default().push(node);
        }
        by_level
            .into_iter()
            .map(|(level, mut nodes)| {
                nodes.sort();
                DependencyLevel { level, nodes }
            })
            .collect()
    }

    /// Builds one seeded random session: tracks with their seven stage boundaries, optional
    /// rack effects with sidechains from other tracks, submixes, routes from arbitrary send taps,
    /// and PDC on a random subset of edges. Calling it twice with the same seed produces two
    /// structurally identical plans with independent processor state.
    fn random_dag_plan(seed: u64) -> (PreparedGraphPlan, GraphRuntimeBindings) {
        let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(16),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let effect_metadata = PreparedEffectMetadata {
            descriptor: &SUM_DESCRIPTOR,
            sample_rate: 48_000,
            quantum: 16,
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
        let track_count = 1 + (xorshift(&mut state) % 5) as usize;
        let submix_count = (xorshift(&mut state) % 3) as usize;
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostSimd1,
            TrackStage::PostDynamic,
            TrackStage::PostSimd2PreFader,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let track_id = |track: usize| StableGraphId::parse(&format!("t{track}")).expect("track ID");
        let stage_node = |track: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: track_id(track),
            stage,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };

        let mut nodes: Vec<GraphNodeId> = Vec::new();
        let mut edges: Vec<GraphEdge> = Vec::new();
        let mut bindings: Vec<GraphNodeBinding> = Vec::new();
        let mut required: Vec<GraphNodeId> = Vec::new();
        let mut routes: Vec<PreparedRoute> = Vec::new();
        let mut effects: Vec<GraphPreparedEffect> = Vec::new();
        let mut delays: Vec<InsertedDelay> = Vec::new();
        let mut taps: Vec<GraphNodeId> = Vec::new();

        let main_edge = |source: &GraphNodeId, destination: &GraphNodeId| GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: destination.clone(),
            },
            source: GraphPortId {
                node: source.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: destination.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.main".to_owned(),
        };

        for track in 0..track_count {
            // The stage chain, with an optional rack effect spliced into two of the boundaries.
            let mut chain: Vec<GraphNodeId> = Vec::new();
            for (index, stage) in stages.iter().enumerate() {
                chain.push(stage_node(track, *stage));
                let rack = match index {
                    1 => Some(RackId::Simd1),
                    2 => Some(RackId::Dynamic),
                    _ => None,
                };
                if let Some(rack) = rack
                    && xorshift(&mut state).is_multiple_of(2)
                {
                    chain.push(GraphNodeId::Effect(EffectNodeId {
                        track_id: track_id(track),
                        rack,
                        effect_id: StableGraphId::parse(&format!("fx{index}")).expect("effect ID"),
                    }));
                }
            }
            for node in &chain {
                nodes.push(node.clone());
                if matches!(node, GraphNodeId::TrackStage { .. }) {
                    taps.push(node.clone());
                }
                if let GraphNodeId::Effect(id) = node {
                    effects.push(GraphPreparedEffect {
                        id: id.clone(),
                        metadata: effect_metadata,
                        processor: Box::new(OptionalSidechainSum {
                            metadata: effect_metadata,
                        }),
                    });
                }
            }
            for pair in chain.windows(2) {
                edges.push(main_edge(&pair[0], &pair[1]));
            }
            // The input is always bound; some later boundaries carry a recursive processor.
            bindings.push(GraphNodeBinding::new(
                chain[0].clone(),
                Box::new(SeededSource {
                    state: 0x51ED_0000 + track as u32,
                }),
            ));
            required.push(chain[0].clone());
            for node in chain.iter().skip(1) {
                let bindable = matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::PostInputBuiltins
                            | TrackStage::PostFader
                            | TrackStage::PostMatrix,
                        ..
                    }
                );
                if bindable && xorshift(&mut state).is_multiple_of(3) {
                    bindings.push(GraphNodeBinding::new(
                        node.clone(),
                        Box::new(Recursive {
                            coefficient: 0.25 + (xorshift(&mut state) % 64) as f32 / 256.0,
                            left: 0.0,
                            right: 0.0,
                        }),
                    ));
                    required.push(node.clone());
                }
            }
        }

        let submixes: Vec<GraphNodeId> = (0..submix_count)
            .map(|index| GraphNodeId::Submix {
                submix_id: StableGraphId::parse(&format!("s{index}")).expect("submix ID"),
            })
            .collect();

        // Routes: every track sends from one or two arbitrary taps; every submix that received a
        // send routes on to the output, so the output's fan-in varies from 1 to well past four.
        let mut route_index = 0_usize;
        let mut fed_submixes: BTreeSet<usize> = BTreeSet::new();
        let mut output_fed = false;
        for track in 0..track_count {
            let sends = 1 + (xorshift(&mut state) % 2) as usize;
            for _ in 0..sends {
                let track_taps: Vec<&GraphNodeId> = taps
                    .iter()
                    .filter(|node| {
                        matches!(node, GraphNodeId::TrackStage { track_id: id, .. }
                            if id.as_str() == format!("t{track}"))
                    })
                    .collect();
                let source = track_taps[(xorshift(&mut state) as usize) % track_taps.len()].clone();
                let destination = if submixes.is_empty() || xorshift(&mut state).is_multiple_of(3) {
                    output_fed = true;
                    output.clone()
                } else {
                    let index = (xorshift(&mut state) as usize) % submixes.len();
                    fed_submixes.insert(index);
                    submixes[index].clone()
                };
                let route_id =
                    StableGraphId::parse(&format!("r{route_index:03}")).expect("route ID");
                route_index += 1;
                let route_node = GraphNodeId::Route {
                    route_id: route_id.clone(),
                };
                nodes.push(route_node.clone());
                edges.push(GraphEdge {
                    id: GraphEdgeId::RouteSource {
                        route_id: route_id.clone(),
                    },
                    source: GraphPortId {
                        node: source,
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: route_node.clone(),
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: "$.route.source".to_owned(),
                });
                edges.push(GraphEdge {
                    id: GraphEdgeId::RouteDestination {
                        route_id: route_id.clone(),
                    },
                    source: GraphPortId {
                        node: route_node.clone(),
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: destination,
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: "$.route.destination".to_owned(),
                });
                // Non-trivial 2x2 matrices and gains, so the folded route is actually exercised.
                let coefficient =
                    |state: &mut u64| (xorshift(state) % 2_001) as f32 / 1_000.0 - 1.0;
                routes.push(PreparedRoute {
                    node: route_node,
                    transform: RouteTransform {
                        gain: 0.25 + (xorshift(&mut state) % 1_500) as f32 / 1_000.0,
                        ll: coefficient(&mut state),
                        lr: coefficient(&mut state),
                        rl: coefficient(&mut state),
                        rr: coefficient(&mut state),
                    },
                });
            }
        }
        for (index, submix) in submixes.iter().enumerate() {
            if !fed_submixes.contains(&index) {
                continue;
            }
            nodes.push(submix.clone());
            let route_id = StableGraphId::parse(&format!("r{route_index:03}")).expect("route ID");
            route_index += 1;
            let route_node = GraphNodeId::Route {
                route_id: route_id.clone(),
            };
            nodes.push(route_node.clone());
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: route_id.clone(),
                },
                source: GraphPortId {
                    node: submix.clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: route_node.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.submix.source".to_owned(),
            });
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteDestination {
                    route_id: route_id.clone(),
                },
                source: GraphPortId {
                    node: route_node.clone(),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: output.clone(),
                    kind: GraphPortKind::MainInput,
                    effect_port: None,
                },
                path: "$.submix.destination".to_owned(),
            });
            output_fed = true;
            routes.push(PreparedRoute {
                node: route_node,
                transform: RouteTransform {
                    gain: 1.0,
                    ll: 0.75,
                    lr: 0.25,
                    rl: -0.25,
                    rr: 0.75,
                },
            });
        }
        assert!(output_fed, "seed {seed}: the output must be fed");
        nodes.push(output.clone());
        bindings.push(GraphNodeBinding::new(output.clone(), Box::new(Noop)));
        required.push(output.clone());

        // Sidechains: each dynamic-rack effect may listen to an earlier track's input boundary,
        // which is always scheduled before it.
        let effect_nodes: Vec<GraphNodeId> = nodes
            .iter()
            .filter(|node| {
                matches!(node, GraphNodeId::Effect(id) if matches!(id.rack, RackId::Dynamic))
            })
            .cloned()
            .collect();
        for node in effect_nodes {
            if !xorshift(&mut state).is_multiple_of(2) {
                continue;
            }
            let GraphNodeId::Effect(id) = &node else {
                unreachable!()
            };
            let source_track = (xorshift(&mut state) as usize) % track_count;
            if format!("t{source_track}") == id.track_id.as_str() {
                continue;
            }
            edges.push(GraphEdge {
                id: GraphEdgeId::EffectSidechain {
                    effect: id.clone(),
                    port: SUM_SIDECHAIN.as_str().to_owned(),
                },
                source: GraphPortId {
                    node: stage_node(source_track, TrackStage::Input),
                    kind: GraphPortKind::MainOutput,
                    effect_port: None,
                },
                destination: GraphPortId {
                    node: node.clone(),
                    kind: GraphPortKind::SidechainInput,
                    effect_port: Some(SUM_SIDECHAIN.as_str().to_owned()),
                },
                path: "$.sidechain".to_owned(),
            });
        }

        edges.sort_by(|left, right| left.id.cmp(&right.id));
        nodes.sort();
        let levels = levels_for(&nodes, &edges);
        let schedule: Vec<GraphNodeId> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();

        // PDC on a random subset of edges, in the order the compiler would emit it.
        for edge in &edges {
            if !xorshift(&mut state).is_multiple_of(4) {
                continue;
            }
            delays.push(InsertedDelay {
                node: GraphNodeId::CompensationDelay {
                    edge_id: Box::new(edge.id.clone()),
                },
                edge_id: edge.id.clone(),
                samples: LatencySamples(1 + xorshift(&mut state) % 40),
            });
        }

        let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 700_000 + seed,
            spec: GraphSpec {
                nodes: sorted_nodes(
                    nodes
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: LatencySamples(0),
                            tail: TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels,
            route_timings: Vec::new(),
            inserted_delays: delays,
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: required,
            routes,
            effects,
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        (
            plan,
            GraphRuntimeBindings {
                #[cfg(not(target_arch = "wasm32"))]
                worker_lease: None,
                envelope,
                nodes: bindings,
                observers: Vec::new(),
            },
        )
    }

    /// One block at its own absolute sample time, so a caller can keep the per-block bits.
    #[cfg(not(target_arch = "wasm32"))]
    fn render_block_at(plan: &mut PreparedRenderPlan, frames: usize, block: u64) -> Vec<u32> {
        let mut samples = vec![0.0_f32; frames * 2];
        plan.render(
            miso_engine_core::realtime::RenderIo {
                input: None,
                output: PlanarBufferMut::try_new(&mut samples, 2, frames, frames).expect("output"),
            },
            miso_engine_core::realtime::RenderTime {
                absolute_sample: block * frames as u64,
            },
        )
        .expect("render");
        samples.iter().map(|sample| sample.to_bits()).collect()
    }

    fn render_blocks(plan: &mut PreparedRenderPlan, frames: usize, blocks: u64) -> Vec<u32> {
        let mut bits = Vec::with_capacity(blocks as usize * frames * 2);
        let mut samples = vec![0.0_f32; frames * 2];
        for block in 0..blocks {
            samples.fill(0.0);
            plan.render(
                miso_engine_core::realtime::RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut samples, 2, frames, frames)
                        .expect("output"),
                },
                miso_engine_core::realtime::RenderTime {
                    absolute_sample: block * frames as u64,
                },
            )
            .expect("render");
            bits.extend(samples.iter().map(|sample| sample.to_bits()));
        }
        bits
    }

    /// What a [`TapRecorder`] writes: how many blocks it saw, and the left-plane bits it saw.
    type TapSink = (Arc<AtomicU64>, Arc<std::sync::Mutex<Vec<u32>>>);

    /// Scales its block, so the buffer an alias points at demonstrably changes when the next op
    /// runs: a tap attached to the wrong op reads the scaled value.
    struct Scale(f32);
    impl GraphRuntimeProcessor for Scale {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for sample in block.left.iter_mut().chain(block.right.iter_mut()) {
                *sample *= self.0;
            }
            Ok(())
        }
    }

    /// Records what an observer saw, so a tap on an elided stage can be checked.
    struct TapRecorder(Arc<AtomicU64>, Arc<std::sync::Mutex<Vec<u32>>>);
    impl GraphRuntimeObserver for TapRecorder {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            let mut sink = self.1.lock().expect("tap sink");
            sink.extend(block.left.iter().map(|sample| sample.to_bits()));
            Ok(())
        }
    }

    /// E9. The three internal rack boundaries are pure aliases, so the lowering elides them and
    /// the executors never copy through them -- and the audio is bit-identical to the same plan
    /// with those stages materialised by a copy-through processor. An observer on an elided stage
    /// still sees the buffer, because a tap fires immediately after the op that last wrote it.
    ///
    /// Red mutation (`tests/MUTATIONS.md`): resolve an alias to its immediate producer instead of
    /// the root of the chain, or attach a tap's observers to the consumer instead of the producer.
    #[test]
    fn aliased_identity_stages_do_not_change_audio() {
        const FRAMES: usize = 16;
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostSimd1,
            TrackStage::PostDynamic,
            TrackStage::PostSimd2PreFader,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let node = |stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("t0").expect("track ID"),
            stage,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: miso_engine_core::SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let build = |materialise: bool, observed: Option<TapSink>| {
            let mut nodes: Vec<GraphNodeId> = stages
                .iter()
                .copied()
                .filter(|stage| {
                    materialise
                        || !matches!(
                            stage,
                            TrackStage::PostSimd1
                                | TrackStage::PostDynamic
                                | TrackStage::PostSimd2PreFader
                        )
                })
                .map(node)
                .collect();
            nodes.push(output.clone());
            let mut edges = Vec::new();
            for pair in nodes.windows(2) {
                edges.push(GraphEdge {
                    id: GraphEdgeId::TrackMain {
                        target: pair[1].clone(),
                    },
                    source: GraphPortId {
                        node: pair[0].clone(),
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: pair[1].clone(),
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: "$.main".to_owned(),
                });
            }
            // `PostFader` scales in place, so the buffer the three internal boundaries alias is
            // rewritten by the very next op: a tap that fires late reads the scaled value.
            let bindings = vec![
                GraphNodeBinding::new(
                    node(TrackStage::Input),
                    Box::new(SeededSource { state: 0x51ED_0007 }),
                ),
                GraphNodeBinding::new(node(TrackStage::PostFader), Box::new(Scale(0.375))),
                GraphNodeBinding::new(output.clone(), Box::new(Noop)),
            ];
            let required = vec![
                node(TrackStage::Input),
                node(TrackStage::PostFader),
                output.clone(),
            ];
            let levels = levels_for(&nodes, &edges);
            let schedule: Vec<GraphNodeId> = levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect();
            let observers = match &observed {
                Some((calls, sink)) => vec![GraphNodeObserverBinding::new(
                    node(TrackStage::PostDynamic),
                    0,
                    Box::new(TapRecorder(Arc::clone(calls), Arc::clone(sink))),
                )],
                None => Vec::new(),
            };
            let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
                plan_id: u64::from(materialise) + 900,
                spec: GraphSpec {
                    nodes: sorted_nodes(
                        nodes
                            .iter()
                            .cloned()
                            .map(|id| GraphNode {
                                id,
                                latency: LatencySamples(0),
                                tail: TailSamples::Finite(0),
                            })
                            .collect(),
                    ),
                    ports: Vec::new(),
                    edges,
                },
                sequential_schedule: schedule,
                dependency_levels: levels,
                route_timings: Vec::new(),
                inserted_delays: Vec::new(),
                buffer_assignments: Vec::new(),
                estimate: empty_estimate(),
                envelope,
                required_bindings: required,
                routes: Vec::new(),
                effects: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers,
            });
            (
                plan,
                GraphRuntimeBindings {
                    #[cfg(not(target_arch = "wasm32"))]
                    worker_lease: None,
                    envelope,
                    nodes: bindings,
                    observers: Vec::new(),
                },
            )
        };

        // The full chain elides exactly the three internal boundaries; the control graph omits
        // them entirely, which is what "these boundaries carry signal unchanged" means.
        let (aliased, aliased_bindings) = build(true, None);
        let program = aliased.program().expect("lowered");
        assert_eq!(
            program.taps.len(),
            3,
            "three internal boundaries are aliases"
        );
        assert_eq!(program.ops.len(), 5);
        assert!(program.buffers <= 2, "the arena is coloured, not per-node");
        let (materialised, materialised_bindings) = build(false, None);
        assert_eq!(
            materialised.program().expect("lowered").taps.len(),
            0,
            "the control graph has no boundary to alias"
        );
        assert_eq!(materialised.program().expect("lowered").ops.len(), 5);

        let mut aliased_plan = aliased
            .bind(aliased_bindings)
            .unwrap_or_else(|failure| panic!("aliased bind: {}", failure.code));
        let mut materialised_plan = materialised
            .bind(materialised_bindings)
            .unwrap_or_else(|failure| panic!("materialised bind: {}", failure.code));
        assert_eq!(
            render_blocks(&mut aliased_plan, FRAMES, 4),
            render_blocks(&mut materialised_plan, FRAMES, 4),
            "aliasing must not change one bit"
        );

        // A processor bound to an elided stage would never run, so the bind refuses it rather
        // than dropping it silently.
        let (elided_binding, mut elided_bindings) = build(true, None);
        elided_bindings.nodes.push(GraphNodeBinding::new(
            node(TrackStage::PostSimd1),
            Box::new(Scale(2.0)),
        ));
        let mut refused = elided_binding;
        refused.required_bindings.push(node(TrackStage::PostSimd1));
        match refused.bind(elided_bindings) {
            Ok(_) => panic!("a binding on an elided stage must be refused"),
            Err(failure) => assert_eq!(failure.code, "graph.scheduler.layout"),
        }

        // An observer bound to an elided stage still observes its buffer, once per block, and
        // sees exactly what the producing op wrote.
        let calls = Arc::new(AtomicU64::new(0));
        let sink = Arc::new(std::sync::Mutex::new(Vec::new()));
        let (observed, observed_bindings) =
            build(true, Some((Arc::clone(&calls), Arc::clone(&sink))));
        let mut observed_plan = observed
            .bind(observed_bindings)
            .unwrap_or_else(|failure| panic!("observed bind: {}", failure.code));
        let observed_bits = render_blocks(&mut observed_plan, FRAMES, 4);
        assert_eq!(
            calls.load(Ordering::SeqCst),
            4,
            "one tap observation per block"
        );
        let seen = sink.lock().expect("tap sink").clone();
        assert_eq!(seen.len(), FRAMES * 4);
        // The tap must see what the producing op wrote -- the seeded source, carried unchanged
        // through `PostInputBuiltins` and the three aliases -- not what `PostFader` then made of
        // that same buffer.
        let mut source = SeededSource { state: 0x51ED_0007 };
        let mut expected = Vec::with_capacity(FRAMES * 4);
        for _ in 0..4 {
            let mut left = [0.0_f32; FRAMES];
            let mut right = [0.0_f32; FRAMES];
            source
                .process(GraphBindingBlock {
                    left: &mut left,
                    right: &mut right,
                    first_sample: 0,
                })
                .expect("seeded source");
            expected.extend(left.iter().map(|sample| sample.to_bits()));
        }
        assert_eq!(
            seen, expected,
            "the tap observes the producer's buffer, unchanged by any consumer"
        );
        let left_only: Vec<u32> = observed_bits
            .chunks(FRAMES * 2)
            .flat_map(|block| block[..FRAMES].iter().copied())
            .collect();
        assert_ne!(
            seen, left_only,
            "the next op rewrites that buffer, so a late tap would be detectable"
        );
    }

    /// E4. A worker that misses its bounded recovery deadline never wedges the callback: the
    /// block returns degraded audio in bounded time, the trapped parcel is left alone until the
    /// worker gives it back, and rendering is bit-exact again afterwards.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): make `recovery_iterations` unbounded -- the render
    /// call never returns and the wall-clock guard fires; skip `set_partition_muted` -- the
    /// degraded block reads a buffer a live worker is still writing.
    /// The scheduler's `fault-injection` feature is enabled unconditionally from this crate's
    /// `[dev-dependencies]`, so it is always available in a test build and never in a production,
    /// host or C-ABI one (`scripts/check-scheduler-policy.sh`).
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_late_worker_degrades_one_block_and_never_wedges_the_callback() {
        use miso_engine_native_scheduler::FaultInjectionV1;
        const FRAMES: usize = 4;
        const BLOCKS: u64 = 24;

        let (sequential_graph, sequential_bindings) = native_parallel_sum_plan(48_000);
        let mut sequential = sequential_graph
            .bind(sequential_bindings)
            .unwrap_or_else(|failure| panic!("sequential bind: {}", failure.code));
        let reference: Vec<Vec<u32>> = (0..BLOCKS)
            .map(|block| render_block_at(&mut sequential, FRAMES, block))
            .collect();

        let (native_graph, mut native_bindings) = native_parallel_sum_plan(48_000);
        let (pool, lease, pool_shape) = test_pool(2);
        native_bindings.worker_lease = lease;
        // A stall far longer than the quantum-derived deadline, applied to the first parcel the
        // worker ever takes. The default deadline is used here on purpose: this is the gate.
        let bound = native_graph
            .bind_native(
                native_bindings,
                NativeGraphBindConfigV1 {
                    render_mode: NativeGraphRenderModeV1::DependencyWaves,
                    scheduler: NativeSchedulerConfigV1::new(
                        core::num::NonZeroUsize::new(2).expect("two lanes"),
                        true,
                        pool_shape,
                    )
                    .with_fault(FaultInjectionV1::StallWorker {
                        worker_id: 0,
                        wave_id: u64::MAX,
                        iterations: 40_000_000,
                    }),
                    maximum_retained_bytes: 1 << 20,
                },
            )
            .unwrap_or_else(|failure| panic!("native bind: {}", failure.code));
        assert_eq!(bound.metadata.selection, SchedulerSelectionV1::Parallel);
        let mut native = bound.into_plan();

        let mut rendered: Vec<Vec<u32>> = Vec::with_capacity(BLOCKS as usize);
        for block in 0..BLOCKS {
            let started = std::time::Instant::now();
            let bits = render_block_at(&mut native, FRAMES, block);
            let elapsed = started.elapsed();
            assert!(
                elapsed < std::time::Duration::from_secs(2),
                "block {block} took {elapsed:?}: the callback must be bounded, never wedged"
            );
            assert!(
                bits.iter().all(|word| f32::from_bits(*word).is_finite()),
                "block {block}: even a degraded block renders defined audio"
            );
            rendered.push(bits);
        }
        assert_eq!(
            native.dispatch_counters()[1],
            1,
            "exactly one deadline miss, and the worker is issued to again afterwards"
        );
        let degraded: Vec<usize> = (0..BLOCKS as usize)
            .filter(|block| rendered[*block] != reference[*block])
            .collect();
        assert!(
            !degraded.is_empty(),
            "a trapped partition must not contribute its audio"
        );
        let last_degraded = *degraded.last().expect("one degraded block");
        assert!(
            last_degraded + 1 < BLOCKS as usize,
            "the trapped parcel must come back inside the observation window"
        );
        for block in last_degraded + 1..BLOCKS as usize {
            assert_eq!(
                rendered[block], reference[block],
                "block {block} is bit-exact once the parcel is reaped"
            );
        }
        drop(native);
        if let Some(pool) = pool {
            pool.stop_and_join();
        }
    }

    /// THE gate for #98 F2 and #100 F1/F8: on fifty seeded random DAGs -- stage chains, rack
    /// effects with sidechains, submixes, sends from arbitrary taps, non-trivial 2x2 routes and
    /// PDC on a quarter of the edges -- the sequential executor and the native dependency-wave
    /// executor render bit-identical PCM over eight blocks, at 1, 2, 4 and 7 worker lanes, with
    /// one pool per lane count whose lease is handed from session to session.
    ///
    /// Red mutations (`tests/MUTATIONS.md`): resolve a native op's inputs from the coloured
    /// buffer instead of its producing op; issue commands in ascending worker order so a worker
    /// can wake a child before that child's command exists.
    #[test]
    fn fifty_random_dag_sessions_render_bit_identically_in_both_executors() {
        const FRAMES: usize = 16;
        const BLOCKS: u64 = 8;
        // One pool per lane count, reused for every session: the pool is control-plane state that
        // outlives its plans, and starting 300 of them would only measure thread churn.
        let lane_counts: Vec<usize> = [1_usize, 2, 4, 7]
            .into_iter()
            .filter(|lanes| {
                *lanes == 1
                    || std::thread::available_parallelism()
                        .map(core::num::NonZeroUsize::get)
                        .unwrap_or(1)
                        >= *lanes
            })
            .collect();
        assert!(
            lane_counts.len() >= 3,
            "the determinism matrix needs at least 1/2/4 worker lanes"
        );
        let mut nontrivial = 0_usize;
        let mut sequential_by_seed: Vec<Vec<u32>> = Vec::with_capacity(50);
        for seed in 0..50_u64 {
            let (sequential_plan, sequential_bindings) = random_dag_plan(seed);
            let fan_in = sequential_plan
                .spec
                .edges
                .iter()
                .filter(|edge| matches!(edge.destination.node, GraphNodeId::Output { .. }))
                .count();
            if fan_in >= 4 {
                nontrivial += 1;
            }
            let mut sequential = sequential_plan
                .bind(sequential_bindings)
                .unwrap_or_else(|failure| panic!("seed {seed} sequential bind: {}", failure.code));
            let bits = render_blocks(&mut sequential, FRAMES, BLOCKS);
            assert!(
                bits.iter().any(|value| *value != 0),
                "seed {seed}: the corpus must not be silent"
            );
            sequential_by_seed.push(bits);
        }
        for lanes in lane_counts {
            let (mut pool, mut lease, pool_shape) = test_pool(lanes);
            for (seed, sequential_bits) in sequential_by_seed.iter().enumerate() {
                let seed = seed as u64;
                for mode in [
                    NativeGraphRenderModeV1::SingleThread,
                    NativeGraphRenderModeV1::DependencyWaves,
                ] {
                    let (native_plan, mut native_bindings) = random_dag_plan(seed);
                    native_bindings.worker_lease = lease.take();
                    let bound = native_plan
                        .bind_native(
                            native_bindings,
                            NativeGraphBindConfigV1 {
                                render_mode: mode,
                                scheduler: NativeSchedulerConfigV1::new(
                                    core::num::NonZeroUsize::new(lanes).expect("lanes"),
                                    true,
                                    pool_shape,
                                )
                                .with_recovery_deadline_ns(DETERMINISM_DEADLINE_NS),
                                maximum_retained_bytes: 1 << 28,
                            },
                        )
                        .unwrap_or_else(|failure| {
                            panic!("seed {seed} native bind: {}", failure.code)
                        });
                    let mut native = bound.into_plan();
                    let native_bits = render_blocks(&mut native, FRAMES, BLOCKS);
                    let counters = native.dispatch_counters();
                    assert_eq!(
                        counters[1], 0,
                        "seed {seed}, {lanes} lanes, {mode:?}: a worker missed its deadline, so \
                         this comparison would be measuring a degraded block"
                    );
                    assert_eq!(
                        &native_bits, sequential_bits,
                        "seed {seed}, {lanes} lanes, {mode:?}: executors disagree"
                    );
                    // Releasing the plan returns the lease to the pool for the next session.
                    drop(native);
                    lease = pool
                        .as_mut()
                        .and_then(NativeGraphWorkerPoolV1::recover_lease);
                    assert_eq!(
                        lease.is_some(),
                        lanes > 1,
                        "seed {seed}, {lanes} lanes: the lease must come back"
                    );
                }
            }
            drop(lease);
            if let Some(pool) = pool {
                pool.stop_and_join();
            }
        }
        assert!(
            nontrivial >= 10,
            "the corpus must contain output fan-ins past the balanced/left-to-right divergence"
        );
    }

    #[test]
    fn native_dependency_waves_match_sequential_state_and_pcm_at_launch_rates() {
        for rate in [44_100_u32, 48_000, 88_200, 96_000] {
            let (sequential_graph, sequential_bindings) = native_parallel_sum_plan(rate);
            let (native_graph, mut native_bindings) = native_parallel_sum_plan(rate);
            let mut sequential = match sequential_graph.bind(sequential_bindings) {
                Ok(plan) => plan,
                Err(_) => panic!("sequential binding failed"),
            };
            let (_pool, lease, pool_shape) = test_pool(2);
            native_bindings.worker_lease = lease;
            let prepared = match native_graph.bind_native(
                native_bindings,
                NativeGraphBindConfigV1 {
                    render_mode: NativeGraphRenderModeV1::DependencyWaves,
                    scheduler: NativeSchedulerConfigV1::new(
                        core::num::NonZeroUsize::new(2).expect("two lanes"),
                        true,
                        pool_shape,
                    )
                    .with_recovery_deadline_ns(DETERMINISM_DEADLINE_NS),
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
                    NativeWorkerPoolShapeV1::default(),
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
                    NativeWorkerPoolShapeV1::default(),
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

    /// Binding is transactional at the worker-lease boundary too: a lease whose worker count
    /// does not match the plan's is refused, and every input -- plan, bindings, lease, config --
    /// comes back unchanged and is reusable.
    ///
    /// Red mutation: drop the `graph.scheduler.lease` check in `bind_native_optional_source_set`
    /// -- a mismatched lease is accepted and the executor issues to workers that do not exist.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn a_mismatched_worker_lease_is_refused_and_every_bind_input_returns() {
        let (graph, mut bindings) = native_parallel_sum_plan(48_000);
        let expected_nodes: Vec<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        // A pool of two workers offered to a plan prepared for three.
        let (pool, lease, _shape) = test_pool(3);
        bindings.worker_lease = lease;
        let config = NativeGraphBindConfigV1 {
            render_mode: NativeGraphRenderModeV1::DependencyWaves,
            scheduler: NativeSchedulerConfigV1::new(
                core::num::NonZeroUsize::new(4).expect("four lanes"),
                true,
                NativeWorkerPoolShapeV1 {
                    worker_count: 3,
                    spin_ns: 2,
                },
            ),
            maximum_retained_bytes: 1 << 20,
        };
        let mut failure = match graph.bind_native(bindings, config) {
            Ok(_) => panic!("a mismatched worker lease was accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.lease");
        assert_eq!(failure.plan.plan_id, 48_000);
        assert_eq!(failure.config, config);
        assert_eq!(failure.bindings.envelope, failure.plan.envelope);
        assert!(failure.bindings.observers.is_empty());
        assert_eq!(
            failure
                .bindings
                .worker_lease
                .as_ref()
                .map(NativeGraphWorkerLeaseV1::worker_count),
            Some(2),
            "the lease comes back with everything else"
        );
        assert_eq!(
            failure
                .bindings
                .nodes
                .iter()
                .map(|binding| binding.node.clone())
                .collect::<Vec<_>>(),
            expected_nodes
        );

        // The returned inputs are reusable against the pool the lease actually came from.
        let pool_shape = pool
            .as_ref()
            .map(NativeGraphWorkerPoolV1::shape)
            .expect("pool");
        let recovered_config = NativeGraphBindConfigV1 {
            render_mode: NativeGraphRenderModeV1::DependencyWaves,
            scheduler: NativeSchedulerConfigV1::new(
                core::num::NonZeroUsize::new(3).expect("three lanes"),
                true,
                pool_shape,
            ),
            maximum_retained_bytes: 1 << 20,
        };
        let bindings = core::mem::replace(
            &mut failure.bindings,
            GraphRuntimeBindings {
                envelope: failure.plan.envelope,
                nodes: Vec::new(),
                observers: Vec::new(),
                worker_lease: None,
            },
        );
        let recovered = failure
            .plan
            .bind_native(bindings, recovered_config)
            .unwrap_or_else(|retry| panic!("returned inputs were not reusable: {}", retry.code));
        assert_eq!(recovered.metadata.selection, SchedulerSelectionV1::Parallel);
        assert_eq!(recovered.metadata.resources.scheduler.worker_count, 2);
        drop(recovered);
        drop(pool);
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
                nodes: sorted_nodes(nodes),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input_a.clone(), input_b.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![route_a.clone(), route_b.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output_node.clone()],
                },
            ],
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
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
                nodes: sorted_nodes(
                    schedule
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: LatencySamples(0),
                            tail: TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges: vec![main_edge, sidechain_edge, route_source, route_destination],
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![main_input.clone(), sidechain_input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![effect_node.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![route_node.clone()],
                },
                DependencyLevel {
                    level: 3,
                    nodes: vec![output_node.clone()],
                },
            ],
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
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
                nodes: sorted_nodes(graph_nodes),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input_direct.clone(), input_effect.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![effect_node.clone(), route_direct.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![route_effect.clone()],
                },
                DependencyLevel {
                    level: 3,
                    nodes: vec![output_node.clone()],
                },
            ],
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
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
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
