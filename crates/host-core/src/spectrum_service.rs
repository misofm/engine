//! One core service for the one-shot spectrum (issue #1591, slice S28 of decision 17, C13).
//!
//! `spectrum.rs` owns the kernels: the render observer, the capture queues and the analyzer. This
//! module owns what a host does with them for one window:
//!
//! * the **request rules** (target kind, target ID, channels, byte budget) that turn raw words into
//!   a [`SpectrumPreparation`];
//! * the **service** ([`SpectrumService`]) that arms, reads, cancels and selects the prepared
//!   single capture or collection, owns the snapshot token and enforces the ruling that an arm the
//!   caller made is never discarded by a later select or a stream start (D7, issue #1592 D4);
//! * the **stream** (issue #1592): the same service starts, selects, reads and stops the
//!   continuous spectrum and keeps its metadata, cadence and last window;
//! * the **analysis** of one window into dB bins, and of the stream's windows with their
//!   smoothing history ([`SpectrumAnalysis`]).
//!
//! Every [`SpectrumService`] method is allocation-free: the browser calls them inside render-locked
//! exports on its audio thread. [`SpectrumAnalysis`] is worker-side and owns a boxed analyzer, made
//! once. The request constructors allocate and run at preparation, before the host is published.

use crate::spectrum::{
    SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB, SPECTRUM_WINDOW_FRAMES, SpectrumAnalysisHistory,
    SpectrumAnalyzer, SpectrumCadence, SpectrumCadenceError, SpectrumCapture,
    SpectrumCaptureCollection, SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionReadError,
    SpectrumCaptureCollectionRequest, SpectrumCaptureCollectionSelectionError,
    SpectrumCaptureReadError, SpectrumCaptureRequest, SpectrumChannels,
    SpectrumContinuousCaptureError, SpectrumContinuousReadError, SpectrumContinuousWindow,
    SpectrumHop, SpectrumOutput, SpectrumSmoothingConfig, SpectrumTarget, SpectrumTargetRef,
    SpectrumWindow,
};

/// Spectrum capture target at a track's `post_input` tap, after its input section.
pub const SPECTRUM_TARGET_TRACK_POST_INPUT: u32 = 1;
/// Spectrum capture target at a track's `post_pan` tap, after its pan/matrix stage.
pub const SPECTRUM_TARGET_TRACK_POST_PAN: u32 = 2;
/// Spectrum capture target at the designated final output node.
pub const SPECTRUM_TARGET_OUTPUT: u32 = 3;
/// Capture the left plane.
pub const SPECTRUM_CHANNEL_LEFT: u32 = 1;
/// Capture the right plane.
pub const SPECTRUM_CHANNEL_RIGHT: u32 = 2;
/// Capture both planes.
pub const SPECTRUM_CHANNEL_BOTH: u32 = SPECTRUM_CHANNEL_LEFT | SPECTRUM_CHANNEL_RIGHT;
/// Longest stable target identity a request may carry.
pub const SPECTRUM_MAXIMUM_ID_BYTES: usize = 127;
/// Largest byte budget of one single-target request. A collection's budget is an aggregate with
/// no such cap.
pub const SPECTRUM_MAXIMUM_CAPTURE_BYTES: u64 = 1 << 20;

/// Refusal of a raw spectrum request word.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumRequestError {
    /// The target kind is not one of the three codes.
    TargetKind,
    /// The target identity is empty or longer than [`SPECTRUM_MAXIMUM_ID_BYTES`].
    TargetId,
    /// The channel mask is not 1, 2 or 3.
    Channels,
    /// The byte budget is zero, or a single request's budget is above
    /// [`SPECTRUM_MAXIMUM_CAPTURE_BYTES`].
    Budget,
}

impl SpectrumRequestError {
    /// The stable diagnostic code of this refusal.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::TargetKind => "spectrum.target.kind",
            Self::TargetId => "spectrum.target.id",
            Self::Channels => "spectrum.channels",
            Self::Budget => "spectrum.budget",
        }
    }
}

/// Validate a raw target kind and identity and borrow them as a target. Allocates nothing.
///
/// # Errors
///
/// [`SpectrumRequestError::TargetId`] for an empty or over-long identity (checked first) and
/// [`SpectrumRequestError::TargetKind`] for an unknown kind.
pub fn spectrum_target_ref(
    kind: u32,
    id: &str,
) -> Result<SpectrumTargetRef<'_>, SpectrumRequestError> {
    if id.is_empty() || id.len() > SPECTRUM_MAXIMUM_ID_BYTES {
        return Err(SpectrumRequestError::TargetId);
    }
    match kind {
        SPECTRUM_TARGET_TRACK_POST_INPUT => Ok(SpectrumTargetRef::TrackPostInputBuiltins(id)),
        SPECTRUM_TARGET_TRACK_POST_PAN => Ok(SpectrumTargetRef::TrackPostMatrix(id)),
        SPECTRUM_TARGET_OUTPUT => Ok(SpectrumTargetRef::Output(id)),
        _ => Err(SpectrumRequestError::TargetKind),
    }
}

/// The raw kind code of a prepared target.
#[must_use]
pub const fn spectrum_target_kind(target: &SpectrumTarget) -> u32 {
    match target {
        SpectrumTarget::TrackPostInputBuiltins(_) => SPECTRUM_TARGET_TRACK_POST_INPUT,
        SpectrumTarget::TrackPostMatrix(_) => SPECTRUM_TARGET_TRACK_POST_PAN,
        SpectrumTarget::Output(_) => SPECTRUM_TARGET_OUTPUT,
    }
}

/// Validate a raw channel mask. Allocates nothing.
///
/// # Errors
///
/// [`SpectrumRequestError::Channels`] for any mask other than 1, 2 or 3.
pub const fn spectrum_channels_from_raw(
    raw: u32,
) -> Result<SpectrumChannels, SpectrumRequestError> {
    match raw {
        SPECTRUM_CHANNEL_LEFT => Ok(SpectrumChannels::Left),
        SPECTRUM_CHANNEL_RIGHT => Ok(SpectrumChannels::Right),
        SPECTRUM_CHANNEL_BOTH => Ok(SpectrumChannels::Stereo),
        _ => Err(SpectrumRequestError::Channels),
    }
}

fn owned_target(target: SpectrumTargetRef<'_>) -> SpectrumTarget {
    match target {
        SpectrumTargetRef::TrackPostInputBuiltins(id) => {
            SpectrumTarget::TrackPostInputBuiltins(id.into())
        }
        SpectrumTargetRef::TrackPostMatrix(id) => SpectrumTarget::TrackPostMatrix(id.into()),
        SpectrumTargetRef::Output(id) => SpectrumTarget::Output(id.into()),
    }
}

/// Spectrum observer preparation: one fixed observer, or several for atomic selection.
///
/// The single form is the compatibility path. The collection form prepares every exact
/// target/mask before the host is published and lets one control-side owner select among them.
#[derive(Clone, Debug)]
pub enum SpectrumPreparation {
    /// Prepare one fixed spectrum observer.
    Single(SpectrumCaptureRequest),
    /// Prepare several exact observers for atomic selection.
    Collection(SpectrumCaptureCollectionRequest),
}

