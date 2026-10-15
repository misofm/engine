//! Typed session edits and the transactional control-plane session store.
//!
//! These edits operate on the accepted issue-004 `SessionModel` model. They neither prepare nor
//! publish a render plan; a successful replacement retains only issue-004's immutable,
//! non-publishable `CompiledSession` control-plane artifact.

use core::fmt;
use std::collections::BTreeMap;

use session::{
    Automation, AutomationSegment, CompileCaps, CompiledSession, Console, ConsoleEntry,
    EditRampRow, EditRampTarget, EditRamps, Effect, EffectIdentity, EffectQuality, Output,
    OutputProfile, Rack, RackName, RenderProfile, Route, RouteDestination, RouteSource,
    SessionModel, SidechainDeclaration, Source, SourceBitDepth, StableId, Submix, Vca,
    compile_session,
};

use crate::{ExpectedRevision, SessionRevision};

/// Stable numeric registry for every V1 `SessionEdit` variant.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum SessionEditOpcode {
    /// `SetSessionId`.
    SetSessionId = 0x0001,
    /// `SetSampleRateHz`.
    SetSampleRateHz = 0x0002,
    /// `SetQuantumFrames`.
    SetQuantumFrames = 0x0003,
    /// `SetRenderProfile`.
    SetRenderProfile = 0x0004,
    /// `SetOutputProfile`.
    SetOutputProfile = 0x0005,
    /// `SetConsole` (decision 12, #1094). Appended: `0x0006` is retired by #241 and never
    /// reallocated.
    SetConsole = 0x0007,
    /// `UpsertSource`.
    UpsertSource = 0x0100,
    /// `RemoveSource`.
    RemoveSource = 0x0101,
    /// `SetSourceContent`.
    SetSourceContent = 0x0103,
    /// `UpsertTrack`.
    UpsertTrack = 0x0200,
    /// `RemoveTrack`.
    RemoveTrack = 0x0201,
    /// `SetTrackSourceAssignment`.
    SetTrackSourceAssignment = 0x0202,
    /// `SetTrackRack`.
    SetTrackRack = 0x0204,
    /// `PutTrackEffect`.
    PutTrackEffect = 0x0205,
    /// `RemoveTrackEffect`.
    RemoveTrackEffect = 0x0206,
    /// `SetTrackEffectOrder`.
    SetTrackEffectOrder = 0x0207,
    /// `SetEffectIdentity`.
    SetEffectIdentity = 0x0208,
    /// `SetEffectQuality`.
    SetEffectQuality = 0x0209,
    /// `SetEffectSidechain`.
    SetEffectSidechain = 0x020c,
    /// `SetTrackConsole` (decision 12, #1094), appended to the strip family (`0x02xx`; #1204 lets
    /// it address a submix).
    SetTrackConsole = 0x0211,
    /// `UpsertSubmix`.
    UpsertSubmix = 0x0300,
    /// `RemoveSubmix`.
    RemoveSubmix = 0x0301,
    /// `UpsertOutput`.
    UpsertOutput = 0x0400,
    /// `RemoveOutput`.
    RemoveOutput = 0x0401,
    /// `UpsertRoute`.
    UpsertRoute = 0x0500,
    /// `RemoveRoute`.
    RemoveRoute = 0x0501,
    /// `SetRouteSource`.
    SetRouteSource = 0x0502,
    /// `SetRouteDestination`.
    SetRouteDestination = 0x0503,
    /// `UpsertAutomation`.
    UpsertAutomation = 0x0600,
    /// `RemoveAutomation`.
    RemoveAutomation = 0x0601,
    /// `SetAutomationTarget`.
    SetAutomationTarget = 0x0602,
    /// `SetAutomationSegments`.
    SetAutomationSegments = 0x0603,
    /// `UpsertVca` (#1241), the first code of the new VCA family (`0x07xx`).
    UpsertVca = 0x0700,
    /// `RemoveVca` (#1241).
    RemoveVca = 0x0701,
    /// `SetControl` (decision 17, #1553): any control by its stable address. The first code of
    /// the control family (`0x08xx`).
    SetControl = 0x0800,
}

impl SessionEditOpcode {
    /// Return the immutable V1 opcode number.
    #[must_use]
    pub const fn raw(self) -> u16 {
        self as u16
    }

    /// Resolve one immutable V1 opcode, rejecting unallocated values.
    #[must_use]
    pub const fn from_raw(value: u16) -> Option<Self> {
        match value {
            0x0001 => Some(Self::SetSessionId),
            0x0002 => Some(Self::SetSampleRateHz),
            0x0003 => Some(Self::SetQuantumFrames),
            0x0004 => Some(Self::SetRenderProfile),
            0x0005 => Some(Self::SetOutputProfile),
            0x0007 => Some(Self::SetConsole),
            0x0100 => Some(Self::UpsertSource),
            0x0101 => Some(Self::RemoveSource),
            0x0103 => Some(Self::SetSourceContent),
            0x0200 => Some(Self::UpsertTrack),
            0x0201 => Some(Self::RemoveTrack),
            0x0202 => Some(Self::SetTrackSourceAssignment),
            0x0204 => Some(Self::SetTrackRack),
            0x0205 => Some(Self::PutTrackEffect),
            0x0206 => Some(Self::RemoveTrackEffect),
            0x0207 => Some(Self::SetTrackEffectOrder),
            0x0208 => Some(Self::SetEffectIdentity),
            0x0209 => Some(Self::SetEffectQuality),
            0x020c => Some(Self::SetEffectSidechain),
            0x0211 => Some(Self::SetTrackConsole),
            0x0300 => Some(Self::UpsertSubmix),
            0x0301 => Some(Self::RemoveSubmix),
            0x0400 => Some(Self::UpsertOutput),
            0x0401 => Some(Self::RemoveOutput),
            0x0500 => Some(Self::UpsertRoute),
            0x0501 => Some(Self::RemoveRoute),
            0x0502 => Some(Self::SetRouteSource),
            0x0503 => Some(Self::SetRouteDestination),
            0x0600 => Some(Self::UpsertAutomation),
            0x0601 => Some(Self::RemoveAutomation),
            0x0602 => Some(Self::SetAutomationTarget),
            0x0603 => Some(Self::SetAutomationSegments),
            0x0700 => Some(Self::UpsertVca),
            0x0701 => Some(Self::RemoveVca),
            0x0800 => Some(Self::SetControl),
            _ => None,
        }
    }
}

/// One complete typed session mutation. Its payload field ordering is frozen in the issue-005
/// BTLV specification; the wire-schema codec maps these variants one-to-one.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)] // Variant docs name the frozen payload; field names are the frozen mapping.
pub enum SessionEdit {
    /// Replace the session stable ID.
    SetSessionId { session_id: StableId },
    /// Replace the explicit engine sample rate without implying SRC.
    SetSampleRateHz { sample_rate_hz: u32 },
    /// Replace the explicit render quantum.
    SetQuantumFrames { quantum_frames: u32 },
    /// Replace the renderer declaration.
    SetRenderProfile { render_profile: RenderProfile },
    /// Replace the V1 output declaration.
    SetOutputProfile { output_profile: OutputProfile },
    /// Replace the session's whole console declaration: both sections, every slot's identity,
    /// quality and link mode (decision 12).
    ///
    /// This is the only edit that changes the slot set. Every strip, track or submix, carries one
    /// entry per slot in slot order, so a transaction that adds, removes, renames or reorders a
    /// slot must rewrite every strip's entries in the same transaction (`SetTrackConsole`,
    /// `UpsertTrack` or `UpsertSubmix`), or the final validation refuses the whole transaction and
    /// nothing is committed. `SetTrackConsole` is accepted only after a `SetConsole` of the same
    /// transaction that changed the console (#1561 D3).
    SetConsole {
        console: Console,
        /// This edit's own ramp length in samples; absent means the session's `control_smoothing`
        /// (#1394). Transaction metadata, never model state.
        ramp_samples: Option<u32>,
    },
    /// Insert or replace one source by stable ID.
    UpsertSource { source: Source },
    /// Remove one source without cascading references.
    RemoveSource { source_id: StableId },
    /// Set a source's complete content-addressed shape.
    SetSourceContent {
        source_id: StableId,
        content: String,
        channels: u8,
        bit_depth: SourceBitDepth,
        frames: u64,
    },
    /// Insert or replace one track by stable ID.
    UpsertTrack { track: session::Track },
    /// Remove one track without cascading routes or automation.
    RemoveTrack { track_id: StableId },
    /// Set the complete source assignment for one track.
    SetTrackSourceAssignment {
        /// Existing track to update.
        track_id: StableId,
        /// Source declaration reference.
        source_id: StableId,
        /// Left source channel.
        left_source_channel: u8,
        /// Right source channel.
        right_source_channel: u8,
    },
    /// Replace a complete named effect rack.
    SetTrackRack {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        rack_name: RackName,
        rack: Rack,
    },
    /// Insert/reposition one effect after removing a same-ID effect.
    PutTrackEffect {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// Rack to change.
        rack_name: RackName,
        /// Zero-based position after same-ID removal.
        final_position: u32,
        /// Effect declaration to insert.
        effect: Effect,
    },
    /// Remove a named effect from one rack.
    RemoveTrackEffect {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        rack_name: RackName,
        effect_id: StableId,
    },
    /// Replace effect order with an exact permutation of current IDs.
    SetTrackEffectOrder {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// Rack to reorder.
        rack_name: RackName,
        /// Exact stable-ID permutation.
        effect_ids: Vec<StableId>,
    },
    /// Set a named effect's identity declaration.
    SetEffectIdentity {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New identity.
        identity: EffectIdentity,
    },
    /// Set a named effect's quality.
    SetEffectQuality {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New quality token.
        quality: EffectQuality,
    },
    /// Set a named effect's sidechain declaration.
    SetEffectSidechain {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New sidechain declaration.
        sidechain: SidechainDeclaration,
    },
    /// Give one strip its console entries after a `SetConsole` of the same transaction changed the
    /// slot set (decision 12, #1561 D3).
    ///
    /// It carries that strip's entries, `bypass` and `params`, one entry per session slot in slot
    /// order. It is structural only: it is accepted only when an earlier `SetConsole` of the
    /// transaction made the candidate console differ from the committed one. Otherwise it is
    /// refused with [`SessionEditError::ValueOpRetired`] at this edit, before any write, because a
    /// console knob's value is set with [`SessionEdit::SetControl`]; a `SetConsole` that repeats
    /// the committed console does not open it. An entry array that adds, drops or reorders a slot
    /// refuses at the transaction's final validation.
    SetTrackConsole {
        /// Existing strip: a track ID or a submix ID, tracks first (#1204).
        track_id: StableId,
        /// The complete ordered entries, one per session console slot.
        console: Vec<ConsoleEntry>,
        /// This edit's own ramp length in samples; absent means the session's `control_smoothing`
        /// (#1394). Transaction metadata, never model state.
        ramp_samples: Option<u32>,
    },
    /// Insert or replace a submix.
    UpsertSubmix { submix: Submix },
    /// Remove a submix without cascading declarations.
    RemoveSubmix { submix_id: StableId },
    /// Insert or replace an output.
    UpsertOutput { output: Output },
    /// Remove an output without cascading routes.
    RemoveOutput { output_id: StableId },
    /// Insert or replace a route.
    UpsertRoute { route: Route },
    /// Remove one route.
    RemoveRoute { route_id: StableId },
    /// Set one route source.
    SetRouteSource {
        route_id: StableId,
        source: RouteSource,
    },
    /// Set one route destination.
    SetRouteDestination {
        route_id: StableId,
        destination: RouteDestination,
    },
    /// Insert or replace persistent automation by stable ID.
    UpsertAutomation { automation: Automation },
    /// Remove persistent automation by stable ID.
    RemoveAutomation { automation_id: StableId },
    /// Set an existing automation target.
    SetAutomationTarget {
        automation_id: StableId,
        target: session::ControlAddress,
    },
    /// Replace an existing automation's complete nonempty ordered segment sequence.
    SetAutomationSegments {
        automation_id: StableId,
        segments: Vec<AutomationSegment>,
    },
    /// Insert or replace a VCA by stable ID: its fader offsets and its whole member list.
    ///
    /// A member may be declared later in the same transaction: membership, namespace, range and
    /// acyclicity are validated once, on the final candidate.
    UpsertVca { vca: Vca },
    /// Remove a VCA without cascading: a VCA that still lists it as a member refuses the
    /// transaction at final validation unless the same transaction rewrites that VCA.
    RemoveVca { vca_id: StableId },
    /// Set any control by its stable address (decision 17, C5, #1553). The core's one validator
    /// refuses a bad write with a typed [`SessionEditError::Control`] before anything changes;
    /// whether the committed edit is live or a rebuild is the classifier's answer for the
    /// resulting model, exactly as for the per-kind edit that writes the same field.
    SetControl {
        address: controls::ControlAddress,
        /// The value in the control's unit; a toggle is exactly 0 or 1 and an enumeration one of
        /// its choice values. Not required to be finite on the wire: the validator refuses it.
        value: f32,
        /// This edit's own ramp length in samples; absent means the row's default. A row whose
        /// ramp is fixed refuses an explicit length. Transaction metadata, never model state.
        ramp_samples: Option<u32>,
        /// The native effect this edit was written for (#1678); only in an `inserts` or `console`
        /// section. A row's numeric ID depends on its effect, and a rebuild can change the effect
        /// an insert or console slot runs, so the core refuses the edit with
        /// [`controls::ControlError::EffectChanged`], before anything is written, when the
        /// instance it names runs another. Absent applies the edit to whatever runs there now.
        expected_effect: Option<StableId>,
    },
}

