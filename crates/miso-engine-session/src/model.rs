//! Typed declarative V1 session model.

use crate::StableId;

pub(crate) trait ClosedToken: Copy + 'static {
    const ALL: &'static [(Self, &'static str)];
}

#[rustfmt::skip]
macro_rules! closed_tokens {
    ($(#[$enum_meta:meta])* pub enum $name:ident {
        $($(#[$variant_meta:meta])* $variant:ident => $token:literal),+ $(,)?
    }) => {
        $(#[$enum_meta])*
        #[repr(u8)]
        pub enum $name { $($(#[$variant_meta])* $variant),+ }

        impl $name {
            /// Every token value in declaration and wire-code order.
            pub const ALL: &'static [(Self, &'static str)] = &[$((Self::$variant, $token)),+];
            /// Return this value's canonical session token.
            #[must_use]
            pub const fn token(self) -> &'static str {
                match self { $(Self::$variant => $token),+ }
            }
            /// Parse one canonical session token.
            #[must_use]
            pub fn from_token(token: &str) -> Option<Self> {
                Self::ALL
                    .iter()
                    .find_map(|(value, candidate)| (*candidate == token).then_some(*value))
            }
            /// Return the stable nonzero wire code in declaration order.
            #[must_use]
            pub const fn wire(self) -> u8 { self as u8 + 1 }
            /// Parse a stable nonzero wire code.
            #[must_use]
            pub fn from_wire(wire: u8) -> Option<Self> {
                wire.checked_sub(1)
                    .and_then(|index| Self::ALL.get(usize::from(index)))
                    .map(|(value, _)| *value)
            }
        }
        impl ClosedToken for $name {
            const ALL: &'static [(Self, &'static str)] = Self::ALL;
        }
    };
}

/// Strict V1 TOML input after syntax/schema parsing.
#[derive(Clone, Debug, PartialEq)]
pub struct SessionTomlV1 {
    /// Must equal `SESSION_SCHEMA_VERSION_V1`.
    pub schema_version: u32,
    /// Stable session identity.
    pub session_id: StableId,
    /// Caller-controlled monotonic revision.
    pub revision: u64,
    /// Explicit engine sample rate in hertz.
    pub sample_rate_hz: u32,
    /// Explicit render quantum in sample frames.
    pub quantum_frames: u32,
    /// Declarative render profile.
    pub render_profile: RenderProfile,
    /// Explicit PCM output shape.
    pub output_profile: OutputProfile,
    /// Session-declared runtime resource limits.
    pub limits: SessionLimits,
    /// Sources, order-insensitive by stable ID.
    pub sources: Vec<Source>,
    /// Tracks, order-insensitive by stable ID.
    pub tracks: Vec<Track>,
    /// Submixes, order-insensitive by stable ID.
    pub submixes: Vec<Submix>,
    /// Outputs, order-insensitive by stable ID.
    pub outputs: Vec<Output>,
    /// Declarative routes; graph semantics are owned by issue 006.
    pub routes: Vec<Route>,
    /// Ordered sample-time automation programs.
    pub automation: Vec<Automation>,
}

/// Renderer selection declaration, not a host capability query.
#[derive(Clone, Debug, PartialEq)]
pub struct RenderProfile {
    /// Stable profile identity.
    pub id: StableId,
    /// A closed V1 profile token.
    pub mode: RenderMode,
}

closed_tokens! {
    /// V1 render profile tokens.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RenderMode {
        /// Deterministic single-control-thread preparation.
        SingleThread => "single_thread",
        /// Later worker availability is declarative only in this issue.
        DependencyWaves => "dependency_waves",
    }
}

/// Explicit PCM output profile.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputProfile {
    /// Stable output-profile identity.
    pub id: StableId,
    /// Number of planar PCM channels. V1 requires exactly two.
    pub channels: u8,
    /// V1 PCM sample representation.
    pub sample_format: SampleFormat,
}

closed_tokens! {
    /// V1 output scalar token.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SampleFormat {
        /// Planar IEEE `f32` PCM.
        F32Planar => "f32_planar",
    }
}

