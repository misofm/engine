//! Explicit, stopped response previews owned by the native effect providers.
//!
//! This module is deliberately a small bridge.  It does not parse a session, inspect a graph or
//! retain a render plan: a caller supplies one native effect configuration (or one pair of input
//! filters), the owner prepares it through its existing response factory, and a query writes into
//! caller-owned buffers.  The browser adapter uses the same entry point, so native and Wasm
//! previews cannot grow separate numerical implementations.

use builtins::{BuiltinParameters, prepare_input_filter_response};
use effect_compiler::launch_native_effect_registry;
use effect_contract::{
    EffectQuality, InitialParameterValue, LinkMode, ParameterChannel, PreparedPorts,
    PreparedResponseAnalysis, ResponseAnalysisError, ResponseOutput, ResponsePrepareLimits,
    ResponseQuery as OwnerResponseQuery, ResponseSummary, default_initial_values,
};
use engine::realtime::{
    ResponseSnapshotAvailability, ResponseSnapshotCapture, ResponseSnapshotError,
    ResponseSnapshotOwnerInfo, ResponseSnapshotSection, ResponseSnapshotSink,
};
use math::{exp, log};
use session::{EffectIdentity, SessionModel, StableId};

/// The explicit target supplied by a stopped preview caller.
#[derive(Clone, Copy, Debug)]
pub enum ResponsePreviewTarget<'a> {
    /// One native effect owner, identified by its generated effect ID.
    Effect {
        /// Generated native effect identity.
        effect_id: &'a str,
        /// Numeric parameter/channel overrides over descriptor defaults.
        overrides: &'a [ResponseParameterOverride],
        /// Native preparation quality.
        quality: EffectQuality,
        /// Effect-wide bypass.
        bypass: bool,
        /// Native channel-link mode.
        link_mode: LinkMode,
    },
    /// One native effect owner with every value resolved (#1575 D4): a session effect's
    /// committed values, overrides applied, from [`resolve_response_target`].
    ResolvedEffect {
        /// Generated native effect identity.
        effect_id: &'a str,
        /// Exactly one value per descriptor parameter and lane, in [`default_initial_values`]
        /// order ([`ResponsePreviewError::InvalidParameter`] otherwise).
        values: &'a [InitialParameterValue],
        /// Native preparation quality.
        quality: EffectQuality,
        /// Effect-wide bypass.
        bypass: bool,
        /// Native channel-link mode.
        link_mode: LinkMode,
    },
    /// The builtin input HPF/LPF pair. Other builtin sections remain at their defaults.
    InputFilters {
        /// Left high-pass cutoff, in Hz.
        left_hpf_hz: f32,
        /// Left low-pass cutoff, in Hz.
        left_lpf_hz: f32,
        /// Right high-pass cutoff, in Hz.
        right_hpf_hz: f32,
        /// Right low-pass cutoff, in Hz.
        right_lpf_hz: f32,
    },
}

/// One numeric override in descriptor parameter ID/channel space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResponseParameterOverride {
    /// Native descriptor parameter ID.
    pub parameter_id: u32,
    /// Native parameter channel.
    pub channel: ParameterChannel,
    /// The descriptor-domain value, before owner validation.
    pub value: f32,
}

/// The section of a strip that a response address names (#1575 D1): the input filters of the
/// builtins section, one console slot's entry or one insert. Any other owner or section names no
/// response owner; a host refuses it as [`ResponseQueryError::Address`] before it resolves.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseAddressSection<'a> {
    /// The strip's builtins section: its input high-pass and low-pass filters.
    Builtins,
    /// The strip's entry of the console slot with this ID.
    Console(&'a str),
    /// The strip's insert with this ID.
    Inserts(&'a str),
}

/// A response target resolved from the committed model (#1575 D2): the values a plan rebuild
/// prepares for the addressed owner, with the request's overrides applied. It owns what it holds;
/// [`Self::target`] borrows it as the target a query prepares, and nothing outlives the request.
#[derive(Clone, Debug, PartialEq)]
pub enum ResolvedResponseTarget {
    /// One native effect: a console slot's entry or an insert.
    Effect {
        /// Generated native effect identity.
        effect_id: String,
        /// One value per descriptor parameter and lane, in [`default_initial_values`] order.
        values: Vec<InitialParameterValue>,
        /// Native preparation quality.
        quality: EffectQuality,
        /// Effect-wide bypass.
        bypass: bool,
        /// Native channel-link mode.
        link_mode: LinkMode,
    },
    /// The strip's input filters: its four cutoffs.
    InputFilters {
        /// Left high-pass cutoff, in Hz.
        left_hpf_hz: f32,
        /// Left low-pass cutoff, in Hz.
        left_lpf_hz: f32,
        /// Right high-pass cutoff, in Hz.
        right_hpf_hz: f32,
        /// Right low-pass cutoff, in Hz.
        right_lpf_hz: f32,
    },
}

impl ResolvedResponseTarget {
    /// The target a response query prepares for this resolution.
    #[must_use]
    pub fn target(&self) -> ResponsePreviewTarget<'_> {
        match self {
            Self::Effect {
                effect_id,
                values,
                quality,
                bypass,
                link_mode,
            } => ResponsePreviewTarget::ResolvedEffect {
                effect_id,
                values,
                quality: *quality,
                bypass: *bypass,
                link_mode: *link_mode,
            },
            Self::InputFilters {
                left_hpf_hz,
                left_lpf_hz,
                right_hpf_hz,
                right_lpf_hz,
            } => ResponsePreviewTarget::InputFilters {
                left_hpf_hz: *left_hpf_hz,
                left_lpf_hz: *left_lpf_hz,
                right_hpf_hz: *right_hpf_hz,
                right_lpf_hz: *right_lpf_hz,
            },
        }
    }
}

/// The builtin row IDs of the input filters (`BUILTIN_PARAMETER_DESCRIPTORS`): `hpf_hz` and
/// `lpf_hz`, the only builtin controls a response override may name.
const INPUT_FILTER_CONTROLS: [u32; 2] = [3, 4];

