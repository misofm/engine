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
    // Explicit wire codes, for a table that has retired a code: an index-derived code would
    // silently renumber every later token when an earlier one is removed.
    ($(#[$enum_meta:meta])* pub enum $name:ident explicit {
        $($(#[$variant_meta:meta])* $variant:ident = $wire:literal => $token:literal),+ $(,)?
    }) => {
        $(#[$enum_meta])*
        #[repr(u8)]
        pub enum $name { $($(#[$variant_meta])* $variant = $wire),+ }

        impl $name {
            /// Every token value in wire-code order.
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
            /// Return the stable nonzero wire code, spelled explicitly per variant.
            #[must_use]
            pub const fn wire(self) -> u8 { self as u8 }
            /// Parse a stable nonzero wire code. A retired code is refused, never reinterpreted.
            #[must_use]
            pub const fn from_wire(wire: u8) -> Option<Self> {
                match wire { $($wire => Some(Self::$variant),)+ _ => None }
            }
        }
        impl ClosedToken for $name {
            const ALL: &'static [(Self, &'static str)] = Self::ALL;
        }
    };
}

/// Strict Session V1 model after JSON syntax/schema parsing.
#[derive(Clone, Debug, PartialEq)]
pub struct SessionModel {
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
    /// Sources, order-insensitive by stable ID.
    pub sources: Vec<Source>,
    /// The session-level console: every track carries every slot, in this order (decision 12).
    pub console: Console,
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
        /// Deterministic single-control-thread preparation, the only V1 render mode. Wire code
        /// `2` was the retired `dependency_waves` token (#1063) and is never reallocated.
        SingleThread => "single_thread",
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

/// A just-in-time source declaration; resolution is deferred to issue 010.
#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    /// Stable source identity.
    pub id: StableId,
    /// Canonical-PCM content identity.
    pub content: String,
    /// Declared source channels.
    pub channels: u8,
    /// Exact canonical sample-depth token.
    pub bit_depth: SourceBitDepth,
    /// Exact source length in sample frames.
    pub frames: u64,
}

/// Canonical source sample-depth token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceBitDepth {
    /// Signed little-endian 16-bit integer PCM.
    Pcm16,
    /// Signed little-endian packed 24-bit integer PCM.
    Pcm24,
    /// Raw IEEE-754 little-endian 32-bit float bits.
    Float32,
}

impl SourceBitDepth {
    /// Stable nonzero BTLV token code.
    #[must_use]
    pub const fn wire(self) -> u8 {
        match self {
            Self::Pcm16 => 1,
            Self::Pcm24 => 2,
            Self::Float32 => 3,
        }
    }

    /// Parse one stable nonzero BTLV token code.
    #[must_use]
    pub const fn from_wire(wire: u8) -> Option<Self> {
        match wire {
            1 => Some(Self::Pcm16),
            2 => Some(Self::Pcm24),
            3 => Some(Self::Float32),
            _ => None,
        }
    }

    /// Exact canonical declaration token.
    #[must_use]
    pub const fn token(self) -> &'static str {
        match self {
            Self::Pcm16 => "16",
            Self::Pcm24 => "24",
            Self::Float32 => "32f",
        }
    }
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
    /// This track's knobs for every session console slot, in exactly the session's slot order:
    /// `console.pre_insert`, then `console.post_insert` (decision 12).
    pub console: Vec<ConsoleEntry>,
    /// Per-track ordered inserts, between the two console sections. Replaces the retired
    /// `dynamic` rack with the same semantics.
    pub inserts: Rack,
    /// Independent left/right fader and mute declaration.
    pub fader: DualMonoFader,
    /// Explicit pan or cross-channel matrix; no implicit stereo operation exists.
    pub matrix_or_pan: MatrixOrPan,
}

/// The session-level console (owner decision 12).
///
/// Each slot is declared once, and every track carries every slot with only its own `bypass` and
/// `params`. `pre_insert` runs before a track's inserts and `post_insert` after them. Either list
/// may be empty. Slot IDs are unique across both lists, because a console address names the slot
/// and not its section.
#[derive(Clone, Debug, PartialEq)]
pub struct Console {
    /// Slots between the input section and the inserts, in chain order.
    pub pre_insert: Vec<ConsoleSlot>,
    /// Slots between the inserts and the fader, in chain order.
    pub post_insert: Vec<ConsoleSlot>,
}

impl Console {
    /// Every declared slot in track-entry order: `pre_insert`, then `post_insert`.
    pub fn slots(&self) -> impl Iterator<Item = &ConsoleSlot> {
        self.pre_insert.iter().chain(&self.post_insert)
    }
}

/// One session console slot: the effect every track runs at this point in its chain.
///
/// A console slot has no sidechain: a keyed effect is an insert (decision 12).
#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleSlot {
    /// Stable slot identity, unique across both console sections.
    pub slot: StableId,
    /// The native effect identity. A third-party (`cid`) identity is refused.
    pub identity: EffectIdentity,
    /// Requested quality profile, shared by every track.
    pub quality: EffectQuality,
    /// Explicit detector/channel link mode, shared by every track.
    pub link_mode: LinkMode,
}

