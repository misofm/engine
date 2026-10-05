//! One track's live input trim/polarity, fader/mute and 2x2 matrix/pan sections, driven exactly as
//! the console drives them.
//!
//! The DSP is the engine's own: the `InputBuiltins` a `BuiltinChain` prepares, retargeted through
//! `set_polarity_invert` and `set_trim_db` (both `InputStage::set_trim_signed`, the D11 retarget of
//! one signed coefficient); `builtins::FaderMuteRampBuiltins` (the section `ConsoleFaderProcessor`
//! binds for a console-driven track); and the `MatrixBuiltins` a `BuiltinChain` prepares,
//! retargeted through `set_target_smoothed` exactly as the matrix drain does. A banked strip runs
//! the same `InputStage`/`FaderRampStage`/`MatrixStage` at a wider lane type, and banking never
//! changes per-lane arithmetic (AGENTS.md; `BuiltinFaderBank` docs), so these bits are the bits a
//! banked track renders.
//!
//! Delivery timing is the console's: a record admitted at sample `at` is drained at the top of the
//! first render block that starts at or after `at` (`drain_fader_controls`/`drain_matrix_controls`
//! in `builtins-compiler`, and the input drain of `BuiltinBankProcessor::drain_controls`), and the
//! chain order is input (trim/polarity, its filters off), then fader/mute, then matrix/pan. An
//! input section at its defaults (trim 0 dB, polarity off, filters off) multiplies by exactly
//! `1.0`, so the fader and matrix measurements render the same bits with or without it.

use builtins::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameters, ChannelParameters, DualMonoBlock,
    FaderMuteRampBuiltins, InputBuiltins, MatrixBuiltins, pan_matrix,
};

/// The render quantum every launch host uses (the browser's Web Audio quantum).
pub const QUANTUM: usize = 128;

#[derive(Clone, Copy, Debug)]
pub enum Control {
    FaderDb(f32),
    Mute(bool),
    /// Both lanes' pan position, lowered through the engine's `pan_matrix(left, right)`.
    Pan(f32),
    /// Both lanes' polarity inversion: the input trim coefficient carried through zero.
    Polarity(bool),
    /// Both lanes' input trim in dB, keeping the polarity.
    TrimDb(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
    /// Sample time at which the control plane admits the record.
    pub at: usize,
    pub control: Control,
    pub smoothing_samples: u32,
}

pub struct Strip {
    input: InputBuiltins,
    fader: FaderMuteRampBuiltins,
    matrix: MatrixBuiltins,
}

impl Strip {
    pub fn new(rate: u32, muted: bool) -> Self {
        Self::with(
            rate,
            ChannelParameters {
                muted,
                ..ChannelParameters::default()
            },
        )
    }

    /// A strip whose two lanes are prepared with `lane`.
    pub fn with(rate: u32, lane: ChannelParameters) -> Self {
        let parameters = BuiltinParameters {
            left: lane,
            right: lane,
            ..BuiltinParameters::default()
        };
        let fader = FaderMuteRampBuiltins::new(parameters).expect("prepared fader");
        let (input, _, matrix) = BuiltinChain::new(rate, parameters)
            .expect("prepared chain")
            .into_sections();
        Self {
            input,
            fader,
            matrix,
        }
    }

    fn apply(&mut self, event: &Event) {
        match event.control {
            Control::Polarity(inverted) => {
                self.input.set_polarity_invert(
                    BuiltinLaneSelector::Both,
                    inverted,
                    event.smoothing_samples,
                );
            }
            Control::TrimDb(db) => self
                .input
                .set_trim_db(BuiltinLaneSelector::Both, db, event.smoothing_samples)
                .expect("trim_db inside [-144, 24]"),
            Control::FaderDb(db) => self
                .fader
                .set_fader_db(BuiltinLaneSelector::Both, db, event.smoothing_samples)
                .expect("fader_db inside [-144, 24]"),
            Control::Mute(muted) => {
                self.fader
                    .set_mute(BuiltinLaneSelector::Both, muted, event.smoothing_samples);
            }
            Control::Pan(position) => {
                let matrix = pan_matrix(position, position).expect("pan inside [-1, 1]");
                self.matrix
                    .set_target_smoothed(matrix, event.smoothing_samples)
                    .expect("matrix coefficients");
            }
        }
    }