impl SpectrumPreparation {
    /// Build a single-target preparation from raw words.
    ///
    /// The checks run in this order: identity, kind, channel mask, then a budget from 1 byte to
    /// [`SPECTRUM_MAXIMUM_CAPTURE_BYTES`].
    ///
    /// # Errors
    ///
    /// The first failing [`SpectrumRequestError`].
    pub fn single(
        kind: u32,
        id: &str,
        channels: u32,
        maximum_capture_bytes: u64,
    ) -> Result<Self, SpectrumRequestError> {
        let target = owned_target(spectrum_target_ref(kind, id)?);
        let channels = spectrum_channels_from_raw(channels)?;
        if maximum_capture_bytes == 0 || maximum_capture_bytes > SPECTRUM_MAXIMUM_CAPTURE_BYTES {
            return Err(SpectrumRequestError::Budget);
        }
        Ok(Self::Single(SpectrumCaptureRequest {
            target,
            channels,
            maximum_capture_bytes,
        }))
    }

    /// Build a collection preparation from raw `(kind, id, channels)` entries.
    ///
    /// A collection's budget is an aggregate over all of its entries: it must be non-zero and has
    /// no 1 MiB cap. The budget is checked before any entry. An empty collection takes budget 0
    /// only and returns `None`: no spectrum is prepared. Duplicate entries are accepted here and
    /// refused by preparation (`host.spectrum.duplicate`).
    ///
    /// # Errors
    ///
    /// The first failing [`SpectrumRequestError`]; [`SpectrumRequestError::Budget`] also when the
    /// entry list cannot be reserved.
    pub fn collection(
        entries: &[(u32, &str, u32)],
        maximum_capture_bytes: u64,
    ) -> Result<Option<Self>, SpectrumRequestError> {
        if entries.is_empty() {
            return if maximum_capture_bytes == 0 {
                Ok(None)
            } else {
                Err(SpectrumRequestError::Budget)
            };
        }
        if maximum_capture_bytes == 0 {
            return Err(SpectrumRequestError::Budget);
        }
        let mut prepared = Vec::new();
        prepared
            .try_reserve_exact(entries.len())
            .map_err(|_| SpectrumRequestError::Budget)?;
        for &(kind, id, channels) in entries {
            prepared.push(SpectrumCaptureCollectionEntry {
                target: owned_target(spectrum_target_ref(kind, id)?),
                channels: spectrum_channels_from_raw(channels)?,
            });
        }
        Ok(Some(Self::Collection(SpectrumCaptureCollectionRequest {
            entries: prepared,
            maximum_capture_bytes,
        })))
    }
}

/// The prepared observer or collection that a [`SpectrumService`] owns.
pub enum PreparedSpectrumCaptures {
    /// One fixed spectrum observer.
    Single(SpectrumCapture),
    /// Several exact observers, one selected at a time.
    Collection(SpectrumCaptureCollection),
}

impl PreparedSpectrumCaptures {
    // REALTIME_POLICY_BEGIN: the one-shot methods the browser's render-locked exports call.
    fn arm(&self) -> Result<(), SpectrumServiceError> {
        let armed = match self {
            Self::Single(capture) => capture.arm(),
            Self::Collection(capture) => capture.arm_selected(),
        };
        armed.map_err(|_| SpectrumServiceError::Busy)
    }

    fn try_read(&mut self) -> Result<SpectrumWindow, SpectrumCaptureReadError> {
        match self {
            Self::Single(capture) => capture.try_read(),
            Self::Collection(capture) => capture.try_read().map_err(|error| match error {
                SpectrumCaptureCollectionReadError::NoSelection => {
                    SpectrumCaptureReadError::NotArmed
                }
                SpectrumCaptureCollectionReadError::Capture(error) => error,
            }),
        }
    }

    fn cancel(&mut self) {
        match self {
            Self::Single(capture) => capture.cancel(),
            Self::Collection(capture) => capture.cancel(),
        }
    }

    /// The selected target: the single capture's, or the collection's selected entry.
    #[must_use]
    pub fn target(&self) -> Option<&SpectrumTarget> {
        match self {
            Self::Single(capture) => Some(capture.target()),
            Self::Collection(capture) => capture.selected_target(),
        }
    }

    /// The selected channel mask. Allocates nothing.
    #[must_use]
    pub fn channels(&self) -> Option<SpectrumChannels> {
        match self {
            Self::Single(capture) => Some(capture.channels()),
            Self::Collection(capture) => capture.selected_channels(),
        }
    }

    // The stream half. The browser calls it through the service, inside render-locked exports.
    fn start_continuous(
        &mut self,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Result<SpectrumCadence, SpectrumContinuousCaptureError> {
        match self {
            Self::Single(capture) => capture.start_continuous(sample_rate_hz, quantum_frames),
            Self::Collection(capture) => capture.start_continuous(sample_rate_hz, quantum_frames),
        }
    }

    fn start_continuous_with_hop(
        &mut self,
        sample_rate_hz: u32,
        quantum_frames: u32,
        hop: SpectrumHop,
    ) -> Result<SpectrumCadence, SpectrumContinuousCaptureError> {
        match self {
            Self::Single(capture) => {
                capture.start_continuous_with_hop(sample_rate_hz, quantum_frames, hop)
            }
            Self::Collection(capture) => {
                capture.start_continuous_with_hop(sample_rate_hz, quantum_frames, hop)
            }
        }
    }

    fn restart_continuous(&mut self) -> Result<(), SpectrumContinuousCaptureError> {
        match self {
            Self::Single(capture) => capture.restart_continuous(),
            Self::Collection(capture) => capture.restart_continuous(),
        }
    }

    fn try_read_continuous(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        match self {
            Self::Single(capture) => capture.try_read_continuous(),
            Self::Collection(capture) => capture.try_read_continuous(),
        }
    }

    fn stop_continuous(&mut self) {
        match self {
            Self::Single(capture) => capture.stop_continuous(),
            Self::Collection(capture) => capture.stop_continuous(),
        }
    }

    fn stream_epoch(&self) -> Option<u64> {
        match self {
            Self::Single(capture) => capture.stream_epoch(),
            Self::Collection(capture) => capture.stream_epoch(),
        }
    }
    // REALTIME_POLICY_END
}

impl From<SpectrumCapture> for PreparedSpectrumCaptures {
    fn from(capture: SpectrumCapture) -> Self {
        Self::Single(capture)
    }
}

impl From<SpectrumCaptureCollection> for PreparedSpectrumCaptures {
    fn from(capture: SpectrumCaptureCollection) -> Self {
        Self::Collection(capture)
    }
}

/// Refusal of a [`SpectrumService`] or [`SpectrumAnalysis`] call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpectrumServiceError {
    /// No spectrum observer was prepared.
    Unprepared,
    /// A single preparation has no selection.
    SelectSingle,
    /// Another capture holds the arm, or the caller's armed window would be discarded.
    Busy,
    /// No capture is armed, or no entry is selected.
    NotArmed,
    /// The graph window was discontinuous, nonfinite or otherwise unusable.
    CaptureInvalid,
    /// The snapshot token or the selection epoch cannot advance without reuse.
    Exhausted,
    /// No prepared entry matches the target and mask exactly.
    UnknownEntry,
    /// The requested channels are not a subset of the selected entry's mask.
    Channels,
    /// A snapshot fails its semantic checks, or the analyzer refused it.
    WindowInvalid,
    /// A stream runs, so the call (a one-shot cancel) would end it silently.
    StreamActive,
    /// No stream runs.
    StreamInactive,
    /// The stream rate is not a launch rate.
    CadenceRate,
    /// The stream quantum or hop cannot define a profile.
    CadenceShape,
    /// The smoothing duration is outside its range.
    Smoothing,
    /// The imported stream metadata fails its semantic checks.
    MetadataInvalid,
}

