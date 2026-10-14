//! One core service for the one-shot spectrum (issue #1591, slice S28 of decision 17, C13).
//!
//! `spectrum.rs` owns the kernels: the render observer, the capture queues and the analyzer. This
//! module owns what a host does with them for one window:
//!
//! * the **request rules** (target kind, target ID, channels, byte budget) that turn raw words into
//!   a [`SpectrumPreparation`];
//! * the **service** ([`SpectrumService`]) that arms, reads, cancels and selects the prepared
//!   single capture or collection, owns the snapshot token and enforces the ruling that an arm the
//!   caller made is never discarded by a later select (D7);
//! * the **analysis** of one window into dB bins ([`SpectrumAnalysis`]).
//!
//! Every [`SpectrumService`] method is allocation-free: the browser calls them inside render-locked
//! exports on its audio thread. [`SpectrumAnalysis`] is worker-side and owns a boxed analyzer, made
//! once. The request constructors allocate and run at preparation, before the host is published.

use crate::spectrum::{
    SPECTRUM_BIN_COUNT, SPECTRUM_FLOOR_DB, SpectrumAnalyzer, SpectrumCadence, SpectrumCapture,
    SpectrumCaptureCollection, SpectrumCaptureCollectionEntry, SpectrumCaptureCollectionReadError,
    SpectrumCaptureCollectionRequest, SpectrumCaptureCollectionSelectionError,
    SpectrumCaptureReadError, SpectrumCaptureRequest, SpectrumChannels,
    SpectrumContinuousCaptureError, SpectrumContinuousReadError, SpectrumContinuousWindow,
    SpectrumHop, SpectrumOutput, SpectrumTarget, SpectrumTargetRef, SpectrumWindow,
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

    /// Start scheduled windows on the selected entry (the stream half; issue #1592 moves it).
    ///
    /// # Errors
    ///
    /// The capture's refusal.
    pub fn start_continuous(
        &mut self,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Result<SpectrumCadence, SpectrumContinuousCaptureError> {
        match self {
            Self::Single(capture) => capture.start_continuous(sample_rate_hz, quantum_frames),
            Self::Collection(capture) => capture.start_continuous(sample_rate_hz, quantum_frames),
        }
    }

    /// Start scheduled windows with an explicit hop (the stream half; issue #1592 moves it).
    ///
    /// # Errors
    ///
    /// The capture's refusal.
    pub fn start_continuous_with_hop(
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

    /// Restart the active stream at a fresh epoch (the stream half; issue #1592 moves it).
    ///
    /// # Errors
    ///
    /// The capture's refusal.
    pub fn restart_continuous(&mut self) -> Result<(), SpectrumContinuousCaptureError> {
        match self {
            Self::Single(capture) => capture.restart_continuous(),
            Self::Collection(capture) => capture.restart_continuous(),
        }
    }

    /// Read one stream window (the stream half; issue #1592 moves it).
    ///
    /// # Errors
    ///
    /// The capture's explicit availability state.
    pub fn try_read_continuous(
        &mut self,
    ) -> Result<SpectrumContinuousWindow, SpectrumContinuousReadError> {
        match self {
            Self::Single(capture) => capture.try_read_continuous(),
            Self::Collection(capture) => capture.try_read_continuous(),
        }
    }

    /// Stop the stream and discard its queued window (the stream half; issue #1592 moves it).
    pub fn stop_continuous(&mut self) {
        match self {
            Self::Single(capture) => capture.stop_continuous(),
            Self::Collection(capture) => capture.stop_continuous(),
        }
    }

    /// The active stream cadence (the stream half; issue #1592 moves it).
    #[must_use]
    pub fn cadence(&self) -> Option<SpectrumCadence> {
        match self {
            Self::Single(capture) => capture.cadence(),
            Self::Collection(capture) => capture.cadence(),
        }
    }

    /// The active stream's capture epoch (the stream half; issue #1592 moves it).
    #[must_use]
    pub fn stream_epoch(&self) -> Option<u64> {
        match self {
            Self::Single(capture) => capture.stream_epoch(),
            Self::Collection(capture) => capture.stream_epoch(),
        }
    }
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

/// The one-shot spectrum: the prepared captures, the snapshot token and the arm ownership.
///
/// Every method is allocation-free. See the module documentation and decision 17 D7: an arm that
/// the caller made with [`Self::arm`] belongs to the caller until a read returns its window or
/// [`SpectrumServiceError::CaptureInvalid`], or until [`Self::cancel`].
pub struct SpectrumService {
    captures: PreparedSpectrumCaptures,
    token: u64,
    sample_rate_hz: u32,
    quantum_frames: u32,
    caller_armed: bool,
}

impl SpectrumService {
    /// Own `captures` for a session at `sample_rate_hz` and `quantum_frames`.
    #[must_use]
    pub const fn new(
        captures: PreparedSpectrumCaptures,
        sample_rate_hz: u32,
        quantum_frames: u32,
    ) -> Self {
        Self {
            captures,
            token: 0,
            sample_rate_hz,
            quantum_frames,
            caller_armed: false,
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

    /// The prepared captures, read-only: the stream half's queries (issue #1592 moves them).
    #[must_use]
    pub const fn captures(&self) -> &PreparedSpectrumCaptures {
        &self.captures
    }

    /// The prepared captures, for the stream half that issue #1592 moves into this type.
    ///
    /// A caller that starts the stream through this hand-off must call [`Self::end_caller_arm`]
    /// when the start succeeds: a collection's stream start replaces the selected entry's arm.
    /// Issue #1592 D4 applies the arm rule to the stream start itself.
    pub fn captures_mut(&mut self) -> &mut PreparedSpectrumCaptures {
        &mut self.captures
    }

    /// End the caller's claim on its armed window, after the stream start replaced the arm.
    pub fn end_caller_arm(&mut self) {
        self.caller_armed = false;
    }

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
    pub fn cancel(&mut self) {
        self.captures.cancel();
        self.caller_armed = false;
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
    /// replaced.
    ///
    /// # Errors
    ///
    /// As [`Self::selection_would_change`].
    pub fn select(
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

/// The worker-side analysis of one window: one boxed analyzer, made once.
pub struct SpectrumAnalysis {
    analyzer: Box<SpectrumAnalyzer>,
}

impl SpectrumAnalysis {
    /// Prepare the analyzer on the control or worker side.
    #[must_use]
    pub fn new() -> Self {
        Self {
            analyzer: Box::new(SpectrumAnalyzer::new()),
        }
    }

    /// The analyzer, for the stream analysis that issue #1592 moves into this type.
    #[must_use]
    pub fn analyzer(&self) -> &SpectrumAnalyzer {
        &self.analyzer
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