/// Resolve the response owner a control address names against the committed model (#1575 D1-D3).
///
/// `strip` is a strip's stable ID (a track's or a submix's) and `section` the part of it. Each
/// override names a control of that section by its row ID (`parameter_id`): `0` is an effect's
/// bypass, [`controls::LINK_MODE_CONTROL`] its link-mode row, any other ID a parameter, and in the
/// builtins section only `hpf_hz` (3) and `lpf_hz` (4). Every override is checked as a control
/// write before anything is resolved or prepared: the row must be one of the rows the control list
/// gives for that section (`control.address.unknown` otherwise), and the one validator
/// [`controls::check`] must accept the value on that lane (its own code otherwise). A (control,
/// lane) pair given twice, or `both` beside a lane, is `response.override.duplicate`. Nothing is
/// written.
///
/// The values are the committed model's: a builtins section reads its four cutoffs with
/// [`controls::get`]; a console slot or an insert resolves its entry's parameters with
/// [`effect_compiler::resolve_initial_values`], the function a rebuild prepares them with, and takes
/// its quality and link mode from the slot or the insert and its bypass from the entry. The
/// overrides then replace the values they name.
///
/// # Errors
///
/// [`ResponseQueryError::AddressUnknown`] for an unknown strip, slot or insert;
/// [`ResponseQueryError::ThirdParty`] for a `cid` effect; [`ResponseQueryError::OverrideControl`]
/// for an override the control validator refuses; [`ResponsePreviewError::InvalidParameter`] for a
/// builtins override of another control than the two cutoffs and
/// [`ResponsePreviewError::DuplicateParameter`] for a duplicate (both as
/// [`ResponseQueryError::Preview`]); [`ResponsePreviewError::UnknownEffect`] for a native effect
/// the registry does not hold; [`ResponseQueryError::Committed`] if the committed values do not
/// resolve.
pub fn resolve_response_target(
    model: &SessionModel,
    strip: &str,
    section: ResponseAddressSection<'_>,
    overrides: &[ResponseParameterOverride],
    sample_rate_hz: u32,
) -> Result<ResolvedResponseTarget, ResponseQueryError> {
    let unknown = ResponseQueryError::AddressUnknown;
    let strip_id = StableId::parse(strip).ok_or(unknown)?;
    let strip_index = controls::strip_index(model, &strip_id).ok_or(unknown)?;
    let strip_ref = model.strips().nth(strip_index).ok_or(unknown)?;
    // The effect the section names, as the plan lowering gives it: a console entry with its slot's
    // identity, quality and link mode, or an insert.
    let lowered = matches!(section, ResponseAddressSection::Console(_))
        .then(|| model.lower_strip(&strip_ref));
    let effect = match section {
        ResponseAddressSection::Builtins => None,
        ResponseAddressSection::Console(slot) => Some(
            lowered
                .iter()
                .flat_map(|racks| racks.pre_insert.iter().chain(&racks.post_insert))
                .find(|candidate| candidate.id.as_str() == slot)
                .ok_or(unknown)?,
        ),
        ResponseAddressSection::Inserts(insert) => Some(
            strip_ref
                .inserts
                .effects
                .iter()
                .find(|candidate| candidate.id.as_str() == insert)
                .ok_or(unknown)?,
        ),
    }
    .map(|effect| {
        (
            &effect.id,
            &effect.identity,
            effect.quality,
            effect.link_mode,
            effect.bypass,
            &effect.params[..],
        )
    });
    let effect_id = match effect {
        Some((_, EffectIdentity::ThirdPartyCid { .. }, ..)) => {
            return Err(ResponseQueryError::ThirdParty);
        }
        Some((_, EffectIdentity::Native { effect_id }, ..)) => Some(effect_id.as_str()),
        None => None,
    };
    let owner = controls::Owner::Strip(strip_id);
    let address_section = match effect {
        None => controls::Section::Builtins,
        Some((id, ..)) => match section {
            ResponseAddressSection::Console(_) => controls::Section::Console(id.clone()),
            _ => controls::Section::Inserts(id.clone()),
        },
    };
    check_response_overrides(
        model,
        strip_index,
        &address_section,
        effect.is_none(),
        overrides,
        sample_rate_hz,
    )?;
    let (Some(effect_id), Some((_, _, quality, link_mode, bypass, params))) = (effect_id, effect)
    else {
        // The builtins section: the committed cutoffs, then the overrides by the lane rule.
        let committed = |control: u32, lane: session::ParameterChannel| {
            controls::get(
                model,
                &controls::MonitorOverlay::default(),
                &controls::ControlAddress {
                    owner: owner.clone(),
                    section: controls::Section::Builtins,
                    control,
                    lane,
                },
                sample_rate_hz,
            )
            .map_err(ResponseQueryError::OverrideControl)
        };
        let (hpf, lpf) = (INPUT_FILTER_CONTROLS[0], INPUT_FILTER_CONTROLS[1]);
        let left = session::ParameterChannel::Left;
        let right = session::ParameterChannel::Right;
        // [left hpf, left lpf, right hpf, right lpf]
        let mut cutoffs = [
            committed(hpf, left)?,
            committed(lpf, left)?,
            committed(hpf, right)?,
            committed(lpf, right)?,
        ];
        for row in overrides {
            let offset = usize::from(row.parameter_id == lpf);
            if matches!(row.channel, ParameterChannel::Left | ParameterChannel::Both) {
                cutoffs[offset] = row.value;
            }
            if matches!(
                row.channel,
                ParameterChannel::Right | ParameterChannel::Both
            ) {
                cutoffs[2 + offset] = row.value;
            }
        }
        return Ok(ResolvedResponseTarget::InputFilters {
            left_hpf_hz: cutoffs[0],
            left_lpf_hz: cutoffs[1],
            right_hpf_hz: cutoffs[2],
            right_lpf_hz: cutoffs[3],
        });
    };
    let registry = launch_native_effect_registry()
        .map_err(|_| ResponseQueryError::Preview(ResponsePreviewError::UnknownEffect))?;
    let descriptor = registry
        .get_ascii(effect_id)
        .ok_or(ResponseQueryError::Preview(
            ResponsePreviewError::UnknownEffect,
        ))?
        .descriptor();
    let mut values = effect_compiler::resolve_initial_values(descriptor, params)
        .map_err(ResponseQueryError::Committed)?;
    let mut bypass = bypass;
    let mut link_mode = contract_link_mode(link_mode);
    for row in overrides {
        // The validator has accepted every value: the bypass is exactly 0 or 1 and the link mode
        // one of the effect's codes.
        match row.parameter_id {
            controls::BYPASS_CONTROL => bypass = row.value.to_bits() == 1.0_f32.to_bits(),
            controls::LINK_MODE_CONTROL => {
                link_mode = [LinkMode::DualMono, LinkMode::Maximum, LinkMode::Average]
                    .into_iter()
                    .find(|mode| *mode as u32 as f32 == row.value)
                    .ok_or(ResponseQueryError::OverrideControl(
                        controls::ControlError::Domain,
                    ))?;
            }
            // A duplicate was refused by `check_response_overrides`, which saw every override;
            // `apply_overrides` sees one row at a time and cannot.
            _ => apply_overrides(
                descriptor,
                &mut values,
                core::slice::from_ref(&ResponseParameterOverride {
                    value: effect_contract::normalize_zero(row.value),
                    ..*row
                }),
            )
            .map_err(ResponseQueryError::Preview)?,
        }
    }
    Ok(ResolvedResponseTarget::Effect {
        effect_id: effect_id.to_owned(),
        values,
        quality: contract_quality(quality),
        bypass,
        link_mode,
    })
}

/// #1575 D3: every override is a control write the validator accepts, on a row of the addressed
/// section, given once; checked before anything is resolved or prepared, and writing nothing.
fn check_response_overrides(
    model: &SessionModel,
    strip_index: usize,
    section: &controls::Section,
    builtins: bool,
    overrides: &[ResponseParameterOverride],
    sample_rate_hz: u32,
) -> Result<(), ResponseQueryError> {
    if overrides.is_empty() {
        return Ok(());
    }
    // The rows the control list gives for this strip (its owner index is its strip index), read
    // for this one owner alone.
    let overlay = controls::MonitorOverlay::default();
    let view = controls::ModelView::whole(model);
    let effective = controls::LocalEffective::new(view, &overlay);
    let mut rows = Vec::new();
    controls::owner_rows_in(
        view,
        &effective,
        &overlay,
        strip_index,
        sample_rate_hz,
        &mut rows,
    );
    for (index, row) in overrides.iter().enumerate() {
        if builtins && !INPUT_FILTER_CONTROLS.contains(&row.parameter_id) {
            return Err(ResponseQueryError::Preview(
                ResponsePreviewError::InvalidParameter,
            ));
        }
        let listed = rows
            .iter()
            .find(|listed| {
                listed.address.section == *section && listed.address.control == row.parameter_id
            })
            .ok_or(ResponseQueryError::OverrideControl(
                controls::ControlError::AddressUnknown,
            ))?;
        controls::check(
            &listed.descriptor,
            session_lane(row.channel),
            row.value,
            None,
        )
        .map_err(ResponseQueryError::OverrideControl)?;
        if overrides[..index].iter().any(|previous| {
            previous.parameter_id == row.parameter_id
                && lanes_overlap(previous.channel, row.channel)
        }) {
            return Err(ResponseQueryError::Preview(
                ResponsePreviewError::DuplicateParameter,
            ));
        }
    }
    Ok(())
}

const fn session_lane(channel: ParameterChannel) -> session::ParameterChannel {
    match channel {
        ParameterChannel::Left => session::ParameterChannel::Left,
        ParameterChannel::Right => session::ParameterChannel::Right,
        ParameterChannel::Both => session::ParameterChannel::Both,
    }
}

/// The native quality of a session quality.
pub(crate) const fn contract_quality(quality: session::EffectQuality) -> EffectQuality {
    match quality {
        session::EffectQuality::Draft => EffectQuality::Draft,
        session::EffectQuality::Normal => EffectQuality::Normal,
        session::EffectQuality::High => EffectQuality::High,
    }
}

/// The native link mode of a session link mode.
pub(crate) const fn contract_link_mode(link_mode: session::LinkMode) -> LinkMode {
    match link_mode {
        session::LinkMode::DualMono => LinkMode::DualMono,
        session::LinkMode::Maximum => LinkMode::Maximum,
        session::LinkMode::Average => LinkMode::Average,
    }
}

/// Bounded preparation resources for one response preview.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponsePreviewLimits {
    /// Maximum concrete owner payload retained by preparation.
    pub maximum_prepared_bytes: usize,
    /// Maximum ordinary native effect state admitted while validating the request.
    pub maximum_total_state_bytes: u64,
    /// Maximum ordinary native effect scratch admitted while validating the request.
    pub maximum_scratch_bytes: u64,
    /// Maximum ordinary automation span capacity admitted while validating the request.
    pub maximum_automation_spans_per_block: u32,
}

impl Default for ResponsePreviewLimits {
    fn default() -> Self {
        Self {
            maximum_prepared_bytes: 1 << 20,
            maximum_total_state_bytes: 1 << 30,
            maximum_scratch_bytes: 1 << 30,
            maximum_automation_spans_per_block: 4096,
        }
    }
}

/// A generated response frequency axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ResponsePreviewGrid {
    /// Evenly spaced inclusive endpoints.
    Linear {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
    /// Logarithmically spaced inclusive positive endpoints.
    Logarithmic {
        /// Number of points, at least two.
        points: usize,
        /// Inclusive positive lower endpoint in Hz.
        minimum_hz: f32,
        /// Inclusive upper endpoint in Hz.
        maximum_hz: f32,
    },
}

impl ResponsePreviewGrid {
    /// Number of requested points.
    #[must_use]
    pub const fn points(self) -> usize {
        match self {
            Self::Linear { points, .. } | Self::Logarithmic { points, .. } => points,
        }
    }

    fn endpoints(self) -> (f32, f32) {
        match self {
            Self::Linear {
                minimum_hz,
                maximum_hz,
                ..
            }
            | Self::Logarithmic {
                minimum_hz,
                maximum_hz,
                ..
            } => (minimum_hz, maximum_hz),
        }
    }
}

/// A stopped response preparation request.
#[derive(Clone, Copy, Debug)]
pub struct ResponsePreviewRequest<'a> {
    /// Opaque caller correlation identity.
    pub configuration_id: u64,
    /// Explicit launch sample rate.
    pub sample_rate_hz: u32,
    /// Explicit render quantum used by native effect validation.
    pub quantum_frames: u32,
    /// One owner-local target.
    pub target: ResponsePreviewTarget<'a>,
    /// Preparation/resource caps.
    pub limits: ResponsePreviewLimits,
}

/// Caller-owned output buffers for one response query.
#[derive(Debug)]
pub struct ResponsePreviewOutput<'a> {
    /// Engine-generated frequency axis.
    pub frequencies_hz: &'a mut [f32],
    /// Left total curve.
    pub total_left_db: &'a mut [f32],
    /// Right total curve.
    pub total_right_db: &'a mut [f32],
    /// Optional section-major left curves.
    pub sections_left_db: Option<&'a mut [f32]>,
    /// Optional section-major right curves.
    pub sections_right_db: Option<&'a mut [f32]>,
}