impl SpectrumServiceError {
    /// The stable diagnostic code of this refusal.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Unprepared => "spectrum.unprepared",
            Self::SelectSingle => "spectrum.select.single",
            Self::Busy => "spectrum.busy",
            Self::NotArmed => "spectrum.not_armed",
            Self::CaptureInvalid => "spectrum.capture.invalid",
            Self::Exhausted => "spectrum.exhausted",
            Self::UnknownEntry => "spectrum.entry.unknown",
            Self::Channels => "spectrum.channels",
            Self::WindowInvalid => "spectrum.window.invalid",
            Self::StreamActive => "spectrum.stream.active",
            Self::StreamInactive => "spectrum.stream.inactive",
            Self::CadenceRate => "spectrum.cadence.rate",
            Self::CadenceShape => "spectrum.cadence.shape",
            Self::Smoothing => "spectrum.smoothing",
            Self::MetadataInvalid => "spectrum.metadata.invalid",
        }
    }
}

fn selection_error(error: SpectrumCaptureCollectionSelectionError) -> SpectrumServiceError {
    match error {
        SpectrumCaptureCollectionSelectionError::UnknownEntry => SpectrumServiceError::UnknownEntry,
        SpectrumCaptureCollectionSelectionError::Busy => SpectrumServiceError::Busy,
        SpectrumCaptureCollectionSelectionError::EpochOverflow => SpectrumServiceError::Exhausted,
    }
}

fn capture_error(error: SpectrumContinuousCaptureError) -> SpectrumServiceError {
    match error {
        SpectrumContinuousCaptureError::Busy => SpectrumServiceError::Busy,
        SpectrumContinuousCaptureError::Cadence(SpectrumCadenceError::UnsupportedRate) => {
            SpectrumServiceError::CadenceRate
        }
        SpectrumContinuousCaptureError::Cadence(
            SpectrumCadenceError::ZeroQuantum | SpectrumCadenceError::UnsupportedHop,
        ) => SpectrumServiceError::CadenceShape,
        SpectrumContinuousCaptureError::Cadence(SpectrumCadenceError::HopOverflow)
        | SpectrumContinuousCaptureError::EpochOverflow => SpectrumServiceError::Exhausted,
    }
}

/// Validate a smoothing duration in milliseconds.
///
/// # Errors
///
/// [`SpectrumServiceError::Smoothing`] unless the value is finite and within 0 through 10,000.
pub fn spectrum_smoothing(
    smoothing_ms: f64,
) -> Result<SpectrumSmoothingConfig, SpectrumServiceError> {
    SpectrumSmoothingConfig::new(smoothing_ms).map_err(|_| SpectrumServiceError::Smoothing)
}

/// The state of the continuous stream. The codes are the browser's `SPECTRUM_STREAM_STATUS_*`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SpectrumStreamStatus {
    /// No stream has run, or a read found it not running.
    #[default]
    Inactive = 0,
    /// The stream runs and has not observed a complete first window.
    Warming = 1,
    /// The stream runs and no completed window is queued.
    Pending = 2,
    /// The bounded queue dropped completed windows.
    Gap = 3,
    /// A discontinuity or a failed render reset the stream history.
    Failed = 4,
    /// The stream was stopped after a successful start.
    Stopped = 5,
    /// A completed window is held for the reader.
    Ready = 6,
}

impl SpectrumStreamStatus {
    /// The stable numeric code of this status.
    #[must_use]
    pub const fn code(self) -> u32 {
        self as u32
    }

    /// The status of a numeric code, or `None` for an unknown code.
    #[must_use]
    pub const fn from_code(code: u32) -> Option<Self> {
        Some(match code {
            0 => Self::Inactive,
            1 => Self::Warming,
            2 => Self::Pending,
            3 => Self::Gap,
            4 => Self::Failed,
            5 => Self::Stopped,
            6 => Self::Ready,
            _ => return None,
        })
    }
}

impl SpectrumStreamMetadata {
    /// Record a read that produced no window: the status, the epoch and the drops it names. A
    /// failed window starts a fresh capture epoch, so without a drop count its count is zero; an
    /// ordinary pending or warming read keeps the accumulated drops of the current epoch.
    fn record_outcome(
        &mut self,
        status: SpectrumStreamStatus,
        epoch: Option<u64>,
        drops: Option<u64>,
    ) {
        self.status = status;
        if let Some(epoch) = epoch {
            self.capture_epoch = epoch;
        }
        if let Some(drops) = drops {
            self.dropped_captures = drops;
        } else if status == SpectrumStreamStatus::Failed && epoch.is_some() {
            self.dropped_captures = 0;
        }
        self.source_underrun = false;
    }
}

/// The facts of the stream and of its last window: the browser's `WebSpectrumStreamMetadata`
/// without its record words.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpectrumStreamMetadata {
    /// The state of the stream.
    pub status: SpectrumStreamStatus,
    /// The raw target kind code, or zero without a selection.
    pub target_kind: u32,
    /// The raw channel mask, or zero without a selection.
    pub channels: u32,
    /// The stream rate.
    pub sample_rate_hz: u32,
    /// The render quantum.
    pub quantum_frames: u32,
    /// The sample-clock spacing of scheduled windows.
    pub hop_frames: u32,
    /// Whether a source underrun was known while the last window was captured.
    pub source_underrun: bool,
    /// The capture epoch; a discontinuity starts a new one.
    pub capture_epoch: u64,
    /// The zero-based window sequence within the capture epoch.
    pub sequence: u64,
    /// Complete windows the bounded queue dropped in this capture epoch.
    pub dropped_captures: u64,
    /// The windows of this capture epoch so far, `sequence + 1` once one was read.
    pub windows: u64,
    /// Absolute sample at the start of the last window.
    pub captured_sample: u64,
    /// Exclusive absolute sample at the end of the last window.
    pub end_sample: u64,
    /// The epoch of the smoothing history, zero without an analysis.
    pub analysis_epoch: u64,
    /// The first sample the smoothing history holds, zero when it is empty.
    pub history_start_sample: u64,
    /// The smoothing duration in milliseconds.
    pub smoothing_ms: f64,
}

