//! One-shot, engine-captured spectrum windows.
//!
//! Capture is a graph observer with a fixed owner-side window.  The observer only copies the
//! selected render block into its prepared storage; [`SpectrumAnalyzer`] runs later, after the
//! window has crossed the SPSC boundary, and is therefore never reachable from render.

use core::num::NonZeroUsize;
#[cfg(any(test, feature = "test-support"))]
use std::cell::Cell;
use std::sync::{
    Arc,
    atomic::{AtomicU8, AtomicU64, Ordering},
};

use crate::observation_demand::HostSpectrumMode;
use engine::realtime::{
    Consumer, Producer, QueueGeneration, RenderError, bounded_spsc, bounded_spsc_retained_payload,
};
use graph::{
    GraphNodeId, GraphNodeObserverBinding, GraphObservationBlock, GraphObservationValidity,
    GraphResidentObservationBlock, GraphRuntimeObserver, StableGraphId, TrackStage,
};

/// The fixed number of samples in one spectrum capture.
pub const SPECTRUM_WINDOW_FRAMES: usize = 2048;
/// The number of nonnegative-frequency bins in a real 2048-point transform.
pub const SPECTRUM_BIN_COUNT: usize = SPECTRUM_WINDOW_FRAMES / 2 + 1;
/// The finite lower dBFS display floor used by the analyzer.
pub const SPECTRUM_FLOOR_DB: f32 = -120.0;
/// A private observer handle outside the meter handle range used by prepared console taps.
const SPECTRUM_OBSERVER_HANDLE: u64 = u64::MAX;
const IDLE: u8 = 0;
const ARMED: u8 = 1;
const CAPTURING: u8 = 2;
const COMPLETE: u8 = 3;
const INVALID: u8 = 4;
const CONTINUOUS_WARMING: u8 = 0;
const CONTINUOUS_CAPTURING: u8 = 1;
const CONTINUOUS_WAITING: u8 = 2;
const ONE_SHOT_MODE: u8 = 0;
const CONTINUOUS_MODE: u8 = 1;

const PROBE_VALIDATION_SAMPLES: usize = 0;
const PROBE_STORAGE_WRITES: usize = 1;
const PROBE_PAYLOAD_CONSTRUCTIONS: usize = 2;
const PROBE_PUBLICATION_ATTEMPTS: usize = 3;

/// Source-operation counts used by the focused spectrum render tests.
#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SpectrumOperationCounts {
    /// Selected samples checked for finiteness.
    pub validation_samples: u64,
    /// Selected samples written to the prepared capture arrays.
    pub selected_storage_writes: u64,
    /// Complete record payloads constructed for publication.
    pub payload_constructions: u64,
    /// Publication attempts, including attempts rejected by a full queue.
    pub publication_attempts: u64,
}

#[cfg(any(test, feature = "test-support"))]
std::thread_local! {
    static SPECTRUM_OPERATION_COUNTS: Cell<SpectrumOperationCounts> = const {
        Cell::new(SpectrumOperationCounts {
            validation_samples: 0,
            selected_storage_writes: 0,
            payload_constructions: 0,
            publication_attempts: 0,
        })
    };
}

#[cfg(any(test, feature = "test-support"))]
#[inline]
fn probe_add(which: usize, amount: usize) {
    SPECTRUM_OPERATION_COUNTS.with(|value| {
        let mut counts = value.get();
        let amount = u64::try_from(amount).unwrap_or(u64::MAX);
        match which {
            PROBE_VALIDATION_SAMPLES => {
                counts.validation_samples = counts.validation_samples.saturating_add(amount)
            }
            PROBE_STORAGE_WRITES => {
                counts.selected_storage_writes =
                    counts.selected_storage_writes.saturating_add(amount)
            }
            PROBE_PAYLOAD_CONSTRUCTIONS => {
                counts.payload_constructions = counts.payload_constructions.saturating_add(amount)
            }
            PROBE_PUBLICATION_ATTEMPTS => {
                counts.publication_attempts = counts.publication_attempts.saturating_add(amount)
            }
            _ => {}
        }
        value.set(counts);
    });
}

#[cfg(not(any(test, feature = "test-support")))]
#[inline(always)]
fn probe_add(_which: usize, _amount: usize) {}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn test_only_reset_spectrum_operation_counts() {
    SPECTRUM_OPERATION_COUNTS.with(|value| value.set(SpectrumOperationCounts::default()));
}

#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn test_only_spectrum_operation_counts() -> SpectrumOperationCounts {
    SPECTRUM_OPERATION_COUNTS.with(Cell::get)
}

/// Channels copied into and analyzed for one spectrum window.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumChannels {
    /// Capture and analyze the left plane only.
    Left = 1,
    /// Capture and analyze the right plane only.
    Right = 2,
    /// Capture and analyze both planes.
    Stereo = 3,
}

impl SpectrumChannels {
    const fn includes_left(self) -> bool {
        (self as u8) & (Self::Left as u8) != 0
    }

    const fn includes_right(self) -> bool {
        (self as u8) & (Self::Right as u8) != 0
    }
}

/// A graph boundary from which one window may be captured.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpectrumTarget {
    /// The selected track immediately after its input builtins.
    TrackPostInputBuiltins(Box<str>),
    /// The selected track immediately after its fader/matrix stage.
    TrackPostMatrix(Box<str>),
    /// The designated final output node.
    Output(Box<str>),
}

impl SpectrumTarget {
    /// Construct the graph node represented by this target.
    pub(crate) fn graph_node(&self) -> Option<GraphNodeId> {
        match self {
            Self::TrackPostInputBuiltins(track_id) => Some(GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track_id)?,
                stage: TrackStage::PostInputBuiltins,
            }),
            Self::TrackPostMatrix(track_id) => Some(GraphNodeId::TrackStage {
                track_id: StableGraphId::parse(track_id)?,
                stage: TrackStage::PostMatrix,
            }),
            Self::Output(output_id) => Some(GraphNodeId::Output {
                output_id: StableGraphId::parse(output_id)?,
            }),
        }
    }

    fn id(&self) -> &str {
        match self {
            Self::TrackPostInputBuiltins(track_id)
            | Self::TrackPostMatrix(track_id)
            | Self::Output(track_id) => track_id,
        }
    }
}

/// Preparation-time request for one fixed, bounded spectrum capture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpectrumCaptureRequest {
    /// The graph boundary to observe.
    pub target: SpectrumTarget,
    /// The channel planes the observer copies and the analyzer publishes.
    pub channels: SpectrumChannels,
    /// Maximum bytes the caller permits this prepared capture to retain.
    pub maximum_capture_bytes: u64,
}

/// One explicitly prepared graph tap in a spectrum capture collection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpectrumCaptureCollectionEntry {
    /// The graph boundary to observe.
    pub target: SpectrumTarget,
    /// The channel planes the observer copies and the analyzer publishes.
    pub channels: SpectrumChannels,
}

/// A bounded set of spectrum taps prepared together for atomic selection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpectrumCaptureCollectionRequest {
    /// The exact target/channel entries admitted by this preparation.
    pub entries: Vec<SpectrumCaptureCollectionEntry>,
    /// Aggregate bytes the caller permits the prepared collection to retain.
    pub maximum_capture_bytes: u64,
}

/// Address-free capture storage facts used by preparation and host projections.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpectrumCaptureResources {
    /// All retained bytes for the observer and one completed-window queue slot.
    pub retained_bytes: u64,
    /// The largest individual requested allocation.
    pub largest_allocation_bytes: u64,
}

/// Refusal while changing the active entry in a prepared spectrum collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumCaptureCollectionSelectionError {
    /// No prepared entry exactly matched the requested target and channel mask.
    UnknownEntry,
    /// The selected entry could not be armed after the old entry was left untouched.
    Busy,
    /// The bounded selection identity could not advance without reuse.
    EpochOverflow,
}

/// Refusal while reading the active entry in a prepared spectrum collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumCaptureCollectionReadError {
    /// The collection has no active entry.
    NoSelection,
    /// The selected entry returned its ordinary capture status.
    Capture(SpectrumCaptureReadError),
}

/// The separately budgeted worker-side state retained by power smoothing.
///
/// This report is intentionally separate from [`SpectrumCaptureResources`]: the arrays belong to
/// the control/worker analyzer and are never retained by the graph observer or touched by render.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpectrumAnalysisHistoryResources {
    /// Bytes retained by one analyzer history state.
    pub retained_bytes: u64,
    /// The largest individual worker allocation represented by this state.
    pub largest_allocation_bytes: u64,
}

/// Return the bounded worker-side storage cost of one smoothing history.
#[must_use]
pub fn spectrum_analysis_history_resources() -> SpectrumAnalysisHistoryResources {
    let bytes = u64::try_from(core::mem::size_of::<SpectrumAnalysisHistory>())
        .expect("spectrum history bytes fit u64");
    SpectrumAnalysisHistoryResources {
        retained_bytes: bytes,
        largest_allocation_bytes: bytes,
    }
}

/// Return the fixed storage cost of one prepared spectrum observer.
#[must_use]
pub fn spectrum_capture_resources() -> SpectrumCaptureResources {
    spectrum_capture_resources_for_id_bytes(1)
}

/// Return the fixed storage cost plus target metadata for one selected target.
#[must_use]
pub fn spectrum_capture_resources_for(target: &SpectrumTarget) -> SpectrumCaptureResources {
    spectrum_capture_resources_for_id_bytes(target.id().len())
}

/// Return the aggregate fixed storage cost of a prepared collection.
///
/// The collection owns one control-side capture value per entry in addition to the observer,
/// queue, and graph binding charged by [`spectrum_capture_resources_for`]. Duplicate exact entries
/// are rejected before any capture is prepared.
pub fn spectrum_capture_collection_resources(
    entries: &[SpectrumCaptureCollectionEntry],
) -> Result<SpectrumCaptureResources, SpectrumPrepareError> {
    let mut retained_bytes = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    for (index, entry) in entries.iter().enumerate() {
        if entries[..index].iter().any(|previous| previous == entry) {
            return Err(SpectrumPrepareError::DuplicateEntry);
        }
        let resources = spectrum_capture_resources_for(&entry.target);
        retained_bytes = retained_bytes
            .checked_add(resources.retained_bytes)
            .ok_or(SpectrumPrepareError::CollectionCapacity)?;
        largest_allocation_bytes = largest_allocation_bytes.max(resources.largest_allocation_bytes);
    }
    let capture_values = u64::try_from(
        entries
            .len()
            .checked_mul(core::mem::size_of::<SpectrumCapture>())
            .ok_or(SpectrumPrepareError::CollectionCapacity)?,
    )
    .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    Ok(SpectrumCaptureResources {
        retained_bytes: retained_bytes
            .checked_add(capture_values)
            .ok_or(SpectrumPrepareError::CollectionCapacity)?,
        largest_allocation_bytes: largest_allocation_bytes.max(capture_values),
    })
}

/// Return the exact storage cost of a host-controlled paired capture collection.
///
/// Controlled preparation retains two complete singular captures for every public entry. The
/// boxed entry array is the only additional collection allocation; it already contains both
/// control-side [`SpectrumCapture`] values and their fixed lifecycle metadata.
pub(crate) fn controlled_spectrum_capture_collection_resources(
    entries: &[SpectrumCaptureCollectionEntry],
) -> Result<SpectrumCaptureResources, SpectrumPrepareError> {
    let mut retained_bytes = 0_u64;
    let mut largest_allocation_bytes = 0_u64;
    for (index, entry) in entries.iter().enumerate() {
        if entries[..index].iter().any(|previous| previous == entry) {
            return Err(SpectrumPrepareError::DuplicateEntry);
        }
        let resources = spectrum_capture_resources_for(&entry.target);
        retained_bytes = retained_bytes
            .checked_add(
                resources
                    .retained_bytes
                    .checked_mul(2)
                    .ok_or(SpectrumPrepareError::CollectionCapacity)?,
            )
            .ok_or(SpectrumPrepareError::CollectionCapacity)?;
        largest_allocation_bytes = largest_allocation_bytes.max(resources.largest_allocation_bytes);
    }
    let entry_bytes = entries
        .len()
        .checked_mul(core::mem::size_of::<ControlledSpectrumCaptureEntry>())
        .ok_or(SpectrumPrepareError::CollectionCapacity)?;
    let entry_bytes =
        u64::try_from(entry_bytes).map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    Ok(SpectrumCaptureResources {
        retained_bytes: retained_bytes
            .checked_add(entry_bytes)
            .ok_or(SpectrumPrepareError::CollectionCapacity)?,
        largest_allocation_bytes: largest_allocation_bytes.max(entry_bytes),
    })
}

fn spectrum_capture_resources_for_id_bytes(id_bytes: usize) -> SpectrumCaptureResources {
    let queue = bounded_spsc_retained_payload::<SpectrumCapturedRecord>(
        NonZeroUsize::new(1).expect("one queue slot"),
    )
    .expect("fixed spectrum queue layout");
    let observer_bytes = core::mem::size_of::<SpectrumCaptureObserver>();
    let queue_bytes = queue
        .total_bytes()
        .expect("fixed spectrum queue bytes fit usize");
    let state_bytes = core::mem::size_of::<SpectrumStateAllocation>();
    let continuous_shared_bytes = core::mem::size_of::<SpectrumContinuousSharedAllocation>();
    let observer_binding_bytes = core::mem::size_of::<GraphNodeObserverBinding>();
    let observer_bytes = u64::try_from(observer_bytes).expect("observer bytes fit u64");
    let queue_bytes = u64::try_from(queue_bytes).expect("queue bytes fit u64");
    let state_bytes = u64::try_from(state_bytes).expect("state bytes fit u64");
    let continuous_shared_bytes =
        u64::try_from(continuous_shared_bytes).expect("continuous state bytes fit u64");
    let observer_binding_bytes =
        u64::try_from(observer_binding_bytes).expect("observer binding bytes fit u64");
    let id_bytes = u64::try_from(id_bytes).expect("target ID bytes fit u64");
    let target_ids = id_bytes.checked_mul(2).expect("target IDs fit u64");
    SpectrumCaptureResources {
        retained_bytes: observer_bytes
            .checked_add(queue_bytes)
            .and_then(|value| value.checked_add(state_bytes))
            .and_then(|value| value.checked_add(state_bytes))
            .and_then(|value| value.checked_add(continuous_shared_bytes))
            .and_then(|value| value.checked_add(observer_binding_bytes))
            .and_then(|value| value.checked_add(target_ids))
            .expect("spectrum storage fits u64"),
        largest_allocation_bytes: observer_bytes
            .max(
                u64::try_from(queue.largest_allocation_bytes()).expect("queue allocation fits u64"),
            )
            .max(state_bytes)
            .max(continuous_shared_bytes)
            .max(observer_binding_bytes)
            .max(id_bytes),
    }
}

#[repr(C)]
struct SpectrumStateAllocation {
    strong: std::sync::atomic::AtomicUsize,
    weak: std::sync::atomic::AtomicUsize,
    state: AtomicU8,
}

/// Preparation refusal for a selected spectrum boundary or its bounded storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumPrepareError {
    /// The target ID is malformed or is absent from the prepared graph.
    UnknownTarget,
    /// The caller's capture budget is smaller than the fixed prepared storage.
    CaptureBudget,
    /// The host's largest-allocation cap is smaller than the observer or queue allocation.
    AllocationBudget,
    /// The fixed queue could not be represented by the platform.
    QueueCapacity,
    /// The request did not select either channel plane.
    NoChannels,
    /// The collection repeated an exact target and channel mask.
    DuplicateEntry,
    /// Collection resource arithmetic or control storage could not be represented.
    CollectionCapacity,
}

/// A completed, immutable 2048-frame engine window.
#[derive(Clone, Copy, Debug)]
pub struct SpectrumWindow {
    /// The captured left plane.
    pub left: [f32; SPECTRUM_WINDOW_FRAMES],
    /// The captured right plane.
    pub right: [f32; SPECTRUM_WINDOW_FRAMES],
    /// Absolute sample at the beginning of the window.
    pub first_sample: u64,
    /// The channel planes actually copied into this window.
    pub channels: SpectrumChannels,
    /// Whether a source underrun was known while this window was captured.
    pub source_underrun: bool,
}

impl SpectrumWindow {
    /// The exclusive absolute sample at the end of this fixed-size window, when representable.
    #[must_use]
    pub fn end_sample(&self) -> Option<u64> {
        self.first_sample
            .checked_add(u64::try_from(SPECTRUM_WINDOW_FRAMES).ok()?)
    }
}

/// One bounded queue item shared by one-shot and continuous capture modes.
///
/// The one-shot projection ignores the stream fields. Keeping one queue item and one observer
/// means continuous capture cannot introduce a second render-side storage path.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SpectrumCapturedRecord {
    window: SpectrumWindow,
    observation_generation: u64,
    stream_epoch: u64,
    sequence: u64,
    dropped_captures: u64,
}

impl SpectrumCapturedRecord {
    /// The graph-observation generation that produced this record.
    #[must_use]
    pub(crate) const fn observation_generation(&self) -> u64 {
        self.observation_generation
    }

    /// Project this record through the one-shot public window shape.
    #[must_use]
    pub(crate) const fn window(&self) -> SpectrumWindow {
        let _observation_generation = self.observation_generation();
        self.window
    }

    /// Project this record through the continuous public window shape.
    #[must_use]
    pub(crate) const fn continuous_window(&self) -> SpectrumContinuousWindow {
        SpectrumContinuousWindow {
            left: self.window.left,
            right: self.window.right,
            first_sample: self.window.first_sample,
            channels: self.window.channels,
            source_underrun: self.window.source_underrun,
            stream_epoch: self.stream_epoch,
            sequence: self.sequence,
            dropped_captures: self.dropped_captures,
        }
    }
}

/// Control-side ownership of one prepared graph observer.
pub struct SpectrumCapture {
    consumer: Consumer<SpectrumCapturedRecord>,
    state: Arc<AtomicU8>,
    mode: Arc<AtomicU8>,
    shared: Arc<SpectrumContinuousShared>,
    seen_failures: u64,
    seen_drops: u64,
    target: SpectrumTarget,
    channels: SpectrumChannels,
}

impl SpectrumCapture {
    #[cfg(test)]
    fn new_one_shot_for_test(
        consumer: Consumer<SpectrumCapturedRecord>,
        state: Arc<AtomicU8>,
        target: SpectrumTarget,
    ) -> Self {
        Self {
            consumer,
            state,
            mode: Arc::new(AtomicU8::new(ONE_SHOT_MODE)),
            shared: SpectrumContinuousShared::new(),
            seen_failures: 0,
            seen_drops: 0,
            target,
            channels: SpectrumChannels::Left,
        }
    }

