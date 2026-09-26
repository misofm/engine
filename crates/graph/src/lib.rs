//! Immutable render-reachable graph data and scalar routing primitives.
//!
//! Parsing, hashing, validation, and lowering live in `graph-compiler`; this crate
//! only retains the already-validated immutable result and its preallocated render state.
#![allow(missing_docs)]

mod observation_activation;
pub mod program;
mod runtime;

pub use observation_activation::{
    GraphObservationAccepted, GraphObservationActivationConfig,
    GraphObservationActivationResources, GraphObservationAdmissionError, GraphObservationApplied,
    GraphObservationController,
};

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub use runtime::{
    TestOnlyBankChainConstructionFacts, TestOnlyBankChainInputs, TestOnlyBankChainOwnership,
    test_only_bank_chain_construction_facts, test_only_bank_chain_ownership,
    test_only_prepare_bank_chain_inputs, test_only_reset_bank_chain_construction_facts,
};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use runtime::{
    TestOnlyFailedBufferCapture, TestOnlySelectedSplitFader, TestOnlySplitPairTableWitness,
    test_only_arm_failed_buffer_capture, test_only_completion_disabled,
    test_only_failed_buffer_capture, test_only_meter_input_counts, test_only_meter_input_reset,
    test_only_observation_dispatch_counts, test_only_observation_dispatch_reset,
    test_only_reset_selected_split_fader, test_only_reset_split_pair_table_witness,
    test_only_resident_input_counts, test_only_resident_input_reset,
    test_only_selected_split_fader, test_only_set_completion_disabled,
    test_only_set_output_route_fold_declined, test_only_set_route_fold_declined,
    test_only_set_scatter_redirect_declined, test_only_set_source_in_place_declined,
    test_only_source_plane_counts, test_only_source_plane_reset,
    test_only_split_pair_table_witness,
};

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub use observation_activation::test_only_observation_transition_entries;

/// Phase-level timing of [`GraphExecutor::render`] for the plumbing-floor diagnosis
/// (`.github/ISSUE_SPECS/DRAFTS/PLAN.md`). Test builds only: nothing here exists in a production
/// build, and a `test-support` build that never calls [`test_only_phase_profile::enable`] pays one
/// thread-local read per block and nothing per unit.
///
/// The clock is `std::time::Instant` (the vDSO monotonic clock), because `crates/graph` may not
/// carry the `unsafe` block `_rdtsc` needs. A probe is taken only where the *kind* of unit
/// changes, so a block whose units are grouped by kind costs a handful of probes rather than one
/// per unit; the harness measures the probe cost and states it beside the numbers.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_only_phase_profile {
    use core::cell::Cell;
    use std::time::Instant;

    /// Render entry to the first source work: the host planes' shape check and
    /// `begin_observation_block`.
    pub const ENTER: usize = 0;
    /// The source set's `begin_block` and the `copy_track_input` loop (empty when the plan binds
    /// no source set, as the console rows do).
    pub const SOURCE: usize = 1;
    /// Plain units whose op is a host-bound processor (`NodeKind::Bound`): on the console rows,
    /// the sixty-four `FrozenGraphSource` copies.
    pub const BOUND: usize = 2;
    /// Plain units whose op is a route (`NodeKind::Route`).
    pub const ROUTE: usize = 3;
    /// The session Output op's unit.
    pub const OUTPUT: usize = 4;
    /// A plain identity unit with one input that is not in place: `reduce_plane`'s copy arm.
    pub const IDENTITY_COPY: usize = 5;
    /// A plain identity unit in place over its one input: dispatch and nothing else.
    pub const IDENTITY_ALIAS: usize = 6;
    /// Any other plain unit.
    pub const OTHER_OP: usize = 7;
    /// A bank unit.
    pub const BANK: usize = 8;
    /// The unit loop's end to the return (on the pre-#916 tree, the end-of-block master copy).
    pub const EXIT: usize = 9;
    pub const COUNT: usize = 10;

    thread_local! {
        static ENABLED: Cell<bool> = const { Cell::new(false) };
        static NANOS: Cell<[u64; COUNT]> = const { Cell::new([0; COUNT]) };
        static BLOCKS: Cell<u64> = const { Cell::new(0) };
        static PROBES: Cell<u64> = const { Cell::new(0) };
        static RUNS: Cell<[(usize, u64); RUNS_CAPACITY]> =
            const { Cell::new([(COUNT, 0); RUNS_CAPACITY]) };
    }

    /// Runs of same-kind units recorded from the first profiled block, `(kind, units)`.
    pub const RUNS_CAPACITY: usize = 16;

    /// The unit-kind runs of the first block profiled since the last reset, in schedule order;
    /// `(COUNT, 0)` marks unused entries.
    #[must_use]
    pub fn runs() -> [(usize, u64); RUNS_CAPACITY] {
        RUNS.with(Cell::get)
    }

    /// Switch the probes on or off for this thread. Off, `render` reads one thread-local per
    /// block and nothing else.
    pub fn enable(enabled: bool) {
        ENABLED.with(|value| value.set(enabled));
    }

    /// Zero the accumulators.
    pub fn reset() {
        NANOS.with(|value| value.set([0; COUNT]));
        BLOCKS.with(|value| value.set(0));
        PROBES.with(|value| value.set(0));
        RUNS.with(|value| value.set([(COUNT, 0); RUNS_CAPACITY]));
    }

    /// `(nanoseconds per phase, blocks profiled, probes taken)` since the last reset.
    #[must_use]
    pub fn snapshot() -> ([u64; COUNT], u64, u64) {
        (
            NANOS.with(Cell::get),
            BLOCKS.with(Cell::get),
            PROBES.with(Cell::get),
        )
    }

    /// One block's running probe. `None` when disabled, so the hot path is one `Option` test.
    pub(crate) struct Probe {
        last: Instant,
        phase: usize,
        nanos: [u64; COUNT],
        probes: u64,
        runs: [(usize, u64); RUNS_CAPACITY],
        run: usize,
    }

    impl Probe {
        #[inline]
        pub(crate) fn start() -> Option<Self> {
            ENABLED.with(Cell::get).then(|| Self {
                last: Instant::now(),
                phase: ENTER,
                nanos: [0; COUNT],
                probes: 1,
                runs: [(COUNT, 0); RUNS_CAPACITY],
                run: 0,
            })
        }

        /// Charge the time since the last probe to the current phase and make `next` current.
        #[inline]
        pub(crate) fn enter(&mut self, next: usize) {
            let now = Instant::now();
            self.nanos[self.phase] +=
                u64::try_from(now.duration_since(self.last).as_nanos()).unwrap_or(u64::MAX);
            self.last = now;
            self.phase = next;
            self.probes += 1;
        }

        /// Charge a unit of kind `kind`; a probe is taken only when the kind changes.
        #[inline]
        pub(crate) fn unit(&mut self, kind: usize) {
            if kind != self.phase {
                self.enter(kind);
                if self.run < RUNS_CAPACITY && self.runs[self.run].1 != 0 {
                    self.run += 1;
                }
                if self.run < RUNS_CAPACITY {
                    self.runs[self.run].0 = kind;
                }
            }
            if self.run < RUNS_CAPACITY {
                self.runs[self.run].1 += 1;
            }
        }

        #[inline]
        pub(crate) fn finish(mut self) {
            self.enter(COUNT - 1);
            NANOS.with(|value| {
                let mut total = value.get();
                for (slot, nanos) in total.iter_mut().zip(self.nanos) {
                    *slot += nanos;
                }
                value.set(total);
            });
            if BLOCKS.with(Cell::get) == 0 {
                RUNS.with(|value| value.set(self.runs));
            }
            BLOCKS.with(|value| value.set(value.get() + 1));
            PROBES.with(|value| value.set(value.get() + self.probes));
        }
    }
}

use core::cell::Cell;
use std::any::Any;
use std::collections::{BTreeMap, BTreeSet};

use effect_contract::{
    BankWidth, ChannelSymmetryWitness, EffectControlLane, LatencySamples, ObservationLane,
    PreparedEffectMetadata, PreparedNativeEffect, ResponseAnalysisError, ResponseSnapshotRequest,
    ResponseSnapshotSummary, SeamSide, TailSamples,
};
use engine::{
    QuantumFrames,
    realtime::{
        BufferArena, PlanUnitEligibility, PlanarBufferMut, PlanarBufferRef, PrepareRenderPlan,
        PreparedPlanExecutor, PreparedRenderPlan, RenderEnvelope, RenderError,
        ResponseSnapshotError, ResponseSnapshotSink,
    },
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
    pub maximum_mask_bytes: u64,
    pub largest_allocation_bytes: u64,
}

/// Checked retained storage for live scalar fader/matrix owners lowered after preparation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphScalarOwnerResourceEstimate {
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
    /// The bounded boxed table reserved for possible serialized split owners. This is a separate
    /// retained allocation from the two concrete fader/matrix owner boxes.
    pub split_pair_table_bytes: u64,
}

/// Checked retained storage for the eligible bounded serialized split-owner table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphScalarSplitRuntimeResourceEstimate {
    /// Number of split-owner entries the preparation bound reserves for possible selection.
    pub possible_pair_count: u64,
    /// Bytes in the boxed `[Box<dyn GraphRuntimeSplitPairProcessor>]` table allocation.
    pub split_pair_table_bytes: u64,
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
}