/// One completed window with the facts an analysis needs.
#[derive(Clone, Copy, Debug)]
pub struct SpectrumSnapshot {
    /// The completed window. Its `channels` is the mask the reader asked for.
    pub window: SpectrumWindow,
    /// The raw target kind code of the captured boundary.
    pub target_kind: u32,
    /// The session rate the window was captured at.
    pub sample_rate_hz: u32,
    /// The host-scoped monotonic token of this window, from 1.
    pub token: u64,
}

impl SpectrumSnapshot {
    /// Check the semantic facts of a snapshot: a valid target kind, a representable end sample, a
    /// non-zero token and a launch sample rate. The mask is a [`SpectrumChannels`] value, so it is
    /// never empty.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::WindowInvalid`].
    pub fn validate(&self) -> Result<(), SpectrumServiceError> {
        if !(SPECTRUM_TARGET_TRACK_POST_INPUT..=SPECTRUM_TARGET_OUTPUT).contains(&self.target_kind)
            || self.window.end_sample().is_none()
            || self.token == 0
            || !engine::is_launch_sample_rate(engine::SampleRateHz(self.sample_rate_hz))
        {
            return Err(SpectrumServiceError::WindowInvalid);
        }
        Ok(())
    }
}

/// The spectrum of one host: the prepared captures, the snapshot token, the arm ownership and the
/// continuous stream.
///
/// Every method is allocation-free. See the module documentation and decision 17 D7: an arm that
/// the caller made with [`Self::arm`] belongs to the caller until a read returns its window or
/// [`SpectrumServiceError::CaptureInvalid`], or until [`Self::cancel`]. A stream start refuses
/// `Busy` while that arm stands (D4 of issue #1592); an arm that [`Self::select`] made is not the
/// caller's and the start replaces it.
///
/// The stream (issue #1592) has one state machine here: [`Self::start_stream`],
/// [`Self::select_stream`], [`Self::read_stream`] and [`Self::stop_stream`] keep the metadata, the
/// last window and the cadence. The smoothing history lives in a [`SpectrumAnalysis`]. A caller
/// that owns both halves passes it to each call; a caller that owns only the capture half (the
/// browser's audio thread) passes `None`.
pub struct SpectrumService {
    captures: PreparedSpectrumCaptures,
    token: u64,
    sample_rate_hz: u32,
    quantum_frames: u32,
    hop: Option<SpectrumHop>,
    caller_armed: bool,
    stream_active: bool,
    stream_cadence: Option<SpectrumCadence>,
    stream_smoothing: Option<SpectrumSmoothingConfig>,
    stream_metadata: SpectrumStreamMetadata,
    stream_window: Box<SpectrumContinuousWindow>,
    stream_window_ready: bool,
}

impl SpectrumService {
    /// Own `captures` for a session at `sample_rate_hz` and `quantum_frames`. A stream starts with
    /// `hop` when the preparation named one, else with the default cadence of the session shape.
    ///
    /// Allocates the one boxed stream window, here and never again.
    #[must_use]
    pub fn new(
        captures: PreparedSpectrumCaptures,
        sample_rate_hz: u32,
        quantum_frames: u32,
        hop: Option<SpectrumHop>,
    ) -> Self {
        Self {
            captures,
            token: 0,
            sample_rate_hz,
            quantum_frames,
            hop,
            caller_armed: false,
            stream_active: false,
            stream_cadence: None,
            stream_smoothing: None,
            stream_metadata: SpectrumStreamMetadata::default(),
            stream_window: Box::new(SpectrumContinuousWindow {
                left: [0.0; SPECTRUM_WINDOW_FRAMES],
                right: [0.0; SPECTRUM_WINDOW_FRAMES],
                first_sample: 0,
                channels: SpectrumChannels::Left,
                source_underrun: false,
                stream_epoch: 0,
                sequence: 0,
                dropped_captures: 0,
            }),
            stream_window_ready: false,
        }
    }

    /// The session rate this service stamps on its snapshots.
    #[must_use]
    pub const fn sample_rate_hz(&self) -> u32 {
        self.sample_rate_hz
    }

    /// The session quantum, in frames.
    #[must_use]
    pub const fn quantum_frames(&self) -> u32 {
        self.quantum_frames
    }

    /// The explicit stream hop of the preparation, if it named one.
    #[must_use]
    pub const fn hop(&self) -> Option<SpectrumHop> {
        self.hop
    }

    // REALTIME_POLICY_BEGIN: arm, read, cancel, select and the stream calls run inside render-locked exports.
    /// Arm the selected capture for its next complete window. A success makes the arm the
    /// caller's.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::Busy`] when the capture is armed, holds an unread window or runs a
    /// stream, and when a collection has no selected entry.
    pub fn arm(&mut self) -> Result<(), SpectrumServiceError> {
        self.captures.arm()?;
        self.caller_armed = true;
        Ok(())
    }

    /// Take the completed window, if the capture has finished.
    ///
    /// Before it pops, the read checks that an entry is selected, that `requested` is a subset of
    /// the selected mask and that the token can advance; a refusal at that point leaves the window
    /// queued. A pending capture is `Ok(None)`. A window advances the token and ends the caller's
    /// arm; the snapshot's window carries `requested` as its mask.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::NotArmed`], [`SpectrumServiceError::Channels`],
    /// [`SpectrumServiceError::Exhausted`] or [`SpectrumServiceError::CaptureInvalid`].
    pub fn read(
        &mut self,
        requested: SpectrumChannels,
    ) -> Result<Option<SpectrumSnapshot>, SpectrumServiceError> {
        let Some(selected) = self.captures.channels() else {
            return Err(SpectrumServiceError::NotArmed);
        };
        if (requested as u32) & !(selected as u32) != 0 {
            return Err(SpectrumServiceError::Channels);
        }
        let Some(target_kind) = self.captures.target().map(spectrum_target_kind) else {
            return Err(SpectrumServiceError::NotArmed);
        };
        let token = self
            .token
            .checked_add(1)
            .ok_or(SpectrumServiceError::Exhausted)?;
        match self.captures.try_read() {
            Ok(mut window) => {
                self.token = token;
                self.caller_armed = false;
                window.channels = requested;
                Ok(Some(SpectrumSnapshot {
                    window,
                    target_kind,
                    sample_rate_hz: self.sample_rate_hz,
                    token,
                }))
            }
            Err(SpectrumCaptureReadError::Pending) => Ok(None),
            Err(SpectrumCaptureReadError::NotArmed) => {
                self.caller_armed = false;
                Err(SpectrumServiceError::NotArmed)
            }
            Err(SpectrumCaptureReadError::Invalid) => {
                self.caller_armed = false;
                Err(SpectrumServiceError::CaptureInvalid)
            }
        }
    }

    /// Cancel a pending capture, discard a completed window and end the caller's arm.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::StreamActive`] while a stream runs; nothing changes.
    pub fn cancel(&mut self) -> Result<(), SpectrumServiceError> {
        if self.stream_active {
            return Err(SpectrumServiceError::StreamActive);
        }
        self.captures.cancel();
        self.caller_armed = false;
        Ok(())
    }