/// One track's knobs for one console slot. It carries no effect fields.
#[derive(Clone, Debug, PartialEq)]
pub struct ConsoleEntry {
    /// The declared slot this entry configures.
    pub slot: StableId,
    /// Latency-preserving bypass for this track.
    pub bypass: bool,
    /// Parameter declarations, canonicalized by parameter ID then channel.
    pub params: Vec<EffectParam>,
}

/// A track's chain lowered to the engine's three internal racks (decision 12, class A by
/// lowering).
///
/// `console.pre_insert` lowers to the first rack (`RackId::Simd1` in the graph), the track's
/// `inserts` to the second (`Dynamic`) and `console.post_insert` to the third (`Simd2`). Each
/// console entry becomes an [`Effect`] whose `id` is the slot, whose identity, quality and link
/// mode come from the session slot, whose bypass and params come from the track, and whose
/// sidechain is `none`. An equivalent session therefore compiles to the identical graph.
#[derive(Clone, Debug, PartialEq)]
pub struct LoweredRacks<'a> {
    /// `console.pre_insert`, lowered.
    pub pre_insert: Vec<Effect>,
    /// The track's own inserts, borrowed unchanged.
    pub inserts: &'a [Effect],
    /// `console.post_insert`, lowered.
    pub post_insert: Vec<Effect>,
}

impl LoweredRacks<'_> {
    /// The three racks in chain order: pre-insert, inserts, post-insert.
    #[must_use]
    pub fn in_chain_order(&self) -> [&[Effect]; 3] {
        [&self.pre_insert, self.inserts, &self.post_insert]
    }

    /// Effect instances across the three racks.
    #[must_use]
    pub fn len(&self) -> usize {
        self.pre_insert.len() + self.inserts.len() + self.post_insert.len()
    }

    /// Whether the track runs no effect at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SessionModel {
    /// Lower one track's console entries and inserts to the three internal racks.
    ///
    /// The console entries are matched to the session slots by position, which validation makes
    /// exact: a validated track carries one entry per slot, in slot order. On an unvalidated model
    /// a surplus entry or slot is dropped rather than guessed at.
    #[must_use]
    pub fn lower_track<'a>(&self, track: &'a Track) -> LoweredRacks<'a> {
        let (pre_entries, post_entries) = track
            .console
            .split_at(self.console.pre_insert.len().min(track.console.len()));
        LoweredRacks {
            pre_insert: lower_section(&self.console.pre_insert, pre_entries),
            inserts: &track.inserts.effects,
            post_insert: lower_section(&self.console.post_insert, post_entries),
        }
    }
}

fn lower_section(slots: &[ConsoleSlot], entries: &[ConsoleEntry]) -> Vec<Effect> {
    slots
        .iter()
        .zip(entries)
        .map(|(slot, entry)| Effect {
            id: slot.slot.clone(),
            identity: slot.identity.clone(),
            quality: slot.quality,
            bypass: entry.bypass,
            link_mode: slot.link_mode,
            params: entry.params.clone(),
            sidechain: SidechainDeclaration::None,
        })
        .collect()
}