impl SessionEdit {
    /// A [`SessionEdit::SetControl`] of `address` to `value`: the one way to set a value
    /// (decision 17, #1561). `ramp_samples` is the edit's own ramp length, or `None` for the
    /// row's default. It names no expected effect; see [`Self::with_expected_effect`].
    #[must_use]
    pub const fn set_control(
        address: controls::ControlAddress,
        value: f32,
        ramp_samples: Option<u32>,
    ) -> Self {
        Self::SetControl {
            address,
            value,
            ramp_samples,
            expected_effect: None,
        }
    }

    /// This edit with the native effect it was written for (#1678). Only a
    /// [`SessionEdit::SetControl`] carries one; any other edit is returned unchanged.
    #[must_use]
    pub fn with_expected_effect(mut self, effect: StableId) -> Self {
        if let Self::SetControl {
            expected_effect, ..
        } = &mut self
        {
            *expected_effect = Some(effect);
        }
        self
    }

    /// Return the stable numeric opcode for this typed edit.
    #[must_use]
    pub const fn opcode(&self) -> SessionEditOpcode {
        match self {
            Self::SetSessionId { .. } => SessionEditOpcode::SetSessionId,
            Self::SetSampleRateHz { .. } => SessionEditOpcode::SetSampleRateHz,
            Self::SetQuantumFrames { .. } => SessionEditOpcode::SetQuantumFrames,
            Self::SetRenderProfile { .. } => SessionEditOpcode::SetRenderProfile,
            Self::SetOutputProfile { .. } => SessionEditOpcode::SetOutputProfile,
            Self::SetConsole { .. } => SessionEditOpcode::SetConsole,
            Self::UpsertSource { .. } => SessionEditOpcode::UpsertSource,
            Self::RemoveSource { .. } => SessionEditOpcode::RemoveSource,
            Self::SetSourceContent { .. } => SessionEditOpcode::SetSourceContent,
            Self::UpsertTrack { .. } => SessionEditOpcode::UpsertTrack,
            Self::RemoveTrack { .. } => SessionEditOpcode::RemoveTrack,
            Self::SetTrackSourceAssignment { .. } => SessionEditOpcode::SetTrackSourceAssignment,
            Self::SetTrackRack { .. } => SessionEditOpcode::SetTrackRack,
            Self::PutTrackEffect { .. } => SessionEditOpcode::PutTrackEffect,
            Self::RemoveTrackEffect { .. } => SessionEditOpcode::RemoveTrackEffect,
            Self::SetTrackEffectOrder { .. } => SessionEditOpcode::SetTrackEffectOrder,
            Self::SetEffectIdentity { .. } => SessionEditOpcode::SetEffectIdentity,
            Self::SetEffectQuality { .. } => SessionEditOpcode::SetEffectQuality,
            Self::SetEffectSidechain { .. } => SessionEditOpcode::SetEffectSidechain,
            Self::SetTrackConsole { .. } => SessionEditOpcode::SetTrackConsole,
            Self::UpsertSubmix { .. } => SessionEditOpcode::UpsertSubmix,
            Self::RemoveSubmix { .. } => SessionEditOpcode::RemoveSubmix,
            Self::UpsertOutput { .. } => SessionEditOpcode::UpsertOutput,
            Self::RemoveOutput { .. } => SessionEditOpcode::RemoveOutput,
            Self::UpsertRoute { .. } => SessionEditOpcode::UpsertRoute,
            Self::RemoveRoute { .. } => SessionEditOpcode::RemoveRoute,
            Self::SetRouteSource { .. } => SessionEditOpcode::SetRouteSource,
            Self::SetRouteDestination { .. } => SessionEditOpcode::SetRouteDestination,
            Self::UpsertAutomation { .. } => SessionEditOpcode::UpsertAutomation,
            Self::RemoveAutomation { .. } => SessionEditOpcode::RemoveAutomation,
            Self::SetAutomationTarget { .. } => SessionEditOpcode::SetAutomationTarget,
            Self::SetAutomationSegments { .. } => SessionEditOpcode::SetAutomationSegments,
            Self::UpsertVca { .. } => SessionEditOpcode::UpsertVca,
            Self::RemoveVca { .. } => SessionEditOpcode::RemoveVca,
            Self::SetControl { .. } => SessionEditOpcode::SetControl,
        }
    }
}

impl SessionEdit {
    /// The per-edit ramp length in samples that this edit carries (#1394), if it is one of the
    /// live-value edits and has one. `None` for every other edit and for a live-value edit that
    /// leaves it absent, which means the session's `control_smoothing`.
    #[must_use]
    pub const fn ramp_samples(&self) -> Option<u32> {
        match self {
            Self::SetConsole { ramp_samples, .. }
            | Self::SetTrackConsole { ramp_samples, .. }
            | Self::SetControl { ramp_samples, .. } => *ramp_samples,
            // Listed one by one on purpose, with no wildcard: a new variant must be classified
            // here, as in `record_edit_ramps`. An existing arm that gains a `ramp_samples` field
            // still compiles, so a reviewer checks the arm of any variant that gains one.
            Self::SetSessionId { .. }
            | Self::SetSampleRateHz { .. }
            | Self::SetQuantumFrames { .. }
            | Self::SetRenderProfile { .. }
            | Self::SetOutputProfile { .. }
            | Self::UpsertSource { .. }
            | Self::RemoveSource { .. }
            | Self::SetSourceContent { .. }
            | Self::UpsertTrack { .. }
            | Self::RemoveTrack { .. }
            | Self::SetTrackSourceAssignment { .. }
            | Self::SetTrackRack { .. }
            | Self::SetTrackEffectOrder { .. }
            | Self::PutTrackEffect { .. }
            | Self::RemoveTrackEffect { .. }
            | Self::SetEffectIdentity { .. }
            | Self::SetEffectQuality { .. }
            | Self::SetEffectSidechain { .. }
            | Self::UpsertOutput { .. }
            | Self::RemoveOutput { .. }
            | Self::UpsertRoute { .. }
            | Self::RemoveRoute { .. }
            | Self::SetRouteSource { .. }
            | Self::SetRouteDestination { .. }
            | Self::UpsertSubmix { .. }
            | Self::RemoveSubmix { .. }
            | Self::UpsertAutomation { .. }
            | Self::RemoveAutomation { .. }
            | Self::SetAutomationTarget { .. }
            | Self::SetAutomationSegments { .. }
            | Self::UpsertVca { .. }
            | Self::RemoveVca { .. } => None,
        }
    }
}