/// Why an explicit preview request was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponsePreviewError {
    /// The requested launch rate/quantum or grid shape is invalid.
    InvalidShape,
    /// The requested native effect ID is absent.
    UnknownEffect,
    /// The effect exists but has no response provider.
    UnsupportedEffect,
    /// A parameter ID/channel row is not valid for this descriptor.
    InvalidParameter,
    /// A parameter ID/channel row was supplied more than once.
    DuplicateParameter,
    /// A shared parameter was addressed as a lane or a lane parameter as both.
    ConflictingChannel,
    /// The owner rejected the explicit preparation.
    Owner(ResponseAnalysisError),
    /// The generated axis is not finite, increasing and inside Nyquist.
    InvalidGrid,
    /// Caller buffers do not exactly match the requested shape.
    OutputShape,
}

/// One owner copied from an exclusive live-plan boundary.
#[derive(Clone, Debug)]
pub struct ResponseSnapshotOwner {
    /// Stable track identity repeated for convenient worker-side validation.
    pub track_id: Box<str>,
    /// Authoritative native provider identity, separate from the session stable identity.
    pub native_id: Box<str>,
    /// Stable native owner identity in declared signal order.
    pub stable_id: Box<str>,
    /// Native rack/stage identity.
    pub rack: u8,
    /// Declared signal-chain slot, independent of bank execution order.
    pub slot: u32,
    /// Owner response kind (`1` EQ, `2` input filters).
    pub kind: u32,
    /// Bypass state captured at the same boundary as the words.
    pub bypassed: bool,
    /// Whether the owner supplied a linear response provider.
    pub availability: ResponseSnapshotAvailability,
    /// Rounded left target sections.
    pub left: Box<[ResponseSnapshotSection]>,
    /// Rounded right target sections.
    pub right: Box<[ResponseSnapshotSection]>,
}

/// Immutable, host-owned copy of one selected track's response owners.
#[derive(Clone, Debug)]
pub struct ResponseSnapshot {
    /// Validated launch sample rate used by every owner.
    pub sample_rate_hz: u32,
    /// Actual next absolute sample at the completed render boundary.
    pub captured_sample: u64,
    /// Owners in declared signal order, including explicit unavailable exclusions.
    pub owners: Box<[ResponseSnapshotOwner]>,
}

/// Preallocated collector for an exclusive engine snapshot callback.
///
/// Construct this storage off render, then capture without allocations on the plan-owning thread.
/// Finish the owned worker representation off render after the capture has completed.
pub struct ResponseSnapshotCollector {
    sample_rate_hz: u32,
    maximum_sections_per_owner: usize,
    next_owner: usize,
    owners: Vec<CollectorOwner>,
}

const COLLECTOR_SECTION_CAPACITY: usize = effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS;

struct CollectorOwner {
    track_id: String,
    native_id: String,
    stable_id: String,
    rack: u8,
    slot: u32,
    kind: u32,
    bypassed: bool,
    availability: ResponseSnapshotAvailability,
    left: [ResponseSnapshotSection; COLLECTOR_SECTION_CAPACITY],
    right: [ResponseSnapshotSection; COLLECTOR_SECTION_CAPACITY],
    left_len: usize,
    right_len: usize,
}

impl ResponseSnapshotCollector {
    /// Allocate bounded worker-side storage for one capture.
    #[must_use]
    pub fn new(
        sample_rate_hz: u32,
        maximum_owners: usize,
        maximum_sections_per_owner: usize,
        maximum_identity_bytes: usize,
    ) -> Self {
        let empty = ResponseSnapshotSection {
            id: 0,
            kind: 0,
            enabled: false,
            word_count: 0,
            words: [0; effect_contract::RESPONSE_SNAPSHOT_WORDS],
        };
        let owners = (0..maximum_owners)
            .map(|_| CollectorOwner {
                track_id: String::with_capacity(maximum_identity_bytes),
                native_id: String::with_capacity(maximum_identity_bytes),
                stable_id: String::with_capacity(maximum_identity_bytes),
                rack: 0,
                slot: 0,
                kind: 0,
                bypassed: false,
                availability: ResponseSnapshotAvailability::DeclaredUnavailable,
                left: [empty; COLLECTOR_SECTION_CAPACITY],
                right: [empty; COLLECTOR_SECTION_CAPACITY],
                left_len: 0,
                right_len: 0,
            })
            .collect();
        Self {
            sample_rate_hz,
            maximum_sections_per_owner: maximum_sections_per_owner.min(COLLECTOR_SECTION_CAPACITY),
            next_owner: 0,
            owners,
        }
    }

    /// Finish the collector off render after the engine owner reports a successful capture.
    #[must_use]
    pub fn finish(self, capture: ResponseSnapshotCapture) -> ResponseSnapshot {
        let owner_count = self.next_owner;
        let owners = self
            .owners
            .into_iter()
            .take(owner_count)
            .map(|owner| ResponseSnapshotOwner {
                track_id: owner.track_id.into_boxed_str(),
                native_id: owner.native_id.into_boxed_str(),
                stable_id: owner.stable_id.into_boxed_str(),
                rack: owner.rack,
                slot: owner.slot,
                kind: owner.kind,
                bypassed: owner.bypassed,
                availability: owner.availability,
                left: owner.left[..owner.left_len].to_vec().into_boxed_slice(),
                right: owner.right[..owner.right_len].to_vec().into_boxed_slice(),
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        ResponseSnapshot {
            sample_rate_hz: self.sample_rate_hz,
            captured_sample: capture.captured_sample,
            owners,
        }
    }
}

impl ResponseSnapshotSink for ResponseSnapshotCollector {
    fn copy_owner(
        &mut self,
        owner: ResponseSnapshotOwnerInfo<'_>,
        left: &[ResponseSnapshotSection],
        right: &[ResponseSnapshotSection],
    ) -> Result<(), ResponseSnapshotError> {
        if self.next_owner >= self.owners.len()
            || left.len() > self.maximum_sections_per_owner
            || right.len() > self.maximum_sections_per_owner
            || left.len() > COLLECTOR_SECTION_CAPACITY
            || right.len() > COLLECTOR_SECTION_CAPACITY
            || owner.track_id.len() > self.owners[self.next_owner].track_id.capacity()
            || owner.native_id.len() > self.owners[self.next_owner].native_id.capacity()
            || owner.stable_id.len() > self.owners[self.next_owner].stable_id.capacity()
        {
            return Err(ResponseSnapshotError::Capacity);
        }
        let destination = &mut self.owners[self.next_owner];
        destination.track_id.clear();
        destination.track_id.push_str(owner.track_id);
        destination.native_id.clear();
        destination.native_id.push_str(owner.native_id);
        destination.stable_id.clear();
        destination.stable_id.push_str(owner.stable_id);
        destination.rack = owner.rack;
        destination.slot = owner.slot;
        destination.kind = owner.kind;
        destination.bypassed = owner.bypassed;
        destination.availability = owner.availability;
        destination.left[..left.len()].copy_from_slice(left);
        destination.right[..right.len()].copy_from_slice(right);
        destination.left_len = left.len();
        destination.right_len = right.len();
        self.next_owner += 1;
        Ok(())
    }
}

/// Caller-owned output for a composed live track subtotal.
#[derive(Debug)]
pub struct ResponseSnapshotOutput<'a> {
    /// Engine-generated frequency axis.
    pub frequencies_hz: &'a mut [f32],
    /// Unfloored-composed left subtotal, published once at the end.
    pub total_left_db: &'a mut [f32],
    /// Unfloored-composed right subtotal, published once at the end.
    pub total_right_db: &'a mut [f32],
}

/// Why a copied live snapshot could not be composed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotQueryError {
    /// The generated axis or sample rate is invalid.
    InvalidGrid,
    /// A point/owner budget or checked scratch allocation was exceeded.
    Capacity,
    /// Caller buffers or owner section shapes do not match their contract.
    OutputShape,
    /// The snapshot has no applicable linear provider after ordered exclusions.
    UnsupportedProvider,
    /// A provider's copied words are malformed.
    MalformedSnapshot,
    /// A provider or composed magnitude was non-finite/negative.
    Numerical,
}

/// Metadata returned by a successful composed target query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResponseSnapshotQuerySummary {
    /// The immutable boundary sample associated with the copied target words.
    pub captured_sample: u64,
    /// Validated sample rate shared by every owner.
    pub sample_rate_hz: u32,
    /// Number of owners copied, including unavailable exclusions.
    pub owners: u32,
    /// Number of declared owners without a linear provider.
    pub excluded_owners: u32,
    /// The frozen live-query mode.
    pub mode: ResponseSnapshotMode,
}

/// Semantic mode for a live target snapshot query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseSnapshotMode {
    /// The response of the target words held at the capture boundary.
    Target,
}

impl crate::prepare::PreparedHost {
    /// Copy one selected track's prepared response owners at the current exclusive boundary.
    ///
    /// The host must call this between render calls. The engine callback borrows prepared words
    /// only for the duration of each sink call; a caller that needs allocation-free capture can
    /// provide fixed storage through [`ResponseSnapshotSink`].
    pub fn copy_response_snapshot(
        &mut self,
        track_id: &str,
        sink: &mut dyn ResponseSnapshotSink,
    ) -> Result<ResponseSnapshotCapture, ResponseSnapshotError> {
        self.plan
            .copy_response_snapshot(engine::realtime::ResponseSnapshotRequest { track_id, sink })
    }
}

/// An immutable native response owner prepared for repeated queries.
pub struct PreparedResponsePreview {
    owner: Box<dyn PreparedResponseAnalysis>,
    configuration_id: u64,
}