    /// Reset a known-free controlled slot before its next graph publication.
    ///
    /// This path intentionally does not use the public arm/start/cancel methods. The controlled
    /// owner has already consumed the removal receipt that made the slot free, and staging must
    /// leave the observer inactive until the graph publishes its new activation snapshot.
    #[allow(dead_code)] // C2 private seam is consumed by controlled host admission in C3/C4.
    fn reset_for_controlled_stage(&mut self, mode: HostSpectrumMode, cadence: SpectrumCadence) {
        // The queue is bounded to one record. A free slot normally has no record, but the single
        // bounded pop makes the ownership invariant explicit without a render-side drain loop.
        let _ = self.consumer.try_pop();
        self.state.store(IDLE, Ordering::Release);
        self.seen_failures = 0;
        self.seen_drops = 0;
        self.shared.active.store(0, Ordering::Release);
        self.shared
            .phase
            .store(CONTINUOUS_WAITING, Ordering::Release);
        self.shared.epoch.store(0, Ordering::Release);
        self.shared.failures.store(0, Ordering::Release);
        self.shared.failure_epoch.store(0, Ordering::Release);
        self.shared.drops.store(0, Ordering::Release);
        self.shared.drop_epoch.store(0, Ordering::Release);
        self.shared.invalidated.store(0, Ordering::Release);
        self.shared.invalidated_epoch.store(0, Ordering::Release);
        match mode {
            HostSpectrumMode::OneShot => {
                self.mode.store(ONE_SHOT_MODE, Ordering::Release);
                self.shared.sample_rate_hz.store(0, Ordering::Release);
                self.shared.quantum_frames.store(0, Ordering::Release);
                self.shared.hop_frames.store(0, Ordering::Release);
            }
            HostSpectrumMode::Continuous => {
                self.mode.store(CONTINUOUS_MODE, Ordering::Release);
                self.shared.epoch.store(1, Ordering::Release);
                self.shared.failure_epoch.store(1, Ordering::Release);
                self.shared.drop_epoch.store(1, Ordering::Release);
                self.shared.invalidated_epoch.store(1, Ordering::Release);
                self.shared
                    .sample_rate_hz
                    .store(u64::from(cadence.sample_rate_hz()), Ordering::Release);
                self.shared
                    .quantum_frames
                    .store(u64::from(cadence.quantum_frames()), Ordering::Release);
                self.shared
                    .hop_frames
                    .store(u64::from(cadence.hop_frames()), Ordering::Release);
            }
        }
    }

    /// Clean one retired controlled slot after its graph removal receipt.
    #[allow(dead_code)] // C2 private seam is consumed by controlled host admission in C4.
    fn retire_controlled_after_receipt(&mut self) {
        // The prepared queue has one item, so retirement is bounded to one off-render pop.
        let _ = self.consumer.try_pop();
        self.state.store(IDLE, Ordering::Release);
        self.mode.store(ONE_SHOT_MODE, Ordering::Release);
        self.shared.active.store(0, Ordering::Release);
        self.shared
            .phase
            .store(CONTINUOUS_WAITING, Ordering::Release);
        self.seen_failures = 0;
        self.seen_drops = 0;
    }

    /// Arm the capture for the next complete graph window.
    ///
    /// Arming is a control-side operation.  The first successfully observed block establishes
    /// the returned window's absolute start sample.
    pub fn arm(&self) -> Result<(), SpectrumCaptureError> {
        if self.mode.load(Ordering::Acquire) != ONE_SHOT_MODE {
            return Err(SpectrumCaptureError::Busy);
        }
        self.state
            .compare_exchange(IDLE, ARMED, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|_| SpectrumCaptureError::Busy)
    }

    /// Cancel the current capture, discarding any completed window.
    pub fn cancel(&mut self) {
        while self.consumer.try_pop().is_ok() {}
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            self.shared.active.store(0, Ordering::Release);
            self.shared
                .phase
                .store(CONTINUOUS_WAITING, Ordering::Release);
            self.mode.store(ONE_SHOT_MODE, Ordering::Release);
        }
        self.state.store(IDLE, Ordering::Release);
    }

    /// Try to take the completed window from its control-side queue.
    pub fn try_read(&mut self) -> Result<SpectrumWindow, SpectrumCaptureReadError> {
        self.try_read_record(None).map(|record| record.window())
    }

    /// Try to take one-shot record identity without projecting it away.
    ///
    /// `None` preserves the legacy state machine. `Some(maximum_pops)` bounds this call to the
    /// supplied number of queue pops, including zero; protected owners pass the queue availability
    /// frozen at their read entry.
    pub(crate) fn try_read_record(
        &mut self,
        maximum_pops: Option<usize>,
    ) -> Result<SpectrumCapturedRecord, SpectrumCaptureReadError> {
        if self.mode.load(Ordering::Acquire) != ONE_SHOT_MODE {
            return Err(SpectrumCaptureReadError::NotArmed);
        }
        match self.state.load(Ordering::Acquire) {
            COMPLETE => {
                if maximum_pops.is_some_and(|maximum| maximum == 0) {
                    return Err(SpectrumCaptureReadError::Pending);
                }
                match self.consumer.try_pop() {
                    Ok(record) => {
                        self.state.store(IDLE, Ordering::Release);
                        Ok(record)
                    }
                    Err(_) => Err(SpectrumCaptureReadError::Pending),
                }
            }
            ARMED | CAPTURING => Err(SpectrumCaptureReadError::Pending),
            INVALID => {
                // The failed render may have queued a completed window before its error. The
                // queue has one slot, so one bounded pop clears that stale result.
                if !maximum_pops.is_some_and(|maximum| maximum == 0) {
                    let _ = self.consumer.try_pop();
                }
                self.state.store(IDLE, Ordering::Release);
                Err(SpectrumCaptureReadError::Invalid)
            }
            IDLE => Err(SpectrumCaptureReadError::NotArmed),
            _ => Err(SpectrumCaptureReadError::Invalid),
        }
    }

    /// Start scheduled windows for the prepared target.
    pub fn start_continuous(
        &mut self,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Result<SpectrumCadence, SpectrumContinuousCaptureError> {
        if self.mode.load(Ordering::Acquire) != ONE_SHOT_MODE
            || self.state.load(Ordering::Acquire) != IDLE
        {
            return Err(SpectrumContinuousCaptureError::Busy);
        }
        let cadence = SpectrumCadence::new(sample_rate_hz, quantum_frames)
            .map_err(SpectrumContinuousCaptureError::Cadence)?;
        self.begin_continuous(cadence, false)?;
        Ok(cadence)
    }

    fn begin_continuous(
        &mut self,
        cadence: SpectrumCadence,
        replace_one_shot: bool,
    ) -> Result<(), SpectrumContinuousCaptureError> {
        if self.mode.load(Ordering::Acquire) != ONE_SHOT_MODE
            || (!replace_one_shot && self.state.load(Ordering::Acquire) != IDLE)
        {
            return Err(SpectrumContinuousCaptureError::Busy);
        }
        if self
            .shared
            .active
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(SpectrumContinuousCaptureError::Busy);
        }
        let Some(epoch) = self.shared.epoch.load(Ordering::Acquire).checked_add(1) else {
            self.shared.active.store(0, Ordering::Release);
            return Err(SpectrumContinuousCaptureError::EpochOverflow);
        };
        // Every fallible admission check is complete before the selected one-shot state is
        // cleared. Collection mode uses this path for an atomic mode replacement.
        self.commit_continuous(cadence, epoch);
        Ok(())
    }

    /// Restart the active continuous capture at one fresh epoch without a fallible stop/start
    /// pair. The caller has already validated the replacement configuration.
    pub fn restart_continuous(&mut self) -> Result<(), SpectrumContinuousCaptureError> {
        if self.mode.load(Ordering::Acquire) != CONTINUOUS_MODE
            || self.shared.active.load(Ordering::Acquire) == 0
        {
            return Err(SpectrumContinuousCaptureError::Busy);
        }
        let Some(epoch) = self.shared.epoch.load(Ordering::Acquire).checked_add(1) else {
            return Err(SpectrumContinuousCaptureError::EpochOverflow);
        };
        let Some(cadence) = self.cadence() else {
            return Err(SpectrumContinuousCaptureError::Busy);
        };
        self.commit_continuous(cadence, epoch);
        Ok(())
    }

    fn commit_continuous(&mut self, cadence: SpectrumCadence, epoch: u64) {
        while self.consumer.try_pop().is_ok() {}
        self.state.store(IDLE, Ordering::Release);
        self.shared.epoch.store(epoch, Ordering::Release);
        self.shared
            .sample_rate_hz
            .store(u64::from(cadence.sample_rate_hz()), Ordering::Release);
        self.shared
            .quantum_frames
            .store(u64::from(cadence.quantum_frames()), Ordering::Release);
        self.shared
            .hop_frames
            .store(u64::from(cadence.hop_frames()), Ordering::Release);
        self.shared
            .phase
            .store(CONTINUOUS_WARMING, Ordering::Release);
        self.shared.failures.store(0, Ordering::Release);
        self.shared.failure_epoch.store(epoch, Ordering::Release);
        self.shared.drops.store(0, Ordering::Release);
        self.shared.drop_epoch.store(epoch, Ordering::Release);
        self.shared.invalidated.store(0, Ordering::Release);
        self.seen_failures = 0;
        self.seen_drops = 0;
        self.mode.store(CONTINUOUS_MODE, Ordering::Release);
    }

    /// Stop scheduled windows and discard queued continuous results.
    pub fn stop_continuous(&mut self) {
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            self.shared.active.store(0, Ordering::Release);
            self.shared
                .phase
                .store(CONTINUOUS_WAITING, Ordering::Release);
            self.mode.store(ONE_SHOT_MODE, Ordering::Release);
            while self.consumer.try_pop().is_ok() {}
            self.state.store(IDLE, Ordering::Release);
            self.seen_failures = 0;
            self.seen_drops = 0;
        }
    }

    /// Return the active continuous profile, when this handle is running continuously.
    #[must_use]
    pub fn cadence(&self) -> Option<SpectrumCadence> {
        if self.mode.load(Ordering::Acquire) != CONTINUOUS_MODE {
            return None;
        }
        Some(SpectrumCadence {
            sample_rate_hz: u32::try_from(self.shared.sample_rate_hz.load(Ordering::Acquire))
                .ok()?,
            quantum_frames: u32::try_from(self.shared.quantum_frames.load(Ordering::Acquire))
                .ok()?,
            hop_frames: u32::try_from(self.shared.hop_frames.load(Ordering::Acquire)).ok()?,
        })
    }

    /// Return the current capture epoch for an active continuous stream.
    #[must_use]
    pub fn stream_epoch(&self) -> Option<u64> {
        if self.mode.load(Ordering::Acquire) != CONTINUOUS_MODE {
            return None;
        }
        Some(self.shared.epoch.load(Ordering::Acquire))
    }

    /// Read one scheduled window and its stream-history metadata.
    pub fn try_read_continuous(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        self.try_read_continuous_record(None)
            .map(|record| record.continuous_window())
    }

    /// Try to take one continuous record while retaining its private identity.
    ///
    /// `None` preserves the legacy drain-until-a-result behavior. `Some(maximum_pops)` bounds
    /// queue consumption to the supplied entry budget; a protected caller freezes that budget
    /// before attempting its first pop, so a stale item cannot trigger a refill chase.
    pub(crate) fn try_read_continuous_record(
        &mut self,
        maximum_pops: Option<usize>,
    ) -> Result<SpectrumCapturedRecord, SpectrumContinuousReadError> {
        if self.mode.load(Ordering::Acquire) != CONTINUOUS_MODE
            || self.shared.active.load(Ordering::Acquire) == 0
        {
            return Err(SpectrumContinuousReadError::NotActive);
        }
        let failures = self.shared.failures.load(Ordering::Acquire);
        if failures != self.seen_failures {
            self.seen_failures = failures;
            self.seen_drops = self.shared.drops.load(Ordering::Acquire);
            return Err(SpectrumContinuousReadError::Failed {
                stream_epoch: self.shared.failure_epoch.load(Ordering::Acquire),
            });
        }
        let drops = self.shared.drops.load(Ordering::Acquire);
        if drops != self.seen_drops {
            self.seen_drops = drops;
            return Err(SpectrumContinuousReadError::Gap {
                stream_epoch: self.shared.drop_epoch.load(Ordering::Acquire),
                dropped_captures: drops,
            });
        }
        let mut remaining_pops = maximum_pops;
        loop {
            if remaining_pops.is_some_and(|remaining| remaining == 0) {
                return Err(
                    if self.shared.phase.load(Ordering::Acquire) == CONTINUOUS_WARMING {
                        SpectrumContinuousReadError::Warming
                    } else {
                        SpectrumContinuousReadError::Pending
                    },
                );
            }
            if let Some(remaining) = remaining_pops.as_mut() {
                *remaining -= 1;
            }
            match self.consumer.try_pop() {
                Ok(record)
                    if self.shared.invalidated.load(Ordering::Acquire) != 0
                        && record.stream_epoch
                            <= self.shared.invalidated_epoch.load(Ordering::Acquire) =>
                {
                    self.shared.invalidated.store(0, Ordering::Release);
                }
                Ok(record) => {
                    return Ok(record);
                }
                Err(_) if self.shared.phase.load(Ordering::Acquire) == CONTINUOUS_WARMING => {
                    return Err(SpectrumContinuousReadError::Warming);
                }
                Err(_) => return Err(SpectrumContinuousReadError::Pending),
            }
        }
    }

    /// The selected graph boundary for this capture.
    #[must_use]
    pub fn target(&self) -> &SpectrumTarget {
        &self.target
    }

    /// The exact channel mask prepared for this capture.
    #[must_use]
    pub const fn channels(&self) -> SpectrumChannels {
        self.channels
    }
}

/// Control-side ownership of several explicitly prepared spectrum captures.
///
/// Every entry owns the same fixed observer and one-slot queue used by the singular API. Only the
/// selected entry is armed, so inactive entries perform no PCM copies or analysis work. Selection
/// is an exclusive control-side operation and must occur between render calls.
pub struct SpectrumCaptureCollection {
    captures: Box<[SpectrumCapture]>,
    selected: Option<usize>,
    selection_epoch: u64,
}

impl SpectrumCaptureCollection {
    fn new(captures: Vec<SpectrumCapture>) -> Self {
        Self {
            captures: captures.into_boxed_slice(),
            selected: None,
            selection_epoch: 0,
        }
    }

    /// Number of prepared entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.captures.len()
    }

    /// Whether this collection prepared no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.captures.is_empty()
    }

    /// Return the accepted entry at `index`, including its stable target identity and mask.
    #[must_use]
    pub fn entry(&self, index: usize) -> Option<SpectrumCaptureCollectionEntry> {
        self.captures
            .get(index)
            .map(|capture| SpectrumCaptureCollectionEntry {
                target: capture.target.clone(),
                channels: capture.channels,
            })
    }

    /// Return the currently selected entry, if any.
    #[must_use]
    pub fn selected_entry(&self) -> Option<SpectrumCaptureCollectionEntry> {
        self.selected.and_then(|index| self.entry(index))
    }

    /// Return the selected target identity without cloning it.
    #[must_use]
    pub fn selected_target(&self) -> Option<&SpectrumTarget> {
        self.selected.map(|index| self.captures[index].target())
    }

    /// Return the selected entry index in preparation order.
    #[must_use]
    pub const fn selected_index(&self) -> Option<usize> {
        self.selected
    }

    /// Monotonic identity of the last committed selection change.
    #[must_use]
    pub const fn selection_epoch(&self) -> u64 {
        self.selection_epoch
    }

    /// Arm the currently selected entry for one fresh one-shot window.
    pub fn arm_selected(&self) -> Result<(), SpectrumCaptureError> {
        let Some(index) = self.selected else {
            return Err(SpectrumCaptureError::Busy);
        };
        self.captures[index].arm()
    }

    /// Cancel every capture and leave the collection unarmed.
    pub fn cancel(&mut self) {
        for capture in &mut self.captures {
            capture.cancel();
        }
    }

    /// Atomically replace the selected one-shot capture with an exact prepared entry.
    ///
    /// The target and mask are validated before touching the current capture. A successful
    /// replacement clears every old partial or queued result and arms the new entry. If the old
    /// entry was running continuously, the same cadence is restarted for the new entry.
    pub fn select(
        &mut self,
        target: &SpectrumTarget,
        channels: SpectrumChannels,
    ) -> Result<SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionSelectionError> {
        let Some(index) = self
            .captures
            .iter()
            .position(|capture| capture.target == *target && capture.channels == channels)
        else {
            return Err(SpectrumCaptureCollectionSelectionError::UnknownEntry);
        };
        if self.selected == Some(index) {
            return self
                .entry(index)
                .ok_or(SpectrumCaptureCollectionSelectionError::UnknownEntry);
        }

        let next_selection_epoch = self
            .selection_epoch
            .checked_add(1)
            .ok_or(SpectrumCaptureCollectionSelectionError::EpochOverflow)?;

        // Prepare the new state before touching the old one. All checks that can fail therefore
        // preserve the old active capture and its queued result.
        let cadence = self.selected.and_then(|old| self.captures[old].cadence());
        if let Some(cadence) = cadence {
            self.captures[index]
                .start_continuous(cadence.sample_rate_hz(), cadence.quantum_frames())
                .map_err(|error| match error {
                    SpectrumContinuousCaptureError::EpochOverflow => {
                        SpectrumCaptureCollectionSelectionError::EpochOverflow
                    }
                    SpectrumContinuousCaptureError::Busy
                    | SpectrumContinuousCaptureError::Cadence(_) => {
                        SpectrumCaptureCollectionSelectionError::Busy
                    }
                })?;
        } else {
            self.captures[index]
                .arm()
                .map_err(|_| SpectrumCaptureCollectionSelectionError::Busy)?;
        }
        for (capture_index, capture) in self.captures.iter_mut().enumerate() {
            if capture_index != index {
                capture.cancel();
            }
        }
        self.selected = Some(index);
        self.selection_epoch = next_selection_epoch;
        self.entry(index)
            .ok_or(SpectrumCaptureCollectionSelectionError::UnknownEntry)
    }

    /// Validate a selection and report whether it would replace the current entry.
    pub fn selection_would_change(
        &self,
        target: &SpectrumTarget,
        channels: SpectrumChannels,
    ) -> Result<bool, SpectrumCaptureCollectionSelectionError> {
        let Some(index) = self
            .captures
            .iter()
            .position(|capture| capture.target == *target && capture.channels == channels)
        else {
            return Err(SpectrumCaptureCollectionSelectionError::UnknownEntry);
        };
        if self.selected == Some(index) {
            return Ok(false);
        }
        self.selection_epoch
            .checked_add(1)
            .ok_or(SpectrumCaptureCollectionSelectionError::EpochOverflow)?;
        Ok(true)
    }

    /// Start continuous capture on the currently selected entry.
    pub fn start_continuous(
        &mut self,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Result<SpectrumCadence, SpectrumContinuousCaptureError> {
        let Some(index) = self.selected else {
            return Err(SpectrumContinuousCaptureError::Busy);
        };
        let cadence = SpectrumCadence::new(sample_rate_hz, quantum_frames)
            .map_err(SpectrumContinuousCaptureError::Cadence)?;
        // `select` arms the entry so that the native replacement operation has the same semantics
        // for one-shot and managed callers. This replacement preflights cadence, ownership and
        // epoch before clearing that arm, preserving it when a reconfiguration is refused.
        self.captures[index].begin_continuous(cadence, true)?;
        Ok(cadence)
    }

    /// Stop continuous capture on the selected entry and leave the collection idle.
    pub fn stop_continuous(&mut self) {
        if let Some(index) = self.selected {
            self.captures[index].stop_continuous();
        }
    }

    /// Restart the selected entry at a fresh continuous capture epoch.
    pub fn restart_continuous(&mut self) -> Result<(), SpectrumContinuousCaptureError> {
        let Some(index) = self.selected else {
            return Err(SpectrumContinuousCaptureError::Busy);
        };
        self.captures[index].restart_continuous()
    }

    /// Return the selected entry's active managed cadence, if any.
    #[must_use]
    pub fn cadence(&self) -> Option<SpectrumCadence> {
        self.selected
            .and_then(|index| self.captures[index].cadence())
    }

    /// Return the selected entry's active managed epoch, if any.
    #[must_use]
    pub fn stream_epoch(&self) -> Option<u64> {
        self.selected
            .and_then(|index| self.captures[index].stream_epoch())
    }

    /// Read a completed one-shot window from the selected entry.
    pub fn try_read(&mut self) -> Result<SpectrumWindow, SpectrumCaptureCollectionReadError> {
        let Some(index) = self.selected else {
            return Err(SpectrumCaptureCollectionReadError::NoSelection);
        };
        self.captures[index]
            .try_read()
            .map_err(SpectrumCaptureCollectionReadError::Capture)
    }

    /// Read one completed continuous window from the selected entry.
    pub fn try_read_continuous(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        let Some(index) = self.selected else {
            return Err(SpectrumContinuousReadError::NotActive);
        };
        self.captures[index].try_read_continuous()
    }
}