/// Checked runtime metadata retained by every graph executor. The semantic graph estimate does
/// not include the lowered runtime layouts. A plain op stores its split slot inside `RuntimeUnit`;
/// a bank stores it inside each `RuntimeOp` member. The larger derived delta is therefore the
/// single safe per-emitted-op charge for mixed graphs. The corresponding containing allocation is
/// bounded by the larger current `RuntimeUnit`/`RuntimeOp` layout multiplied by the same
/// emitted-op bound. The inline slot is charged once, never once as an op and again as a unit.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraphRuntimeMetadataResourceEstimate {
    pub emitted_op_count: u64,
    /// `Runtime`'s inline split-owner table field delta; this is the retained owner term charged.
    pub runtime_field_bytes: u64,
    pub runtime_op_layout_delta_bytes: u64,
    pub runtime_unit_layout_delta_bytes: u64,
    pub emitted_op_layout_delta_bytes: u64,
    pub runtime_op_containing_bytes: u64,
    pub runtime_unit_containing_bytes: u64,
    /// The same table field measured in its containing `GraphExecutor` owner layout. It is
    /// reported for the largest-allocation proof, but is not added a second time to `total_bytes`.
    pub runtime_owner_field_bytes: u64,
    pub runtime_owner_allocation_bytes: u64,
    /// The actual containing `GraphExecutor` layout delta occupied by observation activation
    /// state. This inline runtime term is charged for every graph, including graphs without an
    /// activation controller, and overlaps the activation report's `runtime_state_bytes` field.
    pub observation_runtime_state_bytes: u64,
    /// Boxed response-owner binding table retained by the prepared runtime.
    pub response_binding_table_bytes: u64,
    /// Payload bytes retained by the per-owner stable-id and track-id strings.
    pub response_binding_string_bytes: u64,
    /// Largest individual cloned identity allocation.
    pub largest_response_binding_string_bytes: u64,
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
        let (_, runtime_field_bytes) = runtime::scalar_split_runtime_layout();
        let (op_layout_delta_bytes, runtime_unit_layout_delta_bytes) =
            runtime::scalar_split_op_layout();
        let (runtime_owner_field_bytes, runtime_owner_allocation_bytes) =
            scalar_split_runtime_owner_layout();
        let (_, observation_runtime_state_bytes) = observation_runtime_layout()?;
        let runtime_op_bytes = u64::try_from(core::mem::size_of::<runtime::RuntimeOp>())
            .expect("runtime op layout fits u64");
        let runtime_unit_bytes = u64::try_from(core::mem::size_of::<runtime::RuntimeUnit>())
            .expect("runtime unit layout fits u64");
        let emitted_op_layout_delta_bytes =
            op_layout_delta_bytes.max(runtime_unit_layout_delta_bytes);
        let emitted_op_delta_bytes = emitted_op_layout_delta_bytes.checked_mul(emitted_op_count)?;
        let runtime_op_containing_bytes = runtime_op_bytes.checked_mul(emitted_op_count)?;
        let runtime_unit_containing_bytes = runtime_unit_bytes.checked_mul(emitted_op_count)?;
        let response_binding_entry_bytes =
            u64::try_from(core::mem::size_of::<runtime::ResponseOwnerBinding>())
                .expect("response binding layout fits u64");
        let response_binding_table_bytes =
            response_binding_entry_bytes.checked_mul(response_binding_count)?;
        let total_bytes = runtime_field_bytes
            .checked_add(emitted_op_delta_bytes)?
            .checked_add(observation_runtime_state_bytes)?;
        let total_bytes = total_bytes
            .checked_add(response_binding_table_bytes)?
            .checked_add(response_binding_string_bytes)?;
        Some(Self {
            emitted_op_count,
            runtime_field_bytes,
            runtime_op_layout_delta_bytes: op_layout_delta_bytes,
            runtime_unit_layout_delta_bytes,
            emitted_op_layout_delta_bytes,
            runtime_op_containing_bytes,
            runtime_unit_containing_bytes,
            runtime_owner_field_bytes,
            runtime_owner_allocation_bytes,
            observation_runtime_state_bytes,
            response_binding_table_bytes,
            response_binding_string_bytes,
            largest_response_binding_string_bytes,
            total_bytes,
            largest_allocation_bytes: runtime_owner_allocation_bytes
                .max(runtime_op_containing_bytes)
                .max(runtime_unit_containing_bytes)
                .max(response_binding_table_bytes)
                .max(largest_response_binding_string_bytes),
        })
    }
}