/// Explicit session resource declarations with unit-bearing field names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionLimits {
    /// PCM ring capacity in sample frames.
    pub pcm_ring_frames: u64,
    /// Control queue capacity in messages.
    pub control_queue_messages: u64,
    /// Declared memory budget in bytes.
    pub memory_bytes: u64,
}

/// A just-in-time source declaration; resolution is deferred to issue 010.
#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    /// Stable source identity.
    pub id: StableId,
    /// Declared native source sample rate in hertz; resolution and rate matching are deferred.
    pub sample_rate_hz: u32,
    /// Content identity and a host-resolved locator string.
    pub content: SourceContent,
    /// Explicit channel and region mapping.
    pub mapping: SourceMapping,
}

/// Source identity data without resolver behavior.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceContent {
    /// Opaque deterministic content identity text.
    pub identity: String,
    /// Opaque resolver locator text.
    pub locator: String,
}

/// Mapping from source content to a session source.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceMapping {
    /// Declared source channels.
    pub channel_count: u8,
    /// Region in source sample frames.
    pub region: SourceRegion,
}

/// Inclusive start plus exact length in source sample frames.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceRegion {
    /// First source sample frame.
    pub start_sample: u64,
    /// Number of source sample frames.
    pub length_samples: u64,
}

/// A dual-mono track declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Track {
    /// Stable track identity.
    pub id: StableId,
    /// Reference to a declared source.
    pub source_id: StableId,
    /// Zero-based source channel mapped to the left dual-mono lane.
    pub left_source_channel: u8,
    /// Zero-based source channel mapped to the right dual-mono lane.
    pub right_source_channel: u8,
    /// Independent left/right fixed input processors.
    pub builtins: DualMonoBuiltins,
    /// Homogeneous-rack candidate declarations; compatibility is deferred to issue 008.
    pub simd1: Rack,
    /// General dynamic-rack declarations.
    pub dynamic: Rack,
    /// Second homogeneous-rack candidate declarations.
    pub simd2: Rack,
    /// Independent left/right fader and mute declaration.
    pub fader: DualMonoFader,
    /// Explicit pan or cross-channel matrix; no implicit stereo operation exists.
    pub matrix_or_pan: MatrixOrPan,
}

/// Independent builtins for the two dual-mono lanes.
#[derive(Clone, Debug, PartialEq)]
pub struct DualMonoBuiltins {
    /// Left lane state/parameters.
    pub left: ChannelBuiltins,
    /// Right lane state/parameters.
    pub right: ChannelBuiltins,
}

/// Builtin parameters with explicit `_db`/`_hz` units.
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelBuiltins {
    /// Explicit polarity inversion.
    pub polarity_invert: bool,
    /// Input trim in decibels.
    pub trim_db: f32,
    /// High-pass cutoff in hertz. Nyquist validation is issue 007.
    pub hpf_hz: f32,
    /// Low-pass cutoff in hertz. Nyquist validation is issue 007.
    pub lpf_hz: f32,
}

/// An ordered effect rack.
#[derive(Clone, Debug, PartialEq)]
pub struct Rack {
    /// Effect order is semantically significant and preserved canonically.
    pub effects: Vec<Effect>,
}

/// Typed effect declaration; availability and CID validity are deferred to issue 011.
#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    /// Stable local slot identity.
    pub id: StableId,
    /// Tagged native-or-third-party effect identity.
    pub identity: EffectIdentity,
    /// Requested quality profile.
    pub quality: EffectQuality,
    /// Latency-preserving bypass declaration.
    pub bypass: bool,
    /// Explicit detector/channel link mode.
    pub link_mode: LinkMode,
    /// Parameter declarations, canonicalized by parameter ID then channel.
    pub params: Vec<EffectParam>,
    /// Explicit typed sidechain declaration, including an explicit `none` variant.
    pub sidechain: SidechainDeclaration,
}

