//! The `sixty_four_track_console_fader_ride` row (issue #1750): what it rides and how each move
//! reaches the engine.
//!
//! # The row
//!
//! The standing sixty-four-track console, prepared with every track's fader/mute and matrix/pan
//! live-control lanes on the product's `Concurrent` delivery
//! ([`SessionRuntime::build_with_strip_controls`]). In every block, one transaction moves the
//! fader of the first lane of every bank chain that carries the output section and the pan of the
//! second lane, each as the ordinary live value a host commits: a fader retarget per channel, Left
//! then Right, with the session's `fader_ms` ramp, and a pan lowered to its 2x2 target with the
//! ramp `host-core` gives a pan edit (the track's own pan window, else the session's `pan_ms`).
//! The banks are read off the prepared plan, so the row rides one fader and one pan in every bank
//! at whatever lane width the build dispatches.
//!
//! Every value alternates `base + step` and `base - step` (a pan position clamped into `[-1, 1]`),
//! so every block retargets every ridden cell to a value it does not hold and opens a ramp.
//!
//! # The stepped twin
//!
//! [`FaderRide::write`] with `stepped` writes the same values with a zero-length ramp, an exact
//! set. A twin plan driven that way holds every target from the block's first sample, so a block
//! the riding plan renders differently from its stepped twin is a block in which a ramp of the
//! riding plan was in flight. The bench uses this as its per-block witness, outside the clock.

use builtins::{BuiltinLaneSelector, Matrix2x2};
use builtins_compiler::{TrackControlRecord, TrackFaderRecord, lower_matrix_or_pan};
use controls::{LiveRampRow, LiveRamps};
use session::MatrixOrPan;

use crate::{QUANTUM, SessionRuntime, Workload, console_model};

/// The session the row renders: the standing console.
pub const WORKLOAD: Workload = Workload::SixtyFourTrackConsole;

/// The dB step each ridden fader channel alternates over, either side of its base.
pub const FADER_STEP_DB: f32 = 0.5;

/// The pan-position step each ridden pan channel alternates over, either side of its base.
pub const PAN_STEP: f32 = 0.1;

/// One ridden fader: a track and its two channels' values for even and odd blocks.
#[derive(Clone, Debug)]
pub struct RiddenFader {
    /// The track's session id.
    pub track_id: Box<str>,
    /// The track's strip live-control channel.
    channel: usize,
    /// `[even, odd]` block values, each `[left, right]` dB.
    pub values: [[f32; 2]; 2],
}

/// One ridden pan: a track and its lowered 2x2 targets for even and odd blocks.
#[derive(Clone, Debug)]
pub struct RiddenPan {
    /// The track's session id.
    pub track_id: Box<str>,
    /// The track's strip live-control channel.
    channel: usize,
    /// `[even, odd]` block pan positions, each `[left, right]`.
    pub positions: [[f32; 2]; 2],
    /// `[even, odd]` block targets, `positions` lowered through [`lower_matrix_or_pan`].
    targets: [Matrix2x2; 2],
}

/// The ride, resolved against one prepared arm.
#[derive(Clone, Debug)]
pub struct FaderRide {
    faders: Vec<RiddenFader>,
    pans: Vec<RiddenPan>,
    fader_smoothing: u32,
    pan_smoothing: u32,
}