    /// Renders `left`/`right` in place, block by block, applying each event at the top of the
    /// first block that starts at or after its admission sample. `events` must be sorted by `at`.
    pub fn render(&mut self, left: &mut [f32], right: &mut [f32], events: &[Event]) {
        assert_eq!(left.len(), right.len());
        let mut next = 0;
        let mut start = 0;
        while start < left.len() {
            while next < events.len() && events[next].at <= start {
                self.apply(&events[next]);
                next += 1;
            }
            let end = (start + QUANTUM).min(left.len());
            let block =
                DualMonoBlock::new(&mut left[start..end], &mut right[start..end], start as u64)
                    .expect("block");
            self.input.process(block);
            let block =
                DualMonoBlock::new(&mut left[start..end], &mut right[start..end], start as u64)
                    .expect("block");
            self.fader.process(block);
            let block =
                DualMonoBlock::new(&mut left[start..end], &mut right[start..end], start as u64)
                    .expect("block");
            self.matrix.process(block);
            start = end;
        }
    }
}

/// Renders a unit probe through a fresh strip and returns the per-sample coefficients.
///
/// With `left = 1, right = 0` the outputs are exactly the `ll` (left out) and `rl` (right out)
/// matrix coefficients times the fader gain, because the ramp kernels multiply and nothing else.
pub fn probe(rate: u32, muted: bool, frames: usize, events: &[Event]) -> (Vec<f32>, Vec<f32>) {
    probe_with(
        rate,
        ChannelParameters {
            muted,
            ..ChannelParameters::default()
        },
        frames,
        events,
    )
}

/// [`probe`] through a strip prepared with `lane` on both lanes.
///
/// The input trim multiplies first and nothing else follows it but the fader and matrix
/// multiplies, so with the fader at unity and the identity matrix the left output is exactly the
/// trim coefficient's trajectory.
pub fn probe_with(
    rate: u32,
    lane: ChannelParameters,
    frames: usize,
    events: &[Event],
) -> (Vec<f32>, Vec<f32>) {
    let mut left = vec![1.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    Strip::with(rate, lane).render(&mut left, &mut right, events);
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The engine's mute is the D11 linear ramp: `step = -1 / N`, applied before the multiply, the
    /// exact target on update `N`, and an exact `+0.0` from then on. It starts at the first block
    /// boundary at or after admission.
    #[test]
    fn mute_is_a_linear_ramp_from_the_next_block_boundary() {
        let n = 240_u32;
        let at = 3 * QUANTUM + 5;
        let events = [Event {
            at,
            control: Control::Mute(true),
            smoothing_samples: n,
        }];
        let (g, right) = probe(48_000, false, 8 * QUANTUM, &events);
        let start = 4 * QUANTUM;
        assert!(g[..start].iter().all(|&v| v == 1.0));
        for k in 0..n as usize - 1 {
            let want = 1.0 - (k as f64 + 1.0) / f64::from(n);
            assert!((f64::from(g[start + k]) - want).abs() < 1e-5, "k={k}");
        }
        assert!(g[start + n as usize - 1..].iter().all(|&v| v == 0.0));
        assert!(right.iter().all(|&v| v == 0.0));
    }

    /// A fader retarget ramps linearly in amplitude from the *current* gain, over a fresh window.
    #[test]
    fn fader_retarget_restarts_from_the_current_gain() {
        let events = [
            Event {
                at: 0,
                control: Control::FaderDb(-20.0),
                smoothing_samples: 256,
            },
            Event {
                at: QUANTUM,
                control: Control::FaderDb(0.0),
                smoothing_samples: 256,
            },
        ];
        let (g, _) = probe(48_000, false, 4 * QUANTUM, &events);
        let mid = f64::from(g[QUANTUM - 1]);
        assert!((mid - (1.0 - 0.9 * 128.0 / 256.0)).abs() < 1e-5);
        let step = (1.0 - mid) / 256.0;
        assert!((f64::from(g[QUANTUM]) - (mid + step)).abs() < 1e-5);
        assert_eq!(g[QUANTUM + 255], 1.0);
    }
}
