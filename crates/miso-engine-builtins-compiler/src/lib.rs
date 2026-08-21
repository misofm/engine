//! Off-render preparation adapter for issue-007 builtins.
#![allow(missing_docs)]

use core::num::NonZeroU64;
use std::collections::BTreeSet;

#[cfg(feature = "test-support")]
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use miso_engine_builtins::{
    BuiltinChain, BuiltinParameterError, BuiltinParameters, BuiltinTail, ChannelParameters,
    DualMonoBlock, FaderMuteBuiltins, InputBuiltins, Matrix2x2, MatrixBuiltins, MeterAccumulator,
    MeterConfig, MeterConfigError, MeterHandle, MeterSnapshot, MeterTap, PreparedMeter, pan_matrix,
};
use miso_engine_core::realtime::{Consumer, RenderError, bounded_spsc_retained_payload};
use miso_engine_graph::{
    GraphBindingBlock, GraphNodeId, GraphNodeObserverBinding, GraphObservationBlock,
    GraphRuntimeObserver, GraphRuntimeProcessor, StableGraphId, TrackStage,
};
use miso_engine_session::{CompiledSession, MatrixOrPan, Track};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BuiltinCompileCaps {
    pub maximum_total_state_bytes: u64,
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
    observers: Vec<GraphNodeObserverBinding>,
    meter_consumers: Vec<MeterConsumer>,
    tails: Vec<(Box<str>, BuiltinTail)>,
    requests: Vec<MeterRequestSeal>,
    resources: BuiltinResourceEstimate,
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
static TEST_PHASE_TWO_TOTAL: AtomicU64 = AtomicU64::new(0);
#[cfg(feature = "test-support")]
static TEST_PHASE_TWO_LARGEST: AtomicU64 = AtomicU64::new(0);

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_reset_phase_two_allocation_tracker() {
    TEST_PHASE_TWO_ACTIVE.store(false, Ordering::SeqCst);
    TEST_PHASE_TWO_TOTAL.store(0, Ordering::SeqCst);
    TEST_PHASE_TWO_LARGEST.store(0, Ordering::SeqCst);
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_record_phase_two_allocation(layout: core::alloc::Layout) {
    if !TEST_PHASE_TWO_ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    let size = u64::try_from(layout.size()).expect("platform usize fits u64");
    TEST_PHASE_TWO_TOTAL.fetch_add(size, Ordering::Relaxed);
    let mut largest = TEST_PHASE_TWO_LARGEST.load(Ordering::Relaxed);
    while size > largest {
        match TEST_PHASE_TWO_LARGEST.compare_exchange_weak(
            largest,
            size,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(actual) => largest = actual,
        }
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn test_only_phase_two_allocation_snapshot() -> (u64, u64) {
    (
        TEST_PHASE_TWO_TOTAL.load(Ordering::SeqCst),
        TEST_PHASE_TWO_LARGEST.load(Ordering::SeqCst),
    )
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

/// The only graph-lowering payload obtainable from a sealed builtin artifact.
pub struct PreparedBuiltinsGraphParts {
    pub processors: Vec<miso_engine_graph::GraphNodeBinding>,
    pub observers: Vec<GraphNodeObserverBinding>,
    pub meter_consumers: Vec<MeterConsumer>,
}

/// Deliberate seal corruption available only to the graph compiler's adversarial tests.
///
/// This is not a wire format and is compiled out of production artifacts.  Keeping each seal
/// field independently reachable lets the graph boundary prove that it rejects the exact
/// corrupted tuple before it consumes either prepared input.
#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
}

/// Versioned public name for the exact engine-owned retained-payload report.
///
/// This is deliberately an alias rather than a duplicate accounting type: a caller cannot
/// accidentally read one report while the compiler validates another.
pub type BuiltinResourceReportV1 = BuiltinResourceEstimate;

#[derive(Clone, Copy, Debug, Default)]
struct ResourceAccumulator {
    total: u64,
    largest: u64,
    allocations: u64,
}

impl ResourceAccumulator {
    fn add_layout(&mut self, layout: core::alloc::Layout) -> Option<()> {
        let bytes = u64::try_from(layout.size()).ok()?;
        self.total = self.total.checked_add(bytes)?;
        self.largest = self.largest.max(bytes);
        self.allocations = self.allocations.checked_add(1)?;
        Some(())
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

    /// Consume a validated sealed artifact into the graph's private bindings.
    pub fn into_graph_parts(self) -> PreparedBuiltinsGraphParts {
        PreparedBuiltinsGraphParts {
            processors: self.processors,
            observers: self.observers,
            meter_consumers: self.meter_consumers,
        }
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn test_only_corrupt_for_compiler_test(&mut self, corruption: PreparedBuiltinsCorruption) {
        match corruption {
            PreparedBuiltinsCorruption::SessionIdentity => {
                self.seal.sample_rate = self.seal.sample_rate.checked_add(1).unwrap_or(0);
            }
            PreparedBuiltinsCorruption::Tracks => self.seal.tracks.clear(),
            PreparedBuiltinsCorruption::Processors => self.seal.processors.clear(),
            PreparedBuiltinsCorruption::Tails => self.tails.clear(),
            PreparedBuiltinsCorruption::Requests => self.seal.requests.push(MeterRequestSeal {
                handle: u64::MAX,
                track_id: "forged-request".into(),
                tap: MeterTap::Input,
                reset_generation: 0,
                period_frames: 1,
                peak_hold_frames: 0,
                peak_decay_bits: 0,
                queue_capacity: 1,
            }),
            PreparedBuiltinsCorruption::Observers => {
                self.seal
                    .observers
                    .push(("forged-observer".into(), TrackStage::Input, u64::MAX))
            }
            PreparedBuiltinsCorruption::Consumers => {
                self.seal
                    .consumers
                    .push((u64::MAX, "forged-consumer".into(), MeterTap::Input))
            }
            PreparedBuiltinsCorruption::Resources => {
                self.resources.engine_owned_retained_payload_bytes = self
                    .resources
                    .engine_owned_retained_payload_bytes
                    .checked_add(1)
                    .unwrap_or(0);
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
    let mut values: Vec<_> = tracks
        .iter()
        .flat_map(|track| {
            [
                TrackStage::PostInputBuiltins,
                TrackStage::PostFader,
                TrackStage::PostMatrix,
            ]
            .into_iter()
            .map(move |stage| (track.clone(), stage))
        })
        .collect();
    values.sort();
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
    values.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(values)
}

fn actual_meter_seals(
    observers: &[GraphNodeObserverBinding],
    consumers: &[MeterConsumer],
) -> (Vec<ObserverSeal>, Vec<ConsumerSeal>) {
    let mut observer_values: Vec<_> = observers
        .iter()
        .filter_map(|observer| match &observer.node {
            GraphNodeId::TrackStage { track_id, stage } => {
                Some((Box::<str>::from(track_id.as_str()), *stage, observer.handle))
            }
            _ => None,
        })
        .collect();
    observer_values.sort();
    let mut consumer_values: Vec<_> = consumers
        .iter()
        .map(|consumer| {
            (
                consumer.handle.0.get(),
                Box::<str>::from(&*consumer.track_id),
                consumer.tap,
            )
        })
        .collect();
    consumer_values.sort();
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
    if u64::try_from(requests.len()).map_or(true, |count| count > caps.maximum_meter_streams) {
        diagnostics.push(diag("builtin.resource.limit", "$.meter_requests"));
    }
    let mut request_keys = BTreeSet::new();
    for request in requests {
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
    let mut tails = Vec::with_capacity(track_count);
    for track in &session.normalized_model().tracks {
        let parameters = track_parameters(track, caps.maximum_smoothing_samples)
            .expect("preflighted parameters");
        let chain = BuiltinChain::new(session.sample_rate().0, parameters)
            .expect("preflighted coefficients");
        let tail = chain.tail();
        let (input, fader, matrix) = chain.into_sections();
        tails.push((Box::<str>::from(track.id.as_str()), tail));
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
    for (index, request) in requests.iter().enumerate() {
        let handle = MeterHandle(
            NonZeroU64::new(
                u64::try_from(index)
                    .expect("usize fits u64")
                    .checked_add(1)
                    .expect("request count bounded"),
            )
            .expect("one-based"),
        );
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
    tails.sort_by(|left, right| left.0.cmp(&right.0));
    request_seals.sort();
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
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    add_vector_layout::<Box<str>>(&mut processor, track_count)?;
    add_vector_layout::<(Box<str>, TrackStage)>(&mut processor, processor_count)?;
    add_vector_layout::<(Box<str>, BuiltinTail)>(&mut processor, track_count)?;
    for track in &session.normalized_model().tracks {
        let bytes = track.id.as_str().len();
        // The three graph IDs are independently cloned into their stage bindings, alongside the
        // retained tail, compact track seal, processor seal, and the seal's cloned tail ID.
        for _ in 0..9 {
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
            .add_bytes(queue.ring_header_bytes)
            .and_then(|_| meter.add_bytes(queue.slot_payload_bytes))
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
    Ok(BuiltinResourcePlan {
        report: BuiltinResourceEstimate {
            engine_owned_processor_payload_bytes: processor.total,
            engine_owned_meter_payload_bytes: meter.total,
            engine_owned_retained_payload_bytes: total,
            meter_items,
            maximum_single_allocation_bytes: processor.largest.max(meter.largest),
            retained_allocation_count: allocations,
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
    !value.is_finite()
        || value < 0.0
        || (value > 0.0 && (value < 10.0 || value >= sample_rate as f32 * 0.5))
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
    use core::num::{NonZeroU32, NonZeroUsize};
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
            maximum_total_meter_items: u64::MAX,
            maximum_total_meter_bytes: u64::MAX,
            maximum_single_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_period_frames: u32::MAX,
            maximum_peak_hold_frames: u32::MAX,
            maximum_smoothing_samples: u32::MAX,
        }
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
        .map(|tap| MeterRequest {
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
                    track_id: "missing".to_owned(),
                    tap: MeterTap::Input,
                    config,
                },
                MeterRequest {
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
            vec!["builtin.meter.duplicate", "builtin.meter.unknown_track"]
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

    /// Frozen issue-034 compiler-mutation seed. This exercises preparation requests and caps,
    /// never the scalar DSP render path or a timed workload.
    const BUILTIN_COMPILER_MUTATION_SEED: u64 = 0x34_007_c10_u64;

    #[test]
    fn deterministic_builtin_compiler_mutation_matrix_has_exactly_ten_thousand_cases() {
        let accepted = session();
        let base_config = MeterConfig {
            period_frames: NonZeroU32::new(16).expect("constant"),
            peak_hold_frames: 0,
            peak_decay_db_per_second: 0.0,
            queue_capacity: NonZeroUsize::new(4).expect("constant"),
            reset_generation: 0,
        };
        let baseline_request = MeterRequest {
            track_id: "vocal".to_owned(),
            tap: MeterTap::Input,
            config: base_config,
        };
        let baseline = prepare_session_builtins(&accepted, &[baseline_request], caps())
            .expect("baseline preparation");
        let report = baseline.resource_report();
        let mut state = BUILTIN_COMPILER_MUTATION_SEED;
        let mut transcript_hash = 0xcbf2_9ce4_8422_2325_u64;
        let mut seen_taps = BTreeSet::new();
        let mut classes = [false; 8];
        let mut completed = 0_u32;
        for case in 0_u32..10_000 {
            // xorshift64* is intentionally local and fixed; case descriptions are mixed into a
            // transcript hash so accidental coverage drift is visible in this test.
            state ^= state >> 12;
            state ^= state << 25;
            state ^= state >> 27;
            let value = state.wrapping_mul(0x2545_f491_4f6c_dd1d);
            let tap = [
                MeterTap::Input,
                MeterTap::PostInputBuiltins,
                MeterTap::PostSimd1,
                MeterTap::PostDynamic,
                MeterTap::PostSimd2PreFader,
                MeterTap::PostFader,
                MeterTap::PostMatrix,
            ][usize::try_from(value % 7).expect("bounded tap index")];
            seen_taps.insert(tap);
            let class = usize::try_from(case % 8).expect("bounded mutation class");
            classes[class] = true;
            let mut request = MeterRequest {
                track_id: "vocal".to_owned(),
                tap,
                config: MeterConfig {
                    period_frames: NonZeroU32::new(match class {
                        0 => 1,
                        1 => 127,
                        2 => 128,
                        _ => 16,
                    })
                    .expect("positive period"),
                    peak_hold_frames: match class {
                        3 => u32::MAX,
                        _ => {
                            u32::try_from(value & u64::from(u32::MAX)).expect("masked to u32") % 128
                        }
                    },
                    peak_decay_db_per_second: if class == 4 { f32::NAN } else { 0.0 },
                    queue_capacity: NonZeroUsize::new(match class {
                        5 => 1,
                        _ => 4,
                    })
                    .expect("positive queue"),
                    reset_generation: value,
                },
            };
            let mut requests = vec![request.clone()];
            if class == 5 {
                requests.push(request.clone());
            } else if class == 6 {
                request.track_id = "unknown-track".to_owned();
                requests[0] = request;
            }
            let mut mutation_caps = caps();
            if class == 7 {
                mutation_caps.maximum_total_state_bytes = report
                    .engine_owned_processor_payload_bytes
                    .checked_sub(1)
                    .expect("nonzero baseline state report");
            }
            let result = prepare_session_builtins(&accepted, &requests, mutation_caps);
            match result {
                Ok(prepared) => {
                    assert!(
                        prepared
                            .resource_report()
                            .engine_owned_retained_payload_bytes
                            > 0
                    );
                    transcript_hash ^= 0x51;
                }
                Err(diagnostics) => {
                    assert!(diagnostics.0.windows(2).all(|pair| pair[0] < pair[1]));
                    for diagnostic in diagnostics.0 {
                        for byte in diagnostic.code.bytes().chain(diagnostic.path.bytes()) {
                            transcript_hash ^= u64::from(byte);
                            transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
                        }
                    }
                }
            }
            transcript_hash ^= u64::from(case);
            transcript_hash = transcript_hash.wrapping_mul(0x100_0000_01b3);
            completed = completed.checked_add(1).expect("fixed case count");
        }
        assert_eq!(completed, 10_000);
        assert_eq!(seen_taps.len(), 7);
        assert!(classes.into_iter().all(core::convert::identity));
        assert_eq!(
            transcript_hash, 565_235_985_001_749_527,
            "updated only through a deliberate frozen-case change"
        );
    }
}