impl FaderRide {
    /// Resolves the ride against `runtime`, an arm of [`WORKLOAD`] prepared by
    /// [`SessionRuntime::build_with_strip_controls`].
    ///
    /// # Panics
    ///
    /// Panics when the plan has no bank chain carrying the output section, when such a chain has
    /// fewer than two lanes, when a ridden track has no strip channel or no pan, or when a pan's two
    /// values lower to the same target. Each is a defect of the subject, not a condition to report.
    #[must_use]
    pub fn resolve(runtime: &SessionRuntime) -> Self {
        let model = console_model(WORKLOAD);
        let ramps = LiveRamps::for_session(&model);
        // The bank chains that carry the fader/matrix seam: a banked unit with stages past it.
        let banks: Vec<_> = runtime
            .unit_eligibility()
            .into_iter()
            .filter(|unit| unit.banked && unit.upstream_of_seam_stages < unit.stages)
            .collect();
        assert!(!banks.is_empty(), "the plan banks no output section");
        let track = |id: &str| {
            model
                .tracks
                .iter()
                .find(|track| track.id.as_str() == id)
                .expect("a ridden track of the session")
        };
        let channel = |id: &str| {
            runtime
                .strip_control(id)
                .expect("a strip live-control channel for every track")
        };
        let mut faders = Vec::with_capacity(banks.len());
        let mut pans = Vec::with_capacity(banks.len());
        let mut pan_own_smoothing = None;
        for bank in &banks {
            assert!(
                bank.lane_tracks.len() >= 2,
                "a bank with one lane cannot ride a fader and a pan on two tracks"
            );
            let (fader_id, pan_id) = (&*bank.lane_tracks[0], &*bank.lane_tracks[1]);
            let fader = &track(fader_id).fader;
            let base = [fader.left_db, fader.right_db];
            faders.push(RiddenFader {
                track_id: fader_id.into(),
                channel: channel(fader_id),
                values: [
                    base.map(|db| db + FADER_STEP_DB),
                    base.map(|db| db - FADER_STEP_DB),
                ],
            });
            let MatrixOrPan::Pan {
                left,
                right,
                smoothing_samples,
            } = track(pan_id).matrix_or_pan
            else {
                panic!("{pan_id}: the standing console pans every track");
            };
            // One window for every ridden pan, so the record can state it.
            assert!(
                pan_own_smoothing.is_none_or(|own| own == smoothing_samples),
                "the ridden pans declare different windows"
            );
            pan_own_smoothing = Some(smoothing_samples);
            let positions = [PAN_STEP, -PAN_STEP]
                .map(|step| [left, right].map(|base| (base + step).clamp(-1.0, 1.0)));
            let targets = positions.map(|[left, right]| {
                lower_matrix_or_pan(&MatrixOrPan::Pan {
                    left,
                    right,
                    smoothing_samples,
                })
                .expect("a pan position in domain")
                .0
            });
            let bits = |m: Matrix2x2| [m.ll, m.lr, m.rl, m.rr].map(f32::to_bits);
            assert_ne!(
                bits(targets[0]),
                bits(targets[1]),
                "{pan_id}: the two pan values lower to one target"
            );
            pans.push(RiddenPan {
                track_id: pan_id.into(),
                channel: channel(pan_id),
                positions,
                targets,
            });
        }
        // The ramp a host gives a pan edit with no ramp of its own (`host_core::matrix_record`):
        // the pan's own window when it has one, else the session's `pan_ms`.
        let pan_smoothing = match pan_own_smoothing {
            Some(own) if own != 0 => own,
            _ => ramps.resolve(LiveRampRow::Pan, None),
        };
        Self {
            faders,
            pans,
            fader_smoothing: ramps.resolve(LiveRampRow::Fader, None),
            pan_smoothing,
        }
    }

    /// Writes block `observation`'s values as one transaction, with the session's ramps, or with
    /// a zero-length ramp when `stepped`. Returns `[fader records, matrix records]` written.
    pub fn write(&self, runtime: &mut SessionRuntime, observation: u64, stepped: bool) -> [u64; 2] {
        let parity = usize::from(!observation.is_multiple_of(2));
        let (fader_smoothing, pan_smoothing) = if stepped {
            (0, 0)
        } else {
            (self.fader_smoothing, self.pan_smoothing)
        };
        runtime.write_strip_transaction(|controls, stamp| {
            let mut written = [0_u64; 2];
            for fader in &self.faders {
                let [left, right] = fader.values[parity];
                for (lanes, db) in [
                    (BuiltinLaneSelector::Left, left),
                    (BuiltinLaneSelector::Right, right),
                ] {
                    controls[fader.channel].fader.write(
                        stamp,
                        TrackFaderRecord::FaderDb {
                            lanes,
                            db,
                            smoothing_samples: fader_smoothing,
                        },
                    );
                    written[0] += 1;
                }
            }
            for pan in &self.pans {
                controls[pan.channel].producer.write(
                    stamp,
                    TrackControlRecord {
                        matrix: pan.targets[parity],
                        smoothing_samples: pan_smoothing,
                    },
                );
                written[1] += 1;
            }
            written
        })
    }

    /// Bank chains ridden: one fader and one pan in each.
    #[must_use]
    pub fn banks(&self) -> usize {
        self.faders.len()
    }

    /// The ridden faders, in bank order.
    #[must_use]
    pub fn faders(&self) -> &[RiddenFader] {
        &self.faders
    }

    /// The ridden pans, in bank order.
    #[must_use]
    pub fn pans(&self) -> &[RiddenPan] {
        &self.pans
    }

    /// The ramp, in samples, of a ridden fader record.
    #[must_use]
    pub fn fader_smoothing(&self) -> u32 {
        self.fader_smoothing
    }

    /// The ramp, in samples, of a ridden matrix record.
    #[must_use]
    pub fn pan_smoothing(&self) -> u32 {
        self.pan_smoothing
    }

    /// Fader records per block: a Left and a Right per ridden fader.
    #[must_use]
    pub fn fader_records_per_block(&self) -> u64 {
        2 * self.faders.len() as u64
    }

    /// Matrix records per block: one per ridden pan.
    #[must_use]
    pub fn matrix_records_per_block(&self) -> u64 {
        self.pans.len() as u64
    }

    /// Untimed blocks after one move in which every ramp it opened has ended: the longer ramp in
    /// whole blocks, and one more.
    #[must_use]
    pub fn settle_blocks(&self) -> u64 {
        u64::from(self.fader_smoothing.max(self.pan_smoothing)).div_ceil(QUANTUM as u64) + 1
    }
}