/// A declared native effect ID or opaque third-party CID text, never both.
#[derive(Clone, Debug, PartialEq)]
pub enum EffectIdentity {
    /// Native effect contract identity. Contract validation is deferred to issue 011.
    Native {
        /// Stable native effect contract identifier.
        effect_id: StableId,
    },
    /// Third-party package CIDv1 text. CID validation is deferred to issue 011.
    ThirdPartyCid {
        /// Opaque nonempty CIDv1 text.
        cid: String,
    },
}

closed_tokens! {
    /// Closed V1 quality token set.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum EffectQuality {
        /// Lowest declared effect quality.
        Draft => "draft",
        /// Standard declared effect quality.
        Normal => "normal",
        /// Highest declared effect quality.
        High => "high",
    }
}

closed_tokens! {
    /// Explicit detector link behavior.
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum LinkMode {
        /// Fully independent dual-mono detectors.
        DualMono => "dual_mono",
        /// Maximum of lane detector values.
        Maximum => "maximum",
        /// Arithmetic average of lane detector values.
        Average => "average",
    }
}

/// One typed effect parameter value.
#[derive(Clone, Debug, PartialEq)]
pub struct EffectParam {
    /// Stable parameter ID supplied by its effect contract.
    pub parameter_id: u32,
    /// Explicit parameter lane selection.
    pub channel: ParameterChannel,
    /// Unit token used for schema-local value validation.
    pub unit: ParameterUnit,
    /// Finite `f32` parameter value.
    pub value: f32,
}

closed_tokens! {
    /// Explicit lane selection for a parameter.
    #[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
    pub enum ParameterChannel {
        /// Left dual-mono lane.
        Left => "left",
        /// Right dual-mono lane.
        Right => "right",
        /// Both lanes by an explicit common parameter.
        Both => "both",
    }
}

closed_tokens! {
    /// V1 parameter units. Effect-specific ranges are future effect-contract work.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ParameterUnit {
        /// Decibels.
        Db => "db",
        /// Hertz.
        Hz => "hz",
        /// Milliseconds.
        Milliseconds => "milliseconds",
        /// Sample frames.
        Samples => "samples",
        /// Unitless scalar.
        Linear => "linear",
        /// Ratio scalar.
        Ratio => "ratio",
    }
}

/// Explicit presence or absence of a sidechain.
#[derive(Clone, Debug, PartialEq)]
pub enum SidechainDeclaration {
    /// The effect declares no sidechain input.
    None,
    /// The effect receives a typed sidechain route.
    Routed(Sidechain),
}

/// An explicit sidechain source/tap declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Sidechain {
    /// Typed route source providing detector audio.
    pub source: RouteSource,
    /// Stable target port identity; existence is deferred to issue 006/011.
    pub port_id: StableId,
}

/// An ordered explicit post-input fader declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct DualMonoFader {
    /// Left lane gain in decibels.
    pub left_db: f32,
    /// Right lane gain in decibels.
    pub right_db: f32,
    /// Explicit left-lane mute state.
    pub left_mute: bool,
    /// Explicit right-lane mute state.
    pub right_mute: bool,
}

/// Explicitly choose a regular pan pair or all four cross-channel matrix coefficients.
#[derive(Clone, Debug, PartialEq)]
pub enum MatrixOrPan {
    /// Independent lane pan gains.
    Pan {
        /// Left lane pan gain.
        left: f32,
        /// Right lane pan gain.
        right: f32,
        /// Explicit smoothing duration in sample frames.
        smoothing_samples: u32,
    },
    /// A full left/right 2x2 transfer matrix.
    Matrix {
        /// Left output from left input.
        ll: f32,
        /// Left output from right input.
        lr: f32,
        /// Right output from left input.
        rl: f32,
        /// Right output from right input.
        rr: f32,
        /// Explicit smoothing duration in sample frames.
        smoothing_samples: u32,
    },
}

/// A named mix entity. Its graph behavior is deferred to issue 006.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Submix {
    /// Stable submix identity.
    pub id: StableId,
}

/// A named PCM output entity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Output {
    /// Stable output identity.
    pub id: StableId,
}