    /// Validate a selection and report whether it would replace the selected entry.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::SelectSingle`] on a single preparation,
    /// [`SpectrumServiceError::UnknownEntry`], [`SpectrumServiceError::Exhausted`], and
    /// [`SpectrumServiceError::Busy`] when the change would discard the caller's armed window.
    pub fn selection_would_change(
        &self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
    ) -> Result<bool, SpectrumServiceError> {
        let PreparedSpectrumCaptures::Collection(collection) = &self.captures else {
            return Err(SpectrumServiceError::SelectSingle);
        };
        let would_change = collection
            .selection_would_change(target, channels)
            .map_err(selection_error)?;
        if would_change && self.caller_armed {
            return Err(SpectrumServiceError::Busy);
        }
        Ok(would_change)
    }

    /// Atomically select one exact prepared entry. Returns `true` when the selection changed.
    ///
    /// The collection checks the entry and the epoch before it retires the current capture, so a
    /// refusal changes nothing. A select that would replace the entry while the caller holds an
    /// arm is `Busy` and changes nothing (D7); a select of the already selected entry changes
    /// nothing and is accepted. An arm that an earlier `select` made is not the caller's and is
    /// replaced. While a stream runs, [`Self::select_stream`] selects: it also restarts the
    /// metadata and the history.
    ///
    /// # Errors
    ///
    /// As [`Self::selection_would_change`], and [`SpectrumServiceError::StreamActive`] while a
    /// stream runs.
    pub fn select(
        &mut self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
    ) -> Result<bool, SpectrumServiceError> {
        if self.stream_active {
            return Err(SpectrumServiceError::StreamActive);
        }
        self.select_entry(target, channels)
    }

    fn select_entry(
        &mut self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
    ) -> Result<bool, SpectrumServiceError> {
        let would_change = self.selection_would_change(target, channels)?;
        let PreparedSpectrumCaptures::Collection(collection) = &mut self.captures else {
            return Err(SpectrumServiceError::SelectSingle);
        };
        collection
            .select(target, channels)
            .map_err(selection_error)?;
        Ok(would_change)
    }

    /// The monotonic identity of the committed selection, or zero on a single preparation.
    #[must_use]
    pub fn selection_epoch(&self) -> u64 {
        match &self.captures {
            PreparedSpectrumCaptures::Single(_) => 0,
            PreparedSpectrumCaptures::Collection(collection) => collection.selection_epoch(),
        }
    }

    /// The selected target, if any.
    #[must_use]
    pub fn target(&self) -> Option<&SpectrumTarget> {
        self.captures.target()
    }

    /// The selected target's raw kind code, or zero without a selection.
    #[must_use]
    pub fn target_kind(&self) -> u32 {
        self.captures.target().map_or(0, spectrum_target_kind)
    }

    /// The selected channel mask, if any.
    #[must_use]
    pub fn channels(&self) -> Option<SpectrumChannels> {
        self.captures.channels()
    }

    /// Whether the stream runs.
    #[must_use]
    pub const fn stream_active(&self) -> bool {
        self.stream_active
    }

    /// The facts of the stream and of its last read.
    #[must_use]
    pub const fn stream_metadata(&self) -> &SpectrumStreamMetadata {
        &self.stream_metadata
    }

    /// The window that the last [`Self::read_stream`] returned `Ready`, until the next call that
    /// changes the stream.
    #[must_use]
    pub fn stream_window(&self) -> Option<&SpectrumContinuousWindow> {
        self.stream_window_ready.then_some(&*self.stream_window)
    }

    /// The cadence of the running stream.
    #[must_use]
    pub const fn stream_cadence(&self) -> Option<SpectrumCadence> {
        self.stream_cadence
    }

    /// The capture epoch of the running stream.
    #[must_use]
    pub fn stream_epoch(&self) -> Option<u64> {
        self.captures.stream_epoch()
    }

    /// Start the stream on the selected entry and make its metadata `Warming`.
    ///
    /// Every check runs before the first write. The stream starts with the preparation's hop, or
    /// with the default cadence of the session shape. With `analysis`, its smoothing history is
    /// reset at the same commit.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::Exhausted`] when the analysis epoch cannot advance or the capture
    /// epoch or hop overflows; [`SpectrumServiceError::Busy`] when the caller holds an arm, a
    /// stream already runs, a single capture is not idle or a collection has no selection;
    /// [`SpectrumServiceError::CadenceRate`] and [`SpectrumServiceError::CadenceShape`] for a
    /// session shape that defines no profile.
    pub fn start_stream(
        &mut self,
        smoothing: SpectrumSmoothingConfig,
        mut analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), SpectrumServiceError> {
        if analysis
            .as_deref()
            .is_some_and(SpectrumAnalysis::history_exhausted)
        {
            return Err(SpectrumServiceError::Exhausted);
        }
        // An arm that the caller made keeps its window (decision 17 D4 of issue #1592).
        if self.caller_armed {
            return Err(SpectrumServiceError::Busy);
        }
        let started = match self.hop {
            Some(hop) => self.captures.start_continuous_with_hop(
                self.sample_rate_hz,
                self.quantum_frames,
                hop,
            ),
            None => self
                .captures
                .start_continuous(self.sample_rate_hz, self.quantum_frames),
        };
        let cadence = started.map_err(capture_error)?;
        if let Some(analysis) = analysis.as_deref_mut()
            && analysis.begin_stream(cadence, smoothing).is_err()
        {
            // The preflight makes this unreachable; the stream must not outlive its refusal.
            self.captures.stop_continuous();
            return Err(SpectrumServiceError::Exhausted);
        }
        self.stream_active = true;
        self.stream_cadence = Some(cadence);
        self.stream_smoothing = Some(smoothing);
        self.stream_window_ready = false;
        self.stream_metadata = SpectrumStreamMetadata {
            status: SpectrumStreamStatus::Warming,
            target_kind: self.target_kind(),
            channels: self.channels().map_or(0, |channels| channels as u32),
            sample_rate_hz: cadence.sample_rate_hz(),
            quantum_frames: cadence.quantum_frames(),
            hop_frames: cadence.hop_frames(),
            capture_epoch: self.captures.stream_epoch().unwrap_or(0),
            smoothing_ms: smoothing.smoothing_ms(),
            ..SpectrumStreamMetadata::default()
        };
        self.refresh_analysis_facts(analysis.as_deref());
        Ok(())
    }

