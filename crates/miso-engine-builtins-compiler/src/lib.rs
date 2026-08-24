//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "test-support")]
use core::sync::atomic::{AtomicBool, Ordering};

#[cfg(test)]
use miso_engine_builtins::builtin_filter_cutoff_maximum_hz_v1;

#[cfg(feature = "test-support")]
use std::sync::Mutex;

use sha2::{Digest, Sha256};

use miso_engine_builtins::{
    BuiltinChain, BuiltinInputBankV1, BuiltinParameterError, BuiltinParameters, BuiltinTail,
    ChannelParameters, DualMonoBlock, FaderMuteBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins,
    MeterAccumulator, MeterConfig, MeterConfigError, MeterHandle, MeterSnapshot, MeterTap,
    PreparedMeter, builtin_bank_width, pan_matrix, validate_builtin_filter_cutoff_v1,
};
use miso_engine_core::realtime::{
    Consumer, PreparedRenderPlan, RenderEnvelope, RenderError, bounded_spsc_retained_payload,
};
use miso_engine_graph::{
    DependencyLevel, GraphBindingBlock, GraphBuiltinBankResourceEstimate, GraphNodeId,
    GraphNodeObserverBinding, GraphObservationBlock, GraphPreparedBuiltinBank,
    GraphPreparedBuiltinBankInfo, GraphPreparedBuiltinBankProcessor, GraphPreparedSourceSet,
    GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor, PreparedGraphPlan,
    StableGraphId, TrackStage,
};
use miso_engine_rack::{AoSoaScratch, BankSlotKey, KernelDispatch, RackLocationV1, RackProgramV1};
use miso_engine_rack_compiler::{CohortCandidate, CohortLevel, plan_bank_groups};
use miso_engine_session::{CompiledSession, MatrixOrPan, Track};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinCompileCaps {
    pub maximum_total_state_bytes: u64,
    pub maximum_total_retained_payload_bytes: u64,
    pub maximum_total_meter_items: u64,
    pub maximum_total_meter_bytes: u64,
    pub maximum_single_allocation_bytes: u64,
    pub maximum_meter_streams: u64,
    pub maximum_period_frames: u32,
    pub maximum_peak_hold_frames: u32,
    pub maximum_smoothing_samples: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MeterRequest {
    pub handle: MeterHandle,
    pub track_id: String,
    pub tap: MeterTap,
    pub config: MeterConfig,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinDiagnostic {
    pub code: &'static str,
    pub path: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltinDiagnosticSet(pub Vec<BuiltinDiagnostic>);

impl BuiltinDiagnosticSet {
    pub fn sorted(mut values: Vec<BuiltinDiagnostic>) -> Self {
        values.sort();
        values.dedup();
        Self(values)
    }
}

pub struct MeterConsumer {
    pub handle: MeterHandle,
    pub track_id: Box<str>,
    pub tap: MeterTap,
    pub consumer: Consumer<MeterSnapshot>,
}

/// Opaque, sealed builtin payload. It can only be lowered into a graph once.
pub struct PreparedBuiltinsSession {
    seal: BuiltinSessionSeal,
    processors: Vec<miso_engine_graph::GraphNodeBinding>,
    bank_inputs: Vec<(Box<str>, InputBuiltins)>,
    observers: Vec<GraphNodeObserverBinding>,
    meter_consumers: Vec<MeterConsumer>,
    tails: Vec<(Box<str>, BuiltinTail)>,
    requests: Vec<MeterRequestSeal>,
    resources: BuiltinResourceEstimate,
}

/// Sealed production representation of one full post-input builtin bank.
///
/// It owns the real TPT adapter and is only materialized by the builtin/graph preparation seam.
pub struct PreparedBuiltinInputBankV1 {
    backend: miso_engine_core::KernelBackendV1,
    members: Box<[GraphNodeId]>,
    processor: BuiltinBankProcessor,
    scratch: AoSoaScratch,
}

struct BuiltinBankProcessor {
    bank: BuiltinInputBankV1,
    process_calls: u64,
    frames_processed: u64,
}

impl GraphPreparedBuiltinBankProcessor for BuiltinBankProcessor {
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        let _ = first_sample;
        self.bank.process(left, right, frames);
        self.process_calls = self.process_calls.saturating_add(1);
        self.frames_processed = self.frames_processed.saturating_add(u64::from(frames));
        Ok(())
    }

    fn qualification_counters(&self) -> [u64; 2] {
        [self.process_calls, self.frames_processed]
    }
}

impl PreparedBuiltinInputBankV1 {
    /// The whole-layout resource was already computed once, before the graph was touched; this
    /// only moves ownership, so there is nothing left to recompute here.
    fn into_graph_bank(self) -> GraphPreparedBuiltinBank {
        GraphPreparedBuiltinBank {
            backend: self.backend,
            members: self.members,
            processor: Box::new(self.processor),
            scratch: self.scratch,
        }
    }
}

/// The cohort key of the fixed post-input builtin stage.
///
/// It carries no fields on purpose: the stage is not selectable, its backend and width are fixed
/// for the whole artifact, and its rate and quantum come from the session envelope. Every
/// post-input node is therefore co-bankable with every other one at its dependency level, which
/// is exactly the cohort the planner forms. If a per-track variant is ever added (a quality, a
/// second section order), it becomes a field here and the planner splits the cohorts for free.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct BuiltinStageKeyV1;

impl BankSlotKey for BuiltinStageKeyV1 {}

/// Groups every post-input builtin node of a dependency level into `ceil(n / W)` banks.
///
/// The last bank of a level is short: it holds `1..=W` members and the bank pads the remaining
/// lanes with identity lanes.  Every post-input node on a vector host is therefore a bank member,
/// and scalar post-input bindings survive only when the backend has no bank width at all.
///
/// The grouping itself is **not implemented here**: it delegates to
/// [`miso_engine_rack_compiler::plan_bank_groups`], the workspace's single cohort planner (#96 F1).
/// Padding, level partitioning and the trailing-`None` lane order are that planner's rules, so
/// this function only turns each planned group back into the member list the graph attaches.
fn planned_builtin_bank_members(
    inputs: &[(Box<str>, InputBuiltins)],
    dispatch: KernelDispatch,
    levels: &[DependencyLevel],
) -> Vec<Box<[GraphNodeId]>> {
    let Some(width) = builtin_bank_width(dispatch.backend()) else {
        return Vec::new();
    };
    let level_by_node: BTreeMap<_, _> = levels
        .iter()
        .flat_map(|level| {
            level
                .nodes
                .iter()
                .cloned()
                .map(move |node| (node, level.level))
        })
        .collect();
    let mut by_level = BTreeMap::<u64, Vec<CohortCandidate<GraphNodeId, BuiltinStageKeyV1>>>::new();
    for (track, _) in inputs {
        let node = GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(track).expect("prepared stable ID"),
            stage: TrackStage::PostInputBuiltins,
        };
        if let Some(level) = level_by_node.get(&node).copied() {
            by_level.entry(level).or_default().push(CohortCandidate {
                id: node,
                program: RackProgramV1::new(RackLocationV1::Simd1, vec![BuiltinStageKeyV1]),
            });
        }
    }
    let levels_in: Vec<_> = by_level
        .into_iter()
        .map(|(level, candidates)| CohortLevel { level, candidates })
        .collect();
    let plan = plan_bank_groups(&levels_in, width)
        .expect("one post-input builtin node per track, so ids are unique");
    // Every post-input node is bankable, so the planner's scalar list is empty; if a future stage
    // key ever blocks banking, those tracks simply keep their scalar bindings.
    debug_assert!(plan.scalar.is_empty());
    plan.groups
        .into_iter()
        .map(|group| group.members.into_vec().into_iter().flatten().collect())
        .collect()
}

/// Builds one padded bank from `inputs.len()` tracks in member order.
///
/// Lanes `inputs.len()..width.lanes()` become the bank's identity lanes: the builtins crate owns
/// that contract (`BuiltinInputBankV1::new` accepts `1..=W` inputs and pads the rest), and this
/// is the single call site, so the compiler never builds a mask of its own.
fn build_input_bank(
    dispatch: KernelDispatch,
    width: miso_engine_effect_contract::BankWidth,
    inputs: Vec<InputBuiltins>,
) -> BuiltinInputBankV1 {
    BuiltinInputBankV1::new(dispatch.backend(), width, inputs)
        .expect("planner emits 1..=W members at the width the selected backend chose")
}

fn builtin_bank_resource(
    groups: &[Box<[GraphNodeId]>],
    width: miso_engine_effect_contract::BankWidth,
    quantum: u32,
) -> Option<GraphBuiltinBankResourceEstimate> {
    let bank_count = u64::try_from(groups.len()).ok()?;
    let lanes = u64::from(width.lanes());
    if groups
        .iter()
        .any(|members| members.is_empty() || members.len() as u64 > lanes)
    {
        return None;
    }
    let node_bytes = u64::try_from(core::mem::size_of::<GraphNodeId>()).ok()?;
    let processor_bytes = u64::try_from(core::mem::size_of::<BuiltinBankProcessor>()).ok()?;
    // Two planes: `AoSoaScratch` has no sidechain surface at all (#96 F9 deleted it).
    let scratch_plane_samples = u64::from(quantum).checked_mul(lanes)?;
    let scratch_plane_bytes = scratch_plane_samples.checked_mul(4)?;
    let scratch_samples_per_bank = scratch_plane_samples.checked_mul(2)?;
    let scratch_bytes_per_bank = scratch_samples_per_bank.checked_mul(4)?;
    let mut member_string_bytes = 0_u64;
    let mut largest_member_string = 0_u64;
    let mut payload_bytes = 0_u64;
    let mut largest_member_array = 0_u64;
    for members in groups {
        // A padded bank owns exactly the ids it holds, not a full-width array.
        let member_array_bytes = node_bytes.checked_mul(u64::try_from(members.len()).ok()?)?;
        largest_member_array = largest_member_array.max(member_array_bytes);
        payload_bytes = payload_bytes
            .checked_add(member_array_bytes)?
            .checked_add(processor_bytes)?;
        for member in members.iter() {
            let GraphNodeId::TrackStage { track_id, .. } = member else {
                return None;
            };
            let bytes = u64::try_from(track_id.as_str().len()).ok()?;
            member_string_bytes = member_string_bytes.checked_add(bytes)?;
            largest_member_string = largest_member_string.max(bytes);
        }
    }
    let payload_bytes = payload_bytes.checked_add(member_string_bytes)?;
    let scratch_samples = scratch_samples_per_bank.checked_mul(bank_count)?;
    let scratch_bytes = scratch_bytes_per_bank.checked_mul(bank_count)?;
    let metadata_bytes = u64::try_from(core::mem::size_of::<GraphPreparedBuiltinBank>())
        .ok()?
        .checked_mul(bank_count)?;
    Some(GraphBuiltinBankResourceEstimate {
        bank_count,
        payload_bytes,
        scratch_bytes,
        scratch_samples,
        metadata_bytes,
        largest_allocation_bytes: largest_member_array
            .max(processor_bytes)
            .max(largest_member_string)
            .max(scratch_plane_bytes)
            .max(metadata_bytes),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BuiltinSessionSeal {
    session_sha256: [u8; 32],
    sample_rate: u32,
    quantum: u32,
    tracks: Vec<Box<str>>,
    processors: Vec<(Box<str>, TrackStage)>,
    tails: Vec<(Box<str>, BuiltinTail)>,
    requests: Vec<MeterRequestSeal>,
    observers: Vec<(Box<str>, TrackStage, u64)>,
    consumers: Vec<(u64, Box<str>, MeterTap)>,
    resources: BuiltinResourceEstimate,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct MeterRequestSeal {
    handle: u64,
    track_id: Box<str>,
    tap: MeterTap,
    reset_generation: u64,
    period_frames: u32,
    peak_hold_frames: u32,
    peak_decay_bits: u32,
    queue_capacity: usize,
}

type ObserverSeal = (Box<str>, TrackStage, u64);
type ConsumerSeal = (u64, Box<str>, MeterTap);

/// Test-only phase-two allocation accounting.  The production resource report deliberately
/// remains a layout calculation; this probe independently observes the allocator requests made
/// after phase-one validation has accepted the artifact.
#[cfg(feature = "test-support")]
static TEST_PHASE_TWO_ACTIVE: AtomicBool = AtomicBool::new(false);
#[cfg(feature = "test-support")]
static TEST_PHASE_TWO_LAYOUTS: Mutex<TestPhaseTwoLayoutTable> =
    Mutex::new(TestPhaseTwoLayoutTable::new());

#[cfg(feature = "test-support")]
struct TestPhaseTwoLayoutTable {
    values: [BuiltinRetainedLayoutV1; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
    len: usize,
    overflowed: bool,
}

#[cfg(feature = "test-support")]
impl TestPhaseTwoLayoutTable {
    const fn new() -> Self {
        Self {
            values: [BuiltinRetainedLayoutV1::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
            len: 0,
            overflowed: false,
        }
    }

    fn clear(&mut self) {
        *self = Self::new();
    }

    fn record(&mut self, layout: core::alloc::Layout) {
        let Ok(size_bytes) = u64::try_from(layout.size()) else {
            self.overflowed = true;
            return;
        };
        let Ok(align_bytes) = u64::try_from(layout.align()) else {
            self.overflowed = true;
            return;
        };
        if let Some(value) = self.values[..self.len]
            .iter_mut()
            .find(|value| value.size_bytes == size_bytes && value.align_bytes == align_bytes)
        {
            let Some(count) = value.allocation_count.checked_add(1) else {
                self.overflowed = true;
                return;
            };
            value.allocation_count = count;
            return;
        }
        let Some(slot) = self.values.get_mut(self.len) else {
            self.overflowed = true;
            return;
        };
        *slot = BuiltinRetainedLayoutV1 {
            size_bytes,
            align_bytes,
            allocation_count: 1,
        };
        self.len += 1;
    }
}

/// Independent test-only phase-two allocation observation.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub struct TestPhaseTwoAllocationSnapshot {
    pub total_bytes: u64,
    pub largest_allocation_bytes: u64,
    pub allocation_count: u64,
    pub layouts: Vec<BuiltinRetainedLayoutV1>,
    pub overflowed: bool,
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_reset_phase_two_allocation_tracker() {
    TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.clear();
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_record_phase_two_allocation(layout: core::alloc::Layout) {
    if !TEST_PHASE_TWO_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    if let Ok(mut table) = TEST_PHASE_TWO_LAYOUTS.lock() {
        table.record(layout);
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_phase_two_allocation_snapshot() -> TestPhaseTwoAllocationSnapshot {
    let table = TEST_PHASE_TWO_LAYOUTS
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut layouts = table.values[..table.len].to_vec();
    layouts.sort();
    let mut total_bytes = 0_u64;
    let mut allocation_count = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    let mut overflowed = table.overflowed;
    for layout in &layouts {
        let Some(bytes) = layout.size_bytes.checked_mul(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        let Some(total) = total_bytes.checked_add(bytes) else {
            overflowed = true;
            continue;
        };
        total_bytes = total;
        let Some(count) = allocation_count.checked_add(layout.allocation_count) else {
            overflowed = true;
            continue;
        };
        allocation_count = count;
        largest_allocation_bytes = largest_allocation_bytes.max(layout.size_bytes);
    }
    TestPhaseTwoAllocationSnapshot {
        total_bytes,
        largest_allocation_bytes,
        allocation_count,
        layouts,
        overflowed,
    }
}

#[cfg(feature = "test-support")]
struct TestPhaseTwoAllocationGuard;
#[cfg(feature = "test-support")]
impl TestPhaseTwoAllocationGuard {
    fn begin() -> Self {
        TEST_PHASE_TWO_ACTIVE.store(true, Ordering::SeqCst);
        Self
    }
}
#[cfg(feature = "test-support")]
impl Drop for TestPhaseTwoAllocationGuard {
    fn drop(&mut self) {
        TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    }
}

/// A graph plus genuine compiler-owned builtin bindings with no public parts-extraction seam.
///
/// `R` is caller-owned immutable graph-report metadata. All provenance-bearing fields stay
/// private to this crate, including the unbound graph and the concrete processor/observer parts.
pub struct PreparedBuiltinsGraphArtifact<R> {
    graph: PreparedGraphPlan,
    builtin_processors: Vec<miso_engine_graph::GraphNodeBinding>,
    builtin_observers: Vec<GraphNodeObserverBinding>,
    report: R,
    meter_consumers: Vec<MeterConsumer>,
}

/// The one-way result of consuming and binding a sealed builtin graph artifact.
pub struct PreparedBuiltinsGraphBound {
    pub plan: PreparedRenderPlan,
    pub meter_consumers: Vec<MeterConsumer>,
}

/// Native dependency-wave result of consuming and binding a sealed builtin graph artifact.
#[cfg(not(target_arch = "wasm32"))]
pub struct PreparedBuiltinsNativeGraphBound {
    pub prepared: miso_engine_graph::PreparedNativeGraphPlanV1,
    pub meter_consumers: Vec<MeterConsumer>,
}

/// A rejected external binding preserves the opaque artifact and caller-owned bindings.
pub struct PreparedBuiltinsGraphBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub code: &'static str,
}

/// A rejected source-set binding preserves the opaque artifact and every caller-owned input.
pub struct PreparedBuiltinsGraphSourceBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub source_set: GraphPreparedSourceSet,
    pub code: &'static str,
}

/// A rejected native binding preserves the opaque artifact and every caller-owned input.
#[cfg(not(target_arch = "wasm32"))]
pub struct PreparedBuiltinsNativeGraphBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
    pub config: miso_engine_graph::NativeGraphBindConfigV1,
    pub code: &'static str,
}

/// Deliberate seal corruption available only to the graph compiler's adversarial tests.
///
/// This is not a wire format and is compiled out of production artifacts.  Keeping each seal
/// field independently reachable lets the graph boundary prove that it rejects the exact
/// corrupted tuple before it consumes either prepared input.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruption {
    /// The prepared session identity does not match the graph session.
    SessionIdentity,
    /// The sealed track set does not match the prepared bindings.
    Tracks,
    /// The sealed processor set does not match the prepared bindings.
    Processors,
    /// The retained tail records do not match their seal.
    Tails,
    /// The sealed meter request records do not match their seal.
    Requests,
    /// The sealed observer records do not match their bindings.
    Observers,
    /// The sealed consumer records do not match their queues.
    Consumers,
    /// The sealed resource report does not match the retained report.
    Resources,
}

/// Frozen corruption subcases within the eight seal categories.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum PreparedBuiltinsCorruptionCase {
    SessionHash,
    SessionRate,
    SessionQuantum,
    TrackMissing,
    TrackExtra,
    TrackDuplicate,
    ProcessorMissing,
    ProcessorExtra,
    ProcessorChangedStage,
    TailMissing,
    TailExtra,
    TailChanged,
    RequestMissing,
    RequestExtra,
    RequestDuplicate,
    ObserverMissing,
    ObserverExtra,
    ObserverChangedNode,
    ConsumerMissing,
    ConsumerExtra,
    ConsumerChangedMetadata,
    ConsumerDuplicateHandle,
    ResourceReport,
}

#[cfg(feature = "test-support")]
impl PreparedBuiltinsCorruptionCase {
    #[must_use]
    pub const fn category(self) -> PreparedBuiltinsCorruption {
        match self {
            Self::SessionHash | Self::SessionRate | Self::SessionQuantum => {
                PreparedBuiltinsCorruption::SessionIdentity
            }
            Self::TrackMissing | Self::TrackExtra | Self::TrackDuplicate => {
                PreparedBuiltinsCorruption::Tracks
            }
            Self::ProcessorMissing | Self::ProcessorExtra | Self::ProcessorChangedStage => {
                PreparedBuiltinsCorruption::Processors
            }
            Self::TailMissing | Self::TailExtra | Self::TailChanged => {
                PreparedBuiltinsCorruption::Tails
            }
            Self::RequestMissing | Self::RequestExtra | Self::RequestDuplicate => {
                PreparedBuiltinsCorruption::Requests
            }
            Self::ObserverMissing | Self::ObserverExtra | Self::ObserverChangedNode => {
                PreparedBuiltinsCorruption::Observers
            }
            Self::ConsumerMissing
            | Self::ConsumerExtra
            | Self::ConsumerChangedMetadata
            | Self::ConsumerDuplicateHandle => PreparedBuiltinsCorruption::Consumers,
            Self::ResourceReport => PreparedBuiltinsCorruption::Resources,
        }
    }
}

/// The stable-ID grammar admits at most 127 distinct string allocation sizes. The remaining
/// entries cover every fixed processor, vector, endpoint and queue layout class without imposing
/// any limit on track, meter or allocation counts.
pub const BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY: usize = 160;

/// One exact `(size, alignment)` class in the retained allocation multiset.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct BuiltinRetainedLayoutV1 {
    pub size_bytes: u64,
    pub align_bytes: u64,
    pub allocation_count: u64,
}

impl BuiltinRetainedLayoutV1 {
    const ZERO: Self = Self {
        size_bytes: 0,
        align_bytes: 0,
        allocation_count: 0,
    };
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinResourceEstimate {
    /// Exact engine-owned processor, seal and binding payload bytes retained by this artifact.
    pub engine_owned_processor_payload_bytes: u64,
    /// Exact engine-owned meter and queue payload bytes retained by this artifact.
    pub engine_owned_meter_payload_bytes: u64,
    /// Exact total of all engine-owned retained payload bytes in this artifact.
    pub engine_owned_retained_payload_bytes: u64,
    pub meter_items: u64,
    /// Largest requested engine-owned payload allocation retained by this artifact.
    pub maximum_single_allocation_bytes: u64,
    /// Count of retained engine-owned payload allocations represented by this report.
    pub retained_allocation_count: u64,
    /// Number of populated entries in [`Self::retained_layouts`].
    pub retained_layout_class_count: u16,
    /// Exact ordered multiset classes for all retained allocation requests.
    pub retained_layouts: [BuiltinRetainedLayoutV1; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for BuiltinResourceEstimate {
    fn default() -> Self {
        Self {
            engine_owned_processor_payload_bytes: 0,
            engine_owned_meter_payload_bytes: 0,
            engine_owned_retained_payload_bytes: 0,
            meter_items: 0,
            maximum_single_allocation_bytes: 0,
            retained_allocation_count: 0,
            retained_layout_class_count: 0,
            retained_layouts: [BuiltinRetainedLayoutV1::ZERO;
                BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl BuiltinResourceEstimate {
    /// Populated exact retained layout classes in deterministic `(size, align)` order.
    #[must_use]
    pub fn retained_layouts(&self) -> &[BuiltinRetainedLayoutV1] {
        &self.retained_layouts[..usize::from(self.retained_layout_class_count)]
    }
}

/// Versioned public name for the exact engine-owned retained-payload report.
///
/// This is deliberately an alias rather than a duplicate accounting type: a caller cannot
/// accidentally read one report while the compiler validates another.
pub type BuiltinResourceReportV1 = BuiltinResourceEstimate;

#[derive(Clone, Copy, Debug)]
struct ResourceAccumulator {
    total: u64,
    largest: u64,
    allocations: u64,
    layout_class_count: u16,
    layouts: [BuiltinRetainedLayoutV1; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
}

impl Default for ResourceAccumulator {
    fn default() -> Self {
        Self {
            total: 0,
            largest: 0,
            allocations: 0,
            layout_class_count: 0,
            layouts: [BuiltinRetainedLayoutV1::ZERO; BUILTIN_RETAINED_LAYOUT_CLASS_CAPACITY],
        }
    }
}

impl ResourceAccumulator {
    fn add_layout(&mut self, layout: core::alloc::Layout) -> Option<()> {
        let size_bytes = u64::try_from(layout.size()).ok()?;
        let align_bytes = u64::try_from(layout.align()).ok()?;
        self.add_layout_count(size_bytes, align_bytes, 1)?;
        Some(())
    }

    fn add_layout_count(
        &mut self,
        size_bytes: u64,
        align_bytes: u64,
        allocation_count: u64,
    ) -> Option<()> {
        let retained_bytes = size_bytes.checked_mul(allocation_count)?;
        self.total = self.total.checked_add(retained_bytes)?;
        self.largest = self.largest.max(size_bytes);
        self.allocations = self.allocations.checked_add(allocation_count)?;
        let populated = usize::from(self.layout_class_count);
        if let Some(layout) = self.layouts[..populated]
            .iter_mut()
            .find(|layout| layout.size_bytes == size_bytes && layout.align_bytes == align_bytes)
        {
            layout.allocation_count = layout.allocation_count.checked_add(allocation_count)?;
            return Some(());
        }
        let slot = self.layouts.get_mut(populated)?;
        *slot = BuiltinRetainedLayoutV1 {
            size_bytes,
            align_bytes,
            allocation_count,
        };
        self.layout_class_count = self.layout_class_count.checked_add(1)?;
        Some(())
    }

    fn merge(&mut self, other: Self) -> Option<()> {
        for layout in &other.layouts[..usize::from(other.layout_class_count)] {
            self.add_layout_count(
                layout.size_bytes,
                layout.align_bytes,
                layout.allocation_count,
            )?;
        }
        Some(())
    }

    fn sorted_layouts(mut self) -> Self {
        self.layouts[..usize::from(self.layout_class_count)].sort();
        self
    }

    fn add_bytes(&mut self, bytes: usize) -> Option<()> {
        self.add_layout(core::alloc::Layout::from_size_align(bytes, 1).ok()?)
    }
}

#[derive(Clone, Copy, Debug)]
struct BuiltinResourcePlan {
    report: BuiltinResourceEstimate,
}

impl PreparedBuiltinsSession {
    /// Read-only retained-payload resource report.
    #[must_use]
    pub const fn resource_report(&self) -> BuiltinResourceEstimate {
        self.resources
    }

    /// Number of sealed builtin processor bindings.
    #[must_use]
    pub fn processor_count(&self) -> usize {
        self.processors.len()
    }

    /// Number of sealed builtin tails.
    #[must_use]
    pub fn tail_count(&self) -> usize {
        self.tails.len()
    }

    /// Number of sealed meter observer bindings.
    #[must_use]
    pub fn observer_count(&self) -> usize {
        self.observers.len()
    }

    /// Number of sealed meter consumer endpoints.
    #[must_use]
    pub fn meter_consumer_count(&self) -> usize {
        self.meter_consumers.len()
    }

    /// Read-only builtin tails used by graph lowering.
    pub fn tails(&self) -> impl Iterator<Item = (&str, BuiltinTail)> {
        self.tails
            .iter()
            .map(|(track, tail)| (track.as_ref(), *tail))
    }

    /// Validate the immutable payload against the exact effect-prepared session.
    pub fn validate_for_session(&self, session: &CompiledSession) -> BuiltinDiagnosticSet {
        let mut diagnostics = Vec::new();
        if self.seal.session_sha256 != session_identity(session)
            || self.seal.sample_rate != session.sample_rate().0
            || self.seal.quantum != session.quantum().0
        {
            diagnostics.push(diag("builtin.session.mismatch", "$.session"));
        }
        let expected_tracks: Vec<Box<str>> = session
            .normalized_model()
            .tracks
            .iter()
            .map(|track| track.id.as_str().into())
            .collect();
        if self.seal.tracks != expected_tracks {
            diagnostics.push(diag("builtin.prepared.track_set", "$.builtins.tracks"));
        }
        let expected_processors = processor_seal(&expected_tracks);
        if self.seal.processors != expected_processors
            || !processors_match(&self.processors, &expected_processors)
        {
            diagnostics.push(diag(
                "builtin.prepared.processor_set",
                "$.builtins.processors",
            ));
        }
        let expected_tails = match expected_tails(session) {
            Ok(value) => value,
            Err(()) => {
                diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
                Vec::new()
            }
        };
        if self.seal.tails != expected_tails || self.tails != expected_tails {
            diagnostics.push(diag("builtin.prepared.tail_set", "$.builtins.tails"));
        }
        let (actual_observers, actual_consumers) =
            actual_meter_seals(&self.observers, &self.meter_consumers);
        if self.seal.requests != self.requests {
            diagnostics.push(diag(
                "builtin.prepared.request_set",
                "$.builtins.meter_requests",
            ));
        }
        if self.seal.observers != actual_observers {
            diagnostics.push(diag(
                "builtin.prepared.observer_set",
                "$.builtins.observers",
            ));
        }
        if self.seal.consumers != actual_consumers {
            diagnostics.push(diag(
                "builtin.prepared.consumer_set",
                "$.builtins.meter_consumers",
            ));
        }
        if self.seal.resources != self.resources {
            diagnostics.push(diag(
                "builtin.prepared.resource_report",
                "$.builtins.resources",
            ));
        }
        BuiltinDiagnosticSet::sorted(diagnostics)
    }

    /// Seal an already session-validated graph around these genuine compiler-owned bindings.
    ///
    /// This is deliberately a one-way conversion: callers may carry and bind the resulting
    /// artifact, but cannot extract, replace, or clone its provenance-bearing parts.
    pub fn into_graph_artifact<R>(
        self,
        graph: PreparedGraphPlan,
        report: R,
    ) -> PreparedBuiltinsGraphArtifact<R> {
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: self.processors,
            builtin_observers: self.observers,
            report,
            meter_consumers: self.meter_consumers,
        }
    }

    /// Exact retained resource addition for the selected production bank layout.
    ///
    /// This is a read-only transactional preflight: graph/session caps can reject the final
    /// artifact while both prepared inputs are still owned by their caller.
    pub fn graph_builtin_bank_resource(
        &self,
        dispatch: KernelDispatch,
        levels: &[DependencyLevel],
    ) -> Option<GraphBuiltinBankResourceEstimate> {
        let Some(width) = builtin_bank_width(dispatch.backend()) else {
            return Some(GraphBuiltinBankResourceEstimate::default());
        };
        let groups = planned_builtin_bank_members(&self.bank_inputs, dispatch, levels);
        builtin_bank_resource(&groups, width, self.seal.quantum)
    }

    /// Materialize post-input builtin banks using the already-selected host dispatch.
    ///
    /// Every post-input node in a level with a vector backend is banked; the last bank of a
    /// level is padded with identity lanes.  Scalar `InputProcessor` bindings remain only when
    /// `dispatch.bank_width()` is `None`.
    ///
    /// Lowering is infallible after `graph_builtin_bank_resource`: `with_builtin_banks` consumes
    /// the plan on error, so the read-only preflight is what makes the attach transactional and
    /// the `expect`s here are this crate's own planner invariants.
    pub fn into_graph_artifact_with_banks<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
        dispatch: KernelDispatch,
        levels: &[DependencyLevel],
    ) -> PreparedBuiltinsGraphArtifact<R> {
        let Some(width) = builtin_bank_width(dispatch.backend()) else {
            return self.into_graph_artifact(graph, report);
        };
        let groups = planned_builtin_bank_members(&self.bank_inputs, dispatch, levels);
        if groups.is_empty() {
            return self.into_graph_artifact(graph, report);
        }
        let resource = builtin_bank_resource(&groups, width, self.seal.quantum)
            .expect("preflighted builtin-bank resource");
        let mut bank_inputs: BTreeMap<Box<str>, InputBuiltins> =
            core::mem::take(&mut self.bank_inputs).into_iter().collect();
        let mut selected = BTreeSet::new();
        let mut banks = Vec::with_capacity(groups.len());
        for members in groups {
            let inputs = members
                .iter()
                .map(|member| {
                    let GraphNodeId::TrackStage { track_id, .. } = member else {
                        unreachable!("prepared builtin member shape")
                    };
                    bank_inputs
                        .remove(track_id.as_str())
                        .expect("planner members are owned prepared builtin tracks")
                })
                .collect();
            let bank = build_input_bank(dispatch, width, inputs);
            selected.extend(members.iter().cloned());
            banks.push(PreparedBuiltinInputBankV1 {
                backend: dispatch.backend(),
                members,
                processor: BuiltinBankProcessor {
                    bank,
                    process_calls: 0,
                    frames_processed: 0,
                },
                scratch: AoSoaScratch::new(width, self.seal.quantum)
                    .expect("prepared nonzero graph quantum"),
            });
        }
        self.processors
            .retain(|binding| !selected.contains(&binding.node));
        let graph_banks: Vec<_> = banks
            .into_iter()
            .map(PreparedBuiltinInputBankV1::into_graph_bank)
            .collect();
        let graph = graph
            .with_builtin_banks(graph_banks, resource)
            .expect("validated fixed builtin member shape");
        PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: self.processors,
            builtin_observers: self.observers,
            report,
            meter_consumers: self.meter_consumers,
        }
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_corrupt_for_compiler_test(
        &mut self,
        corruption: PreparedBuiltinsCorruptionCase,
    ) {
        match corruption {
            PreparedBuiltinsCorruptionCase::SessionHash => self.seal.session_sha256[0] ^= 1,
            PreparedBuiltinsCorruptionCase::SessionRate => {
                self.seal.sample_rate = self.seal.sample_rate.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::SessionQuantum => {
                self.seal.quantum = self.seal.quantum.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruptionCase::TrackMissing => self.seal.tracks.clear(),
            PreparedBuiltinsCorruptionCase::TrackExtra => {
                self.seal.tracks.push("forged-track".into());
            }
            PreparedBuiltinsCorruptionCase::TrackDuplicate => {
                if let Some(track) = self.seal.tracks.first().cloned() {
                    self.seal.tracks.push(track);
                }
            }
            PreparedBuiltinsCorruptionCase::ProcessorMissing => {
                self.processors.pop();
            }
            PreparedBuiltinsCorruptionCase::ProcessorExtra => self
                .seal
                .processors
                .push(("forged-processor".into(), TrackStage::PostMatrix)),
            PreparedBuiltinsCorruptionCase::ProcessorChangedStage => {
                if let Some((_, stage)) = self.seal.processors.first_mut() {
                    *stage = TrackStage::Input;
                }
            }
            PreparedBuiltinsCorruptionCase::TailMissing => {
                self.tails.pop();
            }
            PreparedBuiltinsCorruptionCase::TailExtra => self
                .seal
                .tails
                .push(("forged-tail".into(), BuiltinTail::FiniteZero)),
            PreparedBuiltinsCorruptionCase::TailChanged => {
                if let Some((_, tail)) = self.tails.first_mut() {
                    *tail = match *tail {
                        BuiltinTail::FiniteZero => BuiltinTail::Infinite,
                        BuiltinTail::Infinite => BuiltinTail::FiniteZero,
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::RequestMissing => {
                self.requests.pop();
            }
            PreparedBuiltinsCorruptionCase::RequestExtra => {
                self.seal.requests.push(forged_request_seal());
            }
            PreparedBuiltinsCorruptionCase::RequestDuplicate => {
                if let Some(request) = self.requests.first().cloned() {
                    self.requests.push(request);
                }
            }
            PreparedBuiltinsCorruptionCase::ObserverMissing => {
                self.observers.pop();
            }
            PreparedBuiltinsCorruptionCase::ObserverExtra => {
                self.seal
                    .observers
                    .push(("forged-observer".into(), TrackStage::Input, u64::MAX))
            }
            PreparedBuiltinsCorruptionCase::ObserverChangedNode => {
                if let Some(observer) = self.observers.first_mut() {
                    observer.node = GraphNodeId::Output {
                        output_id: StableGraphId::parse("forged-output").expect("stable test ID"),
                    };
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerMissing => {
                self.meter_consumers.pop();
            }
            PreparedBuiltinsCorruptionCase::ConsumerExtra => {
                self.seal
                    .consumers
                    .push((u64::MAX, "forged-consumer".into(), MeterTap::Input))
            }
            PreparedBuiltinsCorruptionCase::ConsumerChangedMetadata => {
                if let Some(consumer) = self.meter_consumers.first_mut() {
                    consumer.track_id = "forged-consumer".into();
                    consumer.tap = MeterTap::PostMatrix;
                }
            }
            PreparedBuiltinsCorruptionCase::ConsumerDuplicateHandle => {
                if self.meter_consumers.len() >= 2 {
                    self.meter_consumers[1].handle = self.meter_consumers[0].handle;
                }
            }
            PreparedBuiltinsCorruptionCase::ResourceReport => {
                self.resources.engine_owned_retained_payload_bytes = self
                    .resources
                    .engine_owned_retained_payload_bytes
                    .checked_add(1)
                    .unwrap_or(0);
            }
        }
    }
}

#[cfg(feature = "test-support")]
fn forged_request_seal() -> MeterRequestSeal {
    MeterRequestSeal {
        handle: u64::MAX,
        track_id: "forged-request".into(),
        tap: MeterTap::Input,
        reset_generation: 0,
        period_frames: 1,
        peak_hold_frames: 0,
        peak_decay_bits: 0,
        queue_capacity: 1,
    }
}

impl<R> PreparedBuiltinsGraphArtifact<R> {
    /// Immutable caller-owned graph report.
    #[must_use]
    /// The sealed graph, by shared reference.
    ///
    /// Read-only, and deliberately so: #99 F5 stopped `GraphCompileReport` from carrying its own
    /// copy of the plan's vectors, so the callers that used to read them from the report read
    /// them here instead. The seal's compile-fail doctests still hold -- a `&` cannot extract,
    /// clone or mutate the artifact's provenance.
    pub const fn graph(&self) -> &PreparedGraphPlan {
        &self.graph
    }
    pub const fn report(&self) -> &R {
        &self.report
    }

    /// Envelope required by the still-unbound graph.
    #[must_use]
    pub const fn envelope(&self) -> RenderEnvelope {
        self.graph.envelope
    }

    /// Number of sealed production post-input builtin banks retained by this artifact.
    #[must_use]
    pub const fn prepared_builtin_bank_count(&self) -> usize {
        self.graph.prepared_builtin_bank_count()
    }

    /// Address-free backend, width, member and active-mask metadata for qualification.
    pub fn prepared_builtin_banks(&self) -> impl Iterator<Item = GraphPreparedBuiltinBankInfo<'_>> {
        self.graph.builtin_bank_info()
    }

    /// Exact graph-owned storage after retained builtin-bank attachment.
    #[must_use]
    pub const fn graph_resource_estimate(&self) -> &miso_engine_graph::GraphResourceEstimate {
        &self.graph.estimate
    }

    /// Ordinary external nodes required in addition to compiler-owned builtin processors.
    pub fn external_binding_nodes(&self) -> impl Iterator<Item = &GraphNodeId> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| &binding.node)
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        self.graph
            .required_bindings
            .iter()
            .filter(move |node| !builtin_nodes.contains(node) && !bank_nodes.contains(node))
    }

    /// Consume the sealed wrapper and attach its private builtin bindings exactly once.
    #[allow(clippy::result_large_err)]
    pub fn into_bound(
        mut self,
        mut bindings: GraphRuntimeBindings,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphBindFailure<R>> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        let expected: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(observer.node, GraphNodeId::TrackStage { .. })
                    && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        if bindings.envelope != self.graph.envelope
            || duplicate_nodes
            || overlaps_builtin
            || supplied != expected
            || !valid_observers
        {
            let code = if !valid_observers {
                "graph.plan.observer"
            } else if bindings.envelope != self.graph.envelope {
                "graph.plan.envelope_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err(PreparedBuiltinsGraphBindFailure {
                artifact: self,
                bindings,
                code,
            });
        }
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
        let plan = match self.graph.bind(bindings) {
            Ok(plan) => plan,
            Err(_) => unreachable!("sealed wrapper prevalidated its complete graph bindings"),
        };
        Ok(PreparedBuiltinsGraphBound {
            plan,
            meter_consumers: self.meter_consumers,
        })
    }

    /// Consume the sealed wrapper and bind one coordinator-owned source set.
    ///
    /// The wrapper first applies the same builtin-node and observer prevalidation as
    /// [`Self::into_bound`]. It then appends only its genuine private bindings and delegates the
    /// source claims to the graph's transactional source-set bind. Every rejection returns the
    /// opaque artifact, caller bindings, and source set without cloning or exposing sealed parts.
    #[allow(clippy::result_large_err)]
    pub fn into_bound_with_source_set(
        mut self,
        mut bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
    ) -> Result<PreparedBuiltinsGraphBound, PreparedBuiltinsGraphSourceBindFailure<R>> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().collect();
        let expected: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let source_nodes: BTreeSet<_> = source_set
            .claims()
            .iter()
            .map(|claim| claim.node.clone())
            .collect();
        let mut all_supplied = supplied.clone();
        all_supplied.extend(source_nodes);
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let builtin_observer_pairs: BTreeSet<_> = self
            .builtin_observers
            .iter()
            .map(|observer| (observer.node.clone(), observer.handle))
            .collect();
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(observer.node, GraphNodeId::TrackStage { .. })
                    && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        if bindings.envelope != self.graph.envelope
            || duplicate_nodes
            || overlaps_builtin
            || all_supplied != expected
            || !valid_observers
        {
            let code = if !valid_observers {
                "graph.plan.observer"
            } else if bindings.envelope != self.graph.envelope {
                "graph.plan.envelope_mismatch"
            } else if duplicate_nodes || overlaps_builtin {
                "graph.plan.binding"
            } else if all_supplied != expected {
                "source.graph.binding_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err(PreparedBuiltinsGraphSourceBindFailure {
                artifact: self,
                bindings,
                source_set,
                code,
            });
        }
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
        match self.graph.bind_with_source_set(bindings, source_set) {
            Ok(plan) => Ok(PreparedBuiltinsGraphBound {
                plan,
                meter_consumers: self.meter_consumers,
            }),
            Err(failure) => {
                let mut builtin_processors = Vec::new();
                let mut external_processors = Vec::new();
                for binding in failure.bindings.nodes {
                    if builtin_nodes.contains(&binding.node) {
                        builtin_processors.push(binding);
                    } else {
                        external_processors.push(binding);
                    }
                }
                let mut builtin_observers = Vec::new();
                let mut external_observers = Vec::new();
                for observer in failure.bindings.observers {
                    if builtin_observer_pairs.contains(&(observer.node.clone(), observer.handle)) {
                        builtin_observers.push(observer);
                    } else {
                        external_observers.push(observer);
                    }
                }
                Err(PreparedBuiltinsGraphSourceBindFailure {
                    artifact: PreparedBuiltinsGraphArtifact {
                        graph: *failure.plan,
                        builtin_processors,
                        builtin_observers,
                        report: self.report,
                        meter_consumers: self.meter_consumers,
                    },
                    bindings: GraphRuntimeBindings {
                        #[cfg(not(target_arch = "wasm32"))]
                        worker_lease: None,
                        envelope: failure.bindings.envelope,
                        nodes: external_processors,
                        observers: external_observers,
                    },
                    source_set: failure.source_set,
                    code: failure.code,
                })
            }
        }
    }

    /// Consume the sealed wrapper into the ownership-split native dependency-wave executor.
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(clippy::result_large_err)]
    pub fn into_bound_native(
        mut self,
        mut bindings: GraphRuntimeBindings,
        config: miso_engine_graph::NativeGraphBindConfigV1,
    ) -> Result<PreparedBuiltinsNativeGraphBound, PreparedBuiltinsNativeGraphBindFailure<R>> {
        let builtin_nodes: BTreeSet<_> = self
            .builtin_processors
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let bank_nodes: BTreeSet<_> = self.graph.builtin_bank_members().cloned().collect();
        let expected: BTreeSet<_> = self
            .graph
            .required_bindings
            .iter()
            .filter(|node| !builtin_nodes.contains(*node) && !bank_nodes.contains(*node))
            .cloned()
            .collect();
        let supplied: BTreeSet<_> = bindings
            .nodes
            .iter()
            .map(|binding| binding.node.clone())
            .collect();
        let duplicate_nodes = supplied.len() != bindings.nodes.len();
        let overlaps_builtin = supplied.iter().any(|node| builtin_nodes.contains(node));
        let builtin_observer_pairs: BTreeSet<_> = self
            .builtin_observers
            .iter()
            .map(|observer| (observer.node.clone(), observer.handle))
            .collect();
        let mut observer_pairs = BTreeSet::new();
        let valid_observers = bindings
            .observers
            .iter()
            .chain(self.builtin_observers.iter())
            .all(|observer| {
                matches!(observer.node, GraphNodeId::TrackStage { .. })
                    && observer_pairs.insert((observer.node.clone(), observer.handle))
            });
        if bindings.envelope != self.graph.envelope
            || duplicate_nodes
            || overlaps_builtin
            || supplied != expected
            || !valid_observers
        {
            let code = if !valid_observers {
                "graph.plan.observer"
            } else if bindings.envelope != self.graph.envelope {
                "graph.plan.envelope_mismatch"
            } else {
                "graph.plan.binding"
            };
            return Err(PreparedBuiltinsNativeGraphBindFailure {
                artifact: self,
                bindings,
                config,
                code,
            });
        }
        bindings.nodes.append(&mut self.builtin_processors);
        bindings.observers.append(&mut self.builtin_observers);
        match self.graph.bind_native(bindings, config) {
            Ok(prepared) => Ok(PreparedBuiltinsNativeGraphBound {
                prepared,
                meter_consumers: self.meter_consumers,
            }),
            Err(failure) => {
                let mut builtin_processors = Vec::new();
                let mut external_processors = Vec::new();
                for binding in failure.bindings.nodes {
                    if builtin_nodes.contains(&binding.node) {
                        builtin_processors.push(binding);
                    } else {
                        external_processors.push(binding);
                    }
                }
                let mut builtin_observers = Vec::new();
                let mut external_observers = Vec::new();
                for observer in failure.bindings.observers {
                    if builtin_observer_pairs.contains(&(observer.node.clone(), observer.handle)) {
                        builtin_observers.push(observer);
                    } else {
                        external_observers.push(observer);
                    }
                }
                Err(PreparedBuiltinsNativeGraphBindFailure {
                    artifact: PreparedBuiltinsGraphArtifact {
                        graph: *failure.plan,
                        builtin_processors,
                        builtin_observers,
                        report: self.report,
                        meter_consumers: self.meter_consumers,
                    },
                    bindings: GraphRuntimeBindings {
                        #[cfg(not(target_arch = "wasm32"))]
                        worker_lease: None,
                        envelope: failure.bindings.envelope,
                        nodes: external_processors,
                        observers: external_observers,
                    },
                    config: failure.config,
                    code: failure.code,
                })
            }
        }
    }
}

fn session_identity(session: &CompiledSession) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(session.canonical_toml().as_bytes());
    hash.update(session.sample_rate().0.to_le_bytes());
    hash.update(session.quantum().0.to_le_bytes());
    hash.finalize().into()
}

fn processor_seal(tracks: &[Box<str>]) -> Vec<(Box<str>, TrackStage)> {
    let capacity = tracks
        .len()
        .checked_mul(3)
        .expect("session preparation preflighted processor count");
    let mut values = Vec::with_capacity(capacity);
    for track in tracks {
        for stage in [
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ] {
            values.push((track.clone(), stage));
        }
    }
    values.sort_unstable();
    values
}

fn processors_match(
    processors: &[miso_engine_graph::GraphNodeBinding],
    expected: &[(Box<str>, TrackStage)],
) -> bool {
    let mut actual: Vec<_> = processors
        .iter()
        .filter_map(|binding| match &binding.node {
            GraphNodeId::TrackStage { track_id, stage } => {
                Some((Box::<str>::from(track_id.as_str()), *stage))
            }
            _ => None,
        })
        .collect();
    actual.sort();
    actual.len() == processors.len() && actual == expected
}

fn expected_tails(session: &CompiledSession) -> Result<Vec<(Box<str>, BuiltinTail)>, ()> {
    let mut values: Vec<(Box<str>, BuiltinTail)> =
        Vec::with_capacity(session.normalized_model().tracks.len());
    for track in &session.normalized_model().tracks {
        let parameters = track_parameters(track, u32::MAX).map_err(|_| ())?;
        let chain = BuiltinChain::new(session.sample_rate().0, parameters).map_err(|_| ())?;
        values.push((track.id.as_str().into(), chain.tail()));
    }
    values.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    Ok(values)
}

fn actual_meter_seals(
    observers: &[GraphNodeObserverBinding],
    consumers: &[MeterConsumer],
) -> (Vec<ObserverSeal>, Vec<ConsumerSeal>) {
    let mut observer_values = Vec::with_capacity(observers.len());
    for observer in observers {
        if let GraphNodeId::TrackStage { track_id, stage } = &observer.node {
            observer_values.push((Box::<str>::from(track_id.as_str()), *stage, observer.handle));
        }
    }
    observer_values.sort_unstable();
    let mut consumer_values = Vec::with_capacity(consumers.len());
    for consumer in consumers {
        consumer_values.push((
            consumer.handle.0.get(),
            Box::<str>::from(&*consumer.track_id),
            consumer.tap,
        ));
    }
    consumer_values.sort_unstable();
    (observer_values, consumer_values)
}

pub fn prepare_session_builtins(
    session: &CompiledSession,
    requests: &[MeterRequest],
    caps: BuiltinCompileCaps,
) -> Result<PreparedBuiltinsSession, BuiltinDiagnosticSet> {
    let mut diagnostics = Vec::new();
    if [
        caps.maximum_total_state_bytes,
        caps.maximum_total_retained_payload_bytes,
        caps.maximum_total_meter_items,
        caps.maximum_total_meter_bytes,
        caps.maximum_single_allocation_bytes,
        caps.maximum_meter_streams,
    ]
    .into_iter()
    .any(|value| value == 0)
    {
        diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
    }
    match u64::try_from(requests.len()) {
        Ok(count) if count > caps.maximum_meter_streams => {
            diagnostics.push(diag("builtin.resource.limit", "$.meter_requests"));
        }
        Err(_) => diagnostics.push(diag(
            "builtin.resource.arithmetic_overflow",
            "$.meter_requests",
        )),
        Ok(_) => {}
    }
    let mut request_keys = BTreeSet::new();
    let mut request_handles = BTreeSet::new();
    for request in requests {
        if !request_handles.insert(request.handle) {
            diagnostics.push(diag("builtin.meter.duplicate_handle", &meter_path(request)));
        }
        let key = (request.track_id.clone(), request.tap);
        if !request_keys.insert(key) {
            diagnostics.push(diag("builtin.meter.duplicate", &meter_path(request)));
        }
        if request.config.period_frames.get() > caps.maximum_period_frames
            || request.config.peak_hold_frames > caps.maximum_peak_hold_frames
        {
            diagnostics.push(diag("builtin.resource.limit", &meter_path(request)));
        }
        if !request.config.peak_decay_db_per_second.is_finite()
            || !(0.0..=120.0).contains(&request.config.peak_decay_db_per_second)
        {
            diagnostics.push(diag("builtin.meter.config", &meter_path(request)));
        }
    }
    let known_tracks: BTreeSet<_> = session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect();
    for request in requests {
        if !known_tracks.contains(request.track_id.as_str()) {
            diagnostics.push(diag("builtin.meter.unknown_track", &meter_path(request)));
        }
    }
    for track in &session.normalized_model().tracks {
        match track_parameters(track, caps.maximum_smoothing_samples)
            .and_then(|parameters| BuiltinChain::new(session.sample_rate().0, parameters))
        {
            Ok(_) => {}
            Err(error) => {
                diagnostics.push(parameter_diagnostic(track, error, session.sample_rate().0))
            }
        }
    }
    let resource_plan = match resource_plan(session, requests) {
        Ok(value) => Some(value),
        Err(error) => {
            diagnostics.push(error);
            None
        }
    };
    if let Some(plan) = resource_plan {
        let report = plan.report;
        if report.engine_owned_processor_payload_bytes > caps.maximum_total_state_bytes
            || report.engine_owned_retained_payload_bytes
                > caps.maximum_total_retained_payload_bytes
            || report.meter_items > caps.maximum_total_meter_items
            || report.engine_owned_meter_payload_bytes > caps.maximum_total_meter_bytes
            || report.maximum_single_allocation_bytes > caps.maximum_single_allocation_bytes
        {
            diagnostics.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
        }
    }
    if !diagnostics.is_empty() {
        return Err(BuiltinDiagnosticSet::sorted(diagnostics));
    }
    let resources = resource_plan.expect("validated resource plan").report;
    #[cfg(feature = "test-support")]
    let _phase_two_tracker = TestPhaseTwoAllocationGuard::begin();
    let track_count = session.normalized_model().tracks.len();
    let processor_count = track_count
        .checked_mul(3)
        .expect("preflighted processor count");
    let mut processors = Vec::with_capacity(processor_count);
    let mut bank_inputs = Vec::with_capacity(track_count);
    let mut tails = Vec::with_capacity(track_count);
    for track in &session.normalized_model().tracks {
        let parameters = track_parameters(track, caps.maximum_smoothing_samples)
            .expect("preflighted parameters");
        let chain = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted coefficients");
        let tail = chain.tail();
        let (input, fader, matrix) = chain.into_sections();
        // The bank candidate is prepared independently from the scalar fallback.  The selected
        // full-bank artifact consumes this copy and removes the corresponding scalar binding;
        // the scalar input remains the transactional fallback until that point.
        let bank_input = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted bank coefficients")
            .into_input_builtins();
        tails.push((Box::<str>::from(track.id.as_str()), tail));
        bank_inputs.push((Box::<str>::from(track.id.as_str()), bank_input));
        let graph_id = StableGraphId::parse(track.id.as_str()).expect("preflighted stable ID");
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id.clone(), TrackStage::PostInputBuiltins),
            Box::new(InputProcessor(input)),
        ));
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id.clone(), TrackStage::PostFader),
            Box::new(FaderProcessor(fader)),
        ));
        processors.push(miso_engine_graph::GraphNodeBinding::new(
            stage_node(graph_id, TrackStage::PostMatrix),
            Box::new(MatrixProcessor(matrix)),
        ));
    }
    let mut observers = Vec::with_capacity(requests.len());
    let mut meter_consumers = Vec::with_capacity(requests.len());
    let mut request_seals = Vec::with_capacity(requests.len());
    for request in requests {
        let handle = request.handle;
        let PreparedMeter {
            accumulator,
            consumer,
        } = MeterAccumulator::prepare(handle, request.config, session.sample_rate().0).map_err(
            |error| BuiltinDiagnosticSet::sorted(vec![meter_diagnostic(request, error)]),
        )?;
        let graph_id = StableGraphId::parse(&request.track_id).expect("known accepted session ID");
        observers.push(GraphNodeObserverBinding::new(
            stage_node(graph_id, stage(request.tap)),
            handle.0.get(),
            Box::new(MeterObserver(accumulator)),
        ));
        meter_consumers.push(MeterConsumer {
            handle,
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            consumer,
        });
        request_seals.push(MeterRequestSeal {
            handle: handle.0.get(),
            track_id: request.track_id.as_str().into(),
            tap: request.tap,
            reset_generation: request.config.reset_generation,
            period_frames: request.config.period_frames.get(),
            peak_hold_frames: request.config.peak_hold_frames,
            peak_decay_bits: request.config.peak_decay_db_per_second.to_bits(),
            queue_capacity: request.config.queue_capacity.get(),
        });
    }
    tails.sort_unstable_by(|left, right| left.0.cmp(&right.0));
    request_seals.sort_unstable();
    let tracks: Vec<Box<str>> = session
        .normalized_model()
        .tracks
        .iter()
        .map(|track| track.id.as_str().into())
        .collect();
    let processor_seal = processor_seal(&tracks);
    let (observer_seal, consumer_seal) = actual_meter_seals(&observers, &meter_consumers);
    Ok(PreparedBuiltinsSession {
        seal: BuiltinSessionSeal {
            session_sha256: session_identity(session),
            sample_rate: session.sample_rate().0,
            quantum: session.quantum().0,
            tracks,
            processors: processor_seal,
            tails: tails.clone(),
            requests: request_seals.clone(),
            observers: observer_seal,
            consumers: consumer_seal,
            resources,
        },
        processors,
        bank_inputs,
        observers,
        meter_consumers,
        tails,
        requests: request_seals,
        resources,
    })
}

fn resource_plan(
    session: &CompiledSession,
    requests: &[MeterRequest],
) -> Result<BuiltinResourcePlan, BuiltinDiagnostic> {
    let track_count = session.normalized_model().tracks.len();
    let processor_count = track_count
        .checked_mul(3)
        .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
    let request_count = requests.len();
    let mut processor = ResourceAccumulator::default();
    let mut meter = ResourceAccumulator::default();
    add_vector_layout::<miso_engine_graph::GraphNodeBinding>(&mut processor, processor_count)?;
    add_vector_layout::<(Box<str>, InputBuiltins)>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    add_vector_layout::<Box<str>>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, TrackStage)>(&mut processor, processor_count)?;
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    for track in &session.normalized_model().tracks {
        let bytes = track.id.as_str().len();
        // The three graph IDs are independently cloned into their stage bindings, alongside the
        // retained tail, compact track seal, processor seal, the seal's cloned tail ID, and
        // the independently retained bank-input candidate ID.
        for _ in 0..10 {
            processor
                .add_bytes(bytes)
                .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
        }
        processor
            .add_layout(core::alloc::Layout::new::<InputProcessor>())
            .and_then(|_| processor.add_layout(core::alloc::Layout::new::<FaderProcessor>()))
            .and_then(|_| processor.add_layout(core::alloc::Layout::new::<MatrixProcessor>()))
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", "$.tracks"))?;
    }
    add_vector_layout::<GraphNodeObserverBinding>(&mut meter, request_count)?;
    add_vector_layout::<MeterConsumer>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<MeterRequestSeal>(&mut meter, request_count)?;
    add_vector_layout::<(Box<str>, TrackStage, u64)>(&mut meter, request_count)?;
    add_vector_layout::<(u64, Box<str>, MeterTap)>(&mut meter, request_count)?;
    let mut meter_items = 0_u64;
    for request in requests {
        let queue =
            bounded_spsc_retained_payload::<MeterSnapshot>(request.config.queue_capacity)
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter_items = meter_items
            .checked_add(
                u64::try_from(queue.slot_count).map_err(|_| {
                    diag("builtin.resource.arithmetic_overflow", &meter_path(request))
                })?,
            )
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        meter
            .add_layout(
                core::alloc::Layout::from_size_align(
                    queue.ring_header_bytes,
                    queue.ring_header_align,
                )
                .map_err(|_| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?,
            )
            .and_then(|_| {
                core::alloc::Layout::from_size_align(
                    queue.slot_payload_bytes,
                    queue.slot_payload_align,
                )
                .ok()
                .and_then(|layout| meter.add_layout(layout))
            })
            .and_then(|_| meter.add_layout(core::alloc::Layout::new::<MeterObserver>()))
            .ok_or_else(|| diag("builtin.resource.arithmetic_overflow", &meter_path(request)))?;
        let bytes = request.track_id.len();
        // Observer graph ID, public consumer ID, retained request, and three seal identities.
        for _ in 0..6 {
            meter.add_bytes(bytes).ok_or_else(|| {
                diag("builtin.resource.arithmetic_overflow", &meter_path(request))
            })?;
        }
    }
    let total = processor.total.checked_add(meter.total).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let allocations = processor
        .allocations
        .checked_add(meter.allocations)
        .ok_or_else(|| {
            diag(
                "builtin.resource.arithmetic_overflow",
                "$.builtin_compile_caps",
            )
        })?;
    let mut retained = processor;
    retained.merge(meter).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    let retained = retained.sorted_layouts();
    Ok(BuiltinResourcePlan {
        report: BuiltinResourceEstimate {
            engine_owned_processor_payload_bytes: processor.total,
            engine_owned_meter_payload_bytes: meter.total,
            engine_owned_retained_payload_bytes: total,
            meter_items,
            maximum_single_allocation_bytes: processor.largest.max(meter.largest),
            retained_allocation_count: allocations,
            retained_layout_class_count: retained.layout_class_count,
            retained_layouts: retained.layouts,
        },
    })
}

fn add_vector_layout<T>(
    accumulator: &mut ResourceAccumulator,
    items: usize,
) -> Result<(), BuiltinDiagnostic> {
    if items == 0 {
        return Ok(());
    }
    let layout = core::alloc::Layout::array::<T>(items).map_err(|_| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })?;
    accumulator.add_layout(layout).ok_or_else(|| {
        diag(
            "builtin.resource.arithmetic_overflow",
            "$.builtin_compile_caps",
        )
    })
}

struct InputProcessor(InputBuiltins);
impl GraphRuntimeProcessor for InputProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
}
struct FaderProcessor(FaderMuteBuiltins);
impl GraphRuntimeProcessor for FaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
}
struct MatrixProcessor(MatrixBuiltins);
impl GraphRuntimeProcessor for MatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        let block = DualMonoBlock::new(block.left, block.right, block.first_sample)
            .map_err(render_error)?;
        self.0.process(block);
        Ok(())
    }
}
struct MeterObserver(MeterAccumulator);
impl GraphRuntimeObserver for MeterObserver {
    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.0
            .observe(block.left, block.right, block.first_sample)
            .map_err(|error| match error {
                miso_engine_builtins::MeterObservationError::SampleTimeOverflow => {
                    RenderError::TimeOverflow
                }
                miso_engine_builtins::MeterObservationError::LaneLength => {
                    RenderError::InvalidEnvelope
                }
            })
    }
}

