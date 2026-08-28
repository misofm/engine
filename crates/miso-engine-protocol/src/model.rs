//! Typed session edits and the transactional control-plane session store.
//!
//! These edits operate on the accepted issue-004 `SessionToml` model. They neither prepare nor
//! publish a render plan; a successful replacement retains only issue-004's immutable,
//! non-publishable `CompiledSession` control-plane artifact.

use core::fmt;

use miso_engine_session::{
    Automation, AutomationSegment, AutomationTarget, ChannelMatrix, CompileCaps, CompiledSession,
    DualMonoBuiltins, DualMonoFader, Effect, EffectIdentity, EffectParam, EffectQuality,
    MatrixOrPan, Output, OutputProfile, Rack, RackName, RenderProfile, Route, RouteDestination,
    RouteSource, SessionLimits, SessionToml, SidechainDeclaration, Source, SourceContent,
    SourceMapping, StableId, Submix, compile_session,
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
    /// `SetLimits`.
    SetLimits = 0x0006,
    /// `UpsertSource`.
    UpsertSource = 0x0100,
    /// `RemoveSource`.
    RemoveSource = 0x0101,
    /// `SetSourceSampleRateHz`.
    SetSourceSampleRateHz = 0x0102,
    /// `SetSourceContent`.
    SetSourceContent = 0x0103,
    /// `SetSourceMapping`.
    SetSourceMapping = 0x0104,
    /// `UpsertTrack`.
    UpsertTrack = 0x0200,
    /// `RemoveTrack`.
    RemoveTrack = 0x0201,
    /// `SetTrackSourceAssignment`.
    SetTrackSourceAssignment = 0x0202,
    /// `SetTrackBuiltins`.
    SetTrackBuiltins = 0x0203,
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
    /// `SetEffectBypass`.
    SetEffectBypass = 0x020a,
    /// `SetEffectLinkMode`.
    SetEffectLinkMode = 0x020b,
    /// `SetEffectSidechain`.
    SetEffectSidechain = 0x020c,
    /// `UpsertEffectParam`.
    UpsertEffectParam = 0x020d,
    /// `RemoveEffectParam`.
    RemoveEffectParam = 0x020e,
    /// `SetTrackFader`.
    SetTrackFader = 0x020f,
    /// `SetTrackMatrixOrPan`.
    SetTrackMatrixOrPan = 0x0210,
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
    /// `SetRouteChannelMatrix`.
    SetRouteChannelMatrix = 0x0504,
    /// `SetRouteGainDb`.
    SetRouteGainDb = 0x0505,
    /// `UpsertAutomation`.
    UpsertAutomation = 0x0600,
    /// `RemoveAutomation`.
    RemoveAutomation = 0x0601,
    /// `SetAutomationTarget`.
    SetAutomationTarget = 0x0602,
    /// `SetAutomationSegments`.
    SetAutomationSegments = 0x0603,
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
            0x0006 => Some(Self::SetLimits),
            0x0100 => Some(Self::UpsertSource),
            0x0101 => Some(Self::RemoveSource),
            0x0102 => Some(Self::SetSourceSampleRateHz),
            0x0103 => Some(Self::SetSourceContent),
            0x0104 => Some(Self::SetSourceMapping),
            0x0200 => Some(Self::UpsertTrack),
            0x0201 => Some(Self::RemoveTrack),
            0x0202 => Some(Self::SetTrackSourceAssignment),
            0x0203 => Some(Self::SetTrackBuiltins),
            0x0204 => Some(Self::SetTrackRack),
            0x0205 => Some(Self::PutTrackEffect),
            0x0206 => Some(Self::RemoveTrackEffect),
            0x0207 => Some(Self::SetTrackEffectOrder),
            0x0208 => Some(Self::SetEffectIdentity),
            0x0209 => Some(Self::SetEffectQuality),
            0x020a => Some(Self::SetEffectBypass),
            0x020b => Some(Self::SetEffectLinkMode),
            0x020c => Some(Self::SetEffectSidechain),
            0x020d => Some(Self::UpsertEffectParam),
            0x020e => Some(Self::RemoveEffectParam),
            0x020f => Some(Self::SetTrackFader),
            0x0210 => Some(Self::SetTrackMatrixOrPan),
            0x0300 => Some(Self::UpsertSubmix),
            0x0301 => Some(Self::RemoveSubmix),
            0x0400 => Some(Self::UpsertOutput),
            0x0401 => Some(Self::RemoveOutput),
            0x0500 => Some(Self::UpsertRoute),
            0x0501 => Some(Self::RemoveRoute),
            0x0502 => Some(Self::SetRouteSource),
            0x0503 => Some(Self::SetRouteDestination),
            0x0504 => Some(Self::SetRouteChannelMatrix),
            0x0505 => Some(Self::SetRouteGainDb),
            0x0600 => Some(Self::UpsertAutomation),
            0x0601 => Some(Self::RemoveAutomation),
            0x0602 => Some(Self::SetAutomationTarget),
            0x0603 => Some(Self::SetAutomationSegments),
            _ => None,
        }
    }
}