/// One signal route declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Route {
    /// Stable route identity.
    pub id: StableId,
    /// Typed source role. Outputs cannot be route sources.
    pub source: RouteSource,
    /// Typed destination role. Tracks cannot be route destinations.
    pub destination: RouteDestination,
    /// Explicit 2x2 route channel mapping; no implicit stereo copy is permitted.
    pub channel_matrix: ChannelMatrix,
    /// Send gain in decibels.
    pub gain_db: f32,
}

/// A graph source whose role is representable without downstream port metadata.
#[derive(Clone, Debug, PartialEq)]
pub enum RouteSource {
    /// A named track boundary.
    Track {
        /// Declared track identity.
        track_id: StableId,
        /// Explicit point in the track chain.
        tap: SendTap,
    },
    /// The output of a declared submix.
    SubmixOutput {
        /// Declared submix identity.
        submix_id: StableId,
    },
}

/// A graph destination whose role is representable without downstream port metadata.
#[derive(Clone, Debug, PartialEq)]
pub enum RouteDestination {
    /// The input of a declared submix.
    SubmixInput {
        /// Declared submix identity.
        submix_id: StableId,
    },
    /// The input of a declared PCM output.
    OutputInput {
        /// Declared output identity.
        output_id: StableId,
    },
}

/// A static 2x2 route channel matrix with `f32` coefficients.
#[derive(Clone, Debug, PartialEq)]
pub struct ChannelMatrix {
    /// Destination left from source left.
    pub ll: f32,
    /// Destination left from source right.
    pub lr: f32,
    /// Destination right from source left.
    pub rl: f32,
    /// Destination right from source right.
    pub rr: f32,
}

closed_tokens! {
    /// Stable explicit chain boundary names.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SendTap {
        /// Input signal.
        Input => "input",
        /// After input builtins.
        PostInputBuiltins => "post_input_builtins",
        /// After SIMD rack 1.
        PostSimd1 => "post_simd1",
        /// After dynamic rack.
        PostDynamic => "post_dynamic",
        /// After SIMD rack 2 and before fader.
        PostSimd2PreFader => "post_simd2_pre_fader",
        /// After fader.
        PostFader => "post_fader",
        /// After matrix/pan.
        PostMatrix => "post_matrix",
    }
}

/// A target and ordered piecewise automation declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct Automation {
    /// Stable automation identity.
    pub id: StableId,
    /// Typed target reference.
    pub target: AutomationTarget,
    /// Ordered segments, preserved canonically.
    pub segments: Vec<AutomationSegment>,
}

/// An effect parameter automation target.
#[derive(Clone, Debug, PartialEq)]
pub struct AutomationTarget {
    /// Owning track identity (only tracks carry racks).
    pub entity_id: StableId,
    /// Rack containing the named local effect.
    pub rack: RackName,
    /// Local effect slot identity.
    pub effect_id: StableId,
    /// Effect parameter identity.
    pub parameter_id: u32,
    /// Explicit target channel.
    pub channel: ParameterChannel,
}

closed_tokens! {
    /// One V1 rack token.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RackName {
        /// First homogeneous rack.
        Simd1 => "simd1",
        /// Dynamic rack.
        Dynamic => "dynamic",
        /// Second homogeneous rack.
        Simd2 => "simd2",
    }
}

/// Piecewise automation segment using absolute sample times.
#[derive(Clone, Debug, PartialEq)]
pub struct AutomationSegment {
    /// Interpolation behavior.
    pub shape: AutomationShape,
    /// Inclusive absolute sample time.
    pub start_sample: u64,
    /// Exclusive absolute sample time.
    pub end_sample: u64,
    /// Value at `start_sample` in `unit`.
    pub start_value: f32,
    /// Value at `end_sample` in `unit`.
    pub end_value: f32,
    /// Explicit unit for both values.
    pub unit: ParameterUnit,
}

closed_tokens! {
    /// Closed V1 interpolation token set.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum AutomationShape {
        /// A constant step segment.
        Step => "step",
        /// Linear interpolation.
        Linear => "linear",
        /// Exponential interpolation.
        Exponential => "exponential",
    }
}