fn render_error(error: BuiltinParameterError) -> RenderError {
    match error {
        BuiltinParameterError::SampleTimeOverflow => RenderError::TimeOverflow,
        _ => RenderError::InvalidEnvelope,
    }
}

fn track_parameters(
    track: &Track,
    maximum_smoothing: u32,
) -> Result<BuiltinParameters, BuiltinParameterError> {
    let left = ChannelParameters {
        polarity_invert: track.builtins.left.polarity_invert,
        trim_db: track.builtins.left.trim_db,
        hpf_hz: track.builtins.left.hpf_hz,
        lpf_hz: track.builtins.left.lpf_hz,
        fader_db: track.fader.left_db,
        muted: track.fader.left_mute,
    };
    let right = ChannelParameters {
        polarity_invert: track.builtins.right.polarity_invert,
        trim_db: track.builtins.right.trim_db,
        hpf_hz: track.builtins.right.hpf_hz,
        lpf_hz: track.builtins.right.lpf_hz,
        fader_db: track.fader.right_db,
        muted: track.fader.right_mute,
    };
    let (matrix, smoothing_samples) = match track.matrix_or_pan {
        MatrixOrPan::Pan {
            left,
            right,
            smoothing_samples,
        } => (pan_matrix(left, right)?, smoothing_samples),
        MatrixOrPan::Matrix {
            ll,
            lr,
            rl,
            rr,
            smoothing_samples,
        } => (Matrix2x2 { ll, lr, rl, rr }.checked()?, smoothing_samples),
    };
    if smoothing_samples > maximum_smoothing {
        return Err(BuiltinParameterError::MatrixSmoothing);
    }
    Ok(BuiltinParameters {
        left,
        right,
        matrix,
        smoothing_samples,
    })
}