/// Record into `ramps` the rows `edit` wrote and the length it gave them (#1394 D3): the last
/// edit that writes a `(target, row)` decides it, so a later edit without a length removes an
/// earlier one's. An upsert, a removal or a rack edit that replaces an effect rewrites the
/// target's rows with no length and clears them.
///
/// Several `SET_CONTROL` rows share one `(target, row)` key (the fader's left and right lanes, the
/// pan and the matrix rows, trim and polarity, a route's mute and follows-mute), and the key holds
/// one length. `control_writes` remembers the length each key's last `SET_CONTROL` gave it (`None`
/// for absent), so a second `SET_CONTROL` that resolves a different length for the same key is
/// refused with [`controls::ControlError::RampConflict`]; equal lengths agree. A per-kind edit or
/// a rewrite of the target is not a control write: it replaces the key's record and keeps its
/// last-edit-decides behaviour.
fn record_edit_ramps(
    ramps: &mut EditRamps,
    control_writes: &mut ControlRampWrites,
    edit: &SessionEdit,
) -> Result<(), controls::ControlError> {
    let length = edit.ramp_samples();
    let mut recorder = RampRecorder {
        ramps,
        control_writes,
        length,
    };
    match edit {
        SessionEdit::SetTrackConsole { track_id, .. } => recorder.write(
            EditRampTarget::Strip(track_id.clone()),
            &[EditRampRow::Link],
        ),
        SessionEdit::SetConsole { .. } => {
            recorder.write(EditRampTarget::Console, &[EditRampRow::Link])
        }
        // A control edit writes the one row its address names, if that row ramps.
        SessionEdit::SetControl { address, .. } => {
            if let Some((target, row)) = control_ramp_row(address) {
                recorder.write_control(target, row)?;
            }
        }
        SessionEdit::UpsertTrack { track } => {
            recorder.clear_target(&EditRampTarget::Strip(track.id.clone()));
        }
        SessionEdit::UpsertSubmix { submix } => {
            recorder.clear_target(&EditRampTarget::Strip(submix.id.clone()));
        }
        SessionEdit::UpsertRoute { route } => {
            recorder.clear_target(&EditRampTarget::Route(route.id.clone()));
        }
        SessionEdit::UpsertVca { vca } => {
            recorder.clear_target(&EditRampTarget::Vca(vca.id.clone()));
        }
        // A removal ends the target's rows, and only an upsert (above) re-creates it, so the
        // removal clears too: no entry outlives the thing it describes.
        SessionEdit::RemoveTrack { track_id } => {
            recorder.clear_target(&EditRampTarget::Strip(track_id.clone()));
        }
        SessionEdit::RemoveSubmix { submix_id } => {
            recorder.clear_target(&EditRampTarget::Strip(submix_id.clone()));
        }
        SessionEdit::RemoveRoute { route_id } => {
            recorder.clear_target(&EditRampTarget::Route(route_id.clone()));
        }
        SessionEdit::RemoveVca { vca_id } => {
            recorder.clear_target(&EditRampTarget::Vca(vca_id.clone()));
        }
        // Every other edit writes no row that `EditRamps` keys: a rack, effect, identity, quality
        // or sidechain edit rewrites no ramp-carrying value, and a route's endpoint edit leaves
        // its gain, mute and matrix. The list is exhaustive on purpose: a new edit must be
        // classified here, not fall into a default.
        SessionEdit::SetSessionId { .. }
        | SessionEdit::SetSampleRateHz { .. }
        | SessionEdit::SetQuantumFrames { .. }
        | SessionEdit::SetRenderProfile { .. }
        | SessionEdit::SetOutputProfile { .. }
        | SessionEdit::UpsertSource { .. }
        | SessionEdit::RemoveSource { .. }
        | SessionEdit::SetSourceContent { .. }
        | SessionEdit::SetTrackSourceAssignment { .. }
        | SessionEdit::SetTrackRack { .. }
        | SessionEdit::PutTrackEffect { .. }
        | SessionEdit::RemoveTrackEffect { .. }
        | SessionEdit::SetTrackEffectOrder { .. }
        | SessionEdit::SetEffectIdentity { .. }
        | SessionEdit::SetEffectQuality { .. }
        | SessionEdit::SetEffectSidechain { .. }
        | SessionEdit::UpsertOutput { .. }
        | SessionEdit::RemoveOutput { .. }
        | SessionEdit::SetRouteSource { .. }
        | SessionEdit::SetRouteDestination { .. }
        | SessionEdit::UpsertAutomation { .. }
        | SessionEdit::RemoveAutomation { .. }
        | SessionEdit::SetAutomationTarget { .. }
        | SessionEdit::SetAutomationSegments { .. } => {}
    }
    Ok(())
}

/// The length each `(target, row)` key's last `SET_CONTROL` edit of the transaction gave it, with
/// `None` for an edit that carried none.
type ControlRampWrites = BTreeMap<(EditRampTarget, EditRampRow), Option<u32>>;

/// The two records [`record_edit_ramps`] keeps for one edit.
struct RampRecorder<'a> {
    ramps: &'a mut EditRamps,
    control_writes: &'a mut ControlRampWrites,
    length: Option<u32>,
}

impl RampRecorder<'_> {
    /// A per-kind edit wrote `rows`: the last edit decides, and the key is no longer a control's.
    fn write(&mut self, target: EditRampTarget, rows: &[EditRampRow]) {
        for row in rows {
            self.control_writes.remove(&(target.clone(), *row));
            self.ramps.set(&target, *row, self.length);
        }
    }

    /// A `SET_CONTROL` edit wrote `row`: refused when an earlier `SET_CONTROL` of the transaction
    /// resolved another length for the same key, before anything is recorded.
    fn write_control(
        &mut self,
        target: EditRampTarget,
        row: EditRampRow,
    ) -> Result<(), controls::ControlError> {
        let key = (target, row);
        if self
            .control_writes
            .get(&key)
            .is_some_and(|earlier| *earlier != self.length)
        {
            return Err(controls::ControlError::RampConflict);
        }
        self.ramps.set(&key.0, row, self.length);
        self.control_writes.insert(key, self.length);
        Ok(())
    }

    /// An upsert or a removal rewrote `target`: every length of it goes, control or not.
    fn clear_target(&mut self, target: &EditRampTarget) {
        self.ramps.clear_target(target);
        self.control_writes.retain(|(owner, _), _| owner != target);
    }
}

/// The `(target, row)` of the recorded ramp length that a write to `address` decides (#1553 D4),
/// or `None` for a row whose ramp is fixed (its kernel owns it, and `controls::check` has refused
/// an explicit length) or that has no ramp row. The row is the one the control's descriptor names,
/// so the table of which control ramps lives in the control core and nowhere else.
fn control_ramp_row(address: &controls::ControlAddress) -> Option<(EditRampTarget, EditRampRow)> {
    use controls::{LiveRampRow, Motion, Owner};
    let Motion::Ramp(ramp_row) = controls::row_motion(
        address.owner.kind(),
        address.section.kind(),
        address.control,
    )?
    else {
        return None;
    };
    // No wildcard: a new live ramp row must be classified here.
    let row = match ramp_row {
        LiveRampRow::Fader => EditRampRow::Fader,
        LiveRampRow::Mute => EditRampRow::Mute,
        LiveRampRow::Pan | LiveRampRow::Matrix => EditRampRow::Matrix,
        LiveRampRow::InputTrim | LiveRampRow::PolarityInvert => EditRampRow::Input,
        LiveRampRow::SendGain => EditRampRow::RouteGain,
        LiveRampRow::SendMute | LiveRampRow::SendFollowsMute => EditRampRow::RouteMute,
        LiveRampRow::SendMatrix => EditRampRow::RouteMatrix,
        LiveRampRow::VcaOffset => EditRampRow::VcaFader,
        LiveRampRow::VcaMute => EditRampRow::VcaMute,
        // Solo is a monitor row (refused by `set`), the link glide has no control row with a
        // per-edit length, and the bypass crossfade is fixed.
        LiveRampRow::Solo | LiveRampRow::LinkGlide | LiveRampRow::BypassCrossfade => return None,
    };
    let target = match &address.owner {
        Owner::Strip(id) => EditRampTarget::Strip(id.clone()),
        Owner::Route(id) => EditRampTarget::Route(id.clone()),
        Owner::Vca(id) => EditRampTarget::Vca(id.clone()),
        Owner::Session => return None,
    };
    Some((target, row))
}

/// An edit-resolution failure before session compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEditError {
    /// A targeted source, strip (a track or a submix), effect, route, automation or VCA was
    /// absent.
    NotFound,
    /// An effect insertion position exceeded the post-removal rack length.
    InvalidFinalPosition,
    /// A requested effect ordering was not an exact current-ID permutation.
    InvalidEffectOrder,
    /// A complete automation segment replacement was empty.
    EmptyAutomationSegments,
    /// A rack-addressed structural or declaration edit named the `console` rack (decision 12).
    ///
    /// A strip cannot add, remove or reorder a console slot, and a slot's identity, quality and
    /// link mode are session-level; a console slot has no sidechain at all. `SetTrackRack`,
    /// `PutTrackEffect`, `RemoveTrackEffect`, `SetTrackEffectOrder`, `SetEffectIdentity`,
    /// `SetEffectQuality` and `SetEffectSidechain` therefore refuse
    /// `console` whatever the model holds. The slot set and its declarations change only through
    /// `SetConsole`; a strip's knobs are set with `SetControl`.
    ConsoleSlotFixed,
    /// A live-value edit carried a `ramp_samples` above the session's sample rate in samples, one
    /// second (#1394 D4, the bound of `control_smoothing`'s 1000 ms). The bound uses the rate the
    /// transaction commits.
    RampOutOfRange,
    /// A `SetControl` edit was refused by the control core's validator (decision 17, C5). The
    /// diagnostic code is the control code itself (`control.domain`, `control.read_only`, ...).
    Control(controls::ControlError),
    /// A `SetTrackConsole` edit came in a transaction with no earlier `SetConsole` that changed the
    /// console (decision 17, #1561 D3). The op only gives a strip its entries after the slot set changed; a console
    /// knob's value is set with `SetControl`. The diagnostic code is `session.edit.value_op_retired`.
    ValueOpRetired,
}

impl fmt::Display for SessionEditError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SessionEditError {}