impl PreparedResponsePreview {
    /// The owner descriptor used by this preparation.
    #[must_use]
    pub fn descriptor(&self) -> &'static effect_contract::ResponseAnalysisDescriptor {
        self.owner.analysis_descriptor()
    }

    /// The owner's normalized immutable configuration view.
    #[must_use]
    pub fn configuration(&self) -> effect_contract::ResponseConfigurationView<'_> {
        self.owner.configuration()
    }

    /// Concrete owner payload bytes retained by this preview.
    #[must_use]
    pub fn retained_bytes(&self) -> usize {
        self.owner.retained_bytes()
    }

    /// Query a generated axis into caller-owned buffers.
    pub fn query_into(
        &self,
        grid: ResponsePreviewGrid,
        output: ResponsePreviewOutput<'_>,
    ) -> Result<ResponseSummary, ResponsePreviewError> {
        let points = grid.points();
        if points < 2 || output.frequencies_hz.len() != points {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        let sample_rate_hz = self.configuration().sample_rate_hz;
        generate_response_grid(grid, sample_rate_hz, output.frequencies_hz)?;
        if output.total_left_db.len() != points || output.total_right_db.len() != points {
            return Err(ResponsePreviewError::OutputShape);
        }
        let sections = self.descriptor().sections.len();
        for values in [
            output.sections_left_db.as_deref(),
            output.sections_right_db.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if values.len()
                != sections
                    .checked_mul(points)
                    .ok_or(ResponsePreviewError::OutputShape)?
            {
                return Err(ResponsePreviewError::OutputShape);
            }
        }
        self.owner
            .query_into(
                OwnerResponseQuery {
                    configuration_id: self.configuration_id,
                    frequencies_hz: output.frequencies_hz,
                    maximum_points: points,
                },
                ResponseOutput {
                    total_left_db: output.total_left_db,
                    total_right_db: output.total_right_db,
                    sections_left_db: output.sections_left_db,
                    sections_right_db: output.sections_right_db,
                },
            )
            .map_err(ResponsePreviewError::Owner)
    }
}

/// Prepare one explicit owner response without compiling a session or instantiating an effect
/// processor.
pub fn prepare_response_preview(
    request: ResponsePreviewRequest<'_>,
) -> Result<PreparedResponsePreview, ResponsePreviewError> {
    if request.sample_rate_hz == 0 || request.quantum_frames == 0 {
        return Err(ResponsePreviewError::InvalidShape);
    }
    let limits = ResponsePrepareLimits {
        maximum_prepared_bytes: request.limits.maximum_prepared_bytes,
    };
    let owner = match request.target {
        ResponsePreviewTarget::InputFilters {
            left_hpf_hz,
            left_lpf_hz,
            right_hpf_hz,
            right_lpf_hz,
        } => {
            let mut configuration = BuiltinParameters::default();
            configuration.left.hpf_hz = left_hpf_hz;
            configuration.left.lpf_hz = left_lpf_hz;
            configuration.right.hpf_hz = right_hpf_hz;
            configuration.right.lpf_hz = right_lpf_hz;
            prepare_input_filter_response(request.sample_rate_hz, configuration, limits)
                .map_err(ResponsePreviewError::Owner)?
        }
        ResponsePreviewTarget::Effect {
            effect_id,
            overrides,
            quality,
            bypass,
            link_mode,
        } => prepare_effect_response(
            &request,
            limits,
            effect_id,
            EffectValues::Overrides(overrides),
            quality,
            bypass,
            link_mode,
        )?,
        ResponsePreviewTarget::ResolvedEffect {
            effect_id,
            values,
            quality,
            bypass,
            link_mode,
        } => prepare_effect_response(
            &request,
            limits,
            effect_id,
            EffectValues::Resolved(values),
            quality,
            bypass,
            link_mode,
        )?,
    };
    Ok(PreparedResponsePreview {
        owner,
        configuration_id: request.configuration_id,
    })
}

/// The values a native effect target prepares with: overrides over the descriptor defaults, or
/// values already resolved for every parameter and lane.
enum EffectValues<'a> {
    Overrides(&'a [ResponseParameterOverride]),
    Resolved(&'a [InitialParameterValue]),
}

/// Prepare one native effect's response owner: the registry entry, its response provider, the
/// initial values and the tail bound, then the owner's own preparation.
fn prepare_effect_response(
    request: &ResponsePreviewRequest<'_>,
    limits: ResponsePrepareLimits,
    effect_id: &str,
    values: EffectValues<'_>,
    quality: EffectQuality,
    bypass: bool,
    link_mode: LinkMode,
) -> Result<Box<dyn PreparedResponseAnalysis>, ResponsePreviewError> {
    let registry =
        launch_native_effect_registry().map_err(|_| ResponsePreviewError::UnknownEffect)?;
    let factory = registry
        .get_ascii(effect_id)
        .ok_or(ResponsePreviewError::UnknownEffect)?;
    let response = factory
        .response_analysis()
        .ok_or(ResponsePreviewError::UnsupportedEffect)?;
    let descriptor = factory.descriptor();
    let defaulted: Vec<InitialParameterValue>;
    let initial_values = match values {
        EffectValues::Overrides(overrides) => {
            let mut initial: Vec<InitialParameterValue> =
                default_initial_values(descriptor).collect();
            apply_overrides(descriptor, &mut initial, overrides)?;
            defaulted = initial;
            &defaulted[..]
        }
        EffectValues::Resolved(values) => {
            // Exactly the slots of the defaults, in their order: a value per parameter and lane.
            let mut slots = default_initial_values(descriptor);
            let complete = values.iter().all(|value| {
                slots.next().is_some_and(|slot| {
                    slot.parameter_index == value.parameter_index && slot.channel == value.channel
                })
            }) && slots.next().is_none();
            if !complete {
                return Err(ResponsePreviewError::InvalidParameter);
            }
            values
        }
    };
    // Issue #1462 D1: the registry's tail-bound entry. A rate or quality the effect does not
    // declare has none, and is the owner's `effect.quality.unsupported` refusal.
    let tail_bound = registry
        .tail_bound(descriptor.id, request.sample_rate_hz, quality)
        .map_err(|error| {
            ResponsePreviewError::Owner(effect_contract::ResponseAnalysisError::Configuration(
                effect_contract::EffectPrepareError { code: error.code },
            ))
        })?;
    response
        .prepare_response(
            effect_contract::PrepareEffectRequest {
                sample_rate: request.sample_rate_hz,
                quantum: request.quantum_frames,
                quality,
                bypass,
                link_mode,
                ports: PreparedPorts {
                    sidechain: effect_contract::PreparedSidechainPort::None,
                },
                initial_values,
                limits: effect_contract::PrepareEffectLimits {
                    maximum_total_state_bytes: request.limits.maximum_total_state_bytes,
                    maximum_scratch_bytes: request.limits.maximum_scratch_bytes,
                    maximum_automation_spans_per_block: request
                        .limits
                        .maximum_automation_spans_per_block,
                },
                tail_bound,
            },
            limits,
        )
        .map_err(ResponsePreviewError::Owner)
}

/// Whether two lanes of one parameter are the same storage: equal lanes, or `both` against any.
/// A (parameter, lane) pair given twice, or `both` beside `left` or `right`, is a duplicate.
fn lanes_overlap(first: ParameterChannel, second: ParameterChannel) -> bool {
    first == second || first == ParameterChannel::Both || second == ParameterChannel::Both
}

/// Apply the numeric overrides over the descriptor defaults, by the decision-17 lane rule: `both`
/// on a per-lane parameter sets both lanes, `left` or `right` sets one, and a shared parameter
/// takes `both` only.
fn apply_overrides(
    descriptor: &'static effect_contract::EffectDescriptor,
    initial_values: &mut [InitialParameterValue],
    overrides: &[ResponseParameterOverride],
) -> Result<(), ResponsePreviewError> {
    for (index, override_value) in overrides.iter().enumerate() {
        let parameter_index = descriptor
            .parameters
            .iter()
            .position(|parameter| parameter.id.0 == override_value.parameter_id)
            .ok_or(ResponsePreviewError::InvalidParameter)?;
        let parameter = &descriptor.parameters[parameter_index];
        let lanes: &[ParameterChannel] = match parameter.channel_policy {
            effect_contract::ParameterChannelPolicy::Shared => {
                if override_value.channel != ParameterChannel::Both {
                    return Err(ResponsePreviewError::ConflictingChannel);
                }
                &[ParameterChannel::Both]
            }
            effect_contract::ParameterChannelPolicy::PerLane => match override_value.channel {
                ParameterChannel::Both => &[ParameterChannel::Left, ParameterChannel::Right],
                ParameterChannel::Left => &[ParameterChannel::Left],
                ParameterChannel::Right => &[ParameterChannel::Right],
            },
        };
        if overrides[..index].iter().any(|previous| {
            previous.parameter_id == override_value.parameter_id
                && lanes_overlap(previous.channel, override_value.channel)
        }) {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        for lane in lanes {
            let slot = initial_values
                .iter_mut()
                .find(|value| {
                    value.parameter_index == parameter_index as u32 && value.channel == *lane
                })
                .ok_or(ResponsePreviewError::InvalidParameter)?;
            slot.value = override_value.value;
        }
    }
    Ok(())
}

/// The input-filter target of a list of override rows over the rows' defaults: the builtin
/// `hpf_hz` and `lpf_hz` parameters, each per lane. An absent lane keeps the row's default;
/// `both` sets both lanes; any other builtin parameter is not part of the input filters.
pub fn input_filter_target(
    overrides: &[ResponseParameterOverride],
) -> Result<ResponsePreviewTarget<'static>, ResponsePreviewError> {
    let default_of = |name: &str| {
        builtins::BUILTIN_PARAMETER_DESCRIPTORS
            .iter()
            .find(|descriptor| descriptor.name == name)
            .map_or(0.0, |descriptor| descriptor.default)
    };
    let (hpf, lpf) = (default_of("hpf_hz"), default_of("lpf_hz"));
    // [left hpf, left lpf, right hpf, right lpf]
    let mut cutoffs = [hpf, lpf, hpf, lpf];
    for (index, row) in overrides.iter().enumerate() {
        let descriptor = builtins::BUILTIN_PARAMETER_DESCRIPTORS
            .iter()
            .find(|descriptor| descriptor.id == row.parameter_id)
            .ok_or(ResponsePreviewError::InvalidParameter)?;
        let offset = match descriptor.name {
            "hpf_hz" => 0,
            "lpf_hz" => 1,
            _ => return Err(ResponsePreviewError::InvalidParameter),
        };
        if overrides[..index].iter().any(|previous| {
            previous.parameter_id == row.parameter_id
                && lanes_overlap(previous.channel, row.channel)
        }) {
            return Err(ResponsePreviewError::DuplicateParameter);
        }
        if matches!(row.channel, ParameterChannel::Left | ParameterChannel::Both) {
            cutoffs[offset] = row.value;
        }
        if matches!(
            row.channel,
            ParameterChannel::Right | ParameterChannel::Both
        ) {
            cutoffs[2 + offset] = row.value;
        }
    }
    Ok(ResponsePreviewTarget::InputFilters {
        left_hpf_hz: cutoffs[0],
        left_lpf_hz: cutoffs[1],
        right_hpf_hz: cutoffs[2],
        right_lpf_hz: cutoffs[3],
    })
}

/// The most points one response query may ask for, on every host.
pub const RESPONSE_MAXIMUM_POINTS: usize = 4096;
/// The most parameter overrides one effect response query may carry, on every host.
pub const RESPONSE_MAXIMUM_OVERRIDES: usize = 256;

/// Which sides of a response a query returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ResponseChannels {
    /// The left side only.
    Left = 1,
    /// The right side only.
    Right = 2,
    /// Both sides.
    Both = 3,
}

