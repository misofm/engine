//! One-shot, engine-captured spectrum windows.
//!
//! Capture is a graph observer with a fixed owner-side window.  The observer only copies the
//! selected render block into its prepared storage; [`SpectrumAnalyzer`] runs later, after the
//! window has crossed the SPSC boundary, and is therefore never reachable from render.

use core::num::NonZeroUsize;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
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
    let queue = bounded_spsc_retained_payload::<SpectrumWindow>(
        NonZeroUsize::new(1).expect("one queue slot"),
    )
    .expect("fixed spectrum queue layout");
    let observer_bytes = core::mem::size_of::<SpectrumCaptureObserver>();
    let queue_bytes = queue
        .total_bytes()
        .expect("fixed spectrum queue bytes fit usize");
    let state_bytes = core::mem::size_of::<SpectrumStateAllocation>();
    let observer_binding_bytes = core::mem::size_of::<GraphNodeObserverBinding>();
    let observer_bytes = u64::try_from(observer_bytes).expect("observer bytes fit u64");
    let queue_bytes = u64::try_from(queue_bytes).expect("queue bytes fit u64");
    let state_bytes = u64::try_from(state_bytes).expect("state bytes fit u64");
    let observer_binding_bytes =
        u64::try_from(observer_binding_bytes).expect("observer binding bytes fit u64");
    let id_bytes = u64::try_from(id_bytes).expect("target ID bytes fit u64");
    let target_ids = id_bytes.checked_mul(2).expect("target IDs fit u64");
    SpectrumCaptureResources {
        retained_bytes: observer_bytes
            .checked_add(queue_bytes)
            .and_then(|value| value.checked_add(state_bytes))
            .and_then(|value| value.checked_add(observer_binding_bytes))
            .and_then(|value| value.checked_add(target_ids))
            .expect("spectrum storage fits u64"),
        largest_allocation_bytes: observer_bytes
            .max(
                u64::try_from(queue.largest_allocation_bytes()).expect("queue allocation fits u64"),
            )
            .max(state_bytes)
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

/// Control-side ownership of one prepared graph observer.
pub struct SpectrumCapture {
    consumer: Consumer<SpectrumWindow>,
    state: Arc<AtomicU8>,
    target: SpectrumTarget,
}

impl SpectrumCapture {
    /// Arm the capture for the next complete graph window.
    ///
    /// Arming is a control-side operation.  The first successfully observed block establishes
    /// the returned window's absolute start sample.
    pub fn arm(&self) -> Result<(), SpectrumCaptureError> {
        self.state
            .compare_exchange(IDLE, ARMED, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| ())
            .map_err(|_| SpectrumCaptureError::Busy)
    }

    /// Cancel the current capture, discarding any completed window.
    pub fn cancel(&mut self) {
        while self.consumer.try_pop().is_ok() {}
        self.state.store(IDLE, Ordering::Release);
    }

    /// Try to take the completed window from its control-side queue.
    pub fn try_read(&mut self) -> Result<SpectrumWindow, SpectrumCaptureReadError> {
        match self.state.load(Ordering::Acquire) {
            COMPLETE => match self.consumer.try_pop() {
                Ok(window) => {
                    self.state.store(IDLE, Ordering::Release);
                    Ok(window)
                }
                Err(_) => Err(SpectrumCaptureReadError::Pending),
            },
            ARMED | CAPTURING => Err(SpectrumCaptureReadError::Pending),
            INVALID => {
                self.state.store(IDLE, Ordering::Release);
                Err(SpectrumCaptureReadError::Invalid)
            }
            IDLE => Err(SpectrumCaptureReadError::NotArmed),
            _ => Err(SpectrumCaptureReadError::Invalid),
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
    let capture = SpectrumCapture {
        consumer,
        state: Arc::clone(&state),
        target: request.target.clone(),
    };
    let observer = SpectrumCaptureObserver {
        producer,
        state,
        left: [0.0; SPECTRUM_WINDOW_FRAMES],
        right: [0.0; SPECTRUM_WINDOW_FRAMES],
        channels: request.channels,
        filled: 0,
        first_sample: 0,
        next_sample: 0,
        armed: false,
        source_underrun: false,
    };
    Ok((
        GraphNodeObserverBinding::new(node, SPECTRUM_OBSERVER_HANDLE, Box::new(observer)),
        capture,
    ))
}

struct SpectrumCaptureObserver {
    producer: Producer<SpectrumWindow>,
    state: Arc<AtomicU8>,
    left: [f32; SPECTRUM_WINDOW_FRAMES],
    right: [f32; SPECTRUM_WINDOW_FRAMES],
    channels: SpectrumChannels,
    filled: usize,
    first_sample: u64,
    next_sample: u64,
    armed: bool,
    source_underrun: bool,
}

impl SpectrumCaptureObserver {
    // REALTIME_POLICY_BEGIN
    fn begin_block(
        &mut self,
        first_sample: u64,
        frames: usize,
        validity: GraphObservationValidity,
    ) -> Option<usize> {
        let state = self.state.load(Ordering::Acquire);
        if state == ARMED {
            if self
                .state
                .compare_exchange(ARMED, CAPTURING, Ordering::AcqRel, Ordering::Acquire)
                .is_err()
            {
                return None;
            }
            self.filled = 0;
            self.first_sample = first_sample;
            self.next_sample = first_sample;
            self.armed = true;
            self.source_underrun = validity.source_underrun;
        } else if state == CAPTURING {
            if validity.source_generation_changed {
                self.invalidate();
                return None;
            }
            self.source_underrun |= validity.source_underrun;
        }
        if self.state.load(Ordering::Acquire) != CAPTURING || !self.armed {
            return None;
        }
        if frames == 0 || first_sample != self.next_sample {
            self.invalidate();
            return None;
        }
        if first_sample
            .checked_add(u64::try_from(frames).unwrap_or(0))
            .is_none()
        {
            self.invalidate();
            return None;
        }
        let remaining = SPECTRUM_WINDOW_FRAMES - self.filled;
        Some(remaining.min(frames))
            .filter(|count| *count > 0)
            .or_else(|| {
                self.invalidate();
                None
            })
    }

    fn finish_block(&mut self, first_sample: u64, count: usize) {
        self.filled += count;
        self.next_sample = first_sample
            .checked_add(u64::try_from(count).unwrap_or(0))
            .unwrap_or(first_sample);
        if self.filled == SPECTRUM_WINDOW_FRAMES {
            let window = SpectrumWindow {
                left: self.left,
                right: self.right,
                first_sample: self.first_sample,
                channels: self.channels,
                source_underrun: self.source_underrun,
            };
            if self.producer.try_push(window).is_ok() {
                self.state.store(COMPLETE, Ordering::Release);
            } else {
                self.invalidate();
            }
            self.armed = false;
        }
    }

    fn capture(
        &mut self,
        left: &[f32],
        right: &[f32],
        first_sample: u64,
        validity: GraphObservationValidity,
    ) {
        if left.len() != right.len() {
            if matches!(self.state.load(Ordering::Acquire), ARMED | CAPTURING) {
                self.invalidate();
            }
            return;
        }
        let Some(count) = self.begin_block(first_sample, left.len(), validity) else {
            return;
        };
        if self.channels.includes_left() && left[..count].iter().any(|value| !value.is_finite())
            || self.channels.includes_right()
                && right[..count].iter().any(|value| !value.is_finite())
        {
            self.invalidate();
            return;
        }
        if self.channels.includes_left() {
            self.left[self.filled..self.filled + count].copy_from_slice(&left[..count]);
        }
        if self.channels.includes_right() {
            self.right[self.filled..self.filled + count].copy_from_slice(&right[..count]);
        }
        self.finish_block(first_sample, count);
    }

    fn capture_resident(&mut self, block: GraphResidentObservationBlock<'_>) {
        if self.state.load(Ordering::Acquire) != ARMED
            && self.state.load(Ordering::Acquire) != CAPTURING
        {
            return;
        }
        let lanes = block.lane.width().lanes() as usize;
        let lane = block.lane.lane();
        let Some(frames) = usize::try_from(block.lane.frames()).ok() else {
            self.invalidate();
            return;
        };
        let Some(words) = frames.checked_mul(lanes) else {
            self.invalidate();
            return;
        };
        let Some(left) = block.lane.left().get(..words) else {
            self.invalidate();
            return;
        };
        let Some(right) = block.lane.right().get(..words) else {
            self.invalidate();
            return;
        };
        let Some(count) = self.begin_block(block.first_sample, frames, block.validity) else {
            return;
        };
        for frame in 0..count {
            let Some(index) = frame
                .checked_mul(lanes)
                .and_then(|value| value.checked_add(lane))
            else {
                self.invalidate();
                return;
            };
            if self.channels.includes_left() {
                let Some(value) = left.get(index).copied() else {
                    self.invalidate();
                    return;
                };
                if !value.is_finite() {
                    self.invalidate();
                    return;
                }
                self.left[self.filled + frame] = value;
            }
            if self.channels.includes_right() {
                let Some(value) = right.get(index).copied() else {
                    self.invalidate();
                    return;
                };
                if !value.is_finite() {
                    self.invalidate();
                    return;
                }
                self.right[self.filled + frame] = value;
            }
        }
        self.finish_block(block.first_sample, count);
    }

    fn invalidate(&mut self) {
        if matches!(self.state.load(Ordering::Acquire), ARMED | CAPTURING) {
            self.armed = false;
            self.state.store(INVALID, Ordering::Release);
        }
    }
    // REALTIME_POLICY_END
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
        math::sqrt(f64::from(value.re).mul_add(
            f64::from(value.re),
            f64::from(value.im) * f64::from(value.im),
        ))
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
        ARMED, SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB, SPECTRUM_WINDOW_FRAMES,
        SpectrumAnalysisError, SpectrumAnalyzer, SpectrumCaptureObserver, SpectrumCaptureReadError,
        SpectrumChannels, SpectrumWindow,
    };
    use dsp_reference::{Complex64, direct_dft_bin, magnitude_db};
    use engine::realtime::{QueueGeneration, bounded_spsc};

    fn window(fill: f32) -> SpectrumWindow {
        SpectrumWindow {
            left: [fill; SPECTRUM_WINDOW_FRAMES],
            right: [fill; SPECTRUM_WINDOW_FRAMES],
            first_sample: 17,
            channels: SpectrumChannels::Stereo,
            source_underrun: false,
        }
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
        let mut observer = SpectrumCaptureObserver {
            producer,
            state: std::sync::Arc::clone(&state),
            left: [0.0; SPECTRUM_WINDOW_FRAMES],
            right: [0.0; SPECTRUM_WINDOW_FRAMES],
            channels: SpectrumChannels::Left,
            filled: 0,
            first_sample: 0,
            next_sample: 0,
            armed: false,
            source_underrun: false,
        };
        let left = [0.25_f32; SPECTRUM_WINDOW_FRAMES];
        let right = [-0.5_f32; SPECTRUM_WINDOW_FRAMES];
        let first_sample = (1_u64 << 53) + 123;
        observer.capture(
            &left,
            &right,
            first_sample,
            super::GraphObservationValidity::CLEAR,
        );
        let window = consumer.try_pop().expect("large-sample capture");
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