/// Apply one edit to a caller-owned candidate model. This does not validate or compile the
/// candidate: [`SessionStore::apply_transaction`] does both atomically after all edits resolve.
pub fn apply_session_edit(
    session: &mut SessionModel,
    edit: &SessionEdit,
) -> Result<(), SessionEditError> {
    match edit {
        SessionEdit::SetSessionId { session_id } => session.session_id = session_id.clone(),
        SessionEdit::SetSampleRateHz { sample_rate_hz } => session.sample_rate_hz = *sample_rate_hz,
        SessionEdit::SetQuantumFrames { quantum_frames } => {
            session.quantum_frames = *quantum_frames
        }
        SessionEdit::SetRenderProfile { render_profile } => {
            session.render_profile = render_profile.clone()
        }
        SessionEdit::SetOutputProfile { output_profile } => {
            session.output_profile = output_profile.clone()
        }
        SessionEdit::SetConsole { console, .. } => session.console = console.clone(),
        SessionEdit::UpsertSource { source } => {
            upsert(&mut session.sources, source, |item| &item.id)
        }
        SessionEdit::RemoveSource { source_id } => {
            remove(&mut session.sources, source_id, |item| &item.id)?
        }
        SessionEdit::SetSourceContent {
            source_id,
            content,
            channels,
            bit_depth,
            frames,
        } => {
            let source = source_mut(session, source_id)?;
            source.content = content.clone();
            source.channels = *channels;
            source.bit_depth = *bit_depth;
            source.frames = *frames;
        }
        SessionEdit::UpsertTrack { track } => upsert(&mut session.tracks, track, |item| &item.id),
        SessionEdit::RemoveTrack { track_id } => {
            remove(&mut session.tracks, track_id, |item| &item.id)?
        }
        SessionEdit::SetTrackSourceAssignment {
            track_id,
            source_id,
            left_source_channel,
            right_source_channel,
        } => {
            let track = track_mut(session, track_id)?;
            track.source_id = source_id.clone();
            track.left_source_channel = *left_source_channel;
            track.right_source_channel = *right_source_channel;
        }
        SessionEdit::SetTrackRack {
            track_id,
            rack_name,
            rack,
        } => {
            *rack_mut(session, track_id, *rack_name)? = rack.clone();
        }
        SessionEdit::PutTrackEffect {
            track_id,
            rack_name,
            final_position,
            effect,
        } => {
            let effects = &mut rack_mut(session, track_id, *rack_name)?.effects;
            if let Some(index) = effects.iter().position(|item| item.id == effect.id) {
                effects.remove(index);
            }
            let position = usize::try_from(*final_position)
                .map_err(|_| SessionEditError::InvalidFinalPosition)?;
            if position > effects.len() {
                return Err(SessionEditError::InvalidFinalPosition);
            }
            effects.insert(position, effect.clone());
        }
        SessionEdit::RemoveTrackEffect {
            track_id,
            rack_name,
            effect_id,
        } => {
            let effects = &mut rack_mut(session, track_id, *rack_name)?.effects;
            let index = effects
                .iter()
                .position(|item| &item.id == effect_id)
                .ok_or(SessionEditError::NotFound)?;
            effects.remove(index);
        }
        SessionEdit::SetTrackEffectOrder {
            track_id,
            rack_name,
            effect_ids,
        } => {
            let effects = &mut rack_mut(session, track_id, *rack_name)?.effects;
            if effect_ids.len() != effects.len()
                || effect_ids
                    .iter()
                    .any(|id| effects.iter().filter(|effect| effect.id == *id).count() != 1)
            {
                return Err(SessionEditError::InvalidEffectOrder);
            }
            let mut source = core::mem::take(effects);
            let mut ordered = Vec::with_capacity(source.len());
            for id in effect_ids {
                let index = source
                    .iter()
                    .position(|effect| effect.id == *id)
                    .ok_or(SessionEditError::InvalidEffectOrder)?;
                ordered.push(source.remove(index));
            }
            *effects = ordered;
        }
        SessionEdit::SetEffectIdentity {
            track_id,
            rack_name,
            effect_id,
            identity,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.identity = identity.clone();
        }
        SessionEdit::SetEffectQuality {
            track_id,
            rack_name,
            effect_id,
            quality,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.quality = *quality;
        }
        SessionEdit::SetEffectSidechain {
            track_id,
            rack_name,
            effect_id,
            sidechain,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.sidechain = sidechain.clone();
        }
        SessionEdit::SetTrackConsole {
            track_id, console, ..
        } => {
            *strip_mut(session, track_id)?.console = console.clone();
        }
        SessionEdit::UpsertSubmix { submix } => {
            upsert(&mut session.submixes, submix, |item| &item.id)
        }
        SessionEdit::RemoveSubmix { submix_id } => {
            remove(&mut session.submixes, submix_id, |item| &item.id)?
        }
        SessionEdit::UpsertOutput { output } => {
            upsert(&mut session.outputs, output, |item| &item.id)
        }
        SessionEdit::RemoveOutput { output_id } => {
            remove(&mut session.outputs, output_id, |item| &item.id)?
        }
        SessionEdit::UpsertRoute { route } => upsert(&mut session.routes, route, |item| &item.id),
        SessionEdit::RemoveRoute { route_id } => {
            remove(&mut session.routes, route_id, |item| &item.id)?
        }
        SessionEdit::SetRouteSource { route_id, source } => {
            route_mut(session, route_id)?.source = source.clone()
        }
        SessionEdit::SetRouteDestination {
            route_id,
            destination,
        } => {
            route_mut(session, route_id)?.destination = destination.clone();
        }
        SessionEdit::UpsertAutomation { automation } => {
            upsert(&mut session.automation, automation, |item| &item.id);
        }
        SessionEdit::RemoveAutomation { automation_id } => {
            remove(&mut session.automation, automation_id, |item| &item.id)?;
        }
        SessionEdit::SetAutomationTarget {
            automation_id,
            target,
        } => {
            automation_mut(session, automation_id)?.target = target.clone();
        }
        SessionEdit::SetAutomationSegments {
            automation_id,
            segments,
        } => {
            if segments.is_empty() {
                return Err(SessionEditError::EmptyAutomationSegments);
            }
            automation_mut(session, automation_id)?.segments = segments.clone();
        }
        SessionEdit::UpsertVca { vca } => upsert(&mut session.vcas, vca, |item| &item.id),
        SessionEdit::RemoveVca { vca_id } => remove(&mut session.vcas, vca_id, |item| &item.id)?,
        // Every check runs inside `set`, before its one model write (the acked-batch rule).
        SessionEdit::SetControl {
            address,
            value,
            ramp_samples,
            expected_effect,
        } => {
            if let Some(expected) = expected_effect {
                controls::check_expected_effect(session, address, expected)
                    .map_err(SessionEditError::Control)?;
            }
            let sample_rate_hz = session.sample_rate_hz;
            controls::set(session, address, *value, *ramp_samples, sample_rate_hz)
                .map_err(SessionEditError::Control)?;
        }
    }
    Ok(())
}

/// A committed transaction result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionCommit {
    /// The exact committed session-model revision.
    pub revision: SessionRevision,
    /// Number of typed edits applied in wire order.
    pub applied_operations: usize,
}

/// An owned, not-yet-authoritative session compilation produced by resolving one exact
/// transaction against one exact base revision.  Dropping it changes no live session state.
///
/// This type intentionally exposes only the immutable compilation needed by downstream
/// control-plane preparation.  Installation remains package-private so protocol replay, events,
/// and the authoritative model are committed together by [`crate::ProtocolController`].
pub struct PreparedSessionTransaction {
    base_revision: SessionRevision,
    form: TransactionForm,
    applied_operations: usize,
    edit_ramps: EditRamps,
    monitor_edits: Vec<controls::MonitorWrite>,
    monitor_only: bool,
    set_control_rows: Option<Vec<controls::ControlAddress>>,
}

/// What a prepared transaction holds of the session it would commit.
enum TransactionForm {
    /// The complete prospective compilation: the clone of the model with the edits applied,
    /// validated, normalized and written as canonical text.
    Compiled(CompiledSession),
    /// A transaction of `SetControl`s on existing values (#1613): the owners the edits changed and
    /// nothing else of the model. The committed compilation is not cloned, compiled or serialized;
    /// the commit installs the owners in it.
    InPlace(InPlaceEdit),
    /// A transaction of solo writes only: the model, the revision and the compilation stay as
    /// they are (#1559 D4), and nothing of them is cloned. It keeps its edits, for
    /// [`SessionStore::materialize`].
    Monitor(Vec<SessionEdit>),
}

/// The owners a transaction of values changed (#1613).
struct InPlaceEdit {
    /// The session header and the edited owners, in ID order (`controls::OwnerEdits`).
    owners: SessionModel,
    /// Where each edited owner sits in `owners` and in the committed model.
    slots: session::OwnerSlots,
    /// The revision the commit gives the model.
    revision: u64,
    /// The resource estimate of the model after the commit: the committed one with the canonical
    /// text's length moved by the change of the edited owners and of the revision's digits.
    estimate: session::ResourceEstimate,
    /// The transaction's edits, for [`PreparedSessionTransaction::materialize`].
    edits: Vec<SessionEdit>,
}

impl PreparedSessionTransaction {
    /// Borrow the prospective immutable compilation without making it authoritative.
    ///
    /// `None` for a transaction prepared in place ([`SessionStore::prepare_transaction_in_place`]
    /// and [`Self::is_in_place`]): it has no compilation of its own. A host that prepares in place
    /// reads [`Self::edited_owners`] and the committed compilation instead.
    #[must_use]
    pub fn compiled(&self) -> Option<&CompiledSession> {
        match &self.form {
            TransactionForm::Compiled(compiled) => Some(compiled),
            TransactionForm::InPlace(_) | TransactionForm::Monitor(_) => None,
        }
    }

    /// Whether the transaction was prepared in place (#1613): it holds no compilation of its own,
    /// so [`Self::compiled`] is not available. Either it changes values (read
    /// [`Self::edited_owners`] for them) or it is a transaction of solo writes only.
    #[must_use]
    pub const fn is_in_place(&self) -> bool {
        !matches!(self.form, TransactionForm::Compiled(_))
    }

    /// The owners a transaction prepared in place changed: a model with the session header and the
    /// edited tracks, submixes, VCAs and routes, in ID order. `None` for a transaction that holds a
    /// compilation, and for one of solo writes only, which changes no owner.
    #[must_use]
    pub fn edited_owners(&self) -> Option<&SessionModel> {
        match &self.form {
            TransactionForm::InPlace(edit) => Some(&edit.owners),
            TransactionForm::Compiled(_) | TransactionForm::Monitor(_) => None,
        }
    }

    /// The resource estimate of the model after the commit, when the transaction was prepared in
    /// place (the committed estimate for solo writes only); `None` for a transaction that holds a
    /// compilation, whose own [`CompiledSession::resource_estimate`] is that estimate.
    #[must_use]
    pub fn in_place_estimate(&self) -> Option<session::ResourceEstimate> {
        match &self.form {
            TransactionForm::InPlace(edit) => Some(edit.estimate),
            TransactionForm::Compiled(_) | TransactionForm::Monitor(_) => None,
        }
    }

    /// The per-edit ramp lengths the transaction's edits carried (#1394). Transaction metadata:
    /// not in the compiled model, not in canonical JSON, and gone when this value is.
    #[must_use]
    pub const fn edit_ramps(&self) -> &EditRamps {
        &self.edit_ramps
    }

    /// Revision the prospective compilation will own if committed.
    #[must_use]
    pub fn revision(&self) -> SessionRevision {
        match &self.form {
            TransactionForm::Compiled(compiled) => {
                SessionRevision(compiled.normalized_model().revision)
            }
            TransactionForm::InPlace(edit) => SessionRevision(edit.revision),
            // Not a mutation: the committed revision.
            TransactionForm::Monitor(_) => self.base_revision,
        }
    }

    /// The writes to monitor rows (solo) that the transaction holds, in transaction order, with
    /// a later write of one strip replacing the earlier (decision 17, C8, #1559 D3). They are not
    /// part of the compiled model: the host applies them to its monitor overlay when it commits.
    #[must_use]
    pub fn monitor_edits(&self) -> &[controls::MonitorWrite] {
        &self.monitor_edits
    }

    /// The addresses of the rows the transaction wrote to the model, in transaction order, when
    /// **every** edit of the transaction is a `SetControl` (a write to a monitor row is in
    /// [`Self::monitor_edits`], not here); `None` as soon as one edit is another kind. A host that
    /// classifies the transaction reads the rows it names instead of comparing the two models
    /// whole, because a `SetControl` writes its row and nothing else (#1612 D2).
    #[must_use]
    pub fn set_control_rows(&self) -> Option<&[controls::ControlAddress]> {
        self.set_control_rows.as_deref()
    }

    /// Whether every edit of the transaction is a write to a monitor row. Such a transaction
    /// leaves the model and the revision as they are: [`Self::compiled`] is the committed
    /// compilation and [`Self::revision`] the committed revision (#1559 D4).
    #[must_use]
    pub const fn is_monitor_only(&self) -> bool {
        self.monitor_only
    }
}

/// Atomic transaction rejection with a precise resolving operation index when one exists.
#[derive(Clone, Debug, PartialEq)]
pub enum SessionStoreError {
    /// Structural commands require an exact revision, never `Any`.
    ExactRevisionRequired,
    /// A transaction must contain at least one typed edit.
    EmptyTransaction,
    /// The request's exact revision does not match current authoritative state.
    RevisionConflict {
        /// Current authoritative revision.
        current: SessionRevision,
    },
    /// The model revision is at the plan revision gate's ceiling, `GATE_REVISION_MAX`, so a
    /// further commit would be unrepresentable in the gate.
    RevisionExhausted,
    /// One edit could not resolve against the candidate. Index is in wire order.
    Edit {
        /// Zero-based operation index.
        operation_index: usize,
        /// Typed resolution error.
        error: SessionEditError,
    },
    /// Final issue-004 compilation failed after all edits. Index equals the operation count.
    Validation {
        /// Index immediately following the last successfully resolved edit.
        operation_index: usize,
        /// Accepted session compiler diagnostics.
        diagnostics: session::DiagnosticSet,
    },
}

impl fmt::Display for SessionStoreError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SessionStoreError {}

