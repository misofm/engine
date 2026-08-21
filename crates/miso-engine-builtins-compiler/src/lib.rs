//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use std::collections::BTreeSet;

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
    PreparedMeter, pan_matrix, validate_builtin_filter_cutoff_v1,
};
use miso_engine_core::realtime::{
    Consumer, PreparedRenderPlan, RenderEnvelope, RenderError, bounded_spsc_retained_payload,
};
use miso_engine_graph::{
    DependencyLevel, GraphBindingBlock, GraphNodeId, GraphNodeObserverBinding,
    GraphObservationBlock, GraphPreparedBuiltinBank, GraphPreparedBuiltinBankProcessor,
    GraphRuntimeBindings, GraphRuntimeObserver, GraphRuntimeProcessor, PreparedGraphPlan,
    StableGraphId, TrackStage,
};
use miso_engine_rack::{AoSoaScratch, KernelDispatch};
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
    width: miso_engine_effect_contract::BankWidth,
    members: Box<[GraphNodeId]>,
    active: Box<[bool]>,
    processor: BuiltinBankProcessor,
    scratch: AoSoaScratch,
}

struct BuiltinBankProcessor {
    bank: BuiltinInputBankV1,
    process_calls: u64,
    tpt_kernel_calls: u64,
}

impl GraphPreparedBuiltinBankProcessor for BuiltinBankProcessor {
    fn process(
        &mut self,
        left: &mut [f32],
        right: &mut [f32],
        frames: u32,
        first_sample: u64,
    ) -> Result<(), RenderError> {
        self.bank
            .process(left, right, frames, first_sample)
            .map_err(render_error)?;
        self.process_calls = self.process_calls.saturating_add(1);
        self.tpt_kernel_calls = self
            .tpt_kernel_calls
            .saturating_add(u64::from(frames).saturating_mul(2));
        Ok(())
    }
}

impl PreparedBuiltinInputBankV1 {
    fn into_graph_bank(self) -> GraphPreparedBuiltinBank {
        let _ = (self.backend, self.width, self.active);
        GraphPreparedBuiltinBank {
            members: self.members,
            processor: Box::new(self.processor),
            scratch: self.scratch,
        }
    }
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

/// A rejected external binding preserves the opaque artifact and caller-owned bindings.
pub struct PreparedBuiltinsGraphBindFailure<R> {
    pub artifact: PreparedBuiltinsGraphArtifact<R>,
    pub bindings: GraphRuntimeBindings,
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

    /// Materialize full post-input builtin banks using the already-selected host dispatch.
    /// Incomplete groups deliberately retain their original scalar bindings.
    pub fn into_graph_artifact_with_banks<R>(
        mut self,
        graph: PreparedGraphPlan,
        report: R,
        dispatch: KernelDispatch,
        levels: &[DependencyLevel],
    ) -> PreparedBuiltinsGraphArtifact<R> {
        let Some(width) = dispatch.bank_width() else {
            return self.into_graph_artifact(graph, report);
        };
        let level_by_node: std::collections::BTreeMap<_, _> = levels
            .iter()
            .flat_map(|level| {
                level
                    .nodes
                    .iter()
                    .cloned()
                    .map(move |node| (node, level.level))
            })
            .collect();
        self.bank_inputs.sort_by(|left, right| left.0.cmp(&right.0));
        let mut selected = std::collections::BTreeSet::new();
        let mut banks = Vec::new();
        for inputs in self.bank_inputs.chunks_mut(width.lanes() as usize) {
            if inputs.len() != width.lanes() as usize {
                continue;
            }
            let members: Vec<_> = inputs
                .iter()
                .map(|(track, _)| GraphNodeId::TrackStage {
                    track_id: StableGraphId::parse(track).expect("prepared stable ID"),
                    stage: TrackStage::PostInputBuiltins,
                })
                .collect();
            let Some(first_level) = level_by_node.get(&members[0]).copied() else {
                continue;
            };
            if members
                .iter()
                .any(|member| level_by_node.get(member).copied() != Some(first_level))
            {
                continue;
            }
            let active = vec![true; inputs.len()];
            let sample_rate = self.seal.sample_rate;
            let bank_inputs = inputs
                .iter_mut()
                .map(|(_, input)| {
                    core::mem::replace(
                        input,
                        BuiltinChain::new(sample_rate, BuiltinParameters::default())
                            .expect("default builtin parameters")
                            .into_input_builtins(),
                    )
                })
                .collect();
            let bank = BuiltinInputBankV1::new(dispatch.backend(), width, bank_inputs, &active)
                .expect("selected backend and exact bank width are preparation-validated");
            selected.extend(members.iter().cloned());
            banks.push(PreparedBuiltinInputBankV1 {
                backend: dispatch.backend(),
                width,
                members: members.into_boxed_slice(),
                active: active.into_boxed_slice(),
                processor: BuiltinBankProcessor {
                    bank,
                    process_calls: 0,
                    tpt_kernel_calls: 0,
                },
                scratch: AoSoaScratch::new(width, self.seal.quantum)
                    .expect("prepared nonzero graph quantum"),
            });
        }
        if selected.is_empty() {
            return self.into_graph_artifact(graph, report);
        }
        self.processors
            .retain(|binding| !selected.contains(&binding.node));
        let graph = graph
            .with_builtin_banks(
                banks
                    .into_iter()
                    .map(PreparedBuiltinInputBankV1::into_graph_bank)
                    .collect(),
            )
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
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
    }
}
struct FaderProcessor(FaderMuteBuiltins);
impl GraphRuntimeProcessor for FaderProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
    }
}
struct MatrixProcessor(MatrixBuiltins);
impl GraphRuntimeProcessor for MatrixProcessor {
    fn process(&mut self, block: GraphBindingBlock<'_>) -> Result<(), RenderError> {
        self.0
            .process(
                DualMonoBlock::new(block.left, block.right, block.first_sample)
                    .map_err(render_error)?,
            )
            .map(|_| ())
            .map_err(render_error)
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
    use miso_engine_session::{CompileCaps, compile_session, parse_session_toml};

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
                        chain
                            .process_dual_mono(block)
                            .expect("valid generated render");
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
            transcript_hash, 3_412_855_810_376_736_927,
            "updated only through a deliberate frozen-case change"
        );
    }
}