/// Refusal returned by a control-side read operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumCaptureReadError {
    /// No capture is currently armed.
    NotArmed,
    /// The observer has not completed 2048 contiguous frames yet.
    Pending,
    /// The graph window was discontinuous, nonfinite, or otherwise unusable.
    Invalid,
}

/// Control-side refusal returned by an arm operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumCaptureError {
    /// Another capture is armed or its completed result has not been consumed.
    Busy,
}

/// The checked, block-aligned profile for a continuous spectrum capture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpectrumCadence {
    sample_rate_hz: u32,
    quantum_frames: u32,
    hop_frames: u32,
}

impl SpectrumCadence {
    /// Derive the launch-supported hop from one prepared render shape.
    pub fn new(sample_rate_hz: u32, quantum_frames: u32) -> Result<Self, SpectrumCadenceError> {
        if !engine::is_launch_sample_rate(engine::SampleRateHz(sample_rate_hz)) {
            return Err(SpectrumCadenceError::UnsupportedRate);
        }
        if quantum_frames == 0 {
            return Err(SpectrumCadenceError::ZeroQuantum);
        }
        let minimum_hop = (u64::from(sample_rate_hz) / 30).max(SPECTRUM_WINDOW_FRAMES as u64);
        let quantum = u64::from(quantum_frames);
        let quanta = minimum_hop
            .checked_add(quantum - 1)
            .ok_or(SpectrumCadenceError::HopOverflow)?
            / quantum;
        let hop_frames = quanta
            .checked_mul(quantum)
            .ok_or(SpectrumCadenceError::HopOverflow)?;
        Ok(Self {
            sample_rate_hz,
            quantum_frames,
            hop_frames: u32::try_from(hop_frames).map_err(|_| SpectrumCadenceError::HopOverflow)?,
        })
    }

    /// Sample rate fixed at preparation.
    #[must_use]
    pub const fn sample_rate_hz(self) -> u32 {
        self.sample_rate_hz
    }

    /// Render quantum fixed at preparation.
    #[must_use]
    pub const fn quantum_frames(self) -> u32 {
        self.quantum_frames
    }

    /// Non-overlapping hop between scheduled window starts.
    #[must_use]
    pub const fn hop_frames(self) -> u32 {
        self.hop_frames
    }
}

/// Refusal while deriving the fixed continuous capture profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumCadenceError {
    /// The sample rate is outside the four launch-supported rates.
    UnsupportedRate,
    /// A zero quantum cannot align capture starts.
    ZeroQuantum,
    /// Checked hop arithmetic overflowed or did not fit the profile.
    HopOverflow,
}

/// A completed scheduled window with its native stream history facts.
#[derive(Clone, Copy, Debug)]
pub struct SpectrumContinuousWindow {
    /// Captured left plane; zero-filled when only right was requested.
    pub left: [f32; SPECTRUM_WINDOW_FRAMES],
    /// Captured right plane; zero-filled when only left was requested.
    pub right: [f32; SPECTRUM_WINDOW_FRAMES],
    /// Absolute sample at the beginning of this scheduled window.
    pub first_sample: u64,
    /// Channel planes actually copied into this window.
    pub channels: SpectrumChannels,
    /// Whether a source underrun occurred in the window's graph blocks.
    pub source_underrun: bool,
    /// Stream history epoch; discontinuities begin a new epoch.
    pub stream_epoch: u64,
    /// Zero-based scheduled-window sequence within the epoch.
    pub sequence: u64,
    /// Number of complete scheduled windows dropped by the full queue in this epoch.
    pub dropped_captures: u64,
}

impl SpectrumContinuousWindow {
    /// Exclusive absolute sample at the end of this 2048-frame window.
    #[must_use]
    pub fn end_sample(&self) -> Option<u64> {
        self.first_sample
            .checked_add(u64::try_from(SPECTRUM_WINDOW_FRAMES).ok()?)
    }

    /// Adapt the captured samples to the existing one-shot analyzer input.
    #[must_use]
    pub fn as_window(&self) -> SpectrumWindow {
        SpectrumWindow {
            left: self.left,
            right: self.right,
            first_sample: self.first_sample,
            channels: self.channels,
            source_underrun: self.source_underrun,
        }
    }
}

/// Status returned while a continuous result is unavailable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumContinuousReadError {
    /// The one prepared stream is stopped.
    NotActive,
    /// Activation succeeded but no valid observed block has started the epoch.
    Warming,
    /// The stream is active and no completed window is queued.
    Pending,
    /// One or more scheduled windows were lost to the bounded queue.
    Gap {
        /// Epoch in which the loss occurred.
        stream_epoch: u64,
        /// Cumulative number dropped in that epoch.
        dropped_captures: u64,
    },
    /// A discontinuity, nonfinite input, or failed render reset history.
    Failed {
        /// New epoch after the failed history was discarded.
        stream_epoch: u64,
    },
}

/// Control-side continuous activation refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumContinuousCaptureError {
    /// The prepared stream is already active.
    Busy,
    /// The requested sample rate or quantum cannot define a launch profile.
    Cadence(SpectrumCadenceError),
    /// A new stream epoch cannot be represented.
    EpochOverflow,
}

struct SpectrumContinuousShared {
    active: AtomicU8,
    phase: AtomicU8,
    epoch: AtomicU64,
    failures: AtomicU64,
    failure_epoch: AtomicU64,
    drops: AtomicU64,
    drop_epoch: AtomicU64,
    sample_rate_hz: AtomicU64,
    quantum_frames: AtomicU64,
    hop_frames: AtomicU64,
    invalidated: AtomicU8,
    invalidated_epoch: AtomicU64,
}

impl SpectrumContinuousShared {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            active: AtomicU8::new(0),
            phase: AtomicU8::new(CONTINUOUS_WARMING),
            epoch: AtomicU64::new(0),
            failures: AtomicU64::new(0),
            failure_epoch: AtomicU64::new(0),
            drops: AtomicU64::new(0),
            drop_epoch: AtomicU64::new(0),
            sample_rate_hz: AtomicU64::new(0),
            quantum_frames: AtomicU64::new(0),
            hop_frames: AtomicU64::new(0),
            invalidated: AtomicU8::new(0),
            invalidated_epoch: AtomicU64::new(0),
        })
    }
}

#[repr(C)]
struct SpectrumContinuousSharedAllocation {
    strong: std::sync::atomic::AtomicUsize,
    weak: std::sync::atomic::AtomicUsize,
    shared: SpectrumContinuousShared,
}

struct SpectrumOneShotState {
    state: Arc<AtomicU8>,
    filled: usize,
    first_sample: u64,
    next_sample: u64,
    armed: bool,
    source_underrun: bool,
    completed_sample: Option<u64>,
}

struct SpectrumContinuousState {
    shared: Arc<SpectrumContinuousShared>,
    started_epoch: u64,
    expected_block_sample: Option<u64>,
    next_window_start: Option<u64>,
    first_sample: u64,
    filled: usize,
    sequence: u64,
    source_underrun: bool,
    completed_sample: Option<u64>,
    completed_sequence: Option<u64>,
}