/// The control-plane authoritative session triple, with no render-plan publication capability.
pub struct SessionStore {
    caps: CompileCaps,
    compiled: CompiledSession,
}

impl SessionStore {
    /// Compile the first authoritative typed model under fixed control-plane compiler caps.
    pub fn new(initial: SessionModel, caps: CompileCaps) -> Result<Self, session::DiagnosticSet> {
        Ok(Self {
            compiled: compile_session(&initial, caps)?,
            caps,
        })
    }

    /// Borrow the immutable accepted control-plane compilation artifact.
    #[must_use]
    pub const fn compiled(&self) -> &CompiledSession {
        &self.compiled
    }

    /// Read the current authoritative model revision.
    #[must_use]
    pub fn revision(&self) -> SessionRevision {
        SessionRevision(self.compiled.normalized_model().revision)
    }

    /// Borrow the exact canonical snapshot cached by issue-004 compilation.
    #[must_use]
    pub fn canonical_snapshot(&self) -> &str {
        self.compiled.canonical_json()
    }

    /// Resolve every edit in wire order, compile the final candidate, and atomically retain it.
    /// Any failure leaves the prior `CompiledSession`, model, revision, and snapshot untouched.
    pub fn apply_transaction(
        &mut self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<SessionCommit, SessionStoreError> {
        let prepared = self.prepare_transaction(expected_revision, edits)?;
        Ok(self.commit_prepared(prepared))
    }

    /// [`Self::apply_transaction`] for a host that owns a monitor overlay: the preparation is
    /// [`Self::prepare_transaction_with_monitor`]'s. A transaction of solo writes only commits no
    /// model: the result names the committed revision and the store is as it was.
    pub fn apply_transaction_with_monitor(
        &mut self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<SessionCommit, SessionStoreError> {
        let prepared = self.prepare_transaction_with_monitor(expected_revision, edits)?;
        Ok(self.commit_or_keep(prepared))
    }

    /// [`Self::apply_transaction_with_monitor`] with the preparation of
    /// [`Self::prepare_transaction_in_place`]: the result, and the store after it, are those of the
    /// full path for the same transaction, whichever form the preparation took.
    pub fn apply_transaction_in_place(
        &mut self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<SessionCommit, SessionStoreError> {
        let prepared = self.prepare_transaction_in_place(expected_revision, edits)?;
        Ok(self.commit_or_keep(prepared))
    }

    fn commit_or_keep(&mut self, prepared: PreparedSessionTransaction) -> SessionCommit {
        if prepared.monitor_only {
            return SessionCommit {
                revision: prepared.base_revision,
                applied_operations: prepared.applied_operations,
            };
        }
        self.commit_prepared(prepared)
    }

    /// Resolve and compile one transaction without changing the authoritative session triple.
    /// The returned affine value owns every prospective allocation and is safe to discard.
    pub fn prepare_transaction(
        &self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<PreparedSessionTransaction, SessionStoreError> {
        self.prepare_transaction_with(expected_revision, edits, false, false)
    }

    /// [`Self::prepare_transaction`] for a host that owns a monitor overlay (decision 17, C8,
    /// #1559 D3): a `SetControl` whose row is a monitor row (solo) is validated against the
    /// candidate at its place in the transaction, written nowhere, and recorded as one of the
    /// preparation's [`PreparedSessionTransaction::monitor_edits`]. Without the overlay
    /// ([`Self::prepare_transaction`]) the same edit is refused with `control.monitor`.
    ///
    /// A transaction of monitor writes only prepares the committed compilation at the committed
    /// revision: it is not a session mutation, so it needs no new revision.
    pub fn prepare_transaction_with_monitor(
        &self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<PreparedSessionTransaction, SessionStoreError> {
        self.prepare_transaction_with(expected_revision, edits, true, false)
    }

    /// [`Self::prepare_transaction_with_monitor`] for a host that commits a transaction of values
    /// without a compilation (#1613).
    ///
    /// A transaction whose edits are all `SetControl`s on values that exist is prepared **in
    /// place**: the owners the edits name are cloned and edited with the very `controls::set` the
    /// full path uses, checked with the session's own value validators, and held with the exact
    /// resource estimate and the revision the commit gives. The committed model is not cloned,
    /// compiled or serialized, and nothing of it is written until the commit. A
    /// transaction of solo writes only is prepared as such, with no clone of the compilation.
    ///
    /// Anything else, and any transaction the in-place preparation cannot prove equal to the full
    /// one, is prepared by the full path and returned as [`Self::prepare_transaction_with_monitor`]
    /// returns it, refusals included: that function alone decides which refusal wins. The in-place
    /// preparation accepts a transaction only if the full path would accept it:
    ///
    /// - every edit is a `SetControl`, on a model row of a strip, a route or a VCA, or on a solo row;
    /// - `controls::set` accepts each edit in order, and the edited owners pass the validators of
    ///   `compile_session` (`session::owners_values_valid`);
    /// - the edited owners hold as many console entries, inserts, parameter entries and members as
    ///   the committed ones, so the resource estimate counts the same and the canonical text keeps
    ///   its structure (a parameter entry that does not exist yet is the full path);
    /// - no ramp is longer than a second, and the canonical text stays within its preflight bound.
    ///
    /// # Errors
    ///
    /// As [`Self::prepare_transaction_with_monitor`].
    pub fn prepare_transaction_in_place(
        &self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
    ) -> Result<PreparedSessionTransaction, SessionStoreError> {
        self.prepare_transaction_with(expected_revision, edits, true, true)
    }

    /// The full preparation of a transaction prepared in place: the same edits, compiled, as
    /// [`Self::prepare_transaction_with_monitor`] gives them at the committed revision. A host that
    /// finds the in-place transaction needs the full path (a rebuild, a classification the fast path
    /// cannot make) calls this and goes on as it always did.
    ///
    /// # Errors
    ///
    /// As [`Self::prepare_transaction_with_monitor`]; an in-place preparation accepts only what the
    /// full one accepts, so an error here is a defect of the host or the store.
    pub fn materialize(
        &self,
        prepared: PreparedSessionTransaction,
    ) -> Result<PreparedSessionTransaction, SessionStoreError> {
        let edits = match &prepared.form {
            TransactionForm::Compiled(_) => return Ok(prepared),
            TransactionForm::InPlace(edit) => &edit.edits,
            TransactionForm::Monitor(edits) => edits,
        };
        self.prepare_transaction_with(
            ExpectedRevision::Exact(prepared.base_revision),
            edits,
            true,
            false,
        )
    }

    fn prepare_transaction_with(
        &self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
        monitor: bool,
        in_place: bool,
    ) -> Result<PreparedSessionTransaction, SessionStoreError> {
        let ExpectedRevision::Exact(expected_revision) = expected_revision else {
            return Err(SessionStoreError::ExactRevisionRequired);
        };
        let current = self.revision();
        if expected_revision != current {
            return Err(SessionStoreError::RevisionConflict { current });
        }
        if edits.is_empty() {
            return Err(SessionStoreError::EmptyTransaction);
        }
        // A transaction of monitor writes commits no revision (#1559 D4, #1603): it advances
        // nothing, so the revision ceiling does not refuse it. The host publishes its records, if
        // any, under the committed revision with a monitor publication.
        let monitor_only = monitor && is_monitor_transaction(edits);
        // The plan revision gate cannot hold a revision above its ceiling (#1432, #1503): the
        // store's own typed refusal comes after the cheap argument checks and before the candidate
        // clone, any edit and the compile, so it wins over the schema's `$.revision` refusal and
        // nothing is written.
        if !monitor_only && current.0 >= engine::realtime::GATE_REVISION_MAX {
            return Err(SessionStoreError::RevisionExhausted);
        }
        if in_place && let Some(prepared) = self.prepare_in_place(current, edits, monitor_only) {
            return Ok(prepared);
        }
        let next_revision = current.0 + 1;
        let mut candidate = self.compiled.normalized_model().clone();
        let mut edit_ramps = EditRamps::new();
        let mut control_writes = ControlRampWrites::new();
        let mut monitor_edits: Vec<controls::MonitorWrite> = Vec::new();
        let mut set_control_rows: Option<Vec<controls::ControlAddress>> = Some(Vec::new());
        // #1561 D3: `SetTrackConsole` is structural, so it is accepted only after a `SetConsole`
        // of the same transaction actually changed the console: the candidate's console, with the
        // earlier edits applied, must differ from the committed one.
        for (operation_index, edit) in edits.iter().enumerate() {
            let refused = |error| SessionStoreError::Edit {
                operation_index,
                error,
            };
            if matches!(edit, SessionEdit::SetTrackConsole { .. })
                && candidate.console == self.compiled.normalized_model().console
            {
                return Err(refused(SessionEditError::ValueOpRetired));
            }
            if monitor
                && let SessionEdit::SetControl {
                    address,
                    value,
                    ramp_samples,
                    expected_effect,
                } = edit
            {
                // A monitor row runs no effect: an expected effect on it is refused (#1678).
                if let Some(expected) = expected_effect {
                    controls::check_expected_effect(&candidate, address, expected)
                        .map_err(|error| refused(SessionEditError::Control(error)))?;
                }
                let rate = candidate.sample_rate_hz;
                // The monitor sort of #1559 D3: a monitor row is checked here and written to
                // the overlay by the host; a model row goes through `set`.
                let write =
                    controls::check_monitor(&candidate, address, *value, *ramp_samples, rate)
                        .map_err(|error| refused(SessionEditError::Control(error)))?;
                if let Some(write) = write {
                    // A later write of one row of one strip replaces the earlier one.
                    monitor_edits
                        .retain(|earlier| earlier.strip != write.strip || earlier.row != write.row);
                    monitor_edits.push(write);
                    continue;
                }
            }
            apply_session_edit(&mut candidate, edit).map_err(refused)?;
            match (edit, &mut set_control_rows) {
                (SessionEdit::SetControl { address, .. }, Some(rows)) => rows.push(address.clone()),
                _ => set_control_rows = None,
            }
            record_edit_ramps(&mut edit_ramps, &mut control_writes, edit)
                .map_err(|error| refused(SessionEditError::Control(error)))?;
        }
        let compiled = if monitor_only {
            // Not a mutation: the committed compilation, at the committed revision.
            self.compiled.clone()
        } else {
            candidate.revision = next_revision;
            compile_session(&candidate, self.caps).map_err(|diagnostics| {
                SessionStoreError::Validation {
                    operation_index: edits.len(),
                    diagnostics,
                }
            })?
        };
        // #1394 D4: every length is bounded by one second at the rate the transaction commits,
        // so the check follows the compile that validated that rate and precedes any commit.
        let maximum_ramp = compiled.normalized_model().sample_rate_hz;
        if let Some(operation_index) = edits
            .iter()
            .position(|edit| edit.ramp_samples().is_some_and(|ramp| ramp > maximum_ramp))
        {
            return Err(SessionStoreError::Edit {
                operation_index,
                error: SessionEditError::RampOutOfRange,
            });
        }
        Ok(PreparedSessionTransaction {
            base_revision: current,
            form: TransactionForm::Compiled(compiled),
            applied_operations: edits.len(),
            edit_ramps,
            monitor_edits,
            monitor_only,
            set_control_rows,
        })
    }

    /// Install a preparation whose base revision was checked by the controller immediately
    /// before this call.  Moving the complete compilation is allocation-free and non-fallible; so
    /// is installing the owners of a transaction prepared in place, which is the first write of the
    /// committed model (#1613): a swap of each edited owner, then the revision and the estimate.
    pub(crate) fn commit_prepared(
        &mut self,
        prepared: PreparedSessionTransaction,
    ) -> SessionCommit {
        debug_assert_eq!(self.revision(), prepared.base_revision);
        debug_assert!(
            !prepared.monitor_only,
            "a monitor-only transaction commits no model"
        );
        let revision = prepared.revision();
        let applied_operations = prepared.applied_operations;
        match prepared.form {
            TransactionForm::Compiled(compiled) => self.compiled = compiled,
            TransactionForm::InPlace(mut edit) => {
                self.compiled.swap_owners(&mut edit.owners, &edit.slots);
                self.compiled.finish_in_place(edit.revision, edit.estimate);
            }
            TransactionForm::Monitor(_) => {
                unreachable!("a monitor-only transaction commits no model")
            }
        }
        SessionCommit {
            revision,
            applied_operations,
        }
    }

    /// The in-place preparation of [`Self::prepare_transaction_in_place`]: `None` unless it can
    /// prove the transaction is a change of values the full path would accept.
    fn prepare_in_place(
        &self,
        current: SessionRevision,
        edits: &[SessionEdit],
        monitor_only: bool,
    ) -> Option<PreparedSessionTransaction> {
        let committed = self.compiled.normalized_model();
        let rate = committed.sample_rate_hz;
        // The full path bounds every ramp by one second at the rate the transaction commits; the
        // rate is the committed one, which a `SetControl` cannot change.
        if edits
            .iter()
            .any(|edit| edit.ramp_samples().is_some_and(|ramp| ramp > rate))
        {
            return None;
        }
        let mut owners = controls::OwnerEdits::new(committed);
        let mut edit_ramps = EditRamps::new();
        let mut control_writes = ControlRampWrites::new();
        let mut monitor_edits: Vec<controls::MonitorWrite> = Vec::new();
        let mut rows: Vec<controls::ControlAddress> = Vec::new();
        for edit in edits {
            let SessionEdit::SetControl {
                address,
                value,
                ramp_samples,
                expected_effect,
            } = edit
            else {
                return None;
            };
            // An edit written for another effect than the one that runs is the full path's to
            // refuse, with its typed code (#1678).
            if let Some(expected) = expected_effect {
                controls::check_expected_effect(committed, address, expected).ok()?;
            }
            // The monitor sort of #1559 D3, as the full path makes it.
            if let Some(write) =
                controls::check_monitor(committed, address, *value, *ramp_samples, rate).ok()?
            {
                monitor_edits
                    .retain(|earlier| earlier.strip != write.strip || earlier.row != write.row);
                monitor_edits.push(write);
                continue;
            }
            if !controls::writes_only_its_row(address) {
                return None;
            }
            owners.set(address, *value, *ramp_samples, rate).ok()?;
            rows.push(address.clone());
            record_edit_ramps(&mut edit_ramps, &mut control_writes, edit).ok()?;
        }
        if monitor_only {
            return Some(PreparedSessionTransaction {
                base_revision: current,
                form: TransactionForm::Monitor(edits.to_vec()),
                applied_operations: edits.len(),
                edit_ramps,
                monitor_edits,
                monitor_only: true,
                set_control_rows: Some(rows),
            });
        }
        let (owners, slots) = owners.finish();
        if !session::owners_values_valid(slots.in_owners(&owners))
            || !session::owners_counts_equal(
                slots.in_committed(committed),
                slots.in_owners(&owners),
            )
        {
            return None;
        }
        let estimate = self.compiled.resource_estimate();
        let before = session::canonical_owner_bytes(slots.in_committed(committed));
        let after = session::canonical_owner_bytes(slots.in_owners(&owners));
        let digits = |revision: u64| u64::from(revision.checked_ilog10().unwrap_or(0)) + 1;
        let revision = current.0 + 1;
        let canonical_bytes = estimate
            .canonical_bytes
            .checked_sub(u64::try_from(before).ok()?)?
            .checked_add(u64::try_from(after).ok()?)?
            .checked_sub(digits(current.0))?
            .checked_add(digits(revision))?;
        // The full path refuses a canonical text over its preflight bound.
        if canonical_bytes > estimate.canonical_upper_bound_bytes {
            return None;
        }
        let compiled_model_bytes = estimate
            .compiled_model_bytes
            .checked_sub(estimate.canonical_bytes)?
            .checked_add(canonical_bytes)?;
        let estimate = session::ResourceEstimate {
            canonical_bytes,
            compiled_model_bytes,
            estimated_bytes: compiled_model_bytes.checked_add(estimate.requested_runtime_bytes)?,
            ..estimate
        };
        Some(PreparedSessionTransaction {
            base_revision: current,
            form: TransactionForm::InPlace(InPlaceEdit {
                owners,
                slots,
                revision,
                estimate,
                edits: edits.to_vec(),
            }),
            applied_operations: edits.len(),
            edit_ramps,
            monitor_edits,
            monitor_only: false,
            set_control_rows: Some(rows),
        })
    }
}

/// Whether every edit of a non-empty transaction is a `SetControl` on a monitor row (solo).
pub(crate) fn is_monitor_transaction(edits: &[SessionEdit]) -> bool {
    !edits.is_empty() && edits.iter().all(is_monitor_row_edit)
}

/// Whether `edit` is a `SetControl` on a monitor row (solo). Decided by the address alone: the
/// row's table says its scope, and the strip is looked up when the edit is checked.
fn is_monitor_row_edit(edit: &SessionEdit) -> bool {
    let SessionEdit::SetControl { address, .. } = edit else {
        return false;
    };
    controls::row_scope(
        address.owner.kind(),
        address.section.kind(),
        address.control,
    ) == Some(controls::ControlScope::Monitor)
}

fn upsert<T: Clone>(items: &mut Vec<T>, item: &T, id: impl Fn(&T) -> &StableId) {
    if let Some(index) = items.iter().position(|existing| id(existing) == id(item)) {
        items[index] = item.clone();
    } else {
        items.push(item.clone());
    }
}

fn remove<T>(
    items: &mut Vec<T>,
    id: &StableId,
    item_id: impl Fn(&T) -> &StableId,
) -> Result<(), SessionEditError> {
    let index = items
        .iter()
        .position(|item| item_id(item) == id)
        .ok_or(SessionEditError::NotFound)?;
    items.remove(index);
    Ok(())
}

fn source_mut<'a>(
    session: &'a mut SessionModel,
    id: &StableId,
) -> Result<&'a mut Source, SessionEditError> {
    session
        .sources
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

fn track_mut<'a>(
    session: &'a mut SessionModel,
    id: &StableId,
) -> Result<&'a mut session::Track, SessionEditError> {
    session
        .tracks
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

/// The two strip fields a structural strip edit writes, borrowed from a track or a submix.
struct StripMut<'a> {
    console: &'a mut Vec<ConsoleEntry>,
    inserts: &'a mut Rack,
}

/// Resolve the strip a strip edit names (#1204 D1, D4): a track ID names the track's strip and a
/// submix ID the submix's. Tracks are searched first, then submixes; an output ID, like any other
/// unknown ID, is [`SessionEditError::NotFound`].
///
/// IDs are unique across tracks, submixes and outputs only in a validated model. Inside a
/// transaction, before final validation, a track and a submix may briefly share an ID (an
/// `UpsertSubmix` is a plain upsert), and then the track wins.
fn strip_mut<'a>(
    session: &'a mut SessionModel,
    id: &StableId,
) -> Result<StripMut<'a>, SessionEditError> {
    let SessionModel {
        tracks, submixes, ..
    } = session;
    if let Some(track) = tracks.iter_mut().find(|item| &item.id == id) {
        return Ok(StripMut {
            console: &mut track.console,
            inserts: &mut track.inserts,
        });
    }
    let submix = submixes
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)?;
    Ok(StripMut {
        console: &mut submix.console,
        inserts: &mut submix.inserts,
    })
}

