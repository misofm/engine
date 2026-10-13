//! The live ramp lengths of a session (#1054 D4, decision 15 D15-1).

use effect_contract::control::LiveRampRow;
use session::SessionModel;

/// The ramp lengths, in samples, that live records carry: the session's `control_smoothing` keys
/// (`fader_ms`, `mute_ms`, `pan_ms`) rounded once by [`SessionModel::control_smoothing_samples`]
/// (#1054 D4, decision 15 D15-1).
///
/// Every live row reads its length through [`Self::for_row`]; no row has a separate default. The
/// table of which row uses which key is [`LiveRampRow`]'s, and `docs/SESSION_SCHEMA_V1.md` has it
/// too.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LiveRamps {
    /// The `fader_ms` key: the ramp of a `FaderDb` record.
    pub fader_samples: u32,
    /// The `mute_ms` key: the ramp of a `Mute` record.
    pub mute_samples: u32,
    /// The `pan_ms` key: the ramp of a pan or matrix record whose post-commit model carries no
    /// `smoothing_samples` of its own.
    pub pan_samples: u32,
    /// The `fader_ms` key, read by the detector link glide (#1371).
    pub link_samples: u32,
}

impl LiveRamps {
    /// The ramps a session's live records carry: its `control_smoothing` keys, or the researched
    /// default table when it says `default`, rounded at the session's sample rate (#1054 D2).
    #[must_use]
    pub fn for_session(model: &SessionModel) -> Self {
        let samples = model.control_smoothing_samples();
        Self {
            fader_samples: samples.fader,
            mute_samples: samples.mute,
            pan_samples: samples.pan,
            link_samples: samples.fader,
        }
    }

    /// The ramp length of one live row: its key's field, and twice `mute_samples` for
    /// [`LiveRampRow::PolarityInvert`] (at most 192,000 samples at 1000 ms and 96 kHz, inside the
    /// trim kernel's exact `f32` countdown, `2^24`).
    #[must_use]
    pub const fn for_row(self, row: LiveRampRow) -> u32 {
        match row {
            LiveRampRow::Fader
            | LiveRampRow::InputTrim
            | LiveRampRow::SendGain
            | LiveRampRow::VcaOffset => self.fader_samples,
            LiveRampRow::LinkGlide => self.link_samples,
            LiveRampRow::Mute
            | LiveRampRow::Solo
            | LiveRampRow::SendMute
            | LiveRampRow::SendFollowsMute
            | LiveRampRow::VcaMute
            | LiveRampRow::BypassCrossfade => self.mute_samples,
            LiveRampRow::PolarityInvert => self.mute_samples.saturating_mul(2),
            LiveRampRow::Pan | LiveRampRow::Matrix | LiveRampRow::SendMatrix => self.pan_samples,
        }
    }

    /// The length one record carries: the edit's own ramp when its transaction gave one (an
    /// explicit 0 included, which steps that edit), else the row's session length
    /// ([`Self::for_row`]). An edit's length is kept as given, so an absent polarity length is
    /// twice `mute_samples` and an explicit one is not doubled (#1394 D5).
    ///
    /// Every record length goes through here; no record path reads a field directly.
    #[must_use]
    pub const fn resolve(self, row: LiveRampRow, edit_ramp: Option<u32>) -> u32 {
        match edit_ramp {
            Some(samples) => samples,
            None => self.for_row(row),
        }
    }
}