fn stage(tap: MeterTap) -> TrackStage {
    match tap {
        MeterTap::Input => TrackStage::Input,
        MeterTap::PostInputBuiltins => TrackStage::PostInputBuiltins,
        MeterTap::PostSimd1 => TrackStage::PostSimd1,
        MeterTap::PostDynamic => TrackStage::PostDynamic,
        MeterTap::PostSimd2PreFader => TrackStage::PostSimd2PreFader,
        MeterTap::PostFader => TrackStage::PostFader,
        MeterTap::PostMatrix => TrackStage::PostMatrix,
    }
}
fn stage_node(track_id: StableGraphId, stage: TrackStage) -> GraphNodeId {
    GraphNodeId::TrackStage { track_id, stage }
}
fn diag(code: &'static str, path: &str) -> BuiltinDiagnostic {
    BuiltinDiagnostic {
        code,
        path: path.to_owned(),
    }
}
fn meter_path(request: &MeterRequest) -> String {
    format!(
        "$.meters[track_id={},tap={:?}]",
        request.track_id, request.tap
    )
}
fn parameter_diagnostic(
    track: &Track,
    error: BuiltinParameterError,
    sample_rate: u32,
) -> BuiltinDiagnostic {
    let code = match error {
        BuiltinParameterError::GainDomain => "builtin.gain.domain",
        BuiltinParameterError::FilterCutoff => "builtin.filter.cutoff",
        BuiltinParameterError::FilterOrder => "builtin.filter.order",
        BuiltinParameterError::FilterCoefficients => "builtin.filter.coefficients",
        BuiltinParameterError::MatrixCoefficient => "builtin.matrix.coefficient",
        BuiltinParameterError::MatrixSmoothing => "builtin.matrix.smoothing",
        _ => "builtin.resource.arithmetic_overflow",
    };
    let track_path = format!("$.tracks[id={}]", track.id);
    let path = match error {
        BuiltinParameterError::GainDomain => gain_path(track, &track_path),
        BuiltinParameterError::FilterCutoff => cutoff_path(track, &track_path, sample_rate),
        BuiltinParameterError::FilterOrder => filter_order_path(track, &track_path),
        BuiltinParameterError::MatrixCoefficient => matrix_path(track, &track_path),
        BuiltinParameterError::MatrixSmoothing => {
            format!("{track_path}.matrix_or_pan.smoothing_samples")
        }
        _ => format!("{track_path}.builtins"),
    };
    diag(code, &path)
}