/// One complete typed session mutation. Its payload field ordering is frozen in the issue-005
/// BTLV specification; the later wire-schema codec maps these variants one-to-one.
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
    /// Replace declarative session resource limits only.
    SetLimits { limits: SessionLimits },
    /// Insert or replace one source by stable ID.
    UpsertSource { source: Source },
    /// Remove one source without cascading references.
    RemoveSource { source_id: StableId },
    /// Set a source's declared native rate.
    SetSourceSampleRateHz {
        source_id: StableId,
        sample_rate_hz: u32,
    },
    /// Set a source's opaque content declaration.
    SetSourceContent {
        source_id: StableId,
        content: SourceContent,
    },
    /// Set a source's channel/region mapping.
    SetSourceMapping {
        source_id: StableId,
        mapping: SourceMapping,
    },
    /// Insert or replace one track by stable ID.
    UpsertTrack { track: miso_engine_session::Track },
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
    /// Set both dual-mono builtin declarations.
    SetTrackBuiltins {
        track_id: StableId,
        builtins: DualMonoBuiltins,
    },
    /// Replace a complete named effect rack.
    SetTrackRack {
        track_id: StableId,
        rack_name: RackName,
        rack: Rack,
    },
    /// Insert/reposition one effect after removing a same-ID effect.
    PutTrackEffect {
        /// Existing track.
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
        track_id: StableId,
        rack_name: RackName,
        effect_id: StableId,
    },
    /// Replace effect order with an exact permutation of current IDs.
    SetTrackEffectOrder {
        /// Existing track.
        track_id: StableId,
        /// Rack to reorder.
        rack_name: RackName,
        /// Exact stable-ID permutation.
        effect_ids: Vec<StableId>,
    },
    /// Set a named effect's identity declaration.
    SetEffectIdentity {
        /// Existing track.
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
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New quality token.
        quality: EffectQuality,
    },
    /// Set a named effect's latency-preserving bypass declaration.
    SetEffectBypass {
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New bypass value.
        bypass: bool,
    },
    /// Set a named effect's detector link mode.
    SetEffectLinkMode {
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New link mode.
        link_mode: miso_engine_session::LinkMode,
    },
    /// Set a named effect's sidechain declaration.
    SetEffectSidechain {
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// New sidechain declaration.
        sidechain: SidechainDeclaration,
    },
    /// Insert or replace an effect parameter keyed by `(parameter_id, channel)`.
    UpsertEffectParam {
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// Parameter to insert or replace.
        param: EffectParam,
    },
    /// Remove an existing parameter by its compound key.
    RemoveEffectParam {
        /// Existing track.
        track_id: StableId,
        /// Rack containing the effect.
        rack_name: RackName,
        /// Existing effect.
        effect_id: StableId,
        /// Effect-contract parameter ID.
        parameter_id: u32,
        /// Explicit parameter channel.
        channel: miso_engine_session::ParameterChannel,
    },
    /// Set a track fader declaration.
    SetTrackFader {
        track_id: StableId,
        fader: DualMonoFader,
    },
    /// Set a track matrix or pan declaration.
    SetTrackMatrixOrPan {
        track_id: StableId,
        matrix_or_pan: MatrixOrPan,
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
    /// Set one route's complete explicit 2x2 channel matrix.
    SetRouteChannelMatrix {
        route_id: StableId,
        channel_matrix: ChannelMatrix,
    },
    /// Set one route's gain in decibels.
    SetRouteGainDb { route_id: StableId, gain_db: f32 },
    /// Insert or replace persistent automation by stable ID.
    UpsertAutomation { automation: Automation },
    /// Remove persistent automation by stable ID.
    RemoveAutomation { automation_id: StableId },
    /// Set an existing automation target.
    SetAutomationTarget {
        automation_id: StableId,
        target: AutomationTarget,
    },
    /// Replace an existing automation's complete nonempty ordered segment sequence.
    SetAutomationSegments {
        automation_id: StableId,
        segments: Vec<AutomationSegment>,
    },
}