/// Construct the graph binding and its separate control-side consumer.
pub(crate) fn prepare_capture(
    request: &SpectrumCaptureRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<(GraphNodeObserverBinding, SpectrumCapture), SpectrumPrepareError> {
    prepare_capture_with_handle(
        request,
        graph_nodes,
        maximum_named_allocation_bytes,
        SPECTRUM_OBSERVER_HANDLE,
        CaptureBindingPolicy::Permanent,
    )
}

#[derive(Clone, Copy)]
enum CaptureBindingPolicy {
    Permanent,
    Controlled,
}

fn prepare_capture_with_handle(
    request: &SpectrumCaptureRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
    observer_handle: u64,
    binding_policy: CaptureBindingPolicy,
) -> Result<(GraphNodeObserverBinding, SpectrumCapture), SpectrumPrepareError> {
    let (node, _resources) =
        validate_capture_request(request, graph_nodes, maximum_named_allocation_bytes)?;
    let (producer, consumer) = bounded_spsc(
        NonZeroUsize::new(1).ok_or(SpectrumPrepareError::QueueCapacity)?,
        QueueGeneration(0x5350_4543),
    )
    .map_err(|_| SpectrumPrepareError::QueueCapacity)?;
    let state = Arc::new(AtomicU8::new(IDLE));
    let mode = Arc::new(AtomicU8::new(ONE_SHOT_MODE));
    let shared = SpectrumContinuousShared::new();
    let capture = SpectrumCapture {
        consumer,
        state: Arc::clone(&state),
        mode: Arc::clone(&mode),
        shared: Arc::clone(&shared),
        seen_failures: 0,
        seen_drops: 0,
        target: request.target.clone(),
        channels: request.channels,
    };
    let observer = SpectrumCaptureObserver::new(producer, state, mode, shared, request.channels);
    let binding = match binding_policy {
        CaptureBindingPolicy::Permanent => {
            GraphNodeObserverBinding::new(node, observer_handle, Box::new(observer))
        }
        CaptureBindingPolicy::Controlled => {
            GraphNodeObserverBinding::controlled(node, observer_handle, Box::new(observer))
        }
    };
    Ok((binding, capture))
}

fn validate_capture_request(
    request: &SpectrumCaptureRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<(GraphNodeId, SpectrumCaptureResources), SpectrumPrepareError> {
    let node = request
        .target
        .graph_node()
        .filter(|node| graph_nodes.iter().any(|candidate| candidate == node))
        .ok_or(SpectrumPrepareError::UnknownTarget)?;
    if !request.channels.includes_left() && !request.channels.includes_right() {
        return Err(SpectrumPrepareError::NoChannels);
    }
    let resources = spectrum_capture_resources_for(&request.target);
    if request.maximum_capture_bytes < resources.retained_bytes {
        return Err(SpectrumPrepareError::CaptureBudget);
    }
    if maximum_named_allocation_bytes < resources.largest_allocation_bytes {
        return Err(SpectrumPrepareError::AllocationBudget);
    }
    Ok((node, resources))
}

/// Prepare every entry in one bounded collection and return its graph observer bindings.
pub(crate) fn prepare_capture_collection(
    request: &SpectrumCaptureCollectionRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<(Vec<GraphNodeObserverBinding>, SpectrumCaptureCollection), SpectrumPrepareError> {
    let resources = spectrum_capture_collection_resources(&request.entries)?;
    if !request.entries.is_empty() && request.maximum_capture_bytes < resources.retained_bytes {
        return Err(SpectrumPrepareError::CaptureBudget);
    }
    if maximum_named_allocation_bytes < resources.largest_allocation_bytes {
        return Err(SpectrumPrepareError::AllocationBudget);
    }
    let mut observers = Vec::new();
    observers
        .try_reserve_exact(request.entries.len())
        .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    let mut captures = Vec::new();
    captures
        .try_reserve_exact(request.entries.len())
        .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    for (index, entry) in request.entries.iter().enumerate() {
        let index = u64::try_from(index).map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
        let observer_handle = SPECTRUM_OBSERVER_HANDLE
            .checked_sub(index)
            .ok_or(SpectrumPrepareError::CollectionCapacity)?;
        let (observer, capture) = prepare_capture_with_handle(
            &SpectrumCaptureRequest {
                target: entry.target.clone(),
                channels: entry.channels,
                maximum_capture_bytes: request.maximum_capture_bytes,
            },
            graph_nodes,
            maximum_named_allocation_bytes,
            observer_handle,
            CaptureBindingPolicy::Permanent,
        )?;
        observers.push(observer);
        captures.push(capture);
    }
    Ok((observers, SpectrumCaptureCollection::new(captures)))
}

#[allow(dead_code)] // Paired slots are consumed by the controlled host path in C3/C4.
const CONTROLLED_SLOTS_PER_ENTRY: usize = 2;

/// One private paired slot for a host-controlled capture entry.
#[allow(dead_code)]
struct ControlledSpectrumSlot {
    capture: SpectrumCapture,
    handle: u64,
    staged: bool,
    admitted_generation: Option<u64>,
    applied_generation: Option<u64>,
    retiring_at_revision: Option<u64>,
}

#[allow(dead_code)]
impl ControlledSpectrumSlot {
    fn new(capture: SpectrumCapture, handle: u64) -> Self {
        Self {
            capture,
            handle,
            staged: false,
            admitted_generation: None,
            applied_generation: None,
            retiring_at_revision: None,
        }
    }

    fn is_free(&self) -> bool {
        !self.staged
            && self.admitted_generation.is_none()
            && self.applied_generation.is_none()
            && self.retiring_at_revision.is_none()
    }
}

/// The paired storage and lifecycle state for one exact public target/channel entry.
#[allow(dead_code)]
struct ControlledSpectrumCaptureEntry {
    slots: [ControlledSpectrumSlot; CONTROLLED_SLOTS_PER_ENTRY],
}

/// Copyable scalar identity used by the host owner when it prepares a publication snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ControlledSpectrumDescriptor {
    pub(crate) entry_index: usize,
    pub(crate) slot_index: usize,
    pub(crate) observer_handle: u64,
    pub(crate) mode: HostSpectrumMode,
}

/// An affine staged controlled capture candidate. It owns no render resources.
#[allow(dead_code)]
pub(crate) struct ControlledSpectrumCandidate {
    entry_index: usize,
    slot_index: usize,
    observer_handle: u64,
    mode: HostSpectrumMode,
    cadence: SpectrumCadence,
}

#[allow(dead_code)]
impl ControlledSpectrumCandidate {
    /// Return the internal controlled graph observer handle.
    #[must_use]
    pub(crate) const fn observer_handle(&self) -> u64 {
        self.observer_handle
    }

    /// Return the fixed scalar identity selected by staging.
    #[must_use]
    pub(crate) const fn descriptor(&self) -> ControlledSpectrumDescriptor {
        ControlledSpectrumDescriptor {
            entry_index: self.entry_index,
            slot_index: self.slot_index,
            observer_handle: self.observer_handle,
            mode: self.mode,
        }
    }
}

/// Host-controlled paired spectrum storage with at most one accepted active slot.
#[allow(dead_code)]
pub(crate) struct ControlledSpectrumCaptureCollection {
    entries: Box<[ControlledSpectrumCaptureEntry]>,
    touched: [usize; CONTROLLED_SLOTS_PER_ENTRY],
    touched_len: usize,
    accepted_slot: Option<usize>,
}

#[allow(dead_code)]
impl ControlledSpectrumCaptureCollection {
    fn new(entries: Vec<ControlledSpectrumCaptureEntry>) -> Self {
        Self {
            entries: entries.into_boxed_slice(),
            touched: [0; CONTROLLED_SLOTS_PER_ENTRY],
            touched_len: 0,
            accepted_slot: None,
        }
    }

    fn slot(&self, flat_slot: usize) -> &ControlledSpectrumSlot {
        let entry_index = flat_slot / CONTROLLED_SLOTS_PER_ENTRY;
        let slot_index = flat_slot % CONTROLLED_SLOTS_PER_ENTRY;
        &self.entries[entry_index].slots[slot_index]
    }

    fn slot_mut(&mut self, flat_slot: usize) -> &mut ControlledSpectrumSlot {
        let entry_index = flat_slot / CONTROLLED_SLOTS_PER_ENTRY;
        let slot_index = flat_slot % CONTROLLED_SLOTS_PER_ENTRY;
        &mut self.entries[entry_index].slots[slot_index]
    }

    fn flat_slot(entry_index: usize, slot_index: usize) -> Option<usize> {
        entry_index
            .checked_mul(CONTROLLED_SLOTS_PER_ENTRY)?
            .checked_add(slot_index)
    }

    fn touch(&mut self, flat_slot: usize) -> bool {
        if self.touched[..self.touched_len].contains(&flat_slot) {
            return true;
        }
        if self.touched_len == self.touched.len() {
            return false;
        }
        self.touched[self.touched_len] = flat_slot;
        self.touched_len += 1;
        true
    }

    fn target_entry(&self, target: &SpectrumTarget, channels: SpectrumChannels) -> Option<usize> {
        self.entries.iter().position(|entry| {
            let capture = &entry.slots[0].capture;
            capture.target() == target && capture.channels() == channels
        })
    }

    fn free_slot(&self, entry_index: usize) -> Option<usize> {
        (0..CONTROLLED_SLOTS_PER_ENTRY).find_map(|slot_index| {
            let flat_slot = Self::flat_slot(entry_index, slot_index)?;
            self.slot(flat_slot).is_free().then_some(flat_slot)
        })
    }

    /// Stage one exact prepared target on an inactive alternate slot.
    pub(crate) fn stage(
        &mut self,
        target: &SpectrumTarget,
        channels: SpectrumChannels,
        mode: HostSpectrumMode,
        cadence: SpectrumCadence,
    ) -> Result<ControlledSpectrumCandidate, SpectrumCaptureCollectionSelectionError> {
        let entry_index = self
            .target_entry(target, channels)
            .ok_or(SpectrumCaptureCollectionSelectionError::UnknownEntry)?;
        if self.touched[..self.touched_len]
            .iter()
            .copied()
            .any(|flat_slot| self.slot(flat_slot).staged)
        {
            return Err(SpectrumCaptureCollectionSelectionError::Busy);
        }
        let flat_slot = self
            .free_slot(entry_index)
            .ok_or(SpectrumCaptureCollectionSelectionError::Busy)?;
        if !self.touch(flat_slot) {
            return Err(SpectrumCaptureCollectionSelectionError::Busy);
        }
        let slot_index = flat_slot % CONTROLLED_SLOTS_PER_ENTRY;
        let observer_handle = self.slot(flat_slot).handle;
        self.slot_mut(flat_slot)
            .capture
            .reset_for_controlled_stage(mode, cadence);
        self.slot_mut(flat_slot).staged = true;
        Ok(ControlledSpectrumCandidate {
            entry_index,
            slot_index,
            observer_handle,
            mode,
            cadence,
        })
    }

    /// Publish a staged candidate's scalar admission metadata after graph publication succeeds.
    pub(crate) fn commit_candidate(
        &mut self,
        candidate: ControlledSpectrumCandidate,
        revision: u64,
    ) {
        let flat_slot = Self::flat_slot(candidate.entry_index, candidate.slot_index)
            .expect("controlled candidate index");
        assert!(
            self.slot(flat_slot).staged,
            "controlled candidate was not staged"
        );
        let previous = self.accepted_slot;
        {
            let slot = self.slot_mut(flat_slot);
            slot.staged = false;
            slot.admitted_generation = Some(revision);
            slot.applied_generation = None;
            slot.retiring_at_revision = None;
        }
        self.accepted_slot = Some(flat_slot);
        if let Some(previous) = previous.filter(|previous| *previous != flat_slot) {
            self.slot_mut(previous).retiring_at_revision = Some(revision);
            debug_assert!(self.touched[..self.touched_len].contains(&previous));
        }
    }

    /// Return a staged candidate to the free pool without changing accepted state.
    pub(crate) fn cancel_candidate(&mut self, candidate: ControlledSpectrumCandidate) {
        let flat_slot = Self::flat_slot(candidate.entry_index, candidate.slot_index)
            .expect("controlled candidate index");
        let slot = self.slot_mut(flat_slot);
        assert!(slot.staged, "controlled candidate was not staged");
        assert!(
            slot.admitted_generation.is_none(),
            "controlled candidate was already admitted"
        );
        slot.staged = false;
        slot.capture.retire_controlled_after_receipt();
        self.compact_touched();
    }

    /// Mark the last accepted slot for the reserved graph removal publication.
    pub(crate) fn commit_removal(&mut self, revision: u64) {
        let Some(accepted) = self.accepted_slot else {
            return;
        };
        let slot = self.slot_mut(accepted);
        if slot.retiring_at_revision.is_none() {
            slot.retiring_at_revision = Some(revision);
        }
    }

    /// Reconcile exactly one graph receipt and clean at most one item per retired slot.
    pub(crate) fn reconcile_applied(&mut self, revision: u64) {
        let mut index = 0;
        while index < self.touched_len {
            let flat_slot = self.touched[index];
            let apply = self.slot(flat_slot).admitted_generation == Some(revision);
            if apply {
                let slot = self.slot_mut(flat_slot);
                slot.admitted_generation = None;
                slot.applied_generation = Some(revision);
            }
            let retire = self.slot(flat_slot).retiring_at_revision == Some(revision);
            if retire {
                let slot = self.slot_mut(flat_slot);
                slot.capture.retire_controlled_after_receipt();
                slot.staged = false;
                slot.admitted_generation = None;
                slot.applied_generation = None;
                slot.retiring_at_revision = None;
                if self.accepted_slot == Some(flat_slot) {
                    self.accepted_slot = None;
                }
            }
            index += 1;
        }
        self.compact_touched();
    }

    fn compact_touched(&mut self) {
        let mut compacted = [0; CONTROLLED_SLOTS_PER_ENTRY];
        let mut compacted_len = 0;
        for &flat_slot in &self.touched[..self.touched_len] {
            let keep = {
                let slot = self.slot(flat_slot);
                slot.staged
                    || slot.admitted_generation.is_some()
                    || slot.applied_generation.is_some()
                    || slot.retiring_at_revision.is_some()
            };
            if keep {
                compacted[compacted_len] = flat_slot;
                compacted_len += 1;
            }
        }
        self.touched = compacted;
        self.touched_len = compacted_len;
    }
}

/// Prepare two controlled observer slots for every exact public entry.
pub(crate) fn prepare_controlled_capture_collection(
    request: &SpectrumCaptureCollectionRequest,
    graph_nodes: &[GraphNodeId],
    maximum_named_allocation_bytes: u64,
) -> Result<
    (
        Vec<GraphNodeObserverBinding>,
        ControlledSpectrumCaptureCollection,
    ),
    SpectrumPrepareError,
> {
    let resources = controlled_spectrum_capture_collection_resources(&request.entries)?;
    if !request.entries.is_empty() && request.maximum_capture_bytes < resources.retained_bytes {
        return Err(SpectrumPrepareError::CaptureBudget);
    }
    if maximum_named_allocation_bytes < resources.largest_allocation_bytes {
        return Err(SpectrumPrepareError::AllocationBudget);
    }
    let slot_count = request
        .entries
        .len()
        .checked_mul(CONTROLLED_SLOTS_PER_ENTRY)
        .ok_or(SpectrumPrepareError::CollectionCapacity)?;
    let slot_count_u64 =
        u64::try_from(slot_count).map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    if slot_count_u64 != 0 {
        let lowest_handle = SPECTRUM_OBSERVER_HANDLE
            .checked_sub(slot_count_u64 - 1)
            .ok_or(SpectrumPrepareError::CollectionCapacity)?;
        // Meter handles occupy the rising nonzero range and are checked against this high range
        // by combined host preparation. This helper can only prove its own handles are nonzero.
        if lowest_handle == 0 {
            return Err(SpectrumPrepareError::CollectionCapacity);
        }
    }

    // Complete every fallible target/channel/handle check before creating a binding.
    for entry in &request.entries {
        validate_capture_request(
            &SpectrumCaptureRequest {
                target: entry.target.clone(),
                channels: entry.channels,
                maximum_capture_bytes: request.maximum_capture_bytes,
            },
            graph_nodes,
            maximum_named_allocation_bytes,
        )?;
    }

    let mut observers = Vec::new();
    observers
        .try_reserve_exact(slot_count)
        .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    let mut entries = Vec::new();
    entries
        .try_reserve_exact(request.entries.len())
        .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
    for (entry_index, entry) in request.entries.iter().enumerate() {
        let mut slots = Vec::with_capacity(CONTROLLED_SLOTS_PER_ENTRY);
        for slot_index in 0..CONTROLLED_SLOTS_PER_ENTRY {
            let flat_slot = entry_index
                .checked_mul(CONTROLLED_SLOTS_PER_ENTRY)
                .and_then(|value| value.checked_add(slot_index))
                .ok_or(SpectrumPrepareError::CollectionCapacity)?;
            let flat_slot_u64 =
                u64::try_from(flat_slot).map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
            let observer_handle = SPECTRUM_OBSERVER_HANDLE
                .checked_sub(flat_slot_u64)
                .ok_or(SpectrumPrepareError::CollectionCapacity)?;
            let (observer, capture) = prepare_capture_with_handle(
                &SpectrumCaptureRequest {
                    target: entry.target.clone(),
                    channels: entry.channels,
                    maximum_capture_bytes: request.maximum_capture_bytes,
                },
                graph_nodes,
                maximum_named_allocation_bytes,
                observer_handle,
                CaptureBindingPolicy::Controlled,
            )?;
            observers.push(observer);
            slots.push(ControlledSpectrumSlot::new(capture, observer_handle));
        }
        let slots: [ControlledSpectrumSlot; CONTROLLED_SLOTS_PER_ENTRY] = slots
            .try_into()
            .map_err(|_| SpectrumPrepareError::CollectionCapacity)?;
        entries.push(ControlledSpectrumCaptureEntry { slots });
    }
    Ok((observers, ControlledSpectrumCaptureCollection::new(entries)))
}

struct SpectrumCaptureObserver {
    producer: Producer<SpectrumCapturedRecord>,
    left: [f32; SPECTRUM_WINDOW_FRAMES],
    right: [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    observation_generation: u64,
    mode: Arc<AtomicU8>,
    one_shot: SpectrumOneShotState,
    continuous: SpectrumContinuousState,
}

struct SpectrumCaptureBuffers<'a> {
    producer: &'a mut Producer<SpectrumCapturedRecord>,
    left: &'a mut [f32; SPECTRUM_WINDOW_FRAMES],
    right: &'a mut [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    observation_generation: u64,
}

impl SpectrumCaptureObserver {
    #[cfg(test)]
    fn new_one_shot_for_test(
        producer: Producer<SpectrumCapturedRecord>,
        state: Arc<AtomicU8>,
        channels: SpectrumChannels,
    ) -> Self {
        Self::new(
            producer,
            state,
            Arc::new(AtomicU8::new(ONE_SHOT_MODE)),
            SpectrumContinuousShared::new(),
            channels,
        )
    }

    fn new(
        producer: Producer<SpectrumCapturedRecord>,
        state: Arc<AtomicU8>,
        mode: Arc<AtomicU8>,
        shared: Arc<SpectrumContinuousShared>,
        channels: SpectrumChannels,
    ) -> Self {
        Self {
            producer,
            left: [0.0; SPECTRUM_WINDOW_FRAMES],
            right: [0.0; SPECTRUM_WINDOW_FRAMES],
            channels,
            observation_generation: 0,
            mode,
            one_shot: SpectrumOneShotState {
                state,
                filled: 0,
                first_sample: 0,
                next_sample: 0,
                armed: false,
                source_underrun: false,
                completed_sample: None,
            },
            continuous: SpectrumContinuousState {
                shared,
                started_epoch: 0,
                expected_block_sample: None,
                next_window_start: None,
                first_sample: 0,
                filled: 0,
                sequence: 0,
                source_underrun: false,
                completed_sample: None,
                completed_sequence: None,
            },
        }
    }

    // REALTIME_POLICY_BEGIN
    /// Apply a controlled activation boundary using only scalar state.
    ///
    /// The graph calls this hook between render blocks. It deliberately leaves the prepared PCM
    /// arrays and queue untouched: an old queued record must retain the generation that produced
    /// it, and all storage remains allocated at preparation.
    fn activation_changed(&mut self, active: bool, generation: u64, first_sample: u64) {
        if active {
            self.observation_generation = generation;
        }
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            self.reset_continuous_scalars(first_sample, active);
        } else {
            self.reset_one_shot_scalars(first_sample, active);
        }
    }

    fn reset_one_shot_scalars(&mut self, first_sample: u64, active: bool) {
        self.one_shot.filled = 0;
        self.one_shot.first_sample = first_sample;
        self.one_shot.next_sample = first_sample;
        self.one_shot.armed = active;
        self.one_shot.source_underrun = false;
        self.one_shot.completed_sample = None;
        self.one_shot
            .state
            .store(if active { ARMED } else { IDLE }, Ordering::Release);
    }

    fn reset_continuous_scalars(&mut self, first_sample: u64, active: bool) {
        let epoch = 1;
        self.continuous.started_epoch = epoch;
        self.continuous.expected_block_sample = None;
        self.continuous.next_window_start = None;
        self.continuous.first_sample = first_sample;
        self.continuous.filled = 0;
        self.continuous.sequence = 0;
        self.continuous.source_underrun = false;
        self.continuous.completed_sample = None;
        self.continuous.completed_sequence = None;
        self.continuous.shared.epoch.store(epoch, Ordering::Release);
        self.continuous.shared.failures.store(0, Ordering::Release);
        self.continuous
            .shared
            .failure_epoch
            .store(epoch, Ordering::Release);
        self.continuous.shared.drops.store(0, Ordering::Release);
        self.continuous
            .shared
            .drop_epoch
            .store(epoch, Ordering::Release);
        self.continuous
            .shared
            .invalidated
            .store(0, Ordering::Release);
        self.continuous
            .shared
            .invalidated_epoch
            .store(epoch, Ordering::Release);
        self.continuous.shared.phase.store(
            if active {
                CONTINUOUS_WARMING
            } else {
                CONTINUOUS_WAITING
            },
            Ordering::Release,
        );
        self.continuous
            .shared
            .active
            .store(u8::from(active), Ordering::Release);
    }

    fn capture(
        &mut self,
        left: &[f32],
        right: &[f32],
        first_sample: u64,
        validity: GraphObservationValidity,
    ) {
        let mut buffers = SpectrumCaptureBuffers {
            producer: &mut self.producer,
            left: &mut self.left,
            right: &mut self.right,
            channels: self.channels,
            observation_generation: self.observation_generation,
        };
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            continuous_capture(
                &mut self.continuous,
                &mut buffers,
                left,
                right,
                first_sample,
                validity,
            );
        } else {
            one_shot_capture(
                &mut self.one_shot,
                &mut buffers,
                left,
                right,
                first_sample,
                validity,
            );
        }
    }

    fn capture_resident(&mut self, block: GraphResidentObservationBlock<'_>) {
        let mut buffers = SpectrumCaptureBuffers {
            producer: &mut self.producer,
            left: &mut self.left,
            right: &mut self.right,
            channels: self.channels,
            observation_generation: self.observation_generation,
        };
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            continuous_capture_resident(&mut self.continuous, &mut buffers, block);
        } else {
            one_shot_capture_resident(&mut self.one_shot, &mut buffers, block);
        }
    }

    fn invalidate(&mut self) {
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            continuous_fail(&mut self.continuous);
        } else {
            one_shot_invalidate(&mut self.one_shot);
        }
    }
    // REALTIME_POLICY_END
}

fn one_shot_begin(
    state: &mut SpectrumOneShotState,
    first_sample: u64,
    frames: usize,
    validity: GraphObservationValidity,
) -> Option<usize> {
    let current = state.state.load(Ordering::Acquire);
    if current == ARMED {
        if state
            .state
            .compare_exchange(ARMED, CAPTURING, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return None;
        }
        state.filled = 0;
        state.first_sample = first_sample;
        state.next_sample = first_sample;
        state.armed = true;
        state.source_underrun = validity.source_underrun;
        state.completed_sample = None;
    } else if current == CAPTURING {
        if validity.source_generation_changed {
            one_shot_invalidate(state);
            return None;
        }
        state.source_underrun |= validity.source_underrun;
    }
    if state.state.load(Ordering::Acquire) != CAPTURING || !state.armed {
        return None;
    }
    if frames == 0 || first_sample != state.next_sample {
        one_shot_invalidate(state);
        return None;
    }
    if first_sample
        .checked_add(u64::try_from(frames).unwrap_or(0))
        .is_none()
    {
        one_shot_invalidate(state);
        return None;
    }
    let remaining = SPECTRUM_WINDOW_FRAMES - state.filled;
    Some(remaining.min(frames))
        .filter(|count| *count > 0)
        .or_else(|| {
            one_shot_invalidate(state);
            None
        })
}

fn one_shot_finish(
    state: &mut SpectrumOneShotState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    first_sample: u64,
    count: usize,
) {
    state.filled += count;
    state.next_sample = first_sample
        .checked_add(u64::try_from(count).unwrap_or(0))
        .unwrap_or(first_sample);
    if state.filled == SPECTRUM_WINDOW_FRAMES {
        probe_add(PROBE_PAYLOAD_CONSTRUCTIONS, 1);
        let window = SpectrumWindow {
            left: *buffers.left,
            right: *buffers.right,
            first_sample: state.first_sample,
            channels: buffers.channels,
            source_underrun: state.source_underrun,
        };
        let record = SpectrumCapturedRecord {
            window,
            observation_generation: buffers.observation_generation,
            stream_epoch: 0,
            sequence: 0,
            dropped_captures: 0,
        };
        probe_add(PROBE_PUBLICATION_ATTEMPTS, 1);
        if buffers.producer.try_push(record).is_ok() {
            state.completed_sample = Some(first_sample);
            state.state.store(COMPLETE, Ordering::Release);
        } else {
            one_shot_invalidate(state);
        }
        state.armed = false;
    }
}

fn one_shot_capture(
    state: &mut SpectrumOneShotState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    left: &[f32],
    right: &[f32],
    first_sample: u64,
    validity: GraphObservationValidity,
) {
    if left.len() != right.len() {
        if matches!(state.state.load(Ordering::Acquire), ARMED | CAPTURING) {
            one_shot_invalidate(state);
        }
        return;
    }
    let Some(count) = one_shot_begin(state, first_sample, left.len(), validity) else {
        return;
    };
    if (buffers.channels.includes_left() && !selected_is_finite(&left[..count]))
        || (buffers.channels.includes_right() && !selected_is_finite(&right[..count]))
    {
        one_shot_invalidate(state);
        return;
    }
    let destination = state.filled;
    if buffers.channels.includes_left() {
        buffers.left[destination..destination + count].copy_from_slice(&left[..count]);
        probe_add(PROBE_STORAGE_WRITES, count);
    }
    if buffers.channels.includes_right() {
        buffers.right[destination..destination + count].copy_from_slice(&right[..count]);
        probe_add(PROBE_STORAGE_WRITES, count);
    }
    one_shot_finish(state, buffers, first_sample, count);
}

fn one_shot_capture_resident(
    state: &mut SpectrumOneShotState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    block: GraphResidentObservationBlock<'_>,
) {
    let current = state.state.load(Ordering::Acquire);
    if current != ARMED && current != CAPTURING {
        return;
    }
    let lanes = block.lane.width().lanes() as usize;
    let lane = block.lane.lane();
    let Some(frames) = usize::try_from(block.lane.frames()).ok() else {
        one_shot_invalidate(state);
        return;
    };
    let Some(words) = frames.checked_mul(lanes) else {
        one_shot_invalidate(state);
        return;
    };
    let Some(left) = block.lane.left().get(..words) else {
        one_shot_invalidate(state);
        return;
    };
    let Some(right) = block.lane.right().get(..words) else {
        one_shot_invalidate(state);
        return;
    };
    let Some(count) = one_shot_begin(state, block.first_sample, frames, block.validity) else {
        return;
    };
    let destination = state.filled;
    for frame in 0..count {
        let Some(index) = frame
            .checked_mul(lanes)
            .and_then(|value| value.checked_add(lane))
        else {
            one_shot_invalidate(state);
            return;
        };
        if buffers.channels.includes_left() {
            let Some(value) = left.get(index).copied() else {
                one_shot_invalidate(state);
                return;
            };
            probe_add(PROBE_VALIDATION_SAMPLES, 1);
            if !value.is_finite() {
                one_shot_invalidate(state);
                return;
            }
            buffers.left[destination + frame] = value;
            probe_add(PROBE_STORAGE_WRITES, 1);
        }
        if buffers.channels.includes_right() {
            let Some(value) = right.get(index).copied() else {
                one_shot_invalidate(state);
                return;
            };
            probe_add(PROBE_VALIDATION_SAMPLES, 1);
            if !value.is_finite() {
                one_shot_invalidate(state);
                return;
            }
            buffers.right[destination + frame] = value;
            probe_add(PROBE_STORAGE_WRITES, 1);
        }
    }
    one_shot_finish(state, buffers, block.first_sample, count);
}