fn gain_path(track: &Track, track_path: &str) -> String {
    for (lane, builtins, fader) in [
        ("left", &track.builtins.left, track.fader.left_db),
        ("right", &track.builtins.right, track.fader.right_db),
    ] {
        if !builtins.trim_db.is_finite() || !(-144.0..=24.0).contains(&builtins.trim_db) {
            return format!("{track_path}.builtins.{lane}.trim_db");
        }
        if !fader.is_finite() || !(-144.0..=24.0).contains(&fader) {
            return format!("{track_path}.fader.{lane}_db");
        }
    }
    format!("{track_path}.builtins")
}

fn cutoff_path(track: &Track, track_path: &str, sample_rate: u32) -> String {
    for (lane, builtins) in [
        ("left", &track.builtins.left),
        ("right", &track.builtins.right),
    ] {
        if invalid_cutoff(builtins.hpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.hpf_hz");
        }
        if invalid_cutoff(builtins.lpf_hz, sample_rate) {
            return format!("{track_path}.builtins.{lane}.lpf_hz");
        }
    }
    format!("{track_path}.builtins")
}

fn filter_order_path(track: &Track, track_path: &str) -> String {
    if track.builtins.left.hpf_hz > 0.0
        && track.builtins.left.lpf_hz > 0.0
        && track.builtins.left.hpf_hz >= track.builtins.left.lpf_hz
    {
        format!("{track_path}.builtins.left.lpf_hz")
    } else {
        format!("{track_path}.builtins.right.lpf_hz")
    }
}