impl ResponseChannels {
    /// Decode the wire bits (`1` left, `2` right, `3` both); any other value is `None`.
    #[must_use]
    pub fn from_raw(raw: impl Into<u32>) -> Option<Self> {
        match raw.into() {
            1 => Some(Self::Left),
            2 => Some(Self::Right),
            3 => Some(Self::Both),
            _ => None,
        }
    }

    const fn left(self) -> bool {
        matches!(self, Self::Left | Self::Both)
    }

    const fn right(self) -> bool {
        matches!(self, Self::Right | Self::Both)
    }
}

/// One complete response query: the owner, the axis and the output shape.
#[derive(Clone, Copy, Debug)]
pub struct ResponseQuery<'a> {
    /// Opaque caller correlation identity, echoed in the curves.
    pub configuration_id: u64,
    /// Explicit launch sample rate.
    pub sample_rate_hz: u32,
    /// Explicit render quantum used by native effect validation.
    pub quantum_frames: u32,
    /// One owner-local target.
    pub target: ResponsePreviewTarget<'a>,
    /// The frequency axis.
    pub grid: ResponsePreviewGrid,
    /// Which sides to return.
    pub channels: ResponseChannels,
    /// Whether to return the per-section curves as well as the totals.
    pub sections: bool,
    /// Preparation/resource caps.
    pub limits: ResponsePreviewLimits,
}

/// Why a response query was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseQueryError {
    /// The shared preview preparation or query refused it.
    Preview(ResponsePreviewError),
    /// The point count is outside `2..=RESPONSE_MAXIMUM_POINTS`.
    Points,
    /// More than `RESPONSE_MAXIMUM_OVERRIDES` overrides.
    Overrides,
    /// The sample rate is not a launch rate.
    SampleRate,
    /// An override value (or, for the input filters, a cutoff) is NaN or infinite.
    OverrideValue,
    /// An address target names no strip's builtins, console or inserts section (#1575 D1).
    Address,
    /// An address target names a strip, console slot or insert the session does not have.
    AddressUnknown,
    /// An address target names a third-party (`cid`) effect.
    ThirdParty,
    /// The control validator refused an override of an address target (#1575 D3); the code is
    /// the validator's own.
    OverrideControl(controls::ControlError),
    /// The committed values of an address target do not resolve; the code is the resolver's.
    Committed(&'static str),
}

/// The class of a refused response query, which a host maps to its own status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResponseRefusal {
    /// A field of the request is outside its domain.
    InvalidField,
    /// A count, size or resource bound is exceeded.
    LimitExceeded,
    /// The named effect does not exist.
    NotFound,
    /// The request is well formed and this build cannot serve it.
    Unavailable,
    /// The owner refused the configuration; the code is the owner's own.
    ValidationFailed,
    /// The query failed after it was admitted; no request change cures it.
    Internal,
}

impl ResponseQueryError {
    /// The one refusal table of the response query (decision 17, C13, D6): the class and the
    /// dotted code that every host reports for this error.
    #[must_use]
    pub fn refusal(&self) -> (ResponseRefusal, &'static str) {
        use ResponseRefusal::{
            Internal, InvalidField, LimitExceeded, NotFound, Unavailable, ValidationFailed,
        };
        match *self {
            Self::Points => (InvalidField, "response.points"),
            Self::Overrides => (LimitExceeded, "response.override.count"),
            Self::SampleRate => (InvalidField, "response.rate"),
            Self::OverrideValue => (InvalidField, "response.override.value"),
            Self::Address => (InvalidField, "response.address"),
            Self::AddressUnknown => (NotFound, "control.address.unknown"),
            Self::ThirdParty => (Unavailable, "effect.third_party.unavailable_at_launch"),
            Self::OverrideControl(error) => (InvalidField, error.code()),
            Self::Committed(code) => (ValidationFailed, code),
            Self::Preview(error) => match error {
                ResponsePreviewError::InvalidShape => (InvalidField, "response.rate"),
                ResponsePreviewError::InvalidGrid => (InvalidField, "response.grid"),
                ResponsePreviewError::UnknownEffect => (NotFound, "response.effect.unknown"),
                ResponsePreviewError::UnsupportedEffect => {
                    (Unavailable, "response.effect.unsupported")
                }
                ResponsePreviewError::InvalidParameter => {
                    (InvalidField, "response.override.control")
                }
                ResponsePreviewError::ConflictingChannel => {
                    (InvalidField, "response.override.lane")
                }
                ResponsePreviewError::DuplicateParameter => {
                    (InvalidField, "response.override.duplicate")
                }
                ResponsePreviewError::OutputShape => (Internal, "response.internal"),
                ResponsePreviewError::Owner(error) => match error {
                    ResponseAnalysisError::InvalidFrequencyGrid => (InvalidField, "response.grid"),
                    ResponseAnalysisError::UnsupportedMode
                    | ResponseAnalysisError::UnsupportedCapability => {
                        (Unavailable, "response.effect.unsupported")
                    }
                    ResponseAnalysisError::Configuration(error) => (ValidationFailed, error.code),
                    ResponseAnalysisError::Capacity | ResponseAnalysisError::ResourceLimit => {
                        (LimitExceeded, "response.limit")
                    }
                    ResponseAnalysisError::Numerical => (Internal, "response.numerical"),
                    ResponseAnalysisError::OutputShape => (Internal, "response.internal"),
                },
            },
        }
    }
}

/// The checks of a response query that need no owner, in the order every host reports them: an
/// unsupported rate, a point count outside `2..=4096`, more than [`RESPONSE_MAXIMUM_OVERRIDES`]
/// override rows, the frequency axis (a NaN or infinite endpoint is `response.grid`), then an
/// override value that is NaN or infinite (`response.override.value`, #1623).
/// [`prepare_response_query`] runs them first; a host that shapes the request before calling it
/// (the control provider folds override rows into a bypass and cutoffs) runs them before its own
/// shaping, so the core's refusal wins whichever request word is wrong first.
pub(crate) fn check_response_request(
    sample_rate_hz: u32,
    grid: ResponsePreviewGrid,
    override_count: usize,
    override_values_finite: bool,
) -> Result<(), ResponseQueryError> {
    if !engine::is_launch_sample_rate(engine::SampleRateHz(sample_rate_hz)) {
        return Err(ResponseQueryError::SampleRate);
    }
    if !(2..=RESPONSE_MAXIMUM_POINTS).contains(&grid.points()) {
        return Err(ResponseQueryError::Points);
    }
    if override_count > RESPONSE_MAXIMUM_OVERRIDES {
        return Err(ResponseQueryError::Overrides);
    }
    // The axis rules are the query's, not the owner's: refuse a bad axis here, before any owner is
    // prepared, so a host answers it the same way and with the same code.
    check_response_axis(grid, sample_rate_hz).map_err(ResponseQueryError::Preview)?;
    if !override_values_finite {
        return Err(ResponseQueryError::OverrideValue);
    }
    Ok(())
}

/// A response query whose owner is prepared; `curves` evaluates it.
pub struct PreparedResponseQuery {
    preview: PreparedResponsePreview,
    grid: ResponsePreviewGrid,
    channels: ResponseChannels,
    sections: bool,
}