/// Independent builtins for the two dual-mono lanes.
#[derive(Clone, Debug, PartialEq)]
pub struct DualMonoBuiltins {
    /// Left lane state/parameters.
    pub left: ChannelBuiltins,
    /// Right lane state/parameters.
    pub right: ChannelBuiltins,
}

/// Inclusive maximum for `ChannelBuiltins::delay_samples`.
///
/// About 1.09 s at 44.1 kHz and 0.5 s at 96 kHz -- an order of magnitude beyond any mic-alignment
/// need -- which bounds the worst-case ring at 192,000 bytes per lane.
pub const CHANNEL_BUILTIN_DELAY_SAMPLES_MAXIMUM: u32 = 48_000;

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
    /// Input-side time alignment, in samples, applied at the track's `Input` stage.
    ///
    /// Samples rather than milliseconds: alignment is a sample-exact operation, the engine is
    /// sample-domain everywhere, and #147's unit-in-name rule makes the unit explicit. A UI
    /// converts from milliseconds; the session never does.
    ///
    /// Per-lane, following the dual-mono law. A track whose two lanes declare **different**
    /// delays is genuinely asymmetric upstream of the mono-collapse seam and declines that
    /// track's collapse; see `session_structural_symmetry`.
    ///
    /// This is deliberately **not** plugin latency. PDC equalizes unrequested arrival-time skew;
    /// this is a musical time shift the session asked for, so it contributes zero to
    /// `GraphNode.latency` and PDC must never compensate it away.
    pub delay_samples: u32,
}

/// An ordered effect rack.
#[derive(Clone, Debug, PartialEq)]
pub struct Rack {
    /// Effect order is semantically significant and preserved canonically.
    pub effects: Vec<Effect>,
}

/// Typed effect declaration; availability is issue-011 work and CID validity issue-029 work.
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
    /// Third-party package CIDv1 text. CID validation is deferred to issue 029.
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
    ///
    /// Decision 12 renamed the tokens and kept every position and wire code `1..=7`. The retired
    /// spellings (`post_input_builtins`, `post_simd1`, `post_dynamic`, `post_simd2_pre_fader`,
    /// `post_matrix`) are unknown values and refuse.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SendTap {
        /// Input signal.
        Input => "input",
        /// After the input section (polarity, trim, filters).
        PostInput => "post_input",
        /// After `console.pre_insert`: the insert send.
        InsertSend => "insert_send",
        /// After the track's inserts: the insert return.
        InsertReturn => "insert_return",
        /// After `console.post_insert` and before the fader.
        PreFader => "pre_fader",
        /// After fader.
        PostFader => "post_fader",
        /// After pan/matrix.
        PostPan => "post_pan",
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
    ///
    /// The wire codes are explicit (decision 12). `1` (`simd1`) and `3` (`simd2`) are retired:
    /// refused, never reallocated, and their spellings are unknown tokens.
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RackName explicit {
        /// A track's inserts. It kept the retired `dynamic` rack's code.
        Inserts = 2 => "inserts",
        /// The strip's own builtin section: trim, polarity, filters, fader, mute and the
        /// matrix/pan pair (issue #178, ruled by #210's D2).
        ///
        /// Not a rack of swappable modules -- it is the chassis. The token joins `RackName`
        /// anyway, and does not get a vocabulary of its own, because an automation target's
        /// shape is `{ entity_id, rack, effect_id, parameter_id, channel }` and the strip is
        /// addressed through exactly that shape: `effect_id` carries the fixed validated literal
        /// `"strip"` (the schema has no optional keys, so the field must carry a value rather
        /// than be omitted) and `parameter_id` carries a builtin descriptor id.
        ///
        /// **Appended, never inserted.** Its code is `4`, spelled explicitly so that retiring
        /// `simd1` and `simd2` renumbered nothing.
        Builtins = 4 => "builtins",
        /// A session console slot: `effect_id` names the slot, in either section (decision 12).
        Console = 5 => "console",
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