fn one_shot_invalidate(state: &mut SpectrumOneShotState) {
    if matches!(state.state.load(Ordering::Acquire), ARMED | CAPTURING) {
        state.armed = false;
        state.state.store(INVALID, Ordering::Release);
    }
}

fn one_shot_invalidate_after_failure(state: &mut SpectrumOneShotState, failed_sample: u64) {
    let current = state.state.load(Ordering::Acquire);
    if matches!(current, ARMED | CAPTURING) {
        one_shot_invalidate(state);
    } else if current == COMPLETE && state.completed_sample == Some(failed_sample) {
        state.armed = false;
        state.state.store(INVALID, Ordering::Release);
    }
}

fn continuous_begin(
    state: &mut SpectrumContinuousState,
    first_sample: u64,
    frames: usize,
    validity: GraphObservationValidity,
) -> Option<usize> {
    if state.shared.active.load(Ordering::Acquire) == 0 {
        return None;
    }
    let epoch = state.shared.epoch.load(Ordering::Acquire);
    if state.started_epoch != epoch {
        state.started_epoch = epoch;
        state.expected_block_sample = None;
        state.next_window_start = None;
        state.first_sample = 0;
        state.filled = 0;
        state.sequence = 0;
        state.source_underrun = false;
        state.completed_sample = None;
        state.completed_sequence = None;
    }
    let Some(end_sample) = first_sample.checked_add(u64::try_from(frames).ok()?) else {
        continuous_fail(state);
        return None;
    };
    if frames == 0 || validity.source_generation_changed {
        continuous_fail(state);
        return None;
    }
    if state
        .expected_block_sample
        .is_some_and(|expected| expected != first_sample)
    {
        continuous_fail(state);
        return None;
    }
    state.expected_block_sample = Some(end_sample);
    let phase = state.shared.phase.load(Ordering::Acquire);
    if phase == CONTINUOUS_CAPTURING {
        state.source_underrun |= validity.source_underrun;
        return Some((SPECTRUM_WINDOW_FRAMES - state.filled).min(frames));
    }
    let Some(window_start) = state.next_window_start else {
        state.next_window_start = Some(first_sample);
        state.first_sample = first_sample;
        state.filled = 0;
        state.source_underrun = validity.source_underrun;
        state
            .shared
            .phase
            .store(CONTINUOUS_CAPTURING, Ordering::Release);
        return Some(SPECTRUM_WINDOW_FRAMES.min(frames));
    };
    if first_sample < window_start {
        if phase == CONTINUOUS_WAITING {
            return None;
        }
        continuous_fail(state);
        return None;
    }
    if first_sample > window_start {
        continuous_fail(state);
        return None;
    }
    if phase != CONTINUOUS_CAPTURING {
        state.first_sample = first_sample;
        state.filled = 0;
        state.source_underrun = validity.source_underrun;
        state
            .shared
            .phase
            .store(CONTINUOUS_CAPTURING, Ordering::Release);
    } else {
        state.source_underrun |= validity.source_underrun;
    }
    Some((SPECTRUM_WINDOW_FRAMES - state.filled).min(frames))
}

fn continuous_finish(
    state: &mut SpectrumContinuousState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    first_sample: u64,
    count: usize,
) {
    state.filled += count;
    if state.filled != SPECTRUM_WINDOW_FRAMES {
        return;
    }
    let stream_epoch = state.shared.epoch.load(Ordering::Acquire);
    let sequence = state.sequence;
    probe_add(PROBE_PAYLOAD_CONSTRUCTIONS, 1);
    let record = SpectrumCapturedRecord {
        window: SpectrumWindow {
            left: *buffers.left,
            right: *buffers.right,
            first_sample: state.first_sample,
            channels: buffers.channels,
            source_underrun: state.source_underrun,
        },
        observation_generation: buffers.observation_generation,
        stream_epoch,
        sequence,
        dropped_captures: state.shared.drops.load(Ordering::Acquire),
    };
    probe_add(PROBE_PUBLICATION_ATTEMPTS, 1);
    if buffers.producer.try_push(record).is_err() {
        if state
            .shared
            .drops
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
                value.checked_add(1)
            })
            .is_err()
        {
            continuous_fail(state);
            return;
        }
        state
            .shared
            .drop_epoch
            .store(stream_epoch, Ordering::Release);
    }
    state.completed_sample = Some(first_sample);
    state.completed_sequence = Some(sequence);
    let Some(next_sequence) = sequence.checked_add(1) else {
        continuous_fail(state);
        return;
    };
    let Some(hop_frames) = u32::try_from(state.shared.hop_frames.load(Ordering::Acquire)).ok()
    else {
        continuous_fail(state);
        return;
    };
    let Some(next_window_start) = state.first_sample.checked_add(u64::from(hop_frames)) else {
        continuous_fail(state);
        return;
    };
    state.sequence = next_sequence;
    state.next_window_start = Some(next_window_start);
    state.filled = 0;
    state.source_underrun = false;
    state
        .shared
        .phase
        .store(CONTINUOUS_WAITING, Ordering::Release);
}

fn continuous_capture(
    state: &mut SpectrumContinuousState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    left: &[f32],
    right: &[f32],
    first_sample: u64,
    validity: GraphObservationValidity,
) {
    if state.shared.active.load(Ordering::Acquire) == 0 {
        return;
    }
    if left.len() != right.len() {
        continuous_fail(state);
        return;
    }
    if (buffers.channels.includes_left() && !selected_is_finite(left))
        || (buffers.channels.includes_right() && !selected_is_finite(right))
    {
        continuous_fail(state);
        return;
    }
    let Some(count) = continuous_begin(state, first_sample, left.len(), validity) else {
        return;
    };
    if count == 0 {
        return;
    }
    let destination = state.filled;
    if buffers.channels.includes_left() {
        buffers.left[destination..destination + count].copy_from_slice(&left[..count]);
        probe_add(PROBE_STORAGE_WRITES, count);
    }
    if buffers.channels.includes_right() {
        buffers.right[destination..destination + count].copy_from_slice(&right[..count]);
        probe_add(PROBE_STORAGE_WRITES, count);
    }
    continuous_finish(state, buffers, first_sample, count);
}

fn continuous_capture_resident(
    state: &mut SpectrumContinuousState,
    buffers: &mut SpectrumCaptureBuffers<'_>,
    block: GraphResidentObservationBlock<'_>,
) {
    if state.shared.active.load(Ordering::Acquire) == 0 {
        return;
    }
    let lanes = block.lane.width().lanes() as usize;
    let lane = block.lane.lane();
    let Some(frames) = usize::try_from(block.lane.frames()).ok() else {
        continuous_fail(state);
        return;
    };
    let Some(words) = frames.checked_mul(lanes) else {
        continuous_fail(state);
        return;
    };
    let Some(left) = block.lane.left().get(..words) else {
        continuous_fail(state);
        return;
    };
    let Some(right) = block.lane.right().get(..words) else {
        continuous_fail(state);
        return;
    };
    if !resident_selected_is_finite(left, right, frames, lanes, lane, buffers.channels) {
        continuous_fail(state);
        return;
    }
    let Some(count) = continuous_begin(state, block.first_sample, frames, block.validity) else {
        return;
    };
    let destination = state.filled;
    for frame in 0..count {
        let Some(index) = frame
            .checked_mul(lanes)
            .and_then(|value| value.checked_add(lane))
        else {
            continuous_fail(state);
            return;
        };
        if buffers.channels.includes_left() {
            let Some(value) = left.get(index).copied() else {
                continuous_fail(state);
                return;
            };
            buffers.left[destination + frame] = value;
            probe_add(PROBE_STORAGE_WRITES, 1);
        }
        if buffers.channels.includes_right() {
            let Some(value) = right.get(index).copied() else {
                continuous_fail(state);
                return;
            };
            buffers.right[destination + frame] = value;
            probe_add(PROBE_STORAGE_WRITES, 1);
        }
    }
    if count != 0 {
        continuous_finish(state, buffers, block.first_sample, count);
    }
}

fn selected_is_finite(values: &[f32]) -> bool {
    for value in values {
        probe_add(PROBE_VALIDATION_SAMPLES, 1);
        if !value.is_finite() {
            return false;
        }
    }
    true
}

fn resident_selected_is_finite(
    left: &[f32],
    right: &[f32],
    frames: usize,
    lanes: usize,
    lane: usize,
    channels: SpectrumChannels,
) -> bool {
    for frame in 0..frames {
        let Some(index) = frame
            .checked_mul(lanes)
            .and_then(|value| value.checked_add(lane))
        else {
            return false;
        };
        if channels.includes_left() {
            probe_add(PROBE_VALIDATION_SAMPLES, 1);
            if left.get(index).is_none_or(|value| !value.is_finite()) {
                return false;
            }
        }
        if channels.includes_right() {
            probe_add(PROBE_VALIDATION_SAMPLES, 1);
            if right.get(index).is_none_or(|value| !value.is_finite()) {
                return false;
            }
        }
    }
    true
}

fn continuous_fail(state: &mut SpectrumContinuousState) {
    if state.shared.active.load(Ordering::Acquire) == 0 {
        return;
    }
    let current_epoch = state.shared.epoch.load(Ordering::Acquire);
    state
        .shared
        .invalidated_epoch
        .store(current_epoch, Ordering::Release);
    state.shared.invalidated.store(1, Ordering::Release);
    let Some(next_epoch) = current_epoch.checked_add(1) else {
        state.shared.active.store(0, Ordering::Release);
        return;
    };
    state.shared.epoch.store(next_epoch, Ordering::Release);
    if state
        .shared
        .failures
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |value| {
            value.checked_add(1)
        })
        .is_err()
    {
        state.shared.active.store(0, Ordering::Release);
        return;
    }
    state
        .shared
        .failure_epoch
        .store(next_epoch, Ordering::Release);
    state.shared.drops.store(0, Ordering::Release);
    state.shared.drop_epoch.store(next_epoch, Ordering::Release);
    state.started_epoch = next_epoch;
    state.expected_block_sample = None;
    state.next_window_start = None;
    state.first_sample = 0;
    state.filled = 0;
    state.sequence = 0;
    state.source_underrun = false;
    state.completed_sample = None;
    state.completed_sequence = None;
    state
        .shared
        .phase
        .store(CONTINUOUS_WARMING, Ordering::Release);
}

impl GraphRuntimeObserver for SpectrumCaptureObserver {
    fn activation_changed(&mut self, active: bool, generation: u64, first_sample: u64) {
        SpectrumCaptureObserver::activation_changed(self, active, generation, first_sample);
    }

    fn observe(&mut self, block: GraphObservationBlock<'_>) -> Result<(), RenderError> {
        self.capture(
            block.left,
            block.right,
            block.first_sample,
            GraphObservationValidity::CLEAR,
        );
        Ok(())
    }

    fn observe_with_validity(
        &mut self,
        block: GraphObservationBlock<'_>,
        validity: GraphObservationValidity,
    ) -> Result<(), RenderError> {
        self.capture(block.left, block.right, block.first_sample, validity);
        Ok(())
    }

    fn observe_resident(
        &mut self,
        block: GraphResidentObservationBlock<'_>,
    ) -> Option<Result<(), RenderError>> {
        self.capture_resident(block);
        Some(Ok(()))
    }

    fn invalidate(&mut self) {
        self.invalidate();
    }

    fn invalidate_after_failure(&mut self, failed_sample: u64) {
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            continuous_fail(&mut self.continuous);
        } else {
            one_shot_invalidate_after_failure(&mut self.one_shot, failed_sample);
        }
    }
}

/// Caller-owned output planes for one analyzed spectrum.
pub struct SpectrumOutput<'a> {
    /// Frequency axis in hertz, from DC through Nyquist.
    pub frequencies_hz: &'a mut [f32],
    /// Optional left-channel amplitude in dBFS. It must be present exactly when the window
    /// selected the left plane.
    pub left_dbfs: Option<&'a mut [f32]>,
    /// Optional right-channel amplitude in dBFS. It must be present exactly when the window
    /// selected the right plane.
    pub right_dbfs: Option<&'a mut [f32]>,
}

const SPECTRUM_DEFAULT_SMOOTHING_MS: f64 = 100.0;
const SPECTRUM_MAX_SMOOTHING_MS: f64 = 10_000.0;

/// The bounded smoothing requested by a managed spectrum analyzer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpectrumSmoothingConfig {
    smoothing_ms: f64,
}

impl SpectrumSmoothingConfig {
    /// The normalized service default, in milliseconds.
    pub const DEFAULT_SMOOTHING_MS: f64 = SPECTRUM_DEFAULT_SMOOTHING_MS;

    /// Validate and normalize one smoothing duration.
    pub fn new(smoothing_ms: f64) -> Result<Self, SpectrumSmoothingConfigError> {
        if !smoothing_ms.is_finite() || !(0.0..=SPECTRUM_MAX_SMOOTHING_MS).contains(&smoothing_ms) {
            return Err(SpectrumSmoothingConfigError::OutOfRange);
        }
        Ok(Self { smoothing_ms })
    }

    /// Return the normalized duration in milliseconds.
    #[must_use]
    pub const fn smoothing_ms(self) -> f64 {
        self.smoothing_ms
    }
}

impl Default for SpectrumSmoothingConfig {
    fn default() -> Self {
        Self {
            smoothing_ms: SPECTRUM_DEFAULT_SMOOTHING_MS,
        }
    }
}

/// Refusal for a smoothing duration outside the bounded service range.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumSmoothingConfigError {
    /// Smoothing must be finite and within 0 through 10,000 milliseconds.
    OutOfRange,
}

/// Stateful worker-side power history for one continuous analyzer.
///
/// The history is deliberately bounded to the two 1025-bin channel planes.  It is only mutated
/// after a complete output has been validated, and it is never reachable from graph render.
#[derive(Clone, Debug)]
pub struct SpectrumAnalysisHistory {
    left_power: [f64; SPECTRUM_BIN_COUNT],
    right_power: [f64; SPECTRUM_BIN_COUNT],
    initialized: bool,
    analysis_epoch: u64,
    history_start_sample: Option<u64>,
    stream_epoch: Option<u64>,
    sequence: Option<u64>,
    configuration: Option<SpectrumAnalysisConfiguration>,
}

impl SpectrumAnalysisHistory {
    /// Create an empty history that will initialize on the first valid window.
    #[must_use]
    pub fn new() -> Self {
        Self {
            left_power: [0.0; SPECTRUM_BIN_COUNT],
            right_power: [0.0; SPECTRUM_BIN_COUNT],
            initialized: false,
            analysis_epoch: 0,
            history_start_sample: None,
            stream_epoch: None,
            sequence: None,
            configuration: None,
        }
    }

    /// Reset history and advance its analysis epoch.
    pub fn reset(&mut self) -> Result<(), SpectrumAnalysisError> {
        let Some(next_epoch) = self.analysis_epoch.checked_add(1) else {
            self.clear();
            return Err(SpectrumAnalysisError::Numerical);
        };
        self.clear();
        self.analysis_epoch = next_epoch;
        Ok(())
    }

    /// The epoch of the currently retained smoothing history.
    #[must_use]
    pub const fn analysis_epoch(&self) -> u64 {
        self.analysis_epoch
    }

    /// The first sample represented by the currently retained history, if initialized.
    #[must_use]
    pub const fn history_start_sample(&self) -> Option<u64> {
        self.history_start_sample
    }

    fn clear(&mut self) {
        self.left_power = [0.0; SPECTRUM_BIN_COUNT];
        self.right_power = [0.0; SPECTRUM_BIN_COUNT];
        self.initialized = false;
        self.history_start_sample = None;
        self.stream_epoch = None;
        self.sequence = None;
        self.configuration = None;
    }
}

impl Default for SpectrumAnalysisHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SpectrumAnalysisConfiguration {
    sample_rate_hz: u32,
    hop_frames: u32,
    channels: SpectrumChannels,
    smoothing_ms: f64,
}

/// Metadata for one successfully analyzed continuous window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpectrumAnalysisMetadata {
    /// Analysis-history epoch, which changes after a gap, capture epoch or configuration reset.
    pub analysis_epoch: u64,
    /// First absolute sample retained by this analysis history.
    pub history_start_sample: u64,
    /// First absolute sample in the latest FFT window.
    pub fft_first_sample: u64,
    /// Exclusive absolute sample at the end of the latest FFT window.
    pub fft_end_sample: u64,
    /// Native capture epoch of the latest window.
    pub stream_epoch: u64,
    /// Scheduled-window sequence of the latest window.
    pub sequence: u64,
    /// Effective smoothing duration used for this result.
    pub smoothing_ms: f64,
    /// Whether the source graph reported an underrun during this window.
    pub source_underrun: bool,
}

/// A portable, allocation-free analyzer for one captured window.
pub struct SpectrumAnalyzer {
    weights: [f32; SPECTRUM_WINDOW_FRAMES],
    weight_sum: f64,
}

impl SpectrumAnalyzer {
    /// Prepare periodic-Hann weights once on the control/worker side.
    #[must_use]
    pub fn new() -> Self {
        let mut weights = [0.0; SPECTRUM_WINDOW_FRAMES];
        let mut weight_sum = 0.0;
        for (index, weight) in weights.iter_mut().enumerate() {
            let phase =
                (2.0 * core::f64::consts::PI * index as f64) / SPECTRUM_WINDOW_FRAMES as f64;
            let value = 0.5 - 0.5 * math::cos(phase);
            *weight = value as f32;
            weight_sum += f64::from(*weight);
        }
        Self {
            weights,
            weight_sum,
        }
    }

    /// Analyze one immutable window into caller-provided output arrays.
    pub fn analyze(
        &self,
        window: &SpectrumWindow,
        sample_rate_hz: u32,
        output: &mut SpectrumOutput<'_>,
    ) -> Result<(), SpectrumAnalysisError> {
        let mut scratch = SpectrumAnalysisScratch::default();
        let (left_requested, right_requested) =
            self.prepare_analysis(window, sample_rate_hz, output, &mut scratch)?;
        let mut left_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        for bin in 0..SPECTRUM_BIN_COUNT {
            if left_requested {
                left_dbfs[bin] = amplitude_db(scratch.left_amplitudes[bin])?;
            }
            if right_requested {
                right_dbfs[bin] = amplitude_db(scratch.right_amplitudes[bin])?;
            }
        }
        output.frequencies_hz.copy_from_slice(&scratch.frequencies);
        if let Some(values) = output.left_dbfs.as_deref_mut() {
            values.copy_from_slice(&left_dbfs);
        }
        if let Some(values) = output.right_dbfs.as_deref_mut() {
            values.copy_from_slice(&right_dbfs);
        }
        Ok(())
    }