/// The evaluated curves of one response query.
#[derive(Clone, Debug, PartialEq)]
pub struct ResponseCurves {
    /// The caller's correlation identity.
    pub configuration_id: u64,
    /// The sample rate the owner was prepared at.
    pub sample_rate_hz: u32,
    /// The lowest level a curve reports, in dB.
    pub floor_db: f32,
    /// Effect-wide bypass as the owner holds it.
    pub bypass: bool,
    /// Number of owner sections.
    pub section_count: u32,
    /// Enabled-section mask of the left side: bit `min(index, 31)` per enabled section.
    pub enabled_left: u32,
    /// Enabled-section mask of the right side.
    pub enabled_right: u32,
    /// Owner payload bytes retained by the preparation.
    pub retained_bytes: u64,
    /// The engine-generated frequency axis.
    pub frequencies_hz: Vec<f32>,
    /// Left total, when the channels include left.
    pub left_db: Option<Vec<f32>>,
    /// Right total, when the channels include right.
    pub right_db: Option<Vec<f32>>,
    /// Left section curves, section-major, when sections are asked for and left is included.
    pub sections_left_db: Option<Vec<f32>>,
    /// Right section curves, section-major, when sections are asked for and right is included.
    pub sections_right_db: Option<Vec<f32>>,
}

/// Prepare one response query: refuse an unsupported rate, point count, override count or frequency
/// axis before any owner is prepared, then prepare the owner through [`prepare_response_preview`].
pub fn prepare_response_query(
    query: &ResponseQuery<'_>,
) -> Result<PreparedResponseQuery, ResponseQueryError> {
    // An input-filters target carries its cutoffs, already folded from its rows: they are its
    // override values.
    let (override_count, finite) = match query.target {
        ResponsePreviewTarget::Effect { overrides, .. } => (
            overrides.len(),
            overrides.iter().all(|row| row.value.is_finite()),
        ),
        // A resolved effect's values are the session's, not the request's: the request's own
        // override count was checked before it was resolved.
        ResponsePreviewTarget::ResolvedEffect { values, .. } => {
            (0, values.iter().all(|value| value.value.is_finite()))
        }
        ResponsePreviewTarget::InputFilters {
            left_hpf_hz,
            left_lpf_hz,
            right_hpf_hz,
            right_lpf_hz,
        } => (
            0,
            [left_hpf_hz, left_lpf_hz, right_hpf_hz, right_lpf_hz]
                .iter()
                .all(|cutoff| cutoff.is_finite()),
        ),
    };
    check_response_request(query.sample_rate_hz, query.grid, override_count, finite)?;
    let preview = prepare_response_preview(ResponsePreviewRequest {
        configuration_id: query.configuration_id,
        sample_rate_hz: query.sample_rate_hz,
        quantum_frames: query.quantum_frames,
        target: query.target,
        limits: query.limits,
    })
    .map_err(ResponseQueryError::Preview)?;
    Ok(PreparedResponseQuery {
        preview,
        grid: query.grid,
        channels: query.channels,
        sections: query.sections,
    })
}

impl PreparedResponseQuery {
    /// Number of sections the owner reports.
    #[must_use]
    pub fn section_count(&self) -> usize {
        self.preview.descriptor().sections.len()
    }

    /// Number of points on the axis.
    #[must_use]
    pub fn points(&self) -> usize {
        self.grid.points()
    }

    /// Evaluate the curves. An error leaves nothing behind.
    pub fn curves(&self) -> Result<ResponseCurves, ResponseQueryError> {
        let points = self.points();
        let section_count = self.section_count();
        let section_points =
            section_count
                .checked_mul(points)
                .ok_or(ResponseQueryError::Preview(
                    ResponsePreviewError::OutputShape,
                ))?;
        let mut frequencies = vec![0.0; points];
        let mut left = vec![0.0; points];
        let mut right = vec![0.0; points];
        let mut sections_left = self.sections.then(|| vec![0.0; section_points]);
        let mut sections_right = self.sections.then(|| vec![0.0; section_points]);
        let summary = self
            .preview
            .query_into(
                self.grid,
                ResponsePreviewOutput {
                    frequencies_hz: &mut frequencies,
                    total_left_db: &mut left,
                    total_right_db: &mut right,
                    sections_left_db: sections_left.as_deref_mut(),
                    sections_right_db: sections_right.as_deref_mut(),
                },
            )
            .map_err(ResponseQueryError::Preview)?;
        let configuration = self.preview.configuration();
        let mask = |values: &[bool]| {
            values
                .iter()
                .enumerate()
                .fold(0_u32, |mask, (index, enabled)| {
                    mask | (u32::from(*enabled) << index.min(31))
                })
        };
        let want_left = self.channels.left();
        let want_right = self.channels.right();
        Ok(ResponseCurves {
            configuration_id: summary.configuration_id,
            sample_rate_hz: summary.sample_rate_hz,
            floor_db: summary.floor_db,
            bypass: configuration.bypass.unwrap_or(false),
            section_count: u32::try_from(section_count).unwrap_or(0),
            enabled_left: mask(configuration.enabled_left),
            enabled_right: mask(configuration.enabled_right),
            retained_bytes: u64::try_from(self.preview.retained_bytes()).unwrap_or(u64::MAX),
            frequencies_hz: frequencies,
            left_db: want_left.then_some(left),
            right_db: want_right.then_some(right),
            sections_left_db: sections_left.filter(|_| want_left),
            sections_right_db: sections_right.filter(|_| want_right),
        })
    }
}

/// The frequency-axis rules of a response query, checked once and before any owner is prepared or
/// any buffer is allocated (decision 17, C13): both endpoints are finite, the minimum is not
/// negative and below the maximum, the maximum is at most Nyquist, a logarithmic axis starts above
/// zero, and the axis has at least two points.
fn check_response_axis(
    grid: ResponsePreviewGrid,
    sample_rate_hz: u32,
) -> Result<(), ResponsePreviewError> {
    let (minimum_hz, maximum_hz) = grid.endpoints();
    if grid.points() < 2
        || !minimum_hz.is_finite()
        || !maximum_hz.is_finite()
        || minimum_hz < 0.0
        || minimum_hz >= maximum_hz
        || maximum_hz > sample_rate_hz as f32 * 0.5
        || (matches!(grid, ResponsePreviewGrid::Logarithmic { .. }) && minimum_hz <= 0.0)
    {
        return Err(ResponsePreviewError::InvalidGrid);
    }
    Ok(())
}

/// Generate and validate one explicit frequency axis with portable f64 interpolation.
pub fn generate_response_grid(
    grid: ResponsePreviewGrid,
    sample_rate_hz: u32,
    output: &mut [f32],
) -> Result<(), ResponsePreviewError> {
    let points = grid.points();
    let (minimum_hz, maximum_hz) = grid.endpoints();
    if output.len() != points {
        return Err(ResponsePreviewError::InvalidGrid);
    }
    check_response_axis(grid, sample_rate_hz)?;
    let denominator = (points - 1) as f64;
    for index in 0..points {
        let t = index as f64 / denominator;
        let result = match grid {
            ResponsePreviewGrid::Linear { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                minimum + (maximum - minimum) * t
            }
            ResponsePreviewGrid::Logarithmic { .. } => {
                let minimum = f64::from(minimum_hz);
                let maximum = f64::from(maximum_hz);
                exp(log(minimum) + (log(maximum) - log(minimum)) * t)
            }
        } as f32;
        if !result.is_finite() || (index > 0 && result <= output[index - 1]) {
            return Err(ResponsePreviewError::InvalidGrid);
        }
        output[index] = result;
    }
    output[0] = minimum_hz;
    output[points - 1] = maximum_hz;
    Ok(())
}

/// Compose one copied track snapshot through the existing native owner evaluators.
///
/// Each owner writes positive unfloored magnitudes into bounded scratch arrays.  The host sums
/// `log10` magnitudes in `f64`, remembers exact zeros separately, and applies the public `-120 dB`
/// floor only once after every owner has been validated.  No render state or caller parameters
/// participate in this operation.
pub fn query_response_snapshot_into(
    snapshot: &ResponseSnapshot,
    grid: ResponsePreviewGrid,
    output: ResponseSnapshotOutput<'_>,
) -> Result<ResponseSnapshotQuerySummary, ResponseSnapshotQueryError> {
    let points = grid.points();
    if points < 2
        || output.frequencies_hz.len() != points
        || output.total_left_db.len() != points
        || output.total_right_db.len() != points
    {
        return Err(ResponseSnapshotQueryError::OutputShape);
    }
    let mut frequencies = Vec::new();
    frequencies
        .try_reserve_exact(points)
        .map_err(|_| ResponseSnapshotQueryError::Capacity)?;
    frequencies.resize(points, 0.0);
    generate_response_grid(grid, snapshot.sample_rate_hz, &mut frequencies)
        .map_err(|_| ResponseSnapshotQueryError::InvalidGrid)?;

    let mut left_sum = Vec::new();
    let mut right_sum = Vec::new();
    let mut left_magnitudes = Vec::new();
    let mut right_magnitudes = Vec::new();
    for values in [
        &mut left_sum,
        &mut right_sum,
        &mut left_magnitudes,
        &mut right_magnitudes,
    ] {
        values
            .try_reserve_exact(points)
            .map_err(|_| ResponseSnapshotQueryError::Capacity)?;
        values.resize(points, 0.0);
    }
    let mut left_zero = vec![false; points];
    let mut right_zero = vec![false; points];
    let mut provided = 0_u32;
    let mut excluded = 0_u32;

    for owner in snapshot.owners.iter() {
        if owner.availability == ResponseSnapshotAvailability::DeclaredUnavailable {
            excluded = excluded
                .checked_add(1)
                .ok_or(ResponseSnapshotQueryError::Capacity)?;
            continue;
        }
        if owner.left.is_empty() || owner.right.is_empty() {
            return Err(ResponseSnapshotQueryError::MalformedSnapshot);
        }
        match owner.kind {
            1 => effect_compiler::query_parametric_eq_snapshot_magnitudes_into(
                snapshot.sample_rate_hz,
                owner.bypassed,
                &owner.left,
                &owner.right,
                &frequencies,
                points,
                &mut left_magnitudes,
                &mut right_magnitudes,
            )
            .map_err(map_eq_snapshot_error)?,
            2 => builtins::query_input_filter_snapshot_magnitudes_into(
                snapshot.sample_rate_hz,
                owner.bypassed,
                &owner.left,
                &owner.right,
                &frequencies,
                points,
                &mut left_magnitudes,
                &mut right_magnitudes,
            )
            .map_err(map_builtin_snapshot_error)?,
            _ => return Err(ResponseSnapshotQueryError::MalformedSnapshot),
        }
        provided = provided
            .checked_add(1)
            .ok_or(ResponseSnapshotQueryError::Capacity)?;
        for index in 0..points {
            let left = left_magnitudes[index];
            let right = right_magnitudes[index];
            if !left.is_finite() || !right.is_finite() || left < 0.0 || right < 0.0 {
                return Err(ResponseSnapshotQueryError::Numerical);
            }
            if left == 0.0 {
                left_zero[index] = true;
            } else if !left_zero[index] {
                left_sum[index] += math::log10(left);
            }
            if right == 0.0 {
                right_zero[index] = true;
            } else if !right_zero[index] {
                right_sum[index] += math::log10(right);
            }
        }
    }
    if provided == 0 {
        return Err(ResponseSnapshotQueryError::UnsupportedProvider);
    }
    for index in 0..points {
        let left_db = if left_zero[index] {
            -120.0
        } else {
            20.0 * left_sum[index]
        };
        let right_db = if right_zero[index] {
            -120.0
        } else {
            20.0 * right_sum[index]
        };
        if !left_db.is_finite() || !right_db.is_finite() {
            return Err(ResponseSnapshotQueryError::Numerical);
        }
        output.total_left_db[index] = (left_db as f32).max(-120.0);
        output.total_right_db[index] = (right_db as f32).max(-120.0);
    }
    output.frequencies_hz.copy_from_slice(&frequencies);
    Ok(ResponseSnapshotQuerySummary {
        captured_sample: snapshot.captured_sample,
        sample_rate_hz: snapshot.sample_rate_hz,
        owners: u32::try_from(snapshot.owners.len())
            .map_err(|_| ResponseSnapshotQueryError::Capacity)?,
        excluded_owners: excluded,
        mode: ResponseSnapshotMode::Target,
    })
}

