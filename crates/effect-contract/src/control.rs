//! The vocabulary of a control row (decision 17: one control abstraction, one core).
//!
//! A control row says how one value behaves: whether a change is a live update or a plan rebuild
//! and why, which `control_smoothing` key ramps it, which render family carries it, and how a
//! frontend should draw it. The builtin parameter rows and the effect rows name these types, so
//! they live here, below both. The `controls` crate re-exports them and owns everything that
//! needs the session model.

/// Why a value is rebuild-only (decision 14's rules, the correctness reason of decision 15, and
/// the reason of a value that a ruling already makes live whose issue has not landed).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RebuildReason {
    /// Decision 14 rule 1: the change needs new memory.
    NewMemory,
    /// Decision 14 rule 1: the change makes a new graph.
    NewGraph,
    /// Decision 14 rule 1: the change makes a new processing order.
    ProcessingOrder,
    /// Decision 14 rule 1: the change makes a new latency.
    Latency,
    /// Decision 14 rule 3: a live change always glitches.
    Glitch,
    /// Decision 14 rule 3: a constant value enables an important optimisation.
    Optimisation,
    /// Decision 15 (E4): a recorded correctness reason.
    Correctness,
    /// A ruling already makes this value live, and the issue that builds it has not landed. That
    /// issue's last step flips the row to [`ControlClass::Live`].
    RuledLiveNotBuilt {
        /// The issue that builds the live path. `0` until root numbers it.
        issue: u32,
    },
}

/// Why a prepared effect parameter (`AutomationRate::None`) is not live, recorded in its own
/// descriptor row (decision 17, D1): the reason, and the condition that reopens the question or,
/// for [`RebuildReason::RuledLiveNotBuilt`], the issue that flips it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PreparedReason {
    /// Why the value is rebuild-only.
    pub reason: RebuildReason,
    /// The reopening condition in one sentence, or the issue that makes the row live.
    pub reopen: &'static str,
}

/// How a change to one control reaches a running plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlClass {
    /// The plan has a slot for the value: a change is a live update, ramped.
    Live,
    /// A change compiles and swaps in a replacement plan.
    RebuildOnly {
        /// Why the value is not live.
        reason: RebuildReason,
        /// The condition that reopens the question, in one sentence.
        reopen: &'static str,
    },
    /// Computed from other controls (an effective fader, a route gate): read-only.
    Derived,
}

/// One live row, and with it the `control_smoothing` key its ramp length comes from (#1054 D3).
///
/// | key | rows |
/// |---|---|
/// | `fader_ms` | fader dB, input trim, send gain, VCA offset, detector link glide |
/// | `mute_ms` | mute, solo, polarity invert (twice `mute_ms`), send mute, send `follows_mute` toggle, VCA mute, effect bypass crossfade |
/// | `pan_ms` | pan, matrix, send matrix |
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRampRow {
    /// A strip's fader dB. Key `fader_ms`.
    Fader,
    /// A strip's input trim. Key `fader_ms`.
    InputTrim,
    /// A send's gain. Key `fader_ms`.
    SendGain,
    /// A VCA's fader offset. Key `fader_ms`.
    VcaOffset,
    /// The detector link glide. Key `fader_ms`.
    LinkGlide,
    /// A strip's mute. Key `mute_ms`.
    Mute,
    /// A strip's solo, which reaches render as a mute record. Key `mute_ms`.
    Solo,
    /// A strip's polarity invert: twice the `mute_ms` ramp, because a flip from `+1` to `-1` over
    /// the mute's own length clicks about 6 dB more than the mute (#1055, `FINDINGS.md` 9.2).
    PolarityInvert,
    /// A send's mute. Key `mute_ms`.
    SendMute,
    /// A send's `follows_mute` toggle. Key `mute_ms`.
    SendFollowsMute,
    /// A VCA's mute. Key `mute_ms`.
    VcaMute,
    /// The effect bypass crossfade (#1341), the one row with no per-edit length. Key `mute_ms`.
    BypassCrossfade,
    /// A strip's pan. Key `pan_ms`.
    Pan,
    /// A strip's raw matrix. Key `pan_ms`.
    Matrix,
    /// A send's channel matrix. Key `pan_ms`.
    SendMatrix,
}

/// The render family that carries a live change: one typed cell and one lowering function each.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Family {
    /// A strip's fader and mute.
    StripLevel,
    /// A strip's pan, matrix and matrix mode.
    StripMatrix,
    /// A strip's input section: trim, polarity, filters, delay.
    StripInput,
    /// A route's gain, mute, matrix and follow gate.
    Route,
    /// An effect's parameters and bypass.
    Effect,
    /// No render record: the value is read when a ramp length is chosen.
    Setting,
}

/// How a frontend should map a control's range to a knob or fader.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Taper {
    /// Equal knob travel per unit of value.
    Linear,
    /// Equal knob travel per octave (frequency).
    Logarithmic,
    /// Equal knob travel per ratio of values.
    Exponential,
    /// The value is already in decibels: equal travel per dB.
    Decibel,
    /// A fixed set of values (a toggle or an enumeration).
    Stepped,
}

/// What kind of value a control holds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlKind {
    /// Any value in the range.
    Continuous,
    /// Exactly `0.0` (off) or `1.0` (on).
    Toggle,
    /// One of a listed set of values.
    Enumeration,
}