    /// Analyze a continuous window and apply bounded off-render power smoothing.
    pub fn analyze_continuous(
        &self,
        window: &SpectrumContinuousWindow,
        cadence: SpectrumCadence,
        smoothing: SpectrumSmoothingConfig,
        history: &mut SpectrumAnalysisHistory,
        output: &mut SpectrumOutput<'_>,
    ) -> Result<SpectrumAnalysisMetadata, SpectrumAnalysisError> {
        let spectrum_window = window.as_window();
        let mut scratch = SpectrumAnalysisScratch::default();
        let (left_requested, right_requested) = match self.prepare_analysis(
            &spectrum_window,
            cadence.sample_rate_hz,
            output,
            &mut scratch,
        ) {
            Ok(requested) => requested,
            Err(error) => {
                if matches!(
                    error,
                    SpectrumAnalysisError::InvalidWindow | SpectrumAnalysisError::Numerical
                ) {
                    let _ = history.reset();
                }
                return Err(error);
            }
        };
        let fft_end_sample = match spectrum_window.end_sample() {
            Some(end_sample) => end_sample,
            None => {
                let _ = history.reset();
                return Err(SpectrumAnalysisError::InvalidWindow);
            }
        };
        let configuration = SpectrumAnalysisConfiguration {
            sample_rate_hz: cadence.sample_rate_hz,
            hop_frames: cadence.hop_frames,
            channels: window.channels,
            smoothing_ms: smoothing.smoothing_ms,
        };
        if let Some(previous) = history.configuration {
            let sequence_contiguous = history
                .sequence
                .and_then(|sequence| sequence.checked_add(1))
                == Some(window.sequence);
            if (previous != configuration
                || history.stream_epoch != Some(window.stream_epoch)
                || !sequence_contiguous)
                && history.reset().is_err()
            {
                return Err(SpectrumAnalysisError::Numerical);
            }
        }

        let smoothing_factor = if smoothing.smoothing_ms == 0.0 {
            0.0
        } else {
            let tau_seconds = smoothing.smoothing_ms / 1_000.0;
            let exponent =
                -f64::from(cadence.hop_frames) / (f64::from(cadence.sample_rate_hz) * tau_seconds);
            let factor = math::exp(exponent);
            if !factor.is_finite() || !(0.0..=1.0).contains(&factor) {
                let _ = history.reset();
                return Err(SpectrumAnalysisError::Numerical);
            }
            factor
        };

        let mut next_left_power = history.left_power;
        let mut next_right_power = history.right_power;
        let mut left_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        for bin in 0..SPECTRUM_BIN_COUNT {
            if left_requested {
                let power = scratch.left_amplitudes[bin] * scratch.left_amplitudes[bin];
                let smoothed_power = if smoothing.smoothing_ms == 0.0 || !history.initialized {
                    power
                } else {
                    smoothing_factor * history.left_power[bin] + (1.0 - smoothing_factor) * power
                };
                if !smoothed_power.is_finite() || smoothed_power < 0.0 {
                    let _ = history.reset();
                    return Err(SpectrumAnalysisError::Numerical);
                }
                next_left_power[bin] = smoothed_power;
                match power_db(smoothed_power) {
                    Ok(value) => left_dbfs[bin] = value,
                    Err(error) => {
                        let _ = history.reset();
                        return Err(error);
                    }
                }
            }
            if right_requested {
                let power = scratch.right_amplitudes[bin] * scratch.right_amplitudes[bin];
                let smoothed_power = if smoothing.smoothing_ms == 0.0 || !history.initialized {
                    power
                } else {
                    smoothing_factor * history.right_power[bin] + (1.0 - smoothing_factor) * power
                };
                if !smoothed_power.is_finite() || smoothed_power < 0.0 {
                    let _ = history.reset();
                    return Err(SpectrumAnalysisError::Numerical);
                }
                next_right_power[bin] = smoothed_power;
                match power_db(smoothed_power) {
                    Ok(value) => right_dbfs[bin] = value,
                    Err(error) => {
                        let _ = history.reset();
                        return Err(error);
                    }
                }
            }
        }

        output.frequencies_hz.copy_from_slice(&scratch.frequencies);
        if let Some(values) = output.left_dbfs.as_deref_mut() {
            values.copy_from_slice(&left_dbfs);
        }
        if let Some(values) = output.right_dbfs.as_deref_mut() {
            values.copy_from_slice(&right_dbfs);
        }

        let history_start_sample = history
            .history_start_sample
            .unwrap_or(spectrum_window.first_sample);
        history.left_power = next_left_power;
        history.right_power = next_right_power;
        history.initialized = true;
        history.history_start_sample = Some(history_start_sample);
        history.stream_epoch = Some(window.stream_epoch);
        history.sequence = Some(window.sequence);
        history.configuration = Some(configuration);
        Ok(SpectrumAnalysisMetadata {
            analysis_epoch: history.analysis_epoch,
            history_start_sample,
            fft_first_sample: spectrum_window.first_sample,
            fft_end_sample,
            stream_epoch: window.stream_epoch,
            sequence: window.sequence,
            smoothing_ms: smoothing.smoothing_ms,
            source_underrun: spectrum_window.source_underrun,
        })
    }

    fn prepare_analysis(
        &self,
        window: &SpectrumWindow,
        sample_rate_hz: u32,
        output: &SpectrumOutput<'_>,
        scratch: &mut SpectrumAnalysisScratch,
    ) -> Result<(bool, bool), SpectrumAnalysisError> {
        let (left_requested, right_requested) =
            validate_analysis_request(window, sample_rate_hz, output)?;
        let mut left = [0.0; SPECTRUM_WINDOW_FRAMES];
        let mut right = [0.0; SPECTRUM_WINDOW_FRAMES];
        for index in 0..SPECTRUM_WINDOW_FRAMES {
            if left_requested {
                left[index] = window.left[index] * self.weights[index];
            }
            if right_requested {
                right[index] = window.right[index] * self.weights[index];
            }
        }
        let left_fft = left_requested.then(|| microfft::real::rfft_2048(&mut left));
        let right_fft = right_requested.then(|| microfft::real::rfft_2048(&mut right));
        for bin in 0..SPECTRUM_BIN_COUNT {
            scratch.frequencies[bin] =
                (bin as f64 * sample_rate_hz as f64 / SPECTRUM_WINDOW_FRAMES as f64) as f32;
            let factor = if bin == 0 || bin == SPECTRUM_WINDOW_FRAMES / 2 {
                1.0
            } else {
                2.0
            };
            if let Some(left_fft) = left_fft.as_ref() {
                let amplitude = fft_magnitude(left_fft, bin) * factor / self.weight_sum;
                if !amplitude.is_finite() || amplitude < 0.0 {
                    return Err(SpectrumAnalysisError::Numerical);
                }
                scratch.left_amplitudes[bin] = amplitude;
            }
            if let Some(right_fft) = right_fft.as_ref() {
                let amplitude = fft_magnitude(right_fft, bin) * factor / self.weight_sum;
                if !amplitude.is_finite() || amplitude < 0.0 {
                    return Err(SpectrumAnalysisError::Numerical);
                }
                scratch.right_amplitudes[bin] = amplitude;
            }
        }
        Ok((left_requested, right_requested))
    }
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

fn amplitude_db(amplitude: f64) -> Result<f32, SpectrumAnalysisError> {
    if !amplitude.is_finite() || amplitude < 0.0 {
        return Err(SpectrumAnalysisError::Numerical);
    }
    if amplitude == 0.0 {
        return Ok(SPECTRUM_FLOOR_DB);
    }
    let db = 20.0 * math::log10(amplitude);
    if !db.is_finite() {
        return Err(SpectrumAnalysisError::Numerical);
    }
    Ok((db.max(f64::from(SPECTRUM_FLOOR_DB))) as f32)
}

fn power_db(power: f64) -> Result<f32, SpectrumAnalysisError> {
    if !power.is_finite() || power < 0.0 {
        return Err(SpectrumAnalysisError::Numerical);
    }
    if power == 0.0 {
        return Ok(SPECTRUM_FLOOR_DB);
    }
    let db = 10.0 * math::log10(power);
    if !db.is_finite() {
        return Err(SpectrumAnalysisError::Numerical);
    }
    Ok((db.max(f64::from(SPECTRUM_FLOOR_DB))) as f32)
}

#[derive(Clone, Copy)]
struct SpectrumAnalysisScratch {
    frequencies: [f32; SPECTRUM_BIN_COUNT],
    left_amplitudes: [f64; SPECTRUM_BIN_COUNT],
    right_amplitudes: [f64; SPECTRUM_BIN_COUNT],
}

impl Default for SpectrumAnalysisScratch {
    fn default() -> Self {
        Self {
            frequencies: [0.0; SPECTRUM_BIN_COUNT],
            left_amplitudes: [0.0; SPECTRUM_BIN_COUNT],
            right_amplitudes: [0.0; SPECTRUM_BIN_COUNT],
        }
    }
}

fn validate_analysis_request(
    window: &SpectrumWindow,
    sample_rate_hz: u32,
    output: &SpectrumOutput<'_>,
) -> Result<(bool, bool), SpectrumAnalysisError> {
    if !engine::is_launch_sample_rate(engine::SampleRateHz(sample_rate_hz)) {
        return Err(SpectrumAnalysisError::UnsupportedRate);
    }
    let left_requested = window.channels.includes_left();
    let right_requested = window.channels.includes_right();
    if output.frequencies_hz.len() != SPECTRUM_BIN_COUNT
        || output
            .left_dbfs
            .as_ref()
            .is_some_and(|values| values.len() != SPECTRUM_BIN_COUNT)
        || output
            .right_dbfs
            .as_ref()
            .is_some_and(|values| values.len() != SPECTRUM_BIN_COUNT)
        || output.left_dbfs.is_some() != left_requested
        || output.right_dbfs.is_some() != right_requested
    {
        return Err(SpectrumAnalysisError::OutputShape);
    }
    if window.end_sample().is_none()
        || window
            .left
            .iter()
            .chain(window.right.iter())
            .any(|value| !value.is_finite())
    {
        return Err(SpectrumAnalysisError::InvalidWindow);
    }
    Ok((left_requested, right_requested))
}

fn fft_magnitude(fft: &[microfft::Complex32; SPECTRUM_WINDOW_FRAMES / 2], bin: usize) -> f64 {
    if bin == 0 {
        f64::from(fft[0].re.abs())
    } else if bin == SPECTRUM_WINDOW_FRAMES / 2 {
        f64::from(fft[0].im.abs())
    } else {
        let value = fft[bin];
        let real = f64::from(value.re);
        let imaginary = f64::from(value.im);
        math::sqrt(real * real + imaginary * imaginary)
    }
}

/// Analysis refusal for malformed input, output, or numerical state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumAnalysisError {
    /// The selected session rate is outside the four launch rates.
    UnsupportedRate,
    /// A caller output plane is not exactly 1025 bins.
    OutputShape,
    /// A captured window is invalid or contains a nonfinite sample.
    InvalidWindow,
    /// The transform or dB conversion produced a nonfinite value.
    Numerical,
}

#[cfg(test)]
mod tests {
    use super::{
        ARMED, CAPTURING, COMPLETE, INVALID, SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB,
        SPECTRUM_WINDOW_FRAMES, SpectrumAnalysisError, SpectrumAnalysisHistory, SpectrumAnalyzer,
        SpectrumCadence, SpectrumCapture, SpectrumCaptureCollectionEntry,
        SpectrumCaptureCollectionRequest, SpectrumCaptureObserver, SpectrumCaptureReadError,
        SpectrumChannels, SpectrumContinuousReadError, SpectrumContinuousWindow,
        SpectrumSmoothingConfig, SpectrumSmoothingConfigError, SpectrumTarget, SpectrumWindow,
    };
    use crate::observation_demand::HostSpectrumMode;
    use dsp_reference::{Complex64, direct_dft_bin, magnitude_db};
    use engine::realtime::{QueueGeneration, bounded_spsc};
    use graph::GraphRuntimeObserver;

    fn window(fill: f32) -> SpectrumWindow {
        SpectrumWindow {
            left: [fill; SPECTRUM_WINDOW_FRAMES],
            right: [fill; SPECTRUM_WINDOW_FRAMES],
            first_sample: 17,
            channels: SpectrumChannels::Stereo,
            source_underrun: false,
        }
    }

    fn continuous_window(
        fill: f32,
        first_sample: u64,
        stream_epoch: u64,
        sequence: u64,
    ) -> SpectrumContinuousWindow {
        let base = window(fill);
        SpectrumContinuousWindow {
            left: base.left,
            right: base.right,
            first_sample,
            channels: base.channels,
            source_underrun: false,
            stream_epoch,
            sequence,
            dropped_captures: 0,
        }
    }