/// Where one track-response member sits in its strip's signal chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResponseMemberSection {
    /// The strip's builtin input section (polarity, trim, HPF, LPF).
    Builtins,
    /// A console slot, named by its session slot ID. Slot IDs are unique across both console
    /// sections, so the session resolves whether it is a pre-insert or post-insert slot.
    Console(Box<str>),
    /// A strip insert, named by its session insert ID.
    Inserts(Box<str>),
}

/// The response provider of one track-response member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ResponseMemberKind {
    /// A native parametric EQ.
    ParametricEq = 1,
    /// The builtin input filters.
    InputFilters = 2,
    /// A declared owner without a linear response provider, excluded from the curves.
    Unavailable = 3,
}

/// One owner that a track response names, in declared signal order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponseMember {
    /// Where the owner sits in the strip.
    pub section: ResponseMemberSection,
    /// The owner's native provider identity.
    pub native_id: Box<str>,
    /// The owner's response provider.
    pub kind: ResponseMemberKind,
    /// Bypass state captured at the same boundary as the curves.
    pub bypassed: bool,
    /// The owner's left section count.
    pub section_count: u8,
    /// Bit `i` is set when left section `i` is enabled.
    pub enabled_left: u32,
    /// Bit `i` is set when right section `i` is enabled.
    pub enabled_right: u32,
}

/// The composed response of one strip at one captured boundary, with the owners that make it.
#[derive(Clone, Debug, PartialEq)]
pub struct TrackResponse {
    /// The strip the response belongs to.
    pub strip_id: Box<str>,
    /// The boundary sample of the captured values.
    pub captured_sample: u64,
    /// The plan's sample rate.
    pub sample_rate_hz: u32,
    /// The floor every curve value is clamped to: `-120.0` dB.
    pub floor_db: f32,
    /// The generated frequency axis.
    pub frequencies_hz: Vec<f32>,
    /// The left curve in dB, when the query asked for the left side.
    pub left_db: Option<Vec<f32>>,
    /// The right curve in dB, when the query asked for the right side.
    pub right_db: Option<Vec<f32>>,
    /// The number of members without a linear response provider.
    pub excluded_count: u32,
    /// Every captured owner, in declared signal order.
    pub members: Vec<ResponseMember>,
}

/// Compose one strip's captured snapshot into its response and its members.
///
/// Refuses a point count outside `2..=RESPONSE_MAXIMUM_POINTS` ([`InvalidGrid`]), and an owner of
/// another strip or with a rack it cannot name ([`MalformedSnapshot`]); then composes the curves
/// through [`query_response_snapshot_into`], so the curve values are that function's own.
///
/// [`InvalidGrid`]: ResponseSnapshotQueryError::InvalidGrid
/// [`MalformedSnapshot`]: ResponseSnapshotQueryError::MalformedSnapshot
pub fn compose_track_response(
    strip_id: &str,
    snapshot: &ResponseSnapshot,
    grid: ResponsePreviewGrid,
    channels: ResponseChannels,
) -> Result<TrackResponse, ResponseSnapshotQueryError> {
    let points = grid.points();
    if !(2..=RESPONSE_MAXIMUM_POINTS).contains(&points) {
        return Err(ResponseSnapshotQueryError::InvalidGrid);
    }
    let members = snapshot
        .owners
        .iter()
        .map(|owner| response_member(strip_id, owner))
        .collect::<Result<Vec<_>, _>>()?;
    let mut frequencies_hz = vec![0.0_f32; points];
    let mut left = vec![0.0_f32; points];
    let mut right = vec![0.0_f32; points];
    let summary = query_response_snapshot_into(
        snapshot,
        grid,
        ResponseSnapshotOutput {
            frequencies_hz: &mut frequencies_hz,
            total_left_db: &mut left,
            total_right_db: &mut right,
        },
    )?;
    Ok(TrackResponse {
        strip_id: strip_id.into(),
        captured_sample: summary.captured_sample,
        sample_rate_hz: summary.sample_rate_hz,
        floor_db: -120.0,
        frequencies_hz,
        left_db: channels.left().then_some(left),
        right_db: channels.right().then_some(right),
        excluded_count: summary.excluded_owners,
        members,
    })
}

fn response_member(
    strip_id: &str,
    owner: &ResponseSnapshotOwner,
) -> Result<ResponseMember, ResponseSnapshotQueryError> {
    if *owner.track_id != *strip_id {
        return Err(ResponseSnapshotQueryError::MalformedSnapshot);
    }
    let section = match owner.rack {
        0 => ResponseMemberSection::Builtins,
        1 | 3 => ResponseMemberSection::Console(owner.stable_id.clone()),
        2 => ResponseMemberSection::Inserts(owner.stable_id.clone()),
        _ => return Err(ResponseSnapshotQueryError::MalformedSnapshot),
    };
    let kind = match (owner.availability, owner.kind) {
        (ResponseSnapshotAvailability::DeclaredUnavailable, _) => ResponseMemberKind::Unavailable,
        (_, 1) => ResponseMemberKind::ParametricEq,
        (_, 2) => ResponseMemberKind::InputFilters,
        _ => return Err(ResponseSnapshotQueryError::MalformedSnapshot),
    };
    let section_count = u8::try_from(owner.left.len())
        .map_err(|_| ResponseSnapshotQueryError::MalformedSnapshot)?;
    Ok(ResponseMember {
        section,
        native_id: owner.native_id.clone(),
        kind,
        bypassed: owner.bypassed,
        section_count,
        enabled_left: enabled_mask(&owner.left)?,
        enabled_right: enabled_mask(&owner.right)?,
    })
}

fn enabled_mask(sections: &[ResponseSnapshotSection]) -> Result<u32, ResponseSnapshotQueryError> {
    if sections.len() > 32 {
        return Err(ResponseSnapshotQueryError::MalformedSnapshot);
    }
    Ok(sections
        .iter()
        .enumerate()
        .filter(|(_, section)| section.enabled)
        .fold(0, |mask, (index, _)| mask | 1 << index))
}

fn map_eq_snapshot_error(error: effect_compiler::EqResponseError) -> ResponseSnapshotQueryError {
    match error {
        effect_compiler::EqResponseError::InvalidFrequencyGrid => {
            ResponseSnapshotQueryError::InvalidGrid
        }
        effect_compiler::EqResponseError::Capacity => ResponseSnapshotQueryError::Capacity,
        effect_compiler::EqResponseError::OutputShape => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
        effect_compiler::EqResponseError::Numerical => ResponseSnapshotQueryError::Numerical,
        effect_compiler::EqResponseError::Configuration(_) => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
    }
}

