//! One track's live fader/mute and 2x2 matrix/pan sections, driven exactly as the console drives
//! them.
//!
//! The DSP is the engine's own: `builtins::FaderMuteRampBuiltins` (the section
//! `ConsoleFaderProcessor` binds for a console-driven track) and the `MatrixBuiltins` a
//! `BuiltinChain` prepares, retargeted through `set_target_smoothed` exactly as the matrix drain
//! does. A banked strip runs the same `FaderRampStage`/`MatrixStage` at a wider lane type, and
//! banking never changes per-lane arithmetic (AGENTS.md; `BuiltinFaderBank` docs), so these bits are
//! the bits a banked track renders.
//!
//! Delivery timing is the console's: a record admitted at sample `at` is drained at the top of the
//! first render block that starts at or after `at` (`drain_fader_controls`/`drain_matrix_controls`
//! in `builtins-compiler`), and the chain order is fader/mute then matrix/pan.

use builtins::{
    BuiltinChain, BuiltinLaneSelector, BuiltinParameters, ChannelParameters, DualMonoBlock,
    FaderMuteRampBuiltins, MatrixBuiltins, pan_matrix,
};

/// The render quantum every launch host uses (the browser's Web Audio quantum).
pub const QUANTUM: usize = 128;

#[derive(Clone, Copy, Debug)]
pub enum Control {
    FaderDb(f32),
    Mute(bool),
    /// Both lanes' pan position, lowered through the engine's `pan_matrix(left, right)`.
    Pan(f32),
}

#[derive(Clone, Copy, Debug)]
pub struct Event {
    /// Sample time at which the control plane admits the record.
    pub at: usize,
    pub control: Control,
    pub smoothing_samples: u32,
}

pub struct Strip {
    fader: FaderMuteRampBuiltins,
    matrix: MatrixBuiltins,
}

impl Strip {
    pub fn new(rate: u32, muted: bool) -> Self {
        let lane = ChannelParameters {
            muted,
            ..ChannelParameters::default()
        };
        let parameters = BuiltinParameters {
            left: lane,
            right: lane,
            ..BuiltinParameters::default()
        };
        let fader = FaderMuteRampBuiltins::new(parameters).expect("prepared fader");
        let (_, _, matrix) = BuiltinChain::new(rate, parameters)
            .expect("prepared chain")
            .into_sections();
        Self { fader, matrix }
    }

    fn apply(&mut self, event: &Event) {
        match event.control {
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
    let mut left = vec![1.0_f32; frames];
    let mut right = vec![0.0_f32; frames];
    Strip::new(rate, muted).render(&mut left, &mut right, events);
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