impl SessionEdit {
    /// Return the stable numeric opcode for this typed edit.
    #[must_use]
    pub const fn opcode(&self) -> SessionEditOpcode {
        match self {
            Self::SetSessionId { .. } => SessionEditOpcode::SetSessionId,
            Self::SetSampleRateHz { .. } => SessionEditOpcode::SetSampleRateHz,
            Self::SetQuantumFrames { .. } => SessionEditOpcode::SetQuantumFrames,
            Self::SetRenderProfile { .. } => SessionEditOpcode::SetRenderProfile,
            Self::SetOutputProfile { .. } => SessionEditOpcode::SetOutputProfile,
            Self::SetLimits { .. } => SessionEditOpcode::SetLimits,
            Self::UpsertSource { .. } => SessionEditOpcode::UpsertSource,
            Self::RemoveSource { .. } => SessionEditOpcode::RemoveSource,
            Self::SetSourceSampleRateHz { .. } => SessionEditOpcode::SetSourceSampleRateHz,
            Self::SetSourceContent { .. } => SessionEditOpcode::SetSourceContent,
            Self::SetSourceMapping { .. } => SessionEditOpcode::SetSourceMapping,
            Self::UpsertTrack { .. } => SessionEditOpcode::UpsertTrack,
            Self::RemoveTrack { .. } => SessionEditOpcode::RemoveTrack,
            Self::SetTrackSourceAssignment { .. } => SessionEditOpcode::SetTrackSourceAssignment,
            Self::SetTrackBuiltins { .. } => SessionEditOpcode::SetTrackBuiltins,
            Self::SetTrackRack { .. } => SessionEditOpcode::SetTrackRack,
            Self::PutTrackEffect { .. } => SessionEditOpcode::PutTrackEffect,
            Self::RemoveTrackEffect { .. } => SessionEditOpcode::RemoveTrackEffect,
            Self::SetTrackEffectOrder { .. } => SessionEditOpcode::SetTrackEffectOrder,
            Self::SetEffectIdentity { .. } => SessionEditOpcode::SetEffectIdentity,
            Self::SetEffectQuality { .. } => SessionEditOpcode::SetEffectQuality,
            Self::SetEffectBypass { .. } => SessionEditOpcode::SetEffectBypass,
            Self::SetEffectLinkMode { .. } => SessionEditOpcode::SetEffectLinkMode,
            Self::SetEffectSidechain { .. } => SessionEditOpcode::SetEffectSidechain,
            Self::UpsertEffectParam { .. } => SessionEditOpcode::UpsertEffectParam,
            Self::RemoveEffectParam { .. } => SessionEditOpcode::RemoveEffectParam,
            Self::SetTrackFader { .. } => SessionEditOpcode::SetTrackFader,
            Self::SetTrackMatrixOrPan { .. } => SessionEditOpcode::SetTrackMatrixOrPan,
            Self::UpsertSubmix { .. } => SessionEditOpcode::UpsertSubmix,
            Self::RemoveSubmix { .. } => SessionEditOpcode::RemoveSubmix,
            Self::UpsertOutput { .. } => SessionEditOpcode::UpsertOutput,
            Self::RemoveOutput { .. } => SessionEditOpcode::RemoveOutput,
            Self::UpsertRoute { .. } => SessionEditOpcode::UpsertRoute,
            Self::RemoveRoute { .. } => SessionEditOpcode::RemoveRoute,
            Self::SetRouteSource { .. } => SessionEditOpcode::SetRouteSource,
            Self::SetRouteDestination { .. } => SessionEditOpcode::SetRouteDestination,
            Self::SetRouteChannelMatrix { .. } => SessionEditOpcode::SetRouteChannelMatrix,
            Self::SetRouteGainDb { .. } => SessionEditOpcode::SetRouteGainDb,
            Self::UpsertAutomation { .. } => SessionEditOpcode::UpsertAutomation,
            Self::RemoveAutomation { .. } => SessionEditOpcode::RemoveAutomation,
            Self::SetAutomationTarget { .. } => SessionEditOpcode::SetAutomationTarget,
            Self::SetAutomationSegments { .. } => SessionEditOpcode::SetAutomationSegments,
        }
    }
}