impl GraphScalarSplitRuntimeResourceEstimate {
    /// Computes only the checked split-table charge. Runtime and op/unit layout deltas are charged
    /// by [`GraphRuntimeMetadataResourceEstimate`] for every graph.
    pub fn checked_for(possible_pair_count: u64) -> Option<Self> {
        let (table_entry_bytes, _) = runtime::scalar_split_runtime_layout();
        let split_pair_table_bytes = table_entry_bytes.checked_mul(possible_pair_count)?;
        let total_bytes = split_pair_table_bytes;
        Some(Self {
            possible_pair_count,
            split_pair_table_bytes,
            total_bytes,
            largest_allocation_bytes: split_pair_table_bytes,
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
        let mut next = self.clone();
        next.graph_metadata_bytes = next
            .graph_metadata_bytes
            .checked_add(resource.total_bytes)?;
        next.incremental_plan_bytes = next
            .incremental_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.session_plus_plan_bytes = next
            .session_plus_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.largest_allocation_bytes = next
            .largest_allocation_bytes
            .max(resource.largest_allocation_bytes);
        *self = next;
        Some(())
    }

    pub fn checked_add_scalar_owners(
        &mut self,
        resource: GraphScalarOwnerResourceEstimate,
    ) -> Option<()> {
        let mut next = self.clone();
        next.graph_metadata_bytes = next
            .graph_metadata_bytes
            .checked_add(resource.total_bytes)?;
        next.incremental_plan_bytes = next
            .incremental_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.session_plus_plan_bytes = next
            .session_plus_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.largest_allocation_bytes = next
            .largest_allocation_bytes
            .max(resource.largest_allocation_bytes);
        *self = next;
        Some(())
    }

    /// Folds the retained executor/runtime metadata once, before graph caps are applied.
    pub fn checked_add_runtime_metadata(
        &mut self,
        resource: GraphRuntimeMetadataResourceEstimate,
    ) -> Option<()> {
        let mut next = self.clone();
        next.graph_metadata_bytes = next
            .graph_metadata_bytes
            .checked_add(resource.total_bytes)?;
        next.incremental_plan_bytes = next
            .incremental_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.session_plus_plan_bytes = next
            .session_plus_plan_bytes
            .checked_add(resource.total_bytes)?;
        next.largest_allocation_bytes = next
            .largest_allocation_bytes
            .max(resource.largest_allocation_bytes);
        *self = next;
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
    effects: Vec<GraphPreparedEffect>,
    /// Issue #140 A: one entry per effect a live console drives. Empty for every session that
    /// asked for no console, which is what keeps the runtime on its byte-identical path.
    effect_controls: Vec<GraphEffectControlBinding>,
    /// Issue #143 D3: one entry per effect that has observation taps. Empty for every session
    /// that named no observation capacity, which is what keeps the runtime unobserved *and*
    /// byte-identical rather than merely disabled.
    effect_observations: Vec<GraphEffectObservationBinding>,
    banks: Vec<GraphPreparedEffectBank>,
    builtin_banks: Vec<GraphPreparedBuiltinBank>,
    observers: Vec<GraphNodeObserverBinding>,
    _not_sync: Cell<()>,
}
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

/// One prepared effect's live-console control channel, carried **beside** the prepared effects
/// rather than inside them (issue #140 A).
///
/// # Why beside, and not a field of [`GraphPreparedEffect`]
///
/// `GraphPreparedEffect` is the payload of `runtime::NodeKind::Effect`. Runtime op/unit layout
/// changes are admitted separately by the derived runtime metadata reservation, while this
/// channel remains in its own vector so `NodeKind`'s largest variant stays unchanged. A
/// console-free plan therefore retains no control-channel payload.
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
/// A prepared homogeneous native bank and its original graph member identities.
pub struct GraphPreparedEffectBank {
    pub members: Box<[EffectNodeId]>,
    /// `true` for every lane that carries a member. #96 binds only full groups, so this is all
    /// `true` today; the field exists so a padded group can be bound without a second bank shape.
    pub active_mask: Box<[bool]>,
    pub processor: Box<dyn effect_contract::PreparedNativeEffectBank>,
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
    pub control_delivery: BuiltinControlDelivery,
}

/// How prepared builtin control producers are owned relative to render calls.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub enum BuiltinControlDelivery {
    #[default]
    Concurrent,
    BetweenRenderCalls,
}
/// Render contract for an already-prepared builtin bank.
pub type BuiltinProcessor = Box<dyn GraphPreparedBuiltinBankProcessor>;
pub type BuiltinPairFactory = fn(
    BuiltinProcessor,
    BuiltinProcessor,
) -> Result<BuiltinProcessor, (BuiltinProcessor, BuiltinProcessor)>;

pub trait GraphPreparedBuiltinBankProcessor: Send + Any {
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    /// Preparation metadata only; render never reads this policy.
    fn control_delivery(&self) -> BuiltinControlDelivery {
        BuiltinControlDelivery::Concurrent
    }
    /// Preparation-only pairing hook. Render never queries or uses this metadata.
    fn pair_factory(&self) -> Option<BuiltinPairFactory> {
        None
    }
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError>;
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

    /// Drain this bank's live-console queues, before any lane of the block is dispatched.
    ///
    /// Forwarded from `rack::BankStage::begin_block`, and it carries that method's
    /// whole contract: an admitted record takes effect on the first sample of the block that
    /// drains it, and a record that writes one channel's upstream word clears the
    /// channel-symmetry witness' `LIVE` term *before* the collapse dispatch reads it. A bank
    /// upstream of the fader/matrix seam that drained inside `process` instead would publish a
    /// one-channel retarget onto both channels of the block that admitted it.
    ///
    /// The default is a no-op, which is what a console-free plan pays.
    fn begin_block(&mut self, first_sample: u64) -> Result<(), RenderError> {
        let _ = first_sample;
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
    /// upstream stage that has not written a one-plane body declines the whole chain.
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
    (PreparedRenderPlan, Option<GraphObservationController>),
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
                control_delivery: bank.processor.control_delivery(),
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
                    .any(|node| stage_of(node) != Some(stage) || !seen.insert(node.clone()))
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
        // `program` is *derived* from this plan's own fields (#99 F2), and since #169 the bank
        // member lists are among them: attaching banks changes which buffers may be shared, so
        // the derivation is redone here. Without this, `program()` would describe the plan as it
        // was before the attach while bind-time `lowered()` described it as it is.
        self.program = self.lower_from_current_fields();
        Ok(self)
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
    /// Since #925 `required_bindings` is one of them too: a builtin stage the plan does not list
    /// has nothing to bind, and lowers as an alias (`program::is_builtin_stage`). It is the plan's
    /// own field, so the program `program()` reports and the one bind derives cannot disagree
    /// about it either.
    fn lower_from_current_fields(&self) -> Option<program::ExecutionProgram> {
        program::lower(
            &self.spec,
            &self.sequential_schedule,
            &self.dependency_levels,
            &self.inserted_delays,
            &bank_member_nodes(&self.banks, &self.builtin_banks),
            &self.required_bindings,
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
        // (`program::is_alias_candidate`), and a builtin stage is elided only when it is *not*
        // listed (`program::is_builtin_stage`, #925), so a listed one always keeps its op -- and a
        // hand-built plan that lists a rack boundary is rejected here rather than silently
        // dropping the binding.
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
            effects: parts.effects,
            effect_controls: parts.effect_controls,
            effect_observations: parts.effect_observations,
            banks: parts.banks,
            builtin_banks: parts.builtin_banks,
            observers: parts.observers,
            _not_sync: Cell::new(()),
        };
        plan.program = plan.lower_from_current_fields();
        plan
    }
    /// The failure carries every input back, which is the point; boxing it would only move the
    /// allocation onto the caller's error path.
    #[allow(clippy::result_large_err)]
    pub fn bind(
        self,
        bindings: GraphRuntimeBindings,
    ) -> Result<PreparedRenderPlan, GraphBindFailure> {
        match self.bind_optional_source_set(bindings, None, None) {
            Ok((plan, None)) => Ok(plan),
            Ok((_, Some(_))) => unreachable!("legacy bind cannot prepare activation"),
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
        self.bind_optional_source_set(bindings, Some(source_set), None)
            .map_err(
                |(plan, bindings, source_set, code)| GraphSourceBindFailure {
                    plan: Box::new(plan),
                    bindings,
                    source_set: source_set.expect("source-set bind retains source set"),
                    code,
                },
            )
            .map(|(plan, activation)| {
                if activation.is_some() {
                    unreachable!("legacy source bind cannot prepare activation")
                }
                plan
            })
    }

    /// Bind with a prepared, host-controlled observer activation endpoint.
    #[allow(clippy::result_large_err)]
    pub fn bind_with_observation_activation(
        self,
        bindings: GraphRuntimeBindings,
        config: GraphObservationActivationConfig,
    ) -> Result<(PreparedRenderPlan, GraphObservationController), GraphBindFailure> {
        match self.bind_optional_source_set(bindings, None, Some(config)) {
            Ok((plan, Some(controller))) => Ok((plan, controller)),
            Ok((_, None)) => unreachable!("configured bind must prepare an activation controller"),
            Err((plan, bindings, _, code)) => Err(GraphBindFailure {
                plan: Box::new(plan),
                bindings,
                code,
            }),
        }
    }

    /// Bind one sealed source set with a prepared observer activation endpoint.
    #[allow(clippy::result_large_err)]
    pub fn bind_with_source_set_and_observation_activation(
        self,
        bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
        config: GraphObservationActivationConfig,
    ) -> Result<(PreparedRenderPlan, GraphObservationController), GraphSourceBindFailure> {
        match self.bind_optional_source_set(bindings, Some(source_set), Some(config)) {
            Ok((plan, Some(controller))) => Ok((plan, controller)),
            Ok((_, None)) => {
                unreachable!("configured source bind must prepare an activation controller")
            }
            Err((plan, bindings, source_set, code)) => Err(GraphSourceBindFailure {
                plan: Box::new(plan),
                bindings,
                source_set: source_set.expect("source-set bind retains source set"),
                code,
            }),
        }
    }

    #[allow(clippy::result_large_err)]
    fn bind_optional_source_set(
        self,
        mut bindings: GraphRuntimeBindings,
        mut source_set: Option<GraphPreparedSourceSet>,
        activation_config: Option<GraphObservationActivationConfig>,
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
        let planning =
            match runtime::preflight_sequential(&self, &program, &bindings, source_set.as_ref()) {
                Ok(planning) => planning,
                Err(code) => return Err((self, bindings, source_set, code)),
            };
        let prepared_activation = match runtime::preflight_observation_activation(
            &self,
            &program,
            &bindings,
            &planning,
            activation_config,
        ) {
            Ok(prepared) => prepared,
            Err(code) => return Err((self, bindings, source_set, code)),
        };
        let (controller, realtime_activation) = prepared_activation
            .map_or((None, None), |prepared| {
                (Some(prepared.controller), Some(prepared.realtime))
            });
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
            realtime_activation,
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
        Ok((render_plan, controller))
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
    /// Issue #210 phase 2: input-side track alignment, one entry per delayed track.
    pub track_delays: Vec<PreparedTrackDelay>,
    pub effects: Vec<GraphPreparedEffect>,
    /// Issue #140 A: live-console control channels, one per driven effect node.
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
    fn can_prepare_source_seek(&self, _source_index: usize) -> bool {
        false
    }
    fn prepare_source_seek(&mut self, _source_index: usize, _generation: u64, _frame: u64) -> bool {
        false
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
}

/// The source set as a bank's gather sees it during the unit loop (issue #918): a shared view of
/// the planes `begin_block` played, handed to [`runtime::Runtime::execute`] after the copy loop.
pub(crate) trait GraphSourcePlanes {
    /// [`GraphPreparedSourceSetDriver::played_planes`] after the set's own claim and length
    /// checks.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])>;
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
}

// REALTIME_POLICY_BEGIN
impl GraphSourcePlanes for GraphPreparedSourceSet {
    /// The same claim-index check `copy_track_input` makes, then the driver's planes, each held to
    /// the quantum `copy_track_input` holds its destinations to. A plane of any other length is a
    /// driver fault; it is refused as `None`, which the gather serves as silence, rather than
    /// handed to a gather that would index past it.
    fn played_planes(&self, claim_index: usize) -> Option<(&[f32], &[f32])> {
        if claim_index >= self.claims.len() {
            return None;
        }
        let quantum = self.envelope.quantum.0 as usize;
        self.driver
            .played_planes(claim_index)
            .filter(|(left, right)| left.len() == quantum && right.len() == quantum)
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
}
pub struct GraphBindingBlock<'a> {
    pub left: &'a mut [f32],
    pub right: &'a mut [f32],
    pub first_sample: u64,
}
pub type ScalarPairFactory = fn(
    Box<dyn GraphRuntimeProcessor>,
    Box<dyn GraphRuntimeProcessor>,
) -> Result<
    Box<dyn GraphRuntimeProcessor>,
    (
        Box<dyn GraphRuntimeProcessor>,
        Box<dyn GraphRuntimeProcessor>,
    ),
>;

/// A prepared owner for a serialized scalar fader/matrix interval whose two graph operations
/// remain at their original schedule positions.
///
/// The fader half begins at the fader operation, while the matrix half finishes at the matrix
/// operation. A settled fader may leave its private in-place output unmaterialized between those
/// positions; [`complete_pending`](Self::complete_pending) is the infallible completion path used
/// before an intervening execution or observer error escapes the render call.
pub trait GraphRuntimeSplitPairProcessor: Send + Any {
    /// Runs the original fader boundary, possibly recording a settled fader for later completion.
    fn begin_fader(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError>;

    /// Runs the original matrix boundary and settles any pending fader before returning.
    fn finish_matrix(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError>;

    /// Materializes a pending settled fader in its original output buffer.
    ///
    /// The graph executor supplies a block shape validated by the prepared render quantum, so
    /// this operation cannot fail and never drains the matrix owner.
    fn complete_pending(&mut self, block: GraphBindingBlock<'_>);
}

pub type ScalarSplitPairFactory = fn(
    Box<dyn GraphRuntimeProcessor>,
    Box<dyn GraphRuntimeProcessor>,
) -> Result<
    Box<dyn GraphRuntimeSplitPairProcessor>,
    (
        Box<dyn GraphRuntimeProcessor>,
        Box<dyn GraphRuntimeProcessor>,
    ),
>;

pub trait GraphRuntimeProcessor: Send + Any {
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

    /// Preparation-only hook for the serialized scalar fader/matrix pair.
    /// Render never queries this metadata.
    fn scalar_pair_factory(&self) -> Option<ScalarPairFactory> {
        None
    }

    /// Preparation-only factory for a split-owner serialized fader/matrix interval.
    /// Render never queries this metadata.
    fn scalar_split_pair_factory(&self) -> Option<ScalarSplitPairFactory> {
        None
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
}
/// A bounded observer invoked after its node has completed.
pub trait GraphRuntimeObserver: Send {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError>;

    /// Notify this observer that its controlled activation changed at a block boundary.
    ///
    /// The default is deliberately empty. Implementations that opt into controlled activation
    /// must keep this hook bounded and scalar: it may invalidate a generation or reset a counter,
    /// but it must not allocate, clear a capture buffer, drain a queue, or reset audio state.
    fn activation_changed(&mut self, _active: bool, _generation: u64, _first_sample: u64) {}

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
}
/// One immutable prepared observer binding, ordered by its stable meter handle.
pub struct GraphNodeObserverBinding {
    pub node: GraphNodeId,
    pub handle: u64,
    observer: Box<dyn GraphRuntimeObserver>,
    controlled: bool,
}
impl GraphNodeObserverBinding {
    pub fn new(node: GraphNodeId, handle: u64, observer: Box<dyn GraphRuntimeObserver>) -> Self {
        Self {
            node,
            handle,
            observer,
            controlled: false,
        }
    }

    /// Prepare an observer for host-controlled activation. Controlled bindings start inactive;
    /// the activation snapshot owns their dispatch eligibility after binding.
    pub fn controlled(
        node: GraphNodeId,
        handle: u64,
        observer: Box<dyn GraphRuntimeObserver>,
    ) -> Self {
        Self {
            node,
            handle,
            observer,
            controlled: true,
        }
    }

    pub(crate) const fn is_controlled(&self) -> bool {
        self.controlled
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedRoute {
    pub node: GraphNodeId,
    pub transform: RouteTransform,
}

/// One track's declared input-side time alignment (#210 phase 2).
///
/// Emitted by the compiler **only** for a track that declared a nonzero delay on at least one lane,
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
    /// The `TrackStage::Input` node this delay is applied at.
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
}

/// Same retained executor owner with the split-table field removed from its embedded runtime.
/// This layout witness lets resource accounting charge the field delta while using the complete
/// `GraphExecutor` allocation for the largest-allocation cap.
#[allow(dead_code)]
struct GraphExecutorWithoutSplitPairTable {
    runtime: runtime::RuntimeWithoutSplitPairTable,
    sample_rate_hz: u32,
    source_set: Option<GraphPreparedSourceSet>,
    source_input_buffers: Box<[(usize, u32)]>,
}

/// Same retained executor owner with observation activation state removed from its embedded
/// runtime. The difference from [`GraphExecutor`] is the exact inline runtime contribution used
/// by activation accounting; all other fields stay identical so owner padding is included.
#[allow(dead_code)]
struct GraphExecutorWithoutObservationActivation {
    runtime: runtime::RuntimeWithoutObservationActivation,
    sample_rate_hz: u32,
    source_set: Option<GraphPreparedSourceSet>,
    source_input_buffers: Box<[(usize, u32)]>,
}

pub(crate) fn observation_runtime_layout() -> Option<(u64, u64)> {
    let runtime_state_bytes = runtime::observation_runtime_layout()?;
    let owner_state_bytes = u64::try_from(core::mem::size_of::<GraphExecutor>().checked_sub(
        core::mem::size_of::<GraphExecutorWithoutObservationActivation>(),
    )?)
    .ok()?;
    Some((runtime_state_bytes, owner_state_bytes))
}

#[cfg(test)]
pub(crate) fn test_only_observation_runtime_layout() -> (u64, u64) {
    let runtime_state_bytes = u64::try_from(
        core::mem::size_of::<runtime::Runtime>()
            .checked_sub(core::mem::size_of::<
                runtime::RuntimeWithoutObservationActivation,
            >())
            .expect("observation runtime witness layout"),
    )
    .expect("observation runtime witness fits u64");
    let owner_state_bytes = u64::try_from(
        core::mem::size_of::<GraphExecutor>()
            .checked_sub(core::mem::size_of::<
                GraphExecutorWithoutObservationActivation,
            >())
            .expect("observation executor witness layout"),
    )
    .expect("observation executor witness fits u64");
    (runtime_state_bytes, owner_state_bytes)
}

fn scalar_split_runtime_owner_layout() -> (u64, u64) {
    let field_delta = core::mem::size_of::<GraphExecutor>()
        .checked_sub(core::mem::size_of::<GraphExecutorWithoutSplitPairTable>())
        .expect("split runtime owner field layout is retained");
    (
        u64::try_from(field_delta).expect("split runtime owner field layout fits u64"),
        u64::try_from(core::mem::size_of::<GraphExecutor>())
            .expect("split runtime owner allocation fits u64"),
    )
}

impl GraphExecutor {
    fn new(
        plan: PreparedGraphPlan,
        program: &program::ExecutionProgram,
        bindings: Vec<GraphNodeBinding>,
        observers: Vec<GraphNodeObserverBinding>,
        source_set: Option<GraphPreparedSourceSet>,
        planning: runtime::SequentialPlan,
        observation_activation: Option<
            crate::observation_activation::RealtimeObservationActivation,
        >,
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
        );
        // Issue #918: the claims a bank's gather may read in place, in claim order. Only a driver
        // that lends its played planes offers any, and `build_sequential` decides which of them
        // are bound in place. `test_only_set_source_in_place_declined` offers none, which binds
        // the path every claim took before the issue.
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
        let runtime = runtime::build_sequential(
            program,
            &plan.spec,
            parts,
            frames,
            planning,
            observation_activation,
            &lent_claims,
        );
        // The copy loop fills exactly the claims that are not read in place.
        let source_input_buffers: Box<[(usize, u32)]> = source_input_buffers
            .iter()
            .copied()
            .filter(|&(claim, buffer)| !runtime.source_in_place(claim, buffer))
            .collect();
        Self {
            runtime,
            sample_rate_hz,
            source_set,
            source_input_buffers,
        }
    }

    /// Routes this bind retired into the session Output op's fused reduction (issue #926). A
    /// count, like `bank_route_folds`: the fold renders the same bits, so only a count can say
    /// whether it fired.
    #[cfg(test)]
    fn output_route_folds(&self) -> u64 {
        self.runtime.output_route_folds()
    }
}

impl PreparedPlanExecutor for GraphExecutor {
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
    /// Render one block into `output`, whose two planes are the session Output op's storage for
    /// the block (issue #916). Nothing is copied out of the arena afterwards: the Output op, and a
    /// folded chain whose master it is, write the planes directly.
    ///
    /// # What `output` holds when this returns
    ///
    /// * **`Ok`:** each plane's `frames` words are this block's master. Nothing past `frames` is
    ///   written, so a `plane_stride` wider than the block keeps its padding.
    /// * **Envelope rejection: untouched.** A non-stereo `output` is refused with
    ///   `Buffer(InvalidPlane)`. A plane that is not exactly `lease.frames()` words is refused with
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
        _input: Option<PlanarBufferRef<'_>>,
        mut output: PlanarBufferMut<'_>,
        time: engine::realtime::RenderTime,
    ) -> Result<(), RenderError> {
        let Self {
            runtime,
            sample_rate_hz: _,
            source_set,
            source_input_buffers,
        } = self;
        #[cfg(any(test, feature = "test-support"))]
        let mut probe = test_only_phase_profile::Probe::start();
        let (left, right) = output.stereo_planes_mut()?;
        let Some(mut host) = runtime::HostMaster::new(left, right, runtime.lease.frames()) else {
            return Err(RenderError::InvalidEnvelope);
        };
        // Snapshot publication is the only activation state transition, and it occurs before any
        // source work so observer hooks and the block's source facts share one boundary.
        runtime.begin_observation_block(time.absolute_sample);
        let selective_observation = runtime.has_observation_activation();
        let active_observation = runtime.has_active_observation();
        #[cfg(any(test, feature = "test-support"))]
        if let Some(probe) = probe.as_mut() {
            probe.enter(test_only_phase_profile::SOURCE);
        }
        let source_validity = if let Some(source_set) = source_set.as_mut() {
            if let Err(error) =
                source_set.begin_block(time.absolute_sample, source_set.envelope.quantum.0)
            {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                return Err(error);
            }
            // A claim bound in place is not in this list: its bank's gather reads the played
            // block's planes (issue #918).
            for &(claim, buffer) in source_input_buffers.iter() {
                #[cfg(any(test, feature = "test-support"))]
                runtime::test_only_count_source_copy();
                let (left, right) = runtime.buffer_mut(buffer);
                if let Err(error) = source_set.copy_track_input(claim, left, right) {
                    runtime.invalidate_observers_after_failure(time.absolute_sample);
                    host.silence();
                    return Err(error);
                }
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
        for unit in 0..runtime.units.len() {
            #[cfg(any(test, feature = "test-support"))]
            if let Some(probe) = probe.as_mut() {
                probe.unit(runtime.test_only_unit_phase(unit));
            }
            if let Err(error) =
                runtime.execute(unit, time.absolute_sample, host.reborrow(), sources)
            {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                #[cfg(any(test, feature = "test-support"))]
                if !runtime::test_only_completion_disabled() {
                    runtime.complete_pending(time.absolute_sample);
                }
                #[cfg(any(test, feature = "test-support"))]
                runtime.test_only_capture_failed_buffer();
                #[cfg(not(any(test, feature = "test-support")))]
                runtime.complete_pending(time.absolute_sample);
                return Err(error);
            }
            if !selective_observation {
                if let Err(error) =
                    runtime.observe_unit(unit, time.absolute_sample, source_validity, &host)
                {
                    runtime.invalidate_observers_after_failure(time.absolute_sample);
                    host.silence();
                    #[cfg(any(test, feature = "test-support"))]
                    if !runtime::test_only_completion_disabled() {
                        runtime.complete_pending(time.absolute_sample);
                    }
                    #[cfg(any(test, feature = "test-support"))]
                    runtime.test_only_capture_failed_buffer();
                    #[cfg(not(any(test, feature = "test-support")))]
                    runtime.complete_pending(time.absolute_sample);
                    return Err(error);
                }
            } else if active_observation
                && let Err(error) =
                    runtime.observe_active_unit(unit, time.absolute_sample, source_validity, &host)
            {
                runtime.invalidate_observers_after_failure(time.absolute_sample);
                host.silence();
                #[cfg(any(test, feature = "test-support"))]
                if !runtime::test_only_completion_disabled() {
                    runtime.complete_pending(time.absolute_sample);
                }
                #[cfg(any(test, feature = "test-support"))]
                runtime.test_only_capture_failed_buffer();
                #[cfg(not(any(test, feature = "test-support")))]
                runtime.complete_pending(time.absolute_sample);
                return Err(error);
            }
        }
        #[cfg(any(test, feature = "test-support"))]
        if let Some(probe) = probe {
            probe.finish();
        }
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
            .units
            .iter()
            .zip(self.runtime.identity.iter())
            .enumerate()
            .map(|(unit, (runtime_unit, identity))| PlanUnitEligibility {
                unit: u32::try_from(unit).unwrap_or(u32::MAX),
                banked: identity.banked,
                stages: identity.stages,
                upstream_of_seam_stages: identity.upstream_of_seam_stages,
                lane_tracks: identity.lane_tracks.clone(),
                lane_eligible: runtime_unit.lane_eligibility().into_boxed_slice(),
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
}

pub fn quantum_samples(quantum: QuantumFrames, count: u64) -> Option<u64> {
    u64::from(quantum.0).checked_mul(count)
}

#[cfg(test)]
mod observation_size_accounting {
    //! Issue #143 R7: the byte accounting for what the binding added, derived rather than pinned.
    //!
    //! Runtime op/unit layout is charged by the derived runtime metadata reservation. This phase
    //! answers the narrower observation question with numbers: "what exactly grew, and by how
    //! much".
    //!
    //! The answer is: `ConsoleEffect` grew by exactly one nullable pointer, and nothing else grew
    //! at all, because `ConsoleEffect` is behind a `Box` inside `NodeKind`. Both halves are stated
    //! as identities over `size_of`, so a future field that changes either one fails here instead
    //! of silently moving a reported byte.

    use super::*;
    use core::mem::size_of;

    #[test]
    fn the_observation_lane_costs_one_nullable_pointer_in_the_console_effect() {
        assert_eq!(
            size_of::<Option<Box<effect_contract::ObservationLane>>>(),
            size_of::<usize>(),
            "an absent lane is a null pointer, not a discriminant plus a pointer"
        );
        assert_eq!(
            size_of::<runtime::ConsoleEffect>(),
            size_of::<GraphPreparedEffect>()
                + size_of::<Box<effect_contract::EffectControlLane>>()
                + size_of::<Box<[effect_contract::PreparedAutomationSpan]>>()
                + size_of::<effect_contract::BypassShunt>()
                + size_of::<Option<Box<effect_contract::ObservationLane>>>(),
            "the console effect is exactly its five fields, and #143 added the fifth"
        );
    }

    #[test]
    fn no_reported_runtime_byte_moved() {
        // `NodeKind::ConsoleEffect` carries a `Box`, so the variant is one pointer and the enum's
        // size is still decided by `NodeKind::Effect(GraphPreparedEffect)` -- the same variant that
        // decided it before #140 and before #143.
        assert_eq!(size_of::<Box<runtime::ConsoleEffect>>(), size_of::<usize>());
        assert!(
            size_of::<runtime::NodeKind>() >= size_of::<GraphPreparedEffect>(),
            "the largest variant is still the unobserved prepared effect"
        );
        assert!(
            size_of::<runtime::NodeKind>()
                < size_of::<GraphPreparedEffect>() + size_of::<runtime::ConsoleEffect>(),
            "the console effect is boxed, so it cannot be the enum's size"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::CompensationDelay;
    use conformance::DualAccumulatorDelayFactory;
    use effect_contract::{
        BankWidth, EffectDescriptor, EffectId, EffectProcessBlock, EffectQuality,
        InitialParameterValue, LinkMode, LinkModeSet, NativeEffectFactory, ParameterChannel,
        PortDescriptor, PortId, PortLayout, PortRole, PrepareEffectLimits, PrepareEffectRequest,
        PreparedPorts, PreparedSidechainPort, ProcessReport, ResetKind, StatePayloadError,
        StatePayloadInput, StatePayloadOutput, StatePayloadSizes,
    };
    use engine::LAUNCH_SAMPLE_RATES;
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
        parameters: &[],
        ports: &SUM_PORTS,
        qualities: &[],
        observations: &[],
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
                split_pair_table_bytes: 0,
            }),
            None
        );
        assert_eq!(
            estimate, before,
            "overflow cannot partially mutate the report"
        );
    }

    #[test]
    fn runtime_metadata_charge_covers_mixed_ops_once_and_refuses_overflow() {
        let emitted = 3_u64;
        let resource = GraphRuntimeMetadataResourceEstimate::checked_for(emitted)
            .expect("checked runtime metadata");
        let (op_delta, unit_delta) = runtime::scalar_split_op_layout();
        let (op_size, unit_size) = (
            u64::try_from(core::mem::size_of::<runtime::RuntimeOp>()).expect("op size"),
            u64::try_from(core::mem::size_of::<runtime::RuntimeUnit>()).expect("unit size"),
        );
        let (_, runtime_field) = runtime::scalar_split_runtime_layout();
        let (executor_field, executor_size) = scalar_split_runtime_owner_layout();
        let runtime_state = u64::try_from(
            core::mem::size_of::<runtime::Runtime>()
                .checked_sub(core::mem::size_of::<
                    runtime::RuntimeWithoutObservationActivation,
                >())
                .expect("observation runtime witness layout"),
        )
        .expect("observation runtime witness fits u64");
        let owner_state = u64::try_from(
            core::mem::size_of::<GraphExecutor>()
                .checked_sub(core::mem::size_of::<
                    GraphExecutorWithoutObservationActivation,
                >())
                .expect("observation executor witness layout"),
        )
        .expect("observation executor witness fits u64");
        assert_eq!(resource.runtime_field_bytes, runtime_field);
        assert_eq!(resource.runtime_op_layout_delta_bytes, op_delta);
        assert_eq!(resource.runtime_unit_layout_delta_bytes, unit_delta);
        assert_eq!(resource.runtime_owner_field_bytes, executor_field);
        assert_eq!(resource.observation_runtime_state_bytes, owner_state);
        assert_eq!(runtime::observation_runtime_layout(), Some(runtime_state));
        assert_eq!(
            resource.emitted_op_layout_delta_bytes,
            op_delta.max(unit_delta)
        );
        assert_eq!(resource.runtime_op_containing_bytes, op_size * emitted);
        assert_eq!(resource.runtime_unit_containing_bytes, unit_size * emitted);
        assert_eq!(
            resource.total_bytes,
            runtime_field + op_delta.max(unit_delta) * emitted + owner_state
        );
        assert_eq!(
            resource.largest_allocation_bytes,
            executor_size
                .max(op_size * emitted)
                .max(unit_size * emitted)
        );

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

    #[test]
    fn observation_runtime_state_is_charged_for_empty_graph_and_overlaps_activation_once() {
        let baseline =
            GraphRuntimeMetadataResourceEstimate::checked_for(0).expect("zero-op runtime metadata");
        let runtime_state = u64::try_from(
            core::mem::size_of::<runtime::Runtime>()
                .checked_sub(core::mem::size_of::<
                    runtime::RuntimeWithoutObservationActivation,
                >())
                .expect("observation runtime witness layout"),
        )
        .expect("observation runtime witness fits u64");
        let owner_state = u64::try_from(
            core::mem::size_of::<GraphExecutor>()
                .checked_sub(core::mem::size_of::<
                    GraphExecutorWithoutObservationActivation,
                >())
                .expect("observation executor witness layout"),
        )
        .expect("observation executor witness fits u64");
        assert_eq!(baseline.observation_runtime_state_bytes, owner_state);
        assert_eq!(runtime::observation_runtime_layout(), Some(runtime_state));
        assert_eq!(
            baseline.total_bytes,
            baseline.runtime_field_bytes + owner_state
        );

        let catalog = vec![observation_activation::ActivationBinding {
            handle: 1,
            entry: observation_activation::ActivationEntry {
                unit: 0,
                member: None,
                observer: 0,
                ordinal: 1,
            },
        }]
        .into_boxed_slice();
        let (controller, _realtime) = observation_activation::prepare_activation(
            catalog,
            &[],
            GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            },
        )
        .expect("activation resource report");
        let activation = controller.resources();
        let combined = baseline
            .total_bytes
            .checked_add(activation.retained_bytes)
            .and_then(|total| total.checked_sub(owner_state))
            .expect("combined runtime accounting");
        assert_eq!(
            combined,
            baseline.total_bytes + activation.retained_bytes - activation.runtime_state_bytes
        );
    }

    #[test]
    fn split_table_resource_is_zero_when_declined_and_one_entry_when_eligible() {
        let zero =
            GraphScalarSplitRuntimeResourceEstimate::checked_for(0).expect("zero split table");
        assert_eq!(zero, GraphScalarSplitRuntimeResourceEstimate::default());
        let one =
            GraphScalarSplitRuntimeResourceEstimate::checked_for(1).expect("one split table entry");
        let entry = u64::try_from(core::mem::size_of::<Box<dyn GraphRuntimeSplitPairProcessor>>())
            .expect("split table entry");
        assert_eq!(one.possible_pair_count, 1);
        assert_eq!(one.split_pair_table_bytes, entry);
        assert_eq!(one.total_bytes, entry);
        assert_eq!(one.largest_allocation_bytes, entry);
        assert_eq!(
            GraphScalarSplitRuntimeResourceEstimate::checked_for(u64::MAX),
            None
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
            sample_rate: engine::SampleRateHz(48_000),
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
                track_delays: Vec::new(),
                effects: Vec::new(),
                effect_controls: Vec::new(),
                effect_observations: Vec::new(),
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
                tail: TailSamples::Finite(0),
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
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
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
    // The classical Higham bound this test states independently is a property of the platform's
    // own `powi`, not `math`'s -- proving D9's error bound against `math::powf` would be
    // circular. Issue #98 is the allowlist entry this `#[expect]` replaces
    // (`scripts/check-math-policy.sh`, retired in favour of `clippy.toml`'s `disallowed-methods`).
    #[expect(
        clippy::disallowed_methods,
        reason = "issue #98: platform powi states the D9 error bound independently of math::powi"
    )]
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
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
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
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
                engine::realtime::RenderTime { absolute_sample: 9 },
            )
            .expect("render");
        assert_eq!(report.plan_id, 42);
        assert_eq!(report.next_absolute_sample, 10);
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
            ports: PreparedPorts {
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
                    input: None,
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

    struct DropWitnessObserver {
        drops: Arc<AtomicU64>,
        calls: Arc<AtomicU64>,
    }
    impl Drop for DropWitnessObserver {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
    impl GraphRuntimeObserver for DropWitnessObserver {
        fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    fn activation_error<T>(result: Result<T, &'static str>) -> &'static str {
        match result {
            Err(error) => error,
            Ok(_) => panic!("activation unexpectedly succeeded"),
        }
    }

    fn test_observer() -> Box<dyn GraphRuntimeObserver> {
        Box::new(TapRecorder(
            Arc::new(AtomicU64::new(0)),
            Arc::new(std::sync::Mutex::new(Vec::new())),
        ))
    }

    #[test]
    fn observation_activation_preflight_is_borrowed_and_maps_plain_and_bank_rows() {
        let (mut plan, bindings, input) = binding_plan();
        let calls = Arc::new(AtomicU64::new(0));
        let sink = Arc::new(std::sync::Mutex::new(Vec::new()));
        plan.observers.push(GraphNodeObserverBinding::controlled(
            input,
            7,
            Box::new(TapRecorder(Arc::clone(&calls), Arc::clone(&sink))),
        ));
        let program = plan.program().cloned().expect("lowered");
        let planning = runtime::preflight_sequential(&plan, &program, &bindings, None)
            .expect("sequential plan");
        let observer_count = plan.observers.len();
        let node_count = bindings.nodes.len();
        let config = GraphObservationActivationConfig {
            maximum_active_observers: 1,
            maximum_retained_bytes: u64::MAX,
        };
        let mut prepared = runtime::preflight_observation_activation(
            &plan,
            &program,
            &bindings,
            &planning,
            Some(config),
        )
        .expect("activation preflight")
        .expect("configured activation");
        assert_eq!(plan.observers.len(), observer_count);
        assert_eq!(bindings.nodes.len(), node_count);

        prepared
            .controller
            .replace(&[7])
            .expect("controlled replacement");
        prepared
            .realtime
            .apply_boundary(19, |_entry, _active, _revision, _first_sample| {});
        let entries = prepared.realtime.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].unit, 0);
        assert_eq!(entries[0].member, None);
        assert_eq!(entries[0].observer, 0);
        assert_eq!(entries[0].ordinal, 0);

        let required = activation_error(runtime::preflight_observation_activation(
            &plan, &program, &bindings, &planning, None,
        ));
        assert_eq!(required, "graph.plan.observation_activation_required");
        let capacity = activation_error(runtime::preflight_observation_activation(
            &plan,
            &program,
            &bindings,
            &planning,
            Some(GraphObservationActivationConfig {
                maximum_active_observers: 0,
                maximum_retained_bytes: u64::MAX,
            }),
        ));
        assert_eq!(capacity, "graph.plan.observation_activation_capacity");
        let retained = activation_error(runtime::preflight_observation_activation(
            &plan,
            &program,
            &bindings,
            &planning,
            Some(GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: 0,
            }),
        ));
        assert_eq!(retained, "graph.plan.observation_activation_retained");
        assert_eq!(plan.observers.len(), observer_count);
        assert_eq!(bindings.nodes.len(), node_count);

        let (mut duplicate_plan, duplicate_bindings, input) = binding_plan();
        let output = duplicate_plan
            .spec
            .nodes
            .iter()
            .find_map(|node| matches!(node.id, GraphNodeId::Output { .. }).then(|| node.id.clone()))
            .expect("output");
        duplicate_plan
            .observers
            .push(GraphNodeObserverBinding::controlled(
                input,
                55,
                test_observer(),
            ));
        duplicate_plan
            .observers
            .push(GraphNodeObserverBinding::controlled(
                output,
                55,
                test_observer(),
            ));
        let duplicate_program = duplicate_plan.program().cloned().expect("lowered");
        let duplicate_planning = runtime::preflight_sequential(
            &duplicate_plan,
            &duplicate_program,
            &duplicate_bindings,
            None,
        )
        .expect("sequential plan");
        let duplicate = activation_error(runtime::preflight_observation_activation(
            &duplicate_plan,
            &duplicate_program,
            &duplicate_bindings,
            &duplicate_planning,
            Some(config),
        ));
        assert_eq!(duplicate, "graph.plan.observation_activation_duplicate");

        let (mut unresolved_plan, unresolved_bindings, _) = binding_plan();
        unresolved_plan
            .observers
            .push(GraphNodeObserverBinding::controlled(
                GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse("missing").expect("ID"),
                    stage: TrackStage::PostDynamic,
                },
                81,
                test_observer(),
            ));
        let unresolved_program = unresolved_plan.program().cloned().expect("lowered");
        let unresolved_planning = runtime::preflight_sequential(
            &unresolved_plan,
            &unresolved_program,
            &unresolved_bindings,
            None,
        )
        .expect("sequential plan");
        let unresolved = activation_error(runtime::preflight_observation_activation(
            &unresolved_plan,
            &unresolved_program,
            &unresolved_bindings,
            &unresolved_planning,
            Some(config),
        ));
        assert_eq!(unresolved, "graph.plan.observer");

        let (bank_plan, mut bank_bindings, _) = four_track_builtin_plan(8_160, true, false);
        let bank_tail = bank_plan
            .required_bindings
            .iter()
            .find(|node| {
                matches!(
                    node,
                    GraphNodeId::TrackStage {
                        track_id,
                        stage: TrackStage::PostInputBuiltins,
                    } if track_id.as_str() == "track3"
                )
            })
            .cloned()
            .expect("bank tail");
        bank_bindings
            .observers
            .push(GraphNodeObserverBinding::controlled(
                bank_tail,
                99,
                test_observer(),
            ));
        let bank_program = bank_plan.program().cloned().expect("lowered");
        let bank_planning =
            runtime::preflight_sequential(&bank_plan, &bank_program, &bank_bindings, None)
                .expect("sequential bank plan");
        let mut bank_activation = runtime::preflight_observation_activation(
            &bank_plan,
            &bank_program,
            &bank_bindings,
            &bank_planning,
            Some(GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            }),
        )
        .expect("bank activation preflight")
        .expect("configured bank activation");
        bank_activation
            .controller
            .replace(&[99])
            .expect("bank replacement");
        bank_activation
            .realtime
            .apply_boundary(0, |_entry, _active, _revision, _first_sample| {});
        assert!(
            bank_activation
                .realtime
                .entries()
                .iter()
                .any(|entry| entry.member == Some(3) && entry.observer == 1)
        );
    }

    #[test]
    fn public_observation_activation_bind_dispatches_only_after_boundary_admission() {
        let (plan, mut bindings, input) = binding_plan();
        let calls = Arc::new(AtomicU64::new(0));
        bindings
            .observers
            .push(GraphNodeObserverBinding::controlled(
                input,
                7,
                Box::new(TapRecorder(
                    Arc::clone(&calls),
                    Arc::new(std::sync::Mutex::new(Vec::new())),
                )),
            ));
        let (mut render_plan, mut controller) = match plan.bind_with_observation_activation(
            bindings,
            GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            },
        ) {
            Ok(bound) => bound,
            Err(failure) => panic!("activation bind failed: {}", failure.code),
        };
        let mut output = [0.0; 2];
        let mut render = |sample| {
            render_plan
                .render(
                    engine::realtime::RenderIo {
                        input: None,
                        output: PlanarBufferMut::try_new(&mut output, 2, 1, 1).expect("output"),
                    },
                    engine::realtime::RenderTime {
                        absolute_sample: sample,
                    },
                )
                .expect("render");
        };
        render(0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);
        assert_eq!(controller.replace(&[7]).expect("admit").revision, 1);
        render(1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            controller.try_applied(),
            Some(GraphObservationApplied {
                revision: 1,
                first_sample: 1,
            })
        );
    }

    #[test]
    fn controlled_legacy_bind_refusal_preserves_callers_for_plain_and_source_retry() {
        let processor_addresses = |bindings: &GraphRuntimeBindings| {
            bindings
                .nodes
                .iter()
                .filter_map(|binding| binding.processor.as_ref())
                .map(|processor| core::ptr::from_ref(&**processor).cast::<()>() as usize)
                .collect::<Vec<_>>()
        };
        let render_one = |plan: &mut PreparedRenderPlan, sample| {
            let mut output = [0.0; 2];
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output: PlanarBufferMut::try_new(&mut output, 2, 1, 1).expect("output"),
                },
                engine::realtime::RenderTime {
                    absolute_sample: sample,
                },
            )
            .expect("render");
        };
        let config = GraphObservationActivationConfig {
            maximum_active_observers: 1,
            maximum_retained_bytes: u64::MAX,
        };

        // A legacy bind must refuse a controlled observer before consuming any caller object.
        let (plan, mut bindings, input) = binding_plan();
        let drops = Arc::new(AtomicU64::new(0));
        let calls = Arc::new(AtomicU64::new(0));
        let observer = GraphNodeObserverBinding::controlled(
            input,
            7,
            Box::new(DropWitnessObserver {
                drops: Arc::clone(&drops),
                calls: Arc::clone(&calls),
            }),
        );
        let observer_address = core::ptr::from_ref(&*observer.observer).cast::<()>() as usize;
        bindings.observers.push(observer);
        let processor_before = processor_addresses(&bindings);
        let failure = match plan.bind(bindings) {
            Ok(_) => panic!("legacy bind accepted a controlled observer"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.observation_activation_required");
        assert_eq!(processor_addresses(&failure.bindings), processor_before);
        assert_eq!(failure.bindings.observers.len(), 1);
        let returned_observer = &failure.bindings.observers[0];
        assert!(returned_observer.is_controlled());
        assert_eq!(returned_observer.handle, 7);
        assert_eq!(
            core::ptr::from_ref(&*returned_observer.observer).cast::<()>() as usize,
            observer_address
        );
        assert_eq!(drops.load(Ordering::SeqCst), 0);
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        let (mut render_plan, mut controller) = failure
            .plan
            .bind_with_observation_activation(failure.bindings, config)
            .unwrap_or_else(|failure| panic!("configured retry rejected: {}", failure.code));
        controller
            .replace(&[7])
            .expect("controlled retry admission");
        render_one(&mut render_plan, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        drop(render_plan);
        drop(controller);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        // The source-set variant returns the source driver and every other caller owner as well.
        let (plan, mut bindings, input) = binding_plan();
        let processor_before = {
            bindings.nodes.retain(|binding| binding.node != input);
            processor_addresses(&bindings)
        };
        let drops = Arc::new(AtomicU64::new(0));
        let calls = Arc::new(AtomicU64::new(0));
        let observer = GraphNodeObserverBinding::controlled(
            input.clone(),
            17,
            Box::new(DropWitnessObserver {
                drops: Arc::clone(&drops),
                calls: Arc::clone(&calls),
            }),
        );
        let observer_address = core::ptr::from_ref(&*observer.observer).cast::<()>() as usize;
        bindings.observers.push(observer);
        let begins = Arc::new(AtomicU64::new(0));
        let copies = Arc::new(AtomicU64::new(0));
        let source_drops = Arc::new(AtomicU64::new(0));
        let source = GraphPreparedSourceSet::new(
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
                drops: Arc::clone(&source_drops),
            }),
        );
        let source_address = core::ptr::from_ref(&*source.driver).cast::<()>() as usize;
        let failure = match plan.bind_with_source_set(bindings, source) {
            Ok(_) => panic!("legacy source bind accepted a controlled observer"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.observation_activation_required");
        assert_eq!(processor_addresses(&failure.bindings), processor_before);
        assert_eq!(failure.bindings.observers.len(), 1);
        let returned_observer = &failure.bindings.observers[0];
        assert!(returned_observer.is_controlled());
        assert_eq!(returned_observer.handle, 17);
        assert_eq!(
            core::ptr::from_ref(&*returned_observer.observer).cast::<()>() as usize,
            observer_address
        );
        assert_eq!(
            core::ptr::from_ref(&*failure.source_set.driver).cast::<()>() as usize,
            source_address
        );
        assert_eq!(begins.load(Ordering::SeqCst), 0);
        assert_eq!(copies.load(Ordering::SeqCst), 0);
        assert_eq!(source_drops.load(Ordering::SeqCst), 0);
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        let (mut render_plan, mut controller) = failure
            .plan
            .bind_with_source_set_and_observation_activation(
                failure.bindings,
                failure.source_set,
                config,
            )
            .unwrap_or_else(|failure| panic!("configured source retry rejected: {}", failure.code));
        controller
            .replace(&[17])
            .expect("controlled source retry admission");
        render_one(&mut render_plan, 0);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(begins.load(Ordering::SeqCst), 1);
        assert_eq!(copies.load(Ordering::SeqCst), 1);
        assert_eq!(source_drops.load(Ordering::SeqCst), 0);
        drop(render_plan);
        drop(controller);
        assert_eq!(source_drops.load(Ordering::SeqCst), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn configured_activation_refusals_preserve_inputs_and_exact_budget() {
        let fixture = |handle| {
            let (plan, mut bindings, input) = binding_plan();
            let drops = Arc::new(AtomicU64::new(0));
            let calls = Arc::new(AtomicU64::new(0));
            let observer = GraphNodeObserverBinding::controlled(
                input,
                handle,
                Box::new(DropWitnessObserver {
                    drops: Arc::clone(&drops),
                    calls: Arc::clone(&calls),
                }),
            );
            let address = core::ptr::from_ref(&*observer.observer).cast::<()>() as usize;
            bindings.observers.push(observer);
            (plan, bindings, drops, calls, address)
        };
        let assert_refusal = |failure: GraphBindFailure,
                              expected: &'static str,
                              drops: &Arc<AtomicU64>,
                              calls: &Arc<AtomicU64>,
                              address: usize| {
            assert_eq!(failure.code, expected);
            assert_eq!(failure.bindings.observers.len(), 1);
            let observer = &failure.bindings.observers[0];
            assert!(observer.is_controlled());
            assert_eq!(observer.handle, 7);
            assert_eq!(
                core::ptr::from_ref(&*observer.observer).cast::<()>() as usize,
                address
            );
            assert_eq!(drops.load(Ordering::SeqCst), 0);
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            failure
        };

        let (plan, bindings, drops, calls, address) = fixture(7);
        let failure = match plan.bind_with_observation_activation(
            bindings,
            GraphObservationActivationConfig {
                maximum_active_observers: 0,
                maximum_retained_bytes: u64::MAX,
            },
        ) {
            Ok(_) => panic!("zero-capacity activation bind accepted"),
            Err(failure) => failure,
        };
        let failure = assert_refusal(
            failure,
            "graph.plan.observation_activation_capacity",
            &drops,
            &calls,
            address,
        );
        drop(failure);

        let (plan, bindings, drops, _calls, _address) = fixture(7);
        let (render_plan, controller) = plan
            .bind_with_observation_activation(
                bindings,
                GraphObservationActivationConfig {
                    maximum_active_observers: 1,
                    maximum_retained_bytes: u64::MAX,
                },
            )
            .unwrap_or_else(|failure| panic!("unlimited activation bind: {}", failure.code));
        let retained_bytes = controller.resources().retained_bytes;
        drop(render_plan);
        drop(controller);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        let (plan, bindings, drops, calls, address) = fixture(7);
        let failure = match plan.bind_with_observation_activation(
            bindings,
            GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: retained_bytes - 1,
            },
        ) {
            Ok(_) => panic!("one-below retained-byte activation bind accepted"),
            Err(failure) => failure,
        };
        let failure = assert_refusal(
            failure,
            "graph.plan.observation_activation_retained",
            &drops,
            &calls,
            address,
        );
        drop(failure);

        let (plan, bindings, drops, _calls, _address) = fixture(7);
        let (render_plan, controller) = plan
            .bind_with_observation_activation(
                bindings,
                GraphObservationActivationConfig {
                    maximum_active_observers: 1,
                    maximum_retained_bytes: retained_bytes,
                },
            )
            .unwrap_or_else(|failure| {
                panic!("inclusive retained-byte activation bind: {}", failure.code)
            });
        assert_eq!(controller.resources().retained_bytes, retained_bytes);
        drop(render_plan);
        drop(controller);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        let (mut plan, mut bindings, input) = binding_plan();
        let output = plan
            .required_bindings
            .iter()
            .find(|node| matches!(node, GraphNodeId::Output { .. }))
            .cloned()
            .expect("output");
        let input_drops = Arc::new(AtomicU64::new(0));
        let output_drops = Arc::new(AtomicU64::new(0));
        let input_observer = GraphNodeObserverBinding::controlled(
            input,
            55,
            Box::new(DropWitnessObserver {
                drops: Arc::clone(&input_drops),
                calls: Arc::new(AtomicU64::new(0)),
            }),
        );
        let output_observer = GraphNodeObserverBinding::controlled(
            output,
            55,
            Box::new(DropWitnessObserver {
                drops: Arc::clone(&output_drops),
                calls: Arc::new(AtomicU64::new(0)),
            }),
        );
        let input_address = core::ptr::from_ref(&*input_observer.observer).cast::<()>() as usize;
        let output_address = core::ptr::from_ref(&*output_observer.observer).cast::<()>() as usize;
        plan.observers.push(input_observer);
        bindings.observers.push(output_observer);
        let failure = match plan.bind_with_observation_activation(
            bindings,
            GraphObservationActivationConfig {
                maximum_active_observers: 2,
                maximum_retained_bytes: u64::MAX,
            },
        ) {
            Ok(_) => panic!("duplicate controlled handles accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.observation_activation_duplicate");
        assert_eq!(failure.plan.observers.len(), 1);
        assert_eq!(failure.bindings.observers.len(), 1);
        assert!(failure.plan.observers[0].is_controlled());
        assert!(failure.bindings.observers[0].is_controlled());
        assert_eq!(failure.plan.observers[0].handle, 55);
        assert_eq!(failure.bindings.observers[0].handle, 55);
        assert_eq!(
            core::ptr::from_ref(&*failure.plan.observers[0].observer).cast::<()>() as usize,
            input_address
        );
        assert_eq!(
            core::ptr::from_ref(&*failure.bindings.observers[0].observer).cast::<()>() as usize,
            output_address
        );
        assert_eq!(input_drops.load(Ordering::SeqCst), 0);
        assert_eq!(output_drops.load(Ordering::SeqCst), 0);
        let mut repaired_plan = *failure.plan;
        let repaired_bindings = failure.bindings;
        drop(
            repaired_plan
                .observers
                .pop()
                .expect("duplicate repair observer"),
        );
        let (render_plan, controller) = repaired_plan
            .bind_with_observation_activation(
                repaired_bindings,
                GraphObservationActivationConfig {
                    maximum_active_observers: 1,
                    maximum_retained_bytes: u64::MAX,
                },
            )
            .unwrap_or_else(|failure| panic!("duplicate repair bind: {}", failure.code));
        drop(render_plan);
        drop(controller);
        assert_eq!(input_drops.load(Ordering::SeqCst), 1);
        assert_eq!(output_drops.load(Ordering::SeqCst), 1);

        let (plan, bindings, _) = binding_plan();
        let failure = match plan.bind_with_observation_activation(
            bindings,
            GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            },
        ) {
            Ok(_) => panic!("empty controlled catalog accepted"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "graph.plan.observation_activation_capacity");
        assert!(failure.plan.observers.is_empty());
        assert!(failure.bindings.observers.is_empty());
        drop(failure);
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
            sample_rate: engine::SampleRateHz(48_000),
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
            //
            // The other two builtin stages are listed and acknowledged with the identity, which
            // is what keeps their ops: since issue #925 an unlisted builtin stage is an alias
            // too, and this fixture is about the three rack boundaries alone.
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
        let (alias_plan, mut alias_bindings) = build(
            true,
            Some((
                Arc::new(AtomicU64::new(0)),
                Arc::new(std::sync::Mutex::new(Vec::new())),
            )),
        );
        alias_bindings
            .observers
            .push(GraphNodeObserverBinding::controlled(
                node(TrackStage::PostInputBuiltins),
                101,
                test_observer(),
            ));
        let alias_program = alias_plan.program().cloned().expect("lowered aliases");
        let alias_planning =
            runtime::preflight_sequential(&alias_plan, &alias_program, &alias_bindings, None)
                .expect("sequential alias plan");
        let mut alias_activation = runtime::preflight_observation_activation(
            &alias_plan,
            &alias_program,
            &alias_bindings,
            &alias_planning,
            Some(GraphObservationActivationConfig {
                maximum_active_observers: 1,
                maximum_retained_bytes: u64::MAX,
            }),
        )
        .expect("alias activation preflight")
        .expect("configured alias activation");
        alias_activation
            .controller
            .replace(&[101])
            .expect("alias replacement");
        alias_activation
            .realtime
            .apply_boundary(0, |_entry, _active, _revision, _first_sample| {});
        let alias_entries = alias_activation.realtime.entries();
        assert_eq!(alias_entries.len(), 2);
        assert_eq!(alias_entries[0].unit, alias_entries[1].unit);
        assert_eq!(alias_entries[0].member, None);
        assert_eq!(alias_entries[1].member, None);
        assert_eq!(alias_entries[0].observer, 0);
        assert_eq!(alias_entries[1].observer, 1);

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

    /// SplitMix64, frozen here so the #925 corpus does not depend on host RNG state.
    fn splitmix(state: &mut u64) -> u64 {
        *state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = *state;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A hostile finite sample: `+0.0` and `-0.0`, signed subnormals, and signed normals whose
    /// magnitude spans `2^-24 .. 2^25`, each with a random mantissa.
    fn hostile_sample(state: &mut u64) -> f32 {
        let bits = splitmix(state);
        let sign = ((bits >> 63) as u32) << 31;
        let mantissa = (bits as u32) & 0x007f_ffff;
        match (bits >> 32) % 8 {
            0 => f32::from_bits(sign),
            1 => f32::from_bits(sign | mantissa.max(1)),
            _ => {
                // Biased exponents 103..=151 are the unbiased -24..=24.
                let exponent = 103 + ((bits >> 40) % 49) as u32;
                f32::from_bits(sign | (exponent << 23) | mantissa)
            }
        }
    }

    /// A bound track input that writes a fresh hostile block every call, left and right drawn
    /// independently.
    struct HostileSource(u64);
    impl GraphRuntimeProcessor for HostileSource {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                *left = hostile_sample(&mut self.0);
                *right = hostile_sample(&mut self.0);
            }
            Ok(())
        }
    }

    /// Every observation one observer saw: first sample, then the left and right words.
    struct PlaneRecorder(Arc<std::sync::Mutex<Vec<u64>>>);
    impl GraphRuntimeObserver for PlaneRecorder {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let mut sink = self.0.lock().expect("plane sink");
            sink.push(block.first_sample);
            sink.extend(block.left.iter().map(|sample| u64::from(sample.to_bits())));
            sink.extend(block.right.iter().map(|sample| u64::from(sample.to_bits())));
            Ok(())
        }
    }

    /// Issue #925, gate 1: a builtins-less plan's three builtin stages, unlisted and therefore
    /// lowered as aliases, render the same words as the same plan with them listed and bound to
    /// the identity -- the shape every builtins-less plan had before #925 -- on the host planes
    /// and in every observer window.
    ///
    /// Three tracks, each `Input -> seven stages -> Route -> Output`, the shape
    /// `GraphCompiler::compile` builds for a session with no effects. The input is hostile: signed
    /// zeros, subnormals and magnitudes over `2^-24 .. 2^25`, fresh every block. Every route
    /// carries a hostile 2x2 and gain, and the hostile *input* carries signed zeros and
    /// subnormals, which is what would expose a copy that flipped a sign or flushed a subnormal:
    /// an identity op's only work is the byte copy in `reduce_plane`'s single-input arm, so the
    /// alias holds the same words, `-0.0` included. Class A is the claim: an identity copy
    /// moves no bit and an in-place identity computes nothing, so removing them must leave every
    /// word where it was.
    ///
    /// An observer on each of track 1's three builtin stages records every window it sees. With the
    /// stages listed they fire after their own ops; unlisted they are taps on the input's buffer,
    /// firing right after the input op. The words must be the same words either way.
    ///
    /// Red mutations (`tests/MUTATIONS.md`, #925): keep the post-input stage out of the alias
    /// predicate, or alias only `PostMatrix`, and the op count below fails.
    #[test]
    fn identity_bound_builtin_stages_alias_without_moving_a_bit() {
        const FRAMES: usize = 16;
        const BLOCKS: u64 = 16;
        const TRACKS: usize = 3;
        const OBSERVED: usize = 1;
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostSimd1,
            TrackStage::PostDynamic,
            TrackStage::PostSimd2PreFader,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let builtin_stages = [
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let stage_node = |track: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("t{track}")).expect("track ID"),
            stage,
        };
        let route_node = |track: usize| GraphNodeId::Route {
            route_id: StableGraphId::parse(&format!("r{track}")).expect("route ID"),
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main").expect("output ID"),
        };
        let envelope = RenderEnvelope {
            sample_rate: engine::SampleRateHz(48_000),
            quantum: QuantumFrames(FRAMES as u32),
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let edge = |id: GraphEdgeId, source: &GraphNodeId, destination: &GraphNodeId| GraphEdge {
            id,
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
        // Hostile route constants, drawn once so both arms fold the same words: a gain in
        // `[2^-2, 2^2)` and a 2x2 of signed normals, with track 1's `lr` exactly `-0.0`.
        let mut constants = 0x925_u64;
        let mut hostile_constant = || {
            let bits = splitmix(&mut constants);
            let sign = ((bits >> 63) as u32) << 31;
            let exponent = 125 + ((bits >> 40) % 4) as u32;
            f32::from_bits(sign | (exponent << 23) | ((bits as u32) & 0x007f_ffff))
        };
        let transforms: Vec<RouteTransform> = (0..TRACKS)
            .map(|track| RouteTransform {
                gain: hostile_constant().abs(),
                ll: hostile_constant(),
                lr: if track == 1 { -0.0 } else { hostile_constant() },
                rl: hostile_constant(),
                rr: hostile_constant(),
            })
            .collect();
        assert_eq!(transforms[1].lr.to_bits(), (-0.0_f32).to_bits());

        let build = |listed: bool, sinks: &[Arc<std::sync::Mutex<Vec<u64>>>]| {
            let mut nodes = vec![output.clone()];
            let mut edges = Vec::new();
            let mut routes = Vec::new();
            let mut required = Vec::new();
            let mut bindings = Vec::new();
            for (track, transform) in transforms.iter().enumerate() {
                let chain: Vec<GraphNodeId> = stages
                    .iter()
                    .map(|stage| stage_node(track, *stage))
                    .collect();
                for pair in chain.windows(2) {
                    edges.push(edge(
                        GraphEdgeId::TrackMain {
                            target: pair[1].clone(),
                        },
                        &pair[0],
                        &pair[1],
                    ));
                }
                let route = route_node(track);
                let route_id = StableGraphId::parse(&format!("r{track}")).expect("route ID");
                edges.push(edge(
                    GraphEdgeId::RouteSource {
                        route_id: route_id.clone(),
                    },
                    &chain[6],
                    &route,
                ));
                edges.push(edge(
                    GraphEdgeId::RouteDestination { route_id },
                    &route,
                    &output,
                ));
                routes.push(PreparedRoute {
                    node: route.clone(),
                    transform: *transform,
                });
                nodes.extend(chain);
                nodes.push(route);
                required.push(stage_node(track, TrackStage::Input));
                bindings.push(GraphNodeBinding::new(
                    stage_node(track, TrackStage::Input),
                    Box::new(HostileSource(0x925_0000 + track as u64)),
                ));
                if listed {
                    for stage in builtin_stages {
                        required.push(stage_node(track, stage));
                        bindings.push(GraphNodeBinding::identity(stage_node(track, stage)));
                    }
                }
            }
            required.push(output.clone());
            bindings.push(GraphNodeBinding::identity(output.clone()));
            edges.sort_by(|left, right| left.id.cmp(&right.id));
            let levels = levels_for(&nodes, &edges);
            let schedule: Vec<GraphNodeId> = levels
                .iter()
                .flat_map(|level| level.nodes.iter().cloned())
                .collect();
            let observers = builtin_stages
                .iter()
                .zip(sinks)
                .enumerate()
                .map(|(handle, (stage, sink))| {
                    GraphNodeObserverBinding::new(
                        stage_node(OBSERVED, *stage),
                        handle as u64 + 1,
                        Box::new(PlaneRecorder(Arc::clone(sink))),
                    )
                })
                .collect();
            let plan = PreparedGraphPlan::new(PreparedGraphPlanParts {
                plan_id: 925 + u64::from(listed),
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
                routes,
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
        let sinks = |_: ()| -> Vec<Arc<std::sync::Mutex<Vec<u64>>>> {
            (0..builtin_stages.len())
                .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
                .collect()
        };

        let unlisted_sinks = sinks(());
        let (unlisted, unlisted_bindings) = build(false, &unlisted_sinks);
        let program = unlisted.program().expect("lowered");
        // `Input` and `Route` per track and the output: the three stages are aliases.
        assert_eq!(program.ops.len(), 2 * TRACKS + 1);
        let op_nodes: Vec<&GraphNodeId> = program
            .ops
            .iter()
            .map(|op| &unlisted.spec.nodes[op.node as usize].id)
            .collect();
        assert!(op_nodes.iter().all(|id| matches!(
            id,
            GraphNodeId::TrackStage {
                stage: TrackStage::Input,
                ..
            } | GraphNodeId::Route { .. }
                | GraphNodeId::Output { .. }
        )));
        // The only dedicated storage by kind is the session output's (`program::is_dedicated`:
        // the post-input stage, SIMD-rack effects and the output), and every route runs in place
        // over its input's buffer: one slot per track and the output's.
        let dedicated = op_nodes
            .iter()
            .filter(|id| {
                matches!(
                    id,
                    GraphNodeId::TrackStage {
                        stage: TrackStage::PostInputBuiltins,
                        ..
                    } | GraphNodeId::Effect(_)
                        | GraphNodeId::Output { .. }
                )
            })
            .count();
        assert_eq!(dedicated, 1, "only the output is dedicated storage");
        for op in program.ops.iter() {
            if matches!(
                unlisted.spec.nodes[op.node as usize].id,
                GraphNodeId::Route { .. }
            ) {
                assert!(op.in_place, "a route reads its input's buffer in place");
            }
        }
        assert_eq!(program.buffers as usize, TRACKS + 1);
        assert_eq!(program.taps.len(), 6 * TRACKS);

        let listed_sinks = sinks(());
        let (listed, listed_bindings) = build(true, &listed_sinks);
        assert_eq!(
            listed.program().expect("lowered").ops.len(),
            5 * TRACKS + 1,
            "the old shape: Input, the three stages and the route per track, and the output"
        );

        let mut unlisted_plan = unlisted
            .bind(unlisted_bindings)
            .unwrap_or_else(|failure| panic!("unlisted bind: {}", failure.code));
        let mut listed_plan = listed
            .bind(listed_bindings)
            .unwrap_or_else(|failure| panic!("listed bind: {}", failure.code));
        let unlisted_bits = render_blocks(&mut unlisted_plan, FRAMES, BLOCKS);
        let listed_bits = render_blocks(&mut listed_plan, FRAMES, BLOCKS);
        assert!(unlisted_bits.iter().any(|bits| *bits != 0));
        assert_eq!(
            unlisted_bits, listed_bits,
            "eliding identity stages must not move one bit of the host planes"
        );
        for (stage, (unlisted, listed)) in builtin_stages
            .iter()
            .zip(unlisted_sinks.iter().zip(&listed_sinks))
        {
            let unlisted = unlisted.lock().expect("sink").clone();
            let listed = listed.lock().expect("sink").clone();
            assert_eq!(
                unlisted.len(),
                BLOCKS as usize * (1 + 2 * FRAMES),
                "{stage:?}: one window per block"
            );
            assert_eq!(
                unlisted, listed,
                "{stage:?}: an observer window moved a bit"
            );
        }
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
            sample_rate: engine::SampleRateHz(48_000),
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
            track_delays: Vec::new(),
            effects: Vec::new(),
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
            engine::realtime::RenderIo {
                input: None,
                output,
            },
            engine::realtime::RenderTime { absolute_sample: 0 },
        )
        .expect("render");
        assert_eq!(samples, [0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 30.0, 0.0]);
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
        }];
    static GAIN_DESCRIPTOR: EffectDescriptor = EffectDescriptor {
        id: GAIN_ID,
        display_name: "Live gain fixture",
        contract_major: 1,
        contract_minor: 0,
        state_layout_version: 1,
        supported_link_modes: LinkModeSet::DUAL_MONO,
        parameters: &GAIN_PARAMETERS,
        ports: &SUM_PORTS,
        qualities: &[],
        observations: &[],
    };

    /// A per-channel gain with a real, declared latency, so bypass has something to preserve.
    struct LiveGain {
        metadata: PreparedEffectMetadata,
        gain: [f32; 2],
        line: Vec<[f32; 2]>,
        latency: usize,
        invalid: u64,
    }
    impl LiveGain {
        fn new(metadata: PreparedEffectMetadata) -> Self {
            let latency = metadata.latency.0 as usize;
            Self {
                metadata,
                gain: [1.0, 1.0],
                line: vec![[0.0; 2]; latency],
                latency,
                invalid: 0,
            }
        }
    }
    impl PreparedNativeEffect for LiveGain {
        fn metadata(&self) -> PreparedEffectMetadata {
            self.metadata
        }
        fn reset(&mut self, _kind: ResetKind) {}
        fn process(&mut self, block: EffectProcessBlock<'_>) -> ProcessReport {
            let mut report = ProcessReport::default();
            // The same strictness every launch effect applies: a `Point` at `first_sample` with
            // bit-identical endpoints, addressed to one lane, inside the declared capacity.
            for (index, span) in block.automation.iter().enumerate() {
                let lane = match span.channel {
                    ParameterChannel::Left => 0_usize,
                    ParameterChannel::Right => 1,
                    ParameterChannel::Both => {
                        report.invalid_spans += 1;
                        continue;
                    }
                };
                let valid = index < self.metadata.automation_capacity as usize
                    && span.parameter_index == 0
                    && span.kind == effect_contract::AutomationSpanKind::Point
                    && span.start_sample == block.first_sample
                    && span.end_sample == block.first_sample
                    && span.start_value.to_bits() == span.end_value.to_bits();
                if !valid {
                    report.invalid_spans += 1;
                    continue;
                }
                self.gain[lane] = span.start_value;
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

    /// One track: source input -> one dynamic-rack effect -> output, at a four-frame quantum.
    ///
    /// `control` is the consumer half of the effect's live-console channel, or `None` for the
    /// console-free plan the workspace has always bound.
    fn console_effect_plan(
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
            input_channels: None,
            output_channels: core::num::NonZeroUsize::new(2).expect("two"),
        };
        let metadata = PreparedEffectMetadata {
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
            tail: TailSamples::Finite(0),
            state_sizes: StatePayloadSizes {
                common_bytes: 0,
                left_bytes: 0,
                right_bytes: 0,
            },
            scratch_bytes: 0,
            automation_capacity: 2,
        };
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
            path: "$.console.main".to_owned(),
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
            path: "$.console.output".to_owned(),
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
                            tail: TailSamples::Finite(0),
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
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            for frame in 0..block.left.len() {
                let value = (block.first_sample as usize + frame) as f32;
                block.left[frame] = value;
                block.right[frame] = -value;
            }
            Ok(())
        }
    }

    fn render_console_blocks(
        plan: &mut PreparedRenderPlan,
        start_block: usize,
        blocks: usize,
    ) -> Vec<f32> {
        let mut collected = Vec::new();
        for block in start_block..start_block + blocks {
            let mut samples = [0.0_f32; 8];
            let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
                engine::realtime::RenderTime {
                    absolute_sample: (block * 4) as u64,
                },
            )
            .expect("render");
            collected.extend_from_slice(&samples);
        }
        collected
    }

    fn control_pair(
        depth: usize,
    ) -> (
        engine::realtime::Producer<effect_contract::EffectControlRecord>,
        Box<EffectControlLane>,
    ) {
        let (producer, consumer) =
            engine::realtime::bounded_spsc::<effect_contract::EffectControlRecord>(
                core::num::NonZeroUsize::new(depth).expect("depth"),
                engine::realtime::QueueGeneration(0),
            )
            .expect("queue");
        (producer, Box::new(EffectControlLane::new(consumer, false)))
    }

    /// #140 A / E1 for the dynamic rack: an admitted parameter command takes effect on the first
    /// sample of the next rendered block, and not one sample before.
    ///
    /// Red mutation: move the `console.control.stage(..)` drain in `execute_op`'s `ConsoleEffect`
    /// arm to *after* `effect.processor.process(block)` -> the command lands one block late and
    /// the `block 1` assertion below fails on its first sample.
    #[test]
    fn a_console_parameter_command_applies_at_the_next_block_boundary() {
        let (mut producer, control) = control_pair(4);
        let mut plan = console_effect_plan(0, Some(control), Box::new(SampleIndexSource));
        let block0 = render_console_blocks(&mut plan, 0, 1);
        assert_eq!(
            &block0[..4],
            &[0.0, 1.0, 2.0, 3.0],
            "unity before any command"
        );

        producer
            .try_push(effect_contract::EffectControlRecord::Parameter {
                parameter_index: 0,
                channel: ParameterChannel::Left,
                value: 0.5,
            })
            .expect("room");
        let block1 = render_console_blocks(&mut plan, 1, 1);
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

    /// Live bypass returns the dry signal delayed by exactly the effect's declared latency, so
    /// every PDC route timing the compiler derived from that latency stays correct.
    ///
    /// Red mutation: delete the `console.shunt.capture(..)` call -> the dry buffer keeps its
    /// initial zeros and a bypassed block renders silence instead of the delayed input.
    #[test]
    fn live_bypass_is_latency_preserving_and_reversible() {
        const LATENCY: u64 = 2;
        let (mut producer, control) = control_pair(4);
        let mut plan = console_effect_plan(LATENCY, Some(control), Box::new(SampleIndexSource));
        producer
            .try_push(effect_contract::EffectControlRecord::Parameter {
                parameter_index: 0,
                channel: ParameterChannel::Left,
                value: 0.0,
            })
            .expect("room");
        let wet = render_console_blocks(&mut plan, 0, 2);
        // Latency 2 with a zero gain: the first two samples are the line's zeros, then zeros.
        assert!(
            wet[..4].iter().all(|value| *value == 0.0),
            "a zero gain silences the wet path"
        );

        producer
            .try_push(effect_contract::EffectControlRecord::Bypass(true))
            .expect("room");
        let bypassed = render_console_blocks(&mut plan, 2, 1);
        assert_eq!(
            &bypassed[..4],
            &[6.0, 7.0, 8.0, 9.0],
            "a bypassed block is the input delayed by exactly the declared latency"
        );
        assert_eq!(&bypassed[4..], &[-6.0, -7.0, -8.0, -9.0]);

        producer
            .try_push(effect_contract::EffectControlRecord::Bypass(false))
            .expect("room");
        let restored = render_console_blocks(&mut plan, 3, 1);
        assert!(
            restored[..4].iter().all(|value| *value == 0.0),
            "releasing bypass returns the *current* wet signal, not a stale one: {:?}",
            &restored[..4],
        );
        assert_eq!(
            &restored[4..],
            &[-10.0, -11.0, -12.0, -13.0],
            "the uncommanded right lane keeps its unity wet path at the declared latency"
        );
    }

    /// A console-free plan renders exactly the bits a console-attached plan renders when no
    /// command is ever sent: the feed is inert until something is admitted.
    ///
    /// This is the class-A identity claim in its smallest form -- the same claim the corpus
    /// digests and the three wasm-gate legs make for whole sessions.
    #[test]
    fn an_idle_console_changes_no_rendered_bit() {
        for latency in [0_u64, 3] {
            let mut without = console_effect_plan(latency, None, Box::new(SampleIndexSource));
            let (_producer, control) = control_pair(4);
            let mut with = console_effect_plan(latency, Some(control), Box::new(SampleIndexSource));
            let plain = render_console_blocks(&mut without, 0, 4);
            let console = render_console_blocks(&mut with, 0, 4);
            assert_eq!(
                plain.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                console.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
                "latency={latency}: an idle console is bit-inert",
            );
        }
    }

    /// The staging window cannot overflow, because preparation caps the queue at the effect's
    /// automation capacity; a violated cap would be counted here rather than written past the end.
    #[test]
    fn the_console_effect_drops_nothing_within_its_prepared_capacity() {
        let (mut producer, control) = control_pair(2);
        let mut plan = console_effect_plan(0, Some(control), Box::new(SampleIndexSource));
        for channel in [ParameterChannel::Left, ParameterChannel::Right] {
            producer
                .try_push(effect_contract::EffectControlRecord::Parameter {
                    parameter_index: 0,
                    channel,
                    value: 2.0,
                })
                .expect("room");
        }
        let block = render_console_blocks(&mut plan, 0, 1);
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
            ports: PreparedPorts {
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
            track_delays: Vec::new(),
            effects: vec![GraphPreparedEffect {
                id: effect_id,
                metadata,
                processor: Box::new(SidechainSum { metadata }),
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

    #[test]
    fn faster_main_and_faster_sidechain_align_on_their_typed_ports() {
        for delay_main in [true, false] {
            let mut plan = sidechain_pdc_plan(delay_main);
            let mut samples = [0.0_f32; 8];
            let output = PlanarBufferMut::try_new(&mut samples, 2, 4, 4).expect("output");
            plan.render(
                engine::realtime::RenderIo {
                    input: None,
                    output,
                },
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
        let processor = factory
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
            sample_rate: engine::SampleRateHz(rate),
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
                            engine::realtime::RenderIo {
                                input: None,
                                output,
                            },
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
            input_channels: None,
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
}