    fn continuous_pair(channels: SpectrumChannels) -> (SpectrumCaptureObserver, SpectrumCapture) {
        let (producer, consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x4353_5045),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(super::IDLE));
        let mode = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(super::ONE_SHOT_MODE));
        let shared = super::SpectrumContinuousShared::new();
        let observer = SpectrumCaptureObserver::new(
            producer,
            std::sync::Arc::clone(&state),
            std::sync::Arc::clone(&mode),
            std::sync::Arc::clone(&shared),
            channels,
        );
        let capture = SpectrumCapture {
            consumer,
            state,
            mode,
            shared,
            seen_failures: 0,
            seen_drops: 0,
            target: SpectrumTarget::Output("main-out".into()),
            channels,
        };
        (observer, capture)
    }

    fn controlled_entries() -> Vec<SpectrumCaptureCollectionEntry> {
        vec![
            SpectrumCaptureCollectionEntry {
                target: SpectrumTarget::Output("main-out".into()),
                channels: SpectrumChannels::Stereo,
            },
            SpectrumCaptureCollectionEntry {
                target: SpectrumTarget::TrackPostMatrix("track-a".into()),
                channels: SpectrumChannels::Left,
            },
            SpectrumCaptureCollectionEntry {
                target: SpectrumTarget::TrackPostInputBuiltins("track-b".into()),
                channels: SpectrumChannels::Right,
            },
        ]
    }

    fn controlled_graph_nodes(
        entries: &[SpectrumCaptureCollectionEntry],
    ) -> Vec<graph::GraphNodeId> {
        entries
            .iter()
            .map(|entry| entry.target.graph_node().expect("valid graph target"))
            .collect()
    }

    #[test]
    fn controlled_preparation_charges_pairs_and_rejects_duplicates_and_one_below() {
        let entries = controlled_entries();
        let resources = super::controlled_spectrum_capture_collection_resources(&entries)
            .expect("controlled resources");
        assert!(resources.retained_bytes > 0);
        assert!(resources.largest_allocation_bytes > 0);
        assert_eq!(
            super::controlled_spectrum_capture_collection_resources(&[
                entries[0].clone(),
                entries[0].clone()
            ]),
            Err(super::SpectrumPrepareError::DuplicateEntry)
        );
        let graph_nodes = controlled_graph_nodes(&entries);
        let request = SpectrumCaptureCollectionRequest {
            entries: entries.clone(),
            maximum_capture_bytes: resources.retained_bytes,
        };
        let (bindings, collection) = super::prepare_controlled_capture_collection(
            &request,
            &graph_nodes,
            resources.largest_allocation_bytes,
        )
        .expect("paired controlled preparation");
        assert_eq!(bindings.len(), entries.len() * 2);
        assert_eq!(collection.entries.len(), entries.len());
        assert_eq!(
            bindings
                .iter()
                .map(|binding| binding.handle)
                .collect::<Vec<_>>(),
            vec![
                u64::MAX,
                u64::MAX - 1,
                u64::MAX - 2,
                u64::MAX - 3,
                u64::MAX - 4,
                u64::MAX - 5
            ]
        );

        let under_capture = SpectrumCaptureCollectionRequest {
            entries: entries.clone(),
            maximum_capture_bytes: resources.retained_bytes - 1,
        };
        assert_eq!(
            super::prepare_controlled_capture_collection(
                &under_capture,
                &graph_nodes,
                resources.largest_allocation_bytes,
            )
            .err()
            .expect("one byte below capture cap"),
            super::SpectrumPrepareError::CaptureBudget
        );
        let under_allocation = SpectrumCaptureCollectionRequest {
            entries,
            maximum_capture_bytes: resources.retained_bytes,
        };
        assert_eq!(
            super::prepare_controlled_capture_collection(
                &under_allocation,
                &graph_nodes,
                resources.largest_allocation_bytes - 1,
            )
            .err()
            .expect("one byte below allocation cap"),
            super::SpectrumPrepareError::AllocationBudget
        );
    }

    #[test]
    fn controlled_lifecycle_keeps_active_slots_untouched_and_reconciles_two_indices() {
        let entries = controlled_entries();
        let resources = super::controlled_spectrum_capture_collection_resources(&entries)
            .expect("controlled resources");
        let request = SpectrumCaptureCollectionRequest {
            entries: entries.clone(),
            maximum_capture_bytes: resources.retained_bytes,
        };
        let graph_nodes = controlled_graph_nodes(&entries);
        let (_, mut collection) = super::prepare_controlled_capture_collection(
            &request,
            &graph_nodes,
            resources.largest_allocation_bytes,
        )
        .expect("paired controlled preparation");
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let first = collection
            .stage(
                &entries[0].target,
                entries[0].channels,
                HostSpectrumMode::Continuous,
                cadence,
            )
            .expect("first stage");
        assert_eq!(first.observer_handle(), u64::MAX);
        assert_eq!(first.descriptor().entry_index, 0);
        collection.commit_candidate(first, 10);
        collection.reconcile_applied(10);
        let active = collection.accepted_slot.expect("active slot");
        assert_eq!(active, 0);
        let active_sample_rate = collection
            .slot(active)
            .capture
            .shared
            .sample_rate_hz
            .load(std::sync::atomic::Ordering::Acquire);
        collection
            .slot_mut(active)
            .capture
            .shared
            .active
            .store(1, std::sync::atomic::Ordering::Release);

        let alternate = collection
            .stage(
                &entries[0].target,
                entries[0].channels,
                HostSpectrumMode::OneShot,
                cadence,
            )
            .expect("same-target alternate stage");
        assert_eq!(alternate.observer_handle(), u64::MAX - 1);
        assert_eq!(
            collection
                .slot(active)
                .capture
                .shared
                .sample_rate_hz
                .load(std::sync::atomic::Ordering::Acquire,),
            active_sample_rate
        );
        assert_eq!(
            collection
                .slot(active)
                .capture
                .shared
                .active
                .load(std::sync::atomic::Ordering::Acquire,),
            1
        );
        let accepted_before_cancel = collection.accepted_slot;
        collection.cancel_candidate(alternate);
        assert_eq!(collection.accepted_slot, accepted_before_cancel);
        assert_eq!(collection.touched_len, 1);

        let replacement = collection
            .stage(
                &entries[0].target,
                entries[0].channels,
                HostSpectrumMode::OneShot,
                cadence,
            )
            .expect("replacement stage");
        collection.commit_candidate(replacement, 11);
        collection.commit_removal(12);
        let replacement_slot = collection.accepted_slot.expect("replacement accepted");
        assert_eq!(replacement_slot, 1);
        assert_eq!(
            collection.slot(replacement_slot).admitted_generation,
            Some(11)
        );
        assert_eq!(
            collection.slot(replacement_slot).retiring_at_revision,
            Some(12)
        );
        assert_eq!(collection.touched_len, 2);

        assert!(
            collection
                .stage(
                    &entries[0].target,
                    entries[0].channels,
                    HostSpectrumMode::Continuous,
                    cadence,
                )
                .is_err()
        );
        collection.reconcile_applied(11);
        assert_eq!(collection.touched_len, 1);
        assert!(collection.slot(0).is_free());
        assert!(!collection.slot(1).is_free());
        collection.reconcile_applied(12);
        assert_eq!(collection.touched_len, 0);
        assert!(collection.slot(1).is_free());
        assert!(collection.accepted_slot.is_none());

        let reused = collection
            .stage(
                &entries[0].target,
                entries[0].channels,
                HostSpectrumMode::Continuous,
                cadence,
            )
            .expect("same-target slot reusable after receipt cleanup");
        assert_eq!(reused.observer_handle(), u64::MAX);
        assert_eq!(collection.touched_len, 1);
    }

    #[test]
    fn analyzer_handles_silence_dc_nyquist_and_axis() {
        let analyzer = SpectrumAnalyzer::new();
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left = [0.0; SPECTRUM_BIN_COUNT];
        let mut right = [0.0; SPECTRUM_BIN_COUNT];
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze(&window(0.0), 48_000, &mut output)
                .expect("silence analyzes");
        }
        assert_eq!(frequencies[0], 0.0);
        assert_eq!(frequencies[SPECTRUM_WINDOW_FRAMES / 2], 24_000.0);
        assert!(frequencies.iter().enumerate().all(|(index, value)| {
            *value == index as f32 * 48_000.0 / SPECTRUM_WINDOW_FRAMES as f32
        }));
        assert!(left.iter().all(|value| *value == SPECTRUM_FLOOR_DB));
        assert!(right.iter().all(|value| *value == SPECTRUM_FLOOR_DB));

        let mut invalid = window(0.0);
        invalid.left[12] = f32::NAN;
        let mut output = super::SpectrumOutput {
            frequencies_hz: &mut frequencies,
            left_dbfs: Some(&mut left),
            right_dbfs: Some(&mut right),
        };
        assert_eq!(
            analyzer
                .analyze(&invalid, 48_000, &mut output)
                .expect_err("nonfinite input refuses"),
            SpectrumAnalysisError::InvalidWindow
        );
        assert_eq!(left[0], SPECTRUM_FLOOR_DB);
    }

    #[test]
    fn analyzer_calibrates_a_half_amplitude_at_minus_six_dbfs() {
        let analyzer = SpectrumAnalyzer::new();
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left = [0.0; SPECTRUM_BIN_COUNT];
        let mut right = [0.0; SPECTRUM_BIN_COUNT];
        let mut output = super::SpectrumOutput {
            frequencies_hz: &mut frequencies,
            left_dbfs: Some(&mut left),
            right_dbfs: Some(&mut right),
        };
        analyzer
            .analyze(&window(0.5), 48_000, &mut output)
            .expect("half-amplitude window analyzes");
        assert!((left[0] + 6.0206).abs() < 0.01, "DC dBFS: {}", left[0]);
        assert!((right[0] + 6.0206).abs() < 0.01, "DC dBFS: {}", right[0]);
    }

    #[test]
    fn analyzer_matches_independent_dft_at_selected_bins_for_launch_rates() {
        let analyzer = SpectrumAnalyzer::new();
        let mut left = [0.0_f32; SPECTRUM_WINDOW_FRAMES];
        let mut right = [0.0_f32; SPECTRUM_WINDOW_FRAMES];
        for (index, (left, right)) in left.iter_mut().zip(right.iter_mut()).enumerate() {
            let phase = core::f64::consts::TAU * index as f64 / SPECTRUM_WINDOW_FRAMES as f64;
            *left = (0.25 + 0.70 * math::sin(37.0 * phase) + 0.15 * math::cos(91.0 * phase)) as f32;
            *right =
                (-0.10 + 0.50 * math::sin(73.0 * phase) + 0.25 * math::cos(511.0 * phase)) as f32;
        }
        let mut windowed_left = [0.0_f64; SPECTRUM_WINDOW_FRAMES];
        let mut windowed_right = [0.0_f64; SPECTRUM_WINDOW_FRAMES];
        let mut weight_sum = 0.0_f64;
        for index in 0..SPECTRUM_WINDOW_FRAMES {
            let phase = core::f64::consts::TAU * index as f64 / SPECTRUM_WINDOW_FRAMES as f64;
            let weight = (0.5 - 0.5 * math::cos(phase)) as f32;
            weight_sum += f64::from(weight);
            windowed_left[index] = f64::from(left[index]) * f64::from(weight);
            windowed_right[index] = f64::from(right[index]) * f64::from(weight);
        }
        let window = SpectrumWindow {
            left,
            right,
            first_sample: 17,
            channels: SpectrumChannels::Stereo,
            source_underrun: false,
        };
        let bins = [0, 37, 73, 91, 511, SPECTRUM_WINDOW_FRAMES / 2];
        for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
            let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
            let mut left_dbfs = [0.0; SPECTRUM_BIN_COUNT];
            let mut right_dbfs = [0.0; SPECTRUM_BIN_COUNT];
            {
                let mut output = super::SpectrumOutput {
                    frequencies_hz: &mut frequencies,
                    left_dbfs: Some(&mut left_dbfs),
                    right_dbfs: Some(&mut right_dbfs),
                };
                analyzer
                    .analyze(&window, sample_rate_hz, &mut output)
                    .expect("launch rate analyzes");
            }
            for &bin in &bins {
                let expected_frequency =
                    bin as f32 * sample_rate_hz as f32 / SPECTRUM_WINDOW_FRAMES as f32;
                assert_eq!(frequencies[bin], expected_frequency);
                let factor = if bin == 0 || bin == SPECTRUM_WINDOW_FRAMES / 2 {
                    1.0
                } else {
                    2.0
                };
                let left_dft = direct_dft_bin(&windowed_left, bin).expect("left DFT");
                let right_dft = direct_dft_bin(&windowed_right, bin).expect("right DFT");
                let left_expected = magnitude_db(
                    Complex64 {
                        re: left_dft.re * factor / weight_sum,
                        im: left_dft.im * factor / weight_sum,
                    },
                    f64::from(SPECTRUM_FLOOR_DB),
                )
                .expect("left dB");
                let right_expected = magnitude_db(
                    Complex64 {
                        re: right_dft.re * factor / weight_sum,
                        im: right_dft.im * factor / weight_sum,
                    },
                    f64::from(SPECTRUM_FLOOR_DB),
                )
                .expect("right dB");
                assert!(
                    (f64::from(left_dbfs[bin]) - left_expected).abs() <= 0.02,
                    "left rate {sample_rate_hz}, bin {bin}: {} vs {left_expected}",
                    left_dbfs[bin]
                );
                assert!(
                    (f64::from(right_dbfs[bin]) - right_expected).abs() <= 0.02,
                    "right rate {sample_rate_hz}, bin {bin}: {} vs {right_expected}",
                    right_dbfs[bin]
                );
            }
        }
    }

    #[test]
    fn analyzer_accepts_selected_left_channel_at_a_large_sample_time() {
        let analyzer = SpectrumAnalyzer::new();
        let mut window = window(0.0);
        window.first_sample = (1_u64 << 53) + 123;
        window.channels = SpectrumChannels::Left;
        window.left[37] = 1.0;
        assert_eq!(window.end_sample(), Some((1_u64 << 53) + 2_171));
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left = [0.0; SPECTRUM_BIN_COUNT];
        let mut output = super::SpectrumOutput {
            frequencies_hz: &mut frequencies,
            left_dbfs: Some(&mut left),
            right_dbfs: None,
        };
        analyzer
            .analyze(&window, 96_000, &mut output)
            .expect("large absolute sample analyzes");
        assert!(left.iter().all(|value| value.is_finite()));
        assert_eq!(frequencies[1], 96_000.0 / SPECTRUM_WINDOW_FRAMES as f32);
    }

    #[test]
    fn capture_preserves_a_large_absolute_sample_without_allocating() {
        let (producer, mut consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(ARMED));
        let mut observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Left,
        );
        let left = [0.25_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [-0.5_f32; SPECTRUM_WINDOW_FRAMES];
        let first_sample = (1_u64 << 53) + 123;
        observer.capture(
            &left,
            &right,
            first_sample,
            super::GraphObservationValidity::CLEAR,
        );
        let window = consumer.try_pop().expect("large-sample capture").window;
        assert_eq!(window.first_sample, first_sample);
        assert_eq!(
            window.end_sample(),
            Some(first_sample + SPECTRUM_WINDOW_FRAMES as u64)
        );
        assert_eq!(window.channels, SpectrumChannels::Left);
        assert!(window.left.iter().all(|value| *value == 0.25));
        assert!(window.right.iter().all(|value| *value == 0.0));
    }

    #[test]
    fn activation_generation_stamps_planar_one_shot_and_continuous_records() {
        let (producer, mut consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(super::IDLE));
        let mut observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Stereo,
        );
        observer.activation_changed(true, 41, 7);
        let left = [0.25_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [-0.5_f32; SPECTRUM_WINDOW_FRAMES];
        observer.capture(&left, &right, 7, super::GraphObservationValidity::CLEAR);
        assert_eq!(
            consumer
                .try_pop()
                .expect("one-shot record")
                .observation_generation(),
            41
        );

        let (mut continuous_observer, mut continuous_capture) =
            continuous_pair(SpectrumChannels::Left);
        continuous_capture
            .mode
            .store(super::CONTINUOUS_MODE, std::sync::atomic::Ordering::Release);
        continuous_observer.activation_changed(true, 73, 0);
        let block = [0.5_f32; 128];
        let silence = [0.0_f32; 128];
        for block_index in 0..16 {
            continuous_observer.capture(
                &block,
                &silence,
                block_index * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let record = continuous_capture
            .try_read_continuous_record(Some(1))
            .expect("continuous record");
        assert_eq!(record.observation_generation(), 73);
        assert_eq!(record.continuous_window().stream_epoch, 1);
    }

    #[test]
    fn activation_resets_scalar_state_without_clearing_capture_arrays_or_queued_records() {
        let (producer, consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(super::IDLE));
        let mut observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Left,
        );
        observer.left[13] = 0.75;
        observer.right[29] = -0.25;
        observer.one_shot.filled = 128;
        observer.one_shot.armed = true;
        observer
            .one_shot
            .state
            .store(CAPTURING, std::sync::atomic::Ordering::Release);
        observer.activation_changed(false, 19, 512);
        assert_eq!(
            state.load(std::sync::atomic::Ordering::Acquire),
            super::IDLE
        );
        assert_eq!(observer.one_shot.filled, 0);
        assert!(!observer.one_shot.armed);
        assert_eq!(observer.left[13], 0.75);
        assert_eq!(observer.right[29], -0.25);
        observer.activation_changed(true, 23, 512);
        assert_eq!(state.load(std::sync::atomic::Ordering::Acquire), ARMED);
        assert!(observer.one_shot.armed);
        assert_eq!(observer.observation_generation, 23);
        assert_eq!(observer.left[13], 0.75);
        assert_eq!(observer.right[29], -0.25);
        drop(consumer);

        let (producer, mut consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(ARMED));
        let mut queued_observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Left,
        );
        queued_observer.activation_changed(true, 31, 0);
        let left = [0.125_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [0.0_f32; SPECTRUM_WINDOW_FRAMES];
        queued_observer.capture(&left, &right, 0, super::GraphObservationValidity::CLEAR);
        queued_observer.activation_changed(false, 0, 0);
        queued_observer.activation_changed(true, 32, SPECTRUM_WINDOW_FRAMES as u64);
        let old = consumer.try_pop().expect("old record remains queued");
        assert_eq!(old.observation_generation(), 31);
    }

    #[test]
    fn bounded_continuous_record_read_does_not_chase_a_refill_after_one_stale_pop() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Left);
        capture
            .start_continuous(48_000, 128)
            .expect("continuous activation");
        let left = [0.5_f32; 128];
        let right = [0.0_f32; 128];
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        observer.invalidate_after_failure(1_920);
        assert!(matches!(
            capture.try_read_continuous_record(Some(1)),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        assert_eq!(
            capture
                .try_read_continuous_record(Some(1))
                .expect_err("one stale pop leaves no refill budget"),
            SpectrumContinuousReadError::Warming
        );
        assert_eq!(capture.consumer.available_at_entry(), 0);
    }

    #[test]
    fn zero_record_pop_budget_leaves_a_completed_one_shot_queued() {
        let (producer, consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(ARMED));
        let mut observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Left,
        );
        let left = [0.25_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [0.0_f32; SPECTRUM_WINDOW_FRAMES];
        observer.capture(&left, &right, 0, super::GraphObservationValidity::CLEAR);
        let mut capture = SpectrumCapture::new_one_shot_for_test(
            consumer,
            state,
            SpectrumTarget::Output("main-out".into()),
        );
        assert_eq!(
            capture
                .try_read_record(Some(0))
                .expect_err("zero budget refuses a pop"),
            SpectrumCaptureReadError::Pending
        );
        assert_eq!(capture.consumer.available_at_entry(), 1);
        assert_eq!(
            capture
                .try_read_record(Some(1))
                .expect("bounded pop")
                .observation_generation(),
            0
        );
    }

    #[test]
    fn spectrum_operation_probes_match_a_large_planar_quantum() {
        super::test_only_reset_spectrum_operation_counts();
        let (producer, state) = {
            let (producer, _consumer) = bounded_spsc(
                core::num::NonZeroUsize::new(1).expect("one capture slot"),
                QueueGeneration(0x5350_4543),
            )
            .expect("capture queue");
            (
                producer,
                std::sync::Arc::new(std::sync::atomic::AtomicU8::new(ARMED)),
            )
        };
        let mut observer = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Stereo,
        );
        let left = vec![0.25_f32; SPECTRUM_WINDOW_FRAMES * 2];
        let right = vec![-0.5_f32; SPECTRUM_WINDOW_FRAMES * 2];
        observer.capture(&left, &right, 0, super::GraphObservationValidity::CLEAR);
        assert_eq!(
            super::test_only_spectrum_operation_counts(),
            super::SpectrumOperationCounts {
                validation_samples: (2 * SPECTRUM_WINDOW_FRAMES) as u64,
                selected_storage_writes: (2 * SPECTRUM_WINDOW_FRAMES) as u64,
                payload_constructions: 1,
                publication_attempts: 1,
            }
        );
    }

    #[test]
    fn inactive_observer_does_no_validation_or_capture_work() {
        super::test_only_reset_spectrum_operation_counts();
        let (mut observer, _capture) = continuous_pair(SpectrumChannels::Stereo);
        observer
            .mode
            .store(super::CONTINUOUS_MODE, std::sync::atomic::Ordering::Release);
        let left = [0.25_f32; 128];
        let right = [-0.5_f32; 128];
        observer.capture(&left, &right, 0, super::GraphObservationValidity::CLEAR);
        assert_eq!(
            super::test_only_spectrum_operation_counts(),
            super::SpectrumOperationCounts::default()
        );
    }

    #[test]
    fn activation_hooks_are_allocation_free_in_both_modes() {
        use bench_support::alloc as bench_alloc;

        bench_alloc::assert_installed();

        let (producer, _consumer) = bounded_spsc(
            core::num::NonZeroUsize::new(1).expect("one capture slot"),
            QueueGeneration(0x5350_4543),
        )
        .expect("capture queue");
        let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(super::IDLE));
        let mut one_shot = SpectrumCaptureObserver::new_one_shot_for_test(
            producer,
            std::sync::Arc::clone(&state),
            SpectrumChannels::Stereo,
        );
        let one_shot_mark = bench_alloc::current_thread_counters();
        one_shot.activation_changed(true, 11, 0);
        one_shot.activation_changed(false, 11, 128);
        one_shot.activation_changed(true, 12, 256);
        let one_shot_delta = bench_alloc::current_thread_delta_since(one_shot_mark);
        assert_eq!(one_shot_delta.allocations, 0);
        assert_eq!(one_shot_delta.reallocations, 0);
        assert_eq!(one_shot_delta.deallocations, 0);

        let (mut continuous, _capture) = continuous_pair(SpectrumChannels::Stereo);
        continuous
            .mode
            .store(super::CONTINUOUS_MODE, std::sync::atomic::Ordering::Release);
        let continuous_mark = bench_alloc::current_thread_counters();
        continuous.activation_changed(true, 21, 0);
        continuous.activation_changed(false, 21, 128);
        continuous.activation_changed(true, 22, 256);
        let continuous_delta = bench_alloc::current_thread_delta_since(continuous_mark);
        assert_eq!(continuous_delta.allocations, 0);
        assert_eq!(continuous_delta.reallocations, 0);
        assert_eq!(continuous_delta.deallocations, 0);
    }

    #[test]
    fn failed_block_invalidates_its_window_and_drains_the_queued_result() {
        const BLOCK_FRAMES: usize = 128;
        let make_observer = || {
            let (producer, consumer) = bounded_spsc(
                core::num::NonZeroUsize::new(1).expect("one capture slot"),
                QueueGeneration(0x5350_4543),
            )
            .expect("capture queue");
            let state = std::sync::Arc::new(std::sync::atomic::AtomicU8::new(ARMED));
            let observer = SpectrumCaptureObserver::new_one_shot_for_test(
                producer,
                std::sync::Arc::clone(&state),
                SpectrumChannels::Left,
            );
            (observer, consumer, state)
        };

        let (mut older, mut older_consumer, older_state) = make_observer();
        let left = [0.25_f32; BLOCK_FRAMES];
        let right = [-0.5_f32; BLOCK_FRAMES];
        for block in 0..(SPECTRUM_WINDOW_FRAMES / BLOCK_FRAMES) {
            older.capture(
                &left,
                &right,
                (block * BLOCK_FRAMES) as u64,
                super::GraphObservationValidity::CLEAR,
            );
        }
        older.invalidate_after_failure(128);
        assert_eq!(
            older_state.load(std::sync::atomic::Ordering::Acquire),
            COMPLETE,
            "a later failed block preserves an older completed window"
        );
        assert!(older_consumer.try_pop().is_ok());

        let (mut current, consumer, state) = make_observer();
        for block in 0..(SPECTRUM_WINDOW_FRAMES / BLOCK_FRAMES) {
            current.capture(
                &left,
                &right,
                2_048 + (block * BLOCK_FRAMES) as u64,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let completing_sample = 2_048 + (SPECTRUM_WINDOW_FRAMES - BLOCK_FRAMES) as u64;
        assert_eq!(current.one_shot.completed_sample, Some(completing_sample));
        current.invalidate_after_failure(completing_sample);
        assert_eq!(
            state.load(std::sync::atomic::Ordering::Acquire),
            INVALID,
            "a failed block cannot publish its completed window"
        );
        let mut capture = super::SpectrumCapture::new_one_shot_for_test(
            consumer,
            state,
            SpectrumTarget::Output("main-out".into()),
        );
        assert_eq!(
            capture
                .try_read()
                .expect_err("failed block reports invalid"),
            SpectrumCaptureReadError::Invalid
        );
        capture
            .arm()
            .expect("draining the invalid result frees the slot");
    }

    #[test]
    fn continuous_cadence_uses_checked_launch_profiles() {
        let expected = [
            (44_100, 2_048),
            (48_000, 2_048),
            (88_200, 2_944),
            (96_000, 3_200),
        ];
        for (sample_rate_hz, hop_frames) in expected {
            let cadence = SpectrumCadence::new(sample_rate_hz, 128).expect("launch cadence");
            assert_eq!(cadence.hop_frames(), hop_frames);
        }
        assert_eq!(
            SpectrumCadence::new(88_200, 192)
                .expect("nondividing launch cadence")
                .hop_frames(),
            3_072
        );
        assert!(SpectrumCadence::new(192_000, 128).is_err());
        assert!(SpectrumCadence::new(48_000, 0).is_err());
    }

    #[test]
    fn one_prepared_capture_switches_between_one_shot_and_continuous_modes() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Stereo);
        let left = [0.25_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [-0.5_f32; SPECTRUM_WINDOW_FRAMES];
        capture.arm().expect("one-shot arm");
        observer.capture(&left, &right, 0, super::GraphObservationValidity::CLEAR);
        assert_eq!(capture.try_read().expect("one-shot result").first_sample, 0);

        let cadence = capture
            .start_continuous(48_000, 128)
            .expect("continuous activation");
        assert_eq!(cadence.hop_frames(), 2_048);
        let block_left = [0.75_f32; 128];
        let block_right = [-0.25_f32; 128];
        for block in 0..16 {
            observer.capture(
                &block_left,
                &block_right,
                2_048 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let continuous = capture.try_read_continuous().expect("continuous result");
        assert_eq!(continuous.first_sample, 2_048);
        assert_eq!(continuous.stream_epoch, 1);
        assert_eq!(continuous.sequence, 0);
        assert_eq!(continuous.left[0], 0.75);
        assert_eq!(continuous.right[0], -0.25);

        capture.stop_continuous();
        capture.arm().expect("one-shot arm after continuous stop");
        observer.capture(&left, &right, 4_096, super::GraphObservationValidity::CLEAR);
        assert_eq!(
            capture
                .try_read()
                .expect("second one-shot result")
                .first_sample,
            4_096
        );
    }

    #[test]
    fn continuous_windows_follow_h_without_reads_and_report_queue_gaps() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Left);
        capture
            .start_continuous(48_000, 128)
            .expect("continuous activation");
        let left = [0.5_f32; 128];
        let right = [0.0_f32; 128];
        for block in 0..32 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("full queue reports gap"),
            SpectrumContinuousReadError::Gap {
                stream_epoch: 1,
                dropped_captures: 1,
            }
        );
        let first = capture.try_read_continuous().expect("first queued window");
        assert_eq!(first.first_sample, 0);
        assert_eq!(first.sequence, 0);
        for block in 32..48 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let recovered = capture.try_read_continuous().expect("later window");
        assert_eq!(recovered.first_sample, 4_096);
        assert_eq!(recovered.sequence, 2);
        assert_eq!(recovered.dropped_captures, 1);
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("reported gap does not repeat"),
            SpectrumContinuousReadError::Pending
        );

        observer.capture(&left, &right, 7_000, super::GraphObservationValidity::CLEAR);
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                7_000 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let new_epoch = capture.try_read_continuous().expect("new epoch after gap");
        assert_eq!(new_epoch.stream_epoch, 2);
        assert_eq!(new_epoch.sequence, 0);
        assert_eq!(new_epoch.dropped_captures, 0);
    }

    #[test]
    fn continuous_profile_skips_to_a_slower_hop_and_resets_after_discontinuity() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Right);
        capture
            .start_continuous(88_200, 128)
            .expect("continuous activation");
        let left = [0.0_f32; 128];
        let right = [0.25_f32; 128];
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let first = capture.try_read_continuous().expect("first slower window");
        assert_eq!(first.first_sample, 0);
        for block in 16..23 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("waiting for slower hop"),
            SpectrumContinuousReadError::Pending
        );
        observer.capture(&left, &right, 2_944, super::GraphObservationValidity::CLEAR);
        for block in 24..39 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let second = capture.try_read_continuous().expect("second slower window");
        assert_eq!(second.first_sample, 2_944);
        assert_eq!(second.sequence, 1);

        observer.capture(&left, &right, 5_000, super::GraphObservationValidity::CLEAR);
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        observer.capture(&left, &right, 5_128, super::GraphObservationValidity::CLEAR);
        for block in 1..16 {
            observer.capture(
                &left,
                &right,
                5_128 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let after_failure = capture.try_read_continuous().expect("new epoch window");
        assert_eq!(after_failure.stream_epoch, 2);
        assert_eq!(after_failure.sequence, 0);
        assert_eq!(after_failure.first_sample, 5_128);
    }

    #[test]
    fn failed_continuous_completion_drains_only_the_failed_queued_window() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Left);
        capture
            .start_continuous(48_000, 128)
            .expect("continuous activation");
        let left = [0.5_f32; 128];
        let right = [0.0_f32; 128];
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        observer.invalidate_after_failure(1_920);
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("failed completion is drained"),
            SpectrumContinuousReadError::Warming
        );
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                2_048 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let recovered = capture.try_read_continuous().expect("post-failure window");
        assert_eq!(recovered.stream_epoch, 2);
        assert_eq!(recovered.sequence, 0);
        assert_eq!(recovered.first_sample, 2_048);
    }

    #[test]
    fn repeated_failed_completions_discard_the_old_epoch_queue_once() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Left);
        capture
            .start_continuous(48_000, 128)
            .expect("continuous activation");
        let left = [0.5_f32; 128];
        let right = [0.0_f32; 128];
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        observer.invalidate_after_failure(1_920);
        for block in 16..32 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        observer.invalidate_after_failure(3_968);
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 3 })
        ));
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("old epoch queue is discarded once"),
            SpectrumContinuousReadError::Warming
        );
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                4_096 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let recovered = capture
            .try_read_continuous()
            .expect("new epoch after repeated failure");
        assert_eq!(recovered.stream_epoch, 3);
        assert_eq!(recovered.sequence, 0);
        assert_eq!(recovered.dropped_captures, 0);
    }

    #[test]
    fn next_hop_overflow_discards_the_published_window() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Stereo);
        capture
            .start_continuous(96_000, 128)
            .expect("continuous activation");
        let left = [0.5_f32; 128];
        let right = [-0.25_f32; 128];
        let first_sample = u64::MAX - 2_500;
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                first_sample + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        assert_eq!(
            capture
                .try_read_continuous()
                .expect_err("overflowed window is discarded"),
            SpectrumContinuousReadError::Warming
        );
    }

    #[test]
    fn nonfinite_input_between_scheduled_windows_resets_continuous_history() {
        let (mut observer, mut capture) = continuous_pair(SpectrumChannels::Right);
        capture
            .start_continuous(88_200, 128)
            .expect("continuous activation");
        let left = [0.0_f32; 128];
        let right = [0.25_f32; 128];
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let first = capture.try_read_continuous().expect("first window");
        assert_eq!(first.first_sample, 0);
        for block in 16..18 {
            observer.capture(
                &left,
                &right,
                block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let mut nonfinite = right;
        nonfinite[3] = f32::NAN;
        observer.capture(
            &left,
            &nonfinite,
            2_304,
            super::GraphObservationValidity::CLEAR,
        );
        assert!(matches!(
            capture.try_read_continuous(),
            Err(SpectrumContinuousReadError::Failed { stream_epoch: 2 })
        ));
        for block in 0..16 {
            observer.capture(
                &left,
                &right,
                2_432 + block * 128,
                super::GraphObservationValidity::CLEAR,
            );
        }
        let recovered = capture
            .try_read_continuous()
            .expect("window after nonfinite reset");
        assert_eq!(recovered.stream_epoch, 2);
        assert_eq!(recovered.sequence, 0);
        assert_eq!(recovered.first_sample, 2_432);
    }

    #[test]
    fn continuous_analysis_smooths_power_and_reports_history_separately_from_fft_span() {
        let analyzer = SpectrumAnalyzer::new();
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let smoothing = SpectrumSmoothingConfig::new(100.0).expect("bounded smoothing");
        let mut history = SpectrumAnalysisHistory::new();
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left = [0.0; SPECTRUM_BIN_COUNT];
        let mut right = [0.0; SPECTRUM_BIN_COUNT];
        let first = continuous_window(0.5, 10, 4, 7);
        let first_metadata = {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&first, cadence, smoothing, &mut history, &mut output)
                .expect("first smoothed window")
        };
        assert_eq!(first_metadata.analysis_epoch, 0);
        assert_eq!(first_metadata.history_start_sample, 10);
        assert_eq!(first_metadata.fft_first_sample, 10);
        assert_eq!(first_metadata.fft_end_sample, 2_058);
        assert_eq!(first_metadata.stream_epoch, 4);
        assert_eq!(first_metadata.sequence, 7);
        assert_eq!(first_metadata.smoothing_ms, 100.0);
        assert!((left[0] + 6.0206).abs() < 0.01);

        let second = continuous_window(1.0, 2_058, 4, 8);
        let second_metadata = {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&second, cadence, smoothing, &mut history, &mut output)
                .expect("second smoothed window")
        };
        let factor = math::exp(-2_048.0 / (48_000.0 * 0.1));
        let expected_power = factor * 0.25 + (1.0 - factor);
        let expected_db = 10.0 * math::log10(expected_power);
        assert!((f64::from(left[0]) - expected_db).abs() < 0.01);
        assert_eq!(second_metadata.history_start_sample, 10);
        assert_eq!(second_metadata.fft_first_sample, 2_058);
        assert_eq!(history.history_start_sample(), Some(10));
    }

    #[test]
    fn continuous_analysis_zero_smoothing_matches_one_shot_and_configures_defaults() {
        assert_eq!(SpectrumSmoothingConfig::default().smoothing_ms(), 100.0);
        assert_eq!(
            SpectrumSmoothingConfig::new(10_001.0).expect_err("duration is bounded"),
            SpectrumSmoothingConfigError::OutOfRange
        );
        assert_eq!(
            SpectrumSmoothingConfig::new(f64::NAN).expect_err("nonfinite duration refuses"),
            SpectrumSmoothingConfigError::OutOfRange
        );
        let history_resources = super::spectrum_analysis_history_resources();
        assert!(history_resources.retained_bytes >= (2 * SPECTRUM_BIN_COUNT * 8) as u64);
        assert_eq!(
            history_resources.retained_bytes,
            history_resources.largest_allocation_bytes
        );
        let analyzer = SpectrumAnalyzer::new();
        let window = continuous_window(0.25, 17, 2, 3);
        let cadence = SpectrumCadence::new(96_000, 192).expect("launch cadence");
        let mut one_frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut one_left = [0.0; SPECTRUM_BIN_COUNT];
        let mut one_right = [0.0; SPECTRUM_BIN_COUNT];
        let mut continuous_frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut continuous_left = [0.0; SPECTRUM_BIN_COUNT];
        let mut continuous_right = [0.0; SPECTRUM_BIN_COUNT];
        let mut history = SpectrumAnalysisHistory::new();
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut one_frequencies,
                left_dbfs: Some(&mut one_left),
                right_dbfs: Some(&mut one_right),
            };
            analyzer
                .analyze(&window.as_window(), 96_000, &mut output)
                .expect("one-shot analysis");
        }
        let metadata = {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut continuous_frequencies,
                left_dbfs: Some(&mut continuous_left),
                right_dbfs: Some(&mut continuous_right),
            };
            analyzer
                .analyze_continuous(
                    &window,
                    cadence,
                    SpectrumSmoothingConfig::new(0.0).expect("zero smoothing"),
                    &mut history,
                    &mut output,
                )
                .expect("zero-smoothing analysis")
        };
        assert_eq!(one_frequencies, continuous_frequencies);
        for (one, continuous) in one_left.iter().zip(continuous_left.iter()) {
            assert!((one - continuous).abs() < 0.0001, "{one} vs {continuous}");
        }
        for (one, continuous) in one_right.iter().zip(continuous_right.iter()) {
            assert!((one - continuous).abs() < 0.0001, "{one} vs {continuous}");
        }
        assert_eq!(metadata.smoothing_ms, 0.0);
        assert_eq!(history.analysis_epoch(), 0);
    }

    #[test]
    fn continuous_analysis_resets_on_gap_epoch_and_smoothing_change() {
        let analyzer = SpectrumAnalyzer::new();
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let smoothing = SpectrumSmoothingConfig::default();
        let mut history = SpectrumAnalysisHistory::new();
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left = [0.0; SPECTRUM_BIN_COUNT];
        let mut right = [0.0; SPECTRUM_BIN_COUNT];
        let first = continuous_window(0.25, 0, 5, 0);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&first, cadence, smoothing, &mut history, &mut output)
                .expect("continuous analysis");
        }
        let gap = continuous_window(1.0, 2_048, 5, 2);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&gap, cadence, smoothing, &mut history, &mut output)
                .expect("gap analysis");
        }
        assert_eq!(history.analysis_epoch(), 1);
        assert_eq!(history.history_start_sample(), Some(2_048));
        assert!(
            left[0] > -0.01,
            "gap starts a fresh power history: {}",
            left[0]
        );

        let epoch_change = continuous_window(0.25, 4_096, 6, 0);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&epoch_change, cadence, smoothing, &mut history, &mut output)
                .expect("epoch-change analysis");
        }
        assert_eq!(history.analysis_epoch(), 2);
        assert_eq!(history.history_start_sample(), Some(4_096));
        assert!((left[0] + 12.0412).abs() < 0.02);

        let config_change = continuous_window(1.0, 6_144, 6, 1);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(
                    &config_change,
                    cadence,
                    SpectrumSmoothingConfig::new(200.0).expect("new smoothing"),
                    &mut history,
                    &mut output,
                )
                .expect("configuration-change analysis");
        }
        assert_eq!(history.analysis_epoch(), 3);
        assert_eq!(history.history_start_sample(), Some(6_144));
        assert!(left[0] > -0.01, "config reset starts with current power");
    }

    #[test]
    fn continuous_analysis_resets_after_invalid_input_without_partial_output_or_history() {
        let analyzer = SpectrumAnalyzer::new();
        let cadence = SpectrumCadence::new(48_000, 128).expect("launch cadence");
        let smoothing = SpectrumSmoothingConfig::default();
        let mut history = SpectrumAnalysisHistory::new();
        let mut frequencies = [7.0; SPECTRUM_BIN_COUNT];
        let mut left = [8.0; SPECTRUM_BIN_COUNT];
        let mut right = [9.0; SPECTRUM_BIN_COUNT];
        let first = continuous_window(0.25, 0, 3, 0);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&first, cadence, smoothing, &mut history, &mut output)
                .expect("initial history");
        }
        let mut invalid = continuous_window(1.0, 2_048, 3, 1);
        invalid.left[11] = f32::NAN;
        let epoch_before_shape = history.analysis_epoch();
        let mut malformed_frequencies = [4.0; SPECTRUM_BIN_COUNT];
        let mut malformed_left = [5.0; SPECTRUM_BIN_COUNT];
        let mut malformed_right = [6.0; SPECTRUM_BIN_COUNT - 1];
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut malformed_frequencies,
                left_dbfs: Some(&mut malformed_left),
                right_dbfs: Some(&mut malformed_right),
            };
            assert_eq!(
                analyzer
                    .analyze_continuous(&invalid, cadence, smoothing, &mut history, &mut output)
                    .expect_err("malformed output refuses before history mutation"),
                SpectrumAnalysisError::OutputShape
            );
        }
        assert_eq!(history.analysis_epoch(), epoch_before_shape);
        assert_eq!(history.history_start_sample(), Some(0));

        let epoch_before_error = history.analysis_epoch();
        frequencies.fill(7.0);
        left.fill(8.0);
        right.fill(9.0);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            assert_eq!(
                analyzer
                    .analyze_continuous(&invalid, cadence, smoothing, &mut history, &mut output)
                    .expect_err("nonfinite continuous input refuses"),
                SpectrumAnalysisError::InvalidWindow
            );
        }
        assert_eq!(history.analysis_epoch(), epoch_before_error + 1);
        assert_eq!(history.history_start_sample(), None);
        assert!(frequencies.iter().all(|value| *value == 7.0));
        assert!(left.iter().all(|value| *value == 8.0));
        assert!(right.iter().all(|value| *value == 9.0));

        let recovered = continuous_window(1.0, 2_048, 9, 0);
        {
            let mut output = super::SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: Some(&mut left),
                right_dbfs: Some(&mut right),
            };
            analyzer
                .analyze_continuous(&recovered, cadence, smoothing, &mut history, &mut output)
                .expect("history recovers after invalid input");
        }
        assert_eq!(history.history_start_sample(), Some(2_048));
        assert!(left[0] > -0.01, "recovery starts with current power");
    }

    #[test]
    fn analyzer_refuses_bad_rate_and_shape_before_writing() {
        let analyzer = SpectrumAnalyzer::new();
        let mut frequencies = [1.0; SPECTRUM_BIN_COUNT];
        let mut left = [2.0; SPECTRUM_BIN_COUNT];
        let mut right = [3.0; SPECTRUM_BIN_COUNT - 1];
        let mut output = super::SpectrumOutput {
            frequencies_hz: &mut frequencies,
            left_dbfs: Some(&mut left),
            right_dbfs: Some(&mut right),
        };
        assert_eq!(
            analyzer
                .analyze(&window(0.0), 192_000, &mut output)
                .expect_err("extended rate refuses"),
            SpectrumAnalysisError::UnsupportedRate
        );
        assert_eq!(frequencies[0], 1.0);
        assert_eq!(left[0], 2.0);
        assert_eq!(right[0], 3.0);
        let _ = SpectrumCaptureReadError::Pending;
    }
}