/// An edit-resolution failure before session compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SessionEditError {
    /// A targeted source, track, effect, route, or automation was absent.
    NotFound,
    /// An effect insertion position exceeded the post-removal rack length.
    InvalidFinalPosition,
    /// A requested effect ordering was not an exact current-ID permutation.
    InvalidEffectOrder,
    /// A complete automation segment replacement was empty.
    EmptyAutomationSegments,
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
    session: &mut SessionToml,
    edit: &SessionEdit,
) -> Result<(), SessionEditError> {
    match edit {
        SessionEdit::SetSessionId { session_id } => session.session_id = session_id.clone(),
        SessionEdit::SetSampleRateHz { sample_rate_hz } => {
            session.sample_rate_hz = *sample_rate_hz
        }
        SessionEdit::SetQuantumFrames { quantum_frames } => {
            session.quantum_frames = *quantum_frames
        }
        SessionEdit::SetRenderProfile { render_profile } => {
            session.render_profile = render_profile.clone()
        }
        SessionEdit::SetOutputProfile { output_profile } => {
            session.output_profile = output_profile.clone()
        }
        SessionEdit::SetLimits { limits } => session.limits = limits.clone(),
        SessionEdit::UpsertSource { source } => {
            upsert(&mut session.sources, source, |item| &item.id)
        }
        SessionEdit::RemoveSource { source_id } => {
            remove(&mut session.sources, source_id, |item| &item.id)?
        }
        SessionEdit::SetSourceSampleRateHz {
            source_id,
            sample_rate_hz,
        } => {
            source_mut(session, source_id)?.sample_rate_hz = *sample_rate_hz;
        }
        SessionEdit::SetSourceContent { source_id, content } => {
            source_mut(session, source_id)?.content = content.clone();
        }
        SessionEdit::SetSourceMapping { source_id, mapping } => {
            source_mut(session, source_id)?.mapping = mapping.clone();
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
        SessionEdit::SetTrackBuiltins { track_id, builtins } => {
            track_mut(session, track_id)?.builtins = builtins.clone();
        }
        SessionEdit::SetTrackRack {
            track_id,
            rack_name,
            rack,
        } => {
            *rack_mut(track_mut(session, track_id)?, *rack_name)? = rack.clone();
        }
        SessionEdit::PutTrackEffect {
            track_id,
            rack_name,
            final_position,
            effect,
        } => {
            let effects = &mut rack_mut(track_mut(session, track_id)?, *rack_name)?.effects;
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
            let effects = &mut rack_mut(track_mut(session, track_id)?, *rack_name)?.effects;
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
            let effects = &mut rack_mut(track_mut(session, track_id)?, *rack_name)?.effects;
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
        SessionEdit::SetEffectBypass {
            track_id,
            rack_name,
            effect_id,
            bypass,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.bypass = *bypass;
        }
        SessionEdit::SetEffectLinkMode {
            track_id,
            rack_name,
            effect_id,
            link_mode,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.link_mode = *link_mode;
        }
        SessionEdit::SetEffectSidechain {
            track_id,
            rack_name,
            effect_id,
            sidechain,
        } => {
            effect_mut(session, track_id, *rack_name, effect_id)?.sidechain = sidechain.clone();
        }
        SessionEdit::UpsertEffectParam {
            track_id,
            rack_name,
            effect_id,
            param,
        } => {
            let params = &mut effect_mut(session, track_id, *rack_name, effect_id)?.params;
            if let Some(index) = params.iter().position(|item| {
                item.parameter_id == param.parameter_id && item.channel == param.channel
            }) {
                params[index] = param.clone();
            } else {
                params.push(param.clone());
            }
        }
        SessionEdit::RemoveEffectParam {
            track_id,
            rack_name,
            effect_id,
            parameter_id,
            channel,
        } => {
            let params = &mut effect_mut(session, track_id, *rack_name, effect_id)?.params;
            let index = params
                .iter()
                .position(|item| item.parameter_id == *parameter_id && item.channel == *channel)
                .ok_or(SessionEditError::NotFound)?;
            params.remove(index);
        }
        SessionEdit::SetTrackFader { track_id, fader } => {
            track_mut(session, track_id)?.fader = fader.clone()
        }
        SessionEdit::SetTrackMatrixOrPan {
            track_id,
            matrix_or_pan,
        } => {
            track_mut(session, track_id)?.matrix_or_pan = matrix_or_pan.clone();
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
        SessionEdit::SetRouteChannelMatrix {
            route_id,
            channel_matrix,
        } => {
            route_mut(session, route_id)?.channel_matrix = channel_matrix.clone();
        }
        SessionEdit::SetRouteGainDb { route_id, gain_db } => {
            route_mut(session, route_id)?.gain_db = *gain_db
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
    compiled: CompiledSession,
    applied_operations: usize,
}

impl PreparedSessionTransaction {
    /// Borrow the prospective immutable compilation without making it authoritative.
    #[must_use]
    pub const fn compiled(&self) -> &CompiledSession {
        &self.compiled
    }

    /// Revision the prospective compilation will own if committed.
    #[must_use]
    pub fn revision(&self) -> SessionRevision {
        SessionRevision(self.compiled.normalized_model().revision)
    }

    /// Number of edits resolved in wire order.
    #[must_use]
    pub const fn applied_operations(&self) -> usize {
        self.applied_operations
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
    /// Incrementing an already maximum model revision is impossible.
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
        diagnostics: miso_engine_session::DiagnosticSet,
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
    pub fn new(
        initial: SessionToml,
        caps: CompileCaps,
    ) -> Result<Self, miso_engine_session::DiagnosticSet> {
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
        self.compiled.canonical_toml()
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

    /// Resolve and compile one transaction without changing the authoritative session triple.
    /// The returned affine value owns every prospective allocation and is safe to discard.
    pub fn prepare_transaction(
        &self,
        expected_revision: ExpectedRevision,
        edits: &[SessionEdit],
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
        // The session schema bounds every u64 field to `i64::MAX`, so that value -- not
        // `u64::MAX` -- is the maximum revision a compiled session can hold.
        let next_revision = current
            .0
            .checked_add(1)
            .filter(|revision| *revision <= i64::MAX as u64)
            .ok_or(SessionStoreError::RevisionExhausted)?;
        let mut candidate = self.compiled.normalized_model().clone();
        for (operation_index, edit) in edits.iter().enumerate() {
            apply_session_edit(&mut candidate, edit).map_err(|error| SessionStoreError::Edit {
                operation_index,
                error,
            })?;
        }
        candidate.revision = next_revision;
        let compiled = compile_session(&candidate, self.caps).map_err(|diagnostics| {
            SessionStoreError::Validation {
                operation_index: edits.len(),
                diagnostics,
            }
        })?;
        Ok(PreparedSessionTransaction {
            base_revision: current,
            compiled,
            applied_operations: edits.len(),
        })
    }

    /// Install a preparation whose base revision was checked by the controller immediately
    /// before this call.  Moving the complete compilation is allocation-free and non-fallible.
    pub(crate) fn commit_prepared(
        &mut self,
        prepared: PreparedSessionTransaction,
    ) -> SessionCommit {
        debug_assert_eq!(self.revision(), prepared.base_revision);
        let revision = prepared.revision();
        let applied_operations = prepared.applied_operations;
        self.compiled = prepared.compiled;
        SessionCommit {
            revision,
            applied_operations,
        }
    }
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
    session: &'a mut SessionToml,
    id: &StableId,
) -> Result<&'a mut Source, SessionEditError> {
    session
        .sources
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

fn track_mut<'a>(
    session: &'a mut SessionToml,
    id: &StableId,
) -> Result<&'a mut miso_engine_session::Track, SessionEditError> {
    session
        .tracks
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

fn route_mut<'a>(
    session: &'a mut SessionToml,
    id: &StableId,
) -> Result<&'a mut Route, SessionEditError> {
    session
        .routes
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

fn automation_mut<'a>(
    session: &'a mut SessionToml,
    id: &StableId,
) -> Result<&'a mut Automation, SessionEditError> {
    session
        .automation
        .iter_mut()
        .find(|item| &item.id == id)
        .ok_or(SessionEditError::NotFound)
}

/// The `Rack` a `SessionEdit` rack-addressed edit names.
///
/// `RackName::Builtins` (#178, ruled by #210's D2) is not one: it is the strip's own builtin
/// section, which is a `DualMonoBuiltins` and holds no `effects` vector at all, so every edit that
/// reaches here -- `SetTrackRack`, `PutTrackEffect`, `RemoveTrackEffect`, `SetEffectQuality` and
/// their siblings -- is addressing something that does not exist. It is refused with
/// [`SessionEditError::NotFound`], the same answer a named-but-absent effect gets, rather than
/// given a panicking arm or a silent no-op.
///
/// The strip **is** editable, through `SetTrackBuiltins` (`:516`), which is the edit that owns it
/// and is unaffected by the new token. The token exists for the automation-target vocabulary and
/// for nothing else.
fn rack_mut(
    track: &mut miso_engine_session::Track,
    rack_name: RackName,
) -> Result<&mut Rack, SessionEditError> {
    match rack_name {
        RackName::Simd1 => Ok(&mut track.simd1),
        RackName::Dynamic => Ok(&mut track.dynamic),
        RackName::Simd2 => Ok(&mut track.simd2),
        RackName::Builtins => Err(SessionEditError::NotFound),
    }
}

fn effect_mut<'a>(
    session: &'a mut SessionToml,
    track_id: &StableId,
    rack_name: RackName,
    effect_id: &StableId,
) -> Result<&'a mut Effect, SessionEditError> {
    rack_mut(track_mut(session, track_id)?, rack_name)?
        .effects
        .iter_mut()
        .find(|effect| &effect.id == effect_id)
        .ok_or(SessionEditError::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use miso_engine_session::{
        ParameterChannel, ParameterUnit, canonical_session_toml, parse_session_toml,
    };

    const EXAMPLE: &str = include_str!("../../../fixtures/session/v1/canonical.toml");

    /// Largest revision a session may declare: the schema bounds u64 fields to `i64::MAX`.
    const MAX_SESSION_REVISION: u64 = i64::MAX as u64;

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
        SessionStore::new(parse_session_toml(EXAMPLE).expect("fixture"), caps()).expect("compile")
    }

    fn id(value: &str) -> StableId {
        StableId::parse(value).expect("test stable ID")
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
            canonical_session_toml(store.compiled().normalized_model()).expect("canonical")
        );
    }

    #[test]
    fn every_launch_rate_initializes_store_and_commits_as_the_final_candidate() {
        for rate in miso_engine_core::LAUNCH_SAMPLE_RATES {
            let mut initial = parse_session_toml(EXAMPLE).expect("fixture");
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
            &[SessionEdit::SetRouteGainDb {
                route_id: id("missing"),
                gain_db: 1.0,
            }],
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
        use miso_engine_session::DiagnosticCode;

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
        let edit = SessionEdit::UpsertEffectParam {
            track_id: id("vocal"),
            rack_name: RackName::Dynamic,
            effect_id: id("eq"),
            param: EffectParam {
                parameter_id: 1,
                channel: ParameterChannel::Left,
                unit: ParameterUnit::Db,
                value: 2.0,
            },
        };
        store
            .apply_transaction(
                ExpectedRevision::Exact(store.revision()),
                &[edit.clone(), edit],
            )
            .expect("same compound key replaces");
        let effect = &store.compiled().normalized_model().tracks[0]
            .dynamic
            .effects[0];
        assert_eq!(
            effect
                .params
                .iter()
                .filter(|param| {
                    param.parameter_id == 1 && param.channel == ParameterChannel::Left
                })
                .count(),
            1
        );
    }

    #[test]
    fn any_and_maximum_revision_are_rejected_without_replacement() {
        let mut model = parse_session_toml(EXAMPLE).expect("fixture");
        model.revision = MAX_SESSION_REVISION;
        let mut store = SessionStore::new(model, caps()).expect("max revision compiles");
        let before = store.canonical_snapshot().to_owned();
        assert_eq!(
            store.apply_transaction(ExpectedRevision::Any, &[]),
            Err(SessionStoreError::ExactRevisionRequired)
        );
        assert_eq!(
            store.apply_transaction(
                ExpectedRevision::Exact(SessionRevision(MAX_SESSION_REVISION)),
                &[]
            ),
            Err(SessionStoreError::EmptyTransaction)
        );
        assert_eq!(
            store.apply_transaction(
                ExpectedRevision::Exact(SessionRevision(MAX_SESSION_REVISION)),
                &[SessionEdit::SetSessionId {
                    session_id: id("never-committed"),
                }],
            ),
            Err(SessionStoreError::RevisionExhausted)
        );
        assert_eq!(store.canonical_snapshot(), before);
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
    }
}