fn route_mut<'a>(
    session: &'a mut SessionModel,
    id: &StableId,
) -> Result<&'a mut Route, SessionEditError> {
    session
        .routes
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

fn automation_mut<'a>(
    session: &'a mut SessionModel,
    id: &StableId,
) -> Result<&'a mut Automation, SessionEditError> {
    session
        .automation
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

/// The `Rack` a `SessionEdit` structural rack-addressed edit names.
///
/// Only a strip's `inserts` -- a track's or a submix's, resolved by [`strip_mut`] -- is a rack of
/// per-strip effect instances.
///
/// - `RackName::Builtins` (#178, ruled by #210's D2) is the strip's own builtin section, a
///   `DualMonoBuiltins` with no `effects` vector, so every edit that reaches here is addressing
///   something that does not exist. It is refused with [`SessionEditError::NotFound`], the answer a
///   named-but-absent effect gets, rather than given a panicking arm or a silent no-op. The strip
///   is edited through `SetControl`'s builtin rows; the token exists for the
///   automation-target vocabulary.
/// - `RackName::Console` (decision 12) names session-level slots. A strip, track or submix, cannot
///   add, remove or reorder one, and a slot's declaration is not per-strip, so a structural or
///   declaration edit naming it is refused with [`SessionEditError::ConsoleSlotFixed`] before the
///   strip is even looked up: the refusal is a property of the edit, not of the model it meets.
fn rack_mut<'a>(
    session: &'a mut SessionModel,
    strip_id: &StableId,
    rack_name: RackName,
) -> Result<&'a mut Rack, SessionEditError> {
    match rack_name {
        RackName::Inserts => Ok(strip_mut(session, strip_id)?.inserts),
        RackName::Builtins => Err(SessionEditError::NotFound),
        RackName::Console => Err(SessionEditError::ConsoleSlotFixed),
    }
}