    /// Select one exact prepared entry of the running stream and, with `smoothing`, change its
    /// smoothing. Returns whether the stream restarted (the entry or the smoothing changed): its
    /// metadata is then `Warming`, its capture epoch is fresh and its history is reset.
    ///
    /// The smoothing history, the capture selection and the metadata change as one bounded
    /// operation: every check runs before the first write. `None` keeps the smoothing.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::StreamInactive`] without a running stream, and as
    /// [`Self::selection_would_change`]; [`SpectrumServiceError::Exhausted`] when the analysis
    /// epoch cannot advance.
    pub fn select_stream(
        &mut self,
        target: SpectrumTargetRef<'_>,
        channels: SpectrumChannels,
        smoothing: Option<SpectrumSmoothingConfig>,
        mut analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<bool, SpectrumServiceError> {
        if !self.stream_active {
            return Err(SpectrumServiceError::StreamInactive);
        }
        let smoothing_changed = smoothing.is_some() && self.stream_smoothing != smoothing;
        let selection_changed = self.selection_would_change(target, channels)?;
        if (selection_changed || smoothing_changed)
            && analysis
                .as_deref()
                .is_some_and(SpectrumAnalysis::history_exhausted)
        {
            return Err(SpectrumServiceError::Exhausted);
        }
        self.select_entry(target, channels)?;
        if smoothing_changed && !selection_changed {
            self.captures.restart_continuous().map_err(capture_error)?;
        }
        if !selection_changed && !smoothing_changed {
            return Ok(false);
        }
        // The preflight above makes this reset infallible at the commit point.
        if let Some(analysis) = analysis.as_deref_mut()
            && analysis.reset_history().is_err()
        {
            return Err(SpectrumServiceError::Exhausted);
        }
        if let Some(smoothing) = smoothing {
            self.stream_smoothing = Some(smoothing);
            self.stream_metadata.smoothing_ms = smoothing.smoothing_ms();
            if let Some(analysis) = analysis.as_deref_mut() {
                analysis.smoothing = Some(smoothing);
            }
        }
        if selection_changed {
            self.stream_metadata.target_kind = self.target_kind();
            self.stream_metadata.channels = self.channels().map_or(0, |channels| channels as u32);
        }
        let metadata = &mut self.stream_metadata;
        metadata.status = SpectrumStreamStatus::Warming;
        metadata.capture_epoch = self.captures.stream_epoch().unwrap_or(0);
        metadata.sequence = 0;
        metadata.dropped_captures = 0;
        metadata.windows = 0;
        metadata.captured_sample = 0;
        metadata.end_sample = 0;
        metadata.source_underrun = false;
        self.stream_window_ready = false;
        self.refresh_analysis_facts(analysis.as_deref());
        Ok(true)
    }

    /// Read the stream once and keep the outcome in [`Self::stream_metadata`]. A ready window
    /// lands in [`Self::stream_window`].
    ///
    /// A status other than `Ready` is not an error: the caller reads it from the metadata. A gap
    /// or a failure resets `analysis`. The only counters this changes are the capture's own read
    /// counters (gap, recovery, failure), by returning a status; a caller that cannot deliver the
    /// status must not call it.
    ///
    /// # Errors
    ///
    /// The metadata is `Failed` and the call refuses with
    /// [`SpectrumServiceError::Exhausted`] when the analysis epoch or the window count overflows,
    /// [`SpectrumServiceError::WindowInvalid`] when the window has no end sample and
    /// [`SpectrumServiceError::StreamInactive`] when the stream lost its cadence.
    pub fn read_stream(
        &mut self,
        mut analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), SpectrumServiceError> {
        // A collection selection can change between reads: report the committed entry on every
        // path, so the metadata cannot keep the previous target.
        self.stream_metadata.target_kind = self.target_kind();
        self.stream_metadata.channels = self.channels().map_or(0, |channels| channels as u32);
        self.stream_window_ready = false;
        if let Some(analysis) = analysis.as_deref_mut() {
            analysis.window_known = false;
        }
        let result = match self.captures.try_read_continuous() {
            Ok(window) => self.keep_stream_window(window, analysis.as_deref_mut()),
            Err(error) => self.stream_unavailable(error, analysis.as_deref_mut()),
        };
        self.refresh_analysis_facts(analysis.as_deref());
        result
    }

    fn stream_unavailable(
        &mut self,
        error: SpectrumContinuousReadError,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), SpectrumServiceError> {
        let (status, epoch, drops) = match error {
            SpectrumContinuousReadError::NotActive => (SpectrumStreamStatus::Inactive, None, None),
            SpectrumContinuousReadError::Warming => (SpectrumStreamStatus::Warming, None, None),
            SpectrumContinuousReadError::Pending => (SpectrumStreamStatus::Pending, None, None),
            SpectrumContinuousReadError::Gap {
                stream_epoch,
                dropped_captures,
            } => (
                SpectrumStreamStatus::Gap,
                Some(stream_epoch),
                Some(dropped_captures),
            ),
            SpectrumContinuousReadError::Failed { stream_epoch } => {
                (SpectrumStreamStatus::Failed, Some(stream_epoch), None)
            }
        };
        if matches!(
            error,
            SpectrumContinuousReadError::Gap { .. } | SpectrumContinuousReadError::Failed { .. }
        ) && analysis.is_some_and(|analysis| analysis.reset_history().is_err())
        {
            self.stream_outcome(SpectrumStreamStatus::Failed, epoch, drops);
            return Err(SpectrumServiceError::Exhausted);
        }
        self.stream_outcome(status, epoch, drops);
        Ok(())
    }

    fn keep_stream_window(
        &mut self,
        window: SpectrumContinuousWindow,
        analysis: Option<&mut SpectrumAnalysis>,
    ) -> Result<(), SpectrumServiceError> {
        let epoch = Some(window.stream_epoch);
        let drops = Some(window.dropped_captures);
        let facts = if self.stream_cadence.is_none() {
            Err(SpectrumServiceError::StreamInactive)
        } else {
            match (window.sequence.checked_add(1), window.end_sample()) {
                (Some(windows), Some(end_sample)) => Ok((windows, end_sample)),
                (None, _) => Err(SpectrumServiceError::Exhausted),
                (_, None) => Err(SpectrumServiceError::WindowInvalid),
            }
        };
        let (windows, end_sample) = match facts {
            Ok(facts) => facts,
            Err(failure) => {
                self.stream_outcome(SpectrumStreamStatus::Failed, epoch, drops);
                return Err(failure);
            }
        };
        let metadata = &mut self.stream_metadata;
        metadata.status = SpectrumStreamStatus::Ready;
        metadata.capture_epoch = window.stream_epoch;
        metadata.sequence = window.sequence;
        metadata.dropped_captures = window.dropped_captures;
        metadata.windows = windows;
        metadata.captured_sample = window.first_sample;
        metadata.end_sample = end_sample;
        metadata.source_underrun = window.source_underrun;
        *self.stream_window = window;
        self.stream_window_ready = true;
        if let Some(analysis) = analysis {
            analysis.window_known = true;
        }
        Ok(())
    }

    /// Record a read that produced no window.
    fn stream_outcome(
        &mut self,
        status: SpectrumStreamStatus,
        epoch: Option<u64>,
        drops: Option<u64>,
    ) {
        self.stream_metadata.record_outcome(status, epoch, drops);
        self.stream_window_ready = false;
    }

    /// Copy the analysis facts into the metadata: zero without an analysis.
    fn refresh_analysis_facts(&mut self, analysis: Option<&SpectrumAnalysis>) {
        self.stream_metadata.analysis_epoch = analysis.map_or(0, SpectrumAnalysis::analysis_epoch);
        self.stream_metadata.history_start_sample =
            analysis.map_or(0, SpectrumAnalysis::history_start_sample);
    }

    /// Stop the stream, discard its queued window and make the metadata `Stopped`. Allowed
    /// without a running stream.
    pub fn stop_stream(&mut self) {
        self.captures.stop_continuous();
        self.stream_active = false;
        self.stream_cadence = None;
        self.stream_smoothing = None;
        self.stream_window_ready = false;
        self.stream_metadata.status = SpectrumStreamStatus::Stopped;
    }
    // REALTIME_POLICY_END
}

/// The words of one analyzed window, in caller-owned storage.
#[derive(Clone, Debug, PartialEq)]
pub struct SpectrumAnalysisResult {
    /// The raw target kind code.
    pub target_kind: u32,
    /// The analyzed channel mask.
    pub channels: SpectrumChannels,
    /// The session rate.
    pub sample_rate_hz: u32,
    /// Absolute sample at the start of the window.
    pub captured_sample: u64,
    /// Exclusive absolute sample at the end of the window.
    pub end_sample: u64,
    /// The snapshot token.
    pub token: u64,
    /// Whether a source underrun was known while the window was captured.
    pub source_underrun: bool,
    /// The frequency axis in hertz, DC through Nyquist.
    pub frequencies_hz: [f32; SPECTRUM_BIN_COUNT],
    /// Left amplitude in dBFS. A plane outside `channels` holds the floor and is not read.
    pub left_dbfs: [f32; SPECTRUM_BIN_COUNT],
    /// Right amplitude in dBFS. A plane outside `channels` holds the floor and is not read.
    pub right_dbfs: [f32; SPECTRUM_BIN_COUNT],
}

impl SpectrumAnalysisResult {
    /// An empty result: a zero axis and floor planes.
    #[must_use]
    pub fn new() -> Self {
        Self {
            target_kind: 0,
            channels: SpectrumChannels::Left,
            sample_rate_hz: 0,
            captured_sample: 0,
            end_sample: 0,
            token: 0,
            source_underrun: false,
            frequencies_hz: [0.0; SPECTRUM_BIN_COUNT],
            left_dbfs: [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT],
            right_dbfs: [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT],
        }
    }
}

impl Default for SpectrumAnalysisResult {
    fn default() -> Self {
        Self::new()
    }
}

/// The worker-side analysis: one boxed analyzer, made once, and the stream's smoothing history.
///
/// The history is boxed once, on the first stream window that is analyzed or imported; a
/// one-shot-only analysis never pays for it. The stream state (running, cadence, smoothing and
/// whether a window is at hand) is the half of the stream that a [`SpectrumService`] cannot hold
/// when the analysis runs in another instance.
pub struct SpectrumAnalysis {
    analyzer: Box<SpectrumAnalyzer>,
    history: Option<Box<SpectrumAnalysisHistory>>,
    stream_active: bool,
    cadence: Option<SpectrumCadence>,
    smoothing: Option<SpectrumSmoothingConfig>,
    window_known: bool,
}

impl SpectrumAnalysis {
    /// Prepare the analyzer on the control or worker side.
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyzer: Box::new(SpectrumAnalyzer::new()),
            history: None,
            stream_active: false,
            cadence: None,
            smoothing: None,
            window_known: false,
        }
    }

    /// Whether a stream window may be analyzed: a stream was started, or metadata was imported,
    /// and no stop has ended it.
    #[must_use]
    pub const fn stream_active(&self) -> bool {
        self.stream_active
    }

    /// The cadence of the stream under analysis.
    #[must_use]
    pub const fn stream_cadence(&self) -> Option<SpectrumCadence> {
        self.cadence
    }

    /// The smoothing of the stream under analysis.
    #[must_use]
    pub const fn stream_smoothing(&self) -> Option<SpectrumSmoothingConfig> {
        self.smoothing
    }

    /// Whether a stream window is at hand: one was read or imported since the last reset.
    #[must_use]
    pub const fn has_stream_window(&self) -> bool {
        self.window_known
    }

    /// Whether the smoothing history exists.
    #[must_use]
    pub const fn has_stream_history(&self) -> bool {
        self.history.is_some()
    }

    /// The epoch of the smoothing history, zero before it exists.
    #[must_use]
    pub fn analysis_epoch(&self) -> u64 {
        self.history
            .as_deref()
            .map_or(0, SpectrumAnalysisHistory::analysis_epoch)
    }

    /// The first sample the smoothing history holds, zero when it is empty.
    #[must_use]
    pub fn history_start_sample(&self) -> u64 {
        self.history
            .as_deref()
            .and_then(SpectrumAnalysisHistory::history_start_sample)
            .unwrap_or(0)
    }

    /// Whether the history epoch cannot advance without reuse.
    fn history_exhausted(&self) -> bool {
        self.analysis_epoch() == u64::MAX
    }

    /// Reset the history, if it exists, and forget the window at hand.
    fn reset_history(&mut self) -> Result<(), SpectrumServiceError> {
        if let Some(history) = self.history.as_deref_mut() {
            history
                .reset()
                .map_err(|_| SpectrumServiceError::Exhausted)?;
        }
        self.window_known = false;
        Ok(())
    }

    /// Begin the analysis of a stream a service just started.
    fn begin_stream(
        &mut self,
        cadence: SpectrumCadence,
        smoothing: SpectrumSmoothingConfig,
    ) -> Result<(), SpectrumServiceError> {
        self.reset_history()?;
        self.stream_active = true;
        self.cadence = Some(cadence);
        self.smoothing = Some(smoothing);
        Ok(())
    }

    /// End the analysis of the stream: the history stays, the next stream resets it.
    pub fn end_stream(&mut self) {
        self.stream_active = false;
        self.cadence = None;
        self.smoothing = None;
        self.window_known = false;
    }

    /// Reset the smoothing history without stopping the stream, and forget the window at hand.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::StreamInactive`] without a stream under analysis;
    /// [`SpectrumServiceError::Exhausted`] when the history epoch cannot advance.
    pub fn reset_stream(&mut self) -> Result<(), SpectrumServiceError> {
        if !self.stream_active {
            return Err(SpectrumServiceError::StreamInactive);
        }
        self.reset_history()
    }

    /// Adopt the stream that `metadata` describes: its cadence, its smoothing and its window.
    ///
    /// The semantic checks of an imported ready window: a ready status, a valid target kind and
    /// channel mask, a non-zero capture epoch, `windows == sequence + 1`, an end sample 2,048
    /// frames after the start, a launch cadence (the default hop, or one of the four explicit
    /// ones) and a smoothing within range. A refusal changes nothing. The record words are the
    /// caller's to check. Allocates the history once, on the first adoption.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::MetadataInvalid`].
    pub fn configure_imported(
        &mut self,
        metadata: &SpectrumStreamMetadata,
    ) -> Result<(), SpectrumServiceError> {
        let invalid = SpectrumServiceError::MetadataInvalid;
        let windows = metadata.sequence.checked_add(1).ok_or(invalid)?;
        if metadata.status != SpectrumStreamStatus::Ready
            || !(SPECTRUM_TARGET_TRACK_POST_INPUT..=SPECTRUM_TARGET_OUTPUT)
                .contains(&metadata.target_kind)
            || !(SPECTRUM_CHANNEL_LEFT..=SPECTRUM_CHANNEL_BOTH).contains(&metadata.channels)
            || metadata.capture_epoch == 0
            || metadata.windows != windows
            || metadata.end_sample
                != metadata
                    .captured_sample
                    .checked_add(SPECTRUM_WINDOW_FRAMES as u64)
                    .ok_or(invalid)?
        {
            return Err(invalid);
        }
        let default_cadence =
            SpectrumCadence::new(metadata.sample_rate_hz, metadata.quantum_frames)
                .map_err(|_| invalid)?;
        let cadence = if metadata.hop_frames == default_cadence.hop_frames() {
            default_cadence
        } else {
            SpectrumCadence::with_hop(
                metadata.sample_rate_hz,
                metadata.quantum_frames,
                metadata.hop_frames,
            )
            .map_err(|_| invalid)?
        };
        let smoothing = SpectrumSmoothingConfig::new(metadata.smoothing_ms).map_err(|_| invalid)?;
        if self.history.is_none() {
            self.history = Some(Box::new(SpectrumAnalysisHistory::new()));
        }
        self.stream_active = true;
        self.cadence = Some(cadence);
        self.smoothing = Some(smoothing);
        self.window_known = true;
        Ok(())
    }

    /// Analyze one stream `window` into `out` and write the analysis facts into `metadata`.
    ///
    /// `window` carries its own capture facts; the cadence and the smoothing are the stream's.
    /// The analysis updates the smoothing history; a gap, a new capture epoch or a changed
    /// configuration resets it first. A refusal leaves `out` and `metadata` unchanged. Allocates
    /// the history if no start or import did.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::StreamInactive`] without a stream under analysis;
    /// [`SpectrumServiceError::WindowInvalid`] without a window at hand (none was read or
    /// imported since the last reset) or when the analyzer refuses the window.
    pub fn analyze_stream_into(
        &mut self,
        window: &SpectrumContinuousWindow,
        out: &mut SpectrumAnalysisResult,
        metadata: &mut SpectrumStreamMetadata,
    ) -> Result<(), SpectrumServiceError> {
        if !self.stream_active {
            return Err(SpectrumServiceError::StreamInactive);
        }
        let (Some(cadence), Some(smoothing)) = (self.cadence, self.smoothing) else {
            return Err(SpectrumServiceError::StreamInactive);
        };
        if !self.window_known {
            return Err(SpectrumServiceError::WindowInvalid);
        }
        let token = window
            .sequence
            .checked_add(1)
            .ok_or(SpectrumServiceError::WindowInvalid)?;
        let history = self
            .history
            .get_or_insert_with(|| Box::new(SpectrumAnalysisHistory::new()));
        let left = matches!(
            window.channels,
            SpectrumChannels::Left | SpectrumChannels::Stereo
        );
        let right = matches!(
            window.channels,
            SpectrumChannels::Right | SpectrumChannels::Stereo
        );
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let analyzed = {
            let mut output = SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: left.then_some(&mut left_dbfs),
                right_dbfs: right.then_some(&mut right_dbfs),
            };
            self.analyzer
                .analyze_continuous(window, cadence, smoothing, history, &mut output)
                .map_err(|_| SpectrumServiceError::WindowInvalid)?
        };
        out.target_kind = metadata.target_kind;
        out.channels = window.channels;
        out.sample_rate_hz = cadence.sample_rate_hz();
        out.captured_sample = analyzed.fft_first_sample;
        out.end_sample = analyzed.fft_end_sample;
        out.token = token;
        out.source_underrun = analyzed.source_underrun;
        out.frequencies_hz = frequencies;
        out.left_dbfs = left_dbfs;
        out.right_dbfs = right_dbfs;
        metadata.status = SpectrumStreamStatus::Ready;
        metadata.source_underrun = analyzed.source_underrun;
        metadata.capture_epoch = analyzed.stream_epoch;
        metadata.sequence = analyzed.sequence;
        metadata.captured_sample = analyzed.fft_first_sample;
        metadata.end_sample = analyzed.fft_end_sample;
        metadata.analysis_epoch = analyzed.analysis_epoch;
        metadata.history_start_sample = analyzed.history_start_sample;
        metadata.smoothing_ms = analyzed.smoothing_ms;
        Ok(())
    }

    /// Validate `snapshot`, analyze its window and fill `out`.
    ///
    /// A refused snapshot leaves `out` unchanged. The caller owns `out`, so a repeated analysis
    /// allocates nothing.
    ///
    /// # Errors
    ///
    /// [`SpectrumServiceError::WindowInvalid`] when the snapshot fails
    /// [`SpectrumSnapshot::validate`] or the analyzer refuses the window.
    pub fn analyze_into(
        &self,
        snapshot: &SpectrumSnapshot,
        out: &mut SpectrumAnalysisResult,
    ) -> Result<(), SpectrumServiceError> {
        snapshot.validate()?;
        let channels = snapshot.window.channels;
        let left = matches!(channels, SpectrumChannels::Left | SpectrumChannels::Stereo);
        let right = matches!(channels, SpectrumChannels::Right | SpectrumChannels::Stereo);
        let end_sample = snapshot
            .window
            .end_sample()
            .ok_or(SpectrumServiceError::WindowInvalid)?;
        let mut frequencies = [0.0; SPECTRUM_BIN_COUNT];
        let mut left_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        let mut right_dbfs = [SPECTRUM_FLOOR_DB; SPECTRUM_BIN_COUNT];
        {
            let mut output = SpectrumOutput {
                frequencies_hz: &mut frequencies,
                left_dbfs: left.then_some(&mut left_dbfs),
                right_dbfs: right.then_some(&mut right_dbfs),
            };
            self.analyzer
                .analyze(&snapshot.window, snapshot.sample_rate_hz, &mut output)
                .map_err(|_| SpectrumServiceError::WindowInvalid)?;
        }
        out.target_kind = snapshot.target_kind;
        out.channels = channels;
        out.sample_rate_hz = snapshot.sample_rate_hz;
        out.captured_sample = snapshot.window.first_sample;
        out.end_sample = end_sample;
        out.token = snapshot.token;
        out.source_underrun = snapshot.window.source_underrun;
        out.frequencies_hz = frequencies;
        out.left_dbfs = left_dbfs;
        out.right_dbfs = right_dbfs;
        Ok(())
    }
}

impl Default for SpectrumAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Turns red if a failed window keeps the old epoch's drop count, or an ordinary pending read
    /// loses the drops of the current epoch.
    #[test]
    fn a_failed_window_starts_a_fresh_epoch_with_no_drops() {
        let mut metadata = SpectrumStreamMetadata {
            capture_epoch: 7,
            dropped_captures: 12,
            source_underrun: true,
            ..SpectrumStreamMetadata::default()
        };
        metadata.record_outcome(SpectrumStreamStatus::Pending, None, None);
        assert_eq!((metadata.capture_epoch, metadata.dropped_captures), (7, 12));
        assert!(!metadata.source_underrun);
        metadata.record_outcome(SpectrumStreamStatus::Failed, Some(8), None);
        assert_eq!((metadata.capture_epoch, metadata.dropped_captures), (8, 0));
        metadata.record_outcome(SpectrumStreamStatus::Gap, Some(8), Some(2));
        assert_eq!(metadata.dropped_captures, 2);
    }
}
