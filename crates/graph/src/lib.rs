//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

pub mod corpus;
pub mod program;
mod runtime;

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use runtime::{
    TestOnlyBankChainConstructionFacts, TestOnlyBankChainInputs, TestOnlyBankChainOwnership,
    test_only_bank_chain_construction_facts, test_only_bank_chain_ownership,
    test_only_builtin_processor_mut, test_only_builtin_stage, test_only_prepare_bank_chain_inputs,
    test_only_reset_bank_chain_construction_facts,
};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use runtime::{
    test_only_bank_meter_passes, test_only_bank_sample_peak_passes, test_only_carry_payload_bytes,
    test_only_disable_bank_skip, test_only_disable_lane_packing, test_only_meter_input_counts,
    test_only_meter_input_reset, test_only_observation_dispatch_counts,
    test_only_observation_dispatch_reset, test_only_reset_carry_payload_bytes,
    test_only_resident_input_counts, test_only_resident_input_reset,
    test_only_route_activity_built, test_only_route_drain_calls, test_only_route_drain_snapshots,
    test_only_route_drained_counts, test_only_route_mix_counts, test_only_route_mix_reset,
    test_only_set_bank_meter_declined, test_only_set_bank_sample_peak_declined,
    test_only_set_route_fold_declined, test_only_set_scatter_redirect_declined,
    test_only_set_source_in_place_declined, test_only_source_plane_counts,
    test_only_source_plane_reset, test_only_state_every_silence_fact,
};

use core::cell::Cell;
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet};

use effect_contract::NodeTailBound;
use effect_contract::{
    BankWidth, ChannelSymmetryWitness, EffectControlLane, LatencySamples, ObservationLane,
    PreparedEffectMetadata, PreparedNativeEffect, ResponseAnalysisError, ResponseSnapshotRequest,
    ResponseSnapshotSummary, SeamSide,
};
use engine::realtime::{
    BufferArena, CarryOutcome, ExecutorBlockReport, LiveSnapshot, ObserverCarry, ObserverKind,
    PlanUnitEligibility, PlanarBufferMut, PrepareRenderPlan, PreparedPlanExecutor,
    PreparedRenderPlan, RenderEnvelope, RenderError, ResponseSnapshotError, ResponseSnapshotSink,
};
use lane::Backend;
use rack::AoSoaScratch;

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
impl StableGraphId {
    /// The heap bytes the ID's text retains, at its capacity (#1074).
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        u64::try_from(self.0.capacity()).unwrap_or(u64::MAX)
    }
}
impl EffectNodeId {
    /// The heap bytes the ID's two texts retain (#1074).
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        self.track_id
            .retained_bytes()
            .saturating_add(self.effect_id.retained_bytes())
    }
}
impl GraphNodeId {
    /// The heap bytes the ID retains (#1074): its texts, and a compensation delay's boxed edge.
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        match self {
            Self::TrackStage { track_id: id, .. }
            | Self::Route { route_id: id }
            | Self::Submix { submix_id: id }
            | Self::Output { output_id: id } => id.retained_bytes(),
            Self::Effect(effect) => effect.retained_bytes(),
            Self::CompensationDelay { edge_id } => engine::realtime::retained::boxed(&**edge_id)
                .saturating_add(edge_id.retained_bytes()),
        }
    }
}
impl GraphEdgeId {
    /// The heap bytes the ID retains (#1074): its texts, at capacity.
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        match self {
            Self::TrackMain { target } => target.retained_bytes(),
            Self::RouteSource { route_id } | Self::RouteDestination { route_id } => {
                route_id.retained_bytes()
            }
            Self::EffectSidechain { effect, port } => effect
                .retained_bytes()
                .saturating_add(u64::try_from(port.capacity()).unwrap_or(u64::MAX)),
        }
    }
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
    /// The node's whole tail bound (#1379 Amendment 1 H2): `T_decay`, `T_rest`, the exact-rest
    /// bound and the composition values the graph extent composes through gain. Control-side
    /// plan data: no render-owned structure holds a `GraphNode`.
    pub bound: NodeTailBound,
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
/// A source-claim compensation line (#1287 D1): a floor on a track's `Input` node (#1285 D1)
/// raises that node's input arrival, and a track's `Input` reads its source through a claim, not
/// through a graph edge, so the compiler realises the raise as a line of `samples` on both lanes
/// of the claim. A submix's `Input` never has one: the routes into it carry its compensation.
///
/// This is a report row. The compiler lowers the line by adding `samples` to both lanes of the
/// node's [`PreparedTrackDelay`] entry (creating it when the session declared no delay), so it runs
/// on the track-delay line and kernel (`runtime::TrackDelayLine`, `pdc_delay_block`). Unlike a
/// declared track delay it is compensation: its samples count in `total_delay_samples` and the PDC
/// caps. Empty for every plan compiled without floors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceClaimLine {
    /// The track's `TrackStage::Input` node.
    pub node: GraphNodeId,
    /// The line's length on each lane: the node's floored arrival minus its incoming arrival (0).
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
    /// Exact additional buffers a native-effect bank's chain owns to gather and scatter: two
    /// lane-major staging planes, and a padded bank's `quantum`-word `+0.0` plane (issue #887).
    pub effect_bank_runtime_buffer_bytes: u64,
    /// Checked bank/member metadata retained before render-plan binding.
    pub effect_bank_metadata_bytes: u64,
    /// Exact prepared post-input builtin bank payload retained by the graph.
    pub builtin_bank_bytes: u64,
    /// Exact `f32` plane payload retained by post-input builtin banks: two AoSoA scratch planes
    /// and the chain's two staging planes per bank, and a padded bank's `+0.0` plane (issue #887).
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
    pub maximum_mask_bytes: u64,
    pub largest_allocation_bytes: u64,
}

/// Checked retained storage for live scalar fader/matrix owners lowered after preparation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphScalarOwnerResourceEstimate {
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
}

/// Checked runtime metadata retained by every graph executor. The semantic graph estimate does
/// not include the lowered runtime layouts. The largest `RuntimeUnit`/`RuntimeOp` allocation is
/// bounded by the larger of the two layouts multiplied by the emitted-op bound, and the executor
/// owner by its own layout; both are reported for the largest-allocation cap only.
///
/// The executor also retains two boxed tables it sizes at bind (issue #936): the indices of the
/// units its render loop dispatches, and the `(claim, arena buffer)` rows of the source claims
/// its copy loop fills. Their lengths are decided at bind, after this estimate is admitted, so
/// each is charged at its bound: one entry per emitted op. A dispatched unit is a unit, and a unit
/// holds at least one op; a copied claim names a distinct graph node, which lowers to at most one
/// op.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphRuntimeMetadataResourceEstimate {
    pub emitted_op_count: u64,
    pub runtime_op_containing_bytes: u64,
    pub runtime_unit_containing_bytes: u64,
    /// The `GraphExecutor` owner layout, reported for the largest-allocation proof.
    pub runtime_owner_allocation_bytes: u64,
    /// Boxed response-owner binding table retained by the prepared runtime.
    pub response_binding_table_bytes: u64,
    /// Payload bytes retained by the per-owner stable-id and track-id strings.
    pub response_binding_string_bytes: u64,
    /// Largest individual cloned identity allocation.
    pub largest_response_binding_string_bytes: u64,
    /// The executor's boxed table of dispatched unit indices (one `u32` each), charged at one
    /// entry per emitted op (issue #936).
    pub active_unit_table_bytes: u64,
    /// The executor's boxed table of the source claims its copy loop fills (one
    /// `(usize, u32)` row each), charged at one entry per emitted op (issue #936).
    pub source_input_table_bytes: u64,
    /// The runtime's boxed table of the units that hold an observer accepting
    /// [`GraphRuntimeObserver::block_end`] (one `u32` each), charged at one entry per emitted op
    /// (issue #1609).
    pub block_end_unit_table_bytes: u64,
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
}

impl GraphRuntimeMetadataResourceEstimate {
    /// Computes the checked retained field and conservative mixed op/unit allocation bound.
    pub fn checked_for(emitted_op_count: u64) -> Option<Self> {
        Self::checked_for_with_response_bindings(emitted_op_count, 0, 0, 0)
    }

    /// Computes runtime metadata plus the retained response-owner table and its cloned identity
    /// strings. The binding count/string payloads are supplied by the graph compiler because the
    /// lowered runtime owns those values after the semantic model is consumed.
    pub fn checked_for_with_response_bindings(
        emitted_op_count: u64,
        response_binding_count: u64,
        response_binding_string_bytes: u64,
        largest_response_binding_string_bytes: u64,
    ) -> Option<Self> {
        let runtime_owner_allocation_bytes = u64::try_from(core::mem::size_of::<GraphExecutor>())
            .expect("runtime owner allocation fits u64");
        let runtime_op_bytes = u64::try_from(core::mem::size_of::<runtime::RuntimeOp>())
            .expect("runtime op layout fits u64");
        let runtime_unit_bytes = u64::try_from(core::mem::size_of::<runtime::RuntimeUnit>())
            .expect("runtime unit layout fits u64");
        let runtime_op_containing_bytes = runtime_op_bytes.checked_mul(emitted_op_count)?;
        let runtime_unit_containing_bytes = runtime_unit_bytes.checked_mul(emitted_op_count)?;
        let response_binding_entry_bytes =
            u64::try_from(core::mem::size_of::<runtime::ResponseOwnerBinding>())
                .expect("response binding layout fits u64");
        let response_binding_table_bytes =
            response_binding_entry_bytes.checked_mul(response_binding_count)?;
        let (active_unit_entry_bytes, source_input_entry_bytes) = executor_table_entry_bytes();
        let active_unit_table_bytes = active_unit_entry_bytes.checked_mul(emitted_op_count)?;
        let source_input_table_bytes = source_input_entry_bytes.checked_mul(emitted_op_count)?;
        let block_end_unit_table_bytes = u64::try_from(core::mem::size_of::<u32>())
            .expect("block-end unit entry fits u64")
            .checked_mul(emitted_op_count)?;
        let total_bytes =
            response_binding_table_bytes.checked_add(response_binding_string_bytes)?;
        let total_bytes = total_bytes
            .checked_add(active_unit_table_bytes)?
            .checked_add(source_input_table_bytes)?
            .checked_add(block_end_unit_table_bytes)?;
        Some(Self {
            emitted_op_count,
            runtime_op_containing_bytes,
            runtime_unit_containing_bytes,
            runtime_owner_allocation_bytes,
            response_binding_table_bytes,
            response_binding_string_bytes,
            largest_response_binding_string_bytes,
            active_unit_table_bytes,
            source_input_table_bytes,
            block_end_unit_table_bytes,
            total_bytes,
            largest_allocation_bytes: runtime_owner_allocation_bytes
                .max(runtime_op_containing_bytes)
                .max(runtime_unit_containing_bytes)
                .max(response_binding_table_bytes)
                .max(largest_response_binding_string_bytes)
                .max(active_unit_table_bytes)
                .max(source_input_table_bytes)
                .max(block_end_unit_table_bytes),
        })
    }
}

/// Conservative coexistence reservation for runtime bank slots and their masks.
///
/// `bank_count` is the combined prepared effect and planned builtin membership count. `mask_bytes`
/// is the largest selected bank mask in bytes, including the target's native bool layout.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphBankSlotResourceEstimate {
    pub bank_count: u64,
    pub mask_bytes: u64,
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
}

impl GraphBankSlotResourceEstimate {
    /// Computes the checked slot reservation for one selected dispatch width.
    pub fn checked_for(bank_count: u64, width: Option<BankWidth>) -> Option<Self> {
        if bank_count == 0 {
            return Some(Self::default());
        }
        let width = width?;
        let mask_bytes = u64::from(width.lanes())
            .checked_mul(u64::try_from(core::mem::size_of::<bool>()).ok()?)?;
        Self::checked_for_mask(bank_count, mask_bytes)
    }

    /// Computes the same reservation from the largest mask observed in prepared banks.
    pub fn checked_for_mask(bank_count: u64, mask_bytes: u64) -> Option<Self> {
        if bank_count == 0 {
            return Some(Self::default());
        }
        let stage_pointer_bytes = u64::try_from(core::mem::size_of::<Box<dyn rack::BankStage>>())
            .ok()?
            .checked_mul(bank_count)?;
        let slot_bytes = u64::try_from(core::mem::size_of::<rack::BankSlot>())
            .ok()?
            .checked_mul(bank_count)?;
        let total_bytes = bank_count.checked_mul(
            u64::try_from(core::mem::size_of::<Box<dyn rack::BankStage>>())
                .ok()?
                .checked_add(
                    u64::try_from(core::mem::size_of::<rack::BankSlot>())
                        .ok()?
                        .checked_mul(3)?,
                )?
                .checked_add(mask_bytes.checked_mul(3)?)?,
        )?;
        Some(Self {
            bank_count,
            mask_bytes,
            total_bytes,
            largest_allocation_bytes: stage_pointer_bytes.max(slot_bytes).max(mask_bytes),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphBuiltinBankAttachError {
    InvalidMembers,
    IncompatibleMembers,
    ResourceMismatch,
    ResourceOverflow,
}

impl GraphResourceEstimate {
    /// Adds the conservative runtime bank-slot reservation transactionally.
    pub fn checked_add_bank_slot_owners(
        &mut self,
        resource: GraphBankSlotResourceEstimate,
    ) -> Option<()> {
        self.checked_add_metadata_bytes(resource.total_bytes, resource.largest_allocation_bytes)
    }

    pub fn checked_add_scalar_owners(
        &mut self,
        resource: GraphScalarOwnerResourceEstimate,
    ) -> Option<()> {
        self.checked_add_metadata_bytes(resource.total_bytes, resource.largest_allocation_bytes)
    }

    /// Folds the retained executor/runtime metadata once, before graph caps are applied.
    pub fn checked_add_runtime_metadata(
        &mut self,
        resource: GraphRuntimeMetadataResourceEstimate,
    ) -> Option<()> {
        self.checked_add_metadata_bytes(resource.total_bytes, resource.largest_allocation_bytes)
    }

    fn checked_add_metadata_bytes(&mut self, total_bytes: u64, largest_bytes: u64) -> Option<()> {
        let metadata = self.graph_metadata_bytes.checked_add(total_bytes)?;
        let incremental = self.incremental_plan_bytes.checked_add(total_bytes)?;
        let session_plus_plan = self.session_plus_plan_bytes.checked_add(total_bytes)?;
        self.graph_metadata_bytes = metadata;
        self.incremental_plan_bytes = incremental;
        self.session_plus_plan_bytes = session_plus_plan;
        self.largest_allocation_bytes = self.largest_allocation_bytes.max(largest_bytes);
        Some(())
    }

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

/// What silences a route, beside its gain and matrix (DESIGN 5.7, issue #1215).
///
/// `mute` silences the whole route; `follow_zeroed[lane]` zeroes the column that carries that
/// source lane (left feeds `ll` and `rl`, right feeds `lr` and `rr`). Every prepared route is
/// [`RouteGate::OPEN`] until a session can mute a route or let it follow its source strip's mute.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RouteGate {
    pub mute: bool,
    pub follow_zeroed: [bool; 2],
}

impl RouteGate {
    pub const OPEN: Self = Self {
        mute: false,
        follow_zeroed: [false; 2],
    };

    /// Muted, or follow-zeroed on both source lanes: the route contributes nothing.
    #[must_use]
    pub const fn silences(self) -> bool {
        self.mute || (self.follow_zeroed[0] && self.follow_zeroed[1])
    }
}

/// The four coefficients a route applies: its 2x2 with the linear gain folded in, gated.
///
/// The one derivation of a route's constants (D3, #98 F4; DESIGN 5.7): the runtime binds a route
/// op and a fold lane from it, and a live producer pushes what it returns, so the bits a plan binds
/// and the bits a live change sends cannot differ. A silencing gate returns `[+0.0; 4]`; otherwise
/// each product is `gain * coefficient` in that operand order, unfused, with the column of a
/// follow-zeroed source lane replaced by `+0.0`. A product that is subnormal, of either sign, is
/// flushed to `+0.0` (issue #1237 D3): inside the route domain (the session model's
/// `ROUTE_GAIN_DB_MINIMUM`, `ROUTE_GAIN_DB_MAXIMUM` and `ROUTE_COEFFICIENT_MAGNITUDE_MAXIMUM`,
/// which this crate cannot link) a coefficient below about `1.9e-31` at today's minimum gain folds
/// to one, and a subnormal constant would only cost the render multiply its slow path for an
/// inaudible contribution.
#[must_use]
pub const fn gated_route_coefficients(transform: &RouteTransform, gate: RouteGate) -> [f32; 4] {
    /// `gain * coefficient`, a subnormal result flushed to `+0.0`.
    const fn fold(gain: f32, coefficient: f32) -> f32 {
        let product = gain * coefficient;
        if product.is_subnormal() { 0.0 } else { product }
    }
    if gate.silences() {
        return [0.0; 4];
    }
    let [left_zeroed, right_zeroed] = gate.follow_zeroed;
    [
        if left_zeroed {
            0.0
        } else {
            fold(transform.gain, transform.ll)
        },
        if right_zeroed {
            0.0
        } else {
            fold(transform.gain, transform.lr)
        },
        if left_zeroed {
            0.0
        } else {
            fold(transform.gain, transform.rl)
        },
        if right_zeroed {
            0.0
        } else {
            fold(transform.gain, transform.rr)
        },
    ]
}
/// The graph's frozen reduction, over one frame's worth of contributions (evidence only).
///
/// Render never calls this: it reduces whole blocks through `lane`'s `sum2_block` and
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
            lane::kernels::sum2_block::<f32>(&mut left, &[*first], &[*second]);
            for next in rest {
                lane::kernels::sum_into_block::<f32>(&mut left, &[*next]);
            }
            left[0]
        }
    }
}

pub struct PreparedGraphPlan {
    /// The executable form of this plan, derived at construction (#99 F2).
    ///
    /// Both executors are built on this lowering: everything in `crate::runtime` -- the ops, their
    /// input order, the identity aliases and the buffer colouring -- is derived from an
    /// `ExecutionProgram` rather than from a second reading of the semantic graph.
    ///
    /// They do not read *this* copy. Binding calls `lowered`, which re-derives from the plan's
    /// *current* fields, because the schedule, levels and inserted delays are public and the
    /// transactional bind contract hands a rejected plan back to be repaired and re-bound. This
    /// construction-time copy is the compile-time gate instead: it is validated on every compile
    /// (see `graph_plans_always_lower_to_an_executable_program`), so a plan that cannot lower is
    /// caught where it is built rather than at bind.
    program: Option<program::ExecutionProgram>,
    /// The compiler's resolved node positions, which every lowering of this plan takes as hints
    /// (#1749). Empty unless the plan was built by [`new_hinted`](Self::new_hinted); a hint that
    /// no longer matches the public fields is checked and ignored, so it never changes a lowering.
    lowering_hint: program::LoweringHint,
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
    /// Issue #210 phase 2: input-side track alignment, one entry per delayed track. Empty on every
    /// session that declared no delay.
    track_delays: Vec<PreparedTrackDelay>,
    /// #1287 D1: the source-claim lines folded into `track_delays`, in node order. Empty for every
    /// plan compiled without floors.
    source_claim_lines: Vec<SourceClaimLine>,
    effects: Vec<GraphPreparedEffect>,
    /// Issue #140 A: one entry per effect live controls drive. Empty for every session that
    /// asked for no live controls, which is what keeps the runtime on its byte-identical path.
    effect_controls: Vec<GraphEffectControlBinding>,
    /// Issue #143 D3: one entry per effect that has observation taps. Empty for every session
    /// that named no observation capacity, which is what keeps the runtime unobserved *and*
    /// byte-identical rather than merely disabled.
    effect_observations: Vec<GraphEffectObservationBinding>,
    /// Issue #1220 D6: one lane per live route, `None` until
    /// [`attach_route_controls`](Self::attach_route_controls) runs. `None` and an empty list both
    /// bind every route op to its prepared constants, exactly as a plan without the field did.
    route_controls: Option<Vec<GraphRouteControlBinding>>,
    /// #1652b: one seven-word lane per route into a submix whose source strip has an automated
    /// mute lane. Empty until #1652c's attach fills it, so every route binds as it did before.
    follow_route_controls: Vec<GraphFollowRouteControlBinding>,
    banks: Vec<GraphPreparedEffectBank>,
    builtin_banks: Vec<GraphPreparedBuiltinBank>,
    observers: Vec<GraphNodeObserverBinding>,
    /// Issue #2135: the strips whose stored automation drives one of their effect bypasses, by
    /// strip ID ([`pin_bypass_automated_strips`](Self::pin_bypass_automated_strips)). The lane
    /// packer pins them for the plan's life. Empty unless the compiler names one.
    pinned_strips: Vec<Box<str>>,
    _not_sync: Cell<()>,
}
/// One prepared per-node effect in the plan's control-side effect table, keyed by its node `id`.
///
/// This record itself never enters render-owned memory. Bind reads its metadata and response
/// facts on the control thread and moves only what render reads -- the processor and the
/// metadata's block quantum -- into the render node (`runtime::EffectNode`). Control-only data,
/// such as the rest of [`PreparedEffectMetadata`], therefore stays here and costs the render node
/// table nothing. The `processor` does enter render-owned memory, and each effect's processor
/// still holds its own `PreparedEffectMetadata` copy; #1461 removes that copy.
pub struct GraphPreparedEffect {
    pub id: EffectNodeId,
    pub metadata: PreparedEffectMetadata,
    pub processor: Box<dyn PreparedNativeEffect>,
    /// Factory-declared response capability, retained beside the prepared processor so an
    /// unsupported hook is a required-owner refusal rather than a silent exclusion.
    pub response_snapshot_declared: bool,
    /// Static descriptor identity of the prepared native effect.
    pub native_id: &'static str,
}

/// Bytes of the render node a per-node prepared effect binds to (`runtime::EffectNode`): the
/// processor and the block quantum, and nothing else. A live-control-driven effect's boxed owner
/// holds one, so `graph-compiler`'s resource estimate charges this size for it.
pub const EFFECT_RENDER_NODE_BYTES: usize = core::mem::size_of::<runtime::EffectNode>();

/// Bytes of a live-control-driven per-node effect's boxed owner (`runtime::LiveControlEffect`),
/// padding included, which `graph-compiler`'s resource estimate charges for it. One size, so a
/// new field of the owner is charged without a second edit (#1816).
pub const LIVE_CONTROL_EFFECT_BYTES: usize = core::mem::size_of::<runtime::LiveControlEffect>();

/// One prepared effect's live-control channel, carried **beside** the prepared effects
/// rather than inside them (issue #140 A).
///
/// # Why beside, and not a field of [`GraphPreparedEffect`]
///
/// A `GraphPreparedEffect`'s render-read fields become the payload of `runtime::NodeKind::Effect`.
/// Runtime op/unit layout changes are admitted separately by the derived runtime metadata
/// reservation, while this channel remains in its own vector, so a live-control-free plan retains
/// no control-channel payload.
pub struct GraphEffectControlBinding {
    /// The effect node this channel drives.
    pub node: EffectNodeId,
    /// Consumer half of the bounded channel; the producer stays on the control plane.
    pub control: Box<EffectControlLane>,
}

/// One prepared effect's observation taps, carried beside the prepared effects (issue #143 D3).
///
/// Beside, and not inside, for the same ownership reason as [`GraphEffectControlBinding`]: an
/// observation channel is retained only for the prepared observation population.
pub struct GraphEffectObservationBinding {
    /// The effect node these taps observe.
    pub node: EffectNodeId,
    /// The render-side lane; the readers stay on the control plane.
    pub observation: Box<ObservationLane>,
}

/// The longest live-route ramp in samples (issue #1220 D2): the indexed ramp's own bound.
pub const ROUTE_RAMP_LENGTH_MAXIMUM: u32 = lane::kernels::INDEXED_RAMP_LENGTH_MAXIMUM;

/// One live change of a send's coefficients (issue #1220 D2).
///
/// `target` is what `graph_compiler::route_coefficients` returns for the route's new gain, matrix,
/// mute and follow state; `mute` is true when that gate silences the route; `length` is the ramp
/// in samples, `0` being a step at the block boundary. The constructor is the only way to build
/// one, so every record a lane drains is in bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RouteControlRecord {
    target: [f32; 4],
    mute: bool,
    length: u32,
}

impl RouteControlRecord {
    // REALTIME_POLICY_BEGIN
    /// `None` when `length > ROUTE_RAMP_LENGTH_MAXIMUM`, or when `mute` is true and any target
    /// coefficient is not `+0.0` (bitwise): a muted route's coefficients are the four `+0.0` its
    /// gate gives ([`gated_route_coefficients`]).
    #[must_use]
    pub fn new(target: [f32; 4], mute: bool, length: u32) -> Option<Self> {
        if length > ROUTE_RAMP_LENGTH_MAXIMUM
            || (mute && target.iter().any(|coefficient| coefficient.to_bits() != 0))
        {
            return None;
        }
        Some(Self {
            target,
            mute,
            length,
        })
    }
    /// The coefficients `[ll, lr, rl, rr]` the ramp ends on.
    #[must_use]
    pub const fn target(&self) -> [f32; 4] {
        self.target
    }
    /// Whether the route is silenced once the ramp ends.
    #[must_use]
    pub const fn mute(&self) -> bool {
        self.mute
    }
    /// The ramp's length in samples; `0` is a step.
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.length
    }
    // REALTIME_POLICY_END
}

/// Words of one route cell: the four target coefficients' bits, the mute flag and the ramp length
/// (#1347 D1). A target and its ramp are one unit, so they share one cell.
pub(crate) const ROUTE_CELL_WORDS: usize = 6;

impl RouteControlRecord {
    /// The cell words `[ll, lr, rl, rr, mute, length]`.
    pub(crate) fn to_words(self) -> [u32; ROUTE_CELL_WORDS] {
        let [ll, lr, rl, rr] = self.target;
        [
            ll.to_bits(),
            lr.to_bits(),
            rl.to_bits(),
            rr.to_bits(),
            u32::from(self.mute),
            self.length,
        ]
    }

    // REALTIME_POLICY_BEGIN
    /// The record a cell's words hold: `None` for words no record wrote, which render skips. The
    /// words come from [`Self::to_words`], so the check is the constructor's own.
    pub(crate) fn from_words(words: [u32; ROUTE_CELL_WORDS]) -> Option<Self> {
        let [ll, lr, rl, rr, mute, length] = words;
        if mute > 1 {
            return None;
        }
        Self::new(
            [
                f32::from_bits(ll),
                f32::from_bits(lr),
                f32::from_bits(rl),
                f32::from_bits(rr),
            ],
            mute == 1,
            length,
        )
    }
    // REALTIME_POLICY_END
}

/// What one live route shares between its writer (control) and its reader (render): one
/// latest-target cell and its dirty word (#1347 D1).
pub(crate) struct RouteLane {
    pub(crate) cell: engine::realtime::LatestCell<ROUTE_CELL_WORDS>,
    pub(crate) dirty: engine::realtime::DirtyWord,
}

impl RouteLane {
    fn new() -> Self {
        Self {
            cell: engine::realtime::LatestCell::new(),
            dirty: engine::realtime::DirtyWord::new(),
        }
    }
}

/// The render-side half of one live route's latest-target cell (issue #1220 D6, #1347 D1): the
/// shared cell, the reader's last applied sequence, the dirty reader and the session's
/// `live_values_superseded` count.
///
/// Built only by [`PreparedGraphPlan::attach_route_controls`]; its route op reads it under the
/// block's live snapshot at the start of every block.
pub struct RouteControlLane {
    pub(crate) lane: std::sync::Arc<RouteLane>,
    pub(crate) reader: engine::realtime::CellReader,
    pub(crate) dirty: engine::realtime::DirtyReader,
    pub(crate) superseded: engine::realtime::LiveValuesSuperseded,
}

/// One live route's lane, carried **beside** the prepared routes (issue #1220 D6), as
/// [`GraphEffectControlBinding`] is beside the prepared effects: a plan without route controls
/// holds none, and its route ops bind exactly the constants they bound before.
pub struct GraphRouteControlBinding {
    /// The route node this lane drives.
    pub node: GraphNodeId,
    /// Reader half of the latest-target cell; the writer stays on the control plane. `None` for a
    /// transition route (#1363 D2): a route a swap adds with no lane in this plan, which binds
    /// live (it never folds) only so its fade-in can ramp it.
    pub control: Option<Box<RouteControlLane>>,
}

/// The control-plane half of one live route's latest-target cell (issue #1220 D7, #1347 D2).
pub struct GraphRouteControlProducer {
    /// The route's session ID.
    pub route_id: Box<str>,
    lane: std::sync::Arc<RouteLane>,
    writer: engine::realtime::CellWriter<ROUTE_CELL_WORDS>,
    /// The bytes of the plan's route-activity table that this attach added and the compile-time
    /// estimate did not charge (D8): the same for every producer of one attach.
    activity_bytes: u64,
    /// The largest single allocation of that table, or `0` with it.
    activity_largest: u64,
}

impl GraphRouteControlProducer {
    /// Write `record` as the route's newest target, stamped with the revision of the transaction
    /// that commits it. A write cannot fail and never waits, however many writes render has not
    /// read: render ramps to the newest and counts each replaced one.
    ///
    /// Validation is [`RouteControlRecord::new`]'s, made before any write. The caller publishes the
    /// stamp's revision on the plan's gate after every write of the transaction; a value is not
    /// applied before that (#1312 D7).
    pub fn write(&mut self, stamp: engine::realtime::LiveStamp<'_>, record: RouteControlRecord) {
        self.writer.write(
            &self.lane.cell,
            &self.lane.dirty,
            0,
            stamp,
            record.to_words(),
        );
    }
}

#[cfg(any(test, feature = "test-support"))]
impl GraphRouteControlProducer {
    /// Test-only: how many writes the cell has taken. Valid while no write is in flight, which a
    /// quiescent test thread guarantees.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_count(&self) -> u64 {
        engine::realtime::CellReader::new()
            .peek_unread(&self.lane.cell)
            .map_or(0, |(_, sequence)| sequence)
    }
}

/// Words of a following route's cell (#1652 D1): the open transform `[gain, ll, lr, rl, rr]`'s
/// bits, the flags word ([`RouteFollowFlags`]) and the ramp length. A route into a submix whose
/// source strip has an automated mute lane holds this cell instead of the six folded words, so
/// render can gate the transform itself at every curve event.
pub(crate) const FOLLOW_ROUTE_CELL_WORDS: usize = 7;

/// The flags word of a following route's cell (#1652 D1, batch 18 layout): bit 0 the route's own
/// `mute`, bit 1 `follows_mute`, bits 2 and 3 `terms[L, R]`, bits 4 and 5 `automated[L, R]`. Every
/// other bit is 0; [`Self::from_word`] refuses a word with any of them set.
///
/// `terms[lane]` is the source lane's effective mute without its curve (`controls::effective`'s
/// source-lane mute: static mute, VCA mute and solo term). `automated[lane]` is fixed by the plan:
/// whether that source lane has a stored mute curve (its own or a reaching VCA's, #1655 D4).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RouteFollowFlags {
    /// The route's own mute: it silences the route whatever the source does.
    pub mute: bool,
    /// Whether the route follows its source strip's mute.
    pub follows_mute: bool,
    /// Per source lane `[L, R]`: the lane's mute terms other than its curve.
    pub terms: [bool; 2],
    /// Per source lane `[L, R]`: whether the plan automates the lane's mute.
    pub automated: [bool; 2],
}

impl RouteFollowFlags {
    const MUTE: u32 = 1 << 0;
    const FOLLOWS_MUTE: u32 = 1 << 1;
    const TERMS: [u32; 2] = [1 << 2, 1 << 3];
    const AUTOMATED: [u32; 2] = [1 << 4, 1 << 5];
    /// Every bit the layout defines; any other bit set makes a word no record wrote.
    const DEFINED: u32 = (1 << 6) - 1;

    /// The flags word.
    #[must_use]
    pub const fn to_word(self) -> u32 {
        const fn bit(set: bool, bit: u32) -> u32 {
            if set { bit } else { 0 }
        }
        bit(self.mute, Self::MUTE)
            | bit(self.follows_mute, Self::FOLLOWS_MUTE)
            | bit(self.terms[0], Self::TERMS[0])
            | bit(self.terms[1], Self::TERMS[1])
            | bit(self.automated[0], Self::AUTOMATED[0])
            | bit(self.automated[1], Self::AUTOMATED[1])
    }

    // REALTIME_POLICY_BEGIN
    /// The flags a word holds, or `None` when a bit above bit 5 is set.
    #[must_use]
    pub const fn from_word(word: u32) -> Option<Self> {
        if word & !Self::DEFINED != 0 {
            return None;
        }
        Some(Self {
            mute: word & Self::MUTE != 0,
            follows_mute: word & Self::FOLLOWS_MUTE != 0,
            terms: [word & Self::TERMS[0] != 0, word & Self::TERMS[1] != 0],
            automated: [
                word & Self::AUTOMATED[0] != 0,
                word & Self::AUTOMATED[1] != 0,
            ],
        })
    }

    /// The route's gate while the source lanes' curves are at `curve` (#1652 D2):
    /// `follow_zeroed[lane] = follows_mute && ((automated[lane] && curve[lane]) || terms[lane])`. A
    /// lane the plan does not automate ignores its `curve` entry. Pure; [`gated_route_coefficients`]
    /// of the open transform with this gate is the route's target.
    #[must_use]
    pub const fn gate(self, curve: [bool; 2]) -> RouteGate {
        let [left, right] = [
            self.follows_mute && ((self.automated[0] && curve[0]) || self.terms[0]),
            self.follows_mute && ((self.automated[1] && curve[1]) || self.terms[1]),
        ];
        RouteGate {
            mute: self.mute,
            follow_zeroed: [left, right],
        }
    }
    // REALTIME_POLICY_END
}

/// One live change of a following route (#1652 D1): its open transform, its flags and the ramp in
/// samples, `0` being a step. Render composes the target itself
/// ([`RouteFollowFlags::gate`], [`gated_route_coefficients`]), so the record holds no folded
/// coefficients. The constructor is the only way to build one, so every record a lane drains is in
/// bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FollowRouteControlRecord {
    transform: RouteTransform,
    flags: RouteFollowFlags,
    length: u32,
}

impl FollowRouteControlRecord {
    // REALTIME_POLICY_BEGIN
    /// `None` when `length > ROUTE_RAMP_LENGTH_MAXIMUM` or when a transform word is not finite: the
    /// gate folds the transform at every curve event, so it must give finite coefficients.
    #[must_use]
    pub fn new(transform: RouteTransform, flags: RouteFollowFlags, length: u32) -> Option<Self> {
        let RouteTransform {
            gain,
            ll,
            lr,
            rl,
            rr,
        } = transform;
        if length > ROUTE_RAMP_LENGTH_MAXIMUM
            || ![gain, ll, lr, rl, rr].iter().all(|word| word.is_finite())
        {
            return None;
        }
        Some(Self {
            transform,
            flags,
            length,
        })
    }
    /// The open transform: the route's linear gain and its 2x2, ungated.
    #[must_use]
    pub const fn transform(&self) -> RouteTransform {
        self.transform
    }
    /// The flags word's fields.
    #[must_use]
    pub const fn flags(&self) -> RouteFollowFlags {
        self.flags
    }
    /// The ramp's length in samples; `0` is a step.
    #[must_use]
    pub const fn length(&self) -> u32 {
        self.length
    }
    // REALTIME_POLICY_END

    /// The cell words `[gain, ll, lr, rl, rr, flags, length]`.
    pub(crate) fn to_words(self) -> [u32; FOLLOW_ROUTE_CELL_WORDS] {
        let RouteTransform {
            gain,
            ll,
            lr,
            rl,
            rr,
        } = self.transform;
        [
            gain.to_bits(),
            ll.to_bits(),
            lr.to_bits(),
            rl.to_bits(),
            rr.to_bits(),
            self.flags.to_word(),
            self.length,
        ]
    }

    // REALTIME_POLICY_BEGIN
    /// The record a cell's words hold: `None` for words no record wrote (a flags bit above bit 5,
    /// a non-finite transform word or a ramp over the bound), which render skips. The words come
    /// from [`Self::to_words`], so the checks are the constructor's and the flags word's own.
    pub(crate) fn from_words(words: [u32; FOLLOW_ROUTE_CELL_WORDS]) -> Option<Self> {
        let [gain, ll, lr, rl, rr, flags, length] = words;
        Self::new(
            RouteTransform {
                gain: f32::from_bits(gain),
                ll: f32::from_bits(ll),
                lr: f32::from_bits(lr),
                rl: f32::from_bits(rl),
                rr: f32::from_bits(rr),
            },
            RouteFollowFlags::from_word(flags)?,
            length,
        )
    }
    // REALTIME_POLICY_END
}

/// What one following route shares between its writer (control) and its reader (render): one
/// seven-word latest-target cell and its dirty word (#1652 D1). Its own type, so a six-word
/// route's [`RouteLane`] keeps its bytes.
pub(crate) struct FollowRouteLane {
    pub(crate) cell: engine::realtime::LatestCell<FOLLOW_ROUTE_CELL_WORDS>,
    pub(crate) dirty: engine::realtime::DirtyWord,
}

/// The render-side half of one following route's cell (#1652 D1): the shared cell, the reader's
/// last applied sequence, the dirty reader and the session's `live_values_superseded` count. The
/// counterpart of [`RouteControlLane`] for the seven-word cell.
pub struct FollowRouteControlLane {
    pub(crate) lane: std::sync::Arc<FollowRouteLane>,
    pub(crate) reader: engine::realtime::CellReader,
    pub(crate) dirty: engine::realtime::DirtyReader,
    pub(crate) superseded: engine::realtime::LiveValuesSuperseded,
}

// REALTIME_POLICY_BEGIN
impl FollowRouteControlLane {
    /// Read the cell under the block's snapshot: the newest record whose revision is `<=
    /// snapshot` and that this reader has not applied, or `None`. An idle cell costs one
    /// `Relaxed` load. When it applies sequence `s` after `p`, the `s - p - 1` writes it replaced
    /// join the session's `live_values_superseded` (#1312 D2). Allocation-free and lock-free.
    pub(crate) fn read(&mut self, snapshot: LiveSnapshot) -> Option<FollowRouteControlRecord> {
        if self.dirty.pending(&self.lane.dirty) & 1 == 0 {
            return None;
        }
        let engine::realtime::CellRead::Applied { words, .. } = self.reader.read_counted(
            &self.lane.cell,
            snapshot,
            &mut self.dirty,
            0,
            &self.superseded,
        ) else {
            return None;
        };
        // Only `write` fills the cell, from a validated record, so the words decode.
        let record = FollowRouteControlRecord::from_words(words);
        debug_assert!(
            record.is_some(),
            "a following route cell held words no record wrote"
        );
        record
    }
}
// REALTIME_POLICY_END

/// Which jump length one cursor of a following route takes (#1652b, #1655 D4): the mute rule for
/// a source lane's own mute, `VcaMute` for a reaching VCA's mute.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphRouteCursorRow {
    /// A source lane's stored mute (#1652).
    SourceMute,
    /// The stored mute of an automated VCA that reaches the source lane (#1655 D4).
    VcaMute,
}

/// The jump lengths, in samples, of a following route's cursor rows for one block, resolved from
/// the block's three session lengths by [`GraphRouteJumpRule`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphRouteJumpLengths {
    /// The mute row's length: a source lane's own mute.
    pub mute: u32,
    /// The `VcaMute` row's length: a reaching VCA's mute curve (#1655 D4).
    pub vca_mute: u32,
}

impl GraphRouteJumpLengths {
    // REALTIME_POLICY_BEGIN
    /// The jump length of a cursor of `row`.
    #[must_use]
    pub const fn for_row(self, row: GraphRouteCursorRow) -> u32 {
        match row {
            GraphRouteCursorRow::SourceMute => self.mute,
            GraphRouteCursorRow::VcaMute => self.vca_mute,
        }
    }
    // REALTIME_POLICY_END
}

/// The jump lengths of a following route's cursor rows, resolved from the block's three session
/// lengths (#1652b, #1655 D4). Host-core hands in the row rules
/// (`controls::LiveRamps::from_lengths` and `for_row` of `LiveRampRow::Mute` and `VcaMute`, the
/// rules the strip's stages use), so graph never spells a row and the strip ramp and the send ramp
/// cannot differ in length.
pub type GraphRouteJumpRule = fn(GraphJumpLengths) -> GraphRouteJumpLengths;

/// What one following route reads of its stored automation (#1652 D2, #1652b, #1655 D4): the
/// plan's immutable automation program, the source lanes' mute cells in it, the cells of the
/// automated VCA mutes that reach each source lane, the arrival `a(tap)` of the route's tap, the
/// cursor rows' jump-length rule and the seeded flags word.
///
/// The program is the one the strip's own stage reads (an `Arc` clone, no copy); the route keeps
/// its own cursors over it, so two readers of one table never share state. `a(tap)` is the route
/// node's floored input arrival (`floored_input_arrival` of the compile report's
/// `input_arrivals`): a route op reads its one undelayed input before its consumer's compensation
/// delay. Only [`Self::new`] builds one, so every cell it names is in its program.
#[derive(Clone)]
pub struct GraphFollowRouteSource {
    pub(crate) program: std::sync::Arc<automation::Program>,
    pub(crate) cells: [Option<u32>; 2],
    /// Per source lane, the mute cells of the automated VCAs that reach it, in ascending VCA ID
    /// (#1655 D4): the route ORs them into the lane's follow term at `a(tap)`.
    pub(crate) vca_mutes: [Box<[u32]>; 2],
    pub(crate) arrival: u64,
    pub(crate) jumps: GraphRouteJumpRule,
    pub(crate) flags: RouteFollowFlags,
    /// Per source lane, whether the predecessor plan holds the same VCA mute tables for it, in
    /// the same order ([`Self::with_continued_vca_mutes`]).
    pub(crate) continued_vca: [bool; 2],
}

impl GraphFollowRouteSource {
    /// The source of a following route: `cells[lane]` is the source lane's mute cell in
    /// `program`, `None` for a lane whose own mute the plan does not automate; `vca_mutes[lane]`
    /// are the mute cells in `program` of the automated VCAs that reach the lane, in ascending VCA
    /// ID (#1655 D4, empty when none does); `arrival` is `a(tap)`; `jumps` resolves the cursor
    /// rows' jump lengths; `flags` is the seeded flags word.
    ///
    /// `None` when no source lane is automated (the route keeps the six-word cell), when a cell is
    /// not in `program`, or when `flags.automated` disagrees with the lanes that have a curve (own
    /// cell or VCA mute cells; the plan fixes which lanes are automated, D1).
    #[must_use]
    pub fn new(
        program: std::sync::Arc<automation::Program>,
        cells: [Option<u32>; 2],
        vca_mutes: [Vec<u32>; 2],
        arrival: u64,
        jumps: GraphRouteJumpRule,
        flags: RouteFollowFlags,
    ) -> Option<Self> {
        let automated = [0, 1].map(|lane| {
            cells.get(lane).copied().flatten().is_some()
                || vca_mutes.get(lane).is_some_and(|list| !list.is_empty())
        });
        let in_program = cells
            .iter()
            .flatten()
            .chain(vca_mutes.iter().flatten())
            .all(|&cell| program.cell(cell as usize).is_some());
        (automated.contains(&true) && in_program && flags.automated == automated).then(|| Self {
            program,
            cells,
            vca_mutes: vca_mutes.map(Vec::into_boxed_slice),
            arrival,
            jumps,
            flags,
            continued_vca: [false; 2],
        })
    }

    /// This source with `lanes[lane]` set for each source lane whose VCA mute tables the
    /// predecessor plan holds unchanged, the same VCAs in the same order (#1655 D6,
    /// `controls::continued_mute_lanes`): at a plan swap that lane's VCA mute cursors carry their
    /// state and heard values verbatim, where otherwise they are placed again at the successor's
    /// node time. By default no lane continues.
    #[must_use]
    pub const fn with_continued_vca_mutes(mut self, lanes: [bool; 2]) -> Self {
        self.continued_vca = lanes;
        self
    }

    /// `a(tap)`, the arrival the route's events are timed at.
    #[must_use]
    pub const fn arrival(&self) -> u64 {
        self.arrival
    }

    /// The seeded flags word.
    #[must_use]
    pub const fn flags(&self) -> RouteFollowFlags {
        self.flags
    }
}

/// One following route's lane, carried beside the prepared routes as
/// [`GraphRouteControlBinding`] is (#1652 D1), with what its route op reads of the source strip's
/// stored mute (#1652b).
pub struct GraphFollowRouteControlBinding {
    /// The route node this lane drives.
    pub node: GraphNodeId,
    /// Reader half of the seven-word cell; the writer stays on the control plane.
    pub control: Box<FollowRouteControlLane>,
    /// The source strip's mute cells, the reaching VCA mute cells, `a(tap)`, the jump rule and
    /// the seeded flags.
    pub source: GraphFollowRouteSource,
    /// The route op's cursor tables, allocated at attach so a failed allocation is a typed
    /// refusal ([`GraphRouteControlError::CursorTable`]); bind moves them into the op.
    pub(crate) columns: runtime::FollowColumns,
}

/// The control-plane half of one following route's seven-word cell (#1652 D1): the counterpart of
/// [`GraphRouteControlProducer`].
pub struct GraphFollowRouteControlProducer {
    /// The route's session ID.
    pub route_id: Box<str>,
    lane: std::sync::Arc<FollowRouteLane>,
    writer: engine::realtime::CellWriter<FOLLOW_ROUTE_CELL_WORDS>,
    /// As [`GraphRouteControlProducer`]'s: the route-activity bytes this attach added, the same
    /// for every producer of one attach.
    activity_bytes: u64,
    /// The largest single allocation of that table, or `0` with it.
    activity_largest: u64,
    /// The route-activity table's activation offsets (#1652 MAJOR-1): one `u32` per prepared
    /// route, which bind allocates only for a plan with a following route, so the compile-time
    /// estimate never charges it. The same for every producer of one attach.
    activity_offset_bytes: u64,
    /// The lanes the plan automates (the bound source's `flags.automated`, #1652 D1): render
    /// takes `automated` from its own cursors, so a record whose bits differ is refused by the
    /// commit before any write (#1653 D5).
    automated: [bool; 2],
    /// The bytes of the route op's cursor tables bind allocates (#1654 T3, R19-3: per column with
    /// a curve, its merged walk's table and its probe twin's), and the largest one.
    cursor_bytes: u64,
    cursor_largest: u64,
}

impl GraphFollowRouteControlProducer {
    /// The source lanes whose mute the plan automates, as bound (#1653 D5): the bits every record
    /// written to this cell must carry.
    #[must_use]
    pub const fn automated(&self) -> [bool; 2] {
        self.automated
    }

    /// Write `record` as the route's newest value, stamped with the revision of the transaction
    /// that commits it. A write cannot fail and never waits; render applies the newest and counts
    /// each replaced one. Validation is [`FollowRouteControlRecord::new`]'s, made before any
    /// write. The caller publishes the stamp's revision on the plan's gate after every write of
    /// the transaction (#1312 D7).
    pub fn write(
        &mut self,
        stamp: engine::realtime::LiveStamp<'_>,
        record: FollowRouteControlRecord,
    ) {
        self.writer.write(
            &self.lane.cell,
            &self.lane.dirty,
            0,
            stamp,
            record.to_words(),
        );
    }
}

#[cfg(any(test, feature = "test-support"))]
impl GraphFollowRouteControlProducer {
    /// Test-only: how many writes the cell has taken. Valid while no write is in flight, which a
    /// quiescent test thread guarantees.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_count(&self) -> u64 {
        engine::realtime::CellReader::new()
            .peek_unread(&self.lane.cell)
            .map_or(0, |(_, sequence)| sequence)
    }
}

/// One following route's binding and producer over one fresh seven-word cell (#1652 D1), reading
/// `source` (#1652b). `activity_bytes` and `activity_largest` are the attach's route-activity charge, as for a
/// six-word producer; `activity_routes` is the plan's prepared route count, whose activation
/// offsets bind allocates for a plan with a following route (#1652 MAJOR-1).
pub(crate) fn follow_route_control_pair(
    route_id: &StableGraphId,
    superseded: &engine::realtime::LiveValuesSuperseded,
    source: GraphFollowRouteSource,
    activity_bytes: u64,
    activity_largest: u64,
    activity_routes: u64,
) -> Result<
    (
        GraphFollowRouteControlBinding,
        GraphFollowRouteControlProducer,
    ),
    automation::ComposeError,
> {
    let columns = runtime::FollowColumns::bind(&source)?;
    let lane = std::sync::Arc::new(FollowRouteLane {
        cell: engine::realtime::LatestCell::new(),
        dirty: engine::realtime::DirtyWord::new(),
    });
    let automated = source.flags().automated;
    let (cursor_bytes, cursor_largest) = runtime::follow_route_cursor_bytes(&source);
    Ok((
        GraphFollowRouteControlBinding {
            node: GraphNodeId::Route {
                route_id: route_id.clone(),
            },
            control: Box::new(FollowRouteControlLane {
                lane: std::sync::Arc::clone(&lane),
                reader: engine::realtime::CellReader::new(),
                dirty: engine::realtime::DirtyReader::new(),
                superseded: superseded.clone(),
            }),
            source,
            columns,
        },
        GraphFollowRouteControlProducer {
            route_id: route_id.as_str().into(),
            lane,
            writer: engine::realtime::CellWriter::new(),
            activity_bytes,
            activity_largest,
            activity_offset_bytes: runtime::route_activity_offset_bytes(activity_routes)
                .unwrap_or(u64::MAX),
            automated,
            cursor_bytes,
            cursor_largest,
        },
    ))
}

/// Why [`PreparedGraphPlan::attach_route_controls`] refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphRouteControlError {
    /// Route controls are already attached to this plan.
    AlreadyAttached,
    /// A route into a submix is also a node the plan requires a processor for; a live route is
    /// never a [`GraphNodeBinding`] (its op would not mix).
    BoundRoute,
    /// A following route's cursor tables were refused (#1654 T3: an allocation failure; a
    /// source's cells are always its program's).
    CursorTable(automation::ComposeError),
    /// A transition route (#1363 D2) is not a route the plan prepares.
    UnknownTransitionRoute,
}

/// The transition routes one [`PreparedGraphPlan::attach_route_controls_following`] call bound
/// (#1363 D2): routes a plan swap adds that bind live with no lane, so a fade arm can ramp them.
/// What they add to the plan is [`RouteControlResources::with_transition_routes`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphTransitionRoutes {
    /// How many routes bind as transition routes.
    pub routes: u64,
    /// Their route IDs' bytes, once (the charge counts them twice: the binding's node ID, held
    /// until bind, and the bind row's copy).
    pub route_id_bytes: u64,
    /// The route-activity table bind builds for them when no lane-ful route charges it: `0` when
    /// the call returned a producer, or a prepared gate silences (the compile charged it then).
    pub activity_bytes: u64,
    /// The largest single allocation of that table; `0` as `activity_bytes` is.
    pub activity_largest: u64,
}

/// The bytes bind allocates for one live route's render-side owner, its boxed state (issue #1220
/// D8): stated so a caller can check [`route_control_resources`] from first principles.
pub const LIVE_ROUTE_OWNER_BYTES: usize = runtime::live_route_owner_bytes();

/// The bytes bind allocates for one following route's render-side owner, its boxed state (#1652
/// D1): stated so a caller can check [`route_control_resources`] from first principles.
pub const LIVE_FOLLOW_ROUTE_OWNER_BYTES: usize = runtime::live_follow_route_owner_bytes();

/// What attaching route controls and binding the plan add (issue #1220 D8): an upper bound on
/// what the attached artifact retains before bind and on what the bound plan retains.
///
/// Every field is in bytes except `routes`. `total_bytes` is the sum of the five byte fields and
/// `largest_allocation_bytes` the largest single allocation among them. Every field but
/// `activity_bytes` is the exact size of what it names; `activity_bytes` is
/// `route_activity_bound_bytes`, an upper bound.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RouteControlResources {
    /// Live routes: one cell, one lane, one binding, one owner and one producer each.
    pub routes: u64,
    /// Each route's one shared allocation, the `Arc` of its latest-target cell and dirty word
    /// (`SharedAllocation<RouteLane>`: the cell's three slots, the dirty word and the owner
    /// header; `SharedAllocation<FollowRouteLane>` for a following route). Exact (#1347 D1,
    /// #1652 D1).
    pub cell_bytes: u64,
    /// Per route: the boxed [`RouteControlLane`], the plan's [`GraphRouteControlBinding`] entry
    /// (held until bind) and the boxed render-side owner bind builds; for a following route the
    /// boxed [`FollowRouteControlLane`], its [`GraphFollowRouteControlBinding`], its owner and
    /// its route op's cursor tables (#1654 T3: per column with a curve, two tables of one cursor
    /// per curve).
    pub owner_bytes: u64,
    /// The producer tables: one [`GraphRouteControlProducer`] per six-word route and one
    /// [`GraphFollowRouteControlProducer`] per following route.
    pub producer_table_bytes: u64,
    /// The route IDs' bytes, twice: the producer's `route_id`, and the plan's
    /// [`GraphRouteControlBinding`] node ID (held until bind) -- a second heap copy (#1220 verdict
    /// MINOR-1).
    pub route_id_bytes: u64,
    /// The route-activity table, with its per-unit route indices, when the compile-time estimate
    /// did not already charge it (no prepared route's gate silences); `0` otherwise.
    pub activity_bytes: u64,
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
}

impl RouteControlResources {
    /// These resources with the transition routes of the same attach added (#1363 D2): each
    /// route's binding entry (held until bind), boxed render-side owner and bind row
    /// ([`GraphLiveRoute`], held until taken), its route ID's bytes twice (the binding's node ID
    /// and the row's copy), and the route-activity table when no producer charged it. Saturating.
    #[must_use]
    pub fn with_transition_routes(self, transition: &GraphTransitionRoutes) -> Self {
        if transition.routes == 0 {
            return self;
        }
        let count = |value: usize| u64::try_from(value).unwrap_or(u64::MAX);
        let binding = count(core::mem::size_of::<GraphRouteControlBinding>());
        let owner = count(LIVE_ROUTE_OWNER_BYTES);
        let row = count(core::mem::size_of::<GraphLiveRoute>());
        let owner_bytes = transition
            .routes
            .saturating_mul(binding.saturating_add(owner).saturating_add(row));
        let id_bytes = transition.route_id_bytes.saturating_mul(2);
        let added = owner_bytes
            .saturating_add(id_bytes)
            .saturating_add(transition.activity_bytes);
        Self {
            routes: self.routes.saturating_add(transition.routes),
            owner_bytes: self.owner_bytes.saturating_add(owner_bytes),
            route_id_bytes: self.route_id_bytes.saturating_add(id_bytes),
            activity_bytes: self
                .activity_bytes
                .saturating_add(transition.activity_bytes),
            total_bytes: self.total_bytes.saturating_add(added),
            largest_allocation_bytes: self
                .largest_allocation_bytes
                .max(owner)
                .max(transition.routes.saturating_mul(binding))
                .max(transition.routes.saturating_mul(row))
                .max(transition.activity_largest),
            ..self
        }
    }
}

/// The resources of one [`PreparedGraphPlan::attach_route_controls`] call's `producers` and
/// `following` producers, and of the bind that consumes their lanes (issue #1220 D8, #1652 D1): an
/// upper bound on both the attached and the bound state ([`RouteControlResources`]). A six-word
/// route is charged its `RouteLane` cell, its owner ([`LIVE_ROUTE_OWNER_BYTES`]) and its
/// producer; a following route its own seven-word cell, owner ([`LIVE_FOLLOW_ROUTE_OWNER_BYTES`])
/// and producer, so the charge of a plan with no following route is what it was. The
/// route-activity table is one per plan, charged once. Saturating; empty for no producers.
#[must_use]
pub fn route_control_resources(
    producers: &[GraphRouteControlProducer],
    following: &[GraphFollowRouteControlProducer],
) -> RouteControlResources {
    let producers: Vec<RouteLaneFacts> = producers
        .iter()
        .map(|producer| RouteLaneFacts {
            route_id_bytes: producer.route_id.len(),
            activity_bytes: producer.activity_bytes,
            activity_largest: producer.activity_largest,
            cursor_bytes: 0,
            cursor_largest: 0,
            activity_offset_bytes: 0,
        })
        .collect();
    let following: Vec<RouteLaneFacts> = following
        .iter()
        .map(|producer| RouteLaneFacts {
            route_id_bytes: producer.route_id.len(),
            activity_bytes: producer.activity_bytes,
            activity_largest: producer.activity_largest,
            cursor_bytes: producer.cursor_bytes,
            cursor_largest: producer.cursor_largest,
            activity_offset_bytes: producer.activity_offset_bytes,
        })
        .collect();
    route_control_resources_of(&producers, &following)
}

/// What one route lane's producer contributes to [`route_control_resources`]: its ID's length,
/// the attach's route-activity charge and, for a following route, its cursor tables and
/// activation offsets.
#[derive(Clone, Copy, Debug)]
struct RouteLaneFacts {
    route_id_bytes: usize,
    activity_bytes: u64,
    activity_largest: u64,
    cursor_bytes: u64,
    cursor_largest: u64,
    activity_offset_bytes: u64,
}

/// [`route_control_resources`] over each lane's [`RouteLaneFacts`]: the one composition, which
/// [`route_control_resources_bound`] runs before any lane exists.
fn route_control_resources_of(
    producers: &[RouteLaneFacts],
    following: &[RouteLaneFacts],
) -> RouteControlResources {
    let count = |value: usize| u64::try_from(value).unwrap_or(u64::MAX);
    if producers.is_empty() && following.is_empty() {
        return RouteControlResources::default();
    }
    let six = count(producers.len());
    let seven = count(following.len());
    let routes = six.saturating_add(seven);
    // Every route's shared allocation of one kind is the same type, so the same bytes.
    let cell = count(core::mem::size_of::<
        engine::realtime::SharedAllocation<RouteLane>,
    >());
    let follow_cell = count(core::mem::size_of::<
        engine::realtime::SharedAllocation<FollowRouteLane>,
    >());
    let cell_bytes = six
        .saturating_mul(cell)
        .saturating_add(seven.saturating_mul(follow_cell));
    let mut route_id_bytes = 0_u64;
    let mut largest = 0_u64;
    if six > 0 {
        largest = largest.max(cell);
    }
    if seven > 0 {
        largest = largest.max(follow_cell);
    }
    let route_ids = producers
        .iter()
        .map(|producer| producer.route_id_bytes)
        .chain(following.iter().map(|producer| producer.route_id_bytes));
    for route_id in route_ids {
        // The producer's copy and the binding's node-ID copy.
        route_id_bytes = route_id_bytes.saturating_add(count(route_id).saturating_mul(2));
        largest = largest.max(count(route_id));
    }
    let lane = count(core::mem::size_of::<RouteControlLane>());
    let binding = count(core::mem::size_of::<GraphRouteControlBinding>());
    let owner = count(LIVE_ROUTE_OWNER_BYTES);
    let follow_lane = count(core::mem::size_of::<FollowRouteControlLane>());
    let follow_binding = count(core::mem::size_of::<GraphFollowRouteControlBinding>());
    let follow_owner = count(LIVE_FOLLOW_ROUTE_OWNER_BYTES);
    // #1654 T3: each following route's cursor tables, sized by its own curves.
    let cursor_bytes = following.iter().fold(0_u64, |sum, producer| {
        sum.saturating_add(producer.cursor_bytes)
    });
    let owner_bytes = six
        .saturating_mul(lane.saturating_add(binding).saturating_add(owner))
        .saturating_add(
            seven.saturating_mul(
                follow_lane
                    .saturating_add(follow_binding)
                    .saturating_add(follow_owner),
            ),
        )
        .saturating_add(cursor_bytes);
    for producer in following {
        largest = largest.max(producer.cursor_largest);
    }
    let six_producers =
        six.saturating_mul(count(core::mem::size_of::<GraphRouteControlProducer>()));
    let seven_producers = seven.saturating_mul(count(core::mem::size_of::<
        GraphFollowRouteControlProducer,
    >()));
    let producer_table_bytes = six_producers.saturating_add(seven_producers);
    let activity = producers
        .iter()
        .map(|producer| (producer.activity_bytes, producer.activity_largest))
        .chain(
            following
                .iter()
                .map(|producer| (producer.activity_bytes, producer.activity_largest)),
        );
    let (activity_bytes, activity_largest) = activity.fold((0, 0), |(bytes, largest), next| {
        (bytes.max(next.0), largest.max(next.1))
    });
    // #1652 MAJOR-1: the activation offsets bind adds to the table for a plan with a following
    // route; charged once, with the table.
    let offsets = following
        .iter()
        .map(|producer| producer.activity_offset_bytes)
        .max()
        .unwrap_or(0);
    let activity_bytes = activity_bytes.saturating_add(offsets);
    let activity_largest = activity_largest.max(offsets);
    if six > 0 {
        largest = largest
            .max(lane)
            .max(owner)
            .max(six.saturating_mul(binding))
            .max(six_producers);
    }
    if seven > 0 {
        largest = largest
            .max(follow_lane)
            .max(follow_owner)
            .max(seven.saturating_mul(follow_binding))
            .max(seven_producers);
    }
    largest = largest.max(activity_largest);
    RouteControlResources {
        routes,
        cell_bytes,
        owner_bytes,
        producer_table_bytes,
        route_id_bytes,
        activity_bytes,
        total_bytes: cell_bytes
            .saturating_add(owner_bytes)
            .saturating_add(producer_table_bytes)
            .saturating_add(route_id_bytes)
            .saturating_add(activity_bytes),
        largest_allocation_bytes: largest,
    }
}

/// One following route as [`route_control_resources_bound`] reads it: its ID, its source's mute
/// cells and the VCA mute cells reaching each source lane.
pub type FollowingRouteBound<'a> = (&'a str, [Option<u32>; 2], [&'a [u32]; 2]);

/// An upper bound on the [`route_control_resources`] of the lanes
/// [`PreparedGraphPlan::attach_route_controls_following`] attaches, with its transition routes
/// ([`RouteControlResources::with_transition_routes`]), before the plan is compiled (issue #2199).
///
/// `six_word` are the IDs of the routes that get a six-word lane, `following` each following
/// route's ID with its source's mute cells and VCA mute cells (the arguments of
/// [`GraphFollowRouteSource::new`]), `transition` the transition routes' IDs. `prepared_routes`
/// and `nodes` are the plan's prepared routes and graph nodes, and `gate_silences` whether a
/// prepared route's gate silences (the compile then charges the activity table). The cursor
/// tables and the activity table are the attach's own; a route the compile's attach gives no lane
/// is charged as one that gets it, so the bound holds whichever lanes attach.
#[must_use]
pub fn route_control_resources_bound(
    six_word: &[&str],
    following: &[FollowingRouteBound<'_>],
    transition: &[&str],
    prepared_routes: u64,
    nodes: u64,
    gate_silences: bool,
) -> RouteControlResources {
    let (activity_bytes, activity_largest) = if gate_silences
        || (six_word.is_empty() && following.is_empty() && transition.is_empty())
    {
        (0, 0)
    } else {
        (
            runtime::route_activity_bound_bytes(prepared_routes, nodes).unwrap_or(u64::MAX),
            runtime::route_activity_largest_bytes(prepared_routes, nodes).unwrap_or(u64::MAX),
        )
    };
    let six: Vec<RouteLaneFacts> = six_word
        .iter()
        .map(|route_id| RouteLaneFacts {
            route_id_bytes: route_id.len(),
            activity_bytes,
            activity_largest,
            cursor_bytes: 0,
            cursor_largest: 0,
            activity_offset_bytes: 0,
        })
        .collect();
    let seven: Vec<RouteLaneFacts> = following
        .iter()
        .map(|(route_id, cells, vca_mutes)| {
            let (cursor_bytes, cursor_largest) =
                runtime::follow_route_cursor_bytes_of(cells, *vca_mutes);
            RouteLaneFacts {
                route_id_bytes: route_id.len(),
                activity_bytes,
                activity_largest,
                cursor_bytes,
                cursor_largest,
                activity_offset_bytes: runtime::route_activity_offset_bytes(prepared_routes)
                    .unwrap_or(u64::MAX),
            }
        })
        .collect();
    let lanes = !six.is_empty() || !seven.is_empty();
    route_control_resources_of(&six, &seven).with_transition_routes(&GraphTransitionRoutes {
        routes: u64::try_from(transition.len()).unwrap_or(u64::MAX),
        route_id_bytes: transition.iter().fold(0_u64, |sum, route_id| {
            sum.saturating_add(u64::try_from(route_id.len()).unwrap_or(u64::MAX))
        }),
        activity_bytes: if lanes { 0 } else { activity_bytes },
        activity_largest: if lanes { 0 } else { activity_largest },
    })
}
/// Words of the plan's jump-length cell (#1639 D1): the session's `fader_ms`, `mute_ms` and
/// `pan_ms` lengths in samples, in that order.
const JUMP_LENGTH_WORDS: usize = 3;

/// The session's three live ramp lengths, in samples, that the plan's jump-length cell holds
/// (#1639 D1): the values of `controls::LiveRamps` for the `fader_ms`, `mute_ms` and `pan_ms` keys
/// (`SessionModel::control_smoothing_samples`). A stage resolves one row's length from them with
/// `LiveRamps::for_row`, which owns every row rule (the polarity doubling among them); the graph
/// holds the three values and knows no row.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphJumpLengths {
    /// The `fader_ms` key, in samples.
    pub fader_samples: u32,
    /// The `mute_ms` key, in samples.
    pub mute_samples: u32,
    /// The `pan_ms` key, in samples.
    pub pan_samples: u32,
}

impl GraphJumpLengths {
    const fn to_words(self) -> [u32; JUMP_LENGTH_WORDS] {
        [self.fader_samples, self.mute_samples, self.pan_samples]
    }

    // REALTIME_POLICY_BEGIN
    const fn from_words(words: [u32; JUMP_LENGTH_WORDS]) -> Self {
        let [fader_samples, mute_samples, pan_samples] = words;
        Self {
            fader_samples,
            mute_samples,
            pan_samples,
        }
    }
    // REALTIME_POLICY_END
}

/// The node-time mapping of one node block (#1640 D3): the timeline value at the block's first
/// sample, and the seek that reaches the block, if any. The source crate's timeline reader returns
/// it, and graph hands it to a builtin bank through [`GraphBlockTime::node_span`] (#1647a).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimelineSpan {
    /// The timeline value at the node block's first sample.
    pub start: i64,
    /// The seek that reaches the node block, if any.
    pub step: Option<TimelineStep>,
}

/// A seek inside a node block: from sample `offset` of the block on, the timeline reads
/// `timeline + (i - offset)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimelineStep {
    /// The node block's sample the seek takes effect at, from `0` to the block length minus one.
    pub offset: u32,
    /// The timeline value at that sample.
    pub timeline: i64,
}

/// One graph block's node-time context (#1647a, R17-3): what a builtin bank needs to map its
/// lanes' samples to session timeline samples for the block. The graph builds it once per block,
/// after the source set's block entry, and sets it on every builtin slot of a bank unit through
/// [`GraphPreparedBuiltinBankProcessor::set_block_time`] before the unit's chain runs. It borrows
/// the source set's driver for the block; a stage reads what it needs during that call and keeps
/// no reference.
#[derive(Clone, Copy)]
pub struct GraphBlockTime<'a> {
    source_read_sample: Option<u64>,
    frames: u32,
    jump_lengths: Option<GraphJumpLengths>,
    timeline: Option<&'a dyn GraphPreparedSourceSetDriver>,
}

// REALTIME_POLICY_BEGIN
impl<'a> GraphBlockTime<'a> {
    /// The context of a block of `frames` samples whose first sample reads source time from
    /// `source_read_sample` (`r + ΣP`, #1396; `None` when that sum leaves the `u64` range, which a
    /// plan with a source set refuses as `TimeOverflow` first), holding `jump_lengths` (#1639 D2),
    /// with `timeline` the source set's driver or `None` in a plan with no source set.
    #[must_use]
    pub const fn new(
        source_read_sample: Option<u64>,
        frames: u32,
        jump_lengths: Option<GraphJumpLengths>,
        timeline: Option<&'a dyn GraphPreparedSourceSetDriver>,
    ) -> Self {
        Self {
            source_read_sample,
            frames,
            jump_lengths,
            timeline,
        }
    }

    /// The source-read sample `r + ΣP` of the block's first sample.
    #[must_use]
    pub const fn source_read_sample(&self) -> Option<u64> {
        self.source_read_sample
    }

    /// The block length.
    #[must_use]
    pub const fn frames(&self) -> u32 {
        self.frames
    }

    /// The three jump lengths the graph holds for the block (#1639 D2), or `None` for a plan with
    /// no jump-length cell.
    #[must_use]
    pub const fn jump_lengths(&self) -> Option<GraphJumpLengths> {
        self.jump_lengths
    }

    /// The node-time mapping of this block for a node of input arrival `arrival` (`a(n)`, the
    /// floored arrival of #1285): the timeline read from source-read sample `r + ΣP - a(n)`,
    /// which is negative while `r + ΣP < a(n)` (R17-5). The difference is exact in `i128`. `None`
    /// in a plan with no timeline or when the block's source-read sample is unknown.
    #[must_use]
    pub fn node_span(&self, arrival: u64) -> Option<TimelineSpan> {
        let first = i128::from(self.source_read_sample?) - i128::from(arrival);
        self.timeline?.timeline_block(first, self.frames)
    }
}
// REALTIME_POLICY_END

/// Node `node`'s floored input arrival `a(n)` in `arrivals`, `GraphCompileReport::input_arrivals`
/// (#1285 D2: every node with a nonzero floored arrival, sorted by node ID): its row, or 0 for a
/// node absent from it. The one lookup both fader owners' installs take (#1647a, #1647b): a
/// builtin bank's members and the scalar oracle's bound fader nodes.
#[must_use]
pub fn floored_input_arrival(
    arrivals: &[(GraphNodeId, LatencySamples)],
    node: &GraphNodeId,
) -> u64 {
    arrivals
        .binary_search_by(|(row, _)| row.cmp(node))
        .ok()
        .and_then(|row| arrivals.get(row))
        .map_or(0, |(_, arrival)| arrival.0)
}

/// What the plan's jump-length cell shares between its writer (control) and its reader (render):
/// one #1312 latest-target cell and its dirty word (#1639 D1).
struct JumpLengthLane {
    cell: engine::realtime::LatestCell<JUMP_LENGTH_WORDS>,
    dirty: engine::realtime::DirtyWord,
}

/// The render half of the plan's jump-length cell (#1639 D2): the one reader. It is not `Clone`
/// and only [`attach_jump_lengths`] builds it, straight into the executor, so a second reader of
/// the cell cannot be written.
struct JumpLengthReader {
    lane: std::sync::Arc<JumpLengthLane>,
    reader: engine::realtime::CellReader,
    dirty: engine::realtime::DirtyReader,
    superseded: engine::realtime::LiveValuesSuperseded,
    /// The lengths this block holds: the prepared ones until a write is applied.
    held: GraphJumpLengths,
}

// REALTIME_POLICY_BEGIN
impl JumpLengthReader {
    /// Render, once per block at block entry, after the block's live snapshot: apply the newest
    /// write at or below `snapshot`, if it was not applied before (#1639 D2). An idle cell costs
    /// one `Relaxed` load. A write a later one replaced before this read joins the session's
    /// `live_values_superseded` count (D15-2).
    fn begin_block(&mut self, snapshot: LiveSnapshot) {
        if self.dirty.pending(&self.lane.dirty) == 0 {
            return;
        }
        if let engine::realtime::CellRead::Applied { words, .. } = self.reader.read_counted(
            &self.lane.cell,
            snapshot,
            &mut self.dirty,
            0,
            &self.superseded,
        ) {
            self.held = GraphJumpLengths::from_words(words);
        }
    }
}
// REALTIME_POLICY_END

/// The control half of a plan's jump-length cell (#1639 D1, D3): the cell's one writer.
pub struct GraphJumpLengthProducer {
    lane: std::sync::Arc<JumpLengthLane>,
    writer: engine::realtime::CellWriter<JUMP_LENGTH_WORDS>,
}

impl GraphJumpLengthProducer {
    /// Write `lengths` as the plan's newest jump lengths, stamped with the revision of the
    /// transaction that commits them. It cannot fail and never waits; the caller publishes the
    /// stamp's revision on the plan's gate after every write of the transaction, and render holds
    /// the lengths from the first block whose snapshot covers it (#1312 D7).
    pub fn write(&mut self, stamp: engine::realtime::LiveStamp<'_>, lengths: GraphJumpLengths) {
        self.writer.write(
            &self.lane.cell,
            &self.lane.dirty,
            0,
            stamp,
            lengths.to_words(),
        );
    }
}

#[cfg(any(test, feature = "test-support"))]
impl GraphJumpLengthProducer {
    /// Test-only: how many writes the cell has taken. Valid while no write is in flight, which a
    /// quiescent test thread guarantees.
    #[doc(hidden)]
    #[must_use]
    pub fn test_only_write_count(&self) -> u64 {
        engine::realtime::CellReader::new()
            .peek_unread(&self.lane.cell)
            .map_or(0, |(_, sequence)| sequence)
    }
}

/// Why [`attach_automation_program`] refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphAutomationProgramError {
    /// The plan is not a graph plan.
    NotAGraphPlan,
    /// The plan holds its program already.
    AlreadyAttached,
}

/// Give a bound graph plan a handle on the stored-automation program its stages and routes
/// read (#1074), so [`PreparedRenderPlan::retained_bytes`] charges that one shared allocation
/// once, at the plan: no per-stage or per-route clone charges it. Off the render thread, before
/// publication; render never reads the handle.
///
/// # Errors
///
/// [`GraphAutomationProgramError::NotAGraphPlan`] for a plan another executor renders, and
/// [`GraphAutomationProgramError::AlreadyAttached`] for a second call on one plan.
pub fn attach_automation_program(
    plan: &mut PreparedRenderPlan,
    program: std::sync::Arc<automation::Program>,
) -> Result<(), GraphAutomationProgramError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphAutomationProgramError::NotAGraphPlan)?;
    if executor.automation_program.is_some() {
        return Err(GraphAutomationProgramError::AlreadyAttached);
    }
    executor.automation_program = Some(program);
    Ok(())
}

/// Why [`attach_jump_lengths`] refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphJumpLengthError {
    /// The plan is not a graph plan.
    NotAGraphPlan,
    /// The plan has its jump-length cell already.
    AlreadyAttached,
}

/// The bytes one plan's jump-length cell retains (#1639 D1): the shared allocation of the cell
/// and its dirty word (`SharedAllocation<JumpLengthLane>`) and the boxed render-side reader. The
/// same for every session. The producer is held inline by its owner.
#[must_use]
pub fn jump_length_retained_bytes() -> u64 {
    let shared = core::mem::size_of::<engine::realtime::SharedAllocation<JumpLengthLane>>();
    let reader = core::mem::size_of::<JumpLengthReader>();
    u64::try_from(shared.saturating_add(reader)).unwrap_or(u64::MAX)
}

/// The largest single allocation of [`jump_length_retained_bytes`].
#[must_use]
pub fn jump_length_largest_allocation_bytes() -> u64 {
    let shared = core::mem::size_of::<engine::realtime::SharedAllocation<JumpLengthLane>>();
    let reader = core::mem::size_of::<JumpLengthReader>();
    u64::try_from(shared.max(reader)).unwrap_or(u64::MAX)
}

/// Give a bound graph plan its jump-length cell (#1639 D1), holding `lengths` from its first
/// block, and return the cell's one writer. Every write a later one replaces before render reads
/// it is added to `superseded`, the session's count. Off the render thread, before publication.
///
/// The graph reads the cell once per block at block entry, beside the source set's
/// `begin_block`, and holds the three values for that block (D2). No stage reads the cell itself.
///
/// # Errors
///
/// [`GraphJumpLengthError::NotAGraphPlan`] for a plan another executor renders, and
/// [`GraphJumpLengthError::AlreadyAttached`] for a second call on one plan.
pub fn attach_jump_lengths(
    plan: &mut PreparedRenderPlan,
    lengths: GraphJumpLengths,
    superseded: &engine::realtime::LiveValuesSuperseded,
) -> Result<GraphJumpLengthProducer, GraphJumpLengthError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphJumpLengthError::NotAGraphPlan)?;
    if executor.jump_lengths.is_some() {
        return Err(GraphJumpLengthError::AlreadyAttached);
    }
    let lane = std::sync::Arc::new(JumpLengthLane {
        cell: engine::realtime::LatestCell::new(),
        dirty: engine::realtime::DirtyWord::new(),
    });
    executor.jump_lengths = Some(Box::new(JumpLengthReader {
        lane: std::sync::Arc::clone(&lane),
        reader: engine::realtime::CellReader::new(),
        dirty: engine::realtime::DirtyReader::new(),
        superseded: superseded.clone(),
        held: lengths,
    }));
    Ok(GraphJumpLengthProducer {
        lane,
        writer: engine::realtime::CellWriter::new(),
    })
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// #1639 gate 1: the jump lengths the graph plan rendered last on this thread held for its
    /// block, or `None` for a block of a plan with no jump-length cell.
    static BLOCK_JUMP_LENGTHS: Cell<Option<GraphJumpLengths>> = const { Cell::new(None) };
}

/// The jump lengths the last graph block rendered on this thread held (#1639 gate 1): the values
/// read at that block's entry, or `None` when its plan has no jump-length cell.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_block_jump_lengths() -> Option<GraphJumpLengths> {
    BLOCK_JUMP_LENGTHS.with(Cell::get)
}

/// A prepared homogeneous native bank and its original graph member identities.
pub struct GraphPreparedEffectBank {
    /// The member on each active lane, in lane order: lanes `0..members.len()`.
    pub members: Box<[EffectNodeId]>,
    /// One entry per lane: `true` exactly on lanes `0..members.len()`, which carry a member, and
    /// `false` on every padded lane after them (issue #1088). It is the mask the effect's factory
    /// was bound with (`effect_contract::PrepareEffectBankRequest::active_mask`) and the mask the
    /// rack chain gathers and scatters by, so a padded lane is never read from or written to a
    /// track. A full bank is all `true`. `graph_compiler`'s padding policy pads every partial
    /// console group and every partial insert group from a per-width member threshold (#888).
    pub active_mask: Box<[bool]>,
    pub processor: Box<dyn effect_contract::PreparedNativeEffectBank>,
    /// The bank's width and shared program key, from its bind result (issue #1461).
    ///
    /// Like [`GraphPreparedEffect::metadata`], it stays on the control side: bind reads it while it
    /// builds the bank's stage (`rack::EffectBankStage` or `rack::LiveControlEffectBankStage`), and
    /// the stage keeps only the scalars its render path reads. The processor holds no copy.
    pub metadata: effect_contract::PreparedBankMetadata,
    /// Factory-declared response capability shared by this homogeneous bank.
    pub response_snapshot_declared: bool,
    /// Static descriptor identity shared by this homogeneous bank.
    pub native_id: &'static str,
    pub scratch: AoSoaScratch,
    /// The cohort chain this bank is one slot of (issue #181).
    ///
    /// The cohort planner has formed multi-slot groups since #99 F3 -- a candidate is a whole
    /// rack chain, and `plan_bank_groups` matches a *signature over slot types and order*. This is
    /// the edge that tells anyone downstream which bound banks came out of the same group, and it
    /// is what `GraphRackBankReport::bound_slots` reports.
    ///
    /// **The runtime no longer reads it** (issue #202 rec 2). It was `runtime::cohort_runs`'s
    /// source of merge candidates, and that was strictly narrower than the merge needs: a group is
    /// pooled per `RackLocation` and a builtin bank has no group at all, so `builtins -> simd1`
    /// and `simd1 -> simd2` were not expressible candidates however plainly one fed the other.
    /// Candidacy now comes from the lowered program's dataflow and the whole lane-wise relation is
    /// proved on it, so this stays as the planner's own report of what it grouped rather than as
    /// an input to what the runtime builds.
    pub cohort: GraphBankCohort,
}

/// Where one bound bank sits in its cohort chain.
///
/// `group` is an index into the plan's groups and is meaningful only within one prepared plan;
/// two banks are slots of the same chain exactly when their `group` agree. `slot` is the position
/// in the chain's program, and it is strictly increasing along the chain but need not be
/// contiguous: a slot every lane leaves at identity, or one no lane can bind, is skipped.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphBankCohort {
    /// The plan group this bank was bound from.
    pub group: u32,
    /// This bank's slot index in that group's program.
    pub slot: u32,
}
/// Every homogeneous bank a prepared plan will render, as one member list each, for the
/// lowering's bank windows (issue #169).
///
/// Grouping matters, and a flat union of members would not do: [`program::lower`] needs each
/// bank's *window* -- first member op to last -- because that is the op range over which
/// `runtime::units_of` reorders the schedule, hoisting members forward and deferring everything
/// else past them.
///
/// Effect banks are the reason this exists: #166 made dynamic-rack effects bank-eligible, and a
/// dynamic rack banks by cohort signature, so one bank's members need not be adjacent and another
/// bank's members can sit between them. Builtin banks are listed on the same terms -- their
/// window is just as real -- even though their members happen to be contiguous today.
fn bank_member_nodes(
    banks: &[GraphPreparedEffectBank],
    builtin_banks: &[GraphPreparedBuiltinBank],
) -> Vec<Vec<GraphNodeId>> {
    // One entry per bound bank, in lane order, effect banks before builtin ones.
    //
    // Issue #181 grouped these by cohort group here, because a cohort chain renders as one unit at
    // its first slot's op position and the window has to cover the whole permutation. Issue #202
    // rec 2 lets a chain fuse across rack locations and into a builtin bank, so the cohort group
    // is no longer the relation that decides which banks share a window -- the graph's dataflow
    // is. `program::chainable_bank_groups` forms that union from the lane-wise producer/consumer
    // relation, which is the same clause `runtime::chains_into` checks, and it can only be a
    // superset of the merges the runtime takes. Grouping here as well would union banks that can
    // never fuse and hold their physical slots for nothing.
    //
    // Lane order matters and is preserved: the union is positional, so lane `i` of one bank is
    // compared against lane `i` of another. `structural_ok` already requires every bank's members
    // to be strictly ascending, which is what makes that comparison well defined.
    banks
        .iter()
        .map(|bank| {
            bank.members
                .iter()
                .cloned()
                .map(GraphNodeId::Effect)
                .collect()
        })
        .chain(builtin_banks.iter().map(|bank| bank.members.to_vec()))
        .collect()
}

/// The track stages a compiler-owned builtin bank may render (issue #212).
///
/// All three are *fixed* graph stages with no automation or sidechain surface, one per track, at
/// one dependency level each -- which is what makes them bankable at all. The three internal rack
/// boundaries are not here because they are elided alias candidates that own no op
/// (`program::is_alias_candidate`), and `Input` is not because it is the source node the host
/// fills.
///
/// Adding a stage here is not sufficient to bank it: the compiler still has to prepare a kernel
/// for it and the planner still has to group it. This predicate is only the graph layer's
/// statement of which stages a bank *may* name.
const fn is_bankable_track_stage(stage: TrackStage) -> bool {
    matches!(
        stage,
        TrackStage::PostInputBuiltins | TrackStage::PostFader | TrackStage::PostMatrix
    )
}

/// A compiler-owned homogeneous post-input-builtin bank.  Unlike effect banks, this is a
/// fixed graph stage and therefore has no automation or sidechain surface.
///
/// Lane `l` is active if and only if `l < members.len()`; lanes `members.len()..width.lanes()`
/// are identity lanes carried by the bank kernel itself.  Membership is the mask, so no mask is
/// stored here: `members.len()` is in `1..=width.lanes()` and the executor gathers into and
/// scatters from exactly those lanes.
pub struct GraphPreparedBuiltinBank {
    pub backend: Backend,
    pub members: Box<[GraphNodeId]>,
    pub processor: Box<dyn GraphPreparedBuiltinBankProcessor>,
    pub scratch: AoSoaScratch,
}

/// Address-free prepared builtin-bank metadata available before render binding.
///
/// Lane `l` is active if and only if `l < members.len()`; lanes `members.len()..width.lanes()`
/// are identity lanes.
pub struct GraphPreparedBuiltinBankInfo<'a> {
    pub stage: TrackStage,
    pub backend: Backend,
    pub width: effect_contract::BankWidth,
    pub members: &'a [GraphNodeId],
}

/// A strip stage whose state a plan swap carries (issue #1277 D1): the owner of a fader and mute
/// lane or of a matrix lane.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BuiltinStripOwner {
    /// The strip's fader and mute stage, owner key `(strip ID, PostFader)`.
    Fader,
    /// The strip's matrix and pan stage, owner key `(strip ID, PostMatrix)`.
    Matrix,
}

/// What a builtin strip bank is (issue #1277 D1, D9): the fader bank or the matrix bank. Two
/// banks pair for a carry only when they are the same.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BuiltinStripStage {
    /// A fader and mute bank.
    Fader,
    /// A matrix and pan bank.
    Matrix,
}

impl BuiltinStripStage {
    /// Whether a bank of this kind holds `owner`'s lanes.
    ///
    /// Each stage holds exactly one owner, so one bank's carried strip moves share one owner: the
    /// carry drain (`carried_strip_lane_mask`, `carry_strip_lanes`) relies on this and has no owner
    /// filter (#1538). A stage that holds two owners must restore that filter.
    #[must_use]
    pub const fn holds(self, owner: BuiltinStripOwner) -> bool {
        matches!(
            (self, owner),
            (Self::Fader, BuiltinStripOwner::Fader) | (Self::Matrix, BuiltinStripOwner::Matrix)
        )
    }
}

/// Render contract for an already-prepared builtin bank.
pub type BuiltinProcessor = Box<dyn GraphPreparedBuiltinBankProcessor>;

pub trait GraphPreparedBuiltinBankProcessor: Send + Any {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    /// The heap bytes this value retains (#1074), exclusive of the value itself: every `Box`,
    /// `Vec`, `String` and `Arc` allocation reachable through its fields, a `Vec` or `String` at
    /// its capacity, an `Arc`'s allocation in full whoever else holds it. Whoever boxes the value
    /// adds `size_of_val` for the box. Computed from capacities on the control thread, never on
    /// the render path. Required, so a new owner cannot under-report by default.
    fn retained_bytes(&self) -> u64;
    /// This processor as mutable `Any`, so [`Self::adopt_input_lane`] can downcast a predecessor
    /// processor to its own concrete type at a plan swap (issue #1276 D3). `None` (the default)
    /// offers nothing.
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        None
    }
    /// How many populated input-section lanes this bank can hand over or take at a plan swap
    /// (issue #1276), or `None` (the default) for a bank whose lanes do not carry. Read off the
    /// render thread, when a carry program is installed.
    fn carried_input_lanes(&self) -> Option<usize> {
        None
    }
    /// Whether [`Self::adopt_input_lane`] would take `predecessor`'s lane `predecessor_lane` into
    /// this bank's lane `lane`: both banks carry input lanes, are of one concrete type and width,
    /// and both lanes are populated. `false` (the default) takes nothing. Render-thread code:
    /// allocation-free and bounded.
    fn can_adopt_input_lane(
        &self,
        lane: usize,
        predecessor: &dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        let _ = (lane, predecessor, predecessor_lane);
        false
    }
    /// Copy `predecessor`'s lane `predecessor_lane` -- its whole input-section state and its live
    /// channel-symmetry terms -- into this bank's lane `lane`, at the swap block (issue #1276
    /// D2). The caller has drained `predecessor`'s live cells first
    /// ([`Self::drain_for_carry`]). `false` (the default) copies nothing. Render-thread code:
    /// allocation-free and bounded.
    fn adopt_input_lane(
        &mut self,
        lane: usize,
        predecessor: &mut dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        let _ = (lane, predecessor, predecessor_lane);
        false
    }
    /// Apply every live-control record waiting at entry in the input cells of the lanes in
    /// `lanes` (bit `n` is lane `n`) to lane state, exactly as [`Self::begin_block`] would at the
    /// next block, before a plan swap exports those lanes (issue #1276 D4). A record drained here
    /// takes effect on the successor's first sample, the sample it would have reached without the
    /// swap.
    ///
    /// `lanes` names the lanes the swap carries, and only those. A lane the swap restarts is not
    /// read, so the writes its cells replaced are not counted in `live_values_superseded` (#1312
    /// D2): the count does not depend on which lanes share a bank, and so not on the lane width.
    ///
    /// Every record of a named lane present at entry is consumed: one whose apply is refused is
    /// counted and the drain goes on, so no later record of the lane or the bank stays behind in
    /// a queue that never renders again. Returns the refused count, which admission validation
    /// makes zero. The default drains nothing and returns zero.
    fn drain_for_carry(&mut self, lanes: u8) -> usize {
        let _ = lanes;
        0
    }
    /// [`Self::drain_for_carry`] for the strip stage `owner`: apply every pending fader, mute or
    /// matrix value of the lanes in `lanes` to lane state (issue #1277 D8). The default drains
    /// nothing and returns zero.
    fn drain_strip_for_carry(&mut self, owner: BuiltinStripOwner, lanes: u8) -> usize {
        let _ = (owner, lanes);
        0
    }
    /// Which strip stage this bank is and how many populated lanes it can hand over or take at a
    /// plan swap (issue #1277 D1), or `None` (the default) for a bank whose lanes do not carry
    /// fader, mute or matrix state. Read off the render thread, when a carry program is installed.
    fn carried_strip_lanes(&self) -> Option<(BuiltinStripStage, usize)> {
        None
    }
    /// Whether [`Self::adopt_strip_lane`] would take `predecessor`'s lane `predecessor_lane` into
    /// this bank's lane `lane` for `owner`: both banks are of one concrete type and width, both
    /// lanes are populated, and the type carries `owner`. `false` (the default) takes nothing.
    /// Render-thread code: allocation-free and bounded.
    fn can_adopt_strip_lane(
        &self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
    ) -> bool {
        let _ = (owner, lane, predecessor, predecessor_lane);
        false
    }
    /// Copy `predecessor`'s lane `predecessor_lane` -- the whole state of `owner`'s stage: its
    /// gains, mutes or matrix and any ramp in flight, and a fader stage's stored automation event
    /// state (#1648 D2) -- into this bank's lane `lane`, at the swap block (issue #1277 D8). The
    /// caller has applied `predecessor`'s pending live values first
    /// ([`Self::drain_strip_for_carry`]). `continued_cells` is the move's
    /// [`GraphStripMove::continued_cells`]. `false` (the default) copies nothing. Render-thread
    /// code: allocation-free and bounded.
    fn adopt_strip_lane(
        &mut self,
        owner: BuiltinStripOwner,
        lane: usize,
        predecessor: &mut dyn GraphPreparedBuiltinBankProcessor,
        predecessor_lane: usize,
        continued_cells: u8,
    ) -> bool {
        let _ = (owner, lane, predecessor, predecessor_lane, continued_cells);
        false
    }
    /// Whether [`Self::swap_builtin_lanes`] would exchange lane `lane`'s whole per-lane state with
    /// `peer`'s lane `peer_lane` (`None`: a lane of this bank): both lanes are populated, and a
    /// `peer` is a bank of this concrete type and width (issue #1765). Decided without mutation,
    /// allocation-free and bounded. The default `false` is a bank that has no swap: it never
    /// claims one, so a chain holding it refuses the swap.
    fn can_swap_builtin_lanes(
        &self,
        lane: usize,
        peer: Option<&dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        let _ = (lane, peer, peer_lane);
        false
    }
    /// Exchange lane `lane`'s whole per-lane state -- stage state, ramps in flight, prepared
    /// endpoints, live cell readers with their dirty bits and carry marks, live symmetry terms and
    /// stored automation -- with `peer`'s lane `peer_lane` (`None`: a lane of this bank), at a
    /// block boundary on the render thread (issue #1765), for `rack::BankStage::swap_lanes`. All
    /// or nothing: returns `false` and changes nothing unless [`Self::can_swap_builtin_lanes`]
    /// holds. Allocation-free and bounded. The default `false` swaps nothing.
    fn swap_builtin_lanes(
        &mut self,
        lane: usize,
        peer: Option<&mut dyn GraphPreparedBuiltinBankProcessor>,
        peer_lane: usize,
    ) -> bool {
        let _ = (lane, peer, peer_lane);
        false
    }
    /// The lanes (bit `n` is lane `n`) whose [`Self::begin_block`] drains a live lane, or `0` for
    /// a bank with none. `BuiltinStage` forwards it to `rack::BankStage::live_lanes`, which
    /// `BankChain::new` checks against the slot's active lanes in a debug build, off the render
    /// thread (#1504 D3). The default `0` is a bank that holds no live lane.
    fn live_lanes(&self) -> u8 {
        0
    }
    /// The lanes of this bank at **exact rest**, bit `n` for lane `n`, padded lanes included
    /// (issue #1781, #1107 D3): one block of exact `+0.0` on both planes would leave every state
    /// word of the lane bit-identical and write exact `+0.0` on both of its output planes. A
    /// **clock** -- a word a `+0.0` block advances by the block's length alone -- is not a state
    /// word for the claim; [`Self::advance_builtin_clocks`] advances it.
    ///
    /// `BuiltinStage` forwards it to `rack::BankStage::lanes_at_rest`, so it carries that
    /// method's contract: it is asked after this block's [`Self::begin_block`] (the drain) and
    /// [`Self::set_block_time`], before [`Self::process`], and it answers for the block those two
    /// prepared. A lane with work of its own in the block (stored automation, a timed operation)
    /// is not at rest. From state only: allocation-free, lock-free and bounded by the lane count.
    ///
    /// The default `0` declines every lane: a bank that has not written the predicate is never
    /// skipped, and a wrong set bit is the only answer that could be unsound.
    fn builtin_lanes_at_rest(&self) -> u32 {
        0
    }
    /// Advance the clocks of the lanes in `lanes_mask` (bit `n` is lane `n`) by `frames`, exactly
    /// as one rendered block of `frames` frames of exact `+0.0` would, for a block the caller skips
    /// (issue #1781). The caller's contract is `rack::BankStage::advance_clocks`': every input
    /// word of the block is `+0.0`, the mask is every lane of the bank (a bank is skipped whole),
    /// and each lane in it is one [`Self::builtin_lanes_at_rest`] reported for the block. After the
    /// call every state word is bit-identical to what that rendered block would leave, with the
    /// two exceptions `rack::BankStage::advance_clocks` states: the `[process calls, frames
    /// processed]` qualification counters do not count a skipped block, and on a collapsed chain
    /// the input's frozen right silence counter advances (the disengage copies the left over it).
    ///
    /// The default is a no-op: exact for a bank with no clocks, and sound for any bank whose
    /// predicate declines, because a declined lane is never skipped. Allocation-free and bounded
    /// by the lane count, never by `frames`.
    fn advance_builtin_clocks(&mut self, lanes_mask: u32, frames: u32) {
        let _ = (lanes_mask, frames);
    }
    /// Arm the fade-in of fader lane `lane`, a strip a plan swap adds during playback (issue #1288
    /// D2): each channel the session does not mute becomes a settled mute that
    /// [`Self::fire_fade_in`] unmutes, and a channel the session mutes arms too, its fire going
    /// to its prepared mute unless a record before the fire unmutes it (#2080 R1). Called by
    /// [`install_fade_arms`], off the render thread, before the plan is published. Returns whether
    /// the lane was armed; the default arms nothing.
    fn arm_fade_in(&mut self, lane: usize) -> bool {
        let _ = lane;
        false
    }
    /// Fire lane `lane`'s armed fade-in over `ramp_samples` at this block boundary (issue #1288
    /// D3): the executor calls it before the lane's unit runs, so the block's own live values
    /// drain after it. Render-thread code: allocation-, lock- and syscall-free and bounded. The
    /// default does nothing.
    fn fire_fade_in(&mut self, lane: usize, ramp_samples: u32) {
        let _ = (lane, ramp_samples);
    }
    /// Take the floored input arrival `a(n)` of each member, in lane order (#1647a, R17-2):
    /// `arrivals[l]` is lane `l`'s, and a lane past the slice is padding. The graph compiler calls
    /// it once, off the render thread, at the end of `compile_with_builtins`, from
    /// `GraphCompileReport::input_arrivals` (#1285). The default keeps nothing.
    fn set_input_arrivals(&mut self, arrivals: &[u64]) {
        let _ = arrivals;
    }
    /// Take this block's node-time context (#1647a, R17-3). The graph calls it on every builtin
    /// slot of a bank unit once per block, before the unit's chain runs, so `begin_block` and
    /// `process` of the same block read what it kept. The context borrows the source set only for
    /// the call. Render-thread code: allocation-, lock- and syscall-free and bounded. The default
    /// keeps nothing.
    fn set_block_time(&mut self, time: &GraphBlockTime<'_>) {
        let _ = time;
    }
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError>;
    /// [`Self::process`] for a block whose right input is its left plane, forwarded from
    /// `rack::BankStage::process_from_left` with that method's contract (issue #975): `right` is
    /// output only, `Ok(true)` when the bank wrote it from `left` itself, `Ok(false)` when it
    /// copied `left` into it first, as this default does.
    fn process_from_left(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<bool, RenderError> {
        right.copy_from_slice(left);
        self.process(left, right, frames, first_sample)?;
        Ok(false)
    }
    /// Copy one bank member's retained response words without advancing render state.
    fn copy_response_snapshot_lane(
        &self,
        _lane: usize,
        _sample_rate_hz: u32,
        _request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        Err(ResponseAnalysisError::UnsupportedCapability)
    }
    /// Whether this prepared builtin declares a response provider for its owner.
    fn response_snapshot_declared(&self) -> bool {
        false
    }
    /// The static native identity of this builtin response provider, when declared.
    fn response_snapshot_native_id(&self) -> Option<&'static str> {
        None
    }
    /// Cumulative `[process_calls, frames_processed]` after render is disarmed.
    fn qualification_counters(&self) -> [u64; 2] {
        [0, 0]
    }

    /// Drain this bank's live-control queues, before any lane of the block is dispatched.
    ///
    /// Forwarded from `rack::BankStage::begin_block`, and it carries that method's
    /// whole contract: an admitted record takes effect on the first sample of the block that
    /// drains it, and a record that writes one channel's upstream word clears the
    /// channel-symmetry witness' `LIVE` term *before* the collapse dispatch reads it. A bank
    /// upstream of the fader/matrix seam that drained inside `process` instead would publish a
    /// one-channel retarget onto both channels of the block that admitted it.
    ///
    /// The default is a no-op, which is what a live-control-free plan pays.
    ///
    /// `snapshot` is the block's live snapshot `S` (#1504): the revision render read from the
    /// running plan's gate at the start of the block. A drain that reads live cells reads under
    /// it; an observation queue is a FIFO and ignores it. The bank stage hands every
    /// bank the block's own, once per block, whether or not the block's audio is silent: silence
    /// may skip a bank's processing and never this drain.
    fn begin_block(
        &mut self,
        first_sample: u64,
        snapshot: LiveSnapshot,
    ) -> Result<(), RenderError> {
        let _ = (first_sample, snapshot);
        Ok(())
    }

    /// This builtin bank's channel-symmetry witness for one lane of the cohort.
    ///
    /// The default declines, for the reason `rack::BankStage::lane_symmetry` gives:
    /// an unclassified stage must not claim eligibility for work nobody checked.
    fn lane_symmetry(&self, lane: usize) -> ChannelSymmetryWitness {
        let _ = lane;
        ChannelSymmetryWitness::DECLINED
    }

    /// Which side of the fader/matrix seam this builtin bank sits on.
    ///
    /// The default is [`SeamSide::UpstreamOfSeam`] for the reason
    /// `rack::BankStage::seam_side` gives: it is the conservative answer, because an
    /// upstream stage that has not written a one-plane body ends the collapsible prefix, and
    /// declines the whole chain when it is the first slot.
    fn seam_side(&self) -> SeamSide {
        SeamSide::UpstreamOfSeam
    }

    /// Whether this bank implements [`process_mono`](Self::process_mono) and
    /// [`desymmetrize`](Self::desymmetrize).
    fn supports_mono_collapse(&self) -> bool {
        false
    }

    /// Render one block with the cohort's two channels collapsed onto `left`.
    ///
    /// There is no right plane here on purpose: a collapsed chain gathers one, and the seam writes
    /// the other after this stage has run.
    ///
    /// Whatever this call publishes besides the plane -- per-channel recovery counts, sanitised
    /// totals, lifetime counters -- must be what a dual block would have published, not the half
    /// this call computed: the right plane the seam is about to write carries exactly the left
    /// plane's samples. `rack::BankStage::process_mono` states the rule and
    /// `builtins/tests/mono_collapse.rs` is the gate on the one bank that has any.
    fn process_mono(
        &mut self,
        left: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = (left, frames, first_sample);
        Err(RenderError::InvalidEnvelope)
    }

    /// Copy every lane's left-channel state onto the right channel (the disengage boundary).
    fn desymmetrize(&mut self) {}

    /// Whether this bank can prove, right now, that its two channels' state is bit-equal.
    ///
    /// The mono collapse's way back (M3). Same contract, same declining default and same cost rule
    /// as `effect_contract::PreparedNativeEffectBank::channels_agree`.
    fn channels_agree(&self) -> bool {
        false
    }
}
type OptionalSourceBindResult = Result<
    PreparedRenderPlan,
    (
        PreparedGraphPlan,
        GraphRuntimeBindings,
        Option<GraphPreparedSourceSet>,
        &'static str,
    ),
>;

impl PreparedGraphPlan {
    /// The input-side track delays this plan lowers, in normalized track order (#210 phase 2).
    ///
    /// Empty is the answer for every session that declared no delay, and that emptiness is the
    /// feature's off gate: an empty list means `node_kind` never leaves its `SourceInput` arm, so
    /// the lowered program is the one this plan would have had before the feature existed.
    #[must_use]
    pub fn track_delays(&self) -> &[PreparedTrackDelay] {
        &self.track_delays
    }

    /// The source-claim lines this plan's `track_delays` include, in node order (#1287 D1).
    ///
    /// Empty for every plan compiled without floors.
    #[must_use]
    pub fn source_claim_lines(&self) -> &[SourceClaimLine] {
        &self.source_claim_lines
    }

    /// Record the source-claim lines the compiler folded into this plan's `track_delays`
    /// (#1287 D1). A report only: the lowering reads `track_delays`, so this changes no program.
    /// Bind refuses a plan whose lines are out of node order, name a node that is not a track-stage
    /// `Input` of the graph, or are longer than either lane of that node's track-delay entry.
    #[must_use]
    pub fn with_source_claim_lines(mut self, lines: Vec<SourceClaimLine>) -> Self {
        self.source_claim_lines = lines;
        self
    }

    fn has_valid_structural_layout(&self) -> bool {
        let graph_nodes: BTreeSet<_> = self.spec.nodes.iter().map(|node| &node.id).collect();
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
                    .any(|node| level_by_node.insert(node, level.level).is_some())
            {
                return false;
            }
            previous_level = Some(level.level);
            flattened.extend(level.nodes.iter());
        }
        if flattened.into_iter().ne(self.sequential_schedule.iter())
            || level_by_node
                .keys()
                .copied()
                .ne(graph_nodes.iter().copied())
        {
            return false;
        }

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

        // Every bank lists graph nodes in strictly ascending id order, and no node is in two
        // banks. Since issue #2134 a bank's members need not share a dependency level: the
        // executor places each bank by the graph contracted by bank (`program::lower`), and a
        // plan whose contraction is cyclic, or one of whose banks has a member feeding another,
        // fails that lowering and is refused here by `lowered` with this same code.
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
            if members.iter().any(|member| {
                !level_by_node.contains_key(member) || !bank_members.insert(member.clone())
            }) {
                return false;
            }
        }
        // #1287 D1: every source-claim line, in node order, names a track-stage `Input` node of
        // the graph whose track-delay entry holds at least the line on both lanes. The lowering
        // reads `track_delays`, so a row it does not hold would report a line that never runs.
        let track_delays: BTreeMap<_, _> = self
            .track_delays
            .iter()
            .map(|delay| (&delay.node, [delay.left_samples, delay.right_samples]))
            .collect();
        self.source_claim_lines
            .windows(2)
            .all(|pair| pair[0].node < pair[1].node)
            && self.source_claim_lines.iter().all(|line| {
                matches!(
                    line.node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                ) && line.samples.0 > 0
                    && graph_nodes.contains(&line.node)
                    && track_delays.get(&line.node).is_some_and(|lanes| {
                        lanes.iter().all(|&lane| u64::from(lane) >= line.samples.0)
                    })
            })
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
    /// Every retained effect bank's members, in lane order.
    ///
    /// The counterpart of [`builtin_bank_members`](Self::builtin_bank_members), and grouped rather
    /// than flattened for the reason `bank_member_nodes` gives: which lanes a bank covers **in
    /// what order** is the whole of what a downstream merge or a track-to-lane join can use, and a
    /// flat union destroys it.
    pub fn effect_bank_members(&self) -> impl Iterator<Item = &[EffectNodeId]> {
        self.banks.iter().map(|bank| bank.members.as_ref())
    }
    /// Address-free semantic membership retained by production builtin banks.
    pub fn builtin_bank_info(&self) -> impl Iterator<Item = GraphPreparedBuiltinBankInfo<'_>> {
        self.builtin_banks
            .iter()
            .map(|bank| GraphPreparedBuiltinBankInfo {
                stage: match bank.members.first() {
                    Some(GraphNodeId::TrackStage { stage, .. }) => *stage,
                    _ => TrackStage::Input,
                },
                backend: bank.backend,
                width: bank.scratch.width(),
                members: &bank.members,
            })
    }
    /// Give each builtin bank its members' floored input arrivals `a(n)` (#1647a, R17-2), through
    /// [`GraphPreparedBuiltinBankProcessor::set_input_arrivals`]. `arrivals` is
    /// `GraphCompileReport::input_arrivals` (#1285 D2): every node with a nonzero floored arrival,
    /// sorted by node ID; a member absent from it has arrival 0. Off the render thread, once,
    /// after the banks are attached and before bind.
    pub fn install_builtin_input_arrivals(&mut self, arrivals: &[(GraphNodeId, LatencySamples)]) {
        for bank in &mut self.builtin_banks {
            let lanes: Vec<u64> = bank
                .members
                .iter()
                .map(|member| floored_input_arrival(arrivals, member))
                .collect();
            bank.processor.set_input_arrivals(&lanes);
        }
    }
    /// Attach sealed fixed-stage banks before binding.  The graph compiler remains responsible
    /// for deciding eligibility; this validates only the immutable graph shape.
    pub fn with_builtin_banks(
        mut self,
        banks: Vec<GraphPreparedBuiltinBank>,
        resource: GraphBuiltinBankResourceEstimate,
    ) -> Result<Self, GraphBuiltinBankAttachError> {
        let mut seen = BTreeSet::new();
        // Membership in `required_bindings`, interned once per call (issue #962). The field is in
        // schedule order and about four entries per track, and every bank member is looked up in
        // it, so a linear `contains` per member made every host's compile quadratic in the track
        // count. This is a sorted copy of the borrows, used for nothing but membership: the field
        // itself, and every order derived from it, is untouched.
        let mut required: Vec<&GraphNodeId> = self.required_bindings.iter().collect();
        required.sort_unstable();
        let level_by_node: BTreeMap<_, _> = self
            .dependency_levels
            .iter()
            .flat_map(|level| level.nodes.iter().map(move |node| (node, level.level)))
            .collect();
        for bank in &banks {
            // Every member of one bank renders the same stage at the same lane order, so the
            // stage is read off lane 0 and every other lane must agree with it. A bank mixing
            // stages would be a kernel applied to the wrong audio; there is no lane mask that
            // could rescue it, so it is refused here rather than declined later.
            let stage_of = |node: &GraphNodeId| match node {
                GraphNodeId::TrackStage { stage, .. } if is_bankable_track_stage(*stage) => {
                    Some(*stage)
                }
                _ => None,
            };
            let Some(stage) = bank.members.first().and_then(stage_of) else {
                return Err(GraphBuiltinBankAttachError::InvalidMembers);
            };
            if bank.members.is_empty()
                || bank.members.len() > bank.scratch.width().lanes() as usize
                || !bank.scratch.width().matches_backend(bank.backend)
                || bank
                    .members
                    .iter()
                    .any(|node| stage_of(node) != Some(stage) || !seen.insert(node))
            {
                return Err(GraphBuiltinBankAttachError::InvalidMembers);
            }
            // Since issue #2134 a bank's members need not share a dependency level: the
            // executor places the bank by the graph contracted by bank, and a plan with no such
            // order fails lowering at bind (`graph.scheduler.layout`).
            if bank.members.iter().any(|member| {
                !level_by_node.contains_key(member) || required.binary_search(&member).is_err()
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
        // `program` is *derived* from this plan's own fields (#99 F2), and since #169 the bank
        // member lists are among them: attaching banks changes which buffers may be shared, so
        // the derivation is redone here. Without this, `program()` would describe the plan as it
        // was before the attach while bind-time `lowered()` described it as it is.
        self.program = self.lower_from_current_fields();
        Ok(self)
    }
    /// Give every route into a submix input, and every route whose source tap precedes its
    /// strip's fader (#1391), a live lane (issue #1220 D1, D7, #1347 D1): one latest-target cell
    /// per route, in canonical route-ID order, whose producers are returned. Every lane adds the
    /// replaced writes it finds to `superseded`, the session's count.
    ///
    /// Runs after compile and before bind. A `post_fader` or `post_pan` route into an output keeps
    /// its prepared constant, its fold and its structural edits. Bind turns each lane's route op
    /// into a live route, which reads its cell under the block's live snapshot at the start of
    /// every block and ramps by the indexed ramp; a live route never folds (VERIFY-2 M3). Without
    /// this call the plan binds what it bound before.
    ///
    /// Refused when called twice ([`GraphRouteControlError::AlreadyAttached`]) or when a route
    /// that gets a lane is a node the plan requires a processor for.
    pub fn attach_route_controls(
        &mut self,
        superseded: &engine::realtime::LiveValuesSuperseded,
    ) -> Result<Vec<GraphRouteControlProducer>, GraphRouteControlError> {
        self.attach_route_controls_following(superseded, true, &[], |_| None)
            .map(|(producers, _, _)| producers)
    }

    /// [`Self::attach_route_controls`] with following routes (#1652 D1, #1652c): every route into
    /// a submix input for which `following` gives a source gets a seven-word lane reading it
    /// (`follow_route_control_pair`) and no six-word lane; every other such route gets a
    /// six-word lane when `six_word` is set and none otherwise; a pre-fader route into an output
    /// takes the six-word rule. Both producer lists are in canonical route-ID order. `following`
    /// is asked once per route into a submix, in that order, so a route into an output never
    /// follows (decision 13 P11).
    ///
    /// Each route of `transition` that gets no lane here becomes a transition route (#1363 D2):
    /// it binds live with no lane, so it never folds and a fade arm
    /// ([`GraphFadeTarget::Route`]) can ramp it, and the third value charges it. A route of
    /// `transition` that gets a lane anyway is live already and takes nothing more.
    ///
    /// Refused as [`Self::attach_route_controls`] is, with
    /// [`GraphRouteControlError::CursorTable`] when a following route's cursor tables cannot be
    /// allocated (nothing is attached then), and with
    /// [`GraphRouteControlError::UnknownTransitionRoute`] when `transition` names a route the plan
    /// does not prepare.
    #[allow(clippy::type_complexity)]
    pub fn attach_route_controls_following(
        &mut self,
        superseded: &engine::realtime::LiveValuesSuperseded,
        six_word: bool,
        transition: &[StableGraphId],
        mut following: impl FnMut(&StableGraphId) -> Option<GraphFollowRouteSource>,
    ) -> Result<
        (
            Vec<GraphRouteControlProducer>,
            Vec<GraphFollowRouteControlProducer>,
            GraphTransitionRoutes,
        ),
        GraphRouteControlError,
    > {
        if self.route_controls.is_some() || !self.follow_route_controls.is_empty() {
            return Err(GraphRouteControlError::AlreadyAttached);
        }
        // D1: the route's destination edge lands on a submix input (a strip `Input` stage, which
        // only a submix has as a route destination; the legacy `Submix` node likewise).
        let into_submix: BTreeSet<&StableGraphId> = self
            .spec
            .edges
            .iter()
            .filter_map(|edge| match (&edge.id, &edge.destination.node) {
                (
                    GraphEdgeId::RouteDestination { route_id },
                    GraphNodeId::TrackStage {
                        stage: TrackStage::Input,
                        ..
                    }
                    | GraphNodeId::Submix { .. },
                ) => Some(route_id),
                _ => None,
            })
            .collect();
        // #1391: a route whose source edge leaves its strip before the fader has a live lane at
        // any destination (`controls::route_rows_live` is the model-side form of this rule). A
        // `post_fader` or `post_pan` route into the output keeps its constant and its fold.
        let pre_fader: BTreeSet<&StableGraphId> =
            self.spec
                .edges
                .iter()
                .filter_map(|edge| match (&edge.id, &edge.source.node) {
                    (
                        GraphEdgeId::RouteSource { route_id },
                        GraphNodeId::TrackStage { stage, .. },
                    ) if *stage < TrackStage::PostFader => Some(route_id),
                    _ => None,
                })
                .collect();
        let mut live: Vec<&GraphNodeId> = self
            .routes
            .iter()
            .map(|route| &route.node)
            .filter(|node| {
                matches!(node, GraphNodeId::Route { route_id }
                    if into_submix.contains(route_id) || pre_fader.contains(route_id))
            })
            .collect();
        live.sort_unstable();
        live.dedup();
        let required: BTreeSet<&GraphNodeId> = self.required_bindings.iter().collect();
        if live.iter().any(|node| required.contains(node)) {
            return Err(GraphRouteControlError::BoundRoute);
        }
        // #1363 D2: the transition routes, each a prepared route that gets no lane below.
        let mut transition_nodes: Vec<GraphNodeId> = Vec::with_capacity(transition.len());
        for route_id in transition {
            let node = GraphNodeId::Route {
                route_id: route_id.clone(),
            };
            if !self.routes.iter().any(|route| route.node == node) {
                return Err(GraphRouteControlError::UnknownTransitionRoute);
            }
            if required.contains(&node) {
                return Err(GraphRouteControlError::BoundRoute);
            }
            transition_nodes.push(node);
        }
        // Which kind of lane each route gets: a following route its source, a six-word route
        // nothing, a route with no lane is left out.
        let mut kinds: Vec<(&StableGraphId, Option<GraphFollowRouteSource>)> =
            Vec::with_capacity(live.len());
        for node in live {
            let GraphNodeId::Route { route_id } = node else {
                continue;
            };
            // Only a route into a submix may follow (decision 13 P11, #1218).
            if !into_submix.contains(route_id) {
                if six_word {
                    kinds.push((route_id, None));
                }
                continue;
            }
            match following(route_id) {
                Some(source) => kinds.push((route_id, Some(source))),
                None if six_word => kinds.push((route_id, None)),
                None => {}
            }
        }
        // A transition route that gets a lane is live already.
        transition_nodes.retain(|node| {
            !matches!(node, GraphNodeId::Route { route_id }
                if kinds.iter().any(|(kind, _)| *kind == route_id))
        });
        transition_nodes.sort_unstable();
        transition_nodes.dedup();
        // D8: bind builds the route-activity table for a plan with a live route. The compile-time
        // estimate charged it already when some prepared gate silences; otherwise it is this
        // attach's to state.
        let (activity_bytes, activity_largest) = if (kinds.is_empty()
            && transition_nodes.is_empty())
            || self.routes.iter().any(|route| route.gate.silences())
        {
            (0, 0)
        } else {
            let routes = u64::try_from(self.routes.len()).unwrap_or(u64::MAX);
            let nodes = u64::try_from(self.spec.nodes.len()).unwrap_or(u64::MAX);
            (
                runtime::route_activity_bound_bytes(routes, nodes).unwrap_or(u64::MAX),
                runtime::route_activity_largest_bytes(routes, nodes).unwrap_or(u64::MAX),
            )
        };
        let follow_count = kinds.iter().filter(|(_, source)| source.is_some()).count();
        let mut bindings = Vec::with_capacity(kinds.len() - follow_count);
        let mut producers = Vec::with_capacity(kinds.len() - follow_count);
        let mut follow_bindings = Vec::with_capacity(follow_count);
        let mut follow_producers = Vec::with_capacity(follow_count);
        for (route_id, source) in kinds {
            if let Some(source) = source {
                let (binding, producer) = follow_route_control_pair(
                    route_id,
                    superseded,
                    source,
                    activity_bytes,
                    activity_largest,
                    u64::try_from(self.routes.len()).unwrap_or(u64::MAX),
                )
                .map_err(GraphRouteControlError::CursorTable)?;
                follow_bindings.push(binding);
                follow_producers.push(producer);
                continue;
            }
            let lane = std::sync::Arc::new(RouteLane::new());
            bindings.push(GraphRouteControlBinding {
                node: GraphNodeId::Route {
                    route_id: route_id.clone(),
                },
                control: Some(Box::new(RouteControlLane {
                    lane: std::sync::Arc::clone(&lane),
                    reader: engine::realtime::CellReader::new(),
                    dirty: engine::realtime::DirtyReader::new(),
                    superseded: superseded.clone(),
                })),
            });
            producers.push(GraphRouteControlProducer {
                route_id: route_id.as_str().into(),
                lane,
                writer: engine::realtime::CellWriter::new(),
                activity_bytes,
                activity_largest,
            });
        }
        // #1363 D2: the activity table is charged once: through the producers when there are any.
        let transition_routes = GraphTransitionRoutes {
            routes: u64::try_from(transition_nodes.len()).unwrap_or(u64::MAX),
            route_id_bytes: transition_nodes.iter().fold(0_u64, |sum, node| {
                let GraphNodeId::Route { route_id } = node else {
                    return sum;
                };
                sum.saturating_add(u64::try_from(route_id.as_str().len()).unwrap_or(u64::MAX))
            }),
            activity_bytes: if producers.is_empty() && follow_producers.is_empty() {
                activity_bytes
            } else {
                0
            },
            activity_largest: if producers.is_empty() && follow_producers.is_empty() {
                activity_largest
            } else {
                0
            },
        };
        bindings.extend(
            transition_nodes
                .into_iter()
                .map(|node| GraphRouteControlBinding {
                    node,
                    control: None,
                }),
        );
        self.route_controls = Some(bindings);
        self.follow_route_controls = follow_bindings;
        Ok((producers, follow_producers, transition_routes))
    }
    /// The lowered executable program, or `None` when the plan's schedule, levels and spec
    /// disagree (which bind-time structural validation rejects).
    #[must_use]
    pub fn program(&self) -> Option<&program::ExecutionProgram> {
        self.program.as_ref()
    }
    /// The one place a plan's executable program is derived from its fields.
    ///
    /// Three callers need it -- construction, `attach_builtin_banks`, and bind-time re-derivation
    /// -- and they must not be able to disagree about which fields feed it. Since #169 the bank
    /// member lists are among those fields, because a bank's window constrains what colouring may
    /// share; before that they were not, which is why attaching banks used to be able to leave
    /// `program` untouched.
    ///
    /// `required_bindings` is not one of them. Issue #925 made it one, to elide a builtin stage
    /// the plan did not list, and issue #958 reverted that with the builtins-less compile it
    /// served (#959): which nodes are alias candidates is decided by node kind alone (elision also
    /// needs the candidate's input edges to allow it).
    fn lower_from_current_fields(&self) -> Option<program::ExecutionProgram> {
        program::lower_hinted(
            &self.spec,
            &self.sequential_schedule,
            &self.dependency_levels,
            &self.inserted_delays,
            &bank_member_nodes(&self.banks, &self.builtin_banks),
            &self.lowering_hint,
        )
        .ok()
    }
    /// Re-derives the executable program from this plan's *current* semantic fields.
    ///
    /// [`program`](Self::program) is derived once at construction and gated on every compile
    /// (#99 F2). The schedule, levels and inserted delays are public, and the transactional bind
    /// contract hands a rejected plan back for the caller to repair and re-bind, so binding must
    /// lower the plan it now holds rather than the one the constructor saw. Lowering is a pure
    /// function of those fields, so the two can never disagree about an unmodified plan.
    fn lowered(&self) -> Option<program::ExecutionProgram> {
        let program = self.lower_from_current_fields()?;
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
        Self::new_hinted(parts, program::LoweringHint::default())
    }
    /// [`new`](Self::new), keeping the node positions the compiler resolved as hints for every
    /// lowering of this plan (#1749). The plan is the one `new` builds for every hint.
    pub fn new_hinted(parts: PreparedGraphPlanParts, lowering_hint: program::LoweringHint) -> Self {
        // #99 F2: the executable program is *derived* from the plan's own spec, schedule, levels,
        // PDC edges and banks, so it cannot disagree with the semantic graph and no caller has to
        // supply or maintain it. `None` means those disagree -- a schedule that is not the
        // concatenation of the levels, an edge running backwards, an unsorted spec -- which
        // `has_valid_structural_layout` rejects at bind time anyway. Hand-built plans in tests are
        // the only things that ever produce it, and they keep working exactly as before.
        //
        // The plan is assembled first and the derivation runs against the assembled plan, so this
        // and `attach_builtin_banks` cannot form different opinions about which fields feed it.
        let mut plan = Self {
            program: None,
            lowering_hint,
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
            track_delays: parts.track_delays,
            source_claim_lines: Vec::new(),
            effects: parts.effects,
            effect_controls: parts.effect_controls,
            effect_observations: parts.effect_observations,
            route_controls: None,
            follow_route_controls: Vec::new(),
            banks: parts.banks,
            builtin_banks: parts.builtin_banks,
            observers: parts.observers,
            pinned_strips: Vec::new(),
            _not_sync: Cell::new(()),
        };
        plan.program = plan.lower_from_current_fields();
        plan
    }

    /// Name the strips whose stored automation drives one of their effect bypasses (issue
    /// #2135): the lane packer pins each one where bind places it, for the plan's life, since its
    /// bypass pattern is data-driven. Off the render thread, before bind; a later call replaces
    /// the list.
    pub fn pin_bypass_automated_strips(&mut self, strips: Vec<Box<str>>) {
        self.pinned_strips = strips;
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
    ) -> OptionalSourceBindResult {
        let (
            duplicate_binding,
            coverage_matches,
            source_overlap,
            source_claims_valid,
            valid_observers,
        ) = {
            let mut supplied = Vec::with_capacity(bindings.nodes.len());
            supplied.extend(bindings.nodes.iter().map(|binding| &binding.node));
            let supplied_count = supplied.len();
            supplied.sort_unstable();
            supplied.dedup();

            let mut builtin_bank_members = Vec::with_capacity(self.builtin_bank_members().count());
            builtin_bank_members.extend(self.builtin_bank_members());
            builtin_bank_members.sort_unstable();
            builtin_bank_members.dedup();

            let mut required = Vec::with_capacity(self.required_bindings.len());
            required.extend(self.required_bindings.iter().filter(|node| {
                builtin_bank_members
                    .binary_search_by(|member| member.cmp(node))
                    .is_err()
            }));
            required.sort_unstable();
            required.dedup();

            let source_claims = source_set
                .as_ref()
                .map(GraphPreparedSourceSet::claims)
                .unwrap_or_default();
            let mut source_claim_nodes = Vec::with_capacity(source_claims.len());
            source_claim_nodes.extend(source_claims.iter().map(|claim| &claim.node));
            let source_claim_count = source_claim_nodes.len();
            source_claim_nodes.sort_unstable();
            source_claim_nodes.dedup();

            let source_claims_valid = source_set.as_ref().is_none_or(|set| {
                set.envelope == self.envelope
                    && set.is_valid()
                    && source_claim_nodes.len() == source_claim_count
            });

            // Both inputs are already sorted and unique, so equal keys advance both cursors and
            // are emitted once into the comparison. No combined coverage collection is needed.
            let coverage_matches = 'coverage: {
                let mut supplied_index = 0;
                let mut source_index = 0;
                let mut required_index = 0;
                while supplied_index < supplied.len() || source_index < source_claim_nodes.len() {
                    let next = match (
                        supplied.get(supplied_index),
                        source_claim_nodes.get(source_index),
                    ) {
                        (Some(supplied), Some(source)) => match supplied.cmp(source) {
                            core::cmp::Ordering::Less => {
                                supplied_index += 1;
                                *supplied
                            }
                            core::cmp::Ordering::Equal => {
                                supplied_index += 1;
                                source_index += 1;
                                *supplied
                            }
                            core::cmp::Ordering::Greater => {
                                source_index += 1;
                                *source
                            }
                        },
                        (Some(supplied), None) => {
                            supplied_index += 1;
                            *supplied
                        }
                        (None, Some(source)) => {
                            source_index += 1;
                            *source
                        }
                        (None, None) => unreachable!("union loop has a remaining input"),
                    };
                    if required
                        .get(required_index)
                        .is_none_or(|required| *required != next)
                    {
                        break 'coverage false;
                    }
                    required_index += 1;
                }
                required_index == required.len()
            };

            let source_overlap = {
                let mut supplied_index = 0;
                let mut source_index = 0;
                let mut overlaps = false;
                while supplied_index < supplied.len() && source_index < source_claim_nodes.len() {
                    match supplied[supplied_index].cmp(source_claim_nodes[source_index]) {
                        core::cmp::Ordering::Less => supplied_index += 1,
                        core::cmp::Ordering::Greater => source_index += 1,
                        core::cmp::Ordering::Equal => {
                            overlaps = true;
                            break;
                        }
                    }
                }
                overlaps
            };

            let mut observer_pairs =
                Vec::with_capacity(self.observers.len() + bindings.observers.len());
            observer_pairs.extend(
                self.observers
                    .iter()
                    .chain(bindings.observers.iter())
                    .map(|binding| (&binding.node, binding.handle)),
            );
            let valid_observers = observer_pairs.iter().all(|(node, _)| {
                matches!(
                    node,
                    GraphNodeId::TrackStage { .. } | GraphNodeId::Output { .. }
                )
            }) && {
                observer_pairs.sort_unstable();
                observer_pairs.windows(2).all(|pair| pair[0] != pair[1])
            };

            (
                supplied.len() != supplied_count,
                coverage_matches,
                source_overlap,
                source_claims_valid,
                valid_observers,
            )
        };
        if bindings.envelope != self.envelope
            || !coverage_matches
            || duplicate_binding
            || source_overlap
            || !source_claims_valid
            || !valid_observers
        {
            let envelope_mismatch = bindings.envelope != self.envelope;
            let code = if source_set.is_some()
                && (!source_claims_valid || source_overlap || !coverage_matches)
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
        if source_set.as_ref().is_some_and(|set| {
            source_claim_slot_written_earlier(&self.spec, &program, set.claims())
        }) {
            return Err((self, bindings, source_set, "graph.scheduler.source_slot"));
        }
        let planning =
            match runtime::preflight_sequential(&self, &program, &bindings, source_set.as_ref()) {
                Ok(planning) => planning,
                Err(code) => return Err((self, bindings, source_set, code)),
            };
        let envelope = self.envelope;
        let plan_id = self.plan_id;
        let mut plan = self;
        let observers = {
            let mut observers = core::mem::take(&mut plan.observers);
            observers.append(&mut bindings.observers);
            observers
        };
        let executor = GraphExecutor::new(
            plan,
            &program,
            bindings.nodes,
            observers,
            source_set.take(),
            planning,
        );
        let render_plan = PreparedRenderPlan::prepare_with_executor(
            PrepareRenderPlan {
                plan_id,
                envelope,
                scratch: &[],
            },
            Box::new(executor),
        )
        .expect("prevalidated graph plan");
        Ok(render_plan)
    }
}

/// Whether some op scheduled before a source claim's input op writes the claim's buffer
/// (issue #932).
///
/// The copy loop fills every claim's buffer at the top of the block, and the colouring hands a
/// freed slot to the next taker. On a hand-built plan an earlier op -- another claim's input, any
/// other node, or a compensation-delay staging copy -- can own the slot first, die, and leave it to
/// the claim's input, so its write would overwrite the copied words. Bind refuses a claim unless its
/// input op is the first op that writes its buffer, and a claim whose node has no op. `claims` are
/// already validated, so each one names a node of `spec`. One pass over the ops, control plane only.
fn source_claim_slot_written_earlier(
    spec: &GraphSpec,
    program: &program::ExecutionProgram,
    claims: &[GraphSourceInputClaim],
) -> bool {
    let mut first_writer = vec![u32::MAX; program.buffers as usize];
    let mut record = |buffer: program::BufferRef, op: u32| {
        if let Some(first) = first_writer.get_mut(buffer.0 as usize)
            && *first == u32::MAX
        {
            *first = op;
        }
    };
    for (op_index, op) in program.ops.iter().enumerate() {
        let op_index = op_index as u32;
        for input in program.inputs_of(op) {
            if let Some(delay) = input.delay {
                record(delay.staging, op_index);
            }
        }
        if let Some(delay) = op.sidechain.and_then(|side| side.delay) {
            record(delay.staging, op_index);
        }
        record(op.output, op_index);
    }
    claims.iter().any(|claim| {
        let node = program::node_index(spec, &claim.node).expect("validated source claim") as usize;
        match program.node_op[node] {
            Some(op) => first_writer.get(program.node_buffer[node].0 as usize) != Some(&op),
            None => true,
        }
    })
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
    /// Issue #210 phase 2: input-side track alignment, one entry per delayed track.
    pub track_delays: Vec<PreparedTrackDelay>,
    pub effects: Vec<GraphPreparedEffect>,
    /// Issue #140 A: live-control channels, one per driven effect node.
    pub effect_controls: Vec<GraphEffectControlBinding>,
    /// Issue #143 D3: observation taps, one entry per observed effect node.
    pub effect_observations: Vec<GraphEffectObservationBinding>,
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
    /// The heap bytes this value retains (#1074), exclusive of the value itself: every `Box`,
    /// `Vec`, `String` and `Arc` allocation reachable through its fields, a `Vec` or `String` at
    /// its capacity, an `Arc`'s allocation in full whoever else holds it. Whoever boxes the value
    /// adds `size_of_val` for the box. Computed from capacities on the control thread, never on
    /// the render path. Required, so a new owner cannot under-report by default.
    fn retained_bytes(&self) -> u64;
    fn can_prepare_source_seek(&self, _source_index: usize) -> bool {
        false
    }
    fn prepare_source_seek(&mut self, _source_index: usize, _generation: u64, _frame: u64) -> bool {
        false
    }
    /// Apply every admitted plain seek on this set's consumers between blocks (#2213), on the
    /// render owner while render does not run; returns the number of consumers that applied one.
    /// A set with no consumers prepares none.
    fn prepare_admitted_seeks(&mut self) -> usize {
        0
    }
    fn claim_count(&self) -> usize;
    fn begin_block(&mut self, first_sample: u64, frames: u32) -> Result<(), RenderError>;
    fn copy_track_input(
        &mut self,
        claim_index: usize,
        left: &mut [f32],
        right: &mut [f32],
    ) -> Result<(), RenderError>;
    /// Source facts for the graph block most recently admitted by `begin_block`.
    fn observation_validity(&self) -> GraphObservationValidity {
        GraphObservationValidity::CLEAR
    }
    fn copy_after_disarm_telemetry(&self, _output: &mut [u64]) -> usize {
        0
    }
    /// The accepted source frames still in this driver's occupied rings that render never read
    /// (#1699 D1). Control thread only, once the plan is retired: a source the successor carried
    /// left a vacant entry here, so what remains is exactly the removed or redeclared sources'
    /// discarded PCM. The default (no rings) is 0.
    fn unread_source_frames(&self) -> u64 {
        0
    }
    /// Whether [`Self::played_planes`] lends this driver's played block (issue #918).
    ///
    /// Read once, at bind. Only a driver that answers `true` has any claim bound in place, so a
    /// driver that never overrode `played_planes` keeps every claim on the copy: its default
    /// `None` would otherwise read as an underrun on every block and render silence where its
    /// `copy_track_input` wrote audio.
    fn provides_played_planes(&self) -> bool {
        false
    }
    /// Borrow claim `claim_index`'s `(left, right)` planes of the block `begin_block` played, in
    /// place, or `None` when no block was played this quantum for it (underrun, end of region).
    ///
    /// Each plane is exactly one quantum: the played frames, then `+0.0` to the quantum on a short
    /// block. On `Some` the words are the ones `copy_track_input` would have written for the claim,
    /// and on `None` it would have written `+0.0` throughout. The planes stay readable until the
    /// next `&mut self` call; the graph reads them only between `begin_block` and the end of the
    /// same render. Realtime: no allocation, lock or syscall.
    fn played_planes(&self, _claim_index: usize) -> Option<(&[f32], &[f32])> {
        None
    }
    /// Whether every word claim `claim_index`'s planes hold this quantum, as `copy_track_input`
    /// writes them or `played_planes` lends them, is exact `+0.0` (raw bits zero) on both planes
    /// (issue #2133). Asked after `begin_block`, on the render thread: an `O(1)` answer from facts
    /// the driver recorded where its PCM was written, never a pass over the words. The engine's
    /// own ring driver learns it on the control thread as the host's chunk is copied in.
    ///
    /// `false` (the default) is always sound: it only costs the lane packer an opportunity. A
    /// `true` must be exact, and it moves no rendered bit even when it is not, because every skip
    /// keeps its own input test. Realtime: no allocation, lock or syscall.
    fn played_block_silent(&self, _claim_index: usize) -> bool {
        false
    }
    /// This driver as `Any`, so a successor's [`Self::adopt_sources`] can downcast its
    /// predecessor to the concrete type it knows how to take sources from. `None` (the default)
    /// offers nothing.
    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        None
    }
    /// Whether source `source_index` is vacant (bound with no ring, waiting for a carried one),
    /// or `None` when the index is out of range or the driver has no vacancy concept (the
    /// default). Read off the render thread, by [`install_carry_program`].
    fn source_vacancy(&self, _source_index: usize) -> Option<bool> {
        None
    }
    /// Sources bound vacant, counted apart from the resource report (they charge no PCM payload
    /// and no ring overhead).
    fn vacant_source_count(&self) -> usize {
        0
    }
    /// Take sources from the predecessor's driver at the swap block: for each `(successor
    /// index, predecessor index)` move the predecessor's source into this driver's vacant entry,
    /// leaving the predecessor's entry vacant. All or nothing: `false` (the default) moves
    /// nothing. Render-thread code: no allocation, free, lock or syscall, bounded by
    /// `moves.len()`.
    fn adopt_sources(
        &mut self,
        _predecessor: &mut dyn GraphPreparedSourceSetDriver,
        _moves: &[(u32, u32)],
    ) -> bool {
        false
    }
    /// This driver as shared `Any`, so [`Self::can_swap_sources`] can downcast the other driver.
    /// `None` (the default) offers nothing.
    fn as_any(&self) -> Option<&dyn Any> {
        None
    }
    /// Whether source `source_index`'s consumer has begun a block (#1344 D5), or `None` when the
    /// index is out of range, the entry is vacant, or the driver cannot tell (the default). Read
    /// off the render thread.
    fn source_has_begun_block(&self, _source_index: usize) -> Option<bool> {
        None
    }
    /// Whether [`Self::swap_sources`] can exchange every `(this index, other index)` pair (#1344
    /// D4): both entries occupied and of equal channel count, both drivers of one quantum. The
    /// occupied-to-occupied twin of [`Self::adopt_sources`]'s check. Read-only, off the render
    /// thread. `false` (the default) swaps nothing.
    fn can_swap_sources(
        &self,
        _other: &dyn GraphPreparedSourceSetDriver,
        _pairs: &[(u32, u32)],
    ) -> bool {
        false
    }
    /// Exchange each checked pair's two entries whole (#1344 D4): ring, queued blocks, generation,
    /// read position and held seek. All or nothing: it re-checks the pairs and on `false` moves
    /// nothing. Control-thread code, on two plans neither of which render owns; it allocates and
    /// frees nothing.
    fn swap_sources(
        &mut self,
        _other: &mut dyn GraphPreparedSourceSetDriver,
        _pairs: &[(u32, u32)],
    ) -> bool {
        false
    }
    /// The session timeline's sample at `source_read_sample` (#1641 D8), from the driver's
    /// timeline history, or `None` (the default) for a driver with no timeline. Read-only and
    /// bounded.
    fn timeline_at(&self, _source_read_sample: u64) -> Option<i64> {
        None
    }
    /// The node-time mapping of the node block of `frames` samples from the signed source-read
    /// sample `first_source_read_sample` (#1640 D3, #1647a), from the driver's timeline history,
    /// or `None` (the default) for a driver with no timeline. Render reads it once per automated
    /// lane per block, through [`GraphBlockTime::node_span`]. Read-only, allocation-free and
    /// bounded; no input overflows or panics (R17-5).
    fn timeline_block(
        &self,
        _first_source_read_sample: i128,
        _frames: u32,
    ) -> Option<TimelineSpan> {
        None
    }
    /// Whether every ring a warm successor carries can be primed (#1355 D4): the timeline and
    /// every occupied source whose consumer reads `prime_required` would play each of the
    /// `lead_blocks + 1` blocks from `source_read_sample` on from queued PCM, with no command and
    /// no held seek in the window (#1320 D2, #1641 D5). Asked on the **active** plan for a due
    /// `Primed` candidate. Render-thread code: read-only, at most `lead_blocks + 2` loads per
    /// flagged ring, no allocation, lock or syscall. The default `false` cannot check, so a
    /// `Primed` candidate is never adopted over it.
    fn prime_ready(&self, _source_read_sample: u64, _lead_blocks: u32) -> bool {
        false
    }
    /// Replay one block of a warm successor's prime (#1355 D5): advance the timeline and each
    /// successor source of `moves` (the carry program's `(successor, predecessor)` pairs) by the
    /// block that reads from `source_read_sample`, exactly as that block would play, observing no
    /// command (#1320 D3, #1641 D5). The played planes are then lent by
    /// [`Self::played_planes`]. Render-thread code: bounded by `moves.len()`, no allocation,
    /// lock or syscall. `false` (the default, or a move that names no occupied entry) primes
    /// nothing.
    fn prime_block_at(&mut self, _source_read_sample: u64, _moves: &[(u32, u32)]) -> bool {
        false
    }
}

/// The source set as a bank's gather sees it during the unit loop (issue #918): a shared view of
/// the planes `begin_block` played, handed to [`runtime::Runtime::execute`] after the copy loop.
pub(crate) trait GraphSourcePlanes {
    /// [`GraphPreparedSourceSetDriver::played_planes`] after the set's own claim and length
    /// checks.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])>;
    /// [`GraphPreparedSourceSetDriver::played_block_silent`] after the set's own claim check.
    fn played_block_silent(&self, claim_index: usize) -> bool;
}

/// A graph-owned, coordinator-only source-set capability.
///
/// Its claims, envelope, resource report, and driver ownership become immutable once prepared;
/// callers can only move it into a transactional graph bind.
///
/// After a successful bind no op scheduled before a claim's input op writes its buffer: bind
/// refuses any other plan with `graph.scheduler.source_slot` (issue #932), so nothing overwrites a
/// claim's block-top copy before its input op. In particular no two claims share a buffer.
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

    /// Source facts for the graph block most recently admitted by the source driver.
    pub(crate) fn observation_validity(&self) -> GraphObservationValidity {
        self.driver.observation_validity()
    }

    /// Copy bounded render-owner telemetry only after the prepared plan is disarmed.
    pub fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
        self.driver.copy_after_disarm_telemetry(output)
    }

    /// Sources bound vacant: no ring until a [`GraphCarryProgram`] moves one in at the swap
    /// block. They are counted here, apart from [`Self::resource_report`], which charges them
    /// no PCM payload and no ring overhead.
    #[must_use]
    pub fn vacant_source_count(&self) -> usize {
        self.driver.vacant_source_count()
    }
}

// REALTIME_POLICY_BEGIN
impl GraphSourcePlanes for GraphPreparedSourceSet {
    /// The same claim-index check `copy_track_input` makes, then the driver's planes, each held to
    /// the quantum `copy_track_input` holds its destinations to. A plane of any other length is a
    /// driver fault; it is refused as `None`, which the reader serves as silence, rather than
    /// handed to a reader that would index past it.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])> {
        if claim_index >= self.claims.len() {
            return None;
        }
        let quantum = self.envelope.quantum.0 as usize;
        self.driver
            .played_planes(claim_index)
            .filter(|(left, right)| left.len() == quantum && right.len() == quantum)
    }

    fn played_block_silent(&self, claim_index: usize) -> bool {
        claim_index < self.claims.len() && self.driver.played_block_silent(claim_index)
    }
}
// REALTIME_POLICY_END

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
    /// Give the bound processor its node's floored input arrival `a(n)` through
    /// [`GraphRuntimeProcessor::set_input_arrival`] (#1647b); an identity binding has none. Off
    /// the render thread, before bind.
    pub fn set_input_arrival(&mut self, arrival: u64) {
        if let Some(processor) = self.processor.as_mut() {
            processor.set_input_arrival(arrival);
        }
    }
}
pub struct GraphBindingBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub first_sample: u64,
    /// The block's live snapshot `S` (#1504): the revision render read from the running plan's
    /// gate at the start of the block. A processor that drains live values reads under it and
    /// drains once in every block, whether or not its audio is silent. An observation queue is a
    /// FIFO and ignores it. A caller that renders a processor outside a plan names
    /// the snapshot it means; [`LiveSnapshot::ALL`] applies every value written.
    pub live_snapshot: LiveSnapshot,
}
pub trait GraphRuntimeProcessor: Send + Any {
    /// The heap bytes this value retains (#1074), exclusive of the value itself: every `Box`,
    /// `Vec`, `String` and `Arc` allocation reachable through its fields, a `Vec` or `String` at
    /// its capacity, an `Arc`'s allocation in full whoever else holds it. Whoever boxes the value
    /// adds `size_of_val` for the box. Computed from capacities on the control thread, never on
    /// the render path. Required, so a new owner cannot under-report by default.
    fn retained_bytes(&self) -> u64;
    /// Process one block in place.
    ///
    /// # The contract for a node with no graph inputs (issue #218)
    ///
    /// A bound node the graph feeds nothing -- a track input, a host source -- **is** its node's
    /// audio, and the buffer it is handed is *undefined* on entry: it may hold the previous
    /// block's words. Such a processor must write every word of `left` and `right`, including the
    /// silence it emits on an underrun. A processor that leaves the block untouched is asking for
    /// a pass-through, and a pass-through is [`GraphNodeBinding::identity`], never a
    /// do-nothing `process`.
    ///
    /// A bound node that *does* have graph inputs is handed its reduction, exactly as before, and
    /// may read the block it is given.
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError>;

    /// Take the node's floored input arrival `a(n)` (#1647b, R17-4): the scalar oracle's bound
    /// fader owners take it as a builtin bank takes its members' through
    /// [`GraphPreparedBuiltinBankProcessor::set_input_arrivals`]. Called once, off the render
    /// thread, before bind ([`GraphNodeBinding::set_input_arrival`]). The default keeps nothing.
    fn set_input_arrival(&mut self, arrival: u64) {
        let _ = arrival;
    }

    /// Take this block's node-time context (#1647b, R17-4): the executor calls it on a bound node
    /// right before [`Self::process`] of the same block, as it sets a bank's builtin slots before
    /// their chain runs ([`GraphPreparedBuiltinBankProcessor::set_block_time`]). Render-thread
    /// code: allocation-, lock- and syscall-free and bounded. The default keeps nothing.
    fn set_block_time(&mut self, time: &GraphBlockTime<'_>) {
        let _ = time;
    }

    /// Copy this scalar owner's retained response words without advancing render state.
    fn copy_response_snapshot(
        &self,
        _sample_rate_hz: u32,
        _request: ResponseSnapshotRequest<'_>,
    ) -> Result<ResponseSnapshotSummary, ResponseAnalysisError> {
        Err(ResponseAnalysisError::UnsupportedCapability)
    }

    /// Whether this prepared scalar owner declares a response provider.
    fn response_snapshot_declared(&self) -> bool {
        false
    }
    /// The static native identity of this scalar response provider, when declared.
    fn response_snapshot_native_id(&self) -> Option<&'static str> {
        None
    }

    /// This bound processor's channel-symmetry witness for the track it renders.
    ///
    /// The scalar-tail sibling of `GraphPreparedBuiltinBankProcessor::lane_symmetry`, defaulted to
    /// declining for the same reason. A host-supplied processor is opaque to the engine and
    /// therefore declines: nothing has compared its two channels' words.
    fn channel_symmetry(&self) -> ChannelSymmetryWitness {
        ChannelSymmetryWitness::DECLINED
    }
}
/// Immutable post-node observation input. Observers cannot alter graph audio.
pub struct GraphObservationBlock<'a> {
    pub left: &'a [f32],
    pub right: &'a [f32],
    pub first_sample: u64,
}

/// Source facts covering the graph block offered to an observer.
///
/// The source layer supplies these facts at the same exclusive render boundary as the copied
/// graph block. A graph-wide flag is intentional: source-to-track attribution is not available at
/// this seam, so consumers must not present it as a target-specific guarantee.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphObservationValidity {
    /// At least one source emitted an in-region underrun during this block.
    pub source_underrun: bool,
    /// At least one source applied a new generation at this block boundary.
    pub source_generation_changed: bool,
}

impl GraphObservationValidity {
    /// No source invalidity or underrun facts were observed.
    pub const CLEAR: Self = Self {
        source_underrun: false,
        source_generation_changed: false,
    };
}

/// Immutable final bank output, offered at the same post-node observation point.
#[derive(Clone, Copy)]
pub struct GraphResidentObservationBlock<'a> {
    pub lane: rack::ResidentOutputLane<'a>,
    pub first_sample: u64,
    /// Source facts for the graph block containing this resident lane.
    pub validity: GraphObservationValidity,
    /// This lane's `[left, right]` sample peak over this block, computed once for the whole bank
    /// (issue #943): the maximum of `+0.0` and every **sanitized** magnitude of the lane's final
    /// words, where sanitized is the meter's `normal_or_zero` (finite and not subnormal, else
    /// `+0.0`). Left is read from the left plane and right from the right plane.
    ///
    /// `Some` only when this unit's bank pass ran this block, which it does only for a unit bound
    /// with at least one final-slot observer whose [`GraphRuntimeObserver::accepts_sample_peak`]
    /// is true, or whose full meter pass ran this block ([`Self::meter`], issue #950), which carries
    /// the same peak. It is then offered to every observer of the lane, so an observer that did not
    /// ask for it must ignore it.
    pub sample_peak: Option<[f32; 2]>,
    /// This lane's full meter pass over this block, computed once for the whole bank (issue #950).
    ///
    /// `Some` only for a final-slot member of a unit whose full pass ran this block, which it does
    /// only for a unit bound with at least one final-slot observer whose
    /// [`GraphRuntimeObserver::accepts_banked_meter`] is true, and only when some final lane's
    /// observer answered [`GraphRuntimeObserver::banked_meter_seed`]. It is offered to every
    /// observer of the lane; an observer uses it only if it accepted the pass, and only if
    /// [`GraphBankedMeterLane::energy_seed`] is its own state.
    pub meter: Option<GraphBankedMeterLane>,
}

/// One lane's full meter pass over one final resident block (issue #950): the sanitized sample
/// peak, the clipped and sanitized counts and the energy sum, per channel as `[left, right]`, left
/// read from the left plane and right from the right plane.
///
/// "Sanitized" is the meter's `normal_or_zero`: a finite, non-subnormal sample is kept and anything
/// else becomes `+0.0` before it is measured. The pass is `lane::kernels::builtins::meter_block` at
/// the bank's width.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GraphBankedMeterLane {
    /// The maximum of `+0.0` and every sanitized magnitude of the block.
    pub sample_peak: [f32; 2],
    /// How many sanitized magnitudes of the block are `>= 1.0`.
    pub clipped: [u32; 2],
    /// How many of the block's words the sanitization replaced: NaN, `±inf` and nonzero
    /// subnormals.
    pub sanitized: [u32; 2],
    /// The energy the pass started from: the first [`GraphRuntimeObserver::banked_meter_seed`]
    /// answer in binding order among this lane's observers, or `+0.0` when none answered.
    pub energy_seed: [f64; 2],
    /// `energy_seed` plus the square of every sanitized sample, widened to `f64` and added one
    /// sample at a time in frame order: the sum the answering observer's own loop would reach.
    pub energy: [f64; 2],
    /// Held recurrence, when a held-only seed search answered for this lane (issue #951).
    pub held: Option<lane::kernels::builtins::MeterHeldBanked>,
}
/// A bounded observer invoked after its node has completed.
pub trait GraphRuntimeObserver: Send {
    /// The heap bytes this value retains (#1074), exclusive of the value itself: every `Box`,
    /// `Vec`, `String` and `Arc` allocation reachable through its fields, a `Vec` or `String` at
    /// its capacity, an `Arc`'s allocation in full whoever else holds it. Whoever boxes the value
    /// adds `size_of_val` for the box. Computed from capacities on the control thread, never on
    /// the render path. Required, so a new owner cannot under-report by default.
    fn retained_bytes(&self) -> u64;
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError>;

    /// Observe one planar block with source validity facts from the same render boundary.
    /// Existing observers ignore the additional facts by default.
    fn observe_with_validity(
        &mut self,
        block: GraphObservationBlock<'_>,
        _validity: GraphObservationValidity,
    ) -> Result<(), RenderError> {
        self.observe(block)
    }

    /// `None` declines without mutation and requests the ordinary planar observation.
    /// Both `Some` results accept once; an accepted error propagates without retry.
    fn observe_resident(
        &mut self,
        _block: GraphResidentObservationBlock<'_>,
    ) -> Option<Result<(), RenderError>> {
        None
    }

    /// Whether this observer consumes [`GraphResidentObservationBlock::sample_peak`] (issue #943).
    ///
    /// Read once, when the runtime is bound, and never on the render path: a bank runs its
    /// sample-peak pass only if some observer of its final slot answers `true` here. The default
    /// declines, so an observer that would not use the peak never pays for it.
    fn accepts_sample_peak(&self) -> bool {
        false
    }

    /// Whether this observer consumes [`GraphResidentObservationBlock::meter`] (issue #950).
    ///
    /// Read once, when the runtime is bound, and never on the render path: a bank runs its full
    /// meter pass only if some observer of its final slot answers `true` here. The default
    /// declines, so an observer that would not use the pass never pays for it.
    fn accepts_banked_meter(&self) -> bool {
        false
    }

    /// The `[left, right]` energy this observer's own loop would start a `frames`-frame block at
    /// `first_sample` from, or `None` to decline the full meter pass for this block (issue #950).
    ///
    /// Called at render, after the unit executed and before any observer of the unit runs, on the
    /// observers of each final-slot member in binding order until one answers. It must be pure --
    /// a read of this observer's own state that changes nothing -- bounded, and allocation-free:
    /// an observer that fails later in the same block must leave every other observer as it was.
    /// The answer seeds the pass; it does not commit anything. An observer that accepts the result
    /// must check [`GraphBankedMeterLane::energy_seed`] against its own state before using it.
    fn banked_meter_seed(&self, _first_sample: u64, _frames: u32) -> Option<[f64; 2]> {
        None
    }

    /// Independent held-state seed search (issue #951), with the same purity, boundedness and
    /// allocation-free rules as `banked_meter_seed`. A held-only meter must not take an energy
    /// seed from another meter. Consumers validate every state/configuration word on both channels.
    fn banked_held_seed(
        &self,
        _first_sample: u64,
        _frames: u32,
    ) -> Option<[lane::kernels::builtins::MeterHeldSeed; 2]> {
        None
    }

    /// Invalidate a pending capture after a failed graph render.
    ///
    /// The default keeps existing meter and observer implementations unchanged.
    fn invalidate(&mut self) {}

    /// Invalidate a capture after a graph render failed at this block's sample boundary.
    ///
    /// Observers that can complete during a block may use the boundary to discard a result
    /// completed by the failed block while preserving an older completed result.
    #[doc(hidden)]
    fn invalidate_after_failure(&mut self, _failed_sample: u64) {
        self.invalidate();
    }

    /// Whether this observer wants [`Self::block_end`] (issue #1609).
    ///
    /// Read once, when the binding is built, and never on the render path: the runtime keeps a
    /// bind-time list of the units that hold an accepting observer and calls only those
    /// observers. The default declines, so an observer that does not use the hook pays nothing.
    fn accepts_block_end(&self) -> bool {
        false
    }

    /// Called once after a block rendered without failure, on every observer that accepted it,
    /// in binding order (issue #1609).
    ///
    /// An observer that completes a result in the middle of a block holds it until here, so a
    /// reader on another thread never takes a result from a block that later fails: a failed
    /// block calls [`Self::invalidate_after_failure`] and never this. It must be bounded and
    /// allocation-free.
    fn block_end(&mut self) {}

    /// The kind of host reader this observer's state belongs to, or `None` (the default) for an
    /// observer whose state a plan swap never carries (#1327 D1). Read off the render thread,
    /// when the plan's observer rows are recorded.
    fn carry_kind(&self) -> Option<ObserverKind> {
        None
    }

    /// Name this observer's state by `handle`, the handle of the binding that now holds it
    /// (#1327 D2). Called in the swap block on both observers of a carried pair after their
    /// boxes were exchanged, since a snapshot carries the handle of the binding it came from.
    /// The default ignores it.
    ///
    /// **Render-thread code.** A scalar write: no allocation, free, lock or syscall.
    fn adopt_binding_handle(&mut self, _handle: u64) {}

    /// Where this observer's open window sits on the sample grid (#1327 D6), or `None` (the
    /// default) for an observer with no window.
    ///
    /// **Render-thread code.** A scalar read: no allocation, free, lock or syscall.
    fn window_position(&self) -> Option<ObserverWindow> {
        None
    }

    /// Open this observer's first window at `position` (#1327 D6), as an observer that saw
    /// silence since that window opened would hold it. `false` (the default) declines: the
    /// observer has no window, or has already observed.
    ///
    /// **Render-thread code.** Scalar writes: no allocation, free, lock or syscall.
    fn join_window(&mut self, _position: ObserverWindow) -> bool {
        false
    }
}

/// Where an observer's open window sits on the sample grid (#1327 D6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ObserverWindow {
    /// First sample of the open window, or `None` before the first observation.
    pub start: Option<u64>,
    /// Frames already in the open window.
    pub frames: u32,
    /// Sequence number of the snapshot the open window will publish.
    pub sequence: u64,
}
/// One immutable prepared observer binding, ordered by its stable meter handle.
pub struct GraphNodeObserverBinding {
    pub node: GraphNodeId,
    pub handle: u64,
    observer: Box<dyn GraphRuntimeObserver>,
    /// [`GraphRuntimeObserver::accepts_block_end`], read once here and never at render.
    block_end: bool,
    /// Whether this is a strip meter binding (#882): its observer's
    /// [`GraphRuntimeObserver::carry_kind`] is [`ObserverKind::Meter`], read once here. Render
    /// skips such a binding in a block whose strip meter level is off.
    meter: bool,
}
impl GraphNodeObserverBinding {
    pub fn new(node: GraphNodeId, handle: u64, observer: Box<dyn GraphRuntimeObserver>) -> Self {
        let block_end = observer.accepts_block_end();
        let meter = observer.carry_kind() == Some(ObserverKind::Meter);
        Self {
            node,
            handle,
            observer,
            block_end,
            meter,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedRoute {
    pub node: GraphNodeId,
    pub transform: RouteTransform,
    pub gate: RouteGate,
}

/// The bytes a bind allocates for its route-activity table when some prepared route's gate
/// silences, bounded from `routes` prepared routes over a graph of `nodes` nodes (issue #1217 D6);
/// `None` on overflow. A plan with no silencing gate builds no table and allocates none of this,
/// unless route controls are attached, whose charge states it ([`route_control_resources`]).
///
/// The bound is `size_of::<RouteActivity>() + routes * size_of::<bool>() + nodes *
/// size_of::<u32>() + routes * (size_of::<(u32, u32)>() + size_of::<RouteInput>())`: the boxed
/// table, one activity bit per route, one entry per unit (at most one per node), and at most one
/// destination range and one route input per route.
#[must_use]
pub fn route_activity_bound_bytes(routes: u64, nodes: u64) -> Option<u64> {
    runtime::route_activity_bound_bytes(routes, nodes)
}

/// One strip's declared input-side time alignment (#210 phase 2; submixes since #1201).
///
/// On a track it delays the source (`runtime::NodeKind::TrackDelay`); on a submix it delays the
/// summed input, after the reduction (`runtime::NodeKind::SumDelay`).
///
/// Emitted by the compiler **only** for a strip that declared a nonzero delay on at least one lane,
/// so an undelayed session carries an empty vector and lowers to exactly the program it lowered to
/// before this feature existed.
///
/// This is not latency and never becomes latency: it contributes nothing to `GraphNode.latency`,
/// nothing to `RouteTiming`, and nothing to `inserted_delays`. See `runtime::TrackDelayLine`.
///
/// Unversioned by #215's ruling: pre-launch internal implementation types carry no `V1` suffix.
/// It sits beside `PreparedRoute`, which is already spelled that way.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedTrackDelay {
    /// The `TrackStage::Input` node this delay is applied at: a track's or a submix's.
    pub node: GraphNodeId,
    /// `builtins.left.delay_samples`.
    pub left_samples: u32,
    /// `builtins.right.delay_samples`.
    pub right_samples: u32,
}

/// The sequential executor: one coloured arena, one pass over the lowered ops.
///
/// It is a *driver* over [`runtime`], not a second implementation of anything: node semantics,
/// reductions, routes, delays and banks all live there and the native executor calls the same
/// functions (#98 F7).
struct GraphExecutor {
    runtime: runtime::Runtime,
    sample_rate_hz: u32,
    source_set: Option<GraphPreparedSourceSet>,
    /// `(claim index, arena buffer)` for every track input the coordinator's source set copies
    /// into the arena. A claim bound in place (issue #918, `Runtime::source_in_place`) is not
    /// here: its bank reads the played block instead.
    source_input_buffers: Box<[(usize, u32)]>,
    /// Issue #936: the units the render loop dispatches, ascending. Every unit is here but an
    /// inert one ([`runtime::Runtime::unit_inert`]): a plain source input that nothing observes,
    /// whose `execute` and `observe_unit` would both return before touching anything. Built once
    /// at bind; the inert units stay in `runtime.units`, so the census, `unit_eligibility` and
    /// every walk over the units still see them.
    active_units: Box<[u32]>,
    /// Process-unique, nonzero identity taken at bind ([`plan_identity`]): what a successor's
    /// [`GraphCarryProgram::predecessor`] names. Every host passes the same `plan_id`, so the
    /// plan id cannot tell two plans apart.
    identity: u64,
    /// The hand-over a successor runs at its swap block, installed before publication.
    carry: Option<GraphCarryProgram>,
    /// The builtin-input section of the hand-over (issue #1276 D6), sorted by predecessor
    /// location; empty (and never allocated) when no strip input section carries.
    carry_inputs: Box<[GraphLaneMove]>,
    /// The strip section of the hand-over (issue #1277 D8), sorted by predecessor location and
    /// owner; empty (and never allocated) when no strip stage carries.
    carry_strips: Box<[GraphStripMove]>,
    /// The console-effect section of the hand-over (issue #1279 D3), sorted by predecessor
    /// location; empty (and never allocated) when no effect lane carries.
    carry_effects: Box<[GraphEffectMove]>,
    /// The one payload scratch the effect section moves every lane through (issue #1279 D3):
    /// preallocated at the largest carried payload, and empty when no effect lane carries.
    carry_scratch: Box<[u8]>,
    /// The first sample of the block after the last one this plan rendered: where its next block
    /// would apply an `Observe` record still queued for a per-node effect, and so where a swap's
    /// carry applies it (#1281; a bank stage keeps its own, #1280 D3). Zero until a block renders.
    next_block_sample: u64,
    /// How far this plan reads its sources ahead of its render clock (#1396 D1): the source
    /// section's block entry reads from `time.absolute_sample + source_read_offset`. Set on the
    /// control thread before publication.
    source_read_offset: u64,
    /// Restores a swap's carry refused on a per-node successor instance, which no bank stage
    /// counts (#1281, #1279 D6). Saturating; read after render.
    node_carry_refused: u64,
    /// The observer section of the hand-over (#1327 D2, D3): `None` when no observer carries.
    observer_carry: Option<Box<ObserverCarryProgram>>,
    /// The compensation-line section of the hand-over (#1283 D1), sorted by predecessor line;
    /// empty (and never allocated) when no line carries.
    carry_delays: Box<[GraphDelayMove]>,
    /// The identity of every compensation line, read once by [`take_compensation_lines`] right
    /// after bind and empty afterwards: a plan retains none of it.
    compensation_lines: Vec<GraphDelayLine>,
    /// The strip-delay section of the hand-over (#1284 D1), sorted by predecessor line; empty
    /// (and never allocated) when no strip delay line carries.
    carry_strip_delays: Box<[GraphStripDelayMove]>,
    /// The live-send section of the hand-over (#1284 D2), sorted by predecessor unit; empty (and
    /// never allocated) when no live route carries.
    carry_routes: Box<[runtime::RouteCarryStep]>,
    /// The prime section of a warm successor's hand-over (#1355 D5): the blocks it replays, 0
    /// for every other plan, and its source-claim and sidechain fills; empty (and never
    /// allocated) without a prime.
    prime_blocks: u32,
    prime_claims: Box<[ClaimFillRow]>,
    prime_sidechains: Box<[SidechainFillRow]>,
    /// Set by a carried hand-over of a plan with a prime: the next block replays the prime before
    /// its sources begin, then clears it.
    prime_pending: bool,
    /// The side table of the moves whose predecessor row is a following route (#1652 D3,
    /// MINOR-3): the comparisons a six-word successor of a following predecessor leaves for its
    /// first block. `None` (never allocated) when no such move exists, so a session without
    /// stored mute automation keeps its bytes.
    route_checks: Option<Box<RouteCarryChecks>>,
    /// The identity of every strip delay line and live route, read once by
    /// [`take_strip_delay_lines`] and [`take_live_routes`] right after bind and empty afterwards:
    /// a plan retains none of it.
    bind_identities: runtime::BindIdentities,
    /// The live snapshot `S` of the block about to render (#1504 D1): stored by
    /// [`PreparedPlanExecutor::begin_live_block`] and taken by `render`, so a block that renders
    /// without its own snapshot is refused and never reads the previous block's.
    live_snapshot: Option<LiveSnapshot>,
    /// The plan's jump-length cell reader (#1639 D2), attached after bind by
    /// [`attach_jump_lengths`]; `None` for a plan with no cell.
    jump_lengths: Option<Box<JumpLengthReader>>,
    /// The fade-in arms of the strips this plan adds during playback (issue #1288 D3), installed
    /// before publication by [`install_fade_arms`]; empty (and never allocated) when none.
    fade_arms: Box<[FadeArm]>,
    /// How many of [`Self::fade_arms`] have not fired: render tests this one integer per block.
    fade_arms_pending: u32,
    /// The fade-arm section of the hand-over (issue #1392 D3); empty (and never allocated) when
    /// no arm carries.
    fade_arm_carry: Box<[GraphFadeArmPair]>,
    /// One flag per entry of [`Self::fade_arms`], set by render when the entry is `Done` (issue
    /// #1392, root ruling of attempt 2), shared with the plan's inventory so a successor drops a
    /// fired arm's row instead of carrying it.
    fade_fired: GraphFadeArmWatch,
    /// The session's stored-automation program (#1074), attached at bind by
    /// [`attach_automation_program`] so the plan's retained-bytes walk charges its one shared
    /// allocation once, whatever number of stages and routes clone it. Control-only: render never
    /// reads it. `None` for a plan without stored automation.
    automation_program: Option<std::sync::Arc<automation::Program>>,
    /// The strip meter level of the next block (#882), stored by
    /// [`PreparedPlanExecutor::set_strip_meter_level`]; `true` until a host sets it. Render skips
    /// every strip meter binding, and the bank meter passes only they read, while it is `false`.
    strip_meters: bool,
}

/// Next graph executor identity; identities start at 1, so zero never names a plan.
static NEXT_GRAPH_IDENTITY: core::sync::atomic::AtomicU64 = core::sync::atomic::AtomicU64::new(1);

/// Take a fresh process-unique identity, at bind (off the render thread).
fn next_graph_identity() -> u64 {
    NEXT_GRAPH_IDENTITY.fetch_add(1, core::sync::atomic::Ordering::Relaxed)
}

/// What a successor graph plan takes from the plan it displaces, at the swap block.
///
/// `sources` lists `(successor source index, predecessor source index)` pairs: each moves the
/// predecessor's source consumer (ring, queued blocks, generation, read position and pending
/// generation change) into the successor's vacant entry. The move happens only when the
/// displaced plan is the graph plan whose identity is `predecessor`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphCarryProgram {
    /// [`plan_identity`] of the plan this successor is built to displace.
    pub predecessor: u64,
    /// `(successor source index, predecessor source index)` pairs.
    pub sources: Box<[(u32, u32)]>,
    /// Whether the source set's timeline carries (#1641 D3): set on every host successor, so the
    /// driver's [`GraphPreparedSourceSetDriver::adopt_sources`] runs even when no source moves.
    pub timeline: bool,
}

impl GraphCarryProgram {
    /// Heap bytes the installed program retains in its plan (its move table), charged to the
    /// plan by [`carry_program_retained_bytes`].
    #[must_use]
    pub fn retained_bytes(&self) -> u64 {
        u64::try_from(core::mem::size_of_val::<[(u32, u32)]>(&self.sources)).unwrap_or(u64::MAX)
    }
}

/// Refusal to install a [`GraphCarryProgram`], off the render thread.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphCarryInstallError {
    /// The plan was not bound by this crate.
    NotAGraphPlan,
    /// The plan has no source set to carry sources into.
    NoSourceSet,
    /// A successor source index is beyond the plan's source set.
    SourceIndexOutOfRange,
    /// A successor source index already owns a ring.
    SourceNotVacant,
    /// A successor or predecessor source index appears twice.
    DuplicateSourceIndex,
    /// A builtin-input carry was installed before the plan's [`GraphCarryProgram`], which names
    /// the predecessor.
    NoCarryProgram,
    /// A successor location is not a populated lane of a builtin bank whose input lanes carry.
    InputLaneOutOfRange,
    /// A successor or predecessor input lane appears twice.
    DuplicateInputLane,
    /// A strip carry was installed before the plan's [`GraphCarryProgram`].
    NoStripCarryProgram,
    /// A successor location is not a populated lane of a builtin bank that holds the owner's
    /// state.
    StripLaneOutOfRange,
    /// A successor or predecessor strip lane appears twice for one owner.
    DuplicateStripLane,
    /// An effect carry was installed before the plan's [`GraphCarryProgram`].
    NoEffectCarryProgram,
    /// A successor location is not a populated lane of an effect bank whose lanes carry.
    EffectLaneOutOfRange,
    /// A successor or predecessor effect lane appears twice.
    DuplicateEffectLane,
    /// An observer carry was installed before the plan's [`GraphCarryProgram`].
    NoObserverCarryProgram,
    /// A successor location is not an observer of the kind the move or join names.
    ObserverOutOfRange,
    /// A successor or predecessor observer appears twice, or a join's fresh observer is carried.
    DuplicateObserver,
    /// A compensation-line carry was installed before the plan's [`GraphCarryProgram`].
    NoDelayCarryProgram,
    /// A successor line is not a compensation line of the plan.
    DelayLineOutOfRange,
    /// A successor or predecessor compensation line appears twice.
    DuplicateDelayLine,
    /// A strip-delay carry was installed before the plan's [`GraphCarryProgram`].
    NoStripDelayCarryProgram,
    /// A successor line is not a strip delay line of the plan, or names no lane.
    StripDelayLineOutOfRange,
    /// A successor or predecessor strip delay line appears twice.
    DuplicateStripDelayLine,
    /// A live-send carry was installed before the plan's [`GraphCarryProgram`].
    NoRouteCarryProgram,
    /// A successor unit is not a live route of the plan.
    RouteOutOfRange,
    /// A successor or predecessor live route appears twice.
    DuplicateRoute,
    /// A prime was installed before the plan's [`GraphCarryProgram`], with no lead block, or on
    /// a plan with no source set (#1355 D5).
    NoPrimeCarryProgram,
    /// A fill names a line that is not one of the plan's, a node that is not a source claim of
    /// the plan, or no lane (#1355 D5).
    PrimeFillOutOfRange,
    /// A fill's line appears twice (#1355 D5).
    DuplicatePrimeFill,
}

/// Where one builtin bank lane sits in a bound graph plan: its runtime unit, its bank chain slot
/// and its lane (issue #1276 D6). Valid only for the plan it was read from.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GraphLaneLocation {
    /// The runtime unit (a bank chain).
    pub unit: u32,
    /// The chain slot.
    pub slot: u16,
    /// The bank lane.
    pub lane: u16,
}

/// One strip input section a successor takes from its predecessor at the swap block: the lane
/// state of `predecessor` in the displaced plan is copied into `successor` (issue #1276 D6).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphLaneMove {
    /// The lane in the successor, the plan the move is installed in.
    pub successor: GraphLaneLocation,
    /// The lane in the plan the successor displaces.
    pub predecessor: GraphLaneLocation,
}

/// Every banked strip input section of a bound graph plan, as `(strip ID, location)` pairs in
/// program order (issue #1276 D6); `None` for any other plan. Read off the render thread, before
/// publication: a successor's preparation joins its own lanes with its predecessor's (recorded
/// when that plan was prepared) by strip ID, never by location.
pub fn builtin_input_lanes(
    plan: &mut PreparedRenderPlan,
) -> Option<Vec<(Box<str>, GraphLaneLocation)>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(executor.runtime.builtin_input_lanes())
}

/// Install the builtin-input section of the hand-over this graph plan runs when it displaces its
/// predecessor (issue #1276 D6), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor location is not a populated lane of a builtin bank whose input lanes
/// carry, or a successor or predecessor location repeats. Predecessor locations are checked
/// against the predecessor at the swap block, where one that does not resolve refuses the whole
/// hand-over. The moves are kept sorted by predecessor location, so the swap block drains each
/// predecessor bank once.
pub fn install_builtin_input_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphLaneMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoCarryProgram);
    }
    for (index, step) in moves.iter().enumerate() {
        if !executor.runtime.has_input_lane(step.successor) {
            return Err(GraphCarryInstallError::InputLaneOutOfRange);
        }
        if moves[..index].iter().any(|earlier| {
            earlier.successor == step.successor || earlier.predecessor == step.predecessor
        }) {
            return Err(GraphCarryInstallError::DuplicateInputLane);
        }
    }
    moves.sort_unstable_by_key(|step| step.predecessor);
    executor.carry_inputs = moves.into_boxed_slice();
    Ok(())
}

/// One strip stage a successor takes from its predecessor at the swap block: `owner`'s lane
/// state at `predecessor` in the displaced plan is copied into `successor` (issue #1277 D8).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphStripMove {
    /// Whose state moves.
    pub owner: BuiltinStripOwner,
    /// The lane in the successor, the plan the move is installed in.
    pub successor: GraphLaneLocation,
    /// The lane in the plan the successor displaces.
    pub predecessor: GraphLaneLocation,
    /// The fader stage's automated cells whose stored automation table the successor's model
    /// keeps bit for bit from the predecessor's (#1648 D2): the fader cells (bit 0 left, bit 1
    /// right) and the mute cells (#1651 T4: bit 2 left, bit 3 right). Such a cell's carried event
    /// state continues verbatim, and every other carried cell is repositioned in its own table at
    /// adoption. Graph passes the byte to the stage unchanged. `0` for a matrix move.
    pub continued_cells: u8,
}

/// One strip stage owner's lane in a bound graph plan (issue #1277 D1).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphStripLane {
    /// Whose lane it is.
    pub owner: BuiltinStripOwner,
    /// What kind of bank holds it.
    pub stage: BuiltinStripStage,
    /// Where it sits.
    pub location: GraphLaneLocation,
}

/// Every strip stage lane of a bound graph plan, as `(strip ID, lane)` pairs in program order
/// (issue #1277 D1); `None` for any other plan. A strip's fader and matrix lanes are listed
/// separately. Read off the render thread, before publication: a successor's preparation joins
/// its own lanes with its predecessor's (recorded when that plan was prepared) by strip ID, never
/// by location.
pub fn builtin_strip_lanes(
    plan: &mut PreparedRenderPlan,
) -> Option<Vec<(Box<str>, GraphStripLane)>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(executor.runtime.builtin_strip_lanes())
}

/// Install the strip section of the hand-over this graph plan runs when it displaces its
/// predecessor (issue #1277 D8), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor location is not a populated lane of a bank that holds the move's owner,
/// or a successor or predecessor location repeats for one owner. Predecessor locations are
/// checked against the predecessor at the swap block, where one that does not resolve refuses the
/// whole hand-over. The moves are kept sorted by predecessor location, so the swap block applies
/// each predecessor bank's pending values once.
pub fn install_builtin_strip_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphStripMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoStripCarryProgram);
    }
    for (index, step) in moves.iter().enumerate() {
        if !executor.runtime.has_strip_lane(step.owner, step.successor) {
            return Err(GraphCarryInstallError::StripLaneOutOfRange);
        }
        if moves[..index].iter().any(|earlier| {
            earlier.owner == step.owner
                && (earlier.successor == step.successor || earlier.predecessor == step.predecessor)
        }) {
            return Err(GraphCarryInstallError::DuplicateStripLane);
        }
    }
    moves.sort_unstable_by_key(|step| (step.predecessor, step.owner));
    executor.carry_strips = moves.into_boxed_slice();
    Ok(())
}

/// One strip a plan swap adds during playback, whose fader lane fades in from its first played
/// block (issue #1288 D1-D5), or one route a swap adds, which fades in the same way (#1363 D3).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphFadeArm {
    /// What fades in: the strip's fader lane, or the route's live route op, in the plan the arm is
    /// installed in.
    pub target: GraphFadeTarget,
    /// The strip's input node when it is a track that reads a source claim; `None` for a submix
    /// (or a track the source set does not claim), whose fade waits for no source. A route's is
    /// its source strip's when that strip is added too (#1363 D1 (c)), else `None`.
    pub source: Option<GraphNodeId>,
    /// `D`: the samples from the strip's input to its fader (for a route: to the route's own op)
    /// that start at rest (latent nodes and compensation lines), which emit zeros once the source
    /// plays.
    pub delay_samples: u32,
    /// The fade's length: the session mute ramp.
    pub ramp_samples: u32,
    /// The samples after the fade's end before its entry settles ([`GraphFadeArmWatch::settled`]):
    /// a fading copy's destination-edge line, which still delivers the fade (#1953b); 0 for every
    /// other arm.
    pub tail_samples: u32,
}

/// What one fade arm fades in or out (issues #1288 D2, #1363 D3, #1953b).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphFadeTarget {
    /// A strip's fader lane, a populated lane of a fader bank.
    Fader(GraphLaneLocation),
    /// A route's op, the runtime unit of a live route of either kind (a six-word one with a lane
    /// or a transition route, #1363 D2; a following one, #2011), as [`GraphLiveRoute::unit`]
    /// names it.
    Route(u32),
    /// A fading copy's op (#1953b): the runtime unit of a transition route that a successor
    /// compiles for a lane-less route it removes, re-points or (#2077) keeps at other values, at
    /// the predecessor's coefficients.
    /// It fades *out*, to `[+0.0; 4]` and muted, over the arm's ramp.
    RouteOut(u32),
}

/// Where one installed fade arm is (issue #1288 D3).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FadeArmState {
    /// The source has not played a block in this plan yet.
    Waiting,
    /// The fade fires at the first block that starts at or after this sample.
    FireAt(u64),
    /// Fired; the entry settles at the first block that starts at or after this sample: the
    /// fire's block plus the ramp and the arm's tail (#1953b).
    Settling(u64),
    /// Fired and settled.
    Done,
}

impl FadeArmState {
    /// Whether the fade has fired: settling or done.
    const fn fired(self) -> bool {
        matches!(self, Self::Settling(_) | Self::Done)
    }
}

/// One installed fade arm (issue #1288 D3): plain data, sized at installation.
#[derive(Clone, Copy, Debug)]
struct FadeArm {
    target: GraphFadeTarget,
    /// The source-set claim index the fade waits for; `None` fires on the first block.
    claim: Option<usize>,
    delay_samples: u32,
    ramp_samples: u32,
    tail_samples: u32,
    state: FadeArmState,
}

/// The bytes [`install_fade_arms`] and [`install_fade_arm_carry`] retain for a table of `arms`
/// arms and `pairs` carried ones, for the plan's resource charge (issues #1288 D7, #1392 D8);
/// `None` on overflow.
#[must_use]
pub fn fade_arm_table_bytes(arms: usize, pairs: usize) -> Option<u64> {
    arms.checked_mul(
        core::mem::size_of::<FadeArm>() + core::mem::size_of::<core::sync::atomic::AtomicU8>(),
    )
    .and_then(|bytes| {
        pairs
            .checked_mul(core::mem::size_of::<GraphFadeArmPair>())
            .and_then(|pair_bytes| bytes.checked_add(pair_bytes))
    })
    // The watch's `Arc` header (issue #1392).
    .and_then(|bytes| bytes.checked_add(if arms == 0 { 0 } else { WATCH_HEADER_BYTES }))
    .and_then(|bytes| u64::try_from(bytes).ok())
}

/// Arm the fade-in of each strip in `arms` and install the table this plan fires them from
/// (issue #1288 D2, D3), off the render thread, before publication; it replaces any earlier
/// table. Each lane is armed now, so the plan's first block already renders it muted. A strip
/// the session mutes, on one channel or both, arms too and gets an entry (#2080): its fire goes
/// to its prepared mute and ramps nowhere unless a record before the fire unmutes it, as a
/// muted route's (#2069). Returns, for each arm in order, the index of its entry in the table, or
/// `None` when it armed nothing: the index a successor's [`GraphFadeArmPair::predecessor`] names
/// (#1392); and the watch render sets each entry's flag in when it fires.
///
/// A source node that is not one of the source set's claims, and every source of a driver that
/// does not lend its played planes, waits for nothing: that fade fires at the first block.
///
/// A route target (#1363 D3) arms its live route op, of either kind: it waits settled at
/// `[+0.0; 4]`, muted, and fires over `ramp_samples`, a six-word route to its prepared target and
/// a following route (#2011) to each column's target composed with the curve it hears at the
/// fire; a record (or a curve event) it reads before the fire retargets the fire only. A route
/// the session mutes arms too and gets an entry (#2069): its fire goes to its prepared mute and
/// ramps nowhere unless a record before the fire unmutes it.
///
/// Refused, arming nothing, when the plan is not a graph plan, a fader location is not a
/// populated lane of a fader bank, a route unit is not a live route's, or a target repeats.
pub fn install_fade_arms(
    plan: &mut PreparedRenderPlan,
    arms: Vec<GraphFadeArm>,
) -> Result<(Vec<Option<u32>>, GraphFadeArmWatch), GraphCarryInstallError> {
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?
        .install_fade_arms(arms)
}

impl GraphExecutor {
    /// [`install_fade_arms`] on a bound executor.
    fn install_fade_arms(
        &mut self,
        arms: Vec<GraphFadeArm>,
    ) -> Result<(Vec<Option<u32>>, GraphFadeArmWatch), GraphCarryInstallError> {
        for (index, arm) in arms.iter().enumerate() {
            let exists = match arm.target {
                GraphFadeTarget::Fader(fader) => {
                    self.runtime.has_strip_lane(BuiltinStripOwner::Fader, fader)
                }
                GraphFadeTarget::Route(unit) | GraphFadeTarget::RouteOut(unit) => {
                    self.runtime.has_live_route(unit)
                }
            };
            if !exists {
                return Err(GraphCarryInstallError::StripLaneOutOfRange);
            }
            if arms[..index]
                .iter()
                .any(|earlier| earlier.target == arm.target)
            {
                return Err(GraphCarryInstallError::DuplicateStripLane);
            }
        }
        let lends_planes = self
            .source_set
            .as_ref()
            .is_some_and(|set| set.driver.provides_played_planes());
        let mut table = Vec::with_capacity(arms.len());
        let mut entries = Vec::with_capacity(arms.len());
        for arm in arms {
            let armed = match arm.target {
                GraphFadeTarget::Fader(fader) => runtime::strip_lane_mut(
                    &mut self.runtime.units,
                    BuiltinStripOwner::Fader,
                    fader,
                )
                .and_then(|(chain, slot, lane)| {
                    runtime::builtin_processor_mut(chain, slot).map(|processor| (processor, lane))
                })
                .is_some_and(|(processor, lane)| processor.arm_fade_in(lane)),
                GraphFadeTarget::Route(unit) => {
                    self.runtime.arm_route_fade_in(unit, arm.ramp_samples)
                }
                GraphFadeTarget::RouteOut(unit) => {
                    self.runtime.arm_route_fade_out(unit, arm.ramp_samples)
                }
            };
            if !armed {
                entries.push(None);
                continue;
            }
            entries.push(u32::try_from(table.len()).ok());
            let claim = arm
                .source
                .as_ref()
                .filter(|_| lends_planes)
                .and_then(|node| {
                    self.source_set
                        .as_ref()
                        .and_then(|set| set.claims().iter().position(|claim| claim.node == *node))
                });
            table.push(FadeArm {
                target: arm.target,
                claim,
                delay_samples: arm.delay_samples,
                ramp_samples: arm.ramp_samples,
                tail_samples: arm.tail_samples,
                state: FadeArmState::Waiting,
            });
        }
        self.fade_arms_pending = u32::try_from(table.len()).unwrap_or(u32::MAX);
        self.fade_fired = GraphFadeArmWatch::new(table.len());
        self.fade_arms = table.into_boxed_slice();
        Ok((entries, self.fade_fired.clone()))
    }

    /// [`install_fade_arm_carry`] on a bound executor.
    fn install_fade_arm_carry(
        &mut self,
        pairs: Vec<GraphFadeArmPair>,
    ) -> Result<(), GraphCarryInstallError> {
        if self.carry.is_none() {
            return Err(GraphCarryInstallError::NoStripCarryProgram);
        }
        for (index, pair) in pairs.iter().enumerate() {
            if pair.successor as usize >= self.fade_arms.len() {
                return Err(GraphCarryInstallError::StripLaneOutOfRange);
            }
            if pairs[..index].iter().any(|earlier| {
                earlier.successor == pair.successor || earlier.predecessor == pair.predecessor
            }) {
                return Err(GraphCarryInstallError::DuplicateStripLane);
            }
        }
        self.fade_arm_carry = pairs.into_boxed_slice();
        Ok(())
    }
}

/// Which entries of a plan's fade table have fired, and which have settled (issue #1392, root
/// ruling of attempt 2; #1953b): one flag word per entry, set by render (a release store, no lock;
/// render is the only writer) and read by the control thread, which drops a fired arm's
/// inventory row so a successor neither carries nor re-arms it. A flag read clear while render
/// fires the entry only costs a re-armed entry that its carry copies fired.
///
/// An entry settles once its fade has ended and, for a fading copy, its destination line has
/// delivered it ([`GraphFadeArm::tail_samples`]): until then a route's op still ramps (or its line
/// still holds the fade), so a successor keeps it live and carries its state.
#[derive(Clone, Debug, Default)]
pub struct GraphFadeArmWatch(std::sync::Arc<[core::sync::atomic::AtomicU8]>);

/// The watch flag of an entry that fired.
const WATCH_FIRED: u8 = 1;
/// The watch flag of an entry that settled.
const WATCH_SETTLED: u8 = 2;

impl GraphFadeArmWatch {
    fn new(entries: usize) -> Self {
        Self(
            (0..entries)
                .map(|_| core::sync::atomic::AtomicU8::new(0))
                .collect(),
        )
    }

    // REALTIME_POLICY_BEGIN
    /// Raise `flags` on `entry`. Render-thread code: one load and one atomic store (render is the
    /// only writer, so no read-modify-write is needed).
    fn set(&self, entry: usize, flags: u8) {
        if let Some(flag) = self.0.get(entry) {
            let held = flag.load(core::sync::atomic::Ordering::Relaxed);
            flag.store(held | flags, core::sync::atomic::Ordering::Release);
        }
    }
    // REALTIME_POLICY_END

    fn flags(&self, entry: u32) -> u8 {
        self.0
            .get(entry as usize)
            .map_or(0, |flag| flag.load(core::sync::atomic::Ordering::Acquire))
    }

    /// Whether `entry` has fired; `false` past the table.
    #[must_use]
    pub fn fired(&self, entry: u32) -> bool {
        self.flags(entry) & WATCH_FIRED != 0
    }

    /// Whether `entry` has settled (#1953b): fired, its ramp ended and its tail delivered; `false`
    /// past the table.
    #[must_use]
    pub fn settled(&self, entry: u32) -> bool {
        self.flags(entry) & WATCH_SETTLED != 0
    }

    /// Heap bytes the watch holds: its flags and the shared allocation's two reference counts;
    /// none for the empty watch of a plan with no arm.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        match self.0.len() {
            0 => 0,
            _ => {
                core::mem::size_of_val::<[core::sync::atomic::AtomicU8]>(&self.0)
                    + WATCH_HEADER_BYTES
            }
        }
    }
}

/// The strong and weak counts at the head of an `Arc` allocation.
const WATCH_HEADER_BYTES: usize = 2 * core::mem::size_of::<usize>();

/// Two watches are equal when they are one table.
impl PartialEq for GraphFadeArmWatch {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for GraphFadeArmWatch {}

/// One fade arm a successor takes from the plan it displaces (issue #1392 D3): entry
/// `predecessor` of that plan's table hands its render state to entry `successor` of this one.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphFadeArmPair {
    /// The entry in this plan's table, as [`install_fade_arms`] returned it.
    pub successor: u32,
    /// The entry in the displaced plan's table, as its own [`install_fade_arms`] returned it.
    pub predecessor: u32,
}

/// Install the fade-arm section of the hand-over (issue #1392 D3), after the plan's
/// [`install_fade_arms`] and its [`GraphCarryProgram`]: at the swap block each pair's successor
/// entry takes its predecessor entry's state, so a waiting fade keeps waiting with its
/// predecessor's progress, a scheduled one keeps its sample, and a fired one stays fired.
///
/// Refused, installing nothing, when the plan is not a graph plan, has no carry program, a
/// successor entry is past the table, or an entry repeats. A predecessor entry past that plan's
/// table is skipped at the swap block (its successor entry waits afresh).
pub fn install_fade_arm_carry(
    plan: &mut PreparedRenderPlan,
    pairs: Vec<GraphFadeArmPair>,
) -> Result<(), GraphCarryInstallError> {
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?
        .install_fade_arm_carry(pairs)
}

// REALTIME_POLICY_BEGIN
/// Copy each carried fade arm's state from the displaced plan's table (issue #1392 D3, D8): a
/// copied `Done` leaves the pending count. A route entry that takes a fired state (#1877 D6: the
/// displaced route fired after this plan's preparation re-armed it; #1953b: it fired before, and
/// its ramp or its copy's tail is still in flight) fires its own arm here, so the route never
/// waits for a fire its table no longer holds; the live-send section that runs after this one
/// then gives a carried route the displaced route's ramp. A strip's lane import, which runs
/// before, already overwrote its arm. Bounded by `pairs`; allocation-free.
fn carry_fade_arms(
    runtime: &mut runtime::Runtime,
    arms: &mut [FadeArm],
    pending: &mut u32,
    fired: &GraphFadeArmWatch,
    predecessor: &[FadeArm],
    pairs: &[GraphFadeArmPair],
) {
    for pair in pairs {
        let (Some(own), Some(from)) = (
            arms.get_mut(pair.successor as usize),
            predecessor.get(pair.predecessor as usize),
        ) else {
            continue;
        };
        if !own.state.fired() && from.state.fired() {
            fired.set(pair.successor as usize, WATCH_FIRED);
            if let GraphFadeTarget::Route(unit) | GraphFadeTarget::RouteOut(unit) = own.target {
                runtime.fire_route_fade_in(unit);
            }
        }
        if own.state != FadeArmState::Done && from.state == FadeArmState::Done {
            *pending = pending.saturating_sub(1);
            fired.set(pair.successor as usize, WATCH_SETTLED);
        }
        // #1953b: a settling entry settles when this plan's own tail has passed: a kept copy
        // whose line grew waits for the growth. A shorter line never settles it earlier: the
        // line carry is head-aligned, so the fade's last samples may still be in it.
        own.state = match from.state {
            FadeArmState::Settling(at) => FadeArmState::Settling(at.saturating_add(u64::from(
                own.tail_samples.saturating_sub(from.tail_samples),
            ))),
            state => state,
        };
    }
}
// REALTIME_POLICY_END

// REALTIME_POLICY_BEGIN
/// Advance every pending fade arm at the block that starts at `block_start` (issue #1288 D3):
/// a waiting arm whose source played this block (or that waits for none) is scheduled
/// `delay_samples` later, an arm whose sample has come fires its lane's fade (in, or out for a
/// fading copy, #1953b), and a fired arm settles once its ramp and its tail have passed. Bounded
/// by the table; allocation-free.
fn step_fade_arms(
    runtime: &mut runtime::Runtime,
    arms: &mut [FadeArm],
    pending: &mut u32,
    fired: &GraphFadeArmWatch,
    block_start: u64,
    sources: Option<&dyn GraphSourcePlanes>,
) {
    for (entry, arm) in arms.iter_mut().enumerate() {
        if arm.state == FadeArmState::Waiting {
            let played = arm
                .claim
                .is_none_or(|claim| sources.is_some_and(|set| set.played_planes(claim).is_some()));
            if played {
                arm.state =
                    FadeArmState::FireAt(block_start.saturating_add(u64::from(arm.delay_samples)));
            }
        }
        if let FadeArmState::FireAt(at) = arm.state
            && at <= block_start
        {
            match arm.target {
                GraphFadeTarget::Fader(fader) => {
                    // Issue #1782a: the arm names the lane bind gave the strip.
                    let fader = runtime.current_location(fader);
                    if let Some((chain, slot, lane)) =
                        runtime::strip_lane_mut(&mut runtime.units, BuiltinStripOwner::Fader, fader)
                        && let Some(processor) = runtime::builtin_processor_mut(chain, slot)
                    {
                        processor.fire_fade_in(lane, arm.ramp_samples);
                    }
                }
                // #1363 D3: before the route's op runs, so this block's live value drains after.
                // #1953b: a fading copy's arm fires the same way, to silence.
                GraphFadeTarget::Route(unit) | GraphFadeTarget::RouteOut(unit) => {
                    runtime.fire_route_fade_in(unit);
                }
            }
            arm.state = FadeArmState::Settling(
                block_start
                    .saturating_add(u64::from(arm.ramp_samples))
                    .saturating_add(u64::from(arm.tail_samples)),
            );
            fired.set(entry, WATCH_FIRED);
        }
        if let FadeArmState::Settling(at) = arm.state
            && at <= block_start
        {
            arm.state = FadeArmState::Done;
            *pending = pending.saturating_sub(1);
            fired.set(entry, WATCH_SETTLED);
        }
    }
}
// REALTIME_POLICY_END

/// Test-only (issue #1288): `(delay_samples, ramp_samples)` of each installed fade arm, in
/// installation order; `None` for any other plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_fade_arms(plan: &mut PreparedRenderPlan) -> Option<Vec<(u32, u32)>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(
        executor
            .fade_arms
            .iter()
            .map(|arm| (arm.delay_samples, arm.ramp_samples))
            .collect(),
    )
}

pub use rack::{EffectLaneKind, EffectLaneLayout};

/// Where an effect instance sits in a bound graph plan, for a swap's carry (issues #1279, #1281):
/// a lane of an effect bank stage, or a per-node instance (a live-controlled effect op). Valid
/// only for the plan it was read from.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GraphEffectSite {
    /// A lane of an effect bank stage.
    Lane(GraphLaneLocation),
    /// A per-node instance: the runtime unit that holds its op.
    Node(u32),
}

/// One effect lane a successor takes from its predecessor at the swap block: the whole state of
/// the instance at `predecessor` in the displaced plan is moved into `successor` (issue #1279 D3).
/// A lane may change representation in the move: a bank lane to a per-node instance or back
/// (#1281). A move between two per-node instances exchanges the processor and the dry shunt by
/// pointer and passes nothing through the payload scratch (#1282).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphEffectMove {
    /// The instance in the successor, the plan the move is installed in.
    pub successor: GraphEffectSite,
    /// The instance in the plan the successor displaces.
    pub predecessor: GraphEffectSite,
}

/// One effect instance of a bound graph plan that a swap can move (issues #1279 D1, #1280, #1281):
/// a populated lane of a bank stage, or a live-controlled per-node instance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphEffectLane {
    /// The rack the effect sits in: `Simd1` for a `pre_insert` console slot, `Simd2` for a
    /// `post_insert` one, `Dynamic` for an insert.
    pub rack: RackId,
    /// The effect ID: a console slot's slot ID.
    pub effect_id: Box<str>,
    /// Where the instance sits.
    pub location: GraphEffectSite,
    /// What the lane must share with the lane it is moved to or from.
    pub layout: EffectLaneLayout,
    /// The kind of stage and control lane the instance has: the instance it is moved to or from
    /// must have the same (#1280).
    pub kind: EffectLaneKind,
}

/// Every effect instance of a bound graph plan that a swap can move, as `(strip ID, instance)`
/// pairs in program order (issues #1279 D1, #1280, #1281); `None` for any other plan. A bank lane
/// with a live control lane or a lowered bypass is listed with its own kind, and so is a per-node
/// live-controlled effect. Read off the render thread, before publication: a successor's
/// preparation joins its own instances with its predecessor's (recorded when that plan was
/// prepared) by owner key, never by location.
pub fn bank_effect_lanes(
    plan: &mut PreparedRenderPlan,
) -> Option<Vec<(Box<str>, GraphEffectLane)>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(executor.runtime.bank_effect_lanes())
}

/// Install the effect section of the hand-over this graph plan runs when it displaces its
/// predecessor (issue #1279 D3), after its [`GraphCarryProgram`], which names that predecessor,
/// and preallocate the payload scratch at the largest carried payload.
///
/// `scratch_bytes` is the largest payload of the layouts the caller joined: a bound plan knows the
/// layout of a bank lane and not of a per-node instance, so the scratch is the larger of this and
/// the bank layouts it can read.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor site is not a populated lane of an effect bank or a live-controlled
/// per-node effect, or a successor or predecessor site repeats. Predecessor sites and layouts are
/// checked against the predecessor at the swap block, where one that does not resolve refuses the
/// whole hand-over. The moves are kept sorted by
/// predecessor site, so the swap block disengages each predecessor chain once.
pub fn install_effect_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphEffectMove>,
    scratch_bytes: u64,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoEffectCarryProgram);
    }
    let mut scratch = scratch_bytes;
    for (index, step) in moves.iter().enumerate() {
        if !executor.runtime.has_effect_site(step.successor) {
            return Err(GraphCarryInstallError::EffectLaneOutOfRange);
        }
        if let GraphEffectSite::Lane(at) = step.successor {
            let Some(layout) = executor.runtime.effect_lane_layout(at) else {
                return Err(GraphCarryInstallError::EffectLaneOutOfRange);
            };
            scratch = scratch.max(layout.payload_bytes());
        }
        if moves[..index].iter().any(|earlier| {
            earlier.successor == step.successor || earlier.predecessor == step.predecessor
        }) {
            return Err(GraphCarryInstallError::DuplicateEffectLane);
        }
    }
    moves.sort_unstable_by_key(|step| step.predecessor);
    // #1633 D1: the carried slots of the successor lanes `moves` names, and no others.
    for step in &moves {
        executor.runtime.install_effect_carry_slots(step.successor);
    }
    executor.carry_scratch =
        vec![0_u8; usize::try_from(scratch).unwrap_or(usize::MAX)].into_boxed_slice();
    executor.carry_effects = moves.into_boxed_slice();
    Ok(())
}

/// The bytes [`install_effect_carry`] allocates for the carried slots of the successor lanes
/// `moves` names (#1633 D1): the part of a carry's charge that depends on the lanes' cell tables.
/// Read before the install, so preparation admits the carry before it allocates. `None` when the
/// plan is not a graph plan, a successor site does not resolve, or the sum overflows.
pub fn effect_carry_slot_bytes(
    plan: &mut PreparedRenderPlan,
    moves: &[GraphEffectMove],
) -> Option<u64> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    moves.iter().try_fold(0_u64, |total, step| {
        total.checked_add(
            executor
                .runtime
                .effect_site_carry_slot_bytes(step.successor)?,
        )
    })
}

/// The effect-section bytes a carry of `moves` lanes with a largest payload of `scratch_bytes`
/// and `slot_bytes` of carried slots ([`effect_carry_slot_bytes`]) retains in its plan: the move
/// table, the payload scratch and the slots ([`install_effect_carry`]). Checked arithmetic: `None`
/// on overflow. The one place the charge is spelled, so preparation's admission and
/// [`carry_program_retained_bytes`] cannot disagree.
#[must_use]
pub fn effect_carry_retained_bytes(
    moves: usize,
    scratch_bytes: u64,
    slot_bytes: u64,
) -> Option<u64> {
    u64::try_from(moves.checked_mul(core::mem::size_of::<GraphEffectMove>())?)
        .ok()?
        .checked_add(scratch_bytes)?
        .checked_add(slot_bytes)
}

/// Restores the console-effect carry refused in this plan's banks, summed over every effect bank
/// stage (issue #1279 D6): each left its lane as prepared, at rest. Saturating. `None` for any
/// other plan.
///
/// Read after render, by whoever holds the plan: an own payload is never refused (#1278), so a
/// nonzero count is a defect. Named like `RealtimePlanOwner::carry_mismatch_count`.
pub fn carry_refusal_count(plan: &mut PreparedRenderPlan) -> Option<u64> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(
        executor
            .runtime
            .carry_refusal_count()
            .saturating_add(executor.node_carry_refused),
    )
}

/// The process-unique identity of a graph plan, taken at bind; `None` for any other plan.
pub fn plan_identity(plan: &mut PreparedRenderPlan) -> Option<u64> {
    plan.executor_any_mut()?
        .downcast_mut::<GraphExecutor>()
        .map(|executor| executor.identity)
}

/// Install the hand-over this graph plan runs when it displaces its predecessor.
///
/// Call it before publication. Refused, leaving any earlier program in place, when the plan is
/// not a graph plan, has no source set for a program that moves a source or the timeline, or a
/// successor index is out of range, not vacant or repeated, or a predecessor index is repeated.
/// Predecessor indices are checked against the predecessor at the swap block, where an
/// out-of-range index refuses the whole move.
pub fn install_carry_program(
    plan: &mut PreparedRenderPlan,
    program: GraphCarryProgram,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    // A program that moves no source and no timeline needs no source set (issue #1276: a plan
    // may carry only strip state). The timeline lives in the source set (#1641 D1).
    let set = match executor.source_set.as_ref() {
        Some(set) => Some(set),
        None if program.sources.is_empty() && !program.timeline => None,
        None => return Err(GraphCarryInstallError::NoSourceSet),
    };
    for (index, &(successor, predecessor)) in program.sources.iter().enumerate() {
        let Some(set) = set else {
            return Err(GraphCarryInstallError::NoSourceSet);
        };
        match set.driver.source_vacancy(successor as usize) {
            None => return Err(GraphCarryInstallError::SourceIndexOutOfRange),
            Some(false) => return Err(GraphCarryInstallError::SourceNotVacant),
            Some(true) => {}
        }
        if program.sources[..index]
            .iter()
            .any(|&(s, p)| s == successor || p == predecessor)
        {
            return Err(GraphCarryInstallError::DuplicateSourceIndex);
        }
    }
    executor.carry = Some(program);
    Ok(())
}

/// Where one observer sits in a bound graph plan: its runtime unit, its member op within a bank
/// (or [`Self::NO_MEMBER`] for a unit that is one op) and its index in that op's observer list
/// (#1327 D2). Valid only for the plan it was read from.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GraphObserverLocation {
    /// The runtime unit.
    pub unit: u32,
    /// The member op of a bank unit, or [`Self::NO_MEMBER`].
    pub member: u32,
    /// The observer's index in the op's observer list.
    pub index: u32,
}

impl GraphObserverLocation {
    /// `member` of an observer in a unit that is a single op.
    pub const NO_MEMBER: u32 = u32::MAX;
}

/// One observer of a bound graph plan that a swap can carry (#1327 D2): its kind, the handle of
/// its binding and where it sits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObserverRow {
    /// The reader vector the observer's state belongs to.
    pub kind: ObserverKind,
    /// The handle of the observer's binding (a meter's is its meter handle).
    pub handle: u64,
    /// Where the observer sits in this plan.
    pub location: GraphObserverLocation,
}

/// One observer a successor takes from its predecessor at the swap block (#1327 D2): the boxed
/// observer at `predecessor` in the displaced plan and the one at `successor` here exchange
/// places, so a reader that holds the carried observer's consumer stays connected to the
/// observer that renders on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObserverMove {
    /// The reader vector both entries index.
    pub kind: ObserverKind,
    /// The observer in the successor, the plan the move is installed in.
    pub successor: GraphObserverLocation,
    /// The observer in the plan the successor displaces.
    pub predecessor: GraphObserverLocation,
    /// The successor reader's index in its kind's reader vector ([`ObserverCarry::successor`]).
    pub successor_entry: usize,
    /// The predecessor reader's index ([`ObserverCarry::predecessor`]).
    pub predecessor_entry: usize,
}

/// A fresh observer that joins a carried observer's window grid (#1327 D6): after the moves, the
/// observer at `fresh` opens its first window where the carried observer at `donor` has its open
/// window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphObserverJoin {
    /// The observer in the successor that has no carry record.
    pub fresh: GraphObserverLocation,
    /// The successor location of a carried observer of the same configuration.
    pub donor: GraphObserverLocation,
}

/// One effect instance's observation taps a successor takes from its predecessor at the swap
/// block (#1327 D1, D2): the taps follow the effect owner, so a move is named by the sites of the
/// two effect instances and applies only if that effect move carried.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphTapMove {
    /// The effect instance in the successor, the plan the move is installed in. Must be the
    /// successor of one of the plan's [`GraphEffectMove`]s and hold an observation lane.
    pub successor: GraphEffectSite,
    /// The effect instance in the plan the successor displaces: the predecessor of that same
    /// effect move.
    pub predecessor: GraphEffectSite,
    /// The successor reader's index in the observation reader vector ([`ObserverCarry::successor`]).
    pub successor_entry: usize,
    /// The predecessor reader's index ([`ObserverCarry::predecessor`]).
    pub predecessor_entry: usize,
}

/// A [`GraphTapMove`] bound to the effect move it follows (#1327 D1).
#[derive(Clone, Copy)]
struct ObserverTapMove {
    step: GraphTapMove,
    /// The index of the effect move in the plan's sorted effect table.
    effect_move: usize,
    /// Whether that effect move carried at the swap block: written by the effect section, read by
    /// the tap section that follows it.
    taken: bool,
}

/// The observer section of a hand-over: the meter moves, the grid joins, the effect tap moves and
/// the preallocated carry record (#1327 D2, D3, D6).
struct ObserverCarryProgram {
    moves: Box<[GraphObserverMove]>,
    joins: Box<[GraphObserverJoin]>,
    /// Sorted by `effect_move`.
    taps: Box<[ObserverTapMove]>,
    /// One slot per move and per tap move, written at the swap block, `carried` of them.
    record: Box<[ObserverCarry]>,
    carried: usize,
}

impl ObserverCarryProgram {
    /// A program of these tables, or `None` for no section at all.
    fn assemble(
        moves: Box<[GraphObserverMove]>,
        joins: Box<[GraphObserverJoin]>,
        taps: Box<[ObserverTapMove]>,
    ) -> Option<Box<Self>> {
        if moves.is_empty() && joins.is_empty() && taps.is_empty() {
            return None;
        }
        let placeholder = ObserverCarry {
            kind: ObserverKind::Meter,
            successor: 0,
            predecessor: 0,
        };
        let record = vec![placeholder; moves.len() + taps.len()].into_boxed_slice();
        Some(Box::new(Self {
            moves,
            joins,
            taps,
            record,
            carried: 0,
        }))
    }

    fn retained_bytes(&self) -> u64 {
        observer_carry_retained_bytes(self.moves.len(), self.joins.len(), self.taps.len())
            .unwrap_or(u64::MAX)
    }
}

/// The bytes an observer section of `moves` meter moves, `joins` grid joins and `taps` effect tap
/// moves retains in its plan (#1327): the section, its three tables and the carry record. `None`
/// on overflow; zero for no section.
#[must_use]
pub fn observer_carry_retained_bytes(moves: usize, joins: usize, taps: usize) -> Option<u64> {
    if moves == 0 && joins == 0 && taps == 0 {
        return Some(0);
    }
    let bytes = core::mem::size_of::<ObserverCarryProgram>()
        .checked_add(moves.checked_mul(core::mem::size_of::<GraphObserverMove>())?)?
        .checked_add(joins.checked_mul(core::mem::size_of::<GraphObserverJoin>())?)?
        .checked_add(taps.checked_mul(core::mem::size_of::<ObserverTapMove>())?)?
        .checked_add(
            moves
                .checked_add(taps)?
                .checked_mul(core::mem::size_of::<ObserverCarry>())?,
        )?;
    u64::try_from(bytes).ok()
}

/// Every observer of a bound graph plan a swap can carry (those that answer
/// [`GraphRuntimeObserver::carry_kind`]), in program order (#1327 D2); `None` for any other plan.
/// Read off the render thread, before publication: a successor's preparation joins its own rows
/// with its predecessor's (recorded when that plan was prepared) by owner key, never by location.
pub fn observer_rows(plan: &mut PreparedRenderPlan) -> Option<Vec<GraphObserverRow>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(executor.runtime.observer_rows())
}

/// Install the observer section of the hand-over this graph plan runs when it displaces its
/// predecessor (#1327 D2, D6), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor location is not an observer of the move's kind, a successor or
/// predecessor location repeats, or a join's donor is not a moved successor observer of the
/// fresh observer's kind or its fresh observer is itself moved. Predecessor locations are checked
/// against the predecessor at the swap block, where one that does not resolve refuses the whole
/// hand-over. Allocates the carry record here, off the render thread.
pub fn install_observer_carry(
    plan: &mut PreparedRenderPlan,
    moves: Vec<GraphObserverMove>,
    joins: Vec<GraphObserverJoin>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoObserverCarryProgram);
    }
    let kinds: BTreeMap<GraphObserverLocation, ObserverKind> = executor
        .runtime
        .observer_rows()
        .into_iter()
        .map(|row| (row.location, row.kind))
        .collect();
    let mut moved_successors = BTreeSet::new();
    let mut moved_predecessors = BTreeSet::new();
    let mut entries = BTreeSet::new();
    for step in &moves {
        if kinds.get(&step.successor) != Some(&step.kind) {
            return Err(GraphCarryInstallError::ObserverOutOfRange);
        }
        if !moved_successors.insert(step.successor)
            || !moved_predecessors.insert(step.predecessor)
            || !entries.insert((step.kind as u8, true, step.successor_entry))
            || !entries.insert((step.kind as u8, false, step.predecessor_entry))
        {
            return Err(GraphCarryInstallError::DuplicateObserver);
        }
    }
    let mut joined = BTreeSet::new();
    for join in &joins {
        let Some(kind) = kinds.get(&join.fresh) else {
            return Err(GraphCarryInstallError::ObserverOutOfRange);
        };
        if kinds.get(&join.donor) != Some(kind) || !moved_successors.contains(&join.donor) {
            return Err(GraphCarryInstallError::ObserverOutOfRange);
        }
        if moved_successors.contains(&join.fresh) || !joined.insert(join.fresh) {
            return Err(GraphCarryInstallError::DuplicateObserver);
        }
    }
    // The tap moves of an earlier `install_observation_carry` stay.
    let taps = executor
        .observer_carry
        .take()
        .map_or_else(Box::default, |program| program.taps);
    executor.observer_carry =
        ObserverCarryProgram::assemble(moves.into_boxed_slice(), joins.into_boxed_slice(), taps);
    Ok(())
}

/// Install the effect tap section of the hand-over this graph plan runs when it displaces its
/// predecessor (#1327 D1, D2), after its [`GraphCarryProgram`] and [`install_effect_carry`], whose
/// sorted move table the tap moves bind to: calling `install_effect_carry` again afterwards
/// invalidates them.
///
/// Each tap move is the observation lane of one effect move. At the swap block it applies only if
/// that effect move carried, after the predecessor's pending `Observe` records were applied to its
/// tap, and exchanges the two lanes' whole tap state ([`effect_contract::ObservationLane::swap_state`]),
/// publisher included. A lane pair with different tap menus exchanges nothing.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a tap move names no effect move of the plan (by both sites), its successor holds no
/// observation lane, or a successor or predecessor site or entry repeats. Allocates the record
/// here, off the render thread.
pub fn install_observation_carry(
    plan: &mut PreparedRenderPlan,
    taps: Vec<GraphTapMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoObserverCarryProgram);
    }
    let mut bound = Vec::with_capacity(taps.len());
    let mut successors = BTreeSet::new();
    let mut predecessors = BTreeSet::new();
    let mut entries = BTreeSet::new();
    for step in taps {
        let Ok(effect_move) = executor
            .carry_effects
            .binary_search_by_key(&step.predecessor, |effect| effect.predecessor)
        else {
            return Err(GraphCarryInstallError::ObserverOutOfRange);
        };
        if executor.carry_effects[effect_move].successor != step.successor
            || !executor.runtime.has_observation_lane(step.successor)
        {
            return Err(GraphCarryInstallError::ObserverOutOfRange);
        }
        if !successors.insert(step.successor)
            || !predecessors.insert(step.predecessor)
            || !entries.insert((true, step.successor_entry))
            || !entries.insert((false, step.predecessor_entry))
        {
            return Err(GraphCarryInstallError::DuplicateObserver);
        }
        bound.push(ObserverTapMove {
            step,
            effect_move,
            taken: false,
        });
    }
    bound.sort_unstable_by_key(|tap| tap.effect_move);
    let (moves, joins) = executor.observer_carry.take().map_or_else(
        || (Box::default(), Box::default()),
        |program| (program.moves, program.joins),
    );
    executor.observer_carry =
        ObserverCarryProgram::assemble(moves, joins, bound.into_boxed_slice());
    Ok(())
}

/// One compensation line of a bound graph plan, named by the edge it delays (#1283 D1): what a
/// successor's preparation joins with its own lines, by edge ID and endpoints, never by `line`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphDelayLine {
    /// The delayed edge.
    pub edge: GraphEdgeId,
    /// Where the edge's signal comes from. For a route's two edges (`RouteSource`,
    /// `RouteDestination`) this is the route's own source, so a route whose source was
    /// re-pointed does not match its predecessor on either edge.
    pub source: GraphNodeId,
    /// Where the edge's signal goes. For a route's two edges this is the route's own
    /// destination.
    pub destination: GraphNodeId,
    /// The line's index in this plan. Meaningful only to this plan's successor's carry program,
    /// which names it as the line to take.
    pub line: u32,
    /// The line's length in samples a channel.
    pub samples: u64,
}

impl GraphDelayLine {
    /// Bytes of ID text the row owns on the heap: the strings of its edge and its two endpoints.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        fn edge(id: &GraphEdgeId) -> usize {
            match id {
                GraphEdgeId::TrackMain { target } => node(target),
                GraphEdgeId::RouteSource { route_id }
                | GraphEdgeId::RouteDestination { route_id } => route_id.as_str().len(),
                GraphEdgeId::EffectSidechain { effect, port } => {
                    effect.track_id.as_str().len() + effect.effect_id.as_str().len() + port.len()
                }
            }
        }
        fn node(id: &GraphNodeId) -> usize {
            match id {
                GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().len(),
                GraphNodeId::Effect(effect) => {
                    effect.track_id.as_str().len() + effect.effect_id.as_str().len()
                }
                GraphNodeId::Route { route_id } => route_id.as_str().len(),
                GraphNodeId::Submix { submix_id } => submix_id.as_str().len(),
                GraphNodeId::Output { output_id } => output_id.as_str().len(),
                GraphNodeId::CompensationDelay { edge_id } => {
                    core::mem::size_of::<GraphEdgeId>() + edge(edge_id)
                }
            }
        }
        edge(&self.edge) + node(&self.source) + node(&self.destination)
    }
}

/// One compensation line a successor takes from its predecessor at the swap block (#1283 D1, D2):
/// the pending samples of `predecessor` in the displaced plan continue in `successor`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphDelayMove {
    /// The line in the successor, the plan the move is installed in.
    pub successor: u32,
    /// The line in the plan the successor displaces.
    pub predecessor: u32,
}

/// The identity of every compensation line `program` lowered from `spec` (#1283 D1), in line order.
fn compensation_lines_of(
    spec: &GraphSpec,
    program: &program::ExecutionProgram,
) -> Vec<GraphDelayLine> {
    // A route's two edges share one identity: the route's own endpoints.
    let mut routes: BTreeMap<&StableGraphId, (Option<&GraphNodeId>, Option<&GraphNodeId>)> =
        BTreeMap::new();
    for edge in &spec.edges {
        match &edge.id {
            GraphEdgeId::RouteSource { route_id } => {
                routes.entry(route_id).or_default().0 = Some(&edge.source.node);
            }
            GraphEdgeId::RouteDestination { route_id } => {
                routes.entry(route_id).or_default().1 = Some(&edge.destination.node);
            }
            GraphEdgeId::TrackMain { .. } | GraphEdgeId::EffectSidechain { .. } => {}
        }
    }
    program
        .delays
        .iter()
        .enumerate()
        .map(|(line, delay)| {
            let edge = &spec.edges[delay.edge as usize];
            let ends = match &edge.id {
                GraphEdgeId::RouteSource { route_id }
                | GraphEdgeId::RouteDestination { route_id } => routes.get(route_id).copied(),
                GraphEdgeId::TrackMain { .. } | GraphEdgeId::EffectSidechain { .. } => None,
            };
            let (source, destination) = ends.unwrap_or_default();
            GraphDelayLine {
                edge: edge.id.clone(),
                source: source.unwrap_or(&edge.source.node).clone(),
                destination: destination.unwrap_or(&edge.destination.node).clone(),
                line: u32::try_from(line).expect("a line index fits the program's index width"),
                samples: delay.samples,
            }
        })
        .collect()
}

/// The compensation lines of a bound graph plan, in line order (#1283 D1); `None` for any other
/// plan, and empty on a second call. Read once, off the render thread, right after bind: a
/// successor's preparation joins its own lines with its predecessor's (recorded when that plan
/// was prepared), and a plan retains none of the identities it handed out.
pub fn take_compensation_lines(plan: &mut PreparedRenderPlan) -> Option<Vec<GraphDelayLine>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(core::mem::take(&mut executor.compensation_lines))
}

/// Install the compensation-line section of the hand-over this graph plan runs when it displaces
/// its predecessor (#1283 D1), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor line is not a compensation line of the plan, or a successor or
/// predecessor line repeats. Predecessor lines are checked against the predecessor at the swap
/// block, where one that does not resolve refuses the whole hand-over.
pub fn install_delay_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphDelayMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoDelayCarryProgram);
    }
    let mut successors = BTreeSet::new();
    let mut predecessors = BTreeSet::new();
    for step in &moves {
        if step.successor as usize >= executor.runtime.delays.len() {
            return Err(GraphCarryInstallError::DelayLineOutOfRange);
        }
        if !successors.insert(step.successor) || !predecessors.insert(step.predecessor) {
            return Err(GraphCarryInstallError::DuplicateDelayLine);
        }
    }
    moves.sort_unstable_by_key(|step| step.predecessor);
    executor.carry_delays = moves.into_boxed_slice();
    Ok(())
}

/// The bytes a compensation-line section of `moves` lines retains in its plan (#1283): its move
/// table. `None` on overflow. The one place the charge is spelled, so preparation's admission and
/// [`carry_program_retained_bytes`] cannot disagree.
#[must_use]
pub fn delay_carry_retained_bytes(moves: usize) -> Option<u64> {
    u64::try_from(moves.checked_mul(core::mem::size_of::<GraphDelayMove>())?).ok()
}

/// Bit of [`GraphStripDelayMove::lanes`] for the left lane of a strip delay line (#1284 D1).
pub const STRIP_DELAY_LEFT: u8 = 1;
/// Bit of [`GraphStripDelayMove::lanes`] for the right lane of a strip delay line (#1284 D1).
pub const STRIP_DELAY_RIGHT: u8 = 2;

/// One strip input alignment line of a bound graph plan (#1284 D1): the delay a track or submix
/// declares as `delay_samples`, named by the strip it belongs to. What a successor's preparation
/// joins with its own lines, by strip ID, never by `line`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphStripDelayLine {
    /// The strip ID: a track's or a submix's.
    pub strip: Box<str>,
    /// The line's index in this plan. Meaningful only to this plan's successor's carry program,
    /// which names it as the line to take.
    pub line: u32,
    /// The length of the left and the right lane's ring, which is the lane's `delay_samples`.
    pub samples: [u32; 2],
}

impl GraphStripDelayLine {
    /// Bytes of ID text the row owns on the heap.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        self.strip.len()
    }
}

/// The lanes of one strip delay line a successor takes from its predecessor at the swap block
/// (#1284 D1): the pending samples of `predecessor` in the displaced plan continue in
/// `successor`, lane by lane.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphStripDelayMove {
    /// The line in the successor, the plan the move is installed in.
    pub successor: u32,
    /// The line in the plan the successor displaces.
    pub predecessor: u32,
    /// The lanes that move: [`STRIP_DELAY_LEFT`] and [`STRIP_DELAY_RIGHT`]. A moved lane's ring
    /// has the same length in both plans.
    pub lanes: u8,
}

/// One live send of a bound graph plan (#1284 D2): a route with a live lane (#1391), named
/// by its route ID. What a successor's preparation joins with its own, by ID.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphLiveRoute {
    /// The route's session ID.
    pub route_id: Box<str>,
    /// The route's runtime unit in this plan. Meaningful only to this plan's successor's carry
    /// program, which names it as the route to take.
    pub unit: u32,
    /// Whether the route is a following route (#1652 D1): a successor's move from this row
    /// carries [`GraphRouteMove::predecessor_follows`].
    pub following: bool,
    /// A following route's automated source lanes (its flags' `automated`, the follow curves its
    /// cursors hear); `[false; 2]` for a six-word route. A successor's join compares it with its
    /// own to find a carried route that loses a source mute curve (B19-R7).
    pub automated: [bool; 2],
}

impl GraphLiveRoute {
    /// Bytes of ID text the row owns on the heap.
    #[must_use]
    pub fn heap_bytes(&self) -> usize {
        self.route_id.len()
    }
}

/// One live send a successor takes from its predecessor at the swap block (#1284 D2): the ramp, its
/// position and its mute, after the predecessor's pending value was applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GraphRouteMove {
    /// The route's unit in the successor, the plan the move is installed in.
    pub successor: u32,
    /// The route's unit in the plan the successor displaces.
    pub predecessor: u32,
    /// Whether the predecessor's row is a following route ([`GraphLiveRoute::following`]): the
    /// move then gets a slot in the section's side table (#1652 D3, MINOR-3).
    pub predecessor_follows: bool,
}

/// The strip delay lines of a bound graph plan, in line order (#1284 D1); `None` for any other
/// plan, and empty on a second call. Read once, off the render thread, right after bind: a
/// successor's preparation joins its own lines with its predecessor's (recorded when that plan was
/// prepared), and a plan retains none of the identities it handed out.
pub fn take_strip_delay_lines(plan: &mut PreparedRenderPlan) -> Option<Vec<GraphStripDelayLine>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(core::mem::take(&mut executor.bind_identities.strip_delays))
}

/// The live sends of a bound graph plan, in unit order (#1284 D2); `None` for any other plan, and
/// empty on a second call. Read once, off the render thread, right after bind, as
/// [`take_strip_delay_lines`].
pub fn take_live_routes(plan: &mut PreparedRenderPlan) -> Option<Vec<GraphLiveRoute>> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(core::mem::take(&mut executor.bind_identities.live_routes))
}

/// Install the strip-delay section of the hand-over this graph plan runs when it displaces its
/// predecessor (#1284 D1), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor line is not a strip delay line of the plan, a move names no lane or a lane
/// beyond the right one, or a successor or predecessor line repeats. Predecessor lines, and the
/// lengths of the lanes, are checked against the predecessor at the swap block, where one that does
/// not resolve refuses the whole hand-over.
pub fn install_strip_delay_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphStripDelayMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoStripDelayCarryProgram);
    }
    let mut successors = BTreeSet::new();
    let mut predecessors = BTreeSet::new();
    for step in &moves {
        if step.successor as usize >= executor.runtime.track_delays.len()
            || step.lanes == 0
            || step.lanes & !(STRIP_DELAY_LEFT | STRIP_DELAY_RIGHT) != 0
        {
            return Err(GraphCarryInstallError::StripDelayLineOutOfRange);
        }
        if !successors.insert(step.successor) || !predecessors.insert(step.predecessor) {
            return Err(GraphCarryInstallError::DuplicateStripDelayLine);
        }
    }
    moves.sort_unstable_by_key(|step| step.predecessor);
    executor.carry_strip_delays = moves.into_boxed_slice();
    Ok(())
}

/// One source-claim line a warm successor fills at its adoption block (#1355 D5, lemma L2): the
/// strip delay line of a track whose source the carry program moves. The source is read `P`
/// further ahead in the successor, so the line takes the `P` frames between: the predecessor's
/// pending words first on the lanes that grew by `P` (the line the predecessor held, if any),
/// then, on every lane of `lanes`, the prime the source replays.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphClaimFill {
    /// The strip delay line in the successor ([`GraphStripDelayLine::line`]).
    pub line: u32,
    /// The track's `TrackStage::Input` node: a source claim of the successor, whose planes the
    /// prime pushes.
    pub claim: GraphNodeId,
    /// The predecessor's line of the same strip whose pending words go in first, on
    /// `pending_lanes`; `None` when the predecessor held none.
    pub predecessor: Option<u32>,
    /// The lanes that grew by `P` ([`STRIP_DELAY_LEFT`], [`STRIP_DELAY_RIGHT`]): they start at
    /// rest and take the predecessor's pending words first.
    pub pending_lanes: u8,
    /// The lanes the prime goes into: the grown lanes and the lanes the strip-delay section moves.
    pub lanes: u8,
}

/// One `Input`-tap sidechain line a warm successor fills at its adoption block (#1355 D5, lemma
/// L3): the compensation line of a sidechain edge from a restarted track's `Input` stage into a
/// carried node. Its consumer continues, so the line takes the head of the stream the
/// predecessor's consumer would have heard: the predecessor's pending words of the same edge,
/// then the source track's claim pending in the predecessor, then the prime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphSidechainFill {
    /// The compensation line in the successor ([`GraphDelayLine::line`]).
    pub line: u32,
    /// The source track's `TrackStage::Input` node: a source claim of the successor.
    pub claim: GraphNodeId,
    /// The predecessor's line of the same edge, `None` when it held none.
    pub predecessor: Option<u32>,
    /// The source track's strip delay line in the predecessor, `None` when it held none.
    pub claim_line: Option<u32>,
}

/// One [`GraphClaimFill`] as the plan runs it: the claim resolved to its index.
#[derive(Clone, Copy, Debug)]
struct ClaimFillRow {
    line: u32,
    claim: u32,
    predecessor: Option<u32>,
    pending_lanes: u8,
    lanes: u8,
}

/// One [`GraphSidechainFill`] as the plan runs it, with the words each channel has taken: the
/// predecessor's parts at the carry, the prime at the first block.
#[derive(Clone, Copy, Debug)]
struct SidechainFillRow {
    line: u32,
    claim: u32,
    predecessor: Option<u32>,
    claim_line: Option<u32>,
    written: [u32; 2],
}

/// The bytes a prime section of `claims` and `sidechains` fills retains in its plan (#1355 D5):
/// its two tables. `None` on overflow.
#[must_use]
pub fn prime_carry_retained_bytes(claims: usize, sidechains: usize) -> Option<u64> {
    let claims = claims.checked_mul(core::mem::size_of::<ClaimFillRow>())?;
    let sidechains = sidechains.checked_mul(core::mem::size_of::<SidechainFillRow>())?;
    u64::try_from(claims.checked_add(sidechains)?).ok()
}

/// Install the prime section of the hand-over a warm successor runs at its adoption block (#1355
/// D5), after its [`GraphCarryProgram`]. In the first block after a carried hand-over the plan
/// replays `lead_blocks` blocks of every source the program moves, and of the timeline, from the
/// source-read sample the predecessor would have read, and pushes them into `claims` and
/// `sidechains` behind the predecessor's pending words (pushed at the carry). The plan's own
/// sources then read `lead_blocks` quanta further ahead, its source-read offset.
///
/// A plan with this section installed fills its lines whenever it is adopted with a carry: there
/// is no adoption of it whose lines stay at rest. Its readiness is the publisher's to check: it is
/// published `Primed { lead_blocks, .. }` (#1311), so the prime never underruns.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program or source set, `lead_blocks` is 0, a fill names a line that is not the plan's, a node
/// that is not one of its source claims, or no lane, or a line appears twice. Predecessor lines are
/// checked against the predecessor at the swap block, where one that does not resolve refuses the
/// whole hand-over.
pub fn install_prime_carry(
    plan: &mut PreparedRenderPlan,
    lead_blocks: u32,
    claims: Vec<GraphClaimFill>,
    sidechains: Vec<GraphSidechainFill>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    let Some(set) = executor.source_set.as_ref() else {
        return Err(GraphCarryInstallError::NoPrimeCarryProgram);
    };
    if executor.carry.is_none() || lead_blocks == 0 {
        return Err(GraphCarryInstallError::NoPrimeCarryProgram);
    }
    let claim_index = |node: &GraphNodeId| {
        set.claims
            .binary_search_by(|claim| claim.node.cmp(node))
            .ok()
            .and_then(|index| u32::try_from(index).ok())
    };
    let both = STRIP_DELAY_LEFT | STRIP_DELAY_RIGHT;
    let mut lines = BTreeSet::new();
    let mut claim_rows = Vec::with_capacity(claims.len());
    for fill in &claims {
        let claim = claim_index(&fill.claim).ok_or(GraphCarryInstallError::PrimeFillOutOfRange)?;
        if fill.line as usize >= executor.runtime.track_delays.len()
            || fill.lanes == 0
            || fill.lanes & !both != 0
            || fill.pending_lanes & !fill.lanes != 0
        {
            return Err(GraphCarryInstallError::PrimeFillOutOfRange);
        }
        if !lines.insert(fill.line) {
            return Err(GraphCarryInstallError::DuplicatePrimeFill);
        }
        claim_rows.push(ClaimFillRow {
            line: fill.line,
            claim,
            predecessor: fill.predecessor,
            pending_lanes: fill.pending_lanes,
            lanes: fill.lanes,
        });
    }
    let mut lines = BTreeSet::new();
    let mut sidechain_rows = Vec::with_capacity(sidechains.len());
    for fill in &sidechains {
        let claim = claim_index(&fill.claim).ok_or(GraphCarryInstallError::PrimeFillOutOfRange)?;
        if fill.line as usize >= executor.runtime.delays.len() {
            return Err(GraphCarryInstallError::PrimeFillOutOfRange);
        }
        if !lines.insert(fill.line) {
            return Err(GraphCarryInstallError::DuplicatePrimeFill);
        }
        sidechain_rows.push(SidechainFillRow {
            line: fill.line,
            claim,
            predecessor: fill.predecessor,
            claim_line: fill.claim_line,
            written: [0; 2],
        });
    }
    executor.prime_blocks = lead_blocks;
    executor.prime_claims = claim_rows.into_boxed_slice();
    executor.prime_sidechains = sidechain_rows.into_boxed_slice();
    Ok(())
}

// REALTIME_POLICY_BEGIN: #1355 D5, the prime's fills run in the adoption block.
/// Whether every predecessor line the prime fills name is a line of `predecessor` (#1355 D5).
fn prime_fills_resolve(
    claims: &[ClaimFillRow],
    sidechains: &[SidechainFillRow],
    predecessor: &runtime::Runtime,
) -> bool {
    let track_line = |line: Option<u32>| {
        line.is_none_or(|line| (line as usize) < predecessor.track_delays.len())
    };
    claims.iter().all(|row| track_line(row.predecessor))
        && sidechains.iter().all(|row| {
            track_line(row.claim_line)
                && row
                    .predecessor
                    .is_none_or(|line| (line as usize) < predecessor.delays.len())
        })
}

/// The lanes of `lanes` of a line's pending words, the others empty.
fn lanes_of(pending: [[&[f32]; 2]; 2], lanes: u8) -> [[&[f32]; 2]; 2] {
    let none: [&[f32]; 2] = [&[], &[]];
    let [left, right] = pending;
    [
        if lanes & STRIP_DELAY_LEFT != 0 {
            left
        } else {
            none
        },
        if lanes & STRIP_DELAY_RIGHT != 0 {
            right
        } else {
            none
        },
    ]
}

/// L2's first part (#1355 D5): a grown claim line takes its predecessor line's pending words on
/// the grown lanes. Bounded by two slice copies a lane.
fn fill_claim_pending(
    row: &ClaimFillRow,
    runtime: &mut runtime::Runtime,
    predecessor: &runtime::Runtime,
) {
    let (Some(from), Some(line)) = (
        row.predecessor
            .and_then(|line| predecessor.track_delays.get(line as usize)),
        runtime.track_delays.get_mut(row.line as usize),
    ) else {
        return;
    };
    let [left, right] = lanes_of(from.pending(), row.pending_lanes);
    line.push(left[0], right[0]);
    line.push(left[1], right[1]);
}

/// L3's first parts (#1355 D5): the sidechain line's head takes the predecessor's pending words
/// of the same edge, then the source track's claim pending in the predecessor.
fn fill_sidechain_pending(
    row: &mut SidechainFillRow,
    runtime: &mut runtime::Runtime,
    predecessor: &runtime::Runtime,
) {
    use runtime::FillPart::Words;
    row.written = [0; 2];
    let Some(line) = runtime.delays.get_mut(row.line as usize) else {
        return;
    };
    if let Some(from) = row
        .predecessor
        .and_then(|line| predecessor.delays.get(line as usize))
    {
        let [left, right] = from.pending();
        line.fill_head(Words(left[0]), Words(right[0]), &mut row.written);
        line.fill_head(Words(left[1]), Words(right[1]), &mut row.written);
    }
    if let Some(from) = row
        .claim_line
        .and_then(|line| predecessor.track_delays.get(line as usize))
    {
        let [left, right] = from.pending();
        line.fill_head(Words(left[0]), Words(right[0]), &mut row.written);
        line.fill_head(Words(left[1]), Words(right[1]), &mut row.written);
    }
}

/// The prime (#1355 D5): `blocks` blocks of the timeline and of every source of `moves`, from
/// the source-read sample `first` on, each pushed into the claim lines (L2) and the head of the
/// sidechain lines (L3) of its source; a block a source played no PCM for (its region's end) is
/// `+0.0`, as its render would be. Bounded by `blocks · (moves + fills)`, at most two slice copies
/// a lane and fill; nothing is staged, allocated or freed.
#[allow(clippy::too_many_arguments)]
fn run_prime(
    runtime: &mut runtime::Runtime,
    set: &mut GraphPreparedSourceSet,
    moves: &[(u32, u32)],
    blocks: u32,
    first: u64,
    claims: &[ClaimFillRow],
    sidechains: &mut [SidechainFillRow],
) {
    use runtime::FillPart::{Silence, Words};
    let quantum = set.envelope.quantum.0;
    let frames = quantum as usize;
    for block in 0..blocks {
        let Some(sample) = u64::from(block)
            .checked_mul(u64::from(quantum))
            .and_then(|offset| first.checked_add(offset))
        else {
            return;
        };
        if !set.driver.prime_block_at(sample, moves) {
            debug_assert!(false, "#1355 D5: a carried prime names its moved sources");
            return;
        }
        for row in claims {
            let Some(line) = runtime.track_delays.get_mut(row.line as usize) else {
                continue;
            };
            match set.played_planes(row.claim as usize) {
                Some((left, right)) => {
                    let [left, right] =
                        lanes_of([[left, &[]], [right, &[]]], row.lanes).map(|lane| lane[0]);
                    line.push(left, right);
                }
                None => line.push_silence(frames, row.lanes),
            }
        }
        for row in sidechains.iter_mut() {
            let Some(line) = runtime.delays.get_mut(row.line as usize) else {
                continue;
            };
            match set.played_planes(row.claim as usize) {
                Some((left, right)) => line.fill_head(Words(left), Words(right), &mut row.written),
                None => line.fill_head(Silence(frames), Silence(frames), &mut row.written),
            }
        }
    }
}

/// A warm plan's pending prime (#1355 D5), for its first block's source-read sample
/// `source_read_sample` (`S + O`, its offset including its lead): the prime starts its lead
/// before it, where its predecessor would have read, so the plan's sources then begin at
/// `S + O`. Run once, by the plan's first block or, for a plan displaced before that block, by its
/// successor's carry (#1355 MAJOR-1).
#[allow(clippy::too_many_arguments)]
fn replay_prime(
    runtime: &mut runtime::Runtime,
    set: &mut GraphPreparedSourceSet,
    moves: &[(u32, u32)],
    blocks: u32,
    source_read_sample: u64,
    claims: &[ClaimFillRow],
    sidechains: &mut [SidechainFillRow],
) {
    let lead = u64::from(blocks) * u64::from(set.envelope.quantum.0);
    match source_read_sample.checked_sub(lead) {
        Some(first) => run_prime(runtime, set, moves, blocks, first, claims, sidechains),
        None => debug_assert!(false, "#1355 D1: a warm offset includes its lead"),
    }
}
// REALTIME_POLICY_END

/// Install the live-send section of the hand-over this graph plan runs when it displaces its
/// predecessor (#1284 D2), after its [`GraphCarryProgram`], which names that predecessor.
///
/// Refused, leaving any earlier section in place, when the plan is not a graph plan, has no carry
/// program, a successor unit is not a live route of the plan, or a successor or predecessor unit
/// repeats. Predecessor units are checked against the predecessor at the swap block, where one that
/// does not resolve refuses the whole hand-over.
pub fn install_route_carry(
    plan: &mut PreparedRenderPlan,
    mut moves: Vec<GraphRouteMove>,
) -> Result<(), GraphCarryInstallError> {
    let executor = plan
        .executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .ok_or(GraphCarryInstallError::NotAGraphPlan)?;
    if executor.carry.is_none() {
        return Err(GraphCarryInstallError::NoRouteCarryProgram);
    }
    let mut successors = BTreeSet::new();
    let mut predecessors = BTreeSet::new();
    for step in &moves {
        if !executor.runtime.is_live_route(step.successor) {
            return Err(GraphCarryInstallError::RouteOutOfRange);
        }
        if !successors.insert(step.successor) || !predecessors.insert(step.predecessor) {
            return Err(GraphCarryInstallError::DuplicateRoute);
        }
    }
    moves.sort_unstable_by_key(|step| step.predecessor);
    let slots: Box<[runtime::RouteCarryCheck]> = moves
        .iter()
        .zip(0_u32..)
        .filter(|(step, _)| step.predecessor_follows)
        .map(|(_, index)| runtime::RouteCarryCheck::new(index))
        .collect();
    executor.route_checks = (!slots.is_empty()).then(|| {
        Box::new(RouteCarryChecks {
            slots,
            pending: false,
        })
    });
    executor.carry_routes = moves
        .into_iter()
        .map(runtime::RouteCarryStep::new)
        .collect();
    Ok(())
}

/// The side table of a live-send section (#1652 D3, MINOR-3): one slot per move whose
/// predecessor row is a following route, in move order, and whether the swap block left a
/// comparison in one of them for the successor's first block.
pub(crate) struct RouteCarryChecks {
    slots: Box<[runtime::RouteCarryCheck]>,
    pending: bool,
}

/// The bytes a strip-delay section of `moves` lines retains in its plan (#1284): its move table.
/// `None` on overflow. The one place the charge is spelled, so preparation's admission and
/// [`carry_program_retained_bytes`] cannot disagree.
#[must_use]
pub fn strip_delay_carry_retained_bytes(moves: usize) -> Option<u64> {
    u64::try_from(moves.checked_mul(core::mem::size_of::<GraphStripDelayMove>())?).ok()
}

/// The bytes a live-send section of `moves` routes retains in its plan (#1284, #1652 D3): its move
/// table and, when `following` of its moves have a following route as their predecessor row, its
/// side table of first-block comparison slots (MINOR-3; nothing with `following == 0`). `None` on
/// overflow; spelled once, as [`strip_delay_carry_retained_bytes`].
#[must_use]
pub fn route_carry_retained_bytes(moves: usize, following: usize) -> Option<u64> {
    let table = moves.checked_mul(core::mem::size_of::<runtime::RouteCarryStep>())?;
    let checks = if following == 0 {
        0
    } else {
        following
            .checked_mul(core::mem::size_of::<runtime::RouteCarryCheck>())?
            .checked_add(core::mem::size_of::<RouteCarryChecks>())?
    };
    u64::try_from(table.checked_add(checks)?).ok()
}

/// The session timeline's sample at `source_read_sample` in a graph plan's source set (#1641 D8):
/// [`GraphPreparedSourceSetDriver::timeline_at`], downcast as [`carry_program_retained_bytes`]
/// does. `None` for another plan, a plan with no source set or a driver with no timeline.
#[cfg(any(test, feature = "test-support"))]
pub fn test_only_timeline_at(
    plan: &mut PreparedRenderPlan,
    source_read_sample: u64,
) -> Option<i64> {
    graph_source_driver_mut(plan)?.timeline_at(source_read_sample)
}

/// The entries of the plan's carry tables that [`carry_program_retained_bytes`] charges, as
/// counts: `[source moves, input-section lane moves, strip-stage moves, effect-lane moves,
/// compensation-line moves]`. All zero for a plan without a carry program or for any other plan.
#[cfg(any(test, feature = "test-support"))]
pub fn test_only_carry_counts(plan: &mut PreparedRenderPlan) -> [usize; 5] {
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .map_or([0; 5], |executor| {
            [
                executor
                    .carry
                    .as_ref()
                    .map_or(0, |program| program.sources.len()),
                executor.carry_inputs.len(),
                executor.carry_strips.len(),
                executor.carry_effects.len(),
                executor.carry_delays.len(),
            ]
        })
}

/// Blocks the plan's bank chains skipped whole (issue #1107), summed over every bank unit; zero
/// for any other plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_bank_silent_skips(plan: &PreparedRenderPlan) -> u64 {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or(0, |executor| {
            executor.runtime.units.iter().fold(0_u64, |total, unit| {
                total.saturating_add(unit.silent_skips())
            })
        })
}

/// Blocks the plan's per-node live effects skipped their wet path settled bypassed (issue #2129
/// D7), summed over every op; zero for any other plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_node_bypass_skips(plan: &PreparedRenderPlan) -> u64 {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or(0, |executor| {
            executor.runtime.units.iter().fold(0_u64, |total, unit| {
                total.saturating_add(unit.bypass_skips())
            })
        })
}

/// Test-only (issue #2133): the silence flag the last rendered block left on each route
/// destination that is a plain sum (a submix input), in unit order; empty for any other plan. The
/// session Output writes the host's planes and has no flag.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_route_destination_silence(plan: &PreparedRenderPlan) -> Vec<bool> {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or_else(Vec::new, |executor| {
            executor.runtime.route_destination_silence()
        })
}

/// One pack group's bank-slot runs since bind (issue #2132), or one bank unit's that is in no
/// pack group. Evidence, read off the render thread.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TestOnlyBankSlotRuns {
    /// The row's first runtime unit. A row is one row of a pack group's lane chains (issue
    /// #2142), its units adjacent among the effect banks, or one unit in no group.
    pub first_unit: u32,
    /// Its last runtime unit, inclusive: `first_unit` for a unit in no group.
    pub last_unit: u32,
    /// The lanes per unit, `W`.
    pub width: u32,
    /// The effect slots of each unit of the row (builtin stages are not counted): one program,
    /// so one count for every unit of a row.
    pub effect_slots: u32,
    /// Bank-slot runs: one per unit per effect slot executed, none for a skipped chain.
    pub runs: u64,
    /// Blocks the group's chains skipped whole (issue #1107), summed over its units.
    pub skips: u64,
    /// Slot-blocks the group's banks skipped their wet path because every lane was settled
    /// bypassed (issue #2129), summed over its units; already left out of `runs`.
    pub bypass_skips: u64,
}

/// Every pack group's bank-slot runs (issue #2132), then, in unit order among them, every bank
/// unit in no group that has an effect slot; empty for any other plan. Read between renders, on
/// the render thread's side of a block boundary.
///
/// Not gated on `test-support`, unlike the other `test_only_` readers: the console bench reads it
/// (`bench console --bank-report`), and building the bench with `test-support` would put that
/// feature's render-path switches into the binary it measures. It reads counters every build
/// keeps (each chain's round trips and whole-chain skips) and adds no render-path state.
#[doc(hidden)]
#[must_use]
pub fn test_only_bank_slot_runs(plan: &mut PreparedRenderPlan) -> Vec<TestOnlyBankSlotRuns> {
    let narrow = |value: u64| u32::try_from(value).unwrap_or(u32::MAX);
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .map_or_else(Vec::new, |executor| {
            executor
                .runtime
                .bank_slot_runs()
                .into_iter()
                .map(
                    |[first, last, width, effect_slots, runs, skips, bypass_skips]| {
                        TestOnlyBankSlotRuns {
                            first_unit: narrow(first),
                            last_unit: narrow(last),
                            width: narrow(width),
                            effect_slots: narrow(effect_slots),
                            runs,
                            skips,
                            bypass_skips,
                        }
                    },
                )
                .collect()
        })
}

/// Test-only (issue #1782a; lane chains, issue #2142): one pack group of a graph plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TestOnlyPackGroup {
    /// Lanes per unit.
    pub width: u32,
    /// The group's lane chains in order: `chains[c][j]` is unit `j` of chain `c`, so every chain
    /// has the group's units per chain and position `p` is lane `p % width` of chain `p / width`
    /// in every row.
    pub chains: Vec<Vec<u32>>,
}

/// Test-only (issue #1782a): each pack group of a graph plan, in unit order; empty for any other
/// plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_pack_groups(plan: &PreparedRenderPlan) -> Vec<TestOnlyPackGroup> {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or_else(Vec::new, |executor| {
            executor
                .runtime
                .pack_groups()
                .into_iter()
                .map(|(width, chains)| TestOnlyPackGroup { width, chains })
                .collect()
        })
}

/// Test-only (issue #1782a): trade the strips at `(unit, lane)` `a` and `b` of one pack group, as
/// #1782b's packing move will at a block boundary. Call it between renders, on the render thread.
/// `false`, with nothing moved, when the plan is not a graph plan or the move is refused.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_swap_bank_lanes(
    plan: &mut PreparedRenderPlan,
    a: (usize, usize),
    b: (usize, usize),
) -> bool {
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .is_some_and(|executor| executor.runtime.swap_bank_lanes(a, b).is_ok())
}

/// Test-only (issue #1981): the lane moves the per-block packer made in a graph plan, packing or
/// homing; zero for any other plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_bank_lane_moves(plan: &PreparedRenderPlan) -> u64 {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or(0, |executor| executor.runtime.lane_moves())
}

/// Test-only (issue #1981): the swaps the per-block packer attempted in a graph plan, made or
/// refused; zero for any other plan.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_bank_pack_attempts(plan: &PreparedRenderPlan) -> u64 {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or(0, |executor| executor.runtime.pack_attempts())
}

/// Test-only (issue #2135): the tracks of a graph plan's pack groups whose strip is not at its home
/// lane, by the track bind placed at that home; empty for any other plan and under the identity map.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_displaced_tracks(plan: &PreparedRenderPlan) -> Vec<Box<str>> {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or_else(Vec::new, |executor| executor.runtime.displaced_tracks())
}

/// Test-only (issue #1981): the strips of a graph plan's pack groups not at their home lane; zero
/// for any other plan and under the identity map.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_displaced_strips(plan: &PreparedRenderPlan) -> u64 {
    plan.executor_any()
        .and_then(|any| any.downcast_ref::<GraphExecutor>())
        .map_or(0, |executor| executor.runtime.displaced_strips())
}

/// Bytes the plan's installed [`GraphCarryProgram`] retains; zero for a plan without one or for
/// any other plan.
pub fn carry_program_retained_bytes(plan: &mut PreparedRenderPlan) -> u64 {
    plan.executor_any_mut()
        .and_then(|any| any.downcast_mut::<GraphExecutor>())
        .map_or(0, |executor| {
            let sources = executor
                .carry
                .as_ref()
                .map_or(0, GraphCarryProgram::retained_bytes);
            let inputs = u64::try_from(core::mem::size_of_val::<[GraphLaneMove]>(
                &executor.carry_inputs,
            ))
            .unwrap_or(u64::MAX);
            let strips = u64::try_from(core::mem::size_of_val::<[GraphStripMove]>(
                &executor.carry_strips,
            ))
            .unwrap_or(u64::MAX);
            let slots = executor
                .carry_effects
                .iter()
                .try_fold(0_u64, |total, step| {
                    total.checked_add(
                        executor
                            .runtime
                            .effect_site_carry_slot_bytes(step.successor)?,
                    )
                })
                .unwrap_or(u64::MAX);
            let effects = effect_carry_retained_bytes(
                executor.carry_effects.len(),
                u64::try_from(executor.carry_scratch.len()).unwrap_or(u64::MAX),
                slots,
            )
            .unwrap_or(u64::MAX);
            let observers = executor
                .observer_carry
                .as_deref()
                .map_or(0, ObserverCarryProgram::retained_bytes);
            let delays =
                delay_carry_retained_bytes(executor.carry_delays.len()).unwrap_or(u64::MAX);
            let strip_delays = strip_delay_carry_retained_bytes(executor.carry_strip_delays.len())
                .unwrap_or(u64::MAX);
            let prime = prime_carry_retained_bytes(
                executor.prime_claims.len(),
                executor.prime_sidechains.len(),
            )
            .unwrap_or(u64::MAX);
            let routes = route_carry_retained_bytes(
                executor.carry_routes.len(),
                executor
                    .route_checks
                    .as_deref()
                    .map_or(0, |checks| checks.slots.len()),
            )
            .unwrap_or(u64::MAX);
            sources
                .saturating_add(delays)
                .saturating_add(strip_delays)
                .saturating_add(prime)
                .saturating_add(routes)
                .saturating_add(inputs)
                .saturating_add(strips)
                .saturating_add(effects)
                .saturating_add(observers)
        })
}

/// Why a source donation between two graph plans is refused (#1344 D4, D5).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphDonationRefusal {
    /// A plan is not a graph plan, or has no source set.
    NotAGraphPlan,
    /// A donor consumer has begun a block: it has played PCM the host was acked for.
    Consumed,
    /// A pair is out of range, names a vacant entry or repeats an index, or the two entries differ
    /// in channel count or quantum.
    Mismatch,
}

fn graph_source_driver(plan: &PreparedRenderPlan) -> Option<&dyn GraphPreparedSourceSetDriver> {
    let executor = plan.executor_any()?.downcast_ref::<GraphExecutor>()?;
    Some(executor.source_set.as_ref()?.driver.as_ref())
}

fn graph_source_driver_mut(
    plan: &mut PreparedRenderPlan,
) -> Option<&mut dyn GraphPreparedSourceSetDriver> {
    let executor = plan.executor_any_mut()?.downcast_mut::<GraphExecutor>()?;
    Some(executor.source_set.as_mut()?.driver.as_mut())
}

/// Check a donation of source consumers from `donor` to `successor` (#1344 D4, D5): for each
/// `(successor index, donor index)` pair, both entries occupied, of equal channel count and
/// quantum, no index repeated, and the donor's consumer has never begun a block. Read-only; both
/// plans are control-owned and neither is published.
pub fn can_donate_sources(
    successor: &PreparedRenderPlan,
    donor: &PreparedRenderPlan,
    pairs: &[(u32, u32)],
) -> Result<(), GraphDonationRefusal> {
    let (Some(into), Some(from)) = (graph_source_driver(successor), graph_source_driver(donor))
    else {
        return Err(GraphDonationRefusal::NotAGraphPlan);
    };
    for (index, &(successor_index, donor_index)) in pairs.iter().enumerate() {
        if pairs[..index]
            .iter()
            .any(|&(s, d)| s == successor_index || d == donor_index)
        {
            return Err(GraphDonationRefusal::Mismatch);
        }
        if from.source_has_begun_block(donor_index as usize) == Some(true) {
            return Err(GraphDonationRefusal::Consumed);
        }
    }
    if into.can_swap_sources(from, pairs) {
        Ok(())
    } else {
        Err(GraphDonationRefusal::Mismatch)
    }
}

/// Swap the source consumers [`can_donate_sources`] checked (#1344 D4): each `(successor index,
/// donor index)` pair exchanges its two entries' whole contents (ring, queued blocks, generation,
/// read position and held seek), so the donor ends with the successor's fresh rings. On the
/// control thread, on two plans neither of which render owns. Returns whether the pairs moved;
/// unchecked pairs that fail the driver's own check move nothing.
pub fn donate_sources(
    successor: &mut PreparedRenderPlan,
    donor: &mut PreparedRenderPlan,
    pairs: &[(u32, u32)],
) -> bool {
    let (Some(into), Some(from)) = (
        graph_source_driver_mut(successor),
        graph_source_driver_mut(donor),
    ) else {
        return false;
    };
    into.swap_sources(from, pairs)
}

/// Bytes of one entry of each executor table sized at bind (issue #936): `(active_units,
/// source_input_buffers)`.
fn executor_table_entry_bytes() -> (u64, u64) {
    (
        u64::try_from(core::mem::size_of::<u32>()).expect("active unit entry fits u64"),
        u64::try_from(core::mem::size_of::<(usize, u32)>()).expect("source input entry fits u64"),
    )
}

#[cfg(any(test, feature = "test-support"))]
thread_local! {
    /// Issue #936: units the render loop dispatched on this thread, over the blocks that
    /// completed, since the last reset.
    static UNIT_DISPATCHES: Cell<u64> = const { Cell::new(0) };
    /// Issue #936: `[active_units, source_input_buffers]` byte lengths of the executor most
    /// recently bound on this thread.
    static EXECUTOR_TABLE_BYTES: Cell<[u64; 2]> = const { Cell::new([0; 2]) };
}

/// One completed block's dispatch count: one thread-local add per block, never one per unit.
#[cfg(any(test, feature = "test-support"))]
fn test_only_count_unit_dispatches(dispatched: u64) {
    UNIT_DISPATCHES.with(|count| count.set(count.get().saturating_add(dispatched)));
}

/// Reset this thread's issue #936 dispatch count.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_unit_dispatch_reset() {
    UNIT_DISPATCHES.with(|count| count.set(0));
}

/// Units the executor's render loop dispatched on this thread since the last reset, summed over
/// the blocks that completed (issue #936). A plan that skips no unit dispatches every unit of its
/// census once per block.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_unit_dispatches() -> u64 {
    UNIT_DISPATCHES.with(Cell::get)
}

#[cfg(any(test, feature = "test-support"))]
fn test_only_record_executor_tables(active_units: &[u32], source_input_buffers: &[(usize, u32)]) {
    let bytes = |entries: usize, entry: usize| {
        u64::try_from(entries.saturating_mul(entry)).unwrap_or(u64::MAX)
    };
    EXECUTOR_TABLE_BYTES.with(|tables| {
        tables.set([
            bytes(active_units.len(), core::mem::size_of::<u32>()),
            bytes(
                source_input_buffers.len(),
                core::mem::size_of::<(usize, u32)>(),
            ),
        ]);
    });
}

/// `[dispatched-unit table, copied-claim table]` byte lengths of the graph executor most recently
/// bound on this thread (issue #936): what the runtime metadata estimate's
/// `active_unit_table_bytes` and `source_input_table_bytes` charge for.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_executor_table_bytes() -> [u64; 2] {
    EXECUTOR_TABLE_BYTES.with(Cell::get)
}

impl GraphExecutor {
    fn new(
        plan: PreparedGraphPlan,
        program: &program::ExecutionProgram,
        bindings: Vec<GraphNodeBinding>,
        observers: Vec<GraphNodeObserverBinding>,
        source_set: Option<GraphPreparedSourceSet>,
        planning: runtime::SequentialPlan,
    ) -> Self {
        let frames = plan.envelope.quantum.0 as usize;
        let sample_rate_hz = plan.envelope.sample_rate.0;
        let source_inputs: BTreeSet<_> = source_set
            .as_ref()
            .map(|set| {
                set.claims()
                    .iter()
                    .map(|claim| claim.node.clone())
                    .collect()
            })
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
        let parts = runtime::RuntimeParts::new(
            &plan.spec,
            plan.routes,
            plan.effects,
            plan.effect_controls,
            plan.effect_observations,
            plan.banks,
            plan.builtin_banks,
            observers,
            bindings,
            source_inputs,
            plan.track_delays,
            frames,
        )
        .with_route_controls(plan.route_controls.unwrap_or_default())
        .with_follow_route_controls(plan.follow_route_controls);
        // Issue #918: the claims a bank's gather may read in place, in claim order. Only a driver
        // that lends its played planes offers any, and `build_sequential` decides which of them
        // are bound in place.
        // `test_only_set_source_in_place_declined` offers none, which binds the path every claim
        // took before issue #918.
        let lent_claims: Vec<GraphNodeId> = source_set
            .as_ref()
            .filter(|set| set.driver.provides_played_planes())
            .map(|set| {
                set.claims()
                    .iter()
                    .map(|claim| claim.node.clone())
                    .collect()
            })
            .unwrap_or_default();
        #[cfg(any(test, feature = "test-support"))]
        let lent_claims = if runtime::test_only_source_in_place_declined() {
            Vec::new()
        } else {
            lent_claims
        };
        let (mut runtime, bind_identities) =
            runtime::build_sequential(program, &plan.spec, parts, frames, planning, &lent_claims);
        runtime.pin_strips(&plan.pinned_strips);
        // The copy loop fills exactly the claims that are not read in place.
        let source_input_buffers: Box<[(usize, u32)]> = source_input_buffers
            .iter()
            .copied()
            .filter(|&(claim, buffer)| !runtime.source_in_place(claim, buffer))
            .collect();
        // Issue #936: the render loop dispatches every unit but the inert ones, in unit order.
        let active_units: Box<[u32]> = (0..runtime.units.len())
            .filter(|&unit| !runtime.unit_inert(unit))
            .map(|unit| u32::try_from(unit).expect("a unit index fits the op index width"))
            .collect();
        #[cfg(any(test, feature = "test-support"))]
        test_only_record_executor_tables(&active_units, &source_input_buffers);
        let compensation_lines = compensation_lines_of(&plan.spec, program);
        Self {
            runtime,
            sample_rate_hz,
            source_set,
            source_input_buffers,
            active_units,
            identity: next_graph_identity(),
            carry: None,
            carry_inputs: Box::default(),
            carry_strips: Box::default(),
            carry_effects: Box::default(),
            carry_scratch: Box::default(),
            next_block_sample: 0,
            source_read_offset: 0,
            node_carry_refused: 0,
            observer_carry: None,
            carry_delays: Box::default(),
            compensation_lines,
            carry_strip_delays: Box::default(),
            prime_blocks: 0,
            prime_claims: Box::default(),
            prime_sidechains: Box::default(),
            prime_pending: false,
            carry_routes: Box::default(),
            route_checks: None,
            bind_identities,
            live_snapshot: None,
            jump_lengths: None,
            fade_arms: Box::default(),
            fade_arms_pending: 0,
            fade_arm_carry: Box::default(),
            fade_fired: GraphFadeArmWatch::default(),
            automation_program: None,
            strip_meters: true,
        }
    }
}

impl PreparedPlanExecutor for GraphExecutor {
    /// The bound graph's heap (#1074): the runtime ([`runtime::Runtime::retained_bytes`]), the
    /// source set (claims and the driver's box and heap), the dispatch and carry tables, the
    /// observer section, the bind-identity vectors (at capacity), the jump-length reader's box
    /// and cell `Arc`, the fade arms, and the stored-automation program's shared `Arc` in full,
    /// once ([`attach_automation_program`]). Not charged: the fired flags' `Arc`, which the
    /// host's plan inventory holds and charges (`GraphFadeArmWatch::heap_bytes`) and frees after
    /// the plan, and each control lane's `superseded` counter, the host's, shared by every lane
    /// and outliving the plan.
    fn retained_bytes(&self) -> u64 {
        use engine::realtime::retained::{arc, boxed, slice, str, vec};
        let program = self.automation_program.as_ref().map_or(0, |program| {
            arc(program).saturating_add(program.retained_bytes())
        });
        let source_set = self.source_set.as_ref().map_or(0, |set| {
            set.claims
                .iter()
                .fold(slice(&set.claims), |total, claim| {
                    total.saturating_add(claim.node.retained_bytes())
                })
                .saturating_add(
                    u64::try_from(core::mem::size_of_val(&*set.driver))
                        .unwrap_or(u64::MAX)
                        .saturating_add(set.driver.retained_bytes()),
                )
        });
        let carry = self
            .carry
            .as_ref()
            .map_or(0, GraphCarryProgram::retained_bytes);
        let observer_carry = self.observer_carry.as_deref().map_or(0, |program| {
            boxed(program)
                .saturating_add(slice(&program.moves))
                .saturating_add(slice(&program.joins))
                .saturating_add(slice(&program.taps))
                .saturating_add(slice(&program.record))
        });
        let route_checks = self.route_checks.as_deref().map_or(0, |checks| {
            boxed(checks).saturating_add(slice(&checks.slots))
        });
        let compensation_lines =
            self.compensation_lines
                .iter()
                .fold(vec(&self.compensation_lines), |total, line| {
                    total
                        .saturating_add(line.edge.retained_bytes())
                        .saturating_add(line.source.retained_bytes())
                        .saturating_add(line.destination.retained_bytes())
                });
        let identities = &self.bind_identities;
        let strip_delays = identities
            .strip_delays
            .iter()
            .fold(vec(&identities.strip_delays), |total, line| {
                total.saturating_add(str(&line.strip))
            });
        let live_routes = identities
            .live_routes
            .iter()
            .fold(vec(&identities.live_routes), |total, route| {
                total.saturating_add(str(&route.route_id))
            });
        let jump_lengths = self
            .jump_lengths
            .as_deref()
            .map_or(0, |reader| boxed(reader).saturating_add(arc(&reader.lane)));
        self.runtime
            .retained_bytes()
            .saturating_add(source_set)
            .saturating_add(slice(&self.source_input_buffers))
            .saturating_add(slice(&self.active_units))
            .saturating_add(carry)
            .saturating_add(slice(&self.carry_inputs))
            .saturating_add(slice(&self.carry_strips))
            .saturating_add(slice(&self.carry_effects))
            .saturating_add(slice(&self.carry_scratch))
            .saturating_add(observer_carry)
            .saturating_add(slice(&self.carry_delays))
            .saturating_add(compensation_lines)
            .saturating_add(slice(&self.carry_strip_delays))
            .saturating_add(slice(&self.prime_claims))
            .saturating_add(slice(&self.prime_sidechains))
            .saturating_add(slice(&self.carry_routes))
            .saturating_add(route_checks)
            .saturating_add(strip_delays)
            .saturating_add(live_routes)
            .saturating_add(jump_lengths)
            .saturating_add(slice(&self.fade_arms))
            .saturating_add(slice(&self.fade_arm_carry))
            .saturating_add(program)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn Any> {
        Some(self)
    }

    fn as_any(&self) -> Option<&dyn Any> {
        Some(self)
    }

    fn set_source_read_offset(&mut self, offset: u64) {
        self.source_read_offset = offset;
    }

    // REALTIME_POLICY_BEGIN
    /// Whether this plan, running, can be displaced by a warm successor whose prime replays
    /// `lead_blocks` blocks at the block from `block_start` (#1355 D4): its source set's check at
    /// the block's source-read sample `block_start + O`, where the successor's prime begins. A
    /// plan with no source set has no ring to prime. An overflowing sample is never ready.
    ///
    /// A warm plan adopted but not yet rendered still owes its own prime of `k` blocks, which the
    /// successor's carry replays first (#1355 MAJOR-1): the check then starts at its prime's
    /// sample `block_start + O - k·q` and walks both primes, `k + lead_blocks` blocks.
    fn prime_ready(&self, block_start: u64, lead_blocks: u32) -> bool {
        let Some(set) = self.source_set.as_ref() else {
            return true;
        };
        let owed = if self.prime_pending {
            self.prime_blocks
        } else {
            0
        };
        let Some((first_sample, blocks)) = block_start
            .checked_add(self.source_read_offset)
            .and_then(|sample| {
                sample.checked_sub(u64::from(owed) * u64::from(set.envelope.quantum.0))
            })
            .zip(lead_blocks.checked_add(owed))
        else {
            return false;
        };
        set.driver.prime_ready(first_sample, blocks)
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Move the sources the installed [`GraphCarryProgram`] names out of the predecessor graph
    /// plan, and copy the strip input lanes [`install_builtin_input_carry`] names (issue #1276),
    /// the fader, mute and matrix stages [`install_builtin_strip_carry`] names (issue #1277) and
    /// the console effect lanes [`install_effect_carry`] names (issue #1279), at the swap block: a
    /// bounded series of swaps and fixed-size lane copies, nothing allocated or freed.
    ///
    /// No program, or a predecessor that is not a graph plan, takes nothing (`NotRequested`). A
    /// predecessor of another identity, an input, strip or effect lane that does not resolve on
    /// either side (an effect lane also when the two lanes' layouts differ or the payload scratch
    /// is short), a source set missing on either side, or a driver that refuses the moves takes
    /// nothing and reports `PredecessorMismatch`. A console effect restore that refuses after the
    /// moves resolved leaves its lane as prepared and counts in [`carry_refusal_count`] (#1279
    /// D6); the hand-over still reports `Carried`.
    ///
    /// A warm successor ([`install_prime_carry`], #1355 D5) also pushes the predecessor's pending
    /// words into the lines its prime fills, and its first block replays the prime.
    fn adopt_predecessor(
        &mut self,
        predecessor: &mut dyn PreparedPlanExecutor,
        block_start: u64,
    ) -> CarryOutcome {
        let outcome = self.take_predecessor(predecessor, block_start);
        // #1355 D5: a warm successor is prepared against the plan render runs, so its carry, and
        // with it its prime, cannot be refused.
        debug_assert!(
            self.prime_blocks == 0 || outcome == CarryOutcome::Carried,
            "a warm successor's hand-over is refused: {outcome:?}"
        );
        outcome
    }

    /// The observers [`Self::adopt_predecessor`] carried (#1327 D3): empty until this plan has
    /// taken its swap block.
    fn observer_carries(&self) -> &[ObserverCarry] {
        self.observer_carry
            .as_deref()
            .map_or(&[], |observers| &observers.record[..observers.carried])
    }
    // REALTIME_POLICY_END

    fn can_prepare_source_seek(&self, source_index: usize) -> bool {
        self.source_set
            .as_ref()
            .is_some_and(|set| set.driver.can_prepare_source_seek(source_index))
    }

    fn prepare_source_seek(&mut self, source_index: usize, generation: u64, frame: u64) -> bool {
        self.source_set.as_mut().is_some_and(|set| {
            set.driver
                .prepare_source_seek(source_index, generation, frame)
        })
    }

    fn prepare_admitted_seeks(&mut self) -> usize {
        self.source_set
            .as_mut()
            .map_or(0, |set| set.driver.prepare_admitted_seeks())
    }

    fn copy_response_snapshot(
        &self,
        track_id: &str,
        _captured_sample: u64,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<u32, ResponseSnapshotError> {
        self.runtime
            .copy_response_snapshot(track_id, self.sample_rate_hz, sink)
    }

    #[doc(hidden)]
    fn invalidate_observers(&mut self) {
        self.runtime.invalidate_observers();
    }

    // REALTIME_POLICY_BEGIN
    /// Keep the block's live snapshot `S` for the drains `render` runs (#1504 D1): a store of one
    /// word, nothing allocated.
    fn begin_live_block(&mut self, snapshot: LiveSnapshot) {
        self.live_snapshot = Some(snapshot);
    }

    /// Keep the block's strip meter level for `observe_unit` (#882): a store of one byte.
    fn set_strip_meter_level(&mut self, on: bool) {
        self.strip_meters = on;
    }
    // REALTIME_POLICY_END

    // REALTIME_POLICY_BEGIN
    /// Render one block into `output`, whose two planes are the session Output op's storage for
    /// the block (issue #916). Nothing is copied out of the arena afterwards: the Output op, and a
    /// folded chain whose master it is, write the planes directly.
    ///
    /// # What `output` holds when this returns
    ///
    /// * **`Ok`:** each plane's `frames` words are this block's master. Nothing past `frames` is
    ///   written, so a `plane_stride` wider than the block keeps its padding.
    /// * **No live snapshot: untouched.** A block whose
    ///   [`PreparedPlanExecutor::begin_live_block`] did not run is refused with `InvalidEnvelope`
    ///   before any drain, because no drain falls back to a default snapshot (#1504 D2).
    ///   `PreparedRenderPlan` calls it before every block.
    /// * **Envelope rejection: untouched.** A non-stereo `output` is refused with
    ///   `Buffer(InvalidPlane)`. A plane that is not exactly `arena.frames()` words is refused with
    ///   `InvalidEnvelope`. Both refusals come before any observer boundary, source work or unit.
    ///   `PreparedRenderPlan::render_inner` already refuses the same mismatch as `OutputShape`
    ///   before it calls this, so this check is belt and braces.
    /// * **Executor-level failure: all `+0.0`.** This covers the source set failing in
    ///   `begin_block` or `copy_track_input`, and a unit or an observer failing inside the unit
    ///   loop. By then the Output op or a folded master may have written part of the block, so
    ///   both planes are filled with `+0.0` before the error returns, and a host that ignores the
    ///   error plays silence. The fill is on the failure paths only; a successful block never
    ///   writes a plane twice.
    fn render(
        &mut self,
        _arena: &mut BufferArena,
        mut output: PlanarBufferMut<'_>,
        time: engine::realtime::RenderTime,
    ) -> Result<ExecutorBlockReport, RenderError> {
        let Self {
            runtime,
            sample_rate_hz: _,
            source_set,
            source_input_buffers,
            active_units,
            identity: _,
            carry,
            carry_inputs: _,
            carry_strips: _,
            carry_effects: _,
            carry_scratch: _,
            next_block_sample,
            source_read_offset,
            node_carry_refused: _,
            observer_carry: _,
            carry_delays: _,
            compensation_lines: _,
            carry_strip_delays: _,
            prime_blocks,
            prime_claims,
            prime_sidechains,
            prime_pending,
            carry_routes,
            route_checks,
            bind_identities: _,
            live_snapshot,
            jump_lengths,
            fade_arms,
            fade_arms_pending,
            fade_arm_carry: _,
            fade_fired,
            automation_program: _,
            strip_meters,
        } = self;
        let strip_meters = *strip_meters;
        // One block's snapshot: taken here, so the next block cannot reuse it.
        let Some(live) = live_snapshot.take() else {
            return Err(RenderError::InvalidEnvelope);
        };
        // #1639 D2: the plan's jump lengths, read once at block entry and held for the block.
        if let Some(jump_lengths) = jump_lengths.as_deref_mut() {
            jump_lengths.begin_block(live);
        }
        #[cfg(any(test, feature = "test-support"))]
        BLOCK_JUMP_LENGTHS.with(|held| held.set(jump_lengths.as_deref().map(|reader| reader.held)));
        *next_block_sample = time
            .absolute_sample
            .saturating_add(runtime.arena.frames() as u64);
        let (left, right) = output.stereo_planes_mut()?;
        let Some(mut host) = runtime::HostMaster::new(left, right, runtime.arena.frames()) else {
            return Err(RenderError::InvalidEnvelope);
        };
        // #1396 D1: the source-read sample, `ΣP` ahead of the render clock.
        let block_source_read_sample = time.absolute_sample.checked_add(*source_read_offset);
        let source_validity = if let Some(source_set) = source_set.as_mut() {
            let Some(source_read_sample) = block_source_read_sample else {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                return Err(RenderError::TimeOverflow);
            };
            // #1355 D5: a warm successor's first block replays its prime first, from the sample
            // its predecessor would have read (`S + O`, its own offset less the lead), so its
            // sources then begin at `S + O + P`.
            if core::mem::take(prime_pending) {
                replay_prime(
                    runtime,
                    source_set,
                    carry.as_ref().map_or(&[][..], |program| &program.sources),
                    *prime_blocks,
                    source_read_sample,
                    prime_claims,
                    prime_sidechains,
                );
            }
            if let Err(error) =
                source_set.begin_block(source_read_sample, source_set.envelope.quantum.0)
            {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                return Err(error);
            }
            // A claim bound in place is not in this list: its bank's gather reads the played
            // block's planes (issue #918), or nothing reads it.
            for &(claim, buffer) in source_input_buffers.iter() {
                #[cfg(any(test, feature = "test-support"))]
                runtime::test_only_count_source_copy();
                let (left, right) = runtime.buffer_mut(buffer);
                if let Err(error) = source_set.copy_track_input(claim, left, right) {
                    runtime.invalidate_observers_after_failure(time.absolute_sample);
                    host.silence();
                    return Err(error);
                }
                // Issue #2133: the copied claim's silence, from the driver's own record.
                runtime.set_buffer_silent(buffer, source_set.played_block_silent(claim));
            }
            source_set.observation_validity()
        } else {
            GraphObservationValidity::CLEAR
        };
        // Issue #918: the played planes, shared for the rest of the block. Every release point of
        // the played block (`begin_block`, a seek's preparation, `end_block`, drop) takes the set
        // `&mut`, so none can run while a unit borrows a plane; the next `begin_block` is the
        // next render's.
        let sources = source_set.as_ref().map(|set| set as &dyn GraphSourcePlanes);
        // #1647a (R17-3): the block's node-time context, set on every builtin slot of a bank unit
        // before its chain runs.
        let block_time = GraphBlockTime::new(
            block_source_read_sample,
            u32::try_from(runtime.arena.frames()).unwrap_or(u32::MAX),
            jump_lengths.as_deref().map(|reader| reader.held),
            source_set
                .as_ref()
                .map(|set| &*set.driver as &dyn GraphPreparedSourceSetDriver),
        );
        // Issue #1288 D3: an added strip's fade-in fires at this block boundary, before any unit
        // runs; one integer test when no arm is pending.
        if *fade_arms_pending != 0 {
            step_fade_arms(
                runtime,
                fade_arms,
                fade_arms_pending,
                fade_fired,
                time.absolute_sample,
                sources,
            );
        }
        // #1652 D3: a six-word successor of a following predecessor compares at its first block,
        // under this block's snapshot and jump lengths, before its op runs.
        if let Some(checks) = route_checks.as_deref_mut()
            && core::mem::take(&mut checks.pending)
        {
            runtime.finish_route_carry(
                carry_routes,
                &mut checks.slots,
                live,
                block_time.jump_lengths(),
            );
        }
        // Issue #1981: pack each pack group's running strips into fewer banks, once, before any
        // unit runs (never between units: `observe_unit` interleaves with `execute`). One load
        // per group when nothing is packable or displaced; nothing in a plan with no group.
        runtime.pack_lanes();
        // Issue #936: only the units bind found to do work. An inert unit's `execute` and
        // `observe_unit` return at once, so skipping it moves no bit.
        #[cfg(any(test, feature = "test-support"))]
        let mut dispatched = 0_u64;
        for unit in active_units.iter().map(|&unit| unit as usize) {
            #[cfg(any(test, feature = "test-support"))]
            {
                dispatched += 1;
            }
            if let Err(error) = runtime.execute(
                unit,
                time.absolute_sample,
                live,
                host.reborrow(),
                sources,
                &block_time,
            ) {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                return Err(error);
            }
            if let Err(error) = runtime.observe_unit(
                unit,
                time.absolute_sample,
                source_validity,
                &host,
                strip_meters,
            ) {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                return Err(error);
            }
        }
        #[cfg(any(test, feature = "test-support"))]
        test_only_count_unit_dispatches(dispatched);
        // Issue #1609: the block rendered without failure, so a result an observer held for it
        // may now be published.
        runtime.block_end_observers();
        // #1537 D3: the block's graph-wide underrun flag, returned with the block rather than
        // asked for afterwards.
        Ok(ExecutorBlockReport {
            source_underrun: source_validity.source_underrun,
        })
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

    fn bank_scatter_redirects(&self) -> u64 {
        self.runtime.scatter_redirects()
    }

    fn bank_route_folds(&self) -> u64 {
        self.runtime.route_folds()
    }

    fn bank_collapse_counters(&self) -> [u64; 2] {
        self.runtime.collapse_counters()
    }

    fn bank_collapse_transitions(&self) -> [u64; 3] {
        self.runtime.collapse_transitions()
    }

    fn force_mono_collapse_off(&mut self, forced: bool) {
        self.runtime.force_mono_collapse_off(forced);
    }

    fn arm_mono_collapse(&mut self, eligible: &dyn Fn(&str) -> bool) {
        self.runtime.arm_mono_collapse(eligible);
    }

    fn bank_shape(&self) -> [u64; 2] {
        self.runtime.units.iter().fold([0, 0], |mut total, unit| {
            let shape = unit.bank_shape();
            total[0] = total[0].saturating_add(shape[0]);
            total[1] = total[1].saturating_add(shape[1]);
            total
        })
    }

    fn symmetry_counters(&self) -> [u64; 2] {
        self.runtime.units.iter().fold([0, 0], |mut total, unit| {
            let counters = unit.symmetry_counters();
            total[0] = total[0].saturating_add(counters[0]);
            total[1] = total[1].saturating_add(counters[1]);
            total
        })
    }

    /// The per-unit form of the census, joined to its bind-time identity.
    ///
    /// The dynamic half comes from the same `RuntimeUnit::symmetry_counters` the census folds, so
    /// the rows and the totals cannot disagree -- summing `[eligible_lanes, lanes]` over these
    /// rows *is* `symmetry_counters`, and `the_half_mono_cohort_banks_like_a_uniform_one (which asserts rows-sum-to-census inline)` pins that.
    fn unit_eligibility(&self) -> Vec<PlanUnitEligibility> {
        self.runtime
            .identity
            .iter()
            .enumerate()
            .map(|(unit, identity)| PlanUnitEligibility {
                unit: u32::try_from(unit).unwrap_or(u32::MAX),
                banked: identity.banking.banked(),
                gathers_track_input: identity.banking.gathers_track_input(),
                armed_on_input: self.runtime.armed_on_input(unit),
                stages: identity.stages,
                upstream_of_seam_stages: identity.upstream_of_seam_stages,
                lane_tracks: identity.lane_tracks.clone(),
                // Issue #1782a: each home lane's strip, wherever it renders now.
                lane_eligible: self.runtime.lane_eligibility_of(unit).into_boxed_slice(),
            })
            .collect()
    }

    fn observation_binding_counts(&self) -> [u64; 3] {
        self.runtime
            .units
            .iter()
            .fold([0, 0, 0], |mut total, unit| {
                let counts = unit.observation_binding_counts();
                for (value, add) in total.iter_mut().zip(counts) {
                    *value = value.saturating_add(add);
                }
                total
            })
    }

    fn observation_retained_bytes(&self) -> u64 {
        self.runtime.units.iter().fold(0_u64, |total, unit| {
            total.saturating_add(unit.observation_retained_bytes() as u64)
        })
    }

    fn unread_source_frames(&self) -> u64 {
        self.source_set
            .as_ref()
            .map_or(0, |set| set.driver.unread_source_frames())
    }
}

impl GraphExecutor {
    // REALTIME_POLICY_BEGIN
    /// The hand-over [`PreparedPlanExecutor::adopt_predecessor`] documents.
    fn take_predecessor(
        &mut self,
        predecessor: &mut dyn PreparedPlanExecutor,
        block_start: u64,
    ) -> CarryOutcome {
        let Some(program) = self.carry.as_ref() else {
            return CarryOutcome::NotRequested;
        };
        let Some(predecessor) = predecessor
            .as_any_mut()
            .and_then(|any| any.downcast_mut::<Self>())
        else {
            // A program asks for state; a predecessor that is not a graph plan has none to give.
            return CarryOutcome::PredecessorMismatch;
        };
        if predecessor.identity != program.predecessor {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1782a: every move table below names a lane by the location bind gave it, and
        // drains a predecessor bank's lanes together, so both plans go back to their identity lane
        // maps first: a bounded series of lane swaps, none under the identity. A move back that
        // refuses is a defect, and then nothing is carried.
        if !predecessor.runtime.restore_home_lanes() || !self.runtime.restore_home_lanes() {
            return CarryOutcome::PredecessorMismatch;
        }
        // #1652 MINOR-2: a predecessor displaced before its own first block still holds the
        // comparisons its carry left. Take them first, as that block would have (under every
        // snapshot and its held jump lengths, as the carry applies pending values), so this
        // plan carries the state the predecessor's next block would have rendered from. (Not
        // gated in graph: two swaps with a following route need host-core's attach, #1652c.
        // Reachable when a render call adopts a plan and is then refused before it renders;
        // gated by host-core's `route_mute`
        // `a_successor_of_a_plan_that_never_rendered_takes_its_pending_comparison`.)
        if let Some(checks) = predecessor.route_checks.as_deref_mut()
            && core::mem::take(&mut checks.pending)
        {
            let lengths = predecessor
                .jump_lengths
                .as_deref()
                .map(|reader| reader.held);
            predecessor.runtime.finish_route_carry(
                &predecessor.carry_routes,
                &mut checks.slots,
                LiveSnapshot::ALL,
                lengths,
            );
        }
        // #1355 MAJOR-1: likewise a warm predecessor displaced before its first block still owes
        // its prime. Replay it at this block, the one it would have rendered first, before its
        // lines and sources are read or taken, so this plan carries the lines and ring positions
        // that block would have left. (The source set's absence leaves nothing to prime, as in
        // render.)
        if predecessor.prime_pending
            && let Some(set) = predecessor.source_set.as_mut()
        {
            let source_read_sample = block_start.checked_add(predecessor.source_read_offset);
            predecessor.prime_pending = false;
            match source_read_sample {
                Some(source_read_sample) => replay_prime(
                    &mut predecessor.runtime,
                    set,
                    predecessor
                        .carry
                        .as_ref()
                        .map_or(&[][..], |program| &program.sources),
                    predecessor.prime_blocks,
                    source_read_sample,
                    &predecessor.prime_claims,
                    &mut predecessor.prime_sidechains,
                ),
                None => debug_assert!(false, "#1355: a displaced prime's sample overflows"),
            }
        }
        // Issue #1276: every builtin-input move must resolve before anything moves, so a refused
        // hand-over moves nothing at all.
        if !self
            .runtime
            .can_carry_input_lanes(&mut predecessor.runtime, &self.carry_inputs)
        {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1277: likewise every strip move.
        if !self
            .runtime
            .can_carry_strip_lanes(&mut predecessor.runtime, &self.carry_strips)
        {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1279: likewise every console-effect move, and the scratch holds every payload.
        if !self.runtime.can_carry_effect_lanes(
            &mut predecessor.runtime,
            &self.carry_effects,
            self.carry_scratch.len(),
        ) {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1327: likewise every observer move.
        if let Some(observers) = self.observer_carry.as_deref()
            && !self
                .runtime
                .can_carry_observers(&mut predecessor.runtime, &observers.moves)
        {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1283: likewise every compensation-line move.
        if !self
            .runtime
            .can_carry_delay_lines(&predecessor.runtime, &self.carry_delays)
        {
            return CarryOutcome::PredecessorMismatch;
        }
        // #1355 D5: likewise every predecessor line a prime fill takes pending words from.
        if !prime_fills_resolve(
            &self.prime_claims,
            &self.prime_sidechains,
            &predecessor.runtime,
        ) {
            return CarryOutcome::PredecessorMismatch;
        }
        // Issue #1284: likewise every strip delay line and live send move.
        if !self
            .runtime
            .can_carry_strip_delays(&predecessor.runtime, &self.carry_strip_delays)
            || !self.runtime.can_carry_live_routes(
                &predecessor.runtime,
                &self.carry_routes,
                self.route_checks
                    .as_deref()
                    .map_or(&[][..], |checks| &checks.slots[..]),
            )
        {
            return CarryOutcome::PredecessorMismatch;
        }
        if !program.sources.is_empty() || program.timeline {
            let (Some(successor_set), Some(predecessor_set)) =
                (self.source_set.as_mut(), predecessor.source_set.as_mut())
            else {
                return CarryOutcome::PredecessorMismatch;
            };
            if !successor_set
                .driver
                .adopt_sources(&mut *predecessor_set.driver, &program.sources)
            {
                return CarryOutcome::PredecessorMismatch;
            }
        }
        self.runtime
            .carry_input_lanes(&mut predecessor.runtime, &self.carry_inputs);
        self.runtime
            .carry_strip_lanes(&mut predecessor.runtime, &self.carry_strips);
        // Issue #1392 D3: after the lane import, each carried fade arm takes its render state.
        if !self.fade_arm_carry.is_empty() {
            carry_fade_arms(
                &mut self.runtime,
                &mut self.fade_arms,
                &mut self.fade_arms_pending,
                &self.fade_fired,
                &predecessor.fade_arms,
                &self.fade_arm_carry,
            );
        }
        self.runtime
            .carry_delay_lines(&mut predecessor.runtime, &self.carry_delays);
        // #1355 D5, L3: each sidechain line's head is the predecessor's edge pending, then the
        // source track's claim pending, read before the strip-delay section moves that line.
        for row in &mut self.prime_sidechains {
            fill_sidechain_pending(row, &mut self.runtime, &predecessor.runtime);
        }
        self.runtime
            .carry_strip_delays(&mut predecessor.runtime, &self.carry_strip_delays);
        // #1355 D5, L2: each grown claim line takes the predecessor's pending words first.
        for row in &self.prime_claims {
            fill_claim_pending(row, &mut self.runtime, &predecessor.runtime);
        }
        // The prime itself needs the block's source-read sample: the first block replays it.
        self.prime_pending = self.prime_blocks != 0;
        let mut no_checks = [];
        let checks = self.route_checks.as_deref_mut();
        let pending = self.runtime.carry_live_routes(
            &mut predecessor.runtime,
            &self.carry_routes,
            checks.map_or(&mut no_checks[..], |checks| &mut checks.slots[..]),
        );
        if let Some(checks) = self.route_checks.as_deref_mut() {
            checks.pending = pending;
        }
        let refused = self.runtime.carry_effect_lanes(
            &mut predecessor.runtime,
            &self.carry_effects,
            &mut self.carry_scratch,
            predecessor.next_block_sample,
            self.observer_carry
                .as_deref_mut()
                .map_or(&mut [][..], |observers| &mut observers.taps[..]),
        );
        self.node_carry_refused = self.node_carry_refused.saturating_add(refused);
        if let Some(observers) = self.observer_carry.as_deref_mut() {
            // The meters' observers first, then the effect taps, which the effect section above
            // already brought up to date with the predecessor's pending subscriptions (#1280 D3).
            let meters = self.runtime.carry_observers(
                &mut predecessor.runtime,
                &observers.moves,
                &observers.joins,
                &mut observers.record,
            );
            let taps = self.runtime.carry_taps(
                &mut predecessor.runtime,
                &observers.taps,
                observers.record.get_mut(meters..).unwrap_or(&mut []),
            );
            observers.carried = meters + taps;
        }
        CarryOutcome::Carried
    }
    // REALTIME_POLICY_END
}

#[cfg(test)]
mod tests {
    use super::*;

    use conformance::DualAccumulatorDelayFactory;
    use effect_contract::{
        BankWidth, EffectDescriptor, EffectId, EffectProcessBlock, EffectQuality,
        InitialParameterValue, LinkMode, LinkModeSet, NativeEffectFactory, ParameterChannel,
        PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectLimits, PrepareEffectRequest,
        PreparedPorts, PreparedSidechainPort, ProcessReport, ResetKind, StatePayloadError,
        StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    };
    use engine::{LAUNCH_SAMPLE_RATES, QuantumFrames};
    use std::sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    };

    struct Noop;
    impl GraphRuntimeProcessor for Noop {
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        fn retained_bytes(&self) -> u64 {
            0
        }
        fn as_any(&self) -> &dyn Any {
            self
        }
        fn into_any(self: Box<Self>) -> Box<dyn Any> {
            self
        }
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
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::arc(&self.order)
        }
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
    static SUM_PORTS: [PortDescriptor; 3] = [
        PortDescriptor {
            id: SUM_MAIN_IN,
            role: PortRole::MainInput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptor {
            id: SUM_MAIN_OUT,
            role: PortRole::MainOutput,
            required: true,
            layout: PortLayout::DualMonoPlanar,
        },
        PortDescriptor {
            id: SUM_SIDECHAIN,
            role: PortRole::SidechainInput,
            required: false,
            layout: PortLayout::DualMonoPlanar,
        },
    ];
    static SUM_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        id: SUM_ID,
        display_name: "Sidechain sum fixture",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        lane_link: false,
        parameters: &[],
        ports: &SUM_PORTS,
        qualities: &[],
        tail_and_rest: |_, _| effect_contract::NodeTailBound {
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
        },
        observations: &[],
    };

    /// Reads no prepared value, so holds none (issue #1461).
    struct SidechainSum;
    impl PreparedNativeEffect for SidechainSum {
        fn retained_bytes(&self) -> u64 {
            0
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
                maximum_mask_bytes: 0,
                largest_allocation_bytes: 16,
            }),
            None
        );
        assert_eq!(
            estimate, before,
            "overflow cannot partially mutate the report"
        );
    }

    #[test]
    fn scalar_owner_resource_overflow_leaves_the_graph_estimate_unchanged() {
        let mut estimate = empty_estimate();
        estimate.graph_metadata_bytes = 3;
        estimate.incremental_plan_bytes = 5;
        estimate.session_plus_plan_bytes = 7;
        let before = estimate.clone();
        assert_eq!(
            estimate.checked_add_scalar_owners(GraphScalarOwnerResourceEstimate {
                total_bytes: u64::MAX,
                largest_allocation_bytes: 64,
            }),
            None
        );
        assert_eq!(
            estimate, before,
            "overflow cannot partially mutate the report"
        );
    }

    #[test]
    fn runtime_metadata_charge_scales_with_emitted_ops_and_refuses_overflow() {
        let emitted = 3_u64;
        let resource = GraphRuntimeMetadataResourceEstimate::checked_for(emitted)
            .expect("checked runtime metadata");
        // Issue #936 and #1609: the executor's two bind-sized tables and the block-end unit table
        // each hold one entry per emitted op, so they scale with the op count.
        let doubled = GraphRuntimeMetadataResourceEstimate::checked_for(2 * emitted)
            .expect("checked runtime metadata");
        for (table, twice) in [
            (
                resource.active_unit_table_bytes,
                doubled.active_unit_table_bytes,
            ),
            (
                resource.source_input_table_bytes,
                doubled.source_input_table_bytes,
            ),
            (
                resource.block_end_unit_table_bytes,
                doubled.block_end_unit_table_bytes,
            ),
        ] {
            assert!(table > 0, "a table is charged for emitted ops");
            assert_eq!(twice, 2 * table, "one entry per emitted op");
        }

        let mut estimate = empty_estimate();
        estimate
            .checked_add_runtime_metadata(resource)
            .expect("runtime metadata fold");
        assert_eq!(estimate.graph_metadata_bytes, resource.total_bytes);
        assert_eq!(estimate.incremental_plan_bytes, resource.total_bytes);
        assert_eq!(estimate.session_plus_plan_bytes, resource.total_bytes);
        assert_eq!(
            estimate.largest_allocation_bytes,
            resource.largest_allocation_bytes
        );

        assert_eq!(
            GraphRuntimeMetadataResourceEstimate::checked_for(u64::MAX),
            None,
            "the bounded emitted-op multiplication refuses overflow"
        );
        let before = estimate.clone();
        estimate.graph_metadata_bytes = u64::MAX;
        let overflow_before = estimate.clone();
        assert!(estimate.checked_add_runtime_metadata(resource).is_none());
        assert_eq!(estimate, overflow_before);
        assert_ne!(overflow_before, before);
    }

    fn binding_plan() -> (PreparedGraphPlan, GraphRuntimeBindings, GraphNodeId) {
        let (parts, bindings, input) = binding_plan_parts();
        (PreparedGraphPlan::new(parts), bindings, input)
    }

    fn binding_plan_parts() -> (PreparedGraphPlanParts, GraphRuntimeBindings, GraphNodeId) {
        let input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("track").expect("ID"),
            stage: TrackStage::Input,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(1),
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let required = vec![input.clone(), output.clone()];
        let graph_nodes = vec![
            GraphNode {
                id: input.clone(),
                latency: LatencySamples(0),
                bound: NodeTailBound::ZERO,
            },
            GraphNode {
                id: output.clone(),
                latency: LatencySamples(0),
                bound: NodeTailBound::ZERO,
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
            PreparedGraphPlanParts {
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
                track_delays: Vec::new(),
                effects: Vec::new(),
                effect_controls: Vec::new(),
                effect_observations: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers: Vec::new(),
            },
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
                    backend: Backend::Simd4,
                    members: members.into_boxed_slice(),
                    processor: Box::<CountingIdentityBuiltin>::default(),
                    scratch: rack::AoSoaScratch::new(BankWidth::Four, 1).expect("W4 scratch"),
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

    /// Issue #169: a bank's window constrains colouring, so a plan that gains a bank after
    /// construction must re-derive its program -- otherwise `program()` describes the plan as it
    /// was and bind-time `lowered()` describes it as it is.
    ///
    /// The `differs` assertion is what keeps this honest: if attaching a bank stopped changing the
    /// colouring, the equality above would hold for the wrong reason.
    #[test]
    fn attaching_builtin_banks_re_derives_the_program_that_bind_will_use() {
        let bank = || GraphPreparedBuiltinBank {
            backend: Backend::Simd4,
            members: (0..4)
                .map(|lane| GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
                    stage: TrackStage::PostInputBuiltins,
                })
                .collect(),
            processor: Box::<CountingIdentityBuiltin>::default(),
            scratch: rack::AoSoaScratch::new(BankWidth::Four, 1).expect("W4 scratch"),
        };
        let (constructed, _, _) = four_track_builtin_plan(1_690, true, false);
        let (unbanked, _, _) = four_track_builtin_plan(1_691, false, false);
        let before = unbanked.program().cloned().expect("lowers");
        let attached = unbanked
            .with_builtin_banks(
                vec![bank()],
                GraphBuiltinBankResourceEstimate {
                    bank_count: 1,
                    ..GraphBuiltinBankResourceEstimate::default()
                },
            )
            .expect("attaches");
        assert_ne!(
            attached.program(),
            Some(&before),
            "the fixture must reach a colouring the bank window changes"
        );
        assert_eq!(attached.program(), constructed.program());
    }

    fn four_track_builtin_plan(
        plan_id: u64,
        banked: bool,
        id_ordered: bool,
    ) -> (PreparedGraphPlan, GraphRuntimeBindings, Arc<AtomicU64>) {
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(1),
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
                bound: NodeTailBound::ZERO,
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
                backend: Backend::Simd4,
                members: members.clone().into_boxed_slice(),
                processor: Box::<CountingIdentityBuiltin>::default(),
                scratch: rack::AoSoaScratch::new(BankWidth::Four, 1).expect("W4 scratch"),
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
            track_delays: Vec::new(),
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
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
                envelope,
                nodes,
                observers,
            },
            observer_order,
        )
    }

    /// One admitted four-lane fold, retaining both plan-owned and caller-owned observers on
    /// inputs. Observing the last bank slot would decline the fold and make fault tests vacuous.
    fn route_fold_recovery_plan() -> (PreparedGraphPlan, GraphRuntimeBindings, Arc<AtomicU64>) {
        let (mut plan, mut bindings, observations) = four_track_builtin_plan(221, true, false);
        let output = plan.sequential_schedule.pop().expect("output");
        let routes: Vec<_> = (0..4)
            .map(|lane| GraphNodeId::Route {
                route_id: StableGraphId::parse(&format!("route{lane}")).expect("route"),
            })
            .collect();
        let mut destinations = Vec::new();
        for edge in &mut plan.spec.edges {
            if let GraphEdgeId::RouteSource { route_id } = &edge.id {
                let route = GraphNodeId::Route {
                    route_id: route_id.clone(),
                };
                edge.destination.node = route.clone();
                destinations.push(GraphEdge {
                    id: GraphEdgeId::RouteDestination {
                        route_id: route_id.clone(),
                    },
                    source: GraphPortId {
                        node: route,
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: output.clone(),
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: "$".into(),
                });
            }
        }
        plan.spec.edges.extend(destinations);
        plan.spec.edges.sort_by(|a, b| a.id.cmp(&b.id));
        plan.spec
            .nodes
            .extend(routes.iter().cloned().map(|id| GraphNode {
                id,
                latency: LatencySamples(0),
                bound: NodeTailBound::ZERO,
            }));
        plan.spec.nodes.sort_by(|a, b| a.id.cmp(&b.id));
        plan.sequential_schedule.extend(routes.iter().cloned());
        plan.sequential_schedule.push(output.clone());
        plan.dependency_levels.pop();
        plan.dependency_levels.push(DependencyLevel {
            level: 2,
            nodes: routes.clone(),
        });
        plan.dependency_levels.push(DependencyLevel {
            level: 3,
            nodes: vec![output],
        });
        plan.routes = routes
            .into_iter()
            .map(|node| PreparedRoute {
                node,
                transform: RouteTransform {
                    gain: 1.0,
                    ll: 1.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
                gate: RouteGate::OPEN,
            })
            .collect();
        for observer in &mut bindings.observers {
            if let GraphNodeId::TrackStage { stage, .. } = &mut observer.node {
                *stage = TrackStage::Input;
            }
        }
        plan.observers.extend(bindings.observers.drain(..2));
        (plan, bindings, observations)
    }

    struct RecoverySource {
        sample: u64,
        begins: Arc<AtomicU64>,
        copies: Arc<AtomicU64>,
        drops: Arc<AtomicU64>,
    }
    impl Drop for RecoverySource {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl GraphPreparedSourceSetDriver for RecoverySource {
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::arc(&self.begins)
                + engine::realtime::retained::arc(&self.copies)
                + engine::realtime::retained::arc(&self.drops)
        }
        fn claim_count(&self) -> usize {
            1
        }
        fn begin_block(&mut self, first_sample: u64, _frames: u32) -> Result<(), RenderError> {
            self.sample = first_sample;
            self.begins.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn copy_track_input(
            &mut self,
            claim: usize,
            left: &mut [f32],
            right: &mut [f32],
        ) -> Result<(), RenderError> {
            assert_eq!(claim, 0);
            self.copies.fetch_add(1, Ordering::SeqCst);
            left.fill((self.sample + 1) as f32);
            right.fill(-((self.sample + 1) as f32));
            Ok(())
        }
    }

    /// Raw installation faults pass through the real public bind boundary. All metadata and
    /// original owners are returned, with the one-shot fault removed for a successful retry.
    #[test]
    fn route_fold_preflight_returns_original_owners_and_sources_for_retry() {
        use runtime::FoldFault;
        let faults = [
            (FoldFault::MissingRun, "graph.route_fold.mapping"),
            (FoldFault::MissingUnit, "graph.route_fold.mapping"),
            (FoldFault::WrongUnit, "graph.route_fold.mapping"),
            (FoldFault::UnbankedRun, "graph.route_fold.bank"),
            (FoldFault::MissingBank, "graph.route_fold.bank"),
            (FoldFault::OversizedMask, "graph.route_fold.mask"),
            (FoldFault::InactiveLane, "graph.route_fold.mask"),
            (FoldFault::ActiveWidth, "graph.route_fold.mask"),
            (FoldFault::MissingMaster, "graph.route_fold.master"),
            (FoldFault::BankedMaster, "graph.route_fold.master"),
            (FoldFault::WrongMaster, "graph.route_fold.master"),
        ];
        let addresses = |plan: &PreparedGraphPlan, bindings: &GraphRuntimeBindings| {
            let mut owners = vec![
                plan.spec.nodes.as_ptr() as usize,
                plan.routes.as_ptr() as usize,
            ];
            owners.extend(
                plan.builtin_banks
                    .iter()
                    .map(|bank| core::ptr::from_ref(&*bank.processor).cast::<()>() as usize),
            );
            owners.extend(
                bindings
                    .nodes
                    .iter()
                    .filter_map(|binding| binding.processor.as_ref())
                    .map(|processor| core::ptr::from_ref(&**processor).cast::<()>() as usize),
            );
            owners.extend(
                plan.observers
                    .iter()
                    .chain(bindings.observers.iter())
                    .map(|binding| core::ptr::from_ref(&*binding.observer).cast::<()>() as usize),
            );
            owners
        };
        for with_source in [false, true] {
            for (fault, expected) in faults {
                let (plan, mut bindings, observations) = route_fold_recovery_plan();
                let begins = Arc::new(AtomicU64::new(0));
                let copies = Arc::new(AtomicU64::new(0));
                let drops = Arc::new(AtomicU64::new(0));
                let source = with_source.then(|| {
                    let input = bindings.nodes.remove(0).node;
                    GraphPreparedSourceSet::new(
                        plan.envelope,
                        vec![GraphSourceInputClaim { node: input }],
                        GraphSourceSetResourceReport {
                            pcm_payload_already_charged_bytes: 0,
                            overhead_bytes: 0,
                            total_engine_owned_bytes: 0,
                            largest_allocation_bytes: 0,
                        },
                        Box::new(RecoverySource {
                            sample: 0,
                            begins: Arc::clone(&begins),
                            copies: Arc::clone(&copies),
                            drops: Arc::clone(&drops),
                        }),
                    )
                });
                let before = addresses(&plan, &bindings);
                runtime::inject_fold_fault(fault);
                let (returned, bindings, source) = match source {
                    Some(source) => {
                        let address = core::ptr::from_ref(&*source.driver).cast::<()>() as usize;
                        let failure = match plan.bind_with_source_set(bindings, source) {
                            Ok(_) => panic!("accepted {fault:?}"),
                            Err(failure) => failure,
                        };
                        assert_eq!(failure.code, expected, "{fault:?}");
                        assert_eq!(
                            core::ptr::from_ref(&*failure.source_set.driver).cast::<()>() as usize,
                            address
                        );
                        (*failure.plan, failure.bindings, Some(failure.source_set))
                    }
                    None => {
                        let failure = match plan.bind(bindings) {
                            Ok(_) => panic!("accepted {fault:?}"),
                            Err(failure) => failure,
                        };
                        assert_eq!(failure.code, expected, "{fault:?}");
                        (*failure.plan, failure.bindings, None)
                    }
                };
                assert_eq!(
                    addresses(&returned, &bindings),
                    before,
                    "{fault:?}: original owners"
                );
                assert_eq!(observations.load(Ordering::SeqCst), 0);
                assert_eq!(begins.load(Ordering::SeqCst), 0);
                assert_eq!(copies.load(Ordering::SeqCst), 0);
                assert_eq!(drops.load(Ordering::SeqCst), 0);
                let retry = match source {
                    Some(source) => returned
                        .bind_with_source_set(bindings, source)
                        .unwrap_or_else(|failure| panic!("retry: {}", failure.code)),
                    None => returned
                        .bind(bindings)
                        .unwrap_or_else(|failure| panic!("retry: {}", failure.code)),
                };
                assert_eq!(retry.bank_route_folds(), 4);
                assert_eq!(
                    render_three_blocks(retry).0.map(f32::to_bits),
                    [10.0_f32, -15.0, 20.0, -30.0, 30.0, -45.0].map(f32::to_bits)
                );
                assert_eq!(observations.load(Ordering::SeqCst), 12);
                assert_eq!(
                    begins.load(Ordering::SeqCst),
                    if with_source { 3 } else { 0 }
                );
                assert_eq!(
                    copies.load(Ordering::SeqCst),
                    if with_source { 3 } else { 0 }
                );
                assert_eq!(drops.load(Ordering::SeqCst), u64::from(with_source));
            }
        }
    }

    fn render_three_blocks(mut plan: PreparedRenderPlan) -> ([f32; 6], [u64; 2]) {
        let mut pcm = [0.0_f32; 6];
        for block in 0..3 {
            let output = PlanarBufferMut::try_new(&mut pcm[block * 2..block * 2 + 2], 2, 1, 1)
                .expect("output");
            plan.render(
                engine::realtime::RenderIo { output },
                engine::realtime::RenderTime {
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

    /// #1749: a lowering hint is checked against the ID it stands for, so a hint that names the
    /// wrong nodes, or no node at all, lowers and binds exactly as no hint does.
    #[test]
    fn a_wrong_lowering_hint_changes_no_program_and_no_bind() {
        let bind_code = |plan: PreparedGraphPlan, bindings| {
            plan.bind(bindings)
                .map(|_| ())
                .map_err(|failure| failure.code)
        };
        let (parts, bindings, _) = binding_plan_parts();
        let plain = PreparedGraphPlan::new(parts);
        assert!(plain.program().is_some(), "the unhinted plan lowers");
        let position = |id: &GraphNodeId| program::node_index(&plain.spec, id).expect("node");
        let schedule: Vec<u32> = plain.sequential_schedule.iter().map(position).collect();
        let ends: Vec<(u32, u32)> = plain
            .spec
            .edges
            .iter()
            .map(|edge| {
                (
                    position(&edge.source.node),
                    position(&edge.destination.node),
                )
            })
            .collect();
        let permuted = program::LoweringHint::new(
            schedule.iter().rev().copied().collect(),
            ends.iter()
                .map(|&(source, destination)| (destination, source))
                .collect(),
        );
        let out_of_range = program::LoweringHint::new(vec![7, u32::MAX], vec![(9, u32::MAX)]);
        let plain_program = plain.program().cloned();
        let plain_bind = bind_code(plain, bindings);
        assert_eq!(plain_bind, Ok(()));
        for hint in [permuted, out_of_range] {
            let (parts, bindings, _) = binding_plan_parts();
            let hinted = PreparedGraphPlan::new_hinted(parts, hint.clone());
            assert_eq!(hinted.program(), plain_program.as_ref(), "{hint:?}");
            assert_eq!(bind_code(hinted, bindings), plain_bind, "{hint:?}");
        }
    }

    #[test]
    fn borrowed_sorted_bind_validation_preserves_set_semantics() {
        // Caller order and duplicate/unsorted plan requirements retain the old set semantics.
        let (mut plan, mut bindings, _) = binding_plan();
        let output = plan
            .required_bindings
            .iter()
            .find(|node| matches!(node, GraphNodeId::Output { .. }))
            .cloned()
            .expect("output");
        let input = plan
            .required_bindings
            .iter()
            .find(|node| matches!(node, GraphNodeId::TrackStage { .. }))
            .cloned()
            .expect("input");
        plan.required_bindings = vec![output.clone(), input.clone(), output];
        bindings.nodes.reverse();
        match plan.bind(bindings) {
            Ok(_) => {}
            Err(failure) => panic!("set-equivalent binding rejected: {}", failure.code),
        }

        // A duplicated requirement for a bank-excluded member is filtered before set comparison.
        let (mut bank_plan, bank_bindings, _) = four_track_builtin_plan(57_001, true, false);
        let excluded = bank_plan.builtin_banks[0].members[0].clone();
        bank_plan
            .required_bindings
            .extend([excluded.clone(), excluded]);
        let bound = bank_plan
            .bind(bank_bindings)
            .unwrap_or_else(|failure| panic!("bank-excluded duplicate rejected: {}", failure.code));
        assert_eq!(
            render_three_blocks(bound).0.map(f32::to_bits),
            [10.0, -15.0, 20.0, -30.0, 30.0, -45.0].map(f32::to_bits)
        );

        // Equal observer values split across plan and caller ownership reject and return both.
        let (mut plan, mut bindings, input) = binding_plan();
        let order = Arc::new(AtomicU64::new(0));
        plan.observers.push(GraphNodeObserverBinding::new(
            input.clone(),
            7,
            Box::new(W4OrderObserver {
                lane: 0,
                order: Arc::clone(&order),
            }),
        ));
        bindings.observers.push(GraphNodeObserverBinding::new(
            input,
            7,
            Box::new(W4OrderObserver { lane: 0, order }),
        ));
        let failure = match plan.bind(bindings) {
            Ok(_) => panic!("value-equal split observers accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.observer");
        assert_eq!(failure.plan.observers.len(), 1);
        assert_eq!(failure.bindings.observers.len(), 1);
        let returned_plan = *failure.plan;
        let mut returned_bindings = failure.bindings;
        returned_bindings
            .observers
            .pop()
            .expect("returned caller observer");
        match returned_plan.bind(returned_bindings) {
            Ok(_) => {}
            Err(failure) => panic!("repaired observer bind rejected: {}", failure.code),
        }
    }

    #[test]
    fn binding_coverage_preserves_validation_and_ownership() {
        let assert_ok = |result: Result<PreparedRenderPlan, GraphBindFailure>| match result {
            Ok(_) => {}
            Err(failure) => panic!("unexpected bind failure: {}", failure.code),
        };
        let assert_source_ok =
            |result: Result<PreparedRenderPlan, GraphSourceBindFailure>| match result {
                Ok(_) => {}
                Err(failure) => panic!("unexpected source bind failure: {}", failure.code),
            };
        let source_err = |result: Result<PreparedRenderPlan, GraphSourceBindFailure>| match result {
            Ok(_) => panic!("unexpected successful source bind"),
            Err(failure) => failure,
        };
        let bind_err = |result: Result<PreparedRenderPlan, GraphBindFailure>| match result {
            Ok(_) => panic!("unexpected successful bind"),
            Err(failure) => failure,
        };
        let make_source = |envelope: RenderEnvelope, claims: Vec<GraphNodeId>| {
            let claim_count = claims.len();
            GraphPreparedSourceSet::new(
                envelope,
                claims
                    .into_iter()
                    .map(|node| GraphSourceInputClaim { node })
                    .collect(),
                GraphSourceSetResourceReport {
                    pcm_payload_already_charged_bytes: 0,
                    overhead_bytes: 0,
                    total_engine_owned_bytes: 0,
                    largest_allocation_bytes: 0,
                },
                Box::new(SilentSourceSetDriver {
                    claims: claim_count,
                }),
            )
        };
        let source_bindings = |bindings: GraphRuntimeBindings, input: &GraphNodeId| {
            let envelope = bindings.envelope;
            let nodes = bindings
                .nodes
                .into_iter()
                .filter(|binding| &binding.node != input)
                .collect();
            GraphRuntimeBindings {
                envelope,
                nodes,
                observers: bindings.observers,
            }
        };

        // Plain and split coverage both bind successfully.
        {
            let (plan, bindings, _) = binding_plan();
            assert_ok(plan.bind(bindings));
            let (plan, bindings, input) = binding_plan();
            let source = silent_source_set(bindings.envelope, input.clone());
            let split = source_bindings(bindings, &input);
            assert_source_ok(plan.bind_with_source_set(split, source));
        }

        // Missing and extra source coverage return every owner and can be repaired in place.
        {
            let (plan, bindings, input) = binding_plan();
            let source = make_source(bindings.envelope, vec![input.clone()]);
            let mut missing = source_bindings(bindings, &input);
            missing.nodes.clear();
            let failure = source_err(plan.bind_with_source_set(missing, source));
            assert_eq!(failure.code, "source.graph.binding_mismatch");
            assert_eq!(failure.bindings.nodes.len(), 0);
            let output = failure
                .plan
                .required_bindings
                .iter()
                .find(|node| matches!(node, GraphNodeId::Output { .. }))
                .cloned()
                .expect("output");
            let mut repaired = failure.bindings;
            repaired
                .nodes
                .push(GraphNodeBinding::new(output, Box::new(Noop)));
            assert_source_ok((*failure.plan).bind_with_source_set(repaired, failure.source_set));

            let (plan, bindings, input) = binding_plan();
            let envelope = bindings.envelope;
            let mut extra = source_bindings(bindings, &input);
            extra.nodes.push(GraphNodeBinding::new(
                GraphNodeId::Output {
                    output_id: StableGraphId::parse("extra").expect("ID"),
                },
                Box::new(Noop),
            ));
            let source = make_source(envelope, vec![input]);
            let failure = source_err(plan.bind_with_source_set(extra, source));
            assert_eq!(failure.code, "source.graph.binding_mismatch");
            assert_eq!(failure.bindings.nodes.len(), 2);
            let mut repaired = failure.bindings;
            repaired.nodes.pop();
            assert_source_ok((*failure.plan).bind_with_source_set(repaired, failure.source_set));
        }

        // Union equality never overrides overlap, duplicate, or empty-coverage validation.
        {
            let (plan, mut bindings, input) = binding_plan();
            let source = make_source(bindings.envelope, vec![input.clone()]);
            bindings
                .nodes
                .push(GraphNodeBinding::new(input, Box::new(Noop)));
            let failure = source_err(plan.bind_with_source_set(
                source_bindings(
                    bindings,
                    &GraphNodeId::Output {
                        output_id: StableGraphId::parse("never").expect("ID"),
                    },
                ),
                source,
            ));
            assert_eq!(failure.code, "source.graph.binding_mismatch");

            let (plan, mut bindings, duplicate) = binding_plan();
            bindings
                .nodes
                .push(GraphNodeBinding::new(duplicate, Box::new(Noop)));
            assert_eq!(bind_err(plan.bind(bindings)).code, "graph.plan.binding");

            let (plan, bindings, input) = binding_plan();
            let source = make_source(bindings.envelope, Vec::new());
            let mut empty = source_bindings(bindings, &input);
            empty.nodes.clear();
            assert_eq!(
                source_err(plan.bind_with_source_set(empty, source)).code,
                "source.graph.binding_mismatch"
            );
        }

        // Source mismatch has priority over observer/envelope, then observer over envelope.
        {
            let (plan, bindings, input) = binding_plan();
            let mut bad = source_bindings(bindings, &input);
            bad.envelope.sample_rate = engine::SampleRateHz(44_100);
            bad.observers.push(GraphNodeObserverBinding::new(
                GraphNodeId::Output {
                    output_id: StableGraphId::parse("main").expect("ID"),
                },
                1,
                Box::new(W4OrderObserver {
                    lane: 0,
                    order: Arc::new(AtomicU64::new(0)),
                }),
            ));
            let source = make_source(plan.envelope, Vec::new());
            assert_eq!(
                source_err(plan.bind_with_source_set(bad, source)).code,
                "source.graph.binding_mismatch"
            );

            let (plan, bindings, input) = binding_plan();
            let mut bad = source_bindings(bindings, &input);
            bad.observers.push(GraphNodeObserverBinding::new(
                GraphNodeId::CompensationDelay {
                    edge_id: Box::new(GraphEdgeId::TrackMain {
                        target: input.clone(),
                    }),
                },
                1,
                Box::new(W4OrderObserver {
                    lane: 0,
                    order: Arc::new(AtomicU64::new(0)),
                }),
            ));
            bad.envelope.sample_rate = engine::SampleRateHz(44_100);
            let source = make_source(plan.envelope, vec![input]);
            assert_eq!(
                source_err(plan.bind_with_source_set(bad, source)).code,
                "graph.plan.observer"
            );

            let (plan, bindings, input) = binding_plan();
            let mut bad = source_bindings(bindings, &input);
            bad.envelope.sample_rate = engine::SampleRateHz(44_100);
            let source = make_source(plan.envelope, vec![input]);
            assert_eq!(
                source_err(plan.bind_with_source_set(bad, source)).code,
                "graph.plan.envelope_mismatch"
            );
        }
    }

    #[test]
    fn level_major_w4_builtin_bank_is_analytic_for_three_blocks() {
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

        let analytic = [10.0, -15.0, 20.0, -30.0, 30.0, -45.0];
        assert_eq!(scalar.0.map(f32::to_bits), analytic.map(f32::to_bits));
        assert_eq!(banked.0.map(f32::to_bits), analytic.map(f32::to_bits));
        assert_eq!(scalar.1, [0, 0]);
        assert_eq!(banked.1, [3, 3]);
        assert_eq!(scalar_observers.load(Ordering::SeqCst), 12);
        assert_eq!(bank_observers.load(Ordering::SeqCst), 12);
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

    /// Issue #932: a hand-built plan that schedules a reader-less input (`b`) before a live claimed
    /// input (`a`) lets `a` take `b`'s freed slot, so `b`'s op writes `a`'s buffer before `a`'s
    /// input op and would overwrite `a`'s block-top copy. Bind refuses it whether `b` is a second
    /// claim or a processor-bound node; with both inputs in level 0 in node order `a` writes its
    /// slot first and the plan binds.
    #[test]
    fn bind_refuses_a_source_claim_whose_buffer_an_earlier_op_writes() {
        let track_input = |track: &str| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(track).expect("ID"),
            stage: TrackStage::Input,
        };
        let (live, dead) = (track_input("a"), track_input("b"));
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let set_levels = |plan: &mut PreparedGraphPlan, levels: &[&[&GraphNodeId]]| {
            plan.dependency_levels = levels
                .iter()
                .enumerate()
                .map(|(level, nodes)| DependencyLevel {
                    level: level as u64,
                    nodes: nodes.iter().map(|node| (*node).clone()).collect(),
                })
                .collect();
            plan.sequential_schedule = plan
                .dependency_levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect();
        };
        // `dead` before `live`, in two levels: the only order that shares the slot.
        let dead_first = |bound: &[&GraphNodeId]| {
            let (mut plan, mut bindings, _) = binding_plan();
            plan.spec.nodes = sorted_nodes(
                [&live, &dead, &output]
                    .map(|id| GraphNode {
                        id: id.clone(),
                        latency: LatencySamples(0),
                        bound: NodeTailBound::ZERO,
                    })
                    .to_vec(),
            );
            plan.spec.edges[0].source.node = live.clone();
            plan.required_bindings = vec![live.clone(), dead.clone(), output.clone()];
            bindings.nodes = bound
                .iter()
                .map(|node| GraphNodeBinding::new((*node).clone(), Box::new(Noop)))
                .collect();
            set_levels(&mut plan, &[&[&dead], &[&live], &[&output]]);
            (plan, bindings)
        };
        let source_set = |claims: &[&GraphNodeId]| {
            GraphPreparedSourceSet::new(
                RenderEnvelope {
                    sample_rate: engine::SampleRateHz(48_000),
                    quantum: QuantumFrames(1),
                    output_channels: core::num::NonZeroUsize::new(2).expect("two"),
                },
                claims
                    .iter()
                    .map(|node| GraphSourceInputClaim {
                        node: (*node).clone(),
                    })
                    .collect(),
                GraphSourceSetResourceReport {
                    pcm_payload_already_charged_bytes: 0,
                    overhead_bytes: 0,
                    total_engine_owned_bytes: 0,
                    largest_allocation_bytes: 0,
                },
                Box::new(SilentSourceSetDriver {
                    claims: claims.len(),
                }),
            )
        };

        // An unclaimed, processor-bound `b` writes the slot `a`'s claim then takes.
        let (plan, bindings) = dead_first(&[&dead, &output]);
        let failure = match plan.bind_with_source_set(bindings, source_set(&[&live])) {
            Ok(_) => panic!("claim on an earlier op's buffer accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.source_slot");

        // Two claims on one buffer.
        let (plan, bindings) = dead_first(&[&output]);
        let failure = match plan.bind_with_source_set(bindings, source_set(&[&live, &dead])) {
            Ok(_) => panic!("two claims on one buffer accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.scheduler.source_slot");
        assert_eq!(failure.source_set.claims().len(), 2);

        let mut plan = *failure.plan;
        set_levels(&mut plan, &[&[&live, &dead], &[&output]]);
        plan.bind_with_source_set(failure.bindings, failure.source_set)
            .unwrap_or_else(|failure| panic!("disjoint source slots: {}", failure.code));
    }

    #[test]
    fn structural_layout_rejects_node_level_edge_and_bank_corruptions() {
        for corruption in 0..9 {
            let (mut graph, bindings, input) = binding_plan();
            let line = |node: &GraphNodeId| SourceClaimLine {
                node: node.clone(),
                samples: LatencySamples(4),
            };
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
                // #1287 D1: a source-claim line the track delays do not hold.
                6 => graph = graph.with_source_claim_lines(vec![line(&input)]),
                7 => {
                    graph.track_delays.push(PreparedTrackDelay {
                        node: input.clone(),
                        left_samples: 4,
                        right_samples: 3,
                    });
                    graph = graph.with_source_claim_lines(vec![line(&input)]);
                }
                8 => {
                    let output = GraphNodeId::Output {
                        output_id: StableGraphId::parse("main").expect("ID"),
                    };
                    graph = graph.with_source_claim_lines(vec![line(&output)]);
                }
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

    /// `GraphNodeBinding::identity` acknowledges a required node without a host processor: the
    /// node is still listed (so a genuinely missing binding still fails), and the executor renders
    /// it with its own reduction kind, bit-for-bit as a do-nothing host processor did.
    #[test]
    fn identity_binding_acknowledges_without_a_processor() {
        struct Constant;
        impl GraphRuntimeProcessor for Constant {
            fn retained_bytes(&self) -> u64 {
                0
            }
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
                engine::realtime::RenderIo { output },
                engine::realtime::RenderTime { absolute_sample: 0 },
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
                engine::realtime::RenderIo { output },
                engine::realtime::RenderTime { absolute_sample: 9 },
            )
            .expect("render");
        assert_eq!(report.plan_id, 42);
        assert_eq!(report.next_absolute_sample, 10);
    }

    // ---- 50 random DAG sessions: the executor bit-identity gate (#98 F2) ----------------------

    /// Adds its sidechain when one is connected, and otherwise trims: the random corpus needs an
    /// effect that is legal both with and without a sidechain edge.
    /// Reads no prepared value, so holds none (issue #1461).
    struct OptionalSidechainSum;
    impl PreparedNativeEffect for OptionalSidechainSum {
        fn retained_bytes(&self) -> u64 {
            0
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        fn retained_bytes(&self) -> u64 {
            0
        }
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.left.len() {
                self.left = lane::softfma::unfused_multiply_add_via_f64(
                    self.coefficient,
                    self.left,
                    block.left[frame] * 0.5,
                );
                self.right = lane::softfma::unfused_multiply_add_via_f64(
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
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(16),
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let effect_metadata = PreparedEffectMetadata {
            descriptor: &SUM_DESCRIPTOR,
            sample_rate: 48_000,
            quantum: 16,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Connected {
                    id: SUM_SIDECHAIN,
                    required: false,
                },
            },
            latency: LatencySamples(0),
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
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
                        processor: Box::new(OptionalSidechainSum),
                        response_snapshot_declared: false,
                        native_id: "miso.test.optional-sidechain-sum",
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
                    gate: RouteGate::OPEN,
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
                gate: RouteGate::OPEN,
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
                            bound: NodeTailBound::ZERO,
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
            track_delays: Vec::new(),
            effects,
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        (
            plan,
            GraphRuntimeBindings {
                envelope,
                nodes: bindings,
                observers: Vec::new(),
            },
        )
    }

    fn render_blocks(plan: &mut PreparedRenderPlan, frames: usize, blocks: u64) -> Vec<u32> {
        let mut bits = Vec::with_capacity(blocks as usize * frames * 2);
        let mut samples = vec![0.0_f32; frames * 2];
        for block in 0..blocks {
            samples.fill(0.0);
            plan.render(
                engine::realtime::RenderIo {
                    output: PlanarBufferMut::try_new(&mut samples, 2, frames, frames)
                        .expect("output"),
                },
                engine::realtime::RenderTime {
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
        fn retained_bytes(&self) -> u64 {
            0
        }
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
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::arc(&self.0) + engine::realtime::retained::arc(&self.1)
        }
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            let mut sink = self.1.lock().expect("tap sink");
            sink.extend(block.left.iter().map(|sample| sample.to_bits()));
            Ok(())
        }
    }

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
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
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
            //
            // The other two builtin stages are listed and acknowledged with the identity, as a
            // compiled plan lists them; this fixture is about the three rack boundaries alone.
            let bindings = vec![
                GraphNodeBinding::new(
                    node(TrackStage::Input),
                    Box::new(SeededSource { state: 0x51ED_0007 }),
                ),
                GraphNodeBinding::identity(node(TrackStage::PostInputBuiltins)),
                GraphNodeBinding::new(node(TrackStage::PostFader), Box::new(Scale(0.375))),
                GraphNodeBinding::identity(node(TrackStage::PostMatrix)),
                GraphNodeBinding::new(output.clone(), Box::new(Noop)),
            ];
            let required = vec![
                node(TrackStage::Input),
                node(TrackStage::PostInputBuiltins),
                node(TrackStage::PostFader),
                node(TrackStage::PostMatrix),
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
                                bound: NodeTailBound::ZERO,
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
                track_delays: Vec::new(),
                effects: Vec::new(),
                effect_controls: Vec::new(),
                effect_observations: Vec::new(),
                banks: Vec::new(),
                builtin_banks: Vec::new(),
                observers,
            });
            (
                plan,
                GraphRuntimeBindings {
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
        // Two coloured slots for the track, plus the buffer the dedicated session output owns
        // since issue #916; before it, the output folded in place onto the matrix's slot.
        assert!(program.buffers <= 3, "the arena is coloured, not per-node");
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
                    live_snapshot: LiveSnapshot::ALL,
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

    /// The #98 F2 corpus: fifty seeded random DAGs -- stage chains, rack effects with sidechains,
    /// submixes, sends from arbitrary taps, non-trivial 2x2 routes and PDC on a quarter of the
    /// edges -- each bind and render eight blocks of non-silent PCM through the sequential
    /// executor.
    ///
    /// This was the differential gate between the sequential and native dependency-wave
    /// executors. The native executor was removed as production-unreachable, so what the corpus
    /// still proves is that every shape in it binds and renders; the cross-executor oracle it
    /// used to provide is gone, and the canonical fixtures and conformance suites carry that
    /// weight instead.
    #[test]
    fn fifty_random_dag_sessions_render_deterministic_nonsilent_pcm() {
        const FRAMES: usize = 16;
        const BLOCKS: u64 = 8;
        let mut nontrivial = 0_usize;
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
        }
        assert!(
            nontrivial >= 10,
            "the corpus must contain output fan-ins past the balanced/left-to-right divergence"
        );
    }

    /// A bound track input that plays a fixed planar sequence, one quantum per block.
    struct PlaneSource {
        left: Vec<f32>,
        right: Vec<f32>,
        cursor: usize,
    }
    impl GraphRuntimeProcessor for PlaneSource {
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::vec(&self.left)
                + engine::realtime::retained::vec(&self.right)
        }
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let frames = block.left.len();
            block
                .left
                .copy_from_slice(&self.left[self.cursor..self.cursor + frames]);
            block
                .right
                .copy_from_slice(&self.right[self.cursor..self.cursor + frames]);
            self.cursor += frames;
            Ok(())
        }
    }

    /// Issue #1201 gate 2: a submix's delay runs on its D9 sum, after the reduction.
    ///
    /// Three bound track inputs each reach the bus `Input` through an identity route; the route
    /// IDs are a permutation of the tracks, so route-ID order is not track order. The bus `Input`
    /// carries a 5-sample delay on both lanes and feeds the output alone. The oracle sums the three
    /// planes left to right in route-ID order, then shifts the sum by 5 samples; the 4-frame
    /// quantum makes the delay span two blocks. The planes mix magnitudes so that the sum's order
    /// shows in its bits, which the test checks rather than assumes.
    #[test]
    fn a_bus_delay_runs_on_the_sum_after_the_reduction() {
        const FRAMES: usize = 4;
        const BLOCKS: u64 = 16;
        const DELAY: usize = 5;
        let total = FRAMES * BLOCKS as usize;
        let node = |track: &str, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(track).expect("strip ID"),
            stage,
        };
        let bus = node("bus", TrackStage::Input);
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let port = |node: &GraphNodeId, kind: GraphPortKind| GraphPortId {
            node: node.clone(),
            kind,
            effect_port: None,
        };
        // (track, route, scale): t0 enters the sum second, t1 third and t2 first.
        let wiring = [
            ("t0", "r1", 1.0_f32),
            ("t1", "r2", 3.0e3),
            ("t2", "r0", -2.9e3),
        ];
        let mut state = 0x1201_u32;
        let mut plane = |scale: f32| -> Vec<f32> {
            (0..total)
                .map(|_| {
                    state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    let value = f32::from(((state >> 16) & 0xffff) as i16) / 3_276.8;
                    if value == 0.0 { scale } else { value * scale }
                })
                .collect()
        };
        let planes: Vec<[Vec<f32>; 2]> = wiring
            .iter()
            .map(|(_, _, scale)| [plane(*scale), plane(*scale * 0.75)])
            .collect();

        let mut nodes = vec![bus.clone(), output.clone()];
        let mut edges = vec![GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: output.clone(),
            },
            source: port(&bus, GraphPortKind::MainOutput),
            destination: port(&output, GraphPortKind::MainInput),
            path: "$.bus.main".to_owned(),
        }];
        let mut routes = Vec::new();
        let mut bindings = vec![GraphNodeBinding::new(output.clone(), Box::new(Noop))];
        let mut required = vec![output.clone()];
        for ((track, route, _), [left, right]) in wiring.iter().zip(&planes) {
            let input = node(track, TrackStage::Input);
            let route_id = StableGraphId::parse(route).expect("route ID");
            let route_node = GraphNodeId::Route {
                route_id: route_id.clone(),
            };
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteSource {
                    route_id: route_id.clone(),
                },
                source: port(&input, GraphPortKind::MainOutput),
                destination: port(&route_node, GraphPortKind::MainInput),
                path: "$.route.source".to_owned(),
            });
            edges.push(GraphEdge {
                id: GraphEdgeId::RouteDestination { route_id },
                source: port(&route_node, GraphPortKind::MainOutput),
                destination: port(&bus, GraphPortKind::MainInput),
                path: "$.route.destination".to_owned(),
            });
            routes.push(PreparedRoute {
                node: route_node.clone(),
                transform: RouteTransform {
                    gain: 1.0,
                    ll: 1.0,
                    lr: 0.0,
                    rl: 0.0,
                    rr: 1.0,
                },
                gate: RouteGate::OPEN,
            });
            bindings.push(GraphNodeBinding::new(
                input.clone(),
                Box::new(PlaneSource {
                    left: left.clone(),
                    right: right.clone(),
                    cursor: 0,
                }),
            ));
            required.push(input.clone());
            nodes.push(input);
            nodes.push(route_node);
        }
        edges.sort_by(|left, right| left.id.cmp(&right.id));
        nodes.sort();
        let levels = levels_for(&nodes, &edges);
        let schedule: Vec<GraphNodeId> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 1_201,
            spec: GraphSpec {
                nodes: sorted_nodes(
                    nodes
                        .into_iter()
                        .map(|id| GraphNode {
                            id,
                            latency: LatencySamples(0),
                            bound: NodeTailBound::ZERO,
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
            routes,
            track_delays: vec![PreparedTrackDelay {
                node: bus,
                left_samples: DELAY as u32,
                right_samples: DELAY as u32,
            }],
            effects: Vec::new(),
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let mut render = plan
            .bind(GraphRuntimeBindings {
                envelope,
                nodes: bindings,
                observers: Vec::new(),
            })
            .unwrap_or_else(|failure| panic!("bind: {}", failure.code));
        let bits = render_blocks(&mut render, FRAMES, BLOCKS);

        // The scalar oracle: the D9 sum in route-ID order (r0, r1, r2), then the delay.
        let by_route = |order: [usize; 3], lane: usize, frame: usize| {
            let [first, second, third] = order.map(|track| planes[track][lane][frame]);
            (first + second) + third
        };
        let route_order = [2, 0, 1];
        let mut order_shows = false;
        for block in 0..BLOCKS as usize {
            for lane in 0..2 {
                for frame in 0..FRAMES {
                    let at = block * FRAMES + frame;
                    let want = at.checked_sub(DELAY).map_or(0.0_f32, |source| {
                        let sum = by_route(route_order, lane, source);
                        order_shows |= sum.to_bits() != by_route([0, 1, 2], lane, source).to_bits();
                        sum
                    });
                    let got = bits[block * FRAMES * 2 + lane * FRAMES + frame];
                    assert_eq!(
                        got,
                        want.to_bits(),
                        "lane {lane} sample {at}: {} != {want}",
                        f32::from_bits(got)
                    );
                }
            }
        }
        assert!(
            order_shows,
            "the fixture must tell route-ID order from track order"
        );
    }

    /// One permanent-observer dispatch as its observer saw it (issue #900): handle, first sample,
    /// whether the resident lane was accepted, and the left and right words.
    type ObservationRow = (u64, u64, bool, Vec<u32>, Vec<u32>);

    /// Appends every dispatch to one shared log, so the log is in global dispatch order.
    /// `resident` accepts the resident lane wherever one is offered.
    struct LoggingObserver {
        handle: u64,
        resident: bool,
        log: Arc<std::sync::Mutex<Vec<ObservationRow>>>,
    }
    impl GraphRuntimeObserver for LoggingObserver {
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::arc(&self.log)
        }
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let bits = |plane: &[f32]| -> Vec<u32> { plane.iter().map(|x| x.to_bits()).collect() };
            self.log.lock().expect("observation log").push((
                self.handle,
                block.first_sample,
                false,
                bits(block.left),
                bits(block.right),
            ));
            Ok(())
        }
        fn observe_resident(
            &mut self,
            block: GraphResidentObservationBlock<'_>,
        ) -> Option<Result<(), RenderError>> {
            if !self.resident {
                return None;
            }
            let lanes = block.lane.width().lanes() as usize;
            let lane = block.lane.lane();
            let frames = block.lane.frames() as usize;
            let bits = |words: &[f32]| -> Vec<u32> {
                (0..frames)
                    .map(|frame| words[frame * lanes + lane].to_bits())
                    .collect()
            };
            self.log.lock().expect("observation log").push((
                self.handle,
                block.first_sample,
                true,
                bits(block.lane.left()),
                bits(block.lane.right()),
            ));
            Some(Ok(()))
        }
    }

    /// What one permanent-path render exposes (issue #900): PCM bits, the ordered observation
    /// log, `[observer, bank member]` dispatch accesses, `[planar, resident offered, resident
    /// accepted]` meter inputs, and the number of per-op `observe` walks.
    struct PermanentWalk {
        pcm: Vec<u32>,
        log: Vec<ObservationRow>,
        dispatch: [u64; 2],
        meter_inputs: [u64; 3],
        observe_calls: u64,
    }

    /// Bind `plan` with one [`LoggingObserver`] per `observed` node and render `blocks` blocks on
    /// the permanent path. `skip_disabled` forces the unconditional pre-#900 unit walk back on.
    fn permanent_walk(
        plan: PreparedGraphPlan,
        mut bindings: GraphRuntimeBindings,
        observed: &[(GraphNodeId, bool)],
        frames: usize,
        blocks: u64,
        skip_disabled: bool,
    ) -> PermanentWalk {
        let log = Arc::new(std::sync::Mutex::new(Vec::new()));
        bindings.observers = observed
            .iter()
            .enumerate()
            .map(|(handle, (node, resident))| {
                GraphNodeObserverBinding::new(
                    node.clone(),
                    handle as u64,
                    Box::new(LoggingObserver {
                        handle: handle as u64,
                        resident: *resident,
                        log: Arc::clone(&log),
                    }),
                )
            })
            .collect();
        let mut bound = plan
            .bind(bindings)
            .unwrap_or_else(|failure| panic!("permanent bind: {}", failure.code));
        runtime::test_only_observe_calls_reset(skip_disabled);
        test_only_observation_dispatch_reset();
        test_only_meter_input_reset(false);
        let pcm = render_blocks(&mut bound, frames, blocks);
        let walk = PermanentWalk {
            pcm,
            log: log.lock().expect("observation log").clone(),
            dispatch: test_only_observation_dispatch_counts(),
            meter_inputs: test_only_meter_input_counts(),
            observe_calls: runtime::test_only_observe_calls(),
        };
        runtime::test_only_observe_calls_reset(false);
        walk
    }

    /// Everything but the walk count must be the same bits whether or not unobserved units are
    /// skipped.
    fn assert_same_observation(skipped: &PermanentWalk, unconditional: &PermanentWalk, at: &str) {
        assert_eq!(skipped.pcm, unconditional.pcm, "{at}: rendered bits");
        assert_eq!(skipped.log, unconditional.log, "{at}: observation log");
        assert_eq!(
            skipped.dispatch[0], unconditional.dispatch[0],
            "{at}: observer accesses"
        );
        assert_eq!(
            skipped.meter_inputs, unconditional.meter_inputs,
            "{at}: meter inputs"
        );
    }

    /// Issue #900, gate 1: skipping the units that hold no observer moves no observed or rendered
    /// bit. Every placement renders twice -- once with the unconditional unit walk forced back
    /// on, once with the skip -- and the PCM, the globally ordered observation log, the observer
    /// accesses and the meter inputs must match. Only the per-op `observe` walks may fall, and
    /// they fall to exactly the ops of the units that hold an observer: a bank with one observed
    /// member is still walked whole, whichever member it is.
    #[test]
    fn permanent_observer_skip_moves_no_observed_or_rendered_bit() {
        const BLOCKS: u64 = 5;
        let stage = |lane: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("track{lane}")).expect("track id"),
            stage,
        };
        let input = |lane| stage(lane, TrackStage::Input);
        let member = |lane| stage(lane, TrackStage::PostInputBuiltins);
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output id"),
        };
        // Four input ops, one four-member builtin bank and the output op: nine ops, six units.
        let placements: [(Vec<(GraphNodeId, bool)>, u64); 5] = [
            (vec![(input(1), false)], 1),
            (vec![(member(3), false)], 4),
            (vec![(member(0), true)], 4),
            (
                vec![
                    (input(0), false),
                    (member(2), true),
                    (output.clone(), false),
                ],
                6,
            ),
            (
                (0..4)
                    .flat_map(|lane| [(input(lane), false), (member(lane), lane % 2 == 0)])
                    .chain([(output.clone(), false)])
                    .collect(),
                9,
            ),
        ];
        for (observed, walked_ops) in &placements {
            let at = format!("bank placement {observed:?}");
            let (plan, bindings, _) = four_track_builtin_plan(900, true, false);
            let unconditional = permanent_walk(plan, bindings, observed, 1, BLOCKS, true);
            let (plan, bindings, _) = four_track_builtin_plan(900, true, false);
            let skipped = permanent_walk(plan, bindings, observed, 1, BLOCKS, false);
            assert_same_observation(&skipped, &unconditional, &at);
            assert_eq!(
                skipped.log.len() as u64,
                observed.len() as u64 * BLOCKS,
                "{at}"
            );
            assert_eq!(unconditional.observe_calls, 9 * BLOCKS, "{at}");
            assert_eq!(unconditional.dispatch[1], 4 * BLOCKS, "{at}");
            assert_eq!(skipped.observe_calls, walked_ops * BLOCKS, "{at}");
            let bank_walked = observed.iter().any(|(node, _)| {
                matches!(
                    node,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::PostInputBuiltins,
                        ..
                    }
                )
            });
            assert_eq!(
                skipped.dispatch[1],
                if bank_walked { 4 * BLOCKS } else { 0 },
                "{at}"
            );
            let accepted = observed.iter().filter(|(_, resident)| *resident).count() as u64;
            assert_eq!(skipped.meter_inputs[2], accepted * BLOCKS, "{at}");
        }

        // The random-DAG corpus: stage chains with elided alias stages, sends, submixes,
        // sidechains and PDC, every unit a single op. A seeded third of the observable nodes
        // carry an observer; the ops walked must be exactly the ops those observers resolve to,
        // directly or through an alias's `after_op`.
        const FRAMES: usize = 16;
        let mut partial = 0_usize;
        let mut aliased = 0_usize;
        for seed in 0..50_u64 {
            let (plan, _) = random_dag_plan(seed);
            let lowered = plan.program().cloned().expect("lowered");
            let mut state = seed.wrapping_mul(0x2545_f491_4f6c_dd1d) | 1;
            let observed: Vec<(GraphNodeId, bool)> = plan
                .spec
                .nodes
                .iter()
                .filter(|node| {
                    matches!(
                        node.id,
                        GraphNodeId::TrackStage { .. } | GraphNodeId::Output { .. }
                    )
                })
                .filter(|_| xorshift(&mut state).is_multiple_of(3))
                .map(|node| (node.id.clone(), false))
                .collect();
            let walked: BTreeSet<u32> = observed
                .iter()
                .map(|(node, _)| {
                    let index = program::node_index(&plan.spec, node).expect("observed node");
                    lowered.node_op[index as usize].unwrap_or_else(|| {
                        aliased += 1;
                        lowered
                            .taps
                            .iter()
                            .find(|tap| tap.node == index)
                            .expect("an elided node has a tap")
                            .after_op
                    })
                })
                .collect();
            let ops = lowered.ops.len() as u64;
            if !walked.is_empty() && (walked.len() as u64) < ops {
                partial += 1;
            }
            let at = format!("seed {seed}");
            let (plan, bindings) = random_dag_plan(seed);
            let unconditional = permanent_walk(plan, bindings, &observed, FRAMES, BLOCKS, true);
            let (plan, bindings) = random_dag_plan(seed);
            let skipped = permanent_walk(plan, bindings, &observed, FRAMES, BLOCKS, false);
            assert_same_observation(&skipped, &unconditional, &at);
            assert_eq!(
                skipped.log.len() as u64,
                observed.len() as u64 * BLOCKS,
                "{at}"
            );
            assert_eq!(unconditional.observe_calls, ops * BLOCKS, "{at}");
            assert_eq!(skipped.observe_calls, walked.len() as u64 * BLOCKS, "{at}");
        }
        assert!(
            partial >= 25,
            "the corpus must mix observed and unobserved units"
        );
        assert!(aliased > 0, "the corpus must observe an elided alias stage");
    }

    /// Issue #900, gate 1: a plan with no observers makes zero `observe` calls and visits no bank
    /// member. The unconditional walk, forced back on, is the control: the same counter sees
    /// every op there.
    #[test]
    fn a_plan_without_observers_makes_zero_observe_calls() {
        const BLOCKS: u64 = 4;
        let (plan, bindings, _) = four_track_builtin_plan(901, true, false);
        let unconditional = permanent_walk(plan, bindings, &[], 1, BLOCKS, true);
        let (plan, bindings, _) = four_track_builtin_plan(901, true, false);
        let skipped = permanent_walk(plan, bindings, &[], 1, BLOCKS, false);
        assert_same_observation(&skipped, &unconditional, "bank plan");
        assert_eq!(
            (unconditional.observe_calls, unconditional.dispatch),
            (9 * BLOCKS, [0, 4 * BLOCKS])
        );
        assert_eq!(
            (
                skipped.observe_calls,
                skipped.dispatch,
                skipped.meter_inputs
            ),
            (0, [0, 0], [0, 0, 0])
        );
        for seed in 0..8_u64 {
            let (plan, bindings) = random_dag_plan(seed);
            let ops = plan.program().expect("lowered").ops.len() as u64;
            let unconditional = permanent_walk(plan, bindings, &[], 16, BLOCKS, true);
            let (plan, bindings) = random_dag_plan(seed);
            let skipped = permanent_walk(plan, bindings, &[], 16, BLOCKS, false);
            assert_same_observation(&skipped, &unconditional, &format!("seed {seed}"));
            assert_eq!(unconditional.observe_calls, ops * BLOCKS, "seed {seed}");
            assert_eq!(skipped.observe_calls, 0, "seed {seed}");
        }
    }

    // ---------------------------------------------------------------------------------------
    // Issue #140 A: the automation-span feed into the dynamic rack.
    // ---------------------------------------------------------------------------------------

    static GAIN_ID: EffectId = match EffectId::new("fixture.gain") {
        Ok(id) => id,
        Err(_) => panic!("static effect ID"),
    };
    static GAIN_PARAMETERS: [effect_contract::ParameterDescriptor; 1] =
        [effect_contract::ParameterDescriptor {
            id: match effect_contract::ParameterId::new(1) {
                Some(id) => id,
                None => panic!("nonzero"),
            },
            display_name: "Gain",
            name: "gain",
            display_unit: "x",
            unit: effect_contract::ParameterUnit::Linear,
            domain: effect_contract::ParameterDomain::Continuous,
            minimum: Some(0.0),
            maximum: Some(4.0),
            default_value: 1.0,
            mapping: effect_contract::ParameterMapping::Linear,
            automation_rate: effect_contract::AutomationRate::Block,
            channel_policy: effect_contract::ParameterChannelPolicy::PerLane,
            smoothing: effect_contract::SmoothingRule::None,
            smoothing_samples: 0,
            readable: true,
            automatable: true,
            enum_choices: &[],
            lattice: effect_contract::ParameterLattice::arithmetic(0.01, 2),
            prepared_reason: None,
        }];
    static GAIN_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        id: GAIN_ID,
        display_name: "Live gain fixture",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        lane_link: false,
        parameters: &GAIN_PARAMETERS,
        ports: &SUM_PORTS,
        qualities: &[],
        tail_and_rest: |_, _| effect_contract::NodeTailBound {
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
        },
        observations: &[],
    };

    /// A per-channel gain with a real, declared latency, so bypass has something to preserve.
    struct LiveGain {
        /// The one prepared value `process` reads (issue #1461).
        automation_capacity: u32,
        gain: [f32; 2],
        line: Vec<[f32; 2]>,
        latency: usize,
        invalid: u64,
    }
    impl LiveGain {
        fn new(metadata: PreparedEffectMetadata) -> Self {
            let latency = metadata.latency.0 as usize;
            Self {
                automation_capacity: metadata.automation_capacity,
                gain: [1.0, 1.0],
                line: vec![[0.0; 2]; latency],
                latency,
                invalid: 0,
            }
        }
    }
    impl PreparedNativeEffect for LiveGain {
        fn retained_bytes(&self) -> u64 {
            engine::realtime::retained::vec(&self.line)
        }
        fn reset(&mut self, _kind: ResetKind) {}
        fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
            let mut report = ProcessReport::default();
            // The same strictness every launch effect applies: a span at `first_sample`,
            // addressed to one lane, inside the declared capacity.
            for (index, span) in block.automation.iter().enumerate() {
                let lane = match span.channel {
                    ParameterChannel::Left => 0_usize,
                    ParameterChannel::Right => 1,
                    ParameterChannel::Both => {
                        report.invalid_spans += 1;
                        continue;
                    }
                };
                let valid = index < self.automation_capacity as usize
                    && span.parameter_index == 0
                    && span.sample == block.first_sample;
                if !valid {
                    report.invalid_spans += 1;
                    continue;
                }
                self.gain[lane] = span.value;
            }
            self.invalid = self.invalid.saturating_add(report.invalid_spans);
            for frame in 0..block.left.len() {
                let wet = [
                    block.left[frame] * self.gain[0],
                    block.right[frame] * self.gain[1],
                ];
                if self.latency == 0 {
                    block.left[frame] = wet[0];
                    block.right[frame] = wet[1];
                    continue;
                }
                let slot = &mut self.line[frame % self.latency];
                let held = *slot;
                *slot = wet;
                block.left[frame] = held[0];
                block.right[frame] = held[1];
            }
            report
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

    /// The gain fixture's prepared metadata, at the four-frame quantum.
    fn gain_metadata(latency: u64, automation_capacity: u32) -> PreparedEffectMetadata {
        PreparedEffectMetadata {
            descriptor: &GAIN_DESCRIPTOR,
            sample_rate: 48_000,
            quantum: 4,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::None,
            },
            latency: LatencySamples(latency),
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
            state_sizes: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_bytes: 0,
            automation_capacity,
        }
    }

    /// One track: source input -> one dynamic-rack effect -> output, at a four-frame quantum.
    ///
    /// `control` is the consumer half of the effect's live-control channel, or `None` for the
    /// live-control-free plan the workspace has always bound.
    fn live_control_effect_plan(
        latency: u64,
        control: Option<Box<EffectControlLane>>,
        source: Box<dyn GraphRuntimeProcessor>,
    ) -> PreparedRenderPlan {
        let input = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse("track").expect("ID"),
            stage: TrackStage::Input,
        };
        let effect_id = EffectNodeId {
            track_id: StableGraphId::parse("track").expect("ID"),
            rack: RackId::Dynamic,
            effect_id: StableGraphId::parse("gain").expect("ID"),
        };
        let effect_node = GraphNodeId::Effect(effect_id.clone());
        let output_node = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let metadata = gain_metadata(latency, 2);
        let main_edge = GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: effect_node.clone(),
            },
            source: GraphPortId {
                node: input.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: effect_node.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.live_controls.main".to_owned(),
        };
        let output_edge = GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: output_node.clone(),
            },
            source: GraphPortId {
                node: effect_node.clone(),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output_node.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.live_controls.output".to_owned(),
        };
        let schedule = vec![input.clone(), effect_node.clone(), output_node.clone()];
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 140,
            spec: GraphSpec {
                nodes: sorted_nodes(
                    schedule
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: LatencySamples(0),
                            bound: NodeTailBound::ZERO,
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges: vec![main_edge, output_edge],
            },
            sequential_schedule: schedule,
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![effect_node],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output_node.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: empty_estimate(),
            envelope,
            required_bindings: vec![input.clone(), output_node.clone()],
            routes: Vec::new(),
            track_delays: Vec::new(),
            effects: vec![GraphPreparedEffect {
                id: effect_id.clone(),
                metadata,
                processor: Box::new(LiveGain::new(metadata)),
                response_snapshot_declared: false,
                native_id: "miso.test.live-gain",
            }],
            effect_controls: match control {
                None => Vec::new(),
                Some(control) => vec![GraphEffectControlBinding {
                    node: effect_id,
                    control,
                }],
            },
            effect_observations: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let bindings = GraphRuntimeBindings {
            envelope,
            nodes: vec![
                GraphNodeBinding::new(input, source),
                GraphNodeBinding::new(output_node, Box::new(Noop)),
            ],
            observers: Vec::new(),
        };
        match graph.bind(bindings) {
            Ok(plan) => plan,
            Err(failure) => panic!("bindings: {}", failure.code),
        }
    }

    /// A source that writes the absolute sample index into the left plane and its negation into
    /// the right, so a rendered block is its own oracle for *when* something changed.
    ///
    /// Deliberately not named for the workspace's ramp type: `check-effect-runtime-policy.sh`
    /// pins that name's definition count at zero (#95), because a private copy of it inside an
    /// effect is exactly the divergence the rule exists to prevent.
    struct SampleIndexSource;
    impl GraphRuntimeProcessor for SampleIndexSource {
        fn retained_bytes(&self) -> u64 {
            0
        }
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.left.len() {
                let value = (block.first_sample as usize + frame) as f32;
                block.left[frame] = value;
                block.right[frame] = -value;
            }
            Ok(())
        }
    }

    fn render_live_control_blocks(
        plan: &mut PreparedRenderPlan,
        start_block: usize,
        blocks: usize,
    ) -> Vec<f32> {
        let mut collected = Vec::new();
        for block in start_block..start_block + blocks {
            let mut samples = [0.0_f32; 8];
            let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
            plan.render(
                engine::realtime::RenderIo { output },
                engine::realtime::RenderTime {
                    absolute_sample: (block * 4) as u64,
                },
            )
            .expect("render");
            collected.extend_from_slice(&samples);
        }
        collected
    }

    /// The gain fixture's lane: one `PerLane` parameter, so a `Left` and a `Right` cell.
    fn control_pair() -> (effect_contract::EffectCellWriter, Box<EffectControlLane>) {
        let (writer, lane) = effect_contract::effect_control_lane(
            effect_contract::EffectLaneSpec {
                shape: effect_contract::EffectLaneShape::from_descriptor(&GAIN_DESCRIPTOR, 0),
                bypass: false,
                observation_depth: 0,
                link: None,
            },
            &engine::realtime::LiveValuesSuperseded::new(),
        )
        .expect("the gain fixture's lane");
        (writer, Box::new(lane))
    }

    /// #1621 D1 / gate 1: a per-node live-control effect's staging window is its lane's live
    /// parameter cells, whatever automation capacity the effect was prepared with. A lane with no
    /// parameter cell -- the parametric EQ's shape, whose parameters ride prepared targets, and a
    /// channel-less session bypass -- holds no span storage; a compressor-shaped lane (shared and
    /// per-lane parameters) holds exactly its cells. Every window holds every cell the lane reads
    /// and stays within the capacity, so no staged value is dropped and no twin pair is cut off.
    ///
    /// Test value: red if `LiveControlEffect::new` sizes the window by `automation_capacity`
    /// again (4,096 spans here), the defect #1621 removes; the other lanes' assertions are the
    /// bank stage's rule (#1611 D3) stated for the per-node owner.
    #[test]
    fn a_per_node_window_is_its_lanes_cells_not_the_capacity() {
        use effect_contract::{EffectLaneShape, EffectLaneSpec, ParameterChannelPolicy};
        const CAPACITY: u32 = 4096;
        let superseded = engine::realtime::LiveValuesSuperseded::new();
        let window = |lane: EffectControlLane| {
            let effect_id = EffectNodeId {
                track_id: StableGraphId::parse("track").expect("ID"),
                rack: RackId::Dynamic,
                effect_id: StableGraphId::parse("gain").expect("ID"),
            };
            let metadata = gain_metadata(0, CAPACITY);
            let node = runtime::LiveControlEffect::new(
                GraphPreparedEffect {
                    id: effect_id,
                    metadata,
                    processor: Box::new(LiveGain::new(metadata)),
                    response_snapshot_declared: false,
                    native_id: "miso.test.live-gain",
                },
                Box::new(lane),
                None,
                4,
            );
            node.window_spans()
        };
        let live = |shape: EffectLaneShape| {
            let (_writer, lane) = effect_contract::effect_control_lane(
                EffectLaneSpec {
                    shape,
                    bypass: false,
                    observation_depth: 0,
                    link: None,
                },
                &superseded,
            )
            .expect("lane");
            lane
        };

        // The parametric EQ's shape: no parameter cell, four prepared-target slots.
        let eq = live(EffectLaneShape::from_live_parameters([], 4));
        assert_eq!(eq.parameter_cells(), 0);
        assert_eq!(window(eq), 0, "an unbanked EQ holds no span window");

        // A compressor's shape: two shared parameters and one per-lane one, four cells.
        let compressor = live(EffectLaneShape::from_live_parameters(
            [
                (0, ParameterChannelPolicy::Shared),
                (1, ParameterChannelPolicy::PerLane),
                (3, ParameterChannelPolicy::Shared),
            ],
            0,
        ));
        assert_eq!(compressor.parameter_cells(), 4);
        assert_eq!(window(compressor), 4, "a compressor's window is its cells");

        // The gain fixture's descriptor-derived lane: one per-lane parameter.
        let (_writer, gain) = control_pair();
        assert_eq!(window(*gain), 2);

        // A channel-less session bypass stages nothing.
        assert_eq!(window(EffectControlLane::without_channel(true)), 0);
    }

    /// Control's end of the fixture: its cell writer and the plan's gate handle. A push is one
    /// transaction: stamp the next revision, write the cell, publish the revision.
    struct LiveControl {
        writer: effect_contract::EffectCellWriter,
        gate: engine::realtime::PlanRevisionGate,
    }

    impl LiveControl {
        fn new(writer: effect_contract::EffectCellWriter, plan: &mut PreparedRenderPlan) -> Self {
            Self {
                writer,
                gate: plan.take_revision_gate().expect("the plan's gate handle"),
            }
        }

        fn push(&mut self, record: effect_contract::EffectControlRecord) {
            let revision = self.gate.last_published() + 1;
            let stamp = self.gate.stamp(revision).expect("a legal revision");
            self.writer
                .write(stamp, record)
                .expect("the fixture has a cell for the record");
            self.gate.publish(revision);
        }
    }

    /// #140 A / E1 for the dynamic rack: an admitted parameter command takes effect on the first
    /// sample of the next rendered block, and not one sample before.
    ///
    /// Red mutation: move the `live.stage(..)` call in `execute_op`'s `LiveControlEffect`
    /// arm to *after* `effect.processor.process(block)` -> the command lands one block late and
    /// the `block 1` assertion below fails on its first sample.
    #[test]
    fn a_live_control_parameter_command_applies_at_the_next_block_boundary() {
        let (writer, control) = control_pair();
        let mut plan = live_control_effect_plan(0, Some(control), Box::new(SampleIndexSource));
        let mut producer = LiveControl::new(writer, &mut plan);
        let block0 = render_live_control_blocks(&mut plan, 0, 1);
        assert_eq!(
            &block0[..4],
            &[0.0, 1.0, 2.0, 3.0],
            "unity before any command"
        );

        producer.push(effect_contract::EffectControlRecord::Parameter {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 0.5,
        });
        let block1 = render_live_control_blocks(&mut plan, 1, 1);
        assert_eq!(
            &block1[..4],
            &[2.0, 2.5, 3.0, 3.5],
            "every sample of the block that drains the command carries it"
        );
        assert_eq!(
            &block1[4..],
            &[-4.0, -5.0, -6.0, -7.0],
            "the right lane is untouched by a left-only command"
        );
    }

    /// A per-node effect reads its cells under the block's live snapshot (#1504, #1345): a value
    /// stamped with a revision the gate has not yet published is not applied by the block that
    /// finds it, and is applied by the first block after its revision is published.
    ///
    /// Test value: red if the per-node effect's `stage` reads under `LiveSnapshot::ALL` (or any
    /// snapshot but the block's own), which applies a transaction's value before the rest of it
    /// can be seen; the bank path has its own test, and no other covers this path.
    #[test]
    fn a_per_node_effect_waits_for_the_revision_that_covers_its_value() {
        let (writer, control) = control_pair();
        let mut plan = live_control_effect_plan(0, Some(control), Box::new(SampleIndexSource));
        let mut producer = LiveControl::new(writer, &mut plan);
        let revision = producer.gate.last_published() + 1;
        {
            let stamp = producer.gate.stamp(revision).expect("a legal revision");
            producer
                .writer
                .write(
                    stamp,
                    effect_contract::EffectControlRecord::Parameter {
                        parameter_index: 0,
                        channel: ParameterChannel::Left,
                        value: 0.5,
                    },
                )
                .expect("the fixture has a cell for the record");
        }
        let before = render_live_control_blocks(&mut plan, 0, 1);
        assert_eq!(
            &before[..4],
            &[0.0, 1.0, 2.0, 3.0],
            "the value is written but its revision is not published: the block leaves it alone"
        );
        producer.gate.publish(revision);
        let after = render_live_control_blocks(&mut plan, 1, 1);
        assert_eq!(
            &after[..4],
            &[2.0, 2.5, 3.0, 3.5],
            "the first block after the publish carries it from its first sample"
        );
    }

    /// Live bypass returns the dry signal delayed by exactly the effect's declared latency, so
    /// every PDC route timing the compiler derived from that latency stays correct.
    ///
    /// Red mutation: delete the settled-bypass skip's `live.shunt.passthrough(..)` call (#2129
    /// D7) -> a bypassed block renders its input undelayed.
    #[test]
    fn live_bypass_is_latency_preserving_and_reversible() {
        const LATENCY: u64 = 2;
        let (writer, control) = control_pair();
        let mut plan =
            live_control_effect_plan(LATENCY, Some(control), Box::new(SampleIndexSource));
        let mut producer = LiveControl::new(writer, &mut plan);
        producer.push(effect_contract::EffectControlRecord::Parameter {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 0.0,
        });
        let wet = render_live_control_blocks(&mut plan, 0, 2);
        // Latency 2 with a zero gain: the first two samples are the line's zeros, then zeros.
        assert!(
            wet[..4].iter().all(|value| *value == 0.0),
            "a zero gain silences the wet path"
        );

        producer.push(effect_contract::EffectControlRecord::Bypass(
            effect_contract::BypassSwitch::step(true),
        ));
        let bypassed = render_live_control_blocks(&mut plan, 2, 1);
        assert_eq!(
            &bypassed[..4],
            &[6.0, 7.0, 8.0, 9.0],
            "a bypassed block is the input delayed by exactly the declared latency"
        );
        assert_eq!(&bypassed[4..], &[-6.0, -7.0, -8.0, -9.0]);

        producer.push(effect_contract::EffectControlRecord::Bypass(
            effect_contract::BypassSwitch::step(false),
        ));
        let restored = render_live_control_blocks(&mut plan, 3, 1);
        // #2129 D2, D3: releasing bypass resets the instance, keeping the zero gain, and holds dry
        // for the warm-up (the declared latency, 2), then the current wet signal.
        assert_eq!(
            &restored[..4],
            &[10.0, 11.0, 0.0, 0.0],
            "releasing bypass holds dry for the warm-up, then returns the *current* wet signal"
        );
        assert_eq!(
            &restored[4..],
            &[-10.0, -11.0, -12.0, -13.0],
            "the uncommanded right lane keeps its unity wet path at the declared latency"
        );
    }

    /// #2129 D7: a per-node live effect skips its wet path on exactly the blocks it begins settled
    /// bypassed with nothing staged: not while its crossfade into bypass runs, and not on a block
    /// that stages a span. (The skipped block's words are `live_bypass_is_latency_preserving_and_
    /// reversible`'s.)
    ///
    /// Red mutations: the skip reads the lane's `bypassed()` without `bypass_ramping()` -> the two
    /// ramp blocks count; the skip ignores `staged.staged` -> the parameter block counts; the mix
    /// runs only when `needs_dry && !settled_bypassed` -> the span block emits wet.
    #[test]
    fn a_settled_bypassed_per_node_effect_skips_only_blocks_without_work() {
        let (writer, control) = control_pair();
        let mut plan = live_control_effect_plan(2, Some(control), Box::new(SampleIndexSource));
        let mut producer = LiveControl::new(writer, &mut plan);
        producer.push(effect_contract::EffectControlRecord::Bypass(
            effect_contract::BypassSwitch {
                bypassed: true,
                ramp_samples: 6,
            },
        ));
        render_live_control_blocks(&mut plan, 0, 2);
        assert_eq!(
            test_only_node_bypass_skips(&plan),
            0,
            "a crossfade into bypass runs the wet path"
        );
        render_live_control_blocks(&mut plan, 2, 3);
        assert_eq!(test_only_node_bypass_skips(&plan), 3, "settled blocks skip");
        producer.push(effect_contract::EffectControlRecord::Parameter {
            parameter_index: 0,
            channel: ParameterChannel::Left,
            value: 0.5,
        });
        let span_block = render_live_control_blocks(&mut plan, 5, 2);
        // The span block runs the wet path (gain 0.5 on the left) and still emits the dry input
        // delayed by the declared latency, on both channels.
        assert_eq!(
            &span_block[..8],
            &[18.0, 19.0, 20.0, 21.0, -18.0, -19.0, -20.0, -21.0],
            "a settled-bypassed block that runs the wet path emits the delayed dry input"
        );
        assert_eq!(
            test_only_node_bypass_skips(&plan),
            4,
            "the block that stages a span runs the wet path, the next skips"
        );
    }

    /// A live-control-free plan renders exactly the bits a live-control-attached plan renders when
    /// no command is ever sent: the feed is inert until something is admitted.
    ///
    /// This is the class-A identity claim in its smallest form -- the same claim the corpus
    /// digests and the three wasm-gate legs make for whole sessions.
    #[test]
    fn idle_live_controls_change_no_rendered_bit() {
        for latency in [0_u64, 3] {
            let mut without = live_control_effect_plan(latency, None, Box::new(SampleIndexSource));
            let (_writer, control) = control_pair();
            let mut with =
                live_control_effect_plan(latency, Some(control), Box::new(SampleIndexSource));
            let plain = render_live_control_blocks(&mut without, 0, 4);
            let live = render_live_control_blocks(&mut with, 0, 4);
            assert_eq!(
                plain.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                live.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                "latency={latency}: idle live controls are bit-inert",
            );
        }
    }

    /// Both of the gain fixture's cells are written and staged in one block: the window is the
    /// effect's automation capacity, which holds the lane's two cells.
    #[test]
    fn the_live_control_effect_applies_every_cell_within_its_window() {
        let (writer, control) = control_pair();
        let mut plan = live_control_effect_plan(0, Some(control), Box::new(SampleIndexSource));
        let mut producer = LiveControl::new(writer, &mut plan);
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            producer.push(effect_contract::EffectControlRecord::Parameter {
                parameter_index: 0,
                channel,
                value: 2.0,
            });
        }
        let block = render_live_control_blocks(&mut plan, 0, 1);
        assert_eq!(&block[..4], &[0.0, 2.0, 4.0, 6.0]);
        assert_eq!(&block[4..], &[0.0, -2.0, -4.0, -6.0]);
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
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let metadata = PreparedEffectMetadata {
            descriptor: &SUM_DESCRIPTOR,
            sample_rate: 48_000,
            quantum: 4,
            quality: EffectQuality::Normal,
            bypass: false,
            link_mode: LinkMode::DualMono,
            ports: PreparedPorts {
                sidechain: PreparedSidechainPort::Connected {
                    id: SUM_SIDECHAIN,
                    required: false,
                },
            },
            latency: LatencySamples(0),
            tail: effect_contract::TailSamples::Finite(0),
            tail_every_peak: effect_contract::TailSamples::Infinite,
            rest: effect_contract::RestBound::Unstated,
            composition: effect_contract::CompositionBound::Unstated,
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
                            bound: NodeTailBound::ZERO,
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
                gate: RouteGate::OPEN,
            }],
            track_delays: Vec::new(),
            effects: vec![GraphPreparedEffect {
                id: effect_id,
                metadata,
                processor: Box::new(SidechainSum),
                response_snapshot_declared: false,
                native_id: "miso.test.sidechain-sum",
            }],
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
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

    /// Batch 16a fold MINOR 11 (#1641 D1). Red if install accepts a timeline carry on a plan
    /// with no source set, which holds the timeline: the swap block would report a mismatch.
    #[test]
    fn install_refuses_a_timeline_carry_on_a_plan_with_no_source_set() {
        let mut plan = sidechain_pdc_plan(false);
        let program = |timeline| GraphCarryProgram {
            predecessor: 1,
            sources: Box::new([]),
            timeline,
        };
        assert_eq!(
            install_carry_program(&mut plan, program(true)),
            Err(GraphCarryInstallError::NoSourceSet)
        );
        assert_eq!(install_carry_program(&mut plan, program(false)), Ok(()));
    }

    #[test]
    fn faster_main_and_faster_sidechain_align_on_their_typed_ports() {
        for delay_main in [true, false] {
            let mut plan = sidechain_pdc_plan(delay_main);
            let mut samples = [0.0_f32; 8];
            let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
            plan.render(
                engine::realtime::RenderIo { output },
                engine::realtime::RenderTime { absolute_sample: 0 },
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
        let effect_contract::PreparedEffect {
            processor,
            metadata,
        } = factory
            .prepare(PrepareEffectRequest {
                sample_rate: rate,
                quantum,
                quality: EffectQuality::Normal,
                bypass,
                link_mode: LinkMode::DualMono,
                ports: PreparedPorts {
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
                tail_bound: conformance::tail_bound_of(
                    Box::new(DualAccumulatorDelayFactory::correct()),
                    rate,
                    EffectQuality::Normal,
                ),
            })
            .expect("effect");
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
                bound: if id == effect_node {
                    NodeTailBound {
                        tail: metadata.tail,
                        tail_every_peak: metadata.tail_every_peak,
                        rest: metadata.rest,
                        composition: metadata.composition,
                    }
                } else {
                    NodeTailBound::ZERO
                },
                id,
            })
            .collect();
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(rate),
            quantum: QuantumFrames(quantum),
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
                    gate: RouteGate::OPEN,
                },
                PreparedRoute {
                    node: route_effect,
                    transform: identity,
                    gate: RouteGate::OPEN,
                },
            ],
            track_delays: Vec::new(),
            effects: vec![GraphPreparedEffect {
                id: effect_id,
                metadata,
                processor,
                response_snapshot_declared: false,
                native_id: "miso.test.sidechain-sum",
            }],
            effect_controls: Vec::new(),
            effect_observations: Vec::new(),
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
                            engine::realtime::RenderIo { output },
                            engine::realtime::RenderTime {
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

    /// Issue #918: a source set lends a driver's planes only for one of its own claims and only
    /// when both are exactly one quantum, the checks `copy_track_input` makes of its destinations.
    /// Anything else is refused as `None`, which a gather serves as silence, never handed on.
    #[test]
    fn a_source_set_lends_only_quantum_planes_of_its_own_claims() {
        const FRAMES: usize = 5;
        struct Lender(Vec<f32>);
        impl GraphPreparedSourceSetDriver for Lender {
            fn retained_bytes(&self) -> u64 {
                engine::realtime::retained::vec(&self.0)
            }
            fn claim_count(&self) -> usize {
                2
            }
            fn begin_block(&mut self, _: u64, _: u32) -> Result<(), RenderError> {
                Ok(())
            }
            fn copy_track_input(
                &mut self,
                _: usize,
                _: &mut [f32],
                _: &mut [f32],
            ) -> Result<(), RenderError> {
                Ok(())
            }
            fn provides_played_planes(&self) -> bool {
                true
            }
            /// Claim 1's right plane is one word short; every other index lends a quantum,
            /// including 2, which is not one of the set's claims.
            fn played_planes(&self, claim: usize) -> Option<(&[f32], &[f32])> {
                let right = if claim == 1 { FRAMES - 1 } else { FRAMES };
                Some((&self.0[..FRAMES], &self.0[FRAMES..FRAMES + right]))
            }
        }
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
            output_channels: core::num::NonZeroUsize::new(2).expect("stereo"),
        };
        let claim = |track: &str| GraphSourceInputClaim {
            node: GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track).expect("track id"),
                stage: TrackStage::Input,
            },
        };
        let words: Vec<f32> = (0..2 * FRAMES).map(|word| word as f32).collect();
        let set = GraphPreparedSourceSet::new(
            envelope,
            vec![claim("a"), claim("b")],
            GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(Lender(words.clone())),
        );
        assert_eq!(
            GraphSourcePlanes::played_planes(&set, 0),
            Some((&words[..FRAMES], &words[FRAMES..])),
            "a whole quantum of a claim is lent as is"
        );
        assert_eq!(
            GraphSourcePlanes::played_planes(&set, 1),
            None,
            "a short plane is refused"
        );
        assert_eq!(
            GraphSourcePlanes::played_planes(&set, 2),
            None,
            "an index past the set's claims is refused"
        );
    }

    /// #1283 D1: the endpoints of a delayed route edge are the route's own, so a route whose
    /// source was re-pointed has other endpoints on its destination edge too, though that edge's
    /// own source (the route node) and destination did not move. Red if a route edge reports its
    /// own two nodes: the carry would join the old line to the re-pointed route.
    #[test]
    fn a_route_edge_names_the_endpoints_of_its_route() {
        let id = |value: &str| StableGraphId::parse(value).expect("graph ID");
        let strip = |value: &str| GraphNodeId::TrackStage {
            track_id: id(value),
            stage: TrackStage::PostMatrix,
        };
        let port = |node: GraphNodeId, kind| GraphPortId {
            node,
            kind,
            effect_port: None,
        };
        let route = GraphNodeId::Route { route_id: id("r") };
        let output = GraphNodeId::Output {
            output_id: id("out"),
        };
        let spec_of = |from: &str| GraphSpec {
            nodes: Vec::new(),
            ports: Vec::new(),
            edges: vec![
                GraphEdge {
                    id: GraphEdgeId::RouteSource { route_id: id("r") },
                    source: port(strip(from), GraphPortKind::MainOutput),
                    destination: port(route.clone(), GraphPortKind::MainInput),
                    path: String::new(),
                },
                GraphEdge {
                    id: GraphEdgeId::RouteDestination { route_id: id("r") },
                    source: port(route.clone(), GraphPortKind::MainOutput),
                    destination: port(output.clone(), GraphPortKind::MainInput),
                    path: String::new(),
                },
            ],
        };
        let program = program::ExecutionProgram {
            ops: Box::new([]),
            inputs: Box::new([]),
            delays: Box::new([program::DelaySpec {
                samples: 480,
                edge: 1,
            }]),
            node_buffer: Box::new([]),
            node_op: Box::new([]),
            taps: Box::new([]),
            buffers: 0,
            output: program::BufferRef(0),
        };
        let before = compensation_lines_of(&spec_of("a"), &program);
        let after = compensation_lines_of(&spec_of("b"), &program);
        assert_eq!(before.len(), 1);
        assert_eq!(
            (&before[0].edge, &before[0].source, &before[0].destination),
            (
                &GraphEdgeId::RouteDestination { route_id: id("r") },
                &strip("a"),
                &output
            )
        );
        assert_eq!(after[0].source, strip("b"));
        assert_eq!(after[0].destination, output);
    }
}

#[cfg(test)]
mod follow_route_tests {
    //! #1652a: the seven-word cell of a following route: its flags word, its record, the producer
    //! write and the charge.
    use super::*;
    use engine::realtime::{LiveSnapshot, LiveValuesSuperseded, RevisionGate};

    fn flags_of(word: u32) -> RouteFollowFlags {
        RouteFollowFlags {
            mute: word & 1 != 0,
            follows_mute: word & 2 != 0,
            terms: [word & 4 != 0, word & 8 != 0],
            automated: [word & 16 != 0, word & 32 != 0],
        }
    }

    fn transform() -> RouteTransform {
        RouteTransform {
            gain: 0.5,
            ll: 1.0,
            lr: -0.0,
            rl: f32::from_bits(1),
            rr: -0.75,
        }
    }

    fn route_id(value: &str) -> StableGraphId {
        StableGraphId::parse(value).expect("a route ID")
    }

    /// The cursor rows' jump lengths of the tests: the session's `mute_ms` length for both rows.
    pub(crate) fn route_jumps(lengths: GraphJumpLengths) -> GraphRouteJumpLengths {
        GraphRouteJumpLengths {
            mute: lengths.mute_samples,
            vca_mute: lengths.mute_samples,
        }
    }

    /// A program of one mute-shaped cell per given lane (steps, completion `End`, block jump
    /// length), and the source over it at `arrival`, following, with no other term.
    pub(crate) fn mute_source(
        lanes: [Option<&[automation::Segment]>; 2],
        arrival: u64,
    ) -> GraphFollowRouteSource {
        let descriptions: Vec<_> = lanes
            .iter()
            .flatten()
            .map(|segments| automation::CellDescription {
                segments,
                grid_ramp: 64,
                grid: 64,
                jump: automation::JumpLength::Block,
                completion: automation::Completion::End,
                no_restart: false,
            })
            .collect();
        let program =
            std::sync::Arc::new(automation::ProgramBuilder::build(&descriptions).expect("cells"));
        let mut next = 0;
        let cells = lanes.map(|lane| {
            lane.map(|_| {
                next += 1;
                next - 1
            })
        });
        let flags = RouteFollowFlags {
            follows_mute: true,
            automated: cells.map(|cell| cell.is_some()),
            ..RouteFollowFlags::default()
        };
        GraphFollowRouteSource::new(
            program,
            cells,
            [Vec::new(), Vec::new()],
            arrival,
            route_jumps,
            flags,
        )
        .expect("a valid source")
    }

    /// A step curve: `values[i]` from `times[i]` on (the first value from the start).
    pub(crate) fn steps(times: &[u64], values: &[f32], end: u64) -> Vec<automation::Segment> {
        times
            .iter()
            .zip(values)
            .enumerate()
            .map(|(index, (&start, &value))| automation::Segment {
                start,
                end: times.get(index + 1).copied().unwrap_or(end),
                start_value: value,
                end_value: values.get(index + 1).copied().unwrap_or(value),
                shape: automation::Shape::Step,
            })
            .collect()
    }

    /// A source refuses no automated lane, a cell outside its program and `automated` flags that
    /// disagree with its cells (own or reaching VCA mute cells, #1655 D4).
    ///
    /// Test value: red if a route with no automation takes the seven-word render state, or if
    /// render could look up a cell its program does not hold.
    #[test]
    fn the_source_refuses_no_lane_a_missing_cell_and_disagreeing_flags() {
        let curve = steps(&[0, 128], &[0.0, 1.0], 4_096);
        let source = mute_source([Some(&curve), None], 7);
        assert_eq!(source.arrival(), 7);
        let program = std::sync::Arc::clone(&source.program);
        let flags = source.flags();
        let new_with = |cells, vca_mutes, flags| {
            GraphFollowRouteSource::new(
                std::sync::Arc::clone(&program),
                cells,
                vca_mutes,
                0,
                route_jumps,
                flags,
            )
            .is_some()
        };
        let new = |cells, flags| new_with(cells, [Vec::new(), Vec::new()], flags);
        assert!(new([Some(0), None], flags));
        // #1655 D4: a lane is automated by its own cell or by a reaching VCA mute's cells.
        let both = RouteFollowFlags {
            automated: [true, true],
            ..flags
        };
        assert!(
            new_with([Some(0), None], [Vec::new(), vec![0]], both),
            "a lane only a VCA mute reaches"
        );
        assert!(
            new_with([None, None], [vec![0, 0], vec![0]], both),
            "VCA mute cells alone"
        );
        assert!(
            !new_with([Some(0), None], [Vec::new(), vec![1]], both),
            "a VCA mute cell outside the program"
        );
        assert!(
            !new_with([Some(0), None], [vec![0], Vec::new()], both),
            "VCA flags that disagree"
        );
        assert!(!new([None, None], RouteFollowFlags::default()), "no lane");
        assert!(!new([Some(1), None], flags), "a cell outside the program");
        assert!(
            !new(
                [Some(0), None],
                RouteFollowFlags {
                    automated: [true, true],
                    ..flags
                }
            ),
            "flags that disagree"
        );
    }

    /// The cursor rows' jump lengths: the source mute takes the mute length and a VCA mute the
    /// `VcaMute` length.
    ///
    /// Test value: red if a route cursor would jump over another row's length.
    #[test]
    fn each_cursor_row_takes_its_own_jump_length() {
        let lengths = GraphRouteJumpLengths {
            mute: 3,
            vca_mute: 4,
        };
        assert_eq!(lengths.for_row(GraphRouteCursorRow::SourceMute), 3);
        assert_eq!(lengths.for_row(GraphRouteCursorRow::VcaMute), 4);
    }

    /// The layout: bit 0 mute, 1 follows_mute, 2-3 terms L/R, 4-5 automated L/R. Every one of the
    /// 64 defined words decodes to its fields and encodes back to itself; a word with any bit
    /// from 6 up set is refused.
    ///
    /// Test value: red if two fields swap bits, if a field is dropped on the round trip, or if a
    /// word with an undefined bit decodes (render would apply words no record wrote).
    #[test]
    fn the_flags_word_round_trips_its_six_bits_and_refuses_every_other_bit() {
        for word in 0..64_u32 {
            let flags = RouteFollowFlags::from_word(word).expect("a defined word");
            assert_eq!(flags, flags_of(word), "word {word:#b}");
            assert_eq!(flags.to_word(), word);
            for bit in 6..32 {
                assert_eq!(
                    RouteFollowFlags::from_word(word | 1 << bit),
                    None,
                    "bit {bit} set"
                );
            }
        }
        assert_eq!(RouteFollowFlags::from_word(u32::MAX), None);
    }

    /// D2's composition, over every flags word and every curve state:
    /// `follow_zeroed[lane] = follows_mute && ((automated[lane] && curve[lane]) || terms[lane])`,
    /// and the route's own mute passes through.
    ///
    /// Test value: red if a curve value reaches a lane the plan does not automate, if a lane reads
    /// the other lane's curve or terms, or if the follow ignores `follows_mute`.
    #[test]
    fn the_gate_follows_the_curve_only_on_an_automated_lane() {
        for word in 0..64_u32 {
            let flags = flags_of(word);
            for curve in [[false, false], [true, false], [false, true], [true, true]] {
                let gate = flags.gate(curve);
                assert_eq!(gate.mute, flags.mute);
                for lane in 0..2 {
                    let expected = flags.follows_mute
                        && ((flags.automated[lane] && curve[lane]) || flags.terms[lane]);
                    assert_eq!(
                        gate.follow_zeroed[lane], expected,
                        "word {word:#b}, curve {curve:?}, lane {lane}"
                    );
                }
            }
        }
    }

    /// The record keeps the open transform's bits exactly (negative zero and a subnormal
    /// coefficient included), its flags and its length through the seven cell words, and refuses a
    /// ramp over the bound, a non-finite transform word and a flags word with an undefined bit.
    ///
    /// Test value: red if a word moves in the layout, if a transform word is rounded or
    /// canonicalised on the way, or if words no record wrote decode.
    #[test]
    fn the_record_round_trips_its_seven_words_and_refuses_out_of_bounds_words() {
        let flags = flags_of(0b10_0110);
        let record = FollowRouteControlRecord::new(transform(), flags, 480).expect("a record");
        let words = record.to_words();
        assert_eq!(
            words,
            [
                0.5_f32.to_bits(),
                1.0_f32.to_bits(),
                (-0.0_f32).to_bits(),
                1,
                (-0.75_f32).to_bits(),
                0b10_0110,
                480,
            ]
        );
        assert_eq!(FollowRouteControlRecord::from_words(words), Some(record));
        assert_eq!(record.transform(), transform());
        assert_eq!(record.flags(), flags);
        assert_eq!(record.length(), 480);

        let bound = FollowRouteControlRecord::new(transform(), flags, ROUTE_RAMP_LENGTH_MAXIMUM)
            .expect("the bound is in bounds");
        assert_eq!(
            FollowRouteControlRecord::from_words(bound.to_words()),
            Some(bound)
        );
        assert_eq!(
            FollowRouteControlRecord::new(transform(), flags, ROUTE_RAMP_LENGTH_MAXIMUM + 1),
            None
        );
        for word in 0..5 {
            for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
                let mut words = words;
                words[word] = bad.to_bits();
                assert_eq!(FollowRouteControlRecord::from_words(words), None);
            }
        }
        let mut long = words;
        long[6] = ROUTE_RAMP_LENGTH_MAXIMUM + 1;
        assert_eq!(FollowRouteControlRecord::from_words(long), None);
        let mut undefined = words;
        undefined[5] |= 1 << 6;
        assert_eq!(FollowRouteControlRecord::from_words(undefined), None);
    }

    /// The producer writes the seven words into the route's own cell, and render's half reads
    /// them under the block's snapshot: nothing before the revision is published, the newest
    /// record once it is, each replaced write counted as superseded, nothing on an idle cell.
    ///
    /// Test value: red if the producer writes another cell than the binding reads, if a value is
    /// readable before its revision is published, or if a replaced write is not counted.
    #[test]
    fn the_producer_write_reaches_the_binding_under_the_published_revision() {
        let superseded = LiveValuesSuperseded::new();
        let (mut binding, mut producer) =
            follow_route_control_pair(&route_id("send"), &superseded, one_lane_source(), 0, 0, 0)
                .expect("tables");
        assert_eq!(&*producer.route_id, "send");
        assert_eq!(
            binding.node,
            GraphNodeId::Route {
                route_id: route_id("send")
            }
        );
        let (gate, mut pin) = RevisionGate::pair(0);
        let first =
            FollowRouteControlRecord::new(transform(), flags_of(0b01_0010), 64).expect("first");
        let second =
            FollowRouteControlRecord::new(transform(), flags_of(0b01_0011), 0).expect("second");

        assert_eq!(binding.control.read(gate.snapshot()), None, "an idle cell");
        producer.write(pin.stamp(1).expect("stamp"), first);
        assert_eq!(
            binding.control.read(gate.snapshot()),
            None,
            "revision 1 is not published"
        );
        pin.publish(&gate, 1);
        producer.write(pin.stamp(2).expect("stamp"), second);
        pin.publish(&gate, 2);
        assert_eq!(producer.test_only_write_count(), 2);
        assert_eq!(binding.control.read(gate.snapshot()), Some(second));
        assert_eq!(superseded.get(), 1, "the first write was replaced unread");
        assert_eq!(
            binding.control.read(LiveSnapshot::ALL),
            None,
            "applied once"
        );
    }

    fn one_lane_source() -> GraphFollowRouteSource {
        mute_source([Some(&steps(&[0], &[0.0], 64)), None], 0)
    }

    fn six_word_producer(id: &str, activity: (u64, u64)) -> GraphRouteControlProducer {
        GraphRouteControlProducer {
            route_id: id.into(),
            lane: std::sync::Arc::new(RouteLane::new()),
            writer: engine::realtime::CellWriter::new(),
            activity_bytes: activity.0,
            activity_largest: activity.1,
        }
    }

    /// The charge of six-word and following routes together counts each route once, the route IDs
    /// twice, and the plan's one route-activity table once. A plan with no following route is
    /// charged its six-word terms alone.
    ///
    /// Test value: red if a kind is left out of the joint charge, or if the activity table is
    /// charged per kind.
    #[test]
    fn the_charge_counts_each_kind_and_the_activity_table_once() {
        let superseded = LiveValuesSuperseded::new();
        /// The plan's prepared routes: their activation offsets join the activity charge
        /// once, for a plan with a following route (#1652 MAJOR-1).
        const ACTIVITY_ROUTES: u64 = 7;
        let activity = (4_096, 1_024);
        let six = [
            six_word_producer("a", activity),
            six_word_producer("bb", activity),
        ];
        let seven: Vec<_> = ["ccc", "dddd", "eeeee"]
            .iter()
            .map(|id| {
                follow_route_control_pair(
                    &route_id(id),
                    &superseded,
                    one_lane_source(),
                    activity.0,
                    activity.1,
                    ACTIVITY_ROUTES,
                )
                .expect("tables")
                .1
            })
            .collect();

        let both = route_control_resources(&six, &seven);
        assert_eq!(both.routes, 5);
        assert_eq!(both.route_id_bytes, 2 * (1 + 2 + 3 + 4 + 5));

        let alone = route_control_resources(&six, &[]);
        assert_eq!(alone.routes, 2);
        assert_eq!(alone.route_id_bytes, 2 * (1 + 2));
        assert_eq!(alone.activity_bytes, activity.0);
        // #1652 MAJOR-1: a plan with a following route also charges the activation offsets bind
        // adds to the table, one `u32` per prepared route, once.
        assert_eq!(
            both.activity_bytes,
            alone.activity_bytes + ACTIVITY_ROUTES * 4,
            "the activation offsets are charged with the table"
        );
        let following = route_control_resources(&[], &seven);
        assert_eq!(
            alone.cell_bytes + following.cell_bytes,
            both.cell_bytes,
            "each kind's cells are charged in the joint plan"
        );
        assert_eq!(
            alone.owner_bytes + following.owner_bytes,
            both.owner_bytes,
            "each kind's owners are charged in the joint plan"
        );
        assert_eq!(
            alone.total_bytes + following.total_bytes - activity.0,
            both.total_bytes,
            "the activity table is charged once"
        );
        assert_eq!(
            route_control_resources(&[], &[]),
            RouteControlResources::default()
        );
    }
}