/// The insert a declaration edit (`SetEffectIdentity`, `SetEffectQuality`,
/// `SetEffectSidechain`) names. A console slot's declaration is the session's, so `console` is
/// refused by [`rack_mut`].
fn effect_mut<'a>(
    session: &'a mut SessionModel,
    strip_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
) -> Result<&'a mut Effect, SessionEditError> {
    rack_mut(session, strip_id, rack_name)?
        .effects
        .iter_mut()
        .find(|effect| &effect.id == effect_id)
        .ok_or(SessionEditError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::control_rows::row_address;
    use controls::{Lane, Owner, Section};
    use session::{ControlSmoothing, ParameterChannel, canonical_session_json, parse_session_json};

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.json");

    /// Largest revision a plan's revision gate can hold; the session schema's `revision` domain.
    const CEILING: u64 = engine::realtime::GATE_REVISION_MAX;

    fn caps() -> CompileCaps {
        CompileCaps {
            max_compiled_model_bytes: u64::MAX,
            max_requested_runtime_bytes: u64::MAX,
            max_single_allocation_bytes: u64::MAX,
            max_queue_items: u64::MAX,
            max_source_ring_frames: u64::MAX,
            max_source_ring_bytes: u64::MAX,
        }
    }

    fn store() -> SessionStore {
        SessionStore::new(parse_session_json(EXAMPLE).expect("fixture"), caps()).expect("compile")
    }

    fn id(value: &str) -> StableId {
        StableId::parse(value).expect("test stable ID")
    }

    /// A `SET_CONTROL` of the row `row` of `route`.
    fn route_row(route: &str, row: &str, value: f32) -> SessionEdit {
        SessionEdit::set_control(
            row_address(Owner::Route(id(route)), Section::Own, row, Lane::Both)
                .expect("a route row"),
            value,
            None,
        )
    }

    /// A `SET_CONTROL` of the session row `row`.
    fn session_row(row: &str, value: f32) -> SessionEdit {
        SessionEdit::set_control(
            row_address(Owner::Session, Section::Own, row, Lane::Both).expect("a session row"),
            value,
            None,
        )
    }

    /// #1613. A transaction of values is prepared in place, holds the edited owners and no
    /// compilation, commits to the session the full path commits to, and is materialized into the
    /// full preparation on demand; a transaction of solo writes only holds nothing; a structural edit
    /// and a refusal are the full path's.
    ///
    /// Red if the in-place commit differs from the full one in model, snapshot, estimate or revision,
    /// if materializing gives another revision or model, or if an edit the in-place form cannot prove
    /// is not handed to the full path.
    #[test]
    fn a_transaction_of_values_is_prepared_in_place_and_commits_as_the_full_path_does() {
        let mut fast = store();
        let mut full = store();
        let revision = ExpectedRevision::Exact(fast.revision());
        let gain = route_row("to-main", "gain_db", -3.123_456);
        let mute = route_row("to-main", "mute", 1.0);
        let edits = [gain.clone(), mute];
        let prepared = fast
            .prepare_transaction_in_place(revision, &edits)
            .expect("prepares");
        assert!(prepared.is_in_place());
        assert_eq!(
            prepared.edited_owners().expect("owners").routes.len(),
            1,
            "one route is edited, not the model"
        );
        assert_eq!(prepared.revision().0, fast.revision().0 + 1);
        let materialized = fast.materialize(prepared).expect("materializes");
        assert!(!materialized.is_in_place());
        let oracle = full
            .prepare_transaction_with_monitor(revision, &edits)
            .expect("prepares");
        assert_eq!(materialized.revision(), oracle.revision());
        assert_eq!(
            materialized
                .compiled()
                .expect("materialized")
                .normalized_model(),
            oracle.compiled().expect("compiled").normalized_model()
        );
        assert_eq!(
            fast.apply_transaction_in_place(revision, &edits),
            full.apply_transaction_with_monitor(revision, &edits)
        );
        assert_eq!(fast.canonical_snapshot(), full.canonical_snapshot());
        assert_eq!(
            fast.compiled().resource_estimate(),
            full.compiled().resource_estimate()
        );
        assert_eq!(
            fast.compiled().normalized_model(),
            full.compiled().normalized_model()
        );
        assert_eq!(fast.revision(), full.revision());

        // Solo writes alone: no compilation, no owner, no commit.
        let track = fast.compiled().normalized_model().tracks[0].id.clone();
        let solo = SessionEdit::set_control(
            controls::ControlAddress {
                owner: Owner::Strip(track),
                section: Section::Own,
                control: 2,
                lane: Lane::Both,
            },
            1.0,
            None,
        );
        let revision = ExpectedRevision::Exact(fast.revision());
        let prepared = fast
            .prepare_transaction_in_place(revision, std::slice::from_ref(&solo))
            .expect("prepares");
        assert!(prepared.is_in_place() && prepared.is_monitor_only());
        assert!(prepared.edited_owners().is_none());
        assert_eq!(prepared.revision(), fast.revision());
        let before = fast.revision();
        fast.apply_transaction_in_place(revision, std::slice::from_ref(&solo))
            .expect("applies");
        assert_eq!(fast.revision(), before, "a solo is no mutation");

        // What the in-place form cannot prove is the full path's.
        let upsert = SessionEdit::UpsertVca {
            vca: session::Vca {
                id: id("extra"),
                fader: session::DualMonoFader {
                    left_db: 0.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: Vec::new(),
            },
        };
        assert!(
            !fast
                .prepare_transaction_in_place(revision, &[gain, upsert])
                .expect("prepares")
                .is_in_place()
        );
        let refused = route_row("to-main", "gain_db", 400.0);
        assert_eq!(
            fast.prepare_transaction_in_place(revision, std::slice::from_ref(&refused))
                .err(),
            fast.prepare_transaction_with_monitor(revision, std::slice::from_ref(&refused))
                .err()
        );
    }

    /// #1612 D2. `set_control_rows` names the rows a transaction of `SetControl`s wrote, in
    /// transaction order, and is `None` as soon as one edit is another kind. A write to a monitor
    /// row is not a model write, so it is not in the list, and a transaction of such writes alone
    /// has an empty list.
    ///
    /// Red if a structural edit leaves the list `Some` (a host would then classify the transaction
    /// from the rows and miss the structure), or a row of the transaction is missing from it.
    #[test]
    fn set_control_rows_name_the_rows_of_a_transaction_of_set_controls() {
        let store = store();
        let revision = ExpectedRevision::Exact(store.revision());
        let rows = |edits: &[SessionEdit]| {
            store
                .prepare_transaction_with_monitor(revision, edits)
                .expect("prepares")
                .set_control_rows()
                .map(<[_]>::to_vec)
        };
        let mute = route_row("to-main", "mute", 1.0);
        let gain = route_row("to-main", "gain_db", -3.0);
        let wanted = |edit: &SessionEdit| match edit {
            SessionEdit::SetControl { address, .. } => address.clone(),
            _ => unreachable!("a SetControl"),
        };
        assert_eq!(
            rows(&[mute.clone(), gain.clone()]),
            Some(vec![wanted(&mute), wanted(&gain)])
        );
        let track = store.compiled().normalized_model().tracks[0].id.clone();
        let solo = SessionEdit::set_control(
            controls::ControlAddress {
                owner: Owner::Strip(track),
                section: Section::Own,
                control: 2,
                lane: Lane::Both,
            },
            1.0,
            None,
        );
        assert_eq!(rows(std::slice::from_ref(&solo)), Some(Vec::new()));
        assert_eq!(
            rows(&[solo.clone(), gain.clone()]),
            Some(vec![wanted(&gain)])
        );
        let structural = SessionEdit::RemoveVca {
            vca_id: id("nobody"),
        };
        // A structural edit among them is refused by its own resolution here (no such VCA), so the
        // case that matters is a structural edit that applies: an upsert.
        assert!(
            store
                .prepare_transaction_with_monitor(revision, &[structural])
                .is_err()
        );
        let upsert = SessionEdit::UpsertVca {
            vca: session::Vca {
                id: id("extra"),
                fader: session::DualMonoFader {
                    left_db: 0.0,
                    right_db: 0.0,
                    left_mute: false,
                    right_mute: false,
                },
                members: Vec::new(),
            },
        };
        assert_eq!(rows(&[gain.clone(), upsert.clone()]), None);
        assert_eq!(rows(&[upsert, gain]), None);
    }

    #[test]
    fn transaction_revisions_and_snapshot_are_authoritative() {
        let mut store = store();
        let expected_snapshot = store.canonical_snapshot().to_owned();
        let commit = store
            .apply_transaction(
                ExpectedRevision::Exact(SessionRevision(7)),
                &[SessionEdit::SetSessionId {
                    session_id: id("next-session"),
                }],
            )
            .expect("commit");
        assert_eq!(commit.revision, SessionRevision(8));
        assert_eq!(commit.applied_operations, 1);
        assert_eq!(store.compiled().normalized_model().revision, 8);
        assert_ne!(store.canonical_snapshot(), expected_snapshot);
        assert_eq!(
            store.canonical_snapshot(),
            canonical_session_json(store.compiled().normalized_model()).expect("canonical")
        );
    }

    /// #1216 D1, #1561: a `SET_CONTROL` of a route's `mute` row switches one route in the
    /// committed model and its canonical snapshot, and switches it back, leaving every other byte
    /// of the snapshot.
    ///
    /// Red if the address writes the wrong route or field, or ignores the value (always muting).
    #[test]
    fn set_route_mute_switches_the_route_in_the_committed_snapshot() {
        let mut store = store();
        let open = store.canonical_snapshot().to_owned();
        assert!(open.contains("\"gain_db\": 0.0,\n      \"mute\": false,\n"));
        for (revision, mute) in [(7, true), (8, false)] {
            store
                .apply_transaction(
                    ExpectedRevision::Exact(SessionRevision(revision)),
                    &[route_row("to-main", "mute", f32::from(u8::from(mute)))],
                )
                .expect("route mute commits");
            assert_eq!(store.compiled().normalized_model().routes[0].mute, mute);
            let expected = open
                .replacen(
                    "\"revision\": \"7\"",
                    &format!("\"revision\": \"{}\"", revision + 1),
                    1,
                )
                .replacen("\"mute\": false", &format!("\"mute\": {mute}"), 1);
            assert_eq!(store.canonical_snapshot(), expected);
        }
    }

    /// #1365 D1, D3 and #1561 D4a: `control_smoothing` is written by `SET_CONTROL` of the session
    /// rows alone. The first write turns `{ "kind": "default" }` into the explicit object, with the
    /// two rows it did not name at their defaults, and each later write moves its own length only.
    /// No transaction can write the `default` arm back.
    ///
    /// Red if a row writes another length (a field into its neighbour), if the unnamed rows are
    /// not taken from the default table, or if a later write resets the earlier ones.
    #[test]
    fn set_control_of_the_session_rows_sets_the_key_in_the_committed_snapshot() {
        let mut store = store();
        let open = store.canonical_snapshot().to_owned();
        let default = "\"control_smoothing\": {\n    \"kind\": \"default\"\n  },";
        assert!(open.contains(default));
        let base = session::CONTROL_SMOOTHING_DEFAULT;
        let explicit = |l: ControlSmoothing| {
            format!(
                "\"control_smoothing\": {{\n    \"kind\": \"explicit\",\n    \"mute_ms\": {:?},\n    \"fader_ms\": {:?},\n    \"pan_ms\": {:?}\n  }},",
                l.mute_ms, l.fader_ms, l.pan_ms
            )
        };
        // Three different keys, so a field cannot be written into its neighbour.
        let mut expected = base;
        for (revision, row, value) in [
            (7, "fader_ms", 15.0),
            (8, "mute_ms", 5.0),
            (9, "pan_ms", 25.0),
        ] {
            match row {
                "fader_ms" => expected.fader_ms = value,
                "mute_ms" => expected.mute_ms = value,
                _ => expected.pan_ms = value,
            }
            store
                .apply_transaction(
                    ExpectedRevision::Exact(SessionRevision(revision)),
                    &[session_row(row, value)],
                )
                .expect("control smoothing commits");
            assert_eq!(
                store.compiled().normalized_model().control_smoothing,
                Some(expected)
            );
            let snapshot = open
                .replacen(
                    "\"revision\": \"7\"",
                    &format!("\"revision\": \"{}\"", revision + 1),
                    1,
                )
                .replacen(default, &explicit(expected), 1);
            assert_eq!(store.canonical_snapshot(), snapshot, "revision {revision}");
        }
    }

    #[test]
    fn every_launch_rate_initializes_store_and_commits_as_the_final_candidate() {
        for rate in engine::LAUNCH_SAMPLE_RATES {
            let mut initial = parse_session_json(EXAMPLE).expect("fixture");
            initial.sample_rate_hz = rate.0;
            let mut store = SessionStore::new(initial, caps()).expect("launch store");
            let revision = store.revision();
            let commit = store
                .apply_transaction(
                    ExpectedRevision::Exact(revision),
                    &[SessionEdit::SetSampleRateHz {
                        sample_rate_hz: rate.0,
                    }],
                )
                .expect("launch final candidate");
            assert_eq!(commit.revision, SessionRevision(revision.0 + 1));
            assert_eq!(store.compiled().normalized_model().sample_rate_hz, rate.0);
        }
    }

    #[test]
    fn failed_edit_and_final_validation_roll_back_wholly() {
        let mut store = store();
        let before = store.canonical_snapshot().to_owned();
        let failed = store.apply_transaction(
            ExpectedRevision::Exact(store.revision()),
            &[route_row("missing", "gain_db", 1.0)],
        );
        assert!(matches!(
            failed,
            Err(SessionStoreError::Edit {
                operation_index: 0,
                ..
            })
        ));
        assert_eq!(store.revision(), SessionRevision(7));
        assert_eq!(store.canonical_snapshot(), before);
        let invalid = store.apply_transaction(
            ExpectedRevision::Exact(store.revision()),
            &[
                SessionEdit::SetSessionId {
                    session_id: id("temporary"),
                },
                SessionEdit::RemoveSource {
                    source_id: id("voice"),
                },
            ],
        );
        assert!(matches!(
            invalid,
            Err(SessionStoreError::Validation {
                operation_index: 2,
                ..
            })
        ));
        assert_eq!(store.revision(), SessionRevision(7));
        assert_eq!(store.canonical_snapshot(), before);
    }

    #[test]
    fn launch_rate_final_validation_is_atomic_and_uses_session_diagnostic() {
        use session::DiagnosticCode;

        for rate in [176_400, 192_000, 352_800, 384_000, 0, 32_000, 192_001] {
            let mut store = store();
            let before_revision = store.revision();
            let before_snapshot = store.canonical_snapshot().to_owned();
            let before_model = store.compiled().normalized_model().clone();
            let edits = [SessionEdit::SetSampleRateHz {
                sample_rate_hz: rate,
            }];
            let error = store
                .apply_transaction(ExpectedRevision::Exact(before_revision), &edits)
                .expect_err("non-launch final candidate rejects");
            match error {
                SessionStoreError::Validation {
                    operation_index,
                    diagnostics,
                } => {
                    assert_eq!(operation_index, edits.len());
                    assert!(diagnostics.diagnostics().iter().any(|diagnostic| {
                        diagnostic.code == DiagnosticCode::SampleRateUnsupportedAtLaunch
                            && diagnostic.path.to_string() == "$.sample_rate_hz"
                            && diagnostic.message
                                == "launch sample_rate_hz must be one of 44100, 48000, 88200, or 96000 Hz"
                    }));
                }
                other => panic!("unexpected transaction result: {other:?}"),
            }
            assert_eq!(store.revision(), before_revision);
            assert_eq!(store.canonical_snapshot(), before_snapshot);
            assert_eq!(store.compiled().normalized_model(), &before_model);
        }
    }

    #[test]
    fn temporary_extended_rate_is_permitted_when_final_candidate_is_launch_rate() {
        let mut store = store();
        let revision = store.revision();
        let edits = [
            SessionEdit::SetSampleRateHz {
                sample_rate_hz: 176_400,
            },
            SessionEdit::SetSampleRateHz {
                sample_rate_hz: 96_000,
            },
        ];
        let commit = store
            .apply_transaction(ExpectedRevision::Exact(revision), &edits)
            .expect("only final candidate is policy checked");
        assert_eq!(commit.applied_operations, edits.len());
        assert_eq!(store.compiled().normalized_model().sample_rate_hz, 96_000);
    }

    #[test]
    fn effects_and_parameters_follow_exact_key_rules() {
        let mut store = store();
        let edit = SessionEdit::set_control(
            controls::ControlAddress {
                owner: Owner::Strip(id("vocal")),
                section: Section::Inserts(id("eq")),
                control: 1,
                lane: Lane::Both,
            },
            1.0,
            None,
        );
        store
            .apply_transaction(
                ExpectedRevision::Exact(store.revision()),
                &[
                    // The fixture's effect id is not a launch effect, so name one the control
                    // core can address.
                    SessionEdit::SetEffectIdentity {
                        track_id: id("vocal"),
                        rack_name: RackName::Inserts,
                        effect_id: id("eq"),
                        identity: EffectIdentity::Native {
                            effect_id: id("miso.parametric-eq"),
                        },
                    },
                    edit.clone(),
                    edit,
                ],
            )
            .expect("same compound key replaces");
        let effect = &store.compiled().normalized_model().tracks[0]
            .inserts
            .effects[0];
        let written: Vec<_> = effect
            .params
            .iter()
            .filter(|param| param.parameter_id == 1 && param.channel == ParameterChannel::Both)
            .collect();
        assert_eq!(written.len(), 1);
        assert_eq!(written[0].value, 1.0);
    }

    fn store_at(revision: u64) -> SessionStore {
        let mut model = parse_session_json(EXAMPLE).expect("fixture");
        model.revision = revision;
        SessionStore::new(model, caps()).expect("revision within the ceiling compiles")
    }

    fn rename(name: &str) -> [SessionEdit; 1] {
        [SessionEdit::SetSessionId {
            session_id: id(name),
        }]
    }

    #[test]
    fn the_last_commit_reaches_the_ceiling_and_the_next_is_exhausted_before_any_work() {
        let mut store = store_at(CEILING - 1);
        let commit = store
            .apply_transaction(
                ExpectedRevision::Exact(SessionRevision(CEILING - 1)),
                &rename("last-commit"),
            )
            .expect("the commit that reaches the ceiling");
        assert_eq!(commit.revision, SessionRevision(CEILING));
        assert_eq!(store.revision(), SessionRevision(CEILING));
        let before = store.canonical_snapshot().to_owned();
        let at_ceiling = ExpectedRevision::Exact(SessionRevision(CEILING));

        assert_eq!(
            store.apply_transaction(ExpectedRevision::Any, &rename("x")),
            Err(SessionStoreError::ExactRevisionRequired)
        );
        assert_eq!(
            store.apply_transaction(at_ceiling, &[]),
            Err(SessionStoreError::EmptyTransaction)
        );
        assert_eq!(
            store.apply_transaction(
                ExpectedRevision::Exact(SessionRevision(CEILING - 1)),
                &rename("x")
            ),
            Err(SessionStoreError::RevisionConflict {
                current: SessionRevision(CEILING)
            })
        );
        // The refusal is the store's own and comes before the candidate exists: an edit that
        // would fail to apply, and one that would fail to compile, both report exhaustion.
        assert_eq!(
            store.apply_transaction(at_ceiling, &rename("never-committed")),
            Err(SessionStoreError::RevisionExhausted)
        );
        assert_eq!(
            store.apply_transaction(
                at_ceiling,
                &[SessionEdit::SetQuantumFrames { quantum_frames: 0 }]
            ),
            Err(SessionStoreError::RevisionExhausted)
        );
        assert_eq!(store.revision(), SessionRevision(CEILING));
        assert_eq!(store.canonical_snapshot(), before);
    }

    #[test]
    fn a_session_above_the_ceiling_is_refused_by_the_one_constructor() {
        let mut model = parse_session_json(EXAMPLE).expect("fixture");
        for revision in [CEILING + 1, u64::MAX] {
            model.revision = revision;
            let error = SessionStore::new(model.clone(), caps())
                .err()
                .expect("above the ceiling");
            let found: Vec<_> = error
                .diagnostics()
                .iter()
                .map(|d| (d.code, d.path.to_string()))
                .collect();
            assert_eq!(
                found,
                vec![(
                    session::DiagnosticCode::NumericOutOfSchemaRange,
                    "$.revision".to_owned()
                )]
            );
        }
    }

    #[test]
    fn opcode_registry_covers_noneditable_revision_and_schema_exclusion() {
        assert_eq!(
            SessionEdit::SetQuantumFrames {
                quantum_frames: 128
            }
            .opcode() as u16,
            3
        );
        assert_eq!(SessionEditOpcode::SetAutomationSegments as u16, 0x0603);
        for deleted in [0x0006, 0x0102, 0x0104] {
            assert_eq!(
                SessionEditOpcode::from_raw(deleted),
                None,
                "deleted #241 opcode 0x{deleted:04x} must remain unallocated"
            );
        }
    }

    /// #1094 appends `SetConsole` (`0x0007`) and `SetTrackConsole` (`0x0211`), #1241 opens the VCA
    /// family with `UpsertVca` and `RemoveVca` (`0x0700`, `0x0701`) and #1553 opens the control
    /// family with `SetControl` (`0x0800`). Decision 17 (C5, #1561) retires the thirteen per-kind
    /// value ops: every allocated code round-trips through `from_raw`, there are exactly 35 of
    /// them, and no retired code came back.
    ///
    /// Red if a retired code is allocated again (a value then has two ops that can drift), if a
    /// `from_raw` arm maps a code to another variant, or if an opcode is added to the enum but not
    /// to `from_raw`.
    #[test]
    fn opcode_registry_appends_the_console_edits_and_reallocates_nothing() {
        let allocated = (0..=u16::MAX)
            .filter_map(|raw| SessionEditOpcode::from_raw(raw).map(|opcode| (raw, opcode)))
            .collect::<Vec<_>>();
        assert_eq!(allocated.len(), 35);
        assert!(allocated.iter().all(|(raw, opcode)| opcode.raw() == *raw));
        assert_eq!(
            SessionEditOpcode::from_raw(0x0007),
            Some(SessionEditOpcode::SetConsole)
        );
        assert_eq!(
            SessionEditOpcode::from_raw(0x0211),
            Some(SessionEditOpcode::SetTrackConsole)
        );
        assert_eq!(
            SessionEditOpcode::from_raw(0x0700),
            Some(SessionEditOpcode::UpsertVca)
        );
        assert_eq!(
            SessionEditOpcode::from_raw(0x0701),
            Some(SessionEditOpcode::RemoveVca)
        );
        assert_eq!(
            SessionEditOpcode::from_raw(0x0800),
            Some(SessionEditOpcode::SetControl)
        );
        // Retired by #241 (0x0006, 0x0102, 0x0104) and by decision 17 (#1561: control smoothing,
        // the builtins, effect bypass, link mode and parameters, fader, pan or matrix, the route
        // value ops and the VCA fader).
        for retired in [
            0x0006, 0x0102, 0x0104, 0x0008, 0x0203, 0x020a, 0x020b, 0x020d, 0x020e, 0x020f, 0x0210,
            0x0504, 0x0505, 0x0506, 0x0507, 0x0702,
        ] {
            assert_eq!(SessionEditOpcode::from_raw(retired), None);
        }
    }
}
