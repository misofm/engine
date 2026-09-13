//! One-shot, engine-captured spectrum windows.
//!
//! Capture is a graph observer with a fixed owner-side window.  The observer only copies the
//! selected render block into its prepared storage; [`SpectrumAnalyzer`] runs later, after the
//! window has crossed the SPSC boundary, and is therefore never reachable from render.

use core::num::NonZeroUsize;
use std::sync::{
    Arc,
    atomic::{AtomicU8, AtomicU64, Ordering},
};

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
const CONTINUOUS_WAITING: u8 = 0;
const CONTINUOUS_CAPTURING: u8 = 1;
const ONE_SHOT_MODE: u8 = 0;
const CONTINUOUS_MODE: u8 = 1;

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

/// Address-free capture storage facts used by preparation and host projections.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpectrumCaptureResources {
    /// All retained bytes for the observer and one completed-window queue slot.
    pub retained_bytes: u64,
    /// The largest individual requested allocation.
    pub largest_allocation_bytes: u64,
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
struct SpectrumCapturedRecord {
    window: SpectrumWindow,
    stream_epoch: u64,
    sequence: u64,
    dropped_captures: u64,
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
        }
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
        if self.mode.load(Ordering::Acquire) != ONE_SHOT_MODE {
            return Err(SpectrumCaptureReadError::NotArmed);
        }
        match self.state.load(Ordering::Acquire) {
            COMPLETE => match self.consumer.try_pop() {
                Ok(record) => {
                    self.state.store(IDLE, Ordering::Release);
                    Ok(record.window)
                }
                Err(_) => Err(SpectrumCaptureReadError::Pending),
            },
            ARMED | CAPTURING => Err(SpectrumCaptureReadError::Pending),
            INVALID => {
                // The failed render may have queued a completed window before its error. The
                // queue has one slot, so one bounded pop clears that stale result.
                let _ = self.consumer.try_pop();
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
        self.shared.epoch.store(epoch, Ordering::Release);
        self.shared
            .sample_rate_hz
            .store(u64::from(sample_rate_hz), Ordering::Release);
        self.shared
            .quantum_frames
            .store(u64::from(quantum_frames), Ordering::Release);
        self.shared
            .hop_frames
            .store(u64::from(cadence.hop_frames()), Ordering::Release);
        self.shared
            .phase
            .store(CONTINUOUS_WAITING, Ordering::Release);
        self.shared.failures.store(0, Ordering::Release);
        self.shared.failure_epoch.store(epoch, Ordering::Release);
        self.shared.drops.store(0, Ordering::Release);
        self.shared.drop_epoch.store(epoch, Ordering::Release);
        while self.consumer.try_pop().is_ok() {}
        self.seen_failures = 0;
        self.seen_drops = 0;
        self.mode.store(CONTINUOUS_MODE, Ordering::Release);
        Ok(cadence)
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

    /// Read one scheduled window and its stream-history metadata.
    pub fn try_read_continuous(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        if self.mode.load(Ordering::Acquire) != CONTINUOUS_MODE
            || self.shared.active.load(Ordering::Acquire) == 0
        {
            return Err(SpectrumContinuousReadError::NotActive);
        }
        let failures = self.shared.failures.load(Ordering::Acquire);
        if failures != self.seen_failures {
            self.seen_failures = failures;
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
        match self.consumer.try_pop() {
            Ok(record) => Ok(SpectrumContinuousWindow {
                left: record.window.left,
                right: record.window.right,
                first_sample: record.window.first_sample,
                channels: record.window.channels,
                source_underrun: record.window.source_underrun,
                stream_epoch: record.stream_epoch,
                sequence: record.sequence,
                dropped_captures: record.dropped_captures,
            }),
            Err(_) if self.shared.failures.load(Ordering::Acquire) != 0 => {
                Err(SpectrumContinuousReadError::Failed {
                    stream_epoch: self.shared.failure_epoch.load(Ordering::Acquire),
                })
            }
            Err(_) if self.shared.drops.load(Ordering::Acquire) != 0 => {
                Err(SpectrumContinuousReadError::Gap {
                    stream_epoch: self.shared.drop_epoch.load(Ordering::Acquire),
                    dropped_captures: self.shared.drops.load(Ordering::Acquire),
                })
            }
            Err(_) if self.shared.phase.load(Ordering::Acquire) == CONTINUOUS_WAITING => {
                Err(SpectrumContinuousReadError::Warming)
            }
            Err(_) => Err(SpectrumContinuousReadError::Pending),
        }
    }

    /// The selected graph boundary for this capture.
    #[must_use]
    pub fn target(&self) -> &SpectrumTarget {
        &self.target
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
}

impl SpectrumContinuousShared {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            active: AtomicU8::new(0),
            phase: AtomicU8::new(CONTINUOUS_WAITING),
            epoch: AtomicU64::new(0),
            failures: AtomicU64::new(0),
            failure_epoch: AtomicU64::new(0),
            drops: AtomicU64::new(0),
            drop_epoch: AtomicU64::new(0),
            sample_rate_hz: AtomicU64::new(0),
            quantum_frames: AtomicU64::new(0),
            hop_frames: AtomicU64::new(0),
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
    };
    let observer = SpectrumCaptureObserver::new(producer, state, mode, shared, request.channels);
    Ok((
        GraphNodeObserverBinding::new(node, SPECTRUM_OBSERVER_HANDLE, Box::new(observer)),
        capture,
    ))
}

struct SpectrumCaptureObserver {
    producer: Producer<SpectrumCapturedRecord>,
    left: [f32; SPECTRUM_WINDOW_FRAMES],
    right: [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    mode: Arc<AtomicU8>,
    one_shot: SpectrumOneShotState,
    continuous: SpectrumContinuousState,
}

struct SpectrumCaptureBuffers<'a> {
    producer: &'a mut Producer<SpectrumCapturedRecord>,
    left: &'a mut [f32; SPECTRUM_WINDOW_FRAMES],
    right: &'a mut [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
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
        let channels = self.channels;
        let producer = &mut self.producer;
        let storage_left = &mut self.left;
        let storage_right = &mut self.right;
        if self.mode.load(Ordering::Acquire) == CONTINUOUS_MODE {
            continuous_capture_resident(
                &mut self.continuous,
                producer,
                storage_left,
                storage_right,
                channels,
                block,
            );
        } else {
            one_shot_capture_resident(
                &mut self.one_shot,
                producer,
                storage_left,
                storage_right,
                channels,
                block,
            );
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
    producer: &mut Producer<SpectrumCapturedRecord>,
    left: &[f32; SPECTRUM_WINDOW_FRAMES],
    right: &[f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    first_sample: u64,
    count: usize,
) {
    state.filled += count;
    state.next_sample = first_sample
        .checked_add(u64::try_from(count).unwrap_or(0))
        .unwrap_or(first_sample);
    if state.filled == SPECTRUM_WINDOW_FRAMES {
        let window = SpectrumWindow {
            left: *left,
            right: *right,
            first_sample: state.first_sample,
            channels,
            source_underrun: state.source_underrun,
        };
        let record = SpectrumCapturedRecord {
            window,
            stream_epoch: 0,
            sequence: 0,
            dropped_captures: 0,
        };
        if producer.try_push(record).is_ok() {
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
    if (buffers.channels.includes_left() && left[..count].iter().any(|value| !value.is_finite()))
        || (buffers.channels.includes_right()
            && right[..count].iter().any(|value| !value.is_finite()))
    {
        one_shot_invalidate(state);
        return;
    }
    let destination = state.filled;
    if buffers.channels.includes_left() {
        buffers.left[destination..destination + count].copy_from_slice(&left[..count]);
    }
    if buffers.channels.includes_right() {
        buffers.right[destination..destination + count].copy_from_slice(&right[..count]);
    }
    one_shot_finish(
        state,
        buffers.producer,
        buffers.left,
        buffers.right,
        buffers.channels,
        first_sample,
        count,
    );
}

fn one_shot_capture_resident(
    state: &mut SpectrumOneShotState,
    producer: &mut Producer<SpectrumCapturedRecord>,
    storage_left: &mut [f32; SPECTRUM_WINDOW_FRAMES],
    storage_right: &mut [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
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
        if channels.includes_left() {
            let Some(value) = left.get(index).copied() else {
                one_shot_invalidate(state);
                return;
            };
            if !value.is_finite() {
                one_shot_invalidate(state);
                return;
            }
            storage_left[destination + frame] = value;
        }
        if channels.includes_right() {
            let Some(value) = right.get(index).copied() else {
                one_shot_invalidate(state);
                return;
            };
            if !value.is_finite() {
                one_shot_invalidate(state);
                return;
            }
            storage_right[destination + frame] = value;
        }
    }
    one_shot_finish(
        state,
        producer,
        storage_left,
        storage_right,
        channels,
        block.first_sample,
        count,
    );
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
    producer: &mut Producer<SpectrumCapturedRecord>,
    left: &[f32; SPECTRUM_WINDOW_FRAMES],
    right: &[f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    first_sample: u64,
    count: usize,
) {
    state.filled += count;
    if state.filled != SPECTRUM_WINDOW_FRAMES {
        return;
    }
    let stream_epoch = state.shared.epoch.load(Ordering::Acquire);
    let sequence = state.sequence;
    let record = SpectrumCapturedRecord {
        window: SpectrumWindow {
            left: *left,
            right: *right,
            first_sample: state.first_sample,
            channels,
            source_underrun: state.source_underrun,
        },
        stream_epoch,
        sequence,
        dropped_captures: state.shared.drops.load(Ordering::Acquire),
    };
    if producer.try_push(record).is_err() {
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
    if left.len() != right.len() {
        continuous_fail(state);
        return;
    }
    let Some(count) = continuous_begin(state, first_sample, left.len(), validity) else {
        return;
    };
    if (buffers.channels.includes_left() && left.iter().any(|value| !value.is_finite()))
        || (buffers.channels.includes_right() && right.iter().any(|value| !value.is_finite()))
    {
        continuous_fail(state);
        return;
    }
    if count == 0 {
        return;
    }
    let destination = state.filled;
    if buffers.channels.includes_left() {
        buffers.left[destination..destination + count].copy_from_slice(&left[..count]);
    }
    if buffers.channels.includes_right() {
        buffers.right[destination..destination + count].copy_from_slice(&right[..count]);
    }
    continuous_finish(
        state,
        buffers.producer,
        buffers.left,
        buffers.right,
        buffers.channels,
        first_sample,
        count,
    );
}

fn continuous_capture_resident(
    state: &mut SpectrumContinuousState,
    producer: &mut Producer<SpectrumCapturedRecord>,
    storage_left: &mut [f32; SPECTRUM_WINDOW_FRAMES],
    storage_right: &mut [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
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
    let Some(count) = continuous_begin(state, block.first_sample, frames, block.validity) else {
        return;
    };
    let destination = state.filled;
    for frame in 0..frames {
        let Some(index) = frame
            .checked_mul(lanes)
            .and_then(|value| value.checked_add(lane))
        else {
            continuous_fail(state);
            return;
        };
        if channels.includes_left() {
            let Some(value) = left.get(index).copied() else {
                continuous_fail(state);
                return;
            };
            if !value.is_finite() {
                continuous_fail(state);
                return;
            }
            if frame < count {
                storage_left[destination + frame] = value;
            }
        }
        if channels.includes_right() {
            let Some(value) = right.get(index).copied() else {
                continuous_fail(state);
                return;
            };
            if !value.is_finite() {
                continuous_fail(state);
                return;
            }
            if frame < count {
                storage_right[destination + frame] = value;
            }
        }
    }
    if count != 0 {
        continuous_finish(
            state,
            producer,
            storage_left,
            storage_right,
            channels,
            block.first_sample,
            count,
        );
    }
}

fn continuous_fail(state: &mut SpectrumContinuousState) {
    if state.shared.active.load(Ordering::Acquire) == 0 {
        return;
    }
    let current_epoch = state.shared.epoch.load(Ordering::Acquire);
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
        .store(CONTINUOUS_WAITING, Ordering::Release);
}

impl GraphRuntimeObserver for SpectrumCaptureObserver {
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
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        for bin in 0..SPECTRUM_BIN_COUNT {
            frequencies[bin] =
                (bin as f64 * sample_rate_hz as f64 / SPECTRUM_WINDOW_FRAMES as f64) as f32;
            let factor = if bin == 0 || bin == SPECTRUM_WINDOW_FRAMES / 2 {
                1.0
            } else {
                2.0
            };
            if let Some(left_fft) = left_fft.as_ref() {
                let left_magnitude = fft_magnitude(left_fft, bin);
                left_dbfs[bin] = amplitude_db(left_magnitude * factor / self.weight_sum)?;
            }
            if let Some(right_fft) = right_fft.as_ref() {
                let right_magnitude = fft_magnitude(right_fft, bin);
                right_dbfs[bin] = amplitude_db(right_magnitude * factor / self.weight_sum)?;
            }
        }
        output.frequencies_hz.copy_from_slice(&frequencies);
        if let Some(values) = output.left_dbfs.as_deref_mut() {
            values.copy_from_slice(&left_dbfs);
        }
        if let Some(values) = output.right_dbfs.as_deref_mut() {
            values.copy_from_slice(&right_dbfs);
        }
        Ok(())
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
        ARMED, COMPLETE, INVALID, SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB, SPECTRUM_WINDOW_FRAMES,
        SpectrumAnalysisError, SpectrumAnalyzer, SpectrumCadence, SpectrumCapture,
        SpectrumCaptureObserver, SpectrumCaptureReadError, SpectrumChannels,
        SpectrumContinuousReadError, SpectrumTarget, SpectrumWindow,
    };
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
        };
        (observer, capture)
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
            SpectrumContinuousReadError::Warming
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