fn map_builtin_snapshot_error(
    error: builtins::InputFilterResponseError,
) -> ResponseSnapshotQueryError {
    match error {
        builtins::InputFilterResponseError::UnsupportedSampleRate
        | builtins::InputFilterResponseError::InvalidFrequencyGrid => {
            ResponseSnapshotQueryError::InvalidGrid
        }
        builtins::InputFilterResponseError::Capacity => ResponseSnapshotQueryError::Capacity,
        builtins::InputFilterResponseError::OutputShape
        | builtins::InputFilterResponseError::Configuration(_) => {
            ResponseSnapshotQueryError::MalformedSnapshot
        }
        builtins::InputFilterResponseError::Numerical => ResponseSnapshotQueryError::Numerical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        HostPrepareCaps, HostShapePolicy, compile_host_model, parse_host_session,
        prepare_host_runtime,
    };
    use effect_contract::{EffectQuality, LinkMode};

    #[test]
    fn explicit_eq_and_input_filter_queries_use_owner_outputs_at_launch_rates() {
        for sample_rate_hz in [44_100, 48_000, 88_200, 96_000] {
            let eq = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 9_007_199_254_740_993,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::Effect {
                    effect_id: "miso.parametric-eq",
                    overrides: &[],
                    quality: EffectQuality::Normal,
                    bypass: false,
                    link_mode: LinkMode::DualMono,
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("EQ response owner");
            let mut frequencies = [0.0; 4];
            let mut left = [0.0; 4];
            let mut right = [0.0; 4];
            let mut sections_left = vec![0.0; frequencies.len() * eq.descriptor().sections.len()];
            let mut sections_right = vec![0.0; frequencies.len() * eq.descriptor().sections.len()];
            let summary = eq
                .query_into(
                    ResponsePreviewGrid::Linear {
                        points: 4,
                        minimum_hz: 0.0,
                        maximum_hz: sample_rate_hz as f32 * 0.5,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("EQ query");
            assert_eq!(summary.configuration_id, 9_007_199_254_740_993);
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert_eq!(frequencies[0], 0.0);
            assert_eq!(frequencies[3], sample_rate_hz as f32 * 0.5);

            let filters = prepare_response_preview(ResponsePreviewRequest {
                configuration_id: 7,
                sample_rate_hz,
                quantum_frames: 128,
                target: ResponsePreviewTarget::InputFilters {
                    left_hpf_hz: 80.0,
                    left_lpf_hz: 18_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                    right_hpf_hz: 120.0,
                    right_lpf_hz: 16_000.0_f32.min(sample_rate_hz as f32 * 0.45),
                },
                limits: ResponsePreviewLimits::default(),
            })
            .expect("input-filter response owner");
            let mut frequencies = [0.0; 3];
            let mut left = [0.0; 3];
            let mut right = [0.0; 3];
            let mut sections_left = [0.0; 6];
            let mut sections_right = [0.0; 6];
            filters
                .query_into(
                    ResponsePreviewGrid::Logarithmic {
                        points: 3,
                        minimum_hz: 20.0,
                        maximum_hz: sample_rate_hz as f32 * 0.45,
                    },
                    ResponsePreviewOutput {
                        frequencies_hz: &mut frequencies,
                        total_left_db: &mut left,
                        total_right_db: &mut right,
                        sections_left_db: Some(&mut sections_left),
                        sections_right_db: Some(&mut sections_right),
                    },
                )
                .expect("input-filter query");
            assert!(
                left.iter()
                    .chain(right.iter())
                    .all(|value| value.is_finite())
            );
            assert!(frequencies.windows(2).all(|pair| pair[1] > pair[0]));
        }
    }

    /// Host caps with every resource ceiling open, for the prepared-session snapshot tests.
    fn unbounded_caps() -> HostPrepareCaps {
        HostPrepareCaps {
            shape: HostShapePolicy::AnyLaunchRate,
            source_ring_frames: 256,
            maximum_automation_spans_per_block: 4096,
            maximum_tracks: u64::MAX,
            maximum_submixes: u64::MAX,
            maximum_vcas: u64::MAX,
            maximum_vca_reach_pairs: u64::MAX,
            maximum_sources: u64::MAX,
            maximum_routes: u64::MAX,
            maximum_effects: u64::MAX,
            maximum_graph_session_plus_plan_bytes: u64::MAX,
            maximum_source_total_bytes: u64::MAX,
            maximum_source_overhead_bytes: u64::MAX,
            maximum_effect_state_bytes: u64::MAX,
            maximum_effect_scratch_bytes: u64::MAX,
            maximum_builtin_retained_bytes: u64::MAX,
            maximum_named_allocation_bytes: u64::MAX,
            maximum_meter_streams: u64::MAX,
            maximum_meter_items: u64::MAX,
            maximum_meter_bytes: u64::MAX,
        }
    }

    /// A per-node effect's snapshot reports its prepared bypass (#1460 D3): the multiband
    /// compressor keeps its session bypass as a prepared `bypass` until #1340 (a lone insert
    /// renders per node), and the owner record must carry it, for both values.
    #[test]
    fn a_per_node_effect_snapshot_reports_its_prepared_bypass() {
        for bypass in [false, true] {
            let caps = unbounded_caps();
            let mut model = parse_host_session(include_str!(
                "../../../fixtures/session/v1/parametric-eq-nine-track.json"
            ))
            .expect("EQ fixture");
            let mut multiband = model.lower_track(&model.tracks[0]).pre_insert[0].clone();
            multiband.id = session::StableId::parse("mb").expect("stable id");
            multiband.identity = session::EffectIdentity::Native {
                effect_id: session::StableId::parse("miso.multiband-compressor")
                    .expect("stable id"),
            };
            multiband.params.clear();
            multiband.bypass = bypass;
            model.tracks[0].inserts.effects = vec![multiband];
            let compiled = compile_host_model(
                &model,
                caps.compile_caps(model.sources.len())
                    .expect("compile caps"),
            )
            .expect("multiband session");
            let mut host = prepare_host_runtime(&compiled, &caps).expect("prepared host");
            let mut collector = ResponseSnapshotCollector::new(
                48_000,
                8,
                effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS,
                32,
            );
            let track = model.tracks[0].id.as_str().to_owned();
            let capture = host
                .copy_response_snapshot(&track, &mut collector)
                .expect("multiband track capture");
            let snapshot = collector.finish(capture);
            let owner = snapshot
                .owners
                .iter()
                .find(|owner| owner.native_id.as_ref() == "miso.multiband-compressor")
                .expect("multiband owner");
            assert_eq!(
                owner.availability,
                ResponseSnapshotAvailability::DeclaredUnavailable
            );
            assert_eq!(owner.bypassed, bypass, "the multiband's prepared bypass");
        }
    }

    #[test]
    fn mixed_prepared_track_composes_real_filter_eq_and_excludes_compressor() {
        let caps = unbounded_caps();
        let mut model = parse_host_session(include_str!(
            "../../../fixtures/session/v1/parametric-eq-nine-track.json"
        ))
        .expect("EQ fixture");
        let compressor = parse_host_session(include_str!(
            "../../../fixtures/session/v1/compressor-dynamic-observation.json"
        ))
        .expect("compressor fixture");
        model.tracks[0].inserts = compressor.tracks[0].inserts.clone();
        let compiled = compile_host_model(
            &model,
            caps.compile_caps(model.sources.len())
                .expect("compile caps"),
        )
        .expect("mixed prepared session");
        let mut host = prepare_host_runtime(&compiled, &caps).expect("prepared host");
        let mut collector = ResponseSnapshotCollector::new(
            48_000,
            8,
            effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS,
            32,
        );
        let before_capture = bench_support::alloc::current_thread_counters();
        let capture = host
            .copy_response_snapshot("eq0", &mut collector)
            .expect("mixed track capture");
        let capture_delta = bench_support::alloc::current_thread_delta_since(before_capture);
        assert_eq!(capture_delta.allocations, 0, "snapshot capture allocated");
        assert_eq!(
            capture_delta.reallocations, 0,
            "snapshot capture reallocated"
        );
        assert_eq!(capture_delta.deallocations, 0, "snapshot capture freed");
        assert_eq!(capture.captured_sample, 0);
        let snapshot = collector.finish(capture);
        assert_eq!(snapshot.owners.len(), 3);
        assert_eq!(
            snapshot
                .owners
                .iter()
                .map(|owner| owner.stable_id.as_ref())
                .collect::<Vec<_>>(),
            vec!["input-filters", "eq", "comp"]
        );
        assert_eq!(
            snapshot
                .owners
                .iter()
                .map(|owner| owner.native_id.as_ref())
                .collect::<Vec<_>>(),
            vec![
                "miso.builtin.input-filters",
                "miso.parametric-eq",
                "miso.compressor",
            ]
        );
        assert_eq!(
            snapshot.owners[2].availability,
            ResponseSnapshotAvailability::DeclaredUnavailable
        );
        assert!(snapshot.owners[2].left.is_empty());
        let eq_owner = snapshot
            .owners
            .iter()
            .find(|owner| owner.native_id.as_ref() == "miso.parametric-eq")
            .expect("EQ owner");
        assert_eq!(
            eq_owner.left.len(),
            effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
        );
        assert_eq!(
            eq_owner.right.len(),
            effect_contract::RESPONSE_SNAPSHOT_MAXIMUM_SECTIONS
        );
        let input_filter_owner = snapshot
            .owners
            .iter()
            .find(|owner| owner.native_id.as_ref() == "miso.builtin.input-filters")
            .expect("input-filter owner");
        assert_eq!(input_filter_owner.left.len(), 2);
        assert_eq!(input_filter_owner.right.len(), 2);

        let mut frequencies = [0.0; 5];
        let mut left = [0.0; 5];
        let mut right = [0.0; 5];
        let summary = query_response_snapshot_into(
            &snapshot,
            ResponsePreviewGrid::Linear {
                points: 5,
                minimum_hz: 20.0,
                maximum_hz: 20_000.0,
            },
            ResponseSnapshotOutput {
                frequencies_hz: &mut frequencies,
                total_left_db: &mut left,
                total_right_db: &mut right,
            },
        )
        .expect("composed mixed response");
        assert_eq!(summary.owners, 3);
        assert_eq!(summary.excluded_owners, 1);
        assert!(
            left.iter()
                .chain(right.iter())
                .all(|value| value.is_finite())
        );
        assert!(left.iter().any(|value| value.abs() > 0.01));
    }
}