fn invalid_cutoff(value: f32, sample_rate: u32) -> bool {
    validate_builtin_filter_cutoff_v1(value, sample_rate, 0.0, 10.0).is_err()
}

fn matrix_path(track: &Track, track_path: &str) -> String {
    match track.matrix_or_pan {
        MatrixOrPan::Pan { left, .. } if !left.is_finite() || !(-1.0..=1.0).contains(&left) => {
            format!("{track_path}.matrix_or_pan.left")
        }
        MatrixOrPan::Pan { .. } => format!("{track_path}.matrix_or_pan.right"),
        MatrixOrPan::Matrix { ll, lr, rl, rr, .. } => {
            for (field, value) in [("ll", ll), ("lr", lr), ("rl", rl), ("rr", rr)] {
                if !value.is_finite() || !(-1.0..=1.0).contains(&value) {
                    return format!("{track_path}.matrix_or_pan.{field}");
                }
            }
            format!("{track_path}.matrix_or_pan")
        }
    }
}
fn meter_diagnostic(request: &MeterRequest, error: MeterConfigError) -> BuiltinDiagnostic {
    diag(
        match error {
            MeterConfigError::DecayDomain => "builtin.meter.config",
            MeterConfigError::Queue => "builtin.resource.arithmetic_overflow",
        },
        &meter_path(request),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::num::{NonZeroU32, NonZeroU64, NonZeroUsize};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use miso_engine_core::{QuantumFrames, SampleRateHz, TargetCapabilities};
    use miso_engine_graph::{
        GraphEdge, GraphEdgeId, GraphNode, GraphNodeBinding, GraphPortId, GraphPortKind,
        GraphPreparedSourceSetDriver, GraphResourceEstimate, GraphSourceInputClaim,
        GraphSourceSetResourceReport, PreparedGraphPlanParts,
    };

    /// The compiler always emits `spec.nodes` sorted by id; hand-built fixtures list them in
    /// reading order, so they sort here (`program::lower` interns ids by binary search).
    fn sorted_nodes(mut nodes: Vec<GraphNode>) -> Vec<GraphNode> {
        nodes.sort_by(|left, right| left.id.cmp(&right.id));
        nodes
    }
    use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};
    use std::sync::Arc;

    fn session() -> CompiledSession {
        let document = include_str!("../../../fixtures/session/v1/canonical.toml");
        compile_session(
            &parse_session_toml(document).expect("parse"),
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile")
    }
    fn caps() -> BuiltinCompileCaps {
        BuiltinCompileCaps {
            maximum_total_state_bytes: u64::MAX,
            maximum_total_retained_payload_bytes: u64::MAX,
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        }
    }

    struct SourceSetDriver {
        claim_count: usize,
        marker: u64,
        drops: Arc<AtomicUsize>,
    }

    impl Drop for SourceSetDriver {
        fn drop(&mut self) {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphPreparedSourceSetDriver for SourceSetDriver {
        fn claim_count(&self) -> usize {
            self.claim_count
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

        fn copy_after_disarm_telemetry(&self, output: &mut [u64]) -> usize {
            if let Some(first) = output.first_mut() {
                *first = self.marker;
                1
            } else {
                0
            }
        }
    }

    struct DropProcessor(Arc<AtomicUsize>);

    impl Drop for DropProcessor {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    impl GraphRuntimeProcessor for DropProcessor {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct NoopObserver;

    impl GraphRuntimeObserver for NoopObserver {
        fn observe(&mut self, _block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    struct SourceBindFixture {
        artifact: PreparedBuiltinsGraphArtifact<u64>,
        bindings: GraphRuntimeBindings,
        source_set: GraphPreparedSourceSet,
        input: GraphNodeId,
        builtin: GraphNodeId,
        output: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    struct SourceBindOwnership {
        input: GraphNodeId,
        builtin_drops: Arc<AtomicUsize>,
        external_drops: Arc<AtomicUsize>,
        source_drops: Arc<AtomicUsize>,
    }

    impl SourceBindFixture {
        fn ownership(&self) -> SourceBindOwnership {
            SourceBindOwnership {
                input: self.input.clone(),
                builtin_drops: Arc::clone(&self.builtin_drops),
                external_drops: Arc::clone(&self.external_drops),
                source_drops: Arc::clone(&self.source_drops),
            }
        }
    }

    fn zero_graph_estimate() -> GraphResourceEstimate {
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

    fn source_bind_fixture() -> SourceBindFixture {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(4),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let track_id = StableGraphId::parse("source-track").expect("test ID");
        let input = GraphNodeId::TrackStage {
            track_id: track_id.clone(),
            stage: TrackStage::Input,
        };
        let builtin = GraphNodeId::TrackStage {
            track_id,
            stage: TrackStage::PostInputBuiltins,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-output").expect("test ID"),
        };
        let edge = |source: GraphNodeId, target: GraphNodeId| GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: target.clone(),
            },
            source: GraphPortId {
                node: source,
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: target,
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.source-bind-test".to_owned(),
        };
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 22,
            spec: miso_engine_graph::GraphSpec {
                nodes: sorted_nodes(
                    [input.clone(), builtin.clone(), output.clone()]
                        .into_iter()
                        .map(|id| GraphNode {
                            id,
                            latency: miso_engine_effect_contract::LatencySamples(0),
                            tail: miso_engine_effect_contract::TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges: vec![
                    edge(input.clone(), builtin.clone()),
                    edge(builtin.clone(), output.clone()),
                ],
            },
            sequential_schedule: vec![input.clone(), builtin.clone(), output.clone()],
            dependency_levels: vec![
                DependencyLevel {
                    level: 0,
                    nodes: vec![input.clone()],
                },
                DependencyLevel {
                    level: 1,
                    nodes: vec![builtin.clone()],
                },
                DependencyLevel {
                    level: 2,
                    nodes: vec![output.clone()],
                },
            ],
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: vec![input.clone(), builtin.clone(), output.clone()],
            routes: Vec::new(),
            effects: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        let builtin_drops = Arc::new(AtomicUsize::new(0));
        let external_drops = Arc::new(AtomicUsize::new(0));
        let source_drops = Arc::new(AtomicUsize::new(0));
        let artifact = PreparedBuiltinsGraphArtifact {
            graph,
            builtin_processors: vec![GraphNodeBinding::new(
                builtin.clone(),
                Box::new(DropProcessor(Arc::clone(&builtin_drops))),
            )],
            builtin_observers: vec![GraphNodeObserverBinding::new(
                builtin.clone(),
                0x22_73,
                Box::new(NoopObserver),
            )],
            report: 0x22_73,
            meter_consumers: Vec::new(),
        };
        let bindings = GraphRuntimeBindings {
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes: vec![GraphNodeBinding::new(
                output.clone(),
                Box::new(DropProcessor(Arc::clone(&external_drops))),
            )],
            observers: Vec::new(),
        };
        let source_set = GraphPreparedSourceSet::new(
            envelope,
            vec![GraphSourceInputClaim {
                node: input.clone(),
            }],
            GraphSourceSetResourceReport {
                pcm_payload_already_charged_bytes: 0,
                overhead_bytes: 0,
                total_engine_owned_bytes: 0,
                largest_allocation_bytes: 0,
            },
            Box::new(SourceSetDriver {
                claim_count: 1,
                marker: 0x22_73,
                drops: Arc::clone(&source_drops),
            }),
        );
        SourceBindFixture {
            artifact,
            bindings,
            source_set,
            input,
            builtin,
            output,
            builtin_drops,
            external_drops,
            source_drops,
        }
    }

    fn assert_source_bind_failure_ownership(
        failure: &PreparedBuiltinsGraphSourceBindFailure<u64>,
        ownership: &SourceBindOwnership,
        expected_binding_nodes: &[GraphNodeId],
    ) {
        assert_eq!(*failure.artifact.report(), 0x22_73);
        assert_eq!(
            failure
                .bindings
                .nodes
                .iter()
                .map(|binding| binding.node.clone())
                .collect::<Vec<_>>(),
            expected_binding_nodes
        );
        assert_eq!(
            failure.source_set.claims(),
            [GraphSourceInputClaim {
                node: ownership.input.clone()
            }]
        );
        let mut telemetry = [0];
        assert_eq!(
            failure
                .source_set
                .copy_after_disarm_telemetry(&mut telemetry),
            1
        );
        assert_eq!(telemetry, [0x22_73]);
        assert_eq!(ownership.builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(ownership.source_drops.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn source_set_bind_succeeds_with_private_builtin_and_external_ownership() {
        let fixture = source_bind_fixture();
        let SourceBindFixture {
            artifact,
            bindings,
            source_set,
            builtin_drops,
            external_drops,
            source_drops,
            ..
        } = fixture;
        let bound = artifact
            .into_bound_with_source_set(bindings, source_set)
            .unwrap_or_else(|failure| panic!("source-set bind rejected: {}", failure.code));
        assert!(bound.meter_consumers.is_empty());
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 0);
        assert_eq!(external_drops.load(Ordering::SeqCst), 0);
        assert_eq!(source_drops.load(Ordering::SeqCst), 0);
        drop(bound);
        assert_eq!(builtin_drops.load(Ordering::SeqCst), 1);
        assert_eq!(external_drops.load(Ordering::SeqCst), 1);
        assert_eq!(source_drops.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn source_set_bind_prevalidation_returns_all_ownership_for_each_code() {
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.envelope.quantum = QuantumFrames(8);
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("envelope mismatch must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.envelope_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.push(GraphNodeBinding::new(
                fixture.output.clone(),
                Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
            ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone(), fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("duplicate external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes[0].node = fixture.builtin.clone();
            let ownership = fixture.ownership();
            let expected = [fixture.builtin.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("external builtin overlap must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.binding");
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture.bindings.nodes.clear();
            fixture.external_drops.store(0, Ordering::SeqCst);
            let ownership = fixture.ownership();
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("missing external binding must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "source.graph.binding_mismatch");
            assert_source_bind_failure_ownership(&failure, &ownership, &[]);
        }
        {
            let mut fixture = source_bind_fixture();
            fixture
                .bindings
                .observers
                .push(GraphNodeObserverBinding::new(
                    fixture.output.clone(),
                    1,
                    Box::new(NoopObserver),
                ));
            let ownership = fixture.ownership();
            let expected = [fixture.output.clone()];
            let failure = match fixture
                .artifact
                .into_bound_with_source_set(fixture.bindings, fixture.source_set)
            {
                Ok(_) => panic!("invalid observer node must reject"),
                Err(failure) => failure,
            };
            assert_eq!(failure.code, "graph.plan.observer");
            assert_eq!(failure.bindings.observers.len(), 1);
            assert_source_bind_failure_ownership(&failure, &ownership, &expected);
        }
    }

    #[test]
    fn delegated_source_rejection_restores_private_and_external_ownership() {
        let mut fixture = source_bind_fixture();
        fixture.bindings.nodes.push(GraphNodeBinding::new(
            fixture.input.clone(),
            Box::new(DropProcessor(Arc::clone(&fixture.external_drops))),
        ));
        let ownership = fixture.ownership();
        let expected = [fixture.output.clone(), fixture.input.clone()];
        let expected_builtin = fixture.builtin.clone();
        let failure = match fixture
            .artifact
            .into_bound_with_source_set(fixture.bindings, fixture.source_set)
        {
            Ok(_) => panic!("source/external overlap must reject in graph bind"),
            Err(failure) => failure,
        };
        assert_eq!(failure.code, "source.graph.binding_mismatch");
        assert_eq!(failure.artifact.builtin_processors.len(), 1);
        assert_eq!(
            failure.artifact.builtin_processors[0].node,
            expected_builtin
        );
        assert_eq!(failure.artifact.builtin_observers.len(), 1);
        assert_eq!(failure.artifact.builtin_observers[0].handle, 0x22_73);
        assert_source_bind_failure_ownership(&failure, &ownership, &expected);
    }

    #[test]
    fn builtin_bank_layout_regroups_by_dependency_wave_and_scalar_falls_back() {
        let inputs: Vec<_> = (0..17)
            .map(|index| {
                (
                    Box::<str>::from(format!("bank{index}")),
                    BuiltinChain::new(48_000, BuiltinParameters::default())
                        .expect("input")
                        .into_input_builtins(),
                )
            })
            .collect();
        let node = |index| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let levels = vec![
            DependencyLevel {
                level: 0,
                nodes: vec![node(0)],
            },
            DependencyLevel {
                level: 1,
                nodes: (1..10).map(node).collect(),
            },
            DependencyLevel {
                level: 2,
                nodes: (10..17).map(node).collect(),
            },
        ];
        // The 1 / 9 / 7 layout, banked per level with the last bank of each level padded:
        // W4 -> 1 | 4 4 1 | 4 3 and W8 -> 1 | 8 1 | 7.  Hand counts of `n.div_ceil(W)` banks per
        // level; every post-input node is a member, so no level contributes a scalar tail.
        for (dispatch, expected_sizes) in [
            (
                KernelDispatch::select(TargetCapabilities::from_detected(
                    true, false, false, false,
                )),
                &[1, 4, 4, 1, 4, 3][..],
            ),
            (
                KernelDispatch::select(TargetCapabilities::from_detected(
                    false, true, false, false,
                )),
                &[1, 4, 4, 1, 4, 3][..],
            ),
            (
                KernelDispatch::select(TargetCapabilities::from_detected(false, false, true, true)),
                &[1, 8, 1, 7][..],
            ),
            // D4: AVX2 without FMA has no bank width at all -- one arithmetic graph everywhere
            // means fusion is written, not inferred, so those tracks stay on the scalar `Lane`.
            (
                KernelDispatch::select(TargetCapabilities::from_detected(
                    false, false, true, false,
                )),
                &[][..],
            ),
        ] {
            let groups = planned_builtin_bank_members(&inputs, dispatch, &levels);
            let sizes: Vec<_> = groups.iter().map(|members| members.len()).collect();
            assert_eq!(sizes, expected_sizes, "{:?}", dispatch.backend());
            assert_eq!(
                sizes.iter().sum::<usize>(),
                if expected_sizes.is_empty() { 0 } else { 17 },
                "every post-input node is banked once"
            );
            let mut group_levels = Vec::new();
            assert!(groups.iter().all(|members| {
                let member_levels: BTreeSet<_> = members
                    .iter()
                    .map(|member| {
                        levels
                            .iter()
                            .find(|level| level.nodes.contains(member))
                            .expect("member level")
                            .level
                    })
                    .collect();
                group_levels.extend(member_levels.iter().copied());
                member_levels.len() == 1 && members.windows(2).all(|pair| pair[0] < pair[1])
            }));
            assert!(
                group_levels.windows(2).all(|pair| pair[0] <= pair[1]),
                "banks are emitted in dependency-level order"
            );
        }
        let scalar = KernelDispatch::select(TargetCapabilities::from_detected(
            false, false, false, false,
        ));
        assert!(planned_builtin_bank_members(&inputs, scalar, &levels).is_empty());
    }
    /// F4/F11: a bank is charged for the two main planes it actually owns and for the member ids
    /// it actually holds -- a padded bank is not charged a full-width id array, and no bank is
    /// charged the two sidechain planes a fixed stage can never reach.
    #[test]
    fn builtin_bank_resource_charges_two_planes_and_actual_members() {
        let node = |index: usize| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&format!("bank{index}")).expect("id"),
            stage: TrackStage::PostInputBuiltins,
        };
        let quantum = 64_u32;
        for (width, sizes) in [
            (miso_engine_effect_contract::BankWidth::Four, &[4, 3][..]),
            (miso_engine_effect_contract::BankWidth::Eight, &[8, 1][..]),
            (miso_engine_effect_contract::BankWidth::Eight, &[5][..]),
        ] {
            let mut next = 0_usize;
            let groups: Vec<Box<[GraphNodeId]>> = sizes
                .iter()
                .map(|size| {
                    (0..*size)
                        .map(|_| {
                            next += 1;
                            node(next - 1)
                        })
                        .collect::<Vec<_>>()
                        .into_boxed_slice()
                })
                .collect();
            let resource = builtin_bank_resource(&groups, width, quantum)
                .expect("padded layout is chargeable");

            // Hand formula, written from `size_of` rather than from the function under test.
            let lanes = u64::from(width.lanes());
            let banks = sizes.len() as u64;
            let node_bytes = core::mem::size_of::<GraphNodeId>() as u64;
            let processor_bytes = core::mem::size_of::<BuiltinBankProcessor>() as u64;
            let plane_bytes = u64::from(quantum) * lanes * 4;
            let string_lengths: Vec<u64> = groups
                .iter()
                .flat_map(|members| members.iter())
                .map(|member| match member {
                    GraphNodeId::TrackStage { track_id, .. } => track_id.as_str().len() as u64,
                    _ => unreachable!("member kind"),
                })
                .collect();
            let strings: u64 = string_lengths.iter().sum();
            let largest_string = string_lengths.iter().copied().max().expect("a member");
            assert_eq!(resource.bank_count, banks);
            assert_eq!(
                resource.scratch_samples,
                banks * u64::from(quantum) * lanes * 2
            );
            assert_eq!(resource.scratch_bytes, banks * plane_bytes * 2);
            assert_eq!(
                resource.payload_bytes,
                sizes
                    .iter()
                    .map(|size| node_bytes * *size as u64 + processor_bytes)
                    .sum::<u64>()
                    + strings
            );
            assert_eq!(
                resource.metadata_bytes,
                banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64
            );
            assert_eq!(
                resource.largest_allocation_bytes,
                [
                    node_bytes * *sizes.iter().max().expect("a bank") as u64,
                    processor_bytes,
                    largest_string,
                    plane_bytes,
                    banks * core::mem::size_of::<GraphPreparedBuiltinBank>() as u64,
                ]
                .into_iter()
                .max()
                .expect("a term")
            );
        }
        // Only an empty or oversized group is unchargeable now.
        assert!(
            builtin_bank_resource(
                &[Vec::new().into_boxed_slice()],
                miso_engine_effect_contract::BankWidth::Four,
                64
            )
            .is_none()
        );
        assert!(
            builtin_bank_resource(
                &[(0..5).map(node).collect::<Vec<_>>().into_boxed_slice()],
                miso_engine_effect_contract::BankWidth::Four,
                64
            )
            .is_none()
        );
    }

    // ---------------------------------------------------------------------------------------
    // #86 A7: the bit-identity harness.
    //
    // These tests render through the production path -- `prepare_session_builtins` ->
    // `into_graph_artifact_with_banks` -> `into_bound` -> `PreparedRenderPlan::render` -- and
    // compare `to_bits()` of the post-input-builtins output, per track, per channel, per block.
    // The oracle is the same generic kernel body at `L = f32` reached through `Scalar` dispatch,
    // which is independent of the vector wrapper, the AoSoA transposes and the padding.
    // ---------------------------------------------------------------------------------------

    /// One session of `n` tracks with deliberately distinct per-track builtins.
    fn n_track_session(n: usize) -> CompiledSession {
        let mut model =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("fixture parse");
        let mut template = model.tracks[0].clone();
        template.simd1.effects.clear();
        template.dynamic.effects.clear();
        template.simd2.effects.clear();
        model.automation.clear();
        model.limits.memory_bytes = u64::MAX;
        model.tracks.clear();
        for index in 0..n {
            let mut track = template.clone();
            track.id = miso_engine_session::StableId::parse(&track_name(index))
                .expect("generated stable ID");
            let scale = index as f32;
            track.builtins.left.hpf_hz = 20.0 + 5.0 * scale;
            track.builtins.left.lpf_hz = 18_000.0 - 100.0 * scale;
            track.builtins.left.trim_db = -3.0 + 0.5 * scale;
            track.builtins.left.polarity_invert = index % 4 == 3;
            track.builtins.right.hpf_hz = if index % 2 == 0 {
                0.0
            } else {
                30.0 + 3.0 * scale
            };
            track.builtins.right.lpf_hz = if index % 3 == 0 {
                0.0
            } else {
                17_000.0 - 250.0 * scale
            };
            track.builtins.right.polarity_invert = index % 2 == 1;
            track.builtins.right.trim_db = 1.0 - 0.25 * scale;
            track.fader.left_db = -1.0 + 0.125 * scale;
            track.fader.right_db = 0.5 - 0.125 * scale;
            model.tracks.push(track);
        }
        model.routes.truncate(1);
        model.routes[0].source = miso_engine_session::RouteSource::Track {
            track_id: miso_engine_session::StableId::parse(&track_name(0)).expect("route track"),
            tap: miso_engine_session::SendTap::PostMatrix,
        };
        compile_session(
            &model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("harness session compile")
    }

    fn track_name(index: usize) -> String {
        format!("t{index:02}")
    }

    const HARNESS_QUANTUM: u32 = 64;
    const HARNESS_BLOCKS: u64 = 3;

    /// A deterministic per-track input signal: an LCG seeded by `(track, first_sample)`.
    ///
    /// It is a plain `GraphRuntimeProcessor` bound at the `Input` stage, so every dispatch under
    /// test sees byte-identical input.
    struct SeededInput {
        seed: u64,
    }

    impl GraphRuntimeProcessor for SeededInput {
        fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            let mut state = self.seed ^ block.first_sample.wrapping_mul(0x9e37_79b9_7f4a_7c15);
            let mut next = || {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                let unit = ((state >> 40) as f32) / ((1_u32 << 24) as f32);
                (unit * 2.0 - 1.0) * 0.8
            };
            for (left, right) in block.left.iter_mut().zip(block.right.iter_mut()) {
                *left = next();
                *right = next();
            }
            Ok(())
        }
    }

    /// Records `to_bits()` of both channels after its node completes.
    struct Capture(Arc<std::sync::Mutex<Vec<u32>>>);

    impl GraphRuntimeObserver for Capture {
        fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
            let mut sink = self.0.lock().expect("harness capture");
            sink.extend(block.left.iter().map(|sample| sample.to_bits()));
            sink.extend(block.right.iter().map(|sample| sample.to_bits()));
            Ok(())
        }
    }

    /// Builds the five-level graph the harness renders: one level per track stage.
    ///
    /// `Output` is fed by track 0's `PostMatrix` only: `GraphEdgeId::TrackMain { target }` is not
    /// unique for fan-in, and a reduction is not what this harness measures.
    fn track_graph(n: usize) -> (PreparedGraphPlan, Vec<DependencyLevel>) {
        let envelope = RenderEnvelope {
            sample_rate: SampleRateHz(48_000),
            quantum: QuantumFrames(HARNESS_QUANTUM),
            input_channels: None,
            output_channels: NonZeroUsize::new(2).expect("two output channels"),
        };
        let stage = |index: usize, stage: TrackStage| GraphNodeId::TrackStage {
            track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
            stage,
        };
        let output = GraphNodeId::Output {
            output_id: StableGraphId::parse("main-out").expect("harness ID"),
        };
        let stages = [
            TrackStage::Input,
            TrackStage::PostInputBuiltins,
            TrackStage::PostFader,
            TrackStage::PostMatrix,
        ];
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for index in 0..n {
            for pair in stages.windows(2) {
                let source = stage(index, pair[0]);
                let target = stage(index, pair[1]);
                edges.push(GraphEdge {
                    id: GraphEdgeId::TrackMain {
                        target: target.clone(),
                    },
                    source: GraphPortId {
                        node: source,
                        kind: GraphPortKind::MainOutput,
                        effect_port: None,
                    },
                    destination: GraphPortId {
                        node: target,
                        kind: GraphPortKind::MainInput,
                        effect_port: None,
                    },
                    path: format!("$.tracks[{index}].chain"),
                });
            }
        }
        edges.push(GraphEdge {
            id: GraphEdgeId::TrackMain {
                target: output.clone(),
            },
            source: GraphPortId {
                node: stage(0, TrackStage::PostMatrix),
                kind: GraphPortKind::MainOutput,
                effect_port: None,
            },
            destination: GraphPortId {
                node: output.clone(),
                kind: GraphPortKind::MainInput,
                effect_port: None,
            },
            path: "$.routes[0]".to_owned(),
        });
        let mut levels = Vec::new();
        for (level, kind) in stages.iter().enumerate() {
            let level_nodes: Vec<_> = (0..n).map(|index| stage(index, *kind)).collect();
            nodes.extend(level_nodes.iter().cloned());
            levels.push(DependencyLevel {
                level: level as u64,
                nodes: level_nodes,
            });
        }
        nodes.push(output.clone());
        levels.push(DependencyLevel {
            level: stages.len() as u64,
            nodes: vec![output.clone()],
        });
        let schedule: Vec<_> = levels
            .iter()
            .flat_map(|level| level.nodes.iter().cloned())
            .collect();
        let graph = PreparedGraphPlan::new(PreparedGraphPlanParts {
            plan_id: 8_600 + n as u64,
            spec: miso_engine_graph::GraphSpec {
                nodes: sorted_nodes(
                    nodes
                        .iter()
                        .cloned()
                        .map(|id| GraphNode {
                            id,
                            latency: miso_engine_effect_contract::LatencySamples(0),
                            tail: miso_engine_effect_contract::TailSamples::Finite(0),
                        })
                        .collect(),
                ),
                ports: Vec::new(),
                edges,
            },
            sequential_schedule: schedule,
            dependency_levels: levels.clone(),
            route_timings: Vec::new(),
            inserted_delays: Vec::new(),
            buffer_assignments: Vec::new(),
            estimate: zero_graph_estimate(),
            envelope,
            required_bindings: nodes,
            routes: Vec::new(),
            effects: Vec::new(),
            banks: Vec::new(),
            builtin_banks: Vec::new(),
            observers: Vec::new(),
        });
        (graph, levels)
    }

    /// Renders `HARNESS_BLOCKS` blocks and returns the post-input-builtins output bits per track.
    fn render_post_input_bits(n: usize, dispatch: KernelDispatch) -> (Vec<Vec<u32>>, usize) {
        let compiled = n_track_session(n);
        let builtins = prepare_session_builtins(&compiled, &[], caps()).expect("harness builtins");
        let (graph, levels) = track_graph(n);
        let mut artifact = builtins.into_graph_artifact_with_banks(graph, (), dispatch, &levels);
        let bank_count = artifact.graph.prepared_builtin_bank_count();
        let captures: Vec<_> = (0..n)
            .map(|_| Arc::new(std::sync::Mutex::new(Vec::new())))
            .collect();
        for (index, capture) in captures.iter().enumerate() {
            artifact
                .builtin_observers
                .push(GraphNodeObserverBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::PostInputBuiltins,
                    },
                    0x8600 + index as u64,
                    Box::new(Capture(Arc::clone(capture))),
                ));
        }
        let envelope = artifact.graph.envelope;
        let mut nodes: Vec<_> = (0..n)
            .map(|index| {
                GraphNodeBinding::new(
                    GraphNodeId::TrackStage {
                        track_id: StableGraphId::parse(&track_name(index)).expect("harness ID"),
                        stage: TrackStage::Input,
                    },
                    Box::new(SeededInput {
                        seed: 0x5eed_0000 ^ index as u64,
                    }) as Box<dyn GraphRuntimeProcessor>,
                )
            })
            .collect();
        nodes.push(GraphNodeBinding::new(
            GraphNodeId::Output {
                output_id: StableGraphId::parse("main-out").expect("harness ID"),
            },
            Box::new(HarnessSink) as Box<dyn GraphRuntimeProcessor>,
        ));
        let mut plan = match artifact.into_bound(GraphRuntimeBindings {
            #[cfg(not(target_arch = "wasm32"))]
            worker_lease: None,
            envelope,
            nodes,
            observers: Vec::new(),
        }) {
            Ok(bound) => bound.plan,
            Err(failure) => panic!("harness bind: {}", failure.code),
        };
        let frames = HARNESS_QUANTUM as usize;
        let mut pcm = vec![0.0_f32; frames * 2];
        for block in 0..HARNESS_BLOCKS {
            plan.render(
                miso_engine_core::realtime::RenderIo {
                    input: None,
                    output: miso_engine_core::realtime::PlanarBufferMut::try_new(
                        &mut pcm, 2, frames, frames,
                    )
                    .expect("harness output"),
                },
                miso_engine_core::realtime::RenderTime {
                    absolute_sample: block * HARNESS_QUANTUM as u64,
                },
            )
            .expect("harness render");
        }
        let bits = captures
            .into_iter()
            .map(|capture| {
                let taken = capture.lock().expect("harness capture").clone();
                assert_eq!(taken.len(), frames * 2 * HARNESS_BLOCKS as usize);
                taken
            })
            .collect();
        (bits, bank_count)
    }

    struct HarnessSink;

    impl GraphRuntimeProcessor for HarnessSink {
        fn process(&mut self, _block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
            Ok(())
        }
    }

    fn host_dispatch() -> KernelDispatch {
        KernelDispatch::select(miso_engine_core::target_capabilities())
    }

    fn scalar_dispatch() -> KernelDispatch {
        KernelDispatch::select(TargetCapabilities::from_detected(
            false, false, false, false,
        ))
    }

    /// E2 (#86 F2 proof, F3): a track's bits do not depend on whether it renders in a bank.
    ///
    /// `n = 1..=9` covers the short first bank, the exactly-full bank and the padded second bank
    /// at both widths. The oracle is the `Scalar` dispatch, which banks nothing at all.
    #[test]
    fn banked_tracks_are_bit_identical_to_their_scalar_tails() {
        let host = host_dispatch();
        for n in 1..=9 {
            let (banked, bank_count) = render_post_input_bits(n, host);
            let (scalar, scalar_banks) = render_post_input_bits(n, scalar_dispatch());
            assert_eq!(scalar_banks, 0, "the scalar oracle banks nothing");
            match host.bank_width() {
                Some(width) => assert_eq!(
                    bank_count,
                    n.div_ceil(width.lanes() as usize),
                    "padded bank count for {n} tracks"
                ),
                None => assert_eq!(bank_count, 0),
            }
            // Distinctness guard: a harness whose tracks all render silence, or render the same
            // thing, would pass every identity assertion below without testing anything.
            assert!(
                banked
                    .iter()
                    .all(|bits| bits.iter().any(|bits| *bits != 0 && *bits != 0x8000_0000)),
                "every track must carry signal"
            );
            let distinct: BTreeSet<_> = banked.iter().collect();
            assert_eq!(distinct.len(), n, "every track must render differently");
            for (track, (banked, scalar)) in banked.iter().zip(&scalar).enumerate() {
                assert_eq!(
                    banked, scalar,
                    "track {track} of {n} differs between the bank and the scalar tail"
                );
            }
        }
    }

    /// E3 (#86 F2 partition invariance): adding a track never moves an existing track's bits.
    ///
    /// 7, 8 and 9 tracks straddle the W8 bank boundary in both directions, and 3/4/5 straddle W4.
    #[test]
    fn track_bits_do_not_depend_on_session_track_count() {
        let host = host_dispatch();
        let renders: BTreeMap<usize, Vec<Vec<u32>>> = [3, 4, 5, 7, 8, 9]
            .into_iter()
            .map(|n| (n, render_post_input_bits(n, host).0))
            .collect();
        let reference = render_post_input_bits(7, scalar_dispatch()).0;
        for (n, bits) in &renders {
            for (track, (bits, reference)) in bits.iter().zip(&reference).enumerate() {
                assert_eq!(
                    bits, reference,
                    "track {track} moved between a {n}-track session and the 7-track scalar oracle"
                );
            }
        }
        for smaller in [3, 4, 5, 7, 8] {
            for larger in [4, 5, 7, 8, 9] {
                if larger <= smaller {
                    continue;
                }
                for (track, (small, large)) in
                    renders[&smaller].iter().zip(&renders[&larger]).enumerate()
                {
                    assert_eq!(
                        small, large,
                        "track {track} moved from a {smaller}-track to a {larger}-track session"
                    );
                }
            }
        }
    }

    fn handle(value: u64) -> MeterHandle {
        MeterHandle(NonZeroU64::new(value).expect("nonzero test meter handle"))
    }
    #[test]
    fn prepares_three_sections_and_each_named_meter_tap() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 8,
            peak_decay_db_per_second: 12.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 3,
        };
        let requests: Vec<_> = [
            MeterTap::Input,
            MeterTap::PostInputBuiltins,
            MeterTap::PostSimd1,
            MeterTap::PostDynamic,
            MeterTap::PostSimd2PreFader,
            MeterTap::PostFader,
            MeterTap::PostMatrix,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, tap)| MeterRequest {
            handle: handle(u64::try_from(index).expect("bounded") + 1),
            track_id: "vocal".to_owned(),
            tap,
            config,
        })
        .collect();
        let prepared = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        assert_eq!(prepared.processors.len(), 3);
        assert_eq!(prepared.observers.len(), 7);
        assert_eq!(prepared.meter_consumers.len(), 7);
        assert_eq!(prepared.resources.meter_items, 35);
        assert!(prepared.resources.engine_owned_processor_payload_bytes > 0);
        assert!(
            prepared.resources.engine_owned_meter_payload_bytes
                > 35 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert!(
            prepared.resources.maximum_single_allocation_bytes
                >= 5 * core::mem::size_of::<MeterSnapshot>() as u64
        );
        assert_eq!(
            prepared.tails().collect::<Vec<_>>(),
            vec![("vocal", BuiltinTail::Infinite)]
        );
    }
    #[test]
    fn rejects_duplicate_and_unknown_meter_transactionally() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let result = prepare_session_builtins(
            &session(),
            &[
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
                MeterRequest {
                    handle: handle(1),
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
            ],
            caps(),
        );
        let Err(error) = result else {
            panic!("must reject");
        };
        assert_eq!(
            error.0.iter().map(|item| item.code).collect::<Vec<_>>(),
            vec![
                "builtin.meter.duplicate",
                "builtin.meter.duplicate_handle",
                "builtin.meter.unknown_track"
            ]
        );
    }

    #[test]
    fn resource_estimate_enforces_the_actual_largest_retained_payload() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let requests = [MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        }];
        let baseline = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        let mut constrained = caps();
        constrained.maximum_single_allocation_bytes = baseline
            .resources
            .maximum_single_allocation_bytes
            .saturating_sub(1);
        let Err(error) = prepare_session_builtins(&session(), &requests, constrained) else {
            panic!("largest retained payload must be capped");
        };
        assert_eq!(
            error.0,
            vec![diag("builtin.resource.limit", "$.builtin_compile_caps")]
        );
    }

    #[test]
    fn retained_payload_boundaries_reject_in_phase_one() {
        let config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(1).expect("constant"),
            reset_generation: 0,
        };
        let requests = [MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::PostMatrix,
            config,
        }];
        let baseline = prepare_session_builtins(&session(), &requests, caps()).expect("prepare");
        let report = baseline.resource_report();
        let mut state_limited = caps();
        state_limited.maximum_total_state_bytes = report
            .engine_owned_processor_payload_bytes
            .checked_sub(1)
            .expect("nonzero processor payload");
        let mut meter_limited = caps();
        meter_limited.maximum_total_meter_bytes = report
            .engine_owned_meter_payload_bytes
            .checked_sub(1)
            .expect("nonzero meter payload");
        for limited in [state_limited, meter_limited] {
            let Err(error) = prepare_session_builtins(&session(), &requests, limited) else {
                panic!("one byte below a retained-payload boundary must reject");
            };
            assert_eq!(
                error.0,
                vec![diag("builtin.resource.limit", "$.builtin_compile_caps")]
            );
        }
    }

    #[test]
    fn cutoff_boundaries_match_compiler_diagnostics_at_every_launch_rate_and_section() {
        let document = include_str!("../../../fixtures/session/v1/canonical.toml");
        let base_model = parse_session_toml(document).expect("parse boundary session");
        for (rate, maximum_bits) in [
            (44_100, 0x46ac_42f7),
            (48_000, 0x46bb_7ede),
            (88_200, 0x472c_42f7),
            (96_000, 0x473b_7ede),
        ] {
            for (high_pass, path) in [
                (true, "$.tracks[id=vocal].builtins.left.hpf_hz"),
                (false, "$.tracks[id=vocal].builtins.left.lpf_hz"),
            ] {
                let prepare = |cutoff: f32| {
                    let mut model = base_model.clone();
                    model.sample_rate_hz = rate;
                    model.sources[0].sample_rate_hz = rate;
                    for track in &mut model.tracks {
                        track.builtins.left.hpf_hz = 0.0;
                        track.builtins.left.lpf_hz = 0.0;
                        track.builtins.right.hpf_hz = 0.0;
                        track.builtins.right.lpf_hz = 0.0;
                    }
                    if high_pass {
                        model.tracks[0].builtins.left.hpf_hz = cutoff;
                    } else {
                        model.tracks[0].builtins.left.lpf_hz = cutoff;
                    }
                    let compiled = compile_session(
                        &model,
                        CompileCaps {
                            max_compiled_model_bytes: u64::MAX,
                            max_requested_runtime_bytes: u64::MAX,
                            max_single_allocation_bytes: u64::MAX,
                            max_queue_items: u64::MAX,
                            max_source_ring_frames: u64::MAX,
                            max_source_ring_bytes: u64::MAX,
                        },
                    )
                    .expect("launch-rate boundary session compiles");
                    prepare_session_builtins(&compiled, &[], caps())
                };
                prepare(f32::from_bits(maximum_bits)).unwrap_or_else(|error| {
                    panic!(
                        "maximum must prepare: rate={rate}, high_pass={high_pass}, error={error:?}"
                    )
                });
                let Err(successor_error) = prepare(f32::from_bits(maximum_bits + 1)) else {
                    panic!("the immediate successor must reject before coefficient preparation");
                };
                assert_eq!(
                    successor_error,
                    BuiltinDiagnosticSet(vec![diag("builtin.filter.cutoff", path)]),
                    "rate={rate}, high_pass={high_pass}"
                );
            }
        }
    }

    /// Frozen issue-034 compiler-mutation seed. This exercises complete preparation requests and
    /// their prepared block/target contract, never a timed workload.
    const BUILTIN_COMPILER_MUTATION_SEED: u64 = 0x34_007_c10_u64;
    const BUILTIN_COMPILER_MUTATION_CLASSES: usize = 49;

    #[test]
    fn deterministic_builtin_compiler_mutation_matrix_has_exactly_ten_thousand_cases() {
        let mut base_model =
            parse_session_toml(include_str!("../../../fixtures/session/v1/canonical.toml"))
                .expect("parse baseline mutation session");
        base_model.tracks[0].dynamic.effects.clear();
        base_model.automation.clear();
        base_model.limits.memory_bytes = u64::MAX;
        let base_config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 0,
        };
        let baseline_request = MeterRequest {
            handle: handle(1),
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config: base_config,
        };
        let accepted = compile_session(
            &base_model,
            CompileCaps {
                max_compiled_model_bytes: u64::MAX,
                max_requested_runtime_bytes: u64::MAX,
                max_single_allocation_bytes: u64::MAX,
                max_queue_items: u64::MAX,
                max_source_ring_frames: u64::MAX,
                max_source_ring_bytes: u64::MAX,
            },
        )
        .expect("compile baseline mutation session");
        let baseline =
            prepare_session_builtins(&accepted, std::slice::from_ref(&baseline_request), caps())
                .expect("baseline preparation");
        let report = baseline.resource_report();
        let mut state = BUILTIN_COMPILER_MUTATION_SEED;
        let mut transcript_hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut seen_taps = BTreeSet::new();
        let mut seen_rates = BTreeSet::new();
        let mut seen_quanta = BTreeSet::new();
        let mut seen_smoothing = BTreeSet::new();
        let mut classes = [false; BUILTIN_COMPILER_MUTATION_CLASSES];
        let mut completed = 0_u32;
        for case in 0_u32..10_000 {
            // xorshift64* is intentionally local and fixed; case descriptions are mixed into a
            // transcript hash so accidental coverage drift is visible in this test.
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let value = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            let class =
                usize::try_from(case).expect("u32 fits usize") % BUILTIN_COMPILER_MUTATION_CLASSES;
            classes[class] = true;
            let taps = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ];
            let tap = taps[usize::try_from(case % 7).expect("bounded tap index")];
            seen_taps.insert(tap);
            let rate = [44_100, 48_000, 88_200, 96_000]
                [usize::try_from(case % 4).expect("bounded rate index")];
            let quantum = [1, 127, 128, 255, 1_024]
                [usize::try_from(case % 5).expect("bounded quantum index")];
            let smoothing = [0, 1, 2, 127, 128, u32::MAX]
                [usize::try_from(case % 6).expect("bounded smoothing index")];
            seen_rates.insert(rate);
            seen_quanta.insert(quantum);
            seen_smoothing.insert(smoothing);

            if class == 0 {
                let invalid = include_str!("../../../fixtures/session/v1/canonical.toml").replacen(
                    "polarity_invert = false",
                    "polarity_invert = 0.5",
                    1,
                );
                let diagnostics = parse_session_toml(&invalid)
                    .expect_err("numeric boolean encoding must reject before preparation");
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(
                    observed,
                    vec![(
                        "schema.wrong_type",
                        "$.tracks[0].builtins.left.polarity_invert".to_owned()
                    )]
                );
                for byte in
                    format!("case={case};class={class};invalid_boolean={observed:?};seed={value}")
                        .bytes()
                {
                    transcript_hash ^= u64::from(byte);
                    transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
                }
                completed = completed.checked_add(1).expect("fixed case count");
                continue;
            }

            let mut model = base_model.clone();
            model.sample_rate_hz = rate;
            model.sources[0].sample_rate_hz = rate;
            model.quantum_frames = quantum;
            model.tracks[0].matrix_or_pan = MatrixOrPan::Matrix {
                ll: 1.0,
                lr: 0.0,
                rl: 0.0,
                rr: 1.0,
                smoothing_samples: smoothing,
            };
            let mut requests = vec![MeterRequest {
                handle: handle(1),
                track_id: "vocal".to_owned(),
                tap,
                config: base_config,
            }];
            let mut mutation_caps = caps();
            let mut expected = Vec::new();
            let mut expected_session = Vec::new();
            let mut target = Matrix2x2::IDENTITY;
            let mut block_probe = 0_u8;
            match class {
                1 => model.tracks[0].builtins.left.polarity_invert = value & 1 != 0,
                2 => model.tracks[0].builtins.left.trim_db = -144.0,
                3 => model.tracks[0].builtins.right.trim_db = 24.0,
                4 => {
                    model.tracks[0].builtins.left.trim_db = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.left.trim_db".to_owned(),
                    ));
                }
                5 => {
                    model.tracks[0].fader.right_db = 24.001;
                    expected.push(diag(
                        "builtin.gain.domain",
                        "$.tracks[id=vocal].fader.right_db",
                    ));
                }
                6 => model.tracks[0].builtins.left.hpf_hz = 0.0,
                7 => model.tracks[0].builtins.left.hpf_hz = 10.0,
                8 => {
                    model.tracks[0].builtins.left.hpf_hz = rate as f32 / 2.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    expected.push(diag(
                        "builtin.filter.cutoff",
                        "$.tracks[id=vocal].builtins.left.hpf_hz",
                    ));
                }
                9 => {
                    model.tracks[0].builtins.left.hpf_hz = 1_000.0;
                    model.tracks[0].builtins.left.lpf_hz = 100.0;
                    expected.push(diag(
                        "builtin.filter.order",
                        "$.tracks[id=vocal].builtins.left.lpf_hz",
                    ));
                }
                10 => {
                    if let MatrixOrPan::Matrix { ll, .. } = &mut model.tracks[0].matrix_or_pan {
                        *ll = -1.0;
                    }
                }
                11 => {
                    if let MatrixOrPan::Matrix { rr, .. } = &mut model.tracks[0].matrix_or_pan {
                        *rr = 1.001;
                    }
                    expected.push(diag(
                        "builtin.matrix.coefficient",
                        "$.tracks[id=vocal].matrix_or_pan.rr",
                    ));
                }
                12 => {
                    requests.push(MeterRequest {
                        handle: handle(1),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    expected.push(diag(
                        "builtin.meter.duplicate_handle",
                        &meter_path(&requests[1]),
                    ));
                }
                13 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        ..requests[0].clone()
                    });
                    expected.push(diag("builtin.meter.duplicate", &meter_path(&requests[1])));
                }
                14 => {
                    requests[0].track_id = "unknown-track".to_owned();
                    expected.push(diag(
                        "builtin.meter.unknown_track",
                        &meter_path(&requests[0]),
                    ));
                }
                15 => requests[0].config.period_frames = NonZeroU32::new(1).expect("constant"),
                16 => {
                    requests[0].config.period_frames = NonZeroU32::new(u32::MAX).expect("constant")
                }
                17 => {
                    requests[0].config.period_frames = NonZeroU32::new(128).expect("constant");
                    mutation_caps.maximum_period_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                18 => requests[0].config.peak_hold_frames = u32::MAX,
                19 => {
                    requests[0].config.peak_hold_frames = 128;
                    mutation_caps.maximum_peak_hold_frames = 127;
                    expected.push(diag("builtin.resource.limit", &meter_path(&requests[0])));
                }
                20 => requests[0].config.peak_decay_db_per_second = 120.0,
                21 => {
                    requests[0].config.peak_decay_db_per_second = f32::NAN;
                    expected.push(diag("builtin.meter.config", &meter_path(&requests[0])));
                }
                22 => requests[0].config.reset_generation = u64::MAX,
                23 => {
                    requests[0].config.queue_capacity =
                        NonZeroUsize::new(usize::MAX).expect("constant");
                    expected.push(diag(
                        "builtin.resource.arithmetic_overflow",
                        &meter_path(&requests[0]),
                    ));
                }
                24 => {
                    target = Matrix2x2 {
                        ll: -1.0,
                        lr: 1.0,
                        rl: 1.0,
                        rr: -1.0,
                    }
                }
                25 => target.ll = f32::NAN,
                26 => target.lr = f32::INFINITY,
                27 => target.rr = 1.001,
                28 => block_probe = 1,
                29 => block_probe = 2,
                30 => block_probe = 3,
                31 => block_probe = 4,
                32 => {
                    mutation_caps.maximum_total_state_bytes =
                        report.engine_owned_processor_payload_bytes
                }
                33 => {
                    mutation_caps.maximum_total_state_bytes = report
                        .engine_owned_processor_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                34 => {
                    mutation_caps.maximum_total_retained_payload_bytes =
                        report.engine_owned_retained_payload_bytes
                }
                35 => {
                    mutation_caps.maximum_total_retained_payload_bytes = report
                        .engine_owned_retained_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                36 => {
                    mutation_caps.maximum_total_meter_bytes =
                        report.engine_owned_meter_payload_bytes
                }
                37 => {
                    mutation_caps.maximum_total_meter_bytes = report
                        .engine_owned_meter_payload_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                38 => {
                    mutation_caps.maximum_single_allocation_bytes =
                        report.maximum_single_allocation_bytes
                }
                39 => {
                    mutation_caps.maximum_single_allocation_bytes = report
                        .maximum_single_allocation_bytes
                        .checked_sub(1)
                        .expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                40 => mutation_caps.maximum_total_meter_items = report.meter_items,
                41 => {
                    mutation_caps.maximum_total_meter_items =
                        report.meter_items.checked_sub(1).expect("nonzero");
                    expected.push(diag("builtin.resource.limit", "$.builtin_compile_caps"));
                }
                42 => mutation_caps.maximum_meter_streams = 1,
                43 => {
                    requests.push(MeterRequest {
                        handle: handle(2),
                        track_id: "vocal".to_owned(),
                        tap: if tap == MeterTap::Input {
                            MeterTap::PostMatrix
                        } else {
                            MeterTap::Input
                        },
                        config: base_config,
                    });
                    mutation_caps.maximum_meter_streams = 1;
                    expected.push(diag("builtin.resource.limit", "$.meter_requests"));
                }
                44 => {
                    let limited = if smoothing == 0 { 0 } else { smoothing - 1 };
                    mutation_caps.maximum_smoothing_samples = limited;
                    if smoothing > 0 {
                        expected.push(diag(
                            "builtin.matrix.smoothing",
                            "$.tracks[id=vocal].matrix_or_pan.smoothing_samples",
                        ));
                    }
                }
                45 => {
                    let coefficient = (u32::try_from(value >> 32).expect("masked") as f32
                        / u32::MAX as f32)
                        * 2.0
                        - 1.0;
                    if let MatrixOrPan::Matrix { lr, rl, .. } = &mut model.tracks[0].matrix_or_pan {
                        *lr = coefficient;
                        *rl = -coefficient;
                    }
                }
                46 => {
                    let maximum = builtin_filter_cutoff_maximum_hz_v1(rate)
                        .expect("matrix only uses launch rates");
                    let successor = f32::from_bits(maximum.to_bits() + 1);
                    model.tracks[0].builtins.left.hpf_hz = 0.0;
                    model.tracks[0].builtins.left.lpf_hz = 0.0;
                    match case % 4 {
                        0 => model.tracks[0].builtins.left.hpf_hz = maximum,
                        1 => model.tracks[0].builtins.left.lpf_hz = maximum,
                        2 => {
                            model.tracks[0].builtins.left.hpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.hpf_hz",
                            ));
                        }
                        3 => {
                            model.tracks[0].builtins.left.lpf_hz = successor;
                            expected.push(diag(
                                "builtin.filter.cutoff",
                                "$.tracks[id=vocal].builtins.left.lpf_hz",
                            ));
                        }
                        _ => unreachable!("case remainder is bounded"),
                    }
                }
                47 => target.rl = f32::NEG_INFINITY,
                48 => {
                    model.tracks[0].builtins.right.lpf_hz = f32::NAN;
                    expected_session.push((
                        "numeric.non_finite",
                        "$.tracks[0].builtins.right.lpf_hz".to_owned(),
                    ));
                }
                _ => unreachable!("frozen class range"),
            }

            let compiled_result = compile_session(
                &model,
                CompileCaps {
                    max_compiled_model_bytes: u64::MAX,
                    max_requested_runtime_bytes: u64::MAX,
                    max_single_allocation_bytes: u64::MAX,
                    max_queue_items: u64::MAX,
                    max_source_ring_frames: u64::MAX,
                    max_source_ring_bytes: u64::MAX,
                },
            );
            if !expected_session.is_empty() {
                let Err(diagnostics) = compiled_result else {
                    panic!("invalid complete session must reject: case={case}, class={class}");
                };
                let observed: Vec<_> = diagnostics
                    .diagnostics()
                    .iter()
                    .map(|diagnostic| (diagnostic.code.as_str(), diagnostic.path.to_string()))
                    .collect();
                assert_eq!(observed, expected_session, "case={case}, class={class}");
                for byte in format!(
                    "case={case};class={class};seed={value};rate={rate};quantum={quantum};tap={tap:?};smoothing={smoothing};session={observed:?}"
                )
                .bytes()
                {
                    transcript_hash ^= u64::from(byte);
                    transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
                }
                completed = completed.checked_add(1).expect("fixed case count");
                continue;
            }
            let compiled = compiled_result
                .expect("complete generated session compiles before builtin preparation");
            let expected = BuiltinDiagnosticSet::sorted(expected);
            let result = prepare_session_builtins(&compiled, &requests, mutation_caps);
            if !expected.0.is_empty() {
                let Err(observed) = result else {
                    panic!("frozen invalid class must reject: case={case}, class={class}");
                };
                assert_eq!(observed, expected, "case={case}, class={class}");
            } else {
                let prepared = match result {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        panic!(
                            "frozen valid preparation class failed: case={case}, class={class}, error={error:?}"
                        )
                    }
                };
                let accepted_report = prepared.resource_report();
                assert!(
                    accepted_report.engine_owned_processor_payload_bytes
                        <= mutation_caps.maximum_total_state_bytes
                );
                assert!(
                    accepted_report.engine_owned_retained_payload_bytes
                        <= mutation_caps.maximum_total_retained_payload_bytes
                );
                assert!(
                    accepted_report.engine_owned_meter_payload_bytes
                        <= mutation_caps.maximum_total_meter_bytes
                );
                assert!(
                    accepted_report.maximum_single_allocation_bytes
                        <= mutation_caps.maximum_single_allocation_bytes
                );
                assert_eq!(
                    accepted_report
                        .retained_layouts()
                        .iter()
                        .try_fold(0_u64, |total, layout| {
                            total.checked_add(
                                layout.size_bytes.checked_mul(layout.allocation_count)?,
                            )
                        }),
                    Some(accepted_report.engine_owned_retained_payload_bytes)
                );

                let parameters = track_parameters(&model.tracks[0], u32::MAX)
                    .expect("accepted class parameters");
                let mut chain = BuiltinChain::new(rate, parameters).expect("accepted chain");
                let target_result = chain.set_matrix_target(target);
                if matches!(class, 25 | 26 | 27 | 47) {
                    assert_eq!(target_result, Err(BuiltinParameterError::MatrixCoefficient));
                } else {
                    target_result.expect("valid target");
                }
                let frames = usize::try_from(quantum).expect("supported quantum fits usize");
                match block_probe {
                    1 => {
                        let mut left = Vec::<f32>::new();
                        let mut right = Vec::<f32>::new();
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::EmptyBlock)
                        ));
                    }
                    2 => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames.checked_sub(1).expect("nonzero")];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, 0),
                            Err(BuiltinParameterError::LaneLength)
                        ));
                    }
                    3 => {
                        let mut left = [0.0_f32];
                        let mut right = [0.0_f32];
                        assert!(matches!(
                            DualMonoBlock::new(&mut left, &mut right, u64::MAX),
                            Err(BuiltinParameterError::SampleTimeOverflow)
                        ));
                    }
                    4 => {
                        let PreparedMeter {
                            mut accumulator, ..
                        } = MeterAccumulator::prepare(handle(99), base_config, rate)
                            .expect("valid discontinuity meter");
                        accumulator.observe(&[0.0], &[0.0], 0).expect("first block");
                        accumulator
                            .observe(&[0.0], &[0.0], 100)
                            .expect("discontinuous block is bounded and accepted");
                    }
                    _ => {
                        let mut left = vec![0.0_f32; frames];
                        let mut right = vec![0.0_f32; frames];
                        let block = DualMonoBlock::new(&mut left, &mut right, u64::from(case))
                            .expect("valid generated block");
                        chain.process_dual_mono(block);
                    }
                }
            }

            let description = format!(
                "case={case};class={class};seed={value};rate={rate};quantum={quantum};tap={tap:?};smoothing={smoothing};handle={};period={};hold={};decay={:08x};reset={};queue={};caps={mutation_caps:?};expected={:?};target={:08x},{:08x},{:08x},{:08x};block={block_probe}",
                requests[0].handle.0,
                requests[0].config.period_frames,
                requests[0].config.peak_hold_frames,
                requests[0].config.peak_decay_db_per_second.to_bits(),
                requests[0].config.reset_generation,
                requests[0].config.queue_capacity,
                expected.0,
                target.ll.to_bits(),
                target.lr.to_bits(),
                target.rl.to_bits(),
                target.rr.to_bits(),
            );
            for byte in description.bytes() {
                transcript_hash ^= u64::from(byte);
                transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
            }
            completed = completed.checked_add(1).expect("fixed case count");
        }
        assert_eq!(completed, 10_000);
        assert_eq!(seen_taps.len(), 7);
        assert_eq!(seen_rates, BTreeSet::from([44_100, 48_000, 88_200, 96_000]));
        assert_eq!(seen_quanta, BTreeSet::from([1, 127, 128, 255, 1_024]));
        assert_eq!(
            seen_smoothing,
            BTreeSet::from([0, 1, 2, 127, 128, u32::MAX])
        );
        assert!(classes.into_iter().all(core::convert::identity));
        assert_eq!(
            transcript_hash, 1_237_728_508_441_328_827,
            "updated only through a deliberate frozen-case change"
        );
    }
}
